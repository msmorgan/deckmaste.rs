use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::BoundedParseOutcome;
use deckmaste_english_v2::parser::Expectation;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseAnalysis;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::SelectionLoserReason;
use deckmaste_english_v2::parser::TerminalClass;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::Onset;

fn card_name_provider() -> CatalogProviderRows {
    CatalogProviderRows::new(
        CatalogProvider::CardNames,
        [
            CatalogProviderRow::new("alpha", "Alpha", Onset::Vowel),
            CatalogProviderRow::new("alpha-beta", "Alpha Beta", Onset::Vowel),
            CatalogProviderRow::new("urzas-saga", "Urza's Saga", Onset::Vowel),
            CatalogProviderRow::new("seven-dwarves", "Seven Dwarves", Onset::Consonant),
        ],
    )
}

fn environment() -> ParserEnvironment {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_parts(declarations, [card_name_provider()])
        .expect("builtin-v2 declaration and catalog environment freezes")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("required declarations are present")
}

fn context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name, false, Onset::Consonant)
        .expect("test card name is a valid parse context")
}

fn damage_recipient(object: Object) -> DamageRecipient {
    DamageRecipient::DamageRecipient(DamageRecipientValue { object })
}

fn legendary_context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name, true, Onset::Consonant)
        .expect("test legendary card name is a valid parse context")
}

fn single_paragraph(oracle_text: &OracleText) -> &Sentences {
    let [
        DocumentBlock::Ability(Ability::Plain(Plain {
            body: AbilityBody::Sentences(sentences),
        })),
    ] = oracle_text.blocks.as_slice()
    else {
        panic!("expected exactly one paragraph ability block: {oracle_text:?}");
    };
    sentences
}

fn claims_overlap(left: TextSpan, right: TextSpan) -> bool {
    left.start < right.end && right.start < left.end
}

#[test]
fn indefinite_articles_are_guarded_by_frozen_onset_without_ast_article_state() {
    let environment = environment();
    let parser = Parser::new(environment.clone()).expect("required declarations are present");
    let context = context("Context Card");

    for text in ["Destroy a player.", "Destroy an artifact."] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text} must parse: {error:?}"));
        let Ability::Plain(Plain {
            body: AbilityBody::Sentences(paragraph),
        }) = &parsed
        else {
            panic!("indefinite destroy is a paragraph")
        };
        let [Sentence::Imperative(imperative)] = paragraph.sentences() else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let Predicate::Atomic(VerbPhrase::TransitivePredicate(TransitivePredicate {
            head: TransitiveVerb::Declaration(_),
            object: Object::ObjectNominal(NominalObject { value }),
        })) = imperative.predicate()
        else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        let UnqualifiedReference::IndefiniteReference(IndefiniteReference {
            nominal: SingularNominal::BareSingularNominal(BareSingularNominal { head }),
        }) = unqualified_reference(value)
        else {
            panic!("the public staged indefinite AST stores its noun head: {parsed:?}")
        };
        assert!(matches!(
            head,
            SingularHead::CommonSingularHead(_)
                | SingularHead::TypeSingularHead(_)
                | SingularHead::ArtifactSubtypeSingularHead(_)
                | SingularHead::BattleSubtypeSingularHead(_)
                | SingularHead::CreatureSubtypeSingularHead(_)
                | SingularHead::EnchantmentSubtypeSingularHead(_)
                | SingularHead::LandSubtypeSingularHead(_)
                | SingularHead::PlaneswalkerSubtypeSingularHead(_)
                | SingularHead::SpellSubtypeSingularHead(_)
        ));
        assert_eq!(parsed.render(&context, &environment), text);
        let ownership = parser
            .analyze(text, &context)
            .ownership()
            .expect("selected indefinite phrase owns all bytes")
            .clone();
        assert!(ownership.summary().covered());
        assert!(
            ownership.parsed_claims().iter().any(|claim| {
                claim.kind() == LexicalProvenanceKind::FormLiteral
                    && text[claim.span().start..claim.span().end].trim()
                        == if text.contains(" an ") { "an" } else { "a" }
            }),
            "article has exact form-literal ownership: {ownership:?}"
        );
    }

    for text in ["Destroy an player.", "Destroy a artifact."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong article must not select: {text}",
        );
    }

    let mut declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    declarations.push(
        macro_ron::v2::read_str(
            "/synthetic/types/Herb.ron",
            r#"Type(
                name:"Herb",
                spelling:"herb",
                grammar:Noun(singular:"herb",singular_onset:Vowel),
            )"#,
        )
        .expect("open declaration onset override normalizes"),
    );
    let overridden_environment =
        ParserEnvironment::try_from_parts(declarations, [card_name_provider()])
            .expect("overridden environment freezes");
    let overridden_parser =
        Parser::new(overridden_environment.clone()).expect("required declarations are present");
    let text = "Destroy an herb.";
    let parsed = overridden_parser
        .parse(text, &context)
        .expect("scanner/build use frozen onset rather than recomputing from `herb`");
    assert_eq!(parsed.render(&context, &overridden_environment), text);
    assert!(
        overridden_parser
            .parse("Destroy a herb.", &context)
            .is_err()
    );
}

fn assert_one_structural_claim(
    claims: &[deckmaste_english_v2::parser::LexicalClaim],
    span: TextSpan,
    owner: &str,
) {
    let matching = claims
        .iter()
        .filter(|claim| claim.span() == span)
        .collect::<Vec<_>>();
    assert_eq!(matching.len(), 1, "exactly one claim must own {span:?}");
    assert_eq!(matching[0].stable_owner_id(), owner);
    assert!(
        claims
            .iter()
            .filter(|claim| claim.span() != span)
            .all(|claim| !claims_overlap(claim.span(), span)),
        "no neighboring lexical claim may overlap {span:?}",
    );
}

#[test]
fn empty_oracle_text_is_a_totally_owned_zero_block_document() {
    let parser = parser();
    let environment = environment();
    let context = context("Grizzly Bears");
    let expected = OracleText { blocks: vec![] };

    let analysis = parser.analyze_oracle_text("", &context);
    assert_eq!(analysis.selected(), Some(&expected));
    assert_eq!(expected.render(&context, &environment), "");

    let ownership = analysis
        .ownership()
        .expect("empty OracleText is a selected parse");
    assert_eq!(ownership.rendered_text(), "");
    assert!(ownership.parsed_claims().is_empty());
    assert!(ownership.rendered_claims().is_empty());
    assert!(ownership.failures().is_empty());

    let summary = ownership.summary();
    assert!(summary.covered());
    assert_eq!(summary.claims(), 0);
    assert_eq!(summary.claimed_bytes(), 0);
    assert_eq!(summary.form_literal_claims(), 0);
    assert_eq!(summary.form_literal_bytes(), 0);
    assert_eq!(summary.vocab_claims(), 0);
    assert_eq!(summary.vocab_bytes(), 0);
    assert_eq!(summary.lexeme_claims(), 0);
    assert_eq!(summary.lexeme_bytes(), 0);
    assert_eq!(summary.codec_claims(), 0);
    assert_eq!(summary.codec_bytes(), 0);
    assert_eq!(summary.identity_claims(), 0);
    assert_eq!(summary.identity_bytes(), 0);
    assert_eq!(summary.gap_spans(), 0);
    assert_eq!(summary.gap_bytes(), 0);
    assert_eq!(summary.overlap_spans(), 0);
    assert_eq!(summary.overlap_bytes(), 0);
    assert_eq!(summary.synthetic_claims(), 0);
    assert_eq!(summary.provenance_plan_mismatches(), 0);
}

#[test]
fn oracle_text_parses_all_eight_two_sentence_faces_as_one_ordered_paragraph() {
    let parser = parser();
    let environment = environment();
    for (card_name, text, first_sentence, second_sentence) in [
        (
            "Cursebreak",
            "Destroy target enchantment. You gain 2 life.",
            "Destroy target enchantment.",
            "You gain 2 life.",
        ),
        (
            "Drain the Well",
            "Destroy target land. You gain 2 life.",
            "Destroy target land.",
            "You gain 2 life.",
        ),
        (
            "Into the Maw of Hell",
            "Destroy target land. Into the Maw of Hell deals 13 damage to target creature.",
            "Destroy target land.",
            "Into the Maw of Hell deals 13 damage to target creature.",
        ),
        (
            "Lich's Caress",
            "Destroy target creature. You gain 3 life.",
            "Destroy target creature.",
            "You gain 3 life.",
        ),
        (
            "Maw of the Mire",
            "Destroy target land. You gain 4 life.",
            "Destroy target land.",
            "You gain 4 life.",
        ),
        (
            "Ray of Dissolution",
            "Destroy target enchantment. You gain 3 life.",
            "Destroy target enchantment.",
            "You gain 3 life.",
        ),
        (
            "Sephiroth's Intervention",
            "Destroy target creature. You gain 2 life.",
            "Destroy target creature.",
            "You gain 2 life.",
        ),
        (
            "Winter's Intervention",
            "Winter's Intervention deals 2 damage to target creature. You gain 2 life.",
            "Winter's Intervention deals 2 damage to target creature.",
            "You gain 2 life.",
        ),
    ] {
        let context = if card_name == "Zacama, Primal Calamity" {
            legendary_context(card_name)
        } else {
            context(card_name)
        };
        let parsed = parser
            .parse_oracle_text(text, &context)
            .unwrap_or_else(|error| panic!("{card_name} must parse: {error:?}"));
        let paragraph = single_paragraph(&parsed);
        assert_eq!(paragraph.sentences().len(), 2, "{card_name}");
        assert_eq!(
            paragraph.sentences()[0].render(&context, &environment),
            first_sentence,
            "{card_name} first sentence",
        );
        assert_eq!(
            paragraph.sentences()[1].render(&context, &environment),
            second_sentence,
            "{card_name} second sentence",
        );
        assert_eq!(parsed.render(&context, &environment), text, "{card_name}");

        let analysis: ParseAnalysis<OracleText> = parser.analyze_oracle_text(text, &context);
        assert_eq!(analysis.selected(), Some(&parsed), "{card_name}");
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("{card_name} has selected ownership"));
        assert!(ownership.failures().is_empty(), "{card_name}");
        assert!(ownership.summary().covered(), "{card_name}");
    }
}

