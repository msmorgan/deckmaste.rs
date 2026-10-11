use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Case;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::WordForm;
use deckmaste_macro_def as metadata;
use serde::Deserialize;
use serde::de::EnumAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

use crate::LexicalSources;
use crate::LoadError;
use crate::source;

const VOCABULARY_PATH: &str = "crates/deckmaste_lexical_source/lexicon/vocabulary.ron";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Vocabulary {
    // Stable declaration provenance survives a change of storage format.
    provenance_path: String,
    declarations: Vec<VocabularyDeclaration>,
}

#[derive(Deserialize)]
enum VocabularyDeclaration {
    Vocab(Vocab),
    Lexeme(NounClass),
    Morphology(Morphology),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Vocab {
    name: String,
    #[serde(default)]
    features: BTreeMap<String, String>,
    members: Vec<VocabMember>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VocabMember {
    name: String,
    surface: String,
    #[serde(default)]
    features: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NounClass {
    name: String,
    morphology: String,
    #[serde(default)]
    features: BTreeMap<String, String>,
    members: Vec<NounMember>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NounMember {
    name: String,
    lemma: String,
    #[serde(default)]
    overrides: Vec<InflectionOverride>,
    #[serde(default)]
    features: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InflectionOverride {
    form: String,
    surface: String,
}

// Retain the authored recipes even though lexical export uses its own morphology.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Morphology {
    name: String,
    feature: String,
    recipe: String,
}

pub(crate) fn load(
    root: &Path,
    output: &mut LexicalSources,
    paradigms: &mut BTreeMap<String, crate::native::Paradigm>,
) -> Result<(), LoadError> {
    let path = root.join(VOCABULARY_PATH);
    let text = crate::error::read(&path)?;
    let vocabulary: Vocabulary = crate::error::decode(&path, &text)?;
    for declaration in vocabulary.declarations {
        match declaration {
            VocabularyDeclaration::Vocab(vocab) => {
                export_vocab(vocab, &vocabulary.provenance_path, output);
            }
            VocabularyDeclaration::Lexeme(lexeme) => {
                export_noun(lexeme, &vocabulary.provenance_path, output)?;
            }
            VocabularyDeclaration::Morphology(morphology) => {
                let _ = (morphology.name, morphology.feature, morphology.recipe);
            }
        }
    }
    let mut inventories = super::ron_documents(root)?
        .into_iter()
        .filter_map(|(path, source)| {
            ron::from_str::<Vec<CoreVerb>>(&source)
                .ok()
                .map(|verbs| (path, verbs))
        });
    let (path, verbs) = inventories.next().ok_or(LoadError::MissingVerbInventory)?;
    if let Some((other, _)) = inventories.next() {
        return Err(LoadError::MultipleVerbInventories {
            paths: std::iter::once(path)
                .chain(std::iter::once(other))
                .chain(inventories.map(|(path, _)| path))
                .collect(),
        });
    }
    let provenance = path.strip_prefix(root).unwrap_or(&path).to_string_lossy();
    for verb in verbs {
        let paradigm = paradigms.remove(&format!("core-verb:{}", verb.identity.0));
        export_verb(&verb, &provenance, output, paradigm);
    }
    Ok(())
}

fn inventory_category(name: &str) -> Option<Category> {
    Some(match name {
        "PredicativeAdjective" | "AttributiveAdjective" | "ColorWord" | "ScalarDegree" => {
            Category::Adjective
        }
        "Preposition" | "TriggerMarker" => Category::Preposition,
        "PredicateNegator" | "FrequencyAdverb" | "FocusAdverb" => Category::Adverb,
        "ComparativeQuantifier"
        | "CompoundDeterminative"
        | "FloatedQuantifier"
        | "SingularDemonstrative"
        | "DefiniteMarker" => Category::Determinative,
        "SubjectPronoun"
        | "ObjectPronoun"
        | "PossessiveDeterminerPronoun"
        | "ReflexivePronoun"
        | "IndefinitePronoun" => Category::Pronoun,
        "Variable" | "ChapterNumeral" => Category::Numeral,
        "Supertype" => Category::Catalog,
        "FixedCostSymbol" => Category::Symbol,
        _ => return None,
    })
}

fn export_vocab(vocab: Vocab, provenance_path: &str, output: &mut LexicalSources) {
    let inventory = vocab.name;
    let Some(category) = inventory_category(&inventory) else {
        for member in vocab.members {
            output.unmapped.push(format!(
                "vocab {inventory}::{}: {:?}",
                member.name, member.surface
            ));
        }
        return;
    };
    let defaults = vocab.features;
    for member in vocab.members {
        let owner = format!("vocab:{inventory}/{}", member.name);
        let mut lexeme = Lexeme::invariant(
            &owner,
            member.surface,
            category,
            source(SourceKind::Core, provenance_path, &owner),
        );
        lexeme.properties.features = defaults.clone();
        lexeme.properties.features.extend(member.features);
        match inventory.as_str() {
            "SubjectPronoun" => lexeme.forms[0].features.case = Some(Case::Nominative),
            "ObjectPronoun" => lexeme.forms[0].features.case = Some(Case::Accusative),
            "PossessiveDeterminerPronoun" => {
                lexeme.forms[0].features.case = Some(Case::Genitive);
            }
            "Variable" | "ChapterNumeral" | "FixedCostSymbol" => {
                lexeme.capitalization = Capitalization::Exact;
            }
            _ => {}
        }
        output.lexemes.push(lexeme);
    }
}

fn export_noun(
    declaration: NounClass,
    provenance_path: &str,
    output: &mut LexicalSources,
) -> Result<(), LoadError> {
    let inventory = declaration.name;
    if inventory != "CommonNoun" || declaration.morphology != "EnglishNoun" {
        for member in declaration.members {
            output.unmapped.push(format!(
                "lexeme {inventory}::{}: {:?}",
                member.name, member.lemma
            ));
        }
        return Ok(());
    }
    let defaults = declaration.features;
    for member in declaration.members {
        let owner = format!("lexeme:{inventory}/{}", member.name);
        let mut features = defaults.clone();
        features.extend(member.features);
        let countability = match features.get("Countability").map(String::as_str) {
            Some("Count") => vec![Countability::Count],
            Some("Mass") => vec![Countability::Mass],
            Some("Both") => vec![Countability::Count, Countability::Mass],
            other => {
                return Err(LoadError::Countability {
                    owner,
                    value: other.map(str::to_owned),
                });
            }
        };
        let mut lexeme = Lexeme::noun(
            &owner,
            member.lemma,
            countability,
            source(SourceKind::Core, provenance_path, &owner),
        );
        lexeme.properties.features = features;
        for replacement in member.overrides {
            let form = match replacement.form.as_str() {
                "Singular" => WordForm::Singular,
                "Plural" => WordForm::Plural,
                other => {
                    return Err(LoadError::NounInflection {
                        owner,
                        value: other.into(),
                    });
                }
            };
            let slot = lexeme
                .forms
                .iter_mut()
                .find(|slot| slot.form == form)
                .ok_or_else(|| LoadError::UnavailableForm {
                    owner: owner.clone(),
                    form,
                })?;
            slot.surfaces = Some(vec![replacement.surface]);
        }
        output.lexemes.push(lexeme);
    }
    Ok(())
}

struct CoreVerbIdentity(String);

impl std::fmt::Debug for CoreVerbIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn core_verb_identity<'de, D>(deserializer: D) -> Result<CoreVerbIdentity, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct IdentityVisitor;

    impl<'de> Visitor<'de> for IdentityVisitor {
        type Value = CoreVerbIdentity;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a unit identity")
        }

        fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: EnumAccess<'de>,
        {
            let (identity, variant) = data.variant_seed(macro_ron::IdentSeed)?;
            variant.unit_variant()?;
            Ok(CoreVerbIdentity(identity.as_str().to_owned()))
        }
    }

    deserializer.deserialize_enum("", &[], IdentityVisitor)
}

#[derive(Deserialize)]
struct CoreVerb {
    #[serde(deserialize_with = "core_verb_identity")]
    identity: CoreVerbIdentity,
    bare: String,
    third_person: String,
    preterite: Option<String>,
    participle: Option<String>,
    frames: Vec<metadata::VerbDeclarationFrame>,
}

fn export_verb(
    verb: &CoreVerb,
    path: &str,
    output: &mut LexicalSources,
    paradigm: Option<crate::native::Paradigm>,
) {
    let owner = format!("core-verb:{}", verb.identity.0);
    if paradigm.is_none()
        && verb
            .frames
            .iter()
            .all(|frame| matches!(frame, metadata::VerbDeclarationFrame::Auxiliary))
    {
        output.unmapped.push(format!(
            "auxiliary {owner}: {:?}/{:?} needs declared finite/nonfinite applicability",
            verb.bare, verb.third_person
        ));
        return;
    }
    let mut lexeme = Lexeme::verb(&owner, &verb.bare, source(SourceKind::Core, path, &owner));
    lexeme
        .properties
        .features
        .insert("OriginalIdentity".into(), format!("{:?}", verb.identity));
    lexeme.forms.retain_mut(|slot| match slot.form {
        WordForm::Preterite => {
            slot.surfaces = verb.preterite.clone().map(|surface| vec![surface]);
            true
        }
        WordForm::PastParticiple => {
            slot.surfaces = verb.participle.clone().map(|surface| vec![surface]);
            true
        }
        WordForm::Present
            if slot.features.number == Some(Number::Singular)
                && slot.features.person == Some(Person::Third) =>
        {
            slot.surfaces = Some(vec![verb.third_person.clone()]);
            true
        }
        _ => true,
    });
    lexeme.properties.frames = super::plugins::declared_frames(&verb.frames);
    if let Some(paradigm) = paradigm {
        lexeme.forms = paradigm.forms;
        lexeme.properties.frames = paradigm.frames;
        lexeme.properties.features.extend(paradigm.features);
        lexeme
            .properties
            .features
            .insert("ParadigmSource".into(), crate::native::PATH.into());
    }
    output.lexemes.push(lexeme);
}

#[cfg(test)]
mod tests {
    use deckmaste_lexical::Lexicon;

    use super::*;

    #[test]
    fn missing_legacy_verb_forms_use_the_declared_regular_paradigm() {
        let verb: CoreVerb = ron::from_str(r#"(identity: Attack, bare: "attack", third_person: "attacks", frames: [Predicate([])])"#).unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        export_verb(&verb, "test-source", &mut output, None);
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        for surface in ["attack", "attacks", "attacked", "attacking"] {
            assert!(
                !lexicon.analyze(surface).matches.is_empty(),
                "missing {surface}"
            );
        }
    }

    #[test]
    fn export_reads_authored_identity_instead_of_reconstructing_it_from_spelling() {
        let verb: CoreVerb = ron::from_str(r#"(identity: Draw, bare: "draw", third_person: "draws", preterite: Some("drew"), participle: Some("drawn"), frames: [Predicate([ObjectNounPhrase])])"#).unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        export_verb(&verb, "test-source", &mut output, None);
        let lexeme = &output.lexemes[0];
        assert_eq!(lexeme.id, "core-verb:Draw");
        assert_eq!(lexeme.source.owner, "core-verb:Draw");
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        assert_ne!(
            lexicon.analyze("drawn").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
        assert_ne!(
            lexicon.analyze("drew").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
        assert_eq!(
            lexicon.analyze("drawed").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
    }

    #[test]
    fn unknown_vocabulary_is_reported_with_members_and_never_given_pos() {
        let vocab: Vocab = ron::from_str(
            r#"(name: "UndeclaredDistribution", members: [(name: "Example", surface: "mystery")])"#,
        )
        .unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        export_vocab(vocab, "test-source", &mut output);
        assert_eq!(output.lexemes, [] as [deckmaste_lexical::Lexeme; 0]);
        assert_eq!(
            output.unmapped,
            ["vocab UndeclaredDistribution::Example: \"mystery\""]
        );
    }

    #[test]
    fn noun_export_preserves_irregular_override_and_mass_restriction() {
        let noun: NounClass = ron::from_str(r#"
            (
                name: "CommonNoun", morphology: "EnglishNoun",
                features: {"Countability": "Count"},
                members: [
                    (name: "Ability", lemma: "ability", overrides: [(form: "Plural", surface: "abilities")]),
                    (name: "Mana", lemma: "mana", features: {"Countability": "Mass"}),
                ],
            )
        "#).unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        export_noun(noun, "test-source", &mut output).unwrap();
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        assert_ne!(
            lexicon.analyze("abilities").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
        assert_eq!(
            lexicon.analyze("abilitys").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
        assert_ne!(
            lexicon.analyze("mana").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
        assert_eq!(
            lexicon.analyze("manas").matches,
            [] as [deckmaste_lexical::LexicalMatch; 0]
        );
    }
}
