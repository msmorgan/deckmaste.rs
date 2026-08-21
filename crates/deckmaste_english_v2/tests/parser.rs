use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
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
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::SurfaceFeature;

fn environment() -> ParserEnvironment {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_declarations(declarations)
        .expect("builtin-v2 declaration environment freezes")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("required declarations are present")
}

fn context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name).expect("test card name is a valid parse context")
}

fn self_reference(spelling: SelfReferenceSpelling, card_name: &str) -> SelfReferenceNp {
    let context = context(card_name);
    SelfReferenceNp::new(spelling, &context).expect("test spelling is valid for its context")
}

#[test]
fn parse_context_rejects_empty_self_names_and_abbreviations() {
    assert!(ParseContext::new("").is_none());
    assert!(ParseContext::new(", the Empty Prefix").is_none());
}

#[test]
fn self_reference_spelling_is_checked_against_its_context() {
    let no_comma = ParseContext::new("Context Card").expect("nonempty context is valid");
    assert!(SelfReferenceNp::new(SelfReferenceSpelling::Abbreviated, &no_comma).is_none());
    assert_eq!(
        SelfReferenceNp::new(SelfReferenceSpelling::Full, &no_comma)
            .expect("full spelling is always distinct from no value")
            .spelling(),
        SelfReferenceSpelling::Full
    );

    let comma =
        ParseContext::new("Zacama, Primal Calamity").expect("nonempty abbreviation is valid");
    assert_eq!(
        SelfReferenceNp::new(SelfReferenceSpelling::Abbreviated, &comma)
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
        SelfReferenceNp::new(spelling, &context("Zacama, Primal Calamity"))
            .expect("the stored arm is valid for one comma abbreviation")
            .spelling(),
        spelling
    );
    assert_eq!(
        SelfReferenceNp::new(spelling, &context("Zoraline, Cosmos Caller"))
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
    let triggered = Triggered::new(TriggerWord::Whenever, event.clone(), effect.clone())
        .expect("an Event clause is valid for Triggered");
    let _: &TriggerWord = &triggered.trigger;
    let _: &Sentence = &triggered.effect;
    let _: &Clause = triggered.event();
    assert_eq!(triggered.event(), &event);
    assert!(
        Triggered::new(
            TriggerWord::Whenever,
            Clause::Where(WhereClause {
                variable: Variable::X,
                value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
            }),
            effect,
        )
        .is_none(),
        "a Where clause is not a valid Triggered event",
    );
    let triggered = Ability::Triggered(triggered);
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
    let clause = Clause::Where(WhereClause {
        variable: Variable::X,
        value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
    });
    let with_where = WithWhere::new(Box::new(body.clone()), clause.clone())
        .expect("a Where clause is valid for WithWhere");
    let _: &Sentence = &with_where.body;
    let _: &Clause = with_where.clause();
    assert_eq!(with_where.clause(), &clause);
    assert!(
        WithWhere::new(Box::new(body), connive_event()).is_none(),
        "an Event clause is not a valid WithWhere clause",
    );
    let with_where = Ability::Spell(Spell {
        effect: Sentence::WithWhere(with_where),
    });
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

    let threshold = SignedNumber {
        sign: Sign::Positive,
        magnitude: 2,
    };
    let count = CountNp::new(creatures(), Pronoun::You, threshold.clone())
        .expect("You is the valid CountNp controller");
    let _: &Noun = &count.head;
    let _: &SignedNumber = &count.threshold;
    let _: Pronoun = count.controller();
    assert_eq!(count.controller(), Pronoun::You);
    assert!(
        CountNp::new(creatures(), Pronoun::It, threshold).is_none(),
        "It is not a valid CountNp controller",
    );
    let count = Ability::Spell(Spell {
        effect: Sentence::Declarative(Declarative {
            subject: NounPhrase::Count(count),
            predicate: VerbPhrase::GainLife(GainLife {
                amount: variable_x(),
            }),
        }),
    });
    let count_text = count.render(&plain_context, &environment);
    assert_eq!(parser.parse(&count_text, &plain_context), Ok(count));
    assert_eq!(
        parser
            .parse(&count_text, &plain_context)
            .expect("rendered CountNp sentence parses")
            .render(&plain_context, &environment),
        count_text,
    );

    assert!(
        SelfReferenceNp::new(SelfReferenceSpelling::Abbreviated, &plain_context).is_none(),
        "an unavailable abbreviation is rejected",
    );
    let abbreviated_context = context("Zacama, Primal Calamity");
    let self_reference =
        SelfReferenceNp::new(SelfReferenceSpelling::Abbreviated, &abbreviated_context)
            .expect("a distinct abbreviated spelling is valid");
    let _: SelfReferenceSpelling = self_reference.spelling();
    assert_eq!(
        self_reference.spelling(),
        SelfReferenceSpelling::Abbreviated
    );
    let self_reference = Ability::Spell(Spell {
        effect: Sentence::Declarative(Declarative {
            subject: NounPhrase::SelfReference(self_reference),
            predicate: VerbPhrase::GainLife(GainLife {
                amount: variable_x(),
            }),
        }),
    });
    let self_reference_text = self_reference.render(&abbreviated_context, &environment);
    assert_eq!(
        parser.parse(&self_reference_text, &abbreviated_context),
        Ok(self_reference)
    );
    assert_eq!(
        parser
            .parse(&self_reference_text, &abbreviated_context)
            .expect("rendered SelfReferenceNp sentence parses")
            .render(&abbreviated_context, &environment),
        self_reference_text,
    );
}

#[test]
fn generated_invariant_production_fields_have_exact_privacy_and_accessors() {
    use syn::Fields;
    use syn::ImplItem;
    use syn::Item;
    use syn::Type;
    use syn::Visibility;

    let source =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/constructions.rs"))
            .expect("production construction source is readable");
    let invocation = deckmaste_construction_core::invocation_from_source(&source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction inventory compiles");
    let file = syn::parse2::<syn::File>(expansion.tokens()).expect("generated Rust parses");

    for (product, expected_fields, expected_methods) in [
        (
            "Triggered",
            &[("trigger", true), ("event", false), ("effect", true)][..],
            &["new", "event"][..],
        ),
        (
            "WithWhere",
            &[("body", true), ("clause", false)][..],
            &["new", "clause"][..],
        ),
        (
            "CountNp",
            &[("head", true), ("controller", false), ("threshold", true)][..],
            &["new", "controller"][..],
        ),
        (
            "SelfReferenceNp",
            &[("spelling", false)][..],
            &["new", "spelling"][..],
        ),
    ] {
        let structure = file
            .items
            .iter()
            .find_map(|item| match item {
                Item::Struct(structure) if structure.ident == product => Some(structure),
                _ => None,
            })
            .unwrap_or_else(|| panic!("generated {product} struct exists"));
        let Fields::Named(fields) = &structure.fields else {
            panic!("generated {product} fields are named");
        };
        assert_eq!(
            fields
                .named
                .iter()
                .map(|field| (
                    field.ident.as_ref().unwrap().to_string(),
                    matches!(field.vis, Visibility::Public(_)),
                ))
                .collect::<Vec<_>>(),
            expected_fields
                .iter()
                .map(|(name, public)| ((*name).to_owned(), *public))
                .collect::<Vec<_>>(),
            "{product} has the exact sealed field privacy",
        );

        let implementation = file.items.iter().find_map(|item| match item {
            Item::Impl(implementation)
                if implementation.trait_.is_none()
                    && matches!(
                        implementation.self_ty.as_ref(),
                        Type::Path(path) if path.path.is_ident(product)
                    ) =>
            {
                Some(implementation)
            }
            _ => None,
        });
        let methods = implementation
            .into_iter()
            .flat_map(|implementation| &implementation.items)
            .filter_map(|item| match item {
                ImplItem::Fn(method) => Some(method.sig.ident.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            methods, expected_methods,
            "{product} has no manufactured accessor or authored implementation",
        );
    }
}

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
            DeclarationId::new(DeclarationKind::Type, "Creature"),
            SurfaceFeature::Singular,
        )
        .expect("Creature is a normalized noun declaration"),
    )
}

fn creatures() -> Noun {
    let environment = environment();
    Noun::Declaration(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Creature"),
            SurfaceFeature::Plural,
        )
        .expect("Creature has a normalized plural noun reading"),
    )
}

