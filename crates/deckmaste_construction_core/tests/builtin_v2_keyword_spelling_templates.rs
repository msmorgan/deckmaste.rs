//! Every keyword action and keyword ability spelling is the uninflected
//! template a card prints (`builtin-v2-keyword-spelling-templates`; the
//! 2026-10-07 amendment to `docs/decisions/builtin-v2-macro-spelling-and-grammar.md`).

use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::FixedKeywordNominalNumber;
use deckmaste_construction_core::macro_def::FixedKeywordParameterGrammar;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingHint;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_builtin_v2;

/// Keyword actions that still spell only their bare head: none. Fight, the
/// last, spells its parameter now that its first creature is the performer
/// (`actor`): ticket `semantics-v2-fight-is-performed-by-the-creature`.
const PENDING_RULING: &[&str] = &[];

const KEYWORD_ACTION_TEMPLATES: &[(&str, &str)] = &[
    ("activate", "activate"),
    ("adapt", "adapt <Param(0)>"),
    ("airbend", "airbend <Param(0)>"),
    ("amass", "amass <Param(0):plural> <Param(1)>"),
    ("assemble", "assemble"),
    ("attach", "attach <Param(0)> to <Param(1)>"),
    ("behold", "behold a <Param(0)>"),
    ("blight", "blight <Param(0)>"),
    ("bolster", "bolster <Param(0)>"),
    ("cast", "cast"),
    ("clash", "clash"),
    ("cloak", "cloak"),
    ("collectEvidence", "collect evidence"),
    ("connive", "connive"),
    ("conniveN", "connive <Param(0)>"),
    ("convert", "convert <Param(0)>"),
    ("counter", "counter <Param(0)>"),
    ("create", "create <Param(0):word> <Param(1)>"),
    ("destroy", "destroy <Param(0)>"),
    ("detain", "detain <Param(0)>"),
    ("discard", "discard <Param(0)>"),
    ("discover", "discover"),
    ("double", "double"),
    ("earthbend", "earthbend <Param(0)>"),
    ("endure", "endure <Param(0)>"),
    ("exchange", "exchange"),
    ("exert", "exert <Param(0)>"),
    ("exile", "exile <Param(0)>"),
    ("explore", "explore"),
    (
        "faceAVillainousChoice",
        "face a villainous choice — <Param(0)>, or <Param(1)>",
    ),
    ("fateseal", "fateseal <Param(0)>"),
    ("fight", "fight <Param(0)>"),
    ("forage", "forage"),
    ("goad", "goad <Param(0)>"),
    ("harness", "harness <Param(0)>"),
    ("heal", "heal <Param(0)>"),
    ("incubate", "incubate"),
    ("investigate", "investigate"),
    ("learn", "learn"),
    ("manifest", "manifest"),
    ("manifestDread", "manifest dread"),
    ("meld", "meld <Param(0)> into <Param(1)>"),
    ("mill", "mill <Param(0):word> cards"),
    ("monstrosity", "monstrosity <Param(0)>"),
    ("play", "play"),
    ("populate", "populate"),
    ("proliferate", "proliferate"),
    ("recruit", "recruit"),
    ("regenerate", "regenerate <Param(0)>"),
    ("reveal", "reveal <Param(0)>"),
    ("sacrifice", "sacrifice <Param(0)>"),
    ("scry", "scry <Param(0)>"),
    ("search", "search <Param(0)> for <Param(1)> <Param(2)>"),
    ("shuffle", "shuffle"),
    ("support", "support"),
    ("surveil", "surveil <Param(0)>"),
    ("suspect", "suspect <Param(0)>"),
    ("tap", "tap <Param(0)>"),
    ("theRingTemptsYou", "the Ring tempts you"),
    ("timeTravel", "time travel"),
    ("transform", "transform <Param(0)>"),
    ("triple", "triple"),
    ("untap", "untap <Param(0)>"),
    ("ventureIntoTheDungeon", "venture into the dungeon"),
    ("vote", "<Param(0)> vote for <Param(1)>"),
    ("waterbend", "waterbend"),
];

