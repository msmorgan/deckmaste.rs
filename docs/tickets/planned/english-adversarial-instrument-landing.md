---
needs: []
---
**`cargo xtask english adversarial` exists, is tested, and has nothing on
trunk pointing at it.** It lives in the sibling workspace
`english-adversarial-corpus`, a sixth sibling to
`recovery`/`roundtrip`/`shapes`/`lint`/`bracket` — and the first *active*
instrument: the other five read the printed snapshot; this one synthesizes
legal-but-unprinted text from verbatim fragments and checks structural laws
against it.

Modes: `--text` (probe one literal text — the CLI previously had no
free-text parse entry point at all), `--calibrate` (derive ceilings from the
printed 99.9th percentile), `--printed` (run the laws over printed faces as
a baseline), and the default synthesized sweep. 48 module tests, 118 in the
xtask crate.

**Why it is unintegrated, in one line:** the drain protocol — its contents
are the open failure list, so it lands only when it is clean.

## What deliberate work remains before it could land

Invariants 3 and 4 from the design — curated minimal-pair contrast sets and
coordination-monotonicity schemas — were deliberately deferred until a
census existed, because authoring minimal pairs before knowing the real
failure shapes is guessing. The census now exists.

## Limitations worth recording so nobody rediscovers them

- `(rule, depth)` does NOT identify a grammatical category. Composition
  therefore does not infer interchangeability; it verifies admissibility by
  requiring every input piece to parse clean standalone.
- The nested-pair filter that removes duplication artifacts can also
  suppress genuine agreement-class findings — one real case (`"Aang
  attack"`, a clean carrier producing an ungrammatical parse) was lost to
  it. The census under-reports that class.
- Bare keyword lines contribute no fragments at all — see
  `english-keyword-only-provenance-gap`.
- One proposed filter was rejected by measurement: adjacent-duplicate-word
  rejection, because 161 of 31,685 printed faces legitimately contain them
  (`"Cascade, cascade, cascade, cascade"`, `"discover, discover"`).
- `forest-growth` found nothing above the printed baseline: 3 findings
  across 2,549 compositions (0.118%) versus 31/31,685 printed (0.098%), and
  the ceilings are by definition the printed 99.9th percentile.
- `ability-independence` found zero findings across 58 compositions — a
  clean result for the parser, not an unrun check.
- No formal whole-branch code review has been run over the instrument's
  seven commits; each task was reviewed individually.

Standard constraints apply.
