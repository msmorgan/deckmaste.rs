use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::EventFilter;
use crate::KeywordAbility;
use crate::Timing;
use crate::continuous::StaticEffect;
use crate::cost::Cost;
use crate::cost::CostComponent;
use crate::effect::OneShotEffect;

/// A spell ability — what an instant or sorcery does on resolution
/// ([CR#113.3a]). Targeting, when present, lives on an
/// `OneShotEffect::Targeted` wrapper in `effect` ([CR#115.1,601.2c]), read back
/// by the anaphors (`It`/`That(Sort)`/`They`, or `Target(n)` for the nth
/// announced slot).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    /// The ability word printed before the em dash ([CR#207.2c] — no rules
    /// meaning), pure render metadata: "Domain — …". NEVER a macro tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_word: Option<crate::Ident>,
    pub effect: OneShotEffect,
}

/// An activated ability: paid with a cost and produces an effect
/// ([CR#113.3b,602]). Targeting lives on an `OneShotEffect::Targeted` wrapper
/// in `effect` ([CR#115.1,601.2c]); the `Resolvable` wrapper of the design
/// sketch is realized as `OneShotEffect::Modal` (see `effect`).
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
    pub effect: OneShotEffect,
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
/// value, created inside an `OneShotEffect`.
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
    /// The "where X is …" definition of an {X} in this ability's text —
    /// ability metadata that survives to render and defines the X a ward
    /// toll prices. Evaluated when the ability RESOLVES, never locked in as
    /// it triggers ([CR#702.21b] — Minthara's "ward {X}, where X is the
    /// number of experience counters you have").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub where_x: Option<crate::Count>,
    pub effect: OneShotEffect,
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

/// One mode of a modal spell or ability ([CR#700.2]). A mode's targets live on
/// an `OneShotEffect::Targeted` wrapper in its `effect` ([CR#700.2c,115.8]); it
/// may carry a per-mode cost ([CR#700.2h]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Mode {
    pub effect: OneShotEffect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<Arc<[CostComponent]>>,
}

/// One modal branch's lowering-time mana-ability facts. Runtime mode
/// announcement selects among these precomputed rows; live game state and
/// external replacement effects never reclassify the ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ManaModeClass {
    pub adds_mana: bool,
    pub targetless: bool,
}

/// How an activated mana ability's lowering-time classification depends on
/// its announced modes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ActivatedManaProfile {
    Always,
    ByAnnouncedMode(Arc<[ManaModeClass]>),
}

/// A mana ability carries the same activated or triggered payload as its
/// ordinary peer plus lowering's explicit classification judgment.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ManaAbility {
    Activated {
        ability: Arc<ActivatedAbility>,
        profile: ActivatedManaProfile,
    },
    Triggered(Arc<TriggeredAbility>),
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
    /// A static ability ([CR#113.3d,604]) — a single [`StaticEffect`], read
    /// POSITIONALLY bare (`Static(Each(SelectAll(...), Modify(It, ...)))`,
    /// `Static(Cant(...))`). Duration is implicit ("while it functions",
    /// [CR#611.3]); conditionality/other qualifiers compose as `StaticEffect`
    /// wrappers ([`Conditionally`](crate::StaticEffect::Conditionally)), never
    /// a struct field. Mirrors Idris `Static : StaticEffect -> Ability`.
    Static(Arc<StaticEffect>),
    Activated(Arc<ActivatedAbility>),
    Triggered(Arc<TriggeredAbility>),
    /// A lowering-classified activated or triggered mana ability.
    Mana(ManaAbility),
    Spell(Arc<SpellAbility>),
    /// A keyword ability ([CR#702]) — always spelled `Keyword(…)` on cards.
    /// The five intrinsic variants are represented directly; semantic lowering
    /// resolves every other authored keyword to [`KeywordAbility::Composite`].
    Keyword(KeywordAbility),
    /// A conferred ability that is a *rule of the object* rather than a card
    /// ability — used sparingly for "this type always behaves like this"
    /// invariants (the Aura [CR#704.5m] graveyard SBA, the Equipment
    /// [CR#301.5] / Fortification [CR#301.6] host restriction). The wrapped
    /// ability is (a) **immune to layer-6 ability removal**: `LoseAllAbilities`
    /// retains it, `LoseAbility` skips it, a `CantHaveAbility` set never
    /// suppresses it; and (b) **invisible to card-facing ability queries**
    /// — an object whose only abilities are `Innate` reads as having no
    /// abilities to other cards ([CR#113.12]). Engine machinery (the SBA
    /// sweep, `attachment_legal`, layer static-application) peels `Innate` to
    /// see the inner ability.
    Innate(Arc<Ability>),
}

