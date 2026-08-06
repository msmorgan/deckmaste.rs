---
needs: [english-coordination-derived]
---
**Inventory every remaining English family and mint its vertical migration.**

Committed continuation of `docs/decisions/english-grammar-is-derived.md`, not
an adoption gate. Enumerate every handwritten family in the construction
registry plus the `Cost`, `KeywordLine`, and `Ability` families owned by the
ability layer. No supported family may remain represented only by an informal
"later" bucket.

For each family record: stable `ConstructionId`; current parser, reduction,
lowering, renderer, and constructor owners; AST shapes; required subtree,
field-slice/lens, scalar, or identity holes; feature, valency, and attachment
constraints; fan-out and backend; ambiguity/dominance relation; stored and
derivable surface witnesses (semantics-bearing vs house style:
`docs/oracle-style-guide.md`); active frame/spelling and serialized consumers;
and the direct-AST, `inspect`, exactness, and negative fixtures that gate its
migration.

Classify families by the first missing capability:

1. fan-out-one whole-subtree declarations on the existing chart;
2. local field-slice/lens, scalar, or identity holes;
3. selection, valency, gap, or attachment constraints;
4. tuple-valued/discontinuous yields requiring PMCFG or a measured CFG
   approximation plus filtering; and
5. the generated ability-layer backend.

Mint the smallest independently migratable vertical-family tickets from that
inventory. Each ticket lands declaration, parse/render/build projections,
consumer and serialized-view changes, direct-AST/`inspect` fixtures, registry
flip, and deletion audit together. Dependencies name only capabilities the
family actually needs; capability-independent families remain parallel.
Heterogeneous and member-scoped coordination from
`english-coordination-structural-design` enter the earliest compatible family
ticket rather than becoming permanent exceptions.

Update `english-derived-grammar-completion`'s `needs:` with every minted chart
family ticket. The separate `english-ability-construction-backend` node already
represents the known ability-layer wave.

Gates: the registry census accounts for every supported family exactly once;
every handwritten row has a migration ticket and completion dependency; no
ticket defers its consumers or deletion work; `scripts/todo check` reports a
complete acyclic graph. This inventory changes no production behavior.

Standard constraints apply.
