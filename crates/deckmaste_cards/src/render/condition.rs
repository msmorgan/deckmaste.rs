//! Rendering for `Condition` predicates — the intervening-if / "only if"
//! clauses around triggered and activated abilities ([CR#603.4,602.5b]).

use std::sync::Arc;

use deckmaste_core::AggregateOp;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::Cmp;
use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::PlayerAttr;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Stat;
use deckmaste_core::StatePredicate;
use deckmaste_core::Status;
use deckmaste_core::Supertype;
use deckmaste_core::Zone;

use super::Ctx;
use super::fragment::strip_expanded;

/// A `Condition` as a clause with no leading/trailing punctuation, sized to sit
/// after "if" / "only if": "it's your turn", "it's an opponent's turn".
/// Unhandled conditions yield an `[unrendered: …]` marker, never a panic.
///
/// `ctx` anchors the reference-bearing conditions (`Compare(CounterCount(r,
/// …), …)`'s `r`, "this enchantment has …") — the caller threads a
/// self-type-noun subject through it for a triggered ability's
/// intervening-if ([CR#603.4], see `ability.rs::self_type_phrase`).
pub(super) fn condition(c: &Condition, ctx: &Ctx) -> String {
    match c {
        // Look through a macro-provenance wrapper.
        Condition::Expanded(e) => condition(&e.value, ctx),
        // `YourTurn` is sugar for `TurnOf(Ref(You))`; both render the same.
        Condition::YourTurn => "it's your turn".to_string(),
        Condition::TurnOf(filter) => format!("it's {}", turn_owner(filter)),
        // [CR#122.1,603.4]: "this enchantment has ten or more luck counters
        // on it" (Chance Encounter) — only the counter-count `AtLeast`
        // shape is recognized; any other `Compare` operand/comparator falls
        // through to the generic marker.
        Condition::Compare(Count::CounterCount(r, kind), Cmp::AtLeast, Count::Literal(n)) => {
            format!(
                "{} has {} or more {} counters on it",
                super::fragment::reference(r, ctx),
                super::fragment::number_word(*n).map_or_else(|| n.to_string(), str::to_string),
                super::fragment::counter_noun(kind.as_str()),
            )
        }
        // "you have the city's blessing" ([`YouHaveTheCitysBlessing`
        // Condition macro](../../../../plugins/builtin/macros/condition/YouHaveTheCitysBlessing.
        // ron)).
        Condition::Matches(Reference::You, filter)
            if matches!(
                strip_expanded(filter),
                Predicate::State(StatePredicate::Designated(name)) if name.as_str() == "CitysBlessing"
            ) =>
        {
            "you have the city's blessing".to_string()
        }
        // "there are seven or more cards in your graveyard"
        // ([`SevenOrMoreCardsInYourGraveyard`]).
        Condition::Compare(
            Count::CountOf(Countable::Objects(f)),
            Cmp::AtLeast,
            Count::Literal(7),
        ) if f.as_ref() == &graveyard_cards_you_own() => {
            "there are seven or more cards in your graveyard".to_string()
        }
        // "creatures you control have total power 8 or greater"
        // ([`CreaturesYouControlHaveTotalPowerEightOrGreater`]).
        Condition::Compare(
            Count::Aggregate(AggregateOp::SumOf, proj),
            Cmp::AtLeast,
            Count::Literal(8),
        ) if matches!(&proj.of, Countable::Objects(f) if f.as_ref() == &creatures_you_control())
            && proj.by.as_ref() == &Count::StatOf(Reference::It, Stat::Power) =>
        {
            "creatures you control have total power 8 or greater".to_string()
        }
        // "you control three or more artifacts" ([`YouControlThreeOrMoreArtifacts`]).
        Condition::Compare(
            Count::CountOf(Countable::Objects(f)),
            Cmp::AtLeast,
            Count::Literal(3),
        ) if f.as_ref() == &permanents_you_control(deckmaste_core::Type::Artifact.name()) => {
            "you control three or more artifacts".to_string()
        }
        // "you have no cards in hand" ([`YouHaveNoCardsInHand`]).
        Condition::Compare(
            Count::PlayerStatOf(Reference::You, PlayerAttr::HandSize),
            Cmp::Eq,
            Count::Literal(0),
        ) => "you have no cards in hand".to_string(),
        // "you control a Swamp"/"an Island"/"a Mountain"/"a Plains"/"a
        // Forest" (the `YouControlA<Land>` basic-land census macros).
        Condition::Exists(f) if basic_land_you_control(f).is_some() => {
            format!(
                "you control {}",
                basic_land_you_control(f).expect("checked above")
            )
        }
        // "you control X" / "an opponent controls X" ([CR#603.4]) — the
        // `YouControl`/`AnOpponentControls` macros' `Exists` shape. AFTER the
        // basic-land census arm above: that arm's 3-atom filter isn't the
        // 2-atom `And([object, ControlledBy])` shape this one reads, and a
        // guarded specific arm must precede the unguarded generic one.
        Condition::Exists(pred) => exists_phrase(pred),
        // "<subject> is <predicate>" — the `SubjectIs` macro's shape. AFTER
        // the city's-blessing arm above, which is a `Matches(You, Designated)`
        // this generic arm would otherwise shadow into a marker.
        Condition::Matches(r, pred) => matches_phrase(r, pred, ctx, false),
        // "<subject> isn't <predicate>" — the `SubjectIsnt` macro's shape
        // (`Not(Matches(..))`). Any other negated condition falls through to
        // the generic marker.
        Condition::Not(inner) => match &**inner {
            Condition::Matches(r, pred) => matches_phrase(r, pred, ctx, true),
            other => format!("[unrendered: Not({other:?})]"),
        },
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The `And([InZone(Battlefield), Owner|ControlledBy(Ref(You)), …])` census
/// filter shapes the `plugins/builtin/macros/condition/*.ron` board-census
/// macros are authored against — matched STRUCTURALLY (never by reading the
/// macro's own `template:` field), mirroring this file's existing
/// `Compare(CounterCount(...), ...)` arm and the crate-wide `Expanded`
/// peeling convention.
fn graveyard_cards_you_own() -> Predicate {
    Predicate::And(vec![
        Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
        Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
            Reference::You,
        )))),
    ])
}