#[test]
fn oracle_text_structural_boundaries_have_exact_generated_claims() {
    let parser = parser();
    let context = context("Sephiroth's Intervention");
    let text = "Destroy target creature. You gain 2 life.";
    let analysis = parser.analyze_oracle_text(text, &context);
    let ownership = analysis.ownership().expect("OracleText is selected");
    assert!(ownership.summary().covered());

    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 23, end: 24 },
        "structural:Sentences/sentences/terminator/0",
    );
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 24, end: 25 },
        "structural:Sentences/sentences/separator/uniform/0",
    );
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 40, end: 41 },
        "structural:Sentences/sentences/terminator/0",
    );
}

#[test]
fn oracle_text_lf_separates_blocks_without_flattening_or_storage() {
    let parser = parser();
    let environment = environment();
    let context = context("Sephiroth's Intervention");
    let one_block_text = "Destroy target creature. You gain 2 life.";
    let two_block_text = "Destroy target creature.\nYou gain 2 life.";
    let one_block = parser
        .parse_oracle_text(one_block_text, &context)
        .expect("space-separated text parses as one block");
    let two_blocks = parser
        .parse_oracle_text(two_block_text, &context)
        .expect("LF-separated text parses as two blocks");

    assert_ne!(one_block, two_blocks);
    let [
        DocumentBlock::Ability(Ability::Plain(Plain {
            body: AbilityBody::Sentences(first),
        })),
        DocumentBlock::Ability(Ability::Plain(Plain {
            body: AbilityBody::Sentences(second),
        })),
    ] = two_blocks.blocks.as_slice()
    else {
        panic!("LF must preserve two paragraph blocks: {two_blocks:?}");
    };
    assert_eq!(first.sentences().len(), 1);
    assert_eq!(second.sentences().len(), 1);
    assert_eq!(two_blocks.render(&context, &environment), two_block_text);

    let analysis = parser.analyze_oracle_text(two_block_text, &context);
    let ownership = analysis.ownership().expect("two blocks are selected");
    assert!(ownership.summary().covered());
    assert_one_structural_claim(
        ownership.parsed_claims(),
        TextSpan { start: 24, end: 25 },
        "structural:OracleText/blocks/separator/uniform/0",
    );
}

#[test]
fn oracle_text_trace_retains_its_exact_value_type_and_root_name() {
    let parser = parser();
    let context = context("Sephiroth's Intervention");
    let trace: ParserTrace<OracleText> = parser.trace_oracle_text(
        "Destroy target creature. You gain 2 life.",
        &context,
        TraceLimits::new(usize::MAX),
    );

    assert_eq!(trace.root_name(), "OracleText");
    assert!(trace.into_parse_result().is_ok());
}

fn self_reference(spelling: SelfReferenceSpelling, card_name: &str) -> SourceSelfReference {
    let context = if spelling == SelfReferenceSpelling::Abbreviated {
        legendary_context(card_name)
    } else {
        context(card_name)
    };
    SourceSelfReference::new(spelling, &context).expect("test spelling is valid for its context")
}

enum Noun {
    Lexeme(CommonNoun),
    Type(TypeNoun),
}

fn singular_nominal(noun: Noun) -> SingularNominal {
    SingularNominal::BareSingularNominal(BareSingularNominal {
        head: match noun {
            Noun::Lexeme(noun) => SingularHead::CommonSingularHead(CommonSingularHead { noun }),
            Noun::Type(noun) => SingularHead::TypeSingularHead(TypeSingularHead { noun }),
        },
    })
}

fn plural_nominal(noun: Noun) -> PluralNominal {
    PluralNominal::BarePluralNominal(BarePluralNominal {
        head: plural_head(noun),
    })
}

fn plural_head(noun: Noun) -> PluralHead {
    match noun {
        Noun::Lexeme(noun) => PluralHead::CommonPluralHead(CommonPluralHead { noun }),
        Noun::Type(noun) => PluralHead::TypePluralHead(TypePluralHead { noun }),
    }
}

fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
            reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                reference: ControllerStage::UnqualifiedControllerStage(
                    UnqualifiedControllerStage { reference },
                ),
            }),
        }),
    })
}

fn unqualified_reference(noun_phrase: &NounPhrase) -> &UnqualifiedReference {
    let NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference:
            NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference:
                    ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                        reference:
                            ControllerStage::UnqualifiedControllerStage(UnqualifiedControllerStage {
                                reference,
                            }),
                    }),
            }),
    }) = noun_phrase
    else {
        panic!("expected an unqualified noun phrase: {noun_phrase:?}")
    };
    reference
}

fn indefinite(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::IndefiniteReference(
        IndefiniteReference {
            nominal: singular_nominal(noun),
        },
    ))
}

fn target_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::OrdinarySingularReference(
        OrdinarySingularReference::new(DeterminerPhrase::TargetDeterminerPhrase(
            TargetDeterminerPhrase {
                nominal: singular_nominal(noun),
            },
        ))
        .expect("target determiner is an ordinary singular reference"),
    ))
}

fn creatures_you_control_with_power_at_most_two() -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: NumericStage::ScalarQualifiedReference(ScalarQualifiedReference {
            reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                reference: ControllerStage::ControllerQualifiedReference(
                    ControllerQualifiedReference {
                        reference: UnqualifiedReference::OrdinaryPluralReference(
                            OrdinaryPluralReference::new(PluralSelector::UnmarkedPluralSelector(
                                UnmarkedPluralSelector {
                                    nominal: plural_nominal(creatures()),
                                },
                            ))
                            .expect("unmarked plural is valid for an ordinary reference"),
                        ),
                        controller_owner: ControllerOwnerQualification::YouControl(
                            YouControl::new(SubjectPronoun::You)
                                .expect("You is a valid controller"),
                        ),
                    },
                ),
            }),
            scalar: ScalarQualification::ScalarQualification(ScalarQualificationValue {
                measure: ScalarMeasure::CharacteristicScalar(CharacteristicScalar {
                    characteristic: ScalarCharacteristic::Power,
                }),
                comparison: ScalarComparison::ScalarOrLess(ScalarOrLess {
                    threshold: ScalarThreshold::FixedScalarThreshold(FixedScalarThreshold {
                        value: ScalarNumber { magnitude: 2 },
                    }),
                }),
            }),
        }),
    })
}

fn that_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::ThatReference(ThatReference {
        nominal: singular_nominal(noun),
    }))
}

fn those_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::ThoseReference(ThoseReference {
        nominal: plural_nominal(noun),
    }))
}

fn nominal_subject(value: NounPhrase) -> Subject {
    Subject::SubjectNominal(NominalSubject { value })
}

fn nominal_object(value: NounPhrase) -> Object {
    Object::ObjectNominal(NominalObject { value })
}

fn subject_you() -> Subject {
    Subject::SubjectPronoun(PersonalSubject {
        word: SubjectPronoun::You,
    })
}

fn object_you() -> Object {
    Object::ObjectPronoun(PersonalObject {
        word: ObjectPronoun::You,
    })
}

fn object_it() -> Object {
    Object::ObjectPronoun(PersonalObject {
        word: ObjectPronoun::It,
    })
}

#[test]
fn ast_re_exports_keep_the_retired_spell_audit_line_local() {
    let public_prefix = concat!("pub ", "use ");
    let retired_name = concat!("Sp", "ell");
    let retired_export = format!("{public_prefix}crate::constructions::{retired_name};");
    for line in include_str!("../src/ast.rs").lines() {
        assert_ne!(line, retired_export, "retired audit match: {line}");
    }
}

#[test]
fn ast_public_surface_excludes_retired_ability_exports_and_aliases() {
    fn collect_use_names(tree: &syn::UseTree, names: &mut Vec<String>) {
        match tree {
            syn::UseTree::Name(name) => names.push(name.ident.to_string()),
            syn::UseTree::Rename(rename) => names.push(rename.rename.to_string()),
            syn::UseTree::Path(path) => collect_use_names(&path.tree, names),
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    collect_use_names(item, names);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    let file = syn::parse_file(include_str!("../src/ast.rs")).expect("AST facade parses as Rust");
    let mut public_names = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Use(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                collect_use_names(&item.tree, &mut public_names);
            }
            syn::Item::Type(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Enum(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            syn::Item::Union(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                public_names.push(item.ident.to_string());
            }
            _ => {}
        }
    }

    for retired in ["Event", "EventClause", "Paragraph"] {
        assert!(
            !public_names.iter().any(|name| name == retired),
            "retired public AST name remains exported or aliased: {retired}"
        );
    }
}

#[test]
fn self_reference_spelling_variants_remain_publicly_importable() {
    use deckmaste_english_v2::ast::SelfReferenceSpelling::Abbreviated;
    use deckmaste_english_v2::ast::SelfReferenceSpelling::Full;

    let _: SelfReferenceSpelling = Full;
    let _: SelfReferenceSpelling = Abbreviated;
}

#[test]
fn parse_context_rejects_only_empty_self_names() {
    assert!(ParseContext::new("", false, Onset::Consonant).is_none());
    assert!(ParseContext::new(", the Empty Prefix", false, Onset::Consonant).is_some());
}

#[test]
fn parse_context_uses_opaque_full_names_and_legendary_only_short_forms() {
    let opaque = ParseContext::new("+2 Mace", false, Onset::Consonant)
        .expect("every nonempty full name is opaque");
    assert_eq!(opaque.card_name(), "+2 Mace");
    assert_eq!(opaque.abbreviated_card_name(), "+2 Mace");

    for name in ["Fear, Fire, Foes!", "Grizzly Bears"] {
        let context = ParseContext::new(name, false, Onset::Consonant)
            .expect("nonlegendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), name);
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_none(),
            "nonlegendary {name:?} must not license an abbreviation",
        );
    }

    for (name, abbreviated) in [
        ("Aang, A Lot to Learn", "Aang"),
        ("The Balrog, Durin's Bane", "The Balrog"),
        ("King Darien XLVIII", "King Darien"),
        ("Sidar Jabari of Zhalfir", "Sidar Jabari"),
        ("Tor Wauki the Younger", "Tor Wauki"),
        ("Sliver Queen", "Sliver"),
    ] {
        let onset = if name.starts_with("Aang") { Onset::Vowel } else { Onset::Consonant };
        let context = ParseContext::new(name, true, onset).expect("legendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), abbreviated, "{name}");
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_some(),
            "legendary {name:?} licenses {abbreviated:?}",
        );
    }

    for name in ["The First Sliver", "Progenitus"] {
        let context =
            ParseContext::new(name, true, Onset::Consonant).expect("legendary full name is valid");
        assert_eq!(context.abbreviated_card_name(), name);
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_none(),
            "legendary {name:?} has no distinct licensed abbreviation",
        );
    }
}

#[test]
fn self_reference_spelling_is_checked_against_its_context() {
    let no_comma = ParseContext::new("Context Card", false, Onset::Consonant)
        .expect("nonempty context is valid");
    assert!(SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &no_comma).is_none());
    assert_eq!(
        SourceSelfReference::new(SelfReferenceSpelling::Full, &no_comma)
            .expect("full spelling is always distinct from no value")
            .spelling(),
        SelfReferenceSpelling::Full
    );

    let comma = ParseContext::new("Zacama, Primal Calamity", true, Onset::Consonant)
        .expect("legendary abbreviation is valid");
    assert_eq!(
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &comma)
            .expect("comma-bearing context has a distinct abbreviation")
            .spelling(),
        SelfReferenceSpelling::Abbreviated
    );
}

