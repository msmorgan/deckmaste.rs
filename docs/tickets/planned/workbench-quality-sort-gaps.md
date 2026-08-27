---
needs: []
---
# Quality-sort gaps and the domainless axis pass

Routed from `workbench-axis-distributive-reads` (close, 2026-08-26), which
landed `ForEachKindOf` (value bound as a quality via `kindValueIntro`) and
`ColorPairAxis`/`ExactlyColors`. Its measured remainders, one region:

1. **`QualitySort`'s three gaps** — card type ("choose a card type", 10
   supported), non-creature-host subtype ("choose a land type" 5 + "choose a
   basic land type" 8), counter kind (2). The honest subtype shape is
   `SubtypeQ : CardType -> QualitySort` mirroring `SubtypeAxis`, and
   `sameQEq` then needs a 15×15 `CardType` equality-reflection lemma — the
   recorded cost; weigh it against the 25 supported lines.
2. **`kindAxisSort`'s corresponding `Nothing` cells** become `Just` as each
   sort lands (Magnigoth's `SubtypeAxis Land` value read; Hurkyl's card-type
   read).
3. **The DOMAINLESS distributive pass** (Niv-Mizzet Reborn): "For each color
   pair, choose …" runs over all ten of [CR#105.5]'s pairs, not the values
   present in a group — `ForEachKindOf`'s mandatory domain slot does not
   reach it. An 8th distributive line of a different shape; land it or name
   it at its one-line size with the rule.
4. Celestial Judgment's body read is refused by the standing rule-backed
   `chosenQualityReadOk Number = False` — that read's fate is
   `workbench-choice-chosen-and-ascription`'s, not this ticket's; recorded
   here only so the pass's witness is found when it flips.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`QualitySort`, `kindAxisSort`,
`sameQEq`), `Phrase.idr`, `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; the
  15×15 lemma cost is paid knowingly or the sort declined on it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
