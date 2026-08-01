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

/// Zone from which an activated ability carrying this effect functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionalZone {
    Graveyard,
}

impl FunctionalZone {
    pub(super) fn ron(self) -> &'static str {
        match self {
            Self::Graveyard => "Graveyard",
        }
    }
}

/// One parsed effect clause: `TargetSpec` RON fragments to declare on the
/// frame (empty when the effect targets nothing), and the
/// `OneShotEffect`/`Action` body RON, which references any declared targets as
/// `It`, `It`…
pub(super) struct ParsedEffect {
    pub(super) functional_zone: Option<FunctionalZone>,
    pub(super) targets: Vec<String>,
    pub(super) effect: String,
}

/// One effect-clause production: a normalized oracle sentence -> a
/// [`ParsedEffect`], or `None` to decline.
type ClauseParser = fn(&str, &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>>;

/// The effect-clause productions, in priority order — first match wins.
/// `parse_if` leads (it folds a base sentence + conditional override into one
/// `OneShotEffect::If`); the bespoke productions follow (they encode
/// targeting/scope the bare macro templates can't carry); an
/// `OneShotEffect`-kind macro template ([`parse_macro_effect`]) is the final
/// fallthrough, so keyword-action lines (`investigate.`, `scry 2.`) route back
/// to the macro whose template renders them. [`ResolveCtx`] carries the reverse
/// template index that the fallthrough (and the conditional's condition-phrase
/// lookup) consults.
///
/// Productions that themselves consult the reverse template index (directly
/// or by re-entering `parse_clause`) propagate `anyhow::Result` directly — an
/// ambiguous macro match is a hard generation error, not a decline; every
/// other (Option-returning, ctx-less) production is lifted to the table's
/// element type by a non-capturing closure. First-match-wins across these
/// DISTINCT productions is unchanged — only a same-kind macro tie inside the
/// index is an error.
const CLAUSE_PARSERS: &[ClauseParser] = &[
    parse_may_reflexive,
    parse_if,
    parse_may,
    parse_player_taps_per_counter,
    |l, _| Ok(parse_destroy_no_regen(l)),
    |l, _| Ok(parse_rhystic_damage(l)),
    |l, _| Ok(parse_becomes_creature(l)),
    parse_sequence,
    parse_delayed_next_end_step,
    |l, _| Ok(parse_exile_target(l)),
    |l, _| Ok(parse_return_that_card(l)),
    |l, _| Ok(parse_damage_and_damage(l)),
    |l, _| Ok(parse_deal_damage(l, 0)),
    |l, _| Ok(parse_draw_then_discard(l)),
    |l, _| Ok(parse_draw(l)),
    |l, _| Ok(parse_discard(l)),
    |l, _| Ok(parse_lose_life(l)),
    |l, _| Ok(parse_gain_life(l)),
    |l, _| Ok(parse_counter(l)),
    parse_put_counters,
    |l, _| Ok(parse_return_to_hand(l)),
    |l, _| Ok(parse_gains_control(l)),
    |l, _| Ok(parse_search_library(l)),
    |l, _| Ok(parse_reanimate(l)),
    |l, _| Ok(parse_bounce_to_library(l)),
    |l, _| Ok(parse_tap_untap(l)),
    |l, _| Ok(parse_destroy(l)),
    parse_destroy_macro_target,
    |l, _| Ok(parse_sacrifice(l)),
    |l, _| Ok(parse_attach(l)),
    parse_pump,
    |l, _| Ok(parse_combat_restriction(l)),
    |l, _| Ok(parse_create_predefined_token(l)),
    |l, _| Ok(parse_create_token(l)),
    parse_get_emblem,
    parse_declarative_subject,
    parse_macro_effect,
];

/// Parses one normalized effect line into a [`ParsedEffect`], or `None` to
/// decline, by trying [`CLAUSE_PARSERS`] in order — first match wins.
pub(super) fn parse_clause(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    for parser in CLAUSE_PARSERS {
        if let Some(p) = parser(line, ctx)? {
            return Ok(Some(p));
        }
    }
    Ok(None)
}

/// "That player taps an untapped artifact, creature, or land they control for
/// each <kind> counter on ~." The upkeep event binds "that player" as
/// `EventActor`; the counter noun resolves through the Counter macro index, so
/// this is a counter-family production rather than a Tangle Wire name check.
/// `Exactly` is intentionally not pre-clamped: the resolver clamps a choice to
/// the available candidates, implementing "as much as possible".
fn parse_player_taps_per_counter(
    line: &str,
    ctx: &ResolveCtx,
) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(counter_phrase) = line
        .strip_prefix(
            "that player taps an untapped artifact, creature, or land they control for each ",
        )
        .and_then(|rest| rest.strip_suffix(" on ~."))
    else {
        return Ok(None);
    };
    let Some(counter) = counter_kind(counter_phrase, ctx)? else {
        return Ok(None);
    };
    Ok(Some(ParsedEffect {
        functional_zone: None,
        targets: vec![],
        effect: format!(
            "Each(binder: Choose(quantity: Exactly(CounterCount(This, {counter})), \
             filter: And([Permanent, Or([Type(Artifact), Type(Creature), \
             Type(Land)]), ControlledBy(Ref(EventActor)), Not(Status(Tapped))]), \
             by: EventActor), effect: By(EventActor, Tap(It)))"
        ),
    }))
}

/// The original rhystic burn template: a chosen permanent's controller, or a
/// chosen player directly, may pay the toll to replace the larger damage with
/// the smaller damage. `Coalesce` selects the first live payer reference:
/// `ControllerOf(Target(0))` for a permanent and `Target(0)` for a player.
fn parse_rhystic_damage(line: &str) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?;
    let (first, second) = body.split_once(". If they do, ")?;
    let (lead, cost) =
        first.split_once(" unless that permanent's controller or that player pays ")?;
    let lead = lead.strip_prefix("~ deals ")?;
    let high = lead.strip_suffix(" damage to any target")?;
    let high = number_word(high)?;
    let second = second.strip_prefix("~ deals ")?;
    let low = second.strip_suffix(" damage to the permanent or player")?;
    let low = number_word(low)?;
    let cost =
        crate::parsers::cost::parse_cost(cost, crate::parsers::cost::VariableMana::Decline, None)
            .ok()
            .flatten()?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec!["AnyTarget".to_owned()],
        effect: format!(
            "MayPay(actor: Coalesce([ControllerOf(Target(0)), Target(0)]), cost: [{}], \
             and_then: DealDamage(This, {low}, Target(0)), \
             or_else: DealDamage(This, {high}, Target(0)))",
            cost.join(", ")
        ),
    })
}

/// A self-animation line of the classic manland form. Becoming a creature
/// sets the named characteristics for the duration while the explicit
/// "still a land" sentence preserves the existing land type.
fn parse_becomes_creature(line: &str) -> Option<ParsedEffect> {
    let body = line
        .strip_prefix("~ becomes a ")
        .or_else(|| line.strip_prefix("~ becomes an "))?
        .strip_suffix(" until end of turn. It's still a land.")?;
    let (pt, descriptor) = body.split_once(' ')?;
    let (power, toughness) = parse_pt(pt)?;
    let (descriptor, keywords) = match descriptor.split_once(" with ") {
        Some((head, tail)) => (head, parse_keyword_grants(tail)?),
        None => (descriptor, Vec::new()),
    };
    let words: Vec<&str> = descriptor.split_whitespace().collect();
    let creature = words.iter().position(|word| *word == "creature")?;
    if creature == 0 || creature + 1 != words.len() {
        return None;
    }
    let subtype = words[creature - 1];
    if !is_subtype_word(subtype) {
        return None;
    }
    let qualities = &words[..creature - 1];
    let (color_words, artifact) = match qualities.last() {
        Some(&"artifact") => (&qualities[..qualities.len() - 1], true),
        _ => (qualities, false),
    };
    let colors: Option<Vec<&str>> = color_words
        .iter()
        .filter(|word| **word != "and")
        .map(|word| match *word {
            "white" => Some("White"),
            "blue" => Some("Blue"),
            "black" => Some("Black"),
            "red" => Some("Red"),
            "green" => Some("Green"),
            _ => None,
        })
        .collect();
    let colors = colors?;
    if colors.is_empty() {
        return None;
    }
    let mut modifications = vec!["CardTypes(Add(\"Creature\"))".to_owned()];
    if artifact {
        modifications.push("CardTypes(Add(\"Artifact\"))".to_owned());
    }
    modifications.extend([
        format!("Subtypes(Add({subtype}))"),
        format!("Colors(Set([{}]))", colors.join(", ")),
        format!("Power(Set({power}))"),
        format!("Toughness(Set({toughness}))"),
    ]);
    modifications.extend(
        keywords
            .into_iter()
            .map(|keyword| format!("GainAbility({keyword})")),
    );
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!(
            "Until(FixedUntil(EndOfTurn), [Modify(This, Several([{}]))])",
            modifications.join(", ")
        ),
    })
}

/// `Target opponent gains control of it.` — the target player becomes the
/// controller of the resolving source. Trigger framing rewrites the target
/// read to `Target(0)` when an event also supplies an anaphoric object.
fn parse_gains_control(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "target ")?.strip_suffix(" gains control of it.")?;
    let filter = match subject {
        "opponent" => "OpponentOf(Ref(You))".to_owned(),
        "player" => "Player".to_owned(),
        _ => return None,
    };
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: "GainControl(This, Target(0))".to_owned(),
    })
}

/// Two damage instructions sharing one grammatical subject:
/// "~ deals N damage to X and M damage to Y." The first and second patients
/// are parsed by the ordinary damage production, then executed sequentially.
///
/// The only production that concatenates two sub-parses' announce lists, so
/// the only one that threads a non-zero slot index: the second patient's
/// declaration lands at index `first.targets.len()` and its body reads it back
/// there ([CR#115.3,601.2c]). Both halves reading the old wildcard `It` was
/// this shape's latent break — the second slot had no name at all.
fn parse_damage_and_damage(line: &str) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?;
    let (lead, rest) = body.split_once(" damage to ")?;
    let (subject, first_amount) = lead.rsplit_once(" deals ")?;
    let (first_patient, second) = rest.split_once(" and ")?;
    if !second.contains(" damage to ") {
        return None;
    }
    let first = parse_deal_damage(
        &format!("{subject} deals {first_amount} damage to {first_patient}."),
        0,
    )?;
    let second = parse_deal_damage(&format!("{subject} deals {second}."), first.targets.len())?;
    let mut targets = first.targets;
    targets.extend(second.targets);
    Some(ParsedEffect {
        functional_zone: None,
        targets,
        effect: format!("Sequentially([{}, {}])", first.effect, second.effect),
    })
}

/// The final effect-clause fallthrough: route the whole clause back to the
/// `OneShotEffect`-kind macro whose `template` renders it — the settled
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
fn parse_macro_effect(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(body) = line.strip_suffix('.') else {
        return Ok(None);
    };
    let body = body.trim();
    // Nullary (param-less) action macro — `investigate`. Full-line consumption
    // (and same-kind ambiguity) is now judged inside the matcher itself.
    if let Some(m) = ctx.index.match_kind("OneShotEffect", body)? {
        return Ok(Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: m.macro_name.to_string(),
        }));
    }
    // Slot-bearing action macro — `scry ${0}`, with each `${i}` slot read by
    // the typed reader.
    if let Some(m) = ctx
        .index
        .match_with("OneShotEffect", body, macro_slot_reader)?
    {
        return Ok(Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: m.invocation,
        }));
    }
    Ok(None)
}

/// Read one `OneShotEffect`-macro template slot of declared type `ty` from the
/// rest of the clause. The slot is the line's tail in the action shapes modeled
/// here (`scry 2` — a `Count` magnitude at the end; `regenerate ~` — a
/// `Reference` subject), so a successful read consumes all of `input`. `Count`
/// and the self-reference forms of `Reference` are read; an unmodeled slot type
/// (or a `Reference` that isn't a self-reference) declines, failing the whole
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
        // A card-type-descriptor slot ("… if it's a ${0} card …"): the
        // descriptor word(s) between "a" and "card", classified into the
        // CR-correct atom. A card TYPE / color-adjective / subtype descriptor
        // reuses the shared library-search reader (`search_card_descriptor` —
        // "land"/"creature" -> `Type(_)`); a bare SUPERTYPE the search reader
        // doesn't take standalone ("snow" -> `Supertype(Snow)`, [CR#205.4a])
        // is handled here so the search grammar (which never sees a bare "a
        // snow card" filter) is not widened. Unlike the tail-anchored
        // `Count`/`Reference` slots, this one is FOLLOWED by the literal
        // " card", so it consumes only up to that boundary (never the whole
        // clause), leaving " card …" for the pattern's trailing literal.
        "CardTypePredicate" => {
            let end = input.find(" card")?;
            let descriptor = input[..end].trim();
            let predicate = search_card_descriptor(descriptor).or_else(|| {
                filter::supertype_ident(descriptor).map(|s| format!("Supertype({s})"))
            })?;
            Some((predicate, end))
        }
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

/// The DECLARATIVE-SUBJECT production — ONE parser arm for the whole
/// player-verb family ("Target player mills two cards", "Each opponent
/// discards a card", "Each player loses 2 life"): the subject phrase parses
/// into the agent, and the remaining THIRD-PERSON verb phrase is handed to
/// the macro-template path — the macro templates do the verb matching (the
/// bare `PlayerAction`-kind `discards ${0:card|cards}` / `loses ${0} life` /
/// `gains ${0} life`, and the `OneShotEffect`-kind Composite macros'
/// two-slot `${0} mills ${1:card|cards}` / `${0} draws …`), so there are NO
/// per-verb parser arms here. A targeted subject declares its announce slot
/// and the agent reads the announced player as `It` ([CR#115.3]); an "each"
/// subject wraps the verb in `Each` over the player set, the agent being the
/// iteration anaphor `It` per element ([CR#608.2d]). The second-person "you"
/// subject is NOT handled here — its verb phrase conjugates differently
/// ("you mill", not "mills"), so it stays with the bespoke you-productions.
fn parse_declarative_subject(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(body) = line.strip_suffix('.') else {
        return Ok(None);
    };
    let Some((subject, verb_phrase)) = player_subject(body) else {
        return Ok(None);
    };
    // Two verb families share this third-person surface but lower differently,
    // so route by the macro KIND that matches (no per-verb arms). Full-line
    // consumption and same-kind ambiguity are judged inside the matcher itself.
    //
    // Both families now carry the actor as their own leading `${0}` Reference,
    // so the subject is SPLICED POSITIONALLY: re-attach the `It` anaphor to the
    // verb phrase and let the macro read it as that slot, yielding
    // `GainsLife(It, N)` / `Mills(It, N)` / `Draws(It, N)` directly.
    //
    // The former `PlayerAction` verbs (gain/lose life) used to take no subject
    // slot — the acting player came from a `By(It, …)` wrapper this function
    // spliced on afterward. The action role reshape deleted `By` and gave those
    // macros a real subject param, so the wrap is gone and the two branches
    // differ only in which macro KIND they search.
    let with_subject = format!("it {verb_phrase}");
    let body = if let Some(m) =
        ctx.index
            .match_with("Action", &with_subject, player_verb_slot_reader)?
    {
        m.invocation
    } else if let Some(m) =
        ctx.index
            .match_with("OneShotEffect", &with_subject, player_verb_slot_reader)?
    {
        m.invocation
    } else {
        return Ok(None);
    };
    Ok(Some(match subject {
        PlayerSubject::Target(spec) => ParsedEffect {
            functional_zone: None,
            targets: vec![spec],
            effect: body,
        },
        PlayerSubject::Each(filter) => ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: format!("Each(binder: Existing(SelectAll({filter})), effect: {body})"),
        },
    }))
}

/// A parsed player-subject phrase: a targeted player slot or a distributive
/// player set.
enum PlayerSubject {
    /// The `TargetSpec` RON to declare — the body reads it back as `It`.
    Target(String),
    /// The player `Predicate` RON an `Each` iterates.
    Each(String),
}

/// Split a declarative player-verb sentence into its subject phrase and the
/// third-person verb phrase — the closed subject vocabulary of the
/// mill/discard/draw/lose family (mirrors [`damage_target`]'s player rows).
fn player_subject(body: &str) -> Option<(PlayerSubject, &str)> {
    for prefix in [
        "target player ",
        "target opponent ",
        "each player ",
        "each opponent ",
    ] {
        let Some(rest) = strip_prefix_ci(body, prefix) else {
            continue;
        };
        let subject = match prefix {
            "target player " => PlayerSubject::Target("TargetOne(Player)".to_owned()),
            "target opponent " => {
                PlayerSubject::Target("TargetOne(OpponentOf(Ref(You)))".to_owned())
            }
            "each player " => PlayerSubject::Each("Player".to_owned()),
            _ => PlayerSubject::Each("OpponentOf(Ref(You))".to_owned()),
        };
        return Some((subject, rest));
    }
    None
}

/// The slot reader for the third-person player-verb templates. Each slot is
/// ONE leading token so the template's own literal tail (" cards", " life")
/// stays for the matcher — unlike [`macro_slot_reader`], which consumes the
/// whole clause tail. Two slot types occur:
///
/// - `Count` — a spelled cardinal, "a", or a bare decimal.
/// - `Reference` — the leading `${0}` actor of the `OneShotEffect`
///   keyword-action templates (`${0} mills …`, `${0} draws …`). Only the `It`
///   anaphor is read here: `parse_declarative_subject` re-attaches it after
///   stripping the subject, so the macro carries the acting player itself.
fn player_verb_slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    let token = input.split_whitespace().next()?;
    // The token is at the head of `input` (templates put whitespace between
    // segments), so its byte length is the consumed span.
    if input.find(token)? != 0 {
        return None;
    }
    match ty {
        "Count" => Some((number_word(token)?.to_string(), token.len())),
        "Reference" if token.eq_ignore_ascii_case("it") => Some(("It".to_owned(), token.len())),
        _ => None,
    }
}

/// The bounded `Count` slot reader for delimiter-separated templates like
/// `"gets +${0}/+${1}"` ([`PowerAndToughnessUp`](crate) et al.): unlike
/// [`player_verb_slot_reader`] (bounds on whitespace, for slots at a clause's
/// tail), this bounds on the first non-digit — `/`, `+`, `-`, whitespace, or
/// end — so a slot ahead of a `/` separator stops there instead of eating the
/// rest of the line. `pub(super)`: the static-ability parser's Modification
/// fold (`static_ability::parse_pt`) is its production caller.
pub(super) fn count_delim_slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    if ty != "Count" {
        return None;
    }
    // Consume the leading run of ASCII digits (Count::Literal); stop at '/',
    // '+', '-', whitespace, or end.
    let end = input
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(input.len());
    if end == 0 {
        return None;
    }
    let token = &input[..end];
    let n = number_word(token)?; // validates + normalizes
    Some((n.to_string(), end))
}

/// The composed `Modification`-macro slot reader: `Count` dispatches to
/// [`count_delim_slot_reader`] UNCHANGED (the 1,638 literal pumps'
/// `PowerAndToughnessUp`/`Down` fold keeps its exact bounded-digit behavior);
/// `Predicate` dispatches to [`modification_predicate_slot`] (the
/// `P·ForEach` family's selection noun, "Forest you control" et al.) — added
/// in isolation, mirroring [`crate::parsers::condition::condition_predicate`]'s
/// `Predicate`-slot handling. `pub(super)`: both this module's own
/// `pump_change_folded` and [`crate::parsers::static_ability`]'s `parse_pt`
/// pass it to `TemplateIndex::match_with`.
pub(super) fn modification_slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    match ty {
        "Count" => count_delim_slot_reader(ty, input),
        "Predicate" => modification_predicate_slot(input),
        _ => None,
    }
}

