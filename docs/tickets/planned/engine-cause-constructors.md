---
needs: []
---
`event::Cause { verb, agency, agent }` is hand-built at ~8 emission sites with literal
verb strings (a typo'd verb silently never matches a trigger); the SBA `Destroy` cause
is byte-identical at `sba.rs:110` and `resolve.rs:309`. Add named `impl Cause`
constructors in `event.rs` to centralize verb spelling and `agent` packing (the
`agency` coordinate stays per-site, since tap-for-cost vs tap-by-effect vs tap-to-attack
are genuinely different). Also add a `relocate_from_current(object, to, cause)` builder
to kill the repeated `ZoneWillChange { enters/position/face: None }` boilerplate plus
the current-zone lookup across the move verbs (`resolve.rs:336`/`488`/`513`/`535`,
`decide.rs:805`). Pure refactor; the open `Ident` verb vocabulary stays a seam.
