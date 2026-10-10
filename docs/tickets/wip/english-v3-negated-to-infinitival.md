---
needs: []
---
# Read *not* before a *to*-infinitival Complement: You may choose not to untap this creature during your untap step.

## Why

*not* reads nowhere: `cargo xtask english-v3 grep '\bnot\b'` finds 111
faces, none covered. The largest single shape is the negated infinitival
Complement of *choose*. Blocker report at trunk `uoqyuxtn`
(`target/english-v3/blockers-wave7.txt`): key `at:choose not` 34.
`grep '\bnot to [a-z]+'` finds **45** faces, all unread and all *choose not
to untap …*; **34** have that line as their only failing line. Objects:
*this creature* 21, *this artifact* 14, *this land* 6, a card name 4. The
positive form reads: "You may choose to untap this creature during your
untap step." 3+ (`grep '\bchoose to [a-z]+'`: 4 faces, 2 covered).

## Goal

*not* reads as the negator of a *to*-infinitival Complement (*choose not to
untap this creature*), wherever the positive Complement reads today, with
byte-exact realisation.

## Analysis

CGEL Ch. 9 §2.3.1, p. 803: "secondary verbal negatives are formed by placing
*not* as premodifier of the VP", with [10v] "*It looks bad* [*for them not to
smile*]"; "The infinitival with *to* has an alternant where *not* follows
*to*". P. 804, [12i]: "*She agreed* [*not to make a formal complaint*]",
called "straightforward cases of secondary negation". Those back *not* as a
dependent inside the non-finite clause, placed before *to*, and the existence
of the *to not* alternant. The representation below is a project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`ToInfinitive` (`[marker: lexical(Subordinator), " ", predicate: node]`,
`require marker.InfinitivalMarker = Yes`) with instances
`(InfinitiveComplement, BarePredicate)`, `(HollowInfinitivalComplement,
BareObjectGap)` and `(PurposeInfinitival, BarePredicate)`. *not* is
`vocab:PredicateNegator/Not` (`lexicon/vocabulary.rs`), loaded as Category
Adverb, with no declared features and no consumer. *choose* already selects
the infinitival Complement.

Orchestrator rulings 2026-10-10:

- NI1. Refine `ToInfinitive`: a second form with a leading negator
  (`[negator: lexical(Adverb), " ", marker, " ", predicate]`). No new
  Construction, and no negated-predicate Category.
- NI2. The licence is a declared feature on the negator lexeme (its
  secondary-negation use), read by the form; no word test. *not* keeps its
  lexeme; no homograph is minted.
- NI3. The negated form is licensed for the `InfinitiveComplement` instance,
  the only attested one (`not to` is 45 / 45 Complement of *choose*). If the
  schema makes it available to the hollow and purpose instances at no cost,
  leave it: do not add a check whose only job is to block an unattested
  string. Record which.
- NI4. One analysis per string. *choose not to untap this creature* has *not*
  inside the infinitival; it must not also read as negation of *choose*
  (project ruling: *choose* is a lexical verb and takes no bare *not*) or
  with *not* as a free Adjunct of the matrix VP. The *to not* alternant has 0 faces and is not built.
- NI5. The trailing *during your untap step* keeps whatever attachments the
  positive sentence has today; this ticket adds none and removes none.
- NI6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected (no covered face contains *not*);
  report any with its shape before landing.

