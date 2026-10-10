use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::DiagnosticGrammar;
use super::DiagnosticKeywordVerbGrammar;
use super::Grammar;
use super::GrammarRow;
use super::GrammarSourceMap;
use super::KeywordVerbGrammar;
use super::NegativePrefixJoin;
use super::ReadError;
use super::SourcePosition;
use super::ValidationError;
use super::declaration_reader;
use super::macro_options;
use super::normalize_grammar;
use super::raw_position;
use super::ron_files_recursive;
use super::source_map_parse_error;
use super::validate_keyword_verb;
use super::validation_error_at;

/// Ordinary lexical recipes owned by one predicate or event definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HelperGrammar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Grammar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<KeywordVerbGrammar>,
    #[serde(default)]
    pub noun_modifier: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negative_prefix_join: Option<NegativePrefixJoin>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct HelperMetadata {
    #[serde(default)]
    grammar: Option<HelperGrammar>,
}

/// A helper's authored lexical recipes and normalized term, with its owner.
#[derive(Debug, Clone)]
pub struct HelperLexicalDeclaration {
    pub name: String,
    pub kind: String,
    pub path: PathBuf,
    pub grammar: HelperGrammar,
    pub term: Option<GrammarRow>,
}

#[derive(Deserialize)]
struct DiagnosticHelper<'a> {
    #[serde(default, borrow)]
    metadata: DiagnosticHelperMetadata<'a>,
}

#[derive(Default, Deserialize)]
struct DiagnosticHelperMetadata<'a> {
    #[serde(default, borrow)]
    grammar: Option<DiagnosticHelperGrammar<'a>>,
}

#[derive(Deserialize)]
struct DiagnosticHelperGrammar<'a> {
    #[serde(default, borrow)]
    term: Option<DiagnosticGrammar<'a>>,
    #[serde(default, borrow)]
    verb: Option<DiagnosticKeywordVerbGrammar<'a>>,
}

/// Reads lexical declarations from predicate and event helper definitions.
/// Helpers without `grammar` contribute nothing. The eight declaration
/// families and their facts consumers continue to use `read_builtin_v2`.
///
/// # Errors
/// Returns source-positioned schema and grammar validation errors, I/O errors,
/// or duplicate helper ownership errors.
pub fn read_builtin_v2_helpers(
    root: impl AsRef<Path>,
) -> Result<Vec<HelperLexicalDeclaration>, ReadError> {
    let root = root.as_ref();
    if root.file_name().and_then(|name| name.to_str()) != Some("builtin") || !root.is_dir() {
        return Err(validation_error_at(
            root,
            SourcePosition { line: 1, column: 1 },
            ValidationError::InvalidBuiltinRoot,
        ));
    }
    let reader = declaration_reader().map_err(|source| ReadError::Reader {
        path: root.into(),
        source: Box::new(source),
    })?;
    let mut paths = Vec::new();
    for family in ["predicates", "events"] {
        paths.extend(ron_files_recursive(&root.join("macros").join(family))?);
    }
    paths.sort();
    let mut declarations = Vec::new();
    let mut owners = HashSet::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).map_err(|source| ReadError::Io {
            path: path.clone(),
            source,
        })?;
        if let Some(declaration) = read_helper(&reader, path, &text)? {
            if !owners.insert((declaration.kind.clone(), declaration.name.clone())) {
                return Err(source_map_parse_error(
                    &declaration.path,
                    SourcePosition { line: 1, column: 1 },
                    "duplicate lexical helper owner",
                ));
            }
            declarations.push(declaration);
        }
    }
    Ok(declarations)
}

fn read_helper(
    reader: &macro_ron::MacroSet,
    path: PathBuf,
    text: &str,
) -> Result<Option<HelperLexicalDeclaration>, ReadError> {
    let definition: macro_ron::MacroDef<HelperMetadata> =
        reader.read_str(text).map_err(|source| ReadError::Parse {
            path: path.clone(),
            source: Box::new(source),
        })?;
    let Some(grammar) = definition.metadata.grammar else {
        return Ok(None);
    };
    let position = SourcePosition { line: 1, column: 1 };
    let kind = match definition.kinds.first().map(macro_ron::Ident::as_str) {
        Some("Predicate") => "predicate",
        Some("GameEvent") => "event",
        _ => {
            return Err(validation_error_at(
                &path,
                position,
                ValidationError::InvalidDeclarationKind,
            ));
        }
    };
    let diagnostic: DiagnosticHelper<'_> =
        macro_options()
            .from_str(text)
            .map_err(|source| ReadError::Parse {
                path: path.clone(),
                source: Box::new(source),
            })?;
    let diagnostic = diagnostic.metadata.grammar.ok_or_else(|| {
        source_map_parse_error(&path, position, "helper grammar source map missing")
    })?;
    if (grammar.noun_modifier || grammar.negative_prefix_join.is_some())
        && !matches!(grammar.term, Some(Grammar::Noun { .. }))
    {
        return Err(validation_error_at(
            &path,
            position,
            ValidationError::TypeWordGrammarMismatch,
        ));
    }
    let term = grammar
        .term
        .clone()
        .map(|term| {
            let diagnostic = diagnostic.term.as_ref().ok_or_else(|| {
                source_map_parse_error(&path, position, "helper term source map missing")
            })?;
            let source_map = GrammarSourceMap::from_diagnostic(&path, text, position, diagnostic)?;
            normalize_grammar(&path, &source_map, position, &[], term)
        })
        .transpose()?;
    if let Some(verb) = &grammar.verb {
        let diagnostic = diagnostic.verb.as_ref().ok_or_else(|| {
            source_map_parse_error(&path, position, "helper verb source map missing")
        })?;
        validate_keyword_verb(
            &path,
            raw_position(&path, text, diagnostic.bare, position)?,
            raw_position(&path, text, diagnostic.frames, position)?,
            verb,
        )?;
    }
    Ok(Some(HelperLexicalDeclaration {
        name: definition.name.to_string(),
        kind: kind.into(),
        path,
        grammar,
        term,
    }))
}
