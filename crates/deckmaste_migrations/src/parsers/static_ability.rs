//! Always-on static abilities on permanents: "gets ±N/±N", "have/has/gain/gains
//! <keyword>", "<subject> can't attack/block". The ±N/±N + keyword-grant +
//! subject/target grammar is shared via [`crate::parsers::modify`]; this module
//! renders the bare positional `Static(<one effect>)` RON. Declines
//! (`Ok(None)`) on spells, durational clauses, targeted subjects, a subject
//! with more than one restricted action (one `StaticEffect` per `Static` — no
//! bundling), or anything its productions don't fully cover.

use crate::parsers::modify;
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;

pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    parse(line, ctx)
}

/// Productions that consult the reverse template index (directly, via
/// [`parse_pt`]'s `Modification` fold) propagate `anyhow::Result` — a
/// same-kind ambiguous macro match is a hard generation error, not a decline;
/// every other (`Option`-returning) production is lifted with `Ok`.
fn parse(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    if ctx.kind == CardKind::Spell {
        return Ok(None);
    }
    let low = line.to_ascii_lowercase();
    if low.contains("until end of turn") || low.contains("this turn") {
        return Ok(None);
    }
    let body = line.strip_suffix('.').unwrap_or(line);

    if let Some(row) = parse_conditional(body, ctx)? {
        return Ok(Some(row));
    }
    if let Some(row) = parse_cost_modifier(body) {
        return Ok(Some(row));
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" can't ", " cannot "]) {
        return Ok(parse_restriction(subj, pred));
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" can block "]) {
        return Ok(parse_block_permission(subj, pred));
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" attacks ", " attack "]) {
        return Ok(parse_requirement(subj, pred));
    }
    if let Some((subj, pred)) = modify::split_marker(body, &[" gets ", " get "]) {
        return parse_pt(subj, pred, ctx);
    }
    if let Some((subj, pred)) =
        modify::split_marker(body, &[" have ", " has ", " gain ", " gains "])
    {
        return Ok(parse_grant(subj, pred, ctx));
    }
    Ok(None)
}

/// "<subject> gets ±N/±M [and have/has/gain/gains <kw…>]." → the always-on P/T
/// anthem/pump static. The change folds to its `Modification`-kind macro
/// invocation (`PowerAndToughnessUp(N, M)`/`PowerAndToughnessDown(N, M)`) when
/// the reverse index has a template that renders the WHOLE "gets ±N/±M"
/// phrase — tried against `pred` as a whole, so a grant-tail combo ("gets
/// +N/+M and have …") never fully consumes (the tail survives past the
/// template's own "±N/±M" span) and falls straight through to the core
/// `changes` list, unaffected. The subject folds independently to its
/// `Selection`-kind macro invocation (`OtherCreaturesYouControl`/
/// `CreaturesOpponentControls`) when the raw English subject exactly matches
/// a Selection template (plural-collective wording, no singularize/pluralize
/// normalization); otherwise it falls back to the core
/// `SelectAll(<filter>)` target. A same-kind ambiguous match from the index is
/// a hard generation error (`?`), not a decline.
fn parse_pt(subj: &str, pred: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let Some(filter) = modify::subject_to_filter(subj) else {
        return Ok(None);
    };
    // Optional combo tail: "+N/+M and have/has/gain/gains <kw…>" → the P/T changes
    // followed by one GainAbility per granted keyword.
    let (pt_part, grant_tail) = modify::split_grant_tail(pred);
    let Some(mut changes) = modify::parse_pt_changes(pt_part.trim()) else {
        return Ok(None);
    };
    if let Some(tail) = grant_tail {
        let Some(kw_changes) = modify::parse_keyword_changes(tail) else {
            return Ok(None);
        };
        changes.extend(kw_changes);
    }
    let gets = format!("gets {}", pred.trim());
    let change = match ctx.index.match_with(
        "Modification",
        &gets,
        crate::parsers::effect::count_delim_slot_reader,
    )? {
        Some(m) if m.consumed == gets.len() => m.invocation,
        _ => modify::changes_to_modification(&changes), // core fallback
    };
    let subject = subj.trim();
    let target = match ctx.index.match_kind("Selection", subject)? {
        Some(m) if m.consumed == subject.len() => {
            format!("Each({}, Modify(It, {}))", m.macro_name, change)
        }
        _ => modify::filter_to_target(&filter).wrap(&change), // core fallback
    };
    Ok(Some(format!("Static({target})")))
}

