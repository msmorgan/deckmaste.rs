---
needs: []
---
Type-check macro parameters (Color, Filter, etc.) at the graduate gate, so that
ill-typed invocations are caught during graduation rather than at engine runtime.

## Closed as already-implemented (2026-06-14)
The headline ask was already satisfied: `param_types()` registers real-type
validators for `Color` and `Filter` (`crates/deckmaste_cards/src/macros.rs:95,101`),
run by `validate_arg` at the graduate gate (`crates/macro_ron/src/expand.rs:962/1034/
1067`), pinned by tests (`color_is_a_registered_param_type`,
`basic_land_type_rejects_non_color`, `basic_land_type_accepts_color`). Closed with no
new code. The "etc." (validators for further param types) is unpinned new scope — file
a fresh ticket if/when a specific type needs one.
