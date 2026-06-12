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

use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;

use crate::ident::to_rust_ident;
use crate::resolve::CardKind;
use crate::ron_output::ron_options;

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
    let Some(name) = match_keyword_name(token) else {
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
/// reaching the registry. A line that still chains keywords on `", "` declines.
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    if line.split(", ").count() != 1 {
        return Ok(None);
    }
    bare_keyword(line.trim())
}

/// The longest `KEYWORD_NAMES` entry that prefixes `token` (case-insensitive)
/// at a word boundary (followed by a space, em-dash, or end).
fn match_keyword_name(token: &str) -> Option<&'static str> {
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
    if arg.starts_with('{') || arg.starts_with('—') {
        return Ok(mana_cost_arg(arg)?.map(|cost| format!("{ident}({cost})")));
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
        Some(cost) => Ok(mana_cost_arg(cost)?.map(|cost| format!("{ident}({n}, {cost})"))),
    }
}

/// A single quality word -> its `Filter` RON: the five colors, or a simple
/// type noun (plural tolerated). `None` declines — compound qualities
/// ("from everything", "artifact creatures", "from red and from white")
/// stay todo.
fn quality_filter(q: &str) -> Option<String> {
    let q = q.trim();
    if q.is_empty() || q.contains(' ') {
        return None;
    }
    let color = match q {
        "white" => Some("White"),
        "blue" => Some("Blue"),
        "black" => Some("Black"),
        "red" => Some("Red"),
        "green" => Some("Green"),
        _ => None,
    };
    if let Some(c) = color {
        return Some(format!("ColorIs({c})"));
    }
    if matches!(q, "sorcery" | "sorceries") {
        return Some("Type(Sorcery)".to_owned());
    }
    let ty = match q.strip_suffix('s').unwrap_or(q) {
        "creature" => "Creature",
        "artifact" => "Artifact",
        "enchantment" => "Enchantment",
        "land" => "Land",
        "planeswalker" => "Planeswalker",
        "instant" => "Instant",
        "battle" => "Battle",
        _ => return None,
    };
    Some(format!("Type({ty})"))
}

/// A pure-mana cost argument (`{2}` or a leading-em-dash `—{2}`) ->
/// `[Mana([Generic(2)])]`. `None` for a non-mana cost (em-dash word costs) or
/// an empty cost.
fn mana_cost_arg(arg: &str) -> anyhow::Result<Option<String>> {
    let cost = arg.strip_prefix('—').unwrap_or(arg).trim();
    let Ok(mana) = cost.parse::<ManaCost>() else {
        return Ok(None);
    };
    if mana.is_empty() || mana.iter().any(|s| matches!(s, ManaSymbol::Variable)) {
        return Ok(None);
    }
    Ok(Some(format!("[Mana({})]", ron_options().to_string(&mana)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bare(token: &str) -> Option<String> { bare_keyword(token).unwrap() }

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
    fn declines_variable_difficult_and_unknown() {
        assert!(bare("Annihilator X").is_none()); // variable integer
        assert!(bare("Ward {X}").is_none()); // variable mana cost
        assert!(bare("Protection from everything").is_none()); // unknown quality
        assert!(bare("Enchant artifact creature").is_none()); // compound quality
        assert!(bare("Cycling—Discard a card").is_none()); // non-mana em-dash cost
        assert!(bare("Whenever this dies, draw a card").is_none()); // not a keyword
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
