---
needs: []
---
# One copular complement: the whole PredicativeComplement sum, plus scalar equality

**R4 — Group R.**

Defect, two halves.

1. `copular_subject_gap_relative_clause`
   (`crates/deckmaste_english_v2/src/constructions.rs:4348`) narrows its
   `complement` role to `PredicativeAdjectiveComplement` rather than the
   `PredicativeComplement` sum that already carries Color, Nominal, Status,
   Ability, Orientation, PowerToughness and Scalar arms. With
   `vocab PredicativeAdjective` holding one member (`Legendary = "legendary"`),
   `Destroy target creature that is legendary.` selects while
   `… that is red.`, `… that is a Goblin.`, `If that land was nonbasic, …` and
   `The same is true for creature spells you control…` all fail.
2. `abstract sum PredicativeComplement` (`:1212`) has no arm for a scalar
   *equality* complement. `Scalar: PredicativeScalarComplement` is
   `predicative_scalar { value: CardinalQuantity }` (`:1762`) — a bare cardinal,
   not `is equal to ‹count›`. So `Syr Elenora's power is equal to the number of
   cards in your hand.`, `Awakened Amalgam's power and toughness are each equal
   to the number of…` and `Yavimaya Kavu's power is equal to…` fail. The
   `ScalarMeasureValue` category already exists as a `PrepositionalComplement`
   arm (`:1094`, added by `english-v2-with-preposition`) and is the value to
   reuse.

Pinned shape. Widen every copular complement site to the `PredicativeComplement`
sum, and give that sum a scalar-equality arm built on the existing
`ScalarMeasureValue` rather than a new sealed atom. All arms are present
regardless of witness counts; the complement-shape census is provenance recorded
in the landing record, exactly as `english-v2-with-preposition` recorded its own.
The adjective inventory itself is `english-v2-adjective-inventory`'s
(`vocab PredicativeAdjective` / `AttributiveAdjective` are transitional homes for
content words); `docs/tickets/fog.md` §"Predicative complement as one copular
frame (A7)" owns the deeper question of the sum's game-carved partition. Do not
duplicate either — reference them and stop at the complement widening.

Fences. Adding a second narrowed copular construction beside the widened one. A
per-complement-kind `checked by`. Adding arms only where a census shows them.

Glossary: Predicative Complement, Copula, Complement, Subject-Gap Relative
Clause, Scalar. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.

2026-09-04: inherits the coverage owed for `Target land becomes a 3/3 creature
until end of turn.` from `english-v2-clause-level-duration`; re-measured on that
landing's tree, the witness is still a parse failure and never regressed.

## Landing record

Measured after `kata refresh` on feature change `llxsktkv` against its
refreshed parent. The implementer's numbers below were taken on the pre-R10
base; the reviewer re-measured on the refreshed tree and the numbers that
govern are in `### Review corrections`. The coverage lock is blessed add-only
at 17,423 covered units.

### PROVE

- The grammar widens `copular_subject_gap_relative_clause` to the complete
  `PredicativeComplement` sum. It also routes the ordinary copular predicate
  through `PredicativeComplementLexicalVerbPhrase`, so the same sum can take
  the existing nonprepositional predicate-adjunct host. No second narrowed
  copular construction, per-kind checker, dominance edge, exception, or
  word-naming guard was added.
- `PredicativeComplement` now has an always-present
  `ScalarEquality: PredicativeScalarEqualityComplement` arm. Its construction
  is optional `FloatedQuantifier` plus the existing `ScalarEquality`
  (`equal to <ScalarValue>`). See `### Review corrections` for why the
  `ScalarMeasureValue` the ticket names is not the reused category.
- Focused assurance selects red and Goblin subject-gap relatives, both singular
  and coordinated scalar equalities, and both forms of the inherited witness:
  `Target land becomes a 3/3 creature.` and `Target land becomes a 3/3
  creature until end of turn.`
- `cargo fmt --all` completed. Strict Clippy for both touched crates completed
  with `Finished dev profile ... in 4.06s`.
- The refreshed-tree test gate completed:
  `deckmaste_english_v2` library 144 passed; predicate grammar 106 passed;
  `xtask` library 453 passed and 1 pre-existing ignored; `xtask` binary 12
  passed; determinism 1 passed; flavor 1 passed; English doctests 2 passed.
  Every reported result was `test result: ok`.
- Coverage ownership and traversal evidence remained internally clean in the
  stopped run: 0 selected-uncovered, 0 internal failures, 0 ownership failures,
  0 traversal failures, 0 leaf-traversal failures, 0 roundtrip mismatches, and
  0 provenance-plan mismatches.

### DISCLOSE

- **STOP — genuine selection ties.** Report-mode coverage and
  `ambiguity --require-resolved` both reject two identities. Each has equal
  surviving analyses
  `PossessiveOwnerPossessiveSelfReference` and
  `PossessiveOwnerPossessiveSingularNominal`; there is no governing dominance
  edge or exception:
  - `9ba9f50f538a3566e7c63df0e94d05db5d65d1c7b76427c0c6eb1e2fcb10f64d`
    — Daretti, Rocketeer Engineer — the selected scalar-equality clause is
    blocked at the possessive owner.
  - `d0556136f9c3127edac724ae4b91b9816cb93f5134e1fd6ea69263e07d488ed1`
    — Nightmare — the selected coordinated scalar-equality clause is blocked at
    the possessive owner.
  The ticket forbids inventing a tie breaker; both analyses are preserved.
  Resolved by coordinator ruling on 2026-09-04 — see
  `### Ruling and implementation: the identity claim-kind ordering`.
- **STOP — acceptance/fence contradiction.** The ticket names “The same is true
  for creature spells you control…” as an acceptance witness while also
  requiring this work to stop before the adjective inventory. The widened
  complement site is available, but that sentence still cannot reach it because
  the required “same”/“true” lexical inventory is absent. That vocabulary is
  owned by `english-v2-adjective-inventory`, and the deeper sum partition is
  owned by fog §A7; neither was duplicated here.
- The “If that land was nonbasic, …” probe reaches the widened status complement,
  but the attempted complete synthetic witnesses failed later in their clause
  bodies. Adding unrelated body templates is outside this ticket, so this work
  does not claim a complete-sentence selection artifact for that wording.
- Because coverage stopped on the two ties, `--check` did not print the lock
  comparison line. Therefore no printed lock delta exists to paste, `--bless`
  was not run, and the separate roundtrip command was not run. The coverage run
  itself observed 0 roundtrip mismatches.
