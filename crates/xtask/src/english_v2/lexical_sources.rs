//! Export adapters; the resulting declarations are consumed without this
//! compiler dependency.

mod core;
mod plugins;
mod supplement;

use std::path::Path;

use anyhow::Context;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

pub(super) struct LexicalSources {
    pub lexemes: Vec<Lexeme>,
    pub unmapped: Vec<String>,
}

pub(super) fn load(root: &Path) -> anyhow::Result<LexicalSources> {
    let mut output = LexicalSources {
        lexemes: Vec::new(),
        unmapped: Vec::new(),
    };
    core::load(root, &mut output)?;
    plugins::load(root, &mut output)?;
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
    supplement::apply(root, &mut output)?;
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
