//! Compiler-derived production adjective declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Onset as InitialSound;
use crate::grammar::AdjectiveComparisonState;
use crate::grammar::Features;
use crate::grammar::adjective_comparison_state;
use crate::grammar::adjective_features;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::ComparisonComplement;
use crate::syntax::ComparisonMarker;
use crate::syntax::InfinitiveClause;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::Phrase;
use crate::syntax::PrepositionalPhrase;
use crate::word::Adjective;
use crate::word::AdjectiveComparisonClass;
use crate::word::CardOrientation;
use crate::word::Vocabulary;

type ComparisonAdjectivePhrase = AdjectivePhrase;
type ComparisonStandard = Phrase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdjectiveFeatureProjection {
    pub(crate) onset: InitialSound,
    pub(crate) comparison: AdjectiveComparisonState,
    pub(crate) card_orientation: bool,
    pub(crate) infinitive_complement: bool,
    pub(crate) predicative_only: bool,
}

fn feature_projection(features: &Features) -> Option<AdjectiveFeatureProjection> {
    let Features::Adjective {
        initial_sound,
        comparison,
        card_orientation,
        infinitive_complement,
        ..
    } = features
    else {
        return None;
    };
    Some(AdjectiveFeatureProjection {
        onset: *initial_sound,
        comparison: *comparison,
        card_orientation: *card_orientation,
        infinitive_complement: *infinitive_complement,
        predicative_only: matches!(comparison, AdjectiveComparisonState::Measured),
    })
}

fn apply_feature_projection(
    source: &Features,
    projection: AdjectiveFeatureProjection,
) -> Option<Features> {
    let Features::Adjective {
        past_participle,
        demonstrative_shared_determiner,
        ..
    } = source
    else {
        return None;
    };
    if projection.predicative_only
        != matches!(projection.comparison, AdjectiveComparisonState::Measured)
    {
        return None;
    }
    Some(Features::Adjective {
        initial_sound: projection.onset,
        comparison: projection.comparison,
        card_orientation: projection.card_orientation,
        infinitive_complement: projection.infinitive_complement,
        past_participle: *past_participle,
        demonstrative_shared_determiner: *demonstrative_shared_determiner,
    })
}

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_adjective(identity: Adjective) -> Result<Adjective, DeclarationViolation> {
    if !is_lexical_adjective(&identity) {
        return Err(violation(
            "adjective",
            "identity is a single lexical adjective",
        ));
    }
    Ok(identity)
}

fn adjective_identity(value: &Adjective) -> Adjective {
    value.clone()
}

fn is_lexical_adjective(value: &Adjective) -> bool {
    !matches!(value, Adjective::CardOrientation(_))
        && Vocabulary::new().render_adjective(value).is_some()
}

fn reduce_lexical_features(identity: &Features) -> Option<Features> {
    let projection = feature_projection(identity)?;
    (!projection.card_orientation).then(|| identity.clone())
}

fn make_adjective_phrase(head: Adjective) -> Result<AdjectivePhrase, DeclarationViolation> {
    if !is_lexical_adjective(&head) {
        return Err(violation(
            "adjective_phrase",
            "head is a single lexical adjective",
        ));
    }
    AdjectivePhrase::try_from_lexical_head(head)
        .ok_or_else(|| violation("adjective_phrase", "head is a single lexical adjective"))
}

fn adjective_phrase_head(value: &AdjectivePhrase) -> Adjective {
    value.head().clone()
}

fn is_base_adjective_phrase(value: &AdjectivePhrase) -> bool {
    value.degree().is_none() && value.complements().is_empty() && is_lexical_adjective(value.head())
}

fn make_adjective_phrase_prepositional(
    owner: AdjectivePhrase,
    preposition: PrepositionalPhrase,
) -> Result<AdjectivePhrase, DeclarationViolation> {
    owner
        .try_attach_declared_prepositional(preposition)
        .ok_or_else(|| {
            violation(
                "adjective_phrase_prepositional",
                "the owner is a lexical adjective phrase with only declared post-head complements",
            )
        })
}

fn adjective_phrase_prepositional_parts(
    value: &AdjectivePhrase,
) -> (AdjectivePhrase, PrepositionalPhrase) {
    value
        .clone()
        .try_split_declared_prepositional()
        .expect("the prepositional dispatcher admits one checked trailing complement")
}

fn is_adjective_phrase_prepositional(value: &AdjectivePhrase) -> bool {
    value.clone().try_split_declared_prepositional().is_some()
}

fn make_adjective_phrase_infinitive(
    owner: AdjectivePhrase,
    infinitive: InfinitiveClause,
) -> Result<AdjectivePhrase, DeclarationViolation> {
    owner
        .try_attach_declared_infinitive(infinitive)
        .ok_or_else(|| {
            violation(
                "adjective_phrase_infinitive",
                "the owner is a lexical adjective phrase with only declared post-head complements",
            )
        })
}

fn adjective_phrase_infinitive_parts(
    value: &AdjectivePhrase,
) -> (AdjectivePhrase, InfinitiveClause) {
    value
        .clone()
        .try_split_declared_infinitive()
        .expect("the infinitive dispatcher admits one checked trailing complement")
}

fn is_adjective_phrase_infinitive(value: &AdjectivePhrase) -> bool {
    value.clone().try_split_declared_infinitive().is_some()
}

fn reduce_posthead_features(owner: &Features, complement: &Features) -> Option<Features> {
    let projection = feature_projection(owner)?;
    if projection.card_orientation
        || matches!(
            projection.comparison,
            AdjectiveComparisonState::Complete | AdjectiveComparisonState::Measured
        )
        || !matches!(
            complement,
            Features::PrepositionalPhrase { .. } | Features::InfinitiveClause
        )
        || matches!(complement, Features::InfinitiveClause) && !projection.infinitive_complement
    {
        return None;
    }
    Some(owner.clone())
}

