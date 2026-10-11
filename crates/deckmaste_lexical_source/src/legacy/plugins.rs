use std::path::Path;

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
use crate::LoadError;
use crate::compound::Declaration as PendingCompoundNoun;
use crate::source;

pub(crate) struct LoadedDeclarations {
    pub compounds: Vec<PendingCompoundNoun>,
    pub bound_keyword_stems: Vec<String>,
}

pub(crate) fn load(
    root: &Path,
    output: &mut LexicalSources,
) -> Result<LoadedDeclarations, LoadError> {
    let mut compounds = Vec::new();
    let mut stem_classes = Vec::new();
    let declarations = metadata::read_builtin_v2(root.join("plugins_v2/builtin"))?;
    let reader = metadata::declaration_macro_set()?;
    for normalized in &declarations {
        let path = normalized.provenance().path();
        let text = crate::error::read(path)?;
        let definition: macro_ron::MacroDef<metadata::Metadata> =
            reader.read_str(&text).map_err(|source| LoadError::Decode {
                path: path.into(),
                source: Box::new(source),
            })?;
        let owner = declaration_owner(normalized);
        if let Some(metadata::Grammar::FixedKeyword {
            bound_suffix: Some(suffix),
            ..
        }) = &definition.metadata.grammar
        {
            stem_classes.push(suffix.stem_class.clone());
        }
        let path = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
        if let Some(recipe) = normalized.compound_noun() {
            compounds.push(PendingCompoundNoun {
                source: source(SourceKind::Plugin, &path, &owner),
                recipe: recipe.clone(),
                noun_class: normalized.noun_class(),
            });
        }
        export_metadata(&owner, &path, definition.metadata, normalized, output);
    }
    for declaration in metadata::read_builtin_v2_helpers(root.join("plugins_v2/builtin"))? {
        export_helper(root, declaration, output);
    }
    Ok(LoadedDeclarations {
        compounds,
        bound_keyword_stems: bound_keyword_stem_members(&declarations, &stem_classes),
    })
}

fn export_helper(
    root: &Path,
    declaration: metadata::HelperLexicalDeclaration,
    output: &mut LexicalSources,
) {
    let owner = format!("lexeme:{}/{}", declaration.kind, declaration.name);
    let path = declaration
        .path
        .strip_prefix(root)
        .unwrap_or(&declaration.path)
        .to_string_lossy();
    if let Some(grammar) = declaration.grammar.term {
        let provenance = source(SourceKind::Plugin, &path, &owner);
        let mut lexeme = grammar_lexeme(&owner, &path, grammar, provenance, None, output);
        if let Some(term) = &declaration.term {
            export_grammar_distribution(&mut lexeme, term);
        }
        if declaration.grammar.noun_modifier {
            lexeme
                .properties
                .features
                .insert("NounPremodifier".into(), "Yes".into());
        }
        if let Some(join) = declaration.grammar.negative_prefix_join {
            output
                .lexemes
                .push(crate::native::negative_lexeme(&lexeme, join));
        }
        output.lexemes.push(lexeme);
    }
    if let Some(verb) = declaration.grammar.verb {
        output
            .lexemes
            .push(keyword_verb_lexeme(&owner, &path, verb));
    }
}

fn declaration_owner(declaration: &metadata::NormalizedDeclaration) -> String {
    format!(
        "lexeme:{}/{}",
        declaration_kind(declaration.identity().kind()),
        declaration.identity().name()
    )
}

