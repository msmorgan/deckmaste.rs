//! The judgment set: hand-written linguistic judgments, one line per entry
//! (`judgments/README.md`), and their evaluation against a bound grammar.

use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

use deckmaste_lexical::LexemeId;
use deckmaste_lexical::Lexicon;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::Constituent;
use deckmaste_syntax::Gap;
use deckmaste_syntax::Parse;
use deckmaste_syntax::Reading;

use crate::corpus::card_name_lexeme;

/// More Readings than this and an entry is not evaluated tree by tree.
pub const READING_LIMIT: u128 = 4096;

/// A grammatical function, as CGEL names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    Subject,
    Object,
    IndirectObject,
    Complement,
    PredicativeComplement,
    Adjunct,
    Modifier,
    Determiner,
}

impl Function {
    const ALL: [Self; 8] = [
        Self::Subject,
        Self::Object,
        Self::IndirectObject,
        Self::Complement,
        Self::PredicativeComplement,
        Self::Adjunct,
        Self::Modifier,
        Self::Determiner,
    ];

    /// The child name the grammar and the syntax engine give a dependent in
    /// this function.
    #[must_use]
    pub fn child_name(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Object => "object",
            Self::IndirectObject => "indirect_object",
            Self::Complement => "complement",
            Self::PredicativeComplement => "predicative_complement",
            Self::Adjunct => "adjunct",
            Self::Modifier => "modifier",
            Self::Determiner => "determiner",
        }
    }

    fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| format!("{f:?}") == name)
    }
}

/// One assertion about a Constituent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    pub category: String,
    pub surface: String,
    /// The 1-based occurrence of `surface` in the text, or any occurrence.
    pub occurrence: Option<usize>,
    /// The lemma or lexeme id of its head word.
    pub head: Option<String>,
    /// The function it bears, and the lemma or lexeme id heading the phrase
    /// it bears it in.
    pub function: Option<(Function, String)>,
    /// The surface of the Antecedent of its Gap.
    pub gap: Option<String>,
}

/// One judgment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgment {
    /// `file:line`, for reports.
    pub location: String,
    /// `+` (true) or `*` (false).
    pub grammatical: bool,
    /// The Category to parse as, or the grammar's root.
    pub category: Option<String>,
    pub card: Option<String>,
    pub text: String,
    /// Alternative groups; each must be satisfied by some one Reading.
    pub groups: Vec<Vec<Assertion>>,
    /// Assertions no Reading may satisfy.
    pub negated: Vec<Assertion>,
    pub readings: Option<usize>,
}

/// A malformed entry.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{location}: {message}")]
pub struct SyntaxError {
    pub location: String,
    pub message: String,
}

/// Parses one judgment file. Blank lines and `#` comments are skipped.
///
/// # Errors
/// The first malformed entry.
pub fn parse_file(name: &str, text: &str) -> Result<Vec<Judgment>, SyntaxError> {
    let mut entries = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let location = format!("{name}:{}", number + 1);
        let entry = parse_entry(line, &location).map_err(|message| SyntaxError {
            location: location.clone(),
            message,
        })?;
        entries.push(entry);
    }
    Ok(entries)
}

/// Every `*.txt` judgment file in a directory, by file name.
///
/// # Errors
/// An unreadable directory or file, or a malformed entry.
pub fn load_dir(dir: &Path) -> Result<Vec<(String, Vec<Judgment>)>, SyntaxError> {
    let io = |e: std::io::Error| SyntaxError {
        location: dir.display().to_string(),
        message: e.to_string(),
    };
    let mut names: Vec<_> = std::fs::read_dir(dir)
        .map_err(io)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let text = std::fs::read_to_string(&path).map_err(io)?;
            Ok((name.clone(), parse_file(&name, &text)?))
        })
        .collect()
}

