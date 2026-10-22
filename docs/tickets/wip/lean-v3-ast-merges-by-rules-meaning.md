---
needs: []
---
# Core AST merges decided by rules meaning (the "Atraxa" review)

Standard constraints apply. This ticket settles which Core AST distinctions to keep and which to
merge. The Core AST here means the semantic expression types in `Words.lean`, `Events.lean` and
`Abilities.lean`. The proposals came from a review of overlapping constructors. The owner ruled
on five of them on 2026-10-21 (sections 1 to 5). The rest are listed at the end, unruled.

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

## 1. `was` is already a Template (owner ruling: nothing to do if so)

Verified 2026-10-21. `Predicate.was` and `Condition.was` (`Templates.lean:2099-2104`) are
Templates that expand to `.did ⟨.any, action, within⟩`. Neither is a constructor. There is no
work here.

## 2. Merge the universal `bare` into `DetPhrase.all`; then audit the domain reading

**Ruling (owner, 2026-10-21, first ruling).**
- Merge `DetPhrase.bare` (`Abilities.lean:277`) into `DetPhrase.all`. A universal bare plural
  ("Players can't gain life", "Creatures you control get +1/+1") and "all …" have the same rules
  meaning and the same plural referent.
- Keep `bare` and `allOf` as distinct Templates.
- First check that no checker rule depends on telling the two apart, and STOP if one does.

**Finding: four checker rules do.** Each treats a bare referent as a *domain* (an uncounted
description) and an "all" referent as an already quantified group:
- **`NounPhrase.countableGroup`** (`Check/Phrase.lean:366-367`) admits `.bare`, not `.all`, as
  what "the number of …" counts (`Amount.countOf`, `:2507-2509`). The pin
  `countRequiresACountableGroup` (`Proofs/RefusalReachability.lean:9-14`) asserts that
  `countOf (bare creature)` checks and `countOf (all creature)` is refused `.countableGroup`.
- **`NounPhrase.existentialMention`** (`Check/Phrase.lean:369-374`) admits only `.bare` as the
  subject of `Condition.exists_` (`:2581-2583`), as in "if there are …". It is pinned by
  `existentialRequiresAnExistentialMention` (`Proofs/RefusalReachability.lean:16-19`).
- **`countGroups`** (`Check/Words.lean:683-685`), which `theRestOk`/`theOtherOk` read, skips
  `.bare` referents, so a counted domain does not compete with the remainder group. It is pinned
  by `bareCountDomainDoesNotCompeteWithTheRemainderGroup` (`Check/Queries.lean:151-159`).
- **`countedGroupSize`** (`Check/Words.lean:726-733`) also skips `.bare` referents.

`DetPhrase.participialNumber?` (`Check/Phrase.lean:284-287`) and `DetPhrase.plur` (`:65-69`)
already treat the two alike.

So `bare` does two jobs:
- the universal bare plural the ruling names, whose rules meaning is "all";
- the domain description that "the number of creatures" and "there are creatures" quantify over.

A plain merge would make the count and existential refusals fire on every domain description, or
force those rules to be dropped.

The first ruling's STOP was taken on this finding.

**Ruling (owner, 2026-10-21, on the STOP).** Merge only the universal reading, then audit the four
checks above against the rules-meaning test. The STOP stays in force for the domain reading until
that audit is done. The work is sequenced:
1. **Merge the universal reading.** Every `bare` site that is not the measured domain of
   `countOf`, `exists_` or `aggregate` expands to `.all`. Examples are a subject of a static or
   deontic clause, or the subject of a grant. The `bare` Template keeps its wording. Classify each
   site by its printed text.
