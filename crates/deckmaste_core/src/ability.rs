use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Count;
use crate::Event;
use crate::Expand;
use crate::Expansion;
use crate::KeywordAbility;
use crate::SupportsMacros;
use crate::Window;
use crate::continuous::StaticEffect;
use crate::cost::Cost;
use crate::cost::CostComponent;
use crate::effect::Effect;

/// A spell ability — what an instant or sorcery does on resolution
/// ([CR#113.3a]). Targeting, when present, lives on an `Effect::Targeted`
/// wrapper in `effect` ([CR#115.1,601.2c]), referenced by index (`Target(0)`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct SpellAbility {
    pub effect: Effect,
}

/// An activated ability: paid with a cost and produces an effect
/// ([CR#113.3b,602]). Targeting lives on an `Effect::Targeted` wrapper in
/// `effect` ([CR#115.1,601.2c]); the `Resolvable` wrapper of the design sketch
/// is realized as `Effect::Modal` (see `effect`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ActivatedAbility {
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
    pub window: Option<Window>,
    /// "Activate only if [state]" ([CR#602.5b..602.5e]) — a predicate over
    /// the game state at activation time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// "Activate only once each turn." ([CR#602.5b]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<UseLimit>,
    pub effect: Effect,
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
}

/// A triggered ability ([CR#113.3c,603]). A named struct because it recurs:
/// delayed ([CR#603.7]) and reflexive ([CR#603.12]) triggers are the same
/// value, created inside an `Effect`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct TriggeredAbility {
    /// The event that triggers it ([CR#603.2]).
    pub event: Event,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limits: Vec<UseLimit>,
    pub effect: Effect,
}

/// A static ability ([CR#113.3d,604]). Its duration is implicit: while it
/// functions ([CR#611.3]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct StaticAbility {
    /// The zone the ability functions from ([CR#113.6,604.3]). `None` = the
    /// battlefield default (omitted on write); a `Some` names another zone the
    /// source must be in for the static to apply — a graveyard/hand static
    /// (Riftstone Portal's land-mana from the graveyard, the incarnation
    /// cycle's "as long as this is in your graveyard …") sets `from:
    /// Graveyard` / `from: Hand`. Mirrors [`ActivatedAbility::from`] /
    /// [`TriggeredAbility::from`]; engine zone-gating for statics lands with
    /// `engine-static-ability-zone-gating`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::Zone>,
    /// When the ability functions, if conditional ([CR#611.3a] — the effect
    /// is never locked in; it applies to whatever its text indicates).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    pub effects: Vec<StaticEffect>,
    /// The one explicit, validated flag ([CR#604.3]): a characteristic-defining
    /// ability applies in layer 7a / a/b/c per its op.
    #[serde(default, skip_serializing_if = "is_false")]
    pub characteristic_defining: bool,
}

/// A `skip_serializing_if` predicate: a `false` bool is omitted from RON.
/// serde requires the predicate to take `&T`, hence the by-ref bool.
#[expect(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn is_false(b: &bool) -> bool {
    !*b
}

/// How a modal spell or ability's modes are chosen ([CR#700.2]). `up_to` is the
/// "up to N" form ([CR#700.2]); `repeats` allows choosing the same mode more
/// than once ([CR#700.2d]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ChooseSpec {
    pub count: Count,
    #[serde(default, skip_serializing_if = "is_false")]
    pub up_to: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub repeats: bool,
}

/// One mode of a modal spell or ability ([CR#700.2]). A mode's targets live on
/// an `Effect::Targeted` wrapper in its `effect` ([CR#700.2c,115.8]); it may
/// carry a per-mode cost ([CR#700.2h]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Mode {
    pub effect: Effect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<Vec<CostComponent>>,
}

/// An ability ([CR#113]). The struct-carrying variants read flat in RON —
/// `Activated(cost: ..., ...)`, not `Activated((cost: ...))` — via the
/// `unwrap_variant_newtypes` extension.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back.
///
/// The struct-carrying variants are deliberately unboxed so they read and
/// write flat in RON via `unwrap_variant_newtypes`; the size spread that
/// triggers `large_enum_variant` (`Triggered` dominates) is accepted here —
/// boxing would push `Box::new` into every construction site for no runtime
/// gain on a type that is itself usually behind a `Vec`/`Box` already
/// (`CardFace::abilities`, `Modification::GainAbility(Box<Ability>)`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
#[allow(clippy::large_enum_variant)]
pub enum Ability {
    Static(StaticAbility),
    Activated(ActivatedAbility),
    Triggered(TriggeredAbility),
    Spell(SpellAbility),
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
    Innate(Box<Ability>),
    /// A remembered macro invocation ([CR#702] keyword abilities, and any other
    /// `Ability` macro). Absorbs the old `Keyword`/`KeywordAbility` shape.
    #[macro_ron(expanded)]
    Expanded(Expansion<Ability>),
}

impl Ability {
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
    use crate::Selection;
    use crate::action::Action;
    use crate::action::PlayerAction;
    use crate::cost::CostComponent;
    use crate::effect::Effect;

