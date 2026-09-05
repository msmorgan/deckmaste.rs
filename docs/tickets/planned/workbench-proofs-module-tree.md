---
needs: [workbench-witness-sweep-residues, workbench-deed-guard-residues, workbench-tapped-for-chosen-mana-type]
---
**Move the pin modules under `Proofs/` with a `Proofs.idr` umbrella,
mirroring `Cards.idr` and `Cards/`.** Ruling 2026-09-04. Today the fourteen
`Experimental.Proofs<Family>` modules sit flat beside `Cards/` and are
listed one by one in `mtg.ipkg`; `Cards.idr` already shows the intended
shape (`import public Experimental.Cards.<Family>`).

Mechanical, no semantic change: rename each `ProofsX.idr` to
`Proofs/X.idr` with module header `Experimental.Proofs.X`; add
`Proofs.idr` (`module Experimental.Proofs`) that `import public`s every
family; update `mtg.ipkg` (and `mtg-dev.ipkg` if it lists them), every
`import Experimental.ProofsX` across `idris/src/`, the glob in
`idris/scripts/check-pin-twins`, the exemption comment in
`idris/scripts/build`, and `idris/VERIFY.md`. Verify with `cargo xtask map
idris` per module before and after (declaration sets identical), the build
at its module count plus one (the umbrella), and `check-pin-twins` still
finding every pin.

Size: S. Done when: no `Experimental.Proofs<Family>` module remains at the
top level; `Experimental.Proofs` imports all of them; build green with the
pin-twin check reporting the same pin count. Standard constraints apply.
