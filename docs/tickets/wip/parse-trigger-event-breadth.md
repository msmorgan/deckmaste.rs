---
needs: []
---
The triggered-ability parser graduates only a narrow set of events (e.g.
`ThisEnters`, which Elvish Visionary uses). Three trigger clauses in the
Goblins/Elves demo corpus decline, leaving their cards as `.ron.todo`:

1. `Whenever ~ attacks, …` — attacker trigger (Dwynen Gilt-Leaf Daen, Goblin
   Piledriver, Goblin Rabblemaster).
2. `Whenever you cast a[n] <Subtype> spell, you may …` — cast trigger filtered to
   a spell subtype, carrying a `you may` rider (Lys Alana Huntmaster).
3. `At the beginning of combat on your turn, …` — phase/step-entry trigger
   (Goblin Rabblemaster).

The effects these clauses carry also decline:
- `for each attacking <X>` — a `CountOf` over an attacking-state filter (Dwynen
  lifegain; Piledriver/Rabblemaster self-pump). `for each <Subtype>` already
  parses (gen-dynamic-count); the new piece is the `attacking` qualifier.
- `you may …` — wrap the effect in a `May` frame.

Scope is PARSING only: emit the existing `Triggered(event: …)` / `CountOf` /
`May` AST in `crates/deckmaste_migrations/src/parsers/` (triggered + effect).
The engine already enumerates `attacks` and `cast` facts (done:
engine-trigger-events); live execution of the new pieces rides
engine-filter-breadth (attacking `StateFilter`) and engine-resolve-effects
(`May`). Verify with a `cargo xtask generate` delta: Dwynen, Goblin Piledriver,
Goblin Rabblemaster, and Lys Alana Huntmaster graduate with the right
trigger/effect shapes. Part of demo-goblins-elves (bench cards).
