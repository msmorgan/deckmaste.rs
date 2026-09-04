---
needs: []
---
**Close six independent core gates: attachment, the storm window, nested tap
costs, `SameAs` counters, three facts/expansion mismatches, and the open cost
predicates.** Cleanroom review 3, 2026-09-04, findings U4, U6, U7, U8, U13 and
D-Q14. Six independent edits.

- **U4 — `Words.attachedCheckOk` is constantly `True`.** Every row
  (`Enchanted`, `Equipped`, `Fortified`) returns `True`, so
  `Phrase.Predicate.IsAttached`/`AttachedBy`'s obligation proves nothing and
  `IsAttached Fortified : Predicate [] Object` admits any host.
  `Words.attachHeadOk` already encodes the rule: an Equipment can't legally be
  attached to anything that isn't a creature [CR#301.5], a Fortification to
  anything that isn't a land [CR#301.6]. Either restate the two obligations
  over `attachHeadOk` against the described noun's head, or delete
  `attachedCheckOk` and the obligations; pin a Fortification on a creature.
- **U6 — `Macros.stormExpansion` counts the wrong window.** It counts
  `SpellCast … ThisTurn`, but storm copies the spell for each other spell that
  was cast *before it* this turn [CR#702.40a], so spells cast in response to
  the trigger are counted today. Add `Words.Lookback.EarlierThisTurn` with its
  rows in `sameWindow` and `Events.sameLookback`, use it in the expansion, and
  pin `ThisTurn` in the storm body.
- **U7 — a nested `Compound` evades the one-tap gate.**
  `Effect.selfTapPayment (Compound _) = False`, so `costTapOnce` counts only
  top-level tap symbols; `ProofsMana.nestedCompoundCost` is an *admitted*
  activation cost carrying two `{T}`, one nesting level below the
  `badDoubleTapCost` pin. A permanent that's already tapped can't be tapped
  again to pay the cost [CR#107.5]. Count recursively.
- **U8 — `Effect.CounterKindSource.SameAs` bypasses scope and ignores the
  amount.** `counterSourceScope (SameAs _) k = counterHolderKind k` holds for
  any object or player, so `PutCounters (Lit 1) (SameAs thisCreature) (target
  AnyPlayer)` is admitted — a creature's `+1/+1` counters put on a player,
  which `ProofsCounters.badGetsBoostCounter` refuses for `PrintedKind`, since
  a `+X/+Y` counter modifies an object's power and toughness [CR#122.1a]. And
  `PutCounters (Lit 7) (SameAs …)` is admitted although "the same number"
  carries no literal. `SameAs` inherits the source noun's scope; the amount
  slot is absent, or pinned, for `SameAs` and `ThoseKinds`.
- **U13 — three mismatches of the same class.** `Macros.fateseal` is
  `Macros.scry` over the agent's own library, but to fateseal N is to look at
  the top N cards of *an opponent's* library [CR#701.29a], so
  `Cards.Keyword.spinIntoMyth` reads "an opponent scries 2"; give fateseal its
  own expansion over the `"Fateseal"` label with a library-possessor slot.
  `Macros.monstrosity` narrows [CR#701.37a]'s "this permanent" to
  `thisCreature`. The generated `FactsGen` row for `SplitSecond` leaves
  `functionsOnStack` `False` although split second functions while the spell
  is on the stack [CR#702.61a] — fix the generator's source, not the generated
  file.
- **D-Q14 — the cost predicates close silently.** `Effect.costActionOk`,
  `costPaidByYou`, `costOffBattlefield` and `costTapOnce` each end in
  `_ = False`, so a new `Instruction` constructor becomes "not a cost" without
  anyone noticing — the same shape U7 hides a nested `Compound` behind. Make
  them case-complete over `Instruction`, so a new row is a compile error.

Size: M. Done when: each of the six gates refuses its probe term with a
same-module positive twin; fateseal reads an opponent's library and
`spinIntoMyth` reads as printed; the `SplitSecond` row is regenerated from the
stub; the four cost predicates are case-complete; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## As landed

- **U4 — `attachedCheckOk`.** Deleted `Words.attachedCheckOk` and the two constant
  obligations on `Phrase.Predicate.IsAttached`/`AttachedBy`; the rule is restated over
  `attachHeadOk` against the described head as a new `noAttachHeadClash` conjunct of
  `contradictionFree` (the `And` row's `ContradictionFree` obligation) plus a new
  `AttachFits n p` obligation on `Matches` for the noun-headed case. New helpers
  `attachWordsIn`/`attachWordsInAll`/`attachTysOk`/`attachWordsOk`. Pin
  `ProofsDescription.badFortifiedCreature` ("a fortified creature"), twin
  `okFortifiedLand`.
- **U6 — storm window.** Added `Words.Lookback.EarlierThisTurn` with its rows in
  `sameWindow` and `Events.sameLookback`; `Macros.stormExpansion` now counts
  `EarlierThisTurn`. Evidence is the checked equality
  `ProofsFaces.stormCountsEarlierThisTurn` — no obligation anywhere consumes a
  `Lookback`, so a `ThisTurn` body cannot be refused by an `Unspellable` (STOP below).
- **U7 — nested tap costs.** `Effect.selfTapPayment : Cost -> Bool` replaced by
  `selfTapUses : Cost -> Nat`, recursing through `Compound` and taking `max` over
  `EitherCost`; `selfTapCount` sums it, so `costTapOnce` counts taps at every nesting
  depth. `ProofsMana.nestedCompoundCost` re-spelled from an admitted witness into a pin
  (twin `okSingleTapCost`, already beside it).
- **U8 — `SameAs`.** `counterSourceScope (SameAs _) k = kindLte k Object`; new
  `kindAmountOk`/`optKindAmountOk` obligations on `PutCounters`, `MoveCounters` and
  `WithCounters` hold the amount slot to the unit placeholder under `SameAs`. Pins
  `ProofsCounters.badSameKindsOnPlayer` and `badSameKindsSevenEach`, twin
  `okSameKindsOnCreature`. `ThoseKinds` deliberately left free (STOP below).
- **U13 — three mismatches.** `Macros.fateseal` has its own expansion over the
  `"Fateseal"` label with a library-possessor slot, built on new
  `lookedSliceOf`/`lookedScopeOf`/`lookedSliceCountOf`/`lookedSlicePlurOf`/`lookAndSortOf`
  that take the looker and the library's owner separately; the four-argument helpers and
  `lookAndSort` are now one-line specialisations, so `scry` and `surveil` are unchanged.
  `Cards.Keyword.spinIntoMyth` reads `fateseal You anOpponent (Lit 2)`.
  `Macros.monstrosity` is over `thisPermanent`. The `SplitSecond` overlay row in
  `crates/xtask/src/facts.rs` carries `functions_on_stack: true`; `FactsGen.idr`
  regenerated.
- **D-Q14 — cost predicates.** `Effect.costActionOk` is case-complete over all 77
  `Instruction` rows (the 20 added rows all return `False`, exactly what `_ = False` gave).
  The other three predicates were already case-complete over their argument type
  `Cost bs`; see the STOP below.

## Landing record

Numbers before → after: build modules 46/46 → 46/46; pin definitions in
`Experimental/Proofs*.idr` 611 → 615; positive twins added 2; checked assertions added 1.
Diff: 12 files, 320 insertions, 83 deletions.

Gate lines (all foreground):

- `cd idris && ./scripts/build` — `46/46: Building Cards (src/Cards.idr)`, exit 0, 0
  Warning lines, 1m49s from a cleared `build/`.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` — `checked 14366 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` — `audited 13 citation site(s)`;
  every rule text read against its claim. No `cite bless` needed: all nine rules cited
  ([CR#107.5], [CR#122.1a], [CR#301.5], [CR#301.6], [CR#701.22a], [CR#701.29a],
  [CR#701.37a], [CR#702.40a], [CR#702.61a]) were already in `cr-citations.lock`.
- `cargo xtask facts check` — `FactsGen.idr is up to date` (after `facts generate`).
- `cargo xtask facts labels` — exit 0.
- `cargo test -p xtask` — `test result: ok. 453 passed; 0 failed; 1 ignored`.
- `cargo fmt --check` — clean.

Pin probes (each mis-stated once, message read, then restored):
`stormCountsEarlierThisTurn` → `Mismatch between: ThisTurn and EarlierThisTurn`;
`nestedCompoundCost` → `nestedCompoundCost Oh is not a valid impossible case`;
`badSameKindsOnPlayer`, `badSameKindsSevenEach`, `badFortifiedCreature` → the same
`... Oh is not a valid impossible case` for each.

Assurance counts: restored 0; re-spelled 2 (`ProofsMana.nestedCompoundCost`, from an
admitted witness to a pin; `xtask facts::tests::the_stack_column_is_the_grant_gate_and_the_regime_column_is_the_body_axis`,
whose `functionsOnStack ⇒ regime == AtCasting` invariant encoded the fact [CR#702.61a]
corrects — re-spelled to admit the one static-on-stack keyword and given a positive
assertion on the new `SplitSecond` row); ignored 0; added 6 (`okFortifiedLand`,
`badFortifiedCreature`, `okSameKindsOnCreature`, `badSameKindsOnPlayer`,
`badSameKindsSevenEach`, `stormCountsEarlierThisTurn`); removed 0.

Deviations and additions:

- U4's obligation was restated on `And` (`contradictionFree`) and `Matches`, not on
  `Described`: a `Described`-level obligation cannot be discharged for the abstract
  predicate every determiner macro (`target`, `a`, `each`, `allOf`, `bare`, `the`,
  `counted`, …) carries, so it would have rippled through the whole macro layer.
  `And` already carries `ContradictionFree`, and an attach word clashing with the head
  type is a rules-defined clash. Four `Matches` macros (`itsA`, `itIsntA`, `itsACard`,
  `itIsntAnAbility`) thread the new `AttachFits`.
- U7 renamed `selfTapPayment` to `selfTapUses` and changed its result to `Nat`; it had no
  users outside `selfTapCount`.
- U8's amount gate was also put on `MoveCounters` and `WithCounters`, the other two rows
  that pair an `Amount` with a `CounterKindSource`. `RemoveCounters` and `LosesCounters`
  were left alone: their slots are `Maybe Quantity` and `Maybe Amount`.
- U13's fateseal generalised the look cluster rather than duplicating it; `scry` and
  `surveil` keep their exact signatures via one-line specialisations.
- `crates/xtask/src/facts.rs` `REGIME_AXIS` prose gained one clause naming split second,
  so the rationale still describes every row that carries `functionsOnStack`.

STOPs taken:

1. **D-Q14's premise does not hold for three of its four predicates.** The ticket says
   `costActionOk`, `costPaidByYou`, `costOffBattlefield` and `costTapOnce` "each end in
   `_ = False`". In the tree only `costActionOk` does; the other three end at
   `ItsManaCost` and are already case-complete over their argument type, `Cost bs`. Their
   only catch-all over `Instruction` is the `(Do _)` row, which is the constant `True` in
   `costTapOnce` and `costOffBattlefield` and a permissive default in `costPaidByYou` —
   the opposite of the silent-exclusion hazard the finding names. Resolution:
   `costActionOk` made case-complete (behaviour-preserving); the other three left as they
   are, because making them case-complete over `Instruction` means three more 77-row
   tables in which 73+ rows carry the same constant, which the near-zero-noise and
   one-row-per-meaning rules refuse. Reported here rather than resolved silently.
2. **U6 has no gate to pin.** Nothing in the grammar consumes a `Lookback` through an
   obligation, so a storm body written with `ThisTurn` cannot be made unspellable without
   a guard that names the storm expansion. The round adds the row and the checked
   equality instead, and leaves the refusal open.
3. **U8's amount half applies to `SameAs` only.** `ThoseKinds` was left free: its bench
   spellings carry `ThatMuch`, `Plus ThatMuch (Lit 1)` and `times 2 ThatMuch` as well as
   `Lit 1` — "put the same number and kind of counters" (Captain Marvel, Apex Avenger) is
   a real amount there, so pinning the slot would refuse a printed sentence. `SameAs`
   carries the number itself ("put the same number of each kind of counter", Denry Klin,
   Editor in Chief; "put its counters on target creature you control", Star Pupil).
4. **Two adjacent gaps left open, both outside the named region.** `fateseal`'s possessor
   slot is free — `fateseal You You` is admitted, and nothing in the grammar tests "is an
   opponent" [CR#701.29a]. And the `"Fateseal"` row in `Words.actFacts` keeps
   `actStepwise = False` although its expansion is now scry-shaped; `"Scry"` and
   `"Surveil"` carry `True`.
