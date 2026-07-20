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
    // Peel a trailing "This ability triggers only once[ each turn]." rider
    // sentence ([CR#603.2h]) off the whole line, lifting it into a `UseLimit`
    // the emitted `Triggered` frame carries; the ability body proper then
    // parses on its own below, instead of the rider sentence failing the whole
    // line. Mirrors the activated-frame sibling
    // `activated_ability::peel_activation_riders`.
    let (line, limit) = peel_trigger_limit(line);
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
        return Ok(Some(render(ability_word, &event, limit, &parsed)));
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
    Ok(Some(render(ability_word, &event, limit, &parsed)))
}

/// Strip a trailing "This ability triggers only once[ each turn]." rider
/// sentence off a trigger line, lifting it into the [`UseLimit`] RON atom the
/// emitted `Triggered` frame carries ([CR#603.2h]) — the parse-direction
/// mirror of `render::ability::triggered`'s trigger-limit rider, and the
/// trigger-frame analogue of `activated_ability::peel_activation_riders`. The
/// self-scoped "This ability" phrase is NOT tilde-normalized (see
/// `extract::self_ref_to_tilde` — "this ability" is an excluded noun), so it
/// appears verbatim here. Returns the line with the rider removed (the effect
/// body keeps its own trailing period, half of the " …" delimiter) and the
/// limit atom, or the unchanged line and `None` when no such rider trails.
///
/// The per-turn form ("… only once each turn.") checks first; the bare
/// per-game form ("… only once.") is a distinct suffix (ends "once." not
/// "turn."), so the two never collide. Longer/narrower riders ("… only once
/// each upkeep.", "… only once for each …", "… only once, no matter how
/// many …") match neither suffix and stay attached — those trigger-frequency
/// shapes aren't modeled by `UseLimit`, so the line stays `Unparsed` rather
/// than lifting a limit the render side can't reproduce.
fn peel_trigger_limit(line: &str) -> (&str, Option<&'static str>) {
    if let Some(body) = line.strip_suffix(" This ability triggers only once each turn.") {
        return (body, Some("OncePerTurn"));
    }
    if let Some(body) = line.strip_suffix(" This ability triggers only once.") {
        return (body, Some("OncePerGame"));
    }
    (line, None)
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

/// Wraps an event + optional [`UseLimit`] + [`ParsedEffect`] in the
/// `Triggered` frame, emitting `limits:` only when a trigger-limit rider was
/// peeled and `targets:` only when the effect declares any.
///
/// The body is wrapped VERBATIM, exactly as the spell/activated/modal/loyalty
/// frames wrap theirs. A trigger's event roles (`anaphora.that_object` &c.) and
/// its announced targets no longer compete: the effect grammar emits every
/// target read positionally at its source ([CR#115.3,601.2c]), so nothing here
/// has to rewrite the body to tell the two apart.
///
/// `limits:` sits on the outer frame between `event:` and `effect:`, in the
/// `TriggeredAbility` struct's declared field order (mirrors the activated
/// frame's rider placement).
fn render(
    ability_word: Option<&str>,
    event: &str,
    limit: Option<&str>,
    parsed: &ParsedEffect,
) -> String {
    // [CR#207.2c]: the stripped ability-word label rides as render metadata.
    let word = ability_word.map_or_else(String::new, |w| format!("ability_word: \"{w}\", "));
    // [CR#603.2h]: the peeled trigger-frequency limit rides as a `limits:` field.
    let limits = limit.map_or_else(String::new, |l| format!(", limits: [{l}]"));
    if parsed.targets.is_empty() {
        format!(
            "Triggered({word}event: {event}{limits}, effect: {})",
            parsed.effect
        )
    } else {
        format!(
            "Triggered({word}event: {event}{limits}, effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect,
        )
    }
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
    if let Some(event) = parse_event_disjunction(clause) {
        return Some(event);
    }
    parse_event_atom(clause)
}

/// Parse the two OR-composed event-clause shapes used by printed triggers:
///
/// - one verb over two subjects: `~ or another Ally you control enters`;
/// - one subject over two verbs: `~ enters or attacks`.
///
/// Each arm is parsed through the ordinary event production, then retained as
/// its own [`EventFilter`](deckmaste_core::EventFilter) disjunct. Keeping the
/// subject/event pairing per arm matters when the verbs name different master
/// forms (`ZoneChange` vs. `AttackDeclared`), and makes the trigger fire only
/// once if one occurrence happens to satisfy both arms ([CR#603.2c]).
///
/// Partner-style names use plural agreement after extraction (`~ enter or
/// attack`). [`parse_compound_event_atom`] singularizes only these bounded
/// event verbs before routing them through the same atom parser.
fn parse_event_disjunction(clause: &str) -> Option<String> {
    let (left, right) = clause.split_once(" or ")?;

    // Shared verb: "~ or another Ally you control enters".
    if let Some((right_subject, verb)) = split_simple_event(right) {
        let left_clause = format!("{left} {}", verb.singular());
        let left_event = parse_compound_event_atom(&left_clause)?;
        let right_event = parse_compound_event_atom(right)?;
        if !right_subject.is_empty() {
            return Some(format!("OneOf([{left_event}, {right_event}])"));
        }
    }

    // Shared subject: "~ enters or attacks". The right side carries only a
    // verb, so borrow the left event's subject and parse the reconstructed
    // second clause through the ordinary event atom production.
    let (subject, _) = split_simple_event(left)?;
    let right_clause = format!("{subject} {right}");
    let left_event = parse_compound_event_atom(left)?;
    let right_event = parse_compound_event_atom(&right_clause)?;
    Some(format!("OneOf([{left_event}, {right_event}])"))
}

#[derive(Clone, Copy)]
enum SimpleEventVerb {
    Enters,
    Dies,
    LeavesBattlefield,
    Attacks,
    Blocks,
    BecomesBlocked,
}

impl SimpleEventVerb {
    fn singular(self) -> &'static str {
        match self {
            Self::Enters => "enters",
            Self::Dies => "dies",
            Self::LeavesBattlefield => "leaves the battlefield",
            Self::Attacks => "attacks",
            Self::Blocks => "blocks",
            Self::BecomesBlocked => "becomes blocked",
        }
    }
}

