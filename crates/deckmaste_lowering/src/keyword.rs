//! `keyword` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::KeywordRef {
    type Target = deckmaste_core::KeywordRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordRef(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::ParamShape {
    type Target = deckmaste_core::ParamShape;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => deckmaste_core::ParamShape::None,
            Self::Counted => deckmaste_core::ParamShape::Counted,
            Self::Costed => deckmaste_core::ParamShape::Costed,
            Self::CountedCost => deckmaste_core::ParamShape::CountedCost,
            Self::Predicated => deckmaste_core::ParamShape::Predicated,
            Self::PredicatedCosted => deckmaste_core::ParamShape::PredicatedCosted,
            Self::Named => deckmaste_core::ParamShape::Named,
        }
    }
}

impl Lower for deckmaste_semantics::KeywordDecl {
    type Target = deckmaste_core::KeywordDecl;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordDecl {
            name: self.name.lower(),
            shape: self.shape.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::KeywordAbility {
    type Target = deckmaste_core::KeywordAbility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FirstStrike => deckmaste_core::KeywordAbility::FirstStrike,
            Self::DoubleStrike => deckmaste_core::KeywordAbility::DoubleStrike,
            Self::Deathtouch => deckmaste_core::KeywordAbility::Deathtouch,
            Self::Trample => deckmaste_core::KeywordAbility::Trample,
            Self::Vigilance => deckmaste_core::KeywordAbility::Vigilance,
            Self::Composite { name, abilities } => deckmaste_core::KeywordAbility::Composite {
                name: name.lower(),
                abilities: abilities.lower(),
            },
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

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_keyword_ref() {
        assert_matches!(
            deckmaste_semantics::KeywordRef("X".into()).lower(),
            deckmaste_core::KeywordRef(_)
        );
    }

    #[test]
    fn lowers_param_shape_none() {
        assert_matches!(
            deckmaste_semantics::ParamShape::None.lower(),
            deckmaste_core::ParamShape::None
        );
    }

    #[test]
    fn lowers_param_shape_counted() {
        assert_matches!(
            deckmaste_semantics::ParamShape::Counted.lower(),
            deckmaste_core::ParamShape::Counted
        );
    }

    #[test]
    fn lowers_param_shape_costed() {
        assert_matches!(
            deckmaste_semantics::ParamShape::Costed.lower(),
            deckmaste_core::ParamShape::Costed
        );
    }

    #[test]
    fn lowers_param_shape_counted_cost() {
        assert_matches!(
            deckmaste_semantics::ParamShape::CountedCost.lower(),
            deckmaste_core::ParamShape::CountedCost
        );
    }

    #[test]
    fn lowers_param_shape_predicated() {
        assert_matches!(
            deckmaste_semantics::ParamShape::Predicated.lower(),
            deckmaste_core::ParamShape::Predicated
        );
    }

    #[test]
    fn lowers_param_shape_predicated_costed() {
        assert_matches!(
            deckmaste_semantics::ParamShape::PredicatedCosted.lower(),
            deckmaste_core::ParamShape::PredicatedCosted
        );
    }

    #[test]
    fn lowers_param_shape_named() {
        assert_matches!(
            deckmaste_semantics::ParamShape::Named.lower(),
            deckmaste_core::ParamShape::Named
        );
    }

    #[test]
    fn lowers_keyword_decl() {
        assert_matches!(
            deckmaste_semantics::KeywordDecl {
                name: "X".into(),
                shape: minimal_param_shape()
            }
            .lower(),
            deckmaste_core::KeywordDecl {
                name: _,
                shape: deckmaste_core::ParamShape::None
            }
        );
    }

    #[test]
    fn lowers_keyword_ability_first_strike() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::FirstStrike.lower(),
            deckmaste_core::KeywordAbility::FirstStrike
        );
    }

    #[test]
    fn lowers_keyword_ability_double_strike() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::DoubleStrike.lower(),
            deckmaste_core::KeywordAbility::DoubleStrike
        );
    }

    #[test]
    fn lowers_keyword_ability_deathtouch() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::Deathtouch.lower(),
            deckmaste_core::KeywordAbility::Deathtouch
        );
    }

    #[test]
    fn lowers_keyword_ability_trample() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::Trample.lower(),
            deckmaste_core::KeywordAbility::Trample
        );
    }

    #[test]
    fn lowers_keyword_ability_vigilance() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::Vigilance.lower(),
            deckmaste_core::KeywordAbility::Vigilance
        );
    }

    #[test]
    fn lowers_keyword_ability_composite() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::Composite {
                name: "X".into(),
                abilities: Vec::new()
            }
            .lower(),
            deckmaste_core::KeywordAbility::Composite {
                name: _,
                abilities: _
            }
        );
    }

    #[test]
    fn lowers_keyword_ability_expanded() {
        assert_matches!(
            deckmaste_semantics::KeywordAbility::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_keyword_ability())
            })
            .lower(),
            deckmaste_core::KeywordAbility::FirstStrike
        );
    }
}
