---
needs: []
---
**Retire the checker's zone and object-state refusals (33 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `zoneIs` | 53 | 54 (15 files) | AbilityRules.lean:13,68,69; PhraseRules.lean:120,277,539,589,642,643,650,655,660,676,686,708 |
| `zoneFits` | 54 | 28 (9 files) | AbilityRules.lean:335; PhraseRules.lean:530,631,666,724,735 |
| `zoneCoherent` | 55 | 20 (8 files) | AbilityRules.lean:259; PhraseRules.lean:192,632 |
| `possessable` | 56 | 1 (1 file) | Events.lean:151; PhraseRules.lean:455 |
| `playableFrom` | 57 | 5 (2 files) | PhraseRules.lean:138,678 |
| `placeArrangementFits` | 58 | 2 (1 file) | PhraseRules.lean:474 |
| `placeOrdinalFits` | 59 | 1 (1 file) | PhraseRules.lean:475 |
| `exposableZone` | 61 | 1 (1 file) | PhraseRules.lean:818 |
| `costSubject` | 62 | 1 (1 file) | AbilityRules.lean:647,650 |
| `paidSubject` | 63 | 1 (1 file) | PhraseRules.lean:408 |
| `stackActOn` | 64 | 1 (1 file) | AbilityRules.lean:239,243; Phrase.lean:1847,1852,1853 |
| `copySourceOk` | 65 | 2 (2 files) | AbilityRules.lean:238 |
| `movable` | 66 | 2 (2 files) | AbilityRules.lean:228; Phrase.lean:1861 |
| `destOk` | 67 | 3 (2 files) | AbilityRules.lean:228; Phrase.lean:1868,1869 |
| `arrangementOk` | 68 | 1 (1 file) | AbilityRules.lean:229; Phrase.lean:205,1872; PhraseRules.lean:474 |
| `placeable` | 69 | 1 (1 file) | AbilityRules.lean:230 |
| `ridersFit` | 70 | 5 (4 files) | AbilityRules.lean:231 |
| `discardOk` | 71 | 0 (0 files) | Phrase.lean:1880 |
| `moveDestination` | 72 | 1 (1 file) | AbilityRules.lean:325 |
| `attachHeadOk` | 74 | 4 (2 files) | PhraseRules.lean:312 |
| `attachFits` | 75 | 5 (2 files) | AbilityRules.lean:186; PhraseRules.lean:89,161,531 |
| `combatRelOk` | 76 | 3 (1 file) | PhraseRules.lean:139,142,644 |
| `damageRecipient` | 77 | 17 (6 files) | AbilityRules.lean:93,163,308,505; Phrase.lean:1817; PhraseRules.lean:583,663; Rules.lean:49 |
| `attackable` | 78 | 3 (3 files) | AbilityRules.lean:83; Phrase.lean:1824; PhraseRules.lean:578,579 |
| `statusHolder` | 79 | 1 (1 file) | AbilityRules.lean:166 |
| `statusMarkable` | 80 | 2 (2 files) | AbilityRules.lean:167,511; PhraseRules.lean:687; Triggers.lean:45 |
| `visibilityOk` | 137 | 1 (1 file) | AbilityRules.lean:713 |
| `attacker` | 184 | 1 (1 file) | PhraseRules.lean:637 |
| `modifyStat` | 193 | 2 (1 file) | AbilityRules.lean:634 |
| `ctrlOverride` | 206 | 1 (1 file) | AbilityRules.lean:516 |
| `controlExchangeZone` | 226 | 1 (1 file) | AbilityRules.lean:50,51 |
| `cardSwapZones` | 227 | 1 (1 file) | AbilityRules.lean:55 |
| `zoneSwap` | 228 | 2 (1 file) | AbilityRules.lean:60 |

Pin lines in the bucket: 174.

The emission-site column is a grep for `.<name>` under `lean/Semantics/Check/`
(Refusal.lean excluded), 2026-10-06. Some matches are a same-named predicate
or another type's constructor (`NounPhrase.discardOk` is a predicate, not an
emission); confirm each before removing it. Pin lines count matching lines
under `lean/Semantics/Proofs/`.

## The work

- Remove each constructor's emission sites and the constructor itself, or
  keep a constructor with a one-line reason where this ticket allows it.
- Re-spell or retire each pin that asserted a retired refusal. A pin whose
  subject still exists is re-spelled against what the checker now says (the
  same term, its new outcome); a pin is removed only when the refusal was its
  whole subject, and the landing record names it.
- Update the facts table and `lean/CONTRACTS.md`: each section this sweep
  touches says what the checker no longer checks.

## Proof

- No reference-resolution or term-shape pin changes outcome.
- `cargo xtask lean-check` over `plugins_v2/canon` and `plugins_v2/testing`
  stays all-pass, and `cargo xtask definition-check` stays all-pass (or
  their regrouped names, `cargo xtask lean check` and `cargo xtask lean
  definitions`, once `semantics-v2-chooser-and-taker-fields` lands).
- `lean/scripts/build` passes.

## Did any retired pin encode a real card defect?

Mandatory. The landing fills this in, one line per retired or re-spelled pin:
**yes**, naming the card and the defect the refusal caught, or **no**. A yes
is reported to the owner with the landing record; it may argue for keeping
the refusal, which needs a dated ruling.

| Pin | File | Real card defect? |
|---|---|---|
| (filled in by the landing) | | |

## Also in this sweep

- **The bearer exemption goes.** `Conferral.checkProperty`
  (`lean/Semantics/Check/Rules.lean:74-78`) exempts a two-stat
  `ptModification` on `this` from the battlefield demand; with the demand
  retired the exemption has nothing to exempt and is deleted (§7 ruling
  2026-10-06, "The bearer exemption stays narrow until the zone sweep").
- **The twin goes with it.** `p1p1CounterCountMultipliedTwin`
  (`lean/Semantics/Proofs/Rules.lean:178-191`, cited at
  `lean/CONTRACTS.md` in the counter-definition section) pins two
  `.zoneIs .battlefield` refusals; re-spell it against what the checker now
  says, and update the CONTRACTS paragraphs that cite it.
- **Zone state on bindings stays** (§7 ruling 2026-10-06, clause (c)):
  `that(Permanent)` against `that(Card)` resolves on it. This sweep removes
  refusals, not fold-state; a change that strips zone from a binding is a
  STOP.
- `zoneIs` alone is 54 pin lines in 15 files.
- `discardOk` has no emission site today (the match in `Check/Phrase.lean` is
  the predicate `NounPhrase.discardOk`); retire the constructor and decide
  whether the predicate has another user.
