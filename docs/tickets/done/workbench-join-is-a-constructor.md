---
needs: [workbench-pins-refuse-rules-impossibility-only]
---
# The kind join is a constructor

Supersedes the shape landed by `workbench-join-shape-flat-or-pair` (ruling
2026-08-22). `Kind` gains the constructor `(\/) : Kind -> Kind -> Kind`
itself. `ObjectOrPlayer`, `joinable`, the `So (joinable a b)` gate, the
`joinableRefl/Sym` lemmas and the join pins (`badJoin*`) are deleted: a join
of any two kinds is writable, because the rules make "a spell or ability",
"a creature or player", "a permanent or player" all meaningful and the
semantics accepts anything rules-meaningful
(`docs/memory/rulings/measurements-live-in-pins.md`).

## The change

- `data Kind = … | (\/) Kind Kind`, `infixl 5 \/` kept.
- `Payload` gains `JoinP : Payload a -> Payload b -> Payload (a \/ b)`. A
  union mention carries what it knows about each half — "target creature or
  player" is `JoinP (ObjectP (Just Creature) …) PlayerP` — which is the pair
  the demonstrative echo reads back (33 of 33). `UnionP` is deleted and its
  readers (`bindingZone`/`bindingTy`, `bindFor`'s union branch) read `JoinP`.
- The join is a free term, so the laws move from the function to a **lattice
  order**, pinned (2026-08-22) as

  ```idris
  kindLte : Kind -> Kind -> Bool
  kindLte (a \/ b) y = kindLte a y && kindLte b y
  kindLte x (a \/ b) = kindLte x a || kindLte x b
  kindLte x y = x == y
  ```

  (left-join clause first; `&&`/`||` laziness is harmless at concrete kinds,
  which is all a `So` gate reduces). An inductive `KindLte` with
  `Same/JoinL/InL/InR` is the documented upgrade if a reader ever needs the
  witness (which half an anaphor resolved to); not minted now. Laws:
  `a ≤ a \/ b`, `a \/ b ≤ b \/ a`, `(a \/ b) \/ c ≤ a \/ (b \/ c)` and their
  converses, proved by case. Every reader that asks "is this binding of kind
  k" — the antecedent counting under `It`/`They`/`That` (`countOnes` and
  kin), `lookbackSubjectOk`, `Targetable` — asks `kindLte` instead, which also
  answers whether an `Object` anaphor may resolve to an `Object \/ Player`
  antecedent (it may). `Eq Kind` stays structural.
- Every exhaustive `Kind` match gains one `\/` clause, generically (recurse
  or combine), instead of one `ObjectOrPlayer` clause per attested pair.
- The `(\/)` docstring stays the decision record, rewritten: flat-vs-pair was a
  level confusion; the pair is `JoinP`; the join is syntax.
