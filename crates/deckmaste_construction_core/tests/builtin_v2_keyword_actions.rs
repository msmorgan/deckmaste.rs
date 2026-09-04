use std::path::Path;

use deckmaste_construction_core::macro_def::CustomTailAtom;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::VerbFrameSet;
use deckmaste_construction_core::macro_def::read_builtin_v2;

const EXPECTED_NAMES: &[&str] = &[
    "Abandon",
    "Activate",
    "Adapt",
    "Airbend",
    "Amass",
    "Assemble",
    "Attach",
    "Behold",
    "Blight",
    "Bolster",
    "Cast",
    "Clash",
    "Cloak",
    "CollectEvidence",
    "Connive",
    "Convert",
    "Counter",
    "Create",
    "Destroy",
    "Detain",
    "Discard",
    "Discover",
    "Double",
    "Earthbend",
    "Endure",
    "Exchange",
    "Exert",
    "Exile",
    "Explore",
    "FaceAVillainousChoice",
    "Fateseal",
    "Fight",
    "Forage",
    "Goad",
    "Harness",
    "Heal",
    "Incubate",
    "Investigate",
    "Learn",
    "Manifest",
    "ManifestDread",
    "Meld",
    "Mill",
    "Monstrosity",
    "OpenAnAttraction",
    "Planeswalk",
    "Play",
    "Populate",
    "Proliferate",
    "Recruit",
    "Regenerate",
    "Reveal",
    "RollToVisitYourAttractions",
    "Sacrifice",
    "Scry",
    "Search",
    "SetInMotion",
    "Shuffle",
    "Support",
    "Surveil",
    "Suspect",
    "Tap",
    "TheRingTemptsYou",
    "TimeTravel",
    "Transform",
    "Triple",
    "Untap",
    "VentureIntoTheDungeon",
    "Vote",
    "Waterbend",
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
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
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
    for declaration in &actions {
        assert!(
            !declaration.is_graduated(),
            "{} must remain a nursery declaration",
            declaration.identity()
        );
        assert!(
            matches!(declaration.spelling(), [SpellingPart::Literal(_)]),
            "{} must have one literal spelling part",
            declaration.identity()
        );
        assert!(
            declaration.grammar().is_some(),
            "{} must contribute grammar",
            declaration.identity()
        );
    }

    let destroy = action(&declarations, "Destroy");
    assert_eq!(
        destroy.spelling(),
        [SpellingPart::Literal("destroy".to_owned())]
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
        action(&declarations, "Explore").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::ObjectNounPhrase]],
            },
        }
    );

    let regenerate = action(&declarations, "Regenerate");
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

    let scry = action(&declarations, "Scry");
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

    assert_eq!(
        action(&declarations, "Connive").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::Amount]],
            },
        }
    );

    assert_eq!(
        action(&declarations, "Vote").grammar().unwrap().recipe(),
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

    let manifest_dread = action(&declarations, "ManifestDread");
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

    let set_in_motion = action(&declarations, "SetInMotion");
    assert_eq!(
        set_in_motion.grammar().unwrap().recipe(),
        &GrammarRecipe::FixedTerm
    );
    assert_eq!(
        surfaces(set_in_motion),
        [(SurfaceFeature::Fixed, "set in motion")]
    );

    let waterbend = action(&declarations, "Waterbend");
    assert_eq!(
        waterbend.grammar().unwrap().recipe(),
        &GrammarRecipe::FixedTerm
    );
    assert_eq!(surfaces(waterbend), [(SurfaceFeature::Fixed, "waterbend")]);

    let ring = action(&declarations, "TheRingTemptsYou");
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
}

#[test]
fn roll_to_visit_your_attractions_has_its_attested_agreeing_surface() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        surfaces(action(&declarations, "RollToVisitYourAttractions")),
        [
            (SurfaceFeature::PLAIN, "roll to visit your Attractions"),
            (
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "rolls to visit their Attractions",
            ),
            (
                SurfaceFeature::PAST_PARTICIPLE,
                "roll to visit your Attractionsed",
            ),
        ]
    );
}

#[test]
fn exchange_has_every_attested_representable_tail_shape() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "Exchange")
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
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "Shuffle").grammar().unwrap().recipe(),
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
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    assert_eq!(
        action(&declarations, "Exile").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![
                    vec![CustomTailAtom::ObjectNounPhrase],
                    vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::PredicativeComplement,
                    ],
                ],
            },
        }
    );
}
