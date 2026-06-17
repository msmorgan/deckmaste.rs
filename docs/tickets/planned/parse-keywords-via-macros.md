---
needs: [macro-parse-index]
---
MVP of `parse-via-macros`: route keyword-ability parsing through the
`macro-parse-index` lookup. Add a generic `macro_template` parser **first** in
the resolve `REGISTRY` (ahead of `keyword_ability`); on a KeywordAbility-kind
match it emits `Keyword(<invocation>)`, on no-match it returns `None` and the
bespoke parsers handle the rest — first-match-wins is the clean handoff, no
double-handling.

Covers nullary keywords (Flying, …) and integer-arg keywords (Annihilator, Crew)
— integers already round-trip without the full codec. Retire the corresponding
`KEYWORD_NAMES` / `match_keyword_prefix` / `render_arg` machinery; keep the
irregular tail (hybrid-cost slots, quality-word equip, genuinely compound lines)
bespoke. Proves the round-trip — authored macro ⇄ rendered ⇄ parsed, one source
— before the parameterized cases (`parse-params-via-macros`).

NB — wiring burden discovered in `macro-parse-index`: **resolve has no `MacroSet`
today**. The `AbilityParser`s are `fn(&str, CardKind)` and the plugin/`MacroSet`
loads only at the `graduate` phase (resolve runs before it in
`xtask::generate`). So this ticket also owns: load the plugin's `MacroSet` and
build the `TemplateIndex` before `resolve`, and thread it through the
`AbilityParser` signature (or a resolve-scoped context) so the generic
`macro_template` parser can consult it. `TemplateIndex::build` +
`match_kind(kind, input)` already exist in `deckmaste_cards::template`.
