//! `ability` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

fn peel_targets(
    effect: deckmaste_semantics::OneShotEffect,
) -> (
    std::sync::Arc<[deckmaste_semantics::TargetSpec]>,
    deckmaste_semantics::OneShotEffect,
) {
    match effect {
        deckmaste_semantics::OneShotEffect::Targeted(targeted) => (
            targeted.targets,
            std::sync::Arc::unwrap_or_clone(targeted.effect),
        ),
        deckmaste_semantics::OneShotEffect::Expanded(expanded) => peel_targets(*expanded.value),
        deckmaste_semantics::OneShotEffect::May(mut may) => {
            let (targets, effect) = peel_targets(std::sync::Arc::unwrap_or_clone(may.effect));
            may.effect = std::sync::Arc::new(effect);
            (targets, deckmaste_semantics::OneShotEffect::May(may))
        }
        other => (std::sync::Arc::from([]), other),
    }
}

fn lower_region(
    kind: crate::region::RegionKind,
    effect: deckmaste_semantics::OneShotEffect,
) -> (
    std::sync::Arc<[deckmaste_core::TargetSpec]>,
    deckmaste_core::Region,
) {
    let (targets, body) = peel_targets(effect);
    let target_count = targets.len();
    let (params, (targets, body)) =
        crate::region::in_region(kind, target_count, || (targets.lower(), body.lower()));
    (targets, deckmaste_core::Region::new(params, body))
}

impl Lower for deckmaste_semantics::SpellAbility {
    type Target = deckmaste_core::SpellAbility;
    fn lower(self) -> <Self as Lower>::Target {
        let (targets, effect) = lower_region(crate::region::RegionKind::Spell, self.effect);
        deckmaste_core::SpellAbility {
            ability_word: self.ability_word.lower(),
            targets,
            effect,
        }
    }
}

