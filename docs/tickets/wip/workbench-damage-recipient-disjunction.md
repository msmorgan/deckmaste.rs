---
needs: []
---
**Admit a bare type disjunction as a damage recipient, through the same
head-type alternatives the deed gates already compute.** Fresh workbench
review 2026-09-03, F1.

`Phrase.DamageRecipient.ObjectTakes` demands `DamageableTy (nounTy n)`.
`nounTy (Described _ (Or ps))` is `seedTyJoin`, which is `Nothing` unless all
arms agree, and `DamageableTy Nothing` is `So False`. The join path does the
opposite: `JoinTakes` / `damageableKind Object (SoleTy Nothing)` is
*permissive* on `Nothing`. The same words are therefore admitted inside a
player join and refused alone.

Probes (`idris2 --check` against the core):

- P1 `DealDamage This (Lit 5) (target (Or [HasType Creature, HasType Planeswalker]))`
  (Fry) — **refused**: `DamageRecipient (Described … (Or …))`.
- P1b the same disjunction inside `Joined (Or …) AnyPlayer` — admitted.
- P1c `IsDealtDamage AnyDamage (a (Or [creature, planeswalker]))` — refused by
  the same gate.

82 supported cards print exactly "damage to target creature or planeswalker",
plus the `IsDealtDamage` triggers that read it.

Fix: replace the `nounTy` check with the alternatives already computed for
deeds — `all (all damageableType) (nounHeadTys n)`, with `[]` meaning
unknown-therefore-permissive — and have `damageableKind` read the same
function, so the lone path and the join path agree by construction instead of
by coincidence. `nounHeadTys` is already `List (List CardType)`, one entry per
disjunctive alternative (`workbench-zone-gate-unify`).

Size: S.

Done when: Fry is a typechecking bench witness and an `IsDealtDamage` trigger
over the same disjunction is a second one; `ProofsDamage` keeps its existing
refusals and gains a pin for a disjunction no arm of which can be dealt
damage, probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Words.damageableHeadTysOk : List (List CardType) -> Bool` replaces
  `Words.DamageableTy` (the `Maybe CardType`-keyed `So` type, now unused and
  deleted): `all (all damageableType)`, `[]` permissive at both levels
  (unknown-therefore-permissive, matching `deedHeadTysOk`'s idiom).
  `Phrase.ObjectTakes`'s `dm` obligation reads `So (damageableHeadTysOk
  (nounHeadTys n))` — the alternatives-aware function `deedNounOk` already
  uses — instead of the old `DamageableTy (nounTy n)`, which collapsed any
  type-disagreeing `Or` to `Nothing` and refused it regardless of whether
  every arm was damageable.
- `Phrase.damageableKind`'s `Object (SoleTy t)` case now reads the same
  function (`damageableHeadTysOk (soleAlt (optCT t))`), so `JoinTakes`
  (the two-kind join path, e.g. "target player or planeswalker") and
  `ObjectTakes` (the lone-object path, e.g. "target creature or
  planeswalker") agree by construction: both ask "is every head type
  alternative reachable from here damageable", not two independently
  permissive/restrictive spellings of what `Nothing` means. `soleAlt (optCT
  t)` reduces to the prior `maybe True damageableType t` exactly, so
  `JoinTakes`'s admitted set is unchanged.
- P1: Fry ("This spell can't be countered. Fry deals 5 damage to target
  creature or planeswalker that's white or blue.") is now a typechecking
  bench witness, `Cards.Damage.fry`. P1b (`Joined (Or …) AnyPlayer`, already
  admitted via `JoinTakes`) is unaffected. P1c is covered by
  `Cards.Damage.terminationFacilitator` — Termination Facilitator's trigger
  "Whenever a creature or planeswalker an opponent controls with a bounty
  counter on it is dealt damage, destroy it." typechecks through the same
  `IsDealtDamage AnyDamage / DamageRecipient` gate as `DealDamage`, so no
  separate probe was needed for the event path.
- `ProofsDamage.badDamageDisjunctHead` ("target artifact or enchantment",
  no arm damageable) keeps refusing, now for the stated reason instead of
  the old "types disagree" one; its positive twin `okDamageDisjunctHead`
  ("target creature or planeswalker", [CR#120.1]) is new, mirrors P1 exactly,
  and sits beside it in the same module. Every other `ObjectTakes
  impossible`/`JoinTakes impossible` pin in `ProofsDamage` (single head type
  or same-kind non-damageable join) is unchanged in shape and re-verified by
  the full rebuild.
- Non-vacuity: reverted `ObjectTakes`'s obligation to the old
  `DamageableTy (nounTy n)` (and restored `DamageableTy` alongside the new
  function) and rebuilt — `okDamageDisjunctHead` (and by the same shape,
  `fry`) fails to elaborate: `Can't find an implementation for
  DamageRecipient (Described (TargetDet …) (Or [HasType Creature, HasType
  Planeswalker]))`. Restored the fix; the diff before and after the probe is
  byte-identical (`jj diff` compared).
- Nothing was left undone; no STOP was needed.

## Landing record

- Numbers before/after. Gate spellings of "is every reachable head type
  damageable": 2 (`DamageableTy` restrictive-on-`Nothing` for the lone
  object, `damageableKind`'s permissive-on-`Nothing` `maybe True …` for the
  join) → 1 (`damageableHeadTysOk`, `[]` permissive at both levels, shared).
  `Words.DamageableTy`: removed (was used at exactly one call site).
  Construction count: 0 core constructors added or removed (only a `Bool`
  helper function changed shape).
- Gate lines. `cd idris && rm -rf build && ./scripts/build`: `44/44:
  Building Cards (src/Cards.idr)`, exit 0, 44 module lines, 0 `Error` and 0
  `Warning` lines (grepped). `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`. `cargo xtask cite check`:
  `checked 18166 citations against cr.txt (eff. 2026-08-07); 0 stale`.
  `cargo xtask cite bless`: `blessed 1583 rules …`, no diff to
  `cr-citations.lock` ([CR#120.1] was already registered). `jj diff --git |
  cargo xtask cite audit --diff`: `audited 1 citation site(s)` —
  `ProofsDamage.idr:205 [CR#120.1]` read against its rule text (damage to
  battles, creatures, planeswalkers, players) and confirmed on-topic.