2. **Audit the domain reading.** For each of the four checks, decide whether the distinction it
   draws is one the CR or anaphora resolution draws. The owner's lead: `countGroups` and
   `countedGroupSize` skip `bare` only so that a domain measured inside `countOf` or `exists_`
   ("the number of cards in your graveyard", "if there are cards in your graveyard") does not
   collide with "the rest" or "the other". If those constructs did not publish their measured
   domain into the outer DRS as a group antecedent, `bare` would not be needed there at all.

   Verified 2026-10-21: they do publish it today. The `countOf` (`Check/Phrase.lean:2505-2508`),
   `aggregate` (`:2499-2504`) and `exists_` (`:2581-2583`) arms all return `out`, the DRS after
   stepping the domain phrase, so its referent stays visible to later clauses. The audit therefore
   decides whether a measured domain should be an antecedent at all:
   - if not, close it like a relative-clause box, and the `bare` skips and the domain determiner
     can go;
   - if so, keep a domain determiner and say which rule draws the distinction.

   Bring the result to the owner before removing any of the four checks or their pins.

## 3. `DetPhrase.the` keeps its plurality (owner ruling)

`DetPhrase.the (plur : Plurality)` (`Abilities.lean:274`) stays. "The exiled card" and "the
exiled cards" differ in referent number and in pronoun agreement (`it`/`them`, J1). That is an
anaphora distinction, so it belongs in the Core AST.

## 4. Delete `CounterBatch` and `rollDice`'s `batch`; the amount carries the distinction (owner ruling)

