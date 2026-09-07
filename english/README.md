# Oracle English grammar in Lean

This independent Lake project states and proves properties of the proposed
English v3 grammar. It has no dependency on or interaction with `lean/`
(Semantics). It does not implement parsing, chart scheduling or a packed forest.

Run the complete grammar and witness gate from the repository root:

```sh
english/scripts/build
```

The project has its own toolchain, manifest and artifacts, with no package
dependencies. `English.lean` imports every checked module. The current authority
is [independent lexical analysis and retained readings](../docs/decisions/english-lexical-analysis.md);
the [workbench decision](../docs/decisions/english-lean-design-workbench.md)
defines the bounded proof contract. The older [source map](../docs/english-grammar-design.md)
and [review](../docs/english-grammar-review.md) preserve the intended breadth and
historical proof obligations. [DECLARATIONS.md](DECLARATIONS.md) accounts for
every declaration in the previous workbench.

## The grammar relation

`English.Analysis` is the import entry point. The canonical value is
`English.Reading L`: a constituent tree whose lexical occurrences retain one
`WordForm L` each. A Word Form carries its declared lexical identity, correlated
Feature Bundle, spelling, capitalization and provenance. Proofs of admission
are separate from that value.

`LexicalAnalysis environment surface word` relates a surface to a declared
lexical alternative without a syntactic context. A declaration supplies bundles,
morphology and grammatical properties. `Morphology.forms` uses one default for
a bundle unless an explicit override replaces it; a list of variants must be
explicitly declared. Unknown identities, undeclared bundles and crossed
spelling/agreement combinations are excluded. This is a logical relation, not
a tokenizer or an executable lexical analyzer.

`Lexical.lexicon`, `Lexical.features` and `Lexical.dependencies` instantiate the
shared grammatical schemas with those complete alternatives. A finite host
must inspect the agreement in the selected verb bundle; it cannot take the
spelling from one alternative and agreement from another. Lexical frame
selection likewise uses the declared form, voice and ordered frame. The frame
contains lexical marker identities; an actual marker occurrence separately
carries its licensed spelling.

`Reading.Valid` combines contextual category/frame derivation,
`Features.Conforms`, `Dependencies.Safe` and `Reading.GrammarConforms`.
`Reading.Admitted` additionally requires the independent `Reading.Realizes`
relation to the given surface. Recursive constituents use the same judgments:
clauses occur inside subordinate clauses, relatives, sentences and documents;
nominals and prepositions fill frames; measures can fill prepositional
Complements; shared gaps and preceding-context ellipsis remain distinct.

Parents can inspect selected finite agreement and tense, inflection and voice,
count/mass use, nominal Case, selected frames, ordered extraction resources,
coordination topology and recoverability context. `Reading.HasTense` exposes
the represented finite predicate's tense; it does not claim a complete account
of temporal interpretation or sequence of tense. These relations specify the
information that Rust must preserve without choosing chart keys or node layout.

## Readings and roundtrips

`Reading.readings` is the set of all admitted values, across lexical and
structural ambiguity classes. `Reading.Preference` annotates two members of
that set and cannot revoke either's admission. `duplicate_derivations` checks
that two proofs of one value do not create two Readings. No selection or scope
quotient is part of v3 admission.

`AnalysisRoundtrip` and `ValueRoundtrip` state obligations on independently
specified construction and realization operations; neither operation is
implemented here. Construction must establish grammatical validity, and the
value law retains the exact structure, lexical identities, features and spelling.
The laws are statements, not the definitions of those operations or additional
claims of implementation correctness. The workbench's proof evidence concerns
the grammar structure and its permitted and excluded compositions.

Declared spelling variants and initial capitalization live in the value.
Document realization checks required initial capitalization instead of silently
rewriting lexical leaves. An already-capitalized declared spelling has no
redundant initial-capitalization alternative. The variant witnesses retain
*indexes*, *indices* and *Indices* as distinct values with their exact surfaces.
The model works with annotated `Surface` atoms; it does not prove tokenizer
correctness or arbitrary raw-source byte preservation.

## Connected family evidence

Names below are in `English`, with their module/namespace prefixes shown.
Lower-level exclusion laws range over the same general production and feature
schemas used by v3 admission; they are not separate permissive grammars.

