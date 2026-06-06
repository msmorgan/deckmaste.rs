use std::sync::LazyLock;

use deckmaste_core::{Color, ManaCost};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;

#[derive(Debug, PartialEq, Serialize)]
enum Stat {
    Number(serde_json::Number),
    NonNumber(String),
}

#[derive(Debug, PartialEq, Serialize)]
enum Subtype {
    Creature(String),
    Artifact(String),
    Enchantment(String),
    Land(String),
    Battle(String),
    Planeswalker(String),
    Spell(String),
}

#[derive(Serialize)]
enum CardFace {
    Todo {
        name: String,
        #[serde(
            skip_serializing_if = "Option::is_none",
            serialize_with = "one_line_if_single_opt"
        )]
        mana_cost: Option<ManaCost>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            serialize_with = "one_line_if_single_opt"
        )]
        color_indicator: Option<Vec<Color>>,
        #[serde(serialize_with = "one_line_if_single")]
        types: Vec<String>,
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            serialize_with = "one_line_if_single"
        )]
        supertypes: Vec<String>,
        #[serde(
            skip_serializing_if = "Vec::is_empty",
            serialize_with = "one_line_if_single"
        )]
        subtypes: Vec<Subtype>,
        #[serde(
            skip_serializing_if = "Option::is_none",
            serialize_with = "one_line_if_single_opt"
        )]
        text: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        power: Option<Stat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        toughness: Option<Stat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        loyalty: Option<Stat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        defense: Option<Stat>,
    },
}

/// Serializes a single-element array on one line (`[Red]`); longer arrays
/// fall through to the chopped pretty-printer. ron's config cannot express
/// this, so the compact form is pre-rendered and embedded as a RawValue.
fn one_line_if_single<T: Serialize, S: serde::Serializer>(
    array: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::Error as _;

    if array.len() != 1 {
        return array.serialize(serializer);
    }
    let compact = ron::Options::default()
        .to_string_pretty(
            &array,
            ron::ser::PrettyConfig::default()
                .escape_strings(false)
                .depth_limit(0),
        )
        .map_err(S::Error::custom)?;
    ron::value::RawValue::from_ron(&compact)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

fn one_line_if_single_opt<T: Serialize, A, S: serde::Serializer>(
    array: &Option<A>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    A: std::ops::Deref<Target = [T]>,
{
    let array = array.as_deref().expect("field is skipped when None");
    one_line_if_single(array, serializer)
}

/// A single face serializes as a bare `Todo(...)`; multiple faces serialize
/// as a tuple named after the card's layout, e.g. `Transform(Todo(...),
/// Todo(...))`.
struct CardFile {
    layout: &'static str,
    faces: Vec<CardFace>,
}

impl Serialize for CardFile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTupleVariant;

        if let [single] = self.faces.as_slice() {
            return single.serialize(serializer);
        }
        let mut faces =
            serializer.serialize_tuple_variant("CardFile", 0, self.layout, self.faces.len())?;
        for face in &self.faces {
            faces.serialize_field(face)?;
        }
        faces.end()
    }
}

fn layout_name(layout: &str) -> anyhow::Result<&'static str> {
    Ok(match layout {
        "adventure" => "Adventure",
        "aftermath" => "Aftermath",
        "flip" => "Flip",
        "meld" => "Meld",
        "modal_dfc" => "ModalDfc",
        "prepare" => "Prepare",
        "split" => "Split",
        "transform" => "Transform",
        other => anyhow::bail!("unsupported multi-face layout: {other:?}"),
    })
}

fn ron_options() -> ron::Options {
    // Implicit Some keeps the optional fields unwrapped, and is a default
    // extension so no #![enable(...)] header is emitted.
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
}

/// Multi-line text is written verbatim and arrays are chopped, one element
/// per line. The multi-face config additionally puts each face on its own
/// indented line, with the depth limit keeping `Hybrid(...)` mana symbols
/// inline (faces are at depth 1, array elements at depth 3, hybrid tuple
/// members at depth 4).
fn pretty_config(multi_face: bool) -> ron::ser::PrettyConfig {
    let config = ron::ser::PrettyConfig::default().escape_strings(false);
    if multi_face {
        config.separate_tuple_members(true).depth_limit(3)
    } else {
        config
    }
}

