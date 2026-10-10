---
needs: []
---
# Read coordinated Object + goal sequences with selected Manner Complements: Put one of them into your hand and the rest on the bottom of your library in any order.

## Why

The 2026-10-10 triage (`docs/memory/english-v3-blocker-triage-2026-10-10.md`,
blocker report at trunk `vwlkmvns`) ranks this third, about 95 sole-blocker
faces. `cargo xtask english-v3 grep '[Pp]ut [^.]* (into|onto|on) [^.,]* and the rest (on|into|onto) '`
finds **191** faces; **130** of them continue with *in any order* / *in a
random order*. 108 faces have a single failing line of that longer shape.

The triage called it "gapped coordination + shared adjunct". Probing shows
two separate gaps, neither of them an Adjunct:

- The Right Nonce-constituent Coordination route
  (`SelectedComplementClustersPredicate` over `ObjectPrepositionTail` /
  `ObjectLocativeTail`) reads only like tails: "Put one of them into your hand
  and the rest into your graveyard." has 1 Reading through it. With unlike
  tails, "Put one of them into your hand and the rest on the bottom of your
  library." has 3 Readings and none uses the route: all three coordinate NPs
  inside the *into* PP (*into [your hand and the rest]*) and attach *on the
  bottom …* afterwards.
- *in any order* is a selected Manner Complement of *put*
  (`verbs.ron`: `[ObjectNounPhrase, LocativeComplement, in MannerComplement]`),
  as in "Put the rest on the bottom of your library in any order." (1). No
  tail has a slot for it, so the full sentence reads 0.

## Goal

Under one selected verb, an Object + marked-NP sequence coordinates with an
Object + Locative Complement sequence, and each sequence may carry the
Manner Complement its own frame selects; byte-exact realisation.

## Analysis

CGEL Ch. 15 §4.3, pp. 1341–1343: right nonce-constituent coordination; "The
coordinated sequences are required to be parallel in structure" (p. 1341);
[17ie] has three elements per coordinate; the coordination itself has "no
function" (p. 1343), only its elements do. That backs coordinating
Object + Complement sequences under one verb only.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ObjectPrepositionTail`, `ObjectLocativeTail` (Category
`SelectedComplementTail`), their Coordination instance with
`SelectedTailConcord` / `PrimitiveRightTail`, the
`SelectedComplementClustersPredicate` schema and `MannerComplement`.
`FiniteGappedClause` is the comparative-clause Complement and is not involved.

Orchestrator rulings 2026-10-10:

- GC1. Refine the existing tail route; no new coordination Construction. Find
  why unlike tails do not combine (the shared segment or the concord policy)
  and relax it by a declared property of the verb's frames: each tail must be
  a frame the head verb declares, with an Object and one goal Complement.
- GC2. The Manner Complement belongs to the final tail, as the third element
  of the frame that selects it. It is not shared and not an Adjunct. Evidence:
  *into your hand in any order* and *onto the battlefield in any order* have
  0 faces, so *put*'s marked-NP frames do not take it (pruning rule). Project
  ruling: the unequal element count is accepted because each tail is a
  complete declared frame; CGEL's parallelism passage does not cover it.
- GC3. Consistent with `ontmkkrs`
  (`docs/tickets/done/english-v3-trailing-adjunct-height.md`): nothing here
  attaches at Clause level. `ClausalPreposition` stays restricted to Clause
  Coordination and gains no host; *in any order* never reads as an Adjunct
  over the coordination.
- GC4. One analysis per string for the new part: the mixed pair reads once
  through the tail route, and the Manner Complement once, inside the final
  tail. Do not also add a clause-coordination-with-ellipsis reading (*and
  [put] the rest …*).
- GC5. The existing NP-coordination Readings (*into [your hand and the
  rest]*) are grammatical and stay (retained-Reading contract); do not prune
  them here.
- GC6. Newly covered faces are gains. Covered faces of the shorter shape
  (*… and the rest on the bottom of your library.*) will gain the tail
  Reading: report every increase with its shape before landing; any increase
  that is not exactly one added tail-route Reading per coordination is a
  defect.

- GC7 (2026-10-10; amends GC2). Manner is not restricted to the final
  tail. Each tail may carry the Manner Complement its own declared frame
  selects. Remove the left-tail restriction and its otherwise-unused feature
  plumbing. Munda, Ambush Leader, Ransack and Rowan's Grim Search attest
  non-final Locative + Manner tails. Marked-NP tails still select no Manner:
  "Put one into your hand in any order and the rest on the bottom of your
  library." remains unread. GC4's one tail-route analysis applies at either
  position; no shared Adjunct or clause-ellipsis route is licensed.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--category document --readings 3 --as-card`, whole line):