#[test]
fn context_identity_stores_only_the_reusable_arm() {
    assert_eq!(
        std::mem::size_of::<SelfReferenceSpelling>(),
        1,
        "the generated value must not retain either context spelling"
    );
    let spelling = SelfReferenceSpelling::Abbreviated;
    assert_eq!(
        SourceSelfReference::new(spelling, &legendary_context("Zacama, Primal Calamity"))
            .expect("the stored arm is valid for one comma abbreviation")
            .spelling(),
        spelling
    );
    assert_eq!(
        SourceSelfReference::new(spelling, &legendary_context("Zoraline, Cosmos Caller"))
            .expect("the same stored arm is valid for another comma abbreviation")
            .spelling(),
        spelling
    );
}

#[test]
fn generated_invariant_products_enforce_values_and_round_trip_publicly() {
    let environment = environment();
    let parser = parser();
    let plain_context = context("Context Card");

    let event = connive_event();
    let effect = gain_life_sentence();
    let triggered = triggered(event.clone(), vec![effect.clone()]);
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if: None,
        body: AbilityBody::Sentences(body),
    }) = &triggered
    else {
        panic!("the linguistic triggered envelope is preserved")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    assert_eq!(&finite.clause, &Clause::Finite(event));
    assert_eq!(body.sentences(), std::slice::from_ref(&effect));
    let triggered_text = triggered.render(&plain_context, &environment);
    assert_eq!(parser.parse(&triggered_text, &plain_context), Ok(triggered));
    assert_eq!(
        parser
            .parse(&triggered_text, &plain_context)
            .expect("rendered Triggered ability parses")
            .render(&plain_context, &environment),
        triggered_text,
    );

    let body = gain_life_sentence();
    let clause = WhereClauseCategory::Where(WhereClause {
        variable: Variable::X,
        value: object_you(),
    });
    let with_where = WithWhere {
        body: Box::new(body.clone()),
        clause: clause.clone(),
    };
    let _: &Sentence = &with_where.body;
    let _: &WhereClauseCategory = &with_where.clause;
    assert_eq!(&with_where.clause, &clause);
    let _ = body;
    let with_where = paragraph(Sentence::WithWhere(with_where));
    let with_where_text = with_where.render(&plain_context, &environment);
    assert_eq!(
        parser.parse(&with_where_text, &plain_context),
        Ok(with_where)
    );
    assert_eq!(
        parser
            .parse(&with_where_text, &plain_context)
            .expect("rendered WithWhere sentence parses")
            .render(&plain_context, &environment),
        with_where_text,
    );

    let controller = YouControl::new(SubjectPronoun::You).expect("You is a valid controller");
    let _: SubjectPronoun = controller.controller();
    assert_eq!(controller.controller(), SubjectPronoun::You);
    assert!(
        YouControl::new(SubjectPronoun::It).is_none(),
        "It is not a valid controller",
    );
    let count = paragraph(declarative(
        nominal_subject(creatures_you_control_with_power_at_most_two()),
        VerbPhrase::GainLife(GainLife {
            amount: variable_x(),
        }),
    ));
    let count_text = count.render(&plain_context, &environment);
    assert_eq!(parser.parse(&count_text, &plain_context), Ok(count));
    assert_eq!(
        parser
            .parse(&count_text, &plain_context)
            .expect("rendered qualified nominal sentence parses")
            .render(&plain_context, &environment),
        count_text,
    );

    assert!(
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &plain_context).is_none(),
        "an unavailable abbreviation is rejected",
    );
    let abbreviated_context = legendary_context("Zacama, Primal Calamity");
    let self_reference =
        SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &abbreviated_context)
            .expect("a distinct abbreviated spelling is valid");
    let _: SelfReferenceSpelling = self_reference.spelling();
    assert_eq!(
        self_reference.spelling(),
        SelfReferenceSpelling::Abbreviated
    );
    let self_reference = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference,
        ))),
        VerbPhrase::GainLife(GainLife {
            amount: variable_x(),
        }),
    ));
    let self_reference_text = self_reference.render(&abbreviated_context, &environment);
    assert_eq!(
        parser.parse(&self_reference_text, &abbreviated_context),
        Ok(self_reference)
    );
    assert_eq!(
        parser
            .parse(&self_reference_text, &abbreviated_context)
            .expect("rendered source-self-reference sentence parses")
            .render(&abbreviated_context, &environment),
        self_reference_text,
    );
}

#[test]
fn typed_where_staging_rejects_a_finite_subordinate_clause_in_the_chart() {
    let parser = parser();
    let context = context("Context Card");
    let denied = "You gain X life, a player connives.";

    assert_eq!(
        parser.parse_sentence(denied, &context),
        Err(ParseError::Failure {
            span: TextSpan { start: 34, end: 35 },
            expectations: BTreeSet::from([
                Expectation::Nonterminal(NonterminalCategory::ScalarReference),
                Expectation::Nonterminal(NonterminalCategory::Amount),
                Expectation::Terminal(TerminalClass::Variable),
                Expectation::Terminal(TerminalClass::ScalarNumber),
                Expectation::Literal(" and "),
                Expectation::Literal(" and/or "),
                Expectation::Literal(" or "),
                Expectation::Literal(", "),
                Expectation::Literal(", and "),
                Expectation::Literal(", and/or "),
                Expectation::Literal(", or "),
                Expectation::Literal("that"),
            ]),
        })
    );

    let trace = parser.trace_sentence(denied, &context, TraceLimits::new(usize::MAX));
    assert!(matches!(
        trace.outcome(),
        BoundedParseOutcome::ParseFailure(_)
    ));
    assert!(trace.accepted_roots().items().is_empty());
    assert!(trace.materialized_candidates().items().is_empty());
    assert!(trace.build_rejection().is_none());
    assert!(trace.checked_completion_rejections().items().is_empty());

    let allowed =
        "You gain X life, where X is the number of creatures you control with power 2 or less.";
    assert!(parser.parse_sentence(allowed, &context).is_ok());
    assert!(
        parser
            .trace_sentence(allowed, &context, TraceLimits::new(8))
            .build_rejection()
            .is_none()
    );

    let malformed = "You gain X life, a player.";
    let malformed_error = parser
        .parse_sentence(malformed, &context)
        .expect_err("incomplete finite-clause syntax remains a chart failure");
    assert!(matches!(malformed_error, ParseError::Failure { .. }));
    assert!(
        parser
            .trace_sentence(malformed, &context, TraceLimits::new(8))
            .build_rejection()
            .is_none()
    );
}

