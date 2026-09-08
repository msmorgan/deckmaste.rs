//! Effects / actions render to imperative sentences (spell mood).

use std::cell::Cell;
use std::fmt::Write as _;

use deckmaste_semantics::Ability;
use deckmaste_semantics::Action;
use deckmaste_semantics::Arrangement;
use deckmaste_semantics::Binder;
use deckmaste_semantics::Characteristic;
use deckmaste_semantics::CharacteristicPredicate;
use deckmaste_semantics::CollectionOp;
use deckmaste_semantics::Color;
use deckmaste_semantics::CopyException;
use deckmaste_semantics::CopySource;
use deckmaste_semantics::Count;
use deckmaste_semantics::CounterSpec;
use deckmaste_semantics::Deontic;
use deckmaste_semantics::DeonticAction;
use deckmaste_semantics::Destination;
use deckmaste_semantics::Duration;
use deckmaste_semantics::EnterRider;
use deckmaste_semantics::LifeOp;
use deckmaste_semantics::Modification;
use deckmaste_semantics::NumericOp;
use deckmaste_semantics::ObjectKind;
use deckmaste_semantics::OneShotEffect;
use deckmaste_semantics::PlayerAttr;
use deckmaste_semantics::Predicate;
use deckmaste_semantics::Reference;
use deckmaste_semantics::Selection;
use deckmaste_semantics::Sort;
use deckmaste_semantics::Stat;
use deckmaste_semantics::StatValue;
use deckmaste_semantics::StaticEffect;
use deckmaste_semantics::Supertype;
use deckmaste_semantics::TargetSpec;
use deckmaste_semantics::Token;
use deckmaste_semantics::TokenSpec;
use deckmaste_semantics::TurnMarker;
use deckmaste_semantics::With;
use deckmaste_semantics::Zone;

use super::Ctx;
use super::fragment;

type SemValue = Count;

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
        OneShotEffect::Expanded(e) => match expanded_effect(e, ctx) {
            Some(s) => ensure_period(&capitalize_first(&s)),
            None => effect(&e.value, ctx),
        },
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
        // [CR#608.2d]: "You may [effect]." — the bare optional-effect wrapper.
        // The collapsed `MayPay`/`MustPay` shape (`effect` is `Pay(cost)`)
        // renders as the may-pay kicker / must-pay punisher instead — see
        // `render_may`.
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
        // The announce list ([CR#115.1,601.2c]): render the inner effect with
        // `ctx.targets` rebound to this node's slots, so the body's positional
        // reads (`Target(n)` / `Targets(n)`) resolve to "target creature" etc.
        // Fresh mention state — this list's slot 0 is its own, and none of its
        // slots have been named yet.
        OneShotEffect::Targeted(t) => {
            let named = Cell::new(0);
            effect(&t.effect, &ctx.with_targets(&t.targets, &named))
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
            if let Some(rendered) = search_library(w).or_else(|| chosen_bounce(w)) {
                return rendered;
            }
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
            // `Expanded`) to its core effect so the collective renderer sees
            // the keyword action ("Each player draws a card."), not
            // "For each player, …". Draw/mill are `Batch(n,
            // Act(Mill/Draw))` over the loop element; destroy/
            // discard stay a single `Act(Composite)`.
            let peeled = peel_expanded(&fe.effect);
            if let Some(collective) = each_collective_batch(peeled, &fe.binder, ctx) {
                return collective;
            }
            if let OneShotEffect::Act(act) = peeled
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
        // [CR#701.27a] the Delver-of-Secrets upkeep peek (whole-shape arm; see
        // `delver_look_top`) — any other `If` keeps `[unrendered]`.
        OneShotEffect::If(f) => delver_look_top(f, ctx).unwrap_or_else(|| unrendered(e)),
        other => unrendered(other),
    }
}

/// The structural fall-through for an effect shape this renderer has no arm
/// for yet — a LOUD `[unrendered: …]` marker (the fidelity gate fails on it,
/// never silently drops the clause).
fn unrendered(e: &OneShotEffect) -> String {
    format!("[unrendered: {e:?}].")
}

/// Render a macro invocation through its own rules-text `template`, resolving
/// args CONTEXT-aware (the template layer is context-free and can't reach
/// `fragment`): a `Reference` arg — a fight's `Target(n)` — renders as its
/// phrase ("target creature you control"), so `${0} fights ${1}` reads back to
/// oracle; a `${n:pro}` slot renders the number-aware PRONOMINAL re-mention
/// ("it"/"they") English uses for an immediate next-sentence back-reference
/// ("… It can't be regenerated."); every other arg keeps the context-free
/// `render_slot`. `None` when there's no template or a slot declines — the
/// caller then falls back to structural rendering of `e.value`.
fn expanded_effect(e: &deckmaste_semantics::Expansion<OneShotEffect>, ctx: &Ctx) -> Option<String> {
    let tmpl = e.template.as_deref()?;
    super::template::fill_with(tmpl, ctx.subject, &e.args, |raw, modifier| {
        if modifier.is_none()
            && let Ok(r) = deckmaste_semantics::ron::options().from_str::<Reference>(raw)
        {
            return Some(fragment::reference(&r, ctx));
        }
        // A `${n:pro}` slot needs `ctx` for the referent's number, so it is
        // resolved here (the context-free `render_slot` declines it).
        if modifier == Some("pro") {
            return fragment::reference_pronoun(raw, ctx);
        }
        super::template::render_slot(raw, modifier)
    })
}

/// The Delver-of-Secrets front ability ([CR#701.27a]): "Look at the top card
/// of your library. You may reveal that card. If [a/an TYPE] card is revealed
/// this way, transform ~." A full-info top-card read
/// (`Matches(Single(TopOfLibrary 1), Or([Type…]))`) whose OPTIONAL
/// reveal-then-transform is one `May` sequence. The look is implicit in the
/// condition and "revealed this way" is English surface, not a distinct node —
/// so the whole sentence is REORDERED relative to the tree ("you may reveal"
/// precedes the "if …" gate), which is why this is one bespoke recognizer
/// emitting the entire surface (like Scry/Explore render their whole effect)
/// rather than a structural walk contextualizing the generic
/// `Matches`/`Or`/`Transform` renderers by their parent. `None` for any
/// `If`/condition/body shape outside this exact family — the caller falls back
/// to `[unrendered]`, matching every other bespoke-shape renderer here.
fn delver_look_top(f: &deckmaste_semantics::If, ctx: &Ctx) -> Option<String> {
    use deckmaste_semantics::Condition;
    // Condition: the sole top card of your library matches an Or of card types.
    let Condition::Matches(cond_ref, Predicate::Or(members)) = &f.condition else {
        return None;
    };
    if !is_single_top_of_your_library(cond_ref) || f.otherwise.is_some() {
        return None;
    }
    let types = or_type_words(members)?;
    // then: you MAY (reveal that same top card, then transform ~).
    let OneShotEffect::May(deckmaste_semantics::May {
        who: _,
        effect: body,
        if_did: None,
        if_not: None,
    }) = f.then.as_ref()
    else {
        return None;
    };
    let OneShotEffect::Sequentially(parts) = body.as_ref() else {
        return None;
    };
    let [
        OneShotEffect::Act(Action::Reveal {
            what: reveal_ref,
            to: None,
        }),
        OneShotEffect::Act(Action::Transform(target)),
    ] = parts.as_ref()
    else {
        return None;
    };
    if !is_single_top_of_your_library(reveal_ref) {
        return None;
    }
    let noun = a_an(&format!("{} card", types.join(" or ")));
    Some(format!(
        "Look at the top card of your library. You may reveal that card. \
         If {noun} is revealed this way, transform {}.",
        fragment::reference(target, ctx),
    ))
}

/// Whether `r` names the sole top card of YOUR library — the Idris
/// `Single(TopOfLibrary(^1))` the Delver peek reads twice (condition + reveal).
fn is_single_top_of_your_library(r: &Reference) -> bool {
    let Reference::Single(sel) = r else {
        return false;
    };
    matches!(
        sel.as_ref(),
        Selection::TopOfLibrary { count, whose: Reference::You }
            if count.literal_value() == Some(1)
    )
}

/// The lowercased type words of an `Or([Type(a), Type(b), …])` card-type
/// disjunction ("instant", "sorcery") — `None` if any member is not a bare
/// `Type` characteristic (so a non-type disjunction declines the Delver arm).
fn or_type_words(members: &[Predicate]) -> Option<Vec<String>> {
    if members.is_empty() {
        return None;
    }
    members
        .iter()
        .map(|m| match m {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                Some(t.as_str().to_lowercase())
            }
            _ => None,
        })
        .collect()
}

/// See [`OneShotEffect::RevealUntil`]'s render arm above.
fn reveal_until(r: &deckmaste_semantics::RevealUntil, ctx: &Ctx) -> String {
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
            None,
        )),
        OneShotEffect::Act(Action::MoveGroup {
            group: Selection::They,
            arrangement,
            to: Destination::Library(anchor),
            riders: group_riders,
        }),
    ] = parts.as_ref()
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

/// The chosen-subject bounce ([CR#400.3]/[CR#402.1]): `With(ChooseOne(filter),
/// Move(That, Hand))` -> "Return a land you control to its owner's hand." /
/// "Return another creature you control to its owner's hand." The controller
/// chooses one permanent they control — which they may not OWN, so the
/// destination reads "its owner's hand" (a graveyard-scoped choice, owned by
/// its player, stays "your"). Distinct from a *targeted* bounce; the bound
/// `That` phrase is built by `subject_phrase`'s `SingularArticle` register so
/// self-exclusion prints "another", not "an other". `None` for any
/// binder/body/destination outside this shape — the caller falls back to the
/// generic `With` rendering, matching the other bespoke recognizers here
/// (`search_library`, `reveal_until`).
fn chosen_bounce(w: &With) -> Option<String> {
    let Binder::ChooseOne { filter, .. } = &w.binder else {
        return None;
    };
    let OneShotEffect::Act(Action::Move(
        Reference::That(_),
        Destination::Zone(Zone::Hand),
        riders,
        None,
    )) = &*w.body
    else {
        return None;
    };
    if !riders.is_empty() {
        return None;
    }
    let subject = fragment::subject_phrase(filter, fragment::SubjectNumber::SingularArticle)?;
    let possessive = if fragment::is_graveyard_scoped(filter) { "your" } else { "its owner's" };
    Some(format!("Return {subject} to {possessive} hand."))
}

/// The library-search / tutor family ([CR#701.23a]): `With(SearchOne(filter),
/// Sequentially([Reveal(That(Card))?, Move(That(Card), <zone>, <riders>),
/// Shuffle]))` -> "Search your library for `<filter>`, [reveal it,] put it
/// `<destination>`, then shuffle." `None` for any binder/body shape outside
/// this exact pattern (a foreign subject, a graveyard search, a plural
/// `Search`, an unrecognized body) — the caller falls back to the generic
/// `With` rendering, matching every other bespoke-shape renderer in this file
/// (`reveal_until`, `separate_piles`, …).
///
/// Only the self-search-your-own-library shape is recognized: `SearchOne`
/// with `by`/`whose` both the default `You` and `from` the default
/// `[Library]` — mirroring `deckmaste_migrations`' `parse_search_library`
/// production, which only ever builds that shape.
fn search_library(w: &With) -> Option<String> {
    let Binder::SearchOne {
        filter,
        by: Reference::You,
        whose: Reference::You,
        from,
        if_none: None,
    } = peel_binder(&w.binder)
    else {
        return None;
    };
    if from.as_ref() != [Zone::Library] {
        return None;
    }
    let OneShotEffect::Sequentially(parts) = w.body.as_ref() else {
        return None;
    };
    let (reveal, move_part, shuffle_part) = match parts.as_ref() {
        [a, b, c] => (Some(a), b, c),
        [a, b] => (None, a, b),
        _ => return None,
    };
    if let Some(r) = reveal
        && !matches!(
            r,
            OneShotEffect::Act(Action::Reveal {
                what: Reference::That(Sort::Card),
                to: None,
            })
        )
    {
        return None;
    }
    let OneShotEffect::Act(Action::Move(
        Reference::That(Sort::Card),
        Destination::Zone(zone),
        riders,
        None,
    )) = move_part
    else {
        return None;
    };
    let (dest_phrase, tapped_ok) = match zone {
        Zone::Hand => ("into your hand", false),
        Zone::Battlefield => ("onto the battlefield", true),
        Zone::Graveyard => ("into your graveyard", false),
        _ => return None,
    };
    let tapped = match riders.as_ref() {
        [] => false,
        [EnterRider::Tapped] if tapped_ok => true,
        _ => return None,
    };
    if !matches!(
        shuffle_part,
        OneShotEffect::Act(Action::Shuffle(Selection::LibraryOf(Reference::You)))
    ) {
        return None;
    }
    let noun = search_filter_phrase(filter)?;
    let mut out = format!("Search your library for {noun}");
    if reveal.is_some() {
        out.push_str(", reveal it");
    }
    let _ = write!(
        out,
        ", put it {dest_phrase}{}",
        if tapped { " tapped" } else { "" }
    );
    out.push_str(", then shuffle.");
    Some(out)
}

/// See through a macro invocation to its expanded value — the `Binder`-side
/// twin of [`peel_expanded`] ([`Predicate`]) / `strip_expanded` (used
/// elsewhere in this crate). No current macro invocation targets a `Binder`
/// position, but every other structural-shape recognizer in this file peels
/// its node's `Expanded` wrapper before matching, so this keeps
/// [`search_library`] consistent rather than assuming its input is never
/// macro-provenance.
fn peel_binder(binder: &Binder) -> &Binder {
    match binder {
        Binder::Expanded(e) => peel_binder(&e.value),
        other => other,
    }
}

