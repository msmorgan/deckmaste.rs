use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::BoundedParseOutcome;
use deckmaste_english_v2::parser::Expectation;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseAnalysis;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::SelectionLoserReason;
use deckmaste_english_v2::parser::TerminalClass;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;

fn card_name_provider() -> CatalogProviderRows {
    CatalogProviderRows::new(
        CatalogProvider::CardNames,
        [
            CatalogProviderRow::new("alpha", "Alpha", Onset::Vowel),
            CatalogProviderRow::new("alpha-beta", "Alpha Beta", Onset::Vowel),
            CatalogProviderRow::new("urzas-saga", "Urza's Saga", Onset::Vowel),
            CatalogProviderRow::new("seven-dwarves", "Seven Dwarves", Onset::Consonant),
        ],
    )
}

fn environment() -> ParserEnvironment {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_parts(declarations, [card_name_provider()])
        .expect("builtin-v2 declaration and catalog environment freezes")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("required declarations are present")
}

fn context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name, false, Onset::Consonant)
        .expect("test card name is a valid parse context")
}

fn legendary_context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name, true, Onset::Consonant)
        .expect("test legendary card name is a valid parse context")
}

fn single_paragraph(oracle_text: &OracleText) -> &Sentences {
    let [DocumentBlock::Ability(ability)] = oracle_text.blocks.as_slice() else {
        panic!("expected exactly one paragraph ability block: {oracle_text:?}");
    };
    let Ability::Plain(Plain { body }) = ability.as_ref() else {
        panic!("expected a plain paragraph ability: {oracle_text:?}");
    };
    let AbilityBody::Sentences(sentences) = body else {
        panic!("expected a sentence paragraph: {oracle_text:?}");
    };
    sentences
}

fn claims_overlap(left: TextSpan, right: TextSpan) -> bool {
    left.start < right.end && right.start < left.end
}

#[test]
fn indefinite_articles_are_guarded_by_frozen_onset_without_ast_article_state() {
    let environment = environment();
    let parser = Parser::new(environment.clone()).expect("required declarations are present");
    let context = context("Context Card");

    for text in ["Destroy a player.", "Destroy an artifact."] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text} must parse: {error:?}"));
        let Ability::Plain(Plain { body }) = &parsed else {
            panic!("indefinite destroy is a paragraph")
        };
        let AbilityBody::Sentences(paragraph) = body else {
            panic!("indefinite destroy is a sentence paragraph")
        };
        let [Sentence::Imperative(imperative)] = paragraph.sentences() else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let Predicate::Atomic(predicate) = imperative.predicate() else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) = predicate.as_ref() else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let LexicalVerbPhrase::TransitiveLexicalVerbPhrase(transitive_lexical_verb_phrase) =
            frame.as_ref()
        else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
            head: _,
            object: Object::ObjectNominal(object),
        }) = transitive_lexical_verb_phrase.as_ref()
        else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let UnqualifiedReference::DeterminedNominal(DeterminedNominal {
            nominal: Nominal::BareSingularNominal(nominal),
            ..
        }) = unqualified_reference(object.value())
        else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        assert!(matches!(nominal.head(), Head::NounSingularHead(_)));
        assert_eq!(parsed.render(&context, &environment), text);
        let ownership = parser
            .analyze(text, &context)
            .ownership()
            .expect("selected indefinite phrase owns all bytes")
            .clone();
        assert!(ownership.summary().covered());
        assert!(
            ownership.parsed_claims().iter().any(|claim| {
                claim.kind() == LexicalProvenanceKind::Codec
                    && text[claim.span().start..claim.span().end].trim()
                        == if text.contains(" an ") { "an" } else { "a" }
            }),
            "article has exact vocabulary ownership: {ownership:?}"
        );
    }

    for text in ["Destroy an player.", "Destroy a artifact."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong article must not select: {text}",
        );
    }

    let mut declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("integrated builtin-v2 declarations load");
    declarations.push(
        deckmaste_construction_core::macro_def::read_str(
            "/synthetic/types/Herb.ron",
            r#"Type(
                name:"Herb",
                spelling:"herb",
                grammar:Noun(singular:"herb",singular_onset:Vowel),
            )"#,
        )
        .expect("open declaration onset override normalizes"),
    );
    let overridden_environment =
        ParserEnvironment::try_from_parts(declarations, [card_name_provider()])
            .expect("overridden environment freezes");
    let overridden_parser =
        Parser::new(overridden_environment.clone()).expect("required declarations are present");
    let text = "Destroy an herb.";
    let parsed = overridden_parser
        .parse(text, &context)
        .expect("scanner/build use frozen onset rather than recomputing from `herb`");
    assert_eq!(parsed.render(&context, &overridden_environment), text);
    assert!(
        overridden_parser
            .parse("Destroy a herb.", &context)
            .is_err()
    );
}

fn assert_one_structural_claim(
    claims: &[deckmaste_english_v2::parser::LexicalClaim],
    span: TextSpan,
    owner: &str,
) {
    let matching = claims
        .iter()
        .filter(|claim| claim.span() == span)
        .collect::<Vec<_>>();
    assert_eq!(matching.len(), 1, "exactly one claim must own {span:?}");
    assert_eq!(matching[0].stable_owner_id(), owner);
    assert!(
        claims
            .iter()
            .filter(|claim| claim.span() != span)
            .all(|claim| !claims_overlap(claim.span(), span)),
        "no neighboring lexical claim may overlap {span:?}",
    );
}

#[test]
fn empty_oracle_text_is_a_totally_owned_zero_block_document() {
    let parser = parser();
    let environment = environment();
    let context = context("Grizzly Bears");
    let expected = OracleText {
        blocks: Box::default(),
    };

    let analysis = parser.analyze_oracle_text("", &context);
    assert_eq!(analysis.selected(), Some(&expected));
    assert_eq!(expected.render(&context, &environment), "");

    let ownership = analysis
        .ownership()
        .expect("empty OracleText is a selected parse");
    assert_eq!(ownership.rendered_text(), "");
    assert!(ownership.parsed_claims().is_empty());
    assert!(ownership.rendered_claims().is_empty());
    assert!(ownership.failures().is_empty());

    let summary = ownership.summary();
    assert!(summary.covered());
    assert_eq!(summary.claims(), 0);
    assert_eq!(summary.claimed_bytes(), 0);
    assert_eq!(summary.form_literal_claims(), 0);
    assert_eq!(summary.form_literal_bytes(), 0);
    assert_eq!(summary.vocab_claims(), 0);
    assert_eq!(summary.vocab_bytes(), 0);
    assert_eq!(summary.lexeme_claims(), 0);
    assert_eq!(summary.lexeme_bytes(), 0);
    assert_eq!(summary.codec_claims(), 0);
    assert_eq!(summary.codec_bytes(), 0);
    assert_eq!(summary.identity_claims(), 0);
    assert_eq!(summary.identity_bytes(), 0);
    assert_eq!(summary.gap_spans(), 0);
    assert_eq!(summary.gap_bytes(), 0);
    assert_eq!(summary.overlap_spans(), 0);
    assert_eq!(summary.overlap_bytes(), 0);
    assert_eq!(summary.synthetic_claims(), 0);
    assert_eq!(summary.provenance_plan_mismatches(), 0);
}

