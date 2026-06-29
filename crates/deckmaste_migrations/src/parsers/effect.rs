//! The reusable effect-clause sub-parser: one normalized oracle effect
//! sentence -> the target declarations + body RON that any ability frame
//! (`Spell` now; triggered/activated later) wraps. Frame-agnostic by design,
//! so every frame parser shares one effect grammar. Targeting lives in the
//! announce list [CR#115.1]; distributive "each" is a resolution-time
//! selection [CR#608.2d].

use crate::parsers::count;
use crate::parsers::filter;
use crate::parsers::modify;
use crate::parsers::modify::strip_prefix_ci;
use crate::resolve::ResolveCtx;

/// One parsed effect clause: `TargetSpec` RON fragments to declare on the
/// frame (empty when the effect targets nothing), and the `Effect`/`Action`
/// body RON, which references any declared targets as `Target(0)`, `Target(1)`…
pub(super) struct ParsedEffect {
    pub(super) targets: Vec<String>,
    pub(super) effect: String,
}

/// Parses one normalized effect line into a [`ParsedEffect`], or `None` to
/// decline. Productions are tried in order; the first match wins. `parse_if`
/// leads (it folds a base sentence + conditional override into one
/// `Effect::If`); the bespoke productions follow (they encode targeting/scope
/// the bare macro templates can't carry); an `Effect`-kind macro template
/// ([`parse_macro_effect`]) is the final fallthrough, so keyword-action lines
/// (`investigate.`, `scry 2.`) route back to the macro whose template renders
/// them. [`ResolveCtx`] carries the reverse template index that the fallthrough
/// (and the conditional's condition-phrase lookup) consults.
pub(super) fn parse_clause(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    parse_if(line, ctx)
        .or_else(|| parse_may(line, ctx))
        .or_else(|| parse_deal_damage(line))
        .or_else(|| parse_draw(line))
        .or_else(|| parse_lose_life(line))
        .or_else(|| parse_gain_life(line))
        .or_else(|| parse_counter(line))
        .or_else(|| parse_put_counters(line, ctx))
        .or_else(|| parse_return_to_hand(line))
        .or_else(|| parse_tap_untap(line))
        .or_else(|| parse_destroy(line))
        .or_else(|| parse_sacrifice(line))
        .or_else(|| parse_attach(line))
        .or_else(|| parse_pump(line))
        .or_else(|| parse_create_predefined_token(line))
        .or_else(|| parse_create_token(line))
        .or_else(|| parse_macro_effect(line, ctx))
}

/// The final effect-clause fallthrough: route the whole clause back to the
/// `Effect`-kind macro whose `template` renders it — the settled
/// "parse-via-macros" direction. This is what lets keyword-action macros
/// (`investigate`, and its slot-bearing kin) stand as effect bodies in any
/// shell (ETB trigger / activated / spell) without a bespoke `parse_<action>`
/// per action. The clause's trailing period is stripped (templates carry the
/// sentence body, not its punctuation); a successful match must consume the
/// WHOLE body (no trailing junk), and declares no targets (a keyword action
/// targets nothing in its own right — any targeting lives in an outer shell).
/// Nullary templates (`investigate`) route through the bare-emittable index;
/// slot-bearing templates (`scry ${0}`) fill each `${i}` via the typed slot
/// readers, mirroring the whole-line keyword-template parser.
fn parse_macro_effect(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?.trim();
    // Nullary (param-less) action macro — `investigate`.
    if let Some(m) = ctx.index.match_kind("Effect", body)
        && m.consumed == body.len()
    {
        return Some(ParsedEffect {
            targets: Vec::new(),
            effect: m.macro_name.to_string(),
        });
    }
    // Slot-bearing action macro — `scry ${0}`, with each `${i}` slot read by
    // the typed reader.
    if let Some(m) = ctx.index.match_with("Effect", body, macro_slot_reader)
        && m.consumed == body.len()
    {
        return Some(ParsedEffect {
            targets: Vec::new(),
            effect: m.invocation,
        });
    }
    None
}

/// Read one `Effect`-macro template slot of declared type `ty` from the rest of
/// the clause. The slot is the line's tail in the action shapes modeled here
/// (`scry 2` — a `Count` magnitude at the end; `regenerate ~` — a `Reference`
/// subject), so a successful read consumes all of `input`. `Count` and the
/// self-reference forms of `Reference` are read; an unmodeled slot type (or a
/// `Reference` that isn't a self-reference) declines, failing the whole
/// template cleanly.
fn macro_slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    match ty {
        // A bare numeral count word — `scry two`, `mill 3`. Emitted as a bare
        // numeral (reader-sugar for `Count::Literal`), matching the sibling
        // `Draw`/`Create` count productions.
        "Count" => Some((number_word(input.trim())?.to_string(), input.len())),
        // An object reference. Only the self-reference forms are modeled here:
        // the `~` sigil (and the `it` / `this creature` anaphors that survive
        // when the upstream `~` rewrite didn't fire) name the object the ability
        // is printed on ([CR#201.5]) — `This`. A `target …` / `enchanted …`
        // reference would need a target declaration or an attachment ref hoisted
        // onto the frame, which a slot reader can't do (it returns only `(arg,
        // consumed)`); those decline, leaving the clause for a later production.
        "Reference" => self_reference(input.trim()).map(|r| (r, input.len())),
        _ => None,
    }
}

/// A self-reference phrase -> the `Reference::This` RON, or `None` for any
/// other reference. The `~` sigil is the normalized self-ref
/// ([`crate::extract`] rewrites a card's by-name self-references to it); the
/// bare anaphors `it` / `this creature` are the un-rewritten generic forms that
/// occasionally survive normalization. All three name the printed-on object
/// ([CR#201.5]) -> `This`.
fn self_reference(phrase: &str) -> Option<String> {
    matches!(phrase, "~" | "it" | "this creature").then(|| "This".to_owned())
}

/// `<base>. If <condition>, [instead] <override> [instead].` -> a within-effect
/// conditional: `If(condition: <cond>, then: <override>, otherwise: <base>)`.
/// This is later text modifying earlier text in one resolving effect
/// ([CR#608.2c] "read the whole text and apply the rules of English") — NOT a
/// triggered-ability intervening "if" ([CR#603.4]). The conditional sentence's
/// effect is the `then` branch (taken when the condition holds); the leading
/// base sentence is the `otherwise` (the default when it doesn't) — matching
/// the rulings on the Ascend reader cards (e.g. Golden Demise, Secrets of the
/// Golden City), where "if you have the city's blessing, … instead" swaps the
/// base for the override at resolution.
///
/// Both branches re-enter [`parse_clause`], so the whole production declines if
/// either branch isn't itself parseable, or if the condition phrase isn't a
/// grounded condition. The condition is read the parse-via-macros way: the
/// phrase is routed to the `Condition`-kind macro whose `template` renders it
/// (e.g. `you have the city's blessing` -> `YouHaveTheCitysBlessing`, authored
/// in `plugins/builtin/macros/condition/`), and the emitted RON carries that
/// macro INVOCATION, which the loader expands to its `Condition` body — exactly
/// as an `Effect` action macro stands as an effect body. New condition phrases
/// are added by authoring a `Condition` macro, with no parser change. v1
/// declines when EITHER branch declares targets: the two clauses share no
/// announce list here, so colliding `Target(0)` references can't be expressed —
/// a later production with a hoisted shared `Targeted` wrapper will lift that.
fn parse_if(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    // The conditional sentence opens at ". If " (the base sentence ends, the
    // "If" clause begins). Split on the LAST such boundary so a base sentence
    // that itself contains "if" survives; in practice these are single-base
    // single-override lines, so the last boundary is the only one.
    let (base, cond_sentence) = line.rsplit_once(". If ")?;
    let base = format!("{base}.");
    // "<phrase>, <override>." — the condition phrase runs to the first comma,
    // the override clause follows.
    let cond_sentence = cond_sentence.strip_suffix('.')?;
    let (phrase, override_clause) = cond_sentence.split_once(", ")?;
    // Route the phrase to its `Condition` macro; the match must consume the
    // WHOLE phrase (no trailing junk). The macro NAME is the parsed condition.
    let phrase = phrase.trim();
    let m = ctx.index.match_kind("Condition", phrase)?;
    if m.consumed != phrase.len() {
        return None;
    }
    let condition = m.macro_name.to_string();

    // "instead" may lead the override ("instead <override>") or trail it
    // ("<override> instead") — strip whichever side carries it, then re-attach
    // the sentence period the inner productions expect.
    let override_body = override_clause.trim();
    let override_body = strip_prefix_ci(override_body, "instead ")
        .or_else(|| override_body.strip_suffix(" instead"))
        .unwrap_or(override_body);
    let override_line = format!("{override_body}.");

    let base_parsed = parse_clause(&base, ctx)?;
    let then_parsed = parse_clause(&override_line, ctx)?;
    // v1: neither branch may declare targets (no shared announce list).
    if !base_parsed.targets.is_empty() || !then_parsed.targets.is_empty() {
        return None;
    }
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!(
            "If(condition: {condition}, then: {}, otherwise: {})",
            then_parsed.effect, base_parsed.effect
        ),
    })
}

/// `you may <effect>` -> the inner effect wrapped in a `May` frame
/// ([CR#603,608] optional-do): `May(effect: <inner>)`. The inner clause is
/// re-parsed by [`parse_clause`], carrying through any targets it declares — so
/// the whole production declines if the inner effect isn't itself parseable.
/// Every inner production accepts a lowercase (mid-sentence) lead, so the
/// stripped clause re-enters them directly. Case-insensitive lead ("You may"
/// opens a trigger effect; "you may" follows a comma).
fn parse_may(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    let inner = strip_prefix_ci(line, "you may ")?;
    let parsed = parse_clause(inner, ctx)?;
    Some(ParsedEffect {
        targets: parsed.targets,
        effect: format!("May(effect: {})", parsed.effect),
    })
}

