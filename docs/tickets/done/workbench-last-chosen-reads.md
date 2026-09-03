---
needs: []
---
**Rename the three "last chosen" reads to carry the printed word, and re-gate
`ChosenNumber` and `ExposedChoice` to `= 1`.** Cleanroom review 2026-09-03,
R1, ruled.

`Phrase.LastChosenPlayer` (`Phrase.idr:232`), `OfLastChosen`
(`Phrase.idr:242`), `LastChosenColor` (`Phrase.idr:1807`), `ChosenNumber`
(`Phrase.idr:1839`) and `ExposedChoice` (`Phrase.idr:2715`) all gate on
`ChoiceStands (countChoice q bs)` — "at least one stands" (`Words.idr:1298`) —
and read the most recent. That is the one place the workbench resolves by
recency.

## The ruling

"The last chosen X" is a **printed lemma whose meaning is the word "last"**.
It is not anaphora by preference, and it is not an exemption to "recency is
permanently off": the recency is in the card's own words, so a constructor may
carry it as long as the name says so. The supported corpus prints "the last
chosen …" on 12 cards (Chromatic Armor, Sanctuary Blade and Psychic Paper
bench it through `OfLastChosen`, `Cards.idr:5538/5605/5617`).

So: keep the three printed-lemma reads and rename them to carry the printed
word — `TheLastChosenPlayer`, `OfTheLastChosen`, `TheLastChosenColor` (or the
nearest names that read as the lemma) — with a declaration note saying the
recency is lexical.

`ChosenNumber` and `ExposedChoice` print no "last". They are ordinary chosen
reads and must be gated `= 1` like `OfChosen` / `ChosenPlayer`; the `≥ 1`
gate is a recency read wearing a plain name.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; the three
printed-lemma reads carry the printed word in their names and a declaration
note stating the recency is lexical; `ChosenNumber` and `ExposedChoice` gate
on `= 1`; every bench site of all five is re-spelled and typechecks, and any
site the tightened gate now refuses is either re-spelled or recorded as
unspellable with its printed sentence; a pin refutes `ChosenNumber` where two
choices stand, and it is non-vacuous. Standard constraints apply.

## As landed

- `Phrase.LastChosenPlayer` → `TheLastChosenPlayer` (gate unchanged,
  `ChoiceStands (countChoice PlayerC bs)`), with a one-line declaration note
  that the recency is lexical.
- `Phrase.OfLastChosen` → `OfTheLastChosen` (gate unchanged), same note.
- `Phrase.LastChosenColor` → `TheLastChosenColor` (gate unchanged), same note.
- `Phrase.ChosenNumber` re-gated `ChoiceStands (countChoice (QSort Number) bs)`
  → `countChoice (QSort Number) bs = 1`.
- `Phrase.ExposedChoice` re-gated `ChoiceStands (countChoice q bs)` →
  `countChoice q bs = 1`.
- Added `Phrase.TheLastChosenNumber` (`Amount`, `ChoiceStands` gate, same
  declaration note) — the printed-lemma read the tightened `ChosenNumber`
  correctly refused on Shapeshifter (see Deviations).
- Witnesses kept under the new names: Chromatic Armor (`Cards.idr:5538`),
  Sanctuary Blade (`:5605`), Psychic Paper (`:5617/5618`) through
  `OfTheLastChosen`; `ProofsG.lastChosenPlayerRead` (two standing player
  choices) and `ProofsG.secondChooserDevotionRead` (two standing colour
  choices) through `TheLastChosenPlayer` / `TheLastChosenColor`;
  `ProofsAnaphora.ofLastChosenColorReadsOnlyPrefix` /
  `…ResolvesInPrefix` and `ProofsF.badLastChosenColorNoChooser`,
  `badLastChosenBeforeChooser`, `badLastChosenWrongSort`.
- Pin added: `Proofs.badChosenNumberTwoStanding` — `ChosenNumber` at
  `[qualityB Number, qualityB Number]`, `Refl impossible`.
- Nothing left undone. No macro renames were needed: none of the five reads
  has a `Macros.idr` spelling.