#[test]
fn oracle_text_parses_all_eight_two_sentence_faces_as_one_ordered_paragraph() {
    let parser = parser();
    let environment = environment();
    for (card_name, text, first_sentence, second_sentence) in [
        (
            "Cursebreak",
            "Destroy target enchantment. You gain 2 life.",
            "Destroy target enchantment.",
            "You gain 2 life.",
        ),
        (
            "Drain the Well",
            "Destroy target land. You gain 2 life.",
            "Destroy target land.",
            "You gain 2 life.",
        ),
        (
            "Into the Maw of Hell",
            "Destroy target land. Into the Maw of Hell deals 9 damage to target creature.",
            "Destroy target land.",
            "Into the Maw of Hell deals 9 damage to target creature.",
        ),
        (
            "Lich's Caress",
            "Destroy target creature. You gain 3 life.",
            "Destroy target creature.",
            "You gain 3 life.",
        ),
        (
            "Maw of the Mire",
            "Destroy target land. You gain 4 life.",
            "Destroy target land.",
            "You gain 4 life.",
        ),
        (
            "Ray of Dissolution",
            "Destroy target enchantment. You gain 3 life.",
            "Destroy target enchantment.",
            "You gain 3 life.",
        ),
        (
            "Sephiroth's Intervention",
            "Destroy target creature. You gain 2 life.",
            "Destroy target creature.",
            "You gain 2 life.",
        ),
        (
            "Winter's Intervention",
            "Winter's Intervention deals 4 damage to target creature. You gain 3 life.",
            "Winter's Intervention deals 4 damage to target creature.",
            "You gain 3 life.",
        ),
    ] {
        let context = if card_name == "Zacama, Primal Calamity" {
            legendary_context(card_name)
        } else {
            context(card_name)
        };
        let parsed = parser
            .parse_oracle_text(text, &context)
            .unwrap_or_else(|error| panic!("{card_name} must parse: {error:?}"));
        let paragraph = single_paragraph(&parsed);
        assert_eq!(paragraph.sentences().len(), 2, "{card_name}");
        assert_eq!(
            paragraph.sentences()[0].render(&context, &environment),
            first_sentence,
            "{card_name} first sentence",
        );
        assert_eq!(
            paragraph.sentences()[1].render(&context, &environment),
            second_sentence,
            "{card_name} second sentence",
        );
        assert_eq!(parsed.render(&context, &environment), text, "{card_name}");

        let analysis: ParseAnalysis<OracleText> = parser.analyze_oracle_text(text, &context);
        assert_eq!(analysis.selected(), Some(&parsed), "{card_name}");
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("{card_name} has selected ownership"));
        assert!(ownership.failures().is_empty(), "{card_name}");
        assert!(ownership.summary().covered(), "{card_name}");
    }
}

#[test]
fn oracle_text_structural_boundaries_have_exact_generated_claims() {
    let parser = parser();
    let context = context("Sephiroth's Intervention");
    let text = "Destroy target creature. You gain 2 life.";
    let analysis = parser.analyze_oracle_text(text, &context);
    let ownership = analysis.ownership().expect("OracleText is selected");
    assert!(ownership.summary().covered());

    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 23, end: 24 },
        "structural:Sentences/sentences/terminator/0",
    );
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 24, end: 25 },
        "structural:Sentences/sentences/separator/uniform/0",
    );
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 40, end: 41 },
        "structural:Sentences/sentences/terminator/0",
    );
}

#[test]
fn oracle_text_lf_separates_blocks_without_flattening_or_storage() {
    let parser = parser();
    let environment = environment();
    let context = context("Sephiroth's Intervention");
    let one_block_text = "Destroy target creature. You gain 2 life.";
    let two_block_text = "Destroy target creature.\nYou gain 2 life.";
    let one_block = parser
        .parse_oracle_text(one_block_text, &context)
        .expect("space-separated text parses as one block");
    let two_blocks = parser
        .parse_oracle_text(two_block_text, &context)
        .expect("LF-separated text parses as two blocks");

    assert_ne!(one_block, two_blocks);
    let [
        DocumentBlock::Ability(first_ability),
        DocumentBlock::Ability(second_ability),
    ] = two_blocks.blocks.as_slice()
    else {
        panic!("LF must preserve two paragraph blocks: {two_blocks:?}");
    };
    let (Ability::Plain(Plain { body: first }), Ability::Plain(Plain { body: second })) =
        (first_ability.as_ref(), second_ability.as_ref())
    else {
        panic!("LF must preserve two plain paragraph abilities: {two_blocks:?}");
    };
    let (AbilityBody::Sentences(first), AbilityBody::Sentences(second)) = (first, second) else {
        panic!("LF must preserve two sentence paragraphs: {two_blocks:?}");
    };
    assert_eq!(first.sentences().len(), 1);
    assert_eq!(second.sentences().len(), 1);
    assert_eq!(two_blocks.render(&context, &environment), two_block_text);

    let analysis = parser.analyze_oracle_text(two_block_text, &context);
    let ownership = analysis.ownership().expect("two blocks are selected");
    assert!(ownership.summary().covered());
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 24, end: 25 },
        "structural:OracleText/blocks/separator/uniform/0",
    );
}

#[test]
fn oracle_text_trace_retains_its_exact_value_type_and_root_name() {
    let parser = parser();
    let context = context("Sephiroth's Intervention");
    let trace: ParserTrace<OracleText> = parser.trace_oracle_text(
        "Destroy target creature. You gain 2 life.",
        &context,
        TraceLimits::new(usize::MAX),
    );

    assert_eq!(trace.root_name(), "OracleText");
    assert!(trace.into_parse_result().is_ok());
}

fn self_reference(spelling: SelfReferenceSpelling, card_name: &str) -> SourceSelfReference {
    let context = if spelling == SelfReferenceSpelling::Abbreviated {
        legendary_context(card_name)
    } else {
        context(card_name)
    };
    SourceSelfReference::new(spelling, &context).expect("test spelling is valid for its context")
}

fn singular_nominal(noun: Noun) -> Nominal {
    Nominal::BareSingularNominal(
        BareSingularNominal::new(Head::NounSingularHead(
            NounSingularHead::new(noun).expect("test noun is countable"),
        ))
        .expect("the Singular nominal accepts a Singular Head"),
    )
}

fn plural_nominal(noun: Noun) -> Nominal {
    Nominal::BarePluralNominal(
        BarePluralNominal::new(plural_head(noun))
            .expect("the Plural nominal accepts a Plural Head"),
    )
}

fn plural_head(noun: Noun) -> Head {
    Head::NounPluralHead(NounPluralHead::new(noun).expect("test noun is countable"))
}

fn singular_nominal_value(noun: Noun) -> Nominal {
    singular_nominal(noun)
}

fn plural_nominal_value(noun: Noun) -> Nominal {
    plural_nominal(noun)
}

fn singular_simple_determinative(head: DeterminativeHeadLemma) -> Determinative {
    Determinative::SingularSimpleDeterminative(SingularSimpleDeterminative {
        head: DeterminativeHead::Closed(head),
    })
}

fn plural_simple_determinative(head: DeterminativeHeadLemma) -> Determinative {
    Determinative::PluralSimpleDeterminative(PluralSimpleDeterminative {
        head: DeterminativeHead::Closed(head),
    })
}

fn determined_nominal(det: Determinative, nominal: Nominal) -> UnqualifiedReference {
    UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(Determiner::Headed(det), nominal)
            .expect("test determiner and nominal number agree"),
    )
}

fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
            UnqualifiedPostmodifiedReference {
                reference: Box::new(reference),
            },
        )),
    })
}

fn unqualified_reference(noun_phrase: &NounPhrase) -> &UnqualifiedReference {
    let NounPhrase::QualifiedNounPhrase(qualified) = noun_phrase else {
        panic!("expected an unqualified noun phrase: {noun_phrase:?}")
    };
    let PostmodifiedReference::UnqualifiedPostmodifiedReference(reference) =
        qualified.reference.as_ref()
    else {
        panic!("expected an unmodified reference: {noun_phrase:?}")
    };
    &reference.reference
}

fn indefinite(noun: Noun) -> NounPhrase {
    noun_phrase(determined_nominal(
        singular_simple_determinative(DeterminativeHeadLemma::IndefiniteArticle),
        singular_nominal_value(noun),
    ))
}

fn target_noun(noun: Noun) -> NounPhrase {
    noun_phrase(determined_nominal(
        Determinative::TargetingMarkerDeterminative(TargetingMarkerDeterminative {
            marker: TargetingMarker::Target,
        }),
        singular_nominal_value(noun),
    ))
}

