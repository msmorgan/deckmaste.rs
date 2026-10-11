---
needs: []
---
# Read *your choice of* + keyword alternatives: Target creature gains your choice of deathtouch or lifelink until end of turn.

## Why

*your choice of* reads before NPs (NP "your choice of a creature or a land"
2; "Put your choice of a +1/+1 counter or a flying counter on target
creature." 2) and keyword alternatives read as a granted ability ("Target
creature gains deathtouch or lifelink until end of turn." 1), but the two
together do not: NP "your choice of deathtouch or lifelink" 0, NP "your
choice of flying" 0, "Target creature gains your choice of deathtouch or
lifelink until end of turn." 0, "… gains your choice of flying, vigilance, or
lifelink until end of turn." 0. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:lifelink until` 9 (one
keyword of many; the rest are spread over `at:<keyword> until` keys).
`cargo xtask english-v3 grep -i '\bgains? your choice of\b'` finds **26**
faces, all unread; **25** have that line as their only failing line (plus
*loses your choice of*, Walking Sponge, 1 sole). All 27 continue with two to
four keyword names joined by *or*.

## Goal

After *gain* / *lose*, *your choice of* K1(, K2,) *or* Kn reads, the keyword
names being the alternatives chosen among: "Target creature gains your choice
of deathtouch or lifelink until end of turn.", with byte-exact realisation.

## Analysis

No CGEL claim: *choice of* + alternatives is an ordinary noun with an *of*
Complement, and that the alternatives may be keyword names is Oracle usage.
Project rulings throughout.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`KeywordGrantedAbility` (`[keyword: KeywordPhrase]`) feeding `core-verb:Gain`'s
frame `Predicate([Role("GrantedAbility")])` beside its
`Predicate([ObjectNounPhrase])` (`lexicon/verbs.ron`);
`KeywordComplementPreposition` (`[head: lexical(Preposition), " ",
complement: KeywordPhrase]`, `require head.KeywordComplement = Yes`), which
is how *with flying* reads: `vocab:Preposition/With` declares
`KeywordComplement: Yes`, `vocab:Preposition/Of` does not
(`lexicon/core.ron`); the noun *choice* with its *of* Complement, which
reads *your choice of a Clue token, a Food token, or a Treasure token* today.

Orchestrator rulings 2026-10-10:

- CK1. The phrase is an NP Object of *gain* / *lose*: [*your choice* [*of*
  keyword alternatives]]. It enters through the Object frame, not through
  the `GrantedAbility` frame, and no new Construction is added: the *of* PP
  takes its keyword Complement through `KeywordComplementPreposition`.
