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
- **Ulasht, the Hate Seed**: landed in
  `english-v4-s7-gapping-remnants-rest-2`. Adjunct Clusters: after a verb
  whose frame has no Slot, Clusters of two Preposition Phrase Adjuncts
  (`ClustersDecl::adjunct_clusters`), a later one opening with a remnant, a
  Preposition Phrase's Complement with its preposition elided
  (`PrepositionRemnant`, `Elided::PrepositionFrame`; CGEL, Ch. 15 §4.3,
  p. 1343, note 65: "a PP+PP-coordination, with ellipsis of *in*"). The
  structural restriction is CGEL's parallelism requirement (p. 1341): every
  Cluster has the initial one's prepositions, the first overt or elided
  (Coordination `parallel`). Measured: Ulasht and Semester's End (*enters
  with an additional +1/+1 counter on it if it's a creature and an
  additional loyalty counter on it if it's a planeswalker*) gain their
  endorsed analysis and no other unit changes; without the parallelism, 10
  more Readings on 3 units, each a non-parallel Cluster.
- **Akroma, Vision of Ixidor; Selective Adaptation**: landed in
  `english-v4-s7-gapping-remnants-rest-2`. *So on* is an adverb whose
  position is `Coordinate` (`adverbs.ron`), and an Idiomatic Coordinate is
  that adverb alone or with a *for*-phrase (*so on for double strike, ...*).
  The grammar's `idiomatic_coordinates` let it be the last coordinate after
  *and* of a coordination of any Category, derived once per Category on its
  first schema that admits *and* (CGEL, Ch. 17 §7.7.3(e), p. 1540). It is no
  conjunct: the overt conjunct before it is the last, and it is outside the
  head line, so Cluster parallelism asks nothing of it. Selective
  Adaptation's Object after the Adjunct reads through the heavy-object shift
  (`spkrozto`).
- **Dralnu's Pet**: landed in `english-v4-s7-gapping-remnants-rest-1`
  (the Kicker line already reads). *with* + keyword phrase is a Nominal's
  Modifier, and a Predicate's Adjunct only as a conjunct of a coordination
  of *with*-phrases beside one that is a Predicate's on its own
  (`Hosts::NominalAndConjoinedPredicate`), never alone, so the 400-unit
  growth of a general Predicate host does not occur. Thirteen kicked
  *enters with ... counters on it and with trample* faces (Kavu Titan, the
  volvers) gain their correct analysis beside the earlier ones.
- **Invasion of Arcavios; Turtles Forever**: landed in
  `english-v4-s7-gapping-remnants-rest-2` on the owner's ruling: *outside
  the game* is a compound noun after *from* and as a conjunct beside zone
  nouns, and elsewhere *outside* is the preposition ([CR#400.11]). The noun
  is count and mass (`nouns.ron`), so it coordinates under the zone nouns'
  determiner (*your [library, graveyard, and/or outside the game]*), and
  *from* no longer takes a *PP[outside]* Complement. A multiword noun whose
  first word is also a preposition is no lexical barrier
  (`english-lexical-analysis.md` §2), so *cards you own outside the game*
  keeps the Preposition Phrase. The noun also reads as a bare mass
  Predicative Complement beside the locative Preposition Phrase (*if it's
  outside the game*), which no corpus unit has. Turtles Forever read before
  only through a wrong analysis (*outside the game* fronted in a second
  Clause) and now has the Object coordination.
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

1. **Census delta** (`swknnvqx`): faces read 32,820 → 32,824 (+4: Akroma, Vision of Ixidor; Invasion of Arcavios; Selective Adaptation; Ulasht, the Hate Seed); total Readings 338,319 → 338,336; lost faces: none. Pass 1 (`nxxvoyzw`): 32,806 → 32,811, Readings 338,087 → 338,162.
2. **Judgment set**: `cargo test -p deckmaste_english_v4` passes; the entries in `judgments/idiomatic_coordinates.txt`, `judgments/outside_the_game.txt`, `judgments/complement_cluster_adjuncts.txt` and `judgments/declaration_batch_b.txt` hold.