/// `<subject> gets ±N/±N [and gain(s) <kw…>] until end of turn.` (and the
/// keyword-only `<subject> gain(s)/have/has <kw…> until end of turn.`) -> a
/// one-shot continuous effect ([CR#611.2]): `Continuously(effect: Modify(of:
/// <scope>, changes: [...]), duration: FixedUntil(EndOfTurn))`. The durational
/// marker is required — it's what makes this a one-shot continuous effect
/// rather than an always-on static anthem ([`crate::parsers::static_ability`],
/// which declines the marker). The ±N/±N + keyword-grant grammar is shared with
/// that anthem parser via [`modify`]; the changes are written inline
/// (`Modification` is not a macro kind, so no `AddPowerToughness` macro can
/// stand here). Subject: a target ("target creature" -> `Of(Target(0))` +
/// `TargetOne(<filter>)`), or a team/self class via the shared subject grammar
/// (`Matching`/`Of`).
fn parse_pump(line: &str) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?;
    // The required "until end of turn" marker may sit on EITHER side of a "for
    // each" count tail: "gets +1/+1 for each … until end of turn" (marker last)
    // or "gets +2/+0 until end of turn for each …" (marker mid, the
    // Piledriver/Rabblemaster order). Strip a trailing marker first; if it isn't
    // trailing, peel the count and strip the marker off the count's head.
    let (body, scaled) = if let Some(head) = body.strip_suffix(" until end of turn") {
        // Marker last — peel any "for each" count off what precedes it.
        match count::strip(head) {
            Some(c) if matches!(c.binder, count::Binder::ForEach) => (c.head, Some(c.count)),
            Some(_) => return None,
            None => (head, None),
        }
    } else {
        // Marker not trailing — it must precede a "for each" count tail.
        let c = count::strip(body)?;
        if !matches!(c.binder, count::Binder::ForEach) {
            return None;
        }
        (c.head.strip_suffix(" until end of turn")?, Some(c.count))
    };
    let changes = pump_changes(body, scaled.as_deref())?;
    let (scope, targets) = pump_scope(pump_subject(body)?)?;
    Some(ParsedEffect {
        targets,
        effect: format!(
            "Continuously(effect: Modify(of: {scope}, changes: [{}]), duration: FixedUntil(EndOfTurn))",
            changes.join(", ")
        ),
    })
}

/// The subject phrase of a pump body — everything before the first modify
/// marker.
fn pump_subject(body: &str) -> Option<&str> {
    modify::split_marker(body, &MODIFY_MARKERS).map(|(subj, _)| subj)
}

/// The changes list of a pump body: "±N/±N [and gain <kw…>]" (the P/T form,
/// with an optional keyword tail) or a bare keyword grant.
fn pump_changes(body: &str, scaled: Option<&str>) -> Option<Vec<String>> {
    if let Some((_, pred)) = modify::split_marker(body, &[" gets ", " get "]) {
        let (pt_part, grant_tail) = modify::split_grant_tail(pred);
        let mut changes = match scaled {
            Some(count) => modify::parse_pt_changes_scaled(pt_part.trim(), count)?,
            None => modify::parse_pt_changes(pt_part.trim())?,
        };
        if let Some(tail) = grant_tail {
            changes.extend(modify::parse_keyword_changes(tail)?);
        }
        return Some(changes);
    }
    // A keyword-only grant can't carry a numeric scaler.
    if scaled.is_some() {
        return None;
    }
    let (_, pred) = modify::split_marker(body, &[" gains ", " gain ", " have ", " has "])?;
    modify::parse_keyword_changes(pred)
}

/// Pump subject -> (`Modify` scope, target declarations). A "target <filter>"
/// subject scopes `Of(Target(0))` and declares `TargetOne(<filter>)`; the
/// source anaphor "it" (a self-pump trigger surface, e.g. "it gets +2/+0 …")
/// scopes `Of(This)`; a team/self class scopes via the shared subject grammar
/// with no target.
fn pump_scope(subj: &str) -> Option<(String, Vec<String>)> {
    if let Some(rest) = modify::strip_prefix_ci(subj.trim(), "target ") {
        let filter = filter::parse_phrase(rest)?;
        return Some((
            "Of(Target(0))".to_owned(),
            vec![format!("TargetOne({filter})")],
        ));
    }
    // "it" — the resolving source pumping itself (trigger anaphor); same scope as
    // a "~ gets …" self-pump.
    if subj.trim().eq_ignore_ascii_case("it") {
        return Some(("Of(This)".to_owned(), Vec::new()));
    }
    let filter = modify::subject_to_filter(subj)?;
    Some((modify::filter_to_scope(&filter), Vec::new()))
}

/// The markers that separate a pump subject from its predicate.
const MODIFY_MARKERS: [&str; 6] = [" gets ", " get ", " gains ", " gain ", " have ", " has "];

/// `Destroy target <subject>.` -> a `TargetOne(<filter>)` declaration (the
/// subject parsed by the shared [`object_target_filter`] grammar — the shared
/// [`filter`] phrase grammar, broadened with a type-noun disjunction for
/// "<type> or <type>" subjects like "artifact or enchantment") and the body
/// `Destroy(Target(0))` ([CR#701.8]). Only the single-target form; board wipes
/// ("destroy all/each …") are a later production. Declines when the subject
/// isn't filter-parseable. Case-insensitive lead, since the clause opens a
/// spell ("Destroy …") or follows a trigger comma ("…, destroy …").
fn parse_destroy(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "destroy ")?
        .strip_suffix('.')?
        .strip_prefix("target ")?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: "Destroy(Target(0))".to_owned(),
    })
}

/// Self-sacrifice productions ([CR#701.16], the "sacrifice it/~" family that
/// rides trigger bodies):
/// - `Sacrifice it.` / `Sacrifice ~.` -> `Sacrifice(This)`, no target. The "it"
///   anaphor in a trigger body is the resolving source ([CR#113.7] — the
///   permanent whose ability triggered), the same `This` the `~` self-reference
///   names; both normalize to the source. This is the resolution of "When ~
///   becomes the target …, sacrifice it." and "At the beginning of the end
///   step, sacrifice ~."
/// - `Sacrifice it/~ unless you pay <cost>.` -> the "unless you pay" toll
///   ([CR#118.12a]): the controller may pay the stated cost to keep the
///   permanent, else sacrifices it. Wrapped in an
///   [`Unless`](deckmaste_core::Effect::Unless) whose payer is the default
///   `You` (the controller — the trigger fires on your own upkeep). Only a
///   single mana cost is modeled (the overwhelmingly common upkeep tax); a
///   richer toll declines. Mirrors the kw-echo macro's `Unless(effect:
///   Sacrifice(This), unless: Param(0))` resolution shape.
///
/// A non-self sacrifice ("Sacrifice a creature", "Sacrifice another …") is a
/// chosen-permanent cost handled in the cost grammar, not here — this body
/// production fires only on the self anaphors.
fn parse_sacrifice(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "sacrifice ")?.strip_suffix('.')?;
    // The self anaphors: a bare "it"/"~", or one carrying an "unless you pay"
    // toll. The subject is the resolving source either way ("it" == "~" == the
    // permanent that triggered).
    let (subject, toll) = match body.split_once(' ') {
        None => (body, None),
        Some((subject, rest)) => (subject, Some(rest)),
    };
    if subject != "it" && subject != "~" {
        return None;
    }
    let Some(toll) = toll else {
        return Some(ParsedEffect {
            targets: Vec::new(),
            effect: "Sacrifice(This)".to_owned(),
        });
    };
    // "unless you pay <cost>." — the controller's optional mana toll.
    let cost = toll.strip_prefix("unless you pay ")?;
    let cost = crate::parsers::cost::parse_cost(cost, crate::parsers::cost::VariableMana::Decline)
        .ok()
        .flatten()?;
    if cost.len() != 1 {
        return None;
    }
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!(
            "Unless(effect: Sacrifice(This), unless: [{}])",
            cost.join(", ")
        ),
    })
}

/// `Attach it to target <subject>.` -> the uniform attachment verb
/// ([CR#701.3a], the one verb the whole Aura/Equipment/Fortification family
/// shares): the resolving source (`it` -> `This`) is attached to a single
/// chosen target. The body is the ETB resolution of "When ~ enters, attach it
/// to target creature you control." (a self-equipping artifact creature). The
/// subject after "target " is parsed by [`object_target_filter`]. The "it"
/// anaphor names the entering permanent (`This`); a non-"it" attachee declines
/// (no card demands a non-self attach body yet).
fn parse_attach(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "attach it to target ")?.strip_suffix('.')?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: "Attach(what: This, to: Target(0))".to_owned(),
    })
}

/// `Counter target spell[ unless its controller pays <cost>].` -> a
/// `TargetOne(Spell)` target on the stack and a `Counter(Target(0))` body
/// ([CR#701.6a]). The "unless its controller pays" rider wraps the counter in
/// an [`Unless`](deckmaste_core::Effect::Unless) ([CR#118.12a]): the spell's
/// controller (`who: ControllerOf(Target(0))`) may pay the stated cost to stop
/// the counter. Only the bare and the mana-tax riders parse; richer riders
/// (replacement clauses, "you may cast …", restricted spell filters) are later
/// productions. Case-insensitive lead (spell clause vs. trigger comma).
fn parse_counter(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "counter target spell")?.strip_suffix('.')?;
    let targets = vec!["TargetOne(Spell)".to_owned()];
    // Bare "Counter target spell."
    if body.is_empty() {
        return Some(ParsedEffect {
            targets,
            effect: "Counter(Target(0))".to_owned(),
        });
    }
    // "… unless its controller pays <cost>." — the spell's controller may pay
    // to avoid the counter. Only a single mana cost is modeled here (the
    // overwhelmingly common "{N}" tax); a "for each …" scaled tax declines.
    let cost = body.strip_prefix(" unless its controller pays ")?;
    let cost = crate::parsers::cost::parse_cost(cost, crate::parsers::cost::VariableMana::Decline)
        .ok()
        .flatten()?;
    if cost.len() != 1 {
        return None;
    }
    Some(ParsedEffect {
        targets,
        effect: format!(
            "Unless(effect: Counter(Target(0)), who: ControllerOf(Target(0)), unless: [{}])",
            cost.join(", ")
        ),
    })
}