fn creatures_you_control_with_power_at_most_two() -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::PrepositionalQualifiedReference(
            PrepositionalQualifiedReference::new(
                Box::new(PostmodifiedReference::RelativeQualifiedReference(
                    RelativeQualifiedReference::new(
                        Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                            UnqualifiedPostmodifiedReference {
                                reference: Box::new(UnqualifiedReference::DeterminedNominal(
                                    DeterminedNominal::new(
                                        Determiner::Zero,
                                        plural_nominal_value(creatures()),
                                    )
                                    .expect("zero-headed plural is valid for a determined nominal"),
                                )),
                            },
                        )),
                        Box::new(ObjectGapRelativeClause::Positive(Box::new(
                            PositiveObjectGapRelativeClause::PositiveObjectGapRelative(
                                PositiveObjectGapRelativeClauseValue {
                                    subject: subject_you(),
                                    head: core_transitive_head(CoreVerbIdentity::Control),
                                },
                            ),
                        ))),
                    )
                    .expect("the relative clause is admissible for the reference"),
                )),
                Box::new(PrepositionalPhrase::PrepositionalPhrase(
                    PrepositionalPhraseValue::new(
                        Preposition::With,
                        Box::new(PrepositionalComplement::ScalarMeasure(
                            ScalarMeasureValue::ScalarMeasureValue(ScalarMeasureValueValue {
                                measure: ScalarMeasure::NominalScalarMeasure(
                                    NominalScalarMeasure::new(singular_nominal(Noun::Lexeme(
                                        CommonNoun::Power,
                                    )))
                                    .expect("power is a Singular nominal scalar measure"),
                                ),
                                value: ScalarMeasureAssignedValue::Comparison(
                                    ScalarComparison::ScalarOrLess(
                                        ScalarOrLess::new(
                                            ScalarThreshold::FixedScalarThreshold(
                                                FixedScalarThreshold {
                                                    value: ScalarNumber { magnitude: 2 },
                                                },
                                            ),
                                            ComparisonDirection::Less,
                                        )
                                        .expect(
                                            "the closed member satisfies the direction requirement",
                                        ),
                                    ),
                                ),
                            }),
                        )),
                    )
                    .expect("the preposition licenses the complement"),
                )),
            )
            .expect("a declared object-attachment licence admits the scalar postmodifier"),
        )),
    })
}

fn number_of(counted: Object) -> NounPhrase {
    let number = determined_nominal(
        singular_simple_determinative(DeterminativeHeadLemma::DefiniteArticle),
        singular_nominal_value(Noun::Lexeme(CommonNoun::Number)),
    );
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::RelationalQualifiedReference(
            RelationalQualifiedReference::new(
                Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                    UnqualifiedPostmodifiedReference {
                        reference: Box::new(number),
                    },
                )),
                Box::new(PrepositionalPhrase::PrepositionalPhrase(
                    PrepositionalPhraseValue::new(
                        Preposition::Of,
                        Box::new(PrepositionalComplement::Object(Box::new(counted))),
                    )
                    .expect("the preposition licenses the complement"),
                )),
            )
            .expect("`of` is a licensed nominal postmodifier"),
        )),
    })
}

fn where_number_of(counted: Object) -> WhereClauseCategory {
    WhereClauseCategory::Where(WhereClause {
        clause: FiniteClause::PlainFiniteClause(
            PlainFiniteClause::new(
                Subject::VariableSubject(VariableSubject {
                    variable: Variable::X,
                }),
                Box::new(Predicate::FiniteCopular(Box::new(
                    FiniteCopularPredicate::FiniteCopularPredicate(FiniteCopularPredicateValue {
                        copula: FiniteCopula::Is,
                        complement: Box::new(PredicativeComplement::Nominal(Box::new(
                            PredicativeNominalComplement::PredicativeNominal(
                                PredicativeNominalValue {
                                    value: number_of(counted),
                                },
                            ),
                        ))),
                    }),
                ))),
            )
            .expect("X agrees with a third-person singular finite copular predicate"),
        ),
    })
}

fn that_noun(noun: Noun) -> NounPhrase {
    noun_phrase(determined_nominal(
        singular_simple_determinative(DeterminativeHeadLemma::DistalDemonstrative),
        singular_nominal_value(noun),
    ))
}

fn those_noun(noun: Noun) -> NounPhrase {
    noun_phrase(determined_nominal(
        plural_simple_determinative(DeterminativeHeadLemma::DistalDemonstrative),
        plural_nominal_value(noun),
    ))
}

fn nominal_subject(value: NounPhrase) -> Subject {
    Subject::SubjectNominal(NominalSubject { value })
}

fn nominal_object(value: NounPhrase) -> Object {
    Object::ObjectNominal(
        NominalObject::new(Box::new(value)).expect("ordinary noun phrase is licensed as an object"),
    )
}

fn subject_you() -> Subject {
    Subject::SubjectPronoun(PersonalSubject {
        word: SubjectPronoun::You,
    })
}

fn object_you() -> Object {
    Object::ObjectPronoun(PersonalObject {
        word: ObjectPronoun::You,
    })
}

fn object_it() -> Object {
    Object::ObjectPronoun(PersonalObject {
        word: ObjectPronoun::It,
    })
}

