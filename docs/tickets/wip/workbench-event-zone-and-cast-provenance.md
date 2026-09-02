---
needs: []
---
# Read events by verb and zone, and finish the cast relation's provenance — umbrella

**Umbrella record; claim the sub-tickets, not this one.** This ticket is
split into four independently-claimable sub-rounds plus one routed mint,
below. It carries no build work of its own.

## Split, 2026-08-26

The unit over the zone-change event, the event-history lookback and the cast
relation was split into four semantically independent sub-rounds (none needs
another's output; each has its own whole-card witness):

- **`workbench-event-zone-1-verbed-event.md`** — the mill/discard EVENT
  reading of a verb (the `VerbedAct`-style label-carrying `EventName`). Size
  S.
- **`workbench-event-zone-2-event-subjects.md`** — the dealer-side read of
  damage in general, and the player-subject attack declaration
  (`AttacksWith`). Size S.
- **`workbench-event-zone-3-targeting-and-disjunction-arms.md`** — the
  targeting relative clause (both voices: `BecomesTarget` event and the
  targeting `Predicate`), and the un-gated `Leaves`, as the n-ary event
  disjunction's arms. Size M.
- **`workbench-event-zone-4-zone-catalog-and-reader-payload.md`** — the
  `Command` zone row, the sorted `EventComplement` payload (zone + coordination),
  and the cast-origin rider, as one mechanism. Size M.

Sequencing: serialise 1 → 2 → 3 (they share the same `EventName`/`Triggers`
table block); sub-round 4 runs in parallel with any of them. If only one
sub-round runs, it is sub-round 1 — its pin is architectural and the discard
family (96 lines) is the largest single unblock in the bundle.

Each sub-round ticket carries, self-contained: the §0 stale-pointer
corrections table below, the §1 constraint lists below, its own item list
with quoted asks, its own re-measured corpus counts, its settled rulings,
and its own open pins **now settled** by conductor ratification
(2026-08-26) — pinned, do not reopen.

**One routed mint**, off the §6 dispositions below:
`docs/tickets/planned/workbench-element-binder-reads.md` — the ordinal
cast's residual 5 plus the Once Upon a Time re-check, and the per-member
event count inside `AggregateOver`'s binder body.

## Stale pointers this ticket's original body carried

The original body's pointers had drifted from the tree — it named tables
that no longer exist (`sameZone`, `searchableZone`), a pin that isn't there
(`badCastInGraveyard`), machinery that has been retired (`eventUse`,
`eventSpan`, `replUseOk`, `triggerWordOk`), an `EventName` row count that is
stale (26, not 34), and a "commander designation is settled in the same
round" line that is simply false (`CommanderD` already exists — the `Zone`
row is the actual gap). **The four sub-tickets carry the corrected
pointers and constraint lists in full** (their own §0 and §1); this
umbrella does not repeat them.

## §6 dispositions — not worth a sub-round, transcribed

Two rows below are corrected from the split report: the report routed two
items to `docs/tickets/done/workbench-amount-comparison-and-quantity.md`,
which is **closed** — done tickets are not plans. Those two route instead to
the newly-minted `docs/tickets/planned/workbench-element-binder-reads.md`.
One row (Doc Aurlock) names `workbench-conditional-and-coordination`, which
is also closed; its live successor `workbench-coordination-family.md` has
been given the cross-check line directly.

