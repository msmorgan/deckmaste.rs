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

## Done

`ChooseTargets::resolve` (`decide/pending/cast.rs`) now skips the whole
Must(Target) enforcement block when `g.placing_trigger.is_some()` — that
staging slot is only occupied while a triggered ability chooses its
placement-time targets ([CR#603.3,603.3d]), an orthogonal discriminator from
the `by` filter's `ObjectKind::Ability` collapse (untouched, still matches
both activated and triggered abilities elsewhere). Verified the rules premise
against `data/rules/cr.json`: [CR#601.2c,602.2a] scope the Flagbearer wording
to casting/activating, while [CR#603.3d] describes a trigger's target choice as
happening while it's "put on the stack" — a distinct action the wording never
names, confirming the exemption.

The exemption is the printed wording's, not the rules': [CR#603.3d] routes
placement through [CR#601.2c] including its must-target obligation clause, so
the wholesale skip holds only while every printed must-target effect carries
the casting/activating scope. Standard Bearer is the corpus's only
`Must(Target(...))` row; the others are `Must(Attack)`/`Must(Block)`.

Extended `flagbearer_*` in `deckmaste_engine/tests/stack.rs`:
`flagbearer_constrains_opposing_target_choice` and
`flagbearer_does_not_constrain_its_controllers_spells` still pass unmodified;
added `flagbearer_does_not_constrain_a_triggered_abilitys_target_choice`
(P1's own bolt kills P0's Footlight Fiend so the killing spell itself isn't
flagbearer-bound; the fiend's dies-trigger then legally targets past P1's
able Standard Bearer).
