---
needs: [english-v3-same-as-comparative, english-v3-enter-as-copy-complement]
---
# Read a clause-final predicative *as* Adjunct: Activate only as a sorcery.

## Why

*Activate only as a sorcery* and its kin never read. On census `xnkuznpo`
(docs-only descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered,
9,369 unread; wave-4 recon of 2026-10-08), the recon bucket "clause-final
predicative *as* (*as a sorcery/instant*)" touches **504** unread faces and is
the sole cause on **372**, the largest single construction left. The string
*only as a sorcery* occurs in no text the grammar reads. Counts are unread
faces *touched* (at least one localised failing unit matches) / *sole* (every
failing unit matches and no other recon STRONG bucket does). They are surface
counts, not gain forecasts.

Probes (admitted roots, `--category Sentence --readings 1`): "Activate only
as a sorcery." 0; "Activate only as an instant." 0; "Activate only once each
turn." 1; "Activate only during your turn." 1.

This widening was deferred by the orchestrator ruling of 2026-10-07 recorded
in `english-v3-systemic-residuals` ("Clause-final predicative-as position"):
it waits for the *same … as* comparative and Enter's selected *as a copy of*
Complement, because the rejected focusing-only attempt misread both as this
Adjunct (Counterbore 0 → 39, Aven Shrine 0 → 9, Clone 0 → 18). Those two are
this ticket's `needs`.

## Goal

A clause-final *as* + NP Adjunct reads after a verb phrase, including under
focusing *only* and in coordination with another restriction (*only as a
sorcery and only once*), with byte-exact realisation. Once the prerequisites
have landed, *same … as* and Enter *as a copy of* keep their own Readings and
gain no Adjunct duplicate.

## Analysis

CGEL, Ch. 4 §5.3, p. 263 ("*As* phrases as predicative adjuncts"): predicatives
marked by *as* function as complement or adjunct; the adjunct takes a
pragmatically appropriate predicand. That passage backs the function only.
Admitting the Adjunct in clause-final position is the 2026-10-07 orchestrator
ruling, not a CGEL position claim.

Existing route: `vocab:Preposition/As` (`core.ron:181`) already licenses
`Adjunct|PreposedAdjunct` with a `PredicativeNounPhrase` Complement, and
`PredicativeComplementPreposition` (`declarations.rs:2689`) builds the PP.
Focusing *only* reads licensed final Adjuncts (`FocusedPhrase`). The missing
piece is the clause-final attachment of this PP, with its predicand left
implicit.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Experimental Synthesizer: "Activate only as a sorcery." (control "Activate
  only once each turn." 1)
- Lion's Eye Diamond: "Activate only as an instant."
- In the Trenches: "Activate only as a sorcery and only once."
- Endbringer's Revel: "Any player may activate this ability but only as a
  sorcery."
- Detention Vortex: "Only your opponents may activate this ability and only as
  a sorcery."

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
   Reading. An *as* phrase read as this Adjunct where it is a *same … as* or Enter Complement, or attached inside the NP it follows, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *any time you could cast a sorcery* (separate bucket, 44 sole faces).
- Ability bodies that fail on their own.
- Re-opening the 2026-10-07 ruling: if the prerequisites have not landed,
  STOP rather than widen.

## Landing record

The six items in CLAUDE.md. Also report, for each of Counterbore, Aven Shrine
and Clone, its Reading count and whether any Reading attaches *as* as this
Adjunct (expected: none).