impl Lower for deckmaste_semantics::ActivatedAbility {
    type Target = deckmaste_core::ActivatedAbility;
    fn lower(self) -> <Self as Lower>::Target {
        let (targets, effect) = lower_region(crate::region::RegionKind::Activated, self.effect);
        deckmaste_core::ActivatedAbility {
            ability_word: self.ability_word.lower(),
            cost: self.cost.lower(),
            from: self.from.lower(),
            window: self.window.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            targets,
            effect,
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
        let (semantic_targets, body) = peel_targets(self.effect);
        let target_count = semantic_targets.len();
        let (params, (targets, body, where_x)) =
            crate::region::in_region(crate::region::RegionKind::Triggered, target_count, || {
                (semantic_targets.lower(), body.lower(), self.where_x.lower())
            });
        deckmaste_core::TriggeredAbility {
            ability_word: self.ability_word.lower(),
            event: self.event.lower(),
            from: self.from.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            where_x,
            targets,
            effect: deckmaste_core::Region::new(params, body),
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
        let (targets, effect) = lower_region(crate::region::RegionKind::Mode, self.effect);
        deckmaste_core::Mode {
            targets,
            effect,
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
}

impl ManaFacts {
    const NEUTRAL: Self = Self {
        adds_mana: false,
        targetless: true,
    };

    fn merge(self, other: Self) -> Self {
        Self {
            adds_mana: self.adds_mana || other.adds_mana,
            targetless: self.targetless && other.targetless,
        }
    }

    fn mode_class(self) -> deckmaste_core::ManaModeClass {
        deckmaste_core::ManaModeClass {
            adds_mana: self.adds_mana,
            targetless: self.targetless,
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

    let loyalty = ability.limits.contains(&UseLimit::LoyaltyOncePerTurn);
    if loyalty {
        return deckmaste_core::Ability::Activated(ability);
    }

    if let [OneShotEffect::Modal(modal)] = ability.effect.body.as_ref() {
        let classes: std::sync::Arc<[deckmaste_core::ManaModeClass]> = modal
            .modes
            .iter()
            .map(|mode| {
                let mut facts = region_mana_facts(&mode.effect);
                facts.targetless &= mode.targets.is_empty();
                facts.mode_class()
            })
            .collect::<Vec<_>>()
            .into();
        if classes
            .iter()
            .any(|class| class.adds_mana && class.targetless)
        {
            return deckmaste_core::Ability::Mana(ManaAbility::Activated {
                ability,
                profile: ActivatedManaProfile::ByAnnouncedMode(classes),
            });
        }
        return deckmaste_core::Ability::Activated(ability);
    }

    let mut facts = region_mana_facts(&ability.effect);
    facts.targetless &= ability.targets.is_empty();
    if facts.adds_mana && facts.targetless {
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
    let mut facts = region_mana_facts(&ability.effect);
    facts.targetless &= ability.targets.is_empty();
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
        _ => false,
    }
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
        // RevealUntil is intentionally inert at runtime; none of these nodes
        // can establish that the executable ability produces mana.
        OneShotEffect::Continuously(_)
        | OneShotEffect::Until(_, _)
        | OneShotEffect::Delayed(_)
        | OneShotEffect::Reflexive(_)
        | OneShotEffect::RevealUntil(_) => ManaFacts::NEUTRAL,
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
        OneShotEffect::AdditionalCost(additional) => effect_mana_facts(&additional.body),
        OneShotEffect::Each(each) => {
            binder_mana_facts(&each.binder).merge(effect_mana_facts(&each.effect))
        }
        OneShotEffect::With(with) => {
            binder_mana_facts(&with.binder).merge(effect_mana_facts(&with.body))
        }
        OneShotEffect::Distribute(distribute) => {
            binder_mana_facts(&distribute.binder).merge(effect_mana_facts(&distribute.body))
        }
        OneShotEffect::Noting(noting) => effect_mana_facts(&noting.effect),
        OneShotEffect::Modal(modal) => {
            modal.modes.iter().fold(ManaFacts::NEUTRAL, |facts, mode| {
                facts.merge(region_mana_facts(&mode.effect))
            })
        }
        OneShotEffect::Repeat(_, body) | OneShotEffect::Batch(_, body) => effect_mana_facts(body),
    }
}

fn region_mana_facts(region: &deckmaste_core::Region) -> ManaFacts {
    region
        .body
        .iter()
        .fold(ManaFacts::NEUTRAL, |facts, instruction| {
            facts.merge(effect_mana_facts(instruction))
        })
}

fn effect_action_facts(action: &deckmaste_core::Action) -> ManaFacts {
    use deckmaste_core::Action;
    match action {
        Action::AddMana(_, _, _) => ManaFacts {
            adds_mana: true,
            ..ManaFacts::NEUTRAL
        },
        Action::Composite { body, .. } => effect_mana_facts(body),
        _ => ManaFacts::NEUTRAL,
    }
}

fn binder_mana_facts(binder: &deckmaste_core::Binder) -> ManaFacts {
    use deckmaste_core::Binder;
    match binder {
        Binder::Produce(action) => effect_action_facts(action),
        Binder::SearchOne { if_none, .. } | Binder::Search { if_none, .. } => if_none
            .as_deref()
            .map_or(ManaFacts::NEUTRAL, effect_mana_facts),
        Binder::TheRef(_)
        | Binder::ChooseOne { .. }
        | Binder::Choose { .. }
        | Binder::Existing(_) => ManaFacts::NEUTRAL,
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
        let lowered = deckmaste_semantics::SpellAbility {
            ability_word: None,
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [deckmaste_core::OneShotEffect::Act(
                deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Count::Reg(_),
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
                )
            )]
        );
    }

    #[test]
    fn lowers_activated_ability() {
        let lowered = deckmaste_semantics::ActivatedAbility {
            ability_word: None,
            cost: minimal_cost(),
            from: None,
            window: None,
            condition: None,
            limits: [].into(),
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [deckmaste_core::OneShotEffect::Act(
                deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Count::Reg(_),
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
                )
            )]
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
        let lowered = deckmaste_semantics::TriggeredAbility {
            ability_word: None,
            event: minimal_event_filter(),
            from: None,
            condition: None,
            limits: [].into(),
            where_x: None,
            effect: minimal_one_shot_effect(),
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [deckmaste_core::OneShotEffect::Act(
                deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Count::Reg(_),
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
                )
            )]
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
                chooser: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
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
        let lowered = deckmaste_semantics::Mode {
            effect: minimal_one_shot_effect(),
            cost: None,
        }
        .lower();
        assert!(lowered.targets.is_empty());
        assert_matches!(
            lowered.effect.body.as_ref(),
            [deckmaste_core::OneShotEffect::Act(
                deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Count::Reg(_),
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
                )
            )]
        );
    }

    #[test]
    fn two_same_sort_targets_lower_to_distinct_region_parameters() {
        let effect = deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
            targets: vec![minimal_target_spec(), minimal_target_spec()].into(),
            effect: std::sync::Arc::new(deckmaste_semantics::OneShotEffect::Sequentially(
                vec![
                    deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::DrawCard(
                        deckmaste_semantics::Reference::Target(0),
                    )),
                    deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::DrawCard(
                        deckmaste_semantics::Reference::Target(1),
                    )),
                ]
                .into(),
            )),
        });
        let lowered = deckmaste_semantics::SpellAbility {
            ability_word: None,
            effect,
        }
        .lower();

