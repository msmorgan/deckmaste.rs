---
needs: []
---
# Replace erased library errors with typed thiserror errors

Convert reusable library APIs that return `anyhow::Error` to concrete error
types derived with the workspace-pinned `thiserror`. Start with
`deckmaste_lexical_source::load_workspace`: callers currently have to inspect
messages to distinguish invalid declarations, unresolved owners or markers,
duplicate properties, decoding failures and filesystem failures.

User decision (2026-09-08): erased errors are limiting for crates like these;
callers need proper error types. The earlier `thiserror-adoption` ticket kept
`anyhow` at application boundaries. Preserve that distinction: CLI orchestration
may add `anyhow` context around typed library errors.

Audit the surviving library boundaries at implementation time, including
lexical-source loading, catalogs, snapshot data, spelling and migrations.
Convert cohesive error families per owning crate and update their callers.
Respect the existing crate-retirement decisions; do not refactor an obsolete
implementation merely to remove its dependency. Record each remaining
library-side `anyhow` use with its reason and surviving replacement owner.

Errors must expose actionable variants and relevant structured fields such as
paths, declaration identities and feature names. Preserve underlying I/O,
decoding and dependency errors through `source()`. Keep useful diagnostic
context; a string-only variant or an `Other(anyhow::Error)` wrapper does not
restore a typed boundary. Reuse existing concrete dependency errors where
appropriate instead of introducing a parallel classification.

Acceptance: callers can distinguish representative failure causes by matching
variants and fields without parsing `Display` text or downcasting an erased
error. Tests exercise real failing loader/validation paths and verify their
structured context and source chains; existing successful loading and CLI
diagnostics still work. Report the boundary inventory and remove unused
`anyhow` dependencies from converted libraries. Standard constraints apply.
