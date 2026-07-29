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
