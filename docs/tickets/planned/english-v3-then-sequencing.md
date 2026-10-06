---
needs: []
---
# Read `, then` sequencing between clauses and predicates

## Why

`, then` joining two imperative or declarative clauses (or predicates sharing
one Subject) is the largest single unread surface in the supported corpus. On
change `wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread)
it occurs in a failing unit of 2,188 unread faces and is the only recognised
cause on 874 of them. These are surface-bucket counts from an unread-face
reconnaissance, not gain forecasts or cause proofs.

The minimal pair is clean: `Draw a card and discard a card.` has one admitted
root and `Draw a card. Then discard a card.` has one, but `Draw a card, then
discard a card.` has none (`cargo xtask english-v3 probe --readings 0`, same
change). `vocab:Adverb/Then` exists; the gap is the linkage, not the word.

## Goal

Admit `X, then Y` where X and Y are imperative clauses, finite clauses, or
predicates sharing a Subject or modal, including the final link of a serial
list (`A, B, then C`). Preserve every grammatical Reading and both roundtrip
laws. Sentence-initial `Then …` already reads in the probe above; the 311
unread units that open with *Then* fail on other causes (comparisons, library
positions, *where X is*), so confirm that and do not count them as this
ticket's gains.

## Analysis

*Then* in sequence sense is a connective adjunct: CGEL lists *next* and *then*
among the sequential pure connectives (CGEL, Ch. 8, §19, p. 778, [9ii]).
In `X, then Y` with no coordinator, *then* is the sole linking item between
the two units. CGEL analyses *so* and *yet* in exactly this position
([*There was a bus strike on,*][*so we had to go by taxi*]) as markers of
coordination, not asyndetic juxtaposition. They also link finite VPs
(CGEL, Ch. 15, §2.10, pp. 1319–1320, [79]–[80]); *then* behaves the same way
in Oracle text. With *and then*, the coordinator marks the coordination and
*then* is a modifier within the second Conjunct (same section, [78]). The
second unit is a Conjunct, so imperative-with-imperative and
predicate-with-predicate sharing must reuse the existing Coordination and
Shared Complement machinery rather than add a sequence-only Clause family.
Whether *then* is analysed as a Coordinator-like marker or as a connective
Adjunct heading the second Conjunct is the implementer's design decision. If
the two analyses differ in Reading count on the witnesses, STOP and report
before choosing.

## Witnesses

- Blur: "Exile target creature you control, then return that card to the
  battlefield under its owner's control."
- Obsessive Stitcher: "{T}: Draw a card, then discard a card."
- Risky Research: "Surveil 2, then draw two cards."
- Beneath the Sands: "Search your library for a basic land card, put it onto
  the battlefield tapped, then shuffle." (serial, *then* on the final link)
- Erode: "Its controller may search their library for a basic land card, put it
  onto the battlefield tapped, then shuffle." (declarative host; three
  predicates share the Subject and *may*)
- Vengeful Villagers: "Tap it, then you may sacrifice an artifact or creature."
  (second Conjunct has its own Subject and modal)

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/then-sequencing-before.json` on the claim parent;
   record no/one/multiple, covered faces and exact Readings, stamped with the
   change id.
2. Write the six witnesses as tests first. Assert the coordination structure,
   not just recognition.
3. Iterate with `--face-id` selectors (the witnesses plus a sample of the 874);
   verify on `--all` once at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A negative or wrong analysis
   that starts parsing (for example *then* read as a temporal Adjunct of the
   first clause only) is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule, `docs/decisions/english-lexical-analysis.md`).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- The 42-face frame-coordination residue in `english-v3-systemic-residuals`:
  none of those faces fails on `, then` (29 fail on library positions and are
  owned by `english-v3-library-position`; 10 fail on Deal's ordered
  amount/recipient segments). This ticket takes over none of them.
- Library positions, comparisons and *where X is* clauses inside a
  `then`-Conjunct; they are owned elsewhere.

## Landing record

In addition to the standard record: the chosen analysis of *then*, with the
Reading counts of both candidate analyses on the six witnesses; serial-list
behaviour (`A, B, then C`) shown on Beneath the Sands; corpus before/after
stamped with change ids; at least 10 newly covered faces by name with their
Reading; timings as integer ns and ns/B with host load and worker count.
