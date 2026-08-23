---
needs: [english-v2-plan-06-forms-and-selection]
---
**Complete and integrate Stage 5 Plan 07: editorial and nominal grammar.**
This plan-boundary completion ticket carries only the reviewed Plan 07 work
and must land before Plan 08 implementation starts.

The completed slice expands the generated nominal grammar through articles,
modifiers, comparisons, count references, coordination, objects, damage, and
destroy forms. Full card names are opaque context identities; shortened
self-reference is licensed only by authoritative per-face `Legendary`
metadata and retains only the full/abbreviated arm in the AST. The source-name
boundary and [CR#201.5c] ruling are recorded in
`docs/decisions/english-v2-rewrite.md`. The production card-name adapter also
derives the complete set outside the bounded onset recipe and requires it to
equal a closed eight-row reviewed per-surface override inventory; no generic
alphabetic, Unicode-class, or punctuation-prefix fallback remains.

Completion evidence on the task-closed tree:

- construction core: 284 unit tests passed;
- construction proc macro: all 45 compile-fail fixtures and 18
  compiled-consumer tests passed;
- macro_ron: 190 library tests and 15 integration tests passed;
- English v2: 129 library tests, 100 integration tests, and 2 doc tests passed;
- xtask: 436 library tests passed, 1 ignored, and 12 CLI tests passed;
- strict five-crate Clippy, changed-file nightly formatting, the five-test
  Plan 04 authority audit, citation checks/audit, and
  expand/report/probe/inspect smokes passed;
- the frozen Plan 07 manifest remains SHA-256
  `3535a10fd5bcecc86dee14c1df28d4f66478f724b0dc465945063a6efb01723d`:
  exactly 145 unchanged, category-counted IDs are selected, byte-exact,
  totally owned, and disjoint from the 412-ID Plan 06 baseline;
- the schema-2 coverage lock is an add-only 412→600 ratchet (+188), preserves
  every Plan 06 ID, includes every Plan 07 target, equals the full production
  `SelectedCovered` ID set, and retains corpus
  fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`;
  the normalized corpus is exactly 32,641 total / 600 selected, covered,
  clean, byte-exact, and totally owned / 32,041 ordinary parse failures;
- every selected-uncovered, unresolved-tie, internal-failure,
  round-trip-mismatch, ownership-failure, gap, overlap, synthetic-claim,
  provenance-plan-mismatch, selection-exception-inventory, and exception-use
  counter is zero;
- the seven separate PlanGate samples were expand 0.768224677s, report
  0.479894892s, parse 12.514651940s, roundtrip 11.662065268s, ambiguity
  12.242661671s, coverage 12.718620994s, and require-complete 11.546378533s.
  Every sample stayed at or below 16.26 seconds. Expand emitted
  `WARNING english-v2-plan07-relative-slowdown gate=expand elapsed_seconds=0.768224677 warning_seconds=0.300000000`;
  report emitted
  `WARNING english-v2-plan07-relative-slowdown gate=report elapsed_seconds=0.479894892 warning_seconds=0.225000000`.
  The require-complete child reported the expected incomplete 600-of-32,641
  Stage 5 boundary with `expected_incomplete=true`.

The full-corpus chart package is optimized in the dev profile, consistent
with the workspace's existing runtime-package policy: controlled identical
parse samples improved from 44.224285016s to 12.380953294s, with a second warm
sample of 12.392660320s. Standard constraints apply.
