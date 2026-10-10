---
needs: []
---
# Read a mode that carries a name: Choose one —⏎• Disarm — Destroy target artifact.

## Why

A modal item reads when its bullets are plain; it reads nowhere when a bullet
opens with an italic name and an em dash. Blocker report at trunk
`uoqyuxtnowpo` (`target/english-v3/blockers-wave7.json`): key `at:— <end>`
has 44 sole-blocker faces, and **all 44** are named-mode faces whose only
failing line is the header (*Choose one —*, *When this creature enters,
choose one —*). `cargo xtask english-v3 grep '—\n• [^\n—.]{1,40} — '` finds
**48** faces, all unread; 44 have exactly one failing line.

The header is not the gap. The per-line report reads each bullet alone, and a
named bullet reads alone through the wrong route (below), so the header is
the only line left failing. Header shapes on the 48 faces: *choose one —* 39,
*choose one at random —* 2, *choose two —* 1 (these read with plain bullets);
*one that hasn't been chosen (this turn)* 3, *one or more* 1, *one or both* 1,
*up to that many* 1 (these do not: see Out of scope).

## Goal

A Mode may open with a name: `• <name> — <body>`. The name is a Flavor Word
label on that mode; the body is whatever a plain mode's body is. Byte-exact
realisation, including the bullet, the name and the em dash.

## Analysis

No CGEL citation applies: bullets and italic names are Oracle layout.
Everything below is a project ruling. Glossary
(`docs/contexts/oracle-english/CONTEXT.md`): **Italic Head**, **Flavor Word**
("an open slot whose label is the italic run captured verbatim").

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ModalItem` (`[header: Clause, " —\n", modes: Modes]`), `Modes`,
`ModeContinuation`, `Mode` (`["• ", body: Paragraph]`), and `FlavorWordHead`
(`[head: lexical(FlavorWord), " ", body: Ability]`, `cost 100`, `boundary
Initial`, exports `LabelKind = FlavorWord`). A Mode's body is a Paragraph, so
a labelled Ability cannot sit under a bullet.

Defect found while probing: Document "• Cure Wounds — You gain 2 life." reads
**1**, as `FlavorWordHead` whose label is `"• Cure Wounds"`: the Flavor Word
slot swallows the bullet. Plain "• You gain 2 life." reads 0 as a Document, as
it should. That wrong Reading is why the blocker report shows named bullets as
reading lines.

Orchestrator rulings 2026-10-10:

- NM1. Refine `Mode`: a second form whose body is headed by a Flavor Word
  label. No new modal Construction, no second `ModalItem`.
- NM2. The licence is the existing lexical Category `FlavorWord` and the
  existing `LabelKind` feature. No list of mode names, no test on the name's
  words, no card test.
- NM3. The label is the name alone (*Cure Wounds*), never `• Cure Wounds`. A
  Flavor Word slot must not start at a bullet. After this ticket Document
  "• Cure Wounds — You gain 2 life." reads 0, like the plain bullet line:
  that is a wrong analysis retired, not a loss. No covered face contains a
  named bullet (grep above, 0 covered), so no face may lose a Reading for it.
- NM4. One analysis per string. The twins to avoid: (a) a named mode must not
  also read as a plain Mode whose Paragraph begins with the name (a Sentence
  *Cure Wounds — You gain 2 life.*); (b) a mode whose body opens with an
  Ability Word (*• Landfall — …*, if attested: count it) keeps the Ability
  Word analysis and gains no Flavor Word twin; (c) the header stays a Clause:
  no Flavor Word reading of *Choose one*.
- NM5. Mixed lists (named and plain bullets in one item) are licensed only if
  attested; record the count. Do not add a check whose only job is to block
  them.
- NM6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected: the 458 covered ` —\n•` faces have no
  named bullet. Any increase is reported with its shape before landing.

## Witnesses

Whole-face Documents, probed 2026-10-10 on the `uoqyuxtnowpo` binary
(`--readings 3 --as-card`); each is 0, and the same text with the names
deleted is the control.

- Megaton's Fate: "Choose one —⏎• Disarm — Destroy target artifact. Create
  four Treasure tokens.⏎• Detonate — …" 0 (names deleted: 2)
- Dawnbringer Cleric: "When this creature enters, choose one —⏎• Cure Wounds
  — You gain 2 life.⏎• Dispel Magic — …⏎• Gentle Repose — …" 0 (2)
- Cult of Skaro: "Whenever Cult of Skaro attacks, choose one at random —⏎•
  Thay — …" (four modes) 0 (3)
- Special Move: "Choose two —⏎• Jump Kick — …⏎• Dash Attack — …⏎• Foot Toss
  — …" 0 (3)
- Battle Menu: "Choose one —⏎• Attack — …⏎• Ability — …⏎• Magic — …⏎• Item —
  …" 0 (2)

Controls that must not change: Document "Choose one —⏎• Destroy target
artifact.⏎• Destroy target enchantment." 1; "When this creature enters,
choose one —⏎• You gain 2 life.⏎• Destroy target enchantment." 1; "Cure
Wounds — You gain 2 life." 1 (Flavor Word head of an ordinary ability);
"• You gain 2 life." 0.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching ` —\n•` (640 faces: 182 unread, 458 covered) plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; NM6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` (and `deckmaste_lexical_source` if the
Flavor Word slot is fixed there), once, at the end: a rebuild after a grammar
edit takes about 3 minutes and the test binary about 7. Tests assert the
role (Mode, its label, its body; the label text excludes the bullet; the NM4
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- Headers that fail with plain bullets too. These hide in multi-line
  failures (the header and every bullet fail together), so the sole-blocker
  table undercounts them; unread faces by grep: *Choose one or both —* **53**,
  *one or more —* 19, *that hasn't been chosen (this turn) —* 18, *up to one
  —* 7. Sentence "Choose one or both." / "Choose one or more." / "Choose up
  to one." read 0 while "Choose one." / "Choose two." read 1 (a Cardinal frame
  of *choose*). Own ticket; `english-v3-fused-cardinal-head` may change the
  route.
- *Choose a Background* (key `at:background <end>`, 26 sole-blocker faces, 32
  lines): a keyword line with no full stop, unrelated to modes.
- Inner failures of a mode body: The Spear of Leonidas (*Phobos*), You Look
  Upon the Tarrasque (*able to block … do so*).
- Hawkeye, Master Marksman and Summon: Magus Sisters: the header carries its
  own blocker (*up to that many*, a chapter label before *Choose one at
  random —*).

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
