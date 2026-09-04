---
needs: []
---
# Durations attach to the clause, not to one verb's frame

**R3 — Group R.** Authority: rewrite ADR "Ruling: adjunct licences removed;
attachment misselection is a recorded class (2026-09-03)" ("A temporal, manner,
or locative adjunct attaches to any verb clause, matrix or embedded") and
"Amendment: attachment class is a declared linguistic property (2026-09-04)".

Defect. `OptionalRole("DurationPhrase")` is declared on `Get`
(`crates/deckmaste_english_v2/src/core_verbs.ron:79`) and on no other verb.
`Gain`, `Become` and `Lose` do not carry it, so:

- `Target creature gets +1/+1 until end of turn.` — **selects**
- 2026-09-04 coordinator ruling, was: `Target creature gains trample until end
  of turn.` — **parse failure**. Re-measure and route it to
  `english-v2-tail-keyword-ability-grant`; it is not this ticket's acceptance.
- 2026-09-04 coordinator ruling, was: `Target land becomes a 3/3 creature until
  end of turn.` — **parse failure**. Re-measure and route it to
  `english-v2-copular-complement-sum`; it is not this ticket's acceptance.

A duration adverbial is a clause-level adjunct; it is not a complement of *get*.
The whitelist deleted by `english-v2-adjunct-licence-removal` was
`AdjunctLicensed`; this per-frame optional role is the same whitelist under a
different name and survived that deletion.

