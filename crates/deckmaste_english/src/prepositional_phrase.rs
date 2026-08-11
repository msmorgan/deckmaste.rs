//! Checked prepositional-phrase construction, semantic inspection, and
//! rendering.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
use crate::features::Conjunction;
use crate::syntax::GerundClause;
use crate::syntax::NounPhrase;
use crate::syntax::Preposition;
pub use crate::syntax::PrepositionalObject;
pub use crate::syntax::PrepositionalObjectKind;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PrepositionalPhraseCoordination;
use crate::word::Vocab;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

/// Builds the noun-phrase P02 object alternative.
///
/// # Errors
///
/// Returns a declaration violation if `value` fails the noun-phrase object
/// constraints.
pub fn build_prepositional_object_noun_phrase(
    value: NounPhrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(Some(value), None, None, None)
}

/// Builds the nested-prepositional-phrase P02 object alternative.
///
/// # Errors
///
/// Returns a declaration violation if the typed nested phrase fails P02's
/// construction constraints.
pub fn build_prepositional_object_prepositional_phrase(
    value: PrepositionalPhrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, Some(value), None, None)
}

/// Builds the gerund-clause P02 object alternative.
///
/// # Errors
///
/// Returns a declaration violation if `value` fails the gerund-clause object
/// constraints.
pub fn build_prepositional_object_gerund_clause(
    value: GerundClause,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, None, Some(value), None)
}

/// Builds the adverb P02 object alternative.
///
/// # Errors
///
/// Returns a declaration violation if `value` fails the adverb object
/// constraints.
pub fn build_prepositional_object_adverb(
    value: Vocab,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, None, None, Some(value))
}

/// Builds a simple P02 prepositional phrase.
///
/// # Errors
///
/// Returns a declaration violation if `object` fails the simple-phrase
/// constraints.
pub fn build_prepositional_phrase(
    preposition: Preposition,
    object: PrepositionalObject,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_phrase(preposition, object)
}

/// Builds a complete C01 sibling coordination from checked simple P02 members.
///
/// # Errors
///
/// Returns a declaration violation if any member is already coordinated, if
/// there is no remaining member, if a nonfinal member has a conjunction, or
/// if the final conjunction is not nominal.
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
