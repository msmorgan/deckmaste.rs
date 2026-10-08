---
needs: []
---
**One cost-substitute mechanism: an action that pays for mana.** Ruling
2026-10-08 ("A cost substitute is one mechanism", `docs/decisions/semantics-v2.md`
§7). It replaces `Cost.tapForGeneric`, the residues landing's waterbend-only
node. Standard constraints apply.

## Shape

- `ManaSubstitute (action : Instruction) (pays : List PipPaid)`, with
  `PipPaid = generic | ofColor (color : ColorTerm)`. The action comes first;
  `pays` reads the acted-on object back by anaphora ("that creature's
  color"). `ColorTerm` (today `lit | chosen`) gains an object's colour,
  `colorOf (object : NounPhrase)`. One row per keyword, no per-colour
  expansion.
- Carriers:
  - `StaticSpec.paySubstitute (subject : NounPhrase) (substitute :
    ManaSubstitute)`, on a spell. It applies after the total cost is locked
    and is neither an additional nor an alternative cost
    [CR#702.51b,702.66b,702.126b].
  - `Cost.withSubstitute (cost : Cost) (substitute : ManaSubstitute)`, for a
    named cost. It pays only that cost's generic mana [CR#701.67b].
- Checker: cases for the anaphor in `pays` (it must resolve to the action's
  patient) and for kinds. It refuses nothing else.
- Rust mirror, RON helper `substitute`, and bench macros spelled like the
  RON target.

## Retires

`Cost.tapForGeneric` (Lean and Rust mirror), the RON helper
`plugins_v2/builtin/macros/abilities/tapForGeneric.ron`, and the checker
cases for both.

## Re-spells (same asserted outcome)

- `waterbend.ron`: `withSubstitute(<cost>, substitute(tap(an(and([untapped,
  or([artifact, creature]), youControl]))), [generic]))` [CR#701.67a].
- Canon: Geyser Leaper.
- Pins in `lean/Semantics/Proofs/KeywordActionResidues.lean`: `okWaterbend`,
  and `badTapForGenericWithPlayers` re-spelled as a substitute whose action
  names players. Also every Rust test that names `tapForGeneric`
  (`crates/deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`,
  `builtin_v2_keyword_spelling_templates.rs`).

## Bodies

- convoke: `paySubstitute(this, substitute(tap(an(and([untapped, creature,
  youControl]))), [generic, ofColor(colorOf(that(Creature)))]))` [CR#702.51a].
- delve: `substitute(exile(a(cardIn(yourGraveyard))), [generic])` [CR#702.66a].
- improvise: tap an untapped artifact you control, `[generic]` [CR#702.126a].
- assist stays bodyless: another player pays, and no action does
  [CR#702.132a]. Record it as a deliberate residue.

## Witnesses

Geyser Leaper (waterbend), plus one canon card each for convoke, delve and
improvise, added if canon lacks one. Pins: convoke's colour readback
resolves; a substitute whose `pays` anaphor has no antecedent is refused.

## Deltas

A, B and C all touch `Words`/`Phrase`/`Abilities` and run in parallel.
Refresh before integrate and expect conflicts there.
