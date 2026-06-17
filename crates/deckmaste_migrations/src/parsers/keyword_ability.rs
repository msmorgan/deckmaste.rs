//! Recognizes a normalized keyword-ability line and renders it as the
//! anticipated macro invocation, by its on-card name: the matched keyword name
//! is the macro head and the argument shape decides arity (nullary / cost /
//! integer / integer+cost). Each keyword will be a macro under its printed name
//! (`Islandwalk`, `Ward`, `Mountaincycling`) that expands to its underlying
//! keyword ability, so the on-card name *is* the invocation.
//!
//! The exception is the seven NATIVE keywords the grammar carries as
//! `KeywordAbility` variants ([CR#702]): those render wrapped —
//! `Keyword(Deathtouch)` — because the variant name IS the printed name and
//! the engine implements them natively; while they stay enum variants, no
//! macro exists for them.

use crate::ident::to_rust_ident;
use crate::parsers::cost::VariableMana;
use crate::parsers::cost::{self};
use crate::resolve::CardKind;

/// Keyword-ability names ([CR#702] / the Scryfall `keyword-abilities` catalog —
/// the same source `_000` builds keyword todos from). Longest-prefix matching
/// needs the multi-word names present verbatim. Order is irrelevant (matching
/// picks the longest prefix).
const KEYWORD_NAMES: &[&str] = &[
    "Living weapon",
    "Jump-start",
    "Commander ninjutsu",
    "Legendary landwalk",
    "Nonbasic landwalk",
    "Megamorph",
    "Haunt",
    "Forecast",
    "Graft",
    "Fortify",
    "Frenzy",
    "Gravestorm",
    "Hideaway",
    "Level Up",
    "Infect",
    "Reach",
    "Rampage",
    "Phasing",
    "Multikicker",
    "Morph",
    "Provoke",
    "Modular",
    "Ninjutsu",
    "Replicate",
    "Recover",
    "Poisonous",
    "Reinforce",
    "Persist",
    "Retrace",
    "Rebound",
    "Miracle",
    "Overload",
    "Outlast",
    "Prowess",
    "Renown",
    "Myriad",
    "Shroud",
    "Trample",
    "Vigilance",
    "Storm",
    "Soulshift",
    "Splice",
    "Transmute",
    "Ripple",
    "Suspend",
    "Vanishing",
    "Transfigure",
    "Wither",
    "Undying",
    "Soulbond",
    "Unleash",
    "Ascend",
    "Assist",
    "Afterlife",
    "Companion",
    "Fabricate",
    "Embalm",
    "Escape",
    "Fuse",
    "Menace",
    "Ingest",
    "Melee",
    "Improvise",
    "Mentor",
    "Partner",
    "Mutate",
    "Tribute",
    "Surge",
    "Skulk",
    "Riot",
    "Spectacle",
    "Forestwalk",
    "Islandwalk",
    "Mountainwalk",
    "Double strike",
    "Cumulative upkeep",
    "First strike",
    "Scavenge",
    "Encore",
    "Deathtouch",
    "Defender",
    "Amplify",
    "Affinity",
    "Bushido",
    "Convoke",
    "Bloodthirst",
    "Absorb",
    "Aura Swap",
    "Changeling",
    "Conspire",
    "Cascade",
    "Annihilator",
    "Battle Cry",
    "Cipher",
    "Bestow",
    "Dash",
    "Awaken",
    "Crew",
    "Aftermath",
    "Afflict",
    "Flanking",
    "Foretell",
    "Fading",
    "Eternalize",
    "Entwine",
    "Epic",
    "Dredge",
    "Delve",
    "Evoke",
    "Exalted",
    "Evolve",
    "Extort",
    "Dethrone",
    "Exploit",
    "Devoid",
    "Emerge",
    "Escalate",
    "Flying",
    "Haste",
    "Hexproof",
    "Indestructible",
    "Intimidate",
    "Lifelink",
    "Horsemanship",
    "Kicker",
    "Madness",
    "Swampwalk",
    "Desertwalk",
    "Craft",
    "Plainswalk",
    "Split second",
    "Augment",
    "Double agenda",
    "Reconfigure",
    "Ward",
    "Partner with",
    "Daybound",
    "Nightbound",
    "Decayed",
    "Disturb",
    "Squad",
    "Enlist",
    "Read Ahead",
    "Ravenous",
    "Blitz",
    "Offering",
    "Living metal",
    "Backup",
    "Banding",
    "Hidden agenda",
    "For Mirrodin!",
    "Friends forever",
    "Casualty",
    "Protection",
    "Compleated",
    "Enchant",
    "Flash",
    "Boast",
    "Demonstrate",
    "Sunburst",
    "Flashback",
    "Cycling",
    "Equip",
    "Buyback",
    "Hexproof from",
    "More Than Meets the Eye",
    "Cleave",
    "Champion",
    "Specialize",
    "Training",
    "Prototype",
    "Toxic",
    "Unearth",
    "Intensity",
    "Plainscycling",
    "Swampcycling",
    "Typecycling",
    "Wizardcycling",
    "Mountaincycling",
    "Basic landcycling",
    "Islandcycling",
    "Forestcycling",
    "Slivercycling",
    "Landcycling",
    "Bargain",
    "Choose a background",
    "Echo",
    "Disguise",
    "Doctor's companion",
    "Landwalk",
    "Umbra armor",
    "Freerunning",
    "Spree",
    "Saddle",
    "Shadow",
    "Warp",
    "Station",
    "Devour",
    "Undaunted",
    "Offspring",
];

