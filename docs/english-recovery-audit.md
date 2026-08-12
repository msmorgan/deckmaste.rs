# Structural-recovery causal audit

`cargo xtask english recovery --worklist` exports every structural-recovery
group in the live supported corpus. Unlike the bounded human `--list` view,
the JSON worklist has no row limit. It is a transient review artifact: write it
outside the repository, annotate it during the sweep, and do not commit it.

```text
cargo xtask english recovery \
  --worklist /tmp/english-recovery-worklist.json
```

Each row is one exact `(role, recovered text)` group. `group_id` is a SHA-256
identity over those two fields. `structural_signature` records the recovery
role and token-kind sequence; `lexical_signature` lowercases words and
abstracts numeric and braced-symbol slots. The two signatures make related
rows sortable and clusterable without treating a corpus row as grammar
authority. Occurrence counts, recovered-token counts, and bounded witness names
are leads for the audit only. Verification intentionally does not preserve
those aggregate values.

The export initializes every `disposition` to `null`. Replace it with one
causal record after direct constituent probes establish the gap:

```json
{
  "cause": "composition",
  "resolution": "implemented",
  "rationale": "The generated constituents parse separately but lacked their parent production.",
  "evidence": [
    "direct typed-tree, source-independent render, and registration-permutation test"
  ]
}
```

The accepted implemented causes are `composition`, `feature_transition`,
`admission`, and `lowering`. They require `resolution: "implemented"`, a
nonempty causal rationale, and at least one nonempty evidence entry. An
implemented group must be absent from a fresh runtime census.

Recovery may remain only with `cause: "unsupported_grammar"` or
`cause: "unsupported_lexicon"`. Those causes require
`resolution: "follow_up"`, a nonempty rationale, and `follow_up_ticket` naming
the focused ticket that owns the missing domain:

```json
{
  "cause": "unsupported_grammar",
  "resolution": "follow_up",
  "rationale": "The isolated constituent needs a construction family not represented by the generated grammar.",
  "follow_up_ticket": "docs/tickets/planned/english-example-domain.md"
}
```

After implementing omissions and minting the focused follow-ups, verify the
annotated worklist against a new full parse:

```text
cargo xtask english recovery \
  --verify-worklist /tmp/english-recovery-worklist.json
```

Verification fails if any original row lacks a disposition, an identity or
signature was edited, a current recovery group was absent from the original
worklist, an implemented group still recovers, or a follow-up group has
disappeared. Thus one export carries the initial complete work queue through
the sweep while the final result is proved from current runtime behavior. No
corpus rows or aggregate recovery snapshot become repository inputs.
