---
needs: []
---
# Read coordinated Object + goal sequences with a final Manner Complement: Put one of them into your hand and the rest on the bottom of your library in any order.

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
Object + Locative Complement sequence, and the final sequence may carry the
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
the final tail, and the GC5 Readings still present), never an incidental count
or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *the other* in the second tail (*… and the other into your graveyard*):
  `english-v3-fused-head-noun-phrases`.
- Three-way lists (*one into your hand, one into your graveyard, and one on
  the bottom of your library*) and *…, then shuffle* continuations, unless
  they fall out with no extra declaration.

## Landing record
