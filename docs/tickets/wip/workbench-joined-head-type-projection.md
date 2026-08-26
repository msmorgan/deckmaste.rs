---
needs: []
---
# Project a joined noun's head type per half

The join landed as a general constructor and left three gates reading a joined
noun's head type through machinery that only ever saw one. All three residues
were found in review, none was in its round's scope, and all three want the same
missing piece: a per-half head-type projection over nouns.

## 1. `seedTy (Joined …)` prefers the left half and discards the right

From `docs/tickets/done/workbench-attackable-defender-join.md`:

> `Attackable` reads `nounTy`, and `seedTy (Joined l r)` prefers the left half
> and discards the right, so `attackableKind (a \/ b) t` re-checks the *same*
> `t` for both halves.

At `Object \/ Player` — every kind `Macros.kindJoin`, `Macros.anyTarget` and
`Macros.thatJoin` can build — that is exactly right, because the `Player` arm is
unconditional and the `Object` arm carries the whole check. A hand-written
`Joined` at `Object \/ Object` slips through: `Macros.a (Joined (HasType
Planeswalker) (HasType Creature))` is admitted as an attack defender while the
reversed order and `Joined AnyPlayer (HasType Creature)` are both refused. That
round's own diagnosis: "Closing it needs a per-half head-type projection over
nouns (`headTys` has no `Joined` case either) — a construction outside this
ticket."

## 2. A same-kind join bypasses `DamageableTy`

From `docs/tickets/done/workbench-union-family-macros.md`:

> `Joined` is general over two kinds, so `Joined (HasType Land) (HasType Land)`
> is a `Predicate bs (Object \/ Object)`, and `JoinTakes` admits it as a damage
> recipient with no zone and no type check — where the object-only phrase `And
> [creature, InZone graveyardZ]` is still refused by `ObjectTakes`.

Recorded there as the price of the general constructor rather than a decision
taken. It is the same hole seen from the `DamageRecipient` site: `JoinTakes`
asks nothing of either half. The attack gate is strictly stronger than the one
it mirrors, so fixing `seedTy`/`headTys` fixes the weaker site too — check that
it does rather than assuming it.

## 3. The kind-polymorphic slots the `AnyTargetFree` sweep opened

Same ledger: `CountOf`, `Aggregate`, `Exists`, `Each`, `Indefinite`, `Definite`,
`CountedGroup` and `AllOf` are kind-polymorphic, so each now admits a joined
predicate — `CountOf Macros.anyTarget` ("the number of any target")
type-checks. Recorded as tolerated overgeneration. Re-read it once the
projection exists: some of these may become refusable on the same evidence, and
the ones that do not should be named at their zero rather than left unmentioned.

## Not this ticket's

`lookbackComplementOk`'s refusal of every joined complement is routed to
[workbench-event-zone-and-cast-provenance](workbench-event-zone-and-cast-provenance.md),
which owns that table.

## Consumption boundary

`idris/src/Experimental.idr` (`seedTy`, `headTys`, `nounTy`, `Attackable` /
`attackableKind`, `DamageRecipient` / `JoinTakes` / `ObjectTakes`, the
kind-polymorphic phrase heads), `idris/src/Experimental/Words.idr` if the
projection needs a word of its own, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- A joined noun projects a head type per half, and `Attackable` checks each half
  against its own; the three admitted-by-accident terms above are re-probed and
  the outcome recorded for each.
- `JoinTakes` is re-read against the projection: either it checks its object
  half or its vacuity is argued from a rule.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

**The projection is a kind-indexed head-type tree, `Payload`'s twin on the
description side.** `HeadTy : Kind -> Type` (`Words.idr`, beside `Payload`)
has two rows — `SoleTy : Maybe CardType -> HeadTy k` and
`JoinTy : HeadTy ka -> HeadTy kb -> HeadTy (ka \/ kb)`. Where `JoinP` records
what a joined MENTION knows about each half, `HeadTy` records what a joined
DESCRIPTION names at each half, and the index makes the two walk in lockstep.

Three projections read it:

- `seedTys : Predicate bs k -> HeadTy k` — `Joined l r` forks, everything
  else is a leaf carrying `seedTy`. `And`/`Or` over a joined kind fold to one
  description: a fold names the whole phrase's head, not one half's.
- `nounTys : Noun bs k -> HeadTy k` — a determiner over a joined head passes
  the head's pair through, `Both` takes one from each arm, every other noun
  (anaphors included) is `SoleTy (nounTy n)`, since a binding remembers one
  type per mention.
- `joinHalfPayload : Phrasal k -> HeadTy k -> Payload k` — the pair replaces
  the single `Maybe CardType` it used to be handed, so `bindFor`'s `PhJoin`
  branch can no longer give one half the other half's type. A leaf at a joined
  kind is handed to both halves, which is what a phrase naming one description
  for the pair means.

