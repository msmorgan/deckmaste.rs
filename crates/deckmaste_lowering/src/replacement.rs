//! `replacement` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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
            Self::Expanded(f0) => deckmaste_core::Replacement::Expanded(f0.lower()),
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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_replacement_instead() {
        assert_lowers_debug(deckmaste_authoring::Replacement::Instead {
            would: minimal_event_filter(),
            instead: minimal_one_shot_effect(),
        });
        assert_matches!(
            deckmaste_authoring::Replacement::Instead {
                would: minimal_event_filter(),
                instead: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::Replacement::Instead { .. }
        );
    }

    #[test]
    fn lowers_replacement_skip() {
        assert_lowers_debug(deckmaste_authoring::Replacement::Skip {
            what: minimal_phase_step(),
        });
        assert_matches!(
            deckmaste_authoring::Replacement::Skip {
                what: minimal_phase_step()
            }
            .lower(),
            deckmaste_core::Replacement::Skip { .. }
        );
    }

    #[test]
    fn lowers_replacement_also() {
        assert_lowers_debug(deckmaste_authoring::Replacement::Also {
            would: minimal_event_filter(),
            also: minimal_one_shot_effect(),
        });
        assert_matches!(
            deckmaste_authoring::Replacement::Also {
                would: minimal_event_filter(),
                also: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::Replacement::Also { .. }
        );
    }

    #[test]
    fn lowers_replacement_expanded() {
        assert_lowers_debug(deckmaste_authoring::Replacement::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_replacement()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::Replacement::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_replacement())
            })
            .lower(),
            deckmaste_core::Replacement::Expanded(..)
        );
    }

    #[test]
    fn lowers_prevention_prevent_next() {
        assert_lowers(deckmaste_authoring::Prevention::PreventNext {
            n: minimal_count(),
            from: minimal_predicate(),
            to: minimal_predicate(),
            duration: None,
        });
        assert_matches!(
            deckmaste_authoring::Prevention::PreventNext {
                n: minimal_count(),
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventNext { .. }
        );
    }

    #[test]
    fn lowers_prevention_prevent_next_instance() {
        assert_lowers(deckmaste_authoring::Prevention::PreventNextInstance {
            from: minimal_predicate(),
            to: minimal_predicate(),
        });
        assert_matches!(
            deckmaste_authoring::Prevention::PreventNextInstance {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::Prevention::PreventNextInstance { .. }
        );
    }

    #[test]
    fn lowers_prevention_prevent_all() {
        assert_lowers(deckmaste_authoring::Prevention::PreventAll {
            from: minimal_predicate(),
            to: minimal_predicate(),
            duration: None,
        });
        assert_matches!(
            deckmaste_authoring::Prevention::PreventAll {
                from: minimal_predicate(),
                to: minimal_predicate(),
                duration: None
            }
            .lower(),
            deckmaste_core::Prevention::PreventAll { .. }
        );
    }
}
