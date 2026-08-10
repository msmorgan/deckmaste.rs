---
needs: [engine-payment-obligation-window]
design: true
---
Generalize payment-decline replay identities beyond the zone-change and mana
handles required by the initial payment window.

The first cut deliberately gives stable logical identities to card objects
created or reminted through `ZoneChange`, and rebinds the decisions and facts
used by the tracked acceptance matrix. It does not yet promise stable handles
for every engine allocator. In particular, a floating replacement instance is
currently recorded by its rollback-local numeric key, a stack copy is minted by
`Copied` without a creation `ZoneChange` and may share its source with the
original, and an emblem is minted outside the card-zone identity path.

Choose one generation-aware logical identity space that can represent those
families without conflating coexisting objects that share an `ObjectSource`.
Use it for transcript decisions and fact comparison as well as reconstruction.
Pin the design with reversal tests in which an earlier record shifts each
allocator and a later retained record chooses or uses the newly allocated
replacement, stack copy, or emblem. Do not extend the current source-keyed map
as a shortcut for stack copies: source identity is not instance identity.
