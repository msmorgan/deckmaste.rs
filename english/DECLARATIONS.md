# V3 workbench declaration disposition

This inventory compares the source declarations before this rewrite (claim
`ulvllurt`) with the v3 model. It accounts for all 789 named source declarations
in the 30 original `English/` modules, including private helpers. A retained
inductive/structure declaration includes its constructors, fields and derived
members; these were not independently removed. The compiled axiom audit also
checks private and generated theorems. `English.lean` remains the import root.

**Kept** means the stated schema or witnessed grammatical outcome survives.
Proofs that use a renamed carrier are re-spelled, not new grammatical claims.
**Replaced** names the new declaration or claim. **Retired** names the removed
subject and its disposition; the superseded pruning interface is not a v3 rule.
No declaration is retained merely as an API compatibility requirement.

## Proof accounting

There were 344 named source theorems; there are now 472. Of the old
theorems, 301 are retained without a proof-text change, 33 are re-spelled or
replaced, and 10 are retired. Restored: 0; ignored: 0. There are 148 new theorem
names (including replacement names). This new-name count overlaps the old
replacement count; it is not an additive measure of new grammar coverage.

Old theorems re-spelled under the same name:

- `English.AnalysisInteractions.Frames.related`
- `English.FrameScope.attachment_included`
- `English.FrameScope.key_exact`
- `English.FrameScope.related_preserves_anchors`
- `English.FrameScope.related_preserves_lexemes`
- `English.GrammaticalScope.association_separate`
- `English.GrammaticalScope.independent_modifier_regions`
- `English.GrammaticalScope.map_related`
- `English.GrammaticalScope.nested_package_exact`
- `English.GrammaticalScope.nested_scope_related`
- `English.GrammaticalScope.package_exact`
- `English.GrammaticalScope.quoted_scope_related`
- `English.GrammaticalScope.related_preserves_hosts`
- `English.GrammaticalScope.related_preserves_lexemes`
- `English.GrammaticalScope.same_class_iff`
- `English.RolePreference.role_pair_selected`
- `English.RolePreference.role_worse_excluded`
- `English.RolePreference.selected_enumeration`
- `English.RolePreference.selected_exists`
- `English.Scope.anchor_shape_separate`
- `English.WordFormInteractions.addressee_were`
- `English.WordFormInteractions.plural_were`
- `English.WordFormInteractions.singular_was`

Every retired theorem is named with its reason in its module below. The
crossed-form counterexamples are classified as repairs of wrong analyses,
with the same strings rejected and the legitimate sentences retained. No
linguistic regression is assigned away to make the build pass.

## Module dispositions

The generic grammar, feature, dependency and document schemas remain inputs to
v3 admission. Their older witness contracts remain bounded schema evidence.
`Selection` and `RolePreference` are optional preference-view algebra;
`Scope`, `GrammaticalScope` and `FrameScope` describe local relationships and
projections. None of those five modules is imported by the v3 admission closure.
Their selection or single-class packaging operations do not define the full
reading set. See [README.md](README.md) for the current entry points and limits.

### AgreementInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.AgreementInteractions.Lexeme` | Kept. |
| `English.AgreementInteractions.pronounText` | Kept. |
| `English.AgreementInteractions.lexicon` | Kept. |
| `English.AgreementInteractions.pronoun` | Kept. |
| `English.AgreementInteractions.attack` | Kept. |
| `English.AgreementInteractions.mixed` | Kept. |
| `English.AgreementInteractions.mixedClause` | Kept. |
| `English.AgreementInteractions.mixed_number_clause` | Kept. |
| `English.AgreementInteractions.mixed_person_clause` | Kept. |
| `English.AgreementInteractions.speaker_person_retained` | Kept. |
| `English.AgreementInteractions.addressee_person_retained` | Kept. |
| `English.AgreementInteractions.additive_singular_rejected` | Kept. |
| `English.AgreementInteractions.alternative` | Kept. |
| `English.AgreementInteractions.alternativeClause` | Kept. |
| `English.AgreementInteractions.mixed_alternative_text` | Kept. |
| `English.AgreementInteractions.proximity_depends_on_position` | Kept. |
| `English.AgreementInteractions.wrong_proximity_rejected` | Kept. |
| `English.AgreementInteractions.andOr` | Kept. |
| `English.AgreementInteractions.andOrClause` | Kept. |
| `English.AgreementInteractions.andOr_plural_text` | Kept. |
| `English.AgreementInteractions.andOr_addressee_text` | Kept. |
| `English.AgreementInteractions.andOr_proximity_depends_on_position` | Kept. |
| `English.AgreementInteractions.andOr_singular_final_conjunct` | Kept. |
| `English.AgreementInteractions.andOr_wrong_proximity_rejected` | Kept. |

### Analysis.lean

| Old declaration | Disposition |
|---|---|
| `English.Analysis.Reading` | Replaced by `English.Reading`; `English.Reading.Admitted`. Separate grammatical values from proof evidence; require the v3 lexical relation. |
| `English.Analysis.IdentityStep` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.identityCount` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.childIdentities` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.identities_append` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.roles_append` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.measure` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.Prefers` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.preference_increases` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.no_cycle` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.Selected` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.selected_exists` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.selected_enumeration` | Retired global identity/rank pruning and its supporting measure/termination machinery. English.Reading.Preference and preference_retains_both express the replacement non-destructive contract; no corresponding admission ranking remains. |
| `English.Analysis.Related` | Replaced by `English.FrameScope.Related`. Keep the local scope relationship; it no longer bounds the full result. |
| `English.Analysis.setoid` | Retired the global selected-class result interface. English.Reading.readings retains all classes; English.FrameScope retains the useful local scope/key laws. |
| `English.Analysis.key` | Retired the global selected-class result interface. English.Reading.readings retains all classes; English.FrameScope retains the useful local scope/key laws. |
| `English.Analysis.key_exact` | Retired the global selected-class result interface. English.Reading.readings retains all classes; English.FrameScope retains the useful local scope/key laws. |
| `English.Analysis.package` | Retired the global selected-class result interface. English.Reading.readings retains all classes; English.FrameScope retains the useful local scope/key laws. |
| `English.Analysis.package_exact` | Retired the global selected-class result interface. English.Reading.readings retains all classes; English.FrameScope retains the useful local scope/key laws. |
| `English.Analysis.no_feature_bypass` | Replaced by `English.Reading.Valid`; `English.Reading.Admitted`. Every admitted value carries recursive feature and dependency checks, independently of packaging. |

### AnalysisInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.AnalysisInteractions.Frames.features` | Kept. |
| `English.AnalysisInteractions.Frames.dependencies` | Kept. |
| `English.AnalysisInteractions.Frames.noun_use` | Kept. |
| `English.AnalysisInteractions.Frames.left_checked` | Kept. |
| `English.AnalysisInteractions.Frames.right_checked` | Kept. |
| `English.AnalysisInteractions.Frames.a` | Kept. |
| `English.AnalysisInteractions.Frames.b` | Kept. |
| `English.AnalysisInteractions.Frames.related` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.AnalysisInteractions.Frames.both_selected` | Replaced by `English.AnalysisInteractions.Frames.both_retained`. Both checked frame alternatives remain grammatical. |
| `English.AnalysisInteractions.Frames.complete_packing` | Replaced by `English.AnalysisInteractions.Frames.both_retained`; `English.AnalysisInteractions.Frames.distinct_frame_readings`. Preserve both distinct alternatives without selected-class packaging. |
| `English.AnalysisInteractions.Identities.agreement` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.lexicon` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.features` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.dependencies` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.identity` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.lexical` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.identity_preference` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Identities.identity_selected` | Retired artificial Echo identity/pronoun fixture and its destructive preference policy. English.AmbiguityWitnesses.unrelated_readings_retained supplies genuine lexical/structural ambiguity; preferred_and_nonpreferred_remain preserves both alternatives. |
| `English.AnalysisInteractions.Rejections.reversed_target_not_candidate` | Kept. |

### BoundaryInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.BoundaryInteractions.semicolonLine` | Kept. |
| `English.BoundaryInteractions.semicolon_text` | Kept. |
| `English.BoundaryInteractions.singleton_has_no_separator` | Kept. |
| `English.BoundaryInteractions.wordPayload` | Kept. |
| `English.BoundaryInteractions.source_space_is_not_a_word` | Kept. |
| `English.BoundaryInteractions.lexical_apostrophe_allowed` | Kept. |
| `English.BoundaryInteractions.source_newline_is_not_a_word` | Kept. |
| `English.BoundaryInteractions.Supplements.nominal` | Kept. |
| `English.BoundaryInteractions.Supplements.np` | Kept. |
| `English.BoundaryInteractions.Supplements.predicate` | Kept. |
| `English.BoundaryInteractions.Supplements.clause` | Kept. |
| `English.BoundaryInteractions.Supplements.sentence` | Kept. |
| `English.BoundaryInteractions.Supplements.at_sentence_end` | Kept. |

### CaseConstraints.lean

| Old declaration | Disposition |
|---|---|
| `English.Case` | Kept. |
| `English.CasePosition` | Kept. |
| `English.Case.Allows` | Kept. |
| `English.Case.common` | Kept. |
| `English.Case.common_allows` | Kept. |
| `English.Case.common_associative` | Kept. |
| `English.Case.Argument` | Kept. |
| `English.Relation.casePosition` | Kept. |
| `English.Syntax.nominalCase` | Kept. |
| `English.CaseAt` | Kept. |
| `English.FrameCases` | Kept. |

### CaseInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.CaseInteractions.Lexeme` | Kept. |
| `English.CaseInteractions.pronouns` | Kept. |
| `English.CaseInteractions.agreement` | Kept. |
| `English.CaseInteractions.text` | Kept. |
| `English.CaseInteractions.features` | Kept. |
| `English.CaseInteractions.object` | Kept. |
| `English.CaseInteractions.seeText` | Kept. |
| `English.CaseInteractions.lexicon` | Kept. |
| `English.CaseInteractions.pronoun` | Kept. |
| `English.CaseInteractions.see` | Kept. |
| `English.CaseInteractions.clause` | Kept. |
| `English.CaseInteractions.withPronoun` | Kept. |
| `English.CaseInteractions.objectGap` | Kept. |
| `English.CaseInteractions.raisedSee` | Kept. |
| `English.CaseInteractions.raised_accusative_object` | Kept. |
| `English.CaseInteractions.raised_nominative_is_category_valid` | Kept. |
| `English.CaseInteractions.raised_nominative_object_rejected` | Kept. |
| `English.CaseInteractions.fronted_case_reaches_body` | Kept. |
| `English.CaseInteractions.unfronted_relative_binds_case` | Kept. |
| `English.CaseInteractions.nominative_subject_and_accusative_object` | Kept. |
| `English.CaseInteractions.accusative_prepositional_complement` | Kept. |
| `English.CaseInteractions.wrong_subject_is_category_valid` | Kept. |
| `English.CaseInteractions.accusative_subject_rejected` | Kept. |
| `English.CaseInteractions.nominative_object_rejected` | Kept. |
| `English.CaseInteractions.nominative_prepositional_complement_rejected` | Kept. |
| `English.CaseInteractions.coordination_requires_every_conjunct` | Kept. |
| `English.CaseInteractions.crossed_case_coordination_rejected` | Kept. |

### Composition.lean

| Old declaration | Disposition |
|---|---|
| `English.Composition.Lexeme` | Kept. |
| `English.Composition.plural` | Kept. |
| `English.Composition.singular` | Kept. |
| `English.Composition.objectFrame` | Kept. |
| `English.Composition.lexicon` | Kept. |
| `English.Composition.creatures` | Kept. |
| `English.Composition.artifacts` | Kept. |
| `English.Composition.turns` | Kept. |
| `English.Composition.attack` | Kept. |
| `English.Composition.duringTurns` | Kept. |
| `English.Composition.canAttack` | Kept. |
| `English.Composition.relativeBody` | Kept. |
| `English.Composition.relativeNominal` | Kept. |
| `English.Composition.creatures_derives` | Kept. |
| `English.Composition.artifacts_derives` | Kept. |
| `English.Composition.turns_derives` | Kept. |
| `English.Composition.attack_derives` | Kept. |
| `English.Composition.during_derives` | Kept. |
| `English.Composition.can_attack_derives` | Kept. |
| `English.Composition.relative_body_gap` | Kept. |
| `English.Composition.relative_derives` | Kept. |
| `English.Composition.coordinatedObjects` | Kept. |
| `English.Composition.destroyObjects` | Kept. |
| `English.Composition.coordinated_complement` | Kept. |
| `English.Composition.noun_phrase_relations` | Kept. |
| `English.Composition.creatures_surface` | Kept. |
| `English.Composition.artifacts_surface` | Kept. |
| `English.Composition.turns_surface` | Kept. |
| `English.Composition.attack_surface` | Kept. |
| `English.Composition.can_attack_surface` | Kept. |
| `English.Composition.during_surface` | Kept. |
| `English.Composition.relative_surface` | Kept. |
| `English.Composition.coordinated_surface` | Kept. |
| `English.Composition.negative_auxiliary` | Kept. |
| `English.Composition.quantity_np` | Kept. |
| `English.Composition.singular_agreement` | Kept. |
| `English.Composition.singular_rejects_plain` | Kept. |
| `English.Composition.gap_not_closed` | Kept. |
| `English.Composition.extra_complement` | Kept. |
| `English.Composition.frame_rejects_extra_complement` | Kept. |
| `English.Composition.elliptic_antecedent` | Kept. |
| `English.Composition.elliptic_surface` | Kept. |
| `English.Composition.modal_singular` | Kept. |
| `English.Composition.voiceLexicon` | Kept. |
| `English.Composition.passive` | Kept. |
| `English.Composition.passive_derives` | Kept. |
| `English.Composition.passive_surface` | Kept. |
| `English.Composition.perfect_rejects_passive` | Kept. |
| `English.Composition.perfect_active_twin` | Kept. |
| `English.Composition.fixed_marker_frame` | Kept. |
| `English.Composition.wrong_fixed_marker` | Kept. |
| `English.Composition.shared_relative_gap` | Kept. |
| `English.Composition.arithmetic` | Kept. |
| `English.Composition.comparisonLexicon` | Kept. |
| `English.Composition.comparison` | Kept. |
| `English.Composition.nonfinite_clause` | Kept. |

