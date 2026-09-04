use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Count;
use crate::EventFilter;
use crate::KeywordAbility;
use crate::Region;
use crate::TargetSpec;
use crate::Timing;
use crate::continuous::StaticSpec;
use crate::cost::Cost;

/// A spell ability — what an instant or sorcery does on resolution
/// ([CR#113.3a]). Targeting, when present, lives in `targets`; the effect reads
/// each announced slot through its region parameter ([CR#115.1,601.2c]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    /// The ability word printed before the em dash ([CR#207.2c] — no rules
    /// meaning), pure render metadata: "Domain — …". NEVER a macro tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_word: Option<crate::Ident>,
    /// The spell's PRINTED ADDITIONAL cost ([CR#118.8,601.2b]) — "as an
    /// additional cost to cast this spell, sacrifice a creature" (Fling). A
    /// block of cost instructions announced and paid with the mana cost
    /// ([CR#118.8a,601.2h]), running in this ability's own activation, so its
    /// paid product is a def `effect` reads. Empty (and omitted on write) for
    /// the ordinary spell with no additional cost. An `ActivatedAbility` folds
    /// the same block into its activation cost; a `TriggeredAbility` has none
    /// — a trigger pays no mana cost and no activation cost, so [CR#118.8]
    /// gives an additional cost nowhere to be paid.
    #[serde(default, skip_serializing_if = "Cost::is_empty")]
    pub cost: Cost,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub targets: Arc<[TargetSpec]>,
    pub effect: Region,
}

/// An activated ability: paid with a cost and produces an effect
/// ([CR#113.3b,602]). Targeting lives in `targets` ([CR#115.1,601.2c]); the
/// `Resolvable` wrapper of the design sketch is realized as
/// `Instruction::Modal` (see `effect`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivatedAbility {
    /// The ability word printed before the em dash ([CR#207.2c] — no rules
    /// meaning), pure render metadata. NEVER a macro tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_word: Option<crate::Ident>,
    pub cost: Cost,
    /// The zone the ability functions from ([CR#113.6] — an object's abilities
    /// usually function only while it is on the battlefield). `None` = that
    /// battlefield default (omitted on write); a `Some` names another zone the
    /// source must be in to activate — cycling functions from hand
    /// ([CR#702.29a]), so `from: Hand`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::Zone>,
    /// "Activate only [timing]" ([CR#602.5d..602.5e]) — an `Only` window
    /// refinement on the activation permission (deontics §3), e.g.
    /// `window: SorcerySpeed`. Distinct from `condition`, which gates on
    /// game STATE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<Timing>,
    /// "Activate only if [state]" ([CR#602.5b..602.5e]) — a predicate over
    /// the game state at activation time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// "Activate only once each turn." ([CR#602.5b]).
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub limits: Arc<[UseLimit]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub targets: Arc<[TargetSpec]>,
    pub effect: Region,
}

/// A limit on how often an ability may be used — a triggered ability
/// triggering ([CR#603.2h]) or an activated ability being activated
/// ([CR#602.5b]). Per object: a reminted object (zone change) starts fresh;
/// a controller change does not reset it ([CR#602.5b]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum UseLimit {
    /// "only once each turn" ([CR#603.2h] / [CR#602.5b]).
    OncePerTurn,
    /// "Activate only once." ([CR#702.177a], exhaust) — once per game.
    OncePerGame,
    /// No loyalty ability of this permanent has been activated this turn
    /// ([CR#606.3]). Unlike `OncePerTurn` (per-ability), this is SHARED
    /// across every loyalty ability a planeswalker carries ([CR#306.5d]):
    /// activating any one of them blocks the rest for the turn. Pairs with
    /// `window: SorcerySpeed` on the same `ActivatedAbility` — [CR#606.3]'s
    /// "any time they have priority and the stack is empty during a main
    /// phase of their turn" is that window.
    LoyaltyOncePerTurn,
}

/// A triggered ability ([CR#113.3c,603]). A named struct because it recurs:
/// delayed ([CR#603.7]) and reflexive ([CR#603.12]) triggers are the same
/// value, created inside an `Instruction`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TriggeredAbility {
    /// The ability word printed before the em dash ([CR#207.2c] — no rules
    /// meaning), pure render metadata: "Landfall — Whenever …". NEVER a
    /// macro tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_word: Option<crate::Ident>,
    /// The event that triggers it ([CR#603.2]).
    pub event: EventFilter,
    /// The zone the ability functions from ([CR#113.6,113.6b]). `None` = the
    /// battlefield default (omitted on write); a `Some` names another zone the
    /// source must be in for the ability to trigger — a graveyard/hand trigger
    /// (Madness, "while this is in your graveyard, …") sets `from: Graveyard` /
    /// `from: Hand`. Mirrors [`ActivatedAbility::from`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::Zone>,
    /// Intervening-if ([CR#603.4]) — `condition`, not `when_if`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// Trigger-frequency limits ([CR#603.2h]).
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub limits: Arc<[UseLimit]>,
    /// The "where X is …" definition of this ability's `{X}` ([CR#702.21b] —
    /// a ward-{X} toll's X "is determined at the time the ability resolves,
    /// not locked in as the ability triggers"). Evaluated ONCE at region
    /// entry into the region's announced-X parameter, so every `{X}` in the
    /// ability's costs and body is one declared register read
    /// (`Provenance::AnnouncedX`) rather than a search of the register file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub where_x: Option<Count>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub targets: Arc<[TargetSpec]>,
    pub effect: Region,
}