    fn read_ability(source: &str) -> Ability {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn activated_ability_parses() {
        let ability = read_ability("Activated(cost: [Tap], effect: Draw(Literal(1)))");
        assert_eq!(
            ability,
            Ability::Activated(ActivatedAbility {
                from: None,
                window: None,
                cost: vec![CostComponent::Tap].into(),
                condition: None,
                limits: vec![],
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::Draw(Count::Literal(1))
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
            .from_str("(cost: [Tap], effect: Sequence([]))")
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
            .from_str("(cost: [Tap], effect: Draw(Literal(1)))")
            .unwrap();
        assert_eq!(parsed.from, None);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert!(!written.contains("from"), "absent from omitted: {written}");

        // `from: Hand` reads as Some(Hand) and round-trips.
        let from_hand: ActivatedAbility = crate::ron::options()
            .from_str("(cost: [Tap], from: Hand, effect: Draw(Literal(1)))")
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
            "Triggered(event: ZoneMove(what: Ref(This), to: Graveyard), effect: Draw(Literal(1)))",
        );
        let Ability::Triggered(triggered) = ability else {
            panic!("expected a triggered ability");
        };
        assert_eq!(
            triggered.effect,
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::Draw(Count::Literal(1))
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
            .from_str("(event: ZoneMove(what: Ref(This), to: Graveyard), effect: Draw(Literal(1)))")
            .unwrap();
        assert_eq!(omitted.from, None);
        let written = crate::ron::options().to_string(&omitted).unwrap();
        assert!(!written.contains("from"), "absent from omitted: {written}");

        let from_gy: TriggeredAbility = crate::ron::options()
            .from_str(
                r#"(event: Performed(verb: "Cast"), from: Graveyard, effect: Draw(Literal(1)))"#,
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

    /// `Ability::Static` is now a struct variant carrying a `StaticAbility`;
    /// the CDA flag is omitted when false.
    #[test]
    fn static_ability_parses_and_omits_cda() {
        let ability = read_ability("Static(effects: [Cant(Attack(by: Ref(This)))])");
        let Ability::Static(static_ability) = &ability else {
            panic!("expected a static ability");
        };
        assert!(!static_ability.characteristic_defining);
        let written = crate::ron::options().to_string(&ability).unwrap();
        assert!(
            !written.contains("characteristic_defining"),
            "false CDA flag should be omitted: {written}"
        );
        assert!(!written.contains("condition"), "absent condition omitted");
        // `from` defaults to the battlefield (None), omitted on write.
        assert!(!written.contains("from"), "absent from omitted: {written}");
    }

    /// A static ability's `from` defaults to the battlefield (`None`, omitted
    /// on write); `from: Graveyard` reads as `Some(Graveyard)` and
    /// round-trips — the graveyard-functioning static (Riftstone Portal,
    /// [CR#604.3]).
    #[test]
    fn static_from_zone_defaults_battlefield_and_reads_graveyard() {
        let omitted: StaticAbility = crate::ron::options()
            .from_str("(effects: [Cant(Attack(by: Ref(This)))])")
            .unwrap();
        assert_eq!(omitted.from, None);

        let from_gy: StaticAbility = crate::ron::options()
            .from_str("(from: Graveyard, effects: [Cant(Attack(by: Ref(This)))])")
            .unwrap();
        assert_eq!(from_gy.from, Some(crate::Zone::Graveyard));
        let reser = crate::ron::options().to_string(&from_gy).unwrap();
        assert!(
            reser.contains("from:Graveyard"),
            "from: Graveyard written: {reser}"
        );
        let reparsed: StaticAbility = crate::ron::options().from_str(&reser).unwrap();
        assert_eq!(reparsed, from_gy);
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
            abilities: Vec::new(),
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
        use crate::Filter;
        use crate::StaticAbility;
        use crate::StaticEffect;

        let inner = Ability::Static(StaticAbility {
            from: None,
            condition: None,
            effects: vec![StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Filter::Ref(Reference::This),
                    to: Filter::Not(Box::new(Filter::Characteristic(
                        crate::CharacteristicFilter::Type(crate::Type::Creature),
                    ))),
                },
            ))],
            characteristic_defining: false,
        });
        let innate = Ability::Innate(Box::new(inner.clone()));
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

        let inner = Ability::Static(StaticAbility {
            from: None,
            condition: None,
            effects: vec![],
            characteristic_defining: false,
        });
        let innate = Ability::Innate(Box::new(inner.clone()));
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

    #[test]
    fn sacrifice_this_reads_flat() {
        // Confirms the new flattened Selection (`This`, not `That(This)`).
        let ability = read_ability(
            "Activated(cost: [Tap, Do(Sacrifice(This))], effect: AddMana(Literal(1), AnyColor))",
        );
        let Ability::Activated(activated) = ability else {
            panic!("expected an activated ability");
        };
        assert_eq!(
            activated.cost[1],
            CostComponent::Do(Box::new(PlayerAction::Sacrifice(Selection::Ref(
                Reference::This
            ))))
        );
    }
}