fn parse_entry(line: &str, location: &str) -> Result<Judgment, String> {
    let mut chars = line.chars();
    let grammatical = match chars.next() {
        Some('+') => true,
        Some('*') => false,
        _ => return Err("an entry starts with `+` or `*`".to_owned()),
    };
    let rest = chars.as_str();
    let category_end = rest
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(rest.len());
    let category = (category_end > 0).then(|| rest[..category_end].to_owned());
    let mut rest = rest[category_end..].trim_start();
    let mut card = None;
    if let Some(after) = rest.strip_prefix("<card=") {
        let end = after.find('>').ok_or("unclosed `<card=`")?;
        card = Some(after[..end].to_owned());
        rest = after[end + 1..].trim_start();
    }
    let (text, assertions) = match rest.find(" => ") {
        Some(at) => (rest[..at].trim(), Some(rest[at + 4..].trim())),
        None => (rest.trim_end_matches("=>").trim(), None),
    };
    if text.is_empty() {
        return Err("no text".to_owned());
    }
    let mut entry = Judgment {
        location: location.to_owned(),
        grammatical,
        category,
        card,
        text: text.to_owned(),
        groups: Vec::new(),
        negated: Vec::new(),
        readings: None,
    };
    let Some(assertions) = assertions else {
        return Ok(entry);
    };
    if !grammatical {
        return Err("an ungrammatical entry takes no assertions".to_owned());
    }
    for group in split_outside_quotes(assertions, "||") {
        let mut positive = Vec::new();
        for item in split_outside_quotes(&group, ";") {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            if let Some(count) = item.strip_prefix("readings ") {
                let count = count
                    .trim()
                    .parse()
                    .map_err(|_| format!("`readings` needs a number: {item}"))?;
                if entry.readings.replace(count).is_some_and(|c| c != count) {
                    return Err("two different `readings` counts".to_owned());
                }
            } else if let Some(negated) = item.strip_prefix('!') {
                entry.negated.push(parse_assertion(negated)?);
            } else {
                positive.push(parse_assertion(item)?);
            }
        }
        if !positive.is_empty() {
            entry.groups.push(positive);
        }
    }
    Ok(entry)
}

/// Splits on `separator` outside double-quoted surfaces (`\"` escapes a
/// quote inside one).
fn split_outside_quotes(text: &str, separator: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut quoted = false;
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        if c == '\\' && quoted && rest[1..].starts_with('"') {
            parts.last_mut().unwrap().push_str("\\\"");
            rest = &rest[2..];
            continue;
        }
        if c == '"' {
            quoted = !quoted;
        }
        if !quoted && rest.starts_with(separator) {
            parts.push(String::new());
            rest = &rest[separator.len()..];
            continue;
        }
        parts.last_mut().unwrap().push(c);
        rest = &rest[c.len_utf8()..];
    }
    parts
}

/// Splits the clauses after a surface into words; a quoted string (a
/// surface, or a multiword lemma such as `"mana value"`) is one word.
fn words(text: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut rest = text.trim_start();
    while !rest.is_empty() {
        if rest.starts_with('"') {
            let (word, after) = quoted(rest)?;
            out.push(word);
            rest = after.trim_start();
        } else {
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            out.push(rest[..end].to_owned());
            rest = rest[end..].trim_start();
        }
    }
    Ok(out)
}

/// Reads a quoted surface at the start of `text`; returns it and the rest.
fn quoted(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let body = text
        .strip_prefix('"')
        .ok_or_else(|| format!("expected a quoted surface: {text}"))?;
    let mut surface = String::new();
    let mut chars = body.char_indices();
    while let Some((at, c)) = chars.next() {
        match c {
            '\\' if body[at + 1..].starts_with('"') => {
                surface.push('"');
                chars.next();
            }
            '"' => return Ok((surface, &body[at + 1..])),
            _ => surface.push(c),
        }
    }
    Err(format!("unclosed surface: {text}"))
}

fn parse_assertion(text: &str) -> Result<Assertion, String> {
    let text = text.trim();
    let category_end = text
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(text.len());
    if category_end == 0 {
        return Err(format!("an assertion starts with a Category: {text}"));
    }
    let category = text[..category_end].to_owned();
    let (surface, mut rest) = quoted(&text[category_end..])?;
    let mut assertion = Assertion {
        category,
        surface,
        occurrence: None,
        head: None,
        function: None,
        gap: None,
    };
    if let Some(after) = rest.strip_prefix('#') {
        let end = after
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(after.len());
        assertion.occurrence = Some(
            after[..end]
                .parse()
                .map_err(|_| format!("bad occurrence: {text}"))?,
        );
        rest = &after[end..];
    }
    let mut words = words(rest)?.into_iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "head" => assertion.head = Some(words.next().ok_or("`head` needs a lemma")?),
            "as" => {
                let function = words.next().ok_or("`as` needs a function")?;
                let function = Function::parse(&function)
                    .ok_or_else(|| format!("unknown function `{function}`"))?;
                if words.next().as_deref() != Some("of") {
                    return Err(format!("expected `as FUNCTION of LEMMA`: {text}"));
                }
                assertion.function = Some((function, words.next().ok_or("`of` needs a lemma")?));
            }
            "gap" => assertion.gap = Some(words.next().ok_or("`gap` needs a surface")?),
            other => return Err(format!("unexpected `{other}` in: {text}")),
        }
    }
    Ok(assertion)
}

