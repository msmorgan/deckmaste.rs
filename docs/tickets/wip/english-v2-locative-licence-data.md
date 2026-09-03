---
needs: [english-v2-homograph-feature]
---
Remove the census-in-a-default from locative/temporal licensing
(preposition-class landing review H1/H2/H3/M3). COORDINATOR GO GRANTED
(user ruling 2026-09-02): this ticket lowers coverage by design (>=161
identities selected only through readings the attachment amendment
forbids); retirement goes through the authenticated `--bless --retire`
path with every retired identity named in the landing record's
obligation line. Retire only identities whose sole surviving reading is a
forbidden one — prove each by probe; a unit that still selects through a
legitimate reading is not retired.

- H1: `feature LocativeTemporalLicense = OfAndOnLicensed` is the default
  for 34/49 closed nouns and all open Type/Subtype nouns; "Draw a card of
  target player." and "Destroy target creature on an artifact." select.
  No locative/temporal default: every noun that licenses a preposition
  declares it (zone nouns, turn parts, the counter/marking `on` hosts);
  everything else licenses none. Move the `of` licence to the HEAD side:
  relational nouns via the existing, relayed-but-unread `Relationality`
  feature ("the top of", "a copy of", "controller of"); non-relational
  heads take no `of` postmodifier. Open declaration nouns (Type/Subtype/
  TurnPart) get their licence from declaration data, not the hardcoded
  table in emit/terminal.rs.
- H2: every Preposition member declares a complement licence (8 of 14 are
  UnrestrictedComplement); the complement sum must not admit a nested PP
  ("Draw a card for from your graveyard." rejects).
- H3: `existential_finite_clause`'s `domain: opt PrepositionalPhrase`
  requires the adjunct-capable attachment feature ("There is a creature
  into your graveyard." rejects).
- M3: implement the attachment rule as a derived decision — host-side
  licensing decides NP-postmodifier vs predicate adjunct ("Sacrifice a
  creature during your upkeep." attaches the PP to the predicate, not the
  NP); a genuine two-survivor case is a STOP. Explain the unique/
  specificity shift in the record.
- Class D (homograph review): the preposition machinery is bypassed by
  14 form-literal rows — `"of"` hardwired in 7 constructions
  (edge_of_phrase, positional_partitive, determinative_partitive,
  number_of_scalar_value, any_number_quantifying_determiner,
  contracted_copular_relative_reference, …), `"among"` in 3, `"under"` in
  enter_control/control_phrase, `"during"` in only_during_restriction,
  `"in"` in 1. Head-side licensing cannot govern literals, so these are IN
  SCOPE: route each through the general PP + licence, or declare it as a
  frame-selected preposition in valence data; none may stay a form
  literal. Any movement of `literal_lexicon_collisions` (72 today) is
  accounted row-by-row in the record, never bumped at coverage.rs:2493.
- Fixture hazard: synthetic test declarations may no longer use a surface
  owned by an unlicensed vocab member (same, declare, other, next,
  additional, first, second, main, base, maximum, …) — the environment
  hard-fails to load; re-spell fixtures with novel surfaces.
Probes: all review negatives reject; all amendment positives parse.
Standard constraints apply.
