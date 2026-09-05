---
needs: [workbench-attack-agent-role]
---
**Spell pronouns and demonstratives as printed: `It`/`Them`, `That`/`Those`.**
Ruling 2026-09-04, correcting a misreading of "one macro per lemma, no
agreement or arity pairs": that rule forbids macro pairs that encode forms of
ONE printed word (verb agreement, arities); it never meant that two distinct
printed words share a macro with a `Plurality` argument. References are
written as on the card, so `Macros.It ManyOf` (73 sites) is `Macros.Them`,
`Macros.That w ManyOf` (20 sites) is `Macros.Those w`, and every other macro
whose `Plurality` argument stands for a distinct printed word (`ItVerbed`,
and any the sweep finds) splits the same way; a macro whose plurality is not
a separate printed word (`They`) stays one. The core `Pro`/`Noun` rows keep
their positional `Plurality`; only the macro surface changes, and the RON
re-emitter maps both spellings onto its pronoun macro with number.

Size: S (mechanical sweep, 357 `It`/`That` sites plus pins). Done when: no
macro in `Macros.idr` takes a `Plurality` for a printed-word split; every
site spells the printed word; build at its module count. Standard
constraints apply, including the RON-shaped constraint.