### Dependencies.lean

| Old declaration | Disposition |
|---|---|
| `English.Dependencies.Declarations` | Kept. |
| `English.Dependencies.GapUse` | Kept. |
| `English.Dependencies.exposed` | Kept. |
| `English.Dependencies.childrenExposed` | Kept. |
| `English.Dependencies.frameExposed` | Kept. |
| `English.Dependencies.RelativePhrase` | Kept. |
| `English.Dependencies.RelativeLicense` | Kept. |
| `English.Dependencies.Local` | Kept. |
| `English.Dependencies.Safe` | Kept. |
| `English.Dependencies.ChildrenSafe` | Kept. |
| `English.Dependencies.Admitted` | Kept. |
| `English.Dependencies.zero_subject_excluded` | Kept. |
| `English.Dependencies.adjunct_island` | Kept. |
| `English.Dependencies.ordinary_coordination_cannot_share` | Kept. |
| `English.Dependencies.wh_category_must_match` | Kept. |

### DependencyInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.DependencyInteractions.Lexeme` | Kept. |
| `English.DependencyInteractions.plural` | Kept. |
| `English.DependencyInteractions.singular` | Kept. |
| `English.DependencyInteractions.second` | Kept. |
| `English.DependencyInteractions.object` | Kept. |
| `English.DependencyInteractions.destination` | Kept. |
| `English.DependencyInteractions.lexicon` | Kept. |
| `English.DependencyInteractions.features` | Kept. |
| `English.DependencyInteractions.dependencies` | Kept. |
| `English.DependencyInteractions.creatures` | Kept. |
| `English.DependencyInteractions.you` | Kept. |
| `English.DependencyInteractions.which` | Kept. |
| `English.DependencyInteractions.controlGap` | Kept. |
| `English.DependencyInteractions.objectBody` | Kept. |
| `English.DependencyInteractions.object_body` | Kept. |
| `English.DependencyInteractions.object_form` | Kept. |
| `English.DependencyInteractions.object_accessible` | Kept. |
| `English.DependencyInteractions.object_body_safe` | Kept. |
| `English.DependencyInteractions.zero_relative_admitted` | Kept. |
| `English.DependencyInteractions.which_relative_admitted` | Kept. |
| `English.DependencyInteractions.subject_zero_rejected` | Kept. |
| `English.DependencyInteractions.whoseController` | Kept. |
| `English.DependencyInteractions.attack` | Kept. |
| `English.DependencyInteractions.subjectBody` | Kept. |
| `English.DependencyInteractions.whose_fronted_phrase` | Kept. |
| `English.DependencyInteractions.whose_agreement_independent` | Kept. |
| `English.DependencyInteractions.intoWhich` | Kept. |
| `English.DependencyInteractions.artifacts` | Kept. |
| `English.DependencyInteractions.destinationBody` | Kept. |
| `English.DependencyInteractions.pied_piping` | Kept. |
| `English.DependencyInteractions.pied_piping_cannot_use_bare_which` | Kept. |
| `English.DependencyInteractions.modifier_gap_rejected` | Kept. |

### DocumentCollections.lean

| Old declaration | Disposition |
|---|---|
| `English.Documents.continuedTrigger` | Kept. |
| `English.Documents.threeModes` | Kept. |
| `English.Documents.modalGroup` | Kept. |
| `English.Documents.modalBody` | Kept. |
| `English.Documents.ordinaryModes` | Kept. |
| `English.Documents.activatedModes` | Kept. |
| `English.Documents.sentenceHeaderModes` | Kept. |
| `English.Documents.emptyDocument` | Kept. |
| `English.Documents.emptyLevelBand` | Kept. |
| `English.Documents.threeCosts` | Kept. |
| `English.Documents.threeKeywords` | Kept. |
| `English.Documents.threeParagraphs` | Kept. |
| `English.Documents.continued_trigger_text` | Kept. |
| `English.Documents.three_modes_text` | Kept. |
| `English.Documents.activated_modes_text` | Kept. |
| `English.Documents.sentence_header_modes_text` | Kept. |
| `English.Documents.empty_document_text` | Kept. |
| `English.Documents.empty_level_band_text` | Kept. |
| `English.Documents.three_costs_text` | Kept. |
| `English.Documents.three_keywords_text` | Kept. |
| `English.Documents.three_paragraphs_text` | Kept. |
| `English.Documents.document_coordination_rejected` | Kept. |
| `English.Documents.nested_document_collection_rejected` | Kept. |
| `English.Documents.nested_body_collection_rejected` | Kept. |
| `English.Documents.mode_collection_arity` | Kept. |
| `English.Documents.cost_collection_arity` | Kept. |
| `English.Documents.supertype_collection_arity` | Kept. |
| `English.Documents.keyword_line_arity` | Kept. |

### DocumentShape.lean

| Old declaration | Disposition |
|---|---|
| `English.LabelKind` | Kept. |
| `English.DocumentCategory` | Kept. |
| `English.KeywordSeparator` | Kept. |
| `English.DocumentRule` | Kept. |
| `English.KeywordPlacement` | Kept. |
| `English.FaceLayout` | Kept. |

### Documents.lean

