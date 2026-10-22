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

## 2. Merge the universal `bare` into `DetPhrase.all` (owner ruling); the domain half is split out

**Ruling (owner, 2026-10-21).**
- Merge the universal reading of `DetPhrase.bare` into `DetPhrase.all`. A universal bare plural
  ("Players can't gain life", "Creatures you control get +1/+1") and "all …" have the same rules
  meaning and the same plural referent.
- Keep `bare` and `allOf` as distinct Templates.
- Classify each site by its printed text. Every `bare` site that is not the measured domain of
  `countOf`, `exists_` or `aggregate` expands to `.all`.

**Step 1 done (2026-10-21).**
- The `bare` Template expands to `.all`. `countOf`, `aggregate`, `exists_` and `thereIsNo` write
  `DetPhrase.bare` themselves.
- Every `bare` call site but one was universal by its printed text. The exception was
  gravestorm's tally subject, "for each permanent that was put into a graveyard from the
  battlefield this turn" [CR#702.69a]. That is a counted tally domain, not "all permanents", so it
  is now written `a isPermanent`, as storm and *Asmira* write a tally subject.
- The bench stayed green. Pin: `bareAndAllOfExpandToOneTree`.
- The pin `badPluralPlayerDamageRecipient` asserted that "deals 2 damage to opponents" is refused.
  Under the merge that text is an `.all` tree that checks, so the pin is rewritten as
  `okBarePluralPlayerDamageRecipient`. The refusal of a raw domain recipient is kept as
  `measuredDomainIsNotAPerMemberRecipient`.

**The domain half is split out** to
[lean-v3-measured-domain-antecedent](../planned/lean-v3-measured-domain-antecedent.md). It holds the
owner's open question, the audit and its corpus evidence, the seven checks that tell `.bare` from
`.all` and their pins, and a proposal pending the owner's ruling. None of those checks or pins
changes here.

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

- **Done (2026-10-21):** *Zidane, Tantalus Thief* is a `spelled` card (`Cards/Anaphora.lean`).
- **STOP on *Guardian Beast*.** Its last sentence, "This effect doesn't remove Auras already
  attached to those artifacts", has no Core AST form; nor does the Ward cycle's "This effect
  doesn't remove this Aura". Spelling the card without it would put a wrong card on the bench.
  Only the first sentence is pinned, as `guardianBeastProtection` / `okGuardianBeastProtection`
  (`Cards/Deontic.lean`), which carries the `cant (other players) (gainControlOf them)` deontic.
  The owner is deciding on a follow-up ticket for the missing construct.

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
  - Add `ok…` theorems for *Zidane, Tantalus Thief* and *Guardian Beast*. Done for Zidane
    (`okZidaneTantalusThief`); Guardian Beast has only the first-sentence pin
    `okGuardianBeastProtection` (see the STOP under Sites).
  - Add a pin that `changeControl` in a resolving position is refused.
  - Add a pin that `gainControl`'s imperative tree is unchanged.
  - `controlGrantRecordsItsPatientIdentity` (`Proofs/ControllerSacrifice.lean:96`) stays green.
- **Section 2.**
  - After the universal merge, add a pin that "Creatures you control get +1/+1" written with
    `bare` and with `allOf` expand to the same tree.
  - The pins of the seven `.bare`-versus-`.all` checks stay as they are; they belong to
    [lean-v3-measured-domain-antecedent](../planned/lean-v3-measured-domain-antecedent.md).
