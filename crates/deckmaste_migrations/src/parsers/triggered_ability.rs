//! The `Triggered` frame parser: a "When/Whenever <event>, <effect>." line ->
//! the bare `Triggered(...)` ability RON. A triggered ability is written as
//! "[trigger condition], [effect]" divided by the trigger word [CR#603.1]. The
//! effect grammar is shared via [`crate::parsers::effect`]; the event grammar
//! (ETB / dies / leaves-the-battlefield; self `~` or any
//! [`crate::parsers::filter`] subject) lives here.

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
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    // Split off a leading ability-word label ("Landfall — …",
    // "Threshold — …"). Ability words have NO rules meaning ([CR#207.2c]) —
    // the ability underneath is what we parse — but the label is RENDER
    // metadata the emitted ability keeps (`ability_word:`), so the render
    // side prints the oracle line back.
    let (ability_word, line) = split_ability_word(line);
    // "At the beginning of …" is a step-entry trigger whose event clause carries
    // an internal comma ("on your turn,"), so it can't share the "When/Whenever
    // … , …" split. Route it to a dedicated event parser that consumes the whole
    // "of … turn" run and returns the residual effect clause.
    if let Some(rest) = line.strip_prefix("At the beginning of ") {
        let Some((event, effect_clause)) = parse_beginning_of(rest) else {
            return Ok(None);
        };
        let Some(parsed) = effect::parse_clause(effect_clause, ctx)? else {
            return Ok(None);
        };
        return Ok(Some(render(ability_word, &event, &parsed)));
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
    let Some(parsed) = effect::parse_clause(effect_clause, ctx)? else {
        return Ok(None);
    };
    Ok(Some(render(ability_word, &event, &parsed)))
}

/// Split a leading ability-word label ("Landfall — ", "Pack Tactics — ")
/// off the line. Ability words are reminder flavor with no rules weight
/// ([CR#207.2c]); the label is the Title-Case run before the spaced
/// em-dash, and the ability that carries weight is whatever follows.
/// Returns `(Some(label), suffix)` when the line opens with such a label
/// AND continues with a trigger word ("When"/"Whenever"/"At") — that
/// follow-on word is the structural signal an ability is underneath,
/// keeping a mid-sentence em-dash (a cost em-dash, a "choose one —" header)
/// from being mistaken for an ability-word break. Otherwise
/// `(None, line)`.
fn split_ability_word(line: &str) -> (Option<&str>, &str) {
    let Some((label, rest)) = line.split_once(" — ") else {
        return (None, line);
    };
    // A bare label: a short Title-Case run, no sentence punctuation (a real
    // effect clause before the em-dash would carry a comma/period/colon).
    let bare_label = !label.is_empty()
        && label.len() <= 24
        && label.chars().next().is_some_and(|c| c.is_ascii_uppercase())
        && !label.contains([',', '.', ':', ';', '"']);
    let trigger_follows =
        rest.starts_with("When ") || rest.starts_with("Whenever ") || rest.starts_with("At ");
    if bare_label && trigger_follows { (Some(label), rest) } else { (None, line) }
}

/// Wraps an event + [`ParsedEffect`] in the `Triggered` frame, emitting
/// `targets:` only when the effect declares any.
///
/// A TRIGGERED targeted body can't lean on the bare `It` fallback the
/// spell/activated parsers use: several trigger events push their OWN role
/// antecedents onto the resolving frame alongside the announced target — a
/// dies/enters `ZoneChanged`, an attacks `Attacking`, a becomes-tapped
/// `Tapped` all bind `anaphora.that_object` on the very frame that also
/// carries `anaphora.targets` ([CR#608.2d]; see `resolve.rs`'s
/// `StackObject::Triggered` frame build). The engine's `Reference::It`
/// read refuses to guess between an event role and the announced target
/// when both are bound, so it degrades to the null id — a silent fizzle,
/// not a load error. Every production in this parser emits at most one
/// target, so the body's `It` reads are rewritten to the positional
/// `Target(0)` ([`target_slot_reads`]), never left bare.
fn render(ability_word: Option<&str>, event: &str, parsed: &ParsedEffect) -> String {
    // [CR#207.2c]: the stripped ability-word label rides as render metadata.
    let word = ability_word.map_or_else(String::new, |w| format!("ability_word: \"{w}\", "));
    if parsed.targets.is_empty() {
        format!("Triggered({word}event: {event}, effect: {})", parsed.effect)
    } else {
        format!(
            "Triggered({word}event: {event}, effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            target_slot_reads(&parsed.effect),
        )
    }
}

/// Rewrites the effect body's slot-anaphor reads (`It` — emitted only as the
/// announced-slot read in targeted bodies; loop binders introduce their own
/// `It` only in untargeted productions) to the positional read `Target(0)`
/// ([CR#115.3,601.2c]). Every production this parser's effect grammar reaches
/// emits at most one target, so index 0 is always the right (and only)
/// slot. Token-exact (ASCII word boundaries — RON identifiers), so
/// identifiers merely containing "It" are never touched.
fn target_slot_reads(body: &str) -> String {
    static IT_READ: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"\bIt\b").unwrap());
    IT_READ.replace_all(body, "Target(0)").into_owned()
}