// ---------------------------------------------------------------------------
// Counter-placement productions ([CR#122.1], the +1/+1 family). The placement
// verb is `PutCounters(<selection>, <CounterRef>, <count>)`; the counter KIND
// is resolved the parse-via-macros way — the "+1/+1 counter" / "-1/-1 counter"
// phrase routes to the `Counter`-kind macro whose `template` renders it, so the
// kind comes out as the macro NAME (`P1P1Counter`, `M1M1Counter`). An unmodeled
// kind (`+2/+2`, `+1/+0`) has no macro and declines cleanly, never minting a
// junk counter ident.
// ---------------------------------------------------------------------------

/// `Put <count> <kind> counter[s] on <where>.` -> a counter placement
/// ([CR#122.1]):
/// - `on target <subject>.` -> `PutCounters(Target(0), <kind>, <n>)` with a
///   `TargetOne(<filter>)` declaration (the subject parsed by the shared
///   [`object_target_filter`] grammar).
/// - `on it.` / `on ~.` -> `PutCounters(This, <kind>, <n>)`, no target — the
///   resolving source counters itself (the combat-damage trigger surface).
///
/// The kind is macro-resolved (so `-1/-1` -> `M1M1Counter` for free); fixed
/// counts only (a "for each"/`X`/"that many" count declines — a later scaled
/// production). Case-insensitive lead (spell clause vs. trigger comma).
fn parse_put_counters(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "put ")?.strip_suffix('.')?;
    // Split the counter clause from its destination at the LAST " on " (a
    // counter-kind phrase never contains " on ").
    let (counter_clause, dest) = body.rsplit_once(" on ")?;
    let (count, kind) = parse_counter_clause(counter_clause, ctx)?;
    // Destination -> (selection, target declarations).
    let (selection, targets) = match dest {
        // Self placement: the resolving source ("it" — a trigger anaphor — or
        // "~"). No target.
        "it" | "~" => ("This".to_owned(), Vec::new()),
        // Targeted placement: "target <subject>".
        _ => {
            let subject = dest.strip_prefix("target ")?;
            let filter = object_target_filter(subject)?;
            ("Target(0)".to_owned(), vec![format!("TargetOne({filter})")])
        }
    };
    Some(ParsedEffect {
        targets,
        effect: format!("PutCounters({selection}, {kind}, {count})"),
    })
}

/// `<count> <kind> counter[s]` (the clause before "on …") -> `(count RON, kind
/// RON)`. The count is a fixed cardinal emitted as a bare numeral
/// (reader-sugar for `Count::Literal`, like the sibling `Draw`/`Create`
/// productions); the kind is the `Counter`-kind macro name the "+1/+1 counter"
/// phrase resolves to. Declines a non-cardinal count (`X`, "that many") and an
/// unmodeled counter kind. Shared with the enters-with-counters replacement
/// production ([`crate::parsers::replacement`]).
pub(super) fn parse_counter_clause(clause: &str, ctx: &ResolveCtx) -> Option<(u32, String)> {
    let (count_word, rest) = clause.split_once(' ')?;
    let count = number_word(count_word)?;
    // Re-singularize the counter-noun so the singular macro template ("+1/+1
    // counter") matches regardless of the count's plurality ("two +1/+1
    // counters").
    let phrase = rest.strip_suffix('s').unwrap_or(rest);
    let kind = counter_kind(phrase, ctx)?;
    Some((count, kind))
}

/// A counter-kind phrase ("+1/+1 counter", "-1/-1 counter") -> the macro NAME
/// it resolves to (`P1P1Counter`, `M1M1Counter`), routed through the
/// `Counter`-kind reverse index. The match must consume the WHOLE phrase (no
/// trailing junk); an unmodeled kind declines.
fn counter_kind(phrase: &str, ctx: &ResolveCtx) -> Option<String> {
    let m = ctx.index.match_kind("Counter", phrase)?;
    (m.consumed == phrase.len()).then(|| m.macro_name.to_string())
}

/// Return-to-hand productions ([CR#400.7], the bounce family):
/// - `Return target <subject> to its owner's hand.` -> battlefield bounce via
///   the dedicated [`ReturnToHand`](deckmaste_core::Action::ReturnToHand) verb,
///   the subject parsed by [`object_target_filter`].
/// - `Return ~ to its owner's hand.` -> a self-bounce (`ReturnToHand(This)`),
///   no target — the effect body of `{cost}: Return ~ to its owner's hand.`
///   activated abilities (the cost is the activated-frame's job).
/// - `Return target <subject> card from your graveyard to your hand.` -> a
///   graveyard-to-hand recursion: a plain zone change ([CR#400.7]) of a card
///   you own in your graveyard, via `Move(Target(0), Hand)`. The subject is a
///   *card* (graveyard zone), so it's the card-type spelling
///   (`Type(Creature)`), not the battlefield-scoped `Creature` macro, scoped
///   `InZone(Graveyard)` + `Owner(Ref(You))`.
fn parse_return_to_hand(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "return ")?.strip_suffix('.')?;
    // Self-bounce: "Return ~/it to its owner's hand." — an activated-ability
    // effect ("~") or a trigger body whose "it" anaphor names the resolving
    // source ([CR#113.7]); both are the source permanent (`This`).
    if body == "~ to its owner's hand" || body == "it to its owner's hand" {
        return Some(ParsedEffect {
            targets: Vec::new(),
            effect: "ReturnToHand(This)".to_owned(),
        });
    }
    // Graveyard recursion: "target <subject> card from your graveyard to your
    // hand." — peeled first so the battlefield arm's "to its owner's hand"
    // suffix can't shadow it. "<subject> card" is the noun phrase; a bare "card"
    // (no type qualifier) leaves an empty subject.
    if let Some(noun) = body
        .strip_suffix(" from your graveyard to your hand")
        .and_then(|s| s.strip_prefix("target "))
    {
        // The trailing "card" terminator; the subject is whatever precedes it.
        let subject = noun
            .strip_suffix(" card")
            .or_else(|| (noun == "card").then_some(""))?;
        let card_filter = graveyard_card_filter(subject)?;
        return Some(ParsedEffect {
            targets: vec![format!("TargetOne({card_filter})")],
            effect: "Move(Target(0), Hand)".to_owned(),
        });
    }
    // Battlefield bounce: "target <subject> to its owner's hand."
    let subject = body
        .strip_suffix(" to its owner's hand")?
        .strip_prefix("target ")?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: "ReturnToHand(Target(0))".to_owned(),
    })
}

/// `Tap target <subject>.` / `Untap target <subject>.` -> the
/// [`Tap`](deckmaste_core::PlayerAction::Tap) /
/// [`Untap`](deckmaste_core::PlayerAction::Untap) verbs ([CR#701.26a..701.26b])
/// over a single target. The subject is parsed by [`object_target_filter`].
/// Riders ("It doesn't untap …", "It gets …") leave trailing text past the
/// period-terminated single sentence, so they decline cleanly here (each is a
/// later multi-clause production). Case-insensitive lead.
fn parse_tap_untap(line: &str) -> Option<ParsedEffect> {
    let (verb, rest) = if let Some(rest) = strip_prefix_ci(line, "tap target ") {
        ("Tap", rest)
    } else if let Some(rest) = strip_prefix_ci(line, "untap target ") {
        ("Untap", rest)
    } else {
        return None;
    };
    let subject = rest.strip_suffix('.')?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: format!("{verb}(Target(0))"),
    })
}

/// An object-target subject phrase -> its `Filter` RON. First the shared
/// [`filter`] phrase grammar (single head noun with adjectives), then a
/// type-noun disjunction fallback for "<type> or <type>[ or <type>]" subjects
/// (`OneOf([…])`) the single-head grammar can't carry — "artifact or
/// enchantment", "creature or planeswalker", "attacking or blocking creature".
fn object_target_filter(subject: &str) -> Option<String> {
    if let Some(f) = filter::parse_phrase(subject) {
        return Some(f);
    }
    type_disjunction(subject)
}

/// "<A> or <B>[ or <C>]" of type-noun (or status-qualified) members ->
/// `OneOf([…])`. Two shapes:
/// - a shared head noun with disjoined status adjectives: "attacking or
///   blocking creature" -> `AllOf([Creature, OneOf([Attacking, Blocking])])` —
///   the head noun trails the last member; the leading members are bare combat-
///   status adjectives. Tried first, since a status adjective ("attacking")
///   would otherwise be misread as a bare-subtype head.
/// - heterogeneous types: "artifact or enchantment" -> `OneOf([Type(Artifact),
///   Type(Enchantment)])`. Every member must be a plain type-noun phrase
///   (creature / artifact / land / …) — NOT a bare-subtype fallthrough, which
///   the strict head check rules out (so "Goblin or Elf" stays unmodeled rather
///   than minting wrong `Subtype` disjuncts).
fn type_disjunction(subject: &str) -> Option<String> {
    let parts: Vec<&str> = subject.split(" or ").map(str::trim).collect();
    if parts.len() < 2 {
        return None;
    }
    // Shared-head status disjunction: "<status> or <status> … <status> <head>".
    if let Some((last_adj, head)) = parts.last()?.split_once(' ')
        && let Some(head_filter) = type_noun_phrase(head)
    {
        let mut adjs: Vec<&str> = parts[..parts.len() - 1].to_vec();
        adjs.push(last_adj);
        if let Some(status) = adjs
            .iter()
            .map(|a| status_atom(a))
            .collect::<Option<Vec<_>>>()
        {
            return Some(format!(
                "AllOf([{head_filter}, OneOf([{}])])",
                status.join(", ")
            ));
        }
    }
    // Heterogeneous type-noun disjunction: every member is a plain type noun.
    let members: Option<Vec<String>> = parts.iter().map(|p| type_noun_phrase(p)).collect();
    Some(format!("OneOf([{}])", members?.join(", ")))
}

/// A bare type-noun phrase (a determiner-led single card type / `permanent`) ->
/// its head `Filter`, declining a bare-subtype fallthrough. Guards
/// [`type_disjunction`] so a disjunction member is a real type noun, never a
/// silently-minted `Subtype`. Strips a leading determiner ("a"/"an") the way
/// the shared phrase grammar does.
fn type_noun_phrase(phrase: &str) -> Option<String> {
    let phrase = strip_prefix_ci(phrase, "a ")
        .or_else(|| strip_prefix_ci(phrase, "an "))
        .unwrap_or(phrase)
        .trim();
    let filter = filter::parse_phrase(phrase)?;
    // A bare-subtype head renders as `AllOf([Permanent, Subtype(...)])` (or a
    // lone `Subtype(...)`); reject those — only true type nouns disjoin here.
    (!filter.contains("Subtype(")).then_some(filter)
}