fn target_creature() -> NounPhrase {
    NounPhrase::Target(TargetNp { head: creature() })
}

fn variable_x() -> Amount {
    Amount::Variable(VariableAmount {
        variable: Variable::X,
    })
}

fn destroy_target_creature() -> Ability {
    Ability::Spell(Spell {
        effect: Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Destroy(Destroy {
                object: target_creature(),
            }),
        }),
    })
}

fn connive_event() -> Clause {
    Clause::Event(EventClause {
        subject: NounPhrase::Common(Common {
            article: Article::A,
            head: Noun::Lexeme(NounLexeme::Player),
        }),
        predicate: VerbPhrase::Connive(Connive),
    })
}

fn triggered_damage() -> Ability {
    Ability::Triggered(
        Triggered::new(
            TriggerWord::Whenever,
            connive_event(),
            Sentence::Declarative(Declarative {
                subject: NounPhrase::Demonstrative(DemonstrativeNp {
                    word: Demonstrative::That,
                    head: creature(),
                }),
                predicate: VerbPhrase::DealDamage(DealDamage {
                    amount: variable_x(),
                    to: NounPhrase::Pronoun(PronounNp { word: Pronoun::It }),
                }),
            }),
        )
        .expect("one effect is valid"),
    )
}

fn gain_life_sentence() -> Sentence {
    Sentence::Declarative(Declarative {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::GainLife(GainLife {
            amount: variable_x(),
        }),
    })
}