/// A `skip_serializing_if` predicate: a `false` bool is omitted from RON.
/// serde requires the predicate to take `&T`, hence the by-ref bool.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde requires the skip_serializing_if predicate to take &T"
)]
pub(crate) fn is_false(b: &bool) -> bool {
    !*b
}

/// How a modal spell or ability's modes are chosen ([CR#700.2]) — `Modal`'s
/// decider (spec §7 "Modal gains who"; [CR#700.2e], Fatal Lore "An opponent
/// chooses one —") and payload rows. `count` is a [`Quantity`](crate::Quantity)
/// — "choose one" = `Exactly(1)`, escalate's printed "one or more" =
/// `AtLeast(1)`. `up_to` is the "up to N" form ([CR#700.2]); `repeats` allows
/// choosing the same mode more than once ([CR#700.2d]); `chooser` names who
/// chooses — always spelled (Law 2: no read-time default on a decider slot).
/// Modes and optional-cost intentions are announced at mode choice
/// ([CR#601.2b]); per-mode targets are chosen only for chosen modes
/// ([CR#601.2c]); the total cost locks at [CR#601.2f].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChooseSpec {
    pub count: crate::Quantity,
    #[serde(default, skip_serializing_if = "is_false")]
    pub up_to: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub repeats: bool,
    pub chooser: crate::Reference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rider: Option<ModalCostRider>,
}

/// A cost rider on a modal choose spec — the entwine/escalate family
/// ([CR#702.42a,702.120a]), announced with the mode choice ([CR#601.2b]) and
/// locked into the total at [CR#601.2f].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ModalCostRider {
    /// `Entwine(c)`: the printed count stays `Exactly(1)`; the caster may
    /// instead choose ALL modes by adding `c` ([CR#702.42a]; the modes then
    /// resolve in written order, [CR#702.42b]).
    Entwine(Cost),
    /// `Escalate(c)`: the printed count is the "one or more" quantity; the
    /// total cost gains `c` for each mode chosen beyond the first
    /// ([CR#702.120a]).
    Escalate(Cost),
}

/// One mode of a modal spell or ability ([CR#700.2]). A mode owns its target
/// telescope and closed effect region ([CR#700.2c,115.8]); it may carry a
/// per-mode cost ([CR#700.2h]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Mode {
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub targets: Arc<[TargetSpec]>,
    pub effect: Region,
    /// The mode's own cost ([CR#700.2h]) — announced with the mode choice and
    /// folded into the total at [CR#601.2f]. A cost block like any other: its
    /// products define into this mode's region ahead of `effect`.
    #[serde(default, skip_serializing_if = "Cost::is_empty")]
    pub cost: Cost,
}

/// One modal branch's mana-ability facts ([CR#605.1a]): whether the branch
/// could add mana as it resolves, and whether it is targetless. Derived from
/// the branch's compiled shape, never authored and never read from live game
/// state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManaModeClass {
    pub adds_mana: bool,
    pub targetless: bool,
}

/// How an activated mana ability's classification depends on its announced
/// modes. `Always` qualifies whatever announcement does; `ByAnnouncedMode`
/// carries one derived row per printed mode ([CR#700.2]), so the announced
/// selection decides.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActivatedManaProfile {
    Always,
    ByAnnouncedMode(Arc<[ManaModeClass]>),
}

/// All four struct-carrying variants (`Static`/`Activated`/`Triggered`/`Spell`)
/// are boxed. This drops `Vec<Ability>`
/// (`CardFace::abilities`) elements from ~1416 B to ~88 B and clears
/// `large_enum_variant` without a suppression. Build them via the boxing
/// constructors ([`Ability::triggered`] / `activated` / `spell` / `r#static`)
/// rather than `Ability::Triggered(Arc::new(…))`. See
/// `engine-event-size-boxing`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Ability {
    /// A static ability ([CR#113.3d,604]) — a single [`StaticSpec`], read
    /// POSITIONALLY bare (`Static(Each(SelectAll(...), Modify(It, ...)))`,
    /// `Static(Cant(...))`). Duration is implicit ("while it functions",
    /// [CR#611.3]); conditionality/other qualifiers compose as `StaticSpec`
    /// wrappers ([`Conditionally`](crate::StaticSpec::Conditionally)), never
    /// a struct field. Mirrors Idris `Static : StaticSpec -> Ability`.
    Static(Arc<Region<StaticSpec>>),
    Activated(Arc<ActivatedAbility>),
    Triggered(Arc<TriggeredAbility>),
    Spell(Arc<SpellAbility>),
    /// A keyword ability ([CR#702]) — always spelled `Keyword(…)` on cards.
    /// The five primitive variants are represented directly; semantic lowering
    /// resolves every other authored keyword to [`KeywordAbility::Composite`].
    Keyword(KeywordAbility),
}

