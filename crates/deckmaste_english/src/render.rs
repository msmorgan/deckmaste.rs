use std::fmt;

use crate::Ability;
use crate::AbilityKind;
use crate::Auxiliary;
use crate::AuxiliaryInflection;
use crate::AuxiliaryKind;
use crate::AuxiliaryNegation;
use crate::Capitalization;
use crate::Clause;
use crate::ConditionalPosition;
use crate::Cost;
use crate::EmbeddedRules;
use crate::EmbeddedRulesFrame;
use crate::KeywordArgumentSeparator;
use crate::KeywordListSeparator;
use crate::LoyaltyCost;
use crate::LoyaltyCostSign;
use crate::LoyaltyCostValue;
use crate::ModalFrame;
use crate::ModalHeaderSuffix;
use crate::ModalPreambleSeparator;
use crate::NumberSpelling;
use crate::OracleText;
use crate::Paragraph;
use crate::Phrase;
use crate::Predicate;
use crate::PredicateConjunction;
use crate::PreverbWord;
use crate::ScalarSign;
use crate::ScalarValue;
use crate::SentenceTerminalKind;
use crate::SentenceTerminalSuffix;
use crate::SimpleClause;
use crate::Subordinator;
use crate::TriggerWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    EmbeddedRulesFrameMismatch,
    InvalidAuxiliaryFeatures(Auxiliary),
    UnsupportedEnglishNumber(ScalarValue),
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmbeddedRulesFrameMismatch => {
                formatter.write_str("embedded rules do not match their structural frame")
            }
            Self::InvalidAuxiliaryFeatures(auxiliary) => {
                write!(
                    formatter,
                    "invalid auxiliary feature combination: {auxiliary:?}"
                )
            }
            Self::UnsupportedEnglishNumber(value) => {
                write!(formatter, "no English spelling for numeric value {value:?}")
            }
        }
    }
}

impl std::error::Error for RenderError {}

struct Renderer<'identity> {
    name: &'identity str,
    short_name: &'identity str,
}

impl OracleText {
    /// Reconstructs Oracle text without access to the parsed source string.
    ///
    /// Card identity is used only to expand normalized `~` and `~~`
    /// self-references. Container spans are intentionally inaccessible here.
    ///
    /// # Errors
    ///
    /// Returns an error when the AST contains an unsupported numeric spelling
    /// or structurally inconsistent embedded rules text.
    pub fn render(&self, name: &str, is_legendary: bool) -> Result<String, RenderError> {
        Renderer::new(name, is_legendary).oracle_text(self)
    }
}

impl<'identity> Renderer<'identity> {
    fn new(name: &'identity str, is_legendary: bool) -> Self {
        let short_name = if is_legendary {
            name.split_once(',').map_or(name, |(short, _)| short)
        } else {
            name
        };
        Self { name, short_name }
    }

    fn oracle_text(&self, oracle_text: &OracleText) -> Result<String, RenderError> {
        oracle_text
            .abilities
            .iter()
            .map(|ability| self.ability(ability))
            .collect::<Result<Vec<_>, _>>()
            .map(|abilities| abilities.join("\n"))
    }

