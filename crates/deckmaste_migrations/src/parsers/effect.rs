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
/// decline. Productions are tried in order; the first match wins. The
/// bespoke productions lead (they encode targeting/scope the bare macro
/// templates can't carry); an `Effect`-kind macro template
/// ([`parse_macro_effect`]) is the final fallthrough, so keyword-action lines
/// (`investigate.`, `scry 2.`) route back to the macro whose template renders
/// them. [`ResolveCtx`] carries the reverse template index that fallthrough
/// consults.
pub(super) fn parse_clause(line: &str, ctx: &ResolveCtx) -> Option<ParsedEffect> {
    parse_may(line, ctx)
        .or_else(|| parse_deal_damage(line))
        .or_else(|| parse_draw(line))
        .or_else(|| parse_lose_life(line))
        .or_else(|| parse_gain_life(line))
        .or_else(|| parse_destroy(line))
        .or_else(|| parse_pump(line))
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
/// (`scry 2` — a `Count` magnitude at the end), so a successful read consumes
/// all of `input`. Only `Count` slots are read for now (the keyword-action
/// subset that takes an argument all take a count); an unmodeled slot type
/// declines, failing the whole template cleanly.
fn macro_slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    match ty {
        // A bare numeral count word — `scry two`, `mill 3`. Emitted as a bare
        // numeral (reader-sugar for `Count::Literal`), matching the sibling
        // `Draw`/`Create` count productions.
        "Count" => Some((number_word(input.trim())?.to_string(), input.len())),
        _ => None,
    }
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
/// subject parsed by the shared [`filter`] grammar) and the body
/// `Destroy(Target(0))` ([CR#701.8]). Only the single-target form; board wipes
/// ("destroy all/each …") are a later production. Declines when the subject
/// isn't filter-parseable. Case-insensitive lead, since the clause opens a
/// spell ("Destroy …") or follows a trigger comma ("…, destroy …").
fn parse_destroy(line: &str) -> Option<ParsedEffect> {
    let subject = strip_prefix_ci(line, "destroy ")?
        .strip_suffix('.')?
        .strip_prefix("target ")?;
    let filter = filter::parse_phrase(subject)?;
    Some(ParsedEffect {
        targets: vec![format!("TargetOne({filter})")],
        effect: "Destroy(Target(0))".to_owned(),
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
            crate::parsers::keyword_ability::match_keyword_name(piece)
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
        "target creature" => (
            vec!["TargetOne(Creature)".to_owned()],
            "Target(0)".to_owned(),
        ),
        "target player" => (vec!["TargetOne(Player)".to_owned()], "Target(0)".to_owned()),
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
        _ => return None,
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
    fn durational_pump_team_like_overrun() {
        // Overrun: a team P/T boost + keyword grant lasting until end of turn.
        assert_eq!(
            parsed("Creatures you control get +3/+3 and gain trample until end of turn."),
            Some((
                String::new(),
                "Continuously(effect: Modify(of: Matching(AllOf([Creature, ControlledBy(Ref(You))])), \
                 changes: [AddPower(3), AddToughness(3), GainAbility(Keyword(Trample))]), \
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
                "Continuously(effect: Modify(of: Of(This), changes: [AddPower(1), \
                 AddToughness(1)]), duration: FixedUntil(EndOfTurn))"
                    .to_owned()
            ))
        );
        // Single-target pump ("target creature gets …"): TargetOne + Of(Target(0)).
        assert_eq!(
            parsed("Target creature gets +3/+3 until end of turn."),
            Some((
                "TargetOne(Creature)".to_owned(),
                "Continuously(effect: Modify(of: Of(Target(0)), changes: [AddPower(3), \
                 AddToughness(3)]), duration: FixedUntil(EndOfTurn))"
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
        // A target subject the filter grammar can't parse declines.
        assert!(declines("Destroy target creature with flying."));
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
        // Predefined token -> needs TokenSpec::Named.
        assert!(declines("Create a Treasure token."));
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
                 changes: [AddPower(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))]))), \
                 AddToughness(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])))]), \
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
                 changes: [AddPower(CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))]))), \
                 AddToughness(0)]), \
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
}