#[test]
fn ast_public_surface_excludes_retired_ability_exports_and_aliases() {
    fn collect_use_names(tree: &syn::UseTree, names: &mut Vec<String>) {
        match tree {
            syn::UseTree::Name(name) => names.push(name.ident.to_string()),
            syn::UseTree::Rename(rename) => names.push(rename.rename.to_string()),
            syn::UseTree::Path(path) => collect_use_names(&path.tree, names),
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    collect_use_names(item, names);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    let file = syn::parse_file(include_str!("../src/ast.rs")).expect("AST facade parses as Rust");
    let mut public_names = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Use(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                collect_use_names(&item.tree, &mut public_names);
            }
            syn::Item::Type(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Enum(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Union(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            _ => {}
        }
    }

    for retired in ["Event", "EventClause", "Paragraph"] {
        assert!(
            !public_names.iter().any(|name| name == retired),
            "retired public AST name remains exported or aliased: {retired}"
        );
    }
}

#[test]
fn self_reference_spelling_variants_remain_publicly_importable() {
    use deckmaste_english_v2::ast::SelfReferenceSpelling::Abbreviated;
    use deckmaste_english_v2::ast::SelfReferenceSpelling::Full;

    let _: SelfReferenceSpelling = Full;
    let _: SelfReferenceSpelling = Abbreviated;
}

#[test]
fn parse_context_rejects_only_empty_self_names() {
    assert!(ParseContext::new("", false, Onset::Consonant).is_none());
    assert!(ParseContext::new(", the Empty Prefix", false, Onset::Consonant).is_some());
}

#[test]
fn parse_context_uses_opaque_full_names_and_legendary_only_short_forms() {
    let opaque = ParseContext::new("+2 Mace", false, Onset::Consonant)
        .expect("every nonempty full name is opaque");
    assert_eq!(opaque.card_name(), "+2 Mace");
    assert_eq!(opaque.abbreviated_card_name(), "+2 Mace");

    for name in ["Fear, Fire, Foes!", "Grizzly Bears"] {
        let context = ParseContext::new(name, false, Onset::Consonant)
            .expect("nonlegendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), name);
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_none(),
            "nonlegendary {name:?} must not license an abbreviation",
        );
    }

    for (name, abbreviated) in [
        ("Aang, A Lot to Learn", "Aang"),
        ("The Balrog, Durin's Bane", "The Balrog"),
        ("King Darien XLVIII", "King Darien"),
        ("Sidar Jabari of Zhalfir", "Sidar Jabari"),
        ("Tor Wauki the Younger", "Tor Wauki"),
        ("Sliver Queen", "Sliver"),
    ] {
        let onset = if name.starts_with("Aang") { Onset::Vowel } else { Onset::Consonant };
        let context = ParseContext::new(name, true, onset).expect("legendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), abbreviated, "{name}");
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_some(),
            "legendary {name:?} licenses {abbreviated:?}",
        );
    }

    for name in ["The First Sliver", "Progenitus"] {
        let context =
            ParseContext::new(name, true, Onset::Consonant).expect("legendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), name);
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_none(),
            "legendary {name:?} has no distinct licensed abbreviation",
        );
    }
}

#[test]
fn self_reference_spelling_is_checked_against_its_context() {
    let no_comma = ParseContext::new("Context Card", false, Onset::Consonant)
        .expect("nonempty context is valid");
    assert!(SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &no_comma).is_none());
    assert_eq!(
        SourceSelfReference::new(SelfReferenceSpelling::Full, &no_comma)
            .expect("full spelling is always distinct from no value")
            .spelling(),
        SelfReferenceSpelling::Full
    );

    let comma = ParseContext::new("Zacama, Primal Calamity", true, Onset::Consonant)
        .expect("legendary abbreviation is valid");
    assert_eq!(
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &comma)
            .expect("comma-bearing context has a distinct abbreviation")
            .spelling(),
        SelfReferenceSpelling::Abbreviated
    );
}

#[test]
fn context_identity_stores_only_the_reusable_arm() {
    assert_eq!(
        std::mem::size_of::<SelfReferenceSpelling>(),
        1,
        "the generated value must not retain either context spelling"
    );
    let spelling = SelfReferenceSpelling::Abbreviated;
    assert_eq!(
        SourceSelfReference::new(spelling, &legendary_context("Zacama, Primal Calamity"))
            .expect("the stored arm is valid for one comma abbreviation")
            .spelling(),
        spelling
    );
    assert_eq!(
        SourceSelfReference::new(spelling, &legendary_context("Zoraline, Cosmos Caller"))
            .expect("the same stored arm is valid for another comma abbreviation")
            .spelling(),
        spelling
    );
}

#[test]
fn generated_invariant_products_enforce_values_and_round_trip_publicly() {
    let environment = environment();
    let parser = parser();
    let plain_context = context("Context Card");

    let event = connive_event();
    let effect = gain_life_sentence();
    let triggered = triggered(event.clone(), vec![effect.clone()]);
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if,
        body,
    }) = &triggered
    else {
        panic!("the linguistic triggered envelope is preserved")
    };
    assert!(intervening_if.as_ref().is_none());
    let AbilityBody::Sentences(body) = body else {
        panic!("the linguistic triggered envelope has a sentence body")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    assert_eq!(finite.clause.as_ref(), &Clause::Finite(Box::new(event)));
    assert_eq!(body.sentences(), std::slice::from_ref(&effect));
    let triggered_text = triggered.render(&plain_context, &environment);
    assert_eq!(parser.parse(&triggered_text, &plain_context), Ok(triggered));
    assert_eq!(
        parser
            .parse(&triggered_text, &plain_context)
            .expect("rendered Triggered ability parses")
            .render(&plain_context, &environment),
        triggered_text,
    );

    let body = gain_life_sentence();
    let clause = where_number_of(object_you());
    let with_where = WithWhere {
        body: Box::new(body.clone()),
        clause: clause.clone(),
    };
    let _: &Sentence = &with_where.body;
    let _: &WhereClauseCategory = &with_where.clause;
    assert_eq!(&with_where.clause, &clause);
    let _ = body;
    let with_where = paragraph(Sentence::WithWhere(with_where));
    let with_where_text = with_where.render(&plain_context, &environment);
    assert_eq!(
        parser.parse(&with_where_text, &plain_context),
        Ok(with_where)
    );
    assert_eq!(
        parser
            .parse(&with_where_text, &plain_context)
            .expect("rendered WithWhere sentence parses")
            .render(&plain_context, &environment),
        with_where_text,
    );

    let count = paragraph(declarative(
        nominal_subject(creatures_you_control_with_power_at_most_two()),
        gain_life(variable_x()),
    ));
    let count_text = count.render(&plain_context, &environment);
    assert_eq!(parser.parse(&count_text, &plain_context), Ok(count));
    assert_eq!(
        parser
            .parse(&count_text, &plain_context)
            .expect("rendered qualified nominal sentence parses")
            .render(&plain_context, &environment),
        count_text,
    );

    assert!(
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &plain_context).is_none(),
        "an unavailable abbreviation is rejected",
    );
    let abbreviated_context = legendary_context("Zacama, Primal Calamity");
    let self_reference =
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &abbreviated_context)
            .expect("a distinct abbreviated spelling is valid");
    let _: SelfReferenceSpelling = self_reference.spelling();
    assert_eq!(
        self_reference.spelling(),
        SelfReferenceSpelling::Abbreviated
    );
    let self_reference = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference,
        ))),
        gain_life(variable_x()),
    ));
    let self_reference_text = self_reference.render(&abbreviated_context, &environment);
    assert_eq!(
        parser.parse(&self_reference_text, &abbreviated_context),
        Ok(self_reference)
    );
    assert_eq!(
        parser
            .parse(&self_reference_text, &abbreviated_context)
            .expect("rendered source-self-reference sentence parses")
            .render(&abbreviated_context, &environment),
        self_reference_text,
    );
}

#[test]
fn typed_where_staging_rejects_a_finite_subordinate_clause_in_the_chart() {
    let parser = parser();
    let context = context("Context Card");
    let denied = "You gain X life, a player connives.";

    let ParseError::Failure { span, expectations } = parser
        .parse_sentence(denied, &context)
        .expect_err("a finite subordinate is not a where-complement")
    else {
        panic!("the rejected subordinate must remain a chart failure")
    };
    assert_eq!(span, TextSpan { start: 34, end: 35 });
    assert!(!expectations.is_empty());
    assert!(expectations.contains(&Expectation::Terminal(TerminalClass::SingularDemonstrative)));

    let trace = parser.trace_sentence(denied, &context, TraceLimits::new(usize::MAX));
    assert!(matches!(
        trace.outcome(),
        BoundedParseOutcome::ParseFailure(_)
    ));
    assert!(trace.accepted_roots().items().is_empty());
    assert!(trace.materialized_candidates().items().is_empty());
    assert!(trace.build_rejection().is_none());
    assert_eq!(
        trace
            .checked_completion_rejections()
            .items()
            .iter()
            .map(|rejection| (rejection.rule_name_v1(), rejection.start(), rejection.end(),))
            .collect::<Vec<_>>(),
        [
            ("DeterminativePluralSimpleDeterminative", 17, 18),
            ("FixedDurationPhraseFixedDurationPhrase", 10, 15),
            ("FixedDurationPhraseFixedDurationPhrase", 10, 15),
            ("NominalBarePluralNominal", 18, 25),
            ("NominalBarePluralNominal", 18, 25),
            ("UnqualifiedReferenceDeterminedNominalDetPresent", 8, 15),
            ("UnqualifiedReferenceDeterminedNominalDetPresent", 8, 15),
        ],
        "the rejected subordinate exposes the exact generic nominal guard inventory"
    );

    let allowed =
        "You gain X life, where X is the number of creatures you control with power 2 or less.";
    assert!(parser.parse_sentence(allowed, &context).is_ok());
    assert!(
        parser
            .trace_sentence(allowed, &context, TraceLimits::new(8))
            .build_rejection()
            .is_none()
    );

    let malformed = "You gain X life, a player.";
    let malformed_error = parser
        .parse_sentence(malformed, &context)
        .expect_err("incomplete finite-clause syntax remains a chart failure");
    assert!(matches!(malformed_error, ParseError::Failure { .. }));
    assert!(
        parser
            .trace_sentence(malformed, &context, TraceLimits::new(8))
            .build_rejection()
            .is_none()
    );
}

