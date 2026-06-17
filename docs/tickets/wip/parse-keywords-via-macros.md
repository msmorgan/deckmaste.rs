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

## DONE

- **ResolveCtx ABI**: `AbilityParser = fn(&str, &ResolveCtx)`; `ResolveCtx {
  kind, index: &TemplateIndex }`. `resolve_cards` loads the plugin `MacroSet` +
  builds the index once (so `xtask::generate` is unchanged). All 7 bespoke
  parsers updated.
- **`macro_template` parser** leads the `REGISTRY`: full-line nullary
  `match_kind("KeywordAbility", line)` → `Keyword(<name>)`, else declines
  (first-match-wins). Param-less keyword *lines* now derive straight from the
  templates.
- **Param-aware index**: `ParsePattern.has_params` + `emits_bare()` — the index
  only claims param-LESS macros, so defaulted-param keywords (`Hexproof`,
  `Landwalk`, whose templates are slot-less) keep their bespoke `Name(...)`
  form. Their bare invocation awaits `macro-bare-defaulted-invocations`.
- **`KEYWORD_NAMES` hand-list RETIRED** → derived: a `LazyLock<Vec<String>>`
  loaded from `data/rules/keywords.json` (Scryfall `keywordAbilities`, the same
  source the stub generator reads) ∪ a small documented `KEYWORD_SUPPLEMENT`
  (~30 parser-specific landwalk/cycling variants + a few multi-word forms
  Scryfall doesn't enumerate). The derived set is a superset of the old
  hand-list (+22 keywords it had missed), so zero regression by construction.
  The single hand-list was the dual-purpose blocker (`bare_keyword` AND
  `match_keyword_name`); deriving it in place retires it without threading the
  index through effect/modify.

Deferred (not this ticket): integer/parameterized keyword *lines* (Annihilator,
Ward, Protection) still bespoke — they need the slot codec (`macro-slot-codec`);
deriving the landwalk/cycling supplement from basic land types; bare
defaulted-param invocations (`macro-bare-defaulted-invocations`).