/// A subtype definition file under the plugin's types/<category> directory,
/// e.g. `CreatureType("Advisor")`.
#[derive(Deserialize)]
enum TypeDef {
    ArtifactType(String),
    BattleType(String),
    CreatureType(String),
    EnchantmentType(String),
    LandType(String),
    PlaneswalkerType(String),
    SpellType(String),
}

impl TypeDef {
    fn into_parts(self) -> (&'static str, String) {
        match self {
            TypeDef::ArtifactType(name) => ("artifact", name),
            TypeDef::BattleType(name) => ("battle", name),
            TypeDef::CreatureType(name) => ("creature", name),
            TypeDef::EnchantmentType(name) => ("enchantment", name),
            TypeDef::LandType(name) => ("land", name),
            TypeDef::PlaneswalkerType(name) => ("planeswalker", name),
            TypeDef::SpellType(name) => ("spell", name),
        }
    }
}

/// Categorized subtype lists, in the lookup order of the jq version (the
/// first category containing a subtype wins).
struct SubtypeCategories(Vec<(Vec<String>, fn(String) -> Subtype)>);

impl SubtypeCategories {
    /// The plugin's types/<category> directories -- written by migration 003
    /// and possibly extended by hand -- are the source of truth for which
    /// subtypes exist and which category each belongs to.
    fn load(plugin: &super::PluginLayout) -> anyhow::Result<Self> {
        const CATEGORIES: [(&str, fn(String) -> Subtype); 7] = [
            ("creature", Subtype::Creature),
            ("artifact", Subtype::Artifact),
            ("enchantment", Subtype::Enchantment),
            ("land", Subtype::Land),
            ("battle", Subtype::Battle),
            ("planeswalker", Subtype::Planeswalker),
            ("spell", Subtype::Spell),
        ];

        let mut categories = Vec::with_capacity(CATEGORIES.len());
        for (category, constructor) in CATEGORIES {
            let dir = plugin.types_dir(category)?;
            let mut names = Vec::new();
            for entry in std::fs::read_dir(&dir)? {
                let path = entry?.path();
                if path.extension() != Some(std::ffi::OsStr::new("ron")) {
                    continue;
                }
                let def: TypeDef = ron::from_str(&std::fs::read_to_string(&path)?)
                    .map_err(|e| anyhow::anyhow!("parsing {}: {e}", path.display()))?;
                let (def_category, name) = def.into_parts();
                if def_category != category {
                    anyhow::bail!(
                        "{} defines a {def_category} type but lives under types/{category}",
                        path.display()
                    );
                }
                names.push(name);
            }
            categories.push((names, constructor));
        }
        Ok(Self(categories))
    }

    fn subtype(&self, subtype: &str) -> anyhow::Result<Subtype> {
        self.0
            .iter()
            .find(|(subtypes, _)| subtypes.iter().any(|s| s == subtype))
            .map(|(_, category)| category(subtype.to_owned()))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "subtype {subtype:?} is not defined in the plugin's types directories \
                     (has migration 003 run?)"
                )
            })
    }
}

// We count non-null, non-"Banned" as legal.
fn is_supported(card: &AtomicCard) -> bool {
    card.legalities.vintage.as_deref().unwrap_or("Banned") != "Banned"
        && card.layout.as_str() != "reversible_card"
}

/// Maps windows-unsafe filename characters to their fullwidth equivalents,
/// e.g. "Fire // Ice" -> "Fire ／／ Ice".
fn to_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if "<>:\"/\\|?*".contains(c) {
                char::from_u32(c as u32 + 0xFEE0).expect("fullwidth ASCII is valid")
            } else {
                c
            }
        })
        .collect()
}

/// Uppercases the first character (ASCII only, like jq's ascii_upcase).
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