fn orientation_phrase(orientation: CardOrientation) -> AdjectivePhrase {
    AdjectivePhrase::from_orientation(orientation)
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "declaration bind adapters share one Result-returning checked-builder contract"
)]
fn make_face_up() -> Result<AdjectivePhrase, DeclarationViolation> {
    Ok(orientation_phrase(CardOrientation::FaceUp))
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "declaration bind adapters share one Result-returning checked-builder contract"
)]
fn make_face_down() -> Result<AdjectivePhrase, DeclarationViolation> {
    Ok(orientation_phrase(CardOrientation::FaceDown))
}

fn no_parts(_value: &AdjectivePhrase) {}

fn is_face_up(value: &AdjectivePhrase) -> bool {
    value == &orientation_phrase(CardOrientation::FaceUp)
}

fn is_face_down(value: &AdjectivePhrase) -> bool {
    value == &orientation_phrase(CardOrientation::FaceDown)
}

fn orientation_features(orientation: CardOrientation) -> Option<Features> {
    let head = Adjective::CardOrientation(orientation);
    adjective_features(&head, true)
}

fn face_up_features() -> Option<Features> {
    orientation_features(CardOrientation::FaceUp)
}

fn face_down_features() -> Option<Features> {
    orientation_features(CardOrientation::FaceDown)
}

fn make_comparison_standard(
    noun_phrase: Option<NounPhrase>,
    adjective_phrase: Option<AdjectivePhrase>,
    clause: Option<Clause>,
) -> Result<Phrase, DeclarationViolation> {
    match (noun_phrase, adjective_phrase, clause) {
        (Some(value), None, None) => Ok(Phrase::NounPhrase(Box::new(value))),
        (None, Some(value), None) => Ok(Phrase::AdjectivePhrase(Box::new(value))),
        (None, None, Some(value)) => Ok(Phrase::Clause(Box::new(value))),
        _ => Err(violation(
            "comparison_standard",
            "exactly one typed comparison standard is present",
        )),
    }
}

fn comparison_standard_parts(
    value: &Phrase,
) -> (Option<NounPhrase>, Option<AdjectivePhrase>, Option<Clause>) {
    match value {
        Phrase::NounPhrase(value) => (Some(value.as_ref().clone()), None, None),
        Phrase::AdjectivePhrase(value) => (None, Some(value.as_ref().clone()), None),
        Phrase::Clause(value) => (None, None, Some(value.as_ref().clone())),
        _ => (None, None, None),
    }
}

fn is_noun_phrase_standard(value: &Phrase) -> bool {
    matches!(value, Phrase::NounPhrase(_))
}

fn is_adjective_phrase_standard(value: &Phrase) -> bool {
    matches!(value, Phrase::AdjectivePhrase(_))
}

fn is_clause_standard(value: &Phrase) -> bool {
    matches!(value, Phrase::Clause(_))
}

fn is_comparison_standard(value: &Phrase) -> bool {
    is_noun_phrase_standard(value)
        || is_adjective_phrase_standard(value)
        || is_clause_standard(value)
}

fn make_comparison_than(standard: Phrase) -> Result<ComparisonComplement, DeclarationViolation> {
    if !is_comparison_standard(&standard) {
        return Err(violation(
            "comparison_than",
            "standard is a noun phrase, adjective phrase, or clause",
        ));
    }
    ComparisonComplement::try_new(ComparisonMarker::Than, standard).ok_or_else(|| {
        violation(
            "comparison_than",
            "standard is a noun phrase, adjective phrase, or clause",
        )
    })
}

fn make_comparison_than_or_equal_to(
    standard: Phrase,
) -> Result<ComparisonComplement, DeclarationViolation> {
    if !is_comparison_standard(&standard) {
        return Err(violation(
            "comparison_than_or_equal_to",
            "standard is a noun phrase, adjective phrase, or clause",
        ));
    }
    ComparisonComplement::try_new(ComparisonMarker::ThanOrEqualTo, standard).ok_or_else(|| {
        violation(
            "comparison_than_or_equal_to",
            "standard is a noun phrase, adjective phrase, or clause",
        )
    })
}

fn comparison_standard(value: &ComparisonComplement) -> Phrase {
    value.standard().clone()
}

fn is_comparison_than(value: &ComparisonComplement) -> bool {
    value.marker() == ComparisonMarker::Than
}

fn is_comparison_than_or_equal_to(value: &ComparisonComplement) -> bool {
    value.marker() == ComparisonMarker::ThanOrEqualTo
}

fn pending_comparison_owner(value: &AdjectivePhrase) -> bool {
    value.degree().is_none()
        && value.complements().is_empty()
        && matches!(
            adjective_comparison_state(value.head()),
            AdjectiveComparisonState::Pending(_)
        )
}

fn comparison_class_matches(owner: &AdjectivePhrase, comparison: &ComparisonComplement) -> bool {
    matches!(
        (
            adjective_comparison_state(owner.head()),
            comparison.marker()
        ),
        (
            AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative),
            ComparisonMarker::Than | ComparisonMarker::ThanOrEqualTo,
        ) | (
            AdjectiveComparisonState::Pending(AdjectiveComparisonClass::ThanOnly),
            ComparisonMarker::Than,
        )
    )
}

fn make_adjective_phrase_comparison(
    owner: AdjectivePhrase,
    comparison: ComparisonComplement,
) -> Result<AdjectivePhrase, DeclarationViolation> {
    if !pending_comparison_owner(&owner) || !comparison_class_matches(&owner, &comparison) {
        return Err(violation(
            "adjective_phrase_comparison",
            "the owner is an unmeasured matching-class comparison-pending adjective phrase with no complements",
        ));
    }
    if !is_comparison_standard(comparison.standard()) {
        return Err(violation(
            "adjective_phrase_comparison",
            "the comparison has a typed standard",
        ));
    }
    owner
        .try_attach_declared_comparison(comparison)
        .ok_or_else(|| {
            violation(
                "adjective_phrase_comparison",
                "the checked adjective owner accepts the typed comparison exactly once",
            )
        })
}