/// Split a simple object event into its subject and normalized verb. The
/// plural spellings are confined to compound-name trigger subjects; the
/// returned verb always reconstructs with singular agreement so the existing
/// event atom parser remains the single source of event emission.
fn split_simple_event(clause: &str) -> Option<(&str, SimpleEventVerb)> {
    const SUFFIXES: [(&str, SimpleEventVerb); 12] = [
        (
            " leaves the battlefield",
            SimpleEventVerb::LeavesBattlefield,
        ),
        (" becomes blocked", SimpleEventVerb::BecomesBlocked),
        (" is blocked", SimpleEventVerb::BecomesBlocked),
        (" enters", SimpleEventVerb::Enters),
        (" enter", SimpleEventVerb::Enters),
        (" dies", SimpleEventVerb::Dies),
        (" die", SimpleEventVerb::Dies),
        (" attacks", SimpleEventVerb::Attacks),
        (" attack", SimpleEventVerb::Attacks),
        (" blocks", SimpleEventVerb::Blocks),
        (" block", SimpleEventVerb::Blocks),
        (" leave", SimpleEventVerb::LeavesBattlefield),
    ];
    SUFFIXES.iter().find_map(|(suffix, verb)| {
        clause
            .strip_suffix(suffix)
            .filter(|subject| !subject.is_empty())
            .map(|subject| (subject, *verb))
    })
}

fn parse_compound_event_atom(clause: &str) -> Option<String> {
    if let Some(event) = parse_event_atom(clause) {
        return Some(event);
    }
    let (subject, verb) = split_simple_event(clause)?;
    parse_event_atom(&format!("{subject} {}", verb.singular()))
}

