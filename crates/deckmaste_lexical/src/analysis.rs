use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::Binding;
use crate::Capitalization;
use crate::Category;
use crate::FeatureBundle;
use crate::Lexeme;
use crate::LexicalReading;
use crate::LexicalValue;
use crate::SurfaceCase;
use crate::morphology;
use crate::numeral::Numeral;
use crate::numeral::NumeralCodec;

#[derive(Debug, thiserror::Error)]
pub enum LexicalError {
    #[error("invalid lexical declaration {lexeme}: {reason}")]
    Declaration { lexeme: String, reason: String },
    #[error("lexical value is not licensed by this lexicon")]
    UnlicensedValue,
}

/// What an italic run written at the head of an ability analyses as
/// [CR#207.2c,207.2d].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItalicHead {
    /// The run is spelled by a declared keyword-category Lexeme. At this
    /// position that is an ability word: the ability words are a listed
    /// inventory [CR#207.2c] and the only declared vocabulary an italic head
    /// carries.
    AbilityWord(LexicalValue),
    /// No declaration spells the run. Flavor words are listed nowhere
    /// [CR#207.2d] — each is tailored to the one ability it heads — so there
    /// is no inventory to consult and the run itself is the label.
    FlavorWord { label: String },
}

/// Half-open occurrences in the lossless Unicode-scalar token sequence.
/// Scalar boundaries allow bound lexical forms without preselecting a word
/// split.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LexicalMatch {
    pub start: usize,
    pub end: usize,
    pub reading: LexicalReading,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzedText {
    pub raw: String,
    /// None means the raw input was analyzed directly.
    pub normalized: Option<String>,
    pub tokens: Vec<char>,
    pub matches: Vec<LexicalMatch>,
}

impl AnalyzedText {
    #[must_use]
    pub fn text(&self) -> &str {
        self.normalized.as_deref().unwrap_or(&self.raw)
    }

    /// Byte locations are derived only when a consumer requests them.
    #[must_use]
    pub fn byte_range(&self, start: usize, end: usize) -> Option<std::ops::Range<usize>> {
        if start > end || end > self.tokens.len() {
            return None;
        }
        let begin: usize = self.tokens[..start].iter().map(|c| c.len_utf8()).sum();
        let length: usize = self.tokens[start..end].iter().map(|c| c.len_utf8()).sum();
        Some(begin..begin + length)
    }

    /// Orthographic words are an inventory view, not a context-sensitive
    /// tokenizer.
    #[must_use]
    pub fn words(&self) -> Vec<std::ops::Range<usize>> {
        let mut result = Vec::new();
        let mut start = 0;
        while start < self.tokens.len() {
            if !word_character(self.tokens[start]) {
                start += 1;
                continue;
            }
            let mut end = start + 1;
            while end < self.tokens.len()
                && (word_character(self.tokens[end])
                    || matches!(self.tokens[end], '\'' | '’')
                        && end + 1 < self.tokens.len()
                        && word_character(self.tokens[end + 1]))
            {
                end += 1;
            }
            result.push(start..end);
            start = end;
        }
        result
    }

    #[must_use]
    pub fn unknown_words(&self) -> Vec<std::ops::Range<usize>> {
        self.words()
            .into_iter()
            .filter(|word| !self.is_covered(word.clone(), |_| true))
            .collect()
    }

    /// Whether adjacent lexical occurrences cover a word, allowing a containing
    /// multiword lexeme. Crossing overlaps alone do not constitute coverage.
    #[must_use]
    pub fn is_covered(
        &self,
        range: std::ops::Range<usize>,
        include: impl Fn(&LexicalReading) -> bool,
    ) -> bool {
        if range.start >= range.end || range.end > self.tokens.len() {
            return false;
        }
        let mut edges = vec![Vec::new(); range.len()];
        for found in &self.matches {
            if found.start < range.end && found.end > range.start && include(&found.reading) {
                let start = found.start.max(range.start) - range.start;
                let end = found.end.min(range.end) - range.start;
                edges[start].push(end);
            }
        }
        let mut reachable = vec![false; range.len() + 1];
        reachable[0] = true;
        for (start, targets) in edges.iter().enumerate() {
            if reachable[start] {
                for end in targets {
                    reachable[*end] = true;
                }
            }
        }
        reachable[range.len()]
    }
}

#[derive(Debug, Default)]
struct TrieNode {
    children: BTreeMap<char, usize>,
    values: Vec<(LexicalValue, Binding)>,
}

/// An immutable index built from declared forms and shared realization rules.
/// Loading and inspecting it needs no construction grammar or compiler.
#[derive(Debug)]
pub struct Lexicon {
    lexemes: BTreeMap<String, Lexeme>,
    surfaces: BTreeMap<LexicalValue, String>,
    trie: Vec<TrieNode>,
}

