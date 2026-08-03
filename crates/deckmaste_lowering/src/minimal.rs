//! Minimal values, one per grammar type, for the per-variant lowering
//! tests. Generated; each picks a variant whose fields all ground.

#![allow(
    dead_code,
    reason = "a type not used at any field position needs no helper caller"
)]

pub fn minimal_ability() -> deckmaste_authoring::Ability {
    deckmaste_authoring::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
}

pub fn minimal_action() -> deckmaste_authoring::Action {
    deckmaste_authoring::Action::DealDamage(
        minimal_reference(),
        minimal_count(),
        minimal_reference(),
    )
}

pub fn minimal_activated_ability() -> deckmaste_authoring::ActivatedAbility {
    deckmaste_authoring::ActivatedAbility {
        ability_word: None,
        cost: minimal_cost(),
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_additional_cost() -> deckmaste_authoring::AdditionalCost {
    deckmaste_authoring::AdditionalCost {
        pay: minimal_cost(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_adjacency() -> deckmaste_authoring::Adjacency {
    deckmaste_authoring::Adjacency::Above
}

pub fn minimal_agency() -> deckmaste_authoring::Agency {
    deckmaste_authoring::Agency::CostPayment
}

pub fn minimal_aggregate_op() -> deckmaste_authoring::AggregateOp {
    deckmaste_authoring::AggregateOp::SumOf
}

pub fn minimal_alternative_cost() -> deckmaste_authoring::AlternativeCost {
    deckmaste_authoring::AlternativeCost::Free
}

pub fn minimal_anchor() -> deckmaste_authoring::Anchor {
    deckmaste_authoring::Anchor::FromTop(minimal_count())
}

pub fn minimal_arrangement() -> deckmaste_authoring::Arrangement {
    deckmaste_authoring::Arrangement::ChosenOrder(minimal_reference())
}

pub fn minimal_as_though() -> deckmaste_authoring::AsThough {
    deckmaste_authoring::AsThough::Counterfactual {
        premise: minimal_predicate(),
        then: std::sync::Arc::new(minimal_deontic()),
    }
}

pub fn minimal_beginning_step() -> deckmaste_authoring::BeginningStep {
    deckmaste_authoring::BeginningStep::Untap
}

pub fn minimal_binder() -> deckmaste_authoring::Binder {
    deckmaste_authoring::Binder::TheRef(minimal_reference())
}

pub fn minimal_card() -> deckmaste_authoring::Card {
    deckmaste_authoring::Card::Normal(minimal_card_face())
}

pub fn minimal_card_face() -> deckmaste_authoring::CardFace {
    deckmaste_authoring::CardFace {
        name: "x".into(),
        mana_cost: deckmaste_authoring::ManaCost::from(std::sync::Arc::<
            [deckmaste_authoring::ManaSymbol],
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

pub fn minimal_cause() -> deckmaste_authoring::Cause {
    deckmaste_authoring::Cause::Cause(minimal_cause_pattern())
}

pub fn minimal_cause_pattern() -> deckmaste_authoring::CausePattern {
    deckmaste_authoring::CausePattern {
        verb: None,
        agency: None,
        agent: None,
    }
}

pub fn minimal_characteristic() -> deckmaste_authoring::Characteristic {
    deckmaste_authoring::Characteristic::Colors
}

pub fn minimal_characteristic_predicate() -> deckmaste_authoring::CharacteristicPredicate {
    deckmaste_authoring::CharacteristicPredicate::Type(minimal_type_ref())
}

pub fn minimal_choose_pile() -> deckmaste_authoring::ChoosePile {
    deckmaste_authoring::ChoosePile {
        from: minimal_pile_source(),
        by: minimal_reference(),
        random: false,
        then: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_choose_spec() -> deckmaste_authoring::ChooseSpec {
    deckmaste_authoring::ChooseSpec {
        count: minimal_quantity(),
        up_to: false,
        repeats: false,
        chooser: minimal_reference(),
        rider: None,
    }
}

pub fn minimal_chosen_value_kind() -> deckmaste_authoring::ChosenValueKind {
    deckmaste_authoring::ChosenValueKind::Color
}

pub fn minimal_cmp() -> deckmaste_authoring::Cmp {
    deckmaste_authoring::Cmp::Eq
}

pub fn minimal_collection_op<T>() -> deckmaste_authoring::CollectionOp<T> {
    deckmaste_authoring::CollectionOp::Set([].into())
}

pub fn minimal_color() -> deckmaste_authoring::Color {
    deckmaste_authoring::Color::White
}

pub fn minimal_color_or_colorless() -> deckmaste_authoring::ColorOrColorless {
    deckmaste_authoring::ColorOrColorless::Colorless
}

pub fn minimal_combat_step() -> deckmaste_authoring::CombatStep {
    deckmaste_authoring::CombatStep::BeginningOfCombat
}

pub fn minimal_condition() -> deckmaste_authoring::Condition {
    deckmaste_authoring::Condition::Compare(minimal_count(), minimal_cmp(), minimal_count())
}

pub fn minimal_conferral_rule() -> deckmaste_authoring::ConferralRule {
    deckmaste_authoring::ConferralRule {
        scope: minimal_predicate(),
        confer: minimal_property(),
    }
}

pub fn minimal_continuously() -> deckmaste_authoring::Continuously {
    deckmaste_authoring::Continuously {
        effect: std::sync::Arc::new(minimal_static_effect()),
        duration: minimal_duration(),
    }
}

pub fn minimal_copiable_values() -> deckmaste_authoring::CopiableValues {
    deckmaste_authoring::CopiableValues {
        name: "x".into(),
        mana_cost: deckmaste_authoring::ManaCost::from(std::sync::Arc::<
            [deckmaste_authoring::ManaSymbol],
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

pub fn minimal_copy_exception() -> deckmaste_authoring::CopyException {
    deckmaste_authoring::CopyException::Modify(minimal_modification())
}

pub fn minimal_copy_retarget() -> deckmaste_authoring::CopyRetarget {
    deckmaste_authoring::CopyRetarget::AsIs
}

pub fn minimal_copy_source() -> deckmaste_authoring::CopySource {
    deckmaste_authoring::CopySource::Object(minimal_reference())
}

pub fn minimal_copy_spec() -> deckmaste_authoring::CopySpec {
    deckmaste_authoring::CopySpec {
        source: minimal_copy_source(),
        exceptions: Vec::new(),
    }
}

pub fn minimal_cost() -> deckmaste_authoring::Cost {
    deckmaste_authoring::Cost([].into())
}

pub fn minimal_cost_change() -> deckmaste_authoring::CostChange {
    deckmaste_authoring::CostChange::Increase([].into())
}

pub fn minimal_cost_component() -> deckmaste_authoring::CostComponent {
    deckmaste_authoring::CostComponent::Mana(deckmaste_authoring::ManaCost::from(std::sync::Arc::<
        [deckmaste_authoring::ManaSymbol],
    >::from([])))
}

pub fn minimal_cost_predicate() -> deckmaste_authoring::CostPredicate {
    deckmaste_authoring::CostPredicate::IncludesTapSymbol
}

pub fn minimal_cost_tag() -> deckmaste_authoring::CostTag {
    deckmaste_authoring::CostTag("X".into())
}

pub fn minimal_count() -> deckmaste_authoring::Count {
    deckmaste_authoring::Count::X
}

pub fn minimal_count_bound() -> deckmaste_authoring::CountBound {
    deckmaste_authoring::CountBound::Eq(minimal_count())
}

pub fn minimal_countable() -> deckmaste_authoring::Countable {
    deckmaste_authoring::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_counter() -> deckmaste_authoring::Counter {
    deckmaste_authoring::Counter {
        name: "X".into(),
        scope: minimal_counter_scope(),
        confers: Vec::new(),
    }
}

pub fn minimal_counter_ref() -> deckmaste_authoring::CounterRef {
    deckmaste_authoring::CounterRef("X".into())
}

pub fn minimal_counter_scope() -> deckmaste_authoring::CounterScope {
    deckmaste_authoring::CounterScope::Object
}

pub fn minimal_counter_spec() -> deckmaste_authoring::CounterSpec {
    deckmaste_authoring::CounterSpec::Named(minimal_counter_ref(), minimal_count())
}

pub fn minimal_damage_result_rule() -> deckmaste_authoring::DamageResultRule {
    deckmaste_authoring::DamageResultRule {
        recipient: minimal_predicate(),
        remove: minimal_counter_ref(),
    }
}

pub fn minimal_decider_spec() -> deckmaste_authoring::DeciderSpec {
    deckmaste_authoring::DeciderSpec::Controller
}

pub fn minimal_deed_agent() -> deckmaste_authoring::DeedAgent {
    deckmaste_authoring::DeedAgent {
        stack_object: None,
        source: None,
    }
}

pub fn minimal_deontic() -> deckmaste_authoring::Deontic {
    deckmaste_authoring::Deontic::May(minimal_deontic_action())
}

pub fn minimal_deontic_action() -> deckmaste_authoring::DeonticAction {
    deckmaste_authoring::DeonticAction::Attack {
        by: minimal_predicate(),
        on: minimal_predicate(),
    }
}

pub fn minimal_designation_decl() -> deckmaste_authoring::DesignationDecl {
    deckmaste_authoring::DesignationDecl {
        name: "X".into(),
        definition: minimal_designation_def(),
    }
}

pub fn minimal_designation_def() -> deckmaste_authoring::DesignationDef {
    deckmaste_authoring::DesignationDef::Stored {
        scope: minimal_designation_scope(),
        shape: minimal_designation_shape(),
        uniqueness: minimal_designation_uniqueness(),
        persistence: minimal_designation_persistence(),
        payload: [].into(),
    }
}

pub fn minimal_designation_persistence() -> deckmaste_authoring::DesignationPersistence {
    deckmaste_authoring::DesignationPersistence::ObjectLifetime
}

pub fn minimal_designation_scope() -> deckmaste_authoring::DesignationScope {
    deckmaste_authoring::DesignationScope::Object
}

pub fn minimal_designation_shape() -> deckmaste_authoring::DesignationShape {
    deckmaste_authoring::DesignationShape::Flag
}

pub fn minimal_designation_uniqueness() -> deckmaste_authoring::DesignationUniqueness {
    deckmaste_authoring::DesignationUniqueness::None
}

pub fn minimal_destination() -> deckmaste_authoring::Destination {
    deckmaste_authoring::Destination::Zone(minimal_zone())
}

pub fn minimal_distribute() -> deckmaste_authoring::Distribute {
    deckmaste_authoring::Distribute {
        amount: minimal_count(),
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_duration() -> deckmaste_authoring::Duration {
    deckmaste_authoring::Duration::FixedUntil(minimal_turn_marker())
}

pub fn minimal_each() -> deckmaste_authoring::Each {
    deckmaste_authoring::Each {
        binder: minimal_binder(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_ending_step() -> deckmaste_authoring::EndingStep {
    deckmaste_authoring::EndingStep::End
}

pub fn minimal_enter_rider() -> deckmaste_authoring::EnterRider {
    deckmaste_authoring::EnterRider::Tapped
}

pub fn minimal_event_filter() -> deckmaste_authoring::EventFilter {
    deckmaste_authoring::EventFilter::ZoneChange {
        what: minimal_predicate(),
        from: None,
        to: None,
        cause: None,
    }
}

pub fn minimal_face() -> deckmaste_authoring::Face {
    deckmaste_authoring::Face::Up
}

pub fn minimal_face_down_characteristics() -> deckmaste_authoring::FaceDownCharacteristics {
    deckmaste_authoring::FaceDownCharacteristics {
        name: None,
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
    }
}

pub fn minimal_face_down_spec() -> deckmaste_authoring::FaceDownSpec {
    deckmaste_authoring::FaceDownSpec::Listed(minimal_face_down_characteristics())
}

pub fn minimal_face_layout() -> deckmaste_authoring::FaceLayout {
    deckmaste_authoring::FaceLayout::Transforming
}

pub fn minimal_if() -> deckmaste_authoring::If {
    deckmaste_authoring::If {
        condition: minimal_condition(),
        then: std::sync::Arc::new(minimal_one_shot_effect()),
        otherwise: None,
    }
}

pub fn minimal_ignore_rule() -> deckmaste_authoring::IgnoreRule {
    deckmaste_authoring::IgnoreRule::IgnoreLowest
}

pub fn minimal_keyword_ability() -> deckmaste_authoring::KeywordAbility {
    deckmaste_authoring::KeywordAbility::FirstStrike
}

pub fn minimal_keyword_decl() -> deckmaste_authoring::KeywordDecl {
    deckmaste_authoring::KeywordDecl {
        name: "X".into(),
        shape: minimal_param_shape(),
    }
}

pub fn minimal_keyword_ref() -> deckmaste_authoring::KeywordRef {
    deckmaste_authoring::KeywordRef("X".into())
}

pub fn minimal_label() -> deckmaste_authoring::Label {
    deckmaste_authoring::Label {
        r#as: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_life_op() -> deckmaste_authoring::LifeOp {
    deckmaste_authoring::LifeOp::Set(minimal_count())
}

pub fn minimal_lock_point() -> deckmaste_authoring::LockPoint {
    deckmaste_authoring::LockPoint::Announce
}

pub fn minimal_lookback() -> deckmaste_authoring::Lookback {
    deckmaste_authoring::Lookback::ThisTurn
}

pub fn minimal_mana_cost() -> deckmaste_authoring::ManaCost {
    deckmaste_authoring::ManaCost::from(std::sync::Arc::<[deckmaste_authoring::ManaSymbol]>::from(
        [],
    ))
}

pub fn minimal_mana_production() -> deckmaste_authoring::ManaProduction {
    deckmaste_authoring::ManaProduction::WithRiders {
        mana: minimal_mana_spec(),
        riders: [].into(),
    }
}

pub fn minimal_mana_rider() -> deckmaste_authoring::ManaRider {
    deckmaste_authoring::ManaRider::SpendOnly(minimal_predicate())
}

pub fn minimal_mana_spec() -> deckmaste_authoring::ManaSpec {
    deckmaste_authoring::ManaSpec::AnyColor
}

pub fn minimal_mana_symbol() -> deckmaste_authoring::ManaSymbol {
    deckmaste_authoring::ManaSymbol::Variable
}

pub fn minimal_may() -> deckmaste_authoring::May {
    deckmaste_authoring::May {
        who: minimal_reference(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
        if_did: None,
        if_not: None,
    }
}

pub fn minimal_modal() -> deckmaste_authoring::Modal {
    deckmaste_authoring::Modal {
        choose: minimal_choose_spec(),
        modes: [].into(),
    }
}

pub fn minimal_modal_cost_rider() -> deckmaste_authoring::ModalCostRider {
    deckmaste_authoring::ModalCostRider::Entwine(minimal_cost())
}

pub fn minimal_mode() -> deckmaste_authoring::Mode {
    deckmaste_authoring::Mode {
        effect: minimal_one_shot_effect(),
        cost: None,
    }
}

pub fn minimal_modification() -> deckmaste_authoring::Modification {
    deckmaste_authoring::Modification::Power(minimal_numeric_op())
}

pub fn minimal_noted_kind() -> deckmaste_authoring::NotedKind {
    deckmaste_authoring::NotedKind::Objects
}

pub fn minimal_noting() -> deckmaste_authoring::Noting {
    deckmaste_authoring::Noting {
        key: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_numeric_op() -> deckmaste_authoring::NumericOp {
    deckmaste_authoring::NumericOp::Set(minimal_stat_value())
}

pub fn minimal_object_kind() -> deckmaste_authoring::ObjectKind {
    deckmaste_authoring::ObjectKind::Ability
}

pub fn minimal_one_shot_effect() -> deckmaste_authoring::OneShotEffect {
    deckmaste_authoring::OneShotEffect::Act(minimal_action())
}

pub fn minimal_optional_cost() -> deckmaste_authoring::OptionalCost {
    deckmaste_authoring::OptionalCost {
        components: [].into(),
        tag: minimal_cost_tag(),
        repeatable: false,
    }
}

pub fn minimal_outcome_gate_kind() -> deckmaste_authoring::OutcomeGateKind {
    deckmaste_authoring::OutcomeGateKind::CantLose
}

pub fn minimal_param_shape() -> deckmaste_authoring::ParamShape {
    deckmaste_authoring::ParamShape::None
}

pub fn minimal_pay_act() -> deckmaste_authoring::PayAct {
    deckmaste_authoring::PayAct::TapToPay(minimal_predicate())
}

pub fn minimal_phase_kind() -> deckmaste_authoring::PhaseKind {
    deckmaste_authoring::PhaseKind::Beginning
}

pub fn minimal_phase_step() -> deckmaste_authoring::PhaseStep {
    deckmaste_authoring::PhaseStep::Beginning(minimal_beginning_step())
}

pub fn minimal_phasing() -> deckmaste_authoring::Phasing {
    deckmaste_authoring::Phasing::In
}

pub fn minimal_pile_source() -> deckmaste_authoring::PileSource {
    deckmaste_authoring::PileSource::Labels([].into())
}

pub fn minimal_pip_class() -> deckmaste_authoring::PipClass {
    deckmaste_authoring::PipClass::Generic
}

pub fn minimal_planar_face() -> deckmaste_authoring::PlanarFace {
    deckmaste_authoring::PlanarFace::Blank
}

pub fn minimal_player_attr() -> deckmaste_authoring::PlayerAttr {
    deckmaste_authoring::PlayerAttr::Life
}

pub fn minimal_player_mod() -> deckmaste_authoring::PlayerMod {
    deckmaste_authoring::PlayerMod::SetTo(minimal_player_attr(), minimal_count())
}

pub fn minimal_predefined_token() -> deckmaste_authoring::PredefinedToken {
    deckmaste_authoring::PredefinedToken::Treasure
}

pub fn minimal_predicate() -> deckmaste_authoring::Predicate {
    deckmaste_authoring::Predicate::Kind(minimal_object_kind())
}

pub fn minimal_prevention() -> deckmaste_authoring::Prevention {
    deckmaste_authoring::Prevention::PreventNext {
        n: minimal_count(),
        from: minimal_predicate(),
        to: minimal_predicate(),
        duration: None,
    }
}

pub fn minimal_projection() -> deckmaste_authoring::Projection {
    deckmaste_authoring::Projection {
        of: minimal_countable(),
        by: std::sync::Arc::new(minimal_count()),
    }
}

pub fn minimal_property() -> deckmaste_authoring::Property {
    deckmaste_authoring::Property::Ability(std::sync::Arc::new(minimal_ability()))
}

pub fn minimal_quantity() -> deckmaste_authoring::Quantity {
    deckmaste_authoring::Quantity::Range(None, None)
}

pub fn minimal_reference() -> deckmaste_authoring::Reference {
    deckmaste_authoring::Reference::This
}

pub fn minimal_relation_predicate() -> deckmaste_authoring::RelationPredicate {
    deckmaste_authoring::RelationPredicate::ControlledBy(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_replacement() -> deckmaste_authoring::Replacement {
    deckmaste_authoring::Replacement::Instead {
        would: minimal_event_filter(),
        instead: minimal_one_shot_effect(),
    }
}

pub fn minimal_retarget_mode() -> deckmaste_authoring::RetargetMode {
    deckmaste_authoring::RetargetMode::ChangeAll
}

pub fn minimal_reveal_until() -> deckmaste_authoring::RevealUntil {
    deckmaste_authoring::RevealUntil {
        whose: minimal_reference(),
        matches: minimal_predicate(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_round_mode() -> deckmaste_authoring::RoundMode {
    deckmaste_authoring::RoundMode::RoundUp
}

pub fn minimal_sba_rule() -> deckmaste_authoring::SbaRule {
    deckmaste_authoring::SbaRule {
        scope: minimal_predicate(),
        when: minimal_condition(),
        then: minimal_one_shot_effect(),
    }
}

pub fn minimal_selection() -> deckmaste_authoring::Selection {
    deckmaste_authoring::Selection::SelectAll(minimal_predicate())
}

pub fn minimal_separate_piles() -> deckmaste_authoring::SeparatePiles {
    deckmaste_authoring::SeparatePiles {
        group: minimal_selection(),
        into: [].into(),
        by: minimal_reference(),
        note: None,
        then: None,
    }
}

pub fn minimal_simple_mana_symbol() -> deckmaste_authoring::SimpleManaSymbol {
    deckmaste_authoring::SimpleManaSymbol::Generic(0)
}

pub fn minimal_sort() -> deckmaste_authoring::Sort {
    deckmaste_authoring::Sort::Player
}

pub fn minimal_spell_ability() -> deckmaste_authoring::SpellAbility {
    deckmaste_authoring::SpellAbility {
        ability_word: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_stat() -> deckmaste_authoring::Stat {
    deckmaste_authoring::Stat::Power
}

pub fn minimal_stat_value() -> deckmaste_authoring::StatValue {
    deckmaste_authoring::StatValue::DefinedByAbility
}

pub fn minimal_state_change() -> deckmaste_authoring::StateChange {
    deckmaste_authoring::StateChange::Tapped
}

pub fn minimal_state_predicate() -> deckmaste_authoring::StatePredicate {
    deckmaste_authoring::StatePredicate::InZone(minimal_zone())
}

pub fn minimal_static_effect() -> deckmaste_authoring::StaticEffect {
    deckmaste_authoring::StaticEffect::Modify(minimal_reference(), minimal_modification())
}

pub fn minimal_status() -> deckmaste_authoring::Status {
    deckmaste_authoring::Status::Tapped
}

pub fn minimal_subtype() -> deckmaste_authoring::Subtype {
    deckmaste_authoring::Subtype {
        name: "X".into(),
        types: [].into(),
        confers: [].into(),
    }
}

pub fn minimal_subtype_ref() -> deckmaste_authoring::SubtypeRef {
    deckmaste_authoring::SubtypeRef(std::sync::Arc::new(minimal_subtype()))
}

pub fn minimal_supertype() -> deckmaste_authoring::Supertype {
    deckmaste_authoring::Supertype::Basic
}

pub fn minimal_symbol_pred() -> deckmaste_authoring::SymbolPred {
    deckmaste_authoring::SymbolPred::AnyColor
}

pub fn minimal_target_spec() -> deckmaste_authoring::TargetSpec {
    deckmaste_authoring::TargetSpec::Target(minimal_quantity(), minimal_predicate())
}

pub fn minimal_targeted() -> deckmaste_authoring::Targeted {
    deckmaste_authoring::Targeted {
        targets: [].into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_timing() -> deckmaste_authoring::Timing {
    deckmaste_authoring::Timing::InstantSpeed
}

pub fn minimal_token() -> deckmaste_authoring::Token {
    deckmaste_authoring::Token {
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

pub fn minimal_token_name() -> deckmaste_authoring::TokenName {
    deckmaste_authoring::TokenName("X".into())
}

pub fn minimal_token_spec() -> deckmaste_authoring::TokenSpec {
    deckmaste_authoring::TokenSpec::Token(std::sync::Arc::new(minimal_token()))
}

pub fn minimal_total_cost() -> deckmaste_authoring::TotalCost {
    deckmaste_authoring::TotalCost {
        base: [].into(),
        trace: [].into(),
        locked: false,
    }
}

pub fn minimal_triggered_ability() -> deckmaste_authoring::TriggeredAbility {
    deckmaste_authoring::TriggeredAbility {
        ability_word: None,
        event: minimal_event_filter(),
        from: None,
        condition: None,
        limits: [].into(),
        where_x: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_turn_marker() -> deckmaste_authoring::TurnMarker {
    deckmaste_authoring::TurnMarker::EndOfTurn
}

pub fn minimal_type() -> deckmaste_authoring::Type {
    deckmaste_authoring::Type::Artifact
}

pub fn minimal_type_def() -> deckmaste_authoring::TypeDef {
    deckmaste_authoring::TypeDef {
        name: "X".into(),
        permanent: false,
        confers: [].into(),
    }
}

pub fn minimal_type_ref() -> deckmaste_authoring::TypeRef {
    deckmaste_authoring::TypeRef(std::sync::Arc::new(minimal_type_def()))
}

pub fn minimal_use_limit() -> deckmaste_authoring::UseLimit {
    deckmaste_authoring::UseLimit::OncePerTurn
}

pub fn minimal_verb_name() -> deckmaste_authoring::VerbName {
    deckmaste_authoring::VerbName("X".into())
}

pub fn minimal_visibility() -> deckmaste_authoring::Visibility {
    deckmaste_authoring::Visibility::Open
}

pub fn minimal_whose_turn() -> deckmaste_authoring::WhoseTurn {
    deckmaste_authoring::WhoseTurn::Your
}

pub fn minimal_with() -> deckmaste_authoring::With {
    deckmaste_authoring::With {
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_zone() -> deckmaste_authoring::Zone {
    deckmaste_authoring::Zone::Battlefield
}
