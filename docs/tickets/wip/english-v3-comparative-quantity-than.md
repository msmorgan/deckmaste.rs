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

## Review ruling

Orchestrator ruling **CQ-R1, 2026-10-10**: accept the Keeper of the Light and
Keeper of the Flame gains with disclosure as *covered by a wrong analysis*;
report 63 correct gains plus 2 wrong-analysis gains. Do not extend this ticket
to restrict causative Have / bare Do. Record these faces and the vocabulary
batch's Diligent Farmhand and Pardic Firecat in systemic residuals. This is a
project ruling, not a CGEL claim.

## Landing record

1. **Census delta** (`rqnkrnur` claim-parent content → `wzpslwxq`): bounded
   selector `\b(more|fewer|less|greater|same)\b[^.]* (than|as)\b` (644 faces)
   plus 300 random covered faces, seed 20261008, gives 938 distinct faces
   (six overlaps). Covered 624 → 689; total Readings 97460 → 98484. Lost:
   none. Official diff: gained **63 correct + 2 covered by a wrong analysis**,
   decreases 0, increases on previously covered faces 0, digest-only 98,
   unchanged 775. Keeper of the Light and Keeper of the Flame each gain
   0 → 3 through causative Have with *more life than you* as Object and a
   following plain Do BareAuxiliaryPredicate / BareEllipsis (OmittedPlain),
   rather than the intended finite comparative clause; accepted with
   disclosure by CQ-R1. Their three variants attach the activation-time
   Adjunct at different levels. The 98 digest-only faces use the existing
   equality `DiscontinuousComparativeNominal`: converting that Construction
   to a schema records its result Category in the Reading's Debug
   representation, changing the digest while retaining all Reading counts.
   Evidence: `target/english-v3/english-v3-comparative-quantity-than/`
   (`before.json`, `after.json`, `diff.json`, `manifest.json`); official diff
   used `--lines`. The 300 random controls were checked covered on the
   claim-parent snapshot. Post-refresh measurement `tmqsnsyt` (empty child of
   `wzpslwxq`): covered 624 → 691, Readings 97460 → 98525; lost 0, gained
   67, decreases 0, increases 0, digest-only 98, unchanged 773. The official
   `after.json` → `refresh-after.json` diff isolates two trunk gains and 41
   Readings: Great Unclean One (5, declared Plaguebearer of Nurgle name) and
   Rumbling Ruin (36, Count Object frame), from vocabulary-batch-2
   `nqnztuzo` / retirement `pwoslssz`. This ticket's gains remain 63 correct
   + 2 covered by a wrong analysis. Refreshed evidence: `refresh-after.json`,
   `refresh-diff.json`, `refresh-trunk-diff.json` in the same directory.
2. **Witnesses** (`rqnkrnur` → `wzpslwxq`): Sunstar Expansionist 0 → 1;
   Okina Nightwatch 0 → 1; Unified Will 0 → 1; Even the Odds 0 → 1;
   Survival Cache document 0 → 1; *You have less life than an opponent*
   0 → 1. All witnesses and controls were probed before implementation.
   Controls unchanged: respective card controls 1, 2, 1, 1, 1;
   *more than two lands* 1; *Its power is greater than its toughness* 1;
   *a creature with power greater than this creature's power* 5 (claim-parent
   reprobe supersedes historical 3); *a card with the same name as that card*
   1. Bare *more votes* attestation: 15 faces / 28 matches; no bare
   Determiner route added. Of nine attested stranded-do faces, the seven
   other than the Keepers remain unread.
3. **Spot checks** (`wzpslwxq`): Advice from the Fae (*more creatures / each
   other player*); Aerial Surveyor (*more lands / you*); Agent of the Shadow
   Thieves (*more life / that player*); Akuta, Born of Ash (*more cards in
   hand / each opponent*); Archaeomancer's Map (*more lands / you*);
   Battle Angels of Tyr (*more cards in hand, lands, life / each other
   player*); Beza, the Bounding Spring (*more lands, life, creatures, cards
   in hand / you*); Birth of the Imperium (*fewer creatures / you*);
   Cartographer's Hawk (*more lands / you*); Claim Jumper (*more lands /
   you*). Selected trees retain the Determinative governor, Nominal head,
   inequality marker and immediate AccusativePhrase Complement in the
   NounPhrase instance of `DiscontinuousComparativeNominal`.
4. **Deviations and STOPs** (`wzpslwxq`): the existing Construction is a
   two-category schema; the core schema binder now supports `lexical(node)`
   via the same category binding used for node fields. A core test checks
   correct lexical categories and invalid bindings. No parallel comparative
   Construction, lexical homograph or loader feature added; lexicon declares
   the comparative Determiner and marker licences. Oracle English glossary
   amends Discontinuous Comparative Nominal and adds Comparative Determiner
   Use, separating CGEL evidence from CQ1–CQ4 rulings. The Keeper STOP was
   resolved by CQ-R1 above; causative/ellipsis restrictions are deferred,
   with four known wrong-analysis faces recorded in systemic residuals.
   Refresh initially exited 69 for concurrent divergence in the multiplier
   workspace; its owner resolved that state, then refresh succeeded. No
   other workspace was repaired or rewritten by this claimant.
5. **Tests** (`wzpslwxq`): `cargo test -p deckmaste_english_v3` passed 752;
   `cargo test -p deckmaste_lexical_source` passed 35 unit + 41 integration;
   `cargo test -p deckmaste_construction_v3_core` passed 37;
   `cargo test -p xtask english_v3` passed 56. All passed with 0 failures.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`
   all passed. Restored 0; re-spelled 1 (same-as independent Reading now
   includes result Category, same asserted outcome); ignored 0; added 5
   (four comparative integration tests, one core binding test); removed 0.
   Citation checks: no noncompliant sites, 0 stale; STDIN diff audit 0 new
   citation sites, so no new registrations to bless. Post-refresh (`tmqsnsyt`),
   the same full commands were rerun: English 753, lexical-source 39 unit +
   41 integration, construction-v3-core 37, xtask English 56; all passed.
   The same three clippy commands also passed after refresh. Refreshed citation
   checks: 0 noncompliant, 0 stale; diff audit 0 new sites.
6. **Timing** (`tmqsnsyt`, post-refresh): bounded after-census **54.49 s**, workers 6,
   samples-per-face 0, measured under `/usr/bin/time -v` with the prebuilt
   workspace-local binary. No full corpus census was run, per the binding
   verification budget.