fn adjective_phrase_comparison_parts(
    value: &AdjectivePhrase,
) -> (AdjectivePhrase, ComparisonComplement) {
    value
        .clone()
        .try_split_declared_comparison()
        .expect("the comparison dispatcher admits one checked comparison")
}

fn is_adjective_phrase_comparison(value: &AdjectivePhrase) -> bool {
    value.clone().try_split_declared_comparison().is_some()
}

fn reduce_comparison_features(owner: &Features, complement: &Features) -> Option<Features> {
    let mut projection = feature_projection(owner)?;
    if !matches!(projection.comparison, AdjectiveComparisonState::Pending(_))
        || projection.card_orientation
        || projection.predicative_only
        || !matches!(complement, Features::None)
    {
        return None;
    }
    projection.comparison = AdjectiveComparisonState::Complete;
    apply_feature_projection(owner, projection)
}

fn make_degree_measure(
    measure: NumberLiteral,
    adjective: Adjective,
) -> Result<AdjectivePhrase, DeclarationViolation> {
    if !crate::syntax::is_valid_degree_measure_number(measure) {
        return Err(violation(
            "adjective_phrase_degree_measure",
            "the measure uses cardinal or ungrouped Arabic notation",
        ));
    }
    let Some(features) = adjective_features(
        &adjective,
        matches!(adjective, Adjective::CardOrientation(_)),
    ) else {
        return Err(violation(
            "adjective_phrase_degree_measure",
            "the adjective has vocabulary-backed features",
        ));
    };
    let Some(projection) = feature_projection(&features) else {
        return Err(violation(
            "adjective_phrase_degree_measure",
            "the adjective has adjective features",
        ));
    };
    if projection.card_orientation
        || projection.predicative_only
        || projection.comparison
            != AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative)
    {
        return Err(violation(
            "adjective_phrase_degree_measure",
            "the adjective is a pending OrComparative and is not a card orientation",
        ));
    }
    AdjectivePhrase::try_from_degree_measure(measure, adjective).ok_or_else(|| {
        violation(
            "adjective_phrase_degree_measure",
            "the checked adjective owner accepts this degree measure",
        )
    })
}

fn degree_measure_parts(value: &AdjectivePhrase) -> (NumberLiteral, Adjective) {
    let measure = value
        .degree()
        .copied()
        .expect("the degree-measure dispatcher admits only measured phrases");
    assert!(
        crate::syntax::is_valid_degree_measure_number(measure),
        "the degree-measure dispatcher admits only cardinal or ungrouped Arabic notation",
    );
    (measure, value.head().clone())
}

fn is_adjective_phrase_degree_measure(value: &AdjectivePhrase) -> bool {
    let Some(measure) = value.degree().copied() else {
        return false;
    };
    value.complements().is_empty() && make_degree_measure(measure, value.head().clone()).is_ok()
}

fn reduce_degree_measure_features(measure: &Features, adjective: &Features) -> Option<Features> {
    if !matches!(measure, Features::Number { .. }) {
        return None;
    }
    let mut projection = feature_projection(adjective)?;
    if projection.card_orientation
        || projection.predicative_only
        || projection.comparison
            != AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative)
    {
        return None;
    }
    projection.comparison = AdjectiveComparisonState::Measured;
    projection.predicative_only = true;
    apply_feature_projection(adjective, projection)
}

