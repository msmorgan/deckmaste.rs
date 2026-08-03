//! `effect` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::OneShotEffect {
    type Target = deckmaste_core::OneShotEffect;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Act(f0) => deckmaste_core::OneShotEffect::Act(f0.lower()),
            Self::Sequentially(f0) => deckmaste_core::OneShotEffect::Sequentially(f0.lower()),
            Self::Simultaneously(f0) => deckmaste_core::OneShotEffect::Simultaneously(f0.lower()),
            Self::Continuously(f0) => deckmaste_core::OneShotEffect::Continuously(f0.lower()),
            Self::Until(f0, f1) => deckmaste_core::OneShotEffect::Until(f0.lower(), f1.lower()),
            Self::Label(f0) => deckmaste_core::OneShotEffect::Label(f0.lower()),
            Self::SeparatePiles(f0) => deckmaste_core::OneShotEffect::SeparatePiles(f0.lower()),
            Self::ChoosePile(f0) => deckmaste_core::OneShotEffect::ChoosePile(f0.lower()),
            Self::May(f0) => deckmaste_core::OneShotEffect::May(f0.lower()),
            Self::If(f0) => deckmaste_core::OneShotEffect::If(f0.lower()),
            Self::AdditionalCost(f0) => deckmaste_core::OneShotEffect::AdditionalCost(f0.lower()),
            Self::Each(f0) => deckmaste_core::OneShotEffect::Each(f0.lower()),
            Self::With(f0) => deckmaste_core::OneShotEffect::With(f0.lower()),
            Self::Distribute(f0) => deckmaste_core::OneShotEffect::Distribute(f0.lower()),
            Self::Noting(f0) => deckmaste_core::OneShotEffect::Noting(f0.lower()),
            Self::Delayed(f0) => deckmaste_core::OneShotEffect::Delayed(f0.lower()),
            Self::Reflexive(f0) => deckmaste_core::OneShotEffect::Reflexive(f0.lower()),
            Self::Modal(f0) => deckmaste_core::OneShotEffect::Modal(f0.lower()),
            Self::Targeted(f0) => deckmaste_core::OneShotEffect::Targeted(f0.lower()),
            Self::Repeat(f0, f1) => deckmaste_core::OneShotEffect::Repeat(f0.lower(), f1.lower()),
            Self::Batch(f0, f1) => deckmaste_core::OneShotEffect::Batch(f0.lower(), f1.lower()),
            Self::RevealUntil(f0) => deckmaste_core::OneShotEffect::RevealUntil(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::OneShotEffect::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Continuously {
    type Target = deckmaste_core::Continuously;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Continuously {
            effect: self.effect.lower(),
            duration: self.duration.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Targeted {
    type Target = deckmaste_core::Targeted;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Targeted {
            targets: self.targets.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::May {
    type Target = deckmaste_core::May;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::May {
            who: self.who.lower(),
            effect: self.effect.lower(),
            if_did: self.if_did.lower(),
            if_not: self.if_not.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::If {
    type Target = deckmaste_core::If;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::If {
            condition: self.condition.lower(),
            then: self.then.lower(),
            otherwise: self.otherwise.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Noting {
    type Target = deckmaste_core::Noting;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Noting {
            key: self.key.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::AdditionalCost {
    type Target = deckmaste_core::AdditionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::AdditionalCost {
            pay: self.pay.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Each {
    type Target = deckmaste_core::Each;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Each {
            binder: self.binder.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::With {
    type Target = deckmaste_core::With;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::With {
            binder: self.binder.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Distribute {
    type Target = deckmaste_core::Distribute;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Distribute {
            amount: self.amount.lower(),
            binder: self.binder.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::RevealUntil {
    type Target = deckmaste_core::RevealUntil;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::RevealUntil {
            whose: self.whose.lower(),
            matches: self.matches.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Modal {
    type Target = deckmaste_core::Modal;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Modal {
            choose: self.choose.lower(),
            modes: self.modes.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Label {
    type Target = deckmaste_core::Label;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Label {
            r#as: self.r#as.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::SeparatePiles {
    type Target = deckmaste_core::SeparatePiles;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SeparatePiles {
            group: self.group.lower(),
            into: self.into.lower(),
            by: self.by.lower(),
            note: self.note.lower(),
            then: self.then.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::ChoosePile {
    type Target = deckmaste_core::ChoosePile;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ChoosePile {
            from: self.from.lower(),
            by: self.by.lower(),
            random: self.random.lower(),
            then: self.then.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::PileSource {
    type Target = deckmaste_core::PileSource;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Labels(f0) => deckmaste_core::PileSource::Labels(f0.lower()),
            Self::Noted { note, of } => deckmaste_core::PileSource::Noted {
                note: note.lower(),
                of: of.lower(),
            },
        }
    }
}
