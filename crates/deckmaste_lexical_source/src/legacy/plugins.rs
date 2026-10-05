use std::path::Path;

use crate::LoadError;
use deckmaste_construction_core::macro_def as metadata;
use deckmaste_lexical::Binding;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::WordForm;

use crate::LexicalSources;
use crate::compound::Declaration as PendingCompoundNoun;
use crate::source;

pub(crate) fn load(
    root: &Path,
    output: &mut LexicalSources,
) -> Result<Vec<PendingCompoundNoun>, LoadError> {
    let mut compounds = Vec::new();
    let declarations = metadata::read_builtin_v2(root.join("plugins_v2/builtin"))?;
    let reader = metadata::declaration_macro_set()?;
    for normalized in declarations {
        let path = normalized.provenance().path();
        let text = crate::error::read(path)?;
        let definition: macro_ron::MacroDef<metadata::Metadata> =
            reader.read_str(&text).map_err(|source| LoadError::Decode {
                path: path.into(),
                source: Box::new(source),
            })?;
        let owner = format!(
            "lexeme:{}/{}",
            declaration_kind(normalized.identity().kind()),
            normalized.identity().name()
        );
        let path = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
        if let Some(recipe) = normalized.compound_noun() {
            compounds.push(PendingCompoundNoun {
                source: source(SourceKind::Plugin, &path, &owner),
                recipe: recipe.clone(),
                noun_class: normalized.noun_class(),
            });
        }
        export_metadata(&owner, &path, definition.metadata, &normalized, output);
    }
    Ok(compounds)
}

fn export_metadata(
    owner: &str,
    path: &str,
    authored: metadata::Metadata,
    normalized: &metadata::NormalizedDeclaration,
    output: &mut LexicalSources,
) {
    let provenance = source(SourceKind::Plugin, path, owner);
    let Some(grammar) = authored.grammar else {
        output.unmapped.push(format!(
            "plugin {owner}: no declared lexical grammar; spelling {:?}",
            authored.spelling
        ));
        return;
    };
    let mut lexeme = grammar_lexeme(owner, path, grammar, provenance, normalized, output);
    export_distribution(&mut lexeme, normalized, authored.noun_class);
    if let Some(type_word) = normalized.type_word() {
        if type_word.noun_modifier {
            lexeme
                .properties
                .features
                .insert("NounPremodifier".into(), "Yes".into());
        }
        output.lexemes.push(crate::native::negative_lexeme(
            &lexeme,
            type_word.negative_prefix_join,
        ));
    }
    // Parameterized spelling remains a grammar recipe, not guessed ordinary
    // words.
    if normalized
        .spelling()
        .iter()
        .any(|part| matches!(part, metadata::SpellingPart::Param(_)))
    {
        output.unmapped.push(format!(
            "parameterized-spelling {owner}: {:?}",
            authored.spelling
        ));
    }
    output.lexemes.push(lexeme);
}

