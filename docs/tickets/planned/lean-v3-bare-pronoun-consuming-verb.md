---
needs: []
---
# A bare pronoun in an action slot reaches only what the verb can act on

Standard constraints apply. Found by `lean-v3-ninjutsu-defender-lki`, which removed the
public-zone filter (`publicOnly`) from reference resolution (`docs/decisions/semantics-v3.md`
§0.1 item 4). `badHiddenCost` ("Return a creature to its owner's hand: Tap it.") had been
refused only by that filter. Once the filter was gone, the bare-`it` spelling was admitted. The
pin now spells the pronoun `itPermanent` (`.atSlot .permanent`), which is refused.

## The defect

`docs/decisions/semantics-v3.md` §1.5 `J1` item 4, "Consuming-verb applicability", says a
referent is accessible only if the consuming verb or prepositional slot can apply to it in its
zone. A creature that died cannot be sacrificed or have counters put on it as a permanent
[CR#109.2,701.21a]. The checker enforces this only when the pronoun is spelled with a slot
carrier (`itPermanent`, `itCard`: `Reach.atSlot`, `slotZoneOk` in `Check/Words.lean`) or with a
carrier word (`that creature`). A bare `it` (`.pro .bare .one`) never consults the verb, so these
are admitted (scratch `#eval`, 2026-10-22):

| Text | Term | Result |
|---|---|---|
| "Return a creature to its owner's hand: Tap it." | `Ability.activated (.perform (putInto (a isCreature) hand)) (.setStatus .tapped it) …` | `[]` |
| "Return a creature to its owner's hand: Sacrifice it." | the same with `sacrifice it` | `[]` |
| "Destroy target creature. Tap it." | `.sequentially [destroy (target isCreature), .setStatus .tapped it]` | `[]` |

Each `it` resolves to an object in a hand or a graveyard, which tap and sacrifice cannot act on
[CR#701.26a,701.21a]. This is text-decidable: the zone the text moved the object to is known
(a text-decidable zone, not runtime visibility, so it stays on the checker's side of the engine
boundary).

## Ruling (owner, 2026-10-24)

Enforce J1 item 4 in the action stepper, not in templates. Authors write a bare `it`. When the
stepper steps a patient slot that requires a specific zone, J1 item 4 filters out referents whose
known current zone contradicts that slot. The slots are `.battlefield` for `setStatus`,
`turnOver`, `combat`, `clearDamage` and `markDamage`, and an `Action.move` whose written `from_`
is the battlefield. This refuses "Destroy target creature. Tap it." and "Return a creature to its
owner's hand: Tap/Sacrifice it." The earlier choice between template-layer slots and stepper
narrowing (the A1 ruling of `docs/tickets/done/workbench-anaphora-a-bare-it.md`) is settled for
the stepper; no STOP is needed.

## Work

- In the action stepper, where a patient slot with a required zone resolves `.bare`, narrow it
  to the slot's carrier (`Reach.atSlot`, `slotZoneOk`, `Check/Words.lean:1062`; `reaches`, `:1215`).
  Referents whose known current zone contradicts the slot are filtered out; a referent of unknown
  zone is kept (runtime visibility stays the engine's).
- Pin the three rows above as refused, add an accepted pin for a bare `it` whose referent is on
  the battlefield (a permanent the text put or left there), and give `badHiddenCost`
  (`Proofs/Turn.lean:60`) back its bare `it`.

## Added 2026-10-25: retire `itPermanent` (Atraxa audit triage)

Once the stepper narrows a bare `it` in a sacrifice or tap slot, the `itPermanent` Template
(`Templates.lean:95`) is redundant: Oracle prints a bare "it" at every site. Write `it` at each
and delete the Template:

- `Cards/Trigger.lean:612`, *Frost Walker*: "When this creature becomes the target of a spell or
  ability, sacrifice it." A probe on 2026-10-25 found that bare `it` here resolves to the
  targeting spell and checks `[]` today, a wrong reading. After the narrowing (sacrifice is a
  move from the battlefield) it must resolve to the source. Add the pin
  `frostWalkerSacrificesItself` asserting the resolved id is the source's.
- `Cards/Choice.lean:1306` and `Cards/Anaphora.lean:585`, *Soul Ransom*: "This Aura's controller
  sacrifices it". Pins `okSoulRansomRansom` (`Cards/Anaphora.lean:586`),
  `possessiveDeicticIsReadableByIt` (`:593`), `possessiveDescribedBaseUnchanged` (`:606`) stay
  green.
- `Proofs/Turn.lean:62`, `badHiddenCost` (already listed above).

If a site still needs `.atSlot` after the narrowing, keep the Template, STOP and report the site.

**Pattern and lookback moves do not narrow.** A pattern's patient is read after the event
(`did (.move it (graveyardOf .you) (some battlefield) …)`, "if it entered from your graveyard",
reads the permanent as it is now). Guard every narrowing in this ticket with
`if drs.actionPattern then subj else …`; without it `lookingBackAtAnEntryKeepsTheCurrentReferent`
(`Proofs/DrtCards.lean:177`) fails. Refusal rows of this ticket report the narrowed reach
(`.atSlot .permanent`), not `.bare`.

## Added 2026-10-25: narrow to the card carrier and retire `itCard`

A bare `it` whose description demands "card", or the patient of a performed move whose written
origin is a card zone, narrows to `.atSlot .card` [CR#109.2a], so `itCard` (`Templates.lean:92-93`)
is retired. Built on a scratch copy on 2026-10-25; `lake build --wfail` passes (patch:
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/q12-itCard-narrowing.patch`).

Add before `NounPhrase.restContext?` (`Check/Phrase.lean:735`):
```lean
/-- Whether a predicate requires a card [CR#109.2a]: `.isCard` itself or as a conjunct. -/
def Predicate.demandsCard (p : Predicate) (fuel : Nat := 16) : Bool :=
  match fuel, p with
  | _ + 1, .isCard => true
  | n + 1, .inCaller body => body.demandsCard n
  | n + 1, .and ps => ps.any (·.demandsCard n)
  | _, _ => false
termination_by structural fuel

/-- A bare pronoun read in a slot that demands `carrier`, narrowed to it
(`semantics-v3.md` §1.5 `J1` item 4). -/
def NounPhrase.narrowBareTo (carrier : SlotCarrier) (np : NounPhrase) (fuel : Nat := 16) :
    NounPhrase :=
  match fuel, np with
  | _ + 1, .pro .bare pl => .pro (.atSlot carrier) pl
  | n + 1, .inCaller body => .inCaller (body.narrowBareTo carrier n)
  | n + 1, .firstMention id body => .firstMention id (body.narrowBareTo carrier n)
  | n + 1, .laterMention id body => .laterMention id (body.narrowBareTo carrier n)
  | _, np => np
termination_by structural fuel

/-- The carrier a move's written origin demands of its patient: a card zone, "card"
[CR#109.2a]. -/
def Location.originCarrier? (from_ : Location) : Option SlotCarrier :=
  match from_.sort with
  | some z => if isCardZone (some z) then some .card else none
  | none => none
```
Narrow at:
1. the `Condition.matches` arm (`Check/Phrase.lean:3510`), first line:
   `let subj := if pred.demandsCard then subj.narrowBareTo .card else subj`;
2. the `.not (.matches subj predicate)` arm (`:3575`), the same with `predicate`, before
   `if subj.selfDefinedOk`;
3. the `Action.move` arm (`:3941`), before `let (selected, refIds, e1) := ops.np readContext
   patientRole true subj`:
   ```lean
   let subj := match from_.originCarrier? with
     | some c => if drs.actionPattern then subj else subj.narrowBareTo c
     | none => subj
   ```

Respell:
- *Bioplasm* (`Cards/Anaphora.lean:420`, "If it's a creature card, …"): `def bioplasmCardTest :
  Condition := is it (and [isCard, isCreature])`. Probed: it now marks the exiled card
  (referent 3) a creature; bare `it` resolved to Bioplasm (referent 0) before.
- *Vaevictis Asmadi, the Dire* (`:784`, "…then puts it onto the battlefield if it's a permanent
  card"): `doIf (is it isPermanentCard) (putOntoBattlefield it)`. Probed: checks `[]`.
- Delete `itCard` (`Templates.lean:92-93`).

Docs: `lean/CONTRACTS.md:1303` ("*Bioplasm*'s `is itCard isCreature` …") → "`is it (and [isCard,
isCreature])` … the bare pronoun narrowed to the card carrier its description demands
[CR#109.2a]"; ADR `docs/decisions/semantics-v3.md` §1.5 `J1` item 4: a description demanding
"card", and a performed move's written card-zone origin, narrow a bare pronoun; pattern and
lookback moves do not.

Pins:
- Must stay green: `okBioplasmCardTest` (`Cards/Anaphora.lean:421`), `bioplasmTestKeepsCardSlot`
  (`:482`; retitle its docstring to say the narrowed read finds the card),
  `lookingBackAtAnEntryKeepsTheCurrentReferent` (`Proofs/DrtCards.lean:177`).
- Rewritten: `callerConditionReadInItsOwnFrameLosesItsReferent` (`Proofs/CallerScope.lean:481-487`)
  now expects `[.anaphor (.atSlot .card) .one 0]` instead of `[.anaphor .bare .one 0]`; the claim
  (the referent is lost) is unchanged, and the payload names the carrier the slot demands.
- Added, in `Proofs/Anaphora.lean`: `bareItInACardConditionReadsTheCard` (in Bioplasm,
  `resolveCandidates (.atSlot .card) .one (nextClause bioplasmAfterExile)` is `[(3, some .exile)]`,
  and after the condition referent 3 has `ty = [creature]`).

Acceptance (both additions): `./lean/scripts/build` (`--wfail`); `grep -rn 'itCard\|itPermanent'
lean/Semantics` is empty.
