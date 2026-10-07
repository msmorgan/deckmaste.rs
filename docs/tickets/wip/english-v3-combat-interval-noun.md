---
needs: []
---
# Declare *combat* a turn-interval noun: beginning of combat, until end of combat, each combat

## Why

Bare *combat* as a game interval never reads. On change `xxknlzypsnwy` (32,828
supported faces, 17,322 covered, 15,506 unread; recon of 2026-10-07),
*beginning/end of combat*, *during/each/this combat* touch **637** unread faces
and are the sole cause on **202** (recon bucket "at end of combat / combat
step-phase": 683 / 202; *beginning of combat* alone is 307 touched / 112 sole).
Counts are unread faces *touched* (at least one localised failing unit matches)
/ *sole* (every failing unit matches and no other recon STRONG bucket does).
They are surface counts, not gain forecasts.

Probes (admitted roots): "At the beginning of combat, draw a card." 0; "At end
of combat, draw a card." 0; "During combat, draw a card." 0; "This creature can
block an additional creature each combat." 0. The *turn* counterparts read:
"Draw a card at end of turn." 1; "Draw a card each turn." 1; "Draw a card
during each combat." 1. In `core.ron` `lexeme:CommonNoun/Turn` declares
`NominalBareClass: Interval` and `NominalAdjunctClass: Temporal`;
`lexeme:turn_part/combat` declares only `NounPremodifier: Yes`. The
constructions exist; *combat* lacks the declared class that licenses them.

## Goal

*combat* reads wherever *turn* reads as a game interval: bare in the
*of*-Complement of *beginning*/*end* (*at the beginning of combat*, *until end
of combat*), bare as the Complement of *during*, and as a determined temporal
NP Adjunct (*each combat*, *this combat*). The change is a feature declaration
on the existing lexeme, consumed by the existing routes; no new construction.
*At the beginning of combat on your turn* must keep the *on your turn* PP
attachment that the turn-based triggers already have (record its Reading
count).

## Analysis

Some singular count nouns occur without a determiner in fixed expressions or
frames, including expressions of time: *at dawn*, *by daybreak*, *before
sunrise* (CGEL, Ch. 5, §8.5(b), p. 409, [20v]). The same passage notes that in
such frames the noun is not used with its standard referential denotation.
Treating *combat* as such a noun alongside *turn* is the project's lexical
classification, not a CGEL ruling about this word. The rules name the combat
phase's first and last steps *beginning of combat* and *end of combat*
[CR#506.1]; the Oracle strings are still compositional (*the beginning of
combat*, *until end of combat*), so do not add atomic step-name lexemes that
would duplicate the compositional analysis (Method 8).

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Dire Fleet Warmonger: "At the beginning of combat on your turn, you may
  sacrifice another creature."
- Might of the Ancestors: "At the beginning of combat on your turn, target
  creature you control gets +2/+0 and gains vigilance until end of turn."
- Glyph of Destruction: "Target blocking Wall you control gets +10/+0 until end
  of combat."
- Phantom Whelp: "When this creature attacks or blocks, return it to its
  owner's hand at end of combat."
- Echo Circlet: "Equipped creature can block an additional creature each
  combat."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-combat-interval-noun-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-combat-interval-noun-after.json` on the final
   tree. Evidence lives under the workspace's ignored `target/english-v3/`,
   never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *combat* read as a Premodifier of a following noun when it is the
   head of the interval NP, or *on your turn* attached inside *combat* when it
   modifies the trigger, is a defect. A wrong analysis that parses is a defect,
   not a gain.
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
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- The *combat phase* atomic-vs-composite duplicate (Moment of Silence and three
  others): `english-v3-systemic-residuals`. Do not widen it; if this ticket's
  declaration adds a third analysis of *combat phase*, that is a defect.
- *combat damage* (premodifier use, already declared), step names (*declare
  blockers step*), *additional combat phase*.

## Landing record

In addition to the standard record: the feature declaration made and every
construction that consumes it; Reading counts of each witness, and of the four
*combat phase* faces before/after; before/after counts stamped with change ids;
timings as integer ns and ns/B with host load and worker count.
