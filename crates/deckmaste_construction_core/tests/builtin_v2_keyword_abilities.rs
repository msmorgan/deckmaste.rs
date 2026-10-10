use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::FixedKeywordNominalNumber;
use deckmaste_construction_core::macro_def::FixedKeywordParameterGrammar;
use deckmaste_construction_core::macro_def::FixedLexeme;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::KeywordParameterClass;
use deckmaste_construction_core::macro_def::KeywordPayloadOrder;
use deckmaste_construction_core::macro_def::KeywordSeparator;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::ParameterType;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::UnsupportedKeywordParameterClass;
use deckmaste_construction_core::macro_def::read_builtin_v2;

fn declaration_name(head: &str) -> String {
    if head == "∞" {
        return "infinity".to_owned();
    }
    let stem = head
        .split([' ', '-'])
        .map(|word| word.replace(['\'', '!'], ""))
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars.flat_map(char::to_lowercase))
                .collect::<String>()
        })
        .collect::<String>();
    let mut chars = stem.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .chain(chars)
        .collect()
}

fn render_without_hints(spelling: &[SpellingPart]) -> String {
    spelling
        .iter()
        .map(|part| match part {
            SpellingPart::Literal(text) => text.clone(),
            SpellingPart::Param { index, .. } => format!("<Param({index})>"),
        })
        .collect()
}

fn separator(separator: KeywordSeparator) -> &'static str {
    match separator {
        KeywordSeparator::Space => " ",
        KeywordSeparator::Dash => "—",
        KeywordSeparator::SpacedDash => " — ",
    }
}

/// The keyword-line template the grammar surface, params and keyword syntax
/// derive: payload before the head, glued to a bound suffix, or after the
/// head joined by the declared separators, with a fixed quality preposition
/// written out and a power/toughness pair joined by a slash.
fn derived_template(declaration: &NormalizedDeclaration) -> String {
    let grammar = declaration
        .grammar()
        .expect("keyword abilities carry grammar");
    let head = grammar.surfaces()[0].text();
    let params = declaration.params().unwrap_or_default();
    if params.is_empty() {
        return head.to_owned();
    }
    let syntax = declaration.keyword_syntax().unwrap_or_default();
    match syntax.payload_order {
        KeywordPayloadOrder::BeforeHead => return format!("<Param(0)> {head}"),
        KeywordPayloadOrder::BoundSuffix => {
            let suffix = grammar
                .surfaces()
                .iter()
                .find(|surface| surface.feature() == SurfaceFeature::BoundSuffix)
                .expect("a bound-suffix payload has a bound suffix surface");
            let mut template = format!("<Param(0)>{}", suffix.text());
            for index in 1..params.len() {
                write!(
                    template,
                    "{}<Param({index})>",
                    separator(syntax.parameter_separator)
                )
                .unwrap();
            }
            return template;
        }
        KeywordPayloadOrder::AfterHead => {}
    }
    let mut slots = (0..params.len())
        .map(|index| format!("<Param({index})>"))
        .collect::<Vec<_>>();
    if let GrammarRecipe::FixedKeyword {
        parameter:
            Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme(_, member),
                ..
            }),
    } = grammar.recipe()
    {
        slots[0] = format!("{} {}", member.to_lowercase(), slots[0]);
    }
    let parameter_separator = separator(syntax.parameter_separator);
    let payload = if params
        == [
            ParameterType::Cost,
            ParameterType::Power,
            ParameterType::Toughness,
        ] {
        format!("{}{parameter_separator}{}/{}", slots[0], slots[1], slots[2])
    } else {
        slots.join(parameter_separator)
    };
    format!("{head}{}{payload}", separator(syntax.head_separator))
}

fn ability<'a>(declarations: &'a [NormalizedDeclaration], name: &str) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::KeywordAbility
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing keyword ability {name}"))
}

