---
needs: [plugins-v3-builtin-actions-and-definitions]
---
# Migrate `Templates.lean` into `plugins_v3/builtin/{primitives,templates}/`

**Split `lean/Semantics/Templates.lean` (~2,508 lines) into individual `.lean` files under
`plugins_v3/builtin/primitives/` (1-to-1 `Semantics.Primitives.*` bindings) and
`plugins_v3/builtin/templates/` (composite phrasings), and enforce that only `primitive`
declarations reference raw `Semantics.Primitives.*` constructors.** Fourth of four atomic tickets
migrating Lean declarations into `plugins_v3/builtin/`
([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5). Standard
constraints apply.

## Scope

1. **Extract 1-to-1 AST constructor bindings into `plugins_v3/builtin/primitives/`:**
   - Move direct surface wrappers over a single `Semantics.Primitives.<Type>.<ctor>` constructor into
     `plugins_v3/builtin/primitives/{action,ability,condition,cost,determiner,event,mana,noun,pile,predicate,pronoun,quantity,static,timing,zone}/<name>.lean`
     using the `primitive` command macro.

2. **Extract composite phrasings into `plugins_v3/builtin/templates/`:**
   - Move all composite phrasing helpers (`gainLife`, `loseLife`, `die`, `enter`, `leave`,
     `entersTapped`, `duringNextTurnOf`, `lookAndSortSlice`, etc.) into
     `plugins_v3/builtin/templates/{ability,ability_word,action,amount,condition,cost,determiner,duration,event,mana,noun,pile,predicate,pronoun,quantity,timing,zone}/<name>.lean`
     using the `template` command macro.
   - Omit manual `firstMention` / `laterMention` in `duringNextTurnOf` and `lookAndSortSlice`
     (handled automatically by `template` on multi-mentioned `NounPhrase` parameters).

3. **Enforce the `primitive` constructor boundary and delete `Templates.lean`:**
   - Enforce in `Authoring.lean` that non-`primitive` declarations (`template`, `action`, `keyword`,
     `counter`, `subtype`, `designation`, `card_type`, `relation`, `token`, `sba`, `conferral`,
     `card`) do not directly reference `Semantics.Primitives.*` constructors.
   - Delete `lean/Semantics/Templates.lean`; `lean/Semantics/Definitions.lean` now loads the entire
     builtin vocabulary via `load_bundle "../plugins_v3/builtin"`.

## Proof

- `lean/Semantics/Templates.lean` and all `lean/Semantics/Definitions/*.lean` submodules are deleted;
  every builtin `primitive`, `template`, `definition`, and rule row lives as an individual `.lean`
  file under `plugins_v3/builtin/{primitives,definitions,templates,rules}/`.
- Non-`primitive` declarations are gated against referencing raw `Semantics.Primitives.*` constructors.
- `cargo xtask lean check` (`lake build --wfail`) exits 0 with zero errors and zero warnings across
  `Semantics`, `SemanticsCards`, and `SemanticsProofs`.
