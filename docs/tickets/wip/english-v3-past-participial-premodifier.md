---
needs: []
---
# Read a past-participial premodifier of an action verb: Return the exiled card to its owner's hand.

## Why

*the exiled card*, *triggered ability* and *the revealed cards* never read.
On census `xnkuznpo` (docs-only descendant of `vvxtllnq`; 32,828 supported
faces, 23,459 covered, 9,369 unread; wave-4 recon of 2026-10-08), the recon
bucket "past-participial premodifier, unlicensed verb" touches **348** unread
faces and is the sole cause on **213**. Counts are unread faces *touched* /
*sole* as defined in the recon; surface counts, not gain forecasts.

Determiner + participle + noun, unread faces / covered faces containing it:
*exiled* 206 / 0, *revealed* 48 / 0, *triggered* 42 / 0, *discarded* 21 / 0,
*milled* 19 / 0. The licensed participles read: *sacrificed* 36 / 71,
*chosen* 23 / 30, *tapped* 30 / 106.

Probes (admitted roots): "Return the exiled card to its owner's hand." 0,
while "Return the chosen card to its owner's hand." 1; "Copy target activated
or triggered ability you control." 0, while "Copy target activated ability
you control." 1.

## Goal

The attested past participles (*exiled*, *revealed*, *triggered*,
*discarded*, *milled*) premodify a Nominal through the existing participial
premodifier route, realised byte-exactly, with passive interpretation.

## Analysis

CGEL, Ch. 3 §1.3, p. 78, [9ii] (*a hurriedly written first draft*): the past
participle has an attributive use, almost always with passive
interpretation.

Existing route: the lexical feature `PastParticipialPremodifier: Yes`
(`core.ron:58,112,113`: sacrifice, choose, activate) is read by
`VerbalPremodifier` → `ParticipialPremodifier` (`declarations.rs:2390`).
The verbs above lack the property. Each is attested; add the property per
lexeme, and only for participles the corpus attests in premodifier position
(pruning rule). If *triggered* is not a verb Lexeme today, record how it is
owned before adding it.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Ugin's Labyrinth: "Return the exiled card to its owner's hand." (ability
  body; control with *chosen* 1)
- Florian, Voldaren Scion: "You may play the exiled card this turn." (control
  "You may play the chosen card this turn." 1)
- Isochron Scepter: "You may copy the exiled card." (ability body)
- Erratic Explosion: "Put the revealed cards on the bottom of your library in
  any order."
- The Princess Takes Flight: "Return the exiled card to the battlefield under
  its owner's control." (chapter body)

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
   Reading. A participle read as a finite verb, or with active interpretation, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *modified* (*modified creature*): no *modify* verb Lexeme; separate gap.
- Postnominal participles (*cards exiled with this creature*): already a
  separate route.
- *the exiled card's* genitive host problems, if any remain: note them.

## Landing record

The six items in CLAUDE.md. Also list each Lexeme given
`PastParticipialPremodifier` with its corpus face count.
