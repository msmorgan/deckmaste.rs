---
needs: [workbench-join-is-a-constructor]
---
# Migrate the union family to functions over the joined-kind core

`AnyTarget`, the cross-kind union head `KindJoin`, the mixed group `YouAnd` and
the `That` / `UnionP` anaphor that reads one back are **not core constructors**
under the settled direction. Their authoring role passes to the card language:
plain (sometimes dependently typed) Idris functions over the joined-kind core,
called macros because the macro doctrine is where they are headed. "Any target"
[CR#115.4] becomes a function expanding to a joined-kind term, not a row.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— the third pinned execution question, the per-site migration. This is the
migration only; the direction is not re-openable, and the prerequisites carry the
parts that are not a straight move.

## Scope

- Retire the four marked constructions from the core and re-offer each as a
  function at the joined kind, keeping the authoring surface — a card author
  writes the same phrase name and gets a joined-kind term.
- Take the derivable wins as theorems rather than transcribing them:
  `DamageRecipient`'s three union rows (`AnyTargetTakes`, `JoinTakes`,
  `GroupTakes`, which today admit without the zone-and-damageable-type check
  `ObjectTakes` makes) collapse to one
  admission at the joined kind [CR#120.1] — widened at the object half, since "a permanent or player" is attested and wider than the damageable set, the choice settling at resolution [CR#120.1a] (ruled 2026-08-22 on review);
  `nounSpansPlayers` becomes a kind test; `headIsPlaceless` loses two of its
  three disjuncts and keeps the source-role disjunct, which is placeless for an
  independent reason [CR#120.7] and is not a union site at all; `bindFor`'s
  union branch stops choosing a payload.
- The seven-verb battlefield refusal must keep holding **without a rule written
  for the purpose** — the zone-and-type projection of the joined kind is what
  refuses destroy/exile/tap/untap/return/counter/sacrifice while the damage
  clause still admits (`JoinTakes` against `badDestroyKindJoin`: the same fact
  refuses seven verbs and permits the damage clause, because damage checks no
  zone). That pair is the strongest single argument for the join; if it needs a
  hand-written rule after the migration, the migration is wrong.
- Overgeneration in the semantics is tolerated **by design** here: a semantics
  value with no production has no English and is refused at the boundary. Do not
  add gates to narrow the joined kind beyond what the prerequisites deliver.
- Per-site residue this closes: **Tahngarth, First Mate** — "Tahngarth is
  attacking that player or planeswalker" wants the attack-defender slot to admit
  a union, and the slot is player-kinded today. Under a joined kind the slot is
  an ordinary joined-kind noun. Land it or record why not.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Macros.idr` (the function layer these move into) and the
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate is touched —
the spelling half already left under
`workbench-union-gate-spelling-rehome`.

## Acceptance

- Every card benched through the four constructions is still benched, by the
  same phrase, through a function.
- No measurement re-homed by `workbench-union-gate-spelling-rehome` is carried
  here as well: it is already at the spelling boundary, and a second copy is a
  defect.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Two code sub-rounds, uncommitted in the feature workspace.

**Sub-round 1 — the union family becomes a function over the join.**
`Predicate.AnyTarget` and `Predicate.KindJoin` are replaced by one
constructor, `Joined : Predicate bs ka -> Predicate bs kb -> Predicate bs
(ka \/ kb)`; `Noun.YouAnd` by `Both : Noun bs ka -> Noun (nomIntro l) kb ->
Noun bs (ka \/ kb)`. `kindOfW JoinW` is now `Object \/ Player`, so the
demonstrative reads a joined mention back at its antecedent's own kind, and
`JoinW` survives as a noun word. `Targetable` and `Phrasal` gain join
constructors (`JoinTgt`, `PhJoin`). The four authoring surfaces re-open in
`Macros` as `anyTarget`, `kindJoin`, `youAnd`, `thatJoin`; every card that
was benched through a retired constructor is benched through the macro of the
same phrase name, and `Cards.idr` still binds 0 implicits.

The facts that used to hang off the marks now fall out of the kind:

- `headIsPlaceless` keeps only its source-role disjunct ([CR#120.7]); the
  union family's placelessness moves into the new `phraseZone`, whose second
  test is `not (kindLte k Object)` — [CR#400.1] makes a zone a place where
  objects can be and [CR#109.1] lists what an object is.
- `DamageRecipient`'s three union rows collapse to one `JoinTakes` at the
  joined kind: it admits every noun at a joined kind. That is not quite the
  union of what the three admitted — see the widening noted under
  Overgeneration — but it removes no check the three made, and the
  object-only `ObjectTakes` keeps its zone and `DamageableTy` gates.
- `bindFor`'s union branch stops *choosing* a payload — the kind fixes the
  shape (`joinHalfPayload`) and the description fixes the object half's type.
- `setZoneIt` / `setZoneThem` now use `itReaches`, the same test `zoneOfIt`
  reads with; `setZone`'s join row is already the identity, so no policy is
  needed for the joined half.
- `OtherThan` takes its own kind index, since a complement subtracts a
  referent rather than naming a member of the phrase's domain.

Deleted outright, having lost every row: `nounSpansPlayers` /
`NotPlayerSpanning`, `nounIsMixedGroup` / `NotMixedGroup`, `nounIsKindJoin`,
`nounIsAnyTarget`, `anyTargetLone` / `AnyTargetLone`, `anyTargetOkAt` /
`AnyTargetAtCount`, `allLoneOk`, `anyTargetZoneOk`, `countAnyTargets`,
`isAnyTarget`, `isKindJoin`, `anyIsKindJoin`, `headIsKindJoin`, `kindJoinTy`,
`anyKindJoinTy`, `headKindJoinTy`, `anyIsAnyTarget`, `headIsAnyTarget`, and
in `Words` the `JoinedPlayer` / `JoinedClass` enums with `joinedClassTy` —
whose content was duplication of predicate rows the core already had.

Diffstat: 10 files, +288/-544. Build green at this point.

**Sub-round 2 — the vacuous `AnyTargetFree` family.** With no `False` row
left, `anyTargetFree` was constantly true and every obligation reduced to
`So True`. Deleted: `anyTargetFree`, `anyTargetFreeAll`, `AnyTargetFree`,
`nounAnyTargetFree`, `placeAnyTargetFree`, `zoneAnyTargetFree`,
`amtAnyTargetFree`, `complementAnyTargetFree`, `nameSrcAnyTargetFree`,
`spendSourceOk`, `SpendSourceOk`; and the slots that fed them — thirteen
constructor obligations (`Superlative`, `Each`, `Indefinite`, `Definite`,
`CountedGroup`, `AllOf`, `CountOf`, `Aggregate`, `Exists`, `Matches`,
`Search`, `CantUntapMoreThan`, `ToCast`), `ToActivate`'s `sf`, two
`TokenPhrase` witnesses, and ten `Macros` wrappers.

Diffstat (increment, including the post-audit cite retarget below):
2 files, +34/-179. Cumulative: 10 code files, +329/-750 (plus this
ticket and the follow-on ticket).

### Ledger

**Pins retired: 22, none minted.** Eighteen because the kind index now
refuses their term outright — the seven verbs and their kin each name a
`Noun bs Object`, so a joined phrase is a type error before any gate is
asked, and an `Unspellable` pin needs a gate to pin:
`badDestroyAnyTarget`, `badTapAnyTarget`, `badExileAnyTarget`,
`badCounterAnyTarget`, `badDiscardAnyTarget`, `badAnyTargetEmbedded`,
`badAnyTargetInOr`, `badAnyTargetInOrLaundered`, `badCantAnyTarget`,
`badMatchesAnyTarget`, `badEmbeddedAnyTargetExact1`, `badCastsAnyTarget`,
`badAnyTargetInGraveyard`, `badTapKindJoin`, `badExileKindJoin`,
`badKindJoinInOr`, `badTapYouAnd`, `badDestroyUnionAnaphor`. Each refusal
got *earlier*, not weaker.

Four because the term is now writable and the rules leave it meaningful —
a count or a redundancy is never a refusal
(`docs/memory/rulings/measurements-live-in-pins.md`):
`badDoubleAnyTarget` (one description written twice is redundant),
`badNestedYouAnd` ("you and you and …" names one player twice, [CR#109.5]),
`badAnyTargetUnderA` ("a permanent or player" is attested — Furnace of Rath —
so the refusal was a spelling fact), `badExistsAnyTarget` (an existential
over the target class describes a real set).

Kept and re-stated through the macros: `badDestroyAnyTargetRemention`,
`badRedirectToGroup`, `badScaleShiftByThatMuch`, `joinedCreatureTy`.
Untouched: `badUnionAnaphorNoAntecedent`, `badUnionAnaphorOnObject`.

**The seven-verb refusal is argued by no purpose-written rule.** The refusal
itself is the kind index at each verb's `Noun bs Object` slot — a type error,
unpinnable and stronger than a gate. The one gate that still *pins* it is
`Macros.destroy`'s own pre-existing `{auto 0 ok : OnBattlefield (nounZone n)}`,
which `badDestroyAnyTargetRemention` fails on `OnField`: the binding a joined
target leaves records no zone, so "destroy it" after "deals damage to any
target" is refused by the destroy gate that was already there. Nothing was
written for the purpose. Two new witnesses record the same fact from both
sides — `anyTargetIsPlaceless` (the joined target's `nounZone` is `Nothing`)
and `anyTargetTakesDamage` (the same term IS a `DamageRecipient`). A third,
`youAndBindsNothing`, records that the mixed group mints no joint binding —
`Refl`, not a count.

**`lookbackComplementOk` refuses every joined complement, and stays refusing.**
Under `kindOfW JoinW = Object \/ Player` a joined complement is now writable,
so this is a live refusal rather than an unreachable one. `corpus --match
'dealt damage by'` returns 34 distinct lines, five of them carrying "or"; none
writes a cross-kind union complement. The nearest miss — "dealt damage by a
red instant or sorcery spell you control or by a red planeswalker you
control" — coordinates two *object* complements. No attested union, so the
clause is not widened; recorded here rather than gated.

**Overgeneration tolerated by design, named.** `Both You (AllOf land)` is a
damage recipient (`JoinTakes` asks nothing of the object half, as `GroupTakes`
asked nothing before it — not new); `Or [Joined …, Joined …]` is writable;
`AttachHost Enchanted JoinW` ("enchanted permanent or player") is writable,
consistent with `attachHeadOk`'s own [CR#303.4] docstring; and "exile it"
after a union target is admitted, since `It` is object-kinded and the read
presupposes the target was an object. None of these has an English
production, so each is refused at the boundary.

Two more, surfaced in review and NEW with this round rather than carried:

- **A same-kind join bypasses `DamageableTy`.** `Joined` is general over two
  kinds, so `Joined (HasType Land) (HasType Land)` is a `Predicate bs
  (Object \/ Object)`, and `JoinTakes` admits it as a damage recipient with
  no zone and no type check — where the object-only phrase `And [creature,
  InZone graveyardZ]` is still refused by `ObjectTakes`. No such term was
  writable before: `KindJoin`'s left arm was a `JoinedPlayer` enum and
  `YouAnd`'s was the deictic `You`. This is the widening at the
  `DamageRecipient` site, and it is the price of the general constructor,
  not a decision taken here.
- **The `AnyTargetFree` sweep opens the kind-polymorphic slots.** `CountOf`,
  `Aggregate`, `Exists`, `Each`, `Indefinite`, `Definite`, `CountedGroup` and
  `AllOf` are kind-polymorphic, so each now admits a joined predicate:
  `CountOf Macros.anyTarget` ("the number of any target") type-checks. The
  Object-kinded slots the family also guarded — `Matches`, `Search`,
  `CantUntapMoreThan`, `ToCast`, `ToActivate` and both `TokenPhrase`
  witnesses — are refused by the kind index instead, so nothing is lost
  there.

**Deviations from the design.**

1. `JoinTakes` cites [CR#120.1], not the design's [CR#120.1a] nor the
   ticket's [CR#615.7]. [CR#120.1a] only forbids damage to an *object* that
   is not a battle, creature or planeswalker; [CR#120.1] is the rule that
   states the whole recipient set — "Objects can deal damage to battles,
   creatures, planeswalkers, and players" — which is exactly what the
   collapsed admission claims. [CR#615.7] is a prevention-shield rule and says nothing
   about who may be dealt damage; it stays where it is, on ProofsF's shield
   pin.
2. `Macros.kindJoin` and `Joined`'s "target creature or player" example cite
   [CR#115.1], not [CR#115.4]. [CR#115.4] is explicitly about the class word
   written *rather than* "target [something]", so it argues against an
   explicit two-noun head rather than for it. [CR#115.1] — "The targets are
   object(s) and/or player(s)" — is the rule that admits the cross-kind head.
   [CR#115.4] is kept wherever the class word itself is the subject
   (`Macros.anyTarget`, and `Joined`'s "any target" sentence).
3. Two `ProofsC` sites needed `MillB {whose = …}` written out. Dropping
   `{na = MkNotPlayerSpanning}` removed the elaboration anchor that had been
   solving `MillB`'s `whose` implicit, and `%unbound_implicits off` then
   refused to bind it. Proof files only; `Cards.idr` still binds 0 implicits.
4. The design's bench prose says "twenty pins retire"; its own §E1/§E2 tables
   list 22. The tables were followed.
5. `Cards.idr` carried 37 `AnyTarget` occurrences, not the 36 lines the
   design enumerated; all were migrated.
6. No chapter or finding numbers, and no chapter prose: this section and the
   docstrings are the decision record (conductor ruling).
7. Not touched, out of the consumption boundary: `idris/src/Bridge.idr`'s
   illustrative comments still name `AnyTarget` in v1/v2 comparison
   pseudo-code that was already stale before this round.

**Gates.** `idris/scripts/build` 19/19 from a clean `build/`; `Cards.idr`
binds 0 implicits; `grep 'AnyTarget\b|KindJoin\b|YouAnd\b'` over
`Experimental.idr` is empty; `cargo xtask cite check --list-noncompliant`
reports 0; `cite check` 0 stale; `cite bless` registered no new rules
(lockfile unchanged); `cite audit --diff` read over all 22 sites, which
produced deviations 1 and 2.

**A pronoun may read a union binding, and no gate was added.** `countOnes k
bs` counts a binding whose kind `k'` satisfies `kindLte k k'`, and
`kindLte Object (Object \/ Player)` and `kindLte Player (Object \/ Player)`
are both true, so `It` and `They` each resolve against a joined binding, as
they did before this round (`bindFor`'s union branch already minted a
joined-kind binding). [CR#115.1] makes both readings real, so the read is
meaningful when the target was of that half's kind and presupposition-fails
otherwise. Nothing is narrowed; the ruling that a count is never a refusal
decides it. `setZoneIt` / `setZoneThem` now reach the same joined binding
`zoneOfIt` stops at, and leave it unchanged.

**Follow-on minted.** `docs/tickets/planned/workbench-attackable-defender-join.md`
carries the Tahngarth, First Mate residue: `Attacks`'s `whom` and
`DefendingPlayer` as joined-kind nouns plus the [CR#506.3] `Attackable` gate
the join does not supply.
