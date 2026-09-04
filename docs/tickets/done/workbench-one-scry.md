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

## As landed

- **Reference split.** `Macros.agentRef agent ok : Noun (agentIntro agent) Player`
  is total over the agent's delta, never its constructor: an agent whose delta
  is empty is re-read as written, anyone else as a Player-kinded `Own` over its
  own delta. `Phrase.Own` was generalised over `Reach` (the scoped twin of
  `Pro`, `Noun bs (reachKind r)`) rather than given a Player-only twin.
  `Phrase.agentIntro` was factored into `agentDelta`/`agentPlur` so that
  `agentIntro n = agentDelta n ++ bs` holds definitionally and `agentRef` can
  split on the list. `LookAgent`, `LookReq`, `lookAgentTop`, `lookedAgentTop`,
  `scryBody`, `surveilBody`, `oneCardAmount` and `lookAtAgentsTop` are gone;
  `grep -rn` over `idris/src/` finds none of them. `nounIsYou` survives (its
  only remaining caller is `Effect.costPaidByYou`) but no look macro reads it.
- **Amount split.** One body, `Macros.lookAndSort`, one clause, shared by all
  three macros: look at the top N of the reader's library, then move
  "any number of them" to the spill destination and "the rest" on top in any
  order. The amount is abstract; the one-card slice goes through the same
  clause as a degenerate group. The `not (oneCardAmount amt)` gate is deleted
  with its witness type.
- **`scry`/`surveil`/`fateseal`** are each one clause over one player ref, with
  every obligation an auto-implicit on the macro type passed by name into the
  core constructors. `fateseal agent amt` now takes its opponent noun.
- **Bench.** All 16 look sites typecheck; only `spinIntoMyth` changed spelling
  (`Macros.fateseal Macros.anOpponent (Lit 2)`). New positive witness
  `Experimental.ProofsKeyword.okEachPlayerScriesOne` — "Each player scries 1."
  as `Macros.scry (Macros.each AnyPlayer) (Lit 1)`; no printed card in
  `data/derived/cards.jsonl` writes the shape, so it is synthetic
  [CR#701.22a,701.22c].
- Nothing was left undone; no STOP was taken.

## Landing record

Measured on change `nxmztrum` (working copy at commit time), tree
`.workspaces/workbench-one-scry`.

**Numbers before / after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| look block in `Macros.idr` (lines) | 208 | 240 |
| clauses: `scry` / `surveil` / `fateseal` | 1 / 1 / 1 over 4-clause bodies | 1 / 1 / 1 over a 1-clause shared body |
| witness data types for the look | 2 (`LookAgent`, `LookReq`) | 0 |
| look sites on the bench | 16 | 16 |
| clean `./scripts/build` wall time | 2m26.7s | 2m21.5s |

**Gate lines**

- `cd idris && ./scripts/build` (after `rm -rf build`) — last line
  `46/46: Building Cards (src/Cards.idr)`; no `Error` and no `Warning` line.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite bless` — `blessed 1421 rules at cr_date 2026-08-07`, newly
  registering `[CR#701.22c]` only.
- `cargo xtask cite check` — `checked 14205 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` — `audited 7
  citation site(s)`; each rule text read against its claim.

**Assurance counts** — restored 0; re-spelled 1 pin
(`ProofsAnaphora.badSharedSubjectTwoInDelta`, for `Own`'s new `Reach`
argument) plus 3 non-pin `Own` sites in the same module; ignored 0; added 1
(`ProofsKeyword.okEachPlayerScriesOne`, a positive witness); removed 0.
Probes: `ProofsZone.badPartitiveOfDescription` re-pointed at a real target
group reports `badPartitiveOfDescription Oh is not a valid impossible case`;
`ProofsAnaphora.badSharedSubjectTwoInDelta` narrowed to one target in the
delta reports `badSharedSubjectTwoInDelta Refl is not a valid impossible
case`. Both were reverted.

**Deviations and additions**

- `Own` generalised over `Reach` instead of gaining a Player twin (the ruling
  allowed either); all seven existing uses re-spelled with `Bare`, behaviour
  unchanged.
- `agentRef` returns **the agent as written** when its delta is empty, not
  literally `You`. `Macros.scry They (Lit 1)` (Eager Construct) has an empty
  delta too, and rewriting it to `You` would silently move the scry to the
  controller's library. `You` remains the answer whenever the agent is `You`.
- `agentPlur` was added beside `agentDelta` because `agentIntro` re-binds an
  `each` agent as `TheD OneOf`: the own-read is singular and `Enact`'s
  `doesInstrIntro ManyOf` supplies the distribution, so "each player scries 1"
  is one singular clause distributed, not a plural own-read.
- The group read is `Own Bare` over the look's own delta, not
  `Pro Bare ManyOf`. With a pronoun, scry 1 reads `Pro Bare OneOf` and is
  ambiguous against any singular object antecedent already in scope —
  Artificer's Assistant ("Whenever you cast a historic spell, scry 1") failed
  its face laws that way. The own-read is scoped to the look's delta and is
  unique by construction, for every N.
- **The gate that refused the one-card slice** was `partitiveBase`: a
  partitive needs a group base, which is what refuses "one of a creature you
  control" (`ProofsZone.badPartitiveOfDescription`). It now admits an own-read
  base, `partitiveBase (Own _ _ _ _) = True`, because [CR#701.22a] defines scry
  once for every N and a one-element group is a group. The description pin
  still refuses, probed above; `nounDelta`/`nounPlur` of a library slice are
  untouched, so "look at the top card … put that card …" stays singular.
- One shared `lookAndSort` body replaces the three hand-inlined copies; the
  spill destination and its proofs are parameters.
- The block grew 208 → 240 lines. The old compactness came from packing
  obligations into `LookAgent`/`LookReq`, which the ruling forbids; threading
  them on three macro types costs more text than it saves. The matching, the
  duplicated bodies and the witness types are gone.
