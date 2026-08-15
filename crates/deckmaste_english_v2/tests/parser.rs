use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_catalogs::CatalogKind;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::Expectation;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::render::Render;

fn catalogs() -> ParserCatalogs {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    ParserCatalogs::load(&path).expect("canonical generated catalogs load")
}

fn parser() -> Parser {
    Parser::new(catalogs())
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
            subject: NounPhrase::SelfReference(SelfReferenceNp {
                spelling: SelfReferenceSpelling::Abbreviated,
            }),
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
        let context = ParseContext::new(card_name);
        assert_eq!(parser.parse(text, &context), Ok(expected.clone()));
        assert_eq!(expected.render(&context), text);

        let rendered = expected.render(&context);
        assert_eq!(parser.parse(&rendered, &context), Ok(expected));
    }
}

#[test]
fn missing_period_reports_chart_derived_literal_expectation() {
    let text = "Destroy target creature";
    assert_eq!(
        parser().parse(text, &ParseContext::new("Context Card")),
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
fn wrong_case_and_wrong_whitespace_are_parse_failures() {
    for text in [
        "destroy target creature.",
        "Destroy Target creature.",
        "Destroy  target creature.",
        "Destroytarget creature.",
        "Whenever a player connives, you Gain X life.",
    ] {
        assert!(
            parser()
                .parse(text, &ParseContext::new("Context Card"))
                .is_err()
        );
    }
}