/// Parses a trigger's event clause (the text between the trigger word and the
/// comma) into the event RON, or `None`. v1 verbs: ETB, dies, leaves the
/// battlefield, attacks, becomes tapped/untapped, is dealt damage, gains
/// life, draws, and the "you cast X" cast trigger family
/// ([`parse_cast_event`]). The state-transition verbs (enters/dies/leaves the
/// battlefield/attacks/becomes tapped/becomes untapped) take an OBJECT
/// subject: self (`~`) uses the `This{Verb}` shorthand macro; any other
/// subject is parsed by the shared [`filter`] grammar and applied to the
/// event macro — so "a creature you control", "a Goblin", etc. all resolve
/// (declining when the filter grammar can't parse the subject). The
/// damage/gain-life/draw verbs instead take a PLAYER-identity (damage
/// recipient) or player (gain-life/draw) subject, parsed by
/// [`filter::recipient_phrase`] — no `~` shorthand, since a permanent is
/// never "you"/"an opponent"/"a player".
///
/// Shared with [`crate::parsers::replacement`]: an `Instead`/`Also`
/// replacement's `would:` is the same `EventFilter`, parsed from the same
/// enters/dies clause grammar.
pub(super) fn parse_event(clause: &str) -> Option<String> {
    // Cast trigger: "you cast X" — the `Cast` onset event ([CR#601.2i]),
    // filtered per [`parse_cast_event`]'s recognized shapes (self, bare
    // spell, card-type/noncreature/instant-or-sorcery, single subtype).
    if let Some(event) = parse_cast_event(clause) {
        return Some(event);
    }
    // Combat-damage trigger: "<subject> deals combat damage to <recipient>" —
    // combat damage dealt ([CR#510.2]) to a recipient, the `DealsCombatDamage`
    // event macro. Its own mid-clause verb phrase (matched by
    // `split_once`, not a suffix), so it can't be shadowed by the generic
    // enters/dies/attacks suffix matcher below. Two-slot event (source +
    // recipient) — unlike those single-slot events, `~` fills only the
    // SOURCE slot with `Ref(This)`, not a nullary `This<Verb>` shorthand
    // (there is no single verb-only macro shape with two slots). The
    // recipient is parsed by [`filter::recipient_phrase`] (player/opponent/
    // you forms `parse_phrase` doesn't reach); either side declining
    // declines the whole clause.
    if let Some((subject, recipient)) = clause.split_once(" deals combat damage to ") {
        let source = if subject == "~" {
            "Ref(This)".to_owned()
        } else {
            filter::parse_phrase(subject)?
        };
        return Some(format!(
            "DealsCombatDamage({source}, {})",
            filter::recipient_phrase(recipient)?
        ));
    }
    // Damage-dealt trigger (generic, non-combat-narrowed): "<subject> is
    // dealt damage" — damage dealt ([CR#120.1]), passive-voice
    // recipient-subject phrasing, the `DealtDamage` event macro. Distinct
    // from the "deals combat damage to" arm above: "is dealt COMBAT
    // damage" is never a suffix match here (the word "combat" sits between
    // "dealt" and "damage", so the literal " is dealt damage" tail never
    // appears in that clause) — neither arm can shadow the other, so "is
    // dealt combat damage" clauses fall through this arm undisturbed (left
    // unhandled, same as before this wave). The recipient draws from the
    // same player-identity/object grammar as the combat-damage recipient
    // ([`filter::recipient_phrase`]).
    if let Some(subject) = clause.strip_suffix(" is dealt damage") {
        let to = if subject == "~" {
            "Ref(This)".to_owned()
        } else {
            filter::recipient_phrase(subject)?
        };
        return Some(format!("DealtDamage({to})"));
    }
    // Two-slot block trigger, active voice: "~ blocks a creature"
    // ([CR#509.3b]: "Whenever [a creature] blocks a creature, …") — BOTH
    // sides of the one block fact narrowed at once (unlike the bare
    // "blocks"/"becomes blocked" suffix arms below, which narrow only one
    // side), via the two-param `Blocking` event macro. Placed ahead of the
    // generic suffix chain since it matches a mid-clause verb phrase
    // (split_once), not a suffix. RESTRICTED to the self subject `~`: the
    // render side narrows a two-sided block only when one side is `Ref(This)`,
    // so a filtered non-self subject ("a creature you control blocks a
    // creature") has no faithful two-sided render arm and would silently drop
    // the `of` narrowing (or, passive, invert the sentence) — it DECLINES here
    // rather than emit an unrenderable filter, alongside the filtered-object
    // and disjunction ("blocks or becomes blocked") forms already deferred.
    // Only the plain "a creature" object is supported (a further-filtered
    // object would likewise drop its modifier on render). The disjunction
    // falls through undisturbed (its object residue is never the literal "a
    // creature", and its subject "~ blocks or" is never exactly "~").
    if let Some((subject, object)) = clause.split_once(" blocks ")
        && object == "a creature"
        && subject == "~"
    {
        return Some("Blocking(Ref(This), Creature)".to_owned());
    }
    // Two-slot block trigger, passive voice: "~ becomes blocked by a creature"
    // ([CR#509.3d]: "Whenever [a creature] becomes blocked by a creature, …") —
    // the mirror image of the arm above (the blocked side is the clause subject
    // here, the blocker is named after "by"). Same self-subject-only, plain-"a
    // creature"-object restriction, same reasoning.
    if let Some((subject, object)) = clause.split_once(" becomes blocked by ")
        && object == "a creature"
        && subject == "~"
    {
        return Some("Blocking(Creature, Ref(This))".to_owned());
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
    // Gain-life trigger: "you gain life" / "an opponent gains life" — a
    // player's own life gain ([CR#119.3]), the `GainsLife` event macro. The
    // subject is a PLAYER identity (parsed by
    // [`filter::recipient_phrase`], the same "you"/"an opponent"/"a
    // player" grammar the damage recipient above uses) — there's no `~`
    // shorthand, since a permanent is never "you". Both the 2nd-person
    // ("you gain") and 3rd-person-singular ("an opponent gains") verb
    // agreements appear in real oracle text.
    if let Some(subject) = clause
        .strip_suffix(" gain life")
        .or_else(|| clause.strip_suffix(" gains life"))
    {
        return Some(format!("GainsLife({})", filter::recipient_phrase(subject)?));
    }
    // Draw trigger: "you draw a card" / "an opponent draws a card" — a
    // player's own draw ([CR#121.1]), the `Draws` event macro (mirrors
    // `GainsLife`'s shape; mind the underlying event — the macro body
    // expands to `Drawn`, not `Draws`).
    if let Some(subject) = clause
        .strip_suffix(" draw a card")
        .or_else(|| clause.strip_suffix(" draws a card"))
    {
        return Some(format!("Draws({})", filter::recipient_phrase(subject)?));
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
    } else if let Some(subject) = clause.strip_suffix(" leaves") {
        // Leaves-the-battlefield trigger ([CR#603.6c]): "When [this object]
        // leaves the battlefield, ..." — the GENERAL zone-change-from-
        // battlefield event (any destination, `to` omitted), of which `Dies`
        // (to the graveyard specifically) is the narrowed sibling. Distinct
        // from the `" dies"` arm above by construction: the two suffixes
        // ("dies" vs "leaves") never overlap on the same clause, so a
        // "dies" clause is never mis-routed here and vice versa (checked
        // ahead of this arm anyway, so "dies" always wins first).
        (subject, "LeavesBattlefield")
    } else if let Some(subject) = clause.strip_suffix(" attacks") {
        (subject, "Attacks")
    } else if let Some(subject) = clause.strip_suffix(" blocks") {
        // Bare self-blocks ([CR#509.3a]: "Whenever [a creature] blocks, …" —
        // triggers once per combat even if it blocks multiple creatures).
        (subject, "Blocks")
    } else if let Some(subject) = clause
        .strip_suffix(" becomes blocked")
        .or_else(|| clause.strip_suffix(" is blocked"))
    {
        // Bare self-becomes-blocked ([CR#509.3c]; "is blocked" is the older
        // oracle-text synonym for the same "becomes blocked" state,
        // [CR#509.1h]).
        (subject, "BecomesBlocked")
    } else if let Some(subject) = clause.strip_suffix(" becomes tapped") {
        (subject, "BecomesTapped")
    } else {
        let subject = clause.strip_suffix(" becomes untapped")?;
        (subject, "BecomesUntapped")
    };
    if subject == "~" {
        Some(format!("This{verb}"))
    } else {
        Some(format!("{verb}({})", filter::parse_phrase(subject)?))
    }
}

/// "you cast X" -> `Cast(who: Ref(You), what: <filter>)` ([CR#601.2i] cast
/// onset; mirrors the Prowess/Cascade macros' filtered-cast shape). Only the
/// controller's own cast is modeled here — an opponent's cast declines
/// (leaves `clause` unconsumed by the `"you cast "` prefix). The recognized
/// `what:` shapes, in order:
///
/// - self ("you cast ~") -> `Ref(This)` (Cascade's own "you cast this spell"
///   reminder-text shape).
/// - a bare spell ("you cast a spell") -> `Kind(Spell)`.
/// - "an instant or sorcery spell" -> `And([Kind(Spell), Or([Type(Instant),
///   Type(Sorcery)])])` (the two-type disjunction; NOT the general
///   heterogeneous-type-disjunction grammar, since only this one pairing is
///   modeled here).
/// - "a noncreature spell" -> `And([Kind(Spell), Not(Type(Creature))])`.
/// - "a[n] <card-type> spell" (creature/artifact/enchantment/instant/ sorcery)
///   -> `And([Kind(Spell), Type(<T>)])`.
/// - "a[n] <Subtype> spell" (the original v1 shape, e.g. "an Elf spell") ->
///   `And([Kind(Spell), Subtype("<X>")])`. A lone non-subtype token still mints
///   a `Subtype`, caught downstream by the catalog lint, as in the shared
///   filter grammar's bare-token head.
///
/// Any other multi-word descriptor (restriction-laden forms: "your first
/// spell each turn", "a spell that targets ~", color/mana-value/historic
/// filters, …) declines, left for the debug/unparsed path — these need extra
/// filter machinery or trigger-condition modeling this v1 production doesn't
/// carry.
fn parse_cast_event(clause: &str) -> Option<String> {
    let rest = clause.strip_prefix("you cast ")?;
    if rest == "~" {
        return Some("Cast(who: Ref(You), what: Ref(This))".to_owned());
    }
    let body = rest
        .strip_prefix("a ")
        .or_else(|| rest.strip_prefix("an "))?;
    if body == "spell" {
        return Some("Cast(who: Ref(You), what: Kind(Spell))".to_owned());
    }
    let descriptor = body.strip_suffix(" spell")?;
    if descriptor.is_empty() {
        return None;
    }
    if descriptor == "instant or sorcery" {
        return Some(
            "Cast(who: Ref(You), what: And([Kind(Spell), \
             Or([Type(\"Instant\"), Type(\"Sorcery\")])]))"
                .to_owned(),
        );
    }
    if descriptor == "noncreature" {
        return Some(
            "Cast(who: Ref(You), what: And([Kind(Spell), Not(Type(\"Creature\"))]))".to_owned(),
        );
    }
    // Any other multi-word descriptor is out of scope for this production
    // (restriction-laden forms deferred).
    if descriptor.contains(' ') {
        return None;
    }
    // A single-word descriptor: a card-type noun first (Type(<T>)), else the
    // original v1 subtype fallback (unconditional, like the shared filter
    // grammar's bare-token head).
    let atom = filter::type_filter(&filter::singularize(descriptor).to_ascii_lowercase())
        .unwrap_or_else(|| format!("Subtype(\"{}\")", crate::ident::to_rust_ident(descriptor)));
    Some(format!(
        "Cast(who: Ref(You), what: And([Kind(Spell), {atom}]))"
    ))
}

/// "At the beginning of <step-phrase>, <effect>" (lead already stripped) ->
/// (`StepBegins(at: <phase>, whose: <whose>)`, effect clause), or `None`. Two
/// step-phrase shapes ([CR#603.3]):
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
/// The phrase->`PhaseStep` map covers the steps cards trigger on today (upkeep,
/// end step, beginning of combat); an unmodeled step or a non-"your" possessive
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
        return Some((
            format!("StepBegins(at: {phase}, whose: {whose_turn})"),
            effect_clause,
        ));
    }
    // The plain "<possessive> <step>" / "the <step>" forms.
    let (whose_turn, step) = if let Some(step) = step_clause.strip_prefix("your ") {
        ("Your", step)
    } else {
        let step = step_clause.strip_prefix("the ")?;
        // No possessive -> every player's turn of that step.
        ("EachPlayers", step)
    };
    let phase = step_phase(step)?;
    Some((
        format!("StepBegins(at: {phase}, whose: {whose_turn})"),
        effect_clause,
    ))
}

