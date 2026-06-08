//! Recognizes a normalized keyword-ability line and renders it as the
//! anticipated macro invocation, by its on-card name: the matched keyword name
//! is the macro head and the argument shape decides arity (nullary / cost /
//! integer / integer+cost). Each keyword will be a macro under its printed name
//! (`Islandwalk`, `Ward`, `Mountaincycling`) that expands to its underlying
//! keyword ability, so the on-card name *is* the invocation. Keyword macros
//! don't exist yet, so the output is a parked draft (see the `.ron.pending`
//! state).

use deckmaste_core::{ManaCost, ManaSymbol};

use super::to_rust_ident;
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

/// Renders a keyword line as one or more ability blocks (each 8-space indent +
/// trailing comma+newline), or `None` if any token isn't a handled keyword (the
/// whole card then declines). Unhandled keywords are logged so the un-handled
/// corpus stays visible.
pub(super) fn render_keyword_line(line: &str) -> anyhow::Result<Option<String>> {
    let mut blocks = String::new();
    for token in line.split(", ") {
        let Some(block) = render_keyword(token.trim())? else {
            return Ok(None);
        };
        blocks.push_str(&block);
    }
    Ok(Some(blocks))
}

/// One keyword token -> its bare invocation RON (`Flying`,
/// `Ward([Mana([Generic(2)])])`), or `None` (declines). The name-match +
/// argument-shape logic; the legacy 8-space block form wraps this.
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
    Ok(Some(invocation))
}

/// One keyword token -> its ability block (8-space indent + trailing comma+
/// newline), or `None`. Wraps [`bare_keyword`] for the legacy migrations.
fn render_keyword(token: &str) -> anyhow::Result<Option<String>> {
    Ok(bare_keyword(token)?.map(|invocation| format!("        {invocation},\n")))
}

/// A registry parser: one keyword-ability line -> the bare invocation RON, or
/// `None`. The input is expected to be a single, already-trimmed keyword line
/// as `extract` guarantees — comma-joined keyword lines are pre-split before
/// reaching the registry. A line that still chains keywords on `", "` declines.
pub(crate) fn resolve_line(line: &str) -> anyhow::Result<Option<String>> {
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

    fn render(line: &str) -> Option<String> { render_keyword_line(line).unwrap() }

    #[test]
    fn nullary_and_chained() {
        assert_eq!(render("Flying").unwrap(), "        Flying,\n");
        assert_eq!(
            render("Flying, vigilance").unwrap(),
            "        Flying,\n        Vigilance,\n"
        );
        assert_eq!(render("First strike").unwrap(), "        FirstStrike,\n");
    }

    #[test]
    fn cost_keywords() {
        assert_eq!(
            render("Ward {2}").unwrap(),
            "        Ward([Mana([Generic(2)])]),\n"
        );
        assert_eq!(
            render("Ward {1}, haste").unwrap(),
            "        Ward([Mana([Generic(1)])]),\n        Haste,\n"
        );
        assert_eq!(
            render("Equip {3}").unwrap(),
            "        Equip([Mana([Generic(3)])]),\n"
        );
    }

    #[test]
    fn integer_and_integer_cost() {
        assert_eq!(
            render("Annihilator 2").unwrap(),
            "        Annihilator(2),\n"
        );
        assert_eq!(
            render("Suspend 4—{1}{R}").unwrap(),
            "        Suspend(4, [Mana([Generic(1),Red])]),\n"
        );
    }

    #[test]
    fn on_card_name_is_the_invocation() {
        // Landwalk / typecycling render as their printed name (a future macro),
        // not an unrolled Landwalk(...) / Typecycling(...).
        assert_eq!(render("Islandwalk").unwrap(), "        Islandwalk,\n");
        assert_eq!(
            render("Legendary landwalk").unwrap(),
            "        LegendaryLandwalk,\n"
        );
        assert_eq!(
            render("Mountaincycling {2}").unwrap(),
            "        Mountaincycling([Mana([Generic(2)])]),\n"
        );
    }

    #[test]
    fn declines_variable_difficult_and_unknown() {
        assert!(render("Annihilator X").is_none()); // variable integer
        assert!(render("Ward {X}").is_none()); // variable mana cost
        assert!(render("Protection from black").is_none()); // word arg
        assert!(render("Enchant creature").is_none());
        assert!(render("Cycling—Discard a card").is_none()); // non-mana em-dash cost
        assert!(render("Whenever this dies, draw a card").is_none()); // not a keyword
    }

    #[test]
    fn resolve_line_bare_keyword() {
        assert_eq!(resolve_line("Flying").unwrap().as_deref(), Some("Flying"));
        assert_eq!(
            resolve_line("Ward {2}").unwrap().as_deref(),
            Some("Ward([Mana([Generic(2)])])")
        );
        assert!(resolve_line("Protection from black").unwrap().is_none());
    }
}
