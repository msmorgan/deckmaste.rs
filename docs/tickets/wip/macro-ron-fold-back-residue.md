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
