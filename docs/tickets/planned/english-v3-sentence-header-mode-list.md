---
needs: []
---
# Read an unnamed Mode list after a complete Sentence line: Choose three. You may choose the same mode more than once. / • …

## Why

A Mode list reads after a dash header (*Choose one —*) and, since NM-R3, a
**named** Mode list reads after a line of complete Sentences. An unnamed list
after such a line does not: the header line reads as an ordinary Ability and
every bullet is then an orphan (`at:<start> •`). Blocker report at trunk
`qzprwuxz` (`target/english-v3/blockers-wave9.json`): `at:<start> •` touches
130 unread faces, none of them a sole-blocker face (a list has two or more
bullets), so the cause is absent from the sole-blocker tables. On **42** of
those faces every non-bullet line reads and every failing line is a bullet;
probing each bullet body alone (`--category document --as-card`), **32** of
the 42 have all bullets reading, so the list attachment is their only
blocker. `cargo xtask english-v3 grep '(?m)\.\n• '` finds 730 supported
faces, 600 covered (dash-header and named lists share the bullet shape) and
130 unread.

Header shapes among the 42: *Choose three. You may choose the same mode more
than once.* (Confluences and kin, 17); *Choose one. If this spell was kicked,
choose any number instead.* and other *… instead.* riders that already read
(6); a triggered or activated header with a rider, *{G}: Choose one. Activate
only once each turn.*, *Whenever this creature attacks or dies, choose one or
more. Each mode must target a different player.* (9); *Tiered* (7) and three
single faces, which have a second blocker (Out of scope).

## Goal

A line of complete Sentences followed by a list of unnamed Modes reads as one
Ability, the Sentence Line List, with byte-exact realisation; the header may
be an imperative paragraph, a triggered clause or the body of an activated
ability.

## Analysis

No CGEL basis: bullets and their attachment are Oracle document Notation
(glossary **Notation**, **Mode List Layout**, **Sentence Line List**). Project
ruling throughout.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`ModalItem` has two forms, `[header: Clause, " —\n", modes: Modes]` and
`[header: SentenceParagraph, "\n", modes: NamedModes]`; its instance
`(Ability, SentenceLineListAbility)` requires `modes.ModeListLayout =
SentenceHeader`, which only `NamedModes` exports (policy `NamedModeFirst`:
`first.LabelKind = FlavorWord`). `Mode` already has the unnamed form `["• ",
body: Paragraph]`. So the unnamed list exists and the sentence header exists;
the pairing is refused.

Orchestrator rulings 2026-10-11:

- SH1. Refine `ModalItem` / `Modes`; no new Construction (R-budget). Let the
  sentence-header form take the unnamed `Modes` instance as well as
  `NamedModes`, by exporting `ModeListLayout` from the header pairing rather
  than from the first bullet's `LabelKind`, or by the smallest equivalent
  change. Amend the glossary entry **Sentence Line List** (it says "A named
  Mode list") in the same landing.
- SH2. The licence reads declared features only: `AbilityTermination =
  Complete` on the header paragraph, `ModeListLayout`, `LabelKind`. No test
  on *choose*, *mode* or a card (R-class: every complete Sentence line may
  host a list; the grammar does not check that the line mentions choosing).
- SH3. The header of an activated ability (*{G}: Choose one. Activate only
  once each turn.* + bullets, Skinshifter, Aetheric Amplifier, Moxite
  Refinery, Triassic Egg, Gnostro) is in scope: the list attaches to the
  Ability whose body paragraph is the header. If `ActivatedAbility` needs a
  form for that, it is a refinement of the same pairing, not a second route.
- SH4. One analysis per string (R-one). A dash-header list keeps exactly its
  present Reading: *Choose three —* + bullets must not also read as a
  sentence header. A named list after a sentence line keeps the NM-R3
  analysis with every bullet a named Mode; it must not gain a second Reading
  in which the same bullets are unnamed Modes whose body starts with a Flavor
  Word (`FlavorWordHead`, cost 100). If both arise, the named analysis is the
  one to keep and the other is pruned by `LabelKind` agreement across the
  list (R-prune: that removes a wrong analysis, not an unattested string).
- SH5. Mixed lists (some bullets named, some not) are unattested: build no
  route for them and add no check against them beyond SH4's agreement.
- SH6. Newly covered faces are gains. The header line of each of the 42
  faces reads today as its own Ability (*Choose three. You may choose the
  same mode more than once.* 2, Fiery Confluence); after the change it is
  consumed by the list Ability, so those faces' per-line segmentation
  changes. No Reading-count increase is expected on any covered face: the
  600 covered bulleted faces must keep their counts and digests. Report any
  change by shape; a covered dash-header or named-list face that changes is a
  defect under SH4.
- SH7. A face whose header line would now read only as a stand-alone Ability
  with the bullets attached to the *previous* line is a wrong analysis
  (R-wrong); assert in tests that the list attaches to the line immediately
  above it.

## Witnesses

Whole faces, probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`, `--readings 3`): all 0. Re-probe on the claim parent first.

- Fiery Confluence: "Choose three. You may choose the same mode more than
  once." + three bullets (0; the header line alone 2; the same bullets under
  "Choose three —" 1).
- Mystic Confluence (same header, 0).
- Inscription of Abundance, lines 2–5: "Choose one. If this spell was kicked,
  choose any number instead." + three bullets (0).
- Skinshifter: "{G}: Choose one. Activate only once each turn." + three
  bullets (0; with "{G}: Choose one —" and two of the bullets 3+).
- Balor, lines 2–5: "Whenever this creature attacks or dies, choose one or
  more. Each mode must target a different player." + three bullets (0).

Controls that must not change: the "Choose three —" variant above 1;
"Choose one —" + "• Target opponent discards a card." + "• Draw a card." 1;
"Choose three. You may choose the same mode more than once." alone 2; every
covered face matching `(?m)^• ` (600), in particular the ten NM-R3 faces
(nine Sieges and Typhoid Mary, Fractured).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?m)^• ` (730; R-reach: the change alters `ModalItem`, so every
face whose Readings use it is selected) plus 300 random covered controls
(`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; SH6 for changes; report
digest-only changes by class and name every re-spelled test. Full `cargo test
-p deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests --
-D warnings`, once, at the end; a rebuild after a grammar edit takes about 2–3
minutes, the six test binaries about 75 seconds. Tests assert the role (the
sentence line as header of the list Ability, each bullet an unnamed Mode, the
SH4 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *Choose one. If …, you may choose both instead.* (21 unread faces, key
  `at:instead .`): the header fails on the fused head *both* ("Choose both."
  0), fused-head family, held. Those faces gain only when both land.
- Dash headers that fail for their own reason (*choose one that hasn't been
  chosen —*, 18 faces; key `at:— <end>`): not this ticket.
- *Tiered* lists with cost-labelled modes (*• Thunder — {0} — …*, 7 faces).
- Non-sentence bullets: *• Target artifact card.* (Call Damage Control),
  *• 3/1 Human Warrior with trample and haste.* (Outlaws' Merriment, Wild
  Shape).
- Bullets that fail for their own reason once the list attaches.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