const KEYWORD_ABILITY_TEMPLATES: &[(&str, &str)] = &[
    ("absorb", "absorb <Param(0)>"),
    ("affinity", "affinity for <Param(0):plural>"),
    ("afflict", "afflict <Param(0)>"),
    ("afterlife", "afterlife <Param(0)>"),
    ("aftermath", "aftermath"),
    ("amplify", "amplify <Param(0)>"),
    ("annihilator", "annihilator <Param(0)>"),
    ("ascend", "ascend"),
    ("assist", "assist"),
    ("auraSwap", "aura swap <Param(0)>"),
    ("awaken", "awaken <Param(0)>—<Param(1)>"),
    ("backup", "backup <Param(0)>"),
    ("banding", "banding"),
    ("bargain", "bargain"),
    ("battleCry", "battle cry"),
    ("bestow", "bestow <Param(0)>"),
    ("blitz", "blitz <Param(0)>"),
    ("bloodthirst", "bloodthirst <Param(0)>"),
    ("boast", "boast — <Param(0)>"),
    ("bushido", "bushido <Param(0)>"),
    ("buyback", "buyback <Param(0)>"),
    ("cascade", "cascade"),
    ("casualty", "casualty <Param(0)>"),
    ("champion", "champion <Param(0)>"),
    ("changeling", "changeling"),
    ("cipher", "cipher"),
    ("cleave", "cleave <Param(0)>"),
    ("companion", "companion — <Param(0)>"),
    ("compleated", "compleated"),
    ("conspire", "conspire"),
    ("convoke", "convoke"),
    ("craft", "craft <Param(0)>"),
    ("crew", "crew <Param(0)>"),
    ("cumulativeUpkeep", "cumulative upkeep <Param(0)>"),
    ("cycling", "cycling <Param(0)>"),
    ("dash", "dash <Param(0)>"),
    ("daybound", "daybound"),
    ("deathtouch", "deathtouch"),
    ("decayed", "decayed"),
    ("defender", "defender"),
    ("delve", "delve"),
    ("demonstrate", "demonstrate"),
    ("dethrone", "dethrone"),
    ("devoid", "devoid"),
    ("devour", "devour <Param(0)>"),
    ("disguise", "disguise <Param(0)>"),
    ("disturb", "disturb <Param(0)>"),
    ("doubleStrike", "double strike"),
    ("dredge", "dredge <Param(0)>"),
    ("echo", "echo <Param(0)>"),
    ("embalm", "embalm <Param(0)>"),
    ("emerge", "emerge <Param(0)>"),
    ("enchant", "enchant <Param(0)>"),
    ("encore", "encore <Param(0)>"),
    ("enlist", "enlist"),
    ("entwine", "entwine <Param(0)>"),
    ("epic", "epic"),
    ("equip", "equip <Param(0)>"),
    ("escalate", "escalate <Param(0)>"),
    ("escape", "escape <Param(0)>"),
    ("eternalize", "eternalize <Param(0)>"),
    ("evoke", "evoke <Param(0)>"),
    ("evolve", "evolve"),
    ("exalted", "exalted"),
    ("exhaust", "exhaust — <Param(0)>"),
    ("exploit", "exploit"),
    ("extort", "extort"),
    ("fabricate", "fabricate <Param(0)>"),
    ("fading", "fading <Param(0)>"),
    ("fear", "fear"),
    ("firebending", "firebending <Param(0)>"),
    ("firstStrike", "first strike"),
    ("flanking", "flanking"),
    ("flash", "flash"),
    ("flashback", "flashback <Param(0)>"),
    ("flying", "flying"),
    ("forMirrodin", "for Mirrodin!"),
    ("forecast", "forecast — <Param(0)>"),
    ("foretell", "foretell <Param(0)>"),
    ("fortify", "fortify <Param(0)>"),
    ("freerunning", "freerunning <Param(0)>"),
    ("frenzy", "frenzy <Param(0)>"),
    ("fuse", "fuse"),
    // Unspelled: bare gift is invoked only by its spelled variants
    // (`plugins-v2-gift-variants`).
    ("gift", ""),
    // The six spelled variants, each the printed phrase [CR#702.174d..702.174i].
    ("giftACard", "gift a card"),
    ("giftAFood", "gift a Food"),
    ("giftATappedFish", "gift a tapped Fish"),
    ("giftATreasure", "gift a Treasure"),
    ("giftAnExtraTurn", "gift an extra turn"),
    ("giftAnOctopus", "gift an Octopus"),
    ("graft", "graft <Param(0)>"),
    ("gravestorm", "gravestorm"),
    ("harmonize", "harmonize <Param(0)>"),
    ("haste", "haste"),
    ("haunt", "haunt"),
    ("hexproof", "hexproof"),
    ("hideaway", "hideaway <Param(0)>"),
    ("horsemanship", "horsemanship"),
    ("impending", "impending <Param(0)>—<Param(1)>"),
    ("improvise", "improvise"),
    ("increment", "increment"),
    ("indestructible", "indestructible"),
    ("infect", "infect"),
    ("infinity", "∞ — <Param(0)>"),
    ("ingest", "ingest"),
    ("intimidate", "intimidate"),
    ("jobSelect", "job select"),
    ("jumpStart", "jump-start"),
    ("kicker", "kicker <Param(0)>"),
    ("landwalk", "<Param(0)>walk"),
    ("levelUp", "level up <Param(0)>"),
    ("lifelink", "lifelink"),
    ("livingMetal", "living metal"),
    ("livingWeapon", "living weapon"),
    ("madness", "madness <Param(0)>"),
    ("maxSpeed", "max speed — <Param(0)>"),
    ("mayhem", "mayhem <Param(0)>"),
    ("melee", "melee"),
    ("menace", "menace"),
    ("mentor", "mentor"),
    ("miracle", "miracle <Param(0)>"),
    ("mobilize", "mobilize <Param(0)>"),
    ("modular", "modular <Param(0)>"),
    ("moreThanMeetsTheEye", "More Than Meets the Eye <Param(0)>"),
    ("morph", "morph <Param(0)>"),
    ("mutate", "mutate <Param(0)>"),
    ("myriad", "myriad"),
    ("nightbound", "nightbound"),
    ("ninjutsu", "ninjutsu <Param(0)>"),
    ("offering", "<Param(0)> offering"),
    ("offspring", "offspring <Param(0)>"),
    ("outlast", "outlast <Param(0)>"),
    ("overload", "overload <Param(0)>"),
    ("paradigm", "paradigm"),
    ("partner", "partner"),
    ("persist", "persist"),
    ("phasing", "phasing"),
    ("plot", "plot <Param(0)>"),
    ("poisonous", "poisonous <Param(0)>"),
    ("powerUp", "power-up — <Param(0)>"),
    ("protection", "protection from <Param(0):plural>"),
    ("prototype", "prototype <Param(0)> — <Param(1)>/<Param(2)>"),
    ("provoke", "provoke"),
    ("prowess", "prowess"),
    ("prowl", "prowl <Param(0)>"),
    ("rampage", "rampage <Param(0)>"),
    ("ravenous", "ravenous"),
    ("reach", "reach"),
    ("readAhead", "read ahead"),
    ("rebound", "rebound"),
    ("reconfigure", "reconfigure <Param(0)>"),
    ("recover", "recover <Param(0)>"),
    ("reinforce", "reinforce <Param(0)>—<Param(1)>"),
    ("renown", "renown <Param(0)>"),
    ("replicate", "replicate <Param(0)>"),
    ("retrace", "retrace"),
    ("riot", "riot"),
    ("ripple", "ripple <Param(0)>"),
    ("saddle", "saddle <Param(0)>"),
    ("scavenge", "scavenge <Param(0)>"),
    ("shadow", "shadow"),
    ("shroud", "shroud"),
    ("skulk", "skulk"),
    ("sneak", "sneak <Param(0)>"),
    ("solved", "solved — <Param(0)>"),
    ("soulbond", "soulbond"),
    ("soulshift", "soulshift <Param(0)>"),
    ("spectacle", "spectacle <Param(0)>"),
    ("splice", "splice onto <Param(0)> <Param(1)>"),
    ("splitSecond", "split second"),
    ("spree", "spree"),
    ("squad", "squad <Param(0)>"),
    ("startYourEngines", "start your engines!"),
    ("station", "station"),
    ("storied", "storied"),
    ("storm", "storm"),
    ("sunburst", "sunburst"),
    ("surge", "surge <Param(0)>"),
    ("suspend", "suspend <Param(0)>—<Param(1)>"),
    ("teamwork", "teamwork <Param(0)>"),
    ("tiered", "tiered"),
    ("toxic", "toxic <Param(0)>"),
    ("training", "training"),
    ("trample", "trample"),
    ("transfigure", "transfigure <Param(0)>"),
    ("transmute", "transmute <Param(0)>"),
    ("tribute", "tribute <Param(0)>"),
    ("umbraArmor", "umbra armor"),
    ("undaunted", "undaunted"),
    ("undying", "undying"),
    ("unearth", "unearth <Param(0)>"),
    ("unleash", "unleash"),
    ("vanishing", "vanishing <Param(0)>"),
    ("vigilance", "vigilance"),
    ("ward", "ward <Param(0)>"),
    ("warp", "warp <Param(0)>"),
    ("webSlinging", "web-slinging <Param(0)>"),
    ("wither", "wither"),
];

