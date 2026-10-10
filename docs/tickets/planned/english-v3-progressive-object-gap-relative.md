---
needs: []
---
# Read an Object-gap relative clause in the progressive: the player or planeswalker it's attacking.

## Why

A bare or *that* relative whose gap is the Object of a progressive verb does
not read: "Destroy target creature this creature blocked." 1, "… this
creature blocks." 1, "Destroy target creature you've attacked." 1, but "…
this creature is blocking." 0. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): keys `at:attacking .` 13 and
`at:blocking .` 7 sole-blocker faces. `cargo xtask english-v3 grep "(creatures?|player|planeswalker|permanent|opponent|other) (that )?(it's|you're|[a-z]+ ([a-z]+ ){0,2}is) (attacking|blocking)\b"`
finds **28** faces, **26** unread; 26 failing lines match, **25** of them the
only failing line of their face. Sole lines by shape: *the player or
planeswalker it's / that creature is attacking* 9; *(target / each) creature
it's / this creature is blocking* 9; *each opponent you're attacking* 2;
*reselect which player or permanent … is attacking* 4 (not a relative, PG5);
other 1. So **21** sole lines are the relative.

## Goal

A finite Object-gap relative (bare, or with *that*) whose predicate is
progressive *be* + a transitive gerund-participle with its Object gapped,
with the strong and the clitic forms of *be*, byte-exact.

## Analysis

CGEL Ch. 12 §2.1, p. 1034, [1iii]: "*He'll be glad to take the toys* [*you
don't want*]", labelled "non-*wh*: bare relative", where "the anaphoric
element is covert, a gap". §3.5.6, p. 1056, [71ii]: "*Have you seen the book*
[*that she was reading*]*?*": an Object gap under the progressive. Those back
the construction. Everything about its licence is a project ruling. Glossary:
**Relative Clause**, **Finite Object Gap Complement**, **Clitic Host**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`AuxiliaryObjectGap: FiniteObjectGap` has two forms: `[head: lexical(Verb), "
", complement: BareObjectGap] require head.frame = BareAuxiliary` (modals,
*do*) and `[…, complement: PastParticipleObjectGap] require head.frame =
PerfectAuxiliary` ("strong or clitic", from
`english-v3-clitic-have-object-relative`). There is no gerund-participle gap
category beside `BareObjectGap` / `PastParticipleObjectGap`, and no
progressive form. `ProgressiveComplement: ParticipialPredicate` (`require
head.form = GerundParticiple`) reads the ungapped progressive ("This creature
is blocking a creature." 1). `ObjectRelativeClause` and the bare relative
consume `FiniteObjectGap` with `clitic_separator(predicate.CliticHost)`.

Orchestrator rulings 2026-10-10:

- PG1. Extend `AuxiliaryObjectGap` with a third form on the perfect's model: a
  gerund-participle Object-gap complement (a sibling of
  `PastParticipleObjectGap`, same feature list) under the auxiliary whose
  frame selects the progressive Complement. No new relative construction; the
  relative routes read it through `FiniteObjectGap`.
- PG2. The licence reads the auxiliary's declared frame, the participle's
  `form = GerundParticiple` and its transitive frame. No test on *attack* or
  *block*, and none on the antecedent noun.
- PG3. Clitic *'s* / *'re*: the same `CliticHost` mechanism the perfect uses;
  no second clitic route.
- PG4. One analysis per string. *the player it's attacking* has the relative
  with an Object gap. It must not also read with *attacking* as an
  adjectival or intransitive predicate and the antecedent unlinked, and
  "Destroy target creature that is blocking this creature." (Subject gap)
  keeps exactly 1. Passive *be* + past participle stays a Subject-gap matter.
- PG5. Interrogative *reselect which player or permanent … is attacking* (4
  sole faces, with the unknown-frame verb *reselect*) is a fused or
  interrogative clause, not a relative: out of scope; report whether it
  follows.
- PG6. Newly covered faces are gains. Reading-count increases are possible on
  covered faces where a noun is followed by NP + *is/are* + transitive
  gerund-participle with a following Object (*each creature that player is
  attacking you with* shapes); report each with its shape. An increase on a
  Subject-gap relative is a defect under PG4.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Wall of Corpses: "{B}, Sacrifice this creature: Destroy target creature
  this creature is blocking." (sentence 0)
- Tinder Wall: "It deals 2 damage to target creature it's blocking." (0)
- Scorch Spitter: "Whenever this creature attacks, it deals 1 damage to the
  player or planeswalker it's attacking." (0)
- Astral Confrontation: "This spell costs {1} less to cast for each opponent
  you're attacking." (0)
- Wall of Shadows: "Prevent all damage that would be dealt to this creature
  by creatures it's blocking." (0)

Also 0 today: "It deals 1 damage to the player it is attacking."; "It deals 1
damage to the player that creature is attacking." Controls that must not
change: "Destroy target creature this creature blocked." 1; "Destroy target
creature this creature blocks." 1; "Destroy target creature that is blocking
this creature." 1; "Destroy target creature blocking this creature." 1;
"Destroy target creature you've attacked." 1; "This creature is blocking a
creature." 1; "It deals 1 damage to the player it attacked." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(is|are|'s|'re|was|were|been|be) [a-z]+ing\b` (every progressive: the
auxiliary's Complement options widen)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
PG6 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the relative clause as modifier of the antecedent, the gap as Object
of the gerund-participle, *be* as progressive auxiliary, the PG4 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *reselect which player … is attacking* (PG5), 4 sole faces.
- *each creature blocking or blocked by it* (10 sole faces): coordination of
  participial postmodifiers sharing an Object, a different gap.
- *all creatures that the other is blocking* (Sorrow's Path): fused *the
  other*.
- Preposition stranding under the progressive (*each creature you're
  attacking with* 0): unattested in the sole lines.

## Landing record
