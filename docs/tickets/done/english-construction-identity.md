---
needs: []
---
**Stable construction identity, declared precedence, and the family
registry.**

Groundwork step of `docs/decisions/english-grammar-is-derived.md`: make
construction selection independent of registration accident before generated
and handwritten families mix.

1. Census the current construction families and every selection site that
   observes `RuleId`, rule insertion order, or alternative index. Preserve a
   direct-AST and `inspect` baseline for each semantically ambiguous fixture
   before changing selection machinery.
2. Introduce a stable `ConstructionId` independent of registry and rule order.
   Give every participating handwritten family an identity before any
   generated implementation can join the registry.
3. Land the generated/handwritten family registry as the migration ratchet's
   single switch point. A family row records its stable identity, active
   implementation owner, backend, and declared dominance edges; switching a
   family is atomic, so no parse or render half can switch independently.
4. Replace semantic dependence on insertion order with explicit dominance.
   Existing parse costs remain only as named, inspectable tie-breakers between
   structurally equivalent derivations. Preserve legitimate packed ties rather
   than inventing precedence to make the output singular.
5. Extend `inspect` to report the selected `ConstructionId`, active owner,
   decisive guard/feature/role, cost, and every equally viable alternative.

Gates: run the same semantic fixtures over the normal, reversed, and a fixed
shuffled family-registration order; selections and packed alternatives remain
identical unless declared dominance says otherwise. The direct-AST/`inspect`
baseline, `cargo xtask english roundtrip --require-clean`, and recovery census
remain unchanged. No generated production traffic is enabled in this ticket.

Standard constraints apply.

Result: every handwritten grammar family now has a stable `ConstructionId`,
owner/backend/evidence metadata, stable per-family production ordinals, and a
validated registry of explicit dominance edges. Forest selection compares
named cost dimensions, declared dominance, and full stable derivation identity;
public provenance and verbose `inspect` expose the selected construction and
all equal-cost candidates, including dominated alternatives. Seven semantic
fixtures are identical under normal, reversed, and fixed-shuffle family
registration. The 31,685-face recovery JSON was byte-identical before and
after (3,340 recovery sites), round-trip reported 31,685 clean faces with zero
mismatches or render errors, and the shape census exited successfully. No
generated production traffic was enabled.
