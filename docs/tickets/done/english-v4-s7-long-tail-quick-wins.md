---
needs: []
---
# Stratum 7: long-tail quick wins

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Four small, general constructions, grammar first. Each is a hypothesis about a missing licence rather than a missing word.

- **Different N than NP** (Light-Paws, Emperor's Voice; Tibalt's Trickery): *a different name than …*. Hypothesis: `ComparativeIndirectComplement` requires Grade Comparative, and *different* is Plain. CGEL treats *different* with a *than* complement as a comparative-taking adjective (Ch. 6 §2.1; Ch. 13 comparison).
- **InterleavedManner** (Raiding Party): *weren't chosen [this way] [by any player]*. Hypothesis: a manner NP interleaves before the internalised *by*-agent of a passive. Proposed production: InterleavedManner. This is wide, so measure it carefully before and after; expect many already-read units to gain Readings, and restrict it if it does.
- **First-conjunct existential agreement** (Flow State): *If there is an instant card and a sorcery card in your graveyard*. Hypothesis: the existential verb agrees with the first conjunct only (CGEL Ch. 16 §6.2.2, p. 1395).
- **Differently named** (Ellywick Tumblestrum): *for each differently named dungeon*. Hypothesis: `AdverbModifiedAdjectivePhrase` requires Predicative, so a premodifying *differently* + participle is blocked.

## Witness cards

- Light-Paws, Emperor's Voice
- Tibalt's Trickery
- Raiding Party
- Flow State
- Ellywick Tumblestrum

## Estimated faces

5 faces.

## Hypotheses, not findings

Every diagnosis above comes from a triage of the unread faces and is a hypothesis. Before changing anything, confirm it against the code and a parse of the card, and drop or replace any diagnosis that does not hold. Measure each construction before and after (§4.4, principle 5), and add a positive judgment per witness asserting its intended analysis (`crates/deckmaste_english_v4/judgments/`).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

Census delta (measured on `nwonryvw` → `oslntzlv`): faces read 32,764 → 32,769 (Light-Paws, Emperor's Voice; Tibalt's Trickery; Flow State; Raiding Party; Ellywick Tumblestrum: 0 → 750, 340, 169, 4 and 2 Readings); the only other unit that changed is Triple Triad (22 → 28, *each other card … than it* through *other*'s Type I attributive Complement); total Readings 343,077 → 344,348 and Surplus 142,141 → 142,949. Lost faces: none.
Judgment set (`oslntzlv`): 3,215 enforced and 317 pending, all enforced holding; the 11 new entries (Stratum 7 witnesses pending) all hold.
