---
needs: [macro-parse-index]
---
The bidirectional typed-slot codec — the shared engine for filling `${i}` slots
in BOTH directions (this **supersedes the render-only `render-template-grammar-args`**,
whose scope it absorbs):

```
parse:  English fragment --codec.read--> arg RON
render: arg RON          --codec.show--> English
```

A per-type codec registry keyed on the param's **declared** type. Two
prerequisites fold in here:

1. **Typed params** — tighten the lazy `params: [Any]` on slot-bearing macros to
   their real type, so the slot type is read straight off `MacroDef.params` (no
   body-AST reflection, which `SupportsMacros` doesn't support). Filter:
   Protection/Enchant/Affinity/the event subjects. Cost: Ward/Equip/Fortify/
   Reconfigure/Flashback/Kicker. Int: Annihilator/Crew/Suspend/Pump. Each is a
   one-line edit, re-checked by the existing load-time validator. Register the
   missing `Cost`/`Int` param-type codecs (`Filter`/`Color` already exist).
2. **Codec-driven matching** — the matcher hands a slot's remaining input to
   `codec.read`, which consumes a maximal grammatically-valid arg and reports
   `consumed_len`; slots are bounded by validity, not regex greed (handles
   `Equip {2}. This ability costs…` without over-capturing the trailing clause).

Leaves reuse the existing typed readers/writers (`filter.rs` / `filter_noun`,
`cost::parse_cost` / cost renderer); recurse through `macro-parse-index` for
macro-backed sub-nodes. Render learns each arg's type by looking the macro up by
`Expansion.name` — this is what unblocks `render_arg` from integers-only and
closes the render direction (the old `render-template-grammar-args` scope,
including its `AsEnters` partial-template question: complete it to `"as ~ enters,
${0}"` or keep replacement framing structural). Structural rendering stays the
permanent floor for provenance-less hand-built `CardView`s.

## PROGRESS (in workspace ../macro-slot-codec, committed, NOT integrated)

**Prerequisite 1 (typed params) DONE + verified.** Added a `Cost` param-type
validator (`read_str::<Vec<CostComponent>>`) in `deckmaste_cards::macros`, and
retyped the slot-bearing macros `params: [Any]` → real types: `[Filter]` for
Protection/Enchant/Affinity + the 4 event subjects (Enters/Dies/Attacks/
Destroyed); `[Cost]` for Ward/Equip/Fortify/Reconfigure/Flashback/Kicker. Verified
by the load-time validator + `validate` (builtin 8 / canon 43 / wizards 3357, all
valid, 0 mismatch — every existing invocation re-expands clean under the tighter
types). Deferred within this prerequisite: `Int` slots (Crew/Pump) — they already
round-trip via `render_arg`'s integer path; retype + an `Int` validator when the
codec needs them.

**Parse direction (`read` / codec-driven matching) DONE + verified.**
`TemplateIndex::match_with(kind, input, slot_reader)` + `SlotMatch` in
`deckmaste_cards::template::index`: walks a slot-bearing pattern, matching
literals and filling each `${i}` via a `slot_reader(declared_type,
remaining_input) -> (arg_ron, consumed)` callback (keeps the index in
deckmaste_cards, slot readers in deckmaste_migrations — no circular dep).
`macro_template` now routes PARAMETERIZED keyword lines: nullary via `match_kind`,
parameterized via `match_with` with a `slot_reader` that dispatches on the
declared type (`Filter` → `keyword_ability::quality_filter`, `Cost` →
`cost_arg`). `Protection from black` → `Keyword(Protection(ColorIs(Black)))`,
`Ward {2}` → `Keyword(Ward([Mana([Generic(2)])]))` — same invocations the bespoke
parser emits, so it shadows keyword_ability for these with zero regression
(regen 3357 graduated, validate 0 mismatch). The irregular tail (em-dash costs,
"from everything", integer keywords, quality-word equip) still falls through to
keyword_ability.

**Render direction (`show`) — FILTER case DONE + verified.** `keyword_name`
(render/keyword.rs) now renders a PARAMETERIZED keyword via its filled `template`
(else the bare printed name — nullary keeps catalog casing for keyword-only
lines, gated on `has_args`); `render_arg` (render/template.rs) renders a `Filter`
arg by parsing the raw RON with the bare core reader and rendering its noun
(`filter_noun` extended for `ColorIs` colors via `effect::color_word`; declines
on `[unrendered]`). No `MacroSet` needed — type is recovered by trying the parse.
`Protection(ColorIs(Black))` → "protection from black", `Enchant(Type(Creature))`
→ "enchant creature". Filter is now FULLY BIDIRECTIONAL. Verified: render.rs 31,
canon 0 mismatch, validate wizards 3357 clean, 961 workspace tests, clippy clean.

**`Cost` render DONE.** `render_cost` (render/template.rs) renders a
`Vec<CostComponent>` to symbols (`Mana(mc)` → `card::mana_cost`, `Tap`→"{T}",
`Untap`→"{Q}"; declines on verb-costs → name fallback); `render_arg` tries it.
`Ward([Mana([Generic(2)])])` → "ward {2}", `Equip(...)` → "equip {3}". **Cost is
now FULLY BIDIRECTIONAL.**

**`Count` typed params DONE.** Added a `Count` validator
(`read_str::<Count>`); `PumpThisUntilEot` retyped `[Any, Any]` → `[Count, Count]`.
(Crew skipped — its body is an empty stub, param unused.) Verified: validate
builtin/wizards clean (3357).

**Codec COMPLETE for the keyword cases — Filter + Cost fully bidirectional,
typed params (Filter/Cost/Count), codec-driven parse matching, render-grammar-args
closed.** Deferred to `parse-params-via-macros` (a separate ticket, not this one):
extending the parse/render directions to EFFECT and FILTER kinds (sub-clausal,
not whole-line keyword). Minor open: `AsEnters` template-completion; an `Int`
slot reader in `macro_template` so a parameterized integer keyword line (none
currently macro-backed) could route. Ready to integrate.
