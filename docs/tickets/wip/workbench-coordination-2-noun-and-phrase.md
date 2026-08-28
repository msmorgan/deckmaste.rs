# coordination-2: the noun and phrase coordinations

Sub-round 2 of [workbench-coordination-family](workbench-coordination-family.md)
(the umbrella — authoritative for measurements and acceptance). Owns
everything the umbrella holds except sub-round 1's two sections: the four
coordinations outside the derived coordinator (coordinated destination —
measure across verbs first; and/or as its own truth condition; the
kind-crossing disjunction with Sugar Coat/Food; the cross-zone disjunction
with `badCrossZoneDisjunction`), the two-descriptions families (coordinated
subject/Echoing cycle; protection-shaped distribution), the two noun
coordinations (player-plus-player distributive; heterogeneous double target),
and every routed item in the umbrella's tail (Concussive Bolt, Mana Clash,
Inspirit, Trouble in Pairs / Avatar Aang, Repeated Reverberation's join
decision, the shuffled mass object probe, Talion's characteristic list, the
Lace union subject).

Pins:
- Same-kind only: no coordination becomes a second spelling of a cross-kind
  join (the kind-index decision is the authority, cited in the umbrella).
- The distributive and joint readings stay distinct rows; the trailing "each"
  IS the construction. Do not fold the player-plus-player pair into the group
  referent.
