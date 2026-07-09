//! Framing for triggered/activated/static abilities (the clause around the
//! effect).

use deckmaste_core::Ability;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::CollectionOp;
use deckmaste_core::Color;
use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::EventFilter;
use deckmaste_core::IgnoreRule;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::PayAct;
use deckmaste_core::PlayerAttr;
use deckmaste_core::PlayerMod;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::StateChange;
use deckmaste_core::StatePredicate;
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
    // Targeting lives on an `OneShotEffect::Targeted` wrapper, which the effect
    // walk rebinds `ctx.targets` from ([CR#115.1]).
    let ctx = Ctx {
        subject: view.name,
        targets: &[],
        that: None,
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
    let body_subject = if matches!(t.from, Some(z) if z != Zone::Battlefield) {
        "this card".to_string()
    } else if matches!(
        t.event,
        EventFilter::CoinFlipped { .. }
            | EventFilter::DiceRolled { .. }
            | EventFilter::RollPlanarDie { .. }
            | EventFilter::TapForMana { .. }
    ) {
        self_type_phrase(view)
    } else {
        "it".to_string()
    };
    let body_ctx = Ctx {
        subject: &body_subject,
        targets: &[],
        that: None,
    };
    let body = lower_first(&effect::effect(&t.effect, &body_ctx));

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
        return format!("{lead} {clause}, if {cond_clause}, {body}");
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
            };
            format!("if {}, ", super::condition::condition(c, &cond_ctx))
        }
        None => String::new(),
    };
    let trig = format!("{lead} {clause}, {cond}{body}");
    from_zone_qualified(t.from, view.name, trig)
}

