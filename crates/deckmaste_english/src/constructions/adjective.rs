//! Compiler-derived adjective declarations.
#![allow(
    dead_code,
    reason = "the adjective declaration group stays inactive until its production-ownership migration"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Onset as InitialSound;
use crate::grammar::AdjectiveComparisonState;
use crate::grammar::Features;
use crate::grammar::adjective_comparison_state;
use crate::grammar::adjective_features;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::ComparisonComplement;
use crate::syntax::ComparisonMarker;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::Phrase;
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
    pub(crate) predicative_only: bool,
}

fn feature_projection(features: &Features) -> Option<AdjectiveFeatureProjection> {
    let Features::Adjective {
        initial_sound,
        comparison,
        card_orientation,
        ..
    } = features
    else {
        return None;
    };
    Some(AdjectiveFeatureProjection {
        onset: *initial_sound,
        comparison: *comparison,
        card_orientation: *card_orientation,
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
    Ok(AdjectivePhrase {
        degree: None,
        head,
        complements: Vec::new(),
    })
}

fn adjective_phrase_head(value: &AdjectivePhrase) -> Adjective {
    value.head.clone()
}

fn is_base_adjective_phrase(value: &AdjectivePhrase) -> bool {
    value.degree.is_none() && value.complements.is_empty() && is_lexical_adjective(&value.head)
}

fn orientation_phrase(orientation: CardOrientation) -> AdjectivePhrase {
    AdjectivePhrase {
        degree: None,
        head: Adjective::CardOrientation(orientation),
        complements: Vec::new(),
    }
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
    Ok(ComparisonComplement {
        marker: ComparisonMarker::Than,
        standard: Box::new(standard),
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
    Ok(ComparisonComplement {
        marker: ComparisonMarker::ThanOrEqualTo,
        standard: Box::new(standard),
    })
}

fn comparison_standard(value: &ComparisonComplement) -> Phrase {
    value.standard.as_ref().clone()
}

fn is_comparison_than(value: &ComparisonComplement) -> bool {
    value.marker == ComparisonMarker::Than
}

fn is_comparison_than_or_equal_to(value: &ComparisonComplement) -> bool {
    value.marker == ComparisonMarker::ThanOrEqualTo
}

fn pending_comparison_owner(value: &AdjectivePhrase) -> bool {
    value.degree.is_none()
        && value.complements.is_empty()
        && matches!(
            adjective_comparison_state(&value.head),
            AdjectiveComparisonState::Pending(_)
        )
}

fn comparison_class_matches(owner: &AdjectivePhrase, comparison: &ComparisonComplement) -> bool {
    matches!(
        (adjective_comparison_state(&owner.head), comparison.marker),
        (
            AdjectiveComparisonState::Pending(AdjectiveComparisonClass::OrComparative),
            ComparisonMarker::Than | ComparisonMarker::ThanOrEqualTo,
        ) | (
            AdjectiveComparisonState::Pending(AdjectiveComparisonClass::ThanOnly),
            ComparisonMarker::Than,
        )
    )
}

fn attach_comparison(
    owner: &AdjectivePhrase,
    comparison: ComparisonComplement,
) -> Result<AdjectiveComplement, DeclarationViolation> {
    if !pending_comparison_owner(owner) || !comparison_class_matches(owner, &comparison) {
        return Err(violation(
            "adjective_phrase_comparison",
            "the owner is an unmeasured matching-class comparison-pending adjective phrase with no complements",
        ));
    }
    if !is_comparison_standard(&comparison.standard) {
        return Err(violation(
            "adjective_phrase_comparison",
            "the comparison has a typed standard",
        ));
    }
    Ok(AdjectiveComplement::Comparison(comparison))
}

fn detach_comparison(
    owner: &AdjectivePhrase,
    complement: AdjectiveComplement,
) -> Option<ComparisonComplement> {
    let AdjectiveComplement::Comparison(comparison) = complement else {
        return None;
    };
    attach_comparison(owner, comparison.clone()).ok()?;
    Some(comparison)
}

fn is_adjective_phrase_comparison(value: &AdjectivePhrase) -> bool {
    let [AdjectiveComplement::Comparison(comparison)] = value.complements.as_slice() else {
        return false;
    };
    let mut owner = value.clone();
    owner.complements.clear();
    attach_comparison(&owner, comparison.clone()).is_ok()
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
    Ok(AdjectivePhrase {
        degree: Some(measure),
        head: adjective,
        complements: Vec::new(),
    })
}

fn degree_measure_parts(value: &AdjectivePhrase) -> (NumberLiteral, Adjective) {
    (
        value
            .degree
            .expect("the degree-measure dispatcher admits only measured phrases"),
        value.head.clone(),
    )
}

fn is_adjective_phrase_degree_measure(value: &AdjectivePhrase) -> bool {
    let Some(measure) = value.degree else {
        return false;
    };
    value.complements.is_empty() && make_degree_measure(measure, value.head.clone()).is_ok()
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

    lens adjective_phrase_complements bind AdjectivePhrase {
        degree: opt NumberLiteral,
        head: value Adjective,
        complements: vec AdjectiveComplement,
    }

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
        bind AdjectivePhrase {
            owner: hole AdjectivePhrase,
            comparison: hole ComparisonComplement,
        }
        lens adjective_phrase_complements from owner {
            append complements with comparison via attach_comparison, detach_comparison;
        }
        derive features: Features = reduce_comparison_features(owner, comparison);
        form only @ 0 inverse check(is_adjective_phrase_comparison) = owner comparison;
        selection unique;
    }

    construction adjective_phrase_degree_measure: AdjectivePhrase {
        bind AdjectivePhrase via make_degree_measure, degree_measure_parts {
            measure: lex NumberLiteral via Numeral,
            adjective: hole Adjective,
        }
        derive features: Features = reduce_degree_measure_features(measure, adjective);
        witness numeral = stored measure;
        form only @ 0 inverse check(is_adjective_phrase_degree_measure) = lex(measure) adjective;
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
    ];

    const WRONG_COMPARISON_CLASS: &str = "WRONG_COMPARISON_CLASS";
    const ALREADY_COMPLETE: &str = "ALREADY_COMPLETE";
    const CARD_ORIENTATION_MEASURE: &str = "CARD_ORIENTATION_MEASURE";
    const ATTRIBUTIVE_DEGREE_MEASURE: &str = "ATTRIBUTIVE_DEGREE_MEASURE";
    const DUPLICATE_COMPARISON: &str = "DUPLICATE_COMPARISON";
    const SURPLUS_COMPLEMENT: &str = "SURPLUS_COMPLEMENT";

    fn comparison(marker: ComparisonMarker) -> ComparisonComplement {
        ComparisonComplement {
            marker,
            standard: Box::new(Phrase::AdjectivePhrase(Box::new(AdjectivePhrase {
                degree: None,
                head: Adjective::Word(Vocab::Target),
                complements: Vec::new(),
            }))),
        }
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
            let expected = AdjectivePhrase {
                degree: None,
                head: Adjective::CardOrientation(orientation),
                complements: Vec::new(),
            };
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

        let noun =
            crate::syntax::NounPhrase::ThisCard(crate::syntax::ThisCardForm::AbbreviatedName);
        let adjective = AdjectivePhrase {
            degree: None,
            head: Adjective::Word(Vocab::Target),
            complements: Vec::new(),
        };
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

        let standard = crate::syntax::Phrase::AdjectivePhrase(Box::new(AdjectivePhrase {
            degree: None,
            head: Adjective::Word(Vocab::Target),
            complements: Vec::new(),
        }));
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
            assert_eq!(built.marker, expected_marker);
            assert_eq!(*built.standard, standard);
            let standard = parts(&built);
            assert_eq!(build(standard).expect("the marker rebuilds"), built);
        }

        let unsupported = crate::syntax::Phrase::Quantity(crate::syntax::Quantity::Both);
        let (noun, adjective, clause) = parts_comparison_standard(&unsupported);
        assert!(build_comparison_standard(noun, adjective, clause).is_err());
        assert!(build_comparison_than(unsupported.clone()).is_err());
        assert!(build_comparison_than_or_equal_to(unsupported).is_err());
    }

    #[test]
    fn inactive_group_lowers_and_matches_both_generic_adjective_categories() {
        let lexical = Adjective::Word(Vocab::Target);
        let phrase = AdjectivePhrase {
            degree: None,
            head: lexical.clone(),
            complements: Vec::new(),
        };
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
            .expect("the inactive adjective group is test-activatable");
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
                GROUPS,
            )
            .expect("the orientation declaration is test-activatable");
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
    fn all_nine_rows_build_project_and_rebuild_their_exact_values() {
        let actual = GROUPS[0]
            .constructions
            .iter()
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(actual, ALL_IDS);

        for marker in [ComparisonMarker::Than, ComparisonMarker::ThanOrEqualTo] {
            let owner = build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap();
            let built = build_adjective_phrase_comparison(owner, comparison(marker)).unwrap();
            let (owner, complement) = parts_adjective_phrase_comparison(&built)
                .expect("the trailing comparison detaches from its owner");
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

        let mut surplus = greater;
        surplus
            .complements
            .push(AdjectiveComplement::PostnominalComparison(comparison(
                ComparisonMarker::Than,
            )));
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
    fn inactive_group_parses_all_required_comparison_and_measure_witnesses() {
        let target = Phrase::AdjectivePhrase(Box::new(AdjectivePhrase {
            degree: None,
            head: Adjective::Word(Vocab::Target),
            complements: Vec::new(),
        }));
        for (source, expected) in [
            (
                "greater than target",
                AdjectivePhrase {
                    degree: None,
                    head: Adjective::Word(Vocab::Greater),
                    complements: vec![AdjectiveComplement::Comparison(ComparisonComplement {
                        marker: ComparisonMarker::Than,
                        standard: Box::new(target.clone()),
                    })],
                },
            ),
            (
                "greater than or equal to target",
                AdjectivePhrase {
                    degree: None,
                    head: Adjective::Word(Vocab::Greater),
                    complements: vec![AdjectiveComplement::Comparison(ComparisonComplement {
                        marker: ComparisonMarker::ThanOrEqualTo,
                        standard: Box::new(target),
                    })],
                },
            ),
            (
                "2 greater",
                AdjectivePhrase {
                    degree: Some(NumberLiteral {
                        value: 2,
                        numeral: Numeral::Arabic(false),
                    }),
                    head: Adjective::Word(Vocab::Greater),
                    complements: Vec::new(),
                },
            ),
            (
                "two greater",
                AdjectivePhrase {
                    degree: Some(NumberLiteral {
                        value: 2,
                        numeral: Numeral::Cardinal,
                    }),
                    head: Adjective::Word(Vocab::Greater),
                    complements: Vec::new(),
                },
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
