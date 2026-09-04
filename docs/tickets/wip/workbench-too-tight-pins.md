---
needs: []
---
**Widen the too-tight gates and split the token-spec tuple.** Cleanroom review
3, 2026-09-04, findings U10, U11 and U12, under the deed-fit ruling
(2026-09-04). A count of printed cards never refuses a rules-meaningful shape.

- **U10 — "of their choice" after a coin flip.** `Phrase.ChoiceMode.TheirChoice`
  gates on `countChoosers bs = 1` counted over the whole stack, so after
  `FlipCoins (each AnyPlayer) …` the flip's subject and the clause's own
  subject both count and Goblin Assassin's printed sentence
  (`sacrifice (each (And [AnyPlayer, CoinCameUp Tails])) (aTheirChoice
  creature)`) is refused; `Cards.Faces.goblinAssassinCoinTails` hides it by
  writing `a creature`. Resolve `TheirChoice` against the enacting agent's own
  delta — the window `workbench-windowed-pro` introduces — and re-spell the
  witness as printed.
- **U11 — `ProofsKeyword.badCompanionSharedColor`.** It refuses "each nonland
  card in your starting deck shares a color" on corpus absence, but colour is
  a characteristic [CR#109.3] and companion's condition is fulfilled by the
  starting deck [CR#702.139a], so the sentence is rules-meaningful. Widen
  `Effect.deckComparable` to the [CR#109.3] characteristics and delete the
  pin.
- **`ProofsDescription.badCantAttackLand`.** It refuses `cantAttack (target
  land)` while the strictly wider `cantAttack (target Permanent)` is admitted.
  Under the deed-fit ruling, `DeedFits` is permissive wherever the described
  head type can be animated (sharing the `StatOf` predicate), and the active
  player chooses which creatures they control will attack [CR#508.1a], so
  "target land can't attack" is admitted and the pin goes. A duration-less
  one-shot restriction is likewise admitted; no pin refuses on absence.
- **`ProofsPiles.badCardWordReadsPiles`.** It refuses "those cards" after
  `SeparateIntoPiles`, although each object in a pile is still an individual
  object [CR#700.3b].
- **`ProofsZone.badEveryCreatureTypeOnLand` and
  `badEveryBasicLandTypeOnCreature`.** They refuse an off-type subtype with no
  rule behind them — neither [CR#205.1a] nor [CR#205.3m] forbids it. Each goes
  unless its docstring can cite the rule that makes the sentence meaningless.
- **U12 — three mis-aimed pins.** `ProofsDamage.badZombieArtifactToken`,
  `badCreatureTokenNoPt` and `badTypelessToken` all refuse on the six-way
  `TokenWellFormed` tuple, so no refusal names the conjunct it means. Split
  the tuple into named obligations on `TokenSpec.TokenWritten` — the spell or
  ability that creates a token sets its name and subtypes [CR#111.4] — and
  re-aim each pin at one of them.

Size: M. Done when: each widened gate admits the sentence named above with a
witness in its own module; every deleted pin is deleted together with its
subject and named in the removed count; the three token pins refuse on
distinct named obligations and probe non-vacuous; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
