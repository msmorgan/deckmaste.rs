---
needs: []
---
**Add a `Pro` read at kind `TurnRef` so turn possessives are nouns.**
Cleanroom review 2026-09-03, R4, ruled: turn possessives become nouns. This
is the small half of the possessor fold and lands first —
`workbench-possessor-noun` needs it.

`Owner.ThatTurns` (`Words.idr:3829–3832`) is a fused word gated by
`TurnDeixis` (`Triggers.idr:588`) over a `TurnRef` binding that `ExtraTurn`
already mints (`Effect.idr:1897`). What is missing is only the *read*: `Noun
bs TurnRef` has no constructor, so "that turn's" cannot be spelled
compositionally and must stay a word.

## The ruling

The binding exists; add the read. A `Pro (Reach) (Plurality)` read at kind
`TurnRef` — the same counted-uniqueness idiom as every other pronoun read, no
recency and no ranking — makes "that turn's upkeep" a `Noun bs TurnRef` and
lets `TurnDeixis` retire. Turn possessives are nouns, not a word class.

Scope here is the read plus its gate, and re-spelling whatever bench sites
spell `ThatTurns` today. Deleting `Owner` itself, and re-typing
`HeaderPossessor` / `DurationEnd` / `TriggerWindow` / `Timing` /
`OnlyDuring`, is `workbench-possessor-noun`.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; `Noun bs TurnRef`
has a `Pro` read with the `= 1` count gate; "that turn's" is spellable as that
read over the `ExtraTurn` binding and is a typechecking bench witness;
`TurnDeixis` is gone or reduced to the generic read's gate; a pin refutes the
read where no turn binding is in scope, and it is non-vacuous. Standard
constraints apply.
