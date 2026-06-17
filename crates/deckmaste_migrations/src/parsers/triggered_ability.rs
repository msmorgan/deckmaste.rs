//! The `Triggered` frame parser: a "When/Whenever <event>, <effect>." line ->
//! the bare `Triggered(...)` ability RON. A triggered ability is written as
//! "[trigger condition], [effect]" divided by the trigger word [CR#603.1]. The
//! effect grammar is shared via [`crate::parsers::effect`]; the event grammar
//! (ETB / dies; self `~` or any [`crate::parsers::filter`] subject) lives here.

use crate::parsers::effect::ParsedEffect;
use crate::parsers::effect::{self};
use crate::parsers::filter;
#[cfg(test)]
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

/// A registry parser: a "When/Whenever <event>, <effect>." or "At the beginning
/// of <step> …, <effect>." line -> the bare `Triggered(...)` RON. Declines
/// (`Ok(None)`) on non-trigger lines or unrecognized events/effects.
/// Self-identifying by the trigger word, so the card's `CardKind` is
/// irrelevant.
///
/// Infallible today, but the `Result` is required by the `AbilityParser`
/// registry signature (sibling parsers render fallibly).
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    // "At the beginning of …" is a step-entry trigger whose event clause carries
    // an internal comma ("on your turn,"), so it can't share the "When/Whenever
    // … , …" split. Route it to a dedicated event parser that consumes the whole
    // "of … turn" run and returns the residual effect clause.
    if let Some(rest) = line.strip_prefix("At the beginning of ") {
        let Some((event, effect_clause)) = parse_beginning_of(rest) else {
            return Ok(None);
        };
        let Some(parsed) = effect::parse_clause(effect_clause, ctx) else {
            return Ok(None);
        };
        return Ok(Some(render(&event, &parsed)));
    }
    let Some(rest) = line
        .strip_prefix("When ")
        .or_else(|| line.strip_prefix("Whenever "))
    else {
        return Ok(None);
    };
    let Some((event_clause, effect_clause)) = rest.split_once(", ") else {
        return Ok(None);
    };
    let Some(event) = parse_event(event_clause) else {
        return Ok(None);
    };
    let Some(parsed) = effect::parse_clause(effect_clause, ctx) else {
        return Ok(None);
    };
    Ok(Some(render(&event, &parsed)))
}

