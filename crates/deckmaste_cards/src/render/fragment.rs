//! Shared noun-phrase / count fragment renderers.

use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Count;
use deckmaste_core::Filter;
use deckmaste_core::ObjectKind;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationFilter;
use deckmaste_core::Scope;
use deckmaste_core::Selection;
use deckmaste_core::StateFilter;
use deckmaste_core::TargetSpec;
use deckmaste_core::Zone;

use super::Ctx;

/// A small count as text: `Literal(n)` -> "n"; callers special-case "a/an".
pub(super) fn count(c: &Count) -> String {
    match c {
        Count::Literal(n) => n.to_string(),
        Count::X => "X".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Selection` as the object of a verb.
pub(super) fn selection(sel: &Selection, ctx: &Ctx) -> String {
    match sel {
        Selection::Ref(r) => reference(r, ctx),
        Selection::Each(f) | Selection::Filter(f) => format!("each {}", filter_noun(f)),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Reference` as a noun phrase.
pub(super) fn reference(r: &Reference, ctx: &Ctx) -> String {
    match r {
        Reference::Target(i) => target_phrase(*i, ctx),
        Reference::This => ctx.subject.to_string(),
        Reference::You => "you".to_string(),
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
        // Unwrap macro expansion layers to reach the real Target.
        TargetSpec::Expanded(exp) => target_spec(&exp.value),
        TargetSpec::Target(_q, filter) if is_any_target(filter) => "any target".to_string(),
        TargetSpec::Target(Quantity::Exactly(Count::Literal(1)), filter) => {
            format!("target {}", filter_noun(filter))
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A simple noun for a filter, used in target phrases and "each <noun>"
/// selection phrases.  Sees through `Expanded` wrappers via [`find_card_type`]
/// and [`strip_expanded`].
fn filter_noun(filter: &Filter) -> String {
    if let Some(t) = find_card_type(filter) {
        return super::card::type_str(t).to_lowercase();
    }
    match strip_expanded(filter) {
        Filter::Kind(ObjectKind::Player) => "player".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// Detect the expanded `AnyTarget` filter. The `AnyTarget` macro expands to
/// `Target(Exactly(Literal(1)), OneOf([Battle, Creature, Planeswalker,
/// Player]))`. The Battle/Creature/Planeswalker members are each
/// `Expanded(Expansion { value: AllOf([..., Characteristic(Type(...))]) })` and
/// the Player member is `Expanded(Expansion { value: Kind(Player) })`.
///
/// A pragmatic check: the filter is a `OneOf` containing at least one member
/// whose resolved value is `Kind(Player)` — this is unique to `AnyTarget` among
/// the `TargetSpec` macros.
fn is_any_target(filter: &Filter) -> bool {
    let Filter::OneOf(members) = filter else { return false };
    if members.len() != 4 {
        return false;
    }
    // The Player entry is the last one: Expanded(Expansion { value: Kind(Player) })
    members.iter().any(|m| match m {
        Filter::Expanded(exp) => matches!(*exp.value, Filter::Kind(ObjectKind::Player)),
        _ => false,
    })
}

// ── Static-ability subject phrases ──────────────────────────────────────────

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
fn capitalize(s: &str) -> String {
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
            Filter::Not(inner) if matches!(strip_expanded(inner), Filter::Ref(Reference::This)) => {
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
    match q {
        Quantity::Exactly(c) => count(c),
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