Pinned shape. Delete `OptionalRole("DurationPhrase")` from `Get`'s frame and let
every duration reach every predicate through `predicate_adjunct_predicate` and
its shared adjunct hosts. `english-v2-tail-keyword-ability-grant` already rules
against the reverse ("the adjunct-class landing ruled per-X slots are replaced,
not inherited; the surviving `get_power_toughness` slot is residue and out of
scope here") — this ticket is where that residue is removed, and the tail ticket
owns the other half of the same failure family (`have_keyword_ability` sitting
outside `abstract sum LexicalVerbPhrase`). Land whichever of the two is claimed
first; the second re-measures.

Sizing, provenance only: the keyword-grant family is 1,522 units and the
"`until` duration/clause on a frame that omits it" failure bucket a further 365
(2026-09-03 census, `docs/tickets/fog.md`; re-measure at claim).

Fences. Adding `OptionalRole("DurationPhrase")` to more verbs instead of
removing it. A `duration: opt DurationPhrase` slot on any construction. A
`checked by` or `require` naming a verb identity. Any census used as a gate.

Glossary: Adjunct, Duration Phrase, Lexical Verb Phrase, Verb Frame,
Complement. Record any gap in `docs/contexts/oracle-english/CONTEXT.md`.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Standard constraints apply.

## Landing record

Measured on change `rmnwvppy`, lock `covered` = 17,068. The parent tip this was
measured against is change `kptwqplsouxw` (lock `covered` = 17,052).

### What landed

`OptionalRole("DurationPhrase")` is gone from `Get`'s Verb Frame and from
`core_verbs.ron` entirely; `get_power_toughness` lost its `duration: opt` field
and moved out of `VerbPhrase` into the `LexicalVerbPhrase` sum, so every
duration now reaches it through `predicate_adjunct_predicate` and the shared
adjunct hosts. `stacked_predicate_adjunct_predicate` widened its host from
`TransitiveLexicalVerbPhrase` to `LexicalVerbPhrase` so a duration and a
following prepositional adjunct still stack over the same predicate.
`instead_predicate` widened its host from `VerbPhrase` to `Predicate`, and
`have_object_control` widened its complement from `VerbPhrase` to
`BarePredicate`, so a duration reaches a predicate inside a replacement host
and inside a causative complement.

### Numbers, before -> after

| Measure | parent tip `kptwqplsouxw` | this tree `rmnwvppy` |
| --- | --- | --- |
| corpus units | 32,641 | 32,641 |
| selected and covered | 17,052 | 17,068 |
| parse failures | 15,589 | 15,573 |
| selection census, unique | 13,453 | 13,452 |
| selection census, specificity-resolved | 3,599 | 3,616 |
| unresolved ties | 0 | 0 |
| constructions | 388 | 388 |
| lock `covered` | 17,052 | 17,068 |

Lock delta against the parent tip: +16 identities, 0 retired. Nothing lost;
no re-coverage or retirement obligation is owed by this landing.

The specificity share rose by 17 units. The construction pair responsible is
`instead_predicate` over `auxiliary_predicate` versus `auxiliary_predicate`
over `instead_predicate`: widening the replacement host to `Predicate` lets a
clause-final `instead` scope over a modal as well as under it. Ten units moved
from `unique` to `specificity` (Blightsteel Colossus x2, Darksteel Colossus x2,
Emeria Shepherd, Ether Well, Legacy Weapon, Obstinate Familiar, Progenitus,
Sweep Away) and five of those ten now select the high attachment
(`instead` over the modal) where the parent tip selected the low one
(Emeria Shepherd, Ether Well, Flaming Gambit, Obstinate Familiar, Sweep Away).
The high attachment is the reading the adjunct-class ruling asks for -- a
clause-level marker attaches to the clause, not inside a verb frame -- and the
resolver is deterministic, with zero unresolved ties. Disclosed rather than
prevented: preventing it would need a per-construction exclusion of
`AuxiliaryPredicate` from the replacement host, which is the same whitelist
this ticket exists to delete.

### Per-unit selection neutrality

`ambiguity` was censused on the parent tip and on this tree and diffed unit by
unit: 0 units lost selection, 16 gained it, and every one of the 2,744 selected
analyses that changed shape is the `get_power_toughness` re-spelling
(`VerbPhraseGetPowerToughness` -> `GetPowerToughnessLexicalVerbPhraseGetPowerToughness`,
lifted by `base_verb_phrase` or hosted by `predicate_adjunct_predicate` /
`stacked_predicate_adjunct_predicate`). The remaining five changed analyses are
the `instead` scope shift described above.

### Newly covered identities and their selected analyses

Twelve reach coverage through the widened replacement host, each ending in an
ordinary `instead` replacement over a finite passive or an adjunct-bearing
predicate:

- Pariah -- `All damage that would be dealt to you is dealt to enchanted creature instead.` (unique)
- Pariah's Shield -- `All damage that would be dealt to you is dealt to equipped creature instead.` (unique)
- Empyrial Archangel -- `All damage that would be dealt to you is dealt to this creature instead.` (unique)
- Protector of the Crown -- `All damage that would be dealt to you is dealt to this creature instead.` (unique)
- Treacherous Link -- `All damage that would be dealt to enchanted creature is dealt to its controller instead.` (unique)
- With Great Power . . . -- `All damage that would be dealt to you is dealt to enchanted creature instead.` (specificity)
- Vassal's Duty -- `The next 1 damage ... is dealt to you instead.` (specificity)
- Razia, Boros Archangel -- `The next 3 damage ... is dealt to another target creature instead.` (specificity)
- Tekuthal, Inquiry Dominus -- `If you would proliferate, proliferate twice instead.` (specificity)
- Increasing Vengeance -- `If this spell was cast from a graveyard, copy that spell twice instead.` (specificity)
- Secrets of the Key -- `If this spell was cast from a graveyard, investigate twice instead.` (unique)
- Whispers of Emrakul -- `... that player discards two cards at random instead.` (specificity)

Four reach coverage through the widened causative complement, each a
`have_object_control` whose complement is a bare predicate carrying its own
adjunct or requirement tail:

- Nath of the Gilt-Leaf -- `you may have target opponent discard a card at random.` (specificity; manner adjunct)
- Grappling Hook -- `you may have target creature block it this turn if able.` (unique; transitive requirement)
- Giant Ambush Beetle -- `you may have target creature block it this turn if able.` (unique; transitive requirement)
- Turntimber Basilisk -- `you may have target creature block this creature this turn if able.` (unique; transitive requirement)

No negative oracle was admitted; every analysis above was read against the
card's printed text.

### STOPs and their resolutions

1. First STOP (implementer): `gains trample until end of turn` and
   `becomes a 3/3 creature until end of turn` still fail. Coordinator ruling
   2026-09-04: no scope expansion; route them to
   `english-v2-tail-keyword-ability-grant` and
   `english-v2-copular-complement-sum`. Both notes are appended to those
   tickets. Re-measured on this tree: both remain parse failures, and neither
   regressed -- they were never covered.
2. Second STOP (implementer): 16 previously covered identities dropped with no
   stated cause. Diagnosed at review. Every one is the causative
   `have <object> get +X/+X until end of turn`; the duration used to live in
   `get_power_toughness`'s own `duration: opt` field, and once that field was
   deleted the only adjunct host was at `Predicate`, which
   `have_object_control`'s `VerbPhrase` complement could not reach. Fixed
   generally by widening that complement to `BarePredicate` -- the same
   category the modal auxiliary already takes -- so any bare predicate,
   adjunct-bearing or not, can be the causative complement. All 16 are covered
   again, and the fix additionally covers four identities the parent tip never
   had. No per-verb role, no `duration: opt` slot, and no guard naming a verb,
   lexeme, construction or card was added.

### Deviations and additions

- `instead_predicate.predicate` widened from `VerbPhrase` to `Predicate`
  (implementer). Beyond the ticket's letter, which names only adjunct hosts.
  Justified: without it the 35 corpus spellings of
  `gets -N/-N until end of turn instead` lose the only host their duration had,
  and the widening is the same clause-level generalization the ADR asks for. It
  is what raises the specificity share; see the census note above.
- `have_object_control.predicate` widened from `VerbPhrase` to `BarePredicate`
  (review). Beyond the letter for the same reason and required by STOP 2.
- `stacked_predicate_adjunct_predicate.predicate` widened from
  `TransitiveLexicalVerbPhrase` to `LexicalVerbPhrase` (implementer). Inside
  the letter: it is one of the shared adjunct hosts the ticket names.
- Two glossary entries added to `docs/contexts/oracle-english/CONTEXT.md`
  (**Duration Phrase**, **Bare Predicate**), per the ticket's glossary clause.
- Two `duration: opt DurationPhrase` fields survive elsewhere and are NOT this
  ticket's residue: `transitive_requirement_predicate` (medial, before the
  fixed `if able` tail) and `contracted_perfect_passive_clause` (after a
  contracted perfect `been`). Neither is a verb-frame role and neither sits in
  the clause-final adjunct position `predicate_adjunct_predicate` occupies, so
  neither can be replaced by the shared host without a separate design. Left
  in place and named here.