| Old declaration | Disposition |
|---|---|
| `English.Documents.Witness` | Kept. |
| `English.Documents.unary` | Kept. |
| `English.Documents.binary` | Kept. |
| `English.Documents.ternary` | Kept. |
| `English.Documents.Lexeme` | Kept. |
| `English.Documents.keywordFrame` | Kept. |
| `English.Documents.quotedFrame` | Kept. |
| `English.Documents.lexicalWords` | Kept. |
| `English.Documents.lexicon` | Kept. |
| `English.Documents.word` | Kept. |
| `English.Documents.mana` | Kept. |
| `English.Documents.roman` | Kept. |
| `English.Documents.level` | Kept. |
| `English.Documents.range` | Kept. |
| `English.Documents.stats` | Kept. |
| `English.Documents.dieRange` | Kept. |
| `English.Documents.threshold` | Kept. |
| `English.Documents.label` | Kept. |
| `English.Documents.legendary` | Kept. |
| `English.Documents.creature` | Kept. |
| `English.Documents.elf` | Kept. |
| `English.Documents.attack` | Kept. |
| `English.Documents.instruction` | Kept. |
| `English.Documents.sentence` | Kept. |
| `English.Documents.body` | Kept. |
| `English.Documents.ordinary` | Kept. |
| `English.Documents.document` | Kept. |
| `English.Documents.repeatedDocument` | Kept. |
| `English.Documents.symbolCost` | Kept. |
| `English.Documents.cost` | Kept. |
| `English.Documents.actionCost` | Kept. |
| `English.Documents.costs` | Kept. |
| `English.Documents.activated` | Kept. |
| `English.Documents.creatures` | Kept. |
| `English.Documents.event` | Kept. |
| `English.Documents.trigger` | Kept. |
| `English.Documents.initialClause` | Kept. |
| `English.Documents.triggered` | Kept. |
| `English.Documents.ward` | Kept. |
| `English.Documents.flying` | Kept. |
| `English.Documents.bareKeyword` | Kept. |
| `English.Documents.landwalk` | Kept. |
| `English.Documents.gain` | Kept. |
| `English.Documents.keywordLine` | Kept. |
| `English.Documents.boundLine` | Kept. |
| `English.Documents.keywordList` | Kept. |
| `English.Documents.quoted` | Kept. |
| `English.Documents.gainQuoted` | Kept. |
| `English.Documents.quotedInstruction` | Kept. |
| `English.Documents.nestedQuote` | Kept. |
| `English.Documents.reminderProse` | Kept. |
| `English.Documents.reminder` | Kept. |
| `English.Documents.mode` | Kept. |
| `English.Documents.modes` | Kept. |
| `English.Documents.weightedMode` | Kept. |
| `English.Documents.labelled` | Kept. |
| `English.Documents.chapter` | Kept. |
| `English.Documents.classLevel` | Kept. |
| `English.Documents.levelBand` | Kept. |
| `English.Documents.solve` | Kept. |
| `English.Documents.solved` | Kept. |
| `English.Documents.dieRow` | Kept. |
| `English.Documents.dieDashRow` | Kept. |
| `English.Documents.station` | Kept. |
| `English.Documents.typeLine` | Kept. |
| `English.Documents.Faces` | Kept. |
| `English.Documents.room` | Kept. |
| `English.Documents.multiface` | Kept. |
| `English.Documents.ordinary_text` | Kept. |
| `English.Documents.activated_text` | Kept. |
| `English.Documents.triggered_text` | Kept. |
| `English.Documents.symbols_bind` | Kept. |
| `English.Documents.keyword_text` | Kept. |
| `English.Documents.bare_keyword_text` | Kept. |
| `English.Documents.bound_text` | Kept. |
| `English.Documents.keyword_second_host` | Kept. |
| `English.Documents.bound_second_host` | Kept. |
| `English.Documents.quote_text` | Kept. |
| `English.Documents.nested_quote_text` | Kept. |
| `English.Documents.reminder_text` | Kept. |
| `English.Documents.chapter_text` | Kept. |
| `English.Documents.class_text` | Kept. |
| `English.Documents.level_text` | Kept. |
| `English.Documents.case_text` | Kept. |
| `English.Documents.die_text` | Kept. |
| `English.Documents.die_dash_text` | Kept. |
| `English.Documents.modes_text` | Kept. |
| `English.Documents.solved_text` | Kept. |
| `English.Documents.station_text` | Kept. |
| `English.Documents.type_line_text` | Kept. |
| `English.Documents.room_faces_text` | Kept. |
| `English.Documents.repeated_paragraphs` | Kept. |
| `English.Documents.empty_sentence_rejected` | Kept. |
| `English.Documents.nested_reminder_rejected` | Kept. |
| `English.Documents.type_line_order` | Kept. |

### EllipsisInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.EllipsisInteractions.Lexeme` | Kept. |
| `English.EllipsisInteractions.agreement` | Kept. |
| `English.EllipsisInteractions.lexicon` | Kept. |
| `English.EllipsisInteractions.attack` | Kept. |
| `English.EllipsisInteractions.instruction` | Kept. |
| `English.EllipsisInteractions.firstSentence` | Kept. |
| `English.EllipsisInteractions.you` | Kept. |
| `English.EllipsisInteractions.doEllipsis` | Kept. |
| `English.EllipsisInteractions.ellipticalClause` | Kept. |
| `English.EllipsisInteractions.condition` | Kept. |
| `English.EllipsisInteractions.continuedClause` | Kept. |
| `English.EllipsisInteractions.secondSentence` | Kept. |
| `English.EllipsisInteractions.paragraph` | Kept. |
| `English.EllipsisInteractions.attack_derives` | Kept. |
| `English.EllipsisInteractions.first_derives` | Kept. |
| `English.EllipsisInteractions.second_derives` | Kept. |
| `English.EllipsisInteractions.paragraph_derives` | Kept. |
| `English.EllipsisInteractions.sentence_forms` | Kept. |
| `English.EllipsisInteractions.paragraph_text` | Kept. |
| `English.EllipsisInteractions.ellipsis_requires_context` | Kept. |
| `English.EllipsisInteractions.omitted_form_must_match` | Kept. |
| `English.EllipsisInteractions.future_cannot_license_first` | Kept. |
| `English.EllipsisInteractions.omission_introduces_nothing` | Kept. |
| `English.EllipsisInteractions.quotation_context_isolated` | Kept. |
| `English.EllipsisInteractions.reminder_inherits_without_exporting` | Kept. |
| `English.EllipsisInteractions.reminderBody` | Kept. |
| `English.EllipsisInteractions.parenthetical` | Kept. |
| `English.EllipsisInteractions.inlineReminder` | Kept. |
| `English.EllipsisInteractions.inline_reminder_derives` | Kept. |
| `English.EllipsisInteractions.inline_reminder_text` | Kept. |

### FeatureConstraints.lean

| Old declaration | Disposition |
|---|---|
| `English.Features.Countability` | Kept. |
| `English.Features.Declarations` | Kept. |
| `English.Features.NominalUse` | Kept. |
| `English.Features.DeterminerUse` | Kept; its `genitive` constructor is replaced by `English.Features.Transparent`, so countability is head-owned. |
| `English.Features.containsTarget` | Kept. |
| `English.Features.Temporal` | Kept. |
| `English.Features.Local` | Kept. |
| `English.Features.Conforms` | Kept. |
| `English.Features.ChildrenConform` | Kept. |
| `English.Features.Admitted` | Kept. |
| `English.Features.admitted_base` | Kept. |
| `English.Features.numeral_rejects_mass` | Kept. |
| `English.Features.target_must_be_outer` | Kept. |
| `English.Features.temporal_adjunct_requires_feature` | Kept. |

### FeatureInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.FeatureInteractions.Lexeme` | Kept. |
| `English.FeatureInteractions.features` | Kept. |
| `English.FeatureInteractions.lexicon` | Kept. |
| `English.FeatureInteractions.damage` | Kept. |
| `English.FeatureInteractions.much` | Kept. |
| `English.FeatureInteractions.muchDamage` | Kept. |
| `English.FeatureInteractions.mass_quantity` | Kept. |
| `English.FeatureInteractions.numerical_mass_excluded` | Kept. |
| `English.FeatureInteractions.creatures` | Kept. |
| `English.FeatureInteractions.whiteCreatures` | Kept. |
| `English.FeatureInteractions.targetWhite` | Kept. |
| `English.FeatureInteractions.targetPhrase` | Kept. |
| `English.FeatureInteractions.target_and_adjective` | Kept. |
| `English.FeatureInteractions.reversed_target_excluded` | Kept. |
| `English.FeatureInteractions.thisTurn` | Kept. |
| `English.FeatureInteractions.destroyed` | Kept. |
| `English.FeatureInteractions.passiveTemporal` | Kept. |
| `English.FeatureInteractions.passive_temporal` | Kept. |
| `English.FeatureInteractions.ordinary_np_not_temporal` | Kept. |
| `English.FeatureInteractions.passive_frame_does_not_acquire_object` | Kept. |
| `English.FeatureInteractions.genitive_preserves_countability` | Replaced by `English.FeatureInteractions.genitive_determiner_is_transparent`. The wildcard `Features.DeterminerUse.genitive` constructor it was proved from is retired in favour of `Features.Transparent`; the replacement states head-owned countability and adds the discriminating conjuncts (the genitive declares no use of its own, and cannot license a head that declares none). |

### FrameDeclarations.lean

| Old declaration | Disposition |
|---|---|
| `English.FrameDeclarations.Item` | Kept. |
| `English.FrameDeclarations.Expands` | Kept. |
| `English.FrameDeclarations.one_optional_exact` | Kept. |
| `English.FrameDeclarations.Lexeme` | Kept. |
| `English.FrameDeclarations.plural` | Kept. |
| `English.FrameDeclarations.nominalCategory` | Kept. |
| `English.FrameDeclarations.role` | Kept. |
| `English.FrameDeclarations.object` | Kept. |
| `English.FrameDeclarations.recipient` | Kept. |
| `English.FrameDeclarations.optionalRecipient` | Kept. |
| `English.FrameDeclarations.lexicon` | Kept. |
| `English.FrameDeclarations.creatures` | Kept. |
| `English.FrameDeclarations.artifacts` | Kept. |
| `English.FrameDeclarations.creatures_derives` | Kept. |
| `English.FrameDeclarations.artifacts_derives` | Kept. |
| `English.FrameDeclarations.optional_absent` | Kept. |
| `English.FrameDeclarations.optional_present` | Kept. |
| `English.FrameDeclarations.optional_does_not_drop_required` | Kept. |
| `English.FrameDeclarations.optional_does_not_change_marker` | Kept. |
| `English.FrameDeclarations.optional_does_not_reorder` | Kept. |
| `English.FrameDeclarations.multiple_complement_orders` | Kept. |
| `English.FrameDeclarations.retainedObject` | Kept. |
| `English.FrameDeclarations.passive_retains_declared_object` | Kept. |
| `English.FrameDeclarations.retained_object_surface` | Kept. |
| `English.FrameDeclarations.passive_cannot_drop_declared_object` | Kept. |
| `English.FrameDeclarations.passive_cannot_acquire_another_object` | Kept. |
| `English.FrameDeclarations.passive_cannot_borrow_active_optional_role` | Kept. |
| `English.FrameDeclarations.retained_gap_keeps_object_relation` | Kept. |
| `English.FrameDeclarations.marked_gap_keeps_complement_relation` | Kept. |

### FrameInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.FrameInteractions.Lexeme` | Kept. |
| `English.FrameInteractions.plural` | Kept. |
| `English.FrameInteractions.nominalCategory` | Kept. |
| `English.FrameInteractions.role` | Kept. |
| `English.FrameInteractions.baseFrame` | Kept. |
| `English.FrameInteractions.markedFrame` | Kept. |
| `English.FrameInteractions.lexicon` | Kept. |
| `English.FrameInteractions.creatures` | Kept. |
| `English.FrameInteractions.artifacts` | Kept. |
| `English.FrameInteractions.onArtifacts` | Kept. |
| `English.FrameInteractions.duringArtifacts` | Kept. |
| `English.FrameInteractions.postmodify` | Kept. |
| `English.FrameInteractions.pair` | Kept. |
| `English.FrameInteractions.immediate_role` | Kept. |
| `English.FrameInteractions.role_pair_admitted` | Kept. |
| `English.FrameInteractions.earlier_eligible_blocks` | Kept. |
| `English.FrameInteractions.earlier_ineligible_skipped` | Kept. |
| `English.FrameInteractions.nonfinal_postmodifier_preserved` | Kept. |
| `English.FrameInteractions.pairedArguments` | Kept. |
| `English.FrameInteractions.putPairs` | Kept. |
| `English.FrameInteractions.putSharedObject` | Kept. |
| `English.FrameInteractions.paired_arguments_text` | Kept. |
| `English.FrameInteractions.shared_object_text` | Kept. |
| `English.FrameInteractions.marked_role_cannot_use_other_marker` | Kept. |
| `English.FrameInteractions.concrete_role_selection` | Kept. |
| `English.FrameInteractions.role_preference_does_not_choose_homographs` | Kept. |

### FrameScope.lean

| Old declaration | Disposition |
|---|---|
| `English.FrameScope.group` | Kept. |
| `English.FrameScope.pairs` | Kept. |
| `English.FrameScope.BoundaryMove` | Kept. |
| `English.FrameScope.Step` | Kept. |
| `English.FrameScope.Related` | Kept. |
| `English.FrameScope.leaves_append` | Kept. |
| `English.FrameScope.boundary_preserves_lexemes` | Kept. |
| `English.FrameScope.step_preserves_lexemes` | Kept. |
| `English.FrameScope.related_preserves_lexemes` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.FrameScope.anchorCount` | Kept. |
| `English.FrameScope.childAnchors` | Kept. |
| `English.FrameScope.anchors_append` | Kept. |
| `English.FrameScope.node_anchors` | Kept. |
| `English.FrameScope.attachment_anchors` | Kept. |
| `English.FrameScope.step_preserves_anchors` | Kept. |
| `English.FrameScope.related_preserves_anchors` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.FrameScope.flat_nested_differ` | Kept. |
| `English.FrameScope.setoid` | Kept. |
| `English.FrameScope.key` | Kept. |
| `English.FrameScope.key_exact` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.FrameScope.attachment_included` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.FrameScope.projectHost` | Kept. |
| `English.FrameScope.frameHost` | Kept. |
| `English.FrameScope.actual_nonfinal_boundary` | Kept. |
| `English.FrameScope.actual_role_boundary` | Kept. |
| `English.FrameScope.fixed_marker_transparent` | Kept. |
| `English.FrameScope.fixed_coordination_transparent` | Kept. |
| `English.FrameScope.actual_first_unique` | Kept. |
| `English.FrameScope.Witnesses.cluster` | Kept. |
| `English.FrameScope.Witnesses.left` | Kept. |
| `English.FrameScope.Witnesses.right` | Kept. |
| `English.FrameScope.Witnesses.same_surface` | Kept. |
| `English.FrameScope.Witnesses.both_written` | Kept. |
| `English.FrameScope.Witnesses.distinct` | Kept. |
| `English.FrameScope.Witnesses.leftReading` | Kept. |
| `English.FrameScope.Witnesses.rightReading` | Kept. |
| `English.FrameScope.Witnesses.related` | Kept. |
| `English.FrameScope.Witnesses.both_packed` | Kept. |
| `English.FrameScope.Witnesses.same_role_count` | Kept. |
| `English.FrameScope.Witnesses.no_boundary_preference` | Kept. |

