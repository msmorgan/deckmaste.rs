---
needs: []
---
# Write each alternative cost in the form it is printed

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("engine
creep"), verified against `wmyykqnv`. Paths are under `lean/Semantics/`. Both parts were applied
to a scratch copy of `lean/` on 2026-10-25 and `lake build --wfail` passed (diff:
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/leanprobe/probe.diff`).

The workbench represents what cards can say (owner direction, 2026-10-25). An alternative cost is
printed in one of three forms, and each is written as its form [CR#118.9]:

- "cast … by paying [cost] rather than …" → `can you (cast this_ [from_] (payment := .payingInstead …))`
- "pay [cost] rather than pay …" → `StaticSpec.altCost`
- "cast … without paying its mana cost" → `can … (payment := withoutPaying)`

Today the split is made by an engine fact instead: whether the cast needs a permission
[CR#601.3] (`docs/tickets/done/lean-v3-alternative-cost-unification.md:11-24`, 2026-10-22), and
warp and *Omniscience* carry an invented condition standing in for the printed "from your hand".
This ticket replaces that classification. Additional and alternative costs stay distinct
(ADR `docs/decisions/semantics-v3.md` §0.1 item 5(a), [CR#118.8,118.9]); both forms here are
alternative costs.

Two parts, each separately committable. Part 1 first.

## Part 1: warp [CR#702.185a]

[CR#702.185a]: "Warp [cost]" means "You may cast this card from your hand by paying [cost]
rather than its mana cost" and "If this spell's warp cost was paid, exile the permanent this
spell becomes at the beginning of the next end step. …". The Oracle reminder agrees (*Bygone
Colossus*: "You may cast this card from your hand for its warp cost").

`Definitions/Keywords.lean:1889-1890`: replace
```lean
    [ static (onlyWhile (altCost this_ (some (.inCaller cost)))
        (is this_ (did any (cast which (from_ := yourHand)) thisObject))),
```
with
```lean
    [ static
        (can you
          (cast this_ (from_ := yourHand) (payment := .payingInstead (.inCaller cost)))),
```
Leave the Resolution Delayed Trigger static (`:1891-1900`) unchanged. In the docstring
(`:1881-1886`) replace "Its first static functions on the stack and grants no cast: the rules
already allow a cast from the hand [CR#601.3], so "cast this card from your hand by paying
[cost]" is an alternative cost [CR#118.9] that applies only while this spell was cast from your
hand." with "Its first static is the printed permission "You may cast this card from your hand
by paying [cost] rather than its mana cost", an alternative cost [CR#118.9]."; keep the Resolution
Delayed Trigger sentence.

Pins, `Proofs/KeywordCastPermissions.lean`:
- `pureAlternativeCostsAreAltCost` (`:284-290`): remove `warp cost` from its list and "warp
  [CR#702.185a] and" from its docstring (`:278-283`).
- `castPermissionsKeepPayingInstead` (`:296`): after the flashback conjunct add
  ```lean
      (componentStatics (warp cost)).any isPayingInsteadPermission = true ∧
      !(componentStatics (warp cost)).any isAltCost = true ∧
  ```
  with the proof `refine ⟨by decide, by decide, by decide, by decide, rfl, rfl⟩`, and add warp
  [CR#702.185a] to its docstring.
- Must stay green: `okWarp` (`Proofs/OffStack.lean:88`), `delayedTriggersWaitForThePermanent`,
  `okDashBlitzWarp`, and the warp cards (`Cards/Keyword.lean:872` *Bygone Colossus*, `:911`).
  `keywordCostPaid (.named "warp")` resolves through facts; cost results are published for both
  forms (`staticTextContextWith`, `Check/Phrase.lean:2697-2703`).

## Part 2: every other alternative cost

**Keywords that move to the cast permission** (`Definitions/Keywords.lean`; only each body's
first static changes):

| Keyword | Line | CR "means" text | New first static |
|---|---|---|---|
| blitz | `:348` | [CR#702.152a] "You may cast this card by paying [cost] rather than its mana cost" | `static (can you (cast this_ (payment := .payingInstead (.inCaller cost))))` |
| cleave | `:429` | [CR#702.148a] "You may cast this spell by paying [cost] rather than paying its mana cost" | same as blitz |
| dash | `:487` | [CR#702.109a] "You may cast this card by paying [cost] rather than its mana cost" | same as blitz |
| evoke | `:740` | [CR#702.74a] "You may cast this card by paying [cost] rather than paying its mana cost" | same as blitz |
| emerge | `:618-620` | [CR#702.119a] "You may cast this spell by paying [cost] and sacrificing a creature rather than paying its mana cost" | `(can you (cast this_ (payment := .payingInstead (.compound [.inCaller cost, .perform (sacrifice (a isCreature))]))))` |
| emergeFrom | `:632-637` | [CR#702.119b] | as emerge, with `.perform (sacrifice (a (and [.inCaller quality, isPermanent])))` |
| webSlinging | `:1903-1908` | [CR#702.188a] "You may cast this spell by paying [cost] and returning a tapped creature you control to its owner's hand rather than paying its mana cost" | `(can you (cast this_ (payment := .payingInstead (.compound [.inCaller cost, .perform (returnToHand (a (and [isCreature, isTapped, youControl])))]))))` |

Before each edit, read the keyword's "means" text in `data/rules/cr.txt` and confirm it prints
"cast … by paying … rather than". Each keyword's own guard statics (`onlyWhile`, `doIf`) stay as
they are.

**Keywords that stay `altCost`**, each printed "pay [cost] rather than pay": awaken (`:286`,
[CR#702.113a]), bestow (`:339`, [CR#702.103a]), freerunning (`:875`, [CR#702.173a]), impending
(`:1023`, [CR#702.176a]), mutate (`:1243`, [CR#702.140a]), overload (`:1321`, [CR#702.96a]),
prowl (`:1440`, [CR#702.76a]), spectacle (`:1636`, [CR#702.137a]), surge (`:1720`, [CR#702.117a]).

**Already in printed form, unchanged:** escape (`:724`), flashback (`:827`), foretell (`:860`),
harmonize (`:931`), madness (`:1155`), mayhem (`:1166`), miracle (`:1213`), sneak (`:1585`),
airbend (`Definitions/Actions.lean:442`), and the `withoutPaying` instructions of cipher (`:426`),
paradigm (`:1341`), rebound (`:1485`), suspend (`:1744`), `castWithoutPaying`
(`Definitions/Actions.lean:341`) and `Definitions/SubtypeRules.lean:32`.

**Cards printed "you may cast this spell without paying its mana cost"** move from
`static (onlyWhile (altCost this_) <cond>)` to
`static (onlyWhile (can you (cast this_ (payment := withoutPaying))) <cond>)`, condition
unchanged:
- *Kyren Legate*, `Cards/Keyword.lean:687`
- *Mogg Salvage*, `Cards/Static.lean:531`
- *Massacre*, `Cards/Static.lean:566`
- *Deflecting Swat*, `Cards/Static.lean:664`

*Omniscience* (`Cards/Choice.lean:1166-1167`, "You may cast spells from your hand without paying
their mana costs.") is `altCost (allOf (and [isSpell, did you (cast which (from_ := yourHand))
thisObject]))`, the same invented condition as warp. Write
`static (can you (cast (allOf isSpell) (from_ := yourHand) (payment := withoutPaying)))`.

**Cards that stay `altCost`** (printed "… rather than pay …"): *Force of Will*
(`Cards/Cost.lean:584`), *Snuff Out* (`Cards/Deontic.lean:462`), *Demon of Death's Gate*
(`Cards/Cost.lean:594`), the five Shoals (`Cards/Mana.lean:451,466,477,487,498`), *Crash*,
*Abolish*, *Gush*, *Sunscour*, *Rouse*, *Thwart*, *The Lady of Otaria*
(`Cards/Static.lean:524,542,549,558,578,586,593`), *Invigorate* (`:657`), *Fist of Suns*
(`:673`), *Rooftop Storm* (`:680`), *Admiral's Order* (`Cards/Trigger.lean:404`), *Heart of
Kiran* (`Cards/Counters.lean:899`), *New Perspectives* (`Cards/Keyword.lean:993`). The hand-built
pins at `Proofs/Actor.lean:279-285`, `Proofs/Static.lean:305`, `Proofs/Faces.lean:301` and
`Check.lean:102` are unaffected.

Pins, `Proofs/KeywordCastPermissions.lean`:
- Section header `:254` → "A "pay … rather than pay" alternative cost is `altCost`; a printed
  "cast … by paying … rather than" is a cast permission".
- `pureAlternativeCostsAreAltCost` → renamed `payRatherThanPayIsAltCost`, list
  `[ awaken 2 cost, bestow cost, freerunning cost, impending 2 cost, mutate cost, overload cost,
  prowl cost, spectacle cost, surge cost ]`, docstring listing exactly those with their CR rules.
- `castPermissionsKeepPayingInstead` → renamed `castByPayingIsACastPermission`; replace Part 1's
  two warp conjuncts with
  ```lean
      [ blitz cost, cleave cost, dash cost, emerge cost, emergeFrom isArtifact cost, evoke cost,
        warp cost, webSlinging cost ].all
        (fun keyword =>
          (componentStatics keyword).any isPayingInsteadPermission &&
            !(componentStatics keyword).any isAltCost) = true ∧
  ```
  with the proof `refine ⟨by decide, by decide, by decide, rfl, rfl⟩`, and the docstring listing
  the eight keywords with their CR rules.
- Must stay green: `okDash`, `okBlitz`, `okWarp` (`Proofs/OffStack.lean:82,85,88`), `okEmerge`,
  `okEmergeFrom` (`Proofs/LinkedReadback.lean:320,322`; emerge's "the sacrificed creature" still
  reads the published cost result), `okCleaveDefinition` (`Proofs/TextChanges.lean:141`),
  `okFreerunningDefinition` (`Proofs/KeywordAbilityCensus.lean:126`), and every moved card's pin.

Docs:
- `lean/CONTRACTS.md:832` (PlayPayment row): replace "`payingInstead` is the alternative cost of
  a cast a permission grants from another zone or at another time, or that an instruction
  directs; a pure alternative cost, granting no cast, is `StaticSpec.altCost` [CR#118.9,601.3]."
  with "`payingInstead` is the alternative cost a printed "cast … by paying … rather than"
  permission or instruction pays; a printed "pay … rather than pay" is `StaticSpec.altCost`
  [CR#118.9]."
- `lean/CONTRACTS.md:1123`: "emerge's `StaticSpec.altCost` cost" → "emerge's cast permission's
  `PlayPayment.payingInstead`".
- `docs/decisions/semantics-v3.md:646`: move emerge [CR#702.119a] to the `.can … payingInstead`
  example and cite awaken [CR#702.113a] as the `altCost` example.
- `Abilities.lean:934-937` (`altCost` docstring): "It grants no cast; a cast from another zone or
  at another time is a `can` permission whose payment is `payingInstead` [CR#601.3]." → "It is
  the printed "pay [cost] rather than pay"; a printed "cast … by paying [cost] rather than …" is
  a `can` permission whose payment is `payingInstead` [CR#118.9]."

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates (bless any
  newly cited rule).
- After Part 2, `grep -n 'altCost' lean/Semantics/Definitions/Keywords.lean` lists only awaken,
  bestow, freerunning, impending, mutate, overload, prowl, spectacle and surge.