impl Ability {
    /// Build [`Ability::Static`], boxing the [`StaticSpec`] payload — the
    /// value-side counterpart of the flat RON `Static(…)`, so call sites never
    /// hand-write `Arc::new`. (Raw ident: `static` is a keyword.)
    #[must_use]
    pub fn r#static(effect: StaticSpec) -> Self {
        Ability::Static(Arc::new(Region::new(crate::event_region_params(), effect)))
    }

    /// Build [`Ability::Activated`], boxing the payload.
    #[must_use]
    pub fn activated(mut ability: ActivatedAbility) -> Self {
        if ability.effect.params.is_empty() {
            ability.effect.params = crate::announced_region_params(ability.targets.len());
        }
        Ability::Activated(Arc::new(ability))
    }

    /// Build [`Ability::Triggered`], boxing the payload.
    ///
    /// # Panics
    ///
    /// Panics if the ability contains more than `u32::MAX` targets or inferred
    /// region parameters.
    #[must_use]
    pub fn triggered(mut ability: TriggeredAbility) -> Self {
        if ability.effect.params.is_empty() {
            ability.effect.params = crate::triggered_region_params(ability.targets.len());
        }
        Ability::Triggered(Arc::new(ability))
    }

    /// View any activated payload, looking through the provenance wrapper.
    #[must_use]
    pub fn as_activated(&self) -> Option<&ActivatedAbility> {
        match self {
            Ability::Activated(ability) => Some(ability),
            _ => None,
        }
    }

    /// View any triggered payload, looking through the provenance wrapper.
    #[must_use]
    pub fn as_triggered(&self) -> Option<&TriggeredAbility> {
        match self {
            Ability::Triggered(ability) => Some(ability),
            _ => None,
        }
    }

    /// This ability's activated mana profile, if it is an activated mana
    /// ability ([CR#605.1a]): targetless, could add mana to a mana pool as it
    /// resolves, and not a loyalty ability ([CR#606]). Mana is an ORTHOGONAL
    /// classification of some activated and triggered abilities ([CR#113.4]),
    /// not a fifth category beside the four of [CR#113.3], so it is derived
    /// from the compiled ability rather than carried as a sibling variant.
    /// [CR#605.2]: derivation reads structure only, so an ability the board
    /// currently stops from producing mana is still a mana ability.
    ///
    /// A single `Modal` body defers the judgment to the announced modes and
    /// yields one derived row per printed mode. The library-movement clause of
    /// [CR#605.1a] is deliberately not evaluated (a runtime reversal barrier
    /// does not reclassify the ability); see the classification tests in
    /// `deckmaste_lowering`.
    #[must_use]
    pub fn mana_profile(&self) -> Option<ActivatedManaProfile> {
        let Ability::Activated(ability) = self else {
            return None;
        };
        if ability.limits.contains(&UseLimit::LoyaltyOncePerTurn) {
            return None;
        }
        if let [crate::Instruction::Modal(modal)] = ability.effect.body.as_ref() {
            let classes: Arc<[ManaModeClass]> = modal
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
                return Some(ActivatedManaProfile::ByAnnouncedMode(classes));
            }
            return None;
        }
        let mut facts = region_mana_facts(&ability.effect);
        facts.targetless &= ability.targets.is_empty();
        (facts.adds_mana && facts.targetless).then_some(ActivatedManaProfile::Always)
    }

    /// Whether this is an activated mana ability ([CR#605.1a]).
    #[must_use]
    pub fn is_activated_mana_ability(&self) -> bool {
        self.mana_profile().is_some()
    }

    /// Whether this is a triggered mana ability ([CR#605.1b]): targetless,
    /// triggered by an activated mana ability or by mana being added, and
    /// could add mana as it resolves.
    #[must_use]
    pub fn is_triggered_mana_ability(&self) -> bool {
        let Ability::Triggered(ability) = self else {
            return false;
        };
        let mut facts = region_mana_facts(&ability.effect);
        facts.targetless &= ability.targets.is_empty();
        facts.adds_mana && facts.targetless && triggered_by_mana(&ability.event)
    }

    /// Whether this ability is a mana ability at all ([CR#605.1]).
    #[must_use]
    pub fn is_mana_ability(&self) -> bool {
        self.is_activated_mana_ability() || self.is_triggered_mana_ability()
    }

    /// Instantiate an activated mana profile from announcement-time mode
    /// indices. Invalid, empty, targeted, or non-producing selections do not
    /// qualify.
    #[must_use]
    pub fn mana_profile_for_modes(&self, modes: &[crate::Uint]) -> bool {
        let Some(profile) = self.mana_profile() else {
            return false;
        };
        match profile {
            ActivatedManaProfile::Always => true,
            ActivatedManaProfile::ByAnnouncedMode(classes) => {
                let selected: Option<Vec<ManaModeClass>> = modes
                    .iter()
                    .map(|&mode| {
                        usize::try_from(mode)
                            .ok()
                            .and_then(|i| classes.get(i).copied())
                    })
                    .collect();
                selected.is_some_and(|selected| {
                    !selected.is_empty()
                        && selected.iter().all(|class| class.targetless)
                        && selected.iter().any(|class| class.adds_mana)
                })
            }
        }
    }

    /// Build [`Ability::Spell`], boxing the payload.
    #[must_use]
    pub fn spell(mut ability: SpellAbility) -> Self {
        if ability.effect.params.is_empty() {
            ability.effect.params = crate::announced_region_params(ability.targets.len());
        }
        Ability::Spell(Arc::new(ability))
    }
}