fn creatures_you_control() -> Predicate {
    permanents_you_control(deckmaste_core::Type::Creature.name())
}

/// `And([InZone(Battlefield), Type(ty), ControlledBy(Ref(You))])` — the
/// battlefield-census filter for a single card type, controlled by you.
fn permanents_you_control(ty: deckmaste_core::Ident) -> Predicate {
    Predicate::And(vec![
        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
        Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(ty)),
        Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
            Reference::You,
        )))),
    ])
}

/// `And([InZone(Battlefield), Subtype(name), ControlledBy(Ref(You))])` for
/// one of the five basic land subtypes — the `YouControlA<Land>` macros'
/// shared shape. Returns the printed land phrase ("a Swamp", "an Island")
/// when `f` is exactly that shape for a recognized basic land name.
fn basic_land_you_control(f: &Predicate) -> Option<&'static str> {
    let Predicate::And(parts) = f else { return None };
    let [
        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
        Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Subtype(name)),
        Predicate::Relation(RelationPredicate::ControlledBy(controller)),
    ] = parts.as_slice()
    else {
        return None;
    };
    if controller.as_ref() != &Predicate::Ref(Reference::You) {
        return None;
    }
    match name.as_str() {
        "Swamp" => Some("a Swamp"),
        "Island" => Some("an Island"),
        "Mountain" => Some("a Mountain"),
        "Plains" => Some("a Plains"),
        "Forest" => Some("a Forest"),
        _ => None,
    }
}