impl Ability {
    /// Build [`Ability::Static`], boxing the [`StaticEffect`] payload — the
    /// value-side counterpart of the flat RON `Static(…)`, so call sites never
    /// hand-write `Arc::new`. (Raw ident: `static` is a keyword.)
    #[must_use]
    pub fn r#static(effect: StaticEffect) -> Self {
        Ability::Static(Arc::new(effect))
    }

    /// Build [`Ability::Activated`], boxing the payload.
    #[must_use]
    pub fn activated(ability: ActivatedAbility) -> Self {
        Ability::Activated(Arc::new(ability))
    }

    /// Build [`Ability::Triggered`], boxing the payload.
    #[must_use]
    pub fn triggered(ability: TriggeredAbility) -> Self {
        Ability::Triggered(Arc::new(ability))
    }

    /// View any activated payload, looking through runtime and provenance
    /// wrappers.
    #[must_use]
    pub fn as_activated(&self) -> Option<&ActivatedAbility> {
        match self {
            Ability::Activated(ability) | Ability::Mana(ManaAbility::Activated { ability, .. }) => {
                Some(ability)
            }
            Ability::Innate(inner) => inner.as_activated(),
            _ => None,
        }
    }

    /// View any triggered payload, looking through runtime and provenance
    /// wrappers.
    #[must_use]
    pub fn as_triggered(&self) -> Option<&TriggeredAbility> {
        match self {
            Ability::Triggered(ability) | Ability::Mana(ManaAbility::Triggered(ability)) => {
                Some(ability)
            }
            Ability::Innate(inner) => inner.as_triggered(),
            _ => None,
        }
    }

    /// The compiled mana wrapper, if this is one, looking through wrappers.
    #[must_use]
    pub fn as_mana(&self) -> Option<&ManaAbility> {
        match self {
            Ability::Mana(mana) => Some(mana),
            Ability::Innate(inner) => inner.as_mana(),
            _ => None,
        }
    }

    /// Instantiate an activated mana profile from announcement-time mode
    /// indices. Invalid, empty, targeted, or non-producing selections do not
    /// qualify.
    #[must_use]
    pub fn mana_profile_for_modes(&self, modes: &[crate::Uint]) -> bool {
        let Some(ManaAbility::Activated { profile, .. }) = self.as_mana() else {
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
    pub fn spell(ability: SpellAbility) -> Self {
        Ability::Spell(Arc::new(ability))
    }

    /// Peel any `Innate` wrapper to the inner ability — the view engine
    /// machinery (SBA sweep, `attachment_legal`, layer static-application)
    /// uses, since `Innate` is consumed normally there ([CR#604.1] statics
    /// still function). Non-`Innate` abilities pass through unchanged.
    #[must_use]
    pub fn peel_innate(&self) -> &Ability {
        match self {
            Ability::Innate(inner) => inner.peel_innate(),
            other => other,
        }
    }

    /// Whether this ability is `Innate` ([CR#113.12]) — used to RETAIN it
    /// through layer-6 ability removal and to FILTER it out of card-facing
    /// ability queries.
    #[must_use]
    pub fn is_innate(&self) -> bool {
        matches!(self, Ability::Innate(_))
    }
}