#[test]
fn demonstrative_references_select_distinct_typed_constructions() {
    let parser = parser();
    let environment = environment();
    let context = context("Context Card");

    for (text, reference, selected_rule) in [
        (
            "That creature deals 3 damage to it.",
            that_noun(creature()),
            "UnqualifiedReferenceThatReference",
        ),
        (
            "Those creatures deal 3 damage to it.",
            those_noun(creatures()),
            "UnqualifiedReferenceThoseReference",
        ),
    ] {
        let expected = declarative(
            nominal_subject(reference),
            VerbPhrase::DealDamage(DealDamage {
                amount: Amount::Number(NumberAmount {
                    number: ScalarNumber { magnitude: 3 },
                }),
                recipient: damage_recipient(object_it()),
            }),
        );

        let trace = parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));
        assert_eq!(trace.clone().into_parse_result(), Ok(expected.clone()));
        assert_eq!(expected.render(&context, &environment), text);
        assert_eq!(trace.materialized_candidates().total(), 1);

        let forest_rules = trace
            .forest()
            .items()
            .iter()
            .map(deckmaste_english_v2::parser::ForestNode::rule_name_v1)
            .collect::<BTreeSet<_>>();
        assert!(
            forest_rules.contains(selected_rule),
            "{text}: {forest_rules:?}"
        );
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the complete source-ordered public type oracle is deliberately literal"
)]
fn assert_complete_public_generated_type_inventory(file: &syn::File) {
    let public_types = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Enum(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                Some(item.ident.to_string())
            }
            syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                Some(item.ident.to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        public_types,
        [
            "FiniteCondition",
            "ExistentialCondition",
            "ExistentialClause",
            "AmongPhrase",
            "Ability",
            "AbilityBody",
            "ModalMode",
            "TriggerPrefix",
            "AtPhrase",
            "CostSymbol",
            "LoyaltyValue",
            "Sentence",
            "PredicateCoordination",
            "PredicativeAdjectiveComplement",
            "PredicativeColorComplement",
            "PredicativeTypeComplement",
            "PredicativeStatusComplement",
            "BlockedByStatusComplement",
            "PredicativeAbilityComplement",
            "PredicativePowerToughnessComplement",
            "BareCopularPredicate",
            "ChangeStatePredicate",
            "PassiveDamagePredicate",
            "PassiveMovementPredicate",
            "PassiveOrientationPredicate",
            "DeclaredTransitivePassivePredicate",
            "BarePassivePredicate",
            "CopularClause",
            "PassiveFiniteClause",
            "FiniteClause",
            "ClauseCoordination",
            "WhereClauseCategory",
            "Subject",
            "Object",
            "SingularHead",
            "PluralHead",
            "NominalModifier",
            "NegativeNominalModifier",
            "CoordinatedNominalModifier",
            "CompoundNominalModifier",
            "SingularNominal",
            "PluralNominal",
            "SingularCoordinationMember",
            "PluralCoordinationMember",
            "SingularNominalCoordination",
            "DeterminerScopedNominalCoordination",
            "PluralNominalCoordination",
            "SingularSelector",
            "PluralSelector",
            "UnqualifiedReference",
            "CountReference",
            "DeterminerPhrase",
            "FullNounPhraseCoordination",
            "MannerReference",
            "ScalarReference",
            "ControllerOwnerQualification",
            "SingularController",
            "ZoneReference",
            "OwnerPossessor",
            "LibraryReference",
            "LibraryCardQuantity",
            "FromSource",
            "IntoDestination",
            "OntoDestination",
            "OnDestination",
            "ToDestination",
            "PostState",
            "ControlPostmodifier",
            "ZoneLocation",
            "AtLocation",
            "ZoneQualification",
            "ScalarThreshold",
            "ScalarMeasure",
            "ScalarComparison",
            "CountComparison",
            "ScalarQualification",
            "ScalarValue",
            "ScalarEquality",
            "ControllerStage",
            "ZoneStage",
            "NumericStage",
            "NounPhrase",
            "PossessiveOwner",
            "Possessive",
            "CardQuantity",
            "CounterKind",
            "PositiveCounterMagnitude",
            "NegativeCounterMagnitude",
            "CounterQuantity",
            "DieObject",
            "VerbPhrase",
            "DamageRecipient",
            "CounterRecipient",
            "CounterSource",
            "ManaAmount",
            "Amount",
            "CardinalQuantity",
            "ActivationCostComponent",
            "Predicate",
            "Clause",
            "PredicativeComplement",
            "PredicativeStatus",
            "PassivePredicate",
            "ClauseAttachment",
            "ConditionClause",
            "DocumentBlock",
            "OracleText",
            "FiniteConditionValue",
            "ExistentialConditionValue",
            "SingularExistentialClause",
            "PluralExistentialClause",
            "AmongPhraseValue",
            "Plain",
            "Sentences",
            "ModalModeValue",
            "PlainModal",
            "Finite",
            "Temporal",
            "AtPhraseValue",
            "Triggered",
            "GenericCostSymbol",
            "FixedSymbol",
            "MonocoloredHybridSymbol",
            "SymbolRun",
            "PositiveLoyalty",
            "ZeroLoyalty",
            "NegativeLoyalty",
            "Loyalty",
            "CostClause",
            "Activated",
            "Imperative",
            "Declarative",
            "Attached",
            "PreposedIf",
            "PreposedIfPredicate",
            "PostposedIf",
            "PostposedIfPredicate",
            "PostposedUnless",
            "PostposedUnlessPredicate",
            "PreposedAsLongAs",
            "PreposedAsLongAsPredicate",
            "PreposedWhile",
            "PreposedWhilePredicate",
            "PreposedDuring",
            "PreposedDuringPredicate",
            "PreposedUntil",
            "PreposedUntilPredicate",
            "ThenSequence",
            "ThenPredicateSequence",
            "ReflexiveSubordinate",
            "ReflexivePredicateSubordinate",
            "WithWhere",
            "AndPredicateCoordination",
            "OrPredicateCoordination",
            "AndOrPredicateCoordination",
            "PredicativeAdjectiveValue",
            "PredicativeColorValue",
            "PredicativeTypeValue",
            "PredicativeStatusValue",
            "BlockedByStatusValue",
            "PredicativeAbilityValue",
            "PredicativePowerToughnessValue",
            "BareCopularPredicateValue",
            "ChangeStatePredicateValue",
            "PassiveDamagePredicateValue",
            "PassiveMovementPredicateValue",
            "PassiveOrientationPredicateValue",
            "DeclaredTransitivePassivePredicateValue",
            "BarePassivePredicateValue",
            "CopularClauseValue",
            "PassiveFiniteClauseValue",
            "PlainFiniteClause",
            "AuxiliaryFiniteClause",
            "AndClauseCoordination",
            "OrClauseCoordination",
            "AndOrClauseCoordination",
            "WhereClause",
            "NominalSubject",
            "PersonalSubject",
            "NominalObject",
            "PersonalObject",
            "ReflexiveObject",
            "ChoiceObject",
            "RandomObject",
            "CommonSingularHead",
            "TypeSingularHead",
            "ArtifactSubtypeSingularHead",
            "BattleSubtypeSingularHead",
            "CreatureSubtypeSingularHead",
            "EnchantmentSubtypeSingularHead",
            "LandSubtypeSingularHead",
            "PlaneswalkerSubtypeSingularHead",
            "SpellSubtypeSingularHead",
            "CommonPluralHead",
            "TypePluralHead",
            "ArtifactSubtypePluralHead",
            "BattleSubtypePluralHead",
            "CreatureSubtypePluralHead",
            "EnchantmentSubtypePluralHead",
            "LandSubtypePluralHead",
            "PlaneswalkerSubtypePluralHead",
            "SpellSubtypePluralHead",
            "ColorModifier",
            "StatusModifier",
            "SupertypeModifier",
            "CommonNounModifier",
            "TypeModifier",
            "ArtifactSubtypeModifier",
            "BattleSubtypeModifier",
            "CreatureSubtypeModifier",
            "EnchantmentSubtypeModifier",
            "LandSubtypeModifier",
            "PlaneswalkerSubtypeModifier",
            "SpellSubtypeModifier",
            "NonColorModifier",
            "NonCommonNounModifier",
            "NonStatusModifier",
            "NonSupertypeModifier",
            "NonTypeModifier",
            "NonArtifactSubtypeModifier",
            "NonBattleSubtypeModifier",
            "NonCreatureSubtypeModifier",
            "NonEnchantmentSubtypeModifier",
            "NonLandSubtypeModifier",
            "NonPlaneswalkerSubtypeModifier",
            "NonSpellSubtypeModifier",
            "NegativeModifierMember",
            "NonTargetCommonNounModifier",
            "CoordinatedModifierMember",
            "CompoundModifierMember",
            "BareSingularNominal",
            "ModifiedSingularNominal",
            "NegativeModifiedSingularNominal",
            "BarePluralNominal",
            "ModifiedPluralNominal",
            "CompoundModifiedSingularNominal",
            "CompoundModifiedPluralNominal",
            "NegativeModifiedPluralNominal",
            "BareSingularCoordinationMember",
            "ModifiedSingularCoordinationMember",
            "NegativeModifiedSingularCoordinationMember",
            "BarePluralCoordinationMember",
            "ModifiedPluralCoordinationMember",
            "NegativeModifiedPluralCoordinationMember",
            "SingularAndNominalCoordination",
            "SingularOrNominalCoordination",
            "SingularAndOrNominalCoordination",
            "DeterminerScopedAndNominalPair",
            "DeterminerScopedAndNominalSeries",
            "DeterminerScopedOrNominalPair",
            "DeterminerScopedOrNominalSeries",
            "DeterminerScopedAndOrNominalPair",
            "DeterminerScopedAndOrNominalSeries",
            "PluralAndNominalCoordination",
            "PluralOrNominalCoordination",
            "PluralAndOrNominalCoordination",
            "UnmarkedSingularSelector",
            "TargetSingularSelector",
            "TargetSingularCoordinationSelector",
            "OtherSingularSelector",
            "OtherTargetSingularSelector",
            "UnmarkedPluralSelector",
            "UnmarkedPluralCoordinationSelector",
            "TargetPluralSelector",
            "TargetPluralCoordinationSelector",
            "OtherPluralSelector",
            "OtherTargetPluralSelector",
            "IndefiniteReference",
            "IndefiniteCoordinationReference",
            "NamedCardReference",
            "OrdinarySingularReference",
            "OrdinaryPluralReference",
            "DefiniteSingularReference",
            "DefinitePluralReference",
            "AnyTargetReference",
            "AnotherReference",
            "AnotherCoordinationReference",
            "EachReference",
            "AllReference",
            "FixedReference",
            "VariableReference",
            "UpToOneReference",
            "UpToManyReference",
            "AnyNumberReference",
            "OneOrMoreReference",
            "ThatMany",
            "CountedReference",
            "ThisReference",
            "ThatReference",
            "DemonstrativePossessiveReference",
            "ThoseReference",
            "DesignatedSingularReference",
            "DesignatedPluralReference",
            "ChosenQualityReference",
            "PossessedSingularReference",
            "PossessedPluralReference",
            "PossessiveAbsoluteReference",
            "TargetDeterminerPhrase",
            "TargetCoordinationDeterminerPhrase",
            "IndefiniteDeterminerPhrase",
            "ThisDeterminerPhrase",
            "ThatDeterminerPhrase",
            "AnotherDeterminerPhrase",
            "FullAndNounPhraseCoordination",
            "FullOrNounPhraseCoordination",
            "FullAndOrNounPhraseCoordination",
            "CoordinatedNounPhrase",
            "SourceSelfReference",
            "ThisWay",
            "ThatMuch",
            "YouControl",
            "OpponentController",
            "OpponentControls",
            "DemonstrativeControls",
            "YouOwn",
            "PossessedZone",
            "UnpossessedZone",
            "SingularOwnerPossessor",
            "PluralOwnerPossessor",
            "OwnerPossessedZone",
            "DefiniteZone",
            "PossessedLibrary",
            "OwnerPossessedLibrary",
            "SingularLibraryCardQuantity",
            "FixedLibraryCardQuantity",
            "FromSourceValue",
            "IntoDestinationValue",
            "OntoBattlefieldDestination",
            "OnLibraryDestination",
            "ToDestinationValue",
            "TappedPostState",
            "DirectControlPostmodifier",
            "OwnerControlPostmodifier",
            "ZoneLocationValue",
            "AtLocationValue",
            "InZone",
            "FromZone",
            "FixedScalarThreshold",
            "VariableScalarThreshold",
            "CharacteristicScalar",
            "ManaValueScalar",
            "ScalarOrLess",
            "ScalarOrGreater",
            "ScalarLessThan",
            "ScalarGreaterThan",
            "ScalarLessThanOrEqualTo",
            "CountOrMore",
            "CountOrFewer",
            "ScalarQualificationValue",
            "PossessedScalarValue",
            "ScalarEqualityValue",
            "UnqualifiedControllerStage",
            "ControllerQualifiedReference",
            "OtherThanQualifiedReference",
            "UnqualifiedZoneStage",
            "ZoneQualifiedReference",
            "UnqualifiedNumericStage",
            "ScalarQualifiedReference",
            "CountComparisonReference",
            "QualifiedNounPhrase",
            "LibrarySlice",
            "PossessiveSelfReference",
            "PossessiveNoun",
            "PossessiveValue",
            "SingularCardQuantity",
            "FixedCardQuantity",
            "VariableCardQuantity",
            "AnaphoricCardQuantity",
            "ComparedCardQuantity",
            "PositivePowerToughnessCounter",
            "NegativePowerToughnessCounter",
            "PositiveCounterMagnitudeValue",
            "NegativeCounterMagnitudeValue",
            "NamedCounter",
            "SingularCounterQuantity",
            "FixedCounterQuantity",
            "VariableCounterQuantity",
            "AnaphoricCounterQuantity",
            "SingularDieObject",
            "FixedDiceObject",
            "D20Object",
            "IntransitivePredicate",
            "TransitivePredicate",
            "NumerativePredicate",
            "DamageRecipientValue",
            "CounterRecipientValue",
            "CounterSourceValue",
            "DealDamage",
            "DealUnspecifiedDamage",
            "GainLife",
            "GainUnspecifiedLife",
            "DealDamageEqualTo",
            "GainLifeEqualTo",
            "LoseLife",
            "LoseLifeEqualTo",
            "PayLife",
            "PayMana",
            "AddMana",
            "DrawCards",
            "DrawCardsEqualTo",
            "RollDice",
            "PutCounters",
            "RemoveCounters",
            "PutInto",
            "PutOnto",
            "PutOn",
            "PutTo",
            "ReturnTo",
            "EnterPostState",
            "EnterLocation",
            "EnterControl",
            "LeaveLocation",
            "LookAt",
            "SearchFor",
            "HaveCardsInHand",
            "HaveLife",
            "HaveNoMaximumHandSize",
            "HaveObjectControl",
            "ManaAmountValue",
            "NumberAmount",
            "VariableAmount",
            "ScalarReferenceAmount",
            "CardinalQuantityValue",
            "Auxiliary",
            "FiniteCopula",
            "BareCopula",
            "PredicativeAdjective",
            "DamageKind",
            "FaceOrientation",
            "AtBoundary",
            "TriggerMarker",
            "TurnOwnerPostmodifier",
            "TurnPart",
            "TurnSpecifier",
            "SubjectPronoun",
            "ObjectPronoun",
            "PossessiveDeterminerPronoun",
            "PossessiveAbsolutePronoun",
            "ReflexivePronoun",
            "Variable",
            "Color",
            "Status",
            "Designation",
            "ChosenQuality",
            "SingularDemonstrative",
            "ControllerNoun",
            "FixedCostSymbol",
            "ModalChooser",
            "ModalChoiceBounds",
            "MonocoloredHybridColor",
            "ScalarCharacteristic",
            "CounterName",
            "DieShape",
            "LibraryPosition",
            "Zone",
            "NonCommonNoun",
            "NonTargetCommonModifier",
            "Supertype",
            "CommonNoun",
            "VerbLexeme",
            "CoreIntransitiveVerb",
            "CoreTransitiveVerb",
            "CoreNumerativeVerb",
            "DamageParticipleLexeme",
            "MovementParticipleLexeme",
            "OrientationParticipleLexeme",
            "DeclarationIntransitiveVerb",
            "IntransitiveVerb",
            "DeclarationTransitiveVerb",
            "TransitiveVerb",
            "DeclarationNumerativeVerb",
            "NumerativeVerb",
            "DeclarationSearchForVerb",
            "DeclarationDamageParticipleHead",
            "DamageParticipleHead",
            "DeclarationMovementParticipleHead",
            "MovementParticipleHead",
            "DeclarationOrientationParticipleHead",
            "OrientationParticipleHead",
            "DeclarationDeclaredTransitiveParticipleHead",
            "DeclarationTypeNoun",
            "TypeNoun",
            "DeclarationArtifactSubtypeNoun",
            "ArtifactSubtypeNoun",
            "DeclarationBattleSubtypeNoun",
            "BattleSubtypeNoun",
            "DeclarationCreatureSubtypeNoun",
            "CreatureSubtypeNoun",
            "DeclarationEnchantmentSubtypeNoun",
            "EnchantmentSubtypeNoun",
            "DeclarationLandSubtypeNoun",
            "LandSubtypeNoun",
            "DeclarationPlaneswalkerSubtypeNoun",
            "PlaneswalkerSubtypeNoun",
            "DeclarationSpellSubtypeNoun",
            "SpellSubtypeNoun",
            "SelfReferenceSpelling",
            "CardName",
            "CardinalNumber",
            "ScalarNumber",
            "LoyaltyMagnitude",
            "ReflexiveSubordinateKind",
            "CatalogProvider",
            "DeclarationClass",
            "TerminalClass",
            "LexicalProvenanceKind",
            "LexicalOwner",
            "BuildViolation",
            "BuildRejection",
            "NonterminalCategory",
        ],
        "the complete public generated type inventory is source ordered",
    );
}

