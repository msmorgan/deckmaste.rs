//! The Syntactic Skeleton profiler: assigns each Corpus Unit its Complexity
//! Stratum (`docs/decisions/english-v4.md` §6.1).
//!
//! A unit is read as a sequence of lexical tokens, each the longest declared
//! analysis at its position. Its Syntactic Skeleton replaces every open-class
//! catalog item with a typed placeholder (`<Subtype>`, `<KeywordAction>`, ...).
//! Its Syntactic Families are the syntactic patterns it needs, each owned
//! by one Stratum; the unit's Stratum is the highest of them (the Max-Stratum
//! Composition Rule). Families are recognized from lexical categories, word
//! forms and declared Subcategorization Frames, never from catalog words; the
//! few core words that mark a family by themselves are named below.
//!
//! The profiler is a partition of the corpus for planning and gating, not a
//! grammar: a Stratum ticket that finds a unit misplaced corrects the family
//! rule here, and the unit moves with every other unit like it.

use std::collections::BTreeSet;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::Category;
use deckmaste_lexical::ItalicHead;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::WordForm;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_syntax::frames::ClauseKind;
use deckmaste_syntax::frames::Frame;
use deckmaste_syntax::frames::FrameFamily;
use deckmaste_syntax::frames::PrepositionFrame;
use deckmaste_syntax::frames::SlotFill;
use dpsi::Ident;

use crate::corpus::UnitKind;

/// A Complexity Stratum, `0..=7`.
pub type Stratum = u8;

/// The highest Stratum.
pub const MAX_STRATUM: Stratum = 7;

/// A Syntactic Family: a named syntactic pattern owned by one Stratum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, serde::Serialize)]
pub struct Family {
    pub stratum: Stratum,
    pub name: &'static str,
}

const fn family(stratum: Stratum, name: &'static str) -> Family {
    Family { stratum, name }
}

// Stratum 0: card frame, headers and non-clausal lines.
pub const TYPE_LINE: Family = family(0, "TypeLine");
pub const KEYWORD_LINE: Family = family(0, "KeywordLine");
pub const LABEL: Family = family(0, "LabelledAbility");
pub const KEYWORD_DASH: Family = family(0, "KeywordDashCost");
pub const MODAL_HEADER: Family = family(0, "ModalHeader");
pub const MODE_BULLET: Family = family(0, "ModeBullet");
pub const COSTED_MODE: Family = family(0, "CostedMode");
pub const CHAPTER_HEADER: Family = family(0, "ChapterHeader");
pub const LEVEL_HEADER: Family = family(0, "LevelHeader");
pub const TABLE_ROW: Family = family(0, "TableRow");
pub const NUMERAL_LINE: Family = family(0, "NumeralLine");
pub const ACTIVATED: Family = family(0, "ActivatedAbility");
pub const SYMBOL_COST: Family = family(0, "SymbolCost");
pub const SENTENCE_SEQUENCE: Family = family(0, "SentenceSequence");
pub const NOTATION: Family = family(0, "Notation");
// Stratum 1: core clauses and NP/PP structure.
pub const CORE_CLAUSE: Family = family(1, "CoreClauseOrPhrase");
// Stratum 2: auxiliaries, copulas, passives and non-finite complements.
pub const AUXILIARY: Family = family(2, "Auxiliary");
pub const NEGATION: Family = family(2, "Negation");
pub const PREDICATIVE: Family = family(2, "PredicativeComplement");
pub const INFINITIVAL: Family = family(2, "Infinitival");
pub const CATENATIVE: Family = family(2, "CatenativeComplement");
pub const GERUND_CLAUSE: Family = family(2, "GerundParticipialClause");
// Stratum 3: relative clauses and gaps.
pub const RELATIVE: Family = family(3, "RelativeClause");
pub const BARE_RELATIVE: Family = family(3, "BareRelative");
pub const PARTICIPIAL_POSTMODIFIER: Family = family(3, "ParticipialPostmodifier");
pub const ADJECTIVAL_POSTMODIFIER: Family = family(3, "AdjectivalPostmodifier");
// Stratum 4: subordinate clauses, triggers and conditionals.
pub const TRIGGER: Family = family(4, "TriggerClause");
pub const CLAUSE_PREPOSITION: Family = family(4, "PrepositionWithClause");
pub const CONTENT_CLAUSE: Family = family(4, "ContentClause");
// Stratum 5: comparatives, measures and partitives.
pub const PARTITIVE: Family = family(5, "Partitive");
pub const COMPARISON: Family = family(5, "Comparison");
pub const ARITHMETIC: Family = family(5, "Arithmetic");
// Stratum 6: coordination.
pub const COORDINATION: Family = family(6, "Coordination");
// Stratum 7: discourse, replacement and the long tail.
pub const REPLACEMENT: Family = family(7, "Replacement");
pub const SEQUENCING: Family = family(7, "DiscourseSequencing");
pub const PRO_FORM: Family = family(7, "ProFormDo");
pub const ELLIPSIS: Family = family(7, "Ellipsis");
pub const ABSOLUTE: Family = family(7, "AbsoluteClause");
pub const OPTIONAL_BRACKETED_TEXT: Family = family(7, "OptionalBracketedText");
pub const APPOSITION: Family = family(7, "Apposition");
pub const QUOTED_ABILITY: Family = family(7, "QuotedAbility");
pub const VOCABULARY_GAP: Family = family(7, "VocabularyGap");
pub const METALINGUISTIC: Family = family(7, "MetalinguisticMention");
pub const TYPE_LINE_WORD: Family = family(7, "TypeLineNonTypeWord");

/// Core words that mark a family by themselves: (category, lemma).
const COMPARATIVE_WORDS: &[&str] = &[
    "than", "equal", "greater", "less", "lesser", "fewer", "fewest", "more", "most", "least",
];
const ARITHMETIC_WORDS: &[&str] = &[
    "plus", "minus", "twice", "half", "double", "triple", "rounded", "round", "times",
];

