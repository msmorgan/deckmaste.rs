//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_semantics::Ability;
use deckmaste_semantics::AsThough;
use deckmaste_semantics::CharacteristicPredicate;
use deckmaste_semantics::Cmp;
use deckmaste_semantics::CollectionOp;
use deckmaste_semantics::Color;
use deckmaste_semantics::Condition;
use deckmaste_semantics::Count;
use deckmaste_semantics::DeedAgent;
use deckmaste_semantics::Deontic;
use deckmaste_semantics::DeonticAction;
use deckmaste_semantics::EventFilter;
use deckmaste_semantics::IgnoreRule;
use deckmaste_semantics::Modification;
use deckmaste_semantics::NumericOp;
use deckmaste_semantics::ObjectKind;
use deckmaste_semantics::PayAct;
use deckmaste_semantics::PlayerAttr;
use deckmaste_semantics::PlayerMod;
use deckmaste_semantics::Predicate;
use deckmaste_semantics::Reference;
use deckmaste_semantics::RelationPredicate;
use deckmaste_semantics::Stat;
use deckmaste_semantics::StatValue;
use deckmaste_semantics::StateChange;
use deckmaste_semantics::StatePredicate;
use deckmaste_semantics::StaticEffect;
use deckmaste_semantics::TriggeredAbility;
use deckmaste_semantics::Type;
use deckmaste_semantics::Zone;

use super::CardView;
use super::Ctx;
use super::effect;

/// "When/Whenever <event>, [if <cond>,] <effect>.", with a leading
/// "While ~ is in your <zone>," qualifier for a graveyard/hand-functioning
/// trigger ([CR#113.6,113.6b]).
pub(super) fn triggered(t: &TriggeredAbility, view: &CardView) -> String {
    // Targeting lives on an `OneShotEffect::Targeted` wrapper, which the effect
    // walk rebinds `ctx.targets` from ([CR#115.1]).
    let ctx = Ctx {
        subject: view.name,
        targets: &[],
        that: None,
        named: None,
    };
    let (lead, clause) = event_clause(&t.event, &ctx);
    // Inside the body the oracle refers to the (already-named) source as
    // "it" — "When ~ dies, it deals 1 damage …", "…, sacrifice it." — EXCEPT
    // a non-battlefield function-zone ([CR#113.6]), whose self-references
    // print "this card" instead (Flashback's "you may cast this card from
    // your graveyard" convention; Death Spark's "return this card to your
    // hand"), since the source has no permanent identity to be "it".
    // Some triggering events introduce no object antecedent at all (only an
    // actor and/or amount, e.g. `CoinFlipped`/`DiceRolled`/`RollPlanarDie`/
    // `StepBegins` — mirroring Idris `EventCaps.hasObject = False`): there
    // is no "it" for the body to point back to, so it must name itself
    // ("this enchantment", Chance Encounter's "put a luck counter on this
    // enchantment") the way [CR#603.4]-family bodies conventionally do when
    // the trigger itself supplies no distinguished object. Every other
    // event (`ZoneChange`/`StateBecame`/…) already established `This` reads
    // as the plain anaphor "it" for real cards, so only the object-less
    // events change here — no regression to the existing convention.
    // ASSUMES a single `body_subject` for the whole body: every `This` read
    // renders with the SAME phrase. Real oracle names the object once (in the
    // intervening-if `condition`, rendered via `self_type_phrase` below) then
    // says "it" for later self-references in the same sentence, which this
    // model would double-name. That only bites a StepBegins ability that ALSO
    // carries a `condition` and a self-referencing body; today the only such
    // shapes (Echo, Cumulative Upkeep) are authored as `Keyword(...)` and
    // render via the keyword line, never reaching `triggered()`, so it is
    // unreachable. A future hand-authored non-macro card of that shape would
    // need first-mention-names / later-mentions-"it" handling here.
    let body_subject = if matches!(t.from, Some(z) if z != Zone::Battlefield) {
        "this card".to_string()
    } else if matches!(t.event, EventFilter::Cast { .. }) {
        // A cast trigger's clause introduces the SPELL as its antecedent
        // (`what:`), never the source — there is no "it" for a
        // self-referencing body to point back to. Real oracle text instead
        // repeats the source's own printed NAME (Guttersnipe: "Whenever you
        // cast an instant or sorcery spell, Guttersnipe deals 2 damage to
        // each opponent."), which the render pipeline's own-name/short-name
        // substitution later folds to "~" (the fidelity checker's own-name
        // fold) — distinct from the object-less events below, which have no
        // name-bearing antecedent noun at all and so name themselves by TYPE
        // instead ("this enchantment").
        view.name.to_string()
    } else if matches!(
        t.event,
        EventFilter::CoinFlipped { .. }
            | EventFilter::DiceRolled { .. }
            | EventFilter::RollPlanarDie { .. }
            | EventFilter::TapForMana { .. }
            | EventFilter::StepBegins { .. }
    ) {
        self_type_phrase(view)
    } else {
        "it".to_string()
    };
    let body_ctx = Ctx {
        subject: &body_subject,
        targets: &[],
        that: None,
        named: None,
    };
    // The trailing "This ability triggers only once[ each turn]." rider the
    // `limits` field prints ([CR#603.2h]) — appended to whichever complete
    // sentence this function returns below. The parse-direction mirror is the
    // migration parser's `triggered_ability::peel_trigger_limit`.
    let limit_rider = trigger_limit_rider(t);
    let raw_body = effect::effect(&t.effect, &body_ctx);
    // `lower_first` de-capitalizes the effect body's own sentence-start
    // capitalization for this mid-clause position ("Draw a card." -> "draw a
    // card."). EXCEPT when the body's own first word IS the just-substituted
    // `body_subject` (the Cast-event own-name case above): the fidelity
    // checker's own-name/short-name fold matches the EXACT printed name
    // (title case), so lowercasing its leading letter here ("aven Wind Mage")
    // would break that fold. `reference_subject`'s `Reference::This =>
    // ctx.subject` inserts the subject verbatim with no capitalization step
    // of its own, so a name-subject body needs no un-capitalizing here.
    let body = if raw_body.starts_with(body_subject.as_str()) {
        raw_body
    } else {
        lower_first(&raw_body)
    };

    // Death Spark's shape: the intervening-if ALREADY states the ability's
    // own function-zone membership inline ("this card is in your graveyard
    // with a creature card directly above it") — an old-templated
    // self-reference that replaces BOTH the generic condition render below
    // AND `from_zone_qualified`'s leading "As long as ~ is in your Y,"
    // qualifier (which would otherwise double-state the zone).
    if let Some(cond_clause) = t
        .condition
        .as_ref()
        .and_then(|c| adjacent_in_zone_if_clause(c, t.from))
    {
        return format!("{lead} {clause}, if {cond_clause}, {body}{limit_rider}");
    }

    // Intervening-if ([CR#603.4]): "…, if <cond>, <effect>." The condition
    // clause conventionally names itself by TYPE ("if this enchantment has
    // ten or more luck counters on it", Chance Encounter) rather than the
    // bare "it" `body_ctx` uses for the effect — the condition is checked
    // independently of whatever antecedent the trigger established, so a
    // stable self-reference is the printed convention. Unlike `body_subject`
    // above (scoped to the object-less-event family), `self_type_phrase(view)`
    // is threaded here as the condition subject for EVERY triggered ability,
    // regardless of event — inert/zero-regression for existing cards whose
    // rendered conditions don't read the subject.
    let cond = match &t.condition {
        Some(c) => {
            let cond_ctx = Ctx {
                subject: &self_type_phrase(view),
                targets: &[],
                that: None,
                named: None,
            };
            format!("if {}, ", super::condition::condition(c, &cond_ctx))
        }
        None => String::new(),
    };
    let trig = format!("{lead} {clause}, {cond}{body}");
    // The limit rider trails the WHOLE ability sentence, after any
    // `from_zone_qualified` "As long as ~ is in your Y, …" wrapper.
    format!(
        "{}{limit_rider}",
        from_zone_qualified(t.from, view.name, trig)
    )
}

/// The trailing "This ability triggers only once[ each turn]." rider a
/// [`TriggeredAbility`]'s `limits` print ([CR#603.2h]) — the render-direction
/// mirror of the migration parser's `triggered_ability::peel_trigger_limit`,
/// and the trigger-frame analogue of [`activation_rider`] (which prints the
/// activated frame's "Activate only …" sentence). Empty when no limit is set.
///
/// Note the per-game trigger form is the bare "only once." — NOT the activated
/// frame's "once each game." — matching real oracle text ("This ability
/// triggers only once."). A `LoyaltyOncePerTurn` limit prints nothing here
/// (that gate is an implicit loyalty rule, never a printed trigger sentence —
/// and a loyalty ability never reaches `triggered()` anyway), mirroring
/// [`activation_rider`]'s suppression.
fn trigger_limit_rider(t: &TriggeredAbility) -> String {
    use deckmaste_semantics::UseLimit;
    let mut out = String::new();
    for limit in t.limits.iter() {
        match limit {
            UseLimit::OncePerTurn => out.push_str(" This ability triggers only once each turn."),
            UseLimit::OncePerGame => out.push_str(" This ability triggers only once."),
            UseLimit::LoyaltyOncePerTurn => {}
        }
    }
    out
}

/// "this enchantment" / "this creature" / … — a card's own type-noun
/// self-reference, first printed type. Falls back to the plain anaphor
/// "it" for a (structurally impossible, but never-crash) typeless card.
fn self_type_phrase(view: &CardView) -> String {
    view.types.first().map_or_else(
        || "it".to_string(),
        |t| format!("this {}", super::card::type_str(t).to_lowercase()),
    )
}

/// Death Spark's "if this card is in your graveyard with a creature card
/// directly above it" ([CR#404.2]): recognizes `Condition::Exists(And([type
/// filter, Adjacent(dir, Ref(This))]))` against a non-battlefield `from`, and
/// folds the ability's own function-zone ([CR#113.6]) into the clause
/// itself. `None` for every other condition/from shape — the generic
/// `condition()` render (and `from_zone_qualified`'s qualifier) still apply
/// there.
fn adjacent_in_zone_if_clause(
    cond: &Condition,
    from: Option<deckmaste_semantics::Zone>,
) -> Option<String> {
    let zone = from.filter(|&z| z != Zone::Battlefield)?;
    let Condition::Exists(Predicate::And(parts)) = cond else {
        return None;
    };
    let [a, b] = parts.as_ref() else { return None };
    let ((Predicate::Adjacent(dir, Reference::This), other)
    | (other, Predicate::Adjacent(dir, Reference::This))) = (a, b)
    else {
        return None;
    };
    let dir_word = match dir {
        deckmaste_semantics::Adjacency::Above => "above",
        deckmaste_semantics::Adjacency::Below => "below",
    };
    let noun = effect::a_an(&format!("{} card", super::fragment::filter_noun(other)));
    Some(format!(
        "this card is in your {} with {noun} directly {dir_word} it",
        super::fragment::zone_word(zone),
    ))
}

/// "{cost}: {effect}" — an activated ability's printed line ([CR#602.1]:
/// cost, colon, effect). The cost renders through the shared symbol
/// renderer; a cost with no clean symbol rendering falls back to the
/// structural form.
pub(super) fn activated(a: &deckmaste_semantics::ActivatedAbility, view: &CardView) -> String {
    // Battlefield-context self-reference conventionally names the card by
    // TYPE ("this artifact"), never by repeating its own printed name — the
    // one activated-ability shape that reads its own `This` in the body is
    // the "exchange control" pair (Avarice Totem's "Exchange control of
    // this artifact and target nonland permanent"), so this only overrides
    // the default `view.name` subject for that shape; every other activated
    // ability keeps today's behavior unchanged.
    let subject = if effect_wants_self_type_phrase(&a.effect) {
        self_type_phrase(view)
    } else {
        view.name.to_string()
    };
    let ctx = Ctx {
        subject: &subject,
        targets: &[],
        that: None,
        named: None,
    };
    let cost = effect::activated_cost(&a.cost.0, &ctx);
    let body = effect::effect(&a.effect, &ctx);
    let rider = activation_rider(a, &ctx);
    from_zone_qualified(a.from, view.name, format!("{cost}: {body}{rider}"))
}

