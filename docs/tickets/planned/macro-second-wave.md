---
needs: []
---
**The second macro wave: the highest-leverage convenience macros over the
existing grammar, ranked by how many un-graduated cards each unblocks.** Builds
directly on the template-forward parser fold (the static-ability parser now
emits English-shaped macro invocations that persist into generated cards, piloted
on Elesh Norn). Every macro here is expressible *today* over existing core
primitives — no new engine subsystem, no new grammar node.

Related: [[macro-first-wave]], [[macro-keyword-templates]],
[[parse-target-hoisted-effect-macros]].

## Why / impact basis

A clustering + feasibility sweep over the live `plugins/wizards` corpus (23,172
cards carrying an `Unparsed` ability; 16,935 of them **single-blocker** — one
macro graduates the whole card) sorted the un-graduated mechanic-families into
verdict buckets. The **pure-macro-win** bucket — expressible as a macro over
existing primitives right now — is **17 families ≈ 3,845 single-blocker cards**.
This ticket implements that bucket, in impact order. (Counts are regex-based,
greedy first-match de-duped across families → directional upper bounds, not
exact. `⚠` marks generic *trigger-wrapper* families: the trigger parses, but the
diverse *payload* is the real blocker, so one macro won't graduate all — treat
`sole` as a ceiling.)

| sole | occ | family | headline macro(s) |
|--:|--:|---|---|
| 566 | 908 | graveyard-recursion | parser **arm** (not a card macro) — see note |
| 453 | 639 | bounce-to-hand | `ReturnToHand` (see core-verb note) |
| 364 | 553 | combat-damage-to-player-trigger | `DealsCombatDamage` (EventFilter) |
| 363 | 609 | cast-spell-trigger | `YouCast` (EventFilter) |
| 339 | 511 | misc-event-trigger | EventFilter sugar (`DealtDamage`, `BecomesTapped`, `Draws`, …) |
| 292 | 507 | combat-restriction | `CantBlockUntilEot` + siblings |
| 287 | 507 | graveyard-hate | `ShuffleYourGraveyardIntoLibrary`, `CardInAGraveyard` |
| 220 | 332 | type-change-animate | `BecomesCreatureStillLand` + siblings |
| 186 | 295 | damage-sweeper | `DealsDamageToEach` |
| 164 | 197 | block-trigger ⚠ | `Blocks`/`ThisBlocks`/`BecomesBlocked` |
| 148 | 405 | enters-with-counters | `EntersWithCounters` |
| 118 | 168 | bounce-to-library | `PutOnLibrary`/`PutOnBottomOfLibrary` |
| 104 | 184 | loot-rummage | `Loot`/`Rummage` |
| 92 | 150 | fight-bite | `Bite` |
| 84 | 166 | pt-equal-to-count | `PowerAndToughnessCountEqual`/`PowerCountEqual` |
| 48 | 136 | ltb-trigger ⚠ | `LeavesBattlefield`/`ThisLeavesBattlefield` |
| 17 | 27 | sacrifice-self-conditional | `YourUpkeep` (EventFilter) — rest rides existing macros |

Tackle top-down; each family's `sole` is the marginal graduation it unblocks, so
even a partial pass (top ~6 families ≈ 2,400 cards) is a large win. Land in
impact-order sub-batches, each its own commit with before/after graduation counts.

## Concrete macros (verify current primitive names against the tree before authoring)

**bounce-to-hand — `ReturnToHand`.** `Move(<ref>, Hand)` IS the bounce (exact twin
of `Exile` = `Move(_, Exile)`); targeting lives in an outer `Targeted` shell.
NOTE: core still carries a legacy `Action::ReturnToHand(Reference)` verb emitting
the identical `Move(ref, Hand)`, and `macro-first-wave` declined a macro for that
reason ("the name is taken by core"). Decide deliberately: either make the
Move-based macro spelling canonical (matching `Exile`) and prune the dead core
verb, or keep the core verb and skip this macro. Do not ship both silently.
```ron
( name: "ReturnToHand", template: "return ${0} to its owner's hand",
  kinds: [OneShotEffect], params: [Reference], body: Move(Param(0), Hand) )
```

**combat-damage-to-player-trigger — `DealsCombatDamage`.** The `Damage` EventFilter
with the combat flag pinned; `${1}` is a Predicate slot so one macro covers "a
player"/"an opponent"/"a creature". (`Damage` is already used inline by
Bloodthirst/Wither/Infect, so the node is proven.)
```ron
( name: "DealsCombatDamage", template: "${0} deals combat damage to ${1}",
  kinds: [EventFilter], params: [Predicate, Predicate],
  body: Damage(source: Param(0), to: Param(1), combat: true) )
```

**cast-spell-trigger — `YouCast`.** The Prowess/heroic base shape, mirroring
`event/Attacks`.
```ron
( name: "YouCast", template: "you cast ${0}", kinds: [EventFilter],
  params: [Predicate], body: Cast(who: Ref(You), what: Param(0)) )
```

**misc-event-trigger — EventFilter sugar** (each wraps one existing master form,
exactly like `event/Dies` wraps `ZoneChange`): `DealtDamage` (`Damage(to:)`),
`BecomesTapped` (`StateBecame(becomes: Tapped)`), `Draws` (`Drawn(who:)`),
`GainsLife` (`LifeGained(who:)`), `BecomesTargeted` (`BecomesTarget(what:)`),
`CounterAdded`/`CounterRemoved` (`CounterPlaced`/`CounterRemoved(on:)`),
`BecomesMonarch` (`DesignationChanged(name: "Monarch")`), `TurnedFaceUp`.

