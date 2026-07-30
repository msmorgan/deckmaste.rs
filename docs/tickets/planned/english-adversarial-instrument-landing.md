---
needs: []
---
**`cargo xtask english adversarial` exists, is tested, and has nothing on
trunk pointing at it.** It lives in the sibling workspace
`english-adversarial-corpus`, a sixth sibling to
`recovery`/`roundtrip`/`shapes`/`lint`/`bracket` — and the first *active*
instrument: the other five read the printed snapshot; this one synthesizes
candidate unprinted text from verbatim fragments and checks structural laws
against it.

Modes: `--text` (probe one literal text — the CLI previously had no
free-text parse entry point at all), `--calibrate` (derive ceilings from the
printed 99.9th percentile), `--printed` (run the laws over printed faces as
a baseline), and the default synthesized sweep. 48 module tests, 118 in the
xtask crate.

**Why it is unintegrated, in one line:** its composition admission and tie
classification are unsound, two designed invariants remain deferred, and its
contents are the open failure list, so it lands only after those are corrected
and the resulting list is clean.

## What deliberate work remains before it could land

Invariants 3 and 4 from the design — curated minimal-pair contrast sets and
coordination-monotonicity schemas — were deliberately deferred until a
census existed, because authoring minimal pairs before knowing the real
failure shapes is guessing. The census now exists.

Composition admission must also become category- and agreement-safe. The
current standalone `parses_clean` check admitted 32 recovered outputs, but a
complete audit found every one grammatically malformed: examples include
`Agent Frank Horrigan enters or attack`, `Foray of Orcs deals all to ...`,
`Then each who lost ...`, and a literal doubled comma. Clean parses of the
input fragments do not prove that a substitution or coordination extension
preserves their grammatical role.

## Limitations worth recording so nobody rediscovers them

- `(rule, depth)` does NOT identify a grammatical category, and requiring
  every input piece to parse clean standalone does not establish
  interchangeability. The landing work must use a stronger typed/schema
  admission rule rather than treating output recovery as a parser frontier.
- The nested-pair filter that removes duplication artifacts can also
  suppress genuine agreement-class findings — one real case (`"Aang
  attack"`, a clean carrier producing an ungrammatical parse) was lost to
  it. The census under-reports that class.
- Rule-indexed fragments intentionally exclude bare keyword lines: a bare
  keyword is one atomic catalog item with no chart derivation beneath its
  ability span. If the instrument needs keyword atoms, mine the typed keyword
  AST/catalog directly rather than manufacturing a grammar rule solely for
  provenance.
- The reported 93.07% printed "tie" baseline is invalid. `facts_for` records a
  tie whenever `tied_alternatives()` is nonempty, but that slice always includes
  the selected minimum-cost alternative. Actual equal-cost ties require
  `len() > 1`; fix the predicate and recalibrate every tie count and assertion.
- One proposed filter was rejected by measurement: adjacent-duplicate-word
  rejection, because 161 of 31,685 printed faces legitimately contain them
  (`"Cascade, cascade, cascade, cascade"`, `"discover, discover"`).
- `forest-growth` found nothing above the printed baseline: 3 findings
  across 2,549 compositions (0.118%) versus 31/31,685 printed (0.098%), and
  the ceilings are by definition the printed 99.9th percentile.
- `ability-independence` found zero findings across 58 compositions — a
  clean result for the parser, not an unrun check.
- No formal whole-branch code review has been run over the instrument's 15
  commits; each task was reviewed individually.

Standard constraints apply.
