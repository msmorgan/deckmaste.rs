---
needs: []
---
# Read the independent genitive pronoun as a Noun Phrase: During turns other than yours, creatures you control get +0/+2.

## Why

*yours* is a declared Lexeme with no route: it reads in no position. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key
`at:than yours` 10 sole-blocker faces. `cargo xtask english-v3 grep -i
'\byours\b'` finds **10** faces, all unread, all *During turns other than
yours, …*, each the only failing line of its face. Probes: "turns other than
yours" (`noun-phrase`) 0; "turns other than your turn" 1; "yours" 0; "That
permanent is yours." 0.

Small, but it is one mechanism and the sole blocker of all ten.

## Goal

An independent genitive pronoun heads a Noun Phrase with no overt noun
(*yours* = *your turn(s)*) where an accusative Noun Phrase is admitted, here
as the Complement of *than* after *other*, with byte-exact realisation.

## Analysis

CGEL Ch. 5 §9, p. 410, [3]: "DEPENDENT *my your her our their no* /
INDEPENDENT *mine yours hers ours theirs none*"; "The dependent forms are
used in (pure) determiner function, the independent ones in head function.
The independent genitive pronouns occur in fused heads and also pure heads".
That backs *yours* as a head-function form making an NP by itself. Which
positions Oracle English uses is a project ruling (one is attested).

Existing declarations: `vocab:PossessiveAbsolutePronoun/Yours` and `/His`
(`crates/deckmaste_lexical_source/lexicon/vocabulary.rs`; forms in
`lexicon/core.ron` with `person: Second, number: Singular, case: Genitive`).
Routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`AccusativePronoun` (`NounPhrase`, `[head: lexical(Pronoun)]`) for *you* /
*it* / *them*; `FusedDeterminerHeadNounPhrase` (`[head:
lexical(Determinative)]`, `require head.FusedHeadUse = Yes`) for *this* and
the landed *the other* / *the rest* fused heads; the *other than* route
(*turns other than your turn* 1, *creatures other than this creature* 1).
Glossary: **Fused Determiner-Head**, **Fused Head Use**, **Genitive**.

Orchestrator rulings 2026-10-10:

- IG1. Refine an existing pronoun-headed NP Construction: `AccusativePronoun`
  gains the independent genitive as a second admitted pronoun class, or
  `FusedDeterminerHeadNounPhrase` admits it, whichever keeps one
  Construction and reads the declared `case: Genitive` plus a declared
  independent-form feature. No new Construction.
- IG2. The licence reads the declared features of the form (independent,
  genitive); no test on *yours*. The dependent genitives (*your*, *their*)
  must not qualify: they lack the independent-form feature. *his* is both
  (CGEL lists no distinct independent form for it); its independent Lexeme
  `PossessiveAbsolutePronoun/His` qualifies by the same feature and its
  dependent Lexeme does not.
- IG3. Number and person of the resulting NP: third person, number
  unspecified by the pronoun (the understood head decides, and nothing in
  the attested position agrees with it). Do not export the pronoun's second
  person: *yours* is not an addressee NP. If an agreement site forces a
  choice, STOP with the site.
- IG4. One analysis per string. The twins to avoid: (a) *his* before a noun
  (*his controller*, if any covered face has it) keeps the dependent Reading
  only; the independent form takes no following Nominal; (b) *turns other
  than your turn* keeps its 1 Reading; (c) *yours* is not also a predicative
  Adjective.
- IG5. Position: any position an accusative NP fills comes for free and is
  not blocked. Predicative *is yours* (0 faces) need not read and gets no
  dedicated work.
- IG6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *yours*; covered faces with
  *his* (count them: `(?i)\bhis\b`) must keep their counts under IG4(a).
  Report any increase with its shape before landing.

### Claim-parent STOP (2026-10-11)

Claim parent `xxuuuvww`; working change `wvxrpyol`. IG3 requires a STOP:
`NounPhrase` and `AccusativePhrase` both require `number`
(`declarations.rs`, Categories at lines 615–618). `AccusativePronoun` uses
`PredicateHeadAgreement`, and `CasePhrase` uses the same policy to forward
Number through the accusative wrapper (lines 1354–1357, 2128–2131,
4635). The compiler's built-in Number domain has only Singular and Plural
(`deckmaste_construction_v3_core/src/ir.rs`, `builtin_domains`), and
`Rule::complete` rejects an absent exported feature
(`src/runtime.rs.txt`, lines 412–414). Thus even the non-agreeing *than*
Complement cannot currently carry an unspecified Number. Omitting an export
fails the compiler's required-Category-feature check; exporting an absent
lexical Number fails admission. The STOP was resolved by IG7 below.