pub(crate) fn bound_keyword_stem_members(
    declarations: &[metadata::NormalizedDeclaration],
    classes: &[metadata::BoundKeywordStemClass],
) -> Vec<String> {
    let mut members = std::collections::BTreeSet::new();
    for class in classes {
        let (categories, card_type) = match class {
            metadata::BoundKeywordStemClass::SubtypesAndType {
                category,
                card_type,
            } => (std::slice::from_ref(category), card_type),
            metadata::BoundKeywordStemClass::SubtypeClassesAndType {
                categories,
                card_type,
            } => (categories.as_slice(), card_type),
        };
        for declaration in declarations {
            let identity = declaration.identity();
            if categories
                .iter()
                .any(|category| identity.kind() == metadata::DeclarationKind::Subtype(*category))
                || (identity.kind() == metadata::DeclarationKind::Type
                    && identity.name() == card_type)
            {
                members.insert(declaration_owner(declaration));
            }
        }
    }
    members.into_iter().collect()
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
        output.unmapped.push(match authored.spelling {
            Some(spelling) => {
                format!("plugin {owner}: no declared lexical grammar; spelling {spelling:?}")
            }
            None => format!("plugin {owner}: no declared lexical grammar; no spelling"),
        });
        return;
    };
    let mut lexeme = grammar_lexeme(owner, path, grammar, provenance, Some(normalized), output);
    export_distribution(&mut lexeme, normalized, authored.noun_class);
    lexeme.properties.features.insert(
        "CreatureSubtypeStem".into(),
        if normalized.identity().kind()
            == metadata::DeclarationKind::Subtype(metadata::SubtypeCategory::Creature)
        {
            "Yes"
        } else {
            "No"
        }
        .into(),
    );
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
    if matches!(
        normalized.identity().kind(),
        metadata::DeclarationKind::Subtype(_)
    ) {
        lexeme
            .properties
            .features
            .insert("TypeLineRole".into(), "Subtype".into());
    }
    // A keyword spelling is a card-usage template (`scry <Param(0)>`): its
    // holes and literals are a frame for the semantic spelling match, not
    // lexical data. Every word the template prints is owned by `grammar` or
    // by its own vocabulary declaration, so the export reads the grammar
    // alone and records nothing for the template.
    output.lexemes.push(lexeme);
}