#[allow(
    clippy::too_many_lines,
    reason = "the generated privacy and accessor inventory is deliberately literal"
)]
#[test]
fn generated_invariant_production_fields_have_exact_privacy_and_accessors() {
    use syn::Fields;
    use syn::ImplItem;
    use syn::Item;
    use syn::Type;
    use syn::Visibility;

    let source =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/constructions.rs"))
            .expect("production construction source is readable");
    let invocation = deckmaste_construction_core::invocation_from_source(&source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction inventory compiles");
    let file = syn::parse2::<syn::File>(expansion.tokens()).expect("generated Rust parses");
    assert_complete_public_generated_type_inventory(&file);

    for (product, expected_fields, expected_methods) in [
        ("Plain", &[("body", true)][..], &[][..]),
        (
            "Triggered",
            &[("trigger", true), ("intervening_if", true), ("body", true)][..],
            &[][..],
        ),
        (
            "ModalModeValue",
            &[("sentences", false)][..],
            &["new", "try_new", "sentences"][..],
        ),
        (
            "PlainModal",
            &[("chooser", false), ("bounds", false), ("modes", false)][..],
            &["new", "try_new", "chooser", "bounds", "modes"][..],
        ),
        (
            "WithWhere",
            &[("body", true), ("clause", true)][..],
            &[][..],
        ),
        (
            "UnqualifiedControllerStage",
            &[("reference", true)][..],
            &[][..],
        ),
        (
            "YouControl",
            &[("controller", false)][..],
            &["new", "try_new", "controller"][..],
        ),
        (
            "OpponentController",
            &[("controller", false)][..],
            &["new", "try_new", "controller"][..],
        ),
        (
            "YouOwn",
            &[("owner", false)][..],
            &["new", "try_new", "owner"][..],
        ),
        (
            "UnpossessedZone",
            &[("zone", false)][..],
            &["new", "try_new", "zone"][..],
        ),
        (
            "CountComparisonReference",
            &[("count", false), ("comparison", true), ("selector", true)][..],
            &["new", "try_new", "count"][..],
        ),
        (
            "SourceSelfReference",
            &[("spelling", false)][..],
            &["new", "try_new", "spelling"][..],
        ),
        (
            "PossessiveSelfReference",
            &[("spelling", false)][..],
            &["new", "try_new", "spelling"][..],
        ),
        ("PossessiveNoun", &[("head", true)][..], &[][..]),
        ("PossessiveValue", &[("owner", true)][..], &[][..]),
        ("OracleText", &[("blocks", true)][..], &[][..]),
    ] {
        let structure = file
            .items
            .iter()
            .find_map(|item| match item {
                Item::Struct(structure) if structure.ident == product => Some(structure),
                _ => None,
            })
            .unwrap_or_else(|| panic!("generated {product} struct exists"));
        let Fields::Named(fields) = &structure.fields else {
            panic!("generated {product} fields are named");
        };
        assert_eq!(
            fields
                .named
                .iter()
                .map(|field| (
                    field.ident.as_ref().unwrap().to_string(),
                    matches!(field.vis, Visibility::Public(_)),
                ))
                .collect::<Vec<_>>(),
            expected_fields
                .iter()
                .map(|(name, public)| ((*name).to_owned(), *public))
                .collect::<Vec<_>>(),
            "{product} has the exact sealed field privacy",
        );

        let implementation = file.items.iter().find_map(|item| match item {
            Item::Impl(implementation)
                if implementation.trait_.is_none()
                    && matches!(
                        implementation.self_ty.as_ref(),
                        Type::Path(path) if path.path.is_ident(product)
                    ) =>
            {
                Some(implementation)
            }
            _ => None,
        });
        let methods = implementation
            .into_iter()
            .flat_map(|implementation| &implementation.items)
            .filter_map(|item| match item {
                ImplItem::Fn(method) => Some(method.sig.ident.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            methods, expected_methods,
            "{product} has no manufactured accessor or authored implementation",
        );
    }

    for (enumeration, expected_variants) in [
        ("Ability", &["Plain", "Triggered", "Activated"][..]),
        ("AbilityBody", &["Sentences", "PlainModal"][..]),
        ("ModalMode", &["ModalMode"][..]),
        ("DocumentBlock", &["Ability"][..]),
    ] {
        let item = file
            .items
            .iter()
            .find_map(|item| match item {
                Item::Enum(item) if item.ident == enumeration => Some(item),
                _ => None,
            })
            .unwrap_or_else(|| panic!("generated {enumeration} enum exists"));
        assert_eq!(
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
            expected_variants,
            "{enumeration} has the exact generated variant inventory",
        );
    }
}

#[test]
fn parse_error_is_a_standard_error_and_converts_to_anyhow() {
    fn require_standard_error(error: &(impl std::error::Error + ?Sized)) {
        let _ = error;
    }

    let error = ParseError::Failure {
        span: TextSpan { start: 11, end: 11 },
        expectations: BTreeSet::from([Expectation::Literal("life")]),
    };
    require_standard_error(&error);

    let _: anyhow::Error = error.into();
}

fn creature() -> Noun {
    let environment = environment();
    Noun::Type(TypeNoun::Declaration(
        DeclarationTypeNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Creature"),
        )
        .expect("Creature is a normalized noun declaration"),
    ))
}

fn creatures() -> Noun {
    let environment = environment();
    Noun::Type(TypeNoun::Declaration(
        DeclarationTypeNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Creature"),
        )
        .expect("Creature has a normalized plural noun reading"),
    ))
}

fn target_creature() -> Object {
    nominal_object(target_noun(creature()))
}

fn variable_x() -> Amount {
    Amount::Variable(VariableAmount {
        variable: Variable::X,
    })
}

fn atomic(predicate: VerbPhrase) -> Predicate {
    Predicate::Atomic(predicate)
}

fn finite_clause(subject: Subject, predicate: VerbPhrase) -> FiniteClause {
    FiniteClause::PlainFiniteClause(
        PlainFiniteClause::new(subject, atomic(predicate))
            .expect("the helper supplies matching subject-predicate agreement"),
    )
}

fn declarative(subject: Subject, predicate: VerbPhrase) -> Sentence {
    Sentence::Declarative(Declarative {
        clause: Clause::Finite(finite_clause(subject, predicate)),
    })
}

fn sentences(values: Vec<Sentence>) -> AbilityBody {
    AbilityBody::Sentences(Sentences::new(values).expect("one or more sentences construct a body"))
}

fn paragraph(sentence: Sentence) -> Ability {
    Ability::Plain(Plain {
        body: sentences(vec![sentence]),
    })
}

fn destroy(object: Object) -> VerbPhrase {
    let environment = environment();
    let head = DeclarationTransitiveVerb::new(
        &environment,
        DeclarationId::new(DeclarationKind::KeywordAction, "Destroy"),
    )
    .expect("the builtin grammar declares transitive Destroy");
    VerbPhrase::TransitivePredicate(TransitivePredicate {
        head: TransitiveVerb::Declaration(head),
        object,
    })
}

fn connive() -> VerbPhrase {
    let environment = environment();
    let head = DeclarationIntransitiveVerb::new(
        &environment,
        DeclarationId::new(DeclarationKind::KeywordAction, "Connive"),
    )
    .expect("the builtin grammar declares intransitive Connive");
    VerbPhrase::IntransitivePredicate(IntransitivePredicate {
        head: IntransitiveVerb::Declaration(head),
    })
}

fn destroy_target_creature() -> Ability {
    paragraph(Sentence::Imperative(
        Imperative::new(atomic(destroy(target_creature())))
            .expect("destroy is a valid bare imperative predicate"),
    ))
}

fn connive_event() -> FiniteClause {
    finite_clause(
        nominal_subject(indefinite(Noun::Lexeme(CommonNoun::Player))),
        connive(),
    )
}

fn triggered(trigger_clause: FiniteClause, consequences: Vec<Sentence>) -> Ability {
    Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(Finite {
            marker: TriggerMarker::Whenever,
            clause: Clause::Finite(trigger_clause),
        }),
        intervening_if: None,
        body: sentences(consequences),
    })
}