fn parameterized_keyword_params() -> BTreeMap<&'static str, &'static [&'static str]> {
    let mut parameterized = BTreeMap::<&str, &[&str]>::new();
    for name in [
        "absorb",
        "afflict",
        "afterlife",
        "amplify",
        "annihilator",
        "backup",
        "bloodthirst",
        "bushido",
        "casualty",
        "crew",
        "devour",
        "dredge",
        "fabricate",
        "fading",
        "firebending",
        "frenzy",
        "graft",
        "hideaway",
        "mobilize",
        "modular",
        "poisonous",
        "rampage",
        "renown",
        "ripple",
        "saddle",
        "soulshift",
        "teamwork",
        "toxic",
        "tribute",
        "vanishing",
    ] {
        assert_eq!(parameterized.insert(name, &["Amount"]), None);
    }
    for name in [
        "auraSwap",
        "bestow",
        "blitz",
        "buyback",
        "cleave",
        "craft",
        "cumulativeUpkeep",
        "cycling",
        "dash",
        "disguise",
        "disturb",
        "echo",
        "embalm",
        "emerge",
        "encore",
        "entwine",
        "equip",
        "escalate",
        "escape",
        "eternalize",
        "evoke",
        "flashback",
        "foretell",
        "fortify",
        "freerunning",
        "harmonize",
        "kicker",
        "levelUp",
        "madness",
        "mayhem",
        "megamorph",
        "miracle",
        "moreThanMeetsTheEye",
        "morph",
        "mutate",
        "ninjutsu",
        "offspring",
        "outlast",
        "overload",
        "plot",
        "prowl",
        "reconfigure",
        "recover",
        "replicate",
        "scavenge",
        "sneak",
        "spectacle",
        "squad",
        "surge",
        "transfigure",
        "transmute",
        "unearth",
        "ward",
        "warp",
        "webSlinging",
    ] {
        assert_eq!(parameterized.insert(name, &["Cost"]), None);
    }
    for name in ["affinity", "landwalk", "offering", "protection"] {
        assert_eq!(parameterized.insert(name, &["Quality"]), None);
    }
    for name in ["champion", "enchant"] {
        assert_eq!(parameterized.insert(name, &["Predicate"]), None);
    }
    // Re-spelled for `plugins-v2-gift-variants`: gift takes its effect as an
    // instruction [CR#702.174b], no longer a noun phrase.
    assert_eq!(parameterized.insert("gift", &["Instruction"]), None);
    for name in ["awaken", "impending", "reinforce", "suspend"] {
        assert_eq!(parameterized.insert(name, &["Amount", "Cost"]), None);
    }
    assert_eq!(parameterized.insert("splice", &["Quality", "Cost"]), None);
    assert_eq!(
        parameterized.insert("emergeFrom", &["Quality", "Cost"]),
        None
    );
    assert_eq!(
        parameterized.insert("typecycling", &["Quality", "Cost"]),
        None
    );
    for name in [
        "boast", "exhaust", "forecast", "infinity", "maxSpeed", "powerUp", "solved",
    ] {
        assert_eq!(parameterized.insert(name, &["Ability"]), None);
    }
    assert_eq!(parameterized.insert("companion", &["Condition"]), None);
    assert_eq!(
        parameterized.insert("prototype", &["Cost", "Power", "Toughness"]),
        None
    );
    assert_eq!(parameterized.len(), 108);
    parameterized
}

