---
needs: [english-semantic-ir]
design: true
---
**[design] Differential parity harness: route the legacy-graduated corpus
through `english` → IR → RON and diff against the encodings the regex pipeline
already produced.** The project's first *oracle-backed* measurement. Every
existing instrument is absolute — faces parsed, spans recovered, rules cited,
cards graduated. This one compares against ~7,353 encodings that already pass
the fidelity gate and already run in the engine, so a disagreement is a
finding rather than a worklist row.

Claimable alongside the live `english-semantic-ir` work rather than after it,
once IR→RON lowering exists at all, even partially. The `needs:` edge is graph correctness, not a
reason to wait — the harness is worth most *while* the IR is being shaped,
least once it is finished.

## Why it is the highest-value instrument available

- **It measures the number that governs planning.** Structural-parse →
  executable-card conversion is unmeasured. ~90% of faces parse; what fraction
  of a parsed tree can be lowered to a running card is unknown, and nothing
  else in the tree answers it.
- **It is the retirement gate.** `deckmaste_migrations` cannot be deleted until
  its successor reproduces its output. Building the gate first makes the
  migration measured from the start instead of a leap.
- **It specs the IR against reality.** The failure mode for a new semantic layer
  is being designed against imagined card text. Real cards with known-good
  target encodings are a better spec than any synthesized set — the discipline
  that put Humility in canon, one layer up.
- **It is cheap.** It needs only the easiest quarter of the corpus, which is by
  construction what the IR can reach first.

## The design hazard, named first

**A byte-diff of RON will report a rate that cannot move.** The same meaning has
many spellings — macro vs expansion, argument order, sugar nodes — so naive
equality drowns real findings in benign difference. This is the `no_tie` shape:
an instrument that fires almost always teaches nothing. **Define the equivalence
relation before reporting any rate.**

Candidates, in preference order:

1. **Expanded-form comparison.** `cargo xtask card` already prints a card with
   its macros expanded; the `macro_ron` expander is the existing normalizer.
   Compare expanded forms, not source. Cheapest, reuses built machinery.
2. **Behavioral equivalence.** Load both encodings and compare engine-visible
   behavior. Strongest, slowest, and needs a stimulus set — treat as the
   adjudicator for cases (1) flags, not the primary sweep.
3. Byte equality — baseline only, expected to be noisy; never the reported
   figure.

Disagreements have three outcomes and the harness must distinguish them, not
just count them: the IR is wrong; the legacy regex was wrong (a real
card-behavior bug, the classic differential-testing payoff); or the difference
is benign and belongs in the equivalence relation.

## Second oracle, extending past the legacy 24%

The fidelity gate renders encodings back to English and diffs against oracle
text. That comparison does not depend on legacy output at all, so it extends the
same measurement to cards the regex pipeline never graduated — IR → RON →
rendered English → oracle text. Weaker than encoding parity (a wrong tree can
still render back cleanly, per the `shapes` module comment) but it is the only
signal available on the other three quarters of the corpus, and it costs
nothing new to run.

## Not in scope

- IR design, normalization, and the idempotence gate — [[english-semantic-ir]].
- Structural-guard coverage where no oracle exists —
  [[english-structural-guard-coverage]]. This ticket partly subsumes that one
  for the legacy-graduated slice: a mis-bracketed tree lowers to wrong RON and
  the diff catches it against a known-good encoding. The meter matters for the
  corpus where no such encoding exists, which is most of it.

## Completion

- An equivalence relation over encodings is defined, justified, and separately
  tested, and every benign-difference class folded into it is named.
- A reported conversion rate over the legacy-graduated corpus, with the three
  disagreement outcomes classified rather than pooled.
- Any legacy-pipeline card-behavior bug the diff exposes is split into its own
  ticket rather than silently conformed to.
- Runnable as a gate, so `deckmaste_migrations` retirement has a criterion.

Standard constraints apply.
