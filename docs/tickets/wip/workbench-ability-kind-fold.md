---
needs: [workbench-copies-are-spells]
---
**Fold `Kind.Ability` into `Object` with an `AbilityP` payload and zone Stack,
and shrink the join family to the object case.** Fresh workbench review
2026-09-03, R2, resolved by ruling.

**Ruling (settled 2026-09-03): the grammar has one object.** An ability on the
stack is in the stack zone [CR#405.1] and is an object [CR#109.1], so
`Words.Kind.Ability` is not a third entity beside `Object` and `Player`. It
becomes `Object` carrying `Words.Payload.AbilityP`, with
`Words.payloadZone (AbilityP _)` returning `Just Stack` rather than `Nothing`.
The review recommended keeping the split and recording the divergence; the
ruling goes the other way, and the divergence disappears instead.

What collapses with it:

- `Phrase.NounWord.JoinW`, `AbilityJoinW`, `CopyJoinW` and `UnionHalf` — four
  join-reading words that exist only because "spell or ability" is a join of
  two kinds. With one kind they are one word over one payload.
- `Phrase.StackActOn` and its `StackAbility` case (`Phrase.idr:2194–2208`,
  with `Counterable` and `Copiable` over it): the ability case is the object
  case with a stack zone, so the zone conjunct already carries it.
- `Words.unionPayload`'s `AbilityP`/`ObjectP` cross-clauses
  (`Words.idr:1461–1467`) and `Phrase.setZone`'s `Ability` no-op clause
  (`Phrase.idr:2613`).
- The `PhAbility` cases in `Phrase.joinHalfPayload`, `Phrase.bindFor` and
  `Phrase.elemPayload`, and `Phrase.IsManaAbility : Predicate bs Ability`,
  which becomes `Predicate bs Object` gated on the payload.

An ability still cannot be moved and has no printed characteristics; those
refusals move from the kind index to the payload, and every pin that names
them must keep refusing for a payload reason rather than a kind reason.

Depends on `workbench-copies-are-spells`: both touch `Words.Payload` origins
and the stack-reading words, and the copy exclusion should already be out
before the join family is rewritten.

Size: L.

Also (copies-are-spells residue): `Words.wordReaches AbilityW` still carries `not (isCopyOrigin og)`, the same divergence from [CR#707.10] ("a copy of an ability is itself an ability") that the spell row just dropped; drop it with the fold and pin the plural ability read after a copy.

Done when: `Kind` has no `Ability` constructor; "counter target spell or
ability" and "copy target activated ability" are typechecking bench witnesses
reading one kind; `JoinW`/`AbilityJoinW`/`CopyJoinW`/`UnionHalf` and
`StackActOn` are gone or reduced to the object case; the pins that refused
moving or animating an ability still refute, re-spelled against the payload
and each probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

Constructor counts (before → after): `Words.Kind` 9 → 8; `Words.NounWord`
14 → 13; `Words.Reach` 8 → 8 (unchanged — see STOP 1).

1. **`Kind.Ability` → `Object` + `AbilityP`.** `Kind` loses `Ability`
   (`kindIx (_ \/ _)` renumbered 8 → 7; the `Ability` clauses of
   `sameKindRefl`, `kindLteInL/R`, `kindLteRefl` go); `AbilityP : Payload
   Object`; `payloadZone (AbilityP _) = Just Stack` [CR#405.1]. Kind-indexed
   sites re-spelled to `Object`: `Effect.counterHolderKind`,
   `GainsAbilitiesOf`/`LosesAllAbilities` except-predicates,
   `Triggers.Activates`/`Triggers`, `Macros.itIsntAnAbility`, the
   `AbilityActivation`/`BecomesTarget` event kind pairs (the latter deduped
   from four pairs to two), and the `MkDeedRole [Ability]` roles of "Activate"
   and "Trigger" (now `[Object] … (Just Stack)`).
2. **Join family.** `NounWord.AbilityJoinW` and `CopyJoinW` are gone; the one
   surviving word over one payload is `StackW`, reaching any binding whose
   payload zone is Stack. `CopyW` absorbs `CopyJoinW`'s reading;
   `AbilityCopyW` keeps the ability-copy reading. `unionPayload`'s
   `ObjectP`/`AbilityP` cross-clauses no longer build a `JoinP`: the union of
   a spell and an ability is one stack-zone `ObjectP`. "Spell or ability" as a
   description is now an ordinary `Or` of two `Predicate bs Object` arms, not
   a `Joined` of two kinds (9 bench sites; Disallow's nested join flattened to
   the printed three-arm `Or`). `JoinW`/`UnionHalf` stay — STOP 1.
3. **`StackActOn`.** `StackAbility` deleted; `StackSpell`'s
   `ZoneIs (nounZone n) Stack` carries the ability case, because the four
   ability predicates now seed zone Stack. `Counterable`/`Copiable` unchanged.
   `CostSubject.AbilityCostSubject` deleted — `MkCostSubject`'s
   `costSubjectOk` (on-stack) admits abilities. `counterKind`/`controlKind`/
   `copyKind` lose their `Ability` clauses. `setZone`'s no-op clause survives,
   keyed on the `AbilityP` payload rather than the kind.
4. **`IsManaAbility` and the payload gates.** `AbilityHead`, `AbilityOf`,
   `ActivatedBy`, `IsManaAbility` are `Predicate bs Object`, all seeding zone
   Stack. New `Phrase.seedsAbility` (+ `seedsAbilityAny`/`All`, the
   `seedsToken` idiom) decides the payload: `bindFor PhObject` emits
   `AbilityP Nothing` when the description seeds an ability. `Phrasal.PhAbility`,
   `Targetable.AbilityTgt` and `Targeter.AbilityTargets` are deleted, so
   `Phrasal Object` stays single-constructor and no bench witness needs a
   `{ph = …}` brace. `Phrase.nounIsAbility` (description + reading word, never
   the antecedent stack, so it reduces under abstract bindings) feeds
   `copyPayloadIn` and `elemPayload`, which now take the flag.
   `Events.deedAbilityRole`/`deedAbilityOk` and a `Bool` argument on
   `deedFits`/`DeedFits` replace the `[Ability]` deed roles.
   `Phrase.DealtThisWay` gains `nz : So (not (zoneIsB (seedZone p) Stack))`.
5. **`AbilityW` copy exclusion.** `wordReaches AbilityW` drops
   `not (isCopyOrigin og)` [CR#707.10]; `copyPayloadIn` gives an ability's
   copy an `AbilityP (Just CopyOrigin)` payload so `AbilityW` and
   `AbilityCopyW` both reach it.
6. **Bench and pins.** See the assurance counts below.

Undone: the `Move` gate against an ability subject — STOP 2.

## Landing record

Gates (all foreground, from the workspace):

- `cd idris && ./scripts/build` — `44/44: Building Cards (src/Cards.idr)`;
  0 `Error` lines, 0 `Warning` lines.
- Clean-tree full build: 2m01.5s before; after, two runs of 2m20.2s and
  1m29.5s — machine variance swamps the difference, so read this as "no
  measurable change".
  `Experimental.Phrase` check (its deps warm, its own ttc removed): 23.9s
  before → 13.0s after. `Experimental.Words` check: 3.6s before → 3.1s after.
- `cargo xtask cite check --list-noncompliant` — `2 non-compliant
  citation-looking string(s)`, both pre-existing prose in
  `docs/tickets/done/ability-kind-taxonomy.md`, neither in this diff.
- `cargo xtask cite bless` — `newly registered rule(s)` listed exactly one,
  [CR#113.1c] ("An ability can be an activated or triggered ability on the
  stack. This kind of ability is an object."), matching its citing claims; no
  prune (`cr-citations.lock` diff is `1 insertion(+), 0 deletions(-)`).
- `cargo xtask cite check` — `checked 18243 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/akf.diff && cargo xtask cite audit --diff <
  /tmp/akf.diff` — `audited 10 citation site(s)`; each read against its rule:
  [CR#109.1] (an ability on the stack is an object), [CR#113.1c] (that kind of
  ability is an object), [CR#405.1] (it goes on the stack with no card
  associated with it), [CR#707.10] (a copy of an ability is itself an
  ability). All four support, rather than undercut, the claims citing them.
- `grep -nw "Ability" idris/src/Experimental/Words.idr` — empty.
- `grep -on "AbilityJoinW\|CopyJoinW" idris/src/Experimental/*.idr
  idris/src/Experimental/Cards/*.idr` — empty. `JoinW` and `UnionHalf` remain
  (STOP 1).

Assurance counts — restored 0, re-spelled 8, ignored 0, added 6, removed 0.

- Re-spelled: `badAbilityJoinAnaphorOnPlayerUnion` →
  `badStackAnaphorOnPlayerUnion` (same sentence, pinned at the noun over
  `StackW` because `CounterSpell` would open a second hole);
  `badAbilityJoinAnaphorOnAbility` → `okStackAnaphorOnAbility` (same sentence,
  outcome flipped from refusal to admission by the ruling — with one kind,
  "that spell or ability" legitimately reads an ability antecedent);
  `spellOrAbilityJoin`/`abilityUnderSpellOrAbility` →
  `spellOrAbilityPayload`/`abilityIsOnTheStack`; `badDealtThisWayAbility`
  (same name, pinned obligation moved from the kind gate `rk` to the new
  stack-zone gate `nz`); `badCoinCameUpOnAbility` → `badCoinCameUpOnOutcome`
  (same asserted outcome and same obligation `rk`, new subject);
  `markTyKeepsOnes`/`markTyKeepsAt` ability clauses (`Ability` → `Object`);
  `badPlayerTargeter` and `badPlayerTargetingEvent` each lose their
  `AbilityTargets impossible` clause with the constructor, and stay pinned by
  `SpellTargets impossible`.
- Added: `okStackAnaphorOnAbility`, `okPluralAbilityReadAfterCopy`,
  `badSingularAbilityReadAfterCopy`, `okAbilityCopyReadAfterCopy`
  (ProofsAnaphora), `okCounterAbility` (ProofsZone), `abilityIsOnTheStack`;
  plus the `afterAnyTargetDamage` context helper.
- Removed: no pin or witness deleted. Four clauses vanished with the
  constructors they enumerated (`countOutcomesIsFold` and
  `countQuantOutcomesIsFold` `Ability` clauses; the two `AbilityTargets`
  impossible clauses above).

Non-vacuity probes — every pin added or re-spelled, and every pin that
refused an ability by kind, was mis-stated once and the message changed:

- `badStackAnaphorOnPlayerUnion` → read `JoinW` instead: `Refl is not a valid
  impossible case`.
- `badSingularAbilityReadAfterCopy` → read `AbilityCopyW` instead: `Refl is
  not a valid impossible case`.
- `badDealtThisWayAbility` → `DealtThisWay AnyPlayer`: `Oh is not a valid
  impossible case`.
- `badCoinCameUpOnOutcome` → at kind `Player`: `Oh is not a valid impossible
  case`.
- `badActivatedSpellClass` → deed "Cast": `Oh is not a valid impossible case`.
- `badCastAbilityClass` → deed "Activate": `Oh is not a valid impossible
  case`.

The moving/printed-characteristics refusals: `setZone`'s `AbilityP` clause
(an ability's zone never changes) and `markTy`'s `ObjectP Nothing`-only rewrite
(an ability takes no printed type) both survive as payload clauses, and both
are proof-covered by `markTyKeepsOnes`/`markTyKeepsAt`. No bench pin named
either before this round, and none names them now — see STOP 2.

### Deviations and additions

Beyond the ticket's letter, all forced by the fold:

- `Phrase.seedsAbility` + `seedsAbilityAny`/`seedsAbilityAll` and
  `Phrase.nounIsAbility`: with `Phrasal.PhAbility` gone, something has to pick
  `AbilityP` over `ObjectP`, and `Phrasal Object` must stay
  single-constructor or every `{auto ph}` site becomes ambiguous and needs a
  brace. The description (and, for anaphora, the reading word) decides.
- `seedZone` clauses giving the four ability predicates `Just Stack`: this is
  what lets `StackSpell`'s zone conjunct carry the ability case, as the ticket
  requires.
- `Events.deedAbilityRole`/`deedAbilityOk` and the `Bool` argument threaded
  into `deedFits`/`DeedFits` (22 call sites, mechanical): without it,
  `badActivatedSpellClass` and `badCastAbilityClass` both stopped refusing,
  because `[Ability]` was the only thing separating "Activate"'s patient from
  "Cast"'s.
- `Phrase.DealtThisWay`'s `nz` gate: same reason for
  `badDealtThisWayAbility`.
- `Joined` was briefly given a `Joins ka kb k` witness so "spell or ability"
  could sit at one kind; that left `Joins Object Object ?k` unsolved wherever
  the kind is free (`OneDefender`) and stuck `badMixedAttackDefenderHalves`.
  Reverted: `Joined` is untouched and stays the two-kind join, and
  "spell or ability" is spelled `Or` instead — which is also what the printed
  text reads.
- `Words.copyPayload` kept (already unused before this round) with the new
  arity rather than deleted.

### STOP 1 — `JoinW` and `UnionHalf` do not belong to the ability join

The ticket's letter says `JoinW`, `AbilityJoinW`, `CopyJoinW` and `UnionHalf`
are "four join-reading words that exist only because 'spell or ability' is a
join of two kinds". That is true of `AbilityJoinW` and `CopyJoinW`, both
`Object \/ Ability`. It is not true of the other two: `kindOfW JoinW =
Object \/ Player`, `wordReaches JoinW` tests `kindLte Player kd`, and
`halfReaches` — the whole of `UnionHalf`'s machinery — handles only `TypeW`,
`PermanentW` and `PlayerW`. They spell "any target" and "that permanent or
player" (`Macros.thatJoin`, `Macros.anyTargetController`,
`okUnionAnaphorAfterJoin`, `Cards/Anaphora.idr:308`), which the fold does not
touch because `Player` remains a separate kind. Current shape kept for both;
the `AbilityJoinW`/`CopyJoinW` half of the collapse is done. Consequence: the
ticket's shape gate ("only the surviving single word") cannot read literally —
three join-reading names survive, `JoinW`, `UnionHalf` and the new `StackW`,
and only the last is the ability one.

### STOP 2 — `Move` no longer refuses an ability subject

`Move : (what : Noun bs Object) -> …` excluded abilities by the kind index
alone. With the fold, `Move (Macros.target (AbilityHead AnyActivated))
Macros.exileZ []` typechecks — verified against a scratch module — so "Exile
target activated ability" is now spellable, contradicting the ticket's "an
ability still cannot be moved". The correct fix is a payload gate,
`{auto 0 nb : So (not (nounIsAbility what))}` on `Move`; it was written and
reverted, because `what` is an abstract variable inside every `Macros` move
wrapper, so the obligation has to be threaded through 16 of them (`move`,
`destroy`, `exile`, `exileWithCounters`, `returnToBattlefieldWithCounters`,
`putOntoBattlefield` and its three variants, `sacrifice`, `discard`,
`meldInto`, `returnTo`, `returnToBattlefieldTransformed`, `shuffleInto`,
`puts`, `exileUntil`) and then out to their callers, and it additionally
produced `Multiple solutions found in search of` in `scry`, `fateseal` and
`surveil`. That is a round of its own, past this ticket's letter and its
region. Current shape kept; the hole is open and needs a live ticket
("gate `Move` against an ability subject, threading the obligation through the
`Macros` move wrappers").
