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
