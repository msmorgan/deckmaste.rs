---
needs: [workbench-macro-discipline-2]
---
**Add the three macros the bench-fidelity sweep found missing.** Residues of
`workbench-bench-fidelity` (2026-09-03), each a raw-core site that no macro
can spell today:

- `Macros.returnTo` hard-codes an empty rider list, so Open the Vaults
  (`Cards.openTheVaults`, raw `Enact "Return"` with `[Under …]`) cannot read
  through it; give `returnTo` its rider list.
- No `Macros.libraryZ`, so `ZoneAt Library Bare` is spelled raw; add it beside
  `handZ`/`graveyardZ`.
- No set-life macro, so four `ChangeLife w (Set a)` sites are raw; add one
  macro per lemma ("your life total becomes N").

Size: S. Done when: the named sites read through macros, no macro duplicates
another, build 23/23. Standard constraints apply.

## As landed

- `Macros.returnTo` now takes a positional `riders : List (TokenRider (nomIntro n))`
  with the matching `RidersFit` obligation; `Cards.openTheVaults` reads through it
  as printed (`[Under (PossessorsOf OwnerAx (It ManyOf))]`), and the six other
  `returnTo` call sites plus `returnToBattlefield` pass `[]`.
- `Macros.libraryZ` added beside `handZ`; the one `Cards.idr` raw
  `ZoneAt Library Bare` (`mirelurkQueenTrigger`) reads through it. The three
  remaining raw sites are in `ProofsC`/`ProofsG`, which the concurrent
  `workbench-pin-hygiene` round owns — not touched.
- Set-life: no macro was added. `Macros.lifeTotalBecomes` already spells this
  lemma (`Macros.idr:1234`, two live bench uses), so the four raw
  `ChangeLife w (Set a)` sites were re-spelled through it.

## Landing record

- Gates: `cd idris && ./scripts/build` — 23/23 modules, 0 `Error`/`Warning`
  lines; last line `23/23: Building Cards (src/Cards.idr)`.
  `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`. `cargo xtask cite check`:
  `checked 17797 citations against cr.txt (eff. 2026-08-07); 0 stale`.
  `cargo xtask cite audit --diff`:
  `audited 0 citation site(s) — nothing selected`.
- Assurance counts: restored 0; re-spelled 12 `Cards.idr` definitions
  (`obeliskOfUndoing`, `flickeringWardBounce`, `openTheVaults`,
  `midnightScavengers`, `custodiSquire`,
  `mistbreathElder` (two sites), `mirelurkQueenTrigger`, `theGoldenThrone`,
  `stunningReversal`, `exquisiteArchangel`, `elderscaleWurm`); ignored 0;
  added 1 macro (`libraryZ`); removed 0 pins, witnesses or macros.
- Deviations and additions: the ticket's third item asked for a set-life macro;
  `Macros.lifeTotalBecomes` already exists, so per the ticket's own
  "no macro duplicates another" the four sites were re-spelled instead of a
  second spelling being added. No gate was added, so no pin was required.
  `returnToBattlefieldTransformed` and `returnToBattlefieldWithCounters` still
  build their `Move` inline rather than routing through the widened `returnTo`;
  outside this ticket's letter, left for a macro round.
- STOP: none.
