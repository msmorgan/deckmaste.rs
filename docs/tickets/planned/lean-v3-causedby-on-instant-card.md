---
needs: []
---
# `onInstantOrSorceryCardOk` refuses a printed `.causedBy` replacement

Standard constraints apply. Found landing `lean-v3-keyword-class-guards-and-observation`.

## Problem

`StaticSpec.onInstantOrSorceryCardOk` (`lean/Semantics/Check/Card.lean:45`) returns false for a
`.causedBy` would-clause (`| .wouldAny _ | .causedBy .. | .nthOccurrence .. => false`, `:62`). An
instant or sorcery card's static ability functions only on the stack [CR#113.6], so a static ability
is admitted there only for the listed self-referring forms. A printed, non-keyword "as it resolves"
replacement on an instant or sorcery card (`replace (causedBy (.event (does .any (resolve thisSpell)))
<move>) …`, the form `Definitions/Actions.lean:63-76` documents; the spell is put into the graveyard
as the final part of its resolution [CR#608.2n]) would be refused. The keyword versions (buyback,
rebound) pass through the facts table, not this arm.

## Witness search (2026-10-22)

`data/derived/cards.jsonl` was searched for instants and sorceries whose own text, outside reminder
text, says "as it resolves" or "as this spell resolves" or "instead of putting it into your
graveyard": **none found**. Every hit is keyword reminder text (buyback, rebound) or a counterspell's
"exile it instead of putting it into its owner's graveyard" (*Dissipate*, a "countered this way"
clause, not a resolution). The printed "as it resolves" replacements are on permanents and are
triggered abilities (*Lilah, Undefeated Slickshot*, *Goliath Daydreamer*, *Rod of Absorption*,
*Soulfire Grand Master*), which `classAbilityOk` already admits. So the gap is latent today. Step 1
is to decide with the owner whether the arm should be widened now or the ticket retired.

## Change

1. Decide: admit a `.causedBy` would-clause whose cause names this spell's own resolution
   [CR#608.2n,113.6] (and, if a witness appears, say which card), or retire this ticket as
   unwitnessed.
2. If admitted, add the arm and the pin below.

## Tests

- `okInstantCardOwnResolutionReplacement`: a synthetic instant with a printed `replace (causedBy
  (.event (does .any (resolve thisSpell))) <move>)` checks clean; the same text with a different
  cause is still refused.
