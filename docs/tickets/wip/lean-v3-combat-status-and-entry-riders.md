---
needs: []
---
# Blocked and Unblocked as combat statuses; ninjutsu, sneak and exalted read them correctly

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#2 (critical), #15, #30, #37. Verified against head 2026-10-21. Everything here is latent (bench,
proofs and the `plugins_v2` RON mirrors only). The engine half of the pre-blocks gap is
[engine-combat-block-status](engine-combat-block-status.md) §1; land the two with the same
reading.

## 1. `isUnblocked` is not `not isBlocked` (critical)

`lean/Semantics/Templates.lean:464-465`:

```lean
isBlocked := .inCombat .blockedBy none
isUnblocked := not isBlocked
```

Before blockers are declared an attacking creature is neither blocked nor unblocked. An attacker
"with no creatures declared as blockers for it becomes an unblocked creature", and "A creature
remains blocked even if all the creatures blocking it are removed from combat" [CR#509.1h]. The
CR glossary's "Unblocked Creature" is "An attacking creature once no creature has been declared
as a blocker for it". A creature removed from combat stops being blocked or unblocked
[CR#506.4].

Consequences at head:
- Sneak (`Definitions/Keywords.lean:1294`, `and [isCreature, isUnblocked, youControl]`) accepts a
  creature that is not attacking. [CR#702.190a] returns "an unblocked creature" during the
  declare blockers step.
- Ninjutsu (`Definitions/Keywords.lean:1045`, `and [isCreature, isAttacking, isUnblocked,
  youControl]`) already requires an attacker, but `not isBlocked` holds in the declare attackers
  step, so it can be activated before blocks [CR#702.49a].
- Forcefield (`Cards/Damage.lean:571`) admits creatures that are not unblocked attackers.

Change:
- Give Blocked and Unblocked their own combat-status reads. Blocked is set when blockers are
  declared and stays set while the creature is in combat [CR#509.1h,506.4]. Unblocked holds only
  for an attacking creature after the declare blockers step's declaration found no blocker for it
  (or one put onto the battlefield attacking after blocks [CR#508.4d]). Neither is the other's
  negation. Document the stickiness on `CombatRelation` (`Words.lean:144-149`), which says
  nothing about it today.
- Re-check every consumer: ninjutsu, sneak, Forcefield, `Cards/Static.lean:860`
  (`and [isCreature, isBlocked]`), `Cards/Turn.lean:56` (`.blockedBy (some thisCreature)`), and
  the RON mirrors `ninjutsu.ron:16` and `sneak.ron:25`.

Pins: an attacker in the declare attackers step is neither `isBlocked` nor `isUnblocked`; a
creature whose only blocker left combat stays `isBlocked`; sneak refuses a non-attacker.

## 2. Ninjutsu and sneak entry riders

- Ninjutsu (`Definitions/Keywords.lean:1039-1046`) ends in
  `putOntoBattlefieldTappedAttacking this_ yourHand`, which expands to `.entersAttacking none`
  (`Templates.lean:1167-1169`), leaving the attack target to the controller's choice [CR#508.4].
  [CR#702.49c]: the Ninja "enters attacking the same player, planeswalker, or battle as the
  creature that was returned". Bind the defender to the returned creature's attack target, read
  by last known information at the moment the cost is paid.
- Sneak (`Definitions/Keywords.lean:1286-1295`) is a bare `can you (cast thisSpell (payment :=
  payingInstead …))` with no Entry Rider, so the permanent does not enter tapped and attacking.
  [CR#702.190b] gives it "enters the battlefield tapped and attacking", attacking the same target.
  Add the rider, gated on the sneak cost having been paid (`keywordCostPaid`).
- Nothing in the repository cites [CR#702.49c] or [CR#702.190b] today. The Rust `Attacking(Option<Reference>)`
  (`crates/deckmaste_core/src/action.rs:67-70`) can already carry the defender.

## 3. Exalted's RON mirror still uses an intervening "if"

The Lean body was fixed (`Definitions/Keywords.lean:649-652`, `whenever (does you (attackWith
(counted (exactly 1) (and [isCreature, youControl])))) …`, no intervening "if"; "the only
creature declared as an attacker" [CR#702.83b]). The team-attack reading of Two-Headed Giant is
in the repository's out-of-scope multiplayer range (`cite-config.json`), so `youControl` stays.
The residue is the mirror:
- `plugins_v2/builtin/macros/keyword_abilities/exalted.ron:8-16` is still
  `triggeredIf(…, compareAmt(countOf(and([creature, attacking])), Eq, 1), …)` with the comment
  "'Attacks alone' is the intervening-if". That rechecks on resolution [CR#603.4] and counts
  creatures that are attacking, not declared [CR#506.5]. Port the Lean shape.

## 4. "Must be blocked if able" is one requirement on the declaration, not Lure

*Loathsome Catoblepas* (`Cards/Deontic.lean:134`), *Frodo Baggins* (`:144`) and *Enkira, Hostile
Scavenger* (`:209`) are `must (by_ (allOf isCreature)) (block thisCreature)`. `by_` is the
explicit-agent passive (`Templates.lean:133`, voice conventions `:1000-1003`), so this reads
"must be blocked by all creatures", the Lure shape. The CR has no "must be blocked" text; the
blocking requirements rule [CR#509.1c] and the rulings govern: *Frodo Baggins* "only one creature
is required to block it"; *Loathsome Catoblepas* doesn't "make any specific creature have to
block".

Change: write "must be blocked if able" as one requirement on the defending player's declaration,
met when at least one creature blocks (`must any (block thisCreature)` with set-level meaning, or a
counted form). Pin that it differs from the Lure tree. The engine's counted `Must(Block)` arm is
the `todo!` at `crates/deckmaste_engine/src/decide/pending/combat.rs:221-224`, owned by
[engine-combat-requirements](engine-combat-requirements.md); this ticket only fixes the Lean
encoding and its meaning.

## Verification

`./lean/scripts/build`; `cargo xtask lean check` (RON mirrors change); `cargo test -p xtask -p
deckmaste_semantics_v2`; the CR citation gates.
