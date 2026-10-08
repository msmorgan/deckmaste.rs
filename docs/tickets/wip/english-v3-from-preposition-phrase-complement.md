---
needs: []
---
# Read *from* with a Preposition Phrase Complement: a land card from among them

## Why

*From among* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *from among* touches **460** unread faces and
is the sole cause on **268** (recon bucket "from among"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Choose a creature card from among them." 0; "Choose
a creature card from them." 1; "Choose a creature card among them." 1; "You may
put a land card from among them into your hand." 0, while "… from them into
your hand." 1.

This ticket takes over from `english-v3-systemic-residuals` the Cavalier of
Thorns / Genesis Ultimatum item ("`among them` has one PP Reading but `from
among them` … none. Consume a declared PP argument for From; the current
PrepositionComplement inventory has only None/NounPhrase") and the remaining
From-among sources of Tracker's Instincts, Pieces of the Puzzle and Kruphix's
Insight from the library-position reconciliation.

## Goal

*from* takes a PP Complement headed by *among* (*from among them*, *from among
the cards milled this way*, *from among permanents you control*), wherever its
NP Complement already reads: as a Postmodifier of the selected NP and as the
source Complement of verbs whose frames declare one. Add a PP value to the
preposition-complement inventory and license it by a declared feature of the
governing preposition and of the complement's head; no word-named pairing.

## Analysis

A preposition can take a PP complement (*The magician emerged from behind the
curtain*; CGEL, Ch. 7, §2, p. 617). *From* takes as complement an NP or a PP
headed by a locative preposition (*from behind the wall*, *from under the
bridge*; Ch. 8, §4, p. 687). That passage's examples do not include *among*;
classifying *among* as eligible is the landing's decision, recorded through a
declared feature, not a CGEL claim.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Ostrich-Horse: "You may put a land card from among them into your hand."
- Satyr Wayfinder: "You may put a land card from among them into your hand."
- Animal Magnetism: "An opponent chooses a creature card from among them."
  (control "An opponent chooses a creature card." 1)
- Seek the Wilds: "You may reveal a creature or land card from among them and
  put it into your hand."
- Tomakul Scrapsmith: "You may put an artifact card from among the cards milled
  this way into your hand."

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
   Reading. *among them* read as a separate Adjunct beside a bare
   *from*, or the PP attached to the wrong NP when a verb frame declares the
   source, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *from exile*, *from anywhere* and the other deferred From source frames
  (from-M, from-A): owned by `english-v3-systemic-residuals` /
  `english-v3-selected-preposition-nominal-licensing` re-application.
- *Remove … counters from among artifacts you control*: count it if it reads,
  do not build a Remove frame here.

## Landing record

In addition to the standard record: the inventory value added and the declared
feature that licenses it; which hosts (NP Postmodifier, verb source frame)
read; the residual items discharged; Reading counts of each witness;
before/after counts stamped with change ids; timings as integer ns and ns/B
with host load and worker count.