#[test]
fn demonstrative_references_select_the_unified_determined_nominal_construction() {
    let parser = parser();
    let environment = environment();
    let context = context("Context Card");

    for (text, reference, selected_rule) in [
        (
            "That creature deals 3 damage to it.",
            that_noun(creature()),
            "UnqualifiedReferenceDeterminedNominalDetPresent",
        ),
        (
            "Those creatures deal 3 damage to it.",
            those_noun(creatures()),
            "UnqualifiedReferenceDeterminedNominalDetPresent",
        ),
    ] {
        let expected = declarative(
            nominal_subject(reference),
            deal_damage(
                Amount::Number(NumberAmount {
                    number: ScalarNumber { magnitude: 3 },
                }),
                object_it(),
            ),
        );

        let trace = parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));
        assert_eq!(trace.clone().into_parse_result(), Ok(expected.clone()));
        assert_eq!(expected.render(&context, &environment), text);
        assert_eq!(trace.materialized_candidates().total(), 1);

        let forest_rules = trace
            .forest()
            .items()
            .iter()
            .map(deckmaste_english_v2::parser::ForestNode::rule_name_v1)
            .collect::<BTreeSet<_>>();
        assert!(
            forest_rules.contains(selected_rule),
            "{text}: {forest_rules:?}"
        );
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the generated privacy and accessor inventory is deliberately literal"
)]
#[test]
fn parse_error_is_a_standard_error_and_converts_to_anyhow() {
    fn require_standard_error(error: &(impl std::error::Error + ?Sized)) {
        let _ = error;
    }

    let error = ParseError::Failure {
        span: TextSpan { start: 11, end: 11 },
        expectations: BTreeSet::from([Expectation::Literal("life")]),
    };
    require_standard_error(&error);

    let _: anyhow::Error = error.into();
}

fn creature() -> Noun {
    let environment = environment();
    Noun::Declaration(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "creature"),
        )
        .expect("Creature is a normalized noun declaration"),
    )
}

fn creatures() -> Noun {
    let environment = environment();
    Noun::Declaration(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "creature"),
        )
        .expect("Creature has a normalized plural noun reading"),
    )
}

fn target_creature() -> Object {
    nominal_object(target_noun(creature()))
}

fn variable_x() -> Amount {
    Amount::Variable(VariableAmount {
        variable: Variable::X,
    })
}

fn atomic(predicate: VerbPhrase) -> Predicate {
    Predicate::Atomic(Box::new(predicate))
}

fn finite_clause(subject: Subject, predicate: VerbPhrase) -> FiniteClause {
    FiniteClause::PlainFiniteClause(
        PlainFiniteClause::new(subject, Box::new(atomic(predicate)))
            .expect("the helper supplies matching subject-predicate concord_class"),
    )
}

fn declarative(subject: Subject, predicate: VerbPhrase) -> Sentence {
    Sentence::Declarative(Declarative {
        clause: Box::new(Clause::Finite(Box::new(finite_clause(subject, predicate)))),
    })
}

fn sentences(values: Vec<Sentence>) -> AbilityBody {
    AbilityBody::Sentences(
        Sentences::new(Box::new(values)).expect("one or more sentences construct a body"),
    )
}

fn paragraph(sentence: Sentence) -> Ability {
    Ability::Plain(Plain {
        body: sentences(vec![sentence]),
    })
}

fn core_transitive_head(identity: CoreVerbIdentity) -> DeclarationTransitiveVerb {
    DeclarationTransitiveVerb::new(&environment(), VerbInventoryRef::Core(identity))
        .expect("core inventory row licenses the transitive frame")
}

fn quantified_mass_object(amount: Amount, noun: CommonNoun) -> Object {
    let determiner = Determiner::Headed(Determinative::MassQuantityDeterminer(
        MassQuantityDeterminer { amount },
    ));
    let nominal = Nominal::MassNominal(MassNominal {
        noun: MassNoun::MassNoun(
            MassNounValue::new(Noun::Lexeme(noun))
                .expect("the helper is called only with declared mass nouns"),
        ),
    });
    let reference = UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(determiner, nominal)
            .expect("a quantity determiner licenses a mass noun"),
    );
    nominal_object(noun_phrase(reference))
}

fn gain_life(amount: Amount) -> VerbPhrase {
    let head = core_transitive_head(CoreVerbIdentity::Gain);
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::TransitiveLexicalVerbPhrase(Box::new(
            TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
                head,
                object: quantified_mass_object(amount, CommonNoun::Life),
            }),
        ))),
    })
}

fn deal_damage(amount: Amount, recipient: Object) -> VerbPhrase {
    let head = DeclarationToObjectVerb::new(
        &environment(),
        VerbInventoryRef::Core(CoreVerbIdentity::Deal),
    )
    .expect("Deal licenses the object-to-object frame");
    VerbPhrase::DeclaredToObjectPredicate(DeclaredToObjectPredicate {
        head,
        object: quantified_mass_object(amount, CommonNoun::Damage),
        complement: recipient,
    })
}

fn destroy(object: Object) -> VerbPhrase {
    let environment = environment();
    let head = DeclarationTransitiveVerb::new(
        &environment,
        VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "destroy",
        )),
    )
    .expect("the builtin grammar declares transitive Destroy");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::TransitiveLexicalVerbPhrase(Box::new(
            TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate { head, object }),
        ))),
    })
}

fn connive() -> VerbPhrase {
    let environment = environment();
    let head = DeclarationIntransitiveVerb::new(
        &environment,
        VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "connive",
        )),
    )
    .expect("the builtin grammar declares intransitive Connive");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
            IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate { head }),
        )),
    })
}

fn destroy_target_creature() -> Ability {
    paragraph(Sentence::Imperative(
        Imperative::new(Box::new(atomic(destroy(target_creature()))))
            .expect("destroy is a valid bare imperative predicate"),
    ))
}

fn connive_event() -> FiniteClause {
    finite_clause(
        nominal_subject(indefinite(Noun::Lexeme(CommonNoun::Player))),
        connive(),
    )
}

fn triggered(trigger_clause: FiniteClause, consequences: Vec<Sentence>) -> Ability {
    Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(Finite {
            marker: TriggerMarker::Whenever,
            clause: Box::new(Clause::Finite(Box::new(trigger_clause))),
        }),
        intervening_if: Box::new(None),
        body: sentences(consequences),
    })
}

fn triggered_damage() -> Ability {
    triggered(
        connive_event(),
        vec![declarative(
            nominal_subject(that_noun(creature())),
            deal_damage(variable_x(), object_it()),
        )],
    )
}

fn gain_life_sentence() -> Sentence {
    declarative(subject_you(), gain_life(variable_x()))
}

fn gain_life_with_where() -> Ability {
    paragraph(Sentence::WithWhere(WithWhere {
        body: Box::new(gain_life_sentence()),
        clause: where_number_of(nominal_object(
            creatures_you_control_with_power_at_most_two(),
        )),
    }))
}

fn zacama_deals_damage() -> Ability {
    paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            ),
        ))),
        deal_damage(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    ))
}

fn triggered_gain_life() -> Ability {
    triggered(connive_event(), vec![gain_life_sentence()])
}

#[test]
fn generic_root_api_preserves_types_and_sentence_root_metadata() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";

    let ability: ParseAnalysis<Ability> = parser.analyze(text, &context);
    let sentence: ParseAnalysis<Sentence> = parser.analyze_sentence(text, &context);
    let trace: ParserTrace<Sentence> =
        parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));

    let Ability::Plain(Plain { body }) = destroy_target_creature() else {
        panic!("the focused fixture is an ordinary paragraph");
    };
    let AbilityBody::Sentences(expected) = body else {
        panic!("the focused fixture has a sentence body");
    };
    let expected_sentence = expected.sentences()[0].clone();
    assert_eq!(
        ability.into_parse_result(),
        Ok(Ability::Plain(Plain {
            body: AbilityBody::Sentences(expected.clone()),
        }))
    );
    assert_eq!(sentence.into_parse_result(), Ok(expected_sentence.clone()));
    assert_eq!(trace.root_name(), "Sentence");
    assert_eq!(trace.clone().into_parse_result(), Ok(expected_sentence));
    assert!(
        trace.scanner_matches().items().iter().any(|scanned| {
            scanned.terminal_name_v1() == "Literal(\".\")"
                && scanned.start() == text.len() - 1
                && scanned.end() == text.len()
        }),
        "the focused Sentence root owns its declared period",
    );
    assert!(
        trace
            .scanner_matches()
            .items()
            .iter()
            .all(|scanned| scanned.terminal_name_v1() != "EndOfInput"),
        "Sentence uses its declared eoi = false metadata",
    );
    let ownership = trace.ownership().expect("Sentence ownership is selected");
    assert!(ownership.covered());
    assert!(trace.selected_lexical_claims().items().iter().any(|claim| {
        claim.span()
            == TextSpan {
                start: text.len() - 1,
                end: text.len(),
            }
            && claim.stable_owner_id() == "root:Sentence/punctuation"
    }));
}