/// "this enchantment" / "this creature" / … — a card's own type-noun
/// self-reference, first printed type. Falls back to the plain anaphor
/// "it" for a (structurally impossible, but never-crash) typeless card.
fn self_type_phrase(view: &CardView) -> String {
    view.types.first().map_or_else(
        || "it".to_string(),
        |t| format!("this {}", super::card::type_str(*t).to_lowercase()),
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
    from: Option<deckmaste_core::Zone>,
) -> Option<String> {
    let zone = from.filter(|&z| z != Zone::Battlefield)?;
    let Condition::Exists(Predicate::And(parts)) = cond else {
        return None;
    };
    let [a, b] = parts.as_slice() else { return None };
    let ((Predicate::Adjacent(dir, Reference::This), other)
    | (other, Predicate::Adjacent(dir, Reference::This))) = (a, b)
    else {
        return None;
    };
    let dir_word = match dir {
        deckmaste_core::Adjacency::Above => "above",
        deckmaste_core::Adjacency::Below => "below",
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
pub(super) fn activated(a: &deckmaste_core::ActivatedAbility, view: &CardView) -> String {
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
    };
    let cost = effect::activated_cost(&a.cost.0, &ctx);
    let body = effect::effect(&a.effect, &ctx);
    from_zone_qualified(a.from, view.name, format!("{cost}: {body}"))
}

/// Whether an activated ability's effect (through its `Targeted` wrapper, if
/// any) is the "exchange control" `Simultaneously` shape — see
/// [`activated`]'s doc comment for why this drives the `Ctx.subject` choice.
fn effect_wants_self_type_phrase(e: &deckmaste_core::OneShotEffect) -> bool {
    use deckmaste_core::OneShotEffect;
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
        lower_first(&text)
    )
}

/// Returns (lead word, the event clause).
/// "When", "Baleful Strix enters" | "Whenever", "Goblin Medics becomes tapped".
pub(super) fn event_clause(e: &EventFilter, ctx: &Ctx) -> (&'static str, String) {
    match e {
        EventFilter::Expanded(exp) => event_clause(&exp.value, ctx),
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
        EventFilter::StateBecame { of, becomes } => (
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
        other => ("When", format!("[unrendered: {other:?}]")),
    }
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
fn whose_word(w: deckmaste_core::WhoseTurn) -> &'static str {
    use deckmaste_core::WhoseTurn;
    match w {
        WhoseTurn::Your => "your",
        WhoseTurn::EachPlayers => "each player's",
        WhoseTurn::AnOpponents => "an opponent's",
    }
}

/// The step/phase noun a [`PhaseStep`](deckmaste_core::PhaseStep) prints
/// after its possessive owner ("your **upkeep**"). `None` for a phase/step
/// this corpus hasn't needed phrasing for yet (a combat sub-step) — the
/// caller falls back to the generic unrendered marker rather than guessing.
fn step_noun(at: deckmaste_core::PhaseStep) -> Option<&'static str> {
    use deckmaste_core::BeginningStep;
    use deckmaste_core::EndingStep;
    use deckmaste_core::PhaseStep;
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

/// A subject filter as a noun ("Baleful Strix" for the self filter,
/// "a creature" for a Creature macro filter).
fn subject_of(f: &Predicate, ctx: &Ctx) -> String {
    let f = super::fragment::strip_expanded(f);
    if f.is_this() {
        return ctx.subject.to_string();
    }
    if let Some(t) = super::fragment::find_card_type(f) {
        return format!("a {}", super::card::type_str(t).to_lowercase());
    }
    format!("[unrendered: {f:?}]")
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
        StaticEffect::ModifyPlayer(who, m) => Some(modify_player(who, m)),
        StaticEffect::TriggerMultiplier {
            cause,
            extra,
            affected,
        } => Some(trigger_multiplier(cause, extra, affected)),
        StaticEffect::Deontic(d) => Some(super::deontic::deontic(d, ctx.subject)),
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
        other => Some(format!("[unrendered: {other:?}].")),
    }
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
            if super::fragment::find_card_type(filter) == Some(Type::Artifact) =>
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
    use deckmaste_core::Stat;

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
/// [`Countable::Singleton`](deckmaste_core::Countable::Singleton) sum, whose
/// per-object-axis-enumeration idiom ("... it has") reads nothing like the
/// group-fold `CountDistinct(_, Objects(..))` phrasing (Domain/Coven's "the
/// number of X among Y"). The recognized shape is `Several([Power(Up(c)),
/// Toughness(Up(c))])` with the SAME `c = Plus(Plus(CountDistinct(Supertypes,
/// Singleton(r)), CountDistinct(Types, Singleton(r))), CountDistinct(
/// Subtypes, Singleton(r)))` for both — the +1/+1-per-unit case (a
/// coefficient-`k` "+k/+k for each ..." generalization is unforced until a
/// real card needs it). `None` for any other shape.
fn axis_sum_pump_clause(r: &Reference, change: &Modification, ctx: &Ctx) -> Option<String> {
    use deckmaste_core::Characteristic;
    use deckmaste_core::Countable;
    use deckmaste_core::Expand;

    let normalized = change.clone().expand_all();
    let Modification::Several(parts) = &normalized else {
        return None;
    };
    let [
        Modification::Power(NumericOp::Up(power_delta)),
        Modification::Toughness(NumericOp::Up(toughness_delta)),
    ] = parts.as_slice()
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
    let changes = Modification::flatten(changes.to_vec());
    let changes = changes.as_slice();

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
        to: Some(deckmaste_core::Zone::Battlefield),
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
    use deckmaste_core::RelationPredicate;
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
            vec![super::card::type_str(*t).to_lowercase()]
        }
        Predicate::Or(items) => items
            .iter()
            .filter_map(|f| match f {
                Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
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
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::PayAct;
        use deckmaste_core::PipClass;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;

        let ctx = Ctx {
            subject: "Test",
            targets: &[],
            that: None,
        };
        let you = || {
            Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                Reference::You,
            ))))
        };
        let ty = |t| Predicate::Characteristic(CharacteristicPredicate::Type(t));

        let convoke = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Predicate::And(vec![ty(Type::Creature), you()])),
        );
        assert_eq!(static_effect(&convoke, &ctx).as_deref(), Some("Convoke"));

        let improvise = StaticEffect::PayPips(
            PipClass::Generic,
            PayAct::TapToPay(Predicate::And(vec![ty(Type::Artifact), you()])),
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
}
