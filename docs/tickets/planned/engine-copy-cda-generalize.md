---
needs: [core-copy-grammar, engine-layers-1-copy-facedown-text]
---
Remove the bespoke CDA-drop special-casing in copy-exception application.
`core-copy-grammar` applies a `CopyException::Modify`/`Retain` by folding it into the
copiable values at layer 1a and, per [CR#707.9d], stripping the source's
characteristic-defining ability for any characteristic the exception provides or
retains — implemented as an explicit "find the CDA that defines this characteristic and
drop it" pass in the copy-application code. That strip is messy and easy to get subtly
wrong. Once `engine-layers-1-copy-facedown-text` lands the real layer-1a
copiable-values machinery, model "the defining ability is not copied" [CR#707.9d] as a
consequence of copiable-value derivation (the providing/retaining exception participates
in the layer-1a fold) rather than a bespoke ability-filter, and delete the special case.
Behavior is unchanged — Quicksilver-of-Tarmogoyf is 7/7 with no `*/1+*` CDA, and a
Populate of that token stays 7/7 [CR#707.3] — this is an internal cleanup.