/// The noun phrase a [`search_library`] filter renders as ("a basic land
/// card", "a Forest card", "an artifact card", "a green creature card", "a
/// basic Plains, Swamp, or Forest card"), or `None` for a filter shape
/// outside the grammar `deckmaste_migrations`' `search_card_filter` builds.
/// The mirror image of that parser: a bare `Subtype`/`Or([Subtype, …])` atom
/// carries NO card-type word at all [CR#205.3m] (a Tribal card gives its
/// printed creature subtype to a noncreature card, so "a Goblin card" must
/// keep matching a Tribal Instant — Goblin — the parser never injects a
/// parent `Type`, and this never prints one back); a `Supertype(Basic)` with
/// no subtype prints "basic land", and a `Supertype(Snow)` prints "snow
/// land" (those two DO carry an explicit `Type("Land")`, since "land" is the
/// printed word, not an inferred category).
fn search_filter_phrase(filter: &Predicate) -> Option<String> {
    if let Predicate::Or(members) = filter {
        // A bare subtype-only disjunction ("a Swamp or Mountain card") — ONE
        // shared article, no per-member "card" (distinct from the top-level
        // full-phrase disjunction below, where each side is its own "a ...
        // card" phrase — Wayfarer's Bauble's "a basic land card or a Desert
        // card").
        if let Some(subtypes) = bare_subtype_list(members) {
            return Some(a_an(&format!("{} card", join_or_list(&subtypes))));
        }
        let phrases: Vec<String> = members
            .iter()
            .map(search_filter_phrase)
            .collect::<Option<_>>()?;
        return Some(phrases.join(" or "));
    }
    if matches!(filter, Predicate::Kind(ObjectKind::Card)) {
        return Some("a card".to_owned());
    }
    // A bare subtype atom alone ("a Forest card", "an Equipment card") — no
    // wrapping `And`/`Type` [CR#205.3m].
    if let Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) = filter {
        return Some(a_an(&format!("{} card", s.name().as_str())));
    }
    let mut ty: Option<&str> = None;
    let mut supertype: Option<Supertype> = None;
    let mut color: Option<&'static str> = None;
    let mut subtypes: Vec<&str> = Vec::new();
    for atom in flatten_and(filter) {
        match atom {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                ty = Some(t.name().as_str());
            }
            Predicate::Characteristic(CharacteristicPredicate::Supertype(s)) => {
                supertype = Some(*s);
            }
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
                color = Some(color_word(*c));
            }
            Predicate::Characteristic(CharacteristicPredicate::Colorless) => {
                color = Some("colorless");
            }
            Predicate::Characteristic(CharacteristicPredicate::Multicolored) => {
                color = Some("multicolored");
            }
            Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) => {
                subtypes.push(s.name().as_str());
            }
            Predicate::Or(members) => {
                for m in members.iter() {
                    let Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) = m else {
                        return None;
                    };
                    subtypes.push(s.name().as_str());
                }
            }
            _ => return None,
        }
    }
    let descriptor = if subtypes.is_empty() {
        let type_word = type_word(ty?)?;
        match (supertype, color) {
            (Some(Supertype::Basic), None) => format!("basic {type_word}"),
            (Some(Supertype::Snow), None) => format!("snow {type_word}"),
            (None, Some(c)) => format!("{c} {type_word}"),
            (None, None) => type_word.to_owned(),
            _ => return None,
        }
    } else if ty.is_none() {
        // "basic <Subtype>[, …]" — the type word is implicit, matching the
        // bare-subtype-alone case above; a plain `Type` atom never rides
        // alongside a `Subtype` in this grammar (guarded rather than
        // silently dropped).
        let list = join_or_list(&subtypes);
        match supertype {
            Some(Supertype::Basic) => format!("basic {list}"),
            None => list,
            _ => return None,
        }
    } else {
        return None;
    };
    Some(a_an(&format!("{descriptor} card")))
}

/// Whether every member of an `Or` is a bare `Subtype` atom — the search
/// filter's subtype-"or"-list register ("a Swamp or Mountain card"). Returns
/// their names in order, or `None` if any member isn't a bare `Subtype` (the
/// top-level full-phrase-disjunction case instead).
fn bare_subtype_list(members: &[Predicate]) -> Option<Vec<&str>> {
    members
        .iter()
        .map(|m| match m {
            Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) => {
                Some(s.name().as_str())
            }
            _ => None,
        })
        .collect()
}

/// The lowercase card-type word a search filter's `Type` atom names ("Land",
/// "Creature", …), or `None` for a type this family never searches for
/// (Dungeon, Kindred — no printed "search your library for a dungeon card").
fn type_word(ty: &str) -> Option<&'static str> {
    Some(match ty {
        "Land" => "land",
        "Creature" => "creature",
        "Artifact" => "artifact",
        "Enchantment" => "enchantment",
        "Instant" => "instant",
        "Sorcery" => "sorcery",
        "Planeswalker" => "planeswalker",
        "Battle" => "battle",
        _ => return None,
    })
}

/// "A" / "A or B" / "A, B, or C" — the search-filter subtype-list register
/// ([`join_or_list`]'s parser-side twin is `search_card_filter`'s
/// `split_or_list`).
fn join_or_list(items: &[&str]) -> String {
    match items {
        [] => String::new(),
        [a] => (*a).to_string(),
        [a, b] => format!("{a} or {b}"),
        _ => {
            let (last, rest) = items.split_last().expect("non-empty");
            format!("{}, or {last}", rest.join(", "))
        }
    }
}

/// Flattens a (possibly trivial) `And` into its member atoms — the `Vec`
/// counterpart of `fragment`'s private `flatten_all_of`, kept local since
/// this file has no visibility into that one.
fn flatten_and(filter: &Predicate) -> Vec<&Predicate> {
    match filter {
        Predicate::And(members) => members.iter().flat_map(flatten_and).collect(),
        other => vec![other],
    }
}