/// "<subject> have/has/gain/gains <kw…>." → the always-on keyword-grant
/// static. `ctx` is threaded for signature symmetry with [`parse_pt`] (a
/// later task folds the shared SUBJECT grammar both productions call through
/// [`modify::subject_to_filter`]); this production has no `Modification` fold
/// of its own, so it stays `Option`-returning — it never reads the index, so
/// it can't hit an ambiguous match.
fn parse_grant(subj: &str, pred: &str, _ctx: &ResolveCtx) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    let changes = modify::parse_keyword_changes(pred)?;
    let change = modify::changes_to_modification(&changes);
    Some(format!(
        "Static({})",
        modify::filter_to_target(&filter).wrap(&change)
    ))
}

/// "<subject> can't <action[ or action…]>" → one `Cant(<verb>)` per action.
/// The subject is a bare `Predicate` (Deontic actions carry a `Predicate`). The
/// active verbs ("attack"/"block") anchor the subject on the
/// actor side (`by`); the passive "be blocked …" evasion forms anchor it on
/// the blocked side (`on`) and read a blocker-quality clause. Any action
/// outside the known set declines the whole.
fn parse_restriction(subj: &str, pred: &str) -> Option<String> {
    let filter = modify::subject_to_filter(subj)?;
    // The passive "be blocked …" evasion clause is a single clause whose tail
    // ("by creatures with power 2 or less") contains its own "or" — handle it
    // BEFORE the active-verb list split, which would shred that tail.
    let low = pred.to_ascii_lowercase();
    if low.starts_with("be blocked") {
        let row = parse_cant_be_blocked(&filter, &low)?;
        return Some(format!("Static({row})"));
    }
    let effects: Option<Vec<String>> = modify::split_list(pred)
        .iter()
        .map(|act| match act.to_ascii_lowercase().as_str() {
            "attack" => Some(format!("Cant(Attack(by: {filter}))")),
            "block" => Some(format!("Cant(Block(by: {filter}))")),
            _ => None,
        })
        .collect();
    // One `StaticEffect` per `Static` ability — multiple restricted actions
    // ("can't attack or block") have no single-effect encoding, so decline
    // rather than emit an invalid bundle; graduation leaves the line
    // `Unparsed` for a future multi-ability split.
    match effects?.as_slice() {
        [one] => Some(format!("Static({one})")),
        _ => None,
    }
}

/// The passive "be blocked …" evasion clause ([CR#509.1b], [CR#702]) — the
/// subject (`on`) is the creature being blocked. `clause` is the lowercased
/// predicate after "can't " (e.g. "be blocked", "be blocked by creatures with
/// power 2 or less", "be blocked by more than one creature"). Returns one
/// `Cant(Block(on: <subj>, …))` row, or `None` for an unrecognized tail.
fn parse_cant_be_blocked(on: &str, clause: &str) -> Option<String> {
    let tail = clause.strip_prefix("be blocked")?.trim();
    // "~ can't be blocked." — unblockable: no blocker may block it.
    if tail.is_empty() {
        return Some(format!("Cant(Block(on: {on}))"));
    }
    // "… by more than one creature." — an arrangement bound: a blocker set
    // larger than one is forbidden ([CR#509.1b]).
    if tail == "by more than one creature" {
        return Some(format!("Cant(Block(on: {on}, count: Greater(1)))"));
    }
    // "… except by N or more creatures." — menace generalized ([CR#702.111b]
    // is the N=2 case): a blocker set of fewer than N is forbidden. N is a
    // spelled cardinal ("three") or a digit.
    if let Some(n) = tail
        .strip_prefix("except by ")
        .and_then(|t| t.strip_suffix(" or more creatures"))
        .and_then(crate::parsers::effect::number_word)
    {
        return Some(format!("Cant(Block(on: {on}, count: Less({n})))"));
    }
    // "… by creatures with power N or less/greater." — a blocker-quality
    // restriction (the candidate blocker's power).
    let by = tail
        .strip_prefix("by ")
        .and_then(crate::parsers::filter::parse_phrase)?;
    Some(format!("Cant(Block(on: {on}, by: {by}))"))
}

