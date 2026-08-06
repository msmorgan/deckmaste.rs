use std::cell::Cell;
use std::fmt;

use crate::catalog::CatalogKind;
use crate::features::Conjunction;
use crate::features::Number;
use crate::features::Onset as InitialSound;
use crate::features::Person;
use crate::identity::short_name;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AttachmentPosition;
use crate::syntax::ChapterAbility;
use crate::syntax::ChoiceInstruction;
use crate::syntax::Clause;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ComparisonMarker;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::CopularComplement;
use crate::syntax::CopularPredicate;
use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::DependentClause;
use crate::syntax::Determiner;
use crate::syntax::EllipticalClause;
use crate::syntax::ExceptionRider;
use crate::syntax::ExistentialClause;
use crate::syntax::FrequencyBound;
use crate::syntax::FrequencyCount;
use crate::syntax::FrequencyPhrase;
use crate::syntax::GerundClause;
use crate::syntax::IndefiniteArticle;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordArgumentSeparator;
use crate::syntax::KeywordCost;
use crate::syntax::KeywordListSeparator;
use crate::syntax::LevelBandAbility;
use crate::syntax::LevelRange;
use crate::syntax::LoyaltyCost;
use crate::syntax::LoyaltyCostSign;
use crate::syntax::LoyaltyCostValue;
use crate::syntax::ModalAbility;
use crate::syntax::ModalFrame;
use crate::syntax::ModalHeaderSuffix;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::OracleSymbol;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::Polarity;
use crate::syntax::Possessor;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateExpression;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicatedArgument;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::QuotedAbility;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeMarker;
use crate::syntax::RollRange;
use crate::syntax::RollRowAbility;
use crate::syntax::ScalarSign;
use crate::syntax::ScalarValue;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::SetExceptionMarker;
use crate::syntax::SignedScalar;
use crate::syntax::SimplePrepositionalPhrase;
use crate::syntax::StationThresholdAbility;
use crate::syntax::Subject;
use crate::syntax::SubordinateBody;
use crate::syntax::Subordinator;
use crate::syntax::ThisCardForm;
use crate::syntax::TransitivePredicate;
use crate::syntax::TriggerCondition;
use crate::syntax::TriggerConditionList;
use crate::syntax::TriggerEvent;
use crate::syntax::TriggerWord;
use crate::syntax::VerbParticle;
use crate::word::Adjective;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::PronounInstance;
use crate::word::Verb;
use crate::word::Vocab;
use crate::word::Vocabulary;
use crate::word::surface_initial_sound;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    MissingLexicalForm(&'static str),
    CardIdentityRequired,
    /// A [`NominalComplement::KeywordArgument`] carrying a `KeywordArgument`
    /// shape the syntax never licenses in nominal-complement position (only
    /// `Costed(Symbols)` and `Predicated` are licensed there) — `kwgrant`
    /// round.
    InvalidKeywordArgumentNominal,
    /// A hand-built `PredicateHead` claims both `distributive_each` and
    /// `first_auxiliary_contracted_with_subject`: the floating-`each`
    /// grammar can never construct that surface, since `each` intervenes
    /// between the subject and the first auxiliary — `qfloat` round.
    InvalidDistributiveEachContraction,
    /// A predicate, modifier, or clause carrier contains the nominal-only
    /// `plus` conjunction. This can be constructed only through the widened
    /// canonical compatibility alias, never by the grammar.
    InvalidPredicateConjunction(Conjunction),
    /// A nominal, noun-phrase, or prepositional carrier contains the
    /// predicate-only `then` conjunction. This can be constructed only
    /// through the widened canonical compatibility alias, never by the
    /// grammar.
    InvalidNominalConjunction(Conjunction),
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLexicalForm(kind) => write!(formatter, "missing {kind} form"),
            Self::CardIdentityRequired => {
                formatter.write_str("card identity is required to render this determiner")
            }
            Self::InvalidKeywordArgumentNominal => formatter
                .write_str("keyword argument shape is not licensed in nominal-complement position"),
            Self::InvalidDistributiveEachContraction => formatter.write_str(
                "a distributive-each predicate head cannot also contract its first auxiliary",
            ),
            Self::InvalidPredicateConjunction(conjunction) => write!(
                formatter,
                "{conjunction:?} is not licensed as a predicate conjunction"
            ),
            Self::InvalidNominalConjunction(conjunction) => write!(
                formatter,
                "{conjunction:?} is not licensed as a nominal conjunction"
            ),
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

/// Renders one [`Fragment`] through the production renderer, at the same
/// method the whole-card path reaches for that node kind.
///
/// The dispatch lives here, next to the private `Renderer`, so no render logic
/// is duplicated outside this module and no renderer internal has to widen its
/// visibility. Each arm's flags are the ones the whole-card path passes when
/// the node stands at the head of its ability:
///
/// - `Nominal` → `Renderer::noun_phrase`, the seam every nominal position
///   already routes through; a noun phrase never capitalizes itself.
/// - `Sentence` → `Renderer::sentence(_, capitalize: true, force_no_period:
///   false)`, matching `Renderer::paragraph`'s first sentence at ability top
///   level. The terminal period stays derived, so a sentence whose tail absorbs
///   it (a closing quoted ability, an Aura enchant line) renders without one
///   exactly as it does inside a card.
/// - `Cost` → `Renderer::cost`, which owns the `, ` joins and the
///   first-lexical-component capitalization.
/// - `KeywordLine` → `Renderer::keyword_ability_list(_, suppress_final_period:
///   false)` followed by [`capitalize_first`]. `AbilityKind::Keyword` is the
///   one ability kind whose renderer method does *not* take a `capitalize`
///   flag: `Renderer::ability` applies `capitalize_first` to the finished body
///   instead, and it is passed `capitalize: true` from every whole-card
///   position (`Renderer::oracle_text` at ability head, `nested_ability` for a
///   quoted or embedded one). Keyword atoms preserve their source spelling, so
///   omitting this step would render a lowercase-initial line (`first strike,
///   flying`) differently as a fragment than as a card.
/// - `Ability` → `Renderer::ability(_, capitalize: true, suppress_final_period:
///   false)`, byte-identical to what `Renderer::oracle_text` passes for a
///   single-line card.
///
/// # Position
///
/// A fragment renders as if it stood at the head of its own ability, because
/// that is the only position a fragment has. So the three kinds that *can*
/// head an ability — `Sentence`, `KeywordLine`, `Ability` — take the
/// capitalization a card gives them there, and `Nominal` does not, because no
/// nominal ever heads an ability: a sentence-initial nominal is capitalized by
/// `Renderer::sentence`, above the fragment.
///
/// One card-internal position differs from ability head: a **triggered
/// ability's effect** paragraph is rendered with
/// `capitalize_first_sentence: false`, so its first sentence stays lowercase
/// after the `When …, ` frame. Text that occupies that position must be
/// rendered as part of its `Ability`, not as a bare `Sentence`; the ability
/// renderer already threads the flag.
///
/// # Errors
///
/// Returns an error when the fragment requests a grammatical form its
/// vocabulary identity does not define, the same conditions
/// [`OracleText::render`] reports.
pub(crate) fn render_fragment(
    fragment: &crate::fragment::Fragment,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    use crate::fragment::Fragment;

    let renderer = Renderer::new(name, is_legendary);
    match fragment {
        Fragment::Nominal(noun_phrase) => renderer.noun_phrase(noun_phrase),
        Fragment::Sentence(sentence) => renderer.sentence(sentence, true, false),
        Fragment::Cost(cost) => renderer.cost(cost),
        Fragment::KeywordLine(list) => renderer
            .keyword_ability_list(list, false)
            .map(capitalize_first),
        Fragment::Ability(ability) => renderer.ability(ability, true, false),
    }
}

#[cfg(test)]
pub(crate) fn render_coordinated_noun_phrase(
    value: &crate::syntax::CoordinatedNounPhrase,
) -> Result<String, RenderError> {
    Renderer::new("", false).coordinated_noun_phrase(value)
}

#[cfg(test)]
pub(crate) fn render_coordinated_nominal_phrase(
    value: &crate::syntax::CoordinatedNominalPhrase,
) -> Result<String, RenderError> {
    Renderer::new("", false).coordinated_nominal_phrase(value)
}

impl Determiner {
    /// Renders a determiner that does not require card-name context.
    ///
    /// # Errors
    ///
    /// A noun-phrase possessor containing a self reference requires the
    /// [`OracleText::render`] identity arguments and is rejected here.
    pub fn render(&self) -> Result<String, RenderError> {
        if let Some(spelling) = self.closed_spelling() {
            return Ok(spelling.to_owned());
        }
        match self {
            Self::Possessive(Possessor::NounPhrase(_)) => Err(RenderError::CardIdentityRequired),
            Self::Target(None) => Ok("target".to_owned()),
            Self::Target(Some(quantity)) => Ok(format!("{} target", render_quantity(*quantity))),
            Self::Quantity(quantity) => Ok(render_quantity(*quantity)),
            Self::Indefinite => unreachable!(
                "indefinite article needs the following material's initial sound, which this \
                 determiner-only method has no access to; NominalPhrase/CoordinatedNominalPhrase \
                 derive it themselves before ever calling this"
            ),
            Self::The
            | Self::Each
            | Self::Another
            | Self::Demonstrative(_)
            | Self::Possessive(Possessor::Pronoun(_))
            | Self::All
            | Self::Any
            | Self::No => unreachable!("closed forms returned above"),
        }
    }
}

struct Renderer<'identity> {
    name: &'identity str,
    /// The face's derived shortened name, or `None` when it has none. An
    /// [`ThisCardForm::AbbreviatedName`] node is emitted only for a face that
    /// has one, so rendering it with the same identity always finds it here.
    short_name: Option<&'identity str>,
    vocabulary: Vocabulary,
    /// How many quoted/embedded abilities enclose the ability currently being
    /// rendered. The Aura enchant keyword line lacks a period only at the top
    /// level (depth 0); nested inside a quote it is ordinary prose that keeps
    /// its period (`becomes an Aura with "enchant creature …."`).
    nesting: Cell<usize>,
    /// The quoted ability that terminates the sentence currently being
    /// rendered, or `None` when that sentence ends in something else.
    ///
    /// Published by [`Self::sentence`] from the same AST tail walk that derives
    /// the sentence's period, and read by [`Self::quoted_ability`] to place
    /// that period *inside* the closing quote (`gains "…."`) rather than
    /// after it (`has "…" and "…."`). This is the channel that lets the
    /// placement be derived instead of stored: a quote cannot see its own
    /// position, but the sentence can see which quote is its tail.
    ///
    /// Held as a bare address so the channel borrows nothing from the tree
    /// being rendered. It is only ever compared by identity, never read
    /// through.
    terminal_quote: Cell<Option<*const QuotedAbility>>,
}

struct GeneratedCoordinationRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    pending_determiner: Option<Determiner>,
    skip_payload_subtrees: usize,
}

impl<'renderer, 'identity> GeneratedCoordinationRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            pending_determiner: None,
            skip_payload_subtrees: 0,
        }
    }

    fn push(&mut self, part: &str) {
        if part.is_empty() {
            return;
        }
        if part == "," {
            self.rendered.push(',');
        } else {
            if !self.rendered.is_empty() {
                self.rendered.push(' ');
            }
            self.rendered.push_str(part);
        }
    }

    fn finish(self) -> String {
        debug_assert!(self.pending_determiner.is_none());
        debug_assert_eq!(self.skip_payload_subtrees, 0);
        self.rendered
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedCoordinationRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if self.skip_payload_subtrees > 0 {
            self.skip_payload_subtrees -= 1;
            return Ok(());
        }
        let value = value as &dyn std::any::Any;
        match category {
            "Determiner" => {
                let determiner = value
                    .downcast_ref::<Determiner>()
                    .expect("the declaration's Determiner hole preserves its Rust type");
                self.pending_determiner = Some(determiner.clone());
            }
            "NounPhrase" => {
                let noun_phrase = value
                    .downcast_ref::<NounPhrase>()
                    .expect("the declaration's NounPhrase hole preserves its Rust type");
                self.push(&self.renderer.noun_phrase(noun_phrase)?);
            }
            "NominalPhrase" => {
                let mut nominal = value
                    .downcast_ref::<NominalPhrase>()
                    .expect("the declaration's NominalPhrase hole preserves its Rust type")
                    .clone();
                if let Some(determiner) = self.pending_determiner.take() {
                    nominal.determiner = Some(determiner);
                }
                self.push(&self.renderer.nominal_phrase(&nominal)?);
            }
            other => panic!("unexpected coordination subtree category `{other}`"),
        }
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        match codec {
            "Conjunction" | "NounPhraseConjunction" => {
                let conjunction = value
                    .downcast_ref::<Conjunction>()
                    .expect("the declaration's Conjunction scalar preserves its Rust type");
                self.push(render_nominal_conjunction(*conjunction)?);
            }
            other => panic!("unexpected coordination scalar codec `{other}`"),
        }
        Ok(())
    }

    fn derived_sequence_scalar(
        &mut self,
        _field: &'static str,
        codec: &'static str,
        _index: usize,
        len: usize,
    ) -> Result<(), Self::Error> {
        match codec {
            "Comma" if len >= 2 => self.push(","),
            "Comma" => {}
            other => panic!("unexpected derived coordination scalar codec `{other}`"),
        }
        Ok(())
    }

    fn bound_value<T: std::any::Any>(
        &mut self,
        element: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if element == "nominal_complement" {
            let complement = (value as &dyn std::any::Any)
                .downcast_ref::<NominalComplement>()
                .expect("the bound NominalComplement keeps its Rust enum type");
            self.push(&self.renderer.nominal_complement(complement)?);
            self.skip_payload_subtrees += 1;
        }
        Ok(())
    }
}

fn finish_generated_coordination(
    result: Result<(), deckmaste_construction_compiler::runtime::LinearizationError<RenderError>>,
    visitor: GeneratedCoordinationRenderer<'_, '_>,
) -> Result<String, RenderError> {
    match result {
        Ok(()) => Ok(visitor.finish()),
        Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
            Err(error)
        }
        Err(error) => {
            unreachable!("validated coordination must select one declared form: {error:?}")
        }
    }
}

impl<'identity> Renderer<'identity> {
    fn new(name: &'identity str, is_legendary: bool) -> Self {
        Self {
            name,
            short_name: short_name(name, is_legendary),
            vocabulary: Vocabulary::new(),
            nesting: Cell::new(0),
            terminal_quote: Cell::new(None),
        }
    }

    fn coordinated_noun_phrase(
        &self,
        value: &crate::syntax::CoordinatedNounPhrase,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedCoordinationRenderer::new(self);
        let result = crate::constructions::coordination::linearize_noun_phrase_coordination_with(
            value,
            &mut visitor,
        );
        finish_generated_coordination(result, visitor)
    }

    fn coordinated_nominal_phrase(
        &self,
        value: &crate::syntax::CoordinatedNominalPhrase,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedCoordinationRenderer::new(self);
        let result = crate::constructions::coordination::linearize_shared_determiner_nominal_with(
            value,
            &mut visitor,
        );
        finish_generated_coordination(result, visitor)
    }

    /// Renders an ability nested inside a quoted or embedded ability, tracking
    /// the enclosing depth so period derivation can tell top-level keyword
    /// lines from the same words appearing as nested prose.
    fn nested_ability(
        &self,
        ability: &Ability,
        capitalize: bool,
        suppress_final_period: bool,
    ) -> Result<String, RenderError> {
        self.nesting.set(self.nesting.get() + 1);
        let rendered = self.ability(ability, capitalize, suppress_final_period);
        self.nesting.set(self.nesting.get() - 1);
        rendered
    }

    fn oracle_text(&self, oracle_text: &OracleText) -> Result<String, RenderError> {
        oracle_text
            .abilities
            .iter()
            .map(|ability| self.ability(ability, true, false))
            .collect::<Result<Vec<_>, _>>()
            .map(|abilities| abilities.join("\n"))
    }

    /// Renders one ability. `suppress_final_period` withholds the derived
    /// terminal period of the ability's final sentence — set only for a quoted
    /// ability that does not close its enclosing sentence, so the period lives
    /// outside the quote (`has "…" and "…."`).
    fn ability(
        &self,
        ability: &Ability,
        capitalize: bool,
        suppress_final_period: bool,
    ) -> Result<String, RenderError> {
        let body = self.ability_kind(&ability.kind, capitalize, suppress_final_period)?;
        let rendered = if let Some(ability_word) = &ability.ability_word {
            format!("{} — {}", ability_word.spelling(), capitalize_first(body))
        } else if let Some(flavor_header) = &ability.flavor_header {
            // A flavor-word label reproduces verbatim before its em dash, the
            // same shape as an ability word; the body after ` — ` carries its
            // own capitalization exactly as it was peeled.
            format!("{} — {}", flavor_header.text(), capitalize_first(body))
        } else {
            body
        };
        Ok(if capitalize { capitalize_first(rendered) } else { rendered })
    }