| Item | Disposition |
|---|---|
| *"The ORDINAL cast — 5 lines, plus 20 in trigger headers"* | **Half delivered, half misrouted.** `GameEvent.NthOccurrence` (`Triggers.idr:348`) landed in the event-algebra round and its docstring gives the cast spelling verbatim — the 20 trigger headers write today. The residual 5 (Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth Doctor, Zimone) want a **noun-phrase element binder over a turn's casts**. **Corrected routing: `docs/tickets/planned/workbench-element-binder-reads.md`** (not `workbench-amount-comparison-and-quantity`, which is closed). The next-spell/first-spell keyword grants stay blocked behind the binder wherever it lands. |
| *"The zone DISJUNCTION on a cast's origin"* (Doc Aurlock, Aven Interrupter, Soulless Jailer) | **The parent contradicted itself.** Its "Closed, do not redo" section said: "DOC AURLOCK still does not land, on the zone DISJUNCTION … which belongs to `workbench-conditional-and-coordination`" — closed. **Corrected routing:** the cross-check ("if sub-round 4's zone-coordination pin lands, Doc Aurlock may fall out for free") has been appended to `docs/tickets/planned/workbench-coordination-family.md`, that ticket's live successor. |
| *"The LIBRARY cell of `CastFrom`"* | **Not a sub-round.** The cell is already **open** — `playableFrom (Just Library) = True`, so `CastFrom (ZoneAt Library …)` type-checks today. Only a whole-card witness is owed, and both candidates carry a second unbuilt thing (Melek: a play permission over the top of a library; Fblthp: a coordinated "entered from your library **or** was cast from your library"). Rides sub-round 4 as a recorded ledger line (carried in that ticket's item list). *Note:* Fblthp's **second** ability benches once sub-round 3 lands, but that does not bench this cell — Fblthp's first ability is the one that writes it. |
| *"The four delayed end-of-combat forms"* | **Unchanged and confirmed by the parent itself**: blocked on the "At this turn's next end of combat" delayed shell, "which no current machinery reaches". Carry forward as a ledger line; no sub-round takes them. |
| *"Recorded measurement, not a gap"* — `negatable (CastBy _)` is False | **Not an item; a re-measure trigger.** It binds any round that mints the agentless passive ("target spell you control that wasn't cast", Errant, Street Artist). Sub-round 3 mints a `Predicate` row and so must **not** touch `negatable`'s `CastBy` cell; if it happens to mint the passive, the cell is re-measured rather than assumed. **Kept as a standing acceptance line on this umbrella** — see below. |
| Routed: *"`lookbackComplementOk` refuses every joined complement, on a corpus zero"* | **A recorded fact, not a gap.** It is a re-measure trigger for sub-round 2 (whose new event's complement is exactly a "to X" phrase that could coordinate) and for sub-round 4 if the payload lands on `lookbackComplementOk`. Cited in both briefs; not scheduled anywhere. |
| Routed: *"Once Upon a Time's history identity read"* | **Dormant.** "Neither built nor pinned; no rule makes it meaningless and no buildable card pays it." Worth one cheap re-check now that `NthOccurrence` exists. **Corrected routing: `docs/tickets/planned/workbench-element-binder-reads.md`**, assigned to whoever takes the ordinal binder there (not `workbench-amount-comparison-and-quantity`, closed). |
| Routed: *"The per-member event count inside `AggregateOver`'s binder body"* | **Belongs with the element binder**, same as the ordinal residue — Thought Sponge and the Windfall/Jace's Archivist cycle need a per-member event subject inside a binder body. **Corrected routing: `docs/tickets/planned/workbench-element-binder-reads.md`**, together with the ordinal 5 (not `workbench-amount-comparison-and-quantity`, closed). |
| *"Not this round's, recorded so they are not folded in"* — the MAGNITUDE read, the coordinated history read, the PROHIBITION family, the mana-SPEND purpose, Continue?, Bill Ferny | **Already fenced out.** Reproduced verbatim below as the standing do-not-fold-in list; no sub-round owns any of them. |
| The closing paragraph, *"The `eventUse` / `eventSpan` / `ReplUse` triple over 26 `EventName` rows"* | **Stale — the tables no longer exist.** Replaced by the live seven-table list in each sub-ticket's §1.1, with a note that the closure-tables document's §2.2 is a demoted 2026-08-22 snapshot. |

## Standing acceptance lines that remain on this umbrella

These do not belong to any one sub-round; they bind whichever round happens
to touch the relevant cell.

- **`negatable (CastBy _)` re-measure trigger.** The AGENTIVE negated cast
  ("a spell you didn't cast") is ZERO lines, so `negatable (CastBy _)` is
  False — but what the phrase would mean is attested one card over through
  an agentless passive ("target spell you control that wasn't cast", Errant,
  Street Artist), which is why the cell is REFUSED and not pinned. **A round
  that mints the passive must re-measure the cell rather than assume it.**

- **Do-not-fold-in fence** — reproduced verbatim from this ticket's original
  body; no sub-round owns any of these:
  - The MAGNITUDE read (`EventSum`) — "if you gained 3 or more life this
    turn" (15 lines) and Gollum, Obsessed Stalker's "life equal to the
    amount of life you gained this turn". A complement is a participant and
    this is a quantity; it stays ledgered on `EventCount`'s own row.
  - The coordinated history read, 4 lines — "blocked or was blocked by a
    Zombie this turn" (Time to Reflect, You Cannot Pass!, Venomous Breath,
    Sea Troll): ONE complement shared across two events, which is the
    coordinated EVENT and not this slot.
  - The PROHIBITION family, 8 lines — "Players can't cast spells from
    graveyards or libraries" (Grafdigger's Cage, Weathered Runestone,
    Kunoros, Soulless Jailer, Ashes of the Abhorrent, Drannith Magistrate,
    Avatar's Wrath, Experimental Frenzy). This is the PERMISSION's negation
    and belongs to `MayPlay`/`PlaySource`.
  - The mana-SPEND purpose, 7 lines — a zone-qualified `SpendPurpose`, which
    belongs with the spend restrictions that name a cost.
  - Continue?, whose zone-change surface is a different spelling from the
    past verb these rows write; and Bill Ferny, still blocked on the
    predefined-token catalog ([CR#111.10]) alone.

Standard constraints apply to every sub-ticket claimed off this split.

- **Routed from workbench-choice-e-subtype-words-and-linkage (close, 2026-08-27):**
  a POSSESSED zone is not a move destination — `DestOk` admits bare zones only,
  so "Put a card exiled with this Saga into its owner's hand" (Roads Go Ever,
  Ever On chapters II/III) does not write. A move-destination gap, not a
  linkage one; the linkage read itself lands.

- **Routed from workbench-coordination-2-noun-and-phrase (close, 2026-08-27):**
  the WHICH-ZONE reader — after a multi-zone search (`SearchScope.SomeZones`,
  landed) nothing reads which zone the found card came from, which blocks
  every and/or-search card whole. A zone-provenance read, so it lands here.

- **Routed from workbench-mana-family-residues (close, 2026-08-28):** the
  SPELL-TO-PERMANENT read — "if that mana is spent to cast a creature spell,
  that creature gains haste" needs the permanent the resolved spell becomes;
  9 of the 11 paid-for-object cells wait on it (the grammar correctly refuses
  granting haste to a stack object). Zone-crossing referent machinery, so it
  lands here.