#[test]
fn generic_root_adapter_is_only_applied_at_the_outer_recursive_sentence_boundary() {
    let parser = parser();
    let context = context("Context Card");
    let text =
        "You gain X life, where X is the number of creatures you control with power 2 or less.";
    let Ability::Plain(Plain { body }) = gain_life_with_where() else {
        panic!("the recursive Sentence fixture is an ordinary paragraph");
    };
    let AbilityBody::Sentences(expected) = body else {
        panic!("the recursive Sentence fixture has a sentence body");
    };
    let expected = expected.sentences()[0].clone();

    let trace = parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));

    assert_eq!(trace.clone().into_parse_result(), Ok(expected));
    assert_eq!(
        trace
            .scanner_matches()
            .items()
            .iter()
            .filter(|scanned| scanned.terminal_name_v1() == "Literal(\".\")")
            .count(),
        1,
        "only the true Sentence root boundary scans its declared period",
    );
}

#[test]
fn parses_and_round_trips_the_five_slice_abilities() {
    let parser = parser();
    for (text, card_name, expected) in [
        (
            "Destroy target creature.",
            "Context Card",
            destroy_target_creature(),
        ),
        (
            "Whenever a player connives, that creature deals X damage to it.",
            "Context Card",
            triggered_damage(),
        ),
        (
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            "Context Card",
            gain_life_with_where(),
        ),
        (
            "Zacama deals 3 damage to target creature.",
            "Zacama, Primal Calamity",
            zacama_deals_damage(),
        ),
        (
            "Whenever a player connives, you gain X life.",
            "Context Card",
            triggered_gain_life(),
        ),
    ] {
        let context = if card_name == "Zacama, Primal Calamity" {
            legendary_context(card_name)
        } else {
            context(card_name)
        };
        assert_eq!(parser.parse(text, &context), Ok(expected.clone()));
        assert_eq!(
            parser.parse(text, &context),
            parser.analyze(text, &context).into_parse_result()
        );
        for limit in [0, 1, usize::MAX] {
            assert_eq!(
                parser.parse(text, &context),
                parser
                    .trace(text, &context, TraceLimits::new(limit))
                    .into_parse_result(),
                "{text:?} at cap {limit}",
            );
        }
        assert_eq!(expected.render(&context, &environment()), text);

        let rendered = expected.render(&context, &environment());
        assert_eq!(parser.parse(&rendered, &context), Ok(expected));
    }
}

#[test]
fn parser_analysis_repeats_exactly_and_preserves_selected_rendered_bytes() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";
    let first = parser.analyze(text, &context);
    let second = parser.analyze(text, &context);

    assert_eq!(first, second);
    assert_eq!(
        first.selected().unwrap().render(&context, &environment()),
        text
    );
    assert_eq!(
        first.decision().unwrap().candidates()[0].construction_path(),
        [
            "AbilityPlain".to_owned(),
            "AbilityBodySentences".to_owned(),
            "SentenceImperative".to_owned(),
            "VerbPhraseBaseVerbPhrase".to_owned(),
            "TransitiveLexicalVerbPhraseTransitivePredicate".to_owned(),
            "ObjectObjectNominal".to_owned(),
            "NounPhraseQualifiedNounPhrase".to_owned(),
            "PostmodifiedReferenceUnqualifiedPostmodifiedReference".to_owned(),
            "UnqualifiedReferenceDeterminedNominal".to_owned(),
            "DeterminativeTargetingMarkerDeterminative".to_owned(),
            "NominalBareSingularNominal".to_owned(),
            "HeadNounSingularHead".to_owned(),
        ]
    );
    assert_eq!(parser.parse(text, &context), first.into_parse_result(),);
}

#[test]
fn parser_analysis_retains_complete_lexical_ownership() {
    let parser = parser();
    let context = context("Context Card");
    let analysis = parser.analyze("Destroy target creature.", &context);
    let ownership = analysis.ownership().expect("selected parse owns terminals");
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert_eq!(ownership.parsed_claims().len(), 4);
    assert_eq!(ownership.rendered_claims().len(), 4);
    assert_eq!(ownership.rendered_text(), "Destroy target creature.");
    let summary = ownership.summary();
    assert_eq!((summary.claims(), summary.claimed_bytes()), (4, 24));
    assert_eq!(
        (summary.form_literal_claims(), summary.form_literal_bytes()),
        (1, 1)
    );
    assert_eq!((summary.lexeme_claims(), summary.lexeme_bytes()), (2, 16));
    assert_eq!((summary.vocab_claims(), summary.vocab_bytes()), (1, 7));
    assert_eq!((summary.codec_claims(), summary.codec_bytes()), (0, 0));
    assert_eq!(summary.identity_claims(), 0);
    assert_eq!(summary.gap_spans(), 0);
    assert_eq!(summary.overlap_spans(), 0);
    assert_eq!(summary.synthetic_claims(), 0);
    assert_eq!(summary.provenance_plan_mismatches(), 0);
}

#[test]
fn generated_morphology_closed_owner_ids_match_scan_and_render_claims() {
    let parser = parser();
    let context = context("Context Card");
    for (text, expected) in [
        ("Deal X damage to target creature.", "core-verb:Deal"),
        ("It deals X damage to target creature.", "core-verb:Deal"),
        (
            "Whenever a player connives, you gain X life.",
            "lexeme:CommonNoun/Player/singular",
        ),
        (
            "Those players deal X damage to it.",
            "lexeme:CommonNoun/Player/plural",
        ),
        (
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            "vocab:FiniteCopula/Is",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("no ownership for {text:?}"));
        assert!(
            ownership
                .parsed_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == expected),
            "scanner claims lack {expected:?} for {text:?}: {:?}",
            ownership.parsed_claims(),
        );
        assert!(
            ownership
                .rendered_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == expected),
            "render claims lack {expected:?} for {text:?}: {:?}",
            ownership.rendered_claims(),
        );
    }
}

#[test]
fn explicit_named_card_identity_scans_exact_longest_renders_and_owns() {
    let parser = parser();
    let environment = environment();
    let context = context("Context Card");
    let text = "Destroy a card named Alpha Beta.";
    let parsed = parser
        .parse(text, &context)
        .expect("explicit card name parses");
    let Ability::Plain(Plain { body }) = &parsed else {
        panic!("named-card sentence is a paragraph: {parsed:?}");
    };
    let AbilityBody::Sentences(paragraph) = body else {
        panic!("named-card sentence has a sentence body: {parsed:?}");
    };
    let [Sentence::Imperative(imperative)] = paragraph.sentences() else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) = predicate.as_ref() else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let LexicalVerbPhrase::TransitiveLexicalVerbPhrase(transitive_lexical_verb_phrase) =
        frame.as_ref()
    else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
        head: _,
        object: Object::ObjectNominal(object),
    }) = transitive_lexical_verb_phrase.as_ref()
    else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let UnqualifiedReference::NamedCardReference(NamedCardReference { name, .. }) =
        unqualified_reference(object.value())
    else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    assert_eq!(name.provider(), CatalogProvider::CardNames);
    assert_eq!(name.canonical_identity(), "alpha-beta");
    assert_eq!(parsed.render(&context, &environment), text);

    let ownership = parser
        .analyze(text, &context)
        .ownership()
        .expect("named identity owns its source bytes")
        .clone();
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert!(ownership.parsed_claims().iter().any(|claim| {
        claim.kind() == LexicalProvenanceKind::Identity
            && claim.stable_owner_id() == "identity:CardNames/alpha-beta"
            && &text[claim.span().start..claim.span().end] == " Alpha Beta"
    }));
    assert!(ownership.rendered_claims().iter().any(|claim| {
        claim.kind() == LexicalProvenanceKind::Identity
            && claim.stable_owner_id() == "identity:CardNames/alpha-beta"
    }));

    assert!(
        parser
            .parse("Destroy a card named alpha beta.", &context)
            .is_err(),
        "catalog identity scan preserves exact case"
    );
    let punctuation = "Destroy a card named Urza's Saga.";
    let punctuation_ast = parser
        .parse(punctuation, &context)
        .expect("punctuation-bearing identity parses");
    assert_eq!(punctuation_ast.render(&context, &environment), punctuation);
}

