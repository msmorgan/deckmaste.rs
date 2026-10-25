---
needs: []
---
# The otherwise-branch of `doIf` reads what its condition mentioned

Standard constraints apply.

`Action.doIf condition action (some otherwise)` steps `otherwise` from the context before the
condition (`withTargets`, `lean/Semantics/Check/Phrase.lean`, the `.doIf` arm of the action
stepper), so a referent the condition introduces is invisible to it: a `.laterMention` there of a
subject whose `.firstMention` is in the condition is refused `bindingIdentity`. The condition is
checked before either branch acts [CR#608.2c], so both branches may read it: a condition that
introduces its subject and two branches that both read it back ("if it's a Knight, it gets
+2/+2; otherwise, it gets +1/+1", the subject written once, in the condition) should check.

Change: let the otherwise-branch read the condition's referents (keeping what it already reads:
the then-branch's announced targets and named amounts), with a pin that a condition-introduced
subject is read back in `otherwise`.
