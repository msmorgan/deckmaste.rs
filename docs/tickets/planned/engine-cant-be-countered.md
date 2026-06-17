---
needs: []
---
"Can't be countered" deontic. `Counter` exists only as an effect `Action`, not as
a `DeonticAction` — so "~ can't be countered" [CR#701.6a] has no representation,
and the engine has no eval hook on the counter-resolution path to enforce it.

Add `DeonticAction::Counter` (so a `Cant(Counter(This))` static can be authored)
+ the eval at the point a spell/ability would be countered (state-based or
replacement-style "this can't be countered" check).

~20 one-away cards (`~ can't be countered.`) + more multi-blocked. The
parse-static-breadth worker built the rest of the Cant/block-restriction family
but deferred this one as the only member needing a new engine primitive.