**`seedTy`'s own `Joined` row stops preferring the left half.** The collapse
is `joinSeed`: the halves' type where they agree, the typed half's where the
other is silent, and none where they name DIFFERENT types. That is `Or`'s rule
for its disjuncts, weakened only where a half names nothing, so a cross-kind
head still projects its object half's type and only a same-kind disagreement
becomes `Nothing`. `headTys` gains `headTys (Joined l r) = headTys l ++ headTys r`,
mirroring `headTysJoin`. `payloadTy`'s `JoinP` row had the identical
prefer-left collapse and now calls the same `joinSeed` (which is why the `Eq
CardType` instance moved up in `Words.idr` — a pure relocation, no clause
touched).

**`Attackable` checks each half against its own head type.**
`attackableKind : (k : Kind) -> HeadTy k -> Bool`; the join row is
`attackableKind (a \/ b) (JoinTy l r) = attackableKind a l && attackableKind b r`,
and a leaf at a joined kind is asked about at every half. `attackableTy` and
`Attackable`'s shape are unchanged.

**`JoinTakes` checks its object half — the vacuity is not arguable.**
[CR#120.1] states the whole recipient set, so a half that names a card type
must name one on it. `damageableKind : (k : Kind) -> HeadTy k -> Bool` is
`attackableKind`'s twin over `damageableHalfTy`, which admits `Nothing`
alongside creature/planeswalker/battle: "a permanent or player" (Furnace of
Rath) names nothing off the set, and [CR#120.1a] bounds the referent. `JoinTakes`
gains `{auto 0 dm : So (damageableKind (ka \/ kb) (nounTys n))}`. The ticket's
guess held: fixing the projection fixed the weaker site, and the constructor
kept its `data` shape.

### Ledger

**Pins minted: 2, both in `ProofsG`. Pins retired: 0.**

- `badMixedAttackDefenderHalves` — "Whenever a creature attacks a planeswalker
  or a creature". [CR#506.3] closes the set an attack may name and the creature
  half is not in it.
- `badSameKindJoinDamage` — "This deals 3 damage to a land or a land."
  [CR#120.1]; the repetition is not what refuses it.

**The three admitted-by-accident terms, re-probed.**

- `OneDefender (Macros.a (Joined (HasType Planeswalker) (HasType Creature)))` —
  was ADMITTED, now REFUSED (`So False` at the `Attackable` gate). Pinned.
- The same halves reversed — REFUSED, as before, and now for the same reason
  rather than by the accident of which half `seedTy` preferred.
- `Joined AnyPlayer (HasType Creature)` — REFUSED, as before.

Still admitted, unchanged: `Macros.a (Macros.kindJoin AnyPlayer (HasType
Planeswalker))`, `Macros.a (HasType Battle)`, `Macros.target Macros.anyTarget`
as a damage recipient, and both benched `youAnd` shapes (their object halves
name no card type, or name `Creature`).

**The `DamageRecipient` widening, narrowed.** `Joined (HasType Land) (HasType
Land)` and `Macros.youAnd (AllOf (HasType Land))` are both refused now. The
widening that remains is the untyped object half, which is the attested case
and is argued from the rule.

**The kind-polymorphic slots, re-probed. No pin minted: none of them becomes
refusable on a rule.** The projection adds a gate only where a rule names a
closed set, and none of these slots names one — [CR#115.1] makes a phrase that
denotes objects and/or players meaningful, so a count, a quantifier or an
existential over one describes a real set.

- Still admitted, each named at its zero: `CountOf` (`CountOf Macros.anyTarget`),
  `Exists`, `Each`, `Indefinite`, `CountedGroup`, `AllOf`, and `CountOfGroup`
  (over a real group mention, `TargetGroup (exactly 2) Macros.anyTarget`).
- Two of the eight the prior sweep listed were NEVER admitted at a joined kind,
  and the list overstated them. `Aggregate` is refused by its own
  `{auto 0 sc : projScope ax = k}` — an axis scopes over one kind, so
  `Object = Object \/ Player` has no proof. `Definite` is refused by
  `Uniquifying`, whose only true case is `Superlative`, which carries the same
  axis-scope obligation; so `Definite` at a joined kind is unreachable. Both
  refusals predate this round and neither is the projection's, so neither is
  pinned here.
- `CountOfGroup`'s first probe (`CountOfGroup (AllOf Macros.anyTarget)`) is
  refused, but by `GroupMention`, not the join: `CountOfGroup (AllOf
  Macros.creature)` is refused identically.

**Gates.** `idris/scripts/build` 23/23 from a clean `build/`, 0 errors, 0
warnings. `cargo xtask cite check --list-noncompliant` 0; `cite check` 0 stale
over 16687 citations; `cite bless` registered no new rules (lockfile
unchanged); `cite audit --diff` read over all 10 sites, which produced one
correction — three [CR#120.1a] docstrings claimed the choice "settles at
resolution", a timing the rule does not state; they now say the rule bounds the
referent, which is what it says.

Diffstat: 3 files, +184/-68.

### Residue

`payloadZone`'s `JoinP` row still prefers the left half. It is sound today
because `joinHalfPayload` gives every joined half `Nothing` for its zone, so
no `JoinP` carries two zones; the row would need the same per-half treatment
if a joined phrase ever placed one of its halves.