/// A `Modification`-macro `Predicate` slot's reader (the `P·ForEach` family's
/// trailing selection noun) — consumes the whole remaining phrase, mirroring
/// [`crate::parsers::condition::condition_predicate`]'s bare
/// [`filter::parse_phrase`] read exactly. The macro body itself wraps the
/// bare predicate in `Objects(...)`/`CountOf(...)` ([`P1P1ForEach`](crate) et
/// al.), so this reader hands back the bare filter RON unwrapped.
fn modification_predicate_slot(input: &str) -> Option<(String, usize)> {
    let phrase = input.trim_end();
    let pred = filter::parse_phrase(phrase)?;
    Some((pred, phrase.len()))
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
/// grounded condition. The condition is read the parse-via-macros way, via the
/// shared [`crate::parsers::condition::resolve`] routing (also used by
/// [`crate::parsers::static_ability`]'s `Conditionally` composer): the phrase
/// is routed to the `Condition`-kind macro whose `template` renders it (e.g.
/// `you have the city's blessing` -> `YouHaveTheCitysBlessing`, authored in
/// `plugins/builtin/macros/condition/`), and the emitted RON carries that
/// macro INVOCATION, which the loader expands to its `Condition` body — exactly
/// as an `OneShotEffect` action macro stands as an effect body. New condition
/// phrases are added by authoring a `Condition` macro, with no parser change.
/// v1 declines when EITHER branch declares targets: the two clauses share no
/// announce list here, so colliding `It` references can't be expressed —
/// a later production with a hoisted shared `Targeted` wrapper will lift that.
fn parse_if(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    // The conditional sentence opens at ". If " (the base sentence ends, the
    // "If" clause begins). Split on the LAST such boundary so a base sentence
    // that itself contains "if" survives; in practice these are single-base
    // single-override lines, so the last boundary is the only one.
    let Some((base, cond_sentence)) = line.rsplit_once(". If ") else {
        return Ok(None);
    };
    let base = format!("{base}.");
    // "<phrase>, <override>." — the condition phrase runs to the first comma,
    // the override clause follows.
    let Some(cond_sentence) = cond_sentence.strip_suffix('.') else {
        return Ok(None);
    };
    let Some((phrase, override_clause)) = cond_sentence.split_once(", ") else {
        return Ok(None);
    };
    // Route the phrase to its `Condition` macro; full-line consumption (and
    // same-kind ambiguity) is judged inside the matcher. The macro invocation
    // (name, or a filled slot-bearing invocation) is the parsed condition.
    let Some(condition) = crate::parsers::condition::resolve(phrase, ctx)? else {
        return Ok(None);
    };

    // "instead" may lead the override ("instead <override>") or trail it
    // ("<override> instead") — strip whichever side carries it, then re-attach
    // the sentence period the inner productions expect.
    let override_body = override_clause.trim();
    let override_body = strip_prefix_ci(override_body, "instead ")
        .or_else(|| override_body.strip_suffix(" instead"))
        .unwrap_or(override_body);
    let override_line = format!("{override_body}.");

    let Some(base_parsed) = parse_clause(&base, ctx)? else {
        return Ok(None);
    };
    let Some(then_parsed) = parse_clause(&override_line, ctx)? else {
        return Ok(None);
    };
    // v1: neither branch may declare targets (no shared announce list).
    if !base_parsed.targets.is_empty() || !then_parsed.targets.is_empty() {
        return Ok(None);
    }
    let functional_zone = base_parsed.functional_zone.or(then_parsed.functional_zone);
    Ok(Some(ParsedEffect {
        functional_zone,
        targets: Vec::new(),
        effect: format!(
            "If(condition: {condition}, then: {}, otherwise: {})",
            then_parsed.effect, base_parsed.effect
        ),
    }))
}

/// `you may <effect>` -> the inner effect wrapped in a `May` frame
/// ([CR#603,608] optional-do): `May(who: You, effect: <inner>)`. The inner
/// clause is re-parsed by [`parse_clause`], carrying through any targets it
/// declares — so the whole production declines if the inner effect isn't itself
/// parseable. Every inner production accepts a lowercase (mid-sentence) lead,
/// so the stripped clause re-enters them directly. Case-insensitive lead ("You
/// may" opens a trigger effect; "you may" follows a comma).
fn parse_may(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(inner) = strip_prefix_ci(line, "you may ") else {
        return Ok(None);
    };
    let Some(parsed) = parse_clause(inner, ctx)? else {
        return Ok(None);
    };
    Ok(Some(ParsedEffect {
        functional_zone: parsed.functional_zone,
        targets: parsed.targets,
        effect: format!("May(who: You, effect: {})", parsed.effect),
    }))
}

/// The reflexive-optional clause family ([CR#603,608]): `you may <offer>. If
/// you do, <did>` (with the `If you don't, <not>` complement), where the "if
/// you do"/"if you don't" continuation sentence(s) bind to the offer — they are
/// NOT independent clauses (each fails to parse alone). This production folds
/// the tail into ONE node so the offer's optional-with-consequence reading is
/// recovered; today both sentences fail and the whole line stays `Unparsed`.
///
/// Two node shapes, chosen by the offer:
/// - `you may pay <cost>. If you do, <did>` -> `MayPay { cost, and_then }` (the
///   dominant form; `actor` defaults to `You`, omitted from RON). The [`Cost`]
///   is read off the BARE post-`pay ` symbol string, so an energy `pay {E}{E}`
///   — whose cost renders WITH a leading `Pay` word, which would double in the
///   `may pay …` render frame — declines here rather than mis-round-tripping.
///   v1 emits no `or_else`: the `MayPay` render spells the negative branch with
///   a `"; if you don't"` semicolon, which cannot round-trip the oracle `". If
///   you don't"`, so a pay offer carrying a negative tail declines.
/// - `you may <verb-phrase>. If you do/don't, <branch>` -> `May { effect,
///   if_did, if_not }`; the verb phrase and each present branch re-enter
///   [`parse_clause`]. The `May` render is period-separated, so BOTH branches
///   round-trip.
///
/// The "if you do" branch consumes GREEDILY to the line end (or the "if you
/// don't" boundary), so a trailing delayed-trigger sentence ("… create a token.
/// Exile that token at the beginning of the next end step.") folds into the
/// branch as a nested [`parse_sequence`] rather than leaking as an
/// unconditional sibling.
///
/// Targets: like [`parse_may`], the offer's announce list carries through (the
/// branches read it anaphorically). A BRANCH that declares its OWN targets has
/// no shared announce list to bind against, so the production declines — the
/// same v1 stance [`parse_if`] takes, awaiting a later hoisted-`Targeted`
/// wrapper pass. Engages only when a `. If you do,`/`. If you don't,` boundary
/// is present; a bare `you may <x>` is [`parse_may`]'s job.
fn parse_may_reflexive(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    const DID_SEP: &str = ". if you do, ";
    const NOT_SEP: &str = ". if you don't, ";

    let Some(rest) = strip_prefix_ci(line, "you may ") else {
        return Ok(None);
    };
    // Locate the reflexive boundaries case-insensitively. ASCII-lowercasing
    // preserves byte length, so offsets in `lower` map straight onto `rest`.
    let lower = rest.to_ascii_lowercase();
    let did_at = lower.find(DID_SEP);
    let not_at = lower.find(NOT_SEP);
    // A bare "you may <x>" (no continuation) is parse_may's job.
    let boundary = match (did_at, not_at) {
        (Some(d), Some(n)) => d.min(n),
        (Some(d), None) => d,
        (None, Some(n)) => n,
        (None, None) => return Ok(None),
    };
    let offer = &rest[..boundary];

    // The "if you do" text runs to the "if you don't" boundary (when it opens
    // AFTER it) or the line end; the "if you don't" text runs to the end. Each
    // is a clause with its terminal period restored for re-parsing.
    let did_text = did_at.map(|d| {
        let start = d + DID_SEP.len();
        let end = not_at.filter(|&n| n > d).unwrap_or(rest.len());
        rest[start..end].trim_end_matches('.')
    });
    let not_text = not_at.map(|n| rest[n + NOT_SEP.len()..].trim_end_matches('.'));

    let (targets, effect, functional_zone) = if let Some(cost_body) = strip_prefix_ci(offer, "pay ")
    {
        // MayPay path. `and_then` is required (a pay offer with only a negative
        // branch declines above via not-without-did); the cost is read bare, so
        // energy (`Pay {E}…`) and any other "Pay"-worded macro cost declines.
        let Some(did_text) = did_text else {
            return Ok(None);
        };
        let Some(cost) = crate::parsers::cost::parse_cost(
            cost_body,
            crate::parsers::cost::VariableMana::Allow,
            Some(ctx.index),
        )?
        else {
            return Ok(None);
        };
        let Some(and_then) = parse_clause(&format!("{did_text}."), ctx)? else {
            return Ok(None);
        };
        // A branch announce list has nowhere to bind; a negative tail can't
        // round-trip the semicolon render. Both decline for v1.
        if !and_then.targets.is_empty() || not_text.is_some() {
            return Ok(None);
        }
        (
            Vec::new(),
            format!(
                "MayPay(cost: [{}], and_then: {})",
                cost.join(", "),
                and_then.effect
            ),
            and_then.functional_zone,
        )
    } else {
        // May path. Offer targets carry through; a branch's own targets decline.
        let Some(offer_parsed) = parse_clause(&format!("{offer}."), ctx)? else {
            return Ok(None);
        };
        // Parse an optional branch into its `, <field>: <effect>` RON fragment
        // (empty when the branch is absent). `Ok(None)` declines the whole
        // production: the branch didn't parse, or it declared its own targets,
        // which have no shared announce list to bind against.
        let branch = |text: Option<&str>,
                      field: &str|
         -> anyhow::Result<Option<(String, Option<FunctionalZone>)>> {
            let Some(text) = text else {
                return Ok(Some((String::new(), None)));
            };
            let Some(parsed) = parse_clause(&format!("{text}."), ctx)? else {
                return Ok(None);
            };
            if !parsed.targets.is_empty() {
                return Ok(None);
            }
            Ok(Some((
                format!(", {field}: {}", parsed.effect),
                parsed.functional_zone,
            )))
        };
        let (Some((did_frag, did_zone)), Some((not_frag, not_zone))) =
            (branch(did_text, "if_did")?, branch(not_text, "if_not")?)
        else {
            return Ok(None);
        };
        (
            offer_parsed.targets,
            format!(
                "May(who: You, effect: {}{did_frag}{not_frag})",
                offer_parsed.effect
            ),
            offer_parsed.functional_zone.or(did_zone).or(not_zone),
        )
    };
    Ok(Some(ParsedEffect {
        functional_zone,
        targets,
        effect,
    }))
}

/// `<subject> gets ±N/±N [and gain(s) <kw…>] until end of turn.` (and the
/// keyword-only `<subject> gain(s)/have/has <kw…> until end of turn.`) -> a
/// one-shot continuous effect ([CR#611.2]): `Continuously(effect:
/// Modify(<ref>, <change>)` or `Each(SelectAll(<filter>), Modify(It,
/// <change>))`, `duration: FixedUntil(EndOfTurn))`. The durational marker is
/// required — it's what makes this a one-shot continuous effect rather than an
/// always-on static anthem ([`crate::parsers::static_ability`], which declines
/// the marker). The ±N/±N + keyword-grant grammar is shared with that anthem
/// parser via [`modify`]. A `±N/±N` change folds to its `Modification` macro
/// via the reverse `TemplateIndex` (see [`pump_change_folded`]), exactly as
/// [`crate::parsers::static_ability`]'s `parse_pt` folds the anthem change —
/// unscaled to `PowerAndToughnessUp`/`Down`, a "for each" scaler to
/// `P1P1ForEach` et al.; a keyword grant tail still keeps the inline core
/// changes.
/// Subject: a target ("target creature" -> bare `It` + `TargetOne(<filter>)`),
/// or a team/self class via the shared subject grammar (a bare `Reference` or a
/// distributed class filter).
fn parse_pump(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(body) = line.strip_suffix('.') else {
        return Ok(None);
    };
    // A marker-free copy of `body`, kept for the `Modification`-macro fold
    // below: the durational "until end of turn" marker sits either after the
    // count clause or between the amount and it (the Piledriver/Rabblemaster
    // order — see the branch below), so it's removed by CONTENT rather than
    // position, reconstructing the "gets ±N/±M [for each <selection>]" phrase
    // the fold matches against (the marker itself is never part of a
    // `Modification` template — it's the outer `Continuously` duration).
    let marker_free = body.replace(" until end of turn", "");
    // The required "until end of turn" marker may sit on EITHER side of a "for
    // each" count tail: "gets +1/+1 for each … until end of turn" (marker last)
    // or "gets +2/+0 until end of turn for each …" (marker mid, the
    // Piledriver/Rabblemaster order). Strip a trailing marker first; if it isn't
    // trailing, peel the count and strip the marker off the count's head.
    let (body, scaled) = if let Some(head) = body.strip_suffix(" until end of turn") {
        // Marker last — peel any "for each" count off what precedes it.
        match count::strip(head) {
            Some(c) if matches!(c.binder, count::Binder::ForEach) => (c.head, Some(c.count)),
            Some(_) => return Ok(None),
            None => (head, None),
        }
    } else {
        // Marker not trailing — it must precede a "for each" count tail.
        let Some(c) = count::strip(body) else {
            return Ok(None);
        };
        if !matches!(c.binder, count::Binder::ForEach) {
            return Ok(None);
        }
        let Some(head) = c.head.strip_suffix(" until end of turn") else {
            return Ok(None);
        };
        (head, Some(c.count))
    };
    let Some(changes) = pump_changes(body, scaled.as_deref()) else {
        return Ok(None);
    };
    let change = pump_change_folded(&marker_free, &changes, ctx)?;
    let Some(subject) = pump_subject(body) else {
        return Ok(None);
    };
    let Some((target, targets)) = pump_scope(subject) else {
        return Ok(None);
    };
    Ok(Some(ParsedEffect {
        functional_zone: None,
        targets,
        effect: format!(
            "Continuously(effect: {}, duration: FixedUntil(EndOfTurn))",
            target.wrap(&change)
        ),
    }))
}

/// Fold a `±N/±N` pump change to its `Modification` macro via the reverse
/// [`crate::parsers::static_ability`]-style `TemplateIndex` lookup, falling
/// back to the inline core `changes` otherwise — mirroring `parse_pt` exactly
/// (see its doc note): an unscaled delta folds to `PowerAndToughnessUp`/`Down`
/// via the `Count` slots, a scaled "for each" delta folds to `P1P1ForEach`
/// et al. via the new `Predicate` slot
/// ([`modification_slot_reader`]/[`modification_predicate_slot`]).
/// `marker_free` is `parse_pump`'s marker-stripped-by-content copy of the
/// body (so the "for each <selection>" clause survives here even though
/// `parse_pump`'s own `body` has already had it peeled into `scaled`'s RON).
/// One case still declines the fold: a keyword grant tail (`"gets +2/+2 and
/// gain haste"` — the tail leaves the `"gets …"` phrase only partially
/// matched, failing the full-consumption gate).
fn pump_change_folded(
    marker_free: &str,
    changes: &[String],
    ctx: &ResolveCtx,
) -> anyhow::Result<String> {
    let Some((_, pred)) = modify::split_marker(marker_free, &[" gets ", " get "]) else {
        return Ok(modify::changes_to_modification(changes));
    };
    let gets = format!("gets {}", pred.trim());
    // Fold only on a FULL-consumption match (a grant tail leaves the phrase
    // partially matched → keep the inline changes). A same-kind ambiguous
    // match is a hard generation error (`?`), not a decline.
    let change = match ctx
        .index
        .match_with("Modification", &gets, modification_slot_reader)?
    {
        Some(m) if m.consumed == gets.len() => m.invocation,
        _ => modify::changes_to_modification(changes),
    };
    Ok(change)
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

/// Pump subject -> (`Modify` target, target declarations). A "target
/// <filter>" subject targets bare `It` and declares `TargetOne(<filter>)`;
/// the source anaphor "it" (a self-pump trigger surface, e.g. "it gets +2/+0
/// …") targets bare `This`; a team/self class distributes via the shared
/// subject grammar with no target declaration.
fn pump_scope(subj: &str) -> Option<(modify::Target, Vec<String>)> {
    if let Some(rest) = modify::strip_prefix_ci(subj.trim(), "target ") {
        let filter = filter::parse_phrase(rest)?;
        return Some((
            modify::Target::Ref("Target(0)".to_owned()),
            vec![format!("TargetOne({filter})")],
        ));
    }
    // "it" — the resolving source pumping itself (trigger anaphor); same scope as
    // a "~ gets …" self-pump.
    if subj.trim().eq_ignore_ascii_case("it") {
        return Some((modify::Target::Ref("This".to_owned()), Vec::new()));
    }
    let filter = modify::subject_to_filter(subj)?;
    Some((modify::filter_to_target(&filter), Vec::new()))
}

/// The markers that separate a pump subject from its predicate.
const MODIFY_MARKERS: [&str; 6] = [" gets ", " get ", " gains ", " gain ", " have ", " has "];

/// "<subject> can't block this turn." / "<subject> can't be blocked this
/// turn." -> a one-shot durational combat restriction ([CR#509.1b] —
/// restrictions/evasion abilities checked against the declared/candidate
/// blockers): `Continuously(effect: Cant(Block(by:/on: <ref>)), duration:
/// FixedUntil(EndOfTurn))`. The PASSIVE "can't be blocked" clause is tried
/// FIRST — "block" is a substring of "blocked", so trying the active clause
/// first risks misfiring on the passive wording once either check loosens
/// past an exact suffix match. This is the DURATIONAL sibling of
/// [`crate::parsers::static_ability::parse_restriction`]'s always-on
/// `Cant(Block(...))` (which declines a "this turn"/"until end of turn" line
/// outright); the two never compete for the same input. Subject scope is
/// [`combat_restriction_scope`]. Multi-target ("up to three target creatures
/// …"), riders ("with power 2 or less"), and distributed class subjects
/// ("Creatures without flying can't block this turn.") are out of scope here.
fn parse_combat_restriction(line: &str) -> Option<ParsedEffect> {
    let body = line.strip_suffix('.')?;
    if let Some(subj) = body.strip_suffix(" can't be blocked this turn") {
        let (reference, targets) = combat_restriction_scope(subj)?;
        return Some(ParsedEffect {
            functional_zone: None,
            targets,
            effect: format!(
                "Continuously(effect: Cant(Block(on: {reference})), \
                 duration: FixedUntil(EndOfTurn))"
            ),
        });
    }
    let subj = body.strip_suffix(" can't block this turn")?;
    let (reference, targets) = combat_restriction_scope(subj)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets,
        effect: format!(
            "Continuously(effect: Cant(Block(by: {reference})), \
             duration: FixedUntil(EndOfTurn))"
        ),
    })
}

/// Combat-restriction subject -> (`Cant(Block(...))`'s `by:`/`on:` `Predicate`
/// value, target declarations) — the `Ref(<Reference>)`-wrapped sibling of
/// [`pump_scope`]'s bare-`Reference` `Modify` target (`Cant`'s `by`/`on` slots
/// are `Predicate`, not `Reference`, so every anchor here wraps in
/// `Ref(...)`). "target <filter>" hoists a `TargetOne(<filter>)` declaration
/// and anchors on `Ref(It)`; "~" anchors on `Ref(This)`; "that <type noun>"
/// anchors on the sorted anaphor `Ref(That(<Type>))` ([CR#608.2d]-style
/// antecedent — e.g. "Chandra deals 1 damage to … a creature… That creature
/// can't block this turn."); bare "it" anchors on `Ref(It)` with NO new
/// target of its own — the plain anaphor reading a target an earlier clause
/// in a `Sequentially` chain already declared. Anything else (a distributed
/// class subject, a rider) declines.
fn combat_restriction_scope(subj: &str) -> Option<(String, Vec<String>)> {
    let subj = subj.trim();
    if let Some(rest) = modify::strip_prefix_ci(subj, "target ") {
        let filter = filter::parse_phrase(rest)?;
        return Some((
            "Ref(Target(0))".to_owned(),
            vec![format!("TargetOne({filter})")],
        ));
    }
    if let Some(rest) = modify::strip_prefix_ci(subj, "that ") {
        let ty = filter::type_code(&filter::singularize(rest.trim()).to_ascii_lowercase())?;
        return Some((format!("Ref(That({ty}))"), Vec::new()));
    }
    if subj.eq_ignore_ascii_case("it") {
        return Some(("Ref(Target(0))".to_owned(), Vec::new()));
    }
    if subj == "~" {
        return Some(("Ref(This)".to_owned(), Vec::new()));
    }
    None
}

