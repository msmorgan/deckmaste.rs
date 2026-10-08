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
focusing *only*, with byte-exact realisation. Coordination and focused Subjects
are excluded by the user ruling of 2026-10-08. Once the prerequisites
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

The user ruling of 2026-10-08 replaces the three witnesses with independent
gaps. At the claim-parent bounded baseline `mqrlzszk`, each full face below
has zero Readings; its body without the final restriction reads (28, 1, 1, 1,
and 1 Readings respectively). Pre-implementation `english-v3 probe` evidence
is under `target/english-v3/english-v3-clause-final-predicative-as/`.

- Experimental Synthesizer: "Activate only as a sorcery."
- Lion's Eye Diamond: "Activate only as an instant."
- Sun-Blessed Guardian: "Activate only as a sorcery."
- Intrepid Tenderfoot: "Activate only as a sorcery."
- Cat Burglar: "Activate only as a sorcery."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (user ruling, 2026-10-08): one bounded census of
   every supported face containing case-insensitive " as " plus 300 random
   covered faces (seed 20261008), using the identity manifest and 12 workers,
   with `--samples-per-face 0`; compare against the claim-parent bounded
   baseline with `cargo xtask english-v3 diff`. Never run `--all`. Tests:
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
- Mixed PP/AdvP coordination under *only*, *but only …*, and focused Subjects
  (user scope ruling, 2026-10-08); the three original blocked witnesses return
  to `english-v3-systemic-residuals` recon.
- Re-opening the 2026-10-07 ruling: if the prerequisites have not landed,
  STOP rather than widen.

## Landing record

1. **Census delta.** Bounded claim-parent baseline `mqrlzszk`: 2,123 covered faces / 145,758 Readings → `noppnrqz`: 2,488 / 295,566 (+365 faces / +149,808 Readings). At `noppnrqz`, lost faces none; removed Readings none; decreases, increases on previously covered faces, digest-only changes, validation issues and internal failures all zero. Every counted Reading passed admission, byte-exact realization and node/word traversal checks. Official comparison: `target/english-v3/english-v3-clause-final-predicative-as/diff.json`. After refresh (`qvurzynu`), the same manifest still yields 2,488 covered faces / 295,566 Readings; the official `diff-refresh.json` reports all 4,253 faces unchanged, including zero inherited decreases. `diff-final.json` retains the +365 faces / +149,808 Readings against the claim-parent baseline.
2. **Witnesses.** All five full faces failed pre-implementation probes with zero Readings (`mqrlzszk`); their bodies without the restriction read, confirming the only failing unit. After (`noppnrqz`): Experimental Synthesizer 56; Lion's Eye Diamond 2; Sun-Blessed Guardian 2; Intrepid Tenderfoot 2; Cat Burglar 2. Each final *as* phrase has retained VP and Clause Adjunct attachments. Counterbore 28, Aven Shrine 9 and Clone 6 (`noppnrqz`), identical to baseline; no Reading makes their *as* phrase an Adjunct.
3. **Spot checks.** Ten newly covered faces at `noppnrqz`: Experimental Synthesizer, Lion's Eye Diamond, Sun-Blessed Guardian, Intrepid Tenderfoot, Cat Burglar, Seeker of Sunlight, Harried Artisan, Concealing Curtains, Ravenous Demon and Vodalian Hypnotist. All select a focused nominal predicative *as* PP as a final VP or Clause Adjunct, with no NP attachment; *instant* on Lion's Eye Diamond and *sorcery* on the other nine. Exhaustive probes completed without issues (`noppnrqz`).
4. **Deviations and STOPs.** The scope STOP was resolved by the user ruling of 2026-10-08: replace three witnesses and exclude their independent repairs. The existing nominal-as Construction was refined in place, with declared Predicative Adjunct Use on the two attested timing nouns and general feature projection; no frame or Construction was added or deleted, and no duplicate or superseded route remains. The owning glossary defines the new licence; its final-position authority is the orchestrator ruling, while CGEL p. 263 backs the predicative function only. No word-naming guard or unresolved glossary gap. Refresh incorporated coordinated-subject agreement (`szlklvns`); its sole merge conflict was resolved by retaining both Nominal Agreement Number and Predicative Adjunct Use exports.
   In the Trenches — residual routed to `english-v3-systemic-residuals` recon: mixed PP/AdvP coordination under *only*.
   Endbringer's Revel — residual routed to `english-v3-systemic-residuals` recon: *but only …*.
   Detention Vortex — residual routed to `english-v3-systemic-residuals` recon: focused Subject *Only your opponents may …*.
5. **Tests.** `cargo test -p deckmaste_english_v3`: 540 passed; `cargo test -p deckmaste_lexical_source`: 59 passed; `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings` passed; formatting check passed (all at `noppnrqz`). Assurance counts at `noppnrqz`: restored 0; re-spelled 3 (Mind Slash's deferred-as exclusion, bare-as final-position exclusion, and Enter's sorcery control); ignored with blocker 0; added 7; removed 0. All original surfaces and the copy/comparison exclusions remain asserted. After refresh (`qvurzynu`), `cargo test -p deckmaste_english_v3 --test main clause_final_predicative_as::` passed 7; the same command with `focusing_only::`, `enter_as_copy_complement::`, and `coordinated_subject_agreement::` passed 12, 8, and 5 respectively. `cargo test -p deckmaste_lexical_source` again passed 59; the same two-crate clippy command passed. `cargo fmt -p deckmaste_english_v3 -p deckmaste_lexical_source -- --check` passed. The refreshed workspace-wide formatting check reported only inherited differences in semantics-v2 (`xrplswky`) and xtask (`ktzmknxk`), outside this construction. No xtask tests or Lean checks.
6. **Timing.** One finished-feature bounded census: 68.480 s including setup, 12 workers (`noppnrqz`). Manifest: 4,253 faces (`mqrlzszk` selector), comprising all 3,976 supported faces whose raw text contains case-insensitive " as " plus 300 random covered controls, seed 20261008, with 23 overlapping identities. Commands: `cargo xtask english-v3 --identity-manifest target/english-v3/english-v3-clause-final-predicative-as/manifest.json --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-clause-final-predicative-as/after.json`; `cargo xtask english-v3 diff BEFORE.json AFTER.json --json target/english-v3/english-v3-clause-final-predicative-as/diff.json`. Evidence stays under that ignored target directory. No full corpus census was run. The changed grammar and lexicon brought in by refresh were checked with the same bounded manifest (`qvurzynu`), followed by affected tests only; the full English suite was not repeated.
