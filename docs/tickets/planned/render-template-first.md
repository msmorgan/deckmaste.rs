---
needs: []
---
Make the card-text renderer (`deckmaste_cards::render`) consistently *template-first,
structural-fallback*. Today that pattern exists at exactly one of the ~15 `Expanded`
positions (`Effect::Expanded` in `effect.rs`); every other `X::Expanded` arm discards
the macro's `name`/`template` (carried on every `Expansion<T>`, populated for all kinds
at expansion) and reverse-engineers the same phrasing back out of the expanded
structure with brittle, deep match arms.

v1 (no design gate, output-identical — the golden render tests pin every string):
- Add one generic helper (`expanded_phrase`/`expanded_text`) = "if the wrapper carries
  a fillable `template`, return it; else `None`" so callers fall back uniformly to
  structural rendering of `exp.value`.
- Route the **nullary** template wins through it and delete the heuristics they replace:
  - `fragment::target_spec` → read `AnyTarget`'s `template: "any target"` off the
    wrapper; **delete `is_any_target`** (the 4-member-`OneOf`-with-a-`Kind(Player)`
    heuristic, ~18 lines).
  - `fragment::filter_noun` → read `Creature`/`Player`/`Planeswalker`/… filter macros'
    `template` for the noun core; demote `find_card_type`/`strip_expanded` digging to
    the fallback path for un-wrapped (hand-built) filters.

Keep structural rendering as a permanent floor: hand-built `CardView`s (tokens, derived
live objects, the synthesized-filter tests) carry no `Expanded` provenance. Leave the
genuinely-structural branches alone — closed enum→string tables (`type_str`, colors,
counts), and composition not owned by any single macro (Other-/you-control framing,
When/Whenever, Also/Instead, Must/Cant).

Deferred (needs a design dialogue, **[design]**): extending `template::fill`'s
`render_arg` beyond integers so `{i}` can render a grammar-node arg (filter/reference) —
the blocker for routing event clauses (`{0} dies`/`{0} attacks`) and arg-bearing macros
through templates. `ExpansionArgs` stores args as raw RON source, so the choice is
re-parse-the-source vs. index-into-the-typed-`value`; pick before coding. AsEnters'
partial template (`"as ~ enters"`, no `{0}`) also wants resolving here.
