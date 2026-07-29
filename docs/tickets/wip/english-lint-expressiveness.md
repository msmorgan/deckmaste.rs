---
needs: [english-shape-rarity]
---
**Lint AST shapes that are structurally under-determined — sound by
construction, no external table required.** The cheapest and
highest-confidence member of the `english-lint-*` set: these fire when the AST
*cannot represent* the distinction the text carries, so the finding needs no
semantics and no adjudication. Whatever the card means, information has been
lost.

Seed shapes:

- **Flat mixed-conjunction coordination.** A coordination node whose members
  carry a mixed `Or`/`And` sequence in one flat list cannot distinguish `(A or
  B) and C` from `A or (B and C)`. Consumers walking the member list have to
  re-guess the grouping. (`english-ast-grouping` fixed the Bonfire instance of
  this by adding `CoordinatedNominalPhrase`; the lint proves no others remain.)
- **Dropped determiner in coordination.** A coordination member with
  `determiner: None` sitting beside one that carries a determiner, where the
  intended reading shares the determiner across members. Note the fix is NOT to
  distribute the determiner onto each member — `target player or planeswalker`
  is ONE target of either type, and distributing yields two.
- Any node whose children admit two readings with nothing in the node
  recording which was chosen.

Sound-by-construction is the acceptance bar: a firing must be a defect without
anyone judging the card. Drop any candidate shape that needs a human to rule on
it — those belong in `english-lint-selectional` with an explicit table.

Standard constraints apply.

## Completion

Two checks in `xtask/src/english/lint.rs`, both sound by construction.

- **`mixed-conjunction`** — 13 findings. A flat member list carrying both `and`
  and `or` cannot distinguish `(A or B) and C` from `A or (B and C)`. Verified
  on Elspeth Resplendent: `Put a +1/+1 counter and a counter from among flying,
  first strike, lifelink, or vigilance on it` parses as one flat mixed list with
  `on it` attached to `vigilance`, when the `or`-run belongs inside `from among`.
- **`bare-singular-conjunct`** — 960 findings. **The first version of this check
  was unsound and the ticket's own framing was wrong**: bareness is not the
  defect, *inconsistency* is. `choose Human, Merfolk, or Goblin` is a uniformly
  determinerless list that strands nothing. Narrowed to a determined conjunct
  beside a determinerless singular sibling (1547 → 1157), then reported per
  coordination rather than per member (→ 960), because `target Shade, Skeleton,
  … or Zombie` is one stranded determiner, not six.

The lesson is worth keeping: "sound by construction" was asserted in a docstring
and falsified by reading what actually fired. Corpus output, not the argument,
settled it.
