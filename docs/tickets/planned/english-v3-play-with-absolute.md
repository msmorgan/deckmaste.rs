---
needs: []
---
# Read intransitive *play* with a *with* + subject–predicate Adjunct: Players play with their hands revealed.

## Why

Blocker report at trunk `uoqyuxtnowpo`: key `at:play with` 23 sole-blocker
faces. `cargo xtask english-v3 grep -i '\bplays? with\b'` finds **26** faces;
25 are unread (the covered one is the card name *Play with Fire*), 25 failing
lines match and **all 25** are the only failing line of their face: *Play
with the top card of your library revealed.* 12, *Players / Your opponents
play with their hands revealed.* 5, *Players play with the top card of their
libraries revealed.* 3, others 5.

Two things are missing, and only the first shows in the probes:

1. *play* has no intransitive frame: "Players play." 0 (`keyword_actions/
   play.ron` declares `frame_set: Transitive`).
2. The *with* phrase is a subject + predicate clause (*their hands* +
   *revealed*), and the grammar has no such Complement of *with*. What it has
   instead reads a neighbouring string by another analysis: "Players attack
   with their hands revealed." 1 and "Players draw cards with their hands
   revealed." 3, every Reading taking *their hands revealed* as an NP whose
   head *hands* has a participial postmodifier (`ParticipialPostmodifiedNominal`).

Adding only the frame would cover the 25 faces through that NP analysis. It
says "with those hands that are revealed", not "with their hands in the
revealed state": the wrong reading for these cards.

## Goal

1. *play* reads with no Object: *Players play …*, imperative *Play …*, *that
   player plays …*, bare *play* under causative *have*.
2. *with* takes a clause Complement with an overt Subject and a predicative
   past participle or Adjective: *with their hands revealed*, *with the top
   card of your library revealed*, *with that card revealed in their hand*.
   The whole PP is an Adjunct of the *play* predicate.

## Analysis

CGEL Ch. 14 §10(a), p. 1267, "Complement to *with* and *without*": "*They
were standing against the wall* [*with their hands above their heads*]"; "The
underlined clauses have subject + predicate structure, but with no verb in
the predicate. *With* and *without* do not license finite complements, but
non-finites are found in addition to the verbless forms (see §8.3 above)".
That backs *with* + a subject–predicate clause, verbless or non-finite.
Treating *revealed* as the predicate of that clause, and everything about
licences, is a project ruling. Glossary: **Verbless Clause**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`VerblessClause` (`[predicate: lexical(Adjective)]`, `predicate.
VerblessPredicateUse = Yes`: subjectless, for *if able*) and
`VerblessClauseComplementPreposition` (`head.VerblessClauseComplement = Yes`,
declared today by `vocab:Preposition/If` in `lexicon/core.ron`), which
already cites p. 1267. Owner of the verb:
`plugins_v2/builtin/macros/keyword_actions/play.ron` (`grammar: Verb(bare:
"play", frame_set: Transitive)`).

Orchestrator rulings 2026-10-10:

- PW1. Data: give *play* an intransitive frame through its owning plugin
  definition (`frame_set: Custom` with the empty and the Object frame). Not
  in core, not a second verb.
- PW2. Refine `VerblessClause` / `VerblessClauseComplementPreposition`: a
  form with an overt Subject NP followed by its predicate (past-participial
  or adjectival). No new Preposition Construction beside it unless the
  existing one cannot carry a second form; say so in the landing record if
  one is added.
- PW3. The licence is declared on the Preposition (which clause Complement it
  takes: subjectless reducible for *if*, subject + predicate for *with*) and
  read by the Construction. No test on *with*, *play*, *revealed* or a card.
  If the existing two-valued `VerblessClauseComplement` cannot express the
  difference, extending that feature is in scope.
- PW4. Both analyses of *with their hands revealed* are grammatical strings
  of English and the NP-postmodifier one already exists for other verbs. Do
  not suppress it with a guard. A witness counts as gained only when the
  subject–predicate analysis is among its Readings; the expected count per
  witness is therefore 2 (clause Complement, NP with postmodifier), and the
  landing record names both shapes. A face covered by the NP analysis alone
  is recorded as covered by a wrong analysis, not as a gain.
- PW5. One analysis per string otherwise. The twins to avoid: (a) *You may
  play that card.*, *Play lands.* keep exactly their Readings: the
  intransitive frame adds none where an Object is present, and *with* +
  clause does not attach inside an Object NP; (b) *enters with a +1/+1
  counter on it* keeps exactly 2: note that *a +1/+1 counter on it* is itself
  a subject + PP predicate candidate (CGEL's *with his hands in his
  pockets*), so a new Reading there is the predictable increase: report the
  count and shape on covered faces, and STOP if it exceeds one extra Reading
  per *with* phrase; (c) the card name *Play with Fire* keeps its name
  Reading only.
- PW6. Newly covered faces are gains (subject to PW4). Increases on
  previously covered faces are possible only in the PW5(b) shape (*with* + NP
  + PP or participle after a verb); report them with counts before landing.
  If the count is large, restricting the new form to a participial or
  adjectival predicate (no PP predicate) is the authorised narrowing, since
  only that is attested as an absolute here.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0.

- Telepathy: "Your opponents play with their hands revealed."
- Enduring Renewal: "Play with your hand revealed."
- Field of Dreams: "Players play with the top card of their libraries
  revealed."
- Future Sight: "Play with the top card of your library revealed."
- Sen Triplets: "… This turn, that player can't cast spells or activate
  abilities and plays with their hand revealed. …" (whole line)

Sentence probes, 0 today: "Players play."; "Players play with their hands.";
"Play with the top card of your library." Controls that must not change:
"You may play that card." 1; "Play lands." 1; "Players can't play lands." 1;
"Players attack." 1; "You may look at that card with it revealed." 1.
Controls expected to change only by PW5(b)/PW6: "Players attack with their
hands revealed." 1; "It enters with a +1/+1 counter on it." 2.

## Method

Standard constraints apply. Plugin `grammar:` data loads at runtime: PW1
needs no rebuild. Bounded census, never `--all`: every supported face
matching `\bplays?\b|\bwith\b[^.]* revealed\b` (475 faces) plus, for PW5(b),
a 300-face random sample of covered faces matching `\bwith (a|an|the|its|
their|your|X)\b`, plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PW4–PW6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3`,
`deckmaste_lexical_source` and any crate reading `play.ron`; `cargo xtask
lean check` because a plugin definition changes. Once, at the end: a rebuild
after a grammar edit takes about 3 minutes and the test binary about 7. Tests
assert the role (the PP is an Adjunct of intransitive *play*; its Complement
is a clause with Subject and predicate; the PW5 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- Temporal Aperture (*for as long as that card remains on top of your
  library, play with … revealed and you may play that card …*) and Stromgald
  Spy (*have defending player play with their hand revealed for as long as
  …*): count them only if the whole face reads.
- *with* + gerund-participial clause; *without* + clause.
- *play* + *as though* / *any time* Adjuncts.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
