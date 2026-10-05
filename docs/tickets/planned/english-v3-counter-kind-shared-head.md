---
needs: []
---
**Evidence for whoever owns the English grammar: counter kinds share one
"counter" head by disjunction in real Oracle text.** A note from the
semantics_v2 design session of 2026-10-05, not a prescription. The English
grammar is owned by another line of work; decide whether these eight to eleven
texts should read. Standard constraints apply.

## What is attested (Vintage-legal, non-funny text)

Disjunction under one head, 8 cards:

- Me, the Immortal: "put your choice of a +1/+1, first strike, vigilance, or
  menace counter on Me".
- Assaultron Dominator: "your choice of a +1/+1, first strike, or trample
  counter".
- T-45 Power Armor: "your choice of a menace, trample, or lifelink counter".
- Owen Grady, Raptor Trainer: "your choice of a reach, menace, trample, or
  haste counter".
- Reluctant Role Model: "put a flying, lifelink, or +1/+1 counter on it".
- The Night of the Doctor: "your choice of a first strike, vigilance, or
  lifelink counter".
- Denry Klin, Editor in Chief: "your choice of a +1/+1, first strike, or
  vigilance counter".
- Frankenstein's Monster: "a +2/+0, +1/+1, or +0/+2 counter".

Bare kinds after "from among", 3 cards:

- Aragorn, Company Leader: "your choice of a counter from among first strike,
  vigilance, deathtouch, and lifelink".
- Elspeth Resplendent: "a counter from among flying, first strike, lifelink, or
  vigilance".
- Crystalline Giant: "from among flying, first strike, deathtouch, hexproof,
  lifelink, menace, reach, trample, vigilance, and +1/+1", a list that ends in
  a bare "+1/+1".

## What is not attested

Coordination by "and" under one head: 0 cards. "charge and loyalty counters"
is never written, and `crates/deckmaste_english_v3/tests/counter_compounds.rs`
already rejects it (the negative shared-modifier probe). The owner, 2026-10-05:
"counter kinds can be coordinated via disjunction (and plausibly, conjunction,
although unattested)".

## Why the lists may not read today

- Every counter declaration yields one compound-noun lexeme "<stem> counter"
  (`plugins_v2/builtin/macros/meta/CounterKind.ron` `compound_noun`;
  `crates/deckmaste_lexical_source/src/compound.rs`). A shared head needs the
  stem to stand alone as a modifier.
- `+1/+1` has no standalone stem lexeme: `p1p1Counter` and `m1m1Counter`
  declare no `grammar:`, unlike keyword kinds (`flyingCounter` has
  `grammar: FixedTerm(surface: "flying")`). So a list containing "+1/+1" or
  another numeric kind before a shared "counter", or a bare "+1/+1" after
  "from among", may be unread.
- `docs/decisions/english-coordination-is-structural.md` ~L111-113 already
  names "first strike, vigilance, or lifelink counter" as a common-head
  coordination.

## Related

`english-v3-systemic-residuals` (planned) records that whole counter NPs
coordinate and that the modifier-sharing charge-and-loyalty-counters analysis
rejects; it does not discuss disjunctive shared heads. This note is kept
separate so that ticket's owner can fold it in or not.
