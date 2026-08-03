//! `keyword` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::KeywordRef {
    type Target = deckmaste_core::KeywordRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordRef(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::ParamShape {
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

impl Lower for deckmaste_authoring::KeywordDecl {
    type Target = deckmaste_core::KeywordDecl;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordDecl {
            name: self.name.lower(),
            shape: self.shape.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::KeywordAbility {
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
            Self::Expanded(f0) => deckmaste_core::KeywordAbility::Expanded(f0.lower()),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_keyword_ref() {
        assert_lowers_debug(deckmaste_authoring::KeywordRef("X".into()));
    }

    #[test]
    fn lowers_param_shape_none() {
        assert_lowers(deckmaste_authoring::ParamShape::None);
    }

    #[test]
    fn lowers_param_shape_counted() {
        assert_lowers(deckmaste_authoring::ParamShape::Counted);
    }

    #[test]
    fn lowers_param_shape_costed() {
        assert_lowers(deckmaste_authoring::ParamShape::Costed);
    }

    #[test]
    fn lowers_param_shape_counted_cost() {
        assert_lowers(deckmaste_authoring::ParamShape::CountedCost);
    }

    #[test]
    fn lowers_param_shape_predicated() {
        assert_lowers(deckmaste_authoring::ParamShape::Predicated);
    }

    #[test]
    fn lowers_param_shape_predicated_costed() {
        assert_lowers(deckmaste_authoring::ParamShape::PredicatedCosted);
    }

    #[test]
    fn lowers_param_shape_named() {
        assert_lowers(deckmaste_authoring::ParamShape::Named);
    }

    #[test]
    fn lowers_keyword_decl() {
        assert_lowers(deckmaste_authoring::KeywordDecl {
            name: "X".into(),
            shape: minimal_param_shape(),
        });
    }

    #[test]
    fn lowers_keyword_ability_first_strike() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::FirstStrike);
    }

    #[test]
    fn lowers_keyword_ability_double_strike() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::DoubleStrike);
    }

    #[test]
    fn lowers_keyword_ability_deathtouch() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::Deathtouch);
    }

    #[test]
    fn lowers_keyword_ability_trample() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::Trample);
    }

    #[test]
    fn lowers_keyword_ability_vigilance() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::Vigilance);
    }

    #[test]
    fn lowers_keyword_ability_composite() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::Composite {
            name: "X".into(),
            abilities: Vec::new(),
        });
    }

    #[test]
    fn lowers_keyword_ability_expanded() {
        assert_lowers_debug(deckmaste_authoring::KeywordAbility::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_keyword_ability()),
            },
        ));
    }
}