    fn ability(&self, ability: &Ability) -> Result<String, RenderError> {
        let mut rendered = match &ability.kind {
            AbilityKind::Activated(activated) => {
                let cost = self.cost(&activated.cost)?;
                let effect = self.paragraph(&activated.effect)?;
                if effect.is_empty() { format!("{cost}:") } else { format!("{cost}: {effect}") }
            }
            AbilityKind::Triggered(triggered) => format!(
                "{} {}, {}",
                trigger_word(triggered.introducer),
                self.simple_clause(&triggered.event)?,
                self.paragraph(&triggered.effect)?
            ),
            AbilityKind::Loyalty(loyalty) => format!(
                "{}: {}",
                loyalty_cost(loyalty.cost),
                self.paragraph(&loyalty.effect)?
            ),
            AbilityKind::Modal(modal) => {
                let mut modal_text = match &modal.frame {
                    ModalFrame::Unframed => String::new(),
                    ModalFrame::Preamble { body, separator } => format!(
                        "{}{}",
                        self.paragraph(body)?,
                        match separator {
                            ModalPreambleSeparator::None => "",
                            ModalPreambleSeparator::Space => " ",
                            ModalPreambleSeparator::CommaSpace => ", ",
                        }
                    ),
                    ModalFrame::Activated(cost) => format!("{}: ", self.cost(cost)?),
                    ModalFrame::Triggered {
                        introducer, event, ..
                    } => format!(
                        "{} {}, ",
                        trigger_word(*introducer),
                        self.simple_clause(event)?
                    ),
                    ModalFrame::Loyalty(cost) => format!("{}: ", loyalty_cost(*cost)),
                };
                modal_text.push_str(&self.paragraph(&modal.header)?);
                if modal.header_suffix == ModalHeaderSuffix::SpacedEmDash {
                    modal_text.push_str(" —");
                }
                for mode in &modal.modes {
                    modal_text.push_str("\n• ");
                    modal_text.push_str(&self.paragraph(&mode.body)?);
                }
                modal_text
            }
            AbilityKind::Keyword(list) => list
                .abilities
                .iter()
                .map(|keyword| {
                    let mut item = self.phrase(&keyword.printed_name)?;
                    if let Some(argument) = &keyword.argument {
                        item.push_str(match keyword.argument_separator {
                            Some(KeywordArgumentSeparator::Space) => " ",
                            Some(KeywordArgumentSeparator::EmDash) => "—",
                            Some(KeywordArgumentSeparator::SpacedEmDash) => " — ",
                            None => "",
                        });
                        item.push_str(&self.phrase(argument)?);
                    }
                    if let Some(separator) = keyword.preceding_separator {
                        item.insert_str(
                            0,
                            match separator {
                                KeywordListSeparator::Comma => ", ",
                                KeywordListSeparator::Semicolon => "; ",
                            },
                        );
                    }
                    Ok(item)
                })
                .collect::<Result<Vec<_>, RenderError>>()?
                .concat(),
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph)?,
        };