/// "you control X" / "an opponent controls X" — the [`YouControl`]/
/// [`AnOpponentControls`] macros' `And([<object>, ControlledBy(<who>)])`
/// shape: splits into the controller phrase and the object's own indefinite
/// noun phrase ([`object_phrase`]). Any other `Exists` shape (or an
/// unrecognized controller/object) falls to the generic marker — the generic
/// macro-template filler can't supply the "a"/"an" article, so this shape
/// gets a hand-written render arm rather than the macro's own template.
///
/// [`YouControl`]: crate's `plugins/builtin/macros/condition/YouControl.ron`
/// [`AnOpponentControls`]: crate's
/// `plugins/builtin/macros/condition/AnOpponentControls.ron`
fn exists_phrase(pred: &Predicate) -> String {
    if let Predicate::And(parts) = strip_expanded(pred)
        && let [object, control] = parts.as_slice()
        && let Predicate::Relation(RelationPredicate::ControlledBy(who)) = strip_expanded(control)
    {
        let verb = match strip_expanded(who) {
            Predicate::Ref(Reference::You) => Some("you control"),
            Predicate::Relation(RelationPredicate::OpponentOf(inner))
                if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
            {
                Some("an opponent controls")
            }
            _ => None,
        };
        if let (Some(verb), Some(noun)) = (verb, object_phrase(object)) {
            return format!("{verb} {noun}");
        }
    }
    format!("[unrendered: {pred:?}]")
}

/// "<subject> is <predicate>" / "<subject> isn't <predicate>" — the
/// [`SubjectIs`]/[`SubjectIsnt`] macros' `Matches`/`Not(Matches(..))` shape.
/// The subject renders via [`condition_subject`] (mid-sentence, lowercase —
/// unlike `render/ability.rs`'s sentence-initial `reference_subject`); the
/// predicate via [`state_predicate_phrase`]. Either side declining (an
/// unrecognized subject reference, or a predicate outside the composition's
/// vocabulary) falls to the generic marker.
///
/// [`SubjectIs`]: crate's `plugins/builtin/macros/condition/SubjectIs.ron`
/// [`SubjectIsnt`]: crate's `plugins/builtin/macros/condition/SubjectIsnt.ron`
fn matches_phrase(reference: &Reference, pred: &Predicate, ctx: &Ctx, negate: bool) -> String {
    match (
        condition_subject(reference, ctx),
        state_predicate_phrase(pred),
    ) {
        (Some(subject), Some(state)) => {
            let copula = if negate { "isn't" } else { "is" };
            format!("{subject} {copula} {state}")
        }
        _ if negate => format!("[unrendered: Not(Matches({reference:?}, {pred:?}))]"),
        _ => format!("[unrendered: Matches({reference:?}, {pred:?})]"),
    }
}

/// A `Condition::Matches` subject as MID-SENTENCE English (lowercase): `This`
/// -> the ctx subject ("~" at top level); `AttachHostOf(This)` -> "enchanted
/// creature" — the lowercase twin of `render/ability.rs`'s
/// `reference_subject` (which capitalizes for sentence-start use). Any other
/// reference declines — this composition's subject vocabulary is the closed
/// set `crate::parsers`' condition routing accepts (see the migrations-side
/// `SUBJECT_WORDS`).
fn condition_subject(r: &Reference, ctx: &Ctx) -> Option<String> {
    match r {
        Reference::This => Some(ctx.subject.to_string()),
        Reference::AttachHostOf(inner) if matches!(**inner, Reference::This) => {
            Some("enchanted creature".to_string())
        }
        _ => None,
    }
}

/// A `Condition::Matches`/`SubjectIsnt` predicate slot's English: a bare
/// combat/tap state (attacking/blocking/tapped/untapped), a color, "legendary"
/// ([`Supertype::Legendary`]), or — falling through — a full
/// object-description noun phrase ([`object_phrase`]). Mirrors
/// `crate::parsers::condition::condition_predicate`'s parse-side vocabulary
/// exactly, so nothing this composition parses renders as `None` here.
fn state_predicate_phrase(pred: &Predicate) -> Option<String> {
    match strip_expanded(pred) {
        Predicate::State(StatePredicate::Attacking) => Some("attacking".to_string()),
        Predicate::State(StatePredicate::Blocking) => Some("blocking".to_string()),
        Predicate::State(StatePredicate::Status(Status::Tapped)) => Some("tapped".to_string()),
        Predicate::State(StatePredicate::Status(Status::Untapped)) => Some("untapped".to_string()),
        Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Legendary)) => {
            Some("legendary".to_string())
        }
        Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
            Some(super::effect::color_word(*c).to_string())
        }
        other => object_phrase(other),
    }
}

