---
needs: []
---
**A carried body reached by a GRANT declares captures the engine does not
supply.** Residue from `core-regions-captures-and-memory`
([Core is explicit regions](../../decisions/core-explicit-regions.md) law 7).
Standard constraints apply.

## The gap

`deckmaste_lowering::region::in_carried_region` gives every carried ability
region the uniform capture ABI: its own intrinsic prefix, then one
`Provenance::Capture` parameter per visible enclosing register. For the bodies
that cross a boundary at RUN time as created objects — `Delayed` and
`Reflexive` — the engine snapshots those captures at creation
([CR#603.7a]) and supplies them at region entry, so a declared capture can
never be unavailable at firing.

A carried body reached by a GRANT instead — `Modify(_, GainAbility(..))`, and
the keyword expansions that ride it as `KeywordAbility::Composite` — is entered
LATER, by the granted-to object, from a bare frame with no creating register
file. Its declared captures therefore resolve to `Value::Unavailable`.

## Why this is unbuilt rather than broken today

No card in the corpus reads such a capture: the fourteen nested subterms
measured on canon differ from their isolated lowering in `params` only, never
in a body read (see
`every_semantic_ability_subterm_appears_at_its_own_depth_in_its_lowered_card`).
The declared-but-unread capture is inert. The moment a card DOES read one, it
would fizzle silently — the exact failure mode law 7 exists to remove.

## Scope

Either supply a grant's captures (snapshot them where the granting continuous
effect is registered, and carry them with the granted ability so its later
region entry can read them), or refuse the shape: make a carried region whose
body READS a capture across a grant boundary a load-time or lowering error.
Decide which on the rules — [CR#611.2,613] for what a granting continuous
effect may carry — and record it in the ADR under law 7, replacing the
residue paragraph there.

## Gates

Standard constraints apply. A fixture card that grants an ability whose body
reads an enclosing register, exercised end to end: it either reads the value
the grant captured, or fails to load with a diagnostic naming the card.

## Landing record

### Decision

Supply the captures. A granted ability is an ability-adding characteristic
modification in layer 6 [CR#613.1f], carried by the continuous effect created
when the granting spell or ability resolves [CR#611.2]. Rejecting the shape
would reject valid Oracle designs whose granted ability later refers to an
object fixed by that resolution. ADR law 7 now records the grant as a closure:
the floating effect snapshots each executable carried region, layer derivation
keeps that runtime payload aligned with the granted ability, and later
activated/triggered entry supplies the frozen capture list. Composite keyword
members are flattened with their own region-specific capture lists.

The end-to-end fixture grants an activated ability that captures the granting
controller, changes the granted-to permanent's controller, then activates the
ability. The body changes the original controller's life total and leaves the
new controller untouched, proving the read came from the grant-time snapshot
rather than the later bare activation frame.

### Numbers and assurance

- Engine library tests: 741 → 742 (+1 fixture); the complete
  `deckmaste_engine` package suite passes.
- Tests restored: 0; re-spelled: 0; ignored with blockers: 0 added; added: 1;
  removed: 0.
- Construction declarations and corpus coverage are unchanged. Provisioning
  regenerated the same 7,323 graduated wizard cards with 23,365 still in
  progress.
- Coverage locks are unchanged. `cr-citations.lock` is unchanged.
- Positive gates: the named fixture test, full `cargo test -p
  deckmaste_engine`, workspace formatting, engine clippy, and citation checks.

### Deviations and additions

- Added one engine-only runtime companion table keyed by continuous-effect
  timestamp. This keeps runtime `Value`s out of the public/core
  `ContinuousEffect` grammar while preserving transactional cloning and
  pruning entries with their owning effects.
- No constructions, production fixtures, or tests beyond the ticket's stated
  grant-capture fixture were added or removed.

No STOP was taken.
