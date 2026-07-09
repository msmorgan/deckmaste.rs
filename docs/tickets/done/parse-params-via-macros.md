---
needs: [macro-slot-codec]
---
Extend `parse-via-macros` past nullary / integer keywords to the full
**parameterized** corpus across kinds: `Protection(<filter>)`, `Ward(<cost>)`,
pump `${0}/${1}`, count-bearing effects, filter and effect sub-clauses. Uses the
`macro-slot-codec` to invert each `${i}` slot from English into arg RON, and the
`macro-parse-index` matcher to recognize the skeleton. The generic
`macro_template` parser also serves as a typed sub-reader that the frame parsers
(`effect.rs`, triggered, activated) call for nested fragments.

## Resolution — coherent core already delivered; closed as satisfied

Batch execution (2026-07-09) verified the ticket's single-reference core is
**already live**, delivered by the dependency plus intervening work:
- `Protection(<filter>)` / `Ward(<cost>)` route through
  `macro_template.rs::match_with` with `Predicate`/`Cost` slot readers
  (`macro-slot-codec`); tests `routes_parameterized_keyword_through_slots`,
  `claims_conditional_param_keyword` pass.
- Count-bearing effect bodies route via `effect.rs::parse_macro_effect`; the
  graduated corpus emits 192 `Scry(N)`/`Surveil(N)` invocations.
- Filter/effect sub-clauses (`parse_declarative_subject`, `parse_if`,
  `counter_kind`) and nested frame fragments (`triggered`/`activated` →
  `effect::parse_clause`) all route through the index (parse-batch campaign).
- pump `${0}/${1}` stays on the bespoke `parse_pump`, which is strictly richer
  (for-each scaling, keyword-grant riders, team/target subjects) — routing it
  through a macro would regress hundreds of parsing cards, contrary to the
  standing "pump bespoke is richer" ruling.
- The `Predicate`-kind FILTER slot reader the dependency deferred here has **no
  macro consumers** (every `Predicate` macro is nullary; `filter.rs` is
  deliberately bespoke), so adding it now would be dead code.

`cargo test -p deckmaste_migrations` = 272 passed / 0 failed; the parse-via-macros
path is green.

The one genuinely-remaining class — **multi-reference / target-hoisted** effect
lines (fights, exile-then-return) that a flat `${i}` slot reader cannot express —
is carved into the design-gated follow-up
`parse-target-hoisted-effect-macros`. No unilateral design call was made here.
