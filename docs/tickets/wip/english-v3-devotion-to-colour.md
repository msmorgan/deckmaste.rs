---
needs: []
---
# Read *devotion* with its *to* Complement: You gain life equal to your devotion to green.

## Why

*your devotion* reads as a Noun Phrase; *your devotion to black* does not, so
almost no devotion card reads. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:is less` 21 sole-blocker
faces (the Theros Gods) is one corner of it. `cargo xtask english-v3 grep -i
'devotion to'` finds **61** faces, **58** unread; 58 failing lines match and
**57** are the only failing line of their face. By shape (sole-line faces):
*equal to your devotion to C* 19; *where X is your devotion to C* 16; *as
long as your devotion to C [and C] is less than five/seven* 21; *if your
devotion to C and C is seven or greater* 1.

`english-v3-data-batch-2` deferred this ("*Devotion to* needs a nominal
Complement marker beyond the currently declared None/Of"). The 21 God lines
also need a spelled cardinal after *less than*
(`english-v3-spelled-cardinal-scalar-measure`); this ticket alone is expected
to cover about 35 faces.

## Goal

*devotion* takes a *to* Complement naming one colour or a coordination of two
(*your devotion to black*, *your devotion to white and black*), and the
resulting Noun Phrase has Scalar Denotation wherever *your life total* does:
after *equal to*, as the value of *X is …*, and as the Subject of a scalar
comparison.

## Analysis

CGEL Ch. 5 §14.1(a), p. 440: "With PP complements the head noun determines
the choice of preposition or the range of permitted choices. The default
preposition is *of*", and nouns such as *injury* "permit either default *of*
or a more specific preposition (… *an injury to*/*of the wrist*)"; "*address*
takes *to*". That backs a noun licensing a *to* Complement. That *devotion*
does, and what the Complement may be, are project rulings.

Existing declarations: `lexeme:CommonNoun/Devotion` (`lexicon/core.ron`,
`countability: [Mass]`, no features, no frames); feature
`NominalComplementMarker { None, Of }` and table `nominal_postmodifier_licence`
(`crates/deckmaste_english_v3/src/declarations.rs`), which license a noun's
*of* Complement; `vocab:Preposition/To` with `PrepositionFunctionLicence`;
the colour words `vocab:ColorWord/Black` etc., which read today as Adjectives
and as Keyword Quality (*protection from black*) but not as a Noun Phrase
("black" as `noun-phrase` 0; "to black" as `preposition-phrase` 0).
`ScalarNominalOperand` and the *equal to* route read
`NounPhrase.ScalarDenotation`.

Orchestrator rulings 2026-10-10:

- DV1. Extend the existing noun-Complement licence: `NominalComplementMarker`
  gains `To`, *devotion* declares it (data), and *to* declares
  `NounComplement` in its Preposition Function Licence if it does not
  already. The existing Construction that reads a noun's *of* Complement
  reads the *to* Complement through the same table; no new Construction.
- DV2. The Complement of *to* here is a colour word. Route it through the
  declared class the colour words already carry (the one Keyword Quality
  reads); if a colour word must head a Noun Phrase for this, that is a
  declared nominal use on the colour-word class, not a new Lexeme per colour
  and not a test on the five words. Two colours coordinate with *and* by the
  ordinary coordination of that category.
- DV3. *devotion* declares Scalar Denotation as *life total* does (data).
  The licence for *equal to …*, *X is …* and *… is less than N* is that
  feature; no test on the noun.
- DV4. One analysis per string. The twins to avoid: (a) *to black* attaching
  as a clause Adjunct or as a Complement of *equal* / *is* instead of
  *devotion* (*equal to your devotion to green* has one *to* per head);
  (b) *black and green* coordinating NP-high (*[your devotion to black] and
  [green]*); (c) the colour word read as an Adjective with an elided noun.
- DV5. *your devotion to that color* (Nykthos, Shrine to Nyx; Nyx Lotus) and
  *Your devotion to each color and each combination of colors* (Altar of
  the Pantheon) are the 3 covered faces. They read today although *devotion*
  licenses no Complement, so their *to* PP attaches somewhere else. With DV1
  the Complement of *to* is whatever NP the Preposition takes, so these gain
  the Complement Reading. Inspect their current Readings on the claim
  parent: a Reading that attaches *to that color* to the verb or the Clause
  is a wrong analysis to retire if a declared feature separates it, and to
  report with its shape if not. No named guard.
- DV6. Newly covered faces are gains. Reading-count changes are expected on
  exactly those 3 faces, of the DV5 shape. Every other covered face,
  including every covered face with a colour word, keeps its count; an
  increase from colour words newly heading NPs elsewhere (*becomes black*,
  *protection from black*) is a DV2 defect. Report every change with its
  shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Nylea's Disciple: "When this creature enters, you gain life equal to your
  devotion to green." (control "… equal to your life total." 1)
- Tymaret, Chosen from Death: "Tymaret's toughness is equal to your devotion
  to black." (control "… equal to your life total." 1)
- Gray Merchant of Asphodel: "When this creature enters, each opponent loses
  X life, where X is your devotion to black. You gain life equal to the life
  lost this way."
- Master of Waves: "When this creature enters, create a number of 1/0 blue
  Elemental creature tokens equal to your devotion to blue."

Also 0 today: "your devotion to black" and "your devotion to black and green"
(`noun-phrase`); "Your devotion to black is 5." (sentence). Controls that
must not change: "your devotion" (`noun-phrase`) 1; "Its power is 5." 1;
"Target creature gets +X/+0 until end of turn, where X is your life total."
1; "You gain life equal to your life total." 1; "Your life total is seven or
greater." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bdevotion\b|\b(black|blue|green|red|white)\b` (3,295
faces; the colour-word half guards DV2, which changes what a colour word can
be) plus 300 random covered controls (`--random 300 --seed 20261010`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; DV6 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` and `deckmaste_lexical_source`, once, at the end: a
rebuild after a grammar edit takes about 2–3 minutes and the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
roles (the *to* PP as Complement of *devotion*, the colour coordination
inside it, the NP's Scalar Denotation, the DV4 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *is less than five* / *less than seven* (21 sole-line faces): the spelled
  cardinal, `english-v3-spelled-cardinal-scalar-measure`. Those faces need
  both tickets.
- Any further gap on the 3 covered faces beyond DV5.
- The game meaning of devotion.

## Landing record

1. **Census delta** — Original claim-parent content `qxyyosvr` (captured at empty `mqyqzunv` before implementation): 3,135 covered faces and 127,430 Readings; implementation `mqyqzunv` before refresh: 3,169 and 127,711. Official diff: 34 gains, no lost faces, no decreases or increases on covered faces, three digest-only changes, 3,527 unchanged. Altar of the Pantheon (3 → 3), Nykthos, Shrine to Nyx (15 → 15), and Nyx Lotus (15 → 15) replace the nominal *to* Modifier with the selected Complement (DV5); existing wider nominal scopes remain. No unintended-only gain found. The bounded manifest contains all 3,295 supported faces matching `(?i)\bdevotion\b|\b(black|blue|green|red|white)\b` plus 300 random covered controls, seed 20261008 (31 overlaps; 3,564 unique faces). Evidence: `target/english-v3/english-v3-devotion-to-colour/{manifest,before,after,diff}.json`; census flags `--workers 6 --samples-per-face 0`. After successful refresh, the same manifest at `pnulonno` (refreshed `mqyqzunv` plus this record correction) reports 3,193 covered faces and 128,190 Readings: review-time diff 58 gains, 0 lost/decreases/increases, three digest-only changes, 3,503 unchanged. The official pre/post-refresh diff attributes the extra 24 gains and 479 Readings to incoming `roomopmr` (phase-in/out), `nzmrrqky` (cause infinitival), and `tlpwnxqn` (named-token apposition); it has no losses, decreases, increases, or digest changes. Original implementation delta is preserved in `pre-refresh-diff.json`; incoming delta in `refresh-diff.json`. Integration-refresh census at `mqyqzunv`: 3,194 covered faces and 128,622 Readings; kind-refresh official diff 59 gains, 0 lost/decreases/increases, three digest-only changes, 3,502 unchanged. Relative to the reviewed census, `wkooyuko` adds Ferrafor, Young Yew (432 Readings); the 34 devotion faces retain their counts and roles with the appended form index, producing 34 digest-only differences. Evidence: `integration-refresh-diff.json`; total gains split 34 devotion / 25 refreshed sibling gains. The integration retry refreshed `mpvousxz` (supplementary temporal adjunct). At `qnzwxwop`, the same manifest reports 3,196 covered faces and 128,627 Readings: final official diff 61 gains, 0 lost/decreases/increases, three digest-only changes, 3,500 unchanged. `temporal-refresh-diff.json` isolates two incoming gains: Norwood Priestess (3 Readings) and Xiahou Dun, the One-Eyed (2), with no other changes. Final gains split 34 devotion / 27 refreshed sibling gains. A further retry refresh brought `rtzolrtx` (preposed duration). At `vnokzpul`, the same bounded census remains 3,196 covered faces / 128,627 Readings, with the same final 61-gain diff; `duration-refresh-diff.json` reports all 3,564 faces unchanged, including counts and digests. The next retry brought `mzkvyunr` (noncombat noun data): at `lsxwmwmx`, the final census has 3,198 covered faces and 128,763 Readings, with 63 gains, 0 lost/decreases/increases, three digest-only changes, and 3,498 unchanged. `noncombat-refresh-diff.json` isolates Blessed Sanctuary (19 Readings) and Ojer Axonil, Deepest Might (117), with no other changes. Final split: 34 devotion / 29 incoming sibling gains. The modal-header retry brought `ztrnpkyv`: at `smxvkoqy`, coverage and Readings remain 3,198 / 128,763. The latest final diff is 63 gains, 0 lost/decreases/increases, 71 digest-only changes, and 3,430 unchanged. `modal-refresh-diff.json` attributes 68 digest-only changes to modal headers becoming Clauses whose selected count Complement is a Quantitative Determiner; counts are preserved. The other three digest changes remain the DV5 Complement replacements. The polyhedral retry brought `vlxrulqw`: at `mqkqrrts`, the latest census has 3,200 covered faces and 131,523 Readings. Final diff: 65 gains (34 devotion / 31 incoming), 0 lost/decreases/increases, 71 digest-only, 3,428 unchanged. `polyhedral-refresh-diff.json` isolates Valiant Endeavor (168 Readings) and Wild Endeavor (2,592), with no other changes. The progressive-relative retry brought `opxvnokr`: at `ttwuqrkr`, the final census has 3,201 covered faces and 131,529 Readings. Final diff: 66 gains (34 devotion / 32 incoming), 0 lost/decreases/increases, 71 digest-only, 3,427 unchanged. `progressive-refresh-diff.json` isolates Undercellar Sweep (6 Readings), with no other changes. The while retry brought `wxwozrtz`: at `kklxwmwk`, the final census has 3,204 covered faces and 131,542 Readings. Final diff: 69 gains (34 devotion / 35 incoming), 0 lost/decreases/increases, 71 digest-only, 3,424 unchanged. `while-refresh-diff.json` isolates Autarch Mammoth (2 Readings), Bridled Bighorn (3), and Seraphic Steed (8), with no other changes. The focused-adjunct retry brought `qktrquum`: at `wkkmwkkp`, the final census has 3,209 covered faces and 131,561 Readings. Final diff: 74 gains (34 devotion / 40 incoming), 0 lost/increases, 2 decreases, 71 digest-only, 3,417 unchanged. `focused-refresh-diff.json` isolates five incoming gains: Chronatog Totem (3 Readings), Heron-Blessed Geist (13), Nature’s Chosen (4), Security Detail (1), and Surge Engine (6). Both decreases are attributed to `qktrquum` and its accepted CF7/W9-A retirement of outer `only [X and only Y]` focus: Defenders of Humanity 21→14 (line 1: 14→7), Essence Anchor 3→2 (line 1: 2→1). Eight removed Readings belong to that trunk retirement; no devotion decrease.
2. **Witnesses** — All seven re-probed witnesses fail (0) on `qxyyosvr`. On `mqyqzunv`: Nylea's Disciple line 1; Tymaret, Chosen from Death line 1; Gray Merchant of Asphodel paragraph 4; Master of Waves line 6; *your devotion to black* 1; *your devotion to black and green* 1; *Your devotion to black is 5.* 1. Each has the selected nominal Complement; the colour coordination stays inside it. Merchant/Waves retain independent wider clause/modifier scopes (their life-total replacements also read 4/6). All seven controls remain 1 → 1, including the Nylea/Tymaret life-total replacements. The formerly covered *that color* and *each color and each combination of colors* fragments retain 1/3 Readings while selecting the Complement; the latter retains outer *of colors* and NP coordination scopes.
3. **Spot checks** — At `mqyqzunv`, ten newly covered faces select *devotion*'s *to* Complement with its Adjective Phrase colour: Nylea's Disciple (green), Tymaret, Chosen from Death (black), Gray Merchant of Asphodel (black), Master of Waves (blue), Karametra's Acolyte (green), Aspect of Hydra (green), Mogis's Marauder (black), Sanguimancy (black), Setessan Petitioner (green), Thassa's Rebuff (blue). Full-face probe evidence is in `spot-*.json` beside the census.
4. **Deviations and STOPs** — No Construction added or deleted. Two frame aliases describe selected *to* + Noun Phrase / Adjective Phrase, because the existing alias hardcodes *of* + Noun Phrase. The existing Adjective Complement Class is exported through ordinary coordination and declared for the eight existing Color Word members in lexicon data, without loader changes or a general colour Noun Phrase route. The superseded nominal *to* Modifier route is retired through marker features; other existing scopes on the three DV5 faces are preserved and disclosed above. Orchestrator review 2026-10-10 ACCEPTED: `AdjectiveComplementClass: ColorWord` is declared on the whole Color Word class, so *devotion to colorless / monocolored / multicolored* also read. These are unattested and accepted as a class-level licence; no check added. Integration refresh brought `wkooyuko` (kind-of-counter) and a conflict in the shared marker/table and Construction forms. Both routes were preserved: the incoming taxonomic form retains index 1, the colour Complement is appended at index 2, and the marker-selection table gains `(To, To)`. The new independent devotion Reading test is updated for this form index; its role/value/roundtrip assertions are unchanged. The first integration was refused because default moved; the required refresh brought `mpvousxz`, merged cleanly, and triggered the complete bounded/test/clippy re-check recorded above. The noncombat refresh also conflicted at the suite_6 module-registration tail; both registrations were retained exactly once and the conflict cleared. The modal-header registration conflict was likewise resolved by retaining both suite_6 modules exactly once. The polyhedral registration conflict was resolved the same way, preserving both modules exactly once. The progressive-relative registration conflict was resolved by retaining both suite_6 modules exactly once. The focused-adjunct refresh conflicted at adjacent category declarations; resolution retains both the Adjective Phrase series Complement Class and the incoming Preposition Phrase series Nominal Adjunct Host feature. No glossary gap or ruling STOP. Kata refresh concurrency guards returned 69 on sibling edits; retries preserved the closed feature and ultimately completed with exit 0. Seed 20261008 follows the binding verification instruction rather than the Method's draft seed. Six role/roundtrip/control tests implement DV1–DV6; no other test change.
5. **Tests** — At `mqyqzunv`, `cargo test -p deckmaste_english_v3`: PASS (902 tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`: PASS; `cargo test -p deckmaste_lexical_source`: PASS (84 tests: 40 unit + 44 integration); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: PASS. Restored 0 / re-spelled 0 / ignored 0 / added 6 / removed 0. No removed tests. After refresh at `pnulonno`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (915 English tests; 84 lexical-source tests). At integration-refresh `mqyqzunv`, `cargo build -p xtask` and the same four full test/clippy commands PASS once more (920 English tests; 84 lexical-source tests), after preserving both selected Complement routes. After the integration retry refresh at `qnzwxwop`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (924 English tests; 84 lexical-source tests). After the duration retry refresh at `vnokzpul`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (928 English tests; 84 lexical-source tests). After the noncombat retry refresh at `lsxwmwmx`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (935 English tests; 84 lexical-source tests). After the modal retry refresh at `smxvkoqy`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (938 English tests; 84 lexical-source tests). After the polyhedral retry refresh at `mqkqrrts`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (941 English tests; 84 lexical-source tests). After the progressive-relative retry refresh at `ttwuqrkr`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (945 English tests; 84 lexical-source tests). After the while retry refresh at `kklxwmwk`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (948 English tests; 86 lexical-source tests: 42 unit + 44 integration). After the focused-adjunct retry refresh at `wkkmwkkp`, `cargo build -p xtask` and the same four full test/clippy commands PASS again (957 English tests; 86 lexical-source tests). No Lean or xtask code changed.
6. **Timing** — Bounded AFTER census at `mqyqzunv`: **235890 milliseconds**, six workers, measured with `/usr/bin/time -v`; the binding budget replaces the full-corpus run.
