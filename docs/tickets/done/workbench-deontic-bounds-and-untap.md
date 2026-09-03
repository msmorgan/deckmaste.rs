---
needs: []
---
**Move the count bound into the `Deontic` carrier as a `Maybe` positional slot,
and re-spell the untap satellites as deontics.** Ruling 2026-09-02 on audit
item R3. Seven constructors delete.

Beside the one-carrier `Deontic` (May/Can't/Must/Gate × deeds × patient) sit
seven satellites:

- Count-bounded: `CantMoreThan who deed k p:295` (8 uses),
  `MayPlayAdditionalLands:416` (11), `MayBlockAdditional:421` (4),
  `MayVoteAdditional:427` (1).
- Untap: `DoesntUntap:289` (2), `MayDeclineUntap:286` (2),
  `UntapsDuringStep:292` (4) — while `Events.deedFacts` already has an "Untap"
  row (`:569`) that no bench line reaches through `Deontic`.

## The ruling

(i) **The bound goes INTO the carrier**, as a `Maybe` positional slot — v1's
shape (`DeonticAction::Block{count: Option<CountBound>}`) — not into a sibling
`DeedBound` row. The four count satellites delete.

(ii) **"Doesn't untap" is a deontic**: `Forbid ["Untap"]` over the existing
`deedFacts` "Untap" row, with `MayDeclineUntap` as the permissive compulsion
and `UntapsDuringStep` as the same deed under a step window. It is not an
event-side can't-happen. The `canthappen-vs-replaces-taxonomy` note records v1
treating it as `CantHappen(Becomes Untapped)` *and* v1 also carrying
`DeonticAction::Untap` — v1 is double-encoded here, and this ruling picks the
deontic side. The three untap satellites delete.

Do not reduce the carrier's arity by splitting it back into v1's eleven typed
verbs; the one-carrier ruling stands, and its 7 positional + 9 auto gates
(`Effect.StaticEffect:268-283`) are the price of it. The bench never writes it
raw — 16 macro carriers do.

Size: S/M.

Done when: build is 23/23; the seven named constructors are gone from
`Effect.StaticEffect`; `Deontic` carries the bound slot; the 32 bench witnesses
across those rows are re-spelled through macro carriers and still typecheck; a
bench line reaches the `deedFacts` "Untap" row through `Deontic`; a pin refutes
a bound on a deed whose facts row admits none, and it is non-vacuous. Standard
constraints apply.

## As landed

- `Deontic` gained the positional slot `bound : Maybe (CountBound (nomIntro n))` between `role` and `patient`, gated by `So (deonticBoundOk deeds bound)`; `Effect.CountBound` is `MoreThan (k : Amount)` | `Additional (q : Quantity)` (the latter carrying the `NonZeroQ`/`WellFormedQ` gates the four count rows had), `boundDelta` must be empty.
- `CantMoreThan`, `MayPlayAdditionalLands`, `MayBlockAdditional`, `MayVoteAdditional`, `DoesntUntap`, `MayDeclineUntap`, `UntapsDuringStep` are gone from `Effect.StaticEffect`, with their `staticKind`/`staticIntro` clauses and the `StaticKind` values `LandAllowance`, `BlockAllowance`, `VoteAllowance`, `UntapGrant`.
- `Words.ActFacts` gained `actBounded : Bool` (read through `Events.deedBoundedOk`), true for the recurring player/object acts; a "Vote" row (agent Player, no patient) was added [CR#701.38].
- Macros, one per lemma: `cantMoreThan who deed k p` (Forbid + `MoreThan (Lit k)`, counterpart `AllOf p`), `mayPlayAdditionalLands who q`, `mayBlockAdditional n q`, `mayVoteAdditional who q` (Permit + `Additional q`), and the untap trio `doesntUntap n w` (Forbid), `mayDeclineUntap n w` (Permit), `untapsDuring n w` (Require), each `Forbid/Permit/Require ["Untap"] Patient` under `OnlyDuring UntapStep w`; the 19 raw `Deontic` sites and `Macros.deontic` pass `Nothing`.
- Bench: the 32 witnesses re-spelled through those macros (Time Vault, Mungha Wurm, Damping Field, Smoke, Winter Orb, Static Orb, Rule of Law, Spirit of the Labyrinth, Winter Moon, Ashnod's Battle Gear, Helm of Possession, Bombur, Exploration, Oracle of Mul Daya, Azusa, Summer Bloom, Explore, Rites of Flourishing, Dryad of the Ilysian Grove, Fastbond, Nahiri's Lithoforming, Seedborn Muse, Unwinding Clock and the two other untap-step lines, Foriysian Brigade, Two-Headed Giant, High Ground, Watcher in the Web, Ballot Broker, `eachPlayerPlaysAdditionalLand`); the untap-step lines now reach the "Untap" act row through `Deontic`. New printed witness `battlefrontKrushokEvasion` (Battlefront Krushok, "can't be blocked by more than one creature": Patient-side `MoreThan` with a `DeonticCounterpart`).
- Pins: new `ProofsG.badCounterBoundTwice` (a bound on "Counter", whose row admits none; [CR#701.6a]); re-spelled `Proofs.badUntapLockGraveyard` (through `doesntUntap`, hole `zn`) and `ProofsD.badUntapCapGraveyardSet` (through `cantMoreThan`, hole moved to `pt` since the counterpart zone check now lives in `deonticPatientOk`). All three probed non-vacuous in a scratch module (deed → "Cast", noun → battlefield creature): each `Oh impossible` is rejected.
- Undone: nothing. `cr-citations.lock` unchanged ([CR#701.6a] was already registered; `bless` would only prune 23 entries no diff of this round touches, so it was restored).
