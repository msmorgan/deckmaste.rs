---
needs: []
---
# Read a comparative Determiner with its *than* Complement after the head: An opponent controls more lands than you.

## Why

*more* / *fewer* / *less* read only inside a Comparative Quantity (*two or
more*, *more than two*). As a Determiner they read nowhere: even NP *more
lands* has 0 Readings. The 2026-10-10 triage
(`docs/memory/english-v3-blocker-triage-2026-10-10.md`, blocker report at
trunk `vwlkmvns`) ranks this fifth, about 75 sole-blocker faces.
`cargo xtask english-v3 grep '\b(more|fewer|less) [a-z/+0-9 ]{1,40} than\b'`
finds **99** faces (*more* 89, *fewer* 9, *less life than* 2); 80 have a
single failing line of that shape. Commonest: *more lands than you* 25,
*more cards in hand than you / each opponent* 15, *more life than …* 17,
*more creatures than you* 6.

## Goal

A comparative Determinative reads as Determiner of a Nominal whose *than*
Comparative Complement follows the head (*more lands than you*, *more life
than an opponent*, *fewer creatures than each opponent*, *less life than an
opponent*, *more cards in hand than each opponent*), with byte-exact
realisation.

## Analysis

CGEL Ch. 5 §7.11, p. 393, [64ii, v]: "*He made* [*more mistakes than you*]",
"*He has got* [*less money than you*]"; "***Many*** and ***few*** select count
plural heads, ***much*** and ***little*** non-count singulars". P. 395: "The
determinatives *more*, *fewer*, and *less* likewise occur in determiner
function, marking the NP as indefinite". Ch. 13 §2.2, p. 1113: a single
element after *than* is the most frequent type, "but the analysis of this
element is problematic" (reduced clause or immediate Complement). Those back
the Determiner function, the head selection and the existence of both
analyses; the choice between them is a project ruling (CQ3).

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`DiscontinuousComparativeNominal` (`[governor: lexical(Adjective), head:
Nominal, marker: lexical(Preposition), complement: AccusativePhrase]`,
`governor.frame = DiscontinuousEquality`, `marker.ExpandedComparisonMarker =
Equality`) reads *the same name as that land*. `ComparativeDeterminativePhrase`
reads *more than one*; `NominalComparativeComplement` is the NP Complement of
a comparative governor; `FiniteGappedClause` is the finite comparative-clause
Complement. *more* is `vocab:ComparativeQuantifier/More`, *fewer*
`vocab:ComparativeQuantifier/Fewer`, *less* `vocab:Determinative/Less`.

Orchestrator rulings 2026-10-10:

- CQ1. Refine `DiscontinuousComparativeNominal` (a second governor Category
  and an inequality marker value on the same Construction, or a schema over
  it); no parallel Construction for the *than* case. The governor is the
  Determinative lexeme; do not mint an Adjective homograph of *more*.
- CQ2. Licences are declared features: the governor declares its comparative
  Determiner use and the head it selects (count plural for *more* / *fewer*,
  non-count for *more* / *less*); *than* declares the inequality marker value
  the Construction reads. No word tests.
- CQ3. The Complement is the immediate NP Complement of *than* (the existing
  comparative-Complement licence), accusative where case shows. No reduced
  clause is built for a bare NP.
- CQ4. The Determiner function is supplied by the governor: the resulting NP
  takes no outer Determiner and is indefinite. Agreement follows the head
  (*more lands* plural, *more life* singular).
- CQ5. Pruning rule: bare *more N* with no *than* Complement is not licensed
  by this ticket unless grep attests it outside *one or more* / *N more*;
  record the count either way. Do not add a check whose only job is to block
  it.
- CQ6. One analysis per string. *more than two lands* keeps exactly its
  Readings and *more lands than you* must not also read as *more* + [*lands
  than you*] with a postmodifier, nor with *than you* as a clause Adjunct.
- CQ7. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is reported with its shape before landing.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--readings 3 --as-card`): witness 0, control as shown.
Trunk has since taken `nvotwtqp`; re-probe on the claim parent first.

- Sunstar Expansionist: "When this creature enters, if an opponent controls
  more lands than you, create a Lander token." (control "… controls two or
  more lands, …" 1)
- Okina Nightwatch: "As long as you have more cards in hand than each
  opponent, this creature gets +3/+3." (control "… have seven cards in hand,
  …" 2)
- Unified Will: "Counter target spell if you control more creatures than that
  spell's controller." (control "… control two or more creatures." 1)
- Even the Odds: "Cast this spell only if you control fewer creatures than
  each opponent." (control "… control two or more creatures." 1)
- Survival Cache (document): "You gain 2 life. Then if you have more life than
  an opponent, draw a card." (control "… if you have 5 life, …" 1)

Also 0 today: "You have less life than an opponent." (attested on 2 faces).
Controls that must not change: "An opponent controls more than two lands." 1;
"Its power is greater than its toughness." 1; "You control a creature with
power greater than this creature's power." 3; NP "a card with the same name
as that card" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(more|fewer|less|greater|same)\b[^.]* (than|as)\b

(644 faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; CQ7 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`,
once, at the end: a rebuild after a grammar edit takes about 2–3 minutes and
the test binary about 7, so batch edits. Tests assert the role (governor as
Determiner, head, *than* marker, Comparative Complement, agreement with the
head, and the CQ6 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only. Glossary:
**Discontinuous Comparative Nominal** names an Adjective governor and *same …
as*; amend the entry as part of the landing.

## Out of scope

- *than you do* / *than they do* (9 faces): the stranded-*do* comparative
  clause. In scope only if the existing finite comparative-clause Complement
  reads it with no extra declaration; otherwise list.
- *at least two more cards in hand than you*, *N life more than …*:
  `english-v3-postpositive-comparative-determinatives`.
- *the most* / *the fewest* (*… or tied for …*).
- *more times each turn than …* (1 face).

## Landing record