fn gain_life_with_where() -> Ability {
    Ability::Spell(Spell {
        effect: Sentence::WithWhere(
            WithWhere::new(
                Box::new(gain_life_sentence()),
                Clause::Where(WhereClause {
                    variable: Variable::X,
                    value: NounPhrase::Count(
                        CountNp::new(
                            creatures(),
                            Pronoun::You,
                            SignedNumber {
                                sign: Sign::Positive,
                                magnitude: 2,
                            },
                        )
                        .expect("You is a valid count controller"),
                    ),
                }),
            )
            .expect("Where is a valid trailing clause"),
        ),
    })
}

fn zacama_deals_damage() -> Ability {
    Ability::Spell(Spell {
        effect: Sentence::Declarative(Declarative {
            subject: NounPhrase::SelfReference(self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            )),
            predicate: VerbPhrase::DealDamage(DealDamage {
                amount: Amount::Number(NumberAmount {
                    number: SignedNumber {
                        sign: Sign::Positive,
                        magnitude: 3,
                    },
                }),
                to: target_creature(),
            }),
        }),
    })
}

fn triggered_gain_life() -> Ability {
    Ability::Triggered(
        Triggered::new(TriggerWord::Whenever, connive_event(), gain_life_sentence())
            .expect("one effect is valid"),
    )
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

    let Ability::Spell(expected) = destroy_target_creature() else {
        panic!("the focused fixture is an ordinary spell");
    };
    assert_eq!(
        ability.into_parse_result(),
        Ok(Ability::Spell(expected.clone()))
    );
    assert_eq!(sentence.into_parse_result(), Ok(expected.effect.clone()));
    assert_eq!(trace.root_name(), "Sentence");
    assert_eq!(trace.clone().into_parse_result(), Ok(expected.effect));
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
        let context = context(card_name);
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
            "AbilitySpell".to_owned(),
            "SentenceImperative".to_owned(),
            "VerbPhraseDestroy".to_owned(),
            "NounPhraseTarget".to_owned(),
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
        (2, 8)
    );
    assert_eq!((summary.lexeme_claims(), summary.lexeme_bytes()), (2, 16));
    assert_eq!(summary.vocab_claims(), 0);
    assert_eq!(summary.codec_claims(), 0);
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
        (
            "Deal X damage to target creature.",
            "lexeme:VerbLexeme/Deal/bare",
        ),
        (
            "It deals X damage to target creature.",
            "lexeme:VerbLexeme/Deal/third_person_singular",
        ),
        (
            "Whenever a player connives, you gain X life.",
            "lexeme:NounLexeme/Player/singular",
        ),
        (
            "Those players deal X damage to it.",
            "lexeme:NounLexeme/Player/plural",
        ),
        (
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            "lexeme:VerbLexeme/Be/third_person_singular",
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
        ("Élan deals 3 damage to target creature.", "Élan"),
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
            assert_eq!(candidate.construction_path().total(), 4);
            assert_eq!(candidate.construction_path().shown(), usize::min(limit, 4));
            assert_eq!(candidate.specificity().total(), 8);
            assert_eq!(candidate.specificity().shown(), usize::min(limit, 8));
        }

        if limit == usize::MAX {
            let candidate = &trace.materialized_candidates().items()[0];
            let parsed = parser.parse(text, &context).expect("selected ability");
            let analysis = parser.analyze(text, &context);
            let complete = analysis.decision().expect("complete Task 3 decision");
            assert_eq!(candidate.ordinal(), 0);
            assert_eq!(candidate.rendered(), text);
            assert_eq!(candidate.ast_debug_v1(), format!("{parsed:?}"));
            assert_eq!(
                candidate.construction_path().items(),
                [
                    "AbilitySpell",
                    "SentenceImperative",
                    "VerbPhraseDestroy",
                    "NounPhraseTarget",
                ]
            );
            assert_eq!(candidate.specificity().total(), 8);
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

    for (limit, expected_shown) in [(0, 0), (1, 1), (8, 2)] {
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
        assert_eq!(failure.expectations().total(), 2);
        assert_eq!(failure.expectations().shown(), expected_shown);
        assert_eq!(failure.expectations().omitted(), 2 - expected_shown);
        if limit > 0 {
            assert_eq!(
                failure.expectations().items()[0],
                deckmaste_english_v2::parser::ExpectationInfo::Literal(",")
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
    let expected = Ability::Spell(Spell {
        effect: Sentence::Declarative(Declarative {
            subject: NounPhrase::SelfReference(self_reference(
                SelfReferenceSpelling::Full,
                "Context Card",
            )),
            predicate: VerbPhrase::DealDamage(DealDamage {
                amount: Amount::Number(NumberAmount {
                    number: SignedNumber {
                        sign: Sign::Positive,
                        magnitude: 3,
                    },
                }),
                to: target_creature(),
            }),
        }),
    });

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn self_reference_identity_preserves_its_inherent_case() {
    let text = "eBay deals 3 damage to target creature.";
    let context = context("eBay");
    let expected = Ability::Spell(Spell {
        effect: Sentence::Declarative(Declarative {
            subject: NounPhrase::SelfReference(self_reference(SelfReferenceSpelling::Full, "eBay")),
            predicate: VerbPhrase::DealDamage(DealDamage {
                amount: Amount::Number(NumberAmount {
                    number: SignedNumber {
                        sign: Sign::Positive,
                        magnitude: 3,
                    },
                }),
                to: target_creature(),
            }),
        }),
    });

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn disallowed_declaration_kind_is_a_parse_failure() {
    let text = "Destroy target flying.";
    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 15, end: 21 },
            expectations: BTreeSet::from([Expectation::Terminal(TerminalClass::Noun)]),
        })
    );
}

#[test]
fn missing_period_reports_chart_derived_literal_expectation() {
    let text = "Destroy target creature";
    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan {
                start: text.len(),
                end: text.len(),
            },
            expectations: BTreeSet::from([Expectation::Literal(","), Expectation::Literal("."),]),
        })
    );
}

