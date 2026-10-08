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

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
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

The six items in CLAUDE.md.