/// `Destroy target <subject>.` -> a `TargetOne(<filter>)` declaration (the
/// subject parsed by the shared [`object_target_filter`] grammar — the shared
/// [`filter`] phrase grammar, broadened with a type-noun disjunction for
/// "<type> or <type>" subjects like "artifact or enchantment") and the body
/// `Destroy(It)` ([CR#701.8]). Only the single-target form; board wipes
/// ("destroy all/each …") are a later production. Declines when the subject
/// isn't filter-parseable. Case-insensitive lead, since the clause opens a
/// spell ("Destroy …") or follows a trigger comma ("…, destroy …").
/// Sentence-order `Sequentially` ([CR#608.2c]): a multi-sentence effect line
/// splits into its sentences and parses each as a clause, in ORACLE ORDER —
/// the telescope surface (each clause elaborates in the context extended by
/// its left siblings' introductions; no binder inversion). Targets may be
/// declared only by the FIRST sentence (a second announce list would
/// collide with the first's); later sentences read the announced slot or the
/// products through the anaphors (`That(Card)`, …). Declines unless every
/// sentence parses.
fn parse_sequence(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(sentences) = split_sentences(line) else {
        return Ok(None);
    };
    // Fold a mid-sequence `you may … . If you do/don't, …` run back into one
    // sentence so [`parse_may_reflexive`] sees the offer and its continuation
    // whole. A fully-coalesced single clause was already offered to every
    // production (reflexive included) at the top level, so there is nothing
    // left for a SEQUENCE to add — bow out (this also stops the coalesce ->
    // parse_clause -> parse_sequence recursion from looping).
    let sentences = coalesce_reflexive(&sentences);
    if sentences.len() < 2 {
        return Ok(None);
    }
    let mut parts: Vec<ParsedEffect> = Vec::with_capacity(sentences.len());
    for sentence in &sentences {
        let Some(parsed) = parse_clause(sentence, ctx)? else {
            return Ok(None);
        };
        parts.push(parsed);
    }
    if parts[1..].iter().any(|p| !p.targets.is_empty()) {
        return Ok(None);
    }
    let functional_zone = parts.iter().find_map(|part| part.functional_zone);
    let effects: Vec<String> = parts.iter().map(|p| p.effect.clone()).collect();
    Ok(Some(ParsedEffect {
        functional_zone,
        targets: parts[0].targets.clone(),
        effect: format!("Sequentially([{}])", effects.join(", ")),
    }))
}

/// Splits a multi-sentence line into its ". "-separated sentences, each with
/// its terminal period restored. `None` unless there are at least two —
/// single sentences stay with the bespoke productions (and avoid the
/// [`parse_sequence`] recursion re-splitting them).
fn split_sentences(line: &str) -> Option<Vec<&str>> {
    if !line.ends_with('.') || !line.contains(". ") {
        return None;
    }
    let mut sentences = Vec::new();
    let mut rest = line;
    while let Some(split) = rest.find(". ") {
        let (sentence, tail) = rest.split_at(split + 1);
        sentences.push(sentence);
        rest = &tail[1..];
    }
    sentences.push(rest);
    (sentences.len() >= 2).then_some(sentences)
}

/// Merge the FIRST `you may …` sentence that is immediately followed by an
/// `If you do,`/`If you don't,` continuation — together with every sentence
/// after it — into ONE combined sentence, so a reflexive fold buried
/// mid-sequence (e.g. `mill three cards. You may put …. If you don't, …`)
/// reaches [`parse_may_reflexive`] as a single clause. Sentences BEFORE the
/// fold stay separate; the greedy tail lets the reflexive parser own the
/// branch extent (a trailing delayed-trigger sentence folds into the branch).
/// Returns owned strings (the join allocates); a line with no such fold is
/// returned verbatim, so the common multi-sentence sequence is unaffected.
fn coalesce_reflexive(sentences: &[&str]) -> Vec<String> {
    let fold_at = (0..sentences.len()).find(|&i| {
        strip_prefix_ci(sentences[i], "you may ").is_some() && i + 1 < sentences.len() && {
            let next = sentences[i + 1].to_ascii_lowercase();
            next.starts_with("if you do,") || next.starts_with("if you don't,")
        }
    });
    match fold_at {
        Some(i) => {
            let mut out: Vec<String> = sentences[..i].iter().map(|s| (*s).to_owned()).collect();
            out.push(sentences[i..].join(" "));
            out
        }
        None => sentences.iter().map(|s| (*s).to_owned()).collect(),
    }
}

/// The delayed-trigger template ([CR#603.7]): "At the beginning of the next
/// end step, <clause>" -> `Delayed(event: StepBegins(at: Ending(End),
/// whose: EachPlayers), effect: <clause>)` — a one-shot schedule created on
/// resolution (fire-once is `OneShotEffect::Delayed`'s own semantics,
/// [CR#603.7c]). The inner clause may not declare targets (a delayed body
/// reads products, never the spell's slots — they are dropped, [CR#603.7c]).
fn parse_delayed_next_end_step(
    line: &str,
    ctx: &ResolveCtx,
) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(rest) = strip_prefix_ci(line, "at the beginning of the next end step, ") else {
        return Ok(None);
    };
    let mut capitalized = String::with_capacity(rest.len());
    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return Ok(None);
    };
    capitalized.extend(first.to_uppercase());
    capitalized.push_str(chars.as_str());
    let Some(inner) = parse_clause(&capitalized, ctx)? else {
        return Ok(None);
    };
    if !inner.targets.is_empty() {
        return Ok(None);
    }
    Ok(Some(ParsedEffect {
        functional_zone: inner.functional_zone,
        targets: Vec::new(),
        effect: format!(
            "Delayed(event: StepBegins(at: Ending(End), whose: EachPlayers), effect: {})",
            inner.effect
        ),
    }))
}

/// `Exile target <subject>.` -> a targeted exile: `Move(It, Exile)`
/// ([CR#701.13a]; the slot spelling stays the runtime-supported read). The
/// exile clause's PRODUCT — the exiled card — is what a following sentence's
/// `That(Card)` resolves to ([CR#400.7j]; the Otherworldly-Journey chain).
///
/// A special-cased subject peels first: `Exile target [<type>] card from a
/// graveyard.` ([CR#400.7], the graveyard-hate family) — ANY player's
/// graveyard, unlike the graveyard-recursion family's "your graveyard"
/// (`graveyard_card_filter`). [`any_graveyard_card_filter`] builds the
/// owner-agnostic twin of that helper; a general (non-graveyard) subject
/// falls through to the shared [`object_target_filter`] grammar as before.
fn parse_exile_target(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "exile ")?
        .strip_suffix('.')?
        .strip_prefix("target ")?;
    if let Some(noun) = subject.strip_suffix(" from a graveyard") {
        let noun = noun
            .strip_suffix(" card")
            .or_else(|| (noun == "card").then_some(""))?;
        let filter = any_graveyard_card_filter(noun)?;
        return Some(ParsedEffect {
            functional_zone: None,
            targets: vec![format!("TargetOne({filter})")],
            effect: "Move(Target(0), Exile)".to_owned(),
        });
    }
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: "Move(Target(0), Exile)".to_owned(),
    })
}

/// `Return that card to the battlefield[ under its owner's control][
/// tapped][ with a +1/+1 counter on it].` -> `Move(That(Card),
/// Battlefield[, riders])` — the sorted anaphor reads the nearest card
/// product ([CR#400.7j]; typically the exile clause to its left, surviving
/// into a `Delayed` body per [CR#603.7c]), and the postposed adjuncts ride
/// as enter riders ([CR#614.12]).
fn parse_return_that_card(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "return that card to the battlefield")?.strip_suffix('.')?;
    let mut riders: Vec<&str> = Vec::new();
    let mut rest = body;
    if let Some(tail) = rest.strip_prefix(" under its owner's control") {
        riders.push("UnderOwnersControl");
        rest = tail;
    }
    if let Some(tail) = rest.strip_prefix(" tapped") {
        riders.push("Tapped");
        rest = tail;
    }
    if let Some(tail) = rest.strip_prefix(" with a +1/+1 counter on it") {
        riders.push("WithCounters(P1P1Counter, 1)");
        rest = tail;
    }
    if !rest.is_empty() {
        return None;
    }
    let effect = if riders.is_empty() {
        "Move(That(Card), Battlefield)".to_owned()
    } else {
        format!("Move(That(Card), Battlefield, [{}])", riders.join(", "))
    };
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect,
    })
}

fn parse_destroy(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "destroy ")?
        .strip_suffix('.')?
        .strip_prefix("target ")?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: "Destroy(Target(0))".to_owned(),
    })
}

/// The destroy family has two grammar shapes that deliberately route through
/// builtin macros rather than duplicating their semantics here:
///
/// - "Destroy target X. It can't be regenerated." uses `DestroyNoRegen(It)`.
/// - A target noun phrase supplied by a nullary `Predicate` macro (currently
///   "nonbasic land") uses that macro as the target filter.
fn parse_destroy_no_regen(line: &str) -> Option<ParsedEffect> {
    let subject =
        strip_prefix_ci(line, "destroy target ")?.strip_suffix(". It can't be regenerated.")?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: "DestroyNoRegen(Target(0))".to_owned(),
    })
}

fn parse_destroy_macro_target(
    line: &str,
    ctx: &ResolveCtx,
) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(subject) =
        strip_prefix_ci(line, "destroy target ").and_then(|rest| rest.strip_suffix('.'))
    else {
        return Ok(None);
    };
    let Some(matched) = ctx.index.match_kind("Predicate", subject)? else {
        return Ok(None);
    };
    Ok(Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({})", matched.macro_name)],
        effect: "Destroy(Target(0))".to_owned(),
    }))
}

/// Self-sacrifice productions ([CR#701.16], the "sacrifice it/~" family that
/// rides trigger bodies):
/// - `Sacrifice it.` / `Sacrifice ~.` -> `Sacrifice(You, This)`, no target. The
///   "it" anaphor in a trigger body is the resolving source ([CR#113.7] — the
///   permanent whose ability triggered), the same `This` the `~` self-reference
///   names; both normalize to the source. This is the resolution of "When ~
///   becomes the target …, sacrifice it." and "At the beginning of the end
///   step, sacrifice ~."
/// - `Sacrifice it/~ unless you pay <cost>.` -> the "unless you pay" toll
///   ([CR#118.12a]): the controller may pay the stated cost to keep the
///   permanent, else sacrifices it. Wrapped in an
///   [`Unless`](deckmaste_core::OneShotEffect::Unless) whose payer is the
///   default `You` (the controller — the trigger fires on your own upkeep).
///   Only a single mana cost is modeled (the overwhelmingly common upkeep tax);
///   a richer toll declines. Mirrors the kw-echo macro's `Unless(effect:
///   Sacrifice(You, This), unless: Param(0))` resolution shape.
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
            functional_zone: None,
            targets: Vec::new(),
            effect: "Sacrifice(You, This)".to_owned(),
        });
    };
    // "unless you pay <cost>." — the controller's optional mana toll.
    let cost = toll.strip_prefix("unless you pay ")?;
    let cost =
        crate::parsers::cost::parse_cost(cost, crate::parsers::cost::VariableMana::Decline, None)
            .ok()
            .flatten()?;
    if cost.len() != 1 {
        return None;
    }
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!(
            "Unless(effect: Sacrifice(You, This), unless: [{}])",
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
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: "Attach(what: This, to: Target(0))".to_owned(),
    })
}

/// `Counter target spell[ unless its controller pays <cost>].` -> a
/// `TargetOne(Spell)` target on the stack and a `Counter(It)` body
/// ([CR#701.6a]). The "unless its controller pays" rider wraps the counter in
/// an [`Unless`](deckmaste_core::OneShotEffect::Unless) ([CR#118.12a]): the
/// spell's controller (`who: ControllerOf(It)`) may pay the stated cost to stop
/// the counter. Only the bare and the mana-tax riders parse; richer riders
/// (replacement clauses, "you may cast …", restricted spell filters) are later
/// productions. Case-insensitive lead (spell clause vs. trigger comma).
fn parse_counter(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "counter target spell")?.strip_suffix('.')?;
    let targets = vec!["TargetOne(Spell)".to_owned()];
    // Bare "Counter target spell."
    if body.is_empty() {
        return Some(ParsedEffect {
            functional_zone: None,
            targets,
            effect: "Counter(Target(0))".to_owned(),
        });
    }
    // "… unless its controller pays <cost>." — the spell's controller may pay
    // to avoid the counter. Only a single mana cost is modeled here (the
    // overwhelmingly common "{N}" tax); a "for each …" scaled tax declines.
    let cost = body.strip_prefix(" unless its controller pays ")?;
    let cost =
        crate::parsers::cost::parse_cost(cost, crate::parsers::cost::VariableMana::Decline, None)
            .ok()
            .flatten()?;
    if cost.len() != 1 {
        return None;
    }
    Some(ParsedEffect {
        functional_zone: None,
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
/// - `on each <subject>.` -> the effect-level `Each` distribution over the
///   matching set (`Each(binder: Existing(SelectAll(<filter>)), effect:
///   PutCounters(It, <kind>, <n>))`), the placement running once per member as
///   the iteration anaphor `It` ([CR#608.2d]). No target — a distributive
///   "each" announces nothing. Reuses the same [`object_target_filter`] grammar
///   (a controller postfix, a subtype/type adjective, a disjunction, …) as the
///   targeted arm, so any filter it already models sweeps too — the counter
///   twin of the damage-sweeper's `each <subject>` arm ([`damage_target`]). The
///   chosen-target `each of up to <n> target …` form declines here (it needs a
///   bounded-choice target selection, a separate seam).
/// - `on it.` / `on ~.` -> `PutCounters(This, <kind>, <n>)`, no target — the
///   resolving source counters itself (the combat-damage trigger surface).
///
/// The kind is macro-resolved (so `-1/-1` -> `M1M1Counter` for free); fixed
/// counts only (a "for each"/`X`/"that many" count declines — a later scaled
/// production). Case-insensitive lead (spell clause vs. trigger comma).
fn parse_put_counters(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    let Some(body) = strip_prefix_ci(line, "put ").and_then(|b| b.strip_suffix('.')) else {
        return Ok(None);
    };
    // Split the counter clause from its destination at the LAST " on " (a
    // counter-kind phrase never contains " on ").
    let Some((counter_clause, dest)) = body.rsplit_once(" on ") else {
        return Ok(None);
    };
    let Some((count, kind)) = parse_counter_clause(counter_clause, ctx)? else {
        return Ok(None);
    };
    // Destination -> (target declarations, effect). The self and mass arms
    // announce no target; the targeted arm declares its announce slot.
    let (targets, effect) = match dest {
        // Self placement: the resolving source ("it" — a trigger anaphor — or
        // "~"). No target.
        "it" | "~" => (Vec::new(), format!("PutCounters(This, {kind}, {count})")),
        // Mass placement: "each <subject>" — the effect-level `Each` binds each
        // matching member as the per-element anaphor `It` and runs the
        // single-object placement body per element ([CR#608.2d]).
        _ if dest.starts_with("each ") => {
            let Some(subject) = dest.strip_prefix("each ") else {
                return Ok(None);
            };
            let Some(filter) = object_target_filter(subject) else {
                return Ok(None);
            };
            (
                Vec::new(),
                format!(
                    "Each(binder: Existing(SelectAll({filter})), \
                     effect: PutCounters(It, {kind}, {count}))"
                ),
            )
        }
        // Targeted placement: "target <subject>".
        _ => {
            let Some(subject) = dest.strip_prefix("target ") else {
                return Ok(None);
            };
            let Some(filter) = object_target_filter(subject) else {
                return Ok(None);
            };
            (
                vec![format!("TargetOne({filter})")],
                format!("PutCounters(Target(0), {kind}, {count})"),
            )
        }
    };
    Ok(Some(ParsedEffect {
        functional_zone: None,
        targets,
        effect,
    }))
}

/// `<count> <kind> counter[s]` (the clause before "on …") -> `(count RON, kind
/// RON)`. The count is a fixed cardinal emitted as a bare numeral
/// (reader-sugar for `Count::Literal`, like the sibling `Draw`/`Create`
/// productions); the kind is the `Counter`-kind macro name the "+1/+1 counter"
/// phrase resolves to. Declines a non-cardinal count (`X`, "that many") and an
/// unmodeled counter kind. Shared with the enters-with-counters replacement
/// production ([`crate::parsers::replacement`]).
pub(super) fn parse_counter_clause(
    clause: &str,
    ctx: &ResolveCtx,
) -> anyhow::Result<Option<(u32, String)>> {
    let Some((count_word, rest)) = clause.split_once(' ') else {
        return Ok(None);
    };
    let Some(count) = number_word(count_word) else {
        return Ok(None);
    };
    // Re-singularize the counter-noun so the singular macro template ("+1/+1
    // counter") matches regardless of the count's plurality ("two +1/+1
    // counters").
    let phrase = rest.strip_suffix('s').unwrap_or(rest);
    let Some(kind) = counter_kind(phrase, ctx)? else {
        return Ok(None);
    };
    Ok(Some((count, kind)))
}

/// A counter-kind phrase ("+1/+1 counter", "-1/-1 counter") -> the macro NAME
/// it resolves to (`P1P1Counter`, `M1M1Counter`), routed through the
/// `Counter`-kind reverse index. Full-line consumption (and same-kind
/// ambiguity) is judged inside the matcher; an unmodeled kind declines.
fn counter_kind(phrase: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    Ok(ctx
        .index
        .match_kind("Counter", phrase)?
        .map(|m| m.macro_name.to_string()))
}

/// Return-to-hand productions ([CR#400.7], the bounce family) — every arm
/// emits the `Move(_, Hand)` primitive (`Action::ReturnToHand` retired; a
/// hand destination is exactly as unremarkable as any other zone move):
/// - `Return target <subject> to its owner's hand.` -> battlefield bounce via
///   `Move(It, Hand)`, the subject parsed by [`object_target_filter`].
/// - `Return ~ to its owner's hand.` / `Return ~ to your hand.` -> a
///   self-bounce (`Move(This, Hand)`), no target — the effect body of `{cost}:
///   Return ~ to its owner's hand.` activated abilities (the cost is the
///   activated-frame's job). "to your hand" is the equally-correct common idiom
///   (`Move(_, Hand)` always lands in the owner's hand regardless of which
///   possessive the oracle text prints).
/// - `Return that card to your hand.` -> the same product-sited anaphor
///   [`parse_return_that_card`] reads for a battlefield return ([CR#400.7j]) —
///   `Move(That(Card), Hand)` — landing in hand instead.
/// - `Return target <subject> card from your graveyard to your hand.` -> a
///   graveyard-to-hand recursion: a plain zone change ([CR#400.7]) of a card
///   you own in your graveyard, via `Move(It, Hand)`. The subject is a *card*
///   (graveyard zone), so it's the card-type spelling (`Type(Creature)`), not
///   the battlefield-scoped `Creature` macro, scoped `InZone(Graveyard)` +
///   `Owner(Ref(You))`.
///
/// - `Return a/another <subject> you control to its owner's hand.` -> a
///   non-target *chosen-subject* bounce ([CR#400.3]): the controller picks one
///   permanent they control (which they may not own — hence "its owner's"), a
///   `With(ChooseOne(<filter>), Move(That(Permanent), Hand))` distinct from the
///   *targeted* bounce. The leading determiner (`a`/`an`/`another`/`other`)
///   fixes N=1 and stays in the phrase so the filter grammar reads `another` as
///   self-exclusion — the same discipline as [`super::cost`]'s `sacrifice`.
fn parse_return_to_hand(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "return ")?.strip_suffix('.')?;
    // Self-bounce: "Return ~/it to its owner's hand." or "...to your hand."
    // — an activated-ability effect ("~") or a trigger body whose "it"
    // anaphor names the resolving source ([CR#113.7]); every phrasing is the
    // source permanent (`This`).
    if matches!(
        body,
        "~ to its owner's hand" | "it to its owner's hand" | "~ to your hand" | "it to your hand"
    ) {
        return Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: "Move(This, Hand)".to_owned(),
        });
    }
    if body == "~ from your graveyard to your hand" {
        return Some(ParsedEffect {
            functional_zone: Some(FunctionalZone::Graveyard),
            targets: Vec::new(),
            effect: "Move(This, Hand)".to_owned(),
        });
    }
    // Self/product anaphor: "Return that card to your hand." — the same
    // "newest object this resolution moved to a public zone" antecedent
    // [`parse_return_that_card`] reads, just landing in hand.
    if body == "that card to your hand" {
        return Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: "Move(That(Card), Hand)".to_owned(),
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
            functional_zone: None,
            targets: vec![format!("TargetOne({card_filter})")],
            effect: "Move(Target(0), Hand)".to_owned(),
        });
    }
    // Battlefield bounce: "target <subject> to its owner's hand." (targeted) or
    // "a/another <subject> to its owner's hand." (chosen, non-target).
    let subject = body.strip_suffix(" to its owner's hand")?;
    if let Some(target_subject) = subject.strip_prefix("target ") {
        let filter = object_target_filter(target_subject)?;
        return Some(ParsedEffect {
            functional_zone: None,
            targets: vec![format!("TargetOne({filter})")],
            effect: "Move(Target(0), Hand)".to_owned(),
        });
    }
    // Chosen-subject bounce ([CR#400.3]): the controller picks one permanent
    // among a filtered set, not a target. The determiner fixes N=1 and stays in
    // the phrase so the filter grammar reads "another" as self-exclusion.
    let (determiner, _) = subject.split_once(' ')?;
    if !matches!(
        determiner.to_ascii_lowercase().as_str(),
        "a" | "an" | "another" | "other"
    ) {
        return None;
    }
    // A determiner alone cannot replace targeting: the non-target form is
    // licensed only when the choice is restricted to permanents the resolving
    // player controls. Otherwise this would silently turn an ordinary bounce
    // spell into a controller-unbounded `ChooseOne`, bypassing target rules.
    let parsed_filter = filter::parse_phrase_detailed(subject)?;
    if !parsed_filter.is_controlled_by(filter::FilterController::You) {
        return None;
    }
    let filter = parsed_filter.into_predicate();
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!(
            "With(binder: ChooseOne(filter: {filter}), body: Move(That(Permanent), Hand))"
        ),
    })
}

