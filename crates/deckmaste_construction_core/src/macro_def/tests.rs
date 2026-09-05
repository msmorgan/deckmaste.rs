use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::*;

const SCRY: &str = r#"
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(0)>",
    grammar: Verb(
        bare: "scry",
        third_person: "scries",
        frame_set: MeasureComplement,
    ),
    body: Scry(Param(0)),
)
"#;

const DESTROY: &str = r#"
KeywordAction(
    name: "Destroy",
    spelling: "destroy",
    grammar: Verb(
        bare: "destroy",
        frame_set: Transitive,
    ),
)
"#;

fn source_path(name: &str) -> PathBuf {
    Path::new("/synthetic").join(name)
}

fn validation(source: &str) -> ValidationError {
    let path = source_path("Broken.ron");
    let error = read_str(path.clone(), source).expect_err("source must fail");
    assert_eq!(error.path(), path);
    let position = error.position().expect("validation has a source position");
    assert!(position.line >= 1);
    assert!(position.column >= 1);
    match error {
        ReadError::Validate { source, .. } => source,
        ReadError::Io { .. } | ReadError::Parse { .. } => {
            panic!("expected validation error, got {error}")
        }
    }
}

fn parse_error(source: &str) -> ReadError {
    let path = source_path("Broken.ron");
    let error = read_str(path.clone(), source).expect_err("source must fail");
    assert_eq!(error.path(), path);
    let position = error.position().expect("parse error has a source span");
    assert!(position.line >= 1);
    assert!(position.column >= 1);
    assert!(matches!(error, ReadError::Parse { .. }));
    error
}

fn assert_located(error: &ReadError, path: &Path) {
    assert_eq!(error.path(), path);
    let position = error.position().expect("error has a source position");
    assert!(position.line >= 1 && position.column >= 1);
}

fn texts(declaration: &NormalizedDeclaration) -> Vec<&str> {
    declaration
        .grammar
        .as_ref()
        .unwrap()
        .surfaces
        .iter()
        .map(|surface| surface.text.as_str())
        .collect()
}

#[test]
fn normalization_freezes_bounded_and_authored_onsets_per_realized_form() {
    let cases = [
        ("Artifact", "artifact", Onset::Vowel),
        ("Player", "player", Onset::Consonant),
        ("Honor", "honor", Onset::Vowel),
        ("Unit", "unit", Onset::Consonant),
        ("Euphemism", "euphemism", Onset::Consonant),
        ("One", "one", Onset::Consonant),
        ("OneTime", "one-time", Onset::Consonant),
        ("Onerous", "onerous", Onset::Vowel),
        ("Oneiric", "oneiric", Onset::Vowel),
        ("X", "X", Onset::Vowel),
        ("B", "B", Onset::Consonant),
        ("MVP", "MVP", Onset::Vowel),
        ("CPU", "CPU", Onset::Consonant),
        ("Nonartifact", "nonartifact", Onset::Consonant),
    ];
    for (name, surface, expected) in cases {
        let source = format!(
            "Type(name:\"{name}\",spelling:\"{surface}\",grammar:FixedTerm(surface:\"{surface}\"))"
        );
        let normalized = read_str(source_path(&format!("{name}.ron")), &source).unwrap();
        let [realized] = normalized.grammar().unwrap().surfaces() else {
            panic!("one fixed surface is normalized")
        };
        assert_eq!(realized.onset(), expected, "{surface}");
        assert_eq!(realized.onset_override(), None, "{surface}");
    }

    let overridden = read_str(
        source_path("Aether.ron"),
        r#"Type(
            name:"Aether",
            spelling:"Æther",
            grammar:FixedTerm(surface:"Æther",onset:Vowel),
        )"#,
    )
    .unwrap();
    let [realized] = overridden.grammar().unwrap().surfaces() else {
        panic!("one overridden surface is normalized")
    };
    assert_eq!(realized.onset(), Onset::Vowel);
    assert_eq!(realized.onset_override(), Some(Onset::Vowel));

    assert!(matches!(
        validation(
            r#"Type(name:"Aether",spelling:"Æther",grammar:FixedTerm(surface:"Æther"))"#
        ),
        ValidationError::UnknownOnset { surface } if surface == "Æther"
    ));
}

#[test]
fn normalized_runtime_carrier_exposes_identity_and_position() {
    let identity = DeclarationIdentity::new(DeclarationKind::KeywordAction, "Scry");
    assert_eq!(identity.kind(), DeclarationKind::KeywordAction);
    assert_eq!(identity.name(), "Scry");

    let declaration = read_str(source_path("Scry.ron"), SCRY).unwrap();
    assert_eq!(
        declaration.grammar().unwrap().recipe().position(),
        GrammarPosition::Verb
    );
}

