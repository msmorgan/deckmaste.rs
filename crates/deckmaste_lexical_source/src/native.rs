use std::collections::BTreeMap;
use std::path::Path;

use crate::LoadError;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::Lexeme;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Inventory {
    pub lexemes: Vec<Lexeme>,
    pub core_verb_paradigms: BTreeMap<String, Paradigm>,
    pub frame_markers: BTreeMap<String, (String, String)>,
    pub frame_additions: BTreeMap<String, Vec<Frame>>,
    pub form_replacements: BTreeMap<String, Vec<FormDeclaration>>,
    #[serde(default)]
    pub feature_additions: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Paradigm {
    pub forms: Vec<FormDeclaration>,
    pub frames: Vec<Frame>,
    #[serde(default)]
    pub features: BTreeMap<String, String>,
}

pub(crate) const PATH: &str = "crates/deckmaste_lexical_source/lexicon/core.ron";

pub(crate) fn load(root: &Path) -> Result<Inventory, LoadError> {
    let path = root.join(PATH);
    let text = crate::error::read(&path)?;
    crate::error::decode(&path, &text)
}

pub(crate) fn reconcile_frames(
    lexemes: &mut [Lexeme],
    markers: &BTreeMap<String, (String, String)>,
) -> Result<(), LoadError> {
    let identities: BTreeMap<_, _> = lexemes
        .iter()
        .map(|lexeme| (lexeme.id.clone(), lexeme.lemma.clone()))
        .collect();
    for (surface, (vocabulary, member)) in markers {
        let lemma = marker_lemma(&identities, vocabulary, member)?;
        if lemma != surface {
            return Err(LoadError::MarkerSpelling {
                vocabulary: vocabulary.clone(),
                member: member.clone(),
                surface: surface.clone(),
                lemma: lemma.into(),
            });
        }
    }
    for lexeme in lexemes {
        for frame in &mut lexeme.properties.frames {
            if frame.kind.is_empty() {
                return Err(LoadError::EmptyFrameKind {
                    owner: lexeme.id.clone(),
                });
            }
            for item in &mut frame.items {
                reconcile_item(item, markers, &identities).map_err(|source| LoadError::Frame {
                    owner: lexeme.id.clone(),
                    source: Box::new(source),
                })?;
            }
        }
    }
    Ok(())
}

pub(crate) fn add_frames(
    lexemes: &mut [Lexeme],
    additions: BTreeMap<String, Vec<Frame>>,
) -> Result<(), LoadError> {
    for (owner, frames) in additions {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .ok_or_else(|| LoadError::UnknownOwner {
                property: "frame",
                owner: owner.clone(),
            })?;
        for frame in frames {
            if lexeme.properties.frames.contains(&frame) {
                return Err(LoadError::DuplicateFrame { owner });
            }
            lexeme.properties.frames.push(frame);
        }
        lexeme
            .properties
            .features
            .insert("FrameSource".into(), PATH.into());
    }
    Ok(())
}

pub(crate) fn replace_forms(
    lexemes: &mut [Lexeme],
    replacements: BTreeMap<String, Vec<FormDeclaration>>,
) -> Result<(), LoadError> {
    for (owner, forms) in replacements {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .ok_or_else(|| LoadError::UnknownOwner {
                property: "form",
                owner: owner.clone(),
            })?;
        lexeme.forms = forms;
        lexeme
            .properties
            .features
            .insert("FormSource".into(), PATH.into());
    }
    Ok(())
}

pub(crate) fn add_features(
    lexemes: &mut [Lexeme],
    additions: BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), LoadError> {
    for (owner, features) in additions {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .ok_or_else(|| LoadError::UnknownOwner {
                property: "feature",
                owner: owner.clone(),
            })?;
        for (name, value) in features {
            if name.is_empty() || value.is_empty() {
                return Err(LoadError::EmptyFeature { owner, name, value });
            }
            if lexeme.properties.features.contains_key(&name) {
                return Err(LoadError::DuplicateFeature { owner, name });
            }
            lexeme
                .properties
                .features
                .insert(format!("FeatureSource:{name}"), PATH.into());
            lexeme.properties.features.insert(name, value);
        }
    }
    Ok(())
}

fn marker_lemma<'a>(
    identities: &'a BTreeMap<String, String>,
    vocabulary: &str,
    member: &str,
) -> Result<&'a str, LoadError> {
    let candidates = [
        format!("vocab:{vocabulary}/{member}"),
        format!("lexeme:{vocabulary}/{member}"),
    ];
    let found: Vec<_> = candidates
        .iter()
        .filter_map(|id| identities.get(id))
        .collect();
    if found.len() != 1 {
        return Err(LoadError::MarkerIdentity {
            vocabulary: vocabulary.into(),
            member: member.into(),
            found: found.len(),
        });
    }
    Ok(found[0])
}

