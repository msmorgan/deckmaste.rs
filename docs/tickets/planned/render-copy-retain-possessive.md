---
needs: [core-copy-grammar]
---
Render a copy-exception `Retain` with the source's possessive, not a flat "its".
`core-copy-grammar` renders `CopyException::Retain(char)` via `copy_retain_clause`
(`crates/deckmaste_cards/src/render/effect.rs:2377`, call site `:2255`), which always emits
"its [characteristic]" ("except it doesn't copy its color" [CR#707.9c,707.9d]). That reads
correctly when the copier is the object gaining the exception, but the sole real precedent —
Vesuvan Doppelganger — prints "that creature's [characteristic]" because the retained
characteristic belongs to the copied object, not the copier. The clause hardcodes the "its"
possessive instead of rendering the possessive case of the exception's source `Reference`.

Make `copy_retain_clause` take (or the caller pass) the source reference and render its
possessive form — "its" when the source is the copier/self, "that creature's" / "that
permanent's" when it names another object — reusing the existing reference-render path.
Deferred from Task 7: no in-repo consumer needs it yet (Vesuvan Doppelganger is not in the
curated canon), so there is no fixture to graduate; add one when a possessive-retain copy card
is authored. RON round-trip and the self-possessive "its" path are already covered.