    fn ability_kind(
        &self,
        kind: &AbilityKind,
        capitalize: bool,
        suppress_final_period: bool,
    ) -> Result<String, RenderError> {
        match kind {
            AbilityKind::Activated(activated) => Ok(format!(
                "{}: {}",
                self.cost(&activated.cost)?,
                // An activated ability's effect always capitalizes after the
                // cost colon; there is no surface where it does not.
                self.paragraph_with_suffix(&activated.effect, true, suppress_final_period)?
            )),
            AbilityKind::ClassLevel(level) => Ok(format!(
                "{}: Level {}",
                self.cost(&level.cost)?,
                level.level.numeral.format(level.level.value),
            )),
            AbilityKind::Triggered(triggered) => Ok(format!(
                "{}, {}",
                self.trigger_condition_list_frame(
                    &triggered.conditions,
                    triggered.intervening_condition.as_ref(),
                )?,
                self.paragraph_with_suffix(&triggered.effect, false, suppress_final_period)?
            )),
            AbilityKind::Loyalty(loyalty) => Ok(format!(
                "[{}]: {}",
                render_loyalty_cost(loyalty.cost),
                self.paragraph_with_suffix(&loyalty.effect, true, suppress_final_period)?
            )),
            AbilityKind::Chapter(chapter) => self.chapter_ability(chapter),
            AbilityKind::RollRow(row) => self.roll_row_ability(row),
            AbilityKind::LevelBand(band) => self.level_band_ability(band),
            AbilityKind::StationThreshold(threshold) => self.station_threshold_ability(threshold),
            AbilityKind::Modal(modal) => self.modal_ability(modal),
            AbilityKind::Keyword(keyword) => {
                self.keyword_ability_list(keyword, suppress_final_period)
            }
            AbilityKind::Paragraph(paragraph) => {
                self.paragraph_with_suffix(paragraph, capitalize, suppress_final_period)
            }
        }
    }

    /// Renders a saga chapter ability in oracle layout: the comma-separated
    /// Roman-numeral header, a spaced em dash, then the single effect body
    /// inline. This is the exact inverse of [`chapter_frame`]; the body never
    /// takes a `• ` bullet, which is what separates a chapter from a modal
    /// choice ability.
    ///
    /// [`chapter_frame`]: crate::grammar
    fn chapter_ability(&self, chapter: &ChapterAbility) -> Result<String, RenderError> {
        let header = chapter
            .chapters
            .iter()
            .map(|number| number.numeral.format(number.value))
            .collect::<Vec<_>>()
            .join(", ");
        let body = self.paragraph(&chapter.body, true)?;
        Ok(format!("{header} \u{2014} {body}"))
    }

    /// Renders a die-roll result row in oracle layout: the face-value key, a
    /// spaced ` | `, then the effect body inline. This is the exact inverse of
    /// [`roll_row_frame`]: an inclusive range's dash stays *unspaced* and keeps
    /// its own glyph, while the ` | ` separator is reproduced verbatim.
    ///
    /// [`roll_row_frame`]: crate::grammar
    fn roll_row_ability(&self, row: &RollRowAbility) -> Result<String, RenderError> {
        let body = self.paragraph(&row.body, true)?;
        Ok(format!("{} | {body}", render_roll_range(row.range)))
    }

    /// Renders a leveler card's level band in oracle layout: the `LEVEL`
    /// header, the bare power/toughness stat line, then one line per contained
    /// ability — the exact inverse of the band frame in [`Parser::parse`]
    /// (`crate::grammar`). Nothing recovers at the header or the stat line:
    /// both are carried structurally, so this is the whole-sentence bracket
    /// read's absence assertion made concrete.
    fn level_band_ability(&self, band: &LevelBandAbility) -> Result<String, RenderError> {
        let mut rendered = format!(
            "LEVEL {}\n{}/{}",
            render_level_range(band.range),
            render_signed_scalar(band.stats.power),
            render_signed_scalar(band.stats.toughness),
        );
        for ability in &band.abilities {
            rendered.push('\n');
            rendered.push_str(&self.ability(ability, true, false)?);
        }
        Ok(rendered)
    }

    /// Renders a station card's threshold striation in oracle layout: the
    /// `N+` key, a spaced ` | `, then the contained ability inline — the
    /// exact inverse of [`station_threshold_frame`]. `self.ability(inner,
    /// true, false)` is the same call the top-level list and the level band
    /// make, so the contained ability renders byte-for-byte as it would
    /// standing alone, which is what holds roundtrip at zero. No
    /// `render_station_*` free function is needed: there is no range enum,
    /// only a bare [`NumberLiteral`] [CR#721.2].
    ///
    /// [`station_threshold_frame`]: crate::grammar
    fn station_threshold_ability(
        &self,
        threshold: &StationThresholdAbility,
    ) -> Result<String, RenderError> {
        Ok(format!(
            "{}+ | {}",
            threshold
                .threshold
                .numeral
                .format(threshold.threshold.value),
            self.ability(&threshold.ability, true, false)?,
        ))
    }

    fn modal_ability(&self, modal: &ModalAbility) -> Result<String, RenderError> {
        let header_is_sentence_initial = !matches!(modal.frame, ModalFrame::Triggered(_));
        // A ` —` header suffix stands in for the final header sentence's period,
        // so that sentence's derived period is withheld (`Choose one —`); a
        // `None` suffix keeps the normal periods (`Choose one. If you …`).
        let suppress_final_period = matches!(modal.header_suffix, ModalHeaderSuffix::SpacedEmDash);
        let header = self.paragraph_with_suffix(
            &modal.header,
            header_is_sentence_initial,
            suppress_final_period,
        )?;
        let header = match modal.header_suffix {
            ModalHeaderSuffix::None => header,
            ModalHeaderSuffix::SpacedEmDash => format!("{header} —"),
        };
        let framed = match &modal.frame {
            ModalFrame::Unframed => header,
            ModalFrame::Activated(cost) => format!("{}: {header}", self.cost(cost)?),
            ModalFrame::Triggered(trigger) => format!(
                "{}, {header}",
                self.trigger_frame(
                    trigger.introducer,
                    &trigger.event,
                    trigger.intervening_condition.as_ref(),
                )?
            ),
            ModalFrame::Loyalty(cost) => {
                format!("[{}]: {header}", render_loyalty_cost(*cost))
            }
            ModalFrame::Chapter(chapters) => {
                let chapters = chapters
                    .iter()
                    .map(|number| number.numeral.format(number.value))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{chapters} \u{2014} {header}")
            }
            ModalFrame::Keyword(atom) => format!("{}{header}", atom.spelling()),
        };
        let modes = modal
            .modes
            .iter()
            .map(|mode| {
                let body = self.paragraph(&mode.body, true)?;
                let rendered = match &mode.heading {
                    Some(heading) => format!(
                        "{} \u{2014} {} \u{2014} {body}",
                        heading.label.text(),
                        self.cost(&heading.cost)?
                    ),
                    None => body,
                };
                Ok(format!("• {rendered}"))
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
        let mut rendered = format!("{} {event}", introducer.spelling());
        if let Some(condition) = intervening_condition {
            rendered.push_str(", ");
            rendered.push_str(&self.dependent_clause(condition)?);
        }
        Ok(rendered)
    }

    fn trigger_condition_list_frame(
        &self,
        conditions: &TriggerConditionList,
        intervening_condition: Option<&DependentClause>,
    ) -> Result<String, RenderError> {
        let TriggerCondition { introducer, event } = &conditions.first;
        let mut rendered = self.trigger_frame(*introducer, event, None)?;
        for coordination in &conditions.rest {
            rendered.push(' ');
            rendered.push_str(render_predicate_conjunction(coordination.conjunction)?);
            rendered.push(' ');
            let TriggerCondition { introducer, event } = &coordination.condition;
            rendered.push_str(&self.trigger_frame(*introducer, event, None)?);
        }
        if let Some(condition) = intervening_condition {
            rendered.push_str(", ");
            rendered.push_str(&self.dependent_clause(condition)?);
        }
        Ok(rendered)
    }

    fn choice_instruction(&self, choice: &ChoiceInstruction) -> Result<String, RenderError> {
        let mut rendered = String::new();
        if let Some(trigger) = &choice.trigger_prefix {
            rendered.push_str(&self.trigger_frame(
                trigger.introducer,
                &trigger.event,
                trigger.intervening_condition.as_ref(),
            )?);
            rendered.push_str(", ");
        }
        rendered.push_str(&self.predicate(&choice.imperative)?);
        if choice.at_random {
            rendered.push_str(" at random");
        }
        Ok(rendered)
    }

    fn keyword_ability_list(
        &self,
        list: &KeywordAbilityList,
        suppress_final_period: bool,
    ) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for item in &list.abilities {
            if let Some(separator) = item.preceding_separator {
                rendered.push_str(match separator {
                    KeywordListSeparator::Comma => ", ",
                    KeywordListSeparator::Semicolon => "; ",
                });
            }
            rendered.push_str(item.ability.spelling());
            rendered.push_str(&self.keyword_argument(&item.argument)?);
        }
        if let Some(trailing) = &list.trailing {
            rendered.push(' ');
            rendered.push_str(&self.paragraph_with_suffix(
                trailing,
                true,
                suppress_final_period,
            )?);
        }
        Ok(rendered)
    }

    /// Renders a keyword argument, including the leading separator that joins
    /// it to the keyword. Each shape reproduces its own surface; the
    /// joining and internal dashes are carried structurally, never inferred
    /// from spelling.
    /// Renders a [`KeywordCost`] alone, without the leading keyword-to-
    /// argument space a bare [`KeywordArgument::Costed`] needs — shared by
    /// `Costed` and [`KeywordArgument::RestrictedCost`], whose restriction
    /// already supplies that leading space.
    fn keyword_cost(&self, cost: &KeywordCost) -> Result<String, RenderError> {
        Ok(match cost {
            KeywordCost::Symbols(symbols) => format!(" {}", render_symbol_sequence(symbols)),
            // `Sentence`'s separator is always the spaced em dash; see the
            // variant's doc comment.
            KeywordCost::Sentence { ability } => format!(
                "{}{}",
                keyword_argument_separator(KeywordArgumentSeparator::SpacedEmDash),
                self.nested_ability(ability, true, false)?
            ),
            // `Components`'s separator is always the tight em dash, and a
            // present terminal is always a bare period; see the variant's
            // doc comment.
            KeywordCost::Components { cost, terminal } => format!(
                "{}{}{}",
                keyword_argument_separator(KeywordArgumentSeparator::EmDash),
                capitalize_first(self.cost(cost)?),
                if *terminal { "." } else { "" }
            ),
        })
    }

    /// The body of a [`KeywordArgument::Predicated`] argument, with no
    /// leading separator — shared by [`Self::keyword_argument`] (which
    /// prepends one space) and the nominal keyword-argument complement
    /// (`kwgrant` round), whose caller already supplies exactly one space via
    /// `join_words`.
    fn predicated_argument(&self, argument: &PredicatedArgument) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for (index, quality) in argument.qualities.iter().enumerate() {
            if index > 0 {
                rendered.push_str(" and ");
            }
            if let Some(preposition) = quality.preposition {
                rendered.push_str(render_preposition(preposition));
                rendered.push(' ');
            }
            rendered.push_str(&self.phrase(&quality.quality)?);
        }
        Ok(rendered)
    }

    fn keyword_argument(&self, argument: &KeywordArgument) -> Result<String, RenderError> {
        Ok(match argument {
            KeywordArgument::Absent => String::new(),
            KeywordArgument::Qualified(phrase) => format!(" {}", self.phrase(phrase)?),
            KeywordArgument::Counted(quantity) => format!(" {}", render_quantity(*quantity)),
            KeywordArgument::Costed(cost) => self.keyword_cost(cost)?,
            KeywordArgument::RestrictedCost {
                preposition,
                restriction,
                cost,
            } => {
                let mut rendered = String::from(" ");
                if let Some(preposition) = preposition {
                    rendered.push_str(render_preposition(*preposition));
                    rendered.push(' ');
                }
                rendered.push_str(&self.noun_phrase(restriction)?);
                rendered.push_str(&self.keyword_cost(cost)?);
                rendered
            }
            KeywordArgument::CountedCost { count, symbols } => {
                format!(
                    " {}—{}",
                    count.numeral.format(count.value),
                    render_symbol_sequence(symbols)
                )
            }
            KeywordArgument::Predicated(predicated) => {
                format!(" {}", self.predicated_argument(predicated)?)
            }
            KeywordArgument::Statted { symbols, stats } => format!(
                " {} — {}/{}",
                render_symbol_sequence(symbols),
                render_signed_scalar(stats.power),
                render_signed_scalar(stats.toughness)
            ),
            KeywordArgument::Named { separator, label } => {
                format!("{}{label}", keyword_argument_separator(*separator))
            }
            // `Recovered`'s separator is always Space; see the variant's doc
            // comment.
            KeywordArgument::Recovered { text } => {
                format!(
                    "{}{}",
                    keyword_argument_separator(KeywordArgumentSeparator::Space),
                    text.spelling()
                )
            }
        })
    }

    fn cost(&self, cost: &Cost) -> Result<String, RenderError> {
        let mut rendered = String::new();
        if let Some(header) = &cost.flavor_header {
            rendered.push_str(header.text());
            rendered.push_str(" \u{2014} ");
        }
        let mut saw_lexical_component = false;
        for (index, component) in cost.components.iter().enumerate() {
            if index > 0 {
                rendered.push_str(", ");
            }
            let is_symbol = matches!(component, CostComponent::Symbols(_));
            let starts_action = matches!(
                component,
                CostComponent::Clause(clause)
                    if matches!(clause.as_ref(), IndependentClause::Imperative(_))
            );
            let capitalize = starts_action || (!saw_lexical_component && !is_symbol);
            let text = self.cost_component(component)?;
            rendered.push_str(&if capitalize { capitalize_first(text) } else { text });
            saw_lexical_component |= !is_symbol;
        }
        Ok(rendered)
    }

    fn cost_component(&self, component: &CostComponent) -> Result<String, RenderError> {
        Ok(match component {
            CostComponent::Symbols(symbols) => render_symbol_sequence(symbols),
            CostComponent::Clause(clause) => self.independent_clause(clause)?,
            CostComponent::Noun(noun) => self.noun_phrase(noun)?,
            CostComponent::Alternative(left, right) => {
                format!(
                    "{} or {}",
                    self.cost_component(left)?,
                    self.cost_component(right)?
                )
            }
            CostComponent::Recovered(text) => text.spelling().to_owned(),
        })
    }

    fn paragraph(
        &self,
        paragraph: &Paragraph,
        capitalize_first_sentence: bool,
    ) -> Result<String, RenderError> {
        self.paragraph_with_suffix(paragraph, capitalize_first_sentence, false)
    }

