//! Every declared verb and noun has its full paradigm, and every lexical value
//! realizes to a spelling that analyzes back to it.

use std::collections::BTreeSet;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_lexical_source_v4::Sources;

/// Only the modals and the negative auxiliaries are defective: they have only
/// finite forms.
fn defective(sources: &Sources) -> BTreeSet<String> {
    sources
        .closed_class
        .auxiliaries
        .iter()
        .filter(|entry| entry.finite_only)
        .map(|entry| format!("verb:{}", entry.lemma))
        .collect()
}

#[test]
fn every_verb_has_its_full_paradigm() {
    let sources = Sources::load().unwrap();
    let defective = defective(&sources);
    let (lexemes, _) = sources.vocabulary().unwrap();
    let finite: Vec<(WordForm, Person, Number)> = [WordForm::Present, WordForm::Preterite]
        .into_iter()
        .flat_map(|form| {
            [Person::First, Person::Second, Person::Third]
                .into_iter()
                .flat_map(move |person| {
                    [Number::Singular, Number::Plural]
                        .into_iter()
                        .map(move |number| (form, person, number))
                })
        })
        .collect();
    for lexeme in lexemes.iter().filter(|l| l.category == Category::Verb) {
        let has = |form: WordForm, person: Option<Person>, number: Option<Number>| {
            lexeme.forms.iter().any(|slot| {
                slot.form == form
                    && slot.features.person == person
                    && slot.features.number == number
            })
        };
        let id = lexeme.id.to_string();
        if defective.contains(&id) {
            for person in [Person::First, Person::Second, Person::Third] {
                for number in [Number::Singular, Number::Plural] {
                    assert!(has(WordForm::Present, Some(person), Some(number)), "{id}");
                }
            }
            continue;
        }
        for form in [
            WordForm::Plain,
            WordForm::GerundParticiple,
            WordForm::PastParticiple,
        ] {
            assert!(has(form, None, None), "{id} lacks its {form:?} form");
        }
        for (form, person, number) in &finite {
            assert!(
                has(*form, Some(*person), Some(*number)),
                "{id} lacks its {form:?} {person:?} {number:?} form"
            );
        }
    }
}

#[test]
fn every_noun_has_its_full_paradigm() {
    let (lexemes, _) = Sources::load().unwrap().vocabulary().unwrap();
    for lexeme in lexemes
        .iter()
        .filter(|l| l.category == Category::Noun && l.binding == Binding::Free)
    {
        let forms: BTreeSet<WordForm> = lexeme.forms.iter().map(|slot| slot.form).collect();
        let count = lexeme
            .properties
            .countability
            .contains(&Countability::Count);
        assert!(
            !lexeme.properties.countability.is_empty(),
            "{} declares no countability",
            lexeme.id
        );
        let expected: BTreeSet<WordForm> = if count {
            [WordForm::Singular, WordForm::Plural].into()
        } else {
            [WordForm::Singular].into()
        };
        assert_eq!(forms, expected, "{}", lexeme.id);
    }
}

#[test]
fn oracle_you_is_one_player() {
    let sources = Sources::load().unwrap();
    let you = sources
        .closed_class
        .pronouns
        .iter()
        .find(|pronoun| pronoun.lemma == "you")
        .expect("you");
    for form in &you.forms {
        assert_eq!(
            (form.person, form.number),
            (Person::Second, Number::Singular),
            "{form:?}"
        );
    }
}

/// Every lexical value realizes to a spelling that analyzes back to the same
/// value, on the cards whose scope admits it: its own card for a Shortened
/// Name, the introducing cards for an Introduced Name or an Anchor Word.
#[test]
fn every_value_realizes_and_analyzes_back() {
    let environment = LexicalEnvironment::load().unwrap();
    let lexicon = &environment.lexicon;
    for value in lexicon.values() {
        let reading = LexicalReading::Word(value.clone());
        let surface = lexicon.realize(&reading).unwrap();
        let cards = &lexicon.lexemes()[&value.lexeme].card_scope;
        let scope: BTreeSet<LexemeId> =
            if cards.is_empty() { [value.lexeme].into() } else { cards.clone() };
        let analyzed = lexicon.analyze_source_scoped(&surface, None, &scope);
        let whole = analyzed.tokens.len();
        assert!(
            analyzed
                .matches
                .iter()
                .any(|found| found.start == 0 && found.end == whole && found.reading == reading),
            "{value:?} realizes as {surface:?}, which does not analyze back to it"
        );
    }
}