### Grammar.lean

| Old declaration | Disposition |
|---|---|
| `English.Number` | Kept. |
| `English.Person` | Kept. |
| `English.Agreement` | Kept. |
| `English.ConcordClass` | Kept. |
| `English.Agreement.concord` | Kept. |
| `English.InflectionalForm` | Kept. |
| `English.Voice` | Kept. |
| `English.Placement` | Kept. |
| `English.Polarity` | Kept. |
| `English.Finiteness` | Kept. |
| `English.FiniteForm` | Kept. |
| `English.Category` | Kept. |
| `English.Relation` | Kept. |
| `English.Complement` | Kept. |
| `English.FrameItem` | Kept. |
| `English.Coordinator` | Kept. |
| `English.Coordinator.surface` | Kept. |
| `English.Person.join` | Kept. |
| `English.Agreement.additive` | Kept. |
| `English.Construction` | Kept. |
| `English.Lexicon` | Kept. |
| `English.WordCategory` | Kept. |
| `English.AdjunctLicense` | Kept. |
| `English.coordinationResult` | Kept. |
| `English.paragraphItem` | Kept. |
| `English.documentItem` | Kept. |
| `English.coordinable` | Kept. |
| `English.InitialAdverbial` | Kept. |
| `English.DocumentProduction` | Kept. |
| `English.Production` | Kept. |
| `English.RelativeForm` | Kept. |
| `English.Syntax` | Kept. |
| `English.Syntax.relative` | Kept. |
| `English.Syntax.reminderFree` | Kept. |
| `English.reminderFreeChildren` | Kept. |
| `English.Syntax.coordinate` | Kept. |
| `English.SubjectPosition` | Kept. |
| `English.subjectAgreement` | Kept. |
| `English.FiniteLicense` | Kept. |
| `English.Syntax.antecedents` | Kept. |
| `English.childAntecedents` | Kept. |
| `English.Construction.childContext` | Kept. |
| `English.Construction.childContext_empty` | Kept. |
| `English.JudgesIn` | Kept. |
| `English.JudgeChildrenIn` | Kept. |
| `English.JudgeFrameIn` | Kept. |
| `English.JudgeParagraph` | Kept. |
| `English.Judges` | Kept. |
| `English.JudgeChildren` | Kept. |
| `English.JudgeFrame` | Kept. |
| `English.JudgesIn.closedNode` | Kept. |
| `English.DerivesIn` | Kept. |
| `English.Derives` | Kept. |
| `English.DocumentLinearizes` | Kept. |
| `English.Linearizes` | Kept. |
| `English.Realizes` | Kept. |
| `English.RealizeChildren` | Kept. |
| `English.Admissible` | Kept. |
| `English.Written` | Kept. |

### GrammaticalScope.lean

| Old declaration | Disposition |
|---|---|
| `English.GrammaticalScope.coord` | Kept. |
| `English.GrammaticalScope.adjunct` | Kept. |
| `English.GrammaticalScope.ScopeMove` | Kept. |
| `English.GrammaticalScope.ScopeStep` | Kept. |
| `English.GrammaticalScope.Reading` | Replaced by `English.GrammaticalScope.SchemaWitness`. Name the generic derivation/surface proof carrier separately from the v3 value. |
| `English.GrammaticalScope.ScopeRelated` | Kept. |
| `English.GrammaticalScope.two_regions` | Kept. |
| `English.GrammaticalScope.scopeSetoid` | Kept. |
| `English.GrammaticalScope.scopeClass` | Kept. |
| `English.GrammaticalScope.same_class_iff` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.constructionLexemes` | Kept. |
| `English.GrammaticalScope.lexicalLeaves` | Kept. |
| `English.GrammaticalScope.childLeaves` | Kept. |
| `English.GrammaticalScope.childLeaves_append` | Kept. |
| `English.GrammaticalScope.move_preserves_lexemes` | Kept. |
| `English.GrammaticalScope.step_preserves_lexemes` | Kept. |
| `English.GrammaticalScope.related_preserves_lexemes` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.hostNode` | Kept. |
| `English.GrammaticalScope.hostStructure` | Kept. |
| `English.GrammaticalScope.hostChildren` | Kept. |
| `English.GrammaticalScope.hostChildren_append` | Kept. |
| `English.GrammaticalScope.move_preserves_hosts` | Kept. |
| `English.GrammaticalScope.step_preserves_hosts` | Kept. |
| `English.GrammaticalScope.related_preserves_hosts` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.map_related` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.package` | Kept. |
| `English.GrammaticalScope.package_exact` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |

### Interactions.lean

| Old declaration | Disposition |
|---|---|
| `English.Interactions.NestedScope.pairNP` | Kept. |
| `English.Interactions.NestedScope.npAdj` | Kept. |
| `English.Interactions.NestedScope.during` | Kept. |
| `English.Interactions.NestedScope.withTurns` | Kept. |
| `English.Interactions.NestedScope.innerWide` | Kept. |
| `English.Interactions.NestedScope.innerNarrow` | Kept. |
| `English.Interactions.NestedScope.vpAdj` | Kept. |
| `English.Interactions.NestedScope.outerWide` | Kept. |
| `English.Interactions.NestedScope.outerNarrow` | Kept. |
| `English.Interactions.NestedScope.nestedSurface` | Kept. |
| `English.Interactions.NestedScope.pairNP_valid` | Kept. |
| `English.Interactions.NestedScope.npAdj_valid` | Kept. |
| `English.Interactions.NestedScope.inner_valid` | Kept. |
| `English.Interactions.NestedScope.nested_valid` | Kept. |
| `English.Interactions.NestedScope.four_distinct` | Kept. |
| `English.Interactions.NestedScope.alternatives` | Kept. |
| `English.Interactions.NestedScope.alternatives_admitted` | Kept. |
| `English.Interactions.NestedScope.all_four_survive` | Kept. |
| `English.Interactions.NestedScope.all_four_packed` | Kept. |
| `English.Interactions.NestedScope.quotedTree` | Kept. |
| `English.Interactions.NestedScope.quotedSurface` | Kept. |
| `English.Interactions.NestedScope.quoted_admitted` | Kept. |
| `English.Interactions.NestedScope.quoted_distinction` | Kept. |
| `English.Interactions.NestedScope.quoted_packing_retains_all` | Kept. |
| `English.Interactions.Quotation.gainNested` | Kept. |
| `English.Interactions.Quotation.instruction` | Kept. |
| `English.Interactions.Quotation.sentence` | Kept. |
| `English.Interactions.Quotation.deep_quote_written` | Kept. |

