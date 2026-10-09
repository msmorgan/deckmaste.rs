---
needs: []
---
**A static that changes a modal spell's own mode choice.** Ruling 2026-10-09
("A mode choice is changed by a static over the spell's own `chooseModes`",
`docs/decisions/semantics-v2.md` §7). Bodies entwine, escalate, spree and
tiered, four of the census residues. Standard constraints apply.

## Shape

- `Instruction.chooseModes (quantity) (modes : List (Option Cost ×
  Instruction))` already carries the quantity and per-mode costs
  [CR#700.2h]. Spree's "+ {cost} —" and tiered's "Name — {cost} —" bullets
  are authored on the modes; no new node for them.
- `StaticSpec.modeChoice (change : ModeChange)`, modifying the `chooseModes`
  of the spell it is printed on. `ModeChange` rows:
  - `allInstead (cost : Option Cost) (condition : Option Condition)`:
    entwine [CR#702.42a] and the 18 non-keyword "if COND, you may choose
    both instead" cards;
  - `extraModesCost (cost : Cost)`: escalate [CR#702.120a];
  - `setQuantity (quantity : Quantity)`: spree `atLeast 1` [CR#702.172a],
    tiered `exactly 1` [CR#702.183a];
  - `repeatable`: "you may choose the same mode more than once" [CR#700.2d]
    (22 cards);
  - `notChosenBefore (within : Duration)`: "that hasn't been chosen [this
    turn]" (21 cards).
- Mode weights: the mode entry gains `weight : Option Nat`, and
  `Quantity.worth (n : Nat)` writes "choose up to N {P} worth of modes"
  [CR#700.2i] (5 cards).
- Checker: kinds and references only. `modalFrameOk` stays a kind/shape
  check; it does not learn the choice's arithmetic. A `modeChoice` on an
  ability that has no `chooseModes` is refused as a shape.
- Rust mirror, RON helpers, bench macros spelled like the RON target.

## Re-spells

Every `chooseModes` site in `lean/Semantics/Cards/`, the bench and canon
gains the empty weight; same asserted outcome. Pins naming the modes list
keep their statements.

## Bodies

- entwine: `static(modeChoice(allInstead(some(Param(0)), none)))`
  [CR#702.42a].
- escalate: `static(modeChoice(extraModesCost(Param(0))))` [CR#702.120a].
  Its paid count reads `timesPaid(byKeyword(escalate))`: the existing
  facet, renamed `LinkedFacet.timesPaid` once `semantics-v2-linked-readback`
  lands. Use whichever spelling the tree has at claim time and name the
  other in the landing record.
- spree: `static(modeChoice(setQuantity(atLeast(1))))` [CR#702.172a].
- tiered: `static(modeChoice(setQuantity(exactly(1))))` [CR#702.183a].

## Witnesses (canon)

One canon card each for entwine, escalate, spree and tiered if canon lacks
one; a conditional "you may choose both instead" card (`allInstead(none,
some(COND))`); a pawprint card (`worth`); a "same mode more than once" card;
a "that hasn't been chosen this turn" card. Pins: each row checks; a
`modeChoice` with no modes to change is refused.

## Siblings

No dependency. `semantics-v2-linked-readback` renames the facet escalate
reads (above). All seven 2026-10-09 tickets touch
`Words`/`Phrase`/`Abilities`: refresh before integrate and expect conflicts.

## Landing record

Measured on `zpkxpkqu` (canon commit, after `kata refresh`).

1. **Census delta**: not applicable. No English grammar, lexicon or keyword
   spelling changed, so no english-v3 census was run. Covered faces and
   Readings are unchanged. Lost faces: none.
2. **Witnesses**: English Readings do not apply (semantics-v2 only). Lean:
   `cargo xtask lean check` proves 185/185 cards (177 canon, 8 testing).
   They include the eight new canon witnesses: Dream's Grip (entwine),
   Borrowed Malevolence (escalate), Explosive Derailment (spree), Fire Magic
   (tiered), Soul Transfer (`allInstead(None, Some(COND))`), Season of
   Weaving (`worth(5)`, weights 1/2/3, `repeatable`), Fiery Confluence
   (`repeatable`) and Monument to Endurance
   (`notChosenBefore(soFarThisTurn)` over a triggered ability's modes), plus
   Ransom Note, re-spelled.
3. **Spot checks** (bodies as they expand in `Declared.lean`): entwine =
   `static(modeChoice(allInstead(Some(Param(0)), None)))` [CR#702.42a];
   escalate = `static(modeChoice(extraModesCost(Param(0))))`
   [CR#702.120a]; spree = `static(modeChoice(setQuantity(atLeast(1))))`
   [CR#702.172a]; tiered = `static(modeChoice(setQuantity(exactly(1))))`
   [CR#702.183a]. There are fewer than ten because the ticket adds four
   bodies and eight cards, and no English faces.
4. **Deviations and STOPs**: no STOP.
   - Escalate's count reads `timesPaid(ByKeyword(escalate), …)` through the
     facet's current name, `PaidFacet.timesPaid`.
     `semantics-v2-linked-readback` (still planned) renames it
     `LinkedFacet.timesPaid`. The pin `okEscalateTimesPaid` checks it.
   - A mode is the triple `(Option Cost × Option Nat × Instruction)`
     (cost, weight, instruction), not a structure.
   - The Lean bench macros `chooseModes` (instruction list) and
     `chooseSpree` (cost/instruction pairs) keep their signatures and supply
     the empty weight themselves, so `Cards/` sites are unchanged. Only
     direct `.chooseModes` constructor sites gained `none`.
   - Shape check: `modalFrameOk` now also refuses a written Mode Change on
     a text with no modal ability. The ruling requires that refusal, and
     only a spell's modes satisfy the existing keyword half, so the check
     needed a modal ability of any of the three classes (spell, activated,
     triggered [CR#700.2a,700.2b]). The "hasn't been chosen this turn"
     cards are all triggered or activated abilities.
   - `StaticSpec.onInstantOrSorceryCardOk` admits `modeChoice`, since a Mode
     Change is a static of the modal spell itself.
   - `StaticSpec.regimeFits` and xtask's `static_regime` read `modeChoice`
     as at-casting [CR#601.2b,700.2a]. `cargo xtask facts generate` then
     gave entwine, escalate, spree and tiered `definition := [⟨.static,
     some .atCasting⟩]`.
   - `ModeChange.check` checks kinds and references only, plus `nonZeroQ`
     and `wellFormedQ` on `setQuantity`'s quantity. `Quantity.worth 0` is
     non-zero-refused.
   - The existing `modesFit` rule still refuses a repeatable "choose up to
     four" over two modes (Moment of Reckoning). That is a recorded gap,
     not fixed here, because the ruling keeps the arithmetic out of the
     checker.
   - `ModeChange` is registered as a v2 syntax type in `lean_macros.rs`,
     with expansion samples in `expansions.rs`.
   - `Declared.lean` also carries trunk's stale `typecycling` entry, which
     trunk's `rmolrxyn` now matches.
   - Tiered's mode names ("Fire —") are not modeled.
   - Glossary: Mode Change is already defined, and no term is missing.
5. **Tests**: all of the following pass.
   - `cargo test -p deckmaste_semantics_v2`: 45 + 7 + 1 + 4 + 44.
   - `cargo test -p xtask`: 398 + 4 + 14, with 1 pre-existing ignore.
   - `cargo test -p deckmaste_construction_core`: 434 + 34.
   - `cargo test -p deckmaste_lexical_source`: 28 + 36.
   - `cargo clippy -p <each> --tests -- -D warnings`: clean.
   - `lake build --wfail`: clean.
   - `cargo xtask lean macros check` and `cargo xtask facts check`: up to
     date.
   - `cargo xtask cite check`: 0 stale, 0 non-compliant.
   - `cite audit --diff`: 50 sites read against rule text.

   Counts:
   - Restored: 0.
   - Re-spelled: 5. These are `okModalTwoModes`, `badModalOneMode`,
     `okModalTwoOfTwo`, `badModalOverreach` and Actor's `literalEndure`
     (empty weight, same statements), plus Ransom Note and `chooseOne.ron`.
   - Ignored: 0.
   - Added: 14. These are 11 Choice pins (`okModeChoiceEntwine`,
     `okModeChoiceConditionalBoth`, `okModeChoiceEscalate`,
     `okEscalateTimesPaid`, `okModeChoiceSpree`, `okModeChoiceTiered`,
     `badModeChoiceZero`, `okModeChoiceRepeatable`,
     `okModeChoiceNotChosenThisTurn`, `okModalWorth`,
     `badModalWorthZero`), 2 Faces pins (`badModeChoiceWithoutModes` =
     `[.modalFrame]`, `okModeChoiceOnTriggeredModes`) and the Rust test
     `a_mode_carries_its_weight_and_a_mode_change_reads`.
   - Removed: 0.
6. **Timing**: no census run. `cargo xtask lean check` took 250.4 s.
   `cargo xtask lean definitions` took 35.0 s (708 definitions).
