---
needs: []
---
**Land the shared English feature vocabulary and its strata.**

First slice of `docs/decisions/english-grammar-is-derived.md`: one canonical
closed Rust type per grammatical concept, stratified as the decision records
(inherent realization features / selection features / surface witnesses /
discourse occurrence roles).

1. Inventory every current parser, reduction, AST, renderer, and frame-facing
   encoding of the named concepts. For each concept, identify the canonical
   type, duplicate encodings to retire, serialized representations that must
   remain stable, and whether the fact participates in chart identity.
2. Seed the inherent stratum from the existing closed enums (`Person`,
   `Number`, `VerbSlot`, `NounCardinality`); add onset, case, pronoun class,
   and conjunction shape only where a current parser or renderer site encodes
   the same closed concept ad hoc.
3. Migrate parser and renderer sites concept by concept, keeping temporary
   conversions only at unmigrated family boundaries. Delete each parallel
   encoding once its final caller moves.
4. Make the strata enforceable at their carriers: inherent and selection
   features may participate in chart keys; surface witnesses travel on packed
   derivation alternatives and exact parse results; discourse occurrence
   roles remain spelling-owned and never enter construction matching.
5. Document each canonical type's stratum where it is declared and add a
   closed inventory test so a newly introduced grammatical feature must choose
   a stratum deliberately.

Packing may share an otherwise identical chart node, but it may not discard or
coalesce alternatives carrying distinct surface witnesses. This ticket must
include a focused packed-forest/exact-result fixture proving that distinction.

No intended behavior change: `cargo xtask english roundtrip --require-clean`
stays clean, the recovery census stays byte-identical, serialized fixtures do
not churn, and no parser/renderer pair retains parallel definitions of a
migrated concept. This slice may proceed while
`english-coordination-comma-defects` completes.

Standard constraints apply.
