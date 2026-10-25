---
needs: []
---
# The otherwise-branch of `doIf` reads what its condition mentioned

Standard constraints apply.

`Action.doIf condition action (some otherwise)` steps `otherwise` from the context before the
condition (`withTargets`, `lean/Semantics/Check/Phrase.lean`, the `.doIf` arm of the action
stepper), so a referent the condition introduces is invisible to it: a `.laterMention` there of a
subject whose `.firstMention` is in the condition is refused `bindingIdentity`. The condition is
checked before either branch acts [CR#608.2c], so both branches may read it: "If its power is less
than 0, it gets −X/−0; otherwise, it gets +X/+0" should check with the subject introduced once, in
the condition.

Found in `lean-v3-rules-and-card-fidelity` §1 (change `vrlrtwru`): `getsScaledStats`
(`lean/Semantics/Definitions/Actions.lean`), the [CR#701.10c,701.11c] expansion of doubling and
tripling a power or toughness, had to be spelled as two complementary guarded changes per axis
(`doIf (… .less 0) …`, then `doIf (… .atLeast 0) …`) instead of one if/otherwise, only because of
this limitation.

Change: let the otherwise-branch read the condition's referents (keeping what it already reads:
the then-branch's announced targets and named amounts), with a pin that a condition-introduced
subject is read back in `otherwise`. Then respell `getsScaledStats` as one `doIf … (some …)` per
axis and update its pins (`negativePowerDoublesBelowZero`, `negativeAxisTriplesBelowZero`,
`Proofs/Numbers.lean`).
