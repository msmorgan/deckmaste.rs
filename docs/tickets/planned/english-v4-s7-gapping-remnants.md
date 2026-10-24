---
needs: []
---
# Stratum 7: gapping and ellipsis remnants

The faces `english-v4-s7-gapping-and-ellipsis` left unread. That slice
generalized Complement Clusters (right nonce-constituent coordination with an
Adjunct ending each Cluster, any number of Clusters, parallel by frame shape;
CGEL, Ch. 15 §4.3, pp. 1341–1343), added qualifying end-attachment (*but only
...*, *and only ...*; §4.5, p. 1347) and *X, but not Y* (§2.6, p. 1313). Each
face below needs a construction beyond those, or a ruling.

- **Hearth Elemental, Frantic Firebolt** (*cards ... that are instant cards,
  sorcery cards, and/or have an Adventure*): a Verb Phrase coordination with a
  gapped middle conjunct, the predicator *are* recovered from the first
  (CGEL, Ch. 15 §4.2). Admitting a gapped Verb Phrase conjunct also gives every
  serial Object list after a verb (*has flying, first strike, and vigilance*)
  a second, gapped analysis; that needs either a structural canonicalization
  (all non-initial conjuncts gapped = the Cluster or basic analysis), which is
  a new entry in `english-lexical-analysis.md` §2 for the owner, or another
  general restriction. Measure before and after.
- **Ulasht, the Hate Seed** (*enters with A for each X and B for each Y*):
  Second triage, alternative: *for each* inside each conjunct conflicts with the global `high_attachment` rule for distributive PPs.
  Clusters of Adjuncts after an intransitive head, with the second *with*
  elided (CGEL, Ch. 15 §4.3, p. 1343, note 65: *We'll be in Paris for a week
  and Bonn for three days*, a PP+PP coordination with ellipsis of *in*).
- **Dominaria's Judgment** (*gain protection from white if ..., from blue if
  ..., ...*): Clusters whose later Objects elide the keyword head *protection*
  and keep its parameter (the same note 65 analysis, the elided element a
  keyword head).
  Note: Complement Clusters (change `yltzvnpt`, recursive clusters that may end in an Adjunct) are sequences of a *verb's* Complements; today clusters share a verb, not a noun's complement frame. The likely route is to extend them so the clusters [from COLOR] [if you control a LAND] share the noun *protection* (or *gain protection*). Check whether a cluster can share a noun's frame before choosing. Dominaria's Judgment stays deferred and in scope; it is not declined.
- **Akroma, Vision of Ixidor; Selective Adaptation**: the idiomatic coordinate
  *and so on* (CGEL, Ch. 17 §7.7.3(e), p. 1540), here with a *for*-phrase
  naming the rest (*and so on for double strike, ..., and partner*); *so on*
  is a vocabulary gap. Selective Adaptation also needs its Object after an
  Adjunct (*Choose from among them a card with flying, ...*), which has no
  Reading today.
- **Dralnu's Pet** (*enters with flying and with X +1/+1 counters on it*):
  Second triage, additionally: the em-dash composite cost *Kicker—{2}{B}, Discard a creature card.*
  *with* + keyword phrase is a Nominal postmodifier only. A complement frame
  for *enter* breaks Master Biomancer (*with ... and as a Mutant*); a
  Predicate host for *with* + keyword grows about 400 units.
- **Invasion of Arcavios** (*search your library, graveyard, and/or outside
  the game*): an Object and a locative Adjunct coordinated. CGEL, Ch. 15 §3.1,
  p. 1323, [4i] rules out coordinating an Object with an Adjunct (*He left
  this morning and the room*); this face needs a ruling before any
  construction. Note: treat *outside the game* ([CR#400.11]) as a compound zone noun in the lexicon. Then *search your library, graveyard, and/or outside the game* is a plain noun-phrase coordination, and *from outside the game* is *from* + noun phrase. The compound must replace the current preposition analysis of *outside*, not add a second tree beside it.
- **Aang, Master of Elements** (*you gain 4 life, draw four cards, put four
  +1/+1 counters on him, and he deals 4 damage to each opponent*): coordination
  at unlike levels, two Verb Phrases between Clauses, the medial coordinates
  with their Subject ellipted (CGEL, Ch. 15 §3.6, pp. 1335–1336, [31iii]: *He
  reads widely, has a questioning mind and he's very mature for his years*).
  Not a mixed-mood serial: `english-v4-s7-serial-coordination-generalizations`
  keeps that one imperative-initial, so *draw four cards* is no imperative.
- **Frantic Firebolt** shares the *are instant cards, sorcery cards, and/or have an Adventure* analysis of Hearth Elemental above.
- **Kosei, Penitent Warlord** (*As long as Kosei is enchanted, equipped, and has a counter on it*): predicative conjuncts and a VP sharing the subject and copula.
- **Scorching Lava** (*that creature can't be regenerated this turn and if it would die this turn, exile it instead*): a declarative and an imperative with a fronted *if*, joined by *and* with no comma. The `english.ron` rule for Finite+Plain coordinations requires `delimited`.

Triage hypotheses for the faces added here (Kosei, Scorching Lava) and the existing ones; confirm each against the code and a parse before changing anything.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles. A witness that
needs a policing flag is a STOP (§6.1, Stratum 7 Long-Tail Rule). Standard
constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with the two landing lines.

## Review notes (hypotheses, verify first)

- Hearth Elemental, Frantic Firebolt and Kosei share one shape: copular *be* + two comma-separated Predicatives + *, and/or* + a lexical VP. Restrict to copular *be* + Predicatives, so object lists (*has flying, first strike, and vigilance*) never gain a gapped parse. One construction reads all three.
- Invasion of Arcavios: the *outside the game* compound noun is right.
- Dominaria's Judgment: extend clusters under *gain protection* narrowly.
- Dralnu's Pet: coordinate *with* + KeywordPhrase with *with* + NP only when explicitly coordinated, never as a general Predicate host (+400 units otherwise).
