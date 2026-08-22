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
  admission at the joined kind [CR#615.7] with **no widening at that site**;
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
