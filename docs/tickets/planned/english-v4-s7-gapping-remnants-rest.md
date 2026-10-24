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

- **Hearth Elemental, Frantic Firebolt, Kosei, Penitent Warlord** (*cards
  ... that are instant cards, sorcery cards, and/or have an Adventure*; *As
  long as Kosei is enchanted, equipped, and has a counter on it*): a Verb
  Phrase coordination whose middle conjuncts are Predicatives with the
  copula elided, recovered from the initial conjunct (gapping, CGEL, Ch. 15
  §4.2). The remnant machinery is the likely route: a remnant of a verb
  whose frame's first Slot is a Predicative, accepted as a non-initial
  conjunct of a Verb Phrase coordination, its elided head matched against
  the initial conjunct's head (`Summary::shared_head`, `Summary::remnant`;
  the engine's `track_remnant` already checks this for any coordination).
  Verb Phrase coordinations have no conjunct Category today, so the remnant
  needs one or an engine variant of the Coordination schema. A coordination
  whose later conjuncts are all remnants (*is a creature, an artifact, and
  an enchantment*) is the coordination of their Predicatives, so it needs a
  structural canonicalization (the last conjunct is no remnant, or no
  conjunct after the first is) to keep one tree; that is a new entry in
  `english-lexical-analysis.md` §2 for the owner. Agreement of the remnant
  with the shared Subject is lost with its verb; check that no wrong form is
  admitted.
- **Ulasht, the Hate Seed** (*enters with A for each X and B for each Y*):
  Clusters of Adjuncts after an intransitive head, the second *with* elided
  (note 65's *Paris for a week and Bonn for three days*, a PP remnant).
  Clusters are built from a verb's Frame Slots, and the *with*-PP here is an
  Adjunct of *enter*, so a Cluster needs Adjunct sequences too. A PP remnant
  would be derived from the Preposition Valence and matched on its
  preposition. The *for each* phrases also conflict with the
  `high_attachment` class for distributive PPs.
- **Akroma, Vision of Ixidor; Selective Adaptation**: the idiomatic
  coordinate *and so on* (CGEL, Ch. 17 §7.7.3(e), p. 1540) with a
  *for*-phrase naming the rest (*and so on for double strike, ..., and
  partner*). Corpus: *and so on* is otherwise only in reminder text, which
  is removed. Selective Adaptation also needs its Object after an Adjunct
  (*Choose from among them a card with flying, ...*; see
  `english-v4-s7-heavy-object-shift`).
- **Dralnu's Pet** (*enters with flying and with X +1/+1 counters on it*):
  *with* + keyword phrase is a Nominal postmodifier only, and a coordination
  hosts only what every conjunct hosts. A complement frame for *enter*
  breaks Master Biomancer (*with ... and as a Mutant*); a Predicate host for
  *with* + keyword grows about 400 units. The em-dash composite cost
  *Kicker—{2}{B}, Discard a creature card.* is a second blocker.
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
- **Aang, Master of Elements** (*you gain 4 life, draw four cards, put four
  +1/+1 counters on him, and he deals 4 damage to each opponent*):
  coordination at unlike levels, Verb Phrases between Clauses, the medial
  coordinates with their Subject ellipted (CGEL, Ch. 15 §3.6, pp. 1335–1336,
  [31iii]). A conjunct Category over Clause and finite Verb Phrase with an
  `unlike` level feature, initial and last Clauses, would admit it, but a
  Clause carries no Subject agreement, so a medial Verb Phrase's agreement
  with the initial Subject must be secured first (*you gain ..., draws ...*
  must not read).

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles. A witness that
needs a policing flag is a STOP (§6.1, Stratum 7 Long-Tail Rule). Standard
constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with the two landing lines.
