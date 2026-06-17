//! Replacement-effect parser ([CR#614]): the closed template list, rendered as
//! a `Static(effects: [Replacement(...)])` ability — replacements are carried
//! by a static ([CR#614.1], a continuous effect that modifies how an event
//! happens). Two templates:
//!
//! * **"As ~ enters, <effect>."** — a self-replacement applied at entry
//!   ([CR#614.1c,614.12]): `Replacement(AsEnters(<effect>))`. The shared
//!   [`effect`] grammar supplies the effect; an effect that *targets* declines
//!   (a self-replacement has no announce list to declare targets on).
//! * **"If [subject] would [die|enter], [effect] instead."** — replace
//!   ([CR#614.1a]): `Replacement(Instead(would: <event>, instead: <effect>))`.
//!   The event clause shares [`triggered_ability::parse_event`]; the effect
//!   shares [`effect`].
//!
//! `~ enters tapped.` (the bare self case) is already structured by the
//! mana-ability parser; this module declines it so the two never both match.
//! Declines (`Ok(None)`) on spells and on anything its productions don't fully
//! cover — a wrong replacement would graduate a wrong card.

use crate::parsers::effect::{self};
use crate::parsers::filter;
use crate::parsers::triggered_ability;
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    Ok(parse(line, ctx))
}

fn parse(line: &str, ctx: &ResolveCtx) -> Option<String> {
    // [CR#614.1]: a replacement is a continuous effect of a permanent's static
    // ability (or a one-shot's during-resolution clause). The permanent-side
    // templates here decline on spells.
    if ctx.kind == CardKind::Spell {
        return None;
    }
    parse_as_enters(line, ctx)
        .or_else(|| parse_instead(line, ctx))
        .or_else(|| parse_tapped_unless(line))
        .or_else(|| parse_tapped_if(line))
}

/// "As <subject> enters, <effect>." → `Replacement(AsEnters(<effect>))`. Only
/// the self subject (`~`) is structured — `AsEnters` is a *self*-replacement
/// ([CR#614.12]), folded into this object's own entry. The effect clause keeps
/// its trailing period (the shared grammar requires it) and must declare no
/// targets (no announce list exists at a replacement).
fn parse_as_enters(line: &str, ctx: &ResolveCtx) -> Option<String> {
    let effect_clause = line.strip_prefix("As ~ enters, ")?;
    let parsed = effect::parse_clause(effect_clause, ctx)?;
    if !parsed.targets.is_empty() {
        return None;
    }
    Some(format!(
        "Static(effects: [Replacement(AsEnters({}))])",
        parsed.effect
    ))
}

/// "If [subject] would [die|enter], [effect] instead." →
/// `Replacement(Instead(would: <event>, instead: <effect>))`. The event is the
/// subject plus its base verb mapped to the present-tense form the shared
/// [`triggered_ability::parse_event`] grammar reads ("die" → "dies", "enter"
/// → "enters"); the effect reuses the shared grammar and must not target (a
/// replacement declares no announce list). Declines unless both halves parse.
fn parse_instead(line: &str, ctx: &ResolveCtx) -> Option<String> {
    let body = line.strip_suffix('.')?;
    let rest = body.strip_prefix("If ")?;
    // "[subject] would [pred], [effect] instead": the verb "would" splits the
    // conditional event from its tail; "instead" terminates the effect.
    let (subject, tail) = rest.split_once(" would ")?;
    let (would_pred, effect_clause) = tail.split_once(", ")?;
    let effect_clause = effect_clause.strip_suffix(" instead")?;
    // Map the base verb after "would" to the event grammar's present tense.
    let verb_clause = match would_pred {
        "die" => "dies",
        "enter" | "enter the battlefield" => "enters",
        _ => return None,
    };
    let event = triggered_ability::parse_event(&format!("{subject} {verb_clause}"))?;
    // The shared effect grammar requires the trailing period the "instead"
    // suffix consumed; restore it.
    let parsed = effect::parse_clause(&format!("{effect_clause}."), ctx)?;
    if !parsed.targets.is_empty() {
        return None;
    }
    Some(format!(
        "Static(effects: [Replacement(Instead(would: {event}, instead: {}))])",
        parsed.effect
    ))
}

