---
needs: []
---
**Retire the checker's timing and window refusals (10 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `windowOk` | 138 | 11 (4 files) | AbilityRules.lean:410,411; PhraseRules.lean:515,668; Triggers.lean:25,27 |
| `pointWindowOk` | 139 | 1 (1 file) | Triggers.lean:28 |
| `eventUnderway` | 178 | 2 (2 files) | Triggers.lean:36 |
| `durationOk` | 181 | 1 (1 file) | AbilityRules.lean:286,388 |
| `triggerCountOk` | 190 | 1 (1 file) | AbilityRules.lean:717 |
| `untriggeredLimit` | 203 | 1 (1 file) | AbilityRules.lean:701,775 |
| `interceptable` | 204 | 2 (2 files) | AbilityRules.lean:702; Events.lean:54 |
| `heldClause` | 217 | 0 (0 files) | AbilityRules.lean:394 |
| `notInstead` | 220 | 2 (2 files) | AbilityRules.lean:391 |
| `chapterDefaults` | 246 | 4 (2 files) | AbilityRules.lean:785 |

Pin lines in the bucket: 25.

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

## Borderline, decided here

`notInstead` and `heldClause` were classified as timing against other game
rules; the sweeping agent decides each with a one-line reason. `heldClause`
has no pins.
