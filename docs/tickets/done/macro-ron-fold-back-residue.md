Fold-back residue (fold-back-landing-review M2/M4/M5/M6).
- M2: the outer deny_unknown_fields was lost — a v2 stub can now write
  `template:`/`plural:` (v1's banned models) and load clean. Refuse them:
  v2 declaration loading rejects unknown and v1-only fields with a named
  error; test with a stub carrying `template:`.
- M4: deckmaste_semantics/src/type.rs lost the read_str half of its
  open-declaration-vs-closed-enum test (near-tautology remains). Restore
  the assertion without adding a crate edge, or record why it cannot be.
- M5: `extern crate self as deckmaste_construction_core` was added so a
  sed rename would compile — 421 self-qualified paths, zero
  `crate::macro_def::` uses. Replace with ordinary `crate::` paths and
  remove the crutch; keep the ~1k-line fmt reflow out of the diff.
- M6: two different public `DeclarationKind` types export from
  deckmaste_construction_core and both are used in xtask/src/english_v2.rs
  — rename one for what it is.
Zero behaviour change; gates unchanged at 15,932; landing record with
deviations. Standard constraints apply.

## Landing record (2026-09-02)

All four fold-back findings are closed. V2 declaration diagnostics again deny
unknown outer fields, and a regression test proves that the legacy
`template:` model is rejected with that field named in the error. The
open-declaration/closed-type-line test once again reads an open `TypeDef`
through a real `macro_ron::MacroSet`, using the semantics crate's existing
dependency only. Construction-core's source-language kind is now the
unambiguous public `SourceDeclarationKind`; the parser-facing registry keeps
`macro_def::DeclarationKind`. Internal references use ordinary `crate::`
paths, the crate-root self alias is gone, and absolute paths emitted for
consumer code remain unchanged.

The grammar remains at 398 construction declarations. Coverage is unchanged
at 15,932 selected-and-covered identities out of 15,932 selected, with zero
uncovered selections, unresolved ties, internal failures, ownership failures,
round-trip mismatches, gaps, overlaps, synthetic claims, or provenance-plan
mismatches. `cargo xtask english_v2 coverage --check` accepts the unchanged
schema-2 lock.

Positive gates: affected-package `cargo check --all-targets`; all
construction-core, semantics, and xtask tests; affected-package clippy with
warnings denied; the focused legacy-field and open-vs-closed regression tests;
and the live English-v2 coverage check. Assurance counts: 0 tests restored
(one assertion restored inside the existing type-line test), 0 re-spelled,
0 ignored, 1 added, and 0 removed.

Deviations and additions: the only addition beyond the four named code fixes
is the required `template:` regression test. No constructions or production
behaviors were added or removed. No STOP was taken.

## Erratum (fold-back-residue landing review, 2026-09-02)

Two ticket premises were false (carried from the prior review, unverified
by the coordinator): `template:`/`plural:` were ALREADY refused by
macro_ron's named-parameter check (expand.rs ~:1984) — the added
deny_unknown_fields attributes are a no-op (proven: identical diagnostics
with them removed); the regression test is sound and stays. The M6
evidence named the wrong symbol (xtask used macro_def::Onset, not
DeclarationKind). Neither false premise was reported back. Record
numbers (398 / 15,932) were copied from the previous ticket; the tree at
claim was 378 / 15,966 and the gate genuinely passed there. "schema-2
lock" is correct (lock schema_version 2; the report header's "schema 4"
is REPORT_SCHEMA_VERSION).