/// The two [CR#605.1a] structural facts, folded over an instruction tree.
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

    fn mode_class(self) -> ManaModeClass {
        ManaModeClass {
            adds_mana: self.adds_mana,
            targetless: self.targetless,
        }
    }
}

/// [CR#605.1b]: the trigger condition is the activation or resolution of an
/// activated mana ability, or mana being added to a mana pool.
fn triggered_by_mana(event: &crate::EventFilter) -> bool {
    use crate::EventFilter;
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

fn effect_mana_facts(effect: &crate::Instruction) -> ManaFacts {
    use crate::Instruction;
    match effect {
        Instruction::Act { action, .. } => effect_action_facts(action),
        Instruction::Sequentially(parts) | Instruction::Simultaneously(parts) => {
            parts.iter().fold(ManaFacts::NEUTRAL, |facts, part| {
                facts.merge(effect_mana_facts(part))
            })
        }
        // RevealUntil is intentionally inert at runtime; none of these nodes
        // can establish that the executable ability produces mana.
        Instruction::Choose(_)
        | Instruction::ChooseValue(_)
        | Instruction::Search(_)
        | Instruction::Let(_)
        | Instruction::Remember(_)
        | Instruction::Continuously(_)
        | Instruction::Until(_, _)
        | Instruction::Delayed(_)
        | Instruction::Reflexive(_)
        | Instruction::RevealUntil(_) => ManaFacts::NEUTRAL,
        Instruction::SeparatePiles(piles) => piles
            .then
            .as_deref()
            .map_or(ManaFacts::NEUTRAL, effect_mana_facts),
        Instruction::ChoosePile(pile) => effect_mana_facts(&pile.then),
        Instruction::May(may) => [
            Some(may.effect.as_ref()),
            may.if_did.as_deref(),
            may.if_not.as_deref(),
        ]
        .into_iter()
        .flatten()
        .fold(ManaFacts::NEUTRAL, |facts, part| {
            facts.merge(effect_mana_facts(part))
        }),
        Instruction::If(branch) => [Some(branch.then.as_ref()), branch.otherwise.as_deref()]
            .into_iter()
            .flatten()
            .fold(ManaFacts::NEUTRAL, |facts, part| {
                facts.merge(effect_mana_facts(part))
            }),
        Instruction::Each(each) => region_mana_facts(&each.body),
        Instruction::Distribute(distribute) => region_mana_facts(&distribute.body),
        Instruction::Modal(modal) => modal.modes.iter().fold(ManaFacts::NEUTRAL, |facts, mode| {
            facts.merge(region_mana_facts(&mode.effect))
        }),
        Instruction::Repeat(_, body) | Instruction::Batch(_, body) => effect_mana_facts(body),
    }
}

fn region_mana_facts(region: &Region) -> ManaFacts {
    region
        .body
        .iter()
        .fold(ManaFacts::NEUTRAL, |facts, instruction| {
            facts.merge(effect_mana_facts(instruction))
        })
}

fn effect_action_facts(action: &crate::Action) -> ManaFacts {
    use crate::Action;
    match action {
        Action::AddMana(_, _, _) => ManaFacts {
            adds_mana: true,
            ..ManaFacts::NEUTRAL
        },
        Action::Composite { body, .. } => effect_mana_facts(body),
        _ => ManaFacts::NEUTRAL,
    }
}