The owner ruling in this thread supersedes the ticket's exclusion of
unattested independent forms: provide independent genitive paradigm cells for
existing personal pronoun Lexemes, without minting absent pronouns. The thread
also fixes the control seed at 20261008, superseding the Method's 20261010.

Claim-parent probes (`cargo xtask english-v3 probe --text TEXT --category
CATEGORY --readings 100 --json`, `xxuuuvww`) all completed with no issues:
Mesa Lynx 0, Geyser Drake 0, Midnight Mangler 0, Glory of Warfare 0;
*turns other than yours*, *yours* and *That permanent is yours.* each 0.
No already-reading witness was dropped. Matching main-clause controls and
*During your turn* controls: Mesa 1/1, Geyser 4/4, Midnight 1/1.
Both Glory controls 1; *turns other than your turn*, *creatures other than
this creature* and *your devotion* each 1. The four Geyser control Readings
are pre-existing ambiguity to preserve under W9-A. Raw evidence and summary:
`target/english-v3/english-v3-independent-genitive-pronoun/before-probes.json`.
At this initial STOP, no census, tests or clippy had been run. IG7 below
resolves that STOP.

### IG7 resolution (orchestrator, 2026-10-11)

Do not add absent Number or twin Readings for the two Numbers. Reuse the
existing number-neutral NP convention if one admits both agreements in one
Reading; otherwise export unmarked Singular and append a hygiene-3 follow-up.
Person remains Third. Witness counts equal their controls under W9-A.

The existing convention is `DeterminerAgreementNumber = SingularOrPlural`,
used by `DeterminerHeadPartitiveNounPhrase` for an unrestricted head over a
plural Oblique. `finite_subject_number` accepts both predicate Numbers for
that same NP Reading. The independent genitive reuses it with unmarked
Singular and Third person; no new Number value or hygiene follow-up is needed.
CGEL Ch. 5 §9.1, p. 410 and §16.4, pp. 470–471 were page-checked: they
support independent forms as head-function forms of personal pronouns, not
this project's agreement encoding.

### Verification findings (2026-10-11)

The unchanged `restricted_nominals::authentic_full_genitive_possessor_preserves_internal_structure`
negative found newly admitted *its's untap step*. Independent genitive heads
now export Genitive Case Use, rather than Common, so the existing Common-case
Genitive possessor requirement rejects a further genitive marker. The existing
nominative/accusative wrapper tables admit that Genitive head in ordinary
Subject and Object positions. No existing case value or test was retired.

The first 19,141-face diff on `wvxrpyol` found 10 gains, no losses or decreases,
and one increase: Ashes of the Fallen 21 → 39 (its second line 20 → 38).
The new shape uses Have's existing Object + nominal predicative Complement
frame: *the chosen creature type in addition to its* as Object, *other types*
as a separate predicative Complement. IG5 admits the independent genitive
in that Object-internal accusative PP Complement; suppressing it there would
contradict IG5. A ruling on Have's nominal-predicative frame is pending.

That census predates the Genitive Case Use regression fix and is preliminary,
not final landing evidence. The exact Ashes second-sentence probe after the
fix still reports 38 Readings; *its's untap step* reports zero. CGEL Ch. 4
§5.4, p. 265 was page-checked: its complex-transitive table labels *have*
`adj`, excluding nominal predicative Complements. Repairing that existing
frame and expanding its affected-face selector needs the requested scope
ruling; no Have frame has been changed.

All following commands completed with exit 0 after the Case Use fix:

- `cargo test -p deckmaste_english_v3`
- `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
- `cargo test -p deckmaste_lexical_source`
- `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
- `cargo test -p deckmaste_construction_v3_core`
- `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`
- `cargo test -p deckmaste_lexical`
- `cargo clippy -p deckmaste_lexical --tests -- -D warnings`
- `cargo test -p deckmaste_lexical_model`
- `cargo clippy -p deckmaste_lexical_model --tests -- -D warnings`

English: 1,093 passed and one pre-existing ignored test; lexical source:
88 passed; construction core: 39 passed; lexical: 62 passed; lexical model:
no tests. Assurance: seven added, two re-spelled (the exact SubjectPronoun
paradigm values in `singular_you`), no removed tests and no new ignores.
The unchanged double-genitive negative was restored by the code fix.
Logs and exact command outcomes are in the evidence directory's
`verification.json`. The four witness counts are 1/4/1/1, matching controls.
The six-item landing record, final census, commit and refresh remain pending
until IG6 is satisfied; the workspace remains claimed and unintegrated.