    /// Renders a paragraph. When `suppress_final_period` is set, the last
    /// sentence's derived terminal period is withheld — used for a modal header
    /// whose ` —` suffix stands in for that period (`Choose one —`).
    fn paragraph_with_suffix(
        &self,
        paragraph: &Paragraph,
        capitalize_first_sentence: bool,
        suppress_final_period: bool,
    ) -> Result<String, RenderError> {
        let mut rendered = String::new();
        if let Some(header) = &paragraph.flavor_header {
            // Verbatim flavor header plus its em-dash separator; the trailing
            // space of the ` — ` separator is contributed by the sentence loop.
            rendered.push_str(header.text());
            rendered.push_str(" \u{2014}");
        }
        let last = paragraph.sentences.len().saturating_sub(1);
        for (index, sentence) in paragraph.sentences.iter().enumerate() {
            let force_no_period = suppress_final_period && index == last;
            let sentence = self.sentence(
                sentence,
                capitalize_first_sentence || index > 0,
                force_no_period,
            )?;
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

    fn sentence(
        &self,
        sentence: &Sentence,
        capitalize: bool,
        force_no_period: bool,
    ) -> Result<String, RenderError> {
        // Publish this sentence's terminal quote for the duration of its body,
        // so a quoted ability rendered deep inside it can recognize itself as
        // the node that absorbs this sentence's period. Restored afterwards: a
        // quote's own interior sentences publish their own terminals while
        // nested inside this one. When the period is forced off — a modal
        // header's ` —` suffix standing in for it — there is nothing to absorb,
        // so nothing is published.
        let published = if force_no_period {
            None
        } else {
            sentence_terminal_quote(sentence).map(std::ptr::from_ref)
        };
        let previous = self.terminal_quote.replace(published);
        let body = self.sentence_body(sentence, capitalize);
        self.terminal_quote.set(previous);
        let (body, capitalize) = body?;
        let mut rendered = if capitalize { capitalize_first(body) } else { body };
        if !force_no_period && self.sentence_takes_period(sentence) {
            rendered.push('.');
        }
        Ok(rendered)
    }

    /// Renders a sentence's body, reporting whether the result still wants
    /// initial capitalization. Split out of [`Self::sentence`] so the
    /// terminal-quote channel is restored on the error path as well.
    fn sentence_body(
        &self,
        sentence: &Sentence,
        capitalize: bool,
    ) -> Result<(String, bool), RenderError> {
        Ok(match &sentence.body {
            SentenceBody::Independent(clause) => (self.independent_clause(clause)?, capitalize),
            SentenceBody::Choice(choice) => (self.choice_instruction(choice)?, capitalize),
            SentenceBody::PowerToughness(value) => (
                format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness),
                ),
                false,
            ),
            SentenceBody::Triggered(triggered) => {
                let frame = self.trigger_frame(
                    triggered.trigger.introducer,
                    &triggered.trigger.event,
                    triggered.trigger.intervening_condition.as_ref(),
                )?;
                let effect = self.independent_clause(&triggered.effect)?;
                (format!("{frame}, {effect}"), capitalize)
            }
            SentenceBody::Recovered(recovery) => (recovery.spelling().to_owned(), false),
        })
    }

    /// Whether a rendered sentence takes a trailing period.
    ///
    /// This re-derives the period that [`Sentence`] no longer stores. Oracle
    /// text terminates every sentence with a period **except** for these
    /// structurally identifiable classes, which the corpus round-trip confirms
    /// are exhaustive:
    ///
    /// - a sentence whose final rendered constituent is a *closed* quoted
    ///   ability (`this creature gains "…"`, `create a token with "…"`) — the
    ///   period then sits inside the closing quote;
    /// - a sentence whose final rendered constituent is a self-reference to a
    ///   card whose name already ends in terminal punctuation (`Exile Blood for
    ///   the Blood God!`, the self-reference being the whole card name) — the
    ///   name supplies the terminator;
    /// - the Aura **enchant ability** line at top level (`Enchant creature`) —
    ///   a subjectless imperative headed by, or a clause subjected by, the
    ///   `enchant` keyword, printed without a period like the keyword ability
    ///   it is (nested inside a quote the same words are ordinary prose and
    ///   keep their period, hence the `nesting` guard); and
    /// - a **recovered** span, which carries its own terminal punctuation
    ///   verbatim.
    ///
    /// A modal `Choose …` header instruction is an ordinary complete sentence
    /// and *does* take a period (`Choose one. If you control …`); the ` —`
    /// header-suffix exception (`Choose one —`) is handled by the modal
    /// renderer withholding the final period, keeping this a structural
    /// function of the modal ability's suffix.
    ///
    /// Every clause branch is a structural function of the sentence's AST tail,
    /// never an inspection of the rendered string's trailing character.
    fn sentence_takes_period(&self, sentence: &Sentence) -> bool {
        let clause = match &sentence.body {
            SentenceBody::Choice(_) | SentenceBody::PowerToughness(_) => return true,
            SentenceBody::Recovered(_) => return false,
            SentenceBody::Independent(clause) => clause,
            SentenceBody::Triggered(triggered) => &triggered.effect,
        };
        if independent_clause_terminal_quote(clause).is_some()
            || self.clause_ends_with_terminated_self_reference(clause)
        {
            return false;
        }
        if self.nesting.get() == 0 && is_aura_enchant_line(clause) {
            return false;
        }
        true
    }

    /// Whether the clause's final rendered constituent is a self-reference to a
    /// card whose (resolved) name ends in sentence-terminal punctuation, so the
    /// name itself supplies the terminator and no period is derived.
    fn clause_ends_with_terminated_self_reference(&self, clause: &IndependentClause) -> bool {
        let Some(form) = independent_clause_final_self_reference(clause) else {
            return false;
        };
        let name = match form {
            ThisCardForm::AbbreviatedName => self.short_name.unwrap_or_default(),
            ThisCardForm::FullName => self.name,
        };
        name.ends_with(['.', '!', '?'])
    }

    fn clause(&self, clause: &Clause) -> Result<String, RenderError> {
        match clause {
            Clause::Independent(clause) => self.independent_clause(clause),
            Clause::Dependent(clause) => self.dependent_clause(clause),
        }
    }

    fn independent_clause(&self, clause: &IndependentClause) -> Result<String, RenderError> {
        match clause {
            IndependentClause::Transitive(subject, predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.transitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            IndependentClause::Intransitive(subject, predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.intransitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            IndependentClause::Copular(subject, predicate) => {
                self.copular_clause(subject, predicate)
            }
            IndependentClause::Passive(subject, predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.passive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            IndependentClause::Predicated(subject, predicate) => {
                self.predicate_expression(subject.as_ref(), predicate)
            }
            IndependentClause::Imperative(predicate) => self.predicate(predicate),
            IndependentClause::Deontic(subject, modal, predicate) => {
                let mut parts = vec![
                    self.subject(subject)?,
                    self.render_auxiliary(modal.auxiliary)?,
                ];
                if let Some(predicate) = predicate {
                    parts.push(self.predicate(predicate)?);
                }
                Ok(join_words(parts))
            }
            IndependentClause::Existential(existential) => self.existential_clause(existential),
            IndependentClause::Proform(subject, predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.render_auxiliary(predicate.auxiliary)?,
            ])),
            IndependentClause::Complex(complex) => {
                let matrix = self.independent_clause(&complex.matrix)?;
                self.clause_with_attachments(&matrix, &complex.attachments)
            }
            IndependentClause::Coordinated(coordinated) => {
                let mut rendered = self.independent_clause(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    if coordination.comma.is_present() {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    if let Some(conjunction) = coordination.conjunction {
                        rendered.push_str(render_predicate_conjunction(conjunction)?);
                        rendered.push(' ');
                    }
                    match &coordination.member {
                        CoordinatedClauseMember::Independent(clause) => {
                            rendered.push_str(&self.independent_clause(clause)?);
                        }
                    }
                }
                Ok(rendered)
            }
        }
    }

    fn predicate_expression(
        &self,
        subject: Option<&Subject>,
        expression: &PredicateExpression,
    ) -> Result<String, RenderError> {
        match expression {
            PredicateExpression::Simple(predicate) => match subject {
                Some(subject) => self.predicate_with_subject(subject, predicate),
                None => self.predicate(predicate),
            },
            PredicateExpression::Coordinated(coordination) => {
                let mut conjuncts = coordination.conjuncts().iter();
                let first = conjuncts
                    .next()
                    .expect("a validated coordination has a first conjunct");
                let mut rendered = self.predicate_expression(subject, first)?;
                for (junction, expression) in coordination.junctions().iter().zip(conjuncts) {
                    if junction.comma.is_present() {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    if let Some(conjunction) = junction.conjunction {
                        rendered.push_str(render_predicate_conjunction(conjunction)?);
                        rendered.push(' ');
                    }
                    rendered.push_str(&self.predicate_expression(None, expression)?);
                }
                Ok(rendered)
            }
        }
    }

    fn predicate_with_subject(
        &self,
        subject: &Subject,
        predicate: &Predicate,
    ) -> Result<String, RenderError> {
        match predicate {
            Predicate::Transitive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.transitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Intransitive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.intransitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Copular(predicate) => self.copular_clause(subject, predicate),
            Predicate::Passive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                Ok(join_words(vec![
                    subject,
                    self.passive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Proform(predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.render_auxiliary(predicate.auxiliary)?,
            ])),
            Predicate::Deontic(predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.deontic_predicate(predicate)?,
            ])),
            Predicate::Attached(predicate) => {
                let matrix = self.predicate_with_subject(subject, &predicate.predicate)?;
                self.clause_with_attachments(&matrix, &predicate.attachments)
            }
        }
    }

    fn subject(&self, subject: &Subject) -> Result<String, RenderError> {
        self.noun_phrase(&subject.0)
    }

    fn clause_with_attachments(
        &self,
        matrix: &str,
        attachments: &[ClauseAttachment],
    ) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for attachment in attachments
            .iter()
            .filter(|attachment| attachment.position == AttachmentPosition::BeforeMatrix)
        {
            rendered.push_str(&self.clause_attachment(&attachment.payload)?);
            if attachment.comma.is_present() {
                rendered.push(',');
            }
            rendered.push(' ');
        }
        rendered.push_str(matrix);
        for attachment in attachments
            .iter()
            .filter(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
        {
            if attachment.comma.is_present() {
                rendered.push(',');
            }
            rendered.push(' ');
            rendered.push_str(&self.clause_attachment(&attachment.payload)?);
        }
        Ok(rendered)
    }

    fn clause_attachment(&self, attachment: &ClauseAttachmentKind) -> Result<String, RenderError> {
        match attachment {
            ClauseAttachmentKind::Dependent(clause) => self.dependent_clause(clause),
            ClauseAttachmentKind::Adjunct(adjunct) => self.predicate_adjunct(adjunct),
            ClauseAttachmentKind::Exception(rider) => self.exception_rider(rider),
            ClauseAttachmentKind::Restriction(run) => self.restriction_run(run),
            ClauseAttachmentKind::Appositive(clause) => {
                // The spaced ` — ` is fixed for this attachment: the enclosing
                // clause loop contributes the leading space, and this arm emits
                // the em dash and the coordinated option run after it. The
                // option run opens a fresh capitalized clause after the dash,
                // exactly as an ability word's em-dash body does.
                Ok(format!(
                    "\u{2014} {}",
                    capitalize_first(self.independent_clause(clause)?)
                ))
            }
        }
    }

    fn exception_rider(&self, rider: &ExceptionRider) -> Result<String, RenderError> {
        let mut rendered = String::from("except ");
        rendered.push_str(&self.independent_clause(&rider.first)?);
        for conjunct in &rider.rest {
            // The serial comma is a function of length and connective, never
            // a stored flag: an asyndetic interior member always takes a
            // comma, and a member with a connective takes one only in a
            // three-or-more-member (Oxford) list. See `ExceptionConjunct`.
            let comma = conjunct.conjunction.is_none() || rider.rest.len() >= 2;
            if comma {
                rendered.push(',');
            }
            rendered.push(' ');
            if let Some(conjunction) = conjunct.conjunction {
                rendered.push_str(render_predicate_conjunction(conjunction)?);
                rendered.push(' ');
            }
            rendered.push_str(&self.independent_clause(&conjunct.clause)?);
        }
        Ok(rendered)
    }

    fn restriction_run(&self, run: &crate::syntax::RestrictionRun) -> Result<String, RenderError> {
        let mut rendered = String::from("only ");
        rendered.push_str(&self.restriction_member(&run.first)?);
        for member in &run.rest {
            // The serial comma is a function of length and connective, never
            // a stored flag: an asyndetic interior member always takes a
            // comma, and a member with a connective takes one only in a
            // three-or-more-member (Oxford) list. See `RestrictionCoordination`.
            let comma = member.conjunction.is_none() || run.rest.len() >= 2;
            if comma {
                rendered.push(',');
            }
            if let Some(conjunction) = member.conjunction {
                rendered.push(' ');
                rendered.push_str(render_predicate_conjunction(conjunction)?);
            }
            rendered.push_str(" only ");
            rendered.push_str(&self.restriction_member(&member.adjuncts)?);
        }
        Ok(rendered)
    }

    /// Renders a restriction member's adjunct sequence, space-joined — the
    /// same spacing the flat `elements` list already uses between adjacent
    /// adjuncts (e.g. the `once` adverb followed by the `each turn` temporal).
    fn restriction_member(&self, adjuncts: &[PredicateAdjunct]) -> Result<String, RenderError> {
        let mut rendered = String::new();
        for (index, adjunct) in adjuncts.iter().enumerate() {
            if index > 0 {
                rendered.push(' ');
            }
            rendered.push_str(&self.predicate_adjunct(adjunct)?);
        }
        Ok(rendered)
    }

    fn subject_with_predicate_head(
        &self,
        subject: &Subject,
        head: &PredicateHead,
    ) -> Result<(String, usize), RenderError> {
        let mut subject = self.subject(subject)?;
        let auxiliary_start = Self::contract_with_first_auxiliary(&mut subject, head)?;
        Ok((subject, auxiliary_start))
    }

    fn contract_with_first_auxiliary(
        rendered_subject: &mut String,
        head: &PredicateHead,
    ) -> Result<usize, RenderError> {
        if !head.first_auxiliary_contracted_with_subject.is_contracted() {
            return Ok(0);
        }
        let auxiliary = head
            .auxiliaries
            .first()
            .copied()
            .ok_or(RenderError::MissingLexicalForm("contracted auxiliary"))?;
        rendered_subject.push_str(contraction_suffix(auxiliary)?);
        Ok(1)
    }

    fn predicate(&self, predicate: &Predicate) -> Result<String, RenderError> {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate),
            Predicate::Intransitive(predicate) => self.intransitive_predicate(predicate),
            Predicate::Copular(predicate) => {
                let mut parts = vec![self.render_auxiliary(predicate.copula.auxiliary)?];
                if predicate.distributive_each {
                    parts.push("each".to_owned());
                }
                parts.push(self.copular_complement(&predicate.complement)?);
                for adjunct in &predicate.adjuncts {
                    parts.push(self.predicate_adjunct(adjunct)?);
                }
                Ok(join_words(parts))
            }
            Predicate::Passive(predicate) => self.passive_predicate(predicate),
            Predicate::Proform(predicate) => self.render_auxiliary(predicate.auxiliary),
            Predicate::Deontic(predicate) => self.deontic_predicate(predicate),
            Predicate::Attached(predicate) => {
                let matrix = self.predicate(&predicate.predicate)?;
                self.clause_with_attachments(&matrix, &predicate.attachments)
            }
        }
    }

