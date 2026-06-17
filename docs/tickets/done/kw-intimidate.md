---
needs: []
---
Intimidate keyword macro [CR#702.13b]: "can't be blocked except by artifact
creatures and/or creatures that share a color with it" — Fear's `Cant(Block)`
shape with the color clause generalized to `SharesColorWith(This)`.

Lands the general **filter `Subject` mechanism** as its machinery (the share-
color test sits in a per-blocker `Filter` slot, where the blocker is
universally quantified): `Reference::Subject` (the candidate the matcher is
currently testing; re-binds across relation filters for free), `Frame.subject`
threading it into condition evaluation, and `Filter::Where(Condition)` — the
candidate matches iff the condition holds with `Subject` bound. `Reference` is
now a registered macro param type. `SharesColorWith(ref)` inlines the 5-branch
color condition (a macro body can't pass its own `Param` to a nested macro, so
no reuse of a standalone `SharesColor`; the set-theoretic form awaits a color-
set value language). Enforcement of the evasion `Cant` rides the shared later
combat task, as with the other evasion keywords.
