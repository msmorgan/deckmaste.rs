use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::EventFilter;
use crate::Expand;
use crate::Expansion;
use crate::KeywordAbility;
use crate::SupportsMacros;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Mode {
    pub effect: OneShotEffect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<Arc<[CostComponent]>>,
}

/// One modal branch's lowering-time mana-ability facts. Runtime mode
/// announcement selects among these precomputed rows; live game state and
/// external replacement effects never reclassify the ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ManaModeClass {
    pub adds_mana: bool,
    pub targetless: bool,
}

/// How an activated mana ability's lowering-time classification depends on
/// its announced modes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ActivatedManaProfile {
    Always,
    ByAnnouncedMode(Arc<[ManaModeClass]>),
}

/// A mana ability carries the same activated or triggered payload as its
/// ordinary peer plus lowering's explicit classification judgment.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaAbility {
    Activated {
        ability: Arc<ActivatedAbility>,
        profile: ActivatedManaProfile,
    },
    Triggered(Arc<TriggeredAbility>),
}

/// An ability ([CR#113]). The struct-carrying variants read flat in RON —
/// `Activated(cost: ..., ...)`, not `Activated((cost: ...))` — via the
/// `unwrap_variant_newtypes` extension.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back.
///
/// All four struct-carrying variants (`Static`/`Activated`/`Triggered`/`Spell`)
/// are boxed. `macro_ron_derive` peels one level of `Box` (`generate::peeled`)
/// and re-boxes on construct, so a boxed payload still reads and writes FLAT in
/// RON via `unwrap_variant_newtypes` (`Triggered(source: ..., ...)`, not
/// `Triggered((source: ...))`) — proven byte-identical by the cards fidelity
/// suite + wizards corpus. Boxing them drops `Vec<Ability>`
/// (`CardFace::abilities`) elements from ~1416 B to ~88 B and clears
/// `large_enum_variant` without a suppression. Build them via the boxing
/// constructors ([`Ability::triggered`] / `activated` / `spell` / `r#static`)
/// rather than `Ability::Triggered(Arc::new(…))`. See
/// `engine-event-size-boxing`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
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
    /// The five intrinsic variants read as themselves (`Keyword(Trample)`);
    /// every other keyword name resolves inside the `KeywordAbility`
    /// position's macro namespace (`Keyword(Flying)` invokes the builtin
    /// `KeywordAbility`-kind macro, landing on `Composite`/`Expanded`).
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
    /// see the inner ability. A look-through wrapper, like `Expanded`.
    Innate(Arc<Ability>),
    /// A remembered macro invocation ([CR#702] keyword abilities, and any other
    /// `Ability` macro). Absorbs the old `Keyword`/`KeywordAbility` shape.
    #[macro_ron(expanded)]
    Expanded(Expansion<Ability>),
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
            Ability::Expanded(expansion) => expansion.value.as_activated(),
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
            Ability::Expanded(expansion) => expansion.value.as_triggered(),
            _ => None,
        }
    }

    /// The compiled mana wrapper, if this is one, looking through wrappers.
    #[must_use]
    pub fn as_mana(&self) -> Option<&ManaAbility> {
        match self {
            Ability::Mana(mana) => Some(mana),
            Ability::Innate(inner) => inner.as_mana(),
            Ability::Expanded(expansion) => expansion.value.as_mana(),
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
    ///
    /// Looks through `Expanded`, the macro-invocation provenance wrapper
    /// (mirroring `peel_innate`'s recursion and the `Expanded` arm in
    /// `ability_is_named`): a macro-expanded `Expanded(Innate(...))` is still
    /// `Innate`, so it survives `LoseAllAbilities` and stays invisible to
    /// card-facing queries the same way a bare `Innate` does. (Currently Innate
    /// is always outermost, but Stage-4 subtype conferral may wrap conferred
    /// abilities in `Expanded`.)
    #[must_use]
    pub fn is_innate(&self) -> bool {
        match self {
            Ability::Innate(_) => true,
            Ability::Expanded(e) => e.value.is_innate(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Count;
    use crate::Reference;
    use crate::action::Action;
    use crate::action::LifeOp;
    use crate::cost::CostComponent;
    use crate::effect::OneShotEffect;

    fn read_ability(source: &str) -> Ability {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn activated_ability_parses() {
        let ability = read_ability("Activated(cost: [Tap], effect: ChangeLife(You, Up(1)))");
        assert_eq!(
            ability,
            Ability::activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: crate::Cost(vec![CostComponent::Tap].into()),
                condition: None,
                limits: vec![].into(),
                effect: OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::Literal(1))
                )),
            })
        );
    }

    /// `condition` and `limits` default-absent: parsing a bare
    /// `ActivatedAbility` without those fields yields `None`/empty-vec, and
    /// the serialized form omits them ([CR#602.5b]).
    #[test]
    fn activated_ability_condition_limits_default_absent() {
        let parsed: ActivatedAbility = crate::ron::options()
            .from_str("(cost: [Tap], effect: Sequentially([]))")
            .unwrap();
        assert_eq!(parsed.condition, None);
        assert!(parsed.limits.is_empty());
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert!(
            !written.contains("condition"),
            "absent condition omitted: {written}"
        );
        assert!(
            !written.contains("limits"),
            "absent limits omitted: {written}"
        );
    }

    /// `from` names the zone the ability functions from ([CR#113.6]); it
    /// defaults to the battlefield (`None`) and is omitted on write. Cycling's
    /// ability functions from hand ([CR#702.29a]) via `from: Hand`.
    #[test]
    fn activated_from_zone_defaults_battlefield_and_reads_hand() {
        // omitted `from` → None (the battlefield default), omitted on write.
        let parsed: ActivatedAbility = crate::ron::options()
            .from_str("(cost: [Tap], effect: ChangeLife(You, Up(1)))")
            .unwrap();
        assert_eq!(parsed.from, None);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert!(!written.contains("from"), "absent from omitted: {written}");

        // `from: Hand` reads as Some(Hand) and round-trips.
        let from_hand: ActivatedAbility = crate::ron::options()
            .from_str("(cost: [Tap], from: Hand, effect: ChangeLife(You, Up(1)))")
            .unwrap();
        assert_eq!(from_hand.from, Some(crate::Zone::Hand));
        let reser = crate::ron::options().to_string(&from_hand).unwrap();
        assert!(reser.contains("from:Hand"), "from: Hand written: {reser}");
        let reparsed: ActivatedAbility = crate::ron::options().from_str(&reser).unwrap();
        assert_eq!(reparsed, from_hand);
    }

    /// `Ability::Triggered` is now a struct variant carrying a
    /// `TriggeredAbility`; the event/effect read flat.
    #[test]
    fn triggered_ability_parses() {
        let ability = read_ability(
            "Triggered(event: ZoneChange(what: Ref(This), to: Graveyard), effect: ChangeLife(You, Up(1)))",
        );
        let Ability::Triggered(triggered) = ability else {
            panic!("expected a triggered ability");
        };
        assert_eq!(
            triggered.effect,
            OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1))
            ))
        );
        assert!(triggered.condition.is_none());
        assert!(triggered.limits.is_empty());
        // `from` defaults to the battlefield (None).
        assert_eq!(triggered.from, None);
    }

    /// A triggered ability's `from` defaults to the battlefield (`None`,
    /// omitted on write); `from: Graveyard` reads as `Some(Graveyard)` and
    /// round-trips — the graveyard-functioning trigger ([CR#113.6b]).
    #[test]
    fn triggered_from_zone_defaults_battlefield_and_reads_graveyard() {
        let omitted: TriggeredAbility = crate::ron::options()
            .from_str(
                "(event: ZoneChange(what: Ref(This), to: Graveyard), effect: ChangeLife(You, Up(1)))",
            )
            .unwrap();
        assert_eq!(omitted.from, None);
        let written = crate::ron::options().to_string(&omitted).unwrap();
        assert!(!written.contains("from"), "absent from omitted: {written}");

        let from_gy: TriggeredAbility = crate::ron::options()
            .from_str(
                "(event: Cast(who: Ref(You)), from: Graveyard, effect: ChangeLife(You, Up(1)))",
            )
            .unwrap();
        assert_eq!(from_gy.from, Some(crate::Zone::Graveyard));
        let reser = crate::ron::options().to_string(&from_gy).unwrap();
        assert!(
            reser.contains("from:Graveyard"),
            "from: Graveyard written: {reser}"
        );
        let reparsed: TriggeredAbility = crate::ron::options().from_str(&reser).unwrap();
        assert_eq!(reparsed, from_gy);
    }

    /// `Ability::Static` is a bare newtype over `StaticEffect` — it reads and
    /// writes positionally with no field name in sight.
    #[test]
    fn static_ability_reads_positionally_bare() {
        let ability = read_ability("Static(Cant(Attack(by: Ref(This))))");
        let Ability::Static(effect) = &ability else {
            panic!("expected a static ability");
        };
        assert!(matches!(effect.as_ref(), StaticEffect::Deontic(_)));
        let written = crate::ron::options().to_string(&ability).unwrap();
        assert!(
            written.starts_with("Static(Cant("),
            "bare positional: {written}"
        );
        assert!(!written.contains("effect"), "no field name: {written}");
        assert_eq!(read_ability(&written), ability, "round-trips");
    }

    /// The anthem shape at the `Ability::Static` position reads and
    /// re-serializes IDENTICALLY: `Static(Each(SelectAll(...), Modify(It,
    /// ...)))`, no `effect:`/`effects:` field name anywhere, positional all
    /// the way down.
    #[test]
    fn static_anthem_reads_and_writes_positionally() {
        let source = "Static(Each(SelectAll(And([Supertype(Basic),ColorIs(White)])),Modify(It,Power(Up(2)))))";
        let ability = read_ability(source);
        let Ability::Static(effect) = &ability else {
            panic!("expected a static ability");
        };
        assert!(matches!(effect.as_ref(), StaticEffect::Each(..)));
        let written = crate::ron::options().to_string(&ability).unwrap();
        assert_eq!(written, source, "round-trips identically: {written}");
        assert_eq!(read_ability(&written), ability);
    }

    /// `Keyword(Trample)` parses to the *known* `Ability::Keyword` variant —
    /// NOT routed to the `Expanded` macro fallthrough — and round-trips back to
    /// the same invocation string.
    #[test]
    fn keyword_parses_as_known_variant_not_expanded() {
        use crate::KeywordAbility;

        let ability = read_ability("Keyword(Trample)");
        assert_eq!(ability, Ability::Keyword(KeywordAbility::Trample));
        assert!(
            !matches!(ability, Ability::Expanded(_)),
            "Keyword must be a known variant, not macro-intercepted into Expanded"
        );
        let written = crate::ron::options().to_string(&ability).unwrap();
        assert_eq!(written, "Keyword(Trample)");
    }

    /// The COMPOSITE keyword form at the `Ability` position —
    /// `Keyword(Composite(name: ..., abilities: ...))`, the spelling
    /// keyword macros expand to — parses and round-trips.
    #[test]
    fn composite_keyword_parses_at_the_ability_position() {
        use crate::KeywordAbility;

        let ability = read_ability(r#"Keyword(Composite(name: "Ward", abilities: []))"#);
        let expected = Ability::Keyword(KeywordAbility::Composite {
            name: crate::Ident::from("Ward"),
            abilities: [].into(),
        });
        assert_eq!(ability, expected);
        let written = crate::ron::options().to_string(&ability).unwrap();
        let reread = read_ability(&written);
        assert_eq!(reread, expected);
    }

    /// `Innate(<ability>)` reads and round-trips (a self-boxed look-through
    /// variant, like `Reference::AttachHostOf`), and `peel_innate` reaches the
    /// inner ability through nesting while `is_innate` recognizes the wrapper.
    #[test]
    fn innate_round_trips_and_peels() {
        use crate::Deontic;
        use crate::DeonticAction;
        use crate::Predicate;
        use crate::StaticEffect;

        let inner = Ability::r#static(StaticEffect::Deontic(Deontic::Cant(
            DeonticAction::Attach {
                what: Predicate::Ref(Reference::This),
                to: Predicate::Not(Arc::new(Predicate::Characteristic(
                    crate::CharacteristicPredicate::Supertype(crate::Supertype::Basic),
                ))),
            },
        )));
        let innate = Ability::Innate(Arc::new(inner.clone()));
        assert!(innate.is_innate());
        assert!(!inner.is_innate());
        // peel_innate reaches the inner ability (through any nesting).
        assert_eq!(innate.peel_innate(), &inner);
        assert_eq!(inner.peel_innate(), &inner);
        // serde round-trip.
        let written = crate::ron::options().to_string(&innate).unwrap();
        assert_eq!(read_ability(&written), innate);
    }

    /// [CR#113.12]: `is_innate` looks THROUGH the `Expanded` macro-invocation
    /// wrapper — an `Expanded(Innate(...))` (which a Stage-4 subtype conferral
    /// may produce) is still recognized as `Innate`, so layer-6 ability removal
    /// retains it and card-facing queries hide it, exactly like a bare
    /// `Innate`.
    #[test]
    fn is_innate_looks_through_expanded() {
        use crate::Expansion;
        use crate::ExpansionArgs;

        let inner = Ability::r#static(StaticEffect::Modify(
            Reference::This,
            crate::Modification::LoseAllAbilities,
        ));
        let innate = Ability::Innate(Arc::new(inner.clone()));
        // A macro-expanded Innate: `Expanded(Innate(...))`.
        let wrapped = Ability::Expanded(Expansion {
            name: "AuraGraveyardRule".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(innate.clone()),
        });
        assert!(wrapped.is_innate(), "Expanded(Innate(...)) is innate");
        // Nesting deeper through another Expanded layer still resolves.
        let double = Ability::Expanded(Expansion {
            name: "Outer".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(wrapped.clone()),
        });
        assert!(
            double.is_innate(),
            "Expanded(Expanded(Innate(...))) is innate"
        );
        // A non-Innate Expanded is NOT innate (no over-matching).
        let not_innate = Ability::Expanded(Expansion {
            name: "Plain".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(inner),
        });
        assert!(
            !not_innate.is_innate(),
            "Expanded(Static(...)) is not innate"
        );
    }

    /// `ChooseSpec` carries a `Quantity` count, a spelled `chooser` (Law 2:
    /// no read-time default — the decider is always spelled), a defaulted
    /// `rider` (`None`), and round-trips with the entwine/escalate riders
    /// ([CR#700.2,702.42a,702.120a]).
    #[test]
    fn choose_spec_quantity_chooser_and_rider_round_trip() {
        use crate::Count;
        use crate::Quantity;

        // Bare "choose one": count + chooser; rider omitted on write.
        let one: ChooseSpec = crate::ron::options()
            .from_str("(count: Range(1, 1), chooser: You)")
            .unwrap();
        assert_eq!(one.count, Quantity::one());
        assert_eq!(one.chooser, crate::Reference::You);
        assert!(one.rider.is_none());
        let written = crate::ron::options().to_string(&one).unwrap();
        assert!(
            !written.contains("rider"),
            "rider default omitted: {written}"
        );
        assert!(written.contains("You"), "chooser is spelled: {written}");

        // Escalate: "one or more" + the per-extra-mode cost ([CR#702.120a]).
        let escalate: ChooseSpec = crate::ron::options()
            .from_str(
                "(count: Range(1, None), chooser: You, rider: Escalate([Mana([Generic(1)])]))",
            )
            .unwrap();
        assert_eq!(
            escalate.count,
            Quantity::Range(Some(Count::Literal(1)), None)
        );
        assert!(matches!(escalate.rider, Some(ModalCostRider::Escalate(_))));
        let written = crate::ron::options().to_string(&escalate).unwrap();
        let reread: ChooseSpec = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reread, escalate);

        // Entwine: printed Exactly(1) + the all-modes cost ([CR#702.42a]);
        // a foreign chooser round-trips.
        let entwine: ChooseSpec = crate::ron::options()
            .from_str(
                "(count: Range(1, 1), chooser: Opponent, rider: Entwine([Mana([Generic(2)])]))",
            )
            .unwrap();
        assert_eq!(entwine.chooser, crate::Reference::Opponent);
        assert!(matches!(entwine.rider, Some(ModalCostRider::Entwine(_))));
        let written = crate::ron::options().to_string(&entwine).unwrap();
        let reread: ChooseSpec = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reread, entwine);
    }

    #[test]
    fn sacrifice_this_reads_flat() {
        // Confirms the new flattened Selection (`This`, not `That(This)`).
        let ability = read_ability(
            "Activated(cost: [Tap, Act(Sacrifice(You, This))], effect: AddMana(You, Literal(1), AnyColor))",
        );
        let Ability::Activated(activated) = ability else {
            panic!("expected an activated ability");
        };
        assert_eq!(
            activated.cost[1],
            CostComponent::do_action(Action::Sacrifice(Reference::You, Reference::This))
        );
    }

    #[test]
    fn modal_mana_profile_is_instantiated_from_announced_modes() {
        let Ability::Activated(ability) =
            read_ability("Activated(cost: [], effect: AddMana(You, Literal(1), AnyColor))")
        else {
            panic!("fixture must be activated");
        };
        let modal = Ability::Mana(ManaAbility::Activated {
            ability,
            profile: ActivatedManaProfile::ByAnnouncedMode(
                vec![
                    ManaModeClass {
                        adds_mana: true,
                        targetless: true,
                    },
                    ManaModeClass {
                        adds_mana: true,
                        targetless: false,
                    },
                ]
                .into(),
            ),
        });
        assert!(modal.mana_profile_for_modes(&[0]));
        assert!(!modal.mana_profile_for_modes(&[1]));
        assert!(!modal.mana_profile_for_modes(&[0, 1]));
        assert!(!modal.mana_profile_for_modes(&[]));
        assert!(!modal.mana_profile_for_modes(&[9]));
    }
}