impl Lexicon {
    /// Validate declarations and freeze both analysis and realization indexes.
    ///
    /// # Errors
    /// Rejects duplicate identities/slots/variants, empty forms, and
    /// inapplicable features.
    pub fn new(lexemes: impl IntoIterator<Item = Lexeme>) -> Result<Self, LexicalError> {
        let mut result = Self {
            lexemes: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            trie: vec![TrieNode::default()],
        };
        for lexeme in lexemes {
            let error = |reason: &str| LexicalError::Declaration {
                lexeme: lexeme.id.clone(),
                reason: reason.to_owned(),
            };
            if lexeme.id.is_empty()
                || lexeme.lemma.trim().is_empty()
                || lexeme.forms.is_empty()
                || lexeme.source.owner.is_empty()
                || lexeme.source.path.is_empty()
            {
                return Err(error("identity, lemma, forms and source must be nonempty"));
            }
            if result.lexemes.contains_key(&lexeme.id) {
                return Err(error("duplicate identity"));
            }
            let mut slots = BTreeSet::new();
            for declaration in &lexeme.forms {
                if !slots.insert((declaration.form, &declaration.features)) {
                    return Err(error("duplicate inflection slot"));
                }
                if !morphology::applicable(lexeme.category, declaration.form, &declaration.features)
                {
                    return Err(error("inapplicable form or feature bundle"));
                }
                let surfaces = declaration.surfaces.clone().unwrap_or_else(|| {
                    vec![morphology::default_surface(
                        &lexeme,
                        declaration.form,
                        &declaration.features,
                    )]
                });
                if surfaces.is_empty() {
                    return Err(error(
                        "an unavailable slot must be omitted, not given an empty override",
                    ));
                }
                let mut distinct = BTreeSet::new();
                for (variant, surface) in surfaces.into_iter().enumerate() {
                    if surface.trim() != surface
                        || surface.is_empty()
                        || !distinct.insert(surface.clone())
                    {
                        return Err(error("empty, padded or duplicate spelling variant"));
                    }
                    let value = LexicalValue {
                        lexeme: lexeme.id.clone(),
                        form: declaration.form,
                        features: declaration.features.clone(),
                        variant,
                        capitalization: SurfaceCase::Declared,
                    };
                    result.insert(value.clone(), surface.clone(), lexeme.binding);
                    if lexeme.capitalization == Capitalization::Initial {
                        let initial = initial_surface(&surface);
                        if initial != surface {
                            result.insert(
                                LexicalValue {
                                    capitalization: SurfaceCase::Initial,
                                    ..value
                                },
                                initial,
                                lexeme.binding,
                            );
                        }
                    }
                }
            }
            result.lexemes.insert(lexeme.id.clone(), lexeme);
        }
        Ok(result)
    }

    fn insert(&mut self, value: LexicalValue, surface: String, binding: Binding) {
        let mut node = 0;
        for ch in surface.chars() {
            node = if let Some(next) = self.trie[node].children.get(&ch) {
                *next
            } else {
                let next = self.trie.len();
                self.trie.push(TrieNode::default());
                self.trie[node].children.insert(ch, next);
                next
            };
        }
        self.trie[node].values.push((value.clone(), binding));
        self.surfaces.insert(value, surface);
    }

    #[must_use]
    pub fn lexemes(&self) -> &BTreeMap<String, Lexeme> {
        &self.lexemes
    }

    /// Enumerates licensed values independently of any analyzed source.
    pub fn values(&self) -> impl Iterator<Item = &LexicalValue> {
        self.surfaces.keys()
    }

