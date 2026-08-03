use serde::Serialize;
use serde::Serializer;

use crate::features::Conjunction;

struct PredicateConjunction(Conjunction);

impl Serialize for PredicateConjunction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (index, variant) = match self.0 {
            Conjunction::And => (0, "And"),
            Conjunction::Or => (1, "Or"),
            Conjunction::Then => (2, "Then"),
            Conjunction::AndOr => (3, "AndOr"),
            Conjunction::Plus => (4, "Plus"),
        };
        serializer.serialize_unit_variant("PredicateConjunction", index, variant)
    }
}

struct NounPhraseConjunction(Conjunction);

impl Serialize for NounPhraseConjunction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (index, variant) = match self.0 {
            Conjunction::And => (0, "And"),
            Conjunction::Or => (1, "Or"),
            Conjunction::Plus => (2, "Plus"),
            Conjunction::AndOr => (3, "AndOr"),
            Conjunction::Then => (4, "Then"),
        };
        serializer.serialize_unit_variant("NounPhraseConjunction", index, variant)
    }
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde serialize_with callbacks receive the field by reference"
)]
pub(super) fn serialize_predicate_conjunction<S: Serializer>(
    conjunction: &Conjunction,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    PredicateConjunction(*conjunction).serialize(serializer)
}

#[expect(
    clippy::ref_option,
    clippy::trivially_copy_pass_by_ref,
    reason = "serde serialize_with callbacks receive the Option field by reference"
)]
pub(super) fn serialize_optional_predicate_conjunction<S: Serializer>(
    conjunction: &Option<Conjunction>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match conjunction {
        Some(conjunction) => serializer.serialize_some(&PredicateConjunction(*conjunction)),
        None => serializer.serialize_none(),
    }
}

#[expect(
    clippy::ref_option,
    clippy::trivially_copy_pass_by_ref,
    reason = "serde serialize_with callbacks receive the Option field by reference"
)]
pub(super) fn serialize_optional_noun_phrase_conjunction<S: Serializer>(
    conjunction: &Option<Conjunction>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match conjunction {
        Some(conjunction) => serializer.serialize_some(&NounPhraseConjunction(*conjunction)),
        None => serializer.serialize_none(),
    }
}