**Ruling (owner, 2026-10-21; replaces the earlier ruling that kept `CounterBatch`).** Delete
`CounterBatch` (`Events.lean:12-18`) and `Action.rollDice`'s `batch : Plurality`
(`Abilities.lean:1095`). The distinction the CR draws is encoded in the amount or quantity the
action already carries:
- "A counter is put" is `.lit 1`, watching each occurrence. An ability triggers once per
  occurrence, and one event can contain several [CR#603.2c]: *Fathom Mage*, *Bloodcrazed
  Hoplite*.
- "One or more counters are put" is `.anyAmount`, watching the event: *Benthic Biomancer*,
  *Simic Ascendancy*. The CR's Saga wording is "When one or more lore counters are put onto this
  Saga" [CR#714.2b].
- Removal uses its `Quantity`. Add `Quantity.theLast` for the last counter: suspend's "When the
  last time counter is removed from this card" [CR#702.62a], a Siege's "When the last defense
  counter is removed" [CR#310.12b].
- Dice follow the same rule through `count`. "Whenever you roll a die" is `.lit 1` (6 cards in
  Oracle text). "Whenever you roll one or more dice" is `.anyAmount` (7 cards).

**Precondition (state the result in the landing).**
- Confirm that nothing reads a pattern's amount or quantity as an exact count. Places to check:
  - the `putCounters`/`removeCounters` arms' `ops.amt` and `stepQuantityWith`;
  - `numberSlots` (`Check/NumberSlots.lean:192-194`);
  - the magnitude a `TallyMode.quantity` tally sums (`Action.eventHasMagnitude`,
    `Check/Events.lean:68-78`);
  - the prospective `thatMuch`/`thatMany` a `would` clause publishes.
- Confirm that "two or more counters" stays expressible. Today it is not expressible either:
  `putCounters` takes an `Amount`, and `CounterBatch` has no "two or more". No Oracle text prints
  "two or more … counters are put" (searched 2026-10-21). If expressing it needs a `Quantity` on
  `putCounters`, say so rather than leave it unwritable.

**Verified pointers (2026-10-21).**
- The checker's `batch != .many` tests are in the `putCounters`/`removeCounters` arms
  (`Check/Phrase.lean:3105-3121`). They become the amount's plurality (`Amount.plur`) and
  `optQ`'s plurality (`Quantity.plur`, with `theLast` reading as the emptying removal).
- The dice test (`batch == .many`, `:3049-3058`) becomes `count.plur`.
- `actionPattern` is set by the lookback (`:979`), `would` (`:1029`), `does` (`:2684`) and the
  deontic clauses (`:3407,3414,3421`). A resolving action never reads the batch.

**Change.**
- Delete `CounterBatch` and the `batch` fields of `Action.putCounters`/`removeCounters`
  (`Abilities.lean:1106-1110`) and `rollDice`.
- Re-sign the Templates that take or default a batch:
  - `putCounters` (`Templates.lean:1500-1502`), `removeCounters` (`:1306-1308`) and
    `removeAllCounters` (`:1314-1316`);
  - `loseCounters` (`:1148-1150`) and `rollDice` (`:1090-1094`);
  - the event Templates `countersPut`, `countersRemoved`, `bareCountersPut`,
    `bareCountersRemoved`, `lastCounterRemovedBy`, `manyCountersPutByEffect`,
    `manyBareCountersPutBy` and `lastCounterRemoved` (`:1611-1649`).
- The explicit `.many` at about 138 resolving card sites disappears with the field: 118 in
  `Cards/Counters.lean`, 10 in `Cards/Mana.lean`, the rest one or two per file, plus
  `Proofs/ActionFamilies.lean:305`.
- The 10 pattern sites take the amount their text prints: `Cards/Counters.lean:322,331,338,349,
  357,371,374,502,1053` and `Cards/Keyword.lean:1227`.
- Update ADR §3.6 (the `counterEvent` and `rollsDice` rows) and `lean/CONTRACTS.md`.

## 5. Control: the imperative is `controlGrant`; `changeControl` stays as the event and deontic head (owner ruling)

**Ruling (owner, 2026-10-21).**
- Unify on `controlGrant`, the layer-2 control-changing effect [CR#613.1b]. The ruling named it
  `ContinuousModification.controlGrant`; the constructor is `StaticSpec.controlGrant`
  (`Abilities.lean:858`).
- Delete `Action.changeControl` only if nothing needs it as an event or deontic head. If
  something does, keep it in that role, make the resolving effect `controlGrant`, and record the
  design.

**Verified state.**
- `gainControl` (`Templates.lean:1376-1381`) already takes an optional `duration` and expands to
  `.establish (.controlGrant controller subject) duration`.
- `Action.changeControl` (`Abilities.lean:1053`) has no Template, Definition or card site.
- `eventHeadOk` (`Check/Events.lean:35`) and `deonticHeadOk` (`:57`) admit `changeControl` but not
  `establish`. So "Whenever an opponent gains control of …" and "can't gain control of …" cannot
  be written through `gainControl`.

**Needed as a head.**
- The CR treats a control change as a trigger event: "abilities that trigger … when a player's
  opponent gains control of an object from that player look back in time" [CR#603.10d].
- *Zidane, Tantalus Thief* prints "Whenever an opponent gains control of a permanent from you".
- *Guardian Beast* prints "other players can't gain control of them". No other card prints that
  deontic wording.
- *Risky Move* ("When you gain control of this enchantment from another player") is the other
  gain-control trigger in Oracle text. Seven cards trigger on "When you lose control of …"
  (*Duplicity*, *Ray of Command*, …), the other half of [CR#603.10d].

**Design (recorded here).**
- "Gain control of X [until …]" as an instruction creates a Continuous Effect with a duration
  [CR#611.2a,613.1b]. Its tree is `establish (controlGrant …) duration`, the same `controlGrant`
  a static ability states.
- An event, lookback or deontic clause names the control change as an occurrence or a forbidden
  act. `establish` is no event or deontic head. Events lower to a closed filter set (ADR §3.6).
- So `Action.changeControl` stays as the head for patterns. Give it a worded Template for the
  pattern moods, and admit it only in pattern positions, as `Cost.anyCost` and `Ballot.anyBallot`
  are (ADR §3.7). The imperative wording then has one tree, `controlGrant`.
- Stamp the pattern's patient with `.core .gainControl`, the stamp the `controlGrant` arm already
  gives (`Check/Phrase.lean:3399-3402`). Then "the permanent … gained control of" reads both alike.
- Whether `changeControl` needs a former-controller field ("from you") or reads it through a
  departure-observed description [CR#603.10d] is the implementer's call. Pin *Zidane* either way.

**Sites.** Author *Zidane, Tantalus Thief* and *Guardian Beast* as `spelled` cards. Neither is on
the bench. The `gainControl` card sites need no change:
- *Mind Flayer* `Cards/Description.lean:78`
- *Phyrexian Infiltrator* `Cards/Anaphora.lean:67-68`
- *Hijack* `:553`
- *Helm of Possession* `Cards/Choice.lean:871`
- *Act of Treason* `Cards/Damage.lean:1331`
- *Blatant Thievery* `Cards/Static.lean:507`

## Not ruled; needs evaluation

None of these is ruled. (`Action.rollDice`'s `batch`, listed here at first, is now ruled in section 4.) Each needs an owner ruling on the rules-meaning question named.
1. **Head nouns (`NounWord`, `Words.lean:309`) unified with Predicate vocabulary**, provided the
   head stays a noun ("that flying" must still be refused). Is a Head Noun's restriction ("this
   creature", "that Aura") the same rules meaning as the matching Card Type or Subtype predicate
   [CR#205.2a]? Or does a Head Noun carry more, such as zone and object class for J1?
2. **`.this (head : Option …)` replacing the 22 object-head `this*` Templates** (`thisCreature`
   … `thisDoor`). Design it together with the spelling-layer audit's self-reference finding.
   Does a printed self-name differ in rules meaning from "this <Head>"? The name refers to the
   particular object it is on [CR#201.5].
3. **Leaf constants (Card Type, Subtype, Supertype, Status, keyword, designation) coercing to
   Predicate**, as authoring sugar. Is a bare `creature` in a predicate slot always `isCreature`,
   with no second reading? This depends on
   [lean-v3-authoring-boilerplate](lean-v3-authoring-boilerplate.md)'s probe.
4. **`controlledBy`/`ownedBy` dispatching by expected type across Predicate and `MoveRider`.**
   "Under its owner's control" places an object [CR#110.2a]; "you control" tests one. Are these
   the same relation in two sorts, or two meanings?
5. **Collapsing the combinatorial move and create Templates into `riders`.** Is "tapped and
   attacking" [CR#508.4] purely Entry Riders, with no meaning beyond them?
6. **`CopyExcept` merged into `CharacteristicEdit`.** A copy exception modifies copiable values
   as part of the copying [CR#707.9b], applied in layer 1 [CR#613.2a]. Is it the same thing as an
   edit applied in its own layer?

## Tests

- **Section 4.**
  - The `ok…` theorems for every rewritten site stay green.
  - Add a pin that "Whenever a +1/+1 counter is put on this creature" (`.lit 1`) and "Whenever
    one or more … are put" (`.anyAmount`) are distinct patterns, with distinct published
    outcomes.
  - Add a pin that `theLast` encodes suspend's last-counter trigger.
  - Add a pin that "roll a die" and "roll one or more dice" are distinct patterns.
  - Rewrite every pin that passes a `CounterBatch` onto the amount form. Keep each claim.
- **Section 5.**
  - Add `ok…` theorems for *Zidane, Tantalus Thief* and *Guardian Beast*.
  - Add a pin that `changeControl` in a resolving position is refused.
  - Add a pin that `gainControl`'s imperative tree is unchanged.
  - `controlGrantRecordsItsPatientIdentity` (`Proofs/ControllerSacrifice.lean:96`) stays green.
- **Section 2.**
  - After the universal merge, add a pin that "Creatures you control get +1/+1" written with
    `bare` and with `allOf` expand to the same tree.
  - The four pins named in section 2 (`countRequiresACountableGroup`,
    `existentialRequiresAnExistentialMention`,
    `bareCountDomainDoesNotCompeteWithTheRemainderGroup`, and the `countedGroupSize` behaviour)
    stay until the audit's result is ruled on.
