use std::path::Path;

use deckmaste_catalogs::CatalogKind;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::walk_amount;
use deckmaste_english_v2::visit::Visitor;

#[derive(Default)]
struct RecordingVisitor {
    amounts: Vec<Amount>,
    catalog_identities: Vec<(CatalogKind, String)>,
    variables: Vec<Variable>,
    signed_numbers: Vec<(Sign, u32)>,
    self_names: Vec<String>,
    self_reference_spellings: Vec<SelfReferenceSpelling>,
    trigger_words: Vec<TriggerWord>,
    verbs: Vec<VerbLexeme>,
    catalog_spellings: Vec<String>,
}

impl Visitor for RecordingVisitor {
    fn visit_amount(&mut self, amount: &Amount) {
        self.amounts.push(amount.clone());
        walk_amount(self, amount);
    }

    fn visit_catalog_identity(&mut self, identity: &CatalogIdentity) {
        self.catalog_identities
            .push((identity.kind(), identity.spelling().to_owned()));
    }

    fn visit_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    fn visit_signed_number(&mut self, number: &SignedNumber) {
        self.signed_numbers
            .push((number.sign(), number.magnitude()));
    }

    fn visit_self_name(&mut self, name: &SelfName) {
        self.self_names.push(name.full().to_owned());
    }

    fn visit_self_reference_spelling(&mut self, spelling: SelfReferenceSpelling) {
        self.self_reference_spellings.push(spelling);
    }

    fn visit_trigger_word(&mut self, word: TriggerWord) {
        self.trigger_words.push(word);
    }

    fn visit_verb_lexeme(&mut self, verb: VerbLexeme) {
        self.verbs.push(verb);
    }

    fn visit_catalog_spelling(&mut self, spelling: &str) {
        self.catalog_spellings.push(spelling.to_owned());
    }
}

fn catalogs() -> ParserCatalogs {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    ParserCatalogs::load(&path).expect("canonical generated catalogs load")
}

fn card_type(spelling: &str) -> Noun {
    Noun::Catalog(
        CatalogIdentity::new(&catalogs(), CatalogKind::CardTypes, spelling)
            .expect("canonical card type is present"),
    )
}

fn creature() -> Noun {
    card_type("Creature")
}

fn artifact() -> Noun {
    card_type("Artifact")
}

fn target_creature() -> NounPhrase {
    NounPhrase::Target(TargetNp { head: creature() })
}

fn it() -> NounPhrase {
    NounPhrase::Pronoun(PronounNp { word: Pronoun::It })
}

fn damage(amount: Amount) -> VerbPhrase {
    VerbPhrase::DealDamage(DealDamage { to: it(), amount })
}

fn triggered_damage() -> Ability {
    let event = Clause::Event(EventClause {
        subject: NounPhrase::Common(Common {
            article: Article::A,
            head: Noun::Lexeme(NounLexeme::Player),
        }),
        predicate: VerbPhrase::Connive(Connive),
    });
    let effect = Sentence::Declarative(Declarative {
        subject: NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            head: creature(),
        }),
        predicate: damage(Amount::Variable(VariableAmount {
            variable: Variable::X,
        })),
    });
    Ability::Triggered(Triggered::new(TriggerWord::Whenever, event, vec![effect]).unwrap())
}

fn gain_life_with_where() -> Sentence {
    Sentence::WithWhere(WithWhere {
        body: Box::new(Sentence::Declarative(Declarative {
            subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
            predicate: VerbPhrase::GainLife(GainLife {
                amount: Amount::Variable(VariableAmount {
                    variable: Variable::X,
                }),
            }),
        })),
        clause: Clause::Where(WhereClause {
            variable: Variable::X,
            value: NounPhrase::Count(CountNp {
                head: creature(),
                controller: Pronoun::You,
                threshold: SignedNumber::new(Sign::Positive, 2),
            }),
        }),
    })
}

#[test]
fn catalog_identity_rejects_a_spelling_absent_from_the_bound_catalog() {
    let catalogs = catalogs();
    assert!(CatalogIdentity::new(&catalogs, CatalogKind::CardTypes, "Creature").is_some());
    assert!(
        CatalogIdentity::new(&catalogs, CatalogKind::CardTypes, "Definitely Not A Type").is_none()
    );
}

#[test]
fn abbreviated_self_reference_requires_a_bound_abbreviation() {
    let full_only = SelfName::new("Nameless One", None::<String>).unwrap();
    assert!(SelfReferenceNp::new(full_only, SelfReferenceSpelling::Abbreviated).is_none());

    let zacama = SelfName::new("Zacama, Primal Calamity", Some("Zacama")).unwrap();
    assert!(SelfReferenceNp::new(zacama, SelfReferenceSpelling::Abbreviated).is_some());
}

