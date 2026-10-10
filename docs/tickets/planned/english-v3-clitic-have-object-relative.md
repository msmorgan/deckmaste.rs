---
needs: []
---
# Read a perfect auxiliary in a bare object relative: X is the number of spells you've cast this turn.

## Why

*you've cast a spell* reads; *spells you've cast* does not. The 2026-10-10
triage (`docs/memory/english-v3-blocker-triage-2026-10-10.md`, blocker report
at trunk `vwlkmvns`) ranks this sixth, about 70 sole-blocker faces.
`cargo xtask english-v3 grep "\b(you|they)'ve\b"` finds **218** faces; about
140 are full clauses (*if you've cast …*), which read in the controls below. A noun directly before
the clitic (`\b(spells?|cards?|lands?|creatures?|permanents?|life|…) (you|they)'ve\b`)
has **59** faces. 76 faces have a single failing line containing a word +
*you've* / *they've*; 13 of those are *for each time you've cast your
commander …*, a different gap.

The triage read this as a clitic problem. It is wider: there is no perfect
object gap at all. "the life you have lost this turn" has exactly 1 Reading,
and it is `ZeroObjectRelativeClause` over lexical *have* (*the life you
have*) with *lost this turn* as a passive participial postmodifier. The strong
form is covered only through that analysis; the clitic
(`core-verb:HaveContracted`, `PredicateFrameUse: No`, perfect auxiliary frame
only) cannot take it, so the contracted form reads 0. "… spells your opponents
have cast this turn" reads the same way (3) and is not part of the gap.

## Goal

A finite perfect auxiliary, strong or clitic, heads the Predicate of an
object-gap relative (*spells you've cast this turn*, *the life you've lost
this turn*, *each spell they've cast this turn*), with byte-exact
realisation.

## Analysis

CGEL Ch. 18 §6.2(a), p. 1615, [6]: the clitic forms of *am*, *are*, *have*
and *will* "attach only to a preceding subject pronoun" (*they've*). Ch. 12
§2.1, p. 1034: non-*wh* relatives are "subdivided into ***that*** relatives
and bare relatives depending on the presence or absence of *that*". Those back
the host restriction and the bare relative only.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`FiniteObjectGap` (`[head: lexical(Verb)]`, `head.frame = Transitive`,
`PredicateFrameUse = Yes`) and `AuxiliaryObjectGap` (`[head: lexical(Verb),
complement: BareObjectGap]`, `head.frame = BareAuxiliary`: *can cast*), both
Category `FiniteObjectGap`, which carries `CliticHost`.
`ObjectRelativeClause`, `ZeroObjectRelativeClause` and `FiniteGappedClause`
already place `clitic_separator(predicate.CliticHost)` and require
`clitic_subject(…)`. `AdverbialGapRelativeClause` takes a whole
`FiniteClause` and already reads *the first time you've cast your commander*
(3). `PerfectComplement` / `PerfectAuxiliaryPredicate` are the gap-free
perfect.

Orchestrator rulings 2026-10-10:

- CH1. Refine `AuxiliaryObjectGap` (a second form, or a schema with a bare and
  a perfect instance) so that a perfect auxiliary takes a past-participle
  object gap. No new relative-clause Construction; the three clause routes
  above consume the result unchanged.
- CH2. Licences read declared features: the auxiliary's declared perfect
  frame, the participle's form and its transitive frame. The host restriction
  stays where it is (`clitic_host` / `clitic_subject`); no word test, and no
  route specific to the clitic.
- CH3. The strong form gains the same analysis: *spells you have cast this
  turn* gets the perfect-gap Reading beside the existing lexical-*have* +
  participial-postmodifier Reading. Both are grammatical and both stay
  (retained-Reading contract). Do not prune the old one here.
- CH4. One analysis per string for the new part: the clitic form reads only
  through the perfect gap (the clitic has no lexical-*have* use), and the
  perfect gap is not also reachable through `AdverbialGapRelativeClause` on a
  head that declares no `RelativisableAdjunctClass`.
- CH5. Coordinated participles under one auxiliary (*each card you've cycled
  or discarded this turn*, Hollow One) are in scope only through the existing
  object-gap Coordination series; list them if they need more.
- CH6. Newly covered faces are gains. CH3 predicts increases on covered faces
  with strong *have* / *has* / *had* + participle in a bare relative: report
  each with its shape before landing; anything other than the one added
  perfect-gap analysis per relative is a defect.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--readings 3 --as-card`): witness 0; control = the same
unit with *you have* / *they have* (count shown; 3 means at least 3). Trunk
has since taken `nvotwtqp`; re-probe on the claim parent first.

- Aetherflux Reservoir: "Whenever you cast a spell, you gain 1 life for each
  spell you've cast this turn." (control 3)
- Kydele, Chosen of Kruphix (document): "{T}: Add {C} for each card you've
  drawn this turn." (control 2)
- Greven, Predator Captain: "Greven gets +X/+0, where X is the amount of life
  you've lost this turn." (control 3)
- Knowledge Is Power: "Creatures you control get +X/+X, where X is the number
  of cards you've drawn this turn." (control 3)
- Rug of Smothering: "Whenever a player casts a spell, they lose 1 life for
  each spell they've cast this turn." (control 2)

Controls that must not change: "Draw a card if you've cast a spell this
turn." 2; "You've cast a spell this turn." 2; NP "the first time you've cast
your commander" 3; NP "the second spell you cast this turn" 3; "X is the
number of spells your opponents have cast this turn." 3 (CH6: expected to
rise).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    ('ve|\b(have|has|had)) [a-z]+(ed|n|t)\b

(766 faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; CH6 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3`, once, at the end: a rebuild after
a grammar edit takes about 2–3 minutes and the test binary about 7, so batch
edits. Tests assert the role (the relative's Subject, the perfect auxiliary
with its lexeme and Clitic Host, the participle with its Object gap; for one
strong-form control, both Readings by structure), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *for each time you've cast your commander from the command zone this game*
  (13 lines): *for each time* + an Adjunct-gap relative.
- *creature … that you've put one or more +1/+1 counters on this turn* (Kid
  Loki): a stranded-preposition gap.
- *a spell you've cast is countered* (Multani's Presence) unless it falls out.

## Landing record
