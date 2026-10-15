//! Card names [CR#201.1] and their Shortened Names [CR#201.5c].

use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::SourceKind;

use crate::CARD_NAMES;
use crate::build::exact;
use crate::build::source;

/// The Shortened Names derived from a full card name: what precedes its first
/// comma (*Gisela, Blade of Goldnight*), its first ` the ` (*Gandalf the Grey*)
/// or its first ` of ` (*Daxos of Meletis*), or else its first word
/// (*Karn Liberated*). A candidate the rest of the vocabulary already spells
/// is dropped (*Sliver* in *Sliver Queen*), except a planeswalker type, which
/// is the name of the character the card is (*Jace* in *Jace, the Mind
/// Sculptor*).
#[must_use]
pub fn shortened_names(name: &str, vocabulary: &Lexicon) -> Vec<String> {
    let comma = name.split_once(',').map(|(head, _)| head);
    let mut candidates: Vec<&str> = comma.into_iter().collect();
    let head = comma.unwrap_or(name);
    for separator in [" the ", " of ", " and "] {
        if let Some((before, _)) = head.split_once(separator) {
            candidates.push(before);
        }
    }
    if comma.is_none()
        && let Some((before, last)) = head.rsplit_once(' ')
        && deckmaste_lexical::NumeralCodec::parse(deckmaste_lexical::Numeral::Roman, last).is_ok()
    {
        candidates.push(before);
    }
    let mut names: Vec<String> = Vec::new();
    let accept = |names: &mut Vec<String>, candidate: &str| {
        let candidate = candidate.trim();
        // A name ends in a capitalized word, never in *and* or *the*.
        if candidate
            .rsplit(' ')
            .next()
            .is_some_and(|last| last.starts_with(char::is_uppercase))
            && candidate != name
            && !names.iter().any(|n| n == candidate)
            && !spelled_by_other_vocabulary(candidate, vocabulary)
        {
            names.push(candidate.to_owned());
        }
    };
    for candidate in candidates {
        accept(&mut names, candidate);
    }
    if comma.is_none()
        && names.is_empty()
        && let Some((first, _)) = name.split_once(' ')
    {
        accept(&mut names, first);
    }
    names
}

fn spelled_by_other_vocabulary(candidate: &str, vocabulary: &Lexicon) -> bool {
    let analyzed = vocabulary.analyze(candidate);
    let whole = analyzed.tokens.len();
    analyzed.matches.iter().any(|found| {
        let LexicalReading::Word(value) = &found.reading else {
            return false;
        };
        found.start == 0
            && found.end == whole
            && vocabulary.lexemes()[&value.lexeme].binding == deckmaste_lexical::Binding::Free
            && !vocabulary.lexemes()[&value.lexeme]
                .properties
                .classes
                .contains("PlaneswalkerType")
    })
}

/// The Lexeme id of a card name.
#[must_use]
pub fn card_name_id(name: &str) -> LexemeId {
    LexemeId::new(&format!("name:{name}"))
}

/// One Lexeme per card name, each with its Shortened Names in its own card's
/// scope only.
pub(crate) fn card_names(text: &str, vocabulary: &Lexicon) -> Vec<Lexeme> {
    let mut seen = std::collections::BTreeSet::new();
    text.lines()
        .map(str::trim)
        .filter(|name| !name.is_empty() && seen.insert(*name))
        .map(|name| {
            let mut lexeme = exact(
                card_name_id(name).as_str(),
                name,
                Category::Catalog,
                source(SourceKind::Catalog, CARD_NAMES, name),
            );
            let short = shortened_names(name, vocabulary);
            if !short.is_empty() {
                let mut surfaces = vec![name.to_owned()];
                surfaces.extend(short.iter().cloned());
                lexeme.forms[0].surfaces = Some(surfaces);
                lexeme.card_scoped_surfaces = short.into_iter().collect();
            }
            lexeme
        })
        .collect()
}
