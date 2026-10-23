---
needs: []
---
# A distributive "other" anchors on the whole group, not on each member

Standard constraints apply. Sits under the Contrast Anchor rules of ADR §0.1
(`docs/decisions/semantics-v3.md`) and the glossary entry **Contrast Anchor**, which this
ticket may need to amend; do not reword the entry to fit code.

## Defect (verified 2026-10-21)

*Hit // Run*, Run: "Attacking creatures you control get +1/+0 until end of turn for each other
attacking creature." Each affected creature gets +1/+0 per attacking creature **other than
itself**. The checker anchors "other" on the group the subject introduced:

```lean
get (allOf (and [isAttacking, isCreature, youControl]))
    (up (forEach (.lit 1) (and [isAttacking, isCreature, isOther]))) (up (.lit 0))
    (some untilEndOfTurn)
-- Action.check = []; contrastAnchors = [mention 1], the mention (determiner all, creature)
```

"Other than the group of attackers you control" excludes every one of those attackers, so as
written the count would be of attackers you do not control, which is wrong: a lone attacker
should get +0, and with two attackers each should get +1. The set of affected creatures is fixed
when the effect begins ([CR#611.2c]), but the amount is a per-member function of that set.
Run is not authored in `Cards/`; the probe above is the only encoding so far.

## Ruling (owner, 2026-10-22, batch 4): approved

When "(for) each other X" sits in a clause whose subject is a distributive group, anchor "other" on
that group's **current member**. This becomes the **first step** of the Contrast Anchor order: it
precedes the existing step 1 (nearest earlier compatible mention) of ADR §0.1, and the glossary
entry **Contrast Anchor** and ADR §0.1 are amended in the same landing.

## Fix (approved; the representation is the implementer's)

- A "for each other X" (or "each other X") inside a clause whose subject is a distributive
  group ("each", "all", a bare plural over a group that a per-member amount mentions) takes the
  distributive reading: the anchor is the current member of that group, not the group.
- Add this as the first step, ahead of ADR §0.1's step 1 (nearest earlier compatible mention), so a
  group subject in the same clause as the "other" amount is read per member. Where the "other"
  phrase is in an unrelated clause, step 1 is unchanged.
- Representation is the implementer's choice: an anchor variant that says "member of mention
  N", or evaluating the `Amount` once per member. The Core AST gains a distinction only if the CR
  or anaphora resolution needs it (owner, 2026-10-21; refined 2026-10-22 as `semantics-v3.md`
  §0.1 item 5); here the checker and the later engine both need to know which reading, so a
  variant is expected. Escalate if it is not.
- If a card needs an anchor the amended steps do not pick, STOP for an owner ruling (ADR §0.1).

## Change

1. The checker: in the clause-level anchor resolution, when the clause's subject is a distributive
   group (see above) and the "other" phrase is inside that clause, the anchor is "member of
   mention N" (the subject's current member), checked before the nearest-earlier-mention step.
   Add the anchor variant (or per-member evaluation) in `Abilities.lean` and its checker arm.
2. Author *Hit // Run*, Run, in `Cards/` (see the Defect for the encoding that is wrong today).
3. Amend the glossary entry **Contrast Anchor** (`docs/contexts/game-model/CONTEXT.md:774`) and
   ADR §0.1 (`docs/decisions/semantics-v3.md`) to list the distributive-member step first, citing
   [CR#611.2c] for the fixed set with a per-member amount. Amend the entry's words; do not
   reword other entries to fit.

## Tests to add (in `Proofs/ContrastAnchor.lean`)

- Run, authored as a `Cards/` bench item: the anchor resolved is the member, not the group
  mention. Pin the group-vs-member distinction by name.
- `distributiveStepPrecedesNearestMention`: with an earlier compatible mention in scope and a
  distributive subject in the same clause, the anchor is still the subject's member (the new step
  is first in the order).
- Negative pins so the new step does not over-fire: a "for each other" in a clause whose subject
  is no group keeps its current anchor. Existing pins (Contractual Safeguard, War Screecher,
  Bartz and Boko) pass unchanged.
- Update the glossary **Contrast Anchor** entry and ADR §0.1 in the same landing, with the CR
  citation.

Gates: `./lean/scripts/build`, `cargo xtask lean check`, the CR citation gates.
