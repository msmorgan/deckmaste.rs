---
needs: []
---
# Read a bare relative whose gap is a predicative Complement: the controller of the permanent it becomes sacrifices it at the beginning of the next cleanup step.

## Why

Ten flash Auras share one sentence: "You may cast this spell as though it had
flash. If you cast it any time a sorcery couldn't have been cast, the
controller of the permanent it becomes sacrifices it at the beginning of the
next cleanup step." Everything in it reads except the relative clause: "If
you cast it any time a sorcery couldn't have been cast, its controller
sacrifices it at the beginning of the next cleanup step." 4 and "Its
controller sacrifices it at the beginning of the next cleanup step." 1,
against NP "the permanent it becomes" 0, NP "the controller of the permanent
it becomes" 0 and "The controller of the permanent it becomes sacrifices it
at the beginning of the next cleanup step." 0. Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:sacrifices it`
10. `cargo xtask english-v3 grep -i '\bpermanent it becomes\b'` finds **10**
faces, all unread; **10** have that line as their only failing line. No
other head + pronoun + copular verb relative is attested
(`'\b(permanent|creature|card|land|artifact) (it|they)
(becomes?|is|was|are|were)\b'`: the same 10 faces).

## Goal

A bare relative clause whose missing element is the predicative Complement of
its verb modifies a nominal: *the permanent it becomes* (it becomes
[that permanent]), with byte-exact realisation.

## Analysis

CGEL Ch. 12 §3.3(c), p. 1045, "Predicative complement": [29ib] "*Her book
displays the fine sceptical intelligence of the scholar* [*she is* —]";
"Relativisation of predicatives is comparatively rare, and almost entirely
limited to the subjective type"; "With integrated relatives it is also very
rare for the antecedent to be in other than a definite NP". Ch. 12 §3.5,
p. 1054, item (g): "Relativised element is ascriptive predicative complement: normally
non-*wh*", [66i] "*He's no longer the trustworthy friend* [(*that*) *he was
in those days*]". Those back a bare (non-*wh*) integrated relative with a
predicative gap after a definite head. Which verbs open the gap is a project
ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ZeroObjectRelativeClause` (`[subject: NominativePhrase, …, predicate:
FiniteObjectGap]`, `export RelativeGapKind = Object`) and
`AdverbialGapRelativeClause` (`RelativeGapKind = TemporalAdjunct`), consumed
by `ObjectRelativeNominal`; `core-verb:Become`'s single frame
`Predicate([PredicativeComplement])` (`lexicon/verbs.ron`), which is why no
Object gap exists for it.

Orchestrator rulings 2026-10-10:

- PG1. Refine the gap formation that feeds `ZeroObjectRelativeClause`: a
  finite predicate may leave its predicative-Complement slot as the gap, and
  the clause exports a `RelativeGapKind` value for it. No new relative
  Construction; `ObjectRelativeNominal` consumes it as it consumes the
  Object gap.
- PG2. The licence is the verb frame's declared slot (the
  `PredicativeComplement` relation), read by the gap rule; no verb is named.
  That opens the gap for *be* and *remain* as well. They are unattested in
  this shape (grep above): do not add a check whose only job is to block
  them, and report every covered face they change.
- PG3. One analysis per string. The danger is *be*: a nominal followed by
  Subject + *is / are* must not gain a relative Reading where the string is
  today a clause boundary or a predicative clause (*… each creature it is
  …*). Under PG2 any Reading-count increase on a covered face is reported
  with its shape before landing; if the increases are not confined to
  strings that genuinely have the relative structure, restrict the gap to
  the nominal-Complement realisation of the slot (a declared feature of the
  frame slot) and say so under Deviations. If that does not suffice, STOP
  with the diff.
- PG4. The antecedent's definiteness (CGEL p. 1045) is not enforced: no
  feature check to block an unattested indefinite head.
- PG5. Newly covered faces are gains: the 10 named below.

## Witnesses

All ten faces carry the same line, their only failing line in
`blockers-wave8.json` (census `ymqwzour`, 0 Readings): Armor of Thorns, Grave
Servitude, Lightning Reflexes, Mystic Veil, Necromancy, Parapet, Relic Ward,
Soar, Spider Climb, Ward of Lights. Fragment probes ran 2026-10-10 on the
`ymqwzour` binary (`--readings 60`); trunk then moved to `pxmykwnl` and the
prebuilt probe no longer loads plugin data. Re-probe on the claim parent
first.

- Soar: "You may cast this spell as though it had flash. If you cast it any
  time a sorcery couldn't have been cast, the controller of the permanent it
  becomes sacrifices it at the beginning of the next cleanup step."
- Fragment: "The controller of the permanent it becomes sacrifices it at the
  beginning of the next cleanup step." 0
- NP: "the permanent it becomes" 0; "the controller of the permanent it
  becomes" 0.

Controls that must not change: "If you cast it any time a sorcery couldn't
have been cast, its controller sacrifices it at the beginning of the next
cleanup step." 4; "Its controller sacrifices it at the beginning of the next
cleanup step." 1; Killing Wave "For each creature, its controller sacrifices
it unless they pay X life." 1 (census); NP "planeswalkers that opponent
controls" 1; Seedship Agrarian "Whenever this creature becomes tapped, create
a Lander token." 1 (census); Tyrite Sanctum "{2}, {T}: Target legendary
creature becomes a God in addition to its other types. Put a +1/+1 counter
on it." 3 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\b(it|they|he|she|you) (becomes?|is|are|was|were|remains?)\b|\bbecomes? (a|an|the)\b

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff --lines` against the claim parent
(size the selector with `cargo xtask english-v3 grep --count` and record it:
it is deliberately the whole pronoun + copular-verb population, because PG2
opens the gap for every such verb). Zero lost; PG3 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` (and `deckmaste_lexical_source` if data changes),
once, at the end; a rebuild after a grammar edit takes about 2–3 minutes, the
six test binaries about 75 seconds. Tests assert the role (the relative
clause's gap as the predicative Complement of *become*; the head nominal as
its antecedent; the PG3 negatives with *be*), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- NP "the permanent it enchants" 0 today: an Object-gap relative that fails
  for a different reason (the verb's frame); report it, do not chase it.
- *wh* and *that* relatives with a predicative gap (0 faces).
- Object-predicand predicatives (*what they consider it to be*).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