/// A combat-status adjective -> its `Filter` status atom. The disjoinable
/// adjectives a shared-head target disjunction admits ("attacking or
/// blocking").
fn status_atom(word: &str) -> Option<String> {
    Some(match word {
        "attacking" => "Attacking".to_owned(),
        "blocking" => "Blocking".to_owned(),
        _ => return None,
    })
}

/// A graveyard-card subject (the noun before " card" in "<subject> card from
/// your graveyard") -> the `Filter` for a card you own in your graveyard:
/// `AllOf([<type>, InZone(Graveyard), Owner(Ref(You))])`. The type is the
/// card-type spelling (`Type(Creature)`, `OneOf([Type(Instant),
/// Type(Sorcery)])` for "instant or sorcery") — NOT the battlefield-scoped
/// macros, since a graveyard card is not a permanent. A bare "card" (no type)
/// is any card you own there.
fn graveyard_card_filter(subject: &str) -> Option<String> {
    let mut atoms: Vec<String> = Vec::new();
    if let Some(ty) = graveyard_card_type(subject) {
        atoms.push(ty);
    } else if !subject.is_empty() {
        // A type word the card-type grammar doesn't model -> decline (never
        // emit a junk filter).
        return None;
    }
    atoms.push("InZone(Graveyard)".to_owned());
    atoms.push("Owner(Ref(You))".to_owned());
    Some(format!("AllOf([{}])", atoms.join(", ")))
}

/// A graveyard-card type phrase -> its card-type `Filter` (`Type(Creature)`,
/// `OneOf([Type(Instant), Type(Sorcery)])`), or `None` for a bare "card" (no
/// type qualifier) or an unmodeled phrase. Card-type spelling via
/// [`filter::type_filter`], so the live matcher reads the printed card type,
/// not a battlefield-only macro.
fn graveyard_card_type(subject: &str) -> Option<String> {
    if subject.is_empty() {
        return None;
    }
    let members: Vec<&str> = subject.split(" or ").map(str::trim).collect();
    let types: Option<Vec<String>> = members
        .iter()
        .map(|m| filter::type_filter(&filter::singularize(m).to_ascii_lowercase()))
        .collect();
    let types = types?;
    Some(if types.len() == 1 {
        types.into_iter().next().unwrap()
    } else {
        format!("OneOf([{}])", types.join(", "))
    })
}

/// `~ deals N damage to <target>.` or `it deals N damage to <target>.` —
/// "it" case-insensitively, since it opens the clause after a cost colon
/// ("Sacrifice ~: It deals …") but follows a comma in trigger clauses.
fn parse_deal_damage(line: &str) -> Option<ParsedEffect> {
    let body = line
        .strip_prefix("~ deals ")
        .or_else(|| strip_prefix_ci(line, "it deals "))?
        .strip_suffix('.')?;
    let (body, dynamic) = match count::strip(body) {
        Some(c) => (c.head, Some(c)),
        None => (body, None),
    };
    let (amount, tail) = match &dynamic {
        // "equal to the number of …": the head is "damage to <target>" — no
        // amount word; the count IS the amount.
        Some(c) if matches!(c.binder, count::Binder::EqualTo) => {
            (c.count.clone(), body.strip_prefix("damage to ")?)
        }
        _ => {
            let (amt, tail) = body.split_once(" damage to ")?;
            let amount = match &dynamic {
                None => number_word(amt)?.to_string(),
                Some(c) => match &c.binder {
                    count::Binder::Variable(var) => {
                        if amt != var {
                            return None;
                        }
                        c.count.clone()
                    }
                    count::Binder::ForEach => {
                        if number_word(amt)? != 1 {
                            return None;
                        }
                        c.count.clone()
                    }
                    count::Binder::EqualTo => unreachable!("handled above"),
                },
            };
            (amount, tail)
        }
    };
    let (targets, selection) = damage_target(tail)?;
    Some(ParsedEffect {
        targets,
        effect: format!("DealDamage({selection}, {amount})"),
    })
}

/// `Draw N card(s).` — no targets. Case-insensitive lead ("draw" or "Draw").
fn parse_draw(line: &str) -> Option<ParsedEffect> {
    let rest = strip_prefix_ci(line, "draw ")?.strip_suffix('.')?;
    // Plural first so "two cards" doesn't strip to "two card".
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = number_word(count)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("Draw({n})"),
    })
}

/// `You lose N life[ for each <filter>].` — the ability's controller loses
/// life. No targets.
fn parse_lose_life(line: &str) -> Option<ParsedEffect> {
    let amount = life_amount(strip_prefix_ci(line, "you lose ")?)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("LoseLife({amount})"),
    })
}

/// `You gain N life[ for each <filter>].` — the ability's controller gains
/// life. No targets.
fn parse_gain_life(line: &str) -> Option<ParsedEffect> {
    let amount = life_amount(strip_prefix_ci(line, "you gain ")?)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("GainLife({amount})"),
    })
}

/// `N life[ for each <filter>].` -> the amount RON: a bare numeral for a fixed
/// count (reader-sugar for `Count::Literal`, like `Draw`), or a `CountOf(...)`
/// for a "1 life for each <filter>" dynamic tail. "life" is invariant (never
/// pluralized). `None` if the count word or the shape is off — and a non-unit
/// base under "for each" (no `Count` product form) declines.
fn life_amount(text: &str) -> Option<String> {
    let body = text.strip_suffix('.')?;
    match count::strip(body) {
        // "1 life for each <filter>": the count IS the amount (base must be 1).
        Some(c) if matches!(c.binder, count::Binder::ForEach) => {
            (number_word(c.head.strip_suffix(" life")?)? == 1).then_some(c.count)
        }
        // where/equal-to are not a natural life-amount form -> decline.
        Some(_) => None,
        None => Some(number_word(body.strip_suffix(" life")?)?.to_string()),
    }
}

/// `Create <count> <Name> token[s].` — a PREDEFINED token maker ([CR#111.10]):
/// "Create a Treasure token.", "Create two Food tokens.". The name is one of
/// the rules-defined tokens deckmaste builds (`Treasure`, `Food`, `Gold`,
/// `Clue`, `Blood`); the creating effect defines no characteristics of its own
/// — the rules do — so it emits `Create(<count>, Named(<Name>))`, the
/// bare-ident `TokenSpec::Named` position. Fixed counts only (the bare numeral
/// = reader sugar for `Count::Literal`, like the sibling creature-token /
/// `Draw` productions). A `tapped` modifier, a dynamic count (`X`, "that many",
/// "a number of …"), an unbuilt predefined token (Powerstone, Map, …), or any
/// trailing clause declines — those are richer than this v1 production.
fn parse_create_predefined_token(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "create ")?.strip_suffix('.')?;
    // The terminator is the bare "token[s]" noun (plural first). A "creature
    // token" line is the inline-definition production's job, not this one — it
    // still has a type word before the noun, so the predefined-name check below
    // rejects it.
    let descriptor = body
        .strip_suffix(" tokens")
        .or_else(|| body.strip_suffix(" token"))?;
    // "<count-word> <Name>" — a literal count word then the predefined name.
    let (count_word, name) = descriptor.split_once(' ')?;
    let count = number_word(count_word)?;
    // Only a name the engine can resolve to a builtin token may become a
    // `Named(...)`; anything else (an unbuilt predefined token, a typo, a
    // "tapped …" modifier left in `name`) declines cleanly.
    deckmaste_core::PredefinedToken::from_name(name)?;
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("Create({count}, Named({name}))"),
    })
}

/// `Create <count> <P/T> [<colors>] [<subtypes>] creature token[s] [with
/// <kw…>].` — a creature-token maker. The creating effect defines the token's
/// characteristics [CR#111.3]; color rides a color indicator [CR#202.2e]
/// (a token has no mana cost); the name defaults to the subtypes plus "Token"
/// at synthesis [CR#111.4]. "Create" puts the tokens onto the battlefield
/// [CR#701.7a] — no target. Fixed counts only; `X`/"for each" decline. The
/// count emits as a bare numeral, reader-sugar for `Count::Literal`, matching
/// the sibling `Draw` production.
fn parse_create_token(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "create ")?.strip_suffix('.')?;
    // A trailing dynamic-count clause ("…, where X is the number of …", "… for
    // each …", "… equal to the number of …") is peeled first so the with-split
    // below never sees a "with" inside the count's filter.
    let (body, dynamic) = match count::strip(body) {
        Some(c) => (c.head, Some(c)),
        None => (body, None),
    };
    // Optional trailing keyword-grant clause.
    let (descriptor, with_clause) = match body.split_once(" with ") {
        Some((d, w)) => (d, Some(w)),
        None => (body, None),
    };
    // Creature-token terminator (plural first so it isn't stripped to "token").
    let descriptor = descriptor
        .strip_suffix(" creature tokens")
        .or_else(|| descriptor.strip_suffix(" creature token"))?;
    // Count RON + the descriptor remainder (starting at the P/T).
    let (count, rest) = resolve_token_count(descriptor, dynamic.as_ref())?;
    // P/T — mandatory; anchors this as a creature token.
    let (pt, rest) = rest.split_once(' ').unwrap_or((rest, ""));
    let (power, toughness) = parse_pt(pt)?;
    // Remaining words: leading color words (and "colorless"), then subtypes.
    let words: Vec<&str> = rest.split_whitespace().collect();
    let mut colors: Vec<&'static str> = Vec::new();
    let mut i = 0;
    while i < words.len() {
        if let Some(c) = super::filter::color_ident(words[i]) {
            colors.push(c);
            i += 1;
        } else if words[i] == "colorless"
            || (words[i] == "and"
                && i > 0
                && words
                    .get(i + 1)
                    .is_some_and(|w| super::filter::color_ident(w).is_some() || *w == "colorless"))
        {
            // "colorless" is an explicit no-color marker; "and" connects color
            // words — both advance past a non-subtype word without recording a color.
            i += 1;
        } else {
            break;
        }
    }
    let subtypes = &words[i..];
    // Every remaining word must be a plausible single creature subtype
    // (uppercase-initial, ASCII-alphabetic). Anything else means the line is
    // richer than this v1 production: a multi-token sentence ("…, a 2/2 …"
    // leaves comma/digit-bearing words), a card-type word ("artifact creature"),
    // or a trailing clause. Decline cleanly rather than emit junk RON.
    if subtypes.iter().any(|word| !is_subtype_word(word)) {
        return None;
    }
    let abilities = match with_clause {
        Some(clause) => parse_keyword_grants(clause)?,
        None => Vec::new(),
    };
    let mut fields: Vec<String> = Vec::new();
    if !colors.is_empty() {
        fields.push(format!("color_indicator: [{}]", colors.join(", ")));
    }
    fields.push("types: [Creature]".to_owned());
    if !subtypes.is_empty() {
        fields.push(format!("subtypes: [{}]", subtypes.join(", ")));
    }
    if !abilities.is_empty() {
        fields.push(format!("abilities: [{}]", abilities.join(", ")));
    }
    fields.push(format!("power: {power}"));
    fields.push(format!("toughness: {toughness}"));
    Some(ParsedEffect {
        targets: Vec::new(),
        effect: format!("Create({count}, Token({}))", fields.join(", ")),
    })
}

