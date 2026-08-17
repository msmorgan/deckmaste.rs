use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_catalogs::CatalogKind;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::Expectation;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::TerminalClass;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::render::Render;

fn catalogs() -> ParserCatalogs {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    ParserCatalogs::load(&path).expect("canonical generated catalogs load")
}

fn parser() -> Parser {
    Parser::new(catalogs())
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
    Noun::Catalog(
        CatalogIdentity::new(&catalogs(), CatalogKind::CardTypes, "Creature")
            .expect("Creature is a canonical card type"),
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
            vec![Sentence::Declarative(Declarative {
                subject: NounPhrase::Demonstrative(DemonstrativeNp {
                    word: Demonstrative::That,
                    head: creature(),
                }),
                predicate: VerbPhrase::DealDamage(DealDamage {
                    amount: variable_x(),
                    to: NounPhrase::Pronoun(PronounNp { word: Pronoun::It }),
                }),
            })],
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
        effect: Sentence::WithWhere(WithWhere {
            body: Box::new(gain_life_sentence()),
            clause: Clause::Where(WhereClause {
                variable: Variable::X,
                value: NounPhrase::Count(CountNp {
                    head: creature(),
                    controller: Pronoun::You,
                    threshold: SignedNumber {
                        sign: Sign::Positive,
                        magnitude: 2,
                    },
                }),
            }),
        }),
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
        Triggered::new(
            TriggerWord::Whenever,
            connive_event(),
            vec![gain_life_sentence()],
        )
        .expect("one effect is valid"),
    )
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
        assert_eq!(expected.render(&context), text);

        let rendered = expected.render(&context);
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
    assert_eq!(first.selected().unwrap().render(&context), text);
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
fn parser_analysis_projects_representative_parse_failures_without_changing_them() {
    let parser = parser();
    for text in [
        "Destroy target creature",
        "You gains X life.",
        "Destroy target Forest.",
    ] {
        let context = context("Context Card");
        assert_eq!(
            parser.parse(text, &context),
            parser.analyze(text, &context).into_parse_result(),
            "{text:?}",
        );
    }
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
    assert_eq!(expected.render(&context), text);
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
    assert_eq!(expected.render(&context), text);
}

#[test]
fn unrelated_catalog_collision_is_a_parse_failure() {
    let text = "Destroy target Forest.";
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
            span: TextSpan { start: 80, end: 84 },
            expectations: BTreeSet::from([Expectation::Literal("less")]),
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
