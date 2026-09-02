use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Cmp;
use crate::CostBinder;
use crate::Count;
use crate::Normalize;
use crate::Predicate;
use crate::Stat;
use crate::mana::ManaCost;
use crate::reference::Reference;

/// A cost-eligible action whose payment subjects are already bound.
///
/// The semantic grammar may place a chooser or random selection inside a
/// keyword-action composite (notably discard). Lowering lifts that binder into
/// [`CostComponent::ChooseAndPay`] and constructs this wrapper only around the
/// remaining bound action. The private field makes it impossible for runnable
/// core costs to bypass that check.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunnableCostAction(Arc<crate::Action>);

/// Why an [`Action`](crate::Action) cannot cross the runnable-cost boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RunnableCostActionError {
    #[error("the action is not eligible to be performed as a cost")]
    Ineligible,
    #[error("the action retains an unresolved choice or random subject")]
    UnresolvedSubject,
}

impl RunnableCostAction {
    /// Check and wrap one lowered action.
    ///
    /// # Errors
    ///
    /// Returns [`RunnableCostActionError::Ineligible`] for an effect-only
    /// action and [`RunnableCostActionError::UnresolvedSubject`] when a
    /// chooser, search, producer, random selection, or other decision remains
    /// inside the action instead of being lifted into payment position.
    pub fn try_new(action: crate::Action) -> Result<Self, RunnableCostActionError> {
        validate_runnable_cost_action(&action)?;
        Ok(Self(Arc::new(action)))
    }

    /// Borrow the checked action.
    #[must_use]
    pub fn as_action(&self) -> &crate::Action {
        &self.0
    }

    /// Clone the checked action's shared allocation.
    #[must_use]
    pub fn arc(&self) -> Arc<crate::Action> {
        Arc::clone(&self.0)
    }

    /// Consume the wrapper and return its shared action allocation.
    #[must_use]
    pub fn into_arc(self) -> Arc<crate::Action> {
        self.0
    }
}

impl std::ops::Deref for RunnableCostAction {
    type Target = crate::Action;

    fn deref(&self) -> &Self::Target {
        self.as_action()
    }
}

impl AsRef<crate::Action> for RunnableCostAction {
    fn as_ref(&self) -> &crate::Action {
        self.as_action()
    }
}

impl TryFrom<crate::Action> for RunnableCostAction {
    type Error = RunnableCostActionError;

    fn try_from(action: crate::Action) -> Result<Self, Self::Error> {
        Self::try_new(action)
    }
}

impl Serialize for RunnableCostAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_action().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RunnableCostAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let action = crate::Action::deserialize(deserializer)?;
        Self::try_new(action).map_err(serde::de::Error::custom)
    }
}

fn validate_runnable_cost_action(action: &crate::Action) -> Result<(), RunnableCostActionError> {
    if !action.is_cost_eligible() {
        return Err(RunnableCostActionError::Ineligible);
    }
    if !runnable_action_subjects_are_bound(action) {
        return Err(RunnableCostActionError::UnresolvedSubject);
    }
    Ok(())
}

fn runnable_action_subjects_are_bound(action: &crate::Action) -> bool {
    match action {
        crate::Action::Sacrifice(agent, subject) => {
            runnable_reference_is_bound(agent) && runnable_reference_is_bound(subject)
        }
        crate::Action::Move(subject, destination, riders, _) => {
            runnable_reference_is_bound(subject)
                && runnable_destination_is_bound(destination)
                && riders.iter().all(runnable_enter_rider_is_bound)
        }
        crate::Action::Tap(subject) | crate::Action::Untap(subject) => {
            runnable_reference_is_bound(subject)
        }
        crate::Action::ChangeLife(subject, operation) => {
            runnable_reference_is_bound(subject) && runnable_life_op_is_bound(operation)
        }
        crate::Action::PutCounters(subject, _, count)
        | crate::Action::RemoveCounters(subject, _, count) => {
            runnable_reference_is_bound(subject) && runnable_count_is_bound(count)
        }
        crate::Action::Reveal { what, to } => {
            runnable_reference_is_bound(what) && to.as_ref().is_none_or(runnable_reference_is_bound)
        }
        crate::Action::Composite { body, .. } => runnable_discard_effect_is_bound(body),
        _ => true,
    }
}

fn runnable_discard_effect_is_bound(effect: &crate::OneShotEffect) -> bool {
    match effect {
        crate::OneShotEffect::Act { action, .. } => {
            action.is_cost_eligible() && runnable_action_subjects_are_bound(action)
        }
        crate::OneShotEffect::Each(each) => {
            each.body.body.iter().all(runnable_discard_effect_is_bound)
        }
        _ => false,
    }
}

fn runnable_reference_is_bound(reference: &Reference) -> bool {
    match reference {
        Reference::Single(selection) => runnable_selection_is_bound(selection),
        Reference::OpponentOf(reference)
        | Reference::ControllerOf(reference)
        | Reference::OwnerOf(reference)
        | Reference::AttachHostOf(reference) => runnable_reference_is_bound(reference),
        Reference::Coalesce(references) => references.iter().all(runnable_reference_is_bound),
        Reference::Reg(_) | Reference::Bound(_) | Reference::Linked(_) | Reference::Source => true,
    }
}

