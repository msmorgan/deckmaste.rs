---
needs: []
---
# Read a negative-prefixed common noun premodifier: Destroy all nontoken creatures.

## Why

*nontoken* never reads. On census `xnkuznpo` (docs-only descendant of
`vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369 unread; wave-4
recon of 2026-10-08), the recon bucket "*nontoken*" touches **324** unread
faces and is the sole cause on **224**. No text the grammar reads contains
*nontoken*. Counts are unread faces *touched* / *sole* as defined in the
recon; surface counts, not gain forecasts.

Probes (admitted roots): "Destroy all nontoken creatures." 0, while "Destroy
all nonland creatures." 1; "Exile target nontoken creature." 0, while "Exile
target nonartifact creature." 1. The lexer finds `vocab:NegativePrefix/Non`
plus `lexeme:CommonNoun/Token`; no single negative Lexeme exists.

## Goal

*nontoken* reads as a negative noun premodifier of a following Nominal
(*nontoken creatures*, *another nontoken Spirit*, *nontoken permanent*),
realised byte-exactly, with the same lexical shape type words already have
(*nonland*, *nonartifact*).

## Analysis

CGEL, Ch. 19 §5.5.1, p. 1687 lists *non·* among the negative prefixes. CGEL,
Ch. 6 §2.4.1(a), p. 537: nouns function as attributive modifiers in a
nominal.

Existing route: negative noun Lexemes are minted by `negative_lexeme`
(`crates/deckmaste_lexical_source/src/native.rs:233`), called only for type
words with `NounPremodifier` (`legacy/plugins.rs:84`, per the 2026-10-04 ADR
amendment on type macros). `lexeme:CommonNoun/Token` (`vocabulary.rs:419`)
is a common noun, outside that recipe. The consumer is `NounPremodifier` /
`NounPremodifiedNominal` (`declarations.rs:2232,2275`). This ticket declares
the negative formation for the common noun *token* through a declared
property, not a spelling rule, and reuses that consumer. The 2026-10-04 ADR
amendment names type and subtype declaration classes as the sources of
negative noun Lexemes; a common noun is outside both. If declaring the
formation on *token* reads as contradicting that amendment rather than
extending it, STOP for a ruling.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Hour of Reckoning: "Destroy all nontoken creatures." (control "Destroy all
  nonland creatures." 1)
- Swift Spiral: "Exile target nontoken creature." (control "Exile target
  nonartifact creature." 1)
- Rhythm of the Wild: "Nontoken creatures you control have riot." (control
  "Nonland creatures you control have riot." 1)
- Isilu, Carrier of Twilight: "Each other nontoken creature you control has
  persist."
- Thraben Watcher: "Other nontoken creatures you control get +1/+1 and have
  vigilance."

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
   Reading. A *non* read as a separate word, or *nontoken* read as the head noun of its phrase, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *permanent* as a noun premodifier (`english-v3-permanent-noun-premodifier`).
- Positive *token* as a premodifier (*token creature*) unless the corpus
  attests it on a face this ticket covers.

## Landing record

The six items in CLAUDE.md.
