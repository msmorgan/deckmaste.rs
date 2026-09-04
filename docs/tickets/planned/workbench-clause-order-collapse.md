---
needs: []
---
**Delete the clause-order witnesses: `Continuously` and `Conditionally` fix
static-first, the word-order macros re-thread the other order, and the bench
carries no implicit handles at all.** Fresh workbench review 2026-09-03, F13,
resolved by ruling.

**Ruling (settled 2026-09-03): clause order is not a slot.** A static clause
and its condition are threaded in one fixed order —
`Effect.StaticEffect.Conditionally` and `Effect.Continuously` thread
static-first — and the printed orders that put the condition first are spelled
by the word-order macros (`throughout`, `onlyWhile`), which re-thread on the
way in. `Effect.StaticThreads` (`Effect.idr:320–323`: `CondFirstDone`,
`StaticFirstDone`), `Effect.SpanStaticThreads` and their `%hint`/`[noHints]`
pairs go. `Effect.OnlyIf`/`Effect.If` are already the constructor mechanism
and stay as they are; it is the witness mechanism that is being removed.

The bench pays for the witnesses in visible handles: `{ts = StaticFirstDone}`
appears 127 times across twelve `Cards/*.idr` families (Damage 24, Choice 19,
Deontic 18, Static 15, Cost 15, Mana 13, Faces 8, Keyword 5, Turn 4, Counters
3, Trigger 2, Copy 1). All 127 collapse to the bare constructor row.

**The braces lint (ruling 11).** With `{ts}` gone, finish the job: the 35
`{bs = …}` annotations disappear once the witness types they disambiguate
carry concrete indices, and the remaining one-offs — `{gm}` 6, `{wf}` 3,
`{nz}` 3, `{st}` 2, `{k}` 1 — go through wrapping macros per
`docs/decisions/card-authoring-binds-no-implicits.md`. Then add a one-line
lint to `idris/scripts/build` that refuses any `{name = ` in
`src/Experimental/Cards/*.idr`, the way that ADR's `grep -cE` already binds
`Cards.idr`. `Proofs*.idr` are exempt — a pin's whole subject is the proof
term it refuses.

Size: S–M for the witness removal; the bench edit is mechanical and large.

Done when: `StaticThreads`, `SpanStaticThreads` and every `%hint`/`[noHints]`
pair for them are gone from the tree; `grep -E '\{[a-zA-Z_]+ *= '
idris/src/Experimental/Cards/*.idr` is empty and the build script fails when
it is not; a condition-first printed sentence (`onlyWhile`) and a
static-first one are both typechecking bench witnesses; the `ProofsDeontic`
pins that bound `{ts = StaticFirstDone}` are re-spelled against the bare row
and still refute, probed non-vacuous; the build is 44/44 with 0 errors and 0
warnings. Standard constraints apply, plus the RON-shaped constraint: a core
constructor is admissible only if the RON re-emitter can produce it from a RON
node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
