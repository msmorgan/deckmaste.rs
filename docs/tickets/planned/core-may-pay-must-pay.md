---
needs: []
---
**Core: `MayPay` / `MustPay` resolution-time payment effects, superseding
`Unless`.** From the 2026-06-28 idris↔rust model audit.

Today the only resolution-time payment branch is `Effect::Unless { effect, who,
unless: Vec<CostComponent> }` (`crates/deckmaste_core/src/effect.rs`): "[do]
unless [who pays]". It takes a bare component list (not the full `Cost` algebra)
and has no "may pay → if you do / if you don't" branch.

Idris splits this into two effects (`idris/src/Core.idr`):
- `MayPay {actor} Cost (and_then) {or_else}` — "[actor] may pay [cost]; if they
  do → `and_then`, else → `or_else`" (a resolution-time kicker).
- `MustPay {actor} Cost (or_else)` — "[actor] must pay [cost], or else
  `or_else`" — the Mana Leak "counter target spell unless its controller pays
  {N}" punisher, which **supersedes** `Unless`.

Both take a full `Cost`.

Adoption: add `MayPay`/`MustPay` over `Cost`; migrate `Unless` to `MustPay` (or
keep `Unless` as sugar over it).

Verdict: **improvement** (full `Cost` algebra vs a bare component list; the
may-pay→branch `Unless` can't express). Effort: **M**. Related:
`idris-effects-costs-and-choices` (done/) landed the Idris side; `core-loyalty-costs`
/ `core-alt-costs` are adjacent cost work.
