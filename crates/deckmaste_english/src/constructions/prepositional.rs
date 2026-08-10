//! Compiler-derived prepositional objects and phrases.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::grammar::Features;
use crate::grammar::NounPhraseCoordinationState;
use crate::grammar::PrepositionalObjectCategory;
use crate::syntax::Clause;
use crate::syntax::DependentClause;
use crate::syntax::GerundClause;
use crate::syntax::NounPhrase;
use crate::syntax::Phrase;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::word::Vocab;

type PrepositionalObject = Phrase;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_prepositional_object(
    noun_phrase: Option<NounPhrase>,
    prepositional_phrase: Option<PrepositionalPhrase>,
    gerund_clause: Option<GerundClause>,
    adverb: Option<Vocab>,
) -> Result<PrepositionalObject, DeclarationViolation> {
    match (noun_phrase, prepositional_phrase, gerund_clause, adverb) {
        (Some(value), None, None, None) => Ok(Phrase::NounPhrase(Box::new(value))),
        (None, Some(value), None, None) => Ok(Phrase::PrepositionalPhrase(Box::new(value))),
        (None, None, Some(value), None) => Ok(Phrase::Clause(Box::new(Clause::Dependent(
            DependentClause::Gerund(value),
        )))),
        (None, None, None, Some(value)) => Ok(Phrase::Adverb(value)),
        _ => Err(violation(
            "prepositional_object",
            "exactly one whole typed object alternative is present",
        )),
    }
}

#[allow(
    dead_code,
    reason = "the declaration retains this inverse adapter for generated linearization"
)]
fn prepositional_object_parts(
    value: &PrepositionalObject,
) -> (
    Option<NounPhrase>,
    Option<PrepositionalPhrase>,
    Option<GerundClause>,
    Option<Vocab>,
) {
    match value {
        Phrase::NounPhrase(value) => (Some(value.as_ref().clone()), None, None, None),
        Phrase::PrepositionalPhrase(value) => (None, Some(value.as_ref().clone()), None, None),
        Phrase::Clause(value) => match value.as_ref() {
            Clause::Dependent(DependentClause::Gerund(value)) => {
                (None, None, Some(value.clone()), None)
            }
            _ => (None, None, None, None),
        },
        Phrase::Adverb(value) => (None, None, None, Some(*value)),
        _ => (None, None, None, None),
    }
}

fn is_noun_phrase_object(value: &PrepositionalObject) -> bool {
    matches!(value, Phrase::NounPhrase(_))
}

fn is_prepositional_phrase_object(value: &PrepositionalObject) -> bool {
    matches!(value, Phrase::PrepositionalPhrase(_))
}

fn is_gerund_clause_object(value: &PrepositionalObject) -> bool {
    matches!(
        value,
        Phrase::Clause(clause)
            if matches!(clause.as_ref(), Clause::Dependent(DependentClause::Gerund(_)))
    )
}

fn is_adverb_object(value: &PrepositionalObject) -> bool {
    matches!(value, Phrase::Adverb(_))
}

fn prepositional_object_category(
    noun_phrase: Option<&Features>,
    prepositional_phrase: Option<&Features>,
    gerund_clause: Option<&Features>,
    adverb: Option<&Features>,
) -> Option<PrepositionalObjectCategory> {
    match (noun_phrase, prepositional_phrase, gerund_clause, adverb) {
        (Some(Features::NounPhrase { .. }), None, None, None) => {
            Some(PrepositionalObjectCategory::NounPhrase)
        }
        (None, Some(Features::PrepositionalPhrase { .. }), None, None) => {
            Some(PrepositionalObjectCategory::PrepositionalPhrase)
        }
        (None, None, Some(Features::GerundClause), None) => {
            Some(PrepositionalObjectCategory::GerundClause)
        }
        (None, None, None, Some(Features::None)) => Some(PrepositionalObjectCategory::Adverb),
        _ => None,
    }
}

const fn prepositional_object_is_gerund(category: PrepositionalObjectCategory) -> bool {
    matches!(category, PrepositionalObjectCategory::GerundClause)
}

fn prepositional_object_has_shared_determiner(
    category: PrepositionalObjectCategory,
    noun_phrase: Option<&Features>,
) -> bool {
    category == PrepositionalObjectCategory::NounPhrase
        && matches!(
            noun_phrase,
            Some(Features::NounPhrase {
                coordination: NounPhraseCoordinationState::Shared,
                ..
            })
        )
}

