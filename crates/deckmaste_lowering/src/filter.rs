//! `filter` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::ObjectKind {
    type Target = deckmaste_core::ObjectKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Ability => deckmaste_core::ObjectKind::Ability,
            Self::Card => deckmaste_core::ObjectKind::Card,
            Self::CardCopy => deckmaste_core::ObjectKind::CardCopy,
            Self::Emblem => deckmaste_core::ObjectKind::Emblem,
            Self::Player => deckmaste_core::ObjectKind::Player,
            Self::Spell => deckmaste_core::ObjectKind::Spell,
            Self::Token => deckmaste_core::ObjectKind::Token,
        }
    }
}

impl Lower for deckmaste_authoring::CharacteristicPredicate {
    type Target = deckmaste_core::CharacteristicPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Type(f0) => deckmaste_core::CharacteristicPredicate::Type(f0.lower()),
            Self::Subtype(f0) => deckmaste_core::CharacteristicPredicate::Subtype(f0.lower()),
            Self::Supertype(f0) => deckmaste_core::CharacteristicPredicate::Supertype(f0.lower()),
            Self::ColorIs(f0) => deckmaste_core::CharacteristicPredicate::ColorIs(f0.lower()),
            Self::Named(f0) => deckmaste_core::CharacteristicPredicate::Named(f0.lower()),
            Self::Stat(f0, f1, f2) => {
                deckmaste_core::CharacteristicPredicate::Stat(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Multicolored => deckmaste_core::CharacteristicPredicate::Multicolored,
            Self::Colorless => deckmaste_core::CharacteristicPredicate::Colorless,
            Self::Has(f0) => deckmaste_core::CharacteristicPredicate::Has(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::StatePredicate {
    type Target = deckmaste_core::StatePredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::InZone(f0) => deckmaste_core::StatePredicate::InZone(f0.lower()),
            Self::Status(f0) => deckmaste_core::StatePredicate::Status(f0.lower()),
            Self::SummoningSick => deckmaste_core::StatePredicate::SummoningSick,
            Self::HasCounter(f0) => deckmaste_core::StatePredicate::HasCounter(f0.lower()),
            Self::Designated(f0) => deckmaste_core::StatePredicate::Designated(f0.lower()),
            Self::RelatedBy(f0, f1) => {
                deckmaste_core::StatePredicate::RelatedBy(f0.lower(), f1.lower())
            }
            Self::Attacking => deckmaste_core::StatePredicate::Attacking,
            Self::Blocking => deckmaste_core::StatePredicate::Blocking,
            Self::Unblocked => deckmaste_core::StatePredicate::Unblocked,
            Self::Targets(f0) => deckmaste_core::StatePredicate::Targets(f0.lower()),
            Self::TargetCount(f0) => deckmaste_core::StatePredicate::TargetCount(f0.lower()),
            Self::WasPaidWith(f0) => deckmaste_core::StatePredicate::WasPaidWith(f0.lower()),
            Self::WasCastWith(f0) => deckmaste_core::StatePredicate::WasCastWith(f0.lower()),
            Self::WasPutFrom(f0) => deckmaste_core::StatePredicate::WasPutFrom(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::RelationPredicate {
    type Target = deckmaste_core::RelationPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ControlledBy(f0) => deckmaste_core::RelationPredicate::ControlledBy(f0.lower()),
            Self::Controls(f0) => deckmaste_core::RelationPredicate::Controls(f0.lower()),
            Self::Owner(f0) => deckmaste_core::RelationPredicate::Owner(f0.lower()),
            Self::OpponentOf(f0) => deckmaste_core::RelationPredicate::OpponentOf(f0.lower()),
            Self::TeammateOf(f0) => deckmaste_core::RelationPredicate::TeammateOf(f0.lower()),
            Self::AttachedTo(f0) => deckmaste_core::RelationPredicate::AttachedTo(f0.lower()),
            Self::Attachment(f0) => deckmaste_core::RelationPredicate::Attachment(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Adjacency {
    type Target = deckmaste_core::Adjacency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Above => deckmaste_core::Adjacency::Above,
            Self::Below => deckmaste_core::Adjacency::Below,
        }
    }
}

impl Lower for deckmaste_authoring::Predicate {
    type Target = deckmaste_core::Predicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Kind(f0) => deckmaste_core::Predicate::Kind(f0.lower()),
            Self::Characteristic(f0) => deckmaste_core::Predicate::Characteristic(f0.lower()),
            Self::State(f0) => deckmaste_core::Predicate::State(f0.lower()),
            Self::Relation(f0) => deckmaste_core::Predicate::Relation(f0.lower()),
            Self::Ref(f0) => deckmaste_core::Predicate::Ref(f0.lower()),
            Self::Adjacent(f0, f1) => deckmaste_core::Predicate::Adjacent(f0.lower(), f1.lower()),
            Self::PlayerStatCmp(f0, f1, f2) => {
                deckmaste_core::Predicate::PlayerStatCmp(f0.lower(), f1.lower(), f2.lower())
            }
            Self::FromSource(f0) => deckmaste_core::Predicate::FromSource(f0.lower()),
            Self::And(f0) => deckmaste_core::Predicate::And(f0.lower()),
            Self::Or(f0) => deckmaste_core::Predicate::Or(f0.lower()),
            Self::Not(f0) => deckmaste_core::Predicate::Not(f0.lower()),
            Self::Where(f0) => deckmaste_core::Predicate::Where(f0.lower()),
            Self::Any => deckmaste_core::Predicate::Any,
            Self::Expanded(f0) => deckmaste_core::Predicate::Expanded(f0.lower()),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_object_kind_ability() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Ability.lower(),
            deckmaste_core::ObjectKind::Ability
        );
    }

    #[test]
    fn lowers_object_kind_card() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Card.lower(),
            deckmaste_core::ObjectKind::Card
        );
    }

    #[test]
    fn lowers_object_kind_card_copy() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::CardCopy.lower(),
            deckmaste_core::ObjectKind::CardCopy
        );
    }

