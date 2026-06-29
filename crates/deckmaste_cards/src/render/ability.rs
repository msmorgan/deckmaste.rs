//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_core::Ability;
use deckmaste_core::CollectionOp;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::Reference;
use deckmaste_core::StateFilterEvent;
use deckmaste_core::StaticAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Zone;

use super::CardView;
use super::Ctx;
use super::effect;

/// "When/Whenever <event>, [if <cond>,] <effect>.", with a leading
/// "While ~ is in your <zone>," qualifier for a graveyard/hand-functioning
/// trigger ([CR#113.6,113.6b]).
pub(super) fn triggered(t: &TriggeredAbility, view: &CardView) -> String {
    // Targeting lives on an `Effect::Targeted` wrapper, which the effect walk
    // rebinds `ctx.targets` from ([CR#115.1]).
    let ctx = Ctx {
        subject: view.name,
        targets: &[],
    };
    let (lead, clause) = event_clause(&t.event, &ctx);
    let body = lower_first(&effect::effect(&t.effect, &ctx));
    // Intervening-if ([CR#603.4]): "…, if <cond>, <effect>."
    let cond = match &t.condition {
        Some(c) => format!("if {}, ", super::condition::condition(c, &ctx)),
        None => String::new(),
    };
    let trig = format!("{lead} {clause}, {cond}{body}");
    from_zone_qualified(t.from, view.name, trig)
}

/// Prefix a "While ~ is in your <zone>," function-zone qualifier
/// ([CR#113.6,113.6b]) for a non-battlefield `from`; the battlefield default
/// (and `None`) is left bare.
pub(super) fn from_zone_qualified(
    from: Option<deckmaste_core::Zone>,
    subject: &str,
    text: String,
) -> String {
    match from {
        None | Some(Zone::Battlefield) => text,
        Some(zone) => format!(
            "As long as {subject} is in your {}, {}",
            super::fragment::zone_word(zone),
            lower_first(&text)
        ),
    }
}

/// Returns (lead word, the event clause).
/// "When", "Baleful Strix enters" | "Whenever", "Goblin Medics becomes tapped".
pub(super) fn event_clause(e: &Event, ctx: &Ctx) -> (&'static str, String) {
    match e {
        Event::Expanded(exp) => event_clause(&exp.value, ctx),
        Event::ZoneMove {
            what,
            to: Some(Zone::Battlefield),
            from: None,
            ..
        } => (lead_for(what), format!("{} enters", subject_of(what, ctx))),
        Event::ZoneMove {
            what,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            ..
        } => (lead_for(what), format!("{} dies", subject_of(what, ctx))),
        Event::StateBecomes { of, becomes, .. } => (
            "Whenever",
            format!("{} becomes {}", subject_of(of, ctx), state_word(becomes)),
        ),
        other => ("When", format!("[unrendered: {other:?}]")),
    }
}

/// One-shot enters/dies of THIS → "When"; a filtered (non-self) subject →
/// "Whenever".
fn lead_for(what: &Filter) -> &'static str {
    if matches!(
        super::fragment::strip_expanded(what),
        Filter::Ref(Reference::This)
    ) {
        "When"
    } else {
        "Whenever"
    }
}

/// A subject filter as a noun ("Baleful Strix" for the self filter,
/// "a creature" for a Creature macro filter).
fn subject_of(f: &Filter, ctx: &Ctx) -> String {
    let f = super::fragment::strip_expanded(f);
    if f.is_this() {
        return ctx.subject.to_string();
    }
    if let Some(t) = super::fragment::find_card_type(f) {
        return format!("a {}", super::card::type_str(t).to_lowercase());
    }
    format!("[unrendered: {f:?}]")
}

fn state_word(s: &StateFilterEvent) -> &'static str {
    match s {
        StateFilterEvent::Tapped => "tapped",
        StateFilterEvent::Untapped => "untapped",
        StateFilterEvent::Attacking => "attacking",
        StateFilterEvent::Blocked => "blocked",
        _ => "[unrendered]",
    }
}

pub(super) fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_lowercase().chain(c).collect(),
        None => String::new(),
    }
}