### RolePreference.lean

| Old declaration | Disposition |
|---|---|
| `English.RolePreference.object` | Kept. |
| `English.RolePreference.selectedTree` | Kept. |
| `English.RolePreference.postmodifierTree` | Kept. |
| `English.RolePreference.postmodifiers` | Kept. |
| `English.RolePreference.matchesRole` | Kept. |
| `English.RolePreference.FirstRole` | Kept. |
| `English.RolePreference.RoleStep` | Kept. |
| `English.RolePreference.role_step_derives` | Kept. |
| `English.RolePreference.immediate_same_surface` | Kept. |
| `English.RolePreference.nonfinal_occurrence_excluded` | Kept. |
| `English.RolePreference.occurrence_order` | Kept. |
| `English.RolePreference.role_candidate_cannot_skip` | Kept. |
| `English.RolePreference.markedCount` | Kept. |
| `English.RolePreference.roleCount` | Kept. |
| `English.RolePreference.roleChildren` | Kept. |
| `English.RolePreference.roleChildren_append` | Kept. |
| `English.RolePreference.role_step_increases` | Kept. |
| `English.RolePreference.Prefers` | Kept. |
| `English.RolePreference.preference_increases` | Kept. |
| `English.RolePreference.no_role_preference_cycle` | Kept. |
| `English.RolePreference.Selected` | Kept. |
| `English.RolePreference.role_worse_excluded` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.RolePreference.selected_exists` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.RolePreference.selected_enumeration` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.RolePreference.role_pair_selected` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.RolePreference.equal_role_counts_cannot_prefer` | Kept. |

### Scope.lean

| Old declaration | Disposition |
|---|---|
| `English.Scope.Host` | Kept. |
| `English.Scope.Host.sites` | Kept. |
| `English.Scope.FirstEligible` | Kept. |
| `English.Scope.first_eligible_unique` | Kept. |
| `English.Scope.first_eligible_member` | Kept. |
| `English.Scope.first_eligible_exists` | Kept. |
| `English.Scope.nonfinal_boundary` | Kept. |
| `English.Scope.frame_transparent` | Kept. |
| `English.Scope.role_edge_opaque` | Kept. |
| `English.Scope.first_eligible_skips` | Kept. |
| `English.Scope.Anchors` | Kept. |
| `English.Scope.flat` | Kept. |
| `English.Scope.nested` | Kept. |
| `English.Scope.Reading` | Replaced by `English.Scope.AnchorPattern`. Name the raw host/anchor pattern separately from a grammatical Reading. |
| `English.Scope.anchor_shape_separate` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.Scope.joint` | Kept. |
| `English.Scope.joint_alternatives_exact` | Kept. |

### ScopeInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.GrammaticalScope.independent_modifier_regions` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.wideReading` | Kept. |
| `English.GrammaticalScope.outer_move` | Kept. |
| `English.GrammaticalScope.inner_move` | Kept. |
| `English.GrammaticalScope.nested_scope_related` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.nested_package_exact` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.quoted_step` | Kept. |
| `English.GrammaticalScope.quoted_scope_related` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.GrammaticalScope.homographs_separate` | Kept. |
| `English.GrammaticalScope.pair_valid` | Kept. |
| `English.GrammaticalScope.association_separate` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |

### Selection.lean

| Old declaration | Disposition |
|---|---|
| `English.Selection.Claims` | Kept. |
| `English.Selection.Claims.rank` | Kept. |
| `English.Selection.Policy` | Kept. |
| `English.Selection.Prefers` | Kept. |
| `English.Selection.Selected` | Kept. |
| `English.Selection.selected_sound` | Kept. |
| `English.Selection.selected_enumeration` | Kept. |
| `English.Selection.selected_permutation` | Kept. |
| `English.Selection.frame_role_preempts` | Kept. |
| `English.Selection.identity_preempts` | Kept. |
| `English.Selection.preference_decreases` | Kept. |
| `English.Selection.preference_asymmetric` | Kept. |
| `English.Selection.preference_transitive` | Kept. |
| `English.Selection.PreferenceChain` | Kept. |
| `English.Selection.no_preference_cycle` | Kept. |
| `English.Selection.admitted_maximum` | Kept. |
| `English.Selection.selected_exists` | Kept. |
| `English.Selection.Package` | Kept. |
| `English.Selection.Packs` | Kept. |
| `English.Selection.pack` | Kept. |
| `English.Selection.packing_exists` | Kept. |
| `English.Selection.packing_exact` | Kept. |
| `English.Selection.packing_unique` | Kept. |
| `English.Selection.packing_enumeration` | Kept. |
| `English.Selection.different_classes_separate` | Kept. |
| `English.Selection.unpack_exact` | Kept. |
| `English.Selection.packed_admissible` | Kept. |

### SelectionWitnesses.lean

| Old declaration | Disposition |
|---|---|
| `English.SelectionWitnesses.neutral` | Kept. |
| `English.SelectionWitnesses.neutral_selected` | Kept. |
| `English.SelectionWitnesses.ModifierScope` | Kept. |
| `English.SelectionWitnesses.modifier_scope_preserves` | Kept. |
| `English.SelectionWitnesses.modifier_scope_unique` | Kept. |
| `English.SelectionWitnesses.acyclic_tie` | Kept. |
| `English.SelectionWitnesses.modifier_package_retains_both` | Kept. |
| `English.SelectionWitnesses.homographs` | Kept. |
| `English.SelectionWitnesses.homographs_admitted` | Kept. |
| `English.SelectionWitnesses.same_surface_separate` | Kept. |
| `English.SelectionWitnesses.CrossHost.pair` | Kept. |
| `English.SelectionWitnesses.CrossHost.auxiliary` | Kept. |
| `English.SelectionWitnesses.CrossHost.adjunct` | Kept. |
| `English.SelectionWitnesses.CrossHost.outside` | Kept. |
| `English.SelectionWitnesses.CrossHost.inside` | Kept. |
| `English.SelectionWitnesses.CrossHost.finalConjunct` | Kept. |
| `English.SelectionWitnesses.CrossHost.surface` | Kept. |
| `English.SelectionWitnesses.CrossHost.alternatives` | Kept. |
| `English.SelectionWitnesses.CrossHost.pair_admitted` | Kept. |
| `English.SelectionWitnesses.CrossHost.auxiliary_admitted` | Kept. |
| `English.SelectionWitnesses.CrossHost.adjunct_admitted` | Kept. |
| `English.SelectionWitnesses.CrossHost.cross_host_admitted` | Kept. |
| `English.SelectionWitnesses.CrossHost.cross_host_scope` | Kept. |
| `English.SelectionWitnesses.declaredPrinciples` | Kept. |
| `English.SelectionWitnesses.ordered_principles_winner` | Kept. |
| `English.SelectionWitnesses.inadmissible_cannot_suppress` | Kept. |
| `English.SelectionWitnesses.qualification_sites_excluded` | Kept. |
| `English.SelectionWitnesses.measure_postmodifier_excluded` | Kept. |

### SharingInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.SharingInteractions.Heads.adjectives` | Kept. |
| `English.SharingInteractions.Heads.wide` | Kept. |
| `English.SharingInteractions.Heads.gapped` | Kept. |
| `English.SharingInteractions.Heads.raised` | Kept. |
| `English.SharingInteractions.Heads.both_admitted` | Kept. |
| `English.SharingInteractions.Heads.both_checked` | Kept. |
| `English.SharingInteractions.Heads.a` | Kept. |
| `English.SharingInteractions.Heads.b` | Kept. |
| `English.SharingInteractions.Heads.shared_head_class` | Replaced by `English.SharingInteractions.Heads.shared_head_related`. Preserve the witnessed shared-head scope move without a global Analysis key. |
| `English.SharingInteractions.Heads.distinct` | Kept. |
| `English.SharingInteractions.Determiners.two` | Kept. |
| `English.SharingInteractions.Determiners.wide` | Kept. |
| `English.SharingInteractions.Determiners.narrow` | Kept. |
| `English.SharingInteractions.Determiners.both_admitted` | Kept. |
| `English.SharingInteractions.Determiners.shared_determiner_class` | Kept. |

### Surface.lean

| Old declaration | Disposition |
|---|---|
| `English.Atom` | Kept. |
| `English.Surface` | Kept. |
| `English.Atom.text` | Kept. |
| `English.Atom.separator` | Kept. |
| `English.Atom.capitalize` | Kept. |
| `English.terminalPeriod` | Kept. |
| `English.Surface.finishSentence` | Kept. |
| `English.Atom.nestedQuote` | Kept. |
| `English.Surface.quote` | Kept. |
| `English.Surface.join` | Kept. |
| `English.Surface.capitalize` | Kept. |
| `English.Surface.followingLine` | Kept. |
| `English.Spells` | Kept. |

### Witnesses.lean

| Old declaration | Disposition |
|---|---|
| `English.Witnesses.Lexeme` | Kept. |
| `English.Witnesses.lexicon` | Kept. |
| `English.Witnesses.narrow` | Kept. |
| `English.Witnesses.shared` | Kept. |
| `English.Witnesses.surface` | Kept. |
| `English.Witnesses.narrow_derives` | Kept. |
| `English.Witnesses.shared_derives` | Kept. |
| `English.Witnesses.white_realizes` | Kept. |
| `English.Witnesses.creatures_realizes` | Kept. |
| `English.Witnesses.artifacts_realizes` | Kept. |
| `English.Witnesses.narrow_realizes` | Kept. |
| `English.Witnesses.shared_realizes` | Kept. |
| `English.Witnesses.distinct_analyses` | Kept. |
| `English.Witnesses.adjective_not_noun` | Kept. |

### WordFormInteractions.lean

| Old declaration | Disposition |
|---|---|
| `English.WordFormInteractions.Lexeme` | Replaced by `English.LexicalWitnesses.Lexeme`. Use the common declared environment instead of a second fixture-specific vocabulary. |
| `English.WordFormInteractions.pastBe` | Kept. |
| `English.WordFormInteractions.lexicon` | Replaced by `English.GrammarWitnesses.grammar`. Lift complete licensed alternatives into the grammar. |
| `English.WordFormInteractions.pronoun` | Replaced by `English.LexicalWitnesses.it`; `English.LexicalWitnesses.they`; `English.LexicalWitnesses.you`; `English.GrammarWitnesses.clause_derives`. Preserve the three named pronoun sentences and generalize the derivation over licensed pronoun values. |
| `English.WordFormInteractions.exiled` | Replaced by `English.GrammarWitnesses.exiledTree`; `English.GrammarWitnesses.exiled_derives`. Retain the declared passive participle in the common grammar. |
| `English.WordFormInteractions.passive` | Replaced by `English.GrammarWitnesses.passive`. The auxiliary occurrence retains its selected Word Form. |
| `English.WordFormInteractions.clause` | Replaced by `English.GrammarWitnesses.clause`. The finite host inspects the selected auxiliary agreement. |
| `English.WordFormInteractions.clause_derives` | Replaced by `English.GrammarWitnesses.clause_derives`. Require licensed subject and auxiliary bundles with matching agreement; the old unconstrained hypothesis is repaired. |
| `English.WordFormInteractions.finite_clause` | Replaced by `English.GrammarWitnesses.clause_valid`; `English.GrammarWitnesses.clause_realizes`. Retain general composition and surface evidence, constrained by selected licensed bundles; the arbitrary was-or-were premise was the exposed defect. |
| `English.WordFormInteractions.singular_was` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.WordFormInteractions.plural_were` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.WordFormInteractions.addressee_were` | Kept; proof/signature re-spelled for the renamed schema carrier or v3 admission. |
| `English.WordFormInteractions.expected_word_form_licenses` | Kept. |
| `English.WordFormInteractions.wrong_addressee_was_admitted` | Replaced by `English.WordFormInteractions.wrong_addressee_was_excluded`. Repair the exposed wrong analysis: the same crossed form is now excluded at actual v3 admission. |
| `English.WordFormInteractions.wrong_singular_were_admitted` | Replaced by `English.WordFormInteractions.wrong_singular_were_excluded`. Repair the exposed wrong analysis: the same crossed form is now excluded at actual v3 admission. |
| `English.WordFormInteractions.wrong_plural_was_admitted` | Replaced by `English.WordFormInteractions.wrong_plural_was_excluded`. Repair the exposed wrong analysis: the same crossed form is now excluded at actual v3 admission. |
| `English.WordFormInteractions.concord_does_not_determine_word_form` | Kept. |
| `English.WordFormInteractions.both_preterite_spellings` | Replaced by `English.WordFormInteractions.distinct_passive_word_forms`; `English.WordFormInteractions.singular_was`; `English.WordFormInteractions.plural_were`. Retire the wrong same-tree analysis. Both legitimate sentences survive with distinct selected Word Forms. |
