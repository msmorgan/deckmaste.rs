---
needs: []
---
# Read the keyword line with a Name parameter: Partner with Rory Williams

## Why

Scoped out of `docs/tickets/done/english-v3-phrasal-keyword-surfaces.md`
(PK2: "*Partner with <name>* is a different thing"; Out of scope: "needs its
own ticket"). Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): spread over about forty one-face
keys `at:<last word of the name> <end>` (`at:kenrith <end>`, `at:frye <end>`,
`at:isengard <end>`), so no key shows it. `cargo xtask english-v3 grep
'(?m)^Partner with'` finds **52** faces, all unread; 52 failing lines match,
**46** the only failing line of their face. It is the largest single
end-of-line cluster in the report.

## Goal

*Partner with* + a card Name reads as one keyword phrase on a keyword line,
alone or in a keyword series, with byte-exact realisation. The Name is the
existing Name constituent, not a Quality.

## Analysis

No CGEL citation applies: keyword lines are Oracle layout. Project rulings
throughout.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`KeywordLine` over `KeywordPhrase`; `QualityKeyword` (`[head:
lexical(Keyword), " ", quality: KeywordQualityPreposition]`, marker agreed
through `KeywordMarker`, whose values already include `With`);
`keyword_quality_marker` (admits `For`, `From`, `Onto`); `SubjectKeyword`;
and for the Name, Category `Name(number)` built by `CatalogName` (`[head:
lexical(Catalog)]`, `IdentityUse = Name`). Plugin side
(`crates/deckmaste_construction_core/src/macro_def.rs`):
`FixedKeywordParameterGrammar` has `Subject` and `Quality { preposition,
nominal_number }`; `KeywordParameterClass` has no Name class.
`plugins_v2/builtin/macros/keyword_abilities/partner.ron` declares bare
*partner* (`params: []`); the dashed variants are separate grammar-only
definitions (`partnerSurvivors.ron`, PK1–PK2).

Orchestrator rulings 2026-10-11:

- PW1 (R-plugin). *Partner with* is its own plugin definition
  (`partnerWith.ron`, `params: [Name]` or the nearest declared parameter
  kind), grammar-only and bodiless under the owner's PK1 ruling, following
  the dashed-variant precedent. Bare *partner* keeps its definition and its
  Readings. Register the facts exemption for the bodiless definition.
- PW2. The parameter grammar is a new `FixedKeywordParameterGrammar` variant
  for a Name with a selected Preposition (mirroring `Quality { preposition }`)
  and a matching `KeywordParameterClass` value. The licence reads that class
  and the agreed `KeywordMarker`; the payload is Category `Name`. No test on
  *partner*, *with* or any card name.
- PW3 (R-budget). Prefer a second form on `QualityKeyword`'s family or a
  schema instance over a new Construction. If the compiler needs a distinct
  Construction for the Name payload, it is ONE (`NameKeyword`), declared up
  front in the landing record against the ceiling: it buys 46 sole faces (52
  touched).
- PW4 (R-one, R-wrong). One analysis per string. The twins to avoid: (a) the
  Name read as a `KeywordQuality` through `NounPhraseKeywordQuality` (a card
  name is not a Quality; do NOT reach green by adding `With` to
  `keyword_quality_marker` and declaring `Quality(preposition: With)` on
  partner: that data-only route is a wrong analysis and does not land); (b)
  bare *Partner* followed by a stray *with*-PP.
- PW5 (R-class). Every Name the catalog declares is admissible after the
  marker; there is no list of partner names. A name with a comma (*Toothy,
  Imaginary Friend*) is one Name Lexeme already (NounPhrase "Toothy,
  Imaginary Friend" 1), so the keyword-series comma needs no special case:
  confirm with "Partner with Toothy, Imaginary Friend" and with a line that
  continues (*Flying, partner with …* if attested: count it).
- PW6. Newly covered faces are gains. No Reading-count increase is expected on
  any covered face (no covered face contains the line); any increase is a
  defect.

- PW7 (orchestrator, 2026-10-11; replaces PW1's exemption sentence). The
  existing Facts row is the semantics authority and stays unchanged and
  unexempted. The definition takes its declared signature, `[Quality]`;
  its Name payload is expressed by the Name parameter grammar recipe (PW2).
  The existing carrier admits a printed name as
  `Predicate::Named { source: NameSource::Printed { name } }`, so no new
  semantic codec class is needed. If the signature cannot carry the Name,
  STOP for the owner and semantics-v2; full xtask, semantics-v2 and
  construction-core suites plus the Lean check must pass before the
  STOP BEFORE INTEGRATE report.

## Witnesses

Each is the whole failing line of a face whose only failing line it is
(reminder text is stripped before parsing). Probed 2026-10-11 with the trunk
binary (`--category document --as-card <face> --readings 8`): all 0.

- Amy Pond: "Partner with Rory Williams"
- Pir, Imaginative Rascal: "Partner with Toothy, Imaginary Friend"
- Blaring Recruiter: "Partner with Blaring Captain"
- Khorvath Brightflame: "Partner with Sylvia Brightspear"
- Rowan Kenrith: "Partner with Will Kenrith"

Controls that must not change: Document "Partner" 1; "Partner—Survivors" 1;
"Splice onto Arcane {1}{R}" 1; "Affinity for artifacts" 2; NounPhrase "a card
named Rory Williams" 1; NounPhrase "Will Kenrith" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bpartner\b` (129 faces, 60 unread) plus 300 random covered
controls (`--random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; PW6 for increases. Because `plugins_v2/` and `macro_def.rs` change: FULL
`cargo test -p xtask`, `-p deckmaste_semantics_v2`, `-p
deckmaste_construction_core`, `-p deckmaste_english_v3`, clippy `--tests -- -D
warnings` for each, and `cargo xtask lean check`, once, at the end (a grammar
rebuild takes 2–3 minutes, the six test binaries about 75 seconds; plugin
`grammar:` data loads at runtime). Tests assert the role (the Name is the
keyword's parameter; PW4's two negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- The six non-sole faces' other lines.
- Engine body and deck-construction semantics of partner with.
- *Doctor's companion*, *Choose a Background*, the dashed partner variants
  (landed).

## Landing record

1. **Census delta — `opkxokpk`; refreshed `ksntxunw`.** Pre-refresh
   claim-parent grammar (`rmxnmksp`, measured in empty `opkxokpk`) versus the implementation: 369 → 415 covered faces,
   9,703 → 11,093 total per-line Readings. Selector: every supported face
   matching `(?i)\bpartner\b` (129), plus 300 disjoint covered controls selected
   with seed 20261008: 429 identities in
   `target/english-v3/english-v3-partner-with-name-keyword/manifest.json`.
   Official `english-v3 diff --lines`: 46 gains; lost faces none; decreases,
   increases and digest-only changes all zero. No unintended-analysis gains.
   BEFORE was run on the claim parent's grammar, not copied from the older
   full baseline; that baseline supplied only the seeded controls. All controls
   remained covered on the claim parent. Final evidence: `before.json`,
   `after.json`, `diff.json` in the same directory. After refresh,
   `ksntxunw` remains 415 covered / 11,093 Readings. Official
   `english-v3 diff --lines`, saved as `refresh-diff.json`, reports all
   429 unchanged. Digest-only
   classes: none, zero changed lines. No inherited decreases to attribute.
2. **Witnesses — `opkxokpk`.** Re-probed before implementation: Amy Pond,
   Pir, Imaginative Rascal, Blaring Recruiter, Khorvath Brightflame and Rowan
   Kenrith each 0 → 1 Reading for the ticket's whole keyword line, through
   `NameKeyword` with its `CatalogName` parameter. None was already covered.
   Controls before/after: Document “Partner” 1/1; “Partner—Survivors” 1/1;
   “Splice onto Arcane {1}{R}” 1/1; “Affinity for artifacts” 2/2; NounPhrase
   “a card named Rory Williams” 1/1 and “Will Kenrith” 1/1. Comma-series
   attestation (`(?im)^[^\n]+, partner with`): zero faces; no invented series
   witness added.
3. **Spot checks — `opkxokpk`.** Ten newly covered faces, each with one
   byte-exact keyword-line Reading selecting `NameKeyword` and the following
   full `CatalogName`: Alisaie Leveilleur → Alphinaud Leveilleur;
   Alphinaud Leveilleur → Alisaie Leveilleur; Amy Pond → Rory Williams;
   Bebop, Skull & Crossbones → Rocksteady, Mutant Marauder;
   Blaring Captain → Blaring Recruiter; Blaring Recruiter → Blaring Captain;
   Blue, Loyal Raptor → Owen Grady, Raptor Trainer;
   Brallin, Skyshark Rider → Shabraz, the Skyshark;
   Cazur, Ruthless Stalker → Ukkima, Stalking Shadow;
   Chakram Retriever → Chakram Slinger. Each face's other-line ambiguities
   remain independent of this route.
4. **Deviations and STOPs — `opkxokpk`.** PW3 authorizes one new
   Construction, `NameKeyword`, against the ultimate 250-Construction ceiling:
   it buys the 46 sole faces among 52 touched keyword-line faces. Existing
   Construction forms, requirements and exports remain unchanged. The new
   Name recipe has zero claim-parent users; its class refinement needs no
   additional declaration-use selector. No grammatical route was retired;
   the bare keyword's lexical owner and grammar both remain. PW7 replaces
   PW1's exemption sentence and resolves the Facts STOP: the existing
   `PartnerWith` row declares `extra: &[Shape::Quality]`, hence signature
   `[Quality]`. Its carrier is `KeywordParam::Quality { predicate }`;
   `Predicate::Named` with `NameSource::Printed` carries the full card name.
   The definition now takes `[Quality]`, forwarding the argument normally,
   while the Name recipe refines its grammatical class to Name. No Facts row,
   argument schema or semantic codec class changes. Only the obsolete
   “spelled through Partner” no-stub exemption is removed, since this
   definition closes that gap; the existing row is not exempted. This is
   bookkeeping, not a change to the row's semantic authority. The definition
   remains bodiless; the engine body belongs to the semantics-v2 line.
   Generated Lean declarations add only this macro. Two generated legacy
   Quality matches admit no Name payload and only receive the exhaustive
   enum arm. No glossary gap. One semantic value test beyond the original
   ticket proves that the existing carrier preserves a printed card name,
   including its comma (PW7).
   Existing numeric inventory assertions are noted in hygiene-3; two timing
   decimals in the unclaimed predicative-gap-relative ticket were rewritten
   with minute/second units without changing their values, fixing citation
   false positives already present on the claim parent. Refresh conflicted
   only on those same timing fixes; default had also fixed them. Kept
   default's values and wording, removing this feature's redundant edit.
   Incoming grammar/data: `onymknyk` (mana passive subjects) and `zquvqlrm`
   (additive slash adjustments); refreshed, conflicts cleared, xtask rebuilt.
   No incoming plugins. Post-refresh bounded census is identical; full
   touched-crate checks passed on `ksntxunw`.
5. **Tests — `opkxokpk`; post-refresh `ksntxunw`.** PW7 verification passed in full:
   `cargo test -p deckmaste_english_v3`, `cargo test -p deckmaste_construction_core`,
   `cargo test -p deckmaste_lexical_source`, `cargo test -p deckmaste_semantics_v2`
   and `cargo test -p xtask`. English: 1,137 passed, one existing ignored;
   xtask: 425 passed, one existing ignored. `cargo xtask facts check` passes
   unchanged generated Facts. The initial xtask run's 16 parameter-schema failures
   were resolved by PW7; its stale generated Lean declaration was regenerated.
   A new negative test's expected value was corrected to the existing earlier
   `UnsupportedKeywordParameterSignature` rejection of `[NounPhrase]`,
   preserving its rejected subject. Clippy passed:
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_core
   -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p xtask --tests --
   -D warnings`. `cargo xtask lean check` passed (245 cards) after regenerating
   declarations; Lean LSP reports no errors for the new macro. After refresh,
   `cargo build -p xtask`, the same five full test commands and the same
   five-crate clippy command all passed on `ksntxunw`: English 1,147 passed
   with one existing ignored; xtask 425 passed with one existing ignored.
   No incoming plugins required another Lean run. Chart-budget tests passed
   without changing their ceilings.
   Citation checks: zero noncompliant, zero stale; bless changed no lock;
   diff audit selected zero new citation sites. Restored 0 / re-spelled 3 /
   newly ignored 0 / added 7 / removed 0. Re-spelled:
   `builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`
   now expects the separate Name recipe and `[Quality]` signature (parameterized
   inventory 110 → 111);
   `every_keyword_declaration_pins_its_template` adds
   `partnerWith: "partner with <Param(0)>"`;
   `every_keyword_template_is_consistent_with_its_params_and_grammar` counts
   66 + 210 declarations instead of 66 + 209, retaining all its parameter and
   template assertions. Four structural English tests, two parameter-recipe
   tests and one semantic value test added; no existing test removed or weakened.
6. **Timing — `ksntxunw`.** Final refreshed bounded census: 6.66s wall under
   `/usr/bin/time -v`, 6 workers, samples per face 0. The bounded census was
   rechecked after PW7, with zero further Reading or digest changes; no
   full-corpus run or `--all`.