fn runnable_enter_rider_is_bound(rider: &crate::EnterRider) -> bool {
    match rider {
        crate::EnterRider::UnderControlOf(controller) => runnable_reference_is_bound(controller),
        crate::EnterRider::Attacking(defender) => {
            defender.as_ref().is_none_or(runnable_reference_is_bound)
        }
        crate::EnterRider::AsCopy(spec) => runnable_copy_spec_is_bound(spec),
        crate::EnterRider::WithCounters(_, count) => runnable_count_is_bound(count),
        crate::EnterRider::Tapped
        | crate::EnterRider::FaceDown
        | crate::EnterRider::UnderOwnersControl => true,
    }
}

fn runnable_destination_is_bound(destination: &crate::Destination) -> bool {
    match destination {
        crate::Destination::Zone(_) => true,
        crate::Destination::Library(
            crate::Anchor::FromTop(count) | crate::Anchor::FromBottom(count),
        ) => runnable_count_is_bound(count),
    }
}

fn runnable_copy_spec_is_bound(spec: &crate::CopySpec) -> bool {
    let source_is_bound = match &spec.source {
        crate::CopySource::Object(source) => runnable_reference_is_bound(source),
        crate::CopySource::SelfCard => true,
    };
    source_is_bound
        && spec.exceptions.iter().all(|exception| match exception {
            crate::CopyException::Modify(modification) => {
                runnable_modification_is_bound(modification)
            }
            crate::CopyException::Retain(_) => true,
            crate::CopyException::AdditionalEffect(rider) => runnable_enter_rider_is_bound(rider),
        })
}

fn runnable_modification_is_bound(modification: &crate::Modification) -> bool {
    use crate::Modification;

    match modification {
        Modification::Power(operation)
        | Modification::Toughness(operation)
        | Modification::BaseLoyalty(operation)
        | Modification::BaseDefense(operation) => runnable_numeric_op_is_bound(operation),
        Modification::SetController(controller) => runnable_reference_is_bound(controller),
        Modification::Several(modifications) => {
            modifications.iter().all(runnable_modification_is_bound)
        }
        // Ability payloads can contain arbitrary action/effect grammar. A
        // cost-side copy exception must not smuggle that open structure past
        // the runnable boundary.
        Modification::GainAbility(_) => false,
        Modification::SwitchPowerToughness
        | Modification::Colors(_)
        | Modification::CardTypes(_)
        | Modification::Subtypes(_)
        | Modification::Supertypes(_)
        | Modification::LoseAbility(_)
        | Modification::LoseAllAbilities
        | Modification::CantHaveAbility(_)
        | Modification::SetText(_)
        | Modification::AllCreatureTypes
        | Modification::BecomeBasicLandType(_) => true,
    }
}

fn runnable_numeric_op_is_bound(operation: &crate::NumericOp) -> bool {
    match operation {
        crate::NumericOp::Set(crate::StatValue::Count(count))
        | crate::NumericOp::Up(count)
        | crate::NumericOp::Down(count) => runnable_count_is_bound(count),
        crate::NumericOp::Set(
            crate::StatValue::DefinedByAbility
            | crate::StatValue::Variable
            | crate::StatValue::Number(_),
        ) => true,
    }
}

fn runnable_life_op_is_bound(operation: &crate::LifeOp) -> bool {
    match operation {
        crate::LifeOp::Set(count) | crate::LifeOp::Up(count) | crate::LifeOp::Down(count) => {
            runnable_count_is_bound(count)
        }
    }
}

fn runnable_quantity_is_bound(quantity: &crate::Quantity) -> bool {
    let (lower, upper) = quantity.bounds();
    lower.is_none_or(runnable_count_is_bound) && upper.is_none_or(runnable_count_is_bound)
}

fn runnable_count_is_bound(count: &crate::Count) -> bool {
    use crate::Count;

    match count {
        Count::CountOf(countable) | Count::CountDistinct(_, countable) => {
            runnable_countable_is_bound(countable)
        }
        Count::StatOf(reference, _)
        | Count::PlayerStatOf(reference, _)
        | Count::Opponents(reference)
        | Count::TargetsOf(reference)
        | Count::Damage(reference)
        | Count::ManaAvailable(reference)
        | Count::ManaAvailableKind(reference, _) => runnable_reference_is_bound(reference),
        Count::CounterCount(reference, _) => runnable_reference_is_bound(reference),
        Count::Min(left, right)
        | Count::Max(left, right)
        | Count::Plus(left, right)
        | Count::Minus(left, right)
        | Count::Times(left, right)
        | Count::Divide(_, left, right)
        | Count::Mod(left, right)
        | Count::Pow(left, right) => {
            runnable_count_is_bound(left) && runnable_count_is_bound(right)
        }
        Count::Half(_, count) => runnable_count_is_bound(count),
        Count::Aggregate(_, projection) => {
            runnable_countable_is_bound(&projection.of)
                && runnable_count_is_bound(&projection.by.body)
        }
        // History predicates are an open recursive grammar. They are not a
        // supported runnable cost magnitude until that grammar has its own
        // checked representation.
        Count::EventCount(..) | Count::EventSum(..) => false,
        Count::Reg(_) | Count::X | Count::Noted(_) | Count::TimesPaid(_) | Count::Literal(_) => {
            true
        }
    }
}