/// Splits lines that are comma-separated lists of keyword abilities into one
/// keyword per line, e.g. "Flying, vigilance" -> "Flying\nVigilance".
fn expand_keyword_lines(text: &str, keyword_abilities: &[DataStr<'_>]) -> String {
    text.split('\n')
        .flat_map(|line| {
            let items: Vec<String> = line.split(", ").map(capitalize).collect();
            if items.iter().all(|item| {
                keyword_abilities
                    .iter()
                    .any(|keyword| item.starts_with(keyword.as_str()))
            }) {
                items
            } else {
                vec![line.to_owned()]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn ron_color(code: &str) -> anyhow::Result<Color> {
    Color::from_code(code).ok_or_else(|| anyhow::anyhow!("unrecognized color indicator: {code:?}"))
}

/// `Number(2)` for values that parse as JSON numbers, `NonNumber("*")`
/// otherwise.
fn stat(value: &str) -> Stat {
    match serde_json::from_str(value) {
        Ok(number) => Stat::Number(number),
        Err(_) => Stat::NonNumber(value.to_owned()),
    }
}

fn render_face(
    card: &AtomicCard,
    keyword_abilities: &[DataStr<'_>],
    categories: &SubtypeCategories,
) -> anyhow::Result<CardFace> {
    Ok(CardFace::Todo {
        name: card.face_name.as_deref().unwrap_or(&card.name).to_owned(),
        mana_cost: card.mana_cost.as_deref().map(str::parse).transpose()?,
        color_indicator: card
            .color_indicator
            .as_deref()
            .filter(|colors| !colors.is_empty())
            .map(|colors| colors.iter().map(|code| ron_color(code)).collect())
            .transpose()?,
        types: card.types.iter().map(|t| t.as_str().to_owned()).collect(),
        supertypes: card
            .supertypes
            .iter()
            .map(|t| t.as_str().to_owned())
            .collect(),
        subtypes: card
            .subtypes
            .iter()
            .map(|subtype| categories.subtype(subtype))
            .collect::<anyhow::Result<_>>()?,
        // One element per line of normalized oracle text -- one ability
        // each, except that modal/leveler lines stay split. Cards whose text
        // is nothing but reminder text (basic lands) end up with no text.
        text: card.text.as_deref().and_then(|text| {
            let text = crate::data::academyruins::normalize_quotes(text);
            let text = expand_keyword_lines(&strip_reminder_text(&text), keyword_abilities);
            let lines: Vec<String> = text
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect();
            (!lines.is_empty()).then_some(lines)
        }),
        power: card.power.as_deref().map(stat),
        toughness: card.toughness.as_deref().map(stat),
        loyalty: card.loyalty.as_deref().map(stat),
        defense: card.defense.as_deref().map(stat),
    })
}

pub(super) struct CardTodos;

impl super::Migration for CardTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let atomic_bytes = crate::data::atomic_cards_bytes()?;
        let atomic_cards = crate::data::mtgjson::AtomicCards::parse(&atomic_bytes)?;
        let keywords_bytes = crate::data::keywords_bytes()?;
        let keyword_abilities =
            crate::data::academyruins::Keywords::parse(&keywords_bytes)?.keyword_abilities;
        let categories = SubtypeCategories::load(plugin)?;
        let dest_dir = plugin.cards_dir()?;
        let options = ron_options();

        for (name, faces) in &atomic_cards.data {
            let supported: Vec<&AtomicCard> = faces.iter().filter(|c| is_supported(c)).collect();
            if supported.is_empty() {
                continue;
            }

            let dest = dest_dir.join(format!("{}.ron", to_filename(name)));
            if !super::is_todo(&dest)? {
                continue;
            }

            let card_file = CardFile {
                // The layout only names the wrapper of multi-face cards; a
                // single face serializes as a bare Todo.
                layout: if supported.len() > 1 {
                    layout_name(&supported[0].layout)?
                } else {
                    "Normal"
                },
                faces: supported
                    .iter()
                    .map(|face| render_face(face, &keyword_abilities, &categories))
                    .collect::<anyhow::Result<_>>()?,
            };
            let serialized = options
                .to_string_pretty(&card_file, pretty_config(supported.len() > 1))
                .map_err(|e| anyhow::anyhow!(e))
                .map_err(|e| e.context(format!("serializing card {name:?}")))?;

            std::fs::write(&dest, serialized + "\n")?;
            eprintln!("wrote {}", dest.display());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::ManaSymbol;

    use super::*;

    #[test]
    fn filenames() {
        assert_eq!(to_filename("Fire // Ice"), "Fire ／／ Ice");
        assert_eq!(to_filename("Question?"), "Question？");
        assert_eq!(to_filename("Lightning Bolt"), "Lightning Bolt");
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
    fn mana_symbols_serialize_flat() {
        // The Simple and Color wrapper variants are untagged: the nested
        // model must not show up in the RON.
        let symbols: ManaCost = "{R}{2}{C}{2/W}{W/P}{G/U/P}{X}{S}".parse().unwrap();
        assert_eq!(
            ron_options().to_string(&symbols).unwrap(),
            "[Red,Generic(2),Colorless,Hybrid(Generic(2),White),Phyrexian(White,None),\
             Phyrexian(Green,Blue),Variable,Snow]"
        );
    }

    #[test]
    fn stats() {
        let render = |value| ron_options().to_string(&stat(value)).unwrap();
        assert_eq!(render("2"), "Number(2)");
        assert_eq!(render("-1"), "Number(-1)");
        assert_eq!(render("*"), "NonNumber(\"*\")");
        assert_eq!(render("1+*"), "NonNumber(\"1+*\")");
        assert_eq!(render("X"), "NonNumber(\"X\")");
    }

    #[test]
    fn subtypes_load_from_plugin_type_files() {
        let base = std::env::temp_dir().join(format!("deckmaste_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("types/creature")).unwrap();
        std::fs::create_dir_all(base.join("types/land")).unwrap();
        std::fs::write(
            base.join("types/creature/TimeLord.ron"),
            "CreatureType(\"Time Lord\")\n",
        )
        .unwrap();
        std::fs::write(base.join("types/land/Urzas.ron"), "LandType(\"Urza's\")\n").unwrap();

        let plugin = crate::layout::PluginLayout::new(&base).unwrap();
        let categories = SubtypeCategories::load(&plugin).unwrap();
        assert_eq!(
            categories.subtype("Time Lord").unwrap(),
            Subtype::Creature("Time Lord".to_owned())
        );
        assert_eq!(
            categories.subtype("Urza's").unwrap(),
            Subtype::Land("Urza's".to_owned())
        );
        // Unknown subtypes are validation errors.
        assert!(categories.subtype("Missingno").is_err());

        // A definition in the wrong directory is a validation error.
        std::fs::write(
            base.join("types/creature/Sneaky.ron"),
            "LandType(\"Sneaky\")\n",
        )
        .unwrap();
        let Err(error) = SubtypeCategories::load(&plugin) else {
            panic!("mismatched type definition should fail to load");
        };
        assert!(error.to_string().contains("defines a land type"), "{error}");

        std::fs::remove_dir_all(&base).unwrap();
    }

    fn test_face(name: &str, vanilla: bool) -> CardFace {
        CardFace::Todo {
            name: name.to_owned(),
            mana_cost: Some(
                vec![
                    ManaSymbol::Hybrid(2.into(), Color::White),
                    ManaSymbol::Simple(Color::Green.into()),
                ]
                .into(),
            ),
            color_indicator: None,
            types: vec!["Creature".to_owned()],
            supertypes: vec![],
            subtypes: vec![Subtype::Creature("Time Lord".to_owned())],
            text: (!vanilla).then(|| {
                vec![
                    "Flying".to_owned(),
                    "Doctor's \"companion\" rule.".to_owned(),
                ]
            }),
            power: Some(stat("2")),
            toughness: Some(stat("*")),
            loyalty: None,
            defense: None,
        }
    }

    #[test]
    fn single_face_serialization() {
        let card = CardFile {
            layout: "Normal",
            faces: vec![test_face("Solo", false)],
        };
        let serialized = ron_options()
            .to_string_pretty(&card, pretty_config(false))
            .unwrap();
        assert_eq!(
            serialized,
            r##"Todo(
    name: "Solo",
    mana_cost: [
        Hybrid(Generic(2), White),
        Green,
    ],
    types: ["Creature"],
    subtypes: [Creature("Time Lord")],
    text: [
        "Flying",
        r#"Doctor's "companion" rule."#,
    ],
    power: Number(2),
    toughness: NonNumber("*"),
)"##
        );
    }

    #[test]
    fn multi_face_serialization() {
        let card = CardFile {
            layout: "Transform",
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = ron_options()
            .to_string_pretty(&card, pretty_config(true))
            .unwrap();
        // Faces are split onto their own lines while Hybrid mana symbols
        // stay inline, and raw string bodies stay unindented.
        assert_eq!(
            serialized,
            r##"Transform(
    Todo(
        name: "Front",
        mana_cost: [
            Hybrid(Generic(2), White),
            Green,
        ],
        types: ["Creature"],
        subtypes: [Creature("Time Lord")],
        text: [
            "Flying",
            r#"Doctor's "companion" rule."#,
        ],
        power: Number(2),
        toughness: NonNumber("*"),
    ),
    Todo(
        name: "Back",
        mana_cost: [
            Hybrid(Generic(2), White),
            Green,
        ],
        types: ["Creature"],
        subtypes: [Creature("Time Lord")],
        power: Number(2),
        toughness: NonNumber("*"),
    ),
)"##
        );
    }
}
