//! `replacement` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for deckmaste_semantics::Replacement {
    type Target = deckmaste_core::Replacement;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Instead { would, instead } => deckmaste_core::Replacement::Instead {
                would: would.lower(),
                instead: instead.lower(),
            },
            Self::Skip { what } => deckmaste_core::Replacement::Skip { what: what.lower() },
            Self::Also { would, also } => deckmaste_core::Replacement::Also {
                would: would.lower(),
                also: also.lower(),
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
    fn lowers_replacement_skip() {
        assert_matches!(
            deckmaste_semantics::Replacement::Skip {
                what: minimal_phase_step()
            }
            .lower(),
            deckmaste_core::Replacement::Skip {
                what: deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap)
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_next() {
        assert_matches!(
            deckmaste_semantics::Prevention::PreventNext {
                n: minimal_count(),
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventNext {
                n: deckmaste_core::Count::Literal(0),
                from: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                duration: None
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_next_instance() {
        assert_matches!(
            deckmaste_semantics::Prevention::PreventNextInstance {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::Prevention::PreventNextInstance {
                from: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_all() {
        assert_matches!(
            deckmaste_semantics::Prevention::PreventAll {
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventAll {
                from: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                duration: None
            }
        );
    }

    #[test]
    fn lowers_replacement_also() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::Replacement::Also {
                would: minimal_event_filter(),
                also: minimal_one_shot_effect(),
            }
            .lower()
        });
        let deckmaste_core::Replacement::Also { would, also } = &lowered else {
            panic!("an `Also` replacement lowers to an `Also` replacement");
        };
        assert_matches!(
            would,
            deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
        assert!(is_minimal_lowered_effect(also));
    }

    /// A macro invocation's provenance does not cross `lower`: the expansion
    /// wrapper is erased and only its value survives.
    #[test]
    fn lowers_replacement_expanded() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::Replacement::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_replacement()),
            })
            .lower()
        });
        let deckmaste_core::Replacement::Instead { would, instead } = &lowered else {
            panic!("the expansion erases to its wrapped `Instead` replacement");
        };
        assert_matches!(
            would,
            deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
        assert!(is_minimal_lowered_effect(instead));
    }

    #[test]
    fn lowers_replacement_instead() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::Replacement::Instead {
                would: minimal_event_filter(),
                instead: minimal_one_shot_effect(),
            }
            .lower()
        });
        let deckmaste_core::Replacement::Instead { would, instead } = &lowered else {
            panic!("an `Instead` replacement lowers to an `Instead` replacement");
        };
        assert_matches!(
            would,
            deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
        assert!(is_minimal_lowered_effect(instead));
    }
}

impl Lower for deckmaste_semantics::Prevention {
    type Target = deckmaste_core::Prevention;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::PreventNext {
                n,
                from,
                to,
                duration,
            } => deckmaste_core::Prevention::PreventNext {
                n: n.lower(),
                from: from.lower(),
                to: to.lower(),
                duration: duration.lower(),
            },
            Self::PreventNextInstance { from, to } => {
                deckmaste_core::Prevention::PreventNextInstance {
                    from: from.lower(),
                    to: to.lower(),
                }
            }
            Self::PreventAll { from, to, duration } => deckmaste_core::Prevention::PreventAll {
                from: from.lower(),
                to: to.lower(),
                duration: duration.lower(),
            },
        }
    }
}
