---
needs: []
---
# Read *rather than* with matrix-matched Complements: rather than pay this spell's mana cost

## Why

*rather than* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *rather than* touches **189** unread faces
and is the sole cause on **125** (recon bucket "rather than"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may pay {W}{U}{B}{R}{G}." 1; "You may pay
{W}{U}{B}{R}{G} rather than pay this spell's mana cost." 0; "Draw a card
rather than gain 1 life." 0; "It assigns combat damage equal to its
toughness." 1; "… equal to its toughness rather than its power." 0.

## Goal

*rather* governs a *than* Complement whose form matches the compared matrix
constituent: a bare infinitival after a bare-infinitival VP (*may pay {U}
rather than pay this spell's mana cost*), a gerund-participial after a gerund
(*by paying life … rather than paying its mana cost*), and an NP after an NP
(*equal to its toughness rather than its power*). The matching is read from
declared features of the two constituents.

## Analysis

*Rather* is a comparative governor taking *than* (CGEL, Ch. 13, §1, p. 1104,
[15iii]). An infinitival is permitted after *than* when the matrix licenses a
complement of the same form (*I'd rather stay at home than go out in this
weather*; Ch. 14, §8.4(b), p. 1263, [27i]). The NP case and the gerund case
are attested; their structure beyond what those passages say is for the
landing to record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Bringer of the White Dawn: "You may pay {W}{U}{B}{R}{G} rather than pay this
  spell's mana cost." (control without the *than* phrase 1)
- Lethargy Trap: "If three or more creatures are attacking, you may pay {U}
  rather than pay this spell's mana cost."
- Abolish: "You may discard a Plains card rather than pay this spell's mana
  cost."
- Gauntlets of Light: "Enchanted creature gets +0/+2 and assigns combat damage
  equal to its toughness rather than its power."
- Access Maze: "Once during each of your turns, you may cast a spell from your
  hand by paying life equal to its mana value rather than paying its mana
  cost."

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
   Reading. A Complement whose form does not match the compared
   constituent, or *than* read as the comparative of an adjective, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *would rather*, *prefer … to*; unattested.
- *Once during each of your turns* if it fails on its own: note it.

## Landing record

In addition to the standard record: the structure of *rather than* (governor and
Complement) and the declared features that match Complement form to the matrix;
Reading counts of each witness; before/after counts stamped with change ids;
timings as integer ns and ns/B with host load and worker count.
