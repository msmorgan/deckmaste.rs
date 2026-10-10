---
needs: []
---
# Read a name with an ascriptive NP supplement: create Angelo, a legendary 1/1 green and white Dog creature token.

## Why

A token introduced by its name, with its description set off by a comma, does
not read: "Create Angelo." 1, "Create a legendary 1/1 green and white Dog
creature token named Angelo." 3, but the Oracle wording 0. Blocker report at
trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): hidden under
`at:token .` 6, `at:" <end>` 5, `at:haste .`, `at:indestructible .` and under
`word:` keys. `cargo xtask english-v3 grep "[Cc]reate [A-Z][A-Za-z' ]+, a
legendary"` finds **40** faces, all unread; 40 failing lines match, **33** of
them the only failing line of their face. On **20** of the 33 every word is
known, so the construction is the sole cause; on **13** the line also has an unknown
word, nearly always the name itself (*Voja*, *Mowu*, *Tiny*, *Ragavan*,
*Smaug*, *Avacyn* …).

## Goal

A proper-name NP followed by a comma and an indefinite NP reads as an anchor
with an ascriptive NP Supplement, as the Object of *create*, with whatever
postmodifiers the description NP has today (*with trample and haste*, *with
"…"*), byte-exact.

## Analysis

CGEL Ch. 15 §5.2(b), p. 1357, [19iib]: "*Kim Jones, a quite outstanding
student, won a scholarship to MIT*", labelled [ascriptive]; p. 1358, [22i]:
"*Her father, a die-hard conservative, refused to even consider the
proposal*", under "Ascriptive NP supplements". Those back a proper-name
anchor with an indefinite ascriptive NP supplement. P. 1353–1354: the anchor
and its supplement "form a construction – a supplementation construction".
Restricting it to the positions below is a project ruling. Glossary:
**Supplement**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`ProperNameNounPhrase` (`form [head: Name]`) reads the name; schema
`Supplementation` (`[anchor: node, ", ", supplement: PrepositionPhrase, "."]`,
instances for `Sentence`/`Clause` and `Ability`/`KeywordPhrase`) is the only
comma-supplement schema and takes a PP supplement on a clause anchor.
`NamedNominal` (`[head: Nominal, " ", modifier: NamePredicate]`) reads *token
named Angelo*.

Orchestrator rulings 2026-10-10:

- NA1. One new NP construction, or an NP instance of the supplementation
  schema if its shape allows: anchor `ProperNameNounPhrase`, `", "`,
  supplement an indefinite `NounPhrase`. The result has the anchor's number
  and is an NP wherever the name is.
- NA2. The licence reads declared features only: the anchor is a `Name`, the
  supplement's `Definiteness` is indefinite. No test on *create*, *legendary*
  or *token*. It is therefore not limited to *create*; that is accepted.
- NA3. One analysis per string. The description is a Supplement to the name.
  It must not also read as a second coordinate (asyndetic NP coordination
  *Angelo, a … token*), nor as two Objects. Where a clause-final comma
  supplement and this NP supplement could both attach, report the pair.
- NA4. A medial supplement closed by a second comma (*Kim, a legendary
  creature, attacks*) is unattested here: do not build it, and add no check to
  block it.
- NA5. Names: this ticket adds no name vocabulary. The 13 faces whose token
  name is an unknown word stay unread until the name inventory admits names
  introduced by a face's own text (`english-v3-name-lexeme-scope` is the
  precedent); list them in the landing as routed, not lost.
- NA6. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with a card name followed by a comma and an indefinite NP
  (name-initial lists); report each with its shape. None is expected from the
  40-face pattern, which has no covered face.

## Witnesses

Each is (the last clause of) the face's only failing line at `ymqwzour`, all
words known. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Rinoa Heartilly: "When Rinoa Heartilly enters, create Angelo, a legendary
  1/1 green and white Dog creature token." (sentence "Create Angelo, a
  legendary 1/1 green and white Dog creature token." 0)
- Minsc, Beloved Ranger: "Create Boo, a legendary 1/1 red Hamster creature
  token with trample and haste." (0)
- Stangg: "When Stangg enters, create Stangg Twin, a legendary 3/4 red and
  green Human Warrior creature token." (0)
- Tamiyo, Compleated Sage: "Create Tamiyo's Notebook, a legendary colorless
  Book artifact token." (0; the face's line continues *with "…" and "…"*)
- Tuktuk the Explorer: "When Tuktuk the Explorer dies, create Tuktuk the
  Returned, a legendary 5/5 colorless Goblin Golem artifact creature token."
  (not probed: the binary stopped loading plugin data when trunk moved)

Controls that must not change: "Create Angelo." 1; "Create a legendary 1/1
green and white Dog creature token named Angelo." 3; "Create a legendary 2/2
white and black Dog creature token." 1; "Create a token named Stangg Twin."
1; "Sacrifice Stangg when that token leaves the battlefield." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `[A-Z][A-Za-z' ]+, an? [a-z0-9]` (a capitalised word, comma, indefinite
article: the surface a name supplement can touch)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
NA6 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the name as anchor and head of the Object NP, the description as its
Supplement, the NA3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Token-name vocabulary (NA5), 13 sole-line faces.
- *Ragavan enters tapped and attacking.* and other later sentences that use
  the introduced name.
- Quoted abilities inside the description that fail for their own reason
  (*and "{T}: Draw a card."*).

## Landing record