#[test]
fn source_schema_rejects_legacy_and_unknown_fields() {
    for extra in [
        r#"template: "scry <Param(0)>","#,
        r#"frames: ["scry <Param(0)>"],"#,
        r#"separator: "—","#,
        "layout: Dash,",
        r"kinds: [OneShotEffect],",
        r#"callback: "plugin_hook","#,
    ] {
        let source = format!("KeywordAction(name: \"Scry\", spelling: \"scry\", {extra})");
        parse_error(&source);
    }

    parse_error(
        r#"KeywordAction(
            name: "Scry",
            spelling: "scry",
            grammar: Verb(
                bare: "scry",
                frame_set: MeasureComplement,
                repetition: "*",
            ),
        )"#,
    );
}

#[test]
fn nursery_and_graduated_sources_use_the_ordinary_macro_reader() {
    let reader = declaration_reader().unwrap();
    for source in [
        r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:FixedKeyword(surface:"flying"))"#,
        r#"Subtype(category:Creature,name:"Merfolk",spelling:"Merfolk",grammar:Noun(singular:"Merfolk",plural:"Merfolk"))"#,
        r#"Type(name:"Creature",spelling:"creature",grammar:Noun(singular:"creature"))"#,
        r#"CounterKind(name:"Stun",spelling:"stun",grammar:FixedTerm(surface:"stun"))"#,
        r#"Designation(name:"Monarch",spelling:"the monarch",grammar:FixedTerm(surface:"the monarch"))"#,
        r#"KeywordAction(name:"Ping",params:[],spelling:"ping",body:Ping)"#,
        SCRY,
    ] {
        let definition: macro_ron::MacroDef<Metadata> = reader.read_str(source).unwrap();
        assert_eq!(definition.kinds.len(), 1, "{source}");
        assert!(!definition.metadata().spelling.is_empty(), "{source}");
        assert_eq!(
            matches!(&definition.params, macro_ron::Params::Positional(params) if params.is_empty()),
            !source.contains("params:") || source.contains("params:[]"),
            "{source}"
        );
    }
}

#[test]
fn morphology_uses_only_dumb_defaults_and_whole_surface_replacements() {
    let destroy = read_str(source_path("Destroy.ron"), DESTROY).unwrap();
    assert_eq!(texts(&destroy), ["destroy", "destroys", "destroyed"]);

    let scry = read_str(source_path("Scry.ron"), SCRY).unwrap();
    assert_eq!(texts(&scry), ["scry", "scries", "scryed"]);

    let turn_face_up = read_str(
        source_path("TurnFaceUp.ron"),
        r#"KeywordAction(name:"TurnFaceUp",spelling:"turn face up",grammar:Verb(bare:"turn face up",third_person:"turns face up",frame_set:Transitive))"#,
    )
    .unwrap();
    assert_eq!(
        texts(&turn_face_up),
        ["turn face up", "turns face up", "turn face uped"]
    );

    for (name, singular) in [("Sheep", "sheep"), ("Merfolk", "Merfolk")] {
        let source = format!(
            "Subtype(category:Creature,name:\"{name}\",spelling:\"{singular}\",grammar:Noun(singular:\"{singular}\",plural:\"{singular}\"))"
        );
        let declaration = read_str(source_path(&format!("{name}.ron")), &source).unwrap();
        assert_eq!(texts(&declaration), [singular, singular]);
    }

    assert!(matches!(
        validation(r#"Type(name:"Player",spelling:"player",grammar:Noun(singular:"player",plural:"players"))"#),
        ValidationError::RedundantOverride { field: "plural", surface } if surface == "players"
    ));
}

#[test]
fn verb_participles_use_one_regular_surface_and_explicit_whole_surface_overrides() {
    let deal = read_str(
        source_path("Deal.ron"),
        r#"KeywordAction(
            name:"Deal",
            spelling:"deal",
            grammar:Verb(
                bare:"deal",
                participle:"dealt",
                frame_set:Transitive,
            ),
        )"#,
    )
    .expect("an irregular participle is a finite declaration surface override");
    assert_eq!(texts(&deal), ["deal", "deals", "dealt"]);

    let turn = read_str(
        source_path("Turn.ron"),
        r#"KeywordAction(
            name:"Turn",
            spelling:"turn",
            grammar:Verb(bare:"turn",frame_set:Transitive),
        )"#,
    )
    .expect("regular participles derive without a declaration callback");
    assert_eq!(texts(&turn), ["turn", "turns", "turned"]);
}

