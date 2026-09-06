---
needs: []
---
[design] **Replace author-supplied stack offsets with grammatical reference
scopes, or explicitly separate the expanded representation that owns them.**
The 2026-09-05 semantics_v2 readiness review confirmed through Lean LSP that
ordinary `it` refuses two creature antecedents, while
`.pro .bare .one (.top 1)` accepts the same context. `Window.top` and
`Window.below` in `lean/Semantics/Words.lean` are public syntax.
`Macros.itPrior`, `itCondSubject`, `agentRef`, `itsOther`, and `lookedCards`
calculate offsets from checker bindings, sometimes against `[]`.

The existing contract in `docs/decisions/semantics-v2.md` assigns positional
references to lowering and requires unique compatible antecedents for ordinary
pronouns. Lean is the successor workbench; preserve that distinction unless
the user explicitly revises it. Decide how a clause's own subject, a previous
instruction's mentions, and a library slice are named grammatically. The
representation is not settled by this ticket.

Done when ordinary authorable pronouns cannot bypass ambiguity by choosing a
depth; internal references have an explicit construction/validation boundary;
and the affected macros preserve their referents when unrelated outer bindings
are added. Re-spell the existing cards and pins, preserving their assertions.
Add a negative ambiguity pin and positive scoped-reference twins using Lean
LSP. Audit empty-context offset calculations rather than merely renaming them.

The parked `workbench-segment-indexed-telescope` concerns recovering loop deltas;
this ticket does not reopen its ruling or require that implementation.
