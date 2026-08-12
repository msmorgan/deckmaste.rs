use std::cell::Cell;

use crate::catalog::CatalogKind;
use crate::constructions::clause::SharedGrantBase;
use crate::constructions::clause::SharedGrantComplement;
use crate::constructions::clause::SharedGrantPrefix;
use crate::features::Conjunction;
use crate::features::Number;
use crate::features::Onset as InitialSound;
use crate::features::Person;
use crate::grammar::ContractedSubjectAuxiliary as GeneratedContractedSubjectAuxiliary;
use crate::grammar::CopularRemainder as GeneratedCopularRemainder;
use crate::grammar::SimpleClause as GeneratedSimpleClause;
use crate::grammar::VerbAnalysis as GeneratedVerb;
use crate::grammar::VerbDependent;
use crate::grammar::VerbPhrase as GeneratedVerbPhrase;
use crate::identity::short_name;
use crate::syntax::Ability;
use crate::syntax::AbilityHeader;
use crate::syntax::AbilityKind;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AttachmentPosition;
use crate::syntax::ChapterAbility;
use crate::syntax::ChoiceInstruction;
use crate::syntax::Clause;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ComparisonComplement;
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
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateExpression;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicatedArgument;
use crate::syntax::PredicatedQuality;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PrepositionalPhraseKind;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::QuotedAbility;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RenderError {
    #[error("missing {0} form")]
    MissingLexicalForm(&'static str),
    #[error("card identity is required to render this determiner")]
    CardIdentityRequired,
    #[error("the card identity has no abbreviated name")]
    AbbreviatedCardNameUnavailable,
    #[error("the following material's onset is required to render an indefinite determiner")]
    DeterminerOnsetRequired,
    /// A [`NominalComplement::KeywordArgument`] carrying a `KeywordArgument`
    /// shape the syntax never licenses in nominal-complement position (only
    /// `Costed(Symbols)` and `Predicated` are licensed there).
    #[error("keyword argument shape is not licensed in nominal-complement position")]
    InvalidKeywordArgumentNominal,
    /// A predicate, modifier, or clause carrier contains the nominal-only
    /// `plus` conjunction. This can be constructed only through the widened
    /// canonical compatibility alias, never by the grammar.
    #[error("{0:?} is not licensed as a predicate conjunction")]
    InvalidPredicateConjunction(Conjunction),
    /// A nominal, noun-phrase, or prepositional carrier contains the
    /// predicate-only `then` conjunction. This can be constructed only
    /// through the widened canonical compatibility alias, never by the
    /// grammar.
    #[error("{0:?} is not licensed as a nominal conjunction")]
    InvalidNominalConjunction(Conjunction),
    #[error("nominal AST does not match exactly one generated construction")]
    InvalidNominalConstruction,
    #[error("adjective AST does not match exactly one generated construction")]
    InvalidAdjectiveConstruction,
    #[error("determiner AST does not match exactly one generated construction")]
    InvalidDeterminerConstruction,
    #[error("noun-phrase AST does not match exactly one generated construction")]
    InvalidNounPhraseConstruction,
    #[error("prepositional AST does not match exactly one generated construction")]
    InvalidPrepositionalConstruction,
    #[error("relative-clause AST does not match exactly one generated construction")]
    InvalidRelativeConstruction,
    #[error(
        "predicate AST does not match exactly one generated construction: {problem} in {owner}, {first:?}/{second:?}, forms {first_form:?}/{second_form:?}"
    )]
    InvalidPredicateConstruction {
        problem: &'static str,
        owner: &'static str,
        first: Option<&'static str>,
        second: Option<&'static str>,
        first_form: Option<u16>,
        second_form: Option<u16>,
    },
}

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

pub(crate) fn render_cost(
    cost: &Cost,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).cost(cost)
}

pub(crate) fn render_keyword_line(
    list: &KeywordAbilityList,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).keyword_ability_list(list, false)
}

pub(crate) fn render_ability(
    ability: &Ability,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).ability(ability, true, false)
}

pub(crate) fn render_adjective_phrase(
    phrase: &AdjectivePhrase,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).adjective_phrase(phrase)
}

pub(crate) fn render_noun_phrase(
    phrase: &NounPhrase,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).noun_phrase(phrase)
}

pub(crate) fn render_infinitive_clause(
    value: &InfinitiveClause,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).infinitive_clause(value)
}

pub(crate) fn render_gerund_clause(
    value: &GerundClause,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).gerund_clause(value)
}

pub(crate) fn render_prepositional_phrase(
    value: &PrepositionalPhrase,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    Renderer::new(name, is_legendary).prepositional_phrase(value)
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GeneratedAdjectiveRender {
    pub(crate) text: String,
    pub(crate) forms: Vec<(&'static str, u16)>,
}

#[cfg(test)]
pub(crate) fn render_generated_adjective_phrase_law(
    value: &AdjectivePhrase,
) -> Result<GeneratedAdjectiveRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedAdjectiveRenderer::new(&renderer);
    GeneratedAdjectiveRenderer::accept_generated(
        crate::constructions::adjective::linearize_adjective_adjective_phrase_with(
            value,
            &mut visitor,
        ),
    )?;
    Ok(GeneratedAdjectiveRender {
        text: visitor.rendered,
        forms: visitor.forms,
    })
}

#[cfg(test)]
pub(crate) fn render_generated_rules_object_noun_phrase_law(
    value: &crate::constructions::noun_phrase::RulesObjectNounPhrase,
) -> Result<String, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedNounPhraseRenderer::new(&renderer);
    match crate::constructions::noun_phrase::linearize_rules_object_noun_phrase_with(
        value,
        &mut visitor,
    ) {
        Ok(()) => Ok(visitor.finish()),
        Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
            Err(error)
        }
        Err(_) => Err(RenderError::InvalidNounPhraseConstruction),
    }
}

/// Exercises the declaration-owned predicate inverse without routing through
/// the production predicate renderer. The argument is crate-private because
/// the staged `VerbPhrase` is an internal chart value, not a public AST
/// ingress; Task 5 uses this seam only under explicit inactive activation.
#[cfg(test)]
pub(crate) fn render_generated_predicate_verb_phrase(
    value: &GeneratedVerbPhrase,
) -> Result<String, RenderError> {
    render_generated_predicate_verb_phrase_law(value).map(|rendered| rendered.text)
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GeneratedPredicateRender {
    pub(crate) text: String,
    pub(crate) construction: &'static str,
    pub(crate) form_ordinal: u16,
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_verb_phrase_law(
    value: &GeneratedVerbPhrase,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_verb_phrase_with(value, &mut visitor),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_clause_law(value: &Clause) -> Result<String, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedClauseRenderer::new(&renderer, 0, false);
    GeneratedClauseRenderer::accept_generated(
        crate::constructions::clause::linearize_clause_clause_with(value, &mut visitor),
    )?;
    Ok(visitor.finish())
}

#[cfg(test)]
pub(crate) fn render_generated_elliptical_clause_law(
    value: &EllipticalClause,
) -> Result<String, RenderError> {
    let renderer = Renderer::new("this card", false);
    renderer.generated_elliptical_clause(value)
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_verb_law(
    value: &crate::grammar::VerbAnalysis,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_verb_with(value, &mut visitor),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_frequency_phrase_law(
    value: &crate::syntax::FrequencyPhrase,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_frequency_phrase_with(
            value,
            &mut visitor,
        ),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_counted_energy_law(
    value: &crate::syntax::CountedEnergy,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_counted_energy_with(
            value,
            &mut visitor,
        ),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_mana_amount(
    value: &PredicateObject,
) -> Result<String, RenderError> {
    render_generated_predicate_mana_amount_law(value).map(|rendered| rendered.text)
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_mana_amount_law(
    value: &PredicateObject,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_mana_amount_with(value, &mut visitor),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_mana_amount_list(
    value: &PredicateObject,
) -> Result<String, RenderError> {
    render_generated_predicate_mana_amount_list_law(value).map(|rendered| rendered.text)
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_mana_amount_list_law(
    value: &PredicateObject,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_mana_amount_list_with(
            value,
            &mut visitor,
        ),
    )?;
    visitor.finish_with_root()
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_coordinated_mana_amount(
    value: &CoordinatedPredicateObject,
) -> Result<String, RenderError> {
    render_generated_predicate_coordinated_mana_amount_law(value).map(|rendered| rendered.text)
}

#[cfg(test)]
pub(crate) fn render_generated_predicate_coordinated_mana_amount_law(
    value: &CoordinatedPredicateObject,
) -> Result<GeneratedPredicateRender, RenderError> {
    let renderer = Renderer::new("this card", false);
    let mut visitor = GeneratedPredicateRenderer::new(&renderer, 0, false, true);
    GeneratedPredicateRenderer::accept_generated(
        crate::constructions::predicate::linearize_predicate_coordinated_mana_amount_with(
            value,
            &mut visitor,
        ),
    )?;
    visitor.finish_with_root()
}

/// Whether a sentence in top-level ability context derives an outer period.
/// Parsing uses this same structural decision to reject a period-form surface
/// that rendering would necessarily erase.
pub(crate) fn sentence_form_is_admitted(
    sentence: &Sentence,
    has_outer_period: bool,
    name: &str,
    is_legendary: bool,
    nested: bool,
) -> bool {
    if !has_outer_period {
        return true;
    }
    let renderer = Renderer::new(name, is_legendary);
    if nested {
        renderer.nesting.set(1);
    }
    renderer.sentence_takes_period(sentence)
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

#[cfg(test)]
pub(crate) fn render_sentence_form(value: &Sentence, ordinal: u16) -> Result<String, RenderError> {
    let renderer = Renderer::new("", false);
    let published = sentence_terminal_quote(value).map(std::ptr::from_ref);
    renderer.terminal_quote.set(published);
    let mut visitor = GeneratedSentenceRenderer::new(&renderer, true);
    match crate::constructions::sentence::linearize_sentence_form_with(value, ordinal, &mut visitor)
    {
        Ok(()) => {
            let (surface, capitalize) = visitor.finish();
            Ok(if capitalize { capitalize_first(surface) } else { surface })
        }
        Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
            Err(error)
        }
        Err(error) => unreachable!("declared exact sentence form is selectable: {error:?}"),
    }
}

#[cfg(test)]
pub(crate) fn render_nominal_construction_form<T>(
    value: &T,
    ordinal: u16,
    linearize: impl FnOnce(
        &T,
        u16,
        &mut GeneratedNominalRenderer<'_, '_>,
    ) -> Result<
        (),
        deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
    >,
) -> Result<String, RenderError> {
    let renderer = Renderer::new("Test Card", false);
    let mut visitor = GeneratedNominalRenderer::new(&renderer);
    match linearize(value, ordinal, &mut visitor) {
        Ok(()) => Ok(visitor.finish()),
        Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
            Err(error)
        }
        Err(error) => unreachable!("declared exact nominal form is selectable: {error:?}"),
    }
}

impl Determiner {
    /// Renders a determiner that does not require card-name context.
    ///
    /// # Errors
    ///
    /// A noun-phrase possessor containing a self reference requires the
    /// [`OracleText::render`] identity arguments and is rejected here. An
    /// indefinite determiner is rejected because its article requires the
    /// following material's onset.
    pub fn render(&self) -> Result<String, RenderError> {
        if matches!(
            self.kind(),
            crate::syntax::DeterminerKind::Possessive(possessor)
                if matches!(
                    possessor,
                    crate::syntax::Possessor::NounPhrase(noun_phrase)
                        if matches!(noun_phrase.kind(), crate::syntax::NounPhraseKind::ThisCard(_))
                )
        ) {
            return Err(RenderError::CardIdentityRequired);
        }
        if matches!(self.kind(), crate::syntax::DeterminerKind::Indefinite) {
            return Err(RenderError::DeterminerOnsetRequired);
        }
        Renderer::new("", false).generated_determiner(self)
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
    quoted_abilities_remaining: usize,
    publish_last_identity_quote: bool,
}

struct GeneratedAdjectiveRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    forms: Vec<(&'static str, u16)>,
}

struct GeneratedDeterminerRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    pending_determiner: Option<Determiner>,
}

pub(crate) struct GeneratedNominalRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    pending_determiner: Option<Determiner>,
}

struct GeneratedSentenceRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    capitalize: bool,
    rendered: Option<(String, bool)>,
}

struct GeneratedNounRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: Option<String>,
}

struct GeneratedNounPhraseRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    quoted_abilities_remaining: usize,
    publish_last_identity_quote: bool,
}

struct GeneratedPrepositionalRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
}

struct GeneratedRelativeRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
}

impl<'renderer, 'identity> GeneratedRelativeRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
        }
    }

    fn push(&mut self, part: &str) {
        if part.is_empty() {
            return;
        }
        if !self.rendered.is_empty() {
            self.rendered.push(' ');
        }
        self.rendered.push_str(part);
    }

    fn finish(self) -> String {
        self.rendered
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidRelativeConstruction,
        })
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedRelativeRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the generated clause visitor exhaustively routes each declared subtree category"
    )]
    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "NounPhrase" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<NounPhrase>()
                    .expect("the relative noun-phrase hole preserves NounPhrase"),
            )?,
            "ObjectGapPredicate" => self.renderer.generated_object_gap_predicate_from(
                value
                    .downcast_ref::<crate::syntax::ObjectGapPredicate>()
                    .expect("the relative object-gap hole preserves ObjectGapPredicate"),
                0,
            )?,
            "Predicate" => self.renderer.generated_predicate_from(
                value
                    .downcast_ref::<Predicate>()
                    .expect("the relative predicate hole preserves Predicate"),
                0,
                false,
            )?,
            "AdjectivePhrase" => self.renderer.adjective_phrase(
                value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the relative adjective hole preserves AdjectivePhrase"),
            )?,
            "CoordinatedAdjectivePhrase" => self.renderer.coordinated_adjective_phrase(
                value
                    .downcast_ref::<CoordinatedAdjectivePhrase>()
                    .expect("the relative coordinated-adjective hole preserves its typed phrase"),
            )?,
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the relative prepositional hole preserves PrepositionalPhrase"),
            )?,
            other => panic!("unexpected relative subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        _codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(RenderError::InvalidRelativeConstruction)
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match (provider, value_type) {
            ("RelativeMarker", "RelativeMarker") => match value
                .downcast_ref::<RelativeMarker>()
                .expect("the relative marker identity preserves RelativeMarker")
            {
                RelativeMarker::That => "that".to_owned(),
                RelativeMarker::Who => "who".to_owned(),
                RelativeMarker::Zero => String::new(),
            },
            ("SubjectAuxiliary", "ContractedSubjectAuxiliary") => {
                let subject_auxiliary = value
                    .downcast_ref::<crate::grammar::ContractedSubjectAuxiliary>()
                    .expect("the relative contraction identity preserves SubjectAuxiliary");
                let mut rendered = self.renderer.subject(&subject_auxiliary.subject)?;
                rendered.push_str(contraction_suffix(subject_auxiliary.auxiliary)?);
                rendered
            }
            other => panic!("unexpected relative identity {other:?}"),
        };
        self.push(&rendered);
        Ok(())
    }
}

impl<'renderer, 'identity> GeneratedPrepositionalRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
        }
    }

    fn push(&mut self, part: &str) {
        if part.is_empty() {
            return;
        }
        if !self.rendered.is_empty() {
            self.rendered.push(' ');
        }
        self.rendered.push_str(part);
    }

    fn finish(self) -> String {
        self.rendered
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidPrepositionalConstruction,
        })
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedPrepositionalRenderer<'_, '_>
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
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "PrepositionalObject" => {
                Self::accept_generated(
                    crate::constructions::prepositional::linearize_prepositional_prepositional_object_with(
                        value
                            .downcast_ref::<crate::constructions::prepositional::PrepositionalObject>()
                            .expect("the prepositional object hole preserves PrepositionalObject"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "NounPhrase" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<NounPhrase>()
                    .expect("the prepositional noun-phrase object preserves NounPhrase"),
            )?,
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the nested prepositional object preserves PrepositionalPhrase"),
            )?,
            "GerundClause" => self.renderer.gerund_clause(
                value
                    .downcast_ref::<GerundClause>()
                    .expect("the prepositional gerund object preserves GerundClause"),
            )?,
            other => panic!("unexpected prepositional subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert_eq!(codec, "Adverb");
        let adverb = (value as &dyn std::any::Any)
            .downcast_ref::<Vocab>()
            .expect("the prepositional adverb object preserves Vocab");
        self.push(adverb.spelling());
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert_eq!(provider, "Preposition");
        assert_eq!(value_type, "Preposition");
        let preposition = (value as &dyn std::any::Any)
            .downcast_ref::<Preposition>()
            .expect("the prepositional identity preserves Preposition");
        self.push(render_preposition(*preposition));
        Ok(())
    }
}

impl<'renderer, 'identity> GeneratedNounPhraseRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            quoted_abilities_remaining: 0,
            publish_last_identity_quote: false,
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
        self.rendered
    }

    fn publish_terminal_quote(&mut self, count: usize, publish_last: bool) {
        self.quoted_abilities_remaining = count;
        self.publish_last_identity_quote = publish_last;
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedNounPhraseRenderer<'_, '_>
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
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "NominalPhrase" => {
                let nominal = value
                    .downcast_ref::<NominalPhrase>()
                    .expect("the noun-phrase nominal hole preserves NominalPhrase");
                let quoted_count = nominal_with_attribute_quoted_ability_count(nominal);
                let publish = self.publish_last_identity_quote
                    && quoted_count > 0
                    && self.quoted_abilities_remaining == quoted_count;
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer.terminal_quote.set(
                        nominal_with_attribute_terminal_quote(nominal).map(std::ptr::from_ref),
                    );
                }
                let rendered = self.renderer.nominal_phrase(nominal);
                self.renderer.terminal_quote.set(previous);
                self.quoted_abilities_remaining =
                    self.quoted_abilities_remaining.saturating_sub(quoted_count);
                rendered?
            }
            "RulesObjectFollowupNominal" => self.renderer.nominal_phrase(
                value
                    .downcast_ref::<crate::constructions::nominal::RulesObjectFollowupNominal>()
                    .expect("the noun-phrase rules-object hole preserves its role wrapper")
                    .as_nominal(),
            )?,
            "Quantity" => render_quantity(
                *value
                    .downcast_ref::<Quantity>()
                    .expect("the noun-phrase quantity hole preserves Quantity"),
            ),
            "NounPhrase" => {
                let noun_phrase = value
                    .downcast_ref::<NounPhrase>()
                    .expect("the recursive noun-phrase hole preserves NounPhrase");
                let quoted_count = noun_phrase_with_attribute_quoted_ability_count(noun_phrase);
                let publish = self.publish_last_identity_quote
                    && quoted_count > 0
                    && self.quoted_abilities_remaining == quoted_count;
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer.terminal_quote.set(
                        noun_phrase_with_attribute_terminal_quote(noun_phrase)
                            .map(std::ptr::from_ref),
                    );
                }
                let rendered = self.renderer.noun_phrase(noun_phrase);
                self.renderer.terminal_quote.set(previous);
                self.quoted_abilities_remaining =
                    self.quoted_abilities_remaining.saturating_sub(quoted_count);
                rendered?
            }
            other => panic!("unexpected noun-phrase subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        match codec {
            "Comma" => {
                let comma = (value as &dyn std::any::Any)
                    .downcast_ref::<crate::features::Comma>()
                    .expect("the noun-phrase comma scalar preserves Comma");
                if comma.is_present() {
                    self.push(",");
                }
            }
            other => panic!("unexpected noun-phrase scalar codec `{other}`"),
        }
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match (value_type, provider) {
            ("Pronoun", "SubjectPronoun") => self
                .renderer
                .vocabulary
                .render_pronoun(PronounInstance {
                    pronoun: *value
                        .downcast_ref::<crate::word::Pronoun>()
                        .expect("the subject identity preserves Pronoun"),
                    case: crate::word::PronounCase::Subject,
                })
                .map(str::to_owned)
                .ok_or(RenderError::MissingLexicalForm("pronoun"))?,
            ("Pronoun", "ObjectPronoun" | "Reciprocal") => self
                .renderer
                .vocabulary
                .render_pronoun(PronounInstance {
                    pronoun: *value
                        .downcast_ref::<crate::word::Pronoun>()
                        .expect("the object identity preserves Pronoun"),
                    case: crate::word::PronounCase::Object,
                })
                .map(str::to_owned)
                .ok_or(RenderError::MissingLexicalForm("pronoun"))?,
            ("ThisCardForm", "ThisCard" | "FullThisCard") => self.renderer.this_card(
                *value
                    .downcast_ref::<ThisCardForm>()
                    .expect("the self-reference identity preserves ThisCardForm"),
            )?,
            ("ThisCardForm", "PossessiveThisCard") => {
                let form = *value
                    .downcast_ref::<ThisCardForm>()
                    .expect("the possessive identity preserves ThisCardForm");
                self.renderer
                    .generated_determiner(&crate::determiner::possessive_this_card(form))?
            }
            ("Demonstrative", "Demonstrative") => value
                .downcast_ref::<crate::syntax::Demonstrative>()
                .expect("the demonstrative identity preserves Demonstrative")
                .spelling()
                .to_owned(),
            ("PartitiveHead", "PartitiveEach") => {
                let head = value
                    .downcast_ref::<crate::syntax::PartitiveHead>()
                    .expect("the partitive identity preserves PartitiveHead");
                crate::constructions::noun_phrase::partitive_head_spelling(*head).to_owned()
            }
            ("SetExceptionMarker", "SetExceptionMarker") => {
                crate::constructions::noun_phrase::set_exception_marker_spelling(
                    *value
                        .downcast_ref::<SetExceptionMarker>()
                        .expect("the exception identity preserves SetExceptionMarker"),
                )
                .to_owned()
            }
            ("Rounding", "RoundingUp" | "RoundingDown") => {
                crate::constructions::noun_phrase::rounding_spelling(
                    *value
                        .downcast_ref::<crate::syntax::Rounding>()
                        .expect("the rounding identity preserves Rounding"),
                )
                .to_owned()
            }
            other => panic!("unexpected noun-phrase identity {other:?}"),
        };
        self.push(&rendered);
        Ok(())
    }
}

