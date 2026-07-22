use std::fmt;

use crate::catalog::CatalogKind;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AttachmentPosition;
use crate::syntax::Clause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CopularComplement;
use crate::syntax::CopularPredicate;
use crate::syntax::Cost;
use crate::syntax::Demonstrative;
use crate::syntax::DependentClause;
use crate::syntax::Determiner;
use crate::syntax::EllipticalClause;
use crate::syntax::ExistentialClause;
use crate::syntax::ExistentialForm;
use crate::syntax::IndefiniteArticle;
use crate::syntax::IndependentClause;
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
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateConjunction;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::QuotedAbility;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeMarker;
use crate::syntax::ScalarSign;
use crate::syntax::ScalarValue;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::SentenceEnding;
use crate::syntax::SignedScalar;
use crate::syntax::Subject;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::syntax::ThisCardForm;
use crate::syntax::TriggerEvent;
use crate::syntax::TriggerWord;
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
            Self::Another => Ok("another".to_owned()),
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
            Self::Any => Ok("any".to_owned()),
            Self::No => Ok("no".to_owned()),
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
        let body = self.ability_kind(&ability.kind, capitalize)?;
        let rendered = if let Some(ability_word) = &ability.ability_word {
            format!("{} — {}", ability_word.spelling(), capitalize_first(body))
        } else {
            body
        };
        Ok(if capitalize { capitalize_first(rendered) } else { rendered })
    }

    fn ability_kind(&self, kind: &AbilityKind, capitalize: bool) -> Result<String, RenderError> {
        match kind {
            AbilityKind::Activated(activated) => Ok(format!(
                "{}: {}",
                self.cost(&activated.cost)?,
                self.paragraph(&activated.effect, activated.effect_initial_uppercase,)?
            )),
            AbilityKind::Triggered(triggered) => Ok(format!(
                "{}, {}",
                self.trigger_frame(
                    triggered.introducer,
                    &triggered.event,
                    triggered.intervening_condition.as_ref(),
                )?,
                self.paragraph(&triggered.effect, false)?
            )),
            AbilityKind::Loyalty(loyalty) => Ok(format!(
                "[{}]: {}",
                render_loyalty_cost(loyalty.cost),
                self.paragraph(&loyalty.effect, true)?
            )),
            AbilityKind::Modal(modal) => self.modal_ability(modal),
            AbilityKind::Keyword(keyword) => self.keyword_ability_list(keyword),
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph, capitalize),
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
            ModalFrame::Triggered {
                introducer,
                event,
                intervening_condition,
            } => format!(
                "{}, {header}",
                self.trigger_frame(*introducer, event, intervening_condition.as_ref())?
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

    fn trigger_frame(
        &self,
        introducer: TriggerWord,
        event: &TriggerEvent,
        intervening_condition: Option<&DependentClause>,
    ) -> Result<String, RenderError> {
        let event = match event {
            TriggerEvent::Clause(clause) => self.independent_clause(clause)?,
            TriggerEvent::Temporal(phrase) => self.noun_phrase(phrase)?,
        };
        let mut rendered = format!("{} {event}", render_trigger_word(introducer));
        if let Some(condition) = intervening_condition {
            rendered.push_str(", ");
            rendered.push_str(&self.dependent_clause(condition)?);
        }
        Ok(rendered)
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
            rendered.push_str(item.ability.spelling());
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
        let mut rendered = String::new();
        let mut saw_lexical_component = false;
        for (index, component) in cost.components.iter().enumerate() {
            if index > 0 {
                rendered.push_str(", ");
            }
            let is_symbol = matches!(
                component,
                Phrase::OracleSymbol(_) | Phrase::SymbolSequence(_)
            );
            let starts_action = matches!(
                component,
                Phrase::Clause(clause)
                    if matches!(
                        clause.as_ref(),
                        Clause::Independent(IndependentClause::Imperative(_))
                    )
            );
            let capitalize = starts_action || (!saw_lexical_component && !is_symbol);
            let component = self.phrase(component)?;
            rendered.push_str(&if capitalize { capitalize_first(component) } else { component });
            saw_lexical_component |= !is_symbol;
        }
        Ok(rendered)
    }

    fn paragraph(
        &self,
        paragraph: &Paragraph,
        capitalize_first_sentence: bool,
    ) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for (index, sentence) in paragraph.sentences.iter().enumerate() {
            let sentence = self.sentence(sentence, capitalize_first_sentence || index > 0)?;
            let is_closing_punctuation = sentence.chars().next().is_some_and(|character| {
                matches!(
                    character,
                    '\'' | '"' | ')' | ']' | ',' | ';' | ':' | '.' | '!' | '?'
                )
            });
            if !rendered.is_empty() && !is_closing_punctuation {
                rendered.push(' ');
            }
            rendered.push_str(&sentence);
        }
        Ok(rendered)
    }

    fn sentence(&self, sentence: &Sentence, capitalize: bool) -> Result<String, RenderError> {
        let (body, capitalize) = match &sentence.body {
            SentenceBody::Independent(clause) => (self.independent_clause(clause)?, capitalize),
            SentenceBody::Unknown(unknown) => (self.expand_self_references(&unknown.0), false),
        };
        let mut rendered = if capitalize { capitalize_first(body) } else { body };
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
            Clause::Independent(clause) => self.independent_clause(clause),
            Clause::Dependent(clause) => self.dependent_clause(clause),
        }
    }

    fn independent_clause(&self, clause: &IndependentClause) -> Result<String, RenderError> {
        match clause {
            IndependentClause::Transitive(subject, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.transitive_predicate(predicate)?,
            ])),
            IndependentClause::Intransitive(subject, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.intransitive_predicate(predicate)?,
            ])),
            IndependentClause::Copular(subject, predicate) => {
                self.copular_clause(subject, predicate)
            }
            IndependentClause::Passive(subject, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.passive_predicate(predicate)?,
            ])),
            IndependentClause::Imperative(predicate) => self.predicate(predicate),
            IndependentClause::Deontic(subject, modal, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.render_auxiliary(modal.auxiliary)?,
                self.predicate(predicate)?,
            ])),
            IndependentClause::Existential(existential) => self.existential_clause(existential),
            IndependentClause::Proform(subject, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.render_auxiliary(predicate.auxiliary)?,
            ])),
            IndependentClause::Complex(complex) => {
                let mut rendered = String::new();
                for attachment in complex
                    .attachments
                    .iter()
                    .filter(|attachment| attachment.position == AttachmentPosition::BeforeMatrix)
                {
                    rendered.push_str(&self.dependent_clause(&attachment.clause)?);
                    if attachment.comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                }
                rendered.push_str(&self.independent_clause(&complex.matrix)?);
                for attachment in complex
                    .attachments
                    .iter()
                    .filter(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
                {
                    if attachment.comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    rendered.push_str(&self.dependent_clause(&attachment.clause)?);
                }
                Ok(rendered)
            }
            IndependentClause::Coordinated(coordinated) => {
                let mut rendered = self.independent_clause(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    if coordination.comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    rendered.push_str(render_predicate_conjunction(coordination.conjunction));
                    rendered.push(' ');
                    match &coordination.member {
                        CoordinatedClauseMember::Independent(clause) => {
                            rendered.push_str(&self.independent_clause(clause)?);
                        }
                        CoordinatedClauseMember::SharedPredicate(predicate) => {
                            rendered.push_str(&self.predicate(predicate)?);
                        }
                    }
                }
                Ok(rendered)
            }
        }
    }

    fn subject(&self, subject: &Subject) -> Result<String, RenderError> {
        self.noun_phrase(&subject.0)
    }

    fn predicate(&self, predicate: &Predicate) -> Result<String, RenderError> {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate),
            Predicate::Intransitive(predicate) => self.intransitive_predicate(predicate),
            Predicate::Copular(predicate) => {
                let mut parts = vec![
                    self.render_auxiliary(predicate.copula.auxiliary)?,
                    self.copular_complement(&predicate.complement)?,
                ];
                for adjunct in &predicate.adjuncts {
                    parts.push(self.predicate_adjunct(adjunct)?);
                }
                Ok(join_words(parts))
            }
            Predicate::Passive(predicate) => self.passive_predicate(predicate),
            Predicate::Proform(predicate) => self.render_auxiliary(predicate.auxiliary),
        }
    }

    fn transitive_predicate(
        &self,
        predicate: &crate::syntax::TransitivePredicate,
    ) -> Result<String, RenderError> {
        let mut parts = vec![
            self.predicate_head(&predicate.head)?,
            self.predicate_object(&predicate.object)?,
        ];
        self.extend_predicate_elements(&mut parts, &predicate.elements)?;
        Ok(join_words(parts))
    }

    fn intransitive_predicate(
        &self,
        predicate: &crate::syntax::IntransitivePredicate,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.predicate_head(&predicate.head)?];
        self.extend_predicate_elements(&mut parts, &predicate.elements)?;
        Ok(join_words(parts))
    }

    fn passive_predicate(
        &self,
        predicate: &crate::syntax::PassivePredicate,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.predicate_head(&predicate.head)?];
        self.extend_predicate_elements(&mut parts, &predicate.elements)?;
        Ok(join_words(parts))
    }

    fn predicate_head(&self, head: &PredicateHead) -> Result<String, RenderError> {
        let mut parts =
            Vec::with_capacity(head.auxiliaries.len() + head.preverb_modifiers.len() + 1);
        for &auxiliary in &head.auxiliaries {
            parts.push(self.render_auxiliary(auxiliary)?);
        }
        parts.extend(head.preverb_modifiers.iter().map(|modifier| {
            match modifier {
                PreverbModifier::Not => "not",
                PreverbModifier::Also => "also",
            }
            .to_owned()
        }));
        parts.push(
            self.vocabulary
                .render_verb_instance(&head.verb)
                .ok_or(RenderError::MissingLexicalForm("verb"))?,
        );
        Ok(join_words(parts))
    }

    fn render_auxiliary(
        &self,
        auxiliary: crate::word::AuxiliaryInstance,
    ) -> Result<String, RenderError> {
        self.vocabulary
            .render_auxiliary(auxiliary)
            .map(str::to_owned)
            .ok_or(RenderError::MissingLexicalForm("auxiliary"))
    }

    fn extend_predicate_elements(
        &self,
        parts: &mut Vec<String>,
        elements: &[PredicateElement],
    ) -> Result<(), RenderError> {
        for element in elements {
            parts.push(match element {
                PredicateElement::Complement(complement) => {
                    self.predicate_complement(complement)?
                }
                PredicateElement::Adjunct(adjunct) => self.predicate_adjunct(adjunct)?,
            });
        }
        Ok(())
    }

    fn predicate_object(&self, object: &PredicateObject) -> Result<String, RenderError> {
        match object {
            PredicateObject::NounPhrase(phrase) => self.noun_phrase(phrase),
            PredicateObject::Ability(ability) => {
                let mut rendered = render_catalog_atom(&ability.ability);
                if let Some(argument) = &ability.argument {
                    rendered.push(' ');
                    rendered.push_str(&self.predicate_object(argument)?);
                }
                Ok(rendered)
            }
            PredicateObject::Quantity(quantity) => Ok(render_quantity(*quantity)),
            PredicateObject::OracleSymbol(symbol) => Ok(symbol.as_str().to_owned()),
            PredicateObject::PowerToughness(value) => Ok(format!(
                "{}/{}",
                render_signed_scalar(value.power),
                render_signed_scalar(value.toughness),
            )),
            PredicateObject::EmbeddedAbility(ability) => self.ability(ability, true),
            PredicateObject::QuotedAbility(quoted) => self.quoted_ability(quoted),
        }
    }

    fn predicate_complement(
        &self,
        complement: &PredicateComplement,
    ) -> Result<String, RenderError> {
        match complement {
            PredicateComplement::IndirectObject(phrase) => self.noun_phrase(phrase),
            PredicateComplement::Adjective(phrase) => self.adjective_phrase(phrase),
            PredicateComplement::Prepositional(phrase) => self.prepositional_phrase(phrase),
            PredicateComplement::Infinitive(clause) => self.infinitive_clause(clause),
        }
    }

    fn predicate_adjunct(&self, adjunct: &PredicateAdjunct) -> Result<String, RenderError> {
        match adjunct {
            PredicateAdjunct::Adverb(adverb) => Ok(adverb.spelling().to_owned()),
            PredicateAdjunct::Temporal(phrase) => self.noun_phrase(phrase),
            PredicateAdjunct::Prepositional(phrase) => self.prepositional_phrase(phrase),
            PredicateAdjunct::Dependent(clause) => self.dependent_clause(clause),
        }
    }

    fn copular_clause(
        &self,
        subject: &Subject,
        predicate: &CopularPredicate,
    ) -> Result<String, RenderError> {
        let subject = self.subject(subject)?;
        let complement = self.copular_complement(&predicate.complement)?;
        let mut parts = Vec::with_capacity(predicate.adjuncts.len() + 2);
        if predicate.copula.contracted_with_subject {
            let auxiliary = self.render_auxiliary(predicate.copula.auxiliary)?;
            parts.push(format!("{subject}{}", contraction_suffix(&auxiliary)?));
        } else {
            parts.push(subject);
            parts.push(self.render_auxiliary(predicate.copula.auxiliary)?);
        }
        parts.push(complement);
        for adjunct in &predicate.adjuncts {
            parts.push(self.predicate_adjunct(adjunct)?);
        }
        Ok(join_words(parts))
    }

    fn copular_complement(&self, complement: &CopularComplement) -> Result<String, RenderError> {
        match complement {
            CopularComplement::NounPhrase(phrase) => self.noun_phrase(phrase),
            CopularComplement::Adjective(phrase) => self.adjective_phrase(phrase),
            CopularComplement::Prepositional(phrase) => self.prepositional_phrase(phrase),
            CopularComplement::CatalogAtom(atom) => Ok(render_catalog_atom(atom)),
        }
    }

    fn existential_clause(&self, clause: &ExistentialClause) -> Result<String, RenderError> {
        let opening = match clause.form {
            ExistentialForm::Is => "there is",
            ExistentialForm::ContractedIs => "there's",
            ExistentialForm::Are => "there are",
            ExistentialForm::Was => "there was",
            ExistentialForm::Were => "there were",
        };
        let mut parts = vec![opening.to_owned(), self.noun_phrase(&clause.pivot)?];
        for adjunct in &clause.adjuncts {
            parts.push(self.predicate_adjunct(adjunct)?);
        }
        Ok(join_words(parts))
    }

    fn dependent_clause(&self, clause: &DependentClause) -> Result<String, RenderError> {
        match clause {
            DependentClause::Subordinate(subordinator, body) => {
                let body = match body {
                    SubordinateBody::Finite(clause) => self.independent_clause(clause)?,
                    SubordinateBody::Elliptical(EllipticalClause::Adjective(phrase)) => {
                        self.adjective_phrase(phrase)?
                    }
                };
                Ok(format!("{} {body}", render_subordinator(*subordinator)))
            }
            DependentClause::Relative(relative) => self.relative_clause(relative),
            DependentClause::Infinitive(infinitive) => self.infinitive_clause(infinitive),
        }
    }

    fn infinitive_clause(&self, clause: &InfinitiveClause) -> Result<String, RenderError> {
        let predicate = self.predicate(&clause.predicate)?;
        Ok(match clause.marker {
            InfinitiveMarker::Bare => predicate,
            InfinitiveMarker::To => format!("to {predicate}"),
        })
    }

    fn relative_clause(&self, clause: &RelativeClause) -> Result<String, RenderError> {
        let marker = match clause.marker {
            RelativeMarker::That => "that",
            RelativeMarker::Which => "which",
            RelativeMarker::Who => "who",
            RelativeMarker::Zero => "",
        };
        let body = match &clause.body {
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate)?,
            RelativeBody::ObjectGap { subject, predicate } => {
                let mut parts = vec![
                    self.subject(subject)?,
                    self.predicate_head(&predicate.head)?,
                ];
                self.extend_predicate_elements(&mut parts, &predicate.elements)?;
                join_words(parts)
            }
        };
        Ok(join_words(vec![marker.to_owned(), body]))
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
                NominalModifier::PowerToughness(value) => format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness),
                ),
                NominalModifier::Unknown(unknown) => self.expand_self_references(&unknown.0),
            });
        }
        parts.push(self.render_noun(&phrase.head)?);
        for complement in &phrase.complements {
            parts.push(match complement {
                NominalComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition)?
                }
                NominalComplement::Relative(relative) => self.relative_clause(relative)?,
                NominalComplement::Unknown(unknown) => self.expand_self_references(&unknown.0),
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
                NominalModifier::PowerToughness(_) => Ok(InitialSound::Consonant),
                NominalModifier::Unknown(unknown) => Ok(spelling_initial_sound(
                    &self.expand_self_references(&unknown.0),
                )),
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
            Noun::Unknown(unknown) => Ok(spelling_initial_sound(
                &self.expand_self_references(&unknown.0),
            )),
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
                AdjectiveComplement::Unknown(unknown) => self.expand_self_references(&unknown.0),
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
            Phrase::Clause(clause) => self.clause(clause),
            Phrase::NounPhrase(noun_phrase) => self.noun_phrase(noun_phrase),
            Phrase::AdjectivePhrase(adjective) => self.adjective_phrase(adjective),
            Phrase::PrepositionalPhrase(preposition) => self.prepositional_phrase(preposition),
            Phrase::Quantity(quantity) => Ok(render_quantity(*quantity)),
            Phrase::Adverb(adverb) => Ok(adverb.spelling().to_owned()),
            Phrase::CatalogAtom(atom) => Ok(render_catalog_atom(atom)),
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
            Phrase::EmbeddedAbility(ability) => self.ability(ability, true),
            Phrase::QuotedAbility(quoted) => self.quoted_ability(quoted),
            Phrase::UnknownPhrase(unknown) => Ok(self.expand_self_references(&unknown.0)),
        }
    }

    fn quoted_ability(&self, quoted: &QuotedAbility) -> Result<String, RenderError> {
        let mut rendered = format!(
            "\"{}",
            self.ability(&quoted.ability, quoted.initial_uppercase)?
        );
        if quoted.closed {
            rendered.push('"');
        }
        Ok(rendered)
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
        Quantity::AtLeast(number) => {
            format!("{} or more", number.numeral.format(number.value))
        }
        Quantity::UpTo(number) => format!("up to {}", number.numeral.format(number.value)),
        Quantity::ThatMany => "that many".to_owned(),
        Quantity::ThatMuch => "that much".to_owned(),
    }
}