    #[test]
    fn lowers_object_kind_emblem() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Emblem.lower(),
            deckmaste_core::ObjectKind::Emblem
        );
    }

    #[test]
    fn lowers_object_kind_player() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Player.lower(),
            deckmaste_core::ObjectKind::Player
        );
    }

    #[test]
    fn lowers_object_kind_spell() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Spell.lower(),
            deckmaste_core::ObjectKind::Spell
        );
    }

    #[test]
    fn lowers_object_kind_token() {
        assert_matches!(
            deckmaste_authoring::ObjectKind::Token.lower(),
            deckmaste_core::ObjectKind::Token
        );
    }

    #[test]
    fn lowers_characteristic_predicate_type() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Type(minimal_type_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Type(deckmaste_core::TypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_subtype() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Subtype(minimal_subtype_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Subtype(deckmaste_core::SubtypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_supertype() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Supertype(minimal_supertype()).lower(),
            deckmaste_core::CharacteristicPredicate::Supertype(deckmaste_core::Supertype::Basic)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_color_is() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::ColorIs(minimal_color()).lower(),
            deckmaste_core::CharacteristicPredicate::ColorIs(deckmaste_core::Color::White)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_named() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Named("X".into()).lower(),
            deckmaste_core::CharacteristicPredicate::Named(_)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_stat() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Stat(
                minimal_stat(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::CharacteristicPredicate::Stat(
                deckmaste_core::Stat::Power,
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_characteristic_predicate_multicolored() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Multicolored.lower(),
            deckmaste_core::CharacteristicPredicate::Multicolored
        );
    }

    #[test]
    fn lowers_characteristic_predicate_colorless() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Colorless.lower(),
            deckmaste_core::CharacteristicPredicate::Colorless
        );
    }

    #[test]
    fn lowers_characteristic_predicate_has() {
        assert_matches!(
            deckmaste_authoring::CharacteristicPredicate::Has(minimal_keyword_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Has(deckmaste_core::KeywordRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_in_zone() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::InZone(minimal_zone()).lower(),
            deckmaste_core::StatePredicate::InZone(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_state_predicate_status() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Status(minimal_status()).lower(),
            deckmaste_core::StatePredicate::Status(deckmaste_core::Status::Tapped)
        );
    }

    #[test]
    fn lowers_state_predicate_summoning_sick() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::SummoningSick.lower(),
            deckmaste_core::StatePredicate::SummoningSick
        );
    }

    #[test]
    fn lowers_state_predicate_has_counter() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::HasCounter(minimal_counter_ref()).lower(),
            deckmaste_core::StatePredicate::HasCounter(deckmaste_core::CounterRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_designated() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Designated("X".into()).lower(),
            deckmaste_core::StatePredicate::Designated(_)
        );
    }

    #[test]
    fn lowers_state_predicate_related_by() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::RelatedBy(
                "X".into(),
                std::sync::Arc::new(minimal_predicate())
            )
            .lower(),
            deckmaste_core::StatePredicate::RelatedBy(_, _)
        );
    }

    #[test]
    fn lowers_state_predicate_attacking() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Attacking.lower(),
            deckmaste_core::StatePredicate::Attacking
        );
    }

    #[test]
    fn lowers_state_predicate_blocking() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Blocking.lower(),
            deckmaste_core::StatePredicate::Blocking
        );
    }

    #[test]
    fn lowers_state_predicate_unblocked() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Unblocked.lower(),
            deckmaste_core::StatePredicate::Unblocked
        );
    }

    #[test]
    fn lowers_state_predicate_targets() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::Targets(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::StatePredicate::Targets(_)
        );
    }

    #[test]
    fn lowers_state_predicate_target_count() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::TargetCount(minimal_count_bound()).lower(),
            deckmaste_core::StatePredicate::TargetCount(deckmaste_core::CountBound::Eq(
                deckmaste_core::Count::X
            ))
        );
    }

    #[test]
    fn lowers_state_predicate_was_paid_with() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::WasPaidWith(minimal_cost_tag()).lower(),
            deckmaste_core::StatePredicate::WasPaidWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_cast_with() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::WasCastWith(minimal_cost_tag()).lower(),
            deckmaste_core::StatePredicate::WasCastWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_put_from() {
        assert_matches!(
            deckmaste_authoring::StatePredicate::WasPutFrom(minimal_zone()).lower(),
            deckmaste_core::StatePredicate::WasPutFrom(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_relation_predicate_controlled_by() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::ControlledBy(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::ControlledBy(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_controls() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::Controls(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::Controls(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_owner() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::Owner(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::RelationPredicate::Owner(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_opponent_of() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::OpponentOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::OpponentOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_teammate_of() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::TeammateOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::TeammateOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attached_to() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::AttachedTo(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::AttachedTo(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attachment() {
        assert_matches!(
            deckmaste_authoring::RelationPredicate::Attachment(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::Attachment(_)
        );
    }

    #[test]
    fn lowers_adjacency_above() {
        assert_matches!(
            deckmaste_authoring::Adjacency::Above.lower(),
            deckmaste_core::Adjacency::Above
        );
    }

    #[test]
    fn lowers_adjacency_below() {
        assert_matches!(
            deckmaste_authoring::Adjacency::Below.lower(),
            deckmaste_core::Adjacency::Below
        );
    }

    #[test]
    fn lowers_predicate_kind() {
        assert_matches!(
            deckmaste_authoring::Predicate::Kind(minimal_object_kind()).lower(),
            deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
        );
    }

    #[test]
    fn lowers_predicate_characteristic() {
        assert_matches!(
            deckmaste_authoring::Predicate::Characteristic(minimal_characteristic_predicate())
                .lower(),
            deckmaste_core::Predicate::Characteristic(
                deckmaste_core::CharacteristicPredicate::Type(deckmaste_core::TypeRef(_))
            )
        );
    }

    #[test]
    fn lowers_predicate_state() {
        assert_matches!(
            deckmaste_authoring::Predicate::State(minimal_state_predicate()).lower(),
            deckmaste_core::Predicate::State(deckmaste_core::StatePredicate::InZone(
                deckmaste_core::Zone::Battlefield
            ))
        );
    }

    #[test]
    fn lowers_predicate_relation() {
        assert_matches!(
            deckmaste_authoring::Predicate::Relation(minimal_relation_predicate()).lower(),
            deckmaste_core::Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(_))
        );
    }

    #[test]
    fn lowers_predicate_ref() {
        assert_matches!(
            deckmaste_authoring::Predicate::Ref(minimal_reference()).lower(),
            deckmaste_core::Predicate::Ref(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_predicate_adjacent() {
        assert_matches!(
            deckmaste_authoring::Predicate::Adjacent(minimal_adjacency(), minimal_reference())
                .lower(),
            deckmaste_core::Predicate::Adjacent(
                deckmaste_core::Adjacency::Above,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_predicate_player_stat_cmp() {
        assert_matches!(
            deckmaste_authoring::Predicate::PlayerStatCmp(
                minimal_player_attr(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Predicate::PlayerStatCmp(
                deckmaste_core::PlayerAttr::Life,
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_predicate_from_source() {
        assert_matches!(
            deckmaste_authoring::Predicate::FromSource(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Predicate::FromSource(_)
        );
    }

    #[test]
    fn lowers_predicate_and() {
        assert_matches!(
            deckmaste_authoring::Predicate::And([].into()).lower(),
            deckmaste_core::Predicate::And(_)
        );
    }

    #[test]
    fn lowers_predicate_or() {
        assert_matches!(
            deckmaste_authoring::Predicate::Or([].into()).lower(),
            deckmaste_core::Predicate::Or(_)
        );
    }

    #[test]
    fn lowers_predicate_not() {
        assert_matches!(
            deckmaste_authoring::Predicate::Not(std::sync::Arc::new(minimal_predicate())).lower(),
            deckmaste_core::Predicate::Not(_)
        );
    }

    #[test]
    fn lowers_predicate_where() {
        assert_matches!(
            deckmaste_authoring::Predicate::Where(std::sync::Arc::new(minimal_condition())).lower(),
            deckmaste_core::Predicate::Where(_)
        );
    }

    #[test]
    fn lowers_predicate_any() {
        assert_matches!(
            deckmaste_authoring::Predicate::Any.lower(),
            deckmaste_core::Predicate::Any
        );
    }

    #[test]
    fn lowers_predicate_expanded() {
        assert_matches!(
            deckmaste_authoring::Predicate::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_predicate())
            })
            .lower(),
            deckmaste_core::Predicate::Expanded(_)
        );
    }
}