#[test]
fn builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let actual = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::KeywordAbility)
        .map(|declaration| (declaration.identity().name().to_owned(), declaration))
        .collect::<BTreeMap<_, _>>();
    let parameterized = parameterized_keyword_params();

    assert_ne!(
        actual.len(),
        0,
        "the authored inventory must exercise this test"
    );
    for (name, declaration) in &actual {
        // Re-spelled by `semantics-v2-macro-bodies-keyword-abilities`, which
        // gives each keyword ability its own definition as the declaration's
        // body: the family is no longer a grammar-only nursery. What the
        // assertion protects is unchanged — a body never lands without the
        // signature that reads it, which is exactly what graduation means.
        assert_eq!(
            declaration.is_graduated(),
            declaration.body().is_some(),
            "{name}: a body and a signature land together"
        );
        let grammar = declaration
            .grammar()
            .expect("every keyword stub contributes grammar");
        let parameter = match name.as_str() {
            "champion" => Some(FixedKeywordParameterGrammar::Subject {
                indefinite_article: true,
            }),
            "affinity" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "For".to_owned()),
                nominal_number: Some(FixedKeywordNominalNumber::Plural),
            }),
            "protection" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "From".to_owned()),
                nominal_number: None,
            }),
            "splice" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "Onto".to_owned()),
                nominal_number: None,
            }),
            _ => None,
        };
        assert_eq!(
            grammar.recipe(),
            &GrammarRecipe::FixedKeyword { parameter },
            "{name}",
        );
        assert_eq!(
            grammar
                .surfaces()
                .iter()
                .filter(|surface| surface.feature() == SurfaceFeature::Fixed)
                .count(),
            1,
            "{name}",
        );
        assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
        assert!(
            grammar.surfaces()[1..]
                .iter()
                .all(|surface| surface.feature() == SurfaceFeature::BoundSuffix),
            "{name}",
        );
        if let Some(expected_params) = parameterized.get(name.as_str()) {
            let params = declaration.params().expect("parameterized row has params");
            assert_eq!(
                params
                    .iter()
                    .map(deckmaste_construction_core::macro_def::ParameterType::as_str)
                    .collect::<Vec<_>>(),
                *expected_params,
                "{name}",
            );
        } else {
            // Re-spelled with the assertion above: a body may not land without
            // a signature (`ValidationError::BodyWithoutSignature`), so a
            // paramless keyword carrying its definition writes `params: []`
            // where a nursery record wrote nothing. Either reads as "this
            // keyword takes no argument".
            assert!(
                declaration.params().is_none_or(<[ParameterType]>::is_empty),
                "{name}"
            );
        }
        // Re-spelled for `builtin-v2-keyword-spelling-templates`: the spelling
        // is the card-usage template the grammar, params and keyword syntax
        // derive, not a bare copy of the surface. The name still derives from
        // the grammar surface.
        assert_eq!(name, &declaration_name(grammar.surfaces()[0].text()));
        // An unspelled keyword (bare gift, `plugins-v2-gift-variants`) has no
        // template to derive: it is invoked only by its spelled variants.
        if declaration.spelling().is_empty() {
            assert_eq!(name, "gift", "only bare gift is unspelled");
            continue;
        }
        assert_eq!(
            render_without_hints(declaration.spelling()),
            derived_template(declaration),
            "{name}",
        );
    }

    for (name, surface, template) in [
        ("flying", "flying", "flying"),
        ("doubleStrike", "double strike", "double strike"),
        ("forMirrodin", "for Mirrodin!", "for Mirrodin!"),
        (
            "moreThanMeetsTheEye",
            "More Than Meets the Eye",
            "More Than Meets the Eye <Param(0)>",
        ),
        (
            "startYourEngines",
            "start your engines!",
            "start your engines!",
        ),
        ("webSlinging", "web-slinging", "web-slinging <Param(0)>"),
        ("infinity", "∞", "∞ — <Param(0)>"),
    ] {
        let declaration = ability(&declarations, name);
        assert_eq!(render_without_hints(declaration.spelling()), template);
        assert_eq!(declaration.grammar().unwrap().surfaces()[0].text(), surface);
    }
}