#[test]
fn bare_own_card_name_remains_unique_source_self_reference() {
    let parser = parser();
    let context = context("Seven Dwarves");
    let text = "Seven Dwarves gains 2 life.";
    let analysis = parser.analyze(text, &context);
    let selected = analysis.selected().expect("source self-reference selects");
    let Ability::Plain(Plain { body }) = selected else {
        panic!("self-reference sentence is a paragraph: {selected:?}");
    };
    let AbilityBody::Sentences(paragraph) = body else {
        panic!("self-reference sentence has a sentence body: {selected:?}");
    };
    let [Sentence::Declarative(declarative)] = paragraph.sentences() else {
        panic!("bare own card name remains source self-reference: {selected:?}");
    };
    let Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("bare own card name remains finite: {selected:?}");
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("bare own card name remains a plain finite clause: {selected:?}");
    };
    let Subject::SubjectNominal(NominalSubject { value }) = clause.subject() else {
        panic!("bare own card name remains source self-reference: {selected:?}");
    };
    assert!(matches!(
        unqualified_reference(value),
        UnqualifiedReference::SelfReference(_)
    ));
    assert_eq!(
        analysis
            .decision()
            .expect("selected analysis retains its decision")
            .candidates()
            .len(),
        1,
        "bare catalog identity contributes no rival noun-phrase construction"
    );
}

#[test]
fn parser_analysis_ownership_covers_every_kind_unicode_and_multitoken_identity() {
    let parser = parser();
    let cases = [
        (
            "Whenever a player connives, that creature deals X damage to it.",
            "Context Card",
        ),
        (
            "Context Card deals 3 damage to target creature.",
            "Context Card",
        ),
        (
            concat!("E", "\u{301}", "lan deals 3 damage to target creature."),
            concat!("E", "\u{301}", "lan"),
        ),
    ];
    let mut kinds = BTreeSet::new();
    for (text, card_name) in cases {
        let context = context(card_name);
        let analysis = parser.analyze(text, &context);
        let ownership = analysis.ownership().expect("selected parse owns terminals");
        assert!(
            ownership.failures().is_empty(),
            "{text:?}: {:?}",
            ownership.failures()
        );
        assert!(ownership.summary().covered(), "{text:?}");
        assert_eq!(ownership.rendered_text(), text);
        let mut cursor = 0;
        for claim in ownership.parsed_claims() {
            assert_eq!(claim.span().start, cursor, "{text:?}");
            assert!(claim.span().end > claim.span().start, "{text:?}");
            assert!(text.is_char_boundary(claim.span().start), "{text:?}");
            assert!(text.is_char_boundary(claim.span().end), "{text:?}");
            kinds.insert(claim.kind());
            cursor = claim.span().end;
        }
        assert_eq!(cursor, text.len(), "{text:?}");
    }
    assert_eq!(
        kinds,
        BTreeSet::from([
            LexicalProvenanceKind::FormLiteral,
            LexicalProvenanceKind::Vocab,
            LexicalProvenanceKind::Lexeme,
            LexicalProvenanceKind::Codec,
            LexicalProvenanceKind::Identity,
        ])
    );
}

#[test]
fn parser_analysis_projects_representative_parse_failures_without_changing_them() {
    let parser = parser();
    for text in [
        "Destroy target creature",
        "You gains X life.",
        "Destroy target flying.",
    ] {
        let context = context("Context Card");
        assert_eq!(
            parser.parse(text, &context),
            parser.analyze(text, &context).into_parse_result(),
            "{text:?}",
        );
        assert!(parser.analyze(text, &context).ownership().is_none());
        for limit in [0, 1, usize::MAX] {
            assert_eq!(
                parser.parse(text, &context),
                parser
                    .trace(text, &context, TraceLimits::new(limit))
                    .into_parse_result(),
                "{text:?} at cap {limit}",
            );
        }
    }
}

fn assert_bounded<T>(bounded: &deckmaste_english_v2::parser::Bounded<T>, limit: usize) {
    assert_eq!(bounded.total(), bounded.shown() + bounded.omitted());
    assert_eq!(bounded.shown(), bounded.items().len());
    assert!(bounded.shown() <= limit);
}

#[test]
fn parser_trace_selected_projection_is_exact_bounded_repeatable_and_private_result_preserving() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";

    for limit in [0, 1, usize::MAX] {
        let limits = TraceLimits::new(limit);
        let trace = parser.trace(text, &context, limits);
        let repeated = parser.trace(text, &context, limits);
        assert_eq!(trace, repeated);
        assert_bounded(trace.scanner_matches(), limit);
        assert_bounded(trace.chart(), limit);
        assert_bounded(trace.forest(), limit);
        assert_bounded(trace.accepted_roots(), limit);
        assert_bounded(trace.checked_completion_rejections(), limit);
        assert_bounded(trace.materialized_candidates(), limit);
        assert_bounded(trace.materialization_cycles(), limit);

        let BoundedParseOutcome::Selected(selected) = trace.outcome() else {
            panic!("rendered slice must select");
        };
        assert_eq!(selected.rendered(), text);
        let selection = selected.selection();
        assert_eq!(
            selection.resolution(),
            deckmaste_english_v2::parser::SelectionResolution::Unique
        );
        assert_bounded(selection.candidates(), limit);
        assert_bounded(selection.comparisons(), limit);
        assert_bounded(selection.survivors(), limit);
        assert_bounded(selection.exception_uses(), limit);
        assert_bounded(selection.unselected_candidates(), limit);
        let selected_candidate = selection.selected().expect("selected metadata");
        assert_eq!(selected_candidate.ordinal(), 0);
        assert_bounded(selected_candidate.construction_path(), limit);
        assert_bounded(selected_candidate.specificity(), limit);

        if limit > 0 {
            let candidate = &trace.materialized_candidates().items()[0];
            assert_eq!(candidate.construction_path().total(), 12);
            assert_eq!(candidate.construction_path().shown(), usize::min(limit, 12));
            assert_eq!(candidate.specificity().total(), 14);
            assert_eq!(candidate.specificity().shown(), usize::min(limit, 14));
        }

        if limit == usize::MAX {
            let candidate = &trace.materialized_candidates().items()[0];
            let parsed = parser.parse(text, &context).expect("selected ability");
            let analysis = parser.analyze(text, &context);
            let complete = analysis.decision().expect("complete selection decision");
            assert_eq!(candidate.ordinal(), 0);
            assert_eq!(candidate.rendered(), text);
            assert_eq!(candidate.ast_debug_v1(), format!("{parsed:?}"));
            assert_eq!(
                candidate.construction_path().items(),
                [
                    "AbilityPlain",
                    "AbilityBodySentences",
                    "SentenceImperative",
                    "VerbPhraseBaseVerbPhrase",
                    "TransitiveLexicalVerbPhraseTransitivePredicate",
                    "ObjectObjectNominal",
                    "NounPhraseQualifiedNounPhrase",
                    "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                    "UnqualifiedReferenceDeterminedNominal",
                    "DeterminativeTargetingMarkerDeterminative",
                    "NominalBareSingularNominal",
                    "HeadNounSingularHead",
                ]
            );
            assert_eq!(candidate.specificity().total(), 14);
            assert!(selection.unselected_candidates().items().is_empty());
            assert_eq!(selection.resolution(), complete.resolution());
            assert_eq!(selection.survivors().items(), complete.survivors());
            assert_eq!(
                selection.exception_uses().items(),
                complete.exception_uses()
            );
            assert_eq!(
                selection.candidates().items()[0]
                    .construction_path()
                    .items(),
                complete.candidates()[0].construction_path()
            );
            assert_eq!(
                selection.candidates().items()[0].specificity().items(),
                complete.candidates()[0].specificity()
            );
        }

        assert_eq!(
            parser.parse(text, &context),
            trace.into_parse_result(),
            "public cap {limit} must not truncate the private legacy result",
        );
    }
}