fn triggered_damage() -> Ability {
    triggered(
        connive_event(),
        vec![declarative(
            nominal_subject(that_noun(creature())),
            VerbPhrase::DealDamage(DealDamage {
                amount: variable_x(),
                recipient: damage_recipient(object_it()),
            }),
        )],
    )
}

fn gain_life_sentence() -> Sentence {
    declarative(
        subject_you(),
        VerbPhrase::GainLife(GainLife {
            amount: variable_x(),
        }),
    )
}

fn gain_life_with_where() -> Ability {
    paragraph(Sentence::WithWhere(WithWhere {
        body: Box::new(gain_life_sentence()),
        clause: WhereClauseCategory::Where(WhereClause {
            variable: Variable::X,
            value: nominal_object(creatures_you_control_with_power_at_most_two()),
        }),
    }))
}

fn zacama_deals_damage() -> Ability {
    paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            ),
        ))),
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            recipient: damage_recipient(target_creature()),
        }),
    ))
}

fn triggered_gain_life() -> Ability {
    triggered(connive_event(), vec![gain_life_sentence()])
}

#[test]
fn generic_root_api_preserves_types_and_sentence_root_metadata() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";

    let ability: ParseAnalysis<Ability> = parser.analyze(text, &context);
    let sentence: ParseAnalysis<Sentence> = parser.analyze_sentence(text, &context);
    let trace: ParserTrace<Sentence> =
        parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));

    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(expected),
    }) = destroy_target_creature()
    else {
        panic!("the focused fixture is an ordinary paragraph");
    };
    let expected_sentence = expected.sentences()[0].clone();
    assert_eq!(
        ability.into_parse_result(),
        Ok(Ability::Plain(Plain {
            body: AbilityBody::Sentences(expected.clone()),
        }))
    );
    assert_eq!(sentence.into_parse_result(), Ok(expected_sentence.clone()));
    assert_eq!(trace.root_name(), "Sentence");
    assert_eq!(trace.clone().into_parse_result(), Ok(expected_sentence));
    assert!(
        trace.scanner_matches().items().iter().any(|scanned| {
            scanned.terminal_name_v1() == "Literal(\".\")"
                && scanned.start() == text.len() - 1
                && scanned.end() == text.len()
        }),
        "the focused Sentence root owns its declared period",
    );
    assert!(
        trace
            .scanner_matches()
            .items()
            .iter()
            .all(|scanned| scanned.terminal_name_v1() != "EndOfInput"),
        "Sentence uses its declared eoi = false metadata",
    );
    let ownership = trace.ownership().expect("Sentence ownership is selected");
    assert!(ownership.covered());
    assert!(trace.selected_lexical_claims().items().iter().any(|claim| {
        claim.span()
            == TextSpan {
                start: text.len() - 1,
                end: text.len(),
            }
            && claim.stable_owner_id() == "root:Sentence/punctuation"
    }));
}

#[test]
fn generic_root_adapter_is_only_applied_at_the_outer_recursive_sentence_boundary() {
    let parser = parser();
    let context = context("Context Card");
    let text =
        "You gain X life, where X is the number of creatures you control with power 2 or less.";
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(expected),
    }) = gain_life_with_where()
    else {
        panic!("the recursive Sentence fixture is an ordinary paragraph");
    };
    let expected = expected.sentences()[0].clone();

    let trace = parser.trace_sentence(text, &context, TraceLimits::new(usize::MAX));

    assert_eq!(trace.clone().into_parse_result(), Ok(expected));
    assert_eq!(
        trace
            .scanner_matches()
            .items()
            .iter()
            .filter(|scanned| scanned.terminal_name_v1() == "Literal(\".\")")
            .count(),
        1,
        "only the true Sentence root boundary scans its declared period",
    );
}

#[test]
fn parses_and_round_trips_the_five_slice_abilities() {
    let parser = parser();
    for (text, card_name, expected) in [
        (
            "Destroy target creature.",
            "Context Card",
            destroy_target_creature(),
        ),
        (
            "Whenever a player connives, that creature deals X damage to it.",
            "Context Card",
            triggered_damage(),
        ),
        (
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            "Context Card",
            gain_life_with_where(),
        ),
        (
            "Zacama deals 3 damage to target creature.",
            "Zacama, Primal Calamity",
            zacama_deals_damage(),
        ),
        (
            "Whenever a player connives, you gain X life.",
            "Context Card",
            triggered_gain_life(),
        ),
    ] {
        let context = if card_name == "Zacama, Primal Calamity" {
            legendary_context(card_name)
        } else {
            context(card_name)
        };
        assert_eq!(parser.parse(text, &context), Ok(expected.clone()));
        assert_eq!(
            parser.parse(text, &context),
            parser.analyze(text, &context).into_parse_result()
        );
        for limit in [0, 1, usize::MAX] {
            assert_eq!(
                parser.parse(text, &context),
                parser
                    .trace(text, &context, TraceLimits::new(limit))
                    .into_parse_result(),
                "{text:?} at cap {limit}",
            );
        }
        assert_eq!(expected.render(&context, &environment()), text);

        let rendered = expected.render(&context, &environment());
        assert_eq!(parser.parse(&rendered, &context), Ok(expected));
    }
}

#[test]
fn parser_analysis_repeats_exactly_and_preserves_selected_rendered_bytes() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";
    let first = parser.analyze(text, &context);
    let second = parser.analyze(text, &context);

    assert_eq!(first, second);
    assert_eq!(
        first.selected().unwrap().render(&context, &environment()),
        text
    );
    assert_eq!(
        first.decision().unwrap().candidates()[0].construction_path(),
        [
            "AbilityPlain".to_owned(),
            "AbilityBodySentences".to_owned(),
            "SentenceImperative".to_owned(),
            "VerbPhraseTransitivePredicate".to_owned(),
            "ObjectObjectNominal".to_owned(),
            "NounPhraseQualifiedNounPhrase".to_owned(),
            "NumericStageUnqualifiedNumericStage".to_owned(),
            "ZoneStageUnqualifiedZoneStage".to_owned(),
            "ControllerStageUnqualifiedControllerStage".to_owned(),
            "UnqualifiedReferenceOrdinarySingularReference".to_owned(),
            "DeterminerPhraseTargetDeterminerPhrase".to_owned(),
            "SingularNominalBareSingularNominal".to_owned(),
            "SingularHeadTypeSingularHead".to_owned(),
        ]
    );
    assert_eq!(parser.parse(text, &context), first.into_parse_result(),);
}

#[test]
fn parser_analysis_retains_complete_lexical_ownership() {
    let parser = parser();
    let context = context("Context Card");
    let analysis = parser.analyze("Destroy target creature.", &context);
    let ownership = analysis.ownership().expect("selected parse owns terminals");
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert_eq!(ownership.parsed_claims().len(), 4);
    assert_eq!(ownership.rendered_claims().len(), 4);
    assert_eq!(ownership.rendered_text(), "Destroy target creature.");
    let summary = ownership.summary();
    assert_eq!((summary.claims(), summary.claimed_bytes()), (4, 24));
    assert_eq!(
        (summary.form_literal_claims(), summary.form_literal_bytes()),
        (2, 8)
    );
    assert_eq!((summary.lexeme_claims(), summary.lexeme_bytes()), (2, 16));
    assert_eq!(summary.vocab_claims(), 0);
    assert_eq!(summary.codec_claims(), 0);
    assert_eq!(summary.identity_claims(), 0);
    assert_eq!(summary.gap_spans(), 0);
    assert_eq!(summary.overlap_spans(), 0);
    assert_eq!(summary.synthetic_claims(), 0);
    assert_eq!(summary.provenance_plan_mismatches(), 0);
}

