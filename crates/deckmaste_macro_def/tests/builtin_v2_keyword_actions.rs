use std::path::Path;

use deckmaste_macro_def::CustomTailAtom;
use deckmaste_macro_def::DeclarationKind;
use deckmaste_macro_def::DerivedSurface;
use deckmaste_macro_def::GrammarRecipe;
use deckmaste_macro_def::InflectionalForm;
use deckmaste_macro_def::NormalizedDeclaration;
use deckmaste_macro_def::RealizedSurface;
use deckmaste_macro_def::SpellingPart;
use deckmaste_macro_def::SurfaceFeature;
use deckmaste_macro_def::VerbFrameSet;
use deckmaste_macro_def::read_builtin_v2;

const EXPECTED_NAMES: &[&str] = &[
    "activate",
    "adapt",
    "airbend",
    "amass",
    "assemble",
    "attach",
    "behold",
    "blight",
    "bolster",
    "cast",
    "clash",
    "cloak",
    "collectEvidence",
    "connive",
    "conniveN",
    "convert",
    "counter",
    "create",
    "destroy",
    "detain",
    "discard",
    "discover",
    "double",
    "earthbend",
    "endure",
    "exchange",
    "exert",
    "exile",
    "explore",
    "faceAVillainousChoice",
    "fateseal",
    "fight",
    "forage",
    "goad",
    "harness",
    "heal",
    "incubate",
    "investigate",
    "learn",
    "manifest",
    "manifestDread",
    "meld",
    "mill",
    "monstrosity",
    "play",
    "populate",
    "proliferate",
    "recruit",
    "regenerate",
    "reveal",
    "sacrifice",
    "scry",
    "search",
    "shuffle",
    "support",
    "surveil",
    "suspect",
    "tap",
    "theRingTemptsYou",
    "timeTravel",
    "transform",
    "triple",
    "untap",
    "ventureIntoTheDungeon",
    "vote",
    "waterbend",
];

fn action<'a>(declarations: &'a [NormalizedDeclaration], name: &str) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::KeywordAction
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing keyword action {name}"))
}

