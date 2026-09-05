---
needs: [english-lean-grammar-design-review]
---
# Plan the constructions! migration from the formal grammar

Design the production migration from the reviewed Lean `English` model into
`english_v2`, following the [Lean design decision](../../decisions/english-lean-design-workbench.md).
Keep declaration-owned types, parsing, rendering, and traversal. Assess reuse
of the chart engine, declaration compiler, lexical environment, scanner,
materialization, selection, and scope machinery against concrete requirements.
Compiler changes are allowed when needed; neither a separate `english_v3` nor
automatic consumption of Lean is required.

Write the correspondence in `docs/english-grammar-design.md`: which formal
categories, judgments, and surface relations map to which declarations or
compiler facilities, and where tests rather than proofs will establish Rust's
behavior. Identify coherent replacement boundaries after considering mutually
dependent categories; temporary compatibility must earn its complexity.

Reconcile the unclaimed implementation tickets listed in the wayfinder. Keep
their examples, negative cases, lost-coverage identities, and independently
justified outcomes; replace obsolete mechanisms and scheduling. Some inherited
prescriptions may conflict with the glossary or formal design: name and resolve
the conflict explicitly. Make each resulting implementation ticket executable
with pinned scope, dependencies, replacement/deletion targets, and structural
acceptance. Retire merged tickets only after moving all obligations and edges.
Do not edit WIP tickets or claims; assess their integrated results when available.

Give every fog family and re-coverage obligation a production owner or an
explicit deferred destination. Update `english-v2-grammar-migration-close`'s
`needs:` to exactly the resulting migration tasks, including tasks minted for
requirements not represented in today's backlog. Update the wayfinder and
validate the graph before closing this design ticket. The old Group R order
does not constrain the new dependency graph.

Acceptance: the implementation sequence accounts for the reviewed model and
all inherited obligations, identifies the surrounding machinery's fate, and
can be executed without rediscovering the grammatical design. No production
code changes or production coverage measurement are required for this ticket.
Standard constraints apply.