fn grammar_lexeme(
    owner: &str,
    path: &str,
    grammar: metadata::Grammar,
    provenance: Source,
    normalized: &metadata::NormalizedDeclaration,
    output: &mut LexicalSources,
) -> Lexeme {
    match grammar {
        metadata::Grammar::Verb {
            bare,
            bare_onset,
            third_person,
            third_person_onset,
            preterite,
            preterite_onset,
            participle,
            participle_onset,
            frame_set,
        } => {
            let mut lexeme = Lexeme::verb(owner, bare, provenance);
            lexeme.forms.retain_mut(|slot| match slot.form {
                WordForm::Present
                    if slot.features.number == Some(Number::Singular)
                        && slot.features.person == Some(Person::Third) =>
                {
                    verb_derived(slot, &third_person, owner, output)
                }
                WordForm::Preterite => {
                    slot.surfaces = preterite.clone().map(|value| vec![value]);
                    true
                }
                WordForm::PastParticiple => verb_derived(slot, &participle, owner, output),
                _ => true,
            });
            for (name, onset) in [
                ("PlainOnset", bare_onset),
                ("ThirdPersonOnset", third_person_onset),
                ("PreteriteOnset", preterite_onset),
                ("ParticipleOnset", participle_onset),
            ] {
                if let Some(onset) = onset {
                    lexeme
                        .properties
                        .features
                        .insert(name.into(), format!("{onset:?}"));
                }
            }
            lexeme.properties.frames = frames(&frame_set);
            lexeme
        }
        metadata::Grammar::Noun {
            singular,
            singular_onset,
            plural,
            plural_onset,
        } => {
            let mut lexeme = Lexeme::noun(owner, singular, vec![Countability::Count], provenance);
            lexeme
                .forms
                .retain_mut(|slot| slot.form != WordForm::Plural || derived(slot, &plural));
            for (name, onset) in [
                ("SingularOnset", singular_onset),
                ("PluralOnset", plural_onset),
            ] {
                if let Some(onset) = onset {
                    lexeme
                        .properties
                        .features
                        .insert(name.into(), format!("{onset:?}"));
                }
            }
            lexeme
        }
        metadata::Grammar::FixedTerm { surface, onset }
        | metadata::Grammar::FixedClause { surface, onset } => {
            let mut lexeme = Lexeme::invariant(owner, surface, Category::Keyword, provenance);
            lexeme.source.kind = SourceKind::Keyword;
            if let Some(onset) = onset {
                lexeme
                    .properties
                    .features
                    .insert("Onset".into(), format!("{onset:?}"));
            }
            lexeme
        }
        metadata::Grammar::FixedKeyword {
            surface,
            onset,
            bound_suffix,
            participial_adjective,
            block_label,
            parameter,
        } => {
            if let Some(suffix) = bound_suffix {
                output.lexemes.push(bound_suffix_lexeme(
                    owner,
                    path,
                    suffix,
                    normalized.keyword_parameter_class(),
                ));
            }
            if let Some(adjective) = participial_adjective {
                output.lexemes.push(participial_adjective_lexeme(
                    owner, path, &surface, adjective,
                ));
            }
            if let Some(label) = block_label {
                output.lexemes.push(block_label_lexeme(owner, path, label));
            }
            let mut lexeme = Lexeme::invariant(owner, surface, Category::Keyword, provenance);
            lexeme.source.kind = SourceKind::Keyword;
            if let Some(onset) = onset {
                lexeme
                    .properties
                    .features
                    .insert("Onset".into(), format!("{onset:?}"));
            }
            if let Some(metadata::FixedKeywordParameterGrammar::Quality {
                preposition,
                nominal_number,
            }) = &parameter
            {
                lexeme
                    .properties
                    .features
                    .insert("KeywordMarker".into(), preposition.1.clone());
                lexeme.properties.features.insert(
                    "KeywordQualityNumber".into(),
                    nominal_number.map_or_else(|| "Any".into(), |number| format!("{number:?}")),
                );
            }
            if let Some(parameter) = parameter {
                lexeme
                    .properties
                    .features
                    .insert("KeywordParameter".into(), format!("{parameter:?}"));
            }
            lexeme
        }
    }
}

fn export_distribution(
    lexeme: &mut Lexeme,
    normalized: &metadata::NormalizedDeclaration,
    noun_class: Option<metadata::NounClassSemantics>,
) {
    if let Some(grammar) = normalized.grammar() {
        for surface in grammar.surfaces() {
            if let Some(onset) = surface.onset_override() {
                lexeme.onsets.insert(
                    surface.text().into(),
                    match onset {
                        metadata::Onset::Consonant => deckmaste_lexical::Onset::Consonant,
                        metadata::Onset::Vowel => deckmaste_lexical::Onset::Vowel,
                    },
                );
            }
        }
        lexeme
            .properties
            .features
            .insert("source_recipe".into(), format!("{:?}", grammar.recipe()));
    }
    if let Some(parameter) = normalized.keyword_parameter_class() {
        lexeme.properties.features.insert(
            "KeywordParameterClass".into(),
            match parameter {
                metadata::KeywordParameterClass::Unsupported(class) => format!("{class:?}"),
                class => format!("{class:?}"),
            },
        );
    }
    if let Some(syntax) = normalized.keyword_syntax() {
        lexeme.properties.features.insert(
            "KeywordPayloadOrder".into(),
            format!("{:?}", syntax.payload_order),
        );
        lexeme.properties.features.insert(
            "KeywordSeparator".into(),
            format!("{:?}", syntax.head_separator),
        );
        lexeme.properties.features.insert(
            "KeywordParameterSeparator".into(),
            format!("{:?}", syntax.parameter_separator),
        );
    }
    if let Some(class) = noun_class {
        lexeme.properties.features.insert(
            "locative_temporal_license".into(),
            format!("{:?}", class.locative_temporal_license),
        );
        lexeme
            .properties
            .features
            .insert("relationality".into(), format!("{:?}", class.relationality));
    }
}

