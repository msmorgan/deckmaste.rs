use std::fmt;

use crate::catalog::CatalogKind;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::ConditionalPosition;
use crate::syntax::Cost;
use crate::syntax::Demonstrative;
use crate::syntax::Determiner;
use crate::syntax::IndefiniteArticle;
use crate::syntax::InfinitiveClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordArgumentSeparator;
use crate::syntax::KeywordListSeparator;
use crate::syntax::LoyaltyCost;
use crate::syntax::LoyaltyCostSign;
use crate::syntax::LoyaltyCostValue;
use crate::syntax::ModalAbility;
use crate::syntax::ModalFrame;
use crate::syntax::ModalHeaderSuffix;
use crate::syntax::ModalPreambleSeparator;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseConjunction;
use crate::syntax::OracleSymbol;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::Possessor;
use crate::syntax::PredicateConjunction;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::ScalarSign;
use crate::syntax::ScalarValue;
use crate::syntax::Sentence;
use crate::syntax::SentenceEnding;
use crate::syntax::SignedScalar;
use crate::syntax::SimpleClause;
use crate::syntax::Subject;
use crate::syntax::Subordinator;
use crate::syntax::ThisCardForm;
use crate::syntax::TriggerWord;
use crate::syntax::VerbDependent;
use crate::syntax::VerbPhrase;
use crate::word::Adjective;
use crate::word::InitialSound;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::PronounInstance;
use crate::word::Vocabulary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    MissingLexicalForm(&'static str),
    InvalidIndefiniteArticle,
    CardIdentityRequired,
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLexicalForm(kind) => write!(formatter, "missing {kind} form"),
            Self::InvalidIndefiniteArticle => formatter.write_str("invalid indefinite article"),
            Self::CardIdentityRequired => {
                formatter.write_str("card identity is required to render this determiner")
            }
        }
    }
}

impl std::error::Error for RenderError {}

impl OracleText {
    /// Renders this semantic tree without consulting its original source text.
    ///
    /// # Errors
    ///
    /// Returns an error when the tree requests a grammatical form that its
    /// vocabulary identity does not define.
    pub fn render(&self, name: &str, is_legendary: bool) -> Result<String, RenderError> {
        Renderer::new(name, is_legendary).oracle_text(self)
    }
}

impl Determiner {
    /// Renders a determiner that does not require card-name context.
    ///
    /// # Errors
    ///
    /// A noun-phrase possessor containing a self reference requires the
    /// [`OracleText::render`] identity arguments and is rejected here.
    pub fn render(&self) -> Result<String, RenderError> {
        match self {
            Self::Possessive(Possessor::Pronoun(pronoun)) => Vocabulary::new()
                .render_possessive_pronoun(*pronoun)
                .map(str::to_owned)
                .ok_or(RenderError::MissingLexicalForm("possessive pronoun")),
            Self::Possessive(Possessor::NounPhrase(_)) => Err(RenderError::CardIdentityRequired),
            Self::The => Ok("the".to_owned()),
            Self::Each => Ok("each".to_owned()),
            Self::Indefinite(IndefiniteArticle::A) => Ok("a".to_owned()),
            Self::Indefinite(IndefiniteArticle::An) => Ok("an".to_owned()),
            Self::Demonstrative(Demonstrative::This) => Ok("this".to_owned()),
            Self::Demonstrative(Demonstrative::That) => Ok("that".to_owned()),
            Self::Demonstrative(Demonstrative::These) => Ok("these".to_owned()),
            Self::Demonstrative(Demonstrative::Those) => Ok("those".to_owned()),
            Self::Target(None) => Ok("target".to_owned()),
            Self::Target(Some(quantity)) => Ok(format!("{} target", render_quantity(*quantity))),
            Self::Quantity(quantity) => Ok(render_quantity(*quantity)),
            Self::All => Ok("all".to_owned()),
        }
    }
}

struct Renderer<'identity> {
    name: &'identity str,
    short_name: &'identity str,
    vocabulary: Vocabulary,
}

impl<'identity> Renderer<'identity> {
    fn new(name: &'identity str, is_legendary: bool) -> Self {
        let short_name = if is_legendary {
            name.split_once(',').map_or(name, |(short, _)| short.trim())
        } else {
            name
        };
        Self {
            name,
            short_name,
            vocabulary: Vocabulary::new(),
        }
    }

    fn oracle_text(&self, oracle_text: &OracleText) -> Result<String, RenderError> {
        oracle_text
            .abilities
            .iter()
            .map(|ability| self.ability(ability, true))
            .collect::<Result<Vec<_>, _>>()
            .map(|abilities| abilities.join("\n"))
    }

    fn ability(&self, ability: &Ability, capitalize: bool) -> Result<String, RenderError> {
        let body = self.ability_kind(&ability.kind)?;
        let rendered = if let Some(ability_word) = &ability.ability_word {
            format!(
                "{} — {}",
                ability_word.canonical().to_lowercase(),
                capitalize_first(body)
            )
        } else {
            body
        };
        Ok(if capitalize { capitalize_first(rendered) } else { rendered })
    }

