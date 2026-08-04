//! The committed golden is the review surface for generated code: any
//! change to emission is a visible diff here, never a silent behavior
//! change. Regenerate with UPDATE_GOLDENS=1 and read the diff by eye.

use deckmaste_english_construction_compiler::emit::emit_group;
use deckmaste_english_construction_compiler::model::AstShape;
use deckmaste_english_construction_compiler::model::Constraint;
use deckmaste_english_construction_compiler::model::ConstructionDeclaration;
use deckmaste_english_construction_compiler::model::DominanceEdge;
use deckmaste_english_construction_compiler::model::ElementDeclaration;
use deckmaste_english_construction_compiler::model::FieldBinding;
use deckmaste_english_construction_compiler::model::FieldKind;
use deckmaste_english_construction_compiler::model::FieldPath;
use deckmaste_english_construction_compiler::model::FormDeclaration;
use deckmaste_english_construction_compiler::model::GroupDeclaration;
use deckmaste_english_construction_compiler::model::Predicate;
use deckmaste_english_construction_compiler::model::SelectionPromise;
use deckmaste_english_construction_compiler::model::Spanned;
use deckmaste_english_construction_compiler::model::SurfaceAtom;
use deckmaste_english_construction_compiler::model::WitnessClass;
use deckmaste_english_construction_compiler::model::WitnessDeclaration;
use deckmaste_english_construction_compiler::validate::validate;

