//! `event` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::PhaseKind {
    type Target = deckmaste_core::PhaseKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning => deckmaste_core::PhaseKind::Beginning,
            Self::PrecombatMain => deckmaste_core::PhaseKind::PrecombatMain,
            Self::Combat => deckmaste_core::PhaseKind::Combat,
            Self::PostcombatMain => deckmaste_core::PhaseKind::PostcombatMain,
            Self::Ending => deckmaste_core::PhaseKind::Ending,
        }
    }
}

impl Lower for deckmaste_authoring::PhaseStep {
    type Target = deckmaste_core::PhaseStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning(f0) => deckmaste_core::PhaseStep::Beginning(f0.lower()),
            Self::PrecombatMain => deckmaste_core::PhaseStep::PrecombatMain,
            Self::Combat(f0) => deckmaste_core::PhaseStep::Combat(f0.lower()),
            Self::PostcombatMain => deckmaste_core::PhaseStep::PostcombatMain,
            Self::Ending(f0) => deckmaste_core::PhaseStep::Ending(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::BeginningStep {
    type Target = deckmaste_core::BeginningStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Untap => deckmaste_core::BeginningStep::Untap,
            Self::Upkeep => deckmaste_core::BeginningStep::Upkeep,
            Self::Draw => deckmaste_core::BeginningStep::Draw,
        }
    }
}

impl Lower for deckmaste_authoring::CombatStep {
    type Target = deckmaste_core::CombatStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::BeginningOfCombat => deckmaste_core::CombatStep::BeginningOfCombat,
            Self::DeclareAttackers => deckmaste_core::CombatStep::DeclareAttackers,
            Self::DeclareBlockers => deckmaste_core::CombatStep::DeclareBlockers,
            Self::FirstCombatDamage => deckmaste_core::CombatStep::FirstCombatDamage,
            Self::CombatDamage => deckmaste_core::CombatStep::CombatDamage,
            Self::EndOfCombat => deckmaste_core::CombatStep::EndOfCombat,
        }
    }
}

impl Lower for deckmaste_authoring::EndingStep {
    type Target = deckmaste_core::EndingStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::End => deckmaste_core::EndingStep::End,
            Self::Cleanup => deckmaste_core::EndingStep::Cleanup,
        }
    }
}

impl Lower for deckmaste_authoring::WhoseTurn {
    type Target = deckmaste_core::WhoseTurn;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Your => deckmaste_core::WhoseTurn::Your,
            Self::EachPlayers => deckmaste_core::WhoseTurn::EachPlayers,
            Self::AnOpponents => deckmaste_core::WhoseTurn::AnOpponents,
        }
    }
}

impl Lower for deckmaste_authoring::StateChange {
    type Target = deckmaste_core::StateChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => deckmaste_core::StateChange::Tapped,
            Self::Untapped => deckmaste_core::StateChange::Untapped,
            Self::Phased(f0) => deckmaste_core::StateChange::Phased(f0.lower()),
            Self::TurnedFace(f0) => deckmaste_core::StateChange::TurnedFace(f0.lower()),
            Self::Transformed => deckmaste_core::StateChange::Transformed,
        }
    }
}

impl Lower for deckmaste_authoring::Agency {
    type Target = deckmaste_core::Agency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CostPayment => deckmaste_core::Agency::CostPayment,
            Self::AttackDeclaration => deckmaste_core::Agency::AttackDeclaration,
            Self::EffectInstruction => deckmaste_core::Agency::EffectInstruction,
            Self::TurnBasedAction => deckmaste_core::Agency::TurnBasedAction,
            Self::StateBasedAction => deckmaste_core::Agency::StateBasedAction,
            Self::ManaAbilityResolution => deckmaste_core::Agency::ManaAbilityResolution,
            Self::SpecialAction => deckmaste_core::Agency::SpecialAction,
        }
    }
}

