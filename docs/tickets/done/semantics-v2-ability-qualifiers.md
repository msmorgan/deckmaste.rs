---
needs: []
---
**The ability printed with a keyword, and the restriction the keyword adds
to it.** Ruling 2026-10-09 ("The ability printed with a keyword is a keyword
parameter", `docs/decisions/semantics-v2.md` §7). Bodies eight census
residues: forecast, boast, exhaust, solved, ∞ (infinity), max speed,
power-up, backup. Standard constraints apply.

## Shape

- `KeywordParam.ability (ability : Ability)`, beside `cost`, `quality`,
  `subject`, `number`, `deckCondition`. The checker's keyword schemas
  already expect it for seven keywords; wire them to it.
- `Ability.qualified (ability : Ability) (timing : Option Timing) (limit :
  Option UsageLimit) (guard : Option Condition) (from : Option Zone)`
  [CR#113.10a,113.6b]. The guard reads "as long as" (static), "this ability
  triggers only if" (triggered) or "activate only if" (activated) by the
  wrapped ability's category [CR#702.169b..702.169d]; `from` is the zone
  the ability functions from [CR#113.6b].
- `Period.thisObject` on `UsageLimit`: "activate only once", reset when the
  object changes zones [CR#602.5b,702.177a].
- `AbilitySelection.printedAfter (keyword : KeywordLabel)`: the abilities
  printed below a keyword, for backup [CR#702.165a,702.165c].
- `PlayerStat` has no speed today: add `PlayerStat.speed` [CR#702.179a]
  and note in the landing record that its game state is `engine-speed`'s.
- Checker: kinds and references only; `qualified` refuses a `limit` on a
  static or triggered wrapped ability as a shape.
- Ability words are unchanged and stay `italicHead` [CR#207.2c].
- Rust mirror, RON helpers, bench macros spelled like the RON target.

## Bodies

- forecast: `qualified(Param(0), timing: inPart(upkeep, owner, current),
  limit: once per turn, from: hand)`, the activation revealing the card
  [CR#702.57a,702.57b].
- boast: activate only if this creature attacked this turn and only once
  each turn [CR#702.142a].
- exhaust: `limit: once per thisObject` [CR#702.177a].
- solved: the guard `hasDesignation(solved, this)` over each category
  [CR#702.169a..702.169d].
- ∞: `conditional(abilityGrant(this, Param(0)), hasDesignation(harnessed))`
  [CR#702.186b].
- max speed: the same with your speed 4, `PlayerStat.speed` [CR#702.178a].
- power-up: once per object, and cost reduced by `bySymbols(manaCostOf(
  this))` if this entered this turn [CR#702.193a]. `bySymbols` is
  `semantics-v2-linked-readback`'s rename of `CostShift.run`; if that has not
  landed, spell it `run` and name the sibling in the landing record.
- backup: the counters trigger, and if that's another creature it gains
  `printedAfter(backup)` until end of turn [CR#702.165a,702.165c].

## Re-spells

Any existing "activate only once each turn" pin re-spells only if its
`UsageLimit` constructor changes; the existing periods keep their meaning.

## Witnesses (canon)

A canon card per keyword bodied (forecast, boast, exhaust, a Case for
solved, a harnessed ∞ card, a max speed card, a power-up card, a backup
card). Pins: a `qualified` activated ability with a guard checks; a
`limit` on a static is refused; `printedAfter` of a keyword the card lacks is
refused as a reference.

## Siblings

Cross-references `semantics-v2-linked-readback` (`bySymbols`) and
`engine-speed` (speed state); neither blocks. Refresh before integrate;
expect conflicts in `Words`/`Phrase`/`Abilities`.

## Landing record

Series `xvnotyvw`..`moqsonsp`. Tests, `lean check` and `lean definitions` were
measured on `moqsonsp` after the refresh that brought in
`semantics-v2-text-changes` and `semantics-v2-off-stack-statics` (one
conflict in `Check/Abilities.lean`, both sides kept; generated files up to
date); the census before on `oktlpppt` (the
claim) and after on `pvmpzlwy` (the series before that refresh).

1. **Census delta**: no English-visible change (no spelling, grammar or
   lexicon edit; the eight declarations keep their spellings). Full
   `cargo xtask english-v3 --all`: 32828 faces, covered faces 26973 (8659
   one, 18314 multiple) before and after, 923730 Readings before and after;
   0 per-face Readings digests changed. Lost faces: none. Lean: `lean check`
   235/235 cards (canon 227, nine of them this landing's witnesses; testing
   8).
2. **Witnesses** (semantic canon cards, each proving `Card.check = []`; English
   Readings after): Spirit en-Dal (forecast) 9, Fearless Pup (boast) 5,
   Skystreak Engineer (exhaust) 3, Brave Brawler (power-up) 3, Golden-Scale
   Aeronaut (backup) 2, Amonkhet Raceway (max speed) 6, The Soul Stone (∞) 5,
   Case of the Crimson Pulse (solved) 12, Expedition Diviner (the non-keyword
   "as long as C, this creature has '[ability]'") 2. Pins in
   `Semantics.Proofs.AbilityQualifiers` (24): a qualified activated ability
   with a guard checks (`okQualifiedActivatedGuard`), a limit on a static is
   refused (`badLimitOnStatic`), `printedAfter` of a keyword the card lacks is
   refused (`badPrintedAfterAbsentKeyword`), and one per body, solved over
   each of its three categories.
3. **Spot checks**: forecast = `qualified(Param(0), timing:
   duringPart(upkeep, ownerOf(this)), limit: oncePerTurn, from: Hand)`;
   boast = `qualified(Param(0), limit: oncePerTurn, guard:
   matches(thisCreature, happenedTo(attacks(relative(Object)),
   soFarThisTurn)))`; exhaust = `qualified(Param(0), limit: oncePerObject)`;
   solved = `qualified(Param(0), guard: matches(thisCase,
   hasDesignation(solved)))`; ∞ = `static(onlyWhile(abilityGrant(
   thisPermanent, Param(0)), matches(thisPermanent,
   hasDesignation(harnessed))))`; max speed = `static(onlyWhile(
   abilityGrant(this, Param(0)), compareAmt(speedOf(you), Eq, 4)))`;
   power-up = the qualifier with `oncePerObject` and `static(onlyWhile(
   reducedBySymbols(thisAbility, manaCostOf(thisPermanent)),
   matches(thisPermanent, happenedTo(enters(relative(Object)),
   soFarThisTurn))))`; backup = `triggered(enters(thisCreature),
   sequentially([putCounters(Param(0), p1p1Counter, target(creature)),
   doIf(matches(that(Creature), otherThan(thisCreature)),
   establish(abilityGrantSelected(that(Creature), thisCreature,
   PrintedAfter(backup)), untilEndOfTurn))]))`; Spirit en-Dal's printed
   ability is `activated(compound([mana([1, White]),
   perform(reveal(thisCreature))]), gain(target(creature), shadow,
   untilEndOfTurn))`; the Case's "To solve" is the triggered ability it means
   [CR#719.3a].
4. **Deviations and STOPs**: no STOPs. Beyond the ticket's letter:
   - Backup's grant needed a carrier for an Ability Selection:
     `StaticSpec.abilityGrantSelected (subject) (source) (selection)`, since
     `abilityGrantFrom` takes classes, not a selection.
   - `printedAfter`'s reference law is a card text law
     (`printedAfterLaws`, refusal `printedAfterKeyword`), walking static
     clauses, establishing instructions and the ability forms around them as
     `namesThisDoor` does; a selection nested in another instruction form is
     not walked. New refusal `qualifiedLimit` for a limit on a non-activated
     ability.
   - Power-up's reduction is a static `costShift` on `thisAbility` beside the
     qualifier, and a cost shift on an ability takes no stack regime
     [CR#602.2b,601.2f] (Lean `StaticSpec.regimeFits`, xtask
     `static_regime`), so power-up's row stays regime-free.
   - Facts: a qualifier over the keyword's own argument has the printed
     ability's category, so the overlay names it from the CR entry (forecast,
     boast, exhaust, power-up activated; solved static, triggered and
     activated); backup's overlay category retires now its body is written.
     `Category`'s stale `dead_code` expectation removed.
   - Forecast's reveal is part of each card's printed cost ("Reveal this card
     from your hand"); the keyword adds no reveal qualifier.
   - `deckmaste_construction_core`'s English keyword-line class for an
     `Ability` parameter stays `Unsupported(Ability)`: that is the
     keyword-line grammar's codec, not the semantic reader, which now carries
     the argument (`KeywordParam::Ability`).
   - `PlayerStat.speed`'s game state is `engine-speed`'s.
   - Glossary: Ability Selection and Usage Limit amended
     (`docs/contexts/game-model/CONTEXT.md`); Qualified Ability was already
     defined.
5. **Tests**: `lake build --wfail` clean (97 jobs); `cargo test -p
   deckmaste_semantics_v2` pass (47+7+1+4+46); `cargo test -p xtask` pass
   (407 passed, 1 ignored pre-existing; 4; 14); `cargo test -p
   deckmaste_construction_core` pass; `cargo test -p deckmaste_lexical_source`
   pass (28+36); `cargo clippy -p <crate> --tests -- -D warnings` clean for
   all four; `cargo xtask lean check` 235/235; `cargo xtask lean definitions`
   711/711; `facts check`, `lean macros check` up to date; `cite check` 0
   stale, 0 non-compliant. Counts: restored 0; re-spelled 1
   (`keywords::tests::a_keyword_ability_forwards_its_signature`: an `Ability`
   parameter is now forwarded as `Ability(Param(1))`, the ticket's change);
   ignored 0; added 24 Lean theorems, 1 Rust test
   (`a_keyword_carries_its_printed_ability_and_qualifies_it`), 9 canon cards;
   removed 0.
6. **Timing**: one full census 285 s wall, 16 workers (`after`, `pvmpzlwy`).