// ── Static abilities ─────────────────────────────────────────────────────────

/// Render a `Static` ability's effects, one rules string each. A non-default
/// `from` zone ([CR#113.6,604.3] — a graveyard/hand static) prefixes each
/// effect with an "As long as ~ is in your <zone>," qualifier.
pub(super) fn static_ability(s: &StaticAbility, ctx: &Ctx) -> Vec<String> {
    s.effects
        .iter()
        .filter_map(|e| static_effect(e, ctx))
        .map(|line| from_zone_qualified(s.from, ctx.subject, line))
        .collect()
}

/// Render one `StaticEffect` as a period-terminated sentence, or `None` for
/// effects that produce no text on their own.
pub(super) fn static_effect(e: &StaticEffect, ctx: &Ctx) -> Option<String> {
    match e {
        StaticEffect::Expanded(exp) => static_effect(&exp.value, ctx),
        StaticEffect::Modify { of, changes } => {
            let (subj, plural) = super::fragment::scope_subject_agreed(of, ctx);
            Some(format!(
                "{subj} {}.",
                modifications_predicate(changes, plural)
            ))
        }
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, ctx.subject)),
        StaticEffect::Replacement(r) => Some(super::replacement::replacement(r, ctx)),
        StaticEffect::CantHappen(_event) => Some("[can't happen]".to_string()), /* keyword cards render via their template */
        other => Some(format!("[unrendered: {other:?}].")),
    }
}

// ── Modification predicate builder ──────────────────────────────────────────

/// Build the predicate for a `Modify` effect:
/// "get +1/+1", "are black", "lose all abilities and have base power and
/// toughness 1/1".
///
/// Clauses are emitted in the order they appear in `changes`, with three
/// exceptions:
/// - All `Power`/`Toughness(Up|Down)` deltas are summed and emitted at the
///   position of the first such modification.
/// - `Power(Set)` + `Toughness(Set)` are combined into one "base P/T N/M"
///   clause at the position of the first such op in the list.
fn modifications_predicate(changes: &[Modification], plural: bool) -> String {
    let mut clauses: Vec<String> = Vec::new();

    // Flatten change-bundling macros (`AddPowerToughness` → `Several([AddPower,
    // AddToughness])`, looked through `Expanded`) so the grouping below renders
    // identically to the inline pair — the graduated-RON change is cosmetic.
    let changes = Modification::flatten(changes.to_vec());
    let changes = changes.as_slice();

    // Pre-scan to compute the combined values for the grouped cases.
    let delta = pt_delta_clause(changes, plural);
    let base = base_pt_clause(changes, plural);

    let mut delta_emitted = false;
    let mut base_emitted = false;

    for m in changes {
        match m {
            // P/T delta group (`Up`/`Down` on power/toughness): emit once at
            // first occurrence.
            Modification::Power(NumericOp::Up(_) | NumericOp::Down(_))
            | Modification::Toughness(NumericOp::Up(_) | NumericOp::Down(_)) => {
                if !delta_emitted {
                    if let Some(ref d) = delta {
                        clauses.push(d.clone());
                    }
                    delta_emitted = true;
                }
            }
            // Base P/T group (`Set` on power/toughness): emit once at first.
            Modification::Power(NumericOp::Set(_)) | Modification::Toughness(NumericOp::Set(_)) => {
                if !base_emitted {
                    if let Some(ref b) = base {
                        clauses.push(b.clone());
                    }
                    base_emitted = true;
                }
            }
            Modification::Colors(CollectionOp::Set(cs)) => {
                clauses.push(format!("{} {}", be(plural), colors_phrase(cs)));
            }
            Modification::GainAbility(a) => {
                clauses.push(format!("{} {}", have(plural), ability_noun(a)));
            }
            Modification::LoseAllAbilities => {
                clauses.push(format!("{} all abilities", lose(plural)));
            }
            other => clauses.push(format!("[unrendered: {other:?}]")),
        }
    }
    if clauses.is_empty() {
        return format!("[unrendered: {changes:?}]");
    }
    clauses.join(" and ")
}