### Assurance counts

Restored 0, re-spelled 2, ignored with a blocker 0, added 1, removed 0.
`predicate_grammar.rs` goes 103 -> 104 tests. The two re-spellings are the
`+3/+1` selection assertion and the `-1/-1` typed sign product: both still
assert the same card, the same head identity, the same magnitudes and the same
`until` duration, now read off the shared `PredicateAdjunctPredicate` envelope
instead of the deleted frame field. The added test,
`a_causative_complement_hosts_the_shared_clause_level_duration`, is the typed
witness for STOP 2's fix. No value comparison was swapped for a discriminant
check; the duration assertion was strengthened at review to require
`DurationPhrase::Until` rather than any duration.

### Glossary gaps

- glossary gap: **Duration Phrase** was absent from
  `docs/contexts/oracle-english/CONTEXT.md`; added.
- glossary gap: **Bare Predicate** was absent; added, because the causative fix
  makes the category load-bearing outside the modal auxiliary.

### Gate artifacts, run once on the final refreshed tree

- `cargo fmt --all` -- clean.
- `cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings` -- clean.
- `cargo clippy -p xtask --all-targets -- -D warnings` -- clean.
- `cargo test --workspace` (gate scope: `core_verbs.ron` changed) -- 128 test
  binaries, 6,131 passed, 0 failed, 0 ignored.
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check --workers 8`
  -- `selected_units 17068 / covered_units 17068 / total 32641`,
  `selected_uncovered_units 0`, `unresolved_ties 0`, `roundtrip_mismatch_units 0`,
  `ownership_failure_units 0`, `gap_bytes 0`, `overlap_bytes 0`,
  `licensing_checker_permitted 20`, `licensing_checker_forbidden 0`.
- `cargo xtask english_v2 coverage --bless --workers 8` -- 20 identities added
  against the in-tree lock, 0 retired; +16 / -0 against the parent tip.
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8` --
  `unresolved_ties=0`, `internal_failures=0`, `exception_uses=0`.