fn bound_suffix_lexeme(
    owner: &str,
    path: &str,
    suffix: metadata::BoundSuffixGrammar,
    parameter_class: Option<metadata::KeywordParameterClass>,
) -> Lexeme {
    let suffix_owner = format!("{owner}/bound-suffix");
    let mut lexeme = Lexeme::invariant(
        &suffix_owner,
        suffix.surface,
        Category::Keyword,
        source(SourceKind::Keyword, path, owner),
    );
    lexeme
        .properties
        .features
        .insert("KeywordPayloadOrder".into(), "BoundSuffix".into());
    lexeme
        .properties
        .features
        .insert("BoundKeyword".into(), "Yes".into());
    if let Some(class) = parameter_class {
        lexeme.properties.features.insert(
            "KeywordParameterClass".into(),
            match class {
                metadata::KeywordParameterClass::Unsupported(class) => format!("{class:?}"),
                class => format!("{class:?}"),
            },
        );
    }
    lexeme.binding = Binding::Suffix;
    lexeme.capitalization = Capitalization::Exact;
    lexeme
}

fn participial_adjective_lexeme(
    owner: &str,
    path: &str,
    surface: &str,
    adjective: metadata::ParticipialAdjectiveGrammar,
) -> Lexeme {
    let spelling = match adjective.surface {
        metadata::DerivedSurface::Derived => deckmaste_lexical::default_participle(surface),
        metadata::DerivedSurface::Override(spelling) => spelling,
        metadata::DerivedSurface::Unavailable => {
            unreachable!("the declaration reader rejects an unavailable adjective")
        }
    };
    let mut lexeme = Lexeme::invariant(
        format!("{owner}/adjective"),
        spelling,
        Category::Adjective,
        source(SourceKind::Keyword, path, owner),
    );
    lexeme.properties.features.insert(
        "BareSingularUse".into(),
        if adjective.bare_singular_use { "Yes" } else { "No" }.into(),
    );
    if let Some(onset) = adjective.onset {
        lexeme
            .properties
            .features
            .insert("Onset".into(), format!("{onset:?}"));
    }
    lexeme
}

fn block_label_lexeme(owner: &str, path: &str, label: metadata::BlockLabelGrammar) -> Lexeme {
    let label_owner = format!("{owner}/block-label");
    let mut lexeme = Lexeme::invariant(
        &label_owner,
        label.surface,
        Category::Keyword,
        source(SourceKind::Keyword, path, owner),
    );
    if let Some(onset) = label.onset {
        lexeme
            .properties
            .features
            .insert("Onset".into(), format!("{onset:?}"));
    }
    lexeme
}

fn verb_derived(
    slot: &mut FormDeclaration,
    source: &metadata::DerivedSurface,
    owner: &str,
    output: &mut LexicalSources,
) -> bool {
    if matches!(source, metadata::DerivedSurface::Unavailable) {
        slot.surfaces = None;
        output.unmapped.push(format!("retired-nonattestation {owner}: {:?} uses default morphology; legacy Unavailable no longer withholds an ordinary verb form", slot.form));
        true
    } else {
        derived(slot, source)
    }
}