#[test]
fn parser_trace_parse_failure_bounds_expectations_without_truncating_private_error() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature";
    let complete = parser.parse(text, &context);
    let Err(ParseError::Failure {
        expectations: complete_expectations,
        ..
    }) = &complete
    else {
        panic!("missing period is a complete chart failure")
    };
    let expected_total = complete_expectations.len();

    for (limit, expected_shown) in [(0, 0), (1, 1), (8, 8)] {
        let trace = parser.trace(text, &context, TraceLimits::new(limit));
        let BoundedParseOutcome::ParseFailure(failure) = trace.outcome() else {
            panic!("missing period is an ordinary parse failure");
        };
        assert_eq!(
            failure.span(),
            TextSpan {
                start: text.len(),
                end: text.len()
            }
        );
        assert_eq!(failure.expectations().total(), expected_total);
        assert_eq!(failure.expectations().shown(), expected_shown);
        assert_eq!(
            failure.expectations().omitted(),
            expected_total - expected_shown
        );
        if limit > 0 {
            assert_eq!(
                failure.expectations().items()[0],
                deckmaste_english_v2::parser::ExpectationInfo::Nonterminal(
                    NonterminalCategory::DurationPhrase,
                )
            );
        }
        assert_eq!(complete, trace.into_parse_result());
    }
}

#[test]
fn parser_trace_public_loser_reason_spellings_are_stable() {
    assert_eq!(SelectionLoserReason::LessSpecific.as_str(), "less_specific");
    assert_eq!(
        SelectionLoserReason::ExceptionLoser.as_str(),
        "exception_loser"
    );
    assert_eq!(
        SelectionLoserReason::UnresolvedSurvivor.as_str(),
        "unresolved_survivor"
    );
}

#[test]
fn no_comma_self_reference_parses_once_as_full_and_round_trips() {
    let text = "Context Card deals 3 damage to target creature.";
    let context = context("Context Card");
    let expected = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(SelfReferenceSpelling::Full, "Context Card"),
        ))),
        deal_damage(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    ));

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn self_reference_identity_preserves_its_inherent_case() {
    let text = "eBay deals 3 damage to target creature.";
    let context = context("eBay");
    let expected = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(SelfReferenceSpelling::Full, "eBay"),
        ))),
        deal_damage(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    ));

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn disallowed_declaration_kind_is_a_parse_failure() {
    let text = "Destroy target morbid.";
    let ParseError::Failure { span, expectations } = parser()
        .parse(text, &context("Context Card"))
        .expect_err("an AbilityWord declaration cannot serve as a noun")
    else {
        panic!("the disallowed declaration kind must remain a chart failure")
    };
    assert_eq!(span, TextSpan { start: 15, end: 21 });
    assert!(!expectations.is_empty());
    assert!(expectations.iter().any(|expectation| {
        matches!(expectation, Expectation::Terminal(class) if class.to_string().contains("noun"))
    }));
}

#[test]
fn missing_period_reports_chart_derived_literal_expectation() {
    let text = "Destroy target creature";
    let ParseError::Failure { span, expectations } = parser()
        .parse(text, &context("Context Card"))
        .expect_err("a complete sentence requires its period")
    else {
        panic!("the missing terminator must remain a chart failure")
    };
    assert_eq!(
        span,
        TextSpan {
            start: text.len(),
            end: text.len(),
        }
    );
    assert!(expectations.contains(&Expectation::Literal(".")));
}

#[test]
fn concord_class_mismatch_reports_a_nonempty_chart_failure() {
    let text = "You gains X life.";
    let Err(ParseError::Failure { span, expectations }) =
        parser().parse(text, &context("Context Card"))
    else {
        panic!("subject-predicate concord_class mismatch must remain a chart failure")
    };
    assert_eq!(span, TextSpan { start: 16, end: 17 });
    assert!(!expectations.is_empty());
    assert!(expectations.contains(&Expectation::Literal(" and ")));
    assert!(expectations.contains(&Expectation::Nonterminal(
        NonterminalCategory::PrepositionalPhrase
    )));
}

#[test]
fn object_gap_relative_subject_is_not_restricted_by_game_role() {
    let text =
        "You gain X life, where X is the number of creatures it controls with power 2 or less.";
    let parser = parser();
    let context = context("Context Card");
    let parsed = parser
        .parse(text, &context)
        .expect("ordinary third-person object-gap relative is grammatical");
    assert_eq!(parsed.render(&context, parser.environment()), text);
    let ownership = parser
        .analyze(text, &context)
        .ownership()
        .expect("selected relative clause owns its bytes")
        .clone();
    assert!(ownership.summary().covered());
    assert!(ownership.failures().is_empty());
}

#[test]
fn lexical_matches_reject_prefixes_of_longer_lexemes() {
    for (text, card_name, expected_span) in [
        (
            "Destroyx target creature.",
            "Context Card",
            TextSpan { start: 0, end: 8 },
        ),
        (
            "Zacama deals 3x damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 14, end: 15 },
        ),
        (
            "Destroy target creaturex.",
            "Context Card",
            TextSpan { start: 23, end: 24 },
        ),
        (
            "Zacamaé deals 3 damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 6, end: 8 },
        ),
    ] {
        let Err(ParseError::Failure { span, expectations }) = parser().parse(
            text,
            &if card_name == "Zacama, Primal Calamity" {
                legendary_context(card_name)
            } else {
                context(card_name)
            },
        ) else {
            panic!("lexical prefix must fail for {text:?}");
        };

        assert_eq!(span, expected_span, "{text:?}");
        assert!(!expectations.is_empty(), "{text:?}");
    }
}

#[test]
fn decimal_punctuation_cannot_split_a_signed_number_in_a_complete_document() {
    let text = "Gain 1.0 life.";
    let Err(ParseError::Failure { span, expectations }) =
        parser().parse(text, &context("Context Card"))
    else {
        panic!("a decimal fraction is outside the signed-decimal grammar");
    };

    assert_eq!(span, TextSpan { start: 7, end: 8 });
    assert!(!expectations.is_empty());
}

#[test]
fn doubled_period_reports_the_first_trailing_byte() {
    let text = "Destroy target creature..";
    let trailing = text.len() - 1;

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan {
                start: trailing,
                end: trailing + 1,
            },
            expectations: BTreeSet::from([
                Expectation::Terminal(TerminalClass::EndOfInput),
                Expectation::Literal("\n"),
                Expectation::Literal(" "),
                Expectation::Literal(" Then "),
            ]),
        })
    );
}

#[test]
fn wrong_case_and_wrong_whitespace_are_parse_failures() {
    for text in [
        "destroy target creature.",
        "Destroy Target creature.",
        "Destroy  target creature.",
        "Destroytarget creature.",
        "Whenever a player connives, you Gain X life.",
    ] {
        assert!(parser().parse(text, &context("Context Card")).is_err());
    }
}
