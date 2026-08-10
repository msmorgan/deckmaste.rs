//! `ability` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::SpellAbility {
    type Target = deckmaste_core::SpellAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SpellAbility {
            ability_word: self.ability_word.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::ActivatedAbility {
    type Target = deckmaste_core::ActivatedAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ActivatedAbility {
            ability_word: self.ability_word.lower(),
            cost: self.cost.lower(),
            from: self.from.lower(),
            window: self.window.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::UseLimit {
    type Target = deckmaste_core::UseLimit;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::OncePerTurn => deckmaste_core::UseLimit::OncePerTurn,
            Self::OncePerGame => deckmaste_core::UseLimit::OncePerGame,
            Self::LoyaltyOncePerTurn => deckmaste_core::UseLimit::LoyaltyOncePerTurn,
        }
    }
}

impl Lower for deckmaste_semantics::TriggeredAbility {
    type Target = deckmaste_core::TriggeredAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TriggeredAbility {
            ability_word: self.ability_word.lower(),
            event: self.event.lower(),
            from: self.from.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            where_x: self.where_x.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::ChooseSpec {
    type Target = deckmaste_core::ChooseSpec;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ChooseSpec {
            count: self.count.lower(),
            up_to: self.up_to.lower(),
            repeats: self.repeats.lower(),
            chooser: self.chooser.lower(),
            rider: self.rider.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::ModalCostRider {
    type Target = deckmaste_core::ModalCostRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Entwine(f0) => deckmaste_core::ModalCostRider::Entwine(f0.lower()),
            Self::Escalate(f0) => deckmaste_core::ModalCostRider::Escalate(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::Mode {
    type Target = deckmaste_core::Mode;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Mode {
            effect: self.effect.lower(),
            cost: self.cost.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Ability {
    type Target = deckmaste_core::Ability;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Static(f0) => deckmaste_core::Ability::Static(f0.lower()),
            Self::Activated(f0) => classify_activated(f0.lower()),
            Self::Triggered(f0) => classify_triggered(f0.lower()),
            Self::Spell(f0) => deckmaste_core::Ability::Spell(f0.lower()),
            Self::Keyword(f0) => deckmaste_core::Ability::Keyword(f0.lower()),
            Self::Innate(f0) => deckmaste_core::Ability::Innate(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

#[derive(Clone, Copy)]
struct ManaFacts {
    adds_mana: bool,
    targetless: bool,
    library_safe: bool,
}

impl ManaFacts {
    const NEUTRAL: Self = Self {
        adds_mana: false,
        targetless: true,
        library_safe: true,
    };

    fn merge(self, other: Self) -> Self {
        Self {
            adds_mana: self.adds_mana || other.adds_mana,
            targetless: self.targetless && other.targetless,
            library_safe: self.library_safe && other.library_safe,
        }
    }

    fn mode_class(self) -> deckmaste_core::ManaModeClass {
        deckmaste_core::ManaModeClass {
            adds_mana: self.adds_mana,
            targetless: self.targetless,
            library_safe: self.library_safe,
        }
    }
}

fn classify_activated(
    ability: std::sync::Arc<deckmaste_core::ActivatedAbility>,
) -> deckmaste_core::Ability {
    use deckmaste_core::ActivatedManaProfile;
    use deckmaste_core::ManaAbility;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::UseLimit;

    let eligible_cost = cost_library_safe(&ability.cost);
    let loyalty = ability.limits.contains(&UseLimit::LoyaltyOncePerTurn);
    if !eligible_cost || loyalty {
        return deckmaste_core::Ability::Activated(ability);
    }

    if let OneShotEffect::Modal(modal) = &ability.effect {
        let classes: std::sync::Arc<[deckmaste_core::ManaModeClass]> = modal
            .modes
            .iter()
            .map(|mode| effect_mana_facts(&mode.effect).mode_class())
            .collect::<Vec<_>>()
            .into();
        if classes
            .iter()
            .any(|class| class.adds_mana && class.targetless && class.library_safe)
        {
            return deckmaste_core::Ability::Mana(ManaAbility::Activated {
                ability,
                profile: ActivatedManaProfile::ByAnnouncedMode(classes),
            });
        }
        return deckmaste_core::Ability::Activated(ability);
    }

    let facts = effect_mana_facts(&ability.effect);
    if facts.adds_mana && facts.targetless && facts.library_safe {
        deckmaste_core::Ability::Mana(ManaAbility::Activated {
            ability,
            profile: ActivatedManaProfile::Always,
        })
    } else {
        deckmaste_core::Ability::Activated(ability)
    }
}

fn classify_triggered(
    ability: std::sync::Arc<deckmaste_core::TriggeredAbility>,
) -> deckmaste_core::Ability {
    let facts = effect_mana_facts(&ability.effect);
    if facts.adds_mana && facts.targetless && triggered_by_mana(&ability.event) {
        deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Triggered(ability))
    } else {
        deckmaste_core::Ability::Triggered(ability)
    }
}

fn triggered_by_mana(event: &deckmaste_core::EventFilter) -> bool {
    use deckmaste_core::EventFilter;
    match event {
        EventFilter::ManaAbilityActivated { .. }
        | EventFilter::ManaProduced { .. }
        | EventFilter::ManaAdded { .. }
        | EventFilter::TapForMana { .. } => true,
        EventFilter::AllOf(parts) => parts.iter().any(triggered_by_mana),
        EventFilter::OneOf(parts) => !parts.is_empty() && parts.iter().all(triggered_by_mana),
        EventFilter::OneOrMore(inner) => triggered_by_mana(inner),
        EventFilter::Nth { of, .. } | EventFilter::When(of, _) | EventFilter::Within(of, _) => {
            triggered_by_mana(of)
        }
        EventFilter::Expanded(expansion) => triggered_by_mana(&expansion.value),
        _ => false,
    }
}

fn cost_library_safe(cost: &deckmaste_core::Cost) -> bool {
    use deckmaste_core::CostComponent;
    cost.iter().all(|component| match component {
        CostComponent::Act(action) => effect_action_facts(action).library_safe,
        CostComponent::Cost(inner) => cost_library_safe(inner),
        CostComponent::ChooseAndPay { body, .. } => cost_library_safe(body),
        CostComponent::Expanded(expansion) => cost_library_safe(&deckmaste_core::Cost(
            vec![(*expansion.value).clone()].into(),
        )),
        CostComponent::Mana(_)
        | CostComponent::ManaCostOf(_)
        | CostComponent::Tap
        | CostComponent::Untap
        | CostComponent::TapTotal { .. } => true,
    })
}

fn effect_mana_facts(effect: &deckmaste_core::OneShotEffect) -> ManaFacts {
    use deckmaste_core::OneShotEffect;
    match effect {
        OneShotEffect::Act(action) => effect_action_facts(action),
        OneShotEffect::Sequentially(parts) | OneShotEffect::Simultaneously(parts) => {
            parts.iter().fold(ManaFacts::NEUTRAL, |facts, part| {
                facts.merge(effect_mana_facts(part))
            })
        }
        OneShotEffect::Continuously(_) | OneShotEffect::Until(_, _) => ManaFacts::NEUTRAL,
        OneShotEffect::Label(label) => effect_mana_facts(&label.effect),
        OneShotEffect::SeparatePiles(piles) => piles
            .then
            .as_deref()
            .map_or(ManaFacts::NEUTRAL, effect_mana_facts),
        OneShotEffect::ChoosePile(pile) => effect_mana_facts(&pile.then),
        OneShotEffect::May(may) => [
            Some(may.effect.as_ref()),
            may.if_did.as_deref(),
            may.if_not.as_deref(),
        ]
        .into_iter()
        .flatten()
        .fold(ManaFacts::NEUTRAL, |facts, part| {
            facts.merge(effect_mana_facts(part))
        }),
        OneShotEffect::If(branch) => [Some(branch.then.as_ref()), branch.otherwise.as_deref()]
            .into_iter()
            .flatten()
            .fold(ManaFacts::NEUTRAL, |facts, part| {
                facts.merge(effect_mana_facts(part))
            }),
        OneShotEffect::AdditionalCost(additional) => {
            let mut facts = effect_mana_facts(&additional.body);
            facts.library_safe &= cost_library_safe(&additional.pay);
            facts
        }
        OneShotEffect::Each(each) => effect_mana_facts(&each.effect),
        OneShotEffect::With(with) => effect_mana_facts(&with.body),
        OneShotEffect::Distribute(distribute) => effect_mana_facts(&distribute.body),
        OneShotEffect::Noting(noting) => effect_mana_facts(&noting.effect),
        OneShotEffect::Delayed(_) | OneShotEffect::Reflexive(_) => ManaFacts::NEUTRAL,
        OneShotEffect::Modal(modal) => {
            modal.modes.iter().fold(ManaFacts::NEUTRAL, |facts, mode| {
                facts.merge(effect_mana_facts(&mode.effect))
            })
        }
        OneShotEffect::Targeted(targeted) => {
            let mut facts = effect_mana_facts(&targeted.effect);
            facts.targetless &= targeted.targets.is_empty();
            facts
        }
        OneShotEffect::Repeat(_, body) | OneShotEffect::Batch(_, body) => effect_mana_facts(body),
        OneShotEffect::RevealUntil(reveal) => effect_mana_facts(&reveal.body),
        OneShotEffect::Expanded(expansion) => effect_mana_facts(&expansion.value),
    }
}

fn effect_action_facts(action: &deckmaste_core::Action) -> ManaFacts {
    use deckmaste_core::Action;
    use deckmaste_core::Destination;
    use deckmaste_core::Zone;
    match action {
        Action::AddMana(_, _, _) => ManaFacts {
            adds_mana: true,
            ..ManaFacts::NEUTRAL
        },
        Action::Composite { body, .. } => effect_mana_facts(body),
        Action::Move(_, destination, _, from) => ManaFacts {
            library_safe: !matches!(destination, Destination::Library(_))
                && *from != Some(Zone::Library),
            ..ManaFacts::NEUTRAL
        },
        Action::MoveGroup { group, to, .. } => ManaFacts {
            library_safe: !matches!(to, Destination::Library(_)) && !selection_reads_library(group),
            ..ManaFacts::NEUTRAL
        },
        Action::DrawCard(_) | Action::Cast(_, _, _) => ManaFacts {
            library_safe: false,
            ..ManaFacts::NEUTRAL
        },
        Action::Pay(cost) => ManaFacts {
            library_safe: cost_library_safe(cost),
            ..ManaFacts::NEUTRAL
        },
        Action::Expanded(expansion) => effect_action_facts(&expansion.value),
        _ => ManaFacts::NEUTRAL,
    }
}

fn selection_reads_library(selection: &deckmaste_core::Selection) -> bool {
    use deckmaste_core::Selection;
    match selection {
        Selection::TopOfLibrary { .. }
        | Selection::BottomOfLibrary { .. }
        | Selection::LibraryOf(_) => true,
        Selection::Union(parts) => parts.iter().any(selection_reads_library),
        Selection::InChosenOrder(inner, _) => selection_reads_library(inner),
        Selection::Expanded(expansion) => selection_reads_library(&expansion.value),
        _ => false,
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
    fn lowers_spell_ability() {
        assert_matches!(
            deckmaste_semantics::SpellAbility {
                ability_word: None,
                effect: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::SpellAbility {
                ability_word: None,
                effect: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_activated_ability() {
        assert_matches!(
            deckmaste_semantics::ActivatedAbility {
                ability_word: None,
                cost: minimal_cost(),
                from: None,
                window: None,
                condition: None,
                limits: [].into(),
                effect: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::ActivatedAbility {
                ability_word: None,
                cost: deckmaste_core::Cost(_),
                from: None,
                window: None,
                condition: None,
                limits: _,
                effect: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_use_limit_once_per_turn() {
        assert_matches!(
            deckmaste_semantics::UseLimit::OncePerTurn.lower(),
            deckmaste_core::UseLimit::OncePerTurn
        );
    }

    #[test]
    fn lowers_use_limit_once_per_game() {
        assert_matches!(
            deckmaste_semantics::UseLimit::OncePerGame.lower(),
            deckmaste_core::UseLimit::OncePerGame
        );
    }

    #[test]
    fn lowers_use_limit_loyalty_once_per_turn() {
        assert_matches!(
            deckmaste_semantics::UseLimit::LoyaltyOncePerTurn.lower(),
            deckmaste_core::UseLimit::LoyaltyOncePerTurn
        );
    }

    #[test]
    fn lowers_triggered_ability() {
        assert_matches!(
            deckmaste_semantics::TriggeredAbility {
                ability_word: None,
                event: minimal_event_filter(),
                from: None,
                condition: None,
                limits: [].into(),
                where_x: None,
                effect: minimal_one_shot_effect()
            }
            .lower(),
            deckmaste_core::TriggeredAbility {
                ability_word: None,
                event: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                    from: None,
                    to: None,
                    cause: None
                },
                from: None,
                condition: None,
                limits: _,
                where_x: None,
                effect: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                ))
            }
        );
    }

    #[test]
    fn lowers_choose_spec() {
        assert_matches!(
            deckmaste_semantics::ChooseSpec {
                count: minimal_quantity(),
                up_to: false,
                repeats: false,
                chooser: minimal_reference(),
                rider: None
            }
            .lower(),
            deckmaste_core::ChooseSpec {
                count: deckmaste_core::Quantity::Range(None, None),
                up_to: false,
                repeats: false,
                chooser: deckmaste_core::Reference::This,
                rider: None
            }
        );
    }

    #[test]
    fn lowers_modal_cost_rider_entwine() {
        assert_matches!(
            deckmaste_semantics::ModalCostRider::Entwine(minimal_cost()).lower(),
            deckmaste_core::ModalCostRider::Entwine(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_modal_cost_rider_escalate() {
        assert_matches!(
            deckmaste_semantics::ModalCostRider::Escalate(minimal_cost()).lower(),
            deckmaste_core::ModalCostRider::Escalate(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_mode() {
        assert_matches!(
            deckmaste_semantics::Mode {
                effect: minimal_one_shot_effect(),
                cost: None
            }
            .lower(),
            deckmaste_core::Mode {
                effect: deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::This,
                    deckmaste_core::Count::X,
                    deckmaste_core::Reference::This
                )),
                cost: None
            }
        );
    }

    #[test]
    fn lowers_ability_static() {
        assert_matches!(
            deckmaste_semantics::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
                .lower(),
            deckmaste_core::Ability::Static(_)
        );
    }

    #[test]
    fn lowers_ability_activated() {
        assert_matches!(
            deckmaste_semantics::Ability::Activated(std::sync::Arc::new(
                minimal_activated_ability()
            ))
            .lower(),
            deckmaste_core::Ability::Activated(_)
        );
    }

    #[test]
    fn lowers_ability_triggered() {
        assert_matches!(
            deckmaste_semantics::Ability::Triggered(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::Ability::Triggered(_)
        );
    }

    #[test]
    fn lowers_ability_spell() {
        assert_matches!(
            deckmaste_semantics::Ability::Spell(std::sync::Arc::new(minimal_spell_ability()))
                .lower(),
            deckmaste_core::Ability::Spell(_)
        );
    }

    #[test]
    fn lowers_ability_keyword() {
        assert_matches!(
            deckmaste_semantics::Ability::Keyword(minimal_keyword_ability()).lower(),
            deckmaste_core::Ability::Keyword(deckmaste_core::KeywordAbility::FirstStrike)
        );
    }

    #[test]
    fn lowers_ability_innate() {
        assert_matches!(
            deckmaste_semantics::Ability::Innate(std::sync::Arc::new(minimal_ability())).lower(),
            deckmaste_core::Ability::Innate(_)
        );
    }

    #[test]
    fn lowers_ability_expanded() {
        assert_matches!(
            deckmaste_semantics::Ability::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_ability())
            })
            .lower(),
            deckmaste_core::Ability::Static(_)
        );
    }

    fn semantic_mana_effect() -> deckmaste_semantics::OneShotEffect {
        deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::AddMana(
            deckmaste_semantics::Reference::You,
            deckmaste_semantics::Count::Literal(1),
            deckmaste_semantics::ManaProduction::from(deckmaste_semantics::Color::Green),
        ))
    }

    fn semantic_mana_activation(
        effect: deckmaste_semantics::OneShotEffect,
    ) -> deckmaste_semantics::Ability {
        let mut ability = minimal_activated_ability();
        ability.cost =
            deckmaste_semantics::Cost(vec![deckmaste_semantics::CostComponent::Tap].into());
        ability.effect = effect;
        deckmaste_semantics::Ability::Activated(std::sync::Arc::new(ability))
    }

    #[test]
    fn classifies_targetless_activated_mana_ability_while_lowering() {
        assert_matches!(
            semantic_mana_activation(semantic_mana_effect()).lower(),
            deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated {
                profile: deckmaste_core::ActivatedManaProfile::Always,
                ..
            })
        );
    }

    #[test]
    fn classifies_sacrifice_cost_mana_ability_without_external_replacement_effects() {
        let mut ability = minimal_activated_ability();
        ability.cost = deckmaste_semantics::Cost(
            vec![deckmaste_semantics::CostComponent::With {
                binder: std::sync::Arc::new(deckmaste_semantics::Binder::ChooseOne {
                    filter: deckmaste_semantics::Predicate::Any,
                    by: deckmaste_semantics::Reference::You,
                }),
                body: deckmaste_semantics::Cost(
                    vec![deckmaste_semantics::CostComponent::do_action(
                        deckmaste_semantics::Action::Sacrifice(
                            deckmaste_semantics::Reference::You,
                            deckmaste_semantics::Reference::That(deckmaste_semantics::Sort::Card),
                        ),
                    )]
                    .into(),
                ),
            }]
            .into(),
        );
        ability.effect = semantic_mana_effect();

        assert_matches!(
            deckmaste_semantics::Ability::Activated(std::sync::Arc::new(ability)).lower(),
            deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated {
                profile: deckmaste_core::ActivatedManaProfile::Always,
                ..
            })
        );
    }

    #[test]
    fn library_touching_mana_producer_remains_ordinary() {
        let effect = deckmaste_semantics::OneShotEffect::Sequentially(
            vec![
                semantic_mana_effect(),
                deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::DrawCard(
                    deckmaste_semantics::Reference::You,
                )),
            ]
            .into(),
        );

        assert_matches!(
            semantic_mana_activation(effect).lower(),
            deckmaste_core::Ability::Activated(_)
        );
    }

    #[test]
    fn does_not_classify_targeted_or_loyalty_mana_producers() {
        let targeted =
            deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(semantic_mana_effect()),
            });
        assert_matches!(
            semantic_mana_activation(targeted).lower(),
            deckmaste_core::Ability::Activated(_)
        );

        let deckmaste_semantics::Ability::Activated(mut loyalty) =
            semantic_mana_activation(semantic_mana_effect())
        else {
            unreachable!()
        };
        std::sync::Arc::make_mut(&mut loyalty).limits =
            vec![deckmaste_semantics::UseLimit::LoyaltyOncePerTurn].into();
        assert_matches!(
            deckmaste_semantics::Ability::Activated(loyalty).lower(),
            deckmaste_core::Ability::Activated(_)
        );
    }

    #[test]
    fn compiles_modal_mana_profile_for_announced_modes() {
        let targeted =
            deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(semantic_mana_effect()),
            });
        let modal = deckmaste_semantics::OneShotEffect::Modal(deckmaste_semantics::Modal {
            choose: minimal_choose_spec(),
            modes: vec![
                deckmaste_semantics::Mode {
                    effect: semantic_mana_effect(),
                    cost: None,
                },
                deckmaste_semantics::Mode {
                    effect: targeted,
                    cost: None,
                },
            ]
            .into(),
        });
        let deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated {
            profile: deckmaste_core::ActivatedManaProfile::ByAnnouncedMode(classes),
            ..
        }) = semantic_mana_activation(modal).lower()
        else {
            panic!("a modal producer with a qualifying mode needs a compiled profile");
        };
        assert_eq!(classes.len(), 2);
        assert!(classes[0].adds_mana && classes[0].targetless && classes[0].library_safe);
        assert!(classes[1].adds_mana && !classes[1].targetless && classes[1].library_safe);
    }

    #[test]
    fn classifies_each_causal_triggered_mana_ability_while_lowering() {
        let any = || deckmaste_semantics::Predicate::Any;
        let causes = [
            deckmaste_semantics::EventFilter::ManaAbilityActivated {
                what: any(),
                by: any(),
            },
            deckmaste_semantics::EventFilter::ManaProduced {
                what: any(),
                by: any(),
            },
            deckmaste_semantics::EventFilter::ManaAdded {
                what: any(),
                by: any(),
            },
            deckmaste_semantics::EventFilter::TapForMana {
                what: any(),
                by: any(),
            },
        ];
        for event in causes {
            let mut triggered = minimal_triggered_ability();
            triggered.event = event.clone();
            triggered.effect = semantic_mana_effect();
            assert_matches!(
                deckmaste_semantics::Ability::Triggered(std::sync::Arc::new(triggered)).lower(),
                deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Triggered(_)),
                "cause {event:?}"
            );
        }
    }
}