/// `~ enters tapped unless <condition>.` ([CR#614.1d]) → an `AsEnters`
/// self-replacement that taps the permanent exactly when the unless-condition
/// does NOT hold: `Replacement(AsEnters(If(condition: Not(<cond>), then:
/// Tap(This))))`. The dual-land "comes into play tapped unless you control …"
/// family. The condition reuses the [`Compare`]/[`Exists`] grammar over a board
/// census ([`parse_board_condition`]); a condition the census grammar can't
/// ground (a player-life or opponent-count read with no `Count` primitive yet)
/// declines the whole line — a wrong gate would graduate a wrong card.
fn parse_tapped_unless(line: &str) -> Option<String> {
    let clause = line
        .strip_prefix("~ enters tapped unless ")?
        .strip_suffix('.')?;
    let condition = parse_board_condition(clause)?;
    Some(format!(
        "Static(effects: [Replacement(AsEnters(If(condition: Not({condition}), \
         then: Tap(This))))])"
    ))
}

/// `If <condition>, ~ enters tapped.` ([CR#614.1d]) — the reversed wording
/// where the "if" directly gates the tap (no negation):
/// `Replacement(AsEnters(If( condition: <cond>, then: Tap(This))))`.
fn parse_tapped_if(line: &str) -> Option<String> {
    let clause = line
        .strip_prefix("If ")?
        .strip_suffix(", ~ enters tapped.")?;
    let condition = parse_board_condition(clause)?;
    Some(format!(
        "Static(effects: [Replacement(AsEnters(If(condition: {condition}, \
         then: Tap(This))))])"
    ))
}

/// A board-census condition the dual-land gate reads: "you control <obj>" /
/// "your opponents control <obj>", either an existence ("a basic land" →
/// [`Condition::Exists`]) or a count comparison ("two or more other lands" →
/// [`Condition::Compare`] over [`Count::CountOf`]). Returns the `Condition`
/// RON, or `None` when the controller phrase or the object description isn't
/// one this grammar grounds.
fn parse_board_condition(clause: &str) -> Option<String> {
    let (controller, object) = strip_controller(clause)?;
    // A leading count word ("two or more …", "eight or more …") makes this a
    // Compare; otherwise a determiner ("a …") makes it an Exists.
    if let Some((cmp, n, subject)) = strip_count(object) {
        let filter = object_filter(subject, &controller)?;
        Some(format!("Compare(CountOf({filter}), {cmp}, {n})"))
    } else {
        let subject = object
            .strip_prefix("a ")
            .or_else(|| object.strip_prefix("an "))?;
        let filter = object_filter(subject, &controller)?;
        Some(format!("Exists({filter})"))
    }
}

/// Peel the controlling-player clause off the front, returning the
/// `ControlledBy(...)` atom and the trailing object description. "you control
/// …" → controller = you; "your opponents control …" → controller = your
/// opponents. Declines any other controller phrase.
fn strip_controller(clause: &str) -> Option<(String, &str)> {
    if let Some(rest) = clause.strip_prefix("you control ") {
        Some(("ControlledBy(Ref(You))".to_owned(), rest))
    } else if let Some(rest) = clause.strip_prefix("your opponents control ") {
        Some(("ControlledBy(OpponentOf(Ref(You)))".to_owned(), rest))
    } else {
        None
    }
}

/// Peel a leading count comparison ("two or more <obj>", "two or fewer <obj>",
/// "eight or more <obj>") into (`Cmp` ident, number, remaining object). The
/// comparator words map "or more" → `AtLeast`, "or fewer"/"or less" → `AtMost`.
fn strip_count(object: &str) -> Option<(&'static str, u32, &str)> {
    let (word, rest) = object.split_once(' ')?;
    let n = count_word(word)?;
    let (cmp, subject) = if let Some(r) = rest.strip_prefix("or more ") {
        ("AtLeast", r)
    } else if let Some(r) = rest.strip_prefix("or fewer ") {
        ("AtMost", r)
    } else if let Some(r) = rest.strip_prefix("or less ") {
        ("AtMost", r)
    } else {
        return None;
    };
    Some((cmp, n, subject))
}