witness 0; control = the same line with *and put the rest* (≥ 3 each). Trunk
has since taken `nvotwtqp`; re-probe on the claim parent first.

- Glimpse the Cosmos: "Look at the top three cards of your library. Put one of
  them into your hand and the rest on the bottom of your library in any
  order."
- Raven Familiar: "When this creature enters, look at the top three cards of
  your library. Put one of them into your hand and the rest on the bottom of
  your library in any order."
- Tezzeret, Master of Metal: "+1: Reveal cards from the top of your library
  until you reveal an artifact card. Put that card into your hand and the rest
  on the bottom of your library in a random order."
- Evolutionary Leap: "{G}, Sacrifice a creature: Reveal cards from the top of
  your library until you reveal a creature card. Put that card into your hand
  and the rest on the bottom of your library in a random order."
- Enshrined Memories: "Reveal the top X cards of your library. Put all
  creature cards revealed this way into your hand and the rest on the bottom
  of your library in any order."

Controls (sentence): "Put one of them into your hand and the rest into your
graveyard." 1; "Put one of them into your hand and the rest on the bottom of
your library." 3 (GC6: expected to rise by one); "Put the rest on the bottom
of your library in any order." 1; "Put one of them into your hand and put the
rest on the bottom of your library in any order." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    ' and (the rest|the other|[a-z]+ of them|that card|those cards) (on|into|onto) '

