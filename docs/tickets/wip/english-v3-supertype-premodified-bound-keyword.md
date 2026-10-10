---
needs: []
---
# Read a supertype Premodifier on a bound-stem keyword: Basic landcycling {1}{G}

## Why

*Landcycling {2}* and *Swampwalk* read; the same keywords with a supertype
before the stem do not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:basic landcycling` 35
sole-blocker faces. `cargo xtask english-v3 grep -i
'\b(snow|legendary|nonbasic|basic) [a-z]*(walk|cycling)\b'` finds **44**
faces, all unread; all 44 failing lines match and **43** are the only failing
line of their face. By shape: *Basic landcycling <cost>* 36 (35 sole);
*Snow forestwalk* / *Snow swampwalk* / *Snow landwalk* 4; *Legendary
landwalk* 2; *Nonbasic landwalk* 2 (one as a granted keyword mention,
Trailblazer's Boots).

`english-v3-typecycling-bound-suffix` left this open (its R8b: *basic* is a
supertype premodifying the *land* stem, not a two-word stem; "record the
count and leave it for a follow-up"), and `english-v3-data-batch-2` deferred
it as needing more than data. This is that follow-up; the landwalk residuals
are the same gap.

## Goal

A bound-stem keyword whose stem is premodified by a supertype reads as a
Keyword Line and as a keyword mention, with byte-exact realisation:
*Basic landcycling {1}{G}*, *Snow forestwalk*, *Legendary landwalk*,
*has nonbasic landwalk*.

## Analysis

No CGEL citation applies (Oracle keyword formation). The glossary's **Bound
Keyword Surface** entry already names *nonbasic landwalk* and *Snow
swampwalk* as cased-together forms.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`BoundQualityKeyword` (`[quality: lexical(Noun), head: lexical(Keyword)]`,
`require quality.frame = BoundKeywordStem`) and `BoundQualityCostKeyword`
(the same plus `" ", cost: CostSymbols`, `head.KeywordParameterClass =
QualityCost`). Declarations: `plugins_v2/builtin/macros/keyword_abilities/
{landwalk,typecycling}.ron` (`bound_suffix: (surface, stem_class)`). The
supertypes are `vocab:Supertype/Basic`, `vocab:Supertype/Snow`,
`vocab:PredicativeAdjective/Legendary` (`lexicon/core.ron`), and *nonbasic*
is the class-declared negative formation.

Orchestrator rulings 2026-10-10:

- SB1. R8b stands: the supertype is a Premodifier of the stem, never part of
  a two-word stem Lexeme and never a new stem-class member. Refine the two
  existing Constructions with one optional leading slot
  (`[modifier: lexical(Adjective), " ", quality, head, …]`); no new
  Construction. If a form cannot be optional there, one added form per
  Construction is the fallback; a third Construction is a STOP.
- SB2. The licence reads declared features: the modifier's declared
  supertype class (the feature the Type Line and noun premodification already
  read) and the stem's `BoundKeywordStem` frame. No test on *basic*, *snow*,
  *land* or the keyword. If no declared feature separates supertype
  Adjectives from other Adjectives, STOP and report rather than listing
  words.
- SB3. Do not restrict which supertype meets which stem: *Snow forestwalk*
  and *Legendary landwalk* are attested, and no check is added whose only job
  is to block an unattested pairing.
- SB4. One analysis per string. The twins to avoid: (a) a compound stem
  Lexeme *basic land* beside the Premodifier analysis; (b) *Basic* read as a
  separate `BareKeyword` or Type Line fragment followed by *landcycling*;
  (c) in mentions, the Adjective attaching to a noun outside the keyword
  (*has nonbasic landwalk* has one Object, the keyword).
- SB5. Capitalization follows the glossary entry: the Premodifier takes the
  line-initial capital and the stem stays lowercase (*Basic landcycling*,
  *Snow swampwalk*); mid-line *legendary landwalk* (Livonya Silone) is all
  lowercase.
- SB6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face matches the attestation pattern. Any
  increase is reported with its shape before landing.
- SB7 (orchestrator, 2026-10-10). Extend `TypeLineRole`, without a dedicated
  supertype-adjective feature. Declare `Supertype` on the existing owning
  Adjective lexemes in data; their negatives inherit it through the existing
  derivation. The Construction reads `modifier.TypeLineRole = Supertype`.
  Other consumers of `TypeLineRole` stay unchanged. Include every face
  containing a supertype word in the selector and show zero changes on covered
  faces. Keep the separate Adjective and Catalog owners as lexicon residue;
  do not merge them here.
- SB8 (orchestrator, 2026-10-10). No helper Constructions. First try an
  optional existing Category that projects an Adjective without phrasal
  material, following NI-R1. Otherwise add general optional `lexical(...)`
  support to the construction compiler, minimally, with compiler tests and
  its full suite/clippy. Both bound-keyword Constructions retain one form.
  Preserve absent-field digests; disclose digest-only changes by class.
- W9-A (standing orchestrator ruling). One analysis per string means one
  route for this construction; preserve and record independent ambiguity
  already present in the control.

### SB2 STOP on claim parent `toslutpn` (2026-10-10)

No declared feature separates supertype Adjectives from other Adjectives.
`TypeLineRole = Supertype` is declared on the `catalog:supertypes.txt/*`
Catalog atoms consumed by `SupertypePrefix`, not on the adjective owners.
The `adjective_classes` recipe declares negative-prefix joining only and
groups Basic, Legendary and Snow with color adjectives; noun
premodification accepts ordinary Adjectives without a supertype distinction.
SB2 therefore requires a STOP before implementation. A further ruling is
needed to declare the adjective licence and preserve it on generated negative
forms. Resolved by SB7 above.

### SB1 compiler capability STOP (2026-10-10)

The claim-parent construction compiler cannot express either prescribed
representation. `optional(lexical(Adjective))` reports "unexpected field
arguments"; adding the fallback form's leading `modifier` field reports
"every form must retain the same named fields and cardinalities".
The existing optional fields hold constituent Categories, not lexical Words.
Confirmed by calling the current compiler on minimal declarations, without
editing the grammar or building tests; source and diagnostics are under
`target/english-v3/english-v3-supertype-premodified-bound-keyword/` as
`compiler-capability.rs` and `compiler-capability.txt`.

Continuing requires a compiler representation extension or a helper
Construction (the latter is explicitly a STOP under SB1). Resolved by SB8: `AdjectivePhrase` admits complemented/coordinated material;
there is no suitable narrower existing Category. The general compiler path
is used, with no helper Construction.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Gleam of Resistance: "Basic landcycling {1}{W}"
- Ash Barrens: "Basic landcycling {1}"
- Rime Dryad: "Snow forestwalk"
- Ayumi, the Last Visitor: "Legendary landwalk"
- Dryad Sophisticate: "Nonbasic landwalk"

Also 0 today: "First strike; legendary landwalk" (Livonya Silone);
"Equipped creature has nonbasic landwalk." (Trailblazer's Boots, sentence).
Controls that must not change: "Landcycling {2}" 1; "Plainscycling {2}" 1;
"Swampcycling {2}" 1; "Forestwalk" 1; "Landwalk" 1; "First strike;
swampwalk" 1; "Equipped creature has swampwalk." 1; "Search your library for
a basic land card." (sentence) 3+.

Claim-parent re-probes on `toslutpn` (2026-10-10), before implementation:
all seven witnesses above remain 0; the first seven controls remain 1 each,
and "Search your library for a basic land card." has 3. Every enumeration
completed, with no issues. Commands used `cargo xtask english-v3 probe
--text TEXT --category document --readings 100 --json` (the three sentence
cases used `--category sentence`). Reports and exact command arguments are
under `target/english-v3/english-v3-supertype-premodified-bound-keyword/`,
including `before-probes.json`. No witness was dropped.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)[a-z]*(walk|cycling)\b|\b(?:non)?(?:basic|snow|legendary|world)\b`
(1,600 faces on `toslutpn`, SB7) plus 300 random
covered controls (`cargo xtask english-v3 diff --manifest … --random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; SB6 for
increases. Full `cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings` (and the same for
`deckmaste_lexical_source` and `deckmaste_construction_v3_core`, whose
data/compiler changed under SB7/SB8), once, at the end: a rebuild after a grammar
edit takes about 2–3 minutes and the six test binaries about 75 seconds, so
batch edits; plugin `grammar:` data loads at runtime. Tests assert the role
(the supertype as Premodifier of the Bound Keyword Stem, the cost as the
keyword's parameter, the SB4 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: extend **Bound Keyword Stem** with the Premodifier in the landing.

## Out of scope

- *gains snow landwalk of the chosen type* (Barbarian Guides, 1 face): the
  *of the chosen type* dependent of a keyword mention.
- Typecycling and landwalk engine bodies.
- Cycling-trigger text (*When you cycle this card*).

## Landing record

1. **Census delta** (`ooumkosu`; BEFORE source is claim parent `toslutpn`, commit `54cab345`):
   1,881 selected faces; covered **1,622 → 1,664**, Readings
   **79,869 → 80,129**. Official `english-v3 diff --lines`: gained **42**,
   lost **none**, decreases **0**, increases **0**, digest-only **0**;
   all 1,622 previously covered faces unchanged, including the 300 seeded
   controls. Selector: `(?i)[a-z]*(walk|cycling)\b|\b(?:non)?(?:basic|snow|legendary|world)\b`,
   1,600 surface faces plus 300 random covered faces (seed 20261008), 19
   overlaps. Evidence: `target/english-v3/english-v3-supertype-premodified-bound-keyword/{manifest,before,after,diff}.json`.
   Post-refresh `pqzrnyok`: **1,622 → 1,685** covered, **79,869 → 81,695**
   Readings; gained **63**, lost/decreases/increases/digest-only all **0**.
   Official `diff-incoming.json` attributes **21** additional gains to trunk:
   19 named-token faces (`tlpwnxqnnquu`), Gandalf the White's cause frame
   (`nzmrrqkysxto`), and Vision Charm's phase frame (`roomopmrpkys`).
   The first AFTER's 1,664 covered faces all retain their counts and digests.
   Refreshed reports: `after-refresh.json`, `diff-refresh.json`.
   Second refresh (`zrsxnupx`) adds Ultimate Spider-Man from
   `wkooyukozlpw`: final covered **1,622 → 1,686**, Readings
   **79,869 → 81,720**, gains **64** (42 ticket / 22 trunk), all other
   change classes **0**. Its incoming diff leaves all 1,685 covered faces
   unchanged. Final reports: `after-refresh2.json`, `diff-refresh2.json`,
   `diff-incoming2.json`.
   Accepted-review refresh: covered **1,622 → 1,693**, Readings
   **79,869 → 87,954**, gains **71** (42 ticket / 29 trunk); lost,
   decreases and covered-face increases remain **0**. Final digest-only
   changes: **27**. The refresh-only diff reports seven additional gains:
   Avengers Disassembled, Fissure Vent and Invasion of Fiora from modal
   headers (`ztrnpkyvzntv`); Gloryheath Lynx from the while adjunct
   (`wxwozrtzrsqp`); Kotose, the Silent Spider from preposed duration
   (`rtzolrtxrrru`); Loyal Retainers from temporal supplementation
   (`mpvousxzotns`); Wild Endeavor from die nouns (`vlxrulqwmzzk`).
   Its 28 digest-only changes (including previously gained Vision Charm)
   retain their Reading counts and are attributed to `ztrnpkyvzntv`'s
   refined count Complement structure. Reports: `after-refresh3.json`,
   `diff-refresh3.json`, `diff-incoming3.json`.
2. **Witnesses** (`ooumkosu`): Gleam of Resistance, Ash Barrens, Rime Dryad,
   Ayumi, the Last Visitor, Dryad Sophisticate, Livonya Silone and
   Trailblazer's Boots each **0 → 1** on their ticket line/sentence.
   Enumeration complete, byte-exact, no issues. Seven bare-keyword/mention
   controls remain **1** each; the basic-land search sentence remains **3**.
   Its independent `for`-phrase complement, predicate-adjunct and library
   postmodifier readings are preserved under W9-A.
3. **Spot checks** (`ooumkosu`): Gleam of Resistance and Ash Barrens select
   `BoundQualityCostKeyword` with Basic modifying the land stem and
   `{1}{W}` / `{1}` as cost; World-Weary selects that same route with
   `{1}{B}`. Rime Dryad, Legions of Lim-Dûl and Zombie Musher select
   `BoundQualityKeyword` with Snow and forest/swamp/land stems; Ayumi
   selects Legendary + land; Dryad Sophisticate selects derived Nonbasic +
   land; Livonya selects interior lowercase Legendary + land in coordination;
   Trailblazer's Boots selects derived nonbasic + land inside the existing
   single GrantedAbility complement. All ten selected keyword analyses are
   byte-exact; no unintended gain found.
4. **Deviations and STOPs**: SB2's missing-feature STOP resolved by SB7:
   four owning Adjective rows declare existing `TypeLineRole = Supertype`,
   negatives inherit; other consumers unchanged. Separate Catalog/Adjective
   owners remain a lexicon residue, per SB7. The optional-field capability
   STOP resolved by SB8: no existing narrow Category fits, so the compiler
   generally accepts `optional(lexical(Category), "separator")` and checks
   direct requirements only on present Words. Added compiler tests under
   SB8; no Construction or frame added/deleted, one form retained in each
   bound Construction. Generated Debug omits absent optional lexical fields
   to preserve existing digests. Glossary extended; no missing term or CGEL
   claim. The older 43-gain estimate becomes 42: Hexhaven Battalion's keyword
   now reads **1**, but its other line remains **0**; Barbarian Guides retains
   the explicitly excluded keyword dependent. Counts measured on `ooumkosu`.
5. **Tests** (`ooumkosu`): all passed:
   `cargo test -p deckmaste_english_v3` (900),
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical_source` (84),
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`;
   `cargo test -p deckmaste_construction_v3_core` (39),
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`.
   Initial new compiler-test encoding expectation corrected; compiler function
   length and English test-helper clippy findings fixed, then affected full
   suites/clippy passed. Restored **0**, re-spelled **2** (Gleam residual and
   negative-role inheritance), ignored **0**, added **6**, removed **0**;
   existing typed bound values also supply `modifier: None`.
   `cargo build -p xtask` passed; citation checks: 0 stale, 0 noncompliant,
   diff audit 0 new sites. After refresh (`pqzrnyok`), `cargo build -p xtask`
   and the same six full test/clippy commands passed again: English **913**,
   lexical-source **84**, compiler **39**; the 13 extra English tests belong
   to the incoming landings. Citation checks remain 0 stale/noncompliant.
   After the second refresh (`zrsxnupx`, importing `wkooyukozlpw`), the
   same build and six full test/clippy commands passed again: English
   **918**, lexical-source **84**, compiler **39**. The five additional
   English tests belong to the kind-of-counter landing.
   Accepted-review refresh: `cargo build -p xtask` and the same six full
   test/clippy commands passed again: English **946**, lexical-source **86**,
   compiler **39**. Incoming landings account for the additional tests.
   Exact commands/logs are under the evidence directory (`refresh3-checks.json`).
6. **Timing** (accepted-review refresh): the required **bounded** AFTER
   census took **1:25.16** wall time under `/usr/bin/time -v`, **6 workers**,
   samples 0.
   No full census was run, per the binding verification budget.