/// The card names an entry puts in scope.
#[must_use]
pub fn scope(entry: &Judgment) -> BTreeSet<LexemeId> {
    entry.card.iter().map(|c| card_name_lexeme(c)).collect()
}

/// Round-trip law 2 for one Reading of `parse`: admitted, realized, and
/// found again in the parse of its realization, with the same names in
/// scope.
///
/// # Errors
/// Which step failed.
pub fn law_two(
    grammar: &CompiledGrammar,
    lexicon: &Lexicon,
    parse: &Parse<'_>,
    reading: &Reading,
    scope: &BTreeSet<LexemeId>,
) -> Result<(), String> {
    parse
        .admit(reading, lexicon)
        .map_err(|e| format!("admission: {e}"))?;
    let text = grammar
        .realize(reading, lexicon)
        .map_err(|e| format!("realization: {e}"))?;
    let analyzed = lexicon.analyze_source_scoped(&text, None, scope);
    if grammar
        .parse_category(&analyzed, parse.category())
        .is_some_and(|again| again.contains(reading))
    {
        Ok(())
    } else {
        Err(format!(
            "round-trip law 2: a Reading of {text:?} does not reparse"
        ))
    }
}

/// What evaluating an entry found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Holds,
    Fails(String),
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Holds => f.write_str("holds"),
            Self::Fails(why) => f.write_str(why),
        }
    }
}

/// One Reading, located: its text and Constituents.
pub struct Located {
    pub text: String,
    pub constituents: Vec<Constituent>,
}

/// Evaluates entries against one bound grammar and its lexicon.
pub struct Judge<'g> {
    pub grammar: &'g CompiledGrammar,
    pub lexicon: &'g Lexicon,
}