(249 faces; the pattern begins and ends with a space) plus 300 random covered
controls (`cargo xtask english-v3 diff --manifest … --random 300 --seed
20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; GC6 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` (and `deckmaste_lexical_source` if a frame declaration
changes), once, at the end: a rebuild after a grammar edit takes about 2–3
minutes and the test binary about 7, so batch edits. Tests assert the role
(two tails under one verb, each goal Complement, the Manner Complement inside
each frame-licensed tail, and the GC5 Readings still present), never an incidental count
or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *the other* in the second tail (*… and the other into your graveyard*):
  `english-v3-fused-head-noun-phrases`.
- Three-way lists (*one into your hand, one into your graveyard, and one on
  the bottom of your library*) and *…, then shuffle* continuations, unless
  they fall out with no extra declaration.

## Landing record

1. **Census delta** (BEFORE `qmznkvqq`, GC7 AFTER `ypywlqqo`, final refresh):
   bounded **549** faces, covered **351 → 484**, total Readings
   **5,285 → 13,230**; gained **133**, lost **none**, decreases **0**,
   digest-only **1**, issues **0**. Pre-refresh `qmznkvqq` covered **447**
   faces with **10,150** Readings and **96** gains. The official refresh-only
   diff adds Jace, Multiverse Architect (**27**), Marina Vendrell (**8**),
   and Tom Bombadil (**30**), enabled by vocabulary change `nqnztuzo`;
   no previously covered face changes across refresh.
   Selector: ` and (the rest|the other|[a-z]+ of them|that card|those cards) (on|into|onto) `
   (**249** supported faces) plus **300** covered random controls from the
   inherited census, selected by `english-v3 diff --random 300 --seed 20261008`
   (**no overlap**). The bounded BEFORE re-measures the claim-parent grammar;
   no full corpus run. Manifest and official comparison:
   `target/english-v3/english-v3-gapped-coordination-shared-adjunct/{manifest,before,after,diff}.json`;
   refreshed evidence is `after-refresh.json`, `diff-refreshed.json`, and
   `refresh-diff.json` in that directory.
   The three covered-face increases are Cream of the Crop **45 → 48**
   (line 0, **+3**), Fertile Thicket **218 → 254** (line 1, **+36**), and
   Planar Atlas **164 → 191** (line 1, **+27**). Complete Ability probes show
   each added whole-line Reading has one tail Coordination under a single
   selected verb: an Object + Locative goal tail and a final Object + Locative
   goal + Manner tail. These
   are ambient line alternatives of the one added tail analysis per
   Coordination (GC6); incumbent NP-coordination Readings survive. The mixed
   shorter sentence control rises **3 → 4**, adding exactly that tail analysis.
   GC6 shorter shape: the mixed shorter shape (marked-NP tail + Locative tail,
   no Manner) has 0 corpus faces, so no covered face of it existed; the 45
   covered shorter-shape faces are like-tail and unchanged; the 3 increases
   are Locative + Locative+Manner faces formerly read only by NP coordination.
   GC7 amendment versus accepted `after-refresh.json`: **2** gains (Munda,
   Ambush Leader **0 → 73** face Readings; Ransack **0 → 3**), **0** lost,
   decreases, increases or digest-only changes. Own gains total **98**;
   the three vocabulary-dependent refresh gains remain attributed to `nqnztuzo`.
   Evidence: `gc7-after.json`, `gc7-diff.json`, `gc7-total-diff.json`.
   Final refresh adds only Pillage the Bog **0 → 7**, enabled by *twice the
   number of lands* from multiplier change `qoswqlns`. No previously covered
   face changes. Final totals include **4** trunk-dependent gains; own **98**.
   Evidence: `gc7-refreshed-after.json`, `gc7-refresh-diff.json`.
   The subsequent refresh adds **31** gains, all containing *the other*, from
   fused-head change `vkqxprpx`. Counterbalance **2 → 2** is digest-only:
   comparative change `wzpslwxq` changes the *same … as* representation.
   Zero lost, decreases or increases across this refresh. Final totals include
   **35** trunk-dependent gains; own **98**. Evidence: `final-after.json`,
   `final-refresh-diff.json`, `final-total-diff.json`.
2. **Witnesses** (`qmznkvqq`, complete whole-line probes before/after;
   their counts and digests are unchanged at refreshed `ksyqnpnq`):
   Glimpse the Cosmos **0 → 3**; Raven Familiar **0 → 3**; Tezzeret, Master
   of Metal **0 → 3**; Evolutionary Leap **0 → 3**; Enshrined Memories
   **0 → 6**. No witness was already covered. Their *and put the rest*
   controls before implementation have **6, 6, 6, 6, 12** Readings respectively.
   The four sentence controls before/after are **1 → 1**, **3 → 4**,
   **1 → 1**, **2 → 2** in the ticket's order. Evidence: `before-probe-*`,
   `before-control-count-*`, and `after-probe-*` under the same target directory.
   GC7 complete Document probes: Munda **0 → 72**, Ransack **0 → 3**,
   Rowan's Grim Search **0 → 0**. Rowan still needs the separate *back* slot
   absent from these tail declarations; no additional route was introduced.
   The non-final Locative + Manner sentence and marked-NP + Manner negative
   both probed **0** before amendment. Evidence: `gc7-before-*`, `gc7-after-*`.
3. **Spot checks** (`qmznkvqq`, retained unchanged at `ksyqnpnq`;
   Atla Palani additionally probed at `ksyqnpnq`): Glimpse the Cosmos; Raven Familiar;
   Tezzeret, Master of Metal; Evolutionary Leap; Enshrined Memories;
   Atla Palani, Nest Tender; Blood Price; Diabolic Vision; Drawn from Dreams;
   Dig Through Time. Each inspected Reading coordinates an Object + marked-NP
   goal tail with an Object + Locative goal + selected Manner Complement tail
   under one verb. Diabolic Vision's final goal is *on top of
   your library*; the others use *on the bottom of your library*. Atla
   Palani's first goal is *onto the battlefield*; the others use *into your
   hand*. Complete probe trees were inspected; Manner belongs inside the final tail, under
   GC2, with no clause-ellipsis route or shared Adjunct. Attestation was
   checked with `english-v3 grep`, including `manner-attestation.json`
   and `onto-attestation.json`.
4. **Deviations and STOPs**: added `ObjectLocativeMannerTail` for the complete
   third-slot frame, retaining the existing Coordination Construction and
   both existing tail routes. Added declared `SelectedTailLicence` on *put*
   and its generic frame-licensing support in the construction compiler;
   GC7 removes `MannerComplementPresent` and its concord plumbing. Each tail
   still matches a complete frame declared by the same lexical owner, and
   the overt head retains the first tail's frame identity. Other verbs keep
   structural frame concord. No frame declarations or routes were superseded
   or removed. The glossary gap **Selected Tail Licence** is filled with
   explicit project-ruling attribution (GC1–GC2, amended by GC7), not a CGEL claim about
   unequal element counts. STOPs: none. The inherited probe executable was
   incompatible with current keyword metadata; it was rebuilt before the
   successful BEFORE probes and census. Refresh initially paused for a
   concurrent edit in `english-v3-clitic-have-object-relative`, then conflicted
   at the two test-module additions in `tests/main.rs`. Harmony retained both
   registrations in `qmznkvqq`; the retry succeeded and this feature's
   ancestry is conflict-free. No loader code or Lean work.
   The compiler core reads the feature names "SelectedTailLicence" /
   "DeclaredFrames" as strings (precedent: "FrameMarker") with the licence
   test living in the english_v3 suite. GC7 lets each Locative tail carry its
   own selected Manner Complement; marked-NP tails still select none.
   GC7 refresh also conflicted at test-module registrations; Harmony retained
   multiplier, predicate-term and gapped-tail suites. Incoming grammar/data
   changes required the same bounded census and full-suite/clippy recheck.
   A second grammar-bearing refresh combined the comparative and fused-head
   landings with an outdated three-input `determiner_license` call in
   `DiscontinuousComparativeDeterminer`. Updated it to the five-input signature,
   passing `head.DeterminerRequirement` and `governor.NominalAdjunctDependent`,
   consistent with all other callers; the policy still requires `No`.
5. **Tests** (pre-refresh `qmznkvqq`, refreshed `ksyqnpnq`):
   `cargo test -p deckmaste_english_v3 -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_lexical_source`
   **passed twice**, all suites unfiltered. Pre-refresh / refreshed counts:
   English **752 / 753**; construction compiler **36 / 36**; construction
   macro integration **42 / 42** plus **2 / 2** doc tests; lexical source
   **35 / 39** unit and **41 / 41** integration tests. Zero failures or
   ignored tests. The extra tests belong to refreshed vocabulary `nqnztuzo`.
   `cargo test -p xtask english_v3` **passed** at `ksyqnpnq`: **56** tests.
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_lexical_source --tests -- -D warnings`
   **passed** at `qmznkvqq`.
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_lexical_source -p xtask --tests -- -D warnings`
   **passed** at `ksyqnpnq`. Restored **0** / re-spelled **0** / ignored **0** / added
   **4** / removed **0**; no removals to justify. New tests assert roles,
   retained complete Reading values, the lexical licence and independent
   construction roundtrip. Citation checks: no noncompliant or stale sites;
   diff audit selects no new citation sites.
   GC7 full suites passed with the same four-package test command: English
   **756**, compiler **36**, macro **42** plus **2** doc tests, lexical source
   **39** unit plus **41** integration. Two preliminary runs exposed mistakes
   in the new positive fixture: bare *one* lacks an Object analysis, and
   incumbent NP-coordination Readings must not be required to use the tail
   route. The final test checks the added route and preserves the incumbents.
   GC7 clippy passed with the same four-package clippy command above,
   `--tests -- -D warnings`.
   Final refresh full suites passed with that same four-package command:
   English **781**, compiler **36**, macro **42** plus **2** doc tests,
   lexical source **39** unit plus **43** integration; no failures or ignores.
   Final refresh `cargo test -p xtask english_v3` passed **56** tests; the filter is the
   prescribed English verification suite.
   Final refresh clippy passed with the five-package command above
   (including xtask), `--tests -- -D warnings`.
   Second grammar-bearing refresh full suites passed with the same command:
   English **790**, compiler **37**, macro **42** plus **2** doc tests,
   lexical source **39** unit plus **43** integration; no failures or ignores.
   Second refresh `cargo test -p xtask english_v3` passed **56** tests;
   the five-package clippy command (including xtask) passed with warnings denied.
   GC7: added **3** tests for the non-final Locative + Manner positive,
   marked-NP negative and attested Munda/Ransack roles. The reviewed negative
   was a probe, not an existing suite test in this workspace. The positive
   uses *one of them*: bare *one* has an independent Object blocker. Total own added **7**, restored **0**,
   re-spelled suite tests **0**, removed **0**, ignored **0**.
6. **Timing** (`ksyqnpnq`): the refreshed bounded AFTER census took **7.05 s** wall
   time under `/usr/bin/time -v`, **6** workers, **0** samples per face.
   GC7 bounded census took **7.49 s** wall time; both later refreshes **6.95 s** each. Full corpus verification belongs to the orchestrator's batch.
