# workbench-attacking-defender-attributive

The attributive combat-role predicate — "attacking creature", "defending
player", "blocked creature" as noun restrictions rather than event headers.
Measured by choice-C (done 2026-08-27) at **94 supported lines**, with real
design questions (role-during-combat is a state read; which roles, which
carriers, interaction with `Attacks`' defender slot). The last blocker on
Beckoning Will-o'-Wisp and Triarch Stalker (whose choosers landed in
choice-C), and a dependency of many combat bundles. Re-verify the count with
`jq 'select(.supported)'` at claim; design before building — this is not a
one-cell round.

Routed from description-2 (close, 2026-08-28): the ATTACKER-voice predicate
("the player who attacked that player" shape — Namor's 4 lines) joins this
family's design; `AttackedBy` (description-1) covers the attacked side.

Routed from damage-prevention (close, 2026-08-28): the `Unblocked` predicate
("[creature] is unblocked" as a description) — Forcefield's last blocker;
combat-role description, this family's design.