/// The trailing "Activate only …" rider sentence an `ActivatedAbility`'s
/// `window`/`condition`/`limits` fields print — the
/// render-direction mirror of the migration parser's
/// `activated_ability::peel_activation_riders`. Empty when none of the three
/// fields are set. Clause order: window, then condition, then each limit —
/// a fixed printed order (real cards vary which clause leads when several
/// combine; this renderer picks one canonical order rather than tracking
/// which was printed first).
fn activation_rider(a: &deckmaste_semantics::ActivatedAbility, ctx: &Ctx) -> String {
    use deckmaste_semantics::UseLimit;

    // A loyalty ability's sorcery-speed-only + shared-once-per-turn gate is
    // an implicit RULE, never printed as its own rider sentence — real cards
    // print "+2: Each player draws a card." with no trailing "Activate
    // only …" at all. `LoyaltyOncePerTurn` uniquely marks that shape (it
    // always pairs with `window: SorcerySpeed` on the same ability, per this
    // field's own doc comment on `UseLimit`), so its presence suppresses the
    // whole rider rather than printing a sentence no loyalty ability prints.
    if a.limits.contains(&UseLimit::LoyaltyOncePerTurn) {
        return String::new();
    }

    let mut clauses: Vec<String> = Vec::new();
    if let Some(window) = a.window {
        clauses.push(activation_window_clause(window));
    }
    if let Some(cond) = &a.condition {
        clauses.push(format!("if {}", super::condition::condition(cond, ctx)));
    }
    clauses.extend(a.limits.iter().map(|l| use_limit_clause(*l)));
    if clauses.is_empty() {
        String::new()
    } else {
        format!(" Activate only {}.", clauses.join(" and only "))
    }
}

/// A [`Timing`](deckmaste_semantics::Timing) window as the clause following
/// "Activate only ": "as a sorcery", "during your turn". Reuses
/// [`whose_word`]/[`step_noun`] — the same building blocks the
/// triggered-ability "At the beginning of your upkeep" event clause uses.
fn activation_window_clause(t: deckmaste_semantics::Timing) -> String {
    use deckmaste_semantics::Timing;
    match t {
        Timing::InstantSpeed => "as an instant".to_string(),
        Timing::SorcerySpeed => "as a sorcery".to_string(),
        Timing::DuringTurn(whose) => format!("during {} turn", whose_word(whose)),
        Timing::DuringStep(step, whose) => match step_noun(step) {
            Some(noun) => format!("during {} {noun}", whose_word(whose)),
            None => format!("[unrendered window: {t:?}]"),
        },
    }
}

/// A [`UseLimit`](deckmaste_semantics::UseLimit) as the clause following
/// "Activate only ": "once each turn", "once each game". The shared loyalty
/// use-limit never reaches this renderer today —
/// a loyalty ability prints via its own `LoyaltyPlus`/`Minus`/`Zero` macro
/// template, not the generic `activated()` line — but reads the same
/// "once each turn" phrase if it ever does.
fn use_limit_clause(l: deckmaste_semantics::UseLimit) -> String {
    use deckmaste_semantics::UseLimit;
    match l {
        UseLimit::OncePerTurn | UseLimit::LoyaltyOncePerTurn => "once each turn".to_string(),
        UseLimit::OncePerGame => "once each game".to_string(),
    }
}

/// Whether an activated ability's effect (through its `Targeted` wrapper, if
/// any) is the "exchange control" `Simultaneously` shape — see
/// [`activated`]'s doc comment for why this drives the `Ctx.subject` choice.
fn effect_wants_self_type_phrase(e: &deckmaste_semantics::OneShotEffect) -> bool {
    use deckmaste_semantics::OneShotEffect;
    let inner: &OneShotEffect = match e {
        OneShotEffect::Targeted(t) => &t.effect,
        other => other,
    };
    matches!(inner, OneShotEffect::Simultaneously(parts) if effect::exchange_control_refs(parts).is_some())
}

/// Prefix a "While ~ is in your <zone>," function-zone qualifier
/// ([CR#113.6,113.6b]) for a non-battlefield `from`; the battlefield default
/// (and `None`) is left bare.
pub(super) fn from_zone_qualified(
    from: Option<deckmaste_semantics::Zone>,
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

/// The "as long as [condition]," qualifier a `Conditionally` static
/// ([CR#611.3a]) prefixes its inner effect with. `Is(reference,
/// InZone(zone))` is the graveyard/hand-functioning static shape
/// ([CR#113.6,604.3]) the deleted `StaticAbility.from` field used to carry —
/// it reuses [`from_zone_qualified`]'s exact phrasing. Any other condition
/// falls back to a generic "As long as [cond]," prefix around the shared
/// intervening-if condition renderer.
fn conditionally_qualified(cond: &Condition, ctx: &Ctx, text: String) -> String {
    let cond = match cond {
        Condition::Expanded(e) => &e.value,
        other => other,
    };
    if let Condition::Matches(reference, filter) = cond
        && let Predicate::State(StatePredicate::InZone(zone)) =
            super::fragment::strip_expanded(filter)
    {
        return from_zone_qualified(
            Some(*zone),
            &super::fragment::modify_subject(reference, ctx),
            text,
        );
    }
    format!(
        "As long as {}, {}",
        super::condition::condition(cond, ctx),
        conditionally_lead(&text, cond, ctx)
    )
}

/// The `Reference` a `Matches`/`Not(Matches(..))` condition names, if any —
/// the subject the inner effect's OWN rendering re-mentions when its body
/// targets that same object (`SubjectIs`/`SubjectIsnt`'s composition:
/// "as long as ~ is attacking, it gets +2/+0.", not "..., Adanto Vanguard
/// gets +2/+0.").
fn matches_subject(cond: &Condition) -> Option<&Reference> {
    match cond {
        Condition::Matches(reference, _) => Some(reference),
        Condition::Not(inner) => match &**inner {
            Condition::Matches(reference, _) => Some(reference),
            _ => None,
        },
        _ => None,
    }
}

/// The conditional-composition twin of [`lower_first`] for a `Conditionally`
/// effect's inner clause ([CR#201.5]):
///
/// - `text` leads with the SAME subject phrase the condition just named
///   (`Matches`/`Not(Matches(..))`'s own reference, e.g. the card's own name
///   for `This`, "Enchanted creature" for `AttachHostOf(This)`) — this is the
///   sentence's SECOND self-mention, so it reads as the pronoun "it" rather
///   than repeating the name/noun.
/// - `text` leads with the card's own name (`ctx.subject`, `Reference::This`'s
///   plain rendering) but the condition DIDN'T name that subject ("As long as
///   you control an artifact, Aerial Engineer gets …") — this IS the sentence's
///   first (only) self-mention, so it must survive BYTE-FOR-BYTE:
///   `fidelity::normalize`'s self-reference collapse is an exact-case substring
///   match, and lowercasing the lead letter (the plain [`lower_first`] this
///   replaces) would break it.
/// - Anything else falls back to the plain [`lower_first`], unchanged.
fn conditionally_lead(text: &str, cond: &Condition, ctx: &Ctx) -> String {
    if let Some(subject) = matches_subject(cond).map(|r| super::fragment::modify_subject(r, ctx))
        && text.starts_with(&subject)
    {
        return format!("it{}", &text[subject.len()..]);
    }
    if text.starts_with(ctx.subject) {
        return text.to_string();
    }
    lower_first(text)
}

/// Returns (lead word, the event clause).
/// "When", "Baleful Strix enters" | "Whenever", "Goblin Medics becomes tapped".
pub(super) fn event_clause(e: &EventFilter, ctx: &Ctx) -> (&'static str, String) {
    match e {
        EventFilter::Expanded(exp) => event_clause(&exp.value, ctx),
        EventFilter::OneOf(events) => disjoined_event_clause(events, ctx)
            .unwrap_or_else(|| ("When", format!("[unrendered: {e:?}]"))),
        EventFilter::ZoneChange {
            what,
            to: Some(Zone::Battlefield),
            from: None,
            ..
        } => (lead_for(what), format!("{} enters", subject_of(what, ctx))),
        EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            ..
        } => (lead_for(what), format!("{} dies", subject_of(what, ctx))),
        // Leaves-the-battlefield ([CR#603.6c]): the GENERAL zone-change-from-
        // battlefield event, any destination (`to` omitted) — the
        // `LeavesBattlefield`/`ThisLeavesBattlefield` macros' expansion.
        // Structurally disjoint from the "dies" arm above (that arm requires
        // `to: Some(Graveyard)`, this one requires `to: None`), so a "dies"
        // shape is never rendered as "leaves the battlefield" or vice versa.
        EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: None,
            ..
        } => (
            lead_for(what),
            format!("{} leaves the battlefield", subject_of(what, ctx)),
        ),
        EventFilter::StateBecame { of, becomes, .. } => (
            "Whenever",
            format!("{} becomes {}", subject_of(of, ctx), state_word(becomes)),
        ),
        // The one block fact, two views ([CR#509.3a,509.3c]): a narrowed
        // `by` reads from the blocker side, a narrowed `of` from the
        // blocked attacker's.
        EventFilter::BlockDeclared {
            by: Predicate::Any,
            of,
        } => (
            "Whenever",
            format!("{} becomes blocked", subject_of(of, ctx)),
        ),
        // Directional two-slot forms ([CR#509.1g..509.1h]): BOTH sides
        // narrowed at once, the `Blocking` event macro's shape. Told apart by
        // which slot is `This` — self is the blocker narrowing the blocked
        // side ([CR#509.3b], "~ blocks a creature") vs. self is the blocked
        // attacker narrowing the blocker ([CR#509.3d], "~ becomes blocked by
        // a creature"). Must precede the `by, ..` catch-all below, which
        // would otherwise swallow `of` and drop the object silently.
        EventFilter::BlockDeclared { by, of }
            if super::fragment::strip_expanded(by).is_this()
                && !matches!(super::fragment::strip_expanded(of), Predicate::Any) =>
        {
            (
                "Whenever",
                format!("{} blocks {}", subject_of(by, ctx), subject_of(of, ctx)),
            )
        }
        EventFilter::BlockDeclared { by, of }
            if super::fragment::strip_expanded(of).is_this()
                && !matches!(super::fragment::strip_expanded(by), Predicate::Any) =>
        {
            (
                "Whenever",
                format!(
                    "{} becomes blocked by {}",
                    subject_of(of, ctx),
                    subject_of(by, ctx)
                ),
            )
        }
        EventFilter::BlockDeclared { by, .. } => {
            ("Whenever", format!("{} blocks", subject_of(by, ctx)))
        }
        EventFilter::AttackDeclared { by, .. } => {
            ("Whenever", format!("{} attacks", subject_of(by, ctx)))
        }
        // "When ~ becomes the target of a spell or ability" — the
        // unconstrained agent reads as the printed "a spell or ability".
        EventFilter::BecomesTarget {
            what,
            by: Predicate::Any,
            source: None,
        } => (
            lead_for(what),
            format!(
                "{} becomes the target of a spell or ability",
                subject_of(what, ctx)
            ),
        ),
        // A turn-step onset ([CR#500.1,603.2b]) — "At the beginning of your
        // upkeep" (Benthic Djinn, Death Spark). No subject: the step names
        // itself, unlike an object-relative "enters"/"dies"/"becomes …".
        EventFilter::StepBegins { at, whose } => match step_noun(*at) {
            Some(noun) => (
                "At the beginning of",
                format!("{} {noun}", whose_word(*whose)),
            ),
            None => ("When", format!("[unrendered: {e:?}]")),
        },
        // [CR#106.12]: "Whenever a player taps a land for mana," (Dictate of
        // Karametra) / "Whenever you tap a land for mana," (Vorinclex) — the
        // `what` (which land) coordinate is left unrendered-generic (no real
        // card in this corpus narrows it; the event kind is already
        // land-scoped, so `what` almost never needs to narrow further).
        EventFilter::TapForMana {
            what: Predicate::Any,
            by,
        } => (
            "Whenever",
            format!("{} taps a land for mana", tap_for_mana_subject(by)),
        ),
        // [CR#705.1,705.2]: "Whenever you win a coin flip," (Chance
        // Encounter, Tavern Scoundrel). Only the `by: You, won: true` shape
        // is recognized — the only one a real card in this corpus needs;
        // any other narrowing (an opponent's flip, a loss) falls through to
        // the generic marker.
        EventFilter::CoinFlipped {
            by: Predicate::Ref(Reference::You),
            won: Some(true),
        } => ("Whenever", "you win a coin flip".to_string()),
        // Combat damage dealt ([CR#510.2]) to a recipient — the
        // `DealsCombatDamage` macro's expansion. Only the `combat: true`
        // narrowing is recognized here (noncombat damage has no oracle
        // "deals combat damage" phrasing); `amount` isn't printed by this
        // family (no card in this corpus narrows it).
        EventFilter::Damage {
            source,
            to,
            combat: Some(true),
            amount: None,
        } => (
            "Whenever",
            format!(
                "{} deals combat damage to {}",
                subject_of(source, ctx),
                recipient_of(to, ctx)
            ),
        ),
        // Generic (non-combat-narrowed) damage dealt to a recipient
        // ([CR#120.1]) — the passive-voice `DealtDamage` macro's expansion:
        // the recipient itself is the trigger's subject ("<recipient> is
        // dealt damage"), unlike the active-voice `DealsCombatDamage` arm
        // above ("<source> deals combat damage to <recipient>"). `source`
        // stays unnarrowed (any source) and `combat` unset (fires on
        // combat OR noncombat damage) — matching the plain "is dealt
        // damage" oracle phrasing (no card in this corpus narrows source,
        // combat, or amount for this event).
        EventFilter::Damage {
            source: Predicate::Any,
            to,
            combat: None,
            amount: None,
        } => (
            lead_for(to),
            format!("{} is dealt damage", recipient_of(to, ctx)),
        ),
        // A player gained life ([CR#119.3]) — the `GainsLife` macro's
        // expansion. Both verb agreements a real card narrows to:
        // 2nd-person "you gain life" and 3rd-person-singular "an opponent
        // gains life"; any other subject falls through to the unrendered
        // marker.
        EventFilter::LifeGained {
            who: Predicate::Ref(Reference::You),
            amount: None,
        } => ("Whenever", "you gain life".to_string()),
        EventFilter::LifeGained {
            who: Predicate::Relation(RelationPredicate::OpponentOf(inner)),
            amount: None,
        } if matches!(
            super::fragment::strip_expanded(inner),
            Predicate::Ref(Reference::You)
        ) =>
        {
            ("Whenever", "an opponent gains life".to_string())
        }
        // A player drew ([CR#121.1]) — the `Draws` macro's expansion (mind
        // the underlying event: `Drawn`, not `Draws`). Same two
        // verb-agreement shapes as `LifeGained` above.
        EventFilter::Drawn {
            who: Predicate::Ref(Reference::You),
            amount: None,
        } => ("Whenever", "you draw a card".to_string()),
        EventFilter::Drawn {
            who: Predicate::Relation(RelationPredicate::OpponentOf(inner)),
            amount: None,
        } if matches!(
            super::fragment::strip_expanded(inner),
            Predicate::Ref(Reference::You)
        ) =>
        {
            ("Whenever", "an opponent draws a card".to_string())
        }
        // Cast onset ([CR#601.2i]) — "<subject> cast(s) X". Three `who:`
        // narrowings are recognized ([`cast_who_phrase`]): the controller's
        // own cast ("you cast"), any player's ("a player casts"), and
        // specifically an opponent's ("an opponent casts"). Self
        // (`Ref(This)`) leads "When" via `lead_for`, matching the enters/dies
        // self convention.
        EventFilter::Cast { who, what } => cast_event_clause(who, what),
        other => ("When", format!("[unrendered: {other:?}]")),
    }
}

