---
needs: [macro-modification-bundle]
---
Implement `Amass` [CR#701.45] as a canonical example of a "composite macro" that decomposes into smaller representable operations.

"Amass [subtype] N" means:
1. If you don't control an Army [subtype] creature, create a 0/0 black Army [subtype] creature token.
2. Choose an Army [subtype] creature you control.
3. Put N +1/+1 counters on that creature.
4. If it isn't already a [subtype], it becomes a [subtype] in addition to its other types.

This should be implemented as a macro in `plugins/builtin/macros/action/Amass.ron` (or similar) using the infrastructure provided by `macro-modification-bundle`.

Tasks:
1. Author the `Amass` macro body using `Sequence`, `CreateToken`, `PutCounters`, and `AddCardTypes`.
2. Ensure the "If you don't control..." logic is handled correctly (may need a `Conditional` effect).
3. Verify against Orcish Bowmasters or similar Amass cards.
