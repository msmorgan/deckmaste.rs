---
needs: []
---
# A Gap passes through a child its Production does not mark transparent

## Withdrawn: investigated

The premise is false. `GerundSubject` declares `transparent: ["head"]`
(`crates/deckmaste_english_v4/grammar/english.ron:539-541`, as does
`GerundClauseSubject`). Its Clause's Gap comes up through that transparent
head, as the engine contract in `crates/deckmaste_syntax/README.md` allows.

Evidence from `cargo xtask english-v4 parse` on the two probes:

- *Choose an ability that entering causes to trigger.* The `GerundSubject`
  Clause *entering causes to trigger* has Gap NP. Its `head` is
  `Valence:VerbPhrase/ComplexCatenative(NP,ToInf)/gap0`, *causes to trigger*,
  with Gap NP: the Object of *cause*.
- *Choose an ability that entering causes a creature to trigger.* The Gap
  starts at `Valence:VerbPhrase/Monotransitive(NP)/gap0`, *trigger*. It rises
  through `ToInfinitival`, `InfinitivalComplement` and the head Verb Phrase to
  `GerundSubject`, and every link is a transparent `head`.

Extraction from the predicate of a gerund-subject Clause is ordinary English,
so both are endorsed analyses.

The engine contract holds. `take_gap` (`crates/deckmaste_syntax/src/step.rs`)
rejects a Closed child with any Gap. `finish` takes the mother's Gap only from
a transparent, raising or shared child, or from the Production's own
`Require("$.Gap", …)`. Binding builds an authored Production's Gap Rules from
its `transparent` list alone. Packing keys on a summary that includes the Gap.

The spurious Panharmonicon-family Readings (a bare relative *creature
[entering causes … to trigger]*) were the bare-relative defect, which
`lxzvuwqu` fixed by requiring `SubjectInitialClause`.

The test `closed_child_with_a_gap_yields_no_mother`
(`crates/deckmaste_syntax/tests/contract.rs`) pins the contract: the same
Production with its head left Closed builds no mother over a gapped head,
while the transparent head passes the Gap up.

Standard constraints apply.

Withdrawn: no grammar change; contract test added.