fn cast_event_clause(who: &Predicate, what: &Predicate) -> (&'static str, String) {
    (
        lead_for(what),
        match cast_who_phrase(who) {
            Some(subject) => format!("{subject} {}", cast_subject(what)),
            None => format!("[unrendered: {who:?}]"),
        },
    )
}

/// The subject phrase for a [`EventFilter::TapForMana`]'s `by` coordinate:
/// "a player" for the unrestricted (any-player) form, "you" for the
/// self-only form. Any other narrowing falls back to the structural marker
/// — no real card in this corpus needs one yet.
fn tap_for_mana_subject(by: &Predicate) -> String {
    match super::fragment::strip_expanded(by) {
        Predicate::Any => "a player".to_string(),
        Predicate::Ref(Reference::You) => "you".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The possessive turn-owner phrase for a [`WhoseTurn`] (mirrors
/// `condition::turn_owner`'s "your"/"an opponent's" phrasing — a templating
/// convention, not itself a cited rule).
fn whose_word(w: deckmaste_semantics::WhoseTurn) -> &'static str {
    use deckmaste_semantics::WhoseTurn;
    match w {
        WhoseTurn::Your => "your",
        WhoseTurn::EachPlayers => "each player's",
        WhoseTurn::AnOpponents => "an opponent's",
    }
}

/// The step/phase noun a [`PhaseStep`](deckmaste_semantics::PhaseStep) prints
/// after its possessive owner ("your **upkeep**"). `None` for a phase/step
/// this corpus hasn't needed phrasing for yet (a combat sub-step) — the
/// caller falls back to the generic unrendered marker rather than guessing.
fn step_noun(at: deckmaste_semantics::PhaseStep) -> Option<&'static str> {
    use deckmaste_semantics::BeginningStep;
    use deckmaste_semantics::EndingStep;
    use deckmaste_semantics::PhaseStep;
    Some(match at {
        PhaseStep::Beginning(BeginningStep::Upkeep) => "upkeep",
        PhaseStep::Beginning(BeginningStep::Untap) => "untap step",
        PhaseStep::Beginning(BeginningStep::Draw) => "draw step",
        PhaseStep::PrecombatMain => "precombat main phase",
        PhaseStep::PostcombatMain => "postcombat main phase",
        PhaseStep::Ending(EndingStep::End) => "end step",
        PhaseStep::Ending(EndingStep::Cleanup) => "cleanup step",
        PhaseStep::Combat(_) => return None,
    })
}

/// One-shot enters/dies of THIS → "When"; a filtered (non-self) subject →
/// "Whenever".
fn lead_for(what: &Predicate) -> &'static str {
    if matches!(
        super::fragment::strip_expanded(what),
        Predicate::Ref(Reference::This)
    ) {
        "When"
    } else {
        "Whenever"
    }
}

/// A subject filter as a noun ("Baleful Strix" for the self filter, "a
/// creature" for a Creature macro filter, "another creature you control" for
/// a qualified filter — [`super::fragment::subject_phrase`]'s singular
/// register).
fn subject_of(f: &Predicate, ctx: &Ctx) -> String {
    let f = super::fragment::strip_expanded(f);
    if f.is_this() {
        return ctx.subject.to_string();
    }
    if let Some(phrase) =
        super::fragment::subject_phrase(f, super::fragment::SubjectNumber::SingularArticle)
    {
        return phrase;
    }
    // Bare subtypes ("another Ally you control") are intentionally nouns in
    // trigger-subject position. `subject_phrase` handles type-based subjects;
    // its filter-noun sibling also recognizes subtype nouns and all of the
    // same restrictors. Inflect its bare "other" into the singular-subject
    // "another", or add the ordinary indefinite article.
    let noun = super::fragment::filter_noun(f);
    if !noun.starts_with("[unrendered") {
        return noun.strip_prefix("other ").map_or_else(
            || super::effect::a_an(&noun),
            |rest| format!("another {rest}"),
        );
    }
    format!("[unrendered: {f:?}]")
}

/// Render a `OneOf` trigger as the printed shared-subject or shared-verb
/// disjunction. Each member remains a full event filter internally; this
/// function only factors their common English surface:
///
/// - `ThisEnters | ThisAttacks` → "~ enters or attacks";
/// - `ThisEnters | Enters(another Ally)` → "~ or another Ally enters".
fn disjoined_event_clause(events: &[EventFilter], ctx: &Ctx) -> Option<(&'static str, String)> {
    let parts: Vec<(&'static str, String, &'static str)> = events
        .iter()
        .map(|event| event_clause_part(event, ctx))
        .collect::<Option<_>>()?;
    if parts.len() < 2 {
        return None;
    }
    let lead = if parts.iter().any(|(lead, _, _)| *lead == "Whenever") {
        "Whenever"
    } else {
        "When"
    };

    if parts.iter().all(|(_, _, verb)| *verb == parts[0].2) {
        let subjects = parts
            .iter()
            .map(|(_, subject, _)| subject.as_str())
            .collect::<Vec<_>>()
            .join(" or ");
        return Some((lead, format!("{subjects} {}", parts[0].2)));
    }

    if parts.iter().all(|(_, subject, _)| subject == &parts[0].1) {
        let plural_subject = compound_name_takes_plural_verb(&parts[0].1, ctx);
        let verbs = parts
            .iter()
            .map(
                |(_, _, verb)| {
                    if plural_subject { plural_event_verb(verb) } else { verb }
                },
            )
            .collect::<Vec<_>>()
            .join(" or ");
        return Some((lead, format!("{} {verbs}", parts[0].1)));
    }
    None
}

/// The renderable object-event subset used by OR-composed trigger clauses.
/// Directional two-slot block events deliberately stay outside this factoring
/// helper: their two printed participants cannot be reduced to one subject +
/// verb without losing a narrowing.
fn event_clause_part(
    event: &EventFilter,
    ctx: &Ctx,
) -> Option<(&'static str, String, &'static str)> {
    match event {
        EventFilter::Expanded(exp) => event_clause_part(&exp.value, ctx),
        EventFilter::ZoneChange {
            what,
            to: Some(Zone::Battlefield),
            from: None,
            ..
        } => Some((lead_for(what), subject_of(what, ctx), "enters")),
        EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            ..
        } => Some((lead_for(what), subject_of(what, ctx), "dies")),
        EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: None,
            ..
        } => Some((
            lead_for(what),
            subject_of(what, ctx),
            "leaves the battlefield",
        )),
        EventFilter::AttackDeclared { by, .. } => {
            Some(("Whenever", subject_of(by, ctx), "attacks"))
        }
        EventFilter::BlockDeclared {
            by,
            of: Predicate::Any,
        } => Some(("Whenever", subject_of(by, ctx), "blocks")),
        EventFilter::BlockDeclared {
            by: Predicate::Any,
            of,
        } => Some(("Whenever", subject_of(of, ctx), "becomes blocked")),
        _ => None,
    }
}

fn compound_name_takes_plural_verb(subject: &str, ctx: &Ctx) -> bool {
    subject == ctx.subject && (subject.contains(" & ") || subject.contains(" and "))
}

fn plural_event_verb(verb: &str) -> &str {
    match verb {
        "enters" => "enter",
        "dies" => "die",
        "leaves the battlefield" => "leave the battlefield",
        "attacks" => "attack",
        "blocks" => "block",
        "becomes blocked" => "become blocked",
        other => other,
    }
}

/// The recipient phrase for a [`EventFilter::Damage`]'s `to` coordinate: the
/// restricted "any target" family a combat-damage recipient draws from
/// ([CR#115.4]: player, planeswalker, battle, creature) plus the
/// player-identity forms (`you`, `an opponent`) [`subject_of`] doesn't reach —
/// the render-direction mirror of the migration parser's
/// `filter::recipient_phrase`. An `Or([...])` disjunction (Lava Spike's "a
/// player or planeswalker") joins its members with "or"; anything else falls
/// back to [`subject_of`]'s bare object-type reading ("a creature").
fn recipient_of(f: &Predicate, ctx: &Ctx) -> String {
    match super::fragment::strip_expanded(f) {
        Predicate::Kind(ObjectKind::Player) => "a player".to_string(),
        Predicate::Ref(Reference::You) => "you".to_string(),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(
                super::fragment::strip_expanded(inner),
                Predicate::Ref(Reference::You)
            ) =>
        {
            "an opponent".to_string()
        }
        // "a player or planeswalker" / "a player or battle" ([CR#115.4]): the
        // article rides once, on the first member — every printed disjunction
        // in this corpus leads with "a player", so only that member keeps its
        // "a "/"an " prefix; later members print as the bare noun.
        Predicate::Or(members) => {
            let mut parts = members.iter().map(|m| recipient_of(m, ctx));
            let head = parts.next().unwrap_or_default();
            std::iter::once(head)
                .chain(parts.map(|p| {
                    p.strip_prefix("a ")
                        .or_else(|| p.strip_prefix("an "))
                        .map_or(p.clone(), str::to_string)
                }))
                .collect::<Vec<_>>()
                .join(" or ")
        }
        other => subject_of(other, ctx),
    }
}

