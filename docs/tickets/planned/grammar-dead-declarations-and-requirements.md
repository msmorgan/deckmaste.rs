---
needs: [grammar-compiler-expressions-and-tables]
---
# Dead declarations, wildcard tables, policy composition, Boolean requirements

What remains of the authoring-layer work after every Category interface was converted to head and local modes. In this order, each step a commit that keeps every reading digest identical on every face (`cargo xtask english-v3 diff --check identical-digests`). The aim is a grammar that reads more clearly; a step that does not make it clearer is skipped and said so.

1. Dead declarations. `PurposeConcord`, `PurposeHostProjection` and `coordinate_number` have no users. Then interface slots nothing reads, one feature at a time: a (feature, Category) slot is dead only if no `require`, `agree` or table argument reads it, directly or through a forward to a slot that is read, AND removing it cannot change admission. Some unread case exports still constrain admission through the table lookup that computes them; those are not dead. `Polarity` sits in 16 interfaces and is read from none. For each removed slot the landing says how it was shown dead; any candidate that cannot be shown dead stays and is listed. There were 91 candidates when the interfaces were converted.
2. Wildcard table rows. They already exist (`_` parses as `TablePattern::Any`, first matching row wins). Convert tables where that makes them clearer, keeping the complete domain: a table such as `coordinated_property_value_position`, 75 rows of which 74 yield `None`, becomes a few rows.
3. Policy composition: a policy may use policies. Cycles are rejected; precedence and conflicting overrides are defined and tested in `crates/deckmaste_construction_v3_core`.
4. Boolean predicates in `require` (`and`, `or`, `not`, `in`), only if step 1 to 3 leave tables that a predicate would state more clearly. Decided: a comparison with an absent operand is unknown, connectives are three-valued, and a requirement admits only when definitely true. A direct requirement on an optional element keeps today's behaviour and applies only when the element is present. A table becomes a predicate only if the two agree on every input including absent ones, checked mechanically per table; `clitic_complement` does not qualify. The 24 AND and 11 OR tables all compute exported values and belong to `grammar-generic-coordination`.

Each completed compiler capability updates `crates/deckmaste_construction_v3_core/README.md`. Note: census delta; the judgment-set line does not apply yet.
