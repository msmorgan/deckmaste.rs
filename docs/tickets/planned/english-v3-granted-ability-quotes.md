---
needs: []
---
# Read quoted granted abilities as complements of has, gain and with

## Why

Quoted ability text granted by *has*/*have*, *gains*/*gain*, or attached to a
token by *with* never reads. Even trivial cases fail: `It has "This token
can't block."` has zero admitted roots, while the inner `This token can't
block.` has one. Likewise `Enchanted creature has "{2}, Sacrifice this
creature: You gain 2 life."` has 0 and its inner ability has 1 (probe on
`wlvwtnppyovn`). On that change (32,828 supported faces, 13,716 covered,
19,112 unread) a quotation mark occurs in a failing unit of 1,145 unread faces
and is the only recognised cause on 396. These are surface-bucket counts, not
gain forecasts.

The machinery is half-built, in two routes. `verbs.ron` declares
`Predicate([Role("GrantedAbility")])` for Gain and Have, and the generic frame
consumer reports that shape as unsupported (the `Complement(GrantedAbility)`
row of the unsupported-inventory table in the done
`english-v3-generic-frame-consumption`). Separately, the `frame_additions`
block of `crates/deckmaste_lexical_source/lexicon/core.ron` gives Have and Gain
an `Object(QuotedText)` frame. `crates/deckmaste_english_v3/src/declarations.rs`
has `QuotedText`, `QuotedClause` and `QuotedKeyword` constructions over
`Document`, `Clause` and `KeywordPhrase`. This ticket takes over the
GrantedAbility row from `english-v3-systemic-residuals`.

## Goal

A quoted granted ability is parsed recursively as one ability line (or
document) of its own, delimited by the quotation marks, and it fills one
selected Complement of *has*/*have*/*gains*/*gain*. A quoted ability after
*with* in a token or emblem description is a Postmodifier of that Nominal.
Coordination with keyword abilities ("has reach and "…"") reuses the existing
Coordination machinery. Reconcile the two routes into one analysis: retire or
re-spell the duplicate rather than keep both, and record which survives and
why.

## Analysis

Quotation marks set off text whose wording is cited rather than freely
composed (CGEL, Ch. 20, §6, p. 1753, [1]). Cited text of this kind can be the
complement of a verb or a supplement, as with embedded direct speech and
citation (CGEL, Ch. 11, §9.2, pp. 1026–1028, [7], [14]–[15]). Its internal
form is that of an independent utterance, not a subordinate clause, which is
why the quoted ability must be parsed by the same start Category as a
top-level ability line. When the quote ends the sentence, the quote's own full
stop is kept and the matrix full stop is suppressed (CGEL, Ch. 20, §6, p. 1755,
[9i]). Oracle text follows that rule: `has "… 2 life."` with no period after
the closing mark. The grammar must realise that byte-exactly in both
roundtrip directions.

## Witnesses

- Compulsory Rest: "Enchanted creature has "{2}, Sacrifice this creature: You
  gain 2 life.""
- Carrier Thrall: "It has "Sacrifice this token: Add {C}.""
- Rain of Filth: "Until end of turn, lands you control gain "Sacrifice this
  land: Add {B}.""
- Heroes of the Revel: "When this creature enters, create a 1/1 red Satyr
  creature token with "This token can't block.""
- Web-Shooters: "Equipped creature gets +1/+1 and has reach and "Whenever this
  creature attacks, tap target creature an opponent controls.""
- Energy Flux: "All artifacts have "At the beginning of your upkeep, sacrifice
  this artifact unless you pay {2}."" (plural *have*)

Each witness's only failing unit is the one quoted. Each inner ability that was
probed separately reads.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. First find why `It has "This token can't block."` fails while both halves
   read, and record the cause before changing anything.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/granted-quotes-before.json` on the claim parent,
   stamped with its change id.
3. Write the six witnesses as tests first. Assert that the quoted span is an
   ability-line constituent with its own internal Reading.
4. Iterate on `--face-id` selectors; verify on `--all` at the end.
5. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
6. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A quoted span read as an
   opaque string, or a wrong inner analysis that starts parsing, is a defect,
   not a gain.
7. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
8. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
9. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Inner-ability failures with their own causes. Psionic Sliver's quote, for
  example, also needs Deal's ordered amount/recipient segments (still in
  `english-v3-systemic-residuals`). Count such a face as a structural gain only
  when its whole face reads.
- Emblem semantics and quoted text in reminder text.

## Landing record

In addition to the standard record: the cause found in Method step 1; which of
the GrantedAbility and QuotedText routes survives, with the re-spelled tests;
the count of quoted spans in the corpus that now have an inner Reading versus
the number whose whole face reads; timings as integer ns and ns/B with host
load and worker count.
