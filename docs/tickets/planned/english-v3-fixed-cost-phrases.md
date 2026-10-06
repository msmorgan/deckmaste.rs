---
needs: []
---
# Read "mana of any color" and "As an additional cost" phrases

## Why

Two short, highly repeated phrases never read:

- Add-mana Noun Phrases with an *of*/*in* tail: "Add one mana of any color.",
  "Add two mana of any one color.", "Add two mana in any combination of
  colors.", "an additional one mana of the chosen color". On change
  `wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread) they
  occur in a failing unit of 701 unread faces and are the only recognised
  cause on 241. The exact failing unit "{T}: Add one mana of any color." occurs
  on 183 unread faces, and on 70 it is the face's only failing unit. Bare
  symbol strings already read: "{T}: Add {C}{C}.", "{T}: Add {R}{R}{R}." and
  "{T}: Add {G} or {W}." each have one admitted root, and are out of scope.
- The opener "As an additional cost to cast this spell, …": 317 unread faces
  touched, 158 sole. 308 unread faces have a failing unit beginning with that
  exact phrase. "As an additional cost to cast this spell, sacrifice a
  creature." has 0 admitted roots, while "Sacrifice a creature." has 1.

These are surface-bucket counts, not gain forecasts. (The minting brief quoted
131 and 58 for the two verbatim counts. The figures above were re-measured from
the same reconnaissance on the same change. Re-measure on the claim parent.)

## Goal

Both phrases read with structural analyses, and the mana NP is consumed by
Add's existing frame.

## Analysis

In *one mana of any color*, *of any color* is an NP-internal PP dependent of
the noun *mana*, the *a school of this type* pattern (CGEL, Ch. 5, §14.2,
p. 446, [14i]). The ADR licenses *of* only NP-internally, so this is the only
admissible attachment. *In any combination of colors* is a PP dependent of
the same Nominal. Check whether it is a Modifier or a Complement of *mana*
before declaring it. *As an additional cost to cast this spell* is a PP headed
by *as* with a predicative NP Complement, in Adjunct function, the *As
treasurer, I recommend …* pattern (CGEL, Ch. 7, §5.1, pp. 636–637, [4ii]). The
infinitival *to cast this spell* is NP-internal to *an additional cost*. The
glossary has no entry for a PP headed by *as* with a predicative Complement.
Add one with that citation if the implementation needs a term.

## Witnesses

- Glimmervoid: "{T}: Add one mana of any color."
- Implements of Sacrifice: "{1}, {T}, Sacrifice this artifact: Add two mana of
  any one color."
- Terrarion: "{2}, {T}, Sacrifice this artifact: Add two mana in any
  combination of colors."
- Altar's Reap: "As an additional cost to cast this spell, sacrifice a
  creature."
- Magmatic Insight: "As an additional cost to cast this spell, discard a land
  card."
- Lightning Axe: "As an additional cost to cast this spell, discard a card or
  pay {5}."

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/fixed-cost-phrases-before.json` on the claim
   parent, stamped with its change id.
2. Write the witnesses as tests first.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A whole-phrase opaque leaf, or
   *of any color* attached to the clause, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- "Activate only …", "Cast this spell only during …", "only if" restrictions.
  No live ticket other than `english-v3-systemic-residuals` owns them (its
  "only-if/only-during" line). They stay there.
- Cost modification ("This spell costs {1} less to cast …"; Cost's
  `ManaAmount, ComparisonDirection, ControlledCostAction` frame).
- Spend restrictions on the produced mana ("Spend this mana only …").

## Landing record

In addition to the standard record: the attachment chosen for *in any
combination of colors*, with its CGEL basis; timings as integer ns and ns/B
with host load and worker count.
