---
needs: []
---
# Read library positions with Look, Reveal, Exile and Put

## Why

Library-position phrases never read: *the top N cards of your library*, *the
top card of …*, *on top of …*, *on the bottom of …*, *the rest*, *in any
order*, *in a random order*. On change `wlvwtnppyovn` (32,828 supported faces,
13,716 covered, 19,112 unread) they occur in a failing unit of 1,304 unread
faces and are the only recognised cause on 298. These are surface-bucket
counts, not gain forecasts. Probes on that change ("Look at the top five cards
of your library.", "Exile the top two cards of your library.", "Put target
nonland permanent on top of its owner's library.") each have zero admitted
roots. `vocab:Adjective/Top` and `vocab:Adjective/Bottom` exist, and the
lexicon's `frame_markers` already map `in` and `order`.

This ticket takes over from `english-v3-systemic-residuals`:

- the Look row `Preposition(At), Complement(Object)` and the two Put rows
  carrying `Preposition(On), Complement(FrameComplement)` (one ending
  `Preposition(In), Complement(ArbitraryDeterminer), CommonNoun(Order)`) of the
  unsupported-inventory table in the done `english-v3-generic-frame-consumption`;
- 29 of the 42 inherited frame-coordination faces, which fail first on a
  library position: Commune with Evil, Strategic Planning, Beast Hunt, Vigean
  Intuition, Mulch, Discerning Taste, Tracker's Instincts, Murmurs from Beyond,
  Maestros Charm, Confounding Riddle, Winding Way, Sultai Soothsayer, Forbidden
  Alchemy, Taigam, Sidisi's Hand, Resentful Revelation, Scattered Thoughts,
  Tamiyo, Collector of Tales, Pieces of the Puzzle, Firja, Judge of Valor,
  Ancestral Memories, Ransack the Lab, Organ Hoarder, Borborygmos Enraged,
  Shadow Guildmage, Testament Bearer, Rakshasa's Bargain, Bitter Revelation,
  Kruphix's Insight, Glimpse the Future. Their identities and earlier evidence
  are in `/tmp/frame-coordination-inherited-reconciliation.json` if it still
  exists. Re-measure; do not depend on it.

## Goal

Library positions read as ordinary Noun Phrases and Preposition Phrases
selected by Look, Reveal, Exile, Put (and Mill/Manifest where they already
take an NP), with the existing typed frame slots reconciled rather than
bypassed. "Put … on top of / on the bottom of …" consumes Put's declared
destination slot. "in any order" / "in a random order" consumes Put's declared
order tail.

## Analysis

In *the top two cards of your library*, *top* is an attributive Modifier of the
Nominal *cards* and *of your library* is NP-internal. The ADR licenses *of*
only NP-internally, as Modifier or Complement of a noun
(`docs/decisions/english-lexical-analysis.md`, Preposition Function Licence
amendment; CGEL, Ch. 5, §14.2, p. 446, [14i]). *On top of X* is a
Prep + N + Prep + X idiom of the *in front of X* type (CGEL, Ch. 7, §3.1,
pp. 618–623; *on top of* is discussed on p. 621). Choose between the
right-branching and layered-head structures by CGEL's tests (pp. 621–623), and
do not lexicalise *on top of* as one Compound Preposition without that
evidence. *On the bottom of X* is a regular PP whose NP has the noun *bottom*
as head with an *of* Complement. *In any order* is a manner PP (CGEL, Ch. 8,
§2.1, p. 671, "NPs and PPs"); with Put it is the declared frame tail, not a
free Adjunct, because Put's frame selects it.

## Witnesses

- Commune with Evil: "Look at the top four cards of your library."
- Reckless Impulse: "Exile the top two cards of your library."
- Totally Lost: "Put target nonland permanent on top of its owner's library."
- Hide: "Put target artifact or enchantment on the bottom of its owner's
  library."
- Winding Way: "Reveal the top four cards of your library." (its other failing
  unit, "Choose creature or land.", stays out of scope)
- Psychic Surgery: "Whenever an opponent shuffles their library, you may look
  at the top two cards of that library." / "Then put the rest on top of that
  library in any order."

The first four witnesses each have only the quoted failing unit.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/library-position-before.json` on the claim
   parent, stamped with its change id.
2. Write the witnesses as tests first. Assert the slot each phrase fills, not
   just recognition.
3. Iterate on `--face-id` selectors (the witnesses and the 29 inherited faces);
   verify on `--all` at the end. Report, by name, which of the 29 now read and
   the next failing cause of each that does not.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *of your library* read as a
   clause Adjunct, or *in any order* as an Adjunct of Look, is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *from among them* and other From-source slots (deferred in
  `english-v3-systemic-residuals`; owner of re-application:
  `english-v3-selected-preposition-nominal-licensing`).
- Ordinal positions ("third from the top", Enigma Sphinx, Lost Hours) and
  "their choice of the top or bottom of their library" (Uncharted Voyage).
  Record whether they read afterwards; do not build for them.
- `, then` links (owned by `english-v3-then-sequencing`).

## Landing record

In addition to the standard record: the outcome for each of the 29 inherited
faces by name (reads, or its next cause); which Look and Put frame rows are now
consumed; timings as integer ns and ns/B with host load and worker count.
