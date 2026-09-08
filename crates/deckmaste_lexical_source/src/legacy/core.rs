use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Context;
use deckmaste_construction_core as construction;
use deckmaste_construction_core::macro_def as metadata;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Case;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Frame;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::WordForm;
use serde::Deserialize;
use serde::de::EnumAccess;
use serde::de::VariantAccess;
use serde::de::Visitor;

use crate::LexicalSources;
use crate::source;

const GRAMMAR_PATH: &str = "crates/deckmaste_english_v2/src/constructions.rs";

pub(crate) fn load(
    root: &Path,
    output: &mut LexicalSources,
    paradigms: &mut BTreeMap<String, crate::native::Paradigm>,
) -> anyhow::Result<()> {
    let text = fs::read_to_string(root.join(GRAMMAR_PATH))?;
    let invocation = construction::invocation_from_source(&text)?;
    let declarations = construction::parse_declarations(invocation.tokens)?;
    for declaration in declarations.declarations {
        match declaration {
            construction::Declaration::Vocab(vocab) => export_vocab(vocab, output),
            construction::Declaration::Lexeme(lexeme) => export_noun(lexeme, output)?,
            construction::Declaration::Construction(construction) => {
                for form in construction.forms {
                    for atom in form.atoms {
                        record_literal(
                            &atom,
                            &format!("{}::{}", construction.name, form.name),
                            output,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    let mut inventories = super::ron_documents(root)?
        .into_iter()
        .filter_map(|(path, source)| {
            ron::from_str::<Vec<CoreVerb>>(&source)
                .ok()
                .map(|verbs| (path, verbs))
        });
    let (path, verbs) = inventories
        .next()
        .context("no authored core verb inventory found in the transitional source tree")?;
    anyhow::ensure!(
        inventories.next().is_none(),
        "multiple authored core verb inventories found in the transitional source tree"
    );
    let provenance = path.strip_prefix(root).unwrap_or(&path).to_string_lossy();
    for verb in verbs {
        let paradigm = paradigms.remove(&format!("core-verb:{}", verb.identity.0));
        export_verb(verb, &provenance, output, paradigm);
    }
    Ok(())
}

fn record_literal(atom: &construction::FormAtom, owner: &str, output: &mut LexicalSources) {
    match atom {
        construction::FormAtom::Literal(value)
        | construction::FormAtom::LicensedLiteral(value)
        | construction::FormAtom::SentenceInitial(value) => {
            output
                .unmapped
                .push(format!("construction-literal {owner}: {:?}", value.value()));
        }
        construction::FormAtom::Bound(bound) => {
            if let Some(affix) = &bound.affix {
                output
                    .unmapped
                    .push(format!("construction-affix {owner}: {:?}", affix.value()));
            }
            record_literal(&bound.value, owner, output);
        }
        _ => {}
    }
}

fn inventory_category(name: &str) -> Option<Category> {
    Some(match name {
        "PredicativeAdjective" | "AttributiveAdjective" | "ColorWord" | "ScalarDegree" => {
            Category::Adjective
        }
        "Preposition" => Category::Preposition,
        "PredicateNegator" | "LocativeProform" | "FrequencyAdverb" | "FocusAdverb"
        | "ReplacementMarker" => Category::Adverb,
        "ComparativeQuantifier"
        | "FloatedQuantifier"
        | "SingularDemonstrative"
        | "DefiniteMarker" => Category::Determinative,
        "SubjectPronoun"
        | "ObjectPronoun"
        | "PossessiveDeterminerPronoun"
        | "PossessiveAbsolutePronoun"
        | "ReflexivePronoun"
        | "IndefinitePronoun" => Category::Pronoun,
        "TriggerMarker" => Category::Subordinator,
        "Variable" | "ChapterNumeral" => Category::Numeral,
        "Supertype" => Category::Catalog,
        "FixedCostSymbol" => Category::Symbol,
        _ => return None,
    })
}

fn export_vocab(vocab: construction::Vocab, output: &mut LexicalSources) {
    let inventory = vocab.name.to_string();
    let Some(category) = inventory_category(&inventory) else {
        for member in vocab.variants {
            output.unmapped.push(format!(
                "vocab {inventory}::{}: {:?}",
                member.name,
                member.word.value()
            ));
        }
        return;
    };
    let defaults = vocab
        .feature_defaults
        .iter()
        .map(|feature| (format!("{:?}", feature.feature), feature.value.to_string()))
        .collect::<BTreeMap<_, _>>();
    for member in vocab.variants {
        let owner = format!("vocab:{inventory}/{}", member.name);
        let mut lexeme = Lexeme::invariant(
            &owner,
            member.word.value(),
            category,
            source(SourceKind::Core, GRAMMAR_PATH, &owner),
        );
        lexeme.properties.features = defaults.clone();
        for feature in member.feature_overrides {
            lexeme
                .properties
                .features
                .insert(format!("{:?}", feature.feature), feature.value.to_string());
        }
        match inventory.as_str() {
            "SubjectPronoun" => lexeme.forms[0].features.case = Some(Case::Nominative),
            "ObjectPronoun" => lexeme.forms[0].features.case = Some(Case::Accusative),
            "PossessiveDeterminerPronoun" | "PossessiveAbsolutePronoun" => {
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
    declaration: construction::Lexeme,
    output: &mut LexicalSources,
) -> anyhow::Result<()> {
    let inventory = declaration.name.to_string();
    if inventory != "CommonNoun" || declaration.morphology != "EnglishNoun" {
        for member in declaration.members {
            output.unmapped.push(format!(
                "lexeme {inventory}::{}: {:?}",
                member.name,
                member.lemma.value()
            ));
        }
        return Ok(());
    }
    let defaults = declaration
        .feature_defaults
        .iter()
        .map(|feature| (format!("{:?}", feature.feature), feature.value.to_string()))
        .collect::<BTreeMap<_, _>>();
    for member in declaration.members {
        let owner = format!("lexeme:{inventory}/{}", member.name);
        let mut features = defaults.clone();
        for feature in member.feature_overrides {
            features.insert(format!("{:?}", feature.feature), feature.value.to_string());
        }
        let countability = match features.get("Countability").map(String::as_str) {
            Some("Count") => vec![Countability::Count],
            Some("Mass") => vec![Countability::Mass],
            Some("Both") => vec![Countability::Count, Countability::Mass],
            other => anyhow::bail!("{owner}: unmapped declared countability {other:?}"),
        };
        let mut lexeme = Lexeme::noun(
            &owner,
            member.lemma.value(),
            countability,
            source(SourceKind::Core, GRAMMAR_PATH, &owner),
        );
        lexeme.properties.features = features;
        for replacement in member.overrides {
            let form = match replacement.feature.to_string().as_str() {
                "Singular" => WordForm::Singular,
                "Plural" => WordForm::Plural,
                other => anyhow::bail!("{owner}: unmapped noun inflection {other}"),
            };
            let slot = lexeme
                .forms
                .iter_mut()
                .find(|slot| slot.form == form)
                .with_context(|| format!("{owner}: override for unavailable {form:?}"))?;
            slot.surfaces = Some(vec![replacement.surface.value()]);
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
    frames: Vec<CoreFrame>,
}

#[derive(Deserialize)]
enum CoreFrame {
    Predicate(Vec<metadata::FrameItem>),
    Auxiliary,
    ProVerb,
}

fn export_verb(
    verb: CoreVerb,
    path: &str,
    output: &mut LexicalSources,
    paradigm: Option<crate::native::Paradigm>,
) {
    let owner = format!("core-verb:{}", verb.identity.0);
    if paradigm.is_none()
        && verb
            .frames
            .iter()
            .all(|frame| matches!(frame, CoreFrame::Auxiliary))
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
    for frame in verb.frames {
        let (kind, items) = match frame {
            CoreFrame::Predicate(items) => (
                "Predicate",
                items.iter().map(super::plugins::frame_item).collect(),
            ),
            CoreFrame::Auxiliary => ("Auxiliary", Vec::new()),
            CoreFrame::ProVerb => ("ProVerb", Vec::new()),
        };
        lexeme.properties.frames.push(Frame {
            kind: kind.to_owned(),
            items,
        });
    }
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
        export_verb(verb, "test-source", &mut output, None);
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
        export_verb(verb, "test-source", &mut output, None);
        let lexeme = &output.lexemes[0];
        assert_eq!(lexeme.id, "core-verb:Draw");
        assert_eq!(lexeme.source.owner, "core-verb:Draw");
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        assert!(!lexicon.analyze("drawn").matches.is_empty());
        assert!(!lexicon.analyze("drew").matches.is_empty());
        assert!(lexicon.analyze("drawed").matches.is_empty());
    }

    #[test]
    fn unknown_vocabulary_is_reported_with_members_and_never_given_pos() {
        let declarations = construction::parse_declarations(
            "vocab UndeclaredDistribution { Example = \"mystery\", }"
                .parse()
                .unwrap(),
        )
        .unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        let construction::Declaration::Vocab(vocab) =
            declarations.declarations.into_iter().next().unwrap()
        else {
            panic!("expected vocab");
        };
        export_vocab(vocab, &mut output);
        assert!(output.lexemes.is_empty());
        assert_eq!(
            output.unmapped,
            ["vocab UndeclaredDistribution::Example: \"mystery\""]
        );
    }

    #[test]
    fn noun_export_preserves_irregular_override_and_mass_restriction() {
        let declarations = construction::parse_declarations(
            r#"
            lexeme CommonNoun using EnglishNoun {
                feature Countability = Count;
                Ability = "ability" { Plural = "abilities", },
                Mana = "mana" { feature Countability = Mass; },
            }
        "#
            .parse()
            .unwrap(),
        )
        .unwrap();
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };
        let construction::Declaration::Lexeme(noun) =
            declarations.declarations.into_iter().next().unwrap()
        else {
            panic!("expected lexeme");
        };
        export_noun(noun, &mut output).unwrap();
        let lexicon = Lexicon::new(output.lexemes).unwrap();
        assert!(!lexicon.analyze("abilities").matches.is_empty());
        assert!(lexicon.analyze("abilitys").matches.is_empty());
        assert!(!lexicon.analyze("mana").matches.is_empty());
        assert!(lexicon.analyze("manas").matches.is_empty());
    }
}