fn runnable_countable_is_bound(countable: &crate::Countable) -> bool {
    match countable {
        crate::Countable::Objects(predicate) | crate::Countable::Players(predicate) => {
            runnable_predicate_is_bound(&predicate.body)
        }
        crate::Countable::ManaSymbols(reference, _)
        | crate::Countable::Singleton(reference)
        | crate::Countable::ManaSpentMatching(reference, _) => {
            runnable_reference_is_bound(reference)
        }
    }
}

fn runnable_predicate_is_bound(predicate: &crate::Predicate) -> bool {
    use crate::Predicate;

    match predicate {
        Predicate::Characteristic(crate::CharacteristicPredicate::Stat(_, _, count))
        | Predicate::PlayerStatCmp(_, _, count) => runnable_count_is_bound(count),
        Predicate::State(
            crate::StatePredicate::RelatedBy(_, predicate)
            | crate::StatePredicate::Targets(predicate),
        )
        | Predicate::FromSource(predicate)
        | Predicate::Not(predicate) => runnable_predicate_is_bound(predicate),
        Predicate::State(crate::StatePredicate::TargetCount(bound)) => {
            runnable_count_is_bound(bound.split().1)
        }
        Predicate::Relation(relation) => match relation {
            crate::RelationPredicate::ControlledBy(predicate)
            | crate::RelationPredicate::Controls(predicate)
            | crate::RelationPredicate::Owner(predicate)
            | crate::RelationPredicate::OpponentOf(predicate)
            | crate::RelationPredicate::TeammateOf(predicate)
            | crate::RelationPredicate::AttachedTo(predicate)
            | crate::RelationPredicate::Attachment(predicate) => {
                runnable_predicate_is_bound(predicate)
            }
        },
        Predicate::Ref(reference) | Predicate::Adjacent(_, reference) => {
            runnable_reference_is_bound(reference)
        }
        Predicate::And(predicates) | Predicate::Or(predicates) => {
            predicates.iter().all(runnable_predicate_is_bound)
        }
        // Conditions can embed every event/action/reference family. Keep the
        // checked cost representation closed until a complete visitor exists.
        Predicate::Where(_) => false,
        Predicate::Kind(_)
        | Predicate::Characteristic(_)
        | Predicate::State(_)
        | Predicate::Any => true,
    }
}

fn runnable_selection_is_bound(selection: &crate::Selection) -> bool {
    match selection {
        crate::Selection::Reg(_) => true,
        crate::Selection::Union(selections) => selections.iter().all(runnable_selection_is_bound),
        crate::Selection::SelectAll(_)
        | crate::Selection::InChosenOrder(..)
        | crate::Selection::Random(..)
        | crate::Selection::TopOfLibrary { .. }
        | crate::Selection::BottomOfLibrary { .. }
        | crate::Selection::LibraryOf(_)
        | crate::Selection::TopOfGraveyard { .. }
        | crate::Selection::ValidTargetsFor(_)
        | crate::Selection::PilesOf { .. }
        | crate::Selection::Pick { .. } => false,
    }
}

/// Whether a binder can safely cross the runnable-cost boundary.
///
/// Payment samples only the exact top-level
/// `Existing(Selection::Random(..))` shape. A random selection hidden inside
/// any other binder expression would reach the ordinary query evaluator
/// without a sampled value, so this predicate rejects it. Producer binders
/// are narrower still: only a `Move` with already-bound operands has a product
/// that the current payment runtime can capture. Search whiff branches are
/// rejected until arbitrary effects have an equivalent checked boundary.
#[must_use]
pub fn cost_binder_is_runnable(binder: &CostBinder) -> bool {
    match binder {
        CostBinder::TheRef(reference) => cost_binder_reference_is_supported(reference),
        CostBinder::ChooseOne { filter, by } => {
            runnable_predicate_is_bound(filter) && cost_binder_reference_is_supported(by)
        }
        CostBinder::Choose {
            quantity,
            filter,
            by,
        } => {
            runnable_quantity_is_bound(quantity)
                && runnable_predicate_is_bound(filter)
                && cost_binder_reference_is_supported(by)
        }
        CostBinder::Produce(action) => cost_binder_producer_is_supported(action),
        CostBinder::SearchOne {
            filter,
            by,
            whose,
            if_none,
            ..
        } => {
            if_none.is_none()
                && runnable_predicate_is_bound(filter)
                && cost_binder_reference_is_supported(by)
                && cost_binder_reference_is_supported(whose)
        }
        CostBinder::Search {
            quantity,
            filter,
            by,
            whose,
            if_none,
            ..
        } => {
            if_none.is_none()
                && runnable_quantity_is_bound(quantity)
                && runnable_predicate_is_bound(filter)
                && cost_binder_reference_is_supported(by)
                && cost_binder_reference_is_supported(whose)
        }
        CostBinder::Existing(crate::Selection::Random(quantity, filter)) => {
            runnable_quantity_is_bound(quantity) && runnable_predicate_is_bound(filter)
        }
        CostBinder::Existing(selection) => cost_binder_selection_is_supported(selection),
    }
}

fn cost_binder_producer_is_supported(action: &crate::Action) -> bool {
    match action {
        crate::Action::Move(..) => validate_runnable_cost_action(action).is_ok(),
        _ => false,
    }
}