/// Resolve the token count + the descriptor remainder (from the P/T onward).
/// Literal path: the leading count word -> bare numeral. Dynamic path: the
/// binder dictates the placeholder the head must carry.
fn resolve_token_count<'a>(
    descriptor: &'a str,
    dynamic: Option<&count::CountClause>,
) -> Option<(String, &'a str)> {
    match dynamic {
        None => {
            let (word, rest) = descriptor.split_once(' ')?;
            Some((number_word(word)?.to_string(), rest))
        }
        Some(c) => match &c.binder {
            count::Binder::Variable(var) => {
                let (word, rest) = descriptor.split_once(' ')?;
                (word == var).then(|| (c.count.clone(), rest))
            }
            count::Binder::ForEach => {
                let (word, rest) = descriptor.split_once(' ')?;
                (number_word(word)? == 1).then(|| (c.count.clone(), rest))
            }
            count::Binder::EqualTo => {
                let rest = descriptor.strip_prefix("a number of ")?;
                Some((c.count.clone(), rest))
            }
        },
    }
}

/// `"1/1"` -> `(1, 1)`. `None` if either side isn't a non-negative integer
/// (a `*`/`X` P/T is a CDA token — not a v1 production).
fn parse_pt(text: &str) -> Option<(u32, u32)> {
    let (p, t) = text.split_once('/')?;
    Some((p.parse().ok()?, t.parse().ok()?))
}

/// A plausible single creature subtype: uppercase-initial and all ASCII
/// alphabetic. Rejects lowercase card-type words ("artifact"), connectives, and
/// any word carrying a comma/slash/digit — the tell-tale of a multi-token line
/// or trailing clause this v1 production doesn't handle.
fn is_subtype_word(word: &str) -> bool {
    let mut chars = word.chars();
    chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && word.chars().all(|c| c.is_ascii_alphabetic())
}

/// A `with <kw>[, <kw>][ and <kw>]` clause (trailing period already stripped)
/// -> the `Keyword(...)` invocations, reusing the keyword catalog. `None` if
/// any piece isn't a recognized no-argument keyword (an argument-taking keyword
/// or a quoted ability declines the WHOLE production — never a partial parse).
fn parse_keyword_grants(clause: &str) -> Option<Vec<String>> {
    clause
        .split(',')
        .flat_map(|piece| piece.split(" and "))
        .map(str::trim)
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            crate::parsers::keyword_ability::match_keyword_invocation(piece)
                .map(|ident| format!("Keyword({ident})"))
        })
        .collect()
}

/// A small spelled cardinal or a bare decimal -> its value. `None` for
/// anything else (e.g. "X", "that many"). Shared with the sibling frame
/// parsers (cost counts spell the same way).
pub(super) fn number_word(word: &str) -> Option<u32> {
    match word {
        "a" | "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        digits => digits.parse().ok(),
    }
}

