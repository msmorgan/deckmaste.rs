---
needs: []
---
# Read Cleave's bracketed text: Destroy target [attacking] creature.

## Why

Every card with Cleave prints square brackets around the words its cleave
cost removes. No bracket reads: "Destroy target [attacking] creature." 0
against "Destroy target attacking creature." 1; "Destroy all creatures [with
mana value 2 or less]." 0 against the bracket-free sentence 2; the keyword
line "Cleave {3}{W}" 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`): failing token `[` 16 sole, 27
touched, shared with the bracketed loyalty cost of
`english-v3-loyalty-ability-nouns`. Cleave brackets: 12 faces (`cargo xtask
english-v3 grep '\[[^\]+−0-9][^\]]*\]'`), all unread, all **12** with that
line as their only failing line. What is bracketed: a premodifier
(*[attacking]*, *[Human]*, *[Wolf or Werewolf]*, *[basic land]*) 4; a
postmodifier (*[with flying]*, *[you control]*, *[with mana value 2 or
less]*, *[that wasn't cast from its owner's hand]*) 5; a whole sentence
(*[X is the number of creatures you control.]*, *[At the beginning of that
turn's end step, you lose the game.]*, *[Your maximum hand size is reduced by
three for the rest of the game.]*) 3; and one coordinate with its comma
(*[reveal it,]*, Dig Up, together with *[basic land]*).

## Goal

A bracketed span reads as the constituent it encloses, in the function that
constituent has without the brackets: a Modifier inside a noun phrase or a
Sentence of the paragraph. Byte-exact realisation, brackets included.

## Analysis

No CGEL basis: the brackets are Oracle Notation marking removable text
(glossary **Notation**, **Document**). Project ruling throughout. The grammar
reads the full text; which Reading the game uses when the cleave cost is paid
is semantics.

Nearest existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`Parenthetical` (`["(", body: Paragraph, ")"]`) with `ParentheticalItem` is
the model of a delimited constituent standing where its content could; the
premodifier and postmodifier categories (`Premodifier`, the PP and relative
postmodifiers) and `SentenceItem` / `ParagraphItem` are the hosts.
`plugins_v2/builtin/macros/keyword_abilities/cleave.ron` declares the keyword
(`FixedKeyword(surface: "cleave")`).

Orchestrator rulings 2026-10-11:

- CB1. New schema, stated up front (R-budget): one bidirectional schema
  `["[", body: node, "]"]` with category instances for the attested hosts
  (premodifier, noun postmodifier, paragraph item), buying 12 faces. It is one
  schema with instances, not one Construction per host; count it as such in
  the landing. If `Parenthetical` can be generalised over its delimiters
  instead, prefer that and say so.
- CB2. The bracketed constituent keeps every export of its body (it is
  transparent to agreement, targeting and the host's requirements), so the
  bracketed sentence has the Readings of the bracket-free sentence and no
  others (R-one: the witness reads exactly as many ways as its control).
- CB3. The licence is the host category. No test on *cleave*, and no check
  that a face has the Cleave keyword line (R-prune: that would only block an
  unattested string); the grammar does not relate the brackets to the keyword.
- CB4. A bracketed whole sentence carries its own full stop inside the
  bracket (*[X is … control.]*): it is a paragraph item whose body is a
  complete Sentence; the paragraph separator stays outside.
- CB5. *[reveal it,]* (Dig Up; 1 face): a bracketed coordinate with its
  trailing comma. In scope only if it follows from CB1 with the comma
  treated as the coordination's separator absorbed into the bracket (compare
  the absorbed comma of `english-v3-quoted-ability-closing-comma`); otherwise
  leave Dig Up unread and list it.
- CB6. Newly covered faces are gains. No covered face contains `[`, so no
  Reading-count change on a covered face is expected; any change is a defect.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Fierce Retribution: "Cleave {5}{W}" / "Destroy target [attacking]
  creature." (sentence 0; control without brackets 1)
- Path of Peril: "Cleave {4}{W}{B}" / "Destroy all creatures [with mana value
  2 or less]." (sentence 0; control document without brackets 2)
- Wash Away: "Counter target spell [that wasn't cast from its owner's
  hand]."
- Inspired Idea: "Draw three cards. [Your maximum hand size is reduced by
  three for the rest of the game.]" (document 0)
- CB5: Dig Up: "Search your library for a [basic land] card, [reveal it,] put
  it into your hand, then shuffle."

Controls that must not change: "Destroy target attacking creature." 1;
"Destroy all creatures with mana value 2 or less." 2; Document "Cleave
{3}{W}" 1; and every covered face with a `Parenthetical` if CB1 generalises
that Construction (R-reach).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\[` (28) and `(?i)cleave` (14), plus, if `Parenthetical` is
changed, every face whose analysed text contains `(`; plus 300 random covered
controls (`cargo xtask english-v3 diff --manifest … --random 300 --seed
20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; CB6 for changes. Full
`cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings`, once, at the end; a rebuild
after a grammar edit takes about 2–3 minutes, the six test binaries about 75
seconds. Tests assert the role (the bracketed span as Modifier or paragraph
item, its body the same constituent as in the control, the CB2 equality of
Reading sets modulo brackets), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Bracketed loyalty costs (*"[+1]: …"*): `english-v3-loyalty-ability-nouns`.
- Bracket contents that fail for their own reason.
- The semantics of cleave (the text with the bracketed words removed).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