- Deviation/addition: `SpecificityTier` gains an `Identity` tier between
  `TypedLexical` and `Literal`, backed by a generated `Lexical::is_identity` in
  `deckmaste_construction_core`, resolving the two ties by coordinator ruling
  [CR#201.5,201.5a,201.5c]. Added by the reviewer; see
  `### Ruling and implementation`.
- Deviation/addition: `predicative_complement_predicate` was recategorized from
  `VerbPhrase` to a new `PredicativeComplementLexicalVerbPhrase` member of the
  existing `LexicalVerbPhrase` sum, so an ordinary copular predicate can take
  the nonprepositional predicate-adjunct host. Added by the reviewer; see
  `### Review corrections`.
- Deviation/addition: 6 corpus units containing `enter the battlefield` change
  their selected analysis from a wrong predicate-nominal reading to
  `enter_location`. Added by the reviewer; see `### Review corrections`.
- Deviation/addition: refreshed trunk exposed an unrelated borrow-after-move
  compile defect in `crates/xtask/src/english_v2/corpus.rs`.
  `write_parser_metrics(&mut diagnostics)` was reborrowed as
  `write_parser_metrics(&mut *diagnostics)`; this is the minimum compile-only
  repair required to run the mandated gates.
- No citation-bearing file changed; citation gates are not applicable.
- Glossary gaps: `Predicative Complement`, `Copula`,
  `Subject-Gap Relative Clause`, and `Scalar`. `Complement` is present in
  the Oracle English glossary.
- Assurance counts: restored 0; re-spelled 0; ignored 0; added 2; removed 0.

### REPORT

- Selection census: 17,068 selected / 15,573 parse failures / 0 ties before;
  17,375 selected / 15,264 parse failures / 2 ties after. Resolution split:
  unique 13,452 -> 13,598; specificity-resolved 3,616 -> 3,777. Coverage-status
  delta is +307 selected, 0 lost. The lock remains 17,068 because STOP prevented
  blessing.
- Construction declarations: 388 -> 389. Licensing checkers remain 20 permitted
  / 0 forbidden; licensed vocabulary/lexicon homographs remain 2; form-literal
  vocabulary overlaps remain 5; exceptions remain 0.
- Selected complement-shape provenance (incidences, not a filter):

  | complement arm | before | after | delta |
  | --- | ---: | ---: | ---: |
  | Adjective | 13 | 13 | 0 |
  | Color | 55 | 69 | +14 |
  | Designation | 98 | 102 | +4 |
  | Face orientation | 1 | 1 | 0 |
  | Nominal | 1,027 | 1,182 | +155 |
  | Power/toughness | 1,180 | 1,230 | +50 |
  | Scalar cardinal | 3 | 3 | 0 |
  | Scalar equality | 0 | 142 | +142 |
  | Status | 880 | 889 | +9 |
  | Ability | 0 | 0 | 0 |

- Performance advisory for the stopped coverage run: 8 workers, 144 s wall
  (124 s gate telemetry), 139,628 ns/B, host load 15.38 / 14.64 / 15.80.
  Concurrent-process count is unavailable through the sandbox; the reviewer must
  stamp contention. The advisory ceiling was exceeded under that load and is
  reported, not fitted to.
- The full post-refresh coverage summary was:
  `total_units=32641 selected_units=17375 covered_units=17375
  selected_uncovered_units=0 parse_failures=15264 unresolved_ties=2
  internal_failures=0 roundtrip_mismatch_units=0 ownership_failure_units=0
  nonterminal_nodes=736809 visited_constructions=736809
  traversal_failure_units=0 expected_leaves=258042 visited_leaves=258042
  leaf_traversal_failure_units=0`, `lock_mode=report`.
- The ambiguity artifact independently reports 17,375 selected: 13,598 unique,
  3,777 specificity-resolved, 2 unresolved ties, 15,264 parse failures, and 0
  internal failures.

### Review corrections

Reviewer: Opus landing reviewer, 2026-09-04, after a second `kata refresh` onto
the R10 (`english-v2-possessive-nominal-form-collapse`) line. The two selection
ties survived that refresh; they were raised as a ruling question, the
coordinator ruled, and the ruling is implemented in this landing (see
`### Ruling and implementation` below). Gates were run once on the final tree.

The complement widening alone, measured before the ordering change landed:
17,114 -> 17,421 selected (+307, 0 lost), unique +146, specificity-resolved
+161, 2 unresolved ties. The `+146` and `+161` account for the whole `+307`, so
no unit already selected on trunk changed its resolution class. The final
figures, after the ordering, are in `### Final measurement`.

Per-unit selection-neutrality proof (`ambiguity --json`, trunk content vs this
tree, both at `--workers 8`): 307 newly selected, 0 lost, 928 units changed
their selected construction path. 922 of the 928 are the pure recategorization
(`VerbPhrase{PredicativeComplementPredicate}` becomes
`VerbPhraseBaseVerbPhrase` +
`PredicativeComplementLexicalVerbPhrase{PredicativeComplementPredicate}`) and
are identical once that rewrite is normalized away.
The remaining 6 are a **selection correction this landing makes and the
implementer did not disclose**: `Grafdigger's Cage`, `Kunoros, Hound of
Athreos`, `Recommission`, `Silver Surfer, Cosmic Voyager`, `Soulless Jailer`
and `Weathered Runestone` all contain `enter the battlefield`. On trunk they
selected `predicative_complement_predicate` with a **predicate-nominal**
complement — `the battlefield` read as a predicative Nominal, which is wrong
English and a wrong analysis. On this tree they select `enter_location` with
`the battlefield` as an Object, which is correct. The extra `base_verb_phrase`
node the recategorization inserts is what lets the locative analysis win the
specificity comparison.

Competing construction pair behind the specificity-share rise (census item):
of the 161 newly specificity-resolved units, 124 are decided at the possessive
owner between `possessive_self_reference` (the card naming itself, which wins)
and `possessive_singular_reference`; the next largest, 85, is
`coordinated_noun_phrase` against `genitive_determiner_coordination_reference`.
None of the 307 newly *selected* units has both `possessive_self_reference` and
`possessive_singular_nominal` among its candidates — that third route is what
produces the two ties below.

Findings corrected in this record, none requiring a code change:

- MEDIUM — the record claimed the scalar-equality arm reuses `ScalarMeasureValue`
  and that its identities select "through `predicative_complement_predicate`".
  Both are false. `scalar_measure_value` is `<ScalarMeasure> <ScalarThreshold |
  ScalarComparison>` (`power 2 or greater`), not `equal to <count>`; the
  category that spells `equal to <count>` is the existing `ScalarEquality`
  (`equal to <ScalarValue>`), already reused by `power_toughness_value` and by
  the equality verb codecs. The implementer built the arm on `ScalarEquality`,
  which satisfies the ticket's actual pin — an existing category, no new sealed
  atom — while departing from the category the ticket names. The ticket's own
  premise is the error; verified by reading both construction declarations.
  Corrected in PROVE and in the identity-list preamble. And the hosts are
  `finite_copular_predicate` (130), `declared_object_predicative_verb_phrase`
  (3) and `predicative_complement_predicate` (2), not one host.
- MEDIUM — undisclosed addition: recategorizing `predicative_complement_predicate`
  from `VerbPhrase` to the new `PredicativeComplementLexicalVerbPhrase` member of
  the `LexicalVerbPhrase` sum is a structural change beyond "widen the complement
  role". It is inside the ticket's letter (the 2026-09-04 note inherits the
  `Target land becomes a 3/3 creature until end of turn.` witness, which only a
  nonprepositional predicate-adjunct host can select) but belonged in Deviations
  and additions. Recorded there now. Probed for over-admission: the new sum
  member reaches `predicate_adjunct_predicate` (nonprepositional adjuncts:
  duration, purpose, frequency, manner), `stacked_predicate_adjunct_predicate`,
  `as_though_predicate` and `alternative_predicate`. Prepositional adjuncts on
  copular predicates were already reachable before and after through
  `base_verb_phrase -> VerbPhrase -> PrepositionalPredicateAdjunctHost`, so
  nothing was widened there. The corpus shows no unit selecting an
  English-rejecting adjunct on a copular predicate: every one of the 126
  ordinary-copular gains reads correctly.
- MEDIUM — the `same`/`true` deferral was not written down anywhere a future
  claimant would find it. Appended as a dated one-line note to
  `docs/tickets/planned/english-v2-adjective-inventory.md`.
- LOW — the implementer's census figures (17,068 -> 17,375, 13,452/3,616 ->
  13,598/3,777, form-literal vocabulary overlaps 5) were measured on the pre-R10
  base and are stale; the table above supersedes them. The `+307`/`0 lost` shape
  is unchanged. On the refreshed tree the form-literal vocabulary overlaps are
  9 and the licensed vocabulary/lexicon homographs 2, both inherited from trunk.

Assurance for the implementer's own diff, verified against it: restored 0;
re-spelled 0; ignored 0; added 2
(`copular_scalar_equalities_use_the_predicative_complement_sum`,
`copular_complements_accept_color_nominal_and_duration_witnesses`); removed 0.
Both use the pre-existing `assert_selected_with_specificity` helper, which
requires a unique or specificity-resolved decision with exactly one survivor,
zero exception uses, an exact round-trip render and covered ownership — no
weakening. No `checked by`, `require`, or comment anywhere in the landing names
a lexeme, construction, verb, noun, preposition or card. The landing's totals,
including the review's own additions, are in `### Final measurement`.

Gate artifacts for the tree as it stood at this point (widening only, before the
ruling landed) are superseded by `### Final measurement`; they are not repeated.
Both corpus gates rejected there on the two ties, so no lock delta existed to
paste and `--bless` was not run.

### Ruling and implementation: the identity claim-kind ordering

The two ties the implementer reported survived the R10 refresh unchanged, were
raised as a ruling question, and the coordinator ruled on 2026-09-04. The
question and its answer are recorded here in full because the fix is part of
this landing.

**The ties.** Both identities were `unresolved_tie`, both surviving candidates
identical except at the possessive owner, with element-for-element identical
specificity vectors, so no specificity comparison could decide them:

- `9ba9f50f538a3566e7c63df0e94d05db5d65d1c7b76427c0c6eb1e2fcb10f64d` —
  Daretti, Rocketeer Engineer —
  `Daretti's power is equal to the greatest mana value among artifacts you control.`
  Candidates 3 and 5, `ordering: "equal", decisive: "tie"`, differing only at
  path index 9: `PossessiveOwnerPossessiveSelfReference` against
  `PossessiveOwnerPossessiveSingularNominal`.
