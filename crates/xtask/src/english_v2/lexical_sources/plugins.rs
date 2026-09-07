use std::fs;
use std::path::Path;

use anyhow::Context;
use deckmaste_construction_core::macro_def as metadata;
use deckmaste_lexical::Binding;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::WordForm;

use super::LexicalSources;
use super::source;

pub(super) fn load(root: &Path, output: &mut LexicalSources) -> anyhow::Result<()> {
    let declarations = metadata::read_builtin_v2(root.join("plugins_v2/builtin"))?;
    let reader = metadata::declaration_macro_set().map_err(anyhow::Error::msg)?;
    for normalized in declarations {
        let path = normalized.provenance().path();
        let text = fs::read_to_string(path)?;
        let definition: macro_ron::MacroDef<metadata::Metadata> =
            reader.read_str(&text).with_context(|| {
                format!("reading authored lexical metadata from {}", path.display())
            })?;
        let owner = format!(
            "lexeme:{}/{}",
            declaration_kind(normalized.identity().kind()),
            normalized.identity().name()
        );
        let path = path.strip_prefix(root).unwrap_or(path).to_string_lossy();
        export_metadata(&owner, &path, definition.metadata, &normalized, output);
    }
    Ok(())
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
    let mut lexeme = match grammar {
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
            export_keyword_supplements(
                owner,
                path,
                bound_suffix,
                participial_adjective,
                block_label,
                output,
            );
            let mut lexeme = Lexeme::invariant(owner, surface, Category::Keyword, provenance);
            lexeme.source.kind = SourceKind::Keyword;
            if let Some(onset) = onset {
                lexeme
                    .properties
                    .features
                    .insert("Onset".into(), format!("{onset:?}"));
            }
            if let Some(parameter) = parameter {
                lexeme
                    .properties
                    .features
                    .insert("KeywordParameter".into(), format!("{parameter:?}"));
            }
            lexeme
        }
    };
    if let Some(grammar) = normalized.grammar() {
        lexeme
            .properties
            .features
            .insert("source_recipe".into(), format!("{:?}", grammar.recipe()));
    }
    if let Some(class) = authored.noun_class {
        lexeme.properties.features.insert(
            "locative_temporal_license".into(),
            format!("{:?}", class.locative_temporal_license),
        );
        lexeme
            .properties
            .features
            .insert("relationality".into(), format!("{:?}", class.relationality));
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

fn export_keyword_supplements(
    owner: &str,
    path: &str,
    bound_suffix: Option<metadata::BoundSuffixGrammar>,
    participial_adjective: Option<metadata::ParticipialAdjectiveGrammar>,
    block_label: Option<metadata::BlockLabelGrammar>,
    output: &mut LexicalSources,
) {
    if let Some(suffix) = bound_suffix {
        let suffix_owner = format!("{owner}/bound-suffix");
        let mut lexeme = Lexeme::invariant(
            &suffix_owner,
            suffix.surface,
            Category::Keyword,
            source(SourceKind::Keyword, path, owner),
        );
        lexeme.binding = Binding::Suffix;
        lexeme.capitalization = Capitalization::Exact;
        output.lexemes.push(lexeme);
    }
    if let Some(adjective) = participial_adjective {
        output.unmapped.push(format!(
            "supplemental-adjective {owner}: {:?}; requires shared lexical derivation adapter",
            adjective.surface
        ));
    }
    if let Some(label) = block_label {
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
        output.lexemes.push(lexeme);
    }
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
    FrameItem::Argument {
        relation,
        category: category.to_owned(),
    }
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
        metadata::FrameItem::Marked(mark, value) => FrameItem::Sequence(vec![
            marker(&mark.vocabulary, &mark.member),
            argument(relation(value.relation), &value.category),
        ]),
        metadata::FrameItem::Optional(value) => FrameItem::Optional(Box::new(frame_item(value))),
        metadata::FrameItem::Literal(value) => FrameItem::Literal(value.clone()),
        metadata::FrameItem::Lex(vocabulary, member) => marker(vocabulary, member),
        metadata::FrameItem::OptionalLex(vocabulary, member) => {
            FrameItem::Optional(Box::new(marker(vocabulary, member)))
        }
        metadata::FrameItem::MarkedRole(vocabulary, member, category) => FrameItem::Sequence(vec![
            marker(vocabulary, member),
            argument(Relation::Complement, category),
        ]),
        metadata::FrameItem::OptionalMarkedRole(vocabulary, member, category) => {
            FrameItem::Optional(Box::new(FrameItem::Sequence(vec![
                marker(vocabulary, member),
                argument(Relation::Complement, category),
            ])))
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
        metadata::DeclarationKind::FlavorWord => "flavor_word",
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
            FrameItem::Optional(Box::new(FrameItem::Sequence(vec![
                FrameItem::Marker {
                    vocabulary: "Preposition".into(),
                    member: "To".into()
                },
                FrameItem::Argument {
                    relation: Relation::Complement,
                    category: "Destination".into()
                },
            ])))
        );
    }
}
