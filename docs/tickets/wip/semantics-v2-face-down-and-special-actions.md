---
needs: [semantics-v2-temporal-suite]
---
**One face-down spec, and special actions written as a cost plus an
instruction.** Ruling 2026-10-08 ("Face-down is one spec",
`docs/decisions/semantics-v2.md` §7). It needs the temporal suite's `Timing`
and `Duration`. Standard constraints apply.

## Shape

- `FaceDown (characteristics : Option Characteristics)`.
  - `none` is a card exiled face down: no characteristics and no status
    [CR#406.3a,110.5d].
  - `some c` is a face-down spell or permanent. `c` holds its copiable values
    (layer 1b) [CR#708.2,613.2b].
- Carriers:
  - `Instruction.turnFaceDown (subject) (spec : FaceDown)`. Manifest is
    "turn it face down; … put that card onto the battlefield face down"
    [CR#701.40a]. Later mentions of the card are anaphoric (moved-referent
    ruling, 2026-10-08).
  - The move rider `faceDown (spec : FaceDown)`, used for exiling face down
    and for entering face down. It replaces `TokenRider.faceDown`. Every
    move's `riders` list already takes a `TokenRider`; renaming that type is
    the landing's call.
  - `PlayPayment.faceDown (spec) (cost)` for morph and disguise
    [CR#702.37a,702.168a].
- Turning face up is `setStatus(faceUp, …)`. The values revert [CR#708.8].
- Special action: `StaticSpec.specialAction (when : Option Timing) (cost :
  Cost) (does : Instruction)`, meaning "any time you have priority, within
  `when`, you may pay this and do this" [CR#116.1,116.2c].
- Look rights (foretold card, hideaway [CR#702.75a], face-down permanents
  [CR#708.5]) use the existing `StaticSpec.visibility`.
- Checker: references and kinds only. Nothing face down is a token.

## Retires

`TokenRider.faceDown`; the helper
`plugins_v2/builtin/macros/instructions/putOntoBattlefieldFaceDown.ron`;
`CostNamed.ofSpecialAction` (a named cost reads `ofKeyword(label)`); the
`SpecialAction` enum (`turnFaceUp | putCompanionIntoHand | foretell |
unlockDoor`), each member becoming a `specialAction` row or a plain
instruction, its new home named in the landing record.

## Re-spells (same asserted outcome)

- Bodies: manifest, cloak, manifestDread.
- Canon: Soul Summons, Ransom Note, Threats Around Every Corner.
- Pins: `okManifest`, `okCloak` and `okManifestDread` in
  `lean/Semantics/Proofs/KeywordActionResidues.lean`, the face-down cases in
  `Faces.lean`, and every pin naming `SpecialAction` or `ofSpecialAction`.

## Bodies

- morph: `PlayPayment.faceDown` for {3}, and
  `specialAction(none, <morph cost>, setStatus(faceUp, this))`
  [CR#702.37a,116.2b].
- megamorph (undeclared today; declare it) [CR#702.37b], disguise
  [CR#702.168a] and hideaway [CR#702.75a].
- foretell: `when = inPart(turn, you, current)`, cost {2}, `does =
  sequentially([exile(this, faceDown(none)), establish(mayCastFrom(it,
  exileZone, payingInstead(Param(0)), between(endOf(turn, current),
  none)))])` [CR#702.143a,116.2h].
- plot: its cost, exile, then the free cast at sorcery speed on a later turn
  [CR#702.170a,702.170d,116.2k].
- suspend [CR#702.62a], if the temporal suite can write it. Otherwise STOP
  and name the gap.

## Witnesses

Soul Summons (manifest); a morph creature turned face up (values revert); a
foretold card cast on a later turn but not the turn it was foretold; a
plotted card. Pins: `exile(…, faceDown(none))` checks, and a `specialAction`
whose `does` reads an unbound anaphor is refused.

## Deltas

Glossary Face-Down and Special Action are already rewritten in
`docs/contexts/game-model/CONTEXT.md`. Amend them if the build shows a flaw.
Engine-side work stays in `engine-face-down` and `engine-special-actions`.