- `AnyTarget`, `KindJoin`, `YouAnd` and the `That` anaphor stay as they are;
  `workbench-union-family-macros` retires them over this shape.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr`, `idris/src/Experimental/Proofs*.idr`,
`idris/src/Experimental/Cards.idr`. No Rust crate is touched.

## Acceptance

- No `ObjectOrPlayer`, `joinable`, `UnionP` or `badJoin*` remains; `kindLte`
  and its laws exist; `JoinP` carries the pair and a witness reads it back.
- `idris/scripts/build` PASS; `Cards.idr` binds no implicits; cites 0/0.

Standard constraints apply.

## As landed (2026-08-22)

`Kind` is now a GADT so the join can carry the decision record as a
constructor docstring: `(\/) : Kind -> Kind -> Kind`, `infixl 5` kept.
Deleted: `ObjectOrPlayer`, `joinable`, the `So (joinable a b)` gate, the
`(\/)` FUNCTION, `joinableRefl`/`joinableSym`, `joinIdem`/`joinComm`/
`joinAssoc`, the three `badJoin*` pins and the four `Refl` join witnesses in
ProofsG. `Eq Kind` gained `(==) (a \/ b) (c \/ d) = a == c && b == d` (plus
the eight mismatch rows each way); `sameKindRefl` is now a lemma over `==`
with `andSo` on the join case.

### Laws proven

`kindLte` landed verbatim as pinned. All proofs are total, hole-free and
hatch-free; nothing was stopped.

- `kindLteInL` / `kindLteInR` — widening into either half, by induction on
  the left kind. These are the workhorses: everything else is a corollary.
- `kindLteRefl` — induction, join case via the two above.
- `kindLteJoinL : So (kindLte a (a \/ b))`, `kindLteJoinR`.
- `kindLteComm : So (kindLte (a \/ b) (b \/ a))`.
- `kindLteAssocR` / `kindLteAssocL` — both directions.
- `kindLteTrans` — **goes through**, no obstacle: induction on the left kind
  (a join splits into both halves), then on the middle kind (a join is
  entered through whichever half the left kind fit), and two unjoined kinds
  fit only by `==`, which is identity. Costs 73 clauses because `kindLte`
  only reduces at a known constructor, so the atomic-left rows are written
  out; `sameKindEq` was therefore not needed and was not minted.

`kindLte`'s docstring carries the documented upgrade — the inductive
`KindLte` with `Same`/`JoinL`/`InL`/`InR`, whose `InL`/`InR` carry which
half an anaphor resolved to. Not minted: no reader asks.

### Readers switched to `kindLte`

Direction throughout: `kindLte <asked> <antecedent>`, so an `Object` anaphor
lands on an `Object \/ Player` antecedent and not the reverse.

- `countOnes`, `countManys`, `anyTargeted`, `countQuality`, `countLetter`.
- New `itReaches : Plurality -> Binding -> Bool` (`kindLte Object b.kind`
  plus plurality), now the single test behind `provOfIt`/`provOfThem`,
  `zoneOfIt`/`zoneOfThem`, `tyOfIt`/`tyOfThem` — six functions that used to
  pattern-match `MkBinding _ Object _ ...` with a union row each.
- New `objGroup : Binding -> Bool`, behind `countGroups`, `countParts`,
  `groupSpent`, `zoneOfGroup`, `tyOfGroup`: a joined group ("two targets")
  is a group of objects too.
- `lookbackSubjectOk ev (a \/ b)` = both halves (the deferred
  `ObjectOrPlayer = False` row is gone): "any target" cannot be what died,
  since a player cannot die, but it can be what took damage.

### Payload readers

`JoinP : Payload a -> Payload b -> Payload (a \/ b)` replaced `UnionP`
(which had been `Payload Object`, so union bindings now carry kind
`Object \/ Player` rather than `Object` — that is what made the `kindLte`
switch load-bearing). New `payloadZone`/`payloadTy`/`payloadProv` read a
join as "the Object half's, else none"; `bindingZone`/`bindingTy` are now
one-liners over them. `pubB`, all eight `wordReaches` words, `verbedMatch`,
`verbedMatchMany` and `setZone` read `JoinP` where they read `UnionP`.
`bindFor`'s union branch mints
`JoinP (ObjectP (headKindJoinTy p) Nothing Nothing Nothing) PlayerP` at
kind `Object \/ Player`, with new `kindJoinTy`/`anyKindJoinTy`/
`headKindJoinTy` and `joinedClassTy` supplying the Object half's card type
(`JoinCreature -> Just Creature` …; the permanent class and the class word
name none, "permanent" not being a card type [CR#205.2a]). So a
`KindJoin who what` binding's `bindingTy` is now the joined class's type
where it used to be `Nothing` — the type the demonstrative echo derives
from.

### Witnesses

- The joined-antecedent readback is already benched in `Cards.idr` and now
  elaborates through `JoinP`: `blastOfGenius`, `riddleOfLightning` and
  `galvanicBlastLine` all announce a union head and read it back with
  `That JoinW`, whose gate is exactly the `JoinP` row of `wordReaches`. No
  new bench minted.
- `Object \/ Ability`: no head spells it (`KindJoin` joins a player), so the
  witness is the payload — `spellOrAbilityJoin : Payload (Object \/ Ability)`
  in ProofsG, for Bolt Bend's "Change the target of target spell or ability
  with a single target", with `abilityUnderSpellOrAbility` reading the
  `Ability` half back through `kindLteJoinR`.
- No pin on the join itself; no reader case turned out rules-meaningless.

### Deliberately not done

- `kindOfW JoinW` stays `Object`. The union family's PHRASE kind is what
  `Targetable`, `DealDamage`'s recipient slot and `Noun bs (kindOfW w)` are
  asked at; moving it to `Object \/ Player` is `workbench-union-family-macros`'
  work, together with retiring `AnyTarget`/`KindJoin`/`YouAnd`/`That`.
- `Targetable` unchanged for the same reason: it is asked at a phrase's kind
  (`AnyTarget : Predicate bs Object`), never at a binding's, so there is
  nothing to switch and a `Targetable (a \/ b)` would be unused.
- `lookbackComplementOk` keeps its `_ _` catch-alls; no join reaches it while
  union phrases are still kind `Object`.

### Ledger (side effects of the kind change, for whoever comes next)

- `countOnes Player` / `countManys Player` -- and therefore `countChoosers`
  -- now count union bindings, since `kindLte Player (Object \/ Player)` is
  `True`. A "target creature or player" mention is thus a candidate chooser
  where it used to be invisible to the count (it sat at kind `Object`).
  Right under the order's reading; flagged to
  `workbench-union-family-macros`, which owns the union family's phrase kind
  and can decide whether a chooser slot wants `Player` exactly.
- `setZoneIt`/`setZoneThem` (`Experimental.idr` ~4043/4050) still pattern-match
  `MkBinding det Object _ (ObjectP ...)`, so a join binding falls through to
  their skip clause, while `zoneOfIt`/`zoneOfThem` now STOP at that binding
  and report its (absent) zone. The asymmetry is pre-existing -- `UnionP` was
  skipped by the same `ObjectP` pattern while the old union row in
  `zoneOfIt` stopped at it -- and is left as found; closing it means deciding
  what moving "that creature or player" does to the Object half, which is a
  payload question, not a kind one.

### Round follow-up (reviewer finding, closed)

`joinedClassTy` -> `bindingTy` was unwitnessed: `Nothing` in `bindFor`'s union
branch built green and `JoinCreature` was constructed nowhere. Closed with
- `firesongJoinEcho` in `Cards.idr`: Firesong and Sunspeaker's
  `KindJoin JoinAnyPlayer JoinCreature` head with a `That JoinW` echo. Its
  head is the sole current-oracle black-border "target creature or player"
  (verified in MTGJSON across DOM/2X2/MUL/PZ2; the other three corpus lines
  are funny-set). No printed card spells the echo -- nothing in the corpus
  pairs "target creature or player" with "that creature or player" -- so the
  second clause is the workbench's, as `galvanicBlastLine` and the ProofsG
  union pins already are. Binds no implicits.
- `joinedCreatureTy` in ProofsG: `tyOfThat JoinW (effIntro ...) = Just
  Creature`, by `Refl`. `That JoinW`'s own gate reads only the payload's
  shape, not its type, so this is the smallest read that observes the type.
  Mutation-checked: with `Nothing` back in `bindFor`, this is the clause that
  fails ("Mismatch between: Just Creature and Nothing"), and it is the only
  one that does.

Gates: clean `idris/scripts/build` 18/18, exit 0. `ObjectOrPlayer|joinable|
UnionP|badJoin` absent from `idris/src/Experimental*`. `Cards.idr` binds 0
implicits. Cites 0 non-compliant / 0 stale, 7 audited sites (3 in
source, 4 in this section) read against their rule text ([CR#112.1],
[CR#115.4], [CR#205.2a]); `bless` registered no new rules. Pin probe: `badDestroyUnionAnaphor`'s term still refuses, now via
the `JoinP` zone read.
