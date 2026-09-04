---
needs: [idris-sba-not-a-static-ability]
---
**Align executable syntax, Abilities, and Effects with the
[Game Model glossary](../../contexts/game-model/CONTEXT.md) and
[CR#609.1,610.1,611.1,614.1,615.1].** Today
`OneShotEffect` names executable semantic syntax, its `Instr` alias points in
the opposite direction, and `StaticEffect` is the payload of
`Ability::Static` even though a Static Ability is not itself an Effect. The
latter enum also mixes continuous rule changes, replacement and prevention
effects, and non-effect rule data.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Make `Instruction` the canonical name for executable semantic syntax and
remove `OneShotEffect` as a compatibility alias. Rename the authored Static
Ability payload accordingly while preserving the compact
`Ability::Static(...)` RON shape. Model actual `OneShotEffect` and
`ContinuousEffect` values only where the engine represents results described
by [CR#610] and [CR#611]; keep `ReplacementEffect` and `PreventionEffect` as
the applicable continuous-effect subfamilies, with self-replacement effects
explicitly outside that family ([CR#614.15]).

Classify every current `StaticEffect` variant before moving it. Rule-affecting
continuous effects may remain continuous effects under [CR#611.1]; state-based
rules, instructions, and authoring-only structure move to their own homes.
Update `deckmaste_core`'s types, the Idris workbench
(`idris/src/Experimental/`), the core-facing RON stubs, lowering's core-facing
output, engine consumers, and terminology documentation together. Do not
retain misleading type aliases for compatibility.

Overlaps `workbench-fused-variants-3`, which collapses and renames overloaded
workbench effect constructors; whichever runs second reconciles the names.

Done when: `Instruction` is the only name for executable syntax and no
`OneShotEffect`/`Instr` alias survives; every former `StaticEffect` variant
sits in its classified home; the workspace is green and `cd idris &&
./scripts/build` is green at its module count.

## As landed

### Classification — every `StaticEffect` variant, before moving it

Inventory command: `grep -rn "OneShotEffect\|\bInstr\b\|StaticEffect"
crates/deckmaste_core/src crates/deckmaste_engine/src
crates/deckmaste_lowering/src plugins/builtin_v2 | wc -l` → 1416 (core 194,
engine 902, lowering 320, `plugins/builtin_v2` 0 — the v2 stubs never name a
core type, so no RON stub needed re-authoring).

| Variant | CR category | Home | Moved? |
|---|---|---|---|
| `Modify(Reference, Modification)` | Continuous effect — modifies characteristics ([CR#611.1,613.1]) | `StaticSpec` | no |
| `BecomesCopy(Reference, CopySpec)` | Continuous effect — layer-1a copy ([CR#611.1,707.4]) | `StaticSpec` | no |
| `Each(Selection, Region<_>)` | Not an effect — authoring structure distributing an inner spec over a `Selection`; the affected set re-gathers live ([CR#613.6]) | `StaticSpec` (combinator) | no |
| `Conditionally(Condition, _)` | Not an effect — authoring structure; the conditional-static wrapper whose content is never locked in ([CR#611.3a]) | `StaticSpec` (combinator) | no |
| `Deontic(Deontic)` | Continuous effect — affects the rules ([CR#611.1,101.2]) | `StaticSpec` | no |
| `CostModifier { of, change }` | Continuous effect — affects the rules of paying ([CR#611.1,118.7]) | `StaticSpec` | no |
| `CostOption(OptionalCost)` | Continuous effect — affects the rules of casting; the optional additional cost announced at [CR#601.2b] ([CR#611.1,118.8b]) | `StaticSpec` | no |
| `TriggerMultiplier { … }` | Continuous effect — affects the rules ([CR#611.1,603.2d]) | `StaticSpec` | no |
| `ModifyPlayer(Reference, PlayerMod)` | Continuous effect — affects players ([CR#611.1]) | `StaticSpec` | no |
| `Replacement(Arc<Replacement>)` | Replacement effect — the applicable continuous-effect subfamily ([CR#614.1]) | `StaticSpec` | no |
| `Prevention(Arc<Prevention>)` | Prevention effect — the applicable continuous-effect subfamily ([CR#615.1]) | `StaticSpec` | no |
| `ReplaceRoll { … }` | Replacement effect — an "instead" roll-more replacement ([CR#614.1a,706.6]) | `StaticSpec` | no |
| `CantPrevent { from, to }` | Continuous effect — affects the rules; gates the prevention class ([CR#611.1,615.12]) | `StaticSpec` | no |
| `SpendAsThough { … }` | Continuous effect — affects the rules of payment only ([CR#611.1,609.4b]) | `StaticSpec` | no |
| `AsThough(AsThough)` | Continuous effect — a scoped counterfactual premise ([CR#611.1,609.4]) | `StaticSpec` | no |
| `OutcomeGate { who, gate }` | Continuous effect — affects the rules of winning/losing; a "can't" gate takes precedence ([CR#611.1,101.2]) | `StaticSpec` | no |
| `CantHappen(EventFilter)` | Continuous effect — a can't-happen effect, not a replacement effect but following similar rules ([CR#611.1,614.17]) | `StaticSpec` | no |
| `PayPips(PipClass, PayAct)` | Continuous effect — affects how a locked-in cost's pips may be paid, never the cost ([CR#611.1,601.2h,702.51b]) | `StaticSpec` | no |
| `Sba { when, then }` | Rule-affecting continuous effect / conditional static — a state-checked static ability ([CR#604.1,702.131b]), NOT a rules-defined state-based action | `StaticSpec` — **left in place under the round's standing instruction**; removal or rename is `state-checked-static-home`'s ruling | no |

**Counts per home: 15 continuous effects under [CR#611.1] (one of them the
STOP-flagged `Sba`), 3 in the replacement/prevention subfamilies
([CR#614.1,614.1a,615.1]), 2 authoring-only combinators, 0 state-based rules,
0 instructions. Zero variants moved.**

The table is the round's finding, not a shortfall: the enum's misnomer was its
*name*, not its membership. Every member is something a static ability states
or a one-shot puts in force, and each is a continuous effect or an applicable
subfamily of one; the two combinators are structure over those members and
have no other home (the Idris workbench mirrors them the same way). The
state-based-rule exodus this ticket anticipated already happened in
`idris-sba-not-a-static-ability`, which moved the rules-defined [CR#704] rules
onto `Property::StateBased`/`SbaRule`; what remains under `Sba` is Ascend's
genuine static ability, held for `state-checked-static-home`.

Self-replacement effects ([CR#614.15]) are explicitly outside the family and
have no `StaticSpec` variant — they are effects of a resolving spell or
ability, not continuous effects. That exclusion is now stated on the enum.

### Renames

Rust (`deckmaste_core`, propagated through `deckmaste_engine`,
`deckmaste_lowering`'s core-facing output, and `deckmaste_plugin`'s
core-facing tests):

- `Instr` (the enum) → **`Instruction`**; the `pub type OneShotEffect = Instr`
  compatibility alias and its `pub use effect::OneShotEffect` re-export are
  **deleted**, not re-pointed. No alias survives.
- `StaticEffect` → **`StaticSpec`**. `Ability::Static(Arc<Region<StaticSpec>>)`
  keeps its compact positional RON shape (`Static(Each(SelectAll(…),
  Modify(It, …)))`) — a type rename changes no RON tag, which the
  `deckmaste_plugin` corpus tests (`builtin`, `canon`, `keywords`, `tokens`,
  `attach_subtypes`, `corpus_identity`) confirm by reading the real stubs.
- No `ContinuousEffect` type was introduced: the engine already has exactly one,
  `deckmaste_engine::layer::ContinuousEffect`, the record of an effect actually
  in force ([CR#611]); its `rows: Vec<StaticSpec>` now reads honestly as the
  specs it applies. No `OneShotEffect` value type exists or was added: a
  one-shot effect ([CR#610.1]) is what executing an `Instruction` produces, and
  the engine materializes no separate record for it.

Idris workbench (`idris/src/Experimental/`) — the same misnomer, mirrored:

- `Effect` (the executable-syntax sort) → **`Instruction`**; `Effects` →
  `Instructions`; `SimEffects` → `SimInstructions`.
- `StaticEffect` → **`StaticSpec`** (positional constructors unchanged, macros
  re-pointed by the same rename, bench witnesses still carry no `{…}` braces).
- Derived helper names followed their sort: `effIntro`/`effDelta`/`effsIntro`/
  `effChoiceDelta`/`effsChoiceDelta`/`doesEffIntro`/`effKeyword`/
  `effectsThreadPrefix`/`simEffectsThreadPrefix` → `instr*`/`instructions*`,
  `EffProfile`/`MkEffProfile`/`effProfile` → `InstrProfile`/`MkInstrProfile`/
  `instrProfile`, and the local `eff` binder → `instr`.
- The module keeps its name `Experimental.Effect`: it houses the whole effect
  *language* (instructions, static specs, durations, damage, costs,
  replacements), not one sort. One qualified reference inside it
  (`Effect.gatePayer`) is a module qualification and stays.
- `workbench-fused-variants-3` reconciles these names with its own constructor
  collapse; no constructor was renamed here, only sorts and their helpers.

Docs: `docs/contexts/game-model/CONTEXT.md` gains a **Static Spec** project
term beside **Instruction** and cites [CR#609.1] ("text itself is never an
effect") on the Instruction entry; its `_Avoid_` line now also names One-Shot
Effect.

### Undone, and why

- `deckmaste_semantics` and `idris/src/Semantics.idr` are untouched, per the
  round's scope ruling. So are `plugins/builtin` (v1), `deckmaste_migrations`,
  `deckmaste_legacy_render`, `deckmaste_spelling`, `macro_ron` and `xtask`,
  whose `OneShotEffect`/`StaticEffect` occurrences are all v1 grammar names or
  v1 macro-kind strings (`kinds: [OneShotEffect]` in `plugins/builtin`).
- `docs/decisions/` and `docs/tickets/done/` keep the old identifier names.
  They are historical records and the round's doc scope is `docs/contexts/`.
- `idris/scripts/check-effprofile-fusion` keeps its filename (two done tickets
  reference it by name); its identifier strings were updated.

## Landing record

**Gates** (all foreground, from the feature workspace):

- `cargo check --workspace --all-targets` → `Finished dev profile
  [unoptimized + debuginfo] target(s) in 5.72s`
- `cargo fmt` then `cargo fmt --check` → exit 0, no diff (the two
  `can't set imports_granularity/group_imports … nightly` lines are the
  repo's pre-existing rustfmt-config warnings, unrelated to this diff)
- `cargo test --workspace --no-fail-fast` → 127 suites, **6089 passed, 0
  failed, 6 ignored**. No pre-existing failures remain (the three the previous
  landing recorded are gone from trunk).
- `cd idris && rm -rf build && ./scripts/build` → exit 0, **44/44 modules**
  (`44/44: Building Cards (src/Cards.idr)`), **0 Warning lines**, 0 Error lines
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` → `checked 18113 citations against cr.txt (eff.
  2026-08-07); 0 stale`
- `cargo xtask cite audit --diff < /tmp/eit.diff` → `audited 38 citation
  site(s) — read each rule text against its claim`; every rule text read
  against the line citing it. No `cite bless` was needed: every rule this round
  cites was already registered in `cr-citations.lock`. Three citations were
  corrected during the audit read, before the commit: `ReplaceRoll`'s
  [CR#614.3] (which only says replacement effects have no casting restrictions)
  → [CR#614.1a] (which is what makes an "instead" roll-more a replacement
  effect); `PayPips`'s [CR#601.2g] (activating mana abilities) → [CR#601.2h]
  (paying the total cost); and `OutcomeGate`'s [CR#104.3e] (an effect may state
  a player loses) → [CR#101.2] (a "can't" effect takes precedence), the rule
  that actually grounds a gate.
- Ticket grep — `grep -rn "OneShotEffect\|\bInstr\b" crates/deckmaste_core
  crates/deckmaste_engine plugins/builtin_v2 docs/contexts
  idris/src/Experimental` → **14 hits, all of the form
  `deckmaste_semantics::OneShotEffect`**, in three `#[cfg(test)]`-only sites
  (`crates/deckmaste_engine/src/resolve/{fixtures,effect,player_action}.rs`).
  Piping the same grep through `grep -v "deckmaste_semantics::"` gives 0. See
  the deviation below: these are v1-facing INPUT in exactly the sense the gate
  exempts for lowering; no non-test engine code names a v1 type, and neither
  `deckmaste_core` nor `plugins/builtin_v2` nor `docs/contexts` nor
  `idris/src/Experimental` has a single hit.
- Idris pin probe (non-vacuity): `badDoubleStaticRider` in
  `ProofsStatic.idr` had its second `Define X` removed; the build changed from
  green to `Error: Impossible pattern gives an error: Can't match on Oh as it
  must have a polymorphic type` at `ProofsStatic:70`. Reverted; build green
  again.
- `plugins/wizards` needed no regeneration: the generator reads the v1
  `plugins/builtin` data, which this round does not touch.

**Assurance counts:** restored 0; **re-spelled 399** Rust test functions across
42 files (pure type-name propagation — every test's subject, card, and asserted
outcome is unchanged) plus **292 Idris `Unspellable` pins** and **545 ok
witnesses** whose sort ascription changed with the rename; ignored-with-blocker
0 added (6 pre-existing ignores untouched); added 0; **removed 0**. No test was
deleted, weakened, or converted to a `matches!`/`discriminant` check.

**Deviations and additions:**

1. **The workbench's `Effect` sort was renamed too**, not only `StaticEffect`.
   The round brief's parenthetical detailed `StaticEffect`, but
   `Experimental.Effect.Effect` is executable syntax (`DealDamage`, `Fights`,
   `SetStatus`, …) carrying the identical misnomer, and the ticket's "Done
   when" is that `Instruction` is the ONLY name for executable syntax. Its
   plural sorts (`Effects`, `SimEffects`) and derived helper names followed, so
   the tree does not read half-renamed. No constructor was renamed;
   `workbench-fused-variants-3` reconciles from here.
2. **The module `Experimental.Effect` keeps its name.** It houses the effect
   *language* (instructions, static specs, durations, damage, costs,
   replacements), so the name is still accurate; renaming it would churn the
   `.ipkg`, four imports, and every future merge with
   `workbench-fused-variants-3` for no taxonomic gain.
3. **`idris/scripts/check-effprofile-fusion` and `compare-effect-tables`** had
   their `effProfile`/`MkEffProfile`/`effIntro` identifier strings updated to
   the renamed names. They are one-off round tools (referenced only by two done
   tickets, wired to no gate); leaving them would have left them naming a tree
   that no longer exists. Their filenames are unchanged so the done tickets'
   references still resolve.
4. **Doc comments on `Instruction` and `StaticSpec` were rewritten** to carry
   the classification with CR grounding rather than merely renamed —
   `Instruction` now states that text is never an effect ([CR#609.1]) and
   `StaticSpec` states why it is not called an effect, names its two applicable
   subfamilies, and excludes self-replacement ([CR#614.15]). Beyond the
   ticket's letter; without it the classification would live only in this
   ticket.
5. **`docs/contexts/game-model/CONTEXT.md`** gains a **Static Spec** project
   term. The ticket asked for terminology docs to be updated together; the
   glossary had a term for the instruction side but none for the static side,
   which is what let the misnomer stand.
6. **Two local aliases were removed rather than renamed**: the engine's
   `use deckmaste_core::OneShotEffect as Ose` (`resolve/action.rs`) and
   lowering's private `type Instr = deckmaste_core::OneShotEffect`
   (`src/effect.rs`), both now the plain `Instruction`. The ticket bans
   misleading aliases for compatibility; these were exactly that, at file
   scope.
7. **The gate grep is not literally empty** — 14 `deckmaste_semantics::
   OneShotEffect` occurrences remain in engine `#[cfg(test)]` code. The gate's
   exemption ("lowering's v1-facing input may still name v1's `OneShotEffect`
   type") is written for lowering, but the engine's resolve tests are the same
   thing: they construct v1 semantic values and lower them to build fixtures.
   Removing the name would mean hiding it behind an alias — the shape the
   ticket bans — or deleting the fixtures. Left as-is and disclosed.
8. **Zero variants moved.** The classification table above is the finding;
   nothing was relocated, split, or deleted, and no variant was reclassified
   away from where the CR puts it.

**STOP:** none taken. `StaticSpec::Sba` was left in place, unmoved and
unrenamed, under the round's standing instruction (classify it as a
rule-affecting continuous effect / conditional static and leave its removal or
rename to `state-checked-static-home`); it is classified in the table and
nothing about it changed beyond the enclosing type's name.
