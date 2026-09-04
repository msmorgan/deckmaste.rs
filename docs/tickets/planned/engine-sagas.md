---
needs: [core-saga-chapters, engine-counters-api]
---
Lore counter advancement, chapter ability firing, final-chapter sacrifice, and
read-ahead. Needs chapter-ability structure (core-saga-chapters) and counter
apply machinery.

Residue routed from `saga-sacrifice-is-state-based` (2026-09-04): the
[CR#714.4] sacrifice is conferred on the Saga subtype in the v2 stub
(`Property::StateBased` over `Count::GreatestWatchedThreshold(This)`) and
executes on the core-native path, but a Saga loaded through
`Plugin::load(plugins/builtin)` carries no such conferral because the v1
registry is read as `deckmaste_semantics`, which is deletion-bound; that
path converges at cutover. Also unexpressed on either path: [CR#714.4]'s
second clause, "isn't the source of a chapter ability that has triggered
but not yet left the stack".