impl Lower for deckmaste_authoring::VerbName {
    type Target = deckmaste_core::VerbName;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::VerbName(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::CausePattern {
    type Target = deckmaste_core::CausePattern;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CausePattern {
            verb: self.verb.lower(),
            agency: self.agency.lower(),
            agent: self.agent.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Cause {
    type Target = deckmaste_core::Cause;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Cause(f0) => deckmaste_core::Cause::Cause(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::EventFilter {
    type Target = deckmaste_core::EventFilter;
    // One arm per variant of the grammar's widest enum, several with many named
    // fields. Splitting the match into helpers is exactly what must NOT happen
    // here: the single exhaustive match is the ratchet that turns a new variant
    // on either side into a build error.
    #[expect(
        clippy::too_many_lines,
        reason = "exhaustive one-arm-per-variant map; splitting it would hide the exhaustiveness check"
    )]
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ZoneChange {
                what,
                from,
                to,
                cause,
            } => deckmaste_core::EventFilter::ZoneChange {
                what: what.lower(),
                from: from.lower(),
                to: to.lower(),
                cause: cause.lower(),
            },
            Self::Damage {
                source,
                to,
                combat,
                amount,
            } => deckmaste_core::EventFilter::Damage {
                source: source.lower(),
                to: to.lower(),
                combat: combat.lower(),
                amount: amount.lower(),
            },
            Self::LifeGained { who, amount } => deckmaste_core::EventFilter::LifeGained {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::LifeLost { who, amount } => deckmaste_core::EventFilter::LifeLost {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Drawn { who, amount } => deckmaste_core::EventFilter::Drawn {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Act {
                verb,
                who,
                on,
                cause,
            } => deckmaste_core::EventFilter::Act {
                verb: verb.lower(),
                who: who.lower(),
                on: on.lower(),
                cause: cause.lower(),
            },
            Self::CounterPlaced {
                kind,
                on,
                amount,
                cause,
            } => deckmaste_core::EventFilter::CounterPlaced {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::CounterRemoved {
                kind,
                on,
                amount,
                cause,
            } => deckmaste_core::EventFilter::CounterRemoved {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::Cast { who, what } => deckmaste_core::EventFilter::Cast {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Copied { who, what } => deckmaste_core::EventFilter::Copied {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Played { who, what } => deckmaste_core::EventFilter::Played {
                who: who.lower(),
                what: what.lower(),
            },
            Self::ActivatedAb { who, what } => deckmaste_core::EventFilter::ActivatedAb {
                who: who.lower(),
                what: what.lower(),
            },
            Self::AttackDeclared { by, against } => deckmaste_core::EventFilter::AttackDeclared {
                by: by.lower(),
                against: against.lower(),
            },
            Self::BlockDeclared { by, of } => deckmaste_core::EventFilter::BlockDeclared {
                by: by.lower(),
                of: of.lower(),
            },
            Self::Attached { what, to } => deckmaste_core::EventFilter::Attached {
                what: what.lower(),
                to: to.lower(),
            },
            Self::StateBecame { of, becomes, cause } => deckmaste_core::EventFilter::StateBecame {
                of: of.lower(),
                becomes: becomes.lower(),
                cause: cause.lower(),
            },
            Self::BecomesTarget { what, by, source } => {
                deckmaste_core::EventFilter::BecomesTarget {
                    what: what.lower(),
                    by: by.lower(),
                    source: source.lower(),
                }
            }
            Self::StepBegins { at, whose } => deckmaste_core::EventFilter::StepBegins {
                at: at.lower(),
                whose: whose.lower(),
            },
            Self::ControlChanged { of, to } => deckmaste_core::EventFilter::ControlChanged {
                of: of.lower(),
                to: to.lower(),
            },
            Self::DesignationChanged { name, of } => {
                deckmaste_core::EventFilter::DesignationChanged {
                    name: name.lower(),
                    of: of.lower(),
                }
            }
            Self::TokenCreated { what, by } => deckmaste_core::EventFilter::TokenCreated {
                what: what.lower(),
                by: by.lower(),
            },
            Self::Shuffled { by } => deckmaste_core::EventFilter::Shuffled { by: by.lower() },
            Self::Revealed { what } => deckmaste_core::EventFilter::Revealed { what: what.lower() },
            Self::Used { of } => deckmaste_core::EventFilter::Used { of: of.lower() },
            Self::CoinFlipped { by, won } => deckmaste_core::EventFilter::CoinFlipped {
                by: by.lower(),
                won: won.lower(),
            },
            Self::DiceRolled { by } => deckmaste_core::EventFilter::DiceRolled { by: by.lower() },
            Self::TapForMana { what, by } => deckmaste_core::EventFilter::TapForMana {
                what: what.lower(),
                by: by.lower(),
            },
            Self::RollPlanarDie { by, face } => deckmaste_core::EventFilter::RollPlanarDie {
                by: by.lower(),
                face: face.lower(),
            },
            Self::BecameDay => deckmaste_core::EventFilter::BecameDay,
            Self::BecameNight => deckmaste_core::EventFilter::BecameNight,
            Self::AllOf(f0) => deckmaste_core::EventFilter::AllOf(f0.lower()),
            Self::OneOf(f0) => deckmaste_core::EventFilter::OneOf(f0.lower()),
            Self::Not(f0) => deckmaste_core::EventFilter::Not(f0.lower()),
            Self::OneOrMore(f0) => deckmaste_core::EventFilter::OneOrMore(f0.lower()),
            Self::Nth { n, of, within } => deckmaste_core::EventFilter::Nth {
                n: n.lower(),
                of: of.lower(),
                within: within.lower(),
            },
            Self::When(f0, f1) => deckmaste_core::EventFilter::When(f0.lower(), f1.lower()),
            Self::Within(f0, f1) => deckmaste_core::EventFilter::Within(f0.lower(), f1.lower()),
            Self::Before(f0) => deckmaste_core::EventFilter::Before(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::EventFilter::Expanded(f0.lower()),
        }
    }
}