/// The open-class catalog a word comes from, or `Core`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Class {
    Core,
    CardName,
    CardType,
    Subtype,
    CounterKind,
    KeywordAction,
    KeywordAbility,
    AbilityWord,
    Designation,
    TurnPart,
    Token,
    Number,
    Symbol,
    FlavorWord,
}

impl Class {
    fn of(reading: &LexicalReading) -> Self {
        match reading {
            LexicalReading::Numeral { .. } => Self::Number,
            LexicalReading::FlavorWord { .. } => Self::FlavorWord,
            LexicalReading::Word(value) => {
                let id = value.lexeme.as_str();
                let prefix = id.split(':').next().unwrap_or("");
                match prefix {
                    "name" | "introduced_name" | "anchor_name" => Self::CardName,
                    "type" | "supertype" => Self::CardType,
                    "subtype" => Self::Subtype,
                    "counter" => Self::CounterKind,
                    "action" | "action_adj" => Self::KeywordAction,
                    "keyword" | "keyword_verb" | "keyword_det" => Self::KeywordAbility,
                    "ability_word" => Self::AbilityWord,
                    "designation" => Self::Designation,
                    "turn_part" => Self::TurnPart,
                    "token" => Self::Token,
                    "symbol" => Self::Symbol,
                    _ => Self::Core,
                }
            }
        }
    }

    /// The Skeleton placeholder of a catalog item.
    fn placeholder(self) -> Option<&'static str> {
        Some(match self {
            Self::Core => return None,
            Self::CardName => "<CardName>",
            Self::CardType => "<CardType>",
            Self::Subtype => "<Subtype>",
            Self::CounterKind => "<CounterKind>",
            Self::KeywordAction => "<KeywordAction>",
            Self::KeywordAbility => "<KeywordAbility>",
            Self::AbilityWord => "<AbilityWord>",
            Self::Designation => "<Designation>",
            Self::TurnPart => "<TurnPart>",
            Self::Token => "<Token>",
            Self::Number => "<Number>",
            Self::Symbol => "<Symbol>",
            Self::FlavorWord => "<FlavorWord>",
        })
    }
}

/// One lexical alternative of a token.
#[derive(Debug, Clone)]
struct Alt<'e> {
    reading: LexicalReading,
    lexeme: Option<&'e Lexeme>,
    class: Class,
    /// Its declared Subcategorization Frames.
    frames: &'e [Frame],
    /// Its declared Complement frames, for a preposition.
    preposition_frames: &'e [PrepositionFrame],
}

/// One token of a unit.
#[derive(Debug, Clone)]
enum Tok<'e> {
    Word { text: String, alts: Vec<Alt<'e>> },
    Unknown(String),
    Punct(char),
}

impl Tok<'_> {
    fn alts(&self) -> &[Alt<'_>] {
        match self {
            Self::Word { alts, .. } => alts,
            _ => &[],
        }
    }

    fn is_punct(&self, c: char) -> bool {
        matches!(self, Self::Punct(p) if *p == c)
    }

    fn is_word(&self) -> bool {
        !matches!(self, Self::Punct(_))
    }

    /// Whether some alternative is a word of this category.
    fn has(&self, category: Category) -> bool {
        self.alts()
            .iter()
            .any(|a| a.lexeme.is_some_and(|l| l.category == category))
    }

    /// Whether some alternative is the core word `lemma` of `category`.
    fn is(&self, category: Category, lemma: &str) -> bool {
        self.alts().iter().any(|a| {
            a.class == Class::Core
                && a.lexeme
                    .is_some_and(|l| l.category == category && l.lemma.eq_ignore_ascii_case(lemma))
        })
    }

    /// Whether some alternative is a core word with one of these lemmas.
    fn lemma_in(&self, lemmas: &[&str]) -> bool {
        self.alts().iter().any(|a| {
            a.class == Class::Core
                && a.lexeme
                    .is_some_and(|l| lemmas.iter().any(|m| l.lemma.eq_ignore_ascii_case(m)))
        })
    }

    fn class_is(&self, class: Class) -> bool {
        self.alts().iter().any(|a| a.class == class)
    }

    fn only_class(&self, classes: &[Class]) -> bool {
        !self.alts().is_empty() && self.alts().iter().all(|a| classes.contains(&a.class))
    }

    /// Some alternative is a verb in one of these forms.
    fn verb_form(&self, forms: &[WordForm]) -> bool {
        self.alts().iter().any(|a| match &a.reading {
            LexicalReading::Word(v) => {
                a.lexeme.is_some_and(|l| l.category == Category::Verb) && forms.contains(&v.form)
            }
            _ => false,
        })
    }

    fn finite_verb(&self) -> bool {
        self.verb_form(&[WordForm::Present, WordForm::Preterite, WordForm::Irrealis])
    }

    /// Could head or end a nominal: a noun or a nominal catalog item.
    fn nominal(&self) -> bool {
        self.has(Category::Noun)
            || self.alts().iter().any(|a| {
                matches!(
                    a.class,
                    Class::CardType
                        | Class::Subtype
                        | Class::Token
                        | Class::CounterKind
                        | Class::CardName
                        | Class::Designation
                        | Class::TurnPart
                )
            })
    }

    fn numeral(&self) -> bool {
        self.class_is(Class::Number)
    }

    fn roman(&self) -> bool {
        self.alts().iter().any(|a| {
            matches!(
                a.reading,
                LexicalReading::Numeral {
                    notation: Numeral::Roman,
                    ..
                }
            )
        })
    }

    fn skeleton(&self) -> String {
        match self {
            Self::Punct(c) => c.to_string(),
            Self::Unknown(text) => format!("?{text}"),
            Self::Word { text, alts } => {
                if alts.iter().any(|a| a.class == Class::Core) {
                    text.to_lowercase()
                } else {
                    // A card name spelled like another catalog item (*Flash*)
                    // is named by that item.
                    alts.iter()
                        .filter_map(|a| {
                            a.class
                                .placeholder()
                                .map(|p| (a.class == Class::CardName, p))
                        })
                        .min()
                        .map_or("<?>", |(_, p)| p)
                        .to_owned()
                }
            }
        }
    }
}

