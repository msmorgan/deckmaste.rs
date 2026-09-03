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
Probes: all review negatives reject; all amendment positives parse.
Standard constraints apply.