| Family | Inhabitant and cross-family use | Exclusion |
|---|---|---|
| Documents, sentences, clauses | `FamilyWitnesses.document_admitted`: “If creatures attack, attack.” combines a document, sentence, conditional and two clauses | `FamilyWitnesses.document_rejects_bare_nominal`; `DocumentShape` and `DocumentCollections` constrain collection items/cardinalities |
| Lexical predicates and frames | `GrammarWitnesses.singular_was`, `plural_were`, `addressee_were`: selected finite auxiliary with passive VP; `AmbiguityWitnesses.verb_admitted`: perception frame with NP and VP | `GrammarWitnesses.wrong_plural_was_excluded`, `wrong_singular_were_excluded`, `wrong_addressee_was_excluded`; `Composition.frame_rejects_extra_complement` |
| Nominals, determiners, adjectives | `FamilyWitnesses.modified_admitted`; `NominalWitnesses.count_admitted`, `mass_admitted`, `quantified_mass_admitted`; modified head participates in shared-gap coordination | `NominalWitnesses.count_determiner_rejects_mass`; `AnalysisInteractions.Rejections.reversed_target_not_candidate` preserves Targeting Marker order |
| Subordination | `FamilyWitnesses.document_admitted` embeds a finite subordinate clause in an initial adverbial | `FamilyWitnesses.wrong_subordinate_finiteness` |
| Prepositions | `FamilyWitnesses.preposition_admitted` combines a selected NP with a preposition; `NotationWitnesses.measure_preposition_interaction` selects a Measure Phrase | `FamilyWitnesses.wrong_preposition_complement` |
| Relatives and extraction | `DependencyWitnesses.relative_admitted`, `shared_relative_admitted`; `NominalWitnesses.target_relative_admitted` embeds the Target Verb frame in a relative | `DependencyWitnesses.zero_subject_relative_excluded` |
| Coordination and sharing | `DependencyWitnesses.shared_relative_admitted`, `raised_admitted`: shared relative Subjects and a following shared modified head | `DependencyWitnesses.ordinary_coordination_cannot_discharge_shared_gap`, `single_gap_is_not_sharing` |
| Measures | `NotationWitnesses.measure_admitted`, `measure_preposition_interaction`: arithmetic notation inside a PP | `NotationWitnesses.measure_not_a_count_determiner` |
| Recoverability and ellipsis | `DependencyWitnesses.contextual_ellipsis_admitted` uses antecedents projected from the overt passive predicate | `DependencyWitnesses.missing_antecedent_excluded`, `wrong_voice_antecedent_excluded` |
| Type Lines | `NotationWitnesses.type_line_admitted`: “Artifact — Golem” combines typed lexical entries and ordered document collections | `NotationWitnesses.type_line_requires_types`, `type_line_order` |
| Target Verb / Targeting Marker | `NominalWitnesses.target_rivalry` retains NP and imperative-clause analyses of “target creatures”; `target_relative_admitted` connects the finite verb to extraction | `NominalWitnesses.marker_is_not_a_verb` |

`LexicalWitnesses.noun_verb_homographs` and
`noun_determinative_homographs` retain separately declared identities.
`crossed_was_plural` and the grammatical exclusions above reject a jointly
invalid combination of independently licensed alternatives.
`AmbiguityWitnesses.unrelated_readings_retained` proves both structures for
“I saw her duck”: possessive Determiner + noun Object, or pronoun Object +
bare Verb Complement. `preferred_and_nonpreferred_remain` uses an illustrative
preference only; it asserts no linguistic ranking.
`control_unambiguous` proves uniqueness over the whole admission relation for
“Artifact” at the Type category in the same declared environment, rather than
checking a hand-picked singleton list.

## Retained schemas and remaining obligations

`Grammar`, `FeatureConstraints`, `CaseConstraints` and `Dependencies` retain
the reusable production schemas. Their older `Admissible` and
`Dependencies.Admitted` contracts do not themselves assert v3 lexical licensing.
Old witness modules continue to prove precisely those lower-level contracts.
`WordFormInteractions` has been re-spelled at actual v3 admission: the former
crossed-form counterexamples now prove rejection, with the same valid English
sentences retained alongside them.

`Selection` and `RolePreference` retain optional preference-view algebra;
`Scope`, `GrammaticalScope` and `FrameScope` retain local scope and projection
laws. A selected view or one scope class is never the complete v3 reading set.
Those modules are outside the admission import closure. Their former carrier
names are now `Scope.AnchorPattern` and `GrammaticalScope.SchemaWitness`, leaving
one public `Reading` concept. The obsolete `Analysis.Selected`, global
identity preference and selected-class packaging interface have been retired.

The [v3 proof-audit ticket](../docs/tickets/planned/english-v3-lean-proof-audit.md)
owns the remaining inherited audit obligations: flat serial-comma versus
nested coordination, fuller tense propagation, head-owned genitive countability,
extraction/anchor projection strength, ordinary-word payload boundaries, and
additional adjective, placement, sharing and auxiliary cases. The
[source obligation register](../docs/english-grammar-migration-obligations.md)
continues to route richer linguistic elaborations and production correspondence.
This landing claims connected families and the named proofs, not an exhaustive
Oracle grammar, a global absence of unintended ambiguity, or correctness of Rust.
