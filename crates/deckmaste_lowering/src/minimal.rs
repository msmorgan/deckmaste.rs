//! Minimal values, one per grammar type, for the per-variant lowering
//! tests. Generated; each picks a variant whose fields all ground.

#![allow(
    dead_code,
    reason = "a type not used at any field position needs no helper caller"
)]

/// Lower `f` inside a minimal spell region.
///
/// Instruction lowering DEFINES registers — a damage or life-change magnitude
/// is pinned by a `Let` at its evaluation moment ([CR#608.2h]) — so any value
/// carrying an instruction can only be lowered with a region context open.
pub fn in_spell_region<T>(f: impl FnOnce() -> T) -> T {
    crate::region::in_region(crate::region::RegionKind::Spell, 0, f).1
}

/// The lowered image of [`minimal_one_shot_effect`]: `DealDamage` pins its
/// magnitude in a `Let` at the instruction's program point and then reads that
/// register ([CR#608.2h]), so one authored action becomes a two-instruction
/// sequence whose second half reads the first half's definition.
pub fn is_minimal_lowered_effect(effect: &deckmaste_core::Instruction) -> bool {
    let deckmaste_core::Instruction::Sequentially(parts) = effect else {
        return false;
    };
    let [
        deckmaste_core::Instruction::Let(deckmaste_core::Let {
            dest,
            expr: deckmaste_core::Expr::Number(_),
        }),
        deckmaste_core::Instruction::Act {
            dest: None,
            action:
                deckmaste_core::Action::DealDamage(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Count::Reg(read),
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                ),
        },
    ] = parts.as_ref()
    else {
        return false;
    };
    deckmaste_core::RefId::from(*dest) == *read
}

pub fn minimal_ability() -> deckmaste_semantics::Ability {
    deckmaste_semantics::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
}

pub fn minimal_action() -> deckmaste_semantics::Action {
    deckmaste_semantics::Action::DealDamage(
        minimal_reference(),
        minimal_count(),
        minimal_reference(),
    )
}

