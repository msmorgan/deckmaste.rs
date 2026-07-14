//! Effects / actions render to imperative sentences (spell mood).

use std::fmt::Write as _;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Arrangement;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::CounterSpec;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Destination;
use deckmaste_core::Duration;
use deckmaste_core::EnterRider;
use deckmaste_core::Modification;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::PlayerAttr;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::Sort;
use deckmaste_core::Stat;
use deckmaste_core::StatValue;
use deckmaste_core::StaticEffect;
use deckmaste_core::TargetSpec;
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

/// Render an `OneShotEffect` as one or more sentences joined into a single
/// rules string.
pub(super) fn effect(e: &OneShotEffect, ctx: &Ctx) -> String {
    match e {
        OneShotEffect::Act(a) => action(a, ctx),
        OneShotEffect::Sequentially(parts) => {
            let mut out = String::new();
            for (i, p) in parts.iter().enumerate() {
                let s = trim_period(&effect(p, ctx));
                // A delayed/reflexive trigger created mid-sequence ("Exile
                // target creature. At the beginning of the next end step,
                // ...", Otherworldly Journey) always reads as its OWN
                // sentence — never joined with ", then" the way two plain
                // instructions are ([CR#603.7,603.12]).
                let new_sentence = matches!(
                    peel_expanded(p),
                    OneShotEffect::Delayed(_) | OneShotEffect::Reflexive(_)
                );
                if i == 0 {
                    out.push_str(&s);
                } else if new_sentence {
                    out.push_str(". ");
                    out.push_str(&capitalize_first(&s));
                } else {
                    out.push_str(", then ");
                    out.push_str(&super::ability::lower_first(&s));
                }
            }
            out.push('.');
            out
        }
        // A macro invocation renders via its template. Args are resolved
        // context-aware here (the template layer is context-free and can't reach
        // `fragment`): a `Reference` arg — a fight's `Target(n)` — renders as its
        // phrase ("target creature you control"), so `${0} fights ${1}` reads
        // back to oracle; other args keep the context-free `render_slot`. The
        // sentence is capitalized (a reference-leading template starts lower).
        OneShotEffect::Expanded(e) => {
            let filled = e.template.as_deref().and_then(|tmpl| {
                super::template::fill_with(tmpl, ctx.subject, &e.args, |raw, modifier| {
                    if modifier.is_none()
                        && let Ok(r) = deckmaste_core::ron::options().from_str::<Reference>(raw)
                    {
                        return Some(fragment::reference(&r, ctx));
                    }
                    super::template::render_slot(raw, modifier)
                })
            });
            match filled {
                Some(s) => ensure_period(&capitalize_first(&s)),
                None => effect(&e.value, ctx),
            }
        }
        OneShotEffect::Continuously(c) => {
            let clause = super::ability::static_effect_one_shot(&c.effect, ctx).map_or_else(
                || format!("[unrendered: {:?}]", c.effect),
                |s| trim_period(&s),
            );
            // A `Deontic` restriction/permission ([CR#509.1b] "can't
            // block"/"can't be blocked", …) reads its `FixedUntil(EndOfTurn)`
            // trailer as "… this turn." — never the generic "… until end of
            // turn." every other `Continuously` shape (pump, keyword grant, …)
            // takes ([CR#611.2]). Every other duration/effect combo keeps the
            // generic front/trail qualifier.
            if is_deontic_restriction(&c.effect)
                && c.duration == Duration::FixedUntil(TurnMarker::EndOfTurn)
            {
                format!("{clause} this turn.")
            } else {
                duration_qualified(&c.duration, &clause, has_dynamic_pt_delta(&c.effect))
            }
        }
        // [CR#701.12a]: a batch of one-shot sub-effects sharing one snapshot
        // — the "Exchange" keyword action's primitive, covering both
        // "exchange control" ([CR#701.12b]) and "exchange life totals"
        // ([CR#701.12c]). Both current consumers are two MIRRORED halves of
        // one symmetric verb, so this dispatches to their dedicated phrasing
        // rather than joining the parts generically; a `Simultaneously`
        // outside those two shapes has no oracle-text precedent yet.
        OneShotEffect::Simultaneously(parts) => exchange_control_phrase(parts, ctx)
            .or_else(|| exchange_life_phrase(parts, ctx))
            .unwrap_or_else(|| format!("[unrendered: {e:?}].")),
        // [CR#608.2d]: "You may [effect]." — the bare optional-effect wrapper
        // (distinct from `MayPay`'s pay-a-cost kicker; this rule's own
        // example, "You may sacrifice a creature. If you don't, you lose 4
        // life.", is this exact "may … if you don't" shape). `if_did`/
        // `if_not` are unused by any current fixture; the base "You may …"
        // form is the
        // only shape rendered so far.
        OneShotEffect::May(m) => render_may(m, ctx),
        // The multi-part spelling of `Continuously` ([CR#611.2c] — a list of
        // static parts sharing one duration, the Boros Charm mode-2 shape).
        // Parts join "and" — the common case is one part; the corpus has no
        // multi-part `Until` fixture yet, so N>1 stays a plain "and" splice
        // rather than fabricated punctuation.
        OneShotEffect::Until(duration, parts) => {
            let clauses: Vec<String> = parts
                .iter()
                .map(|p| {
                    super::ability::static_effect_one_shot(p, ctx)
                        .map_or_else(|| format!("[unrendered: {p:?}]"), |s| trim_period(&s))
                })
                .collect();
            let clause = clauses.join(" and ");
            let leads = parts.len() == 1 && has_dynamic_pt_delta(&parts[0]);
            duration_qualified(duration, &clause, leads)
        }
        // A target-scoping wrapper ([CR#115.1,601.2c]): render the inner effect
        // with `ctx.targets` rebound to this node's targets, so the inner
        // the slot-bound anaphors resolve to "target creature" etc.
        OneShotEffect::Targeted(t) => effect(
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
        OneShotEffect::MustPay(m) => {
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
        OneShotEffect::MayPay(m) => {
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
        // [CR#601.2f,118.8]: "As an additional cost to cast ~, [pay]. [body]"
        // — the printed additional-cost clause whose body reads the paid
        // object (via the event references). The payment renders as a
        // symbol cost ("pay {2}") or an object-moving verb phrase
        // ("sacrifice a creature"); declines structurally if the cost has
        // no clean rendering. Only the SPELL-root wording is implemented
        // (every current corpus/test usage sits there); an activated
        // ability's printed additional cost ("to activate this ability")
        // is unhandled until a card needs it.
        OneShotEffect::AdditionalCost(ac) => match additional_payment(&ac.pay.0, ctx) {
            Some(pay) => {
                // "the sacrificed creature" (Fling), when the body reads it
                // back via `EventObject` — the cost-side twin of `With`'s
                // binder-phrase threading through `ctx.that`.
                let obj_phrase = additional_cost_object_phrase(&ac.pay.0);
                let body = match &obj_phrase {
                    Some(phrase) => effect(&ac.body, &ctx.with_that(phrase)),
                    None => effect(&ac.body, ctx),
                };
                format!(
                    "As an additional cost to cast {}, {pay}.\n{body}",
                    ctx.subject
                )
            }
            None => format!("[unrendered: {ac:?}]."),
        },
        // [CR#601.2d]: a divided distribution — the body picks the verb
        // ("deal … damage" vs "distribute … counters"), the amount is divided
        // "as you choose" among the group.
        OneShotEffect::Distribute(d) => divide_among(d, ctx),
        // [CR#601.2b]: a choose-then-act binder. The binder's noun phrase
        // ("a creature", "two cards") is bound as the body's `That`/`Those`
        // anaphor, so `With(ChooseOne(Creature), Sacrifice(That))` renders
        // "Sacrifice a creature."
        OneShotEffect::With(w) => {
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
        OneShotEffect::Each(fe) => {
            // Peel a remembered macro invocation (`Draws(It, 1)` →
            // `Expanded`) to its core `Act` so the collective renderer sees the
            // keyword-action tag ("Each player draws a card."), not "For each
            // player, …".
            if let OneShotEffect::Act(act) = peel_expanded(&fe.effect)
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
        // [CR#700.3a]: an opponent/the caster splits a group into labeled
        // piles, then (`then`) acts on them — the Do-or-Die/Fact-or-Fiction
        // family. Bespoke, like `divide_among`/`additional_payment`: the
        // pile-shape has no generic "spell one sentence per node" reading,
        // so this recognizes the shapes the corpus actually needs.
        OneShotEffect::SeparatePiles(piles) => separate_piles(piles, ctx),
        OneShotEffect::ChoosePile(cp) => choose_pile(cp, ctx),
        // [CR#700.2]: a modal spell/ability — an optional Escalate/Entwine
        // cost-rider line, then "Choose ... —" and one bulleted mode per
        // line.
        OneShotEffect::Modal(modal) => modal_effect(modal, ctx),
        // [CR#603.7]: a delayed triggered ability created on resolution —
        // "At the beginning of [event], [effect]." One-shot by construction,
        // so a step-based event reads with whatever "next"-qualified
        // phrasing its own macro template supplies (`NextEndStep` -> "the
        // beginning of the next end step") rather than the generic
        // recurring phrasing `ability::event_clause` uses for a permanent's
        // own (repeating) triggers.
        OneShotEffect::Delayed(t) => delayed(t, ctx),
        // [CR#702.85,701.57] the variable-length dig-until (cascade/discover's
        // shape): "Reveal cards from the top of [whose]'s library until you
        // reveal [a/an X]. [body]" — the found card reads as `It` ("that
        // card"), the passed-over prefix as the plural anaphor ("the rest").
        // Bespoke like `separate_piles`/`choose_pile`: recognizes the
        // corpus's put-found-card / relocate-prefix-to-an-ordered-library-
        // position shape, declines structurally otherwise.
        OneShotEffect::RevealUntil(r) => reveal_until(r, ctx),
        other => format!("[unrendered: {other:?}]."),
    }
}

/// See [`OneShotEffect::RevealUntil`]'s render arm above.
fn reveal_until(r: &deckmaste_core::RevealUntil, ctx: &Ctx) -> String {
    let whose = fragment::reference(&r.whose, ctx);
    let whose_poss = if whose.eq_ignore_ascii_case("you") {
        "your".to_string()
    } else {
        format!("{whose}'s")
    };
    let noun = format!("{} card", fragment::filter_noun(&r.matches));
    let lead = format!(
        "Reveal cards from the top of {whose_poss} library until you reveal {}.",
        a_an(&noun),
    );
    match reveal_until_body(&r.body, &whose_poss) {
        Some(body) => format!("{lead} {body}"),
        None => format!("{lead} [unrendered: {:?}].", r.body),
    }
}

/// Recognizes the corpus's dig-until body shape: put the found card (`It`)
/// somewhere, then relocate the passed-over prefix group (the plural
/// `Selection::They`) to an ordered library position — "Put that card into
/// your hand and the rest on the bottom of your library in a random order."
/// (Evolutionary Leap). `None` for any other body shape — no other card in
/// the corpus needs one yet.
fn reveal_until_body(body: &OneShotEffect, whose_poss: &str) -> Option<String> {
    let OneShotEffect::Sequentially(parts) = body else {
        return None;
    };
    let [
        OneShotEffect::Act(Action::Move(
            Reference::It,
            Destination::Zone(found_zone),
            found_riders,
        )),
        OneShotEffect::Act(Action::MoveGroup {
            group: Selection::They,
            arrangement,
            to: Destination::Library(anchor),
            riders: group_riders,
        }),
    ] = parts.as_slice()
    else {
        return None;
    };
    if !found_riders.is_empty() || !group_riders.is_empty() {
        return None;
    }
    let found_dest = match found_zone {
        Zone::Hand => "into your hand",
        _ => return None,
    };
    let order = match arrangement {
        Arrangement::RandomOrder => " in a random order",
        Arrangement::AnyOrder | Arrangement::ChosenOrder(_) => " in any order",
        Arrangement::SameOrder => "",
    };
    Some(format!(
        "Put that card {found_dest} and the rest on {} of {whose_poss} library{order}.",
        fragment::library_position(anchor),
    ))
}

/// A delayed triggered ability's lead-in + body ([CR#603.7]). See
/// [`OneShotEffect::Delayed`]'s render arm above for why this doesn't just call
/// `ability::event_clause` uncritically.
fn delayed(t: &deckmaste_core::TriggeredAbility, ctx: &Ctx) -> String {
    use deckmaste_core::EventFilter;
    let lead = match &t.event {
        EventFilter::Expanded(e) if e.template.is_some() => {
            format!("At {}", e.template.as_deref().unwrap_or_default())
        }
        other => {
            let (lead, clause) = super::ability::event_clause(other, ctx);
            format!("{lead} {clause}")
        }
    };
    let cond = match &t.condition {
        Some(c) => format!("if {}, ", super::condition::condition(c, ctx)),
        None => String::new(),
    };
    let body = super::ability::lower_first(&trim_period(&effect(&t.effect, ctx)));
    format!("{lead}, {cond}{body}.")
}

/// See through a macro invocation to its expanded value — the effect-side
/// twin of `fragment::strip_expanded` (which does the same for `Predicate`).
fn peel_expanded(e: &OneShotEffect) -> &OneShotEffect {
    match e {
        OneShotEffect::Expanded(exp) => peel_expanded(&exp.value),
        other => other,
    }
}

/// `SeparatePiles { group, into, by, note, then }` ([CR#700.3a]) — "[by]
/// separates [group] into [n] piles.", with a `TopOfLibrary` group preceded
/// by its own "Reveal the top N cards of [owner]'s library." sentence (the
/// group is revealed as part of being separated, [CR#701.20a]). `then`
/// (almost always a `ChoosePile`) follows as its own sentence.
fn separate_piles(piles: &deckmaste_core::SeparatePiles, ctx: &Ctx) -> String {
    let by = fragment::reference(&piles.by, ctx);
    let is_you = by.eq_ignore_ascii_case("you");
    let (preamble, noun) = match &piles.group {
        Selection::TopOfLibrary { count, whose } => {
            let owner = fragment::reference(whose, ctx);
            (
                Some(format!(
                    "Reveal the top {} cards of {}'s library.",
                    fragment::count(count),
                    if owner.eq_ignore_ascii_case("you") {
                        "your".to_string()
                    } else {
                        format!("{owner}'s")
                    },
                )),
                "those cards".to_string(),
            )
        }
        Selection::SelectAll(f) => (None, plural_group_noun(f, ctx)),
        other => (None, format!("[unrendered: {other:?}]")),
    };
    let piles_word = fragment::number_word(u32::try_from(piles.into.len()).unwrap_or(0))
        .map_or_else(|| piles.into.len().to_string(), str::to_string);
    let mut out = String::new();
    if let Some(pre) = preamble {
        out.push_str(&pre);
        out.push(' ');
    }
    if is_you {
        // The default, unnamed actor (`by: You`) renders imperative, like
        // every other bare-you verb ("Destroy target creature.") — no
        // subject pronoun ("Separate ..." not "You separate ...").
        let _ = write!(out, "Separate {noun} into {piles_word} piles.");
    } else {
        out.push_str(&capitalize_first(&by));
        let _ = write!(out, " separates {noun} into {piles_word} piles.");
    }
    if let Some(then) = &piles.then {
        out.push(' ');
        out.push_str(&ensure_period(&effect(then, ctx)));
    }
    out
}

/// The plural noun a `Selection::SelectAll` names, for the "[by] separates
/// [noun] into..." slot — "all creatures target player controls" (Do or
/// Die's shape: a `Type` + a `ControlledBy(<dynamic reference>)` restrictor
/// the shared `fragment::filter_noun` doesn't cover, since it only prints
/// the fixed you/opponent controller phrases).
fn plural_group_noun(f: &deckmaste_core::Predicate, ctx: &Ctx) -> String {
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Predicate;
    use deckmaste_core::RelationPredicate;
    let parts: Vec<&Predicate> = match f {
        Predicate::And(members) => members.iter().collect(),
        other => vec![other],
    };
    let mut noun = None;
    let mut controller = None;
    for part in parts {
        match part {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                noun = Some(format!("{}s", t.as_str().to_lowercase()));
            }
            Predicate::Relation(RelationPredicate::ControlledBy(who)) => {
                if let Predicate::Ref(r) = who.as_ref() {
                    controller = Some(format!("{} controls", fragment::reference(r, ctx)));
                }
            }
            _ => {}
        }
    }
    match (noun, controller) {
        (Some(n), Some(c)) => format!("all {n} {c}"),
        (Some(n), None) => format!("all {n}"),
        (None, _) => format!("[unrendered: {f:?}]"),
    }
}

/// `ChoosePile { from, by, random, then }` ([CR#700.3b]) — recognizes the
/// "act on the chosen pile" shape (`then` is an `Each` over
/// `Existing(Them(Pile))`, peeling a macro wrapper) and renders the
/// collective sentence the corpus needs ("Destroy all creatures in the pile
/// of `[by]`'s choice. They can't be regenerated."); anything else declines
/// structurally.
fn choose_pile(cp: &deckmaste_core::ChoosePile, ctx: &Ctx) -> String {
    let chooser = fragment::reference(&cp.by, ctx);
    let pile_phrase = format!("the pile of {chooser}'s choice");
    if let OneShotEffect::Each(each) = peel_expanded(&cp.then)
        && matches!(
            &each.binder,
            deckmaste_core::Binder::Existing(Selection::Them(Sort::Pile))
        )
        && let Some(collective) = pile_collective(peel_expanded(&each.effect), &pile_phrase)
    {
        return collective;
    }
    format!("[unrendered: {cp:?}].")
}

/// The per-element body of a `ChoosePile`'s pile-wide `Each`, collapsed to
/// the collective sentence CR text uses ("Destroy all creatures in
/// [group]." rather than "For each creature in [group], destroy it."). Only
/// the shapes the corpus needs are recognized; `None` declines to the
/// caller's structural fallback.
fn pile_collective(body: &OneShotEffect, group_phrase: &str) -> Option<String> {
    use deckmaste_core::KeywordAction;
    match body {
        OneShotEffect::Act(Action::Composite(KeywordAction::Destroy(Reference::It), _)) => {
            Some(format!("Destroy all creatures in {group_phrase}."))
        }
        // `DestroyNoRegen`'s expansion: `Sequentially([Composite(Destroy(It),
        // Move(It, Graveyard)), Until(ForThisEvent, [Cant(Regenerate(on:
        // It))])])` ([CR#701.19c]).
        OneShotEffect::Sequentially(parts) => match parts.as_slice() {
            [
                OneShotEffect::Act(Action::Composite(KeywordAction::Destroy(Reference::It), _)),
                OneShotEffect::Until(Duration::ForThisEvent, statics),
            ] => match statics.as_slice() {
                [StaticEffect::Deontic(Deontic::Cant(DeonticAction::Regenerate { .. }))] => Some(
                    format!("Destroy all creatures in {group_phrase}. They can't be regenerated."),
                ),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// A modal spell/ability ([CR#700.2]): an optional Escalate/Entwine cost-
/// rider line, "Choose ... —", and one bulleted mode per line.
fn modal_effect(modal: &deckmaste_core::Modal, ctx: &Ctx) -> String {
    use deckmaste_core::ModalCostRider;
    let mut lines = Vec::new();
    if let Some(rider) = &modal.choose.rider {
        let (name, cost) = match rider {
            ModalCostRider::Escalate(cost) => ("Escalate", cost),
            ModalCostRider::Entwine(cost) => ("Entwine", cost),
        };
        if let Some(symbols) = super::template::render_cost(&cost.0) {
            // Reminder text ("(Pay this cost for each mode chosen beyond
            // the first.)") is never rendered — the fidelity gate strips it
            // from the ORACLE side only, so the rules-text renderer must
            // already omit it (mirrors every keyword's bare-name render).
            lines.push(format!("{name} {symbols}"));
        }
    }
    lines.push(choose_line(&modal.choose));
    for mode in &modal.modes {
        lines.push(format!(
            "\u{2022} {}",
            ensure_period(&effect(&mode.effect, ctx))
        ));
    }
    lines.join("\n")
}

/// The modal spec's "Choose ..." lead line ([CR#700.2]). Only the shapes the
/// corpus needs (`AtLeast(1)` = "one or more") are named; anything else
/// falls back to that reading rather than fabricating unverified wording.
fn choose_line(spec: &deckmaste_core::ChooseSpec) -> String {
    let (lo, hi) = spec.count.bounds();
    let lo_n = lo.and_then(Count::literal_value);
    let hi_n = hi.and_then(Count::literal_value);
    match (lo_n, hi_n, spec.up_to) {
        (Some(1), Some(1), false) => "Choose one \u{2014}".to_string(),
        _ => "Choose one or more \u{2014}".to_string(),
    }
}

/// The noun phrase a [`Binder`](deckmaste_core::Binder) contributes to its
/// `OneShotEffect::With` / `CostComponent::With` body — read by the body's
/// `That` / `Those` anaphor ([CR#601.2b]). A one-binder yields "a creature"; a
/// many-binder yields "two cards"; the reference/existing forms defer to the
/// shared fragment renderers.
fn binder_phrase(binder: &deckmaste_core::Binder, ctx: &Ctx) -> String {
    use deckmaste_core::Binder;
    #[expect(
        clippy::match_same_arms,
        reason = "the chooser binders (ChooseOne/Choose) and the search binders (SearchOne/Search) are distinct grammar categories kept separate for the documented reasons above; they coincidentally render the same noun phrase"
    )]
    match binder {
        // The chooser (`by`) does not surface in the noun phrase — the body's
        // verb rendering carries the acting player; a foreign chooser has no
        // corpus card yet.
        Binder::ChooseOne { filter, .. } => a_an(&fragment::filter_noun(filter)),
        Binder::Choose {
            quantity, filter, ..
        } => {
            format!(
                "{} {}",
                fragment::quantity(quantity),
                fragment::filter_object(filter)
            )
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

pub(super) fn a_an(noun: &str) -> String {
    let lowercase = noun.to_lowercase();
    let is_vowel = |c: char| "aeiou".contains(c);
    if let Some(first_char) = lowercase.chars().next()
        && is_vowel(first_char)
    {
        return format!("an {noun}");
    }
    format!("a {noun}")
}

/// The collective rendering of an [`OneShotEffect::Each`] whose body is a
/// single group verb acting on the per-element [`Reference::It`] — the natural
/// "<verb> each <group>" / "put <group> on <dest>" surface ([CR#608]), the
/// renderer half the `core-many-binder-group-move` seam calls for. Returns
/// `None` for any body the collapse does not recognise, so the caller falls
/// back to the per-element "For each <group>, …" form.
fn each_collective(act: &Action, binder: &deckmaste_core::Binder, ctx: &Ctx) -> Option<String> {
    // "each <bare group noun>" — the recipient/patient of a set-wide verb.
    let each_group = || format!("each {}", binder_group_noun(binder, ctx));
    match act {
        // "<source> deals N damage to each <group>." — damage names its
        // source; the default `This` reads as the carrier itself.
        Action::DealDamage(source, amount, Reference::It) => {
            let dealer = match source {
                Reference::This => ctx.subject.to_string(),
                other => fragment::reference(other, ctx),
            };
            Some(format!(
                "{} deals {} damage to {}.",
                capitalize_first(&dealer),
                fragment::count(amount),
                each_group(),
            ))
        }
        // "Destroy each <group>." ([CR#701.8a]) — destroy is the
        // `Composite(Destroy(It), Move(It, Graveyard))` the `Destroy` macro
        // builds; the tag carries the printed keyword.
        Action::Composite(deckmaste_core::KeywordAction::Destroy(Reference::It), _) => {
            Some(format!("Destroy {}.", each_group()))
        }
        // [CR#121.1,121.2]: "Each player draws N cards." — draw is
        // `Composite(Draw(It, n), <body>)` over the loop element as performer
        // (Jace Beleren's "[+2]: Each player draws a card."); the twin mill form
        // reads "Each player mills N cards." Both render the tag.
        Action::Composite(deckmaste_core::KeywordAction::Draw(Reference::It, n), _) => {
            Some(format!(
                "{} draws {}.",
                capitalize_first(&each_group()),
                counted_cards(n),
            ))
        }
        Action::Composite(deckmaste_core::KeywordAction::Mill(Reference::It, n), _) => {
            Some(format!(
                "{} mills {}.",
                capitalize_first(&each_group()),
                counted_cards(n),
            ))
        }
        // A group move to the library reads "Put <group> on top/the bottom of
        // your library." — Brainstorm's "put two cards … on top": the chosen
        // group's own phrase, not "each" ([CR#401.7]). A multi-card put
        // prints the "in any order" rider the oracle sentence carries.
        // Riders never apply to a library destination, so a rider-carrying
        // move falls through.
        Action::Move(Reference::It, Destination::Library(anchor), riders) if riders.is_empty() => {
            Some(format!(
                "Put {} on {} of your library{}.",
                binder_phrase(binder, ctx),
                fragment::library_position(anchor),
                if binder_is_plural(binder) { " in any order" } else { "" },
            ))
        }
        // Set-wide tap/untap ([CR#701.26a,701.26b]): "Tap each <group>."
        Action::By(_, PlayerAction::Tap(Reference::It)) => Some(format!("Tap {}.", each_group())),
        Action::By(_, PlayerAction::Untap(Reference::It)) => {
            Some(format!("Untap {}.", each_group()))
        }
        // [CR#119.1,119.5]: "Each player's life total becomes N." — a
        // possessive-subject sentence (the value belongs to the loop
        // element), unlike the subject-verb pattern the generic `By(It, pa)`
        // arm below handles ("Each player mills …"). Arbiter of Knollridge's
        // own shape: `count` is typically a cross-player `Aggregate`
        // ([CR#119.1]) reading "the highest life total among all players".
        Action::By(Reference::It, PlayerAction::SetLife(count)) => Some(format!(
            "{}'s life total becomes {}.",
            capitalize_first(&each_group()),
            fragment::count(count)
        )),
        // Subject-declarative player verbs over the loop element as AGENT
        // ([CR#608.2d] distributive each; [CR#701.17a,701.9,121.1,119.3]):
        // "Each player mills two cards." / "Each opponent loses 2 life."
        Action::By(Reference::It, pa) => {
            let verb = third_person_verb_phrase(pa)?;
            Some(format!("{} {verb}.", capitalize_first(&each_group())))
        }
        _ => None,
    }
}

/// Whether a binder binds MORE than one object (a multi-card group move
/// prints its "in any order" rider).
fn binder_is_plural(binder: &deckmaste_core::Binder) -> bool {
    use deckmaste_core::Binder;
    use deckmaste_core::Count;
    use deckmaste_core::Selection;
    match binder {
        Binder::Choose { quantity, .. } | Binder::Search { quantity, .. } => !quantity.is_one(),
        Binder::ChooseOne { .. } | Binder::SearchOne { .. } | Binder::TheRef(_) => false,
        // A literal single-card ordered selection (Soldevi Digger's "the top
        // card of your graveyard") is NOT plural — only a >1 (or dynamic,
        // conservatively treated as plural) count carries the rider.
        Binder::Existing(
            Selection::TopOfLibrary { count, .. } | Selection::TopOfGraveyard { count, .. },
        ) => !matches!(count, Count::Literal(1)),
        Binder::Existing(_) | Binder::Produce(_) => true,
        Binder::Expanded(e) => binder_is_plural(&e.value),
    }
}

/// The bare collective noun a [`Binder`](deckmaste_core::Binder) contributes to
/// an [`OneShotEffect::Each`] "each <noun>" / "For each <noun>" construction:
/// the whole matching set yields the bare noun ("creature", so the surrounding
/// "each" supplies the quantifier — not "each each creature"), a
/// bound/announced group its plural anaphor ("them"), and a chosen group its
/// full phrase.
fn binder_group_noun(binder: &deckmaste_core::Binder, ctx: &Ctx) -> String {
    use deckmaste_core::Binder;
    use deckmaste_core::Selection;
    match binder {
        Binder::Existing(Selection::SelectAll(f)) => fragment::filter_noun(f),
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

/// The LEADING spelling of a duration ("Until end of turn, ..."), the
/// mirror-image of [`duration_suffix`]'s trailing "... until end of turn.".
fn duration_prefix(d: &Duration) -> Option<String> {
    match d {
        Duration::FixedUntil(m) => Some(format!("Until {}", turn_marker(*m))),
        Duration::EndOfGame => None,
        other => Some(format!("[unrendered: {other:?}]")),
    }
}

/// Join a one-shot-created continuous effect's clause with its duration,
/// choosing FRONT ("Until end of turn, target creature gets +1/+1 for each
/// ...") vs TRAILING ("Target creature gets +2/+2 until end of turn.")
/// placement. Real oracle text varies stylistically, but consistently fronts
/// the duration for a DYNAMIC-magnitude effect (Embiggen's per-axis pump,
/// Exponential Growth's doubling) and trails it for the common fixed-number
/// pump / ability-grant shape (Giant Growth, Collective Resistance) —
/// `leads` carries that call from the caller, which has the effect shape in
/// hand.
/// Structural match for one HALF of an "exchange control" pair ([CR#701.12a],
/// Avarice Totem): `Continuously(Forever, Modify(<target>, SetController(
/// ControllerOf(<other>))))`. Returns `(target, other)` — the modified
/// permanent and the permanent whose controller it's assuming. `None` for any
/// other shape (a plain duration-bounded control grant, e.g., isn't this).
fn exchange_control_half(e: &OneShotEffect) -> Option<(&Reference, &Reference)> {
    let OneShotEffect::Continuously(c) = e else {
        return None;
    };
    if c.duration != Duration::EndOfGame {
        return None;
    }
    let StaticEffect::Modify(target, Modification::SetController(source)) = c.effect.as_ref()
    else {
        return None;
    };
    let Reference::ControllerOf(other) = source else {
        return None;
    };
    Some((target, other.as_ref()))
}

/// Whether a `Simultaneously`'s two parts are a mirrored "exchange control"
/// pair — used both by the phrase renderer below and by
/// [`super::ability::effect_wants_self_type_phrase`] (the activated
/// ability's `Ctx.subject` needs "this artifact", not the printed name, for
/// this one shape).
pub(super) fn exchange_control_refs(parts: &[OneShotEffect]) -> Option<(&Reference, &Reference)> {
    let [a, b] = parts else { return None };
    let (target_a, other_a) = exchange_control_half(a)?;
    let (target_b, other_b) = exchange_control_half(b)?;
    if target_a == other_b && target_b == other_a {
        Some((target_a, target_b))
    } else {
        None
    }
}

/// "Exchange control of {a} and {b}." ([CR#701.12a,701.12b]) — Avarice
/// Totem's activated ability body. The `Simultaneously`-of-two-`Continuously`
/// primitive shape has no generic oracle phrasing of its own (see the
/// `Simultaneously` dispatch above), so this recognizes the mirrored pair
/// structurally rather than walking each half independently.
fn exchange_control_phrase(parts: &[OneShotEffect], ctx: &Ctx) -> Option<String> {
    let (a, b) = exchange_control_refs(parts)?;
    Some(format!(
        "Exchange control of {} and {}.",
        fragment::reference(a, ctx),
        fragment::reference(b, ctx),
    ))
}

/// Structural match for one HALF of an "exchange life totals" pair
/// ([CR#701.12a,701.12c], Axis of Mortality): `By(<actor>, SetLife(
/// PlayerStatOf(<other>, Life)))`. Returns `(actor, other)` — the player
/// whose life is being set and the player whose (pre-exchange) total it's
/// copying.
fn exchange_life_half(e: &OneShotEffect) -> Option<(&Reference, &Reference)> {
    let OneShotEffect::Act(Action::By(actor, PlayerAction::SetLife(count))) = e else {
        return None;
    };
    let Count::PlayerStatOf(other, PlayerAttr::Life) = count else {
        return None;
    };
    Some((actor, other))
}

/// "have two target players exchange life totals" ([CR#701.12a,701.12c],
/// Axis of Mortality's `May`-wrapped trigger body): the mirrored `SetLife`/
/// `PlayerStatOf` pair reads as one collective verb over the announced
/// targets, not two separate "X's life total becomes Y" sentences — the
/// per-half phrasing (`Action::By(Reference::It, PlayerAction::SetLife(_))`,
/// used by the distributive-`Each` "Each player's life total becomes N")
/// doesn't apply here since neither half's actor is the loop anaphor `It`.
fn exchange_life_phrase(parts: &[OneShotEffect], ctx: &Ctx) -> Option<String> {
    let [a, b] = parts else { return None };
    let (actor_a, other_a) = exchange_life_half(a)?;
    let (actor_b, other_b) = exchange_life_half(b)?;
    if actor_a != other_b || actor_b != other_a {
        return None;
    }
    let targets_phrase = two_same_targets_phrase(ctx.targets).unwrap_or_else(|| {
        format!(
            "{} and {}",
            fragment::reference(actor_a, ctx),
            fragment::reference(actor_b, ctx)
        )
    });
    Some(format!("have {targets_phrase} exchange life totals"))
}

/// Two SEPARATELY-announced target slots sharing one predicate collapse to a
/// single count phrase ([CR#115.3]): `[TargetOne(Player), TargetOne(Player)]`
/// -> "two target players" — distinct from
/// [`fragment::announced_group_phrase`], which reads ONE slot's plural
/// `Quantity` ("one, two, or three targets"). `None` for any other target
/// list shape (different predicates, a non-singleton quantity, …).
fn two_same_targets_phrase(targets: &[TargetSpec]) -> Option<String> {
    // Peel macro provenance first — `TargetOne`/`Exactly`/… (the RON-surface
    // spellings every fixture actually authors) are builtin macros expanding
    // to the raw `Target(Quantity, Predicate)` node, not that node directly.
    fn peel(t: &TargetSpec) -> &TargetSpec {
        match t {
            TargetSpec::Expanded(exp) => peel(&exp.value),
            other => other,
        }
    }
    let [a, b] = targets else { return None };
    let (TargetSpec::Target(qa, fa), TargetSpec::Target(qb, fb)) = (peel(a), peel(b)) else {
        return None;
    };
    if !qa.is_one() || !qb.is_one() || fa != fb {
        return None;
    }
    Some(format!("two target {}s", fragment::filter_noun(fa)))
}

fn duration_qualified(d: &Duration, clause: &str, leads: bool) -> String {
    if leads {
        match duration_prefix(d) {
            Some(prefix) => format!("{prefix}, {}.", super::ability::lower_first(clause)),
            None => format!("{clause}."),
        }
    } else {
        match duration_suffix(d) {
            Some(suffix) => format!("{clause} {suffix}."),
            None => format!("{clause}."),
        }
    }
}

/// Whether a `StaticEffect`'s modification carries a non-literal P/T delta —
/// "gets +1/+1 for each ..." / "double ... power X times" — vs the common
/// fixed-number pump ("gets +N/+N") or ability grant. Drives
/// [`duration_qualified`]'s front-vs-trail choice.
fn has_dynamic_pt_delta(e: &StaticEffect) -> bool {
    match e {
        StaticEffect::Expanded(exp) => has_dynamic_pt_delta(&exp.value),
        StaticEffect::Modify(_, change) => modification_has_dynamic_pt_delta(change),
        _ => false,
    }
}

/// Whether a `StaticEffect` is a `Deontic` (`Cant`/`May`/`Must`/`Gate`)
/// restriction/permission — the `Continuously` family that reads its
/// `FixedUntil(EndOfTurn)` trailer as "this turn." rather than "until end of
/// turn." ([CR#509.1b] "can't block"/"can't be blocked", et al.).
fn is_deontic_restriction(e: &StaticEffect) -> bool {
    match e {
        StaticEffect::Expanded(exp) => is_deontic_restriction(&exp.value),
        StaticEffect::Deontic(_) => true,
        _ => false,
    }
}

/// `has_dynamic_pt_delta`'s `Modification`-level recursion — looks through
/// `Several`/`Expanded` (a macro-bundled "+N/+N" carries the same shape).
fn modification_has_dynamic_pt_delta(m: &Modification) -> bool {
    use deckmaste_core::NumericOp;
    match m {
        Modification::Power(NumericOp::Up(c) | NumericOp::Down(c))
        | Modification::Toughness(NumericOp::Up(c) | NumericOp::Down(c)) => {
            !matches!(c, Count::Literal(_))
        }
        Modification::Several(parts) => parts.iter().any(modification_has_dynamic_pt_delta),
        Modification::Expanded(exp) => modification_has_dynamic_pt_delta(&exp.value),
        _ => false,
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
        // Damage always names its source in oracle text ("~ deals 3 damage
        // to any target"): the default `This` source reads as the carrier
        // itself — the card name at a spell root, "it" inside a trigger
        // body — "<source> deals N damage to <target>". A dynamic amount
        // prints the oracle X-form with its "where X is …" definition
        // clause.
        Action::DealDamage(source, amount, target) => {
            let dealer = match source {
                Reference::This => ctx.subject.to_string(),
                other => fragment::reference(other, ctx),
            };
            // A stat-derived amount ("equal to the sacrificed creature's
            // power", Fling; the one-sided "bite" shape's "equal to its
            // power") reads a different template than the plain/dynamic-X
            // forms — "damage EQUAL TO X" up front, not "N damage ... where
            // X is N" (there is no substituted variable here at all).
            if let Count::StatOf(r, stat) = amount {
                // `This` is the self-possessive pronoun "its" — NOT the
                // generic `reference(r, ctx) + "'s"` composition, which for
                // `This` reads `ctx.subject` a second time (e.g. "it") and
                // would print the ungrammatical "it's power" instead of
                // "its power".
                let whose = match r {
                    Reference::This => "its".to_string(),
                    other => format!("{}'s", fragment::reference(other, ctx)),
                };
                return format!(
                    "{} deals damage equal to {whose} {} to {}.",
                    capitalize_first(&dealer),
                    stat_word(*stat),
                    fragment::reference(target, ctx),
                );
            }
            let (value, where_x) = damage_amount(amount);
            format!(
                "{} deals {value} damage to {}{}.",
                capitalize_first(&dealer),
                fragment::reference(target, ctx),
                where_x.map_or_else(String::new, |w| format!(", {w}")),
            )
        }
        // [CR#701.8a]: destroy is `Composite(Destroy(r), Move(r, Graveyard))` —
        // render the tag as "Destroy <patient>." context-aware (the `Move` body
        // is engine realization, not printed). This specific arm precedes the
        // generic `Composite(_, body)` below.
        Action::Composite(deckmaste_core::KeywordAction::Destroy(r), _) => {
            format!("Destroy {}.", fragment::reference(r, ctx))
        }
        // [CR#701.17a]: mill is `Composite(Mill(who, n), <group move>)` — render
        // the tag. `You` keeps the imperative "Mill N cards."; a non-`You`
        // performer renders subject-declarative ("Target player mills five
        // cards.", [CR#701.17a]). The `MoveGroup` body is engine realization, not
        // printed. Precedes the generic `Composite(_, body)` arm below.
        Action::Composite(deckmaste_core::KeywordAction::Mill(who, n), _) => match who {
            Reference::You => mill_imperative(n),
            other => format!(
                "{} mills {}.",
                capitalize_first(&fragment::reference(other, ctx)),
                counted_cards(n),
            ),
        },
        // [CR#121.1,121.2]: draw is `Composite(Draw(who, n), <body>)` — render
        // the tag, twin of the mill arm above. `You` keeps the imperative "Draw
        // N cards."; a non-`You` performer renders subject-declarative ("Target
        // player draws N cards."). The `Each` body is engine realization (the
        // per-card late library-top bind), not printed.
        Action::Composite(deckmaste_core::KeywordAction::Draw(who, n), _) => match who {
            Reference::You => draw_imperative(n),
            other => format!(
                "{} draws {}.",
                capitalize_first(&fragment::reference(other, ctx)),
                counted_cards(n),
            ),
        },
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
        // A rider list never applies to a library destination.
        Action::Move(r, Destination::Library(anchor), riders) if riders.is_empty() => format!(
            "Put {} on {} of your library.",
            fragment::reference(r, ctx),
            fragment::library_position(anchor),
        ),
        // [CR#401.4]: a GROUP move to an ordered library position — Brainstorm's
        // "Put <group> on top of your library in any order." The "any order"
        // rider prints for the arranged arrangements ([CR#401.4]); a fixed
        // `SameOrder`/`RandomOrder` group carries no such rider.
        Action::MoveGroup {
            group,
            arrangement,
            to: Destination::Library(anchor),
            riders,
        } if riders.is_empty() => format!(
            "Put {} on {} of your library{}.",
            fragment::selection(group, ctx),
            fragment::library_position(anchor),
            if matches!(
                arrangement,
                deckmaste_core::Arrangement::AnyOrder | deckmaste_core::Arrangement::ChosenOrder(_)
            ) {
                " in any order"
            } else {
                ""
            },
        ),
        // [CR#701]: a named keyword action is transparent to structural
        // rendering — its meaning IS its body (the printed keyword name rides
        // the macro template when authored via a macro).
        Action::Composite(_, body) => effect(body, ctx),
        // [CR#701.13a]/[CR#400.7]: exiling a graveyard-hate target — "Exile
        // target [<type>] card from a graveyard." — any player's graveyard,
        // per `any_graveyard_card_filter` (migrations effect.rs), so "from a
        // graveyard" is static text appended here, not derived from the
        // reference. Unlike the reanimation arm's bare reference-SHAPE guard
        // (`It`/`This` alone), this `Move(It, Exile)` shape is shared with the
        // general "Exile target <subject>." production below, so the zone
        // must be read off the actual target filter.
        Action::Move(r, Destination::Zone(Zone::Exile), riders)
            if riders.is_empty()
                && fragment::sole_target_filter(r, ctx)
                    .is_some_and(fragment::is_graveyard_scoped) =>
        {
            format!("Exile {} from a graveyard.", fragment::reference(r, ctx))
        }
        // Exiling is a pure zone move ([CR#701.13]) — "Exile <r>." (the
        // source-agent twin of `PlayerAction::Move`'s identical exile arm).
        Action::Move(r, Destination::Zone(Zone::Exile), riders) if riders.is_empty() => {
            format!("Exile {}.", fragment::reference(r, ctx))
        }
        // [CR#402.1]: a hand destination — "Return <r> to your hand." The
        // self-referential graveyard-return idiom (Death Spark's "return
        // this card to your hand") and the "return ~/it/that card to your
        // hand" bounce forms (the retired `ReturnToHand` verb's migrated
        // self/anaphor productions) are the shapes this corpus exercises
        // yet; a target-relative "to its owner's hand" (Unsummon-style
        // bounce, or a self-bounce whose owner may differ from its
        // controller) is unbuilt — flagged for the next card that needs it,
        // since the renderer has no signal here to tell an owned-by-you
        // reference from an arbitrary one.
        Action::Move(r, Destination::Zone(Zone::Hand), riders) if riders.is_empty() => {
            format!("Return {} to your hand.", fragment::reference(r, ctx))
        }
        // [CR#400.7]: graveyard reanimation — the empty-rider battlefield
        // case (the migrations `parse_reanimate` production's `Move(It|This,
        // Battlefield)`, no rider since the owner-control default already
        // applies to a "your graveyard" subject). "from your graveyard" is
        // static text here, not derived from the reference: a bare `Move`
        // carries no origin-zone field. The reference SHAPE — `It`/`This`,
        // the only forms `parse_reanimate` emits — is what reserves this
        // phrasing for the graveyard-recursion family. It must NOT catch a
        // bare-reference return from another origin: `parse_return_that_card`
        // emits a riderless `Move(That(Card), Battlefield)` for an exile
        // return (Otherworldly Journey), which is not graveyard-sourced and
        // would be mislabelled here — so the guard excludes `That`/`Target`,
        // letting those fall through to the zone-agnostic/unrendered path.
        Action::Move(
            r @ (Reference::It | Reference::This),
            Destination::Zone(Zone::Battlefield),
            riders,
        ) if riders.is_empty() => {
            format!(
                "Return {} from your graveyard to the battlefield.",
                fragment::reference(r, ctx)
            )
        }
        // A battlefield destination WITH arrival riders ([CR#614.12],
        // Otherworldly Journey's delayed return): "Return <r> to the
        // battlefield <rider phrase>."
        Action::Move(r, Destination::Zone(Zone::Battlefield), riders) if !riders.is_empty() => {
            format!(
                "Return {} to the battlefield{}.",
                fragment::reference(r, ctx),
                enter_rider_phrase(riders, ctx),
            )
        }
        // A non-`You` agent renders subject-declarative ("Target player
        // mills two cards.", [CR#701.17a]); the implicit-`You` default keeps
        // the imperative form ("Draw a card."). A verb with no third-person
        // phrase falls back to the imperative render.
        Action::By(who, pa) => match who {
            Reference::You => player_action(pa, ctx),
            other => third_person_verb_phrase(pa).map_or_else(
                || player_action(pa, ctx),
                |verb| {
                    format!(
                        "{} {verb}.",
                        capitalize_first(&fragment::reference(other, ctx))
                    )
                },
            ),
        },
        // [CR#701.19a]: a regeneration shield — rendered as "Regenerate <target>."
        // when the replacement body has the standard structure. The top-level
        // `Regenerate` keyword macro emits this via its template.
        Action::CreateReplacement { subject, .. } => {
            format!("Regenerate {}.", fragment::reference(subject, ctx))
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// A damage amount as its printed value plus, for a dynamic amount, the
/// "where X is …" definition clause the oracle sentence carries — the
/// `where_x` adjunct must survive to the render, never silently collapse to
/// a bare macro name.
fn damage_amount(amount: &Count) -> (String, Option<String>) {
    match amount {
        Count::Literal(_) | Count::X | Count::ThatMany | Count::ThatMuch => {
            (fragment::count(amount), None)
        }
        // A dynamic amount: skip a macro invocation's own one-word template
        // (e.g. Domain's "domain") — the definition clause spells the
        // computation out.
        Count::Expanded(e) => (
            "X".to_string(),
            Some(format!("where X is {}", fragment::count(&e.value))),
        ),
        other => (
            "X".to_string(),
            Some(format!("where X is {}", fragment::count(other))),
        ),
    }
}

/// The trailing " under ... control with a +1/+1 counter on it" clause an
/// arrival-rider list contributes ([CR#614.12]) — only the riders the
/// corpus needs (`UnderOwnersControl`/`UnderControlOf`, `WithCounters`) are
/// named; `Tapped`/`FaceDown`/`Attacking` fall back to a plain joined word so
/// the enum stays exhaustive without fabricating unverified phrasing.
fn enter_rider_phrase(riders: &[EnterRider], ctx: &Ctx) -> String {
    let mut parts = Vec::new();
    for rider in riders {
        match rider {
            EnterRider::UnderOwnersControl => parts.push("under its owner's control".to_string()),
            EnterRider::UnderControlOf(who) => {
                parts.push(format!("under {}'s control", fragment::reference(who, ctx)));
            }
            EnterRider::WithCounters(kind, count) => {
                parts.push(format!("with {} on it", counter_phrase(kind, count)));
            }
            EnterRider::Tapped => parts.push("tapped".to_string()),
            EnterRider::FaceDown => parts.push("face down".to_string()),
            EnterRider::Attacking(_) => parts.push("attacking".to_string()),
        }
    }
    if parts.is_empty() { String::new() } else { format!(" {}", parts.join(" ")) }
}

/// A counter placement as its printed article + count noun phrase — "a
/// +1/+1 counter" / "two +1/+1 counters". `P1P1Counter`/`M1M1Counter`
/// (Undying/Persist's pip family) print their `+N/+N` symbol, exactly like
/// the card frame does; any other named kind ("a lore counter") uses the
/// plain word, same as `fragment`'s `counter_noun`.
pub(super) fn counter_phrase(kind: &deckmaste_core::CounterRef, count: &Count) -> String {
    let symbol = match kind.as_str() {
        "P1P1Counter" => "+1/+1".to_string(),
        "M1M1Counter" => "-1/-1".to_string(),
        other => other.trim_end_matches("Counter").to_lowercase(),
    };
    match count.literal_value() {
        Some(1) => format!("a {symbol} counter"),
        Some(n) => format!(
            "{} {symbol} counters",
            fragment::number_word(n).map_or_else(|| n.to_string(), str::to_string)
        ),
        None => format!("{} {symbol} counters", fragment::count(count)),
    }
}

/// A `Stat` axis's printed noun ([CR#208,209,210,202.3]) — "power",
/// "toughness", …
fn stat_word(s: Stat) -> &'static str {
    match s {
        Stat::Power => "power",
        Stat::Toughness => "toughness",
        Stat::ManaValue => "mana value",
        Stat::Loyalty => "loyalty",
        Stat::Defense => "defense",
    }
}

/// Render a divided distribution ([CR#601.2d]). The body selects the verb;
/// `group` is rendered as the set it divides among — an announced plural
/// target slot prints its announce phrase ("one, two, or three targets").
fn divide_among(d: &deckmaste_core::Distribute, ctx: &Ctx) -> String {
    let amount = fragment::count(&d.amount);
    let group = divided_group_phrase(&d.binder, ctx);
    match &*d.body {
        OneShotEffect::Act(Action::DealDamage(source, _, _)) => {
            let dealer = match source {
                Reference::This => ctx.subject.to_string(),
                other => fragment::reference(other, ctx),
            };
            format!(
                "{} deals {amount} damage divided as you choose among {group}.",
                capitalize_first(&dealer)
            )
        }
        OneShotEffect::Act(Action::By(_, PlayerAction::PutCounters(_, kind, _))) => {
            format!(
                "Distribute {amount} {} counters among {group}.",
                kind.as_str()
            )
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// The group a divided distribution names: an announced plural target slot
/// (read back as `They`) prints its announce phrase — "one,
/// two, or three targets" ([CR#601.2d]); anything else falls back to the
/// binder's own phrase.
fn divided_group_phrase(binder: &deckmaste_core::Binder, ctx: &Ctx) -> String {
    use deckmaste_core::Binder;
    use deckmaste_core::Selection;
    let slot = match binder {
        // The plural anaphor over a single announced slot reads that slot.
        Binder::Existing(Selection::They) if ctx.targets.len() == 1 => ctx.targets.first(),
        _ => None,
    };
    slot.and_then(fragment::announced_group_phrase)
        .unwrap_or_else(|| binder_phrase(binder, ctx))
}

/// The payment clause of an [`OneShotEffect::AdditionalCost`] ([CR#601.2f]): an
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
                for inner_comp in body {
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

/// An activated ability's printed cost line ([CR#602.1] — cost components
/// separated by commas, e.g. "{G}, Sacrifice a creature:"). A run of
/// symbol-only components (mana/tap/…) renders as ONE glued group via the
/// shared `render_cost` glyph renderer (so `{1}{T}` stays adjacent, never
/// comma-split mid-symbol-run); a verb component (`Do`/`With`) renders its
/// lowercased clause, exactly like [`additional_payment`]'s reader — the two
/// kinds of segment then join with ", ". No existing corpus card mixes a
/// symbol run with a verb component in an ACTIVATION cost yet (only
/// `AdditionalCost`'s printed-additional-cost clause did, which is why this
/// is a distinct function rather than a reuse of `additional_payment`: that
/// one's all-symbol case reads "pay {cost}", which is wrong here — an
/// activation cost line never says "pay").
/// The bracketed loyalty-cost prefix for a planeswalker loyalty ability's cost
/// verb ([CR#606.4], "the cost to activate a loyalty ability is to put on or
/// remove that many loyalty counters"): `PutCounters(This, LoyaltyCounter, n)`
/// prints `[+n]` (or `[0]` when `n == 0` — a `LoyaltyZero` ability), and
/// `RemoveCounters(This, LoyaltyCounter, n)` prints `[−n]` (the `−` is U+2212
/// MINUS SIGN, the glyph the printed card uses, not an ASCII hyphen). Keyed on
/// the `LoyaltyCounter` name and the `This` subject, mirroring the engine's
/// `is_loyalty_ability` discriminator — any other counter cost (a different
/// counter, or one on a non-`This` subject) returns `None` and renders through
/// the generic `player_action` clause. Only a bare literal count brackets; a
/// dynamic `−X` loyalty cost (out of scope) falls back to the generic render.
fn loyalty_cost_prefix(pa: &PlayerAction) -> Option<String> {
    let is_loyalty = |c: &deckmaste_core::CounterRef| c.as_str() == "LoyaltyCounter";
    match pa {
        PlayerAction::PutCounters(Reference::This, counter, count) if is_loyalty(counter) => {
            match count.literal_value()? {
                0 => Some("[0]".to_owned()),
                n => Some(format!("[+{n}]")),
            }
        }
        PlayerAction::RemoveCounters(Reference::This, counter, count) if is_loyalty(counter) => {
            Some(format!("[\u{2212}{}]", count.literal_value()?))
        }
        _ => None,
    }
}

pub(super) fn activated_cost(cost: &[deckmaste_core::CostComponent], ctx: &Ctx) -> String {
    use deckmaste_core::CostComponent;

    fn flush_symbol_run(run: &mut Vec<deckmaste_core::CostComponent>, parts: &mut Vec<String>) {
        if run.is_empty() {
            return;
        }
        parts.push(
            super::template::render_cost(run).unwrap_or_else(|| format!("[unrendered: {run:?}]")),
        );
        run.clear();
    }

    let mut parts = Vec::new();
    let mut symbol_run = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(_)
            | CostComponent::Tap
            | CostComponent::Untap
            | CostComponent::ManaCostOf(_)
            | CostComponent::TapTotal { .. } => symbol_run.push(component.clone()),
            CostComponent::Do(pa) => {
                flush_symbol_run(&mut symbol_run, &mut parts);
                // A planeswalker loyalty ability's cost is a `PutCounters`/
                // `RemoveCounters` of the `LoyaltyCounter` on `This`
                // ([CR#606.4]); it prints as the bracketed `[+N]`/`[−N]`/`[0]`
                // prefix, not the generic "put/remove … counter" clause.
                if let Some(prefix) = loyalty_cost_prefix(pa) {
                    parts.push(prefix);
                } else {
                    let phrase = trim_period(&player_action(pa, ctx));
                    // [CR#602.1]'s printed convention capitalizes each verb-cost
                    // segment ("{T}, Sacrifice a Goblin: ..."), unlike a body
                    // verb clause joined mid-sentence.
                    parts.push(capitalize_first(&phrase));
                }
            }
            // The same choose-then-pay reader `additional_payment` uses:
            // bind the binder's noun phrase as the body verbs' `That`
            // anaphor, then render the body's `Do` verbs — "Sacrifice a
            // creature".
            CostComponent::With { binder, body } => {
                flush_symbol_run(&mut symbol_run, &mut parts);
                let phrase = binder_phrase(binder, ctx);
                let inner = ctx.with_that(&phrase);
                for inner_comp in body {
                    match inner_comp {
                        CostComponent::Do(pa) => {
                            let p = trim_period(&player_action(pa, &inner));
                            parts.push(capitalize_first(&p));
                        }
                        other => parts.push(format!("[unrendered: {other:?}]")),
                    }
                }
            }
            // A macro-expanded cost component ("Pay {E}{E}") renders through its
            // own `template` — the printed cost text, e.g. the energy-repeat
            // construct — overriding the generic structural render of its
            // `value` ([CR#602.1]). The template already carries the segment's
            // capitalized verb ("Pay …"), so it is pushed verbatim; only a
            // template-less / unrenderable macro falls back.
            CostComponent::Expanded(e) => {
                flush_symbol_run(&mut symbol_run, &mut parts);
                match super::template::expanded(e, ctx.subject) {
                    Some(s) => parts.push(s),
                    None => parts.push(format!("[unrendered: {:?}]", e.value)),
                }
            }
            other @ CostComponent::Cost(_) => {
                flush_symbol_run(&mut symbol_run, &mut parts);
                parts.push(format!("[unrendered: {other:?}]"));
            }
        }
    }
    flush_symbol_run(&mut symbol_run, &mut parts);
    parts.join(", ")
}

/// The phrase an `AdditionalCost` body's `EventObject` anaphor should read,
/// derived from the payment itself ([CR#601.2f], "the sacrificed creature's
/// power", Fling) — the cost-side twin of `With`'s binder-phrase threading.
/// `None` for any payment shape besides the bare single sacrifice (the
/// `EventObject` render then falls back to the plain "it").
fn additional_cost_object_phrase(cost: &[deckmaste_core::CostComponent]) -> Option<String> {
    use deckmaste_core::Binder;
    use deckmaste_core::CostComponent;
    if let [CostComponent::With { binder, body }] = cost
        && let Binder::ChooseOne { filter, .. } = binder.as_ref()
        && let [CostComponent::Do(pa)] = body.0.as_slice()
        && let PlayerAction::Sacrifice(Reference::That(_)) = pa.as_ref()
    {
        return Some(format!("the sacrificed {}", fragment::filter_noun(filter)));
    }
    None
}

/// "a card" / "three cards" / "N cards" — the counted-cards object a mill/draw
/// phrase takes ([CR#701.17a,121.1]). Literal 1 is "a card"; small literals
/// spell out; a dynamic count renders through [`fragment::count`].
fn counted_cards(c: &Count) -> String {
    match c.literal_value() {
        Some(1) => "a card".to_owned(),
        Some(n) => match fragment::number_word(n) {
            Some(word) => format!("{word} cards"),
            None => format!("{n} cards"),
        },
        None => format!("{} cards", fragment::count(c)),
    }
}

/// The imperative you-form of mill ([CR#701.17a]) — "Mill a card." / "Mill
/// three cards." / "Mill X cards." The `You`-performer render of a
/// `Composite(Mill(You, n), …)`.
fn mill_imperative(c: &Count) -> String {
    match c {
        Count::Literal(1) => "Mill a card.".to_string(),
        Count::Literal(n) => match fragment::number_word(*n) {
            Some(word) => format!("Mill {word} cards."),
            None => format!("Mill {n} cards."),
        },
        c => format!("Mill {} cards.", fragment::count(c)),
    }
}

/// The imperative you-form of draw ([CR#121.1]) — "Draw a card." / "Draw three
/// cards." / "Draw X cards." The `You`-performer render of a
/// `Composite(Draw(You, n), …)`.
fn draw_imperative(c: &Count) -> String {
    match c {
        Count::Literal(1) => "Draw a card.".to_string(),
        Count::Literal(n) => match fragment::number_word(*n) {
            Some(word) => format!("Draw {word} cards."),
            None => format!("Draw {n} cards."),
        },
        c => format!("Draw {} cards.", fragment::count(c)),
    }
}

/// The THIRD-PERSON verb phrase of a player action — the declarative-subject
/// tail ("mills two cards", "loses 2 life") a non-`You` `By` agent or an
/// `Each` player loop prefixes with its subject. A remembered verb-macro
/// expansion (`Mills(2)`) renders through its own template; the core verbs
/// carry structural fallbacks. `None` = no third-person phrase (the caller
/// falls back to the imperative render).
fn third_person_verb_phrase(pa: &PlayerAction) -> Option<String> {
    match pa {
        PlayerAction::Expanded(e) => {
            super::template::expanded(e, "it").or_else(|| third_person_verb_phrase(&e.value))
        }
        PlayerAction::Discard {
            count,
            what: None,
            random: false,
        } => Some(format!("discards {}", counted_cards(count))),
        PlayerAction::LoseLife(c) => Some(format!("loses {} life", fragment::count(c))),
        PlayerAction::GainLife(c) => Some(format!("gains {} life", fragment::count(c))),
        // Dictate of Karametra's "that land's controller adds one mana of
        // any type that land produced" — reuse `add_mana_text`'s clause,
        // stripped of its imperative "Add "/trailing period. A shape
        // `add_mana_text` itself declines (`[unrendered: …]`) has no
        // "Add "/"." to strip, so this falls through to `None` (the caller's
        // imperative fallback) rather than fabricating a phrase.
        PlayerAction::AddMana(count, production) => {
            let imperative = add_mana_text(count, production);
            imperative
                .strip_prefix("Add ")
                .and_then(|s| s.strip_suffix('.'))
                .map(|s| format!("adds {s}"))
        }
        _ => None,
    }
}

fn player_action(pa: &PlayerAction, ctx: &Ctx) -> String {
    match pa {
        // A remembered verb-macro expansion (`Mills(2)` under an explicit
        // `By`): the imperative frame renders the expanded CORE action —
        // the third-person template belongs to the declarative subjects.
        PlayerAction::Expanded(e) => player_action(&e.value, ctx),
        // Life totals move in digits, with the explicit "you" subject the
        // oracle prints ("You gain 2 life.").
        PlayerAction::GainLife(c) => format!("You gain {} life.", fragment::count(c)),
        PlayerAction::LoseLife(c) => format!("You lose {} life.", fragment::count(c)),
        // A mana ability's production ([CR#106.1]): "Add {W}.", "Add
        // {C}{C}.", "Add one mana of any color."
        PlayerAction::AddMana(count, production) => add_mana_text(count, production),
        // Rider-carrying token creation ("tapped and attacking") falls back
        // to the structural form until its surface lands (macro-first-wave).
        PlayerAction::Create(count, spec, riders) if riders.is_empty() => create_text(count, spec),
        PlayerAction::Tap(r) => format!("Tap {}.", fragment::reference(r, ctx)),
        PlayerAction::Untap(r) => format!("Untap {}.", fragment::reference(r, ctx)),
        // A sacrifice ([CR#701.21]) — the patient is a single reference. A
        // chosen permanent ("sacrifice a creature", Fling) arrives pre-bound as
        // `Reference::That` from an enclosing `With`, which supplies the phrase.
        PlayerAction::Sacrifice(r) => format!("Sacrifice {}.", fragment::reference(r, ctx)),
        // Discard ([CR#701.9]): a named card via `what` (e.g. the `With`-bound
        // anaphor), else `count` cards chosen from hand; `random` appends the
        // "at random" qualifier ([CR#701.9b]).
        PlayerAction::Discard { what: Some(r), .. } => {
            format!("Discard {}.", fragment::reference(r, ctx))
        }
        PlayerAction::Discard {
            count: Count::Literal(1),
            what: None,
            random,
        } => {
            if *random {
                "Discard a card at random.".to_string()
            } else {
                "Discard a card.".to_string()
            }
        }
        PlayerAction::Discard {
            count,
            what: None,
            random,
        } => {
            let suffix = if *random { " at random" } else { "" };
            format!("Discard {} cards{suffix}.", fragment::count(count))
        }
        // A player-performed relocation ([CR#400.7]). Exiling is a pure zone
        // move ([CR#701.13]) — "Exile X."; a library destination mirrors
        // `Action::Move`'s "Put X on top/the bottom of your library."
        PlayerAction::Move(r, Destination::Zone(Zone::Exile), riders) if riders.is_empty() => {
            format!("Exile {}.", fragment::reference(r, ctx))
        }
        PlayerAction::Move(r, Destination::Library(anchor), riders) if riders.is_empty() => {
            format!(
                "Put {} on {} of your library.",
                fragment::reference(r, ctx),
                fragment::library_position(anchor),
            )
        }
        PlayerAction::GetDesignation(name) if name.as_ref() == "CitysBlessing" => {
            "You get the city's blessing.".to_string()
        }
        PlayerAction::GetDesignation(name) => format!("You get {name}."),
        // [CR#114.1]: "You get an emblem with «ability»." The emblem carries
        // only its abilities ([CR#114.3]) — render them through the same
        // `rules` walk a card face uses (a nameless, typeless view), quoted as
        // the emblem's text.
        PlayerAction::GetEmblem(abilities) => {
            let view = super::CardView {
                name: "",
                mana_cost: None,
                supertypes: &[],
                types: &[],
                subtypes: &[],
                power: None,
                toughness: None,
                abilities,
            };
            format!(
                "You get an emblem with \"{}\".",
                super::rules(&view).join(" ")
            )
        }
        // [CR#701.19a]: remove all damage as part of regeneration.
        PlayerAction::RemoveDamage(r) => {
            format!("Remove all damage from {}.", fragment::reference(r, ctx))
        }
        // [CR#122.1]: "Put a luck counter on this enchantment." — reuses the
        // same `counter_phrase` the `Distribute`/keyword-counter renderers
        // already share.
        PlayerAction::PutCounters(r, kind, count) => format!(
            "Put {} on {}.",
            counter_phrase(kind, count),
            fragment::reference(r, ctx),
        ),
        // [CR#608.2g]: "Cast that card." — the resolution-time cast verb. The
        // enclosing `May` supplies the "You may "/"If you don't, …" framing
        // (Chandra's "You may cast that card."); this renders the bare
        // instruction, its patient the surrounding effect's anaphor.
        PlayerAction::Cast(r) => format!("Cast {}.", fragment::reference(r, ctx)),
        // [CR#104.2b]: "You win the game." — immediate on resolution; the
        // `CantWin` suppression is engine-side, not part of the sentence.
        PlayerAction::WinGame => "You win the game.".to_string(),
        // [CR#104.3e]: the loss twin of `WinGame`.
        PlayerAction::LoseGame => "You lose the game.".to_string(),
        other => format!("[unrendered: {other:?}]."),
    }
}

/// [CR#608.2d]: "You may [effect]." with the optional "If you do, …" / "If you
/// don't, …" riders (Chandra's "You may cast that card. If you don't, ~ deals 2
/// damage to each opponent."). Each rider is a full sentence whose subject the
/// inner effect supplies, so it stands capitalized after the base clause.
fn render_may(m: &deckmaste_core::May, ctx: &Ctx) -> String {
    use std::fmt::Write as _;
    let inner = super::ability::lower_first(&trim_period(&effect(&m.effect, ctx)));
    let mut out = format!("You may {inner}.");
    if let Some(did) = &m.if_did {
        let did = super::ability::lower_first(&trim_period(&effect(did, ctx)));
        let _ = write!(out, " If you do, {did}.");
    }
    if let Some(not) = &m.if_not {
        let not = trim_period(&effect(not, ctx));
        let _ = write!(out, " If you don't, {not}.");
    }
    out
}

/// "Add {W}." / "Add {C}{C}." / "Add one mana of any color." / "Add {W} or
/// {U}." — a mana ability's production ([CR#106.1]). Riders and dynamic
/// counts fall back to the structural form.
fn add_mana_text(count: &Count, production: &deckmaste_core::ManaProduction) -> String {
    use deckmaste_core::ManaProduction;
    use deckmaste_core::ManaSpec;
    let ManaProduction::Bare(spec) = production else {
        return format!("[unrendered: AddMana({count:?}, {production:?})].");
    };
    let Some(n) = count.literal_value() else {
        return format!("[unrendered: AddMana({count:?}, {production:?})].");
    };
    match spec {
        ManaSpec::Specific(c) => {
            let symbol = format!("{{{}}}", super::card::color_letter(*c));
            format!("Add {}.", symbol.repeat(n as usize))
        }
        ManaSpec::AnyColor => {
            let amount = if n == 1 {
                "one mana of any color".to_string()
            } else {
                let word = fragment::number_word(n).map_or_else(|| n.to_string(), str::to_string);
                format!("{word} mana of any one color")
            };
            format!("Add {amount}.")
        }
        ManaSpec::OneOf(choices) => {
            let symbols: Vec<String> = choices
                .iter()
                .map(|c| format!("{{{}}}", super::card::color_letter(*c)))
                .collect();
            format!("Add {}.", symbols.join(" or "))
        }
        // The filterland cycle: a choice among multi-symbol runs, rendered as
        // the Oxford-comma list "Add {W}{W}, {W}{U}, or {U}{U}." Each run is
        // the concatenation of its symbols; a two-run choice is a bare "A or
        // B", three or more the Oxford "A, B, or C".
        ManaSpec::OneOfRuns(runs) => {
            let options: Vec<String> = runs
                .iter()
                .map(|run| {
                    run.iter()
                        .map(|c| format!("{{{}}}", super::card::color_letter(*c)))
                        .collect::<Vec<_>>()
                        .concat()
                })
                .collect();
            let list = match options.as_slice() {
                [] => String::new(),
                [only] => only.clone(),
                [a, b] => format!("{a} or {b}"),
                [rest @ .., last] => format!("{}, or {last}", rest.join(", ")),
            };
            format!("Add {list}.")
        }
        // [CR#106.12a]: "add one mana of any type that land produced"
        // (Dictate of Karametra, Vorinclex) — only the sound amount=1 shape
        // is rendered; the corpus has no multi-copy `ProducedByEvent` card
        // yet, so a larger count declines structurally rather than guessing
        // plural wording.
        ManaSpec::ProducedByEvent if n == 1 => {
            "Add one mana of any type that land produced.".to_string()
        }
        // `AmongColorsOf` needs the referenced object's own phrase (Chrome
        // Mox's "the exiled card", Katilda's "this creature") — no fixture
        // in this corpus exercises it yet (both real candidates hit an
        // unbuilt gap, see the ticket's completion notes), so it declines
        // structurally rather than guessing a phrase.
        ManaSpec::AmongColorsOf(_) | ManaSpec::ProducedByEvent => {
            format!("[unrendered: AddMana({count:?}, {production:?})].")
        }
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
        parts.push(super::card::type_str(ty).to_lowercase());
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
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
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
        let draw = || {
            Box::new(OneShotEffect::Act(Action::draw(
                Reference::You,
                Count::Literal(1),
            )))
        };
        let lose = || {
            Box::new(OneShotEffect::act_by_you(PlayerAction::LoseLife(
                Count::Literal(1),
            )))
        };

        // -- MayPay: "[payer] may pay {1}. If [payer] do(es), draw a card; if
        //    [payer] do(esn't), [lose]." --
        let may_you = OneShotEffect::MayPay(MayPay {
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

        let may_them = OneShotEffect::MayPay(MayPay {
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
        let must_you = OneShotEffect::MustPay(MustPay {
            actor: Reference::You,
            cost: one(),
            or_else: lose(),
        });
        let rendered = effect(&must_you, &ctx);
        assert!(
            rendered.contains("unless you pay {1}") && !rendered.contains("unless you pays"),
            "second-person MustPay: {rendered}"
        );

        let must_them = OneShotEffect::MustPay(MustPay {
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

    /// A planeswalker loyalty ability's activation cost ([CR#606.4]) prints as
    /// the bracketed prefix `[+N]` / `[−N]` / `[0]` (the `−` is U+2212 MINUS
    /// SIGN), NOT the generic "put/remove a loyalty counter on ~" clause: the
    /// cost verb is `Do(PutCounters/RemoveCounters(This, LoyaltyCounter, N))`.
    /// A non-loyalty counter cost (or one on a non-`This` subject) keeps
    /// rendering generically — the loyalty prefix must not over-broaden the
    /// match.
    #[test]
    fn loyalty_cost_renders_bracketed_prefix() {
        use deckmaste_core::CostComponent;
        use deckmaste_core::CounterRef;
        use deckmaste_core::PlayerAction;

        let ctx = Ctx {
            subject: "Jace Beleren",
            targets: &[],
            that: None,
        };
        let cost = |pa: PlayerAction| super::activated_cost(&[CostComponent::do_(pa)], &ctx);
        let loyalty = || CounterRef::from("LoyaltyCounter");

        // [+2]: PutCounters(This, LoyaltyCounter, 2)
        assert_eq!(
            cost(PlayerAction::PutCounters(
                Reference::This,
                loyalty(),
                Count::Literal(2),
            )),
            "[+2]",
        );
        // [−1]: RemoveCounters(This, LoyaltyCounter, 1) — U+2212
        assert_eq!(
            cost(PlayerAction::RemoveCounters(
                Reference::This,
                loyalty(),
                Count::Literal(1),
            )),
            "[\u{2212}1]",
        );
        // [−10]: RemoveCounters(This, LoyaltyCounter, 10) — U+2212
        assert_eq!(
            cost(PlayerAction::RemoveCounters(
                Reference::This,
                loyalty(),
                Count::Literal(10),
            )),
            "[\u{2212}10]",
        );
        // [0]: PutCounters(This, LoyaltyCounter, 0) — a zero-cost loyalty
        // ability (LoyaltyZero) prints "[0]", not "[+0]".
        assert_eq!(
            cost(PlayerAction::PutCounters(
                Reference::This,
                loyalty(),
                Count::Literal(0),
            )),
            "[0]",
        );

        // Regression: a NON-loyalty counter cost still renders generically —
        // the loyalty prefix must key on the "LoyaltyCounter" name.
        let generic = cost(PlayerAction::PutCounters(
            Reference::This,
            CounterRef::from("P1P1Counter"),
            Count::Literal(1),
        ));
        assert!(
            !generic.starts_with('['),
            "non-loyalty counter cost must not get a bracketed loyalty prefix: {generic}"
        );
    }

    /// A mana ability's produced-mana forms render their oracle text: the
    /// single-color choice "Add {W} or {U}.", and the filterland multi-symbol
    /// run choice "Add {W}{W}, {W}{U}, or {U}{U}." ([CR#106.1b]).
    #[test]
    fn add_mana_renders_run_and_color_choices() {
        use deckmaste_core::Color::Blue;
        use deckmaste_core::Color::White;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;
        use deckmaste_core::PlayerAction;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let render = |spec: ManaSpec| {
            effect(
                &OneShotEffect::act_by_you(PlayerAction::AddMana(Count::Literal(1), spec.into())),
                &ctx,
            )
        };
        let w = || ColorOrColorless::Color(White);
        let u = || ColorOrColorless::Color(Blue);
        assert_eq!(render(ManaSpec::OneOf(vec![w(), u()])), "Add {W} or {U}.");
        assert_eq!(
            render(ManaSpec::OneOfRuns(vec![
                vec![w(), w()],
                vec![w(), u()],
                vec![u(), u()],
            ])),
            "Add {W}{W}, {W}{U}, or {U}{U}."
        );
        // The two-run form drops the Oxford comma.
        assert_eq!(
            render(ManaSpec::OneOfRuns(vec![vec![w(), w()], vec![u(), u()]])),
            "Add {W}{W} or {U}{U}."
        );
    }

    /// Damage always names its source: the default `This` source reads as
    /// the carrier ("Pouncer deals N damage to X" — "it deals …" inside a
    /// trigger body, where the ctx subject is "it"); an explicit non-`This`
    /// source names the dealer (the fight / redirected-damage surface).
    #[test]
    fn deal_damage_source_renders_dealer_phrase() {
        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "Pouncer",
            targets: std::slice::from_ref(&target),
            that: None,
        };

        let default = Action::deal_damage(Reference::It, Count::Literal(3));
        assert_eq!(
            action(&default, &ctx),
            "Pouncer deals 3 damage to target creature."
        );

        let sourced = Action::DealDamage(Reference::It, Count::Literal(3), Reference::It);
        assert_eq!(
            action(&sourced, &ctx),
            "Target creature deals 3 damage to target creature."
        );
    }

    /// The one-sided "bite" shape's render round-trip ([CR#120]; `Fight`'s
    /// [CR#701.14a] reciprocal-less half): `StatOf(This, Power)` prints "…
    /// equal to its power", faithfully round-tripping
    /// [`crate::parsers::effect`]'s `deal_damage_bite_equal_to_its_power`
    /// parse ("~ deals damage equal to its power to target creature." — the
    /// corpus's actual word order: the variable-amount clause sits between
    /// "damage" and "to <target>", unlike a literal numeral). Regression for
    /// the ungrammatical "it's power" a naive `reference(This, ctx) + "'s"`
    /// composition would print inside a triggered/activated body (ctx
    /// subject "it").
    #[test]
    fn deal_damage_stat_of_this_power_renders_its_power() {
        use deckmaste_core::Stat;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
        };
        let bite = Action::DealDamage(
            Reference::This,
            Count::StatOf(Reference::This, Stat::Power),
            Reference::It,
        );
        assert_eq!(
            action(&bite, &ctx),
            "It deals damage equal to its power to target creature."
        );
    }

    /// A `This`-power amount is rendered the same possessive at a spell root
    /// (ctx subject = the card name), never repeating the card's own name a
    /// second time ("Cinder Shade deals damage equal to its power to X.",
    /// never "… equal to Cinder Shade's power").
    #[test]
    fn deal_damage_stat_of_this_power_at_spell_root() {
        use deckmaste_core::Stat;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "Cinder Shade",
            targets: std::slice::from_ref(&target),
            that: None,
        };
        let bite = Action::DealDamage(
            Reference::This,
            Count::StatOf(Reference::This, Stat::Power),
            Reference::It,
        );
        assert_eq!(
            action(&bite, &ctx),
            "Cinder Shade deals damage equal to its power to target creature."
        );
    }

    /// A non-`This` stat reference keeps the generic possessive composition
    /// (an "equal to <other>'s power" amount, e.g. Fling-style "the sacrificed
    /// creature's power") — only the `This` case gets the hardcoded "its"
    /// pronoun. (This pins the non-`This` branch with a `That(Card)` anaphor
    /// rendering as "the sacrificed creature"; it is representative of that
    /// shape, not Fling's exact `EventObject`/`Target` encoding.)
    #[test]
    fn deal_damage_stat_of_other_reference_keeps_generic_possessive() {
        use deckmaste_core::Stat;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: Some("the sacrificed creature"),
        };
        let other_ref = Action::DealDamage(
            Reference::This,
            Count::StatOf(Reference::That(deckmaste_core::Sort::Card), Stat::Power),
            Reference::EventActor,
        );
        assert_eq!(
            action(&other_ref, &ctx),
            "It deals damage equal to the sacrificed creature's power to that player."
        );
    }

    /// A `Move`-to-library destination renders the anchor: `FromTop(0)` ->
    /// "top", `FromBottom(0)` -> "the bottom".
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
            vec![],
        );
        assert_eq!(action(&top, &ctx), "Put it on top of your library.");
        let bottom = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![],
        );
        assert_eq!(
            action(&bottom, &ctx),
            "Put it on the bottom of your library."
        );
    }

    /// Bounce-to-library — the migrations `parse_bounce_to_library`
    /// production's TARGETED `Move(It, Library(anchor))` shape round-trips
    /// through the SAME generic library-destination arm as the self form
    /// above, reading the announced target's phrase via `Reference::It`
    /// (mirroring `parse_return_to_hand`'s battlefield-bounce shape).
    ///
    /// KNOWN LIMITATION (not the desired final output): both "its owner's
    /// library" and "your library" input phrasings collapse onto this one
    /// canonical "your library" render, same as `Move(_, Hand)`'s
    /// "your hand"/"its owner's hand" idiom pair. That is WRONG for a
    /// targeted, possibly-opponent-owned creature (oracle says "its owner's
    /// library"), so most real bounce-to-library cards fail the byte-exact
    /// fidelity gate and do not graduate until an owner-relative render arm
    /// lands — tracked in the `bounce-followups` ticket (§1). This test pins
    /// the current collapse behavior, not a blessed target output.
    #[test]
    fn move_to_library_renders_targeted_top_and_bottom() {
        use deckmaste_core::Anchor;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
        };
        let top = Action::Move(
            Reference::It,
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
            vec![],
        );
        assert_eq!(
            action(&top, &ctx),
            "Put target creature on top of your library."
        );
        let bottom = Action::Move(
            Reference::It,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![],
        );
        assert_eq!(
            action(&bottom, &ctx),
            "Put target creature on the bottom of your library."
        );
    }

    /// Graveyard reanimation ([CR#400.7]) — the empty-rider `Move(_,
    /// Battlefield)` shape `parse_reanimate` (migrations effect.rs) emits —
    /// round-trips both the targeted and self forms. The targeted form
    /// exercises `filter_noun`'s graveyard-card "card" noun (both typed and
    /// bare); a rider-carrying return (Otherworldly Journey's delayed
    /// return) stays on the OTHER arm and never gets the "from your
    /// graveyard" clause.
    #[test]
    fn reanimate_from_graveyard_round_trips() {
        use deckmaste_core::EnterRider;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;

        let graveyard_creature = Predicate::And(vec![
            Predicate::creature(),
            Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
            Predicate::Relation(RelationPredicate::Owner(Box::new(Predicate::Ref(
                Reference::You,
            )))),
        ]);
        let target = TargetSpec::Target(Quantity::one(), graveyard_creature);
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
        };
        let targeted = Action::Move(Reference::It, Destination::Zone(Zone::Battlefield), vec![]);
        assert_eq!(
            action(&targeted, &ctx),
            "Return target creature card from your graveyard to the battlefield."
        );

        // Bare "card" (no type qualifier).
        let bare_card = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
            Predicate::Relation(RelationPredicate::Owner(Box::new(Predicate::Ref(
                Reference::You,
            )))),
        ]);
        let bare_target = TargetSpec::Target(Quantity::one(), bare_card);
        let bare_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&bare_target),
            that: None,
        };
        assert_eq!(
            action(&targeted, &bare_ctx),
            "Return target card from your graveyard to the battlefield."
        );

        // Self-reanimation, no target.
        let self_ctx = Ctx {
            subject: "Ashputtle",
            targets: &[],
            that: None,
        };
        let self_move = Action::Move(
            Reference::This,
            Destination::Zone(Zone::Battlefield),
            vec![],
        );
        assert_eq!(
            action(&self_move, &self_ctx),
            "Return Ashputtle from your graveyard to the battlefield."
        );

        // A rider-carrying battlefield return is a DIFFERENT family (exile,
        // not graveyard) and keeps its own phrasing, no "from your
        // graveyard" clause.
        let riders = Action::Move(
            Reference::That(deckmaste_core::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![EnterRider::UnderOwnersControl],
        );
        assert_eq!(
            action(&riders, &self_ctx),
            "Return that card to the battlefield under its owner's control."
        );

        // Regression guard: a RIDERLESS `Move(That(Card), Battlefield)` is the
        // exile-return family's no-adjunct form (`parse_return_that_card`), NOT
        // graveyard reanimation. The reanimation arm is guarded to `It`/`This`,
        // so this must NOT be mislabelled "from your graveyard".
        let riderless_that = Action::Move(
            Reference::That(deckmaste_core::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![],
        );
        assert!(
            !action(&riderless_that, &self_ctx).contains("from your graveyard"),
            "riderless That(Card) reanimation-arm leak: {}",
            action(&riderless_that, &self_ctx)
        );
    }

    /// `MoveCounters` renders the `AllKinds` and named-kind forms ([CR#122]);
    /// `from`/`to` resolve through `ctx.targets`.
    #[test]
    fn move_counters_renders_all_kinds_and_named() {
        use deckmaste_core::CounterRef;
        use deckmaste_core::CounterSpec;

        let slot = || TargetSpec::Target(Quantity::one(), Predicate::creature());
        let targets = [slot(), slot()];
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

    /// `Distribute` renders by its body: a `DealDamage` body -> "Deal N damage
    /// divided as you choose among <group>" ([CR#601.2d]).
    #[test]
    fn divide_among_renders_divided_damage() {
        use deckmaste_core::Distribute;
        use deckmaste_core::Predicate;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let divide = super::effect(
            &deckmaste_core::OneShotEffect::Distribute(Distribute {
                amount: Count::Literal(3),
                binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
                body: Box::new(deckmaste_core::OneShotEffect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Allotment,
                ))),
            }),
            &ctx,
        );
        assert_eq!(
            divide,
            "It deals 3 damage divided as you choose among each creature."
        );
    }

    /// `AdditionalCost` renders the printed clause: a chosen-creature sacrifice
    /// cost reads "As an additional cost to cast ~, sacrifice a creature."
    /// followed by the body sentence ([CR#601.2f,118.8]).
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
            &deckmaste_core::OneShotEffect::AdditionalCost(AdditionalCost {
                // "sacrifice a creature" is now the choose-then-pay `With` cost
                // step: ChooseOne(Creature) binds `That`, then `Sacrifice(That)`.
                pay: Cost(vec![CostComponent::With {
                    binder: Box::new(Binder::ChooseOne {
                        filter: Predicate::creature(),
                        by: Reference::You,
                    }),
                    body: Cost(vec![CostComponent::do_(PlayerAction::Sacrifice(
                        Reference::That(deckmaste_core::Sort::OfType(
                            deckmaste_core::Type::Creature,
                        )),
                    ))]),
                }]),
                body: Box::new(deckmaste_core::OneShotEffect::Act(
                    deckmaste_core::Action::draw(deckmaste_core::Reference::You, Count::Literal(1)),
                )),
            }),
            &ctx,
        );
        assert_eq!(
            fling,
            "As an additional cost to cast Fling, sacrifice a creature.\nDraw a card."
        );
    }

    /// `OneShotEffect::With` binds the binder's noun phrase as the body's
    /// `That` anaphor ([CR#601.2b]): a `ChooseOne` one-binder renders
    /// "Sacrifice a creature." — the choose-then-act surface that replaced
    /// the old verb-patient `Choose`.
    #[test]
    fn with_choose_one_renders_sacrifice_a_creature() {
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "Altar",
            targets: &[],
            that: None,
        };
        let with = OneShotEffect::With(With {
            binder: Binder::ChooseOne {
                filter: Predicate::creature(),
                by: Reference::You,
            },
            body: Box::new(OneShotEffect::act_by_you(PlayerAction::Sacrifice(
                Reference::That(deckmaste_core::Sort::OfType(deckmaste_core::Type::Creature)),
            ))),
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
        let with = OneShotEffect::With(With {
            binder: Binder::Choose {
                quantity: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                filter: Predicate::Kind(ObjectKind::Card),
                by: Reference::You,
            },
            body: Box::new(OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(2),
                what: Some(Reference::That(deckmaste_core::Sort::Card)),
                random: false,
            })),
        });
        assert_eq!(effect(&with, &ctx), "Discard two cards.");
    }

    /// The loot/rummage render round-trip ([CR#121.1,701.9b,608.2c]): the
    /// `Sequentially([Draw, Discard])` shape the migrations parser's
    /// `parse_draw_then_discard` production emits needs NO dedicated render
    /// arm — the existing `Sequentially` ", then" joiner (this file's
    /// `effect` match arm above) already renders it back to the exact oracle
    /// sentence, both orders (loot and rummage).
    #[test]
    fn sequentially_renders_loot_and_rummage() {
        use deckmaste_core::PlayerAction;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let draw = || OneShotEffect::Act(Action::draw(Reference::You, Count::Literal(1)));
        let discard = || {
            OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(1),
                what: None,
                random: false,
            })
        };
        let loot = OneShotEffect::Sequentially(vec![draw(), discard()]);
        assert_eq!(effect(&loot, &ctx), "Draw a card, then discard a card.");
        let rummage = OneShotEffect::Sequentially(vec![discard(), draw()]);
        assert_eq!(effect(&rummage, &ctx), "Discard a card, then draw a card.");
    }

    /// `OneShotEffect::Each` over a many-binder collapses a single group-verb
    /// body (acting on the per-element `It`) to the collective surface —
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
        let destroy = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
            effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
        });
        assert_eq!(effect(&destroy, &ctx), "Destroy each creature.");
        // A body the collapse does not recognise → the per-element form.
        let gain = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
            effect: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                Count::Literal(1),
            ))),
        });
        assert_eq!(effect(&gain, &ctx), "For each creature, you gain 1 life.");
    }

    /// The damage-sweeper family's own macro
    /// (`plugins/builtin/macros/effect/DealsDamageToEach.ron`), loaded
    /// through the REAL plugin macro set — the migrations parser
    /// (`parse_deal_damage`/`damage_target`) emits this invocation, so this
    /// is the render half of the parse⇄render round trip: a macro-shorthand
    /// filter argument (`Creature`) doesn't parse through the template
    /// layer's bare core reader, so this exercises the "template fill
    /// declines, fall back to structural" path through
    /// [`each_collective`](super::each_collective); a bare-core-parseable
    /// filter (`OpponentOf(Ref(You))`) exercises the template-fill path
    /// directly. Either way the printed sentence must read back faithfully —
    /// a shape with no render arm would silently print "[unrendered: ...]"
    /// and pass every parser-only test.
    #[test]
    fn deals_damage_to_each_round_trips_render() {
        use std::path::Path;

        use crate::plugin::Plugin;

        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let ctx = Ctx {
            subject: "~",
            targets: &[],
            that: None,
        };

        let creature: OneShotEffect = plugin
            .macros
            .read_str("DealsDamageToEach(2, Creature)")
            .expect("expands");
        assert_eq!(
            effect(&creature, &ctx),
            "~ deals 2 damage to each creature."
        );

        let opponent: OneShotEffect = plugin
            .macros
            .read_str("DealsDamageToEach(1, OpponentOf(Ref(You)))")
            .expect("expands");
        assert_eq!(
            effect(&opponent, &ctx),
            "~ deals 1 damage to each opponent."
        );

        // The filtered recipient the migrations parser's new "each <subject>"
        // fallback arm adds ([CR#608.2d]; [CR#102.2]): "each creature your
        // opponents control".
        let filtered: OneShotEffect = plugin
            .macros
            .read_str("DealsDamageToEach(3, And([Creature, ControlledBy(OpponentOf(Ref(You)))]))")
            .expect("expands");
        assert_eq!(
            effect(&filtered, &ctx),
            "~ deals 3 damage to each creature an opponent controls."
        );
    }

    /// [CR#119.1,119.5]: "Each player's life total becomes N." — the
    /// possessive-subject collective, Arbiter of Knollridge's own shape
    /// (`N` = a cross-player `Aggregate` reading "the highest life total
    /// among all players").
    #[test]
    fn each_player_set_life_renders_possessive_becomes_clause() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Countable;
        use deckmaste_core::ObjectKind;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::PlayerAttr;
        use deckmaste_core::Projection;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        let highest_life = Count::Aggregate(
            AggregateOp::MaxOf,
            Projection {
                of: Countable::Players(Box::new(Predicate::Kind(ObjectKind::Player))),
                by: Box::new(Count::PlayerStatOf(Reference::It, PlayerAttr::Life)),
            },
        );
        let set_life = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
            effect: Box::new(OneShotEffect::Act(Action::By(
                Reference::It,
                PlayerAction::SetLife(highest_life),
            ))),
        });
        assert_eq!(
            effect(&set_life, &ctx),
            "Each player's life total becomes the highest life total among all players."
        );
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
            vec![],
        ));
        assert_eq!(action(&exile, &ctx), "Exile Scavenger.");
    }

    /// Graveyard-hate exile ([CR#701.13a]/[CR#400.7]) — the migrations
    /// `parse_exile_target` production's `any_graveyard_card_filter` shape —
    /// round-trips with the "from a graveyard" clause appended, both typed
    /// and bare "card"; the plain (non-graveyard) exile production, sharing
    /// the identical `Move(It, Exile)` shape, must NOT pick up the clause.
    #[test]
    fn exile_target_card_from_a_graveyard_round_trips() {
        use deckmaste_core::StatePredicate;

        let graveyard_creature = Predicate::And(vec![
            Predicate::creature(),
            Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
        ]);
        let target = TargetSpec::Target(Quantity::one(), graveyard_creature);
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
        };
        let exiled = Action::Move(Reference::It, Destination::Zone(Zone::Exile), vec![]);
        assert_eq!(
            action(&exiled, &ctx),
            "Exile target creature card from a graveyard."
        );

        // Bare "card" (no type qualifier).
        let bare_card = Predicate::State(StatePredicate::InZone(Zone::Graveyard));
        let bare_target = TargetSpec::Target(Quantity::one(), bare_card);
        let bare_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&bare_target),
            that: None,
        };
        assert_eq!(
            action(&exiled, &bare_ctx),
            "Exile target card from a graveyard."
        );

        // Regression guard: the plain "Exile target <subject>." production
        // shares this exact `Move(It, Exile)` shape for a NON-graveyard
        // filter — must keep its own phrasing, no "from a graveyard" clause.
        let plain_target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let plain_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&plain_target),
            that: None,
        };
        assert_eq!(action(&exiled, &plain_ctx), "Exile target creature.");
    }

    /// "Shuffle your graveyard into your library." ([CR#701.24a]/[CR#400.7])
    /// — the builtin `ShuffleYourGraveyardIntoLibrary` macro invocation
    /// actually rendered (not just structurally pinned): loaded through the
    /// real plugin/macro registry, so this exercises the template-first
    /// render path a bare parser-test pin would silently skip.
    #[test]
    fn shuffle_your_graveyard_into_your_library_renders_via_template() {
        use std::path::Path;

        use crate::plugin::Plugin;

        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let parsed: OneShotEffect = plugin
            .macros
            .read_str("ShuffleYourGraveyardIntoLibrary")
            .unwrap();
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
        };
        assert_eq!(
            effect(&parsed, &ctx),
            "Shuffle your graveyard into your library."
        );
    }
}