fn surfaces(declaration: &NormalizedDeclaration) -> Vec<(SurfaceFeature, &str)> {
    declaration
        .grammar()
        .expect("keyword action must contribute grammar")
        .surfaces()
        .iter()
        .map(|surface| (surface.feature(), surface.text()))
        .collect()
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the nursery authority compares every normalized keyword-action declaration together"
)]
fn builtin_v2_keyword_action_nursery_is_complete_and_normalized() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let actions = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::KeywordAction)
        .collect::<Vec<_>>();

    assert_eq!(
        actions
            .iter()
            .map(|declaration| declaration.identity().name())
            .collect::<Vec<_>>(),
        EXPECTED_NAMES
    );
    // The family graduates declaration by declaration
    // (`semantics-v2-macro-bodies-keyword-actions`): each one either is still a
    // grammar-only nursery record or is fully graduated. Half-graduation — a
    // body with no positional signature, or a signature with no body — is what
    // this must never admit, and every declaration keeps its spelling and
    // grammar either way.
    let mut graduated = 0usize;
    for declaration in &actions {
        if declaration.is_graduated() {
            graduated += 1;
        } else {
            assert!(
                declaration.body().is_none(),
                "{} carries a body without a positional signature",
                declaration.identity()
            );
        }
        // Re-spelled for `builtin-v2-keyword-spelling-templates`: the spelling
        // is the card-usage template, so its head literal is the grammar head
        // and every hole names a declared parameter (the full shape is
        // `builtin_v2_keyword_spelling_templates.rs`). Re-spelled again for
        // `builtin-v2-keyword-spelling-templates-residue`: a predicate
        // modifier the card prints before the verb takes a leading hole
        // ("<Param(0)> vote for <Param(1)>", "secretly votes for"), so the
        // head literal is the first literal, separated from that hole.
        let leading_hole = matches!(
            declaration.spelling().first(),
            Some(SpellingPart::Param { .. })
        );
        let Some(head) = declaration.spelling().iter().find_map(|part| match part {
            SpellingPart::Literal(literal) => Some(literal),
            SpellingPart::Param { .. } => None,
        }) else {
            panic!("{} must spell its head literal", declaration.identity())
        };
        assert!(
            !leading_hole || head.starts_with(' '),
            "{} glues its head literal to a leading hole",
            declaration.identity()
        );
        let head = head.trim_start();
        let grammar_head = declaration
            .grammar()
            .map(|grammar| grammar.surfaces()[0].text());
        assert!(
            grammar_head.is_some_and(|grammar_head| head
                .trim_end()
                .strip_prefix(grammar_head)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))),
            "{} spelling head `{head}` is not its grammar head",
            declaration.identity()
        );
        let len = declaration.params().map_or(0, <[_]>::len);
        assert!(
            declaration
                .spelling()
                .iter()
                .all(|part| !matches!(part, SpellingPart::Param { index, .. } if *index >= len)),
            "{} spelling names a parameter it does not declare",
            declaration.identity()
        );
        assert!(
            declaration.grammar().is_some(),
            "{} must contribute grammar",
            declaration.identity()
        );
    }
    assert!(
        graduated > 0,
        "no keyword action carries a semantic body; the family's graduation was lost"
    );

    let destroy = action(&declarations, "destroy");
    assert_eq!(
        destroy.spelling(),
        [
            SpellingPart::Literal("destroy ".to_owned()),
            SpellingPart::Param {
                index: 0,
                hint: None,
            },
        ]
    );
    assert_eq!(
        destroy.grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Transitive,
        }
    );
    assert_eq!(
        surfaces(destroy),
        [
            (SurfaceFeature::PLAIN, "destroy"),
            (SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, "destroys"),
            (SurfaceFeature::PAST_PARTICIPLE, "destroyed"),
        ]
    );

    assert_eq!(
        surfaces(action(&declarations, "search")),
        [
            (SurfaceFeature::PLAIN, "search"),
            (SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, "searches"),
            (SurfaceFeature::PRETERITE, "searched"),
            (SurfaceFeature::PAST_PARTICIPLE, "searched"),
        ],
        "homographic Inflectional Forms remain distinct declaration rows",
    );

    assert_eq!(
        action(&declarations, "explore").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::ObjectNounPhrase]],
            },
        }
    );

    let regenerate = action(&declarations, "regenerate");
    assert_eq!(
        regenerate.grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Transitive,
        }
    );
    assert_eq!(
        surfaces(regenerate),
        [
            (SurfaceFeature::PLAIN, "regenerate"),
            (SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, "regenerates"),
            (SurfaceFeature::PAST_PARTICIPLE, "regenerated"),
        ]
    );

    let scry = action(&declarations, "scry");
    assert_eq!(
        scry.grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![],
                    vec![CustomTailAtom::Amount],
                    vec![CustomTailAtom::ObjectNounPhrase],
                ],
            },
        }
    );
    assert_eq!(
        surfaces(scry),
        [
            (SurfaceFeature::PLAIN, "scry"),
            (SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, "scries"),
            (SurfaceFeature::PAST_PARTICIPLE, "scryed"),
        ]
    );
    assert!(
        scry.grammar()
            .unwrap()
            .surfaces()
            .iter()
            .all(|surface| surface.text() != "scrys")
    );

    // Bare "connive" [CR#701.50a] and "connive N" [CR#701.50d] are two
    // declarations sharing the head; the frame selects between them.
    assert_eq!(
        action(&declarations, "connive").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Intransitive,
        }
    );
    assert_eq!(
        action(&declarations, "conniveN")
            .grammar()
            .unwrap()
            .recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::MeasureComplement,
        }
    );

    assert_eq!(
        action(&declarations, "vote").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![],
                    vec![CustomTailAtom::ObjectNounPhrase],
                    vec![
                        CustomTailAtom::Lex("Preposition".to_owned(), "For".to_owned()),
                        CustomTailAtom::ObjectNounPhrase,
                    ],
                ],
            },
        }
    );

    let manifest_dread = action(&declarations, "manifestDread");
    assert_eq!(
        surfaces(manifest_dread),
        [
            (SurfaceFeature::PLAIN, "manifest dread"),
            (
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "manifests dread"
            ),
            (SurfaceFeature::PAST_PARTICIPLE, "manifest dreaded"),
        ]
    );

    let waterbend = action(&declarations, "waterbend");
    assert_eq!(
        waterbend.grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::Role("CostSymbols".into())]],
            },
        }
    );
    assert_eq!(
        surfaces(waterbend),
        [
            (SurfaceFeature::PLAIN, "waterbend"),
            (SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, "waterbends"),
            (SurfaceFeature::PAST_PARTICIPLE, "waterbended"),
        ]
    );

    let ring = action(&declarations, "theRingTemptsYou");
    assert_eq!(
        ring.spelling(),
        [SpellingPart::Literal("the Ring tempts you".to_owned())]
    );
    assert_eq!(
        ring.grammar().unwrap().recipe(),
        &GrammarRecipe::FixedClause
    );
    assert_eq!(
        surfaces(ring),
        [(SurfaceFeature::Fixed, "the Ring tempts you")]
    );
    assert_eq!(
        ring.grammar().unwrap().verb(),
        Some(&deckmaste_macro_def::KeywordVerbGrammar {
            bare: "tempt".into(),
            frames: vec![deckmaste_macro_def::VerbDeclarationFrame::Predicate(vec![
                deckmaste_macro_def::FrameItem::Argument(deckmaste_macro_def::FrameComplement {
                    relation: deckmaste_macro_def::FrameRelation::Object,
                    category: "NounPhrase".into(),
                },),
            ])],
            preterite: DerivedSurface::default(),
            participle: DerivedSurface::default(),
            gerund_participle: DerivedSurface::default(),
        }),
    );
}

