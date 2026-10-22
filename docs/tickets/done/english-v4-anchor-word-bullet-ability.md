---
needs: []
---
**English v4: a bulleted `• <Anchor Word> — <Ability>` line is an Ability
paragraph, not a Mode.** Standard constraints apply. Style guide sections:
"Anchor words" and "Modal choices" (`docs/oracle-style-guide.md`).

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #28.

## What is wrong at head

`crates/deckmaste_english_v4/grammar/english.ron`:

- `:477` comment: `// A mode labelled by an Anchor Word [CR#614.12c]: *• Khans — ...*.`
- `:478` `AnchoredAbility` (`Ability → AnchorWord — Ability`) has no bullet; the
  only bullet production over `Ability` is `:489` `ModeBullet`
  (`Mode → • Ability`), and an anchored line reaches `Document` only through
  `:382` `TextMode`. So `• Khans — …` parses only as a Mode.

[CR#614.12c]: "'[Anchor word] — [ability]' means 'As long as [anchor word] was
chosen as this permanent entered the battlefield, this permanent has
[ability].'" The abilities are linked to the choice ([CR#607.2m]). Modes are
chosen as a spell is cast, an ability is activated, or a trigger is put on the
stack ([CR#700.2a,700.2b]), none of which happens here. The style guide's
"Anchor words" section says the same: "The bulleted abilities are not modes
[CR#700.2]".

## Change

- Add a paragraph-level production that reads `• <AnchorWord> — <Ability>` as
  an Ability (so it can follow the "As this permanent enters, choose X or Y."
  paragraph in `Paragraphs`).
- Stop `ModeBullet` from taking an anchored head: a bullet followed by an
  `AnchorWord` label is never a Mode. (Splitting a non-anchored Ability category
  for `ModeBullet`'s head is one way.)
- Reword the `:477` comment.
- Judgments: the Khans/Dragons and Mirran/Phyrexian bullets read as anchored
  Abilities, with exactly one Reading each, and no Mode Reading.

1. **Census delta** (mxtmsmpm): faces read 28,800 → 28,800; Readings 237,320 →
   237,320 (report byte-identical); lost faces: none.
2. **Judgment set** (mxtmsmpm): all enforced entries hold; `anchored_abilities.txt`
   adds four, three enforced and holding, the Phyrexian line pending (Stratum 7).