/// "<subject> can block <predicate>" → a blocking restriction/permission.
///   - "only creatures with flying" → `Cant(Block(by: <subj>, on: Not(<X>)))`:
///     the subject can't block anything that isn't `<X>` ([CR#509.1a]).
///   - "an additional creature each combat" → a multi-block permission
///     ([CR#509.1a] default = one). Not yet engine-evaluated (the May/Gate
///     Block seam), but representable, so it graduates.
fn parse_block_permission(subj: &str, pred: &str) -> Option<String> {
    let by = modify::subject_to_filter(subj)?;
    if let Some(only) = pred.strip_prefix("only ") {
        let on = crate::parsers::filter::parse_phrase(only.trim())?;
        return Some(format!("Static(Cant(Block(by: {by}, on: Not({on}))))"));
    }
    if pred == "an additional creature each combat" {
        // The default per-blocker cap is one creature ([CR#509.1a]); this row
        // raises it to two. A May permission over the second-block slot.
        return Some(format!("Static(May(Block(by: {by}, count: AtMost(2))))"));
    }
    None
}

/// "<subject> attack[s] each combat if able" → a `Must(Attack(by: <subject>))`
/// requirement static ([CR#508.1d]). The predicate must be exactly the
/// "each combat if able" requirement tail — any other prose after the verb
/// (e.g. "this turn", "a player") declines, so the bare verb marker never
/// swallows a durational or targeted clause.
fn parse_requirement(subj: &str, pred: &str) -> Option<String> {
    if pred.trim() != "each combat if able" {
        return None;
    }
    let filter = modify::subject_to_filter(subj)?;
    Some(format!("Static(Must(Attack(by: {filter})))"))
}

/// "<adjective> spell(s) [you cast] cost {N} less/more to cast" → a
/// [`CostModifier`](deckmaste_core::StaticEffect) static ([CR#118.7,601.2f]):
/// "less" is a `Reduce`, "more" an `Increase`, of {N} generic mana, whose `of`
/// filter is the spell-subject predicate (Goblin Warchief = "Goblin spells you
/// cast"). This is the PARSE half — the engine's total-cost
/// application of the emitted row is engine-cost-modification. The amount is a
/// bare mana run (`{X}` declines — a variable reduction is not a fixed pipeline
/// step); anything else about the shape declines and the line stays `Unparsed`.
fn parse_cost_modifier(body: &str) -> Option<String> {
    use crate::parsers::cost::VariableMana;

    let stem = body.strip_suffix(" to cast")?;
    let (subject_amount, change_kw) = stem
        .strip_suffix(" less")
        .map(|s| (s, "Reduce"))
        .or_else(|| stem.strip_suffix(" more").map(|s| (s, "Increase")))?;
    let (subject, amount) = subject_amount.rsplit_once(" cost ")?;
    let of = crate::parsers::filter::spell_subject(subject.trim())?;
    let component = crate::parsers::cost::mana_component(amount.trim(), VariableMana::Decline)
        .ok()
        .flatten()?;
    Some(format!(
        "Static(CostModifier(of: {of}, change: {change_kw}([{component}])))"
    ))
}