impl Judge<'_> {
    /// Parses an entry's text as it asks.
    #[must_use]
    pub fn parse(&self, entry: &Judgment) -> Option<Parse<'_>> {
        let analyzed = self.lexicon.analyze_source_scoped(
            &deckmaste_data::oracle_text::strip_reminder_text(&entry.text),
            None,
            &scope(entry),
        );
        match &entry.category {
            Some(category) => self.grammar.parse_category(&analyzed, category),
            None => Some(self.grammar.parse_analyzed(&analyzed)),
        }
    }

    /// Whether the entry holds, with both round-trip laws checked on every
    /// Reading.
    #[must_use]
    pub fn evaluate(&self, entry: &Judgment) -> Outcome {
        let Some(parse) = self.parse(entry) else {
            return Outcome::Fails(format!(
                "the grammar has no Category `{}`",
                entry.category.as_deref().unwrap_or("")
            ));
        };
        let count = parse.derivation_count();
        // An undeclared word (an off-card Shortened Name, a misspelled type)
        // leaves no Reading, which is what an ungrammatical entry claims.
        if !parse.vocabulary_gaps.is_empty() {
            return if entry.grammatical {
                Outcome::Fails(format!("vocabulary gaps at {:?}", parse.vocabulary_gaps))
            } else {
                Outcome::Holds
            };
        }
        if !entry.grammatical {
            return if count == 0 {
                Outcome::Holds
            } else {
                Outcome::Fails(format!("{count} derivations of an ungrammatical text"))
            };
        }
        if count == 0 {
            return Outcome::Fails("no Reading".to_owned());
        }
        if count > READING_LIMIT {
            return Outcome::Fails(format!("{count} derivations, over the reading limit"));
        }
        if let Err(fault) = parse.verify_realization(self.lexicon) {
            return Outcome::Fails(format!("round-trip law 1: {fault}"));
        }
        let readings = parse.readings();
        let mut located = Vec::with_capacity(readings.len());
        for reading in &readings {
            if let Err(why) = law_two(self.grammar, self.lexicon, &parse, reading, &scope(entry)) {
                return Outcome::Fails(why);
            }
            match self.grammar.constituents(reading, self.lexicon) {
                Ok((text, constituents)) => located.push(Located { text, constituents }),
                Err(e) => return Outcome::Fails(format!("realization: {e}")),
            }
        }
        if let Some(n) = entry.readings
            && n != readings.len()
        {
            return Outcome::Fails(format!("{} Readings, not {n}", readings.len()));
        }
        for group in &entry.groups {
            if !located
                .iter()
                .any(|r| group.iter().all(|a| self.holds(a, r)))
            {
                return Outcome::Fails(format!("no Reading satisfies {group:?}"));
            }
        }
        for assertion in &entry.negated {
            if located.iter().any(|r| self.holds(assertion, r)) {
                return Outcome::Fails(format!("a Reading satisfies !{assertion:?}"));
            }
        }
        Outcome::Holds
    }

    /// Whether some entry Reading is endorsed: it satisfies some group (or
    /// the entry has none) and no negated assertion.
    #[must_use]
    pub fn endorses(&self, entry: &Judgment, reading: &Located) -> bool {
        (entry.groups.is_empty()
            || entry
                .groups
                .iter()
                .any(|g| g.iter().all(|a| self.holds(a, reading))))
            && !entry.negated.iter().any(|a| self.holds(a, reading))
    }

    /// Whether some Constituent of the Reading satisfies the assertion.
    #[must_use]
    pub fn holds(&self, assertion: &Assertion, reading: &Located) -> bool {
        let text = &reading.text;
        // Occurrences count whole words only: the `1` of `-10` is none.
        let wanted_start = assertion.occurrence.map(|n| {
            text.match_indices(&assertion.surface)
                .filter(|(at, found)| {
                    !text[..*at]
                        .chars()
                        .next_back()
                        .is_some_and(char::is_alphanumeric)
                        && !text[at + found.len()..]
                            .chars()
                            .next()
                            .is_some_and(char::is_alphanumeric)
                })
                .nth(n.saturating_sub(1))
                .map(|(at, _)| at)
        });
        reading.constituents.iter().enumerate().any(|(index, c)| {
            c.category == assertion.category
                && text.get(c.span.clone()) == Some(assertion.surface.as_str())
                && wanted_start.is_none_or(|start| start == Some(c.span.start))
                && assertion
                    .head
                    .as_ref()
                    .is_none_or(|h| self.names(c.head.as_ref(), h))
                && assertion.function.as_ref().is_none_or(|(function, of)| {
                    c.function == function.child_name()
                        && c.parent
                            .is_some_and(|p| self.names(reading.constituents[p].head.as_ref(), of))
                })
                && assertion
                    .gap
                    .as_ref()
                    .is_none_or(|antecedent| has_antecedent(reading, index, antecedent))
        })
    }

    /// Whether `lexeme` is the word `name` (a lemma or a lexeme id).
    fn names(&self, lexeme: Option<&LexemeId>, name: &str) -> bool {
        lexeme.is_some_and(|id| {
            id.as_str() == name
                || self
                    .lexicon
                    .lexemes()
                    .get(id)
                    .is_some_and(|l| l.lemma.eq_ignore_ascii_case(name))
        })
    }
}

/// Whether the Gap inside Constituent `index` has an Antecedent spelled
/// `antecedent`. The Gap is bound at its lowest gapless ancestor `A`:
/// when the gapped child of `A` is its head (Right Node Raising), the
/// Antecedent is a sibling of that child; otherwise `A` modifies a
/// head, which is the Antecedent (a relative clause and its nominal).
fn has_antecedent(reading: &Located, index: usize, antecedent: &str) -> bool {
    let cs = &reading.constituents;
    if cs[index].gap == Gap::None {
        return false;
    }
    let mut child = index;
    let Some(binder) = std::iter::from_fn(|| {
        let parent = cs[child].parent?;
        if cs[parent].gap == Gap::None {
            return Some(Some(parent));
        }
        child = parent;
        Some(None)
    })
    .flatten()
    .next() else {
        return false;
    };
    let spelled = |c: &Constituent| reading.text.get(c.span.clone()) == Some(antecedent);
    let children = |of: usize| cs.iter().filter(move |c| c.parent == Some(of));
    if cs[child].function == "head" {
        children(binder).any(|c| c.function != "head" && spelled(c))
    } else {
        cs[binder]
            .parent
            .is_some_and(|host| children(host).any(|c| c.function == "head" && spelled(c)))
    }
}