- Assurance: restored 0; re-spelled 0 (no existing pin body needed a text
  edit — every prior `ObjectTakes impossible`/`JoinTakes impossible` pin
  keeps typechecking as a refusal under the new obligation type without
  changes); ignored 0; added 3 (`ProofsDamage.okDamageDisjunctHead`,
  `Cards.Damage.fry`, `Cards.Damage.terminationFacilitator`); removed 0
  tests/pins (`Words.DamageableTy` removed, but it was a type alias, not a
  test).
- Non-vacuity probed by reverting the fix (see As landed) for
  `okDamageDisjunctHead`/`badDamageDisjunctHead`'s shared obligation, which
  is the representative case for every other single-type `ObjectTakes`
  pin in the module (same obligation shape, unaffected by the `Or`-specific
  fix). `badSameKindJoinDamage` (`JoinTakes`, same-kind non-damageable join)
  was not touched by this round and was re-verified passing by the full
  rebuild rather than re-probed.
- Deviations and additions:
  1. `Words.DamageableTy` deleted (dead after the fix; its one call site was
     rewired to `damageableHeadTysOk`). Not named in the ticket, but the
     ticket's fix left it with no callers.
  2. `ProofsDamage.okDamageDisjunctHead` added as `badDamageDisjunctHead`'s
     positive twin (the pin-methodology rule in `VERIFY.md`: a pin is
     evidence only once its twin is kept beside it in the same module);
     the ticket's "gains a pin for a disjunction no arm of which can be
     dealt damage" is `badDamageDisjunctHead` itself, already present and
     now probed non-vacuous under the new gate rather than the old one.
  3. `Cards.Damage.terminationFacilitator` models the full printed card
     (both abilities — the bounty-counter-placement activated ability and
     the damage trigger) rather than just the trigger clause, since the
     bounty-counter vocabulary (`HasCounters`, `PrintedKind (Named
     "Bounty")`) was already available and a partial card would have left
     the counter-placement half of the printed text unrepresented for no
     reason.
  4. `DealtBy`/zone/possessor machinery used by `terminationFacilitator`
     (`HasPossessor ControllerAx Macros.anOpponent`, `HasCounters (Just
     (Named "Bounty"))`) is pre-existing vocabulary, used here for the
     first time in combination with `IsDealtDamage`; no new constructor or
     macro was added.
- STOP: none taken.
