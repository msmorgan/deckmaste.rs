---
needs: []
---
# A pure alternative cost is `StaticSpec.altCost`; a cast permission keeps `payingInstead`

Standard constraints apply. Provenance: row B3 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). The ruling
chose Core `altCost`, not the audit row's proposal (the permission form with an `altCost`
Template); do not follow the row.

## Ruling (owner, 2026-10-22)

- Every pure alternative cost ("You may pay [cost] rather than pay this spell's mana cost", with
  no new zone or timing permission) uses `StaticSpec.altCost`: evoke, dash, surge, awaken, mutate,
  overload, prowl, spectacle, bestow, *Force of Will*, *Snuff Out*, *Fist of Suns*, *Omniscience*.
  Verify each fits.
- `PlayPayment.payingInstead` on `procedure .cast` stays where the effect grants a cast permission
  from another zone (flashback, escape, disturb, foretell, madness, plot, airbend) or instructs a
  cast during resolution (miracle, *Bring to Light*) [CR#118.9].

An alternative cost is one a spell's controller "may pay rather than paying the spell's mana
cost", listed in its text or applied from another effect, phrased "rather than pay" or "without
paying" [CR#118.9]. Whether a player may cast a spell at all is a separate permission [CR#601.3].

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

`StaticSpec.altCost (subject : NounPhrase) (cost : Option Cost)` (`Abilities.lean:883`, no doc
comment; Template `Templates.lean:1943-1944`). `cost = none` is "without paying its mana cost".
The subject may be any noun phrase, so a non-self subject works. There is no condition, zone or
timing slot; a condition goes through the `onlyWhile`/`onlyUnless` wrapper (`.conditional`,
pinned by `unlessIsIfNot`, `Proofs/Static.lean:305`). Checker: `Check/Phrase.lean:3624-3627`
steps the subject, then the cost, with no refusal of its own.

| Item | Fits | Today |
|---|---|---|
| evoke [CR#702.74a] | yes | `payingInstead`, `Definitions/Keywords.lean:661` |
| dash [CR#702.109a] | yes; haste and the return are separate statics (`:450-456`) | `:449` |
| surge [CR#702.117a] | yes, condition via `onlyWhile` | `:1526` |
| awaken [CR#702.113a] | yes; the spell gates on `keywordCostPaid`, which reads facts | `:268` |
| mutate [CR#702.140a] | yes; target and merge gate on `choiceWasTaken` (facts), body indices 1 and 2 | `:1088` |
| overload [CR#702.96a] | yes; the text change is a separate static | `:1166` |
| prowl [CR#702.76a] | yes, via `onlyWhile` | `:1254` |
| spectacle [CR#702.137a] | yes, via `onlyWhile` | `:1442` |
| bestow [CR#702.103a] | already `altCost thisSpell …` | `:321` |
| *Force of Will* | already `altCost` | `Cards/Cost.lean:570` |
| *Snuff Out* | already `onlyWhile (altCost this_ (payLife 4)) …` | `Cards/Deontic.lean:449` |
| *Fist of Suns* | already `altCost (allOf (and [isSpell, castBy you])) …` | `Cards/Static.lean:641` |
| *Omniscience* | yes: no zone or timing granted, an alternative cost applied from another effect | `can you (cast (allOf isSpell) (from_ := yourHand) (payment := withoutPaying))`, `Cards/Choice.lean:947-949` |

`keywordCostPaid` and `choiceWasTaken (.byKeyword k)` resolve through the facts table
(`Check/Keywords.lean:122-126`, `paidCost` in `Check/Facts.lean:130-153`), not through the cost's
encoding, so the moves keep them working.

**Corrections to the second list.** flashback (`:736`), escape (`:645`), foretell (`:769`),
madness (`:1014`) and airbend (`Definitions/Actions.lean:447`) do use `payingInstead`. But:

- disturb (`:535-536`) and miracle (`:1061-1062`) are bare `keywordCosting` with no body, so no
  `payingInstead` exists yet. When written, they take it, as ruled.
- plot's own body has no cast (`:1191-1195`); the cast right is the rule row `plottedCardCast`
  (`Proofs/Rules.lean:97-104`) with `withoutPaying` [CR#702.170d]. It stays.
- *Bring to Light* is not encoded.
- The ruling's citation `[CR#601.3e]` is about alternative characteristics when checking a cast's
  legality (it fits disturb's transformed cast). The rule that a cast needs a permission is
  [CR#601.3].

**"Every" covers more than the list.** These keyword bodies have the same pure shape and move
too: blitz [CR#702.152a] (`:329`), cleave [CR#702.148a] (`:406`), emerge [CR#702.119a] (`:567`),
emerge from [CR#702.119b] (`:582`), impending [CR#702.176a] (`:901`), web-slinging
[CR#702.188a] (`:1670`). Sneak (`:1391`) grants timing [CR#702.190a] and stays. Warp
(`:1655`) is "cast this card from your hand by paying [cost]" [CR#702.185a]: no new zone, but
`altCost` has no zone slot; write it as `altCost` with a subject restricted to the card in hand,
or keep it, and say which.

**Already `altCost`, unchanged**: *Demon of Death's Gate* (`Cards/Cost.lean:580`), *Admiral's
Order* (`Cards/Trigger.lean:402`), the five Shoals (`Cards/Mana.lean:443,455,466,476,487`),
`Cards/Static.lean:492,499,510,517,526,534,546,554,561,625,632,648`, *Kyren Legate*
(`Cards/Keyword.lean:684`). *Heart of Kiran*'s crew (`Cards/Counters.lean:833`), *New
Perspectives*' cycling (`Cards/Keyword.lean:948`) and *Thick-Skinned Goblin*'s echo (`:953`) use
`altCost` on an ability's cost; no ruling item covers ability costs, so they stay as they are.
(Found in passing: *New Perspectives* omits "As long as you have seven or more cards in hand";
echo's cost is paid while a trigger resolves [CR#702.30a], not an activation cost. Ticket these
if not fixed here.)

## Change

1. Move body[0] of evoke, dash, awaken, mutate and overload, and the `onlyWhile (can …)` of
   prowl, spectacle and surge, to `altCost this_ (some (.inCaller cost))` (with `onlyWhile` for
   the conditions). Keep mutate's body indices 1 and 2 in place. Then blitz, cleave, emerge,
   emerge from, impending and web-slinging; warp as decided.
2. Use `this_`, not `thisSpell`: `onInstantOrSorceryCardOk` (`Check/Card.lean:48`) accepts only
   `.altCost .this _`. Either write `this_` (and change bestow's `thisSpell`) or widen that check to
   any subject that names this object.
3. Emerge and harmonize read back the cost they paid ("the sacrificed creature"), which
   `staticTextContextWith` (`Check/Phrase.lean:2089-2111`) publishes only for `.can … payingInstead
   …` and `plusAdditional` (`:2099`). Add an `.altCost _ (some cost)` arm before moving emerge, or
   `okEmerge` and `okEmergeFrom` (`Proofs/LinkedReadback.lean:313,315`) fail. Note that `altCost`
   steps its cost without `stepPlayPaymentWith`'s reset of pending roles (`:1869-1879`); check the
   arm does what the permission's did.
4. *Omniscience*: `altCost (allOf (and [isSpell, <cast by you>, <cast from your hand>])) none`.
   Its current tree also reads a card in hand as a spell. Write the provenance with whatever
   `lean-v3-cast-predicates-and-ordinals` has landed (`castBy`/`castFrom` today, `did` after).
5. Docs: give `StaticSpec.altCost` a doc comment ([CR#118.9]); `lean/CONTRACTS.md:964-966`
   (emerge and harmonize as published alternative-cost results) and `:712,728`; ADR
   `docs/decisions/semantics-v3.md:591` (the publishing form) if step 3 adds the arm.

## Tests

- Rewrite `awakenDefinition` and `okAwakenDefinition` (`Proofs/KeywordAbilityCensus.lean:61,78`):
  a hand copy of awaken's body on the retired permission form. Its doc (`:9`) says "awaken as an
  alternative cost"; keep that claim.
- Keep passing (positional or shape-based): `mutateTargetsOnlyOnceItsCostIsChosen` and
  `mutateMergeIsGatedByTheSameChoice` (`Proofs/KeywordCastPermissions.lean:143,153`),
  `delayedTriggersWaitForThePermanent` (`Proofs/OffStack.lean:190`), `okDash`, `okDashBlitzWarp`
  (`Proofs/OffStack.lean:82,207`), `okOverloadDefinition` (`Proofs/TextChanges.lean:112`),
  `okProwl`, `alternativeCostReadbacks` (`Proofs/Faces.lean:522,803`), `okMutate`
  (`Proofs/LinkedReadback.lean:328`), `okEmerge`, `okEmergeFrom`, and the `spelled` cards using the
  moved keywords (`Cards/Damage.lean:1058,1069`, `Cards/Mana.lean:713`, `Cards/Keyword.lean:970`).
- Add `pureAlternativeCostsAreAltCost`: each moved keyword's body contains an `altCost` and no
  `.can … payingInstead`.
- Add `castPermissionsKeepPayingInstead`: flashback, escape, foretell and madness still carry the
  permission.
- Optional rename: `alternativeCostKeywords` (`Proofs/Faces.lean:798`) also lists additional-cost
  keywords.

## Other tickets on the same files

- `lean-v3-cast-predicates-and-ordinals` deletes `castBy`/`castFrom`, used by *Fist of Suns*,
  *Rooftop Storm* and *Omniscience*'s new subject. Land this first or write the `did` form.
- `lean-v3-becomes-blocked-status` and `lean-v3-keyword-class-guards-and-observation` edit other
  hunks of `Definitions/Keywords.lean`.
- `lean-v3-pay-life-action` changes *Snuff Out*'s and *Force of Will*'s `payLife` tree (source
  unchanged).
