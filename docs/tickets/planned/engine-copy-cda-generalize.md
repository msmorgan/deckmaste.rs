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
For power/toughness the behaviour is unchanged — Quicksilver-of-Tarmogoyf is 7/7 with
no `*/1+*` CDA, and a Populate of that token stays 7/7 [CR#707.3].

**Behaviour changes on colour and subtypes (corrected 2026-10-21).** The current drop
covers only P/T: `crates/deckmaste_engine/src/copy.rs:30-38` (module doc, "Power/Toughness
axes only") and `retain_characteristic` (`copy.rs:574-594`, a no-op arm for `Colors`
and `Subtypes`); the Modify path (`copy.rs:334`, `:337`) does no CDA drop either.
[CR#707.9d] drops the copied CDA for any characteristic the exception provides or
retains, and for colour also the colour indicator, except where the exception says
"in addition to its other types". Devoid ([CR#702.114a]) and changeling
([CR#702.73a]) are CDAs ([CR#604.3a]). So a "1/1 green Frog" copy exception
(Croaking Counterpart) over a devoid creature must come out green, not colourless
(live: devoid's layer-5 colour set is implemented), and a changeling copy with a
subtype exception must drop changeling (latent while `AllCreatureTypes` is a no-op,
`layer.rs:1925`). Extend the layer-1a fold to colour and subtype CDAs and the colour
indicator, honouring the carve-out. An exception that ADDS a colour over a
colour-defining CDA is underdetermined in the CR; register it rather than guess.
Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #31.
