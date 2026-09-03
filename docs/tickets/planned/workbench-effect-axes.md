---
needs: []
---
**Move the counter family onto the `CounterKindSource` axis, unify the entry
riders, and keep only the `From` forms of prevention and redirection.**
2026-09-02 workbench audit (F7, F8, F9).

## Counter effects over `CounterKindSource` (F7, M)

`Effect.Effect:1238-1292`: `PutCounters amt (kind : CounterKindSource) on`
already carries the "which kind" axis (`Words.CounterKindSource:3618`:
`PrintedKind | ChosenKind | DistinctChosenKinds | BoundKind`), but
`PutCountersOfThoseKinds:1268` (7 uses), `PutSameCounters:1265` (2),
`GiveCountersOfOwnKinds:1280` (0), `DoubleCountersOfOwnKinds:1283` (2),
`GiveAbilityCountersOfOwnKinds:1286` (0), `RemoveCountersOfOwnKinds:1287` (0),
`GetsCounters:1273` (player patient, 4) and `GetsCountersOfThoseKinds:1276` (1)
spell the axis as constructor suffixes. `GetsCounters`/`PutCounters` differ
only by patient kind, while `GiveCountersOfOwnKinds` is already
kind-polymorphic (`kindLte k (Object \/ Player)`).

Fix: `CounterKindSource += ThoseKinds {countOutcomes CountersPut bs = 1} |
OwnKinds | SameAs (src : Noun)`; `PutCounters {k} amt kind (on : Noun k)` with
the existing `CounterSourceScope kind k` gate; `RemoveCounters` likewise;
`Double…` becomes `PutCounters (CountersOn It) OwnKinds` or one row. 12 → 5.
v1's basis (three actions + `CounterSpec`) is the smaller shape the mirror
ruling prefers.

## One entry-rider vocabulary (F8, M)

`Triggers.TokenRider:26-31` (`EntersAs v | EntersAttacking | EntersTransformed
| EntersMelded`) versus the statics `EntersWithCounters:441` (5),
`EntersAsCopy:445` (5), `EntersUnderInstead:395` (1), and `Effect.MoveRiders:880`
which carries entry riders, controller override and counters as *three*
fields. v1's `EnterRider` (core `action.rs:43`) holds Tapped/FaceDown/
UnderControlOf/UnderOwnersControl/Attacking/WithCounters/AsCopy in one enum.

Fix: `TokenRider += WithCounters amt (src : CounterKindSource) mark | Under who
| AsCopyOf src exc`; `MoveRiders` becomes `List (TokenRider bs)`; the three
statics collapse into `EntersRider n rider:438`. `EntersChoice:450` stays — it
binds outward (`staticChoiceDelta:841`).

## Prevention/redirection: keep the `From` forms only (F9, S/M)

`Prevents kind (size : Shield) scope by also:366` (11 uses) is
`PreventsFrom kind (src : DamageAgent) scope (cut : PreventCut) use also:371`
(14) with `Shield.AllOfIt/TheNext:616` ⊂ `PreventCut.CutAll/CutSome:667`, `by :
Maybe Noun` = `DamageAgent.Unattributed/DealtBy:637`, and `use = Repeatedly`.
Same for `Redirects:378` (8) ⊂ `RedirectsFrom:384` (3).

Fix: delete `Prevents`, `Redirects`, `Shield`, `byIntro`; rename the `From`
rows. The macros `preventAll`/`preventNext`/`preventAllBy`/`preventNextBy`
become four spellings of one row.

Done when: build is 23/23; the seven counter-suffix constructors, the three
entry statics, and `Prevents`/`Redirects`/`Shield`/`byIntro` are gone from the
tree; `MoveRiders` is a `List (TokenRider bs)`; every bench witness of a
deleted row is re-spelled against the surviving row and still typechecks; the
counter-kind and prevention pins in `Proofs*` are re-spelled and remain
non-vacuous. Standard constraints apply.