fn parse_event_atom(clause: &str) -> Option<String> {
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

/// "<subject> cast(s) X" -> `Cast(who: <who>, what: <filter>)` ([CR#601.2i]
/// cast onset; mirrors the Prowess/Cascade macros' filtered-cast shape). The
/// recognized `who:` subjects, via [`who_and_rest`]: the controller's own
/// cast ("you cast"), any player's ("a player casts"), and specifically an
/// opponent's ("an opponent casts"). The recognized `what:` shapes, in order:
///
/// - self ("… cast ~") -> `Ref(This)` (Cascade's own "you cast this spell"
///   reminder-text shape).
/// - a bare spell ("… cast a spell") -> `Kind(Spell)`.
/// - "a[n] <card-type> spell" (creature/artifact/enchantment/instant/
///   sorcery/…) -> `And([Kind(Spell), Type(<T>)])`.
/// - "a[n] <Subtype> spell" (the original v1 shape, e.g. "an Elf spell") ->
///   `And([Kind(Spell), Subtype("<X>")])`. A lone non-subtype token still mints
///   a `Subtype`, caught downstream by the catalog lint, as in the shared
///   filter grammar's bare-token head.
/// - "noncreature" -> `And([Kind(Spell), Not(Type(Creature))])`.
/// - a type/subtype disjunction ("an instant or sorcery spell", "a Spirit or
///   Arcane spell") -> `And([Kind(Spell), Or([...])])`, via
///   [`disjunction_atom`] — a GENERAL 2-way disjunction (of card types XOR of
///   catalog subtypes, never mixed), not a hardcoded pairing. An Oxford-comma
///   3+-way list ("an artifact, instant, or sorcery spell") declines — see
///   [`disjunction_atom`]'s doc for why.
/// - "a spell with mana value N or greater/less" -> `And([Kind(Spell),
///   Stat(ManaValue, AtLeast/AtMost, N)])` ([CR#202.3]).
/// - "a spell that targets ~" (heroic's head, [CR#115.9b]) ->
///   `And([Kind(Spell), Targets(Ref(This))])` — restricted to the self target,
///   the only shape real oracle text uses here.
/// - any TWO of the above combined ("a creature spell with mana value 3 or
///   less") -> `And([Kind(Spell), <head atom>, <postfix atom>])`, via
///   [`split_at_spell`].
///
/// Any other multi-word descriptor or postfix (restriction-laden forms:
/// "your first spell each turn", "a spell that's one or more colors", "cast
/// or copy", …) declines, left for the debug/unparsed path — these need
/// extra filter machinery or trigger-condition modeling this v1 production
/// doesn't carry, or (for "cast or copy") have no `EventFilter` combinator to
/// carry a copy-or-cast disjunction at all.
fn parse_cast_event(clause: &str) -> Option<String> {
    let (who, rest) = who_and_rest(clause)?;
    if rest == "~" {
        return Some(format!("Cast(who: {who}, what: Ref(This))"));
    }
    let body = rest
        .strip_prefix("a ")
        .or_else(|| rest.strip_prefix("an "))?;
    let (descriptor, postfix) = split_at_spell(body)?;
    let mut atoms = Vec::new();
    if !descriptor.is_empty() {
        atoms.extend(descriptor_atoms(descriptor)?);
    }
    if let Some(p) = postfix {
        atoms.push(spell_postfix_atom(p)?);
    }
    if atoms.is_empty() {
        return Some(format!("Cast(who: {who}, what: Kind(Spell))"));
    }
    Some(format!(
        "Cast(who: {who}, what: And([Kind(Spell), {}]))",
        atoms.join(", ")
    ))
}

/// Strips the recognized "<subject> cast(s) " lead off a cast-trigger clause,
/// returning the `who:` RON atom and the rest of the clause (the `what:`
/// phrase). Three subjects appear in real oracle text ([CR#601.2i]): the
/// controller's own cast ("you cast" -> `Ref(You)`), any player's ("a player
/// casts" -> `Player`, the [`Predicate`](deckmaste_core::Predicate) macro that
/// expands to `Kind(Player)`), and specifically an opponent's ("an opponent
/// casts" -> `OpponentOf(Ref(You))`, mirroring
/// [`filter::recipient_phrase`]'s "an opponent" reading). Any other subject
/// (an unrecognized phrase, "you cast or copy") declines — none of the three
/// prefixes match, so `rest` is never produced and the caller's own
/// descriptor match never runs.
fn who_and_rest(clause: &str) -> Option<(&'static str, &str)> {
    if let Some(rest) = clause.strip_prefix("you cast ") {
        return Some(("Ref(You)", rest));
    }
    if let Some(rest) = clause.strip_prefix("a player casts ") {
        return Some(("Player", rest));
    }
    if let Some(rest) = clause.strip_prefix("an opponent casts ") {
        return Some(("OpponentOf(Ref(You))", rest));
    }
    None
}

/// Splits the "a"/"an"-stripped `what:` body into its descriptor (the run
/// before "spell", empty for a bare "spell") and an optional postfix
/// qualifier clause (the run after "spell"): "creature spell" ->
/// `("creature", None)`; "spell with mana value 4 or greater" -> `("",
/// Some("with mana value 4 or greater"))`; "creature spell with mana value 3
/// or less" -> `("creature", Some("with mana value 3 or less"))`. `None` when
/// the body has no "spell" head noun at all.
fn split_at_spell(body: &str) -> Option<(&str, Option<&str>)> {
    if body == "spell" {
        return Some(("", None));
    }
    if let Some(rest) = body.strip_prefix("spell ") {
        return Some(("", Some(rest)));
    }
    if let Some(descriptor) = body.strip_suffix(" spell") {
        return Some((descriptor, None));
    }
    let idx = body.find(" spell ")?;
    Some((&body[..idx], Some(&body[idx + " spell ".len()..])))
}

/// The descriptor's atom(s) beyond `Kind(Spell)`: a card-type/catalog-subtype
/// disjunction ([`disjunction_atom`]), the "noncreature" negation, or a
/// single card-type/subtype token (the original v1 shapes — a lone
/// non-subtype token still mints a `Subtype`, unconditional like the shared
/// filter grammar's bare-token head). `None` for any other multi-word
/// descriptor (out of scope — restriction-laden forms this v1 production
/// doesn't carry).
fn descriptor_atoms(descriptor: &str) -> Option<Vec<String>> {
    if descriptor == "noncreature" {
        return Some(vec!["Not(Type(Creature))".to_owned()]);
    }
    if descriptor.contains(" or ") {
        return disjunction_atom(descriptor).map(|atom| vec![atom]);
    }
    if descriptor.contains(' ') {
        return None;
    }
    let atom = filter::type_filter(&filter::singularize(descriptor).to_ascii_lowercase())
        .unwrap_or_else(|| format!("Subtype({})", crate::ident::to_rust_ident(descriptor)));
    Some(vec![atom])
}

/// The `Or([...])` atom for an "X or Y" 2-way disjunction descriptor — a
/// card-type disjunction (`Or([Type("Instant"), Type("Sorcery")])`, the
/// general form of the former instant-or-sorcery-only special case) or a
/// catalog-subtype disjunction (`Or([Subtype("Spirit"), Subtype("Arcane")])`,
/// the "Spirit or Arcane" shape) — never a MIX of the two kinds (a
/// [`filter::type_filter`] miss on EITHER member falls through to the subtype
/// check, which requires BOTH members to validate against the subtype
/// catalog). `None` for anything else (not shaped like a 2-way disjunction, a
/// multi-word member, or a member neither a type noun nor a real subtype).
///
/// Restricted to 2-way: an Oxford-comma 3+-way list ("an artifact, instant,
/// or sorcery spell") carries an INTERNAL comma that `resolve_line`'s own
/// "When/Whenever <event>, <effect>" split (a single `split_once(", ")`)
/// would cut at instead of the real event/effect boundary — unreachable from
/// real oracle text through this parser without first fixing that shared
/// split, out of scope here. A 2-way "X or Y" never has an internal comma, so
/// it's unaffected.
fn disjunction_atom(descriptor: &str) -> Option<String> {
    let (a, b) = split_or_pair(descriptor)?;
    if let (Some(ta), Some(tb)) = (
        filter::type_filter(&filter::singularize(a).to_ascii_lowercase()),
        filter::type_filter(&filter::singularize(b).to_ascii_lowercase()),
    ) {
        return Some(format!("Or([{ta}, {tb}])"));
    }
    (filter::is_subtype(a) && filter::is_subtype(b)).then(|| {
        format!(
            "Or([Subtype({}), Subtype({})])",
            crate::ident::to_rust_ident(a),
            crate::ident::to_rust_ident(b)
        )
    })
}

/// Splits an "X or Y" 2-way disjunction descriptor into its two single-word
/// members, or `None` if the descriptor isn't shaped like one, or either
/// member is itself multi-word (out of scope — only bare type/subtype names
/// disjoin here, e.g. "Faerie or Wizard permanent" declines since "Wizard
/// permanent" isn't a bare member).
fn split_or_pair(s: &str) -> Option<(&str, &str)> {
    let (a, b) = s.split_once(" or ")?;
    (!a.is_empty() && !a.contains(' ') && !b.is_empty() && !b.contains(' ')).then_some((a, b))
}

/// A postfix qualifier clause trailing "spell" -> its `Predicate` atom: "with
/// mana value N or greater/less" ([CR#202.3]) or "that targets ~" (heroic's
/// head, [CR#115.9b] — restricted to the self target, the only shape real
/// oracle text uses here). `None` for anything else (out of scope — a
/// color/historic/kicked rider, "that's one or more colors", "with cascade",
/// …).
fn spell_postfix_atom(postfix: &str) -> Option<String> {
    if let Some(rest) = postfix.strip_prefix("with mana value ") {
        let (n, word) = rest.split_once(" or ")?;
        let n: u32 = n.parse().ok()?;
        let cmp = match word {
            "greater" => "AtLeast",
            "less" => "AtMost",
            _ => return None,
        };
        return Some(format!("Stat(ManaValue, {cmp}, {n})"));
    }
    (postfix == "that targets ~").then(|| "Targets(Ref(This))".to_owned())
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
    } else if let Some(step) = step_clause.strip_prefix("each player's ") {
        ("EachPlayers", step)
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
            Some("Triggered(event: Enters(And([Permanent, Subtype(Goblin)])), effect: Draw(1))")
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
    fn once_each_turn_rider_lifts_per_turn_limit() {
        // The trailing "This ability triggers only once each turn." rider
        // ([CR#603.2h]) is peeled off the effect body and lifted into a
        // `OncePerTurn` use-limit; the body ("draw a card.") parses on its own.
        assert_eq!(
            trig("Whenever you cast an instant or sorcery spell, draw a card. This ability triggers only once each turn.")
                .as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Or([Type(Instant), Type(Sorcery)])])), \
                 limits: [OncePerTurn], effect: Draw(1))"
            )
        );
    }

    #[test]
    fn once_bare_rider_lifts_per_game_limit() {
        // The bare "This ability triggers only once." per-game form (no "each
        // turn") maps to `OncePerGame` ([CR#603.2h]).
        assert_eq!(
            trig("When ~ enters, draw a card. This ability triggers only once.").as_deref(),
            Some("Triggered(event: ThisEnters, limits: [OncePerGame], effect: Draw(1))")
        );
    }

    #[test]
    fn once_each_turn_rider_before_targeted_wrapper() {
        // `limits:` sits on the outer frame, between `event:` and the
        // `Targeted` effect wrapper — not inside it.
        assert_eq!(
            trig(
                "When ~ dies, destroy target creature. This ability triggers only once each turn."
            )
            .as_deref(),
            Some(
                "Triggered(event: ThisDies, limits: [OncePerTurn], effect: \
                 Targeted(targets: [TargetOne(Creature)], effect: Destroy(Target(0))))"
            )
        );
    }

    #[test]
    fn narrower_once_rider_stays_attached_and_declines() {
        // Only the two exact rider forms are modeled. A narrower trigger-
        // frequency rider ("… only once each upkeep.") matches neither suffix,
        // stays attached to the body, and the whole line declines rather than
        // lifting a limit the render side can't reproduce.
        assert!(
            trig("At the beginning of each player's upkeep, draw a card. This ability triggers only once each upkeep.")
                .is_none()
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
                 Several([Power(Up(CountOf(Objects(And([Permanent, Subtype(Goblin), Not(Ref(This)), Attacking]))))), \
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
                "Triggered(event: ThisAttacks, effect: GainLife(CountOf(Objects(And([Permanent, Subtype(Elf), \
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
    fn blocks_or_becomes_blocked_by_creature_disjunction() {
        // The second arm keeps its directional blocker narrowing while the
        // first remains the ordinary bare "blocks" event.
        assert_eq!(
            trig("Whenever ~ blocks or becomes blocked by a creature, draw a card.").as_deref(),
            Some(
                "Triggered(event: OneOf([ThisBlocks, Blocking(Creature, Ref(This))]), effect: Draw(1))"
            )
        );
    }

    #[test]
    fn blocks_or_becomes_blocked_gives_target_opponent_control() {
        assert_eq!(
            trig("Whenever ~ blocks or becomes blocked, target opponent gains control of it.")
                .as_deref(),
            Some(
                "Triggered(event: OneOf([ThisBlocks, ThisBecomesBlocked]), effect: Targeted(targets: [TargetOne(OpponentOf(Ref(You)))], effect: GainControl(This, Target(0))))"
            )
        );
    }

    #[test]
    fn or_subject_and_or_event_triggers() {
        assert_eq!(
            trig("Whenever ~ or another Ally you control enters, draw a card.").as_deref(),
            Some(
                "Triggered(event: OneOf([ThisEnters, Enters(And([Permanent, Subtype(Ally), Not(Ref(This)), ControlledBy(Ref(You))]))]), effect: Draw(1))"
            )
        );
        assert_eq!(
            trig("When ~ enters or dies, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisEnters, ThisDies]), effect: Draw(1))")
        );
        assert_eq!(
            trig("Whenever ~ enters or attacks, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisEnters, ThisAttacks]), effect: Draw(1))")
        );
        assert_eq!(
            trig("When ~ enters or leaves the battlefield, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisEnters, ThisLeavesBattlefield]), effect: Draw(1))")
        );
        assert_eq!(
            trig("Whenever ~ attacks or blocks, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisAttacks, ThisBlocks]), effect: Draw(1))")
        );
    }

    #[test]
    fn plural_compound_name_or_event_trigger() {
        assert_eq!(
            trig("Whenever ~ enter or attack, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisEnters, ThisAttacks]), effect: Draw(1))")
        );
        assert_eq!(
            trig("Whenever ~ attack or block, draw a card.").as_deref(),
            Some("Triggered(event: OneOf([ThisAttacks, ThisBlocks]), effect: Draw(1))")
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
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Subtype(Elf)])), \
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
                "Triggered(event: Cast(who: Ref(You), what: And([Kind(Spell), Type(Creature)])), \
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
                 what: And([Kind(Spell), Or([Type(Instant), Type(Sorcery)])])), \
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
                 what: And([Kind(Spell), Not(Type(Creature))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_trigger_declines_out_of_scope() {
        // "cast or copy" has no `EventFilter` combinator for a copy-or-cast
        // disjunction — "or copy …" is never consumed by the "you cast "
        // prefix, so `who_and_rest` itself never matches.
        assert!(
            trig("Whenever you cast or copy an instant or sorcery spell, draw a card.").is_none()
        );
        // A multi-word postfix that isn't the modeled mana-value/targets
        // shapes declines (out of scope — this v1 production doesn't carry
        // color-composite filters).
        assert!(
            trig("Whenever you cast a spell that's one or more colors, draw a card.").is_none()
        );
        // "Faerie or Wizard permanent" isn't a bare-member disjunction list
        // (the second member is two words) -> declines rather than mis-parse.
        assert!(
            trig("Whenever you cast a Faerie or Wizard permanent spell, draw a card.").is_none()
        );
    }

    #[test]
    fn cast_type_disjunction_general() {
        // The general 2-way card-type disjunction (any pairing, not just the
        // former instant-or-sorcery-only special case).
        assert_eq!(
            trig("Whenever you cast a creature or planeswalker spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Or([Type(Creature), Type(Planeswalker)])])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_type_disjunction_with_player_subject() {
        // A "a player casts" subject paired with the 2-way disjunction.
        assert_eq!(
            trig("Whenever a player casts a creature spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Player, \
                 what: And([Kind(Spell), Type(Creature)])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_type_disjunction_three_way_oxford_comma_declines() {
        // An Oxford-comma 3+-way list ("an artifact, instant, or sorcery
        // spell") carries an INTERNAL comma that `resolve_line`'s own
        // "When/Whenever <event>, <effect>" split (a single `split_once(",
        // ")`) cuts at instead of the real event/effect boundary — this
        // parser never even sees the intact clause, so it declines (a
        // pre-existing `resolve_line` limitation, out of scope for this
        // production to fix).
        assert!(
            trig("Whenever a player casts an artifact, instant, or sorcery spell, draw a card.")
                .is_none()
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn cast_subtype_disjunction() {
        // "a Spirit or Arcane spell" — a catalog-subtype disjunction, distinct
        // from the card-type disjunction (neither word is a type noun).
        assert_eq!(
            trig("Whenever you cast a Spirit or Arcane spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Or([Subtype(Spirit), Subtype(Arcane)])])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_mana_value_threshold() {
        assert_eq!(
            trig("Whenever you cast a spell with mana value 4 or greater, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Stat(ManaValue, AtLeast, 4)])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_creature_spell_with_mana_value_threshold() {
        // A head atom (card type) combined with a postfix atom (mana-value
        // threshold) — "a creature spell with mana value 3 or less".
        assert_eq!(
            trig("Whenever you cast a creature spell with mana value 3 or less, draw a card.")
                .as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Type(Creature), Stat(ManaValue, AtMost, 3)])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_spell_that_targets_self() {
        // Heroic's head ([CR#115.9b]): "a spell that targets ~".
        assert_eq!(
            trig("Whenever you cast a spell that targets ~, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: Ref(You), \
                 what: And([Kind(Spell), Targets(Ref(This))])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_opponent_subject() {
        assert_eq!(
            trig("Whenever an opponent casts a creature spell, draw a card.").as_deref(),
            Some(
                "Triggered(event: Cast(who: OpponentOf(Ref(You)), \
                 what: And([Kind(Spell), Type(Creature)])), \
                 effect: Draw(1))"
            )
        );
    }

    #[test]
    fn cast_player_subject_bare_spell() {
        assert_eq!(
            trig("Whenever a player casts a spell, draw a card.").as_deref(),
            Some("Triggered(event: Cast(who: Player, what: Kind(Spell)), effect: Draw(1))")
        );
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
    fn each_players_upkeep_taps_per_counter() {
        assert_eq!(
            trig_builtin(
                "At the beginning of each player's upkeep, that player taps an untapped artifact, creature, or land they control for each fade counter on ~."
            )
            .as_deref(),
            Some(
                "Triggered(event: StepBegins(at: Beginning(Upkeep), whose: EachPlayers), effect: Each(binder: Choose(quantity: Exactly(CounterCount(This, FadeCounter)), filter: And([Permanent, Or([Type(Artifact), Type(Creature), Type(Land)]), ControlledBy(Ref(EventActor)), Not(Status(Tapped))]), by: EventActor), effect: By(EventActor, Tap(It))))"
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
                 event: Enters(And([Type(Land), ControlledBy(Ref(You))])), \
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
                 event: Enters(And([Type(Land), ControlledBy(Ref(You))])), \
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