fn reduce_prepositional_object_features(
    noun_phrase: Option<&Features>,
    prepositional_phrase: Option<&Features>,
    gerund_clause: Option<&Features>,
    adverb: Option<&Features>,
) -> Option<Features> {
    let category =
        prepositional_object_category(noun_phrase, prepositional_phrase, gerund_clause, adverb)?;
    Some(Features::PrepositionalObject {
        object_category: category,
        gerund: prepositional_object_is_gerund(category),
        shared_determiner: prepositional_object_has_shared_determiner(category, noun_phrase),
    })
}

const fn prepositional_nominal_attachment_is_eligible(
    preposition: Preposition,
    gerund: bool,
) -> bool {
    !(matches!(preposition, Preposition::By) && gerund)
}

const fn prepositional_nearer_relative_host(_preposition: Preposition, _object: &Features) -> bool {
    false
}

fn reduce_prepositional_phrase_features(
    preposition: &Features,
    object: &Features,
) -> Option<Features> {
    let Features::Preposition(preposition) = preposition else {
        return None;
    };
    let Features::PrepositionalObject {
        object_category: _,
        gerund,
        shared_determiner,
    } = object
    else {
        return None;
    };
    Some(Features::PrepositionalPhrase {
        preposition: *preposition,
        nominal_attachment: prepositional_nominal_attachment_is_eligible(*preposition, *gerund),
        shared_determiner_object: *shared_determiner,
        nearer_relative_host: prepositional_nearer_relative_host(*preposition, object),
    })
}

fn make_prepositional_phrase(
    preposition: Preposition,
    object: PrepositionalObject,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    if !is_noun_phrase_object(&object)
        && !is_prepositional_phrase_object(&object)
        && !is_gerund_clause_object(&object)
        && !is_adverb_object(&object)
    {
        return Err(violation(
            "prepositional_phrase",
            "the object is one validated prepositional object category",
        ));
    }
    Ok(PrepositionalPhrase::simple(preposition, object))
}

#[allow(
    dead_code,
    reason = "the declaration retains this inverse adapter for generated linearization"
)]
fn prepositional_phrase_parts(value: &PrepositionalPhrase) -> (Preposition, PrepositionalObject) {
    let value = value.head();
    (value.preposition, value.object.as_ref().clone())
}

fn is_simple_prepositional_phrase(value: &PrepositionalPhrase) -> bool {
    value.as_simple().is_some()
}

deckmaste_constructions_macro::constructions! {
    group prepositional;

    construction prepositional_phrase: PrepositionalPhrase {
        bind PrepositionalPhrase via make_prepositional_phrase, prepositional_phrase_parts {
            preposition: identity Preposition via Preposition,
            object: hole PrepositionalObject,
        }
        derive features: Features = reduce_prepositional_phrase_features(preposition, object);
        form only @ 0 inverse check(is_simple_prepositional_phrase) = identity(preposition) object;
        selection unique;
    }

    construction prepositional_object: PrepositionalObject {
        bind Phrase via make_prepositional_object, prepositional_object_parts {
            noun_phrase: opt hole NounPhrase,
            prepositional_phrase: opt hole PrepositionalPhrase,
            gerund_clause: opt hole GerundClause,
            adverb: opt lex Vocab via Adverb,
        }
        require any(
            all(noun_phrase.is_some(), prepositional_phrase.is_none(), gerund_clause.is_none(), adverb.is_none()),
            all(noun_phrase.is_none(), prepositional_phrase.is_some(), gerund_clause.is_none(), adverb.is_none()),
            all(noun_phrase.is_none(), prepositional_phrase.is_none(), gerund_clause.is_some(), adverb.is_none()),
            all(noun_phrase.is_none(), prepositional_phrase.is_none(), gerund_clause.is_none(), adverb.is_some())
        );
        derive features: Features = reduce_prepositional_object_features(noun_phrase, prepositional_phrase, gerund_clause, adverb);
        evidence feature "prepositional object category" from output object_category;
        form noun_phrase @ 0 when all(noun_phrase.is_some(), prepositional_phrase.is_none(), gerund_clause.is_none(), adverb.is_none()) inverse check(is_noun_phrase_object) = noun_phrase;
        form prepositional_phrase @ 1 when all(noun_phrase.is_none(), prepositional_phrase.is_some(), gerund_clause.is_none(), adverb.is_none()) inverse check(is_prepositional_phrase_object) = prepositional_phrase;
        form gerund_clause @ 2 when all(noun_phrase.is_none(), prepositional_phrase.is_none(), gerund_clause.is_some(), adverb.is_none()) inverse check(is_gerund_clause_object) = gerund_clause;
        form adverb @ 3 when all(noun_phrase.is_none(), prepositional_phrase.is_none(), gerund_clause.is_none(), adverb.is_some()) inverse check(is_adverb_object) = lex(adverb);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&PREPOSITIONAL_DECLARATION];