#[test]
fn generated_morphology_closed_owner_ids_match_scan_and_render_claims() {
    let parser = parser();
    let context = context("Context Card");
    for (text, expected) in [
        (
            "Deal X damage to target creature.",
            "lexeme:VerbLexeme/Deal/bare",
        ),
        (
            "It deals X damage to target creature.",
            "lexeme:VerbLexeme/Deal/third_person_singular",
        ),
        (
            "Whenever a player connives, you gain X life.",
            "lexeme:CommonNoun/Player/singular",
        ),
        (
            "Those players deal X damage to it.",
            "lexeme:CommonNoun/Player/plural",
        ),
        (
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            "lexeme:VerbLexeme/Be/third_person_singular",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("no ownership for {text:?}"));
        assert!(
            ownership
                .parsed_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == expected),
            "scanner claims lack {expected:?} for {text:?}: {:?}",
            ownership.parsed_claims(),
        );
        assert!(
            ownership
                .rendered_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == expected),
            "render claims lack {expected:?} for {text:?}: {:?}",
            ownership.rendered_claims(),
        );
    }
}

#[test]
fn explicit_named_card_identity_scans_exact_longest_renders_and_owns() {
    let parser = parser();
    let environment = environment();
    let context = context("Context Card");
    let text = "Destroy a card named Alpha Beta.";
    let parsed = parser
        .parse(text, &context)
        .expect("explicit card name parses");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(paragraph),
    }) = &parsed
    else {
        panic!("named-card sentence is a paragraph: {parsed:?}");
    };
    let [Sentence::Imperative(imperative)] = paragraph.sentences() else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let Predicate::Atomic(VerbPhrase::TransitivePredicate(TransitivePredicate {
        head: TransitiveVerb::Declaration(_),
        object: Object::ObjectNominal(NominalObject { value }),
    })) = imperative.predicate()
    else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    let UnqualifiedReference::NamedCardReference(NamedCardReference { name }) =
        unqualified_reference(value)
    else {
        panic!("explicit card name has its generated AST construction: {parsed:?}");
    };
    assert_eq!(name.provider(), CatalogProvider::CardNames);
    assert_eq!(name.canonical_identity(), "alpha-beta");
    assert_eq!(parsed.render(&context, &environment), text);

    let ownership = parser
        .analyze(text, &context)
        .ownership()
        .expect("named identity owns its source bytes")
        .clone();
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert!(ownership.parsed_claims().iter().any(|claim| {
        claim.kind() == LexicalProvenanceKind::Identity
            && claim.stable_owner_id() == "identity:CardNames/alpha-beta"
            && &text[claim.span().start..claim.span().end] == " Alpha Beta"
    }));
    assert!(ownership.rendered_claims().iter().any(|claim| {
        claim.kind() == LexicalProvenanceKind::Identity
            && claim.stable_owner_id() == "identity:CardNames/alpha-beta"
    }));

    assert!(
        parser
            .parse("Destroy a card named alpha beta.", &context)
            .is_err(),
        "catalog identity scan preserves exact case"
    );
    let punctuation = "Destroy a card named Urza's Saga.";
    let punctuation_ast = parser
        .parse(punctuation, &context)
        .expect("punctuation-bearing identity parses");
    assert_eq!(punctuation_ast.render(&context, &environment), punctuation);
}

#[test]
fn bare_own_card_name_remains_unique_source_self_reference() {
    let parser = parser();
    let context = context("Seven Dwarves");
    let text = "Seven Dwarves gains 2 life.";
    let analysis = parser.analyze(text, &context);
    let selected = analysis.selected().expect("source self-reference selects");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(paragraph),
    }) = selected
    else {
        panic!("self-reference sentence is a paragraph: {selected:?}");
    };
    let [
        Sentence::Declarative(Declarative {
            clause: Clause::Finite(FiniteClause::PlainFiniteClause(clause)),
        }),
    ] = paragraph.sentences()
    else {
        panic!("bare own card name remains source self-reference: {selected:?}");
    };
    let Subject::SubjectNominal(NominalSubject { value }) = clause.subject() else {
        panic!("bare own card name remains source self-reference: {selected:?}");
    };
    assert!(matches!(
        unqualified_reference(value),
        UnqualifiedReference::SelfReference(_)
    ));
    assert_eq!(
        analysis
            .decision()
            .expect("selected analysis retains its decision")
            .candidates()
            .len(),
        1,
        "bare catalog identity contributes no rival noun-phrase construction"
    );
}

#[test]
fn parser_analysis_ownership_covers_every_kind_unicode_and_multitoken_identity() {
    let parser = parser();
    let cases = [
        (
            "Whenever a player connives, that creature deals X damage to it.",
            "Context Card",
        ),
        (
            "Context Card deals 3 damage to target creature.",
            "Context Card",
        ),
        (
            concat!("E", "\u{301}", "lan deals 3 damage to target creature."),
            concat!("E", "\u{301}", "lan"),
        ),
    ];
    let mut kinds = BTreeSet::new();
    for (text, card_name) in cases {
        let context = context(card_name);
        let analysis = parser.analyze(text, &context);
        let ownership = analysis.ownership().expect("selected parse owns terminals");
        assert!(
            ownership.failures().is_empty(),
            "{text:?}: {:?}",
            ownership.failures()
        );
        assert!(ownership.summary().covered(), "{text:?}");
        assert_eq!(ownership.rendered_text(), text);
        let mut cursor = 0;
        for claim in ownership.parsed_claims() {
            assert_eq!(claim.span().start, cursor, "{text:?}");
            assert!(claim.span().end > claim.span().start, "{text:?}");
            assert!(text.is_char_boundary(claim.span().start), "{text:?}");
            assert!(text.is_char_boundary(claim.span().end), "{text:?}");
            kinds.insert(claim.kind());
            cursor = claim.span().end;
        }
        assert_eq!(cursor, text.len(), "{text:?}");
    }
    assert_eq!(
        kinds,
        BTreeSet::from([
            LexicalProvenanceKind::FormLiteral,
            LexicalProvenanceKind::Vocab,
            LexicalProvenanceKind::Lexeme,
            LexicalProvenanceKind::Codec,
            LexicalProvenanceKind::Identity,
        ])
    );
}

#[test]
fn parser_analysis_projects_representative_parse_failures_without_changing_them() {
    let parser = parser();
    for text in [
        "Destroy target creature",
        "You gains X life.",
        "Destroy target flying.",
    ] {
        let context = context("Context Card");
        assert_eq!(
            parser.parse(text, &context),
            parser.analyze(text, &context).into_parse_result(),
            "{text:?}",
        );
        assert!(parser.analyze(text, &context).ownership().is_none());
        for limit in [0, 1, usize::MAX] {
            assert_eq!(
                parser.parse(text, &context),
                parser
                    .trace(text, &context, TraceLimits::new(limit))
                    .into_parse_result(),
                "{text:?} at cap {limit}",
            );
        }
    }
}

fn assert_bounded<T>(bounded: &deckmaste_english_v2::parser::Bounded<T>, limit: usize) {
    assert_eq!(bounded.total(), bounded.shown() + bounded.omitted());
    assert_eq!(bounded.shown(), bounded.items().len());
    assert!(bounded.shown() <= limit);
}

#[test]
fn parser_trace_selected_projection_is_exact_bounded_repeatable_and_private_result_preserving() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature.";

    for limit in [0, 1, usize::MAX] {
        let limits = TraceLimits::new(limit);
        let trace = parser.trace(text, &context, limits);
        let repeated = parser.trace(text, &context, limits);
        assert_eq!(trace, repeated);
        assert_bounded(trace.scanner_matches(), limit);
        assert_bounded(trace.chart(), limit);
        assert_bounded(trace.forest(), limit);
        assert_bounded(trace.accepted_roots(), limit);
        assert_bounded(trace.checked_completion_rejections(), limit);
        assert_bounded(trace.materialized_candidates(), limit);
        assert_bounded(trace.materialization_cycles(), limit);

        let BoundedParseOutcome::Selected(selected) = trace.outcome() else {
            panic!("rendered slice must select");
        };
        assert_eq!(selected.rendered(), text);
        let selection = selected.selection();
        assert_eq!(
            selection.resolution(),
            deckmaste_english_v2::parser::SelectionResolution::Unique
        );
        assert_bounded(selection.candidates(), limit);
        assert_bounded(selection.comparisons(), limit);
        assert_bounded(selection.survivors(), limit);
        assert_bounded(selection.exception_uses(), limit);
        assert_bounded(selection.unselected_candidates(), limit);
        let selected_candidate = selection.selected().expect("selected metadata");
        assert_eq!(selected_candidate.ordinal(), 0);
        assert_bounded(selected_candidate.construction_path(), limit);
        assert_bounded(selected_candidate.specificity(), limit);

        if limit > 0 {
            let candidate = &trace.materialized_candidates().items()[0];
            assert_eq!(candidate.construction_path().total(), 13);
            assert_eq!(candidate.construction_path().shown(), usize::min(limit, 13));
            assert_eq!(candidate.specificity().total(), 15);
            assert_eq!(candidate.specificity().shown(), usize::min(limit, 15));
        }

        if limit == usize::MAX {
            let candidate = &trace.materialized_candidates().items()[0];
            let parsed = parser.parse(text, &context).expect("selected ability");
            let analysis = parser.analyze(text, &context);
            let complete = analysis.decision().expect("complete Task 3 decision");
            assert_eq!(candidate.ordinal(), 0);
            assert_eq!(candidate.rendered(), text);
            assert_eq!(candidate.ast_debug_v1(), format!("{parsed:?}"));
            assert_eq!(
                candidate.construction_path().items(),
                [
                    "AbilityPlain",
                    "AbilityBodySentences",
                    "SentenceImperative",
                    "VerbPhraseTransitivePredicate",
                    "ObjectObjectNominal",
                    "NounPhraseQualifiedNounPhrase",
                    "NumericStageUnqualifiedNumericStage",
                    "ZoneStageUnqualifiedZoneStage",
                    "ControllerStageUnqualifiedControllerStage",
                    "UnqualifiedReferenceOrdinarySingularReference",
                    "DeterminerPhraseTargetDeterminerPhrase",
                    "SingularNominalBareSingularNominal",
                    "SingularHeadTypeSingularHead",
                ]
            );
            assert_eq!(candidate.specificity().total(), 15);
            assert!(selection.unselected_candidates().items().is_empty());
            assert_eq!(selection.resolution(), complete.resolution());
            assert_eq!(selection.survivors().items(), complete.survivors());
            assert_eq!(
                selection.exception_uses().items(),
                complete.exception_uses()
            );
            assert_eq!(
                selection.candidates().items()[0]
                    .construction_path()
                    .items(),
                complete.candidates()[0].construction_path()
            );
            assert_eq!(
                selection.candidates().items()[0].specificity().items(),
                complete.candidates()[0].specificity()
            );
        }

        assert_eq!(
            parser.parse(text, &context),
            trace.into_parse_result(),
            "public cap {limit} must not truncate the private legacy result",
        );
    }
}

