//! `replacement` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Replacement {
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
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Prevention {
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
    fn lowers_replacement_instead() {
        assert_matches!(
            deckmaste_authoring::Replacement::Instead {
                would: minimal_event_filter(),
                instead: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::Replacement::Instead {
                would: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                    from: None,
                    to: None,
                    cause: None
                },
                instead: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_replacement_skip() {
        assert_matches!(
            deckmaste_authoring::Replacement::Skip {
                what: minimal_phase_step()
            }
            .lower(),
            deckmaste_core::Replacement::Skip {
                what: deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap)
            }
        );
    }

    #[test]
    fn lowers_replacement_also() {
        assert_matches!(
            deckmaste_authoring::Replacement::Also {
                would: minimal_event_filter(),
                also: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::Replacement::Also {
                would: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                    from: None,
                    to: None,
                    cause: None
                },
                also: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_replacement_expanded() {
        assert_matches!(
            deckmaste_authoring::Replacement::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_replacement())
            })
            .lower(),
            deckmaste_core::Replacement::Instead {
                would: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                    from: None,
                    to: None,
                    cause: None
                },
                instead: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_next() {
        assert_matches!(
            deckmaste_authoring::Prevention::PreventNext {
                n: minimal_count(),
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventNext {
                n: deckmaste_core::Count::X,
                from: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                to: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                duration: None
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_next_instance() {
        assert_matches!(
            deckmaste_authoring::Prevention::PreventNextInstance {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::Prevention::PreventNextInstance {
                from: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                to: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_prevention_prevent_all() {
        assert_matches!(
            deckmaste_authoring::Prevention::PreventAll {
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventAll {
                from: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                to: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                duration: None
            }
        );
    }
}
