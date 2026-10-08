---
needs: []
---
# Read coordinated and premodified property values: base power and toughness 0/2

## Why

*Base power and toughness 4/4* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *base power* touches **224**
unread faces and is the sole cause on **178** (recon bucket "base power and
toughness"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Enchanted creature has power 0." 1; "Enchanted
creature has power 1 and toughness 2." 1; "Enchanted creature has power and
toughness 0/2." 0; "Enchanted creature has base power 0." 0; "Enchanted
creature has base power and toughness 0/2." 0; "Create a creature token with
power and toughness 2/2." 0; "Each creature with base power 2 gets +1/+1." 0;
"Its base power is 2." 1.

## Goal

The scalar property value NP takes (a) a premodified property Nominal (*base
power 0*) and (b) a coordinated property Nominal whose value is a slash pair
matching the coordinates in order (*power and toughness 0/2*, *base power and
toughness X/X*), in the hosts where *power 2* already reads (Object of *have*,
*with* PP Postmodifier, *becomes a Bird with …*). A slash pair after a single
property noun (*power 0/2*) and a single value after the coordination
(*power and toughness 2*) do not read.

## Analysis

The structure of *power 2* is the project ruling recorded in
[english-v3-scalar-property-values](../done/english-v3-scalar-property-values.md#project-ruling-scalar-property-values)
(2026-10-07): a singular scalar property noun followed by a value forms an NP
with the value as its selected Complement. That ruling and its CGEL notes say
nothing about premodified or coordinated property heads or slash-pair values;
this ticket extends the ruling and the landing records the extension as a
project decision, not a CGEL citation. Reuse the existing `SlashPair` value and
the ordinary Coordination schemas; no parallel property frame.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Reduce in Stature: "Enchanted creature has base power and toughness 0/2."
- Illusory Wrappings: "Enchanted creature has base power and toughness 0/2."
- Godhead of Awe: "Other creatures have base power and toughness 1/1."
  (control "Other creatures have power 1." 1)
- Admiral Brass, Unsinkable: "It has base power and toughness 4/4."
- Glamer Gifter: "Until end of turn, that creature has base power and toughness
  4/4 and gains all creature types."

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
   Reading. The slash pair read as a Premodifier or as an Object of
   *have* apart from the property Nominal, or *base* read as a separate NP, is a
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

- *where X is …* continuations beyond the value (Candlekeep Inspiration):
  existing variable-*where* route; measure only.
- *Switch … power and toughness*, *base power* as a Subject of *be* (already
  reads).

## Landing record

In addition to the standard record: the extension of the scalar-property-value
ruling as recorded; how the slash pair is matched to the coordinates (declared
features); Reading counts of each witness and of the two negatives; before/after
counts stamped with change ids; timings as integer ns and ns/B with host load
and worker count.
