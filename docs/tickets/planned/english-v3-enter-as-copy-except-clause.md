---
needs: []
---
# Read the *except* clause after *enter as a copy of*: You may have this creature enter as a copy of any creature on the battlefield, except it has flying.

Split from `english-v3-quoted-ability-coordination`: the quotation is not the
cause. Possibly data-only (one host licence in `lexicon/core.ron`).

## Why

The copy-exception clause reads after *becomes a copy of* and *a token that's
a copy of*, and not after *enter(s) as a copy of*. Blocker report at trunk
`uoqyuxtnowpo`: 14 of the 25 sole-blocker faces under key `at:" <end>` are
this shape, but most instances carry no quotation. `cargo xtask english-v3
grep 'enters? (tapped )?as a copy of [^.]*, except'` finds **43** faces, all
unread; 43 failing lines match, **42** of them the only failing line of their
face. After *except*: *it has "…"* 9, *it's a(n) … in addition to its other
types* (alone or continued) about 20, *it has* + keyword 3, *it enters with
…* 3, others.

## Goal

The *, except* + content clause reads as a matrix-licensed Complement when
the host predicate is *enter* with its *as a copy of …* Complement, both
finite (*this creature enters as a copy of …, except …*) and under causative
*have* (*you may have this creature enter as a copy of …, except …*).

## Analysis

CGEL Ch. 7 §5.1(g), pp. 641–643 is cited at the existing schema for the
matrix licensing of the embedded PP; the comment there already records that
"nominal and copy-description exceptions use declared host licences" as a
project analysis. Nothing new is claimed from CGEL here: this ticket is a
project ruling throughout.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`LicensedMatrixComplementPredicate` (`[head: node, ", ", modifier:
PrepositionPhrase]`, `licensed_matrix_complement(head.MatrixComplementLicence,
modifier.MatrixComplementRequirement) = Yes`), features
`MatrixComplementLicence { None, Nominal, Content }` and
`MatrixComplementRequirement`, table `matrix_host_concord`.
`vocab:Preposition/Except` declares `MatrixComplementRequirement = Content`.
Hosts declaring `MatrixComplementLicence = Content` in
`crates/deckmaste_lexical_source/lexicon/core.ron`:
`lexeme:keyword_action/create`, `core-verb:Copy`, `core-verb:Become`. *enter*
declares none. The enter-as-copy Complement itself landed in
`docs/tickets/done/english-v3-enter-as-copy-complement.md`, which scoped the
except clause out ("do not change the except route here").

Orchestrator rulings 2026-10-10:

- EC1. Data first: declare the Content licence on the *enter* host and let
  `LicensedMatrixComplementPredicate` read the clause. No new Construction
  and no second except route.
- EC2. If the licence does not survive to the predicate (through the *as*
  Complement, the *tapped* depictive in *enter tapped as a copy of*, or the
  bare Complement of causative *have*), fix the projection of
  `MatrixComplementLicence` on the existing Constructions. That is within the
  ticket; a new Construction is not. STOP if projection alone cannot do it.
- EC3. The licence stays a declared host feature read by the existing table.
  No test on *copy*, *except*, the verb or a card.
- EC4. One analysis per string. The twins to avoid: (a) the except clause
  attaches once, to the *enter* predicate: not also inside the NP *any
  creature on the battlefield*, and not also to the outer *may* / *have*
  predicate; (b) *Create a token that's a copy of target creature, except it
  has flying.* keeps exactly its 2 Readings; (c) *This creature becomes a
  copy of target creature, except it has flying.* keeps exactly 1.
- EC5. The licence is a property of the verb, so *enters …, except …* without
  a copy Complement becomes licensed too. Grep it (`\benters?\b[^.]*, except`,
  55 faces, minus the 43): if unattested, that is the pruning rule at work and no check
  is added to block it; if attested, report what the clause is.
- EC6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected: 0 covered faces match the selector.
  Any increase is reported with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0.

- Evil Twin: `You may have this creature enter as a copy of any creature on
  the battlefield, except it has "{U}{B}, {T}: Destroy target creature with
  the same name as this creature."` (controls: the same line cut at the comma
  3; `This creature becomes a copy of target creature, except it has "{U}{B},
  {T}: …"` 2)
- Copycrook: `…, except it has "Whenever this creature attacks, it
  connives."`
- Phantasmal Image: `…, except it's an Illusion in addition to its other
  types and it has "When this creature becomes the target of a spell or
  ability, sacrifice it."`
- Machine God's Effigy: `You may have this artifact enter as a copy of any
  creature on the battlefield, except it's an artifact and it has "{T}: Add
  {U}."`

Sentence probes, 0 today: "You may have this creature enter as a copy of any
creature on the battlefield, except it has flying."; "… except it's an
artifact."; "This creature enters as a copy of any creature on the
battlefield, except it has flying." Controls that must not change: "You may
have this creature enter as a copy of any creature on the battlefield." 3;
"This creature enters as a copy of any creature on the battlefield." 3; "You
may have this creature enter tapped as a copy of any creature on the
battlefield." 3; "Create a token that's a copy of target creature, except
it's an artifact and it has flying." 3; "This creature becomes a copy of
target creature, except it's an artifact in addition to its other types." 3.

## Method

Standard constraints apply. Lexicon data loads at runtime: no rebuild if EC1
suffices. Bounded census, never `--all`: every supported face matching
`\bexcept\b|\bas a copy of\b` (302 faces) plus 300
random covered controls, `--workers 6 --samples-per-face 0`, compared with
`cargo xtask english-v3 diff` against the claim parent. Zero lost; EC6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`, once, at
the end (a rebuild after a grammar edit takes about 3 minutes, the test
binary about 7). Tests assert the role (the except clause is the
matrix-licensed Complement of the *enter* predicate; the EC4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only.

## Out of scope

- The content of the except clause where it fails on its own: *except it's
  7/7* reads 0 after *becomes a copy of* too; *except its name is …*; *except
  it isn't legendary, is an artifact …*; *except it's not legendary*
  (`english-v3-analytic-not-after-be`). Count such a face only when its whole
  face reads.
- *except it has this ability* / *it doesn't copy that creature's color*
  beyond what already reads after *becomes a copy of*.
- Unterminated quotations: `english-v3-quoted-ability-coordination`.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