    fn deontic_predicate(
        &self,
        predicate: &crate::syntax::DeonticPredicate,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.render_auxiliary(predicate.modal.auxiliary)?];
        if let Some(inner) = &predicate.inner {
            parts.push(self.predicate(inner)?);
        }
        Ok(join_words(parts))
    }

    fn transitive_predicate(
        &self,
        predicate: &crate::syntax::TransitivePredicate,
    ) -> Result<String, RenderError> {
        self.transitive_predicate_from(predicate, 0)
    }

    fn transitive_predicate_from(
        &self,
        predicate: &crate::syntax::TransitivePredicate,
        auxiliary_start: usize,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.predicate_head_from(&predicate.head, auxiliary_start)?];
        self.extend_predicate_elements(&mut parts, &predicate.pre_object_elements)?;
        let mut element_start = 0;
        while let Some(element) = predicate.elements.get(element_start) {
            match element {
                PredicateElement::Complement(PredicateComplement::IndirectObject(
                    indirect_object,
                )) => {
                    parts.push(self.noun_phrase(indirect_object)?);
                    element_start += 1;
                }
                _ => break,
            }
        }
        parts.push(self.predicate_object(&predicate.object)?);
        self.extend_predicate_elements(&mut parts, &predicate.elements[element_start..])?;
        Ok(join_words(parts))
    }

    fn intransitive_predicate(
        &self,
        predicate: &crate::syntax::IntransitivePredicate,
    ) -> Result<String, RenderError> {
        self.intransitive_predicate_from(predicate, 0)
    }

    fn intransitive_predicate_from(
        &self,
        predicate: &crate::syntax::IntransitivePredicate,
        auxiliary_start: usize,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.predicate_head_from(&predicate.head, auxiliary_start)?];
        self.extend_predicate_elements(&mut parts, &predicate.elements)?;
        Ok(join_words(parts))
    }

    fn passive_predicate(
        &self,
        predicate: &crate::syntax::PassivePredicate,
    ) -> Result<String, RenderError> {
        self.passive_predicate_from(predicate, 0)
    }

    fn passive_predicate_from(
        &self,
        predicate: &crate::syntax::PassivePredicate,
        auxiliary_start: usize,
    ) -> Result<String, RenderError> {
        let mut parts = vec![self.predicate_head_from(&predicate.head, auxiliary_start)?];
        if let Some(retained_object) = &predicate.retained_object {
            parts.push(self.predicate_object(retained_object)?);
        }
        self.extend_predicate_elements(&mut parts, &predicate.elements)?;
        Ok(join_words(parts))
    }

    fn predicate_head_from(
        &self,
        head: &PredicateHead,
        auxiliary_start: usize,
    ) -> Result<String, RenderError> {
        if head.distributive_each && head.first_auxiliary_contracted_with_subject.is_contracted() {
            return Err(RenderError::InvalidDistributiveEachContraction);
        }
        let mut parts =
            Vec::with_capacity(head.auxiliaries.len() + head.preverb_modifiers.len() + 2);
        if head.distributive_each {
            parts.push("each".to_owned());
        }
        for &auxiliary in &head.auxiliaries[auxiliary_start..] {
            parts.push(self.render_auxiliary(auxiliary)?);
        }
        parts.extend(head.preverb_modifiers.iter().map(|modifier| {
            match modifier {
                PreverbModifier::Not => "not",
                PreverbModifier::Also => "also",
                PreverbModifier::Next => "next",
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
                PredicateElement::Particle(particle) => match particle {
                    VerbParticle::In => "in".to_owned(),
                    VerbParticle::Out => "out".to_owned(),
                },
                // The exact inverse of the `up heads`/`up tails` scanner: it
                // maps the typed value back to its literal surface without
                // ever inspecting a stored string [CR#705.1,705.2].
                PredicateElement::CoinResult(side) => match side {
                    crate::syntax::CoinSide::Heads => "up heads".to_owned(),
                    crate::syntax::CoinSide::Tails => "up tails".to_owned(),
                },
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
            PredicateObject::SymbolSequence(symbols) => Ok(render_symbol_sequence(symbols)),
            PredicateObject::PowerToughness(value) => Ok(format!(
                "{}/{}",
                render_signed_scalar(value.power),
                render_signed_scalar(value.toughness),
            )),
            PredicateObject::EmbeddedAbility(ability) => self.nested_ability(ability, true, false),
            PredicateObject::QuotedAbility(quoted) => self.quoted_ability(quoted),
            PredicateObject::Coordinated(coordinated) => {
                let mut rendered = self.predicate_object(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    // The serial comma is a function of length and
                    // connective, never a stored flag: an asyndetic interior
                    // member always takes a comma, and a member with a
                    // connective takes one only in a three-or-more-member
                    // (Oxford) list. See `PredicateObjectCoordination`.
                    let comma = coordination.conjunction.is_none() || coordinated.rest.len() >= 2;
                    if comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    if let Some(conjunction) = coordination.conjunction {
                        rendered.push_str(render_predicate_conjunction(conjunction)?);
                        rendered.push(' ');
                    }
                    rendered.push_str(&self.predicate_object(&coordination.object)?);
                }
                Ok(rendered)
            }
        }
    }

    fn predicate_complement(
        &self,
        complement: &PredicateComplement,
    ) -> Result<String, RenderError> {
        match complement {
            PredicateComplement::IndirectObject(phrase) => self.noun_phrase(phrase),
            PredicateComplement::Adjective(phrase) => self.adjective_phrase(phrase),
            PredicateComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(coordinated)
            }
            PredicateComplement::Prepositional(phrase) => self.prepositional_phrase(phrase),
            PredicateComplement::Infinitive(clause) => self.infinitive_clause(clause),
        }
    }

    fn predicate_adjunct(&self, adjunct: &PredicateAdjunct) -> Result<String, RenderError> {
        match adjunct {
            PredicateAdjunct::Adverb(adverb) => Ok(adverb.spelling().to_owned()),
            PredicateAdjunct::Frequency(frequency) => Ok(render_frequency(*frequency)),
            PredicateAdjunct::Temporal(phrase) | PredicateAdjunct::Manner(phrase) => {
                self.noun_phrase(phrase)
            }
            PredicateAdjunct::Prepositional(phrase) => self.prepositional_phrase(phrase),
            PredicateAdjunct::Exception(phrase) => {
                Ok(format!("except {}", self.prepositional_phrase(phrase)?))
            }
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
        if predicate.copula.contracted_with_subject.is_contracted() {
            parts.push(format!(
                "{subject}{}",
                contraction_suffix(predicate.copula.auxiliary)?
            ));
        } else {
            parts.push(subject);
            parts.push(self.render_auxiliary(predicate.copula.auxiliary)?);
        }
        if predicate.negated {
            parts.push("not".to_owned());
        }
        if predicate.distributive_each {
            parts.push("each".to_owned());
        }
        parts.extend(
            predicate
                .precomplement_adverbs
                .iter()
                .map(|adverb| adverb.spelling().to_owned()),
        );
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
            CopularComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(coordinated)
            }
            CopularComplement::Prepositional(phrase) => self.prepositional_phrase(phrase),
            CopularComplement::PowerToughness(value) => Ok(format!(
                "{}/{}",
                render_signed_scalar(value.power),
                render_signed_scalar(value.toughness),
            )),
            CopularComplement::CatalogAtom(atom) => Ok(render_catalog_atom(atom)),
        }
    }

    fn existential_clause(&self, clause: &ExistentialClause) -> Result<String, RenderError> {
        let opening = clause.form.spelling();
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
                    SubordinateBody::Infinitive(clause) => self.infinitive_clause(clause)?,
                    SubordinateBody::Gerund(clause) => self.gerund_clause(clause)?,
                    SubordinateBody::Elliptical(EllipticalClause::Adjective(phrase)) => {
                        self.adjective_phrase(phrase)?
                    }
                };
                Ok(format!("{} {body}", render_subordinator(*subordinator)))
            }
            DependentClause::Relative(relative) => self.relative_clause(relative),
            DependentClause::Infinitive(infinitive) => self.infinitive_clause(infinitive),
            DependentClause::Gerund(gerund) => self.gerund_clause(gerund),
        }
    }

    fn gerund_clause(&self, clause: &GerundClause) -> Result<String, RenderError> {
        let matrix = self.predicate(&clause.predicate)?;
        let mut rendered = String::new();
        for attachment in clause
            .attachments
            .iter()
            .filter(|attachment| attachment.position == AttachmentPosition::BeforeMatrix)
        {
            rendered.push_str(&self.dependent_clause(&attachment.payload)?);
            if attachment.comma.is_present() {
                rendered.push(',');
            }
            rendered.push(' ');
        }
        rendered.push_str(&matrix);
        for attachment in clause
            .attachments
            .iter()
            .filter(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
        {
            if attachment.comma.is_present() {
                rendered.push(',');
            }
            rendered.push(' ');
            rendered.push_str(&self.dependent_clause(&attachment.payload)?);
        }
        Ok(rendered)
    }

    fn infinitive_clause(&self, clause: &InfinitiveClause) -> Result<String, RenderError> {
        let predicate = self.predicate(&clause.predicate)?;
        let infinitive = match clause.marker {
            InfinitiveMarker::Bare => predicate,
            InfinitiveMarker::To => format!("to {predicate}"),
        };
        Ok(if clause.negated { format!("not {infinitive}") } else { infinitive })
    }

    fn relative_clause(&self, clause: &RelativeClause) -> Result<String, RenderError> {
        let mut marker = match clause.marker {
            RelativeMarker::That => "that",
            RelativeMarker::Who => "who",
            RelativeMarker::Zero => "",
        }
        .to_owned();
        let body = match &clause.body {
            RelativeBody::SubjectGap(Predicate::Transitive(predicate))
                if predicate
                    .head
                    .first_auxiliary_contracted_with_subject
                    .is_contracted() =>
            {
                let auxiliary_start =
                    Self::contract_with_first_auxiliary(&mut marker, &predicate.head)?;
                self.transitive_predicate_from(predicate, auxiliary_start)?
            }
            RelativeBody::SubjectGap(Predicate::Intransitive(predicate))
                if predicate
                    .head
                    .first_auxiliary_contracted_with_subject
                    .is_contracted() =>
            {
                let auxiliary_start =
                    Self::contract_with_first_auxiliary(&mut marker, &predicate.head)?;
                self.intransitive_predicate_from(predicate, auxiliary_start)?
            }
            RelativeBody::SubjectGap(Predicate::Passive(predicate))
                if predicate
                    .head
                    .first_auxiliary_contracted_with_subject
                    .is_contracted() =>
            {
                let auxiliary_start =
                    Self::contract_with_first_auxiliary(&mut marker, &predicate.head)?;
                self.passive_predicate_from(predicate, auxiliary_start)?
            }
            RelativeBody::SubjectGap(Predicate::Copular(predicate))
                if predicate.copula.contracted_with_subject.is_contracted() =>
            {
                marker.push_str(contraction_suffix(predicate.copula.auxiliary)?);
                let mut parts = Vec::new();
                if predicate.distributive_each {
                    parts.push("each".to_owned());
                }
                parts.push(self.copular_complement(&predicate.complement)?);
                for adjunct in &predicate.adjuncts {
                    parts.push(self.predicate_adjunct(adjunct)?);
                }
                join_words(parts)
            }
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate)?,
            RelativeBody::ObjectGap { subject, predicate } => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, &predicate.head)?;
                let mut parts = vec![
                    subject,
                    self.predicate_head_from(&predicate.head, auxiliary_start)?,
                ];
                self.extend_predicate_elements(&mut parts, &predicate.elements)?;
                join_words(parts)
            }
        };
        Ok(join_words(vec![marker, body]))
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
            NounPhrase::Possessive(possessor) => {
                self.determiner(&Determiner::Possessive(possessor.clone()))
            }
            NounPhrase::Demonstrative(demonstrative) => Ok(demonstrative.spelling().to_owned()),
            NounPhrase::Quantity(quantity) => Ok(render_quantity(*quantity)),
            NounPhrase::ThisCard(form) => self.this_card(*form),
            NounPhrase::Partitive(partitive) => Ok(format!(
                "{} of {}",
                match partitive.head {
                    crate::syntax::PartitiveHead::Quantity(quantity) => render_quantity(quantity),
                    crate::syntax::PartitiveHead::Each => "each".to_owned(),
                },
                self.noun_phrase(&partitive.whole)?
            )),
            NounPhrase::CoordinatedNominal(coordinated) => {
                self.coordinated_nominal_phrase(coordinated)
            }
            NounPhrase::Coordinated(coordinated) => self.coordinated_noun_phrase(coordinated),
            NounPhrase::SetException(exception) => {
                let mut rendered = self.noun_phrase(&exception.included)?;
                if exception.comma.is_present() {
                    rendered.push(',');
                }
                rendered.push_str(match exception.marker {
                    SetExceptionMarker::Bare => " except ",
                    SetExceptionMarker::For => " except for ",
                });
                rendered.push_str(&self.noun_phrase(&exception.excluded)?);
                Ok(rendered)
            }
            NounPhrase::Arithmetic(value) => self.arithmetic_value(value),
        }
    }

    fn arithmetic_value(
        &self,
        value: &crate::syntax::ArithmeticValue,
    ) -> Result<String, RenderError> {
        match value {
            crate::syntax::ArithmeticValue::Minus { left, right } => Ok(format!(
                "{} minus {}",
                self.noun_phrase(left)?,
                self.noun_phrase(right)?
            )),
            crate::syntax::ArithmeticValue::Half { value, rounding } => {
                let mut rendered = format!("half {}", self.noun_phrase(value)?);
                match rounding {
                    Some(crate::syntax::Rounding::Up) => rendered.push_str(", rounded up"),
                    Some(crate::syntax::Rounding::Down) => rendered.push_str(", rounded down"),
                    None => {}
                }
                Ok(rendered)
            }
        }
    }

    fn nominal_phrase(&self, phrase: &NominalPhrase) -> Result<String, RenderError> {
        let mut parts = Vec::with_capacity(phrase.modifiers.len() + phrase.complements.len() + 2);
        let mut trailing_modifier_complements = Vec::new();
        if let Some(determiner) = &phrase.determiner {
            // The indefinite article's word is not stored: it is the initial
            // sound of the material that follows it (`a card`, `an Elf`).
            // See `Determiner::Indefinite`.
            if *determiner == Determiner::Indefinite {
                let sound = self.nominal_initial_sound(phrase)?;
                parts.push(indefinite_article_for(sound).to_owned());
            } else {
                parts.push(self.determiner(determiner)?);
            }
        }
        for modifier in &phrase.modifiers {
            let (rendered, trailing) = self.render_nominal_modifier(modifier)?;
            trailing_modifier_complements.extend(trailing);
            parts.push(rendered);
        }
        parts.push(self.render_noun(&phrase.head)?);
        for complement in &phrase.complements {
            parts.push(self.nominal_complement(complement)?);
        }
        parts.extend(trailing_modifier_complements);
        Ok(join_words(parts))
    }

    fn nominal_complement(&self, complement: &NominalComplement) -> Result<String, RenderError> {
        Ok(match complement {
            NominalComplement::Adjective(adjective) => self.adjective_phrase(adjective)?,
            NominalComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(coordinated)?
            }
            NominalComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition)?
            }
            NominalComplement::Infinitive(infinitive) => self.infinitive_clause(infinitive)?,
            NominalComplement::Relative(relative) => self.relative_clause(relative)?,
            NominalComplement::ReducedRecipientPassive(predicate) => {
                self.transitive_predicate(predicate)?
            }
            NominalComplement::Quantity(quantity) => render_quantity(*quantity),
            NominalComplement::PowerToughness(value) => format!(
                "{}/{}",
                render_signed_scalar(value.power),
                render_signed_scalar(value.toughness),
            ),
            NominalComplement::Devotion(colors) => render_devotion_colors(*colors),
            NominalComplement::EventClause(clause) => self.independent_clause(clause)?,
            NominalComplement::KeywordArgument(argument) => match argument {
                KeywordArgument::Costed(KeywordCost::Symbols(symbols)) => {
                    render_symbol_sequence(symbols)
                }
                KeywordArgument::Predicated(predicated) => self.predicated_argument(predicated)?,
                _ => return Err(RenderError::InvalidKeywordArgumentNominal),
            },
        })
    }

    /// Renders one modifier slot to its surface, returning any trailing
    /// complements to hoist past the head (an adjective modifier's postnominal
    /// comparison). A [`NominalModifier::Coordinated`] slot recurses over its
    /// conjuncts, replaying the exact comma/conjunction surface between them.
    fn render_nominal_modifier(
        &self,
        modifier: &NominalModifier,
    ) -> Result<(String, Vec<String>), RenderError> {
        match modifier {
            NominalModifier::Adjective { polarity, phrase } => {
                let (head, trailing) = self.nominal_modifier_adjective(phrase)?;
                Ok((
                    apply_polarity(*polarity, head, adjective_is_rules_bundle(&phrase.head)),
                    trailing,
                ))
            }
            NominalModifier::Noun { polarity, noun } => Ok((
                apply_polarity(
                    *polarity,
                    self.render_noun(noun)?,
                    noun_is_rules_bundle(noun),
                ),
                Vec::new(),
            )),
            NominalModifier::CombatStepName { participants } => Ok((
                format!("declare {}", self.render_noun(participants)?),
                Vec::new(),
            )),
            NominalModifier::Quantity(quantity) => Ok((render_quantity(*quantity), Vec::new())),
            NominalModifier::PowerToughness(value) => Ok((
                format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness),
                ),
                Vec::new(),
            )),
            NominalModifier::Coordinated(coordinated) => {
                let (first, mut trailing) = self.render_nominal_modifier(&coordinated.first)?;
                let mut rendered = first;
                for coordination in &coordinated.rest {
                    // The serial comma is a function of length and connective,
                    // never a stored flag: an asyndetic interior member always
                    // takes a comma, and a member with a connective takes one
                    // only in a three-or-more-member (Oxford) list. See
                    // `ModifierCoordination`.
                    let comma = coordination.conjunction.is_none() || coordinated.rest.len() >= 2;
                    if comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    if let Some(conjunction) = coordination.conjunction {
                        rendered.push_str(render_predicate_conjunction(conjunction)?);
                        rendered.push(' ');
                    }
                    let (member, member_trailing) =
                        self.render_nominal_modifier(&coordination.modifier)?;
                    rendered.push_str(&member);
                    trailing.extend(member_trailing);
                }
                Ok((rendered, trailing))
            }
        }
    }

    fn nominal_initial_sound(&self, phrase: &NominalPhrase) -> Result<InitialSound, RenderError> {
        if let Some(first) = phrase.modifiers.first() {
            return self.modifier_initial_sound(first);
        }
        self.noun_initial_sound(&phrase.head)
    }

    fn modifier_initial_sound(
        &self,
        modifier: &NominalModifier,
    ) -> Result<InitialSound, RenderError> {
        match modifier {
            // A negated modifier's surface starts with `non`, so its initial
            // sound is a consonant regardless of the base it negates.
            NominalModifier::Adjective { polarity, .. }
            | NominalModifier::Noun { polarity, .. }
                if polarity.is_negative() =>
            {
                Ok(InitialSound::Consonant)
            }
            NominalModifier::Adjective { phrase, .. } => self.adjective_initial_sound(&phrase.head),
            NominalModifier::Noun { noun, .. } => self.noun_initial_sound(noun),
            // Always literally `declare …` — a fixed consonant onset.
            NominalModifier::CombatStepName { .. } => Ok(InitialSound::Consonant),
            NominalModifier::Quantity(quantity) => {
                Ok(surface_initial_sound(&render_quantity(*quantity)))
            }
            NominalModifier::PowerToughness(value) => Ok(value.initial_sound()),
            // A coordinated slot's leading surface is its first conjunct's.
            NominalModifier::Coordinated(coordinated) => {
                self.modifier_initial_sound(&coordinated.first)
            }
        }
    }

    fn noun_initial_sound(&self, noun: &NounInstance) -> Result<InitialSound, RenderError> {
        let noun = match noun {
            NounInstance::Singular(noun)
            | NounInstance::Plural(noun)
            | NounInstance::Mass(noun) => noun,
        };
        match noun {
            Noun::Word(vocab) => Ok(self.vocabulary.initial_sound(*vocab)),
            Noun::Catalog(atom) => Ok(surface_initial_sound(atom.canonical())),
            Noun::Die(_) => Ok(InitialSound::Consonant),
            Noun::Gerund(verb) => self
                .vocabulary
                .render_verb_instance(&crate::word::VerbInstance {
                    verb: verb.clone(),
                    slot: crate::word::VerbSlot::PresentParticiple,
                })
                .map(|surface| surface_initial_sound(&surface))
                .ok_or(RenderError::MissingLexicalForm("gerund")),
            Noun::Agentive(verb) => self
                .vocabulary
                .render_noun(&NounInstance::Singular(Noun::Agentive(verb.clone())))
                .map(|surface| surface_initial_sound(&surface))
                .ok_or(RenderError::MissingLexicalForm("agent noun")),
            Noun::Opaque(opaque) => Ok(surface_initial_sound(opaque.spelling())),
        }
    }

    fn adjective_initial_sound(&self, adjective: &Adjective) -> Result<InitialSound, RenderError> {
        match adjective {
            Adjective::Word(vocab) => Ok(self.vocabulary.initial_sound(*vocab)),
            _ => self
                .vocabulary
                .render_adjective(adjective)
                .map(|surface| surface_initial_sound(&surface))
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
            Determiner::Possessive(Possessor::NounPhrase(possessor)) => Ok(format!(
                "{}{}",
                self.noun_phrase(possessor)?,
                possessive_marker(possessor)
            )),
            _ => determiner.render(),
        }
    }

    fn adjective_phrase(&self, phrase: &AdjectivePhrase) -> Result<String, RenderError> {
        let mut parts = adjective_degree(phrase.degree.as_ref())
            .into_iter()
            .collect::<Vec<_>>();
        parts.push(self.adjective_head(&phrase.head)?);
        for complement in &phrase.complements {
            parts.push(self.adjective_complement(complement)?);
        }
        Ok(join_words(parts))
    }

    /// Renders a coordinated run of predicative adjective phrases, replaying
    /// the exact comma/connective surface recorded between the conjuncts.
    fn coordinated_adjective_phrase(
        &self,
        coordinated: &CoordinatedAdjectivePhrase,
    ) -> Result<String, RenderError> {
        let mut rendered = self.adjective_phrase(&coordinated.first)?;
        for coordination in &coordinated.rest {
            // The serial comma is a function of length and connective, never
            // a stored flag: an asyndetic interior member always takes a
            // comma, and a member with a connective takes one only in a
            // three-or-more-member (Oxford) list. See
            // `AdjectivePhraseCoordination`.
            let comma = coordination.conjunction.is_none() || coordinated.rest.len() >= 2;
            if comma {
                rendered.push(',');
            }
            rendered.push(' ');
            if let Some(conjunction) = coordination.conjunction {
                rendered.push_str(render_predicate_conjunction(conjunction)?);
                rendered.push(' ');
            }
            rendered.push_str(&self.adjective_phrase(&coordination.phrase)?);
        }
        Ok(rendered)
    }

    fn nominal_modifier_adjective(
        &self,
        phrase: &AdjectivePhrase,
    ) -> Result<(String, Vec<String>), RenderError> {
        let mut immediate = adjective_degree(phrase.degree.as_ref())
            .into_iter()
            .collect::<Vec<_>>();
        immediate.push(self.adjective_head(&phrase.head)?);
        let mut trailing = Vec::new();
        for complement in &phrase.complements {
            let rendered = self.adjective_complement(complement)?;
            if matches!(complement, AdjectiveComplement::PostnominalComparison(_)) {
                trailing.push(rendered);
            } else {
                immediate.push(rendered);
            }
        }
        Ok((join_words(immediate), trailing))
    }

    fn adjective_head(&self, adjective: &Adjective) -> Result<String, RenderError> {
        self.vocabulary
            .render_adjective(adjective)
            .ok_or(RenderError::MissingLexicalForm("adjective"))
    }

    fn adjective_complement(
        &self,
        complement: &AdjectiveComplement,
    ) -> Result<String, RenderError> {
        Ok(match complement {
            AdjectiveComplement::Comparison(comparison)
            | AdjectiveComplement::PostnominalComparison(comparison) => {
                let marker = match comparison.marker {
                    ComparisonMarker::Than => "than",
                    ComparisonMarker::ThanOrEqualTo => "than or equal to",
                };
                format!("{marker} {}", self.phrase(&comparison.standard)?)
            }
            AdjectiveComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition)?
            }
            AdjectiveComplement::Infinitive(infinitive) => self.infinitive_clause(infinitive)?,
        })
    }

    fn prepositional_phrase(&self, phrase: &PrepositionalPhrase) -> Result<String, RenderError> {
        match phrase {
            PrepositionalPhrase::Simple(simple) => self.simple_prepositional_phrase(simple),
            PrepositionalPhrase::Coordinated(coordinated) => {
                // The serial comma is a function of length, never a stored
                // flag: `A and B` takes none, `A, B, and C` takes one before
                // every member. See `PrepositionalPhraseCoordination`.
                let serial_comma = coordinated.rest.len() > 1;
                let mut rendered = self.simple_prepositional_phrase(&coordinated.first)?;
                for coordination in &coordinated.rest {
                    if serial_comma {
                        rendered.push(',');
                    }
                    rendered.push(' ');
                    if let Some(conjunction) = coordination.conjunction {
                        rendered.push_str(render_nominal_conjunction(conjunction)?);
                        rendered.push(' ');
                    }
                    rendered.push_str(&self.simple_prepositional_phrase(&coordination.phrase)?);
                }
                Ok(rendered)
            }
        }
    }

    fn simple_prepositional_phrase(
        &self,
        phrase: &SimplePrepositionalPhrase,
    ) -> Result<String, RenderError> {
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
            Phrase::Cost(cost) => self.cost(cost),
            Phrase::ThisCard(form) => self.this_card(*form),
            Phrase::OracleSymbol(symbol) => Ok(symbol.as_str().to_owned()),
            Phrase::SymbolSequence(symbols) => Ok(render_symbol_sequence(symbols)),
            Phrase::NumberLiteral(number) => Ok(number.numeral.format(number.value)),
            Phrase::SignedScalar(scalar) => Ok(render_signed_scalar(*scalar)),
            Phrase::PowerToughness(power_toughness) => Ok(format!(
                "{}/{}",
                render_signed_scalar(power_toughness.power),
                render_signed_scalar(power_toughness.toughness)
            )),
            Phrase::EmbeddedAbility(ability) => self.nested_ability(ability, true, false),
            Phrase::QuotedAbility(quoted) => self.quoted_ability(quoted),
            Phrase::Recovered(recovery) => Ok(recovery.spelling().to_owned()),
        }
    }

    fn quoted_ability(&self, quoted: &QuotedAbility) -> Result<String, RenderError> {
        // The interior's derived terminal period is withheld unless this quote
        // closes its enclosing sentence — the period sits inside the quote only
        // when the surface put it there (`gains "…."` vs `has "…" and "…."`).
        //
        // Derived, not stored: the enclosing sentence published the address of
        // the quote its AST tail walk reached, and this is that quote exactly
        // when the two addresses agree. Identity is the right test because the
        // walk borrows from the very tree being rendered — two distinct quotes
        // with equal content are still only one sentence tail.
        let terminal_period = self
            .terminal_quote
            .get()
            .is_some_and(|terminal| std::ptr::eq(terminal, quoted));
        let mut interior =
            self.nested_ability(&quoted.ability, quoted.initial_uppercase, !terminal_period)?;
        // Keyword lines do not derive sentence punctuation themselves. A
        // quoted keyword ability can nevertheless take a terminal from the
        // quote's own position, after the parser has kept it out of the
        // argument. `Qualified` (`kwbandsother` round, `bands with other
        // legendary creatures.` [CR#702.22b,702.22c]) joins `Costed(Symbols)`
        // here for the same reason: neither shape's own render (see
        // `keyword_argument` below) ever prints a period itself, unlike
        // `Costed(Components { terminal, .. })`/`Costed(Sentence { .. })`,
        // which track or derive their own trailing punctuation and must not
        // have a second one appended here.
        if terminal_period
            && matches!(
                &quoted.ability.kind,
                AbilityKind::Keyword(list)
                    if matches!(
                        list.abilities.last().map(|ability| &ability.argument),
                        Some(
                            KeywordArgument::Costed(KeywordCost::Symbols(_))
                                | KeywordArgument::Qualified(_)
                        )
                    )
            )
        {
            interior.push('.');
        }
        // The closing quote always prints: `closed` was removed (surface-fact
        // sweep, 2026-07-30) after every construction site was found to set
        // it to `true` — 0 mismatches across all 31685 supported faces.
        Ok(format!("\"{interior}\""))
    }

    fn render_noun(&self, noun: &NounInstance) -> Result<String, RenderError> {
        self.vocabulary
            .render_noun(noun)
            .ok_or(RenderError::MissingLexicalForm("noun"))
    }

    fn this_card(&self, form: ThisCardForm) -> Result<String, RenderError> {
        let rendered = match form {
            ThisCardForm::AbbreviatedName => self.short_name.expect(
                "AbbreviatedName is emitted only for a face with a shortened name; \
                 render with the parse identity",
            ),
            ThisCardForm::FullName => self.name,
        };
        if rendered.is_empty() {
            Err(RenderError::CardIdentityRequired)
        } else {
            Ok(rendered.to_owned())
        }
    }
}

