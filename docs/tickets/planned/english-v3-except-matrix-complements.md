---
needs: []
---
# Read the matrix-licensed Complements of *except*: can't be blocked except by Walls

## Why

*except* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *except* touches **268** unread faces and is the
sole cause on **152** (recon bucket "except"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Slivers can't be blocked by Slivers." 1; "Slivers
can't be blocked except by Slivers." 0; "Create a token that's a copy of target
creature you control." 1; "… , except it isn't legendary." 0; "Each player
sacrifices all lands they control." 1; "… except for three." 0.

## Goal

The preposition *except* heads a PP Adjunct whose Complement is the category
the matrix licenses: a PP (*except by creatures with flying or reach*), a *for*
PP (*except for three*) and a declarative content clause without *that*
(*except its name is Sunfrill Imitator and it has this ability*, *except it's
an artifact in addition to its other types*). Matrix licensing reads declared
features of the host, not of a named verb.

## Analysis

*Except* takes as its complement something licensed by features of the clause
containing it: NP, bare role NP, PP, AdjP, AdvP, infinitivals,
gerund-participial and content clauses are all attested (CGEL, Ch. 7, §5.1(g),
p. 642, [21]); CGEL calls these **matrix-licensed complements**. *Except* takes
either a *for* complement or an object (Ch. 7, §5.1(c), p. 640, [15i]). Only the PP, *for*
PP and declarative content clause are attested in the witnesses; add no other
complement category (pruning rule).

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Evil Eye of Orms-by-Gore: "This creature can't be blocked except by Walls."
- Shifting Sliver: "Slivers can't be blocked except by Slivers." (control
  without *except* 1)
- Seeker: "Enchanted creature can't be blocked except by artifact creatures
  and/or white creatures."
- Quantum Misalignment: "Create a token that's a copy of target creature you
  control, except it isn't legendary."
- Keldon Firebombers: "When this creature enters, each player sacrifices all
  lands they control except for three."

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
   Reading. The content clause read as a Postmodifier of a noun, or
   *except* attached inside the preceding NP where the matrix licenses it, is a
   defect. A wrong analysis that parses is a defect, not a gain.
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

- *except* governing a bare NP after *all … except* where no witness needs it.
- *but*, *save*, *including*.

## Landing record

In addition to the standard record: the complement categories admitted and the
declared matrix features that license each; Reading counts of each witness;
before/after counts stamped with change ids; timings as integer ns and ns/B
with host load and worker count.