#[test]
fn graduated_declaration_expands_and_normalizes() {
    let reader = declaration_reader().unwrap();
    let definition: macro_ron::MacroDef<Metadata> = reader.read_str(SCRY).unwrap();
    assert_eq!(definition.kinds, [macro_ron::Ident::from("KeywordAction")]);
    assert!(matches!(
        &definition.params,
        macro_ron::Params::Positional(params) if params.len() == 1
    ));

    let declaration = read_str(source_path("Scry.ron"), SCRY).unwrap();
    assert!(declaration.is_graduated());
    assert_eq!(
        declaration.identity,
        DeclarationIdentity {
            kind: DeclarationKind::KeywordAction,
            name: "Scry".to_owned(),
        }
    );
    assert_eq!(
        declaration.spelling,
        vec![
            SpellingPart::Literal("scry ".to_owned()),
            SpellingPart::Param(0),
        ]
    );
    let row = declaration.grammar.unwrap();
    assert_eq!(
        row.recipe,
        GrammarRecipe::Verb {
            frame_set: VerbFrameSet::MeasureComplement,
        }
    );
    assert_eq!(
        row.surfaces,
        vec![
            RealizedSurface {
                feature: SurfaceFeature::PLAIN,
                text: "scry".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
            RealizedSurface {
                feature: SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                text: "scries".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
            RealizedSurface {
                feature: SurfaceFeature::PAST_PARTICIPLE,
                text: "scryed".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
        ]
    );
    assert!(!row.surfaces.iter().any(|surface| surface.text == "scrys"));
}

#[test]
fn nursery_declaration_uses_dumb_verb_morphology() {
    let declaration = read_str(source_path("Destroy.ron"), DESTROY).unwrap();
    assert!(!declaration.is_graduated());
    let row = declaration.grammar.unwrap();
    assert_eq!(
        row.surfaces,
        vec![
            RealizedSurface {
                feature: SurfaceFeature::PLAIN,
                text: "destroy".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
            RealizedSurface {
                feature: SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                text: "destroys".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
            RealizedSurface {
                feature: SurfaceFeature::PAST_PARTICIPLE,
                text: "destroyed".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
        ]
    );
}

#[test]
fn noun_override_replaces_default_and_unavailable_suppresses_it() {
    let merfolk = read_str(
        source_path("Merfolk.ron"),
        r#"
Subtype(
    category: Creature,
    name: "Merfolk",
    spelling: "Merfolk",
    grammar: Noun(singular: "Merfolk", plural: "Merfolk"),
)
"#,
    )
    .unwrap();
    assert_eq!(
        merfolk.grammar.unwrap().surfaces,
        vec![
            RealizedSurface {
                feature: SurfaceFeature::Singular,
                text: "Merfolk".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
            RealizedSurface {
                feature: SurfaceFeature::Plural,
                text: "Merfolk".to_owned(),
                onset: Onset::Consonant,
                onset_override: None,
            },
        ]
    );

    let proper_name = read_str(
        source_path("Jace.ron"),
        r#"
Subtype(
    category: Planeswalker,
    name: "Jace",
    spelling: "Jace",
    grammar: Noun(singular: "Jace", plural: Unavailable),
)
"#,
    )
    .unwrap();
    assert_eq!(
        proper_name.grammar.unwrap().surfaces,
        vec![RealizedSurface {
            feature: SurfaceFeature::Singular,
            text: "Jace".to_owned(),
            onset: Onset::Consonant,
            onset_override: None,
        }]
    );
}

#[test]
fn redundant_and_invalid_morphology_are_rejected() {
    let error = validation(
        r#"
Type(
    name: "Player",
    spelling: "player",
    grammar: Noun(singular: "player", plural: "players"),
)
"#,
    );
    assert!(matches!(
        error,
        ValidationError::RedundantOverride {
            field: "plural",
            surface,
        } if surface == "players"
    ));

    let error = validation(
        r#"
KeywordAction(
    name: "Scry",
    spelling: "scry",
    grammar: Verb(
        bare: "scry",
        third_person: "",
        frame_set: MeasureComplement,
    ),
)
"#,
    );
    assert!(matches!(
        error,
        ValidationError::InvalidSurface {
            field: "third_person"
        }
    ));

    parse_error(
        r#"
KeywordAction(
    name: "Scry",
    spelling: "scry",
    grammar: Verb(bare: "scry", third_person: 3, frame_set: MeasureComplement),
)
"#,
    );
}

#[test]
fn custom_frame_set_has_exact_finite_atom_shapes() {
    let expected = [
        VerbFrameSet::Intransitive,
        VerbFrameSet::Transitive,
        VerbFrameSet::MeasureComplement,
        VerbFrameSet::Custom {
            frames: vec![
                vec![],
                vec![
                    CustomTailAtom::Literal("with".to_owned()),
                    CustomTailAtom::Lex("Preposition".to_owned(), "For".to_owned()),
                    CustomTailAtom::Amount,
                    CustomTailAtom::ObjectNounPhrase,
                    CustomTailAtom::PredicativeComplement,
                ],
            ],
        },
    ];
    for (index, frame_set) in [
        "Intransitive",
        "Transitive",
        "MeasureComplement",
        "Custom(frames: [[], [Literal(\"with\"), Lex(\"Preposition\", \"For\"), Amount, ObjectNounPhrase, PredicativeComplement]])",
    ]
    .into_iter()
    .enumerate()
    {
        let source = format!(
            "KeywordAction(name:\"Verb{index}\",spelling:\"verb{index}\",grammar:Verb(bare:\"verb{index}\",frame_set:{frame_set}))"
        );
        let declaration = read_str(source_path(&format!("Verb{index}.ron")), &source).unwrap();
        assert_eq!(
            declaration.grammar.unwrap().recipe,
            GrammarRecipe::Verb {
                frame_set: expected[index].clone(),
            }
        );
    }

    let connive = read_str(
        source_path("Connive.ron"),
        r#"
KeywordAction(
    name: "Connive",
    spelling: "connive",
    grammar: Verb(
        bare: "connive",
        frame_set: Custom(frames: [[], [Amount]]),
    ),
)
"#,
    )
    .unwrap();
    assert_eq!(
        connive.grammar.unwrap().recipe,
        GrammarRecipe::Verb {
            frame_set: VerbFrameSet::Custom {
                frames: vec![vec![], vec![CustomTailAtom::Amount]],
            },
        }
    );

    let error = validation(
        r#"
KeywordAction(
    name: "Connive",
    spelling: "connive",
    grammar: Verb(bare: "connive", frame_set: Custom(frames: [])),
)
"#,
    );
    assert!(matches!(error, ValidationError::EmptyCustomVerbFrameSet));

    let error = validation(
        r#"
KeywordAction(
    name: "Connive",
    spelling: "connive",
    grammar: Verb(
        bare: "connive",
        frame_set: Custom(frames: [[Amount], [Amount]]),
    ),
)
"#,
    );
    assert!(matches!(error, ValidationError::DuplicateVerbFrame { .. }));

    let error = validation(
        r#"
KeywordAction(
    name: "Clash",
    spelling: "clash",
    grammar: Verb(
        bare: "clash",
        frame_set: Custom(frames: [[Literal("")]]),
    ),
)
"#,
    );
    assert!(matches!(error, ValidationError::InvalidCustomLiteral));

    parse_error(
        r#"
KeywordAction(
    name: "Clash",
    spelling: "clash",
    grammar: Verb(
        bare: "clash",
        frame_set: Custom(frames: [[Clause]]),
    ),
)
"#,
    );
    parse_error(
        r#"
KeywordAction(
    name: "Clash",
    spelling: "clash",
    grammar: Verb(bare: "clash"),
)
"#,
    );
    for literal in ["Literal(\" \")", "Literal(\"line\\nbreak\")"] {
        let source = format!(
            "KeywordAction(name:\"Clash\",spelling:\"clash\",grammar:Verb(bare:\"clash\",frame_set:Custom(frames:[[{literal}]])))"
        );
        assert!(matches!(
            validation(&source),
            ValidationError::InvalidCustomLiteral
        ));
    }
}

#[test]
fn spelling_and_body_holes_are_positional_and_bounded() {
    let error = validation(
        r#"
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(1)>",
)
"#,
    );
    assert!(matches!(
        error,
        ValidationError::ParamOutOfRange {
            location: "spelling",
            index: 1,
            len: 1,
        }
    ));

    let error = validation(
        r#"
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(0)>",
    body: Scry(Param(amount)),
)
"#,
    );
    assert!(
        matches!(error, ValidationError::InvalidBody { reason } if reason.contains("positional"))
    );

    let error = validation(
        r#"
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(0)>",
    body: Scry(Param(1)),
)
"#,
    );
    assert!(matches!(
        error,
        ValidationError::ParamOutOfRange {
            location: "body",
            index: 1,
            len: 1,
        }
    ));

    let error = validation(
        r#"
KeywordAction(
    name: "Scry",
    spelling: "scry",
    body: Scry(1),
)
"#,
    );
    assert!(matches!(error, ValidationError::BodyWithoutSignature));

    for spelling in [
        "scry <Param(amount)>",
        "scry <Param(00)>",
        "scry <Param(0)",
        "scry >",
    ] {
        let source = format!(
            r#"
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "{spelling}",
)
"#
        );
        assert!(matches!(
            validation(&source),
            ValidationError::InvalidSpelling { .. }
        ));
    }
}

#[test]
fn keyword_parameter_signatures_are_closed_or_explicitly_deferred() {
    let unknown_type = validation(
        r#"KeywordAbility(
    name: "Quorbling",
    params: [Mystery],
    spelling: "quorbling",
    grammar: FixedKeyword(surface: "quorbling"),
)"#,
    );
    assert_eq!(
        unknown_type,
        ValidationError::UnknownParameterType {
            name: "Mystery".to_owned(),
        }
    );

    let unsupported_vector = validation(
        r#"KeywordAbility(
    name: "Quorbling",
    params: [Cost, Amount],
    spelling: "quorbling",
    grammar: FixedKeyword(surface: "quorbling"),
)"#,
    );
    assert_eq!(
        unsupported_vector,
        ValidationError::UnsupportedKeywordParameterSignature {
            signature: "Cost, Amount".to_owned(),
        }
    );
}

#[test]
fn formatted_body_holes_are_validation_errors_at_the_body_value() {
    for (body, expected) in [
        (
            "Scry(Param( /* the positional index */ 1 ))",
            ValidationError::ParamOutOfRange {
                location: "body",
                index: 1,
                len: 1,
            },
        ),
        (
            "Scry(Param( /* named holes are forbidden */ amount ))",
            ValidationError::InvalidBody {
                reason: "holes `Param(amount)`, but v2 declarations are positional".to_owned(),
            },
        ),
    ] {
        let source = format!(
            r#"KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(0)>",
    body: {body},
)"#
        );
        let error = read_str(source_path("Scry.ron"), &source).unwrap_err();
        assert_eq!(
            error.position(),
            Some(SourcePosition {
                line: 5,
                column: 11,
            })
        );
        assert_eq!(error.validation(), Some(&expected));
    }
}