fn reconcile_item(
    item: &mut FrameItem,
    markers: &BTreeMap<String, (String, String)>,
    identities: &BTreeMap<String, String>,
) -> Result<(), LoadError> {
    if let FrameItem::Literal(surface) = item {
        let (vocabulary, member) =
            markers
                .get(surface)
                .ok_or_else(|| LoadError::UnresolvedLiteral {
                    surface: surface.clone(),
                })?;
        *item = FrameItem::Marker {
            vocabulary: vocabulary.clone(),
            member: member.clone(),
        };
    }
    match item {
        FrameItem::Marker { vocabulary, member } => {
            marker_lemma(identities, vocabulary, member)?;
        }
        FrameItem::Marked {
            vocabulary,
            member,
            slot,
        } => {
            marker_lemma(identities, vocabulary, member)?;
            if slot.category.is_empty() {
                return Err(LoadError::EmptySlotCategory);
            }
        }
        FrameItem::Argument(slot) => {
            if slot.category.is_empty() {
                return Err(LoadError::EmptySlotCategory);
            }
        }
        FrameItem::Optional(item) => reconcile_item(item, markers, identities)?,
        FrameItem::Literal(_) => unreachable!("literal reconciled above"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use deckmaste_lexical::Category;
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::Finiteness;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::LexicalValue;
    use deckmaste_lexical::Lexicon;
    use deckmaste_lexical::Number;
    use deckmaste_lexical::Person;
    use deckmaste_lexical::SurfaceCase;
    use deckmaste_lexical::Tense;
    use deckmaste_lexical::WordForm;

    use super::*;

    fn workspace() -> Lexicon {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        Lexicon::new(crate::load_workspace(&root).unwrap().lexemes).unwrap()
    }

    #[test]
    fn lexical_measure_gaps_have_declared_categories_forms_and_countability() {
        use deckmaste_lexical::Countability::Count;
        use deckmaste_lexical::Countability::Mass;

        let lexicon = workspace();
        for (surface, id, category, form) in [
            (
                "defending",
                "lexeme:Verb/Defend",
                Category::Verb,
                WordForm::GerundParticiple,
            ),
            (
                "named",
                "lexeme:Verb/Name",
                Category::Verb,
                WordForm::PastParticiple,
            ),
            (
                "spent",
                "lexeme:Verb/Spend",
                Category::Verb,
                WordForm::PastParticiple,
            ),
            (
                "addition",
                "lexeme:CommonNoun/Addition",
                Category::Noun,
                WordForm::Singular,
            ),
            (
                "game",
                "lexeme:CommonNoun/Game",
                Category::Noun,
                WordForm::Singular,
            ),
            (
                "amount",
                "lexeme:CommonNoun/Amount",
                Category::Noun,
                WordForm::Singular,
            ),
        ] {
            let input = lexicon.analyze(surface);
            let values: Vec<_> = input
                .matches
                .iter()
                .filter_map(|found| {
                    if found.start != 0 || found.end != input.tokens.len() {
                        return None;
                    }
                    let LexicalReading::Word(value) = &found.reading else { panic!("{found:?}") };
                    assert_eq!(value.lexeme, id, "unexpected POS or Lexeme for {surface}");
                    assert_eq!(lexicon.lexemes()[id].category, category);
                    Some(value)
                })
                .collect();
            assert!(values.iter().any(|value| value.form == form), "{surface}");
        }
        for surface in [
            "spended",
            "spents",
            "defendinged",
            "nameds",
            "additioned",
            "amounting",
            "gamesed",
        ] {
            let input = lexicon.analyze(surface);
            assert!(
                !input
                    .matches
                    .iter()
                    .any(|found| found.start == 0 && found.end == input.tokens.len()),
                "{surface}"
            );
        }
        for (id, uses) in [
            ("Addition", vec![Count, Mass]),
            ("Game", vec![Count]),
            ("Amount", vec![Count]),
        ] {
            assert_eq!(
                lexicon.lexemes()[&format!("lexeme:CommonNoun/{id}")]
                    .properties
                    .countability,
                uses
            );
        }
        let name = &lexicon.lexemes()["lexeme:Verb/Name"];
        assert_eq!(
            name.properties.frames[1].items,
            vec![
                FrameItem::Argument(deckmaste_lexical::FrameSlot {
                    relation: deckmaste_lexical::Relation::Object,
                    category: "NounPhrase".into()
                }),
                FrameItem::Argument(deckmaste_lexical::FrameSlot {
                    relation: deckmaste_lexical::Relation::Complement,
                    category: "Name".into()
                }),
            ]
        );
        let amount = &lexicon.lexemes()["lexeme:CommonNoun/Amount"];
        assert_eq!(
            amount.properties.frames,
            vec![
                Frame {
                    kind: "Nominal".into(),
                    items: vec![]
                },
                Frame {
                    kind: "Nominal".into(),
                    items: vec![FrameItem::Marked {
                        vocabulary: "Preposition".into(),
                        member: "Of".into(),
                        slot: deckmaste_lexical::FrameSlot {
                            relation: deckmaste_lexical::Relation::Complement,
                            category: "CostSymbols".into(),
                        },
                    },]
                },
            ]
        );
        assert_eq!(
            lexicon.lexemes()["catalog:card-names.txt/Powerstone Shard"]
                .properties
                .features["IdentityUse"],
            "Name"
        );
        assert!(
            !lexicon.lexemes()["catalog:card-types.txt/Artifact"]
                .properties
                .features
                .contains_key("IdentityUse")
        );
    }

    #[test]
    fn every_declared_value_and_frame_survives_realization_and_reanalysis() {
        let lexicon = workspace();
        let mut adjectives = 0;
        let mut frames = 0;
        for lexeme in lexicon.lexemes().values() {
            let decoded: Lexeme = ron::from_str(&ron::to_string(lexeme).unwrap()).unwrap();
            assert_eq!(&decoded, lexeme);
            frames += lexeme.properties.frames.len();
            adjectives += usize::from(lexeme.category == Category::Adjective);
        }
        assert!(adjectives > 0 && frames > 0);
        for value in lexicon.values() {
            let reading = LexicalReading::Word(value.clone());
            let surface = lexicon.realize(&reading).unwrap();
            let analyzed = lexicon.analyze(&surface);
            assert!(
                analyzed.matches.iter().any(|found| found.start == 0
                    && found.end == analyzed.tokens.len()
                    && found.reading == reading),
                "{reading:?}: {surface}"
            );
        }
    }

    #[test]
    fn copula_and_modal_values_keep_the_declared_agreement_and_tense() {
        let lexicon = workspace();
        let value = LexicalReading::Word(LexicalValue {
            lexeme: "core-verb:Be".into(),
            form: WordForm::Present,
            features: FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::First),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                case: None,
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        });
        assert_eq!(lexicon.realize(&value).unwrap(), "am");
        let analyzed = lexicon.analyze("am");
        assert!(analyzed.matches.iter().any(|found| found.reading == value));
        for found in &lexicon.analyze("is").matches {
            if let LexicalReading::Word(value) = &found.reading {
                assert_eq!(value.features.person, Some(Person::Third));
                assert_eq!(value.features.number, Some(Number::Singular));
            }
        }
        for surface in [
            "canning", "canned", "maying", "musts", "woulded", "can'ting",
        ] {
            assert!(!lexicon.analyze(surface).matches.iter().any(|found| found.start == 0 && found.end == surface.chars().count() && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme.starts_with("core-verb:"))), "{surface}");
        }
        let can = &lexicon.lexemes()["core-verb:Can"];
        assert_eq!(
            can.properties.frames,
            vec![Frame {
                kind: "Auxiliary".into(),
                items: vec![FrameItem::Argument(deckmaste_lexical::FrameSlot {
                    relation: deckmaste_lexical::Relation::Complement,
                    category: "BarePredicate".into()
                })]
            }]
        );
        assert_eq!(can.forms.len(), 12);
    }

    #[test]
    fn contractions_genitives_and_homographs_remain_independent_alternatives() {
        let lexicon = workspace();
        for text in [
            "it's", "it’s", "you've", "they're", "owner's", "owners'", "nonland",
        ] {
            assert!(lexicon.analyze(text).unknown_words().is_empty(), "{text}");
        }
        let text = lexicon.analyze("it's");
        let categories: std::collections::BTreeSet<_> = text
            .matches
            .iter()
            .filter_map(|found| {
                if found.start != 2 || found.end != 4 {
                    return None;
                }
                let LexicalReading::Word(value) = &found.reading else {
                    return None;
                };
                Some(lexicon.lexemes()[&value.lexeme].category)
            })
            .collect();
        assert_eq!(categories, [Category::Verb, Category::Clitic].into());
        let one = lexicon.analyze("one");
        assert!(
            one.matches
                .iter()
                .any(|found| matches!(found.reading, LexicalReading::Numeral { value: 1, .. }))
        );
        for category in [Category::Noun, Category::Determinative] {
            assert!(one.matches.iter().any(|found| matches!(&found.reading, LexicalReading::Word(value) if lexicon.lexemes()[&value.lexeme].category == category)));
        }
        for surface in ["enchanted", "equipped", "fortified", "kicked"] {
            let found = lexicon.analyze(surface);
            assert!(found.matches.iter().any(|found| matches!(&found.reading, LexicalReading::Word(value) if lexicon.lexemes()[&value.lexeme].category == Category::Adjective)), "{surface}");
        }
    }

    #[test]
    fn frame_reconciliation_rejects_missing_or_wrong_marker_declarations() {
        let mut lexemes = vec![Lexeme::verb(
            "verb",
            "act",
            crate::source(deckmaste_lexical::SourceKind::Core, "test", "verb"),
        )];
        lexemes[0].properties.frames = vec![Frame {
            kind: "Predicate".into(),
            items: vec![FrameItem::Literal("to".into())],
        }];
        let error = reconcile_frames(&mut lexemes, &BTreeMap::new()).unwrap_err();
        match &error {
            LoadError::Frame { owner, source } => {
                assert_eq!(owner, "verb");
                match source.as_ref() {
                    LoadError::UnresolvedLiteral { surface } => assert_eq!(surface, "to"),
                    other => panic!("{other:?}"),
                }
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(
            std::error::Error::source(&error).unwrap().to_string(),
            "unresolved frame literal \"to\""
        );
        let markers = BTreeMap::from([("to".into(), ("Preposition".into(), "To".into()))]);
        match reconcile_frames(&mut lexemes, &markers).unwrap_err() {
            LoadError::MarkerIdentity {
                vocabulary,
                member,
                found,
            } => {
                assert_eq!(vocabulary, "Preposition");
                assert_eq!(member, "To");
                assert_eq!(found, 0);
            }
            other => panic!("{other:?}"),
        }
        lexemes.push(Lexeme::invariant(
            "vocab:Preposition/To",
            "from",
            Category::Preposition,
            crate::source(deckmaste_lexical::SourceKind::Core, "test", "marker"),
        ));
        match reconcile_frames(&mut lexemes, &markers).unwrap_err() {
            LoadError::MarkerSpelling {
                vocabulary,
                member,
                surface,
                lemma,
            } => {
                assert_eq!(vocabulary, "Preposition");
                assert_eq!(member, "To");
                assert_eq!(surface, "to");
                assert_eq!(lemma, "from");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn feature_additions_require_an_existing_owner_and_preserve_declared_properties() {
        let mut declarations = vec![Lexeme::invariant(
            "determiner",
            "each",
            deckmaste_lexical::Category::Determinative,
            crate::source(deckmaste_lexical::SourceKind::Core, "test", "determiner"),
        )];
        let features = BTreeMap::from([("DeterminerUse".into(), "SingularCount".into())]);
        match add_features(
            &mut declarations,
            BTreeMap::from([("missing".into(), features.clone())]),
        )
        .unwrap_err()
        {
            LoadError::UnknownOwner { property, owner } => {
                assert_eq!(property, "feature");
                assert_eq!(owner, "missing");
            }
            other => panic!("{other:?}"),
        }
        add_features(
            &mut declarations,
            BTreeMap::from([("determiner".into(), features)]),
        )
        .unwrap();
        assert_eq!(
            declarations[0].properties.features["DeterminerUse"],
            "SingularCount"
        );
        assert_eq!(
            declarations[0].properties.features["FeatureSource:DeterminerUse"],
            PATH
        );
        let before = declarations.clone();
        let replacement = BTreeMap::from([("DeterminerUse".into(), "Mass".into())]);
        match add_features(
            &mut declarations,
            BTreeMap::from([("determiner".into(), replacement)]),
        )
        .unwrap_err()
        {
            LoadError::DuplicateFeature { owner, name } => {
                assert_eq!(owner, "determiner");
                assert_eq!(name, "DeterminerUse");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(declarations, before);
    }
    #[test]
    fn personal_pronouns_reject_wrong_concord_and_multiword_verbs_inflect_the_head() {
        let lexicon = workspace();
        let mut it = LexicalValue {
            lexeme: "vocab:SubjectPronoun/It".into(),
            form: WordForm::Invariant,
            features: FeatureBundle {
                person: Some(Person::Third),
                number: Some(Number::Singular),
                case: Some(deckmaste_lexical::Case::Nominative),
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        };
        assert_eq!(
            lexicon.realize(&LexicalReading::Word(it.clone())).unwrap(),
            "it"
        );
        it.features.number = Some(Number::Plural);
        assert!(lexicon.realize(&LexicalReading::Word(it.clone())).is_err());
        it.lexeme = "vocab:SubjectPronoun/They".into();
        assert_eq!(
            lexicon.realize(&LexicalReading::Word(it.clone())).unwrap(),
            "they"
        );
        it.features.number = Some(Number::Singular);
        assert!(lexicon.realize(&LexicalReading::Word(it)).is_err());
        for surface in ["faced a villainous choice", "facing a villainous choice"] {
            let analyzed = lexicon.analyze(surface);
            assert!(analyzed.matches.iter().any(|found| found.start == 0 && found.end == analyzed.tokens.len() && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme == "lexeme:keyword_action/faceAVillainousChoice")));
        }
        for surface in ["face a villainous choiced", "face a villainous choicing"] {
            let analyzed = lexicon.analyze(surface);
            assert!(
                !analyzed
                    .matches
                    .iter()
                    .any(|found| found.start == 0 && found.end == analyzed.tokens.len())
            );
        }
    }
}
