//! Authored-source adapters for the independent lexical engine.
//!
//! Callers receive normalized declarations and do not observe the temporary
//! source formats used to salvage the current inventory.

mod card_names;
mod error;
pub use error::LoadError;

mod legacy;
mod native;
mod supplement;

use std::path::Path;

use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;

pub struct LexicalSources {
    pub lexemes: Vec<Lexeme>,
    pub unmapped: Vec<String>,
}

/// Loads the current authored lexical inventory from a workspace source tree.
///
/// # Errors
/// Returns an error when a source cannot be read or normalized, an override
/// has no target, or two declarations produce the same lexical identity.
pub fn load_workspace(root: &Path) -> Result<LexicalSources, LoadError> {
    let mut output = LexicalSources {
        lexemes: Vec::new(),
        unmapped: Vec::new(),
    };
    let mut native = native::load(root)?;
    output.lexemes.extend(native.lexemes);
    legacy::core::load(root, &mut output, &mut native.core_verb_paradigms)?;
    if !native.core_verb_paradigms.is_empty() {
        return Err(LoadError::UnresolvedParadigms {
            owners: native.core_verb_paradigms.into_keys().collect(),
        });
    }
    legacy::plugins::load(root, &mut output)?;
    let directory = root.join("data/gen/catalogs");
    let catalogs = deckmaste_catalogs::CatalogSet::load(&directory)?;
    let nicknames = card_names::load(root)?;
    for kind in catalogs.kinds() {
        for surface in catalogs.get(kind) {
            let owner = format!("catalog:{}/{}", kind.filename(), surface);
            let source = source(
                SourceKind::Catalog,
                &format!("data/gen/catalogs/{}", kind.filename()),
                &owner,
            );
            let category = match kind {
                deckmaste_catalogs::CatalogKind::KeywordAbilities
                | deckmaste_catalogs::CatalogKind::KeywordActions
                | deckmaste_catalogs::CatalogKind::AbilityWords => Category::Keyword,
                _ => Category::Catalog,
            };
            let mut lexeme = Lexeme::invariant(&owner, surface, category, source);
            lexeme.capitalization = Capitalization::Exact;
            if kind == deckmaste_catalogs::CatalogKind::CardNames {
                if let Some(nickname) = nicknames.get(surface) {
                    lexeme.forms[0].surfaces = Some(vec![surface.clone(), nickname.clone()]);
                }
                lexeme
                    .properties
                    .features
                    .insert("IdentityUse".into(), "Name".into());
            }
            if kind == deckmaste_catalogs::CatalogKind::AbilityWords {
                lexeme
                    .properties
                    .features
                    .insert("LabelKind".into(), "AbilityWord".into());
            }
            if matches!(
                kind,
                deckmaste_catalogs::CatalogKind::ArtifactTypes
                    | deckmaste_catalogs::CatalogKind::BattleTypes
                    | deckmaste_catalogs::CatalogKind::CreatureTypes
                    | deckmaste_catalogs::CatalogKind::EnchantmentTypes
                    | deckmaste_catalogs::CatalogKind::LandTypes
                    | deckmaste_catalogs::CatalogKind::PlaneswalkerTypes
                    | deckmaste_catalogs::CatalogKind::SpellTypes
            ) {
                lexeme
                    .properties
                    .features
                    .insert("TypeLineRole".into(), "Subtype".into());
            }
            output.lexemes.push(lexeme);
        }
    }
    supplement::apply(root, &mut output)?;
    native::replace_forms(&mut output.lexemes, native.form_replacements)?;
    native::add_frames(&mut output.lexemes, native.frame_additions)?;
    native::add_features(&mut output.lexemes, native.feature_additions)?;
    native::add_adjective_classes(&mut output.lexemes, native.adjective_classes)?;
    native::reconcile_frames(&mut output.lexemes, &native.frame_markers)?;
    for lexeme in &mut output.lexemes {
        if lexeme.source.kind == SourceKind::Catalog || lexeme.category == Category::Keyword {
            lexeme.surface_structure = SurfaceStructure::Opaque;
        } else if lexeme.lemma.contains(' ') {
            lexeme.surface_structure = SurfaceStructure::Multiword;
        }
    }
    output.lexemes.sort_by(|left, right| left.id.cmp(&right.id));
    if let Some(pair) = output
        .lexemes
        .windows(2)
        .find(|pair| pair[0].id == pair[1].id)
    {
        return Err(LoadError::DuplicateIdentity {
            identity: pair[0].id.clone(),
        });
    }
    output.unmapped.sort();
    output.unmapped.dedup();
    Ok(output)
}

fn source(kind: SourceKind, path: &str, owner: &str) -> Source {
    Source {
        kind,
        path: path.to_owned(),
        owner: owner.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use deckmaste_lexical::FrameItem;
    use deckmaste_lexical::Lexeme;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::Lexicon;

    use super::load_workspace;

    fn contains_marked(item: &FrameItem) -> bool {
        match item {
            FrameItem::Marked { .. } => true,
            FrameItem::Optional(item) => contains_marked(item),
            FrameItem::Argument(_) | FrameItem::Marker { .. } | FrameItem::Literal(_) => false,
        }
    }

    #[test]
    fn discovered_source_tree_roundtrips_a_marked_lexeme_through_the_engine() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let sources = load_workspace(&root).unwrap();
        let authored = sources
            .lexemes
            .iter()
            .find(|lexeme| {
                lexeme
                    .properties
                    .frames
                    .iter()
                    .flat_map(|frame| &frame.items)
                    .any(contains_marked)
            })
            .expect("the authored inventory contains a marked lexical frame");
        let serialized = ron::to_string(authored).unwrap();
        let decoded = ron::from_str::<Lexeme>(&serialized).unwrap();
        assert_eq!(&decoded, authored);

        let lexicon = Lexicon::new([decoded]).unwrap();
        for value in lexicon.values() {
            let reading = LexicalReading::Word(value.clone());
            let surface = lexicon.realize(&reading).unwrap();
            let analyzed = lexicon.analyze_source(&surface, None);
            assert!(analyzed.matches.iter().any(|found| {
                found.start == 0 && found.end == analyzed.tokens.len() && found.reading == reading
            }));
        }
    }
}

#[cfg(test)]
mod card_name_tests {
    use super::*;
    use deckmaste_lexical::{LexicalReading, Lexicon};

    #[test]
    fn legendary_nicknames_are_reversible_variants_of_the_full_name() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let sources = load_workspace(&root).unwrap();
        for (full, short) in [
            ("Nissa Revane", "Nissa"),
            ("King Darien XLVIII", "King Darien"),
            ("The Balrog, Durin's Bane", "The Balrog"),
            ("Tor Wauki the Younger", "Tor Wauki"),
        ] {
            let lexeme = sources
                .lexemes
                .iter()
                .find(|lexeme| lexeme.lemma == full)
                .unwrap();
            assert_eq!(
                lexeme.forms[0].surfaces,
                Some(vec![full.into(), short.into()])
            );
            let lexicon = Lexicon::new([lexeme.clone()]).unwrap();
            let mut spelled = Vec::new();
            for value in lexicon.values() {
                let reading = LexicalReading::Word(value.clone());
                let text = lexicon.realize(&reading).unwrap();
                let analyzed = lexicon.analyze_source(&text, None);
                assert!(analyzed.matches.iter().any(|found| found.start == 0
                    && found.end == analyzed.tokens.len()
                    && found.reading == reading));
                spelled.push(text);
            }
            assert_eq!(spelled, [full, short]);
        }
    }
}