#[test]
fn validation_locations_ignore_comment_string_and_raw_string_decoys() {
    let redundant_plural = r##"Type(
    name: "Player",
    spelling: r#"plural and name are decoys"#,
    grammar: Noun(
        singular: "player",
        plural:
            "players",
    ),
)"##;
    let error = read_str(source_path("Player.ron"), redundant_plural).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition {
            line: 7,
            column: 13,
        })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::RedundantOverride {
            field: "plural",
            ..
        })
    ));

    let spelling_mismatch = r#"// spelling: "scry <Param(0)>" is only a comment
KeywordAction(
    name: "Mill",
    params: [Amount],
    grammar: Verb(bare: "mill", frame_set: MeasureComplement),
    spelling:
        "scry <Param(0)>",
)"#;
    let error = read_str(source_path("Mill.ron"), spelling_mismatch).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 7, column: 9 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::GrammarSpellingMismatch { .. })
    ));

    let first_path = source_path("a.ron");
    let second_path = source_path("z.ron");
    let duplicate = r#"KeywordAction(
    spelling: "name is a decoy",
    name:
        "Destroy",
)"#;
    let error = read_sources(vec![
        DeclarationSource::new(
            first_path,
            r#"KeywordAction(name:"Destroy",spelling:"destroy")"#,
        ),
        DeclarationSource::new(second_path.clone(), duplicate),
    ])
    .unwrap_err();
    assert_eq!(error.path(), second_path);
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 4, column: 9 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DuplicateIdentity { .. })
    ));
}

