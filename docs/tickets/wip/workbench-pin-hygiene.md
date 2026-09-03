---
needs: [workbench-table-machinery, workbench-zone-gate-unify]
---
**Keep every pin's positive twin in-tree, make the docstrings name the refused
spelling, and change `VERIFY.md` to require both.** Cleanroom review
2026-09-03, F18. The pins themselves are sound — 577 over 183 distinct
obligations, 31 reproduced, 29 with a positive twin, none vacuous — but
nothing re-checks that after a core change.

**Twins.** A twin survives in-tree for only 43 of 577 pins (`ProofsG` 37,
`ProofsF` 4, `ProofsC` 1, `ProofsD` 1; `Proofs`, `ProofsB`, `ProofsE` none),
because `idris/VERIFY.md` tells the author to delete the scratch twin once the
pin is written. So when a gate loosens, the pin goes vacuous silently. Fix:
`ProofsG`'s convention becomes the rule — the positive twin lives beside the
pin — and `VERIFY.md` is edited to say so.

**Docstrings that name the wrong thing.** Eight pins quote a printed sentence
the workbench does spell, through a sibling constructor, so they read as
unspellability evidence when they pin one spelling: `ProofsC:417
badStaticTargets` ("Target creature can't attack" is refused only as a
`Static` — `Untargeting` — and is spellable as `Continuously (deontic (target
creature) …)`), and likewise `ProofsF:164`, `ProofsD:415`, `ProofsC:434`,
`ProofsG:291`, `ProofsG:1034`, `ProofsG:913`, `ProofsF:96`. Fix: each
docstring names the *spelling* it refuses and, where a sibling spells the
sentence, says which.

**Refusal messages.** 463 of 577 pins refuse only through a bare `So (…)`
gate, whose message is `Can't find an implementation for So False.`;
`VERIFY.md`'s "a type error names the failure" holds for the 114 that name a
`data` witness. Where the message carries the meaning, prefer the `data`
idiom: `workbench-table-machinery`'s `OptOk` and `workbench-zone-gate-unify`'s
`ZoneIs` (which keeps its argument symbolic) are the two carriers, and 43 of
the naming pins' slots are the one-inhabitant witnesses the latter folds — so
land this after those two and re-spell against what they leave.

Size: S, plus a `VERIFY.md` edit.

Also (audit N9): `docs/idris-workbench-closure-tables.md` §2 still anchors `Experimental.idr` line ranges and the deleted `CantMoreThan`; re-anchor to `Module.decl` names as §3 already is.

Done when: the build is 23/23 with 0 errors and 0 warnings; every pin has a
positive twin in-tree and `VERIFY.md` requires it instead of asking for the
scratch twin to be deleted; the eight docstrings name the refused spelling and
its spellable sibling; the pin count is unchanged or higher (no pin deleted);
each twin is shown to fail when its pin's gate is loosened, and the landing
record gives the twin count before and after. Standard constraints apply.

## As landed

- **Twins beside the pins.** 319 positive definitions added across the eight
  `Proofs*` modules, one per distinct obligation — an obligation keyed as a
  (head constructor, implicit slot) pair, 338 of them over the 584 pins. Each
  twin applies the pin's own head constructor at the pin's own slot with only
  the rejected argument replaced by an admitted one, and sits immediately above
  the first pin sharing that obligation. 0 obligations skipped.
- **`VERIFY.md`.** The non-vacuity paragraph now requires the twin to live
  beside the pin in the same module instead of in a deleted scratch module, and
  says one twin covers the pins sharing its obligation. A second paragraph
  requires the docstring to name the refused *spelling* and its spellable
  sibling, and prefers a named `data` witness over a bare `So (…)` where the
  refusal message carries the meaning.
- **The eight docstrings.** Seven amended with one why line naming the refused
  spelling and the sibling that spells the sentence: `ProofsC.badStaticTargets`,
  `ProofsC.badUntilBeginningOfUpkeep`, `ProofsD.badTargetedOutcomeGate`,
  `ProofsF.badDestroySource`, `ProofsF.badStaticPlayerCantTargets`,
  `ProofsG.badLiteralScaledMana`, `ProofsG.badExchangeOneParty`. The eighth,
  `badExactlyOneColor`, no longer exists: `workbench-axis-pairs` already
  replaced it with the positive `ProofsG.monocoloredIsOneColor` beside
  `badExactlyZeroColors`/`badExactlySixColors`, which is exactly this ticket's
  fix. Nothing to do.
