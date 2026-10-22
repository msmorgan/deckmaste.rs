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

## Resolution

**Owner's ruling (round 6, 2026-10-22):** proceed with this ticket; re-measure
first, since `english-v4-set-off-first-conjunct` and the sixth
canonicalization had landed since it was written, then supply any missing
main-Clause analysis structurally.

Re-measured on `suupnrpm`, the analysis is already present and the trapped one
is gone. `kzzovunt` (`english-v4-set-off-first-conjunct`, under the owner's
round-6 ruling recorded there) closed a preposition's Clause Complement
against a comma-set post-adjunct (`Summary::post_adjunct`; the
`PresentComplementClause` Production is `undelimited`). That removed
*[this Vehicle becomes crewed for the first time each turn, until end of
turn]* as the Complement of *whenever*, which was the first candidate cause
above in another form. Nothing was blocking the main-Clause analysis. The
comma-fronted Preposition Phrase Adjunct on `Clause` stacks: *[Whenever …],
[[until end of turn], [this Vehicle becomes …, except …]]*. The sixth
canonicalization puts *, except …* inside the duration's host Clause.

All 16 Readings of the unit now have exactly that trigger Complement and
*until end of turn* as a preposed Adjunct of the main Clause. The trapped
Reading is not a separate defect to remove: none is left. The other
alternations are retained ambiguity, outside this ticket: adjectival or
verbal *crewed*, the host of *this turn*, the *except* scope tree, and the
attachment of *in addition to its other types*. No grammar or engine change
was needed. The landing adds the `+` judgment in
`judgments/trigger_clauses.txt`, asserting the trigger Complement by span,
*until end of turn* as an Adjunct of *become*, the fronted Clause's span, and
`!Clause "this Vehicle becomes crewed for the first time each turn, until
end of turn"`.

1. **Census delta** (`suupnrpm`, full corpus): faces read 28,799 → 28,799;
   total Readings (product) 214,300 → 214,300 and Surplus 88,560 → 88,560;
   every unit unchanged, Mindlink Mech at 16. Cheapest tier: singleton in
   14,800 of 16,562 ambiguous units, summed 18,929, largest 8, unchanged.
   Lost faces: none.
2. **Judgment set** (`suupnrpm`): 3,029 enforced, 207 pending; all hold.
