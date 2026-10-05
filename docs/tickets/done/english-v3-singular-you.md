---
needs: []
---
# Remove plural second-person readings from Oracle English

User ruling: Oracle you denotes one player. Retain singular nominative and
accusative you, possessive your/yours and reflexive yourself. Remove their
plural alternatives and yourselves. Preserve ordinary second-person verb
agreement and real plural coordinated subjects. Standard constraints apply.
Record all coverage and Reading-count changes; use at least eight corpus workers.

## Landing record

### PROVE

The user explicitly retires second-person plural pronoun analyses from Oracle
English. Singular subject/object you, genitive your/yours and yourself retain
their complete lexical values and casing. Third-person plural pronouns and
plural concord for genuinely coordinated subjects remain available.
Independent fixtures verify exact value/tree sets and both roundtrip laws;
plural independent values and yourselves are rejected. Production changes
are declarations only; no word/card/construction-named guard is added.

### DISCLOSE

Retired analyses: plural vocab:SubjectPronoun/You,
vocab:ObjectPronoun/You, vocab:PossessiveDeterminerPronoun/Your and
vocab:PossessiveAbsolutePronoun/Yours. Removed lexical owner:
vocab:ReflexivePronoun/Yourselves. Singular Yourself is retained. The four
plural feature bundles and the Yourselves source declaration/form replacement
are removed directly; no filtering hides an otherwise admitted Reading.
The requested omission comment is beside the SubjectPronoun declaration.

Positive witnesses now use real Oracle passages or attested constituents with
card/context comments: Greta and Tinybones for you draw a card, Apocalypse for
You discard your hand, Bloodroot Apothecary for you and target opponent,
Next of Kin for possessive PPs, and Deep Sight, Reach Through Mists and
Revitalize for enumeration and source-validation checks. Bare pronoun lexical
values remain independent morphology witnesses. Invented draw-your-hand and
face-down-player positives are retired at the user's explicit instruction;
no production grammar filter is added. The lexical ADR records that grammatical
nonsense may be admitted but must not become invented positive regression
contracts; actual fragments need their actual context.

No construction is added or removed; cost ranking and genuine structural
ambiguities remain untouched. No test function is deleted or ignored.

### REPORT

Complete before/after census and gate evidence follow below.

Measured change `zkzlwqtpoultppwvtyslqzrtmrvsukyn`; covered 4,995 (v3 emits no English
coverage-lock field). Source SHA-256 `49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
Before lexical inventory `0477f6932c32870c3fe216bbba74039a49756825cc4f257abda1978caaf737d7`; after
`dfab2d351583dcd38e3eb29bd2c6fa8b13bc5befec7db3ea8a8bed8bc549777f`.

All 32,828 identities reconcile: 4,995 covered before and after, zero gains
or losses. Retained Readings 37,458 → 13,492 (23,966 retired). Unique/multiple
census 3,127/1,868 → 3,687/1,308; 560 formerly ambiguous faces become unique.
No specificity selection erases alternatives. Zero issues, internal failures,
duplicates, cyclic derivations or limited enumerations. Every retained Reading
passes admission, exact realization and lexical/construction traversal.
All 32,828 Type Lines remain uniquely covered with zero issues.

Exactly 1,366 covered faces decrease. Every face satisfies
`before = after × 2^n`, where n counts the retired-family pronoun occurrences
in its baseline selected tree: 942 faces have n=1, 328 n=2, 72 n=3, 18 n=4,
5 n=5 and 1 n=6. The other 3,629 covered faces have unchanged counts.
The census stores one selected tree per face: this reconciles all count
reductions and selected values, not full before/after fingerprint-set inclusion.
No unexpected decrease or removed plural second-person value appears in
selected after trees. Yourselves occurs on no supported source face.

Lexical inventory 34,837 owners;
43,116 independent lexical values all pass.
Relative to the preceding inventory, one owner and ten values are retired
(eight plural You/Your/Yours capitalization values and two Yourselves values).
Raw unknown words remain 9,171 occurrences / 1,279 spellings.
Construction count unchanged at 459. New homographs: none. New form literals
and form-literal/vocabulary overlaps: none; existing surviving inventories
are unchanged. V3 emits no legacy permitted-licensing-checker total; lexical
source loading succeeds and no new named guard is introduced.

Subset evidence: Balefire Liege 3,456 → 108; Ashenmoor Liege 180 → 45.
Knight of the Skyward Eye remains uncovered. Skyfisher Spider is also unchanged
at zero complete Readings with no unknown words; its separate construction
gaps are not a pronoun-removal regression. Read-only constituent probes
identify two independent blockers: creature card lacks noun/type
premodification, and nonland permanent lacks negative type-modifier composition.
For each creature and When you do already parse; no Spider-specific code
or new modifier construction is added in this feature.

Performance advisory: complete before 47.206s,
after 36.680s; quiet-host reference 16.26s. After
613,227 ns/B; 24 workers, host load
[13.0966796875, 8.396484375, 5.5703125]. Gate and lexical/type censuses overlapped the after run;
this is not a controlled quiet-host speed comparison.

Tests added: 6; existing tests re-spelled: 9 (both possessive tests and seven
xtask census/validation tests); restored, removed or newly ignored: 0.
Focused lexical-source tests pass 3/3; final authentic English tests pass 3/3
new and 2/2 existing possessive tests. All eight xtask English-v3 tests pass
with the final authentic witnesses. One inherited enumeration expectation
relied on the retired plural You analysis; the final capped-enumeration test
uses Deep Sight's actual two-Reading Oracle text plus two unique real texts,
preserving limited/complete/unknown-total assertions. Reminder/Unicode fault
injections remain diagnostic probes around valid Oracle text, not positive
nonsense grammar examples.

Required gate: `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The source/construction/English package suites pass. The affected xtask package
rerun passes its complete suite, including existing whole-corpus and Lean
integration checks. Subsequent edits change only test witnesses and documentation;
all affected final English/xtask test files are rerun and pass. Production corpus
results are unchanged and reused. Initial draft orientation fixtures and the
intermediate invented PP-attachment example were rejected by the user and
replaced by authentic examples before landing.

Formatting and citation checks pass; zero noncompliant strings and stale
citations. No CR citation is added or changed.
