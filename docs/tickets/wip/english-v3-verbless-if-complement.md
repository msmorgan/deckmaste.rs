---
needs: []
---
# Read *if* with a verbless Complement: attacks each combat if able

## Why

*If able* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *if able / able to* touches **283** unread faces
and is the sole cause on **221** (recon bucket "if able / able to"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Ares attacks each combat if able." 0, while "Ares
attacks each combat." 1; "Jurin must be blocked if able." 0, while "Jurin must
be blocked." 1; "Target creature blocks this turn if able." 0; "Target creature
attacks this turn if possible." 0 with *possible* a vocabulary gap; "… if you
control a Zombie." 1.

This ticket takes over from `english-v3-systemic-residuals` the item "Reduced
if-able hosts."

## Goal

*if* takes a verbless clause consisting of the Adjective *able* as its
Complement, in clause-final Adjunct position, in finite
clauses with active and passive (*must be blocked*) predicates and under
coordinated predicates (*attacks or blocks this turn if able*). The
Adjective's unexpressed infinitival Complement is not realised.

## Analysis

*If*, *when*(*ever*), *where*(*ver*) and marginally *unless* take *necessary* and
*possible* as complements; the missing subject's interpretation derives from a
proposition in the matrix (CGEL, Ch. 14, §10(b), p. 1267, [4]). The passage
names only *necessary* and *possible*. Admitting *able* is attested (283
faces) but not stated by CGEL: the landing records it as an attestation-based
project decision carried by a declared Adjective feature, and STOPs for a
ruling if it needs more than that. *Possible* is not attested in the failing
units and is not added (pruning rule).

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Ares, God of War: "Ares attacks each combat if able."
- Jurin, Leading the Charge: "Jurin must be blocked if able."
- Tangle Angler: "{G}: Target creature blocks this creature this turn if able."
- Alluring Siren: "{T}: Target creature an opponent controls attacks you this
  turn if able."
- Hustle: "Target creature attacks or blocks this turn if able."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *able* read as a depictive or Postmodifier of the Subject,
   or *if able* attached inside an NP, is a defect. A wrong analysis that parses is a defect, not a gain.
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

## Out of scope

- Postpositive *able to* AdjPs (*All creatures with flying able to block this
  creature do so.*, Talruum Piper) and *do so*.
- *if possible*, *if necessary*: unattested in the failing units.

## Landing record

In addition to the standard record: the category and function chosen for the
verbless Complement and the declared feature licensing *able*; Reading counts
of each witness; the residual item discharged; before/after counts stamped
with change ids; timings as integer ns and ns/B with host load and worker
count.