/// An English cardinal word → its value, for the threshold counts in the
/// dual-land family ("two or more …", "eight or more …"). Broader than the
/// shared [`effect::number_word`] (which tops out at "three"): the count
/// thresholds here run up to "eight". A bare digit string also parses.
fn count_word(word: &str) -> Option<u32> {
    Some(match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        digits => digits.parse().ok()?,
    })
}

/// Build the object `Filter` RON for a dual-land subject phrase plus its
/// controller atom. Covers the descriptions the shared [`filter::parse_phrase`]
/// doesn't: the `basic`/`legendary` supertype adjectives and subtype
/// disjunctions ("a Swamp or a Mountain", "a Mount or Vehicle"). A single
/// subtype/type head with no special adjective reuses `parse_phrase` (with the
/// controller clause appended) so the shared head vocabulary stays the one
/// source of truth.
fn object_filter(subject: &str, controller: &str) -> Option<String> {
    let subject = subject.trim();
    // Subtype disjunction: "X or Y" / "X or a Y" / "X or an Y" — each side a
    // bare subtype. A `OneOf` of the `Subtype` atoms, scoped to `Permanent`
    // ([CR#109.2]) and the controller.
    if let Some(disjunction) = subtype_disjunction(subject) {
        return Some(format!(
            "AllOf([OneOf([{disjunction}]), Permanent, {controller}])"
        ));
    }
    // Supertype adjective: "basic <head>" / "legendary <head>".
    if let Some((supertype, head)) = strip_supertype(subject) {
        let type_atom = filter::type_head_atom(head)?;
        return Some(format!(
            "AllOf([{type_atom}, Supertype({supertype}), {controller}])"
        ));
    }
    // Otherwise the shared phrase parser, with the controller clause appended so
    // its postfix grammar emits the same `ControlledBy(...)` atom (and "other"
    // is handled by its prefix grammar). The controller suffix is re-derived
    // from the atom we already chose.
    let suffix = controller_suffix(controller)?;
    filter::parse_phrase(&format!("{subject} {suffix}"))
}

/// The English postfix clause whose [`filter::parse_phrase`] atom equals
/// `controller`, so a single-head subject can route through the shared phrase
/// parser. Only the two dual-land controllers are mapped.
fn controller_suffix(controller: &str) -> Option<&'static str> {
    match controller {
        "ControlledBy(Ref(You))" => Some("you control"),
        "ControlledBy(OpponentOf(Ref(You)))" => Some("your opponents control"),
        _ => None,
    }
}

/// A leading `basic`/`legendary` supertype adjective → (`Supertype` ident,
/// remaining head). Only these two supertypes appear in the dual-land family.
fn strip_supertype(subject: &str) -> Option<(&'static str, &str)> {
    let (first, rest) = subject.split_once(' ')?;
    let supertype = match first.to_ascii_lowercase().as_str() {
        "basic" => "Basic",
        "legendary" => "Legendary",
        _ => return None,
    };
    Some((supertype, rest.trim_start()))
}

/// "X or Y" / "X or a Y" / "X or an Y" where each side is a single catalog
/// subtype → the comma-joined `Subtype("X"), Subtype("Y")` body of a `OneOf`.
/// Declines unless BOTH sides are bare single-word subtypes (so a type-noun or
/// a multi-word side falls through to the single-head path / declines).
fn subtype_disjunction(subject: &str) -> Option<String> {
    let (left, right) = subject.split_once(" or ")?;
    let right = right
        .strip_prefix("a ")
        .or_else(|| right.strip_prefix("an "))
        .unwrap_or(right);
    let l = subtype_atom(left.trim())?;
    let r = subtype_atom(right.trim())?;
    Some(format!("{l}, {r}"))
}

