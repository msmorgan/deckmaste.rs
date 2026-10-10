---
needs: []
---
# Read modal headers with a compound mode count: Choose one or both —

## Why

A modal header reads only when its count is a single cardinal or *any number*
(*Choose one —*, *Choose two —*, *Choose three —*, *Choose any number —*).
The other counts fail, and because a failed header splits the item into
header + bullets, every line of the item fails: these faces never appear as
sole-blocker faces and the ranked tables hide them (keys `at:— <end>` 83
lines, `at:<start> •` 225 lines). Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`). `cargo xtask english-v3 grep
'[Cc]hoose (one or both|one or more|up to (one|two|three|four|five|X)|one that hasn.t been chosen[a-z ]*) —'`
finds **98** faces, all unread; on **94** of them every failing line is the
header or one of its bullets. By header (unread faces / header-and-bullets
only): *one or both* 57 / 52; *one or more* 24 / 19; *one that hasn't been
chosen (this turn)* 22 / 16; *up to one / two* 8 / 7. First noted in
`english-v3-named-modes` and listed out of scope in
`english-v3-fused-cardinal-head`.

The 94 is an upper bound: a bullet that fails for its own reason is hidden
behind the header until the header reads.

## Goal

The header clause of a `ModalItem` reads with a coordinated count (*one or
both*, *one or more*), a bounded count (*up to one*) and a count with a
relative clause (*one that hasn't been chosen*), in the imperative and after a
trigger (*When this creature enters, choose one or both —*).

## Analysis

CGEL Ch. 5 §9.2 (fused-head NPs) is the general background; no passage is
relied on here beyond what `english-v3-fused-cardinal-head` already cites.
Everything below is project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`ModalItem` (`form [header: Clause, " —\n", modes: Modes]`, policy
`DashHeaderModalItem`). The header is an ordinary Clause. "Choose one." reads
1 through `core-verb:Choose` frame index 2, whose Complement is an
`Argument(Cardinal { … })`: a `Cardinal`, not an NP. "Choose any number." 1;
"Choose one or two." 1. Read 0: "Choose one or both.", "Choose both.",
"Choose one or more.", "Choose two or more.", "Choose up to one.", "Choose up
to three.", "Choose one that hasn't been chosen.", "You may choose both
instead.". Read with an overt noun: "Choose one or more creatures." 1, "Choose
up to one creature." 1, "Choose one mode that hasn't been chosen." 2.

Orchestrator rulings 2026-10-10:

- MH1. Refine the count Complement of that *choose* frame; do not touch
  `ModalItem`, `Modes` or `Mode`, and do not reopen the shelved general fused
  cardinal head (FC-R1): the licence stays the verb's declared count frame.
- MH2. The count category admits what a quantity already is elsewhere: the
  bounded quantities that `english-v3-scalar-cardinals` and
  `english-v3-bounded-quantity-mass-noun` read before a noun (*up to N*, *N or
  more*), and a coordination of cardinals in which a coordinate may be the
  Determinative *both*. Reuse those constructions' category; no parallel
  count grammar. No test on *one*, *both* or *more*.
- MH3. *one that hasn't been chosen (this turn)*: the count takes a relative
  clause only through the route "Choose one mode that hasn't been chosen."
  uses, with the head fused. If that needs the general fused head, leave this
  header out, with its 22 faces listed, rather than widen the licence.
- MH4. One analysis per string. *Choose one or both —* has one header Reading
  (a coordinated count Complement). It must not also read as *choose one* +
  coordinated clause fragment, and "Choose one or more creatures." keeps its
  NP analysis only.
- MH5. Sentence-header forms (*Choose one. If …, you may choose both
  instead.*) are out unless they follow; report them.
- MH6. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with *choose* + quantity + noun (*choose up to two target
  creatures*): none is expected, because the count frame has no noun
  position; an increase there means the count Complement and the NP Object
  both read one string, a defect under MH4.

## Witnesses

Document probes 2026-10-10, `ymqwzour` binary (`--category document --readings
3`), header + "⏎• Draw a card.⏎• Target creature gets -2/-2 until end of
turn.": witness 0; control "Choose one —" + the same bullets 1. Re-probe on
the claim parent first.

- Saheeli's Artistry, Decoy Ploy: "Choose one or both —" (0)
- Collective Brutality (after its Escalate line): "Choose one or more —" (0)
- "Choose up to one —" (0); "Choose up to two —" (0)
- "Choose one that hasn't been chosen —" (0)

Controls that must not change (same bullets): "Choose one —" 1; "Choose two
—" 1; "Choose three —" 1; "Choose any number —" 1. Sentence controls: "Choose
one." 1; "Choose one or two." 1; "Choose one of them." 1; "Choose one or more
creatures." 1; "Choose up to one creature." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `[Cc]hoose[sn]?\b` (2,116 faces; the frame's verb, any Complement: the change
refines a frame of *choose*) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; MH6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings` (and `deckmaste_lexical_source` if the frame spelling changes),
once, at the end: a rebuild after a grammar edit takes 2–3 minutes and the six
test binaries about 75 seconds. Tests assert the role (the count as the
Complement of *choose* in the header Clause of a `ModalItem`, each bullet a
Mode, the MH4 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only. Report, per header shape,
how many of the 98 faces read and what blocks the rest.

## Out of scope

- The general fused cardinal head (*Put one into your hand.* 0), shelved in
  `english-v3-fused-cardinal-head`.
- *Choose one at random —*, named modes: read today.
- *Choose both.* / *you may choose both instead* outside a header.
- Bullets that fail for their own reason once the header reads.

## Landing record

1. **Census delta** (`ztrnpkyv`; BEFORE uses claim-parent `pmwtxppo` code):
   2,396 faces selected by `[Cc]hoose[sn]?\b` (2,116), every `or both` face
   (one additional face, Dream Cache), and 300 covered controls from the
   official diff manifest with seed 20261008 (21 overlap). All controls were
   covered on the claim parent. BEFORE combines the original selector report
   and Dream Cache's separate report, both measured before implementation.
   `cargo xtask english-v3 diff --lines` reports covered 1,774 → 1,850;
   Readings 130,536 → 131,883; gained 76; lost/decreased/increased 0;
   digest-only 507 (the refined count Complement structure); unchanged 1,813;
   source changes and unmatched identities 0. No/one/multiple faces:
   622/456/1,318 → 546/498/1,352; undetermined and issues 0. Lost faces: none.
   No wrong-analysis gain observed. Evidence:
   `target/english-v3/english-v3-modal-header-quantities/{selector,before-combined,after,diff}.json`.

2. **Witnesses** (`ztrnpkyv`, complete probe enumeration): with the ticket's
   two control bullets, *Choose one or both —* (Saheeli's Artistry / Decoy
   Ploy), *Choose one or more —* (Collective Brutality), *Choose up to one —*,
   and *Choose up to two —* each 0 → 1; the triggered *When this creature
   enters, choose one or both —* 0 → 1. *Choose one that hasn't been chosen —*
   remains 0 under MH3. Whole-face census counts: Saheeli's Artistry 16,
   Decoy Ploy 1, Collective Brutality 8. Header controls *one*, *two*, *three*,
   *any number* remain 1. Sentence controls *Choose one.*, *Choose one or
   two.*, *Choose one of them.*, *Choose one or more creatures.*, and
   *Choose up to one creature.* remain 1; the overt *one mode that hasn't
   been chosen* remains 2. *Choose any number* retains its NP Object.
   *Choose both.* and *You may choose both instead.* remain 0; *Choose two
   or more.* and *Choose up to three.* follow the refined frame, 0 → 1.
   `before-probes.json` and `after-probes.json` retain every probe.

3. **Spot checks** (`ztrnpkyv`): Decoy Ploy, Saheeli's Artistry, Aid the
   Fallen, and Crush Contraband select `CardinalDeterminativeCoordination`
   (*one or both*); Collective Brutality and Collective Effort select
   `ComparativeQuantityDeterminer` (*one or more*); Sawblade Slinger, Ertai
   Resurrected, and Dreamshackle Geist select `PrepositionDeterminer`
   (*up to one*); The Ruinous Wrecking Crew selects `PrepositionDeterminer`
   (*up to X*). Each is the declared count Complement of *choose* inside
   the `ModalItem` header Clause, with the bullets retained as Modes.
   Ten complete-face probes inspected the intended analysis, with no issues;
   evidence is `spot-probes.json`.

4. **Deviations and STOPs** (`ztrnpkyv`): one mixed Cardinal Numeral /
   Determinative Coordination added under MH2, using the existing
   `QuantitativeDeterminer` category and the declared **Quantitative Conjunct
   Use** licence, added to the Oracle English glossary as a project term.
   The Cardinal count frame and shared-complement schema are retired on both
   lexicon and grammar sides and replaced with their Quantitative Determiner
   equivalents. No changes to `ModalItem`, `Modes`, or `Mode`; no loader code
   or general fused cardinal head. MH3 leaves the relative header deferred:
   current supported grep finds 18 faces, rather than the drafting count 22:
   Breeches, Eager Pillager; Captive Audience; Demonic Pact; Gala Greeters;
   Galadriel, Light of Valinor; Gandalf the Grey; Gollum, Riddle Master;
   Henrika Domnathi; Kargan Intimidator; Kimoyo Beads; Monument to Endurance;
   Parapet Thrasher; Silent Hallcreeper; Sol'Kanar the Tainted; Survivor's
   Med Kit; Teval's Judgment; The Vision; Three Bowls of Porridge.
   Of the 98 target faces, *one or both* reads 51/53, *one or more* 18/19,
   bounded counts 7/8, and relative counts 0/18. The four other residuals
   are Biblioplex Tomekeeper (unknown *unprepared*), Deadly Complication
   (unknown *longer*), Collective Defiance (its discard/draw bullet fails as
   a Paragraph), and Expose the Culprit (its exile/shuffle/cloak bullet fails
   as a Paragraph; the turn-face-up bullet reads). The 26 standalone-*both*
   faces remain unread under MH5; `sentence-both.txt` lists them. The binding
   user seed 20261008 overrides the ticket's Method seed. No STOP required.

5. **Tests** (`ztrnpkyv`): `cargo test -p deckmaste_english_v3` passed
   in full on the rerun; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed; `cargo test -p deckmaste_lexical_source` passed in full;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Added 3; re-spelled 2
   (`independent_shared_cardinal_and_infinitive_complements_preserve_selected_categories`
   and `independent_atomic_heads_preserve_every_exact_selected_signature`),
   retaining their exact surfaces and complete value comparisons against
   the replacement signature. Restored 0; ignored 0; removed 0.
   The first full English run exposed the retired atomic-head fixture and
   the new test's incorrect assumption that *any number* was a count
   Complement; both were corrected without deleting a subject or weakening
   its outcome, then the full suite was rerun. Citation checks: 0
   noncompliant, 0 stale; diff audit selected no new CR citations.

6. **Timing** (`ztrnpkyv`): the complete bounded AFTER census took **4m37.43s**
   wall time under `/usr/bin/time -v`, with **6 workers** and
   `--samples-per-face 0`. No full-corpus census or Lean check was run.
