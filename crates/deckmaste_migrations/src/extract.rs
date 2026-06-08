//! `extract` — emit every supported card as a Card-shaped `<name>.ron.todo`
//! (a `TodoCard`) whose abilities are `Unparsed` oracle lines. Owns the oracle
//! normalization helpers (`strip_reminder_text`, `expand_keyword_lines`,
//! `self_ref_to_tilde`) and mtgjson field accessors. Covers the `normal` and
//! `modal_dfc` layouts (the two core `Card` variants); other layouts are
//! skipped until core `Card` grows variants for them.

use std::path::Path;
use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::plugin::card_file;
use deckmaste_core::{Color, StatValue};
use regex::Regex;

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::ident::to_rust_ident;
use crate::todo_card::{RawIdent, TodoAbility, TodoCard, TodoCardFace, render};

// We count non-null, non-"Banned" as legal.
fn is_supported(card: &AtomicCard) -> bool {
    card.legalities.vintage.as_deref().unwrap_or("Banned") != "Banned"
        && card.layout.as_str() != "reversible_card"
}

/// Uppercases the first character (ASCII only, like jq's `ascii_upcase`).
fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Removes single-line parentheticals (reminder text), keeping at most one
/// of the surrounding spaces. Lines that consisted solely of reminder text
/// are dropped entirely.
fn strip_reminder_text(text: &str) -> String {
    static PARENTHETICAL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r" ?\([^)\n]+\)( ?)").unwrap());

    text.split('\n')
        .filter_map(|line| {
            let stripped = PARENTHETICAL.replace_all(line, "$1");
            (!stripped.is_empty() || line.is_empty()).then(|| stripped.into_owned())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `capitalize(item).starts_with(keyword)` without building the string.
fn starts_with_capitalized(item: &str, keyword: &str) -> bool {
    let mut item_chars = item.chars();
    let mut keyword_chars = keyword.chars();
    match (item_chars.next(), keyword_chars.next()) {
        (_, None) => true,
        (None, Some(_)) => false,
        (Some(i), Some(k)) => {
            i.to_ascii_uppercase() == k && item_chars.as_str().starts_with(keyword_chars.as_str())
        }
    }
}

/// Splits lines that are comma-separated lists of keyword abilities into one
/// keyword per line, e.g. "Flying, vigilance" -> "Flying\nVigilance".
/// Most lines aren't keyword lists, so nothing is allocated until one is.
fn expand_keyword_lines(text: &str, keyword_abilities: &[DataStr<'_>]) -> String {
    let is_keyword = |item: &str| {
        keyword_abilities
            .iter()
            .any(|keyword| starts_with_capitalized(item, keyword))
    };
    text.split('\n')
        .flat_map(|line| {
            if line.split(", ").all(is_keyword) {
                line.split(", ").map(capitalize).collect()
            } else {
                vec![line.to_owned()]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Replaces every whole-word, case-sensitive occurrence of `name` with `~`.
/// The substring pre-check skips building a regex for the common case of a
/// card that never mentions the name at all.
fn replace_whole_word(text: &str, name: &str) -> String {
    if name.is_empty() || !text.contains(name) {
        return text.to_owned();
    }
    let pattern = format!(r"\b{}\b", regex::escape(name));
    // Always valid: an escaped literal wrapped in word boundaries.
    let re = Regex::new(&pattern).expect("escaped name is a valid regex");
    re.replace_all(text, "~").into_owned()
}

/// Replaces a card's references to *itself* with the `~` self-reference sigil.
///
/// Replaces every whole-word, case-sensitive occurrence of `face_name`. For a
/// legendary face whose name has a comma, the pre-comma short name (e.g.
/// "Boromir" from "Boromir, Gondor's Hope") is also collapsed -- unless it is
/// a keyword ability ("Storm") or shorter than three characters ("Me"), both
/// too collision-prone to replace blindly.
fn self_ref_to_tilde(
    text: &str,
    face_name: &str,
    is_legendary: bool,
    keyword_abilities: &[DataStr<'_>],
) -> String {
    let out = replace_whole_word(text, face_name);

    let short = is_legendary
        .then(|| face_name.split_once(',').map(|(short, _)| short.trim()))
        .flatten()
        .filter(|short| short.chars().count() >= 3)
        .filter(|short| {
            !keyword_abilities
                .iter()
                .any(|kw| kw.as_str().eq_ignore_ascii_case(short))
        });

    let named = match short {
        Some(short) => replace_whole_word(&out, short),
        None => out,
    };
    this_self_ref_to_tilde(&named)
}

/// Collapses generic "this `<noun>`" / "This `<noun>`" self-references to `~`,
/// independent of the card's name. `~` denotes the object the text belongs to
/// -- the card itself, or, inside a granted (quoted) ability, the token gaining
/// it -- so this is a blind whole-phrase substitution: `WotC` templating
/// reserves "this X" for the source object (any other object is
/// "that"/"it"/"the X"), so one line may legitimately yield two `~`s (a card
/// and a token it makes).
///
/// The noun set is fixed and **case-sensitive**: lowercase card-type and
/// generic nouns, plus the capitalized permanent-subtype nouns that occur in
/// the corpus. Case-sensitivity is load-bearing -- it keeps lowercase English
/// ("in this case", "this way") from matching the subtype `Case`. Nouns that
/// never denote the object stay literal (turn, way, mana, ability, combat,
/// effect, phase, step, game).
///
/// `door` is deliberately left out for now: very borderline -- "this door" is
/// one half of a Room, not the whole card, and a lowercase common word risks
/// collisions. (Capitalized `Room` is included -- it self-refs the whole card.)
fn this_self_ref_to_tilde(text: &str) -> String {
    static THIS_NOUN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"\b[Tt]his (creature|land|artifact|enchantment|planeswalker|battle|permanent|spell|card|token|Aura|Equipment|Vehicle|Saga|Siege|Class|Spacecraft|Case|Room)\b",
        )
        .expect("self-reference noun pattern is valid")
    });
    THIS_NOUN.replace_all(text, "~").into_owned()
}

fn ron_color(code: &str) -> anyhow::Result<Color> {
    Color::from_code(code).ok_or_else(|| anyhow::anyhow!("unrecognized color indicator: {code:?}"))
}

/// One mtgjson type/subtype/supertype name → a bare-ident `RawIdent`
/// (`"Time Lord"` → `TimeLord`), matching the macro-invocation name the
/// macro-aware reader expands at graduation.
fn ident(name: &str) -> RawIdent { RawIdent(to_rust_ident(name)) }

/// mtgjson stat string → core `StatValue`: integers (incl. negative) are
/// `Number`; `X` is `Variable`; anything else (`*`, `1+*`) is
/// `DefinedByAbility`.
fn stat_value(text: &str) -> StatValue {
    if let Ok(n) = text.parse::<deckmaste_core::Int>() {
        StatValue::Number(n)
    } else if text == "X" {
        // `X` is loyalty defined by the casting cost (the only place `X` stats
        // appear on standard-legal cards); `*`/`1+*` take the DefinedByAbility arm.
        StatValue::Variable
    } else {
        StatValue::DefinedByAbility
    }
}

/// Builds a `TodoCardFace` from one mtgjson face: structured fields plus one
/// `Unparsed` ability per normalized oracle line (strip reminder text, split
/// comma-joined keyword lines, `~` self-refs).
///
/// # Errors
/// If the mana cost or a color indicator fails to parse.
fn face(card: &AtomicCard, keyword_abilities: &[DataStr<'_>]) -> anyhow::Result<TodoCardFace> {
    let face_name = card.face_name.as_deref().unwrap_or(card.name.as_str());
    let is_legendary = card.supertypes.iter().any(|t| t.as_str() == "Legendary");
    let abilities = card.text.as_deref().map_or_else(Vec::new, |text| {
        let text = crate::data::academyruins::normalize_quotes(text);
        let text = expand_keyword_lines(&strip_reminder_text(&text), keyword_abilities);
        let text = self_ref_to_tilde(&text, face_name, is_legendary, keyword_abilities);
        text.split('\n')
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| TodoAbility::Unparsed(line.to_owned()))
            .collect()
    });
    Ok(TodoCardFace {
        name: face_name.to_owned(),
        mana_cost: card
            .mana_cost
            .as_deref()
            .map(str::parse)
            .transpose()?
            .unwrap_or_default(),
        color_indicator: card
            .color_indicator
            .iter()
            .map(|c| ron_color(c.as_str()))
            .collect::<anyhow::Result<_>>()?,
        supertypes: card.supertypes.iter().map(|t| ident(t.as_str())).collect(),
        types: card.types.iter().map(|t| ident(t.as_str())).collect(),
        subtypes: card.subtypes.iter().map(|t| ident(t.as_str())).collect(),
        abilities,
        power: card.power.as_deref().map(stat_value),
        toughness: card.toughness.as_deref().map(stat_value),
        loyalty: card.loyalty.as_deref().map(stat_value),
        defense: card.defense.as_deref().map(stat_value),
    })
}

/// The `TodoCard` for a card's supported faces, or `None` if its layout isn't a
/// core `Card` variant (only `normal` / `modal_dfc`).
///
/// # Errors
/// If a face fails to build (see [`face`]).
fn todo_card(
    layout: &str,
    faces: &[&AtomicCard],
    keyword_abilities: &[DataStr<'_>],
) -> anyhow::Result<Option<TodoCard>> {
    Ok(match (layout, faces) {
        ("normal", [f]) => Some(TodoCard::Normal(face(f, keyword_abilities)?)),
        ("modal_dfc", [front, back]) => Some(TodoCard::ModalDfc(
            face(front, keyword_abilities)?,
            face(back, keyword_abilities)?,
        )),
        _ => None,
    })
}

/// Writes a `<name>.ron.todo` for every supported card that isn't already
/// finished (`<name>.ron`) or in progress (`<name>.ron.todo`).
///
/// # Errors
/// If the mtgjson/keyword data is unreadable or a card fails to render.
pub fn extract_cards(plugin_dir: &Path) -> anyhow::Result<()> {
    let layout = crate::layout::PluginLayout::new(plugin_dir)?;
    let cards_dir = layout.cards_dir()?;
    let atomic_bytes = crate::data::mtgjson::atomic_cards_bytes()?;
    let atomic = crate::data::mtgjson::AtomicCards::parse(&atomic_bytes)?;
    let keywords_bytes = crate::data::academyruins::keywords_bytes()?;
    let keyword_abilities =
        crate::data::academyruins::Keywords::parse(&keywords_bytes)?.keyword_abilities;

    for (name, all_faces) in &atomic.data {
        let supported: Vec<&AtomicCard> = all_faces.iter().filter(|c| is_supported(c)).collect();
        if supported.is_empty() {
            continue;
        }
        let final_path = cards_dir.join(card_file(name.as_str()));
        let todo_path = cards_dir.join(format!("{}.todo", card_file(name.as_str())));
        if final_path.exists() || todo_path.exists() {
            continue; // already finished or already in progress
        }
        let Some(card) = todo_card(supported[0].layout.as_str(), &supported, &keyword_abilities)?
        else {
            continue; // unsupported layout
        };
        std::fs::write(&todo_path, render(&card)?)
            .with_context(|| format!("writing {}", todo_path.display()))?;
        eprintln!("wrote {}", todo_path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::mtgjson::Legalities;

    /// A minimal normal-layout creature fixture; `text` is the oracle text.
    fn creature(text: Option<&'static str>) -> AtomicCard<'static> {
        AtomicCard {
            name: "Test Bear".into(),
            face_name: None,
            mana_cost: None,
            color_indicator: vec![],
            types: vec!["Creature".into()],
            supertypes: vec![],
            subtypes: vec!["Bear".into()],
            text: text.map(Into::into),
            power: Some("2".into()),
            toughness: Some("2".into()),
            loyalty: None,
            defense: None,
            layout: "normal".into(),
            legalities: Legalities::default(),
        }
    }

    /// `stat_value` is the only branchy conversion; pin every arm.
    #[test]
    fn stat_value_branches() {
        assert_eq!(stat_value("2"), StatValue::Number(2));
        assert_eq!(stat_value("-1"), StatValue::Number(-1));
        assert_eq!(stat_value("*"), StatValue::DefinedByAbility);
        assert_eq!(stat_value("1+*"), StatValue::DefinedByAbility);
        assert_eq!(stat_value("X"), StatValue::Variable);
    }

    /// A bare-`normal` creature with one keyword line: structured fields become
    /// `RawIdent`s, the oracle line becomes a single `Unparsed` ability.
    #[test]
    fn normal_creature_builds() {
        let card = creature(Some("Flying"));

        let todo = todo_card("normal", &[&card], &[]).unwrap().unwrap();
        let TodoCard::Normal(face) = &todo else {
            panic!("expected Normal");
        };
        assert_eq!(face.types, [RawIdent("Creature".into())]);
        assert_eq!(face.subtypes, [RawIdent("Bear".into())]);
        assert_eq!(face.power, Some(StatValue::Number(2)));
        assert_eq!(face.toughness, Some(StatValue::Number(2)));
        // Empty keyword list: "Flying" stays one non-keyword line -> one ability.
        assert!(
            matches!(&face.abilities[..], [TodoAbility::Unparsed(s)] if s == "Flying"),
            "abilities = {:?}",
            face.abilities
        );

        // Render round-trip: the on-disk house style carries the bare subtype
        // ident and the Unparsed oracle line.
        let rendered = render(&todo).unwrap();
        assert!(rendered.contains("subtypes: [Bear]"), "{rendered}");
        assert!(rendered.contains(r#"Unparsed("Flying")"#), "{rendered}");
    }

    /// `text: None` → no oracle text → empty `abilities` vec; the rendered
    /// output omits the `abilities` field entirely.
    #[test]
    fn no_text_yields_empty_abilities() {
        let card = creature(None);
        let TodoCard::Normal(face) = todo_card("normal", &[&card], &[]).unwrap().unwrap() else {
            panic!("expected Normal");
        };
        assert!(face.abilities.is_empty());
        // The rendered card omits the abilities field entirely.
        let rendered = render(&TodoCard::Normal(face)).unwrap();
        assert!(!rendered.contains("abilities"));
    }

    /// A non-core layout (`split`) yields no `TodoCard`.
    #[test]
    fn unsupported_layout_is_skipped() {
        let card = AtomicCard {
            name: "Whatever".into(),
            face_name: None,
            mana_cost: None,
            color_indicator: vec![],
            types: vec!["Instant".into()],
            supertypes: vec![],
            subtypes: vec![],
            text: None,
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
            layout: "split".into(),
            legalities: Legalities::default(),
        };
        assert!(todo_card("split", &[&card], &[]).unwrap().is_none());
    }

    #[test]
    fn reminder_text() {
        assert_eq!(
            strip_reminder_text("Flying (This creature can't be blocked except by...)"),
            "Flying"
        );
        // Matches the jq behavior: the captured trailing space survives
        // when the parenthetical starts the line.
        assert_eq!(strip_reminder_text("(Reminder) Foo"), " Foo");
        assert_eq!(strip_reminder_text("A (b) c"), "A c");
        // Lines that are nothing but reminder text disappear entirely.
        assert_eq!(
            strip_reminder_text("({R/P} can be paid with {R} or 2 life.)\nGain control."),
            "Gain control."
        );
    }

    /// A reminder-only oracle line that collapses to a lone space after
    /// `strip_reminder_text` (pattern: `"(text) "` → `" "`) must NOT produce an
    /// `Unparsed(" ")` ability slot — that would keep the card as `.ron.todo`
    /// forever. The `.map(str::trim)` in `face()` prevents this.
    #[test]
    fn whitespace_only_line_yields_no_ability() {
        // "(reminder) " → strip → " " (lone space, non-empty without trim).
        // Without `.map(str::trim)` this would produce Unparsed(" ").
        let card = creature(Some("(This is reminder text.) "));
        let TodoCard::Normal(face) = todo_card("normal", &[&card], &[]).unwrap().unwrap() else {
            panic!("expected Normal");
        };
        assert!(
            face.abilities.is_empty(),
            "expected no abilities, got: {:?}",
            face.abilities
        );
    }

    #[test]
    fn keyword_lines() {
        let keywords: Vec<DataStr> = vec!["Flying".into(), "Vigilance".into(), "Equip".into()];
        assert_eq!(
            expand_keyword_lines("flying, vigilance", &keywords),
            "Flying\nVigilance"
        );
        assert_eq!(expand_keyword_lines("Equip {2}", &keywords), "Equip {2}");
        assert_eq!(
            expand_keyword_lines("Draw a card, then discard a card.", &keywords),
            "Draw a card, then discard a card."
        );
    }

    #[test]
    fn self_reference() {
        let no_kw: &[DataStr] = &[];

        // Full name -> ~, every occurrence, case-sensitive whole words.
        assert_eq!(
            self_ref_to_tilde(
                "Storm of Steel deals 2 damage.",
                "Storm of Steel",
                false,
                no_kw
            ),
            "~ deals 2 damage."
        );
        assert_eq!(
            self_ref_to_tilde(
                "Exile Long Rest. Return Long Rest.",
                "Long Rest",
                false,
                no_kw
            ),
            "Exile ~. Return ~."
        );
        // Possessive: the apostrophe is a word boundary, so the name still matches.
        assert_eq!(
            self_ref_to_tilde(
                "Norman Osborn's controller draws.",
                "Norman Osborn",
                false,
                no_kw
            ),
            "~'s controller draws."
        );
        // "named <self>" collapses to "named ~".
        assert_eq!(
            self_ref_to_tilde(
                "A deck can have any number of cards named Rat Colony.",
                "Rat Colony",
                false,
                no_kw
            ),
            "A deck can have any number of cards named ~."
        );
        // Only the card's own name is touched, not another card's.
        assert_eq!(
            self_ref_to_tilde(
                "Norman Osborn can't be blocked.",
                "Green Goblin",
                false,
                no_kw
            ),
            "Norman Osborn can't be blocked."
        );

        // Legendary: pre-comma short name also collapses.
        assert_eq!(
            self_ref_to_tilde(
                "Boromir can't be blocked.",
                "Boromir, Gondor's Hope",
                true,
                no_kw
            ),
            "~ can't be blocked."
        );
        // Short name matches whole words only -- "Gut" leaves "guts" alone.
        assert_eq!(
            self_ref_to_tilde(
                "Gut attacks. The guts spill.",
                "Gut, True Soul Zealot",
                true,
                no_kw
            ),
            "~ attacks. The guts spill."
        );
        // Short-name pass is legendary-only: a comma'd non-legend keeps its prefix.
        assert_eq!(
            self_ref_to_tilde(
                "Borrowing 100 arrows.",
                "Borrowing 100,000 Arrows",
                false,
                no_kw
            ),
            "Borrowing 100 arrows."
        );
        // Guard: short name shorter than three chars is too risky -- skip it.
        assert_eq!(
            self_ref_to_tilde("Me draws a card.", "Me, the Immortal", true, no_kw),
            "Me draws a card."
        );
        // Guard: short name that is a keyword ability is skipped.
        let storm: &[DataStr] = &["Storm".into()];
        assert_eq!(
            self_ref_to_tilde("Storm gets +1/+1.", "Storm, the Tempest", true, storm),
            "Storm gets +1/+1."
        );
        // ...but a distinctive short name still collapses even when other text
        // happens to contain a keyword.
        assert_eq!(
            self_ref_to_tilde(
                "Ral deals 1 damage to any target.",
                "Ral, Storm Conduit",
                true,
                storm
            ),
            "~ deals 1 damage to any target."
        );

        // No self-reference: text is returned unchanged.
        assert_eq!(
            self_ref_to_tilde("Draw a card.", "Some Other Card", false, no_kw),
            "Draw a card."
        );
    }

    /// The generic "this <noun>" pass is name-independent, so an unrelated
    /// face name exercises it in isolation. `~` is the object the text belongs
    /// to — the card, or, inside a granted (quoted) ability, the token.
    #[test]
    fn generic_self_reference() {
        let no_kw: &[DataStr] = &[];
        let tilde = |text: &str| self_ref_to_tilde(text, "Unrelated Name", false, no_kw);

        // Card-type nouns (lowercase), leading and mid-sentence.
        assert_eq!(tilde("This land enters tapped."), "~ enters tapped.");
        assert_eq!(tilde("Sacrifice this creature."), "Sacrifice ~.");
        assert_eq!(tilde("This creature can't block."), "~ can't block.");
        assert_eq!(
            tilde("Return this artifact to its owner's hand."),
            "Return ~ to its owner's hand."
        );
        assert_eq!(tilde("When this enchantment enters"), "When ~ enters");
        assert_eq!(tilde("Exile this permanent."), "Exile ~.");

        // Generic nouns: spell / card.
        assert_eq!(
            tilde("When you cast this spell, draw a card."),
            "When you cast ~, draw a card."
        );
        assert_eq!(
            tilde("Discard this card: Draw a card."),
            "Discard ~: Draw a card."
        );

        // Capitalized permanent-subtype nouns.
        assert_eq!(tilde("Sacrifice this Saga."), "Sacrifice ~.");
        assert_eq!(tilde("When this Vehicle attacks"), "When ~ attacks");
        assert_eq!(
            tilde("Solved — Sacrifice this Case:"),
            "Solved — Sacrifice ~:"
        );
        assert_eq!(tilde("When this Room is unlocked"), "When ~ is unlocked");

        // A token's granted (quoted) ability self-refs the token, so it tildes.
        assert_eq!(
            tilde(
                r#"create a 0/1 Eldrazi Spawn creature token with "Sacrifice this token: Add {C}.""#
            ),
            r#"create a 0/1 Eldrazi Spawn creature token with "Sacrifice ~: Add {C}.""#
        );
        // Two ~ on one line: the card, then the token it makes.
        assert_eq!(
            tilde(
                r#"When this creature dies, create a 1/1 token with "this creature can't block.""#
            ),
            r#"When ~ dies, create a 1/1 token with "~ can't block.""#
        );

        // Possessive keeps the trailing 's (apostrophe is a word boundary).
        assert_eq!(
            tilde("this creature's controller draws a card."),
            "~'s controller draws a card."
        );

        // Non-matches: the bare word (no "this"), excluded nouns, lowercase "case".
        assert_eq!(tilde("Destroy all creatures."), "Destroy all creatures.");
        assert_eq!(
            tilde("Draw two cards this turn."),
            "Draw two cards this turn."
        );
        assert_eq!(tilde("In this case, you win."), "In this case, you win.");
        assert_eq!(tilde("Add this much mana."), "Add this much mana.");
        assert_eq!(
            tilde("Activate this ability only once each turn."),
            "Activate this ability only once each turn."
        );

        // Composes with the name pass: both the name and the "this land" collapse.
        assert_eq!(
            self_ref_to_tilde(
                "Coastal Tower enters tapped. This land taps for mana.",
                "Coastal Tower",
                false,
                no_kw
            ),
            "~ enters tapped. ~ taps for mana."
        );
    }
}