/// The genitive marker for a noun-phrase possessor: a plural head takes the
/// bare apostrophe (`owners'`), everything else takes `'s` (`owner's`). Read
/// from the head's number rather than the rendered spelling so a singular
/// noun that happens to end in `s` still renders `'s`.
fn possessive_marker(possessor: &NounPhrase) -> &'static str {
    match possessor {
        NounPhrase::Nominal(nominal) => match nominal.head {
            NounInstance::Plural(_) => "'",
            NounInstance::Singular(_) | NounInstance::Mass(_) => "'s",
        },
        _ => "'s",
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

/// Renders a die-roll row's face-value key. Inclusive spans join their bounds
/// with an *unspaced* en dash (`–`, U+2013) — the single canonical range glyph
/// the input boundary normalizes every roll-row separator to — while the `+`
/// and `or less` thresholds reproduce their surface too.
fn render_roll_range(range: RollRange) -> String {
    match range {
        RollRange::Single(value) => value.numeral.format(value.value),
        RollRange::Inclusive { low, high } => format!(
            "{}\u{2013}{}",
            low.numeral.format(low.value),
            high.numeral.format(high.value),
        ),
        RollRange::OrMore(value) => format!("{}+", value.numeral.format(value.value)),
        RollRange::OrLess(value) => format!("{} or less", value.numeral.format(value.value)),
    }
}

/// Renders a level band's counter range [CR#711.2a,711.2b]. The separator is
/// an **ASCII hyphen**, not the en dash [`render_roll_range`] emits for a
/// die-roll row: a level symbol is not a roll-row key, so
/// `normalize_roll_row_dashes` never touches it, and emitting anything but
/// the ASCII hyphen here would make this renderer a non-inverse of the frame.
fn render_level_range(range: LevelRange) -> String {
    match range {
        LevelRange::Band { low, high } => format!(
            "{}-{}",
            low.numeral.format(low.value),
            high.numeral.format(high.value),
        ),
        LevelRange::AtLeast(value) => format!("{}+", value.numeral.format(value.value)),
    }
}

/// Whether a clause is an Aura's enchant-ability line — the top-level keyword
/// ability oracle prints without a period. The parser lowers it two ways
/// depending on the object's shape, both anchored on the `enchant` keyword:
///
/// - `Enchant creature`, `Enchant land you control` → a subjectless imperative
///   headed by the `enchant` verb; and
/// - `Enchant tapped creature`, `Enchant modified creature` → a clause whose
///   *subject* is the `Enchant` keyword atom (the following adjective is
///   misread as the verb).
///
/// Requiring the imperative or the enchant-keyword subject keeps an ordinary
/// `this creature enchants a creature` clause (a real subject, real verb)
/// taking its period.
fn is_aura_enchant_line(clause: &IndependentClause) -> bool {
    match clause {
        IndependentClause::Imperative(predicate) => predicate_verb_is_enchant(predicate),
        IndependentClause::Transitive(subject, _)
        | IndependentClause::Intransitive(subject, _)
        | IndependentClause::Copular(subject, _)
        | IndependentClause::Passive(subject, _)
        | IndependentClause::Proform(subject, _)
        | IndependentClause::Deontic(subject, _, _)
        | IndependentClause::Predicated(Some(subject), _) => subject_is_enchant_keyword(subject),
        _ => false,
    }
}

fn predicate_verb_is_enchant(predicate: &Predicate) -> bool {
    if let Predicate::Attached(predicate) = predicate {
        return predicate_verb_is_enchant(&predicate.predicate);
    }
    let head = match predicate {
        Predicate::Transitive(predicate) => &predicate.head,
        Predicate::Intransitive(predicate) => &predicate.head,
        Predicate::Passive(predicate) => &predicate.head,
        Predicate::Copular(_)
        | Predicate::Proform(_)
        | Predicate::Deontic(_)
        | Predicate::Attached(_) => return false,
    };
    head.verb.verb == Verb::Word(Vocab::Enchant)
}

/// Whether a subject is the bare `Enchant` keyword-ability atom (the misparse
/// of `Enchant <adjective> <type>` that treats the keyword as the subject
/// noun).
fn subject_is_enchant_keyword(subject: &Subject) -> bool {
    let NounPhrase::Nominal(nominal) = &subject.0 else {
        return false;
    };
    if !nominal.modifiers.is_empty() || nominal.determiner.is_some() {
        return false;
    }
    let (NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun)) =
        &nominal.head;
    matches!(
        noun,
        Noun::Catalog(atom)
            if atom.kind == CatalogKind::KeywordAbility && atom.canonical() == "Enchant"
    )
}

/// The [`ThisCardForm`] of a clause's final rendered constituent when that
/// constituent is a self-reference, else `None`. Used to suppress the derived
/// period when the card's name already ends in terminal punctuation.
fn independent_clause_final_self_reference(clause: &IndependentClause) -> Option<ThisCardForm> {
    let (IndependentClause::Transitive(_, predicate)
    | IndependentClause::Imperative(Predicate::Transitive(predicate))
    | IndependentClause::Predicated(
        _,
        PredicateExpression::Simple(Predicate::Transitive(predicate)),
    )) = clause
    else {
        return None;
    };
    if !predicate.elements.is_empty() {
        return None;
    }
    match &predicate.object {
        PredicateObject::NounPhrase(NounPhrase::ThisCard(form)) => Some(*form),
        _ => None,
    }
}

/// The closed quoted ability that terminates a sentence, if any — the one node
/// whose interior keeps the sentence's period inside its closing quote.
///
/// A [`SentenceBody::Choice`] header and a bare power/toughness body have no
/// object position to hold a quote, and a recovered span reproduces its own
/// punctuation verbatim, so none of the three can move a period into a quote.
fn sentence_terminal_quote(sentence: &Sentence) -> Option<&QuotedAbility> {
    let clause = match &sentence.body {
        SentenceBody::Choice(_) | SentenceBody::PowerToughness(_) | SentenceBody::Recovered(_) => {
            return None;
        }
        SentenceBody::Independent(clause) => clause,
        SentenceBody::Triggered(triggered) => &triggered.effect,
    };
    independent_clause_terminal_quote(clause)
}

