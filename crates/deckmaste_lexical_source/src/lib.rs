//! Authored-source adapters for the independent lexical engine.
//!
//! Callers receive normalized declarations and do not observe the temporary
//! source formats used to salvage the current inventory.

mod legacy;

use std::path::Path;

use anyhow::Context;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

pub struct LexicalSources {
    pub lexemes: Vec<Lexeme>,
    pub unmapped: Vec<String>,
}

/// Loads the current authored lexical inventory from a workspace source tree.
///
/// # Errors
/// Returns an error when a source cannot be read or normalized, an override
/// has no target, or two declarations produce the same lexical identity.
pub fn load_workspace(root: &Path) -> anyhow::Result<LexicalSources> {
    let mut output = LexicalSources {
        lexemes: Vec::new(),
        unmapped: Vec::new(),
    };
    legacy::core::load(root, &mut output)?;
    legacy::plugins::load(root, &mut output)?;
    let directory = root.join("data/gen/catalogs");
    let catalogs = deckmaste_catalogs::CatalogSet::load(&directory)
        .context("loading canonical lexical catalogs")?;
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
            output.lexemes.push(lexeme);
        }
    }
    legacy::supplement::apply(root, &mut output)?;
    output.lexemes.sort_by(|left, right| left.id.cmp(&right.id));
    anyhow::ensure!(
        output
            .lexemes
            .windows(2)
            .all(|pair| pair[0].id != pair[1].id),
        "duplicate lexical export identity"
    );
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