        if let Some(ability_word) = &ability.ability_word {
            rendered = format!("{} — {rendered}", self.phrase(ability_word)?);
        }
        Ok(rendered)
    }

    fn cost(&self, cost: &Cost) -> Result<String, RenderError> {
        cost.components
            .iter()
            .map(|component| self.phrase(component))
            .collect::<Result<Vec<_>, _>>()
            .map(|components| components.join(", "))
    }

    fn paragraph(&self, paragraph: &Paragraph) -> Result<String, RenderError> {
        paragraph
            .sentences
            .iter()
            .map(|sentence| {
                let mut rendered = self.clause(&sentence.clause)?;
                if let Some(terminal) = sentence.terminal {
                    let character = match terminal.kind {
                        SentenceTerminalKind::Period => '.',
                        SentenceTerminalKind::ExclamationMark => '!',
                        SentenceTerminalKind::QuestionMark => '?',
                    };
                    for _ in 0..terminal.repetitions {
                        rendered.push(character);
                    }
                    if terminal.suffix == SentenceTerminalSuffix::DoubleQuote {
                        rendered.push('"');
                    } else if terminal.suffix == SentenceTerminalSuffix::SingleQuote {
                        rendered.push('\'');
                    }
                }
                Ok(rendered)
            })
            .collect::<Result<Vec<_>, RenderError>>()
            .map(|sentences| sentences.join(" "))
    }

    fn clause(&self, clause: &Clause) -> Result<String, RenderError> {
        match clause {
            Clause::Simple(clause) => self.simple_clause(clause),
            Clause::CommaSeparated(clause) => Ok(format!(
                "{}, {}",
                self.simple_clause(&clause.first)?,
                self.simple_clause(&clause.second)?
            )),
            Clause::Conditional(conditional) => {
                let condition = self.simple_clause(&conditional.condition)?;
                let consequence = self.simple_clause(&conditional.consequence)?;
                let subordinator = subordinator(
                    conditional.subordinator,
                    conditional.subordinator_capitalization,
                );
                Ok(match conditional.position {
                    ConditionalPosition::BeforeConsequence => {
                        format!("{subordinator} {condition}, {consequence}")
                    }
                    ConditionalPosition::AfterConsequence => {
                        format!("{consequence} {subordinator} {condition}")
                    }
                })
            }
        }
    }

    fn simple_clause(&self, clause: &SimpleClause) -> Result<String, RenderError> {
        if let Some(unparsed) = &clause.unparsed {
            return self.phrase(unparsed);
        }

        let mut rendered = String::new();
        if let Some(subject) = &clause.subject {
            rendered.push_str(&self.phrase(subject)?);
            rendered.push(' ');
        }
        if let Some(predicate) = &clause.predicate {
            rendered.push_str(&self.predicate(predicate)?);
        }
        for coordinated in &clause.coordinated_predicates {
            rendered.push_str(coordination_separator(
                coordinated.conjunction,
                coordinated.has_comma,
            ));
            rendered.push_str(&self.predicate(&coordinated.predicate)?);
        }
        for coordinated in &clause.coordinated_clauses {
            rendered.push_str(coordination_separator(
                coordinated.conjunction,
                coordinated.has_comma,
            ));
            rendered.push_str(&self.simple_clause(&coordinated.clause)?);
        }
        Ok(rendered)
    }

    fn predicate(&self, predicate: &Predicate) -> Result<String, RenderError> {
        let mut rendered = String::new();
        if let Some(auxiliary) = predicate.auxiliary {
            let text = auxiliary_text(auxiliary)
                .ok_or(RenderError::InvalidAuxiliaryFeatures(auxiliary))?;
            rendered.push_str(&text);
            rendered.push(' ');
        }
        for word in &predicate.preverb_words {
            rendered.push_str(match word {
                PreverbWord::Not => "not ",
                PreverbWord::Also => "also ",
            });
        }
        rendered.push_str(&self.phrase(&predicate.verb)?);
        if let Some(complement) = &predicate.complement {
            let complement = self.phrase(complement)?;
            if !complement.starts_with([',', '.', ';', ':', '!', '?', ')', ']']) {
                rendered.push(' ');
            }
            rendered.push_str(&complement);
        }
        Ok(rendered)
    }

    fn phrase(&self, phrase: &Phrase) -> Result<String, RenderError> {
        Ok(match phrase {
            Phrase::ThisCard { form, .. } => match form {
                crate::ThisCardForm::AbbreviatedName => self.short_name.to_owned(),
                crate::ThisCardForm::FullName => self.name.to_owned(),
            },
            Phrase::OracleSymbol { symbol, .. } => format!("{{{symbol}}}"),
            Phrase::SymbolSequence { symbols, .. } => symbols
                .iter()
                .map(|symbol| format!("{{{symbol}}}"))
                .collect::<Vec<_>>()
                .concat(),
            Phrase::NumberLiteral {
                value, spelling, ..
            } => render_number(*value, *spelling)
                .ok_or(RenderError::UnsupportedEnglishNumber(*value))?,
            Phrase::QuantityPhrase(phrase) => format!(
                "{} {}",
                self.phrase(&phrase.quantity)?,
                self.phrase(&phrase.unit)?
            ),
            Phrase::PowerToughness(expression) => format!(
                "{}/{}",
                render_signed_scalar(expression.power),
                render_signed_scalar(expression.toughness)
            ),
            Phrase::NounPhrase(phrase) => format!(
                "{} {}",
                self.expand_self_references(&phrase.determiner.text),
                self.phrase(&phrase.head)?
            ),
            Phrase::ModifiedNounPhrase(phrase) => format!(
                "{} {}",
                self.phrase(&phrase.modifier)?,
                self.phrase(&phrase.head)?
            ),
            Phrase::UnknownPhrase(text)
            | Phrase::Lexeme { text, .. }
            | Phrase::CatalogTerm { text, .. } => self.expand_self_references(text),
            Phrase::EmbeddedRulesPhrase {
                text,
                embedded_rules,
            } => self.embedded_rules_phrase(text, embedded_rules)?,
        })
    }

    fn embedded_rules_phrase(
        &self,
        text: &str,
        embedded_rules: &[EmbeddedRules],
    ) -> Result<String, RenderError> {
        if let [embedded] = embedded_rules
            && embedded.frame == EmbeddedRulesFrame::Bare
        {
            return self.ability(&embedded.ability);
        }

        let mut rendered = String::new();
        let mut rest = text;
        for embedded in embedded_rules {
            let EmbeddedRulesFrame::DoubleQuoted { closed } = embedded.frame else {
                return Err(RenderError::EmbeddedRulesFrameMismatch);
            };
            let Some((prefix, after_open)) = rest.split_once('"') else {
                return Err(RenderError::EmbeddedRulesFrameMismatch);
            };
            rendered.push_str(&self.expand_self_references(prefix));
            rendered.push('"');
            rendered.push_str(&self.ability(&embedded.ability)?);
            if closed {
                let Some((_, suffix)) = after_open.split_once('"') else {
                    return Err(RenderError::EmbeddedRulesFrameMismatch);
                };
                rendered.push('"');
                rest = suffix;
            } else {
                rest = "";
            }
        }
        rendered.push_str(&self.expand_self_references(rest));
        Ok(rendered)
    }

    fn expand_self_references(&self, text: &str) -> String {
        let mut rendered = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(index) = rest.find('~') {
            rendered.push_str(&rest[..index]);
            rest = &rest[index + 1..];
            if let Some(after_second) = rest.strip_prefix('~') {
                rendered.push_str(self.name);
                rest = after_second;
            } else {
                rendered.push_str(self.short_name);
            }
        }
        rendered.push_str(rest);
        rendered
    }
}

