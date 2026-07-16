---
needs: []
---
Split noted-name reads out of `Predicate::Named`. The among-target machinery
made `Named(name)` consult `state.resolution_notes.get(name)` BEFORE the
literal card name (`target.rs`), which overloads note KEYS onto the card-name
string namespace — a stringly binder, which the authored-surface rulings ban.
Nothing enforces that a note key isn't a real card name: a note keyed
"Gray Ogre" → "Shock" silently rewrites every `Named("Gray Ogre")` check
engine-wide (trigger patterns, targeting legality for the next announce,
static filters) for as long as the note lives — and notes are only cleared
when the NEXT resolution begins (`resolve/mod.rs`), so a stale note outlives
its own resolution. Canon avoids collision purely by convention (Cursed
Scroll notes under "CursedScrollName").

Fix: a distinct `NamedNoted(key)` predicate (or typed note keys) so the
literal-name read and the noted-name read are different grammar; while at it,
decide the note lifetime story ("during this resolution" per the `state.rs`
doc vs the actual clear-on-next-resolution) and scope reads accordingly.
Migrate Cursed Scroll's authored RON to the new predicate.