/// A singular object-description `Predicate` -> its indefinite noun phrase —
/// the reverse of the CLOSED vocabulary
/// `crate::parsers::condition::condition_predicate`'s
/// [`crate::parsers::filter::parse_phrase`] fallback accepts: a bare card
/// type / a creature-type subtype noun (bare, "a Human", OR as an adjective
/// before an explicit type noun, "a Domri planeswalker"/"a Griffin creature")
/// / a single color or multicolored/colorless adjective / the
/// "other"/"another" self-exclusion. `None` for a filter built from any other
/// atom — the caller then falls back to a visible marker rather than
/// dropped/garbled English.
fn object_phrase(pred: &Predicate) -> Option<String> {
    let mut other = false;
    let mut color: Option<deckmaste_core::Color> = None;
    let mut multicolored = false;
    let mut colorless = false;
    let mut subtype: Option<String> = None;
    for part in super::fragment::flatten_all_of(pred) {
        match strip_expanded(part) {
            Predicate::Not(inner) if strip_expanded(inner).is_this() => {
                other = true;
            }
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => color = Some(*c),
            Predicate::Characteristic(CharacteristicPredicate::Multicolored) => {
                multicolored = true;
            }
            Predicate::Characteristic(CharacteristicPredicate::Colorless) => colorless = true,
            Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) => {
                subtype = Some(s.to_string());
            }
            _ => {}
        }
    }
    // The explicitly-WRITTEN type noun, if any: `find_card_type`'s `Type(_)`
    // atom, or a macro-provenance noun OTHER than "permanent" — `parse_phrase`
    // emits a BARE subtype's implicit `Permanent` scope atom (never a printed
    // noun; "a Human", not "a Human permanent") through the exact same
    // macro-expansion path as an explicit "creature"/"planeswalker" noun
    // (`strip_subtype_adjective`'s consumed head), so "permanent" specifically
    // is excluded here ONLY when a subtype is ALSO present (the bare-subtype
    // case) — with no subtype, "permanent" is a real printed noun in its own
    // right ("another multicolored permanent", "a red permanent").
    let type_noun = super::fragment::find_card_type(pred)
        .map(|t| t.as_str().to_lowercase())
        .or_else(|| super::fragment::find_macro_noun(pred));
    let type_noun = if subtype.is_some() {
        type_noun.filter(|n| n != "permanent")
    } else {
        type_noun
    };
    let noun = match (subtype, type_noun) {
        (Some(s), Some(t)) => format!("{s} {t}"),
        (Some(s), None) => s,
        (None, Some(t)) => t,
        (None, None) => return None,
    };
    let described = if let Some(c) = color {
        format!("{} {noun}", super::effect::color_word(c))
    } else if multicolored {
        format!("multicolored {noun}")
    } else if colorless {
        format!("colorless {noun}")
    } else {
        noun
    };
    Some(if other {
        format!("another {described}")
    } else {
        super::effect::a_an(&described)
    })
}

