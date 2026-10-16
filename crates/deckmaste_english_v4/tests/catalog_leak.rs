//! No catalog vocabulary leaks into the grammar data file [CR#701.1]: the
//! grammar names only core closed-class lexemes, and none of its own names
//! spells a catalog item.

use std::collections::BTreeSet;

use deckmaste_lexical::Category;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical_source_v4::Sources;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::grammar::SchemaDecl;
use deckmaste_syntax::grammar::SymbolDecl;

/// Grammatical terms that happen to spell a catalog item, each the standard
/// name of a grammatical concept rather than a use of the catalog word.
// Case is the nominative/accusative/genitive distinction in english-v4 §1.
const GRAMMATICAL_TERMS: [&str; 1] = ["case"];

#[test]
fn the_grammar_names_only_core_closed_class_lexemes() {
    let grammar = deckmaste_english_v4::grammar().unwrap();
    let (lexemes, _) = Sources::load().unwrap().vocabulary().unwrap();
    let ids: Vec<&String> = grammar
        .closed_words
        .values()
        .chain(grammar.lexical_classes.values().flatten())
        .collect();
    for id in ids {
        let lexeme = lexemes
            .iter()
            .find(|lexeme| lexeme.id == LexemeId::new(id))
            .unwrap_or_else(|| {
                panic!("the grammar names `{id}`, which the lexicon does not declare")
            });
        assert_eq!(lexeme.source.kind, SourceKind::Core, "{id}");
        assert!(
            matches!(
                lexeme.category,
                Category::Determinative
                    | Category::Pronoun
                    | Category::Preposition
                    | Category::Coordinator
                    | Category::Subordinator
                    | Category::Clitic
                    | Category::Verb
            ),
            "{id} is a {:?}",
            lexeme.category
        );
    }
}

/// The words of an identifier: `SubjectPredicate` is `subject predicate`.
fn words(identifier: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    for ch in identifier.chars() {
        if ch.is_uppercase() && !word.is_empty() || !ch.is_alphanumeric() {
            if !word.is_empty() {
                words.push(std::mem::take(&mut word));
            }
            if !ch.is_alphanumeric() {
                continue;
            }
        }
        word.extend(ch.to_lowercase());
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

fn catalog_lemmas() -> BTreeSet<String> {
    let (lexemes, _) = Sources::load().unwrap().vocabulary().unwrap();
    lexemes
        .iter()
        .filter(|lexeme| lexeme.source.kind == SourceKind::Catalog)
        .map(|lexeme| lexeme.lemma.to_lowercase())
        .collect()
}

/// Every name the grammar declares or uses, with each catalog item it spells.
fn leaks(grammar: &Grammar, catalog: &BTreeSet<String>) -> BTreeSet<String> {
    let mut names: Vec<&str> = vec![grammar.root.as_str()];
    names.extend(grammar.features.keys().map(String::as_str));
    names.extend(grammar.features.values().flatten().map(String::as_str));
    names.extend(grammar.categories.keys().map(String::as_str));
    names.extend(grammar.closed_words.keys().map(String::as_str));
    names.extend(grammar.lexical_classes.keys().map(String::as_str));
    names.extend(grammar.typed_classes.iter().map(String::as_str));
    names.extend(grammar.slot_categories.values().map(String::as_str));
    for production in &grammar.productions {
        names.push(&production.name);
        names.push(&production.lhs);
        names.extend(production.transparent.iter().map(String::as_str));
        for symbol in &production.rhs {
            match symbol {
                SymbolDecl::Attached(child, name, _)
                | SymbolDecl::Cat(child, name)
                | SymbolDecl::Hosted(child, name, _)
                | SymbolDecl::Word(child, name)
                | SymbolDecl::Class(child, name) => names.extend([child.as_str(), name.as_str()]),
                SymbolDecl::FreeLex(child, _)
                | SymbolDecl::Numeral(child, _)
                | SymbolDecl::Lex(child, _) => names.push(child),
                SymbolDecl::Punct(_) | SymbolDecl::Mark(..) => {}
            }
        }
    }
    for schema in &grammar.schemas {
        match schema {
            SchemaDecl::Premodification { modifier, host, .. } => {
                names.extend([modifier.as_str(), host.as_str()]);
            }
            SchemaDecl::Parameters { phrase, .. } => names.push(phrase),
            SchemaDecl::Valence {
                phrase,
                passive,
                adjectival,
                ..
            } => {
                names.push(phrase);
                names.extend(passive.as_deref());
                names.extend(adjectival.as_deref());
            }
            SchemaDecl::Adjunct { host, adjunct, .. } => {
                names.extend([host.as_str(), adjunct.as_str()]);
            }
            SchemaDecl::Coordination {
                category,
                coordinators,
                ..
            } => {
                names.extend([category.as_str(), coordinators.as_str()]);
            }
        }
    }
    let mut leaks = BTreeSet::new();
    for name in names {
        let words = words(name);
        for start in 0..words.len() {
            for end in start + 1..=words.len() {
                let phrase = words[start..end].join(" ");
                if catalog.contains(&phrase) && !GRAMMATICAL_TERMS.contains(&phrase.as_str()) {
                    leaks.insert(format!("{name}: {phrase}"));
                }
            }
        }
    }
    leaks
}

#[test]
fn no_grammar_name_spells_a_catalog_item() {
    let grammar = deckmaste_english_v4::grammar().unwrap();
    let leaks = leaks(&grammar, &catalog_lemmas());
    assert!(
        leaks.is_empty(),
        "grammar names spelling catalog items: {leaks:?}"
    );
}

#[test]
fn the_lint_finds_a_keyword_action_named_in_a_production() {
    let mut grammar = deckmaste_english_v4::grammar().unwrap();
    grammar.productions[0].name = "DestroyTarget".to_owned();
    assert_eq!(
        leaks(&grammar, &catalog_lemmas()),
        BTreeSet::from(["DestroyTarget: destroy".to_owned()])
    );
}
