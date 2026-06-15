---
needs: []
---
An activated ability whose effect counts "<things> you control"
(`Create` / `AddMana(CountOf(AllOf([Subtype(X), ControlledBy(Ref(You))])), …)`)
over-counts by one: the activation mints a Stack-zone object that reuses the
SOURCE's `CardId`, and the unzoned "you control" filter matches that on-stack
copy too. With three Elves, Elvish Archdruid taps for 4 (should be 3); Krenko,
Mob Boss makes 4 tokens (should be 3); Priest of Titania is the same. This is the
LIVE count path (`eval_count` / `filter_matches_live`) — the watcher is threaded
correctly, so this is a ZONE-scoping gap, not the frameless-carrier one
(engine-static-scope-carrier). Fix: narrow permanent "you control" counts to the
battlefield (exclude the activating source's own Stack-zone LKI), or canonicalize
such filters with `InZone(Battlefield)` at parse. Verify the three cards above
count exactly their battlefield matches. Part of demo-goblins-elves.
