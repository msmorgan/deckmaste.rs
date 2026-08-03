//! `count` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Stat {
    type Target = deckmaste_core::Stat;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power => deckmaste_core::Stat::Power,
            Self::Toughness => deckmaste_core::Stat::Toughness,
            Self::ManaValue => deckmaste_core::Stat::ManaValue,
            Self::Loyalty => deckmaste_core::Stat::Loyalty,
            Self::Defense => deckmaste_core::Stat::Defense,
        }
    }
}

impl Lower for deckmaste_authoring::RoundMode {
    type Target = deckmaste_core::RoundMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::RoundUp => deckmaste_core::RoundMode::RoundUp,
            Self::RoundDown => deckmaste_core::RoundMode::RoundDown,
        }
    }
}

impl Lower for deckmaste_authoring::Characteristic {
    type Target = deckmaste_core::Characteristic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Colors => deckmaste_core::Characteristic::Colors,
            Self::Types => deckmaste_core::Characteristic::Types,
            Self::Subtypes => deckmaste_core::Characteristic::Subtypes,
            Self::BasicLandTypes => deckmaste_core::Characteristic::BasicLandTypes,
            Self::Supertypes => deckmaste_core::Characteristic::Supertypes,
            Self::Power => deckmaste_core::Characteristic::Power,
            Self::Toughness => deckmaste_core::Characteristic::Toughness,
            Self::Defense => deckmaste_core::Characteristic::Defense,
            Self::ManaCost => deckmaste_core::Characteristic::ManaCost,
            Self::Name => deckmaste_core::Characteristic::Name,
        }
    }
}

impl Lower for deckmaste_authoring::Countable {
    type Target = deckmaste_core::Countable;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects(f0) => deckmaste_core::Countable::Objects(f0.lower()),
            Self::Players(f0) => deckmaste_core::Countable::Players(f0.lower()),
            Self::ManaSymbols(f0, f1) => {
                deckmaste_core::Countable::ManaSymbols(f0.lower(), f1.lower())
            }
            Self::Singleton(f0) => deckmaste_core::Countable::Singleton(f0.lower()),
            Self::ManaSpentMatching(f0, f1) => {
                deckmaste_core::Countable::ManaSpentMatching(f0.lower(), f1.lower())
            }
        }
    }
}

impl Lower for deckmaste_authoring::AggregateOp {
    type Target = deckmaste_core::AggregateOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SumOf => deckmaste_core::AggregateOp::SumOf,
            Self::MinOf => deckmaste_core::AggregateOp::MinOf,
            Self::MaxOf => deckmaste_core::AggregateOp::MaxOf,
            Self::AverageOf(f0) => deckmaste_core::AggregateOp::AverageOf(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Projection {
    type Target = deckmaste_core::Projection;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Projection {
            of: self.of.lower(),
            by: self.by.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Count {
    type Target = deckmaste_core::Count;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::X => deckmaste_core::Count::X,
            Self::CountOf(f0) => deckmaste_core::Count::CountOf(f0.lower()),
            Self::CountDistinct(f0, f1) => {
                deckmaste_core::Count::CountDistinct(f0.lower(), f1.lower())
            }
            Self::StatOf(f0, f1) => deckmaste_core::Count::StatOf(f0.lower(), f1.lower()),
            Self::PlayerStatOf(f0, f1) => {
                deckmaste_core::Count::PlayerStatOf(f0.lower(), f1.lower())
            }
            Self::Opponents(f0) => deckmaste_core::Count::Opponents(f0.lower()),
            Self::CounterCount(f0, f1) => {
                deckmaste_core::Count::CounterCount(f0.lower(), f1.lower())
            }
            Self::Min(f0, f1) => deckmaste_core::Count::Min(f0.lower(), f1.lower()),
            Self::Max(f0, f1) => deckmaste_core::Count::Max(f0.lower(), f1.lower()),
            Self::Plus(f0, f1) => deckmaste_core::Count::Plus(f0.lower(), f1.lower()),
            Self::Minus(f0, f1) => deckmaste_core::Count::Minus(f0.lower(), f1.lower()),
            Self::Times(f0, f1) => deckmaste_core::Count::Times(f0.lower(), f1.lower()),
            Self::Half(f0, f1) => deckmaste_core::Count::Half(f0.lower(), f1.lower()),
            Self::Divide(f0, f1, f2) => {
                deckmaste_core::Count::Divide(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Mod(f0, f1) => deckmaste_core::Count::Mod(f0.lower(), f1.lower()),
            Self::Pow(f0, f1) => deckmaste_core::Count::Pow(f0.lower(), f1.lower()),
            Self::TargetsOf(f0) => deckmaste_core::Count::TargetsOf(f0.lower()),
            Self::ThatMany => deckmaste_core::Count::ThatMany,
            Self::ThatMuch => deckmaste_core::Count::ThatMuch,
            Self::Allotment => deckmaste_core::Count::Allotment,
            Self::EventCount(f0, f1) => deckmaste_core::Count::EventCount(f0.lower(), f1.lower()),
            Self::EventSum(f0, f1) => deckmaste_core::Count::EventSum(f0.lower(), f1.lower()),
            Self::Noted(f0) => deckmaste_core::Count::Noted(f0.lower()),
            Self::TimesPaid(f0) => deckmaste_core::Count::TimesPaid(f0.lower()),
            Self::Damage(f0) => deckmaste_core::Count::Damage(f0.lower()),
            Self::ManaAvailable(f0) => deckmaste_core::Count::ManaAvailable(f0.lower()),
            Self::Aggregate(f0, f1) => deckmaste_core::Count::Aggregate(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::Count::Expanded(f0.lower()),
            Self::Literal(f0) => deckmaste_core::Count::Literal(f0.lower()),
        }
    }
}
