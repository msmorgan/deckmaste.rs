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


Orchestrator ruling 2026-10-11:

- PG6. Apply PG3 in order, stopping at the first successful restriction.
  First restrict the gap to the nominal-Complement realisation, declared on
  the frame slot. If increases or failures persist because *be* also takes
  a nominal Complement, declare the slot's gap licence on the class of
  change-of-state copular frames whose predicative Complement does not also
  host an auxiliary construction; include every existing verb sharing that
  frame class, and exclude *be*'s auxiliary-hosting frame. Record the
  structural exclusion of unattested *the permanent it is* under Deviations
  and append a line to hygiene-3 section A if that implementation lands.
  Acceptance: Aura witnesses read; 0 increases, 0 decreases, 0 lost on the
  8,801-face manifest; the three existing progressive-object-gap-relative
  tests pass unchanged. If neither restriction succeeds, restore the code,
  record the findings and return this ticket to planned with a ticket-only
  change. Otherwise finish and STOP BEFORE INTEGRATE.

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

1. **Census delta — withdrawn attempt `xtulvkxv`, claim parent `wzxklzkx`
   (trunk `xxzltuxw`).** Bounded selector: Method grep
   `(?i)\b(it|they|he|she|you) (becomes?|is|are|was|were|remains?)\b|\bbecomes? (a|an|the)\b`
   (1,048 faces), union with every existing `ZeroObjectRelativeClause`
   face from `baseline-wave9mid.json` (8,501 union faces), plus 300 covered
   outside-union controls selected by `english-v3 diff --manifest --random
   300 --seed 20261008`: 8,801 faces. Claim-parent census contains 7,781
   faces using that declaration. Initial attempt: covered 8,669 → 8,681,
   Readings 271,394 → 271,753; 12 gains, 13 defective increases. PG6 step 2:
   covered 8,669 → 8,679, Readings 271,394 → 271,597; ten gains, one defective
   increase, Seifer, Balamb Rival 10 → 11 (line 2: 8 → 9). Lost none;
   decreases 0; digest-only 0, all classes 0 lines. `diff --lines` reports
   30 changed lines. These are experimental deltas; no grammar change lands.
   Evidence: `target/english-v3/english-v3-predicative-gap-relative/`
   `{manifest,before,after,diff,pg6-step2-after,pg6-step2-diff}.json`.
2. **Witnesses — withdrawn attempt `xtulvkxv`.** All ten whole-face
   witnesses and the three failing fragments were probed before editing:
   0 Readings, complete, no issues. PG6 step 2 per-face counts: Armor of
   Thorns 18; Grave Servitude 18; Lightning Reflexes 18; Mystic Veil 18;
   Necromancy 40; Parapet 17; Relic Ward 18; Soar 18; Spider Climb 18;
   Ward of Lights 19. The shared line has 16, matching its *it controls*
   attachment control's 16; the controller NP has 2, matching its control's
   2 (W9-A). Original controls: 4 / 1 / 1 / 1 / 1 / 3, in ticket order.
   These witnesses remain unsupported after restoration; the ticket returns
   to planned rather than claiming those experimental gains as landed.
3. **Spot checks — withdrawn attempt `xtulvkxv`.** Armor of Thorns,
   Grave Servitude, Lightning Reflexes, Mystic Veil, Necromancy, Parapet,
   Relic Ward, Soar, Spider Climb and Ward of Lights have genuine candidate
   Readings with *permanent* as antecedent, *it* as relative Subject and the
   declared predicative-Complement slot of *becomes* as gap. The remaining
   defective Reading is Seifer: in *a creature attacking one of your
   opponents becomes blocked by two or more creatures*, a bare relative
   ends at *becomes*, with *blocked* read as the finite matrix verb.
   It is an unintended analysis, not an approved gain.
4. **Deviations and STOPs — withdrawn attempt `xtulvkxv`.** PG6 step 1
   declared nominal-Complement realisation on the frame slot via generic
   lexicon feature classes and projected slot features; Wall of Corpses'
   changed line still had 2 Readings versus 1 before, and Seifer's had 9
   versus 8. Step 2 excluded classes hosting an Auxiliary frame, retaining
   the shared Predicate slot class of *become* and *remain*, without a
   verb-naming guard. Wall returned to 1 and the progressive regressions
   passed, but Seifer's non-auxiliary *becomes* ambiguity remained. Neither
   permitted step meets acceptance. Per PG6, all grammar, lexicon, loader,
   construction-core, glossary and new-test changes were restored to the
   claim parent. No hygiene-3 deferral is landed because no gap route lands.
   Existing tests are unchanged; no additional boundary guard was invented.
   A ruling on this remaining participial/clause-boundary ambiguity is
   required before another implementation attempt. Kata's completion
   transition marks a claimed ticket done, so the ticket-only findings are
   returned to planned using its `drop --return-items` transition after
   commit and refresh.
5. **Tests — withdrawn attempt `xtulvkxv`.** Initial
   `cargo test -p deckmaste_english_v3`: failed, 1,121 passed, 3 failed,
   1 pre-existing ignored; all five experimental tests passed. Initial
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`: passed.
   PG6 step 2 `cargo test -p deckmaste_english_v3 --test suite_6
   progressive_object_gap_relative::`: passed, 4 tests, including the three
   unchanged acceptance tests:
   `strong_phrase_subjects_and_that_relatives_share_the_object_gap_route`,
   `ticket_witnesses_link_the_progressive_object_gap_to_the_antecedent`,
   `subject_gaps_passives_and_ungapped_progressives_keep_their_roles`.
   `cargo build -p xtask`: passed for slot-feature experiment. Final
   ticket-only change: restored / re-spelled / newly ignored / added /
   removed existing tests all 0. The five unlanded experimental tests are
   withdrawn together with their unlanded subject; no existing test was
   removed. No consumed code/data changes remain requiring further crate
   tests or clippy. Initial citation checks: 0 noncompliant, 0 stale;
   diff audit selected 0 new citation sites.
6. **Timing — withdrawn attempt `xtulvkxv`.** PG6 step 2 bounded census:
   2m 14.74s wall, 6 workers, samples per face 0, under
   `/usr/bin/time -v`. Initial bounded census: 2m 18.42s. No `--all`
   or full-corpus census was run.