#[test]
fn agreement_mismatch_reports_a_nonempty_chart_failure() {
    let text = "You gains X life.";

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 12, end: 16 },
            expectations: BTreeSet::from([Expectation::Literal("life")]),
        })
    );
}

#[test]
fn count_controller_mismatch_reports_a_nonempty_chart_failure() {
    let text =
        "You gain X life, where X is the number of creatures it controls with power 2 or less.";

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 55, end: 63 },
            expectations: BTreeSet::from([Expectation::Terminal(TerminalClass::VerbLexeme)]),
        })
    );
}

#[test]
fn lexical_matches_reject_prefixes_of_longer_lexemes() {
    for (text, card_name, expected_span) in [
        (
            "Destroyed target creature.",
            "Context Card",
            TextSpan { start: 0, end: 9 },
        ),
        (
            "Zacama deals 3x damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 13, end: 15 },
        ),
        (
            "Destroy target creaturex.",
            "Context Card",
            TextSpan { start: 15, end: 24 },
        ),
        (
            "Zacamaé deals 3 damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 0, end: 8 },
        ),
    ] {
        let Err(ParseError::Failure { span, expectations }) =
            parser().parse(text, &context(card_name))
        else {
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

    assert_eq!(span, TextSpan { start: 6, end: 7 });
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
            expectations: BTreeSet::from([Expectation::Terminal(TerminalClass::EndOfInput,)]),
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
