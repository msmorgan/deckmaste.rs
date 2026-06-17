---
needs: [macro-template-sigil]
---
Lookup primitive for `parse-via-macros`: a kind-scoped reverse index over the
registered macros — `template-shape → macro` — so a parser that knows the macro
kind it targets (KeywordAbility, Effect, Filter, …) finds the macro whose
template matches a parsed fragment and emits its invocation.

Each templated macro compiles its `template` into a `ParsePattern` = an ordered
sequence of `Literal(str)` | `Slot{index/name, param_type}`; `Slot.param_type`
is read from `MacroDef.params`. Stored `Kind → [ParsePattern]`, matched
case-folded + word-boundary anchored, ordered by specificity (greatest total
literal length first — this resolves the one prefix pair, `flash` ⊂ `flashback
${0}`). Expose `match_kind(kind, input) → Option<Match{macro_name, slots,
consumed_len}>`.

Scope here: build the index + matcher + **nullary matching** (the 33 nullary + 1
self-only templates — Flying, AnyTarget, battle, Sacrifice this permanent, …).
Validate on those and delete the nullary half of `keyword_ability.rs`'s
`KEYWORD_NAMES`. Typed-slot filling is `macro-slot-codec`; rerouting a top-level
parser is `parse-keywords-via-macros`.
