---
needs: []
---
Generator task. Add a "Destroy target <subject>" production to the shared effect
parser (`effect.rs` `parse_clause`) — declares `TargetOne(<filter>)` (subject via
`filter.rs`) and renders the effect body `Destroy(Target(0))` [CR#701.8]. Benefits
every frame that shares the effect grammar (spell/triggered/activated), so removal
spells and "when X, destroy target Y" triggers graduate. ~869 unparsed "destroy
target …" lines today. The engine already models destroy
(`Action::Destroy(Selection)`, destroy-with-cause). Board wipes ("destroy all/each
…") and durational pump ("gets +N/+N until end of turn" — needs the `Continuously`
RON encoding established first, since nothing uses it yet) are separate follow-ups.
Feeds canon-goblins-elves; part of tui-client.