    fn ability_kind(&self, kind: &AbilityKind) -> Result<String, RenderError> {
        match kind {
            AbilityKind::Activated(activated) => Ok(format!(
                "{}: {}",
                self.cost(&activated.cost)?,
                self.paragraph(&activated.effect, true)?
            )),
            AbilityKind::Triggered(triggered) => Ok(format!(
                "{} {}, {}",
                render_trigger_word(triggered.introducer),
                self.simple_clause(&triggered.event)?,
                self.paragraph(&triggered.effect, false)?
            )),
            AbilityKind::Loyalty(loyalty) => Ok(format!(
                "[{}]: {}",
                render_loyalty_cost(loyalty.cost),
                self.paragraph(&loyalty.effect, true)?
            )),
            AbilityKind::Modal(modal) => self.modal_ability(modal),
            AbilityKind::Keyword(keyword) => self.keyword_ability_list(keyword),
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph, true),
        }
    }

    fn modal_ability(&self, modal: &ModalAbility) -> Result<String, RenderError> {
        let header_is_sentence_initial = !matches!(modal.frame, ModalFrame::Triggered { .. });
        let header = self.paragraph(&modal.header, header_is_sentence_initial)?;
        let header = match modal.header_suffix {
            ModalHeaderSuffix::None => header,
            ModalHeaderSuffix::SpacedEmDash => format!("{header} —"),
        };
        let framed = match &modal.frame {
            ModalFrame::Unframed => header,
            ModalFrame::Preamble { body, separator } => {
                let separator = match separator {
                    ModalPreambleSeparator::None => "",
                    ModalPreambleSeparator::Space => " ",
                    ModalPreambleSeparator::CommaSpace => ", ",
                };
                format!("{}{separator}{header}", self.paragraph(body, true)?)
            }
            ModalFrame::Activated(cost) => format!("{}: {header}", self.cost(cost)?),
            ModalFrame::Triggered { introducer, event } => format!(
                "{} {}, {}",
                render_trigger_word(*introducer),
                self.simple_clause(event)?,
                header
            ),
            ModalFrame::Loyalty(cost) => {
                format!("[{}]: {header}", render_loyalty_cost(*cost))
            }
        };
        let modes = modal
            .modes
            .iter()
            .map(|mode| {
                self.paragraph(&mode.body, true)
                    .map(|body| format!("• {body}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if modes.is_empty() {
            Ok(framed)
        } else {
            Ok(format!("{framed}\n{}", modes.join("\n")))
        }
    }

    fn keyword_ability_list(&self, list: &KeywordAbilityList) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for item in &list.abilities {
            if let Some(separator) = item.preceding_separator {
                rendered.push_str(match separator {
                    KeywordListSeparator::Comma => ", ",
                    KeywordListSeparator::Semicolon => "; ",
                });
            }
            rendered.push_str(&item.ability.canonical().to_lowercase());
            if let Some(argument) = &item.argument {
                rendered.push_str(match item.argument_separator {
                    Some(KeywordArgumentSeparator::Space) | None => " ",
                    Some(KeywordArgumentSeparator::EmDash) => "—",
                    Some(KeywordArgumentSeparator::SpacedEmDash) => " — ",
                });
                rendered.push_str(&self.phrase(argument)?);
            }
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

    fn paragraph(
        &self,
        paragraph: &Paragraph,
        capitalize_first_sentence: bool,
    ) -> Result<String, RenderError> {
        paragraph
            .sentences
            .iter()
            .enumerate()
            .map(|(index, sentence)| {
                self.sentence(sentence, capitalize_first_sentence || index > 0)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|sentences| sentences.join(" "))
    }

    fn sentence(&self, sentence: &Sentence, capitalize: bool) -> Result<String, RenderError> {
        let clause = self.clause(&sentence.clause)?;
        let mut rendered = if capitalize { capitalize_first(clause) } else { clause };
        let (terminal, count) = match sentence.ending {
            SentenceEnding::None => ('\0', 0),
            SentenceEnding::Period(count) => ('.', count),
            SentenceEnding::Exclamation(count) => ('!', count),
            SentenceEnding::Question(count) => ('?', count),
        };
        rendered.extend(std::iter::repeat_n(terminal, usize::from(count)));
        Ok(rendered)
    }

    fn clause(&self, clause: &Clause) -> Result<String, RenderError> {
        match clause {
            Clause::Simple(simple) => self.simple_clause(simple),
            Clause::Conditional(conditional) => {
                let condition = self.clause(&conditional.condition)?;
                let consequence = self.clause(&conditional.consequence)?;
                let subordinator = render_subordinator(conditional.subordinator);
                Ok(match conditional.position {
                    ConditionalPosition::BeforeConsequence => {
                        format!("{subordinator} {condition}, {consequence}")
                    }
                    ConditionalPosition::AfterConsequence => {
                        format!("{consequence} {subordinator} {condition}")
                    }
                })
            }
            Clause::Coordinated(coordinated) => {
                let mut rendered = self.clause(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    if coordination.comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    rendered.push_str(render_predicate_conjunction(coordination.conjunction));
                    rendered.push(' ');
                    rendered.push_str(&self.clause(&coordination.clause)?);
                }
                Ok(rendered)
            }
            Clause::Unknown(unknown) => Ok(unknown.0.clone()),
        }
    }

    fn simple_clause(&self, clause: &SimpleClause) -> Result<String, RenderError> {
        let mut parts = Vec::with_capacity(2);
        if let Some(subject) = &clause.subject {
            parts.push(self.subject(subject)?);
        }
        parts.push(self.verb_phrase(&clause.predicate)?);
        Ok(join_words(parts))
    }

    fn subject(&self, subject: &Subject) -> Result<String, RenderError> {
        match subject {
            Subject::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase),
            Subject::Unknown(unknown) => Ok(unknown.0.clone()),
        }
    }

    fn verb_phrase(&self, phrase: &VerbPhrase) -> Result<String, RenderError> {
        let mut parts = Vec::with_capacity(
            phrase.auxiliaries.len() + phrase.preverb_modifiers.len() + phrase.dependents.len() + 1,
        );
        for &auxiliary in &phrase.auxiliaries {
            parts.push(
                self.vocabulary
                    .render_auxiliary(auxiliary)
                    .map(str::to_owned)
                    .ok_or(RenderError::MissingLexicalForm("auxiliary"))?,
            );
        }
        parts.extend(phrase.preverb_modifiers.iter().map(|modifier| {
            match modifier {
                PreverbModifier::Not => "not",
                PreverbModifier::Also => "also",
            }
            .to_owned()
        }));
        parts.push(
            self.vocabulary
                .render_verb_instance(&phrase.verb)
                .ok_or(RenderError::MissingLexicalForm("verb"))?,
        );
        for dependent in &phrase.dependents {
            parts.push(self.verb_dependent(dependent)?);
        }
        Ok(join_words(parts))
    }

    fn verb_dependent(&self, dependent: &VerbDependent) -> Result<String, RenderError> {
        match dependent {
            VerbDependent::DirectObject(noun) | VerbDependent::IndirectObject(noun) => {
                self.noun_phrase(noun)
            }
            VerbDependent::PredicateComplement(phrase)
            | VerbDependent::Scalar(phrase)
            | VerbDependent::Statistic(phrase)
            | VerbDependent::Adverbial(phrase) => self.phrase(phrase),
            VerbDependent::Prepositional(preposition) => self.prepositional_phrase(preposition),
            VerbDependent::Infinitive(infinitive) => self.infinitive_clause(infinitive),
            VerbDependent::Subordinate(clause) => self.clause(clause),
            VerbDependent::Unknown(unknown) => Ok(unknown.0.clone()),
        }
    }

    fn infinitive_clause(&self, clause: &InfinitiveClause) -> Result<String, RenderError> {
        let predicate = self.verb_phrase(&clause.predicate)?;
        Ok(match clause.marker {
            InfinitiveMarker::Bare => predicate,
            InfinitiveMarker::To => format!("to {predicate}"),
        })
    }

    fn noun_phrase(&self, phrase: &NounPhrase) -> Result<String, RenderError> {
        match phrase {
            NounPhrase::Nominal(nominal) => self.nominal_phrase(nominal),
            NounPhrase::Pronoun { pronoun, case } => self
                .vocabulary
                .render_pronoun(PronounInstance {
                    pronoun: *pronoun,
                    case: *case,
                })
                .map(str::to_owned)
                .ok_or(RenderError::MissingLexicalForm("pronoun")),
            NounPhrase::ThisCard(form) => self.this_card(*form),
            NounPhrase::Coordinated(coordinated) => {
                let mut rendered = self.noun_phrase(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    rendered.push(' ');
                    rendered.push_str(match coordination.conjunction {
                        NounPhraseConjunction::And => "and",
                        NounPhraseConjunction::Or => "or",
                    });
                    rendered.push(' ');
                    rendered.push_str(&self.noun_phrase(&coordination.phrase)?);
                }
                Ok(rendered)
            }
        }
    }

    fn nominal_phrase(&self, phrase: &NominalPhrase) -> Result<String, RenderError> {
        if let Some(Determiner::Indefinite(article)) = phrase.determiner {
            let expected = self.nominal_initial_sound(phrase)?;
            let actual = match article {
                IndefiniteArticle::A => InitialSound::Consonant,
                IndefiniteArticle::An => InitialSound::Vowel,
            };
            if actual != expected {
                return Err(RenderError::InvalidIndefiniteArticle);
            }
        }

        let mut parts = Vec::with_capacity(phrase.modifiers.len() + phrase.complements.len() + 2);
        if let Some(determiner) = &phrase.determiner {
            parts.push(self.determiner(determiner)?);
        }
        for modifier in &phrase.modifiers {
            parts.push(match modifier {
                NominalModifier::Adjective(adjective) => self.adjective_phrase(adjective)?,
                NominalModifier::Noun(noun) => self.render_noun(noun)?,
            });
        }
        parts.push(self.render_noun(&phrase.head)?);
        for complement in &phrase.complements {
            parts.push(match complement {
                NominalComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition)?
                }
                NominalComplement::Relative(relative) => self.clause(&relative.clause)?,
                NominalComplement::Unknown(unknown) => unknown.0.clone(),
            });
        }
        Ok(join_words(parts))
    }

    fn nominal_initial_sound(&self, phrase: &NominalPhrase) -> Result<InitialSound, RenderError> {
        if let Some(first) = phrase.modifiers.first() {
            return match first {
                NominalModifier::Adjective(adjective) => {
                    self.adjective_initial_sound(&adjective.head)
                }
                NominalModifier::Noun(noun) => self.noun_initial_sound(noun),
            };
        }
        self.noun_initial_sound(&phrase.head)
    }

    fn noun_initial_sound(&self, noun: &NounInstance) -> Result<InitialSound, RenderError> {
        let noun = match noun {
            NounInstance::Singular(noun)
            | NounInstance::Plural(noun)
            | NounInstance::Mass(noun) => noun,
        };
        match noun {
            Noun::Word(vocab) => Ok(self.vocabulary.initial_sound(*vocab)),
            Noun::Catalog(atom) => Ok(spelling_initial_sound(atom.canonical())),
            Noun::Gerund(verb) => self
                .vocabulary
                .render_verb_instance(&crate::word::VerbInstance {
                    verb: verb.clone(),
                    slot: crate::word::VerbSlot::PresentParticiple,
                })
                .map(|surface| spelling_initial_sound(&surface))
                .ok_or(RenderError::MissingLexicalForm("gerund")),
        }
    }

    fn adjective_initial_sound(&self, adjective: &Adjective) -> Result<InitialSound, RenderError> {
        match adjective {
            Adjective::Word(vocab) => Ok(self.vocabulary.initial_sound(*vocab)),
            _ => self
                .vocabulary
                .render_adjective(adjective)
                .map(|surface| spelling_initial_sound(&surface))
                .ok_or(RenderError::MissingLexicalForm("adjective")),
        }
    }

    fn determiner(&self, determiner: &Determiner) -> Result<String, RenderError> {
        match determiner {
            Determiner::Possessive(Possessor::Pronoun(pronoun)) => self
                .vocabulary
                .render_possessive_pronoun(*pronoun)
                .map(str::to_owned)
                .ok_or(RenderError::MissingLexicalForm("possessive pronoun")),
            Determiner::Possessive(Possessor::NounPhrase(possessor)) => {
                Ok(format!("{}'s", self.noun_phrase(possessor)?))
            }
            _ => determiner.render(),
        }
    }

    fn adjective_phrase(&self, phrase: &AdjectivePhrase) -> Result<String, RenderError> {
        let mut parts = vec![
            self.vocabulary
                .render_adjective(&phrase.head)
                .ok_or(RenderError::MissingLexicalForm("adjective"))?,
        ];
        for complement in &phrase.complements {
            parts.push(match complement {
                AdjectiveComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition)?
                }
                AdjectiveComplement::Infinitive(infinitive) => {
                    self.infinitive_clause(infinitive)?
                }
                AdjectiveComplement::Unknown(unknown) => unknown.0.clone(),
            });
        }
        Ok(join_words(parts))
    }

    fn prepositional_phrase(&self, phrase: &PrepositionalPhrase) -> Result<String, RenderError> {
        Ok(format!(
            "{} {}",
            render_preposition(phrase.preposition),
            self.phrase(&phrase.object)?
        ))
    }

    fn phrase(&self, phrase: &Phrase) -> Result<String, RenderError> {
        match phrase {
            Phrase::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase),
            Phrase::AdjectivePhrase(adjective) => self.adjective_phrase(adjective),
            Phrase::PrepositionalPhrase(preposition) => self.prepositional_phrase(preposition),
            Phrase::Quantity(quantity) => Ok(render_quantity(*quantity)),
            Phrase::CatalogAtom(atom) => Ok(match atom.kind {
                CatalogKind::KeywordAbility
                | CatalogKind::KeywordAction
                | CatalogKind::AbilityWord => atom.canonical().to_lowercase(),
                _ => atom.render_adjective(),
            }),
            Phrase::ColorWord(color) => Ok(color.spelling().to_owned()),
            Phrase::ThisCard(form) => self.this_card(*form),
            Phrase::OracleSymbol(symbol) => Ok(symbol.as_str().to_owned()),
            Phrase::SymbolSequence(symbols) => {
                Ok(symbols.iter().map(OracleSymbol::as_str).collect())
            }
            Phrase::NumberLiteral(number) => Ok(number.numeral.format(number.value)),
            Phrase::SignedScalar(scalar) => Ok(render_signed_scalar(*scalar)),
            Phrase::PowerToughness(power_toughness) => Ok(format!(
                "{}/{}",
                render_signed_scalar(power_toughness.power),
                render_signed_scalar(power_toughness.toughness)
            )),
            Phrase::QuotedAbility(quoted) => {
                let mut rendered = format!("\"{}", self.ability(&quoted.ability, true)?);
                if quoted.closed {
                    rendered.push('"');
                }
                Ok(rendered)
            }
            Phrase::UnknownPhrase(unknown) => Ok(unknown.0.clone()),
        }
    }

    fn render_noun(&self, noun: &NounInstance) -> Result<String, RenderError> {
        self.vocabulary
            .render_noun(noun)
            .ok_or(RenderError::MissingLexicalForm("noun"))
    }

    fn this_card(&self, form: ThisCardForm) -> Result<String, RenderError> {
        let rendered = match form {
            ThisCardForm::AbbreviatedName => self.short_name,
            ThisCardForm::FullName => self.name,
        };
        if rendered.is_empty() {
            Err(RenderError::CardIdentityRequired)
        } else {
            Ok(rendered.to_owned())
        }
    }
}

