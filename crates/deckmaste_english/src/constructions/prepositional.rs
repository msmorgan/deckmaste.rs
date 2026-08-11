//! Compiler-derived prepositional objects and phrases.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::grammar::Features;
use crate::grammar::NounPhraseCoordinationState;
use crate::grammar::PrepositionalObjectCategory;
use crate::grammar::PrepositionalRoleMember;
use crate::syntax::GerundClause;
use crate::syntax::NounPhrase;
use crate::syntax::Preposition;
pub use crate::syntax::PrepositionalObject;
pub use crate::syntax::PrepositionalObjectKind;
use crate::syntax::PrepositionalPhrase;
use crate::word::Vocab;

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
        (Some(value), None, None, None) => Ok(PrepositionalObject::from_kind(
            PrepositionalObjectKind::NounPhrase(Box::new(value)),
        )),
        (None, Some(value), None, None) => Ok(PrepositionalObject::from_kind(
            PrepositionalObjectKind::PrepositionalPhrase(Box::new(value)),
        )),
        (None, None, Some(value), None) => Ok(PrepositionalObject::from_kind(
            PrepositionalObjectKind::GerundClause(Box::new(value)),
        )),
        (None, None, None, Some(value)) if value.is_adverb() => Ok(PrepositionalObject::from_kind(
            PrepositionalObjectKind::Adverb(value),
        )),
        (None, None, None, Some(_)) => Err(violation(
            "prepositional_object",
            "the adverb alternative has adverb lexical capability",
        )),
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
    match value.kind() {
        PrepositionalObjectKind::NounPhrase(value) => {
            (Some(value.as_ref().clone()), None, None, None)
        }
        PrepositionalObjectKind::PrepositionalPhrase(value) => {
            (None, Some(value.as_ref().clone()), None, None)
        }
        PrepositionalObjectKind::GerundClause(value) => {
            (None, None, Some(value.as_ref().clone()), None)
        }
        PrepositionalObjectKind::Adverb(value) => (None, None, None, Some(*value)),
    }
}

fn is_noun_phrase_object(value: &PrepositionalObject) -> bool {
    matches!(value.kind(), PrepositionalObjectKind::NounPhrase(_))
}

fn is_prepositional_phrase_object(value: &PrepositionalObject) -> bool {
    matches!(
        value.kind(),
        PrepositionalObjectKind::PrepositionalPhrase(_)
    )
}

fn is_gerund_clause_object(value: &PrepositionalObject) -> bool {
    matches!(value.kind(), PrepositionalObjectKind::GerundClause(_))
}

fn is_adverb_object(value: &PrepositionalObject) -> bool {
    matches!(value.kind(), PrepositionalObjectKind::Adverb(_))
}

pub(crate) fn every_prepositional_member(
    value: &PrepositionalPhrase,
    mut predicate: impl FnMut(&crate::syntax::SimplePrepositionalPhrase) -> bool,
) -> bool {
    match value.kind() {
        crate::syntax::PrepositionalPhraseKind::Simple(value) => predicate(value),
        crate::syntax::PrepositionalPhraseKind::Coordinated(value) => {
            predicate(value.first()) && value.rest().iter().all(|member| predicate(member.phrase()))
        }
    }
}

pub(crate) fn expect_prepositional_phrase(
    preposition: Preposition,
    object: PrepositionalObjectKind,
) -> PrepositionalPhrase {
    let object = match object {
        PrepositionalObjectKind::NounPhrase(value) => {
            build_prepositional_object(Some(*value), None, None, None)
        }
        PrepositionalObjectKind::PrepositionalPhrase(value) => {
            build_prepositional_object(None, Some(*value), None, None)
        }
        PrepositionalObjectKind::GerundClause(value) => {
            build_prepositional_object(None, None, Some(*value), None)
        }
        PrepositionalObjectKind::Adverb(value) => {
            build_prepositional_object(None, None, None, Some(value))
        }
    }
    .expect("the internal typed object satisfies P02");
    build_prepositional_phrase(preposition, object)
        .expect("the internal prepositional object satisfies P02")
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
        role_members: vec![PrepositionalRoleMember {
            preposition: *preposition,
            nominal_attachment: prepositional_nominal_attachment_is_eligible(*preposition, *gerund),
        }],
        shared_determiner_object: *shared_determiner,
        nearer_relative_host: prepositional_nearer_relative_host(*preposition, object),
    })
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "the declaration adapter retains the generated checked-builder ABI"
)]
fn make_prepositional_phrase(
    preposition: Preposition,
    object: PrepositionalObject,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    Ok(PrepositionalPhrase::from_prepositional_declaration(
        preposition,
        object,
    ))
}

#[allow(
    dead_code,
    reason = "the declaration retains this inverse adapter for generated linearization"
)]
fn prepositional_phrase_parts(value: &PrepositionalPhrase) -> (Preposition, PrepositionalObject) {
    let value = value.head();
    (value.preposition, value.object.clone())
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
        bind PrepositionalObject via make_prepositional_object, prepositional_object_parts {
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

pub(crate) fn linearize_simple_with<V>(
    value: &PrepositionalPhrase,
    visitor: &mut V,
) -> Result<(), deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
where
    V: deckmaste_construction_compiler::runtime::LinearizationVisitor,
{
    linearize_prepositional_prepositional_phrase_with(value, visitor)
}

pub(crate) static GROUPS: &[&GroupData] = &[&PREPOSITIONAL_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::NominalPhrase;
    use crate::syntax::PrepositionalPhraseKind;
    use crate::word::Noun;
    use crate::word::NounInstance;

    fn card() -> NounPhrase {
        crate::constructions::noun_phrase::build_noun_phrase_nominal(
            NominalPhrase::try_from_noun(NounInstance::unchecked_singular(Noun::Word(Vocab::Card)))
                .expect("card is a nominal"),
        )
        .expect("the nominal enters P01")
    }

    #[test]
    fn simple_p02_stores_its_checked_typed_object() {
        let card = card();
        let object = build_prepositional_object(Some(card.clone()), None, None, None)
            .expect("one noun-phrase alternative is valid");
        let phrase = build_prepositional_phrase(Preposition::Of, object)
            .expect("the typed object enters a simple P02 phrase");
        let PrepositionalPhraseKind::Simple(simple) = phrase.kind() else {
            panic!("P02 produces a direct simple alternative")
        };
        assert_eq!(simple.preposition(), Preposition::Of);
        assert!(matches!(
            simple.object().kind(),
            PrepositionalObjectKind::NounPhrase(value) if value.as_ref() == &card
        ));
    }

    #[test]
    fn object_sum_rejects_multiple_alternatives() {
        assert!(build_prepositional_object(Some(card()), None, None, Some(Vocab::Again)).is_err());
    }

    #[test]
    fn adverb_object_accepts_a_declared_adverb() {
        let object = build_prepositional_object(None, None, None, Some(Vocab::Again))
            .expect("Again has adverb lexical capability");
        assert!(matches!(
            object.kind(),
            PrepositionalObjectKind::Adverb(Vocab::Again)
        ));
    }

    #[test]
    fn adverb_object_rejects_a_non_adverb_vocab() {
        assert_eq!(
            build_prepositional_object(None, None, None, Some(Vocab::Card)),
            Err(DeclarationViolation {
                construction: "prepositional_object",
                requirement: "the adverb alternative has adverb lexical capability",
            })
        );
    }
}
