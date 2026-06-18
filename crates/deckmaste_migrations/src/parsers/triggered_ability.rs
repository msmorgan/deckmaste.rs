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
    // Strip a leading ability-word label ("Landfall — …", "Threshold — …").
    // Ability words have NO rules meaning ([CR#207.2c]) — they're a flavor tag
    // on the trigger that follows — so the ability underneath is what we parse.
    let line = strip_ability_word(line);
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

/// Strip a leading ability-word label ("Landfall — ", "Pack Tactics — ").
/// Ability words are reminder flavor with no rules weight ([CR#207.2c]); the
/// label is the Title-Case run before the spaced em-dash, and the ability that
/// carries weight is whatever follows. Returns the suffix when the line opens
/// with such a label AND continues with a trigger word ("When"/"Whenever"/"At")
/// — that follow-on word is the structural signal an ability is underneath,
/// keeping a mid-sentence em-dash (a cost em-dash, a "choose one —" header)
/// from being mistaken for an ability-word break. Otherwise returns the line
/// unchanged.
fn strip_ability_word(line: &str) -> &str {
    let Some((label, rest)) = line.split_once(" — ") else {
        return line;
    };
    // A bare label: a short Title-Case run, no sentence punctuation (a real
    // effect clause before the em-dash would carry a comma/period/colon).
    let bare_label = !label.is_empty()
        && label.len() <= 24
        && label.chars().next().is_some_and(|c| c.is_ascii_uppercase())
        && !label.contains([',', '.', ':', ';', '"']);
    let trigger_follows =
        rest.starts_with("When ") || rest.starts_with("Whenever ") || rest.starts_with("At ");
    if bare_label && trigger_follows { rest } else { line }
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
    // Becomes-target trigger: "<subject> becomes the target of a spell or
    // ability" — the `BecomesTarget` event ([CR#601.2c] announce-time; ward is
    // the family exemplar). "a spell or ability" carries no controller
    // restriction, so `by` is omitted (matches any targeting source); a narrowed
    // "… an opponent controls" rider is the Ward shape, not this bare form, and
    // declines here.
    if let Some(subject) = clause.strip_suffix(" becomes the target of a spell or ability") {
        return Some(if subject == "~" {
            "BecomesTarget(what: Ref(This))".to_owned()
        } else {
            format!("BecomesTarget(what: {})", filter::parse_phrase(subject)?)
        });
    }
    // "dies" also spells out as "is put into a graveyard from the battlefield"
    // ([CR#700.4]: the long form IS the definition of dies) — fold it onto the
    // same Dies event up front so the suffix match below routes it through the
    // Dies macro.
    if let Some(subject) = clause.strip_suffix(" is put into a graveyard from the battlefield") {
        return Some(if subject == "~" {
            "ThisDies".to_owned()
        } else {
            format!("Dies({})", filter::parse_phrase(subject)?)
        });
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

/// "At the beginning of <step-phrase>, <effect>" (lead already stripped) ->
/// (`BeginningOf(<phase>, <whose>)`, effect clause), or `None`. Two step-phrase
/// shapes ([CR#603.3]):
///
/// - A possessive step naming whose turn it watches: "your upkeep" / "your end
///   step" -> `(<phase>, Your)`. The whose-turn is the leading possessive; the
///   phase is the remaining step words.
/// - A "the"-led step with no possessive: "the end step" / "the upkeep" — fires
///   on EVERY player's turn of that step ([CR#513.1a] end-step triggers; the
///   bare "the" carries no turn restriction), so the whose-turn is
///   `EachPlayers`.
/// - The combat-only "<step> on <whose> turn" form ("combat on your turn")
///   whose "on … turn" run carries an INTERNAL comma — kept for the existing
///   beginning-of-combat trigger.
///
/// The phrase->`Phase` map covers the steps cards trigger on today (upkeep, end
/// step, beginning of combat); an unmodeled step or a non-"your" possessive
/// declines.
fn parse_beginning_of(rest: &str) -> Option<(String, &str)> {
    let (step_clause, effect_clause) = rest.split_once(", ")?;
    // The combat form carries an internal "on <whose> turn" run.
    if let Some(combat_step) = step_clause.strip_suffix(" turn") {
        let (step, whose) = combat_step.split_once(" on ")?;
        let phase = step_phase(step)?;
        let whose_turn = match whose {
            "your" => "Your",
            _ => return None,
        };
        return Some((format!("BeginningOf({phase}, {whose_turn})"), effect_clause));
    }
    // The plain "<possessive> <step>" / "the <step>" forms.
    let (whose_turn, step) = if let Some(step) = step_clause.strip_prefix("your ") {
        ("Your", step)
    } else if let Some(step) = step_clause.strip_prefix("the ") {
        // No possessive -> every player's turn of that step.
        ("EachPlayers", step)
    } else {
        return None;
    };
    let phase = step_phase(step)?;
    Some((format!("BeginningOf({phase}, {whose_turn})"), effect_clause))
}

/// A step phrase -> its `Phase` RON, or `None` for an unmodeled step. Covers
/// the steps that carry "at the beginning of" triggers today
/// ([CR#502,503,513]).
fn step_phase(step: &str) -> Option<&'static str> {
    Some(match step {
        "upkeep" => "Beginning(Upkeep)",
        "draw step" => "Beginning(Draw)",
        "end step" => "Ending(End)",
        "combat" => "Combat(BeginningOfCombat)",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trig(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    /// Like [`trig`] but over the BUILTIN macro index — needed by effect bodies
    /// that route through a macro template (`+1/+1 counter` -> `P1P1Counter`).
    fn trig_builtin(line: &str) -> Option<String> {
        resolve_line(
            line,
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap()
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
        // An unmodeled step declines.
        assert!(trig("At the beginning of the cleanup step, draw a card.").is_none());
        // A non-"your" turn declines for the combat form (v1 covers your-turn only).
        assert!(trig("At the beginning of combat on each player's turn, draw a card.").is_none());
    }

    #[test]
    fn beginning_of_your_upkeep_sacrifice_unless_pay() {
        // Cumulative-style upkeep toll: "sacrifice ~ unless you pay {M}{M}" =>
        // the same `BeginningOf(Beginning(Upkeep), Your)` + `Unless` shape the
        // kw-echo macro emits.
        assert_eq!(
            trig("At the beginning of your upkeep, sacrifice ~ unless you pay {G}{G}.").as_deref(),
            Some(
                "Triggered(event: BeginningOf(Beginning(Upkeep), Your), \
                 effect: Unless(effect: Sacrifice(This), unless: [Mana([Green,Green])]))"
            )
        );
    }

    #[test]
    fn beginning_of_end_step_sacrifice_each_players() {
        // "the end step" (no possessive) fires every turn => EachPlayers.
        assert_eq!(
            trig("At the beginning of the end step, sacrifice ~.").as_deref(),
            Some(
                "Triggered(event: BeginningOf(Ending(End), EachPlayers), \
                 effect: Sacrifice(This))"
            )
        );
    }

    #[test]
    fn becomes_target_self_sacrifice() {
        // Illusion-family drawback: "When ~ becomes the target of a spell or
        // ability, sacrifice it." => BecomesTarget(what: Ref(This)) + Sacrifice.
        assert_eq!(
            trig("When ~ becomes the target of a spell or ability, sacrifice it.").as_deref(),
            Some("Triggered(event: BecomesTarget(what: Ref(This)), effect: Sacrifice(This))")
        );
    }

    #[test]
    fn dies_long_form_put_into_graveyard_return_to_hand() {
        // "is put into a graveyard from the battlefield" IS the definition of
        // "dies" ([CR#700.4]) — same ThisDies event.
        assert_eq!(
            trig("When ~ is put into a graveyard from the battlefield, return it to its owner's hand.")
                .as_deref(),
            Some("Triggered(event: ThisDies, effect: ReturnToHand(This))")
        );
        assert_eq!(
            trig("When ~ is put into a graveyard from the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: ThisDies, effect: Draw(1))")
        );
    }

    #[test]
    fn dies_long_form_filtered_subject() {
        // A non-self subject routes through the Dies macro over the filter.
        assert_eq!(
            trig("Whenever another creature you control is put into a graveyard from the battlefield, draw a card.")
                .as_deref(),
            Some(
                "Triggered(event: Dies(AllOf([Creature, Not(Ref(This)), ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn landfall_ability_word_stripped_then_pump() {
        // "Landfall — " is a flavor label ([CR#207.2c]); the land-ETB trigger +
        // self-pump underneath is what parses.
        assert_eq!(
            trig("Landfall — Whenever a land you control enters, ~ gets +2/+2 until end of turn.")
                .as_deref(),
            Some(
                "Triggered(event: Enters(AllOf([Type(Land), ControlledBy(Ref(You))])), \
                 effect: Continuously(effect: Modify(of: Of(This), \
                 changes: [AddPowerToughness(2, 2)]), duration: FixedUntil(EndOfTurn)))"
            )
        );
    }

    #[test]
    fn landfall_ability_word_put_counter() {
        assert_eq!(
            trig_builtin(
                "Landfall — Whenever a land you control enters, put a +1/+1 counter on ~."
            )
            .as_deref(),
            Some(
                "Triggered(event: Enters(AllOf([Type(Land), ControlledBy(Ref(You))])), \
                 effect: PutCounters(This, P1P1Counter, 1))"
            )
        );
    }

    #[test]
    fn etb_attach_to_target_creature() {
        // Self-equipping artifact creature: "When ~ enters, attach it to target
        // creature you control."
        assert_eq!(
            trig("When ~ enters, attach it to target creature you control.").as_deref(),
            Some(
                "Triggered(event: ThisEnters, effect: Targeted(targets: \
                 [TargetOne(AllOf([Creature, ControlledBy(Ref(You))]))], \
                 effect: Attach(what: This, to: Target(0))))"
            )
        );
    }

    #[test]
    fn ability_word_strip_does_not_eat_mid_sentence_em_dash() {
        // A real effect line that happens to carry an em-dash but no
        // ability-word label (no trigger word right after) is left intact, so
        // the leading "Choose one —" style header isn't mistaken for a label.
        assert_eq!(
            super::strip_ability_word("Threshold — Whenever ~ attacks, draw a card."),
            "Whenever ~ attacks, draw a card."
        );
        // No trigger word after the dash => left whole.
        assert_eq!(
            super::strip_ability_word("Choose one — draw a card."),
            "Choose one — draw a card."
        );
    }

    #[test]
    fn becomes_target_declines_ward_shape() {
        // The "an opponent controls"-narrowed form is the Ward shape, not this
        // bare production; it has no plain becomes-target match and declines
        // (the rider stays unparsed in the residual subject).
        assert!(
            trig("When ~ becomes the target of a spell or ability an opponent controls, sacrifice it.")
                .is_none()
        );
    }
}
