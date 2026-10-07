---
needs: []
---
**Retire the checker's other game-rule refusals (43 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `designationScope` | 33 | 4 (3 files) | AbilityRules.lean:203,208; PhraseRules.lean:319,522,524,688 |
| `designationChecked` | 34 | 1 (1 file) | AbilityRules.lean:205,208; PhraseRules.lean:522,525 |
| `facesFit` | 145 | 0 (0 files) | AbilityRules.lean:214 |
| `opponentsLibrary` [CR#701.29a] | 164 | 2 (2 files) | AbilityRules.lean:336; Words.lean:416 |
| `deedRides` | 165 | 0 (0 files) | AbilityRules.lean:197 |
| `enactAgentOk` | 169 | 3 (2 files) | AbilityRules.lean:334 |
| `nontarget` | 174 | 5 (4 files) | AbilityRules.lean:788; PhraseRules.lean:677,705,743; Rules.lean:33,91,94 |
| `headerNontarget` | 179 | 1 (1 file) | EventContext.lean:307; Triggers.lean:44 |
| `doorNamesHost` | 180 | 1 (1 file) | AbilityRules.lean:206 |
| `selfExchanged` | 183 | 4 (2 files) | AbilityRules.lean:64 |
| `grantSubject` | 195 | 5 (3 files) | AbilityRules.lean:655 |
| `grantable` | 196 | 3 (2 files) | Abilities.lean:617,619,623,670,690; AbilityRules.lean:604,655,737; Rules.lean:41,87 |
| `notCarvedOut` | 199 | 0 (0 files) | Abilities.lean:721,722; AbilityRules.lean:689 |
| `clauseStatic` | 201 | 2 (2 files) | AbilityRules.lean:286 |
| `becomesOk` | 202 | 21 (6 files) | AbilityRules.lean:602,686 |
| `damageOpUse` | 205 | 1 (1 file) | AbilityRules.lean:706 |
| `manaSymbolOk` [CR#107.4e,107.4f] | 209 | 3 (2 files) | PhraseRules.lean:39 |
| `twoParties` | 225 | 2 (1 file) | Abilities.lean:33; AbilityRules.lean:46 |
| `tokenTyped` | 229 | 2 (2 files) | AbilityRules.lean:583 |
| `tokenPtOk` | 230 | 1 (1 file) | AbilityRules.lean:583 |
| `subsFitLine` | 231 | 2 (2 files) | AbilityRules.lean:584 |
| `tokenAbilities` | 232 | 1 (1 file) | AbilityRules.lean:585 |
| `tokenCanonical` | 233 | 3 (2 files) | AbilityRules.lean:585,601 |
| `tokenQualsFit` | 234 | 0 (0 files) | AbilityRules.lean:586 |
| `emblemAbilities` | 245 | 3 (2 files) | AbilityRules.lean:293 |
| `tokenNamed` [CR#111.10] | 249 | 1 (1 file) | Rules.lean:55 |
| `definitionHolder` [CR#122.1] | 252 | 1 (1 file) | Rules.lean:111 |
| `definitionNamed` | 254 | 3 (1 file) | Rules.lean:112,114,116 |
| `definitionScoped` [CR#701.15b] | 257 | 1 (1 file) | Rules.lean:118 |
| `cardLine` | 259 | 5 (1 file) | Card.lean:189,279 |
| `cardText` | 260 | 11 (1 file) | Card.lean:180 |
| `modalFrame` | 261 | 2 (1 file) | Card.lean:180 |
| `chapterFrame` | 262 | 1 (1 file) | Card.lean:181 |
| `doorFrame` | 263 | 2 (2 files) | Card.lean:185 |
| `cardName` | 264 | 6 (2 files) | Card.lean:195; PhraseRules.lean:61,488,489; Words.lean:53 |
| `cardBox` | 265 | 20 (2 files) | Card.lean:196,210,254,269 |
| `cardCost` | 266 | 2 (1 file) | Card.lean:197,211,265 |
| `adventureInset` | 268 | 1 (1 file) | Card.lean:214,285; Facts.lean:367 |
| `flipHalf` | 269 | 1 (1 file) | Card.lean:288,289 |
| `levelRange` | 270 | 1 (1 file) | Card.lean:252 |
| `levelerFrame` | 271 | 1 (1 file) | Card.lean:253,292 |
| `bandsDisjoint` | 272 | 1 (1 file) | Card.lean:293 |
| `prototypeFrame` | 273 | 2 (1 file) | Card.lean:268,296 |

Pin lines in the bucket: 132.

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

## Decided at sweep time

- **The card-frame rules** (`cardLine` through `prototypeFrame`) are retired
  only at the owner's call, put to the owner when this sweep is worked: they
  are not instructions and do not move with the model (§7 ruling 2026-10-06,
  clause (d)).
- **The registry-definition refusals** (`definitionHolder`,
  `definitionNamed`, `definitionScoped`, `tokenNamed`) are shape checks on
  declarations rather than game rules, and probably stay. The sweeping agent
  decides each with a one-line reason.
- **Borderline constructors**, decided by the sweeping agent with a one-line
  reason each: `designationScope` and `designationChecked` (this bucket or
  zone), `notInstead` and `heldClause` (timing or this bucket, decided by
  `lean-checker-sweep-timing`), `manaSymbolOk`, `twoParties` and `nontarget`
  (this bucket or term shape). `selfDefinedOk`, `counterMemory`,
  `jointChoices`, `linkSource` and `readAmount` were borderline between
  reference and term shape; they are reference-side and stay.
