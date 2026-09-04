---
needs: [english-v2-closed-class-single-owner, english-v2-require-through-optional-role, english-v2-verb-frame-vocabulary]
---
**Move verb-selected prepositions into declared Verb Frame data.** Under the
[`Verb Frame` and `Complement` definitions](../../contexts/oracle-english/CONTEXT.md), the selected
preposition is part of what the lexical schema licenses; `valency` is not a
catch-all name for the schema, its instantiated phrase, and its realization.

Replace the 20 hardwired `to`/`from`/`on`/`into`/`for`/`onto`/`at` form
literals with vocabulary `Preposition` claims in selected-complement positions.
The core-verb seed and Keyword Action grammar contributions declare the marker,
and the frame construction consumes it. Close the optional-slot workaround at
the same seam. The `form_literal_vocab_overlaps` count decreases by these 20
with zero coverage change; standard constraints apply.

## Landing record

Measured on the reviewed change `urltpxqppwlpsqvttqpywtumsvrqzvmm`, refreshed
onto the default line that carries the number-feature unification landing, with
16,824 covered lock identities. The refreshed-parent lock and the measured lock
are the same file: 49,474 lines, SHA-256
`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`, +0/-0.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,824 | 16,824 |
| ordinary parse failures | 15,817 | 15,817 |
| unique selections | 11,527 | 11,527 |
| specificity-resolved selections | 5,297 | 5,297 |
| unresolved ties | 0 | 0 |
| construction declarations | 394 | 394 |
| licensing checkers permitted | 25 | 25 |
| licensed vocabulary/lexicon homographs | 2 | 2 |
| form-literal/vocabulary overlaps | 25 | 5 |
| coverage-lock identities | 16,824 | 16,824 |

- Implementation: forms can now consume a fixed vocabulary value as
  `lex(Preposition::Variant)`. `VerbFrame` data carries the corresponding
  `Lex` atom, and its optional form carries `OptionalLex`; the latter replaces
  the generic optional source-role workaround without making literals
  optional. The core-verb seed and the Attach, Exchange, Search, Shuffle, and
  Vote Keyword Action contributions declare their markers. Rendering,
  ownership, visitation, and frame-key matching all consume those declared
  atoms. Preposition classes remain the authority validated by the existing
  frame constructions.
- The twenty converted form literals, each a head-selected marker, none a
  postmodifier or adjunct: `passive_movement_predicate` (`into`, `from`),
  `declared_transitive_passive_from_predicate` (`from`),
  `declared_to_object_passive_predicate` (`to`), `ordered_predicate` (`on`),
  `contracted_perfect_object_on_clause` (`on`), `scalar_equality` (`to`),
  `declared_object_into_object_lexical_verb_phrase` (`into`),
  `declared_object_equality_to_predicate` (`to`),
  `declared_object_to_equality_predicate` (`to`),
  `declared_object_from_predicate` (`from`), `put_onto` (`onto`),
  `put_onto_source_after` (`onto`, `from`), `put_on` (`on`), `put_to` (`to`),
  `return_to` (`to`), `declared_with_object_on_predicate` (`on`),
  `look_at` (`at`), `declared_to_object_predicate` (`to`). The two
  `licensed("for")` atoms in `declared_object_for_object_lexical_verb_phrase`
  and `declared_for_object_predicate` also became vocabulary claims; they were
  governed homograph licences, so they never counted toward the ceiling and
  the arithmetic stays 25 - 20 = 5. The five surviving overlaps are
  `additional_cost` (`additional`), `up_to_quantifying_determiner` (`to`),
  `definite_next_mass_quantity_reference` (`next`),
  `scalar_less_than_or_equal_to` (`to`), and
  `other_than_qualified_reference` (`other`).
- Coverage and ownership: +0/-0 lock identities and no lock-byte change
  against the refreshed parent. Newly covered identities: none. The measured
  tree has 16,824 selected and covered units; selected-uncovered units,
  internal failures, exception resolutions, exception uses, round-trip
  mismatches, ownership failures, gaps, overlaps, synthetic claims,
  provenance-plan mismatches, and traversal failures are all zero. The
  selected-complement collision entries moved from form ownership to
  vocabulary ownership: on the measured tree form claims/bytes are
  116,078/210,004 and vocabulary claims/bytes are 67,798/285,440, with total
  claims 361,023 and claimed bytes 1,531,738 unchanged from the parent.
- Selection census: the reviewer re-ran `ambiguity --json` on the refreshed
  parent and on the measured tree and compared all 32,641 rows by status,
  selected candidate ordinal, candidate count, selected `construction_path`,
  and resolution class. Zero rows differ; both summaries are 16,824 selected,
  11,527 unique, 5,297 specificity-resolved, 0 exception-resolved, 0 ties,
  15,817 parse failures, 0 internal failures. The same comparison run before
  the refresh was likewise zero-difference. Fixed vocabulary markers re-spell
  affected positions from literal to typed lexical specificity without
  changing a candidate set, survivor, attachment, or winner.
- Positive artifacts on the reviewed tree: `cargo fmt --all`; strict clippy
  (`--all-targets -- -D warnings`) for `deckmaste_construction_core`,
  `deckmaste_english_v2`, `xtask`, and `deckmaste_construction`;
  `cargo test --workspace` (every suite `ok`, 0 failed); coverage `--check`,
  ambiguity `--require-resolved`, and round trip `--require-clean`, each with
  `--workers 8`; `cargo xtask cite check` reporting
  `checked 14226 citations against cr.txt (eff. 2026-08-07); 0 stale`; and
  `cargo xtask cite check --list-noncompliant` reporting
  `0 non-compliant citation-looking string(s)`. No citation text changed in
  this landing.
