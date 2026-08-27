---
needs: []
---
# Retire or record: the verbed event's overlap with three dedicated rows

Routed from `workbench-event-zone-1-verbed-event` (close, 2026-08-26).
`VerbedEvent` (the label-carrying act event) overlaps three dedicated
`GameEvent` rows — `IsDestroyed` [CR#701.8a], `StatusEvent`/tapped
[CR#701.26a], and `PutInto` — so one happening has two writable terms.
Refused by no rule; named in the row's docstring. Decide retire-vs-record
per row: retiring a dedicated row must not lose its richer slots (e.g.
`PutInto`'s source/destination) — if the verbed form cannot carry them, the
dedicated row stays and the overlap is recorded as deliberate, with the
spelling boundary told which term each printed form elaborates to.

## Consumption boundary

`idris/src/Experimental/Triggers.idr`, `Events.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- One written verdict per overlapping row; no witness lost either way.
- `idris/scripts/build` PASS, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-proliferate-row (close, 2026-08-27):** the proliferate EVENT — "Whenever you proliferate during your turn" (Contagion Dispenser). PROBE FIRST: `VerbedEvent (Just You) "Proliferate" Nothing` may already compose (the `verbFacts` row landed with no patient); if it does, bench it here and record delivered; if not, the gap is this ticket's verbed-event machinery.

## As landed (2026-08-27)

### The three verdicts

**`IsDestroyed` — RETIRED into the verbed form.**

> "Destruction is the verbed event, and `IsDestroyed` said strictly less
> about it. The passive verbed reading spells the same happening in the
> same voice: [CR#701.8b] confines destruction to an effect that uses the
> word or to the lethal-damage/deathtouch state-based actions
> [CR#704.5g,704.5h], all of which the actorless voice covers without
> claiming a destroyer. What the dedicated row had over it was one gate —
> [CR#701.8a]'s battlefield confinement — and that gate is data about the
> ACT, so it moves onto `VerbFacts` where every other act gets it too.
> What the verbed row has over the dedicated one is announcement: it
> stamps the patient with its participle, so 'if a creature is destroyed
> this way' reads back, and its `VerbedAct` classifier opens the
> 'was destroyed this turn' lookback that `Destruction` closed with
> `lookbackSubjectOk Destruction Object = False` — a cell the corpus
> contradicts."

Moved: `GameEvent`'s `IsDestroyed` row and its four table cells
(`eventName`, `eventIntro`, `eventAfter`, `eventSubjectPlur`);
`EventName`'s `Destruction` arm and its five cells (`sameEventName` ×2,
`eventHasMagnitude`, `lookbackSubjectOk` ×2) — no other table named it.
The one witness, `clergyOfTheHolyNimbus`, is now
`Intercepts (VerbedEvent Nothing "Destroy" (Just thisCreature))`;
`interceptOk (VerbedAct _) = True` already supplied its gate.

**`StatusEvent` on a tap — RECORDED, deliberate.**

> "Not two terms for one event: two events the rules keep apart.
> [CR#508.1f] says attacking 'simply causes creatures to become tapped'
> and that the tapping is not a cost — a status change [CR#110.5c] with
> no effect performing [CR#701.26a]'s act at all. The corpus proves the
> same from the other side: 'Whenever a creature an opponent controls
> becomes tapped, if it isn't being declared as an attacker' has to
> exclude by hand exactly what the status reading catches and the act
> reading never would. The two printed forms are also disjoint — the
> status form writes no actor, the act form writes nothing else — so the
> elaboration boundary is unambiguous."

Elaboration mapping told to the spelling boundary:
`"[n] becomes tapped"` → `StatusEvent {c = TapC} n Tapped` (97 supported
header lines); `"[who] tap(s) [what]"` → `VerbedEvent (Just who) "Tap"
(Just what)` (39 lines, all with an adjunct — see the ledger). The
passive verbed tap (`VerbedEvent Nothing "Tap" _`, "[n] is tapped" as a
header) is unattested at 0 and refused by no rule: tolerated
overgeneration, recorded at its zero.

**`PutInto` — RECORDED, deliberate; the dedicated row stays.**

> "The verbed form cannot carry the slots. `PutInto` writes both ends —
> a required `ZoneExpr` destination gated by `placementDestOk` and an
> `EventSource` origin gated by `placementOriginOk` — and `VerbedEvent`
> has neither slot and no way to acquire one from `verbFacts`: 'Put' is
> no [CR#701] keyword action and states no zone of its own, which is
> precisely why its `actDest` and `actZone` are both `Nothing`. 251
> supported lines write the placement header, and every one of them
> names the destination ('is put into a graveyard from the battlefield').
> The ticket's own condition decides it: retiring would lose the richer
> slots, so the dedicated row stays."

Elaboration mapping: `"[n] is/are put into [to] from [from]"` →
`PutInto n to from`. `VerbedEvent _ "Put" _` remains spellable, says less
(no destination, an unreadable stamp — `participleOf "Put" = Nothing`),
and is attested at 0; it is tolerated overgeneration recorded at its
zero, not pinned — no rule makes "[who] put(s) [what]" meaningless, only
poorer. The "Put" row's comment in `verbFacts` now says so.

### The mechanism the retirement bought

`VerbFacts` gains a third rule-bearing field beside `actPatient` and
`actDest`:

- `actZone : Maybe Zone` — where the act's own rule FINDS its patient.
  `Just Battlefield` for Destroy [CR#701.8a] and Sacrifice [CR#701.21a]
  and Tap [CR#701.26a]; `Just Hand` for Discard [CR#701.9a];
  `Just Library` for Mill [CR#701.17a]; `Nothing` for Exile
  ([CR#701.13a] takes it "from wherever it is"), for Put (the clause
  names the zone), and for the patientless acts. Read back by
  `actZoneOf`.

`VerbedEvent` gains `{auto 0 zn : ZoneFits (patientZone what)
(actZoneOf v)}`, with `patientZone` beside `agentIntro` in the
`Triggers.idr` mutual block. `zoneFits` passes an unwritten zone on
either side, so patientless acts and nouns that name no zone stay
ungated; the gate bites only where a written patient names a zone the
act's rule cannot reach. This is a widening fix beyond the retirement:
before it, "you discard a permanent you control" was spellable.

### The probe — DELIVERED

`VerbedEvent (Just You) "Proliferate" Nothing` composes as landed, with
no change to the verbed-event machinery. `KnownVerb` finds the
`"Proliferate"` row (minted with the round-1 field work);
`VerbPatient`'s `ActOnNothing` takes it because
`actPatientOf "Proliferate" = Nothing` [CR#701.34a]; `VerbedVoice`'s
`ActiveAct` takes the named actor. No gap.

Benched as a whole card: `schemingAspirant` — Scheming Aspirant,
{1}{B} 1/3, "Whenever you proliferate, each opponent loses 2 life and
you gain 2 life." Header entire plus body.

The card the routed note named, Contagion Dispenser, is not in the
supported corpus. The supported proliferate-event headers are four:
Scheming Aspirant, Ezuri, Stalker of Spheres ("Whenever you proliferate,
draw a card"), Venser, Corpse Puppet ("…, choose one —"), and Voidwing
Hybrid ("When you proliferate, return this card from your graveyard to
your hand"). None writes a window, so no "during your turn" is needed.

### Corpus, measured (supported faces)

- `"Whenever/When … is/are destroyed"` header: **0**. The destruction
  event's whole printed use is the REPLACEMENT — 54 "would be destroyed"
  lines, of which the regeneration reminder is the bulk. The one active
  destroy header is causer-voiced (see ledger).
- `"is/are destroyed"` anywhere: **5**, all `this way` readbacks — which
  is what the retirement's participle stamp now serves.
- `"becomes tapped"` header: **97**. `"Whenever [who] tap(s) …"` header:
  **39**.
- `"is/are put into/onto"` header: **251**. `"Whenever you put …"`
  header: **26**, all counter events (`CounterEvent`), none a placement.
- proliferate-event headers: **4** (above).

### Witnesses and pins

- `clergyOfTheHolyNimbus` — migrated, whole ability, unchanged in
  meaning.
- `schemingAspirant` — new whole card, the probe.
- `badDestroyInGraveyard` (`ProofsG.idr`) — "a card in a graveyard is
  destroyed", refused by the new zone gate [CR#701.8a,701.8b]. This is
  the retirement's lossless-ness made a witness: it is exactly the
  refusal `IsDestroyed`'s battlefield gate carried.
- `badDiscardFromBattlefield` (`ProofsG.idr`) — "you discard a permanent
  you control", [CR#701.9a]. The gate is not destroy-specific.

Both pins fail closed: the build accepts the `impossible` clauses, so
neither is silently passing.

### Ledger — needs routing at integrate

- **The causer-voiced act.** "Whenever a spell or ability an opponent
  controls destroys a noncreature permanent you control" (1 line) — the
  only active destroy header printed, and `VerbedVoice` has no causer
  arm beside `ActiveAct`/`PassiveAct`. `CounterEvent`/`TokensCreated`
  carry a `Causer` slot; the verbed event does not.
- **The act lookback with a causer.** "If a noncreature permanent under
  your control was destroyed this turn by a spell or ability an opponent
  controlled" — the subject cell is now open
  (`lookbackSubjectOk (VerbedAct "Destroy") Object = True`), but the
  by-source agent phrase on a lookback is unbuilt (it is the same
  by-source ledger tag the `GameEvent` docstring already names).
- **"for mana" on the tap act.** All 39 active-tap headers carry an
  adjunct; 35 of them are "for mana"/"for {C}", which no slot on
  `VerbedEvent` spells. The header without it is spellable today; the printed ones are
  not.
- **"if it isn't being declared as an attacker"** — the intervening
  condition on a becomes-tapped header (2 lines) names the attack
  declaration as a state, which no `Condition` writes.

Standard constraints applied; `idris/scripts/build` PASS (23/23, 0
errors, 0 warnings), `cargo xtask cite check` 0 stale, 0 non-compliant,
21 audited citation sites read against their rule text.
