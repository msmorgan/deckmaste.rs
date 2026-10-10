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

- PG7 (orchestrator review, 2026-10-10). ACCEPT: the 22 intended gains
  mirror the perfect's Object-gap sibling under PG1. Following CQ-R1 and
  NM-R1, Misleading Signpost, Portal Mage and Windshaper Planetar are
  accepted as faces covered by a known wrong analysis: an interrogative
  clause read as *which* NP + Object-gap relative. Count the three separately
  from the 22 intended gains; they are not blocked by any check. Record them
  in `english-v3-systemic-residuals` rather than repairing interrogatives
  in this ticket. Integration authorized after the required refresh checks.

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

1. **Census delta** (`opxvnokr`, original claim-parent grammar `zmkuwouk`;
   final refreshed measurement `yrlklvkv`): bounded 492-face census, covered
   436 → 462, total Readings 8,137 → 8,825; lost,
   decreases, covered-face increases and digest-only changes all 0. The tool
   reports 26 newly covered faces: this ticket's 22 intended gains and 3
   known-wrong PG5 admissions accepted under PG7, counted separately: Misleading Signpost, Portal
   Mage and
   Windshaper Planetar (2 wrong relative Readings on each reselect line).
   Capricopian remains unread. Refresh added The Masamune, 15 Readings,
   attributed to cause landing `nzmrrqky`; `refresh-diff.json` reports that
   sole change and 0 losses/decreases/increases/digest-only changes. The
   post-acceptance refresh imported named-token apposition;
   `accepted-refresh-diff.json` reports all 492 faces unchanged against the
   reviewed census. Selector:
   case-insensitive
   `\b(am|is|are|'m|'s|'re|was|were|isn't|aren't|wasn't|weren't|been|be) [a-z]+ing\b`,
   196 surface faces plus 300 covered controls from `baseline-wave8.json`
   (`mqrlzszk`), seed 20261008, 492 identities after overlaps. Evidence:
   `target/english-v3/english-v3-progressive-object-gap-relative/{manifest,before,after,diff}.json`;
   comparison is the supplied `english-v3 diff --lines`, with no custom comparator.
2. **Witnesses** (`zmkuwouk` → `opxvnokr`, complete probe enumeration): Wall of
   Corpses 0 → 1 at Document; Tinder Wall 0 → 1; Scorch Spitter 0 → 2;
   Astral Confrontation 0 → 2; Wall of Shadows 0 → 10; strong pronoun and
   phrase-Subject examples each 0 → 1. Every original sentence probe was 0
   before implementation; none was dropped. Wall of Corpses is an activated
   Ability, so Sentence remains 0; the preserved claim-parent binary also
   verifies Document 0. The seven ticket controls retain 1 each. Scorch Spitter
   retains coordinated-head/right-conjunct attachment; Wall of Shadows retains
   nominal attachment scopes, all with the progressive Object gap.
3. **Spot checks** (`opxvnokr`): ten newly covered faces, selected Readings all
   byte-exact with the relative as nominal Modifier, progressive auxiliary and
   transitive gerund-participle Object gap: Astral Confrontation (opponent);
   Cavalcade of Calamity (player or planeswalker); Fathom Fleet Swordjack
   (player or planeswalker); Gomazoa (creature); Guardian of the Gateless
   (creature); Hellrider (player or planeswalker); Mage Slayer (player or
   planeswalker); Myr Battlesphere (player or planeswalker); Scorch Spitter
   (player or planeswalker); Wall of Corpses (creature).
4. **Deviations and STOPs** (`opxvnokr`): PG1–PG3 add only the gap Category,
   its leaf Construction and the third AuxiliaryObjectGap form; no lexicon,
   loader, duplicate clitic or retired-route edits. The selector was widened
   to include every finite auxiliary spelling, requiring a bounded repeat;
   expanded BEFORE uses the preserved binary built on the claim parent before
   editing, with unchanged lexical data. PG5 follows through the already-landed
   Reselect Object-NP frame (`nqnztuzo`), but its three newly covered interrogative
   faces have unintended nominal-relative analyses. PG7 accepts these as
   known-wrong covered faces, separately from the 22 intended gains; their
   repair remains outside this ticket and is recorded in systemic residuals. Two new-test expectations were
   corrected after the first full run: preserve nominal attachment scopes while
   requiring the intended antecedent Reading, and preserve both declared
   adjective and verbal-passive analyses of blocked. Refresh initially exited
   69 for a sibling changed after banking; it succeeded after that sibling's
   working copy was verified empty. It imported cause `nzmrrqky` and phase
   `roomopmr`; their lexical data and English tests were rechecked in full.
   The post-acceptance refresh imported named-token-apposition grammar and
   tests; the required build, bounded census, full suites and clippy passed.
   No glossary gap. The mandated review stop was observed; orchestrator review
   accepted the landing and authorized integration under PG7 on 2026-10-10.
5. **Tests** (`opxvnokr`): `cargo test -p deckmaste_english_v3` first reported
   898 passed / 2 new-test expectation failures, then passed all 900 after
   corrections; no existing test failed. English clippy with `--tests -- -D warnings`
   passed. After refresh (`uzqxtqnu`), `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` passed 909 and
   `cargo test -p deckmaste_lexical_source` passed 44.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` both passed
   on the first refreshed state. After the post-acceptance refresh
   (`yrlklvkv`), `cargo build -p xtask` passed; the same full test commands
   passed 913 English tests and 84 lexical-source tests (40 unit + 44
   integration), and both exact clippy commands above passed. Evidence:
   `target/english-v3/english-v3-progressive-object-gap-relative/accepted-refresh-*.log`.
   This ticket: restored 0 / re-spelled 0 / ignored 0 /
   added 4 / removed 0 (no removals); own code changes only in English v3.
   No Lean check. Citation checks report 0 non-compliant and 0 stale citations;
   no new CR registrations.
6. **Timing** (`yrlklvkv`, final refreshed state): final bounded AFTER census
   **57.39 s**, 6 workers,
   measured with `/usr/bin/time -v`; no full-corpus census or `--all` run.
