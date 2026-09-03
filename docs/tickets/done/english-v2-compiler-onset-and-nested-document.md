---
needs: [english-v2-stage-5-grammar-buildout-13-10]
---
Two construction-compiler gaps the 13-10 round hit and worked around:

1. The compiler emits `onset_for_<category>` only for categories some
   construction reads onset from, so a symbol run (`{P}`) used as a measure
   unit inside a nominal fails to compile. This blocks the general "N X
   worth of Y" measure construction and with it the whole pawprint modal
   family (`Choose up to five {P} worth of modes.`, 5 faces, [CR#700.2i]).
2. `OracleText` cannot be used as a nested field (a quoted document): the
   generated code fails, so the quoted-ability interior is `DocumentBlock`
   behind a `QuotedBlock` wrapper. Add a compiler test that pins the
   current behaviour as a reproducer, then lift the limitation and switch
   the quoted interior to the nested document.

Fix both in `deckmaste_construction_core`; the grammar-side changes are
one-line follow-ups in english_v2. Standard constraints apply.

Coordinator amendment: only item 1 (onset for `{P}` symbol runs; 5 faces)
is a 14-10 blocker. Item 2 (nested `OracleText`) is non-blocking —
`QuotedBlock` over `DocumentBlock` works and carries units today; do it
here for structural correctness, but 14-10 may proceed once item 1 lands.

## Landing record (2026-09-02)

Construction feature helpers now emit for categories that carry `Onset`, even
when no existing construction reads that feature. The generated-name inventory
reserves the same latent helper, so an authored `onset_for_<category>` value is
rejected before emission. English v2's `ManaAmount` wrapper now declares the
consonant onset of its symbol-run payload; this makes the production grammar
exercise the compiler path needed by the 14-10 pawprint measure construction.

Nested structural products now propagate their context requirement through
construction fields. `QuotedBlock` therefore stores and renders an `OracleText`
directly, including its recursively boxed document-block sequence, instead of
using a single `DocumentBlock` as a workaround.

| gate | before | after |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered identities | 16,174 | 16,174 |
| unresolved ties / internal failures | 0 / 0 | 0 / 0 |
| construction declarations | 378 | 378 |
| coverage lock | current, 48,824 lines | current, unchanged |

The coverage lock remains byte-unchanged (SHA-256
`cdd3c0ae4096c6bd538f7453ad46ef3679a38ee7a36d1ffe74f7fe0a634ffe04`).
The final coverage check reports zero selected-uncovered units, exception uses,
round-trip mismatches, ownership failures, gaps, overlaps, synthetic claims,
or provenance-plan mismatches. The ambiguity gate reports 10,487 unique and
5,687 specificity-resolved selections. Both corpus gates emitted the existing
common-path wall-clock warning against the 16.260-second ceiling in this shared
debug build; both completed successfully and their accepted identity and byte
counts were unchanged.

Positive gates: all `deckmaste_construction_core` test targets; all
`deckmaste_english_v2` test targets; strict all-target Clippy for both crates;
`cargo xtask english_v2 coverage --check`; `cargo xtask english_v2 ambiguity
--require-resolved`; and `jj fix` with no changes.

### Deviations and additions

- Added compiler regressions for a latent onset provider used by a following
  onset read and for a standalone structural product nested inside a
  construction. Extended the generated-value namespace test to cover the
  latent onset helper collision.
- Re-spelled three existing AST tests for the recursively boxed `OracleText`
  document-block sequence. Their inputs and asserted parser, renderer, visitor,
  and quoted-ability outcomes are unchanged.
- Assurance counts: restored 0 tests; re-spelled 3 tests; ignored 0 tests with
  new blockers; added 2 tests; removed 0 tests.
- No constructions were added or deleted. No CR citations changed. No STOP was
  taken.

## Erratum (compiler-onset landing review, 2026-09-03)

Only the compiler half of item 1 landed: all 5 pawprint faces remain
parse failures until 14-10 builds the "N {P} worth of Y" measure
construction. Undisclosed widening: the quoted interior is now a
newline-separated block sequence (`OracleText` is a `seq`), so a
multi-line quoted grant parses — unattested (0 corpus units) but
well-formed English, admitted under generative fidelity; recorded here.
`mana_amount`'s Consonant onset is authored without corpus attestation
(0 `a {…}` witnesses; `+1/+1` precedent). Namespace hole for the other
nine provider features routed to construction-core-provider-namespace.
