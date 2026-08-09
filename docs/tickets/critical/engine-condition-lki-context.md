---
needs: [engine-reference-resolution-snapshot-channel]
---
**Engine: make `Condition::Matches` explicit about whether it asks for current
state or last-known information.** Its current fallback applies any predicate
to a bound snapshot when the reference is gone. That is correct for trigger
conditions that inspect a departed event object, but wrong for a current-state
guard such as `Permanent`: a snapshot whose `left` zone was the battlefield is
treated as a permanent after it has left.

Introduce a context/policy in the reference or condition evaluation path so
only LKI-entitled reads consume the snapshot. Current-state membership and
action preconditions must return false for a departed object. Do not solve this
by disabling snapshot fallback globally; dies/intervening-if predicates still
need it.

Foundations witness: Affectionate Indrik. If it leaves before its ETB fight
trigger resolves, the Fight composite's both-creatures guard must fail, no
power count is evaluated, and no damage/fight event occurs. Also retain a
positive snapshot-condition test for a dies trigger so the fix proves both
sides of the distinction.