/// One keyword token -> its invocation RON, always wrapped —
/// `Keyword(Flying)`, `Keyword(Ward([Mana([Generic(2)])]))` — or `None`
/// (declines). Intrinsic enum variants and `KeywordAbility`-kind macros
/// share the wrapper: card definitions always call out keyword-ness
/// explicitly, and non-intrinsic names resolve (or stay todo) inside the
/// `KeywordAbility` position's macro namespace.
fn bare_keyword(token: &str) -> anyhow::Result<Option<String>> {
    let Some(name) = match_keyword_prefix(token) else {
        return Ok(None);
    };
    let arg = token[name.len()..].trim();
    let ident = to_rust_ident(name);
    let Some(invocation) = render_arg(&ident, arg)? else {
        eprintln!("keyword_ability: unhandled keyword {name:?} (arg {arg:?})");
        return Ok(None);
    };
    Ok(Some(format!("Keyword({invocation})")))
}

/// A registry parser: one keyword-ability line -> the bare invocation RON, or
/// `None`. The input is expected to be a single, already-trimmed keyword line
/// as `extract` guarantees — comma-joined keyword lines are pre-split before
/// reaching the registry. A line that still CHAINS keywords on `", "` (every
/// piece after the first starts a keyword name) declines; other commas are
/// argument text ("Ward—{2}, Pay 2 life.") and parse as one keyword.
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    let chained: Vec<&str> = line.split(", ").skip(1).collect();
    if !chained.is_empty()
        && chained
            .iter()
            .all(|piece| match_keyword_prefix(piece).is_some())
    {
        return Ok(None);
    }
    bare_keyword(line.trim())
}

/// The longest `KEYWORD_NAMES` entry that prefixes `token` (case-insensitive)
/// at a word boundary (followed by a space, em-dash, or end).
fn match_keyword_prefix(token: &str) -> Option<&'static str> {
    let lower = token.to_ascii_lowercase();
    KEYWORD_NAMES
        .iter()
        .copied()
        .filter(|name| {
            lower
                .strip_prefix(&name.to_ascii_lowercase())
                .is_some_and(|rest| {
                    rest.is_empty() || rest.starts_with(' ') || rest.starts_with('—')
                })
        })
        .max_by_key(|name| name.len())
}

