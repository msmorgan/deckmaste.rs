---
needs: []
---
**A cost may not name a random subject, and the validator that should say
so has a hole.** Six restored payment tests are `#[ignore]`d on the first
half; the other two items are the same seam seen from the engine side.
Routed from `core-regions-test-restoration`'s landing, 2026-09-02.
Standard constraints apply.

## Scope

1. **No cost spelling for a sampled subject.** `validate_announced`
   rejects a `Let` whose expression is a random object selection as a
   decision inside an expression, and the runnable-cost check refuses the
   composite form, so "sacrifice a creature at random" has no cost
   spelling at all. Core needs a sampling twin of `CostComponent::Choose`:
   an instruction with a destination whose subject is drawn rather than
   picked, so the payer surfaces no decision and the register still binds.
   Six ignored tests in `crates/deckmaste_engine/tests/payment.rs` name
   this blocker and un-ignore with it.
2. **The decision-in-expression check is partial**, a hole in
   [Core is explicit regions](../../decisions/core-explicit-regions.md)
   law 5. `region::validate_cost` inspects only the plural object
   expression, so a singular `Let` reading a random selection passes
   validation and then resolves to a null object rather than being
   refused at load. Close it over every expression shape. Tightening may
   reject corpus cards that load today; if it does, list them and route
   them rather than widening the hole.
3. **A dead tier arm.** The engine carries an unreachable deferred-payment
   arm for a `Let` obligation over a random selection which inverts the
   obligation tier order for that shape. It becomes reachable the moment
   item 1 lands, so decide it deliberately: either the sampled instruction
   is never deferred, or the arm is correct and gets a test.

## Gates

The six ignored payment tests un-ignore and pass. A singular `Let` over a
random selection is refused at load with a diagnostic, not resolved to
null. Standard suites green.

## Landing record

The core cost instruction inventory grew from 10 to 11 variants. The added
`Sample` instruction owns a quantity, an explicit candidate region, and the
destination objects register; lowering emits it for an existing random cost
subject. It consumes RNG without surfacing a player decision, records the
chosen group and post-sample RNG position, and replay retains that outcome.

The deferred-tier arm is deliberate: `Sample` and the action that consumes its
register both belong to the remaining-cost tier under [CR#601.2h]. The tier
gate now considers the first outstanding register writer before separating
ordinary and deferred obligations, so no consuming action can pass its sample.

Decision-in-expression validation went from checking one direct plural-object
shape to recursively checking all three `Expr` result shapes: object, objects,
and number. Random and chosen-order selections are rejected anywhere below a
`Let`; ordinary register reads remain validated by the enclosing region. The
workspace corpus rejected 0 cards after this tightening, so there are no
follow-up cards to route.

Test accounting:

- Restored: 6 payment tests changed from ignored to active; all 6 pass.
- Re-spelled: those 6 tests now use the shared `Sample` cost fixtures instead
  of `Let(Random(..))` fixtures.
- Strengthened: the existing core rejection test covers all 3 expression
  shapes, and the existing lowering test authenticates the candidate and
  controller provenance of the sample region.
- Ignored with blockers: 0 ticket-specific tests. The payment suite's 2
  unrelated whole-body-preflight ignores are unchanged.
- Added or removed test functions: 0. Coverage and citation locks are
  unchanged.

Positive gates: `cargo fmt --all -- --check`; `cargo test --workspace` (all
non-ignored tests and doctests passed); payment integration suite 30 passed,
0 failed, 2 unrelated ignored; focused core and lowering tests each passed;
`cargo clippy --workspace --all-targets -- -D warnings`; citation scan 0
non-compliant strings, 17,882 citations checked with 0 stale, and every added
diff citation audited against its rule text.

The explicit candidate region, retained random replay record, and dependent
action tier flag are implementation additions required to preserve the ADR's
region boundary and payment replay semantics. There were no scope deviations,
lock changes, routed corpus regressions, or STOPs.
