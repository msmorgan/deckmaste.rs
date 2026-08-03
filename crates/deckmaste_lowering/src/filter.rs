//! `filter` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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