fn render_trigger_word(word: TriggerWord) -> &'static str {
    match word {
        TriggerWord::When => "when",
        TriggerWord::Whenever => "whenever",
        TriggerWord::At => "at",
    }
}

fn render_loyalty_cost(cost: LoyaltyCost) -> String {
    let sign = match cost.sign {
        LoyaltyCostSign::None => "",
        LoyaltyCostSign::Plus => "+",
        LoyaltyCostSign::Minus => "−",
    };
    let value = match cost.value {
        LoyaltyCostValue::Number(value) => value.to_string(),
        LoyaltyCostValue::X => "X".to_owned(),
    };
    format!("{sign}{value}")
}

fn render_quantity(quantity: Quantity) -> String {
    match quantity {
        Quantity::Exact(number) => number.numeral.format(number.value),
        Quantity::UpTo(number) => format!("up to {}", number.numeral.format(number.value)),
        Quantity::ThatMany => "that many".to_owned(),
        Quantity::ThatMuch => "that much".to_owned(),
    }
}

fn render_signed_scalar(scalar: SignedScalar) -> String {
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

fn render_subordinator(subordinator: Subordinator) -> &'static str {
    match subordinator {
        Subordinator::When => "when",
        Subordinator::If => "if",
        Subordinator::Unless => "unless",
        Subordinator::AsLongAs => "as long as",
        Subordinator::Until => "until",
        Subordinator::Because => "because",
    }
}

