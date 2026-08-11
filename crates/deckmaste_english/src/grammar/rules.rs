use super::EnglishLexicalSlot;
use super::Expected;
use super::HashMap;
use super::Nonterminal;
use super::ParseCost;
use super::Rule;
use super::RuleId;
#[cfg(test)]
use crate::construction::ConstructionId;
use crate::construction::ProductionId;

#[derive(Debug, Clone, Copy)]
pub(super) enum RuleImpl {
    Generated(GeneratedRuleRef),
    GeneratedAux(GeneratedAuxRuleRef),
}

#[cfg(test)]
fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// One generated production's home: the group's declaration data plus the
/// construction/form indices inside it. Indices, not references, so the type
/// stays `Copy` without self-referential borrows.
#[derive(Debug, Clone, Copy)]
pub(super) struct GeneratedRuleRef {
    pub(super) group: &'static deckmaste_construction_compiler::runtime::GroupData,
    pub(super) construction: usize,
    pub(super) form: usize,
    /// Bit `i` records that sequence-valued surface atom `i` used its
    /// one-or-more helper. A clear bit means the field is the empty sequence.
    pub(super) sequence_atoms: u64,
    pub(super) context: GeneratedRuleContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GeneratedRuleContext {
    Value,
    ObjectGap,
    ReducedRecipientPassive,
    SharedPreposition,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum GeneratedAuxRuleRef {
    Transparent,
    ElementStruct {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
        present_fields: u64,
    },
    ElementVariant {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
        variant: usize,
    },
    SequenceSeed {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
    },
    SequenceExtend {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
    },
}

#[derive(Default)]
pub(super) struct RuleBuilder {
    registrations: Vec<RuleRegistration>,
}

struct RuleRegistration {
    rule_impl: RuleImpl,
    rule: Rule<Nonterminal, EnglishLexicalSlot>,
}

pub(super) struct RuleBook {
    pub(super) rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    pub(super) impls: Vec<RuleImpl>,
    pub(super) rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RegistrationOrder {
    Normal,
    #[cfg(test)]
    Reversed,
    #[cfg(test)]
    FixedShuffle,
}

impl RuleBuilder {
    /// Generated productions carry EXPLICIT ordinals from their declaration
    /// (`form … @ N`); nothing here may consult `next_ordinals`, whose
    /// incremental numbering generated families must never depend on.
    pub(super) fn add_generated(
        &mut self,
        rule_impl: RuleImpl,
        production: ProductionId,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
    ) {
        self.add_generated_with_cost(rule_impl, production, lhs, rhs, ParseCost::default());
    }

    pub(super) fn add_generated_with_cost(
        &mut self,
        rule_impl: RuleImpl,
        production: ProductionId,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
        local_cost: ParseCost,
    ) {
        self.registrations.push(RuleRegistration {
            rule_impl,
            rule: Rule {
                production,
                lhs,
                rhs: rhs.into_iter().collect(),
                local_cost,
            },
        });
    }

    pub(super) fn finish(mut self, order: RegistrationOrder) -> RuleBook {
        reorder_registrations(&mut self.registrations, order);
        let mut rules = Vec::with_capacity(self.registrations.len());
        let mut impls = Vec::with_capacity(self.registrations.len());
        let mut rules_by_lhs = HashMap::<Nonterminal, Vec<RuleId>>::new();
        for registration in self.registrations {
            let id = RuleId::new(rules.len());
            rules_by_lhs
                .entry(registration.rule.lhs)
                .or_default()
                .push(id);
            rules.push(registration.rule);
            impls.push(registration.rule_impl);
        }
        RuleBook {
            rules,
            impls,
            rules_by_lhs,
        }
    }
}

fn reorder_registrations(registrations: &mut [RuleRegistration], order: RegistrationOrder) {
    if order == RegistrationOrder::Normal {
        return;
    }
    #[cfg(not(test))]
    let _ = registrations;
    #[cfg(test)]
    {
        let mut families = Vec::<ConstructionId>::new();
        for registration in registrations.iter() {
            let id = registration.rule.production.construction;
            if !families.contains(&id) {
                families.push(id);
            }
        }
        match order {
            RegistrationOrder::Normal => unreachable!(),
            RegistrationOrder::Reversed => families.reverse(),
            RegistrationOrder::FixedShuffle => {
                let original = families.clone();
                families.sort_unstable_by_key(|id| (fnv1a(id.as_str()), *id));
                if families == original && families.len() > 1 {
                    families.rotate_left(1);
                }
            }
        }
        let ranks = families
            .into_iter()
            .enumerate()
            .map(|(rank, id)| (id, rank))
            .collect::<HashMap<_, _>>();
        registrations.sort_by_key(|registration| ranks[&registration.rule.production.construction]);
    }
}
