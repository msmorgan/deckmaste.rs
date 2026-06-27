---
needs: []
---
**Idris grammar: the naming residuals a prior sweep didn't reach.** Mechanical renames and
field-alignments in `idris/src/Core.idr`; one build + cite cycle, no design. From the
2026-06-26 grammar census.

1. **Un-abbreviate `MkCaps` / `MkQuery` / the `evCaps` field** — they abbreviate where
   `MkBindings`/`MkCharacteristics` spell it out.
2. **`ThatMuch` → `EventAmount`** — it breaks the `EventObject`/`EventActor` event-anaphor
   naming family.
3. **`Characteristic.CardTypes` vs its field `types`** — the only constructor/field name
   mismatch; align them.
4. **`ChoiceRefKind` (a gate) collides with the `chosenRefKind` field** and reads like a
   projection though it returns a predicate gate — rename the gate.
5. **`TollTiming` → `PricedTiming`** — the constructor became `Priced` (see
   `idris-grammar-collapses`) but the timing enum kept the old "Toll" name.
6. **`Promote` subtype instances ordered Land-before-Artifact**, against the `data Subtype`
   / `subtypeCategory` order — reorder to match.
7. **`Sacrifice` should take `{default You actor}`** — the conjugation rename landed, but it
   still forces an explicit `Reference b APlayer`; give it the default-actor every other
   player-verb has (the `EventKind` namespace already type-disambiguates).

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
