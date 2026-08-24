---
needs: [workbench-join-is-a-constructor]
---
# Re-derive the kind-indexed `Binding` under a joined kind

A `Binding` is `Kind`-indexed, and several of the union family's facts are today
*derived* from that index rather than measured. A lattice kind can index a
binding, so those derivations evaporate and the facts become measured silences.
This ticket re-derives the binding record under the joined kind and gives each
lost derivation an explicit home.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— the second of its three pinned execution questions. Every measured fact this
round must preserve is inlined below.

> **Superseded in part (2026-08-22):** `workbench-join-is-a-constructor`
> makes `\/` a `Kind` constructor and mints `JoinP : Payload a -> Payload b
> -> Payload (a \/ b)`, which is the joined payload this ticket was to
> design. What remains here is re-deriving the named witnesses below under
> that shape (`bindsNothing`, the `KindJoin`/`YouAnd` asymmetry, the
> union-narrowing container).

## What is lost and must be re-homed

- **`YouAnd`'s no-binding fact.** Today: "a `Binding` is
  `Kind`-indexed and this mention has two kinds in it, so there is no binding to
  leave" [CR#109.5]. Under any lattice kind the derivation is gone and what
  remains beside it is a *count*, not an argument. Needs an explicit
  `bindsNothing` witness.
- **`ForEachOf`'s Join-cell refusal**, whose ledger cites `YouAnd`'s settled
  argument by name — the same derivation, load-bearing one construction away.
- **The `KindJoin`-binds / `YouAnd`-doesn't asymmetry.** Today it falls out of
  *where* each lives: a `Predicate` head goes through `bindFor`, a `Noun` row
  writes its own delta. Under a lattice both are phrases at a joined kind and
  the difference needs a gate of its own.
- **`bindFor`'s `UnionP` branch** — the one branch that *chooses* a payload
  rather than filling one in. Under a lattice the payload follows from the kind
  and the branch stops being a choice, **except** for the collapse rule that the
  shared payload actually encodes; that part belongs to
  `workbench-unhomed-union-gates` and must not be re-invented here.

## Also in scope — the one queued construction this unblocks

**The union-narrowing container** (Screaming Nemesis, 1 sentence): "If a player
is dealt damage this way, they can't gain life" — the damage went to a phrase
spanning both kinds and the container picks out the player case and binds it for
`They` to read. Under a joined kind this is a kind refinement on a binding, which
is exactly this ticket's machinery; land the cell here or record why it does not
fall in.

## Ledger from `workbench-join-shape-flat-or-pair`

- The kind index landed flat: `ObjectOrPlayer`, joined by `\/` under
  `So (joinable a b)`. It has **no `Payload` constructor yet** — that is this
  ticket's `Payload ObjectOrPlayer`, carrying the Object-half type the 33/33
  echo reads and the player-side restriction; `UnionP` retires with it.
- `lookbackSubjectOk _ ObjectOrPlayer = False` (`Events.idr`) was forced by
  coverage and is deferred, not decided; revisit once the payload exists.
- `joinAssoc` states associativity given all four gates; definedness closure
  (`ab ∧ abc ⟹ bc ∧ a_bc`) is unstated. Harmless while `joinable` is fixed;
  any gate widening must add it.

## Consumption boundary

`idris/src/Experimental.idr` (`Binding`, `bindFor`, `ForEachOf`, the noun
deltas), `idris/src/Experimental/Words.idr` (`Payload`), and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate is touched.

## Acceptance

- Every fact in the list above is either derived from the new index or carried
  by a named witness with its measurement quoted at it (`YouAnd`'s player half
  is 35/35 `You`, and no supported sentence reads the group back).
- No measurement is dropped: a measured zero that loses its derivation and gains
  no gate is a defect, not a simplification.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

The ticket predates `workbench-join-is-a-constructor` and
`workbench-union-family-macros`. Six of its seven items are audits of work
those two rounds already did; one item (the union-narrowing container) was the
only code this round wrote.

