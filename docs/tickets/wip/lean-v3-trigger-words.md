---
needs: [lean-v3-gets-printed-zero]
---
# Triggers carry their printed word; "enters" is `enter`; "gets … and gains …" has one tree

Standard constraints apply. This covers finding B2 of the warts audit (the triggered family),
finding 5 of the spelling-layer audit ("enters"), and the gets-and-gains split.

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes (the semantic expression types in
  `Words.lean`, `Events.lean` and `Abilities.lean`) or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

## Rulings (owner, 2026-10-21)

1. **Trigger words.**
   - Fold the mechanical `triggered` sites onto `when`, `whenever` and `at_`, each carrying the
     word its text prints.
   - Give those three Templates optional parameters, so the variant Templates collapse into
     them.
2. **"Enters".** Fix the triggers in `Cards/Piles.lean` written
   `when (does any (putOntoBattlefield <self>))` where the card prints "When this … enters".
3. **Gets and gains.**
   - Unify the four encodings of "<subject> gets +X/+Y and gains <keyword> until end of turn".
   - Coordinate with [lean-v3-gets-printed-zero](lean-v3-gets-printed-zero.md), and state which
     lands first.

4. **Optional named parameters (owner, 2026-10-21, option A).**
   [Card authoring binds no implicits](../../decisions/card-authoring-binds-no-implicits.md) is
   superseded for the Lean workbench: Templates may take optional named parameters with defaults,
   placed after the required ones, as `dealDamage`'s `kind` and the deontic Templates' `timing`
   already do. The ADR carries the dated supersession note.

## 1. Trigger words

**Verified state (2026-10-21).**
- `when`, `whenever`, `at_` and `after` (`Templates.lean:1814-1824`) all expand through
  `triggered` (`:1786`).
- Eleven variants each fill one more field of `Ability.triggered` (`Abilities.lean:1197-1199`):
  `triggeredIf`, `whenIf`, `wheneverIf`, `atIf` (`:1711-1726`), `triggeredOnlyOnce` (`:1728`),
  `triggeredJoined` (`:1789`), `triggeredOr` (`:1795`), `triggeredWhile` (`:1799`),
  `triggeredOnlyDuring` (`:1803`), `triggeredOnlyOnceDuring` (`:1807`) and `triggeredOrOnlyOnce`
  (`:1994`).
- No combination outside that list can be written. For example, "Whenever …, if …, … This ability
  triggers only once each turn" has no Template.

**Change.**
- Give `when`, `whenever` and `at_` (and `after`, the results-table word) these optional named
  parameters, all defaulting to empty:
  - `(or_ : List GameEvent := [])`
  - `(joins : List JoinedHeader := [])`
  - `(while_ : Option Timing := none)`
  - `(during : Option Timing := none)`
  - `(limit : Option UsageLimit := none)`
  - `(if_ : Option Condition := none)`

  This is the shape the deontic Templates already have.
