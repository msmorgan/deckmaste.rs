//! Checked public construction API for phrase coordination.
//!
//! The incremental list constructions remain internal: public callers provide
//! complete semantic members, and these helpers choose the binary or Oxford
//! declaration from arity while preserving each member's typed role.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::features::Conjunction;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AdjectivePhraseCoordination;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedModifier;
use crate::syntax::KeywordAbility;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::Polarity;
use crate::syntax::PowerToughness;
use crate::syntax::QuotedAbility;
pub use crate::syntax::WithAttributeCoordination;
pub use crate::syntax::WithAttributeList;
pub use crate::syntax::WithAttributeMember;
use crate::word::NounInstance;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

/// Builds an adjective member of an attributive modifier coordination.
///
/// # Errors
///
/// Returns a declaration violation when the adjective is not admitted in the
/// attributive member role.
pub fn build_modifier_adjective(
    adjective: AdjectivePhrase,
) -> Result<NominalModifier, DeclarationViolation> {
    crate::constructions::coordination::build_modifier_conjunct_adjective(adjective)
}

/// Builds a noun member of an attributive modifier coordination.
///
/// # Errors
///
/// Returns a declaration violation when the noun is not admitted in the
/// attributive member role.
pub fn build_modifier_noun(noun: NounInstance) -> Result<NominalModifier, DeclarationViolation> {
    crate::constructions::coordination::build_modifier_conjunct_noun(noun)
}

/// Validates a negatively polarized adjective or noun coordination member.
///
/// # Errors
///
/// Returns a declaration violation when `modifier` is not a negative lexical
/// adjective or noun member.
pub fn build_modifier_negated(
    modifier: NominalModifier,
) -> Result<NominalModifier, DeclarationViolation> {
    crate::constructions::coordination::build_modifier_conjunct_negated(modifier)
}

fn checked_modifier_member(
    modifier: NominalModifier,
) -> Result<NominalModifier, DeclarationViolation> {
    match modifier {
        NominalModifier::Adjective {
            polarity: Polarity::Positive,
            phrase,
        } => build_modifier_adjective(phrase),
        NominalModifier::Noun {
            polarity: Polarity::Positive,
            noun,
        } => build_modifier_noun(noun),
        modifier @ (NominalModifier::Adjective {
            polarity: Polarity::Negative,
            ..
        }
        | NominalModifier::Noun {
            polarity: Polarity::Negative,
            ..
        }) => build_modifier_negated(modifier),
        _ => Err(violation(
            "coordinated_modifier",
            "every member is an attributive adjective or noun",
        )),
    }
}

/// Builds a complete attributive modifier coordination.
///
/// `middle` contains the comma-separated members between `first` and `last`.
/// An empty `middle` selects the binary form; a nonempty `middle` selects the
/// Oxford form. The punctuation is therefore derived from arity.
///
/// # Errors
///
/// Returns a declaration violation when a member cannot fill the attributive
/// role or `conjunction` is not an admitted coordinating conjunction.
pub fn build_coordinated_modifier(
    first: NominalModifier,
    middle: Vec<NominalModifier>,
    conjunction: Conjunction,
    final_modifier: NominalModifier,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    let first = checked_modifier_member(first)?;
    let final_modifier = checked_modifier_member(final_modifier)?;
    let oxford = !middle.is_empty();
    let mut prefix = crate::constructions::coordination::build_modifier_list_single(first)?;
    for modifier in middle {
        prefix = crate::constructions::coordination::build_modifier_list_comma(
            prefix,
            checked_modifier_member(modifier)?,
        )?;
    }
    if oxford {
        crate::constructions::coordination::build_coordinated_modifier_oxford(
            prefix,
            conjunction,
            final_modifier,
        )
    } else {
        crate::constructions::coordination::build_coordinated_modifier_conjoined(
            prefix,
            conjunction,
            final_modifier,
        )
    }
}

/// Attaches one checked coordinated modifier beneath a nominal's shared head.
///
/// # Errors
///
/// Returns a declaration violation when the nominal's modifier phase is
/// already closed or a member repeats the overt grammatical head.
pub fn build_nominal_coordinated_modifier(
    coordinated: CoordinatedModifier,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::coordination::build_nominal_coordinated_modifier(coordinated, nominal)
}

