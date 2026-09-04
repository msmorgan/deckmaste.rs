---
needs: []
---
**Home the Saga sacrifice on `Property::StateBased`, not a card-level
static.** Residue of `state-checked-static-home` (2026-09-04). The testing
Sagas (`plugins/testing/cards/Test Saga*.ron`) author the [CR#714.4]
sacrifice as a card-level static, so after the rename it lowers to
`Static(ConditionallyDo(…))` — a rules-defined state-based action still
wearing a static ability. [CR#714.4] says the sacrifice is a state-based
action that doesn't use the stack; its home is a `Property::StateBased`
conferral on the Saga subtype (the shape Aura's [CR#704.5m] rule took in
`idris-sba-not-a-static-ability`) or an `SbaRule`. The v2 Saga stub
declares neither. Ascend is not in this bucket: [CR#702.131b] makes it a
static ability outright.

Fix: confer the [CR#714.4] rule on the Saga subtype in the v2 stub (or the
engine's SBA rule set), drop the card-level static from the test Sagas,
keep every Saga test green (re-spelled, never deleted), and make lowering
refuse a card-level static that restates a rules-defined SBA where it can
recognise one. Scope: workbench, core, engine, lowering's core-facing
output; `deckmaste_semantics` untouched.

Size: S–M. Done when: no test Saga carries the sacrifice as a static;
the engine still sacrifices a finished Saga; workspace tests green.
Standard constraints apply.

## As landed

**The [CR#714.4] sacrifice is a `Property::StateBased` conferral of the Saga
subtype, and the rule names no chapter number — it reads the card's own.**

- **`deckmaste_core::Count::GreatestWatchedThreshold(Reference)`** (new): the
  greatest threshold a referenced object's own abilities watch a crossing of.
  [CR#714.2d]'s final chapter number, read generically — a chapter ability's
  intervening-if IS a `Condition::Crossed` gate ([CR#714.2b]), so the greatest
  entry across those thresholds is that number, and no chapter ability means 0.
  This is what lets ONE subtype row serve every Saga; the previous rounds'
  "a generic registry row cannot name a card's own final chapter number" no
  longer holds.
- **`plugins/builtin_v2/macros/stubs/subtypes/enchantment/Saga.ron`** gains a
  third conferral beside the [CR#714.3a] replacement and the [CR#714.3c]
  turn-based increment:
  `StateBased(condition: And([Compare(GreatestWatchedThreshold(This), AtLeast, 1),
  Compare(CounterCount(This, LoreCounter), AtLeast, GreatestWatchedThreshold(This))]),
  effect: Sacrifice(You, This))`. The first conjunct is [CR#714.4]'s "with one
  or more chapter abilities" ([CR#714.2d]: none means a final chapter number
  of 0).
- **Engine.** `Count::GreatestWatchedThreshold` evaluates in both count
  evaluators (`resolve::count`, off the layered view; `layer::eval_count`, off
  the derivation in progress) through one shared `layer::watched_thresholds`
  reader. The SBA sweep needed no change: `attachment_sbas` pass 1 already
  collects type/subtype `Property::StateBased` rows off the DERIVED
  characteristics, so the Saga row rides the same path the Aura's does.
- **`plugins/testing/cards/Test Saga.ron` / `Test Saga Range.ron`** no longer
  carry the sacrifice; both are now chapters only.
- **Lowering refuses the card-level spelling.** `deckmaste_lowering`'s
  `CardFace` arm rejects a card-level state-checked static ([CR#604.1]
  `ConditionallyDo`) whose instruction removes its own object — sacrifice
  ([CR#701.21a]), a zone change ([CR#400.7]), or `Cease` ([CR#704.5d]). Every
  rules-defined removal of that shape is a [CR#704] game action, not an ability
  ([CR#704.1,704.1a]). Ascend ([CR#702.131b]) passes: its instruction grants a
  designation and removes nothing.
- **Comment corrections** the change falsifies: `plugins/builtin/macros/types/
  enchantment/Saga.ron` and `plugins/builtin/macros/ability/Chapter.ron` both
  said the sacrifice is spelled card-side.

Unchanged, deliberately: `deckmaste_semantics`, `idris/`, `plugins/builtin`'s
Saga conferral list (see the STOP), `plugins/builtin/rules/sba/`.

## Landing record

**Gates** (all foreground, from the feature workspace):

- `cargo check --workspace --all-targets` → `Finished dev profile [unoptimized + debuginfo] target(s) in 2.34s`
- `cargo fmt --check` → exit 0, no output (nightly-only-option warnings filtered)
- `cargo clippy --workspace --all-targets` → `Finished dev profile [unoptimized + debuginfo] target(s) in 0.15s`; **11 warnings, all pre-existing** (10 × `unneeded_wildcard_pattern` in `deckmaste_lowering/src/card.rs`, 1 × `items_after_test_module` in `ability.rs`). The round introduced one `collapsible_match` and removed it before landing; it introduces none.
- `cargo test --workspace` → 127 suites, **6119 passed, 0 failed, 6 ignored**
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`
- `cargo xtask cite check` → `checked 14247 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `cargo xtask cite audit --diff < /tmp/round.diff` → `audited 50 citation site(s)`; each rule text read against its claim. No `cite bless` was needed — every rule this round cites is already registered in `cr-citations.lock`.
- `cargo xtask generate plugins/wizards` → re-ran; tree hash identical before and after (`d9f0e03638242e2e8cf5e790f018f2ff`). Expected: the generator's inputs are v1 semantic text, and this round changes only comments there.

**Assurance counts:** restored 0; re-spelled 1
(`deckmaste_construction_core::builtin_v2_noncreature_subtypes::rules_defined_conferrals_stay_on_their_subtype_declarations`
— same subject and same asserted outcome, expected RON re-spelled to the
three-conferral list); ignored-with-blocker 0; added 5; removed 0.

Added: `deckmaste_lowering::card::tests::a_self_removing_state_checked_static_is_refused`
and `…::a_state_checked_static_that_removes_nothing_compiles` (the refusal and
its Ascend counter-case); `deckmaste_engine::sba::tests::state_based_conferral_sacrifices_a_finished_saga`,
`…::a_chapter_range_finishes_at_its_greatest_threshold` (a range's greatest
member, not the chapter count, is the final chapter number — [CR#714.2c,714.2d]),
and `…::a_chapterless_saga_is_not_sacrificed` (layer-6 stripping, [CR#613.1f]).

**Deviations and additions:**

1. `Count::GreatestWatchedThreshold` is beyond the ticket's letter. Without a
   term for [CR#714.2d]'s final chapter number, a conferral cannot express
   [CR#714.4] at all and the ticket's "confer it on the subtype" is
   unreachable. It names no lexeme or card: it reads `Condition::Crossed`, a
   declared core feature.
2. Three engine tests rather than one — the range case and the chapterless case
   are the two ways a chapter COUNT would silently pass a wrong answer.
3. The refusal recognises a SHAPE, not a registry entry: lowering holds no
   rules-SBA registry, so "where it can recognise one" is the self-removal
   family. Ascend's counter-case is asserted so the boundary is pinned.
4. Two v1 comment corrections (no data change) — both stated that the
   sacrifice is spelled card-side.

**STOP — the conferral does not reach the v1 plugin data path.**

The engine's subtype registry is built from `plugins/builtin`, which is read as
`deckmaste_semantics`, and this ticket's scope ruling holds that crate
untouched. A generic Saga row must name the card's own final chapter number;
no `deckmaste_semantics::Count` expresses it, and adding one is the exact
change the ruling forbids. Resolved by declaring the row on the v2 stub and
executing it from core: a Saga loaded through `Plugin::load(plugins/builtin)`
carries no [CR#714.4] conferral until the v2 registry replaces v1 at cutover.

Nothing in the corpus regresses. `plugins/wizards` graduates no Saga permanent
(its one Saga mention, Keldon Warcaller, targets one), and the only Sagas in
tree are the two `plugins/testing` fixtures whose card-level static this round
removes; no test asserted their sacrifice. The capability is proven on the
core-native path by the three engine tests above.

Residue for the coordinator to route: the v1-side conferral (or its cutover
replacement) belongs to `engine-sagas` (planned), which already owns
"final-chapter sacrifice". [CR#714.4]'s second clause — "isn't the source of a
chapter ability that has triggered but not yet left the stack" — is also
unexpressed, exactly as it was in the card-level spelling this round replaces;
it belongs to the same ticket.

No other STOP was taken.
