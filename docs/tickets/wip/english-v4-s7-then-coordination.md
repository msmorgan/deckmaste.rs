---
needs: [english-v4-s7-then-initial]
---
# Stratum 7: *Then* in serial and coordinated sequences

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read *X, then Y*, *X and then Y*, *may then* and serial *A, B, then C* across Clauses and Verb Phrases that share a Subject or modal. The conjuncts may be imperative or declarative, including mixed moods. *Then* may scope over a coordinated last conjunct (*then shuffle and put that card on top*), and a sentence may hold several *then*s (Pox, Living Death).

Analysis: Clause conjuncts, with *then* as an Adjunct of the last conjunct (the pending judgments state it). That makes this a coordination whose final link is marked by a connective rather than a Coordinator. A `Coordination` declaration today always names a coordinator class, so the engine probably needs a coordinator-less, marked final link that reuses the Coordination and Right Node Raising machinery. Do not make *then* a Coordinator: that contradicts the judgments.

Both the grouping *A, [B, then C]* and the flat *[A, B, then C]* are grammatical, as are *then* as an Adjunct and the comma scope of a supplemented first conjunct (*..., where X is ..., then ...*). Admit each that English admits; suppress none (`docs/decisions/english-v4.md` §4.4, principle 1). Where two analyses of one sentence differ in their count of Readings, measure and report it before landing; do not suppress to equalize.

The face Mishra, Claimed by Gix also needs `english-v4-s7-preposed-clause-coordination`.

## Grounding

- CGEL, Ch. 15 §2.10, pp. 1319–1320 [78]–[80], on *so* and *yet* as sole linking items.
- CGEL, Ch. 8 §19, p. 778.
- `docs/oracle-style-guide.md` §3, "Commas and the serial comma" (before *then* within a sequence); §10, "Sequence".

## Witness cards

- Blur
- Obsessive Stitcher
- Beneath the Sands (serial)
- Vampiric Tutor (*then shuffle and put that card on top*)
- Pox (several *then*s)

## Estimated faces

About 2050 faces (an estimate from the 2026-10-23 census, not a measured figure).
67 more faces also need `english-v4-s7-quoted-abilities`; 14 also need `english-v4-s7-stranded-auxiliaries`.

## Engine

Likely. See Engine below.

## Pending judgments this slice makes hold

`then_sequencing.txt:3,6–14`; `predeterminer_all.txt:7–9,21`; `at_random_before_source.txt:6,12`; `up_to_that_many.txt:5`; `ring_tempts_you.txt:4`; `all_but_exception.txt:15`; `anchored_abilities.txt:5`; `attributive_scalar_comparative.txt:13`; `card_noun_premodifier.txt:11`; `mana_symbol_passive_subject.txt:6`; `pronominal_other.txt:7`; `trailing_adjunct_height.txt:29`; `where_variable_clause.txt:13`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

*X, then Y* is a Linked Coordination: an engine Coordination with no
Coordinator (`link:`), whose Linked Conjunct (LinkedClause, LinkedVerbPhrase)
is marked by its initial connective, an Adjunct in the new `Linking` position
(CGEL, Ch. 15 §2.10, pp. 1319–1320), set off by the comma before it. Clauses
of like or mixed moods and Verb Phrases sharing a Subject or modal link,
binary and serial; a chain has one tree, its serial (`Summary::link`); *then*
may scope over a coordination by a Coordinator (Vampiric Tutor also keeps
*[A, then B] and C*); and a Clause Complement of a preposition is never a
Linked Coordination (`unlinked`). *Then* is also central (*may then*). An
adverb before a bare imperative is its Clause's front Adjunct only
(`CentralPosition`; *secretly* and *simultaneously* gain a front position),
which removes the *also* duplicate. `then_sequencing.txt`'s *Then draw a card
and discard a card* now has two Readings, the scopes the instead-tail change
admits for a comma-less preposed Adjunct. `card_noun_premodifier.txt:11` still
fails on *[card or Gate] card*, which its stratum-6 sentence without *then*
admits too.

1. **Census delta** (`xonulypx` → `nsmwosmr`, full corpus): faces read 30,426 → 32,574; total Readings 280,596 → 315,929; Surplus 117,211 → 132,421; 2,212 units changed (2,207 newly read, 1 grown, 4 shrunk). Lost faces: none.
2. **Judgment set** (`nsmwosmr`): 3,103 enforced, 272 pending; all hold.