#[test]
fn grammar_and_spelling_are_bound_to_one_declaration() {
    let error = validation(
        r#"
KeywordAction(
    name: "Mill",
    params: [Amount],
    spelling: "scry <Param(0)>",
    grammar: Verb(bare: "mill", frame_set: MeasureComplement),
)
"#,
    );
    assert!(matches!(
        error,
        ValidationError::GrammarSpellingMismatch {
            spelling_head,
            grammar_head,
        } if spelling_head == "scry" && grammar_head == "mill"
    ));

    let fixed = read_str(
        source_path("TheRingTemptsYou.ron"),
        r#"
KeywordAction(
    name: "TheRingTemptsYou",
    spelling: "the Ring tempts you",
    grammar: FixedClause(surface: "the Ring tempts you"),
)
"#,
    )
    .unwrap();
    assert_eq!(fixed.grammar.unwrap().recipe, GrammarRecipe::FixedClause);
}

#[test]
fn fixed_keyword_can_derive_a_separate_participial_adjective() {
    let declaration = read_str(
        source_path("Equip.ron"),
        r#"
KeywordAbility(
    name: "Turn",
    spelling: "turn",
    grammar: FixedKeyword(
        surface: "turn",
        participial_adjective: (),
    ),
)
"#,
    )
    .expect("a keyword line may separately declare its participial adjective");
    let grammar = declaration.grammar().expect("grammar normalizes");
    assert_eq!(
        grammar.recipe(),
        &GrammarRecipe::FixedKeyword { parameter: None }
    );
    assert_eq!(grammar.surfaces().len(), 1);
    assert_eq!(grammar.surfaces()[0].text(), "turn");
    let adjective = grammar
        .participial_adjective()
        .expect("the supplemental adjective is retained separately from FixedKeyword");
    assert_eq!(adjective.feature(), SurfaceFeature::PAST_PARTICIPLE);
    assert_eq!(adjective.text(), "turned");
    assert_eq!(adjective.onset(), Onset::Consonant);
}

