---
needs: [english-v2-corpus-wide-visitor-traversal-property]
---
Extend the corpus-wide traversal property to lexeme leaves (visitor-traversal
landing review MEDIUM-1). Today the coverage gate proves every selected unit's
visitor enters exactly its ordered construction path, but leaf visits are
unobserved: a generated visitor that drops every lexeme/terminal leaf visit
(`emit/visit.rs` leaf arm returning `None`) leaves `coverage --check` green.
The materialized derivation already knows its leaves in surface order; record
the visitor's leaf visits alongside `enter_construction` and require ordered
equality with the derivation's leaves (or the surface token sequence) per
selected unit, reported in the coverage report as visited/expected leaf counts
with per-unit failure evidence. Mutation-verify: the leaf-drop mutation must
fail the gate. Zero grammar change; lock byte-unchanged. Gate scope: emit/
change → `cargo test --workspace`. Standard constraints apply.

Also (literal-opacity landing review LOW-1, same `emit/` directory): add a
short comment at the `Bound`/`Circumfix` arms of `emit/final_constituent.rs`
stating that the fold deliberately looks through delimiters — the predicate is
"rightmost non-delimiter constituent" — and that `quoted_ability` and the two
quote-terminator `checked by` sites depend on it. Facts only; no essay.