fn derived(slot: &mut FormDeclaration, source: &metadata::DerivedSurface) -> bool {
    match source {
        metadata::DerivedSurface::Derived => {
            slot.surfaces = None;
            true
        }
        metadata::DerivedSurface::Override(value) => {
            slot.surfaces = Some(vec![value.clone()]);
            true
        }
        metadata::DerivedSurface::Unavailable => false,
    }
}

fn frames(source: &metadata::VerbFrameSet) -> Vec<Frame> {
    let sequences = match source {
        metadata::VerbFrameSet::Intransitive => vec![Vec::new()],
        metadata::VerbFrameSet::Transitive => vec![vec![argument(Relation::Object, "NounPhrase")]],
        metadata::VerbFrameSet::MeasureComplement => {
            vec![vec![argument(Relation::Complement, "MeasurePhrase")]]
        }
        metadata::VerbFrameSet::Custom { frames } => frames
            .iter()
            .map(|frame| frame.iter().map(frame_item).collect())
            .collect(),
    };
    sequences
        .into_iter()
        .map(|items| Frame {
            kind: "Predicate".into(),
            items,
        })
        .collect()
}

fn argument(relation: Relation, category: &str) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation,
        category: category.to_owned(),
    })
}
fn marker(vocabulary: &str, member: &str) -> FrameItem {
    FrameItem::Marker {
        vocabulary: vocabulary.to_owned(),
        member: member.to_owned(),
    }
}
fn relation(source: metadata::FrameRelation) -> Relation {
    match source {
        metadata::FrameRelation::Subject => Relation::Subject,
        metadata::FrameRelation::Object => Relation::Object,
        metadata::FrameRelation::Complement => Relation::Complement,
    }
}

pub(super) fn frame_item(item: &metadata::FrameItem) -> FrameItem {
    match item {
        metadata::FrameItem::Argument(value) => argument(relation(value.relation), &value.category),
        metadata::FrameItem::Fixed(value) => marker(&value.vocabulary, &value.member),
        metadata::FrameItem::Marked(mark, value) => FrameItem::Marked {
            vocabulary: mark.vocabulary.clone(),
            member: mark.member.clone(),
            slot: FrameSlot {
                relation: relation(value.relation),
                category: value.category.clone(),
            },
        },
        metadata::FrameItem::Optional(value) => FrameItem::Optional(Box::new(frame_item(value))),
        metadata::FrameItem::Literal(value) => FrameItem::Literal(value.clone()),
        metadata::FrameItem::Lex(vocabulary, member) => marker(vocabulary, member),
        metadata::FrameItem::OptionalLex(vocabulary, member) => {
            FrameItem::Optional(Box::new(marker(vocabulary, member)))
        }
        metadata::FrameItem::MarkedRole(vocabulary, member, category) => FrameItem::Marked {
            vocabulary: vocabulary.clone(),
            member: member.clone(),
            slot: FrameSlot {
                relation: Relation::Complement,
                category: category.clone(),
            },
        },
        metadata::FrameItem::OptionalMarkedRole(vocabulary, member, category) => {
            FrameItem::Optional(Box::new(FrameItem::Marked {
                vocabulary: vocabulary.clone(),
                member: member.clone(),
                slot: FrameSlot {
                    relation: Relation::Complement,
                    category: category.clone(),
                },
            }))
        }
        metadata::FrameItem::Amount => argument(Relation::Complement, "Amount"),
        metadata::FrameItem::ObjectNounPhrase => argument(Relation::Object, "NounPhrase"),
        metadata::FrameItem::PredicativeComplement => {
            argument(Relation::Complement, "PredicativeComplement")
        }
        metadata::FrameItem::Role(category) => argument(Relation::Complement, category),
        metadata::FrameItem::OptionalRole(category) => {
            FrameItem::Optional(Box::new(argument(Relation::Complement, category)))
        }
    }
}

