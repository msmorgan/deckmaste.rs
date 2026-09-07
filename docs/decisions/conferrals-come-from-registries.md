# Conferrals come from registries

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

> Amendment (2026-09-07): registries carry what a designation **is**, not who
> confers it. `DesignationDecl.conferrers`, the `confers` columns on the
> keyword-ability, keyword-action and deed facts, and the `Conferral` tag on
> `Instruction.gainDesignation` are all removed. A keyword's definition body
> grants the designation, and that is the whole fact [CR#701.37a]; the same
> fact kept three times and checked for agreement was bookkeeping, not a rule.
> The registry still supplies the designation's scope, holder, persistence and
> whether an instruction may confer it at all.

## Decision

Type, subtype, counter, and designation behavior is sourced from loaded
registry values. Rust and Idris consumers derive from those values rather than
recreating conferrals or scope through name-based match tables.

## Rationale

These namespaces are open plugin data. Parallel name maps drift, require edits
for every new declaration, and can make validation disagree with runtime
behavior.

## Consequences

Registries carry the structural fields and conferred properties consumers need.
Emitters and engine systems may translate those values into their own
representations, but a new declared name must not require a corresponding
hard-coded behavior branch.

## Tracked references

- [Rules taxonomy](../rules-taxonomy.md)
- [Keyword policy](../keyword-policy.md)
- [Card data](../card-data.md)
