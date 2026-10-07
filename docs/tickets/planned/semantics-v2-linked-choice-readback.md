---
needs: []
---
**A player chosen while paying a cost cannot be read back by the ability
linked to it.** Found by the stopped `plugins-v2-gift-variants` attempt
(2026-10-05). Standard constraints apply. The device was decided 2026-10-06:
option 1, the paid-cost facet (below).

## Evidence

Gift's first ability is "As an additional cost to cast this spell, you may
choose an opponent" [CR#702.174a]; its second hands the effect to "the chosen
player" [CR#702.174d..702.174i]. The rules link the two: "If an object has an
ability printed on it that causes a player to 'choose a [value]' and an
ability printed on it that refers to 'the chosen [value],' 'the last chosen
[value],' or similar, those abilities are linked. The second ability refers
only to a choice made as a result of the first ability." [CR#607.2d]

In the model the second ability writes `chosenPlayer`
(`lean/Semantics/Macros.lean` ~L218, `.chosenPlayer .theChoice`), which
reads a choice made in the same ability. The opponent chosen during casting
is not in that scope, so the checker refuses it as `choiceRef theChoice
player 0` (`lean/Semantics/Check/Refusal.lean`: no choice found). Nothing
reads a player back from a paid cost: `PaidFacet`
(`lean/Semantics/Words.lean` ~L361) has only `colorsSpent`,
`manaValueSpent`, `timesPaid` and `readback` (whether a named cost was paid).
So gift's second ability, on a permanent or a spell, has no faithful way to
name its performer.

## The options put to the owner

1. **A paid-cost facet**: a `PaidFacet` (or `PaidCostName`-keyed reference)
   for "the player chosen as [named cost] was paid", read by the second ability
   through the same `byKeyword "Gift"` name `readback` already uses for "its
   gift cost was paid". Narrow; fits gift exactly.
2. **A linked-ability binding** [CR#607.1,607.2d]: the first ability publishes
   its choice under a link the keyword definition declares, and the second
   ability's `chosenPlayer` resolves through that link rather than its own
   scope. General; also serves other choose/"the chosen" pairs printed as two
   abilities.

Either way: a pin where gift's second ability reads the chosen opponent, and
a refusal where an unlinked ability reads `chosenPlayer` with no choice in
scope. Related engine-side store: `engine-linked-abilities` (v1 engine
`Reference::Linked` resolution), not a prerequisite here.

## Decided 2026-10-06

**Option 1, the paid-cost facet**, because that is how the rules link the two
abilities (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and
the additional-cost keywords). Owner: "idk do what the CR says."

- Gift's first ability is a cost: "As an additional cost to cast this spell,
  you may choose an opponent." [CR#702.174a]. A printed ability that allows
  an additional cost and one that refers to whether it was paid are linked
  [CR#607.2i], as kicker's are [CR#702.33e]; a "choose" and "the chosen" pair
  is linked too [CR#607.2d].
- `PaidFacet` (`lean/Semantics/Words.lean`, `inductive PaidFacet`) gains a
  facet "the player chosen as [named cost] was paid", keyed by `PaidCostName`
  (same file) and read through `byKeyword` as `readback` already is for "its
  gift cost was paid". Mirror it in Rust and RON in the same landing.
- Option 2, a general linked-ability binding, is not needed for gift. The
  enters-time "choose X / the chosen X" pairs are a different family and stay
  with `engine-linked-abilities`.
- Pins as above: gift's second ability reads the chosen opponent through the
  facet, and an unlinked `chosenPlayer` with no choice in scope is still
  refused.
