//! Compiler-derived adjective base declarations.
#![allow(
    dead_code,
    reason = "the adjective declaration group stays inactive until its production-ownership migration"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::grammar::Features;
use crate::grammar::adjective_features;
use crate::syntax::AdjectivePhrase;
use crate::word::Adjective;
use crate::word::CardOrientation;
use crate::word::Vocabulary;

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
    matches!(
        identity,
        Features::Adjective {
            card_orientation: false,
            ..
        }
    )
    .then(|| identity.clone())
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

fn make_face_up() -> Result<AdjectivePhrase, DeclarationViolation> {
    Ok(orientation_phrase(CardOrientation::FaceUp))
}

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
}

pub(crate) static GROUPS: &[&GroupData] = &[&ADJECTIVE_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalogs;
    use crate::syntax::AdjectivePhrase;
    use crate::word::Adjective;
    use crate::word::CardOrientation;
    use crate::word::Vocab;

    const BASE_IDS: &[&str] = &[
        "adjective",
        "adjective_phrase",
        "adjective_phrase_face_up",
        "adjective_phrase_face_down",
    ];

    fn rebuild_adjective(value: Adjective) -> Adjective {
        let built = build_adjective(value).expect("the lexical adjective is admitted");
        let identity = parts_adjective(&built);
        build_adjective(identity).expect("the lexical adjective rebuilds")
    }

    fn rebuild_phrase(
        value: AdjectivePhrase,
        parts: fn(&AdjectivePhrase),
        build: fn() -> Result<
            AdjectivePhrase,
            deckmaste_construction_compiler::runtime::DeclarationViolation,
        >,
    ) -> AdjectivePhrase {
        let built = build().expect("the orientation phrase is admitted");
        assert_eq!(built, value);
        parts(&built);
        build().expect("the orientation phrase rebuilds")
    }

    #[test]
    fn base_declarations_have_the_exact_census_and_checked_round_trip_laws() {
        let actual = GROUPS[0]
            .constructions
            .iter()
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
            assert_eq!(rebuild_phrase(expected.clone(), parts, build), expected);
        }

        assert!(matches!(
            build_adjective_phrase(Adjective::CardOrientation(CardOrientation::FaceUp)),
            Err(deckmaste_construction_compiler::runtime::DeclarationViolation { .. })
        ));
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
}
