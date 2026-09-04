---
needs: []
---
**Refuse a distributive deed whose body mutates the enclosing stack.** Residue
of `workbench-loop-delta` (2026-09-04): the loop twin is closed by
`KeepsOuter`, but `Macros.exile (Macros.each Opponent) (Macros.It OneOf)`
followed by `untap (It OneOf)` is still admitted — `doesInstrIntro ManyOf` /
`distributedDelta` republish `nomIntro s` over a mutated stack. Guarding
`Enact` directly refuses the plurality-polymorphic macros at their definitions
(`exile`, `sacrifice`, `discard`, `puts`, `mills`, `scry`, `surveil`,
`shuffleInto`) because `nounPlur agent` is abstract there; the fix threads the
`KeepsOuter`-style obligation through that macro family.

Size: M. Done when: the sentence above is refused by a pin (non-vacuous), the
eight macros keep their surface, every bench use still typechecks, build at
its module count. Standard constraints apply.
