//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_core::Ability;
use deckmaste_core::CharacteristicFilter;
use deckmaste_core::CollectionOp;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::PayAct;
use deckmaste_core::PlayerAttr;
use deckmaste_core::PlayerMod;
use deckmaste_core::Reference;
use deckmaste_core::StateFilterEvent;
use deckmaste_core::StaticAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;
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
        that: None,
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
        StaticEffect::ModifyPlayer(who, m) => Some(modify_player(who, m)),
        StaticEffect::TriggerMultiplier {
            cause,
            extra,
            affected,
        } => Some(trigger_multiplier(cause, extra, affected)),
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, ctx.subject)),
        StaticEffect::Replacement(r) => Some(super::replacement::replacement(r, ctx)),
        StaticEffect::CantHappen(_event) => Some("[can't happen]".to_string()), /* keyword cards render via their template */
        StaticEffect::PayPips(_class, act) => Some(pay_pips_keyword(act)),
        other => Some(format!("[unrendered: {other:?}].")),
    }
}

/// The printed keyword clause for a `PayPips` alternative-payment static
/// ([CR#702.51a,702.66a,702.126a]). Delve exiles, so `ExileToPay` → "Delve";
/// convoke and improvise both tap, told apart by what they tap — improvise an
/// artifact ([CR#702.126a]), convoke a creature ([CR#702.51a]). Keyword cards
/// normally render through their macro template (the `Expanded` invocation
/// carries one); this is the fallback for a directly written `PayPips` static.
fn pay_pips_keyword(act: &PayAct) -> String {
    match act {
        PayAct::ExileToPay(_) => "Delve".to_string(),
        PayAct::TapToPay(filter)
            if super::fragment::find_card_type(filter) == Some(Type::Artifact) =>
        {
            "Improvise".to_string()
        }
        PayAct::TapToPay(_) => "Convoke".to_string(),
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

// ── ModifyPlayer rendering ──────────────────────────────────────────────────

/// Render a `ModifyPlayer` static ([CR#611]) as a sentence: Reliquary Tower's
/// "You have no maximum hand size." ([CR#402.2]) and Exploration's "You may
/// play an additional land on each of your turns." ([CR#305.2]) are the
/// canonical shapes; other attributes fall through to a generic phrasing.
fn modify_player(who: &Reference, m: &PlayerMod) -> String {
    let subj = player_subject(who);
    match m {
        PlayerMod::NoMax(PlayerAttr::HandSizeLimit) => {
            format!("{subj} have no maximum hand size.")
        }
        PlayerMod::Raise(PlayerAttr::LandPlaysPerTurn, n) => {
            let lands = match literal_count(n) {
                Some(1) => "an additional land".to_string(),
                Some(k) => format!("{k} additional lands"),
                None => "additional lands".to_string(),
            };
            format!("{subj} may play {lands} on each of your turns.")
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// The subject word for a `ModifyPlayer`'s affected player. `You` renders as
/// "You" (the controller); other references fall through to a debug form.
fn player_subject(who: &Reference) -> String {
    match who {
        Reference::You => "You".to_string(),
        other => format!("[{other:?}]"),
    }
}

/// A `Count`'s literal value, or `None` if it is dynamic.
fn literal_count(c: &Count) -> Option<i64> {
    match c {
        Count::Literal(n) => Some(i64::from(*n)),
        _ => None,
    }
}

// ── TriggerMultiplier rendering ──────────────────────────────────────────────

/// Render a `TriggerMultiplier` static ([CR#603.2d]): Panharmonicon's "If an
/// artifact or creature entering the battlefield causes a triggered ability of
/// a permanent you control to trigger, that ability triggers an additional
/// time." The canonical enter-cause and you-control affected shapes render
/// faithfully; other shapes fall through to generic phrasing.
fn trigger_multiplier(cause: &Event, extra: &Count, affected: &Filter) -> String {
    let times = match literal_count(extra) {
        Some(1) => "an additional time".to_string(),
        Some(n) => format!("{n} additional times"),
        None => "additional times".to_string(),
    };
    let cause_phrase = cause_phrase(cause);
    let affected_phrase = affected_phrase(affected);
    format!(
        "If {cause_phrase} causes a triggered ability of {affected_phrase} to trigger, that ability triggers {times}."
    )
}

/// The cause clause: an enter-the-battlefield event renders as "{noun} entering
/// the battlefield"; anything else as a generic "an event".
fn cause_phrase(cause: &Event) -> String {
    if let Event::ZoneMove {
        what,
        to: Some(deckmaste_core::Zone::Battlefield),
        ..
    } = cause
    {
        return format!("{} entering the battlefield", types_noun(what));
    }
    "an event".to_string()
}

/// The affected-source clause: the "you control" default renders as "a
/// permanent you control"; anything else as a generic "an affected permanent".
fn affected_phrase(affected: &Filter) -> String {
    use deckmaste_core::RelationFilter;
    if matches!(
        affected,
        Filter::Relation(RelationFilter::ControlledBy(inner))
            if matches!(&**inner, Filter::Ref(Reference::You))
    ) {
        return "a permanent you control".to_string();
    }
    "an affected permanent".to_string()
}

/// A type filter as an indefinite noun: a single `Type` → "a creature"; a
/// `OneOf` of types → "an artifact or creature"; anything else → "an object".
fn types_noun(what: &Filter) -> String {
    let names: Vec<String> = match what {
        Filter::Characteristic(CharacteristicFilter::Type(t)) => {
            vec![super::card::type_str(*t).to_lowercase()]
        }
        Filter::OneOf(items) => items
            .iter()
            .filter_map(|f| match f {
                Filter::Characteristic(CharacteristicFilter::Type(t)) => {
                    Some(super::card::type_str(*t).to_lowercase())
                }
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };
    if names.is_empty() {
        return "an object".to_string();
    }
    let joined = names.join(" or ");
    format!("{} {joined}", article_for(&joined))
}

/// "a" or "an" for `word` by its leading sound (vowel-letter heuristic).
fn article_for(word: &str) -> &'static str {
    match word.chars().next() {
        Some(c) if "aeiou".contains(c.to_ascii_lowercase()) => "an",
        _ => "a",
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

    /// A bare `PayPips` static renders to its keyword name
    /// ([CR#702.51a,702.66a,702.126a]): delve exiles → "Delve"; convoke /
    /// improvise tap, told apart by what they tap (creature → "Convoke",
    /// artifact → "Improvise"). (Keyword cards render via their macro template;
    /// this is the fallback arm for a directly written static.)
    #[test]
    fn pay_pips_renders_its_keyword_name() {
        use deckmaste_core::CharacteristicFilter;
        use deckmaste_core::PayAct;
        use deckmaste_core::PipClass;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
        };
        let you = || {
            Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                Reference::You,
            ))))
        };
        let ty = |t| Filter::Characteristic(CharacteristicFilter::Type(t));

        let convoke = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Filter::AllOf(vec![ty(Type::Creature), you()])),
        );
        assert_eq!(static_effect(&convoke, &ctx).as_deref(), Some("Convoke"));

        let improvise = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Filter::AllOf(vec![ty(Type::Artifact), you()])),
        );
        assert_eq!(
            static_effect(&improvise, &ctx).as_deref(),
            Some("Improvise")
        );

        let delve = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::ExileToPay(Filter::State(StateFilter::InZone(Zone::Graveyard))),
        );
        assert_eq!(static_effect(&delve, &ctx).as_deref(), Some("Delve"));
    }
}
