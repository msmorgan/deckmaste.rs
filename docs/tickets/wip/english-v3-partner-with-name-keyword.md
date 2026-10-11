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