/// Whole-phrase, case-insensitive match against the keyword catalog → the
/// `Keyword(...)` ident form. Used by the static-ability grant family
/// ("… have flying"). A phrase with leftover (e.g. "protection from red", a
/// parameterized keyword) does NOT match here — the grant family declines it.
pub(crate) fn match_keyword_name(phrase: &str) -> Option<String> {
    let phrase = phrase.trim();
    KEYWORD_NAMES
        .iter()
        .find(|name| name.eq_ignore_ascii_case(phrase))
        .map(|name| to_rust_ident(name))
}

/// Argument-shape render. `None` declines (the card stays a todo).
fn render_arg(ident: &str, arg: &str) -> anyhow::Result<Option<String>> {
    // Keyword-specific word-argument shapes first — the parser owns the
    // authored macros' spelling conventions (a quality word renders as the
    // Filter the macro's param expects).
    match ident {
        // The macro's `from` param is defaulted, but an all-defaulted
        // invocation still needs its parens — bare `Hexproof` doesn't read.
        "Hexproof" if arg.is_empty() => return Ok(Some("Hexproof()".to_owned())),
        // "Hexproof from [quality]" is the same macro, param supplied.
        "HexproofFrom" => {
            return Ok(quality_filter(arg).map(|q| format!("Hexproof(from: {q})")));
        }
        "Protection" => {
            let Some(q) = arg.strip_prefix("from ") else {
                return Ok(None);
            };
            // "from everything" = protection regardless of qualities
            // [CR#702.16j]: the match-all Filter in every row of the bundle.
            if q == "everything" {
                return Ok(Some("Protection(Any)".to_owned()));
            }
            return Ok(quality_filter(q).map(|q| format!("Protection({q})")));
        }
        "Affinity" => {
            let Some(q) = arg.strip_prefix("for ") else {
                return Ok(None);
            };
            return Ok(quality_filter(q).map(|q| format!("Affinity({q})")));
        }
        "Enchant" => return Ok(quality_filter(arg).map(|q| format!("Enchant({q})"))),
        _ => {}
    }
    if arg.is_empty() {
        return Ok(Some(ident.to_owned()));
    }
    // A bare mana run ("Ward {2}") or an em-dash cost ("Ward—Pay 3 life.",
    // "Ward—{2}, Pay 2 life." [CR#702.21a]) — the shared cost grammar.
    if arg.starts_with('{') || arg.starts_with('—') {
        let clause = arg.strip_prefix('—').unwrap_or(arg);
        return Ok(cost_arg(clause)?.map(|cost| format!("{ident}({cost})")));
    }
    let (num, cost) = match arg.split_once('—') {
        Some((n, c)) => (n.trim(), Some(c.trim())),
        None => (arg, None),
    };
    let Ok(n) = num.parse::<u32>() else {
        return Ok(None);
    };
    match cost {
        None => Ok(Some(format!("{ident}({n})"))),
        Some(cost) => Ok(cost_arg(cost)?.map(|cost| format!("{ident}({n}, {cost})"))),
    }
}

/// A keyword cost argument -> its rendered cost list (`[Mana([Generic(2)])]`,
/// `[Do(LoseLife(3))]`, …): the shared cost grammar over the ", "-separated
/// clause. The worded form's trailing period is stripped; `{X}` is allowed
/// (the printed cost carries it — what X equals is announced by the
/// controller or stated by the card, [CR#107.3a,702.21b]).
fn cost_arg(text: &str) -> anyhow::Result<Option<String>> {
    let trimmed = text.trim();
    let clause = trimmed.strip_suffix('.').unwrap_or(trimmed);
    let Some(components) = cost::parse_cost(clause, VariableMana::Allow)? else {
        return Ok(None);
    };
    Ok(Some(format!("[{}]", components.join(", "))))
}

