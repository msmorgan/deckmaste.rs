---
needs: []
---
# Stratum 7: the gapping remnants still unread

The faces `english-v4-s7-gapping-remnants` left unread. That ticket added
Complement Cluster remnants: a Cluster after the first may fill its first
Slot with a phrase whose head is elided, the head of the initial Cluster's
first Slot (`ClustersDecl::remnants`, `Parameters` `remnant`; CGEL, Ch. 15
§4.3, p. 1343, note 65), which reads Dominaria's Judgment. It also admitted
an undelimited declarative + imperative whose imperative carries a fronted
dependent (Scorching Lava). Each face below needs a construction beyond
those, or a ruling. The hypotheses are unverified unless marked.

- **Hearth Elemental, Frantic Firebolt, Kosei, Penitent Warlord**: landed
  in `english-v4-s7-gapping-remnants-rest-1`. They are no gapping but a
  coordination at the level of the final conjunct, the copula elided at the
  beginning of the medial ones (CGEL, Ch. 15 §3.6, pp. 1335–1336, [31ii]):
  a Valence schema derives a copular phrase's Predicative Complement
  without its verb (`PredicativeRemnant`, `remnants`), a medial conjunct of
  the Verb Phrase coordination (`Coordination` `remnant`), its elided head
  any verb declaring the frame of the initial conjunct's head
  (`Elided::Frame`). A remnant is never the initial or last conjunct, the
  structural canonicalization *medial remnants* of
  `english-lexical-analysis.md` §2, so *is a creature, an artifact, and an
  enchantment* keeps its one tree. The remnant has no verb and no
  agreement; the overt conjuncts agree with the Subject as before.
- **Ulasht, the Hate Seed** (*enters with A for each X and B for each Y*):
  Clusters of Adjuncts after an intransitive head, the second *with* elided
  (note 65's *Paris for a week and Bonn for three days*, a PP remnant).
  Clusters are built from a verb's Frame Slots, and the *with*-PP here is an
  Adjunct of *enter*, so a Cluster needs Adjunct sequences too. A PP remnant
  would be derived from the Preposition Valence and matched on its
  preposition. The *for each* phrases also conflict with the
  `high_attachment` class for distributive PPs. Still open after
  `english-v4-s7-gapping-remnants-rest-1`: Adjunct Clusters are a new
  capability beside every *verb PP PP and PP PP* in the corpus, and CGEL
  is silent on Clusters of Adjuncts alone (note 65 is the nearest).
- **Akroma, Vision of Ixidor; Selective Adaptation**: the idiomatic
  coordinate *and so on* (CGEL, Ch. 17 §7.7.3(e), p. 1540) with a
  *for*-phrase naming the rest (*and so on for double strike, ..., and
  partner*). Corpus: *and so on* is otherwise only in reminder text, which
  is removed. Selective Adaptation also needs its Object after an Adjunct
  (*Choose from among them a card with flying, ...*; see
  `english-v4-s7-heavy-object-shift`). Still open after
  `english-v4-s7-gapping-remnants-rest-1`: *so on* is a vocabulary gap, and
  CGEL gives the idiom no category or function, only the gloss "and more /
  other things of this kind" (p. 1540), so the final coordinate stands for
  conjuncts of any Category and needs an exemption from Cluster frame-shape
  parallelism; only Akroma could read before the heavy-object shift lands.
- **Dralnu's Pet**: landed in `english-v4-s7-gapping-remnants-rest-1`
  (the Kicker line already reads). *with* + keyword phrase is a Nominal's
  Modifier, and a Predicate's Adjunct only as a conjunct of a coordination
  of *with*-phrases beside one that is a Predicate's on its own
  (`Hosts::NominalAndConjoinedPredicate`), never alone, so the 400-unit
  growth of a general Predicate host does not occur. Thirteen kicked
  *enters with ... counters on it and with trample* faces (Kavu Titan, the
  volvers) gain their correct analysis beside the earlier ones.
- **Invasion of Arcavios** (*search your library, graveyard, and/or outside
  the game*): an Object and a locative PP coordinated. CGEL, Ch. 15 §3.1,
  p. 1323, [4i] rules out coordinating an Object with an Adjunct, so this
  needs a ruling. The earlier note (treat *outside the game* as a compound
  zone noun replacing the preposition analysis) does not survive the corpus:
  *outside* is a preposition in every other use (*a person outside the
  game*, *cards you own outside the game*, *if it's outside the game*), so a
  noun cannot replace it without losing those analyses, and beside it would
  give *from outside the game* a second tree. Turtles Forever (*Search your
  library and/or outside the game for ...*) has the same shape.
- **Aang, Master of Elements**: landed in
  `english-v4-s7-gapping-remnants-rest-1`. A Clause carries its Subject's
  Number and Person, unshared among coordinated Clauses; a finite Verb
  Phrase is a Subject remnant (`SubjectRemnant`), a medial conjunct of a
  Clause coordination (CGEL, Ch. 15 §3.6, pp. 1335–1336, [31iii]) that
  shares Number and Person with the initial Clause through the serial tail
  (`Coordination` `recovered`), so *you gain ..., draws ...* does not read.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles. A witness that
needs a policing flag is a STOP (§6.1, Stratum 7 Long-Tail Rule). Standard
constraints apply.

## Landing note

1. **Census delta** (`nxxvoyzw`): faces read 32,806 → 32,811 (+5: Aang, Master of Elements; Dralnu's Pet; Frantic Firebolt; Hearth Elemental; Kosei, Penitent Warlord); total Readings 338,087 → 338,162; lost faces: none.
2. **Judgment set**: `cargo test -p deckmaste_english_v4` passes; the entries in `judgments/medial_remnants.txt` and `judgments/preposition_function_licences.txt` hold.