/// "+N/+N" or "-N/-N" clause from `Power`/`Toughness(Up|Down)`, or `None` if
/// none present.
fn pt_delta_clause(changes: &[Modification], plural: bool) -> Option<String> {
    let mut p: i64 = 0;
    let mut t: i64 = 0;
    let mut found = false;
    for c in changes {
        match c {
            Modification::Power(NumericOp::Up(Count::Literal(n))) => {
                p += i64::from(*n);
                found = true;
            }
            Modification::Toughness(NumericOp::Up(Count::Literal(n))) => {
                t += i64::from(*n);
                found = true;
            }
            Modification::Power(NumericOp::Down(Count::Literal(n))) => {
                p -= i64::from(*n);
                found = true;
            }
            Modification::Toughness(NumericOp::Down(Count::Literal(n))) => {
                t -= i64::from(*n);
                found = true;
            }
            _ => {}
        }
    }
    if !found {
        return None;
    }
    Some(format!("{} {p:+}/{t:+}", get(plural)))
}

/// "have base power and toughness N/M" from `Power(Set)` + `Toughness(Set)`.
fn base_pt_clause(changes: &[Modification], plural: bool) -> Option<String> {
    let mut sp: Option<i64> = None;
    let mut st: Option<i64> = None;
    for c in changes {
        match c {
            Modification::Power(NumericOp::Set(Count::Literal(n))) => sp = Some(i64::from(*n)),
            Modification::Toughness(NumericOp::Set(Count::Literal(n))) => st = Some(i64::from(*n)),
            _ => {}
        }
    }
    match (sp, st) {
        (Some(p), Some(t)) => Some(format!("{} base power and toughness {p}/{t}", have(plural))),
        (Some(p), None) => Some(format!("{} base power {p}", have(plural))),
        (None, Some(t)) => Some(format!("{} base toughness {t}", have(plural))),
        (None, None) => None,
    }
}

// ── Verb helpers (plural/singular) ──────────────────────────────────────────

fn get(plural: bool) -> &'static str {
    if plural { "get" } else { "gets" }
}
fn be(plural: bool) -> &'static str {
    if plural { "are" } else { "is" }
}
fn have(plural: bool) -> &'static str {
    if plural { "have" } else { "has" }
}
fn lose(plural: bool) -> &'static str {
    if plural { "lose" } else { "loses" }
}

// ── Phrase helpers ───────────────────────────────────────────────────────────

/// A list of colors as a phrase: "black", "white and blue".
fn colors_phrase(cs: &[Color]) -> String {
    cs.iter()
        .map(|&c| color_name(c))
        .collect::<Vec<_>>()
        .join(" and ")
}

fn color_name(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Blue => "blue",
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
    }
}

/// An `Ability` as a noun phrase for "have <noun>".
fn ability_noun(a: &Ability) -> String {
    match a {
        Ability::Keyword(k) => super::keyword::keyword_name(k).to_lowercase(),
        other => format!("[unrendered: {other:?}]"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The function-zone qualifier ([CR#113.6,113.6b]) shared by the triggered
    /// and static renderers: the battlefield default (and `None`) is bare; a
    /// graveyard/hand `from` prefixes "As long as ~ is in your <zone>," and
    /// lowercases the wrapped clause's first letter.
    #[test]
    fn from_zone_qualifier_prefixes_nonbattlefield_only() {
        assert_eq!(
            from_zone_qualified(None, "X", "Creatures you control get +1/+1.".into()),
            "Creatures you control get +1/+1."
        );
        assert_eq!(
            from_zone_qualified(Some(Zone::Battlefield), "X", "Foo.".into()),
            "Foo."
        );
        assert_eq!(
            from_zone_qualified(
                Some(Zone::Graveyard),
                "Anger",
                "Creatures you control have haste.".into()
            ),
            "As long as Anger is in your graveyard, creatures you control have haste."
        );
        assert_eq!(
            from_zone_qualified(Some(Zone::Hand), "Force of Will", "Foo.".into()),
            "As long as Force of Will is in your hand, foo."
        );
    }
}