#[test]
fn landwalk_declares_one_bound_suffix_surface() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "landwalk")
        .grammar()
        .expect("Landwalk contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "landwalk");
    assert_eq!(grammar.surfaces()[1].feature(), SurfaceFeature::BoundSuffix);
    assert_eq!(grammar.surfaces()[1].text(), "walk");
    let text = std::fs::read_to_string(
        workspace_root.join("plugins_v2/builtin/macros/keyword_abilities/landwalk.ron"),
    )
    .unwrap();
    let definition: macro_ron::MacroDef<deckmaste_construction_core::macro_def::Metadata> =
        deckmaste_construction_core::macro_def::declaration_macro_set()
            .unwrap()
            .read_str(&text)
            .unwrap();
    assert_eq!(
        definition.metadata.grammar,
        Some(deckmaste_construction_core::macro_def::Grammar::FixedKeyword {
            surface: "landwalk".into(),
            onset: None,
            bound_suffix: Some(deckmaste_construction_core::macro_def::BoundSuffixGrammar {
                surface: "walk".into(),
                stem_class: deckmaste_construction_core::macro_def::BoundKeywordStemClass::SubtypesAndType {
                    category: deckmaste_construction_core::macro_def::SubtypeCategory::Land,
                    card_type: "land".into(),
                },
            }),
            participial_adjective: None,
            verb: None,
            block_label: None,
            parameter: None,
        })
    );
}

#[test]
fn prepositional_quality_keywords_declare_their_selected_markers() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    for (name, member, nominal_number) in [
        ("affinity", "For", Some(FixedKeywordNominalNumber::Plural)),
        ("protection", "From", None),
        ("splice", "Onto", None),
    ] {
        assert_eq!(
            ability(&declarations, name).grammar().unwrap().recipe(),
            &GrammarRecipe::FixedKeyword {
                parameter: Some(FixedKeywordParameterGrammar::Quality {
                    preposition: FixedLexeme("Preposition".to_owned(), member.to_owned()),
                    nominal_number,
                }),
            },
            "{name}",
        );
    }
}

#[test]
fn unsupported_keyword_parameter_families_are_explicitly_deferred() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    for name in [
        "boast", "exhaust", "forecast", "infinity", "maxSpeed", "powerUp", "solved",
    ] {
        assert_eq!(
            ability(&declarations, name).keyword_parameter_class(),
            Some(KeywordParameterClass::Unsupported(
                UnsupportedKeywordParameterClass::Ability
            )),
            "{name}",
        );
    }
    assert_eq!(
        ability(&declarations, "companion").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::Condition
        )),
    );
    assert_eq!(
        ability(&declarations, "prototype").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::CostPowerToughness
        )),
    );
    // Bare gift takes its effect as an instruction and has no spelling: only
    // its spelled variants are printed [CR#702.174a,702.174b].
    let gift = ability(&declarations, "gift");
    assert_eq!(
        gift.keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::Instruction
        )),
    );
    assert_eq!(gift.spelling(), []);
}

#[test]
fn attachment_keywords_declare_their_participial_adjective_surfaces() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    for (name, surface) in [
        ("equip", "equipped"),
        ("enchant", "enchanted"),
        ("fortify", "fortified"),
    ] {
        let grammar = ability(&declarations, name)
            .grammar()
            .expect("keyword declaration contributes its primary grammar");
        assert_eq!(
            grammar.recipe(),
            &GrammarRecipe::FixedKeyword { parameter: None }
        );
        let adjective = grammar
            .participial_adjective()
            .expect("attachment keyword contributes a participial adjective");
        assert_eq!(adjective.feature(), SurfaceFeature::PAST_PARTICIPLE);
        assert_eq!(adjective.text(), surface, "{name}");
    }
}

#[test]
fn level_up_declares_its_distinct_block_label_surface() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "levelUp")
        .grammar()
        .expect("Level Up contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "level up");
    let label = grammar
        .block_label()
        .expect("Level Up contributes its level-band label");
    assert_eq!(label.feature(), SurfaceFeature::BlockLabel);
    assert_eq!(label.text(), "LEVEL");
}
