---
needs: [english-v4-except-clause-coordination]
---
# A first conjunct that ends in a comma-set post-dependent

## Witnesses

- **Aspect of Wolf** (deferred under this ticket): *Enchanted creature gets
  +X/+Y, where X is half the number of Forests you control, rounded down, and Y
  is half the number of Forests you control, rounded up.* The intended analysis
  is *where* [[X is …, rounded down], and [Y is …, rounded up]]: the
  coordination of two Participial-Supplemented Clauses is the Complement of
  *where*. Its judgment (`slash_measure_selection.txt`) is pending while it is
  deferred.
- **The pending `where_variable_clause.txt` entry** *Search target player's
  library for X cards, where X is the number of cards in your hand, and exile
  them. Then that player shuffles.* The intended analysis is [[Search … for X
  cards, where X is …], and [exile them]]: the *where* phrase is a Supplement of
  the first conjunct (`docs/oracle-style-guide.md` §3, "around a trailing
  variable definition"). It has 0 Derivations.

Both moved here from `english-v4-except-clause-coordination` (owner ruling,
round 4).

## Cause

`step.rs` sets `set_off` on any Production whose second child is a comma, and a
`set_off` constituent is never a first conjunct. `set_off` exists for a
comma-fronted dependent (*When …, …*), which scopes over a whole coordination.
A host-first [head, `,`, post-dependent] Production (a comma-set post-adjunct,
Supplement or `ParticipialSupplement`) also has a comma second, so it is marked
too. `Summary::fronted` (head last, after a comma) is the narrower fact.

## Measured data (Experiment A, on `kmntwkws`)

Using `fronted` in place of `set_off` for the first-conjunct check, and removing
`undelimited` from both `FiniteComplementClause` and `PresentComplementClause`:
product 218,493 → 222,194, Surplus 91,278 → 93,319, faces read 28,702 →
28,705, 88 units rose, none emptied.

- Aspect of Wolf 0 → 28 Readings, only 4 of them intended. The *Search* entry
  reads with 3, all intended.
- The `set_off` half alone admits [C, Post] as a first conjunct everywhere:
  [[C, except A] and B] (Ember Island Production 6 → 12, Mockingbird 702 →
  1,404, Irenicus's Vile Duplication 3 → 6) and cost coordinations (Skeleton
  Shard, Pearl Shard: *{3}, {T} or {B}, {T}*).
- (The `undelimited` half, *for as long as [A, and B]* and *if [A, and B]*, is
  settled by `english-v4-except-clause-coordination`: only a comma-set
  Preposition Phrase takes a delimited Clause coordination.)

Aspect of Wolf's wrong Readings have three shapes:

- **(a)** [[C, where X is …, rounded down], and [Y is …, rounded up]]: the
  coordination outside *where*;
- **(b)** *where* [[X is …, rounded down, and Y is …], rounded up]: *rounded up*
  a Supplement of the whole coordination;
- **(c)** [[C, where X is half N], rounded down]: *rounded down* outside the
  *where* phrase.

## Open question (needs an owner ruling)

The *Search* analysis has the same shape as (a) and the Ember Island rise. They
differ only in whether the following conjunct could instead continue the
comma-set dependent's Clause Complement: *exile them* is Plain and cannot
coordinate inside *where*'s present-tense Complement; *Y is …* and *it's a 4/4
Hero* can. Telling them apart is a right-attachment rule: a comma-set
dependent's Clause Complement takes the following coordination tail, or a
following comma-set Participial Supplement, when it can. The chart cannot
decide that compositionally, and an approximation on `VerbForm` comes close to
policing. **STOP for an owner ruling on that rule before implementing.**

Aspect of Wolf also needs rulings on:

1. whether *rounded up* as a Supplement of a whole coordination (shape (b)) is
   admissible, since the Ranking Key prefers whole-coordination attachment
   (`docs/decisions/english-v4.md` §4.3);
2. the stacking order of shape (c), [[C, where X], rounded down] against
   [C, where [X, rounded down]].

## Owner rulings

**Round 5, item 7a (verbatim):** "set-off-first-conjunct: NO VerbForm
carve-out. Replace set_off with fronted on the first-conjunct check; accept [C,
except A] and B and C, except [A and B] as both valid scope trees and let the
ranking key rank C, except [A and B] first. On Aspect of Wolf keep only the
structural canonicalizations: uncoordinated: ["head"] on ParticipialSupplement
(`, rounded up` doesn't wrap a coordination whose first conjunct already has `,
rounded down`), and ParticipialSupplement ordered inside Supplement ([C, where
[X, rounded down]] not [[C, where X], rounded down])."

**Round 6 (verbatim):** "Go with (A): Close a preposition's Clause complement
(when, whenever, if, etc.) against comma-set post-adjuncts (OSG §8, checklist
4; [CR#603.1,603.4]), keeping ParticipialSupplement `, rounded down` admissible
inside `, where …` for Aspect of Wolf. If any face is lost when measured, bring
it back. Confirmed: leave the 5 copy-except units un-tuned, and leave the
*Search … Then that player shuffles* judgment pending for Stratum 7 (Then)."

These rulings answer the open question above and supersede the Done-when below
where they differ. Rises from the newly admitted scope trees are allowed.
The 5 un-tuned copy-*except* units are tracked in
`english-v4-ranking-residuals`. Shape (a) is kept as a scope tree. The
*Search* entry stays pending until sentence-initial *Then* reads (Stratum 7).

## Done when

- Aspect of Wolf reads with its intended analysis among its Readings and
  cheapest; its deferral is removed; its pending judgment holds, and a `+`
  judgment asserts `Clause "X is half the number of Forests you control,
  rounded down, and Y is half the number of Forests you control, rounded up"`.
- No Reading puts a comma-set post-adjunct inside a preposition's Clause
  Complement (an intervening *if* inside *when …*).
- No new feature in `english.ron`. If the fix needs one, STOP and return to the
  owner.
- No face is lost.

Standard constraints apply.

1. **Census delta** (`kzzovunt`, full corpus): faces read 28,798 → 28,799
   (Aspect of Wolf); total Readings (product) 219,746 → 216,788 and Surplus
   92,028 → 90,205; 37 units rose (29 copy *except* scope trees, 5 cost
   alternatives, 3 *where* coordinations including Aspect of Wolf) and 869 fell
   (782 trigger conditions and 33 other preposition Complements lost a
   comma-set post-adjunct inside them; 54 fronted dependents no longer head a
   first conjunct). Cheapest tier: singleton in 14,383 of 16,960 ambiguous
   units, summed 20,250, largest 8. Lost faces: none.
2. **Judgment set** (`kzzovunt`): 3,021 enforced, 205 pending; all hold.