/// Builds a complete predicative adjective coordination.
///
/// # Errors
///
/// Returns a declaration violation when `conjunction` is not `and`, `or`, or
/// `and/or`.
pub fn build_coordinated_adjective_phrase(
    first: AdjectivePhrase,
    middle: Vec<AdjectivePhrase>,
    conjunction: Conjunction,
    last: AdjectivePhrase,
) -> Result<CoordinatedAdjectivePhrase, DeclarationViolation> {
    if !matches!(
        conjunction,
        Conjunction::And | Conjunction::Or | Conjunction::AndOr
    ) {
        return Err(violation(
            "verb_phrase_coordinated_adjective",
            "the final connective coordinates adjective phrases",
        ));
    }
    let mut rest = middle
        .into_iter()
        .map(|phrase| AdjectivePhraseCoordination::from_declaration_parts(None, phrase))
        .collect::<Vec<_>>();
    rest.push(AdjectivePhraseCoordination::from_declaration_parts(
        Some(conjunction),
        last,
    ));
    crate::constructions::coordination::build_coordinated_adjective_phrase(Box::new(first), rest)
}

/// Validates a keyword ability as one member of a mixed `with` list.
///
/// # Errors
///
/// Returns a declaration violation unless the member is a bare or counted
/// keyword ability admitted by the dedicated closed sum.
pub fn build_with_attribute_keyword(
    keyword: KeywordAbility,
) -> Result<WithAttributeMember, DeclarationViolation> {
    crate::constructions::coordination::build_with_attribute_member_keyword(keyword)
}

/// Builds the quoted-ability alternative of the mixed `with` member sum.
///
/// # Errors
///
/// Returns a declaration violation if the quoted ability fails the declared
/// member construction.
pub fn build_with_attribute_quoted(
    quoted: QuotedAbility,
) -> Result<WithAttributeMember, DeclarationViolation> {
    crate::constructions::coordination::build_with_attribute_member_quoted(quoted)
}

fn checked_with_attribute_member(
    member: WithAttributeMember,
) -> Result<WithAttributeMember, DeclarationViolation> {
    match member {
        WithAttributeMember::Keyword(keyword) => build_with_attribute_keyword(keyword),
        WithAttributeMember::Quoted(quoted) => build_with_attribute_quoted(quoted),
    }
}

/// Builds a complete, genuinely mixed `with` attribute list.
///
/// `middle` contains the comma-separated members between `first` and `last`.
/// Binary and Oxford punctuation are selected from arity and are not stored as
/// caller-controlled flags.
///
/// # Errors
///
/// Returns a declaration violation unless the list closes with `and` or `or`
/// and contains at least one keyword and one quoted-ability member.
pub fn build_with_attribute_list(
    first: WithAttributeMember,
    middle: Vec<WithAttributeMember>,
    conjunction: Conjunction,
    final_member: WithAttributeMember,
) -> Result<WithAttributeList, DeclarationViolation> {
    let oxford = !middle.is_empty();
    let mut prefix = crate::constructions::coordination::build_with_attribute_list_single(
        checked_with_attribute_member(first)?,
    )?;
    for member in middle {
        prefix = crate::constructions::coordination::build_with_attribute_list_comma(
            prefix,
            checked_with_attribute_member(member)?,
        )?;
    }
    let final_member = checked_with_attribute_member(final_member)?;
    let closed = if oxford {
        crate::constructions::coordination::build_with_attribute_list_oxford(
            prefix,
            conjunction,
            final_member,
        )?
    } else {
        crate::constructions::coordination::build_with_attribute_list_conjoined(
            prefix,
            conjunction,
            final_member,
        )?
    };
    let has_keyword = std::iter::once(closed.first())
        .chain(closed.rest().iter().map(WithAttributeCoordination::member))
        .any(|member| matches!(member, WithAttributeMember::Keyword(_)));
    let has_quoted = std::iter::once(closed.first())
        .chain(closed.rest().iter().map(WithAttributeCoordination::member))
        .any(|member| matches!(member, WithAttributeMember::Quoted(_)));
    if has_keyword && has_quoted {
        Ok(closed)
    } else {
        Err(violation(
            "with_attribute_list",
            "the closed attribute list contains both member variants",
        ))
    }
}

/// Attaches a complete mixed `with` attribute list to its nominal owner.
///
/// # Errors
///
/// Returns a declaration violation unless the list is closed, genuinely
/// mixed, and the nominal is still in its general-complement phase.
pub fn build_nominal_with_attributes(
    nominal: NominalPhrase,
    attributes: WithAttributeList,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::coordination::build_nominal_with_attributes(nominal, attributes)
}

/// Attaches a power/toughness value after a characteristic nominal.
///
/// # Errors
///
/// Returns a declaration violation unless the nominal is a supported `power`
/// or `toughness` characteristic in the required attachment phase.
pub fn build_nominal_power_toughness_complement(
    nominal: NominalPhrase,
    stats: PowerToughness,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::coordination::build_nominal_power_toughness_complement(nominal, stats)
}
