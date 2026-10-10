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