fn state_word(s: &StateChange) -> &'static str {
    match s {
        StateChange::Tapped => "tapped",
        StateChange::Untapped => "untapped",
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

/// Render a `Static` ability's one effect as its rules string. A
/// graveyard/hand-functioning static ([CR#113.6,604.3]) is now a
/// `StaticEffect::Conditionally(Is(this, InZone(zone)), inner)` wrapper
/// (`conditionally_qualified` gives it the "As long as ~ is in your <zone>,"
/// qualifier), so no separate `from`-zone step is needed here. The
/// adjacent-can't-attack-and-block merge ([CR#509] Pacifism shape) spans TWO
/// `Ability::Static` items now (one effect per `Static`), so it is re-homed
/// to the per-ability render loop in `render/mod.rs`, which sees both
/// abilities.
pub(super) fn static_ability(s: &StaticEffect, ctx: &Ctx) -> Vec<String> {
    static_effect(s, ctx).into_iter().collect()
}

/// Render one `StaticEffect` as a period-terminated sentence, or `None` for
/// effects that produce no text on their own.
pub(super) fn static_effect(e: &StaticEffect, ctx: &Ctx) -> Option<String> {
    static_effect_kind(e, ctx, false)
}

/// The one-shot twin of [`static_effect`] — a `Modify` inside
/// `OneShotEffect::Continuously`/`OneShotEffect::Until` ([CR#611.2]) reads
/// "gains X" for an ability grant, never the PERMANENT static's "has X"
/// (Collective Resistance's "Target creature gains hexproof and indestructible
/// until end of turn.", not "... has hexproof and has indestructible ...").
/// Every other `StaticEffect` shape reads identically either way, so this only
/// changes `Modify`'s verb choice.
pub(super) fn static_effect_one_shot(e: &StaticEffect, ctx: &Ctx) -> Option<String> {
    static_effect_kind(e, ctx, true)
}

fn static_effect_kind(e: &StaticEffect, ctx: &Ctx, one_shot: bool) -> Option<String> {
    match e {
        StaticEffect::Expanded(exp) => static_effect_kind(&exp.value, ctx, one_shot),
        // A bare `Modify` targets ONE object — singular agreement ("Test Aura
        // gets +1/+1.", "Enchanted creature gets +2/+2."). Two bespoke
        // real-card shapes (Exponential Growth's doubling verb, Embiggen's
        // triple-axis "for each ... it has" pump) read structurally
        // differently from the "gets +N/+N" family below, so they get their
        // own recognizers, checked first.
        StaticEffect::Modify(r, change) => {
            if let Some(clause) = doubling_power_clause(r, change, ctx)
                .or_else(|| axis_sum_pump_clause(r, change, ctx))
                .or_else(|| for_each_pump_clause(r, change, ctx))
            {
                return Some(format!("{clause}."));
            }
            let subj = super::fragment::modify_subject(r, ctx);
            Some(format!(
                "{subj} {}.",
                modifications_predicate(std::slice::from_ref(change), false, one_shot)
            ))
        }
        // `Each` distributes an inner effect over a `Selection` — plural
        // agreement ("Creatures you control get +1/+1.", the anthem shape,
        // [CR#613.6]). The inner effect is almost always `Modify(It, …)`;
        // the `It` binding itself carries no render-visible content (the
        // subject phrase already comes from the selection), so only the
        // inner `Modification` is read.
        StaticEffect::Each(sel, inner) => {
            let subj = super::fragment::each_subject(sel, ctx);
            let predicate = each_inner_predicate(inner, one_shot);
            Some(format!("{subj} {predicate}."))
        }
        // "As long as [condition], [inner]" ([CR#611.3a]).
        StaticEffect::Conditionally(cond, inner) => {
            let text = static_effect_kind(inner, ctx, one_shot)?;
            Some(conditionally_qualified(cond, ctx, text))
        }
        // "X becomes a copy of Y[, except …]" ([CR#707.4,707.9]) — the
        // becomes-a-copy static (Volrath's "becomes a copy of target
        // creature with a counter on it, except it's 7/5 and it has this
        // ability."). Subject agreement mirrors `Modify`'s bare-`Reference`
        // reading; the source/exceptions phrase is the same one every other
        // copy delivery site shares (`render/effect.rs`'s copy-effects
        // section).
        StaticEffect::BecomesCopy(who, spec) => {
            let subj = super::fragment::modify_subject(who, ctx);
            Some(format!(
                "{subj} becomes a copy of {}{}.",
                effect::copy_source_phrase(&spec.source, ctx),
                effect::copy_exceptions_clause(&spec.exceptions)
            ))
        }
        StaticEffect::ModifyPlayer(who, m) => Some(modify_player(who, m)),
        StaticEffect::TriggerMultiplier {
            cause,
            extra,
            affected,
        } => Some(trigger_multiplier(cause, extra, affected)),
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, ctx)),
        StaticEffect::Replacement(r) => Some(super::replacement::replacement(r, ctx)),
        StaticEffect::CantHappen(_event) => Some("[can't happen]".to_string()), /* keyword cards render via their template */
        StaticEffect::ReplaceRoll {
            query,
            extra,
            ignore,
        } => Some(replace_roll_sentence(query, extra, *ignore).unwrap_or_else(|| {
            format!(
                "[unrendered: ReplaceRoll {{ query: {query:?}, extra: {extra:?}, ignore: {ignore:?} }}]."
            )
        })),
        StaticEffect::PayPips(_class, act) => Some(pay_pips_keyword(act)),
        // A lone `OutcomeGate` (no adjacent partner for the paired-merge
        // shape in `render/mod.rs`'s per-ability loop) still reads as its own
        // full sentence: "You can't lose the game." ([CR#104],[CR#704]).
        StaticEffect::OutcomeGate { who, gate } => Some(
            super::outcome::outcome_gate_sentence(who, *gate).unwrap_or_else(|| {
                format!("[unrendered: OutcomeGate {{ who: {who:?}, gate: {gate:?} }}].")
            }),
        ),
        // "[objects] can be the targets of [agent] as though they didn't have
        // [keyword]" — the counterfactual overlay ([CR#609.4], Glaring
        // Spotlight): the game is evaluated as though the object didn't satisfy
        // `premise`, for the action named by `then` only.
        StaticEffect::AsThough(AsThough::Counterfactual { premise, then }) => {
            Some(asthough_counterfactual(premise, then))
        }
        other => Some(format!("[unrendered: {other:?}].")),
    }
}

/// The counterfactual overlay as a sentence. Renders the one shape with a real
/// card today — a `May(Target)` selector seen through a `Not(Has(keyword))`
/// premise (Glaring Spotlight) — and falls back to the structural marker for
/// any other inner/premise, which has no established phrasing yet.
fn asthough_counterfactual(premise: &Predicate, then: &Deontic) -> String {
    if let Deontic::May(DeonticAction::Target { by, on }) = peel_deontic(then)
        && let Some(seen_through) = premise_seen_through(premise)
    {
        let subject = super::fragment::capitalize(&super::fragment::filter_subject(on));
        return format!(
            "{subject} can be the targets of {} {seen_through}.",
            deed_agent_noun(by),
        );
    }
    format!("[unrendered: AsThough({premise:?}, {then:?})].")
}

fn peel_deontic(d: &Deontic) -> &Deontic {
    match d {
        Deontic::Expanded(e) => peel_deontic(&e.value),
        other => other,
    }
}

/// A two-slot targeting agent as a noun phrase — "spells and abilities you
/// control" ([CR#702.11d]'s two-armed agent read as a single subject).
fn deed_agent_noun(by: &DeedAgent) -> String {
    let control = by
        .stack_object
        .as_ref()
        .map(control_qualifier)
        .unwrap_or_default();
    format!("spells and abilities{control}")
}

/// The " you control" / " an opponent controls" qualifier of a `ControlledBy`
/// agent filter (empty for any other shape — falls back to a bare agent noun).
fn control_qualifier(f: &Predicate) -> String {
    if let Predicate::Relation(RelationPredicate::ControlledBy(inner)) =
        super::fragment::strip_expanded(f)
    {
        let inner = super::fragment::strip_expanded(inner);
        if matches!(inner, Predicate::Ref(Reference::You)) {
            return " you control".to_string();
        }
        if let Predicate::Relation(RelationPredicate::OpponentOf(who)) = inner
            && matches!(
                super::fragment::strip_expanded(who),
                Predicate::Ref(Reference::You)
            )
        {
            return " an opponent controls".to_string();
        }
    }
    String::new()
}

/// "as though they didn't have [keyword]" from a `Not(Has(K))` premise — the
/// counterfactual read as a trailing clause. `None` for a premise shape with no
/// established phrasing.
fn premise_seen_through(premise: &Predicate) -> Option<String> {
    if let Predicate::Not(inner) = premise
        && let Predicate::Characteristic(CharacteristicPredicate::Has(kw)) = inner.as_ref()
    {
        return Some(format!(
            "as though they didn't have {}",
            kw.as_str().to_lowercase()
        ));
    }
    None
}

/// The predicate half of an `Each`'s inner effect — everything after the
/// selection-derived subject phrase, with NO trailing period (the caller adds
/// it). The inner effect is normally `Modify(It, change)` (the anthem); `It`
/// itself renders nothing here (the subject already came from the
/// selection), so only `change` is read. Any other inner shape has no
/// established plural predicate reading yet, so it falls back to the
/// structural marker rather than guessing.
fn each_inner_predicate(inner: &StaticEffect, one_shot: bool) -> String {
    match inner {
        StaticEffect::Expanded(exp) => each_inner_predicate(&exp.value, one_shot),
        StaticEffect::Modify(_, change) => {
            modifications_predicate(std::slice::from_ref(change), true, one_shot)
        }
        other => format!("[unrendered each-inner: {other:?}]"),
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
            if super::fragment::find_card_type(filter) == Some(Type::Artifact.name()) =>
        {
            "Improvise".to_string()
        }
        PayAct::TapToPay(_) => "Convoke".to_string(),
    }
}

/// Krark's Thumb's own phrasing ([CR#614.3] roll-more replacement;
/// [CR#706.6] "ignore one"): "If you would flip a coin, instead flip two
/// coins and ignore one." Recognizes exactly the ONE real-card shape this
/// grammar node has today — a plain (any-actor-narrowed) flip-coin `query`,
/// one extra flip (`extra` literal `1`), one flip discarded by the
/// flipper's choice (`ignore: IgnoreChosen(1)`, [CR#706.6]) — and falls back
/// to the structural marker for any other `query`/`extra`/`ignore`
/// combination (a die-roll query, a different extra count, `IgnoreLowest`,
/// …), which has no established phrasing in this corpus yet.
fn replace_roll_sentence(query: &EventFilter, extra: &Count, ignore: IgnoreRule) -> Option<String> {
    if is_flip_coin_query(query)
        && extra.literal_value() == Some(1)
        && matches!(ignore, IgnoreRule::IgnoreChosen(1))
    {
        Some("If you would flip a coin, instead flip two coins and ignore one.".to_string())
    } else {
        None
    }
}

/// A "flip a coin" query, any actor/won narrowing — looks through a
/// remembered macro invocation the same way `is_this_enters`/`is_tap_this`
/// (`render/replacement.rs`) look through theirs.
fn is_flip_coin_query(e: &EventFilter) -> bool {
    match e {
        EventFilter::Expanded(exp) => is_flip_coin_query(&exp.value),
        EventFilter::CoinFlipped { won: None, .. } => true,
        _ => false,
    }
}

// ── Bespoke real-card `Modify` shapes ────────────────────────────────────────