/// The closed quoted ability a clause's final *rendered* constituent is, if
/// any — the single node whose interior absorbs the enclosing sentence's
/// terminal period. Returning the node rather than a bare `bool` is what lets
/// [`Renderer::quoted_ability`] derive a period placement the AST no longer
/// stores: the sentence renderer publishes this node's address, and the quote
/// that recognizes itself in it keeps its interior period.
fn independent_clause_terminal_quote(clause: &IndependentClause) -> Option<&QuotedAbility> {
    match clause {
        IndependentClause::Transitive(_, predicate) => transitive_terminal_quote(predicate),
        IndependentClause::Intransitive(_, predicate) => {
            last_element_terminal_quote(&predicate.elements)
        }
        IndependentClause::Passive(_, predicate) => passive_terminal_quote(predicate),
        IndependentClause::Copular(_, predicate) => copular_terminal_quote(predicate),
        IndependentClause::Predicated(_, expression) => {
            predicate_expression_terminal_quote(expression)
        }
        IndependentClause::Imperative(predicate)
        | IndependentClause::Deontic(_, _, Some(predicate)) => predicate_terminal_quote(predicate),
        IndependentClause::Coordinated(clause) => coordinated_terminal_quote(clause),
        IndependentClause::Complex(clause) => complex_terminal_quote(clause),
        IndependentClause::Existential(_)
        | IndependentClause::Proform(..)
        | IndependentClause::Deontic(_, _, None) => None,
    }
}

fn predicate_expression_terminal_quote(expression: &PredicateExpression) -> Option<&QuotedAbility> {
    match expression {
        PredicateExpression::Simple(predicate) => predicate_terminal_quote(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .last()
            .and_then(predicate_expression_terminal_quote),
    }
}

fn predicate_terminal_quote(predicate: &Predicate) -> Option<&QuotedAbility> {
    match predicate {
        Predicate::Transitive(predicate) => transitive_terminal_quote(predicate),
        Predicate::Intransitive(predicate) => last_element_terminal_quote(&predicate.elements),
        Predicate::Passive(predicate) => passive_terminal_quote(predicate),
        Predicate::Copular(predicate) => copular_terminal_quote(predicate),
        Predicate::Proform(_) => None,
        Predicate::Deontic(predicate) => predicate
            .inner
            .as_deref()
            .and_then(predicate_terminal_quote),
        Predicate::Attached(predicate) => attached_predicate_terminal_quote(predicate),
    }
}

fn attached_predicate_terminal_quote(
    predicate: &crate::syntax::AttachedPredicate,
) -> Option<&QuotedAbility> {
    match predicate
        .attachments
        .iter()
        .rev()
        .find(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
    {
        Some(attachment) => match &attachment.payload {
            ClauseAttachmentKind::Adjunct(adjunct) => adjunct_terminal_quote(adjunct),
            ClauseAttachmentKind::Dependent(_)
            | ClauseAttachmentKind::Exception(_)
            | ClauseAttachmentKind::Restriction(_) => None,
            ClauseAttachmentKind::Appositive(clause) => independent_clause_terminal_quote(clause),
        },
        None => predicate_terminal_quote(&predicate.predicate),
    }
}

/// A transitive predicate ends with its final adjunct/complement element, or —
/// when it has none — with its object (`this creature gains "…"` leaves the
/// quoted ability as the object with no trailing element).
fn transitive_terminal_quote(predicate: &TransitivePredicate) -> Option<&QuotedAbility> {
    if predicate.elements.is_empty() {
        predicate_object_terminal_quote(&predicate.object)
    } else {
        last_element_terminal_quote(&predicate.elements)
    }
}

/// A passive predicate ends with its final element, or — when it has none —
/// with its retained object, mirroring [`transitive_terminal_quote`]'s object
/// fallback; ordinary passives (no retained object, no elements) never end in a
/// closed quote here.
fn passive_terminal_quote(predicate: &crate::syntax::PassivePredicate) -> Option<&QuotedAbility> {
    if predicate.elements.is_empty() {
        predicate
            .retained_object
            .as_ref()
            .and_then(predicate_object_terminal_quote)
    } else {
        last_element_terminal_quote(&predicate.elements)
    }
}

fn copular_terminal_quote(predicate: &CopularPredicate) -> Option<&QuotedAbility> {
    if let Some(adjunct) = predicate.adjuncts.last() {
        adjunct_terminal_quote(adjunct)
    } else if let CopularComplement::Prepositional(prepositional) = &predicate.complement {
        phrase_terminal_quote(&prepositional.tail().object)
    } else {
        None
    }
}

fn coordinated_terminal_quote(clause: &CoordinatedIndependentClause) -> Option<&QuotedAbility> {
    match clause.rest.last() {
        Some(coordination) => match &coordination.member {
            CoordinatedClauseMember::Independent(clause) => {
                independent_clause_terminal_quote(clause)
            }
        },
        None => independent_clause_terminal_quote(&clause.first),
    }
}

/// A complex clause renders its after-matrix attachments after the matrix, so
/// the tail is the last such attachment when present, and the matrix otherwise.
fn complex_terminal_quote(clause: &ComplexClause) -> Option<&QuotedAbility> {
    match clause
        .attachments
        .iter()
        .rev()
        .find(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
    {
        Some(attachment) => match &attachment.payload {
            ClauseAttachmentKind::Adjunct(adjunct) => adjunct_terminal_quote(adjunct),
            ClauseAttachmentKind::Dependent(_) => None,
            ClauseAttachmentKind::Exception(rider) => match rider.rest.last() {
                Some(conjunct) => independent_clause_terminal_quote(&conjunct.clause),
                None => independent_clause_terminal_quote(&rider.first),
            },
            ClauseAttachmentKind::Restriction(run) => match run.rest.last() {
                Some(member) => member.adjuncts.last().and_then(adjunct_terminal_quote),
                None => run.first.last().and_then(adjunct_terminal_quote),
            },
            ClauseAttachmentKind::Appositive(clause) => independent_clause_terminal_quote(clause),
        },
        None => independent_clause_terminal_quote(&clause.matrix),
    }
}

fn last_element_terminal_quote(elements: &[PredicateElement]) -> Option<&QuotedAbility> {
    elements.last().and_then(predicate_element_terminal_quote)
}

fn predicate_element_terminal_quote(element: &PredicateElement) -> Option<&QuotedAbility> {
    match element {
        PredicateElement::Complement(complement) => complement_terminal_quote(complement),
        PredicateElement::Adjunct(adjunct) => adjunct_terminal_quote(adjunct),
        PredicateElement::Particle(_) | PredicateElement::CoinResult(_) => None,
    }
}

fn complement_terminal_quote(complement: &PredicateComplement) -> Option<&QuotedAbility> {
    match complement {
        PredicateComplement::Prepositional(prepositional) => {
            phrase_terminal_quote(&prepositional.tail().object)
        }
        PredicateComplement::IndirectObject(_)
        | PredicateComplement::Adjective(_)
        | PredicateComplement::CoordinatedAdjective(_)
        | PredicateComplement::Infinitive(_) => None,
    }
}

fn adjunct_terminal_quote(adjunct: &PredicateAdjunct) -> Option<&QuotedAbility> {
    match adjunct {
        PredicateAdjunct::Prepositional(prepositional)
        | PredicateAdjunct::Exception(prepositional) => {
            phrase_terminal_quote(&prepositional.tail().object)
        }
        PredicateAdjunct::Adverb(_)
        | PredicateAdjunct::Frequency(_)
        | PredicateAdjunct::Temporal(_)
        | PredicateAdjunct::Manner(_)
        | PredicateAdjunct::Dependent(_) => None,
    }
}

/// The closed quoted ability an object position terminates with, if any. A
/// coordination's tail member supplies it, so `has "…" and "…."` yields the
/// second quote — the only one the enclosing sentence's period moves inside.
fn predicate_object_terminal_quote(object: &PredicateObject) -> Option<&QuotedAbility> {
    match object {
        PredicateObject::QuotedAbility(quoted) => Some(quoted),
        PredicateObject::Coordinated(coordinated) => coordinated_object_terminal_quote(coordinated),
        _ => None,
    }
}

fn coordinated_object_terminal_quote(
    coordinated: &CoordinatedPredicateObject,
) -> Option<&QuotedAbility> {
    match coordinated.rest.last() {
        Some(coordination) => predicate_object_terminal_quote(&coordination.object),
        None => predicate_object_terminal_quote(&coordinated.first),
    }
}

fn phrase_terminal_quote(phrase: &Phrase) -> Option<&QuotedAbility> {
    match phrase {
        Phrase::QuotedAbility(quoted) => Some(quoted),
        _ => None,
    }
}

/// Renders the `to <color>` argument of a `devotion` value nominal [CR#700.5]
/// from the carried color identities — a single color or an `and`-joined pair.
fn render_devotion_colors(colors: crate::syntax::DevotionColors) -> String {
    match colors {
        crate::syntax::DevotionColors::Color(color) => format!("to {}", color.spelling()),
        crate::syntax::DevotionColors::Pair(first, second) => {
            format!("to {} and {}", first.spelling(), second.spelling())
        }
    }
}

/// The indefinite article's surface word for a given initial sound. See
/// `Determiner::Indefinite`: the AST never stores which word was written, so
/// every render site derives it here from the following material's initial
/// sound, reusing `IndefiniteArticle::spelling()` rather than inlining the
/// literal words.
fn indefinite_article_for(sound: InitialSound) -> &'static str {
    match sound {
        InitialSound::Consonant => IndefiniteArticle::A.spelling(),
        InitialSound::Vowel => IndefiniteArticle::An.spelling(),
    }
}

/// Inverse of the parse-side `quantity_value`: `Variable` is spelled `X`.
/// (Per the design contract *Productions ship their inverse*,
/// docs/decisions/english-productions-ship-their-inverse.md.)
fn render_quantity_value(value: crate::syntax::QuantityValue) -> String {
    match value {
        crate::syntax::QuantityValue::Literal(number) => number.numeral.format(number.value),
        crate::syntax::QuantityValue::Variable => "X".to_owned(),
    }
}

fn render_quantity(quantity: Quantity) -> String {
    match quantity {
        Quantity::Exact(number) => number.numeral.format(number.value),
        Quantity::AtLeast(value) => {
            format!("at least {}", render_quantity_value(value))
        }
        Quantity::OrComparison(value, word) => {
            format!("{} or {}", render_quantity_value(value), word.spelling())
        }
        Quantity::Or(first, second) => format!(
            "{} or {}",
            first.numeral.format(first.value),
            second.numeral.format(second.value)
        ),
        Quantity::UpTo(value) => format!("up to {}", render_quantity_value(value)),
        Quantity::MoreThan(value) => {
            format!("more than {}", render_quantity_value(value))
        }
        Quantity::FewerThan(value) => {
            format!("fewer than {}", render_quantity_value(value))
        }
        Quantity::X => "X".to_owned(),
        Quantity::Both => "both".to_owned(),
        Quantity::ThatMany => "that many".to_owned(),
        Quantity::ThatMuch => "that much".to_owned(),
    }
}

fn render_frequency(frequency: FrequencyPhrase) -> String {
    let bound = match frequency.bound {
        FrequencyBound::MoreThan => "more than",
        FrequencyBound::NoMoreThan => "no more than",
    };
    let count = match frequency.count {
        FrequencyCount::Once => "once".to_owned(),
        FrequencyCount::Twice => "twice".to_owned(),
        FrequencyCount::Times(number) => {
            format!("{} times", number.numeral.format(number.value))
        }
    };
    format!("{bound} {count}")
}

fn render_catalog_atom(atom: &crate::catalog::CatalogAtom) -> String {
    match atom.kind {
        CatalogKind::KeywordAbility | CatalogKind::KeywordAction | CatalogKind::AbilityWord => {
            atom.spelling().to_owned()
        }
        _ => atom.render_adjective(),
    }
}

fn contraction_suffix(auxiliary: AuxiliaryInstance) -> Result<&'static str, RenderError> {
    use Auxiliary as A;
    use AuxiliaryInflection as I;
    use Number as N;
    use Person as P;

    match (auxiliary.auxiliary, auxiliary.inflection) {
        (
            A::Be,
            I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Ok("'re"),
        (
            A::Be | A::Have,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Ok("'s"),
        (
            A::Have,
            I::Base
            | I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Ok("'ve"),
        _ => Err(RenderError::MissingLexicalForm("subject contraction")),
    }
}

/// Renders a run of oracle symbols by concatenating their spellings with no
/// separator — the exact inverse of how a braced run is tokenized. Shared by
/// the symbol cost component, `Phrase::SymbolSequence`, and the keyword symbol
/// costs.
fn render_symbol_sequence(symbols: &[OracleSymbol]) -> String {
    symbols.iter().map(OracleSymbol::as_str).collect()
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
    subordinator.spelling()
}

fn render_predicate_conjunction(conjunction: Conjunction) -> Result<&'static str, RenderError> {
    match conjunction {
        Conjunction::And | Conjunction::Or | Conjunction::Then | Conjunction::AndOr => {
            Ok(conjunction.spelling())
        }
        Conjunction::Plus => Err(RenderError::InvalidPredicateConjunction(conjunction)),
    }
}

fn render_nominal_conjunction(conjunction: Conjunction) -> Result<&'static str, RenderError> {
    match conjunction {
        Conjunction::And | Conjunction::Or | Conjunction::Plus | Conjunction::AndOr => {
            Ok(conjunction.spelling())
        }
        Conjunction::Then => Err(RenderError::InvalidNominalConjunction(conjunction)),
    }
}

fn keyword_argument_separator(separator: KeywordArgumentSeparator) -> &'static str {
    match separator {
        KeywordArgumentSeparator::Space => " ",
        KeywordArgumentSeparator::EmDash => "—",
        KeywordArgumentSeparator::SpacedEmDash => " — ",
    }
}

fn render_preposition(preposition: Preposition) -> &'static str {
    preposition.spelling()
}

/// Whether an adjective base is a rules collective shorthand (see
/// [`crate::catalog::Bundle`]) — only [`Adjective::Catalog`] atoms can be, and
/// only those tagged as bundles. Bundle words hyphenate under `non-`.
fn adjective_is_rules_bundle(adjective: &Adjective) -> bool {
    match adjective {
        Adjective::Catalog(atom) => atom.is_rules_bundle(),
        Adjective::Word(_)
        | Adjective::Color(_)
        | Adjective::CardOrientation(_)
        | Adjective::Participle(..)
        | Adjective::Ordinal(_) => false,
    }
}

/// Whether a noun base is a rules collective shorthand (see
/// [`crate::catalog::Bundle`]) — only [`Noun::Catalog`] atoms can be. Bundle
/// words hyphenate under `non-`.
fn noun_is_rules_bundle(noun: &NounInstance) -> bool {
    let (NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun)) =
        noun;
    match noun {
        Noun::Catalog(atom) => atom.is_rules_bundle(),
        Noun::Word(_) | Noun::Die(_) | Noun::Gerund(_) | Noun::Agentive(_) | Noun::Opaque(_) => {
            false
        }
    }
}

/// Prefixes a rendered modifier base with its `non-` negation, deriving the
/// hyphenation glyph rather than replaying a stored flag: on the supported
/// corpus a capitalized base (`Human`, `Phyrexian`) is always hyphenated
/// (`non-Human`), and so is a rules collective shorthand — witnessed by
/// `outlaw` (`non-outlaw`, Shoot the Sheriff) — recognized by category, never
/// by comparing spellings. Every other lowercase base (`land`, `black`) stays
/// solid (`nonland`).
fn apply_polarity(polarity: Polarity, base: String, hyphenated_by_category: bool) -> String {
    match polarity {
        Polarity::Positive => base,
        Polarity::Negative => {
            if hyphenated_by_category || base.starts_with(char::is_uppercase) {
                format!("non-{base}")
            } else {
                format!("non{base}")
            }
        }
    }
}

fn capitalize_first(text: String) -> String {
    let mut characters = text.chars();
    let Some(first) = characters.next() else {
        return text;
    };
    first.to_uppercase().chain(characters).collect()
}

fn adjective_degree(degree: Option<&NumberLiteral>) -> Option<String> {
    degree.map(|number| number.numeral.format(number.value))
}

#[cfg(test)]
mod render_quantity_value_tests {
    use super::render_quantity;
    use crate::syntax::ComparativeWord;
    use crate::syntax::Quantity;
    use crate::syntax::QuantityValue;

