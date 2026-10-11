---
needs: []
---
# Read a quoted ability that closes with a comma inside the quotation mark: has "Whenever this creature attacks, you gain 1 life," and is a Cleric

## Why

A quoted ability in the middle of a sentence reads when it closes without
punctuation (*with "Jumblebones can't block" and "…"*; `[^.,]" (and|where|
until)` 44 covered faces) and when it closes the sentence with its own full
stop. When a comma follows it, Oracle text prints the comma inside the
closing quotation mark, and that does not read: 'Equipped creature gets
+1/+1, has "Whenever this creature attacks, you gain 1 life," and is a Cleric
in addition to its other types.' 0 against the same sentence with *has
vigilance,* 3+. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`): two thirds of failing token `,` (33
sole, 34 touched; keys `at:life ,`, `at:color ,`, `at:block ,`, …). `cargo
xtask english-v3 grep ',"'` finds **22** faces (23 matches), all unread; all
**22** have that line as their only failing line. What follows the closing
mark: *and* 15 (a further coordinate: *and is a Cleric*, *and it loses all
other abilities*, *and "…"*), *where X is …* 5 (a supplement), another list
item 2 (*," equip Pirate {1}, and equip {3}*, *," "Whenever …"*).

## Goal

A quoted ability may close with `,"` wherever the sentence would have a comma
after an unquoted constituent in the same position: a non-final coordinate of
a comma-separated coordination, and the anchor of a *where* supplement.
Byte-exact realisation.

## Analysis

CGEL Ch. 20 §6, p. 1755, "(d) Relative order of comma or full stop and
closing quotation mark": "AmE has a rule that when a comma or full stop is
adjacent to a closing quotation mark the latter must follow, irrespective of
the relative semantic scope." That backs the comma belonging to the enclosing
sentence although it is printed inside the mark. How the grammar represents
that displacement is a project ruling. Glossary: **Notation**, **Supplement**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`QuotedClause` (`["\"", clause: Ability, "\""]`, `require
clause.AbilityTermination = Unterminated`, `clause.terminal_punctuation =
None`) reads the punctuation-free medial quote; `QuotedText` (`["\"", text:
Document, "\""]`) the quote with its own full stop; the feature
`terminal_punctuation { None, QuotedFullStop }` with the separators
`sentence_terminal` and `keyword_cost_terminal` already model a full stop
absorbed into a sentence-final quote; `QuotedGrantedAbility`,
`GrantedAbilityCoordination` and the coordination schemas place the quote;
schema `Supplementation` (`[anchor, ", ", supplement, "."]`, `require
anchor.terminal_punctuation = None`).

Orchestrator rulings 2026-10-11:

- QC1. Refine, no new Construction (R-budget). Follow the `QuotedFullStop`
  precedent: the quote exports a declared punctuation value saying that it has
  absorbed a comma (a new value of `terminal_punctuation` or a sibling
  feature), and each separator that would print `", "` after the quote prints
  `" "` instead when it reads that value. The sites are the coordination
  schemas' comma separators and `Supplementation`'s `", "`.
- QC2. The licence is the declared punctuation feature. No rule names a
  verb (*has*, *gains*) or a quote's content. Because the separators are
  shared, every coordination and supplement that can contain a quoted ability
  gets the behaviour (R-class), not only *has "…," and*.
- QC3. One analysis per string (R-one). A quote closes in exactly one way:
  `,"` is the absorbed comma of the enclosing structure, never a comma
  belonging to the quoted ability and never a quote with no terminal followed
  by a missing separator. The quoted text itself reads as the same Ability it
  is without the comma (unterminated).
- QC4. Do not admit `,"` where no comma would stand (R-prune applies the
  other way here: a `,"` followed by nothing that takes a comma is
  ungrammatical punctuation, so the absorbed value must be *required* by a
  comma site, not optional everywhere).
- QC5. A quoted ability ending in its own full stop before a comma
  (`."` + `,`) is unattested: no work.
- QC6. Newly covered faces are gains. No covered face contains `,"` (grep: 0
  covered), so no Reading-count increase on a covered face is expected. The
  1,074 faces containing `,"` or `."` are the control class: their counts and
  digests must not move unless QC1 adds a field to a shared separator, in
  which case report the digest-only changes by class (R-reach).

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- White Mage's Staff: 'Equipped creature gets +1/+1, has "Whenever this
  creature attacks, you gain 1 life," and is a Cleric in addition to its other
  types.' (sentence 0)
- Swashbuckler's Whip: 'Equipped creature has reach, "{2}, {T}: Tap target
  artifact or creature," and "{8}, {T}: Discover 10."' (sentence 0)
- Tangled Colony: 'When this creature dies, create X 1/1 black Rat creature
  tokens with "This token can't block," where X is the amount of damage dealt
  to it this turn.' (0 without the trigger; with *flying* in place of the
  quote 3+)
- Nahiri, the Lithomancer: 'It has indestructible, "Equipped creature gets
  +5/+5 and has double strike," and equip {0}.' (sentence 0)
- Minimus Containment: 'Enchanted permanent is a Treasure artifact with "{T},
  Sacrifice this artifact: Add one mana of any color," and it loses all other
  abilities.'

Controls that must not change: 'Equipped creature has "Whenever this creature
attacks, you gain 1 life."' 1; 'Equipped creature has reach and "{8}, {T}:
Discover 10."' 1; 'Create X 1/1 black Rat creature tokens with "This token
can't block."' 3+; "Equipped creature gets +1/+1, has vigilance, and is a
Cleric in addition to its other types." 3+ (re-probe the last two with a
higher `--readings` and record exact counts).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `"` (R-reach: QC1 alters shared separators and the quote's
exports, so every face with a quotation is selected; `,"|\."` alone is 1,074
faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent.
Zero lost; QC6 for changes; name every re-spelled test. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings`, once, at the end; a rebuild after a grammar edit takes about 2–3
minutes, the six test binaries about 75 seconds. Tests assert the role (the
quote as a coordinate or supplement anchor, the absorbed comma as that
structure's separator, the QC3–QC4 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Quoted abilities whose content fails for its own reason.
- *If it doesn't have "…," it gains that ability.* (Musician) if the preposed
  *if* clause's comma is not one of the QC1 sites: report it.
- Nested single-quoted abilities.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
