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

Orchestrator ruling 2026-10-10 (EX-R1): RETAIN both scopes. In the
Phantasmal Image and Machine God's Effigy wording, the coordinated *and it
has …* Clause may sit inside the exception's Content Clause or outside it.
Both are grammatical; the existing create-token control already retains both.
Re-spell the two new assertions as value assertions requiring both
constituencies. This resolves the `koswtsww` STOP; no admission change is
required. After full suites, clippy, the landing record, commit and refresh,
integration and workspace removal are authorized without a further STOP.
Bounded evidence is in `target/english-v3/english-v3-enter-as-copy-except-clause/`.

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

1. **Census delta** (`mqrlzszk` → refreshed `lzsrwprt`, own `koswtsww`; baseline grammar identical
   to claim parent `pmwlrxnt`): bounded selector
   `\bexcept\b|\bas a copy of\b`, 301 supported faces plus 300 covered
   controls from seed 20261008, 599 distinct faces. Covered 494 → 532;
   Readings 15,308 → 19,112. Aggregate gains 38, own gains 37;
   The Clone Saga (0 → 8) came from trunk `zunolrnv` during refresh.
   Lost none, decreases none,
   increases none, digest-only none, issues none. Evidence: `after-refresh-2.json`, `diff-refresh-2.json`,
   `diff-refresh-only.json` and `diff-refresh-2-only.json` under `target/english-v3/english-v3-enter-as-copy-except-clause/`.
2. **Witnesses** (`koswtsww`): claim-parent probes all 0; Evil Twin 6,
   Copycrook 3, Phantasmal Image 54, Machine God's Effigy 6 on its witness
   line (7 on the whole face). Finite, causative, artifact and tapped sentence
   witnesses each 0 → 3. Controls unchanged: bare/tapped causatives 5,
   finite 3, artifact create/become 4, flying create 2 and become 1,
   Evil Twin become 2. The old cap of 3 understated several controls.
3. **Spot checks** (`koswtsww`): Evil Twin, Copycrook, Phantasmal Image,
   Machine God's Effigy, Auton Soldier, Dack's Duplicate, Copy Artifact,
   Copy Land, Mercurial Pretender and Gigantoplasm each attach the exception
   PP to the lexical *enter* predicate, with a finite Content Clause inside.
   Phantasmal Image and Machine God's Effigy retain BOTH scopes: *and it
   has …* inside the exception's Content Clause, or coordinated outside it
   with the matrix Clause (orchestrator ruling EX-R1, 2026-10-10).
   No sampled wrong analysis.
4. **Deviations and STOPs** (`koswtsww`): one runtime lexical feature
   addition; no Construction or frame added/deleted, no loader or projection
   changes. Eight tests added in one suite_6 module. The whole-tail STOP was
   resolved by EX-R1; both constituencies now have value assertions, while
   independently licensed Adjunct attachments remain. No glossary gap.
   EC5's 55 broad grep matches minus 43 enter-as-copy matches are 12 cases
   whose exceptions attach to later become/create/destroy predicates, not
   non-copy *enter*. No spelling guard added. No superseded route. The first refresh refused because a sibling moved;
   retry succeeded and brought `lvpxmkkl` and `zunolrnv`. A later refresh
   brought the focus-only-object landing, with zero delta on all 599 faces.
5. **Tests** (refreshed `lzsrwprt`, own `koswtsww`): `cargo test -p deckmaste_english_v3`,
   `cargo test -p deckmaste_lexical_source`,
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`:
   all passed (812 English tests, 82 lexical-source tests).
   `cargo build -p xtask` also passed after refresh. Initial English run exposed two overrestrictive
   new assertions; EX-R1 resolved their scope requirement. Assurance counts:
   restored 0 / re-spelled 2 new assertions / ignored 0 / added 8 tests /
   removed 0. Citation audit has no new sites; checks report 0 stale and
   0 noncompliant citations.
6. **Timing** (refreshed `lzsrwprt`): the bounded census took 19.75 s
   wall time under `/usr/bin/time -v`, with 6 workers and 0 samples per face.
   No full corpus census or Lean check was run.