#[test]
fn fixed_keyword_participial_adjective_can_override_its_derived_surface() {
    let declaration = read_str(
        source_path("Custom.ron"),
        r#"
KeywordAbility(
    name: "Custom",
    spelling: "custom",
    grammar: FixedKeyword(
        surface: "custom",
        participial_adjective: (surface: "custom-made",),
    ),
)
"#,
    )
    .expect("an authored participial adjective surface normalizes");
    let adjective = declaration
        .grammar()
        .and_then(GrammarRow::participial_adjective)
        .expect("the authored participial adjective is retained");
    assert_eq!(adjective.feature(), SurfaceFeature::PAST_PARTICIPLE);
    assert_eq!(adjective.text(), "custom-made");
}

#[test]
fn source_errors_always_carry_their_path_and_position() {
    let error = parse_error(
        r#"
NotADeclaration(
    name: "Scry",
    spelling: "scry",
)
"#,
    );
    assert!(error.to_string().contains("/synthetic/Broken.ron"));

    let error = validation(
        r#"
KeywordAction(
    name: "not a name",
    spelling: "scry",
)
"#,
    );
    assert!(matches!(error, ValidationError::InvalidName { .. }));
}

#[test]
fn source_sets_are_sorted_and_reject_duplicate_identities() {
    let first_path = source_path("a.ron");
    let second_path = source_path("z.ron");
    let error = read_sources(vec![
        DeclarationSource::new(second_path.clone(), DESTROY),
        DeclarationSource::new(first_path.clone(), DESTROY),
    ])
    .unwrap_err();
    assert_eq!(error.path(), second_path);
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DuplicateIdentity {
            identity,
            first_path: found,
        }) if identity.name == "Destroy" && found == &first_path
    ));
}

#[test]
fn normalization_keeps_category_safe_identities_and_surface_ambiguity() {
    let rows = read_sources(vec![
        DeclarationSource::new(
            source_path("action/Flying.ron"),
            r#"KeywordAction(name:"Flying",spelling:"fly",grammar:Verb(bare:"fly",frame_set:Intransitive))"#,
        ),
        DeclarationSource::new(
            source_path("ability/Flying.ron"),
            r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:FixedKeyword(surface:"flying"))"#,
        ),
    ])
    .unwrap();
    assert_ne!(rows[0].identity.kind, rows[1].identity.kind);

    let rows = read_sources(vec![
        DeclarationSource::new(
            source_path("a.ron"),
            r#"Type(name:"ChargeType",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
        ),
        DeclarationSource::new(
            source_path("b.ron"),
            r#"CounterKind(name:"Charge",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
        ),
    ])
    .unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(texts(&rows[0]), ["charge"]);
    assert_eq!(texts(&rows[1]), ["charge"]);
}

fn write_builtin(root: &Path, relative: &str, source: &str) {
    let path = root.join("macros").join("stubs").join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, source).unwrap();
}

#[derive(Debug, PartialEq, Eq)]
struct NormalizedProjection {
    identity: DeclarationIdentity,
    params: Option<Vec<ParameterType>>,
    spelling: Vec<SpellingPart>,
    grammar: Option<(GrammarRecipe, Vec<RealizedSurface>)>,
    body: Option<Box<ron::value::RawValue>>,
    relative_provenance: PathBuf,
}

fn normalized_projection(
    root: &Path,
    declarations: Vec<NormalizedDeclaration>,
) -> Vec<NormalizedProjection> {
    declarations
        .into_iter()
        .map(|declaration| NormalizedProjection {
            identity: declaration.identity,
            params: declaration.params,
            spelling: declaration.spelling,
            grammar: declaration
                .grammar
                .map(|grammar| (grammar.recipe, grammar.surfaces)),
            body: declaration.body,
            relative_provenance: declaration
                .provenance
                .path
                .strip_prefix(root)
                .unwrap()
                .to_owned(),
        })
        .collect()
}