- `cargo xtask english_v2 roundtrip --require-clean --workers 8` --
  `parse accepted 17068`, `clean 17068`, `mismatched 0`.
- No CR citation site changed, so the cite gates were not exercised.

### Structural laws and provenance inventories

Proved on the final tree: byte-exact roundtrip `mismatched 0` over all 17,068
accepted units; lexical ownership `gap_bytes 0`, `overlap_bytes 0`,
`ownership_failure_units 0`, `provenance_plan_mismatches 0`; construction and
leaf traversal identity `nonterminal_nodes 718,669 = visited_constructions
718,669` and `expected_leaves 251,990 = visited_leaves 251,990` with
`traversal_failure_units 0` and `leaf_traversal_failure_units 0`;
`unresolved_ties 0`; `internal_failures 0`. Licensing checkers: 20 permitted,
0 forbidden -- no checker names a lexeme, construction, verb, noun,
preposition or card, and this landing added none.

Provenance inventories, reported and never fitted to. Licensed-vocabulary /
lexicon homographs (2): `AttributiveAdjective::Untap` beside the `Untap`
keyword-action Verb declaration; `TargetingMarker::Target` beside
`CommonNoun::Target`. Form-literal / vocabulary overlaps (5): `additional` at
`additional_cost`; `to` at `up_to_quantifying_determiner`; `next` at
`definite_next_mass_quantity_reference`; `to` at
`scalar_less_than_or_equal_to`; `other` at `other_than_qualified_reference`.
Both inventories are unchanged from the parent tip.

### Performance advisory

| gate | wall | thread-CPU per byte | host load 1m/5m/15m |
| --- | --- | --- | --- |
| coverage `--check` | 108 s | 122,080 ns/B | 4.02 / 5.08 / 6.70 |
| coverage `--bless` | 109 s | 122,436 ns/B | 9.48 / 6.70 / 7.06 |
| ambiguity | 118 s | 136,126 ns/B | 8.76 / 11.34 / 9.69 |
| roundtrip | 114 s | 130,259 ns/B | 10.24 / 8.43 / 8.67 |

All four are far over the 16.26 s quiet-host ceiling, which is advisory under
this load. Contention stamp: 1 executor plus 2 reviews running concurrently on
this host, and a second full corpus census was run from a scratch reflink of
the parent tip for the per-unit selection-neutrality proof. The per-byte figure
is the number to watch: 122,080 ns/B on coverage is the parse-time datum for
the next round, not the wall clock.

### Review corrections

- HIGH, fixed: STOP 2's 16 lost identities. `have_object_control.predicate`
  widened `VerbPhrase` -> `BarePredicate`
  (`crates/deckmaste_english_v2/src/constructions.rs`), restoring all 16 and
  covering 4 more.
- MEDIUM, fixed: the landing record carried no census, no stamped tree, no
  deviations list, no assurance counts and no glossary gaps. Rewritten above.
- MEDIUM, fixed: the two routing notes sat above the receiving tickets' title
  lines and called a never-covered witness a re-coverage debt. Moved to the
  end of each ticket and reworded as coverage owed.
- LOW, fixed: the re-spelled duration assertion had weakened from
  `DurationPhrase::Until` to any `PredicateAdjunct::Duration`; strengthened
  back and factored into `assert_duration_adjunct`.
