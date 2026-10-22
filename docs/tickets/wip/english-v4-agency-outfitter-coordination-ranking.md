---
needs: []
---
# Agency Outfitter: Right Node Raising must take the right edge of a conjunct

## Owner's rulings

Round 7, item 7: "Ticket investigating why Agency Outfitter's wrong
clause-coordination Readings rank ahead of the intended `may [search … and put
…]` (fix via clean structure/ranking, no hax)."

Round 9 recasts it (`docs/decisions/english-v4.md` §4.4, items 6–10). Ranking
is a heuristic, so where the intended Reading ranks is no longer the defect.
The defect is that the grammar admits a Right Node Raising whose shared
constituent is not at the right edge of a conjunct, and no English grammar
admits that. This is a grammatical admission fix, not ranking work. The
former lead 3, which tuned the Ranking Key's Right Association, is dropped
(item 8).

## Evidence

The corrected line (`corrections.txt`) is *When this creature enters, you may
search your graveyard, hand, and/or library for a card named Magnifying Glass
and/or a card named Thinking Cap and put them onto the battlefield.* Measured
on the round-7 base, it has 17 Readings:

- **Readings 1–2 (keys 0/4/43 and 0/11/44): ill-formed.** They are a Clause
  coordination, *[you may search … for a card named Magnifying Glass] and/or
  [a card [named __ Thinking Cap and put them onto] the battlefield]*.
  - The second conjunct is a preterite declarative.
  - Its Verb Phrase coordination shares *the battlefield* by Right Node
    Raising.
  - In *named* (`ComplexTransitive(NP,Pred)/gap0`), the shared phrase fills
    the gapped Object, which the predicative *Thinking Cap* follows. The Gap
    is not at the conjunct's right edge.
- **Reading 3 (0/15/43): intended.** *you may [[search …] and [put them onto
  the battlefield]]*.
- **Readings 4–17: other scopes.** Among them is *[may search …] and
  [put …]*. These are well-formed English, and they stay (§4.4, item 6).

The mixed tense of Readings 1–2 is not the fault. A mixed-tense coordination
is grammatical English, and the owner's round-8 treatment plan retired
`english-v4-coordinated-clause-tense`.

## Change

Right Node Raising needs the shared constituent at the right edge of every
conjunct. *named __ Thinking Cap* has its Gap before the predicative, so it
cannot be a raising conjunct.
1. Find why the `gap0` derivation counts as right-edge here
   (`Summary::gap_right_edge` and `shared_gap_right_edge`,
   `crates/deckmaste_syntax/src/step.rs`).
2. Close it structurally, so that a Gap followed by any further dependent
   inside its conjunct is not at the right edge. This is a correction to the
   engine's existing right-edge rule, so it adds no new suppression
   mechanism (§4.4, item 2) and no `Summary`, `State` or `ProductionDecl`
   field.
3. Add a `!` assertion to the Agency Outfitter judgment in
   `judgments/coordination.txt` against the raising analysis, such as no
   constituent *named Thinking Cap and put them onto*. Keep the entry's
   positive assertions.
4. List every corpus unit whose Readings change. Each Reading removed must be
   an ill-formed raising. A well-formed Reading removed anywhere is a
   regression.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
