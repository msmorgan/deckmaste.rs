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

### NM-R1 — orchestrator ruling, 2026-10-10

ACCEPT with disclosure. Summon: Magus Sisters reads because its chapter
label `I, II, III` is taken as a Flavor Word. This is a pre-existing Saga
chapter-line defect, also present on trunk for Nightmares and Daydreams,
not a named-mode defect. Count 39 valid gains plus one face covered by a
known wrong analysis. Record both faces in `english-v3-systemic-residuals`;
chapter-label line forms remain outside this ticket. This is a project
ruling, not a CGEL analysis.

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

1. **Census delta** (`zsqyvxns` claim parent, verified identical to
   `mqrlzszkuszn` baseline on the selector; `mztlsnts` after integration refresh,
   containing `pxtwrtvu`): bounded manifest
   `target/english-v3/english-v3-named-modes/manifest.json`, all 640 supported
   ` —\n•` faces plus 300 random covered faces, seed 20261008; 938 distinct
   faces. Covered 756 → 799; counted Readings 35,683 → 36,278. The native
   `diff --lines` reports 43 admissions: **40 valid named-mode gains (39 before refresh,
   plus You Look Upon the Tarrasque jointly enabled by do-so/able-to), one face covered by a known wrong
   analysis (accepted with disclosure under NM-R1)**, Summon: Magus Sisters (three counted line Readings: two modal
   Readings mislabel the chapter, plus one Haste Reading). The other
   admissions are Blade of the Swarm (four Readings), brought by trunk's
   keyword-mention change `zvtzutzx`, and Glorious Sunrise (eight Readings),
   brought by quoted abilities `lrynurmu`. You Look Upon the Tarrasque
   gains 160 Readings only after `zzwnrkot` supplies its able-to/do-so body;
   probes select Mode → FlavorWordHead with Run and Hide / Gather Your Courage,
   retaining parsed Paragraph bodies. The do-so ticket explicitly deferred
   this named-mode face; count it as one joint valid gain, not a trunk-only
   gain. Lost faces: none; decreases,
   increases and issues: zero. Digest-only changes: 14. Two are The Sound of Drums
   (6 → 6) and Thraben Charm (14 → 14), from trunk's half-rounded change
   `soxvyspk`; the other 12 previously covered faces gain an empty optional
   negator field from `wqkrtoxr`, as that ticket's ruling expects. Native
   `integration-refresh-5-diff` reports 15 digest changes in that refresh,
   including three newly covered named-mode faces; all counts are unchanged.
   Named modes causes no digest-only change. Evidence:
   `target/english-v3/english-v3-named-modes/{before,after-integration-refresh-5,integration-final-5-diff}.json`.
   Native `integration-refresh-{2,3,4,5}-diff --lines` separates these trunk
   changes from the named-modes gains.
2. **Witnesses** (`zsqyvxns` before; `pxtwrtvu` after; complete whole-Document
   probes): Megaton's Fate 0 → 2; Dawnbringer Cleric 0 → 2; Cult of Skaro
   0 → 14; Special Move 0 → 12; Battle Menu 0 → 2. Their name-stripped
   controls remain 2, 2, 14, 12, 2. Ordinary modal controls remain one Reading
   each; ordinary Cure Wounds remains one Flavor Word-headed Ability; plain
   standalone bullet remains zero. The wrong standalone named-bullet Reading
   is retired, one → zero, under NM3: `• Cure Wounds — You gain 2 life.`
   read alone as a Document no longer reads. Its label is never `• Cure Wounds`;
   the named-mode route admits it only at Category Mode or inside a modal item.
   Previously covered faces changing count because of this retirement: zero;
   the census confirms zero increases and zero decreases.