/// The plan's fixture family, hand-built. Task 11 proves the DSL text
/// parses to exactly this value (spans aside).
pub fn fixture_coordination_group() -> GroupDeclaration {
    GroupDeclaration {
        name: Spanned::call_site("fixture_coordination".to_owned()),
        elements: vec![ElementDeclaration {
            name: Spanned::call_site("fixture_member".to_owned()),
            fields: vec![
                FieldBinding {
                    field: Spanned::call_site("comma".to_owned()),
                    kind: FieldKind::Optional {
                        inner: Box::new(FieldKind::Scalar {
                            codec: Spanned::call_site("Comma".to_owned()),
                        }),
                    },
                },
                FieldBinding {
                    field: Spanned::call_site("phrase".to_owned()),
                    kind: FieldKind::Subtree {
                        category: Spanned::call_site("FixturePhrase".to_owned()),
                        boxed: false,
                    },
                },
            ],
        }],
        constructions: vec![
            ConstructionDeclaration {
                id: Spanned::call_site("fixture_pair".to_owned()),
                category: Spanned::call_site("FixturePair".to_owned()),
                internal: false,
                ast: AstShape::Own {
                    name: Spanned::call_site("FixturePairNode".to_owned()),
                    fields: vec![
                        FieldBinding {
                            field: Spanned::call_site("members".to_owned()),
                            kind: FieldKind::Sequence {
                                element: Spanned::call_site("fixture_member".to_owned()),
                            },
                        },
                        FieldBinding {
                            field: Spanned::call_site("conjunction".to_owned()),
                            kind: FieldKind::Scalar {
                                codec: Spanned::call_site("Conjunction".to_owned()),
                            },
                        },
                    ],
                },
                constraints: vec![
                    Constraint::Require(Spanned::call_site(Predicate::LenAtLeast {
                        path: FieldPath::call_site("members"),
                        min: 2,
                    })),
                    Constraint::Require(Spanned::call_site(Predicate::In {
                        path: FieldPath::call_site("conjunction"),
                        allowed: vec!["And".to_owned(), "Or".to_owned()],
                    })),
                ],
                witnesses: vec![WitnessDeclaration {
                    name: Spanned::call_site("oxford".to_owned()),
                    class: WitnessClass::Stored {
                        path: FieldPath::call_site("members.last.comma"),
                    },
                }],
                forms: vec![
                    FormDeclaration {
                        name: Spanned::call_site("plain".to_owned()),
                        ordinal: Spanned::call_site(0),
                        surface: vec![
                            SurfaceAtom::Hole(FieldPath::call_site("members")),
                            SurfaceAtom::Lexeme(FieldPath::call_site("conjunction")),
                        ],
                        guard: Some(Spanned::call_site(Predicate::In {
                            path: FieldPath::call_site("conjunction"),
                            allowed: vec!["And".to_owned()],
                        })),
                    },
                    FormDeclaration {
                        name: Spanned::call_site("fancy".to_owned()),
                        ordinal: Spanned::call_site(1),
                        surface: vec![
                            SurfaceAtom::Hole(FieldPath::call_site("members")),
                            SurfaceAtom::Literal(Spanned::call_site(",".to_owned())),
                            SurfaceAtom::Lexeme(FieldPath::call_site("conjunction")),
                        ],
                        guard: Some(Spanned::call_site(Predicate::In {
                            path: FieldPath::call_site("conjunction"),
                            allowed: vec!["Or".to_owned()],
                        })),
                    },
                ],
                dominance: vec![DominanceEdge {
                    winner: Spanned::call_site("fixture_pair".to_owned()),
                    loser: Spanned::call_site("fixture_solo".to_owned()),
                }],
                selection: SelectionPromise::Packed,
                deserialize: false,
            },
            ConstructionDeclaration {
                id: Spanned::call_site("fixture_solo".to_owned()),
                category: Spanned::call_site("FixturePair".to_owned()),
                internal: false,
                ast: AstShape::Own {
                    name: Spanned::call_site("FixtureSoloNode".to_owned()),
                    fields: vec![
                        FieldBinding {
                            field: Spanned::call_site("phrase".to_owned()),
                            kind: FieldKind::Subtree {
                                category: Spanned::call_site("FixturePhrase".to_owned()),
                                boxed: false,
                            },
                        },
                        FieldBinding {
                            field: Spanned::call_site("alt".to_owned()),
                            kind: FieldKind::Optional {
                                inner: Box::new(FieldKind::Subtree {
                                    category: Spanned::call_site("FixturePhrase".to_owned()),
                                    boxed: false,
                                }),
                            },
                        },
                    ],
                },
                constraints: vec![Constraint::Require(Spanned::call_site(Predicate::IsNone {
                    path: FieldPath::call_site("alt"),
                }))],
                witnesses: vec![WitnessDeclaration {
                    name: Spanned::call_site("gap".to_owned()),
                    class: WitnessClass::Free {
                        ty: Spanned::call_site("Comma".to_owned()),
                    },
                }],
                forms: vec![FormDeclaration {
                    name: Spanned::call_site("only".to_owned()),
                    ordinal: Spanned::call_site(0),
                    surface: vec![
                        SurfaceAtom::Hole(FieldPath::call_site("phrase")),
                        SurfaceAtom::Hole(FieldPath::call_site("alt")),
                    ],
                    guard: None,
                }],
                dominance: vec![],
                selection: SelectionPromise::Unique,
                deserialize: true,
            },
        ],
    }
}

fn formatted_emission() -> String {
    let group = fixture_coordination_group();
    let validated = validate(&group).expect("the fixture family validates clean");
    let file: syn::File =
        syn::parse2(emit_group(&validated)).expect("emission parses as a Rust file");
    prettyplease::unparse(&file)
}

#[test]
fn emission_matches_committed_golden() {
    assert!(
        !(std::env::var_os("CI").is_some() && std::env::var_os("UPDATE_GOLDENS").is_some()),
        "UPDATE_GOLDENS must never run in CI: golden drift would rewrite-and-pass"
    );
    let formatted = formatted_emission();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/goldens/fixture_coordination.rs"
    );
    if std::env::var_os("UPDATE_GOLDENS").is_some() {
        std::fs::write(path, &formatted).expect("write golden");
    }
    let golden = std::fs::read_to_string(path)
        .expect("golden missing: rerun with UPDATE_GOLDENS=1 and review the diff");
    assert_eq!(
        formatted, golden,
        "regenerate with UPDATE_GOLDENS=1 and review the diff"
    );
}

#[test]
fn emission_is_byte_stable() {
    // Two independent full pipeline runs; catches any map-ordering or
    // interner nondeterminism anywhere in validate/emit/format.
    assert_eq!(formatted_emission(), formatted_emission());
}