- Retire `triggered` and the eleven variants. Every site names its printed word.
- An intervening "if" is a field, not a word [CR#603.4]: "When/Whenever/At [trigger event], if
  [condition], [effect]".

**Plain `triggered` sites.**
- 36 in `Definitions/Keywords.lean`.
- 3 in `Definitions/Actions.lean`: the Ring-bearer abilities at `:758,764,772`.
- 4 in Cards: *Iron Man, Bleeding Edge* `Cards/Copy.lean:151`, *Donal, Herald of Wings* `:164`,
  *Xenosquirrels* `Cards/Description.lean:495`, *Frenzied Gorespawn* `Cards/Keyword.lean:1219`.
- 7 in Proofs.

Each card's word comes from its Oracle text. Each keyword's word comes from its CR definition:

| Word | Keywords (`Definitions/Keywords.lean` line, CR rule) |
|---|---|
| When | backup `:279` [CR#702.165a]; blitz `:330` [CR#702.152a]; champion `:361,367` [CR#702.72a]; decayed `:462` [CR#702.147a]; demonstrate `:474` [CR#702.144a]; exploit `:656` [CR#702.110a]; fabricate `:665` [CR#702.123a]; For Mirrodin! `:705` [CR#702.163a]; gravestorm `:768` [CR#702.69a]; job select `:901` [CR#702.182a]; living weapon `:924` [CR#702.92a]; madness `:934` [CR#702.35a]; recover `:1214` [CR#702.59a]; soulshift `:1322` [CR#702.46a]; vanishing's last-counter trigger `:1526` [CR#702.63a] |
| Whenever | afflict `:200` [CR#702.130a]; annihilator `:230` [CR#702.86a]; battle cry `:308` [CR#702.91a]; cipher `:386` [CR#702.99a]; dethrone `:483` [CR#702.105a]; extort `:660` [CR#702.101a]; firebending `:682` [CR#702.189a]; flanking `:689` [CR#702.25a]; ingest `:889` [CR#702.115a]; melee `:963` [CR#702.121a]; mentor `:980` [CR#702.134a]; mobilize `:987` [CR#702.181a]; myriad `:1017` [CR#702.116a]; poisonous `:1105` [CR#702.70a]; provoke `:1141` [CR#702.39a]; prowess `:1150` [CR#702.108a]; rampage `:1163` [CR#702.23a]; training `:1431` [CR#702.149a]; ward `:1531` [CR#702.21a] |
| At | fading `:671` [CR#702.32a] |

**Variant sites** (counted 2026-10-21): `whenIf` 20 card / 14 definition / 3 proof; `wheneverIf`
13 / 5; `atIf` 24 / 3; `triggeredOnlyOnce` 6 card; `triggeredOr` 8 card / 1 definition (bushido,
`Definitions/Keywords.lean:96`); `triggeredWhile` 5 card; `triggeredJoined` 3 card; `triggeredOnlyDuring` 2 card;
`triggeredOnlyOnceDuring` 1 card. `rg` for each name lists them.

## 2. "Enters"

The intransitive `enter` is the one encoding of a printed "enter(s)"/"entered" (ADR §4.2,
`docs/decisions/semantics-v3.md`). Rewrite the ruled sites:
- *Tezzeret's Gatebreaker* `Cards/Piles.lean:21` ("When this artifact enters"): becomes
  `when (does thisArtifact enter)`.
- *Orchard Elemental* `:80`, *Custodi Squire* `:124`, *Lieutenants of the Guard* `:140` ("When
  this creature enters"): each becomes `when (does thisCreature enter)`.

The same finding covers these sites, which print "enter"/"entered" and write a put Template.
Rewrite them onto `enter` too:
- "entered this turn": *Force of Despair* `Cards/Trigger.lean:119` and *Cradle to Grave* `:123`
  ("that entered this turn"), *Dark Fortress* `Cards/Mana.lean:684` ("this land entered this
  turn"), *Tromell, Seymour's Butler* `Cards/Counters.lean:444`, power-up
  `Definitions/Keywords.lean:1112` ("If this permanent entered this turn" [CR#702.193a]), and
  read ahead `:1191` ("the turn it entered the battlefield" [CR#702.155a]).
- "would enter": *Containment Priest* `Cards/Keyword.lean:1191` and *Hallowed Moonlight*
  `Cards/Trigger.lean:733` ("if a … creature would enter").

Mutate's replacement (`Definitions/Keywords.lean:1008`) watches a resolving creature spell that,
per [CR#702.140c], "doesn't enter the battlefield". Check its wording against that rule before
changing it.

## 3. Gets and gains

**Encodings today** (Oracle verified 2026-10-21):
- **(a)** `establishFor S [getsPt S …, abilityGrant S K] d`, with the subject repeated:
  *Ghor-Clan Rampager* `Cards/Cost.lean:731-735`.
- **(b)** `establishThroughout (conjunction [getsPt S …, abilityGrant it K]) d`:
  - *Elspeth's Talent* `Cards/Cost.lean:719-722`
  - *Chittering Host* `Cards/Faces.lean:381-385`
  - *Awaken the Bear* `Cards/Keyword.lean:352-354`
  - *Aim High* `:1155-1157`
  - *Volatile Claws* `Cards/Static.lean:440-444` ("get +2/+0 and gain all creature types")
- **(c)** `sequentially [get S … d, gain it K d]`, two `establish`es: *Archpriest of Iona*
  `Cards/Description.lean:512-516`.
- **(d)** a `modification` pair inside a `conjunction`: *Arlinn Kord* `Cards/Faces.lean:295-299`,
  and *Berserk* `Cards/Turn.lean:512-516` ("gains trample and gets +X/+0").

**Unified tree.** One sentence with one written subject and coordinated predicates is one
`establish` of a `conjunction` that shares the subject:
`establishFor S [getsPt it P T, abilityGrant it K, …] (some untilEndOfTurn)`, with `them` for a
plural subject.
- The subject is stepped once.
- The power/toughness change [CR#613.4c] and the ability grant [CR#613.1f] apply in their own
  layers from that one Continuous Effect [CR#611.2a].
- Form (a) already has this shape but repeats the subject. Writing the subject once also closes
  the *Berserk* and *Ghor-Clan Rampager* item of
  [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3 ("Bind
  `target isCreature` once"), which this ticket takes over.
- A Template may spell the frame ("gets … and gains …", and *Berserk*'s "gains … and gets …"
  order) over that tree.

**Order.** [lean-v3-gets-printed-zero](lean-v3-gets-printed-zero.md) lands first. It turns
*Arlinn Kord*'s and *Berserk*'s `modification` pairs into one `getsPt` carrying the printed
"+0". This ticket then moves those `getsPt`s under the shared-subject `establishFor`. Hence the
`needs:` edge.

## Overlaps

- [lean-v3-move-deeds-and-verbs](lean-v3-move-deeds-and-verbs.md) touches *Custodi Squire*
  (`Cards/Piles.lean:130`).
- [lean-v3-spell-damage-dealer](lean-v3-spell-damage-dealer.md) touches *Arlinn, Embraced by the
  Moon* (`Cards/Faces.lean:324`).
- [lean-v3-another-contrast-anchor](lean-v3-another-contrast-anchor.md) touches battle cry
  `:308`, backup `:279`, training `:1431`, champion `:361` and *Chittering Host*'s "other".

Whichever lands second rebases.

## Tests

- Every `ok…` theorem on a rewritten site stays green under `./lean/scripts/build` (`--wfail`).
- **Add** a pin that one trigger combining `if_` and `limit` is writable and checks. No Template
  admits that combination today.
- **Add** a pin that *Tezzeret's Gatebreaker*'s trigger is the `enter` pattern with this artifact
  as Actor.
- **Add** a pin that the gets-and-gains cards share one shape: one `establish`, one
  shared-subject `conjunction`, and one stepped subject referent. Assert the shape, not a full
  `rfl` restatement.
- **Rewrite** the 7 proof sites that call `triggered` onto the worded Templates. Keep each one's
  claim.