#[test]
fn triggered_ability_rejects_any_effect_count_other_than_one() {
    let event = Clause::Event(EventClause {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::Connive(Connive),
    });
    assert!(Triggered::new(TriggerWord::Whenever, event.clone(), Vec::new()).is_none());
    let effect = Sentence::Imperative(Imperative {
        predicate: VerbPhrase::Connive(Connive),
    });
    assert!(Triggered::new(TriggerWord::Whenever, event, vec![effect.clone(), effect],).is_none());
}

#[test]
fn renders_destroy_target_creature_exactly() {
    let value = Ability::Spell(Spell {
        effect: Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Destroy(Destroy {
                object: target_creature(),
            }),
        }),
    });
    assert_eq!(value.render(), "Destroy target creature.");
}

#[test]
fn renders_triggered_damage_exactly_without_capitalizing_after_the_comma() {
    let value = triggered_damage();
    assert_eq!(
        value.render(),
        "Whenever a player connives, that creature deals X damage to it."
    );
}

#[test]
fn renders_gain_life_with_a_where_binder_exactly() {
    let value = gain_life_with_where();
    assert_eq!(
        value.render(),
        "You gain X life, where X is the number of creatures you control with power 2 or less."
    );
}

#[test]
fn renders_real_abbreviated_self_reference_with_a_catalog_identity() {
    let zacama = SelfName::new("Zacama, Primal Calamity", Some("Zacama")).unwrap();
    let subject = SelfReferenceNp::new(zacama, SelfReferenceSpelling::Abbreviated).unwrap();
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::SelfReference(subject),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: SignedNumber::new(Sign::Positive, 3),
            }),
            to: target_creature(),
        }),
    });
    assert_eq!(value.render(), "Zacama deals 3 damage to target creature.");
}

#[test]
fn renders_those_with_a_plural_noun_and_bare_verb() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            head: creature(),
        }),
        predicate: damage(Amount::Number(NumberAmount {
            number: SignedNumber::new(Sign::Positive, 3),
        })),
    });
    assert_eq!(value.render(), "Those creatures deal 3 damage to it.");
}

#[test]
fn renders_an_with_a_singular_noun_and_third_person_verb() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Common(Common {
            article: Article::An,
            head: artifact(),
        }),
        predicate: damage(Amount::Number(NumberAmount {
            number: SignedNumber::new(Sign::Positive, 3),
        })),
    });
    assert_eq!(value.render(), "An artifact deals 3 damage to it.");
}

#[test]
fn visitor_reaches_every_vertical_slice_leaf() {
    let destroy = Ability::Spell(Spell {
        effect: Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Destroy(Destroy {
                object: target_creature(),
            }),
        }),
    });
    let zacama = SelfName::new("Zacama, Primal Calamity", Some("Zacama")).unwrap();
    let self_reference = Sentence::Declarative(Declarative {
        subject: NounPhrase::SelfReference(
            SelfReferenceNp::new(zacama, SelfReferenceSpelling::Abbreviated).unwrap(),
        ),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: SignedNumber::new(Sign::Positive, 3),
            }),
            to: target_creature(),
        }),
    });

    let mut visitor = RecordingVisitor::default();
    visitor.visit_ability(&destroy);
    visitor.visit_ability(&triggered_damage());
    visitor.visit_sentence(&gain_life_with_where());
    visitor.visit_sentence(&self_reference);

    assert_eq!(
        visitor.amounts,
        vec![
            Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
            Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
            Amount::Number(NumberAmount {
                number: SignedNumber::new(Sign::Positive, 3),
            }),
        ]
    );
    assert_eq!(
        visitor.catalog_identities,
        vec![
            (CatalogKind::CardTypes, "Creature".to_owned()),
            (CatalogKind::CardTypes, "Creature".to_owned()),
            (CatalogKind::CardTypes, "Creature".to_owned()),
            (CatalogKind::CardTypes, "Creature".to_owned()),
        ]
    );
    assert_eq!(
        visitor.variables,
        vec![Variable::X, Variable::X, Variable::X]
    );
    assert_eq!(
        visitor.signed_numbers,
        vec![(Sign::Positive, 2), (Sign::Positive, 3)]
    );
    assert_eq!(visitor.self_names, vec!["Zacama, Primal Calamity"]);
    assert_eq!(
        visitor.self_reference_spellings,
        vec![SelfReferenceSpelling::Abbreviated]
    );
    assert_eq!(visitor.trigger_words, vec![TriggerWord::Whenever]);
    assert_eq!(
        visitor.verbs,
        vec![
            VerbLexeme::Destroy,
            VerbLexeme::Connive,
            VerbLexeme::Deal,
            VerbLexeme::Gain,
            VerbLexeme::Be,
            VerbLexeme::Control,
            VerbLexeme::Deal,
        ]
    );
    assert_eq!(
        visitor.catalog_spellings,
        vec!["Creature", "Creature", "Creature", "Creature"]
    );
}
