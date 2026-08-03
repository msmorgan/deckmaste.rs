//! `mana` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::PlanarFace {
    type Target = deckmaste_core::PlanarFace;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Blank => deckmaste_core::PlanarFace::Blank,
            Self::Chaos => deckmaste_core::PlanarFace::Chaos,
            Self::Planeswalker => deckmaste_core::PlanarFace::Planeswalker,
        }
    }
}

impl Lower for deckmaste_authoring::ManaSpec {
    type Target = deckmaste_core::ManaSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => deckmaste_core::ManaSpec::AnyColor,
            Self::OneOf(f0) => deckmaste_core::ManaSpec::OneOf(f0.lower()),
            Self::OneOfRuns(f0) => deckmaste_core::ManaSpec::OneOfRuns(f0.lower()),
            Self::AmongColorsOf(f0) => deckmaste_core::ManaSpec::AmongColorsOf(f0.lower()),
            Self::ProducedByEvent => deckmaste_core::ManaSpec::ProducedByEvent,
            Self::Specific(f0) => deckmaste_core::ManaSpec::Specific(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::SimpleManaSymbol {
    type Target = deckmaste_core::SimpleManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic(f0) => deckmaste_core::SimpleManaSymbol::Generic(f0.lower()),
            Self::Specific(f0) => deckmaste_core::SimpleManaSymbol::Specific(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::ManaSymbol {
    type Target = deckmaste_core::ManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Variable => deckmaste_core::ManaSymbol::Variable,
            Self::Snow => deckmaste_core::ManaSymbol::Snow,
            Self::Hybrid(f0, f1) => deckmaste_core::ManaSymbol::Hybrid(f0.lower(), f1.lower()),
            Self::Phyrexian(f0, f1) => {
                deckmaste_core::ManaSymbol::Phyrexian(f0.lower(), f1.lower())
            }
            Self::Simple(f0) => deckmaste_core::ManaSymbol::Simple(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::SymbolPred {
    type Target = deckmaste_core::SymbolPred;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => deckmaste_core::SymbolPred::AnyColor,
            Self::AnyType => deckmaste_core::SymbolPred::AnyType,
            Self::CountsAs(f0) => deckmaste_core::SymbolPred::CountsAs(f0.lower()),
            Self::IsGeneric => deckmaste_core::SymbolPred::IsGeneric,
            Self::And(f0) => deckmaste_core::SymbolPred::And(f0.lower()),
            Self::Or(f0) => deckmaste_core::SymbolPred::Or(f0.lower()),
            Self::Not(f0) => deckmaste_core::SymbolPred::Not(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::ManaRider {
    type Target = deckmaste_core::ManaRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SpendOnly(f0) => deckmaste_core::ManaRider::SpendOnly(f0.lower()),
            Self::GrantOnSpend(f0) => deckmaste_core::ManaRider::GrantOnSpend(f0.lower()),
            Self::TriggerOnSpend(f0) => deckmaste_core::ManaRider::TriggerOnSpend(f0.lower()),
            Self::Persistent(f0) => deckmaste_core::ManaRider::Persistent(f0.lower()),
            Self::Snow => deckmaste_core::ManaRider::Snow,
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::ManaProduction {
    type Target = deckmaste_core::ManaProduction;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::WithRiders { mana, riders } => deckmaste_core::ManaProduction::WithRiders {
                mana: mana.lower(),
                riders: riders.lower(),
            },
            Self::Bare(f0) => deckmaste_core::ManaProduction::Bare(f0.lower()),
        }
    }
}

/// HAND-WRITTEN, not scaffolded.
///
/// `ManaCost` is a newtype over a PRIVATE `Arc<[ManaSymbol]>` (authoring's
/// `mana.rs`), so `self.0` is not reachable from this crate and the scaffold
/// generator correctly refused to emit an arm for it. This is still an identity
/// mapping — it just travels through the public `From` conversions on either
/// side of the newtype rather than through the field.
///
/// `ParseManaError` gets no arm at all: it is the `FromStr` error type, not
/// grammar, and is unreachable from any container.
impl Lower for deckmaste_authoring::ManaCost {
    type Target = deckmaste_core::ManaCost;
    fn lower(self) -> <Self as Lower>::Target {
        let symbols: std::sync::Arc<[deckmaste_authoring::ManaSymbol]> = self.into();
        symbols.lower().into()
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
    fn lowers_planar_face_blank() {
        assert_matches!(
            deckmaste_authoring::PlanarFace::Blank.lower(),
            deckmaste_core::PlanarFace::Blank
        );
    }

    #[test]
    fn lowers_planar_face_chaos() {
        assert_matches!(
            deckmaste_authoring::PlanarFace::Chaos.lower(),
            deckmaste_core::PlanarFace::Chaos
        );
    }

    #[test]
    fn lowers_planar_face_planeswalker() {
        assert_matches!(
            deckmaste_authoring::PlanarFace::Planeswalker.lower(),
            deckmaste_core::PlanarFace::Planeswalker
        );
    }

    #[test]
    fn lowers_mana_spec_any_color() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::AnyColor.lower(),
            deckmaste_core::ManaSpec::AnyColor
        );
    }

    #[test]
    fn lowers_mana_spec_one_of() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::OneOf([].into()).lower(),
            deckmaste_core::ManaSpec::OneOf(_)
        );
    }

    #[test]
    fn lowers_mana_spec_one_of_runs() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::OneOfRuns([].into()).lower(),
            deckmaste_core::ManaSpec::OneOfRuns(_)
        );
    }

    #[test]
    fn lowers_mana_spec_among_colors_of() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::AmongColorsOf(minimal_reference()).lower(),
            deckmaste_core::ManaSpec::AmongColorsOf(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_mana_spec_produced_by_event() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::ProducedByEvent.lower(),
            deckmaste_core::ManaSpec::ProducedByEvent
        );
    }

    #[test]
    fn lowers_mana_spec_specific() {
        assert_matches!(
            deckmaste_authoring::ManaSpec::Specific(minimal_color_or_colorless()).lower(),
            deckmaste_core::ManaSpec::Specific(deckmaste_core::ColorOrColorless::Colorless)
        );
    }

    #[test]
    fn lowers_simple_mana_symbol_generic() {
        assert_matches!(
            deckmaste_authoring::SimpleManaSymbol::Generic(0).lower(),
            deckmaste_core::SimpleManaSymbol::Generic(0)
        );
    }

    #[test]
    fn lowers_simple_mana_symbol_specific() {
        assert_matches!(
            deckmaste_authoring::SimpleManaSymbol::Specific(minimal_color_or_colorless()).lower(),
            deckmaste_core::SimpleManaSymbol::Specific(deckmaste_core::ColorOrColorless::Colorless)
        );
    }

    #[test]
    fn lowers_mana_symbol_variable() {
        assert_matches!(
            deckmaste_authoring::ManaSymbol::Variable.lower(),
            deckmaste_core::ManaSymbol::Variable
        );
    }

    #[test]
    fn lowers_mana_symbol_snow() {
        assert_matches!(
            deckmaste_authoring::ManaSymbol::Snow.lower(),
            deckmaste_core::ManaSymbol::Snow
        );
    }

    #[test]
    fn lowers_mana_symbol_hybrid() {
        assert_matches!(
            deckmaste_authoring::ManaSymbol::Hybrid(minimal_simple_mana_symbol(), minimal_color())
                .lower(),
            deckmaste_core::ManaSymbol::Hybrid(
                deckmaste_core::SimpleManaSymbol::Generic(0),
                deckmaste_core::Color::White
            )
        );
    }

    #[test]
    fn lowers_mana_symbol_phyrexian() {
        assert_matches!(
            deckmaste_authoring::ManaSymbol::Phyrexian(minimal_color(), None).lower(),
            deckmaste_core::ManaSymbol::Phyrexian(deckmaste_core::Color::White, None)
        );
    }

    #[test]
    fn lowers_mana_symbol_simple() {
        assert_matches!(
            deckmaste_authoring::ManaSymbol::Simple(minimal_simple_mana_symbol()).lower(),
            deckmaste_core::ManaSymbol::Simple(deckmaste_core::SimpleManaSymbol::Generic(0))
        );
    }

    #[test]
    fn lowers_symbol_pred_any_color() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::AnyColor.lower(),
            deckmaste_core::SymbolPred::AnyColor
        );
    }

    #[test]
    fn lowers_symbol_pred_any_type() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::AnyType.lower(),
            deckmaste_core::SymbolPred::AnyType
        );
    }

    #[test]
    fn lowers_symbol_pred_counts_as() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::CountsAs(minimal_color()).lower(),
            deckmaste_core::SymbolPred::CountsAs(deckmaste_core::Color::White)
        );
    }

    #[test]
    fn lowers_symbol_pred_is_generic() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::IsGeneric.lower(),
            deckmaste_core::SymbolPred::IsGeneric
        );
    }

    #[test]
    fn lowers_symbol_pred_and() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::And([].into()).lower(),
            deckmaste_core::SymbolPred::And(_)
        );
    }

    #[test]
    fn lowers_symbol_pred_or() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::Or([].into()).lower(),
            deckmaste_core::SymbolPred::Or(_)
        );
    }

    #[test]
    fn lowers_symbol_pred_not() {
        assert_matches!(
            deckmaste_authoring::SymbolPred::Not(std::sync::Arc::new(minimal_symbol_pred()))
                .lower(),
            deckmaste_core::SymbolPred::Not(_)
        );
    }

    #[test]
    fn lowers_mana_rider_spend_only() {
        assert_matches!(
            deckmaste_authoring::ManaRider::SpendOnly(minimal_predicate()).lower(),
            deckmaste_core::ManaRider::SpendOnly(deckmaste_core::Predicate::Kind(
                deckmaste_core::ObjectKind::Ability
            ))
        );
    }

    #[test]
    fn lowers_mana_rider_grant_on_spend() {
        assert_matches!(
            deckmaste_authoring::ManaRider::GrantOnSpend(std::sync::Arc::new(
                minimal_one_shot_effect()
            ))
            .lower(),
            deckmaste_core::ManaRider::GrantOnSpend(_)
        );
    }

    #[test]
    fn lowers_mana_rider_trigger_on_spend() {
        assert_matches!(
            deckmaste_authoring::ManaRider::TriggerOnSpend(std::sync::Arc::new(
                minimal_one_shot_effect()
            ))
            .lower(),
            deckmaste_core::ManaRider::TriggerOnSpend(_)
        );
    }

    #[test]
    fn lowers_mana_rider_persistent() {
        assert_matches!(
            deckmaste_authoring::ManaRider::Persistent(minimal_turn_marker()).lower(),
            deckmaste_core::ManaRider::Persistent(deckmaste_core::TurnMarker::EndOfTurn)
        );
    }

    #[test]
    fn lowers_mana_rider_snow() {
        assert_matches!(
            deckmaste_authoring::ManaRider::Snow.lower(),
            deckmaste_core::ManaRider::Snow
        );
    }

    #[test]
    fn lowers_mana_rider_expanded() {
        assert_matches!(
            deckmaste_authoring::ManaRider::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_mana_rider())
            })
            .lower(),
            deckmaste_core::ManaRider::SpendOnly(deckmaste_core::Predicate::Kind(
                deckmaste_core::ObjectKind::Ability
            ))
        );
    }

    #[test]
    fn lowers_mana_production_with_riders() {
        assert_matches!(
            deckmaste_authoring::ManaProduction::WithRiders {
                mana: minimal_mana_spec(),
                riders: [].into()
            }
            .lower(),
            deckmaste_core::ManaProduction::WithRiders {
                mana: deckmaste_core::ManaSpec::AnyColor,
                riders: _
            }
        );
    }

    #[test]
    fn lowers_mana_production_bare() {
        assert_matches!(
            deckmaste_authoring::ManaProduction::Bare(minimal_mana_spec()).lower(),
            deckmaste_core::ManaProduction::Bare(deckmaste_core::ManaSpec::AnyColor)
        );
    }

    #[test]
    fn lowers_mana_cost() {
        assert_lowers(deckmaste_authoring::ManaCost::from(std::sync::Arc::<
            [deckmaste_authoring::ManaSymbol],
        >::from([])));
    }
}