fn cost_binder_reference_is_supported(reference: &Reference) -> bool {
    match reference {
        Reference::Single(selection) => cost_binder_selection_is_supported(selection),
        Reference::OpponentOf(reference)
        | Reference::ControllerOf(reference)
        | Reference::OwnerOf(reference)
        | Reference::AttachHostOf(reference) => cost_binder_reference_is_supported(reference),
        Reference::Coalesce(references) => {
            references.iter().all(cost_binder_reference_is_supported)
        }
        Reference::Reg(_) | Reference::Bound(_) | Reference::Linked(_) | Reference::Source => true,
    }
}

fn cost_binder_selection_is_supported(selection: &crate::Selection) -> bool {
    match selection {
        crate::Selection::Random(..) => false,
        crate::Selection::Reg(_) => true,
        crate::Selection::SelectAll(filter) => runnable_predicate_is_bound(&filter.body),
        crate::Selection::Union(selections) => {
            selections.iter().all(cost_binder_selection_is_supported)
        }
        crate::Selection::InChosenOrder(selection, by) => {
            cost_binder_selection_is_supported(selection) && cost_binder_reference_is_supported(by)
        }
        crate::Selection::TopOfLibrary { count, whose }
        | crate::Selection::BottomOfLibrary { count, whose }
        | crate::Selection::TopOfGraveyard { count, of: whose } => {
            runnable_count_is_bound(count) && cost_binder_reference_is_supported(whose)
        }
        crate::Selection::LibraryOf(whose)
        | crate::Selection::ValidTargetsFor(whose)
        | crate::Selection::PilesOf { of: whose, .. } => cost_binder_reference_is_supported(whose),
        crate::Selection::Pick { proj, .. } => {
            runnable_countable_is_bound(&proj.of) && runnable_count_is_bound(&proj.by.body)
        }
    }
}

/// A single component of an ability's cost ([CR#601.2b]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CostComponent {
    /// Mana payment, e.g. `Mana([Generic(2)])`.
    Mana(ManaCost),
    /// Pay mana equal to a referenced object's printed mana cost
    /// ([CR#202.1]) — the full *colored* cost, the cost-language twin of the
    /// numeric `ManaValueOf` read. Spells the granted-flashback "the flashback
    /// cost is equal to its mana cost" (Snapcaster Mage), where the referenced
    /// object is the card the flashback ability rides on (`This`). The engine
    /// resolves the reference, reads that object's mana cost, and requires
    /// paying it ([CR#601.2h]).
    ManaCostOf(Reference),
    /// The {T} symbol ([CR#107.5]).
    Tap,
    /// The {Q} symbol.
    Untap,
    /// Pay by performing an action ([CR#118.3]) — the Idris `Do : Action b
    /// -> Cost b`. The payer is `You` in cost context — spelled explicitly on
    /// every agent-bearing verb (`Do(Sacrifice(You, This))`, Law 2: no
    /// read-time default). [`RunnableCostAction`] retains the full action so a
    /// keyword-action composite can be a cost — "Discard a card:" is lifted
    /// into [`ChooseAndPay`](CostComponent::ChooseAndPay), while cycling's
    /// "Discard this card:" keeps the bound
    /// [`discard_what`](crate::Action::discard_what) action
    /// ([CR#701.9,702.29a]). Its private checked constructor enforces both
    /// [`Action::is_cost_eligible`](crate::Action::is_cost_eligible) and the
    /// absence of unresolved chooser/random subjects. The wrapper owns a
    /// shared action allocation, keeping this list element compact.
    Act(RunnableCostAction),
    /// A *nested* cost list produced when semantic lowering splices a
    /// list-valued parameter into a larger cost list (cycling,
    /// [CR#702.29a]). Plain deserialization preserves the nested value;
    /// [`Cost::normalize`] splices it into the surrounding list before the
    /// engine consumes it.
    Cost(Cost),
    /// An *aggregate-stat* cost: tap a chosen subset of the permanents matching
    /// `filter` whose summed `stat` satisfies `cmp` `count` ([CR#601.2b],
    /// tapping [CR#107.5]). The one shape for Crew — "Crew N" = "tap any number
    /// of other untapped creatures you control with total power N or greater"
    /// ([CR#702.122a]) is `TapTotal(Power, AtLeast, N, <those creatures>)` —
    /// and the convoke/devotion-scaling cost family the engine authors
    /// flagged it should subsume. The payer chooses which qualifying
    /// permanents to tap; the engine taps a subset whose summed `stat`
    /// meets the bound. `stat` is a numeric axis (power/toughness/defense).
    /// `filter` is an open [`Predicate`], so it stays plugin-safe; it is boxed
    /// because an open filter is large and `CostComponent` rides in
    /// `Vec<CostComponent>` cost lists, so an unboxed field would size
    /// every element to it (`clippy::large_enum_variant`).
    ///
    /// Named fields keep this four-part shape readable in core RON.
    TapTotal {
        stat: Stat,
        cmp: Cmp,
        count: Count,
        filter: Arc<Predicate>,
    },
    /// A choice/bind step made BEFORE the cost actions it scopes
    /// ([CR#601.2b]). The `binder` makes the choice (e.g.
    /// `ChooseOne(Creature)`), writes it to `dest`, and `body` pays through
    /// that register. Keeps choosing
    /// OUT of the verb — the cost action receives an already-bound
    /// reference. `binder` is boxed (an open [`CostBinder`] is large and
    /// `CostComponent` rides in `Vec<CostComponent>` cost lists).
    ChooseAndPay {
        dest: crate::DefId,
        binder: Arc<CostBinder>,
        body: Cost,
    },
}