- `d0556136f9c3127edac724ae4b91b9816cb93f5134e1fd6ea69263e07d488ed1` —
  Nightmare —
  `Nightmare's power and toughness are each equal to the number of Swamps you control.`
  Candidates 4 and 5, same `decisive: "tie"`, differing only at path index 14 in
  the same pair; the other 32 path entries and all 33 specificity entries
  identical.

**Cause.** `possessive_self_reference: PossessiveOwner` is
`{ spelling: identity SelfReferenceSpelling }`;
`possessive_singular_nominal: PossessiveOwner` is `{ head: lex Noun }` with
`require all(head.countability is Count, head.properness is Proper)`. When a
card's self-reference short name is also a declared proper Noun the two spell
the identical surface with the identical specificity vector.
`plugins/builtin_v2/macros/stubs/subtypes/creature/Nightmare.ron` declares
`grammar: Noun(singular: "Nightmare")`;
`.../subtypes/planeswalker/Daretti.ron` declares
`grammar: Noun(singular: "Daretti", plural: Unavailable)`. The collision is
pre-existing; trunk carried 0 ties only because neither clause parsed before
this landing supplied the scalar-equality complement.

**The ruling.** The ordering is a rules fact, not a preference. Text that refers
to the object it is on by name means just that particular object and no other
object with that name [CR#201.5]; a shortened name used that way is treated as
the full name [CR#201.5c]; and the same holds for a name inside an ability one
object grants another [CR#201.5a]. Inside a card's own text its name **is** the
self-reference, so the proper-Noun reading of those bytes is wrong.

**Implementation — a general claim-kind ordering, not a tie breaker.**
`SpecificityTier` gains a fourth tier, `Identity`, between `TypedLexical` and
`Literal`, so the ranking is Literal ≻ Identity ≻ TypedLexical ≻ Nonterminal.
A position claims `Identity` when its terminal is a spelling supplied by the
parse context or by a catalog; `Lexical::is_identity` is generated in
`crates/deckmaste_construction_core/src/emit/runtime.rs` from the declared
identity inventory, so it covers `SelfReferenceSpelling` and `CardName` today
and any future identity without further code. There is no dominance edge
between named constructions, no selection exception (`exception_uses` remains
0 corpus-wide), and no checker, requirement or comment naming a card, word or
construction. Identity was placed *below* Literal — the ruling pins only
Identity ≻ TypedLexical — because that is the placement that moves nothing
else; the choice is recorded in the ADR amendment.

The ruling is recorded as
`## Amendment: an identity claim outranks a declared type over the same bytes
(2026-09-04)` in `docs/decisions/english-v2-rewrite.md`, citing
[CR#201.5,201.5a,201.5c]. `cargo xtask cite check --list-noncompliant` is empty,
`cite check` reports 0 stale over 14,395 citations, all three rules were already
in `cr-citations.lock`, and `jj diff --git | cargo xtask cite audit --diff`
audited 5 citation sites; each rule's text was read against its claim.

**Effect, measured.** Daretti and Nightmare each select
`possessive_self_reference` — the correct reading — by specificity, with one
survivor and no exception use. Corpus-wide the ordering moved **nothing else**:
the per-unit trunk-vs-tree diff below is identical to the one taken before the
ordering existed, apart from those two identities. The
`possessive_singular_nominal` route still wins where no identity claim covers
the bytes (`Lumbering Worldwagon`, `This Vehicle's power is equal to the number
of lands you control.`), and the 29 `named_card_reference` units
(`a card named Alpine Watchdog`) are unchanged.

Assurance for the ordering (all added, none removed):
`the_claim_kind_order_ranks_an_identity_above_a_typed_lexical` (unit, tiers and
their order), `an_identity_claim_outranks_a_declared_type_over_the_same_bytes`
(the minimal pair: identical bytes, self-reference wins under the card's own
name, declared type wins under another card's, and the `Identity` tier is
present only in the first), and
`a_declared_type_owns_a_possessive_without_an_identity_claim` (the attested
corpus witness for the losing side). Seven `Witness` rows in
`tests/nominal_grammar.rs` were re-spelled `T` -> `I` at the identity position
— same cards, same asserted paths and outcomes, new spelling of the specificity
string.

### Final measurement (reviewer, refreshed tree, gates green)

| quantity | base (lock/trunk) | tree | delta |
| --- | ---: | ---: | ---: |
| selected units | 17,114 | 17,423 | +309 |
| lock `covered` (blessed add-only) | 17,114 | 17,423 | +309 / -0 |
| unique | 13,468 | 13,614 | +146 |
| specificity-resolved | 3,646 | 3,809 | +163 |
| unresolved ties | 0 | 0 | 0 |
| parse failures | 15,527 | 15,218 | -309 |
| construction declarations | 388 | 389 | +1 |

+307 of the gains are the complement widening and the scalar-equality arm; the
remaining +2 are Daretti and Nightmare, which the ordering resolved. Licensing
checkers remain 20 permitted / 0 forbidden, exceptions 0, licensed
vocabulary/lexicon homographs 2, form-literal vocabulary overlaps 9 — every one
inherited from trunk. `selected_uncovered_units` 0, `roundtrip_mismatch_units`
0, `ownership_failure_units` 0, `traversal_failure_units` 0,
`leaf_traversal_failure_units` 0, `gap_spans` 0, `overlap_spans` 0,
`synthetic_claims` 0, `provenance_plan_mismatches` 0.

Gate artifacts, all foreground, `--workers 8`, `CARGO_BUILD_JOBS=8`:

- `cargo fmt --all` clean; `cargo clippy -p deckmaste_construction_core -p
  deckmaste_english_v2 -p xtask --all-targets -- -D warnings` clean for each
  crate.
- `cargo test --workspace` (mandatory: the diff touches
  `crates/deckmaste_construction_core/src/emit/`) — 128 `test result: ok`
  lines, 0 failed, 2 pre-existing ignored.
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check`
  before blessing: `newly covered 309 corpus identities`, 0 stopped being
  covered. After `--bless`, `--check` prints
  `summary {"total_units":32641,"selected_units":17423,"covered_units":17423,
  "selected_uncovered_units":0,"parse_failures":15218,"unresolved_ties":0,
  "internal_failures":0,...} lock_mode=report` with no delta lines.
- `cargo xtask english_v2 ambiguity --require-resolved` —
  `selected=17423 unique=13614 specificity_resolved=3809 exception_resolved=0
  unresolved_ties=0 parse_failures=15218 internal_failures=0 exception_uses=0`,
  exit 0.
- `cargo xtask english_v2 roundtrip --require-clean` —
  `parse accepted 17423 / clean 17423 / mismatched 0 / not parse accepted
  15218`, exit 0.

Performance advisory: `coverage` 8 workers, 101 s wall, 116,077 ns/B, host load
8.83 / 10.68 / 8.78; `ambiguity` 8 workers, 108 s wall, 143,004 ns/B, host load
8.74 / 10.28 / 9.29; `roundtrip` 8 workers, 101 s wall, 118,793 ns/B, host load
8.26 / 9.83 / 9.24. All three exceed the 16.26 s quiet-host ceiling
(`criterion=capped_workers`) and are reported, not fitted to. **Contention
stamp: 2 concurrent workloads — one executor
(`english-v2-granted-ability-coordination`, codex sol, live on
`constructions.rs`) plus this review.**

Total assurance for the landing: restored 0; re-spelled 7; ignored 0; added 5
(2 by the implementer, 3 by the review); removed 0.

#### Newly selected identities: scalar-equality analysis (137)

Each identity below selects through the copular host ->
`PredicativeScalarEqualityComplement` -> `ScalarEquality` -> `ScalarValue`
(possibly inside a larger selected clause). Re-measured hosts on the refreshed
tree: `finite_copular_predicate` 130, `declared_object_predicative_verb_phrase`
3, `predicative_complement_predicate` 2.

- `9018b33beac0a9ee4a591dc936a96303cad1a109d21c1b44ae1ccd95f9a77906` — Abominable Treefolk
- `f2ad526d84134129403afe7330e38b0b278775fdb93f6ec44dba49b448391ed4` — Altar Golem
- `1edffe59cc25de8627f7332b446a24bc82a5f469236c25b8b2c78574f4d25d59` — Apocalypse Demon
- `0b7650266142506ddfdd1cb8e628ef56bd83b0b337ff29a7e50489b924e02165` — Avalanche of Sector 7
- `a024a6c4d282040c1724a270d887cd6189541ee75e61e8be6024fb0a5cba8102` — Aven Trailblazer
- `f8c3dbace2b3ac68258dd768030aea2b03e40125caca7473e67a21dac0199912` — Battle Squadron
- `4ae64bcd01ab6ee311d3c461cd8e9e5dab2fb7279a51d0626a0a7ce61e279a37` — Beanstalk Giant // Fertile Footsteps // Beanstalk Giant
- `d51fb781cc487b5755942316af9fb8059f754dadfcc2b6b8c89f9b9d1b50df39` — Beast of Burden
- `e607877655e29c992255ba6cc881cfb7f6b0fa7c3bf9ba6b15589ac0021b29c7` — Belligerent Yearling
- `13abc91610665cf0b818680a88fc1e868b95c88857f12c0e2c35fcf4cc4b77db` — Body of Knowledge
- `c94c83fcf2f0e17b383916ed6606b554a675ebd1b34da5fde96ef4b65b64fe14` — Boneyard Mycodrax
- `28793533b34f42120242327933742eaeb896d3b0fd486cd3812acbdb015b9827` — Boneyard Wurm
- `e76be9a515461ec06ca27c1c0db79f9d5f12e7aaf81efb442d22efa2221d3300` — Broken Visage
- `7a68f89b8b432dd0bf3d1c89cb04c198a6957c2d1a16e6d29d38a02681d0f7be` — Brotherhood Vertibird
- `a494dbe5e4ab7d5f9448f4b6b4fca0f973858b6c491094f9fe77aa89610cf111` — Burrowguard Mentor
- `610c7f391e3930b2603611b6a1fbe6de2e68eeb94881bc9716651af14a28640f` — Caller of the Hunt
- `b934fcc613b51c4ffb3bd55ab73950fcf645885dd94a9cfb641d8f3d9d36318f` — Cantivore
- `e9f8dbea6815c2271ba78cf7757143f4d0fafc9fafb233a3712a5e6f284c66c0` — Cephalopod Sentry
- `e8a4787d526f9637a3ebcf6193a7349809fbb02a0590788b1fa27cf0ecd95760` — Chameleon Spirit
- `c3c778822be9f3d854278e024a3aab8cc584b4531801c66e0bb817b657f75096` — Chimeric Mass
- `45b3d9138c8a735d8f7eca1386993564329b4f4059d56adc39d55aed5a3c50b2` — Cognivore
- `d2e4f0d1155423e278d8c8b50eaa9df1aa3eb9ced79b46e54124b2077b284b7d` — Coiling Woodworm
- `3f67b3fc60b360146ac1999c5c526400225eef40a13462776c00ab28c8019102` — Crowd of Cinders
- `b84cec481716344084bce6145a277b6fe32cce81511dee64f89bdfa96b50af09` — Cruel Somnophage // Can't Wake Up // Cruel Somnophage
- `b2226185681730b47c3bcdf0d2ca604bce1e214b4a5508642959168713c16436` — Crusader of Odric
- `63a6d632cd63f17f8b631405beb3d0505f3ff7e2d63ae6019db2dabd533a1fd6` — Dakkon Blackblade
- `e0fa852344f0279dff5e127d846fe03d7ea4dff4bcb5cad697038ec02411023e` — Dakmor Sorceress
- `3f4ab30d43d50dbdedbf5e296e6a5955cb15fbeeadf87d77e4abca6e9c092619` — Darksteel Juggernaut
- `d23e0062d7699bcf2cb19142dfefb7ef3cef601a9c34d85281b5fdf8a8630888` — Dauthi Warlord
- `b412d4dfbc7e655212a92690e10b04d423075af5bc0493bb63e0dd7e28c4aae8` — Dodgy Jalopy
- `e83254c0b4e4735d8b5a3998556a1b4a60c196c8eaa872240b7c69a29332cc16` — Dollmaker's Shop // Porcelain Gallery // Porcelain Gallery
- `c8d7a5804ce6cd87814410c657e2c7a266077bf4ffd267deb7b32e74d083a2e9` — Doubtless One
- `12166fc6ccf7b31a18c6f95f103288cfeb5ae5974dd34e19550717a37ed9a454` — Drift of the Dead
- `1228cde147be9c98da0ed7a1b8e64844b7bc89926e37e39d7cf8209cb1e3fdbc` — Drove of Elves
- `df44a8dffec8511e1518efffa9bfe36b2710f32e76f21004e55daa7283579c1c` — Dungrove Elder
- `4300ca3e630a336d79c64c487b4ad76de4037643d855a15b3c54a82b5cce789e` — Elephant Resurgence
- `64b263916a32a50bd37317c18cd294b076aab9838ba6926b48401939142e4d93` — Enigma Drake
- `6facdf2ff8c5cf06e15a3d544ed88955515e6cfe99caa7af0ad34bbdc5a2c697` — Entropic Specter
- `910fbe4833d49233c72f16879e81a99461dbd63821768cb5fc97ada61b7049bd` — Exdeath, Void Warlock // Neo Exdeath, Dimension's End // Neo Exdeath, Dimension's End
- `193a49757c001221b44da6ed6e2d93c38a0ec4d27eb24f0b9ce05a1a49882dee` — Faerie Swarm
- `498995a605e7296259c91cf1bbe9923fd6ea39ef27437c09cef388633c30226f` — Filigree Attendant
- `34e6fbb5f306caf14fb95f62b146969a2f13e25a1d97f6225ff14f4545d6f7bb` — Flora Colossus
- `50176617dd7a31d807478500a391445e47b03125fef1d770e3d0a1def8938f69` — Freedom Fighter Recruit
- `af33ffda537da3cf3338ac66a44c3f3d8a36f1847298c2176bf2cb6df6202aae` — Geist-Honored Monk
- `1186e59776db9c07049197e9181e727dfa16e23c5718f715067bcac50851a175` — Greensleeves, Maro-Sorcerer
- `de78c89470d3368bcf61e8c705098e4f0e1730581f9bb38fdbbf415a8cc3bc5f` — Hanweir Militia Captain // Westvale Cult Leader // Westvale Cult Leader
- `b880fb696bc7f4412690a5c435bb1a47712ddd64c4e6adfe4170c5c57ceeb578` — Harmonious Grovestrider
- `950acc9ed0455efd97fd1c7785b3989e8e405310f7fb9e2916ab9fa821d54bdb` — Haughty Djinn
- `bdd451f071f4d3ff3d728279098ddebb5c7d673d590689102fcbe4a280c11a11` — Heedless One
- `a641a18a00a71768a09f6a72bc3e58b68e30ab803a2209ea0a3e150cd286a6ee` — Horde of Boggarts
- `3579e583d5a8680164df8c446a3b12e0fb29b5079659a70c0e934c18202636ab` — Invasion of Kaladesh // Aetherwing, Golden-Scale Flagship // Aetherwing, Golden-Scale Flagship
- `9dbaab90f936f39eade780ca8742840463a8e4271bbc7fc1e15abd0fbd84ce37` — Invasion of Lorwyn // Winnowing Forces // Winnowing Forces
- `ddeb0a0c7e6652d461f02b5d54ad375fb9280aa94d90be1de90313c197ce8567` — Invasion of Xerex // Vertex Paladin // Vertex Paladin
- `2063ad2b571f500fd8213e5ae1747811bace6c9466c9353d9783330678d5634e` — Ironroot Warlord
- `ca140e10d286f108795b363f9c79cde3c5ecbf75d67318b0ad732afa064b418b` — Jagged-Scar Archers
- `05de6dce7d99c876bb582e5d7ee99b0b7e5fab96574bcb0924634ad899534351` — Kagemaro, First to Suffer
- `dc9f889bbe9f39339df82e8f8b2f5a297ea4afd9d9bb3adbeb52cf63dba196db` — Kalitas, Bloodchief of Ghet
- `453a9b35a270b6720bc1299a25759df77c88a95f52e61344ee8f37f4ffdec972` — Kalonian Twingrove
- `ea8d431e7ed1cb2391b5d54962360a70ffb651c74cb75eae94952414b5c160cd` — Katilda, Dawnhart Martyr // Katilda's Rising Dawn // Katilda, Dawnhart Martyr
- `dd9dcd14f7d792262ef3e67bf1b08b90823098dd2f0242e5ec14d113998d737d` — Keldon Warlord
- `578357860f5555e21542b73ac2ce96d747bc3ff0ca681babf36e7207ca89db46` — Kinetic Augur
- `0e2752abf961a42f272a7dbe2d858225e092f7a87e4c9628e904132bc3096605` — Kithkin Rabble
- `aca951e3afa5378c841fe52fbc29cbaf0175e9be86d17d7dc88c77b1d030b641` — Kiyomaro, First to Stand
- `c352530bf9659a9ca83ad638c2603071259afc8909214030e571260019bafdcb` — Kolaghan Forerunners
- `f86bd098a9c933b8cb4d2876459b575c2c72f9aab0bdf4c0bc20c76f6237496a` — Kraven, Proud Predator
- `ab5c0c65596498b491628673c71369cfff54a1f4c09cf4bb0ce79e99dd9f3ae0` — Krovikan Mist
- `57cb1f0503f66fe18296921743dcfc351213f04c86d3dc04bb7801e4ec646f1e` — Living Lore
- `120502c892755d1e0497ba75bc9ca98aec2edbf37f7de0a449fce477c219ad64` — Lord of Extinction
- `a91f1c2d8ca14e51e4e96054c3397945b7c9e7da863246de6c1922164ce5f53e` — Lumbering Worldwagon
- `20ee43f49db303c10017140f03b00241cdad92604b515261237dbe76cf9d2003` — Lumra, Bellow of the Woods
- `aa48ed4c410f23c7eb3a0137086f84a3667ecf6553627026ae8996dc54e5d233` — Magnivore
- `b96c58f7b6edf7e782b91d6b6ef036512b85d078b3ba8a90dff51c313a4670de` — Maraxus of Keld
- `217fb1ddde7612eba2beb8c6b036fde328df50f042d9b0a6202ef281d9839887` — Maro
- `ecc018cd85109227f2a64100d49999c2c25dbf886c02e839d1d859c59f820546` — Master of Etherium
- `5e02724128ac8abea422067fdc2aff5473b055125c98ec86316e5ae70eb4bb4e` — Masumaro, First to Live
- `f7b0bd6e7674a470a31a093b093e3eceb754e4c22ec8b38e50fea3069033acb2` — Matca Rioters
- `6988211bf7d112e860c3ed64661600b9732254fd7dd4941063c6d24d1ccf4345` — Melek, Reforged Researcher
- `7e8e00a59634af8d31a8817b9dac8f0f088d37e062868af285683fe7d5141f8f` — Mirkwood Pathmaker
- `89ae1b3860bfe309ff7772472e2c22e54d63637586869f338ffc08adc7881821` — Molimo, Maro-Sorcerer
- `f3584b30a00c056dbabdf3c66a9dbd5cc617701682884b59e67c9519b5c0f46a` — Mortivore
- `135d87526cfe84a87e3e1529224f90a1822d8ec62a202172c459dd873fe31880` — Nameless One
- `1e2b71d76644e91c108aa7d3f58d2da61a483f3d668e499aeae432b2ba3eb6eb` — Necrogoyf
- `09d34e2799ddbfa20a9d8e635c65672eaed19125d884301d928ec70be072fc31` — Nightstalker Engine
- `b55f851785043e535389a898ffac430ec022a6083a815121a178e5e5eced25cb` — Nishoba Brawler
- `832d5f42498fec4b98f13b7dcf81056355b59a52d6cc9f14890ef65347f8e122` — Overbeing of Myth
- `1c4f6a63e1dc81ed35fdc230ded2e6429f1e7aaef45bc45eb59aefa84fd6e216` — Pack Rat
- `6569a5e03f4d822347c18ff75b4dc3c2b46510264b77ae703c58fd9ad28e9d2d` — Pallimud
- `1c4cc8a012c0402405b3a16a7fa8ec09248f7da7a4717ca63df5220b51213d8c` — People of the Woods
- `c332b376fd35ae11ad08b79f5e19b1f042f2dad0a866ccdc8f0e6516c83b627a` — Pestilence Rats
- `c7c3229fc194b88768c9d8387cc6ef3e51a2509c2d89be607e656f24a4917314` — Psychosis Crawler
- `792db31722b8cb527c098f13d80e80208fd0ea4f3688cb759aa76a01316e0aa0` — PuPu UFO
- `a665054d172c23b89f886f261b792e329cfcd6d6e5dd6713b71efbcb658016fa` — Reckless One
- `130c082d11c1fcea49c327a54661bdb96011ba41f31d3ba57eec5933390e5d80` — Regal Bunnicorn
- `dc0b1f02d851eb4f19487eec444b7c6874cb276fd4499400a34510f6b94a63a8` — Revenant
- `d0fd46b844559238e6ab845609bac06216ac56ab193d7ba0c35fab6a618a8adf` — Ritual of the Returned
- `f0a773b2aaa283fc7ced61f061f2f6e38ef5db9929b08f7215d6c0494e96b9da` — Rubblehulk
- `76c3ceab3f730d4b6b514f84c1b47c184e8637fd2e3ec98a2111f9d6e3981033` — Rusting Golem
- `721ec681d3c2716d3dee2d44b16e9633045a6c16444ac7c32a10f7f2a65336a0` — Sandman, Shifting Scoundrel
- `0411c909755140b11af25baac3f377591aceaf4371df4117860ccbf2caf9c009` — Scion of the Wild
- `61e56cb82fb94615c7f35a4cd0222308439a05bebb6fb6a787f39aa2c5b0058b` — Seraph of the Masses
- `1374d31a8cbd843d0d36f7570454d99b74478d9d1f44eaecad26a6fa08040fbf` — Sewer Nemesis
- `5464be76e5b5282e96ed2188b76666809644de0a43b587bb2203384d46c78417` — Shambling Suit
- `54ee92a65fce49aa4ce43481894d2764ea51f3a243ff3098e7d331ec3c8110ce` — Sharpshooter Elf
- `f02fae2fd937c720414ff178dafad19f9e4735dacf6ec9d9897922214a677dbe` — Sima Yi, Wei Field Marshal
- `15ff9f761fbd4e338180b30c404d70f8f35d03a73774ade4e0d3759e26218d63` — Skyshroud War Beast
- `f8b61146751dce795abf0ae83c718f65486c798f3f279317b98aabded7d6c7cf` — Slag Fiend
- `6fdfd615dbb2947753ebef527bbe619f7046aee4b1642ac74d4ebb15c4355e1e` — Smoldering Stagecoach
- `c3a37671f2fb13e929618f3772efd732fdceb2b6c22917d9d744201df2d417bf` — Snow Villiers
- `a208228a79beb3253e1e2b3e93e763bf3a8e391eb922f8fcb6759cceb9b74881` — Soramaro, First to Dream
- `a868c5e6e45dd56cc299fdc37a2807264a67662ed81e844d6259b07ef23f176f` — Soulsurge Elemental
- `a8834956cedbbe24a7dcb37b8ba76dd5e8220996979a5fa565d1cdc076f86631` — Spellheart Chimera
- `7d93a8f32d2e9ced2c762963c96b052711399dc511eb1215c4164c5a7f831b75` — Splinterfright
- `9138fdbd2a77a752670dae6ffdca9f602f71e3f1faa1bab017a4f614d29ce731` — Squawkroaster
- `8f595dbef0b17dcf9467f6d866f1465f0c7efd6c97d0d183f641ace1b9a0f562` — Squelching Leeches
- `745dacb65e8c4b5f1da26cc653fa8056c5efa38ab9b0e2935e0de9702e895c30` — Sturmgeist
- `fc1a3037d39f1f9edb029bdeb1a7671b88b88ff6c639901f81e14a5ea485bf37` — Sumala Rumblers
- `33ba30b54a4ea29463761e073230d9d3398c7eb2982ac655ba8bad4efb0a7b9f` — Swarm of Rats
- `f7f0a1a59efc3e78ba6b2c1724c07d256a3226393abb7a42dd38c98a607ec2e1` — Sylvan Yeti
- `753bc2e3136cd2de8ceea2077d9a7ba7cf1c7ef0caebb6e2c6b59fb9758ade0a` — Terravore
- `b55aae5b6768723200d3c4e4c73f67bb0c699c18022991199911e8598a513af7` — Territorial Kavu
- `9833ed8c56c6014bbd400d53cef94bcb2534c1564411bb3627dda822d7133cb0` — Territorial Maro
- `64e9bbd8718b3dd568d733b37b912c50587334c9eeb53e15721d6911b8e09f6d` — The Eleventh Doctor
- `f9393f0fadea1aeff38f8cbb9c9d11e3c43094a9054c00e24281e405e19d61b0` — Thousand Moons Smithy // Barracks of the Thousand // Thousand Moons Smithy
- `a369163644d5c10d4d1e757826cfc4bbd00646fbd3f8ddab0699aec7d56b4bea` — Tishana, Voice of Thunder
- `ea04d016322485212e6e3b3683f567176de0fc849d67f1b699cb740b5a5e796c` — Toph, the Blind Bandit
- `7dd62bfa042dfb82b34a4ee0bbea7e88c62552976e283c9136fdbf4742ad3a7e` — Towering Gibbon
- `a2b0c1e45c5a337f3d2d4a85ce0c1064b53aa54fe9bb528f4cf07150228e5472` — Treefolk Seedlings
- `959ee8b2dec7ebaa76b26cb03cc4ded58112f3637d7ff1cdafffa4809682a51c` — Trench Gorger
- `096532c9276d385f2f4f89eb875abb6f779cd4636bffb5453fa998fdc07038a4` — Uchuulon
- `d4aa08aeecd78a5fe6e71c8be644267ff68763218d44979b4b56a23bffb43b73` — Uktabi Wildcats
- `06fe8c3d183151e9da5c838856ba4cbcdaafdcf70cee6a73c40b8d9758da5aca` — Ulvenwald Hydra
- `41995f1267edc6ef0754974cb0c7797f6a0dfb049843a8c21df004ffc73ff394` — Uurg, Spawn of Turg
- `0e2c9081df758bcd252a3472da5c1486f2f22b9e2319c857d0c6a1c3741dcca0` — Vernal Sovereign
- `81b68e82a9da63cfea5413fbfe2f422628329dc446a8f27b0a83dea03a051e0f` — Wilderness Elemental
- `20af08bf5b62b79c5a31751a2adf88fb7d9c6ce2591f319de9b76436af429e1d` — Yavimaya Kavu
- `7429086017e1214bed5000a0ca2e3ec66be44df725468e186610e54a9a2d4fe6` — Zendikar Incarnate
- `c3106c3c5060e9ddc7cd7f8eec0906ca43a92d4a6e4603f2853c67c00c79b256` — Zurgo's Vanguard

#### Newly selected identities: widened copular subject-gap analysis (44)

Each identity below selects through
`copular_subject_gap_relative_clause` -> `PredicativeComplement` and its
attested concrete arm.

- `b79afe7c0b159e09bb75dad4dfb3bdd32666a78bdc5d4162f29d6afd21e2d13c` — All Is Dust
- `0a7ea0c61837eeb6813796a0b6e052ed7a2af03ca175c0e55b58fc54de6d6fdd` — Blow Your House Down
- `c7746f95f3b7720a14db731e961c754a7ae5b76a672c0640aeece39cd4e761e9` — Brainspoil
- `ad33f94e02ae67ae19da88328b27b2608e86c3e2c9b59cf176453f672030556f` — Burning-Tree Shaman
- `ac9b94c5991437eba2e7917392a7b829d165d88aa7ec5a1c3284dbb8a6e99d68` — Chronozoa
- `7611856771ed0fb43aca035fc01adbd1f99c0087f8fd8b1d275c7df0fa2f2bbf` — Cleaver Skaab
- `4c9b0147e2d230c465571132d81bd363c410570729f739f2f5494ab832bbfcc9` — Collision Course
- `49aa065494e6923cfdc5a1dfc5f2a4a6b69b9acb260d972fc03bc58ba0b07013` — Covert Cutpurse // Covetous Geist // Covert Cutpurse
- `f6828f09fc45e1f8e5e5487210360040300ea2b5fa79572343b962300d73aaf5` — Crackdown Construct
- `f01eaea2f43a16ca36a2fb25a6b0b3fdb90d935808b1f4e6a0f4a4cceb2abb89` — Doppelgang
- `b8993db20c0e0452ce033e43bc1ac3b1a08580d532c644ad75a7d790d8f999c5` — Downwind Ambusher
- `dcb9ef18c475b1f753339928b218b8e844498bcd4296b6122d28505fc5f2d702` — Fatal Blow
- `f8048494c789a002fafa8bac430ced1225e8e1ceaecb8118c0354aa72206dd04` — Fathom Fleet Cutthroat
- `6aa66186ea9dfd48140eee7b5565dd5e4ec5db2e51f80c5cd610e6daf7990a2d` — Fencer's Magemark
- `71b3642dabfec1496d5de424ecee15fe808d4c8b11dc1d6e1127fdee57fd7053` — Final-Sting Faerie
- `99f5bba6a3e03e88aa4c3b7a348d4255d9769e550ebde0bca59bf9a5d0cf5b7f` — Flamescroll Celebrant // Revel in Silence // Flamescroll Celebrant
- `7f6078ac682da2589dab28a1f87f9e4b4f24c44bf459e9215e41cd199ec09d09` — Glyph of Doom
- `d8e55daf248182f4257e2bee863f35b72fac7b791cbb0655622a2246bcd690d9` — Guardian's Magemark
- `2bce3adff29b2903160fdfc761df2493cf1025ac485edb88a70c51ca5067aea9` — Hand to Hand
- `483a265da271276de3e9a99ae150772ad51f89f40b4e108f361f8b2720a57b98` — Hexgold Hoverwings
- `148fda9d489abe6b9924e5b1aafe736f0a2b6e4c8e099cb8943a625d1d94dd02` — Hooded Assassin
- `a0c6809e30b12ef8f3f5f1756059be252f964cd9ae275480e683bb0f6de7dbbe` — Infiltrator's Magemark
- `d234ff4e3d49adcc18463442901c4248c6a3b49a22e88e3740514a0126f384d9` — Iridian Maelstrom
- `8774e05eebc7579ad1556512c27e105f00a19cbc47fb4e428b65d2ae3ef5e673` — Jarl of the Forsaken
- `45d3957122a7f13bd8c255091d04e494cdb3a9e44238d75aa3c874c097ac3169` — Lurking Deadeye
- `d22e73a958c83701e0693b78e44c685f54899a153199e1dd1ebb94bdef2fdd08` — Manticore
- `0ebbff9c74b1889f62ca675f5a14478e599b9faa568f07e974b473b4dd459947` — Mirrodin Avenged
- `ab2b733684a84ba4958d22bb126ea4507506756a95d8dc2d9bf63e931fbbba71` — Ogre Siegebreaker
- `e0e3c2aeb1cd45082166b454e82c11a378ddaaf9162b27c286951563eb23a16a` — On Wings of Gold
- `8ea3f86f1e479b336a9be758c22b48d1e68193384c6c2ad7c56995c28785d7c1` — Parhelion II
- `8fcd0418fc163013fb36f774c365acd953c106d650505833501e46fae20284c5` — Parhelion II // Parhelion II // Parhelion II
- `8374e0b0e5479ebd1b05c31d82d64e1bc113817e1cd2c98d21c0da6e46543097` — Ravager Wurm
- `d60c80a871c59d86678190875a829ba64314ac43e695cba491fe200893033d6f` — Rooftop Assassin
- `a9b9769f545b140e6d03b6bdca3a67e9b5e0bf0e1b05dd830492c46d4efaf0e3` — Runic Armasaur
- `d83ef9a4d03670d93d4079e5881526721e20262dba0dc3a68e84eca29df99c75` — Song of Serenity
- `472a53a2b6357dd68645b05452b3a393bf89a4ddaf6797879777b0fba78891ca` — Stingblade Assassin
- `34d985a6486ca4d2343a0fdd734647ee45aafbad97ad0005e0f7a70b2dd33256` — Swift Demise
- `d9e642bde057bbc16e55eaa86c4928bb56b631ddfee6dcc3e007cf0e628d3b9e` — Tsabo's Web
- `a28adca7331c2a3478b64f744bb62b3cf60152567496979e97fcfad9f12be151` — Vraska's Finisher
- `bc5e961c613f7dce16ce34ab943d2564b63c66a8855a0e4b254c4fc20a9cfa2e` — Winds of Rath
- `ac1204e2e336019d8de49e8e6c06298b4323da12dac2d433a2ef9f4f70b86cee` — Witch's Mist
- `cc46c4263a028a3f2f1578759abba2a7e02a51a2bf273e54dddadaecdcfae553` — Wrathful Raptors
- `f2eb8ec909a80aae67dced8a24237ae1fd6a98883fc95eba8a219b5c25bf0dca` — Wrathful Red Dragon
- `fc28aa78c2ec396bec34ceb08445eee15b926938688f04591287f43d11c2d4b4` — You Are Already Dead

#### Newly selected identities: ordinary copular complement analysis (126)

Each identity below selects through `predicative_complement_predicate` ->
`PredicativeComplementLexicalVerbPhrase` -> the existing nonprepositional
predicate-adjunct host, with the attested concrete `PredicativeComplement` arm.

- `10e0bd8130e283aa484fec2caf564576710b5f1864e2290031f069c9026d28dd` — Absorb Identity
- `b6fa255f1651aa40b0d6923de58ed0dfb391ac6a8dba949b1918e296b931c4fa` — Angel's Tomb
- `1bfe0708d90fd9a38cfb48096417dc269ff01a88b0de10f5c85bc821f4d30564` — Armed and Armored
- `ca11553a25fddbfaaac878b586c55228178f7952df24a652dc497f8ae4a6038b` — Atarka Monument
- `02ee72e52a3cee45c376efb52e853d12a2c743c06d796b32c08b918e362c4cab` — Aurora Griffin
- `7e354480866d669c27d8d747ebf131c491d4d437549dda1d6660b8878e368373` — Azorius Keyrune
- `0e106a87ee1953f33e002ccda2a0938853cec1f0e625eb69c5abfefc4ba742af` — Blind Seer
- `5daec33595fe1b2e05be4839893287eabecbecf1cb4b73cb51b10885de14749c` — Boldwyr Intimidator
- `135ebd5276eaf9a17b00750e6d9018a01574d3c929c102dc8d7f8c8ff76331bc` — Boros Keyrune
- `66398a7f2745f70140d670567f11a82202c19a83482d7565c5e84ed62dda9fe5` — Caldera Kavu
- `5f293d34fd7f2c32df41a3edf3a33e09488ca6da59674e0e627c5e06da1aa9ef` — Capenna Express
- `a48022e68cbfe2d7f0f797e24d0341830b944b9707466746ccdba50e372bc46e` — Cerulean Wisps
- `82eb39e1293a1ceae92c0412c199a8fbe7b2a2aeb71667c108807addb9536e3c` — Chimeric Egg
- `f730e46459c4282dd422ca87ceb7cb41aa136a525ba95524c9bc796e0e22fac6` — Chimeric Idol
- `60eb593b5cdf61ca391738693f6821ab283f8827f7de2d978f85d5de26f4dfa0` — Cloudchaser Kestrel
- `6c2d982b8e7e19638dfea518afce56ab42a93b8aeec520c5276cb88720414d3f` — Cyberdrive Awakener
- `31679503467ca84e3e5b36a05a4d500353005566d08c43a7637640a92ef0d3dd` — Cytoshape
- `37e9064f49d4f6f52e23b8881d456d8ec67debe8dadfaf6f78bc481612bc0c4a` — Darksteel Brute
- `5cdab67144b6e1a34500961f221c115629e4d6f9b44285e3bacfd479210d485f` — Deceiver of Form
- `9e6a1e97086dec1c57f281f18b1b66189a2f182839d43be24b6bbd4fa7e76604` — Deepwood Elder
- `0696510823b2f9dc2468e2caf794d1a25f1f31aa4e54632b37d7b207cdf421c4` — Dimir Keyrune
- `22346e2e8fbf5d549c68ffe9cbb5cde3da1764312b596e924320c192045d4f51` — Dire Mimic
- `e4f357ce692b79c13aae090f9fba72268657d48383b6ef03a725d71eb596fecb` — Disciple of Kangee
- `8d49204a49d17b75679a1ed6ff03642ca0573d767e1049b2b1c9e4ec6b87161b` — Distorting Lens
- `cc005cc34084b4f4e7d76ce064784087dfee9ff888b3fa15f4f39bed30b61596` — Dream Thrush
- `1f570ed8b4eff03b5b70c53f93861cd06fc751765a29597ff1250b20efe58ef7` — Dromoka Monument
- `3110587c9670bb77bd08b9f885678d4e373599b4896f58e726686b19a5b191d7` — Due Respect
- `c9e558b94d8affbdb61119aed4ca2c77eabc8e32becaa39da838bfd19991ded1` — Dwarven Song
- `cf1725aaa619b938a4c2b0697965f1729eb807625e84ea7b43907c91e7089bd1` — Earthrumbler
- `69f8feebb0c2f5e4023c16d30df4ad11b08cd8994d282c02359dc1eee6b6677f` — Elsewhere Flask
- `3a5917f8773f776d01d9763ada7341a0ee5305a62f4db4fc5fa6ca0c837b61c2` — Ensouled Scimitar
- `e8c47f47359a9aa0d194240ecad7b936a3e38ed19767bda88a7e9cfc6c747b1c` — Eye of Malcator
- `3932d412cd9ecf20e8286ce57cdfa486c0999c59f97939294d7bf6631f26ce9c` — Firdoch Core
- `f5e793c1dcec7ff769ef1317a71193a3b393e8d84d69d9ee577951de4acddb67` — Fleetwheel Cruiser
- `ece041b04e2e6d031c11a8c94b699763e6f95022d6119c52ee59851c23af461a` — Foriysian Totem
- `74452e307ec665109d6c3e5785d6e447ad5bddc2574996733da420775e37ceb0` — Fountain of Ichor
- `a844e28b05645af32bb876800336d6d5ff3dc0e792dfb1a3e29437360af24cfd` — Fylamarid
- `24472500baabb18f7f1f816a1069a3c394afb57de593edafd6a5a28fa93857da` — Glint Hawk Idol
- `9e5b16eba38d83a88d834d3c25a842c813d074a550ede56024abb15f4f8091ee` — Golgari Keyrune
- `702509e0e5f34e219bc15ca4557461ac69fee203d1f5d3fe728887d2585a6c13` — Grixis Illusionist
- `02289c91d2bf54de25382666ee4917c597dc061cbfa299b8b3b4754f95ef7ea4` — Gruul Keyrune
- `b1c0a1818c5f391675eff4577786e910dcfa764aadca98424a6714bf31bb9d2e` — Gruul War Plow
- `ec8187d4ab5467eaf5e6ee49b5b83c2c4433a97a3400d12473bcc4c1d34d7ddd` — Guardian Idol
- `0c7a82b49f042ce89f57d6be8d40a1f5fea112e91d658c1791a85749ea4152ed` — Heaven's Gate
- `50f491a231968fc3b743dd3b799b20dd67839c16ab87794d580d011ccda23f51` — Honeymoon Hearse
- `f8f1c8154243abd7942d87e4cda04e428ed84eed75e5b91d8590fb613f043ec0` — Identity Thief
- `ac6403f3e50bd881b0fa5b9ecb1ee6c4f1c06c002ccf7b972b3f3c9730408fc8` — Illusion // Reality // Illusion
- `1a7b57061f1a3a7d4e0de56da2da914d871a670f38df3f0dc9bd50945d5582db` — Incite
- `55186310911dca6398342c28ca415913c861fd63a004c053a5e95c410f7f76ab` — Iron Suitcase
- `61291787f892059b50a95b59df6be43921e5961c35702783ec6f7d6553171da1` — Jade Idol
- `edb1be5bb2ef968587dae790e78de365885812ce8d6ccab8b9b529886ced823c` — Jared Carthalion, True Heir
- `35298a00e66e9513a0aaf8dd34eeb382c3e19532365318c597de20d694aed964` — Jinx
- `a38c321194da6bf2d5fe737598d143dfccbc48e1f3446edc8f3edee315e1ae5f` — Karn's Touch
- `4f4b2376d1b3f9225f3ff08422858c682c5429344c4a5291efedff4f03d157f8` — Karn, Silver Golem
- `00a506330815b5dd6d77cbd9594e12a148a99d5ebae3b8052b074dea767f220b` — Kavu Chameleon
- `9c4dc5089e7e61ab96883d72150f3825ead613b64258b031d399aa871e08325b` — Kavu Recluse
- `223b56889524dd683597f3cace9c203e0dd19e039d2f6a65ccdc0949b0448654` — Kolaghan Monument
- `4de77786298c75be9a875f903f01c8414c7e3e14110657e327fc68dd48a6ca7e` — Kolodin, Triumph Caster
- `e55b7d61e0bfcbb3e181dff64844393a53a1a726800cd4d26717859ffd926749` — Levitating Statue
- `51285e4773b530242b6dfffc3146b3627a3bfc1ab26fec2392fc4d9393a69673` — Living Brain, Mechanical Marvel
- `74896679e75238eb4fe17a69a6ae20750e06dd13ea22081f23b14c800371e294` — Meldweb Strider
- `4690b2b8aa132e005740f4d7b409a7d9c6d5d63a802804835231ce71d858efb6` — Metathran Transport
- `c2509005f036452fede2ad1c0c15bc0e912caa0fc5473e78aab147ab8f33ecff` — Mimic
- `daeb57a5443db2c45df7673e29f7d83f109a0a36facfeb3ad2c622180ca1fb34` — Mirage Mirror
- `4d2b036adc40fde01bd8d714f34ba52d05b3d64076ee6982670d9e60f48eae05` — Mirkwood Meditator
- `fdbab6a996f45c5db96c175273920d732d10752f14ffde4a2e52f434100039ef` — Mirrorweave
- `2c5f673eb0174a5e9076cb814c42a304fe4ff6c566961acfc8759158ddb159ef` — Mistform Dreamer
- `e24443e8710460d6700d686bef8ffdc7b635cde41f225c02dbdffa46fe1c5ce1` — Mistform Mask
- `487ed8d7917f41d304fb9087395395687a1c248905bb95655fc07fd3cd126237` — Mistform Seaswift
- `71a70c8b5b6bc786b782784730dc01283b5a188744b0732cd6dc89b611f8b66c` — Mistform Shrieker
- `f6318592cffc550097cfc00683b77da54c3357adc420e68ae8b426df5fa4f40b` — Mistform Skyreaver
- `f55b24b30213996211bb87238a309e7dc5d164c21dee92cd79064e71e4159050` — Mistform Wakecaster
- `78a0736ee58dfb51ccc50b5eddd1e400abc23ab922028803dbfa6d2a2feb8164` — Mistform Wall
- `cb73e9e8b58e1514c6d8e1b2ceb2a8854fd1a31309378a7b48aba2cc18f2f6fa` — Moonbow Illusionist
- `4d1f8e0d624cc8f5f72dec644bae1a198a417ae3078fc60ad5c3fc7f04f6d995` — Mystic Compass
- `e504fddcebb759af29427b2baee568e2c324310d9fe2965432c7d9c402ff6deb` — Nahiri's Lithoforming
- `0cd9d15a0944d36a4a61f6895de8eb7ca545c00f2008b36a3a89560f0162b63e` — Niko, Light of Hope
- `4414ca3a13340804ac18215f3011007b505528a7dfeca50db4ebafed7b1364da` — Niveous Wisps
- `e231bda29ce8e4a22185a937341a911db0e62e412bf4359b350aef56969a3c00` — Ojutai Monument
- `736e5ab998e44cf55b9ece54a343290674d8a6c10604ebebf76be953ff06fa91` — Omnibian
- `50ce44ceaa8c43c278386b72bb5d91c9e19dd4b70fda885e147add67d8da107b` — Orcish Farmer
- `b841fc331b219bb5a169c97cee43333eae4ff99b0032aec584329143ce8404b7` — Orzhov Keyrune
- `a790c0ee3835f6b762b9c87bbc8f43dd6bc954f9c23b49e04b48a5b92a5c7426` — Peacewalker Colossus
- `5fec97a8dcd9a6074cc259f2b5f0704a41a73f78608ad99cf784cb481b9383ed` — Peacewalker Colossus // Peacewalker Colossus // Peacewalker Colossus
- `73d549738d33f6b15f6b88bcc8b6b6b6ac09f5612e046e640d8aca0b039e737f` — Phyrexian Totem
- `9007dec528b88fe79924016657338885bdc2839f56b40e65f3af4ecaca4abf66` — Prismwake Merrow
- `e4287ee4de8c4500e4d815f47fc9cdb6821cb982cddafd66d70acca0ce1ea105` — Quickchange
- `fa3b80325c457854dd8c12698df87380cec76251ad3e4eeb37aa19e57c6c3adb` — Rainbow Crow
- `a36a6b6d773578ea203a24c66b8b4cfffc06c59e937529b63df573863b9f448e` — Rakdos Keyrune
- `f16732b479760f3486701643541ffb803234ca2c264a742f836111e7636350fe` — Reef Shaman
- `65b9cfbe72e6f802a28ddc87ad8adbb44e72d9636be204c90b73605721580c3d` — Renegade Doppelganger
- `91b35991456434cd82bf7294be466df942fff22643d0bd3dfaaee811f39fd2e9` — Sanguine Statuette
- `6171352a7005fa7791004939f06573d6c3b3459a93cb9f95a76edc7887cab080` — Scrapbasket
- `8ebe754a00c48fa62f55139fdc4753f89f5d437f5bdaf721e72a29aa5e586281` — Scuttlemutt
- `5f62fc0da038ff9bf2de0cd7cf0dec38c03e79b141140d42380d6abedf750548` — Sea Kings' Blessing
- `40f0db2e8fcc06c6fbba509fbe4f1c07a67f55c824cb5eed8d1cd9146b736777` — Sea Snidd
- `3f4881bcc69c9c8e8fbd724f52765681c98aa747eae1bf4bc27f3813edf7db54` — Selesnya Keyrune
- `d1be19252b3e0ca8af2577a65cee6629a3129e146b4b87f9ffbb612b59b396fc` — Shapesharer
- `7eb7c07acefd9c24dc54b88d3e0c320d11fdefafe518c48303d3f3b29a773db4` — Shimmering Mirage
- `3f68ba7dca2fcf6d564fd45dd905fd7a3919ec0302234f25d524bc3578c8389d` — Silumgar Monument
- `966bf6e6a69622c850f50f535d966bc624c63c665aea121632dfd6a78f93828e` — Simic Keyrune
- `88b9eb7cfbb90e9993ddf48a985d72cceecdb99c68e332652d7a834ac1b84b36` — Singe
- `a00e13c6125697ae72cc4a3df16f5172ac25556d3ecdb45f9af40f3379a2b471` — Sisay's Ingenuity
- `91efb11b90a830d6c99cca6896e1856518574a70f8c24813af5cfc6244eb2e55` — Slimy Kavu
- `7a5d9a6a47f86baf52235d86a9f598e4cf4d15d3cb106e603144597fcdf603d4` — Spiritmonger
- `3cfa30c58313bb9f3f05782a552c1091977fc0176c697a0b0b34b004add6a460` — Spring-Loaded Sawblades // Bladewheel Chariot // Bladewheel Chariot
- `f17f99b0ef4f497030ca2d17db4dd799ab43d71be67e37bb4fb9f19a98483b2b` — Start Your Engines
- `8a10b76f744c60d37747cb2e8a95e83dbfab98ef0229de04fe499566427ee476` — Stuffed Bear
- `ea7c9afd56d54c7de2698f2a30d0a83ee96a8bdd2b881b1713f35b5c3cf67f96` — Sway of Illusion
- `5f83ebdaa30efa06d5401ff6ae2d480bab8ac233f8985d0943d37f3dcfc23df1` — Swirling Spriggan
- `ed08af6530904d609abb3651db7c9757dea4f2459789127b09415ec6e6a8d619` — Sylvan Paradise
- `157a5503c08131ad6fe240b6e9fbf3a89084640c19429a5279c5f0eebfda67e1` — Tangle Tumbler
- `3363faaf6b0131c950a1e770ad2898d8a55cba19c9bce5edb2f89e970647294a` — Terraformer
- `e15de54c130e2c3305292b6665f7bdcbc9ba457299b1eae3564ddb66d34db21b` — The Antiquities War
- `fd9363510803046aa10e8b21e79ed08d64207bf42aa6b6e04f61bf62400b58c9` — The Thanos-Copter
- `7773fd18729b9be1a20db1361c273a829876f3fb349b074c4f9a5c8c003e231b` — Thunder Totem
- `7dba4b06499cc31619451b1edc8d7c197692d6c63fdb80506737aa0b5a451fe2` — Tidal Visionary
- `ec5a9b261473c013ea67ca78dd71815ff75654a5b3c7c5663176dc7c35c4aa94` — Tidal Warrior
- `44029cfbea60bbdd818005eb331471b9c53c48d6b96f5a39c64ce47018e79b8c` — Tilonalli's Skinshifter
- `2ffccc5a632f883628ef947e15460d46e7ce9d0d43fd04b251eaa032e35136d3` — Touch of Darkness
- `7c060a4ead15ee93044d253f4b7f978664b3af6b71cce42931aa9a850b872124` — Toymaker
- `c2c6c946db7098f63fbd09fc2fcdcfcd395768d30b7a7131fe8b819fda7a5306` — Tundra Kavu
- `94999b78530627f01691aa0341a42cd64450624d54b234dfa056e238c349a548` — Unruly Krasis
- `0a3588fadacfdcfb7ee7390405cd7ae2d19ed36a84a6fe5d67b1d7ae18d1f156` — Unstable Frontier
- `f95447635cb948e396c4aaef28a5dcedd95e8d1fce7dd0e203a107df81e06f76` — Weatherseed Totem
- `d4bf9d60b9b2c474dbd2c3182a5ca0101845d644f6d9077a471c34c7a439a3dc` — Wild Mongrel