/// "As long as <condition>, <static clause>." -> `Static(Conditionally(<condition>,
/// <inner>))` ([CR#611.3a]) — the COMPOSITION of the `Condition`-macro path
/// and the existing static-ability productions, nothing new in either
/// vocabulary. Condition-FIRST only: the renderer
/// (`crates/deckmaste_cards/src/render/ability.rs`'s `conditionally_qualified`)
/// only emits the "As long as X, Y." phrasing — `Conditionally` carries no
/// order marker (the `(Condition, StaticEffect)` pair can't remember which way
/// the oracle wrote it), and the SAME condition/effect pair appears in BOTH
/// orders across different real cards (e.g. "As long as you control an
/// artifact, ~ gets +1/+0 and has deathtouch." vs "~ gets +1/+1 as long as you
/// control an artifact."), so no per-card heuristic recovers it. A
/// condition-LAST card ("Y as long as X.") is left `Unparsed` — an order
/// marker would be new core grammar, outside this composition's hard
/// constraint.
///
/// The condition phrase routes through the shared `Condition`-macro path
/// ([`crate::parsers::condition::resolve`], the same routing
/// [`crate::parsers::effect::parse_if`] uses for one-shot `If`) — new
/// condition phrasings are added by authoring a macro, never here. The static
/// clause after the comma re-enters [`parse`] (recursing through every
/// existing static production), so any already-supported subject/effect shape
/// composes under a condition at no cost in new static vocabulary. A leading
/// "it"/"It" pronoun in that clause — the common anaphor referring back to the
/// condition's own subject ("enchanted creature is black, it gets +1/+1") —
/// is rewritten to that exact subject phrase first ([`rewrite_it_anaphor`]),
/// so the recursive [`parse`] sees a subject its own productions already
/// resolve.
fn parse_conditional(body: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let Some(rest) = body.strip_prefix("As long as ") else {
        return Ok(None);
    };
    let Some((cond_phrase, tail)) = rest.split_once(", ") else {
        return Ok(None);
    };
    let cond_phrase = cond_phrase.trim();
    let Some(condition) = crate::parsers::condition::resolve(cond_phrase, ctx)? else {
        return Ok(None);
    };
    let tail = rewrite_it_anaphor(cond_phrase, tail.trim());
    let tail_line = format!("{tail}.");
    let Some(inner) = parse(&tail_line, ctx)? else {
        return Ok(None);
    };
    let Some(inner_effect) = inner
        .strip_prefix("Static(")
        .and_then(|s| s.strip_suffix(')'))
    else {
        return Ok(None);
    };
    Ok(Some(format!(
        "Static(Conditionally({condition}, {inner_effect}))"
    )))
}