/// A single bare subtype word → its `Subtype("…")` atom, or `None` if the word
/// is empty / multi-word / not a known catalog subtype.
fn subtype_atom(word: &str) -> Option<String> {
    if word.is_empty() || word.contains(' ') || !filter::is_subtype(word) {
        return None;
    }
    Some(format!(
        "Subtype(\"{}\")",
        crate::ident::to_rust_ident(word)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rep(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    #[test]
    fn as_enters_self_pump_one_shot() {
        // "As ~ enters" with a durational pump body — a self-replacement that
        // augments the entry with a one-shot continuous effect.
        assert_eq!(
            rep("As ~ enters, ~ gets +1/+1 until end of turn.").as_deref(),
            Some(
                "Static(effects: [Replacement(AsEnters(Continuously(effect: \
                 Modify(of: Of(This), changes: [AddPower(1), AddToughness(1)]), \
                 duration: FixedUntil(EndOfTurn))))])"
            )
        );
    }

    #[test]
    fn instead_self_dies_draw() {
        // "If ~ would die, draw a card instead" — the enters/dies event grammar
        // supplies `ThisDies`, the effect grammar supplies `Draw(1)`.
        assert_eq!(
            rep("If ~ would die, draw a card instead.").as_deref(),
            Some("Static(effects: [Replacement(Instead(would: ThisDies, instead: Draw(1)))])")
        );
    }

    #[test]
    fn instead_filtered_subject_dies() {
        assert_eq!(
            rep("If a creature you control would die, you gain 1 life instead.").as_deref(),
            Some(
                "Static(effects: [Replacement(Instead(would: \
                 Dies(AllOf([Creature, ControlledBy(Ref(You))])), instead: GainLife(1)))])"
            )
        );
    }

    #[test]
    fn declines_spells() {
        assert!(
            resolve_line(
                "If ~ would die, draw a card instead.",
                &crate::parsers::test_ctx::ctx(CardKind::Spell)
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line(
                "As ~ enters, ~ gets +1/+1 until end of turn.",
                &crate::parsers::test_ctx::ctx(CardKind::Spell)
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn declines_bare_enters_tapped() {
        // Already structured by the mana-ability parser; this module abstains so
        // the two never both match the same line.
        assert!(rep("~ enters tapped.").is_none());
    }

    // --- enters-tapped-unless dual-land family ([CR#614.1d]) ---

    /// The `AsEnters(If(...))` wrapper a tapped-unless line graduates to: it
    /// enters tapped exactly when the unless-condition does NOT hold.
    fn tapped_unless(condition: &str) -> String {
        format!(
            "Static(effects: [Replacement(AsEnters(If(condition: Not({condition}), \
             then: Tap(This))))])"
        )
    }

    #[test]
    fn unless_control_count_lands() {
        // "two or more other lands" → Compare(other lands you control, AtLeast, 2).
        assert_eq!(
            rep("~ enters tapped unless you control two or more other lands.").as_deref(),
            Some(
                tapped_unless(
                    "Compare(CountOf(AllOf([Type(Land), Not(Ref(This)), \
                     ControlledBy(Ref(You))])), AtLeast, 2)"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_count_fewer() {
        // "two or fewer other lands" → AtMost.
        assert_eq!(
            rep("~ enters tapped unless you control two or fewer other lands.").as_deref(),
            Some(
                tapped_unless(
                    "Compare(CountOf(AllOf([Type(Land), Not(Ref(This)), \
                     ControlledBy(Ref(You))])), AtMost, 2)"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_count_basic_lands() {
        // "two or more basic lands" — the Basic supertype, no "other".
        assert_eq!(
            rep("~ enters tapped unless you control two or more basic lands.").as_deref(),
            Some(
                tapped_unless(
                    "Compare(CountOf(AllOf([Type(Land), Supertype(Basic), \
                     ControlledBy(Ref(You))])), AtLeast, 2)"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_count_other_subtype() {
        // "three or more other Swamps" — the subtype head + "other".
        assert_eq!(
            rep("~ enters tapped unless you control three or more other Swamps.").as_deref(),
            Some(
                tapped_unless(
                    "Compare(CountOf(AllOf([Permanent, Subtype(\"Swamp\"), Not(Ref(This)), \
                     ControlledBy(Ref(You))])), AtLeast, 3)"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_a_basic_land() {
        // "a basic land" (a determiner, no count) → Exists.
        assert_eq!(
            rep("~ enters tapped unless you control a basic land.").as_deref(),
            Some(
                tapped_unless(
                    "Exists(AllOf([Type(Land), Supertype(Basic), ControlledBy(Ref(You))]))"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_a_legendary_creature() {
        assert_eq!(
            rep("~ enters tapped unless you control a legendary creature.").as_deref(),
            Some(
                tapped_unless(
                    "Exists(AllOf([Creature, Supertype(Legendary), ControlledBy(Ref(You))]))"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_a_subtype() {
        assert_eq!(
            rep("~ enters tapped unless you control a Swamp.").as_deref(),
            Some(
                tapped_unless(
                    "Exists(AllOf([Permanent, Subtype(\"Swamp\"), ControlledBy(Ref(You))]))"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_subtype_disjunction() {
        // "a Swamp or a Mountain" → OneOf two subtype filters, each you-control.
        assert_eq!(
            rep("~ enters tapped unless you control a Swamp or a Mountain.").as_deref(),
            Some(
                tapped_unless(
                    "Exists(AllOf([OneOf([Subtype(\"Swamp\"), Subtype(\"Mountain\")]), \
                     Permanent, ControlledBy(Ref(You))]))"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_control_typeline_disjunction() {
        // "a Mount or Vehicle" (no repeated determiner) → OneOf.
        assert_eq!(
            rep("~ enters tapped unless you control a Mount or Vehicle.").as_deref(),
            Some(
                tapped_unless(
                    "Exists(AllOf([OneOf([Subtype(\"Mount\"), Subtype(\"Vehicle\")]), \
                     Permanent, ControlledBy(Ref(You))]))"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn unless_opponents_control_count_lands() {
        assert_eq!(
            rep("~ enters tapped unless your opponents control eight or more lands.").as_deref(),
            Some(
                tapped_unless(
                    "Compare(CountOf(AllOf([Type(Land), \
                     ControlledBy(OpponentOf(Ref(You)))])), AtLeast, 8)"
                )
                .as_str()
            )
        );
    }

    #[test]
    fn reversed_if_control_enters_tapped() {
        // "If you control two or more other lands, ~ enters tapped." — the
        // condition is NOT negated (the if directly gates the tap).
        assert_eq!(
            rep("If you control two or more other lands, ~ enters tapped.").as_deref(),
            Some(
                "Static(effects: [Replacement(AsEnters(If(condition: \
                 Compare(CountOf(AllOf([Type(Land), Not(Ref(This)), ControlledBy(Ref(You))])), \
                 AtLeast, 2), then: Tap(This))))])"
            )
        );
    }

    #[test]
    fn declines_unbuilt_count_conditions() {
        // "two or more opponents" needs an opponent-count Count primitive (unbuilt).
        assert!(rep("~ enters tapped unless you have two or more opponents.").is_none());
        // "a player has 13 or less life" needs a player-life Count primitive (unbuilt).
        assert!(rep("~ enters tapped unless a player has 13 or less life.").is_none());
    }

    #[test]
    fn declines_unbuilt_on_spell() {
        assert!(
            resolve_line(
                "~ enters tapped unless you control a basic land.",
                &crate::parsers::test_ctx::ctx(CardKind::Spell)
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn declines_unparseable_event_or_effect() {
        // Unknown event verb ("would be exiled").
        assert!(rep("If ~ would be exiled, draw a card instead.").is_none());
        // Unknown effect ("exile it" is not in the effect grammar yet).
        assert!(rep("If ~ would die, exile it instead.").is_none());
        // A targeting effect has nowhere to declare its target in a replacement.
        assert!(rep("As ~ enters, destroy target creature.").is_none());
        // Not a replacement line at all.
        assert!(rep("Draw a card.").is_none());
        // "If … would …" present but missing the "instead" terminator.
        assert!(rep("If ~ would die, draw a card.").is_none());
    }
}