struct GeneratedPredicateRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    forms: Vec<(&'static str, u16)>,
    skipped_auxiliaries: usize,
    suppress_synthetic_do: bool,
    publish_identity_quotes: bool,
    quoted_abilities_remaining: usize,
    publish_last_identity_quote: bool,
}

struct GeneratedClauseRenderer<'renderer, 'identity> {
    renderer: &'renderer Renderer<'identity>,
    rendered: String,
    quoted_abilities_remaining: usize,
    publish_terminal_quote: bool,
}

impl<'renderer, 'identity> GeneratedClauseRenderer<'renderer, 'identity> {
    fn new(
        renderer: &'renderer Renderer<'identity>,
        quoted_ability_count: usize,
        publish_terminal_quote: bool,
    ) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            quoted_abilities_remaining: quoted_ability_count,
            publish_terminal_quote,
        }
    }

    fn child_publishes_terminal_quote(&self, quoted_ability_count: usize) -> bool {
        self.publish_terminal_quote
            && quoted_ability_count > 0
            && self.quoted_abilities_remaining == quoted_ability_count
    }

    fn consume_quoted_abilities(&mut self, quoted_ability_count: usize) {
        self.quoted_abilities_remaining = self
            .quoted_abilities_remaining
            .saturating_sub(quoted_ability_count);
    }

    fn render_identity_quote(&mut self, quoted: &QuotedAbility) -> Result<String, RenderError> {
        let publish = self.child_publishes_terminal_quote(1);
        self.consume_quoted_abilities(1);
        if publish {
            let previous = self
                .renderer
                .terminal_quote
                .replace(Some(std::ptr::from_ref(quoted)));
            let rendered = self.renderer.quoted_ability(quoted);
            self.renderer.terminal_quote.set(previous);
            rendered
        } else {
            self.renderer.quoted_ability(quoted)
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
        self.rendered
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        GeneratedPredicateRenderer::accept_generated(result)
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedClauseRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the generated clause visitor exhaustively dispatches typed declaration categories"
    )]
    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "SimpleClause" => {
                let simple = value
                    .downcast_ref::<GeneratedSimpleClause>()
                    .expect("the clause hole preserves SimpleClause");
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_simple_clause_with(simple, self),
                )?;
                return Ok(());
            }
            "CoordinatedPredicateAttachment" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_coordinated_predicate_attachment_with(
                        value
                            .downcast_ref::<ClauseAttachment>()
                            .expect("the coordinated-predicate attachment hole preserves ClauseAttachment"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "CopularRemainder" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_copular_remainder_with(
                        value
                            .downcast_ref::<GeneratedCopularRemainder>()
                            .expect("the clause hole preserves CopularRemainder"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "SharedCopularPredicate" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_shared_copular_predicate_with(
                        value
                            .downcast_ref::<Predicate>()
                            .expect("the shared-copular hole preserves Predicate"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "SharedGrantComplement" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_shared_grant_complement_with(
                        value
                            .downcast_ref::<SharedGrantComplement>()
                            .expect("the shared-grant hole preserves SharedGrantComplement"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "SharedGrantPrefix" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_shared_grant_prefix_with(
                        value
                            .downcast_ref::<SharedGrantPrefix>()
                            .expect("the shared-grant prefix hole preserves SharedGrantPrefix"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "SharedGrantBase" => {
                Self::accept_generated(
                    crate::constructions::clause::linearize_clause_shared_grant_base_with(
                        value
                            .downcast_ref::<SharedGrantBase>()
                            .expect("the shared-grant base hole preserves SharedGrantBase"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "Clause" => {
                let clause = value
                    .downcast_ref::<Clause>()
                    .expect("the attachment hole preserves Clause");
                let quoted_ability_count = clause_quoted_ability_count(clause);
                let publish = self.child_publishes_terminal_quote(quoted_ability_count);
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer
                        .terminal_quote
                        .set(clause_terminal_quote(clause).map(std::ptr::from_ref));
                }
                let rendered = self.renderer.clause(clause);
                self.renderer.terminal_quote.set(previous);
                self.consume_quoted_abilities(quoted_ability_count);
                rendered?
            }
            "ExceptionRider" => {
                Self::accept_generated(
                    crate::constructions::attachment::linearize_attachment_exception_rider_with(
                        value
                            .downcast_ref::<ExceptionRider>()
                            .expect("the attachment hole preserves ExceptionRider"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "ExceptionRiderList" => {
                Self::accept_generated(
                    crate::constructions::attachment::linearize_attachment_exception_rider_list_with(
                        value
                            .downcast_ref::<crate::syntax::ExceptionRiderList>()
                            .expect("the attachment hole preserves ExceptionRiderList"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "RestrictionMember" => {
                Self::accept_generated(
                    crate::constructions::attachment::linearize_attachment_restriction_member_with(
                        value
                            .downcast_ref::<crate::syntax::RestrictionMember>()
                            .expect("the attachment hole preserves RestrictionMember"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "VerbPhrase" => {
                let value = value
                    .downcast_ref::<GeneratedVerbPhrase>()
                    .expect("the clause predicate hole preserves VerbPhrase");
                let quoted_ability_count = generated_verb_phrase_quoted_ability_count(value);
                let publish = self.child_publishes_terminal_quote(quoted_ability_count);
                let mut predicate = GeneratedPredicateRenderer::new(self.renderer, 0, false, false);
                predicate.publish_terminal_quote(quoted_ability_count, publish);
                predicate.render_verb_phrase(value)?;
                let rendered = predicate.finish();
                self.consume_quoted_abilities(quoted_ability_count);
                rendered
            }
            "GerundClause" => {
                Self::accept_generated(
                    crate::constructions::nonfinite::linearize_nonfinite_gerund_clause_with(
                        value
                            .downcast_ref::<GerundClause>()
                            .expect("the nonfinite hole preserves GerundClause"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "NounPhrase" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<NounPhrase>()
                    .expect("the clause noun-phrase hole preserves NounPhrase"),
            )?,
            "AdjectivePhrase" => self.renderer.adjective_phrase(
                value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the clause adjective hole preserves AdjectivePhrase"),
            )?,
            "CoordinatedAdjectivePhrase" => self.renderer.coordinated_adjective_phrase(
                value
                    .downcast_ref::<CoordinatedAdjectivePhrase>()
                    .expect("the clause coordinated-adjective hole preserves its typed phrase"),
            )?,
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the clause PP hole preserves PrepositionalPhrase"),
            )?,
            "Quantity" => render_quantity(
                *value
                    .downcast_ref::<Quantity>()
                    .expect("the variable subject hole preserves Quantity"),
            ),
            other => panic!("unexpected clause subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match codec {
            "Conjunction" => render_predicate_conjunction(
                *value
                    .downcast_ref::<Conjunction>()
                    .expect("the attachment scalar preserves Conjunction"),
            )?
            .to_owned(),
            "PowerToughness" => {
                let value = value
                    .downcast_ref::<crate::syntax::PowerToughness>()
                    .expect("the clause stats scalar preserves PowerToughness");
                format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness)
                )
            }
            "Numeral" => {
                let value = value
                    .downcast_ref::<NumberLiteral>()
                    .expect("the clause numeral scalar preserves NumberLiteral");
                value.numeral.format(value.value)
            }
            other => panic!("unexpected clause scalar codec `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match provider {
            "LexicalVerb" => self
                .renderer
                .vocabulary
                .render_verb_instance(
                    value
                        .downcast_ref::<GeneratedVerb>()
                        .expect("the shared-grant identity preserves VerbAnalysis")
                        .instance(),
                )
                .ok_or(RenderError::MissingLexicalForm("verb"))?,
            "AbilityItem" => render_catalog_atom(
                value
                    .downcast_ref::<crate::catalog::CatalogAtom>()
                    .expect("the shared-grant ability identity preserves CatalogAtom"),
            ),
            "QuotedAbility" => self.render_identity_quote(
                value
                    .downcast_ref::<QuotedAbility>()
                    .expect("the shared-grant quote identity preserves QuotedAbility"),
            )?,
            "Auxiliary" | "Copula" => self.renderer.render_auxiliary(
                *value
                    .downcast_ref::<crate::word::AuxiliaryInstance>()
                    .expect("the clause auxiliary identity preserves AuxiliaryInstance"),
            )?,
            "SubjectAuxiliary" => {
                let value = value
                    .downcast_ref::<GeneratedContractedSubjectAuxiliary>()
                    .expect("the contracted identity preserves its staged value");
                format!(
                    "{}{}",
                    self.renderer.subject(&value.subject)?,
                    contraction_suffix(value.auxiliary)?
                )
            }
            "Existential" => value
                .downcast_ref::<crate::syntax::ExistentialForm>()
                .expect("the existential identity preserves ExistentialForm")
                .spelling()
                .to_owned(),
            "Adverb" => value
                .downcast_ref::<Vocab>()
                .expect("the clause adverb identity preserves Vocab")
                .spelling()
                .to_owned(),
            "SentenceAdverbial" => value
                .downcast_ref::<Vocab>()
                .expect("the sentence-adverbial identity preserves Vocab")
                .spelling()
                .to_owned(),
            "Subordinator" => render_subordinator(
                *value
                    .downcast_ref::<Subordinator>()
                    .expect("the subordinator identity preserves Subordinator"),
            )
            .to_owned(),
            other => panic!("unexpected clause identity provider `{other}` for `{value_type}`"),
        };
        self.push(&rendered);
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
            other => panic!("unexpected derived attachment scalar codec `{other}`"),
        }
        Ok(())
    }
}

impl<'renderer, 'identity> GeneratedPredicateRenderer<'renderer, 'identity> {
    fn new(
        renderer: &'renderer Renderer<'identity>,
        skipped_auxiliaries: usize,
        suppress_synthetic_do: bool,
        publish_identity_quotes: bool,
    ) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            forms: Vec::new(),
            skipped_auxiliaries,
            suppress_synthetic_do,
            publish_identity_quotes,
            quoted_abilities_remaining: 0,
            publish_last_identity_quote: false,
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

    fn publish_terminal_quote(&mut self, count: usize, publish_last: bool) {
        self.quoted_abilities_remaining = count;
        self.publish_last_identity_quote = publish_last;
    }

    fn render_identity_quote(&mut self, quoted: &QuotedAbility) -> Result<String, RenderError> {
        let publish = self.publish_identity_quotes
            || (self.publish_last_identity_quote && self.quoted_abilities_remaining == 1);
        self.quoted_abilities_remaining = self.quoted_abilities_remaining.saturating_sub(1);
        if publish {
            let previous = self
                .renderer
                .terminal_quote
                .replace(Some(std::ptr::from_ref(quoted)));
            let rendered = self.renderer.quoted_ability(quoted);
            self.renderer.terminal_quote.set(previous);
            rendered
        } else {
            self.renderer.quoted_ability(quoted)
        }
    }

    fn render_verb_phrase(&mut self, value: &GeneratedVerbPhrase) -> Result<(), RenderError> {
        if let Some((predicate, VerbDependent::AbilityPostmodifier(postmodifier))) =
            value.declaration_last_dependent_parts()
        {
            self.render_verb_phrase(&predicate)?;
            self.push("with");
            let quoted = self.render_identity_quote(postmodifier.ability())?;
            self.push(&quoted);
            return Ok(());
        }
        match crate::constructions::predicate::linearize_predicate_verb_phrase_with(value, self) {
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    ..
                },
            ) => Self::accept_generated(
                crate::constructions::coordination::linearize_verb_phrase_coordinated_adjective_with(
                    value,
                    self,
                ),
            ),
            result => Self::accept_generated(result),
        }
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                group,
            } => RenderError::InvalidPredicateConstruction {
                problem: "no matching construction",
                owner: group,
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            },
            deckmaste_construction_compiler::runtime::LinearizationError::MultipleMatchingConstructions {
                group,
                first,
                second,
            } => RenderError::InvalidPredicateConstruction {
                problem: "multiple matching constructions",
                owner: group,
                first: Some(first),
                second: Some(second),
                first_form: None,
                second_form: None,
            },
            deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
                construction,
            } => RenderError::InvalidPredicateConstruction {
                problem: "no matching form",
                owner: construction,
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            },
            deckmaste_construction_compiler::runtime::LinearizationError::MultipleMatchingForms {
                construction,
                first,
                second,
            } => RenderError::InvalidPredicateConstruction {
                problem: "multiple matching forms",
                owner: construction,
                first: None,
                second: None,
                first_form: Some(first),
                second_form: Some(second),
            },
        })
    }

    fn finish(self) -> String {
        self.rendered
    }

    #[cfg(test)]
    fn finish_with_root(self) -> Result<GeneratedPredicateRender, RenderError> {
        let Some(&(construction, form_ordinal)) = self.forms.first() else {
            return Err(RenderError::InvalidPredicateConstruction {
                problem: "visitor received no selected form",
                owner: "predicate",
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            });
        };
        Ok(GeneratedPredicateRender {
            text: self.rendered,
            construction,
            form_ordinal,
        })
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedPredicateRenderer<'_, '_>
{
    type Error = RenderError;

    fn begin_form(
        &mut self,
        construction: &'static str,
        _form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        self.forms.push((construction, ordinal));
        Ok(())
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "NounPhrase" => {
                let noun_phrase = value
                    .downcast_ref::<NounPhrase>()
                    .expect("the predicate noun-phrase hole preserves NounPhrase");
                let quoted_count = noun_phrase_with_attribute_quoted_ability_count(noun_phrase);
                let publish = self.publish_last_identity_quote
                    && quoted_count > 0
                    && self.quoted_abilities_remaining == quoted_count;
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer.terminal_quote.set(
                        noun_phrase_with_attribute_terminal_quote(noun_phrase)
                            .map(std::ptr::from_ref),
                    );
                }
                let rendered = self.renderer.noun_phrase(noun_phrase);
                self.renderer.terminal_quote.set(previous);
                self.quoted_abilities_remaining =
                    self.quoted_abilities_remaining.saturating_sub(quoted_count);
                rendered?
            }
            "AdjectivePhrase" => self.renderer.adjective_phrase(
                value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the predicate adjective hole preserves AdjectivePhrase"),
            )?,
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the predicate PP hole preserves PrepositionalPhrase"),
            )?,
            "InfinitiveClause" => self.renderer.infinitive_clause(
                value
                    .downcast_ref::<InfinitiveClause>()
                    .expect("the predicate infinitive hole preserves InfinitiveClause"),
            )?,
            "Verb" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_verb_with(
                        value
                            .downcast_ref::<GeneratedVerb>()
                            .expect("the predicate Verb hole preserves VerbAnalysis"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "VerbPhrase" => {
                self.render_verb_phrase(
                    value
                        .downcast_ref::<GeneratedVerbPhrase>()
                        .expect("the recursive predicate hole preserves VerbPhrase"),
                )?;
                return Ok(());
            }
            "FrequencyPhrase" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_frequency_phrase_with(
                        value
                            .downcast_ref::<FrequencyPhrase>()
                            .expect("the frequency hole preserves FrequencyPhrase"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "CoordinatedAdjectivePhrase" => self.renderer.coordinated_adjective_phrase(
                value
                    .downcast_ref::<CoordinatedAdjectivePhrase>()
                    .expect("the coordinated-adjective hole preserves its typed value"),
            )?,
            "Quantity" => render_quantity(
                *value
                    .downcast_ref::<Quantity>()
                    .expect("the quantity hole preserves Quantity"),
            ),
            "CountedEnergy" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_counted_energy_with(
                        value
                            .downcast_ref::<crate::syntax::CountedEnergy>()
                            .expect("the counted-energy hole preserves CountedEnergy"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "ManaAmount" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_mana_amount_with(
                        value
                            .downcast_ref::<PredicateObject>()
                            .expect("the mana hole preserves PredicateObject"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "ManaAmountList" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_mana_amount_list_with(
                        value
                            .downcast_ref::<PredicateObject>()
                            .expect("the mana-list hole preserves PredicateObject"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "CoordinatedManaAmount" => {
                Self::accept_generated(
                    crate::constructions::predicate::linearize_predicate_coordinated_mana_amount_with(
                        value
                            .downcast_ref::<CoordinatedPredicateObject>()
                            .expect("the coordinated mana hole preserves its typed value"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            other => panic!("unexpected predicate subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match codec {
            "Conjunction" => render_predicate_conjunction(
                *value
                    .downcast_ref::<Conjunction>()
                    .expect("the predicate conjunction preserves Conjunction"),
            )?
            .to_owned(),
            "PowerToughness" => {
                let value = value
                    .downcast_ref::<crate::syntax::PowerToughness>()
                    .expect("the predicate stats scalar preserves PowerToughness");
                format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness)
                )
            }
            "Comma" => {
                if value
                    .downcast_ref::<crate::features::Comma>()
                    .expect("the comma scalar preserves Comma")
                    .is_present()
                {
                    ",".to_owned()
                } else {
                    String::new()
                }
            }
            other => panic!("unexpected predicate scalar codec `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match provider {
            "LexicalVerb" => self
                .renderer
                .vocabulary
                .render_verb_instance(
                    value
                        .downcast_ref::<GeneratedVerb>()
                        .expect("the lexical identity preserves VerbAnalysis")
                        .instance(),
                )
                .ok_or(RenderError::MissingLexicalForm("verb"))?,
            "Auxiliary" if self.skipped_auxiliaries > 0 => {
                self.skipped_auxiliaries -= 1;
                return Ok(());
            }
            "Auxiliary"
                if self.suppress_synthetic_do
                    && value
                        .downcast_ref::<AuxiliaryInstance>()
                        .is_some_and(|value| value.auxiliary == crate::word::Auxiliary::Do) =>
            {
                self.suppress_synthetic_do = false;
                return Ok(());
            }
            "Auxiliary" => self.renderer.render_auxiliary(
                *value
                    .downcast_ref::<AuxiliaryInstance>()
                    .expect("the auxiliary identity preserves AuxiliaryInstance"),
            )?,
            "Adverb" => value
                .downcast_ref::<Vocab>()
                .expect("the adverb identity preserves Vocab")
                .spelling()
                .to_owned(),
            "PreverbAdverb" => match value
                .downcast_ref::<PreverbModifier>()
                .expect("the preverb identity preserves PreverbModifier")
            {
                PreverbModifier::Not => "not",
                PreverbModifier::Also => "also",
                PreverbModifier::Next => "next",
            }
            .to_owned(),
            "VerbParticle" => match value
                .downcast_ref::<VerbParticle>()
                .expect("the particle identity preserves VerbParticle")
            {
                VerbParticle::Down => "down",
                VerbParticle::In => "in",
                VerbParticle::Out => "out",
                VerbParticle::Up => "up",
            }
            .to_owned(),
            "CoinResult" => match value
                .downcast_ref::<crate::syntax::CoinSide>()
                .expect("the result identity preserves CoinSide")
            {
                crate::syntax::CoinSide::Heads => "up heads",
                crate::syntax::CoinSide::Tails => "up tails",
            }
            .to_owned(),
            "Frequency" => render_frequency(
                *value
                    .downcast_ref::<FrequencyPhrase>()
                    .expect("the frequency identity preserves FrequencyPhrase"),
            ),
            "AbilityItem" => render_catalog_atom(
                value
                    .downcast_ref::<crate::catalog::CatalogAtom>()
                    .expect("the ability identity preserves CatalogAtom"),
            ),
            "QuotedAbility" => {
                let quoted = value
                    .downcast_ref::<QuotedAbility>()
                    .expect("the quoted identity preserves QuotedAbility");
                self.render_identity_quote(quoted)?
            }
            "OracleSymbol" => value
                .downcast_ref::<OracleSymbol>()
                .expect("the symbol identity preserves OracleSymbol")
                .as_str()
                .to_owned(),
            "SymbolSequence" => render_symbol_sequence(
                value
                    .downcast_ref::<Vec<OracleSymbol>>()
                    .expect("the symbol sequence identity preserves Vec<OracleSymbol>"),
            ),
            other => panic!("unexpected predicate identity provider `{other}` for `{value_type}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn derived_sequence_scalar(
        &mut self,
        _field: &'static str,
        codec: &'static str,
        _index: usize,
        _len: usize,
    ) -> Result<(), Self::Error> {
        match codec {
            "Comma" => self.push(","),
            other => panic!("unexpected derived predicate scalar codec `{other}`"),
        }
        Ok(())
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedNounRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        panic!("noun identity declarations have no literal `{literal}`")
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        panic!("noun identity declarations have no `{category}` subtree")
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        panic!("noun identity declarations have no `{codec}` scalar")
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert!(matches!(provider, "KnownNoun" | "OpaqueNoun"));
        assert_eq!(value_type, "NounInstance");
        let noun = (value as &dyn std::any::Any)
            .downcast_ref::<NounInstance>()
            .expect("noun identity retains its declared Rust type");
        self.rendered = Some(
            self.renderer
                .vocabulary
                .render_noun(noun)
                .ok_or(RenderError::MissingLexicalForm("noun"))?,
        );
        Ok(())
    }
}

impl<'renderer, 'identity> GeneratedSentenceRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>, capitalize: bool) -> Self {
        Self {
            renderer,
            capitalize,
            rendered: None,
        }
    }

    fn finish(self) -> (String, bool) {
        self.rendered
            .expect("the sentence declaration always visits its Clause hole")
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedSentenceRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        let (rendered, _) = self
            .rendered
            .as_mut()
            .expect("the sentence declaration visits its body before punctuation");
        rendered.push_str(literal);
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert_eq!(category, "Clause");
        let clause = (value as &dyn std::any::Any)
            .downcast_ref::<Clause>()
            .expect("the sentence declaration supplies its typed Clause field");
        let Clause::Independent(clause) = clause else {
            unreachable!("sentence construction admits only independent clauses")
        };
        // Adapted bind destructuring materializes the declared `Clause` value.
        // Republish the corresponding quote inside that value so the
        // identity-based quote renderer still recognizes the terminal node.
        let previous = self.renderer.terminal_quote.get();
        if previous.is_some() {
            self.renderer
                .terminal_quote
                .set(independent_clause_terminal_quote(clause).map(std::ptr::from_ref));
        }
        let rendered = self.renderer.independent_clause(clause);
        self.renderer.terminal_quote.set(previous);
        self.rendered = Some((rendered?, self.capitalize));
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        panic!("sentence declaration has no `{codec}` scalar")
    }
}

impl<'renderer, 'identity> GeneratedCoordinationRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            pending_determiner: None,
            skip_payload_subtrees: 0,
            quoted_abilities_remaining: 0,
            publish_last_identity_quote: false,
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

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidNominalConstruction,
        })
    }

    fn publish_terminal_quote(&mut self, count: usize, publish_last: bool) {
        self.quoted_abilities_remaining = count;
        self.publish_last_identity_quote = publish_last;
    }

    fn child_publishes_terminal_quote(&self, quoted_ability_count: usize) -> bool {
        self.publish_last_identity_quote
            && quoted_ability_count > 0
            && self.quoted_abilities_remaining == quoted_ability_count
    }

    fn consume_quoted_abilities(&mut self, quoted_ability_count: usize) {
        self.quoted_abilities_remaining = self
            .quoted_abilities_remaining
            .saturating_sub(quoted_ability_count);
    }
}

impl<'renderer, 'identity> GeneratedAdjectiveRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            forms: Vec::new(),
        }
    }

    fn push(&mut self, part: &str) {
        if part.is_empty() {
            return;
        }
        if !self.rendered.is_empty() {
            self.rendered.push(' ');
        }
        self.rendered.push_str(part);
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidAdjectiveConstruction,
        })
    }
}

impl<'renderer, 'identity> GeneratedDeterminerRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            pending_determiner: None,
        }
    }

    fn push(&mut self, part: &str) {
        if !part.is_empty() {
            if !self.rendered.is_empty() {
                self.rendered.push(' ');
            }
            self.rendered.push_str(part);
        }
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidDeterminerConstruction,
        })
    }

    fn finish(self) -> String {
        debug_assert!(self.pending_determiner.is_none());
        self.rendered
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedDeterminerRenderer<'_, '_>
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
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "Quantity" => render_quantity(
                *value
                    .downcast_ref::<Quantity>()
                    .expect("the determiner quantity hole preserves Quantity"),
            ),
            "Determiner" => {
                self.pending_determiner = Some(
                    value
                        .downcast_ref::<Determiner>()
                        .expect("the possessive determiner hole preserves Determiner")
                        .clone(),
                );
                return Ok(());
            }
            "PossessiveNominal" => {
                let nominal = value
                    .downcast_ref::<NominalPhrase>()
                    .expect("the possessive nominal hole preserves NominalPhrase");
                if let Some(determiner) = self.pending_determiner.take() {
                    let determiner =
                        if matches!(determiner.kind(), crate::syntax::DeterminerKind::Indefinite) {
                            indefinite_article_for(self.renderer.nominal_initial_sound(nominal)?)
                                .to_owned()
                        } else {
                            self.renderer.generated_determiner(&determiner)?
                        };
                    self.push(&determiner);
                }
                let mut visitor = GeneratedDeterminerRenderer::new(self.renderer);
                Self::accept_generated(
                    crate::constructions::determiner::linearize_determiner_possessive_nominal_with(
                        nominal,
                        &mut visitor,
                    ),
                )?;
                visitor.finish()
            }
            "AdjectivePhrase" => self.renderer.adjective_phrase(
                value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the possessive adjective hole preserves AdjectivePhrase"),
            )?,
            other => panic!("unexpected determiner subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        panic!("determiner declarations contain no `{codec}` scalar")
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match (value_type, provider) {
            ("ClosedDeterminer", "Determiner") => match *value
                .downcast_ref::<crate::syntax::ClosedDeterminer>()
                .expect("the closed determiner identity preserves its Rust type")
            {
                crate::syntax::ClosedDeterminer::The => "the".to_owned(),
                crate::syntax::ClosedDeterminer::Each => "each".to_owned(),
                crate::syntax::ClosedDeterminer::Another => "another".to_owned(),
                crate::syntax::ClosedDeterminer::Indefinite => {
                    unreachable!("indefinite spelling is derived by its enclosing nominal")
                }
                crate::syntax::ClosedDeterminer::Demonstrative(value) => {
                    value.spelling().to_owned()
                }
                crate::syntax::ClosedDeterminer::PossessivePronoun(value) => self
                    .renderer
                    .vocabulary
                    .render_possessive_pronoun(value)
                    .map(str::to_owned)
                    .ok_or(RenderError::MissingLexicalForm("possessive pronoun"))?,
                crate::syntax::ClosedDeterminer::All => "all".to_owned(),
                crate::syntax::ClosedDeterminer::Any => "any".to_owned(),
                crate::syntax::ClosedDeterminer::No => "no".to_owned(),
            },
            ("NounInstance", "PossessiveNoun") => {
                let noun = value
                    .downcast_ref::<NounInstance>()
                    .expect("the possessive noun identity preserves NounInstance");
                let mut rendered = self
                    .renderer
                    .vocabulary
                    .render_noun(noun)
                    .ok_or(RenderError::MissingLexicalForm("noun"))?;
                let plural_ending_in_s =
                    matches!(noun.kind(), crate::word::NounInstanceKind::Plural(_))
                        && (rendered.ends_with('s') || rendered.ends_with('S'));
                rendered.push_str(if plural_ending_in_s { "'" } else { "'s" });
                rendered
            }
            ("ThisCardForm", "PossessiveThisCard") => {
                let form = *value
                    .downcast_ref::<ThisCardForm>()
                    .expect("the self-reference identity preserves ThisCardForm");
                format!("{}'s", self.renderer.this_card(form)?)
            }
            other => panic!("unexpected determiner identity {other:?}"),
        };
        self.push(&rendered);
        Ok(())
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedAdjectiveRenderer<'_, '_>
{
    type Error = RenderError;

    fn begin_form(
        &mut self,
        construction: &'static str,
        _form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        self.forms.push((construction, ordinal));
        Ok(())
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "Adjective" => {
                Self::accept_generated(
                    crate::constructions::adjective::linearize_adjective_adjective_with(
                        value
                            .downcast_ref::<Adjective>()
                            .expect("the adjective hole preserves Adjective"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "AdjectivePhrase" | "ComparisonAdjectivePhrase" => {
                Self::accept_generated(
                    crate::constructions::adjective::linearize_adjective_adjective_phrase_with(
                        value
                            .downcast_ref::<AdjectivePhrase>()
                            .expect("the adjective-phrase hole preserves AdjectivePhrase"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "ComparisonComplement" => {
                Self::accept_generated(
                    crate::constructions::adjective::linearize_adjective_comparison_complement_with(
                        value
                            .downcast_ref::<ComparisonComplement>()
                            .expect("the comparison hole preserves ComparisonComplement"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "ComparisonStandard" => {
                Self::accept_generated(
                    crate::constructions::adjective::linearize_comparison_standard_with(
                        value
                            .downcast_ref::<Phrase>()
                            .expect("the comparison-standard hole preserves Phrase"),
                        self,
                    ),
                )?;
                return Ok(());
            }
            "NounPhrase" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<NounPhrase>()
                    .expect("the comparison standard preserves NounPhrase"),
            )?,
            "Clause" => self.renderer.clause(
                value
                    .downcast_ref::<Clause>()
                    .expect("the comparison standard preserves Clause"),
            )?,
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the adjective complement preserves PrepositionalPhrase"),
            )?,
            "InfinitiveClause" => self.renderer.infinitive_clause(
                value
                    .downcast_ref::<InfinitiveClause>()
                    .expect("the adjective complement preserves InfinitiveClause"),
            )?,
            other => panic!("unexpected adjective subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        match codec {
            "DegreeMeasureNumeral" => {
                let number = (value as &dyn std::any::Any)
                    .downcast_ref::<NumberLiteral>()
                    .expect("the degree scalar preserves NumberLiteral");
                if !crate::syntax::is_valid_degree_measure_number(*number) {
                    return Err(RenderError::InvalidAdjectiveConstruction);
                }
                self.push(&number.numeral.format(number.value));
            }
            other => panic!("unexpected adjective scalar codec `{other}`"),
        }
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert_eq!(provider, "Adjective");
        assert_eq!(value_type, "Adjective");
        let adjective = (value as &dyn std::any::Any)
            .downcast_ref::<Adjective>()
            .expect("the adjective identity preserves Adjective");
        let rendered = self.renderer.adjective_head(adjective)?;
        self.push(&rendered);
        Ok(())
    }
}

impl<'renderer, 'identity> GeneratedNominalRenderer<'renderer, 'identity> {
    fn new(renderer: &'renderer Renderer<'identity>) -> Self {
        Self {
            renderer,
            rendered: String::new(),
            pending_determiner: None,
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
        self.rendered
    }

    fn accept_generated(
        result: Result<
            (),
            deckmaste_construction_compiler::runtime::LinearizationError<RenderError>,
        >,
    ) -> Result<(), RenderError> {
        result.map_err(|error| match error {
            deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error) => error,
            _ => RenderError::InvalidNominalConstruction,
        })
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor
    for GeneratedNominalRenderer<'_, '_>
{
    type Error = RenderError;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.push(literal);
        Ok(())
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the total generated category adapter keeps every supported nominal hole explicit"
    )]
    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match category {
            "NounInstance" => self.renderer.render_noun(
                value
                    .downcast_ref::<NounInstance>()
                    .expect("the nominal noun hole preserves NounInstance"),
            )?,
            "AdjectivePhrase" => self.renderer.adjective_phrase(
                value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the nominal adjective hole preserves AdjectivePhrase"),
            )?,
            "NominalPhrase" => {
                let nominal = value
                    .downcast_ref::<NominalPhrase>()
                    .expect("the recursive nominal hole preserves NominalPhrase");
                if let Some(determiner) = self.pending_determiner.take() {
                    let determiner =
                        if matches!(determiner.kind(), crate::syntax::DeterminerKind::Indefinite) {
                            indefinite_article_for(self.renderer.nominal_initial_sound(nominal)?)
                                .to_owned()
                        } else {
                            self.renderer.determiner(&determiner)?
                        };
                    self.push(&determiner);
                }
                self.renderer.nominal_phrase(nominal)?
            }
            "RulesObjectNominal" => self.renderer.nominal_phrase(
                value
                    .downcast_ref::<crate::constructions::nominal::RulesObjectNominal>()
                    .expect("the rules-object base hole preserves its typed role")
                    .as_nominal(),
            )?,
            "RulesObjectFollowupNominal" => self.renderer.nominal_phrase(
                value
                    .downcast_ref::<crate::constructions::nominal::RulesObjectFollowupNominal>()
                    .expect("the rules-object followup hole preserves its typed role")
                    .as_nominal(),
            )?,
            "ReducedRecipientPassiveTheme" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<crate::constructions::nominal::ReducedRecipientPassiveTheme>()
                    .expect("the reduced-passive theme hole preserves its typed role")
                    .as_noun_phrase(),
            )?,
            "NounPhrase" => self.renderer.noun_phrase(
                value
                    .downcast_ref::<NounPhrase>()
                    .expect("the predicated-quality hole preserves NounPhrase"),
            )?,
            "Determiner" => {
                self.pending_determiner = Some(
                    value
                        .downcast_ref::<Determiner>()
                        .expect("the nominal determiner hole preserves Determiner")
                        .clone(),
                );
                return Ok(());
            }
            "Quantity" => render_quantity(
                *value
                    .downcast_ref::<Quantity>()
                    .expect("the nominal quantity hole preserves Quantity"),
            ),
            "PowerToughness" => {
                let value = value
                    .downcast_ref::<crate::syntax::PowerToughness>()
                    .expect("the nominal stats hole preserves PowerToughness");
                format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness)
                )
            }
            "PrepositionalPhrase" => self.renderer.prepositional_phrase(
                value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the nominal PP hole preserves PrepositionalPhrase"),
            )?,
            "InfinitiveClause" => self.renderer.infinitive_clause(
                value
                    .downcast_ref::<InfinitiveClause>()
                    .expect("the nominal infinitive hole preserves InfinitiveClause"),
            )?,
            "RelativeClause" => self.renderer.relative_clause(
                value
                    .downcast_ref::<RelativeClause>()
                    .expect("the nominal relative hole preserves RelativeClause"),
            )?,
            "TransitivePredicate" => self.renderer.transitive_predicate(
                value
                    .downcast_ref::<TransitivePredicate>()
                    .expect("the reduced-passive hole preserves TransitivePredicate"),
            )?,
            "ComparisonComplement" => {
                self.renderer
                    .adjective_complement(&AdjectiveComplement::PostnominalComparison(
                        value
                            .downcast_ref::<crate::syntax::ComparisonComplement>()
                            .expect("the nominal comparison hole preserves ComparisonComplement")
                            .clone(),
                    ))?
            }
            "PredicatedQualityFrom" => {
                let value = value
                    .downcast_ref::<PredicatedQuality>()
                    .expect("the keyword-quality hole preserves PredicatedQuality");
                Self::accept_generated(
                    crate::constructions::nominal::linearize_predicated_quality_from_with(
                        value, self,
                    ),
                )?;
                return Ok(());
            }
            "PredicatedQualityBare" => {
                let value = value
                    .downcast_ref::<PredicatedQuality>()
                    .expect("the keyword-quality hole preserves PredicatedQuality");
                Self::accept_generated(
                    crate::constructions::nominal::linearize_predicated_quality_bare_with(
                        value, self,
                    ),
                )?;
                return Ok(());
            }
            "PredicatedArgumentFrom" => {
                let value = value
                    .downcast_ref::<PredicatedArgument>()
                    .expect("the keyword argument hole preserves PredicatedArgument");
                Self::accept_generated(
                    crate::constructions::nominal::linearize_nominal_predicated_argument_from_with(
                        value, self,
                    ),
                )?;
                return Ok(());
            }
            "PredicatedArgumentBare" => {
                let value = value
                    .downcast_ref::<PredicatedArgument>()
                    .expect("the keyword argument hole preserves PredicatedArgument");
                Self::accept_generated(
                    crate::constructions::nominal::linearize_nominal_predicated_argument_bare_with(
                        value, self,
                    ),
                )?;
                return Ok(());
            }
            "DevotionColors" => {
                let value = value
                    .downcast_ref::<crate::syntax::DevotionColors>()
                    .expect("the devotion hole preserves DevotionColors");
                Self::accept_generated(
                    crate::constructions::nominal::linearize_nominal_devotion_colors_with(
                        value, self,
                    ),
                )?;
                return Ok(());
            }
            "IndependentClause" => self.renderer.independent_clause(
                value
                    .downcast_ref::<IndependentClause>()
                    .expect("the times-clause hole preserves IndependentClause"),
            )?,
            other => panic!("unexpected nominal subtree category `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match codec {
            "OracleSymbol" => value
                .downcast_ref::<OracleSymbol>()
                .expect("the symbol scalar preserves OracleSymbol")
                .as_str()
                .to_owned(),
            "SymbolSequence" => render_symbol_sequence(
                value
                    .downcast_ref::<Vec<OracleSymbol>>()
                    .expect("the symbol sequence scalar preserves Vec<OracleSymbol>"),
            ),
            "Conjunction" => render_nominal_conjunction(
                *value
                    .downcast_ref::<Conjunction>()
                    .expect("the nominal conjunction scalar preserves Conjunction"),
            )?
            .to_owned(),
            "ColorWord" => value
                .downcast_ref::<crate::word::ColorWord>()
                .expect("the color scalar preserves ColorWord")
                .spelling()
                .to_owned(),
            "Preposition" => render_preposition(
                *value
                    .downcast_ref::<Preposition>()
                    .expect("the predicated preposition scalar preserves Preposition"),
            )
            .to_owned(),
            other => panic!("unexpected nominal scalar codec `{other}`"),
        };
        self.push(&rendered);
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let rendered = match value_type {
            "NounInstance" => self.renderer.render_noun(
                value
                    .downcast_ref::<NounInstance>()
                    .expect("the nominal identity preserves NounInstance"),
            )?,
            "NominalModifier" => {
                assert_eq!(provider, "NegatedModifier");
                let (rendered, trailing) = self.renderer.render_nominal_modifier(
                    value
                        .downcast_ref::<NominalModifier>()
                        .expect("the negated identity preserves NominalModifier"),
                )?;
                debug_assert!(trailing.is_empty());
                rendered
            }
            "CatalogAtom" => value
                .downcast_ref::<crate::catalog::CatalogAtom>()
                .expect("the devotion identity preserves CatalogAtom")
                .render_noun(false),
            other => panic!("unexpected nominal identity type `{other}` from `{provider}`"),
        };
        self.push(&rendered);
        Ok(())
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

    #[allow(
        clippy::too_many_lines,
        reason = "the generated phrase-coordination category dispatcher is intentionally exhaustive"
    )]
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
                let quoted_count = noun_phrase_with_attribute_quoted_ability_count(noun_phrase);
                let publish = self.child_publishes_terminal_quote(quoted_count);
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer.terminal_quote.set(
                        noun_phrase_with_attribute_terminal_quote(noun_phrase)
                            .map(std::ptr::from_ref),
                    );
                }
                let rendered = self.renderer.noun_phrase(noun_phrase);
                self.renderer.terminal_quote.set(previous);
                self.consume_quoted_abilities(quoted_count);
                self.push(&rendered?);
            }
            "NominalPhrase" => {
                let nominal = value
                    .downcast_ref::<NominalPhrase>()
                    .expect("the declaration's NominalPhrase hole preserves its Rust type");
                let quoted_count = nominal_with_attribute_quoted_ability_count(nominal);
                let publish = self.child_publishes_terminal_quote(quoted_count);
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer.terminal_quote.set(
                        nominal_with_attribute_terminal_quote(nominal).map(std::ptr::from_ref),
                    );
                }
                if let Some(determiner) = self.pending_determiner.take() {
                    let determiner =
                        if matches!(determiner.kind(), crate::syntax::DeterminerKind::Indefinite) {
                            indefinite_article_for(self.renderer.nominal_initial_sound(nominal)?)
                                .to_owned()
                        } else {
                            self.renderer.determiner(&determiner)?
                        };
                    self.push(&determiner);
                }
                let rendered = self.renderer.nominal_phrase(nominal);
                self.renderer.terminal_quote.set(previous);
                self.consume_quoted_abilities(quoted_count);
                self.push(&rendered?);
            }
            "AdjectivePhrase" => {
                let adjective = value
                    .downcast_ref::<AdjectivePhrase>()
                    .expect("the coordination adjective hole preserves AdjectivePhrase");
                self.push(&self.renderer.adjective_phrase(adjective)?);
            }
            "NounInstance" => {
                let noun = value
                    .downcast_ref::<NounInstance>()
                    .expect("the coordination noun hole preserves NounInstance");
                self.push(&self.renderer.render_noun(noun)?);
            }
            "ModifierConjunct" => {
                let modifier = value
                    .downcast_ref::<NominalModifier>()
                    .expect("the modifier-conjunct hole preserves NominalModifier");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_modifier_conjunct_with(
                        modifier,
                        self,
                    ),
                )?;
            }
            "ModifierList" => {
                let list = value
                    .downcast_ref::<crate::syntax::CoordinatedModifier>()
                    .expect("the modifier-list hole preserves CoordinatedModifier");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_modifier_list_with(
                        list,
                        self,
                    ),
                )?;
            }
            "CoordinatedModifier" => {
                let coordinated = value
                    .downcast_ref::<crate::syntax::CoordinatedModifier>()
                    .expect("the coordinated-modifier hole preserves its typed value");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_coordinated_modifier_with(
                        coordinated,
                        self,
                    ),
                )?;
            }
            "CoordinatedAdjectivePhrase" => {
                let coordinated = value
                    .downcast_ref::<CoordinatedAdjectivePhrase>()
                    .expect("the coordinated-adjective hole preserves its typed value");
                self.push(&self.renderer.coordinated_adjective_phrase(coordinated)?);
            }
            "PrepositionalPhrase" => {
                let preposition = value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the coordination PP hole preserves PrepositionalPhrase");
                self.push(&self.renderer.prepositional_phrase(preposition)?);
            }
            "PrepositionalPhraseList" => {
                let list = value
                    .downcast_ref::<PrepositionalPhrase>()
                    .expect("the PP-list hole preserves PrepositionalPhrase");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_prepositional_phrase_list_with(
                        list,
                        self,
                    ),
                )?;
            }
            "PowerToughness" => {
                let value = value
                    .downcast_ref::<crate::syntax::PowerToughness>()
                    .expect("the coordination stats hole preserves PowerToughness");
                self.push(&format!(
                    "{}/{}",
                    render_signed_scalar(value.power),
                    render_signed_scalar(value.toughness),
                ));
            }
            "Quantity" => {
                let quantity = value
                    .downcast_ref::<Quantity>()
                    .expect("the counted keyword hole preserves Quantity");
                self.push(&render_quantity(*quantity));
            }
            "WithAttributeKeyword" => {
                let keyword = value
                    .downcast_ref::<crate::syntax::KeywordAbility>()
                    .expect("the with-attribute keyword hole preserves KeywordAbility");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_with_attribute_keyword_with(
                        keyword,
                        self,
                    ),
                )?;
            }
            "WithAttributeMember" => {
                let member = value
                    .downcast_ref::<crate::syntax::WithAttributeMember>()
                    .expect("the with-attribute member hole preserves its closed sum");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_with_attribute_member_with(
                        member,
                        self,
                    ),
                )?;
            }
            "WithAttributeList" => {
                let list = value
                    .downcast_ref::<crate::syntax::WithAttributeList>()
                    .expect("the with-attribute list hole preserves its typed value");
                Self::accept_generated(
                    crate::constructions::coordination::linearize_noun_coordination_with_attribute_list_with(
                        list,
                        self,
                    ),
                )?;
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
            "Comma" => {
                let comma = value
                    .downcast_ref::<crate::features::Comma>()
                    .expect("the coordination comma scalar preserves Comma");
                if comma.is_present() {
                    self.push(",");
                }
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
        _element: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        if let Some(complement) = (value as &dyn std::any::Any).downcast_ref::<NominalComplement>()
        {
            let quoted_count = nominal_complement_with_attribute_quoted_ability_count(complement);
            let publish = self.child_publishes_terminal_quote(quoted_count);
            let previous = self.renderer.terminal_quote.get();
            if publish {
                self.renderer.terminal_quote.set(
                    nominal_complement_with_attribute_terminal_quote(complement)
                        .map(std::ptr::from_ref),
                );
            }
            let rendered = self.renderer.nominal_complement(complement);
            self.renderer.terminal_quote.set(previous);
            self.consume_quoted_abilities(quoted_count);
            self.push(&rendered?);
            self.skip_payload_subtrees += 1;
        }
        Ok(())
    }

    fn identity<T: std::any::Any>(
        &mut self,
        provider: &'static str,
        value_type: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        match (provider, value_type) {
            ("NegatedModifier", "NominalModifier") => {
                let modifier = value
                    .downcast_ref::<NominalModifier>()
                    .expect("the negated-modifier identity preserves NominalModifier");
                let (prefix, trailing) = self.renderer.render_nominal_modifier(modifier)?;
                debug_assert!(trailing.is_empty());
                self.push(&prefix);
            }
            ("AbilityItem", "CatalogAtom") => {
                let ability = value
                    .downcast_ref::<crate::catalog::CatalogAtom>()
                    .expect("the ability identity preserves CatalogAtom");
                self.push(ability.spelling());
            }
            ("QuotedAbility", "QuotedAbility") => {
                let quoted = value
                    .downcast_ref::<QuotedAbility>()
                    .expect("the quoted identity preserves QuotedAbility");
                let publish =
                    self.publish_last_identity_quote && self.quoted_abilities_remaining == 1;
                self.quoted_abilities_remaining = self.quoted_abilities_remaining.saturating_sub(1);
                let previous = self.renderer.terminal_quote.get();
                if publish {
                    self.renderer
                        .terminal_quote
                        .set(Some(std::ptr::from_ref(quoted)));
                }
                let rendered = self.renderer.quoted_ability(quoted);
                self.renderer.terminal_quote.set(previous);
                self.push(&rendered?);
            }
            other => panic!("unexpected coordination identity {other:?}"),
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
        visitor.publish_terminal_quote(
            coordinated_noun_phrase_with_attribute_quoted_ability_count(value),
            self.terminal_quote_is(coordinated_noun_phrase_with_attribute_terminal_quote(value)),
        );
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
        visitor.publish_terminal_quote(
            coordinated_nominal_with_attribute_quoted_ability_count(value),
            self.terminal_quote_is(coordinated_nominal_with_attribute_terminal_quote(value)),
        );
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
        let mut rendered = String::new();
        if let Some(header) = ability.header() {
            match header {
                AbilityHeader::AbilityWord(word) => rendered.push_str(word.spelling()),
                AbilityHeader::Flavor(header) => rendered.push_str(header.text()),
            }
            rendered.push_str(" — ");
        }
        let kind = self.ability_kind(
            ability.kind(),
            capitalize || ability.header().is_some(),
            suppress_final_period,
        )?;
        if ability.header().is_some() {
            rendered.push_str(&capitalize_first(kind));
        } else {
            rendered.push_str(&kind);
        }
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
        let abilities = list.separated_abilities();
        let mut rendered = abilities.first().ability.spelling().to_owned();
        rendered.push_str(&self.keyword_argument(&abilities.first().argument)?);
        for continuation in abilities.rest() {
            rendered.push_str(match continuation.separator() {
                KeywordListSeparator::Comma => ", ",
                KeywordListSeparator::Semicolon => "; ",
            });
            let ability = continuation.value();
            rendered.push_str(ability.ability.spelling());
            rendered.push_str(&self.keyword_argument(&ability.argument)?);
        }
        if let Some(paragraph) = list.trailing() {
            // Space-separated symbol costs do not carry punctuation inside
            // their argument node. When a following paragraph establishes a
            // sentence boundary, reproduce that boundary before joining the
            // tail. Structured sentence costs retain their own terminal.
            if matches!(
                &abilities
                    .rest()
                    .last()
                    .map(|continuation| &continuation.value().argument)
                    .unwrap_or(&abilities.first().argument),
                KeywordArgument::Costed(
                    KeywordCost::Symbols(_) | KeywordCost::CoordinatedSymbols { .. }
                )
            ) {
                rendered.push('.');
            }
            rendered.push(' ');
            rendered.push_str(&self.paragraph_with_suffix(
                paragraph,
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
            KeywordCost::CoordinatedSymbols {
                first,
                conjunction,
                second,
            } => format!(
                " {} {} {}",
                render_symbol_sequence(first),
                render_nominal_conjunction(*conjunction)?,
                render_symbol_sequence(second)
            ),
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

    fn generated_predicated_argument(
        &self,
        argument: &PredicatedArgument,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedNominalRenderer::new(self);
        let result = crate::constructions::nominal::linearize_nominal_predicated_argument_with(
            argument,
            &mut visitor,
        );
        GeneratedNominalRenderer::accept_generated(result)?;
        Ok(visitor.finish())
    }

    fn generated_devotion_colors(
        &self,
        colors: crate::syntax::DevotionColors,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedNominalRenderer::new(self);
        let result = crate::constructions::nominal::linearize_nominal_devotion_colors_with(
            &colors,
            &mut visitor,
        );
        GeneratedNominalRenderer::accept_generated(result)?;
        Ok(visitor.finish())
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
                format!(" {}", self.generated_predicated_argument(predicated)?)
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
        if let Some(header) = cost.flavor_header() {
            rendered.push_str(header.text());
            rendered.push_str(" — ");
        }
        let mut saw_lexical_component = false;
        for (index, component) in cost.components().iter().enumerate() {
            if index > 0 {
                rendered.push_str(", ");
            }
            let is_symbol = matches!(component, CostComponent::Symbols(_));
            let starts_action = matches!(
                component,
                CostComponent::Clause(clause) if independent_clause_is_imperative(clause)
            );
            let capitalize = starts_action || (!saw_lexical_component && !is_symbol);
            let component = self.cost_component(component)?;
            rendered.push_str(&if capitalize { capitalize_first(component) } else { component });
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
        let form = if !force_no_period && self.sentence_takes_period(sentence) {
            crate::constructions::sentence::period_form_ordinal()
        } else {
            crate::constructions::sentence::terminal_form_ordinal()
        };
        let rendered: Result<(String, bool), RenderError> =
            if matches!(sentence.body, SentenceBody::Independent(_)) {
                let mut visitor = GeneratedSentenceRenderer::new(self, capitalize);
                match crate::constructions::sentence::linearize_sentence_form_with(
                    sentence,
                    form,
                    &mut visitor,
                ) {
                    Ok(()) => Ok(visitor.finish()),
                    Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(
                        error,
                    )) => Err(error),
                    Err(error) => unreachable!(
                        "sentence renderer selects a matching declared form: {error:?}"
                    ),
                }
            } else {
                self.sentence_body(&sentence.body, capitalize)
                    .map(|(mut rendered, capitalize)| {
                        if form == crate::constructions::sentence::period_form_ordinal() {
                            rendered.push('.');
                        }
                        (rendered, capitalize)
                    })
            };
        self.terminal_quote.set(previous);
        let (body, capitalize) = rendered?;
        Ok(if capitalize { capitalize_first(body) } else { body })
    }

    /// Renders a sentence's body, reporting whether the result still wants
    /// initial capitalization. Split out of [`Self::sentence`] so the
    /// terminal-quote channel is restored on the error path as well.
    fn sentence_body(
        &self,
        body: &SentenceBody,
        capitalize: bool,
    ) -> Result<(String, bool), RenderError> {
        Ok(match body {
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
        if let IndependentClause::Complex(complex) = clause
            && matches!(
                complex.attachment().payload(),
                ClauseAttachmentKind::Appositive(_)
            )
        {
            let matrix = self.independent_clause(complex.host())?;
            return self.clause_with_attachment(&matrix, complex.attachment());
        }
        let generated = Clause::Independent(clause.clone());
        let mut attachment_visitor = GeneratedClauseRenderer::new(
            self,
            independent_clause_quoted_ability_count(clause),
            self.terminal_quote_is(independent_clause_terminal_quote(clause)),
        );
        match crate::constructions::attachment::linearize_attachment_clause_with(
            &generated,
            &mut attachment_visitor,
        ) {
            Ok(()) => return Ok(attachment_visitor.finish()),
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    ..
                },
            ) => {}
            Err(error) => {
                GeneratedClauseRenderer::accept_generated(Err(error))?;
                unreachable!("generated attachment error is returned above")
            }
        }
        let mut visitor = GeneratedClauseRenderer::new(
            self,
            independent_clause_quoted_ability_count(clause),
            self.terminal_quote_is(independent_clause_terminal_quote(clause)),
        );
        match crate::constructions::clause::linearize_clause_clause_with(
            &generated,
            &mut visitor,
        ) {
            Ok(()) => return Ok(visitor.finish()),
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    ..
                },
            ) => {}
            Err(error) => {
                GeneratedClauseRenderer::accept_generated(Err(error))?;
                unreachable!("generated clause error is returned above")
            }
        }
        match clause {
            IndependentClause::Finite(finite) => {
                self.predicate_expression(finite.subject(), finite.predicate())
            }
            IndependentClause::Existential(_) => {
                unreachable!("sealed existential clauses render through their declaration")
            }
            IndependentClause::Complex(complex) => {
                let host = self.independent_clause(complex.host())?;
                self.clause_with_attachment(&host, complex.attachment())
            }
            IndependentClause::Coordinated(_) => Err(RenderError::InvalidPredicateConstruction {
                problem: "no matching construction",
                owner: "clause",
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            }),
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
            PredicateExpression::Coordinated(_) => {
                self.generated_coordinated_predicate_expression(subject, expression)
            }
        }
    }

    fn generated_coordinated_predicate_expression(
        &self,
        subject: Option<&Subject>,
        expression: &PredicateExpression,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedClauseRenderer::new(
            self,
            predicate_expression_quoted_ability_count(expression),
            self.terminal_quote_is(predicate_expression_terminal_quote(expression)),
        );
        GeneratedClauseRenderer::accept_generated(
            crate::constructions::clause::linearize_predicate_expression_with(
                subject,
                expression,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn predicate_with_subject(
        &self,
        subject: &Subject,
        predicate: &Predicate,
    ) -> Result<String, RenderError> {
        match predicate {
            Predicate::Transitive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, predicate.head())?;
                Ok(join_words(vec![
                    subject,
                    self.transitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Intransitive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, predicate.head())?;
                Ok(join_words(vec![
                    subject,
                    self.intransitive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Copular(predicate) => self.copular_clause(subject, predicate),
            Predicate::Passive(predicate) => {
                let (subject, auxiliary_start) =
                    self.subject_with_predicate_head(subject, predicate.head())?;
                Ok(join_words(vec![
                    subject,
                    self.passive_predicate_from(predicate, auxiliary_start)?,
                ]))
            }
            Predicate::Proform(predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.generated_predicate_from(&Predicate::Proform(*predicate), 0, false)?,
            ])),
            Predicate::Deontic(predicate) => Ok(join_words(vec![
                self.subject(subject)?,
                self.deontic_predicate(predicate)?,
            ])),
            Predicate::Attached(predicate) => {
                if let Some(rendered) = self.generated_attached_predicate(Some(subject), predicate)
                {
                    return rendered;
                }
                let matrix = self.predicate_with_subject(subject, predicate.predicate())?;
                self.clause_with_attachment(&matrix, predicate.attachment())
            }
        }
    }

    fn subject(&self, subject: &Subject) -> Result<String, RenderError> {
        self.noun_phrase(&subject.0)
    }

    fn clause_with_attachment(
        &self,
        matrix: &str,
        attachment: &ClauseAttachment,
    ) -> Result<String, RenderError> {
        let payload = self.clause_attachment(attachment.payload())?;
        let comma = if attachment.comma().is_present() { "," } else { "" };
        Ok(match attachment.position() {
            AttachmentPosition::BeforeMatrix => format!("{payload}{comma} {matrix}"),
            AttachmentPosition::AfterMatrix => format!("{matrix}{comma} {payload}"),
        })
    }

    fn clause_attachment(&self, attachment: &ClauseAttachmentKind) -> Result<String, RenderError> {
        match attachment {
            ClauseAttachmentKind::Dependent(clause) => self.dependent_clause(clause),
            ClauseAttachmentKind::Adjunct(adjunct) => self.predicate_adjunct(adjunct),
            ClauseAttachmentKind::Exception(_) | ClauseAttachmentKind::Restriction(_) => {
                unreachable!("sealed attachments render through their generated Clause owner")
            }
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
        if !head
            .first_auxiliary_contracted_with_subject()
            .is_contracted()
        {
            return Ok(0);
        }
        let auxiliary = head
            .auxiliaries()
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
                let mut parts = vec![self.render_auxiliary(predicate.copula().auxiliary())?];
                if predicate.distributive_each() {
                    parts.push("each".to_owned());
                }
                parts.push(self.copular_complement(predicate.complement())?);
                for adjunct in predicate.adjuncts() {
                    parts.push(self.predicate_adjunct(adjunct)?);
                }
                Ok(join_words(parts))
            }
            Predicate::Passive(predicate) => self.passive_predicate(predicate),
            Predicate::Proform(_) => self.generated_predicate_from(predicate, 0, false),
            Predicate::Deontic(predicate) => self.deontic_predicate(predicate),
            Predicate::Attached(predicate) => {
                if let Some(rendered) = self.generated_attached_predicate(None, predicate) {
                    return rendered;
                }
                let matrix = self.predicate(predicate.predicate())?;
                self.clause_with_attachment(&matrix, predicate.attachment())
            }
        }
    }

    fn generated_attached_predicate(
        &self,
        subject: Option<&Subject>,
        predicate: &crate::syntax::AttachedPredicate,
    ) -> Option<Result<String, RenderError>> {
        (!matches!(
            predicate.attachment().payload(),
            ClauseAttachmentKind::Appositive(_)
        ))
        .then(|| {
            // Clause coordination may move a complete attachment onto the first
            // predicate of a shared-subject coordination. Enter the declaration
            // inverse through its sealed predicate adapter. Dash appositives are
            // ability-owned and use their dedicated linearization below.
            let quoted_ability_count = predicate_quoted_ability_count(predicate.predicate())
                + clause_attachment_quoted_ability_count(predicate.attachment());
            let mut visitor = GeneratedClauseRenderer::new(
                self,
                quoted_ability_count,
                self.terminal_quote_is(attached_predicate_terminal_quote(predicate)),
            );
            GeneratedClauseRenderer::accept_generated(
                crate::constructions::attachment::linearize_attached_predicate_with(
                    subject,
                    predicate,
                    &mut visitor,
                ),
            )?;
            Ok(visitor.finish())
        })
    }

    fn deontic_predicate(
        &self,
        predicate: &crate::syntax::DeonticPredicate,
    ) -> Result<String, RenderError> {
        let publish_terminal_quote = self.terminal_quote_is(
            predicate
                .inner()
                .and_then(predicate_expression_terminal_quote),
        );
        self.deontic_predicate_from(predicate, publish_terminal_quote)
    }

    fn deontic_predicate_from(
        &self,
        predicate: &crate::syntax::DeonticPredicate,
        publish_terminal_quote: bool,
    ) -> Result<String, RenderError> {
        if matches!(
            predicate.inner(),
            Some(
                PredicateExpression::Simple(Predicate::Copular(_))
                    | PredicateExpression::Coordinated(_)
            )
        ) {
            return Ok(join_words(vec![
                self.render_auxiliary(predicate.modal().auxiliary())?,
                self.predicate_expression(
                    None,
                    predicate.inner().expect("checked inner is present"),
                )?,
            ]));
        }
        self.generated_predicate_from(
            &Predicate::Deontic(predicate.clone()),
            0,
            publish_terminal_quote,
        )
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
        self.generated_predicate_from(
            &Predicate::Transitive(predicate.clone()),
            auxiliary_start,
            self.terminal_quote_is(transitive_terminal_quote(predicate)),
        )
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
        self.generated_predicate_from(
            &Predicate::Intransitive(predicate.clone()),
            auxiliary_start,
            self.terminal_quote_is(last_element_terminal_quote(predicate.elements())),
        )
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
        self.generated_predicate_from(
            &Predicate::Passive(predicate.clone()),
            auxiliary_start,
            self.terminal_quote_is(passive_terminal_quote(predicate)),
        )
    }

    fn generated_predicate_from(
        &self,
        predicate: &Predicate,
        skipped_auxiliaries: usize,
        publish_terminal_quote: bool,
    ) -> Result<String, RenderError> {
        let head = match predicate {
            Predicate::Transitive(value) => Some(value.head()),
            Predicate::Intransitive(value) => Some(value.head()),
            Predicate::Passive(value) => Some(value.head()),
            Predicate::Attached(_)
            | Predicate::Copular(_)
            | Predicate::Deontic(_)
            | Predicate::Proform(_) => None,
        };
        let distributive_each = head.is_some_and(PredicateHead::distributive_each);
        let ellipsis = matches!(predicate, Predicate::Deontic(value) if value.inner().is_none());
        let value = crate::constructions::predicate::inverse_public_predicate(predicate).map_err(
            |error| RenderError::InvalidPredicateConstruction {
                problem: error.requirement,
                owner: error.construction,
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            },
        )?;
        self.generated_verb_phrase_from(
            &value,
            distributive_each,
            skipped_auxiliaries,
            ellipsis,
            predicate_quoted_ability_count(predicate),
            publish_terminal_quote,
        )
    }

    fn generated_object_gap_predicate_from(
        &self,
        predicate: &crate::syntax::ObjectGapPredicate,
        skipped_auxiliaries: usize,
    ) -> Result<String, RenderError> {
        let head = predicate.head();
        let value = crate::constructions::predicate::inverse_public_object_gap_predicate(predicate)
            .map_err(|error| RenderError::InvalidPredicateConstruction {
                problem: error.requirement,
                owner: error.construction,
                first: None,
                second: None,
                first_form: None,
                second_form: None,
            })?;
        self.generated_verb_phrase_from(
            &value,
            head.distributive_each(),
            skipped_auxiliaries,
            false,
            predicate
                .elements()
                .iter()
                .map(predicate_element_quoted_ability_count)
                .sum(),
            self.terminal_quote_is(last_element_terminal_quote(predicate.elements())),
        )
    }

    fn terminal_quote_is(&self, candidate: Option<&QuotedAbility>) -> bool {
        self.terminal_quote.get().is_some_and(|published| {
            candidate.is_some_and(|candidate| std::ptr::eq(published, candidate))
        })
    }

    fn generated_verb_phrase_from(
        &self,
        value: &GeneratedVerbPhrase,
        distributive_each: bool,
        skipped_auxiliaries: usize,
        suppress_synthetic_do: bool,
        quoted_ability_count: usize,
        publish_terminal_quote: bool,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedPredicateRenderer::new(
            self,
            skipped_auxiliaries,
            suppress_synthetic_do,
            false,
        );
        visitor.publish_terminal_quote(quoted_ability_count, publish_terminal_quote);
        visitor.render_verb_phrase(value)?;
        let rendered = visitor.finish();
        Ok(if distributive_each { format!("each {rendered}") } else { rendered })
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
            PredicateAdjunct::AbilityPostmodifier(postmodifier) => Ok(format!(
                "with {}",
                self.quoted_ability(postmodifier.ability())?
            )),
            PredicateAdjunct::Dependent(clause) => self.dependent_clause(clause),
        }
    }

    fn copular_clause(
        &self,
        subject: &Subject,
        predicate: &CopularPredicate,
    ) -> Result<String, RenderError> {
        let subject = self.subject(subject)?;
        let complement = self.copular_complement(predicate.complement())?;
        let mut parts = Vec::with_capacity(predicate.adjuncts().len() + 2);
        if predicate.copula().contracted_with_subject().is_contracted() {
            parts.push(format!(
                "{subject}{}",
                contraction_suffix(predicate.copula().auxiliary())?
            ));
        } else {
            parts.push(subject);
            parts.push(self.render_auxiliary(predicate.copula().auxiliary())?);
        }
        if predicate.negated() {
            parts.push("not".to_owned());
        }
        if predicate.distributive_each() {
            parts.push("each".to_owned());
        }
        parts.extend(
            predicate
                .precomplement_adverbs()
                .iter()
                .map(|adverb| adverb.spelling().to_owned()),
        );
        parts.push(complement);
        for adjunct in predicate.adjuncts() {
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

    fn generated_elliptical_clause(
        &self,
        clause: &EllipticalClause,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedClauseRenderer::new(self, 0, false);
        GeneratedClauseRenderer::accept_generated(
            crate::constructions::clause::linearize_clause_elliptical_clause_with(
                clause,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn dependent_clause(&self, clause: &DependentClause) -> Result<String, RenderError> {
        match clause {
            DependentClause::Subordinate(subordinator, body) => {
                let body = match body {
                    SubordinateBody::Finite(clause) => self.independent_clause(clause)?,
                    SubordinateBody::CoordinatedFinite(body) => format!(
                        "{} {} {} {}",
                        self.independent_clause(body.first())?,
                        body.conjunction().spelling(),
                        render_subordinator(body.repeated_subordinator()),
                        self.independent_clause(body.next())?,
                    ),
                    SubordinateBody::Infinitive(clause) => self.infinitive_clause(clause)?,
                    SubordinateBody::Gerund(clause) => self.gerund_clause(clause)?,
                    SubordinateBody::Elliptical(clause) => {
                        self.generated_elliptical_clause(clause)?
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
        let mut visitor = GeneratedClauseRenderer::new(self, 0, false);
        GeneratedClauseRenderer::accept_generated(
            crate::constructions::nonfinite::linearize_nonfinite_gerund_clause_with(
                clause,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn infinitive_clause(&self, clause: &InfinitiveClause) -> Result<String, RenderError> {
        if clause.marker() == InfinitiveMarker::Bare {
            let predicate = self.predicate(clause.predicate())?;
            return Ok(if clause.negated() { format!("not {predicate}") } else { predicate });
        }
        let mut visitor = GeneratedClauseRenderer::new(self, 0, false);
        GeneratedClauseRenderer::accept_generated(
            crate::constructions::nonfinite::linearize_nonfinite_infinitive_clause_with(
                clause,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn relative_clause(&self, clause: &RelativeClause) -> Result<String, RenderError> {
        let mut visitor = GeneratedRelativeRenderer::new(self);
        GeneratedRelativeRenderer::accept_generated(
            crate::constructions::relative::linearize_relative_relative_clause_with(
                clause,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn noun_phrase(&self, phrase: &NounPhrase) -> Result<String, RenderError> {
        let mut visitor = GeneratedNounPhraseRenderer::new(self);
        visitor.publish_terminal_quote(
            noun_phrase_with_attribute_quoted_ability_count(phrase),
            self.terminal_quote_is(noun_phrase_with_attribute_terminal_quote(phrase)),
        );
        match crate::constructions::noun_phrase::linearize_noun_phrase_noun_phrase_with(
            phrase,
            &mut visitor,
        ) {
            Ok(()) => Ok(visitor.finish()),
            Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
                Err(error)
            }
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    ..
                },
            ) => match phrase.kind() {
                crate::syntax::NounPhraseKind::CoordinatedNominal(coordinated) => {
                    self.coordinated_nominal_phrase(coordinated)
                }
                crate::syntax::NounPhraseKind::Coordinated(coordinated) => {
                    self.coordinated_noun_phrase(coordinated)
                }
                _ => Err(RenderError::InvalidNounPhraseConstruction),
            },
            Err(_) => Err(RenderError::InvalidNounPhraseConstruction),
        }
    }

    fn nominal_phrase(&self, phrase: &NominalPhrase) -> Result<String, RenderError> {
        let mut visitor = GeneratedNominalRenderer::new(self);
        match crate::constructions::nominal::linearize_nominal_nominal_phrase_with(
            phrase,
            &mut visitor,
        ) {
            Ok(()) => Ok(visitor.finish()),
            Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
                Err(error)
            }
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    ..
                },
            ) => {
                let mut visitor = GeneratedCoordinationRenderer::new(self);
                let quoted_count = nominal_with_attribute_quoted_ability_count(phrase);
                visitor.publish_terminal_quote(
                    quoted_count,
                    self.terminal_quote_is(nominal_with_attribute_terminal_quote(phrase)),
                );
                let result = crate::constructions::coordination::linearize_noun_coordination_nominal_phrase_with(
                    phrase,
                    &mut visitor,
                );
                finish_generated_coordination(result, visitor)
            }
            Err(_) => Err(RenderError::InvalidNominalConstruction),
        }
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
            NominalComplement::Devotion(colors) => {
                format!("to {}", self.generated_devotion_colors(*colors)?)
            }
            NominalComplement::EventClause(clause) => self.independent_clause(clause)?,
            NominalComplement::WithAttributes(attributes) => {
                let mut visitor = GeneratedCoordinationRenderer::new(self);
                let quoted_count =
                    nominal_complement_with_attribute_quoted_ability_count(complement);
                visitor.publish_terminal_quote(
                    quoted_count,
                    self.terminal_quote_is(nominal_complement_with_attribute_terminal_quote(
                        complement,
                    )),
                );
                let result = crate::constructions::coordination::linearize_noun_coordination_with_attribute_list_with(
                    attributes,
                    &mut visitor,
                );
                finish_generated_coordination(result, visitor)?
            }
            NominalComplement::KeywordArgument(argument) => match argument {
                KeywordArgument::Costed(KeywordCost::Symbols(symbols)) => {
                    render_symbol_sequence(symbols)
                }
                KeywordArgument::Predicated(predicated) => {
                    self.generated_predicated_argument(predicated)?
                }
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
                    apply_polarity(*polarity, head, adjective_is_rules_bundle(phrase.head())),
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
                let mut visitor = GeneratedCoordinationRenderer::new(self);
                let result = crate::constructions::coordination::linearize_noun_coordination_coordinated_modifier_with(
                    coordinated,
                    &mut visitor,
                );
                Ok((finish_generated_coordination(result, visitor)?, Vec::new()))
            }
        }
    }

    fn nominal_initial_sound(&self, phrase: &NominalPhrase) -> Result<InitialSound, RenderError> {
        if let Some(first) = phrase.modifiers().first() {
            return self.modifier_initial_sound(first);
        }
        self.noun_initial_sound(phrase.head())
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
            NominalModifier::Adjective { phrase, .. } => {
                self.adjective_initial_sound(phrase.head())
            }
            NominalModifier::Noun { noun, .. } => self.noun_initial_sound(noun),
            // Always literally `declare …` — a fixed consonant onset.
            NominalModifier::CombatStepName { .. } => Ok(InitialSound::Consonant),
            NominalModifier::Quantity(quantity) => {
                Ok(surface_initial_sound(&render_quantity(*quantity)))
            }
            NominalModifier::PowerToughness(value) => Ok(value.initial_sound()),
            // A coordinated slot's leading surface is its first conjunct's.
            NominalModifier::Coordinated(coordinated) => {
                self.modifier_initial_sound(coordinated.first())
            }
        }
    }

    fn noun_initial_sound(&self, noun: &NounInstance) -> Result<InitialSound, RenderError> {
        match noun.noun() {
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
            Noun::Agentive(_) => self
                .vocabulary
                .render_noun(noun)
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
        self.generated_determiner(determiner)
    }

    fn generated_determiner(&self, determiner: &Determiner) -> Result<String, RenderError> {
        let mut visitor = GeneratedDeterminerRenderer::new(self);
        GeneratedDeterminerRenderer::accept_generated(
            crate::constructions::determiner::linearize_determiner_determiner_with(
                determiner,
                &mut visitor,
            ),
        )?;
        Ok(visitor.finish())
    }

    fn adjective_phrase(&self, phrase: &AdjectivePhrase) -> Result<String, RenderError> {
        let mut visitor = GeneratedAdjectiveRenderer::new(self);
        GeneratedAdjectiveRenderer::accept_generated(
            crate::constructions::adjective::linearize_adjective_adjective_phrase_with(
                phrase,
                &mut visitor,
            ),
        )?;
        Ok(visitor.rendered)
    }

    fn coordinated_adjective_phrase(
        &self,
        coordinated: &CoordinatedAdjectivePhrase,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedCoordinationRenderer::new(self);
        let result =
            crate::constructions::coordination::linearize_coordinated_adjective_phrase_with(
                coordinated,
                &mut visitor,
            );
        finish_generated_coordination(result, visitor)
    }

    fn nominal_modifier_adjective(
        &self,
        phrase: &AdjectivePhrase,
    ) -> Result<(String, Vec<String>), RenderError> {
        let (immediate_phrase, comparison) = phrase
            .clone()
            .try_split_postnominal_comparison()
            .map_or_else(
                || (phrase.clone(), None),
                |(owner, comparison)| (owner, Some(comparison)),
            );
        let immediate = self.adjective_phrase(&immediate_phrase)?;
        let trailing = comparison
            .map(|comparison| {
                self.adjective_complement(&AdjectiveComplement::PostnominalComparison(comparison))
            })
            .transpose()?
            .into_iter()
            .collect();
        Ok((immediate, trailing))
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
                let mut visitor = GeneratedAdjectiveRenderer::new(self);
                match crate::constructions::adjective::linearize_adjective_comparison_complement_with(
                    comparison,
                    &mut visitor,
                ) {
                    Ok(()) => visitor.rendered,
                    Err(
                        deckmaste_construction_compiler::runtime::LinearizationError::Visitor(
                            error,
                        ),
                    ) => return Err(error),
                    Err(_) => return Err(RenderError::InvalidAdjectiveConstruction),
                }
            }
            AdjectiveComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition)?
            }
            AdjectiveComplement::Infinitive(infinitive) => self.infinitive_clause(infinitive)?,
        })
    }

    fn prepositional_phrase(&self, phrase: &PrepositionalPhrase) -> Result<String, RenderError> {
        match phrase.kind() {
            PrepositionalPhraseKind::Simple(_) => self.generated_prepositional_phrase(phrase),
            PrepositionalPhraseKind::Coordinated(_) => {
                let mut visitor = GeneratedCoordinationRenderer::new(self);
                let result = crate::constructions::coordination::linearize_noun_coordination_prepositional_phrase_with(
                    phrase,
                    &mut visitor,
                );
                finish_generated_coordination(result, visitor)
            }
        }
    }

    fn generated_prepositional_phrase(
        &self,
        phrase: &PrepositionalPhrase,
    ) -> Result<String, RenderError> {
        let mut visitor = GeneratedPrepositionalRenderer::new(self);
        GeneratedPrepositionalRenderer::accept_generated(
            crate::constructions::prepositional::linearize_simple_with(phrase, &mut visitor),
        )?;
        Ok(visitor.finish())
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
                quoted.ability.kind(),
                AbilityKind::Keyword(list)
                    if matches!(
                        list.abilities().last().map(|ability| &ability.argument),
                        Some(
                            KeywordArgument::Absent
                                | KeywordArgument::Costed(KeywordCost::Symbols(_))
                                | KeywordArgument::Costed(KeywordCost::CoordinatedSymbols { .. })
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
        let mut visitor = GeneratedNounRenderer {
            renderer: self,
            rendered: None,
        };
        match crate::constructions::noun::linearize_with(noun, &mut visitor) {
            Ok(()) => Ok(visitor
                .rendered
                .expect("noun declaration always visits its identity field")),
            Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
                Err(error)
            }
            Err(_) => Err(RenderError::InvalidNounPhraseConstruction),
        }
    }

    fn this_card(&self, form: ThisCardForm) -> Result<String, RenderError> {
        let rendered = match form {
            ThisCardForm::AbbreviatedName => self
                .short_name
                .ok_or(RenderError::AbbreviatedCardNameUnavailable)?,
            ThisCardForm::FullName => self.name,
        };
        if rendered.is_empty() {
            Err(RenderError::CardIdentityRequired)
        } else {
            Ok(rendered.to_owned())
        }
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
    let IndependentClause::Finite(finite) = clause else {
        return false;
    };
    match finite.subject() {
        None => predicate_expression_verb_is_enchant(finite.predicate()),
        Some(subject) => subject_is_enchant_keyword(subject),
    }
}

fn independent_clause_is_imperative(clause: &IndependentClause) -> bool {
    matches!(clause, IndependentClause::Finite(finite) if finite.subject().is_none())
}

fn predicate_expression_verb_is_enchant(expression: &PredicateExpression) -> bool {
    matches!(expression, PredicateExpression::Simple(predicate) if predicate_verb_is_enchant(predicate))
}

fn predicate_verb_is_enchant(predicate: &Predicate) -> bool {
    if let Predicate::Attached(predicate) = predicate {
        return predicate_verb_is_enchant(predicate.predicate());
    }
    let head = match predicate {
        Predicate::Transitive(predicate) => predicate.head(),
        Predicate::Intransitive(predicate) => predicate.head(),
        Predicate::Passive(predicate) => predicate.head(),
        Predicate::Copular(_)
        | Predicate::Proform(_)
        | Predicate::Deontic(_)
        | Predicate::Attached(_) => return false,
    };
    head.verb().verb == Verb::Word(Vocab::Enchant)
}

/// Whether a subject is the bare `Enchant` keyword-ability atom (the misparse
/// of `Enchant <adjective> <type>` that treats the keyword as the subject
/// noun).
fn subject_is_enchant_keyword(subject: &Subject) -> bool {
    let crate::syntax::NounPhraseKind::Nominal(nominal) = subject.0.kind() else {
        return false;
    };
    if !nominal.modifiers().is_empty() || nominal.determiner().is_some() {
        return false;
    }
    matches!(
        nominal.head().noun(),
        Noun::Catalog(atom)
            if atom.kind == CatalogKind::KeywordAbility && atom.canonical() == "Enchant"
    )
}

/// The [`ThisCardForm`] of a clause's final rendered constituent when that
/// constituent is a self-reference, else `None`. Used to suppress the derived
/// period when the card's name already ends in terminal punctuation.
fn independent_clause_final_self_reference(clause: &IndependentClause) -> Option<ThisCardForm> {
    match clause {
        IndependentClause::Finite(finite) => {
            predicate_expression_final_self_reference(finite.predicate())
        }
        IndependentClause::Coordinated(coordination) => match coordination.rest().last() {
            Some(member) => match member.member() {
                CoordinatedClauseMember::Independent(clause) => {
                    independent_clause_final_self_reference(clause)
                }
            },
            None => independent_clause_final_self_reference(coordination.first()),
        },
        IndependentClause::Complex(complex) => {
            let attachment = complex.attachment();
            if attachment.position() == AttachmentPosition::AfterMatrix {
                match attachment.payload() {
                    ClauseAttachmentKind::Appositive(clause) => {
                        independent_clause_final_self_reference(clause)
                    }
                    _ => None,
                }
            } else {
                independent_clause_final_self_reference(complex.host())
            }
        }
        IndependentClause::Existential(_) => None,
    }
}

fn predicate_expression_final_self_reference(
    expression: &PredicateExpression,
) -> Option<ThisCardForm> {
    match expression {
        PredicateExpression::Simple(predicate) => predicate_final_self_reference(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .last()
            .and_then(predicate_expression_final_self_reference),
    }
}

fn predicate_final_self_reference(predicate: &Predicate) -> Option<ThisCardForm> {
    let predicate = match predicate {
        Predicate::Transitive(predicate) => predicate,
        Predicate::Deontic(predicate) => {
            return predicate
                .inner()
                .and_then(predicate_expression_final_self_reference);
        }
        Predicate::Attached(attached) => {
            let attachment = attached.attachment();
            if attachment.position() == AttachmentPosition::AfterMatrix {
                return match attachment.payload() {
                    ClauseAttachmentKind::Appositive(clause) => {
                        independent_clause_final_self_reference(clause)
                    }
                    _ => None,
                };
            }
            return predicate_final_self_reference(attached.predicate());
        }
        Predicate::Intransitive(_)
        | Predicate::Copular(_)
        | Predicate::Passive(_)
        | Predicate::Proform(_) => return None,
    };
    if !predicate.elements().is_empty() {
        return None;
    }
    let PredicateObject::NounPhrase(noun_phrase) = predicate.object() else {
        return None;
    };
    let crate::syntax::NounPhraseKind::ThisCard(form) = noun_phrase.kind() else {
        return None;
    };
    Some(*form)
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

/// Whether sentence structure itself supplies a terminator without renderer
/// context such as a card name or modal-header suffix.
pub(crate) fn sentence_has_structural_terminator(sentence: &Sentence) -> bool {
    matches!(sentence.body, SentenceBody::Recovered(_))
        || sentence_terminal_quote(sentence).is_some()
}

/// The closed quoted ability a clause's final *rendered* constituent is, if
/// any — the single node whose interior absorbs the enclosing sentence's
/// terminal period. Returning the node rather than a bare `bool` is what lets
/// [`Renderer::quoted_ability`] derive a period placement the AST no longer
/// stores: the sentence renderer publishes this node's address, and the quote
/// that recognizes itself in it keeps its interior period.
fn independent_clause_terminal_quote(clause: &IndependentClause) -> Option<&QuotedAbility> {
    match clause {
        IndependentClause::Finite(finite) => {
            predicate_expression_terminal_quote(finite.predicate())
        }
        IndependentClause::Coordinated(clause) => coordinated_terminal_quote(clause),
        IndependentClause::Complex(clause) => complex_terminal_quote(clause),
        IndependentClause::Existential(_) => None,
    }
}

fn independent_clause_quoted_ability_count(clause: &IndependentClause) -> usize {
    match clause {
        IndependentClause::Finite(finite) => {
            predicate_expression_quoted_ability_count(finite.predicate())
        }
        IndependentClause::Complex(value) => {
            independent_clause_quoted_ability_count(value.host())
                + clause_attachment_quoted_ability_count(value.attachment())
        }
        IndependentClause::Coordinated(value) => {
            independent_clause_quoted_ability_count(value.first())
                + value
                    .rest()
                    .iter()
                    .map(|member| match member.member() {
                        CoordinatedClauseMember::Independent(clause) => {
                            independent_clause_quoted_ability_count(clause)
                        }
                    })
                    .sum::<usize>()
        }
        IndependentClause::Existential(_) => 0,
    }
}

fn clause_quoted_ability_count(clause: &Clause) -> usize {
    match clause {
        Clause::Independent(clause) => independent_clause_quoted_ability_count(clause),
        Clause::Dependent(clause) => dependent_clause_quoted_ability_count(clause),
    }
}

fn clause_terminal_quote(clause: &Clause) -> Option<&QuotedAbility> {
    match clause {
        Clause::Independent(clause) => independent_clause_terminal_quote(clause),
        Clause::Dependent(_) => None,
    }
}

fn generated_verb_phrase_quoted_ability_count(value: &GeneratedVerbPhrase) -> usize {
    crate::constructions::predicate::project_public_predicate(value.clone())
        .map_or(0, |value| predicate_quoted_ability_count(&value.predicate))
}

fn clause_attachment_quoted_ability_count(attachment: &ClauseAttachment) -> usize {
    match attachment.payload() {
        ClauseAttachmentKind::Dependent(clause) => dependent_clause_quoted_ability_count(clause),
        ClauseAttachmentKind::Adjunct(adjunct) => predicate_adjunct_quoted_ability_count(adjunct),
        ClauseAttachmentKind::Exception(rider) => {
            independent_clause_quoted_ability_count(rider.first())
                + rider
                    .rest()
                    .iter()
                    .map(|member| independent_clause_quoted_ability_count(member.clause()))
                    .sum::<usize>()
        }
        ClauseAttachmentKind::Restriction(run) => {
            restriction_member_quoted_ability_count(run.first())
                + run
                    .rest()
                    .iter()
                    .map(|member| restriction_member_quoted_ability_count(member.member()))
                    .sum::<usize>()
        }
        ClauseAttachmentKind::Appositive(clause) => independent_clause_quoted_ability_count(clause),
    }
}

fn restriction_member_quoted_ability_count(member: &crate::syntax::RestrictionMember) -> usize {
    member
        .adjuncts()
        .iter()
        .map(predicate_adjunct_quoted_ability_count)
        .sum()
}

fn dependent_clause_quoted_ability_count(clause: &DependentClause) -> usize {
    match clause {
        DependentClause::Subordinate(_, body) => match body {
            SubordinateBody::Finite(clause) => independent_clause_quoted_ability_count(clause),
            SubordinateBody::CoordinatedFinite(body) => {
                independent_clause_quoted_ability_count(body.first())
                    + independent_clause_quoted_ability_count(body.next())
            }
            SubordinateBody::Infinitive(clause) => {
                predicate_quoted_ability_count(clause.predicate())
            }
            SubordinateBody::Gerund(clause) => gerund_clause_quoted_ability_count(clause),
            SubordinateBody::Elliptical(_) => 0,
        },
        DependentClause::Infinitive(clause) => predicate_quoted_ability_count(clause.predicate()),
        DependentClause::Gerund(clause) => gerund_clause_quoted_ability_count(clause),
        DependentClause::Relative(clause) => match clause.body() {
            crate::syntax::RelativeBody::SubjectGap(predicate) => {
                predicate_quoted_ability_count(predicate)
            }
            crate::syntax::RelativeBody::ObjectGap { predicate, .. } => predicate
                .elements()
                .iter()
                .map(predicate_element_quoted_ability_count)
                .sum(),
        },
    }
}

fn gerund_clause_quoted_ability_count(clause: &GerundClause) -> usize {
    match clause.kind() {
        crate::syntax::GerundClauseKind::Base { predicate } => {
            predicate_quoted_ability_count(predicate)
        }
        crate::syntax::GerundClauseKind::RatherThan {
            matrix,
            alternative,
        } => {
            gerund_clause_quoted_ability_count(matrix)
                + gerund_clause_quoted_ability_count(alternative)
        }
    }
}

fn predicate_adjunct_quoted_ability_count(adjunct: &PredicateAdjunct) -> usize {
    match adjunct {
        PredicateAdjunct::Dependent(clause) => dependent_clause_quoted_ability_count(clause),
        PredicateAdjunct::AbilityPostmodifier(_) => 1,
        PredicateAdjunct::Adverb(_)
        | PredicateAdjunct::Frequency(_)
        | PredicateAdjunct::Temporal(_)
        | PredicateAdjunct::Manner(_)
        | PredicateAdjunct::Prepositional(_)
        | PredicateAdjunct::Exception(_) => 0,
    }
}

fn predicate_expression_quoted_ability_count(expression: &PredicateExpression) -> usize {
    match expression {
        PredicateExpression::Simple(predicate) => predicate_quoted_ability_count(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .iter()
            .map(predicate_expression_quoted_ability_count)
            .sum(),
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
        Predicate::Intransitive(predicate) => last_element_terminal_quote(predicate.elements()),
        Predicate::Passive(predicate) => passive_terminal_quote(predicate),
        Predicate::Copular(predicate) => copular_terminal_quote(predicate),
        Predicate::Proform(_) => None,
        Predicate::Deontic(predicate) => predicate
            .inner()
            .and_then(predicate_expression_terminal_quote),
        Predicate::Attached(predicate) => attached_predicate_terminal_quote(predicate),
    }
}

fn predicate_quoted_ability_count(predicate: &Predicate) -> usize {
    match predicate {
        Predicate::Transitive(predicate) => transitive_predicate_quoted_ability_count(predicate),
        Predicate::Intransitive(predicate) => predicate
            .elements()
            .iter()
            .map(predicate_element_quoted_ability_count)
            .sum(),
        Predicate::Passive(predicate) => passive_predicate_quoted_ability_count(predicate),
        Predicate::Deontic(predicate) => predicate
            .inner()
            .map_or(0, predicate_expression_quoted_ability_count),
        Predicate::Attached(predicate) => {
            predicate_quoted_ability_count(predicate.predicate())
                + clause_attachment_quoted_ability_count(predicate.attachment())
        }
        Predicate::Copular(_) | Predicate::Proform(_) => 0,
    }
}

fn transitive_predicate_quoted_ability_count(predicate: &TransitivePredicate) -> usize {
    predicate_object_quoted_ability_count(predicate.object())
        + predicate
            .pre_object_elements()
            .iter()
            .chain(predicate.elements().iter())
            .map(predicate_element_quoted_ability_count)
            .sum::<usize>()
}

fn passive_predicate_quoted_ability_count(predicate: &crate::syntax::PassivePredicate) -> usize {
    predicate
        .retained_object()
        .map_or(0, predicate_object_quoted_ability_count)
        + predicate
            .elements()
            .iter()
            .map(predicate_element_quoted_ability_count)
            .sum::<usize>()
}

fn predicate_element_quoted_ability_count(element: &PredicateElement) -> usize {
    match element {
        PredicateElement::Complement(PredicateComplement::Infinitive(infinitive)) => {
            predicate_quoted_ability_count(infinitive.predicate())
        }
        PredicateElement::Adjunct(PredicateAdjunct::AbilityPostmodifier(_)) => 1,
        PredicateElement::Complement(_)
        | PredicateElement::Adjunct(_)
        | PredicateElement::Particle(_)
        | PredicateElement::CoinResult(_) => 0,
    }
}

fn predicate_object_quoted_ability_count(object: &PredicateObject) -> usize {
    match object {
        PredicateObject::QuotedAbility(_) => 1,
        PredicateObject::NounPhrase(noun_phrase) => {
            noun_phrase_with_attribute_quoted_ability_count(noun_phrase)
        }
        PredicateObject::Ability(ability) => ability
            .argument
            .as_deref()
            .map_or(0, predicate_object_quoted_ability_count),
        PredicateObject::Coordinated(coordination) => {
            predicate_object_quoted_ability_count(&coordination.first)
                + coordination
                    .rest
                    .iter()
                    .map(|member| predicate_object_quoted_ability_count(&member.object))
                    .sum::<usize>()
        }
        PredicateObject::Quantity(_)
        | PredicateObject::CountedEnergy(_)
        | PredicateObject::OracleSymbol(_)
        | PredicateObject::SymbolSequence(_)
        | PredicateObject::PowerToughness(_)
        | PredicateObject::EmbeddedAbility(_) => 0,
    }
}

fn noun_phrase_with_attribute_quoted_ability_count(noun_phrase: &NounPhrase) -> usize {
    match noun_phrase.kind() {
        crate::syntax::NounPhraseKind::Nominal(nominal) => {
            nominal_with_attribute_quoted_ability_count(nominal)
        }
        crate::syntax::NounPhraseKind::Partitive(partitive) => {
            noun_phrase_with_attribute_quoted_ability_count(&partitive.whole)
        }
        crate::syntax::NounPhraseKind::AnyNumberOf(value) => {
            noun_phrase_with_attribute_quoted_ability_count(value.complement())
        }
        crate::syntax::NounPhraseKind::CoordinatedNominal(value) => {
            coordinated_nominal_with_attribute_quoted_ability_count(value)
        }
        crate::syntax::NounPhraseKind::Coordinated(value) => {
            coordinated_noun_phrase_with_attribute_quoted_ability_count(value)
        }
        crate::syntax::NounPhraseKind::SetException(value) => {
            noun_phrase_with_attribute_quoted_ability_count(&value.included)
                + noun_phrase_with_attribute_quoted_ability_count(&value.excluded)
        }
        crate::syntax::NounPhraseKind::Arithmetic(crate::syntax::ArithmeticValue::Minus {
            left,
            right,
        }) => {
            noun_phrase_with_attribute_quoted_ability_count(left)
                + noun_phrase_with_attribute_quoted_ability_count(right)
        }
        crate::syntax::NounPhraseKind::Arithmetic(crate::syntax::ArithmeticValue::Half {
            value,
            ..
        }) => noun_phrase_with_attribute_quoted_ability_count(value),
        crate::syntax::NounPhraseKind::Pronoun { .. }
        | crate::syntax::NounPhraseKind::PossessiveThisCard(_)
        | crate::syntax::NounPhraseKind::Demonstrative(_)
        | crate::syntax::NounPhraseKind::Quantity(_)
        | crate::syntax::NounPhraseKind::ThisCard(_)
        | crate::syntax::NounPhraseKind::TargetsBeyondFirst => 0,
    }
}

fn coordinated_nominal_with_attribute_quoted_ability_count(
    value: &crate::syntax::CoordinatedNominalPhrase,
) -> usize {
    nominal_with_attribute_quoted_ability_count(value.first())
        + value
            .rest()
            .iter()
            .map(|member| nominal_with_attribute_quoted_ability_count(&member.phrase))
            .sum::<usize>()
        + value
            .complements()
            .iter()
            .map(nominal_complement_with_attribute_quoted_ability_count)
            .sum::<usize>()
}

fn coordinated_noun_phrase_with_attribute_quoted_ability_count(
    value: &crate::syntax::CoordinatedNounPhrase,
) -> usize {
    noun_phrase_with_attribute_quoted_ability_count(value.first())
        + value
            .rest()
            .iter()
            .map(|member| noun_phrase_with_attribute_quoted_ability_count(&member.phrase))
            .sum::<usize>()
}

fn nominal_with_attribute_quoted_ability_count(nominal: &NominalPhrase) -> usize {
    nominal
        .complements()
        .iter()
        .map(nominal_complement_with_attribute_quoted_ability_count)
        .sum()
}

fn nominal_complement_with_attribute_quoted_ability_count(complement: &NominalComplement) -> usize {
    let NominalComplement::WithAttributes(attributes) = complement else {
        return 0;
    };
    usize::from(matches!(
        attributes.first(),
        crate::syntax::WithAttributeMember::Quoted(_)
    )) + attributes
        .rest()
        .iter()
        .filter(|continuation| {
            matches!(
                continuation.member(),
                crate::syntax::WithAttributeMember::Quoted(_)
            )
        })
        .count()
}

fn attached_predicate_terminal_quote(
    predicate: &crate::syntax::AttachedPredicate,
) -> Option<&QuotedAbility> {
    let attachment = predicate.attachment();
    match attachment.position() {
        AttachmentPosition::AfterMatrix => clause_attachment_terminal_quote(attachment),
        AttachmentPosition::BeforeMatrix => predicate_terminal_quote(predicate.predicate()),
    }
}

/// A transitive predicate ends with its final adjunct/complement element, or —
/// when it has none — with its object (`this creature gains "…"` leaves the
/// quoted ability as the object with no trailing element).
fn transitive_terminal_quote(predicate: &TransitivePredicate) -> Option<&QuotedAbility> {
    if predicate.elements().is_empty() {
        predicate_object_terminal_quote(predicate.object())
    } else {
        last_element_terminal_quote(predicate.elements())
    }
}

/// A passive predicate ends with its final element, or — when it has none —
/// with its retained object, mirroring [`transitive_terminal_quote`]'s object
/// fallback; ordinary passives (no retained object, no elements) never end in a
/// closed quote here.
fn passive_terminal_quote(predicate: &crate::syntax::PassivePredicate) -> Option<&QuotedAbility> {
    if predicate.elements().is_empty() {
        predicate
            .retained_object()
            .and_then(predicate_object_terminal_quote)
    } else {
        last_element_terminal_quote(predicate.elements())
    }
}

fn copular_terminal_quote(predicate: &CopularPredicate) -> Option<&QuotedAbility> {
    predicate.adjuncts().last().and_then(adjunct_terminal_quote)
}

fn coordinated_terminal_quote(clause: &CoordinatedIndependentClause) -> Option<&QuotedAbility> {
    match clause.rest().last() {
        Some(coordination) => match coordination.member() {
            CoordinatedClauseMember::Independent(clause) => {
                independent_clause_terminal_quote(clause)
            }
        },
        None => independent_clause_terminal_quote(clause.first()),
    }
}

/// A complex clause renders its after-matrix attachments after the matrix, so
/// the tail is the last such attachment when present, and the matrix otherwise.
fn complex_terminal_quote(clause: &ComplexClause) -> Option<&QuotedAbility> {
    let attachment = clause.attachment();
    match attachment.position() {
        AttachmentPosition::AfterMatrix => clause_attachment_terminal_quote(attachment),
        AttachmentPosition::BeforeMatrix => independent_clause_terminal_quote(clause.host()),
    }
}

fn clause_attachment_terminal_quote(attachment: &ClauseAttachment) -> Option<&QuotedAbility> {
    match attachment.payload() {
        ClauseAttachmentKind::Adjunct(adjunct) => adjunct_terminal_quote(adjunct),
        ClauseAttachmentKind::Dependent(_) => None,
        ClauseAttachmentKind::Exception(rider) => match rider.rest().last() {
            Some(conjunct) => independent_clause_terminal_quote(conjunct.clause()),
            None => independent_clause_terminal_quote(rider.first()),
        },
        ClauseAttachmentKind::Restriction(run) => match run.rest().last() {
            Some(member) => member
                .member()
                .adjuncts()
                .last()
                .and_then(adjunct_terminal_quote),
            None => run
                .first()
                .adjuncts()
                .last()
                .and_then(adjunct_terminal_quote),
        },
        ClauseAttachmentKind::Appositive(clause) => independent_clause_terminal_quote(clause),
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
        PredicateComplement::Prepositional(_)
        | PredicateComplement::IndirectObject(_)
        | PredicateComplement::Adjective(_)
        | PredicateComplement::CoordinatedAdjective(_)
        | PredicateComplement::Infinitive(_) => None,
    }
}

fn adjunct_terminal_quote(adjunct: &PredicateAdjunct) -> Option<&QuotedAbility> {
    match adjunct {
        PredicateAdjunct::AbilityPostmodifier(postmodifier) => Some(postmodifier.ability()),
        PredicateAdjunct::Adverb(_)
        | PredicateAdjunct::Frequency(_)
        | PredicateAdjunct::Temporal(_)
        | PredicateAdjunct::Manner(_)
        | PredicateAdjunct::Prepositional(_)
        | PredicateAdjunct::Exception(_)
        | PredicateAdjunct::Dependent(_) => None,
    }
}

/// The closed quoted ability an object position terminates with, if any. A
/// coordination's tail member supplies it, so `has "…" and "…."` yields the
/// second quote — the only one the enclosing sentence's period moves inside.
fn predicate_object_terminal_quote(object: &PredicateObject) -> Option<&QuotedAbility> {
    match object {
        PredicateObject::QuotedAbility(quoted) => Some(quoted),
        PredicateObject::NounPhrase(noun_phrase) => {
            noun_phrase_with_attribute_terminal_quote(noun_phrase)
        }
        PredicateObject::Coordinated(coordinated) => coordinated_object_terminal_quote(coordinated),
        _ => None,
    }
}

/// Finds a mixed-`with` quote only along the noun phrase's final rendered
/// constituent. The quoted-ability count walks every child; this walk follows
/// the corresponding surface tail, stopping before a trailing rounding rider.
fn noun_phrase_with_attribute_terminal_quote(noun_phrase: &NounPhrase) -> Option<&QuotedAbility> {
    match noun_phrase.kind() {
        crate::syntax::NounPhraseKind::Nominal(nominal) => {
            nominal_with_attribute_terminal_quote(nominal)
        }
        crate::syntax::NounPhraseKind::Partitive(partitive) => {
            noun_phrase_with_attribute_terminal_quote(&partitive.whole)
        }
        crate::syntax::NounPhraseKind::AnyNumberOf(value) => {
            noun_phrase_with_attribute_terminal_quote(value.complement())
        }
        crate::syntax::NounPhraseKind::CoordinatedNominal(value) => {
            coordinated_nominal_with_attribute_terminal_quote(value)
        }
        crate::syntax::NounPhraseKind::Coordinated(value) => {
            coordinated_noun_phrase_with_attribute_terminal_quote(value)
        }
        crate::syntax::NounPhraseKind::SetException(value) => {
            noun_phrase_with_attribute_terminal_quote(&value.excluded)
        }
        crate::syntax::NounPhraseKind::Arithmetic(crate::syntax::ArithmeticValue::Minus {
            right,
            ..
        }) => noun_phrase_with_attribute_terminal_quote(right),
        crate::syntax::NounPhraseKind::Arithmetic(crate::syntax::ArithmeticValue::Half {
            value,
            rounding: None,
        }) => noun_phrase_with_attribute_terminal_quote(value),
        crate::syntax::NounPhraseKind::Arithmetic(crate::syntax::ArithmeticValue::Half {
            rounding: Some(_),
            ..
        })
        | crate::syntax::NounPhraseKind::Pronoun { .. }
        | crate::syntax::NounPhraseKind::PossessiveThisCard(_)
        | crate::syntax::NounPhraseKind::Demonstrative(_)
        | crate::syntax::NounPhraseKind::Quantity(_)
        | crate::syntax::NounPhraseKind::ThisCard(_)
        | crate::syntax::NounPhraseKind::TargetsBeyondFirst => None,
    }
}

fn coordinated_nominal_with_attribute_terminal_quote(
    value: &crate::syntax::CoordinatedNominalPhrase,
) -> Option<&QuotedAbility> {
    if let Some(complement) = value.complements().last() {
        return nominal_complement_with_attribute_terminal_quote(complement);
    }
    match value.rest().last() {
        Some(member) => nominal_with_attribute_terminal_quote(&member.phrase),
        None => nominal_with_attribute_terminal_quote(value.first()),
    }
}

fn coordinated_noun_phrase_with_attribute_terminal_quote(
    value: &crate::syntax::CoordinatedNounPhrase,
) -> Option<&QuotedAbility> {
    match value.rest().last() {
        Some(member) => noun_phrase_with_attribute_terminal_quote(&member.phrase),
        None => noun_phrase_with_attribute_terminal_quote(value.first()),
    }
}

fn nominal_with_attribute_terminal_quote(nominal: &NominalPhrase) -> Option<&QuotedAbility> {
    nominal_complement_with_attribute_terminal_quote(nominal.complements().last()?)
}

fn nominal_complement_with_attribute_terminal_quote(
    complement: &NominalComplement,
) -> Option<&QuotedAbility> {
    let NominalComplement::WithAttributes(attributes) = complement else {
        return None;
    };
    let member = attributes
        .rest()
        .last()
        .map_or(attributes.first(), |continuation| continuation.member());
    match member {
        crate::syntax::WithAttributeMember::Quoted(quoted) => Some(quoted),
        crate::syntax::WithAttributeMember::Keyword(_) => None,
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

/// The indefinite article's surface word for a given initial sound. See
/// `crate::determiner::indefinite()`: the AST never stores which word was
/// written, so every render site derives it here from the following material's
/// initial sound, reusing `IndefiniteArticle::spelling()` rather than inlining
/// the literal words.
fn indefinite_article_for(sound: InitialSound) -> &'static str {
    match sound {
        InitialSound::Consonant => IndefiniteArticle::A.spelling(),
        InitialSound::Vowel => IndefiniteArticle::An.spelling(),
    }
}

fn render_quantity(quantity: Quantity) -> String {
    let mut renderer = GeneratedQuantityRenderer::default();
    crate::constructions::quantity::linearize_quantity_group_with(&quantity, &mut renderer)
        .expect("every Quantity variant dispatches to one declared form");
    renderer.parts.join(" ")
}

#[derive(Default)]
struct GeneratedQuantityRenderer {
    parts: Vec<String>,
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor for GeneratedQuantityRenderer {
    type Error = std::convert::Infallible;

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.parts.push(literal.to_owned());
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        unreachable!("quantity declarations contain no {category} subtree")
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        let spelling = match codec {
            "Numeral" => value
                .downcast_ref::<crate::syntax::NumberLiteral>()
                .map(|number| number.numeral.format(number.value))
                .or_else(|| {
                    value
                        .downcast_ref::<crate::syntax::QuantityValue>()
                        .map(|value| match value {
                            crate::syntax::QuantityValue::Literal(number) => {
                                number.numeral.format(number.value)
                            }
                            crate::syntax::QuantityValue::Variable => "X".to_owned(),
                        })
                })
                .expect("Numeral scalar preserves NumberLiteral or QuantityValue"),
            "ComparativeWord" => value
                .downcast_ref::<crate::syntax::ComparativeWord>()
                .expect("ComparativeWord scalar preserves its finite enum")
                .spelling()
                .to_owned(),
            other => unreachable!("quantity declaration used unknown scalar codec {other}"),
        };
        self.parts.push(spelling);
        Ok(())
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
    match noun.noun() {
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

#[cfg(test)]
mod render_quantity_value_tests {
    use super::render_quantity;
    use crate::Numeral;
    use crate::syntax::ComparativeWord;
    use crate::syntax::NumberLiteral;
    use crate::syntax::Quantity;
    use crate::syntax::QuantityValue;

    #[test]
    fn variable_bounds_render_as_x() {
        assert_eq!(
            render_quantity(Quantity::unchecked_up_to(QuantityValue::Variable)),
            "up to X"
        );
        assert_eq!(
            render_quantity(Quantity::unchecked_or_comparison(
                QuantityValue::Variable,
                ComparativeWord::Less
            )),
            "X or less"
        );
        // Inverse completeness for the unwitnessed arms — the inverse must be
        // total even where the corpus is silent, per *Productions ship their
        // inverse*.
        assert_eq!(
            render_quantity(Quantity::unchecked_at_least(QuantityValue::Variable)),
            "at least X"
        );
        assert_eq!(
            render_quantity(Quantity::unchecked_more_than(QuantityValue::Variable)),
            "more than X"
        );
        assert_eq!(
            render_quantity(Quantity::unchecked_fewer_than(QuantityValue::Variable)),
            "fewer than X"
        );
        assert_eq!(
            render_quantity(Quantity::unchecked_or_comparison(
                QuantityValue::Variable,
                ComparativeWord::Greater
            )),
            "X or greater"
        );
    }

    #[test]
    fn generated_quantity_inverse_covers_every_variant_and_notation() {
        let number = |value, numeral| NumberLiteral { value, numeral };
        for (numeral, expected) in [
            (Numeral::Cardinal, "three"),
            (Numeral::Ordinal, "third"),
            (Numeral::Arabic(false), "3"),
            (Numeral::Arabic(true), "3"),
            (Numeral::Roman, "III"),
        ] {
            assert_eq!(
                render_quantity(Quantity::unchecked_exact(number(3, numeral))),
                expected
            );
        }
        let two = QuantityValue::Literal(number(2, Numeral::Cardinal));
        for (quantity, expected) in [
            (Quantity::unchecked_at_least(two), "at least two"),
            (
                Quantity::unchecked_or_comparison(two, ComparativeWord::Fewer),
                "two or fewer",
            ),
            (
                Quantity::unchecked_or(
                    number(1, Numeral::Cardinal),
                    number(2, Numeral::Arabic(false)),
                ),
                "one or 2",
            ),
            (Quantity::unchecked_x(), "X"),
            (Quantity::unchecked_both(), "both"),
            (Quantity::unchecked_up_to(two), "up to two"),
            (Quantity::unchecked_that_many(), "that many"),
            (Quantity::unchecked_that_much(), "that much"),
            (Quantity::unchecked_more_than(two), "more than two"),
            (Quantity::unchecked_fewer_than(two), "fewer than two"),
        ] {
            assert_eq!(render_quantity(quantity), expected);
        }
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
    use super::GeneratedAdjectiveRenderer;
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

    #[test]
    fn production_renderer_does_not_reconstruct_grammar_carriers() {
        let source = include_str!("renderer.rs");
        let production = source
            .split_once("#[cfg(test)]\nmod tests {")
            .expect("renderer tests remain in their cfg(test) module")
            .0;
        assert!(
            !production.contains("from_declaration_parts"),
            "active rendering must consume sealed values through construction inverses"
        );
        let unchecked_agentive = ["NounInstance::unchecked_singular", "(Noun::Agentive"].concat();
        assert!(
            !source.contains(&unchecked_agentive),
            "agentive spelling must reuse the sealed noun instance"
        );
    }

    #[test]
    fn conditioned_predicate_coordination_renders_through_sealed_adapters() {
        let source = "This creature gets +0/+2 as long as you control a Plains, has flying as long as you control an Island, gets +2/+0 as long as you control a Swamp, has first strike as long as you control a Mountain, and has trample as long as you control a Forest.";
        let catalogs = Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["First strike", "Flying", "Trample"],
            )
            .with_catalog(CatalogKind::CardType, ["Creature"])
            .with_catalog(
                CatalogKind::LandType,
                ["Forest", "Island", "Mountain", "Plains", "Swamp"],
            );
        let report = crate::parse_with_catalogs(source, &catalogs);
        assert!(report.ast().recoveries().is_empty(), "{report:#?}");
        assert_eq!(
            report.ast().render("Test Card", false),
            Ok(source.to_owned())
        );
    }

    fn checked_ability(kind: AbilityKind) -> Ability {
        crate::ability::build_ability(None, kind).expect("renderer fixture is a valid ability")
    }

    fn render_degree_measure_scalar(number: NumberLiteral) -> Result<String, RenderError> {
        let renderer = Renderer::new("Test Card", false);
        let mut visitor = GeneratedAdjectiveRenderer::new(&renderer);
        deckmaste_construction_compiler::runtime::LinearizationVisitor::scalar(
            &mut visitor,
            "DegreeMeasureNumeral",
            &number,
        )?;
        Ok(visitor.rendered)
    }

    #[test]
    fn degree_measure_scalar_sink_rejects_non_degree_notations() {
        assert_eq!(
            render_degree_measure_scalar(NumberLiteral {
                value: 2,
                numeral: Numeral::Cardinal,
            }),
            Ok("two".to_owned()),
        );
        assert_eq!(
            render_degree_measure_scalar(NumberLiteral {
                value: 2,
                numeral: Numeral::Arabic(false),
            }),
            Ok("2".to_owned()),
        );
        for measure in [
            NumberLiteral {
                value: 2,
                numeral: Numeral::Ordinal,
            },
            NumberLiteral {
                value: 10,
                numeral: Numeral::Roman,
            },
            NumberLiteral {
                value: 2_000,
                numeral: Numeral::Arabic(true),
            },
        ] {
            assert_eq!(
                render_degree_measure_scalar(measure),
                Err(RenderError::InvalidAdjectiveConstruction),
                "non-degree notation reached the scalar sink: {measure:?}",
            );
        }
    }

    #[derive(Debug)]
    struct VerbPhrase {
        auxiliaries: Vec<AuxiliaryInstance>,
        preverb_modifiers: Vec<PreverbModifier>,
        verb: VerbInstance,
        frame: crate::word::PredicateFrame,
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
    fn mixed_with_attributes_round_trip_with_terminal_quote_punctuation() {
        let catalogs = fixture_catalogs()
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Haste", "Toxic", "First strike", "Vigilance"],
            )
            .with_catalog(
                CatalogKind::CreatureType,
                ["Goblin", "Alien", "Angel", "Phyrexian", "Mite"],
            )
            .with_catalog(CatalogKind::CardType, ["Creature", "Artifact"]);
        for source in [
            "Create a 1/1 red Alien creature token with haste and \"This token attacks each combat if able.\"",
            "Create two 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\"",
            "Create a 2/2 black Alien Angel artifact creature token with first strike, vigilance, and \"Whenever an opponent casts a creature spell, this token isn't a creature until end of turn.\"",
        ] {
            let report = crate::parse_with_catalogs(source, &catalogs);
            assert!(
                report.diagnostics().is_empty(),
                "{source}: {:?}",
                report.diagnostics(),
            );
            assert!(report.ast().recoveries().is_empty(), "{source}");
            let ast = report.into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn mixed_with_attributes_keep_terminal_punctuation_through_any_number_of() {
        let catalogs = fixture_catalogs()
            .with_catalog(CatalogKind::KeywordAbility, ["Haste"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::CardType, ["Creature"]);
        let source = "Create any number of 1/1 red Goblin creature tokens with haste and \"This token can't block.\"";

        let report = crate::parse_with_catalogs(source, &catalogs);
        assert!(
            report.diagnostics().is_empty(),
            "{source}: {:?}",
            report.diagnostics(),
        );
        assert!(report.ast().recoveries().is_empty(), "{source}");
        let ast = report.into_ast();
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn keyword_abilities_render_from_canonical_catalog_identity() {
        let catalogs = fixture_catalogs();
        let ast = OracleText {
            abilities: vec![checked_ability(AbilityKind::Keyword(
                crate::keyword_line::build_keyword_line(
                    crate::syntax::SeparatedNonEmpty::new(
                        KeywordAbility {
                            ability: keyword_atom(&catalogs, "flying"),
                            argument: KeywordArgument::Absent,
                        },
                        vec![crate::syntax::Separated::new(
                            KeywordListSeparator::Comma,
                            KeywordAbility {
                                ability: keyword_atom(&catalogs, "deathtouch"),
                                argument: KeywordArgument::Absent,
                            },
                        )],
                    ),
                    None,
                )
                .expect("well-formed keyword line must satisfy the declaration"),
            ))],
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
    fn perfect_auxiliary_inverse_precedes_its_temporal_nominal() {
        // Bronze Cudgels: the parser classifies `this turn` against the bare
        // past participle, then wraps that predicate in perfect `has`. The
        // generated inverse must peel the auxiliary in the same structural
        // order instead of reclassifying the temporal as a direct object.
        let source = "This ability has resolved this turn.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    #[test]
    fn elided_modal_relative_clause_renders_the_bare_modal_without_synthesized_do() {
        let source = "Exile each creature that can't.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = ast.abilities[0].kind() else {
            panic!("expected a paragraph ability");
        };
        let (subject, predicate) = transitive_clause(&paragraph.sentences[0].body);
        assert!(
            subject.is_none(),
            "expected an imperative transitive clause"
        );
        let PredicateObject::NounPhrase(object) = predicate.object() else {
            panic!("expected a nominal object");
        };
        let crate::syntax::NounPhraseKind::Nominal(object) = object.kind() else {
            panic!("expected a nominal object");
        };
        let [NominalComplement::Relative(relative)] = object.complements() else {
            panic!("expected one relative complement: {object:#?}");
        };
        let RelativeBody::SubjectGap(Predicate::Deontic(predicate)) = relative.body() else {
            panic!("expected a deontic subject gap: {:#?}", relative.body());
        };
        assert!(predicate.inner().is_none(), "{predicate:#?}");
        assert_eq!(source_free(&ast, "Test Card", false), source);
    }

    fn relative_fixture_with_base(base: &str, surface: &str) -> RelativeClause {
        let source = format!("{base} {surface}");
        let parsed = crate::parse_fragment(
            &source,
            &fixture_catalogs(),
            crate::FragmentKind::Nominal,
            "Test Card",
            false,
        )
        .into_fragment()
        .unwrap_or_else(|| panic!("relative fixture must parse for {surface:?}"));
        let crate::Fragment::Nominal(phrase) = parsed else {
            panic!("relative fixture must lower as a nominal fragment: {source:?}")
        };
        let NounPhraseKind::Nominal(nominal) = phrase.kind() else {
            panic!("relative fixture must lower as a nominal: {source:?}")
        };
        nominal
            .complements()
            .iter()
            .find_map(|complement| match complement {
                NominalComplement::Relative(relative) => Some(relative.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("relative fixture must retain a relative: {source:?}"))
    }

    fn relative_fixture(surface: &str) -> RelativeClause {
        relative_fixture_with_base("a card", surface)
    }

    #[test]
    fn generated_relative_renderer_linearizes_direct_ast_for_every_form_and_nested_relatives() {
        use crate::clause as clause_api;
        use crate::predicate as predicate_api;

        let object = relative_fixture("you cast");

        let contracted_object = relative_fixture("you've cast");

        let progressive = predicate_api::build_predicate_verb(
            VerbInstance {
                verb: Verb::Word(Vocab::Attack),
                slot: VerbSlot::PresentParticiple,
            },
            predicate_api::PredicateFrameChoice::Intransitive,
        )
        .and_then(predicate_api::finish_predicate)
        .unwrap();
        let contracted_subject = clause_api::build_relative_subject_contracted_auxiliary(
            AuxiliaryInstance {
                auxiliary: Auxiliary::Be,
                inflection: AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
                contracted_negation: crate::features::Contraction::Full,
            },
            progressive,
        )
        .unwrap();

        let nested_subject = relative_fixture("that attacks a creature you control");

        let distributive =
            relative_fixture_with_base("creature cards", "that each have a different mana value");

        let copular_noun = relative_fixture("that's a creature");

        let copular_adjective = relative_fixture("that's red");

        let copular_prepositional = relative_fixture("that's in exile");

        let renderer = Renderer::new("Test Card", false);
        for (value, expected) in [
            (object, "you cast"),
            (contracted_object, "you've cast"),
            (contracted_subject, "that's attacking"),
            (nested_subject, "that attacks a creature you control"),
            (distributive, "that each have a different mana value"),
            (copular_noun, "that's a creature"),
            (copular_adjective, "that's red"),
            (copular_prepositional, "that's in exile"),
        ] {
            assert_eq!(renderer.relative_clause(&value).unwrap(), expected);
        }
    }

    #[test]
    fn generated_relative_renderer_distinguishes_zero_marker_that_from_explicit_markers() {
        let demonstrative = relative_fixture("that opponent controls");
        assert_eq!(demonstrative.marker(), RelativeMarker::Zero);
        assert!(matches!(
            demonstrative.body(),
            RelativeBody::ObjectGap { subject, .. }
                if matches!(
                    subject.0.kind(),
                    NounPhraseKind::Nominal(nominal)
                        if matches!(
                            nominal.determiner().map(Determiner::kind),
                            Some(DeterminerKind::Demonstrative(Demonstrative::That))
                        )
                )
        ));

        let explicit_that = relative_fixture("that attacks");
        assert_eq!(explicit_that.marker(), RelativeMarker::That);

        let explicit_who = relative_fixture("who attacks");
        assert_eq!(explicit_who.marker(), RelativeMarker::Who);

        let renderer = Renderer::new("Test Card", false);
        assert_eq!(
            renderer.relative_clause(&demonstrative).unwrap(),
            "that opponent controls"
        );
        assert_eq!(
            renderer.relative_clause(&explicit_that).unwrap(),
            "that attacks"
        );
        assert_eq!(
            renderer.relative_clause(&explicit_who).unwrap(),
            "who attacks"
        );
    }

    #[test]
    fn generated_relative_renderer_owns_its_coordinated_adjective_tail() {
        let value = relative_fixture("that's red or green");
        assert_eq!(
            Renderer::new("Test Card", false)
                .relative_clause(&value)
                .unwrap(),
            "that's red or green",
        );
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

        let AbilityKind::Paragraph(paragraph) = ast.abilities[0].kind() else {
            panic!("expected a paragraph ability");
        };
        let (subject, predicate) = transitive_clause(&paragraph.sentences[0].body);
        assert!(subject.is_some(), "expected a finite transitive clause");
        assert_eq!(
            predicate.head().verb().verb,
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
        // Whole-AST inverse for possessive determiners: proves the generated
        // inverse is total for a premodified possessor, singular and plural.
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

        let AbilityKind::Paragraph(paragraph) = ast.abilities[0].kind() else {
            panic!("expected a paragraph ability");
        };
        let (subject, predicate) = transitive_clause(&paragraph.sentences[0].body);
        assert!(
            subject.is_none(),
            "expected an imperative transitive clause"
        );
        assert!(
            !predicate.pre_object_elements().is_empty(),
            "the adverb must be carried before the object: {predicate:#?}"
        );
        assert!(predicate.elements().is_empty(), "{predicate:#?}");

        for source in ["Draw only one card.", "Draw again one card."] {
            let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();
            assert_eq!(source_free(&ast, "Test Card", false), source);
        }
    }

    #[test]
    fn post_object_adverbs_render_after_the_object() {
        let source = "Draw one card only.";
        let ast = crate::parse_with_catalogs(source, &fixture_catalogs()).into_ast();

        let AbilityKind::Paragraph(paragraph) = ast.abilities[0].kind() else {
            panic!("expected a paragraph ability");
        };
        let (subject, predicate) = transitive_clause(&paragraph.sentences[0].body);
        assert!(
            subject.is_none(),
            "expected an imperative transitive clause"
        );
        assert!(predicate.pre_object_elements().is_empty(), "{predicate:#?}");
        assert!(
            !predicate.elements().is_empty(),
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
                    Some(crate::determiner::indefinite()),
                    vec![],
                    NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
                    vec![],
                ))],
            ),
        ));
        assert_eq!(source_free(&draw, "Test Card", false), "Draw a card.");

        let spells_cost = paragraph_ability(simple(
            Some(Subject(nominal(
                None,
                vec![],
                NounInstance::unchecked_plural(Noun::Word(Vocab::Spell)),
                vec![],
            ))),
            verb_phrase(
                Vocab::Cost,
                THIRD_PLURAL_PRESENT,
                vec![
                    VerbDependent::Scalar(Phrase::OracleSymbol(OracleSymbol::new("{1}").unwrap())),
                    VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(
                        crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Less))
                            .unwrap(),
                    ))),
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
                Some(crate::determiner::indefinite()),
                vec![],
                NounInstance::unchecked_singular(Noun::Word(Vocab::Hour)),
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
            Some(crate::determiner::possessive_pronoun(Pronoun::You).unwrap()),
            vec![],
            NounInstance::unchecked_plural(Noun::Word(Vocab::Opponent)),
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
                    NounInstance::unchecked_plural(Noun::Word(Vocab::Spell)),
                    vec![],
                ))],
            ),
        ));
        assert_eq!(
            source_free(&cannot_cast, "Test Card", false),
            "Your opponents can't cast spells."
        );
        assert_eq!(
            crate::determiner::possessive_pronoun(Pronoun::You)
                .unwrap()
                .render()
                .unwrap(),
            "your"
        );
    }

    #[test]
    fn noun_modifiers_relative_clauses_and_shared_subjects_keep_their_structure() {
        let catalogs = fixture_catalogs();
        let creatures = controlled_creature(&catalogs, false, true);
        let crate::syntax::NounPhraseKind::Nominal(creatures) = creatures.kind() else {
            panic!("controlled creature fixture must be nominal");
        };
        let first = simple(
            Some(Subject(nominal(
                None,
                vec![
                    NominalModifier::Adjective {
                        polarity: Polarity::Positive,
                        phrase: crate::adjective::build_adjective_phrase(Adjective::Word(
                            Vocab::Other,
                        ))
                        .unwrap(),
                    },
                    NominalModifier::Noun {
                        polarity: Polarity::Positive,
                        noun: catalog_noun(&catalogs, "Goblin", false),
                    },
                ],
                creatures.head().clone(),
                creatures.complements().to_vec(),
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
        let Clause::Independent(IndependentClause::Finite(first)) = first else {
            panic!("first coordinated fixture must be a finite clause");
        };
        let Some(subject) = first.subject().cloned() else {
            panic!("first coordinated fixture must have a subject");
        };
        let PredicateExpression::Simple(Predicate::Transitive(first)) = first.predicate().clone()
        else {
            panic!("first coordinated fixture must be transitive");
        };
        let ast = paragraph_ability(Clause::Independent(IndependentClause::Finite(
            FiniteClause::from_declaration_parts(
                Some(subject),
                PredicateExpression::Coordinated(crate::syntax::Coordination::new(
                    PredicateExpression::Simple(Predicate::Transitive(first)),
                    crate::syntax::CoordinationJunction::from_declaration_parts(
                        Some(Conjunction::And),
                        crate::features::Comma::Absent,
                    ),
                    PredicateExpression::Simple(second),
                )),
            ),
        )));

        assert_eq!(
            source_free(&ast, "Test Card", false),
            "Other Goblin creatures you control get +1/+1 and have haste."
        );
    }

    #[test]
    fn ditransitive_and_prepositional_sentences_render_structurally() {
        let target_player = nominal(
            Some(crate::determiner::target(None)),
            vec![],
            NounInstance::unchecked_singular(Noun::Word(Vocab::Player)),
            vec![],
        );
        let target_source = nominal(
            Some(crate::determiner::target(None)),
            vec![],
            NounInstance::unchecked_singular(Noun::Word(Vocab::Source)),
            vec![],
        );

        // Test "Ask target player a number."
        let ask = paragraph_ability(simple(
            None,
            verb_phrase_with_frame(
                Vocab::Ask,
                VerbSlot::Imperative,
                1,
                vec![
                    VerbDependent::IndirectObject(target_player.clone()),
                    VerbDependent::DirectObject(nominal(
                        Some(crate::determiner::indefinite()),
                        vec![],
                        NounInstance::unchecked_singular(Noun::Word(Vocab::Number)),
                        vec![],
                    )),
                ],
            ),
        ));
        assert_eq!(
            ask.render("Test Card", false)
                .expect("the generated ditransitive predicate renders"),
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
                        Some(crate::determiner::all()),
                        vec![],
                        NounInstance::unchecked_mass(Noun::Word(Vocab::Damage)),
                        vec![],
                    )),
                    VerbDependent::Prepositional(
                        crate::constructions::prepositional::expect_prepositional_phrase(
                            Preposition::From,
                            crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                                target_source,
                            )),
                        ),
                    ),
                ],
            ),
        ));
        assert_eq!(
            prevent
                .render("Test Card", false)
                .expect("the generated selected-preposition predicate renders"),
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
                            Some(crate::determiner::indefinite()),
                            vec![],
                            NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
                            vec![],
                        ))],
                    ),
                )),
            }],
        };
        let triggered = OracleText {
            abilities: vec![checked_ability(AbilityKind::Triggered(TriggeredAbility {
                conditions: TriggerConditionList {
                    first: TriggerCondition {
                        introducer: TriggerWord::Whenever,
                        event: TriggerEvent::Clause(independent(simple(
                            Some(Subject(NounPhrase::from_this_card_declaration(
                                ThisCardForm::AbbreviatedName,
                            ))),
                            verb_phrase(Vocab::Attack, THIRD_SINGULAR_PRESENT, vec![]),
                        ))),
                    },
                    rest: Vec::new(),
                },
                intervening_condition: None,
                effect: draw_effect,
            }))],
        };
        assert_eq!(
            source_free(&triggered, "Aang, A Lot to Learn", true),
            "Whenever Aang attacks, draw a card."
        );

        let cards = nominal(
            None,
            vec![],
            NounInstance::unchecked_plural(Noun::Word(Vocab::Card)),
            vec![NominalComplement::Prepositional(
                crate::constructions::prepositional::expect_prepositional_phrase(
                    Preposition::In,
                    crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(nominal(
                        Some(crate::determiner::possessive_pronoun(Pronoun::You).unwrap()),
                        vec![],
                        NounInstance::unchecked_singular(Noun::Word(Vocab::Hand)),
                        vec![],
                    ))),
                ),
            )],
        );
        let number = nominal(
            Some(crate::determiner::the()),
            vec![],
            NounInstance::unchecked_singular(Noun::Word(Vocab::Number)),
            vec![NominalComplement::Prepositional(
                crate::constructions::prepositional::expect_prepositional_phrase(
                    Preposition::Of,
                    crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(cards)),
                ),
            )],
        );
        let full_name = paragraph_ability(simple(
            Some(Subject(nominal(
                Some(crate::determiner::possessive_this_card(
                    ThisCardForm::FullName,
                )),
                vec![],
                NounInstance::unchecked_mass(Noun::Word(Vocab::Power)),
                vec![],
            ))),
            verb_phrase(
                Vocab::Be,
                THIRD_SINGULAR_PRESENT,
                vec![VerbDependent::PredicateComplement(Phrase::AdjectivePhrase(
                    Box::new(
                        AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Equal))
                            .and_then(|phrase| {
                                phrase.try_attach_declared_prepositional(
                                    crate::constructions::prepositional::expect_prepositional_phrase(
                                        Preposition::To,
                                        crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(number)),
                                    ),
                                )
                            })
                            .unwrap(),
                    ),
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
        // so a recovered span round-trips byte-for-byte with no self-reference
        // expansion.
        let source = "Aang frobnitzes Aang, A Lot to Learn.";
        let ast =
            crate::parse_with_identity(source, &fixture_catalogs(), "Aang, A Lot to Learn", true)
                .into_ast();

        assert_eq!(source_free(&ast, "Aang, A Lot to Learn", true), source);
    }

    #[test]
    fn triggered_modal_headers_render_in_their_enclosing_sentence_context() {
        let ast = OracleText {
            abilities: vec![checked_ability(AbilityKind::Modal(ModalAbility {
                frame: ModalFrame::Triggered(TriggerHeader {
                    introducer: TriggerWord::Whenever,
                    event: TriggerEvent::Clause(independent(simple(
                        Some(Subject(NounPhrase::from_this_card_declaration(
                            ThisCardForm::AbbreviatedName,
                        ))),
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
                modes: vec![Mode {
                    heading: None,
                    body: Paragraph {
                        flavor_header: None,
                        sentences: vec![Sentence {
                            body: SentenceBody::Recovered(RecoveredText::new("Draw a card.", 4)),
                        }],
                    },
                }],
            }))],
        };

        assert_eq!(
            source_free(&ast, "Aang, A Lot to Learn", true),
            "Whenever Aang attacks, choose one —\n• Draw a card."
        );
    }

    #[test]
    fn flavor_header_renders_verbatim_with_its_em_dash_separator() {
        let ast = OracleText {
            abilities: vec![checked_ability(AbilityKind::Paragraph(Paragraph {
                flavor_header: Some(FlavorHeader::new("Throw ...", 2)),
                sentences: vec![Sentence {
                    body: SentenceBody::Recovered(RecoveredText::new("Draw a card.", 4)),
                }],
            }))],
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
            abilities: vec![checked_ability(AbilityKind::RollRow(RollRowAbility {
                range,
                body: Paragraph {
                    flavor_header: None,
                    sentences: vec![Sentence {
                        body: SentenceBody::Recovered(RecoveredText::new("Draw a card.", 4)),
                    }],
                },
            }))],
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
            abilities: vec![checked_ability(AbilityKind::LevelBand(LevelBandAbility {
                range: LevelRange::Band {
                    low: arabic(6),
                    high: arabic(11),
                },
                stats: stat(6),
                abilities: vec![],
            }))],
        };
        assert_eq!(
            source_free(&band_only, "Test Card", false),
            "LEVEL 6-11\n6/6"
        );

        // At-least range with one contained keyword ability.
        let band_with_keyword = OracleText {
            abilities: vec![checked_ability(AbilityKind::LevelBand(LevelBandAbility {
                range: LevelRange::AtLeast(arabic(12)),
                stats: stat(9),
                abilities: vec![checked_ability(AbilityKind::Keyword(
                    crate::keyword_line::build_keyword_line(
                        crate::syntax::SeparatedNonEmpty::new(
                            KeywordAbility {
                                ability: keyword_atom(&catalogs, "flying"),
                                argument: KeywordArgument::Absent,
                            },
                            Vec::new(),
                        ),
                        None,
                    )
                    .expect("well-formed keyword line must satisfy the declaration"),
                ))],
            }))],
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
                Some(crate::determiner::indefinite()),
                vec![],
                NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
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
        let ast = paragraph_ability(Clause::Independent(IndependentClause::Finite(
            FiniteClause::from_declaration_parts(
                None,
                PredicateExpression::Coordinated(Coordination::new(
                    PredicateExpression::Simple(first),
                    CoordinationJunction::from_declaration_parts(
                        Some(Conjunction::Plus),
                        crate::features::Comma::Absent,
                    ),
                    PredicateExpression::Simple(second),
                )),
            ),
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

    #[test]
    fn predicated_keyword_rendering_rejects_shapes_outside_the_declared_family() {
        // Mutation caught: bypass the declaration linearizer for the permissive
        // generic renderer, which accepts arbitrary prepositions instead of requiring
        // one of the two declared keyword-argument families.
        let argument = KeywordArgument::Predicated(PredicatedArgument {
            qualities: vec![PredicatedQuality {
                preposition: Some(Preposition::To),
                quality: Phrase::ColorWord(crate::word::ColorWord::Red),
            }],
        });

        assert_eq!(
            Renderer::new("Test Card", false).keyword_argument(&argument),
            Err(RenderError::InvalidNominalConstruction)
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
                CatalogValue::Word(WordMatch::Noun(noun))
                    if matches!(noun.kind(), crate::word::NounInstanceKind::Plural(_))
                        && plural =>
                {
                    Some(noun)
                }
                CatalogValue::Word(WordMatch::Noun(noun))
                    if matches!(noun.kind(), crate::word::NounInstanceKind::Singular(_))
                        && !plural =>
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
        NounPhrase::from_nominal_declaration(NominalPhrase::test_from_projection_parts(
            determiner,
            modifiers,
            head,
            complements,
        ))
    }

    fn verb_phrase(vocab: Vocab, slot: VerbSlot, dependents: Vec<VerbDependent>) -> VerbPhrase {
        verb_phrase_with_frame(vocab, slot, 0, dependents)
    }

    fn verb_phrase_with_frame(
        vocab: Vocab,
        slot: VerbSlot,
        frame_index: usize,
        dependents: Vec<VerbDependent>,
    ) -> VerbPhrase {
        VerbPhrase {
            auxiliaries: vec![],
            preverb_modifiers: vec![],
            verb: VerbInstance {
                verb: Verb::Word(vocab),
                slot,
            },
            frame: vocab.predicate_frames()[frame_index],
            dependents,
        }
    }

    fn simple(subject: Option<Subject>, predicate: VerbPhrase) -> Clause {
        let predicate = strict_predicate(predicate);
        let independent = IndependentClause::Finite(FiniteClause::from_declaration_parts(
            subject,
            PredicateExpression::Simple(predicate),
        ));
        Clause::Independent(independent)
    }

    fn transitive_clause(body: &SentenceBody) -> (Option<&Subject>, &TransitivePredicate) {
        let SentenceBody::Independent(IndependentClause::Finite(finite)) = body else {
            panic!("expected a finite clause: {body:#?}");
        };
        let PredicateExpression::Simple(Predicate::Transitive(predicate)) = finite.predicate()
        else {
            panic!("expected a simple transitive predicate: {finite:#?}");
        };
        (finite.subject(), predicate)
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
            abilities: vec![checked_ability(AbilityKind::Paragraph(Paragraph {
                flavor_header: None,
                sentences: vec![Sentence {
                    body: match clause {
                        Clause::Independent(clause) => SentenceBody::Independent(clause),
                        Clause::Dependent(_) => panic!("sentence fixture must be independent"),
                    },
                }],
            }))],
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
        let predicate = crate::constructions::predicate::inverse_public_predicate(
            &Predicate::Intransitive(relative_predicate),
        )
        .and_then(crate::constructions::relative::project_object_gap_predicate_hole)
        .expect("the fixture predicate preserves one checked object gap");
        let relative = crate::constructions::relative::checked_build_relative_object(
            NounPhrase::from_pronoun_declaration(Pronoun::You, PronounCase::Subject),
            predicate,
        )
        .expect("the fixture subject agrees with its object-gap predicate");
        nominal(
            (!plural).then_some(crate::determiner::target(None)),
            vec![],
            catalog_noun(catalogs, surface, plural),
            vec![NominalComplement::Relative(relative)],
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
        let arabic_phrase = crate::adjective::build_adjective_phrase_degree_measure(
            degcmp_arabic(2),
            Adjective::Word(Vocab::Greater),
        )
        .unwrap();
        assert_eq!(
            renderer.adjective_phrase(&arabic_phrase).unwrap(),
            "2 greater"
        );
        let cardinal_phrase = crate::adjective::build_adjective_phrase_degree_measure(
            cardinal(2),
            Adjective::Word(Vocab::Greater),
        )
        .unwrap();
        assert_eq!(
            renderer.adjective_phrase(&cardinal_phrase).unwrap(),
            "two greater"
        );
    }

    #[test]
    fn adjective_renderer_handles_declared_prepositional_and_infinitival_complements() {
        let renderer = Renderer::new("Test Card", false);
        let prepositional = crate::adjective::build_adjective_phrase_prepositional(
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Equal)).unwrap(),
            crate::constructions::prepositional::expect_prepositional_phrase(
                Preposition::To,
                crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                    NounPhrase::from_this_card_declaration(ThisCardForm::FullName),
                )),
            ),
        )
        .unwrap();
        assert_eq!(
            renderer.adjective_phrase(&prepositional).unwrap(),
            "equal to Test Card"
        );

        let infinitive = crate::adjective::build_adjective_phrase_infinitive(
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Able)).unwrap(),
            crate::clause::build_infinitive_to(&strict_predicate(verb_phrase(
                Vocab::Attack,
                VerbSlot::Infinitive,
                vec![],
            )))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            renderer.adjective_phrase(&infinitive).unwrap(),
            "able to attack"
        );
    }

    #[test]
    fn nominal_modifier_adjective_renders_a_checked_postnominal_comparison() {
        let renderer = Renderer::new("Test Card", false);
        let target =
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Target)).unwrap();
        let standard =
            crate::adjective::build_comparison_standard(None, Some(target), None).unwrap();
        let comparison = crate::adjective::build_comparison_than(standard).unwrap();
        let phrase = AdjectivePhrase::try_from_lexical_head(Adjective::Word(Vocab::Greater))
            .and_then(|phrase| phrase.try_attach_postnominal_comparison(comparison))
            .unwrap();
        let (immediate, trailing) = renderer.nominal_modifier_adjective(&phrase).unwrap();
        assert_eq!(immediate, "greater");
        assert_eq!(trailing, vec!["than target".to_string()]);
    }

    fn power_toughness_nominal(power: ScalarValue) -> NominalPhrase {
        NominalPhrase::test_from_projection_parts(
            Some(crate::determiner::indefinite()),
            vec![NominalModifier::PowerToughness(PowerToughness {
                power: SignedScalar {
                    sign: ScalarSign::None,
                    value: power,
                },
                toughness: SignedScalar {
                    sign: ScalarSign::None,
                    value: power,
                },
            })],
            NounInstance::unchecked_singular(Noun::Word(Vocab::Token)),
            vec![],
        )
    }

    #[test]
    fn indefinite_article_derives_from_the_power_toughness_onset() {
        // `crate::determiner::indefinite()` carries no word (see its doc comment): the
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
            frame: phrase.frame,
            verb: phrase.verb,
            distributive_each: false,
        };
        let mut object = None;
        let mut pre_object_elements = Vec::new();
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
                    object = Some(PredicateObject::Quantity(Quantity::unchecked_exact(number)));
                }
                VerbDependent::Infinitive(marker, predicate) => {
                    let predicate = strict_predicate(*predicate);
                    let infinitive = match marker {
                        InfinitiveMarker::Bare => InfinitiveClause::declaration_bare(predicate),
                        InfinitiveMarker::To => {
                            crate::clause::build_infinitive_to(&predicate).unwrap()
                        }
                    };
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::Infinitive(infinitive),
                    ));
                }
                VerbDependent::IndirectObject(noun_phrase) => {
                    let target =
                        if object.is_none() { &mut pre_object_elements } else { &mut elements };
                    target.push(PredicateElement::Complement(
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
                    pre_object_elements,
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

    fn generated_add_object(object: PredicateObject) -> Predicate {
        Predicate::Transitive(crate::syntax::HeadedPredicate {
            head: PredicateHead {
                auxiliaries: vec![],
                first_auxiliary_contracted_with_subject: crate::features::Contraction::Full,
                preverb_modifiers: vec![],
                verb: VerbInstance {
                    verb: Verb::Word(Vocab::Add),
                    slot: VerbSlot::Imperative,
                },
                frame: Verb::Word(Vocab::Add).predicate_frames()[0],
                distributive_each: false,
            },
            kind: crate::syntax::Transitive {
                pre_object_elements: vec![],
                object,
            },
            elements: vec![],
        })
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
            renderer.predicate(&generated_add_object(object)).unwrap(),
            "add {W}, {B}, or {G}"
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
            renderer.predicate(&generated_add_object(object)).unwrap(),
            "add {W}, {U}, {B}, {R}, and {G}"
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
        assert_eq!(
            renderer.predicate(&generated_add_object(object)).unwrap(),
            "add {R} or {G}"
        );
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
        assert_eq!(
            renderer.predicate(&generated_add_object(object)).unwrap(),
            "add {U} or {C}{U}"
        );
    }
}
