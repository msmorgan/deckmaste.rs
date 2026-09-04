---
needs: []
---
**One `scry`, one player ref, one clause.** Ruling 2026-09-04 on the look
bodies (review R6, fused-variants-3 STOP 2). `Macros.scryBody` (40 lines,
four clauses) and `surveilBody` (36 lines) match on the `LookAgent`/`LookReq`
witnesses (50 more lines) to split on which reference the agent is (`You`
versus a bound player), and again on scry 1 versus scry N; `fateseal` is a
third hand-inlined copy that hard-codes `anOpponent`. About 140 lines for
three keyword actions whose rules text is one sentence each.

Both splits go:

- **Reference split → a delta-directed re-read.** One grammar-level
  function `agentRef agent : Noun (agentIntro agent) Player`, total over the
  agent's delta rather than its constructor: `You` when `nounDelta agent =
  []`, otherwise a Player-kinded `Own` reading the agent's own delta
  positionally (generalise `Phrase.Noun.Own` over kind, or add its Player
  twin; `Own` already carries plurality, so `each AnyPlayer` re-reads as a
  plural own-read and `Enact`'s distributive machinery supplies the
  per-player meaning). Card references stay as written; no macro ever
  matches on who the agent is; no matching on implicits anywhere in the
  result. `LookAgent`, `LookReq`, `lookAgentTop`, `lookedAgentTop`,
  `scryBody`, `surveilBody` and `nounIsYou`'s use in them are deleted.
- **Amount split → the rule's single definition.** [CR#701.22a] defines
  scry once for every N ("look at the top N cards… then put any number of
  them on the bottom… and the rest on top in any order"); the workbench's
  scry-1 clause copies the printed reminder text instead. Delete the
  `not (oneCardAmount amt)` gate and admit a one-card slice as a degenerate
  group (any number of one card; "the rest" possibly empty); scry 1 sites
  read through the same clause. Same for surveil [CR#701.25a] and fateseal
  [CR#701.29a]. Bench "each player scries 1" as a rules-meaningful synthetic
  witness.

Result: `scry agent amt`, `surveil agent amt`, `fateseal agent amt` are each
one macro over one player noun with one clause, obligations on the macro's
type and searched at the card site. Re-spell every bench site (16 today) and
the look pins; probe non-vacuous.

Size: M. Done when: the three macros are one clause each over one player
ref; the witness types and the one-card gate are gone from the tree; every
scry/surveil/fateseal witness typechecks; "each player scries 1" is
benched; build at its module count. Standard constraints apply, including
the RON-shaped constraint.