#[test]
fn builtin_reader_authenticates_every_final_path_family() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    for (relative, source, kind, name) in [
        (
            "keyword_actions/Scry.ron",
            r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",frame_set:MeasureComplement))"#,
            DeclarationKind::KeywordAction,
            "Scry",
        ),
        (
            "keyword_abilities/Flying.ron",
            r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:FixedKeyword(surface:"flying"))"#,
            DeclarationKind::KeywordAbility,
            "Flying",
        ),
        (
            "types/Creature.ron",
            r#"Type(name:"Creature",spelling:"creature",grammar:Noun(singular:"creature"))"#,
            DeclarationKind::Type,
            "Creature",
        ),
        (
            "counter_kinds/Stun.ron",
            r#"CounterKind(name:"Stun",spelling:"stun",grammar:FixedTerm(surface:"stun"))"#,
            DeclarationKind::CounterKind,
            "Stun",
        ),
        (
            "designations/Monarch.ron",
            r#"Designation(name:"Monarch",spelling:"the monarch",grammar:FixedTerm(surface:"the monarch"))"#,
            DeclarationKind::Designation,
            "Monarch",
        ),
        (
            "subtypes/artifact/Clue.ron",
            r#"Subtype(category:Artifact,name:"Clue",spelling:"Clue",grammar:Noun(singular:"Clue"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Artifact),
            "Clue",
        ),
        (
            "subtypes/battle/Siege.ron",
            r#"Subtype(category:Battle,name:"Siege",spelling:"Siege",grammar:Noun(singular:"Siege"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Battle),
            "Siege",
        ),
        (
            "subtypes/creature/Merfolk.ron",
            r#"Subtype(category:Creature,name:"Merfolk",spelling:"Merfolk",grammar:Noun(singular:"Merfolk"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Creature),
            "Merfolk",
        ),
        (
            "subtypes/enchantment/Aura.ron",
            r#"Subtype(category:Enchantment,name:"Aura",spelling:"Aura",grammar:Noun(singular:"Aura"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Enchantment),
            "Aura",
        ),
        (
            "subtypes/land/Forest.ron",
            r#"Subtype(category:Land,name:"Forest",spelling:"Forest",grammar:Noun(singular:"Forest"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Land),
            "Forest",
        ),
        (
            "subtypes/planeswalker/Jace.ron",
            r#"Subtype(category:Planeswalker,name:"Jace",spelling:"Jace",grammar:Noun(singular:"Jace",plural:Unavailable))"#,
            DeclarationKind::Subtype(SubtypeCategory::Planeswalker),
            "Jace",
        ),
        (
            "subtypes/spell/Arcane.ron",
            r#"Subtype(category:Spell,name:"Arcane",spelling:"Arcane",grammar:Noun(singular:"Arcane"))"#,
            DeclarationKind::Subtype(SubtypeCategory::Spell),
            "Arcane",
        ),
    ] {
        write_builtin(&root, relative, source);
        let expected_path = root.join("macros/stubs").join(relative);
        let declaration = read_builtin_v2(&root)
            .unwrap()
            .into_iter()
            .find(|row| row.provenance.path == expected_path)
            .unwrap();
        assert_eq!(declaration.identity.kind, kind);
        assert_eq!(declaration.identity.name, name);
        assert_eq!(declaration.provenance.path, expected_path);
    }

    let missing_temporary = tempfile::tempdir().unwrap();
    let missing = missing_temporary.path().join("builtin_v2");
    let error = read_builtin_v2(&missing).unwrap_err();
    assert_eq!(error.path(), missing);
    let position = error.position().unwrap();
    assert!(position.line >= 1 && position.column >= 1);
}

#[test]
fn builtin_reader_uses_final_paths_and_is_iteration_independent() {
    let left = tempfile::tempdir().unwrap();
    let left_root = left.path().join("builtin_v2");
    fs::create_dir(&left_root).unwrap();
    write_builtin(
        &left_root,
        "keyword_actions/Scry.ron",
        r#"
KeywordAction(
    name: "Scry",
    spelling: "scry",
    grammar: Verb(bare: "scry", third_person: "scries", frame_set: MeasureComplement),
)
"#,
    );
    write_builtin(&left_root, "keyword_actions/Destroy.ron", DESTROY);
    write_builtin(
        &left_root,
        "types/ChargeType.ron",
        r#"Type(name:"ChargeType",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
    );
    write_builtin(
        &left_root,
        "counter_kinds/Charge.ron",
        r#"CounterKind(name:"Charge",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
    );

    let right = tempfile::tempdir().unwrap();
    let right_root = right.path().join("builtin_v2");
    fs::create_dir(&right_root).unwrap();
    write_builtin(&right_root, "keyword_actions/Destroy.ron", DESTROY);
    write_builtin(
        &right_root,
        "keyword_actions/Scry.ron",
        r#"
KeywordAction(
    name: "Scry",
    spelling: "scry",
    grammar: Verb(bare: "scry", third_person: "scries", frame_set: MeasureComplement),
)
"#,
    );
    write_builtin(
        &right_root,
        "counter_kinds/Charge.ron",
        r#"CounterKind(name:"Charge",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
    );
    write_builtin(
        &right_root,
        "types/ChargeType.ron",
        r#"Type(name:"ChargeType",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
    );

    let left = normalized_projection(&left_root, read_builtin_v2(&left_root).unwrap());
    let right = normalized_projection(&right_root, read_builtin_v2(&right_root).unwrap());
    assert_eq!(left, right);
    assert_eq!(
        left.iter()
            .map(|declaration| declaration.identity.name.as_str())
            .collect::<Vec<_>>(),
        ["Charge", "Destroy", "Scry", "ChargeType"]
    );
    assert_eq!(
        left.iter()
            .filter(|declaration| {
                declaration
                    .grammar
                    .as_ref()
                    .unwrap()
                    .1
                    .iter()
                    .any(|surface| surface.text == "charge")
            })
            .count(),
        2
    );
}