/// Graveyard reanimation ([CR#400.7]) — the graveyard→BATTLEFIELD twin of
/// [`parse_return_to_hand`]'s graveyard→hand arm, reusing the same
/// [`graveyard_card_filter`] helper and only swapping the destination:
/// - `Return target <subject> card from your graveyard to the battlefield.` ->
///   `TargetOne(graveyard_card_filter(<subject>))` + `Move(It, Battlefield)`.
/// - `Return ~ from your graveyard to the battlefield.` / `Return it from your
///   graveyard to the battlefield.` -> self-reanimation, no target (`Move(This,
///   Battlefield)`).
///
/// No [`EnterRider`](deckmaste_core::EnterRider) is emitted: a "your
/// graveyard" subject is already owned by the resolving player, and the
/// engine derives battlefield-entry control from the object's stored
/// `controller` field, which is forced to the owner while off the
/// battlefield — so a bare arrival already lands under the owner's control.
/// `UnderOwnersControl` would be redundant here, and riders aren't executed
/// by the engine yet (a `todo!()` in the zone-move apply path), so adding one
/// would invite a panic rather than express anything true. "entering
/// tapped", "with a finality counter on it", "attached to that creature"
/// (riders), a mana-value filter, and "up to two target … cards"
/// (multi-target) all leave a trailing/leading clause neither match arm
/// below strips, so those decline (`None`) rather than mis-parse — reported
/// as a follow-up, not built here.
/// The library-search / tutor family ([CR#701.23a]): `Search your library
/// for <filter>[, reveal <pronoun>], put <pronoun> <destination>[ tapped],
/// then shuffle.` -> `With(binder: SearchOne(filter: <predicate>), body:
/// Sequentially([Reveal(what: That(Card))?, Move(That(Card), <zone>,
/// <riders>), Shuffle(LibraryOf(You))]))`. `SearchOne`'s `by`/`whose`/`from`
/// all stay defaulted (`You`/`You`/`[Library]`) — every card this production
/// covers is a self-search of one's own library; a foreign subject ("its
/// controller may search their library …") is a distinct `Binder::by`/`by:
/// EventActor` shape this production doesn't attempt, and a graveyard-search
/// twin (`from`) likewise declines.
///
/// Only ONE found card (`SearchOne`, quantity always exactly one) is built
/// here: a plural "up to N … cards" search needs a group verb
/// ([`Action::MoveGroup`]) this migration doesn't yet produce, and a plural
/// reveal has no primitive at all (`PlayerAction::Reveal.what` is a single
/// [`Reference`], never a group) — both stay `Unparsed` rather than emit a
/// lossy/wrong structure. Likewise declined: "a card named …" (self-name
/// search — no established `Named` self-reference convention yet), a
/// dynamic-count filter ("… with mana value X"), a heterogeneous multi-find
/// ("a Zombie card and a Swamp card" — two DIFFERENT filters, not one), and
/// any destination outside hand/battlefield/graveyard (attach riders,
/// control-changing riders, a foreign "their hand").
fn parse_search_library(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "search your library for ")?.strip_suffix('.')?;
    let body = body.strip_suffix(", then shuffle")?;
    let (head, reveal, zone, tapped) = search_tail(body)?;
    let filter = search_card_filter(head.trim())?;
    let riders = if tapped { "[Tapped]" } else { "[]" };
    let mut parts = Vec::with_capacity(3);
    if reveal {
        parts.push("Reveal(what: That(Card))".to_owned());
    }
    parts.push(format!("Move(That(Card), {zone}, {riders})"));
    parts.push("Shuffle(LibraryOf(You))".to_owned());
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!(
            "With(binder: SearchOne(filter: {filter}), body: Sequentially([{}]))",
            parts.join(", ")
        ),
    })
}

/// Peels the trailing "[reveal <pronoun>, ]put <pronoun> <destination>"
/// clause off a search body (the ", then shuffle" tail already stripped by
/// the caller), returning `(head, reveal, zone RON, tapped)`. The pronoun
/// reads either "it"/"them" or the equally-common "that card"/"those cards"
/// register — both oracle idioms for the same found-card anaphor
/// ([CR#701.23a]), so both parse to the identical structure; only the
/// singular register is built (see [`parse_search_library`]'s doc for the
/// plural gap). `head` still carries the filter phrase verbatim, commas and
/// all ("a basic Plains, Swamp, or Forest card") — the suffix match is exact,
/// so it can't misfire on the filter's own internal commas.
fn search_tail(body: &str) -> Option<(&str, bool, &'static str, bool)> {
    const CANDIDATES: &[(&str, bool, &str, bool)] = &[
        (", reveal it, put it into your hand", true, "Hand", false),
        (
            ", reveal that card, put it into your hand",
            true,
            "Hand",
            false,
        ),
        (", put it into your hand", false, "Hand", false),
        (", put that card into your hand", false, "Hand", false),
        (
            ", put it onto the battlefield tapped",
            false,
            "Battlefield",
            true,
        ),
        (
            ", put that card onto the battlefield tapped",
            false,
            "Battlefield",
            true,
        ),
        (", put it onto the battlefield", false, "Battlefield", false),
        (
            ", put that card onto the battlefield",
            false,
            "Battlefield",
            false,
        ),
        (", put it into your graveyard", false, "Graveyard", false),
        (
            ", put that card into your graveyard",
            false,
            "Graveyard",
            false,
        ),
    ];
    for (suffix, reveal, zone, tapped) in CANDIDATES {
        if let Some(head) = body.strip_suffix(suffix) {
            return Some((head, *reveal, zone, *tapped));
        }
    }
    None
}

/// The library-search filter grammar: an object-description phrase headed by
/// a determiner ("a"/"an") describing a CARD (not a battlefield permanent —
/// the library-search twin of [`graveyard_card_type`]) -> its `Predicate`
/// RON, or `None` for an unmodeled phrase. Recognizes:
/// - bare "a card" -> `Kind(Card)`.
/// - a plain card-type noun ("a creature card", "an artifact card", "a land
///   card") -> `Type("<T>")`, optionally with a color/color-count adjective ("a
///   green creature card" -> `And([Type("Creature"), ColorIs(Green)])`; "a
///   colorless artifact card" -> `And([Type("Artifact"), Colorless])`).
/// - "a basic land card" / "a basic <Subtype>[, <Subtype>, or <Subtype>] card"
///   -> `Supertype(Basic)` (plus a `Subtype`/`Or([Subtype, …])` when a specific
///   land type is named — the oracle text never redundantly repeats "land"
///   alongside a named subtype, so the type word is implicit and supplied here;
///   "land" itself IS emitted as `Type("Land")` since it's the printed word,
///   not an inferred category).
/// - "a snow land card" -> `Supertype(Snow)`.
/// - a bare subtype (or subtype "or"-list), no "basic": "a Forest card", "an
///   Equipment card", "a Swamp or Mountain card", "a Goblin card" -> the
///   `Subtype`/`Or([…])` atom ALONE — no parent-Type wrapper. [CR#205.3m]:
///   Tribal cards give a printed creature subtype to a NONCREATURE card
///   (Tarfire is a Tribal Instant — Goblin), so injecting the subtype's typical
///   parent type (via [`filter::subtype_category`]) would wrongly exclude it —
///   Goblin Matron's "a Goblin card" must find Tarfire. A "<Subtype> permanent
///   card" phrase ("a Dragon permanent card") is NOT modeled — see
///   [`search_card_descriptor`]'s doc.
/// - "<X> card or a <Y> card" -> `Or([<X>, <Y>])`, each side recursively this
///   same grammar (Wayfarer's Bauble's "a basic land card or a Desert card").
fn search_card_filter(head: &str) -> Option<String> {
    if let Some((left, right)) = split_full_phrase_or(head) {
        return Some(format!(
            "Or([{}, {}])",
            search_card_filter(left.trim())?,
            search_card_filter(right.trim())?
        ));
    }
    let phrase = strip_prefix_ci(head, "a ").or_else(|| strip_prefix_ci(head, "an "))?;
    let descriptor = phrase
        .strip_suffix(" card")
        .or_else(|| (phrase == "card").then_some(""))?;
    search_card_descriptor(descriptor)
}

/// Splits "<X> or a <Y>" / "<X> or an <Y>" at the first " or " boundary whose
/// right side re-opens with its own determiner — the marker that
/// distinguishes a top-level disjunction of two FULL noun phrases from a
/// bare subtype-list disjunction inside one phrase ("a Swamp or Mountain
/// card" has no determiner after "or", so it stays one phrase for
/// [`search_card_descriptor`] to split instead).
fn split_full_phrase_or(head: &str) -> Option<(&str, &str)> {
    let idx = head.find(" or ")?;
    let left = &head[..idx];
    let right = &head[idx + " or ".len()..];
    (right.starts_with("a ") || right.starts_with("an ")).then_some((left, right))
}

/// The determiner-stripped, "card"/"cards"-suffix-stripped descriptor ->
/// `Predicate` RON. See [`search_card_filter`] for the shapes.
fn search_card_descriptor(d: &str) -> Option<String> {
    if d.is_empty() {
        return Some("Kind(Card)".to_owned());
    }
    if let Some(rest) = strip_prefix_ci(d, "basic ") {
        if rest.eq_ignore_ascii_case("land") {
            return Some("And([Type(Land), Supertype(Basic)])".to_owned());
        }
        // `category` VALIDATES the word(s) (a real land subtype, and — for a
        // list — every member sharing one category); it is NOT emitted — see
        // the bare-subtype arm below for why a parent-Type atom is wrong.
        let (category, subtype_expr) = subtype_list_predicate(rest)?;
        if category != "Land" {
            return None;
        }
        return Some(format!("And([Supertype(Basic), {subtype_expr}])"));
    }
    if let Some(rest) = strip_prefix_ci(d, "snow ") {
        return rest
            .eq_ignore_ascii_case("land")
            .then(|| "And([Type(Land), Supertype(Snow)])".to_owned());
    }
    // A color/color-count adjective ("a green creature card", "a colorless
    // artifact card", "a multicolored creature card").
    if let Some((color_atom, rest)) = filter::strip_color(d)
        && let Some(ty) = graveyard_card_type(rest)
    {
        return Some(format!("And([{ty}, {color_atom}])"));
    }
    if let Some(ty) = graveyard_card_type(d) {
        return Some(ty);
    }
    // A bare subtype (or subtype "or"-list) — "a Forest card", "a Swamp or
    // Mountain card", "a Goblin card". The SUBTYPE ALONE is the CR-correct
    // filter [CR#205.3m]: a Tribal card gives its printed creature subtype to
    // a NONCREATURE card (Tarfire is a Tribal Instant — Goblin), so injecting
    // the subtype's typical parent type would wrongly exclude it — Goblin
    // Matron's "a Goblin card" must find Tarfire. `subtype_category` still
    // VALIDATES the word(s) here (a real catalog subtype, and — for a list —
    // every member sharing one category, a conservative guard against e.g.
    // "a Swamp or Equipment card"); its result is never emitted.
    //
    // NOTE: a trailing "permanent" qualifier ("a Dragon permanent card", "a
    // Rebel permanent card") is NOT stripped here either: "permanent" is a
    // card-type-CLASS constraint (any permanent type), which no existing
    // grammar expresses — this declines rather than silently drop it.
    let (_category, subtype_expr) = subtype_list_predicate(d)?;
    Some(subtype_expr)
}

/// A bare subtype word, or an "or"-list of them ("Plains", "Swamp or
/// Mountain", "Plains, Swamp, or Forest") -> (parent card-type category,
/// `Subtype`/`Or([Subtype, …])` RON). The category is a VALIDATION signal —
/// callers use it to gate a same-category list (a "Swamp or Equipment"
/// cross-category list is unmodeled) or a "basic"-land check; it is never
/// itself emitted into the predicate (see [`search_card_descriptor`]'s doc —
/// a subtype alone is the CR-correct filter, [CR#205.3m]). An unknown subtype
/// declines the whole clause.
fn subtype_list_predicate(text: &str) -> Option<(&'static str, String)> {
    let members = split_or_list(text);
    let mut category: Option<&'static str> = None;
    let mut atoms = Vec::with_capacity(members.len());
    for member in &members {
        if member.is_empty() || member.contains(' ') {
            return None;
        }
        let cat = filter::subtype_category(member)?;
        match category {
            Some(c) if c != cat => return None,
            _ => category = Some(cat),
        }
        atoms.push(format!("Subtype({})", crate::ident::to_rust_ident(member)));
    }
    let expr = if atoms.len() == 1 {
        atoms.into_iter().next().unwrap()
    } else {
        format!("Or([{}])", atoms.join(", "))
    };
    Some((category?, expr))
}

/// Splits an "A"/"A or B"/"A, B, or C" list (the search-filter subtype-list
/// register — no repeated determiner, unlike [`split_full_phrase_or`]) into
/// its members.
fn split_or_list(text: &str) -> Vec<&str> {
    if let Some(idx) = text.rfind(", or ") {
        let mut members: Vec<&str> = text[..idx].split(", ").map(str::trim).collect();
        members.push(text[idx + ", or ".len()..].trim());
        return members;
    }
    if let Some((a, b)) = text.split_once(" or ") {
        return vec![a.trim(), b.trim()];
    }
    vec![text.trim()]
}

fn parse_reanimate(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "return ")?.strip_suffix('.')?;
    // Self-reanimation: "Return ~/it from your graveyard to the
    // battlefield." — the activated/triggered effect body naming its own
    // source permanent ([CR#113.7]).
    if matches!(
        body,
        "~ from your graveyard to the battlefield" | "it from your graveyard to the battlefield"
    ) {
        let functional_zone = (body == "~ from your graveyard to the battlefield")
            .then_some(FunctionalZone::Graveyard);
        return Some(ParsedEffect {
            functional_zone,
            targets: Vec::new(),
            effect: "Move(This, Battlefield)".to_owned(),
        });
    }
    // Targeted reanimation: "target <subject> card from your graveyard to
    // the battlefield." Mirrors `parse_return_to_hand`'s graveyard-to-hand
    // arm exactly, down to the "<subject> card" noun-phrase split.
    let noun = body
        .strip_suffix(" from your graveyard to the battlefield")
        .and_then(|s| s.strip_prefix("target "))?;
    let subject = noun
        .strip_suffix(" card")
        .or_else(|| (noun == "card").then_some(""))?;
    let card_filter = graveyard_card_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({card_filter})")],
        effect: "Move(Target(0), Battlefield)".to_owned(),
    })
}

/// Bounce-to-library productions (the library twin of
/// [`parse_return_to_hand`]) — every arm emits the `Move(_, Library(anchor))`
/// primitive with a MANDATORY anchor (a bare `Library` destination is
/// rejected by the grammar; see [`deckmaste_core::Destination`]):
/// - `Put ~ on top of its owner's library.` / `Put ~ on the bottom of its
///   owner's library.` -> a self-bounce (`Move(This, Library(FromTop(0)))` /
///   `Move(This, Library(FromBottom(0)))`), no target — the effect body of
///   `{cost}: Put ~ on top of its owner's library.` activated abilities (the
///   cost is the activated-frame's job).
/// - `Put ~ on top of your library.` / `Put ~ on the bottom of your library.`
///   -> the same self-bounce; "your library" is the equally-correct common
///   idiom (`Move(_, Library(anchor))` always lands in the owner's library
///   regardless of which possessive the oracle text prints, mirroring
///   [`parse_return_to_hand`]'s "your hand"/"its owner's hand" equivalence).
/// - `Put target <subject> on top of its owner's library.` / `...on the bottom
///   of its owner's library.` / `...on top of your library.` / `...on the
///   bottom of your library.` -> `Move(It, Library(anchor))`, the subject
///   parsed by [`object_target_filter`].
///
/// DEFERRED (not built here): `Return ~/target <subject> to its owner's
/// library.` with NO top/bottom qualifier is a shuffle-in move (a different,
/// Shuffle(LibraryOf(You))-bearing shape — the library position isn't fixed, so
/// it needs a shuffle) — out of scope. Parameterized/non-zero anchors ("the
/// second from the top", "Nth from the top", "X cards from the top") and
/// owner-vs-controller disambiguation are likewise unbuilt.
fn parse_bounce_to_library(line: &str) -> Option<ParsedEffect> {
    let body = strip_prefix_ci(line, "put ")?.strip_suffix('.')?;
    // Self-bounce: "Put ~/it on top/the bottom of its owner's/your library."
    // — an activated-ability effect ("~") or a trigger body whose "it"
    // anaphor names the resolving source ([CR#113.7]); every phrasing is the
    // source permanent (`This`).
    if matches!(
        body,
        "~ on top of its owner's library"
            | "it on top of its owner's library"
            | "~ on top of your library"
            | "it on top of your library"
    ) {
        return Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: "Move(This, Library(FromTop(0)))".to_owned(),
        });
    }
    if matches!(
        body,
        "~ on the bottom of its owner's library"
            | "it on the bottom of its owner's library"
            | "~ on the bottom of your library"
            | "it on the bottom of your library"
    ) {
        return Some(ParsedEffect {
            functional_zone: None,
            targets: Vec::new(),
            effect: "Move(This, Library(FromBottom(0)))".to_owned(),
        });
    }
    // Targeted: "target <subject> on top/the bottom of its owner's/your
    // library." The anchor suffix is peeled first (top before bottom, since
    // neither is a suffix of the other), then the leading "target ".
    let (rest, anchor) = if let Some(r) = body
        .strip_suffix(" on top of its owner's library")
        .or_else(|| body.strip_suffix(" on top of your library"))
    {
        (r, "FromTop(0)")
    } else {
        let r = body
            .strip_suffix(" on the bottom of its owner's library")
            .or_else(|| body.strip_suffix(" on the bottom of your library"))?;
        (r, "FromBottom(0)")
    };
    let subject = rest.strip_prefix("target ")?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: format!("Move(Target(0), Library({anchor}))"),
    })
}