    /// # Errors
    /// Rejects values not admitted by this lexicon, including lossy numeral
    /// values.
    pub fn realize(&self, reading: &LexicalReading) -> Result<String, LexicalError> {
        match reading {
            LexicalReading::Word(value) => self
                .surfaces
                .get(value)
                .cloned()
                .ok_or(LexicalError::UnlicensedValue),
            LexicalReading::Numeral {
                value,
                notation,
                capitalization,
            } => {
                let text = notation
                    .try_format(*value)
                    .map_err(|_| LexicalError::UnlicensedValue)?;
                if *capitalization == SurfaceCase::Declared {
                    return Ok(text);
                }
                let initial = initial_surface(&text);
                if initial == text {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(initial)
            }
        }
    }

    /// Analyzes an italic run written at the head of an ability
    /// [CR#207.2c,207.2d].
    ///
    /// A run no declaration spells is not a vocabulary gap here: it is a
    /// flavor word, and the label is the run verbatim. Nothing enumerates
    /// flavor words, so this is the only open slot the lexicon reports rather
    /// than an unknown word.
    #[must_use]
    pub fn analyze_italic_head(&self, run: &str) -> ItalicHead {
        let analyzed = self.analyze(run);
        let whole = analyzed.tokens.len();
        let declared = analyzed.matches.iter().find_map(|found| {
            let LexicalReading::Word(value) = &found.reading else {
                return None;
            };
            let keyword = self
                .lexemes
                .get(&value.lexeme)
                .is_some_and(|lexeme| lexeme.category == Category::Keyword);
            (found.start == 0 && found.end == whole && keyword).then(|| value.clone())
        });
        match declared {
            Some(value) => ItalicHead::AbilityWord(value),
            None => ItalicHead::FlavorWord {
                label: run.to_owned(),
            },
        }
    }

    #[must_use]
    pub fn analyze(&self, text: &str) -> AnalyzedText {
        self.analyze_source(text, None)
    }

    #[must_use]
    pub fn analyze_source(&self, raw: &str, normalized: Option<&str>) -> AnalyzedText {
        let tokens: Vec<char> = normalized.unwrap_or(raw).chars().collect();
        let mut matches = Vec::new();
        for start in 0..tokens.len() {
            let mut node = 0;
            for end in start..tokens.len() {
                let Some(next) = self.trie[node].children.get(&tokens[end]) else {
                    break;
                };
                node = *next;
                for (value, binding) in &self.trie[node].values {
                    if edges_match(&tokens, start, end + 1, *binding) {
                        matches.push(LexicalMatch {
                            start,
                            end: end + 1,
                            reading: LexicalReading::Word(value.clone()),
                        });
                    }
                }
            }
        }
        numeral_matches(&tokens, &mut matches);
        AnalyzedText {
            raw: raw.to_owned(),
            normalized: normalized.map(str::to_owned),
            tokens,
            matches,
        }
    }
}

fn word_character(ch: char) -> bool {
    unicode_ident::is_xid_continue(ch)
}

fn edges_match(tokens: &[char], start: usize, end: usize, binding: Binding) -> bool {
    let left = start == 0 || !word_character(tokens[start - 1]) || !word_character(tokens[start]);
    let right =
        end == tokens.len() || !word_character(tokens[end - 1]) || !word_character(tokens[end]);
    (left || matches!(binding, Binding::Suffix | Binding::Bound))
        && (right || matches!(binding, Binding::Prefix | Binding::Bound))
}

fn initial_surface(surface: &str) -> String {
    let mut chars = surface.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

fn numeral_matches(tokens: &[char], matches: &mut Vec<LexicalMatch>) {
    for start in 0..tokens.len() {
        if tokens[start].is_whitespace() || !edges_match(tokens, start, start + 1, Binding::Prefix)
        {
            continue;
        }
        let mut end = start;
        let mut text = String::new();
        loop {
            let component_start = end;
            while end < tokens.len() && word_character(tokens[end]) {
                end += 1;
            }
            if component_start == end {
                if end == start && tokens[end] == '-' {
                    text.push('-');
                    end += 1;
                    continue;
                }
                break;
            }
            let component: String = tokens[component_start..end].iter().collect();
            let lower = component.to_lowercase();
            if !crate::numeral::numeral_component(&component)
                && !crate::numeral::numeral_component(&lower)
            {
                break;
            }
            text.push_str(&component);
            if edges_match(tokens, start, end, Binding::Free) {
                for notation in [
                    Numeral::Cardinal,
                    Numeral::Ordinal,
                    Numeral::Arabic(false),
                    Numeral::Arabic(true),
                    Numeral::Roman,
                ] {
                    for capitalization in [SurfaceCase::Declared, SurfaceCase::Initial] {
                        let candidate = if capitalization == SurfaceCase::Initial {
                            let mut chars = text.chars();
                            let Some(first) = chars.next() else {
                                continue;
                            };
                            first.to_lowercase().chain(chars).collect::<String>()
                        } else {
                            text.clone()
                        };
                        let Ok(value) = notation.parse(&candidate) else {
                            continue;
                        };
                        if capitalization == SurfaceCase::Initial
                            && (initial_surface(&candidate) != text || candidate == text)
                        {
                            continue;
                        }
                        matches.push(LexicalMatch {
                            start,
                            end,
                            reading: LexicalReading::Numeral {
                                value,
                                notation,
                                capitalization,
                            },
                        });
                    }
                }
            }
            if end == tokens.len() || !matches!(tokens[end], ' ' | '-' | ',') {
                break;
            }
            while end < tokens.len() && matches!(tokens[end], ' ' | '-' | ',') {
                text.push(tokens[end]);
                end += 1;
            }
            if end == tokens.len() {
                break;
            }
        }
    }
}