/// A leading "it"/"It" in `tail` is the pronoun anaphor referring back to
/// `cond_phrase`'s own subject ([CR#608.2d]) — substitute the EXACT antecedent
/// text (never blindly `~`: "enchanted creature is black, it gets +1/+1"
/// means the enchanted creature gets +1/+1, not the aura/equipment itself), so
/// the recursive static parse sees a subject its own productions already
/// resolve. Reuses [`crate::parsers::condition::SUBJECT_WORDS`] — the same
/// closed subject set the condition-phrase macro slot accepts, so both sides
/// agree on what counts as a valid antecedent. Declines (returns `tail`
/// unchanged) when `cond_phrase`'s subject isn't one of that set, or `tail`
/// has no leading pronoun to rewrite.
fn rewrite_it_anaphor(cond_phrase: &str, tail: &str) -> String {
    let Some(pronoun_tail) = modify::strip_prefix_ci(tail, "it ") else {
        return tail.to_string();
    };
    for (antecedent, _) in crate::parsers::condition::SUBJECT_WORDS {
        if let Some(after) = modify::strip_prefix_ci(cond_phrase, antecedent)
            && after.starts_with(' ')
        {
            return format!("{antecedent} {pronoun_tail}");
        }
    }
    tail.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(line: &str) -> Option<String> {
        resolve_line(line, &crate::parsers::test_ctx::ctx(CardKind::Permanent)).unwrap()
    }

    /// `stat`'s twin over the REAL builtin macro index, so the
    /// `Modification`-kind macro fold (`PowerAndToughnessUp`/`Down`) is
    /// exercised.
    fn stat_with_macros(line: &str) -> Option<String> {
        resolve_line(
            line,
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap()
    }

    #[test]
    fn anthem_change_folds_to_modification_macro() {
        // Both the CHANGE and the SUBJECT fold to macro invocations via the
        // reverse template index — the fully-folded Elesh Norn end-state.
        assert_eq!(
            stat_with_macros("Other creatures you control get +2/+2.").as_deref(),
            Some("Static(Each(OtherCreaturesYouControl, Modify(It, PowerAndToughnessUp(2, 2))))")
        );
        assert_eq!(
            stat_with_macros("Creatures your opponents control get -2/-2.").as_deref(),
            Some(
                "Static(Each(CreaturesOpponentControls, Modify(It, PowerAndToughnessDown(2, 2))))"
            )
        );
    }

    #[test]
    fn unknown_subject_falls_back_to_core_selectall() {
        // "Creatures you control" has no Selection macro (only
        // "other creatures you control" and "creatures your opponents
        // control" are registered) — the subject fold misses and the target
        // falls back to the core `SelectAll(<filter>)` shape.
        assert_eq!(
            stat_with_macros("Creatures you control get +1/+1.")
                .as_deref()
                .map(|s| s.contains("SelectAll(")),
            Some(true)
        );
    }

    #[test]
    fn pt_anthem_you_control() {
        assert_eq!(
            stat("Creatures you control get +1/+1.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), Modify(It, Several([Power(Up(1)), Toughness(Up(1))]))))"
            )
        );
    }

    #[test]
    fn pt_negative_and_mixed() {
        assert_eq!(
            stat("Creatures your opponents control get -1/-1.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Creature, ControlledBy(OpponentOf(Ref(You)))])), Modify(It, Several([Power(Down(1)), Toughness(Down(1))]))))"
            )
        );
        assert_eq!(
            stat("~ gets +1/-1.").as_deref(),
            Some("Static(Modify(This, Several([Power(Up(1)), Toughness(Down(1))])))")
        );
    }

    #[test]
    fn pt_declines() {
        assert!(stat("Creatures you control get +1/+1.").is_some());
        assert!(
            resolve_line(
                "Target creature gets +2/+2.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line(
                "Creatures you control get +1/+1 until end of turn.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        assert!(
            resolve_line(
                "Creatures you control get +1/+1.",
                &crate::parsers::test_ctx::ctx(CardKind::Spell)
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn grant_single_and_list() {
        assert_eq!(
            stat("Other Goblins have haste.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), Modify(It, GainAbility(Keyword(Haste)))))"
            )
        );
        assert_eq!(
            stat("Creatures you control have flying and vigilance.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), Modify(It, Several([GainAbility(Keyword(Flying)), GainAbility(Keyword(Vigilance))]))))"
            )
        );
        assert_eq!(
            stat("Creatures you control have flying, vigilance, and trample.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), Modify(It, Several([GainAbility(Keyword(Flying)), GainAbility(Keyword(Vigilance)), GainAbility(Keyword(Trample))]))))"
            )
        );
    }

    #[test]
    fn restriction_attack_block() {
        // "can't attack or block" restricts TWO actions — one `StaticEffect`
        // per `Static` ability has no single-effect encoding for that, so it
        // now declines rather than emit an invalid multi-effect bundle.
        assert!(stat("Enchanted creature can't attack or block.").is_none());
        assert_eq!(
            stat("Creatures you control can't attack.").as_deref(),
            Some("Static(Cant(Attack(by: And([Creature, ControlledBy(Ref(You))]))))")
        );
    }

    #[test]
    fn restriction_declines_unknown_action() {
        assert!(stat("Enchanted creature can't transform.").is_none());
    }

    #[test]
    fn cant_be_blocked_unblockable() {
        // Unblockable: no blocker may block This ([CR#509.1b]).
        assert_eq!(
            stat("~ can't be blocked.").as_deref(),
            Some("Static(Cant(Block(on: Ref(This))))")
        );
        // Equipment/aura conferral: the host can't be blocked.
        assert_eq!(
            stat("Equipped creature can't be blocked.").as_deref(),
            Some("Static(Cant(Block(on: Ref(AttachHostOf(This)))))")
        );
        assert_eq!(
            stat("Enchanted creature can't be blocked.").as_deref(),
            Some("Static(Cant(Block(on: Ref(AttachHostOf(This)))))")
        );
    }

    #[test]
    fn cant_be_blocked_by_power() {
        // Blocker-quality power restriction (the candidate blocker's power).
        assert_eq!(
            stat("~ can't be blocked by creatures with power 2 or less.").as_deref(),
            Some("Static(Cant(Block(on: Ref(This), by: And([Creature, Stat(Power, AtMost, 2)]))))")
        );
        assert_eq!(
            stat("~ can't be blocked by creatures with power 3 or greater.").as_deref(),
            Some(
                "Static(Cant(Block(on: Ref(This), by: And([Creature, Stat(Power, AtLeast, 3)]))))"
            )
        );
    }

    #[test]
    fn cant_be_blocked_by_more_than_one() {
        // An arrangement bound: a blocker set larger than one is forbidden.
        assert_eq!(
            stat("~ can't be blocked by more than one creature.").as_deref(),
            Some("Static(Cant(Block(on: Ref(This), count: Greater(1))))")
        );
        // Conferred form on the equip host.
        assert_eq!(
            stat("Each creature you control can't be blocked by more than one creature.")
                .as_deref(),
            Some(
                "Static(Cant(Block(on: And([Creature, ControlledBy(Ref(You))]), count: Greater(1))))"
            )
        );
    }

    #[test]
    fn cant_be_blocked_except_by_n_or_more() {
        // Menace generalized to N=3 — a spelled cardinal in the corpus.
        assert_eq!(
            stat("~ can't be blocked except by three or more creatures.").as_deref(),
            Some("Static(Cant(Block(on: Ref(This), count: Less(3))))")
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn can_block_only_flying() {
        // "can block only creatures with flying" → can't block non-flying
        // ([CR#509.1a]).
        assert_eq!(
            stat("~ can block only creatures with flying.").as_deref(),
            Some("Static(Cant(Block(by: Ref(This), on: Not(And([Creature, Has(Flying)])))))")
        );
    }

    #[test]
    fn can_block_additional_creature() {
        // "can block an additional creature each combat" raises the per-blocker
        // cap to two ([CR#509.1a]) — a May permission.
        assert_eq!(
            stat("~ can block an additional creature each combat.").as_deref(),
            Some("Static(May(Block(by: Ref(This), count: AtMost(2))))")
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn equipped_creature_gets() {
        // "Equipped creature gets +N/+N" → Modify on the attach host, the same
        // shape as the already-wired "Enchanted creature".
        assert_eq!(
            stat("Equipped creature gets +2/+0.").as_deref(),
            Some("Static(Modify(AttachHostOf(This), Several([Power(Up(2)), Toughness(Up(0))])))")
        );
        assert_eq!(
            stat("Equipped creature gets +1/+1 and has trample.").as_deref(),
            Some(
                "Static(Modify(AttachHostOf(This), Several([Power(Up(1)), Toughness(Up(1)), GainAbility(Keyword(Trample))])))"
            )
        );
    }

    #[test]
    fn cant_be_blocked_declines_unknown_tail() {
        // An unrecognized "be blocked …" tail (a phrase the filter parser can't
        // resolve) declines — no wrong Block row.
        assert!(stat("~ can't be blocked by creatures wearing hats.").is_none());
        assert!(stat("~ can block only creatures wearing hats.").is_none());
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn requirement_attack_each_combat() {
        // Goblin Rabblemaster's requirement clause → a Must(Attack) static over
        // the subject filter ([CR#508.1d]).
        assert_eq!(
            stat("Other Goblin creatures you control attack each combat if able.").as_deref(),
            Some(
                "Static(Must(Attack(by: And([Creature, Not(Ref(This)), Subtype(\"Goblin\"), ControlledBy(Ref(You))]))))"
            )
        );
        // A self-ref subject ("~ attacks each combat if able") → Must over This.
        assert_eq!(
            stat("~ attacks each combat if able.").as_deref(),
            Some("Static(Must(Attack(by: Ref(This))))")
        );
    }

    #[test]
    fn requirement_declines_partial_phrase() {
        // The bare " attack " marker must not swallow non-requirement prose; only
        // the exact "each combat if able" tail (singular or plural verb) qualifies.
        assert!(stat("Creatures you control attack this turn if able.").is_none());
        assert!(stat("Creatures you control attack a player.").is_none());
        // A targeted subject still declines (no class filter).
        assert!(stat("Target creature attacks each combat if able.").is_none());
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn pt_anthem_subtype_adjective() {
        // Elvish Archdruid's anthem: a subtype-adjective subject ("Elf creatures").
        assert_eq!(
            stat("Other Elf creatures you control get +1/+1.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Creature, Not(Ref(This)), Subtype(\"Elf\"), ControlledBy(Ref(You))])), Modify(It, Several([Power(Up(1)), Toughness(Up(1))]))))"
            )
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn cost_modifier_reduce_subtype_you_cast() {
        // Goblin Warchief's reducer: "Goblin spells you cast cost {1} less to
        // cast" → a CostModifier scoped to Goblin spells the caster controls
        // ([CR#118.7,601.2f]).
        assert_eq!(
            stat("Goblin spells you cast cost {1} less to cast.").as_deref(),
            Some(
                "Static(CostModifier(of: And([Kind(Spell), Subtype(\"Goblin\"), ControlledBy(Ref(You))]), change: Reduce([Mana([Generic(1)])])))"
            )
        );
    }

    #[test]
    fn cost_modifier_increase_tax_form() {
        // The symmetric taxer: "cost {2} more" → an Increase over the same
        // spell-subject shape.
        assert_eq!(
            stat("Creature spells you cast cost {2} more to cast.").as_deref(),
            Some(
                "Static(CostModifier(of: And([Kind(Spell), Type(\"Creature\"), ControlledBy(Ref(You))]), change: Increase([Mana([Generic(2)])])))"
            )
        );
        // A color adjective and no "you cast" scope (affects all such spells).
        assert_eq!(
            stat("Red spells cost {1} more to cast.").as_deref(),
            Some(
                "Static(CostModifier(of: And([Kind(Spell), ColorIs(Red)]), change: Increase([Mana([Generic(1)])])))"
            )
        );
    }

    #[test]
    fn cost_modifier_declines() {
        // A variable {X} reduction is not a fixed pipeline step.
        assert!(stat("Goblin spells you cast cost {X} less to cast.").is_none());
        // No "spell(s)" head noun — a permanent anthem, not a cost modifier.
        assert!(stat("Goblins you control cost {1} less to cast.").is_none());
        // Unknown adjective declines rather than mint a wrong filter.
        assert!(stat("Wibble spells you cast cost {1} less to cast.").is_none());
        // A durational cost line is not a static (handled by the guard).
        assert!(
            resolve_line(
                "Goblin spells you cast cost {1} less to cast this turn.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype/keyword parse"
    )]
    fn grant_combo_with_pt() {
        assert_eq!(
            stat("Other Goblins get +1/+1 and have mountainwalk.").as_deref(),
            Some(
                "Static(Each(SelectAll(And([Permanent, Subtype(\"Goblin\"), Not(Ref(This))])), Modify(It, Several([Power(Up(1)), Toughness(Up(1)), GainAbility(Keyword(Mountainwalk))]))))"
            )
        );
    }

    #[test]
    fn grant_declines() {
        // unknown keyword
        assert!(stat("Creatures you control have wibble.").is_none());
        // parameterized keyword (leftover) — deferred to macro-keyword-templates
        assert!(stat("Creatures you control have protection from red.").is_none());
        // durational gain
        assert!(
            resolve_line(
                "Creatures you control gain trample until end of turn.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn conditional_you_control_pt_folds() {
        // "As long as you control an artifact, ~ gets +1/+0." ([CR#611.3a]) —
        // the ticket's own condition-first "you control" composition; the
        // `+1/+0` change ALSO folds to its `Modification` macro since this
        // uses the real builtin index.
        assert_eq!(
            stat_with_macros("As long as you control an artifact, ~ gets +1/+0.").as_deref(),
            Some(
                "Static(Conditionally(YouControl(Type(\"Artifact\")), Modify(This, PowerAndToughnessUp(1, 0))))"
            )
        );
    }

    #[test]
    fn conditional_self_reference_with_it_anaphor() {
        // "As long as ~ is attacking, it gets +2/+0." — the ticket's own
        // condition-first self-reference example: the "it" pronoun rewrites to
        // "~" (the condition's own subject) before the recursive static parse.
        assert_eq!(
            stat_with_macros("As long as ~ is attacking, it gets +2/+0.").as_deref(),
            Some(
                "Static(Conditionally(SubjectIs(This, Attacking), Modify(This, PowerAndToughnessUp(2, 0))))"
            )
        );
    }

    #[test]
    fn conditional_enchanted_creature_it_anaphor_targets_the_host() {
        // "As long as enchanted creature is black, it gets +1/+1." — the "it"
        // rewrites to "enchanted creature" (the condition's own subject), NOT
        // "~" (the aura) — a wrong rewrite here would apply the boost to the
        // aura instead of the enchanted creature.
        assert_eq!(
            stat_with_macros("As long as enchanted creature is black, it gets +1/+1.").as_deref(),
            Some(
                "Static(Conditionally(SubjectIs(AttachHostOf(This), ColorIs(Black)), Modify(AttachHostOf(This), PowerAndToughnessUp(1, 1))))"
            )
        );
    }

    #[test]
    fn conditional_keyword_grant() {
        // "As long as ~ is untapped, ~ has hexproof." — a keyword-grant static
        // body under a condition.
        assert_eq!(
            stat_with_macros("As long as ~ is untapped, ~ has hexproof.").as_deref(),
            Some(
                "Static(Conditionally(SubjectIs(This, Status(Untapped)), Modify(This, GainAbility(Keyword(Hexproof())))))"
            )
        );
    }

    #[test]
    fn conditional_declines_condition_last_order() {
        // "~ gets +2/+2 as long as you control an artifact." — condition-LAST:
        // the renderer only emits the condition-first phrasing, so this order
        // is left `Unparsed` rather than silently mis-rendering on round trip.
        assert!(stat_with_macros("~ gets +2/+2 as long as you control an artifact.").is_none());
    }

    #[test]
    fn conditional_declines_unrecognized_condition() {
        assert!(stat("As long as the moon is full, ~ gets +1/+1.").is_none());
    }

    #[test]
    fn conditional_declines_when_inner_static_unsupported() {
        // A recognized condition whose tail isn't itself a supported static
        // clause declines the whole (no partial/wrong RON).
        assert!(
            stat_with_macros(
                "As long as ~ is attacking, it can attack as though it didn't have defender."
            )
            .is_none()
        );
    }
}