deckmaste_constructions_macro::constructions! {
    group adjective;

    construction adjective: Adjective {
        bind Adjective via make_adjective, adjective_identity {
            identity: identity Adjective via Adjective,
        }
        derive features: Features = reduce_lexical_features(identity);
        form only @ 0 inverse check(is_lexical_adjective) = identity(identity);
        selection unique;
    }

    construction adjective_phrase: AdjectivePhrase {
        bind AdjectivePhrase via make_adjective_phrase, adjective_phrase_head {
            head: hole Adjective,
        }
        derive features: Features = reduce_lexical_features(head);
        form only @ 0 inverse check(is_base_adjective_phrase) = head;
        selection unique;
    }

    construction adjective_phrase_face_up: AdjectivePhrase {
        bind AdjectivePhrase via make_face_up, no_parts {}
        derive features: Features = face_up_features();
        form only @ 0 inverse check(is_face_up) = "face" "up";
        selection unique;
    }

    construction adjective_phrase_face_down: AdjectivePhrase {
        bind AdjectivePhrase via make_face_down, no_parts {}
        derive features: Features = face_down_features();
        form only @ 0 inverse check(is_face_down) = "face" "down";
        selection unique;
    }

    internal construction comparison_standard: ComparisonStandard {
        bind Phrase via make_comparison_standard, comparison_standard_parts {
            noun_phrase: opt hole NounPhrase,
            adjective_phrase: opt hole ComparisonAdjectivePhrase,
            clause: opt hole Clause,
        }
        require any(
            all(noun_phrase.is_some(), adjective_phrase.is_none(), clause.is_none()),
            all(noun_phrase.is_none(), adjective_phrase.is_some(), clause.is_none()),
            all(noun_phrase.is_none(), adjective_phrase.is_none(), clause.is_some())
        );
        form noun_phrase @ 0 when all(noun_phrase.is_some(), adjective_phrase.is_none(), clause.is_none()) inverse check(is_noun_phrase_standard) = noun_phrase;
        form adjective_phrase @ 1 when all(noun_phrase.is_none(), adjective_phrase.is_some(), clause.is_none()) inverse check(is_adjective_phrase_standard) = adjective_phrase;
        form clause @ 2 when all(noun_phrase.is_none(), adjective_phrase.is_none(), clause.is_some()) inverse check(is_clause_standard) = clause;
        selection unique;
    }

    construction comparison_than: ComparisonComplement {
        bind ComparisonComplement via make_comparison_than, comparison_standard {
            standard: hole ComparisonStandard,
        }
        form only @ 0 inverse check(is_comparison_than) = "than" standard;
        selection unique;
    }

    construction comparison_than_or_equal_to: ComparisonComplement {
        bind ComparisonComplement via make_comparison_than_or_equal_to, comparison_standard {
            standard: hole ComparisonStandard,
        }
        form only @ 0 inverse check(is_comparison_than_or_equal_to) = "than" "or" "equal" "to" standard;
        selection unique;
    }

    construction adjective_phrase_comparison: AdjectivePhrase {
        bind AdjectivePhrase via make_adjective_phrase_comparison, adjective_phrase_comparison_parts {
            owner: hole AdjectivePhrase,
            comparison: hole ComparisonComplement,
        }
        derive features: Features = reduce_comparison_features(owner, comparison);
        form only @ 0 inverse check(is_adjective_phrase_comparison) = owner comparison;
        selection unique;
    }

    construction adjective_phrase_degree_measure: AdjectivePhrase {
        bind AdjectivePhrase via make_degree_measure, degree_measure_parts {
            measure: lex NumberLiteral via DegreeMeasureNumeral,
            adjective: hole Adjective,
        }
        derive features: Features = reduce_degree_measure_features(measure, adjective);
        witness numeral = stored measure;
        form only @ 0 inverse check(is_adjective_phrase_degree_measure) = lex(measure) adjective;
        selection unique;
    }

    internal construction adjective_phrase_prepositional: AdjectivePhrase {
        bind AdjectivePhrase via make_adjective_phrase_prepositional, adjective_phrase_prepositional_parts {
            owner: hole AdjectivePhrase,
            preposition: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_posthead_features(owner, preposition);
        form only @ 0 inverse check(is_adjective_phrase_prepositional) = owner preposition;
        selection unique;
    }

    construction adjective_phrase_infinitive: AdjectivePhrase {
        bind AdjectivePhrase via make_adjective_phrase_infinitive, adjective_phrase_infinitive_parts {
            owner: hole AdjectivePhrase,
            infinitive: hole InfinitiveClause,
        }
        derive features: Features = reduce_posthead_features(owner, infinitive);
        form only @ 0 inverse check(is_adjective_phrase_infinitive) = owner infinitive;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&ADJECTIVE_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalogs;
    use crate::numeral::Numeral;
    use crate::syntax::AdjectiveComplement;
    use crate::syntax::AdjectivePhrase;
    use crate::syntax::NumberLiteral;
    use crate::word::Adjective;
    use crate::word::CardOrientation;
    use crate::word::ColorWord;
    use crate::word::Vocab;

    const BASE_IDS: &[&str] = &[
        "adjective",
        "adjective_phrase",
        "adjective_phrase_face_up",
        "adjective_phrase_face_down",
    ];

    const COMPARISON_IDS: &[&str] = &[
        "comparison_standard",
        "comparison_than",
        "comparison_than_or_equal_to",
    ];

    const ALL_IDS: &[&str] = &[
        "adjective",
        "adjective_phrase",
        "adjective_phrase_face_up",
        "adjective_phrase_face_down",
        "comparison_standard",
        "comparison_than",
        "comparison_than_or_equal_to",
        "adjective_phrase_comparison",
        "adjective_phrase_degree_measure",
        "adjective_phrase_prepositional",
        "adjective_phrase_infinitive",
    ];

    const WRONG_COMPARISON_CLASS: &str = "WRONG_COMPARISON_CLASS";
    const ALREADY_COMPLETE: &str = "ALREADY_COMPLETE";
    const CARD_ORIENTATION_MEASURE: &str = "CARD_ORIENTATION_MEASURE";
    const ATTRIBUTIVE_DEGREE_MEASURE: &str = "ATTRIBUTIVE_DEGREE_MEASURE";
    const DUPLICATE_COMPARISON: &str = "DUPLICATE_COMPARISON";
    const SURPLUS_COMPLEMENT: &str = "SURPLUS_COMPLEMENT";

    fn comparison(marker: ComparisonMarker) -> ComparisonComplement {
        ComparisonComplement::try_new(
            marker,
            Phrase::AdjectivePhrase(Box::new(
                AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap(),
            )),
        )
        .unwrap()
    }

    fn rebuild_adjective(value: Adjective) -> Adjective {
        let built = build_adjective(value).expect("the lexical adjective is admitted");
        let identity = parts_adjective(&built);
        build_adjective(identity).expect("the lexical adjective rebuilds")
    }

    fn rebuild_phrase(
        expected: &AdjectivePhrase,
        parts: fn(&AdjectivePhrase),
        build_phrase: fn() -> Result<
            AdjectivePhrase,
            deckmaste_construction_compiler::runtime::DeclarationViolation,
        >,
    ) -> AdjectivePhrase {
        let phrase = build_phrase().expect("the orientation phrase is admitted");
        assert_eq!(&phrase, expected);
        parts(&phrase);
        build_phrase().expect("the orientation phrase rebuilds")
    }

    #[test]
    fn base_declarations_have_the_exact_census_and_checked_round_trip_laws() {
        let actual = GROUPS[0]
            .constructions
            .iter()
            .take(BASE_IDS.len())
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(actual, BASE_IDS);

        let lexical = Adjective::Word(Vocab::Target);
        assert_eq!(rebuild_adjective(lexical.clone()), lexical);

        let phrase = build_adjective_phrase(lexical).expect("a lexical head forms a base phrase");
        let head = parts_adjective_phrase(&phrase);
        assert_eq!(
            build_adjective_phrase(head).expect("the base phrase rebuilds"),
            phrase,
        );

        for (orientation, parts, build) in [
            (
                CardOrientation::FaceUp,
                parts_adjective_phrase_face_up as fn(&AdjectivePhrase),
                build_adjective_phrase_face_up as fn() -> Result<_, _>,
            ),
            (
                CardOrientation::FaceDown,
                parts_adjective_phrase_face_down,
                build_adjective_phrase_face_down,
            ),
        ] {
            let expected = AdjectivePhrase::from_orientation(orientation);
            assert_eq!(rebuild_phrase(&expected, parts, build), expected);
        }

        assert!(matches!(
            build_adjective_phrase(Adjective::CardOrientation(CardOrientation::FaceUp)),
            Err(deckmaste_construction_compiler::runtime::DeclarationViolation { .. })
        ));
    }

    #[test]
    fn comparison_standard_sum_and_marker_laws_cover_every_variant() {
        let actual = GROUPS[0]
            .constructions
            .iter()
            .skip(BASE_IDS.len())
            .take(COMPARISON_IDS.len())
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(actual, COMPARISON_IDS);

        let noun = crate::syntax::NounPhrase::from_this_card_declaration(
            crate::syntax::ThisCardForm::AbbreviatedName,
        );
        let adjective =
            AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap();
        let clause = crate::grammar::parse_nonterminal_with_activation(
            "copy that spell",
            &Catalogs::default(),
            crate::grammar::Nonterminal::Clause,
            crate::grammar::GeneratedActivation::Production,
        )
        .expect("the production grammar supplies a representative clause")
        .clause()
        .expect("the representative lowers as a clause")
        .clone();

        for (expected, noun, adjective, clause) in [
            (
                crate::syntax::Phrase::NounPhrase(Box::new(noun.clone())),
                Some(noun),
                None,
                None,
            ),
            (
                crate::syntax::Phrase::AdjectivePhrase(Box::new(adjective.clone())),
                None,
                Some(adjective),
                None,
            ),
            (
                crate::syntax::Phrase::Clause(Box::new(clause.clone())),
                None,
                None,
                Some(clause),
            ),
        ] {
            let built = build_comparison_standard(noun, adjective, clause)
                .expect("one typed comparison standard is admitted");
            assert_eq!(built, expected);
            let (noun, adjective, clause) = parts_comparison_standard(&built);
            assert_eq!(
                build_comparison_standard(noun, adjective, clause)
                    .expect("the typed comparison standard rebuilds"),
                expected,
            );
        }

        let standard = crate::syntax::Phrase::AdjectivePhrase(Box::new(
            AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap(),
        ));
        for (expected_marker, build, parts) in [
            (
                crate::syntax::ComparisonMarker::Than,
                build_comparison_than as fn(crate::syntax::Phrase) -> Result<_, _>,
                parts_comparison_than as fn(&crate::syntax::ComparisonComplement) -> _,
            ),
            (
                crate::syntax::ComparisonMarker::ThanOrEqualTo,
                build_comparison_than_or_equal_to,
                parts_comparison_than_or_equal_to,
            ),
        ] {
            let built = build(standard.clone()).expect("the marker admits a typed standard");
            assert_eq!(built.marker(), expected_marker);
            assert_eq!(built.standard(), &standard);
            let standard = parts(&built);
            assert_eq!(build(standard).expect("the marker rebuilds"), built);
        }

        let unsupported =
            crate::syntax::Phrase::Quantity(crate::syntax::Quantity::unchecked_both());
        let (noun, adjective, clause) = parts_comparison_standard(&unsupported);
        assert!(build_comparison_standard(noun, adjective, clause).is_err());
        assert!(build_comparison_than(unsupported.clone()).is_err());
        assert!(build_comparison_than_or_equal_to(unsupported).is_err());
    }

    #[test]
    fn declaration_lowers_and_matches_both_generic_adjective_categories() {
        let lexical = Adjective::Word(Vocab::Target);
        let phrase = AdjectivePhrase::try_from_lexical_head(lexical.clone()).unwrap();
        for (source, category, expected, construction) in [
            (
                "target",
                "Adjective",
                &lexical as &dyn std::any::Any,
                "adjective",
            ),
            (
                "target",
                "AdjectivePhrase",
                &phrase as &dyn std::any::Any,
                "adjective_phrase",
            ),
        ] {
            let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                source,
                &Catalogs::default(),
                category,
                expected,
                10_000,
                GROUPS,
            )
            .expect("the adjective declaration parses through its category adapters");
            for parses in orders {
                assert_eq!(
                    parses
                        .iter()
                        .map(|parse| parse.ast().construction)
                        .collect::<Vec<_>>(),
                    [construction],
                );
            }
        }

        for (source, orientation, construction) in [
            (
                "face up",
                CardOrientation::FaceUp,
                "adjective_phrase_face_up",
            ),
            (
                "face down",
                CardOrientation::FaceDown,
                "adjective_phrase_face_down",
            ),
        ] {
            let expected = orientation_phrase(orientation);
            let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                source,
                &Catalogs::default(),
                "AdjectivePhrase",
                &expected,
                10_000,
                crate::constructions::GROUPS,
            )
            .expect("the production orientation declaration parses");
            for parses in orders {
                assert_eq!(
                    parses
                        .iter()
                        .map(|parse| parse.ast().construction)
                        .collect::<Vec<_>>(),
                    [construction],
                );
            }
        }
    }

    #[test]
    fn all_eleven_rows_build_project_and_rebuild_their_exact_values() {
        let actual = GROUPS[0]
            .constructions
            .iter()
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(actual, ALL_IDS);

        for marker in [ComparisonMarker::Than, ComparisonMarker::ThanOrEqualTo] {
            let owner = build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap();
            let built = build_adjective_phrase_comparison(owner, comparison(marker)).unwrap();
            let (owner, complement) = parts_adjective_phrase_comparison(&built);
            assert_eq!(
                build_adjective_phrase_comparison(owner, complement).unwrap(),
                built
            );
        }

        for numeral in [Numeral::Arabic(false), Numeral::Cardinal] {
            let measure = NumberLiteral { value: 2, numeral };
            let built =
                build_adjective_phrase_degree_measure(measure, Adjective::Word(Vocab::Greater))
                    .unwrap();
            let (measure, adjective) = parts_adjective_phrase_degree_measure(&built);
            assert_eq!(
                build_adjective_phrase_degree_measure(measure, adjective).unwrap(),
                built
            );
        }

        assert_eq!(
            build_adjective_phrase_face_up().unwrap(),
            orientation_phrase(CardOrientation::FaceUp)
        );
        assert_eq!(
            build_adjective_phrase_face_down().unwrap(),
            orientation_phrase(CardOrientation::FaceDown)
        );

        let preposition = crate::constructions::prepositional::expect_prepositional_phrase(
            crate::syntax::Preposition::With,
            crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                crate::syntax::NounPhrase::from_this_card_declaration(
                    crate::syntax::ThisCardForm::FullName,
                ),
            )),
        );
        let prepositional = build_adjective_phrase_prepositional(
            build_adjective_phrase(Adjective::Word(Vocab::Able)).unwrap(),
            preposition,
        )
        .unwrap();
        let (owner, preposition) = parts_adjective_phrase_prepositional(&prepositional);
        assert_eq!(
            build_adjective_phrase_prepositional(owner, preposition).unwrap(),
            prepositional,
        );

        let infinitive = crate::grammar::parse_nonterminal_with_activation(
            "to attack",
            &Catalogs::default(),
            crate::grammar::Nonterminal::InfinitiveClause,
            crate::grammar::GeneratedActivation::Production,
        )
        .expect("the production grammar supplies a representative infinitive")
        .infinitive_clause()
        .expect("the representative lowers as an infinitive")
        .clone();
        let chain = build_adjective_phrase_infinitive(prepositional.clone(), infinitive).unwrap();
        let (owner, infinitive) = parts_adjective_phrase_infinitive(&chain);
        assert_eq!(owner, prepositional);
        assert_eq!(
            build_adjective_phrase_infinitive(owner, infinitive).unwrap(),
            chain,
        );
        assert_eq!(
            crate::renderer::render_generated_adjective_phrase_law(&chain)
                .unwrap()
                .forms,
            [
                ("adjective_phrase_infinitive", 0),
                ("adjective_phrase_prepositional", 0),
                ("adjective_phrase", 0),
                ("adjective", 0),
            ],
        );
    }

    #[test]
    fn infinitive_builder_requires_lexically_selected_adjective_complement() {
        let infinitive = crate::grammar::parse_nonterminal_with_activation(
            "to attack",
            &Catalogs::default(),
            crate::grammar::Nonterminal::InfinitiveClause,
            crate::grammar::GeneratedActivation::Production,
        )
        .expect("the production grammar supplies a representative infinitive")
        .infinitive_clause()
        .expect("the representative lowers as an infinitive")
        .clone();

        let able = build_adjective_phrase(Adjective::Word(Vocab::Able)).unwrap();
        assert!(build_adjective_phrase_infinitive(able, infinitive.clone()).is_ok());

        for adjective in [
            Adjective::Color(ColorWord::Red),
            Adjective::Word(Vocab::Target),
        ] {
            let owner = build_adjective_phrase(adjective).unwrap();
            assert!(build_adjective_phrase_infinitive(owner, infinitive.clone()).is_err());
        }
    }

    #[test]
    fn degree_measure_builder_rejects_non_degree_notations() {
        for measure in [
            NumberLiteral {
                value: 2,
                numeral: Numeral::Ordinal,
            },
            NumberLiteral {
                value: 10,
                numeral: Numeral::Roman,
            },
            NumberLiteral {
                value: 2_000,
                numeral: Numeral::Arabic(true),
            },
        ] {
            assert!(
                build_adjective_phrase_degree_measure(measure, Adjective::Word(Vocab::Greater))
                    .is_err(),
                "non-degree notation reached the checked builder: {measure:?}",
            );
        }
    }

    #[test]
    fn degree_measure_owner_rejects_non_degree_notations() {
        for measure in [
            NumberLiteral {
                value: 2,
                numeral: Numeral::Ordinal,
            },
            NumberLiteral {
                value: 10,
                numeral: Numeral::Roman,
            },
            NumberLiteral {
                value: 2_000,
                numeral: Numeral::Arabic(true),
            },
        ] {
            assert!(
                AdjectivePhrase::try_from_degree_measure(measure, Adjective::Word(Vocab::Greater),)
                    .is_none(),
                "non-degree notation reached owner storage: {measure:?}",
            );
        }
    }

    #[test]
    fn degree_measure_parser_rejects_non_degree_notations() {
        for source in ["second greater", "X greater", "2,000 greater"] {
            assert!(
                crate::grammar::parse_nonterminal_with_activation(
                    source,
                    &Catalogs::default(),
                    crate::grammar::Nonterminal::AdjectivePhrase,
                    crate::grammar::GeneratedActivation::Groups(GROUPS),
                )
                .is_err(),
                "non-degree notation parsed as a degree measure: {source:?}",
            );
        }
    }

    #[test]
    fn ungrouped_arabic_degree_measure_has_one_semantic_alternative() {
        let expected = AdjectivePhrase::try_from_degree_measure(
            NumberLiteral {
                value: 2,
                numeral: Numeral::Arabic(false),
            },
            Adjective::Word(Vocab::Greater),
        )
        .unwrap();
        let orders = crate::grammar::exact::parse_groups_as_adjective_phrase_values_in_both_orders(
            "2 greater",
            &Catalogs::default(),
            10_000,
            GROUPS,
        )
        .unwrap();
        for alternatives in orders {
            assert_eq!(alternatives.as_slice(), std::slice::from_ref(&expected));
        }
    }

    #[test]
    fn than_only_adjective_accepts_the_than_marker() {
        let owner = build_adjective_phrase(Adjective::Word(Vocab::Other)).unwrap();
        let comparison = comparison(ComparisonMarker::Than);
        let built = build_adjective_phrase_comparison(owner, comparison).unwrap();
        assert_eq!(built.head(), &Adjective::Word(Vocab::Other));
        let AdjectiveComplement::Comparison(comparison) = &built.complements()[0] else {
            panic!("the declared complement is a comparison")
        };
        assert_eq!(comparison.marker(), ComparisonMarker::Than);
    }

    #[test]
    fn comparison_and_measure_negative_matrix_names_every_domain_boundary() {
        let measure = NumberLiteral {
            value: 2,
            numeral: Numeral::Arabic(false),
        };
        assert!(
            build_adjective_phrase_degree_measure(measure, Adjective::Word(Vocab::Other)).is_err(),
            "{WRONG_COMPARISON_CLASS}"
        );
        assert!(
            build_adjective_phrase_comparison(
                build_adjective_phrase(Adjective::Word(Vocab::Other)).unwrap(),
                comparison(ComparisonMarker::ThanOrEqualTo),
            )
            .is_err(),
            "{WRONG_COMPARISON_CLASS}"
        );
        assert!(
            build_adjective_phrase_degree_measure(
                measure,
                Adjective::CardOrientation(CardOrientation::FaceUp),
            )
            .is_err(),
            "{CARD_ORIENTATION_MEASURE}"
        );

        let greater = build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap();
        let completed =
            build_adjective_phrase_comparison(greater.clone(), comparison(ComparisonMarker::Than))
                .unwrap();
        assert!(
            build_adjective_phrase_comparison(
                completed.clone(),
                comparison(ComparisonMarker::Than),
            )
            .is_err(),
            "{ALREADY_COMPLETE}"
        );
        assert!(
            build_adjective_phrase_comparison(
                completed,
                comparison(ComparisonMarker::ThanOrEqualTo),
            )
            .is_err(),
            "{DUPLICATE_COMPARISON}"
        );

        let surplus = greater
            .try_attach_postnominal_comparison(comparison(ComparisonMarker::Than))
            .expect("a pending owner accepts one checked postnominal comparison");
        assert!(
            build_adjective_phrase_comparison(surplus, comparison(ComparisonMarker::Than)).is_err(),
            "{SURPLUS_COMPLEMENT}"
        );

        assert!(
            !crate::grammar::reduction::nominal_attributive_adjective_is_admitted(false, true),
            "{ATTRIBUTIVE_DEGREE_MEASURE}"
        );
    }

    #[test]
    fn declaration_parses_all_required_comparison_and_measure_witnesses() {
        let target = Phrase::AdjectivePhrase(Box::new(
            AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Target)).unwrap(),
        ));
        for (source, expected) in [
            (
                "greater than target",
                AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Greater))
                    .unwrap()
                    .try_attach_declared_comparison(
                        ComparisonComplement::try_new(ComparisonMarker::Than, target.clone())
                            .unwrap(),
                    )
                    .unwrap(),
            ),
            (
                "greater than or equal to target",
                AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Greater))
                    .unwrap()
                    .try_attach_declared_comparison(
                        ComparisonComplement::try_new(ComparisonMarker::ThanOrEqualTo, target)
                            .unwrap(),
                    )
                    .unwrap(),
            ),
            (
                "2 greater",
                AdjectivePhrase::try_from_degree_measure(
                    NumberLiteral {
                        value: 2,
                        numeral: Numeral::Arabic(false),
                    },
                    Adjective::Word(Vocab::Greater),
                )
                .unwrap(),
            ),
            (
                "two greater",
                AdjectivePhrase::try_from_degree_measure(
                    NumberLiteral {
                        value: 2,
                        numeral: Numeral::Cardinal,
                    },
                    Adjective::Word(Vocab::Greater),
                )
                .unwrap(),
            ),
            ("face up", orientation_phrase(CardOrientation::FaceUp)),
            ("face down", orientation_phrase(CardOrientation::FaceDown)),
        ] {
            let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                source,
                &Catalogs::default(),
                "AdjectivePhrase",
                &expected,
                10_000,
                GROUPS,
            )
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            for parses in orders {
                assert_eq!(parses.len(), 1, "{source}");
            }
        }
    }

    #[test]
    fn generated_inverse_renders_and_exactly_reparses_every_adjective_shape() {
        let target = build_adjective_phrase(Adjective::Word(Vocab::Target)).unwrap();
        let comparison = |marker, standard| {
            let complement = match marker {
                ComparisonMarker::Than => build_comparison_than(standard),
                ComparisonMarker::ThanOrEqualTo => build_comparison_than_or_equal_to(standard),
            }
            .unwrap();
            build_adjective_phrase_comparison(
                build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap(),
                complement,
            )
            .unwrap()
        };
        let clause = crate::grammar::parse_nonterminal_with_activation(
            "draw a card",
            &Catalogs::default(),
            crate::grammar::Nonterminal::Clause,
            crate::grammar::GeneratedActivation::Production,
        )
        .unwrap()
        .clause()
        .unwrap()
        .clone();
        let noun_standard = crate::grammar::parse_nonterminal_with_activation(
            "a card",
            &Catalogs::default(),
            crate::grammar::Nonterminal::NounPhrase,
            crate::grammar::GeneratedActivation::Production,
        )
        .unwrap()
        .noun_phrase()
        .unwrap()
        .clone();
        let rows = [
            ("target", target.clone(), "adjective_phrase", None),
            (
                "face up",
                build_adjective_phrase_face_up().unwrap(),
                "adjective_phrase_face_up",
                None,
            ),
            (
                "face down",
                build_adjective_phrase_face_down().unwrap(),
                "adjective_phrase_face_down",
                None,
            ),
            (
                "2 greater",
                build_adjective_phrase_degree_measure(
                    NumberLiteral {
                        value: 2,
                        numeral: Numeral::Arabic(false),
                    },
                    Adjective::Word(Vocab::Greater),
                )
                .unwrap(),
                "adjective_phrase_degree_measure",
                None,
            ),
            (
                "two greater",
                build_adjective_phrase_degree_measure(
                    NumberLiteral {
                        value: 2,
                        numeral: Numeral::Cardinal,
                    },
                    Adjective::Word(Vocab::Greater),
                )
                .unwrap(),
                "adjective_phrase_degree_measure",
                None,
            ),
            (
                "greater than a card",
                comparison(
                    ComparisonMarker::Than,
                    build_comparison_standard(Some(noun_standard), None, None).unwrap(),
                ),
                "adjective_phrase_comparison",
                Some(0),
            ),
            (
                "greater than or equal to target",
                comparison(
                    ComparisonMarker::ThanOrEqualTo,
                    build_comparison_standard(None, Some(target), None).unwrap(),
                ),
                "adjective_phrase_comparison",
                Some(1),
            ),
            (
                "greater than draw a card",
                comparison(
                    ComparisonMarker::Than,
                    build_comparison_standard(None, None, Some(clause)).unwrap(),
                ),
                "adjective_phrase_comparison",
                Some(2),
            ),
        ];

        for (source, expected, root, standard_form) in rows {
            let rendered = crate::renderer::render_generated_adjective_phrase_law(&expected)
                .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            assert_eq!(rendered.text, source);
            assert_eq!(rendered.forms.first().map(|form| form.0), Some(root));
            if let Some(ordinal) = standard_form {
                assert!(
                    rendered.forms.contains(&("comparison_standard", ordinal)),
                    "{source}: {:?}",
                    rendered.forms,
                );
            }
            let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                source,
                &Catalogs::default(),
                "AdjectivePhrase",
                &expected,
                10_000,
                crate::constructions::GROUPS,
            )
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            for parses in orders {
                assert_eq!(
                    parses
                        .iter()
                        .map(|parse| parse.ast().construction)
                        .collect::<Vec<_>>(),
                    [root],
                    "{source}",
                );
            }
        }
    }

    #[test]
    fn declared_rows_report_generated_ownership_in_both_registration_orders() {
        use crate::grammar::Nonterminal;

        for (source, nonterminal, expected) in [
            ("target", Nonterminal::Adjective, "adjective"),
            ("target", Nonterminal::AdjectivePhrase, "adjective_phrase"),
            (
                "face up",
                Nonterminal::AdjectivePhrase,
                "adjective_phrase_face_up",
            ),
            (
                "face down",
                Nonterminal::AdjectivePhrase,
                "adjective_phrase_face_down",
            ),
            (
                "than target",
                Nonterminal::ComparisonComplement,
                "comparison_standard",
            ),
            (
                "than target",
                Nonterminal::ComparisonComplement,
                "comparison_than",
            ),
            (
                "than or equal to target",
                Nonterminal::ComparisonComplement,
                "comparison_than_or_equal_to",
            ),
            (
                "greater than target",
                Nonterminal::AdjectivePhrase,
                "adjective_phrase_comparison",
            ),
            (
                "2 greater",
                Nonterminal::AdjectivePhrase,
                "adjective_phrase_degree_measure",
            ),
        ] {
            let orders = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &Catalogs::default(),
                nonterminal,
                &crate::identity::SelfReference::default(),
                crate::grammar::GeneratedActivation::Groups(GROUPS),
            );
            for parsed in orders {
                let decision = parsed
                    .construction_decisions()
                    .iter()
                    .find(|decision| decision.selected().as_str() == expected)
                    .unwrap_or_else(|| {
                        panic!(
                            "missing generated {expected} for {source:?}: {:#?}",
                            parsed.construction_decisions(),
                        )
                    });
                assert_eq!(
                    decision.backend(),
                    crate::ConstructionBackend::Chart,
                    "{source}"
                );
            }
        }
    }

    #[test]
    fn declaration_feature_transitions_complete_and_measure_only_admitted_inputs() {
        let greater = adjective_features(&Adjective::Word(Vocab::Greater), false).unwrap();
        let complement = Features::None;
        let comparison = reduce_adjective_features(7, &[Some(&greater), Some(&complement)])
            .expect("a pending comparison completes");
        assert_eq!(
            feature_projection(&comparison).unwrap().comparison,
            AdjectiveComparisonState::Complete
        );
        assert!(reduce_adjective_features(7, &[Some(&comparison), Some(&complement)]).is_none());

        let number = Features::Number { is_one: false };
        let measured = reduce_adjective_features(8, &[Some(&number), Some(&greater)])
            .expect("a numeric OrComparative becomes measured");
        let measured = feature_projection(&measured).unwrap();
        assert_eq!(measured.comparison, AdjectiveComparisonState::Measured);
        assert!(measured.predicative_only);

        let other = adjective_features(&Adjective::Word(Vocab::Other), false).unwrap();
        let face_up =
            adjective_features(&Adjective::CardOrientation(CardOrientation::FaceUp), true).unwrap();
        assert!(reduce_adjective_features(8, &[Some(&number), Some(&other)]).is_none());
        assert!(reduce_adjective_features(8, &[Some(&number), Some(&face_up)]).is_none());
    }
}