/// The possessive turn-owner phrase for `TurnOf(<player predicate>)`:
/// `Ref(You)` → "your turn", `OpponentOf(Ref(You))` → "an opponent's turn",
/// `TeammateOf(Ref(You))` → "a teammate's turn".
fn turn_owner(filter: &Predicate) -> String {
    match strip_expanded(filter) {
        Predicate::Ref(Reference::You) => "your turn".to_string(),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "an opponent's turn".to_string()
        }
        Predicate::Relation(RelationPredicate::TeammateOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "a teammate's turn".to_string()
        }
        other => format!("[unrendered: {other:?}] turn"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Ctx<'static> {
        Ctx {
            subject: "~",
            targets: &[],
            that: None,
            named: None,
        }
    }

    /// `YourTurn` and its `TurnOf` generalization render the timing clause; the
    /// two spellings of "your turn" agree, and an opponent's turn reads.
    #[test]
    fn renders_turn_of_and_your_turn() {
        assert_eq!(condition(&Condition::YourTurn, &ctx()), "it's your turn");
        assert_eq!(
            condition(&Condition::TurnOf(Predicate::Ref(Reference::You)), &ctx()),
            "it's your turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Predicate::Relation(RelationPredicate::OpponentOf(
                    Arc::new(Predicate::Ref(Reference::You))
                ))),
                &ctx()
            ),
            "it's an opponent's turn"
        );
        assert_eq!(
            condition(
                &Condition::TurnOf(Predicate::Relation(RelationPredicate::TeammateOf(
                    Arc::new(Predicate::Ref(Reference::You))
                ))),
                &ctx()
            ),
            "it's a teammate's turn"
        );
    }

    /// The board-census "Activate only if …" condition macros
    /// (`plugins/builtin/macros/condition/*.ron`) render back to their own
    /// printed phrase — checked structurally, matching how `condition()`
    /// recognizes them (never by reading the macro's own `template:` field).
    #[test]
    fn renders_board_census_condition_macros() {
        assert_eq!(
            condition(
                &Condition::Compare(
                    Count::CountOf(Countable::Objects(Arc::new(graveyard_cards_you_own()))),
                    Cmp::AtLeast,
                    Count::Literal(7),
                ),
                &ctx()
            ),
            "there are seven or more cards in your graveyard"
        );
        assert_eq!(
            condition(
                &Condition::Compare(
                    Count::Aggregate(
                        AggregateOp::SumOf,
                        deckmaste_core::Projection {
                            of: Countable::Objects(Arc::new(creatures_you_control())),
                            by: Arc::new(Count::StatOf(Reference::It, Stat::Power)),
                        },
                    ),
                    Cmp::AtLeast,
                    Count::Literal(8),
                ),
                &ctx()
            ),
            "creatures you control have total power 8 or greater"
        );
        assert_eq!(
            condition(
                &Condition::Compare(
                    Count::CountOf(Countable::Objects(Arc::new(permanents_you_control(
                        deckmaste_core::Type::Artifact.name()
                    )))),
                    Cmp::AtLeast,
                    Count::Literal(3),
                ),
                &ctx()
            ),
            "you control three or more artifacts"
        );
        assert_eq!(
            condition(
                &Condition::Compare(
                    Count::PlayerStatOf(Reference::You, PlayerAttr::HandSize),
                    Cmp::Eq,
                    Count::Literal(0),
                ),
                &ctx()
            ),
            "you have no cards in hand"
        );
        for (subtype, phrase) in [
            ("Swamp", "a Swamp"),
            ("Island", "an Island"),
            ("Mountain", "a Mountain"),
            ("Plains", "a Plains"),
            ("Forest", "a Forest"),
        ] {
            let filter = Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Subtype(
                    subtype.into(),
                )),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]);
            assert_eq!(
                condition(&Condition::Exists(filter), &ctx()),
                format!("you control {phrase}")
            );
        }
        assert_eq!(
            condition(
                &Condition::Matches(
                    Reference::You,
                    Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                ),
                &ctx()
            ),
            "you have the city's blessing"
        );
    }

    /// `Exists(And([<object>, ControlledBy(<who>)]))` — the `YouControl`/
    /// `AnOpponentControls` macros' shape — renders "you control X" / "an
    /// opponent controls X", the object's own indefinite noun phrase.
    #[test]
    fn renders_exists_you_control_and_opponent_controls() {
        let artifact = Predicate::type_(deckmaste_core::Type::Artifact);
        assert_eq!(
            condition(
                &Condition::Exists(Predicate::And(vec![
                    artifact.clone(),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You
                    )))),
                ])),
                &ctx()
            ),
            "you control an artifact"
        );
        // A BARE creature-type subtype noun ("a Human") carries the implicit
        // `Permanent` scope atom — a macro-provenance `Expanded` value, the
        // exact shape `crate::parsers::filter::parse_phrase` emits — which
        // must NOT print as "a Human permanent".
        let permanent = Predicate::Expanded(deckmaste_core::Expansion {
            name: deckmaste_core::Ident::from("Permanent"),
            args: deckmaste_core::ExpansionArgs::none(),
            template: Some("permanent".to_string()),
            value: Box::new(Predicate::State(StatePredicate::InZone(
                deckmaste_core::Zone::Battlefield,
            ))),
        });
        let human = Predicate::And(vec![
            permanent,
            Predicate::Characteristic(CharacteristicPredicate::Subtype(
                deckmaste_core::Ident::from("Human"),
            )),
        ]);
        assert_eq!(
            condition(
                &Condition::Exists(Predicate::And(vec![
                    human,
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                        Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(
                            Predicate::Ref(Reference::You)
                        )))
                    ))),
                ])),
                &ctx()
            ),
            "an opponent controls a Human"
        );
        // A subtype ADJECTIVE before an EXPLICIT type noun ("a Griffin
        // creature", "a Domri planeswalker") — the noun DOES print, unlike
        // the bare-subtype case above.
        let griffin_creature = Predicate::And(vec![
            Predicate::creature(),
            Predicate::Characteristic(CharacteristicPredicate::Subtype(
                deckmaste_core::Ident::from("Griffin"),
            )),
        ]);
        assert_eq!(
            condition(
                &Condition::Exists(Predicate::And(vec![
                    griffin_creature,
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You
                    )))),
                ])),
                &ctx()
            ),
            "you control a Griffin creature"
        );
        // "another multicolored permanent" / "a red permanent" — a bare
        // `Permanent` macro-provenance noun with NO subtype alongside DOES
        // print ("permanent" is the real head noun here, unlike the bare
        // "a Human" case above, where `Permanent` is only the implicit scope
        // atom the subtype itself stands in for).
        let permanent_noun = || {
            Predicate::Expanded(deckmaste_core::Expansion {
                name: deckmaste_core::Ident::from("Permanent"),
                args: deckmaste_core::ExpansionArgs::none(),
                template: Some("permanent".to_string()),
                value: Box::new(Predicate::State(StatePredicate::InZone(
                    deckmaste_core::Zone::Battlefield,
                ))),
            })
        };
        let another_multicolored_permanent = Predicate::And(vec![
            permanent_noun(),
            Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
            Predicate::Characteristic(CharacteristicPredicate::Multicolored),
        ]);
        assert_eq!(
            condition(
                &Condition::Exists(Predicate::And(vec![
                    another_multicolored_permanent,
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You
                    )))),
                ])),
                &ctx()
            ),
            "you control another multicolored permanent"
        );
        let red_permanent = Predicate::And(vec![
            permanent_noun(),
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(deckmaste_core::Color::Red)),
        ]);
        assert_eq!(
            condition(
                &Condition::Exists(Predicate::And(vec![
                    red_permanent,
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You
                    )))),
                ])),
                &ctx()
            ),
            "you control a red permanent"
        );
    }

    /// `Matches`/`Not(Matches(..))` — the `SubjectIs`/`SubjectIsnt` macros'
    /// shape — renders "<subject> is/isn't <predicate>": `This` -> the ctx
    /// subject ("~"), `AttachHostOf(This)` -> "enchanted creature" (lowercase,
    /// mid-sentence); the predicate a bare state word, a color, or a full
    /// object noun phrase.
    #[test]
    fn renders_matches_subject_and_predicate_shapes() {
        assert_eq!(
            condition(
                &Condition::Matches(Reference::This, Predicate::State(StatePredicate::Attacking)),
                &ctx()
            ),
            "~ is attacking"
        );
        assert_eq!(
            condition(
                &Condition::Matches(
                    Reference::AttachHostOf(Arc::new(Reference::This)),
                    Predicate::Characteristic(CharacteristicPredicate::ColorIs(
                        deckmaste_core::Color::Black
                    )),
                ),
                &ctx()
            ),
            "enchanted creature is black"
        );
        assert_eq!(
            condition(
                &Condition::Not(Arc::new(Condition::Matches(
                    Reference::AttachHostOf(Arc::new(Reference::This)),
                    Predicate::Characteristic(CharacteristicPredicate::Supertype(
                        Supertype::Legendary
                    )),
                ))),
                &ctx()
            ),
            "enchanted creature isn't legendary"
        );
    }

    /// A `Matches` subject outside the composition's closed vocabulary (a
    /// targeted reference here) falls to the generic marker rather than wrong
    /// or dropped text.
    #[test]
    fn matches_declines_unrecognized_subject_to_a_marker() {
        let rendered = condition(
            &Condition::Matches(Reference::It, Predicate::State(StatePredicate::Attacking)),
            &ctx(),
        );
        assert!(rendered.contains("[unrendered:"), "{rendered}");
    }
}
