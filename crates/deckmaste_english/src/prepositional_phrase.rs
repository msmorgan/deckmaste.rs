//! Checked prepositional-phrase construction, projection, and rendering.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
pub use crate::constructions::prepositional::PrepositionalObject;
pub use crate::constructions::prepositional::PrepositionalObjectKind;
use crate::features::Conjunction;
use crate::syntax::Clause;
use crate::syntax::DependentClause;
use crate::syntax::GerundClause;
use crate::syntax::NounPhrase;
use crate::syntax::Phrase;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PrepositionalPhraseCoordination;
use crate::syntax::PrepositionalPhraseKind;
use crate::word::Vocab;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

/// Builds a checked P02 prepositional object from a legacy whole [`Phrase`].
///
/// # Errors
///
/// Returns a declaration violation unless `value` is a noun phrase, nested
/// prepositional phrase, gerund clause, or adverb.
pub fn build_prepositional_object(
    value: Phrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    match value {
        Phrase::NounPhrase(value) => build_prepositional_object_noun_phrase(*value),
        Phrase::PrepositionalPhrase(value) => {
            build_prepositional_object_prepositional_phrase(*value)
        }
        Phrase::Clause(value) => match *value {
            Clause::Dependent(DependentClause::Gerund(value)) => {
                build_prepositional_object_gerund_clause(value)
            }
            value => crate::constructions::prepositional::build_prepositional_object_from_phrase(
                Phrase::Clause(Box::new(value)),
            ),
        },
        Phrase::Adverb(value) => build_prepositional_object_adverb(value),
        value => crate::constructions::prepositional::build_prepositional_object_from_phrase(value),
    }
}

/// Builds the noun-phrase P02 object alternative.
pub fn build_prepositional_object_noun_phrase(
    value: NounPhrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(Some(value), None, None, None)
}

/// Builds the nested-prepositional-phrase P02 object alternative.
pub fn build_prepositional_object_prepositional_phrase(
    value: PrepositionalPhrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, Some(value), None, None)
}

/// Builds the gerund-clause P02 object alternative.
pub fn build_prepositional_object_gerund_clause(
    value: GerundClause,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, None, Some(value), None)
}

/// Builds the adverb P02 object alternative.
pub fn build_prepositional_object_adverb(
    value: Vocab,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, None, None, Some(value))
}

/// Returns a checked P02 object's borrowed typed alternative.
#[must_use]
pub fn parts_prepositional_object(value: &PrepositionalObject) -> PrepositionalObjectKind<'_> {
    value.kind()
}

/// Builds a simple P02 prepositional phrase.
pub fn build_prepositional_phrase(
    preposition: Preposition,
    object: PrepositionalObject,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_phrase(preposition, object)
}

/// Returns a simple P02 phrase's owned declaration parts.
///
/// # Panics
///
/// Panics when `value` is a C01 coordination rather than a simple P02 phrase.
#[must_use]
pub fn parts_prepositional_phrase(
    value: &PrepositionalPhrase,
) -> (Preposition, PrepositionalObject) {
    assert!(
        matches!(value.kind(), PrepositionalPhraseKind::Simple(_)),
        "prepositional_phrase admits only Simple",
    );
    crate::constructions::prepositional::parts_prepositional_phrase(value)
}

/// Builds a complete C01 sibling coordination from checked simple P02 members.
///
/// # Errors
///
/// Returns a declaration violation when there is no remaining member, any
/// member is already coordinated, a nonfinal member has a conjunction, or the
/// final conjunction is not nominal.
pub fn build_prepositional_phrase_coordination(
    first: PrepositionalPhrase,
    rest: Vec<(Option<Conjunction>, PrepositionalPhrase)>,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    let Some(first) = first.into_simple() else {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the first member is a simple P02 phrase",
        ));
    };
    if rest.is_empty()
        || rest
            .iter()
            .take(rest.len().saturating_sub(1))
            .any(|(conjunction, _)| conjunction.is_some())
        || !matches!(
            rest.last().and_then(|(conjunction, _)| *conjunction),
            Some(Conjunction::And | Conjunction::Or | Conjunction::AndOr)
        )
    {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the run has nonfinal bare members and one nominal final conjunction",
        ));
    }
    let rest = rest
        .into_iter()
        .map(|(conjunction, phrase)| {
            phrase
                .into_simple()
                .map(|phrase| PrepositionalPhraseCoordination {
                    conjunction,
                    phrase,
                })
                .ok_or_else(|| {
                    violation(
                        "prepositional_phrase_sibling_coordinated",
                        "every remaining member is a simple P02 phrase",
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PrepositionalPhrase::coordinated(first, rest))
}

/// Returns a C01 sibling coordination's owned checked phrase members.
///
/// # Panics
///
/// Panics when `value` is a simple P02 phrase rather than a C01 coordination.
#[must_use]
pub fn parts_prepositional_phrase_coordination(
    value: &PrepositionalPhrase,
) -> (
    PrepositionalPhrase,
    Vec<(Option<Conjunction>, PrepositionalPhrase)>,
) {
    let PrepositionalPhraseKind::Coordinated(value) = value.kind() else {
        panic!("prepositional_phrase_sibling_coordinated admits only Coordinated")
    };
    let first = crate::constructions::prepositional::build_prepositional_phrase_from_phrase(
        value.first().preposition(),
        value.first().object().clone(),
    )
    .expect("a coordinated member remains a checked simple phrase");
    let rest = value
        .rest()
        .iter()
        .map(|member| {
            (
                member.conjunction(),
                crate::constructions::prepositional::build_prepositional_phrase_from_phrase(
                    member.phrase().preposition(),
                    member.phrase().object().clone(),
                )
                .expect("a coordinated member remains a checked simple phrase"),
            )
        })
        .collect();
    (first, rest)
}

/// Renders a checked simple or coordinated prepositional phrase.
///
/// # Errors
///
/// Returns an error from a contained value, including a missing lexical form
/// or unavailable card-identity context.
pub fn render(
    value: &PrepositionalPhrase,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_prepositional_phrase(value, name, is_legendary)
}