| Item, as the ticket wrote it | Verdict | Where the fact lives now |
| --- | --- | --- |
| `Payload ObjectOrPlayer` / retire `UnionP` | **Superseded.** `ObjectOrPlayer`, `joinable`, `UnionP` and `badJoin*` are absent from `idris/src/Experimental*` | `JoinP : Payload a -> Payload b -> Payload (a \/ b)` (`Words.idr:769`); the union mention carries what it knows about each half |
| `YouAnd`'s no-binding fact needs a `bindsNothing` witness | **Landed, verified.** `Refl`, not a count | `youAndBindsNothing` (`ProofsG.idr:373`): `nounDelta (Macros.youAnd Macros.thisCreature) = []`, docstring naming [CR#109.5] — "you" is deixis and mints nothing, and a coordination mints each arm's bindings and no third |
| The `KindJoin`-binds / `YouAnd`-doesn't asymmetry "needs a gate of its own" | **Wrong on the premise; no gate needed.** The asymmetry did not survive as a lattice ambiguity — it is a constructor-level split | `Macros.kindJoin who what = Joined what who` is a `Predicate`, so a determiner over it goes through `bindFor`'s `PhJoin` row (`Experimental.idr:1187`) and mints ONE `JoinP` binding at `ka \/ kb`. `Macros.youAnd n = Both You n` is a `Noun`, and `nounDelta (Both l r) = nounDelta r ++ nounDelta l` (`:1351`) mints each arm's own and writes no third. Argument recorded in `Both`'s docstring |
| `bindFor`'s `UnionP` branch — "the one branch that chooses a payload" | **Superseded.** It fills in rather than chooses | `bindFor det plur (PhJoin l r)` (`Experimental.idr:1187`) over `joinHalfPayload` (`:1165`): the kind fixes the shape, the description fixes the object half's type [CR#205.2a] |
| `lookbackSubjectOk _ ObjectOrPlayer = False`, deferred | **Superseded.** The deferred row is gone | `lookbackSubjectOk ev (a \/ b) = lookbackSubjectOk ev a && lookbackSubjectOk ev b` (`Events.idr:206`) — both halves, so "any target" cannot be what died but can be what took damage |
| `joinAssoc`'s unstated definedness closure | **Moot.** `joinable`, `joinIdem`, `joinComm` and `joinAssoc` are all deleted; a free constructor has no definedness side-condition to close | The laws are on the order instead, all total: `kindLteInL`/`InR`/`Refl`/`JoinL`/`JoinR`/`Comm`/`AssocR`/`AssocL`/`Trans` (`Words.idr:346..423`) |
| `ForEachOf`'s Join-cell refusal, "citing `YouAnd`'s settled argument by name" | **Argument dead; refusal kept, routed.** See the Ledger | `ForEachOf : (grp : Noun bs Object)` (`Experimental.idr:3384`) — a joined domain is a type error, not a gate, so there is nothing to pin |
| The union-narrowing container (Screaming Nemesis) | **Landed.** One `Condition` constructor, whole card benched | `Condition.DealtThisWay` (`Experimental.idr:1921`), `screamingNemesis` (`Cards.idr:4061`) |

### The union-narrowing container

Screaming Nemesis composes whole, not just the sentence:

```idris
Macros.card "Screaming Nemesis" (Just [Macros.generic 2, Macros.pip Red]) []
     (MkTypeLine [Spirit] [Creature])
     [ Macros.keyword Haste
     , Macros.triggered Whenever
                        (IsDealtDamage Macros.thisCreature)
                        (Sequentially
                           [ DealDamage It ThatMuch
                               (Macros.target (And [Macros.anyTarget,
                                                    OtherThan This]))
                           , If (DealtThisWay AnyPlayer)
                                (Continuously (PlayerCant GainsLife They)
                                              (Just RestOfGame))
                                Nothing ]) ]
     (Just (3, 3))
```

The pre-existing `screamingNemesisTrigger : Ability` (first sentence only) is
replaced by it.

**Second witness: Sonic Shrieker** (`Cards.idr`, `sonicShrieker`) — "When
this creature enters, it deals 2 damage to any target and you gain 2 life. If
a player is dealt damage this way, they discard a card." The "and" is a
`Sequentially` per house idiom, which puts `Macros.gainsLife You (Lit 2)`
between the damage clause and the conditional; the conditional reads past it
to the damage. It is the card that decided the gate's shape.

**One constructor.** `DealtThisWay : {k : Kind} -> (p : Predicate bs k) -> …
-> Condition bs`, with `condDelta (DealtThisWay _) = []` and
`condNegated (DealtThisWay _) = False`. Two erased gates, both probed:

- `So (damageDealtInScope bs)` — new in `Words.idr`, an **existence** test:
  some `DamageDealt` outcome mention anywhere in scope licenses "this way".
  [CR#608.2c] is the rule that makes the reading available ("later text on
  the card may modify the meaning of earlier text"), one of its two worked
  examples being a "…this way" back-reference — and it settles no more than
  that. WHICH earlier instruction the phrase names is the reader's (and the
  engine's) resolution, not the grammar's, so no discrimination is attempted
  here. It is deliberately neither of the two tighter shapes: not `= 1`,
  because Screaming Nemesis carries two `DamageDealt` mentions at the
  conditional (the trigger's event left one, its own damage clause the
  other); and not a head test, because Sonic Shrieker's life-gain clause
  stands between the damage and the conditional and displaces the head — a
  head test **refuses a printed card**, which the pins ruling makes the worst
  outcome.
- `So (kindLte k (Object \/ Player))` — the description is bounded by what
  damage can be dealt to [CR#120.1]; a refinement at any other kind is a
  category error.

**It introduces no binding, and does not need to.** The ticket asked for a
condition that "picks out the player case and binds it for `They` to read".
Under the landed readers it binds nothing: `They`'s own gate is
`countOnes Player bs = 1`, `countOnes` asks `kindLte`, and
`kindLte Player (Object \/ Player)` is `True`, so `They` lands on the joined
target binding directly and reads it at its player half [CR#115.1]. Minting a
`Player` binding here would have made `countOnes Player` **2** and broken the
card. What the condition supplies is not the referent but the licence: the
union-family round left `They`-on-a-join as a read that presupposes the target
was of that half's kind, and this condition is exactly what discharges that
presupposition. So the arm stays with the other described-set conditions that
"test, and name no referent".

**Pins minted: 2**, both in `ProofsG`, both rules-grounded per
`docs/memory/rulings/measurements-live-in-pins.md`:

- `badDealtThisWayNoDamage` — "You draw a card. If a player is dealt damage
  this way, …": "this way" reads the instruction it follows [CR#608.2c] and a
  draw dealt no damage anywhere in scope to read back — the licence is loose
  about *which* damage, but it still needs one. Probe (re-run after the
  loosening): `Can't find an implementation for So (damageDealtInScope [])`.
- `badDealtThisWayAbility` — "… deals 2 damage to any target. If a mana
  ability is dealt damage this way, …": damage is dealt to battles,
  creatures, planeswalkers and players [CR#120.1], and an ability is none of
  them. Probe:
  `Can't find an implementation for So False`.

### Ledger

**`ForEachOf`'s Join refusal has lost its argument and is now unargued.** The
closure table's cell (`docs/idris-workbench-closure-tables.md:745`) reads
"Join (1 line, Kaboom!, no binding possible per `YouAnd`'s Kind-indexed
`Binding` argument)". That derivation is gone twice over: a lattice kind
indexes a binding fine (`bindFor`'s `PhJoin` row proves it), and `YouAnd`'s
own no-binding fact turned out to be about coordination, not about the index.
What refuses a joined domain today is only `grp : Noun bs Object` — a type
error, so unpinnable, and per the pins ruling a restriction that leaves a
printed line unrepresentable is the worst case. **Two lines are attested**,
not the table's one, and both read the element back at the joined kind:

- Kaboom! — "Choose any number of target players or planeswalkers. For each of
  them, … deals damage … to **that player or planeswalker**, then …"
- Soulfire Eruption — "Choose any number of target creatures, planeswalkers,
  and/or players. For each of them, … deals damage … to **that permanent or
  player**."

Not widened here: `elemIntro` (`Experimental.idr:1376`) builds an `ObjectP`
from `nounTy`/`nounZone` directly, so a joined domain needs it kind-indexed
over a `Phrasal k` the way `bindFor` is — the same generalisation
`workbench-turn-structure-and-procedures` already scopes for the **player**
element, and the two are one change. Routed there rather than re-scoped here;
that ticket's "For-each-player elements" section gains the joined cell.

**The licence is loose, and the mis-pairing it admits is named.** Nothing
orders the discourse's outcome mentions, so a text carrying two `DamageDealt`
mentions can write a refinement pointing at one damage clause and an anaphor
resolving against the other's recipient. Screaming Nemesis carries exactly two
(the trigger's own event, then its damage clause) and reads correctly, but the
grammar is not what makes it read correctly — [CR#608.2c] leaves "this way" to
whoever reads the card. Tolerated overgeneration, refused at the spelling
boundary; closing it would need the discourse to order its outcome mentions,
which nothing else asks for. `DealtThisWay`'s docstring carries this.

For the same reason `DealtThisWay` is writable in a damage trigger's body
before any damage clause of its own, referring to the triggering damage rather
than to damage this text dealt. Same tolerance, same boundary.

**The conditional form is a quarter of the "this way" family; the rest is out
of a `Condition`'s reach.** 40 distinct oracle lines write "dealt damage this
way". **10** write it as the finite clause this constructor spells ("If a
player is dealt damage this way…", "If a creature is dealt damage this way…",
"If a nonred permanent is dealt damage this way…"). The other **30** write it
as a bare participle modifying a noun — "a creature dealt damage this way",
"each player dealt damage this way", "players dealt damage this way", "the
number of opponents dealt damage this way" — which is a `Noun`-side surface,
not a `Condition` at all, and closest to the `TheVerbed`/`ThoseVerbed`
`ThisWay` marking that today has no `Damage` in `VerbName`. `DealtThisWay`
does not and should not reach them; the participle family is its own round.

**Not done, and why.** The ticket's acceptance asks that "no measurement is
dropped". Two of its inlined measurements are dead rather than dropped: the
33/33 demonstrative echo is already carried by `JoinP`'s docstring and the
five `Macros.thatJoin` benches, and `YouAnd`'s 35/35 player half is a spelling
fact that left with `workbench-union-gate-spelling-rehome` — per
`docs/memory/rulings/measurements-live-in-pins.md` a count is never a gate, so
neither is re-homed into source here.
