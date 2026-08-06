---
needs: []
---
**Engine: `Status::Flipped` / `Status::Unflipped` are unevaluated predicates.**

The `Status` predicate arm in `crates/deckmaste_engine/src/target.rs` and its
LKI/snapshot twin in `crates/deckmaste_engine/src/trigger.rs` cover no status
values yet. The arm's other values have owners — `PhasedOut`/`PhasedIn` →
`engine-phasing`, `FaceDown`/`FaceUp` → `engine-face-down` — but
`Flipped`/`Unflipped` had none.

[CR#110.5] makes flipped/unflipped one of the four status categories every
permanent always has a value for, and [CR#710.3] is the rule that requires
that value be tracked and unambiguous at all times — which is what the engine
does not do yet. [CR#710.1] describes the card frame itself: flip cards carry
a second set of characteristics printed upside down, so a flipped permanent
has a different name, types, and P/T. The census lists 20 `flip`-layout cards
(`shape-flip`), so this rides on that shape landing.

A filter over an unevaluated status must keep tripping rather than silently
read a default — the answer changes targeting legality.

Effort: **S** once `shape-flip` exists.
