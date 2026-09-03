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
