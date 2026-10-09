---
needs: []
---
# Read the landwalk bound-suffix keyword: Swampwalk

## Why

No landwalk keyword reads, as a label line or in running text. On census
`xnkuznpo` (docs-only descendant of `vvxtllnq`; 32,828 supported faces,
23,459 covered, 9,369 unread; wave-4 recon of 2026-10-08), the recon bucket
"landwalk keyword (bound *-walk*)" touches **213** unread faces and is the
sole cause on **192**. Counts are unread faces *touched* / *sole* as defined
in the recon; surface counts, not gain forecasts.

Probes (admitted roots; label lines are Documents, running text is a
Sentence): "Swampwalk" 0 and "Forestwalk" 0 (`--category Document`), while
"Flying" 1; "Other Goblins get +1/+1 and have mountainwalk." 0 with
vocabulary gap *mountainwalk*, while the *flying* string reads 1. "Landwalk"
and "Legendary landwalk" read 0.

## Goal

A landwalk keyword reads as a Quality stem followed by the bound suffix
*walk*, with no separator, both capitalised at line start (*Swampwalk*) and
lowercase in running text (*gains forestwalk*), realised byte-exactly. The
generic *landwalk* and its quality forms (*legendary landwalk*, *snow
swampwalk*, *landwalk of the chosen type*) are counted if they read, not
required.

## Analysis

This is an Oracle keyword form, not an English construction CGEL describes;
no CGEL citation applies.

Existing route: `plugins_v2/builtin/macros/keyword_abilities/landwalk.ron`
declares `FixedKeyword(surface: "landwalk", bound_suffix: (surface:
"walk"))`; `bound_suffix_lexeme`
(`crates/deckmaste_lexical_source/src/legacy/plugins.rs:297`) emits the
suffix Lexeme with `KeywordPayloadOrder = BoundSuffix`. The grammar declares
that value (`declarations.rs:191`), but only `AfterHead` (`QualityKeyword`,
`:1821`) and `BeforeHead` (`BeforeHeadQualityKeyword`, `:1830`) have
consumers. The keyword-labels landing recorded the remaining obligation: a
declared stem class and a bounded consumer. The lexer analyses *Swamp* as
`lexeme:land_subtype/swamp` plus the suffix, but lowercase *swamp* inside
*forestwalk* is a vocabulary gap today; the stem class must supply the
lowercase bound stems from the land-type declarations, not from a spelling
rule.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Sheoldred, Whispering One: "Swampwalk" (Document; control "Flying" 1)
- Scarwood Bandits: "Forestwalk" (Document)
- River Bear: "Islandwalk" (Document)
- Goblin King: "Other Goblins get +1/+1 and have mountainwalk." (control
  with *flying* 1)
- Veldrane of Sengir: "Veldrane gets -3/-0 and gains forestwalk until end of
  turn." (ability body; control with *flying* 1)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (owner override, 2026-10-08): one bounded census of
   every supported face containing the case-insensitive substring *walk*,
   plus 300 random covered controls (seed 20261008), using an identity manifest,
   12 workers and zero samples per face. Compare against the claim parent's
   census with `cargo xtask english-v3 diff`; never run `--all`. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation (keyword labels as Documents,
   running text as Sentences); write the tests once and run them at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A stem read as a free noun with *walk* as a separate word, or a Reading that changes the source bytes, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- Typecycling (*Swampcycling {2}*, *basic landcycling*): separate bucket
  (91 sole), a different keyword shape.
- Engine semantics of landwalk.

## Landing record

1. **Census delta** (`swrqrxlw` → `pvmmspls`): bounded, case-insensitive
   *walk* selector: 1,213 supported faces plus 300 covered controls, seed
   20261008; ten overlaps, 1,503 distinct faces. Covered 991 → 1,166;
   Readings 67,340 → 68,928; 175 gains. Lost faces: none; decreases,
   increases and digest-only changes: zero. Claim-parent evidence combines
   the inherited census, the landed floated-each subset and pre-change runtime
   evidence (`mqrlzszk`, grammar/lexicon/core identical to the claim parent).
   That runtime evidence corrects inherited omissions for Cult Guildmage,
   Serum-Core Chimera and Termination Facilitator; these are not feature gains.
   The corrected manifest replaces the initial control sample. Official
   `english-v3 diff` evidence is under `target/english-v3/english-v3-landwalk-bound-suffix/`.
   All selected enumerations complete; all Readings validate exact roundtrip
   and traversal, with zero issues, duplicate Readings or internal failures
   (`pvmmspls`).
2. **Witnesses**: All five units fail with zero Readings before implementation
   (`mqrlzszk`, claim-equivalent cached runtime). After (`pvmmspls`):
   Sheoldred's *Swampwalk* 1; Scarwood Bandits' *Forestwalk* 1;
   River Bear's *Islandwalk* 1 (Documents); Goblin King's sentence 2;
   Veldrane's scoped sentence 3 (Sentences). All realized byte-exactly.
3. **Spot checks** (`pvmmspls`, whole-face counts): Anaconda (1),
   Anurid Murkdiver (1) — bound *Swampwalk* labels; Aysen Highway (1) —
   *White creatures* Subject granting bound *plainswalk*;
   Bayou Dragonfly (1) — *Flying, swampwalk* keyword line;
   Benthic Behemoth (1), Benthic Djinn (1), River Bear (1) — bound
   *Islandwalk* labels; Desert Nomads (8) — bound *Desertwalk* label;
   Goblin King (2) — coordinated clauses with a bound *mountainwalk* grant;
   Sheoldred, Whispering One (2) — bound *Swampwalk* label followed by
   triggered clauses. One selected Reading each inspected in `spot-0.json`
   through `spot-9.json`; all byte-exact, stems retain declared noun identities.
4. **Deviations and STOPs** (`pvmmspls`): Added the `BoundQualityKeyword`
   Construction, distinct `BoundKeywordStem` frame and declared singular
   noun-stem class. Desert joins the required hosts because Desert Nomads
   attests *Desertwalk*. Retired landwalk's unreachable free lexical export;
   there was no bound-suffix grammar consumer to supersede, and the other
   payload-order consumers remain. Generic *Landwalk* reads (1); *Legendary
   landwalk*, *snow swampwalk*, *nonbasic landwalk* and *landwalk of the chosen
   type* remain unread (0), as permitted. Added structural/roundtrip,
   malformed-form, free-noun-leakage and class-validation tests. Defined
   **Bound Keyword Stem** and corrected **Keyword Quality**'s Nominal-only
   wording; no glossary gap remains. Keyword formation follows the ticket and
   owner's ruling, without a CGEL claim. No STOPs; the owner's bounded-census
   override replaces Method's superseded full-census requirement.
5. **Tests** (`pvmmspls`): `cargo test -p deckmaste_english_v3` passes
   (558 tests). `cargo test -p deckmaste_lexical_source`: 26 unit tests and
   34 integration tests pass; one obsolete free-export assertion fails and is
   re-spelled against the bound suffix, preserving its landwalk owner and
   payload feature comparisons. The focused rerun
   `cargo test -p deckmaste_lexical_source --test main keyword_payload_features::authored_keyword_payload_contracts_preserve_separators_and_markers`
   passes. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` pass.
   Restored 0; re-spelled 1; ignored-with-blocker 0; added 6; removed 0.
6. **Timing** (`pvmmspls`): final bounded census, 15.700 seconds including
   setup, 12 workers.