/// What the profiler found in a unit.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Profile {
    pub stratum: Stratum,
    pub families: BTreeSet<Family>,
    pub skeleton: String,
    /// Words no declared lexeme spells.
    pub unknown_words: Vec<String>,
}

/// Profiles units against one lexical environment.
pub struct Profiler<'e> {
    env: &'e LexicalEnvironment,
}

impl<'e> Profiler<'e> {
    #[must_use]
    pub fn new(env: &'e LexicalEnvironment) -> Self {
        Self { env }
    }

    /// The Profile of one unit, given its lexical analysis.
    #[must_use]
    pub fn profile(&self, kind: UnitKind, analyzed: &AnalyzedText) -> Profile {
        let tokens = self.tokens(analyzed, kind);
        let mut families = BTreeSet::new();
        // A label's words are not analysed: a Flavor Word is in no inventory.
        let label = match kind {
            UnitKind::TypeLine => 0,
            UnitKind::Line => self.label_length(&tokens),
        };
        let unknown_words: Vec<String> = tokens[label..]
            .iter()
            .filter_map(|t| match t {
                Tok::Unknown(w) => Some(w.clone()),
                _ => None,
            })
            .collect();
        if !unknown_words.is_empty() {
            families.insert(VOCABULARY_GAP);
        }
        match kind {
            UnitKind::TypeLine => {
                families.insert(TYPE_LINE);
                if tokens.iter().any(|t| {
                    matches!(t, Tok::Word { .. })
                        && !t.class_is(Class::CardType)
                        && !t.class_is(Class::Subtype)
                }) {
                    families.insert(TYPE_LINE_WORD);
                }
            }
            UnitKind::Line => self.line(&tokens, &mut families),
        }
        let stratum = families.iter().map(|f| f.stratum).max().unwrap_or(0);
        let skeleton = tokens
            .iter()
            .map(Tok::skeleton)
            .collect::<Vec<_>>()
            .join(" ");
        Profile {
            stratum,
            families,
            skeleton,
            unknown_words,
        }
    }