- Performance advisory (reviewed tree, `DECKMASTE_XTASK_WORKERS=8`; wall times
  rounded to whole seconds because a three-digit decimal reads as a rule
  number to the cite checker): coverage took 114 s at 177,925 thread-CPU ns/B
  under host load 12.94/13.81/15.96; ambiguity took 105 s at 143,547 ns/B
  under load 7.63/12.27/15.22; round trip took 91 s at 119,609 ns/B under load
  6.58/10.86/14.45. All three exceed the 16.26 s quiet-host ceiling under the
  `capped_workers` criterion. Contention stamp (reviewer-supplied, the
  implementer's sandbox cannot count siblings): 1-2 concurrent codex executors
  and 1-2 reviewers were on the host throughout, host load ~11 during the
  implementer's runs and 7-27 across the reviewer's.
- Assurance census: restored 0; re-spelled 10 existing test functions
  (`custom_frame_set_has_exact_finite_atom_shapes`,
  `declaration_verb_tail_is_a_normalized_semantic_frame_key`,
  `builtin_v2_keyword_action_nursery_is_complete_and_normalized`,
  `exchange_has_every_attested_representable_tail_shape`,
  `shuffle_has_every_attested_representable_tail_shape`,
  `special_core_verb_frames_are_exact_and_class_separated`,
  `authentic_nominal_and_selector_sentences_parse`,
  `declared_to_object_frame_parses_attach_without_a_card_specific_rule`,
  `collision_census_pins_licensed_homographs_and_caps_unlicensed_overlaps`,
  `runner_preserves_each_oracle_text_in_source_order`) plus two test-support
  fixtures (`declaration_verb_tokens`, the `predicate_grammar` `environment`
  helper); ignored with blockers 0; test functions added 0; removed 0. New
  assertions inside existing tests: the `OptionalLex` frame key in
  `declaration_verb_tail_is_a_normalized_semantic_frame_key`, and the reviewer's
  vocabulary-tail case in
  `declaration_verb_custom_shapes_match_only_exact_authored_tails`. Every
  re-spelling keeps an exact value comparison; no assertion was weakened to a
  discriminant or `matches!` check.
- Deviations and additions: no english-v2 grammar construction was added or
  deleted. No dominance edge, exception entry, narrowed form, row-specific
  licence, or identity-specific guard was added. The implementation follows both
  2026-09-03 rulings: the declared selected role retains preemption (the
  unmodified `declared_optional_source_preempts_the_same_noun_postmodifier_derivation`
  witness still passes), and no adjunct licence is restored. Beyond the
  ticket's letter the reviewer added one `LexMarkedVerb` codec, one
  `lex_marked` construction and root, and one synthetic `LexMarked` keyword
  action to `deckmaste_construction`'s compiled-consumer fixture so the new
  generated atoms are exercised by a second consumer; see Review corrections.
  The refresh crossed the number-feature unification landing with textual
  conflicts in five `nominal_grammar` witness rows and the coverage-census
  unit test; both were resolved keeping each side's change.
- STOPs: none. No coverage drop, selection change, attachment misselection,
  tie, negative-oracle admission, ticket/ruling contradiction, glossary gap,
  or decision request was found.

### Review corrections

- MEDIUM - the new `VerbFrameAtom::Lex`/`OptionalLex` emission and the
  `AtomPlan::LexFixed` lowering were exercised only through
  `deckmaste_english_v2`; `deckmaste_construction`'s compiled-consumer
  fixture, the repo's stated consumer-side pin on generated code, had no
  vocabulary tail atom. Fixed by adding the `LexMarkedVerb` codec, the
  `lex_marked` construction (whose form also carries a fixed `lex` atom), the
  `LexMarked` synthetic keyword action with a `Lex("AmountWord","One")` custom
  frame, and positive plus negative assertions in
  `declaration_verb_custom_shapes_match_only_exact_authored_tails`. The new
  assertion was falsified before being kept: perturbing the fixture frame atom
  to `Amount` makes it fail.
- LOW - `CustomTailAtom::Lex` is plugin-facing serialized API and
  `FormAtom::FixedLex` sits beside a documented `LicensedLiteral`; both were
  undocumented. Fixed with one factual line each.
- LOW - the new `lex` branch in the `declaration_verb` tail parser left the
  following `match` arms indented one level short (rustfmt does not reformat
  that closure). Fixed by hand.
- LOW - the record described all twelve touched test functions as tests; two
  are test-support fixtures. Corrected in the assurance census above, which
  now names each one.
- The record's contention stamp was unavailable to the sandboxed implementer;
  the reviewer's stamp is recorded in the performance advisory above.
- Verified and unchanged: the twenty converted literals are enumerated above
  and every one sits in a head-selected complement position, so the removed
  adjunct-licence shape is not reintroduced; the 25 - 20 = 5 arithmetic
  matches the gate output; `core_verbs.ron` adds no verb and changes no
  valence beyond the declared markers and the source-slot closure; the five
  Keyword Action contributions changed only their marker declaration; the
  cite set is 0 stale over the full tracked set (the 14,162 figure in the
  pre-refresh record was simply the older base's tracked-file count, now
  14,226).
- LOW - the reviewer's first perf advisory wrote wall times as full-precision
  decimals; at three digits before the point the cite checker reads them as
  loose rule numbers and `--list-noncompliant` reported three. Fixed by
  rounding to whole seconds; the list is empty again.