/// `Tap target <subject>.` / `Untap target <subject>.` -> the
/// [`Tap`](deckmaste_core::Action::Tap) /
/// [`Untap`](deckmaste_core::Action::Untap) verbs ([CR#701.26a..701.26b])
/// over a single target. The subject is parsed by [`object_target_filter`].
/// Riders ("It doesn't untap …", "It gets …") leave trailing text past the
/// period-terminated single sentence, so they decline cleanly here (each is a
/// later multi-clause production). Case-insensitive lead.
fn parse_tap_untap(line: &str) -> Option<ParsedEffect> {
    let (verb, rest) = if let Some(rest) = strip_prefix_ci(line, "tap target ") {
        ("Tap", rest)
    } else {
        let rest = strip_prefix_ci(line, "untap target ")?;
        ("Untap", rest)
    };
    let subject = rest.strip_suffix('.')?;
    let filter = object_target_filter(subject)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: vec![format!("TargetOne({filter})")],
        effect: format!("{verb}(Target(0))"),
    })
}

/// An object-target subject phrase -> its `Predicate` RON. First the shared
/// [`filter`] phrase grammar (single head noun with adjectives), then a
/// type-noun disjunction fallback for "<type> or <type>[ or <type>]" subjects
/// (`Or([…])`) the single-head grammar can't carry — "artifact or
/// enchantment", "creature or planeswalker", "attacking or blocking creature".
fn object_target_filter(subject: &str) -> Option<String> {
    if let Some(f) = filter::parse_phrase(subject) {
        return Some(f);
    }
    type_disjunction(subject)
}

/// "<A> or <B>[ or <C>]" of type-noun (or status-qualified) members ->
/// `Or([…])`. Two shapes:
/// - a shared head noun with disjoined status adjectives: "attacking or
///   blocking creature" -> `And([Creature, Or([Attacking, Blocking])])` — the
///   head noun trails the last member; the leading members are bare combat-
///   status adjectives. Tried first, since a status adjective ("attacking")
///   would otherwise be misread as a bare-subtype head.
/// - heterogeneous types: "artifact or enchantment" -> `Or([Type(Artifact),
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
            return Some(format!("And([{head_filter}, Or([{}])])", status.join(", ")));
        }
    }
    // Heterogeneous type-noun disjunction: every member is a plain type noun.
    let members: Option<Vec<String>> = parts.iter().map(|p| type_noun_phrase(p)).collect();
    Some(format!("Or([{}])", members?.join(", ")))
}

/// A bare type-noun phrase (a determiner-led single card type / `permanent`) ->
/// its head `Predicate`, declining a bare-subtype fallthrough. Guards
/// [`type_disjunction`] so a disjunction member is a real type noun, never a
/// silently-minted `Subtype`. Strips a leading determiner ("a"/"an") the way
/// the shared phrase grammar does.
fn type_noun_phrase(phrase: &str) -> Option<String> {
    let phrase = strip_prefix_ci(phrase, "a ")
        .or_else(|| strip_prefix_ci(phrase, "an "))
        .unwrap_or(phrase)
        .trim();
    let filter = filter::parse_phrase_detailed(phrase)?;
    // Only a genuine type-noun head disjoins here. The filter parser retains
    // that identity explicitly, so this gate never re-parses rendered RON.
    (filter.head() == filter::FilterHead::TypeNoun).then(|| filter.into_predicate())
}

/// A combat-status adjective -> its `Predicate` status atom. The disjoinable
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
/// your graveyard") -> the `Predicate` for a card you own in your graveyard:
/// `And([<type>, InZone(Graveyard), Owner(Ref(You))])`. The type is the
/// card-type spelling (`Type(Creature)`, `Or([Type(Instant),
/// Type(Sorcery)])` for "instant or sorcery") — NOT the battlefield-scoped
/// macros, since a graveyard card is not a permanent. A bare "card" (no type)
/// is any card you own there.
pub(super) fn graveyard_card_filter(subject: &str) -> Option<String> {
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
    Some(format!("And([{}])", atoms.join(", ")))
}

/// The any-graveyard twin of [`graveyard_card_filter`]: a card of the given
/// type in A graveyard — any player's, not just yours — omitting the
/// `Owner(Ref(You))` restriction the your-graveyard sibling carries:
/// `And([<type>, InZone(Graveyard)])`. Used by "Exile target [<type>] card
/// from a graveyard." ([CR#701.13a]/[CR#400.7], the graveyard-hate family),
/// whose "a graveyard" (contrast the recursion family's "your graveyard")
/// names no owner. Reuses [`graveyard_card_type`] for the type atom, so
/// "instant or sorcery" disjunctions parse identically. A bare "card" (no
/// type qualifier) is the lone `InZone(Graveyard)` atom, unwrapped — the
/// corpus convention of never nesting a singleton filter in an `And` of one
/// (see [`graveyard_card_filter`]'s doc for the your-graveyard analog, which
/// always carries at least the `Owner` atom alongside).
pub(super) fn any_graveyard_card_filter(subject: &str) -> Option<String> {
    match graveyard_card_type(subject) {
        Some(ty) => Some(format!("And([{ty}, InZone(Graveyard)])")),
        None if subject.is_empty() => Some("InZone(Graveyard)".to_owned()),
        // A type word the card-type grammar doesn't model -> decline (never
        // emit a junk filter).
        None => None,
    }
}

/// A graveyard-card type phrase -> its card-type `Predicate` (`Type(Creature)`,
/// `Or([Type(Instant), Type(Sorcery)])`), or `None` for a bare "card" (no
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
        format!("Or([{}])", types.join(", "))
    })
}

/// `~ deals N damage to <target>.` or `it deals N damage to <target>.` —
/// "it" case-insensitively, since it opens the clause after a cost colon
/// ("Sacrifice ~: It deals …") but follows a comma in trigger clauses.
///
/// LANDMINE (source anaphor): this resolves the "it deals" subject to `This`
/// (the carrier) unconditionally. That is correct for a leading/standalone
/// clause, but "It deals …" as a NON-leading sentence can be a same-effect
/// anaphor to a previously-declared target ("Target creature you control gets
/// +1/+0. It deals damage equal to its power to …" — Ambuscade/Clear Shot),
/// where "it" is the pumped creature, not the carrier. This does not misfire
/// today only because `parse_sequence` refuses to combine when a non-first
/// sentence declares its own target, and the remaining anaphor cards have a
/// non-parsing first sentence. If either gap closes, this fn (esp. the
/// `StatOf(This, Power)` bite branch, which doubles down on `This` for both
/// source AND amount) will silently emit a wrong `This`-sourced effect —
/// add a position-aware guard (decline an anaphoric non-leading "it deals")
/// before extending it, mirroring the `Ref(It)` idiom in
/// `combat_restriction_scope`.
fn parse_deal_damage(line: &str, slot: usize) -> Option<ParsedEffect> {
    let body = line
        .strip_prefix("~ deals ")
        .or_else(|| strip_prefix_ci(line, "it deals "))?
        .strip_suffix('.')?;
    // The one-sided "bite" shape ([CR#120] variable damage amount; `Fight`'s
    // [CR#701.14a] two-sided shape minus the reciprocal half): "deals damage
    // equal to its/~'s power to <target>" -> `StatOf(This, Power)` ("its"/
    // "~'s" both name the SAME self-reference the "~ deals"/"it deals" prefix
    // already resolved to `This`). Corpus-verified word order: unlike a
    // literal numeral, which precedes "damage" ("deals 3 damage to X"), the
    // variable-amount clause here sits BETWEEN "damage" and "to <target>"
    // ("deals damage equal to its power to X") — so it can't share the
    // trailing-clause `count::strip` peel below (that peels a clause AFTER
    // the target, e.g. "damage to X equal to the number of Y"). Checked
    // first, before the dynamic-count split.
    if let Some(tail) = body
        .strip_prefix("damage equal to its power to ")
        .or_else(|| body.strip_prefix("damage equal to ~'s power to "))
    {
        let patient = damage_target(tail, slot)?;
        let (targets, selection) = match patient {
            DamagePatient::Reference { targets, reference } => (targets, reference),
            DamagePatient::EachOf { filter } => (Vec::new(), format!("SelectAll({filter})")),
        };
        return Some(ParsedEffect {
            functional_zone: None,
            targets,
            effect: format!("DealDamage(This, StatOf(This, Power), {selection})"),
        });
    }
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
                None if amt == "that much" => "ThatMuch".to_owned(),
                None if amt == "X" => "X".to_owned(),
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
    let patient = damage_target(tail, slot)?;
    // Bare X is currently grounded only for mass-damage selections (the
    // Hurricane family). Other X-damage frames need their surrounding spell
    // cost threaded into this parser before they can safely opt in.
    if amount == "X" && !matches!(patient, DamagePatient::EachOf { .. }) {
        return None;
    }
    // A verb takes a single `Reference`; a "to each / to all" shape's patient is
    // a `SelectAll(...)` SELECTION, which can't ride the verb directly — that
    // shape is the `DealsDamageToEach` macro
    // (`plugins/builtin/macros/effect/DealsDamageToEach.ron`): `Each` over the
    // many-`Binder` `Existing(SelectAll(<filter>))`, binding each member in
    // turn as the iteration anaphor `It` per element ([CR#608.2d] to-each),
    // with `DealDamage` taking that anaphor. Emitting the macro invocation
    // (rather than inlining the `Each`/`SelectAll` shell here) is what lets
    // this shape render back through its own template
    // ([`crate::parsers::filter`]'s recipient filter is the invocation's
    // second argument) — the same "production emits the macro" pattern the
    // trigger families use for their `EventFilter` macros. A targeted
    // shape's patient is a `Reference` (`It`) and rides the verb unchanged,
    // no macro needed.
    let (targets, effect) = match patient {
        DamagePatient::EachOf { filter } => {
            (Vec::new(), format!("DealsDamageToEach({amount}, {filter})"))
        }
        DamagePatient::Reference { targets, reference } => {
            (targets, format!("DealDamage(This, {amount}, {reference})"))
        }
    };
    Some(ParsedEffect {
        functional_zone: None,
        targets,
        effect,
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
        functional_zone: None,
        targets: Vec::new(),
        effect: format!("Draw({n})"),
    })
}

/// `Discard N card(s)[ at random].` — no targets. Case-insensitive lead
/// ("discard" or "Discard"). The common form ([CR#701.9b]): no `what`, so
/// the discarding player (`You`, implicit) chooses `count` cards from hand
/// — the `Discard(N)` composite macro. The "at random" rider routes to the
/// `DiscardAtRandom(N)` twin (an `Existing(Random(..))` binder in place of
/// `Choose` — no decision surfaces, the engine samples uniformly). The
/// imperative effect-body sibling of [`parse_draw`]; the cost-side
/// "Discard a card" (`DiscardCards(N)`) is [`crate::parsers::cost::discard`]
/// — same composite, different frame.
fn parse_discard(line: &str) -> Option<ParsedEffect> {
    let rest = strip_prefix_ci(line, "discard ")?.strip_suffix('.')?;
    let (rest, random) = match rest.strip_suffix(" at random") {
        Some(r) => (r, true),
        None => (rest, false),
    };
    // Plural first so "two cards" doesn't strip to "two card".
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = number_word(count)?;
    let effect = if random { format!("DiscardAtRandom({n})") } else { format!("Discard({n})") };
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect,
    })
}

/// The loot/rummage idiom ([CR#121.1,701.9]): "Draw N card(s), then discard
/// M card(s)[ at random]." (loot) or the reverse "Discard M card(s)[ at
/// random], then draw N card(s)." (rummage) — ONE oracle sentence whose
/// internal ", then " joins two imperative clauses, N and M independently
/// scaled. The general [`parse_sequence`] machinery doesn't cover this: it
/// only splits ". "-separated SENTENCES ([CR#608.2c]), and this pair rides a
/// single sentence's comma-"then". Deliberately narrow — NOT a general
/// comma-then splitter (the corpus has dozens of unrelated ", then "
/// multi-clause lines this production must not touch) — by requiring both
/// halves to be exactly one [`parse_draw`] and one [`parse_discard`] leaf, in
/// either order. Declines otherwise (a conditional tail, "discard your
/// hand", or anything richer stays with the macro fallthrough / unparsed).
/// No targets (neither leaf declares one).
fn parse_draw_then_discard(line: &str) -> Option<ParsedEffect> {
    let (head, tail) = line.split_once(", then ")?;
    let head = format!("{head}.");
    let (first, second) = match (parse_draw(&head), parse_discard(&head)) {
        (Some(draw), None) => (draw, parse_discard(tail)?),
        (None, Some(discard)) => (discard, parse_draw(tail)?),
        _ => return None,
    };
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!("Sequentially([{}, {}])", first.effect, second.effect),
    })
}

/// `You lose N life[ for each <filter>].` — the ability's controller loses
/// life. No targets.
fn parse_lose_life(line: &str) -> Option<ParsedEffect> {
    let amount = life_amount(strip_prefix_ci(line, "you lose ")?)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!("ChangeLife(You, Down({amount}))"),
    })
}

/// `You gain N life[ for each <filter>].` — the ability's controller gains
/// life. No targets.
fn parse_gain_life(line: &str) -> Option<ParsedEffect> {
    let amount = life_amount(strip_prefix_ci(line, "you gain ")?)?;
    Some(ParsedEffect {
        functional_zone: None,
        targets: Vec::new(),
        effect: format!("ChangeLife(You, Up({amount}))"),
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
/// — the rules do — so it emits `Create(agent: You, count: <count>, token:
/// Named(<Name>))`, the bare-ident `TokenSpec::Named` position. Fixed counts
/// only (the bare numeral = reader sugar for `Count::Literal`, like the sibling
/// creature-token / `Draw` productions). A `tapped` modifier, a dynamic count
/// (`X`, "that many", "a number of …"), an unbuilt predefined token
/// (Powerstone, Map, …), or any trailing clause declines — those are richer
/// than this v1 production.
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
        functional_zone: None,
        targets: Vec::new(),
        effect: format!("Create(agent: You, count: {count}, token: Named({name}))"),
    })
}

/// `You get an emblem with "<ability>".` -> `GetEmblem(You, [<ability RON>])`.
/// The player gets an emblem whose only characteristics are the quoted
/// abilities ([CR#114.1,114.3]); the quoted text is ONE full ability line, so
/// it re-resolves through the whole frame REGISTRY (an emblem can carry any
/// frame — static, triggered, activated), mirroring the render direction
/// (`GetEmblem` renders its abilities through the shared `rules` walk and
/// re-quotes them). Out of scope, declining: multi-ability emblems (`"…" and
/// "…"`) and nested-quote conferrals (any interior `"`), and a "this emblem"
/// self-reference (extraction only normalizes the CARD's name to `~`, so the
/// inner line keeps the phrase and its frame parse declines on the unknown
/// subject).
fn parse_get_emblem(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<ParsedEffect>> {
    // The printed convention ends the sentence with the quoted ability's own
    // period — no period outside the closing quote.
    let Some(inner) = line
        .strip_prefix("You get an emblem with \"")
        .and_then(|s| s.strip_suffix('"'))
    else {
        return Ok(None);
    };
    // One plainly-quoted ability only: an interior quote is a multi-ability
    // join or a nested conferral, both out of scope.
    if inner.contains('"') {
        return Ok(None);
    }
    for parser in crate::resolve::REGISTRY {
        if let Some(ability) = parser(inner, ctx)? {
            return Ok(Some(ParsedEffect {
                functional_zone: None,
                targets: Vec::new(),
                effect: format!("GetEmblem(You, [{ability}])"),
            }));
        }
    }
    Ok(None)
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
        functional_zone: None,
        targets: Vec::new(),
        effect: format!(
            "Create(agent: You, count: {count}, token: Token({}))",
            fields.join(", ")
        ),
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
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        "thirteen" => Some(13),
        "fourteen" => Some(14),
        "fifteen" => Some(15),
        "sixteen" => Some(16),
        "seventeen" => Some(17),
        "eighteen" => Some(18),
        "nineteen" => Some(19),
        "twenty" => Some(20),
        digits => digits.parse().ok(),
    }
}

/// The two structurally distinct patients a damage clause can name.
enum DamagePatient {
    /// One object/player reference, with any target declarations it requires.
    Reference {
        targets: Vec<String>,
        reference: String,
    },
    /// A distributive set represented by the filter inside `SelectAll`.
    EachOf { filter: String },
}

