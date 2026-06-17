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
