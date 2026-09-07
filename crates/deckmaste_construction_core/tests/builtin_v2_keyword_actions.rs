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
    assert!(
        graduated > 0,
        "no keyword action carries a semantic body; the family's graduation was lost"
    );

    let destroy = action(&declarations, "destroy");
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

    assert_eq!(
        action(&declarations, "connive").grammar().unwrap().recipe(),
        &GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::Amount]],
            },
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
        &GrammarRecipe::FixedTerm
    );
    assert_eq!(surfaces(waterbend), [(SurfaceFeature::Fixed, "waterbend")]);

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
                        CustomTailAtom::PredicativeComplement,
                    ],
                ],
            },
        }
    );
}
