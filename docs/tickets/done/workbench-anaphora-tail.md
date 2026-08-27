---
needs: []
---
# Anaphora tail: the plural provenance twin, and the condition's re-mark

The two actionable remainders of the anaphora split (parent umbrella:
workbench-anaphora-mentions-and-creation; sub-rounds A–E closed 2026-08-27).

1. **`ThemVerbed`** — `ItVerbed`'s plural twin (39 occ / 38 cards): a new
   mention constructor, deliberately left out of sub-round C's §5-only
   budget. Cost: the constructor + `countManyVerbedIt`-style counter +
   Words readers + 14 total-table rows + one ProofsAnaphora §2 identity and
   §3 pair (additive), on `ItVerbed`'s model exactly.
2. **The condition-level RE-MARK** — Bioplasm's "If it's a creature card"
   should re-mark the tested binding's type so later reads see it, but a
   `condDelta` row would break `condIntroIsDeltaThenPrefix`'s `Refl`; the
   shape is a `settleTargets`-like in-place re-mark at `condIntro` level
   (clause 2's licensed form). No settled pin covers it — design against
   the binder contract and state the §5 cost honestly. (Bioplasm's bare
   "it" already writes via `ItAt CardSlot`; this item is about the
   knowledge the test leaves behind.)

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Phrase.idr`,
`ProofsAnaphora.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Item 1 lands with its proof debt paid; Bioplasm whole is item 2's
  acceptance witness (its last blocker after the exile-stamp read landed).
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

**Item 1 — `ThemVerbed`, delivered whole.** `ItVerbed`'s plural twin, on its
model exactly:

- `Words.idr`: `themVerbedReaches` (`itReaches ManyOf` + the label's stamp),
  `countVerbedThem`, `provOfVerbedThem`, `zoneOfVerbedThem`, `tyOfVerbedThem`.
- `Phrase.idr`: the `ThemVerbed` constructor beside `Them`, `setZoneVerbedThem`,
  and 15 total-table rows (`nounEqRef`, `nounDelta`, `anchorPhrase`,
  `choosable`, `groupMention`, `costNounOk`, `nounIsYou`, `nounTargeted`,
  `counterMemoryOk`, `moveDestOk`, `moveIntro`, `nounProv`, `nounZone`,
  `nounTy`, `nounPlur`). Measured deviation: the ticket costed 14 rows;
  `ItVerbed` in fact occupies 15, and so does the twin.
- `Macros.idr`: `themVerbed`.
- `ProofsAnaphora.idr`: §2 `countVerbedThemIsFold`, §3
  `themVerbedReadsOnlyPrefix` / `themVerbedResolvesInPrefix`.
- Bench: **Scapeshift**, whole — "Sacrifice any number of lands. Search your
  library for up to that many land cards, put them onto the battlefield
  tapped, then shuffle." Chosen because it is a witness rather than an
  illustration: `scapeshiftTwoGroups : countManys Object … = 2` proves the
  bare `Them` is genuinely refused where the pronoun is written, and
  `scapeshiftOneSearchedGroup` / `scapeshiftOneSacrificedGroup` prove each
  label picks exactly one of the two groups.

**Item 2 — the condition-level re-mark, delivered, with the acceptance
witness corrected.** The shape is the licensed in-place one, not a
`condDelta` row:

- `Words.idr`: `markTy` (writes a card type onto a binding that recorded
  none, leaving determiner, kind, plurality, zone and stamp alone) and
  `markFirst` (re-marks the first binding a test admits, in place).
- `Phrase.idr`: `remarkTest` (which prefix mention a test's SUBJECT names —
  answerable exactly for `It`, `ItAt` and `ItVerbed`, the three type-less
  singular object reads), `condRemarkAt`, `condRemark`, and
  `condIntro c = condDelta c ++ condRemark c`.
- **§5 cost, paid in full**: `condIntroIsDeltaThenPrefix` is replaced by
  `condIntroIsDeltaThenRemark` (still `Refl`), and the re-mark's licence is
  proved rather than assumed — `markTyKeepsOnes`, `markTyKeepsAt`,
  `markFirstKeepsLength` (nothing inserted, nothing dropped), `keptBy` /
  `countByCons` / `countByHeadCong`, `markFirstKeeps`, and
  `condRemarkKeepsOnes` / `condRemarkKeepsAt`. `gateSplitsAtCondIntro` and
  `slotGateSplitsAtCondIntro` are re-proved through those: every counted
  gate reads the re-marked prefix exactly as it read the prefix, so the
  re-mark changes what a later read FINDS on a mention and never how many
  mentions there are.
- Scope recorded on the rows themselves: only the POSITIVE, un-coordinated
  copula re-marks (a negated test names a type the mention is not, which one
  type slot cannot hold; `AndCond`'s arms are not walked), and the write
  happens only where the mention recorded no type.
- Bench: `bioplasmAfterTest`, `bioplasmTestRemarksType`
  (`tyOfVerbedIt "Exile" … = Just Creature` — the re-mark lands),
  `bioplasmTestMintsNothing` and `bioplasmTestKeepsCardSlot` (it re-marks and
  does not announce).

**Measured deviation — Bioplasm whole is NOT this item's witness.** The
ticket calls the re-mark Bioplasm's "last blocker after the exile-stamp read
landed". Against the code it is not: `bioplasmNoTypedRead`'s own docstring
already names TWO reasons the participle read at a type word is refused, and
the re-mark answers only the second. `bioplasmTypedReadStillRefused :
countVerbed "Exile" (TypeW Creature) bioplasmAfterTest = 0` records the
remainder as a pin. Bioplasm's "the exiled creature card's power" additionally
needs (a) `verbedWordOk (TypeW t)`'s `wasField` gate, which no card taken off
a library can satisfy, and (b) a head word for "creature card" — `NounWord`
has `TypeW t` and `CardW` and no combination of the two. Neither is knowledge
a test leaves behind, so neither belongs to this item.
