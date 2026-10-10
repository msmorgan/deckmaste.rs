---
needs: []
---
**Copy keywords offer new targets unconditionally.** Ruling 2026-10-09 ("A
spell copy's target guard is vacuous", `docs/decisions/semantics-v2.md` §7).
Bodies casualty, replicate, epic and conspire's copy half, the census's
"condition that a spell has any targets" group. Standard constraints apply.

## Shape

- No optional subject on `Predicate.targets`; no "has any targets"
  condition. A copy keyword writes `may(chooseNewTargets(…))` with no guard,
  as storm does: its player may leave every target unchanged, so a copy
  with no targets has none to change [CR#707.10c].
- `TargetExtent.only`: "targets only …" (24 cards), beside `someTarget` and
  `soleTarget`.
- Copying "for each … it could target" [CR#707.10d] is not this ticket:
  it stays with `engine-copy-target-variants`.
- Rust mirror and RON helper for `only`.

## Re-spells (same asserted outcome)

- gravestorm (`plugins_v2/builtin/macros/keyword_abilities/gravestorm.ron`):
  its `doIf(matches(thisSpell, targets(…, someTarget)), …)` guard goes;
  `may(chooseNewTargets(…))` follows the copy directly. Regenerate
  `lean/Semantics/Macros/Declared.lean` so `cargo xtask lean macros check`
  passes; pins over gravestorm keep their statements.
- Any other copy keyword or canon card with the same guard, re-spelled the
  same way.

## Bodies

- casualty: `addedCost(perform(sacrifice(a(and([creature,
  powerAtLeast(N)])))), true)` plus a cast trigger copying the spell under
  `keywordCostPaid(casualty, this)` [CR#702.153a].
- replicate: `addedCost(Param(0), true, anyNumberOfTimes)` plus a cast
  trigger copying it `timesPaid` times [CR#702.56a].
- epic: the `restOfGame` can't-cast-spells deontic and `delay(beginningOf(
  upkeep, you, each), copy(this, except: ability(epic)), restOfGame)`
  [CR#702.50a].
- conspire's copy half: the cast trigger copying under `keywordCostPaid`
  [CR#702.78a]. Its cost (two untapped creatures you control that share a
  color with it) is `semantics-v2-sharing`'s `shares`; if sharing has not
  landed, conspire stays bodyless and its STOP names that ticket.

## Witnesses (canon)

A canon card each for casualty, replicate and epic if canon lacks one; one
"targets only …" card (`only`). Pins: each body checks; `only` over a
player-kinded subject where an object is required is refused.

## Siblings

Cross-references `semantics-v2-sharing` (conspire's cost) and, through the
`timesPaid` spelling, `semantics-v2-linked-readback`'s rename; neither
blocks. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.

## Landing record

Series `ruyztron`..`smmsxsvu` on `vlplxnos`. `lake build`, the crate tests,
`lean check` and `lean definitions` were measured on `smmsxsvu` after the
refreshes that brought in `english-v3-last-turn-temporal-adjunct`,
`semantics-v2-text-changes`, `semantics-v2-off-stack-statics` and
`semantics-v2-ability-qualifiers` (conflicts in
`Proofs.lean`'s import list, both imports kept, and in the generated
`Facts.lean`, regenerated with `Declared.lean`). The census `after` was measured on `mmwvuwqm` and `before` on
`vlplxnos`, both before that refresh.

1. **Census delta**: no English-visible change; no keyword grammar or
   spelling changed, and the two new helpers live in families the lexicon
   does not read. Full `cargo xtask english-v3 --all`, before (`vlplxnos`)
   and after (`mmwvuwqm`): 32828 faces, covered 26973 (8659 one, 18314
   multiple) both; 923730 Readings both; the same lexical inventory hash and
   0 per-face Readings digests changed. Lost faces: none. Lean: `lean check`
   240/240 (canon 232, five of them this landing's witnesses; testing 8).
2. **Witnesses** (canon cards proving `Card.check = []`; Reading counts from
   the `after` census): Light 'Em Up (casualty) 2, Train of Thought
   (replicate) 2, Endless Swarm (epic) 36, Burn Trail (conspire) 2, Bill
   Potts ("targets only", `TargetExtent.only`) 0 (not yet covered by
   English, unchanged by this landing). Ominous Harvest (gravestorm) keeps
   proving. Pins in `Semantics.Proofs.SpellCopies` (8): `okSpellTargetsOnly`,
   `badPlayerTargetsOnly` (`only` under a player-kinded subject where an
   object is required is refused `[.targeter .player]`),
   `targetsOnlyIsTargetsAtOnly`, and one per body (`okCasualty`,
   `okReplicate`, `okEpic`, `okConspire`, `okGravestorm`).
3. **Spot checks**: casualty = `addedCost(perform(sacrifice(a(and([creature,
   compare(AtLeast, N, [Stat(Power)])])))), true)` and the cast trigger under
   `keywordCostPaid(casualty, thisSpell)` copying once, then
   `may(chooseNewTargets(that(Copy)))`; replicate = `addedCost(cost, true,
   AnyNumberOfTimes)` and `copySpell(thisSpell, timesPaid(ByKeyword(
   replicate), thisSpell))` then `may(chooseNewTargets(those(Copy)))` under
   `keywordCostPaid`; epic = `spell(establish(playerCant(cast, you),
   restOfGame))` and `spell(delayWithin(beginningOfYour(upkeep), restOfGame,
   copy(FromStack, thisSpell, 1, [Edits([RemovedAbilities(Specified([Term(
   The(epic))]))])]) then may-new-targets))`; conspire = the sharing cost
   (`tap(counted(exactly(2), untapped creature you control that
   shares(Color, Share, thisSpell)))`) and the copy under `keywordCostPaid`;
   gravestorm = copy for each permanent put into a graveyard this turn, then
   `may(chooseNewTargets(those(Copy)))` with no guard; Bill Potts =
   `triggeredOrOnlyOnce(casts(you, instant or sorcery spell
   targetsOnly(thisCreature)), [activates(you, activated ability
   targetsOnly(thisCreature))], oncePerTurn, copy(FromStack, that(Stack), 1)
   then may-new-targets)`; Endless Swarm = `create(numberOf(bare(cardIn(
   yourHand))), 1/1 green Snake)` and epic; Light 'Em Up, Train of Thought
   and Burn Trail carry their keywords beside their spell abilities.
4. **Deviations and STOPs**: no STOPs. Beyond the ticket's letter:
   - Epic's copy exception is `Edits([RemovedAbilities(Specified([Term(The(
     epic))]))])`, not `ability(epic)`: `CopyExcept.ability` holds an
     `Ability`, so naming epic there would embed epic's own definition in
     itself; the keyword term removes it by name.
   - Casualty's "power N or greater" is `compare(AtLeast, N, [Stat(Power)])`;
     no `powerAtLeast` helper exists and none was added.
   - New helpers `predicates/targetsOnly` (with its bench twin in
     `Macros.lean`, aligned) and `abilities/triggeredOrOnlyOnce` (Bill
     Potts's two-header trigger with a usage limit; no helper combined
     both).
   - Facts: Epic's overlay row gains regime `AtResolution` (two spell
     abilities [CR#702.50a,113.3a]) and `onPermanentCard` false (every
     printed epic card is a sorcery); not `functionsOnStack`, as cipher's
     spell-ability stack part is not. Generated `definition` rows for
     casualty, replicate, conspire and epic.
   - Bill Potts omits "Doctor's companion", a partner ability for Commander
     deck construction [CR#702.124a,702.124m], outside Vintage-playable
     scope. Radiate was not used: its "targets only a single" is
     `soleTarget`, and its copy-for-each-could-target is
     `engine-copy-target-variants`.
   - The owner's clarification is one added sentence in the §7 2026-10-09
     copy ruling (the guard was dropped because writing it needed a new
     constructor; an existing-vocabulary guard stays as the CR writes it;
     this does not generalise), and the five keyword comments state that
     reason. Gravestorm's old guard read "the spell targets a source", not
     the CR's guard, so it re-spells away as the ruling says.
   - `Proofs/Sharing.lean`'s `okConspireCost` doc comment points at the
     full body; its statement is unchanged.
   - No glossary gap.
5. **Tests**: `lake build --wfail` clean (98 jobs); `cargo test -p
   deckmaste_semantics_v2` pass (47+7+1+4+46); `cargo test -p xtask` pass
   (407 passed, 1 ignored pre-existing; 4; 14); `cargo test -p
   deckmaste_construction_core` pass (434 + 13 suites); `cargo test -p
   deckmaste_lexical_source` pass (35+36); `cargo clippy -p <crate> --tests
   -- -D warnings` clean for all four; `cargo xtask lean check` 240/240;
   `cargo xtask lean definitions` 711/711; `facts check` and `lean macros
   check` up to date; `cite check` 0 stale, 0 non-compliant (115.9 newly
   registered). Counts: restored 0; re-spelled 1 (xtask
   `a_keyword_declaration_builds_its_wrapper`: its bodyless-`Amount` subject
   moves from casualty, which now has a body, to toxic, which stays
   bodyless by ruling; same asserted outcome) plus the gravestorm body;
   ignored 0; added 8 Lean theorems and 5 canon cards; removed 0.
6. **Timing**: one full census 4 m 12 s wall, 16 workers (`after`,
   `mmwvuwqm`).