#[test]
fn builtin_reader_authenticates_kind_category_name_and_root() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    write_builtin(
        &root,
        "keyword_actions/Scry.ron",
        r#"
KeywordAbility(
    name: "Scry",
    spelling: "scry",
    grammar: FixedKeyword(surface: "scry"),
)
"#,
    );
    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 2, column: 1 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationKindMismatch {
            expected: DeclarationKind::KeywordAction,
            actual: DeclarationKind::KeywordAbility,
        })
    ));

    fs::remove_dir_all(root.join("macros")).unwrap();
    write_builtin(
        &root,
        "subtypes/creature/Merfolk.ron",
        r#"
Subtype(
    category: Land,
    name: "Merfolk",
    spelling: "Merfolk",
    grammar: Noun(singular: "Merfolk", plural: "Merfolk"),
)
"#,
    );
    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition {
            line: 3,
            column: 15
        })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationKindMismatch {
            expected: DeclarationKind::Subtype(SubtypeCategory::Creature),
            actual: DeclarationKind::Subtype(SubtypeCategory::Land),
        })
    ));

    fs::remove_dir_all(root.join("macros")).unwrap();
    write_builtin(
        &root,
        "keyword_actions/Scry.ron",
        r#"
KeywordAction(
    name: "Surveil",
    spelling: "surveil",
    grammar: Verb(bare: "surveil", frame_set: MeasureComplement),
)
"#,
    );
    let error = read_builtin_v2(&root).unwrap_err();
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationNameMismatch { expected, actual })
            if expected == "Scry" && actual == "Surveil"
    ));

    let wrong_root = temporary.path().join("something_else");
    fs::create_dir(&wrong_root).unwrap();
    let error = read_builtin_v2(&wrong_root).unwrap_err();
    assert!(matches!(
        error.validation(),
        Some(ValidationError::InvalidBuiltinRoot)
    ));
}

#[test]
fn builtin_kind_and_name_mismatches_ignore_decoys() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    write_builtin(
        &root,
        "keyword_actions/Scry.ron",
        r#"// KeywordAbility is a decoy
KeywordAbility(
    name: "Scry",
    spelling: "scry",
)"#,
    );
    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 2, column: 1 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationKindMismatch { .. })
    ));

    fs::remove_dir_all(root.join("macros")).unwrap();
    write_builtin(
        &root,
        "keyword_actions/Scry.ron",
        r##"KeywordAction(
    spelling: r#"name is only a decoy"#,
    name:
        "Surveil",
)"##,
    );
    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 4, column: 9 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationNameMismatch { .. })
    ));
}

#[test]
fn builtin_subtype_category_mismatch_locates_the_category_field_value() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    write_builtin(
        &root,
        "subtypes/creature/Land.ron",
        r#"
Subtype(
    name: "Land",
    spelling: "land",
    category
        :
        Land,
    grammar: Noun(singular: "land"),
)
"#,
    );

    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 7, column: 9 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationKindMismatch {
            expected: DeclarationKind::Subtype(SubtypeCategory::Creature),
            actual: DeclarationKind::Subtype(SubtypeCategory::Land),
        })
    ));
}

#[test]
fn builtin_subtype_category_mismatch_skips_field_text_inside_raw_strings() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    write_builtin(
        &root,
        "subtypes/creature/Land.ron",
        r##"
Subtype(
    name: "Land",
    spelling: r#"decoy " category: Land still raw"#,
    category
        :
        Land,
)
"##,
    );

    let error = read_builtin_v2(&root).unwrap_err();
    assert_eq!(
        error.position(),
        Some(SourcePosition { line: 7, column: 9 })
    );
    assert!(matches!(
        error.validation(),
        Some(ValidationError::DeclarationKindMismatch {
            expected: DeclarationKind::Subtype(SubtypeCategory::Creature),
            actual: DeclarationKind::Subtype(SubtypeCategory::Land),
        })
    ));
}

#[test]
fn builtin_reader_rejects_malformed_and_nonfinal_locations() {
    for (relative, source) in [
        ("keyword_actions/extra/Scry.ron", DESTROY),
        (
            "subtypes/unknown/Thing.ron",
            r#"Subtype(category:Creature,name:"Thing",spelling:"Thing")"#,
        ),
        ("mystery/Scry.ron", DESTROY),
        ("keyword_actions/Scry.ron", "KeywordAction(name: \"Scry\""),
    ] {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path().join("builtin_v2");
        fs::create_dir(&root).unwrap();
        write_builtin(&root, relative, source);
        let path = root.join("macros/stubs").join(relative);
        let error = read_builtin_v2(&root).unwrap_err();
        assert_located(&error, &path);
    }
}

#[test]
fn unknown_builtin_nursery_locations_fail_closed() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("builtin_v2");
    fs::create_dir(&root).unwrap();
    write_builtin(&root, "mystery/Scry.ron", DESTROY);
    let error = read_builtin_v2(&root).unwrap_err();
    assert!(matches!(
        error.validation(),
        Some(ValidationError::UnexpectedBuiltinLocation { .. })
    ));
}
