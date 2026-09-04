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

## As landed

- **U10 — "of their choice" after a coin flip.** `Events.ChoiceMode.TheirChoice`
  now takes a `Window` positional and gates on
  `countChoosers (view w bs) = 1`, mirroring `Phrase.Noun.Pro`;
  `Macros.aTheirChoice` fixes that window at `Top 1`, the binding the clause's
  own subject introduced, so an earlier clause's player no longer counts.
  `Cards.Faces.goblinAssassinCoinTails` re-spelled as printed
  (`sacrifice (each (And [AnyPlayer, CoinCameUp Tails])) (aTheirChoice creature)`).
  `ProofsMana.badUnboundTheirChoice` still refuses and was re-probed;
  `ProofsAnaphora.theirChoiceReadsOnlyPrefix` re-spelled with `Whole`. All
  eight existing `aTheirChoice` sites typecheck unchanged.
- **U11 — `ProofsKeyword.badCompanionSharedColor`.** `Effect.deckComparable`
  widened to the [CR#109.3] characteristics (`Color`, `SubtypeQ _`, `CardName`,
  `CardTypeQ`; `Number` and `CounterKindQ` refused). Pin deleted; witness
  `okCompanionSharedColor` in its place, and `badCompanionSharedCounterKind`
  added so `DeckComparable` keeps a pin.