    #[test]
    fn variable_bounds_render_as_x() {
        assert_eq!(
            render_quantity(Quantity::UpTo(QuantityValue::Variable)),
            "up to X"
        );
        assert_eq!(
            render_quantity(Quantity::OrComparison(
                QuantityValue::Variable,
                ComparativeWord::Less
            )),
            "X or less"
        );
        // Inverse completeness for the unwitnessed arms — the inverse must be
        // total even where the corpus is silent, per *Productions ship their
        // inverse*.
        assert_eq!(
            render_quantity(Quantity::AtLeast(QuantityValue::Variable)),
            "at least X"
        );
        assert_eq!(
            render_quantity(Quantity::MoreThan(QuantityValue::Variable)),
            "more than X"
        );
        assert_eq!(
            render_quantity(Quantity::FewerThan(QuantityValue::Variable)),
            "fewer than X"
        );
        assert_eq!(
            render_quantity(Quantity::OrComparison(
                QuantityValue::Variable,
                ComparativeWord::Greater
            )),
            "X or greater"
        );
    }
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
    use super::Renderer;
    use super::render_nominal_conjunction;
    use crate::Numeral;
    use crate::RenderError;
    use crate::catalog::CatalogKind;
    use crate::catalog::CatalogSlot;
    use crate::catalog::CatalogValue;
    use crate::catalog::Catalogs;
    use crate::features::Conjunction;
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
        IndirectObject(NounPhrase),
        PredicateComplement(Phrase),
        Scalar(Phrase),
        Adverbial(Phrase),
        Prepositional(PrepositionalPhrase),
        Infinitive(InfinitiveMarker, Box<VerbPhrase>),
    }

    #[test]
    fn keyword_abilities_render_from_canonical_catalog_identity() {
        let catalogs = fixture_catalogs();
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Keyword(KeywordAbilityList {
                    abilities: vec![
                        KeywordAbility {
                            preceding_separator: None,
                            ability: keyword_atom(&catalogs, "flying"),
                            argument: KeywordArgument::Absent,
                        },
                        KeywordAbility {
                            preceding_separator: Some(KeywordListSeparator::Comma),
                            ability: keyword_atom(&catalogs, "deathtouch"),
                            argument: KeywordArgument::Absent,
                        },
                    ],
                    trailing: None,
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
    fn activated_effect_always_capitalizes_after_the_cost_colon() {
        // `effect_initial_uppercase` is gone: an activated ability's effect
        // always capitalizes after the cost colon, regardless of the source's
        // own casing at that position (Necratog's live Oracle text is a typo
        // that prints this lowercase; the input boundary's
        // `normalize_sentence_case` corrects it before this ever parses).
        let source = "Exile a card: this creature gets +2/+2 until end of turn.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Exile a card: This creature gets +2/+2 until end of turn."
        );
    }

    #[test]
    fn an_ordinary_sentence_gets_a_derived_period() {
        // The terminal period is not stored; the renderer derives it because the
        // sentence's final constituent is not a period-absorbing one.
        let ast = crate::parse_with_catalogs("Draw a card.", &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), "Draw a card.");
    }

    #[test]
    fn a_quote_final_sentence_takes_no_outer_period() {
        // The period lives inside the closing quote, so no outer period is
        // derived; a following ordinary sentence still gets its own.
        let catalogs = fixture_catalogs();
        for source in [
            "It has \"Whenever this creature attacks, draw a card.\"",
            "It has \"Whenever this creature attacks, draw a card.\" Draw a card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn a_top_level_aura_enchant_line_omits_its_period() {
        // `Enchant creature` is a keyword-ability line printed without a period.
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Enchant"]);
        for source in ["Enchant creature", "Enchant creature you control"] {
            let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
        // But an ordinary clause that merely uses the verb keeps its period.
        let ast =
            crate::parse_with_catalogs("This creature enchants a creature.", &catalogs).into_ast();
        assert_eq!(
            source_free(&ast, "Test Card", false),
            "This creature enchants a creature."
        );
    }

    #[test]
    fn a_modal_choose_header_period_follows_the_suffix() {
        let catalogs = fixture_catalogs();
        // A ` —` header suffix stands in for the header's period.
        let em_dash = "Choose one —\n• Draw a card.\n• Draw a card.";
        let ast = crate::parse_with_catalogs(em_dash, &catalogs).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), em_dash);
        // No suffix: the `Choose …` header sentence keeps its period.
        let period = "Choose one. Each mode must be chosen once.\n• Draw a card.\n• Draw a card.";
        let ast = crate::parse_with_catalogs(period, &catalogs).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), period);
    }

    #[test]
    fn a_recovered_sentence_reproduces_its_terminal_verbatim() {
        // A recovered span keeps whatever terminal punctuation it had: a period
        // when present, none when absent — no derivation involved.
        for source in ["You frobnitz a card.", "You frobnitz a card"] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn later_sentences_always_capitalize_regardless_of_source_case() {
        // `Sentence.initial_uppercase` is gone: position alone decides
        // capitalization (`capitalize_first_sentence || index > 0`), so a
        // non-initial sentence always capitalizes even when the source
        // itself printed it lowercase (Sphinx Summoner's stale snapshot is
        // exactly this shape; the input boundary's `normalize_sentence_case`
        // corrects it before this ever parses).
        let source = "Draw a card. then shuffle.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Draw a card. Then shuffle."
        );
    }

    #[test]
    fn mid_sentence_proper_nouns_do_not_lose_their_case() {
        let source = "Exile artifacts named Eye of Vecna and Hand of Vecna: Draw a card.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn keyword_atom_first_word_of_a_name_keeps_its_source_casing() {
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Storm"]);
        let source = "Sacrifice a creature named Storm Crow: Draw a card.";
        let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn keyword_atom_in_predicate_noun_position_stays_lowercase() {
        let catalogs = fixture_catalogs().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let source = "This creature gains flying.";
        let ast = crate::parse_with_catalogs(source, &catalogs).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn elided_modal_relative_clause_renders_the_bare_modal_without_synthesized_do() {
        let source = "Exile each creature that can't.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!("expected a paragraph ability");
        };
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &paragraph.sentences[0].body
        else {
            panic!("expected an imperative transitive clause");
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
            panic!("expected a nominal object");
        };
        let [NominalComplement::Relative(relative)] = object.complements.as_slice() else {
            panic!("expected one relative complement: {object:#?}");
        };
        assert!(
            matches!(
                relative.body,
                RelativeBody::SubjectGap(Predicate::Deontic(crate::syntax::DeonticPredicate {
                    inner: None,
                    ..
                }))
            ),
            "{:#?}",
            relative.body
        );
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn bare_proform_without_a_modal_still_synthesizes_do() {
        let source = "If you do, draw a card.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn affect_is_a_transitive_verb_not_an_opaque_noun() {
        // Motivating shape (Undergrowth / Fallaji Wayfarer): a negated
        // does-support clause. As an opaque noun this rendered "doesn't do
        // affect …"; as a required-object transitive verb it must invert.
        let negated = "This creature doesn't affect combat damage.";
        let ast = crate::parse_with_catalogs(negated, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!("expected a paragraph ability");
        };
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &paragraph.sentences[0].body
        else {
            panic!(
                "expected a transitive clause: {:#?}",
                paragraph.sentences[0].body
            );
        };
        assert_eq!(
            predicate.head.verb.verb,
            Verb::Word(Vocab::Affect),
            "affect must parse as the transitive verb, not an opaque noun"
        );
        assert_eq!(source_free(&ast, "Test Card", false), negated);

        // Mirror direction: plain present-tense inflection with no does-support.
        let plain = "This creature affects combat damage.";
        let ast = crate::parse_with_catalogs(plain, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), plain);
    }

    #[test]
    fn ring_bearer_proper_term_preserves_case() {
        // Motivating shape (One Ring to Rule Them All): the possessive noun
        // path bypasses the mid-sentence caps gate, so the lowercase lemma
        // used to render "ring-bearer"; the case-preserved proper term must
        // round-trip capitalized.
        let possessive = "Each player mills cards equal to your Ring-bearer's power.";
        let ast = crate::parse_with_catalogs(possessive, &fixture_catalogs()).into_ast();
        let rendered = source_free(&ast, "Test Card", false);
        assert_eq!(rendered, possessive);
        assert!(
            rendered.contains("Ring-bearer") && !rendered.contains("ring-bearer"),
            "the proper term must keep its capitalization: {rendered:?}"
        );

        // Mirror direction: the bare (non-possessive) proper term must keep the
        // same casing through the opaque path.
        let bare = "Put a +1/+1 counter on your Ring-bearer.";
        let ast = crate::parse_with_catalogs(bare, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), bare);
    }

    #[test]
    fn a_plural_genitive_round_trips_with_a_bare_apostrophe() {
        let plural = "Return all creatures to their owners' hands.";
        let ast = crate::parse_with_catalogs(plural, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), plural);

        // Singular control: still renders `owner's`, not `owners'` or `owner's's`.
        let singular = "Return target permanent to its owner's hand.";
        let ast = crate::parse_with_catalogs(singular, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), singular);
    }

    #[test]
    fn premodified_possessors_round_trip_both_genitive_markers() {
        // Whole-AST inverse for round `opqposs`: proves the existing generic
        // renderer inverse (determiner -> noun_phrase -> possessive_marker)
        // is total for a premodified possessor, singular and plural.
        let singular = "This creature deals damage equal to the sacrificed creature's power.";
        let ast = crate::parse_with_catalogs(singular, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), singular);

        let plural = "Return the sacrificed creatures' power to their owners.";
        let ast = crate::parse_with_catalogs(plural, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), plural);
    }

    #[test]
    fn a_nested_quoted_ability_apostrophe_stays_a_delimiter() {
        // Reef Worm: the innermost quoted ability's closing `'` follows a
        // Period token, never a plural word, so it must not be reinterpreted
        // as a bare genitive apostrophe.
        let source = "When this creature dies, create a 3/3 blue Fish creature token with \"When this token dies, create a 6/6 blue Whale creature token with 'When this token dies, create a 9/9 blue Kraken creature token.'\"";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn pre_object_adverbs_render_before_the_object() {
        let source = "Draw only one card.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!("expected a paragraph ability");
        };
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &paragraph.sentences[0].body
        else {
            panic!("expected an imperative transitive clause");
        };
        assert!(
            !predicate.pre_object_elements.is_empty(),
            "the adverb must be carried before the object: {predicate:#?}"
        );
        assert!(predicate.elements.is_empty(), "{predicate:#?}");

        for source in ["Draw only one card.", "Draw again one card."] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn post_object_adverbs_render_after_the_object() {
        let source = "Draw one card only.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = &ast.abilities[0].kind else {
            panic!("expected a paragraph ability");
        };
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &paragraph.sentences[0].body
        else {
            panic!("expected an imperative transitive clause");
        };
        assert!(predicate.pre_object_elements.is_empty(), "{predicate:#?}");
        assert!(
            !predicate.elements.is_empty(),
            "the adverb must be carried after the object: {predicate:#?}"
        );

        for source in ["Draw one card only.", "Draw one card again."] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
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
    fn capitalized_proper_name_words_do_not_start_asyndetic_imperatives() {
        let source = "Meld them into Ragnarok, Divine Deliverance.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn ordinary_auxiliary_and_infinitive_sentences_render_structurally() {
        let draw = paragraph_ability(simple(
            None,
            verb_phrase(
                Vocab::Draw,
                VerbSlot::Imperative,
                vec![VerbDependent::DirectObject(nominal(
                    Some(Determiner::Indefinite),
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
                        degree: None,
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
                Some(Determiner::Indefinite),
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
                contracted_negation: crate::features::Contraction::Full,
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
                contracted_negation: crate::features::Contraction::Contracted,
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
                    NominalModifier::Adjective {
                        polarity: Polarity::Positive,
                        phrase: AdjectivePhrase {
                            degree: None,
                            head: Adjective::Word(Vocab::Other),
                            complements: vec![],
                        },
                    },
                    NominalModifier::Noun {
                        polarity: Polarity::Positive,
                        noun: catalog_noun(&catalogs, "Goblin", false),
                    },
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
        let Clause::Independent(IndependentClause::Transitive(subject, first)) = first else {
            panic!("first coordinated fixture must be a finite transitive clause");
        };
        let ast = paragraph_ability(Clause::Independent(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(crate::syntax::Coordination::new(
                PredicateExpression::Simple(Predicate::Transitive(first)),
                crate::syntax::CoordinationJunction {
                    conjunction: Some(Conjunction::And),
                    comma: crate::features::Comma::Absent,
                },
                PredicateExpression::Simple(second),
            )),
        )));

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Other Goblin creatures you control get +1/+1 and have haste."
        );
    }

    #[test]
    fn ditransitive_and_prepositional_sentences_render_structurally() {
        let target_player = nominal(
            Some(Determiner::Target(None)),
            vec![],
            NounInstance::Singular(Noun::Word(Vocab::Player)),
            vec![],
        );
        let target_source = nominal(
            Some(Determiner::Target(None)),
            vec![],
            NounInstance::Singular(Noun::Word(Vocab::Source)),
            vec![],
        );

        // Test "Ask target player a number."
        let ask = paragraph_ability(simple(
            None,
            verb_phrase(
                Vocab::Ask,
                VerbSlot::Imperative,
                vec![
                    VerbDependent::IndirectObject(target_player.clone()),
                    VerbDependent::DirectObject(nominal(
                        Some(Determiner::Indefinite),
                        vec![],
                        NounInstance::Singular(Noun::Word(Vocab::Number)),
                        vec![],
                    )),
                ],
            ),
        ));
        assert_eq!(
            source_free(&ask, "Test Card", false),
            "Ask target player a number."
        );

        // Test "Prevent all damage from target source."
        let prevent = paragraph_ability(simple(
            None,
            verb_phrase(
                Vocab::Prevent,
                VerbSlot::Imperative,
                vec![
                    VerbDependent::DirectObject(nominal(
                        Some(Determiner::All),
                        vec![],
                        NounInstance::Mass(Noun::Word(Vocab::Damage)),
                        vec![],
                    )),
                    VerbDependent::Prepositional(PrepositionalPhrase::simple(
                        Preposition::From,
                        Phrase::NounPhrase(Box::new(target_source)),
                    )),
                ],
            ),
        ));
        assert_eq!(
            source_free(&prevent, "Test Card", false),
            "Prevent all damage from target source."
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
            flavor_header: None,
            sentences: vec![Sentence {
                body: sentence_body(simple(
                    None,
                    verb_phrase(
                        Vocab::Draw,
                        VerbSlot::Imperative,
                        vec![VerbDependent::DirectObject(nominal(
                            Some(Determiner::Indefinite),
                            vec![],
                            NounInstance::Singular(Noun::Word(Vocab::Card)),
                            vec![],
                        ))],
                    ),
                )),
            }],
        };
        let triggered = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Triggered(TriggeredAbility {
                    conditions: TriggerConditionList {
                        first: TriggerCondition {
                            introducer: TriggerWord::Whenever,
                            event: TriggerEvent::Clause(independent(simple(
                                Some(Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName))),
                                verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                            ))),
                        },
                        rest: Vec::new(),
                    },
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
            vec![NominalComplement::Prepositional(
                PrepositionalPhrase::simple(
                    Preposition::In,
                    Phrase::NounPhrase(Box::new(nominal(
                        Some(Determiner::Possessive(Possessor::Pronoun(Pronoun::You))),
                        vec![],
                        NounInstance::Singular(Noun::Word(Vocab::Hand)),
                        vec![],
                    ))),
                ),
            )],
        );
        let number = nominal(
            Some(Determiner::The),
            vec![],
            NounInstance::Singular(Noun::Word(Vocab::Number)),
            vec![NominalComplement::Prepositional(
                PrepositionalPhrase::simple(Preposition::Of, Phrase::NounPhrase(Box::new(cards))),
            )],
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
                        degree: None,
                        head: Adjective::Word(Vocab::Equal),
                        complements: vec![AdjectiveComplement::Prepositional(
                            PrepositionalPhrase::simple(
                                Preposition::To,
                                Phrase::NounPhrase(Box::new(number)),
                            ),
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
    fn recovered_spans_emit_self_reference_names_verbatim() {
        // `frobnitzes` is unknown, so the text recovers as its raw source span.
        // In the name-bearing domain the face's names are already spelled out,
        // so a recovered span round-trips byte-for-byte with no sigil expansion.
        let source = "Aang frobnitzes Aang, A Lot to Learn.";
        let ast =
            crate::parse_with_identity(source, &fixture_catalogs(), "Aang, A Lot to Learn", true)
                .into_ast();

        assert_eq!(source_free(&ast, "Aang, A Lot to Learn", true), source);
    }

    #[test]
    fn triggered_modal_headers_render_in_their_enclosing_sentence_context() {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Modal(ModalAbility {
                    frame: ModalFrame::Triggered(TriggerHeader {
                        introducer: TriggerWord::Whenever,
                        event: TriggerEvent::Clause(independent(simple(
                            Some(Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName))),
                            verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                        ))),
                        intervening_condition: None,
                    }),
                    header: Paragraph {
                        flavor_header: None,
                        // A modal header instruction is a `Choice` body — as the
                        // real parser produces — so no period is derived; the
                        // ` —` header suffix follows instead.
                        sentences: vec![Sentence {
                            body: SentenceBody::Choice(ChoiceInstruction {
                                trigger_prefix: None,
                                imperative: strict_predicate(verb_phrase(
                                    Vocab::Choose,
                                    VerbSlot::Imperative,
                                    vec![VerbDependent::Scalar(Phrase::NumberLiteral(cardinal(1)))],
                                )),
                                at_random: false,
                            }),
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

    #[test]
    fn flavor_header_renders_verbatim_with_its_em_dash_separator() {
        let ast = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: Some(FlavorHeader::new("Throw ...", 2)),
                    sentences: vec![Sentence {
                        body: SentenceBody::Recovered(RecoveredText::new("Draw a card.", 4)),
                    }],
                }),
            }],
        };
        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Throw ... — Draw a card."
        );
    }

    #[test]
    fn chapter_abilities_render_inline_after_their_header() {
        for source in [
            "I — Draw a card.",
            "I, II — Draw a card.",
            "I, II, III — Draw a card.",
            "II, III, IV — Draw a card.",
            // A flavor header stacked inside the chapter body stays on the line.
            "I — Boom! — Draw a card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn a_multi_chapter_saga_face_round_trips_line_by_line() {
        let source = "I — Draw a card.\nII, III — Draw a card.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn roll_row_abilities_render_inline_after_their_range() {
        for source in [
            "20 | Draw a card.",
            "2–9 | Create five tokens.",
            "1–9 | Draw a card.",
            "15+ | Draw a card.",
            "9 or less | Draw a card.",
            // A flavor header stacked inside the row body stays on the line.
            "1 | Trapped! — Draw a card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn a_multi_row_die_roll_table_round_trips_line_by_line() {
        let source = "20 | Draw a card.\n2–9 | Create five tokens.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn inclusive_roll_range_renders_its_dash_unspaced_from_the_ast() {
        // Direct-AST render proves the inverse independently of the parser: the
        // bounds join with an unspaced en dash — the single canonical range
        // glyph the renderer always emits.
        let arabic = |value| NumberLiteral {
            value,
            numeral: Numeral::Arabic(false),
        };
        let row = |range| OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::RollRow(RollRowAbility {
                    range,
                    body: Paragraph {
                        flavor_header: None,
                        sentences: vec![Sentence {
                            body: SentenceBody::Recovered(RecoveredText::new("Draw a card.", 4)),
                        }],
                    },
                }),
            }],
        };
        for (range, expected) in [
            (
                RollRange::Inclusive {
                    low: arabic(2),
                    high: arabic(9),
                },
                "2–9 | Draw a card.",
            ),
            (RollRange::OrMore(arabic(15)), "15+ | Draw a card."),
            (RollRange::OrLess(arabic(9)), "9 or less | Draw a card."),
            (RollRange::Single(arabic(20)), "20 | Draw a card."),
        ] {
            assert_eq!(source_free(&row(range), "Test Card", false), expected);
        }
    }

    #[test]
    fn level_band_abilities_render_across_their_striation_lines() {
        let catalogs = fixture_catalogs();
        let arabic = |value| NumberLiteral {
            value,
            numeral: Numeral::Arabic(false),
        };
        let stat = |value: i32| PowerToughness {
            power: SignedScalar {
                sign: ScalarSign::None,
                value: ScalarValue::Integer(value.unsigned_abs()),
            },
            toughness: SignedScalar {
                sign: ScalarSign::None,
                value: ScalarValue::Integer(value.unsigned_abs()),
            },
        };

        // ASCII hyphen band range, empty body: no trailing newline.
        let band_only = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::LevelBand(LevelBandAbility {
                    range: LevelRange::Band {
                        low: arabic(6),
                        high: arabic(11),
                    },
                    stats: stat(6),
                    abilities: vec![],
                }),
            }],
        };
        assert_eq!(
            source_free(&band_only, "Test Card", false),
            "LEVEL 6-11\n6/6"
        );

        // At-least range with one contained keyword ability.
        let band_with_keyword = OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::LevelBand(LevelBandAbility {
                    range: LevelRange::AtLeast(arabic(12)),
                    stats: stat(9),
                    abilities: vec![Ability {
                        ability_word: None,
                        flavor_header: None,
                        kind: AbilityKind::Keyword(KeywordAbilityList {
                            abilities: vec![KeywordAbility {
                                preceding_separator: None,
                                ability: keyword_atom(&catalogs, "flying"),
                                argument: KeywordArgument::Absent,
                            }],
                            trailing: None,
                        }),
                    }],
                }),
            }],
        };
        assert_eq!(
            source_free(&band_with_keyword, "Test Card", false),
            "LEVEL 12+\n9/9\nFlying"
        );
    }

    #[test]
    fn station_threshold_abilities_render_inline_after_their_key() {
        for source in [
            "Station\n8+ | Flying",
            "Station\n12+ | Whenever you attack, draw a card.",
        ] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
        // The separator and key shape asserted literally, independent of parsing.
        let ast =
            crate::parse_with_catalogs("Station\n8+ | Flying", &fixture_catalogs()).into_ast();
        let rendered = source_free(&ast, "Test Card", false);
        assert!(rendered.contains("8+ | Flying"), "rendered:\n{rendered}");
    }

    #[test]
    fn a_modal_choice_ability_still_renders_its_bullets() {
        // Mirror of the chapter case: a genuine modal choice keeps its `• ` bullet
        // modes; the inline chapter layout must not strip them.
        let source = "Choose one —\n• Draw a card.\n• Draw two cards.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn predicate_carrier_rejects_nominal_only_conjunction_without_unwinding() {
        let card = || {
            nominal(
                Some(Determiner::Indefinite),
                vec![],
                NounInstance::Singular(Noun::Word(Vocab::Card)),
                vec![],
            )
        };
        let first = strict_predicate(verb_phrase(
            Vocab::Draw,
            VerbSlot::Imperative,
            vec![VerbDependent::DirectObject(card())],
        ));
        let second = strict_predicate(verb_phrase(
            Vocab::Discard,
            VerbSlot::Infinitive,
            vec![VerbDependent::DirectObject(card())],
        ));
        let ast = paragraph_ability(Clause::Independent(IndependentClause::Predicated(
            None,
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(first),
                CoordinationJunction {
                    conjunction: Some(Conjunction::Plus),
                    comma: crate::features::Comma::Absent,
                },
                PredicateExpression::Simple(second),
            )),
        )));

        assert_eq!(
            ast.render("Test Card", false),
            Err(RenderError::InvalidPredicateConjunction(Conjunction::Plus))
        );
    }

    #[test]
    fn nominal_conjunction_renderer_rejects_predicate_only_conjunction() {
        assert_eq!(
            render_nominal_conjunction(Conjunction::Then),
            Err(RenderError::InvalidNominalConjunction(Conjunction::Then))
        );
    }

    fn source_free(ast: &OracleText, name: &str, legendary: bool) -> String {
        ast.render(name, legendary).unwrap()
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::new(
            ["Flying", "Deathtouch", "Haste", "Station"],
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
            (Some(subject), Predicate::Deontic(predicate)) => IndependentClause::Deontic(
                subject,
                predicate.modal,
                predicate.inner.map(|inner| *inner),
            ),
            (Some(subject), predicate @ Predicate::Attached(_)) => {
                IndependentClause::Predicated(Some(subject), PredicateExpression::Simple(predicate))
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
                flavor_header: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![Sentence {
                        body: match clause {
                            Clause::Independent(clause) => SentenceBody::Independent(clause),
                            Clause::Dependent(_) => panic!("sentence fixture must be independent"),
                        },
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
                contracted_negation: crate::features::Contraction::Contracted,
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
                        kind: crate::syntax::ObjectGap,
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

    fn degcmp_arabic(value: i32) -> NumberLiteral {
        NumberLiteral {
            value,
            numeral: Numeral::Arabic(false),
        }
    }

    #[test]
    fn adjective_phrase_renders_a_degree_measure_before_the_head() {
        // RB1: predicative path, notation-fidelity pin (design point 3).
        let renderer = Renderer::new("Test Card", false);
        let greater = || Adjective::Word(Vocab::Greater);
        let arabic_phrase = AdjectivePhrase {
            degree: Some(degcmp_arabic(2)),
            head: greater(),
            complements: vec![],
        };
        assert_eq!(
            renderer.adjective_phrase(&arabic_phrase).unwrap(),
            "2 greater"
        );
        let cardinal_phrase = AdjectivePhrase {
            degree: Some(cardinal(2)),
            head: greater(),
            complements: vec![],
        };
        assert_eq!(
            renderer.adjective_phrase(&cardinal_phrase).unwrap(),
            "two greater"
        );
    }

    #[test]
    fn nominal_modifier_adjective_renders_a_degree_measure() {
        // RB2: attributive path — unreachable from parsing (§2), but the
        // renderer must not silently drop the field.
        let renderer = Renderer::new("Test Card", false);
        let phrase = AdjectivePhrase {
            degree: Some(degcmp_arabic(2)),
            head: Adjective::Word(Vocab::Greater),
            complements: vec![AdjectiveComplement::PostnominalComparison(
                ComparisonComplement {
                    marker: ComparisonMarker::Than,
                    standard: Box::new(Phrase::NumberLiteral(cardinal(1))),
                },
            )],
        };
        let (immediate, trailing) = renderer.nominal_modifier_adjective(&phrase).unwrap();
        assert_eq!(immediate, "2 greater");
        assert_eq!(trailing, vec!["than one".to_string()]);
    }

    fn power_toughness_nominal(power: ScalarValue) -> NominalPhrase {
        NominalPhrase {
            determiner: Some(Determiner::Indefinite),
            modifiers: vec![NominalModifier::PowerToughness(PowerToughness {
                power: SignedScalar {
                    sign: ScalarSign::None,
                    value: power,
                },
                toughness: SignedScalar {
                    sign: ScalarSign::None,
                    value: power,
                },
            })],
            head: NounInstance::Singular(Noun::Word(Vocab::Token)),
            complements: vec![],
        }
    }

    #[test]
    fn indefinite_article_derives_from_the_power_toughness_onset() {
        // `Determiner::Indefinite` carries no word (see its doc comment): the
        // renderer always derives `a`/`an` from what follows, in both
        // directions, rather than trusting (and risking a mismatched) stored
        // article.
        let renderer = Renderer::new("Test Card", false);
        let x_over_x = power_toughness_nominal(ScalarValue::X);
        assert_eq!(
            renderer.nominal_phrase(&x_over_x).unwrap(),
            "an X/X token",
            "`X/X` has a vowel onset (\"ex\"), so it takes `an`"
        );
        let one_over_one = power_toughness_nominal(ScalarValue::Integer(1));
        assert_eq!(
            renderer.nominal_phrase(&one_over_one).unwrap(),
            "a 1/1 token",
            "`1/1` has a consonant onset (\"one\"), so it takes `a`"
        );
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
                negated: false,
                copula: Copula {
                    auxiliary: AuxiliaryInstance {
                        auxiliary: Auxiliary::Be,
                        inflection: auxiliary_inflection(phrase.verb.slot),
                        contracted_negation: crate::features::Contraction::Full,
                    },
                    contracted_with_subject: crate::features::Contraction::Full,
                },
                distributive_each: false,
                precomplement_adverbs: vec![],
                complement,
                adjuncts: vec![],
            });
        }

        let head = PredicateHead {
            auxiliaries: phrase.auxiliaries,
            first_auxiliary_contracted_with_subject: crate::features::Contraction::Full,
            preverb_modifiers: phrase.preverb_modifiers,
            verb: phrase.verb,
            distributive_each: false,
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
                            negated: false,
                            marker,
                            predicate: Box::new(strict_predicate(*predicate)),
                        }),
                    ));
                }
                VerbDependent::IndirectObject(noun_phrase) => {
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::IndirectObject(noun_phrase),
                    ));
                }
                VerbDependent::Prepositional(preposition) => {
                    elements.push(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                        preposition,
                    )));
                }
                VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(preposition)) => {
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::Prepositional(*preposition),
                    ));
                }
                other => panic!("unsupported predicate fixture: {other:?}"),
            }
        }
        match object {
            Some(object) => Predicate::Transitive(TransitivePredicate {
                head,
                kind: crate::syntax::Transitive {
                    pre_object_elements: Vec::new(),
                    object,
                },
                elements,
            }),
            None => Predicate::Intransitive(IntransitivePredicate {
                head,
                kind: crate::syntax::Intransitive,
                elements,
            }),
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

    fn oracle_symbol_object(surface: &str) -> PredicateObject {
        PredicateObject::OracleSymbol(OracleSymbol::new(surface).unwrap())
    }

    #[test]
    fn renders_oxford_mana_list() {
        let object = PredicateObject::Coordinated(CoordinatedPredicateObject {
            first: Box::new(oracle_symbol_object("{W}")),
            rest: vec![
                PredicateObjectCoordination {
                    conjunction: None,
                    object: oracle_symbol_object("{B}"),
                },
                PredicateObjectCoordination {
                    conjunction: Some(Conjunction::Or),
                    object: oracle_symbol_object("{G}"),
                },
            ],
        });
        let renderer = Renderer::new("Test Card", false);
        assert_eq!(
            renderer.predicate_object(&object).unwrap(),
            "{W}, {B}, or {G}"
        );
    }

    #[test]
    fn renders_and_mana_list() {
        let object = PredicateObject::Coordinated(CoordinatedPredicateObject {
            first: Box::new(oracle_symbol_object("{W}")),
            rest: vec![
                PredicateObjectCoordination {
                    conjunction: None,
                    object: oracle_symbol_object("{U}"),
                },
                PredicateObjectCoordination {
                    conjunction: None,
                    object: oracle_symbol_object("{B}"),
                },
                PredicateObjectCoordination {
                    conjunction: None,
                    object: oracle_symbol_object("{R}"),
                },
                PredicateObjectCoordination {
                    conjunction: Some(Conjunction::And),
                    object: oracle_symbol_object("{G}"),
                },
            ],
        });
        let renderer = Renderer::new("Test Card", false);
        assert_eq!(
            renderer.predicate_object(&object).unwrap(),
            "{W}, {U}, {B}, {R}, and {G}"
        );
    }

    #[test]
    fn renders_binary_mana_alternatives() {
        let object = PredicateObject::Coordinated(CoordinatedPredicateObject {
            first: Box::new(oracle_symbol_object("{R}")),
            rest: vec![PredicateObjectCoordination {
                conjunction: Some(Conjunction::Or),
                object: oracle_symbol_object("{G}"),
            }],
        });
        let renderer = Renderer::new("Test Card", false);
        assert_eq!(renderer.predicate_object(&object).unwrap(), "{R} or {G}");
    }

    #[test]
    fn renders_mixed_symbol_group_alternative() {
        let object = PredicateObject::Coordinated(CoordinatedPredicateObject {
            first: Box::new(oracle_symbol_object("{U}")),
            rest: vec![PredicateObjectCoordination {
                conjunction: Some(Conjunction::Or),
                object: PredicateObject::SymbolSequence(vec![
                    OracleSymbol::new("{C}").unwrap(),
                    OracleSymbol::new("{U}").unwrap(),
                ]),
            }],
        });
        let renderer = Renderer::new("Test Card", false);
        assert_eq!(renderer.predicate_object(&object).unwrap(), "{U} or {C}{U}");
    }

    // --- qfloat: finite verbal quantifier float ------------------------------

    #[test]
    fn distributive_each_renders_before_the_verb_and_rejects_contraction() {
        let head = PredicateHead {
            auxiliaries: vec![],
            first_auxiliary_contracted_with_subject: crate::features::Contraction::Full,
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(Vocab::Draw),
                slot: THIRD_PLURAL_PRESENT,
            },
            distributive_each: true,
        };
        let renderer = Renderer::new("Test Card", false);
        assert_eq!(renderer.predicate_head_from(&head, 0).unwrap(), "each draw");

        let contracted = PredicateHead {
            first_auxiliary_contracted_with_subject: crate::features::Contraction::Contracted,
            ..head
        };
        assert_eq!(
            renderer.predicate_head_from(&contracted, 1),
            Err(RenderError::InvalidDistributiveEachContraction),
            "the floating-`each` grammar never constructs a contracted-subject \
             surface, since `each` intervenes between the subject and the first \
             auxiliary"
        );
    }
}