fn render(spelling: &[SpellingPart]) -> String {
    spelling
        .iter()
        .map(|part| match part {
            SpellingPart::Literal(text) => text.clone(),
            SpellingPart::Param { index, hint: None } => format!("<Param({index})>"),
            SpellingPart::Param {
                index,
                hint: Some(hint),
            } => format!("<Param({index}):{}>", hint.as_str()),
        })
        .collect()
}

fn keyword_declarations() -> Vec<NormalizedDeclaration> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load")
        .into_iter()
        .filter(|declaration| {
            matches!(
                declaration.identity().kind(),
                DeclarationKind::KeywordAction | DeclarationKind::KeywordAbility
            )
        })
        .collect()
}

#[test]
fn every_keyword_declaration_pins_its_template() {
    let declarations = keyword_declarations();
    for (kind, pins) in [
        (DeclarationKind::KeywordAction, KEYWORD_ACTION_TEMPLATES),
        (DeclarationKind::KeywordAbility, KEYWORD_ABILITY_TEMPLATES),
    ] {
        let actual = declarations
            .iter()
            .filter(|declaration| declaration.identity().kind() == kind)
            .map(|declaration| {
                (
                    declaration.identity().name().to_owned(),
                    render(declaration.spelling()),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let expected = pins
            .iter()
            .map(|(name, template)| ((*name).to_owned(), (*template).to_owned()))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(actual, expected, "{kind:?} templates");
    }
}

#[test]
fn every_keyword_template_is_consistent_with_its_params_and_grammar() {
    let declarations = keyword_declarations();
    assert_eq!(declarations.len(), 66 + 198);
    for declaration in &declarations {
        let name = declaration.identity().name();
        let params = declaration.params().unwrap_or_default();
        let spelling = declaration.spelling();
        if spelling.is_empty() {
            assert_eq!(
                (declaration.identity().kind(), name),
                (DeclarationKind::KeywordAbility, "gift"),
                "only bare gift is unspelled"
            );
            continue;
        }

        // Every hole names a declared parameter, each parameter exactly once,
        // and a hint applies to its parameter's kind.
        let mut slots = Vec::new();
        for part in spelling {
            if let SpellingPart::Param { index, hint } = part {
                let kind = params
                    .get(*index)
                    .unwrap_or_else(|| panic!("{name}: Param({index}) is out of range"));
                if let Some(hint) = hint {
                    assert!(hint.applies_to(kind), "{name}: {hint:?} on {kind:?}");
                }
                slots.push(*index);
            }
        }
        slots.sort_unstable();
        if PENDING_RULING.contains(&name) {
            assert!(
                slots.is_empty(),
                "{name} is templated now; remove it from PENDING_RULING"
            );
        } else {
            assert_eq!(
                slots,
                (0..params.len()).collect::<Vec<_>>(),
                "{name}: every parameter appears in the template exactly once"
            );
        }

        // The template's head literal is the grammar head (or, glued after a
        // hole, the grammar's bound suffix).
        let grammar = declaration
            .grammar()
            .expect("keyword declarations carry grammar");
        let grammar_head = grammar.surfaces()[0].text();
        let (head, glued) = match spelling {
            [SpellingPart::Literal(head), ..] => (head.trim(), false),
            [SpellingPart::Param { .. }, SpellingPart::Literal(head), ..] => {
                (head.trim(), !head.starts_with(' '))
            }
            _ => panic!("{name}: the template has no head literal"),
        };
        if glued {
            assert!(
                grammar.surfaces().iter().any(|surface| {
                    surface.feature() == SurfaceFeature::BoundSuffix && surface.text() == head
                }),
                "{name}: glued head `{head}` is not the grammar's bound suffix"
            );
        } else {
            assert!(
                head.strip_prefix(grammar_head)
                    .is_some_and(|rest| rest.is_empty() || rest.starts_with(' ')),
                "{name}: template head `{head}` is not the grammar head `{grammar_head}`"
            );
        }

        // A quality the grammar declares plural is hinted plural.
        if let GrammarRecipe::FixedKeyword {
            parameter:
                Some(FixedKeywordParameterGrammar::Quality {
                    nominal_number: Some(FixedKeywordNominalNumber::Plural),
                    ..
                }),
        } = grammar.recipe()
        {
            assert!(
                spelling.contains(&SpellingPart::Param {
                    index: 0,
                    hint: Some(SpellingHint::Plural),
                }),
                "{name}: the grammar's plural quality is not hinted plural"
            );
        }
    }
}