#[test]
fn exchange_has_every_attested_representable_tail_shape() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "exchange")
            .grammar()
            .unwrap()
            .recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![CustomTailAtom::ObjectNounPhrase],
                    vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::Lex("Preposition".to_owned(), "With".to_owned()),
                        CustomTailAtom::ObjectNounPhrase,
                    ],
                    vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::Lex("Preposition".to_owned(), "For".to_owned()),
                        CustomTailAtom::ObjectNounPhrase,
                    ],
                ],
            },
        }
    );
}

#[test]
fn shuffle_has_every_attested_representable_tail_shape() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "shuffle").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![],
                    vec![CustomTailAtom::ObjectNounPhrase],
                    vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::Lex("Preposition".to_owned(), "Into".to_owned()),
                        CustomTailAtom::ObjectNounPhrase,
                    ],
                ],
            },
        }
    );
}

#[test]
fn exile_declares_its_object_resultative_frame() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "exile").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![CustomTailAtom::ObjectNounPhrase],
                    vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::Role("ResultativeComplement".into()),
                    ],
                ],
            },
        }
    );
}

/// The derived past participle handles the `-e` rule (`exile` → `exiled`) but
/// not consonant doubling: a one-syllable stem ending in a single vowel and
/// consonant (`tap` → `tapped`) must declare its participle instead
/// (`keyword-actions-tap-untap-participle`).
#[test]
fn keyword_actions_with_doubling_stems_declare_their_participle() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
    let mut wrong = Vec::new();
    for declaration in declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::KeywordAction)
    {
        let Some(grammar) = declaration.grammar() else {
            continue;
        };
        let surface = |feature: SurfaceFeature| {
            grammar
                .surfaces()
                .iter()
                .find(|surface| surface.feature() == feature)
                .map(RealizedSurface::text)
        };
        let (Some(bare), Some(participle)) = (
            surface(SurfaceFeature::PLAIN),
            surface(SurfaceFeature::Inflectional(
                InflectionalForm::PastParticiple,
            )),
        ) else {
            continue;
        };
        // The last word of the stem, syllables counted as vowel groups.
        let word = bare.rsplit(' ').next().unwrap_or(bare);
        let chars: Vec<char> = word.chars().collect();
        let syllables = chars
            .iter()
            .enumerate()
            .filter(|(i, c)| is_vowel(**c) && (*i == 0 || !is_vowel(chars[i - 1])))
            .count();
        let doubling = syllables == 1
            && chars.len() >= 2
            && !is_vowel(chars[chars.len() - 1])
            && !matches!(chars[chars.len() - 1], 'w' | 'x' | 'y')
            && is_vowel(chars[chars.len() - 2])
            && (chars.len() == 2 || !is_vowel(chars[chars.len() - 3]));
        if doubling && participle == format!("{bare}ed") {
            wrong.push(format!("{}: {participle}", declaration.identity().name()));
        }
    }
    assert!(
        wrong.is_empty(),
        "derived participles that need an override: {wrong:?}"
    );
}