- NI-R1 (orchestrator ruling, 2026-10-10): supersedes NI1's second-form
  requirement. Prefer an OPTIONAL negator field in the existing single form,
  projecting a Category if necessary; only extend the compiler generally if
  that cannot be expressed. The absent case preserves existing Reading
  counts; empty-field digest changes are expected and counted. Negation
  retains one analysis per positive Reading shape, with the NI4 negatives
  unread. The implemented route uses `optional(SecondaryNegator)` and a
  unary lexical projection owning its trailing separator, licensed solely
  by `head.SecondaryNegation`. No existing unary projection has both that
  licence and the needed separator; no compiler change is needed.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category sentence --readings 3`, `--as-card` for
the name): witness 0; control = the same sentence without *not*, 3+ each
(enumeration capped). Re-probe on the claim parent first and record the
uncapped control counts.

- Rust Tick: "You may choose not to untap this creature during your untap
  step."
- Amber Prison: "You may choose not to untap this artifact during your untap
  step."
- "You may choose not to untap this land during your untap step." (6 faces)
- Rubinia Soulsinger: "You may choose not to untap Rubinia Soulsinger during
  your untap step."

Controls that must not change: "You may untap this creature during your
untap step." 3+; "This creature doesn't untap during your untap step." 3+;
"You may choose new targets for the copy." 3+.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(not|choose to|chooses to)\b

(115 faces, case-insensitive) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; NI6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 3 minutes and the test binary about 7, so batch edits; lexicon
data loads at runtime. Tests assert the role (the negator as dependent of
the infinitival, the infinitival as Complement of the matrix verb, the NI4
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only. Glossary: add the secondary-negation
term the feature is named after, citing CGEL p. 803.

## Out of scope

All other *not* (66 faces), each a different mechanism:

- analytic *not* after a clitic or full copula (*it's not your turn*, *it's
  not legendary*, *that's not a …*, *you're not the monarch*): about 40
  lines; primary negation.
- *not* before a past-participial postmodifier (*cards not cast from your
  hand*, *creature not chosen*, *permanent not named …*): about 20 lines.
  CGEL p. 803 [10iv] is the same secondary negation on a different host; a
  follow-up may reuse NI2's feature.
- *has not been*, *but not more than*, *but not from*: 5 lines.

## Landing record

1. **Census delta** (`wqkrtoxrxvor` against claim-parent census
   `mqrlzszkuszn`): 415 compared faces, selected by
   `\b(not|choose to|chooses to)\b` case-insensitively (115 faces), plus
   300 random covered controls, seed 20261008. Covered 302 → 336; total
   Readings 4,356 → 4,741. Lost: none; gained 34; decreases 0; increases 0;
   digest-only 9 (the absent optional negator field). No wrong-analysis gains.
   Evidence: `target/english-v3/english-v3-negated-to-infinitival/{manifest,after,diff}.json`;
   comparison by `english-v3 diff --lines`, with no source-hash changes.
2. **Witnesses**: on claim parent `rouunsxt`, all four witness Sentences had
   0 Readings and their positive counterparts each had 6, uncapped. On
   `wqkrtoxrxvor`, Rust Tick, Amber Prison, the land witness (Ice Floe), and
   Rubinia Soulsinger each have 6 Sentence Readings; their complete faces
   have 12, 12, 12, and 7 respectively. Positive counterparts remain at 6;
   the three additional controls remain at 4. The three NI-R1 negative
   orders each read 0. Corpus grep finds 0 faces for both `choose to not`
   and `to not`; the alternant remains unbuilt.
3. **Spot checks** (`wqkrtoxrxvor`): Amber Prison, Ashnod's Battle Gear,
   Callous Oppressor, Coffin Queen, Deserter's Quarters, Endoskeleton,
   Entrancing Lyre, Everglove Courier, Flamestick Courier, Flowstone Armor.
   Each selected Document Reading places the Secondary Negator inside the
   To-infinitival Complement selected by *choose*, with *untap* as its
   predicate. The witness tests preserve the positive attachment set.
4. **Deviations and STOPs**: the original second-form edit hit the compiler's
   identical-fields obligation. Orchestrator ruling NI-R1 (2026-10-10)
   resolved that STOP with the single optional-field form. One unary
   `SecondaryNegator` Construction/Category supplies the lexical dependent
   and trailing separator; no existing projection supplies both. Its guard
   reads `SecondaryNegation`, declared on the existing *not* Lexeme; no
   compiler or loader changes. All three ToInfinitive instances admit the
   optional dependent under NI3. Seven existing independent-value builders
   gain `negator: None`, preserving their assertions. Added the glossary
   terms Secondary Negation (CGEL p. 803; representation cited as NI1–NI4
   amended by NI-R1) and Secondary Negator; no remaining glossary gap.
5. **Tests** (`wqkrtoxrxvor`): `cargo test -p deckmaste_english_v3` passed
   (795 tests); `cargo test -p deckmaste_lexical_source` passed (82 tests).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   The first English run found a mistaken intransitive frame in the new
   independent *untap* fixture; corrected it to the declared Object-taking
   frame and reran the full suite without weakening assertions.
   Restored 0 / re-spelled 0 / ignored 0 / added 5 / removed 0.
   Citation checks: no noncompliant sites, 0 stale; diff audit selects no new
   CR citation sites.
6. **Timing** (`wqkrtoxrxvor`): bounded census 38.32 s wall time, 6 workers,
   `--samples-per-face 0`, prebuilt binary under `/usr/bin/time -v`.