    fn tokens(&self, analyzed: &AnalyzedText, kind: UnitKind) -> Vec<Tok<'e>> {
        let chars = &analyzed.tokens;
        let lexemes = self.env.lexicon.lexemes();
        let catalog = &self.env.frames;
        let mut out = Vec::new();
        let mut at = 0;
        while at < chars.len() {
            if chars[at].is_whitespace() {
                at += 1;
                continue;
            }
            // A Flavor Word reading spans its label and dash; the spine finds
            // labels itself. A type line names no card.
            let words = || {
                analyzed.matches.iter().filter(|m| {
                    !matches!(m.reading, LexicalReading::FlavorWord { .. })
                        && (kind != UnitKind::TypeLine || Class::of(&m.reading) != Class::CardName)
                })
            };
            let end = words().filter(|m| m.start == at).map(|m| m.end).max();
            if let Some(end) = end {
                let alts = words()
                    .filter(|m| m.start == at && m.end == end)
                    .map(|m| {
                        let id = match &m.reading {
                            LexicalReading::Word(v) => Some(&v.lexeme),
                            _ => None,
                        };
                        Alt {
                            class: Class::of(&m.reading),
                            lexeme: id.and_then(|id| lexemes.get(id)),
                            frames: id
                                .and_then(|id| catalog.frames.get(id))
                                .map_or(&[], Vec::as_slice),
                            preposition_frames: id
                                .and_then(|id| catalog.prepositions.get(id))
                                .map_or(&[], Vec::as_slice),
                            reading: m.reading.clone(),
                        }
                    })
                    .collect();
                out.push(Tok::Word {
                    text: chars[at..end].iter().collect(),
                    alts,
                });
                at = end;
            } else if chars[at].is_alphanumeric() {
                let mut end = at + 1;
                while end < chars.len()
                    && (chars[end].is_alphanumeric() || matches!(chars[end], '\'' | '’' | '-'))
                {
                    end += 1;
                }
                out.push(Tok::Unknown(chars[at..end].iter().collect()));
                at = end;
            } else {
                out.push(Tok::Punct(chars[at]));
                at += 1;
            }
        }
        out
    }

    /// The number of leading tokens forming a `<label> —` head, if any.
    fn label_length(&self, tokens: &[Tok<'e>]) -> usize {
        let mut start = usize::from(tokens.first().is_some_and(|t| t.is_punct('•')));
        loop {
            let Some(dash) = tokens[start..]
                .iter()
                .position(|t| t.is_punct('—'))
                .map(|d| start + d)
            else {
                return 0;
            };
            if dash + 1 == tokens.len() {
                return 0;
            }
            let head = &tokens[start..dash];
            if head.iter().all(|t| t.roman() || t.is_punct(',')) {
                // A chapter header, possibly before a label.
                start = dash + 1;
                continue;
            }
            if head.iter().all(|t| t.class_is(Class::CardType)) {
                // A type line (`Creature — Elf`) has no label.
                return 0;
            }
            let mut families = BTreeSet::new();
            self.spine(&tokens[start..], &mut families);
            return if families.contains(&LABEL) { dash } else { 0 };
        }
    }

    /// A rules-text line: its spine, then its payload.
    fn line(&self, tokens: &[Tok<'e>], families: &mut BTreeSet<Family>) {
        let mut tokens: Vec<Tok<'e>> = tokens.to_vec();
        if tokens.first().is_some_and(|t| t.is_punct('•')) {
            families.insert(MODE_BULLET);
            tokens.remove(0);
        }
        // A quoted ability is a whole nested Document; its own Stratum is not
        // looked into here.
        if tokens.iter().any(|t| t.is_punct('"') || t.is_punct('“')) {
            families.insert(QUOTED_ABILITY);
            tokens = without_quotations(&tokens);
        }
        if tokens
            .first()
            .is_some_and(|t| t.class_is(Class::Symbol) && t.skeleton() == "level")
            || tokens
                .first()
                .is_some_and(|t| matches!(t, Tok::Word { text, .. } if text == "LEVEL"))
        {
            families.insert(LEVEL_HEADER);
            return;
        }
        if !tokens.is_empty() && tokens.iter().all(|t| t.numeral() || !t.is_word()) {
            families.insert(NUMERAL_LINE);
            return;
        }
        if is_type_line(&tokens) {
            families.insert(TYPE_LINE);
            return;
        }
        let payload = self.spine(&tokens, families);
        if payload.is_empty() {
            return;
        }
        if keyword_line(&payload, families) {
            return;
        }
        if let Some(colon) = payload.iter().position(|t| t.is_punct(':')) {
            families.insert(ACTIVATED);
            let (cost, effect) = (&payload[..colon], &payload[colon + 1..]);
            if is_notation(cost) {
                families.insert(SYMBOL_COST);
            } else {
                clause(cost, true, families);
            }
            clause(effect, true, families);
            return;
        }
        clause(&payload, true, families);
    }

    /// Strips a `<label> —` spine and returns the rest.
    fn spine(&self, tokens: &[Tok<'e>], families: &mut BTreeSet<Family>) -> Vec<Tok<'e>> {
        if let Some(pipe) = tokens.iter().position(|t| t.is_punct('|'))
            && tokens[..pipe]
                .iter()
                .all(|t| t.numeral() || matches!(t, Tok::Punct('+' | '-' | '—')))
        {
            families.insert(TABLE_ROW);
            return self.spine(&tokens[pipe + 1..], families);
        }
        let Some(dash) = tokens.iter().position(|t| t.is_punct('—')) else {
            return tokens.to_vec();
        };
        let (head, rest) = (&tokens[..dash], &tokens[dash + 1..]);
        if rest.is_empty() {
            families.insert(MODAL_HEADER);
            let selection = if let Some(colon) = head.iter().position(|t| t.is_punct(':')) {
                families.insert(ACTIVATED);
                let cost = &head[..colon];
                if is_notation(cost) {
                    families.insert(SYMBOL_COST);
                } else {
                    clause(cost, true, families);
                }
                &head[colon + 1..]
            } else {
                head
            };
            if !modal_choice(selection) {
                clause(selection, true, families);
            }
            return Vec::new();
        }
        if head.iter().all(|t| t.roman() || t.is_punct(',')) {
            families.insert(CHAPTER_HEADER);
            // A chapter may be labelled: *III — Hall of Sorrow — ...*.
            return self.spine(rest, families);
        } else if is_notation(head) {
            families.insert(COSTED_MODE);
        } else {
            let text = head
                .iter()
                .map(|t| match t {
                    Tok::Word { text, .. } | Tok::Unknown(text) => text.clone(),
                    Tok::Punct(c) => c.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" ");
            match self.env.lexicon.analyze_italic_head(&text) {
                ItalicHead::Keyword(_) => families.insert(KEYWORD_DASH),
                ItalicHead::AbilityWord(_)
                | ItalicHead::AnchorWord(_)
                | ItalicHead::FlavorWord { .. } => families.insert(LABEL),
            };
        }
        rest.to_vec()
    }
}

/// A line of keyword abilities with their non-clausal parameters.
fn keyword_line<'e>(payload: &[Tok<'e>], families: &mut BTreeSet<Family>) -> bool {
    if payload.iter().any(|t| t.is_punct('.') || t.is_punct(':')) {
        return false;
    }
    let items: Vec<&[Tok<'e>]> = payload
        .split(|t| t.is_punct(','))
        .filter(|item| !item.is_empty())
        .collect();
    // A keyword, or a bound keyword after its stem (*Swamp*·*walk*).
    let keyword_at = |item: &[Tok<'_>]| {
        item.iter().enumerate().find_map(|(i, token)| {
            (token.class_is(Class::KeywordAbility)
                && token.has(Category::Keyword)
                && item[..i].iter().all(|t| {
                    t.class_is(Class::CardType)
                        || t.alts().iter().any(|a| {
                            a.lexeme
                                .is_some_and(|l| l.properties.classes.contains("Supertype"))
                        })
                        || t.class_is(Class::Subtype)
                        || t.has(Category::Affix)
                        || t.alts().iter().any(|a| {
                            a.lexeme
                                .is_some_and(|l| l.binding == deckmaste_lexical::Binding::Prefix)
                        })
                }))
            .then_some(i + 1)
        })
    };
    let is_keyword_line = !items.is_empty() && items.iter().all(|item| keyword_at(item).is_some());
    if is_keyword_line {
        families.insert(KEYWORD_LINE);
        for item in items {
            clause(&item[keyword_at(item).unwrap_or(1)..], false, families);
        }
    }
    is_keyword_line
}

/// Symbols, numerals, loyalty brackets and their punctuation only.
fn is_notation(tokens: &[Tok<'_>]) -> bool {
    !tokens.is_empty()
        && tokens.iter().all(|t| match t {
            Tok::Punct(c) => matches!(c, ',' | '[' | ']' | '+' | '-' | '−' | '/'),
            Tok::Word { .. } => t.only_class(&[Class::Symbol, Class::Number]),
            Tok::Unknown(_) => false,
        })
}

/// Supertypes and card types, optionally followed by `—` and subtypes.
fn is_type_line(tokens: &[Tok<'_>]) -> bool {
    let (types, subtypes) = match tokens.iter().position(|t| t.is_punct('—')) {
        Some(dash) => (&tokens[..dash], &tokens[dash + 1..]),
        None => (tokens, &[][..]),
    };
    !types.is_empty()
        && types.iter().all(|t| t.class_is(Class::CardType))
        && subtypes.iter().all(|t| t.class_is(Class::Subtype))
}

/// `Choose one`, `Choose one or more`, `Choose any number`: a modal header
/// with nothing but a number of modes.
fn modal_choice(head: &[Tok<'_>]) -> bool {
    head.first().is_some_and(|t| t.is(Category::Verb, "choose"))
        && head[1..].iter().all(|t| {
            !t.is_word()
                || t.numeral()
                || t.has(Category::Determinative)
                || t.has(Category::Coordinator)
                || t.is(Category::Noun, "number")
                || t.lemma_in(&["up", "to", "more"])
        })
}

/// Drops quoted spans (and their quotation marks).
fn without_quotations<'e>(tokens: &[Tok<'e>]) -> Vec<Tok<'e>> {
    let mut out = Vec::new();
    let mut quoted = false;
    for t in tokens {
        if t.is_punct('"') || t.is_punct('“') || t.is_punct('”') {
            quoted = !(t.is_punct('”') || quoted && t.is_punct('"'));
            continue;
        }
        if !quoted {
            out.push(t.clone());
        }
    }
    out
}

/// The families of clausal or phrasal material. `baseline` adds the core
/// clause family when there is any word (keyword parameters do not).
#[expect(clippy::too_many_lines, reason = "one rule table reads best whole")]
fn clause(tokens: &[Tok<'_>], baseline: bool, families: &mut BTreeSet<Family>) {
    let words: Vec<usize> = (0..tokens.len()).filter(|&i| tokens[i].is_word()).collect();
    if words.is_empty() {
        return;
    }
    if baseline {
        if tokens
            .iter()
            .all(|t| !t.is_word() || t.only_class(&[Class::Symbol, Class::Number]))
        {
            families.insert(NOTATION);
        } else {
            families.insert(CORE_CLAUSE);
        }
    }
    for (i, token) in tokens.iter().enumerate() {
        if token.is_punct('[')
            && tokens[i + 1..]
                .iter()
                .position(|t| t.is_punct(']'))
                .is_some_and(|end| !is_notation(&tokens[i + 1..i + 1 + end]))
        {
            families.insert(OPTIONAL_BRACKETED_TEXT);
        }
    }
    for window in tokens.windows(3) {
        if window[1].is_punct(',')
            && (window[0].has(Category::Adjective)
                || window[0].has(Category::Noun)
                || window[0].verb_form(&[WordForm::GerundParticiple, WordForm::PastParticiple]))
            && (window[2].has(Category::Adjective) || window[2].has(Category::Affix))
        {
            families.insert(COORDINATION);
        }
        if window[0].class_is(Class::CardName)
            && window[1].is_punct(',')
            && window[2].has(Category::Determinative)
        {
            families.insert(APPOSITION);
        }
    }
    if tokens
        .first()
        .is_some_and(|t| t.verb_form(&[WordForm::PastParticiple]))
        && tokens
            .get(1)
            .is_some_and(|t| t.is(Category::Preposition, "by"))
    {
        families.insert(AUXILIARY);
    }
    for (i, token) in tokens.iter().enumerate() {
        if token.is(Category::Preposition, "with")
            && tokens
                .get(i + 1)
                .is_some_and(|t| t.has(Category::Determinative) || t.has(Category::Pronoun))
            && tokens[i + 2..]
                .iter()
                .take_while(|t| !t.is_punct('.') && !t.is_punct(',') && !t.is_punct(';'))
                .filter(|t| t.is_word())
                .last()
                .is_some_and(|t| t.verb_form(&[WordForm::PastParticiple]))
        {
            families.insert(ABSOLUTE);
        }
        // *The same is true for ...* refers back to the previous sentence,
        // as *do the same* does (a pro-form, CGEL, Ch. 17 §7.6, p. 1533).
        if token.lemma_in(&["same"])
            && i > 0
            && tokens[i - 1].is(Category::Determinative, "the")
            && tokens.get(i + 1).is_some_and(|t| t.lemma_in(&["be"]))
            && tokens.get(i + 2).is_some_and(|t| t.lemma_in(&["true"]))
        {
            families.insert(PRO_FORM);
        }
        if token.is(Category::Adjective, "same")
            && tokens[i + 1..]
                .iter()
                .any(|t| t.is(Category::Preposition, "as"))
        {
            families.insert(COMPARISON);
        }
    }
    let sentences = tokens.iter().filter(|t| t.is_punct('.')).count();
    if sentences > 1 {
        families.insert(SENTENCE_SEQUENCE);
    }
    // Whether a clause with a finite verb follows `i` before the next clause
    // boundary: the verb comes after some subject, not straight after a
    // preposition or determinative.
    let finite_ahead = |i: usize| {
        (i + 2..tokens.len())
            .take_while(|&j| {
                !(tokens[j - 1].is_punct(',')
                    || tokens[j - 1].is_punct('.')
                    || tokens[j - 1].is_punct(';'))
            })
            .any(|j| {
                // A keyword used as a verb after a subject pronoun (*as you
                // cascade*) is a predicate too.
                let keyword_predicate =
                    tokens[j].has(Category::Keyword) && tokens[j - 1].has(Category::Pronoun);
                (tokens[j].finite_verb() || keyword_predicate)
                    && !tokens[j - 1].has(Category::Preposition)
                    && !tokens[j - 1].has(Category::Determinative)
                    && !tokens[j - 1].has(Category::Subordinator)
                    && !tokens[j - 1].has(Category::Adjective)
            })
    };
    let next = |i: usize| tokens.get(i + 1);
    let previous = |i: usize| i.checked_sub(1).map(|p| &tokens[p]);
    for (i, t) in tokens.iter().enumerate() {
        if !t.is_word() {
            continue;
        }
        // Stratum 2.
        // A contracted auxiliary spelled like the genitive clitic is left to
        // the genitive.
        // *'s* after a pronoun or relative is a contracted auxiliary (*it's*,
        // *that's*); after anything else it is the genitive.
        let contraction = t.has(Category::Clitic)
            && previous(i).is_some_and(|p| {
                p.has(Category::Pronoun) || p.lemma_in(&["that", "there", "who", "which"])
            });
        if t.has(Category::Clitic) && !contraction {
            continue;
        }
        let auxiliary = t.alts().iter().any(|a| {
            a.class == Class::Core
                && a.lexeme.is_some_and(|l| {
                    l.category == Category::Verb && l.source.path.ends_with("closed_class.ron")
                })
        });
        let bare_verb = |n: &Tok<'_>, form: WordForm| {
            n.verb_form(&[form]) && !n.has(Category::Noun) && !n.has(Category::Adjective)
        };
        // The pro-form *do so* is a verb of its own (CGEL, Ch. 17 §7.5,
        // pp. 1529–1532).
        if t.lemma_in(&["do so"]) {
            families.insert(PRO_FORM);
        }
        if auxiliary {
            // An auxiliary whose Verb Phrase is left unsaid: at the end of its
            // clause, or before the next clause's finite verb (*each opponent
            // who can't loses 3 life*).
            let stranded = next(i).is_none_or(|n| {
                n.is_punct(',')
                    || n.is_punct('.')
                    || n.verb_form(&[WordForm::Present, WordForm::Preterite])
                        && !n.verb_form(&[WordForm::Plain])
            });
            // Contracted *'s* may be *be* as well as *have*.
            if t.lemma_in(&["do", "have"]) && !t.lemma_in(&["be"]) {
                // Main-verb *do* and *have* are core clauses; the pro-form is
                // Stratum 7.
                // *do so* and *do the same* (CGEL, Ch. 17 §7.5, pp. 1529–1532;
                // §7.6, p. 1533).
                let same = next(i).is_some_and(|n| n.is(Category::Determinative, "the"))
                    && tokens.get(i + 2).is_some_and(|n| n.lemma_in(&["same"]));
                if t.lemma_in(&["do"]) && (stranded || same) {
                    families.insert(PRO_FORM);
                }
                if next(i).is_some_and(|n| {
                    bare_verb(n, WordForm::PastParticiple)
                        || t.lemma_in(&["do"]) && bare_verb(n, WordForm::Plain)
                        || n.is(Category::Adverb, "not")
                }) {
                    families.insert(AUXILIARY);
                }
            } else {
                families.insert(AUXILIARY);
                if stranded && !t.lemma_in(&["be"]) {
                    families.insert(ELLIPSIS);
                }
            }
            if t.alts().iter().any(|a| {
                a.lexeme.is_some_and(|l| {
                    l.properties.features.get("Polarity").map(Ident::as_str) == Some("Negative")
                })
            }) {
                families.insert(NEGATION);
            }
        }
        if t.is(Category::Adverb, "not") || t.is(Category::Adverb, "never") {
            families.insert(NEGATION);
        }
        // *to* before a plain verb, unless that verb is a premodifier or the
        // Targeting Marker of a following nominal (*to target creature*).
        if t.is(Category::Subordinator, "to")
            && next(i).is_some_and(|n| n.verb_form(&[WordForm::Plain]))
            && !(next(i)
                .is_some_and(|n| n.has(Category::Adjective) || n.has(Category::Determinative))
                && tokens.get(i + 2).is_some_and(Tok::nominal))
        {
            families.insert(INFINITIVAL);
        }
        // A noun-or-verb word after a determinative, preposition or adjective
        // is the noun (*until end of turn*, *your next turn*).
        let nominal_context = t.has(Category::Noun)
            && previous(i).is_some_and(|p| {
                p.has(Category::Determinative)
                    || p.has(Category::Preposition)
                    || p.has(Category::Adjective)
                    || p.has(Category::Clitic)
            });
        let verb_frames = |family: &[FrameFamily]| {
            !nominal_context
                && t.alts().iter().any(|a| {
                    a.lexeme.is_some_and(|l| l.category == Category::Verb)
                        && a.frames.iter().any(|f| family.contains(&f.family))
                })
        };
        if !auxiliary
            && verb_frames(&[
                FrameFamily::Copular,
                FrameFamily::ComplexTransitive,
                FrameFamily::Idiom,
            ])
        {
            families.insert(PREDICATIVE);
        }
        // A bare-infinitival or gerund-participial complement; *to*
        // infinitivals are found by *to*.
        let bare_complement = t.alts().iter().any(|a| {
            a.lexeme.is_some_and(|l| l.category == Category::Verb)
                && a.frames.iter().any(|f| {
                    matches!(
                        f.family,
                        FrameFamily::Catenative | FrameFamily::ComplexCatenative
                    ) && f
                        .slots
                        .iter()
                        .any(|s| matches!(s.fill, SlotFill::BareInf | SlotFill::Gerund))
                })
        });
        if !nominal_context
            && bare_complement
            && (i + 1..tokens.len())
                .take_while(|&j| !(tokens[j].is_punct(',') || tokens[j].is_punct('.')))
                .any(|j| {
                    (bare_verb(&tokens[j], WordForm::Plain)
                        || bare_verb(&tokens[j], WordForm::GerundParticiple))
                        && !tokens[j - 1].is(Category::Subordinator, "to")
                })
        {
            families.insert(CATENATIVE);
        }
        if bare_verb(t, WordForm::GerundParticiple)
            && previous(i).is_some_and(|p| p.has(Category::Preposition) && !p.has(Category::Verb))
            && next(i).is_none_or(|n| !n.nominal() || n.has(Category::Determinative))
        {
            families.insert(GERUND_CLAUSE);
        }
        // Stratum 3.
        // A relative word after its antecedent: a nominal, keyword, numeral
        // or symbol (*mana value X whose controller ...*).
        let antecedent = |p: &Tok<'_>| {
            p.nominal() || p.has(Category::Keyword) || p.numeral() || p.class_is(Class::Symbol)
        };
        // A relative pronoun, after its antecedent or fronted with a
        // preposition (*each permanent for which*); a fused relative
        // (*whichever comes first*).
        if t.has(Category::Pronoun)
            && t.lemma_in(&["who", "which"])
            && previous(i).is_some_and(|p| {
                antecedent(p)
                    || p.is_punct(',')
                    || p.has(Category::Preposition)
                        && i.checked_sub(2)
                            .and_then(|j| tokens.get(j))
                            .is_some_and(antecedent)
            })
            || t.is(Category::Determinative, "whichever")
        {
            families.insert(RELATIVE);
        }
        // An adverbial relative (*the next time a source would deal damage*).
        if t.is(Category::Noun, "time")
            && previous(i).is_some_and(|p| p.lemma_in(&["next", "first", "last"]))
            && next(i).is_some_and(|n| n.has(Category::Determinative) || n.has(Category::Pronoun))
            && finite_ahead(i)
        {
            families.insert(RELATIVE);
        }
        if t.is(Category::Subordinator, "that") {
            let after = next(i);
            if (previous(i).is_some_and(|p| antecedent(p) || p.has(Category::Adjective))
                || (i == 0
                    && (finite_ahead(i)
                        || (after.is_some_and(|n| n.is(Category::Determinative, "each"))
                            && tokens.get(i + 2).is_some_and(Tok::finite_verb)))))
                && after.is_some_and(|n| {
                    n.finite_verb()
                        || n.has(Category::Pronoun)
                        || n.has(Category::Determinative)
                        || n.nominal() && finite_ahead(i)
                })
            {
                families.insert(RELATIVE);
            } else if previous(i).is_some_and(|p| p.has(Category::Verb))
                && after.is_some_and(|n| {
                    !n.nominal()
                        && (n.has(Category::Pronoun)
                            || n.has(Category::Determinative)
                                && !n.lemma_in(&["much", "many", "more"]))
                })
                && finite_ahead(i)
            {
                families.insert(CONTENT_CLAUSE);
            }
        }
        // A keyword ending a preposition's complement, or a symbol, may host
        // a postmodifier (*without flanking blocking it*, *{G}{G} spent to
        // cast it*).
        let host = t.nominal()
            || t.has(Category::Keyword)
                && previous(i).is_some_and(|p| p.has(Category::Preposition))
            || t.class_is(Class::Symbol) && previous(i).is_some_and(|p| p.class_is(Class::Symbol));
        // A premodifier or the Targeting Marker is no host (*target attacking
        // creature*).
        if host && !t.has(Category::Adjective) && !t.has(Category::Determinative) {
            // A bare relative: nominal, subject, finite verb.
            if let Some(subject) = next(i) {
                let verb_at = if subject.has(Category::Pronoun)
                    && tokens.get(i + 2).is_some_and(Tok::finite_verb)
                {
                    Some(i + 2)
                } else if subject.has(Category::Determinative)
                    || subject.has(Category::Pronoun)
                    || subject.has(Category::Adjective)
                {
                    (i + 2..tokens.len())
                        .find(|&j| !tokens[j].has(Category::Adjective))
                        .filter(|&j| tokens[j].nominal())
                        .map(|j| j + 1)
                } else {
                    None
                };
                if verb_at
                    .and_then(|j| tokens.get(j))
                    .is_some_and(Tok::finite_verb)
                    && !subject.lemma_in(&["who", "which", "that"])
                {
                    families.insert(BARE_RELATIVE);
                }
            }
            // A finite homograph inside a nominal (e.g. `this turn`) is
            // not evidence for a later matrix predicate.
            let predicate_at = |j: usize| {
                tokens[j].finite_verb()
                    && !tokens[j].has(Category::Adjective)
                    && !tokens[j].has(Category::Determinative)
                    && j.checked_sub(1).is_none_or(|k| {
                        !tokens[k].has(Category::Determinative)
                            && !tokens[k].has(Category::Clitic)
                            && (!tokens[k].finite_verb() || tokens[k].nominal())
                            && !tokens[k].alts().iter().any(|a| matches!(&a.reading, LexicalReading::Word(v) if v.features.case == Some(deckmaste_lexical::Case::Genitive)))
                            && !tokens[k].has(Category::Preposition)
                            && !tokens[k].has(Category::Adjective)
                    })
            };
            let later_predicate = (i + 2..tokens.len())
                .take_while(|&j| !tokens[j].is_punct('.') && !tokens[j].is_punct(';'))
                .any(predicate_at);
            let governing_prefix = tokens.first().is_some_and(|p| p.has(Category::Preposition))
                || (0..i).any(|j| predicate_at(j) && !tokens[j].has(Category::Adjective));
            if next(i).is_some_and(|n| {
                let participle =
                    n.verb_form(&[WordForm::PastParticiple, WordForm::GerundParticiple]);
                participle
                    && (!n.has(Category::Adjective) || later_predicate)
                    && (!n.verb_form(&[WordForm::Present])
                        || later_predicate
                        || previous(i).is_some_and(|p| p.nominal() || p.has(Category::Adjective)))
            }) && (later_predicate
                || governing_prefix
                    && tokens
                        .get(i + 2)
                        .is_some_and(|n| !(n.is_punct('.') || n.is_punct(',')))
                || !tokens.iter().any(|p| p.is_punct('.')))
            {
                families.insert(PARTICIPIAL_POSTMODIFIER);
            }
        }
        // An adjective with its complement after a nominal (*creatures able
        // to block*).
        if t.has(Category::Adjective)
            && previous(i).is_some_and(Tok::nominal)
            && next(i).is_some_and(|n| n.is(Category::Subordinator, "to"))
        {
            families.insert(ADJECTIVAL_POSTMODIFIER);
        }
        // Stratum 4.
        let clausal = |a: &Alt<'_>| {
            a.preposition_frames.iter().map(|f| &f.complement).fold(
                (false, true),
                |(any, all), fill| {
                    let c = matches!(fill, SlotFill::Clause(_));
                    (any || c, all && c)
                },
            )
        };
        if t.has(Category::Preposition) {
            let (any, all) = t
                .alts()
                .iter()
                .filter(|a| {
                    a.lexeme
                        .is_some_and(|l| l.category == Category::Preposition)
                })
                .map(clausal)
                .fold((false, false), |(x, y), (a, b)| (x || a, y || b && a));
            // A bare-infinitival complement is a clause too (*rather than
            // pay*).
            let bare_infinitival = t.alts().iter().any(|a| {
                a.preposition_frames
                    .iter()
                    .any(|f| f.complement == SlotFill::BareInf)
            }) && next(i).is_some_and(|n| bare_verb(n, WordForm::Plain));
            let verbless = t.alts().iter().any(|a| {
                a.preposition_frames
                    .iter()
                    .any(|f| f.complement == SlotFill::Clause(ClauseKind::Verbless))
            }) && next(i).is_some_and(|n| {
                n.has(Category::Adjective) || n.verb_form(&[WordForm::PastParticiple])
            });
            if bare_infinitival || verbless || any && (all || finite_ahead(i)) {
                if t.lemma_in(&["when", "whenever"]) {
                    families.insert(TRIGGER);
                } else {
                    families.insert(CLAUSE_PREPOSITION);
                }
            }
        }
        if (i == words[0] || previous(i).is_some_and(|t| t.is_punct('.')))
            && t.is(Category::Preposition, "at")
            && tokens[i + 1..].iter().any(|n| n.is_punct(','))
            && tokens[i + 1..].iter().any(|n| {
                n.is(Category::Noun, "beginning")
                    || n.is(Category::Noun, "end")
                    || n.class_is(Class::TurnPart)
            })
        {
            families.insert(TRIGGER);
        }
        if t.is(Category::Subordinator, "whether")
            || (t.lemma_in(&["how", "which"]) && previous(i).is_some_and(|p| p.has(Category::Verb)))
        {
            families.insert(CONTENT_CLAUSE);
        }
        // Stratum 5.
        if (t.has(Category::Determinative) || t.numeral() || t.is(Category::Noun, "number"))
            && next(i).is_some_and(|n| n.is(Category::Preposition, "of"))
        {
            families.insert(PARTITIVE);
        }
        if t.lemma_in(COMPARATIVE_WORDS)
            || (t.is(Category::Preposition, "up")
                && next(i).is_some_and(|n| n.is(Category::Preposition, "to")))
            || (t.is(Category::Preposition, "at")
                && next(i).is_some_and(|n| n.lemma_in(&["least", "most"])))
        {
            families.insert(COMPARISON);
        }
        if t.lemma_in(ARITHMETIC_WORDS)
            || (t.is(Category::Noun, "time")
                && previous(i).is_some_and(Tok::numeral)
                && next(i).is_some_and(|t| t.numeral() || t.class_is(Class::Symbol)))
        {
            families.insert(ARITHMETIC);
        }
        // Stratum 6.
        if t.has(Category::Coordinator) {
            families.insert(COORDINATION);
        }
        // An elliptical *but* coordinate (*but not more than ...*, *but only
        // during their turn*) is Ellipsis, Stratum 7.
        if t.is(Category::Coordinator, "but")
            && next(i).is_some_and(|n| n.lemma_in(&["not", "only"]))
        {
            families.insert(ELLIPSIS);
        }
        // Stratum 7.
        if t.lemma_in(&["instead"]) {
            families.insert(REPLACEMENT);
        }
        if t.is(Category::Adverb, "then") || t.is(Category::Adverb, "otherwise") {
            families.insert(SEQUENCING);
        }
        if t.is(Category::Verb, "repeat") {
            families.insert(SEQUENCING);
        }
        if matches!(t, Tok::Unknown(_)) {
            families.insert(VOCABULARY_GAP);
        }
        // A bare singular subtype ending a preposition's complement names the
        // word itself (*replacing ... with Vampire*, *other than Wall*; CGEL,
        // Ch. 5 §8.2, p. 401), unless it may be a name or a plural.
        let word_form = |form: WordForm| {
            t.alts().iter().any(|a| {
                a.class == Class::Subtype
                    && matches!(&a.reading, LexicalReading::Word(v) if v.form == form)
            })
        };
        // A vote names its options; a listed one that is no noun (*votes for
        // planeswalk or chaos*) is the word itself.
        if t.lemma_in(&["vote"])
            && next(i).is_some_and(|n| n.is(Category::Preposition, "for"))
            && tokens.get(i + 2).is_some_and(|o| {
                o.is_word()
                    && !o.has(Category::Noun)
                    && !o.has(Category::Determinative)
                    && !o.has(Category::Pronoun)
                    && !o.has(Category::Adjective)
            })
            && tokens
                .get(i + 3)
                .is_some_and(|c| c.has(Category::Coordinator))
        {
            families.insert(METALINGUISTIC);
        }
        // Text-changing effects replace instances of words (*replacing all
        // instances of one color word with another*).
        if t.is(Category::Noun, "instance") && tokens[..i].iter().any(|w| w.lemma_in(&["replace"]))
        {
            families.insert(METALINGUISTIC);
        }
        if previous(i).is_some_and(|p| p.has(Category::Preposition))
            && next(i).is_some_and(|n| n.is_punct('.'))
            && word_form(WordForm::Singular)
            && !word_form(WordForm::Plural)
            && !t.class_is(Class::CardName)
        {
            families.insert(METALINGUISTIC);
        }
    }
}