fn render_signed_scalar(scalar: crate::SignedScalar) -> String {
    let sign = match scalar.sign {
        ScalarSign::None => "",
        ScalarSign::Plus => "+",
        ScalarSign::Minus => "-",
    };
    let value = match scalar.value {
        ScalarValue::Integer(value) => value.to_string(),
        ScalarValue::X => "X".to_owned(),
        ScalarValue::Star => "*".to_owned(),
    };
    format!("{sign}{value}")
}

fn render_number(value: ScalarValue, spelling: NumberSpelling) -> Option<String> {
    match spelling {
        NumberSpelling::Digits => match value {
            ScalarValue::Integer(value) => Some(value.to_string()),
            ScalarValue::X => Some("X".to_owned()),
            ScalarValue::Star => Some("*".to_owned()),
        },
        NumberSpelling::EnglishWord(capitalization) => {
            let word = match value {
                ScalarValue::Integer(0) => "zero",
                ScalarValue::Integer(1) => "one",
                ScalarValue::Integer(2) => "two",
                ScalarValue::Integer(3) => "three",
                ScalarValue::Integer(4) => "four",
                ScalarValue::Integer(5) => "five",
                ScalarValue::Integer(6) => "six",
                ScalarValue::Integer(7) => "seven",
                ScalarValue::Integer(8) => "eight",
                ScalarValue::Integer(9) => "nine",
                ScalarValue::Integer(10) => "ten",
                ScalarValue::Integer(11) => "eleven",
                ScalarValue::Integer(12) => "twelve",
                ScalarValue::Integer(13) => "thirteen",
                ScalarValue::Integer(14) => "fourteen",
                ScalarValue::Integer(15) => "fifteen",
                ScalarValue::Integer(16) => "sixteen",
                ScalarValue::Integer(17) => "seventeen",
                ScalarValue::Integer(18) => "eighteen",
                ScalarValue::Integer(19) => "nineteen",
                ScalarValue::Integer(20) => "twenty",
                ScalarValue::Integer(50) => "fifty",
                ScalarValue::X => "X",
                ScalarValue::Integer(_) | ScalarValue::Star => return None,
            };
            Some(apply_capitalization(word, capitalization))
        }
    }
}

