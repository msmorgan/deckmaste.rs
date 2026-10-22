---
needs: []
---
# Coordinated bare relatives: a SubjectInitialClause coordination (R3, C-a)

Retired (no real witness; owner round 9).

From `english-v4-child-name-control-and-category-fission-audit`, Finding C.
The owner ruled R3 in round 9 (C-a only: add a SubjectInitialClause
coordination, and cross-reference-comment the duplicated Productions), and
instructed the implementer to search the corpus for a real witness before
building. There is none, so the coordination is not built.

## Evidence for the restatement

SubjectInitialClause (`english.ron`) keeps a preposed dependent from coming
before a bare relative's Subject: "Bare relatives always have the subject in
initial position" (CGEL, Ch. 12 §3.5.5, p. 1055). To do so, it restates four
Clause Productions:
- RelativeSubjectPredicate restates SubjectPredicate;
- RelativeCliticSubjectPredicate restates CliticSubjectPredicate;
- RelativeFloatedEach restates FloatedEach;
- RelativeFloatedAll restates FloatedAll.

Each differs only in `$.ClauseMarking`. RelativeAdjunctGap is a fifth
SubjectInitialClause Production, restating AdjunctGap.

The Category also shuts out coordinated bare relatives: *each creature you
control and an opponent owns* has 0 Readings, against 1 with *that*. Probe P3
added `Coordination(category: "SubjectInitialClause", …)`. No face changed.
Six units gained Readings, each wrong, for example Orzhov Charm's *you control
and all Auras you control attached to it*.

## The witness search (the owner's instruction: search before building)

The target is a real card with two bare relatives coordinated under one head
(*X you control and an opponent owns*, *X you cast or an opponent controls*).
Searched, all with no witness:

- `data/scryfall/oracle-cards.jsonl` (every face's `oracle_text`, 38,705
  cards) and `data/derived/cards.jsonl` (the census units' source text, 41,924
  card faces).
- Search 1: any `SUBJECT … and SUBJECT …` pair, where SUBJECT is *you*, *an
  opponent*, *that player*, *they*, *its controller*, *its owner*, *your
  opponents*, *a player*, *each player*, *each opponent*, *he/she/it*,
  *defending player* or a capitalized name, each followed by up to three
  words and a verb, the second verb followed by punctuation or a
  preposition or conjunction (so its object is a Gap).
- Search 2: the same pairs with *or*.
- Search 3: any verb that heads a bare relative (*control(s)*, *own(s)*,
  *cast*, *chose*, *have/has*, *played*, *sacrificed*, *targeted*, *named*,
  *dealt*, *put*, *revealed*, *exiled*, *activated*, *attached*, …) followed
  directly by *and* or *or* and a second Subject.
- Every hit was read. All are coordinated finite Clauses, not coordinated
  bare relatives: *you draw a card and you lose 1 life*; *target creature
  an opponent controls and you gain 4 life* (Snapdax, Vengeant Vampire);
  *all lands target player controls and that player loses all unspent mana*
  (Mana Short); *each creature you control and they gain vigilance*
  (Overseer of Vault 76); *a creature you control and it gains flying*
  (My Followers Ascend); *a creature you control and an opponent* in Risky
  Move is two coordinated Noun Phrases, not two relatives. In each, the
  second Subject begins a new Clause, so the Gap-bearing first conjunct is
  a bare relative alone, and the coordination sits above the Clause.
- Pairs of Subjects with a shared relative head, such as *If an opponent
  controls a Swamp and you control a Plains*, coordinate two Conditions'
  Clauses, not two relatives.

The only form the owner's example *each creature you control and an opponent
owns* describes does not occur on any card. The probe's six extra Readings
were the only effect of the coordination.

## Change landed

Only the cross-reference comments: each restated pair points at the other and
says a change to one must be made to the other (SubjectPredicate and
RelativeSubjectPredicate, CliticSubjectPredicate and
RelativeCliticSubjectPredicate, FloatedEach and RelativeFloatedEach,
FloatedAll and RelativeFloatedAll, AdjunctGap and RelativeAdjunctGap). The
grammar's meaning is unchanged.

## Reopen if

A real card whose Oracle text coordinates two bare relatives under one head is
printed. Then add `Coordination(category: "SubjectInitialClause", …)` with the
coordinators and shared features its conjuncts need, stated generally (§4.4,
item 3), a `+` judgment on that card's text asserting the coordinated relative
and each conjunct's Gap, and adjudicate any Readings it adds (a wrong but
well-formed English Reading stays, §4.4, item 6).

Standard constraints apply.

Census delta (measured on `oktotoom` → this landing): comments only; 237,320 Readings, Surplus 101,963 and 28,800 faces read, before and after; lost faces: none.
Judgment set: `cargo test -p deckmaste_english_v4` passes (judgments 6 of 6); no judgment added, since there is no witness card.