fn declaration_kind(kind: metadata::DeclarationKind) -> &'static str {
    match kind {
        metadata::DeclarationKind::KeywordAction => "keyword_action",
        metadata::DeclarationKind::KeywordAbility => "keyword_ability",
        metadata::DeclarationKind::AbilityWord => "ability_word",
        metadata::DeclarationKind::Subtype(category) => match category {
            metadata::SubtypeCategory::Artifact => "artifact_subtype",
            metadata::SubtypeCategory::Battle => "battle_subtype",
            metadata::SubtypeCategory::Creature => "creature_subtype",
            metadata::SubtypeCategory::Enchantment => "enchantment_subtype",
            metadata::SubtypeCategory::Land => "land_subtype",
            metadata::SubtypeCategory::Planeswalker => "planeswalker_subtype",
            metadata::SubtypeCategory::Spell => "spell_subtype",
        },
        metadata::DeclarationKind::Type => "type",
        metadata::DeclarationKind::TurnPart => "turn_part",
        metadata::DeclarationKind::CounterKind => "counter_kind",
        metadata::DeclarationKind::Designation => "designation",
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::Lexicon;

    use super::*;

    #[test]
    fn legacy_nonattestation_does_not_withhold_plugin_verb_forms() {
        let text = r#"KeywordAction(name: "Attack", spelling: "attack", grammar: Verb(bare: "attack", third_person: Unavailable, participle: Unavailable, frame_set: Intransitive))"#;
        let normalized = metadata::read_str("Attack.ron", text).unwrap();
        let reader = metadata::declaration_macro_set().unwrap();
        let definition: macro_ron::MacroDef<metadata::Metadata> = reader.read_str(text).unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        export_metadata(
            "lexeme:keyword_action/Attack",
            "Attack.ron",
            definition.metadata,
            &normalized,
            &mut output,
        );
        assert_eq!(
            output
                .unmapped
                .iter()
                .filter(|message| message.starts_with("retired-nonattestation "))
                .count(),
            2
        );
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        for surface in ["attack", "attacks", "attacked", "attacking"] {
            assert!(
                !lexicon.analyze(surface).matches.is_empty(),
                "missing {surface}"
            );
        }
    }

    #[test]
    fn overrides_replace_defaults_and_unavailable_slots_are_removed() {
        let mut slot = FormDeclaration {
            form: WordForm::Plural,
            features: FeatureBundle::default(),
            surfaces: None,
        };
        assert!(derived(
            &mut slot,
            &metadata::DerivedSurface::Override("Elves".into())
        ));
        assert_eq!(slot.surfaces, Some(vec!["Elves".to_owned()]));
        assert!(!derived(&mut slot, &metadata::DerivedSurface::Unavailable));
        assert!(derived(&mut slot, &metadata::DerivedSurface::Derived));
        assert_eq!(slot.surfaces, None);
    }

    #[test]
    fn optional_marked_complement_keeps_marker_and_argument_together() {
        let mapped = frame_item(&metadata::FrameItem::OptionalMarkedRole(
            "Preposition".into(),
            "To".into(),
            "Destination".into(),
        ));
        assert_eq!(
            mapped,
            FrameItem::Optional(Box::new(FrameItem::Marked {
                vocabulary: "Preposition".into(),
                member: "To".into(),
                slot: FrameSlot {
                    relation: Relation::Complement,
                    category: "Destination".into(),
                },
            }))
        );
    }
    #[test]
    fn declared_adjective_derivation_uses_the_shared_default_or_replacing_override() {
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        output.lexemes.push(participial_adjective_lexeme(
            "default",
            "fixture",
            "fortify",
            metadata::ParticipialAdjectiveGrammar::default(),
        ));
        output.lexemes.push(participial_adjective_lexeme(
            "override",
            "fixture",
            "equip",
            metadata::ParticipialAdjectiveGrammar {
                bare_singular_use: false,
                surface: metadata::DerivedSurface::Override("equipped".into()),
                onset: None,
            },
        ));
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        for (owner, spelling) in [
            ("default/adjective", "fortified"),
            ("override/adjective", "equipped"),
        ] {
            let value = deckmaste_lexical::LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: deckmaste_lexical::SurfaceCase::Declared,
            };
            assert_eq!(
                lexicon
                    .realize(&deckmaste_lexical::LexicalReading::Word(value))
                    .unwrap(),
                spelling
            );
        }
        assert_eq!(
            lexicon.analyze("equiped").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
    }
}
