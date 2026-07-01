//! Effects / actions render to imperative sentences (spell mood).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::CounterSpec;
use deckmaste_core::Destination;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::StatValue;
use deckmaste_core::Token;
use deckmaste_core::TokenSpec;
use deckmaste_core::TurnMarker;
use deckmaste_core::Zone;

use super::Ctx;
use super::fragment;

/// MTG card text uses second-person verb agreement only for the pronoun "you";
/// every other payer ("that player", "its controller", a player's name) takes
/// the third-person `-s` form. Returns the `(does, doesn't, pays)` verb forms
/// agreeing with the ALREADY-RENDERED `payer` so a `MayPay`/`MustPay` clause
/// reads "you may pay … if you **do**, …" rather than "if you **does**, …".
fn payer_verbs(payer: &str) -> (&'static str, &'static str, &'static str) {
    if payer.eq_ignore_ascii_case("you") {
        ("do", "don't", "pay")
    } else {
        ("does", "doesn't", "pays")
    }
}

/// Render an `Effect` as one or more sentences joined into a single rules
/// string.
pub(super) fn effect(e: &Effect, ctx: &Ctx) -> String {
    match e {
        Effect::Act(a) => action(a, ctx),
        Effect::Sequence(parts) => {
            let mut out = String::new();
            for (i, p) in parts.iter().enumerate() {
                let s = trim_period(&effect(p, ctx));
                if i == 0 {
                    out.push_str(&s);
                } else {
                    out.push_str(", then ");
                    out.push_str(&super::ability::lower_first(&s));
                }
            }
            out.push('.');
            out
        }
        Effect::Expanded(e) => match super::template::expanded(e, ctx.subject) {
            Some(s) => ensure_period(&s),
            None => effect(&e.value, ctx),
        },
        Effect::Continuously(c) => {
            let clause = super::ability::static_effect(&c.effect, ctx).map_or_else(
                || format!("[unrendered: {:?}]", c.effect),
                |s| trim_period(&s),
            );
            match duration_suffix(&c.duration) {
                Some(d) => format!("{clause} {d}."),
                None => format!("{clause}."),
            }
        }
        // A target-scoping wrapper ([CR#115.1,601.2c]): render the inner effect
        // with `ctx.targets` rebound to this node's targets, so the inner
        // `Reference::Target(n)` resolves to "target creature" etc.
        Effect::Targeted(t) => effect(
            &t.effect,
            &Ctx {
                subject: ctx.subject,
                targets: &t.targets,
                that: ctx.that,
            },
        ),
        // [CR#118.12a]: "[or_else] unless [actor] pays [cost]" — the resolution-
        // time punisher (Mana Leak). Starts with the rendered punisher effect
        // (already capitalized). Declines structurally if the cost has no symbol
        // rendering (e.g. a `Do(...)` verb cost).
        Effect::MustPay(m) => {
            let payer = fragment::reference(&m.actor, ctx);
            let (_, _, pays) = payer_verbs(&payer);
            match super::template::render_cost(&m.cost.0) {
                Some(c) => format!(
                    "{} unless {payer} {pays} {c}.",
                    trim_period(&effect(&m.or_else, ctx))
                ),
                None => format!("[unrendered: {m:?}]."),
            }
        }
        // [CR#603,608]: "[actor] may pay [cost]. If [actor] does, [and_then];
        // if [actor] doesn't, [or_else]" — a resolution-time kicker.
        Effect::MayPay(m) => {
            let payer = fragment::reference(&m.actor, ctx);
            let (does, doesnt, _) = payer_verbs(&payer);
            match super::template::render_cost(&m.cost.0) {
                Some(c) => {
                    let did = super::ability::lower_first(&trim_period(&effect(&m.and_then, ctx)));
                    let tail = m.or_else.as_ref().map_or_else(String::new, |or_else| {
                        let didnt =
                            super::ability::lower_first(&trim_period(&effect(or_else, ctx)));
                        format!("; if {payer} {doesnt}, {didnt}")
                    });
                    fragment::capitalize(&format!(
                        "{payer} may pay {c}. If {payer} {does}, {did}{tail}."
                    ))
                }
                None => format!("[unrendered: {m:?}]."),
            }
        }
        // [CR#601.2f,118.8]: "As an additional cost, [pay]. [body]" — the
        // printed/nested additional-cost clause whose body reads the paid object
        // (via the event references). The payment renders as a symbol cost
        // ("pay {2}") or an object-moving verb phrase ("sacrifice a creature");
        // declines structurally if the cost has no clean rendering.
        Effect::AdditionalCost(ac) => match additional_payment(&ac.pay.0, ctx) {
            Some(pay) => format!("As an additional cost, {pay}. {}", effect(&ac.body, ctx)),
            None => format!("[unrendered: {ac:?}]."),
        },
        // [CR#601.2d]: a divided distribution — the body picks the verb
        // ("deal … damage" vs "distribute … counters"), the amount is divided
        // "as you choose" among the group.
        Effect::DivideAmong(d) => divide_among(d, ctx),
        // [CR#601.2b]: a choose-then-act binder. The binder's noun phrase
        // ("a creature", "two cards") is bound as the body's `That`/`Those`
        // anaphor, so `With(ChooseOne(Creature), Sacrifice(That))` renders
        // "Sacrifice a creature."
        Effect::With(w) => {
            let phrase = binder_phrase(&w.binder, ctx);
            effect(&w.body, &ctx.with_that(&phrase))
        }
        // [CR#608]: iterate a many-binder, the body reading each element as
        // `It`. A single group-verb body collapses to the natural collective
        // sentence — "Deal 2 damage to each creature." (Pyroclasm), "Destroy
        // each creature." (a wrath), "Put two cards … on top of your library."
        // (Brainstorm's group-move) — rather than the distributive "For each
        // creature, …"; a body the collapse does not recognise falls back to
        // that per-element form ([CR#608.2]). This is the renderer half of the
        // `core-many-binder-group-move` seam.
        Effect::Each(fe) => {
            if let Effect::Act(act) = &*fe.effect
                && let Some(collective) = each_collective(act, &fe.binder, ctx)
            {
                return collective;
            }
            format!(
                "For each {}, {}.",
                binder_group_noun(&fe.binder, ctx),
                super::ability::lower_first(&trim_period(&effect(
                    &fe.effect,
                    &ctx.with_that("it"),
                )))
            )
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// The noun phrase a [`Binder`](deckmaste_core::Binder) contributes to its
/// `Effect::With` / `CostComponent::With` body — read by the body's `That` /
/// `Those` anaphor ([CR#601.2b]). A one-binder yields "a creature"; a
/// many-binder yields "two cards"; the reference/existing forms defer to the
/// shared fragment renderers.
fn binder_phrase(binder: &deckmaste_core::Binder, ctx: &Ctx) -> String {
    use deckmaste_core::Binder;
    match binder {
        Binder::ChooseOne(f) => a_an(&fragment::filter_noun(f)),
        Binder::Choose(q, f) => {
            format!("{} {}", fragment::quantity(q), fragment::filter_object(f))
        }
        Binder::TheRef(r) => fragment::reference(r, ctx),
        Binder::Existing(sel) => fragment::selection(sel, ctx),
        // The producer/search binders have no corpus card yet (their engine
        // resolution is a seam); these arms render the bound object's noun the
        // body's `That` reads — the found card (`SearchOne`/`Search`) or the
        // produced object (`Produce`) — so the enum stays exhaustive without
        // fabricating the search/produce verb text.
        Binder::SearchOne { filter, .. } => a_an(&fragment::filter_noun(filter)),
        Binder::Search {
            quantity, filter, ..
        } => {
            format!(
                "{} {}",
                fragment::quantity(quantity),
                fragment::filter_object(filter)
            )
        }
        Binder::Produce(_) => "the produced object".to_string(),
        Binder::Expanded(e) => binder_phrase(&e.value, ctx),
    }
}

fn a_an(noun: &str) -> String {
    let lowercase = noun.to_lowercase();
    let is_vowel = |c: char| "aeiou".contains(c);
    if let Some(first_char) = lowercase.chars().next() {
        if is_vowel(first_char) {
            return format!("an {noun}");
        }
    }
    format!("a {noun}")
}

/// The collective rendering of an [`Effect::Each`] whose body is a single group
/// verb acting on the per-element [`Reference::It`] — the natural
/// "<verb> each <group>" / "put <group> on <dest>" surface ([CR#608]), the
/// renderer half the `core-many-binder-group-move` seam calls for. Returns
/// `None` for any body the collapse does not recognise, so the caller falls
/// back to the per-element "For each <group>, …" form.
fn each_collective(act: &Action, binder: &deckmaste_core::Binder, ctx: &Ctx) -> Option<String> {
    // "each <bare group noun>" — the recipient/patient of a set-wide verb.
    let each_group = || format!("each {}", binder_group_noun(binder, ctx));
    match act {
        // "Deal N damage to each <group>." — the implicit-source form; a named
        // source reads "<source> deals N damage to each <group>."
        Action::DealDamage(Reference::It, amount, source) => Some(match source {
            Reference::This => {
                format!(
                    "Deal {} damage to {}.",
                    fragment::count(amount),
                    each_group()
                )
            }
            _ => format!(
                "{} deals {} damage to {}.",
                capitalize_first(&fragment::reference(source, ctx)),
                fragment::count(amount),
                each_group(),
            ),
        }),
        // "Destroy each <group>." ([CR#701.8a]).
        Action::Destroy(Reference::It) => Some(format!("Destroy {}.", each_group())),
        // A group move to the library reads "Put <group> on top/the bottom of
        // your library." — Brainstorm's "put two cards … on top": the chosen
        // group's own phrase, not "each" ([CR#401.7]).
        Action::Move(Reference::It, Destination::Library(anchor)) => Some(format!(
            "Put {} on {} of your library.",
            binder_phrase(binder, ctx),
            fragment::library_position(anchor),
        )),
        // Set-wide tap/untap ([CR#701.26a,701.26b]): "Tap each <group>."
        Action::By(_, PlayerAction::Tap(Reference::It)) => Some(format!("Tap {}.", each_group())),
        Action::By(_, PlayerAction::Untap(Reference::It)) => {
            Some(format!("Untap {}.", each_group()))
        }
        _ => None,
    }
}

/// The bare collective noun a [`Binder`](deckmaste_core::Binder) contributes to
/// an [`Effect::Each`] "each <noun>" / "For each <noun>" construction: the
/// whole matching set yields the bare noun ("creature", so the surrounding
/// "each" supplies the quantifier — not "each each creature"), a
/// bound/announced group its plural anaphor ("them"), and a chosen group its
/// full phrase.
fn binder_group_noun(binder: &deckmaste_core::Binder, ctx: &Ctx) -> String {
    use deckmaste_core::Binder;
    use deckmaste_core::Selection;
    match binder {
        Binder::Existing(Selection::Filter(f)) => fragment::filter_noun(f),
        Binder::Existing(sel) => fragment::selection(sel, ctx),
        Binder::Expanded(e) => binder_group_noun(&e.value, ctx),
        other => binder_phrase(other, ctx),
    }
}

fn duration_suffix(d: &Duration) -> Option<String> {
    match d {
        Duration::FixedUntil(m) => Some(format!("until {}", turn_marker(*m))),
        Duration::EndOfGame => None,
        other => Some(format!("[unrendered: {other:?}]")),
    }
}

fn turn_marker(m: TurnMarker) -> &'static str {
    match m {
        TurnMarker::EndOfTurn => "end of turn",
        TurnMarker::EndOfCombat => "end of combat",
        TurnMarker::YourNextTurn => "your next turn",
    }
}

fn action(a: &Action, ctx: &Ctx) -> String {
    match a {
        // Default source (`This`): the implicit "deal N damage to X". An
        // explicit non-`This` source names the dealer — "<source> deals N
        // damage to <target>" (the fight / redirected-damage surface).
        Action::DealDamage(target, amount, Reference::This) => format!(
            "Deal {} damage to {}.",
            fragment::count(amount),
            fragment::reference(target, ctx)
        ),
        Action::DealDamage(target, amount, source) => format!(
            "{} deals {} damage to {}.",
            capitalize_first(&fragment::reference(source, ctx)),
            fragment::count(amount),
            fragment::reference(target, ctx)
        ),
        Action::Destroy(r) => format!("Destroy {}.", fragment::reference(r, ctx)),
        // [CR#701.6a]: counter a spell or ability on the stack — "Counter
        // target spell" (Mana Leak's punisher branch).
        Action::Counter(r) => format!("Counter {}.", fragment::reference(r, ctx)),
        // [CR#122]: move counters between two objects. `AllKinds` -> "all
        // counters"; a named kind -> "<n> <kind> counter(s)".
        Action::MoveCounters(spec, from, to) => {
            let from_p = fragment::reference(from, ctx);
            let to_p = fragment::reference(to, ctx);
            match spec {
                CounterSpec::AllKinds => {
                    format!("Move all counters from {from_p} onto {to_p}.")
                }
                CounterSpec::Named(kind, count) => {
                    let plural = if count.literal_value() == Some(1) { "" } else { "s" };
                    format!(
                        "Move {} {} counter{plural} from {from_p} onto {to_p}.",
                        fragment::count(count),
                        kind.as_str(),
                    )
                }
            }
        }
        // [CR#401.7]: a library destination — "Put <cards> on top/the bottom of
        // your library." (the former `PutInLibrary`, now a `Move` destination).
        Action::Move(r, Destination::Library(anchor)) => format!(
            "Put {} on {} of your library.",
            fragment::reference(r, ctx),
            fragment::library_position(anchor),
        ),
        Action::By(_who, pa) => player_action(pa, ctx),
        // [CR#701.19a]: a regeneration shield — rendered as "Regenerate <target>."
        // when the replacement body has the standard structure. The top-level
        // `Regenerate` keyword macro emits this via its template.
        Action::CreateReplacement { subject, .. } => {
            format!("Regenerate {}.", fragment::reference(subject, ctx))
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// Render a divided distribution ([CR#601.2d]). The body selects the verb;
/// `group` is rendered as the set it divides among.
fn divide_among(d: &deckmaste_core::DivideAmong, ctx: &Ctx) -> String {
    let amount = fragment::count(&d.amount);
    let group = binder_phrase(&d.binder, ctx);
    match &*d.body {
        Effect::Act(Action::DealDamage(..)) => {
            format!("Deal {amount} damage divided as you choose among {group}.")
        }
        Effect::Act(Action::By(_, PlayerAction::PutCounters(_, kind, _))) => {
            format!(
                "Distribute {amount} {} counters among {group}.",
                kind.as_str()
            )
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// The payment clause of an [`Effect::AdditionalCost`] ([CR#601.2f]): an
/// all-symbol cost reads "pay {cost}"; an object-moving verb cost reads as its
/// lowercased verb phrase ("sacrifice a creature"). Declines (`None`) on an
/// empty or no-clean-rendering cost, so the effect falls back to the structural
/// form.
fn additional_payment(cost: &[deckmaste_core::CostComponent], ctx: &Ctx) -> Option<String> {
    use deckmaste_core::CostComponent;
    if cost.is_empty() {
        return None;
    }
    // An all-symbol cost ({2}, {T}) reads "pay {cost}".
    if let Some(symbols) = super::template::render_cost(cost) {
        return Some(format!("pay {symbols}"));
    }
    // Otherwise render each component as its lowercased verb clause.
    let mut parts = Vec::new();
    for component in cost {
        match component {
            CostComponent::Do(pa) => {
                let phrase = trim_period(&player_action(pa, ctx));
                parts.push(super::ability::lower_first(&phrase));
            }
            // A cost-side choose-then-pay step ([CR#601.2b]): bind the binder's
            // noun phrase as the body verbs' `That`/`Those` anaphor, then render
            // the body's `Do` verbs — "sacrifice a creature".
            CostComponent::With { binder, body } => {
                let phrase = binder_phrase(binder, ctx);
                let inner = ctx.with_that(&phrase);
                for inner_comp in body.iter() {
                    match inner_comp {
                        CostComponent::Do(pa) => {
                            let p = trim_period(&player_action(pa, &inner));
                            parts.push(super::ability::lower_first(&p));
                        }
                        _ => return None,
                    }
                }
            }
            _ => return None,
        }
    }
    Some(parts.join(" and "))
}

fn player_action(pa: &PlayerAction, ctx: &Ctx) -> String {
    match pa {
        PlayerAction::Draw(Count::Literal(1)) => "Draw a card.".to_string(),
        PlayerAction::Draw(c) => format!("Draw {} cards.", fragment::count(c)),
        PlayerAction::GainLife(c) => format!("Gain {} life.", fragment::count(c)),
        PlayerAction::LoseLife(c) => format!("Lose {} life.", fragment::count(c)),
        PlayerAction::Create(count, spec) => create_text(count, spec),
        PlayerAction::Tap(r) => format!("Tap {}.", fragment::reference(r, ctx)),
        PlayerAction::Untap(r) => format!("Untap {}.", fragment::reference(r, ctx)),
        // A sacrifice ([CR#701.21]) — the patient is a single reference. A
        // chosen permanent ("sacrifice a creature", Fling) arrives pre-bound as
        // `Reference::That` from an enclosing `With`, which supplies the phrase.
        PlayerAction::Sacrifice(r) => format!("Sacrifice {}.", fragment::reference(r, ctx)),
        // Discard ([CR#701.9]): a named card via `what` (e.g. the `With`-bound
        // anaphor), else `count` cards chosen from hand.
        PlayerAction::Discard { what: Some(r), .. } => {
            format!("Discard {}.", fragment::reference(r, ctx))
        }
        PlayerAction::Discard {
            count: Count::Literal(1),
            what: None,
        } => "Discard a card.".to_string(),
        PlayerAction::Discard { count, what: None } => {
            format!("Discard {} cards.", fragment::count(count))
        }
        // A player-performed relocation ([CR#400.7]). Exiling is a pure zone
        // move ([CR#701.13]) — "Exile X."; a library destination mirrors
        // `Action::Move`'s "Put X on top/the bottom of your library."
        PlayerAction::Move(r, Destination::Zone(Zone::Exile)) => {
            format!("Exile {}.", fragment::reference(r, ctx))
        }
        PlayerAction::Move(r, Destination::Library(anchor)) => format!(
            "Put {} on {} of your library.",
            fragment::reference(r, ctx),
            fragment::library_position(anchor),
        ),
        PlayerAction::GetDesignation(name) if name.as_ref() == "CitysBlessing" => {
            "You get the city's blessing.".to_string()
        }
        PlayerAction::GetDesignation(name) => format!("You get {name}."),
        // [CR#701.19a]: remove all damage as part of regeneration.
        PlayerAction::RemoveDamage(r) => {
            format!("Remove all damage from {}.", fragment::reference(r, ctx))
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

// ── Token creation
// ────────────────────────────────────────────────────────────

fn create_text(count: &Count, spec: &TokenSpec) -> String {
    match spec {
        TokenSpec::Token(t) => {
            let plural = count.literal_value() != Some(1);
            let count_word = token_count_word(count);
            let descriptor = token_descriptor(t);
            let noun = if plural { "tokens" } else { "token" };
            let abilities_suffix = token_abilities_suffix(&t.abilities);
            format!("Create {count_word} {descriptor} {noun}{abilities_suffix}.")
        }
        // A predefined token ([CR#111.10]) renders by its bare name —
        // "Create a Treasure token." — the bidirectional truth the parser's
        // `create a <Name> token` production routes back to.
        TokenSpec::Named(name) => {
            let plural = count.literal_value() != Some(1);
            let count_word = token_count_word(count);
            let noun = if plural { "tokens" } else { "token" };
            format!("Create {count_word} {} {noun}.", name.as_str())
        }
    }
}

fn token_count_word(count: &Count) -> &'static str {
    match count {
        Count::Literal(1) => "a",
        Count::Literal(2) => "two",
        Count::Literal(3) => "three",
        Count::Literal(4) => "four",
        Count::Literal(5) => "five",
        Count::Literal(6) => "six",
        Count::Literal(7) => "seven",
        Count::Literal(8) => "eight",
        Count::Literal(9) => "nine",
        Count::X => "X",
        _ => "some",
    }
}

fn token_descriptor(t: &Token) -> String {
    let mut parts: Vec<String> = Vec::new();

    // P/T
    if let (Some(p), Some(toughness)) = (&t.power, &t.toughness) {
        let ps = stat_value_str(p);
        let ts = stat_value_str(toughness);
        parts.push(format!("{ps}/{ts}"));
    }

    // Colors
    for color in &t.color_indicator {
        parts.push(color_word(*color).to_string());
    }

    // Supertypes
    for s in &t.supertypes {
        parts.push(super::card::supertype_str(*s).to_lowercase());
    }

    // Subtypes (proper-cased names)
    for s in &t.subtypes {
        parts.push(s.name.to_string());
    }

    // Types
    for ty in &t.types {
        parts.push(super::card::type_str(*ty).to_lowercase());
    }

    parts.join(" ")
}

fn stat_value_str(v: &StatValue) -> String {
    match v {
        StatValue::Number(n) => n.to_string(),
        _ => "*".to_string(),
    }
}

pub(super) fn color_word(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Blue => "blue",
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
    }
}

fn token_abilities_suffix(abilities: &[Ability]) -> String {
    if abilities.is_empty() {
        return String::new();
    }
    let mut kw_names: Vec<String> = Vec::new();
    for ability in abilities {
        if let Ability::Keyword(k) = ability {
            kw_names.push(super::keyword::keyword_name(k).to_lowercase());
        }
    }
    if kw_names.is_empty() {
        return String::new();
    }
    let joined = match kw_names.len() {
        1 => kw_names.into_iter().next().unwrap(),
        2 => format!("{} and {}", kw_names[0], kw_names[1]),
        _ => {
            let (last, rest) = kw_names.split_last().unwrap();
            format!("{}, and {}", rest.join(", "), last)
        }
    };
    format!(" with {joined}")
}

fn trim_period(s: &str) -> String {
    s.strip_suffix('.').unwrap_or(s).to_string()
}

/// Capitalize the first character (sentence-start use, e.g. a named damage
/// source: "Target creature deals …").
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn ensure_period(s: &str) -> String {
    if s.ends_with(['.', '!', '?']) { s.to_string() } else { format!("{s}.") }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Action;
    use deckmaste_core::Binder;
    use deckmaste_core::Count;
    use deckmaste_core::Destination;
    use deckmaste_core::Each;
    use deckmaste_core::Effect;
    use deckmaste_core::Filter;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::With;
    use deckmaste_core::Zone;

    use super::Ctx;
    use super::action;
    use super::effect;

    /// `MayPay`/`MustPay` agree the payer's verb with its grammatical person
    /// ([CR#603,608,118.12a]): the default `you` actor takes second-person
    /// "do / don't / pay", a third-person actor ("that player") takes "does /
    /// doesn't / pays". Regression for the hardcoded third-person forms that
    /// rendered the ungrammatical "if you **does**, …" / "unless you **pays**".
    #[test]
    fn pay_clauses_agree_verb_person_with_payer() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::MayPay;
        use deckmaste_core::MustPay;
        use deckmaste_core::PlayerAction;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let one = || Cost(vec![CostComponent::Mana("{1}".parse().unwrap())]);
        let draw = || Box::new(Effect::act_by_you(PlayerAction::Draw(Count::Literal(1))));
        let lose = || {
            Box::new(Effect::act_by_you(PlayerAction::LoseLife(Count::Literal(
                1,
            ))))
        };

        // -- MayPay: "[payer] may pay {1}. If [payer] do(es), draw a card; if
        //    [payer] do(esn't), [lose]." --
        let may_you = Effect::MayPay(MayPay {
            actor: Reference::You,
            cost: one(),
            and_then: draw(),
            or_else: Some(lose()),
        });
        let rendered = effect(&may_you, &ctx);
        assert!(
            rendered.contains("If you do, ") && rendered.contains("; if you don't, "),
            "second-person MayPay: {rendered}"
        );
        assert!(
            !rendered.contains("you does") && !rendered.contains("you doesn't"),
            "no third-person -s for the `you` payer: {rendered}"
        );

        let may_them = Effect::MayPay(MayPay {
            actor: Reference::EventActor,
            cost: one(),
            and_then: draw(),
            or_else: Some(lose()),
        });
        let rendered = effect(&may_them, &ctx);
        assert!(
            rendered.contains("If that player does, ")
                && rendered.contains("; if that player doesn't, "),
            "third-person MayPay: {rendered}"
        );

        // -- MustPay: "[or_else] unless [payer] pay(s) {1}." --
        let must_you = Effect::MustPay(MustPay {
            actor: Reference::You,
            cost: one(),
            or_else: lose(),
        });
        let rendered = effect(&must_you, &ctx);
        assert!(
            rendered.contains("unless you pay {1}") && !rendered.contains("unless you pays"),
            "second-person MustPay: {rendered}"
        );

        let must_them = Effect::MustPay(MustPay {
            actor: Reference::EventActor,
            cost: one(),
            or_else: lose(),
        });
        let rendered = effect(&must_them, &ctx);
        assert!(
            rendered.contains("unless that player pays {1}"),
            "third-person MustPay: {rendered}"
        );
    }

    /// The default `This` source renders the implicit "Deal N damage to X";
    /// an explicit non-`This` source names the dealer — "<dealer> deals N
    /// damage to <target>" (the fight / redirected-damage surface).
    #[test]
    fn deal_damage_source_renders_dealer_phrase() {
        let target = TargetSpec::Target(Quantity::one(), Filter::creature());
        let ctx = Ctx {
            subject: "Pouncer",
            targets: std::slice::from_ref(&target),
            that: None,
        };

        let default = Action::deal_damage(Reference::Target(0), Count::Literal(3));
        assert_eq!(action(&default, &ctx), "Deal 3 damage to target creature.");

        let sourced = Action::DealDamage(
            Reference::Target(0),
            Count::Literal(3),
            Reference::Target(0),
        );
        assert_eq!(
            action(&sourced, &ctx),
            "Target creature deals 3 damage to target creature."
        );
    }

    /// A `Move`-to-library destination renders the anchor: `FromTop(0)` ->
    /// "top", `FromBottom(0)` -> "the bottom" ([CR#401.7]).
    #[test]
    fn move_to_library_renders_top_and_bottom() {
        use deckmaste_core::Anchor;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let top = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
        );
        assert_eq!(action(&top, &ctx), "Put it on top of your library.");
        let bottom = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
        );
        assert_eq!(
            action(&bottom, &ctx),
            "Put it on the bottom of your library."
        );
    }

    /// `MoveCounters` renders the `AllKinds` and named-kind forms ([CR#122]);
    /// `from`/`to` resolve through `ctx.targets`.
    #[test]
    fn move_counters_renders_all_kinds_and_named() {
        use deckmaste_core::CounterRef;
        use deckmaste_core::CounterSpec;

        let creature = TargetSpec::Target(Quantity::one(), Filter::creature());
        let targets = [creature.clone(), creature];
        let ctx = Ctx {
            subject: "it",
            targets: &targets,
            that: None,
        };
        let all = Action::MoveCounters(
            CounterSpec::AllKinds,
            Reference::Target(0),
            Reference::Target(1),
        );
        assert_eq!(
            action(&all, &ctx),
            "Move all counters from target creature onto target creature."
        );
        let named = Action::MoveCounters(
            CounterSpec::Named(CounterRef::from("P1P1Counter"), Count::Literal(1)),
            Reference::Target(0),
            Reference::Target(1),
        );
        assert_eq!(
            action(&named, &ctx),
            "Move 1 P1P1Counter counter from target creature onto target creature."
        );
    }

    /// `DivideAmong` renders by its body: a `DealDamage` body -> "Deal N damage
    /// divided as you choose among <group>" ([CR#601.2d]).
    #[test]
    fn divide_among_renders_divided_damage() {
        use deckmaste_core::DivideAmong;
        use deckmaste_core::Filter;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let divide = super::effect(
            &deckmaste_core::Effect::DivideAmong(DivideAmong {
                amount: Count::Literal(3),
                binder: Binder::Existing(Selection::Filter(Filter::creature())),
                body: Box::new(deckmaste_core::Effect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Allotment,
                ))),
            }),
            &ctx,
        );
        assert_eq!(
            divide,
            "Deal 3 damage divided as you choose among each creature."
        );
    }

    /// `AdditionalCost` renders the printed clause: a chosen-creature sacrifice
    /// cost reads "As an additional cost, sacrifice a creature." followed by
    /// the body sentence ([CR#601.2f,118.8]).
    #[test]
    fn additional_cost_renders_sacrifice_clause() {
        use deckmaste_core::AdditionalCost;
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::PlayerAction;

        let ctx = Ctx {
            subject: "Fling",
            targets: &[],
            that: None,
        };
        let fling = super::effect(
            &deckmaste_core::Effect::AdditionalCost(AdditionalCost {
                // "sacrifice a creature" is now the choose-then-pay `With` cost
                // step: ChooseOne(Creature) binds `That`, then `Sacrifice(That)`.
                pay: Cost(vec![CostComponent::With {
                    binder: Box::new(Binder::ChooseOne(Filter::creature())),
                    body: Cost(vec![CostComponent::do_(PlayerAction::Sacrifice(
                        Reference::That,
                    ))]),
                }]),
                body: Box::new(deckmaste_core::Effect::act_by_you(PlayerAction::Draw(
                    Count::Literal(1),
                ))),
            }),
            &ctx,
        );
        assert_eq!(
            fling,
            "As an additional cost, sacrifice a creature. Draw a card."
        );
    }

    /// `Effect::With` binds the binder's noun phrase as the body's `That`
    /// anaphor ([CR#601.2b]): a `ChooseOne` one-binder renders "Sacrifice a
    /// creature." — the choose-then-act surface that replaced the old
    /// verb-patient `Choose`.
    #[test]
    fn with_choose_one_renders_sacrifice_a_creature() {
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "Altar",
            targets: &[],
            that: None,
        };
        let with = Effect::With(With {
            binder: Binder::ChooseOne(Filter::creature()),
            body: Box::new(Effect::act_by_you(PlayerAction::Sacrifice(Reference::That))),
        });
        assert_eq!(effect(&with, &ctx), "Sacrifice a creature.");
    }

    /// A `Choose` many-binder contributes its "N <object>" phrase to the body's
    /// anaphor: `With(Choose(2, cards), Discard(That))` renders "Discard 2
    /// cards." ([CR#601.2b]).
    #[test]
    fn with_choose_many_renders_discard_two_cards() {
        use deckmaste_core::ObjectKind;
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "Wheel",
            targets: &[],
            that: None,
        };
        let with = Effect::With(With {
            binder: Binder::Choose(
                Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                Filter::Kind(ObjectKind::Card),
            ),
            body: Box::new(Effect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(2),
                what: Some(Reference::That),
            })),
        });
        assert_eq!(effect(&with, &ctx), "Discard 2 cards.");
    }

    /// `Effect::Each` over a many-binder collapses a single group-verb body
    /// (acting on the per-element `It`) to the collective surface —
    /// `Destroy(It)` → "Destroy each creature." — and falls back to the
    /// per-element "For each <group>, …" form for a body the collapse does not
    /// recognise ([CR#608]). This is the renderer half of
    /// `core-many-binder-group-move`.
    #[test]
    fn each_renders_collectively_or_per_element() {
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        // A group verb on the per-element `It` → the collective sentence.
        let destroy = Effect::Each(Each {
            binder: Binder::Existing(Selection::Filter(Filter::creature())),
            effect: Box::new(Effect::Act(Action::Destroy(Reference::It))),
        });
        assert_eq!(effect(&destroy, &ctx), "Destroy each creature.");
        // A body the collapse does not recognise → the per-element form.
        let gain = Effect::Each(Each {
            binder: Binder::Existing(Selection::Filter(Filter::creature())),
            effect: Box::new(Effect::act_by_you(PlayerAction::GainLife(Count::Literal(
                1,
            )))),
        });
        assert_eq!(effect(&gain, &ctx), "For each creature, gain 1 life.");
    }

    /// A player `Move` to the exile zone renders "Exile <subject>." — exiling
    /// is a pure zone move, not a dedicated verb ([CR#701.13]).
    #[test]
    fn player_move_to_exile_renders_exile_subject() {
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "Scavenger",
            targets: &[],
            that: None,
        };
        let exile = Action::by_you(PlayerAction::Move(
            Reference::This,
            Destination::Zone(Zone::Exile),
        ));
        assert_eq!(action(&exile, &ctx), "Exile Scavenger.");
    }
}