/// "double {subj}'s power {X} times" — Exponential Growth's own imperative
/// verb, structurally distinct from the "gets +N/+N" pump family below (it is
/// not additive-delta phrasing at all). The recognized shape is the doubling
/// delta [`Count::Pow`] builds: `Up(Minus(Times(StatOf(r, Power), Pow(2,
/// exp)), StatOf(r, Power)))` — current power times 2^exp, minus current
/// power, i.e. "gets +X/+0" where X takes power to its post-doubling value
/// (the official ruling's own phrasing). `None` for any other shape.
fn doubling_power_clause(r: &Reference, change: &Modification, ctx: &Ctx) -> Option<String> {
    use deckmaste_semantics::Stat;

    let Modification::Power(NumericOp::Up(delta)) = change else {
        return None;
    };
    let Count::Minus(a, b) = delta else { return None };
    let Count::Times(base, exp) = a.as_ref() else {
        return None;
    };
    let Count::StatOf(base_ref, Stat::Power) = base.as_ref() else {
        return None;
    };
    let Count::Pow(two, x) = exp.as_ref() else {
        return None;
    };
    if !matches!(two.as_ref(), Count::Literal(2)) {
        return None;
    }
    let Count::StatOf(sub_ref, Stat::Power) = b.as_ref() else {
        return None;
    };
    if base_ref != r || sub_ref != r {
        return None;
    }
    let subj = super::fragment::reference(r, ctx);
    let times_word = super::fragment::count(x);
    Some(format!("double {subj}'s power {times_word} times"))
}

/// "{subj} gets +1/+1 for each supertype, card type, and subtype it has" —
/// Embiggen's own real-card phrasing for a triple-axis
/// [`Countable::Singleton`](deckmaste_semantics::Countable::Singleton) sum,
/// whose per-object-axis-enumeration idiom ("... it has") reads nothing like
/// the group-fold `CountDistinct(_, Objects(..))` phrasing (Domain/Coven's "the
/// number of X among Y"). The recognized shape is `Several([Power(Up(c)),
/// Toughness(Up(c))])` with the SAME `c = Plus(Plus(CountDistinct(Supertypes,
/// Singleton(r)), CountDistinct(Types, Singleton(r))), CountDistinct(
/// Subtypes, Singleton(r)))` for both — the +1/+1-per-unit case (a
/// coefficient-`k` "+k/+k for each ..." generalization is unforced until a
/// real card needs it). `None` for any other shape.
fn axis_sum_pump_clause(r: &Reference, change: &Modification, ctx: &Ctx) -> Option<String> {
    use deckmaste_semantics::Characteristic;
    use deckmaste_semantics::Countable;
    use deckmaste_semantics::Expand;

    let normalized = change.clone().expand_all();
    let Modification::Several(parts) = &normalized else {
        return None;
    };
    let [
        Modification::Power(NumericOp::Up(power_delta)),
        Modification::Toughness(NumericOp::Up(toughness_delta)),
    ] = parts.as_ref()
    else {
        return None;
    };
    if power_delta != toughness_delta {
        return None;
    }
    let is_singleton_axis = |c: &Count, want: Characteristic| {
        matches!(
            c,
            Count::CountDistinct(a, Countable::Singleton(rr)) if *a == want && rr.as_ref() == r
        )
    };
    let Count::Plus(ab, subtypes) = power_delta else {
        return None;
    };
    let Count::Plus(supertypes, types) = ab.as_ref() else {
        return None;
    };
    if !is_singleton_axis(supertypes, Characteristic::Supertypes)
        || !is_singleton_axis(types, Characteristic::Types)
        || !is_singleton_axis(subtypes, Characteristic::Subtypes)
    {
        return None;
    }
    let subj = super::fragment::reference(r, ctx);
    Some(format!(
        "{subj} gets +1/+1 for each supertype, card type, and subtype it has"
    ))
}

/// "{subj} gets +P/+T for each <noun>" — the plain "for each `<selection>`"
/// scaled pump family ([CR#107.3] "for each"; Blanchwood Armor, Primal
/// Bellow, Might of the Masses, Goblin Piledriver's own "+2/+0" coefficient).
/// Because render flattens a change-bundling macro through `Expanded` before
/// `modifications_predicate` ever runs (its `pt_delta_clause` piece matches
/// only `Count::Literal`), this raw structural shape needs its own
/// recognizer — mirrors [`axis_sum_pump_clause`]'s symmetric-`Several` shape,
/// with `c = CountOf(Objects(pred))` (coefficient 1) or `Times(Literal(n),
/// CountOf(Objects(pred)))` (coefficient n). A lone `Power(Up(c))` (no
/// `Toughness` node at all — the stored `P1P0ForEach`/`P2P0ForEach`/
/// `P3P0ForEach` macro body shape) reads "+n/+0".
///
/// A `Several`'s two axes are evaluated INDEPENDENTLY via [`for_each_axis`]:
/// each is either SCALED (a `for_each_magnitude` shape, carrying a
/// coefficient and a selection predicate) or FLAT ZERO (`Literal(0)`, an
/// idle axis). At least one axis must be scaled; if BOTH are, their
/// predicates must be equal (the shared selection). This is the OTHER
/// asymmetric shape distinct from the lone-`Power` case above: a raw
/// `Several([Power(Up(Times(Literal(n), CountOf(Objects(pred))))),
/// Toughness(Up(Literal(0)))])` with one axis scaled and the other flat
/// zero. Goblin Piledriver itself does NOT take this shape — its stored
/// `P2P0ForEach` invocation IS the lone-`Power` case above (`P2P0ForEach`'s
/// body is a bare `Power(Up(Times(Literal(2), CountOf(Objects(Param(0))))))`,
/// no `Toughness` node); no card in the corpus currently stores this raw
/// asymmetric `Several` shape — the recognizer covers it defensively (a
/// hand-built asymmetric pump would take it), exercised only synthetically
/// in the test below (reusing Goblin Piledriver's predicate/wording).
///
/// Looks through only a `Modification::Expanded` wrapper on `change` itself
/// (mirrors `effect::modification_has_dynamic_pt_delta`'s shallow
/// look-through) — NOT a deep [`Expand::expand_all`], which would strip the
/// macro-provenance `Predicate::Expanded` markers `fragment::filter_noun`
/// reads off the nested selection filter (e.g. the bare `Permanent` macro
/// atom in Primal Bellow's `And([Permanent, Subtype("Forest"), …])`). `None`
/// for any other shape.
fn for_each_pump_clause(r: &Reference, change: &Modification, ctx: &Ctx) -> Option<String> {
    let (p, t, pred) = match strip_modification_expanded(change) {
        Modification::Several(parts) => {
            let [
                Modification::Power(NumericOp::Up(power_delta)),
                Modification::Toughness(NumericOp::Up(toughness_delta)),
            ] = parts.as_ref()
            else {
                return None;
            };
            let (p, power_pred) = for_each_axis(power_delta)?;
            let (t, toughness_pred) = for_each_axis(toughness_delta)?;
            let pred = match (power_pred, toughness_pred) {
                (Some(pp), Some(tp)) => {
                    if pp != tp {
                        return None;
                    }
                    pp
                }
                (Some(pp), None) => pp,
                (None, Some(tp)) => tp,
                // Both axes flat zero: no scaled axis at all, so this isn't
                // a "for each" pump.
                (None, None) => return None,
            };
            (p, t, pred)
        }
        Modification::Power(NumericOp::Up(delta)) => {
            let (coeff, pred) = for_each_magnitude(delta)?;
            (coeff, 0, pred)
        }
        _ => return None,
    };
    // `modify_subject` (not the plain `reference`): this clause always reads
    // as a full sentence subject in EITHER position it's used from — the bare
    // top-level static's own line (Blanchwood Armor's "Enchanted creature
    // gets …", needing `modify_subject`'s `AttachHostOf(This)` -> "Enchanted
    // creature" arm `reference` lacks) or a `Continuously`-wrapped one-shot
    // (Primal Bellow, Goblin Piledriver), whose `duration_qualified` always
    // `lower_first`s this clause anyway (a "for each" delta is always
    // `leads`), so the capitalized form costs nothing there.
    let subj = super::fragment::modify_subject(r, ctx);
    Some(format!(
        "{subj} gets +{p}/+{t} for each {}",
        super::fragment::filter_noun(pred)
    ))
}

/// Look through a `Modification::Expanded` wrapper — the shallow,
/// single-layer twin of `effect::modification_has_dynamic_pt_delta`'s own
/// look-through, kept local since [`for_each_pump_clause`] needs the
/// reference (not a bool).
fn strip_modification_expanded(m: &Modification) -> &Modification {
    match m {
        Modification::Expanded(exp) => strip_modification_expanded(&exp.value),
        other => other,
    }
}

/// Look through a `Count::Expanded` wrapper — the shallow, single-layer
/// twin of [`strip_modification_expanded`], used at each position
/// [`for_each_magnitude`]/[`for_each_axis`] inspect.
fn strip_count_expanded(c: &Count) -> &Count {
    match c {
        Count::Expanded(exp) => strip_count_expanded(&exp.value),
        other => other,
    }
}

/// The magnitude half of [`for_each_pump_clause`]'s recognized shape:
/// `CountOf(Objects(pred))` (coefficient 1) or `Times(Literal(n),
/// CountOf(Objects(pred)))` (coefficient n, [CR#107.1] "twice X"). Looks
/// through a `Count::Expanded` wrapper at each position, same shallow
/// look-through reason as [`strip_modification_expanded`]. `None` for any
/// other `Count` shape.
fn for_each_magnitude(c: &Count) -> Option<(i64, &Predicate)> {
    use deckmaste_semantics::Countable;

    match strip_count_expanded(c) {
        Count::CountOf(Countable::Objects(pred)) => Some((1, pred.as_ref())),
        Count::Times(a, b) => match (strip_count_expanded(a), strip_count_expanded(b)) {
            (Count::Literal(n), Count::CountOf(Countable::Objects(pred))) => {
                Some((i64::from(*n), pred.as_ref()))
            }
            _ => None,
        },
        _ => None,
    }
}