/// A step phrase -> its `PhaseStep` RON, or `None` for an unmodeled step.
/// Covers the steps that carry "at the beginning of" triggers today
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
                "Triggered(event: ThisDies, effect: Targeted(targets: [AnyTarget], effect: DealDamage(This, 1, Target(0))))"
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
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn etb_filtered_subject_via_filter_grammar() {
        // "a creature you control" — the filter parser supplies the subject.
        assert_eq!(
            trig("Whenever a creature you control enters, draw a card.").as_deref(),
            Some(
                "Triggered(event: Enters(And([Creature, ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
        // A subtype subject — "a Goblin enters". The bare-subtype head carries
        // the battlefield scope ([CR#109.2]); harmless on an enters event (the
        // subject is entering the battlefield).
        assert_eq!(
            trig("Whenever a Goblin enters, draw a card.").as_deref(),
            Some(
                "Triggered(event: Enters(And([Permanent, Subtype(\"Goblin\")])), effect: Draw(1))"
            )
        );
    }

    #[test]
    fn dies_another_you_control_aristocrats() {
        assert_eq!(
            trig("Whenever another creature you control dies, you lose 1 life.").as_deref(),
            Some(
                "Triggered(event: Dies(And([Creature, Not(Ref(This)), ControlledBy(Ref(You))])), \
                 effect: LoseLife(1))"
            )
        );
    }

    #[test]
    fn leaves_battlefield_self_via_thisleavesbattlefield_macro() {
        // "When ~ leaves the battlefield, ..." ([CR#603.6c]) — the GENERAL
        // zone-change-from-battlefield event (any destination; `to` omitted),
        // via the `ThisLeavesBattlefield` shorthand macro (mirrors `ThisDies`).
        assert_eq!(
            trig("When ~ leaves the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: ThisLeavesBattlefield, effect: Draw(1))")
        );
    }

    #[test]
    fn leaves_battlefield_filtered_subject() {
        // "Whenever a creature leaves the battlefield, ..." — a non-`~`
        // subject parses via the shared filter grammar, same as the
        // enters/dies/attacks events, over the `LeavesBattlefield` macro.
        assert_eq!(
            trig("Whenever a creature leaves the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: LeavesBattlefield(Creature), effect: Draw(1))")
        );
    }

    #[test]
    fn leaves_battlefield_another_you_control() {
        // "Whenever another creature you control leaves the battlefield, ..."
        // — mirrors `dies_another_you_control_aristocrats`: the same
        // `Not(Ref(This))` + `ControlledBy(Ref(You))` filter, over
        // `LeavesBattlefield` instead of `Dies`.
        assert_eq!(
            trig("Whenever another creature you control leaves the battlefield, you lose 1 life.")
                .as_deref(),
            Some(
                "Triggered(event: LeavesBattlefield(And([Creature, Not(Ref(This)), \
                 ControlledBy(Ref(You))])), effect: LoseLife(1))"
            )
        );
    }

    #[test]
    fn leaves_battlefield_does_not_shadow_dies_and_vice_versa() {
        // The "dies" and "leaves the battlefield" arms match disjoint
        // clause suffixes ("dies" vs "leaves"), so neither can shadow the
        // other — "dies" always resolves to the graveyard-specific `Dies`
        // event, never the general `LeavesBattlefield` event, and vice versa.
        assert_eq!(
            trig("When ~ dies, draw a card.").as_deref(),
            Some("Triggered(event: ThisDies, effect: Draw(1))")
        );
        assert_eq!(
            trig("When ~ leaves the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: ThisLeavesBattlefield, effect: Draw(1))")
        );
    }

    #[test]
    fn impact_tremors_etb_payoff() {
        assert_eq!(
            trig("Whenever a creature you control enters, ~ deals 1 damage to each opponent.")
                .as_deref(),
            Some(
                "Triggered(event: Enters(And([Creature, ControlledBy(Ref(You))])), \
                 effect: DealsDamageToEach(1, OpponentOf(Ref(You))))"
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
        // Unknown effect declines.
        assert!(trig("When ~ dies, manifest the top card of your library.").is_none());
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
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn attacks_self_via_thisattacks_macro() {
        // Goblin Piledriver / Rabblemaster: "Whenever this creature attacks, …"
        // ("this creature" normalizes to ~). The self-pump scales by an
        // attacking-Goblin count (the "for each attacking <X>" effect piece).
        assert_eq!(
            trig("Whenever ~ attacks, it gets +1/+0 until end of turn for each other attacking Goblin.")
                .as_deref(),
            Some(
                "Triggered(event: ThisAttacks, effect: Continuously(effect: Modify(This, \
                 Several([Power(Up(CountOf(Objects(And([Permanent, Subtype(\"Goblin\"), Not(Ref(This)), Attacking]))))), \
                 Toughness(Up(0))])), duration: FixedUntil(EndOfTurn)))"
            )
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn attacks_self_dwynen_lifegain_for_each_attacking() {
        // Dwynen, Gilt-Leaf Daen: "Whenever Dwynen attacks, you gain 1 life for
        // each attacking Elf you control." (name normalizes to ~).
        assert_eq!(
            trig("Whenever ~ attacks, you gain 1 life for each attacking Elf you control.")
                .as_deref(),
            Some(
                "Triggered(event: ThisAttacks, effect: GainLife(CountOf(Objects(And([Permanent, Subtype(\"Elf\"), \
                 Attacking, ControlledBy(Ref(You))])))))"
            )
        );
    }

    #[test]
    fn attacks_filtered_subject() {
        // A non-self attacker subject parses via the shared filter grammar.
        assert_eq!(
            trig("Whenever a creature you control attacks, draw a card.").as_deref(),
            Some(
                "Triggered(event: Attacks(And([Creature, ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn blocks_self_via_thisblocks_macro() {
        // Bare self-blocks ([CR#509.3a]).
        assert_eq!(
            trig("Whenever ~ blocks, draw a card.").as_deref(),
            Some("Triggered(event: ThisBlocks, effect: Draw(1))")
        );
    }

    #[test]
    fn blocks_filtered_subject() {
        // A non-self blocker subject parses via the shared filter grammar
        // (mirrors attacks_filtered_subject).
        assert_eq!(
            trig("Whenever a creature you control blocks, draw a card.").as_deref(),
            Some(
                "Triggered(event: Blocks(And([Creature, ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn becomes_blocked_self_via_thisbecomesblocked_macro() {
        // Bare self-becomes-blocked ([CR#509.3c]).
        assert_eq!(
            trig("Whenever ~ becomes blocked, draw a card.").as_deref(),
            Some("Triggered(event: ThisBecomesBlocked, effect: Draw(1))")
        );
    }

    #[test]
    fn becomes_blocked_tolerates_is_blocked_wording() {
        // "is blocked" is the older oracle-text synonym for the same
        // "becomes blocked" state ([CR#509.1h]).
        assert_eq!(
            trig("Whenever ~ is blocked, draw a card.").as_deref(),
            Some("Triggered(event: ThisBecomesBlocked, effect: Draw(1))")
        );
    }

    #[test]
    fn blocks_a_creature_two_slot() {
        // "Whenever [a creature] blocks a creature, …" ([CR#509.3b]) — both
        // sides of the one block fact narrowed at once, via the two-param
        // `Blocking` event macro.
        assert_eq!(
            trig("Whenever ~ blocks a creature, draw a card.").as_deref(),
            Some("Triggered(event: Blocking(Ref(This), Creature), effect: Draw(1))")
        );
    }

    #[test]
    fn becomes_blocked_by_a_creature_two_slot() {
        // "Whenever [a creature] becomes blocked by a creature, …"
        // ([CR#509.3d]) — the passive-voice mirror of the arm above.
        assert_eq!(
            trig("Whenever ~ becomes blocked by a creature, draw a card.").as_deref(),
            Some("Triggered(event: Blocking(Creature, Ref(This)), effect: Draw(1))")
        );
    }

    #[test]
    fn two_slot_block_filtered_non_self_subject_declines() {
        // The two-slot forms are RESTRICTED to the self subject `~`: the render
        // side narrows a two-sided block only when one side is `Ref(This)`, so a
        // filtered non-self subject would drop the `of` narrowing (active) or
        // invert the sentence (passive) on render. Both voices decline rather
        // than emit an unrenderable filter ([CR#509.3b],[CR#509.3d]).
        assert!(trig("Whenever a creature you control blocks a creature, draw a card.").is_none());
        assert!(
            trig(
                "Whenever another creature you control becomes blocked by a creature, draw a card."
            )
            .is_none()
        );
    }

    #[test]
    fn blocks_or_becomes_blocked_disjunction_declines() {
        // No core `EventFilter` combinator (Any/Or) holds "blocks or becomes
        // blocked" — deferred (~101 corpus cards; the compound object residue
        // never resolves to the literal "a creature", so this correctly
        // declines rather than mis-parsing).
        assert!(trig("Whenever ~ blocks or becomes blocked by a creature, draw a card.").is_none());
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
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Subtype(\"Elf\")])), \
                 effect: May(effect: Create(1, Token(color_indicator: [Green], types: [Creature], \
                 subtypes: [Elf, Warrior], power: 1, toughness: 1))))"
            )
        );
    }

    #[test]
    fn cast_self_spell() {
        // Cascade's own reminder-text shape: "Whenever you cast this
        // spell, ..." (normalized to the `~` sigil by extraction).
        assert_eq!(
            trig("Whenever you cast ~, draw a card.").as_deref(),
            Some("Triggered(event: Cast(who: Ref(You), what: Ref(This)), effect: Draw(1))")
        );
    }

    #[test]
    fn cast_bare_spell() {
        assert_eq!(
            trig("Whenever you cast a spell, draw a card.").as_deref(),
            Some("Triggered(event: Cast(who: Ref(You), what: Kind(Spell)), effect: Draw(1))")
        );
    }

    #[test]
    fn cast_creature_spell() {
        assert_eq!(
            trig("Whenever you cast a creature spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), what: And([Kind(Spell), Type(\"Creature\")])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_instant_or_sorcery_spell() {
        assert_eq!(
            trig("Whenever you cast an instant or sorcery spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Or([Type(\"Instant\"), Type(\"Sorcery\")])])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_noncreature_spell() {
        assert_eq!(
            trig("Whenever you cast a noncreature spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Not(Type(\"Creature\"))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_trigger_declines_out_of_scope() {
        // A multi-word descriptor before "spell" that isn't the modeled
        // "instant or sorcery"/"noncreature" shapes -> declines (this v1
        // production handles only single-token and those two disjunction/
        // negation shapes).
        assert!(trig("Whenever you cast a creature or artifact spell, draw a card.").is_none());
        // An opponent's cast is not the controller-cast surface modeled here.
        assert!(trig("Whenever an opponent casts a spell, draw a card.").is_none());
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn beginning_of_combat_your_turn_create_token() {
        // Goblin Rabblemaster: "At the beginning of combat on your turn, create a
        // 1/1 red Goblin creature token with haste."
        assert_eq!(
            trig("At the beginning of combat on your turn, create a 1/1 red Goblin creature token with haste.")
                .as_deref(),
            Some(
                "Triggered(event: StepBegins(at: Combat(BeginningOfCombat), whose: Your), \
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
        // the same `StepBegins(at: Beginning(Upkeep), whose: Your)` + `Unless`
        // shape the kw-echo macro emits.
        assert_eq!(
            trig("At the beginning of your upkeep, sacrifice ~ unless you pay {G}{G}.").as_deref(),
            Some(
                "Triggered(event: StepBegins(at: Beginning(Upkeep), whose: Your), \
                 effect: Unless(effect: Sacrifice(This), unless: [Mana([Green,Green])]))"
            )
        );
    }

    #[test]
    fn beginning_of_your_upkeep_sacrifice_unconditional() {
        // Necrotic Plague's granted ability: "At the beginning of your
        // upkeep, sacrifice this creature." (oracle "this creature" is the
        // printed-name convention the render side spells out; the parser
        // reads either that or "~" as the same self anaphor).
        assert_eq!(
            trig("At the beginning of your upkeep, sacrifice ~.").as_deref(),
            Some(
                "Triggered(event: StepBegins(at: Beginning(Upkeep), whose: Your), \
                 effect: Sacrifice(This))"
            )
        );
    }

    #[test]
    fn beginning_of_your_upkeep_sacrifice_unless_non_mana_toll_declines() {
        // A non-mana "unless you <action>" toll (Bog Elemental's "unless you
        // sacrifice a land", Argentum Masticore's "unless you discard a
        // card") declines: only a single mana cost is modeled
        // ([`parse_sacrifice`]'s doc comment) because the `MustPay` render
        // arm ([CR#118.12a]) can only reproduce a symbol cost today
        // (`render_cost`) — a verb-shaped toll has no render arm, so the
        // parser stays narrower than what a richer cost grammar could in
        // principle accept, rather than emit RON the renderer can't
        // reproduce. Deferred, not built.
        assert!(
            trig("At the beginning of your upkeep, sacrifice ~ unless you sacrifice a land.")
                .is_none()
        );
        assert!(
            trig("At the beginning of your upkeep, sacrifice ~ unless you discard a card.")
                .is_none()
        );
    }

    #[test]
    fn beginning_of_end_step_sacrifice_each_players() {
        // "the end step" (no possessive) fires every turn => EachPlayers.
        assert_eq!(
            trig("At the beginning of the end step, sacrifice ~.").as_deref(),
            Some(
                "Triggered(event: StepBegins(at: Ending(End), whose: EachPlayers), \
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
            Some("Triggered(event: ThisDies, effect: Move(This, Hand))")
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
                "Triggered(event: Dies(And([Creature, Not(Ref(This)), ControlledBy(Ref(You))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn landfall_ability_word_stripped_then_pump() {
        // "Landfall — " is a flavor label ([CR#207.2c]); the land-ETB trigger +
        // self-pump underneath is what parses, and the label survives as the
        // emitted ability's `ability_word` render metadata.
        assert_eq!(
            trig("Landfall — Whenever a land you control enters, ~ gets +2/+2 until end of turn.")
                .as_deref(),
            Some(
                "Triggered(ability_word: \"Landfall\", \
                 event: Enters(And([Type(\"Land\"), ControlledBy(Ref(You))])), \
                 effect: Continuously(effect: Modify(This, \
                 Several([Power(Up(2)), Toughness(Up(2))])), duration: FixedUntil(EndOfTurn)))"
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
                "Triggered(ability_word: \"Landfall\", \
                 event: Enters(And([Type(\"Land\"), ControlledBy(Ref(You))])), \
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
                 [TargetOne(And([Creature, ControlledBy(Ref(You))]))], \
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
            super::split_ability_word("Threshold — Whenever ~ attacks, draw a card."),
            (Some("Threshold"), "Whenever ~ attacks, draw a card.")
        );
        // No trigger word after the dash => left whole, no label captured.
        assert_eq!(
            super::split_ability_word("Choose one — draw a card."),
            (None, "Choose one — draw a card.")
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

    #[test]
    fn combat_damage_self_to_player() {
        // Poisonous/Renown/Ingest's shared trigger clause
        // [CR#702.70a,702.112a,702.115a]: "~ deals combat damage to a
        // player" — the dominant recipient form (625 corpus cards).
        assert_eq!(
            trig_builtin("Whenever ~ deals combat damage to a player, draw a card.").as_deref(),
            Some("Triggered(event: DealsCombatDamage(Ref(This), Player), effect: Draw(1))")
        );
    }

    #[test]
    fn combat_damage_filtered_subject() {
        // Non-`~` subject: the source is parsed by the shared filter grammar,
        // same as the enters/dies/attacks events.
        assert_eq!(
            trig_builtin(
                "Whenever a creature you control deals combat damage to a player, you gain 1 life."
            )
            .as_deref(),
            Some(
                "Triggered(event: DealsCombatDamage(And([Creature, ControlledBy(Ref(You))]), \
                 Player), effect: GainLife(1))"
            )
        );
    }

    #[test]
    fn combat_damage_alternate_recipients() {
        // "a creature" — the recipient falls through to `filter::parse_phrase`.
        assert_eq!(
            trig_builtin("Whenever ~ deals combat damage to a creature, draw a card.").as_deref(),
            Some("Triggered(event: DealsCombatDamage(Ref(This), Creature), effect: Draw(1))")
        );
        // "an opponent" — the player-identity form `recipient_phrase` adds.
        assert_eq!(
            trig_builtin("Whenever ~ deals combat damage to an opponent, draw a card.").as_deref(),
            Some(
                "Triggered(event: DealsCombatDamage(Ref(This), OpponentOf(Ref(You))), \
                 effect: Draw(1))"
            )
        );
        // "a player or planeswalker" (Lava Spike's restricted-any-target
        // shape, [CR#115.4]) — an "X or Y" disjunction over two recipients.
        assert_eq!(
            trig_builtin(
                "Whenever ~ deals combat damage to a player or planeswalker, draw a card."
            )
            .as_deref(),
            Some(
                "Triggered(event: DealsCombatDamage(Ref(This), Or([Player, Planeswalker])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn dealt_damage_self_and_filtered() {
        // "~ is dealt damage" — the generic (non-combat-narrowed) damage-dealt
        // event ([CR#120.1]), passive-voice recipient-subject phrasing. Real
        // corpus text: Goblin Medics-adjacent "Enrage" cards' "Whenever this
        // creature is dealt damage, …".
        assert_eq!(
            trig_builtin("Whenever ~ is dealt damage, draw a card.").as_deref(),
            Some("Triggered(event: DealtDamage(Ref(This)), effect: Draw(1))")
        );
        // Non-`~` subject: the recipient is parsed by the shared filter
        // grammar, same as the combat-damage recipient.
        assert_eq!(
            trig_builtin("Whenever a creature is dealt damage, draw a card.").as_deref(),
            Some("Triggered(event: DealtDamage(Creature), effect: Draw(1))")
        );
        // "an opponent" — the player-identity form `recipient_phrase` adds
        // (real corpus text: "Whenever an opponent is dealt damage, …").
        assert_eq!(
            trig_builtin("Whenever an opponent is dealt damage, draw a card.").as_deref(),
            Some("Triggered(event: DealtDamage(OpponentOf(Ref(You))), effect: Draw(1))")
        );
    }

    #[test]
    fn dealt_damage_does_not_shadow_combat_damage_phrasing() {
        // "is dealt COMBAT damage" is a different, currently-unhandled
        // passive phrasing (out of this wave's scope) — the extra "combat"
        // word means the clause never ends in the literal " is dealt
        // damage" tail, so it correctly declines instead of being
        // mis-routed to the generic (non-combat) `DealtDamage` event.
        assert!(trig_builtin("Whenever ~ is dealt combat damage, draw a card.").is_none());
    }

    #[test]
    fn gains_life_you_and_opponent() {
        // "you gain life" — the controller's own life gain ([CR#119.3]), the
        // `GainsLife` event macro; no `~` shorthand (a permanent is never
        // "you").
        assert_eq!(
            trig_builtin("Whenever you gain life, draw a card.").as_deref(),
            Some("Triggered(event: GainsLife(Ref(You)), effect: Draw(1))")
        );
        // "an opponent gains life" — the 3rd-person-singular verb agreement
        // real oracle text uses for a non-"you" subject.
        assert_eq!(
            trig_builtin("Whenever an opponent gains life, draw a card.").as_deref(),
            Some("Triggered(event: GainsLife(OpponentOf(Ref(You))), effect: Draw(1))")
        );
    }

    #[test]
    fn draws_you_and_opponent() {
        // "you draw a card" — the controller's own draw ([CR#121.1]), the
        // `Draws` event macro (mind the underlying event: the macro body
        // expands to `Drawn`, not `Draws`).
        assert_eq!(
            trig_builtin("Whenever you draw a card, draw a card.").as_deref(),
            Some("Triggered(event: Draws(Ref(You)), effect: Draw(1))")
        );
        // "an opponent draws a card" — the 3rd-person-singular verb agreement.
        assert_eq!(
            trig_builtin("Whenever an opponent draws a card, draw a card.").as_deref(),
            Some("Triggered(event: Draws(OpponentOf(Ref(You))), effect: Draw(1))")
        );
    }

    #[test]
    fn becomes_tapped_self_and_filtered() {
        assert_eq!(
            trig("Whenever ~ becomes tapped, draw a card.").as_deref(),
            Some("Triggered(event: ThisBecomesTapped, effect: Draw(1))")
        );
        // Non-`~` subject: parsed by the shared filter grammar, same as the
        // enters/dies/attacks events.
        assert_eq!(
            trig("Whenever a creature becomes tapped, draw a card.").as_deref(),
            Some("Triggered(event: BecomesTapped(Creature), effect: Draw(1))")
        );
    }

    #[test]
    fn becomes_tapped_matches_goblin_medics_real_card_text() {
        // Goblin Medics (`plugins/canon/cards/Goblin Medics.ron`) is
        // hand-authored today with a comment noting "the becomes-tapped
        // event shape is beyond the triggered-ability parser today" — this
        // production closes that gap; pin the exact real oracle line to
        // the card's own hand-authored RON shape.
        assert_eq!(
            trig("Whenever ~ becomes tapped, it deals 1 damage to any target.").as_deref(),
            Some(
                "Triggered(event: ThisBecomesTapped, effect: Targeted(targets: [AnyTarget], \
                 effect: DealDamage(This, 1, Target(0))))"
            )
        );
    }

    #[test]
    fn becomes_untapped_self_and_filtered() {
        assert_eq!(
            trig("Whenever ~ becomes untapped, draw a card.").as_deref(),
            Some("Triggered(event: ThisBecomesUntapped, effect: Draw(1))")
        );
        assert_eq!(
            trig("Whenever a creature becomes untapped, draw a card.").as_deref(),
            Some("Triggered(event: BecomesUntapped(Creature), effect: Draw(1))")
        );
    }
}