fn render_catalog_atom(atom: &crate::catalog::CatalogAtom) -> String {
    match atom.kind {
        CatalogKind::KeywordAbility | CatalogKind::KeywordAction | CatalogKind::AbilityWord => {
            atom.spelling().to_owned()
        }
        _ => atom.render_adjective(),
    }
}

fn contraction_suffix(auxiliary: &str) -> Result<&'static str, RenderError> {
    match auxiliary {
        "am" => Ok("'m"),
        "are" => Ok("'re"),
        "is" => Ok("'s"),
        _ => Err(RenderError::MissingLexicalForm("copular contraction")),
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
        Subordinator::As => "as",
        Subordinator::While => "while",
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
        Preposition::Among => "among",
        Preposition::As => "as",
        Preposition::At => "at",
        Preposition::Before => "before",
        Preposition::By => "by",
        Preposition::During => "during",
        Preposition::For => "for",
        Preposition::From => "from",
        Preposition::In => "in",
        Preposition::Into => "into",
        Preposition::Of => "of",
        Preposition::On => "on",
        Preposition::Onto => "onto",
        Preposition::To => "to",
        Preposition::Until => "until",
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

    #[derive(Debug)]
    struct VerbPhrase {
        auxiliaries: Vec<AuxiliaryInstance>,
        preverb_modifiers: Vec<PreverbModifier>,
        verb: VerbInstance,
        dependents: Vec<VerbDependent>,
    }

    #[derive(Debug)]
    enum VerbDependent {
        DirectObject(NounPhrase),
        PredicateComplement(Phrase),
        Scalar(Phrase),
        Adverbial(Phrase),
        Infinitive(InfinitiveMarker, Box<VerbPhrase>),
    }

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
    fn catalog_phrases_preserve_their_matched_surface_conventions() {
        let catalogs = Catalogs::new(
            [
                "For Mirrodin!",
                "Choose a background",
                "Cumulative upkeep",
                "Bands with other legendary creatures",
            ],
            ["Time Travel"],
            std::iter::empty::<&str>(),
        )
        .with_catalog(CatalogKind::CardType, ["Creature"]);

        for source in [
            "For Mirrodin!",
            "Choose a Background",
            "Time travel.",
            "Cumulative upkeep—Pay 1 life.",
            "Creatures you control have \"bands with other legendary creatures.\"",
        ] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn cost_components_capitalize_only_independent_actions() {
        let catalogs = fixture_catalogs();

        for source in [
            "{2}{W}, {T}, Sacrifice a green creature, a white creature, and a blue creature: Draw a card.",
            "{1}{B}, Pay 2 life, Sacrifice a creature: Draw a card.",
            "Pay half your life, rounded up: Draw a card.",
            "Sacrifice an artifact, creature, or land: Draw a card.",
            "Sacrifice a creature named Feral Shadow, a creature named Breathstealer, and this creature: Draw a card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn opaque_closing_quote_does_not_gain_sentence_spacing() {
        let catalogs = fixture_catalogs()
            .with_catalog(CatalogKind::CardType, ["Emblem", "Land"])
            .with_catalog(CatalogKind::LandType, ["Mountain"]);
        let source = "You get an emblem with \"Mountains you control have '{T}: This land deals 1 damage to any target.'\"";

        let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn quoted_abilities_preserve_explicit_initial_case() {
        let catalogs = fixture_catalogs();

        for source in [
            "It has \"bands with other creatures.\"",
            "It has \"enchant creature put onto the battlefield.\"",
            "It has \"Whenever this creature attacks, draw a card.\"",
        ] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn activated_effect_preserves_explicit_initial_case() {
        let source = "Exile a card: this creature gets +2/+2 until end of turn.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn mid_sentence_proper_nouns_do_not_lose_their_case() {
        let source = "Exile artifacts named Eye of Vecna and Hand of Vecna: Draw a card.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn capitalized_subtypes_do_not_start_coordinated_imperatives() {
        let catalogs = fixture_catalogs()
            .with_catalog(CatalogKind::KeywordAction, ["Food"])
            .with_catalog(CatalogKind::ArtifactType, ["Food"]);

        for source in [
            "Enchant creature or Food",
            "Return target creature or Food card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
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
            Some(Subject(nominal(
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
                    VerbDependent::Infinitive(
                        InfinitiveMarker::To,
                        Box::new(verb_phrase(Vocab::Cast, VerbSlot::Infinitive, vec![])),
                    ),
                ],
            ),
        ));
        assert_eq!(
            source_free(&spells_cost, "Test Card", false),
            "Spells cost {1} less to cast."
        );

        let hour = paragraph_ability(simple_with_auxiliaries(
            Some(Subject(nominal(
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
            Some(Subject(opponents)),
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
        let first = simple(
            Some(Subject(nominal(
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
            verb_phrase(
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
        );
        let second = strict_predicate(verb_phrase(
            Vocab::Have,
            THIRD_PLURAL_PRESENT,
            vec![VerbDependent::PredicateComplement(Phrase::CatalogAtom(
                keyword_atom(&catalogs, "haste"),
            ))],
        ));
        let Clause::Independent(first) = first else {
            panic!("first coordinated fixture must be independent");
        };
        let ast = paragraph_ability(Clause::Independent(IndependentClause::Coordinated(
            CoordinatedIndependentClause {
                first: Box::new(first),
                rest: vec![ClauseCoordination {
                    conjunction: PredicateConjunction::And,
                    comma: false,
                    member: CoordinatedClauseMember::SharedPredicate(second),
                }],
            },
        )));

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Other Goblin creatures you control get +1/+1 and have haste."
        );
    }

    #[test]
    fn target_and_negated_relative_clause_ambiguity_is_resolved_in_the_ast() {
        let catalogs = fixture_catalogs();
        let ast = paragraph_ability(simple(
            Some(Subject(controlled_creature(&catalogs, false, false))),
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
                body: sentence_body(simple(
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
                )),
                ending: SentenceEnding::Period(1),
            }],
        };
        let triggered = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Triggered(TriggeredAbility {
                    introducer: TriggerWord::Whenever,
                    event: TriggerEvent::Clause(independent(simple(
                        Some(Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName))),
                        verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                    ))),
                    intervening_condition: None,
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
            Some(Subject(nominal(
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
    fn opaque_fallbacks_expand_self_reference_sigils() {
        let ast = crate::parse("~ frobnitzes ~~.").into_ast();

        assert_eq!(
            source_free(&ast, "Aang, A Lot to Learn", true),
            "Aang frobnitzes Aang, A Lot to Learn."
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
                        event: TriggerEvent::Clause(independent(simple(
                            Some(Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName))),
                            verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                        ))),
                        intervening_condition: None,
                    },
                    header: Paragraph {
                        sentences: vec![Sentence {
                            body: sentence_body(simple(
                                None,
                                verb_phrase(
                                    Vocab::Choose,
                                    VerbSlot::Imperative,
                                    vec![VerbDependent::Scalar(Phrase::NumberLiteral(cardinal(1)))],
                                ),
                            )),
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
        let predicate = strict_predicate(predicate);
        let independent = match (subject, predicate) {
            (None, predicate) => IndependentClause::Imperative(predicate),
            (Some(subject), Predicate::Transitive(predicate)) => {
                IndependentClause::Transitive(subject, predicate)
            }
            (Some(subject), Predicate::Intransitive(predicate)) => {
                IndependentClause::Intransitive(subject, predicate)
            }
            (Some(subject), Predicate::Copular(predicate)) => {
                IndependentClause::Copular(subject, predicate)
            }
            (Some(subject), Predicate::Passive(predicate)) => {
                IndependentClause::Passive(subject, predicate)
            }
            (Some(subject), Predicate::Proform(predicate)) => {
                IndependentClause::Proform(subject, predicate)
            }
        };
        Clause::Independent(independent)
    }

    fn simple_with_auxiliaries(
        subject: Option<Subject>,
        auxiliaries: Vec<AuxiliaryInstance>,
        mut predicate: VerbPhrase,
    ) -> Clause {
        predicate.auxiliaries = auxiliaries;
        simple(subject, predicate)
    }

    fn independent(clause: Clause) -> IndependentClause {
        let Clause::Independent(clause) = clause else {
            panic!("fixture clause must be independent");
        };
        clause
    }

    fn sentence_body(clause: Clause) -> SentenceBody {
        SentenceBody::Independent(independent(clause))
    }

    fn paragraph_ability(clause: Clause) -> OracleText {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    sentences: vec![Sentence {
                        body: match clause {
                            Clause::Independent(clause) => SentenceBody::Independent(clause),
                            Clause::Dependent(_) => panic!("sentence fixture must be independent"),
                        },
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
        let Predicate::Intransitive(relative_predicate) = strict_predicate(relative_predicate)
        else {
            panic!("relative object-gap fixture must be intransitive before filling its gap");
        };
        nominal(
            (!plural).then_some(Determiner::Target(None)),
            vec![],
            catalog_noun(catalogs, surface, plural),
            vec![NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                body: RelativeBody::ObjectGap {
                    subject: Subject(NounPhrase::Pronoun {
                        pronoun: Pronoun::You,
                        case: PronounCase::Subject,
                    }),
                    predicate: ObjectGapPredicate {
                        head: relative_predicate.head,
                        elements: relative_predicate.elements,
                    },
                },
            })],
        )
    }

    fn cardinal(value: i32) -> NumberLiteral {
        NumberLiteral {
            value,
            numeral: Numeral::Cardinal,
        }
    }

    fn strict_predicate(phrase: VerbPhrase) -> Predicate {
        if phrase.verb.verb == Verb::Word(Vocab::Be) {
            let mut dependents = phrase.dependents.into_iter();
            let complement = match dependents.next().expect("copular complement") {
                VerbDependent::PredicateComplement(Phrase::AdjectivePhrase(phrase))
                | VerbDependent::Adverbial(Phrase::AdjectivePhrase(phrase)) => {
                    CopularComplement::Adjective(*phrase)
                }
                other => panic!("unsupported copular fixture: {other:?}"),
            };
            assert!(dependents.next().is_none());
            return Predicate::Copular(CopularPredicate {
                copula: Copula {
                    auxiliary: AuxiliaryInstance {
                        auxiliary: Auxiliary::Be,
                        inflection: auxiliary_inflection(phrase.verb.slot),
                        contracted_negation: false,
                    },
                    contracted_with_subject: false,
                },
                complement,
                adjuncts: vec![],
            });
        }

        let head = PredicateHead {
            auxiliaries: phrase.auxiliaries,
            preverb_modifiers: phrase.preverb_modifiers,
            verb: phrase.verb,
        };
        let mut object = None;
        let mut elements = Vec::new();
        for dependent in phrase.dependents {
            match dependent {
                VerbDependent::DirectObject(phrase) => {
                    object = Some(PredicateObject::NounPhrase(phrase));
                }
                VerbDependent::PredicateComplement(Phrase::PowerToughness(value)) => {
                    object = Some(PredicateObject::PowerToughness(value));
                }
                VerbDependent::PredicateComplement(Phrase::CatalogAtom(atom)) => {
                    object = Some(PredicateObject::Ability(AbilityObject {
                        ability: atom,
                        argument: None,
                    }));
                }
                VerbDependent::PredicateComplement(Phrase::AdjectivePhrase(phrase))
                | VerbDependent::Adverbial(Phrase::AdjectivePhrase(phrase)) => {
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::Adjective(*phrase),
                    ));
                }
                VerbDependent::Scalar(Phrase::OracleSymbol(symbol)) => {
                    object = Some(PredicateObject::OracleSymbol(symbol));
                }
                VerbDependent::Scalar(Phrase::NumberLiteral(number)) => {
                    object = Some(PredicateObject::Quantity(Quantity::Exact(number)));
                }
                VerbDependent::Infinitive(marker, predicate) => {
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::Infinitive(InfinitiveClause {
                            marker,
                            predicate: Box::new(strict_predicate(*predicate)),
                        }),
                    ));
                }
                other => panic!("unsupported predicate fixture: {other:?}"),
            }
        }
        match object {
            Some(object) => Predicate::Transitive(TransitivePredicate {
                head,
                object,
                elements,
            }),
            None => Predicate::Intransitive(IntransitivePredicate { head, elements }),
        }
    }

    fn auxiliary_inflection(slot: VerbSlot) -> AuxiliaryInflection {
        match slot {
            VerbSlot::Infinitive | VerbSlot::Imperative => AuxiliaryInflection::Base,
            VerbSlot::Present { person, number } => AuxiliaryInflection::Present { person, number },
            VerbSlot::Past { person, number } => AuxiliaryInflection::Past { person, number },
            VerbSlot::PresentParticiple => AuxiliaryInflection::PresentParticiple,
            VerbSlot::PastParticiple => AuxiliaryInflection::PastParticiple,
        }
    }
}