## Landing record

Before → after (`idris/src/Experimental/`):

- printed-lemma reads carrying the printed word: 0 → 4
  (`TheLastChosenPlayer`, `OfTheLastChosen`, `TheLastChosenColor`,
  `TheLastChosenNumber`).
- reads gated `ChoiceStands` (≥ 1): 5 → 4; reads gated `= 1` among the
  ticket's five: 0 → 2 (`ChosenNumber`, `ExposedChoice`).
- `Amount` constructors: 34 → 35. Pins in `Proofs.idr`: +1.
- Bench sites re-spelled: 6 in `Cards.idr` (4 `OfTheLastChosen`, 2 on
  Shapeshifter), 6 in the proof modules (`ProofsG` 2, `ProofsF` 3,
  `ProofsAnaphora` 1).

Bench resolution under the tightened gates — every surviving site resolves at
exactly one standing choice: `ChosenNumber` on Sanctum Prelate
(`Cards.idr:6058`), Expel the Interlopers (`:6104`), Celestial Judgment
(`:11224`), Talion, the Kindly Lord (`:13562`); `ExposedChoice` on Emissary of
Grudges (`:15533`). One site did not: Shapeshifter (`:10236/10238`), which has
two standing number choices (the as-enters choice and the upkeep choice) — its
printed line is "Shapeshifter's power is equal to the last chosen number and
its toughness is equal to 7 minus that number", so it reads by the printed
word and was re-spelled with `TheLastChosenNumber`, not by loosening the gate.

Gates:

- `cd idris && ./scripts/build` (after `rm -rf build`): `23/23: Building Cards
  (src/Cards.idr)`, 0 Error and 0 Warning lines.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 17719 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`: `audited 0
  citation site(s) — nothing selected` (the round adds no CR citations, so
  `cr-citations.lock` is untouched and no `bless` was run).

Non-vacuity probes (scratch module, deleted afterwards):

- positive twins typecheck — `ChosenNumber` at `[qualityB Number]`,
  `TheLastChosenNumber` at `[qualityB Number, qualityB Number]`,
  `ExposedChoice PlayerC` at `[choiceB PlayerC]`.
- the pin mis-stated at ONE standing choice fails: `Error: misPin Refl is not
  a valid impossible case.` — the landed pin at two standing choices compiles,
  so it refuses something.
- the `ExposedChoice` re-gate bites: `ExposedChoice PlayerC` at
  `[choiceB PlayerC, choiceB PlayerC]` is refused with `Can't find an
  implementation for countChoice PlayerC [choiceB PlayerC, choiceB PlayerC] =
  1`.

Assurance: restored 0, re-spelled 12 (6 bench sites, 6 proof sites), ignored 0,
added 1 (`badChosenNumberTwoStanding`), removed 0.

Deviations and additions:

- **Added `Phrase.TheLastChosenNumber`** beyond the ticket's letter. The
  ticket allowed a refused site to be "re-spelled or recorded as unspellable";
  Shapeshifter's printed text carries "the last chosen number" (verified
  against MTGJSON oracle text, and the corpus prints that phrase), so under
  ruling R1 it is the same printed lemma as the other three and the faithful
  re-spelling is a lemma read, not a retirement. Deleting the bench card would
  have been an assurance removal.
- The pin's doc line names Shapeshifter's shape but quotes the refused
  (hypothetical) "the chosen number" wording, not the card's printed line.
- Ledger: `docs/idris-workbench-closure-tables.md` still names
  `Phrase.OfLastChosen` / `Phrase.LastChosenColor` in three rows and the
  `OfLastChosenColor` entry; the round's edit scope was `idris/` plus this
  ticket, so the table was not touched and needs a follow-up pass.

Working-copy divergence: `jj workspace update-stale` (a sibling workspace had
advanced the shared op log) left `yusqtkox` divergent. The halves differed only
in this ticket file; `236145f6` carried the landing record and `5d51f68e` did
not, so `jj edit 236145f6` / `jj abandon 5d51f68e`. The build gate was re-run
clean on the kept half after the rebase.

STOP taken: none.