        assert_eq!(lowered.targets.len(), 2);
        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DrawCard(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(6))
                )),
                deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DrawCard(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(7))
                ))
            ]
        );
        assert_eq!(
            deckmaste_core::validate_telescope(&lowered.effect, &lowered.targets, None),
            Ok(())
        );
    }

    #[test]
    fn trigger_event_roles_lower_to_the_fixed_region_prefix() {
        let mut ability = minimal_triggered_ability();
        ability.effect = deckmaste_semantics::OneShotEffect::Sequentially(
            vec![
                deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::DealDamage(
                    deckmaste_semantics::Reference::EventObject,
                    deckmaste_semantics::Count::Literal(1),
                    deckmaste_semantics::Reference::EventPatient,
                )),
                deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::DrawCard(
                    deckmaste_semantics::Reference::EventActor,
                )),
            ]
            .into(),
        );
        let lowered = ability.lower();

        assert_matches!(
            lowered.effect.body.as_ref(),
            [
                deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(2)),
                    _,
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(3))
                )),
                deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DrawCard(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(4))
                ))
            ]
        );
        assert_eq!(
            deckmaste_core::validate_telescope(
                &lowered.effect,
                &lowered.targets,
                lowered.where_x.as_ref()
            ),
            Ok(())
        );
    }

    #[test]
    fn modal_and_delayed_regions_own_their_target_telescopes() {
        let targeted_mode = deckmaste_semantics::Mode {
            effect: deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(deckmaste_semantics::OneShotEffect::Act(
                    deckmaste_semantics::Action::DrawCard(deckmaste_semantics::Reference::Target(
                        0,
                    )),
                )),
            }),
            cost: None,
        }
        .lower();
        assert_eq!(targeted_mode.targets.len(), 1);
        assert_matches!(
            targeted_mode.effect.body.as_ref(),
            [deckmaste_core::OneShotEffect::Act(
                deckmaste_core::Action::DrawCard(deckmaste_core::Reference::Reg(
                    deckmaste_core::RefId(6)
                ))
            )]
        );

        let delayed = deckmaste_semantics::OneShotEffect::Delayed(std::sync::Arc::new(
            minimal_triggered_ability(),
        ));
        let outer = deckmaste_semantics::SpellAbility {
            ability_word: None,
            effect: deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
                targets: vec![minimal_target_spec()].into(),
                effect: std::sync::Arc::new(delayed),
            }),
        }
        .lower();
        let [deckmaste_core::OneShotEffect::Delayed(inner)] = outer.effect.body.as_ref() else {
            panic!("outer targeted spell should contain one delayed trigger")
        };
        assert!(inner.targets.is_empty());
        assert_eq!(
            inner
                .effect
                .params
                .iter()
                .filter(|param| matches!(
                    param.provenance,
                    deckmaste_core::Provenance::AnnouncedTarget(_)
                ))
                .count(),
            0
        );
        assert_eq!(
            deckmaste_core::validate_telescope(&outer.effect, &outer.targets, None),
            Ok(())
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
    fn reversibility_does_not_change_mana_classification() {
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
            deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated { .. })
        );
    }

    #[test]
    fn mana_produced_by_a_binder_participates_in_classification() {
        let effect = deckmaste_semantics::OneShotEffect::With(deckmaste_semantics::With {
            binder: deckmaste_semantics::Binder::Produce(std::sync::Arc::new(
                deckmaste_semantics::Action::AddMana(
                    deckmaste_semantics::Reference::You,
                    deckmaste_semantics::Count::Literal(1),
                    deckmaste_semantics::ManaProduction::from(deckmaste_semantics::Color::Green),
                ),
            )),
            body: std::sync::Arc::new(minimal_one_shot_effect()),
        });

        assert_matches!(
            semantic_mana_activation(effect).lower(),
            deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated { .. })
        );
    }

    #[test]
    fn inert_reveal_until_body_does_not_establish_mana_production() {
        let effect =
            deckmaste_semantics::OneShotEffect::RevealUntil(deckmaste_semantics::RevealUntil {
                whose: deckmaste_semantics::Reference::You,
                matches: deckmaste_semantics::Predicate::Any,
                body: std::sync::Arc::new(semantic_mana_effect()),
            });

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
        assert!(classes[0].adds_mana && classes[0].targetless);
        assert!(classes[1].adds_mana && !classes[1].targetless);
    }

    #[test]
    fn modal_mana_profile_does_not_treat_reversibility_as_classification() {
        let unsafe_cost = vec![deckmaste_semantics::CostComponent::do_action(
            deckmaste_semantics::Action::Move(
                deckmaste_semantics::Reference::This,
                deckmaste_semantics::Destination::Library(deckmaste_semantics::Anchor::FromBottom(
                    deckmaste_semantics::Count::Literal(0),
                )),
                [].into(),
                Some(deckmaste_semantics::Zone::Battlefield),
            ),
        )]
        .into();
        let modal = deckmaste_semantics::OneShotEffect::Modal(deckmaste_semantics::Modal {
            choose: minimal_choose_spec(),
            modes: vec![
                deckmaste_semantics::Mode {
                    effect: semantic_mana_effect(),
                    cost: None,
                },
                deckmaste_semantics::Mode {
                    effect: semantic_mana_effect(),
                    cost: Some(unsafe_cost),
                },
            ]
            .into(),
        });
        let lowered = semantic_mana_activation(modal).lower();
        let deckmaste_core::Ability::Mana(deckmaste_core::ManaAbility::Activated {
            profile: deckmaste_core::ActivatedManaProfile::ByAnnouncedMode(classes),
            ..
        }) = &lowered
        else {
            panic!("runtime barriers do not affect mana-ability classification");
        };

        assert!(
            classes
                .iter()
                .all(|class| class.adds_mana && class.targetless)
        );
        assert!(lowered.mana_profile_for_modes(&[1]));
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