fn loyalty_cost(cost: LoyaltyCost) -> String {
    let sign = match cost.sign {
        LoyaltyCostSign::None => "",
        LoyaltyCostSign::Plus => "+",
        LoyaltyCostSign::Minus => "−",
    };
    let value = match cost.value {
        LoyaltyCostValue::Number(value) => value.to_string(),
        LoyaltyCostValue::X => "X".to_owned(),
    };
    format!("[{sign}{value}]")
}

fn trigger_word(word: TriggerWord) -> &'static str {
    match word {
        TriggerWord::When => "When",
        TriggerWord::Whenever => "Whenever",
        TriggerWord::At => "At",
    }
}

fn subordinator(subordinator: Subordinator, capitalization: Capitalization) -> String {
    let text = match subordinator {
        Subordinator::When => "when",
        Subordinator::If => "if",
        Subordinator::Unless => "unless",
        Subordinator::AsLongAs => "as long as",
        Subordinator::Until => "until",
        Subordinator::Because => "because",
    };
    apply_capitalization(text, capitalization)
}

fn coordination_separator(conjunction: PredicateConjunction, has_comma: bool) -> &'static str {
    match (conjunction, has_comma) {
        (PredicateConjunction::And, false) => " and ",
        (PredicateConjunction::Or, false) => " or ",
        (PredicateConjunction::Then, false) => " then ",
        (PredicateConjunction::And, true) => ", and ",
        (PredicateConjunction::Or, true) => ", or ",
        (PredicateConjunction::Then, true) => ", then ",
    }
}

fn auxiliary_text(auxiliary: Auxiliary) -> Option<String> {
    use AuxiliaryInflection::Base;
    use AuxiliaryInflection::ThirdPersonSingular;
    use AuxiliaryKind::Can;
    use AuxiliaryKind::Could;
    use AuxiliaryKind::Do;
    use AuxiliaryKind::May;
    use AuxiliaryKind::Might;
    use AuxiliaryKind::Must;
    use AuxiliaryKind::Shall;
    use AuxiliaryKind::Should;
    use AuxiliaryKind::Will;
    use AuxiliaryKind::Would;
    use AuxiliaryNegation::Contracted;
    use AuxiliaryNegation::Fused;
    use AuxiliaryNegation::None as NoNegation;

    let text = match (auxiliary.kind, auxiliary.inflection, auxiliary.negation) {
        (Can, Base, NoNegation) => "can",
        (Can, Base, Contracted) => "can't",
        (Can, Base, Fused) => "cannot",
        (Could, Base, NoNegation) => "could",
        (Do, Base, NoNegation) => "do",
        (Do, Base, Contracted) => "don't",
        (Do, ThirdPersonSingular, NoNegation) => "does",
        (Do, ThirdPersonSingular, Contracted) => "doesn't",
        (May, Base, NoNegation) => "may",
        (Might, Base, NoNegation) => "might",
        (Must, Base, NoNegation) => "must",
        (Shall, Base, NoNegation) => "shall",
        (Should, Base, NoNegation) => "should",
        (Will, Base, NoNegation) => "will",
        (Will, Base, Contracted) => "won't",
        (Would, Base, NoNegation) => "would",
        _ => return Option::None,
    };
    Some(apply_capitalization(text, auxiliary.capitalization))
}

fn apply_capitalization(text: &str, capitalization: Capitalization) -> String {
    match capitalization {
        Capitalization::Lowercase => text.to_owned(),
        Capitalization::Capitalized => {
            let mut chars = text.chars();
            chars
                .next()
                .map(char::to_uppercase)
                .into_iter()
                .flatten()
                .chain(chars)
                .collect()
        }
    }
}