- The heterogeneous double target is TWO mentions, never one filter
  [CR#601.2c].
- "and/or" is its own truth condition, never an environment of the derived
  word.
- Repeated Reverberation's disagreeing-arms JOIN is a deliberate re-decision
  seat: make it or refuse it WITH the rule, and flag the outcome prominently
  in your report.
- Doc Aurlock cross-check first: if the event-zone split's zone-coordination
  landed, re-check the cross-zone section against it before minting anything.
- The measured zeros (either/or, both, neither-nor) stay unbought.

Acceptance: the umbrella's remaining acceptance lines. `idris/scripts/build`
PASS. Standard constraints apply.

## As landed

### FLAGGED FIRST: Repeated Reverberation's disagreeing-arms JOIN — REFUSED

The seat round's whole-agreement verdict **stands**, and the rule is the pair
[CR#115.1] / [CR#603.7b].

A `Joined` kind is [CR#115.1]'s union: ONE referent standing in two kind
positions at once, which is why `halfReaches` may read either half of it. That
is not what disjunctive event arms leave. [CR#603.7b] makes exactly one arm's
event cause a delayed ability to trigger — "the next time its trigger event
occurs", and where several occur at once the controller chooses which one
causes it — so under Repeated Reverberation's third arm no spell was cast at
all. A join over the arms would mint a payload whose object half names
nothing, and `ThatHalf SpellW` would read "that spell" on a turn when none was
cast. That is a reference to a non-existent object, not the tolerated spelling
overgeneration the routed ledger names.

What the family actually wants is a different mechanism — a discourse union of
ALTERNATIVES ("the object of whichever arm fired"), which is not `JoinP` and
has no constructor. It is not built here, and nothing would bench if it were:
Repeated Reverberation has a **second, independent blocker** —
`Effect.CopyStack` takes `what : Noun bs Object` gated `OnStack (nounZone
what)`, so a joined "that spell or ability" cannot be its subject whatever the
header announces. (`CounterSpell` is kind-indexed and Shimmering Glasskite
benches on it; `CopyStack` is not.)

### The row: the fifth coordination

`Noun.BothOf : (l : Noun bs k) -> (r : Noun (nomIntro l) k) -> Noun bs k` —
the **same-kind noun conjunction**, the one empty cell in the grid
`EitherOf`'s own docstring enumerates (cross-kind noun conjunction `Both`,
cross-kind head `Joined`, same-kind predicate disjunction `Or`, same-kind noun
disjunction `EitherOf`). `Both` could not write any of these: the kind join is
syntax and never reduces, so `Both` at Object/Object yields `Object \/ Object`
and no Object-taking slot accepts it.

The right arm is read in the LEFT arm's discourse, so each arm mints its own
binding — TWO mentions, never one filter. [CR#601.2c] is the warrant and names
this exact sentence: a spell writing "target" in multiple places may choose the
same object once per instance, "Destroy target artifact and target land" being
the rule's own example.

Projections: `nounPlur = ManyOf`; `nounZone`/`nounTy` project the arms' answer
where they agree and `Nothing` where they do not ([CR#109.2a] — a phrase naming
two places names no one place); `nounTys = SoleTy`, never `JoinTy`, since the
pair is not a join; no joint referent and so `groupMention`/`choosable`/
`anchorPhrase`/`costNounOk` all `False`, as `Both`'s are. Thirteen total-table
rows.

`Noun.EachOfBoth : (pair : Noun bs k) -> {auto 0 pr : CoordinatedPair pair} ->
Noun bs k` — the **distributive over the pair**, a DISTINCT row from the joint
one as the pin requires. Its gate is `coordinatedPair`, true of `BothOf` alone.
It is not `EachOf`, whose `GroupMention` partitions a group an earlier mention
named; a pair leaves no joint referent to partition, and what this distributes
over is the arms.

### The gate split: `parallelDisjuncts` heads vs zones

`parallelDisjuncts` demanded that every arm agree on BOTH `seedZone` and
`seedType`. It now demands the zone of every arm alike, and the type **only of
arms that write no head**:

- A head word CARRIES the type it presupposes — [CR#205.3c] correlates a
  subtype to its own card type — so alternatives that each write a head may
  name different ones. "Enchant creature or Food" names a creature or an
  artifact and is one phrase all the same.
- A head word carries no PLACE. [CR#109.2a] locates a card-worded description
  by the zone the phrase states and `phraseZone` defaults an unstated one to the
  battlefield, so two arms naming two zones would project none and be read onto
  the battlefield. That is mis-placement, not width.

`zonesUniform` is the new half; `seedsUniform` is unchanged and still runs on
headless arms.

### "and/or" — the row it earns, and the deviation

**Deviation from the ticket's pin, flagged.** The pin says "and/or" is its own
truth condition and never an environment of the derived word. Measured, the 365
supported lines split four ways: **77** counted domain, **139** multi-zone
search, **95** type-word, **54** other (kicker costs, mana combinations, tribal
name lists).

At an object DESCRIPTION — the 77 + 95 = 172 lines of buckets (a) and (c), and
the description half of many in (b) ("search your library for up to two basic
land cards and/or Gate cards") — the truth condition IS `Predicate.Or`'s: a
referent answering either alternative or both, counted once. `Or [HasType
Artifact, HasType Creature]` writes it today. Minting a second `Predicate`
constructor there would be two core rows for one term and is exactly the error
`docs/decisions/kind-index-joins-union-marking-is-spelling.md` names — union
marking is spelling, and which of "and", "or" and "and/or" English prints at a
given determiner is spelling-boundary knowledge. So no description-sort row was
minted, and the reason is the umbrella's own cited authority.

Where the word earns a constructor is over ZONES, and there the grammar had
nothing: `SearchScope` offered `OneZone` and a fixed `GraveyardHandLibraryOf`
for one phrasing. That named row is **gone**, replaced by

`SearchScope.SomeZones : (whose : Maybe (Noun bs Player)) -> (zs : List Zone)
-> {auto 0 tw : AtLeastTwoZones zs} -> SearchScope bs`

— ordinary coordination at the zone sort per the same decision, the shape
`EventComplement.FromZones` already took. The clause looks in every zone named
[CR#701.23a] and the count it finds is a count of CARDS rather than one per
zone [CR#701.23d], so the list is the whole construction. The possessor is
written once over the coordination in every printed line ("your graveyard, hand
and/or library"; "that player's graveyard, hand, and library"), so it sits
beside the list and announces once — which is also what keeps Memoricide's and
Eradicate's following "that player" resolving. `AtLeastTwoZones` folds in
distinctness. Two total-table rows, one row deleted; net zero constructors.

### The cross-check that paid: Doc Aurlock needed nothing

`workbench-event-zone-4`'s landed zone coordination is on `EventComplement`,
not on the `Predicate` projection, so it does not reach this section — but the
cross-check found the real answer anyway. `seedZone (CastFrom _)` is `Nothing`:
[CR#601.2a] moves the card OUT of the zone it was in as it is cast, so an
origin is history and not a place the object is. Doc Aurlock's "from your
graveyard or from exile" was therefore parallel already and needed no row at
all. Benched.

### `badCrossZoneDisjunction` KEEPS its refusal, restated

Restated at the pin with the head/zone split above, and joined by a new sibling
`badSpellOrPermanentSubject`: the routed **Lace union subject** turns out to be
this same gap wearing a different head. `Macros.spell = InZone stackZ` seeds
Stack [CR#109.2b] and `Permanent` seeds nothing, so "target spell or permanent"
(the Lace cycle, Blind Seer, Aether Gust, Divide by Zero, Illusion // Reality,
Eight-and-a-Half-Tails, Jeskai Revelation) is refused by the zone projection and
not by any kind or headedness question. Retiring either needs a union-valued
`seedZone`, which is a cross-cutting change to `nounZone` and every `ZoneFits`
gate below it. Patrician Geist stays benched and untouched.

### Measurements (all `jq 'select(.supported)'` first)

| family | umbrella | measured | note |
| --- | ---: | ---: | --- |
| "and/or" lines | 365 | **365** | confirms; 77/139/95/54 by bucket |
| coordinated destination | 1 named | **133** one-verb, **7** two-verb | general, not the move's |
| cross-zone disjunction | 17 | **17** | 14 hand/gy, 2 hand/lib, 1 gy/exile |
| repeated-preposition origin | — | **5** | Doc Aurlock and kin |
| player-plus-player distributive | 39 / 36 | **41 / 38** | bare joint pair still **0** |
| heterogeneous double target | 12 (+1) | **12 (+1)**, plus Reign of Chaos | 13th of the shape |
| protection-shaped | 12 | **8** | overcount; full list in the report |
| coordinated subject | 10 (+1) | **10 (+1)**; 28 in the broader frame | |
| shuffled mass object | 21 | **19** | |

### Benches (`Cards.idr`)

`bileBlight`, `echoingRuin` (coordinated subject, two verbs) · `stompAndHowl`,
`churningEddy` (heterogeneous double target, destroy and move) ·
`secretRendezvous`, `manaClashFlip` (player-plus-player distributive, plain and
per-referent verb) · `weftwalkingShuffle` (coordinated mass object) ·
`sugarCoat` (WHOLE CARD — the kind-crossing disjunction) · `docAurlockCost` ·
`agencyOutfitterSearch`, `deliveryMoogleSearch` (the "and/or" zone list; Agency
Outfitter carries the word at both of its positions in one clause) ·
`concussiveBolt` (routed) · `troubleInPairsArms` (routed) ·
`nongreenSpellsOrAbilities` (the protection phrase, described side).

### Verdicts and zeros

- **Coordinated destination: SPELLING, no row.** 133 supported lines write one
  verb over two destination phrases and 7 write two verbs, so the coordination
  is not the move's own — and the grammar already writes it as two `Move`
  clauses. `Cards.revealFourPartition` has been the standing witness since
  before this round.
- **Protection-shaped: the umbrella's premise is WRONG and the count is 8.**
  The colour is not one modifier distributed over two conjuncts. The spell arm
  tests the SPELL's colour; the ability arm tests its SOURCE's, because an
  ability on the stack has no colour of its own and [CR#113.7] gives it a
  source instead — Gaea's Revenge's own ruling says "all nongreen spells and
  abilities from nongreen sources". The phrase writes today
  (`nongreenSpellsOrAbilities`); the one blocker is the missing TARGETING
  RESTRICTION, there being no "can't be the target of" row in `ObjectAct`,
  `Deed` or `PlayerAct`.
- **Talion: the gate is RIGHT, the shape is a characteristic list.** "mana
  value, power, or toughness" are headless arms presupposing different types
  (none, Creature, Creature), so `seedsUniform` still refuses them and should.
  The honest widening is a `List Characteristic` on `Compare`; 1 supported
  line against a total-table change to a heavily used constructor, so recorded,
  not built.
- **Inspirit, Flagship Vessel: recorded.** "your choice of a +1/+1 counter or
  two charge counters" carries a different AMOUNT per arm over ONE shared
  target mention. `ChosenKind` holds a `List CounterKind` with the amount
  outside the menu, and `Modal` would mint a target per arm [CR#601.2c]. The
  shape wanted is an amount beside each menu arm. 1 line.
- **Trouble in Pairs' shared subject: SPELLING.** English elides the repeated
  subject; the semantics writes each arm's own, and `troubleInPairsArms`
  benches two of the three. Nothing is lost while the body reads no arm. The
  card whole wants an attack header with a defender and a counted attacking
  group — not this ticket's.
- **Avatar Aang: not chased.** "if you've done all four this turn" is a count
  over a coordination's occurrences, and the card also needs the transform
  verb; the four bending verbs are four `verbFacts` rows. Ledgered below.
- **Rukarumel does not bench**, and the coordination is not why: `seedTy` has
  no `HasSubtype` row, so "Slivers you control" projects no head type and
  `AddsChosenQuality`'s `HostedRead` refuses it. The left arm ALONE fails the
  same gate. Ledgered below.
- **Measured zeros stay unbought**: "either … or" as a coordinator 0, "both X
  and Y" as a two-referent coordinator 0, "neither … nor" 12 of which 10 are
  the state idiom.
- The closure grid: this round moved no `negatable` cell — neither new
  constructor is a `Predicate`.

### Ledger — needs routing to live planned tickets

1. **The targeting restriction** ("can't be the target of …", [CR#115.1]) — 8
   supported sentences of the protection shape plus whatever else wants it. The
   described side already writes.
2. **A union-valued `seedZone`** — retires `badCrossZoneDisjunction` and
   `badSpellOrPermanentSubject` together; 17 shared-preposition lines plus the
   "target spell or permanent" family.
3. **`seedTy (HasSubtype s)`** — a subtype-headed phrase projects no head type,
   which is what refuses Rukarumel.
4. **A `List Characteristic` on `Compare`** — Talion, 1 line.
5. **An amount per menu arm on `ChosenKind`** — Inspirit, Flagship Vessel, 1
   line.
6. **The which-zone reader** — "If you search your library this way, shuffle"
   blocks every "and/or" search card whole, the clause itself now writing.
7. **The distributive possessive** — "each player shuffles THEIR hand and
   graveyard into their library" is 19 lines' dominant surface; the "your"
   spelling benches.
8. **A discourse union of alternatives** and **`CopyStack` at a joined kind** —
   Repeated Reverberation's two blockers, refused above.
