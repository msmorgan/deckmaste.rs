---
needs: []
---
**Engine: a `DeedAgent` `by`-filter can't tell a triggered ability from an
activated one.**

Standard Bearer's Flagbearer requirement applies "while an opponent is
choosing targets as part of casting a spell they control or activating an
ability they control" — scoped to announcing a spell's targets ([CR#601.2c])
or activating an ability ([CR#602.2a]). A triggered ability's targets are
chosen when it is put on the stack ([CR#603.3]), a moment the wording never
names, so triggered abilities are exempt.

The engine enforces `Must(Target)` rows in `ChooseTargets::resolve`
(`decide/pending/cast.rs`) by matching the row's `by: DeedAgent` filter
against the targeting stack object. But the only stack-object-kind
discriminator reachable from `Predicate` is `Predicate::Kind(ObjectKind)`, and
`object_kind` (`target.rs`) deliberately collapses both ability kinds into one
value — `StackObject::Triggered { .. } | StackObject::Activated { .. } =>
ObjectKind::Ability`. That collapse is load-bearing elsewhere (it is what lets
a filter match "target activated or triggered ability" for Stifle-style
effects), so it must not change. There is simply no way to author "an
activated ability but not a triggered one", and the site panics rather than
exempting the trigger.

Fix: add a discriminator orthogonal to the existing collapse — an arm
distinguishing `StackObject::Triggered` from `StackObject::Activated`, both
already structurally available on the stack entry — and consult it in the
`must_rows` loop instead of panicking when a placing trigger is in flight.
Leave what `Kind(Ability)` matches untouched.

Done when a `Must(Target)` row that doesn't specifically require triggered
abilities stops applying to a trigger's target choice, with the `flagbearer_*`
engine tests extended: casting past an able Flagbearer stays illegal, and a
triggered ability targeting past one becomes legal.

Effort: **S**.
