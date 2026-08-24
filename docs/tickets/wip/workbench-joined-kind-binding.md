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
