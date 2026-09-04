---
needs: []
---
Use the [`Ability` and `Static Ability` glossary meanings](../../contexts/game-model/CONTEXT.md): an Ability is a
rules-defined quality of an Object or Player (or an activated/triggered ability on the stack), and a
Static Ability is one that is simply true while its Object is in the relevant zone — neither
describes a state-based action. Aura's actual `enchant` keyword remains an ordinary Static Ability
([CR#702.5a]), while the Aura type rules remain type rules ([CR#303.4,704.5m]); the two must not be
collapsed. Apply the same classification audit to Equipment, Fortification, Saga, and the other type
and subtype conferrals, not to Aura alone.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

**[design] An SBA is not a static ability — hoist `Sba` out of the `Static`
ability path.** `StaticEffect::Sba` (reachable only as `Ability::Static (Sba …)`) models a
state-based action ([CR#704]) as a kind of static ability ([CR#604]). The two are distinct CR
categories — [CR#704.1] "state-based actions are game actions that happen automatically… don't use
the stack," and [CR#704.1a] pointedly says abilities that watch game state are *triggered*
abilities, "not state-based actions"; [CR#604.1] static abilities "do something all the time." An
SBA is not an ability of any kind, so `Static (Sba …)` is a 704-under-604 category error.

**Where it bites.** `StaticEffect` is the engine's continuous-effect family — `Modify` ([CR#611]),
`Replaces`/`CantHappen` ([CR#614]), `CostModifier`, `AsThough`, and `Sba` ([CR#704]) — all wrapped
by `Ability::Static`. Most members belong: a replacement effect *is* established by a static
ability, a continuous P/T mod *is* one. `Sba` is the outlier — a 704 game action smuggled into the
ability hierarchy. The Aura must-be-attached rule ([CR#704.5m]) is authored (in
`plugins/builtin_v2/macros/stubs/subtypes/enchantment/Aura.ron`) as
`Ability(Innate(Static(Sba(…))))`. That is the shape to remove.

**The clean encoding already exists.**
- `Property::StateBased { condition, effect }` (a `Property` variant DISTINCT from
  `Property::Ability`) is the ability-free "this confers an SBA" flavor. Today every conferring
  subtype uses the `Property::Ability(…Static(Sba…))` form instead — that is the symptom.
- `SbaRule` (`crates/deckmaste_core/src/sba_rule.rs`) is documented as `StaticEffect::Sba` "lifted
  to a global, scoped rule" — SBA-as-game-rule, the taxonomically-correct home (see
  [State-based actions are data](../../decisions/state-based-actions-are-data.md)).

**Fix (design-gated — pair before touching the model).**
1. Remove `Sba` from `StaticEffect` (`crates/deckmaste_core/src/continuous.rs`), so an SBA can no
   longer be reached via `Ability::Static`. The workbench's `Experimental.Effect.StaticEffect` has
   no `Sba` constructor and must not gain one.
2. Re-author the conferring subtypes onto the SBA/state flavors: Aura's falls-off → a
   `Property::StateBased { condition, effect }` (not `Property::Ability(…Static(Sba…))`), and the
   same for the other core-facing subtype stubs. Saga's lore is a `TurnBased`/replacement mechanic,
   not an SBA — keep it out of the SBA path.

**Verify.** `cd idris && ./scripts/build` is green at its module count; workspace green; cite audit
if any CR citation moves. Grep confirms `Sba` is unreachable from `Ability::Static` in
`deckmaste_core` and absent from `idris/src/Experimental/`.

*Exposed by `idris-subtype-open-names` (done): putting conferrals on the subtype value surfaced that
every subtype confer is the `Property::Ability` flavor, i.e. SBAs were being modeled as abilities.
`[design]` — the model change is the soundness gate; open a design dialogue before implementing.*

## As landed

**Step 2 (conferring subtypes) — landed. Step 1 (`StaticEffect::Sba` removal) — STOPped; see below.**

Per subtype (the audit covers every non-creature subtype stub carrying a
`confers:` list — only four exist in `plugins/builtin_v2`):

- **Aura** (`plugins/builtin_v2/macros/stubs/subtypes/enchantment/Aura.ron`) —
  `Ability(Innate(Static(Sba(when: Not(LegallyAttached(This)), then: Move(This,
  Graveyard)))))` → `StateBased(condition: Not(LegallyAttached(This)), effect:
  Move(This, Graveyard))`. [CR#704.5m]/[CR#303.4c] is a [CR#704] game action,
  and [CR#704.1]/[CR#704.1a] put it outside the ability hierarchy, so the
  conferral is now the ability-free `Property::StateBased` flavor. The Enchant
  keyword ([CR#702.5a]) — a genuine static ability — is unaffected and stays
  off the subtype declaration.
- **Equipment** (`artifact/Equipment.ron`) — unchanged, correct as authored.
  `Ability(Innate(Static(May(Attach(what: Ref(This), to: Type(Creature))))))`
  is a [CR#301.5] capability grant, i.e. a static ability, not an SBA. The
  [CR#704.5n] illegal-attachment cleanup is engine-generic (`attachment_sbas`
  pass 2) and is conferred by nothing.
- **Fortification** (`artifact/Fortification.ron`) — unchanged, same reasoning
  under [CR#301.6].
- **Saga** (`enchantment/Saga.ron`) — unchanged. Its two conferrals are a
  [CR#714.3a] enters-replacement and a `TurnBased(at: PrecombatMain)` lore
  counter ([CR#714.3c]); neither is an SBA. The [CR#714.4] sacrifice is not
  declared on the v2 stub at all, so nothing needed moving. Saga's lore stayed
  out of the SBA path as the ticket requires.

**The `StaticEffect` change — NOT made (STOP).** `StaticEffect::Sba` remains.
Its doc comment was corrected instead: it now reads as a state-checked static
([CR#604.1,702.131b]) and explicitly disclaims being the home of a
rules-defined SBA, routing type/subtype rules to `Property::StateBased` and
global rules to `SbaRule`.

**Engine consumers.** `deckmaste_engine::sba::attachment_sbas` pass 1 now
collects its `{when, then}` rows from TWO sources: the derived
`card_types`/`subtypes` `Property::StateBased` conferrals (new — the same read
`counter_state_based_sbas` does for a counter, off the DERIVED characteristics
so a layer-4 grant contributes like a printed subtype), and the
`StaticEffect::Sba` statics an object carries (unchanged — still the spelling
of ascend [CR#702.131b] and of the `plugins/testing` Saga cards). Both feed the
same `removed_by_sba` set, so pass 2's [CR#704.5n,704.5p] cleanup is unchanged.

**Other doc corrections** (statements the change falsified):
`deckmaste_core::Condition::LegallyAttached`, `deckmaste_core::Property::
StateBased` (dropped the stale "engine executes it in stage 3"), and
`deckmaste_core::SbaRule`.

**Undone, and why.** Nothing else. `plugins/builtin` (v1) and
`deckmaste_semantics` keep the old spelling by the ticket's own scope ruling
("their side converges at cutover"); `idris/src/Experimental/` needed no change
(it has neither an `Sba` constructor nor a `Property` type) and was not
touched.

## Landing record

**Gates** (all foreground, from the feature workspace):

- `cargo check --workspace --all-targets` → `Finished dev profile [unoptimized + debuginfo] target(s) in 2.29s`
- `cargo fmt --check` → exit 0, no output
- `cargo test --workspace --no-fail-fast` → 127 suites, **6083 passed, 3
  failed, 6 ignored**. All three failures are PRE-EXISTING and untouched by
  this diff:
  - `deckmaste_migrations::resolve::tests::ascend_gate_const_matches_canonical_condition`
    — the sibling fix round's subject.
  - `deckmaste_noncanon::strategy::tests::sped_red_strategy_parses_for_both_seats`
    and `deckmaste_noncanon::game::tests::sped_red_vs_stompy99_completes` —
    both panic on `crates/deckmaste_noncanon/strategies/sped_red.ron` line 8,
    "Unexpected variant named `Kind` in enum `Predicate`". The data file and
    `Predicate` are both outside this diff.
- `cd idris && rm -rf build && ./scripts/build` → exit 0, **44/44 modules**
  (`44/44: Building Cards (src/Cards.idr)`), **0 Warning lines**
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` → `checked 18072 citations against cr.txt (eff.
  2026-08-07); 0 stale`
- `cargo xtask cite audit --diff < /tmp/sba.diff` → `audited 33 citation
  site(s)`; every rule text read against its claim. No `cite bless` was needed —
  every rule cited by this round was already registered in `cr-citations.lock`.
- Ticket greps: `grep -rn "Sba\|StateBased" idris/src/Experimental/` → empty
  (workbench still has no `Sba` and gains none). `grep -rn "Static(Sba"
  plugins/builtin_v2/` → empty. `Sba` IS still reachable from `Ability::Static`
  in `deckmaste_core` — that gate is NOT met, by the STOP below.
- `plugins/wizards` needed no regeneration: the generator reads the v1
  `plugins/builtin` conferrals, which are unchanged.

**Assurance counts:** restored 0; re-spelled 0; ignored-with-blocker 0; added 2
(`state_based_conferral_moves_unattached_aura_to_graveyard`,
`state_based_conferral_grants_no_ability`, both in
`deckmaste_engine::sba::tests`); removed 0. One expected-value edit, not a
removal: `deckmaste_construction_core::builtin_v2_noncreature_subtypes::
rules_defined_conferrals_stay_on_their_subtype_declarations` keeps asserting
Aura's conferral, re-spelled to the new RON. The existing
`sba_attach_unattached_aura_goes_to_graveyard` and
`ability_less_aura_still_graveyards` still cover the `StaticEffect::Sba`
spelling, which still exists and still has live v1 data behind it, so neither
was retired.

**Deviations and additions:**

1. Added the two engine tests above (beyond the ticket's letter) — without them
   the new `Property::StateBased` execution path would ship untested, and the
   second one is the only positive proof of the taxonomic claim (the conferral
   grants no ability).
2. Added the engine execution path for type/subtype-conferred
   `Property::StateBased`. The ticket names only the data re-authoring, but the
   v2 stub's new shape would otherwise be inert. The shape is not new: it
   mirrors `counter_state_based_sbas`, which already executes counter-conferred
   `Property::StateBased`.
3. Corrected four core doc comments that the change falsified
   (`StaticEffect::Sba`, `Condition::LegallyAttached`, `Property::StateBased`,
   `SbaRule`). No behavior change.
4. NOT done: any edit to `plugins/builtin` (v1), `deckmaste_semantics`,
   `deckmaste_lowering`, or `idris/`. Scope ruling + no change needed.

**STOP — step 1 (`remove Sba from StaticEffect`) not performed.**

Two independent blockers, either one sufficient:

1. **The v1 lowering path has no target.** `deckmaste_semantics::StaticEffect::
   Sba` is explicitly out of scope ("deletion-bound and untouched"), and the
   `Lower` trait is total (`fn lower(self) -> Self::Target`, one arm per
   variant, no fallible/drop arm). `crates/deckmaste_lowering/src/continuous.rs`
   must therefore produce SOME `deckmaste_core::StaticEffect` for the semantic
   `Sba`. Core cannot lose a variant its still-frozen source grammar keeps.
   Nothing in `Property`/`SbaRule` is a `StaticEffect`, so no re-target exists
   without widening one of them.
2. **Ascend is a genuine static ability, and `Sba` is its only core spelling.**
   [CR#702.131b]: "Ascend on a permanent represents a static ability. It means
   'Any time you control ten or more permanents and you don't have the city's
   blessing, you get the city's blessing for the rest of the game.'" It is
   authored as `Static(Sba(...))` in `plugins/builtin/macros/keyword/
   Ascend.ron`, constructed in `deckmaste_engine::sba::tests`, and pinned by
   `deckmaste_plugin::keywords::ascend_macro_expands_to_static_sba` against the
   `ASCEND_GATE` constant in `deckmaste_migrations::resolve`. Re-homing it to
   `Property::StateBased` (it is conferred by nothing) or to `SbaRule` (it is a
   card ability, not a rules-defined game rule) would commit the mirror-image
   category error the ticket is fixing — [CR#704.1a] does not license moving a
   static ability into the SBA machinery. Making it a triggered ability
   contradicts [CR#702.131b] outright.
   `plugins/testing/cards/Test Saga.ron` and `Test Saga Range.ron` are a third,
   smaller instance: card-level `Static(Sba(...))` for the [CR#714.4] sacrifice.

   The ticket's premise — that `StaticEffect::Sba` is only ever a [CR#704]
   game action smuggled into the ability hierarchy — is therefore true of the
   CONFERRAL sites (fixed here) and false of the card-level ones. Resolving
   that requires a coordinator ruling on where a state-checked static ability
   lives, so it is raised rather than invented.

No other STOP was taken.