#[test]
fn parser_trace_parse_failure_bounds_expectations_without_truncating_private_error() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature";
    let complete = parser.parse(text, &context);

    for (limit, expected_shown) in [(0, 0), (1, 1), (8, 8)] {
        let trace = parser.trace(text, &context, TraceLimits::new(limit));
        let BoundedParseOutcome::ParseFailure(failure) = trace.outcome() else {
            panic!("missing period is an ordinary parse failure");
        };
        assert_eq!(
            failure.span(),
            TextSpan {
                start: text.len(),
                end: text.len()
            }
        );
        assert_eq!(failure.expectations().total(), 48);
        assert_eq!(failure.expectations().shown(), expected_shown);
        assert_eq!(failure.expectations().omitted(), 48 - expected_shown);
        if limit > 0 {
            assert_eq!(
                failure.expectations().items()[0],
                deckmaste_english_v2::parser::ExpectationInfo::Nonterminal(
                    NonterminalCategory::SingularHead,
                )
            );
        }
        assert_eq!(complete, trace.into_parse_result());
    }
}

#[test]
fn parser_trace_public_loser_reason_spellings_are_stable() {
    assert_eq!(SelectionLoserReason::LessSpecific.as_str(), "less_specific");
    assert_eq!(
        SelectionLoserReason::ExceptionLoser.as_str(),
        "exception_loser"
    );
    assert_eq!(
        SelectionLoserReason::UnresolvedSurvivor.as_str(),
        "unresolved_survivor"
    );
}

#[test]
fn no_comma_self_reference_parses_once_as_full_and_round_trips() {
    let text = "Context Card deals 3 damage to target creature.";
    let context = context("Context Card");
    let expected = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(SelfReferenceSpelling::Full, "Context Card"),
        ))),
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            recipient: damage_recipient(target_creature()),
        }),
    ));

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn self_reference_identity_preserves_its_inherent_case() {
    let text = "eBay deals 3 damage to target creature.";
    let context = context("eBay");
    let expected = paragraph(declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(SelfReferenceSpelling::Full, "eBay"),
        ))),
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            recipient: damage_recipient(target_creature()),
        }),
    ));

    assert_eq!(parser().parse(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, &environment()), text);
}

#[test]
fn disallowed_declaration_kind_is_a_parse_failure() {
    let text = "Destroy target flying.";
    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 15, end: 21 },
            expectations: BTreeSet::from([
                Expectation::Nonterminal(NonterminalCategory::SingularHead),
                Expectation::Nonterminal(NonterminalCategory::PluralHead),
                Expectation::Nonterminal(NonterminalCategory::NominalModifier),
                Expectation::Nonterminal(NonterminalCategory::NegativeNominalModifier),
                Expectation::Nonterminal(NonterminalCategory::CoordinatedNominalModifier),
                Expectation::Nonterminal(NonterminalCategory::CompoundNominalModifier),
                Expectation::Nonterminal(NonterminalCategory::SingularNominal),
                Expectation::Nonterminal(NonterminalCategory::PluralNominal),
                Expectation::Nonterminal(NonterminalCategory::SingularCoordinationMember),
                Expectation::Nonterminal(NonterminalCategory::PluralCoordinationMember),
                Expectation::Nonterminal(NonterminalCategory::SingularNominalCoordination),
                Expectation::Nonterminal(NonterminalCategory::PluralNominalCoordination),
                Expectation::Terminal(TerminalClass::Color),
                Expectation::Terminal(TerminalClass::Status),
                Expectation::Terminal(TerminalClass::NonTargetCommonModifier),
                Expectation::Terminal(TerminalClass::Supertype),
                Expectation::Terminal(TerminalClass::Noun),
                Expectation::Terminal(TerminalClass::DeclarationNoun(51)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(52)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(53)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(54)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(55)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(56)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(57)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(58)),
                Expectation::Literal("non"),
                Expectation::Literal("non-"),
            ]),
        })
    );
}

#[test]
fn missing_period_reports_chart_derived_literal_expectation() {
    let text = "Destroy target creature";
    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan {
                start: text.len(),
                end: text.len(),
            },
            expectations: BTreeSet::from([
                Expectation::Nonterminal(NonterminalCategory::SingularHead),
                Expectation::Nonterminal(NonterminalCategory::PluralHead),
                Expectation::Nonterminal(NonterminalCategory::NominalModifier),
                Expectation::Nonterminal(NonterminalCategory::CompoundNominalModifier),
                Expectation::Nonterminal(NonterminalCategory::SingularNominal),
                Expectation::Nonterminal(NonterminalCategory::PluralNominal),
                Expectation::Nonterminal(NonterminalCategory::DeterminerPhrase),
                Expectation::Nonterminal(NonterminalCategory::ControllerOwnerQualification),
                Expectation::Nonterminal(NonterminalCategory::SingularController),
                Expectation::Nonterminal(NonterminalCategory::ZoneQualification),
                Expectation::Nonterminal(NonterminalCategory::ScalarQualification),
                Expectation::Terminal(TerminalClass::SubjectPronoun),
                Expectation::Terminal(TerminalClass::Color),
                Expectation::Terminal(TerminalClass::Status),
                Expectation::Terminal(TerminalClass::Supertype),
                Expectation::Terminal(TerminalClass::Noun),
                Expectation::Terminal(TerminalClass::DeclarationNoun(51)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(52)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(53)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(54)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(55)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(56)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(57)),
                Expectation::Terminal(TerminalClass::DeclarationNoun(58)),
                Expectation::Literal(" and "),
                Expectation::Literal(" and/or "),
                Expectation::Literal(" or "),
                Expectation::Literal(","),
                Expectation::Literal(", "),
                Expectation::Literal(", then "),
                Expectation::Literal("."),
                Expectation::Literal(": "),
                Expectation::Literal("a"),
                Expectation::Literal("an"),
                Expectation::Literal("another"),
                Expectation::Literal("at"),
                Expectation::Literal("from"),
                Expectation::Literal("if"),
                Expectation::Literal("in"),
                Expectation::Literal("non"),
                Expectation::Literal("non-"),
                Expectation::Literal("of"),
                Expectation::Literal("other"),
                Expectation::Literal("target"),
                Expectation::Literal("that"),
                Expectation::Literal("this"),
                Expectation::Literal("unless"),
                Expectation::Literal("with"),
            ]),
        })
    );
}

#[test]
fn agreement_mismatch_reports_a_nonempty_chart_failure() {
    let text = "You gains X life.";

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 16, end: 17 },
            expectations: BTreeSet::from([
                Expectation::Literal(" and "),
                Expectation::Literal(" and/or "),
                Expectation::Literal(" or "),
                Expectation::Literal(", "),
            ]),
        })
    );
}

#[test]
fn count_controller_mismatch_reports_a_nonempty_chart_failure() {
    let text =
        "You gain X life, where X is the number of creatures it controls with power 2 or less.";

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan { start: 62, end: 63 },
            expectations: BTreeSet::from([Expectation::Literal(".")]),
        })
    );
}

#[test]
fn lexical_matches_reject_prefixes_of_longer_lexemes() {
    for (text, card_name, expected_span) in [
        (
            "Destroyed target creature.",
            "Context Card",
            TextSpan { start: 0, end: 9 },
        ),
        (
            "Zacama deals 3x damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 13, end: 15 },
        ),
        (
            "Destroy target creaturex.",
            "Context Card",
            TextSpan { start: 23, end: 24 },
        ),
        (
            "Zacamaé deals 3 damage to target creature.",
            "Zacama, Primal Calamity",
            TextSpan { start: 0, end: 8 },
        ),
    ] {
        let Err(ParseError::Failure { span, expectations }) = parser().parse(
            text,
            &if card_name == "Zacama, Primal Calamity" {
                legendary_context(card_name)
            } else {
                context(card_name)
            },
        ) else {
            panic!("lexical prefix must fail for {text:?}");
        };

        assert_eq!(span, expected_span, "{text:?}");
        assert!(!expectations.is_empty(), "{text:?}");
    }
}

#[test]
fn decimal_punctuation_cannot_split_a_signed_number_in_a_complete_document() {
    let text = "Gain 1.0 life.";
    let Err(ParseError::Failure { span, expectations }) =
        parser().parse(text, &context("Context Card"))
    else {
        panic!("a decimal fraction is outside the signed-decimal grammar");
    };

    assert_eq!(span, TextSpan { start: 6, end: 7 });
    assert!(!expectations.is_empty());
}

#[test]
fn doubled_period_reports_the_first_trailing_byte() {
    let text = "Destroy target creature..";
    let trailing = text.len() - 1;

    assert_eq!(
        parser().parse(text, &context("Context Card")),
        Err(ParseError::Failure {
            span: TextSpan {
                start: trailing,
                end: trailing + 1,
            },
            expectations: BTreeSet::from([
                Expectation::Terminal(TerminalClass::EndOfInput),
                Expectation::Literal(" "),
            ]),
        })
    );
}

#[test]
fn wrong_case_and_wrong_whitespace_are_parse_failures() {
    for text in [
        "destroy target creature.",
        "Destroy Target creature.",
        "Destroy  target creature.",
        "Destroytarget creature.",
        "Whenever a player connives, you Gain X life.",
    ] {
        assert!(parser().parse(text, &context("Context Card")).is_err());
    }
}
