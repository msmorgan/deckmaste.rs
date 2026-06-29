//! Shared noun-phrase / count fragment renderers.

use deckmaste_core::Characteristic;
use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Count;
use deckmaste_core::Extremum;
use deckmaste_core::Filter;
use deckmaste_core::ObjectKind;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationFilter;
use deckmaste_core::RoundMode;
use deckmaste_core::Scope;
use deckmaste_core::Selection;
use deckmaste_core::Stat;
use deckmaste_core::StateFilter;
use deckmaste_core::TargetSpec;
use deckmaste_core::Zone;

use super::Ctx;

/// A small count as text: `Literal(n)` -> "n"; callers special-case "a/an".
pub(super) fn count(c: &Count) -> String {
    match c {
        Count::Literal(n) => n.to_string(),
        Count::X => "X".to_string(),
        Count::Damage(r) => format!(
            "damage marked on {}",
            reference(
                r,
                &Ctx {
                    subject: "it",
                    targets: &[]
                }
            )
        ),
        // [CR#107.1] value arithmetic.
        Count::Plus(a, b) => format!("{} plus {}", count(a), count(b)),
        Count::Minus(a, b) => format!("{} minus {}", count(a), count(b)),
        Count::Times(a, b) => format!("{} times {}", count(a), count(b)),
        Count::Max(a, b) => format!("the greater of {} and {}", count(a), count(b)),
        Count::Min(a, b) => format!("the lesser of {} and {}", count(a), count(b)),
        Count::Half(mode, inner) => {
            let rounding = match mode {
                RoundMode::RoundUp => "rounded up",
                RoundMode::RoundDown => "rounded down",
            };
            format!("half {}, {rounding}", count(inner))
        }
        // [CR#107.3] distinct-union count (Domain / Coven / Tarmogoyf).
        Count::CountDistinct(axis, filter) => {
            format!(
                "the number of {} among {}",
                characteristic_word(*axis),
                filter_noun(filter)
            )
        }
        // A remembered count macro (e.g. `Domain`): prefer its own template,
        // else render the expansion structurally.
        Count::Expanded(e) => super::template::expanded(e, "it").unwrap_or_else(|| count(&e.value)),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The plural noun for a [`Characteristic`] axis, used by the distinct-count
/// phrase ("the number of subtypes among …").
fn characteristic_word(axis: Characteristic) -> &'static str {
    match axis {
        Characteristic::Colors => "colors",
        Characteristic::Types => "types",
        Characteristic::Subtypes => "subtypes",
        Characteristic::Supertypes => "supertypes",
        Characteristic::Power => "powers",
        Characteristic::Toughness => "toughnesses",
        Characteristic::Defense => "defenses",
        Characteristic::ManaCost => "mana costs",
        Characteristic::Name => "names",
    }
}

/// A `Selection` as the object of a verb.
pub(super) fn selection(sel: &Selection, ctx: &Ctx) -> String {
    match sel {
        Selection::Ref(r) => reference(r, ctx),
        Selection::Each(f) | Selection::Filter(f) => format!("each {}", filter_noun(f)),
        // [CR#107.1] the extremal element: "the creature with the greatest
        // power". The projection's axis is named when it is a simple stat read.
        Selection::Pick { op, of, by } => {
            let extreme = match op {
                Extremum::Greatest => "greatest",
                Extremum::Least => "least",
            };
            let axis = match by.as_ref() {
                Count::StatOf(_, Stat::Power) => " power",
                Count::StatOf(_, Stat::Toughness) => " toughness",
                _ => "",
            };
            format!("the {} with the {extreme}{axis}", filter_noun(of))
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Reference` as a noun phrase.
pub(super) fn reference(r: &Reference, ctx: &Ctx) -> String {
    match r {
        Reference::Target(i) => target_phrase(*i, ctx),
        Reference::This => ctx.subject.to_string(),
        Reference::You => "you".to_string(),
        // The triggering event's roles ([CR#603.2e,608.2k]). Agent/patient
        // render as the generic anaphor "it" (no type info at this layer);
        // `ThatObject` is the agent's migration alias.
        Reference::EventAgent | Reference::EventPatient | Reference::ThatObject => "it".to_string(),
        // The responsible player ("that player"); `ThatPlayer` is its alias.
        Reference::EventActor | Reference::ThatPlayer => "that player".to_string(),
        // The combat defender ([CR#506.2]) — always a player.
        Reference::DefendingPlayer => "the defending player".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// Resolve `Target(i)` against the ability's i-th `TargetSpec`.
fn target_phrase(i: usize, ctx: &Ctx) -> String {
    match ctx.targets.get(i) {
        Some(spec) => target_spec(spec),
        None => "[unrendered: missing target]".to_string(),
    }
}

/// A `TargetSpec` as the phrase naming what it points at.
pub(super) fn target_spec(spec: &TargetSpec) -> String {
    match spec {
        // Macro-provenance: prefer the invocation's own template (e.g. AnyTarget
        // -> "any target"); fall back to the expansion. Target templates name
        // the target, never the host, so the subject is irrelevant here.
        TargetSpec::Expanded(exp) => {
            super::template::expanded(exp, "").unwrap_or_else(|| target_spec(&exp.value))
        }
        TargetSpec::Target(q, filter) if q.is_one() => {
            format!("target {}", filter_noun(filter))
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A simple noun for a filter, used in target phrases and "each <noun>"
/// selection phrases.  Prefers a filter macro's own noun template ("creature",
/// "player", ...); falls back to structural derivation ([`find_card_type`] /
/// [`strip_expanded`]) for hand-built (un-wrapped) filters.
pub(super) fn filter_noun(filter: &Filter) -> String {
    if let Filter::Expanded(exp) = filter
        && let Some(noun) = super::template::expanded(exp, "")
    {
        return noun;
    }
    if let Some(t) = find_card_type(filter) {
        return super::card::type_str(t).to_lowercase();
    }
    match strip_expanded(filter) {
        Filter::Characteristic(CharacteristicFilter::ColorIs(c)) => {
            super::effect::color_word(*c).to_string()
        }
        Filter::Kind(ObjectKind::Player) => "player".to_string(),
        // An ability on the stack ([CR#602.2a,603.3]): "counter target ability".
        Filter::Kind(ObjectKind::Ability) => "ability".to_string(),
        // Team-relative player nouns ([CR#102.3]): "target opponent" /
        // "target teammate" (relative to the carrier's controller).
        Filter::Relation(RelationFilter::OpponentOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
        {
            "opponent".to_string()
        }
        Filter::Relation(RelationFilter::TeammateOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
        {
            "teammate".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

// ── Static-ability subject phrases ──────────────────────────────────────────

/// A zone as the noun used in "in your <zone>" / "from your <zone>" phrases.
pub(super) fn zone_word(z: Zone) -> &'static str {
    match z {
        Zone::Battlefield => "battlefield",
        Zone::Command => "command zone",
        Zone::Exile => "exile",
        Zone::Graveyard => "graveyard",
        Zone::Hand => "hand",
        Zone::Library => "library",
        Zone::Stack => "stack",
    }
}

/// See through macro-provenance wrappers on a `Filter`.
pub(super) fn strip_expanded(f: &Filter) -> &Filter {
    match f {
        Filter::Expanded(e) => strip_expanded(&e.value),
        other => other,
    }
}

/// `(subject phrase, is_plural)`.  `ctx` resolves `Reference::This` and
/// `Reference::Target(i)`.
pub(super) fn scope_subject_agreed(scope: &Scope, ctx: &super::Ctx) -> (String, bool) {
    match scope {
        Scope::Matching(f) => (filter_subject(f), true),
        Scope::Of(r) => (reference_subject(r, ctx), false),
        Scope::These(rs) => (
            rs.iter()
                .map(|r| reference_subject(r, ctx))
                .collect::<Vec<_>>()
                .join(" and "),
            rs.len() != 1,
        ),
    }
}

/// A `Reference` as a static-effect subject phrase (capitalized for
/// sentence-start use).
fn reference_subject(r: &Reference, ctx: &super::Ctx) -> String {
    match r {
        Reference::This => ctx.subject.to_string(),
        Reference::Target(i) => match ctx.targets.get(*i) {
            Some(spec) => capitalize(&target_spec(spec)),
            None => "[unrendered: missing target]".to_string(),
        },
        // Aura host: "Enchanted creature gets +2/+2." (matches deontic_subject).
        Reference::AttachHostOf(inner) if matches!(**inner, Reference::This) => {
            "Enchanted creature".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// Capitalize the first character of a string.
pub(super) fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// A `Filter` as a plural subject noun phrase: "Creatures you control",
/// "Other creatures you control", "Creatures your opponents control".
///
/// The Creature filter macro expands as
/// `Expanded(value=AllOf([Expanded(Permanent),
/// Characteristic(Type(Creature))]))`. `flatten_all_of` and `find_card_type`
/// see through both layers.
pub(super) fn filter_subject(f: &Filter) -> String {
    let parts = flatten_all_of(f);
    let mut other = false;
    let mut base = "Permanents".to_string();
    let mut control: Option<String> = None;
    for p in parts {
        match strip_expanded(p) {
            Filter::Characteristic(CharacteristicFilter::Type(t)) => {
                base = format!("{}s", super::card::type_str(*t));
            }
            Filter::Not(inner) if strip_expanded(inner).is_this() => {
                other = true;
            }
            Filter::Relation(RelationFilter::ControlledBy(inner)) => {
                control = Some(controller_phrase(inner));
            }
            // The Creature macro expands to AllOf([Expanded(Permanent),
            // Characteristic(Type(Creature))]); check whether this part holds
            // a card type buried in a nested AllOf.
            stripped => {
                if let Some(t) = find_card_type(stripped) {
                    base = format!("{}s", super::card::type_str(t));
                }
            }
        }
    }
    let mut s = String::new();
    if other {
        s.push_str("Other ");
        s.push_str(&base.to_lowercase());
    } else {
        s.push_str(&base);
    }
    if let Some(c) = control {
        s.push(' ');
        s.push_str(&c);
    }
    s
}

/// Recursively search a stripped filter for a `Characteristic(Type(t))`.
/// Used to find the type name inside a macro-expanded Creature/Land/etc.
/// filter.
pub(super) fn find_card_type(f: &Filter) -> Option<deckmaste_core::Type> {
    match strip_expanded(f) {
        Filter::Characteristic(CharacteristicFilter::Type(t)) => Some(*t),
        Filter::AllOf(vs) => vs.iter().find_map(find_card_type),
        _ => None,
    }
}

fn controller_phrase(f: &Filter) -> String {
    match strip_expanded(f) {
        Filter::Ref(Reference::You) => "you control".to_string(),
        Filter::Relation(RelationFilter::OpponentOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
        {
            "your opponents control".to_string()
        }
        Filter::Relation(RelationFilter::TeammateOf(inner))
            if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
        {
            "your teammates control".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

fn flatten_all_of(f: &Filter) -> Vec<&Filter> {
    match strip_expanded(f) {
        Filter::AllOf(v) => v.iter().collect(),
        single => vec![single],
    }
}

// ── PutInLibrary helpers ─────────────────────────────────────────────────────

/// A `Selection` as the object of "put __": "2 cards from your hand".
pub(super) fn selection_object(sel: &Selection, ctx: &Ctx) -> String {
    match sel {
        Selection::Choose(q, filter) => format!("{} {}", quantity(q), filter_object(filter)),
        other => selection(other, ctx),
    }
}

fn quantity(q: &Quantity) -> String {
    // `Quantity` is one `Range(lo, hi)` primitive (seen through a remembered
    // macro by `bounds`). An exactly-N range renders as the count; richer
    // phrasings ("up to N", "any number of") are a renderer follow-up.
    match q.bounds() {
        (Some(lo), Some(hi)) if lo == hi => count(lo),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Filter` as the object noun for cards: "cards from your hand".
fn filter_object(f: &Filter) -> String {
    let parts = flatten_all_of(f);
    let mut zone = "";
    let mut yours = false;
    for p in parts {
        match strip_expanded(p) {
            Filter::State(StateFilter::InZone(Zone::Hand)) => zone = "hand",
            Filter::Relation(RelationFilter::Owner(inner))
                if matches!(strip_expanded(inner), Filter::Ref(Reference::You)) =>
            {
                yours = true;
            }
            _ => {}
        }
    }
    match (zone, yours) {
        ("hand", true) => "cards from your hand".to_string(),
        ("hand", false) => "cards from a hand".to_string(),
        _ => format!("cards [unrendered: {f:?}]"),
    }
}

/// Library position from a `Count`: 0 -> "top", else "the bottom".
pub(super) fn library_position(c: &Count) -> String {
    match c {
        Count::Literal(0) => "top".to_string(),
        _ => "the bottom".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Ctx<'static> {
        Ctx {
            subject: "Grizzly Bears",
            targets: &[],
        }
    }

    /// The provenance-explicit event roles render as English anaphora: the
    /// agent/patient as "it", the actor as "that player", and the combat
    /// defender as "the defending player" ([CR#603.2e,608.2k,506.2]). The
    /// legacy `ThatObject`/`ThatPlayer` aliases render like their roles.
    #[test]
    fn event_role_references_render() {
        let c = ctx();
        assert_eq!(reference(&Reference::EventAgent, &c), "it");
        assert_eq!(reference(&Reference::EventPatient, &c), "it");
        assert_eq!(reference(&Reference::ThatObject, &c), "it");
        assert_eq!(reference(&Reference::EventActor, &c), "that player");
        assert_eq!(reference(&Reference::ThatPlayer, &c), "that player");
        assert_eq!(
            reference(&Reference::DefendingPlayer, &c),
            "the defending player"
        );
    }

    /// The new filter nouns: an ability on the stack, and the team-relative
    /// player relations relative to "you".
    #[test]
    fn filter_noun_renders_ability_and_team_relative_players() {
        let you = || Box::new(Filter::Ref(Reference::You));
        assert_eq!(filter_noun(&Filter::Kind(ObjectKind::Ability)), "ability");
        assert_eq!(
            filter_noun(&Filter::Relation(RelationFilter::OpponentOf(you()))),
            "opponent"
        );
        assert_eq!(
            filter_noun(&Filter::Relation(RelationFilter::TeammateOf(you()))),
            "teammate"
        );
    }

    /// "creatures your teammates control" — the team-relative controller phrase
    /// inside a plural subject ([CR#102.3]).
    #[test]
    fn filter_subject_renders_teammate_controller_phrase() {
        let f = Filter::AllOf(vec![
            Filter::Characteristic(CharacteristicFilter::Type(deckmaste_core::Type::Creature)),
            Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Relation(
                RelationFilter::TeammateOf(Box::new(Filter::Ref(Reference::You))),
            )))),
        ]);
        assert_eq!(filter_subject(&f), "Creatures your teammates control");
    }
}
