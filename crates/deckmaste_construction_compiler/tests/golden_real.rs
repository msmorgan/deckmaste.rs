//! The committed golden is the review surface for generated code: any
//! change to emission is a visible diff here, never a silent behavior
//! change. Regenerate with `UPDATE_GOLDENS=1` and read the diff by eye.

use deckmaste_construction_compiler::emit::emit_group;
use deckmaste_construction_compiler::model::AstShape;
use deckmaste_construction_compiler::model::Constraint;
use deckmaste_construction_compiler::model::ConstructionDeclaration;
use deckmaste_construction_compiler::model::DominanceEdge;
use deckmaste_construction_compiler::model::ElementDeclaration;
use deckmaste_construction_compiler::model::FieldBinding;
use deckmaste_construction_compiler::model::FieldKind;
use deckmaste_construction_compiler::model::FieldPath;
use deckmaste_construction_compiler::model::FormDeclaration;
use deckmaste_construction_compiler::model::GroupDeclaration;
use deckmaste_construction_compiler::model::Predicate;
use deckmaste_construction_compiler::model::SelectionPromise;
use deckmaste_construction_compiler::model::Spanned;
use deckmaste_construction_compiler::model::SurfaceAtom;
use deckmaste_construction_compiler::model::WitnessClass;
use deckmaste_construction_compiler::model::WitnessDeclaration;
use deckmaste_construction_compiler::validate::validate;

fn fixture_elements() -> Vec<ElementDeclaration> {
    vec![
        ElementDeclaration {
            name: Spanned::call_site("fixture_member".to_owned()),
            bind_path: None,
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
        },
        ElementDeclaration {
            name: Spanned::call_site("bound_fixture_member".to_owned()),
            bind_path: Some(Spanned::call_site("BoundMember".to_owned())),
            fields: vec![
                FieldBinding {
                    field: Spanned::call_site("comma".to_owned()),
                    kind: FieldKind::Scalar {
                        codec: Spanned::call_site("Comma".to_owned()),
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
        },
        ElementDeclaration {
            name: Spanned::call_site("empty_payload".to_owned()),
            bind_path: Some(Spanned::call_site("BoundPayload".to_owned())),
            fields: vec![],
        },
    ]
}

/// The plan's fixture family, hand-built. Task 11 proves the DSL text
/// parses to exactly this value (spans aside).
#[must_use]
pub fn fixture_coordination_group() -> GroupDeclaration {
    GroupDeclaration {
        name: Spanned::call_site("fixture_coordination".to_owned()),
        elements: fixture_elements(),
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
                    Constraint::Require(Spanned::call_site(Predicate::In {
                        path: FieldPath::call_site("members.last.comma"),
                        allowed: vec!["Present".to_owned()],
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

/// The plan's fixture family, as `constructions!` DSL text. Byte-identical
/// to `parse.rs`'s `fixture_dsl()` and to Task 13's `constructions!`
/// invocation — kept as a separate copy of the DSL *text* by design (see
/// Task 11's controller ruling); only the ~140-line hand-built IR literal
/// above is not duplicated.
fn fixture_dsl() -> proc_macro2::TokenStream {
    quote::quote! {
        group fixture_coordination;

        element fixture_member {
            comma: opt lex Comma,
            phrase: hole FixturePhrase,
        }

        element bound_fixture_member bind BoundMember {
            comma: lex Comma,
            phrase: hole FixturePhrase,
        }

        element empty_payload bind BoundPayload {}

        construction fixture_pair: FixturePair {
            own FixturePairNode {
                members: seq fixture_member,
                conjunction: lex Conjunction,
            }
            require members.len() >= 2;
            require conjunction in [And, Or];
            require members.last.comma in [Present];
            witness oxford = stored members.last.comma;
            form plain @ 0 when conjunction in [And] = members lex(conjunction);
            form fancy @ 1 when conjunction in [Or] = members "," lex(conjunction);
            dominates fixture_solo;
        }

        construction fixture_solo: FixturePair {
            own FixtureSoloNode {
                phrase: hole FixturePhrase,
                alt: opt hole FixturePhrase,
            }
            require alt.is_none();
            witness gap = free Comma;
            form only @ 0 = phrase alt;
            selection unique;
            deserialize;
        }
    }
}

/// Pins the parser and the golden to ONE declaration rather than two copies
/// that must be kept honest: the DSL text above parses to exactly the
/// hand-built `fixture_coordination_group()` IR (span-insensitive equality).
#[test]
fn fixture_family_parses_to_the_handbuilt_ir() {
    let parsed = deckmaste_construction_compiler::parse::parse_group(fixture_dsl())
        .expect("fixture DSL parses");
    assert_eq!(parsed, fixture_coordination_group());
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
