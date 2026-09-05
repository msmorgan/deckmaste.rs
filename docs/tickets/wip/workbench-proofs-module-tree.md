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

## As landed

Fourteen `idris/src/Experimental/Proofs<Family>.idr` files moved (jj-tracked
renames) to `idris/src/Experimental/Proofs/<Family>.idr` with module header
`Experimental.Proofs.<Family>` (Anaphora, Choice, Counters, Damage, Deontic,
Description, Faces, Keyword, Mana, Piles, Static, Trigger, Turn, Zone). Each
moved file's self-qualified references to its own declarations
(`ProofsX.someName`, matching the old module's last path component) were
re-qualified to `X.someName` to match the new namespace's last component —
the same self-qualification style already used in `Experimental/Cards/*.idr`
(e.g. `Anaphora.anyTargetAnnounced`); no cross-family imports existed, so no
`import Experimental.ProofsX` lines needed fixing anywhere in `idris/src/`.

Added `idris/src/Experimental/Proofs.idr`: `module Experimental.Proofs` plus
`import public Experimental.Proofs.<Family>` for all fourteen families (same
order as the old `mtg.ipkg` listing), `%default total`, mirroring
`Cards.idr`.

Updated `idris/mtg.ipkg`: the fourteen flat `Experimental.ProofsX` entries
replaced with `Experimental.Proofs` followed by the fourteen
`Experimental.Proofs.<Family>` entries (umbrella placed immediately before
its family list, mirroring where `Experimental.Cards` sits relative to
`Experimental.Cards.<Family>`). `idris/mtg-dev.ipkg` lists no Proofs modules
(they're excluded from the fast inner loop) so only its three comment lines
mentioning `Proofs*` were reworded to `Proofs` (no glob needed since the
comment now names the tree via its umbrella, not a flat-file pattern).

Updated `idris/scripts/check-pin-twins` line 80: glob
`src/Experimental/Proofs*.idr` -> `src/Experimental/Proofs/*.idr`. The
script's other logic (declaration parsing, `os.path.relpath` for reporting)
carries no module-name assumption, so no further change was needed there.

Updated `idris/scripts/build` line 4 comment: `Proofs*.idr are exempt` ->
`Proofs/*.idr are exempt`.

Updated `idris/VERIFY.md`: the `` `Proofs*` `` pin-module references (lines
42, 82, 87) reworded to `` `Proofs` ``; the family-listing line
`` `Experimental.Proofs<Family>` `` became `` `Experimental.Proofs.<Family>` ``
and its parenthetical family list dropped the `Proofs` prefix from each name
(`Anaphora`, `Description`, `Zone`, `Damage`, `Trigger`, `Static`,
`Counters`, `Mana`, `Keyword`, `Deontic`, `Choice`, `Turn`, `Faces`,
`Piles`), matching the bare-family-name style already used for the `Cards`
family list two lines above it.

No comments were added and no code changed beyond module headers, the
in-file self-qualified references described above, and imports/module
lists.

## Landing record

**File moves** (jj-tracked renames, `idris/src/Experimental/`):
`ProofsAnaphora.idr` -> `Proofs/Anaphora.idr`, `ProofsChoice.idr` ->
`Proofs/Choice.idr`, `ProofsCounters.idr` -> `Proofs/Counters.idr`,
`ProofsDamage.idr` -> `Proofs/Damage.idr`, `ProofsDeontic.idr` ->
`Proofs/Deontic.idr`, `ProofsDescription.idr` -> `Proofs/Description.idr`,
`ProofsFaces.idr` -> `Proofs/Faces.idr`, `ProofsKeyword.idr` ->
`Proofs/Keyword.idr`, `ProofsMana.idr` -> `Proofs/Mana.idr`,
`ProofsPiles.idr` -> `Proofs/Piles.idr`, `ProofsStatic.idr` ->
`Proofs/Static.idr`, `ProofsTrigger.idr` -> `Proofs/Trigger.idr`,
`ProofsTurn.idr` -> `Proofs/Turn.idr`, `ProofsZone.idr` -> `Proofs/Zone.idr`.
Added `Proofs.idr` (umbrella). Non-source files touched:
`idris/mtg.ipkg`, `idris/mtg-dev.ipkg`, `idris/scripts/check-pin-twins`,
`idris/scripts/build`, `idris/VERIFY.md`.

**`cargo xtask map idris` diff (before vs. after the move):** identical for
all fourteen families — every family's `data`-declaration dump is empty
both before and after (the pin modules declare no `data` types themselves;
they only consume `data` declared in `Words`/`Events`/`Card` etc.), so the
before/after diff is empty for each of the fourteen. Confirms no
declaration-set change from the move.

**Gates (foreground), last line of each:**
- `cd idris && ./scripts/build` (clean, `rm -rf build` first): 47/47
  modules built, `47/47: Building Cards (src/Cards.idr)`; 0 Error/Warning
  lines (`grep -icE 'error|warning'` on the captured log: 0).
- `idris/scripts/check-pin-twins` (run standalone, also invoked inside
  `./scripts/build`): exit 0, no output (0 twinless pins) both before and
  after the move.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14435 citations against cr.txt (eff.
  2026-08-07); 0 stale`.

**Pin counts** (via the same declaration-scan `check-pin-twins` uses,
summed per family): before = 649, after = 649 (Anaphora 80, Choice 33,
Counters 47, Damage 66, Deontic 33, Description 52, Faces 72, Keyword 36,
Mana 43, Piles 13, Static 19, Trigger 24, Turn 42, Zone 89 — unchanged
per-family too).

**Module count:** 46 before (per `mtg.ipkg`'s prior flat list) -> 47 after
(fourteen families + the new `Experimental.Proofs` umbrella replacing the
fourteen flat top-level entries, net +1), matching the ticket's expected
"module count plus one."

No semantic change; no regression found; nothing removed beyond the
renamed-away flat files (replaced 1:1 by their `Proofs/` counterparts).