impl CostComponent {
    /// Pay by performing a full [`Action`](crate::Action) — the
    /// keyword-action composite forms ("Discard a card:" —
    /// [CR#701.9,601.2b]).
    ///
    /// # Panics
    ///
    /// Panics when `action` is not eligible as a cost or retains an unresolved
    /// payment subject. Use [`Self::try_do_action`] for untrusted input.
    #[must_use]
    pub fn do_action(action: crate::Action) -> CostComponent {
        Self::try_do_action(action).expect("CostComponent::do_action requires a runnable action")
    }

    /// Fallible constructor for an action cost.
    ///
    /// # Errors
    ///
    /// Returns the same checked-boundary error as
    /// [`RunnableCostAction::try_new`].
    pub fn try_do_action(action: crate::Action) -> Result<CostComponent, RunnableCostActionError> {
        RunnableCostAction::try_new(action).map(CostComponent::Act)
    }
}

/// A cost: an ordered list of [`CostComponent`]s ([CR#601.2b]). A newtype
/// (not a bare `Vec`) so it can carry a [`Normalize`] impl. Plain serde
/// preserves a nested [`CostComponent::Cost`] produced by semantic lowering
/// (cycling, [CR#702.29a]). [`Cost::normalize`] is the explicit boundary step that
/// splices the nested `Cost` back into one flat list, yielding
/// `[Mana(…), Do(…)]`. Serializes and deserializes transparently as the bare
/// list.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Cost(pub Arc<[CostComponent]>);

impl Normalize for CostComponent {
    /// Recurse into a nested cost; every other variant is a leaf for
    /// normalization (no in-scope structural redundancy below it).
    fn normalize(self) -> Self {
        match self {
            CostComponent::Cost(inner) => CostComponent::Cost(inner.normalize()),
            // Recurse into a choice step's scoped body so a nested cost there
            // still flattens.
            CostComponent::ChooseAndPay { dest, binder, body } => CostComponent::ChooseAndPay {
                dest,
                binder,
                body: body.normalize(),
            },
            other => other,
        }
    }
}

impl Normalize for Cost {
    /// Splice every [`CostComponent::Cost`] one level into the surrounding
    /// list (associativity of cost concatenation, [CR#601.2b]) — the inner
    /// list is already normalized by the per-component recursion, so one pass
    /// flattens arbitrarily-deep nesting. Semantic lowering may produce the
    /// nested shape; `.normalize()` collapses it at the engine boundary
    /// (cycling, [CR#702.29a]).
    fn normalize(self) -> Self {
        let mut flat: Vec<CostComponent> = Vec::with_capacity(self.0.len());
        for component in self.0.iter().cloned() {
            match component.normalize() {
                CostComponent::Cost(inner) => flat.extend(inner.0.iter().cloned()),
                other => flat.push(other),
            }
        }
        Cost(flat.into())
    }
}

