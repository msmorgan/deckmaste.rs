---
needs: []
---
**Mine rare AST productions across the supported corpus to generate misparse
candidates mechanically.** Discovery instrument for the parse-defect class that
every existing gate is blind to: round-trip proves a parse is *lossless*, never
*correct*, so a misattached modifier renders back byte-identically and reports
full structural recovery. Measured 2026-07-29: 31,685 supported faces, 0
round-trip mismatches, 0 render errors — every free signal saturated green while
`english-ast-grouping` was fixing 763 genuinely wrong rows found by hand.

Hand-auditing is the current discovery method and it does not scale to 31,685
faces. Rarity mining replaces it: annotation defects concentrate in
low-frequency productions, because correct structures recur across cards and
misparses are idiosyncratic.

- Derive `Serialize` (NOT `Deserialize`) on the AST types reachable from
  `OracleText`. The derive is bought for its free, exhaustive,
  compiler-checked traversal — it stays correct as the AST changes, which a
  hand-written visitor does not. Some atoms (`CatalogAtom`, the lexicon `Word`
  types) likely live in `deckmaste_core`; confirm the crate boundary first.
- Implement a counting `Serializer` sink in xtask. Do **not** route through
  `serde_json::Value`: externally-tagged enums collapse a unit variant and a
  string to the same `Value::String`, so card names and `spelling` fields would
  become millions of spurious singleton signatures and bury the real tail. The
  `Serializer` sees `serialize_unit_variant` and `serialize_str` as distinct
  calls; the ambiguity never arises.
- Emit a production signature per node — local tree with list fields flattened
  to their child-variant sequence. Two granularity knobs, computed in one pass:
  `--lexicalize none|head` (whether head atoms enter the signature) and
  `--depth 1|2` (include the grandparent edge for attachment context).
- Output TSV sorted ascending by count: `count · signature · 3 exemplar faces ·
  compact subtree excerpt`.

**Rarity output is a worklist, never a finding.** Rarity is not proof; nothing
ships from this report directly. Each confirmed family gets encoded as a sound
lint (see the `english-lint-*` siblings), and the lints are what report
defects. Steady state stays machine-adjudicated: a lint firing is itself proof.

Calibration: the miner must be validated against a corpus with known defects
before its tail is trusted. `english-ast-grouping`'s pre-fix state is the
available labeled snapshot.

Relationship to `english-recovery-walker-derive`: that ticket replaces the
hand-maintained `RecoveryWalker` and is gated on the recovery campaign. This
miner is a NEW consumer, not a replacement — do not touch `RecoveryWalker`
here. If the serde derive proves out, it becomes a candidate shape for that
ticket's "derived walk".

Standard constraints apply.