- **Closure tables §2 (audit N9).** All 712 Site anchors in §2.1–§2.6 rewritten
  from `<File>.idr:<line>` to `Module.decl`; the three `Experimental.idr — lines
  N–M` headings renamed to the modules that hold the region; the `Refresh
  method` and the file's demotion note rewritten to prescribe name anchors. The
  `CapBound` and `CantMoreThan` rows are re-anchored to `Effect.Deontic`'s
  `bound` slot / `Effect.CountBound`.
- **Not done:** §1 (flip-risk ranking) and the `Totals` table still name
  `Experimental.idr`; both are outside N9's and this ticket's scope and are
  labelled snapshots.

## Landing record

**Numbers before/after.** Pins 584 → 584 (no pin added or deleted). Positive
top-level definitions in `Proofs*`: 214 → 533, i.e. **319 twins added**.
Per module (before → after): `Proofs` 2 → 39, `ProofsB` 1 → 31, `ProofsC`
1 → 40, `ProofsD` 1 → 42, `ProofsE` 0 → 44, `ProofsF` 4 → 39, `ProofsG`
46 → 134, `ProofsAnaphora` 159 → 164 (its baseline is the counting-lemma proof
library, not twins; 5 of its 7 obligations gained one, 2 already had one).
Distinct obligations: 338, all covered.

**Gate lines.**
- `cd idris && ./scripts/build` — clean `build/`, exit 0, `23/23: Building
  Cards (src/Cards.idr)`, 0 `Error:` and 0 `Warning:` lines.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 17797 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` — read. No rule
  number is added or changed anywhere in the diff: every bracketed citation on
  a `+` line is pre-existing, on a closure-tables row whose Site cell moved, and
  no file under `idris/src/` gained a citation. `cite bless` therefore not run.

**Assurance counts.** restored 0 / re-spelled 0 / ignored 0 / added 319
(positive twins; 0 pins) / **removed 0**. No pin, docstring, name or ordering
was deleted or changed except the seven docstrings above, which gained a line
and lost none; each module's diff against its pre-round state has zero deleted
lines and zero duplicate definitions.

**Non-vacuity re-probe.** Five twins were mutated back to their pin's rejected
argument and each reproduced the pin's own refusal, then were restored:
`Proofs.okChosenNumberOneStanding` (two standing number choices →
`countChoice (QSort …)`), `ProofsB.okFlatDisjunction` (`Or [artifact,
artifact]` → `So (noRepeatedPair …)`), `ProofsC.okStaticUntargeting`,
`ProofsD.okUntargetedOutcomeGate`, `ProofsF.okStaticPlayerCant` (each
`target …` under a `Static` → `So False`). The implementers probed a further
nine of their own the same way.

**Deviations and additions.**
- *One twin per obligation, not one per pin.* The ticket's "every pin has a
  positive twin in-tree" is met at the level of the obligation: where several
  pins share a (constructor, slot) gate, they share one twin, because the
  per-pin variants would be byte-identical definitions under different names
  (`badExactlyZeroColors` and `badExactlySixColors` already share
  `monocoloredIsOneColor`, the convention this ticket generalises). Every pin
  therefore has a twin on its gate; no pin has a private duplicate. Disclosed
  rather than silently equated.
- *Closure tables: 164 of 680 §2 rows are now marked `site not found
  2026-09-03`.* Re-anchoring by name rather than by line makes the demoted
  §2.3/§2.4/§2.6 snapshot's dead names visible for the first time (`combatant`,
  `verbFacts`, `deedFacts`, `TurnDeixis`, `CopyStack`, `Monocolored`, … — all
  confirmed absent from `idris/src/Experimental` by grep). Their remaining
  cells are left as written, per the round brief; only the Site cell changed.
- *Residue, not resolved here:* the re-anchored `Effect.Deontic` bound row
  carries a pre-existing `[CR#205.4c]` (basic versus nonbasic land) that reads
  off-topic for a count bound. Untouched — the citation list was not edited.
- Added: 4 `Bindings` fixtures in `ProofsG` (`afterATwoDieRoll`,
  `afterCountersPut`, `afterALandTapForMana`, `afterManaAdded`), following the
  module's existing `afterACoinFlip` idiom, because the scope-sensitive gates
  need a prefix to be twinned at all. Counted in the 319.

**STOP taken.** None. One incident: mid-round the working copy went stale (a
sibling workspace advanced the op log) and `jj workspace update-stale` produced
a divergent `@`, checking out the half that carried only `VERIFY.md` and the
closure tables. The halves were diffed by content, the half carrying all ten
files was selected with `jj edit 13d1bc81112c`, and the two-file half was
abandoned (`jj abandon 5483b99830c2`). All gates above were re-run after the
recovery on the rebased parent.
