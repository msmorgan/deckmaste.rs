---
needs: [effect-instruction-taxonomy]
---
**Distinguish the CR Ability categories from orthogonal classifications and
the authored keyword surface.** Use the [`Ability` family in the domain
glossary](../../contexts/game-model/CONTEXT.md). [CR#113.3] has four general Ability categories:
spell, activated, triggered, and static. Mana Ability is an orthogonal
classification of some activated and triggered Abilities ([CR#113.4]); it is
not a fifth sibling kind.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Remove `Ability::Mana` and express mana classification on the applicable
activated or triggered Ability. Preserve `Ability::Keyword(...)` as an authored
sibling in the core-facing RON: it is the clean named container that lets
primitive and composite keyword definitions appear beside `Static(...)`, and it
does not claim that Keyword is a fifth CR Ability category.

Rename the engine's directly implemented keyword forms from `intrinsic` to
`primitive`. Reserve Intrinsic Ability for the CR usage, including the mana
Abilities supplied by basic land types ([CR#305.6]). Keep the existing
composite keyword shape—name plus nested Ability definitions—so declarations
such as a future `TripleThreat = [FirstStrike, Lifelink, Exalted]` remain
expressible without another RON wrapper. Apply the vocabulary consistently in
`deckmaste_core`, the engine, lowering's core-facing output, the Idris
workbench (`idris/src/Experimental/`), keyword data, classifiers, comments, and
policy documentation.

Overlaps `workbench-ability-kind-fold`, which folds the workbench's ability
kind into `Object` with an `AbilityP` payload; whichever runs second reconciles
the names.

Done when: `Ability` has no `Mana` variant and mana classification rides on the
activated or triggered Ability that carries it; no `intrinsic` spelling
survives for the directly implemented keyword forms; the workspace is green and
`cd idris && ./scripts/build` is green at its module count.

## As landed

### Mana classification — a derived classifier, no authored field

`Ability::Mana` and the `ManaAbility` wrapper are **deleted**. Lowering now
emits plain `Ability::Activated` / `Ability::Triggered`, and mana-ness is asked
of the ability:

- `Ability::mana_profile() -> Option<ActivatedManaProfile>` — `Some` exactly
  when this is an activated mana ability ([CR#605.1a]).
- `Ability::is_activated_mana_ability()` / `is_triggered_mana_ability()`
  ([CR#605.1a] / [CR#605.1b]) and `is_mana_ability()` ([CR#605.1]).
- `Ability::mana_profile_for_modes(&[Uint])` keeps its name and its exact
  semantics; it now derives the mode rows instead of reading authored ones.

**Why derived rather than a positional field.** [CR#605.1a..605.1b] state the
classification entirely in terms of what the ability *is* — target-freedom,
whether it could add mana as it resolves, loyalty-ness, and (for a trigger) the
cause it triggers from. All four are structure already present in the compiled
`ActivatedAbility` / `TriggeredAbility`. The decisive evidence was that
lowering's `classify_activated` / `classify_triggered` read **only core types**
(`core::ActivatedAbility`, `core::Instruction`, `core::Action`,
`core::EventFilter`): the judgment was never lowering-specific. The classifier
therefore moved verbatim into `deckmaste_core` (`ManaFacts`,
`region_mana_facts`, `effect_mana_facts`, `effect_action_facts`,
`triggered_by_mana`, all private), and lowering shed 176 lines. An authored
flag would have had to be re-authored by hand in every core-facing RON stub and
test fixture, and could disagree with the ability it sits on; a derived
classifier cannot.

`ActivatedManaProfile` and `ManaModeClass` survive as the **result** of that
derivation, not as authored data: their `Serialize`/`Deserialize` derives are
dropped, since neither appears in RON any more. The engine's `ManaAction`
runtime record still carries a `profile`, now the derived value.

[CR#605.2] falls out for free and is stated on `mana_profile`: derivation reads
structure only, so an ability the board currently stops from producing mana is
still a mana ability.

**One [CR#605.1a] clause is deliberately not evaluated** — "its cost and effect
don't move any card to or from a library". That was already true before this
round (a pinned lowering test asserts a library-moving mode cost does not
reclassify the ability); the move preserved the behaviour exactly and the
omission is now documented on `mana_profile` rather than left implicit.

`Ability::Keyword(...)` is untouched — still the authored sibling, still name
plus nested ability definitions, so `TripleThreat = [FirstStrike, Lifelink,
Exalted]` stays expressible without another RON wrapper.

### intrinsic → primitive

No identifier and no RON key ever spelled the keyword classification
`intrinsic`; it lived only in doc comments and policy prose, so this is a
vocabulary change with no data or wire change. Renamed:

- `crates/deckmaste_core/src/keyword.rs` — module doc and eight doc comments
  (`the five implemented true primitives`, `Every PRIMITIVE variant`,
  `Primitives only`, …); its classification pointer now names
  `docs/keyword-policy.md §1..§2` and says outright that the descriptive
  catalog spells the class *intrinsic*, a name [CR#305.6] already owns.
- `crates/deckmaste_core/src/ability.rs` — `Ability::Keyword`'s doc.
- `crates/deckmaste_engine/src/{combat,activation,derive}.rs` — five doc
  comments about the enum's directly implemented forms.
- `docs/keyword-policy.md` — title (`Keyword primitives policy`) and 20 sites,
  Part I and Part II alike, plus a header note mapping the project term onto
  the descriptive catalog's.
- `docs/decisions/minimal-core-data-driven-rules.md` (`a primitive keyword
  action`), `docs/engine-adrs.md` (`vigilance is primitive`),
  `plugins/testing/cards/README.md` (two).

Three further sites used `intrinsic` as the synonym for *engine-primitive* that
`docs/contexts/game-model/CONTEXT.md` explicitly tells authors to avoid —
`core/src/effect.rs` (twice, "a single intrinsic instruction") and
`core/src/continuous.rs` (`BecomeBasicLandType`'s "One intrinsic") — and were
renamed with it.

Intrinsic Ability is now reserved for the CR sense throughout core, the engine,
`plugins/builtin_v2`, `docs/contexts` and `docs/decisions`.

### Workbench — nothing to mirror

`Experimental.Effect.AbilityAt` has exactly `KeywordAbility`, `Activated`,
`Triggered`, `Static`, `Spell` (plus the `MayBeginOnBattlefield` /
`AlsoForKeywords` / `ItalicHead` wrappers): **no mana kind**, and
`Words.AbilityClass` (`AnyOnStack`/`AnyActivated`/`AnyTriggered`/
`LoyaltyClass`/`KeywordClass`) has no mana member either. Mana is already the
orthogonal shape this ticket asks for — `Phrase.IsManaAbility : Predicate bs
Ability`, a description-side predicate over an ability, used only inside
`Predicate` positions in the card bench. No `intrinsic`/`primitive` spelling
exists anywhere under `idris/src/Experimental/`. The tree was therefore left
untouched and the build re-run as a regression check. Nothing here makes
`workbench-ability-kind-fold` harder: no constructor, sort, or macro moved.

### Undone, and why

- `deckmaste_semantics` and `idris/src/Semantics.idr` untouched, per the scope
  ruling; lowering still reads v1's `Ability::Mana` on its input side.
- `docs/rules-taxonomy.md` §5/§10 keep `intrinsic`. That file is the
  *descriptive* derivation, explicitly pinned to the mtg-rules skill catalog
  (v1.10.0), whose own vocabulary is "intrinsic"; renaming it would
  desynchronise the repo from its pinned source. `docs/keyword-policy.md` — the
  prescriptive companion — now carries the mapping between the two names.
- The `core-intrinsic-keywords-policy` ticket name is a proper noun and stays.
- The region term "a region's intrinsic parameters" (`core-explicit-regions`,
  `target.rs`, `resolve/effect.rs`) is a different sense — the parameters a
  region owns versus its captures — and was left alone, as was a
  verbatim-quoted user ruling in `target.rs` about `Selection::FromHand`.

## Landing record

**Gates** (all foreground, from the feature workspace):

- `cargo check --workspace --all-targets` → clean, 0 errors, 0 warnings
- `cargo fmt` then `cargo fmt --check` → exit 0, no diff (the
  `can't set imports_granularity/group_imports … nightly` lines are the repo's
  pre-existing rustfmt-config warnings)
- `cargo test --workspace` → 127 suites, **6089 passed, 0 failed, 6 ignored**
  (identical to the pre-round baseline)
- `cd idris && ./scripts/build` → exit 0, **44/44 modules**
  (`44/44: Building Cards (src/Cards.idr)`), **0 Warning lines**
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` → `checked 18182 citations against cr.txt (eff.
  2026-08-07); 0 stale`
- `cargo xtask cite bless` → registered exactly one new rule, [CR#605.2] ("A mana
  ability remains a mana ability even if the game state doesn't allow it to
  produce mana"), which is what the line citing it claims; the lock diff is
  `+ "[CR#605.2]": …` and nothing else — no prune.
- `cargo xtask cite audit --diff < /tmp/akt.diff` → `audited 19 citation
  site(s)`; every rule text read against the line citing it, no correction
  needed.
- `grep -rn "Ability::Mana" crates/deckmaste_core crates/deckmaste_engine
  plugins/builtin_v2` → **0**
- `grep -rniw "intrinsic" crates/deckmaste_core crates/deckmaste_engine
  plugins/builtin_v2 docs/contexts docs/decisions` → 27 hits, **none for a
  keyword form**: the CR sense (a face's intrinsic printed abilities,
  [CR#305.6]/[CR#306.5b] intrinsic abilities, the glossary's own
  **Intrinsic Ability** entry and its two `_Avoid_` lines), the
  `core-explicit-regions` region-parameter sense, `english-v2-rewrite`'s
  intrinsic-vs-contextual Agreement, and one quoted user ruling.
- `plugins/wizards` needed no regeneration: it is untracked generated data and
  contains no `Ability::Mana` / `Mana(Activated…)` RON; the plugin corpus
  suites that read it are green.

**Numbers before → after:** `Ability` variants 7 → 6 (`Mana` removed); core
public mana API `as_mana` → `mana_profile` + three predicates;
`deckmaste_lowering/src/ability.rs` 1149 → 973 lines (the classifier moved, not
deleted); `deckmaste_core/src/ability.rs` 404 → 572. Locks: `cr-citations.lock`
+1 rule, `cr-coverage.lock` unchanged. Construction count: not applicable (no
english_v2 surface touched).

**Assurance counts:** restored 0; **re-spelled 10** test functions — nine
lowering classification tests (`classifies_targetless_activated_mana_ability_
while_lowering`, `reversibility_does_not_change_mana_classification`,
`mana_produced_by_a_binder_participates_in_classification`,
`inert_reveal_until_body_does_not_establish_mana_production`,
`does_not_classify_targeted_or_loyalty_mana_producers`,
`compiles_modal_mana_profile_for_announced_modes`,
`modal_mana_profile_does_not_treat_reversibility_as_classification`,
`classifies_each_causal_triggered_mana_ability_while_lowering`,
`classifies_sacrifice_cost_mana_ability_without_external_replacement_effects`)
whose variant match became a `mana_profile()` / `is_*_mana_ability()` assertion
on the same lowered value, and one engine test (see deviation 2) — plus one
shared engine fixture (`modal_payment_fixture_with_ordinary_mode`) that now
**asserts** the derived mode rows it used to author; ignored-with-blocker 0
added (the six pre-existing ignores untouched); added 0; **removed 0**. No test
was deleted, weakened, `#[ignore]`d, or converted to a
`matches!`/`discriminant` check; the diff contains zero added and zero removed
`#[test]` attributes. 36 `Ability::Mana` constructions across seven files were
rewritten mechanically (script + `cargo fmt`) to `Ability::Activated` /
`Ability::Triggered`, and every remaining read site by hand.

**Deviations and additions:**

1. **`deckmaste_plugin` was edited** although the brief's crate list stops at
   core/engine/lowering: `src/plugin.rs`, `tests/corpus_identity.rs` and
   `tests/tokens.rs` matched or built `Ability::Mana` and could not compile
   otherwise. Each edit deletes a now-impossible arm or retargets a
   construction; no behaviour changed.
2. **One engine test's fixture was re-spelled onto a different [CR#605.1a]
   exclusion.** `bare_nonmana_ability_mana_added_trigger_resolves_immediately`
   built its source as a bare `Ability::activated` whose untargeted effect adds
   mana — a value that *asserts* non-mana-ness while satisfying every
   [CR#605.1a] criterion, and one lowering could never produce. Derivation
   correctly classifies it as a mana ability, so the fixture's premise was
   unrepresentable. Re-spelled with `UseLimit::LoyaltyOncePerTurn` — the
   exclusion [CR#605.1a] names explicitly, and a real shape (Chandra, Torch of
   Defiance's "+1: Add {R}{R}") — keeping the test's subject and asserted
   outcome exactly: a [CR#605.1b] `ManaAdded` trigger is a mana ability and
   resolves before priority even when the ability that added the mana is not.
   This is the round's one genuine behaviour change, and it is a correction:
   the old fixture encoded a rules-impossible core value.
3. **`docs/keyword-policy.md` was renamed** even though the brief scoped docs
   to `docs/contexts/` and `docs/decisions/`. It is the ticket's "policy
   documentation" — the normative file for exactly this classification, titled
   "Keyword intrinsics policy" — and leaving it would have left the code saying
   *primitive* while the doc it cites says *intrinsic*. Its Part II keyword
   *actions* were renamed with Part I, matching the `docs/decisions` line the
   gate did require. `docs/rules-taxonomy.md` was deliberately not renamed (see
   "Undone").
4. **Three non-keyword `intrinsic` uses were fixed** (`core/src/effect.rs` ×2,
   `core/src/continuous.rs`): each was the engine-primitive synonym the
   glossary tells authors to avoid. Beyond the ticket's letter, three words,
   no behaviour.
5. **`Serialize`/`Deserialize` were dropped from `ActivatedManaProfile` and
   `ManaModeClass`.** They are derived facts now, never authored, so a wire
   form would be a shape nothing writes. Confirmed no RON in `plugins/` or
   `crates/*/tests` spells either.
6. **`ManaFacts` and the effect walk moved into `deckmaste_core` unchanged.**
   Same arms, same `RevealUntil`-is-inert comment, same neutral element — so
   the round cannot have altered classification by rewriting the traversal.

**STOP:** none taken. Two judgment calls are disclosed above rather than
stopped on (deviation 3's doc boundary, deviation 2's fixture re-spelling);
neither contradicts a recorded ruling — the glossary already reserves
*Intrinsic Ability* for the CR sense and already names *Primitive Keyword
Ability* as the project term.
