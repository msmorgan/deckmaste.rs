---
needs: []
---
# Resolution-time riders and guards: "can't be regenerated", countering abilities, current-state guards, stopping a delayed trigger

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#3 (Lean side), #13, #23, #56. Verified against head 2026-10-21. Latent in Lean except where a RON
mirror is named.

Boundary: `docs/decisions/semantics-v3.md` §0.1 item 4 (the checker models text meaning and rule data; runtime state is the engine's); triage in
[lean-v3-checker-runtime-state-audit](lean-v3-checker-runtime-state-audit.md).

## 1. "It can't be regenerated" applies to the destruction it rides on

`destroyCantRegenerate` (`Definitions/Actions.lean:139-140`) is `doAndForbid (destroy subject)
(regeneration …)`, and `doAndForbid` (`Templates.lean:1462-1463`) is `.sequentially [action,
.establish (.cant .any forbidden {}) (some untilEndOfTurn)]`. So the destroy happens first and a
regeneration shield replaces it [CR#614.8] before the prohibition exists. The CR names this text:
"'Destroy target creature. It can't be regenerated' … Don't just apply effects step by step"
[CR#608.2c], and such effects "cause regeneration shields to not be applied" [CR#701.19c]. The
printed rider has no duration. Sites: *Terror*, *Snuff Out*, *Wrath of God*, *Damnation*
(`Cards/Deontic.lean:472,481,487,493`), `Cards/Piles.lean:191`; mirror `doAndForbid.ron`.

The engine is right (`crates/deckmaste_engine/src/state.rs:401-411`, a `ForThisEvent`
`Cant(Regenerate)` rider). Lean's `Duration` (`Abilities.lean:183-187`, `between | forAsLongAs`)
has no per-event scope, so add one: a rider on the destroy event with no duration, mirroring the
engine. Keep `doAndForbid` with `untilEndOfTurn` for printed "this turn" forms. Rewrite the pin
`destroyCantRegenerateIsDestroyThenProhibition` (`Proofs/Deontic.lean:278-281`) to pin the rider
(it asserts the wrong order, so it is rewritten, not retired; see
[lean-v3-proofs-and-docs-alignment](lean-v3-proofs-and-docs-alignment.md) §2). Add a pin that a
shield created before the destroy does not apply.

The pin that a shield created before the destroy does not apply is runtime simulation: out of
scope for Lean, it belongs to engine tests (ADR §0.1 item 4). The rider's denotation stays.

## 2. A countered ability ceases to exist; only a spell moves to a graveyard

`counter` (`Definitions/Actions.lean:32-38`) is always "one labeled stack-to-graveyard move", even
for abilities (*Stifle*, `Cards/Damage.lean:1362-1366`; pin `okCounterAbility`,
`Proofs/Zone.lean:566-568`). Countering a spell or ability is "removing it from the stack", and
only "A countered spell is put into its owner's graveyard" [CR#701.6a]; an ability on the stack is
no card and has a controller, not an owner [CR#113.8], as a resolved ability simply "ceases to
exist" [CR#608.2n]. The engine already makes a countered ability
cease (`crates/deckmaste_engine/src/resolve/action.rs:117-125`).

Change: make `counter` depend on its subject: a spell moves to its owner's graveyard, an ability
ceases with no zone move. Pin that *Stifle* introduces no graveyard referent. [CR#404.1]'s "Any
object that's countered" is looser than [CR#701.6a]; record that as an underdetermined wording
(a UD entry in `docs/engine-adrs.md`) and follow [CR#701.6a].

## 3. A resolution-time guard reads current state (fight)

The fight Definition's guard (`Definitions/Actions.lean:182-197`, `doIf (and [is (…fight.other…)
isCreature, is (…fight.source…) isCreature]) …`) is right only if `is X isCreature` means "is a
creature on the battlefield now". [CR#701.14b]: if either creature is "no longer on the
battlefield or [is] no longer [a creature], neither of them fights or deals damage". No contract
text says whether a `doIf` guard reads current state or last known information:
`docs/decisions/semantics-v3.md:179-189` licenses an LKI characteristic read of a departed
permanent [CR#608.2h] and restricts only action slots to current state, so the guard is ambiguous.

Change: state in `lean/CONTRACTS.md` and the ADR that a resolution-time guard on an object's
presence or type tests current state, with fight [CR#701.14b] as the witness and a pin
(*Affectionate Indrik* leaving before its trigger resolves: no fight). The Rust evaluator defect
(`Condition::Matches` falling back to a snapshot) is
[engine-condition-lki-context](../critical/engine-condition-lki-context.md); this item only fixes
the Lean contract the lowering will read.

The *Affectionate Indrik* pin is runtime simulation: out of scope for Lean, it belongs to engine
tests (ADR §0.1 item 4). The contract statement stays.

## 4. Quenchable Fire's {U} is a Special Action, not an "unless" on resolution

*Quenchable Fire* (`Cards/Damage.lean:1055-1056`) is `delay (beginningOf .upkeep (some you)
nextTime) (instruct splitOverPlaneswalker (doUnless (instruct this_ (dealDamage (.lit 3)
thatJoin)) (mana [pip .blue])))`. Paying to stop a delayed trigger from triggering is a Special
Action, taken "any time they have priority" [CR#116.2c]; the ruling (2009-02-01): "If it
triggers, the player can't stop it by paying {U}".

Change: make the delayed trigger unconditional, and establish a Special Action for "that player or
that planeswalker's controller", available until the trigger would trigger, costing {U}, whose
effect stops that delayed trigger. This needs two things that do not exist: an Action that stops a
named delayed trigger from triggering, and a way to grant `specialAction` (`Templates.lean:1982`,
which takes a bare `Timing`) to a player other than the controller.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; `cargo test -p xtask -p deckmaste_semantics_v2`
and clippy for both; the CR citation gates.
