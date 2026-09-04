//! `filter` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

/// The core predicate for semantics' exclusive object-kind axis
/// ([CR#109.1]). `Player` is an Entity-level classification — a player is not
/// an object ([CR#102.1]) — and every other member names one CR object class,
/// which core tests independently of the rest.
#[must_use]
pub fn object_kind_predicate(kind: deckmaste_semantics::ObjectKind) -> deckmaste_core::Predicate {
    use deckmaste_core::ObjectClass as C;
    use deckmaste_semantics::ObjectKind as K;
    let class = match kind {
        K::Player => {
            return deckmaste_core::Predicate::Entity(deckmaste_core::EntityClass::Player);
        }
        K::Ability => C::AbilityOnStack,
        K::Card => C::Card,
        K::CardCopy => C::CopyOfACard,
        K::Emblem => C::Emblem,
        K::Spell => C::Spell,
        K::Token => C::Token,
    };
    deckmaste_core::Predicate::Class(class)
}

impl Lower for deckmaste_semantics::CharacteristicPredicate {
    type Target = deckmaste_core::CharacteristicPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Type(f0) => deckmaste_core::CharacteristicPredicate::Type(f0.lower()),
            Self::Subtype(f0) => deckmaste_core::CharacteristicPredicate::Subtype(f0.lower()),
            Self::Supertype(f0) => deckmaste_core::CharacteristicPredicate::Supertype(f0.lower()),
            Self::ColorIs(f0) => deckmaste_core::CharacteristicPredicate::ColorIs(f0.lower()),
            Self::Named(name) => crate::region::named(&name).map_or_else(
                || deckmaste_core::CharacteristicPredicate::Named(name.lower()),
                deckmaste_core::CharacteristicPredicate::NamedReg,
            ),
            Self::Stat(f0, f1, f2) => {
                deckmaste_core::CharacteristicPredicate::Stat(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Multicolored => deckmaste_core::CharacteristicPredicate::Multicolored,
            Self::Colorless => deckmaste_core::CharacteristicPredicate::Colorless,
            Self::Has(f0) => deckmaste_core::CharacteristicPredicate::Has(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::StatePredicate {
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

impl Lower for deckmaste_semantics::RelationPredicate {
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

impl Lower for deckmaste_semantics::Adjacency {
    type Target = deckmaste_core::Adjacency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Above => deckmaste_core::Adjacency::Above,
            Self::Below => deckmaste_core::Adjacency::Below,
        }
    }
}

impl Lower for deckmaste_semantics::Predicate {
    type Target = deckmaste_core::Predicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Kind(f0) => object_kind_predicate(f0),
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
            Self::Where(f0) => deckmaste_core::Predicate::Where(std::sync::Arc::new(
                crate::region::candidate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
            Self::Any => deckmaste_core::Predicate::Any,
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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

    use super::object_kind_predicate;
    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_object_kind_ability() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Ability),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
        );
    }

    #[test]
    fn lowers_object_kind_card() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Card),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::Card)
        );
    }

    #[test]
    fn lowers_object_kind_card_copy() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::CardCopy),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::CopyOfACard)
        );
    }

    #[test]
    fn lowers_object_kind_emblem() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Emblem),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::Emblem)
        );
    }

    #[test]
    fn lowers_object_kind_player() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Player),
            deckmaste_core::Predicate::Entity(deckmaste_core::EntityClass::Player)
        );
    }

    #[test]
    fn lowers_object_kind_spell() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Spell),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::Spell)
        );
    }

    #[test]
    fn lowers_object_kind_token() {
        assert_matches!(
            object_kind_predicate(deckmaste_semantics::ObjectKind::Token),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::Token)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_type() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Type(minimal_type_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Type(deckmaste_core::TypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_subtype() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Subtype(minimal_subtype_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Subtype(deckmaste_core::SubtypeRef(_))
        );
    }

    #[test]
    fn lowers_characteristic_predicate_supertype() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Supertype(minimal_supertype()).lower(),
            deckmaste_core::CharacteristicPredicate::Supertype(deckmaste_core::Supertype::Basic)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_color_is() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::ColorIs(minimal_color()).lower(),
            deckmaste_core::CharacteristicPredicate::ColorIs(deckmaste_core::Color::White)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_named() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Named("X".into()).lower(),
            deckmaste_core::CharacteristicPredicate::Named(_)
        );
    }

    #[test]
    fn lowers_characteristic_predicate_stat() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Stat(
                minimal_stat(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::CharacteristicPredicate::Stat(
                deckmaste_core::Stat::Power,
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_characteristic_predicate_multicolored() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Multicolored.lower(),
            deckmaste_core::CharacteristicPredicate::Multicolored
        );
    }

    #[test]
    fn lowers_characteristic_predicate_colorless() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Colorless.lower(),
            deckmaste_core::CharacteristicPredicate::Colorless
        );
    }

    #[test]
    fn lowers_characteristic_predicate_has() {
        assert_matches!(
            deckmaste_semantics::CharacteristicPredicate::Has(minimal_keyword_ref()).lower(),
            deckmaste_core::CharacteristicPredicate::Has(deckmaste_core::KeywordRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_in_zone() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::InZone(minimal_zone()).lower(),
            deckmaste_core::StatePredicate::InZone(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_state_predicate_status() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::Status(minimal_status()).lower(),
            deckmaste_core::StatePredicate::Status(deckmaste_core::Status::Tapped)
        );
    }

    #[test]
    fn lowers_state_predicate_summoning_sick() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::SummoningSick.lower(),
            deckmaste_core::StatePredicate::SummoningSick
        );
    }

    #[test]
    fn lowers_state_predicate_has_counter() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::HasCounter(minimal_counter_ref()).lower(),
            deckmaste_core::StatePredicate::HasCounter(deckmaste_core::CounterRef(_))
        );
    }

    #[test]
    fn lowers_state_predicate_designated() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::Designated("X".into()).lower(),
            deckmaste_core::StatePredicate::Designated(_)
        );
    }

    #[test]
    fn lowers_state_predicate_related_by() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::RelatedBy(
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
            deckmaste_semantics::StatePredicate::Attacking.lower(),
            deckmaste_core::StatePredicate::Attacking
        );
    }

    #[test]
    fn lowers_state_predicate_blocking() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::Blocking.lower(),
            deckmaste_core::StatePredicate::Blocking
        );
    }

    #[test]
    fn lowers_state_predicate_unblocked() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::Unblocked.lower(),
            deckmaste_core::StatePredicate::Unblocked
        );
    }

    #[test]
    fn lowers_state_predicate_targets() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::Targets(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::StatePredicate::Targets(_)
        );
    }

    #[test]
    fn lowers_state_predicate_target_count() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::TargetCount(minimal_count_bound()).lower(),
            deckmaste_core::StatePredicate::TargetCount(deckmaste_core::CountBound::Eq(
                deckmaste_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_state_predicate_was_paid_with() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::WasPaidWith(minimal_cost_tag()).lower(),
            deckmaste_core::StatePredicate::WasPaidWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_cast_with() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::WasCastWith(minimal_cost_tag()).lower(),
            deckmaste_core::StatePredicate::WasCastWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_state_predicate_was_put_from() {
        assert_matches!(
            deckmaste_semantics::StatePredicate::WasPutFrom(minimal_zone()).lower(),
            deckmaste_core::StatePredicate::WasPutFrom(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_relation_predicate_controlled_by() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::ControlledBy(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::ControlledBy(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_controls() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::Controls(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::Controls(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_owner() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::Owner(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::RelationPredicate::Owner(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_opponent_of() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::OpponentOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::OpponentOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_teammate_of() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::TeammateOf(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::TeammateOf(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attached_to() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::AttachedTo(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::AttachedTo(_)
        );
    }

    #[test]
    fn lowers_relation_predicate_attachment() {
        assert_matches!(
            deckmaste_semantics::RelationPredicate::Attachment(std::sync::Arc::new(
                minimal_predicate()
            ))
            .lower(),
            deckmaste_core::RelationPredicate::Attachment(_)
        );
    }

    #[test]
    fn lowers_adjacency_above() {
        assert_matches!(
            deckmaste_semantics::Adjacency::Above.lower(),
            deckmaste_core::Adjacency::Above
        );
    }

    #[test]
    fn lowers_adjacency_below() {
        assert_matches!(
            deckmaste_semantics::Adjacency::Below.lower(),
            deckmaste_core::Adjacency::Below
        );
    }

    #[test]
    fn lowers_predicate_kind() {
        assert_matches!(
            deckmaste_semantics::Predicate::Kind(minimal_object_kind()).lower(),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
        );
    }

    #[test]
    fn lowers_predicate_characteristic() {
        assert_matches!(
            deckmaste_semantics::Predicate::Characteristic(minimal_characteristic_predicate())
                .lower(),
            deckmaste_core::Predicate::Characteristic(
                deckmaste_core::CharacteristicPredicate::Type(deckmaste_core::TypeRef(_))
            )
        );
    }

    #[test]
    fn lowers_predicate_state() {
        assert_matches!(
            deckmaste_semantics::Predicate::State(minimal_state_predicate()).lower(),
            deckmaste_core::Predicate::State(deckmaste_core::StatePredicate::InZone(
                deckmaste_core::Zone::Battlefield
            ))
        );
    }

    #[test]
    fn lowers_predicate_relation() {
        assert_matches!(
            deckmaste_semantics::Predicate::Relation(minimal_relation_predicate()).lower(),
            deckmaste_core::Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(_))
        );
    }

    #[test]
    fn lowers_predicate_ref() {
        assert_matches!(
            deckmaste_semantics::Predicate::Ref(minimal_reference()).lower(),
            deckmaste_core::Predicate::Ref(deckmaste_core::Reference::Reg(deckmaste_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_predicate_adjacent() {
        assert_matches!(
            deckmaste_semantics::Predicate::Adjacent(minimal_adjacency(), minimal_reference())
                .lower(),
            deckmaste_core::Predicate::Adjacent(
                deckmaste_core::Adjacency::Above,
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_predicate_player_stat_cmp() {
        assert_matches!(
            deckmaste_semantics::Predicate::PlayerStatCmp(
                minimal_player_attr(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Predicate::PlayerStatCmp(
                deckmaste_core::PlayerAttr::Life,
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_predicate_from_source() {
        assert_matches!(
            deckmaste_semantics::Predicate::FromSource(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Predicate::FromSource(_)
        );
    }

    #[test]
    fn lowers_predicate_and() {
        assert_matches!(
            deckmaste_semantics::Predicate::And([].into()).lower(),
            deckmaste_core::Predicate::And(_)
        );
    }

    #[test]
    fn lowers_predicate_or() {
        assert_matches!(
            deckmaste_semantics::Predicate::Or([].into()).lower(),
            deckmaste_core::Predicate::Or(_)
        );
    }

    #[test]
    fn lowers_predicate_not() {
        assert_matches!(
            deckmaste_semantics::Predicate::Not(std::sync::Arc::new(minimal_predicate())).lower(),
            deckmaste_core::Predicate::Not(_)
        );
    }

    #[test]
    fn lowers_predicate_where() {
        assert_matches!(
            deckmaste_semantics::Predicate::Where(std::sync::Arc::new(minimal_condition())).lower(),
            deckmaste_core::Predicate::Where(_)
        );
    }

    #[test]
    fn lowers_predicate_any() {
        assert_matches!(
            deckmaste_semantics::Predicate::Any.lower(),
            deckmaste_core::Predicate::Any
        );
    }

    #[test]
    fn lowers_predicate_expanded() {
        assert_matches!(
            deckmaste_semantics::Predicate::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_predicate())
            })
            .lower(),
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
        );
    }
}