- CK2. The licence is declared. Preferred: the head noun selects the keyword
  Complement class for its *of* frame (data on *choice*'s nominal frame),
  so *of* + keyword reads only under a noun that declares it. Declaring
  `KeywordComplement: Yes` on *of* itself is the fallback and must be
  measured on every face containing *of* + a keyword name before it is
  accepted (17,681 *of* tokens; an *of flying* PP must not start modifying
  arbitrary nouns).
- CK3. The alternatives are a keyword coordination with *or*, including the
  serial-comma series (*flying, vigilance, or lifelink*). If
  `KeywordComplementPreposition`'s Complement is a single `KeywordPhrase`,
  widen it to the keyword series category the granted-ability route already
  coordinates; do not write a second coordination.
- CK4. One analysis per string. *gains deathtouch or lifelink until end of
  turn* keeps exactly its Reading. In the witness, *until end of turn*
  attaches to *gains* once; the twin to avoid is a Reading in which only the
  last keyword is inside the *of* PP (*your choice of deathtouch* coordinated
  with *lifelink*).
- CK5. Parameterised keywords in the list (*protection from red*: Jodah's
  Avenger; *banding*: Urza's Avenger) read if their keyword phrase reads
  alone; no special case.
- CK6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: the 18 covered *your choice of* faces (57 in all;
  counter, token and source alternatives) keep their counts. Report any increase with its shape.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Orcish Medicine: "Target creature gains your choice of lifelink or
  indestructible until end of turn."
- Alchemist's Gift: "Target creature gets +1/+1 and gains your choice of
  deathtouch or lifelink until end of turn."
- Shifting Ceratops: "{G}: This creature gains your choice of reach, trample,
  or haste until end of turn."
- Golem Artisan: "{2}: Target artifact creature gains your choice of flying,
  trample, or haste until end of turn."
- Walking Sponge: "{T}: Target creature loses your choice of flying, first
  strike, or trample until end of turn."

Controls that must not change: "Target creature gains deathtouch or lifelink
until end of turn." 1; Flycatcher Giraffid "This creature enters with your
choice of a reach counter or a vigilance counter on it." 5 (census); The
Third Doctor "When The Third Doctor enters, create your choice of a Clue
token, a Food token, or a Treasure token." 2 (census); Asha's Favor
"Enchanted creature has flying, first strike, and vigilance." 1 (census); NP
"a spell with flying" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\byour choice of\b|\b(gains?|loses?|has|have) [a-z ,]+ or [a-z ]+ until\b

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent (size
the selector with `cargo xtask english-v3 grep --count` and record it). If
the CK2 fallback is taken, the selector also takes every face matching *of* +
a declared keyword surface. Zero lost; CK6 for increases. Full `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (*your
choice* as Object head, the keyword alternatives as Complement of *of*; the
CK4 negative), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *put / create / with your choice of* … (11 unread faces), e.g. *put your
  choice of a +1/+1, first strike, or trample counter on* …: shared-head
  counter-kind premodifier coordination under one article.
- *enters with your choice of two different counters on it from among* …
  (Grimdancer); *it becomes your choice of 5/1 or 1/5* (Aquamorph Entity);
  *put your choice of a counter from among* … (Aragorn, Company Leader).
- Ballroom Brawlers (*both gain*), Hunter's Axe (inside a granted quote) if
  those hosts block.

## Landing record

1. **Census delta** (claim parent `rolzltpt`, measured at empty `zspvqono`; implementation `zspvqono`): bounded selector `(?i)\bchoice\b|\b(gains?|loses?|has|have) [a-z ,]+ or [a-z ]+ until\b` selects 611 faces, plus 300 covered controls (seed 20261008), 904 distinct faces. Covered 810 → 835; Readings 31,468 → 31,549; 25 gains; lost/decreases/increases 0; 519 digest-only changes; 360 unchanged; source changes and internal issues 0. Digest-only changes preserve counts while re-spelling ordinary *choice* with its empty Nominal frame and promoting the keyword PP to a shared schema. Lost faces: none. No wrong-analysis gains found. Evidence: `target/english-v3/english-v3-your-choice-of-keywords/{before,after,diff}.json`; standard diff includes `--lines`. After refresh, `after-refresh.json`/`final-diff.json` reproduce every count above; `refresh-diff.json` reports all 904 faces unchanged, including digests. Original digest-only lines, counted exclusively by declaration class: *choice* lexeme/frame 510; `KeywordComplementPreposition` schema/category 20; `InfinitiveComplementNominal` → `SelectedComplementNominal` name/form requirement 8. Twenty of the Choice lines also contain a keyword PP node. The review-required wider fallback `(?i)\bchoices?\b|\b(with|without)\b|\bto [a-z]+\b` matched 13,907 supported faces / 19,685 matches; union with the same 300 covered controls gives **14,088 distinct faces**. Re-measured on refreshed claim parent `rolzltpt` (empty measurement child `xopwzvsw`) and implementation `zspvqono` (empty measurement child `zuvroszn`): covered 12,436 → 12,461; Readings 555,874 → 555,955; the same 25 gained faces; lost/decreases/increases/source changes/internal issues 0; 2,148 digest-only faces; 11,915 unchanged. The standard `wider-diff.json` with `--lines` has 2,223 digest-only changed lines: exclusive attribution Choice 510 / keyword PP 1,419 / infinitival nominal 294 (inclusive keyword PP count 1,439, overlapping 20 Choice lines). Every such line contains one of these changed declarations; no unclassified shape. Evidence: `wider-{manifest,before,after,diff}.json` and `line-classes.json` under the same target directory. Plural *choices* is covered: Malik, Grim Manipulator; Call to the Void; Expert-Level Safe; Prisoner's Dilemma remain unread on both sides. Wider `/usr/bin/time -v` wall times: before **19m47.37s** (8m56s build; 10m29.27s corpus), after **16m04.39s** (9m11s build; 6m39.98s corpus), both 6 workers / 0 samples per face. Integration refresh brought degree arithmetic `lvsykyns` and devour quality `vuyokrvx` / `ulkrnzyt`. Required wider re-census on empty `zulrxrqt` completed in **8m35.53s wall**, 6 workers / 0 samples, with no internal issues. The standard `review-refresh-diff.json --lines` comparison adds four inherited gains: Benevolent Hydra (6), Conclave Mentor (3), High Score (21), from `lvsykyns`; Feasting Hobbit (3), from `vuyokrvx`. Lost/decreases/increases 0; 271 inherited digest-only faces reflect `lvsykyns`'s optional degree-determiner Modifier. Covered 12,461 → 12,465; Readings 555,955 → 555,988. This ticket's own gain count remains 25; that refreshed tree has 29 gains against `wider-before.json`. A later mandatory refresh brought property equality `svlsnsmv` / review `vzuttnmp` and facts/protection/ripple work `mowpkrnr`, `ozoomvpm`, `mtwtyxxk` (landing `nqytplpr`). The second wider re-census, stamped at empty `svptumpw`, took **5m47.66s wall**, again 6 workers / 0 samples and zero internal issues. `review-refresh2-diff.json --lines` reports 17 inherited property-equality gains, no lost/decreased faces, no digest-only changes, and the inherited Woodcaller Automaton increase **285 → 293** from `svlsnsmv`: eight PE4/W9-A whole-property-coordination equality Readings, combining two independent base scopes, two comparison-Complement scopes and two conditional scopes. The documented pre-existing toughness-only attachments remain hygiene-3 A8's scope. No keyword-choice Reading is added on a previously covered face. The standard combined `integration-diff.json --lines` reports covered **12,436 → 12,482**, Readings **555,874 → 556,769**, **46 gains (25 this ticket + 21 inherited)**, lost/decreases 0, the one inherited Woodcaller increase, 2,392 digest-only faces and 11,649 unchanged. The third refresh brought `xswzoyrx` (predeterminer all), `xzwlutqp` (up to that many), and the plugin/macro changes through `ulklzlrx`. The same 14,088-face census at empty `wuuvkqkn` took **9m58.45s wall**, 6 workers / 0 samples, with zero internal issues. `review-refresh3-diff.json --lines` reports 24 inherited gains (15 up-to-that-many faces from `xzwlutqp`, nine predeterminer-all faces from `xswzoyrx`), no lost/decreased faces and no digest-only changes. Eight inherited PA7 increases from `xswzoyrx` exactly match its approved record: Ashiok, Nightmare Weaver 51 → 57; Author of Shadows 24 → 48; Kaya’s Guile 7 → 13; Mnemonic Betrayal 16 → 31; Multani, Maro-Sorcerer 8 → 15; Phyrexian Scriptures 5 → 6; Realm Seekers 20 → 34; Werewolf of Ancient Hunger 9 → 16. Their added shape is the outer Predeterminer [all [opponents’ graveyards]] / [all [players’ hands]], preserving the existing possessor-determiner route. No new keyword-choice ambiguity. The final standard `integration-diff.json --lines` reports covered **12,436 → 12,506**, Readings **555,874 → 557,632**, **70 gains (25 this ticket + 45 inherited)**, lost/decreases 0, nine inherited increased faces (Woodcaller plus the eight PA7 faces), 2,392 digest-only faces and 11,617 unchanged.
2. **Witnesses** (`zspvqono`; before → after probe counts / corresponding control without *your choice of*): Orcish Medicine 0 → 1 / 1; Alchemist's Gift 0 → 2 / 2; Shifting Ceratops 0 → 1 / 1; Golem Artisan 0 → 1 / 1; Walking Sponge 0 → 1 / 1. No witness dropped. Alchemist's Gift preserves the control's local-versus-coordinated-predicate duration scope under W9-A, with one route for the new material. Existing controls remain: direct keyword grant 1, Flycatcher Giraffid 5, The Third Doctor 2, Asha's Favor 1, *a spell with flying* 1, ordinary choice NP 2, counter-choice sentence 2. Keyword-choice NP probes now each have 1 Reading.
3. **Spot checks** (`zspvqono`): all ten have *your choice* as the Object of *gain*/*lose*, with the complete keyword Coordination inside the selected *of* Complement: Alchemist's Gift (deathtouch/lifelink); Argivian Avenger (flying/vigilance/deathtouch/haste); Assassin Initiate (flying/deathtouch/lifelink); Golem Artisan (flying/trample/haste); Jodah's Avenger (double strike/protection from red/vigilance/shadow); Manifold Mouse (double strike/trample); Orcish Medicine (lifelink/indestructible); Shifting Ceratops (reach/trample/haste); Urza's Avenger (banding/flying/first strike/trample); Walking Sponge (flying/first strike/trample). The keyword series uses the existing Coordination schemas. Alchemist's Gift and Urza's Avenger each have 2 probe Readings beside 2 in their control without *your choice of*, retaining independent matrix Coordination scope; no last-keyword-outside-*of* twin.
4. **Deviations and STOPs** (`zspvqono`): no new Construction or loader code. `KeywordComplementPreposition` becomes a schema with a noun-selected category instance, consumed only by a declared noun frame; *of*'s general keyword licence is unchanged (CK2 fallback unused). Added `NominalKeywordPreposition` and *choice*'s empty/selected Nominal frames. Renamed/refined `InfinitiveComplementNominal` to `SelectedComplementNominal`, retaining its infinitival form; existing value tests are re-spelled. Ordinary *choice* now uses `BareFramedNoun`; old unframed fixtures for Avacyn, Jeweled Spirit/Giver of Runes, and Sway of Illusion are re-spelled without weakening comparisons or changing outcomes. Initial full English runs exposed the Avacyn and keyword-quality fixtures; the remaining Sway fixtures were found by inspecting the same declared owner. Four construction-role tests added, registered once in `suite_6`. Multiform Wonder's new choice line reads once, but its independent *gets +2/-2 or -2/+2* line remains unread; Ballroom Brawlers retains the explicitly excluded *both gain* host. No STOP or glossary gap. Selector expanded from *your choice of* to every singular *choice* face to check ordinary uses. Initial review refresh brought scalar-difference noun work `ropwqyov` and starting-with adjunct work `wxxvsntt`; no conflict or corpus delta resulted, and no plugin definitions changed. Measurement-only review fix: widened the manifest without changing grammar, lexicon or tests. Unattested strings that read as class consequences: single keyword after *your choice of*; an *and* keyword series; *a/the choice of …*; and *your choice of flying or a card* as [your choice of flying] or [a card], an outer NP Coordination. These are class consequences, not corpus attestations. The integration refresh had one registration conflict in `suite_6.rs`; both degree-arithmetic and keyword-choice modules are retained exactly once, with no assertion or implementation edit for this review fix.
5. **Tests**: `cargo test -p deckmaste_english_v3` passed in full (1,029 tests) after the disclosed fixture repairs; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed, including a final re-check after those repairs. `cargo test -p deckmaste_lexical_source` passed (88 tests); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. After committing `zspvqono` and running `kata refresh`, `cargo build -p xtask` passed; the same four full test/clippy commands passed again on refreshed `zspvqono` (census stamp: empty `ukttuyvw`), with 1,041 English tests and 88 lexical-source tests. The 12 additional English tests belong to the incoming trunk changes. Citation compliance/check passed with 0 noncompliant/0 stale; diff audit found no newly cited sites. Assurance: restored 0 / re-spelled 13 / ignored 0 / added 4 / removed 0; no removal to justify. Re-spelled tests preserve full values, both roundtrip laws, the same cards, and the same outcomes. After the plugin/grammar integration refresh, `cargo build -p xtask` passed (5m37s); `cargo test -p deckmaste_english_v3` passed 1,047 tests; `cargo test -p deckmaste_lexical_source` passed 88; full `cargo test -p xtask` passed 425; `cargo test -p deckmaste_semantics_v2` passed 106; `cargo test -p deckmaste_construction_core` passed 474. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`, `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, `cargo clippy -p xtask --tests -- -D warnings`, `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings`, and `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` all passed. `cargo xtask lean check` passed 243/243 cards across canon/testing, required by the incoming plugin changes. `xtask` has one inherited ignore, `macro_schema_census_count_matches_21`, with its existing reason “cross-checks the live corpus against the census; run on demand”; this landing added no ignore. Evidence: `review-refresh-checks.json` and associated logs. The initial citation-compliance re-check flagged the two three-digit decimal corpus timings as rule-looking strings; expressed those timings in minutes/seconds, then rechecked successfully. The second plugin/grammar refresh repeated `cargo build -p xtask` (passed, 6m04s), the same five full test commands (English 1,052; lexical source 88; xtask 425; semantics-v2 106; construction-core 474), the same five full clippy commands (all passed), and `cargo xtask lean check` (244/244 cards passed). The same inherited xtask ignore remains; no new ignore. Citation compliance/check passed again with 0 noncompliant / 0 stale. Second-cycle evidence: `review-refresh2-checks.json` and associated logs.
6. **Timing** (`zspvqono`): the bounded after census took **45.14 s wall time with 6 workers**, measured with `/usr/bin/time -v`; `--samples-per-face 0`. The required refreshed bounded census took **28.92 s wall time with 6 workers**, also under `/usr/bin/time -v` with the same 904-face identity manifest and `--samples-per-face 0`. The review-required wider run has its before/after wall and corpus times in item 1. No full-corpus census ran. The standing plugin-refresh rule required the later full Lean check recorded in item 5.
