---
needs: [ci-idris-gate, ci-noncanon-gate, comment-reviewer-hotspots, engine-verb-ident-table, readme-oracle-text-primer, readme-reviewer-path, todo-harness-ci]
---
**Epic: the presentation capstone.** Close the reviewer-facing credibility
gaps from the 2026-08-03 external review, then verify the result as one cold
clone. This is a publication boundary, not a demand to finish every internal
cleanup ticket.

The needs list covers three claims the capstone must be able to defend:

- **The advertised path is real:** a reviewer gets direct artifact links and
  a clean clone can build, run the demo, and execute the documented checks.
- **The gates are real:** citation, Idris lockstep, noncanon, ticket-graph, and
  ordinary Rust suites execute in CI rather than surviving as comments,
  ignored files, or maintainer-machine commands.
- **Known failure modes are deliberate:** engine-interpreted verbs are not
  typo-prone string dispatch, and every remaining engine abort is classified
  as malformed semantic input, unsupported mechanics, or an internal invariant.

Final pass:

1. **Disposable fresh-clone verification.** Re-run `docs/publish-prep.md` §6
   in an environment with no pre-existing `data/`, `target/`, mtg-rules skill,
   or repository-specific environment variables. Follow the README literally:
   build, first-run `cargo run`, `cargo test`, and every newly wired gate.
   Record the commands, toolchain/OS, elapsed time, and result in the publish
   checklist so the proof is repeatable rather than "worked on my machine."
2. **README claims audit.** Check every externally visible claim against that
   run. Retire ci.yml gap comments that closed. Soften or fix the chart-parser
   framing (the chart grammar currently tops out at `Sentence`; the
   ability/document layer is hand-written recursive descent) unless the
   derived-grammar program has moved the boundary by then.
3. **Regression sweep.** Run `jj-kata kanban check`, the complete citation
   toolchain, the Idris baseline, the noncanon fixed-seed suite, and the
   work-log-vocabulary grep from `comment-reviewer-hotspots`.

Explicitly outside this capstone: the full multi-million-token
`comment-discipline-sweep`, the design-gated
`lowering-mirror-drift-gate`, pure helper/function extraction tracked by
`review-followups-2026-08-03`, the external method write-up, and
`post-reshape-comment-rot`.