3. **Spot checks** (`pxtwrtvu`, ten valid gains): each selects a Clause header
   and Mode → FlavorWordHead → OrdinaryAbility → Paragraph, with the label
   excluding the bullet: Megaton's Fate (Disarm / Detonate); Dawnbringer Cleric
   (Cure Wounds / Dispel Magic / Gentle Repose); Cult of Skaro (Thay / Caan /
   Sec / Jast); Special Move (Jump Kick / Dash Attack / Foot Toss); Battle Menu
   (Attack / Ability / Magic / Item); Astarion, the Decadent (Feed / Friends);
   Baleful Beholder (Antimagic Cone / Fear Ray); Choose Your Weapon (Two-Weapon
   Fighting / Archery); Daily Bugle Reporters (Puff Piece / Investigative
   Journalism); Damage Control Crew (Repair / Impound).
4. **Deviations and STOPs** (`pxtwrtvu`): no new Construction or Frame; Mode
   gains its second form using existing Ability and LabelKind. The open slot
   is implemented in `deckmaste_lexical`. **Orchestrator ruling NM-R2
   (2026-10-10):** the literal bullet checks are permitted LAYOUT handling,
   like the existing line-start and ` —` anchors; they name no lexeme,
   Construction or card. The three sites are `flavor_word_matches` in
   `crates/deckmaste_lexical/src/analysis.rs`, which skips a line-initial
   `• ` so the label starts after it; `valid_flavor_label` in that file,
   which rejects a label beginning with `•`; and the `FlavorWord` occurrence
   check in `crates/deckmaste_english_v3/src/chart.rs`, which admits an
   occurrence immediately after a line-initial `• ` as well as at line start.
   An Ability Word under a bullet would read only as a Flavor Word: the named
   form requires `LabelKind = FlavorWord`; that shape is unattested today.
   The open label slot runs to the first ` —` on the line, so a label can
   swallow a sentence; corpus grep finds zero matching faces.
   Corpus grep on `zsqyvxns`: 48 named-mode faces; zero mixed lists and zero
   Ability Word mode heads. No special blocker for those unattested shapes.
   The new test initially required every paragraph-scope alternative to keep
   the last sentence inside the final Mode; corrected it to require the exact
   full-body Reading while preserving the plain Mode's other scope Readings.
   **STOP resolved by orchestrator ruling NM-R1 (2026-10-10):** accept and
   disclose Summon: Magus Sisters as coverage through the pre-existing wrong
   chapter-label analysis; both complete whole-Document Readings mislabel
   `I, II, III` as a Flavor Word. The same defect on Nightmares and Daydreams
   is recorded with it under “Faces covered by a known wrong analysis” in
   `english-v3-systemic-residuals`. Chapter repair is deferred. No glossary gap.
5. **Tests** (`pxtwrtvu` initially; `mztlsnts` after integration refresh):
   the initial English suite failed the new scope expectation; its corrected
   full rerun passed 795 tests. Final `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` passed 853 tests,
   `cargo test -p deckmaste_lexical` passed 60 tests, and
   `cargo test -p deckmaste_lexical_source` passed 83 tests.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Every refresh bringing code/data reran build, bounded census, all three
   full suites and clippy: `lvpxmkkl` / `zunolrnv`, `zqnqzmnu` / `koswtsww`,
   `zvtzutzx`, `soxvyspk` / `lrynurmu`, `zzwnrkot`, and `wqkrtoxr`.
   Registration conflicts preserved all suites; the residual-document
   conflict preserved both entries under their correct headings. Assurance:
   restored 0 / re-spelled 0 / ignored 0 / added 6 / removed 0; no removals.
   Citation checks: zero noncompliant and zero stale; piped
   `jj --no-pager diff --git | target/debug/cargo-xtask cite audit --diff`
   selected zero changed citation sites. Logs are under the evidence directory.
6. **Timing** (`mztlsnts`): the bounded integration-refresh census took
   **30.27 s wall** under `/usr/bin/time -v`, six workers, samples per face
   zero. No full-corpus census or Lean check was run.
