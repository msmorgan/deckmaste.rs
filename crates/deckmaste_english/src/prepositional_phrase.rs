//! Checked prepositional-phrase construction, semantic inspection, and
//! rendering.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
use crate::features::Comma;
use crate::features::Conjunction;
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

/// Builds the noun-phrase object alternative.
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

/// Builds the nested-prepositional-phrase object alternative.
///
/// # Errors
///
/// Returns a declaration violation if the typed nested phrase fails the
/// construction constraints.
pub fn build_prepositional_object_prepositional_phrase(
    value: PrepositionalPhrase,
) -> Result<PrepositionalObject, DeclarationViolation> {
    crate::constructions::prepositional::build_prepositional_object(None, Some(value), None, None)
}

/// Builds the gerund-clause object alternative.
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

/// Builds the adverb object alternative.
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

/// Builds a simple prepositional phrase.
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

/// Builds a complete sibling coordination from checked simple phrases.
///
/// The ergonomic tuple representation is folded through the generated list
/// and coordination builders, so this facade has the same semantic checks as
/// parser lowering and projection.
///
/// # Errors
///
/// Returns a declaration violation if any member is already coordinated, if
/// there is no remaining member, if a nonfinal member has a conjunction, or
/// if the final conjunction is not nominal.
pub fn build_prepositional_phrase_coordination(
    first: PrepositionalPhrase,
    mut rest: Vec<(Option<Conjunction>, PrepositionalPhrase)>,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    let Some((conjunction, final_phrase)) = rest.pop() else {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the coordination has a remaining member",
        ));
    };
    let Some(conjunction) = conjunction else {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the final member carries a conjunction",
        ));
    };

    if rest.is_empty() {
        return crate::constructions::coordination::build_prepositional_phrase_sibling_coordinated(
            Some(first),
            None,
            None,
            conjunction,
            final_phrase,
        );
    }

    let mut members = rest.into_iter();
    let Some((second_conjunction, second)) = members.next() else {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "a longer coordination has an open-list member",
        ));
    };
    if second_conjunction.is_some() {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "nonfinal members do not carry conjunctions",
        ));
    }
    let mut list =
        crate::constructions::coordination::build_prepositional_phrase_list_pair(first, second)?;
    for (member_conjunction, member) in members {
        if member_conjunction.is_some() {
            return Err(violation(
                "prepositional_phrase_sibling_coordinated",
                "nonfinal members do not carry conjunctions",
            ));
        }
        list = crate::constructions::coordination::build_prepositional_phrase_list_comma(
            list, member,
        )?;
    }
    crate::constructions::coordination::build_prepositional_phrase_sibling_coordinated(
        None,
        Some(list),
        Some(Comma::Present),
        conjunction,
        final_phrase,
    )
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