#[expect(
    clippy::too_many_lines,
    reason = "lexical export dispatch covers every closed declaration grammar recipe"
)]
fn grammar_lexeme(
    owner: &str,
    path: &str,
    grammar: metadata::Grammar,
    provenance: Source,
    normalized: Option<&metadata::NormalizedDeclaration>,
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
        metadata::Grammar::Adjective { surface, onset } => {
            let mut lexeme = Lexeme::invariant(owner, surface, Category::Adjective, provenance);
            if let Some(onset) = onset {
                lexeme
                    .properties
                    .features
                    .insert("Onset".into(), format!("{onset:?}"));
            }
            lexeme
        }
        metadata::Grammar::FixedClause { noun, verb, .. } => {
            output.lexemes.push(keyword_verb_lexeme(owner, path, verb));
            Lexeme::noun(
                format!("{owner}/noun"),
                noun,
                vec![Countability::Count],
                provenance,
            )
        }
        metadata::Grammar::FixedTerm { surface, onset } => {
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
            verb,
        } => {
            if let Some(verb) = verb {
                output.lexemes.push(keyword_verb_lexeme(owner, path, verb));
            }
            if let Some(suffix) = bound_suffix {
                return bound_suffix_lexeme(
                    owner,
                    path,
                    suffix,
                    normalized.and_then(metadata::NormalizedDeclaration::keyword_parameter_class),
                );
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
            if let Some(metadata::FixedKeywordParameterGrammar::Subject { indefinite_article }) =
                &parameter
            {
                lexeme.properties.features.insert(
                    "KeywordIndefiniteArticle".into(),
                    if *indefinite_article { "Yes" } else { "No" }.into(),
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
        export_grammar_distribution(lexeme, grammar);
    }
    if let Some(parameter) = normalized.keyword_parameter_class() {
        lexeme
            .properties
            .features
            .insert("NounPremodifier".into(), "Keyword".into());
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

fn export_grammar_distribution(lexeme: &mut Lexeme, grammar: &metadata::GrammarRow) {
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

fn bound_suffix_lexeme(
    owner: &str,
    path: &str,
    suffix: metadata::BoundSuffixGrammar,
    parameter_class: Option<metadata::KeywordParameterClass>,
) -> Lexeme {
    let creature_stem_use = match &suffix.stem_class {
        metadata::BoundKeywordStemClass::SubtypesAndType { category, .. } => {
            *category == metadata::SubtypeCategory::Creature
        }
        metadata::BoundKeywordStemClass::SubtypeClassesAndType { categories, .. } => {
            categories.contains(&metadata::SubtypeCategory::Creature)
        }
    };
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
    lexeme.properties.features.insert(
        "CreatureSubtypeStemUse".into(),
        if creature_stem_use { "Yes" } else { "No" }.into(),
    );
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

fn keyword_verb_lexeme(owner: &str, path: &str, verb: metadata::KeywordVerbGrammar) -> Lexeme {
    let mut lexeme = Lexeme::verb(
        format!("{owner}/verb"),
        verb.bare,
        source(SourceKind::Plugin, path, owner),
    );
    lexeme.forms.retain_mut(|slot| {
        let declaration = match slot.form {
            WordForm::Preterite => &verb.preterite,
            WordForm::PastParticiple => &verb.participle,
            WordForm::GerundParticiple => &verb.gerund_participle,
            _ => return true,
        };
        derived(slot, declaration)
    });
    lexeme.properties.frames = declared_frames(&verb.frames);
    lexeme
}

pub(super) fn declared_frames(source: &[metadata::VerbDeclarationFrame]) -> Vec<Frame> {
    source
        .iter()
        .map(|frame| {
            let (kind, items) = match frame {
                metadata::VerbDeclarationFrame::Predicate(items) => {
                    ("Predicate", items.iter().map(frame_item).collect())
                }
                metadata::VerbDeclarationFrame::Auxiliary => ("Auxiliary", Vec::new()),
                metadata::VerbDeclarationFrame::ProVerb => ("ProVerb", Vec::new()),
            };
            Frame {
                kind: kind.into(),
                items,
            }
        })
        .collect()
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
    fn subject_article_is_exported_from_the_parameter_recipe() {
        for (indefinite_article, expected) in [(false, "No"), (true, "Yes")] {
            let mut output = LexicalSources {
                lexemes: vec![],
                unmapped: vec![],
            };
            let lexeme = grammar_lexeme(
                "lexeme:keyword_ability/quorbling",
                "quorbling.ron",
                metadata::Grammar::FixedKeyword {
                    surface: "quorbling".into(),
                    onset: None,
                    bound_suffix: None,
                    participial_adjective: None,
                    block_label: None,
                    verb: None,
                    parameter: Some(metadata::FixedKeywordParameterGrammar::Subject {
                        indefinite_article,
                    }),
                },
                source(
                    SourceKind::Plugin,
                    "quorbling.ron",
                    "lexeme:keyword_ability/quorbling",
                ),
                None,
                &mut output,
            );
            assert_eq!(
                lexeme.properties.features["KeywordIndefiniteArticle"],
                expected
            );
        }
    }

    #[test]
    fn bound_stem_class_selects_declaration_category_and_card_type() {
        let declarations: Vec<_> = [
            r#"Subtype(category: Land, name: "cave", spelling: "Cave")"#,
            r#"Subtype(category: Creature, name: "forest", spelling: "Forest")"#,
            r#"Type(name: "land", spelling: "land")"#,
            r#"Type(name: "artifact", spelling: "artifact")"#,
        ]
        .into_iter()
        .map(|text| metadata::read_str("class-member.ron", text).unwrap())
        .collect();
        let class = metadata::BoundKeywordStemClass::SubtypesAndType {
            category: metadata::SubtypeCategory::Land,
            card_type: "land".into(),
        };
        assert_eq!(
            bound_keyword_stem_members(&declarations, &[class.clone(), class]),
            vec!["lexeme:land_subtype/cave", "lexeme:type/land"],
        );
    }

    #[test]
    fn multiple_subtype_classes_share_stems_without_admitting_other_types() {
        let declarations: Vec<_> = [
            r#"Subtype(category: Land, name: "cave", spelling: "Cave")"#,
            r#"Subtype(category: Creature, name: "wizard", spelling: "Wizard")"#,
            r#"Subtype(category: Artifact, name: "equipment", spelling: "Equipment")"#,
            r#"Type(name: "land", spelling: "land")"#,
            r#"Type(name: "artifact", spelling: "artifact")"#,
        ]
        .into_iter()
        .map(|text| metadata::read_str("class-member.ron", text).unwrap())
        .collect();
        let classes = [
            metadata::BoundKeywordStemClass::SubtypesAndType {
                category: metadata::SubtypeCategory::Land,
                card_type: "land".into(),
            },
            metadata::BoundKeywordStemClass::SubtypeClassesAndType {
                categories: vec![
                    metadata::SubtypeCategory::Land,
                    metadata::SubtypeCategory::Creature,
                ],
                card_type: "land".into(),
            },
        ];
        assert_eq!(
            bound_keyword_stem_members(&declarations, &classes),
            vec![
                "lexeme:creature_subtype/wizard",
                "lexeme:land_subtype/cave",
                "lexeme:type/land",
            ]
        );
    }

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
