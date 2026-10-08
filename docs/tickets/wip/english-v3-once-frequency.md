---
needs: [english-v3-focusing-only]
---
# Read bounding frequency *once* with a distributive NP: only once each turn

## Why

*Once each turn* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *only once each turn*
touches **281** unread faces and is the sole cause on **80**; *once* without
*only* occurs on 11 (recon bucket "activation/timing restriction", read for
this string). Counts are unread faces *touched* (at least one localised failing
unit matches) / *sole* (every failing unit matches and no other recon STRONG
bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Draw a card each turn." 1; "Draw a card once." 0;
"Draw a card once each turn." 0; "Draw a card twice." 0; "Activate only once." 0.
The *each turn* NP Adjunct reads; *once* does not.

## Goal

*once* (and *twice*) read as a bounding frequency Adjunct, optionally followed
by a distributive temporal NP (*once each turn*), clause-finally. Combined with
`english-v3-focusing-only` (the `needs:` edge), *Activate only once each turn.*
and *Do this only once each turn.* read with *only* focusing the frequency
Adjunct.

## Analysis

The bounding frequency expressions are the adverbs *once* and *twice*, NPs
headed by *times*, and *on* + *occasion* PPs (CGEL, Ch. 8, §9, p. 715). *Once*
can also express temporal location (*I once liked this kind of music*; p. 715,
[8ib]); only the frequency use is attested in these witnesses, so the
temporal-location use is not added (pruning rule). How *each turn* combines
with *once* (a separate NP Adjunct or a dependent of *once*) is for the landing
to settle and record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket except the restriction bucket (`xxknlzypsnwy`):

- Kozilek's Translator: "Activate only once each turn."
- Jeskai Devotee: "Activate only once each turn."
- Basking Rootwalla: "Activate only once each turn."
- Twinblade Slasher: "Activate only once each turn."
- Ondu Spiritdancer: "Do this only once each turn."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-once-frequency-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-once-frequency-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *once* read as a temporal-location Adjunct or as a subordinator
   (*once X happens*), or *each turn* read as the Object, is a defect. A wrong
   analysis that parses is a defect, not a gain.
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
11. Timings as integer ns and ns/B, with host load and worker count.
12. Tests: `cargo test -p <crate>` and clippy for every crate changed; Lean checks only if Lean sources or plugin definitions changed.

## Out of scope

- Subordinator *once* with a clause, *the first time each turn*, *N times*,
  *twice that many*.
- *Activate only once each turn and only if …*: coordination of the two
  restrictions is ordinary coordination; note it, do not build it.

## Landing record

In addition to the standard record: the frequency Adjunct's category and the
combination with *each turn*; whether *twice* is attested and added; Reading
counts of each witness; before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.

### Superseded implementation checkpoint and STOP (2026-10-08)

This checkpoint describes the void first census and the STOP before the
orchestrator resolution below. It is not the final landing evidence.
The full census has been consumed under the user's binding one-run budget;
its mechanically validated gains remain provisional pending the repair below.

Measured tree **`wzlsrqmlskqpplpvwxwzuzlokyxwslrm` / covered 21,916**;
claim-parent-equivalent baseline **`urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` /
covered 21,701**, original claim parent `vozmwtpxlwsypukuktrunnuwsppmkrlu`.
`target/english-v3/english-v3-once-frequency/baseline-provenance.json` records
source equivalence and the shared input hash. The refreshed trunk focusing
self-stacking guard is from `sqzukyvnqplmsyxxrprltlkztpwtzkrs`; no count decrease
was brought in by that refresh.

| Census | Before | Attempted feature |
| --- | ---: | ---: |
| Supported faces | 32,828 | 32,828 |
| Covered faces | 21,701 | 21,916 |
| No Reading | 11,127 | 10,912 |
| One Reading | 7,472 | 7,514 |
| Multiple Readings | 14,229 | 14,402 |
| Exact Readings | 782,215 | 944,281 |

Face-by-face comparison: **215 provisional gains, zero losses, zero Reading
count decreases, nine previously covered count increases**. Every face completed
without limits, issues, duplicate derivations, cyclic derivations or internal
failures. Mechanical validation establishes admission, lexical ownership,
byte-exact realization and node/Word traversal identity; it does not establish
linguistic validity. All 215 gains have sampled structural analyses matching
the full census's count and complete Reading digest in
`gain-analysis-ledger.json`; no gain is certified for landing yet.

**STOP: transitive Do exposes a shared pronoun/relative-boundary defect.**
The two complete Readings of Step Between Worlds in `increase-selector.json`
show the new extra route: `ObjectRelativeNominal` → `ZeroObjectRelativeClause`
uses Who as an ordinary `NominativePronoun` subject and Do's new transitive
frame as a `FiniteObjectGap`. The intended reading of *each player who does*
is a `PronounSubjectRelative` with Who as marker, auxiliary Do and an elided
Bare Complement. The extra Reading is invalid, not a coverage benefit.

| Exposed previously covered face | Before | Attempted feature |
| --- | ---: | ---: |
| Tempt with Immortality | 250 | 500 |
| Step Between Worlds | 1 | 2 |
| Tempt with Glory | 2 | 4 |
| Tempt with Bunnies | 12 | 24 |
| Tempt with Reflections | 16 | 32 |
| Turtles in Time | 2 | 4 |
| The Second Doctor | 4 | 8 |
| Tempt with Vengeance | 896 | 1,792 |
| Tempting Contract | 1 | 2 |

The counts are from the attempted-feature tree stamped above; the mechanism
is directly demonstrated on Step Between Worlds, and the other eight form the
matching *who does* exposure set. Removing the required transitive Do frame
would undo the Ondu witness. A word-naming exclusion is forbidden. The proposed
repair is a declared relative/ordinary-pronoun distribution permission enforced
at ordinary NP admission. That changes the shared pronoun/relative boundary
outside this ticket and may retire inherited invalid Readings elsewhere.
Under the assurance rule, this scope expansion needs a ruling. It would also
change grammar after the one full census; that measurement cannot certify the
repaired finished feature under the binding budget. **No repair applied;
no integration; STOP unresolved.** Detailed evidence and the proposed repair
are recorded in `target/english-v3/english-v3-once-frequency/STOP.md` and
`increase-reading-diff.txt`.

Witness counts on the attempted-feature tree: Kozilek's Translator **1**,
Jeskai Devotee **24**, Basking Rootwalla **2**, Twinblade Slasher **2**, Ondu
Spiritdancer **2**. Once/Twice are bounding-frequency Adverb Phrases in final
Adjunct function; Only modifies that phrase. *Each turn* is a separate existing
temporal NP Adjunct, not the Object. The exact constituency is this landing's
project analysis, not attributed to CGEL. CGEL Ch. 8 §9, pp. 714–715 supports
the frequency classification; Ch. 6 §7.3.1, p. 587 [42ii,iv] supports focusing
modification of PPs/AdvPs. Twice is attested by Nadu, Winged Wisdom (**4**)
and Agent Frank Horrigan (**5**). The attempted implementation replaces the
PP-only focusing route with the shared PP/AdvP schema and reuses the existing
Once/Twice lexical identities; no parallel FrequencyPhrase route remains.

Checks passed: `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source`
(**393** English integration tests, **56** lexical-source tests; zero failed or
ignored), and `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source
--tests -- -D warnings`. English tests and clippy passed again after refresh.
`cargo +nightly fmt -p deckmaste_english_v3 -p deckmaste_lexical_source -- --check`
passed. Citation checks: **0** noncompliant, **0** stale; the actual feature diff
piped to `cargo xtask cite audit --diff` selected **0** changed CR sites. No Lean
checks, xtask tests or derived gate. Assurance relative to claim parent:
restored **0**, re-spelled **2**, added **10**, ignored **0**, removed **0**.
An intermediate auxiliary frame-index regression was fixed in the data while
leaving its existing independent roundtrip test unchanged.

The two re-spelled existing tests are
`focusing_only::focus_does_not_supply_missing_constituents_or_other_functions`
and
`focusing_only::independently_constructed_focus_preserves_structure_and_lexical_identity`.
Independent new focus and fused-head/Do values retain full realization/parse-back
value comparisons; node and lexical leaf identities are asserted for both
frequency adverbs. The glossary adds Fused Determiner-Head and Fused Head Use
(CGEL Ch. 5 §§9.1–9.2, pp. 410–415), and explicitly extends the project Focused
Use permission to AdvPs. Zero new word-naming guards; environment load-error
tests pass. V3 has no coverage lock or legacy licensing-checker totals;
these fields are not applicable.

Provenance on the attempted-feature tree stamped above: **202 constructions +
49 shared schemas = 251**, **706** compiled productions, **146** Categories,
**3,413** nonblank declaration lines (baseline **3,360**, refreshed parent
**3,379**). The distinct fused-head NP needed by Ondu adds one construction;
the final-breadth target is not met by hiding or merging unrelated grammar.
Named inventories in `inventory-before.json` and `inventory-after.json`:
spellings **38,380 → 38,392**, homographs **1,139 → 1,139**, case-folded
homographs **1,092 → 1,092**, form literals **23 → 23**, form-literal/vocabulary
overlaps **0 → 0** (empty named list). Added spellings: Process, Processes,
Repeat, Repeated, Repeating, Repeats and their lower-case forms. The full lexical
hash `b0b0b7a98db683ad92f7c61e8c98791fbced1d61b5e04882b685d9bf8b57018f`
matches the census. Native Repeat/Object and Process entries are attested by
Remorseless Punishment (**6**) and disclosed as additions beyond the five
mandatory witnesses, along with fused-head This and transitive Do required by
Ondu; verbs.ron, plugin bodies and Lean sources are unchanged.

Performance advisory: before at `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered
**21,701**, corpus wall **88,681,432,200 ns**, setup **2,387,749,763 ns**,
checked-text thread CPU **383,972 ns/B**, **12** workers, host load
`[18.49072265625, 25.17041015625, 24.150390625]`. Attempted feature at
`wzlsrqmlskqpplpvwxwzuzlokyxwslrm` / covered **21,916**, corpus wall
**122,732,791,543 ns**, setup **4,266,048,131 ns**, checked-text thread CPU
**426,264 ns/B**, **12** workers, host load
`[8.1982421875, 7.05908203125, 8.88525390625]`. Both exceed the quiet-host
**16,260,000,000 ns** ceiling; neither was measured on a quiet host. These are
advisory measurements, not a performance gate.

Evidence is entirely under `target/english-v3/english-v3-once-frequency/`.
The fuller draft analysis is `landing-prose.md`; completing this record,
identity-level gain disclosure and integration await resolution of the STOP.

### STOP resolution — orchestrator ruling, 2026-10-08

> “apply your proposed-relative-boundary.patch as is”

> “One replacement full census (--workers 12) is authorized; the earlier census is void.”

The ruling authorizes the generic declared-feature repair: RelativeUse gains
a No default and ordinary NominativePronoun admission requires RelativeUse No.
The restriction is at the lexical-pronoun head of the NP, inside the existing
CasePhrase projection; relative pronouns remain markers of their relative
construction. The patch is applied exactly. A new regression test asserts
a single Reading of *each player who does*, including full independently
constructed PronounSubjectRelative/auxiliary-Do/elided-complement equality.
The earlier full report is retained as `after-void.json`, never final evidence.
The replacement full census and all-face loss/Reading reconciliation below
will provide the final measurements.