pub fn minimal_activated_ability() -> deckmaste_semantics::ActivatedAbility {
    deckmaste_semantics::ActivatedAbility {
        ability_word: None,
        cost: minimal_cost(),
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_additional_cost() -> deckmaste_semantics::AdditionalCost {
    deckmaste_semantics::AdditionalCost {
        pay: minimal_cost(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_adjacency() -> deckmaste_semantics::Adjacency {
    deckmaste_semantics::Adjacency::Above
}

pub fn minimal_agency() -> deckmaste_semantics::Agency {
    deckmaste_semantics::Agency::CostPayment
}

pub fn minimal_aggregate_op() -> deckmaste_semantics::AggregateOp {
    deckmaste_semantics::AggregateOp::SumOf
}

pub fn minimal_alternative_cost() -> deckmaste_semantics::AlternativeCost {
    deckmaste_semantics::AlternativeCost::Free
}

pub fn minimal_anchor() -> deckmaste_semantics::Anchor {
    deckmaste_semantics::Anchor::FromTop(minimal_count())
}

pub fn minimal_arrangement() -> deckmaste_semantics::Arrangement {
    deckmaste_semantics::Arrangement::ChosenOrder(minimal_reference())
}

pub fn minimal_as_though() -> deckmaste_semantics::AsThough {
    deckmaste_semantics::AsThough::Counterfactual {
        premise: minimal_predicate(),
        then: std::sync::Arc::new(minimal_deontic()),
    }
}

pub fn minimal_beginning_step() -> deckmaste_semantics::BeginningStep {
    deckmaste_semantics::BeginningStep::Untap
}

pub fn minimal_binder() -> deckmaste_semantics::Binder {
    deckmaste_semantics::Binder::TheRef(minimal_reference())
}

pub fn minimal_card() -> deckmaste_semantics::Card {
    deckmaste_semantics::Card::Normal(minimal_card_face())
}

pub fn minimal_card_face() -> deckmaste_semantics::CardFace {
    deckmaste_semantics::CardFace {
        name: "x".into(),
        mana_cost: deckmaste_semantics::ManaCost::from(std::sync::Arc::<
            [deckmaste_semantics::ManaSymbol],
        >::from([])),
        color_indicator: Vec::new(),
        supertypes: Vec::new(),
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }
}

pub fn minimal_cause() -> deckmaste_semantics::Cause {
    deckmaste_semantics::Cause::Cause(minimal_cause_pattern())
}

pub fn minimal_cause_pattern() -> deckmaste_semantics::CausePattern {
    deckmaste_semantics::CausePattern {
        verb: None,
        agency: None,
        agent: None,
    }
}

pub fn minimal_characteristic() -> deckmaste_semantics::Characteristic {
    deckmaste_semantics::Characteristic::Colors
}

pub fn minimal_characteristic_predicate() -> deckmaste_semantics::CharacteristicPredicate {
    deckmaste_semantics::CharacteristicPredicate::Type(minimal_type_ref())
}

pub fn minimal_choose_pile() -> deckmaste_semantics::ChoosePile {
    deckmaste_semantics::ChoosePile {
        from: minimal_pile_source(),
        by: minimal_reference(),
        random: false,
        then: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_choose_spec() -> deckmaste_semantics::ChooseSpec {
    deckmaste_semantics::ChooseSpec {
        count: minimal_quantity(),
        up_to: false,
        repeats: false,
        chooser: minimal_reference(),
        rider: None,
    }
}

pub fn minimal_chosen_value_kind() -> deckmaste_semantics::ChosenValueKind {
    deckmaste_semantics::ChosenValueKind::Color
}

pub fn minimal_cmp() -> deckmaste_semantics::Cmp {
    deckmaste_semantics::Cmp::Eq
}

pub fn minimal_collection_op<T>() -> deckmaste_semantics::CollectionOp<T> {
    deckmaste_semantics::CollectionOp::Set([].into())
}

pub fn minimal_color() -> deckmaste_semantics::Color {
    deckmaste_semantics::Color::White
}

pub fn minimal_color_or_colorless() -> deckmaste_semantics::ColorOrColorless {
    deckmaste_semantics::ColorOrColorless::Colorless
}

pub fn minimal_combat_step() -> deckmaste_semantics::CombatStep {
    deckmaste_semantics::CombatStep::BeginningOfCombat
}

pub fn minimal_condition() -> deckmaste_semantics::Condition {
    deckmaste_semantics::Condition::Compare(minimal_count(), minimal_cmp(), minimal_count())
}

pub fn minimal_conferral_rule() -> deckmaste_semantics::ConferralRule {
    deckmaste_semantics::ConferralRule {
        scope: minimal_predicate(),
        confer: minimal_property(),
    }
}

pub fn minimal_continuously() -> deckmaste_semantics::Continuously {
    deckmaste_semantics::Continuously {
        effect: std::sync::Arc::new(minimal_static_effect()),
        duration: minimal_duration(),
    }
}

pub fn minimal_copiable_values() -> deckmaste_semantics::CopiableValues {
    deckmaste_semantics::CopiableValues {
        name: "x".into(),
        mana_cost: deckmaste_semantics::ManaCost::from(std::sync::Arc::<
            [deckmaste_semantics::ManaSymbol],
        >::from([])),
        color_indicator: Vec::new(),
        supertypes: Vec::new(),
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }
}

pub fn minimal_copy_exception() -> deckmaste_semantics::CopyException {
    deckmaste_semantics::CopyException::Modify(minimal_modification())
}

pub fn minimal_copy_retarget() -> deckmaste_semantics::CopyRetarget {
    deckmaste_semantics::CopyRetarget::AsIs
}

pub fn minimal_copy_source() -> deckmaste_semantics::CopySource {
    deckmaste_semantics::CopySource::Object(minimal_reference())
}

pub fn minimal_copy_spec() -> deckmaste_semantics::CopySpec {
    deckmaste_semantics::CopySpec {
        source: minimal_copy_source(),
        exceptions: Vec::new(),
    }
}

pub fn minimal_cost() -> deckmaste_semantics::Cost {
    deckmaste_semantics::Cost([].into())
}

pub fn minimal_cost_change() -> deckmaste_semantics::CostChange {
    deckmaste_semantics::CostChange::Increase([].into())
}

pub fn minimal_cost_component() -> deckmaste_semantics::CostComponent {
    deckmaste_semantics::CostComponent::Mana(deckmaste_semantics::ManaCost::from(std::sync::Arc::<
        [deckmaste_semantics::ManaSymbol],
    >::from([])))
}

pub fn minimal_cost_predicate() -> deckmaste_semantics::CostPredicate {
    deckmaste_semantics::CostPredicate::IncludesTapSymbol
}

pub fn minimal_cost_tag() -> deckmaste_semantics::CostTag {
    deckmaste_semantics::CostTag("X".into())
}

pub fn minimal_count() -> deckmaste_semantics::Count {
    deckmaste_semantics::Count::Literal(0)
}

pub fn minimal_count_bound() -> deckmaste_semantics::CountBound {
    deckmaste_semantics::CountBound::Eq(minimal_count())
}

pub fn minimal_countable() -> deckmaste_semantics::Countable {
    deckmaste_semantics::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_counter() -> deckmaste_semantics::Counter {
    deckmaste_semantics::Counter {
        name: "X".into(),
        scope: minimal_counter_scope(),
        confers: Vec::new(),
    }
}

pub fn minimal_counter_ref() -> deckmaste_semantics::CounterRef {
    deckmaste_semantics::CounterRef("X".into())
}

pub fn minimal_counter_scope() -> deckmaste_semantics::CounterScope {
    deckmaste_semantics::CounterScope::Object
}

pub fn minimal_counter_spec() -> deckmaste_semantics::CounterSpec {
    deckmaste_semantics::CounterSpec::Named(minimal_counter_ref(), minimal_count())
}

pub fn minimal_damage_result_rule() -> deckmaste_semantics::DamageResultRule {
    deckmaste_semantics::DamageResultRule {
        recipient: minimal_predicate(),
        remove: minimal_counter_ref(),
    }
}

pub fn minimal_decider_spec() -> deckmaste_semantics::DeciderSpec {
    deckmaste_semantics::DeciderSpec::Controller
}

pub fn minimal_deed_agent() -> deckmaste_semantics::DeedAgent {
    deckmaste_semantics::DeedAgent {
        stack_object: None,
        source: None,
    }
}

pub fn minimal_deontic() -> deckmaste_semantics::Deontic {
    deckmaste_semantics::Deontic::May(minimal_deontic_action())
}

pub fn minimal_deontic_action() -> deckmaste_semantics::DeonticAction {
    deckmaste_semantics::DeonticAction::Attack {
        by: minimal_predicate(),
        on: minimal_predicate(),
    }
}

pub fn minimal_designation_decl() -> deckmaste_semantics::DesignationDecl {
    deckmaste_semantics::DesignationDecl {
        name: "X".into(),
        definition: minimal_designation_def(),
        conferrers: [].into(),
    }
}

pub fn minimal_designation_def() -> deckmaste_semantics::DesignationDef {
    deckmaste_semantics::DesignationDef::Stored {
        scope: minimal_designation_scope(),
        shape: minimal_designation_shape(),
        uniqueness: minimal_designation_uniqueness(),
        persistence: minimal_designation_persistence(),
        payload: [].into(),
    }
}

pub fn minimal_designation_persistence() -> deckmaste_semantics::DesignationPersistence {
    deckmaste_semantics::DesignationPersistence::ObjectLifetime
}

pub fn minimal_designation_scope() -> deckmaste_semantics::DesignationScope {
    deckmaste_semantics::DesignationScope::Object
}

pub fn minimal_designation_shape() -> deckmaste_semantics::DesignationShape {
    deckmaste_semantics::DesignationShape::Flag
}

pub fn minimal_designation_uniqueness() -> deckmaste_semantics::DesignationUniqueness {
    deckmaste_semantics::DesignationUniqueness::None
}

pub fn minimal_destination() -> deckmaste_semantics::Destination {
    deckmaste_semantics::Destination::Zone(minimal_zone())
}

pub fn minimal_distribute() -> deckmaste_semantics::Distribute {
    deckmaste_semantics::Distribute {
        amount: minimal_count(),
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_duration() -> deckmaste_semantics::Duration {
    deckmaste_semantics::Duration::FixedUntil(minimal_turn_marker())
}

pub fn minimal_each() -> deckmaste_semantics::Each {
    deckmaste_semantics::Each {
        binder: minimal_binder(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_ending_step() -> deckmaste_semantics::EndingStep {
    deckmaste_semantics::EndingStep::End
}

pub fn minimal_enter_rider() -> deckmaste_semantics::EnterRider {
    deckmaste_semantics::EnterRider::Tapped
}

pub fn minimal_event_filter() -> deckmaste_semantics::EventFilter {
    deckmaste_semantics::EventFilter::ZoneChange {
        what: minimal_predicate(),
        from: None,
        to: None,
        cause: None,
    }
}

pub fn minimal_face() -> deckmaste_semantics::Face {
    deckmaste_semantics::Face::Up
}

pub fn minimal_face_down_characteristics() -> deckmaste_semantics::FaceDownCharacteristics {
    deckmaste_semantics::FaceDownCharacteristics {
        name: None,
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
    }
}

pub fn minimal_face_down_spec() -> deckmaste_semantics::FaceDownSpec {
    deckmaste_semantics::FaceDownSpec::Listed(minimal_face_down_characteristics())
}

pub fn minimal_face_layout() -> deckmaste_semantics::FaceLayout {
    deckmaste_semantics::FaceLayout::Transforming
}

pub fn minimal_if() -> deckmaste_semantics::If {
    deckmaste_semantics::If {
        condition: minimal_condition(),
        then: std::sync::Arc::new(minimal_one_shot_effect()),
        otherwise: None,
    }
}

pub fn minimal_ignore_rule() -> deckmaste_semantics::IgnoreRule {
    deckmaste_semantics::IgnoreRule::IgnoreLowest
}

pub fn minimal_keyword_ability() -> deckmaste_semantics::KeywordAbility {
    deckmaste_semantics::KeywordAbility::FirstStrike
}

pub fn minimal_keyword_decl() -> deckmaste_semantics::KeywordDecl {
    deckmaste_semantics::KeywordDecl {
        name: "X".into(),
        shape: minimal_param_shape(),
    }
}

pub fn minimal_keyword_ref() -> deckmaste_semantics::KeywordRef {
    deckmaste_semantics::KeywordRef("X".into())
}

pub fn minimal_label() -> deckmaste_semantics::Label {
    deckmaste_semantics::Label {
        r#as: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_life_op() -> deckmaste_semantics::LifeOp {
    deckmaste_semantics::LifeOp::Set(minimal_count())
}

pub fn minimal_lock_point() -> deckmaste_semantics::LockPoint {
    deckmaste_semantics::LockPoint::Announce
}

pub fn minimal_lookback() -> deckmaste_semantics::Lookback {
    deckmaste_semantics::Lookback::ThisTurn
}

pub fn minimal_mana_cost() -> deckmaste_semantics::ManaCost {
    deckmaste_semantics::ManaCost::from(std::sync::Arc::<[deckmaste_semantics::ManaSymbol]>::from(
        [],
    ))
}

pub fn minimal_mana_production() -> deckmaste_semantics::ManaProduction {
    deckmaste_semantics::ManaProduction::WithRiders {
        mana: minimal_mana_spec(),
        riders: [].into(),
    }
}

pub fn minimal_mana_rider() -> deckmaste_semantics::ManaRider {
    deckmaste_semantics::ManaRider::SpendOnly(minimal_predicate())
}

pub fn minimal_mana_spec() -> deckmaste_semantics::ManaSpec {
    deckmaste_semantics::ManaSpec::AnyColor
}

pub fn minimal_mana_symbol() -> deckmaste_semantics::ManaSymbol {
    deckmaste_semantics::ManaSymbol::Variable
}

pub fn minimal_may() -> deckmaste_semantics::May {
    deckmaste_semantics::May {
        who: minimal_reference(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
        if_did: None,
        if_not: None,
    }
}

pub fn minimal_modal() -> deckmaste_semantics::Modal {
    deckmaste_semantics::Modal {
        choose: minimal_choose_spec(),
        modes: [].into(),
    }
}

pub fn minimal_modal_cost_rider() -> deckmaste_semantics::ModalCostRider {
    deckmaste_semantics::ModalCostRider::Entwine(minimal_cost())
}

pub fn minimal_mode() -> deckmaste_semantics::Mode {
    deckmaste_semantics::Mode {
        effect: minimal_one_shot_effect(),
        cost: None,
    }
}

pub fn minimal_modification() -> deckmaste_semantics::Modification {
    deckmaste_semantics::Modification::Power(minimal_numeric_op())
}

pub fn minimal_noted_kind() -> deckmaste_semantics::NotedKind {
    deckmaste_semantics::NotedKind::Objects
}

pub fn minimal_noting() -> deckmaste_semantics::Noting {
    deckmaste_semantics::Noting {
        key: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_numeric_op() -> deckmaste_semantics::NumericOp {
    deckmaste_semantics::NumericOp::Set(minimal_stat_value())
}

pub fn minimal_object_kind() -> deckmaste_semantics::ObjectKind {
    deckmaste_semantics::ObjectKind::Ability
}

pub fn minimal_one_shot_effect() -> deckmaste_semantics::OneShotEffect {
    deckmaste_semantics::OneShotEffect::Act(minimal_action())
}

pub fn minimal_optional_cost() -> deckmaste_semantics::OptionalCost {
    deckmaste_semantics::OptionalCost {
        components: [].into(),
        tag: minimal_cost_tag(),
        repeatable: false,
    }
}

pub fn minimal_outcome_gate_kind() -> deckmaste_semantics::OutcomeGateKind {
    deckmaste_semantics::OutcomeGateKind::CantLose
}

pub fn minimal_param_shape() -> deckmaste_semantics::ParamShape {
    deckmaste_semantics::ParamShape::None
}

pub fn minimal_pay_act() -> deckmaste_semantics::PayAct {
    deckmaste_semantics::PayAct::TapToPay(minimal_predicate())
}

pub fn minimal_phase_kind() -> deckmaste_semantics::PhaseKind {
    deckmaste_semantics::PhaseKind::Beginning
}

pub fn minimal_phase_step() -> deckmaste_semantics::PhaseStep {
    deckmaste_semantics::PhaseStep::Beginning(minimal_beginning_step())
}

pub fn minimal_phasing() -> deckmaste_semantics::Phasing {
    deckmaste_semantics::Phasing::In
}

pub fn minimal_pile_source() -> deckmaste_semantics::PileSource {
    deckmaste_semantics::PileSource::Labels([].into())
}

pub fn minimal_pip_class() -> deckmaste_semantics::PipClass {
    deckmaste_semantics::PipClass::Generic
}

pub fn minimal_planar_face() -> deckmaste_semantics::PlanarFace {
    deckmaste_semantics::PlanarFace::Blank
}

pub fn minimal_player_attr() -> deckmaste_semantics::PlayerAttr {
    deckmaste_semantics::PlayerAttr::Life
}

pub fn minimal_player_mod() -> deckmaste_semantics::PlayerMod {
    deckmaste_semantics::PlayerMod::SetTo(minimal_player_attr(), minimal_count())
}

pub fn minimal_predefined_token() -> deckmaste_semantics::PredefinedToken {
    deckmaste_semantics::PredefinedToken::Treasure
}

pub fn minimal_predicate() -> deckmaste_semantics::Predicate {
    deckmaste_semantics::Predicate::Kind(minimal_object_kind())
}

pub fn minimal_prevention() -> deckmaste_semantics::Prevention {
    deckmaste_semantics::Prevention::PreventNext {
        n: minimal_count(),
        from: minimal_predicate(),
        to: minimal_predicate(),
        duration: None,
    }
}

pub fn minimal_projection() -> deckmaste_semantics::Projection {
    deckmaste_semantics::Projection {
        of: minimal_countable(),
        by: std::sync::Arc::new(minimal_count()),
    }
}

pub fn minimal_property() -> deckmaste_semantics::Property {
    deckmaste_semantics::Property::Ability(std::sync::Arc::new(minimal_ability()))
}

pub fn minimal_quantity() -> deckmaste_semantics::Quantity {
    deckmaste_semantics::Quantity::Range(None, None)
}

pub fn minimal_reference() -> deckmaste_semantics::Reference {
    deckmaste_semantics::Reference::This
}

pub fn minimal_relation_predicate() -> deckmaste_semantics::RelationPredicate {
    deckmaste_semantics::RelationPredicate::ControlledBy(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_replacement() -> deckmaste_semantics::Replacement {
    deckmaste_semantics::Replacement::Instead {
        would: minimal_event_filter(),
        instead: minimal_one_shot_effect(),
    }
}

pub fn minimal_retarget_mode() -> deckmaste_semantics::RetargetMode {
    deckmaste_semantics::RetargetMode::ChangeAll
}

pub fn minimal_reveal_until() -> deckmaste_semantics::RevealUntil {
    deckmaste_semantics::RevealUntil {
        whose: minimal_reference(),
        matches: minimal_predicate(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_round_mode() -> deckmaste_semantics::RoundMode {
    deckmaste_semantics::RoundMode::RoundUp
}

pub fn minimal_sba_rule() -> deckmaste_semantics::SbaRule {
    deckmaste_semantics::SbaRule {
        scope: minimal_predicate(),
        when: minimal_condition(),
        then: minimal_one_shot_effect(),
    }
}

pub fn minimal_selection() -> deckmaste_semantics::Selection {
    deckmaste_semantics::Selection::SelectAll(minimal_predicate())
}

pub fn minimal_separate_piles() -> deckmaste_semantics::SeparatePiles {
    deckmaste_semantics::SeparatePiles {
        group: minimal_selection(),
        into: [].into(),
        by: minimal_reference(),
        note: None,
        then: None,
    }
}

pub fn minimal_simple_mana_symbol() -> deckmaste_semantics::SimpleManaSymbol {
    deckmaste_semantics::SimpleManaSymbol::Generic(0)
}

pub fn minimal_sort() -> deckmaste_semantics::Sort {
    deckmaste_semantics::Sort::Player
}

pub fn minimal_spell_ability() -> deckmaste_semantics::SpellAbility {
    deckmaste_semantics::SpellAbility {
        ability_word: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_stat() -> deckmaste_semantics::Stat {
    deckmaste_semantics::Stat::Power
}

pub fn minimal_stat_value() -> deckmaste_semantics::StatValue {
    deckmaste_semantics::StatValue::DefinedByAbility
}

pub fn minimal_state_change() -> deckmaste_semantics::StateChange {
    deckmaste_semantics::StateChange::Tapped
}

pub fn minimal_state_predicate() -> deckmaste_semantics::StatePredicate {
    deckmaste_semantics::StatePredicate::InZone(minimal_zone())
}

pub fn minimal_static_effect() -> deckmaste_semantics::StaticEffect {
    deckmaste_semantics::StaticEffect::Modify(minimal_reference(), minimal_modification())
}

pub fn minimal_status() -> deckmaste_semantics::Status {
    deckmaste_semantics::Status::Tapped
}

pub fn minimal_subtype() -> deckmaste_semantics::Subtype {
    deckmaste_semantics::Subtype {
        name: "X".into(),
        types: [].into(),
        confers: [].into(),
    }
}

pub fn minimal_subtype_ref() -> deckmaste_semantics::SubtypeRef {
    deckmaste_semantics::SubtypeRef(std::sync::Arc::new(minimal_subtype()))
}

pub fn minimal_supertype() -> deckmaste_semantics::Supertype {
    deckmaste_semantics::Supertype::Basic
}

pub fn minimal_symbol_pred() -> deckmaste_semantics::SymbolPred {
    deckmaste_semantics::SymbolPred::AnyColor
}

pub fn minimal_target_spec() -> deckmaste_semantics::TargetSpec {
    deckmaste_semantics::TargetSpec::Target(minimal_quantity(), minimal_predicate())
}

pub fn minimal_targeted() -> deckmaste_semantics::Targeted {
    deckmaste_semantics::Targeted {
        targets: [].into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_timing() -> deckmaste_semantics::Timing {
    deckmaste_semantics::Timing::InstantSpeed
}

pub fn minimal_token() -> deckmaste_semantics::Token {
    deckmaste_semantics::Token {
        name: None,
        color_indicator: [].into(),
        supertypes: [].into(),
        types: [].into(),
        subtypes: [].into(),
        abilities: [].into(),
        power: None,
        toughness: None,
    }
}

pub fn minimal_token_name() -> deckmaste_semantics::TokenName {
    deckmaste_semantics::TokenName("X".into())
}

pub fn minimal_token_spec() -> deckmaste_semantics::TokenSpec {
    deckmaste_semantics::TokenSpec::Token(std::sync::Arc::new(minimal_token()))
}

pub fn minimal_total_cost() -> deckmaste_semantics::TotalCost {
    deckmaste_semantics::TotalCost {
        base: [].into(),
        trace: [].into(),
        locked: false,
    }
}

pub fn minimal_triggered_ability() -> deckmaste_semantics::TriggeredAbility {
    deckmaste_semantics::TriggeredAbility {
        ability_word: None,
        event: minimal_event_filter(),
        from: None,
        condition: None,
        limits: [].into(),
        where_x: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_turn_marker() -> deckmaste_semantics::TurnMarker {
    deckmaste_semantics::TurnMarker::EndOfTurn
}

pub fn minimal_type() -> deckmaste_semantics::Type {
    deckmaste_semantics::Type::Artifact
}

pub fn minimal_type_def() -> deckmaste_semantics::TypeDef {
    deckmaste_semantics::TypeDef {
        name: "X".into(),
        permanent: false,
        confers: [].into(),
    }
}

pub fn minimal_type_ref() -> deckmaste_semantics::TypeRef {
    deckmaste_semantics::TypeRef(std::sync::Arc::new(minimal_type_def()))
}

pub fn minimal_use_limit() -> deckmaste_semantics::UseLimit {
    deckmaste_semantics::UseLimit::OncePerTurn
}

pub fn minimal_verb_name() -> deckmaste_semantics::VerbName {
    deckmaste_semantics::VerbName("X".into())
}

pub fn minimal_visibility() -> deckmaste_semantics::Visibility {
    deckmaste_semantics::Visibility::Open
}

pub fn minimal_whose_turn() -> deckmaste_semantics::WhoseTurn {
    deckmaste_semantics::WhoseTurn::Your
}

pub fn minimal_with() -> deckmaste_semantics::With {
    deckmaste_semantics::With {
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_zone() -> deckmaste_semantics::Zone {
    deckmaste_semantics::Zone::Battlefield
}
