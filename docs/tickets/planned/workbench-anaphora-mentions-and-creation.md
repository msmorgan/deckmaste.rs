---
needs: []
---
# Settle what a later phrase may read back, and finish the mention surfaces — umbrella record

**Non-claimable umbrella. Claim the sub-tickets, not this one.** Split
2026-08-27 into five sub-tickets after a completed split analysis found the
body's own pointers had drifted badly against the current tree (see each
sub-ticket's own §0). This file now carries only the disposition record for
items that turned out not to be their own sub-round, and the standing fences
that must not be re-proposed by anything that reads this family later.

## The five sub-tickets

- `docs/tickets/planned/workbench-anaphora-a-bare-it.md` — Sub-round A. What a
  bare "it" resolves to; sort-scoped anaphor resolution. Size L, the
  architectural round, runs first.
- `docs/tickets/planned/workbench-anaphora-b-verb-provenance.md` — Sub-round
  B. Verb provenance on the bare anaphor, and the stamp payload. Size M,
  independent of A.
- `docs/tickets/planned/workbench-anaphora-c-announcement-audit.md` —
  Sub-round C. What a clause announces: the intro/delta audit. Size L,
  interlocks with A, runs after it.
- `docs/tickets/planned/workbench-anaphora-d-creation-and-replacement.md` —
  Sub-round D. The creation clause and the replacement side. Size M (may
  shrink to S), independent of A/B/C.
- `docs/tickets/planned/workbench-anaphora-e-partitive-surfaces.md` —
  Sub-round E. Partitive, possessor and determiner surfaces. Size S, fully
  independent of A–D.

Sequencing across all five: A first; C after A; B/D/E parallel-safe with A and
with each other.

## Dispositions — not worth a sub-round

Transcribed verbatim from the split report.

| Item | Disposition |
|---|---|
| *"Cross-ability memory — a both-layer gap"* (Phyrexian Ingester, Drach'Nyen; "the exiled creature card's power") | **Belongs elsewhere, and is close to settled against.** §1.3's no-cross-ability-discourse ruling forecloses threading a mention between abilities; whatever survives is a **card-scope forward obligation** and belongs with `docs/tickets/planned/workbench-choice-chosen-and-ascription.md`'s face law, not with a binder change here. The parent's own note that both pieces are *"discourse-local by construction, the discourse being one ability wide"* is the ruling arriving early. Routed with a one-line cross-check. |
| *"The plural read-back mention"* — "Each player scries N", `Each` mints `ManyOf` where the anaphor wants a single `Player` | **Belongs elsewhere: `docs/tickets/planned/workbench-element-binder-reads.md`** — this is the per-member binder read the event-zone split minted that ticket for. Same shape as the `AggregateOver` per-member item already routed there. Routed. |
| *"The card's mana cost is unthreaded"* — `Card.text : AbilitySeq (costLetters cost)`, Prosperity | **Belongs elsewhere: a `Card.idr` face law**, not a mention change. The ADR already records it verbatim under the obligation clause ("A card's own mana cost is not yet threaded … [CR#107.3k] is the rule to read first"). Routed to a new ticket, `docs/tickets/planned/workbench-card-cost-letters.md`; no anaphora sub-round touches `Card.idr`. |
| *"the counted-group SIZE as a readable amount"* (Screeching Scorchbeast, The Wise Mothman, Bruvac) | **Probably delivered — probe, do not schedule.** `GroupSize` exists with a ProofsAnaphora entry and is written on the bench (§0 in each sub-ticket). The parent itself routed it conditionally ("if no other ticket owns it"). If the probe fails, it is a one-line addition to sub-round C, not an item of its own. |
| *"The 'as long as' CONDITION, 16 lines"* (Bonds of Faith and kin) | **Probably delivered** — `OnlyWhile` (§0) is the postposed static conditional, typed at `staticIntro se`, and its docstring names the ask. Probe Bonds of Faith; if green, mark delivered and drop the "argument-order question" acceptance bullet. |
| *"the marked-read row admits a marked read after a single non-repeating chooser"* (finding 1030, 12 carriers) | **Rides sub-round B**, per the parent: *"worth nothing on its own but free here."* Not its own item. |
| PLAGUE SPORES | **Fenced out, verbatim from the parent**: *"it needs two separate targets under one plural anaphor, which the double-target work owns"* — `workbench-conditional-and-coordination`'s live successor, `docs/tickets/planned/workbench-coordination-family.md`. |
| Blockers inside the nine until-condition cards — Tainted Pact / Helm of Obedience's *"whichever comes first"*; Timesifter's tie condition | **Fenced out.** The disjunctive termination is the routed OR cell (`docs/tickets/wip/workbench-event-disjunction-seat.md`); the tie condition is queued with extremal selection and is sub-round C's own `condIntro` consumer. |
| *"RETURN is not a `VerbName` row"* | **Not an item — a standing fence.** Carried into sub-round E's brief with the `VerbLabel`/`verbFacts` restatement (§0). Its four measured zeros stay recorded. |
| The parent's "Consumption boundary" section | **Stale in full** (§0). Each sub-ticket writes its own file list; this umbrella does not carry the old one. |

## Standing fences — preserved verbatim, do not re-propose

**RETURN fence** (from the split report, verbatim): *"RETURN is not a
`VerbName` row — do not re-propose it"* — carried as a fence (restated in
`verbFacts` terms per §0), including *"What IS open here is smaller: … If a
witness ever wants 'the returned card', it wants a STAMP WITHOUT A TAG —
decide that only against such a witness."* Measured: "return" is absent from
[CR#701]'s seventy keyword actions and the CR defines it nowhere; "would be
returned" is 0 supported lines, "was returned to" 0, "the returned card" 0 —
against mill's 2, 12 and 19. The five "is returned to [a] hand" triggers are
zone-change triggers ([CR#603.6]) keyed on the DESTINATION, and "returned this
way" (10) is the verb-general deictic idiom, attaching to "removed",
"prevented", "moved", "died" and "enchanted" too. Owned by sub-round E.

**PLAGUE SPORES fence** (from the split report, verbatim): PLAGUE SPORES is
the CR's own worked example for the family but is **not** any of the five
sub-rounds': "it needs two separate targets under one plural anaphor, which
the double-target work owns" — `workbench-conditional-and-coordination`'s live
successor, `docs/tickets/planned/workbench-coordination-family.md`. Sub-round
B's binding rulings explicitly restate this: PLAGUE SPORES is not sub-round
B's, even though it superficially resembles the verb-provenance family.

Standard constraints apply.