/// A single quality word -> its `Filter` RON: the five colors, or a simple
/// type noun (plural tolerated). `None` declines — compound qualities
/// ("artifact creatures", "monocolored") stay todo. "From everything" is the
/// `Protection` arm's special case, and multi-quality lines ("from red and
/// from white") are expanded into one line per quality by `extract`
/// [CR#702.16g] before the registry sees them.
fn quality_filter(q: &str) -> Option<String> {
    let q = q.trim();
    if q.is_empty() || q.contains(' ') {
        return None;
    }
    if let Some(c) = super::filter::color_ident(q) {
        return Some(format!("ColorIs({c})"));
    }
    // Share the type-noun vocabulary + singularizer with the filter-head
    // parser; `quality_filter`'s divergent wrapper is the always-`Type(<T>)`
    // form (and it has no `permanent`, which `type_filter` declines).
    super::filter::type_filter(&super::filter::singularize(q).to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bare(token: &str) -> Option<String> {
        bare_keyword(token).unwrap()
    }

    #[test]
    fn intrinsic_keywords_render_wrapped() {
        assert_eq!(bare("Vigilance").as_deref(), Some("Keyword(Vigilance)"));
        assert_eq!(
            bare("First strike").as_deref(),
            Some("Keyword(FirstStrike)")
        );
        assert_eq!(bare("Deathtouch").as_deref(), Some("Keyword(Deathtouch)"));
    }

    #[test]
    fn macro_keywords_render_wrapped_too() {
        // Non-intrinsics are KeywordAbility-kind macros invoked INSIDE the
        // wrapper — keyword-ness is always explicit on the card.
        assert_eq!(bare("Flying").as_deref(), Some("Keyword(Flying)"));
        assert_eq!(bare("Lifelink").as_deref(), Some("Keyword(Lifelink)"));
    }

    #[test]
    fn nullary_keywords() {
        assert_eq!(bare("Menace").as_deref(), Some("Keyword(Menace)"));
        assert_eq!(bare("Defender").as_deref(), Some("Keyword(Defender)"));
    }

    #[test]
    fn cost_keywords() {
        assert_eq!(
            bare("Ward {2}").as_deref(),
            Some("Keyword(Ward([Mana([Generic(2)])]))")
        );
        assert_eq!(
            bare("Equip {3}").as_deref(),
            Some("Keyword(Equip([Mana([Generic(3)])]))")
        );
    }

    #[test]
    fn integer_and_integer_cost() {
        assert_eq!(
            bare("Annihilator 2").as_deref(),
            Some("Keyword(Annihilator(2))")
        );
        assert_eq!(
            bare("Suspend 4—{1}{R}").as_deref(),
            Some("Keyword(Suspend(4, [Mana([Generic(1),Red])]))")
        );
    }

    #[test]
    fn on_card_name_is_the_invocation() {
        // Landwalk / typecycling render as their printed name (a future macro),
        // not an unrolled Landwalk(...) / Typecycling(...).
        assert_eq!(bare("Islandwalk").as_deref(), Some("Keyword(Islandwalk)"));
        assert_eq!(
            bare("Legendary landwalk").as_deref(),
            Some("Keyword(LegendaryLandwalk)")
        );
        assert_eq!(
            bare("Mountaincycling {2}").as_deref(),
            Some("Keyword(Mountaincycling([Mana([Generic(2)])]))")
        );
    }

    #[test]
    fn declines_difficult_and_unknown() {
        assert!(bare("Annihilator X").is_none()); // variable integer
        assert!(bare("Enchant artifact creature").is_none()); // compound quality
        assert!(bare("Whenever this dies, draw a card").is_none()); // not a keyword
        // Or-costs and cost riders aren't productions.
        assert!(bare("Equip—Pay {3} or discard a card.").is_none());
        assert!(bare("Ward—Discard a card at random.").is_none());
        // The joint multi-quality line declines here — extract expands it
        // into one line per quality before the registry sees it.
        assert!(bare("Protection from black and from red").is_none());
    }

    #[test]
    fn word_costs_after_the_em_dash() {
        assert_eq!(
            bare("Ward—Pay 3 life.").as_deref(),
            Some("Keyword(Ward([Do(LoseLife(3))]))")
        );
        assert_eq!(
            bare("Cycling—Discard a card.").as_deref(),
            Some("Keyword(Cycling([Do(Discard(1))]))")
        );
        // A chosen-sacrifice cost after the em dash (the filter grammar now
        // resolves the subject) — e.g. Ward—Sacrifice a creature.
        assert_eq!(
            bare("Ward—Sacrifice a creature.").as_deref(),
            Some("Keyword(Ward([Do(Sacrifice(Choose(Exactly(Literal(1)), Creature)))]))")
        );
        assert_eq!(
            bare("Equip—Discard a card.").as_deref(),
            Some("Keyword(Equip([Do(Discard(1))]))")
        );
        // A comma-separated cost list mixes mana and word costs.
        assert_eq!(
            bare("Ward—{2}, Pay 2 life.").as_deref(),
            Some("Keyword(Ward([Mana([Generic(2)]), Do(LoseLife(2))]))")
        );
    }

    #[test]
    fn variable_mana_costs() {
        // {X} is part of the printed cost; what X equals is announced by the
        // controller or stated by the card [CR#107.3a,702.21b], not the
        // parser's business.
        assert_eq!(
            bare("Ward {X}").as_deref(),
            Some("Keyword(Ward([Mana([Variable])]))")
        );
        assert_eq!(
            bare("Cycling {X}{1}{U}").as_deref(),
            Some("Keyword(Cycling([Mana([Variable,Generic(1),Blue])]))")
        );
    }

    #[test]
    fn protection_from_everything() {
        // [CR#702.16j]: protection regardless of qualities — the match-all
        // Filter in every row of the Protection bundle.
        assert_eq!(
            bare("Protection from everything").as_deref(),
            Some("Keyword(Protection(Any))")
        );
    }

    #[test]
    fn resolve_line_comma_gate() {
        use crate::resolve::CardKind;
        // A keyword CHAIN still declines (extract pre-splits those; a line
        // that reaches the registry chained is stale input).
        assert!(
            resolve_line("First strike, vigilance", CardKind::Permanent)
                .unwrap()
                .is_none()
        );
        // But a comma inside a cost list is argument text, not a chain.
        assert_eq!(
            resolve_line("Ward—{2}, Pay 2 life.", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Keyword(Ward([Mana([Generic(2)]), Do(LoseLife(2))]))")
        );
    }

    #[test]
    fn word_arg_keywords_render_quality_filters() {
        assert_eq!(
            bare("Protection from black").as_deref(),
            Some("Keyword(Protection(ColorIs(Black)))")
        );
        assert_eq!(
            bare("Enchant creature").as_deref(),
            Some("Keyword(Enchant(Type(Creature)))")
        );
        assert_eq!(
            bare("Affinity for artifacts").as_deref(),
            Some("Keyword(Affinity(Type(Artifact)))")
        );
        // Bare hexproof keeps its parens (all-defaulted invocation); the
        // from-variant supplies the named param on the SAME macro.
        assert_eq!(bare("Hexproof").as_deref(), Some("Keyword(Hexproof())"));
        assert_eq!(
            bare("Hexproof from blue").as_deref(),
            Some("Keyword(Hexproof(from: ColorIs(Blue)))")
        );
    }

    #[test]
    fn resolve_line_bare_keyword() {
        use crate::resolve::CardKind;
        assert_eq!(
            resolve_line("Flying", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Keyword(Flying)")
        );
        assert_eq!(
            resolve_line("Ward {2}", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Keyword(Ward([Mana([Generic(2)])]))")
        );
        assert_eq!(
            resolve_line("Protection from black", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Keyword(Protection(ColorIs(Black)))")
        );
    }
}