impl std::ops::Deref for Cost {
    type Target = Arc<[CostComponent]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Arc<[CostComponent]>> for Cost {
    fn from(components: Arc<[CostComponent]>) -> Self {
        Cost(components)
    }
}

impl<'a> IntoIterator for &'a Cost {
    type Item = &'a CostComponent;
    type IntoIter = std::slice::Iter<'a, CostComponent>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// A bare-identifier NAME for an optional cost ([CR#702.33e,607] — kicker's
/// linked "if it was kicked" readers refer to the specific kicker ability by
/// identity). Written as a bare ident in RON (`tag: Kicker`), mirroring
/// [`CounterRef`](crate::CounterRef)/[`KeywordRef`](crate::KeywordRef).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostTag(pub crate::Ident);

impl CostTag {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}

impl From<&str> for CostTag {
    fn from(s: &str) -> Self {
        CostTag(s.into())
    }
}

impl Serialize for CostTag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A unit variant writes as a bare identifier in RON.
        serializer.serialize_unit_variant("CostTag", 0, self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for CostTag {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `CounterRef`/`KeywordRef` read through.
        struct NameVisitor;
        impl<'de> serde::de::Visitor<'de> for NameVisitor {
            type Value = CostTag;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a cost-tag name (bare identifier)")
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::VariantAccess;
                let (ident, variant) = data.variant_seed(crate::IdentSeed)?;
                variant.unit_variant()?;
                Ok(CostTag(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// A declared OPTIONAL cost ([CR#118.8b] "you may pay an additional [cost] as
/// you cast this spell") — the kicker/multikicker/buyback identity, carried by
/// `StaticEffect::CostOption`. One `tag`, three read channels:
/// `Condition::PaidCost(tag)` (was it paid — kicked, [CR#702.33d]),
/// `Count::TimesPaid(tag)` (how many times — multikicker, [CR#702.33c]), and
/// `Predicate::WasPaidWith(tag)` (an object whose cost was paid with it,
/// [CR#702.33e,607.2]). `repeatable: true` is multikicker's "any number of
/// times" ([CR#702.33c]); buyback is one more tag ([CR#702.27a]). Intentions
/// are announced at [CR#601.2b]; the total locks at [CR#601.2f].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OptionalCost {
    pub components: Arc<[CostComponent]>,
    pub tag: CostTag,
    #[serde(default, skip_serializing_if = "crate::ability::is_false")]
    pub repeatable: bool,
}

/// The total-cost record of the [CR#601.2f] pipeline — an ENGINE-traffic
/// value, not card grammar: base (mana or alternative, per [CR#601.2b]) →
/// the `trace` of applied steps (`CostChange`: additional/increases, then
/// reductions, [CR#601.2f]) → mana floor → direct effects → `locked`.
/// After the lock (`LockPoint::TotalCost`) nothing changes the total
/// ([CR#601.2h]'s sacrificed-cost-reducer example); payment-stage
/// substitutions (the cost-modification hook: convoke/delve/…,
/// [CR#702.51b]) edit HOW it is paid, never the total. Paying a changed
/// cost still pays the original ([CR#118.7,118.11]).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TotalCost {
    pub base: Arc<[CostComponent]>,
    /// Applied modification steps, in application order ([CR#601.2f]).
    pub trace: Arc<[crate::CostChange]>,
    /// Set at [CR#601.2f]; a locked total never moves.
    pub locked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::ManaCost;
    use crate::mana::ManaSymbol;
    use crate::mana::SimpleManaSymbol;
    use crate::reference::Reference;

    fn read(source: &str) -> CostComponent {
        crate::ron::options().from_str(source).unwrap()
    }

    fn to_string(value: &CostComponent) -> String {
        crate::ron::options().to_string(value).unwrap()
    }

    /// The cost-level `ChooseAndPay` step keeps choosing OUT of the verb — it
    /// writes the choice to a register BEFORE the action that pays. And exile,
    /// being a pure zone move, is paid as `Do(Move(This, Exile))`
    /// (Scavenge), not a dedicated verb. Both round-trip.
    #[test]
    fn with_choice_step_and_move_are_cost_forms() {
        use crate::CharacteristicPredicate;
        use crate::CostBinder;
        use crate::Predicate;

        // "sacrifice a creature": choose one creature, then Sacrifice(You,
        // That(Creature)).
        let creature =
            Predicate::Characteristic(CharacteristicPredicate::Supertype(crate::Supertype::Basic));
        let with = CostComponent::ChooseAndPay {
            dest: crate::DefId(2),
            binder: Arc::new(CostBinder::ChooseOne {
                filter: creature,
                by: Reference::Reg(crate::RefId(1)),
            }),
            body: Cost(
                vec![CostComponent::do_action(crate::Action::Sacrifice(
                    Reference::Reg(crate::RefId(1)),
                    Reference::Reg(crate::RefId(2)),
                ))]
                .into(),
            ),
        };
        assert_eq!(read(&to_string(&with)), with, "With cost round-trips");

        // Exile-as-cost is a Move to the Exile zone — `Action::Move` is
        // agent-silent, so this spells with no agent slot.
        let exile = CostComponent::do_action(crate::Action::move_to(
            Reference::Reg(crate::RefId(0)),
            crate::Zone::Exile,
        ));
        assert_eq!(
            read(&to_string(&exile)),
            exile,
            "Do(Move(This, Exile)) round-trips"
        );
    }

    /// Read is FAITHFUL: a nested `Cost` component survives deserialization
    /// verbatim, and `.normalize()` is the explicit step that splices it into
    /// one flat list. Semantic lowering can produce
    /// `[Cost([Mana(…)]), Do(…)]`; normalization collapses it to
    /// `[Mana(…), Do(…)]` (cycling, [CR#702.29a]).
    #[test]
    fn nested_cost_survives_read_and_normalizes_flat() {
        // The named Cost variant wraps a sub-list.
        let comp: CostComponent = crate::ron::options().from_str("Cost([Tap])").unwrap();
        assert!(
            matches!(comp, CostComponent::Cost(_)),
            "Cost([…]) reads as the Cost variant, got {comp:?}"
        );

        // Plain serde preserves the nested Cost verbatim.
        let mana_two = CostComponent::Mana(ManaCost::from(Arc::<[ManaSymbol]>::from(vec![
            ManaSymbol::Simple(SimpleManaSymbol::Generic(2)),
        ])));
        let discard_self =
            CostComponent::do_action(crate::Action::discard_what(Reference::Reg(crate::RefId(0))));
        let lumpy = Cost(
            vec![
                CostComponent::Cost(Cost(vec![mana_two.clone()].into())),
                discard_self.clone(),
            ]
            .into(),
        );
        assert_eq!(
            lumpy,
            Cost(
                vec![
                    CostComponent::Cost(Cost(vec![mana_two.clone()].into())),
                    discard_self.clone(),
                ]
                .into()
            ),
            "the nested cost shape is preserved",
        );

        // `.normalize()` splices the nested Cost into one flat list.
        assert_eq!(
            lumpy.normalize(),
            Cost(vec![mana_two, discard_self].into()),
            "normalize collapses the nested Cost",
        );
    }

    /// Normalization is one-pass over arbitrary nesting depth (the inner list
    /// is normalized before its parent splices it) and idempotent.
    #[test]
    fn normalize_flattens_deeply_and_is_idempotent() {
        let deep: Cost = crate::ron::options()
            .from_str("[Cost([Cost([Tap]), Untap])]")
            .unwrap();
        let flat = deep.normalize();
        assert_eq!(
            flat,
            Cost(vec![CostComponent::Tap, CostComponent::Untap].into())
        );
        assert_eq!(flat.clone().normalize(), flat, "normalize is idempotent");
    }

    #[test]
    fn cost_components_parse() {
        assert_eq!(
            read("Mana([Simple(Generic(2))])"),
            CostComponent::Mana(ManaCost::from(Arc::<[ManaSymbol]>::from(vec![
                ManaSymbol::Simple(SimpleManaSymbol::Generic(2)),
            ]))),
        );
        assert_eq!(read("Tap"), CostComponent::Tap);
        assert_eq!(
            read("Act(Sacrifice(Reg(1), Reg(0)))"),
            CostComponent::do_action(crate::Action::Sacrifice(
                Reference::Reg(crate::RefId(1)),
                Reference::Reg(crate::RefId(0))
            )),
        );
    }

    #[test]
    fn runnable_action_rejects_a_random_subject_hidden_in_a_reference() {
        let random = Reference::Single(Arc::new(crate::Selection::Random(
            crate::Quantity::one(),
            crate::Predicate::Any,
        )));
        assert_eq!(
            CostComponent::try_do_action(crate::Action::Sacrifice(
                Reference::Reg(crate::RefId(1)),
                random
            )),
            Err(RunnableCostActionError::UnresolvedSubject),
        );
    }

    #[test]
    fn runnable_action_rejects_random_references_in_counts_riders_and_discard_bodies() {
        let random = || {
            Reference::Single(Arc::new(crate::Selection::Random(
                crate::Quantity::one(),
                crate::Predicate::Any,
            )))
        };
        let random_count = || Count::StatOf(random(), crate::Stat::Power);
        for action in [
            crate::Action::ChangeLife(
                Reference::Reg(crate::RefId(1)),
                crate::LifeOp::Down(random_count()),
            ),
            crate::Action::PutCounters(
                Reference::Reg(crate::RefId(0)),
                crate::CounterRef::from("Charge"),
                random_count(),
            ),
            crate::Action::Move(
                Reference::Reg(crate::RefId(0)),
                crate::Destination::Library(crate::Anchor::FromTop(random_count())),
                vec![crate::EnterRider::WithCounters(
                    crate::CounterRef::from("Charge"),
                    random_count(),
                )]
                .into(),
                None,
            ),
        ] {
            assert_eq!(
                CostComponent::try_do_action(action),
                Err(RunnableCostActionError::UnresolvedSubject),
            );
        }

        let spoofed_discard = crate::Action::Composite {
            name: crate::VerbName::from("Discard"),
            body: Arc::new(crate::OneShotEffect::act(crate::Action::DealDamage(
                random(),
                Count::Literal(1),
                Reference::Reg(crate::RefId(1)),
            ))),
        };
        assert_eq!(
            CostComponent::try_do_action(spoofed_discard),
            Err(RunnableCostActionError::UnresolvedSubject),
        );
    }

    #[test]
    fn runnable_cost_binder_admits_supported_deterministic_and_exact_random_shapes() {
        let selected_legend = Reference::Single(Arc::new(crate::Selection::SelectAll(Arc::new(
            crate::Region::new(
                Arc::from([]),
                Predicate::And(
                    vec![
                        Predicate::Characteristic(crate::CharacteristicPredicate::Supertype(
                            crate::Supertype::Legendary,
                        )),
                        Predicate::Relation(crate::RelationPredicate::ControlledBy(Arc::new(
                            Predicate::Ref(Reference::Reg(crate::RefId(1))),
                        ))),
                    ]
                    .into(),
                ),
            ),
        ))));
        assert!(cost_binder_is_runnable(&CostBinder::TheRef(
            selected_legend
        )));
        assert!(cost_binder_is_runnable(&CostBinder::Existing(
            crate::Selection::Random(crate::Quantity::one(), Predicate::Any),
        )));
        assert!(cost_binder_is_runnable(&CostBinder::Produce(Arc::new(
            crate::Action::Move(
                Reference::Reg(crate::RefId(0)),
                crate::Destination::Zone(crate::Zone::Exile),
                Arc::from([]),
                None,
            ),
        ))));
    }

    #[test]
    fn runnable_cost_binder_rejects_nested_random_and_unchecked_producers() {
        let random_reference = || {
            Reference::Single(Arc::new(crate::Selection::Random(
                crate::Quantity::one(),
                Predicate::Any,
            )))
        };
        assert!(!cost_binder_is_runnable(&CostBinder::TheRef(
            Reference::ControllerOf(Arc::new(random_reference())),
        )));
        assert!(!cost_binder_is_runnable(&CostBinder::Existing(
            crate::Selection::Union(vec![
                crate::Selection::SelectAll(Arc::new(crate::Region::new(
                    Arc::from([]),
                    Predicate::Any
                ))),
                crate::Selection::Random(crate::Quantity::one(), Predicate::Any),
            ]),
        )));

        let random_count = Count::StatOf(random_reference(), crate::Stat::Power);
        assert!(!cost_binder_is_runnable(&CostBinder::Existing(
            crate::Selection::Random(
                crate::Quantity::Range(Some(random_count.clone()), Some(random_count)),
                Predicate::Any,
            ),
        )));
        assert!(!cost_binder_is_runnable(&CostBinder::Produce(Arc::new(
            crate::Action::Move(
                random_reference(),
                crate::Destination::Zone(crate::Zone::Exile),
                Arc::from([]),
                None,
            ),
        ))));
        assert!(!cost_binder_is_runnable(&CostBinder::Produce(Arc::new(
            crate::Action::Move(
                Reference::Single(Arc::new(crate::Selection::SelectAll(Arc::new(
                    crate::Region::new(Arc::from([]), Predicate::Any),
                )))),
                crate::Destination::Zone(crate::Zone::Exile),
                Arc::from([]),
                None,
            ),
        ))));
        assert!(!cost_binder_is_runnable(&CostBinder::Produce(Arc::new(
            crate::Action::DrawCard(Reference::Reg(crate::RefId(1))),
        ))));

        let unchecked_whiff = crate::OneShotEffect::act(crate::Action::Move(
            random_reference(),
            crate::Destination::Zone(crate::Zone::Exile),
            Arc::from([]),
            None,
        ));
        assert!(!cost_binder_is_runnable(&CostBinder::SearchOne {
            filter: Predicate::Any,
            by: Reference::Reg(crate::RefId(1)),
            whose: Reference::Reg(crate::RefId(1)),
            from: Arc::from([crate::Zone::Library]),
            if_none: Some(Arc::new(unchecked_whiff)),
        }));
    }

    /// `ManaCostOf(Reference)` reads and round-trips through the
    /// `serde::Deserialize, serde::Serialize` serde (a referenced object's mana cost,
    /// [CR#202.1]).
    #[test]
    fn mana_cost_of_round_trips() {
        assert_eq!(
            read("ManaCostOf(Reg(0))"),
            CostComponent::ManaCostOf(Reference::Reg(crate::RefId(0))),
        );
        let v = CostComponent::ManaCostOf(Reference::ControllerOf(Arc::new(Reference::Reg(
            crate::RefId(0),
        ))));
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&written), v, "round-trips: {written}");
    }

    /// `TapTotal(stat, cmp, count, filter)` reads flat and round-trips — the
    /// aggregate-stat (Crew) cost shape ([CR#702.122a]). The `filter` field is
    /// a boxed open [`Predicate`], so a bare `Type(Creature)` reads into it
    /// transparently.
    #[test]
    fn tap_total_reads_and_round_trips() {
        use crate::Cmp;
        use crate::Count;
        use crate::Predicate;
        use crate::Stat;

        // The Crew shape: "tap any number of creatures with total power 3 or
        // greater" ([CR#702.122a]).
        let crew = CostComponent::TapTotal {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: Count::Literal(3),
            filter: Arc::new(Predicate::Characteristic(
                crate::CharacteristicPredicate::Supertype(crate::Supertype::Basic),
            )),
        };
        assert_eq!(
            read(
                "TapTotal(stat: Power, cmp: AtLeast, count: Literal(3), filter: Characteristic(Supertype(Basic)))"
            ),
            crew,
        );

        // Serialize -> read is identity (the highest-risk arm — a 4-field tuple
        // variant with a boxed filter and a bare-literal count).
        let written = crate::ron::options().to_string(&crew).unwrap();
        assert_eq!(read(&written), crew, "round-trips: {written}");
    }

    /// `OptionalCost` — the kicker/multikicker/buyback identity
    /// ([CR#118.8b,702.33a]) — reads flat with a bare-ident tag, omits the
    /// default `repeatable: false`, and round-trips; `repeatable: true` is
    /// multikicker ([CR#702.33c]).
    #[test]
    fn optional_cost_round_trips() {
        let kicker: crate::OptionalCost = crate::ron::options()
            .from_str("(components: [Mana([Simple(Generic(2))])], tag: Kicker)")
            .unwrap();
        assert_eq!(kicker.tag, crate::CostTag::from("Kicker"));
        assert!(!kicker.repeatable, "repeatable defaults false");
        let written = crate::ron::options().to_string(&kicker).unwrap();
        assert!(
            !written.contains("repeatable"),
            "default repeatable omitted: {written}"
        );
        assert!(
            written.contains("tag:Kicker"),
            "tag writes as a bare ident: {written}"
        );
        let reread: crate::OptionalCost = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reread, kicker);

        let multi: crate::OptionalCost = crate::ron::options()
            .from_str(
                "(components: [Mana([Simple(Generic(1))])], tag: Multikicker, repeatable: true)",
            )
            .unwrap();
        assert!(multi.repeatable);
        let written = crate::ron::options().to_string(&multi).unwrap();
        let reread: crate::OptionalCost = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reread, multi);
    }

    #[test]
    fn cost_list_round_trips() {
        // `Sacrifice` carries its agent slot explicitly now ([CR#701.21a]
        // "its controller"); in a cost, `You` is the payer.
        let source = "[Mana([Simple(Generic(2))]),Tap,Act(Sacrifice(Reg(1), Reg(0)))]";
        let parsed: Arc<[CostComponent]> = crate::ron::options().from_str(source).unwrap();
        let written = crate::ron::options().to_string(&parsed).unwrap();
        let reparsed: Arc<[CostComponent]> = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
