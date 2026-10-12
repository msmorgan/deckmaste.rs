---
needs: [english-v3-licence-plumbing-economy]
---
# Boolean requirements, policy composition, wildcard tables, dead slots removed

Second half of replacing the authoring layer: finish the compiler capabilities the prototype did not need, then convert the whole grammar to use them.

## Change

- Requirements accept `and`, `or`, `not` and `in` (today an equation is an export of the same feature, a constant, a table call, `agree`, or a `require` of a constant or table result: `crates/deckmaste_construction_v3_core/src/parse.rs:127–137`). This removes the hand-written two-argument `{No, Yes}` tables: 24 AND and 11 OR.
- Policies compose: a policy may use other policies. Today a policy admits only feature equations (`parse.rs:620–648`, check at `:634–645`) and `use` is rejected in policies; 135 policies span 1,143 lines, 61 referenced once, 2 never.
- Wildcard table rows. 216 tables span 1,288 lines (about 1,207 arms). A 75-row table such as `coordinated_property_value_position` (only additive first plus second yields a pair, every other row yields none) shrinks to a few rows; the complete domain is preserved, so a missing row must not reject valid coordination.
- Dead slots are removed from the compiler.
- Convert all of `declarations.rs` to the new forms, class by class. Remove every declaration the conversion makes unused. Expected export saving across the file: about 1,306–1,575 lines.

## Gate

Every `readings_sha256` identical on every face after each class of conversion and at the end. Three-line note: census delta (none); size delta per declaration kind (the ratchet file is lowered to the new totals); judgment-set result.
