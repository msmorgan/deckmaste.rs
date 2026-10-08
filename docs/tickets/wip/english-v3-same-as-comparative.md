---
needs: []
---
# Read the *same … as* equality comparative: all other lands with the same name as that land

## Why

*the same N as NP* never reads. On census `xnkuznpo` (docs-only descendant of
`vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369 unread; wave-4
recon of 2026-10-08), the recon bucket "*same … as*" touches **110** unread
faces and is the sole cause on **75**. It is also one of the two
prerequisites of `english-v3-clause-final-predicative-as` (orchestrator
ruling 2026-10-07). Counts are unread faces *touched* / *sole* as defined in
the recon; surface counts, not gain forecasts.

Probes (admitted roots): "Destroy target land and all other lands with the
same name as that land." 0, while "Destroy target land and all other lands
with that name." 1; "Your opponents can't cast spells with the same name as a
card exiled with Circu." 0, while "Your opponents can't cast spells with that
name." 1.

## Goal

*same* governs an *as* + NP comparative Complement inside the NP it
premodifies (*the same name as that land*, *the same mana value as that
spell*), realised byte-exactly. The *as* phrase is the Complement licensed by
*same*, not an Adjunct.

## Analysis

CGEL, Ch. 13 §1.1, p. 1100: *as* is the main marker of equality comparison,
scalar or non-scalar (*the same bus as last time*). Ch. 13 §5.1, p. 1139
("*Same as* with NP complement"): the comparative complement commonly
consists of *as* + NP. The orchestrator resolution of 2026-10-06 (fixed-cost
phrases) assigns this Complement to *same*'s licence, not to an Adjunct
licence.

Existing route: `Same` is a member of `vocab AttributiveAdjective`
(`vocabulary.rs:113`). The comparative-Complement route already reads
*equal to*, *less than*, *other than* through `ComparativeGovernorHead` and
`NominalComparativeComplement`; `vocab:Preposition/As` declares
`ExpandedComparisonMarker: Equality` (`core.ron:181`). This ticket gives
*same* the equality governor property that route reads.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Wake of Destruction: "Destroy target land and all other lands with the same
  name as that land." (control "… with that name." 1)
- Sever the Bloodline: "Exile target creature and all other creatures with the
  same name as that creature." (control "… with that name." 1)
- Echoing Truth: "Return target nonland permanent and all other permanents
  with the same name as that permanent to their owners' hands."
- Circu, Dimir Lobotomist: "Your opponents can't cast spells with the same name
  as a card exiled with Circu." (control "… with that name." 1)
- Extirpate: "Search its owner's graveyard, hand, and library for all cards
  with the same name as that card and exile them."

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
   Reading. An *as* phrase read as a clause Adjunct, or one *same … as* string read both as Complement and as Adjunct, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *the same* without *as* (*with the same name*, *the same is true for*):
  separate residual (86 touched).
- Clause-final predicative *as*: `english-v3-clause-final-predicative-as`.

## Landing record

The six items in CLAUDE.md.
