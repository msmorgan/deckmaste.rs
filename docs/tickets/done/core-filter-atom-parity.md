---
needs: []
---
**Core: filter/condition atom parity — `Teammate` (team-relative players),
`IsAbility` object kind, and `TurnOf` over any player.** From the 2026-06-28
idris↔rust model audit. Three small grammar atoms the Rust model lacks
(`crates/deckmaste_core/src/filter.rs`, `condition.rs`):

- **Team-relative players.** Rust has `RelationFilter::OpponentOf(Filter)` but no
  `Teammate` / "your team". `Opponent` is *not* `Not (SameAs You)` — in
  Two-Headed Giant a teammate is neither you nor an opponent. Idris carries
  `Opponent` and `Teammate` as primitives ("your team" = `Or [SameAs You,
  Teammate]`).
- **`IsAbility` object kind.** Rust `ObjectKind` has `Player` but no `Ability`, so
  an activated/triggered ability on the stack can't be filtered ("counter target
  activated ability"). Idris `ObjectKind.IsAbility`.
- **`TurnOf` over a player predicate.** Rust has the bare `Condition::YourTurn`;
  Idris `Condition.TurnOf (Predicate APlayer)` generalizes it ("during an
  opponent's turn"; `yourTurn = TurnOf (SameAs You)`).

Adoption: add `Teammate` (with team semantics), an `Ability` object kind, and a
`TurnOf`-over-predicate condition (keep `YourTurn` as sugar).

Verdict: **improvement** (small, clean expressivity wins; team-relative is a
soundness fix for multiplayer / Two-Headed Giant). Effort: **S–M**. Related:
NONE.