- **`ProofsDescription.badCantAttackLand`.** `Events.deedAnimatedOk` makes the
  **agent** slot of a deed a creature performs fit any permanent head type
  ([CR#205.1b], [CR#208.3a]); patient slots keep the set the rules name. Pin
  deleted; witnesses `okCantAttackLand` and `okCantAttackLandNoSpan` (the
  duration-less restriction, which was already admitted — no pin refused on
  absence).
- **`ProofsPiles.badCardWordReadsPiles`.** `Effect.instrIntro`/`instrProfile`
  for `SeparateIntoPiles` keep the object group (`partsClosed` in place of
  `groupSpent Object`) per [CR#700.3b]. Pin deleted; witness
  `okCardWordReadsPiles`. `badPileWordWithoutAPartition` still refuses.
- **`ProofsZone.badEveryCreatureTypeOnLand` / `badEveryBasicLandTypeOnCreature`.**
  Both KEPT: [CR#205.3d] ("an object can't gain a subtype that doesn't
  correspond to one of that object's types") is the rule that makes each
  sentence meaningless, and each docstring now cites it. Both re-probed.
- **U12 — three mis-aimed pins.** `Effect.TokenWellFormed` deleted; its six
  conjuncts are named implicit obligations on `TokenSpec.TokenWritten`
  (`tt`/`tp`/`sf`/`ta`/`tc`/`qf`), threaded by `Macros.create` and
  `Macros.createTappedAttacking`. `badTypelessToken` refuses `tt`,
  `badCreatureTokenNoPt` refuses `tp`, `badZombieArtifactToken` refuses `sf`;
  aiming any of the three at a sibling obligation stops the build.

## Landing record

Numbers before/after: Idris modules 46 → 46. Pins 611 → 603 (9 deleted, 1
added). Positive witnesses added: 10. Bench witnesses re-spelled: 1
(Goblin Assassin).

Gate lines:

- `cd idris && ./scripts/build` → `46/46: Building Cards (src/Cards.idr)`;
  no `Error` and no `Warning` lines; `check-pin-twins` clean.
- `cargo xtask cite check --list-noncompliant` →
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` →
  `checked 14384 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → 30 citation
  sites, each rule text read against the line citing it. No `cite bless` was
  needed: every rule cited this round was already in `cr-citations.lock`, so
  the lock is unchanged.

Assurance counts: restored 0; re-spelled 8 (7 token pins re-aimed at named
obligations after `TokenWellFormed` was deleted —
`badZombieArtifactToken`, `badCreatureTokenNoPt`, `badTypelessToken`,
`badTokenDuplicateType`, `badTokenDuplicateColor`, `badTokenSpellAbility`,
`badTokenDuplicateSupertype` — plus `theirChoiceReadsOnlyPrefix`); ignored 0;
added 11 (1 pin, 10 witnesses); removed 9 pins, each replaced in its own module
by a positive witness asserting the now-admitted sentence, so no sentence lost
coverage. Pins deleted, by name and reason:

- `ProofsKeyword.badCompanionSharedColor` — colour is a characteristic
  [CR#109.3]; sentence admitted (ticket).
- `ProofsPiles.badCardWordReadsPiles` — each object in a pile is still an
  individual object [CR#700.3b]; sentence admitted (ticket).
- `ProofsDescription.badCantAttackLand` — [CR#508.1a,205.1b,208.3a] (ticket).
- `ProofsDescription.badCantDisjunctSubject` — "target creature or land can't
  block"; both head types can be blocking creatures [CR#205.1b,208.3a].
- `ProofsDeontic.badMustAttackLand` — "target land attacks each combat if
  able" [CR#205.1b,208.3a].
- `ProofsDeontic.badCoordinatedLandHostBlocks` — "enchanted land gets +1/+1 and
  can't block" [CR#205.1b,208.3a].
- `ProofsFaces.badPlaneswalkerAttacks` — "enchanted planeswalker can't attack"
  [CR#205.1b,208.3a].
- `ProofsStatic.badLandBecomesBlocking` — "target land blocks an attacking
  creature" [CR#205.1b,509.1a].
- `ProofsDamage.badFightLand` — "target land fights target creature you don't
  control"; a fighter that is no longer a creature simply does not fight
  [CR#701.14b].

Non-vacuity probes (each pin mis-stated once, build re-run, message changed):
the three U12 pins aimed at a sibling obligation fail at the *type*
(`Can't find an implementation for So False`), which is what proves the three
obligations distinct; the four re-spelled token pins likewise; both
`ProofsZone` pins with the space swapped give
`... Oh is not a valid impossible case`; `badCompanionSharedCounterKind` with
`Color` and `badUnboundTheirChoice` given a subject give the same. Bounds of
the deed widening checked by evaluation: `deedTypeOk "Attack" Agent Land =
True`, `Instant = False`, `Kindred = False`, `deedTypeOk "Attack" Patient Land
= False`, `deedTypeOk "Block" Patient Planeswalker = False`.

Deviations and additions:

- The deed-fit widening retired six pins the ticket does not name
  (`badCantDisjunctSubject`, `badMustAttackLand`,
  `badCoordinatedLandHostBlocks`, `badPlaneswalkerAttacks`,
  `badLandBecomesBlocking`, `badFightLand`). Each sentence is rules-meaningful
  under the same ruling; each became a positive witness carrying its cite,
  listed above.
- `badCompanionSharedCounterKind` added: deleting the only pin on
  `DeckComparable` would have left the widened gate unpinned.
- `okCantAttackLandNoSpan` added to carry the ticket's "a duration-less
  one-shot restriction is likewise admitted" — no pin refused it, so nothing
  was deleted for it.
- `Effect.instrProfile`'s announced row for `SeparateIntoPiles` changed with
  `instrIntro` so the two stay in step. `Words.groupSpent` survives; its other
  caller is `Phrase.moveIntro (TheRest …)`.
- `TheirChoice`'s window is `Top 1` in `Macros.aTheirChoice` rather than
  `Top (length (agentDelta agent))`: the agent's delta length is not visible at
  the noun, and the chooser the subject introduces is always the head binding,
  so `Top 1` is the tighter spelling of the same read. The core constructor
  takes a general `Window`, as `Pro` does.

STOP taken and resolved: the ruling asks the deed-fit predicate to *share the
`StatOf` predicate*, but literal sharing would also admit
`StatOf Power (target land)` and so retire
`ProofsDescription.badLandPower`, which the ticket does not name and which
[CR#208.3] grounds ("a noncreature permanent has no power"). Resolved by
sharing the animatability *notion* — `Words.permanentType`, read by the new
`Events.deedAnimatedOk` — and leaving `Words.statHeadTysOk` untouched:
[CR#208.3] makes a value read of a noncreature permanent's power meaningless,
while [CR#208.3a] makes a restriction written on the same permanent
meaningful. `badLandPower` and `okAnimatedLandPower` both still hold.