/// Maps the "to <X>" tail of a damage clause to its `(target declarations,
/// body selection)`. Targeted shapes declare a `TargetSpec` and the body reads
/// `Target(0)`; "each" shapes declare nothing and inline a `Filter(...)`
/// selection.
fn damage_target(text: &str) -> Option<(Vec<String>, String)> {
    Some(match text {
        "any target" => (vec!["AnyTarget".to_owned()], "Target(0)".to_owned()),
        "target player" => (vec!["TargetOne(Player)".to_owned()], "Target(0)".to_owned()),
        // "target opponent" — a single opponent of you ([CR#102.2]).
        "target opponent" => (
            vec!["TargetOne(OpponentOf(Ref(You)))".to_owned()],
            "Target(0)".to_owned(),
        ),
        // The restricted "any target" minus its object members ([CR#115.4]):
        // a player or planeswalker, never a creature/battle (Lava Spike).
        "target player or planeswalker" => (
            vec!["TargetOne(OneOf([Player, Planeswalker]))".to_owned()],
            "Target(0)".to_owned(),
        ),
        "each creature" => (Vec::new(), "Filter(Creature)".to_owned()),
        "each player" => (Vec::new(), "Filter(Player)".to_owned()),
        // "each opponent" — the players who are opponents of you ([CR#102.2]).
        "each opponent" => (Vec::new(), "Filter(OpponentOf(Ref(You)))".to_owned()),
        // "each creature and each player" — every member of the combined set
        // ([CR#608.2d] distributive each). The two "each" groups union into one
        // `Filter(OneOf([…]))` selection (Pestilence / Earthquake-style sweeps).
        "each creature and each player" => {
            (Vec::new(), "Filter(OneOf([Creature, Player]))".to_owned())
        }
        // A "target <subject>" object target whose subject parses through the
        // shared object-target grammar (single head noun, or a "<type> or
        // <type>" / "attacking or blocking creature" disjunction).
        _ => {
            let subject = text.strip_prefix("target ")?;
            let filter = object_target_filter(subject)?;
            (vec![format!("TargetOne({filter})")], "Target(0)".to_owned())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve::CardKind;

    /// `(targets joined by ", ", effect)` for terse assertions. Uses the EMPTY
    /// reverse index (the macro-template fallthrough declines), so these pin
    /// the BESPOKE productions in isolation.
    fn parsed(line: &str) -> Option<(String, String)> {
        let ctx = crate::parsers::test_ctx::ctx(CardKind::Permanent);
        parse_clause(line, &ctx).map(|p| (p.targets.join(", "), p.effect))
    }

    /// `(targets, effect)` resolved against the REAL builtin macro index, so
    /// the `Effect`-kind macro-template fallthrough is exercised.
    fn parsed_with_macros(line: &str) -> Option<(String, String)> {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        parse_clause(line, &ctx).map(|p| (p.targets.join(", "), p.effect))
    }

    /// Whether the clause declines under the EMPTY index (pins a bespoke
    /// production's non-match without the macro fallthrough shadowing it).
    fn declines(line: &str) -> bool {
        let ctx = crate::parsers::test_ctx::ctx(CardKind::Permanent);
        parse_clause(line, &ctx).is_none()
    }

    #[test]
    fn deal_damage_targeted_shapes() {
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 2 damage to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(Target(0), 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 4 damage to target player."),
            Some((
                "TargetOne(Player)".to_owned(),
                "DealDamage(Target(0), 4)".to_owned()
            ))
        );
        // Lava Spike's restricted target: player-or-planeswalker (can't hit
        // creatures), a strict subset of "any target".
        assert_eq!(
            parsed("~ deals 3 damage to target player or planeswalker."),
            Some((
                "TargetOne(OneOf([Player, Planeswalker]))".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_each_shapes() {
        assert_eq!(
            parsed("~ deals 2 damage to each creature."),
            Some((String::new(), "DealDamage(Filter(Creature), 2)".to_owned()))
        );
        assert_eq!(
            parsed("~ deals 20 damage to each player."),
            Some((String::new(), "DealDamage(Filter(Player), 20)".to_owned()))
        );
        // "each opponent" -> the player set "opponents of you".
        assert_eq!(
            parsed("~ deals 1 damage to each opponent."),
            Some((
                String::new(),
                "DealDamage(Filter(OpponentOf(Ref(You))), 1)".to_owned()
            ))
        );
    }

    #[test]
    fn destroy_target_shapes() {
        // The target subject parses via filter.rs into a `TargetOne(<filter>)`.
        assert_eq!(
            parsed("Destroy target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target artifact."),
            Some((
                "TargetOne(Type(Artifact))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target nonland permanent."),
            Some((
                "TargetOne(AllOf([Permanent, Not(Type(Land))]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // Lowercase lead (the clause after a trigger comma) parses too. The
        // bare-subtype head is battlefield-scoped ([CR#109.2,115.2]).
        assert_eq!(
            parsed("destroy target Goblin."),
            Some((
                "TargetOne(AllOf([Permanent, Subtype(\"Goblin\")]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn destroy_target_with_keyword_quality() {
        // "with <keyword>" now resolves to a `Has(<Keyword>)` filter clause, so a
        // keyword-quality target parses (shared filter grammar gain).
        assert_eq!(
            parsed("Destroy target creature with flying."),
            Some((
                "TargetOne(AllOf([Creature, Has(Flying)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_team_like_overrun() {
        // Overrun: a team P/T boost + keyword grant lasting until end of turn.
        assert_eq!(
            parsed("Creatures you control get +3/+3 and gain trample until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [AddPowerToughness(3, 3), GainAbility(Keyword(Trample))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_self_and_target() {
        // Self pump ("~ gets …"): scope Of(This), no target.
        assert_eq!(
            parsed("~ gets +1/+1 until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Of(This), changes: [AddPowerToughness(1, 1)]), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Single-target pump ("target creature gets …"): TargetOne + Of(Target(0)).
        assert_eq!(
            parsed("Target creature gets +3/+3 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(of: Of(Target(0)), changes: [AddPowerToughness(3, 3)]), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Keyword-only durational grant on a target.
        assert_eq!(
            parsed("Target creature gains flying until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(of: Of(Target(0)), changes: [GainAbility(Keyword(Flying))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn declines_unknown_damage_targets_and_non_effects() {
        // A damage target the grammar doesn't model still declines.
        assert!(declines("~ deals 3 damage to each artifact."));
        assert!(declines("Flying"));
        assert!(declines("~ deals X damage to any target."));
        // Destroy without the "target" form (board wipes) is a later follow-up.
        assert!(declines("Destroy all creatures."));
        // A target subject the filter grammar can't parse declines. ("with
        // flying" now resolves to a keyword-quality filter, so that target
        // parses — see `destroy_target_with_keyword_quality`; pick a phrase the
        // filter grammar still rejects here.)
        assert!(declines("Destroy target creature wearing hats."));
        // A pump without the durational marker isn't an effect-grammar pump (it's
        // a static anthem's job on a permanent).
        assert!(declines("Creatures you control get +1/+1."));
    }

    #[test]
    fn draw_counts_from_words_and_digits() {
        assert_eq!(
            parsed("Draw a card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
        assert_eq!(
            parsed("Draw one card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
        assert_eq!(
            parsed("Draw two cards."),
            Some((String::new(), "Draw(2)".to_owned()))
        );
        assert_eq!(
            parsed("Draw three cards."),
            Some((String::new(), "Draw(3)".to_owned()))
        );
        assert_eq!(
            parsed("Draw 5 cards."),
            Some((String::new(), "Draw(5)".to_owned()))
        );
    }

    #[test]
    fn draw_declines_unparseable_counts() {
        // "X" and "that many" aren't v1 productions.
        assert!(declines("Draw X cards."));
        assert!(declines("Draw that many cards."));
    }

    #[test]
    fn deal_damage_accepts_it_subject() {
        // Trigger surface: "it deals …" (the source), same RON as "~ deals …".
        assert_eq!(
            parsed("it deals 1 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 1)".to_owned()
            ))
        );
        // Activated surface: clause-initial "It deals …" after a cost colon.
        assert_eq!(
            parsed("It deals 2 damage to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(Target(0), 2)".to_owned()
            ))
        );
    }

    #[test]
    fn draw_is_case_insensitive() {
        // Trigger surface: lowercase "draw a card." (mid-sentence).
        assert_eq!(
            parsed("draw a card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
    }

    #[test]
    fn spell_surface_still_parses() {
        // Regression: the spell forms must keep working after generalization.
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
        assert_eq!(
            parsed("Draw two cards."),
            Some((String::new(), "Draw(2)".to_owned()))
        );
    }

    #[test]
    fn lose_and_gain_life() {
        assert_eq!(
            parsed("You lose 1 life."),
            Some((String::new(), "LoseLife(1)".to_owned()))
        );
        assert_eq!(
            parsed("you lose 2 life."),
            Some((String::new(), "LoseLife(2)".to_owned()))
        );
        assert_eq!(
            parsed("You gain 3 life."),
            Some((String::new(), "GainLife(3)".to_owned()))
        );
        assert_eq!(
            parsed("you gain three life."),
            Some((String::new(), "GainLife(3)".to_owned()))
        );
    }

    #[test]
    fn life_declines_unparseable() {
        assert!(declines("you lose life."));
        assert!(declines("you gain X life."));
    }

    #[test]
    fn create_token_fixed_count_vanilla() {
        assert_eq!(
            parsed("Create three 1/1 red Goblin creature tokens."),
            Some((
                String::new(),
                "Create(3, Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Create a 1/1 red Goblin creature token."),
            Some((
                String::new(),
                "Create(1, Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_predefined_token_singular_and_plural() {
        // [CR#111.10]: a predefined token by bare name -> `Named(<Name>)`.
        assert_eq!(
            parsed("Create a Treasure token."),
            Some((String::new(), "Create(1, Named(Treasure))".to_owned()))
        );
        assert_eq!(
            parsed("create a Food token."),
            Some((String::new(), "Create(1, Named(Food))".to_owned()))
        );
        assert_eq!(
            parsed("Create two Treasure tokens."),
            Some((String::new(), "Create(2, Named(Treasure))".to_owned()))
        );
        // Gold, Clue, Blood are also built.
        assert_eq!(
            parsed("create a Gold token."),
            Some((String::new(), "Create(1, Named(Gold))".to_owned()))
        );
        assert_eq!(
            parsed("create a Blood token."),
            Some((String::new(), "Create(1, Named(Blood))".to_owned()))
        );
        assert_eq!(
            parsed("create a Clue token."),
            Some((String::new(), "Create(1, Named(Clue))".to_owned()))
        );
    }

    #[test]
    fn create_predefined_token_declines_unbuilt_and_modified() {
        // An unbuilt predefined token (no resolving definition yet) declines.
        assert!(declines("create a tapped Powerstone token."));
        assert!(declines("create a Powerstone token."));
        assert!(declines("create a Map token."));
        // A "tapped" modifier is not yet representable on a Named token.
        assert!(declines("create a tapped Treasure token."));
        // Dynamic counts stay out of this v1 production.
        assert!(declines("create X Treasure tokens."));
        assert!(declines("create that many Treasure tokens."));
        // A plain unknown name is not a predefined token.
        assert!(declines("create a Bogus token."));
    }

    #[test]
    fn create_token_with_keyword_grants() {
        assert_eq!(
            parsed("create a 1/1 red Goblin creature token with haste."),
            Some((
                String::new(),
                "Create(1, Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], abilities: [Keyword(Haste)], power: 1, toughness: 1))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Create a 2/2 white Cat creature token with flying and vigilance."),
            Some((
                String::new(),
                "Create(1, Token(color_indicator: [White], types: [Creature], subtypes: [Cat], abilities: [Keyword(Flying), Keyword(Vigilance)], power: 2, toughness: 2))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_multicolor_and_multi_subtype() {
        assert_eq!(
            parsed("Create two 1/1 black and green Elf Warrior creature tokens."),
            Some((
                String::new(),
                "Create(2, Token(color_indicator: [Black, Green], types: [Creature], subtypes: [Elf, Warrior], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_colorless_omits_color() {
        assert_eq!(
            parsed("Create a 1/1 colorless Eldrazi Scion creature token."),
            Some((
                String::new(),
                "Create(1, Token(types: [Creature], subtypes: [Eldrazi, Scion], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_no_subtype_omits_subtypes() {
        assert_eq!(
            parsed("Create a 1/1 red creature token."),
            Some((
                String::new(),
                "Create(1, Token(color_indicator: [Red], types: [Creature], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_declines_out_of_scope() {
        // Dynamic count -> gen-dynamic-count.
        assert!(declines("Create X 1/1 red Goblin creature tokens."));
        // Argument-taking keyword grant declines the whole production.
        assert!(declines(
            "Create a 1/1 white Cat creature token with ward {2}."
        ));
        // Quoted granted ability -> follow-up seam.
        assert!(declines(
            "Create a 1/1 red Goblin creature token with \"~ attacks each combat if able.\"."
        ));
    }

    #[test]
    fn create_token_declines_multi_and_typed_tokens() {
        // Multi-token sentence (comma-separated tokens) — previously emitted
        // junk RON (double comma) and crashed the pipeline; must decline.
        assert!(declines(
            "Create a 1/1 green Snake creature token, a 2/2 green Wolf creature token, and a 3/3 green Elephant creature token."
        ));
        // Two tokens joined by a trailing conjunction after a with-clause.
        assert!(declines(
            "Create a 1/1 red Dinosaur creature token with haste and a 1/1 white Human Soldier creature token."
        ));
        // Artifact creature token — the "artifact" card-type word is out of scope.
        assert!(declines(
            "Create a 3/3 colorless Phyrexian Golem artifact creature token."
        ));
        // Trailing clause after the token.
        assert!(declines(
            "Create a 1/1 white Bird creature token with flying, then populate."
        ));
    }

    #[test]
    fn create_token_dynamic_where_x() {
        // Krenko, Mob Boss.
        assert_eq!(
            parsed("Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control."),
            Some((
                String::new(),
                "Create(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])), \
                 Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_dynamic_for_each() {
        assert_eq!(
            parsed("Create a 1/1 red Goblin creature token for each Goblin you control."),
            Some((
                String::new(),
                "Create(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])), \
                 Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_dynamic_equal_to() {
        assert_eq!(
            parsed("Create a number of 1/1 white Soldier creature tokens equal to the number of creatures you control."),
            Some((
                String::new(),
                "Create(CountOf(AllOf([Creature, ControlledBy(Ref(You))])), \
                 Token(color_indicator: [White], types: [Creature], subtypes: [Soldier], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_dynamic_mismatched_var_declines() {
        // The leading count word must equal the where-clause variable.
        assert!(declines(
            "Create Y 1/1 red Goblin creature tokens, where X is the number of Goblins you control."
        ));
        // A non-unit base under "for each" has no Count product form -> decline.
        assert!(declines(
            "Create two 1/1 red Goblin creature tokens for each Goblin you control."
        ));
    }

    #[test]
    fn deal_damage_dynamic_equal_to() {
        assert_eq!(
            parsed("~ deals damage to any target equal to the number of Goblins you control."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])))".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_dynamic_where_x() {
        assert_eq!(
            parsed("~ deals X damage to target player, where X is the number of Goblins you control."),
            Some((
                "TargetOne(Player)".to_owned(),
                "DealDamage(Target(0), CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])))".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_literal_still_bare() {
        // Regression: the literal path keeps emitting a bare numeral.
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(Target(0), 3)".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_for_each() {
        assert_eq!(
            parsed("Creatures you control get +1/+1 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [Power(Up(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])))), \
                 Toughness(Up(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))]))))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_for_each_half_scaled() {
        // "+1/+0 for each": power scales, toughness fixed at 0.
        assert_eq!(
            parsed("Creatures you control get +1/+0 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [Power(Up(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])))), \
                 Toughness(Up(0))]), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_for_each_nonunit_declines() {
        // "+2/+2 for each" has no Count product form -> decline.
        assert!(declines(
            "Creatures you control get +2/+2 for each Goblin you control until end of turn."
        ));
    }

    #[test]
    fn gain_life_for_each_attacking_filter() {
        // Dwynen lifegain: "you gain 1 life for each attacking Elf you control."
        // The "attacking" status rides the shared filter grammar; base must be 1.
        assert_eq!(
            parsed("you gain 1 life for each attacking Elf you control."),
            Some((
                String::new(),
                "GainLife(CountOf(AllOf([Permanent, Subtype(\"Elf\"), Attacking, ControlledBy(Ref(You))])))"
                    .to_owned()
            ))
        );
        // Fixed life is still a bare numeral (regression).
        assert_eq!(
            parsed("You gain 3 life."),
            Some((String::new(), "GainLife(3)".to_owned()))
        );
        // A non-unit base under "for each" has no Count product form -> declines.
        assert!(declines(
            "you gain 2 life for each attacking Elf you control."
        ));
    }

    #[test]
    fn may_wraps_inner_effect() {
        // Lys Alana rider: "you may create a 1/1 green Elf Warrior creature token."
        assert_eq!(
            parsed("you may create a 1/1 green Elf Warrior creature token."),
            Some((
                String::new(),
                "May(effect: Create(1, Token(color_indicator: [Green], types: [Creature], \
                 subtypes: [Elf, Warrior], power: 1, toughness: 1)))"
                    .to_owned()
            ))
        );
        // A `you may` over a targeted effect carries the inner target through.
        assert_eq!(
            parsed("You may draw a card."),
            Some((String::new(), "May(effect: Draw(1))".to_owned()))
        );
    }

    #[test]
    fn may_declines_unparseable_inner() {
        // The whole `you may` production declines when the inner effect doesn't
        // parse (no partial parse).
        assert!(declines("you may flip a coin."));
    }

    /// A nullary `Effect`-kind macro template (`investigate`) resolves as the
    /// effect body through the final macro-template fallthrough — emitting the
    /// bare invocation, no targets. Both the bare and Title-Case leads match
    /// (the template is case-folded).
    #[test]
    fn macro_effect_investigate_resolves_via_template() {
        assert_eq!(
            parsed_with_macros("Investigate."),
            Some((String::new(), "Investigate".to_owned()))
        );
        // Mid-sentence (lowercase) lead — the clause after a trigger comma.
        assert_eq!(
            parsed_with_macros("investigate."),
            Some((String::new(), "Investigate".to_owned()))
        );
    }

    /// A `you may <macro-action>` rider wraps the macro effect in `May`, so
    /// "you may investigate." resolves through the shared `May` production over
    /// the macro-template fallthrough.
    #[test]
    fn macro_effect_under_may_rider() {
        assert_eq!(
            parsed_with_macros("you may investigate."),
            Some((String::new(), "May(effect: Investigate)".to_owned()))
        );
    }

    /// A slot-bearing `Effect`-kind macro whose param is a `Reference` resolves
    /// through the fallthrough: the self-reference sigil `~` fills the slot as
    /// `This` ([CR#201.5]). Regenerate (`template: "regenerate ${0}"`,
    /// `params: [Reference]`) is the flagship — `regenerate ~.` ->
    /// `Regenerate(This)`.
    #[test]
    fn macro_effect_reference_slot_reads_self_ref() {
        assert_eq!(
            parsed_with_macros("Regenerate ~."),
            Some((String::new(), "Regenerate(This)".to_owned()))
        );
        // Mid-sentence (lowercase) lead — the clause after a trigger comma.
        assert_eq!(
            parsed_with_macros("regenerate ~."),
            Some((String::new(), "Regenerate(This)".to_owned()))
        );
    }

    /// "it" / "this creature" are source-anaphor self-references in a Reference
    /// slot — they too read `This` ([CR#201.5]); they survive normalization
    /// uncollapsed only when the upstream `~` rewrite didn't fire (e.g. a
    /// granted or generic anaphor), so the slot reader admits them
    /// directly.
    #[test]
    fn macro_effect_reference_slot_reads_it_anaphor() {
        assert_eq!(
            parsed_with_macros("Regenerate it."),
            Some((String::new(), "Regenerate(This)".to_owned()))
        );
    }

    /// An activated ability whose effect is a Reference-slot macro
    /// ("{1}{G}: Regenerate ~.") graduates through the activated shell — the
    /// slot reader runs inside the shared effect grammar the activated frame
    /// wraps. Albino Troll's regenerate ability is the canonical near-miss.
    #[test]
    fn activated_regenerate_self_graduates() {
        let out = crate::parsers::activated_ability::resolve_line(
            "{1}{G}: Regenerate ~.",
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap();
        assert_eq!(
            out.as_deref(),
            Some("Activated(cost: [Mana([Generic(1),Green])], effect: Regenerate(This))")
        );
    }

    #[test]
    fn sacrifice_self_anaphors() {
        // "it" and "~" both name the resolving source.
        assert_eq!(
            parsed("Sacrifice it."),
            Some((String::new(), "Sacrifice(This)".to_owned()))
        );
        assert_eq!(
            parsed("Sacrifice ~."),
            Some((String::new(), "Sacrifice(This)".to_owned()))
        );
    }

    #[test]
    fn sacrifice_unless_pay_toll() {
        assert_eq!(
            parsed("Sacrifice ~ unless you pay {2}."),
            Some((
                String::new(),
                "Unless(effect: Sacrifice(This), unless: [Mana([Generic(2)])])".to_owned()
            ))
        );
        assert_eq!(
            parsed("Sacrifice it unless you pay {W}{W}."),
            Some((
                String::new(),
                "Unless(effect: Sacrifice(This), unless: [Mana([White,White])])".to_owned()
            ))
        );
    }

    #[test]
    fn sacrifice_non_self_declines() {
        // A chosen sacrifice ("a creature") is a cost-grammar concern, not this
        // self-anaphor body production.
        assert!(parsed("Sacrifice a creature.").is_none());
        assert!(parsed("Sacrifice another creature.").is_none());
    }

    #[test]
    fn attach_it_to_target() {
        assert_eq!(
            parsed("Attach it to target creature you control."),
            Some((
                "TargetOne(AllOf([Creature, ControlledBy(Ref(You))]))".to_owned(),
                "Attach(what: This, to: Target(0))".to_owned()
            ))
        );
    }

    /// A Reference slot whose text is neither `~` nor a modeled anaphor (a
    /// `target …`/`enchanted …` reference that would need a target declaration
    /// or an attachment ref) declines cleanly — the slot reader can't hoist
    /// a target onto the frame, so those stay a follow-up rather than mint
    /// junk RON.
    #[test]
    fn macro_effect_reference_slot_declines_target_subject() {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        assert!(parse_clause("Regenerate target creature.", &ctx).is_none());
        assert!(parse_clause("Regenerate enchanted creature.", &ctx).is_none());
    }

    /// The fallthrough requires the WHOLE clause to be the template — a clause
    /// with trailing text past the action word declines (no partial parse).
    #[test]
    fn macro_effect_declines_on_trailing_text() {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        // "investigate twice" is a repeated-action shape, not the bare template.
        assert!(parse_clause("investigate twice.", &ctx).is_none());
        // An unknown action word still declines.
        assert!(parse_clause("teleport.", &ctx).is_none());
    }

    /// The bespoke productions still LEAD: a line both a bespoke parser and a
    /// macro template could claim goes to the bespoke parser (`Draw(1)`, not a
    /// hypothetical draw macro). Regression that the fallthrough is last.
    #[test]
    fn bespoke_productions_lead_over_macro_templates() {
        assert_eq!(
            parsed_with_macros("Draw a card."),
            Some((String::new(), "Draw(1)".to_owned()))
        );
    }

    #[test]
    fn conditional_instead_at_end() {
        // Secrets of the Golden City: base draw, override draw, "instead" at the
        // end of the conditional sentence. The conditional branch is `then`; the
        // base is `otherwise` (the branch when the condition is false).
        assert_eq!(
            parsed_with_macros(
                "Draw two cards. If you have the city's blessing, draw three cards instead."
            ),
            Some((
                String::new(),
                "If(condition: YouHaveTheCitysBlessing, then: Draw(3), otherwise: Draw(2))"
                    .to_owned()
            ))
        );
    }

    #[test]
    fn conditional_instead_at_front() {
        // "If you have the city's blessing, instead <override>." — the "instead"
        // leads the override clause.
        assert_eq!(
            parsed_with_macros(
                "Creatures you control get +1/+1 until end of turn. \
                 If you have the city's blessing, instead creatures you control get +2/+2 until end of turn."
            ),
            Some((
                String::new(),
                "If(condition: YouHaveTheCitysBlessing, \
                 then: Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [AddPowerToughness(2, 2)]), duration: FixedUntil(EndOfTurn)), \
                 otherwise: Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [AddPowerToughness(1, 1)]), duration: FixedUntil(EndOfTurn)))".to_owned()
            ))
        );
    }

    #[test]
    fn conditional_no_instead() {
        // "If you have the city's blessing, <override>." with no "instead" word —
        // still a conditional branch (the override is `then`, base is `otherwise`).
        assert_eq!(
            parsed_with_macros("Draw a card. If you have the city's blessing, draw two cards."),
            Some((
                String::new(),
                "If(condition: YouHaveTheCitysBlessing, then: Draw(2), otherwise: Draw(1))"
                    .to_owned()
            ))
        );
    }

    #[test]
    fn conditional_declines_unknown_condition() {
        // An ungrounded condition phrase declines the whole production (the line
        // stays Unparsed rather than emit an unverified condition).
        assert!(
            parsed_with_macros("Draw a card. If you control a creature, draw two cards instead.")
                .is_none()
        );
    }

    #[test]
    fn conditional_declines_unparseable_branch() {
        // Either branch failing to parse declines the whole `If` (no partial).
        // The unparseable branch is a face-down effect (`engine-face-down` is
        // unbuilt — no Manifest production or macro), so it robustly declines
        // regardless of which keyword-action macros the effect grammar gains.
        assert!(
            parsed_with_macros(
                "Manifest the top card of your library. If you have the city's blessing, draw a card instead."
            )
            .is_none()
        );
        assert!(
            parsed_with_macros(
                "Draw a card. If you have the city's blessing, manifest the top card of your library instead."
            )
            .is_none()
        );
    }

    #[test]
    fn conditional_declines_targeted_branch() {
        // v1 declines when a branch declares targets: both branches would emit a
        // `Target(0)` with no shared announce list, which the `If` node can't
        // express yet. Decline cleanly rather than emit colliding targets.
        assert!(
            parsed_with_macros(
                "Destroy target creature. If you have the city's blessing, destroy target artifact instead."
            )
            .is_none()
        );
    }

    #[test]
    fn counter_target_spell_bare() {
        // The spell on the stack is the target (Spell filter); the body counters
        // it ([CR#701.6a]).
        assert_eq!(
            parsed("Counter target spell."),
            Some((
                "TargetOne(Spell)".to_owned(),
                "Counter(Target(0))".to_owned()
            ))
        );
        // Lowercase lead (mid-sentence after a trigger comma).
        assert_eq!(
            parsed("counter target spell."),
            Some((
                "TargetOne(Spell)".to_owned(),
                "Counter(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn counter_target_spell_unless_pays_mana() {
        // "unless its controller pays {2}" wraps the counter in an Unless: the
        // spell's controller may pay the tax to stop it ([CR#118.12a]).
        assert_eq!(
            parsed("Counter target spell unless its controller pays {2}."),
            Some((
                "TargetOne(Spell)".to_owned(),
                "Unless(effect: Counter(Target(0)), who: ControllerOf(Target(0)), \
                 unless: [Mana([Generic(2)])])"
                    .to_owned()
            ))
        );
        assert_eq!(
            parsed("Counter target spell unless its controller pays {1}.").map(|p| p.1),
            Some(
                "Unless(effect: Counter(Target(0)), who: ControllerOf(Target(0)), \
                 unless: [Mana([Generic(1)])])"
                    .to_owned()
            )
        );
    }

    #[test]
    fn counter_declines_richer_riders() {
        // Riders past the bare/mana-tax forms decline (later productions).
        assert!(declines(
            "Counter target spell unless its controller pays {X}."
        ));
        assert!(declines(
            "Counter target spell unless its controller pays {1} for each card in your graveyard."
        ));
        assert!(declines("Counter target spell. You gain 5 life."));
        assert!(declines("Counter target spell you don't control."));
        // Spell-on-the-stack restrictions ("that targets a creature") aren't
        // modeled here.
        assert!(declines("Counter target spell that targets a creature."));
    }

    #[test]
    fn return_target_to_hand_battlefield() {
        assert_eq!(
            parsed("Return target creature to its owner's hand."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "ReturnToHand(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Return target permanent to its owner's hand."),
            Some((
                "TargetOne(Permanent)".to_owned(),
                "ReturnToHand(Target(0))".to_owned()
            ))
        );
        // "nonland permanent" rides the shared filter grammar's negation.
        assert_eq!(
            parsed("Return target nonland permanent to its owner's hand."),
            Some((
                "TargetOne(AllOf([Permanent, Not(Type(Land))]))".to_owned(),
                "ReturnToHand(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn return_self_to_hand_no_target() {
        // The effect body of `{cost}: Return ~ to its owner's hand.` — a
        // self-bounce, no target (the cost is the activated frame's job).
        assert_eq!(
            parsed("Return ~ to its owner's hand."),
            Some((String::new(), "ReturnToHand(This)".to_owned()))
        );
    }

    #[test]
    fn return_card_from_graveyard_to_hand() {
        // A graveyard card you own, returned to hand via a plain zone change
        // ([CR#400.7]). The type is the card-type spelling (not the
        // battlefield-scoped macro), scoped to the graveyard + owned by you.
        assert_eq!(
            parsed("Return target creature card from your graveyard to your hand."),
            Some((
                "TargetOne(AllOf([Type(Creature), InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        // Bare "card" (no type qualifier) — any card you own there.
        assert_eq!(
            parsed("Return target card from your graveyard to your hand."),
            Some((
                "TargetOne(AllOf([InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        // "instant or sorcery card" — a card-type disjunction.
        assert_eq!(
            parsed("Return target instant or sorcery card from your graveyard to your hand."),
            Some((
                "TargetOne(AllOf([OneOf([Type(Instant), Type(Sorcery)]), InZone(Graveyard), \
                 Owner(Ref(You))]))"
                    .to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
    }

    #[test]
    fn tap_and_untap_target() {
        assert_eq!(
            parsed("Tap target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Tap(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Untap target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Untap(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Untap target permanent."),
            Some((
                "TargetOne(Permanent)".to_owned(),
                "Untap(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Untap target land."),
            Some((
                "TargetOne(Type(Land))".to_owned(),
                "Untap(Target(0))".to_owned()
            ))
        );
        // A trailing rider sentence leaves text past the period — declines here.
        assert!(declines(
            "Tap target creature. It doesn't untap during its controller's next untap step."
        ));
    }

    #[test]
    fn destroy_target_disjunction_types() {
        // "artifact or enchantment" -> a OneOf of the two card types.
        assert_eq!(
            parsed("Destroy target artifact or enchantment."),
            Some((
                "TargetOne(OneOf([Type(Artifact), Type(Enchantment)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // "creature or planeswalker" -> the battlefield macros disjoined.
        assert_eq!(
            parsed("Destroy target creature or planeswalker."),
            Some((
                "TargetOne(OneOf([Creature, Planeswalker]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target artifact or land."),
            Some((
                "TargetOne(OneOf([Type(Artifact), Type(Land)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // Single-type "permanent" still parses through the shared phrase grammar.
        assert_eq!(
            parsed("Destroy target permanent."),
            Some((
                "TargetOne(Permanent)".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_broadened_targets() {
        // "target opponent" — a single opponent player.
        assert_eq!(
            parsed("~ deals 1 damage to target opponent."),
            Some((
                "TargetOne(OpponentOf(Ref(You)))".to_owned(),
                "DealDamage(Target(0), 1)".to_owned()
            ))
        );
        // "target attacking or blocking creature" — a shared-head status
        // disjunction.
        assert_eq!(
            parsed("~ deals 4 damage to target attacking or blocking creature."),
            Some((
                "TargetOne(AllOf([Creature, OneOf([Attacking, Blocking])]))".to_owned(),
                "DealDamage(Target(0), 4)".to_owned()
            ))
        );
        // "target creature or planeswalker" — disjoined object target.
        assert_eq!(
            parsed("~ deals 5 damage to target creature or planeswalker."),
            Some((
                "TargetOne(OneOf([Creature, Planeswalker]))".to_owned(),
                "DealDamage(Target(0), 5)".to_owned()
            ))
        );
        // "each creature and each player" — the unioned distributive sweep.
        assert_eq!(
            parsed("~ deals 2 damage to each creature and each player."),
            Some((
                String::new(),
                "DealDamage(Filter(OneOf([Creature, Player])), 2)".to_owned()
            ))
        );
    }

    #[test]
    fn put_counter_on_target() {
        // "Put a +1/+1 counter on target creature." — the counter kind resolves
        // to `P1P1Counter` via the `Counter`-kind macro template; the target is
        // a single creature; one counter ([CR#122.1]).
        assert_eq!(
            parsed_with_macros("Put a +1/+1 counter on target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "PutCounters(Target(0), P1P1Counter, 1)".to_owned()
            ))
        );
        // Lowercase lead (the clause after a trigger comma).
        assert_eq!(
            parsed_with_macros("put a +1/+1 counter on target creature you control."),
            Some((
                "TargetOne(AllOf([Creature, ControlledBy(Ref(You))]))".to_owned(),
                "PutCounters(Target(0), P1P1Counter, 1)".to_owned()
            ))
        );
        // "two +1/+1 counters" — the plural count.
        assert_eq!(
            parsed_with_macros("Put two +1/+1 counters on target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "PutCounters(Target(0), P1P1Counter, 2)".to_owned()
            ))
        );
        // "-1/-1 counter" generalizes to `M1M1Counter` for free.
        assert_eq!(
            parsed_with_macros("Put a -1/-1 counter on target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "PutCounters(Target(0), M1M1Counter, 1)".to_owned()
            ))
        );
    }

    #[test]
    fn put_counter_on_self() {
        // "Put a +1/+1 counter on it." / "on ~." — the resolving source counters
        // itself (combat-damage triggers); no target, `This` selection.
        assert_eq!(
            parsed_with_macros("put a +1/+1 counter on it."),
            Some((
                String::new(),
                "PutCounters(This, P1P1Counter, 1)".to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("Put a +1/+1 counter on ~."),
            Some((
                String::new(),
                "PutCounters(This, P1P1Counter, 1)".to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("put two +1/+1 counters on ~."),
            Some((
                String::new(),
                "PutCounters(This, P1P1Counter, 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("Put three +1/+1 counters on it."),
            Some((
                String::new(),
                "PutCounters(This, P1P1Counter, 3)".to_owned()
            ))
        );
        // "-1/-1 counter on it" -> M1M1Counter.
        assert_eq!(
            parsed_with_macros("put a -1/-1 counter on it."),
            Some((
                String::new(),
                "PutCounters(This, M1M1Counter, 1)".to_owned()
            ))
        );
    }

    #[test]
    fn put_counter_declines_out_of_scope() {
        // An unmodeled counter kind (no `+2/+2`/`+1/+0` macro) declines cleanly —
        // the named-counter vocabulary doesn't carry it yet.
        assert!(parsed_with_macros("Put a +2/+2 counter on target creature.").is_none());
        assert!(parsed_with_macros("put a +1/+0 counter on ~.").is_none());
        // `X` and "that many" counts aren't v1 productions.
        assert!(parsed_with_macros("Put X +1/+1 counters on it.").is_none());
        // A "for each" scaled count declines (later production).
        assert!(
            parsed_with_macros("put a +1/+1 counter on target Shrine for each Shrine you control.")
                .is_none()
        );
        // A target subject the filter grammar can't parse declines.
        assert!(
            parsed_with_macros("Put a +1/+1 counter on target creature wearing hats.").is_none()
        );
        // Under the EMPTY index (no counter macro) the production declines —
        // pins that the counter kind is macro-resolved, not hardcoded.
        assert!(declines("Put a +1/+1 counter on target creature."));
    }
}
