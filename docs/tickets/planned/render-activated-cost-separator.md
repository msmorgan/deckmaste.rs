---
needs: []
---
**An activated/keyword ability's cost list renders with no separator between
cost components — `{S}{T}:` instead of the printed `{S}, {T}:` — in the
oracle-text RENDER, failing fidelity for ~576 cards.** Multi-component costs
(a mana symbol plus `{T}`, or two mana groups, or mana + a life/sacrifice cost)
lose the `, ` that MTG prints between an ability's cost components ([CR#602.1]).

This is the RENDER side (core → English oracle text), **distinct from
`parse-mana-cost-comma-spacing`** (which is the parser emitting `Mana([Red,Red])`
into RON with no inner space — a `.todo`/RON-serialization concern, not the
rendered card face).

Source: `crates/deckmaste_cards/src/render/ability.rs:265` renders the line as
`format!("{cost}: {body}{rider}")` (`:268`) where `cost = effect::activated_cost(&a.cost.0, &ctx)`
(`render/effect.rs`). `activated_cost` joins the cost components without `", "`,
so a two-component cost comes out crammed. Fix: join the rendered cost
components with `", "` (MTG's printed convention between cost components), while
keeping a single mana group's symbols contiguous (`{1}{W}` stays `{1}{W}`; the
comma separates *components*, e.g. `{1}{W}, {T}` and `{T}, Sacrifice a creature`).

Surfaced by `parse-look-at-top`/`canon-delver`: Frost Augur graduated far enough
to be fidelity-checked and its effect body is byte-identical to oracle — the ONLY
diff is `{S}, {T}:` rendering as `{S}{T}:`.

Verify: Frost Augur (and the ~576 affected cards — e.g. Frostbridge Guard
`{2}{W}, {T}`) render `…, {T}:`; check `cargo xtask fidelity plugins/wizards`
failing-count DROPS by the affected set (not just Frost Augur); `cargo test
--workspace` green; a single mana group is unaffected (`{1}{W}` unchanged).
