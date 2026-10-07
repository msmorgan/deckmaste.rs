---
needs: []
---
**Retire the checker's cost and deontic refusals (15 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `modesCosted` [CR#702.172a] | 144 | 1 (1 file) | AbilityRules.lean:385 |
| `keywordCost` | 159 | 1 (1 file) | PhraseRules.lean:715 |
| `keywordCostPaidByYou` | 161 | 2 (2 files) | AbilityRules.lean:769 |
| `deonticBoundOk` | 186 | 1 (1 file) | AbilityRules.lean:676 |
| `deonticPatientOk` | 187 | 8 (3 files) | AbilityRules.lean:677 |
| `asThoughOk` | 188 | 2 (1 file) | AbilityRules.lean:678 |
| `deonticRiderOk` | 189 | 5 (1 file) | AbilityRules.lean:680 |
| `altPayment` | 197 | 3 (2 files) | AbilityRules.lean:491,650 |
| `addedPayment` | 198 | 2 (1 file) | AbilityRules.lean:651 |
| `manaRun` | 207 | 2 (2 files) | AbilityRules.lean:75,460; Card.lean:266 |
| `costAction` | 211 | 13 (3 files) | AbilityRules.lean:463 |
| `payable` | 213 | 5 (5 files) | Abilities.lean:1003,1004,1005,1013; AbilityRules.lean:340 |
| `payAgrees` | 214 | 7 (2 files) | AbilityRules.lean:340 |
| `costTapOnce` | 215 | 2 (1 file) | AbilityRules.lean:774 |
| `costPaidByYou` | 216 | 6 (2 files) | AbilityRules.lean:774 |

Pin lines in the bucket: 60.

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

## Note

The refusal of sequenced costs (`costAction`, pin `badSequentialCost` in
`Proofs/Deontic.lean`) was recorded as staying on purpose in
`plugins-v2-keyword-body-defects` ("Sequenced costs stay refused", costs paid
in any order [CR#601.2h]). That decision predates the charter; this sweep
decides it again with a one-line reason.