**combat-restriction — `CantBlockUntilEot`** and siblings, a `Continuously`-scoped
Deontic lasting until end of turn (same substrate as the keyword macros
Defender/Menace):
```ron
( name: "CantBlockUntilEot", template: "${0} can't block this turn",
  kinds: [OneShotEffect], params: [Predicate],
  body: Continuously(effect: Cant(Block(by: Param(0))), duration: FixedUntil(EndOfTurn)) )
```
Siblings: `MustBeBlockedUntilEot` (`Must(Block(on:))`), `CanBlockAdditional`
(`Static(May(Block(count: AtMost(2))))`), `AttackTax` (`Static(Gate(Attack(on:), cost))`).

**graveyard-hate — `ShuffleYourGraveyardIntoLibrary`** (pure composition over
`Each`/`Move`/`Shuffle`) + Predicate macro `CardInAGraveyard`
(`AllOf([Card, InZone(Graveyard)])`) so "Exile target card from a graveyard"
parses via the existing `Exile` + `TargetOne` path.

**damage-sweeper — `DealsDamageToEach`** (mass burn over `Each`/`DealDamage`;
`${1}` recipient is a Predicate slot):
```ron
( name: "DealsDamageToEach", template: "deals ${0} damage to each ${1}",
  kinds: [OneShotEffect], params: [Count, Predicate],
  body: Each(binder: Existing(SelectAll(Param(1))), effect: DealDamage(This, Param(0), It)) )
```

**block-trigger** — `Blocks`/`ThisBlocks` (`BlockDeclared(by:)`) +
`BecomesBlocked`/`ThisBecomesBlocked` (`BlockDeclared(of:)`), mirroring `event/Attacks`.

**bounce-to-library** — `PutOnLibrary` (`Move(_, Library(FromTop(0)))`) /
`PutOnBottomOfLibrary` (`FromBottom(0)`); the template supplies the "its owner's
library" possessive.

**loot-rummage** — `Loot` (`Sequentially([Draw, Discard])`) / `Rummage` (reverse);
template plural codec `${0:card|cards}`. Supplies the intra-sentence ", then " split.

**fight-bite — `Bite`** (one-sided fight; source spelled explicitly for
deathtouch/lifelink attribution): `DealDamage(Param(0), StatOf(Param(0), Power), Param(1))`.

**pt-equal-to-count — `PowerAndToughnessCountEqual`** (CDA over the layer engine's
dynamic `Set`): `Static(Modify(This, Several([Power(Set(CountOf(Objects(f)))), Toughness(Set(...))])))`;
`PowerCountEqual` is the power-only sibling.

**enters-with-counters — `EntersWithCounters`** (the inlined self-replacement
`Static(Replacement(Also(would: ThisEnters, also: PutCounters(This, P1P1Counter, ${0}))))`
— must be inlined, since a macro body can't thread its own Param into a nested
macro slot; same constraint as Modular/Graft).

**ltb-trigger** — `LeavesBattlefield` (`ZoneChange(what:, from: Battlefield)`, `to`
omitted = any zone) / `ThisLeavesBattlefield`.

**type-change-animate — `BecomesCreatureStillLand`** and siblings
(`Continuously`+`Modify` with `CardTypes(Add(Creature))` to keep Land); siblings
cover "loses all abilities", base-P/T set, land-type add.

## Notes / nuances

- **graveyard-recursion (566)** is NOT a card-authored macro — the subject-type
  filter is a variable noun phrase a fixed template can't carry (exactly why the
  shipped graveyard-to-hand path is a Rust parser arm). Add a battlefield twin to
  `parse_return_to_hand` in `crates/deckmaste_migrations/src/parsers/effect.rs`,
  reusing `graveyard_card_filter`, swapping `Hand`→`Battlefield` + the
  `UnderOwnersControl` rider (and `WithCounters` for "with N +1/+1 counters").
- The `⚠` trigger-wrapper families graduate a card only when its *payload* also
  parses; expect the realized graduation to fall short of `sole`.
- The 18 **already-supported** families (biggest raw pool — etb/destroy/draw/
  token/etc.) are NOT in scope: they need *broader parsers* for harder phrasings,
  not new macros — a separate coverage effort.

## Done

- Each family's headline macro(s) registered with a render template + typed params
  + an acceptance card that graduates AND round-trips (parse⇄render).
- The graveyard-recursion battlefield arm lands (parser, not macro).
- Graduation count strictly rises; record before/after per sub-batch in completion notes.
- Renderer output for every new macro passes the fidelity gate.
- The `ReturnToHand` core-verb-vs-macro question is resolved deliberately (one home, not two).

## Verification

- `cargo test --workspace` green; `cargo xtask validate` clean.
- Wipe-first wizards regen (`rm -rf plugins/wizards && cargo xtask generate plugins/wizards`);
  graduation count strictly greater than before this ticket.
- `cargo xtask fidelity plugins/canon` green; any new canon acceptance cards blessed.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check` 0 stale
  for any rules the new macros cite.