/// Maps the "to <X>" tail of a damage clause to its typed patient. Targeted
/// shapes declare a `TargetSpec` and the body reads
/// it back POSITIONALLY as `Target(slot)` ([CR#115.3,601.2c]) — `slot` is the
/// announce-list index this declaration will occupy, threaded in by the caller
/// (0 for a lone damage clause; the two-patient shape
/// [`parse_damage_and_damage`] gives its second patient index 1). "each" shapes
/// declare nothing and retain the filter to which damage distributes.
fn damage_target(text: &str, slot: usize) -> Option<DamagePatient> {
    Some(match text {
        "any target" => DamagePatient::Reference {
            targets: vec!["AnyTarget".to_owned()],
            reference: format!("Target({slot})"),
        },
        "you" => DamagePatient::Reference {
            targets: Vec::new(),
            reference: "You".to_owned(),
        },
        "target player" => DamagePatient::Reference {
            targets: vec!["TargetOne(Player)".to_owned()],
            reference: format!("Target({slot})"),
        },
        // "target opponent" — a single opponent of you ([CR#102.2]).
        "target opponent" => DamagePatient::Reference {
            targets: vec!["TargetOne(OpponentOf(Ref(You)))".to_owned()],
            reference: format!("Target({slot})"),
        },
        // The restricted "any target" minus its object members ([CR#115.4]):
        // a player or planeswalker, never a creature/battle (Lava Spike).
        "target player or planeswalker" => DamagePatient::Reference {
            targets: vec!["TargetOne(Or([Player, Planeswalker]))".to_owned()],
            reference: format!("Target({slot})"),
        },
        "each creature" => DamagePatient::EachOf {
            filter: "Creature".to_owned(),
        },
        "each player" => DamagePatient::EachOf {
            filter: "Player".to_owned(),
        },
        // "each opponent" — the players who are opponents of you ([CR#102.2]).
        "each opponent" => DamagePatient::EachOf {
            filter: "OpponentOf(Ref(You))".to_owned(),
        },
        // "each creature and each player" — every member of the combined set
        // ([CR#608.2d] distributive each). The two "each" groups union into one
        // `SelectAll(Or([…]))` selection (Pestilence / Earthquake-style sweeps).
        "each creature and each player" => DamagePatient::EachOf {
            filter: "Or([Creature, Player])".to_owned(),
        },
        // A qualified creature class unioned with all players (Hurricane):
        // preserve the creature qualifier on only that arm.
        _ if text.starts_with("each ") && text.ends_with(" and each player") => {
            let subject = text
                .strip_prefix("each ")?
                .strip_suffix(" and each player")?;
            let filter = object_target_filter(subject)?;
            DamagePatient::EachOf {
                filter: format!("Or([{filter}, Player])"),
            }
        }
        // A "each <subject>" mass-burn recipient class beyond the bare-noun
        // shapes above (the damage-sweeper family, ~186 corpus lines):
        // "each creature your opponents control" ([CR#608.2d] distributive
        // each over a characteristic-filtered set; [CR#102.2] opponent),
        // reusing the shared object-target grammar so any filter it already
        // models (a controller postfix, a color/subtype adjective, a type
        // disjunction, …) sweeps too.
        _ if text.starts_with("each ") => {
            let subject = text.strip_prefix("each ")?;
            let filter = object_target_filter(subject)?;
            DamagePatient::EachOf { filter }
        }
        // A "target <subject>" object target whose subject parses through the
        // shared object-target grammar (single head noun, or a "<type> or
        // <type>" / "attacking or blocking creature" disjunction).
        _ => {
            let subject = text.strip_prefix("target ")?;
            let filter = object_target_filter(subject)?;
            DamagePatient::Reference {
                targets: vec![format!("TargetOne({filter})")],
                reference: format!("Target({slot})"),
            }
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
        parse_clause(line, &ctx)
            .unwrap()
            .map(|p| (p.targets.join(", "), p.effect))
    }

    /// `(targets, effect)` resolved against the REAL builtin macro index, so
    /// the `OneShotEffect`-kind macro-template fallthrough is exercised.
    fn parsed_with_macros(line: &str) -> Option<(String, String)> {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        parse_clause(line, &ctx)
            .unwrap()
            .map(|p| (p.targets.join(", "), p.effect))
    }

    /// `you may pay {cost}. If you do, <effect>` folds the offer and its "if
    /// you do" continuation into one [`MayPay`] node — the dominant reflexive
    /// form. `actor` defaults to `You` (omitted); no `or_else` (positive form).
    #[test]
    fn may_pay_reflexive_folds_to_may_pay() {
        assert_eq!(
            parsed_with_macros("you may pay {2}. If you do, draw a card."),
            Some((
                String::new(),
                "MayPay(cost: [Mana([Generic(2)])], and_then: Draw(1))".to_owned(),
            ))
        );
    }

    /// A non-`pay` offer folds to [`May`] with the verb phrase and the branch
    /// each re-parsed by `parse_clause`; `if_did` carries the "if you do" tail.
    #[test]
    fn may_verb_reflexive_folds_to_may_if_did() {
        assert_eq!(
            parsed_with_macros("you may draw a card. If you do, discard a card."),
            Some((
                String::new(),
                "May(who: You, effect: Draw(1), if_did: Discard(1))".to_owned(),
            ))
        );
    }

    /// The negative-only `If you don't, …` complement populates `if_not` with
    /// no `if_did` (the milled-Plains shape family).
    #[test]
    fn may_verb_reflexive_negative_only_tail() {
        assert_eq!(
            parsed_with_macros("you may discard a card. If you don't, put a +1/+1 counter on ~."),
            Some((
                String::new(),
                "May(who: You, effect: Discard(1), if_not: PutCounters(This, P1P1Counter, 1))"
                    .to_owned(),
            ))
        );
    }

    /// Both branches present -> both `if_did` and `if_not`, in field order.
    #[test]
    fn may_verb_reflexive_both_branches() {
        assert_eq!(
            parsed_with_macros(
                "you may draw a card. If you do, discard a card. If you don't, put a +1/+1 counter on ~."
            ),
            Some((
                String::new(),
                "May(who: You, effect: Draw(1), if_did: Discard(1), if_not: PutCounters(This, P1P1Counter, 1))"
                    .to_owned(),
            ))
        );
    }

    /// A reflexive fold buried mid-sequence: `parse_sequence` coalesces the
    /// `you may …`/`If you don't, …` run so it reaches the reflexive parser
    /// whole, while the leading sentence stays a `Sequentially` sibling.
    #[test]
    fn may_reflexive_mid_sequence_coalesces() {
        assert_eq!(
            parsed_with_macros(
                "put a +1/+1 counter on ~. You may draw a card. If you don't, draw two cards."
            ),
            Some((
                String::new(),
                "Sequentially([PutCounters(This, P1P1Counter, 1), \
                 May(who: You, effect: Draw(1), if_not: Draw(2))])"
                    .to_owned(),
            ))
        );
    }

    /// The "if you do" branch consumes GREEDILY to the line end, folding a
    /// trailing sentence into `and_then` (a nested `Sequentially`) rather than
    /// leaking it as an unconditional sibling.
    #[test]
    fn may_pay_reflexive_branch_is_greedy() {
        assert_eq!(
            parsed_with_macros("you may pay {2}. If you do, draw a card. Draw two cards."),
            Some((
                String::new(),
                "MayPay(cost: [Mana([Generic(2)])], \
                 and_then: Sequentially([Draw(1), Draw(2)]))"
                    .to_owned(),
            ))
        );
    }

    /// v1 declines the cases whose render cannot round-trip the oracle: an
    /// energy `pay {E}{E}` (cost renders WITH a "Pay" word, doubling in the
    /// "may pay" frame) and a pay offer carrying a negative `If you don't`
    /// branch (`MayPay` renders that branch with a "; if you don't" semicolon).
    #[test]
    fn may_pay_reflexive_declines_unroundtrippable() {
        assert!(parsed_with_macros("you may pay {E}{E}. If you do, draw a card.").is_none());
        assert!(
            parsed_with_macros(
                "you may pay {2}. If you do, draw a card. If you don't, lose 1 life."
            )
            .is_none()
        );
    }

    /// A bare `you may <x>` with no continuation is left to [`parse_may`] — the
    /// reflexive parser engages only on a `. If you do,`/`. If you don't,`
    /// boundary.
    #[test]
    fn may_reflexive_ignores_bare_may() {
        assert_eq!(
            parsed_with_macros("you may draw a card."),
            Some((String::new(), "May(who: You, effect: Draw(1))".to_owned()))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn classic_manland_animation() {
        assert_eq!(
            parsed("~ becomes a 2/1 red Warrior creature with first strike until end of turn. It's still a land."),
            Some((
                String::new(),
                "Until(FixedUntil(EndOfTurn), [Modify(This, Several([CardTypes(Add(\"Creature\")), Subtypes(Add(Warrior)), Colors(Set([Red])), Power(Set(2)), Toughness(Set(1)), GainAbility(Keyword(FirstStrike))]))])".to_owned(),
            ))
        );
    }

    /// `You get an emblem with "<ability>".` re-resolves the quoted ability
    /// through the whole frame registry and wraps it in `GetEmblem(You, […])`
    /// ([CR#114.1,114.3]) — here the anthem static, via the static-ability
    /// frame parser.
    #[test]
    fn get_emblem_wraps_registry_ability() {
        assert_eq!(
            parsed(r#"You get an emblem with "Creatures you control get +1/+1.""#),
            Some((
                String::new(),
                "GetEmblem(You, [Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(1)), Toughness(Up(1))]))))])"
                    .to_owned(),
            ))
        );
    }

    #[test]
    fn get_emblem_declines_out_of_scope_shapes() {
        // Two quoted abilities joined by ` and ` — the interior quote gate.
        assert!(
            parsed(
                r#"You get an emblem with "You have no maximum hand size" and "Whenever a card is put into your graveyard from anywhere, you may return it to your hand.""#
            )
            .is_none()
        );
        // A "this emblem" self-reference is an unknown subject to the inner
        // frame parse (only the card's own name normalizes to `~`).
        assert!(
            parsed(
                r#"You get an emblem with "Whenever you cast a spell, this emblem deals 5 damage to any target.""#
            )
            .is_none()
        );
        // An unstructurable inner ability declines the whole line.
        assert!(parsed(r#"You get an emblem with "Gibberish happens.""#).is_none());
    }

    #[test]
    fn rhystic_damage_uses_target_sensitive_payer() {
        assert_eq!(
            parsed("~ deals 4 damage to any target unless that permanent's controller or that player pays {2}. If they do, ~ deals 2 damage to the permanent or player."),
            Some((
                "AnyTarget".to_owned(),
                "MayPay(actor: Coalesce([ControllerOf(Target(0)), Target(0)]), cost: [Mana([Generic(2)])], and_then: DealDamage(This, 2, Target(0)), or_else: DealDamage(This, 4, Target(0)))".to_owned(),
            ))
        );
    }

    /// Whether the clause declines under the EMPTY index (pins a bespoke
    /// production's non-match without the macro fallthrough shadowing it).
    fn declines(line: &str) -> bool {
        let ctx = crate::parsers::test_ctx::ctx(CardKind::Permanent);
        parse_clause(line, &ctx).unwrap().is_none()
    }

    /// The declarative-subject production (ONE arm for the whole player-verb
    /// family): subject phrase → agent; verb phrase → the `PlayerAction`-kind
    /// macro whose template matches ([CR#701.17a,701.9b,121.1,119.3]). No
    /// per-verb parser arms — a new verb is a new macro template only.
    #[test]
    fn declarative_subject_player_verbs() {
        assert_eq!(
            parsed_with_macros("Target player mills two cards."),
            Some(("TargetOne(Player)".to_owned(), "Mills(It, 2)".to_owned()))
        );
        assert_eq!(
            parsed_with_macros("Each opponent mills a card."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(OpponentOf(Ref(You)))), effect: Mills(It, 1))"
                    .to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("Each player discards two cards."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(Player)), effect: Discards(It, 2))".to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("Target opponent loses 2 life."),
            Some((
                "TargetOne(OpponentOf(Ref(You)))".to_owned(),
                "LosesLife(It, 2)".to_owned()
            ))
        );
        assert_eq!(
            parsed_with_macros("Target player draws three cards."),
            Some(("TargetOne(Player)".to_owned(), "Draws(It, 3)".to_owned()))
        );
        // The count reader tops out at spelled cardinals + decimals; a
        // trailing rider ("at random") leaves the phrase unconsumed.
        assert!(
            parsed_with_macros("Target player mills half their library, rounded down.").is_none()
        );
        assert_eq!(
            parsed_with_macros("Each player discards a card at random."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(Player)), effect: DiscardsAtRandom(It, 1))"
                    .to_owned()
            )),
            "the at-random declarative parses through the DiscardsAtRandom macro"
        );
    }

    /// The bounded `Count` slot reader for `"gets +${0}/+${1}"`-shaped
    /// templates: unlike [`player_verb_slot_reader`] (bounds on whitespace),
    /// this one bounds on the first non-digit so slot 0 stops at the `/`
    /// separator rather than swallowing the whole `"2/+2"` tail.
    #[test]
    fn count_reader_bounds_at_slash() {
        assert_eq!(
            count_delim_slot_reader("Count", "2/+2"),
            Some(("2".to_string(), 1))
        );
        assert_eq!(
            count_delim_slot_reader("Count", "2"),
            Some(("2".to_string(), 1))
        );
        assert_eq!(count_delim_slot_reader("Reference", "2/+2"), None); // wrong type declines
    }

    /// End-to-end fold: `"gets +2/+2"` against the real `Modification`
    /// templates, reading both `Count` slots via the bounded reader — proves
    /// slot 0 stops at `/` instead of consuming past it.
    #[test]
    fn modification_pump_folds_via_bounded_reader() {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        let m = ctx
            .index
            .match_with("Modification", "gets +2/+2", count_delim_slot_reader)
            .unwrap()
            .expect("folds");
        assert_eq!(m.invocation, "PowerAndToughnessUp(2, 2)");
        assert_eq!(m.consumed, "gets +2/+2".len());
    }

    /// End-to-end through `parse_clause` with the builtin index: a plain
    /// (unscaled, no-tail) durational pump RETAINS the change macro — the
    /// surface-retention payoff, mirroring the static anthem's `parse_pt` fold.
    #[test]
    fn durational_pump_folds_change_macro() {
        assert_eq!(
            parsed_with_macros("Target creature gets +3/+3 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(Target(0), PowerAndToughnessUp(3, 3)), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // The debuff twin folds to `PowerAndToughnessDown`.
        assert_eq!(
            parsed_with_macros("Target creature gets -2/-2 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(Target(0), PowerAndToughnessDown(2, 2)), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
    }

    /// A "for each" scaler now folds under the builtin index too: the
    /// `P1P1ForEach` macro's template spells "for each ${0}" literally, so
    /// the WHOLE "gets +1/+1 for each <selection>" phrase matches it via the
    /// `Predicate` slot reader — no more raw `Several(...)`.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_scaled_folds_to_for_each_macro() {
        assert_eq!(
            parsed_with_macros(
                "Creatures you control get +1/+1 for each Goblin you control until end of turn."
            ),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, P1P1ForEach(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))])))), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    /// A keyword grant tail declines the change fold EVEN under the builtin
    /// index: `"gets +3/+3 and gain trample"` leaves the `"gets …"` phrase only
    /// partially matched, failing the full-consumption gate, so the whole
    /// change (P/T pair + `GainAbility`) stays inline.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_grant_tail_declines_change_fold() {
        assert_eq!(
            parsed_with_macros(
                "Creatures you control get +3/+3 and gain trample until end of turn."
            ),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(3)), Toughness(Up(3)), GainAbility(Keyword(Trample))]))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
    }

    /// The emitted invocations READ back through the builtin macros: the
    /// declarative-subject keyword-action macros expand to their core
    /// `Composite` (`Mills(It, 2)` → `Composite(Mill(It, 2), …)`), remembered
    /// with their template so the render side prints the verb phrase back — the
    /// parse-via-macros round trip at the read boundary. (`Mills`/`Draws` are
    /// now `OneShotEffect`-kind `Composite` macros, no longer `PlayerAction`s
    /// under `By`.)
    #[test]
    fn declarative_subject_emissions_read_back() {
        use std::path::Path;
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = deckmaste_cards::plugin::Plugin::load(plugins.join("builtin")).unwrap();
        let effect: deckmaste_core::OneShotEffect = plugin
            .macros
            .read_str("Each(binder: Existing(SelectAll(Player)), effect: Mills(It, 2))")
            .unwrap();
        let deckmaste_core::OneShotEffect::Each(each) = effect else {
            panic!("expected Each, got {effect:?}");
        };
        let deckmaste_core::OneShotEffect::Expanded(exp) = &*each.effect else {
            panic!(
                "expected a remembered Mills expansion, got {:?}",
                each.effect
            );
        };
        assert_eq!(exp.name.as_str(), "Mills");
        // `Mills(It, 2)` is the slice-family `Batch(2, Act(Composite(name:
        // Mill, …)))`; the performer (`It`) rides the body's `TopOfLibrary`
        // selection, not a typed atom.
        let deckmaste_core::OneShotEffect::Batch(_, inner) = exp.value.as_ref() else {
            panic!("Mills(It, 2) expands to a Batch, got {:?}", exp.value);
        };
        assert!(
            matches!(
                inner.as_ref(),
                deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::Composite { name, .. })
                    if name.as_str() == "Mill"
            ),
            "Mills(It, 2) is a Batch over Composite(name: Mill, …), got {inner:?}"
        );
    }

    #[test]
    fn deal_damage_targeted_shapes() {
        assert_eq!(
            parsed("~ deals 3 damage to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(This, 3, Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 2 damage to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(This, 2, Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 4 damage to target player."),
            Some((
                "TargetOne(Player)".to_owned(),
                "DealDamage(This, 4, Target(0))".to_owned()
            ))
        );
        // Lava Spike's restricted target: player-or-planeswalker (can't hit
        // creatures), a strict subset of "any target".
        assert_eq!(
            parsed("~ deals 3 damage to target player or planeswalker."),
            Some((
                "TargetOne(Or([Player, Planeswalker]))".to_owned(),
                "DealDamage(This, 3, Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn deal_damage_each_shapes() {
        // A "to each" shape emits the `DealsDamageToEach` macro invocation
        // (`plugins/builtin/macros/effect/DealsDamageToEach.ron`) — the
        // `Each`/`SelectAll`/`DealDamage(This, n, It)` shell ([CR#608.2d]
        // distributive each; a verb takes a single `Reference`, never a
        // `Predicate`) is the macro's body, not inlined by the parser.
        assert_eq!(
            parsed("~ deals 2 damage to each creature."),
            Some((String::new(), "DealsDamageToEach(2, Creature)".to_owned()))
        );
        assert_eq!(
            parsed("~ deals 20 damage to each player."),
            Some((String::new(), "DealsDamageToEach(20, Player)".to_owned()))
        );
        // "each opponent" -> the player set "opponents of you".
        assert_eq!(
            parsed("~ deals 1 damage to each opponent."),
            Some((
                String::new(),
                "DealsDamageToEach(1, OpponentOf(Ref(You)))".to_owned()
            ))
        );
    }

    /// The damage-sweeper family's filtered recipient: "each creature your
    /// opponents control" ([CR#608.2d] distributive each; [CR#102.2]
    /// opponent) reuses the shared object-target filter grammar
    /// ([`filter::parse_phrase`]'s controller postfix), sweeping past the
    /// bare-noun shapes `deal_damage_each_shapes` pins.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn deal_damage_each_filtered_recipient() {
        assert_eq!(
            parsed("~ deals 3 damage to each creature your opponents control."),
            Some((
                String::new(),
                "DealsDamageToEach(3, And([Creature, ControlledBy(OpponentOf(Ref(You)))]))"
                    .to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 2 damage to each creature with flying."),
            Some((
                String::new(),
                "DealsDamageToEach(2, And([Creature, Has(Flying)]))".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals 2 damage to each creature without flying."),
            None
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
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
                "TargetOne(And([Permanent, Not(Type(Land))]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // Lowercase lead (the clause after a trigger comma) parses too. The
        // bare-subtype head is battlefield-scoped ([CR#109.2,115.2]).
        assert_eq!(
            parsed("destroy target Goblin."),
            Some((
                "TargetOne(And([Permanent, Subtype(Goblin)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn destroy_target_with_keyword_quality() {
        // "with <keyword>" now resolves to a `Has(<Keyword>)` filter clause, so a
        // keyword-quality target parses (shared filter grammar gain).
        assert_eq!(
            parsed("Destroy target creature with flying."),
            Some((
                "TargetOne(And([Creature, Has(Flying)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_team_like_overrun() {
        // Overrun: a team P/T boost + keyword grant lasting until end of turn.
        assert_eq!(
            parsed("Creatures you control get +3/+3 and gain trample until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(3)), Toughness(Up(3)), GainAbility(Keyword(Trample))]))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_self_and_target() {
        // Self pump ("~ gets …"): bare `This`, no target.
        assert_eq!(
            parsed("~ gets +1/+1 until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(This, Several([Power(Up(1)), Toughness(Up(1))])), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Single-target pump ("target creature gets …"): TargetOne + bare `Target(0)`.
        assert_eq!(
            parsed("Target creature gets +3/+3 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(Target(0), Several([Power(Up(3)), Toughness(Up(3))])), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Keyword-only durational grant on a target.
        assert_eq!(
            parsed("Target creature gains flying until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(Target(0), GainAbility(Keyword(Flying))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
    }

    #[test]
    fn durational_combat_restriction_can_block() {
        // Single-target: "Target creature can't block this turn." -> TargetOne +
        // bare `Target(0)` anchored on `by:`.
        assert_eq!(
            parsed("Target creature can't block this turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Cant(Block(by: Ref(Target(0)))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Self: "~ can't block this turn." -> bare `This`, no target.
        assert_eq!(
            parsed("~ can't block this turn."),
            Some((
                String::new(),
                "Continuously(effect: Cant(Block(by: Ref(This))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // "That creature" anaphor (Chandra, Torch of Defiance's corpus phrasing):
        // the sorted anaphor `That(Creature)`, no target of its own.
        assert_eq!(
            parsed("That creature can't block this turn."),
            Some((
                String::new(),
                "Continuously(effect: Cant(Block(by: Ref(That(Creature)))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // The static (always-on, no "this turn") sibling still declines here —
        // this production requires the durational marker.
        assert!(declines("Enchanted creature can't block."));
    }

    #[test]
    fn durational_combat_restriction_cant_be_blocked() {
        // Single-target: "Target creature can't be blocked this turn." ->
        // TargetOne + bare `Target(0)` anchored on `on:` — the evasion/passive form.
        assert_eq!(
            parsed("Target creature can't be blocked this turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Cant(Block(on: Ref(Target(0)))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Self: "~ can't be blocked this turn." -> bare `This`, no target.
        assert_eq!(
            parsed("~ can't be blocked this turn."),
            Some((
                String::new(),
                "Continuously(effect: Cant(Block(on: Ref(This))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Bare "It" anaphor: a later sentence in a `Sequentially` chain reading a
        // target an earlier sentence already declared — no NEW target here.
        assert_eq!(
            parsed("It can't be blocked this turn."),
            Some((
                String::new(),
                "Continuously(effect: Cant(Block(on: Ref(Target(0)))), \
                 duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // "can't be blocked" is checked BEFORE "can't block" — the active
        // clause's exact suffix never fires on the passive wording.
        assert!(declines(
            "Creatures without flying can't be blocked this turn."
        ));
    }

    #[test]
    fn declines_unknown_damage_targets_and_non_effects() {
        // A damage target the grammar doesn't model still declines. ("each
        // artifact" now resolves via the mass-burn recipient-class fallback
        // — see `deal_damage_each_shapes`/`deal_damage_each_filtered_recipient`
        // — so pick a phrase the shared filter grammar still rejects here.)
        assert!(declines("~ deals 3 damage to each creature wearing hats."));
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
                "DealDamage(This, 1, Target(0))".to_owned()
            ))
        );
        // Activated surface: clause-initial "It deals …" after a cost colon.
        assert_eq!(
            parsed("It deals 2 damage to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(This, 2, Target(0))".to_owned()
            ))
        );
    }

    #[test]
    fn two_damage_instructions_share_the_subject() {
        assert_eq!(
            parsed("~ deals 1 damage to any target and 1 damage to you."),
            Some((
                "AnyTarget".to_owned(),
                "Sequentially([DealDamage(This, 1, Target(0)), DealDamage(This, 1, You)])"
                    .to_owned(),
            ))
        );
    }

    /// TWO announced slots, each read by its own index ([CR#115.3,601.2c]) —
    /// the shape the wildcard anaphor could not express. Both halves used to
    /// emit a bare `It`: the first read was a guess between two same-sort
    /// antecedents and the second slot had no name at all, so the model's
    /// uniqueness gate refused the card. Positional reads name each slot
    /// outright, so nothing here is ambiguous and no labelling apparatus is
    /// needed.
    #[test]
    fn two_single_target_damage_instructions_index_their_own_slots() {
        assert_eq!(
            parsed("~ deals 2 damage to target creature and 1 damage to target creature."),
            Some((
                "TargetOne(Creature), TargetOne(Creature)".to_owned(),
                "Sequentially([DealDamage(This, 2, Target(0)), DealDamage(This, 1, Target(1))])"
                    .to_owned(),
            ))
        );
    }

    #[test]
    fn that_much_damage_to_you() {
        assert_eq!(
            parsed("it deals that much damage to you."),
            Some((String::new(), "DealDamage(This, ThatMuch, You)".to_owned()))
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
                "DealDamage(This, 3, Target(0))".to_owned()
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
            Some((String::new(), "ChangeLife(You, Down(1))".to_owned()))
        );
        assert_eq!(
            parsed("you lose 2 life."),
            Some((String::new(), "ChangeLife(You, Down(2))".to_owned()))
        );
        assert_eq!(
            parsed("You gain 3 life."),
            Some((String::new(), "ChangeLife(You, Up(3))".to_owned()))
        );
        assert_eq!(
            parsed("you gain three life."),
            Some((String::new(), "ChangeLife(You, Up(3))".to_owned()))
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
                "Create(agent: You, count: 3, token: Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Create a 1/1 red Goblin creature token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_predefined_token_singular_and_plural() {
        // [CR#111.10]: a predefined token by bare name -> `Named(<Name>)`.
        assert_eq!(
            parsed("Create a Treasure token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Treasure))".to_owned()
            ))
        );
        assert_eq!(
            parsed("create a Food token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Food))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Create two Treasure tokens."),
            Some((
                String::new(),
                "Create(agent: You, count: 2, token: Named(Treasure))".to_owned()
            ))
        );
        // Gold, Clue, Blood are also built.
        assert_eq!(
            parsed("create a Gold token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Gold))".to_owned()
            ))
        );
        assert_eq!(
            parsed("create a Blood token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Blood))".to_owned()
            ))
        );
        assert_eq!(
            parsed("create a Clue token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Clue))".to_owned()
            ))
        );
        // [CR#111.10w] Vibranium is built (indestructible + restricted {C}).
        assert_eq!(
            parsed("create a Vibranium token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Named(Vibranium))".to_owned()
            ))
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
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn create_token_with_keyword_grants() {
        assert_eq!(
            parsed("create a 1/1 red Goblin creature token with haste."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], abilities: [Keyword(Haste)], power: 1, toughness: 1))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Create a 2/2 white Cat creature token with flying and vigilance."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Token(color_indicator: [White], types: [Creature], subtypes: [Cat], abilities: [Keyword(Flying), Keyword(Vigilance)], power: 2, toughness: 2))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_multicolor_and_multi_subtype() {
        assert_eq!(
            parsed("Create two 1/1 black and green Elf Warrior creature tokens."),
            Some((
                String::new(),
                "Create(agent: You, count: 2, token: Token(color_indicator: [Black, Green], types: [Creature], subtypes: [Elf, Warrior], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_colorless_omits_color() {
        assert_eq!(
            parsed("Create a 1/1 colorless Eldrazi Scion creature token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Token(types: [Creature], subtypes: [Eldrazi, Scion], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    fn create_token_no_subtype_omits_subtypes() {
        assert_eq!(
            parsed("Create a 1/1 red creature token."),
            Some((
                String::new(),
                "Create(agent: You, count: 1, token: Token(color_indicator: [Red], types: [Creature], power: 1, toughness: 1))".to_owned()
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
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn create_token_dynamic_where_x() {
        // Krenko, Mob Boss.
        assert_eq!(
            parsed("Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control."),
            Some((
                String::new(),
                "Create(agent: You, count: CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))), token: \
                 Token(color_indicator: [Red], types: [Creature], subtypes: [Goblin], power: 1, toughness: 1))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn create_token_dynamic_for_each() {
        assert_eq!(
            parsed("Create a 1/1 red Goblin creature token for each Goblin you control."),
            Some((
                String::new(),
                "Create(agent: You, count: CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))), token: \
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
                "Create(agent: You, count: CountOf(Objects(And([Creature, ControlledBy(Ref(You))]))), token: \
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
        // A non-unit base under "for each": the `Count::Times` product form
        // exists (the pump path at `parse_pt_changes_scaled`/`scaled_side`
        // emits it) but isn't wired into this call site's count-word logic
        // -> decline.
        assert!(declines(
            "Create two 1/1 red Goblin creature tokens for each Goblin you control."
        ));
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn deal_damage_dynamic_equal_to() {
        assert_eq!(
            parsed("~ deals damage to any target equal to the number of Goblins you control."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(This, CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))), Target(0))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn deal_damage_dynamic_where_x() {
        assert_eq!(
            parsed("~ deals X damage to target player, where X is the number of Goblins you control."),
            Some((
                "TargetOne(Player)".to_owned(),
                "DealDamage(This, CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))), Target(0))".to_owned()
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
                "DealDamage(This, 3, Target(0))".to_owned()
            ))
        );
    }

    /// The one-sided "bite" shape ([CR#120]; `Fight`'s reciprocal-less half):
    /// "deals damage equal to its power to <target>" -> a `StatOf(This,
    /// Power)` amount. Corpus-verified word order (Cinder Shade, Balduvian
    /// Berserker, …) — the variable-amount clause sits between "damage" and
    /// "to <target>", unlike a literal numeral.
    #[test]
    fn deal_damage_bite_equal_to_its_power() {
        assert_eq!(
            parsed("~ deals damage equal to its power to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(This, StatOf(This, Power), Target(0))".to_owned()
            ))
        );
    }

    /// The "it" subject case-insensitive variant (post cost-colon / trigger
    /// comma), targeting "any target".
    #[test]
    fn deal_damage_bite_it_subject_any_target() {
        assert_eq!(
            parsed("It deals damage equal to its power to any target."),
            Some((
                "AnyTarget".to_owned(),
                "DealDamage(This, StatOf(This, Power), Target(0))".to_owned()
            ))
        );
    }

    /// The rarer "~'s power" phrasing (same self-reference, named rather
    /// than pronominal).
    #[test]
    fn deal_damage_bite_tilde_s_power_variant() {
        assert_eq!(
            parsed("~ deals damage equal to ~'s power to target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "DealDamage(This, StatOf(This, Power), Target(0))".to_owned()
            ))
        );
    }

    /// The wider "creature or planeswalker" recipient (Bite Down, Heartfire
    /// Immolator, …) — the existing `damage_target`/`object_target_filter`
    /// type-disjunction grammar already yields the right filter, so no
    /// target-parser changes were needed for this scope item.
    #[test]
    fn deal_damage_bite_creature_or_planeswalker_target() {
        assert_eq!(
            parsed("~ deals damage equal to its power to target creature or planeswalker."),
            Some((
                "TargetOne(Or([Creature, Planeswalker]))".to_owned(),
                "DealDamage(This, StatOf(This, Power), Target(0))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_for_each() {
        assert_eq!(
            parsed("Creatures you control get +1/+1 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))))), \
                 Toughness(Up(CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))])))))]))), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_for_each_half_scaled() {
        // "+1/+0 for each": power scales, toughness fixed at 0.
        assert_eq!(
            parsed("Creatures you control get +1/+0 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))))), \
                 Toughness(Up(0))]))), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_for_each_nonunit_scales_by_product() {
        // "+2/+2 for each": both sides scale by the Times product.
        assert_eq!(
            parsed("Creatures you control get +2/+2 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(Times(Literal(2), CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))])))))), \
                 Toughness(Up(Times(Literal(2), CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))]))))))]))), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn durational_pump_for_each_asymmetric_scales_by_product() {
        // "+2/+0 for each" (Goblin Piledriver shape): power scales via the
        // Times product, toughness fixed at 0.
        assert_eq!(
            parsed("Creatures you control get +2/+0 for each Goblin you control until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, Several([Power(Up(Times(Literal(2), CountOf(Objects(And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))])))))), \
                 Toughness(Up(0))]))), \
                 duration: FixedUntil(EndOfTurn))".to_owned()
            ))
        );
    }

    #[test]
    fn durational_pump_for_each_malformed_token_declines() {
        // "+01": the wildcard arm strips '+' and parses "01" as u32 1, but
        // fails the `n >= 2` guard (neither the "+0" nor "+1" literal arm
        // matches the zero-padded string) -> decline.
        assert!(declines(
            "Creatures you control get +01/+01 for each Goblin you control until end of turn."
        ));
        // A numeral too large for u32 fails the wildcard arm's parse -> decline.
        assert!(declines(
            "Creatures you control get +99999999999/+0 for each Goblin you control until end of turn."
        ));
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn gain_life_for_each_attacking_filter() {
        // Dwynen lifegain: "you gain 1 life for each attacking Elf you control."
        // The "attacking" status rides the shared filter grammar; base must be 1.
        assert_eq!(
            parsed("you gain 1 life for each attacking Elf you control."),
            Some((
                String::new(),
                "ChangeLife(You, Up(CountOf(Objects(And([Permanent, Subtype(Elf), Attacking, ControlledBy(Ref(You))])))))"
                    .to_owned()
            ))
        );
        // Fixed life is still a bare numeral (regression).
        assert_eq!(
            parsed("You gain 3 life."),
            Some((String::new(), "ChangeLife(You, Up(3))".to_owned()))
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
                "May(who: You, effect: Create(agent: You, count: 1, token: Token(color_indicator: [Green], types: [Creature], \
                 subtypes: [Elf, Warrior], power: 1, toughness: 1)))"
                    .to_owned()
            ))
        );
        // A `you may` over a targeted effect carries the inner target through.
        assert_eq!(
            parsed("You may draw a card."),
            Some((String::new(), "May(who: You, effect: Draw(1))".to_owned()))
        );
    }

    #[test]
    fn may_declines_unparseable_inner() {
        // The whole `you may` production declines when the inner effect doesn't
        // parse (no partial parse).
        assert!(declines("you may flip a coin."));
    }

    /// A nullary `OneShotEffect`-kind macro template (`investigate`) resolves
    /// as the effect body through the final macro-template fallthrough —
    /// emitting the bare invocation, no targets. Both the bare and
    /// Title-Case leads match (the template is case-folded).
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
            Some((
                String::new(),
                "May(who: You, effect: Investigate)".to_owned()
            ))
        );
    }

    /// A slot-bearing `OneShotEffect`-kind macro whose param is a `Reference`
    /// resolves through the fallthrough: the self-reference sigil `~` fills
    /// the slot as `This` ([CR#201.5]). Regenerate (`template: "regenerate
    /// ${0}"`, `params: [Reference]`) is the flagship — `regenerate ~.` ->
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
            Some((String::new(), "Sacrifice(You, This)".to_owned()))
        );
        assert_eq!(
            parsed("Sacrifice ~."),
            Some((String::new(), "Sacrifice(You, This)".to_owned()))
        );
    }

    #[test]
    fn sacrifice_unless_pay_toll() {
        assert_eq!(
            parsed("Sacrifice ~ unless you pay {2}."),
            Some((
                String::new(),
                "Unless(effect: Sacrifice(You, This), unless: [Mana([Generic(2)])])".to_owned()
            ))
        );
        assert_eq!(
            parsed("Sacrifice it unless you pay {W}{W}."),
            Some((
                String::new(),
                "Unless(effect: Sacrifice(You, This), unless: [Mana([White,White])])".to_owned()
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
                "TargetOne(And([Creature, ControlledBy(Ref(You))]))".to_owned(),
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
        assert!(
            parse_clause("Regenerate target creature.", &ctx)
                .unwrap()
                .is_none()
        );
        assert!(
            parse_clause("Regenerate enchanted creature.", &ctx)
                .unwrap()
                .is_none()
        );
    }

    /// The fallthrough requires the WHOLE clause to be the template — a clause
    /// with trailing text past the action word declines (no partial parse).
    #[test]
    fn macro_effect_declines_on_trailing_text() {
        let ctx = crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent);
        // "investigate twice" is a repeated-action shape, not the bare template.
        assert!(parse_clause("investigate twice.", &ctx).unwrap().is_none());
        // An unknown action word still declines.
        assert!(parse_clause("teleport.", &ctx).unwrap().is_none());
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
                 then: Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, PowerAndToughnessUp(2, 2))), duration: FixedUntil(EndOfTurn)), \
                 otherwise: Continuously(effect: Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
                 Modify(It, PowerAndToughnessUp(1, 1))), duration: FixedUntil(EndOfTurn)))".to_owned()
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
        // stays Unparsed rather than emit an unverified condition). ("you
        // control a creature" no longer qualifies as ungrounded — the
        // `YouControl` condition macro, authored for the `Conditionally`
        // composition, grounds it too; the shared routing means `parse_if`
        // benefits from every condition macro, not just its own.)
        assert!(
            parsed_with_macros("Draw a card. If the moon is full, draw two cards instead.")
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
        // `It` with no shared announce list, which the `If` node can't
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

    /// The sentence-order `Sequentially` production: two parseable sentences
    /// join in oracle order, targets declared by the first only.
    #[test]
    fn sequence_parses_counter_then_gain() {
        let parsed = parse_clause(
            "Counter target spell. You gain 5 life.",
            &crate::parsers::test_ctx::ctx(crate::resolve::CardKind::Spell),
        )
        .unwrap()
        .expect("both sentences are productions");
        assert_eq!(parsed.targets, vec!["TargetOne(Spell)".to_owned()]);
        assert_eq!(
            parsed.effect,
            "Sequentially([Counter(Target(0)), ChangeLife(You, Up(5))])"
        );
    }

    /// The loot idiom ([CR#121.1,701.9b]): "Draw a card, then discard a
    /// card." — the narrow draw/discard "then" production, not the general
    /// sentence-order `Sequentially` (this is ONE sentence, no ". " split).
    #[test]
    fn loot_draws_then_discards() {
        assert_eq!(
            parsed("Draw a card, then discard a card."),
            Some((
                String::new(),
                "Sequentially([Draw(1), Discard(1)])".to_owned()
            ))
        );
    }

    /// The rummage idiom ([CR#701.9b,121.1]) — the reverse order: "Discard a
    /// card, then draw a card."
    #[test]
    fn rummage_discards_then_draws() {
        assert_eq!(
            parsed("Discard a card, then draw a card."),
            Some((
                String::new(),
                "Sequentially([Discard(1), Draw(1)])".to_owned()
            ))
        );
    }

    /// A scaled loot with independently-sized halves (Blessed Breath, Fact or
    /// Fiction's kin): "Draw two cards, then discard a card."
    #[test]
    fn loot_scales_independently_per_side() {
        assert_eq!(
            parsed("Draw two cards, then discard a card."),
            Some((
                String::new(),
                "Sequentially([Draw(2), Discard(1)])".to_owned()
            ))
        );
    }

    /// The "at random" rider ([CR#701.9b]) is a trivial flag on the existing
    /// `Discard` leaf, not a `what` selection: "Draw a card, then discard a
    /// card at random."
    #[test]
    fn loot_discard_at_random() {
        assert_eq!(
            parsed("Draw a card, then discard a card at random."),
            Some((
                String::new(),
                "Sequentially([Draw(1), DiscardAtRandom(1)])".to_owned()
            ))
        );
    }

    /// Riders/shapes outside this production's scope decline cleanly (the
    /// macro-effect fallthrough — not exercised here — is the next arm to try,
    /// but under the EMPTY index it too declines, so the whole clause
    /// declines): a conditional tail, and "discard your hand" (not a
    /// count-N `Discard`).
    #[test]
    fn draw_then_discard_declines_riders_out_of_scope() {
        assert!(declines(
            "Draw a card, then discard a card unless her additional cost was paid."
        ));
        assert!(declines("Discard your hand, then draw seven cards."));
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
        // "Counter target spell. You gain 5 life." parses now — the
        // sentence-order Sequentially production picked it up (both sentences
        // are productions); see `sequence_parses_counter_then_gain`.
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
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        assert_eq!(
            parsed("Return target permanent to its owner's hand."),
            Some((
                "TargetOne(Permanent)".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        // "nonland permanent" rides the shared filter grammar's negation.
        assert_eq!(
            parsed("Return target nonland permanent to its owner's hand."),
            Some((
                "TargetOne(And([Permanent, Not(Type(Land))]))".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
    }

    #[test]
    fn return_chosen_subject_to_hand() {
        // A non-target chosen-subject bounce ([CR#400.3]): the controller picks
        // one permanent they control (may not own -> "its owner's hand"), a
        // `With(ChooseOne, Move(That, Hand))`, no announced target.
        assert_eq!(
            parsed("Return a land you control to its owner's hand."),
            Some((
                String::new(),
                "With(binder: ChooseOne(filter: And([Type(Land), ControlledBy(Ref(You))])), body: Move(That(Permanent), Hand))".to_owned()
            ))
        );
        // The determiner stays in the phrase so "another" reads as
        // self-exclusion (`Not(Ref(This))`).
        assert_eq!(
            parsed("Return another creature you control to its owner's hand."),
            Some((
                String::new(),
                "With(binder: ChooseOne(filter: And([Creature, Not(Ref(This)), ControlledBy(Ref(You))])), body: Move(That(Permanent), Hand))".to_owned()
            ))
        );
        // Controller metadata comes from the parsed filter, not the phrase's
        // final words: a later postfix may follow the controller clause.
        assert_eq!(
            parsed("Return an artifact you control with flying to its owner's hand."),
            Some((
                String::new(),
                "With(binder: ChooseOne(filter: And([Type(Artifact), Has(Flying), ControlledBy(Ref(You))])), body: Move(That(Permanent), Hand))".to_owned()
            ))
        );
        // A bare determiner does not make the effect non-targeted: without the
        // controller restriction this surface must remain unresolved rather
        // than choose from every matching permanent.
        assert!(parsed("Return an artifact to its owner's hand.").is_none());
        assert!(parsed("Return an artifact an opponent controls to its owner's hand.").is_none());
    }

    #[test]
    fn return_self_to_hand_no_target() {
        // The effect body of `{cost}: Return ~ to its owner's hand.` — a
        // self-bounce, no target (the cost is the activated frame's job).
        assert_eq!(
            parsed("Return ~ to its owner's hand."),
            Some((String::new(), "Move(This, Hand)".to_owned()))
        );
    }

    #[test]
    fn return_self_to_your_hand_no_target() {
        // "to your hand" is the common-case idiom of the same self-bounce —
        // `Move(_, Hand)` always lands in the owner's hand either way.
        assert_eq!(
            parsed("Return ~ to your hand."),
            Some((String::new(), "Move(This, Hand)".to_owned()))
        );
        assert_eq!(
            parsed("Return it to your hand."),
            Some((String::new(), "Move(This, Hand)".to_owned()))
        );
    }

    #[test]
    fn return_that_card_to_your_hand() {
        // "Return that card to your hand." — the same product-sited anaphor
        // `parse_return_that_card` reads for a battlefield return
        // ([CR#400.7j]), landing in hand instead.
        assert_eq!(
            parsed("Return that card to your hand."),
            Some((String::new(), "Move(That(Card), Hand)".to_owned()))
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
                "TargetOne(And([Type(Creature), InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        // Bare "card" (no type qualifier) — any card you own there.
        assert_eq!(
            parsed("Return target card from your graveyard to your hand."),
            Some((
                "TargetOne(And([InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
        // "instant or sorcery card" — a card-type disjunction.
        assert_eq!(
            parsed("Return target instant or sorcery card from your graveyard to your hand."),
            Some((
                "TargetOne(And([Or([Type(Instant), Type(Sorcery)]), InZone(Graveyard), \
                 Owner(Ref(You))]))"
                    .to_owned(),
                "Move(Target(0), Hand)".to_owned()
            ))
        );
    }

    #[test]
    fn return_target_card_from_graveyard_to_battlefield() {
        // Reanimation ([CR#400.7]): the same `graveyard_card_filter` shape as
        // the hand twin, landing on the battlefield instead — no rider (the
        // owner-control default already applies).
        assert_eq!(
            parsed("Return target creature card from your graveyard to the battlefield."),
            Some((
                "TargetOne(And([Type(Creature), InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Battlefield)".to_owned()
            ))
        );
        // Bare "card" (no type qualifier) — any card you own there.
        assert_eq!(
            parsed("Return target card from your graveyard to the battlefield."),
            Some((
                "TargetOne(And([InZone(Graveyard), Owner(Ref(You))]))".to_owned(),
                "Move(Target(0), Battlefield)".to_owned()
            ))
        );
    }

    #[test]
    fn return_self_from_graveyard_to_battlefield() {
        // Self-reanimation, no target — `{cost}: Return ~ from your
        // graveyard to the battlefield.` (an activated ability's cost is
        // the activated-frame's job, not this clause's).
        assert_eq!(
            parsed("Return ~ from your graveyard to the battlefield."),
            Some((String::new(), "Move(This, Battlefield)".to_owned()))
        );
        assert_eq!(
            parsed("Return it from your graveyard to the battlefield."),
            Some((String::new(), "Move(This, Battlefield)".to_owned()))
        );
    }

    #[test]
    fn reanimate_declines_riders_and_multi_target() {
        // A trailing rider leaves text the base match doesn't strip — the
        // clause declines rather than mis-parse the base reanimation shape.
        assert!(declines(
            "Return target creature card from your graveyard to the battlefield tapped."
        ));
        assert!(declines(
            "Return ~ from your graveyard to the battlefield tapped."
        ));
        assert!(declines(
            "Return target creature card from your graveyard to the battlefield with a finality counter on it."
        ));
        // Multi-target ("up to two target … cards") isn't a single `target`
        // subject — declines.
        assert!(declines(
            "Return up to two target creature cards from your graveyard to the battlefield."
        ));
        // A mana-value filter isn't modeled by `graveyard_card_filter` yet.
        assert!(declines(
            "Return target creature card with mana value 3 or less from your graveyard to the battlefield."
        ));
    }

    #[test]
    fn bounce_self_to_top_of_library() {
        // The effect body of `{cost}: Put ~ on top of its owner's library.` —
        // a self-bounce, no target (the cost is the activated frame's job).
        assert_eq!(
            parsed("Put ~ on top of its owner's library."),
            Some((String::new(), "Move(This, Library(FromTop(0)))".to_owned()))
        );
        assert_eq!(
            parsed("Put it on top of its owner's library."),
            Some((String::new(), "Move(This, Library(FromTop(0)))".to_owned()))
        );
        // "your library" is the equally-correct common idiom.
        assert_eq!(
            parsed("Put ~ on top of your library."),
            Some((String::new(), "Move(This, Library(FromTop(0)))".to_owned()))
        );
    }

    #[test]
    fn bounce_self_to_bottom_of_library() {
        assert_eq!(
            parsed("Put ~ on the bottom of its owner's library."),
            Some((
                String::new(),
                "Move(This, Library(FromBottom(0)))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Put it on the bottom of its owner's library."),
            Some((
                String::new(),
                "Move(This, Library(FromBottom(0)))".to_owned()
            ))
        );
        // "your library" idiom, bottom anchor.
        assert_eq!(
            parsed("Put ~ on the bottom of your library."),
            Some((
                String::new(),
                "Move(This, Library(FromBottom(0)))".to_owned()
            ))
        );
    }

    #[test]
    fn bounce_target_to_library() {
        assert_eq!(
            parsed("Put target creature on top of its owner's library."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Move(Target(0), Library(FromTop(0)))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Put target creature on the bottom of its owner's library."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Move(Target(0), Library(FromBottom(0)))".to_owned()
            ))
        );
        // "your library" idiom on a targeted subject too.
        assert_eq!(
            parsed("Put target creature on top of your library."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Move(Target(0), Library(FromTop(0)))".to_owned()
            ))
        );
        // "nonland permanent" rides the shared filter grammar's negation.
        assert_eq!(
            parsed("Put target nonland permanent on top of its owner's library."),
            Some((
                "TargetOne(And([Permanent, Not(Type(Land))]))".to_owned(),
                "Move(Target(0), Library(FromTop(0)))".to_owned()
            ))
        );
    }

    #[test]
    fn bounce_to_library_declines_deferred_shapes() {
        // Bare "to its owner's library" (no top/bottom qualifier) is a
        // shuffle-in move — a different, Shuffle(LibraryOf(You))-bearing shape,
        // deferred.
        assert!(declines("Return target creature to its owner's library."));
        assert!(declines("Return ~ to its owner's library."));
        // Parameterized/non-zero anchors aren't modeled here.
        assert!(declines(
            "Put target creature second from the top of its owner's library."
        ));
        assert!(declines(
            "Put target creature third from the top of its owner's library."
        ));
    }

    /// The library-search / tutor family's hand-destination shape
    /// ([CR#701.23a]) — the most common (Renegade Map's "reveal it, put it
    /// into your hand"), plus the "reveal that card" register and the
    /// no-reveal variant, both equally common oracle idioms for the same
    /// structure.
    #[test]
    fn search_library_hand_destination() {
        assert_eq!(
            parsed(
                "Search your library for a basic land card, reveal it, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Land), Supertype(Basic)])), body: \
                 Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // The "that card" register parses identically.
        assert_eq!(
            parsed(
                "Search your library for a basic land card, reveal that card, put it into your hand, then shuffle."
            ),
            parsed(
                "Search your library for a basic land card, reveal it, put it into your hand, then shuffle."
            ),
        );
        // No reveal — an equally attested oracle idiom for the same hand
        // destination.
        assert_eq!(
            parsed("Search your library for a card, put it into your hand, then shuffle."),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Kind(Card)), body: Sequentially([Move(That(Card), \
                 Hand, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// The battlefield-destination shape, tapped and untapped — no reveal
    /// (a battlefield arrival is already public).
    #[test]
    fn search_library_battlefield_destination() {
        assert_eq!(
            parsed(
                "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Land), Supertype(Basic)])), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        assert_eq!(
            parsed(
                "Search your library for a basic land card, put it onto the battlefield, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Land), Supertype(Basic)])), body: \
                 Sequentially([Move(That(Card), Battlefield, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // The "that card" register, again equivalent.
        assert_eq!(
            parsed(
                "Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle."
            ),
            parsed(
                "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle."
            ),
        );
    }

    /// The graveyard-destination shape.
    #[test]
    fn search_library_graveyard_destination() {
        assert_eq!(
            parsed("Search your library for a card, put it into your graveyard, then shuffle."),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Kind(Card)), body: Sequentially([Move(That(Card), \
                 Graveyard, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// A bare card-type-noun filter ("a creature card", "an artifact card")
    /// -> `Type("<T>")` — the library-search twin of
    /// [`graveyard_card_type`], which this production reuses directly.
    #[test]
    fn search_library_type_filters() {
        assert_eq!(
            parsed(
                "Search your library for a creature card, reveal it, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Type(Creature)), body: \
                 Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        assert_eq!(
            parsed(
                "Search your library for a land card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Type(Land)), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // A colored card-type filter — "a green creature card".
        assert_eq!(
            parsed(
                "Search your library for a green creature card, reveal it, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Creature), ColorIs(Green)])), body: \
                 Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // A color-COUNT adjective (not a color itself) — "a colorless
        // creature card" (Eldrazi tutors: Sylvan Scrying's twin, "colorless"
        // is `Colorless`, not `ColorIs`).
        assert_eq!(
            parsed(
                "Search your library for a colorless creature card, reveal it, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Creature), Colorless])), body: \
                 Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// A bare subtype (or subtype list) filter, with no "basic" qualifier —
    /// the `Subtype`/`Or([Subtype, …])` atom ALONE, no parent-Type wrapper
    /// [CR#205.3m]: a Tribal card gives its printed creature subtype to a
    /// NONCREATURE card, so "a Goblin card" (Goblin Matron) must also match a
    /// Tribal Instant — Goblin (Tarfire), which an injected `Type("Creature")`
    /// would wrongly exclude.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype parse"
    )]
    fn search_library_bare_subtype_filters() {
        assert_eq!(
            parsed(
                "Search your library for a Forest card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Subtype(Forest)), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // An artifact subtype — "an Equipment card".
        assert_eq!(
            parsed(
                "Search your library for an Equipment card, reveal it, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Subtype(Equipment)), \
                 body: Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), \
                 Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // A creature subtype — "a Goblin card" (Goblin Matron) — no
        // `Type("Creature")` wrapper, so a Tribal Instant/Sorcery — Goblin
        // (Tarfire) still matches.
        assert_eq!(
            parsed(
                "Search your library for a Goblin card, reveal that card, put it into your hand, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Subtype(Goblin)), \
                 body: Sequentially([Reveal(what: That(Card)), Move(That(Card), Hand, []), \
                 Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // A two-member subtype disjunction, no "basic" — "a Swamp or Mountain
        // card".
        assert_eq!(
            parsed(
                "Search your library for a Swamp or Mountain card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Or([Subtype(Swamp), \
                 Subtype(Mountain)])), body: Sequentially([Move(That(Card), Battlefield, \
                 [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// A "<Subtype> permanent card" phrase declines rather than dropping the
    /// "permanent" qualifier: "a Rebel permanent card" must match a Tribal
    /// Enchantment — Rebel (Bound in Silence), which injecting
    /// `Type("Creature")` (Rebel's catalog category) would wrongly exclude —
    /// "permanent" is a card-type-CLASS constraint, not a redundant repeat of
    /// the subtype's own category.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype parse"
    )]
    fn search_library_declines_subtype_permanent_card() {
        assert!(declines(
            "Search your library for a Dragon permanent card, put that card onto the battlefield, then shuffle."
        ));
        assert!(declines(
            "Search your library for a Rebel permanent card, put that card onto the battlefield, then shuffle."
        ));
    }

    /// "basic" plus a single land subtype, or a comma-"or" list of them —
    /// Wayfarer's Bauble / dual-land fetch shapes.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype parse"
    )]
    fn search_library_basic_subtype_filters() {
        assert_eq!(
            parsed(
                "Search your library for a basic Plains card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Supertype(Basic), \
                 Subtype(Plains)])), body: Sequentially([Move(That(Card), Battlefield, \
                 [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // A 3-member comma-"or" list — "a basic Plains, Swamp, or Forest
        // card".
        assert_eq!(
            parsed(
                "Search your library for a basic Plains, Swamp, or Forest card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Supertype(Basic), \
                 Or([Subtype(Plains), Subtype(Swamp), Subtype(Forest)])])), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
        // "snow land" — a different supertype, same no-subtype shape as
        // "basic land".
        assert_eq!(
            parsed(
                "Search your library for a snow land card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: And([Type(Land), Supertype(Snow)])), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// A top-level "X card or a Y card" disjunction of two full filter
    /// phrases (Wayfarer's Bauble's "a basic land card or a Desert card") —
    /// distinct from the bare subtype-list disjunction (no repeated
    /// determiner) [`search_library_basic_subtype_filters`] covers.
    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype parse"
    )]
    fn search_library_or_of_full_filters() {
        assert_eq!(
            parsed(
                "Search your library for a basic land card or a Desert card, put it onto the battlefield tapped, then shuffle."
            ),
            Some((
                String::new(),
                "With(binder: SearchOne(filter: Or([And([Type(Land), Supertype(Basic)]), \
                 Subtype(Desert)])), body: \
                 Sequentially([Move(That(Card), Battlefield, [Tapped]), Shuffle(LibraryOf(You))]))"
                    .to_owned()
            ))
        );
    }

    /// Declined shapes: a plural "up to N" search (no group-reveal primitive,
    /// and this migration doesn't yet produce `MoveGroup`), a self-name
    /// search ("a card named ~" — no established self-reference convention
    /// for `Named`), a dynamic mana-value filter, a heterogeneous multi-find,
    /// and a foreign destination ("their hand").
    #[test]
    fn search_library_declines_unbuilt_shapes() {
        assert!(declines(
            "Search your library for up to two basic land cards, reveal them, put them into your hand, then shuffle."
        ));
        assert!(declines(
            "Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle."
        ));
        assert!(declines(
            "Search your library for a card named ~, put it onto the battlefield tapped, then shuffle."
        ));
        assert!(declines(
            "Search your library for a creature card with mana value 3 or less, reveal it, put it into your hand, then shuffle."
        ));
        assert!(declines(
            "Search your library for a Zombie card and a Swamp card, reveal them, put them into your hand, then shuffle."
        ));
        assert!(declines(
            "Search your library for a basic land card, reveal it, put it into their hand, then shuffle."
        ));
        // The library-position tail this production doesn't build.
        assert!(declines(
            "Search your library for an enchantment card, reveal it, then shuffle and put that card on top."
        ));
    }

    /// Graveyard-hate exile ([CR#701.13a],[CR#400.7]): "Exile target [<type>]
    /// card from a graveyard." — ANY player's graveyard, unlike the
    /// graveyard-recursion family's owner-scoped filter, so no `Owner` atom
    /// rides the predicate.
    #[test]
    fn exile_target_card_from_a_graveyard() {
        assert_eq!(
            parsed("Exile target creature card from a graveyard."),
            Some((
                "TargetOne(And([Type(Creature), InZone(Graveyard)]))".to_owned(),
                "Move(Target(0), Exile)".to_owned()
            ))
        );
        // Bare "card" (no type qualifier) — any card in a graveyard, no `And`
        // wrapper around the lone `InZone(Graveyard)` atom.
        assert_eq!(
            parsed("Exile target card from a graveyard."),
            Some((
                "TargetOne(InZone(Graveyard))".to_owned(),
                "Move(Target(0), Exile)".to_owned()
            ))
        );
        // "instant or sorcery card" — a card-type disjunction, same as the
        // your-graveyard sibling.
        assert_eq!(
            parsed("Exile target instant or sorcery card from a graveyard."),
            Some((
                "TargetOne(And([Or([Type(Instant), Type(Sorcery)]), InZone(Graveyard)]))"
                    .to_owned(),
                "Move(Target(0), Exile)".to_owned()
            ))
        );
        // The general (non-graveyard) exile production is untouched.
        assert_eq!(
            parsed("Exile target creature."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Move(Target(0), Exile)".to_owned()
            ))
        );
    }

    /// "Shuffle your graveyard into your library." graduates via the builtin
    /// `ShuffleYourGraveyardIntoLibrary` macro's nullary `OneShotEffect`
    /// bare-emittable fallthrough — no bespoke parser arm.
    #[test]
    fn shuffle_your_graveyard_into_your_library_macro() {
        assert_eq!(
            parsed_with_macros("Shuffle your graveyard into your library."),
            Some((String::new(), "ShuffleYourGraveyardIntoLibrary".to_owned()))
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
            "Tap target creature. Target(0) doesn't untap during its controller's next untap step."
        ));
    }

    #[test]
    fn destroy_target_disjunction_types() {
        // "artifact or enchantment" -> a Or of the two card types.
        assert_eq!(
            parsed("Destroy target artifact or enchantment."),
            Some((
                "TargetOne(Or([Type(Artifact), Type(Enchantment)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // "creature or planeswalker" -> the battlefield macros disjoined.
        assert_eq!(
            parsed("Destroy target creature or planeswalker."),
            Some((
                "TargetOne(Or([Creature, Planeswalker]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        assert_eq!(
            parsed("Destroy target artifact or land. It can't be regenerated."),
            Some((
                "TargetOne(Or([Type(Artifact), Type(Land)]))".to_owned(),
                "DestroyNoRegen(Target(0))".to_owned(),
            ))
        );
        assert_eq!(
            parsed_with_macros("Destroy target nonbasic land."),
            Some((
                "TargetOne(NonbasicLand)".to_owned(),
                "Destroy(Target(0))".to_owned(),
            ))
        );
        assert_eq!(
            parsed("Destroy target artifact or land."),
            Some((
                "TargetOne(Or([Type(Artifact), Type(Land)]))".to_owned(),
                "Destroy(Target(0))".to_owned()
            ))
        );
        // Catalog subtype heads are not card-type nouns. The distinction is
        // retained by `ParsedFilter`, never recovered from its RON spelling.
        assert!(parsed("Destroy target Goblin or Elf.").is_none());
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
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn deal_damage_broadened_targets() {
        // "target opponent" — a single opponent player.
        assert_eq!(
            parsed("~ deals 1 damage to target opponent."),
            Some((
                "TargetOne(OpponentOf(Ref(You)))".to_owned(),
                "DealDamage(This, 1, Target(0))".to_owned()
            ))
        );
        // "target attacking or blocking creature" — a shared-head status
        // disjunction.
        assert_eq!(
            parsed("~ deals 4 damage to target attacking or blocking creature."),
            Some((
                "TargetOne(And([Creature, Or([Attacking, Blocking])]))".to_owned(),
                "DealDamage(This, 4, Target(0))".to_owned()
            ))
        );
        // "target creature or planeswalker" — disjoined object target.
        assert_eq!(
            parsed("~ deals 5 damage to target creature or planeswalker."),
            Some((
                "TargetOne(Or([Creature, Planeswalker]))".to_owned(),
                "DealDamage(This, 5, Target(0))".to_owned()
            ))
        );
        // "each creature and each player" — the unioned distributive sweep,
        // via the `DealsDamageToEach` macro over the unioned filter
        // ([CR#608.2d]).
        assert_eq!(
            parsed("~ deals 2 damage to each creature and each player."),
            Some((
                String::new(),
                "DealsDamageToEach(2, Or([Creature, Player]))".to_owned()
            ))
        );
        assert_eq!(
            parsed("~ deals X damage to each creature with flying and each player."),
            Some((
                String::new(),
                "DealsDamageToEach(X, Or([And([Creature, Has(Flying)]), Player]))".to_owned()
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
                "TargetOne(And([Creature, ControlledBy(Ref(You))]))".to_owned(),
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
    fn put_counter_on_each() {
        // "Put a +1/+1 counter on each creature you control." — the mass
        // patient wraps the single-object placement in the effect-level `Each`
        // distribution over the matching set, binding each member as `It`
        // ([CR#122.1,608.2d]); no target. Reuses the shared object-target
        // grammar for the selection filter.
        assert_eq!(
            parsed_with_macros("Put a +1/+1 counter on each creature you control."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(And([Creature, ControlledBy(Ref(You))]))), \
                 effect: PutCounters(It, P1P1Counter, 1))"
                    .to_owned()
            ))
        );
        // The bare "each creature" set (no controller postfix).
        assert_eq!(
            parsed_with_macros("Put a -1/-1 counter on each creature."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(Creature)), \
                 effect: PutCounters(It, M1M1Counter, 1))"
                    .to_owned()
            ))
        );
        // Plural count over the mass patient: "two +1/+1 counters on each …".
        assert_eq!(
            parsed_with_macros("Put two +1/+1 counters on each creature you control."),
            Some((
                String::new(),
                "Each(binder: Existing(SelectAll(And([Creature, ControlledBy(Ref(You))]))), \
                 effect: PutCounters(It, P1P1Counter, 2))"
                    .to_owned()
            ))
        );
        // The chosen-target "each of up to <n> target …" form is a separate
        // seam (bounded-choice target selection) and declines cleanly here.
        assert!(
            parsed_with_macros("Put a +1/+1 counter on each of up to two target creatures.")
                .is_none()
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