fn render_predicate_conjunction(conjunction: PredicateConjunction) -> &'static str {
    match conjunction {
        PredicateConjunction::And => "and",
        PredicateConjunction::Or => "or",
        PredicateConjunction::Then => "then",
    }
}

fn render_preposition(preposition: Preposition) -> &'static str {
    match preposition {
        Preposition::At => "at",
        Preposition::By => "by",
        Preposition::For => "for",
        Preposition::From => "from",
        Preposition::In => "in",
        Preposition::Into => "into",
        Preposition::Of => "of",
        Preposition::On => "on",
        Preposition::Onto => "onto",
        Preposition::To => "to",
        Preposition::Under => "under",
        Preposition::With => "with",
        Preposition::Without => "without",
    }
}

fn spelling_initial_sound(surface: &str) -> InitialSound {
    if surface.chars().next().is_some_and(|character| {
        matches!(character.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }) {
        InitialSound::Vowel
    } else {
        InitialSound::Consonant
    }
}

fn capitalize_first(text: String) -> String {
    let mut characters = text.chars();
    let Some(first) = characters.next() else {
        return text;
    };
    first.to_uppercase().chain(characters).collect()
}

fn join_words(parts: Vec<String>) -> String {
    parts
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::Numeral;
    use crate::catalog::CatalogKind;
    use crate::catalog::CatalogSlot;
    use crate::catalog::CatalogValue;
    use crate::catalog::Catalogs;
    use crate::syntax::*;
    use crate::word::Adjective;
    use crate::word::Auxiliary;
    use crate::word::AuxiliaryInflection;
    use crate::word::AuxiliaryInstance;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::NounUsage;
    use crate::word::Number;
    use crate::word::Person;
    use crate::word::Pronoun;
    use crate::word::PronounCase;
    use crate::word::Verb;
    use crate::word::VerbInstance;
    use crate::word::VerbSlot;
    use crate::word::Vocab;
    use crate::word::WordMatch;

    const SECOND_SINGULAR_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Second,
        number: Number::Singular,
    };
    const THIRD_SINGULAR_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    };
    const THIRD_PLURAL_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Plural,
    };

    #[test]
    fn keyword_abilities_render_from_canonical_catalog_identity() {
        let catalogs = fixture_catalogs();
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Keyword(KeywordAbilityList {
                    abilities: vec![
                        KeywordAbility {
                            preceding_separator: None,
                            ability: keyword_atom(&catalogs, "flying"),
                            argument_separator: None,
                            argument: None,
                        },
                        KeywordAbility {
                            preceding_separator: Some(KeywordListSeparator::Comma),
                            ability: keyword_atom(&catalogs, "deathtouch"),
                            argument_separator: None,
                            argument: None,
                        },
                    ],
                }),
            }],
        };

        assert_eq!(source_free(&ast, "Test Card", false), "Flying, deathtouch");
    }

    #[test]
    fn ordinary_auxiliary_and_infinitive_sentences_render_structurally() {
        let draw = paragraph_ability(simple(
            None,
            verb_phrase(
                Vocab::Draw,
                VerbSlot::Imperative,
                vec![VerbDependent::DirectObject(nominal(
                    Some(Determiner::Indefinite(IndefiniteArticle::A)),
                    vec![],
                    NounInstance::Singular(Noun::Word(Vocab::Card)),
                    vec![],
                ))],
            ),
        ));
        assert_eq!(source_free(&draw, "Test Card", false), "Draw a card.");

        let spells_cost = paragraph_ability(simple(
            Some(Subject::NounPhrase(nominal(
                None,
                vec![],
                NounInstance::Plural(Noun::Word(Vocab::Spell)),
                vec![],
            ))),
            verb_phrase(
                Vocab::Cost,
                THIRD_PLURAL_PRESENT,
                vec![
                    VerbDependent::Scalar(Phrase::OracleSymbol(OracleSymbol::new("{1}").unwrap())),
                    VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(AdjectivePhrase {
                        head: Adjective::Word(Vocab::Less),
                        complements: vec![],
                    }))),
                    VerbDependent::Infinitive(InfinitiveClause {
                        marker: InfinitiveMarker::To,
                        predicate: Box::new(verb_phrase(Vocab::Cast, VerbSlot::Infinitive, vec![])),
                    }),
                ],
            ),
        ));
        assert_eq!(
            source_free(&spells_cost, "Test Card", false),
            "Spells cost {1} less to cast."
        );

        let hour = paragraph_ability(simple_with_auxiliaries(
            Some(Subject::NounPhrase(nominal(
                Some(Determiner::Indefinite(IndefiniteArticle::An)),
                vec![],
                NounInstance::Singular(Noun::Word(Vocab::Hour)),
                vec![],
            ))),
            vec![AuxiliaryInstance {
                auxiliary: Auxiliary::Have,
                inflection: AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
                contracted_negation: false,
            }],
            verb_phrase(Vocab::Pass, VerbSlot::PastParticiple, vec![]),
        ));
        assert_eq!(
            source_free(&hour, "Test Card", false),
            "An hour has passed."
        );

        let opponents = nominal(
            Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::You))),
            vec![],
            NounInstance::Plural(Noun::Word(Vocab::Opponent)),
            vec![],
        );
        let cannot_cast = paragraph_ability(simple_with_auxiliaries(
            Some(Subject::NounPhrase(opponents)),
            vec![AuxiliaryInstance {
                auxiliary: Auxiliary::Can,
                inflection: AuxiliaryInflection::Base,
                contracted_negation: true,
            }],
            verb_phrase(
                Vocab::Cast,
                VerbSlot::Infinitive,
                vec![VerbDependent::DirectObject(nominal(
                    None,
                    vec![],
                    NounInstance::Plural(Noun::Word(Vocab::Spell)),
                    vec![],
                ))],
            ),
        ));
        assert_eq!(
            source_free(&cannot_cast, "Test Card", false),
            "Your opponents can't cast spells."
        );
        assert_eq!(
            Determiner::Possessive(Possessor::Pronoun(Pronoun::You))
                .render()
                .unwrap(),
            "your"
        );
    }

    #[test]
    fn noun_modifiers_relative_clauses_and_shared_subjects_keep_their_structure() {
        let catalogs = fixture_catalogs();
        let NounPhrase::Nominal(creatures) = controlled_creature(&catalogs, false, true) else {
            panic!("controlled creature fixture must be nominal");
        };
        let first = Clause::Simple(SimpleClause {
            subject: Some(Subject::NounPhrase(nominal(
                None,
                vec![
                    NominalModifier::Adjective(AdjectivePhrase {
                        head: Adjective::Word(Vocab::Other),
                        complements: vec![],
                    }),
                    NominalModifier::Noun(catalog_noun(&catalogs, "Goblin", false)),
                ],
                creatures.head,
                creatures.complements,
            ))),
            predicate: verb_phrase(
                Vocab::Get,
                THIRD_PLURAL_PRESENT,
                vec![VerbDependent::PredicateComplement(Phrase::PowerToughness(
                    PowerToughness {
                        power: SignedScalar {
                            sign: ScalarSign::Plus,
                            value: ScalarValue::Integer(1),
                        },
                        toughness: SignedScalar {
                            sign: ScalarSign::Plus,
                            value: ScalarValue::Integer(1),
                        },
                    },
                ))],
            ),
        });
        let second = Clause::Simple(SimpleClause {
            subject: None,
            predicate: verb_phrase(
                Vocab::Have,
                THIRD_PLURAL_PRESENT,
                vec![VerbDependent::PredicateComplement(Phrase::CatalogAtom(
                    keyword_atom(&catalogs, "haste"),
                ))],
            ),
        });
        let ast = paragraph_ability(Clause::Coordinated(CoordinatedClause {
            first: Box::new(first),
            rest: vec![ClauseCoordination {
                conjunction: PredicateConjunction::And,
                comma: false,
                clause: second,
            }],
        }));

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Other Goblin creatures you control get +1/+1 and have haste."
        );
    }

    #[test]
    fn target_and_negated_relative_clause_ambiguity_is_resolved_in_the_ast() {
        let catalogs = fixture_catalogs();
        let ast = paragraph_ability(simple(
            Some(Subject::NounPhrase(controlled_creature(
                &catalogs, false, false,
            ))),
            verb_phrase(
                Vocab::Fight,
                THIRD_SINGULAR_PRESENT,
                vec![VerbDependent::DirectObject(controlled_creature(
                    &catalogs, true, false,
                ))],
            ),
        ));

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Target creature you control fights target creature you don't control."
        );
    }

    #[test]
    fn self_references_expand_without_source_text() {
        let draw_effect = Paragraph {
            sentences: vec![Sentence {
                clause: simple(
                    None,
                    verb_phrase(
                        Vocab::Draw,
                        VerbSlot::Imperative,
                        vec![VerbDependent::DirectObject(nominal(
                            Some(Determiner::Indefinite(IndefiniteArticle::A)),
                            vec![],
                            NounInstance::Singular(Noun::Word(Vocab::Card)),
                            vec![],
                        ))],
                    ),
                ),
                ending: SentenceEnding::Period(1),
            }],
        };
        let triggered = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Triggered(TriggeredAbility {
                    introducer: TriggerWord::Whenever,
                    event: SimpleClause {
                        subject: Some(Subject::NounPhrase(NounPhrase::ThisCard(
                            ThisCardForm::AbbreviatedName,
                        ))),
                        predicate: verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                    },
                    effect: draw_effect,
                }),
            }],
        };
        assert_eq!(
            source_free(&triggered, "Aang, A Lot to Learn", true),
            "Whenever Aang attacks, draw a card."
        );

        let cards = nominal(
            None,
            vec![],
            NounInstance::Plural(Noun::Word(Vocab::Card)),
            vec![NominalComplement::Prepositional(PrepositionalPhrase {
                preposition: Preposition::In,
                object: Box::new(Phrase::NounPhrase(Box::new(nominal(
                    Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::You))),
                    vec![],
                    NounInstance::Singular(Noun::Word(Vocab::Hand)),
                    vec![],
                )))),
            })],
        );
        let number = nominal(
            Some(Determiner::The),
            vec![],
            NounInstance::Singular(Noun::Word(Vocab::Number)),
            vec![NominalComplement::Prepositional(PrepositionalPhrase {
                preposition: Preposition::Of,
                object: Box::new(Phrase::NounPhrase(Box::new(cards))),
            })],
        );
        let full_name = paragraph_ability(simple(
            Some(Subject::NounPhrase(nominal(
                Some(Determiner::Possessive(Possessor::NounPhrase(Box::new(
                    NounPhrase::ThisCard(ThisCardForm::FullName),
                )))),
                vec![],
                NounInstance::Mass(Noun::Word(Vocab::Power)),
                vec![],
            ))),
            verb_phrase(
                Vocab::Be,
                THIRD_SINGULAR_PRESENT,
                vec![VerbDependent::PredicateComplement(Phrase::AdjectivePhrase(
                    Box::new(AdjectivePhrase {
                        head: Adjective::Word(Vocab::Equal),
                        complements: vec![AdjectiveComplement::Prepositional(
                            PrepositionalPhrase {
                                preposition: Preposition::To,
                                object: Box::new(Phrase::NounPhrase(Box::new(number))),
                            },
                        )],
                    }),
                ))],
            ),
        ));
        assert_eq!(
            source_free(&full_name, "Aang, A Lot to Learn", true),
            "Aang, A Lot to Learn's power is equal to the number of cards in your hand."
        );
    }

    #[test]
    fn triggered_modal_headers_render_in_their_enclosing_sentence_context() {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Modal(ModalAbility {
                    frame: ModalFrame::Triggered {
                        introducer: TriggerWord::Whenever,
                        event: SimpleClause {
                            subject: Some(Subject::NounPhrase(NounPhrase::ThisCard(
                                ThisCardForm::AbbreviatedName,
                            ))),
                            predicate: verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                        },
                    },
                    header: Paragraph {
                        sentences: vec![Sentence {
                            clause: simple(
                                None,
                                verb_phrase(
                                    Vocab::Choose,
                                    VerbSlot::Imperative,
                                    vec![VerbDependent::Scalar(Phrase::NumberLiteral(cardinal(1)))],
                                ),
                            ),
                            ending: SentenceEnding::None,
                        }],
                    },
                    header_suffix: ModalHeaderSuffix::SpacedEmDash,
                    modes: vec![],
                }),
            }],
        };

        assert_eq!(
            source_free(&ast, "Aang, A Lot to Learn", true),
            "Whenever Aang attacks, choose one —"
        );
    }

    fn source_free(ast: &OracleText, name: &str, legendary: bool) -> String {
        ast.render(name, legendary).unwrap()
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::new(
            ["Flying", "Deathtouch", "Haste"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        )
        .with_catalog(CatalogKind::CreatureType, ["Goblin"])
        .with_catalog(CatalogKind::CardType, ["Creature"])
    }

    fn keyword_atom(catalogs: &Catalogs, surface: &str) -> crate::catalog::CatalogAtom {
        let mut matches = catalogs.matches(surface, CatalogSlot::AbilityItem);
        assert_eq!(matches.len(), 1);
        let CatalogValue::Atom(atom) = matches.remove(0).value else {
            panic!("expected keyword ability {surface:?}");
        };
        atom
    }

    fn catalog_noun(catalogs: &Catalogs, surface: &str, plural: bool) -> NounInstance {
        catalogs
            .matches(surface, CatalogSlot::Noun(NounUsage::Count))
            .into_iter()
            .find_map(|catalog_match| match catalog_match.value {
                CatalogValue::Word(WordMatch::Noun(noun @ NounInstance::Plural(_))) if plural => {
                    Some(noun)
                }
                CatalogValue::Word(WordMatch::Noun(noun @ NounInstance::Singular(_)))
                    if !plural =>
                {
                    Some(noun)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("expected catalog noun {surface:?}"))
    }

    fn nominal(
        determiner: Option<Determiner>,
        modifiers: Vec<NominalModifier>,
        head: NounInstance,
        complements: Vec<NominalComplement>,
    ) -> NounPhrase {
        NounPhrase::Nominal(NominalPhrase {
            determiner,
            modifiers,
            head,
            complements,
        })
    }

    fn verb_phrase(vocab: Vocab, slot: VerbSlot, dependents: Vec<VerbDependent>) -> VerbPhrase {
        VerbPhrase {
            auxiliaries: vec![],
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(vocab),
                slot,
            },
            dependents,
        }
    }

    fn simple(subject: Option<Subject>, predicate: VerbPhrase) -> Clause {
        Clause::Simple(SimpleClause { subject, predicate })
    }

    fn simple_with_auxiliaries(
        subject: Option<Subject>,
        auxiliaries: Vec<AuxiliaryInstance>,
        mut predicate: VerbPhrase,
    ) -> Clause {
        predicate.auxiliaries = auxiliaries;
        simple(subject, predicate)
    }

    fn paragraph_ability(clause: Clause) -> OracleText {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    sentences: vec![Sentence {
                        clause,
                        ending: SentenceEnding::Period(1),
                    }],
                }),
            }],
        }
    }

    fn controlled_creature(catalogs: &Catalogs, negated: bool, plural: bool) -> NounPhrase {
        let relative_predicate = if negated {
            let mut predicate = verb_phrase(Vocab::Control, VerbSlot::Infinitive, vec![]);
            predicate.auxiliaries.push(AuxiliaryInstance {
                auxiliary: Auxiliary::Do,
                inflection: AuxiliaryInflection::Present {
                    person: Person::Second,
                    number: Number::Singular,
                },
                contracted_negation: true,
            });
            predicate
        } else {
            verb_phrase(Vocab::Control, SECOND_SINGULAR_PRESENT, vec![])
        };
        let surface = if plural { "creatures" } else { "creature" };
        nominal(
            (!plural).then_some(Determiner::Target(None)),
            vec![],
            catalog_noun(catalogs, surface, plural),
            vec![NominalComplement::Relative(RelativeClause {
                clause: Box::new(simple(
                    Some(Subject::NounPhrase(NounPhrase::Pronoun {
                        pronoun: Pronoun::You,
                        case: PronounCase::Subject,
                    })),
                    relative_predicate,
                )),
            })],
        )
    }

    fn cardinal(value: i32) -> NumberLiteral {
        NumberLiteral {
            value,
            numeral: Numeral::Cardinal,
        }
    }
}
