//! Shared noun-phrase / count fragment renderers.

use deckmaste_core::Count;
use deckmaste_core::Filter;
use deckmaste_core::ObjectKind;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::TargetSpec;

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

/// A simple noun for a filter, used in target phrases.
fn filter_noun(filter: &Filter) -> String {
    match filter {
        Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(t)) => {
            format!("{t:?}").to_lowercase()
        }
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