### IG8 resolution (orchestrator, 2026-10-11)

IG8 supersedes the independent *its* cell: the explicit `It` paradigm now
lists only its nominative cell, declaring the independent-genitive gap by
omission, as existing missing paradigm cells are represented. No grammar
word guard or Have frame change is involved. CGEL Ch. 5 §16.4, p. 471 was
page-checked: “Examples are very occasionally found in iii and v, but we
have no attested instances of iv”. The passage admits rare independent
uses; excluding them here is the IG8 project ruling, not a claim that CGEL
prohibits every independent use. Existing *mine, yours, his, hers, theirs*
cells remain; *ours* awaits an existing *we* Lexeme under the owner principle.
IG7 retains the existing one-Reading number-neutral agreement encoding.
The Have frame observation is appended to hygiene-3 section A per IG8.
The final bounded rerun and all five full suites/clippy passes succeeded;
IG8 restores Ashes to its claim-parent count and resolves the pending STOP.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Mesa Lynx: "During turns other than yours, this creature gets +0/+2."
- Geyser Drake: "During turns other than yours, spells you cast cost {1}
  less to cast."
- Midnight Mangler: "During turns other than yours, this Vehicle is an
  artifact creature."
- Glory of Warfare: "During turns other than yours, creatures you control
  get +0/+2." (sentence; control "During turns other than your turn,
  creatures you control get +0/+2." 1; "During your turn, creatures you
  control get +0/+2." 1)

