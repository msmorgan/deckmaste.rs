---
needs: []
---
# A comma-set duration after a trigger is the main Clause's preposed Adjunct

## Witness

- **Mindlink Mech:** *Whenever this Vehicle becomes crewed for the first time
  each turn, until end of turn, this Vehicle becomes a copy of target
  nonlegendary creature that crewed it this turn, except it's 4/3, it's a
  Vehicle artifact in addition to its other types, and it has flying.*

On `lxzvuwqu` (`english-v4-bare-relative-fronted-adjunct`) the unit has 16
Readings. Every one of them traps *until end of turn* inside the trigger: it is
a comma-set post-adjunct of the trigger's Clause, *[this Vehicle becomes crewed
for the first time each turn, until end of turn]*. No Reading makes it a
preposed Adjunct of the main Clause. The face counts as read only through wrong
Readings. (Before `lxzvuwqu` it also had 42 Readings with a bare relative
opening with a preposed adjunct, *the first time [each turn, until end of turn,
this Vehicle becomes …]*. Those are gone.)

## Intended analysis

*Whenever …* and *until end of turn* are both preposed Adjuncts of the main
Clause *this Vehicle becomes a copy …*, each set off by its comma. The duration
modifies the effect, not the trigger event:

- A triggered ability is [trigger condition], [effect] (`docs/oracle-style-guide.md`
  checklist 4; §8, Ability architecture; [CR#603.1]).
- A duration belongs to a temporary change, such as this copy effect, not to
  the trigger event (§11, Continuous and bounded durations: "Do not omit a
  duration from a temporary … color/type change").

## Change

- Find why the main-Clause analysis is missing. Candidates:
  - the sixth canonicalization (`english-lexical-analysis.md` §2): a host of a
    comma-set post-dependent never begins with a comma-fronted dependent, and
    *Whenever …* is fronted;
  - a restriction on stacking two comma-fronted dependents on one Clause;
  - a Production that stops the trigger Preposition Phrase from ending before
    a second fronted dependent.
- Admit *[Whenever …], [until end of turn], [Clause]* structurally.
- Then decide by measurement whether *until end of turn* inside the trigger is
  a separate wrong Reading to remove. Its comma sits between the trigger and
  the effect.

## Done when

- Mindlink Mech reads with *until end of turn* as a preposed Adjunct of the
  main Clause. Both Clauses are headed by *become*, so the `+` judgment asserts
  it by span: `Clause "this Vehicle becomes crewed for the first time each
  turn" as Complement of whenever`. Today that Complement is always *…, until
  end of turn*.
- **No new features in `english.ron`.** If the fix needs one, STOP and return
  to the owner.
- No face is lost, and no unit other than the witness gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
