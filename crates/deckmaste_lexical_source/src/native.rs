use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Context;
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

pub(crate) fn load(root: &Path) -> anyhow::Result<Inventory> {
    let text =
        std::fs::read_to_string(root.join(PATH)).context("reading v3 lexical declarations")?;
    ron::from_str(&text).context("decoding v3 lexical declarations")
}

pub(crate) fn reconcile_frames(
    lexemes: &mut [Lexeme],
    markers: &BTreeMap<String, (String, String)>,
) -> anyhow::Result<()> {
    let identities: BTreeMap<_, _> = lexemes
        .iter()
        .map(|lexeme| (lexeme.id.clone(), lexeme.lemma.clone()))
        .collect();
    for (surface, (vocabulary, member)) in markers {
        let lemma = marker_lemma(&identities, vocabulary, member)?;
        anyhow::ensure!(
            lemma == surface,
            "frame marker {vocabulary}/{member} does not spell declared literal {surface:?}"
        );
    }
    for lexeme in lexemes {
        for frame in &mut lexeme.properties.frames {
            anyhow::ensure!(!frame.kind.is_empty(), "{}: empty frame kind", lexeme.id);
            for item in &mut frame.items {
                reconcile_item(item, markers, &identities)
                    .with_context(|| format!("frame for {}", lexeme.id))?;
            }
        }
    }
    Ok(())
}

pub(crate) fn add_frames(
    lexemes: &mut [Lexeme],
    additions: BTreeMap<String, Vec<Frame>>,
) -> anyhow::Result<()> {
    for (owner, frames) in additions {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .with_context(|| format!("unknown frame owner {owner}"))?;
        for frame in frames {
            anyhow::ensure!(
                !lexeme.properties.frames.contains(&frame),
                "duplicate added frame for {owner}"
            );
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
) -> anyhow::Result<()> {
    for (owner, forms) in replacements {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .with_context(|| format!("unknown form owner {owner}"))?;
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
) -> anyhow::Result<()> {
    for (owner, features) in additions {
        let lexeme = lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == owner)
            .with_context(|| format!("unknown feature owner {owner}"))?;
        for (name, value) in features {
            anyhow::ensure!(
                !name.is_empty() && !value.is_empty(),
                "empty feature declaration for {owner}"
            );
            anyhow::ensure!(
                !lexeme.properties.features.contains_key(&name),
                "duplicate feature {name} for {owner}"
            );
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
) -> anyhow::Result<&'a str> {
    let candidates = [
        format!("vocab:{vocabulary}/{member}"),
        format!("lexeme:{vocabulary}/{member}"),
    ];
    let found: Vec<_> = candidates
        .iter()
        .filter_map(|id| identities.get(id))
        .collect();
    anyhow::ensure!(
        found.len() == 1,
        "frame marker {vocabulary}/{member} must resolve to one declared identity"
    );
    Ok(found[0])
}

fn reconcile_item(
    item: &mut FrameItem,
    markers: &BTreeMap<String, (String, String)>,
    identities: &BTreeMap<String, String>,
) -> anyhow::Result<()> {
    if let FrameItem::Literal(surface) = item {
        let (vocabulary, member) = markers
            .get(surface)
            .with_context(|| format!("unresolved frame literal {surface:?}"))?;
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
            anyhow::ensure!(!slot.category.is_empty(), "empty frame slot category");
        }
        FrameItem::Argument(slot) => {
            anyhow::ensure!(!slot.category.is_empty(), "empty frame slot category");
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
        assert!(
            reconcile_frames(&mut lexemes, &BTreeMap::new())
                .unwrap_err()
                .chain()
                .any(|error| error.to_string().contains("unresolved frame literal"))
        );
        let markers = BTreeMap::from([("to".into(), ("Preposition".into(), "To".into()))]);
        assert!(reconcile_frames(&mut lexemes, &markers).is_err());
        lexemes.push(Lexeme::invariant(
            "vocab:Preposition/To",
            "from",
            Category::Preposition,
            crate::source(deckmaste_lexical::SourceKind::Core, "test", "marker"),
        ));
        assert!(
            reconcile_frames(&mut lexemes, &markers)
                .unwrap_err()
                .to_string()
                .contains("does not spell")
        );
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
        assert!(
            add_features(
                &mut declarations,
                BTreeMap::from([("missing".into(), features.clone())])
            )
            .is_err()
        );
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
        assert!(
            add_features(
                &mut declarations,
                BTreeMap::from([("determiner".into(), replacement)])
            )
            .is_err()
        );
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