/// One axis of a `Several([Power(Up(_)), Toughness(Up(_))])` pump's `Up`
/// operand ([CR#107.3] "for each") — SCALED (delegates to
/// [`for_each_magnitude`], carrying a coefficient and the selection
/// predicate) or FLAT ZERO (`Literal(0)`, the idle axis of an asymmetric
/// pump — a hand-built "+2/+0 for each other attacking Goblin" `Several`
/// would carry this as its `Toughness(Up(Literal(0)))` half; Goblin
/// Piledriver's ACTUAL stored ability is the lone-`Power` `P2P0ForEach`
/// invocation, not this `Several` shape). `None` for any other `Count`
/// shape — [`for_each_pump_clause`] then rejects the whole `Several`.
fn for_each_axis(c: &Count) -> Option<(i64, Option<&Predicate>)> {
    if matches!(strip_count_expanded(c), Count::Literal(0)) {
        return Some((0, None));
    }
    let (coeff, pred) = for_each_magnitude(c)?;
    Some((coeff, Some(pred)))
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
fn modifications_predicate(changes: &[Modification], plural: bool, one_shot: bool) -> String {
    let mut clauses: Vec<String> = Vec::new();

    // Flatten change-bundling macros (`PowerAndToughnessUp`/`Down` →
    // `Several([Power, Toughness])`, looked through `Expanded`) so the grouping
    // below renders identically to the inline pair — the graduated-RON change
    // is cosmetic.
    let changes = Modification::flatten(changes);
    let changes = changes.as_ref();

    // Pre-scan to compute the combined values for the grouped cases.
    let delta = pt_delta_clause(changes, plural);
    let base = base_pt_clause(changes, plural);
    let gained = gain_ability_clause(changes, plural, one_shot);

    let mut delta_emitted = false;
    let mut base_emitted = false;
    let mut gained_emitted = false;

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
            // Every `GainAbility` in the list merges into ONE clause,
            // emitted at the first occurrence — "gains hexproof and
            // indestructible", never "gains hexproof and gains
            // indestructible" ([CR#613.1f]).
            Modification::GainAbility(_) => {
                if !gained_emitted {
                    if let Some(ref g) = gained {
                        clauses.push(g.clone());
                    }
                    gained_emitted = true;
                }
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

/// "have flying and vigilance" (static) / "gain flying and vigilance"
/// (one-shot) from every `GainAbility` in the list, `None` if there are
/// none. `one_shot` picks the verb ([CR#613.1f] grant vs [CR#611.2] a
/// duration-bound one-shot grant use different oracle verbs for the
/// identical layer-6 effect).
fn gain_ability_clause(changes: &[Modification], plural: bool, one_shot: bool) -> Option<String> {
    let nouns: Vec<String> = changes
        .iter()
        .filter_map(|m| match m {
            Modification::GainAbility(a) => Some(ability_noun(a)),
            _ => None,
        })
        .collect();
    if nouns.is_empty() {
        return None;
    }
    let verb = if one_shot { gain(plural) } else { have(plural) };
    Some(format!("{verb} {}", nouns.join(" and ")))
}

/// "have base power and toughness N/M" from `Power(Set)` + `Toughness(Set)`.
fn base_pt_clause(changes: &[Modification], plural: bool) -> Option<String> {
    let mut sp: Option<i64> = None;
    let mut st: Option<i64> = None;
    for c in changes {
        match c {
            Modification::Power(NumericOp::Set(StatValue::Number(n))) => sp = Some(i64::from(*n)),
            Modification::Toughness(NumericOp::Set(StatValue::Number(n))) => {
                st = Some(i64::from(*n));
            }
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
fn trigger_multiplier(cause: &EventFilter, extra: &Count, affected: &Predicate) -> String {
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
fn cause_phrase(cause: &EventFilter) -> String {
    if let EventFilter::ZoneChange {
        what,
        to: Some(deckmaste_semantics::Zone::Battlefield),
        ..
    } = cause
    {
        // Modern oracle elides the zone: "an artifact or creature entering
        // causes …" (Panharmonicon).
        return format!("{} entering", types_noun(what));
    }
    "an event".to_string()
}

/// The affected-source clause: the "you control" default renders as "a
/// permanent you control"; anything else as a generic "an affected permanent".
fn affected_phrase(affected: &Predicate) -> String {
    use deckmaste_semantics::RelationPredicate;
    if matches!(
        affected,
        Predicate::Relation(RelationPredicate::ControlledBy(inner))
            if matches!(&**inner, Predicate::Ref(Reference::You))
    ) {
        return "a permanent you control".to_string();
    }
    "an affected permanent".to_string()
}

/// A type filter as an indefinite noun: a single `Type` → "a creature"; a
/// `Or` of types → "an artifact or creature"; anything else → "an object".
fn types_noun(what: &Predicate) -> String {
    let names: Vec<String> = match what {
        Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
            vec![t.name().as_str().to_lowercase()]
        }
        Predicate::Or(items) => items
            .iter()
            .filter_map(|f| match f {
                Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                    Some(t.name().as_str().to_lowercase())
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

/// A [`EventFilter::Cast`]'s `who:` narrowing as the subject+verb phrase
/// leading the `what:` noun phrase ([CR#601.2i]): "you cast" (2nd person,
/// `Ref(You)`), "a player casts" (3rd person, the `Player` macro expanding to
/// `Kind(Player)`), "an opponent casts" (3rd person, `OpponentOf(Ref(You))` —
/// mirrors [`recipient_of`]'s "an opponent" reading). `None` for any other
/// narrowing (no real card in this corpus needs one yet).
fn cast_who_phrase(who: &Predicate) -> Option<&'static str> {
    match super::fragment::strip_expanded(who) {
        Predicate::Ref(Reference::You) => Some("you cast"),
        Predicate::Kind(ObjectKind::Player) => Some("a player casts"),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(
                super::fragment::strip_expanded(inner),
                Predicate::Ref(Reference::You)
            ) =>
        {
            Some("an opponent casts")
        }
        _ => None,
    }
}

/// A [`EventFilter::Cast`]'s `what:` narrowing as the noun phrase following
/// the [`cast_who_phrase`] subject+verb ([CR#601.2i]). Self (`Ref(This)`)
/// reads as the real oracle "this spell" convention (Cascade's reminder
/// text) — never the card's own printed name, unlike an ETB/dies subject: a
/// cast trigger describes the spell ON THE STACK referring to itself. A bare
/// `Kind(Spell)` is "a spell"; anything else pairs `Kind(Spell)` with up to
/// TWO characteristic/state atoms — at most one HEAD atom (a
/// card-type/subtype, its negation, or a type/subtype disjunction —
/// [`cast_head_phrase`]) fixing the noun before "spell", and at most one
/// POSTFIX atom (a mana-value threshold or a "that targets ~" clause —
/// [`cast_postfix_phrase`]) trailing after it — e.g. "a creature spell with
/// mana value 3 or less" pairs both. Falls back to the structural marker for
/// any other narrowing this v1 production doesn't model (restriction-laden
/// forms: first/second spell each turn, a color-composite filter, …).
fn cast_subject(what: &Predicate) -> String {
    let what = super::fragment::strip_expanded(what);
    if matches!(what, Predicate::Ref(Reference::This)) {
        return "this spell".to_string();
    }
    if matches!(what, Predicate::Kind(ObjectKind::Spell)) {
        return "a spell".to_string();
    }
    let Predicate::And(parts) = what else {
        return format!("[unrendered: {what:?}]");
    };
    let rest: Vec<&Predicate> = parts
        .iter()
        .map(super::fragment::strip_expanded)
        .filter(|p| !matches!(p, Predicate::Kind(ObjectKind::Spell)))
        .collect();
    if rest.is_empty() || rest.len() > 2 {
        return format!("[unrendered: {what:?}]");
    }
    let mut head: Option<&Predicate> = None;
    let mut postfix: Option<String> = None;
    for atom in rest {
        if let Some(phrase) = cast_postfix_phrase(atom) {
            if postfix.is_some() {
                return format!("[unrendered: {what:?}]");
            }
            postfix = Some(phrase);
        } else if head.is_none() {
            head = Some(atom);
        } else {
            return format!("[unrendered: {what:?}]");
        }
    }
    let base = head.map_or_else(|| "a spell".to_string(), cast_head_phrase);
    if base.starts_with("[unrendered") {
        return base;
    }
    match postfix {
        Some(p) => format!("{base} {p}"),
        None => base,
    }
}

/// The HEAD atom of a [`cast_subject`] narrowing — the noun immediately
/// before "spell": a card-type negation ("a non&lt;type&gt; spell"), a single
/// catalog subtype (the Prowess/"an Elf spell" shape), a card-type
/// disjunction ([`types_noun`], "an artifact or creature spell"), or a
/// catalog-subtype disjunction ("a Spirit or Arcane spell", tried only once
/// the type-disjunction reading fails — the two kinds are never mixed, same
/// as the migration parser's `disjunction_atom`). Falls back to the
/// structural marker for any other atom shape.
fn cast_head_phrase(only: &Predicate) -> String {
    if let Predicate::Not(inner) = only {
        return match super::fragment::strip_expanded(inner) {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                format!("a non{} spell", t.name().as_str().to_lowercase())
            }
            other => format!("[unrendered: {other:?}]"),
        };
    }
    if let Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) = only {
        return format!(
            "{} {} spell",
            article_for(name.name().as_str()),
            name.name()
        );
    }
    if let Predicate::Or(items) = only {
        return subtype_disjunction_noun(items).map_or_else(
            || format!("{} spell", types_noun(only)),
            |noun| format!("{noun} spell"),
        );
    }
    format!("{} spell", types_noun(only))
}

/// The catalog-subtype reading of a `Predicate::Or` disjunction's members —
/// "a Spirit or Arcane spell" — or `None` if any member isn't a bare
/// `Subtype` atom (so [`cast_head_phrase`] falls back to [`types_noun`]'s
/// card-type reading, which itself degrades to "an object" if that ALSO
/// misses — the same silent-drop behavior it already had before this
/// function existed, unchanged here).
fn subtype_disjunction_noun(items: &[Predicate]) -> Option<String> {
    let names: Vec<&str> = items
        .iter()
        .map(|f| match f {
            Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) => {
                Some(name.name().as_str())
            }
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;
    let joined = names.join(" or ");
    Some(format!("{} {joined}", article_for(&joined)))
}

/// The POSTFIX atom of a [`cast_subject`] narrowing — a clause trailing
/// "spell": a mana-value threshold ("with mana value N or greater/less",
/// [CR#202.3]) or a "that targets ~" clause (heroic's head, [CR#115.9b] —
/// restricted to the self target, the only shape real oracle text uses
/// here). `None` for any other atom (a head-position atom — the caller tries
/// [`cast_head_phrase`] instead).
fn cast_postfix_phrase(atom: &Predicate) -> Option<String> {
    match atom {
        Predicate::Characteristic(CharacteristicPredicate::Stat(Stat::ManaValue, cmp, count)) => {
            let n = count.literal_value()?;
            let bound = match cmp {
                Cmp::AtLeast => "or greater",
                Cmp::AtMost => "or less",
                _ => return None,
            };
            Some(format!("with mana value {n} {bound}"))
        }
        Predicate::State(StatePredicate::Targets(inner))
            if super::fragment::strip_expanded(inner).is_this() =>
        {
            Some("that targets ~".to_string())
        }
        _ => None,
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
fn gain(plural: bool) -> &'static str {
    if plural { "gain" } else { "gains" }
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
    use std::sync::Arc;

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

    /// The trailing "Activate only …" rider — a window, a use-limit, both
    /// combined, and neither — renders back to the parser's own peeled
    /// sentence; a `LoyaltyOncePerTurn` limit suppresses the whole rider
    /// (a loyalty ability's sorcery-speed-only + shared-once-per-turn gate
    /// is an implicit rule, never printed).
    #[test]
    fn activation_rider_renders_window_and_limit() {
        use deckmaste_semantics::Action;
        use deckmaste_semantics::ActivatedAbility;
        use deckmaste_semantics::Cost;
        use deckmaste_semantics::LifeOp;
        use deckmaste_semantics::OneShotEffect;
        use deckmaste_semantics::Timing;
        use deckmaste_semantics::UseLimit;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let base = ActivatedAbility {
            ability_word: None,
            cost: Cost(vec![].into()),
            from: None,
            window: None,
            condition: None,
            limits: vec![].into(),
            effect: OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1)),
            )),
        };

        let sorcery = ActivatedAbility {
            window: Some(Timing::SorcerySpeed),
            ..base.clone()
        };
        assert_eq!(
            activation_rider(&sorcery, &ctx),
            " Activate only as a sorcery."
        );

        let once_per_turn = ActivatedAbility {
            limits: vec![UseLimit::OncePerTurn].into(),
            ..base.clone()
        };
        assert_eq!(
            activation_rider(&once_per_turn, &ctx),
            " Activate only once each turn."
        );

        let combo = ActivatedAbility {
            window: Some(Timing::SorcerySpeed),
            limits: vec![UseLimit::OncePerTurn].into(),
            ..base.clone()
        };
        assert_eq!(
            activation_rider(&combo, &ctx),
            " Activate only as a sorcery and only once each turn."
        );

        assert_eq!(activation_rider(&base, &ctx), "");

        let loyalty = ActivatedAbility {
            window: Some(Timing::SorcerySpeed),
            limits: vec![UseLimit::LoyaltyOncePerTurn].into(),
            ..base
        };
        assert_eq!(activation_rider(&loyalty, &ctx), "");
    }

    /// A bare `PayPips` static renders to its keyword name
    /// ([CR#702.51a,702.66a,702.126a]): delve exiles → "Delve"; convoke /
    /// improvise tap, told apart by what they tap (creature → "Convoke",
    /// artifact → "Improvise"). (Keyword cards render via their macro template;
    /// this is the fallback arm for a directly written static.)
    #[test]
    fn pay_pips_renders_its_keyword_name() {
        use deckmaste_semantics::PayAct;
        use deckmaste_semantics::PipClass;
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::StatePredicate;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let you = || {
            Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                Reference::You,
            ))))
        };
        let ty = |t: Type| Predicate::r#type(t);

        let convoke = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Predicate::And(vec![ty(Type::Creature), you()].into())),
        );
        assert_eq!(static_effect(&convoke, &ctx).as_deref(), Some("Convoke"));

        let improvise = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Predicate::And(vec![ty(Type::Artifact), you()].into())),
        );
        assert_eq!(
            static_effect(&improvise, &ctx).as_deref(),
            Some("Improvise")
        );

        let delve = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::ExileToPay(Predicate::State(StatePredicate::InZone(Zone::Graveyard))),
        );
        assert_eq!(static_effect(&delve, &ctx).as_deref(), Some("Delve"));
    }

    /// `EventFilter::Damage { combat: Some(true), .. }` — the
    /// `DealsCombatDamage` macro's expansion — renders back to oracle
    /// "deals combat damage to <recipient>" text, over the recipient forms
    /// [`recipient_of`] models: the dominant "a player", a bare object type
    /// ("a creature"), the player-identity "an opponent", and an `Or([...])`
    /// disjunction ("a player or planeswalker", [CR#115.4]).
    #[test]
    fn damage_combat_event_clause_renders_recipients() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let this = || Predicate::Ref(Reference::This);
        let event = |to| EventFilter::Damage {
            source: this(),
            to,
            combat: Some(true),
            amount: None,
        };

        assert_eq!(
            event_clause(&event(Predicate::Kind(ObjectKind::Player)), &ctx),
            (
                "Whenever",
                "Test deals combat damage to a player".to_string()
            )
        );
        assert_eq!(
            event_clause(&event(Predicate::r#type(Type::Creature)), &ctx),
            (
                "Whenever",
                "Test deals combat damage to a creature".to_string()
            )
        );
        assert_eq!(
            event_clause(
                &event(Predicate::Relation(RelationPredicate::OpponentOf(
                    Arc::new(Predicate::Ref(Reference::You))
                ))),
                &ctx
            ),
            (
                "Whenever",
                "Test deals combat damage to an opponent".to_string()
            )
        );
        assert_eq!(
            event_clause(
                &event(Predicate::Or(
                    vec![
                        Predicate::Kind(ObjectKind::Player),
                        Predicate::r#type(Type::Planeswalker),
                    ]
                    .into()
                )),
                &ctx
            ),
            (
                "Whenever",
                "Test deals combat damage to a player or planeswalker".to_string()
            )
        );
    }

    /// `EventFilter::Damage { source: Any, combat: None, .. }` — the
    /// `DealtDamage` macro's expansion (generic, non-combat-narrowed damage;
    /// the recipient-subject passive phrasing "<recipient> is dealt
    /// damage"). Round-trips the self ("When" lead) and object/player-
    /// identity recipient forms.
    #[test]
    fn dealt_damage_event_clause_renders_recipients() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let event = |to| EventFilter::Damage {
            source: Predicate::Any,
            to,
            combat: None,
            amount: None,
        };

        assert_eq!(
            event_clause(&event(Predicate::Ref(Reference::This)), &ctx),
            ("When", "Test is dealt damage".to_string())
        );
        assert_eq!(
            event_clause(&event(Predicate::r#type(Type::Creature)), &ctx),
            ("Whenever", "a creature is dealt damage".to_string())
        );
        assert_eq!(
            event_clause(
                &event(Predicate::Relation(RelationPredicate::OpponentOf(
                    Arc::new(Predicate::Ref(Reference::You))
                ))),
                &ctx
            ),
            ("Whenever", "an opponent is dealt damage".to_string())
        );
    }

    /// `EventFilter::ZoneChange { from: Some(Battlefield), to: None, .. }` —
    /// the `LeavesBattlefield`/`ThisLeavesBattlefield` macros' expansion
    /// ([CR#603.6c]). Round-trips the self ("When" lead, mirrors `ThisDies`'s
    /// render) and the bare object-type filtered subject ("Whenever a
    /// creature leaves the battlefield", mirrors `Dies`'s). Structurally
    /// distinct from the "dies" shape (`to: Some(Graveyard)`) — never
    /// collides with `event_clause`'s dies-shape renders.
    #[test]
    fn leaves_battlefield_event_clause_renders_self_and_filtered_subject() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let event = |what| EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: None,
            cause: None,
        };

        assert_eq!(
            event_clause(&event(Predicate::Ref(Reference::This)), &ctx),
            ("When", "Test leaves the battlefield".to_string())
        );
        assert_eq!(
            event_clause(&event(Predicate::r#type(Type::Creature)), &ctx),
            ("Whenever", "a creature leaves the battlefield".to_string())
        );
    }

    /// `EventFilter::OneOf` keeps each event's master-form pairing while the
    /// renderer factors the shared printed surface. Covers both directions:
    /// one subject with multiple events and one event over multiple subjects.
    /// Compound card names retain Oracle's plural agreement.
    #[test]
    fn event_disjunction_renders_shared_subject_or_verb() {
        use deckmaste_semantics::CharacteristicPredicate;
        use deckmaste_semantics::RelationPredicate;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let this = || Predicate::Ref(Reference::This);
        let enters = |what| EventFilter::ZoneChange {
            what,
            from: None,
            to: Some(Zone::Battlefield),
            cause: None,
        };
        let dies = |what| EventFilter::ZoneChange {
            what,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let attacks = |by| EventFilter::AttackDeclared {
            by,
            against: Predicate::Any,
        };

        assert_eq!(
            event_clause(
                &EventFilter::OneOf(vec![enters(this()), dies(this())].into()),
                &ctx
            ),
            ("When", "Test enters or dies".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::OneOf(vec![enters(this()), attacks(this())].into()),
                &ctx
            ),
            ("Whenever", "Test enters or attacks".to_string())
        );

        let another_ally = Predicate::And(
            vec![
                Predicate::State(deckmaste_semantics::StatePredicate::InZone(
                    Zone::Battlefield,
                )),
                Predicate::Characteristic(CharacteristicPredicate::Subtype(
                    deckmaste_semantics::SubtypeRef::named("Ally".into()),
                )),
                Predicate::Not(Arc::new(this())),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        assert_eq!(
            event_clause(
                &EventFilter::OneOf(vec![enters(this()), enters(another_ally)].into()),
                &ctx
            ),
            (
                "Whenever",
                "Test or another Ally you control enters".to_string()
            )
        );

        let compound_ctx = Ctx {
            subject: "Krang & Shredder",
            ..ctx
        };
        assert_eq!(
            event_clause(
                &EventFilter::OneOf(vec![enters(this()), attacks(this())].into()),
                &compound_ctx
            ),
            ("Whenever", "Krang & Shredder enter or attack".to_string())
        );
    }

    /// `EventFilter::StepBegins` bodies get NO antecedent to call "it" — the
    /// step names itself, not an object ([CR#603.2b]; mirrors Idris
    /// `EventCaps.hasObject = False`), so a self-referencing body must name
    /// itself the [CR#603.4]-family way (`self_type_phrase`), same as the
    /// `CoinFlipped`/`DiceRolled`/`RollPlanarDie`/`TapForMana` family just
    /// above. Round-trips the WHOLE oracle sentence — not just the event
    /// clause — for the "at the beginning of your upkeep, sacrifice ~[
    /// unless you pay <cost>]" family end to end (parse-shape construction
    /// -> full `triggered()` render), against REAL printed Oracle text:
    /// Necrotic Plague's granted "At the beginning of your upkeep, sacrifice
    /// this creature." and Aura Flux's granted "At the beginning of your
    /// upkeep, sacrifice this enchantment unless you pay {2}."
    #[test]
    fn step_begins_self_sacrifice_names_its_own_type() {
        use deckmaste_semantics::Action;
        use deckmaste_semantics::BeginningStep;
        use deckmaste_semantics::Cost;
        use deckmaste_semantics::CostComponent;
        use deckmaste_semantics::ManaCost;
        use deckmaste_semantics::ManaSymbol;
        use deckmaste_semantics::May;
        use deckmaste_semantics::OneShotEffect;
        use deckmaste_semantics::PhaseStep;
        use deckmaste_semantics::SimpleManaSymbol;
        use deckmaste_semantics::WhoseTurn;

        let event = EventFilter::StepBegins {
            at: PhaseStep::Beginning(BeginningStep::Upkeep),
            whose: WhoseTurn::Your,
        };
        let sacrifice_this =
            || OneShotEffect::Act(Action::Sacrifice(Reference::You, Reference::This));
        let bare = |event: EventFilter, effect: OneShotEffect| TriggeredAbility {
            ability_word: None,
            event,
            from: None,
            condition: None,
            limits: [].into(),
            where_x: None,
            effect,
        };

        // Unconditional (Necrotic Plague's granted ability).
        let unconditional = bare(event.clone(), sacrifice_this());
        let creature_view = CardView {
            name: "Bog Elemental",
            mana_cost: None,
            supertypes: &[],
            types: &[Type::Creature.def()],
            subtypes: &[],
            power: None,
            toughness: None,
            abilities: &[],
        };
        assert_eq!(
            triggered(&unconditional, &creature_view),
            "At the beginning of your upkeep, sacrifice this creature."
        );

        // The "unless you pay <mana>" toll (the collapsed `May(Pay(cost))`
        // `MustPay` shape, the `Unless` macro's read-time expansion,
        // [CR#118.12a]) — Aura Flux's real Oracle text, verbatim.
        let toll = bare(
            event,
            OneShotEffect::May(May {
                who: Reference::You,
                effect: Arc::new(OneShotEffect::Act(Action::Pay(Cost(
                    vec![CostComponent::Mana(ManaCost::from(
                        Arc::<[ManaSymbol]>::from(vec![ManaSymbol::Simple(
                            SimpleManaSymbol::Generic(2),
                        )]),
                    ))]
                    .into(),
                )))),
                if_did: None,
                if_not: Some(Arc::new(sacrifice_this())),
            }),
        );
        let enchantment_view = CardView {
            name: "Aura Flux",
            mana_cost: None,
            supertypes: &[],
            types: &[Type::Enchantment.def()],
            subtypes: &[],
            power: None,
            toughness: None,
            abilities: &[],
        };
        assert_eq!(
            triggered(&toll, &enchantment_view),
            "At the beginning of your upkeep, sacrifice this enchantment unless you pay {2}."
        );
    }

    /// A triggered ability's `limits` print the trailing "This ability
    /// triggers only once[ each turn]." rider ([CR#603.2h]) — the
    /// render-direction mirror of the migration parser's
    /// `triggered_ability::peel_trigger_limit`. Both the per-turn and the bare
    /// per-game forms round-trip.
    #[test]
    fn trigger_limit_rider_renders_per_turn_and_per_game() {
        use deckmaste_semantics::Action;
        use deckmaste_semantics::BeginningStep;
        use deckmaste_semantics::OneShotEffect;
        use deckmaste_semantics::PhaseStep;
        use deckmaste_semantics::UseLimit;
        use deckmaste_semantics::WhoseTurn;

        let event = EventFilter::StepBegins {
            at: PhaseStep::Beginning(BeginningStep::Upkeep),
            whose: WhoseTurn::Your,
        };
        let sacrifice_this = OneShotEffect::Act(Action::Sacrifice(Reference::You, Reference::This));
        let view = CardView {
            name: "Test Enchantment",
            mana_cost: None,
            supertypes: &[],
            types: &[Type::Enchantment.def()],
            subtypes: &[],
            power: None,
            toughness: None,
            abilities: &[],
        };
        let with_limit = |limit: UseLimit| TriggeredAbility {
            ability_word: None,
            event: event.clone(),
            from: None,
            condition: None,
            limits: vec![limit].into(),
            where_x: None,
            effect: sacrifice_this.clone(),
        };
        assert_eq!(
            triggered(&with_limit(UseLimit::OncePerTurn), &view),
            "At the beginning of your upkeep, sacrifice this enchantment. \
             This ability triggers only once each turn."
        );
        assert_eq!(
            triggered(&with_limit(UseLimit::OncePerGame), &view),
            "At the beginning of your upkeep, sacrifice this enchantment. \
             This ability triggers only once."
        );
    }

    /// `EventFilter::BlockDeclared` — the one block fact's four renderable
    /// views: the two bare single-slot forms ([CR#509.3a] blocks,
    /// [CR#509.3c] becomes blocked, one side `Any`) plus the two directional
    /// two-slot forms the `Blocking` event macro adds ([CR#509.3b] "blocks a
    /// creature", [CR#509.3d] "becomes blocked by a creature", both sides
    /// narrowed, told apart by which slot is `This`).
    #[test]
    fn block_declared_event_clause_renders_all_four_views() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let this = || Predicate::Ref(Reference::This);
        let creature = || Predicate::r#type(Type::Creature);

        // Bare self-blocks ([CR#509.3a]).
        assert_eq!(
            event_clause(
                &EventFilter::BlockDeclared {
                    by: this(),
                    of: Predicate::Any
                },
                &ctx
            ),
            ("Whenever", "Test blocks".to_string())
        );
        // Bare self-becomes-blocked ([CR#509.3c]).
        assert_eq!(
            event_clause(
                &EventFilter::BlockDeclared {
                    by: Predicate::Any,
                    of: this()
                },
                &ctx
            ),
            ("Whenever", "Test becomes blocked".to_string())
        );
        // Two-slot: self blocks a creature ([CR#509.3b]).
        assert_eq!(
            event_clause(
                &EventFilter::BlockDeclared {
                    by: this(),
                    of: creature()
                },
                &ctx
            ),
            ("Whenever", "Test blocks a creature".to_string())
        );
        // Two-slot: self becomes blocked by a creature ([CR#509.3d]).
        assert_eq!(
            event_clause(
                &EventFilter::BlockDeclared {
                    by: creature(),
                    of: this()
                },
                &ctx
            ),
            ("Whenever", "Test becomes blocked by a creature".to_string())
        );
    }

    /// `EventFilter::LifeGained` / `EventFilter::Drawn` — the `GainsLife` /
    /// `Draws` macros' expansions. Both events share the same two
    /// player-identity subject shapes a real card narrows to (2nd-person
    /// "you", 3rd-person-singular "an opponent"); always "Whenever" (a
    /// player is never `Ref(This)`, so `lead_for` doesn't apply here).
    #[test]
    fn gains_life_and_draws_event_clause_renders_subjects() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let opponent = || {
            Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(Predicate::Ref(
                Reference::You,
            ))))
        };

        assert_eq!(
            event_clause(
                &EventFilter::LifeGained {
                    who: Predicate::Ref(Reference::You),
                    amount: None,
                },
                &ctx
            ),
            ("Whenever", "you gain life".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::LifeGained {
                    who: opponent(),
                    amount: None,
                },
                &ctx
            ),
            ("Whenever", "an opponent gains life".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::Drawn {
                    who: Predicate::Ref(Reference::You),
                    amount: None,
                },
                &ctx
            ),
            ("Whenever", "you draw a card".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::Drawn {
                    who: opponent(),
                    amount: None,
                },
                &ctx
            ),
            ("Whenever", "an opponent draws a card".to_string())
        );
    }

    /// `EventFilter::StateBecame { becomes: Untapped, .. }` — the
    /// `BecomesUntapped`/`ThisBecomesUntapped` macros' expansion. The
    /// `Tapped` half of this render arm is already exercised by Goblin
    /// Medics' hand-authored card; this pins the new `Untapped` state word.
    #[test]
    fn state_became_renders_untapped() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        assert_eq!(
            event_clause(
                &EventFilter::StateBecame {
                    of: Predicate::Ref(Reference::This),
                    becomes: StateChange::Untapped,
                    cause: None,
                },
                &ctx
            ),
            ("Whenever", "Test becomes untapped".to_string())
        );
    }

    /// `EventFilter::Cast { who: Ref(You), .. }` ([CR#601.2i]) — the cast
    /// trigger family the migrations parser's `parse_cast_event` mints.
    /// Round-trips every new `what:` shape (plus the retained self and
    /// single-subtype forms) back to its oracle phrase.
    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "exhaustive per-form round-trip cases for the cast-event clause family; data-heavy by nature"
    )]
    fn cast_event_clause_renders_forms() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let event = |what| EventFilter::Cast {
            who: Predicate::Ref(Reference::You),
            what,
        };

        // Self — Cascade's own "you cast this spell" reminder-text shape;
        // leads "When" like an enters/dies self trigger.
        assert_eq!(
            event_clause(&event(Predicate::Ref(Reference::This)), &ctx),
            ("When", "you cast this spell".to_string())
        );
        // Bare spell.
        assert_eq!(
            event_clause(&event(Predicate::Kind(ObjectKind::Spell)), &ctx),
            ("Whenever", "you cast a spell".to_string())
        );
        // A single card-type filter.
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::r#type(Type::Creature),
                    ]
                    .into()
                )),
                &ctx
            ),
            ("Whenever", "you cast a creature spell".to_string())
        );
        // The instant-or-sorcery disjunction.
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::Or(
                            vec![
                                Predicate::r#type(Type::Instant),
                                Predicate::r#type(Type::Sorcery),
                            ]
                            .into()
                        ),
                    ]
                    .into()
                )),
                &ctx
            ),
            (
                "Whenever",
                "you cast an instant or sorcery spell".to_string()
            )
        );
        // The noncreature negation.
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::Not(Arc::new(Predicate::r#type(Type::Creature))),
                    ]
                    .into()
                )),
                &ctx
            ),
            ("Whenever", "you cast a noncreature spell".to_string())
        );
        // The retained single-subtype form ("an Elf spell").
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::Characteristic(CharacteristicPredicate::Subtype(
                            deckmaste_semantics::SubtypeRef::named(
                                deckmaste_semantics::Ident::from("Elf")
                            )
                        )),
                    ]
                    .into()
                )),
                &ctx
            ),
            ("Whenever", "you cast an Elf spell".to_string())
        );
        // A catalog-subtype disjunction ("a Spirit or Arcane spell") — the
        // subtype-only reading `subtype_disjunction_noun` tries once the
        // type-disjunction reading misses.
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::Or(
                            vec![
                                Predicate::Characteristic(CharacteristicPredicate::Subtype(
                                    deckmaste_semantics::SubtypeRef::named(
                                        deckmaste_semantics::Ident::from("Spirit")
                                    )
                                )),
                                Predicate::Characteristic(CharacteristicPredicate::Subtype(
                                    deckmaste_semantics::SubtypeRef::named(
                                        deckmaste_semantics::Ident::from("Arcane")
                                    )
                                )),
                            ]
                            .into()
                        ),
                    ]
                    .into()
                )),
                &ctx
            ),
            ("Whenever", "you cast a Spirit or Arcane spell".to_string())
        );
        // A mana-value threshold postfix ([CR#202.3]) — no head atom, so the
        // base noun defaults to "a spell".
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::Characteristic(CharacteristicPredicate::Stat(
                            Stat::ManaValue,
                            Cmp::AtLeast,
                            Count::Literal(4)
                        )),
                    ]
                    .into()
                )),
                &ctx
            ),
            (
                "Whenever",
                "you cast a spell with mana value 4 or greater".to_string()
            )
        );
        // A head atom (card type) combined with a postfix atom (mana-value
        // threshold) — "a creature spell with mana value 3 or less".
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::r#type(Type::Creature),
                        Predicate::Characteristic(CharacteristicPredicate::Stat(
                            Stat::ManaValue,
                            Cmp::AtMost,
                            Count::Literal(3)
                        )),
                    ]
                    .into()
                )),
                &ctx
            ),
            (
                "Whenever",
                "you cast a creature spell with mana value 3 or less".to_string()
            )
        );
        // Heroic's head ([CR#115.9b]): "a spell that targets ~".
        assert_eq!(
            event_clause(
                &event(Predicate::And(
                    vec![
                        Predicate::Kind(ObjectKind::Spell),
                        Predicate::State(StatePredicate::Targets(Arc::new(Predicate::Ref(
                            Reference::This
                        )))),
                    ]
                    .into()
                )),
                &ctx
            ),
            ("Whenever", "you cast a spell that targets ~".to_string())
        );
    }

    /// `EventFilter::Cast`'s `who:` narrowing — the three subjects real
    /// oracle text uses ([`cast_who_phrase`]): "you cast" (`Ref(You)`), "a
    /// player casts" (`Player`), "an opponent casts" (`OpponentOf(Ref(You))`).
    #[test]
    fn cast_who_renders_you_player_and_opponent() {
        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let what = || {
            Predicate::And(
                vec![
                    Predicate::Kind(ObjectKind::Spell),
                    Predicate::r#type(Type::Creature),
                ]
                .into(),
            )
        };
        assert_eq!(
            event_clause(
                &EventFilter::Cast {
                    who: Predicate::Ref(Reference::You),
                    what: what(),
                },
                &ctx
            ),
            ("Whenever", "you cast a creature spell".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::Cast {
                    who: Predicate::Kind(ObjectKind::Player),
                    what: what(),
                },
                &ctx
            ),
            ("Whenever", "a player casts a creature spell".to_string())
        );
        assert_eq!(
            event_clause(
                &EventFilter::Cast {
                    who: Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(
                        Predicate::Ref(Reference::You)
                    ))),
                    what: what(),
                },
                &ctx
            ),
            ("Whenever", "an opponent casts a creature spell".to_string())
        );
    }

    /// The plain "gets +N/+M for each `<selection>`" pump family
    /// ([CR#107.3] "for each") — the structural render arm for the RAW
    /// `CountOf(Objects)` shape ([`for_each_pump_clause`]), since render
    /// flattens a change-bundling macro through `Expanded` before
    /// `modifications_predicate`'s `pt_delta_clause` piece (which matches
    /// only `Count::Literal`) ever runs. Covers the symmetric `Several`
    /// shape (Blanchwood Armor/Primal Bellow/Might of the Masses), the
    /// power-only lone-`Power` shape (no `Toughness` node — the
    /// `P1P0ForEach`/`P2P0ForEach` macro body shape, Goblin Piledriver's OWN
    /// stored shape), the `Times(Literal(n), ..)` coefficient shape, and the
    /// asymmetric `Several` shape whose toughness axis is a flat
    /// `Toughness(Up(Literal(0)))` — a distinct, hand-built stored shape (no
    /// card in the corpus currently uses it) that also reads "+n/+0",
    /// exercised below via a synthetic case built on Goblin Piledriver's own
    /// predicate/wording.
    #[test]
    fn for_each_pump_renders_symmetric_power_only_and_coefficient() {
        use deckmaste_semantics::Countable;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
            named: None,
        };
        let pred = Predicate::And(
            vec![
                Predicate::r#type(Type::Creature),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        let count = Count::CountOf(Countable::Objects(Arc::new(pred.clone())));

        let symmetric = StaticEffect::Modify(
            Reference::This,
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(count.clone())),
                    Modification::Toughness(NumericOp::Up(count.clone())),
                ]
                .into(),
            ),
        );
        assert_eq!(
            static_effect(&symmetric, &ctx).as_deref(),
            Some("Test gets +1/+1 for each creature you control.")
        );

        let power_only =
            StaticEffect::Modify(Reference::This, Modification::Power(NumericOp::Up(count)));
        assert_eq!(
            static_effect(&power_only, &ctx).as_deref(),
            Some("Test gets +1/+0 for each creature you control.")
        );

        let coeff_count = Count::Times(
            Arc::new(Count::Literal(2)),
            Arc::new(Count::CountOf(Countable::Objects(Arc::new(pred)))),
        );
        let coeff = StaticEffect::Modify(
            Reference::This,
            Modification::Power(NumericOp::Up(coeff_count.clone())),
        );
        assert_eq!(
            static_effect(&coeff, &ctx).as_deref(),
            Some("Test gets +2/+0 for each creature you control.")
        );

        // Goblin Piledriver's REAL stored shape (plugins/wizards/cards/
        // "Goblin Piledriver.ron") IS the lone-`Power` `P2P0ForEach`
        // invocation covered above: `Modify(This, P2P0ForEach(And([
        // Permanent, Subtype("Goblin"), Not(Ref(This)), Attacking])))`. This
        // asymmetric `Several([Power(Up(Times(Literal(2),
        // CountOf(Objects(pred))))), Toughness(Up(Literal(0)))])` is a
        // different, hand-built shape — not what P2P0ForEach expands to —
        // built here only to exercise the render arm's raw-`Several`
        // asymmetric branch directly, reusing Goblin Piledriver's own
        // predicate and wording.
        let goblin_pred = Predicate::And(
            vec![
                Predicate::Characteristic(CharacteristicPredicate::Subtype(
                    deckmaste_semantics::SubtypeRef::named("Goblin".into()),
                )),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::State(StatePredicate::Attacking),
            ]
            .into(),
        );
        let goblin_count = Count::Times(
            Arc::new(Count::Literal(2)),
            Arc::new(Count::CountOf(Countable::Objects(Arc::new(goblin_pred)))),
        );
        let goblin_piledriver = StaticEffect::Modify(
            Reference::This,
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(goblin_count)),
                    Modification::Toughness(NumericOp::Up(Count::Literal(0))),
                ]
                .into(),
            ),
        );
        assert_eq!(
            static_effect(&goblin_piledriver, &ctx).as_deref(),
            Some("Test gets +2/+0 for each other attacking Goblin.")
        );
    }
}