/// A delayed triggered ability's lead-in + body ([CR#603.7]). See
/// [`OneShotEffect::Delayed`]'s render arm above for why this doesn't just call
/// `ability::event_clause` uncritically.
fn delayed(t: &deckmaste_semantics::TriggeredAbility, ctx: &Ctx) -> String {
    use deckmaste_semantics::EventFilter;
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
fn separate_piles(piles: &deckmaste_semantics::SeparatePiles, ctx: &Ctx) -> String {
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
fn plural_group_noun(f: &deckmaste_semantics::Predicate, ctx: &Ctx) -> String {
    use deckmaste_semantics::CharacteristicPredicate;
    use deckmaste_semantics::Predicate;
    use deckmaste_semantics::RelationPredicate;
    let parts: Vec<&Predicate> = match f {
        Predicate::And(members) => members.iter().collect(),
        other => vec![other],
    };
    let mut noun = None;
    let mut controller = None;
    for part in parts {
        match part {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                noun = Some(format!("{}s", t.name().as_str().to_lowercase()));
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
fn choose_pile(cp: &deckmaste_semantics::ChoosePile, ctx: &Ctx) -> String {
    let chooser = fragment::reference(&cp.by, ctx);
    let pile_phrase = format!("the pile of {chooser}'s choice");
    if let OneShotEffect::Each(each) = peel_expanded(&cp.then)
        && matches!(
            &each.binder,
            deckmaste_semantics::Binder::Existing(Selection::Them(Sort::Pile))
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
    // Destroy-of-`It` is `Composite{name:"Destroy", body: Move(It,
    // Graveyard)}`.
    let is_destroy_it = |b: &OneShotEffect| {
        matches!(b, OneShotEffect::Act(Action::Composite { name, body })
            if name.as_str() == "Destroy"
                && matches!(composite_move_patient(body), Some(Reference::It)))
    };
    match body {
        b if is_destroy_it(b) => Some(format!("Destroy all creatures in {group_phrase}.")),
        // `DestroyNoRegen`'s expansion: `Sequentially([Composite(name: Destroy,
        // body: Move(It, Graveyard)), Until(ForThisEvent, [Cant(Regenerate(on:
        // It))])])` ([CR#701.19c]).
        OneShotEffect::Sequentially(parts) => match parts.as_ref() {
            [b, OneShotEffect::Until(Duration::ForThisEvent, statics)] if is_destroy_it(b) => {
                match statics.as_ref() {
                    [StaticEffect::Deontic(Deontic::Cant(DeonticAction::Regenerate { .. }))] => {
                        Some(format!(
                            "Destroy all creatures in {group_phrase}. They can't be regenerated."
                        ))
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// A modal spell/ability ([CR#700.2]): an optional Escalate/Entwine cost-
/// rider line, "Choose ... —", and one bulleted mode per line.
fn modal_effect(modal: &deckmaste_semantics::Modal, ctx: &Ctx) -> String {
    use deckmaste_semantics::ModalCostRider;
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
    for mode in modal.modes.iter() {
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
fn choose_line(spec: &deckmaste_semantics::ChooseSpec) -> String {
    let (lo, hi) = spec.count.bounds();
    let lo_n = lo.and_then(Count::literal_value);
    let hi_n = hi.and_then(Count::literal_value);
    match (lo_n, hi_n, spec.up_to) {
        (Some(1), Some(1), false) => "Choose one \u{2014}".to_string(),
        _ => "Choose one or more \u{2014}".to_string(),
    }
}

/// The noun phrase a [`Binder`](deckmaste_semantics::Binder) contributes to its
/// `OneShotEffect::With` / `CostComponent::With` body — read by the body's
/// `That` / `Those` anaphor ([CR#601.2b]). A one-binder yields "a creature"; a
/// many-binder yields "two cards"; the reference/existing forms defer to the
/// shared fragment renderers.
fn binder_phrase(binder: &deckmaste_semantics::Binder, ctx: &Ctx) -> String {
    use deckmaste_semantics::Binder;
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

/// The single-move patient of a keyword-action `Composite` body — `Destroy`'s
/// `Move(r, Graveyard)` head names `r`, else `None`. Reads the patient off the
/// stored body, the name-keyed engine dispatch's renderer twin ([CR#701.8a]).
fn composite_move_patient(body: &OneShotEffect) -> Option<&Reference> {
    match peel_expanded(body) {
        OneShotEffect::Act(Action::Move(r, _, _, _)) => Some(r),
        _ => None,
    }
}

/// The milling/drawing player of a slice-verb body ([CR#121,701.17a]): the
/// `whose` of the body's `TopOfLibrary` selection (draw's `Each` binder, mill's
/// `MoveGroup` group). `None` for any other shape.
fn slice_whose(body: &OneShotEffect) -> Option<&Reference> {
    use deckmaste_semantics::Selection;
    match peel_expanded(body) {
        OneShotEffect::Each(each) => match &each.binder {
            deckmaste_semantics::Binder::Existing(Selection::TopOfLibrary { whose, .. }) => {
                Some(whose)
            }
            _ => None,
        },
        OneShotEffect::Act(Action::MoveGroup {
            group: Selection::TopOfLibrary { whose, .. },
            ..
        }) => Some(whose),
        _ => None,
    }
}

/// The collective rendering of an [`OneShotEffect::Each`] over a
/// `Batch`-wrapped per-card action ([CR#121.1,701.17a]) with the loop element
/// as performer — "Each player draws/mills N cards." (Jace Beleren's "+2:
/// Each player draws a card."). Both spell `Batch(count, …)` with the batch
/// count as the card count, but they name their performer differently, because
/// only one of them is a keyword action:
///
/// - **mill** ([CR#701.17a]) is a keyword action, so its per-unit body is a
///   `Composite` and the performer rides that body's `TopOfLibrary` slice.
/// - **draw** ([CR#121.1]) is NOT ([CR#701] does not list it) and is
///   irreducible ([CR#121.5]), so it has no body at all — its per-unit form is
///   `DrawCard(who)` and the performer is its own agent slot.
///
/// `None` for any other shape.
fn each_collective_batch(
    effect: &OneShotEffect,
    binder: &deckmaste_semantics::Binder,
    ctx: &Ctx,
) -> Option<String> {
    let OneShotEffect::Batch(count, body) = effect else {
        return None;
    };
    let (verb, whose) = match peel_expanded(body) {
        OneShotEffect::Act(Action::Composite {
            name,
            body: verb_body,
        }) => (name.as_str(), slice_whose(verb_body)?),
        OneShotEffect::Act(Action::DrawCard(who)) => ("Draw", who),
        _ => return None,
    };
    if !matches!(whose, Reference::It) {
        return None;
    }
    let each_group = format!("each {}", binder_group_noun(binder, ctx));
    match verb {
        "Draw" => Some(format!(
            "{} draws {}.",
            capitalize_first(&each_group),
            counted_cards(count),
        )),
        "Mill" => Some(format!(
            "{} mills {}.",
            capitalize_first(&each_group),
            counted_cards(count),
        )),
        _ => None,
    }
}

/// The collective rendering of an [`OneShotEffect::Each`] whose body is a
/// single group verb acting on the per-element [`Reference::It`] — the natural
/// "<verb> each <group>" / "put <group> on <dest>" surface ([CR#608]), the
/// renderer half the `core-many-binder-group-move` seam calls for. Returns
/// `None` for any body the collapse does not recognise, so the caller falls
/// back to the per-element "For each <group>, …" form.
fn each_collective(
    act: &Action,
    binder: &deckmaste_semantics::Binder,
    ctx: &Ctx,
) -> Option<String> {
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
        // `Composite{name:"Destroy", body: Move(It, Graveyard)}` the `Destroy`
        // macro builds over the loop element; the name carries the printed
        // keyword and the patient rides the body's `Move` source.
        Action::Composite { name, body }
            if name.as_str() == "Destroy"
                && matches!(composite_move_patient(body), Some(Reference::It)) =>
        {
            Some(format!("Destroy {}.", each_group()))
        }
        // [CR#701.9a,701.9b]: "Each player discards N cards[ at random]." —
        // discard is `Composite{name:"Discard", body: With(Choose/Random, ..)}`
        // over the loop element as performer (draw/mill's per-card twin, but
        // Batch-wrapped and handled by `each_collective_batch`); the at-random
        // qualifier rides the body's `With` binder shape.
        Action::Composite { name, body }
            if name.as_str() == "Discard"
                && matches!(
                    deckmaste_semantics::discard_body_whose(body),
                    Some(Reference::It)
                ) =>
        {
            Some(format!(
                "{} discards {}{}.",
                capitalize_first(&each_group()),
                counted_cards(
                    deckmaste_semantics::discard_body_count(body).unwrap_or(&Count::Literal(1))
                ),
                if deckmaste_semantics::discard_body_random(body) { " at random" } else { "" },
            ))
        }
        // A group move to the library reads "Put <group> on top/the bottom of
        // your library." — Brainstorm's "put two cards … on top": the chosen
        // group's own phrase, not "each" ([CR#401.7]). A multi-card put
        // prints the "in any order" rider the oracle sentence carries.
        // Riders never apply to a library destination, so a rider-carrying
        // move falls through.
        Action::Move(Reference::It, Destination::Library(anchor), riders, None)
            if riders.is_empty() =>
        {
            Some(format!(
                "Put {} on {} of your library{}.",
                binder_phrase(binder, ctx),
                fragment::library_position(anchor),
                if binder_is_plural(binder) { " in any order" } else { "" },
            ))
        }
        // Set-wide tap/untap ([CR#701.26a,701.26b]): "Tap each <group>."
        Action::Tap(Reference::It) => Some(format!("Tap {}.", each_group())),
        Action::Untap(Reference::It) => Some(format!("Untap {}.", each_group())),
        // Set-wide counter placement ([CR#122.1,608.2d]): "Put a +1/+1 counter
        // on each <group>." — the mass twin of the targeted "Put … on <ref>."
        // (`player_action`'s `PutCounters` arm), the loop element the
        // placement patient `It`. Agent-silent, so no agent plays any part in
        // the printed sentence.
        Action::PutCounters(Reference::It, kind, count) => Some(format!(
            "Put {} on {}.",
            counter_phrase(kind, count),
            each_group(),
        )),
        // [CR#119.1,119.5]: "Each player's life total becomes N." — a
        // possessive-subject sentence (the value belongs to the loop
        // element), unlike the subject-verb pattern the generic per-verb-agent
        // arm below handles ("Each player mills …"). Arbiter of Knollridge's
        // own shape: `count` is typically a cross-player `Aggregate`
        // ([CR#119.1]) reading "the highest life total among all players".
        Action::ChangeLife(Reference::It, LifeOp::Set(count)) => Some(format!(
            "{}'s life total becomes {}.",
            capitalize_first(&each_group()),
            fragment::count(count)
        )),
        // Subject-declarative player verbs over the loop element as AGENT
        // ([CR#608.2d] distributive each; [CR#701.17a,701.9,121.1,119.3]):
        // "Each player mills two cards." / "Each opponent loses 2 life."
        act if verb_agent(act) == Some(&Reference::It) => {
            let verb = third_person_verb_phrase(act)?;
            Some(format!("{} {verb}.", capitalize_first(&each_group())))
        }
        _ => None,
    }
}

/// Whether a binder binds MORE than one object (a multi-card group move
/// prints its "in any order" rider).
fn binder_is_plural(binder: &deckmaste_semantics::Binder) -> bool {
    use deckmaste_semantics::Binder;
    use deckmaste_semantics::Count;
    use deckmaste_semantics::Selection;
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

/// The bare collective noun a [`Binder`](deckmaste_semantics::Binder)
/// contributes to an [`OneShotEffect::Each`] "each <noun>" / "For each <noun>"
/// construction: the whole matching set yields the bare noun ("creature", so
/// the surrounding "each" supplies the quantifier — not "each each creature"),
/// a bound/announced group its plural anaphor ("them"), and a chosen group its
/// full phrase.
fn binder_group_noun(binder: &deckmaste_semantics::Binder, ctx: &Ctx) -> String {
    use deckmaste_semantics::Binder;
    use deckmaste_semantics::Selection;
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
/// ([CR#701.12a,701.12c], Axis of Mortality): `ChangeLife(<actor>, Set(
/// PlayerStatOf(<other>, Life)))`. Returns `(actor, other)` — the player
/// whose life is being set and the player whose (pre-exchange) total it's
/// copying.
fn exchange_life_half(e: &OneShotEffect) -> Option<(&Reference, &Reference)> {
    let OneShotEffect::Act(Action::ChangeLife(actor, LifeOp::Set(count))) = e else {
        return None;
    };
    let Count::PlayerStatOf(other, PlayerAttr::Life) = count else {
        return None;
    };
    Some((actor, other))
}

/// "have two target players exchange life totals" ([CR#701.12a,701.12c],
/// Axis of Mortality's `May`-wrapped trigger body): the mirrored `ChangeLife`/
/// `PlayerStatOf` pair reads as one collective verb over the announced
/// targets, not two separate "X's life total becomes Y" sentences — the
/// per-half phrasing (`Action::ChangeLife(Reference::It, LifeOp::Set(_))`,
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
    use deckmaste_semantics::NumericOp;
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

#[expect(
    clippy::too_many_lines,
    reason = "one arm per Action variant; splitting would scatter the render dispatch"
)]
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
        // [CR#701.6a]: counter a spell or ability on the stack — "Counter
        // target spell" (Mana Leak's punisher branch).
        Action::Counter(r) => format!("Counter {}.", fragment::reference(r, ctx)),
        // [CR#701.27a]: flip a transforming DFC to its other face.
        Action::Transform(r) => format!("Transform {}.", fragment::reference(r, ctx)),
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
        Action::Move(r, Destination::Library(anchor), riders, None) if riders.is_empty() => {
            format!(
                "Put {} on {} of {} library.",
                fragment::reference(r, ctx),
                fragment::library_position(anchor),
                fragment::move_possessive(r, ctx),
            )
        }
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
                deckmaste_semantics::Arrangement::AnyOrder
                    | deckmaste_semantics::Arrangement::ChosenOrder(_)
            ) {
                " in any order"
            } else {
                ""
            },
        ),
        // [CR#701]: a named keyword action is transparent to structural
        // rendering — its meaning IS its body (the printed keyword name rides
        // the macro template when authored via a macro, the common corpus
        // path; a raw composite renders its body).
        Action::Composite { body, .. } => effect(body, ctx),
        // [CR#701.13a]/[CR#400.7]: exiling a graveyard-hate target — "Exile
        // target [<type>] card from a graveyard." — any player's graveyard,
        // per `any_graveyard_card_filter` (migrations effect.rs), so "from a
        // graveyard" is static text appended here, not derived from the
        // reference. Unlike the reanimation arm's bare reference-SHAPE guard
        // (`It`/`This` alone), this `Move(It, Exile)` shape is shared with the
        // general "Exile target <subject>." production below, so the zone
        // must be read off the actual target filter.
        Action::Move(r, Destination::Zone(Zone::Exile), riders, None)
            if riders.is_empty()
                && fragment::target_slot_filter(r, ctx)
                    .is_some_and(fragment::is_graveyard_scoped) =>
        {
            format!("Exile {} from a graveyard.", fragment::reference(r, ctx))
        }
        // Exiling is a pure zone move ([CR#701.13]) — "Exile <r>." (the
        // player-agent `Move` twin was DELETED and merged into this one,
        // agent-silent, verb).
        Action::Move(r, Destination::Zone(Zone::Exile), riders, None) if riders.is_empty() => {
            format!("Exile {}.", fragment::reference(r, ctx))
        }
        // [CR#402.1]/[CR#400.3]: a hand destination — "Return <r> to your
        // hand." / "Return <r> to its owner's hand." The possessive is chosen
        // by `fragment::move_possessive` from the reference shape: the self /
        // search / reveal forms (`This`/`It`/`That`) and a graveyard-scoped
        // target print "your"; a targeted permanent that could be an
        // opponent's (Unsummon-style bounce) prints "its owner's".
        Action::Move(r, Destination::Zone(Zone::Hand), riders, None) if riders.is_empty() => {
            format!(
                "Return {} to {} hand.",
                fragment::reference(r, ctx),
                fragment::move_possessive(r, ctx),
            )
        }
        // [CR#400.7]: graveyard reanimation — the empty-rider battlefield case
        // (the migrations `parse_reanimate` production's emission, no rider
        // since the owner-control default already applies to a "your
        // graveyard" subject). "from your graveyard" is static text here, not
        // derived from the reference: a bare `Move` carries no origin-zone
        // field, so the arm must establish the graveyard origin some other way.
        //
        // The TARGETED form reads its slot's own filter ([CR#115.3,601.2c]) —
        // the same discipline as the exile-from-a-graveyard arm above — so a
        // battlefield-return of a target that isn't graveyard-scoped falls
        // through rather than being mislabelled.
        Action::Move(
            r @ Reference::Target(_),
            Destination::Zone(Zone::Battlefield),
            riders,
            None,
        ) if riders.is_empty()
            && fragment::target_slot_filter(r, ctx).is_some_and(fragment::is_graveyard_scoped) =>
        {
            format!(
                "Return {} from your graveyard to the battlefield.",
                fragment::reference(r, ctx)
            )
        }
        // The untargeted forms have no slot to read a filter off, so the
        // reference SHAPE reserves the phrasing for the graveyard-recursion
        // family. It must NOT catch a bare-reference return from another
        // origin: `parse_return_that_card` emits a riderless `Move(That(Card),
        // Battlefield)` for an exile return (Otherworldly Journey), which is
        // not graveyard-sourced and would be mislabelled here — so the guard
        // excludes `That`, letting it fall through to the
        // zone-agnostic/unrendered path.
        Action::Move(
            r @ (Reference::It | Reference::This),
            Destination::Zone(Zone::Battlefield),
            riders,
            None,
        ) if riders.is_empty() => {
            format!(
                "Return {} from your graveyard to the battlefield.",
                fragment::reference(r, ctx)
            )
        }
        // A battlefield destination WITH arrival riders ([CR#614.12],
        // Otherworldly Journey's delayed return): "Return <r> to the
        // battlefield <rider phrase>."
        Action::Move(r, Destination::Zone(Zone::Battlefield), riders, None)
            if !riders.is_empty() =>
        {
            format!(
                "Return {} to the battlefield{}.",
                fragment::reference(r, ctx),
                enter_rider_phrase(riders, ctx),
            )
        }
        // Agent-silent player verbs ([CR#701.26a..701.26b,122.1,614.8]) —
        // no `By` agent to check, so always imperative.
        Action::Tap(r) => format!("Tap {}.", fragment::reference(r, ctx)),
        Action::Untap(r) => format!("Untap {}.", fragment::reference(r, ctx)),
        Action::PutCounters(r, kind, count) => format!(
            "Put {} on {}.",
            counter_phrase(kind, count),
            fragment::reference(r, ctx),
        ),
        Action::RemoveDamage(r) => {
            format!("Remove all damage from {}.", fragment::reference(r, ctx))
        }
        // A non-`You` agent renders subject-declarative ("Target opponent
        // loses 2 life."); the implicit-`You` default keeps the imperative
        // form ("Discard a card."). A verb with no third-person phrase falls
        // back to the imperative render. (Mill/draw no longer route here —
        // they left the player-verb family for the `Composite` keyword-action
        // lane.)
        act if verb_agent(act).is_some() => {
            let who = verb_agent(act).expect("guarded above");
            match who {
                Reference::You => player_action(act, ctx),
                other => third_person_verb_phrase(act).map_or_else(
                    || player_action(act, ctx),
                    |verb| {
                        format!(
                            "{} {verb}.",
                            capitalize_first(&fragment::reference(other, ctx))
                        )
                    },
                ),
            }
        }
        // [CR#701.19a]: a regeneration shield. The protected permanent is the
        // `That` the enclosing `With` bound (no authored `subject`), so the
        // bare fallback names it as "that"; the top-level `Regenerate` keyword
        // macro's `regenerate ${0}` template is the primary render (the
        // remembered invocation carries the real reference).
        Action::CreateReplacement { .. } => {
            let that = deckmaste_semantics::Reference::That(deckmaste_semantics::Sort::Card);
            format!("Regenerate {}.", fragment::reference(&that, ctx))
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
        Count::Literal(_) | SemValue::X | Count::ThatMany | Count::ThatMuch => {
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
            // [CR#110.2a]: "under your control" — Cloudshift's wording, and
            // English's only irregular possessive pronoun in this position
            // ("you's" is not a word). Every other reference takes the
            // regular `'s` possessive.
            EnterRider::UnderControlOf(Reference::You) => {
                parts.push("under your control".to_string());
            }
            EnterRider::UnderControlOf(who) => {
                parts.push(format!("under {}'s control", fragment::reference(who, ctx)));
            }
            EnterRider::WithCounters(kind, count) => {
                parts.push(format!("with {} on it", counter_phrase(kind, count)));
            }
            EnterRider::Tapped => parts.push("tapped".to_string()),
            EnterRider::FaceDown => parts.push("face down".to_string()),
            EnterRider::Attacking(_) => parts.push("attacking".to_string()),
            // "enters as a copy of [source]" ([CR#707.5]) — the rider phrase
            // itself; the surrounding "You may have ~ enter …" framing is a
            // separate ETB-replacement seam this rider list doesn't build
            // (no caller wraps it that way yet). Exceptions ([CR#707.9])
            // append the shared ", except …" clause.
            EnterRider::AsCopy(spec) => parts.push(format!(
                "enters as a copy of {}{}",
                copy_source_phrase(&spec.source, ctx),
                copy_exceptions_clause(&spec.exceptions)
            )),
        }
    }
    if parts.is_empty() { String::new() } else { format!(" {}", parts.join(" ")) }
}

/// A counter placement as its printed article + count noun phrase — "a
/// +1/+1 counter" / "two +1/+1 counters". `P1P1Counter`/`M1M1Counter`
/// (Undying/Persist's pip family) print their `+N/+N` symbol, exactly like
/// the card frame does; any other named kind ("a lore counter") uses the
/// plain word, same as `fragment`'s `counter_noun`.
pub(super) fn counter_phrase(kind: &deckmaste_semantics::CounterRef, count: &Count) -> String {
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
fn divide_among(d: &deckmaste_semantics::Distribute, ctx: &Ctx) -> String {
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
        OneShotEffect::Act(Action::PutCounters(_, kind, _)) => {
            format!(
                "Distribute {amount} {} counters among {group}.",
                kind.as_str()
            )
        }
        other => format!("[unrendered: {other:?}]."),
    }
}

/// The group a divided distribution names: an announced plural target slot
/// (read by position as `Targets(n)`) prints its announce phrase — "one,
/// two, or three targets" ([CR#601.2d]); anything else falls back to the
/// binder's own phrase.
fn divided_group_phrase(binder: &deckmaste_semantics::Binder, ctx: &Ctx) -> String {
    use deckmaste_semantics::Binder;
    use deckmaste_semantics::Selection;
    let slot = match binder {
        // The nth announced slot read as its whole group ([CR#115.3,601.2c]).
        Binder::Existing(Selection::Targets(n)) => ctx.targets.get(*n),
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
fn additional_payment(cost: &[deckmaste_semantics::CostComponent], ctx: &Ctx) -> Option<String> {
    use deckmaste_semantics::CostComponent;
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
                let phrase = trim_period(&do_action_phrase(pa, ctx));
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
                            let p = trim_period(&do_action_phrase(pa, &inner));
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

/// The verb phrase a `CostComponent::Do(action)` cost renders
/// ([CR#601.2b]): a player verb (agent slot spelled `You` in cost context)
/// through the imperative `player_action` clause; a keyword-action composite
/// ("Discard a card:", [CR#701.9]) through its `action` tag arm.
fn do_action_phrase(act: &Action, ctx: &Ctx) -> String {
    if verb_agent(act).is_some() {
        player_action(act, ctx)
    } else {
        action(act, ctx)
    }
}

/// An activated ability's printed cost line ([CR#602.1] — cost components
/// separated by commas, e.g. "{G}, {T}, Sacrifice a creature:"). Each
/// symbol-only component (mana/tap/…) renders through the shared `render_cost`
/// glyph renderer; the symbols within ONE mana component remain glued
/// (`{1}{W}`), while distinct components are comma-separated (`{1}{W}, {T}`).
/// A verb component (`Do`/`With`) renders its lowercased clause, exactly like
/// [`additional_payment`]'s reader — all segments then join with ", ". No
/// existing corpus card mixes a
/// symbol run with a verb component in an ACTIVATION cost yet (only
/// `AdditionalCost`'s printed-additional-cost clause did, which is why this
/// is a distinct function rather than a reuse of `additional_payment`: that
/// one's all-symbol case reads "pay {cost}", which is wrong here — an
/// activation cost line never says "pay").
/// The loyalty-cost prefix for a planeswalker loyalty ability's cost
/// verb ([CR#606.4], "the cost to activate a loyalty ability is to put on or
/// remove that many loyalty counters"): `PutCounters(This, LoyaltyCounter, n)`
/// prints `+n` (or `0` when `n == 0` — a `LoyaltyZero` ability), and
/// `RemoveCounters(This, LoyaltyCounter, n)` prints `−n` (the `−` is U+2212
/// MINUS SIGN, the glyph the printed card uses, not an ASCII hyphen). Keyed on
/// the `LoyaltyCounter` name and the `This` subject, mirroring the engine's
/// `is_loyalty_ability` discriminator — any other counter cost (a different
/// counter, or one on a non-`This` subject) returns `None` and renders through
/// the generic `player_action` clause. A bare literal count brackets, and so
/// does the variable `−X` loyalty cost ([CR#107.7] — the semantic X variant,
/// e.g. Ugin, the Spirit Dragon); any other dynamic count falls
/// back to the generic render.
fn loyalty_cost_prefix(action: &Action) -> Option<String> {
    let is_loyalty = |c: &deckmaste_semantics::CounterRef| c.as_str() == "LoyaltyCounter";
    match action {
        Action::PutCounters(Reference::This, counter, count) if is_loyalty(counter) => {
            match count.literal_value()? {
                0 => Some("0".to_owned()),
                n => Some(format!("+{n}")),
            }
        }
        Action::RemoveCounters(Reference::This, counter, SemValue::X) if is_loyalty(counter) => {
            Some("\u{2212}X".to_owned())
        }
        Action::RemoveCounters(Reference::This, counter, count) if is_loyalty(counter) => {
            Some(format!("\u{2212}{}", count.literal_value()?))
        }
        _ => None,
    }
}

pub(super) fn activated_cost(cost: &[deckmaste_semantics::CostComponent], ctx: &Ctx) -> String {
    use deckmaste_semantics::CostComponent;

    fn push_symbol(component: &deckmaste_semantics::CostComponent, parts: &mut Vec<String>) {
        parts.push(
            super::template::render_cost(std::slice::from_ref(component))
                .unwrap_or_else(|| format!("[unrendered: {component:?}]")),
        );
    }

    let mut parts = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(_)
            | CostComponent::Tap
            | CostComponent::Untap
            | CostComponent::ManaCostOf(_)
            | CostComponent::TapTotal { .. } => push_symbol(component, &mut parts),
            CostComponent::Do(pa) => {
                // A planeswalker loyalty ability's cost is a `PutCounters`/
                // `RemoveCounters` of the `LoyaltyCounter` on `This`
                // ([CR#606.4]); it prints as the `+N`/`−N`/`0` prefix, not the
                // generic "put/remove … counter" clause.
                if let Some(prefix) = loyalty_cost_prefix(pa) {
                    parts.push(prefix);
                } else {
                    let phrase = trim_period(&do_action_phrase(pa, ctx));
                    // [CR#602.1]'s printed convention capitalizes each
                    // verb-cost segment ("{T}, Sacrifice a
                    // Goblin: ..."), unlike a body
                    // verb clause joined mid-sentence.
                    parts.push(capitalize_first(&phrase));
                }
            }
            // The same choose-then-pay reader `additional_payment` uses:
            // bind the binder's noun phrase as the body verbs' `That`
            // anaphor, then render the body's `Do` verbs — "Sacrifice a
            // creature".
            CostComponent::With { binder, body } => {
                let phrase = binder_phrase(binder, ctx);
                let inner = ctx.with_that(&phrase);
                for inner_comp in body {
                    match inner_comp {
                        CostComponent::Do(pa) => {
                            let p = trim_period(&do_action_phrase(pa, &inner));
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
            CostComponent::Expanded(e) => match super::template::expanded(e, ctx.subject) {
                Some(s) => parts.push(s),
                None => parts.push(format!("[unrendered: {:?}]", e.value)),
            },
            other @ CostComponent::Cost(_) => {
                parts.push(format!("[unrendered: {other:?}]"));
            }
        }
    }
    parts.join(", ")
}

/// The phrase an `AdditionalCost` body's `EventObject` anaphor should read,
/// derived from the payment itself ([CR#601.2f], "the sacrificed creature's
/// power", Fling) — the cost-side twin of `With`'s binder-phrase threading.
/// `None` for any payment shape besides the bare single sacrifice (the
/// `EventObject` render then falls back to the plain "it").
fn additional_cost_object_phrase(cost: &[deckmaste_semantics::CostComponent]) -> Option<String> {
    use deckmaste_semantics::Binder;
    use deckmaste_semantics::CostComponent;
    if let [CostComponent::With { binder, body }] = cost
        && let Binder::ChooseOne { filter, .. } = binder.as_ref()
        && let [CostComponent::Do(pa)] = body.0.as_ref()
        && let Action::Sacrifice(_, Reference::That(_)) = pa.as_ref()
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

/// The agent slot of a former-`PlayerAction` verb — `None` for a source verb,
/// an agent-silent verb (`Tap`/`Untap`/`PutCounters`/`RemoveCounters`/
/// `Reveal`/`RemoveDamage`, no `By` to check any more), or `Pay`/`Retarget`/
/// `CopySpell.retarget` (not this render layer's concern). Mirrors the
/// verb set `player_action`/`third_person_verb_phrase` render.
fn verb_agent(action: &Action) -> Option<&Reference> {
    match action {
        Action::ChangeLife(who, _)
        | Action::AddMana(who, _, _)
        | Action::Sacrifice(who, _)
        | Action::DrawCard(who)
        | Action::GetEmblem(who, _)
        | Action::GetDesignation(who, _)
        | Action::CastCopy(who, _)
        | Action::FlipCoins(who, _, _)
        | Action::RollDice(who, _, _)
        | Action::RollPlanarDie(who)
        | Action::WinGame(who)
        | Action::LoseGame(who)
        | Action::Cast(who, _, _)
        | Action::ChooseValue(who, _, _) => Some(who),
        Action::Create { agent, .. } => Some(agent),
        Action::CopySpell { controller, .. } => Some(controller),
        Action::Expanded(e) => verb_agent(&e.value),
        _ => None,
    }
}

/// The THIRD-PERSON verb phrase of a player action — the declarative-subject
/// tail ("loses 2 life", "discards a card") a non-`You` agent or an
/// `Each` player loop prefixes with its subject. A remembered verb-macro
/// expansion (`LosesLife(2)`) renders through its own template; the core verbs
/// carry structural fallbacks. `None` = no third-person phrase (the caller
/// falls back to the imperative render).
fn third_person_verb_phrase(action: &Action) -> Option<String> {
    match action {
        Action::Expanded(e) => {
            super::template::expanded(e, "it").or_else(|| third_person_verb_phrase(&e.value))
        }
        Action::ChangeLife(_, LifeOp::Down(c)) => {
            Some(format!("loses {} life", fragment::count(c)))
        }
        Action::ChangeLife(_, LifeOp::Up(c)) => Some(format!("gains {} life", fragment::count(c))),
        // Dictate of Karametra's "that land's controller adds one mana of
        // any type that land produced" — reuse `add_mana_text`'s clause,
        // stripped of its imperative "Add "/trailing period. A shape
        // `add_mana_text` itself declines (`[unrendered: …]`) has no
        // "Add "/"." to strip, so this falls through to `None` (the caller's
        // imperative fallback) rather than fabricating a phrase.
        Action::AddMana(_, count, production) => {
            let imperative = add_mana_text(count, production);
            imperative
                .strip_prefix("Add ")
                .and_then(|s| s.strip_suffix('.'))
                .map(|s| format!("adds {s}"))
        }
        _ => None,
    }
}

fn player_action(action: &Action, ctx: &Ctx) -> String {
    match action {
        // A remembered verb-macro expansion (`Mills(2)` under an explicit
        // agent): the imperative frame renders the expanded CORE action —
        // the third-person template belongs to the declarative subjects.
        Action::Expanded(e) => player_action(&e.value, ctx),
        // Life totals move in digits, with the explicit "you" subject the
        // oracle prints ("You gain 2 life.").
        Action::ChangeLife(_, LifeOp::Down(c)) => format!("You lose {} life.", fragment::count(c)),
        Action::ChangeLife(_, LifeOp::Up(c)) => format!("You gain {} life.", fragment::count(c)),
        // A mana ability's production ([CR#106.1]): "Add {W}.", "Add
        // {C}{C}.", "Add one mana of any color."
        Action::AddMana(_, count, production) => add_mana_text(count, production),
        // Rider-carrying token creation ("tapped and attacking") falls back
        // to the structural form until its surface lands (macro-first-wave).
        Action::Create {
            count,
            token: spec,
            riders,
            ..
        } if riders.is_empty() => create_text(count, spec, ctx),
        // A sacrifice ([CR#701.21]) — the patient is a single reference. A
        // chosen permanent ("sacrifice a creature", Fling) arrives pre-bound as
        // `Reference::That` from an enclosing `With`, which supplies the phrase.
        Action::Sacrifice(_, what) => format!("Sacrifice {}.", fragment::reference(what, ctx)),
        Action::GetDesignation(_, name) if name.as_ref() == "CitysBlessing" => {
            "You get the city's blessing.".to_string()
        }
        Action::GetDesignation(_, name) => format!("You get {name}."),
        // [CR#114.1]: "You get an emblem with «ability»." The emblem carries
        // only its abilities ([CR#114.3]) — render them through the same
        // `rules` walk a card face uses (a nameless, typeless view), quoted as
        // the emblem's text.
        Action::GetEmblem(_, abilities) => {
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
            // No period outside the closing quote — the printed convention
            // ends the sentence with the quoted ability's own period.
            format!(
                "You get an emblem with \"{}\"",
                super::rules(&view).join(" ")
            )
        }
        // [CR#608.2g]: "Cast that card." — the resolution-time cast verb. The
        // enclosing `May` supplies the "You may "/"If you don't, …" framing
        // (Chandra's "You may cast that card."); this renders the bare
        // instruction, its patient the surrounding effect's anaphor.
        // [CR#118.9,702.35a]: an alternative-cost cast ("by paying its madness
        // cost") appends the cost; the bare form ([CR#608.2g]) omits it.
        Action::Cast(_, what, for_cost) => match for_cost
            .as_ref()
            .and_then(|c| super::template::render_cost(c))
        {
            Some(cost) => format!("Cast {} by paying {cost}.", fragment::reference(what, ctx)),
            None => format!("Cast {}.", fragment::reference(what, ctx)),
        },
        // [CR#707.12]: "cast a copy of [source]" — the resolution-time
        // cast-a-copy delivery site (Wrenn and Realmbreaker, Reflection of
        // Kiki-Jiki, Ral, Storm Conduit's "you may cast a copy of it").
        // Exceptions ([CR#707.9]) append the shared ", except …" clause the
        // other three copy delivery sites carry.
        Action::CastCopy(_, spec) => format!(
            "Cast a copy of {}{}.",
            copy_source_phrase(&spec.source, ctx),
            copy_exceptions_clause(&spec.exceptions)
        ),
        // [CR#104.2b]: "You win the game." — immediate on resolution; the
        // `CantWin` suppression is engine-side, not part of the sentence.
        Action::WinGame(_) => "You win the game.".to_string(),
        // [CR#104.3e]: the loss twin of `WinGame`.
        Action::LoseGame(_) => "You lose the game.".to_string(),
        other => format!("[unrendered: {other:?}]."),
    }
}

/// [CR#608.2d]: "You may [effect]." with the optional "If you do, …" / "If you
/// don't, …" riders (Chandra's "You may cast that card. If you don't, ~ deals 2
/// damage to each opponent."). Each rider is a full sentence whose subject the
/// inner effect supplies, so it stands capitalized after the base clause.
fn render_may(m: &deckmaste_semantics::May, ctx: &Ctx) -> String {
    use std::fmt::Write as _;
    // [CR#118.12a,118.12,603,608]: the collapsed `MayPay`/`MustPay` shape —
    // `effect` is `Pay(cost)`, so `if_did`/`if_not` read as cost semantics
    // (the may-pay kicker / must-pay punisher) instead of the plain
    // "You may [effect]." wrapper below. Declines structurally (falls to
    // `[unrendered]`) if the cost has no symbol rendering (e.g. a `Do(...)`
    // verb cost).
    if let OneShotEffect::Act(Action::Pay(cost)) = m.effect.as_ref() {
        let payer = fragment::reference(&m.who, ctx);
        return match super::template::render_cost(&cost.0) {
            Some(c) => match (&m.if_did, &m.if_not) {
                // MustPay shape (no `if_did`): "[if_not] unless [who] pays
                // [cost]." — the resolution-time punisher (Mana Leak).
                (None, Some(if_not)) => {
                    let (_, _, pays) = payer_verbs(&payer);
                    format!(
                        "{} unless {payer} {pays} {c}.",
                        trim_period(&effect(if_not, ctx))
                    )
                }
                // MayPay shape: "[who] may pay [cost]. If [who] does,
                // [if_did][; if [who] doesn't, [if_not]]." — the
                // resolution-time kicker.
                (Some(if_did), if_not) => {
                    let (does, doesnt, _) = payer_verbs(&payer);
                    let did = super::ability::lower_first(&trim_period(&effect(if_did, ctx)));
                    let tail = if_not.as_ref().map_or_else(String::new, |not| {
                        let didnt = super::ability::lower_first(&trim_period(&effect(not, ctx)));
                        format!("; if {payer} {doesnt}, {didnt}")
                    });
                    fragment::capitalize(&format!(
                        "{payer} may pay {c}. If {payer} {does}, {did}{tail}."
                    ))
                }
                // Branchless `May(Pay(cost))` ([CR#608.2] optionality, no
                // kicker/punisher tail): "[who] may pay [cost]."
                (None, None) => fragment::capitalize(&format!("{payer} may pay {c}.")),
            },
            None => format!("[unrendered: {m:?}]."),
        };
    }
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
fn add_mana_text(count: &Count, production: &deckmaste_semantics::ManaProduction) -> String {
    use deckmaste_semantics::ManaProduction;
    use deckmaste_semantics::ManaSpec;
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
        // [CR#106.12a]: only the sound amount=1 shape is rendered; a larger
        // count declines structurally rather than guessing plural wording.
        ManaSpec::ProducedByEvent if n == 1 => {
            "Add one mana of any type that land produced.".to_string()
        }
        // `AmongColorsOf` needs the referenced object's own phrase, which is
        // unavailable here, so it declines structurally rather than guessing.
        ManaSpec::AmongColorsOf(_) | ManaSpec::ProducedByEvent => {
            format!("[unrendered: AddMana({count:?}, {production:?})].")
        }
    }
}

// ── Token creation
// ────────────────────────────────────────────────────────────

fn create_text(count: &Count, spec: &TokenSpec, ctx: &Ctx) -> String {
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
        // A token copy ([CR#707.1]): "Create a token that's a copy of X." /
        // "Create N tokens that are copies of X." — plural swaps the copula
        // AND the noun ("copies", not "copy"), matching the corpus (Rite of
        // Replication's overload, planeswalker ultimates that make several
        // token copies at once). Exceptions ([CR#707.9]) append the shared
        // ", except …" clause every copy delivery site carries.
        TokenSpec::Copy(spec) => {
            let plural = count.literal_value() != Some(1);
            let count_word = token_count_word(count);
            let noun = if plural { "tokens" } else { "token" };
            let source = copy_source_phrase(&spec.source, ctx);
            let exceptions = copy_exceptions_clause(&spec.exceptions);
            if plural {
                format!("Create {count_word} {noun} that are copies of {source}{exceptions}.")
            } else {
                format!("Create {count_word} {noun} that's a copy of {source}{exceptions}.")
            }
        }
    }
}

// ── Copy effects ([CR#707]) ─────────────────────────────────────────────────
//
// The shared `CopySpec` rendering every one of the four copy delivery sites
// (`TokenSpec::Copy` above, `EnterRider::AsCopy` in `enter_rider_phrase`,
// `Action::CastCopy` in `player_action`, and `StaticEffect::BecomesCopy`
// in `render/ability.rs`, which calls back into this section via
// `effect::copy_source_phrase`/`effect::copy_exceptions_clause`) prints after
// its own family verb — "create a token that's ~", "cast ~", "becomes ~",
// "enters as ~".

/// A copy effect's source ([CR#707.1]) as a noun phrase: a referenced object
/// reads through the ordinary reference grammar ("target creature", "that
/// creature", …); the copying object's OWN card — the graveyard/exile
/// self-copy keywords (Embalm/Eternalize copy the exiled card) — reads as the
/// plain anaphor "it", matching the real corpus ("Create a token that's a
/// copy of it.", Embalm's reminder text).
pub(super) fn copy_source_phrase(source: &CopySource, ctx: &Ctx) -> String {
    match source {
        CopySource::Object(r) => fragment::reference(r, ctx),
        CopySource::SelfCard => "it".to_string(),
    }
}

/// The ", except …" clause a copy effect's exceptions render as ([CR#707.9]),
/// appended directly after the source phrase every copy delivery site prints
/// ("a copy of target creature, except it's 7/7."). Empty for no exceptions.
pub(super) fn copy_exceptions_clause(exceptions: &[CopyException]) -> String {
    if exceptions.is_empty() {
        return String::new();
    }
    format!(
        ", except {}",
        join_and_list(&copy_exception_clauses(exceptions))
    )
}

/// One clause per `CopyException`, in list order — except a `Power(Set)` +
/// `Toughness(Set)` pair anywhere in the list collapses into ONE "n/m" clause
/// at the position of its first occurrence ([CR#707.9d]'s "except it's 7/7"),
/// mirroring `modifications_predicate`'s identical P/T-pair grouping for the
/// ordinary `Modify` static.
fn copy_exception_clauses(exceptions: &[CopyException]) -> Vec<String> {
    let pt = copy_pt_set_clause(exceptions);
    let mut pt_emitted = false;
    let mut clauses = Vec::new();
    for e in exceptions {
        match e {
            CopyException::Modify(m) => match peel_modification(m) {
                Modification::Power(NumericOp::Set(_))
                | Modification::Toughness(NumericOp::Set(_)) => {
                    if !pt_emitted {
                        if let Some(ref clause) = pt {
                            clauses.push(clause.clone());
                        }
                        pt_emitted = true;
                    }
                }
                other => clauses.push(copy_modify_clause(other)),
            },
            CopyException::Retain(c) => clauses.push(copy_retain_clause(*c)),
            CopyException::AdditionalEffect(r) => clauses.push(copy_additional_effect_clause(r)),
        }
    }
    clauses
}

/// See through a `Modification`'s macro-provenance wrapper — the
/// `Modification` twin of `fragment::strip_expanded`'s `Predicate` treatment.
fn peel_modification(m: &Modification) -> &Modification {
    match m {
        Modification::Expanded(e) => peel_modification(&e.value),
        other => other,
    }
}

/// "it's n/m" from a `Power(Set)` + `Toughness(Set)` pair among a copy
/// exception list ([CR#707.9d]) — Quicksilver Gargantuan's "except it's
/// 7/7.", Volrath's "except it's 7/5 and it has this ability.". `None` if
/// neither axis is set; a lone axis still renders (unverified against a real
/// card, but a valid `CopySpec` value the total renderer must not decline).
fn copy_pt_set_clause(exceptions: &[CopyException]) -> Option<String> {
    let mut p: Option<i64> = None;
    let mut t: Option<i64> = None;
    for e in exceptions {
        let CopyException::Modify(m) = e else { continue };
        match peel_modification(m) {
            Modification::Power(NumericOp::Set(StatValue::Number(n))) => p = Some(i64::from(*n)),
            Modification::Toughness(NumericOp::Set(StatValue::Number(n))) => {
                t = Some(i64::from(*n));
            }
            _ => {}
        }
    }
    match (p, t) {
        (Some(p), Some(t)) => Some(format!("it's {p}/{t}")),
        (Some(p), None) => Some(format!("it's power {p}")),
        (None, Some(t)) => Some(format!("it's toughness {t}")),
        (None, None) => None,
    }
}

/// One `Modify` copy-exception's clause, excluding the P/T-`Set` pair
/// `copy_exception_clauses` already peeled off.
fn copy_modify_clause(m: &Modification) -> String {
    match m {
        // "in addition to its other types" ([CR#707.9b,707.9d]) — a card
        // type/supertype/subtype ADDED as part of the copy (Copy Artifact's
        // "except it's an enchantment in addition to its other types.",
        // Sakashima's Student's "except it's a Ninja in addition to its
        // other [creature] types."). Card types and supertypes print
        // lowercase; a subtype prints its proper-cased printed name.
        Modification::CardTypes(CollectionOp::Add(ident)) => {
            copy_type_add_clause(&ident.to_lowercase())
        }
        Modification::Supertypes(CollectionOp::Add(s)) => {
            copy_type_add_clause(&super::card::supertype_str(*s).to_lowercase())
        }
        Modification::Subtypes(CollectionOp::Add(ident)) => copy_type_add_clause(ident.as_str()),
        // "it's [color(s)]" ([CR#707.9d]) — a copy exception SETTING the color
        // outright (Embalm's "except it's a white Zombie", Eternalize's "except
        // it's a 4/4 black Zombie", generalized to the flat per-exception
        // "it's black"/"it's white" this grammar's exceptions clause joins). A
        // multi-color Set joins the colors ("it's white and blue"); an empty
        // Set (a copy made colorless) reads "it's colorless".
        Modification::Colors(CollectionOp::Set(colors)) => copy_colors_set_clause(colors),
        // "it has [ability]" ([CR#707.9a]) — Progenitor Mimic's "except it
        // has "At the beginning of your upkeep, …"", Quicksilver Gargantuan
        // sibling cards' "except it has flying."
        Modification::GainAbility(a) => format!("it has {}", copy_gained_ability_phrase(a)),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// "it's black" / "it's white and blue" / "it's colorless" — a copy exception
/// that SETS the copy's color(s) outright ([CR#707.9d], the Embalm/Eternalize
/// shape). The colors join with "and"; an empty Set is "colorless".
fn copy_colors_set_clause(colors: &[Color]) -> String {
    if colors.is_empty() {
        return "it's colorless".to_string();
    }
    let joined = colors
        .iter()
        .map(|&c| color_word(c))
        .collect::<Vec<_>>()
        .join(" and ");
    format!("it's {joined}")
}

/// "artifact"/"Spirit" + "in addition to its other types" — the shared tail
/// of every type-add copy exception.
fn copy_type_add_clause(word: &str) -> String {
    format!(
        "it's {} {word} in addition to its other types",
        article_for(word)
    )
}

/// A gained ability's phrase: a keyword prints its bare lowercase name ("it
/// has flying"); any other ability prints its rendered rules text in quotes
/// ("it has \"At the beginning of your upkeep, …\""), reusing the same
/// nameless/typeless `CardView` trick `Action::GetEmblem` renders an
/// emblem's abilities through.
fn copy_gained_ability_phrase(a: &Ability) -> String {
    if let Ability::Keyword(k) = a {
        return super::keyword::keyword_name(k).to_lowercase();
    }
    let view = super::CardView {
        name: "",
        mana_cost: None,
        supertypes: &[],
        types: &[],
        subtypes: &[],
        power: None,
        toughness: None,
        abilities: std::slice::from_ref(a),
    };
    format!("\"{}\"", super::rules(&view).join(" "))
}

/// "it doesn't copy its {characteristic}" ([CR#707.9c,707.9d]) — the
/// axis-retention exception (Vesuvan Doppelganger's "except it doesn't copy
/// that creature's color", generalized to the flat "its" possessive so the
/// clause reads the same regardless of how the source phrase itself renders).
fn copy_retain_clause(c: Characteristic) -> String {
    match characteristic_word(c) {
        Some(word) => format!("it doesn't copy its {word}"),
        None => format!("[unrendered: Retain({c:?})]"),
    }
}

/// The printed noun for a `Characteristic` axis, as it reads in "it doesn't
/// copy its {word}". `BasicLandTypes` is a derived/aggregate axis no copy
/// effect actually retains, so it has no established word and declines
/// structurally rather than guessing.
fn characteristic_word(c: Characteristic) -> Option<&'static str> {
    match c {
        Characteristic::Colors => Some("color"),
        Characteristic::Power => Some("power"),
        Characteristic::Toughness => Some("toughness"),
        Characteristic::Defense => Some("defense"),
        Characteristic::Name => Some("name"),
        Characteristic::ManaCost => Some("mana cost"),
        Characteristic::Types => Some("types"),
        Characteristic::Subtypes => Some("subtypes"),
        Characteristic::Supertypes => Some("supertypes"),
        Characteristic::BasicLandTypes => None,
    }
}

/// "it enters with N [kind] counters on it" ([CR#707.9e]) — Altered Ego's
/// "except it enters with X additional +1/+1 counters on it." (the
/// "additional" qualifier is the surrounding ETB-replacement framing's, not
/// this clause's — see the `EnterRider::AsCopy` doc). The only `EnterRider`
/// shape a real copy exception's additional effect uses; any other rider
/// declines structurally.
fn copy_additional_effect_clause(r: &EnterRider) -> String {
    match r {
        EnterRider::WithCounters(kind, count) => {
            format!("it enters with {} on it", counter_phrase(kind, count))
        }
        other => format!("[unrendered: AdditionalEffect({other:?})]"),
    }
}

/// "a" or "an" for `word` by its leading sound (vowel-letter heuristic) — the
/// `render/ability.rs` twin of this file's own copy-exception type-add
/// clause (that module's `article_for` is private to it).
fn article_for(word: &str) -> &'static str {
    match word.chars().next() {
        Some(c) if "aeiou".contains(c.to_ascii_lowercase()) => "an",
        _ => "a",
    }
}

/// "A" / "A and B" / "A, B, and C" — the Oxford-comma join a copy effect's
/// exception clauses use ([CR#707.9]), matching the corpus ("except it's
/// 1/1, it's a Spirit in addition to its other types, and it has flying.").
fn join_and_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [a] => a.clone(),
        [a, b] => format!("{a} and {b}"),
        _ => {
            let (last, rest) = items.split_last().expect("non-empty");
            format!("{}, and {last}", rest.join(", "))
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
        SemValue::X => "X",
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
    for color in t.color_indicator.iter() {
        parts.push(color_word(*color).to_string());
    }

    // Supertypes
    for s in t.supertypes.iter() {
        parts.push(super::card::supertype_str(*s).to_lowercase());
    }

    // Subtypes (proper-cased names)
    for s in t.subtypes.iter() {
        parts.push(s.name.to_string());
    }

    // Types
    for ty in t.types.iter() {
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
    use std::sync::Arc;

    use deckmaste_semantics::Ability;
    use deckmaste_semantics::Action;
    use deckmaste_semantics::Binder;
    use deckmaste_semantics::Count;
    use deckmaste_semantics::Destination;
    use deckmaste_semantics::Each;
    use deckmaste_semantics::LifeOp;
    use deckmaste_semantics::Modification;
    use deckmaste_semantics::OneShotEffect;
    use deckmaste_semantics::Predicate;
    use deckmaste_semantics::Quantity;
    use deckmaste_semantics::Reference;
    use deckmaste_semantics::Selection;
    use deckmaste_semantics::TargetSpec;
    use deckmaste_semantics::With;
    use deckmaste_semantics::Zone;

    use super::Ctx;
    use super::action;
    use super::copy_exceptions_clause;
    use super::create_text;
    use super::effect;
    use super::enter_rider_phrase;
    use super::player_action;

    /// The builtin plugin, loaded once — verb keyword actions now render via
    /// their macro TEMPLATE (the `Expanded` provenance the corpus carries),
    /// not a per-enum render arm, so tests build them through the real macro
    /// layer rather than raw ctors.
    fn kw(src: &str) -> OneShotEffect {
        use std::sync::LazyLock;
        static BUILTIN: LazyLock<deckmaste_plugin::plugin::Plugin> = LazyLock::new(|| {
            let root =
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
            deckmaste_plugin::plugin::Plugin::load(root).expect("load builtin plugin")
        });
        BUILTIN
            .macros
            .read_str(src)
            .unwrap_or_else(|e| panic!("expanding keyword action {src:?}: {e}"))
    }

    /// `DestroyNoRegen`'s rider pronoun is a number-aware PRONOMINAL re-mention
    /// (`${0:pro}`), not a hardcoded literal: a singular target renders the
    /// capitalized "It" ([CR#608.2d]) that opens the second sentence, yielding
    /// the exact oracle string the parser round-trips. Regression against the
    /// old baked-capital "It" literal — same output, principled machinery.
    #[test]
    fn destroy_no_regen_renders_capitalized_it_pronoun() {
        let e = kw("DestroyNoRegen(Target(0))");
        let targets = [TargetSpec::Target(Quantity::one(), Predicate::creature())];
        let named = std::cell::Cell::new(0);
        let ctx = Ctx {
            subject: "Doom Blade",
            targets: &targets,
            that: None,
            named: Some(&named),
        };
        assert_eq!(
            effect(&e, &ctx),
            "Destroy target creature. It can't be regenerated."
        );
    }

    /// The collapsed `May(Pay(cost))` shape agrees the payer's verb with its
    /// grammatical person ([CR#603,608,118.12a]): the default `you` payer
    /// takes second-person "do / don't / pay", a third-person payer ("that
    /// player") takes "does / doesn't / pays". Regression for the hardcoded
    /// third-person forms that rendered the ungrammatical "if you **does**,
    /// …" / "unless you **pays**".
    #[test]
    fn pay_clauses_agree_verb_person_with_payer() {
        use deckmaste_semantics::Cost;
        use deckmaste_semantics::CostComponent;
        use deckmaste_semantics::May;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let pay = || {
            Arc::new(OneShotEffect::Act(Action::Pay(Cost(
                vec![CostComponent::Mana("{1}".parse().unwrap())].into(),
            ))))
        };
        let draw = || Arc::new(kw("Draw(1)"));
        let lose = || {
            Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Down(Count::Literal(1)),
            )))
        };

        // -- MayPay shape: "[payer] may pay {1}. If [payer] do(es), draw a
        //    card; if [payer] do(esn't), [lose]." --
        let may_you = OneShotEffect::May(May {
            who: Reference::You,
            effect: pay(),
            if_did: Some(draw()),
            if_not: Some(lose()),
        });
        let rendered = effect(&may_you, &ctx);
        assert!(
            rendered.contains("If you do, ") && rendered.contains("; if you don't, "),
            "second-person MayPay shape: {rendered}"
        );
        assert!(
            !rendered.contains("you does") && !rendered.contains("you doesn't"),
            "no third-person -s for the `you` payer: {rendered}"
        );

        let may_them = OneShotEffect::May(May {
            who: Reference::EventActor,
            effect: pay(),
            if_did: Some(draw()),
            if_not: Some(lose()),
        });
        let rendered = effect(&may_them, &ctx);
        assert!(
            rendered.contains("If that player does, ")
                && rendered.contains("; if that player doesn't, "),
            "third-person MayPay shape: {rendered}"
        );

        // -- MustPay shape: "[if_not] unless [payer] pay(s) {1}." --
        let must_you = OneShotEffect::May(May {
            who: Reference::You,
            effect: pay(),
            if_did: None,
            if_not: Some(lose()),
        });
        let rendered = effect(&must_you, &ctx);
        assert!(
            rendered.contains("unless you pay {1}") && !rendered.contains("unless you pays"),
            "second-person MustPay shape: {rendered}"
        );

        let must_them = OneShotEffect::May(May {
            who: Reference::EventActor,
            effect: pay(),
            if_did: None,
            if_not: Some(lose()),
        });
        let rendered = effect(&must_them, &ctx);
        assert!(
            rendered.contains("unless that player pays {1}"),
            "third-person MustPay shape: {rendered}"
        );
    }

    /// Activated costs separate distinct components with `, ` while preserving
    /// the contiguous glyphs within a single mana component ([CR#602.1]).
    #[test]
    fn activated_cost_separates_components_but_not_mana_symbols() {
        use std::path::Path;

        use deckmaste_plugin::plugin::Plugin;
        use deckmaste_semantics::CostComponent;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let mana = |symbols: &str| CostComponent::Mana(symbols.parse().unwrap());
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let energy = plugin
            .macros
            .read_str("PayEnergy(2)")
            .expect("PayEnergy expands");

        assert_eq!(super::activated_cost(&[mana("{1}{W}")], &ctx), "{1}{W}",);
        assert_eq!(
            super::activated_cost(&[mana("{S}"), CostComponent::Tap], &ctx),
            "{S}, {T}",
        );
        assert_eq!(
            super::activated_cost(&[mana("{2}{W}"), energy, CostComponent::Tap], &ctx),
            "{2}{W}, Pay {E}{E}, {T}",
        );
    }

    /// A planeswalker loyalty ability's activation cost ([CR#606.4]) prints as
    /// the prefix `+N` / `−N` / `0` (the `−` is U+2212 MINUS SIGN), NOT the
    /// generic "put/remove a loyalty counter on ~" clause: the
    /// cost verb is `Do(PutCounters/RemoveCounters(This, LoyaltyCounter, N))`.
    /// A non-loyalty counter cost (or one on a non-`This` subject) keeps
    /// rendering generically — the loyalty prefix must not over-broaden the
    /// match.
    #[test]
    fn loyalty_cost_renders_oracle_prefix() {
        use deckmaste_semantics::CostComponent;
        use deckmaste_semantics::CounterRef;

        let ctx = Ctx {
            subject: "Jace Beleren",
            targets: &[],
            that: None,
            named: None,
        };
        let cost = |act: Action| super::activated_cost(&[CostComponent::do_action(act)], &ctx);
        let loyalty = || CounterRef::from("LoyaltyCounter");

        // +2: PutCounters(This, LoyaltyCounter, 2)
        assert_eq!(
            cost(Action::PutCounters(
                Reference::This,
                loyalty(),
                Count::Literal(2),
            )),
            "+2",
        );
        // −1: RemoveCounters(This, LoyaltyCounter, 1) — U+2212
        assert_eq!(
            cost(Action::RemoveCounters(
                Reference::This,
                loyalty(),
                Count::Literal(1),
            )),
            "\u{2212}1",
        );
        // −10: RemoveCounters(This, LoyaltyCounter, 10) — U+2212
        assert_eq!(
            cost(Action::RemoveCounters(
                Reference::This,
                loyalty(),
                Count::Literal(10),
            )),
            "\u{2212}10",
        );
        // 0: PutCounters(This, LoyaltyCounter, 0) — a zero-cost loyalty
        // ability (LoyaltyZero) prints "0", not "+0".
        assert_eq!(
            cost(Action::PutCounters(
                Reference::This,
                loyalty(),
                Count::Literal(0),
            )),
            "0",
        );

        // Regression: a NON-loyalty counter cost still renders generically —
        // the loyalty prefix must key on the "LoyaltyCounter" name.
        let generic = cost(Action::PutCounters(
            Reference::This,
            CounterRef::from("P1P1Counter"),
            Count::Literal(1),
        ));
        assert!(
            !matches!(generic.as_bytes().first(), Some(b'+' | b'-'))
                && !generic.starts_with('\u{2212}'),
            "non-loyalty counter cost must not get a loyalty prefix: {generic}"
        );
    }

    /// A mana ability's produced-mana forms render their oracle text: the
    /// single-color choice "Add {W} or {U}.", and the filterland multi-symbol
    /// run choice "Add {W}{W}, {W}{U}, or {U}{U}." ([CR#106.1b]).
    #[test]
    fn add_mana_renders_run_and_color_choices() {
        use deckmaste_semantics::Color::Blue;
        use deckmaste_semantics::Color::White;
        use deckmaste_semantics::ColorOrColorless;
        use deckmaste_semantics::ManaSpec;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let render = |spec: ManaSpec| {
            effect(
                &OneShotEffect::Act(Action::AddMana(
                    Reference::You,
                    Count::Literal(1),
                    spec.into(),
                )),
                &ctx,
            )
        };
        let w = || ColorOrColorless::Color(White);
        let u = || ColorOrColorless::Color(Blue);
        assert_eq!(
            render(ManaSpec::OneOf(vec![w(), u()].into())),
            "Add {W} or {U}."
        );
        assert_eq!(
            render(ManaSpec::OneOfRuns(
                vec![vec![w(), w()], vec![w(), u()], vec![u(), u()],].into()
            )),
            "Add {W}{W}, {W}{U}, or {U}{U}."
        );
        // The two-run form drops the Oxford comma.
        assert_eq!(
            render(ManaSpec::OneOfRuns(
                vec![vec![w(), w()], vec![u(), u()]].into()
            )),
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
            named: None,
        };

        let default = Action::deal_damage(Reference::Target(0), Count::Literal(3));
        assert_eq!(
            action(&default, &ctx),
            "Pouncer deals 3 damage to target creature."
        );

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

    /// The one-sided "bite" shape's render round-trip ([CR#120]; `Fight`'s
    /// [CR#701.14a] reciprocal-less half): `StatOf(This, Power)` prints "…
    /// equal to its power", faithfully round-tripping
    /// [`deckmaste_migrations::parsers::effect`]'s
    /// `deal_damage_bite_equal_to_its_power` parse ("~ deals damage equal
    /// to its power to target creature." — the corpus's actual word order:
    /// the variable-amount clause sits between "damage" and "to <target>",
    /// unlike a literal numeral). Regression for the ungrammatical "it's
    /// power" a naive `reference(This, ctx) + "'s"` composition would print
    /// inside a triggered/activated body (ctx subject "it").
    #[test]
    fn deal_damage_stat_of_this_power_renders_its_power() {
        use deckmaste_semantics::Stat;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let bite = Action::DealDamage(
            Reference::This,
            Count::StatOf(Reference::This, Stat::Power),
            Reference::Target(0),
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
        use deckmaste_semantics::Stat;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "Cinder Shade",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let bite = Action::DealDamage(
            Reference::This,
            Count::StatOf(Reference::This, Stat::Power),
            Reference::Target(0),
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
        use deckmaste_semantics::Stat;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: Some("the sacrificed creature"),
            named: None,
        };
        let other_ref = Action::DealDamage(
            Reference::This,
            Count::StatOf(
                Reference::That(deckmaste_semantics::Sort::Card),
                Stat::Power,
            ),
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
        use deckmaste_semantics::Anchor;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let top = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(action(&top, &ctx), "Put it on top of your library.");
        let bottom = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&bottom, &ctx),
            "Put it on the bottom of your library."
        );
    }

    /// [CR#400.3]: a non-graveyard target may be owned by another player, so
    /// its library destination uses "its owner's library."
    #[test]
    fn move_to_library_renders_targeted_top_and_bottom() {
        use deckmaste_semantics::Anchor;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let top = Action::Move(
            Reference::Target(0),
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&top, &ctx),
            "Put target creature on top of its owner's library."
        );
        let bottom = Action::Move(
            Reference::Target(0),
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&bottom, &ctx),
            "Put target creature on the bottom of its owner's library."
        );
    }

    /// Bounce possessive ([CR#400.3]/[CR#402.1]): a hand/library return prints
    /// "your X" only when the moved object is provably the controller's — a
    /// self-bounce (`This`) or a target scoped to a graveyard (cards in a
    /// graveyard are owned by that graveyard's player) — and "its owner's X"
    /// for a targeted permanent that could be an opponent's. Covers self,
    /// battlefield-target, and graveyard-target ownership cases.
    #[test]
    fn bounce_possessive_your_vs_owners() {
        use deckmaste_semantics::Anchor;
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::StatePredicate;

        // Self-bounce -> "your hand".
        let self_ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let self_hand = Action::Move(
            Reference::This,
            Destination::Zone(Zone::Hand),
            vec![].into(),
            None,
        );
        assert_eq!(action(&self_hand, &self_ctx), "Return it to your hand.");

        // Targeted permanent (battlefield) -> "its owner's hand".
        let battlefield = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let bf_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&battlefield),
            that: None,
            named: None,
        };
        let bf_hand = Action::Move(
            Reference::Target(0),
            Destination::Zone(Zone::Hand),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&bf_hand, &bf_ctx),
            "Return target creature to its owner's hand."
        );

        // Target scoped to a graveyard -> "your" (owned by its graveyard's
        // player); the possessive keeps "your library" even for a `Target`.
        let grave = TargetSpec::Target(
            Quantity::one(),
            Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
                    Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ]
                .into(),
            ),
        );
        let grave_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&grave),
            that: None,
            named: None,
        };
        let grave_lib = Action::Move(
            Reference::Target(0),
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
            vec![].into(),
            None,
        );
        let rendered = action(&grave_lib, &grave_ctx);
        assert!(
            rendered.contains("of your library."),
            "graveyard-scoped target stays \"your\": {rendered}"
        );
        assert!(
            !rendered.contains("owner"),
            "graveyard-scoped target is not owner-relative: {rendered}"
        );
    }

    /// The chosen-subject bounce ([CR#400.3]/[CR#402.1]) —
    /// `With(ChooseOne(filter), Move(That, Hand))` renders "Return a land you
    /// control to its owner's hand." (a permanent you control but may not own)
    /// and, with self-exclusion, "Return another creature you control to its
    /// owner's hand." — exercising `subject_phrase`'s "another" register (not
    /// "an other"). Distinct from the targeted bounce.
    #[test]
    fn chosen_subject_bounce_renders_its_owners_hand() {
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::Sort;

        let ctx = Ctx {
            subject: "Skyfisher",
            targets: &[],
            that: None,
            named: None,
        };
        let you = || Arc::new(Predicate::Ref(Reference::You));
        let chosen_hand = |filter| {
            OneShotEffect::With(With {
                binder: Binder::ChooseOne {
                    filter,
                    by: Reference::You,
                },
                body: Arc::new(OneShotEffect::Act(Action::Move(
                    Reference::That(Sort::Permanent),
                    Destination::Zone(Zone::Hand),
                    vec![].into(),
                    None,
                ))),
            })
        };

        let land_you_control = Predicate::And(
            vec![
                Predicate::r#type(deckmaste_semantics::Type::Land),
                Predicate::Relation(RelationPredicate::ControlledBy(you())),
            ]
            .into(),
        );
        assert_eq!(
            effect(&chosen_hand(land_you_control), &ctx),
            "Return a land you control to its owner's hand."
        );

        let another_creature = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(you())),
            ]
            .into(),
        );
        assert_eq!(
            effect(&chosen_hand(another_creature), &ctx),
            "Return another creature you control to its owner's hand."
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
        use deckmaste_semantics::EnterRider;
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::StatePredicate;

        let graveyard_creature = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
                Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        let target = TargetSpec::Target(Quantity::one(), graveyard_creature);
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let targeted = Action::Move(
            Reference::Target(0),
            Destination::Zone(Zone::Battlefield),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&targeted, &ctx),
            "Return target creature card from your graveyard to the battlefield."
        );

        // Bare "card" (no type qualifier).
        let bare_card = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
                Predicate::Relation(RelationPredicate::Owner(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        let bare_target = TargetSpec::Target(Quantity::one(), bare_card);
        let bare_ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&bare_target),
            that: None,
            named: None,
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
            named: None,
        };
        let self_move = Action::Move(
            Reference::This,
            Destination::Zone(Zone::Battlefield),
            vec![].into(),
            None,
        );
        assert_eq!(
            action(&self_move, &self_ctx),
            "Return Ashputtle from your graveyard to the battlefield."
        );

        // A rider-carrying battlefield return is a DIFFERENT family (exile,
        // not graveyard) and keeps its own phrasing, no "from your
        // graveyard" clause.
        let riders = Action::Move(
            Reference::That(deckmaste_semantics::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![EnterRider::UnderOwnersControl].into(),
            None,
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
            Reference::That(deckmaste_semantics::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![].into(),
            None,
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
        use deckmaste_semantics::CounterRef;
        use deckmaste_semantics::CounterSpec;

        let slot = || TargetSpec::Target(Quantity::one(), Predicate::creature());
        let targets = [slot(), slot()];
        let ctx = Ctx {
            subject: "it",
            targets: &targets,
            that: None,
            named: None,
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
        use deckmaste_semantics::Distribute;
        use deckmaste_semantics::Predicate;
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let divide = super::effect(
            &deckmaste_semantics::OneShotEffect::Distribute(Distribute {
                amount: Count::Literal(3),
                binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
                body: Arc::new(deckmaste_semantics::OneShotEffect::Act(
                    Action::deal_damage(Reference::It, Count::Allotment),
                )),
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
        use deckmaste_semantics::AdditionalCost;
        use deckmaste_semantics::Cost;
        use deckmaste_semantics::CostComponent;

        let ctx = Ctx {
            subject: "Fling",
            targets: &[],
            that: None,
            named: None,
        };
        let fling = super::effect(
            &deckmaste_semantics::OneShotEffect::AdditionalCost(AdditionalCost {
                // "sacrifice a creature" is now the choose-then-pay `With` cost
                // step: ChooseOne(Creature) binds `That`, then `Sacrifice(That)`.
                pay: Cost(
                    vec![CostComponent::With {
                        binder: Arc::new(Binder::ChooseOne {
                            filter: Predicate::creature(),
                            by: Reference::You,
                        }),
                        body: Cost(
                            vec![CostComponent::do_action(Action::Sacrifice(
                                Reference::You,
                                Reference::That(deckmaste_semantics::Sort::OfType(
                                    deckmaste_semantics::Type::Creature,
                                )),
                            ))]
                            .into(),
                        ),
                    }]
                    .into(),
                ),
                body: Arc::new(kw("Draw(1)")),
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
        let ctx = Ctx {
            subject: "Altar",
            targets: &[],
            that: None,
            named: None,
        };
        let with = OneShotEffect::With(With {
            binder: Binder::ChooseOne {
                filter: Predicate::creature(),
                by: Reference::You,
            },
            body: Arc::new(OneShotEffect::Act(Action::Sacrifice(
                Reference::You,
                Reference::That(deckmaste_semantics::Sort::OfType(
                    deckmaste_semantics::Type::Creature,
                )),
            ))),
        });
        assert_eq!(effect(&with, &ctx), "Sacrifice a creature.");
    }

    /// The imperative `Discard(N)` verb renders "Discard two cards." via its
    /// macro template ([CR#701.9a,601.2b]). (The choose-then-discard
    /// `With(Choose(N), Discard(That))` surface — where the binder contributes
    /// the "N cards" phrase to the body's anaphor — is a later reshape of
    /// discard's choice path; the `With`→`That` binding mechanism itself is
    /// covered by `with_choose_one_renders_sacrifice_a_creature`.)
    #[test]
    fn discard_many_renders_discard_two_cards() {
        let ctx = Ctx {
            subject: "Wheel",
            targets: &[],
            that: None,
            named: None,
        };
        assert_eq!(effect(&kw("Discard(2)"), &ctx), "Discard two cards.");
    }

    /// The library-search / tutor family's bespoke render ([CR#701.23a]) —
    /// the migrations parser's `With(SearchOne(filter), Sequentially([...]))`
    /// shape round-trips to the exact oracle sentence, both the
    /// hand-destination (reveal) and battlefield-destination (tapped) forms.
    #[test]
    fn search_library_renders_hand_and_battlefield_destinations() {
        use deckmaste_semantics::CharacteristicPredicate;
        use deckmaste_semantics::EnterRider;
        use deckmaste_semantics::Sort;
        use deckmaste_semantics::Supertype;

        let ctx = Ctx {
            subject: "Tutor",
            targets: &[],
            that: None,
            named: None,
        };
        let basic_land = || {
            Predicate::And(
                vec![
                    Predicate::r#type(deckmaste_semantics::Type::Land),
                    Predicate::Characteristic(CharacteristicPredicate::Supertype(Supertype::Basic)),
                ]
                .into(),
            )
        };
        let hand = OneShotEffect::With(With {
            binder: Binder::SearchOne {
                filter: basic_land(),
                by: Reference::You,
                whose: Reference::You,
                from: vec![Zone::Library].into(),
                if_none: None,
            },
            body: Arc::new(OneShotEffect::Sequentially(
                vec![
                    OneShotEffect::Act(Action::Reveal {
                        what: Reference::That(Sort::Card),
                        to: None,
                    }),
                    OneShotEffect::Act(Action::Move(
                        Reference::That(Sort::Card),
                        Destination::Zone(Zone::Hand),
                        vec![].into(),
                        None,
                    )),
                    OneShotEffect::Act(Action::Shuffle(Selection::LibraryOf(Reference::You))),
                ]
                .into(),
            )),
        });
        assert_eq!(
            effect(&hand, &ctx),
            "Search your library for a basic land card, reveal it, put it into your hand, then shuffle."
        );

        let battlefield = OneShotEffect::With(With {
            binder: Binder::SearchOne {
                filter: basic_land(),
                by: Reference::You,
                whose: Reference::You,
                from: vec![Zone::Library].into(),
                if_none: None,
            },
            body: Arc::new(OneShotEffect::Sequentially(
                vec![
                    OneShotEffect::Act(Action::Move(
                        Reference::That(Sort::Card),
                        Destination::Zone(Zone::Battlefield),
                        vec![EnterRider::Tapped].into(),
                        None,
                    )),
                    OneShotEffect::Act(Action::Shuffle(Selection::LibraryOf(Reference::You))),
                ]
                .into(),
            )),
        });
        assert_eq!(
            effect(&battlefield, &ctx),
            "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle."
        );
    }

    /// A bare `Subtype` filter (no parent-`Type` wrapper) renders as "a
    /// <Subtype> card", NOT "a <Subtype> <type> card" [CR#205.3m]: the parser
    /// never injects a parent type for a bare subtype (a Tribal card gives
    /// its printed creature subtype to a noncreature card, so "a Goblin
    /// card" must keep matching a Tribal Instant — Goblin), and the renderer
    /// mirrors that by never printing one back. Also covers the bare
    /// subtype-"or"-list register ("a Swamp or Mountain card" — ONE shared
    /// article, not "a Swamp card or a Mountain card").
    #[test]
    fn search_library_renders_bare_subtype_with_no_type_word() {
        use deckmaste_semantics::CharacteristicPredicate;
        use deckmaste_semantics::Sort;

        let ctx = Ctx {
            subject: "Tutor",
            targets: &[],
            that: None,
            named: None,
        };
        let subtype = |name: &'static str| {
            Predicate::Characteristic(CharacteristicPredicate::Subtype(
                deckmaste_semantics::SubtypeRef::named(deckmaste_semantics::Ident::new(name)),
            ))
        };
        let goblin = OneShotEffect::With(With {
            binder: Binder::SearchOne {
                filter: subtype("Goblin"),
                by: Reference::You,
                whose: Reference::You,
                from: vec![Zone::Library].into(),
                if_none: None,
            },
            body: Arc::new(OneShotEffect::Sequentially(
                vec![
                    OneShotEffect::Act(Action::Reveal {
                        what: Reference::That(Sort::Card),
                        to: None,
                    }),
                    OneShotEffect::Act(Action::Move(
                        Reference::That(Sort::Card),
                        Destination::Zone(Zone::Hand),
                        vec![].into(),
                        None,
                    )),
                    OneShotEffect::Act(Action::Shuffle(Selection::LibraryOf(Reference::You))),
                ]
                .into(),
            )),
        });
        assert_eq!(
            effect(&goblin, &ctx),
            "Search your library for a Goblin card, reveal it, put it into your hand, then shuffle."
        );

        let swamp_or_mountain = OneShotEffect::With(With {
            binder: Binder::SearchOne {
                filter: Predicate::Or(vec![subtype("Swamp"), subtype("Mountain")].into()),
                by: Reference::You,
                whose: Reference::You,
                from: vec![Zone::Library].into(),
                if_none: None,
            },
            body: Arc::new(OneShotEffect::Sequentially(
                vec![
                    OneShotEffect::Act(Action::Move(
                        Reference::That(Sort::Card),
                        Destination::Zone(Zone::Battlefield),
                        vec![].into(),
                        None,
                    )),
                    OneShotEffect::Act(Action::Shuffle(Selection::LibraryOf(Reference::You))),
                ]
                .into(),
            )),
        });
        assert_eq!(
            effect(&swamp_or_mountain, &ctx),
            "Search your library for a Swamp or Mountain card, put it onto the battlefield, then shuffle."
        );
    }

    /// The loot/rummage render round-trip ([CR#121.1,701.9b,608.2c]): the
    /// `Sequentially([Draw, Discard])` shape the migrations parser's
    /// `parse_draw_then_discard` production emits needs NO dedicated render
    /// arm — the existing `Sequentially` ", then" joiner (this file's
    /// `effect` match arm above) already renders it back to the exact oracle
    /// sentence, both orders (loot and rummage).
    #[test]
    fn sequentially_renders_loot_and_rummage() {
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let draw = || kw("Draw(1)");
        let discard = || kw("Discard(1)");
        let loot = OneShotEffect::Sequentially(vec![draw(), discard()].into());
        assert_eq!(effect(&loot, &ctx), "Draw a card, then discard a card.");
        let rummage = OneShotEffect::Sequentially(vec![discard(), draw()].into());
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
        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        // A group verb on the per-element `It` → the collective sentence.
        let destroy = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
            effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
        });
        assert_eq!(effect(&destroy, &ctx), "Destroy each creature.");
        // A body the collapse does not recognise → the per-element form.
        let gain = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
            effect: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::You,
                LifeOp::Up(Count::Literal(1)),
            ))),
        });
        assert_eq!(effect(&gain, &ctx), "For each creature, you gain 1 life.");
        // Set-wide counter placement ([CR#122.1,608.2d]) collapses to the
        // "Put … on each <group>." surface, the render half of the migrations
        // parser's `on each <subject>` arm.
        let counters = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::creature())),
            effect: Arc::new(OneShotEffect::Act(Action::PutCounters(
                Reference::It,
                deckmaste_semantics::CounterRef::from("P1P1Counter"),
                Count::Literal(1),
            ))),
        });
        assert_eq!(
            effect(&counters, &ctx),
            "Put a +1/+1 counter on each creature."
        );
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

        use deckmaste_plugin::plugin::Plugin;

        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let ctx = Ctx {
            subject: "~",
            targets: &[],
            that: None,
            named: None,
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
        use deckmaste_semantics::AggregateOp;
        use deckmaste_semantics::Countable;
        use deckmaste_semantics::ObjectKind;
        use deckmaste_semantics::PlayerAttr;
        use deckmaste_semantics::Projection;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let highest_life = Count::Aggregate(
            AggregateOp::MaxOf,
            Projection {
                of: Countable::Players(Arc::new(Predicate::Kind(ObjectKind::Player))),
                by: Arc::new(Count::PlayerStatOf(Reference::It, PlayerAttr::Life)),
            },
        );
        let set_life = OneShotEffect::Each(Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
            effect: Arc::new(OneShotEffect::Act(Action::ChangeLife(
                Reference::It,
                LifeOp::Set(highest_life),
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
        let ctx = Ctx {
            subject: "Scavenger",
            targets: &[],
            that: None,
            named: None,
        };
        let exile = Action::Move(
            Reference::This,
            Destination::Zone(Zone::Exile),
            vec![].into(),
            None,
        );
        assert_eq!(action(&exile, &ctx), "Exile Scavenger.");
    }

    /// Graveyard-hate exile ([CR#701.13a]/[CR#400.7]) — the migrations
    /// `parse_exile_target` production's `any_graveyard_card_filter` shape —
    /// round-trips with the "from a graveyard" clause appended, both typed
    /// and bare "card"; the plain (non-graveyard) exile production, sharing
    /// the identical `Move(It, Exile)` shape, must NOT pick up the clause.
    #[test]
    fn exile_target_card_from_a_graveyard_round_trips() {
        use deckmaste_semantics::StatePredicate;

        let graveyard_creature = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::State(StatePredicate::InZone(Zone::Graveyard)),
            ]
            .into(),
        );
        let target = TargetSpec::Target(Quantity::one(), graveyard_creature);
        let ctx = Ctx {
            subject: "it",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let exiled = Action::Move(
            Reference::Target(0),
            Destination::Zone(Zone::Exile),
            vec![].into(),
            None,
        );
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
            named: None,
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
            named: None,
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

        use deckmaste_plugin::plugin::Plugin;

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
            named: None,
        };
        assert_eq!(
            effect(&parsed, &ctx),
            "Shuffle your graveyard into your library."
        );
    }

    /// [CR#701.27a]: "Transform ~." — flip a transforming DFC to its other
    /// face.
    #[test]
    fn renders_transform() {
        let ctx = Ctx {
            subject: "~",
            targets: &[],
            that: None,
            named: None,
        };
        assert_eq!(
            action(&Action::Transform(Reference::This), &ctx),
            "Transform ~."
        );
    }

    // ── Copy effects ([CR#707]) ─────────────────────────────────────────────

    fn target_creature_ctx(target: &TargetSpec) -> Ctx<'_> {
        Ctx {
            subject: "it",
            targets: std::slice::from_ref(target),
            that: None,
            named: None,
        }
    }

    /// `TokenSpec::Copy` ([CR#707.1]): "Create a token that's a copy of
    /// target creature." — the singular token-copy delivery site, no
    /// exceptions.
    #[test]
    fn token_copy_renders_create_a_token_thats_a_copy() {
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;
        use deckmaste_semantics::TokenSpec;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = target_creature_ctx(&target);
        let spec = TokenSpec::Copy(
            CopySpec {
                source: CopySource::Object(Reference::Target(0)),
                exceptions: vec![],
            }
            .into(),
        );
        assert_eq!(
            create_text(&Count::Literal(1), &spec, &ctx),
            "Create a token that's a copy of target creature.",
            "singular token copy"
        );
    }

    /// The plural token-copy family swaps the copula AND the noun ("tokens
    /// that ARE COPIES of", never "tokens that's a copy of") — Rite of
    /// Replication's overload shape.
    #[test]
    fn token_copy_plural_renders_tokens_that_are_copies() {
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;
        use deckmaste_semantics::TokenSpec;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = target_creature_ctx(&target);
        let spec = TokenSpec::Copy(
            CopySpec {
                source: CopySource::Object(Reference::Target(0)),
                exceptions: vec![],
            }
            .into(),
        );
        assert_eq!(
            create_text(&Count::Literal(5), &spec, &ctx),
            "Create five tokens that are copies of target creature.",
            "plural token copy"
        );
    }

    /// `Action::CastCopy` ([CR#707.12]): "Cast a copy of [source]." —
    /// the resolution-time cast-a-copy delivery site.
    #[test]
    fn cast_copy_renders_cast_a_copy_of_source() {
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = target_creature_ctx(&target);
        let act = Action::CastCopy(
            Reference::You,
            CopySpec {
                source: CopySource::Object(Reference::Target(0)),
                exceptions: vec![],
            },
        );
        assert_eq!(player_action(&act, &ctx), "Cast a copy of target creature.",);
    }

    /// `EnterRider::AsCopy` ([CR#707.5]): the rider phrase itself renders
    /// "enters as a copy of [source]" — the surrounding "You may have ~
    /// enter …" ETB-replacement framing is a separate, not-yet-built seam
    /// (see the rider's own doc comment), so this exercises the rider
    /// fragment directly rather than a full semantic ability sentence.
    #[test]
    fn enter_rider_as_copy_renders_enters_as_a_copy_of_source() {
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;
        use deckmaste_semantics::EnterRider;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = target_creature_ctx(&target);
        let riders = [EnterRider::AsCopy(CopySpec {
            source: CopySource::Object(Reference::Target(0)),
            exceptions: vec![],
        })];
        assert_eq!(
            enter_rider_phrase(&riders, &ctx),
            " enters as a copy of target creature",
        );
    }

    /// `CopySource::SelfCard` reads as the plain anaphor "it" — the
    /// graveyard/exile self-copy register Embalm/Eternalize use ("Create a
    /// token that's a copy of it.").
    #[test]
    fn copy_source_self_card_renders_it() {
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;
        use deckmaste_semantics::TokenSpec;

        let ctx = Ctx {
            subject: "it",
            targets: &[],
            that: None,
            named: None,
        };
        let spec = TokenSpec::Copy(
            CopySpec {
                source: CopySource::SelfCard,
                exceptions: vec![],
            }
            .into(),
        );
        assert_eq!(
            create_text(&Count::Literal(1), &spec, &ctx),
            "Create a token that's a copy of it."
        );
    }

    /// A `Power(Set)` + `Toughness(Set)` exception pair collapses into ONE
    /// "n/m" clause ([CR#707.9d]) — Quicksilver Gargantuan's "except it's
    /// 7/7."
    #[test]
    fn copy_exception_pt_set_pair_renders_slash_pt() {
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::NumericOp;
        use deckmaste_semantics::StatValue;

        let exceptions = vec![
            CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(7)))),
            CopyException::Modify(Modification::Toughness(NumericOp::Set(StatValue::Number(
                7,
            )))),
        ];
        assert_eq!(copy_exceptions_clause(&exceptions), ", except it's 7/7");
    }

    /// A card-type addition renders "except it's a[n] [type] in addition to
    /// its other types" ([CR#707.9b], Copy Artifact/Phyrexian Metamorph); a
    /// subtype addition prints its proper-cased name (Sakashima's Student).
    #[test]
    fn copy_exception_type_add_renders_in_addition_to_its_other_types() {
        use deckmaste_semantics::CollectionOp;
        use deckmaste_semantics::CopyException;

        let card_type = vec![CopyException::Modify(Modification::CardTypes(
            CollectionOp::Add("Artifact".into()),
        ))];
        assert_eq!(
            copy_exceptions_clause(&card_type),
            ", except it's an artifact in addition to its other types"
        );

        let subtype = vec![CopyException::Modify(Modification::Subtypes(
            CollectionOp::Add("Spirit".into()),
        ))];
        assert_eq!(
            copy_exceptions_clause(&subtype),
            ", except it's a Spirit in addition to its other types"
        );
    }

    /// `GainAbility` with a keyword prints the bare lowercase keyword name
    /// ([CR#707.9a]) — "except it has flying."-style sibling cards.
    #[test]
    fn copy_exception_gain_ability_keyword_renders_it_has_keyword() {
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::KeywordAbility;

        let exceptions = vec![CopyException::Modify(Modification::GainAbility(Arc::new(
            Ability::Keyword(KeywordAbility::Trample),
        )))];
        assert_eq!(
            copy_exceptions_clause(&exceptions),
            ", except it has trample"
        );
    }

    /// `Modification::Colors(Set(...))` ([CR#707.9d]) — a copy exception that
    /// SETS the copy's color renders "it's [color]" (Embalm's "except it's
    /// white", Eternalize's "except it's black"), not the `[unrendered]`
    /// catch-all it hit before this arm existed. Empty Set → "it's colorless";
    /// a multi-color Set joins the colors.
    #[test]
    fn copy_exception_colors_set_renders_its_color() {
        use deckmaste_semantics::CollectionOp;
        use deckmaste_semantics::Color;
        use deckmaste_semantics::CopyException;

        assert_eq!(
            copy_exceptions_clause(&[CopyException::Modify(Modification::Colors(
                CollectionOp::Set(vec![Color::Black].into())
            ))]),
            ", except it's black"
        );
        assert_eq!(
            copy_exceptions_clause(&[CopyException::Modify(Modification::Colors(
                CollectionOp::Set(vec![Color::White].into())
            ))]),
            ", except it's white"
        );
        assert_eq!(
            copy_exceptions_clause(&[CopyException::Modify(Modification::Colors(
                CollectionOp::Set(vec![Color::White, Color::Blue].into())
            ))]),
            ", except it's white and blue"
        );
        assert_eq!(
            copy_exceptions_clause(&[CopyException::Modify(Modification::Colors(
                CollectionOp::Set(vec![].into())
            ))]),
            ", except it's colorless"
        );
    }

    /// The full Eternalize-shape exception list ([CR#702.129a,707.9d]) renders
    /// end-to-end — proving the new Colors arm slots into the Oxford-comma
    /// join beside the P/T-pair collapse and the subtype-add clause, with no
    /// `[unrendered]` fragment. (Per-exception grammar output — this need not
    /// collapse to a real card's "black 4/4 Zombie" phrasing, a separate
    /// nicety out of scope.)
    #[test]
    fn copy_exceptions_eternalize_shape_renders_end_to_end() {
        use deckmaste_semantics::CollectionOp;
        use deckmaste_semantics::Color;
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::NumericOp;
        use deckmaste_semantics::StatValue;

        let exceptions = vec![
            CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(4)))),
            CopyException::Modify(Modification::Toughness(NumericOp::Set(StatValue::Number(
                4,
            )))),
            CopyException::Modify(Modification::Colors(CollectionOp::Set(
                vec![Color::Black].into(),
            ))),
            CopyException::Modify(Modification::Subtypes(CollectionOp::Add("Zombie".into()))),
        ];
        assert_eq!(
            copy_exceptions_clause(&exceptions),
            ", except it's 4/4, it's black, and it's a Zombie in addition to its other types"
        );
    }

    /// `Retain(Colors)` ([CR#707.9c,707.9d]) — Vesuvan Doppelganger's "except
    /// it doesn't copy that creature's color", generalized to the flat "its"
    /// possessive this grammar's exceptions clause uses.
    #[test]
    fn copy_exception_retain_colors_renders_doesnt_copy_its_color() {
        use deckmaste_semantics::Characteristic;
        use deckmaste_semantics::CopyException;

        let exceptions = vec![CopyException::Retain(Characteristic::Colors)];
        assert_eq!(
            copy_exceptions_clause(&exceptions),
            ", except it doesn't copy its color"
        );
    }

    /// `AdditionalEffect(EnterRider::WithCounters)` ([CR#707.9e]) — Altered
    /// Ego's "except it enters with X additional +1/+1 counters on it."
    /// (this clause itself carries no "additional" — that qualifier belongs
    /// to the surrounding ETB-replacement framing, not the rider payload).
    #[test]
    fn copy_exception_additional_effect_with_counters_renders() {
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::EnterRider;

        let exceptions = vec![CopyException::AdditionalEffect(EnterRider::WithCounters(
            "P1P1Counter".into(),
            Count::Literal(2),
        ))];
        assert_eq!(
            copy_exceptions_clause(&exceptions),
            ", except it enters with two +1/+1 counters on it"
        );
    }

    /// Multiple exceptions join as an Oxford-comma list — the corpus's
    /// "except it's 1/1, it's a Spirit in addition to its other types, and
    /// it has flying." shape (a token-copy `except` with a P/T-Set pair, a
    /// subtype add, and a keyword grant).
    #[test]
    fn copy_exceptions_multiple_join_with_oxford_comma() {
        use deckmaste_semantics::CollectionOp;
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::KeywordAbility;
        use deckmaste_semantics::NumericOp;
        use deckmaste_semantics::StatValue;

        let exceptions = vec![
            CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(1)))),
            CopyException::Modify(Modification::Toughness(NumericOp::Set(StatValue::Number(
                1,
            )))),
            CopyException::Modify(Modification::Subtypes(CollectionOp::Add("Spirit".into()))),
            CopyException::Modify(Modification::GainAbility(Arc::new(Ability::Keyword(
                KeywordAbility::Trample,
            )))),
        ];
        assert_eq!(
            copy_exceptions_clause(&exceptions),
            ", except it's 1/1, it's a Spirit in addition to its other types, and it has trample"
        );
    }

    /// `StaticEffect::BecomesCopy` ([CR#707.4]) round-trips through the full
    /// `effect()` entry — a `Continuously`/`Until`-wrapped becomes-a-copy
    /// static, Volrath's "becomes a copy of target creature, except it's
    /// 7/5 and it has this ability."-family shape (minus the ability
    /// exception, which the `Ability` render family covers separately).
    #[test]
    fn becomes_copy_renders_via_static_effect() {
        use deckmaste_semantics::CopyException;
        use deckmaste_semantics::CopySource;
        use deckmaste_semantics::CopySpec;
        use deckmaste_semantics::NumericOp;
        use deckmaste_semantics::StatValue;
        use deckmaste_semantics::StaticEffect;

        let target = TargetSpec::Target(Quantity::one(), Predicate::creature());
        let ctx = Ctx {
            subject: "Volrath",
            targets: std::slice::from_ref(&target),
            that: None,
            named: None,
        };
        let e = StaticEffect::BecomesCopy(
            Reference::This,
            CopySpec {
                source: CopySource::Object(Reference::Target(0)),
                exceptions: vec![
                    CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(
                        7,
                    )))),
                    CopyException::Modify(Modification::Toughness(NumericOp::Set(
                        StatValue::Number(5),
                    ))),
                ],
            },
        );
        assert_eq!(
            super::super::ability::static_effect(&e, &ctx),
            Some("Volrath becomes a copy of target creature, except it's 7/5.".to_string()),
        );
    }
}