Controls that must not change (`noun-phrase`): "turns other than your turn"
1; "creatures other than this creature" 1; "your devotion" 1. The matching
main-clause controls for Mesa Lynx, Geyser Drake and Midnight Mangler were
not probed (the trunk moved under the prebuilt binary); probe them on the
claim parent.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(mine|yours|his|hers|its|ours|theirs)\b|\bother than\b`
(3,187 faces), union the `(?i)\bthis\b|\banywhere\b` surfaces of the existing
fused-head Construction (17,565 faces), plus 300 random covered controls
(`--random 300 --seed 20261008`): 19,141 distinct faces in
`target/english-v3/english-v3-independent-genitive-pronoun/manifest.json`.
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; IG6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 2–3 minutes and the six test binaries about 75 seconds; lexicon
data loads at runtime. Tests assert the role (the independent genitive as
head of the NP that is the Complement of *than*, the IG4 negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only. Glossary: add **Independent Genitive** (CGEL Ch. 5
§9, p. 410) in the landing; the Lexeme class name `PossessiveAbsolute…`
follows the glossary, not the reverse.

## Out of scope

- New personal Pronoun Lexemes: the owner ruling requires the existing
  paradigm cells, including unattested forms, but forbids minting absent
  pronouns. There is no existing *we* Lexeme, so this landing adds no *ours*.
- Angry Mob (in the 10): its first sentence must also read.
- Resolving what noun the independent genitive stands for.

## Landing record

1. **Census delta** (`wvxrpyolrzty`, claim-parent grammar → implementation):
   bounded 19,141 faces; covered 17,685 → 17,695, total per-line Readings
   508,251 → 508,281. Lost none; decreases 0; increases 0; digest-only 0
   (all classes and per-class line counts zero). Official `diff.json`,
   `--lines`: 10 gains and 18 newly recorded lines. BEFORE was taken on
   claim-parent `xxuuuvww` grammar/data with only ticket notes in `wvxrpyol`.
   Post-refresh (`wvxrpyolrzty`), same manifest: covered 17,715, Readings
   508,625; claim-parent comparison has 30 gains, 0 lost/decreased/increased/
   digest-only. Pre-refresh → post-refresh official comparison isolates
   20 trunk gains (344 Readings), 0 losses/count changes/digest-only.
   `oslovlpzzopl`: Elemental Expressionist, From the Catacombs, Geth, Thane
   of Contracts, Gruesome Encore, Isareth the Awakener, Llanowar Greenwidow,
   Moira and Teshar, Nephalia Academy, Realmbreaker, the Invasion Tree,
   Spirit-Sister's Call, Storm Herald, The Eighth Doctor, Whip of Erebos.
   `nklknwxyulsq`: Ghazbán Ogre, Scholarship Sponsor, Sokenzan Renegade,
   Tectonic Hellion, Thoughtbound Primoc, Wild Dogs, Wild Mammoth. Evidence:
   `after-refresh.json`, `diff-after-refresh.json`, `diff-refresh-incoming.json`.
   All digest-only classes and per-class line counts remain zero.
   Selector: every independent-form or *its* surface anywhere (3,187 faces,
   also *other than*), existing fused-head surfaces *this/anywhere* (17,565),
   plus 300 random covered faces, seed 20261008; union `manifest.json`.
2. **Witnesses** (`wvxrpyolrzty`): Mesa Lynx 1, Geyser Drake 4, Midnight
   Mangler 1, Glory of Warfare 1; each was 0 before and equals its control
   after. Geyser's four Readings preserve independent ambiguity under W9-A.
3. **Spot checks** (`wvxrpyolrzty`): Angry Mob 3, Geyser Drake 5, Glory of
   Warfare 2, Hurkyl's Final Meditation 2, Mesa Lynx 1, Midnight Mangler 2,
   Naiad of Hidden Coves 4, Oak Street Innkeeper 2, Vibrating Sphere 2,
   Warden of the Wall 7 (census counts sum per-line Readings). All ten have
   the independent *yours* fused Head inside the accusative NP Complement
   of *than* in *turns other than yours*, in the initial *During* adjunct;
   complete-document probes confirm every line reads, including Angry Mob's
   first sentence. No unintended-analysis-only gains.
4. **Deviations and STOPs**: IG7 resolved the mandatory-Number STOP by
   reusing `SingularOrPlural` with unmarked Singular and Third person, one
   NP Reading; no hygiene B follow-up. IG8 resolved the Ashes increase STOP:
   the independent *its* cell is a declared paradigm gap, with CGEL's rare
   uses distinguished from the project exclusion. No Have frame change;
   hygiene-3 A9 records its separate licensing question. Owner paradigm:
   *mine/yours/his/hers/theirs* on existing Lexemes; no absent *we* Lexeme
   minted for *ours*. Added `IndependentGenitive` Word Form and Genitive
   Case Use, with explicit spellings and morphology validation; Genitive
   Case Use fixes the unchanged *its's* negative. Refined the existing
   FusedDeterminerHeadNounPhrase; no Construction added/deleted. Retired
   PossessiveAbsolutePronoun inventory and its loader handling and form
   replacements; the replacement is the existing pronoun paradigm.
   Refresh brought trunk `oslovlpzzopl` (anywhere-else) and `nklknwxyulsq`
   (superlative quantifiers), including the construction-core field-matched
   form invariant. Resolved the fused-head declaration and suite_6 conflicts
   by preserving both routes and both module registrations. Independent
   genitives retain the optional dependent field with presence required No;
   the new exact-value helper was re-spelled with `dependent: None`.
   Glossary gap filled: Independent Genitive (page-checked CGEL pp. 410,
   470–471), Avoid Possessive Absolute Pronoun.
5. **Tests** (`wvxrpyolrzty`): full `cargo test -p CRATE` and
   `cargo clippy -p CRATE --tests -- -D warnings` for each of
   `deckmaste_english_v3`, `deckmaste_lexical_source`,
   `deckmaste_construction_v3_core`, `deckmaste_lexical`,
   `deckmaste_lexical_model`; IG8 rerun: all ten commands exit 0; English 1,093 passed (one existing
   ignored), lexical source 88, construction core 39, lexical 62, lexical
   model no tests. Exact outcomes recorded in
   `target/english-v3/english-v3-independent-genitive-pronoun/verification.json`.
   Seven added, two re-spelled, one existing negative restored, zero removed,
   zero newly ignored. Re-spelled
   `second_person_pronouns_have_exact_singular_case_and_capitalization_values`:
   *yours* is You's IndependentGenitive cell and owner values include both
   nominative and genitive. Re-spelled
   `unrelated_third_person_pronouns_retain_their_existing_values`: *his* is
   He's independent cell alongside its nominative; They also includes its
   independent cell, all exact values retained. Existing ignored
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
   retains blocker MS4, `english-v3-mana-symbol-verbless-subject`.
   Post-refresh (`wvxrpyolrzty`): `cargo build -p xtask` exit 0, and the
   same ten full-suite/clippy commands exit 0 again; English 1,102 passed
   with the same pre-existing ignore. The unchanged double-genitive negative
   and all five new English tests pass. No remaining conflicts.
   Citation checks repeated: 0 noncompliant, 0 stale, diff audit 0 new sites.
6. **Timing** (`wvxrpyolrzty`): final post-refresh bounded census 5:29.05
   wall, six workers, `/usr/bin/time -v`; no full-corpus census was run.
