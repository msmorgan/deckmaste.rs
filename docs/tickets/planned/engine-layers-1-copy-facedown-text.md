---
needs: [core-copy-grammar, engine-face-down]
---
The layer-1/3 work the layers slice left as slots: layer 1a copy ([CR#613.2a]
copiable values from a copy source — the `base_values` seam, gated on
`core-copy-grammar` for "becomes a copy of"), face-down characteristics, and the
layer-3 text-changing effects.

Also owns the **Clone consumer-macro authoring** deferred from `core-copy-grammar`
Task 9. The copy grammar (`EnterRider::AsCopy`, render arms, idris `BecomeCopyOf`)
landed, but the Clone macro itself — plus its full-sentence render arm and behavioral
copy tests — waits on this ticket's AsCopy runtime seam: AsCopy currently fizzles at
runtime by design, and Clone's "enters as a copy … except it's not legendary" shape
(`Instead` + `May`) needs the ETB-replacement carrier this slice builds. Author the
Clone macro here once the seam exists.
