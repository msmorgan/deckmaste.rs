---
needs: [workbench-union-family-macros]
---
# Make the attack defender a joined-kind noun, gated by [CR#506.3]

`workbench-union-family-macros` gave the semantics a joined kind, but the
attack-defender slot did not move with it: `Attacks`'s `whom` is still
`Maybe (Noun bs Player)`, so no card can name a defender that may be a player
or a planeswalker. **Tahngarth, First Mate** is the per-site residue that
ticket recorded and left unlanded. Its oracle line:

> Whenever an opponent attacks with one or more creatures, if Tahngarth is
> tapped, you may have that opponent gain control of Tahngarth until end of
> combat. If you do, choose a player or planeswalker that opponent is
> attacking. Tahngarth is attacking that player or planeswalker.

## Scope

- `Attacks`'s `whom` becomes a joined-kind noun — `Maybe (Noun (nomIntro n) kd)`
  — so "that player or planeswalker" is an ordinary `Macros.kindJoin` term.
- The same change for `DefendingPlayer`.
- The increment the join does **not** supply: an `Attackable` gate carrying
  [CR#506.3]'s set — "Only a player, a planeswalker, or a battle can be
  attacked." Kind `Object` covers creatures, and a creature is never attacked,
  so the joined kind alone would overgenerate against a rule that names the
  set. This is a rules refusal, not a corpus one, and its docstring cites
  [CR#506.3].
- Bench Tahngarth, First Mate as the positive witness. A pin for the refused
  half (attacking a creature) names [CR#506.3].

The attackable gate is why this is its own ticket: it is a second
construction, not a consequence of the join.

Standard constraints apply.

## As-landed

**The defender slot is a kind-polymorphic noun under an `Attackable` gate.**
`GameEvent.Attacks`'s `whom` is no longer `Maybe (Noun (nomIntro n) Player)` but
`AttackDefender (nomIntro n)`, a two-row slot type:

```idris
data AttackDefender : Bindings -> Type where
  NoDefender : AttackDefender bs
  OneDefender : {k : Kind} -> (m : Noun bs k) ->
                {auto 0 sg : nounPlur m = OneOf} ->
                {auto 0 at : Attackable m} -> AttackDefender bs
```

*Deviation from the scope's `Maybe (Noun (nomIntro n) kd)`, and why.* Under a
`Maybe`, the `Nothing` row leaves `kd` with nothing to solve it, so every
defenderless attack — `Macros.attacks`, and the bare `Attacks _ Nothing` in
`ProofsD` — becomes an unsolved-implicit error; the only escapes are a
`{default}` or a phantom `kd = Player` on a row that names no defender. Folding
the option into the slot type is how the core already writes an optional
kind-polymorphic gated noun (`DamageScope`'s `Everywhere` / `ToRecipient`), and
the unwritten row then carries no kind at all, which is what is true of it. The
old `AttackDefender` gate is absorbed rather than deleted: its singularity check
([CR#508.1b] announces one defender per attacking creature) rides on `OneDefender`.

**The gate.** Three definitions in `Experimental`'s mutual block, beside the noun
projections they read, so no out-of-block function enters a constructor type:

- `attackableTy : Maybe CardType -> Bool` — `Planeswalker`, `Battle`, nothing else.
- `attackableKind : Kind -> Maybe CardType -> Bool` — `Player` passes on any
  description, `Object` only at an attackable type, `a \/ b` only when **both**
  halves pass, every other kind fails.
- `Attackable {bs} {k} n = So (attackableKind k (nounTy n))` — the `So`-gated
  projection, used only in erased positions, mirroring `SingleRecipient`.

The both-halves rule is what makes the joined head work in the right direction:
"a player or planeswalker" projects `Just Planeswalker` through `seedTy`/`bindFor`,
so both halves pass, while "any target" projects `Nothing` at its object half and
is refused.

**`DefendingPlayer` moved with it:** `{k : Kind} -> (m : Noun bs k) ->
{auto 0 at : Attackable m} -> DeonticPatient {bs} Attack Agent`. Its two bench
sites (`blazingArchonCant`'s `DefendingPlayer You`, `goadedAttacksOther`'s
"a player other than you") are byte-unchanged: the gate admits `Player` on `Oh`.

**`Macros.attacksPlayer` widened in place** to `{k : Kind}`, so its three
`Cards.idr` sites are byte-unchanged and the same macro now spells a joined
defender. `Macros.attacks` is unchanged but for `Nothing` becoming `NoDefender`.

Diffstat: 5 files, +95/-21. `idris/scripts/build` 19/19 from clean;
`Cards.idr` binds 0 implicits; no `{default` anywhere touched.

### Ledger

**Pin minted: 1.** `badCreatureAttackDefender` (`ProofsE`) — "Whenever a creature
attacks another creature". [CR#506.3] names the closed set and a creature is not
in it, so the term is a category error, not a corpus absence. **Pins retired: 0.**
`badPluralAttackDefender` is restated through the new row and now fails on
`nounPlur … = OneOf` (`Refl impossible`) rather than on the retired gate type.

**Pin probe** (temporary `Probe.idr`, checked and removed). Two refusals, both
reported as `Can't find an implementation for So False` at the `OneDefender`
argument — the `Attackable` gate, not a neighbouring one:

- `OneDefender (Macros.a Macros.creature)` — attacking a creature. REFUSED.
- `OneDefender (Macros.target Macros.anyTarget)` — attacking "any target".
  REFUSED (its object half projects no single attackable type).

Three admissions, all elaborating clean: `Macros.a (HasType Planeswalker)`,
`Macros.a (HasType Battle)`, and the joined head
`Macros.a (Macros.kindJoin AnyPlayer (HasType Planeswalker))`.

**Tahngarth, First Mate — benched extent.** Oracle re-verified verbatim against
the ticket's quotation via the `card` script. Two of the four clauses are benched,
as `tahngarthChoosesDefender` / `tahngarthAttacksThatJoin` in `Cards.idr`:
the choose mints the joined binding and the attack declaration reads it back with
`Macros.thatJoin` in the defender slot. That pair is the ticket's positive
witness — the slot admits a phrase that names either half, and the card never
says which.

Blockers, each a named missing constructor:

1. *"Whenever an opponent attacks with one or more creatures"* — `GameEvent`
   has no attack-declaration row with a **player** subject; `Attacks`'s subject
   is `Noun bs Object`. With no such row the opponent is never bound, so clause 2
   ("that opponent") has no antecedent either.
2. *"you may have that opponent gain control of Tahngarth until end of combat"* —
   `May` + `GainsControl` + `Reflexively` ("If you do") all exist; this clause is
   blocked only by (1), for want of the "that opponent" binding.
3. *"a player or planeswalker **that opponent is attacking**"* — no `Predicate`
   describes a player or planeswalker by what is attacking it. `Attacking` is
   `Predicate bs Object` and describes the *attacker*. The restriction is dropped
   from the bench; the head is written.
4. *"Tahngarth **is attacking** that player or planeswalker"* — no `Effect` row
   makes an on-battlefield permanent an attacking creature. `EntersAttacking` is
   a `TokenRider` on entry (`MoveRiders` / `Create`) and carries no defender;
   `RemoveFromCombat` is only its inverse. The clause is therefore benched as the
   `GameEvent` it declares, in the context the choose leaves behind, rather than
   as an effect that asserts it.

Blockers 1 and 4 are each a new construction; neither is in this ticket's scope
and neither was minted.

**Residue found in review: the join check sees one head type, not two.**
`Attackable` reads `nounTy`, and `seedTy (Joined l r)` prefers the left half and
discards the right, so `attackableKind (a \/ b) t` re-checks the *same* `t` for
both halves. At `Object \/ Player` — every kind `Macros.kindJoin`,
`Macros.anyTarget` and `Macros.thatJoin` can build — that is exactly right,
because the `Player` arm is unconditional and the `Object` arm carries the whole
check. A hand-written `Joined` at `Object \/ Object` slips through: the slot
admits `Macros.a (Joined (HasType Planeswalker) (HasType Creature))` — "a
planeswalker or a creature" — while the reversed order and
`Joined AnyPlayer (HasType Creature)` are both refused. No macro produces that
kind and no bench site writes `Joined` by hand, and the weakness is `seedTy`'s,
not this gate's: the analogous `DamageRecipient` admits *every* join vacuously
via `JoinTakes`, so the new gate is strictly stronger than the one it mirrors.
Closing it needs a per-half head-type projection over nouns (`headTys` has no
`Joined` case either) — a construction outside this ticket.

**Not benched: a printed deontic with a joined defender.** The attested phrasings
are "attacks you or a planeswalker you control" and "can't attack you or …", whose
player half is the deictic *you*. `kindJoin` takes a `Predicate bs Player` and the
only two are `AnyPlayer` and `Opponent`; "you" is a `Noun`. Recorded here rather
than worked around — it is a missing player predicate, unrelated to the gate.
