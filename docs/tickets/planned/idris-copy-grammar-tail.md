---
needs: [core-copy-grammar]
---
Finish the two deferred pieces of the Idris copy-grammar parity that Task 8 left as
documented Gaps. Task 8 unified copy onto existing constructors — `Action.Copy : Reference b
AnObject -> List (Modification b) -> Action b` (bare token copy = `Copy r []`) and
`BecomeCopyOf : Reference -> Modification`, with copy exceptions modeled as SEPARATE
higher-layer modifications (`ApplyAll [BecomeCopyOf src, <exception mods>]`), never bundled
into `BecomeCopyOf`/`Copy` arguments (doctrine at `idris/src/Semantics.idr:2791-2792`). Two pieces
were deferred because no Task 9 macro exercised them:

1. **AsCopy arrival carrier.** `EnterRider::AsCopy` ("enters as a copy of…", Clone-style) maps
   to `BecomeCopyOf src`, and the payload + unit test landed, but the full round-trip needs an
   ETB-replacement carrier that is an unbuilt Rust seam — the runtime AsCopy path fizzles by
   design and its exec is owned by `engine-layers-1-copy-facedown-text`. Emit the carrier once
   that seam lands so a full Clone-style card round-trips parse → render → idris.
2. **`Retain` + `AdditionalEffect` exceptions.** Model `CopyException::Retain` ("doesn't copy
   its color" [CR#707.9c,707.9d]) and `AdditionalEffect(EnterRider)` [CR#707.9e] as sibling
   `Modification`s in the soundness core, following the ApplyAll doctrine above.

Both are additive to the core (no surface change; the RON shapes already parse/render and
`idris-check` exits 0 without them). Deliverable (1) is additionally gated on
`engine-layers-1-copy-facedown-text` landing the ETB-replacement carrier; (2) is pure Idris
modeling and can land independently.