/// Wraps an event + [`ParsedEffect`] in the `Triggered` frame, emitting
/// `targets:` only when the effect declares any.
fn render(event: &str, parsed: &ParsedEffect) -> String {
    if parsed.targets.is_empty() {
        format!("Triggered(event: {event}, effect: {})", parsed.effect)
    } else {
        format!(
            "Triggered(event: {event}, effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}

/// Parses a trigger's event clause (the text between the trigger word and the
/// comma) into the event RON, or `None`. v1 verbs: ETB, dies, attacks, and the
/// "you cast a[n] <subtype> spell" cast trigger. The state-transition verbs
/// (enters/dies/attacks) take a subject: self (`~`) uses the `This{Verb}`
/// shorthand macro; any other subject is parsed by the shared [`filter`]
/// grammar and applied to the event macro — so "a creature you control", "a
/// Goblin", etc. all resolve (declining when the filter grammar can't parse the
/// subject).
///
/// Shared with [`crate::parsers::replacement`]: an `Instead`/`Also`
/// replacement's `would:` is the same `Event`, parsed from the same
/// enters/dies clause grammar.
pub(super) fn parse_event(clause: &str) -> Option<String> {
    // Cast trigger: "you cast a[n] <subtype> spell" — the Performed("Cast") view
    // ([CR#601.2i]), filtered to a spell of the named subtype (Prowess shape).
    if let Some(event) = parse_cast_event(clause) {
        return Some(event);
    }
    // Tolerate the older "enters the battlefield" wording (current oracle:
    // "enters").
    let clause = clause.strip_suffix(" the battlefield").unwrap_or(clause);
    let (subject, verb) = if let Some(subject) = clause.strip_suffix(" enters") {
        (subject, "Enters")
    } else if let Some(subject) = clause.strip_suffix(" dies") {
        (subject, "Dies")
    } else if let Some(subject) = clause.strip_suffix(" attacks") {
        (subject, "Attacks")
    } else {
        return None;
    };
    if subject == "~" {
        Some(format!("This{verb}"))
    } else {
        Some(format!("{verb}({})", filter::parse_phrase(subject)?))
    }
}

/// "you cast a[n] <Subtype> spell" -> `Performed(verb: "Cast", by: Ref(You),
/// on: AllOf([Kind(Spell), Subtype("<X>")]))` ([CR#601.2i] cast view; mirrors
/// the Prowess macro's filtered-cast shape). Only the controller's own cast of
/// a single-subtype spell is modeled here; any other cast surface (an
/// opponent's cast, a card-type-filtered spell, no subtype) declines.
fn parse_cast_event(clause: &str) -> Option<String> {
    let rest = clause.strip_prefix("you cast ")?;
    let body = rest
        .strip_prefix("a ")
        .or_else(|| rest.strip_prefix("an "))?;
    let subtype = body.strip_suffix(" spell")?;
    // Exactly one descriptor word: an empty descriptor ("a spell") or a
    // multi-word one ("a creature or artifact spell") declines — this v1 form
    // models only the single-subtype cast. A lone non-subtype token (a card-type
    // word) still mints a `Subtype`, caught downstream by the catalog lint, as in
    // the shared filter grammar's bare-token head.
    if subtype.is_empty() || subtype.contains(' ') {
        return None;
    }
    Some(format!(
        "Performed(verb: \"Cast\", by: Ref(You), on: AllOf([Kind(Spell), Subtype(\"{}\")]))",
        crate::ident::to_rust_ident(subtype)
    ))
}

/// "At the beginning of <step> on <whose> turn, <effect>" (lead already
/// stripped) -> (`BeginningOf(<phase>, <whose>)`, effect clause), or `None`. v1
/// step: "combat" -> `Combat(BeginningOfCombat)`; v1 turn: "your" -> `Your`.
/// The "on … turn" run carries an internal comma, so the effect clause is split
/// off at the FIRST ", " after the turn phrase.
fn parse_beginning_of(rest: &str) -> Option<(String, &str)> {
    let (step_clause, effect_clause) = rest.split_once(", ")?;
    let step_clause = step_clause.strip_suffix(" turn")?;
    let (step, whose) = step_clause.split_once(" on ")?;
    let phase = match step {
        "combat" => "Combat(BeginningOfCombat)",
        _ => return None,
    };
    let whose_turn = match whose {
        "your" => "Your",
        _ => return None,
    };
    Some((format!("BeginningOf({phase}, {whose_turn})"), effect_clause))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trig(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    /// An ETB trigger whose effect is a keyword-action macro
    /// ("When ~ enters, investigate.") resolves through the macro-template
    /// fallthrough in the shared effect grammar.
    #[test]
    fn etb_keyword_action_macro_like_investigate() {
        let out = resolve_line(
            "When ~ enters, investigate.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            out.as_deref(),
            Some("Triggered(event: ThisEnters, effect: Investigate)")
        );
    }

    #[test]
    fn etb_self_draw() {
        assert_eq!(
            trig("When ~ enters, draw a card.").as_deref(),
            Some("Triggered(event: ThisEnters, effect: Draw(1))")
        );
    }

    #[test]
    fn dies_self_targeted_damage() {
        assert_eq!(
            trig("When ~ dies, it deals 1 damage to any target.").as_deref(),
            Some(
                "Triggered(event: ThisDies, effect: Targeted(targets: [AnyTarget], effect: DealDamage(Target(0), 1)))"
            )
        );
    }

    #[test]
    fn whenever_a_creature_dies_lose_life() {
        assert_eq!(
            trig("Whenever a creature dies, you lose 1 life.").as_deref(),
            Some("Triggered(event: Dies(Creature), effect: LoseLife(1))")
        );
    }

    #[test]
    fn whenever_a_creature_enters_draw() {
        assert_eq!(
            trig("Whenever a creature enters, draw a card.").as_deref(),
            Some("Triggered(event: Enters(Creature), effect: Draw(1))")
        );
    }

    #[test]
    fn etb_filtered_subject_via_filter_grammar() {
        // "a creature you control" — the filter parser supplies the subject.
        assert_eq!(
            trig("Whenever a creature you control enters, draw a card.").as_deref(),
            Some(
                "Triggered(event: Enters(AllOf([Creature, ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
        // A subtype subject — "a Goblin enters". The bare-subtype head carries
        // the battlefield scope ([CR#109.2]); harmless on an enters event (the
        // subject is entering the battlefield).
        assert_eq!(
            trig("Whenever a Goblin enters, draw a card.").as_deref(),
            Some(
                "Triggered(event: Enters(AllOf([Permanent, Subtype(\"Goblin\")])), effect: Draw(1))"
            )
        );
    }

    #[test]
    fn dies_another_you_control_aristocrats() {
        assert_eq!(
            trig("Whenever another creature you control dies, you lose 1 life.").as_deref(),
            Some(
                "Triggered(event: Dies(AllOf([Creature, Not(Ref(This)), ControlledBy(Ref(You))])), \
                 effect: LoseLife(1))"
            )
        );
    }

    #[test]
    fn impact_tremors_etb_payoff() {
        assert_eq!(
            trig("Whenever a creature you control enters, ~ deals 1 damage to each opponent.")
                .as_deref(),
            Some(
                "Triggered(event: Enters(AllOf([Creature, ControlledBy(Ref(You))])), \
                 effect: DealDamage(Filter(OpponentOf(Ref(You))), 1))"
            )
        );
    }

    #[test]
    fn tolerates_enters_the_battlefield_wording() {
        assert_eq!(
            trig("When ~ enters the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: ThisEnters, effect: Draw(1))")
        );
    }

    #[test]
    fn declines_non_triggers_unknown_events_and_effects() {
        // Not a trigger line.
        assert!(trig("Draw a card.").is_none());
        // Unknown event (cast trigger not in v1).
        assert!(trig("When you cast ~, draw a card.").is_none());
        // Unknown effect (exile isn't a production yet).
        assert!(trig("When ~ dies, exile target creature.").is_none());
        // Trigger word present but no ", " separator (no effect clause).
        assert!(trig("When ~ dies").is_none());
    }

    #[test]
    fn dies_destroy_target_via_effect_grammar() {
        assert_eq!(
            trig("When ~ dies, destroy target creature.").as_deref(),
            Some(
                "Triggered(event: ThisDies, effect: \
                 Targeted(targets: [TargetOne(Creature)], effect: Destroy(Target(0))))"
            )
        );
    }

    #[test]
    fn attacks_self_via_thisattacks_macro() {
        // Goblin Piledriver / Rabblemaster: "Whenever this creature attacks, …"
        // ("this creature" normalizes to ~). The self-pump scales by an
        // attacking-Goblin count (the "for each attacking <X>" effect piece).
        assert_eq!(
            trig("Whenever ~ attacks, it gets +1/+0 until end of turn for each other attacking Goblin.")
                .as_deref(),
            Some(
                "Triggered(event: ThisAttacks, effect: Continuously(effect: Modify(of: Of(This), \
                 changes: [AddPower(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), Not(Ref(This)), Attacking]))), \
                 AddToughness(0)]), duration: FixedUntil(EndOfTurn)))"
            )
        );
    }

    #[test]
    fn attacks_self_dwynen_lifegain_for_each_attacking() {
        // Dwynen, Gilt-Leaf Daen: "Whenever Dwynen attacks, you gain 1 life for
        // each attacking Elf you control." (name normalizes to ~).
        assert_eq!(
            trig("Whenever ~ attacks, you gain 1 life for each attacking Elf you control.")
                .as_deref(),
            Some(
                "Triggered(event: ThisAttacks, effect: GainLife(CountOf(AllOf([Permanent, Subtype(\"Elf\"), \
                 Attacking, ControlledBy(Ref(You))]))))"
            )
        );
    }

    #[test]
    fn attacks_filtered_subject() {
        // A non-self attacker subject parses via the shared filter grammar.
        assert_eq!(
            trig("Whenever a creature you control attacks, draw a card.").as_deref(),
            Some(
                "Triggered(event: Attacks(AllOf([Creature, ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_subtype_spell_with_may_rider() {
        // Lys Alana Huntmaster: "Whenever you cast an Elf spell, you may create a
        // 1/1 green Elf Warrior creature token." — a filtered-cast trigger
        // carrying a `you may` rider over a token-maker.
        assert_eq!(
            trig("Whenever you cast an Elf spell, you may create a 1/1 green Elf Warrior creature token.")
                .as_deref(),
            Some(
                "Triggered(event: Performed(verb: \"Cast\", by: Ref(You), \
                 on: AllOf([Kind(Spell), Subtype(\"Elf\")])), \
                 effect: May(effect: Create(1, Token(color_indicator: [Green], types: [Creature], \
                 subtypes: [Elf, Warrior], power: 1, toughness: 1))))"
            )
        );
    }

    #[test]
    fn cast_trigger_declines_out_of_scope() {
        // No subtype (a bare "a spell") -> declines (no single-subtype token).
        assert!(trig("Whenever you cast a spell, draw a card.").is_none());
        // A multi-word descriptor before "spell" -> declines (this v1 production
        // handles only a single subtype token).
        assert!(trig("Whenever you cast a creature or artifact spell, draw a card.").is_none());
        // An opponent's cast is not the controller-cast surface modeled here.
        assert!(trig("Whenever an opponent casts a spell, draw a card.").is_none());
    }

    #[test]
    fn beginning_of_combat_your_turn_create_token() {
        // Goblin Rabblemaster: "At the beginning of combat on your turn, create a
        // 1/1 red Goblin creature token with haste."
        assert_eq!(
            trig("At the beginning of combat on your turn, create a 1/1 red Goblin creature token with haste.")
                .as_deref(),
            Some(
                "Triggered(event: BeginningOf(Combat(BeginningOfCombat), Your), \
                 effect: Create(1, Token(color_indicator: [Red], types: [Creature], \
                 subtypes: [Goblin], abilities: [Keyword(Haste)], power: 1, toughness: 1)))"
            )
        );
    }

    #[test]
    fn beginning_of_declines_unmodeled_step_or_turn() {
        // An unmodeled step (upkeep) declines (v1 covers combat only).
        assert!(trig("At the beginning of your upkeep, draw a card.").is_none());
        // A non-"your" turn declines (v1 covers your-turn only).
        assert!(trig("At the beginning of combat on each player's turn, draw a card.").is_none());
    }
}
