//! The set-valued exact-parse law harness: budgeted enumeration of packed
//! alternatives into deduplicated `ExactParse` sets, with `linearize` as the
//! generated render entry. Strict exact mode throughout — `OpacityMode::Exact`,
//! no opaque-noun retry — so a law can never pass through an escape hatch.
//! Chart-level admission only: form guards are not consulted (Milestone-3
//! status quo), and dominance-losing derivations stay in the set (dominance
//! is selection preference, not admission).

use deckmaste_construction_compiler::runtime::AtomData;

use super::Catalogs;
use super::EnglishGrammar;
use super::EnglishSurfaceWitness;
use super::GrammarError;
use super::MeaningKey;
use super::Nonterminal;
use super::OpacityMode;
use super::generated::GeneratedActivation;
use super::lowering::lower;
use super::lowering::selected_rule_children;
use super::parse_chart;
use super::parse_support::EnglishForest;
use super::rules::RegistrationOrder;
use super::rules::RuleImpl;
use crate::features::ExactParse;
use crate::forest::AlternativeSelection;
use crate::forest::ChoiceMap;
use crate::forest::NodeId;
use crate::forest::SelectionEnumerationError;
use crate::identity::SelfReference;
use crate::surface::collapse_full_names;
use crate::surface::lex;

/// One enumerated generated derivation: the construction, the DECLARED form
/// ordinal it parsed through (the form-choice witness — comma presence and
/// its like are facts of which form matched), and parts mirroring that
/// form's surface atoms in declaration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GeneratedParse {
    pub(crate) construction: &'static str,
    pub(crate) ordinal: u16,
    pub(crate) parts: Vec<GeneratedPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GeneratedPart {
    Literal(&'static str),
    Scalar {
        field: &'static str,
        value: GeneratedScalar,
    },
    Subtree {
        field: &'static str,
        parse: GeneratedParse,
    },
}

/// Captured scalar-lexeme values, mirroring `generated::codec_slot`'s closed
/// codec table exactly — a codec admitted there must be renderable here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedScalar {
    Conjunction(crate::features::Conjunction),
    Comma(crate::features::Comma),
}

fn generated_parse<S: AlternativeSelection>(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    selection: &S,
) -> Option<GeneratedParse> {
    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(selection.alternative(node)?)?;
    let rule = alternative.rule?;
    let RuleImpl::Generated(generated) = grammar.impls.get(rule.index()).copied()? else {
        return None;
    };
    let construction = generated.group.constructions.get(generated.construction)?;
    let form = construction.forms.get(generated.form)?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut children = Vec::new();
    selected_rule_children(forest, *intermediate, selection, &mut children)?;
    if children.len() != form.atoms.len() {
        return None;
    }
    let mut parts = Vec::with_capacity(form.atoms.len());
    for (&atom, &child) in form.atoms.iter().zip(&children) {
        parts.push(match atom {
            AtomData::Literal(literal) => GeneratedPart::Literal(literal),
            AtomData::Lexeme(field) => GeneratedPart::Scalar {
                field,
                value: scalar_value(forest, child)?,
            },
            AtomData::Hole(field) => GeneratedPart::Subtree {
                field,
                parse: generated_parse(grammar, forest, child, selection)?,
            },
        });
    }
    Some(GeneratedParse {
        construction: construction.id,
        ordinal: form.ordinal,
        parts,
    })
}

fn scalar_value(forest: &EnglishForest, node: NodeId) -> Option<GeneratedScalar> {
    match forest.node(node).key.lexical_value()? {
        MeaningKey::Conjunction(conjunction) => Some(GeneratedScalar::Conjunction(*conjunction)),
        MeaningKey::Punctuation(crate::surface::Punctuation::Comma) => {
            Some(GeneratedScalar::Comma(crate::features::Comma::Present))
        }
        MeaningKey::Punctuation(_) => None,
        _ => None,
    }
}

/// The generated render entry: parts back to bytes. Tokens join with a
/// single space, except nothing precedes a comma — the fixture surface
/// convention (`"and, or"`), and the rule the byte-equality laws hold
/// linearization to.
pub(crate) fn linearize(parse: &GeneratedParse) -> String {
    let mut tokens = Vec::new();
    collect_tokens(parse, &mut tokens);
    let mut rendered = String::new();
    for token in tokens {
        if !(rendered.is_empty() || token == ",") {
            rendered.push(' ');
        }
        rendered.push_str(token);
    }
    rendered
}

fn collect_tokens(parse: &GeneratedParse, tokens: &mut Vec<&'static str>) {
    for part in &parse.parts {
        match part {
            GeneratedPart::Literal(literal) => tokens.push(literal),
            GeneratedPart::Scalar {
                value: GeneratedScalar::Conjunction(conjunction),
                ..
            } => tokens.push(conjunction.spelling()),
            GeneratedPart::Scalar {
                value: GeneratedScalar::Comma(comma),
                ..
            } => {
                if comma.is_present() {
                    tokens.push(",");
                }
            }
            GeneratedPart::Subtree { parse, .. } => collect_tokens(parse, tokens),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoordinationVerdict {
    Admitted,
    Refused { requirement: &'static str },
}

#[derive(Default)]
struct CoordinationLinearizer {
    rendered: String,
    pending_determiner: Option<crate::syntax::Determiner>,
    skip_payload_subtrees: usize,
}

impl CoordinationLinearizer {
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

impl deckmaste_construction_compiler::runtime::LinearizationVisitor for CoordinationLinearizer {
    type Error = crate::renderer::RenderError;

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
                    .downcast_ref::<crate::syntax::Determiner>()
                    .expect("the declaration's Determiner hole preserves its Rust type");
                self.pending_determiner = Some(determiner.clone());
            }
            "NounPhrase" => {
                let noun = value
                    .downcast_ref::<crate::syntax::NounPhrase>()
                    .expect("the declaration's NounPhrase hole preserves its Rust type");
                let rendered = crate::render_fragment(
                    &crate::fragment::Fragment::Nominal(noun.clone()),
                    "",
                    false,
                )?;
                self.push(&rendered);
            }
            "NominalPhrase" => {
                let nominal = value
                    .downcast_ref::<crate::syntax::NominalPhrase>()
                    .expect("the declaration's NominalPhrase hole preserves its Rust type");
                let mut nominal = nominal.clone();
                if let Some(determiner) = self.pending_determiner.take() {
                    nominal.determiner = Some(determiner);
                }
                let rendered = crate::render_fragment(
                    &crate::fragment::Fragment::Nominal(crate::syntax::NounPhrase::Nominal(
                        nominal,
                    )),
                    "",
                    false,
                )?;
                self.push(&rendered);
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
            "Comma" => {
                let comma = value
                    .downcast_ref::<crate::features::Comma>()
                    .expect("the declaration's Comma scalar preserves its Rust type");
                if comma.is_present() {
                    self.push(",");
                }
            }
            "Conjunction" => {
                let conjunction = value
                    .downcast_ref::<crate::features::Conjunction>()
                    .expect("the declaration's Conjunction scalar preserves its Rust type");
                self.push(conjunction.spelling());
            }
            other => panic!("unexpected coordination scalar codec `{other}`"),
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
                .downcast_ref::<crate::syntax::NominalComplement>()
                .expect("the bound NominalComplement keeps its Rust enum type");
            self.push(&crate::renderer::render_nominal_complement(complement)?);
            self.skip_payload_subtrees += 1;
        }
        Ok(())
    }
}

fn finish_coordination_linearization(
    result: Result<
        (),
        deckmaste_construction_compiler::runtime::LinearizationError<crate::renderer::RenderError>,
    >,
    visitor: CoordinationLinearizer,
) -> Result<String, crate::renderer::RenderError> {
    match result {
        Ok(()) => Ok(visitor.finish()),
        Err(deckmaste_construction_compiler::runtime::LinearizationError::Visitor(error)) => {
            Err(error)
        }
        Err(error) => {
            panic!("validated coordination must select exactly one declared form: {error:?}")
        }
    }
}

pub(crate) fn linearize_coordinated_noun_phrase(
    value: &crate::syntax::CoordinatedNounPhrase,
) -> Result<String, crate::renderer::RenderError> {
    assert_eq!(
        coordination_verdict(value),
        CoordinationVerdict::Admitted,
        "only declaration-admitted noun-phrase coordinations can be linearized",
    );

    let mut visitor = CoordinationLinearizer::default();
    let value = crate::constructions::coordination::DerivedCoordinatedNounPhrase::try_new(
        value.first.clone(),
        value.rest.clone(),
    )
    .expect("the verdict admitted this value");
    let result = crate::constructions::coordination::linearize_noun_phrase_coordination_with(
        &value,
        &mut visitor,
    );
    finish_coordination_linearization(result, visitor)
}

pub(crate) fn linearize_coordinated_nominal_phrase(
    value: &crate::syntax::CoordinatedNominalPhrase,
) -> Result<String, crate::renderer::RenderError> {
    assert_eq!(
        nominal_coordination_verdict(value),
        CoordinationVerdict::Admitted,
        "only declaration-admitted shared-determiner coordinations can be linearized",
    );

    let mut visitor = CoordinationLinearizer::default();
    let value = crate::constructions::coordination::DerivedCoordinatedNominalPhrase::try_new(
        value.determiner.clone(),
        value.first.clone(),
        value.rest.clone(),
        value.complements.clone(),
    )
    .expect("the verdict admitted this value");
    let result = crate::constructions::coordination::linearize_shared_determiner_nominal_with(
        &value,
        &mut visitor,
    );
    finish_coordination_linearization(result, visitor)
}

pub(crate) fn coordination_verdict(
    value: &crate::syntax::CoordinatedNounPhrase,
) -> CoordinationVerdict {
    let (first, rest) = crate::constructions::coordination::parts_noun_phrase_coordination(value);
    match crate::constructions::coordination::build_noun_phrase_coordination(
        first.clone(),
        rest.clone(),
    ) {
        Ok(_) => CoordinationVerdict::Admitted,
        Err(violation) => CoordinationVerdict::Refused {
            requirement: violation.requirement,
        },
    }
}

pub(crate) fn nominal_coordination_verdict(
    value: &crate::syntax::CoordinatedNominalPhrase,
) -> CoordinationVerdict {
    let (determiner, first, rest, complements) =
        crate::constructions::coordination::parts_shared_determiner_nominal(value);
    match crate::constructions::coordination::build_shared_determiner_nominal(
        determiner.clone(),
        first.clone(),
        rest.clone(),
        complements.clone(),
    ) {
        Ok(_) => CoordinationVerdict::Admitted,
        Err(violation) => CoordinationVerdict::Refused {
            requirement: violation.requirement,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactParseError {
    Grammar(GrammarError),
    /// The forest reaches a cycle; enumeration refuses rather than risking
    /// unbounded derivations (see `SelectionEnumerationError::Cycle`).
    Cycle,
    /// More complete selections exist than the caller's budget admits. A
    /// hard, loud ceiling — never silent truncation.
    TooManyAlternatives {
        budget: usize,
    },
}

/// Strict-exact, set-valued parse: every per-node-consistent derivation of
/// `source` as `nonterminal`, extracted as [`GeneratedParse`] and paired
/// with the root alternative's surface payload, deduplicated. An empty set
/// means no admitted root yielded an extractable generated derivation; with
/// this milestone's generated-categories-only usage that coincides with
/// nothing being admitted. Dominance-losing derivations are included
/// (dominance is selection preference, not admission), and no opaque-noun
/// retry ever runs.
pub(crate) fn parse_as(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    activation: GeneratedActivation,
    budget: usize,
) -> Result<Vec<ExactParse<GeneratedParse, EnglishSurfaceWitness>>, ExactParseError> {
    parse_as_with_registration_order(
        source,
        catalogs,
        nonterminal,
        activation,
        budget,
        RegistrationOrder::Normal,
    )
}

fn parse_as_with_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    activation: GeneratedActivation,
    budget: usize,
    order: RegistrationOrder,
) -> Result<Vec<ExactParse<GeneratedParse, EnglishSurfaceWitness>>, ExactParseError> {
    let self_reference = SelfReference::default();
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let grammar = EnglishGrammar::with_opacity_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        OpacityMode::Exact,
        self_reference,
        order,
        activation,
    );
    let chart = parse_chart(&grammar, &tokens).map_err(ExactParseError::Grammar)?;
    let mut remaining = budget;
    let mut results = Vec::new();
    for &root in &chart.roots {
        let selections = chart
            .forest
            .enumerate_selections(root, &mut remaining)
            .map_err(|error| match error {
                SelectionEnumerationError::Cycle(_) => ExactParseError::Cycle,
                SelectionEnumerationError::BudgetExhausted => {
                    ExactParseError::TooManyAlternatives { budget }
                }
            })?;
        for selection in selections {
            let Some(ast) = generated_parse(&grammar, &chart.forest, root, &selection) else {
                continue;
            };
            let Some(alternative) = selection.alternative(root) else {
                continue;
            };
            let Some(exact) = chart.forest.exact_result(root, alternative, ast) else {
                continue;
            };
            if !results.contains(&exact) {
                results.push(exact);
            }
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::WitnessClassData;
    use proptest::prelude::*;

    use super::super::generated::GeneratedActivation;
    use super::*;
    use crate::catalog::CatalogKind;
    use crate::constructions::coordination;
    use crate::constructions::law;
    use crate::constructions::probe;
    use crate::features::Comma;
    use crate::features::Conjunction;
    use crate::grammar::lowering::Lowered;
    use crate::syntax::Determiner;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalPhrase;
    use crate::syntax::NominalPhraseCoordination;
    use crate::syntax::NounPhrase;
    use crate::syntax::NounPhraseCoordination;
    use crate::syntax::Quantity;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Vocab;

    fn probe_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(probe::GROUPS);
        Nonterminal::Generated(cats[name])
    }

    fn law_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(law::GROUPS);
        Nonterminal::Generated(cats[name])
    }

    /// Order-insensitive set equality — permutations may reorder discovery.
    fn same_exact_set(
        left: &[ExactParse<GeneratedParse, EnglishSurfaceWitness>],
        right: &[ExactParse<GeneratedParse, EnglishSurfaceWitness>],
    ) -> bool {
        left.len() == right.len() && left.iter().all(|member| right.contains(member))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Flying", "Lifelink", "Protection"],
            )
            .with_catalog(
                CatalogKind::CardType,
                [
                    "Artifact",
                    "Battle",
                    "Creature",
                    "Instant",
                    "Land",
                    "Planeswalker",
                    "Sorcery",
                ],
            )
            .with_catalog(
                CatalogKind::CreatureType,
                [
                    "Avatar", "Citizen", "Halfling", "Kithkin", "Scout", "Soldier",
                ],
            )
    }

    fn nominal(head: Vocab) -> NominalPhrase {
        NominalPhrase {
            determiner: None,
            modifiers: Vec::new(),
            head: NounInstance::Singular(Noun::Word(head)),
            complements: Vec::new(),
        }
    }

    fn noun_phrase(head: Vocab) -> NounPhrase {
        NounPhrase::Nominal(nominal(head))
    }

    const FIXTURE_READING_BUDGET: usize = 50_000;

    struct NounPhraseEnumeration {
        readings: Vec<NounPhrase>,
        selections: usize,
        noun_phrase_lowerings: usize,
    }

    fn fixture_enumeration<T: std::fmt::Debug>(
        source: &str,
        catalogs: &Catalogs,
        nonterminal: Nonterminal,
        self_reference: SelfReference,
        mut extract: impl FnMut(Lowered) -> Option<T>,
    ) -> (Vec<T>, usize, usize) {
        let surface = lex(source);
        let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
        let grammar = EnglishGrammar::with_opacity_mode(
            source,
            catalogs,
            nonterminal,
            OpacityMode::Exact,
            self_reference,
        );
        let chart = parse_chart(&grammar, &tokens)
            .unwrap_or_else(|error| panic!("chart failed for {source:?}: {error:?}"));
        let mut remaining = FIXTURE_READING_BUDGET;
        let mut readings = Vec::new();
        let mut debug_keys = Vec::new();
        let mut selections = 0;
        let mut lowerings = 0;
        for &root in &chart.roots {
            let root_selections = chart
                .forest
                .enumerate_selections(root, &mut remaining)
                .unwrap_or_else(|error| {
                    panic!(
                        "fixture enumeration failed for {source:?} with budget \
                         {FIXTURE_READING_BUDGET}: {error:?}",
                    )
                });
            selections += root_selections.len();
            for selection in root_selections {
                let Some(reading) =
                    lower(&grammar, &chart.forest, root, &selection).and_then(&mut extract)
                else {
                    continue;
                };
                lowerings += 1;
                let key = format!("{reading:?}");
                if !debug_keys.contains(&key) {
                    debug_keys.push(key);
                    readings.push(reading);
                }
            }
        }
        (readings, selections, lowerings)
    }

    fn noun_phrase_enumeration(source: &str) -> NounPhraseEnumeration {
        let (readings, selections, noun_phrase_lowerings) = fixture_enumeration(
            source,
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
            SelfReference::default(),
            |lowered| match lowered {
                Lowered::NounPhrase(reading) => Some(reading),
                _ => None,
            },
        );
        NounPhraseEnumeration {
            readings,
            selections,
            noun_phrase_lowerings,
        }
    }

    fn sentence_readings(
        source: &str,
        catalogs: &Catalogs,
        self_reference: SelfReference,
    ) -> Vec<crate::syntax::Sentence> {
        fixture_enumeration(
            source,
            catalogs,
            Nonterminal::Sentence,
            self_reference,
            |lowered| match lowered {
                Lowered::Sentence(reading) => Some(reading),
                _ => None,
            },
        )
        .0
    }

    fn parse_fixture_noun_phrase(source: &str) -> NounPhrase {
        let catalogs = fixture_catalogs();
        let self_reference = SelfReference::default();
        let surface = lex(source);
        let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
        let grammar = EnglishGrammar::with_opacity_mode(
            source,
            &catalogs,
            Nonterminal::NounPhrase,
            OpacityMode::Exact,
            self_reference,
        );
        let chart = parse_chart(&grammar, &tokens)
            .unwrap_or_else(|error| panic!("chart failed for {source:?}: {error:?}"));
        let mut parsed = None;
        chart
            .forest
            .best_root_matching(
                chart.roots.iter().copied(),
                super::super::construction::registry(),
                |root, best| {
                    let Some(Lowered::NounPhrase(noun_phrase)) =
                        lower(&grammar, &chart.forest, root, best)
                    else {
                        return false;
                    };
                    parsed = Some(noun_phrase);
                    true
                },
            )
            .unwrap_or_else(|error| panic!("best-root search failed for {source:?}: {error:?}"))
            .unwrap_or_else(|| panic!("no exact noun-phrase root for {source:?}"));
        parsed.unwrap_or_else(|| panic!("best exact root did not lower for {source:?}"))
    }

    fn noun_phrase_readings(source: &str) -> Vec<NounPhrase> {
        noun_phrase_enumeration(source).readings
    }

    fn nominal_head_spelling(nominal: &NominalPhrase) -> &str {
        let noun = match &nominal.head {
            NounInstance::Singular(noun)
            | NounInstance::Plural(noun)
            | NounInstance::Mass(noun) => noun,
        };
        match noun {
            Noun::Word(vocab) => vocab.spelling(),
            Noun::Catalog(atom) => atom.canonical(),
            other => panic!("fixture nominal has a non-lexical head: {other:#?}"),
        }
    }

    fn collect_noun_coordinations<'syntax>(
        noun_phrase: &'syntax NounPhrase,
        found: &mut Vec<&'syntax crate::syntax::CoordinatedNounPhrase>,
    ) {
        match noun_phrase {
            NounPhrase::Nominal(nominal) => collect_nominal_coordinations(nominal, found),
            NounPhrase::CoordinatedNominal(coordination) => {
                collect_nominal_coordinations(&coordination.first, found);
                for member in &coordination.rest {
                    collect_nominal_coordinations(&member.phrase, found);
                }
                for complement in &coordination.complements {
                    collect_complement_coordinations(complement, found);
                }
            }
            NounPhrase::Coordinated(coordination) => {
                found.push(coordination);
                collect_noun_coordinations(&coordination.first, found);
                for member in &coordination.rest {
                    collect_noun_coordinations(&member.phrase, found);
                }
            }
            NounPhrase::Partitive(partitive) => {
                collect_noun_coordinations(&partitive.whole, found);
            }
            NounPhrase::SetException(exception) => {
                collect_noun_coordinations(&exception.included, found);
                collect_noun_coordinations(&exception.excluded, found);
            }
            NounPhrase::Arithmetic(crate::syntax::ArithmeticValue::Minus { left, right }) => {
                collect_noun_coordinations(left, found);
                collect_noun_coordinations(right, found);
            }
            NounPhrase::Arithmetic(crate::syntax::ArithmeticValue::Half { value, .. }) => {
                collect_noun_coordinations(value, found);
            }
            NounPhrase::Pronoun { .. }
            | NounPhrase::Possessive(_)
            | NounPhrase::Demonstrative(_)
            | NounPhrase::Quantity(_)
            | NounPhrase::ThisCard(_) => {}
        }
    }

    fn collect_nominal_coordinations<'syntax>(
        nominal: &'syntax NominalPhrase,
        found: &mut Vec<&'syntax crate::syntax::CoordinatedNounPhrase>,
    ) {
        for complement in &nominal.complements {
            collect_complement_coordinations(complement, found);
        }
    }

    fn collect_complement_coordinations<'syntax>(
        complement: &'syntax NominalComplement,
        found: &mut Vec<&'syntax crate::syntax::CoordinatedNounPhrase>,
    ) {
        let NominalComplement::Prepositional(preposition) = complement else {
            return;
        };
        for member in preposition.members() {
            if let crate::syntax::Phrase::NounPhrase(noun_phrase) = member.object.as_ref() {
                collect_noun_coordinations(noun_phrase, found);
            }
        }
    }

    fn main_transitive_object(sentence: &crate::syntax::Sentence) -> Option<&NounPhrase> {
        let crate::syntax::SentenceBody::Independent(clause) = &sentence.body else {
            return None;
        };
        main_clause_transitive_object(clause)
    }

    fn main_clause_transitive_object(
        clause: &crate::syntax::IndependentClause,
    ) -> Option<&NounPhrase> {
        let predicate = match clause {
            crate::syntax::IndependentClause::Transitive(_, predicate)
            | crate::syntax::IndependentClause::Imperative(crate::syntax::Predicate::Transitive(
                predicate,
            )) => predicate,
            crate::syntax::IndependentClause::Complex(complex) => {
                return main_clause_transitive_object(&complex.matrix);
            }
            _ => return None,
        };
        let crate::syntax::PredicateObject::NounPhrase(noun_phrase) = &predicate.object else {
            return None;
        };
        Some(noun_phrase)
    }

    fn has_base_modifier(nominal: &NominalPhrase) -> bool {
        nominal.modifiers.iter().any(|modifier| {
            matches!(
                modifier,
                crate::syntax::NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Word(vocab)),
                    ..
                } if vocab.spelling() == "base"
            )
        })
    }

    fn is_local_power_toughness_group(
        coordination: &crate::syntax::CoordinatedNounPhrase,
        requires_base: bool,
        requires_value: bool,
    ) -> bool {
        let NounPhrase::Nominal(power) = coordination.first.as_ref() else {
            return false;
        };
        let [member] = coordination.rest.as_slice() else {
            return false;
        };
        let NounPhrase::Nominal(toughness) = &member.phrase else {
            return false;
        };
        nominal_head_spelling(power) == "power"
            && nominal_head_spelling(toughness) == "toughness"
            && member.conjunction == Some(Conjunction::And)
            && member.comma == Comma::Absent
            && (!requires_base || has_base_modifier(power))
            && (!requires_value
                || matches!(
                    toughness.complements.as_slice(),
                    [NominalComplement::PowerToughness(_)]
                ))
    }

    fn is_keyword_noun_phrase(noun_phrase: &NounPhrase, keyword: &str) -> bool {
        matches!(
            noun_phrase,
            NounPhrase::Nominal(nominal)
                if nominal_head_spelling(nominal).eq_ignore_ascii_case(keyword)
        )
    }

    fn local_power_toughness_precedes_keyword(
        coordination: &crate::syntax::CoordinatedNounPhrase,
        keyword: &str,
    ) -> bool {
        if let (NounPhrase::Coordinated(local), [following]) =
            (coordination.first.as_ref(), coordination.rest.as_slice())
        {
            return is_local_power_toughness_group(local, true, true)
                && following.conjunction == Some(Conjunction::And)
                && following.comma == Comma::Absent
                && is_keyword_noun_phrase(&following.phrase, keyword);
        }

        let (NounPhrase::Nominal(power), [toughness_member, following]) =
            (coordination.first.as_ref(), coordination.rest.as_slice())
        else {
            return false;
        };
        let NounPhrase::Nominal(toughness) = &toughness_member.phrase else {
            return false;
        };
        nominal_head_spelling(power) == "power"
            && has_base_modifier(power)
            && nominal_head_spelling(toughness) == "toughness"
            && matches!(
                toughness.complements.as_slice(),
                [NominalComplement::PowerToughness(_)]
            )
            && toughness_member.conjunction == Some(Conjunction::And)
            && toughness_member.comma == Comma::Absent
            && following.conjunction == Some(Conjunction::And)
            && following.comma == Comma::Absent
            && is_keyword_noun_phrase(&following.phrase, keyword)
    }

    #[test]
    fn per_conjunct_postmodifier_stays_on_the_first_member() {
        let source = "each creature with flying and each player";
        let NounPhrase::Coordinated(coordination) = parse_fixture_noun_phrase(source) else {
            panic!("expected a complete noun-phrase coordination for {source:?}");
        };
        let NounPhrase::Nominal(first) = coordination.first.as_ref() else {
            panic!("expected a nominal first member: {coordination:#?}");
        };
        let [second] = coordination.rest.as_slice() else {
            panic!("expected exactly two members: {coordination:#?}");
        };
        let NounPhrase::Nominal(second) = &second.phrase else {
            panic!("expected a nominal second member: {coordination:#?}");
        };
        assert_eq!(nominal_head_spelling(first), "Creature");
        assert!(matches!(
            first.complements.as_slice(),
            [NominalComplement::Prepositional(preposition)]
                if preposition.head().preposition == crate::syntax::Preposition::With
        ));
        assert_eq!(nominal_head_spelling(second), "player");
        assert!(second.complements.is_empty());
        assert_eq!(
            coordination_verdict(&coordination),
            CoordinationVerdict::Admitted
        );
        assert_eq!(
            linearize_coordinated_noun_phrase(&coordination)
                .expect("the declaration-admitted ownership fixture linearizes"),
            source,
        );
    }

    #[test]
    fn target_artifact_or_creature_is_one_shared_determiner_group() {
        let source = "target artifact or creature";
        let NounPhrase::CoordinatedNominal(coordination) = parse_fixture_noun_phrase(source) else {
            panic!("expected CoordinatedNominal for {source:?}");
        };
        let (determiner, first, rest, complements) =
            coordination::parts_shared_determiner_nominal(&coordination);
        assert_eq!(determiner, &Determiner::Target(None));
        assert_eq!(nominal_head_spelling(first.as_ref()), "Artifact");
        let [second] = rest.as_slice() else {
            panic!("expected exactly two nominal members: {coordination:#?}");
        };
        assert_eq!(second.conjunction, Some(Conjunction::Or));
        assert_eq!(second.comma, Comma::Absent);
        assert_eq!(nominal_head_spelling(&second.phrase), "Creature");
        assert!(complements.is_empty());
        assert_eq!(
            nominal_coordination_verdict(&coordination),
            CoordinationVerdict::Admitted,
        );
        assert_eq!(
            linearize_coordinated_nominal_phrase(&coordination)
                .expect("the declaration-admitted shared-determiner fixture linearizes"),
            source,
        );
    }

    #[test]
    fn base_power_change_keeps_both_local_power_toughness_groups() {
        let source = concat!(
            "At the beginning of your upkeep, change Halfdane's base power and toughness ",
            "to the power and toughness of target creature other than Halfdane until the ",
            "end of your next upkeep."
        );
        let readings = sentence_readings(
            source,
            &fixture_catalogs(),
            SelfReference::new("Halfdane", true),
        );
        let matching = readings.iter().find(|sentence| {
            let Some(object) = main_transitive_object(sentence) else {
                return false;
            };
            let mut groups = Vec::new();
            collect_noun_coordinations(object, &mut groups);
            groups.len() == 2
                && groups
                    .iter()
                    .any(|group| is_local_power_toughness_group(group, true, false))
                && groups.iter().any(|group| {
                    is_local_power_toughness_group(group, false, false)
                        && !matches!(
                            group.first.as_ref(),
                            NounPhrase::Nominal(power) if has_base_modifier(power)
                        )
                })
        });
        assert!(
            matching.is_some(),
            "no strict full-sentence reading kept both local groups: {readings:#?}",
        );
    }

    #[test]
    fn base_power_group_closes_before_lifelink() {
        let source = concat!(
            "If Frodo is a Citizen, it becomes a Halfling Scout with base power and ",
            "toughness 2/3 and lifelink."
        );
        let readings = sentence_readings(
            source,
            &fixture_catalogs(),
            SelfReference::new("Frodo, Sauron's Bane", true),
        );
        let matching = readings.iter().find(|sentence| {
            let Some(object) = main_transitive_object(sentence) else {
                return false;
            };
            let mut groups = Vec::new();
            collect_noun_coordinations(object, &mut groups);
            groups
                .iter()
                .any(|group| local_power_toughness_precedes_keyword(group, "Lifelink"))
        });
        assert!(
            matching.is_some(),
            "no strict full-sentence reading closes the P/T group before lifelink: {readings:#?}",
        );
    }

    #[test]
    fn base_power_group_closes_before_protection() {
        let source = concat!(
            "If this creature is a Soldier, it becomes a Kithkin Avatar with base power ",
            "and toughness 7/8 and protection from each of your opponents."
        );
        let readings = sentence_readings(source, &fixture_catalogs(), SelfReference::default());
        let matching = readings.iter().find(|sentence| {
            let Some(object) = main_transitive_object(sentence) else {
                return false;
            };
            let mut groups = Vec::new();
            collect_noun_coordinations(object, &mut groups);
            groups
                .iter()
                .any(|group| local_power_toughness_precedes_keyword(group, "Protection"))
        });
        assert!(
            matching.is_some(),
            "no strict full-sentence reading closes the P/T group before protection: {readings:#?}",
        );
    }

    #[test]
    fn finite_clause_comma_is_not_consumed_by_noun_coordination() {
        let source = concat!(
            "you may cast target instant or sorcery card from a graveyard, and ",
            "mana of any type can be spent to cast that spell"
        );
        let readings = fixture_enumeration(
            source,
            &fixture_catalogs(),
            Nonterminal::Clause,
            SelfReference::default(),
            |lowered| match lowered {
                Lowered::Clause(reading) => Some(reading),
                _ => None,
            },
        )
        .0;
        let matching = readings.iter().find(|reading| {
            let crate::syntax::Clause::Independent(crate::syntax::IndependentClause::Coordinated(
                coordination,
            )) = reading
            else {
                return false;
            };
            let [continuation] = coordination.rest.as_slice() else {
                return false;
            };
            if continuation.comma != Comma::Present
                || continuation.conjunction != Some(Conjunction::And)
            {
                return false;
            }
            let crate::syntax::IndependentClause::Deontic(
                _,
                _,
                Some(crate::syntax::Predicate::Transitive(first_predicate)),
            ) = coordination.first.as_ref()
            else {
                return false;
            };
            let crate::syntax::PredicateObject::NounPhrase(object) = &first_predicate.object else {
                return false;
            };
            let mut noun_coordinations = Vec::new();
            collect_noun_coordinations(object, &mut noun_coordinations);
            noun_coordinations.len() == 1
                && noun_coordinations.iter().all(|noun_coordination| {
                    noun_coordination
                        .rest
                        .iter()
                        .all(|member| member.comma == Comma::Absent)
                        && coordination_verdict(noun_coordination) == CoordinationVerdict::Admitted
                })
        });
        assert!(
            matching.is_some(),
            "no strict reading preserved the outer clause comma: {readings:#?}",
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one exhaustive provenance check keeps the generated declaration's metadata together"
    )]
    fn coordination_declaration_provenance_is_complete() {
        assert_eq!(coordination::GROUPS.len(), 1);
        let group = coordination::GROUPS[0];
        assert_eq!(group.name, "noun_coordination");
        assert_eq!(
            group.elements,
            [
                "noun_phrase_member",
                "nominal_phrase_member",
                "nominal_complement"
            ],
        );
        assert_eq!(
            group
                .element_data
                .iter()
                .map(|element| element.name)
                .collect::<Vec<_>>(),
            [
                "noun_phrase_member",
                "nominal_phrase_member",
                "nominal_complement"
            ],
        );
        assert_eq!(
            group
                .constructions
                .iter()
                .map(|construction| {
                    (
                        construction.id,
                        construction.own_type,
                        construction.bind_path,
                    )
                })
                .collect::<Vec<_>>(),
            [
                (
                    "noun_phrase_coordination",
                    Some("DerivedCoordinatedNounPhrase"),
                    None
                ),
                (
                    "shared_determiner_nominal",
                    Some("DerivedCoordinatedNominalPhrase"),
                    None
                ),
            ],
        );

        let noun = group
            .constructions
            .iter()
            .find(|construction| construction.id == "noun_phrase_coordination")
            .expect("noun coordination declaration");
        assert_eq!(
            noun.fields
                .iter()
                .map(|field| field.name)
                .collect::<Vec<_>>(),
            ["first", "rest"],
        );
        assert_eq!(
            noun.fields
                .iter()
                .find(|field| field.name == "rest")
                .expect("noun rest sequence")
                .kind,
            FieldKindData::Sequence {
                element: "noun_phrase_member"
            },
        );
        assert_eq!(noun.witnesses.len(), 1);
        assert_eq!(noun.feature_combinators.len(), 1);
        assert_eq!(
            (
                noun.feature_combinators[0].target,
                noun.feature_combinators[0].combinator,
                noun.feature_combinators[0].args,
            ),
            ("first", "noun_phrase_coordination", &["first", "rest"][..])
        );
        assert_eq!(noun.witnesses[0].name, "oxford");
        assert_eq!(
            noun.witnesses[0].class,
            WitnessClassData::Stored {
                path: "rest.last.comma"
            },
        );
        assert_eq!(noun.forms.len(), 1);
        assert_eq!(noun.forms[0].name, "flat");
        assert_eq!(noun.forms[0].ordinal, 0);
        assert!(!noun.forms[0].guarded);
        assert_eq!(
            noun.forms[0].atoms,
            [AtomData::Hole("first"), AtomData::Hole("rest")],
        );

        let shared = group
            .constructions
            .iter()
            .find(|construction| construction.id == "shared_determiner_nominal")
            .expect("shared-determiner declaration");
        assert_eq!(
            shared
                .fields
                .iter()
                .map(|field| field.name)
                .collect::<Vec<_>>(),
            ["determiner", "first", "rest", "complements"],
        );
        assert_eq!(
            shared
                .fields
                .iter()
                .find(|field| field.name == "rest")
                .expect("nominal rest sequence")
                .kind,
            FieldKindData::Sequence {
                element: "nominal_phrase_member"
            },
        );
        assert_eq!(
            shared
                .fields
                .iter()
                .find(|field| field.name == "complements")
                .expect("nominal complement sequence")
                .kind,
            FieldKindData::Sequence {
                element: "nominal_complement"
            },
        );
        assert_eq!(shared.witnesses.len(), 1);
        assert_eq!(shared.feature_combinators.len(), 1);
        assert_eq!(
            (
                shared.feature_combinators[0].target,
                shared.feature_combinators[0].combinator,
                shared.feature_combinators[0].args,
            ),
            (
                "first",
                "noun_phrase_coordination",
                &["determiner", "first", "rest"][..]
            )
        );
        assert_eq!(shared.witnesses[0].name, "oxford");
        assert_eq!(
            shared.witnesses[0].class,
            WitnessClassData::Stored {
                path: "rest.last.comma"
            },
        );
        assert_eq!(shared.forms.len(), 1);
        assert_eq!(shared.forms[0].name, "shared");
        assert_eq!(shared.forms[0].ordinal, 0);
        assert!(!shared.forms[0].guarded);
        assert_eq!(
            shared.forms[0].atoms,
            [
                AtomData::Hole("determiner"),
                AtomData::Hole("first"),
                AtomData::Hole("rest"),
                AtomData::Hole("complements"),
            ],
        );

        let noun_member = group
            .element_data
            .iter()
            .find(|element| element.name == "noun_phrase_member")
            .expect("noun member mapping");
        assert_eq!(noun_member.bind_path, Some("NounPhraseCoordination"));
        assert_eq!(
            noun_member
                .fields
                .iter()
                .map(|field| field.name)
                .collect::<Vec<_>>(),
            ["comma", "conjunction", "phrase"],
        );
        let nominal_member = group
            .element_data
            .iter()
            .find(|element| element.name == "nominal_phrase_member")
            .expect("nominal member mapping");
        assert_eq!(nominal_member.bind_path, Some("NominalPhraseCoordination"));
        assert_eq!(
            nominal_member
                .fields
                .iter()
                .map(|field| field.name)
                .collect::<Vec<_>>(),
            ["comma", "conjunction", "phrase"],
        );
        let nominal_complement = group
            .element_data
            .iter()
            .find(|element| element.name == "nominal_complement")
            .expect("nominal complement mapping");
        assert_eq!(nominal_complement.bind_path, Some("NominalComplement"));
        assert!(nominal_complement.fields.is_empty());
    }

    #[test]
    fn noun_coordination_builder_admits_binary_and_refuses_binary_oxford() {
        let built = coordination::build_noun_phrase_coordination(
            Box::new(NounPhrase::Quantity(Quantity::Both)),
            vec![NounPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::And),
                phrase: NounPhrase::Quantity(Quantity::X),
            }],
        )
        .expect("a binary noun coordination without an Oxford comma is admitted");
        let (first, rest) = coordination::parts_noun_phrase_coordination(&built);
        assert_eq!(first.as_ref(), &NounPhrase::Quantity(Quantity::Both));
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].comma, Comma::Absent);
        assert_eq!(rest[0].conjunction, Some(Conjunction::And));
        assert_eq!(rest[0].phrase, NounPhrase::Quantity(Quantity::X));

        let violation = coordination::build_noun_phrase_coordination(
            Box::new(NounPhrase::Quantity(Quantity::Both)),
            vec![NounPhraseCoordination {
                comma: Comma::Present,
                conjunction: Some(Conjunction::And),
                phrase: NounPhrase::Quantity(Quantity::X),
            }],
        )
        .expect_err("a binary noun coordination cannot carry an Oxford comma");
        assert_eq!(violation.construction, "noun_phrase_coordination");
        assert_eq!(
            violation.requirement,
            "any(rest.len() >= 2, rest.last.comma in [Absent])",
        );
    }

    #[test]
    fn generated_builder_rejects_a_chart_unshaped_interior_member() {
        let violation = coordination::build_noun_phrase_coordination(
            Box::new(noun_phrase(Vocab::Card)),
            vec![
                NounPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: None,
                    phrase: noun_phrase(Vocab::Spell),
                },
                NounPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: Some(Conjunction::And),
                    phrase: noun_phrase(Vocab::Ability),
                },
                NounPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: Some(Conjunction::Or),
                    phrase: noun_phrase(Vocab::Card),
                },
            ],
        )
        .expect_err("only the final sequence member may carry a conjunction");
        assert_eq!(violation.requirement, "rest.nonfinal.conjunction.is_none()",);
    }

    #[test]
    fn shared_determiner_builder_enforces_oxford_and_empty_complements() {
        let built = coordination::build_shared_determiner_nominal(
            Determiner::Any,
            Box::new(nominal(Vocab::Card)),
            vec![NominalPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::Or),
                phrase: nominal(Vocab::Spell),
            }],
            Vec::new(),
        )
        .expect("a binary shared-determiner nominal with no complement is admitted");
        let (determiner, first, rest, complements) =
            coordination::parts_shared_determiner_nominal(&built);
        assert_eq!(determiner, &Determiner::Any);
        assert_eq!(first.as_ref(), &nominal(Vocab::Card));
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].comma, Comma::Absent);
        assert_eq!(rest[0].conjunction, Some(Conjunction::Or));
        assert_eq!(rest[0].phrase, nominal(Vocab::Spell));
        assert!(complements.is_empty());

        let oxford = coordination::build_shared_determiner_nominal(
            Determiner::Any,
            Box::new(nominal(Vocab::Card)),
            vec![NominalPhraseCoordination {
                comma: Comma::Present,
                conjunction: Some(Conjunction::Or),
                phrase: nominal(Vocab::Spell),
            }],
            Vec::new(),
        )
        .expect_err("a binary shared-determiner nominal cannot carry an Oxford comma");
        assert_eq!(oxford.construction, "shared_determiner_nominal");
        assert_eq!(
            oxford.requirement,
            "any(rest.len() >= 2, rest.last.comma in [Absent])",
        );

        let complement = coordination::build_shared_determiner_nominal(
            Determiner::Any,
            Box::new(nominal(Vocab::Card)),
            vec![NominalPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::Or),
                phrase: nominal(Vocab::Spell),
            }],
            vec![NominalComplement::Quantity(Quantity::Both)],
        )
        .expect_err("the opaque nominal-complement sequence is currently proved empty");
        assert_eq!(complement.construction, "shared_determiner_nominal");
        assert_eq!(complement.requirement, "complements.len() == 0");
    }

    #[test]
    fn noun_coordination_linearizes_binary_and_oxford_members_in_declared_order() {
        let binary = coordination::build_noun_phrase_coordination(
            Box::new(noun_phrase(Vocab::Card)),
            vec![NounPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::And),
                phrase: noun_phrase(Vocab::Spell),
            }],
        )
        .expect("binary noun coordination is declared");
        assert_eq!(coordination_verdict(&binary), CoordinationVerdict::Admitted);
        assert_eq!(
            linearize_coordinated_noun_phrase(&binary).expect("binary members render"),
            "card and spell",
        );

        let oxford = coordination::build_noun_phrase_coordination(
            Box::new(noun_phrase(Vocab::Card)),
            vec![
                NounPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: None,
                    phrase: noun_phrase(Vocab::Spell),
                },
                NounPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: Some(Conjunction::Or),
                    phrase: noun_phrase(Vocab::Ability),
                },
            ],
        )
        .expect("Oxford noun coordination is declared");
        let rendered = linearize_coordinated_noun_phrase(&oxford).expect("Oxford members render");
        assert_eq!(rendered, "card, spell, or ability");
        assert!(rendered.starts_with("card,"));
        assert!(!rendered.contains("card ,"));
    }

    #[test]
    fn nominal_coordination_linearizes_binary_and_oxford_with_one_shared_determiner() {
        let binary = coordination::build_shared_determiner_nominal(
            Determiner::Any,
            Box::new(nominal(Vocab::Card)),
            vec![NominalPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::Or),
                phrase: nominal(Vocab::Spell),
            }],
            Vec::new(),
        )
        .expect("binary shared-determiner coordination is declared");
        assert_eq!(
            nominal_coordination_verdict(&binary),
            CoordinationVerdict::Admitted,
        );
        assert_eq!(
            linearize_coordinated_nominal_phrase(&binary).expect("binary nominals render"),
            "any card or spell",
        );
        let (_, first, rest, complements) = coordination::parts_shared_determiner_nominal(&binary);
        assert_eq!(first.determiner, None);
        assert_eq!(rest[0].phrase.determiner, None);
        assert!(complements.is_empty());

        let oxford = coordination::build_shared_determiner_nominal(
            Determiner::Any,
            Box::new(nominal(Vocab::Card)),
            vec![
                NominalPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: None,
                    phrase: nominal(Vocab::Spell),
                },
                NominalPhraseCoordination {
                    comma: Comma::Present,
                    conjunction: Some(Conjunction::Or),
                    phrase: nominal(Vocab::Ability),
                },
            ],
            Vec::new(),
        )
        .expect("Oxford shared-determiner coordination is declared");
        let rendered =
            linearize_coordinated_nominal_phrase(&oxford).expect("Oxford nominals render");
        assert_eq!(rendered, "any card, spell, or ability");
        assert_eq!(rendered.matches("any").count(), 1);
        assert!(rendered.starts_with("any card,"));
    }

    #[test]
    fn coordination_verdicts_report_the_generated_requirement_that_refused_the_value() {
        let noun = crate::syntax::CoordinatedNounPhrase {
            first: Box::new(noun_phrase(Vocab::Card)),
            rest: vec![NounPhraseCoordination {
                comma: Comma::Present,
                conjunction: Some(Conjunction::And),
                phrase: noun_phrase(Vocab::Spell),
            }],
        };
        assert_eq!(
            coordination_verdict(&noun),
            CoordinationVerdict::Refused {
                requirement: "any(rest.len() >= 2, rest.last.comma in [Absent])",
            },
        );

        let nominal = crate::syntax::CoordinatedNominalPhrase {
            determiner: Determiner::Any,
            first: Box::new(nominal(Vocab::Card)),
            rest: vec![NominalPhraseCoordination {
                comma: Comma::Absent,
                conjunction: Some(Conjunction::Or),
                phrase: nominal(Vocab::Spell),
            }],
            complements: vec![NominalComplement::Quantity(Quantity::Both)],
        };
        assert_eq!(
            nominal_coordination_verdict(&nominal),
            CoordinationVerdict::Refused {
                requirement: "complements.len() == 0",
            },
        );
    }

    #[test]
    fn parsed_coordination_ground_obeys_linearization_law() {
        const NOUN_COORDINATION_GROUND: &[&str] = &[
            "target creature and target land",
            "target artifact or target creature",
            "target artifact, target creature, or target land",
            "target artifact, target creature, target land, and target planeswalker",
        ];
        const NOMINAL_COORDINATION_GROUND: &[&str] = &[
            "target artifact or creature",
            "target artifact, creature, or land",
        ];

        for &source in NOUN_COORDINATION_GROUND {
            let readings = noun_phrase_readings(source);
            let expected = readings
                .iter()
                .filter_map(|reading| match reading {
                    NounPhrase::Coordinated(coordination) => Some(coordination),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let admitted = expected
                .iter()
                .copied()
                .filter(|coordination| {
                    coordination_verdict(coordination) == CoordinationVerdict::Admitted
                })
                .collect::<Vec<_>>();
            eprintln!(
                "noun ground {source:?}: readings={}, expected={}, admitted={}",
                readings.len(),
                expected.len(),
                admitted.len(),
            );
            assert!(
                !admitted.is_empty(),
                "no admitted CoordinatedNounPhrase reading for {source:?}: {readings:#?}",
            );
            for coordination in admitted {
                assert_eq!(
                    linearize_coordinated_noun_phrase(coordination).unwrap_or_else(|error| panic!(
                        "linearization failed for {source:?}: {error}"
                    )),
                    source,
                    "parsed-ground law 1 failed for {coordination:#?}",
                );
            }
        }

        for &source in NOMINAL_COORDINATION_GROUND {
            let readings = noun_phrase_readings(source);
            let expected = readings
                .iter()
                .filter_map(|reading| match reading {
                    NounPhrase::CoordinatedNominal(coordination) => Some(coordination),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let admitted = expected
                .iter()
                .copied()
                .filter(|coordination| {
                    nominal_coordination_verdict(coordination) == CoordinationVerdict::Admitted
                })
                .collect::<Vec<_>>();
            eprintln!(
                "nominal ground {source:?}: readings={}, expected={}, admitted={}",
                readings.len(),
                expected.len(),
                admitted.len(),
            );
            assert!(
                !admitted.is_empty(),
                "no admitted CoordinatedNominalPhrase reading for {source:?}: {readings:#?}",
            );
            for coordination in admitted {
                assert_eq!(
                    linearize_coordinated_nominal_phrase(coordination).unwrap_or_else(
                        |error| panic!("linearization failed for {source:?}: {error}")
                    ),
                    source,
                    "parsed-ground law 1 failed for {coordination:#?}",
                );
            }
        }
    }

    #[test]
    fn handwritten_binary_oxford_surfaces_are_refused_by_the_declaration() {
        const REQUIREMENT: &str = "any(rest.len() >= 2, rest.last.comma in [Absent])";

        let noun_source = "target creature, and target land";
        let noun_readings = noun_phrase_readings(noun_source);
        let noun_coordinations = noun_readings
            .iter()
            .filter_map(|reading| match reading {
                NounPhrase::Coordinated(coordination) => Some(coordination),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(
            !noun_coordinations.is_empty(),
            "the handwritten grammar produced no complete coordinated reading",
        );
        for coordination in noun_coordinations {
            assert_eq!(
                coordination_verdict(coordination),
                CoordinationVerdict::Refused {
                    requirement: REQUIREMENT,
                },
                "binary Oxford noun reading was not refused: {coordination:#?}",
            );
        }

        let nominal_source = "target artifact, or creature";
        let nominal_readings = noun_phrase_readings(nominal_source);
        let nominal_coordinations = nominal_readings
            .iter()
            .filter_map(|reading| match reading {
                NounPhrase::CoordinatedNominal(coordination) => Some(coordination),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(
            !nominal_coordinations.is_empty(),
            "the handwritten grammar produced no shared-determiner coordinated reading",
        );
        for coordination in nominal_coordinations {
            assert_eq!(
                nominal_coordination_verdict(coordination),
                CoordinationVerdict::Refused {
                    requirement: REQUIREMENT,
                },
                "binary Oxford nominal reading was not refused: {coordination:#?}",
            );
        }
        eprintln!(
            "illegal binary Oxford surfaces: noun coordinated readings={}, nominal coordinated readings={}",
            noun_readings
                .iter()
                .filter(|reading| matches!(reading, NounPhrase::Coordinated(_)))
                .count(),
            nominal_readings
                .iter()
                .filter(|reading| matches!(reading, NounPhrase::CoordinatedNominal(_)))
                .count(),
        );
    }

    fn noun_coordination_depth(noun_phrase: &NounPhrase) -> usize {
        match noun_phrase {
            NounPhrase::Coordinated(coordination) => {
                let child_depth = std::iter::once(coordination.first.as_ref())
                    .chain(coordination.rest.iter().map(|member| &member.phrase))
                    .map(noun_coordination_depth)
                    .max()
                    .unwrap_or(0);
                child_depth + 1
            }
            _ => 0,
        }
    }

    #[test]
    fn chain_associativity_keeps_two_asts_for_the_same_bytes() {
        let source = "target creature and target land or target artifact";
        let enumeration = noun_phrase_enumeration(source);
        assert!(
            enumeration.selections >= enumeration.noun_phrase_lowerings,
            "lowerings cannot outnumber completed ChoiceMap selections",
        );
        assert!(
            enumeration.noun_phrase_lowerings > enumeration.readings.len(),
            "the M4 Debug-key dedup guard did not eliminate any duplicate lowerings",
        );
        let coordinated = enumeration
            .readings
            .iter()
            .filter_map(|reading| match reading {
                NounPhrase::Coordinated(coordination) => Some((reading, coordination)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(
            coordinated
                .iter()
                .any(|(reading, _)| noun_coordination_depth(reading) == 1),
            "no flat coordination reading: {:#?}",
            enumeration.readings,
        );
        assert!(
            coordinated
                .iter()
                .any(|(reading, _)| noun_coordination_depth(reading) >= 2),
            "no nested coordination reading: {:#?}",
            enumeration.readings,
        );
        let admitted = coordinated
            .iter()
            .filter(|(_, coordination)| {
                coordination_verdict(coordination) == CoordinationVerdict::Admitted
            })
            .collect::<Vec<_>>();
        assert!(!admitted.is_empty(), "no admitted coordination reading");
        for (_, coordination) in &admitted {
            assert_eq!(
                linearize_coordinated_noun_phrase(coordination)
                    .expect("admitted chain reading linearizes"),
                source,
            );
        }
        eprintln!(
            "chain associativity: selections={}, noun-phrase lowerings={}, unique readings={}, coordinated={}, admitted={}, debug-key dedup fired={}",
            enumeration.selections,
            enumeration.noun_phrase_lowerings,
            enumeration.readings.len(),
            coordinated.len(),
            admitted.len(),
            enumeration.noun_phrase_lowerings > enumeration.readings.len(),
        );
    }

    fn chart_shaped_noun_rest(
        members: impl IntoIterator<Item = NounPhrase>,
        conjunction: Conjunction,
        member_count: usize,
    ) -> Vec<NounPhraseCoordination> {
        members
            .into_iter()
            .enumerate()
            .map(|(index, phrase)| NounPhraseCoordination {
                comma: Comma::from(member_count >= 3),
                conjunction: (index + 2 == member_count).then_some(conjunction),
                phrase,
            })
            .collect()
    }

    fn chart_shaped_nominal_rest(
        members: impl IntoIterator<Item = NominalPhrase>,
        conjunction: Conjunction,
        member_count: usize,
    ) -> Vec<NominalPhraseCoordination> {
        members
            .into_iter()
            .enumerate()
            .map(|(index, phrase)| NominalPhraseCoordination {
                comma: Comma::from(member_count >= 3),
                conjunction: (index + 2 == member_count).then_some(conjunction),
                phrase,
            })
            .collect()
    }

    fn parsed_noun_member(index: usize) -> NounPhrase {
        const SOURCES: &[&str] = &[
            "target artifact",
            "target creature",
            "target land",
            "target planeswalker",
        ];
        parse_fixture_noun_phrase(SOURCES[index])
    }

    fn parsed_nominal_member(index: usize) -> NominalPhrase {
        const SOURCES: &[&str] = &["artifact", "creature", "land", "planeswalker"];
        let source = SOURCES[index];
        match parse_fixture_noun_phrase(source) {
            NounPhrase::Nominal(nominal) if nominal.determiner.is_none() => nominal,
            reading => panic!(
                "strict nominal member ground was not a bare nominal for {source:?}: {reading:#?}"
            ),
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn built_noun_coordination_round_trips_through_enumerated_parses(
            member_indices in prop::collection::vec(0_usize..4, 2..=4),
            conjunction in prop::sample::select(vec![Conjunction::And, Conjunction::Or]),
        ) {
            let members = member_indices
                .iter()
                .copied()
                .map(parsed_noun_member)
                .collect::<Vec<_>>();
            let member_count = members.len();
            let first = Box::new(members[0].clone());
            let rest = chart_shaped_noun_rest(
                members.into_iter().skip(1),
                conjunction,
                member_count,
            );
            let mut generated_builder_called = false;
            let mut build = |first, rest| {
                generated_builder_called = true;
                coordination::build_noun_phrase_coordination(first, rest)
            };
            let built = build(first, rest)
                .expect("chart-shaped noun coordination satisfies the declaration");
            prop_assert!(
                generated_builder_called,
                "the law value did not enter through the generated noun builder",
            );
            let bytes = linearize_coordinated_noun_phrase(&built)
                .expect("a built noun coordination linearizes");
            let reparsed = noun_phrase_readings(&bytes);
            prop_assert!(
                reparsed.contains(&NounPhrase::Coordinated(built.clone())),
                "generated bytes did not recover the built noun value: bytes={bytes:?}, built={built:#?}, reparsed={reparsed:#?}",
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn built_shared_determiner_nominal_round_trips_through_enumerated_parses(
            member_indices in prop::collection::vec(0_usize..4, 2..=4),
            conjunction in prop::sample::select(vec![Conjunction::And, Conjunction::Or]),
        ) {
            let members = member_indices
                .iter()
                .copied()
                .map(parsed_nominal_member)
                .collect::<Vec<_>>();
            let member_count = members.len();
            let first = Box::new(members[0].clone());
            let rest = chart_shaped_nominal_rest(
                members.into_iter().skip(1),
                conjunction,
                member_count,
            );
            let mut generated_builder_called = false;
            let mut build = |determiner, first, rest, complements| {
                generated_builder_called = true;
                coordination::build_shared_determiner_nominal(
                    determiner,
                    first,
                    rest,
                    complements,
                )
            };
            let built = build(
                Determiner::Target(None),
                first,
                rest,
                Vec::new(),
            )
            .expect("chart-shaped nominal coordination satisfies the declaration");
            prop_assert!(
                generated_builder_called,
                "the law value did not enter through the generated shared-determiner builder",
            );
            let bytes = linearize_coordinated_nominal_phrase(&built)
                .expect("a built shared-determiner coordination linearizes");
            let reparsed = noun_phrase_readings(&bytes);
            prop_assert!(
                reparsed.contains(&NounPhrase::CoordinatedNominal(built.clone())),
                "generated bytes did not recover the built nominal value: bytes={bytes:?}, built={built:#?}, reparsed={reparsed:#?}",
            );
        }
    }

    #[test]
    fn choice_map_lowering_matches_best_parse_lowering() {
        let catalogs = fixture_catalogs();
        for &(nonterminal, source) in &[
            (Nonterminal::Clause, "copy that spell"),
            (Nonterminal::NounPhrase, "the declare attackers step"),
            (
                Nonterminal::Sentence,
                "Exile target artifact, creature, or planeswalker and target land or battle.",
            ),
        ] {
            let self_reference = SelfReference::default();
            let surface = lex(source);
            let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
            let grammar = EnglishGrammar::with_opacity_mode(
                source,
                &catalogs,
                nonterminal,
                OpacityMode::Exact,
                self_reference,
            );
            let chart = parse_chart(&grammar, &tokens)
                .unwrap_or_else(|error| panic!("chart failed for {source:?}: {error:?}"));
            let mut best_syntax = None;
            let (root, best) = chart
                .forest
                .best_root_matching(
                    chart.roots.iter().copied(),
                    super::super::construction::registry(),
                    |root, best| {
                        best_syntax = lower(&grammar, &chart.forest, root, best);
                        best_syntax.is_some()
                    },
                )
                .expect("acyclic forest")
                .unwrap_or_else(|| panic!("no lowerable root for {source:?}"));
            let map = ChoiceMap::from_best(&best);
            let via_map = lower(&grammar, &chart.forest, root, &map)
                .unwrap_or_else(|| panic!("map lowering declined for {source:?}"));
            assert_eq!(
                format!("{via_map:?}"),
                format!("{:?}", best_syntax.expect("matching root lowered")),
                "ChoiceMap lowering diverged from BestParse lowering for {source:?}",
            );
        }
    }

    #[test]
    fn linearize_joins_tokens_with_no_space_before_comma() {
        let padded = GeneratedParse {
            construction: "probe_word",
            ordinal: 7,
            parts: vec![
                GeneratedPart::Literal(","),
                GeneratedPart::Scalar {
                    field: "word",
                    value: GeneratedScalar::Conjunction(Conjunction::And),
                },
            ],
        };
        assert_eq!(linearize(&padded), ", and");
        let pair = GeneratedParse {
            construction: "probe_pair",
            ordinal: 0,
            parts: vec![
                GeneratedPart::Subtree {
                    field: "first",
                    parse: GeneratedParse {
                        construction: "probe_word",
                        ordinal: 0,
                        parts: vec![GeneratedPart::Scalar {
                            field: "word",
                            value: GeneratedScalar::Conjunction(Conjunction::And),
                        }],
                    },
                },
                GeneratedPart::Scalar {
                    field: "tail",
                    value: GeneratedScalar::Comma(Comma::Present),
                },
                GeneratedPart::Subtree {
                    field: "second",
                    parse: GeneratedParse {
                        construction: "probe_word",
                        ordinal: 0,
                        parts: vec![GeneratedPart::Scalar {
                            field: "word",
                            value: GeneratedScalar::Conjunction(Conjunction::Or),
                        }],
                    },
                },
            ],
        };
        assert_eq!(linearize(&pair), "and, or");

        let absent = GeneratedParse {
            construction: "probe_pair",
            ordinal: 0,
            parts: vec![
                GeneratedPart::Scalar {
                    field: "first",
                    value: GeneratedScalar::Conjunction(Conjunction::And),
                },
                GeneratedPart::Scalar {
                    field: "comma",
                    value: GeneratedScalar::Comma(Comma::Absent),
                },
                GeneratedPart::Scalar {
                    field: "second",
                    value: GeneratedScalar::Conjunction(Conjunction::Or),
                },
            ],
        };
        assert_eq!(linearize(&absent), "and or");
    }

    #[test]
    fn parse_as_carries_the_declared_form_ordinal_as_the_form_witness() {
        let bare = parse_as(
            "or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the bare probe word parses");
        assert_eq!(bare.len(), 1);
        assert_eq!(bare[0].ast().construction, "probe_word");
        assert_eq!(bare[0].ast().ordinal, 0);
        assert_eq!(linearize(bare[0].ast()), "or");

        let padded = parse_as(
            ", and",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the padded probe word parses");
        assert_eq!(padded.len(), 1);
        assert_eq!(padded[0].ast().construction, "probe_word");
        assert_eq!(
            padded[0].ast().ordinal,
            7,
            "the declared `@ 7` ordinal is the form witness",
        );
        assert_eq!(linearize(padded[0].ast()), ", and");
    }

    #[test]
    fn parse_as_returns_the_empty_set_when_nothing_is_admitted() {
        // Contrast case first: the comma-bearing pair parses under these
        // same groups, so an empty set below is admission refusal, not a
        // wiring failure.
        let admitted = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the comma-bearing pair parses");
        assert!(!admitted.is_empty());
        assert!(admitted.iter().all(|exact| {
            exact.ast().parts.iter().any(|part| {
                matches!(
                    part,
                    GeneratedPart::Scalar {
                        field: "tail",
                        value: GeneratedScalar::Comma(Comma::Present),
                    }
                )
            })
        }));
        let refused = parse_as(
            "and or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("no complete parse is the EMPTY SET, not an error");
        assert_eq!(refused, Vec::new());
    }

    #[test]
    fn exceeding_the_budget_is_too_many_alternatives() {
        // "and" as ProbeRoot packs probe_pick and probe_pick_shadow: two
        // selections against a budget of one.
        let error = parse_as(
            "and",
            &Catalogs::default(),
            probe_category("ProbeRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            1,
        )
        .expect_err("two derivations cannot fit a budget of one");
        assert!(
            matches!(error, ExactParseError::TooManyAlternatives { budget: 1 }),
            "unexpected error: {error:?}",
        );
    }

    #[test]
    fn one_byte_string_two_asts() {
        let set = parse_as(
            "or",
            &Catalogs::default(),
            law_category("LawRoot"),
            GeneratedActivation::Groups(law::GROUPS),
            100,
        )
        .expect("the ambiguous root parses");
        assert_eq!(set.len(), 2, "law_first and law_second both admit: {set:?}");
        let mut ids = set
            .iter()
            .map(|member| member.ast().construction)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(ids, ["law_first", "law_second"]);
        for member in &set {
            assert_eq!(linearize(member.ast()), "or");
        }
    }

    #[test]
    fn one_ast_two_surfaces_distinguished_by_form_witness() {
        // probe_word admits the same scalar value through two forms: bare
        // (`@ 0`) and comma-padded (`@ 7`). Same construction, same field
        // values, different form witness, different bytes — and linearize
        // maps each exact parse back to ITS OWN surface.
        //
        // The two `GeneratedParse` values below are NOT equal as wholes —
        // only `construction` and the scalar field values are asserted equal.
        // The padded form's `parts` carries an extra `Literal(",")` and a
        // different `ordinal`; this test checks exactly the fields named,
        // not full-value equality.
        let activation = GeneratedActivation::Groups(probe::GROUPS);
        let bare = parse_as(
            "or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            activation,
            100,
        )
        .expect("bare parses");
        let padded = parse_as(
            ", or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            activation,
            100,
        )
        .expect("padded parses");
        assert_eq!(bare.len(), 1, "one exact parse for the bare surface");
        assert_eq!(padded.len(), 1, "one exact parse for the padded surface");
        let (bare, padded) = (&bare[0], &padded[0]);
        assert_eq!(bare.ast().construction, padded.ast().construction);
        let scalar_fields = |parse: &GeneratedParse| {
            parse
                .parts
                .iter()
                .filter_map(|part| match part {
                    GeneratedPart::Scalar { field, value } => Some((*field, *value)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            scalar_fields(bare.ast()),
            scalar_fields(padded.ast()),
            "the two surfaces carry the same field values",
        );
        assert_ne!(
            bare.ast().ordinal,
            padded.ast().ordinal,
            "the witness differs"
        );
        assert_eq!(linearize(bare.ast()), "or");
        assert_eq!(linearize(padded.ast()), ", or");
    }

    #[test]
    fn dominance_losing_derivations_stay_in_the_set() {
        // probe_pick dominates probe_pick_shadow; the SELECTED parse drops
        // the shadow, but the exact-parse SET keeps it — dominance is
        // selection preference, not admission.
        let set = parse_as(
            "and",
            &Catalogs::default(),
            probe_category("ProbeRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the probe root parses");
        assert_eq!(set.len(), 2);
        let mut ids = set
            .iter()
            .map(|member| member.ast().construction)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(ids, ["probe_pick", "probe_pick_shadow"]);
        for member in &set {
            assert_eq!(linearize(member.ast()), "and");
        }
    }

    #[test]
    fn every_exact_parse_linearizes_to_its_source_and_reparses_to_itself() {
        // The two round-trip laws over the nested pair: probe_pair's product
        // space ("and, or" = pick/shadow choices in each hole = 4 exact
        // parses). Law 1: linearize(ep) == source, for every member. Law 2:
        // reparsing linearize(ep) under a REVERSED registration order still
        // contains ep — the exact parse recovered from its own linearization
        // survives a re-derivation that took a different path to get there,
        // not just a repeat of the same deterministic call.
        let activation = GeneratedActivation::Groups(probe::GROUPS);
        let source = "and, or";
        let set = parse_as(
            source,
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            activation,
            100,
        )
        .expect("the pair parses");
        assert_eq!(set.len(), 4, "2 root choices x 2 per hole: {set:?}");
        for member in &set {
            assert_eq!(linearize(member.ast()), source, "law 1 for {member:?}");
        }
        for member in &set {
            let reparsed = parse_as_with_registration_order(
                &linearize(member.ast()),
                &Catalogs::default(),
                probe_category("ProbePairRoot"),
                activation,
                100,
                RegistrationOrder::Reversed,
            )
            .expect("linearized bytes reparse under a reversed registration order");
            assert!(reparsed.contains(member), "law 2 for {member:?}");
        }
    }

    #[test]
    fn law_outputs_survive_registration_and_group_permutations() {
        let normal = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("normal order parses");
        for order in [RegistrationOrder::Reversed, RegistrationOrder::FixedShuffle] {
            let permuted = parse_as_with_registration_order(
                "and, or",
                &Catalogs::default(),
                probe_category("ProbePairRoot"),
                GeneratedActivation::Groups(probe::GROUPS),
                100,
                order,
            )
            .expect("permuted order parses");
            assert!(
                same_exact_set(&normal, &permuted),
                "law outputs changed under {order:?}: {normal:?} vs {permuted:?}",
            );
        }
        let reversed_groups = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS_REVERSED),
            100,
        )
        .expect("reversed group list parses");
        assert!(
            same_exact_set(&normal, &reversed_groups),
            "law outputs changed under group-list reversal",
        );
    }

    #[test]
    fn handwritten_enumeration_drops_unlowerable_selections_and_keeps_the_best() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"]);
        let source = "copy that spell";
        let self_reference = SelfReference::default();
        let surface = lex(source);
        let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
        let grammar = EnglishGrammar::with_opacity_mode(
            source,
            &catalogs,
            Nonterminal::Clause,
            OpacityMode::Exact,
            self_reference,
        );
        let chart = parse_chart(&grammar, &tokens).expect("the chart parses");
        assert!(!chart.roots.is_empty());

        let mut best_syntax = None;
        let (_, _) = chart
            .forest
            .best_root_matching(
                chart.roots.iter().copied(),
                super::super::construction::registry(),
                |root, best| {
                    best_syntax = lower(&grammar, &chart.forest, root, best);
                    best_syntax.is_some()
                },
            )
            .expect("acyclic forest")
            .expect("a lowerable root exists");
        let best_debug = format!("{:?}", best_syntax.expect("matching root lowered"));

        let mut remaining = 10_000_usize;
        let mut lowered = Vec::new();
        let mut declined = 0_usize;
        for &root in &chart.roots {
            let selections = chart
                .forest
                .enumerate_selections(root, &mut remaining)
                .unwrap_or_else(|error| {
                    panic!(
                        "enumeration over the handwritten grammar failed: {error:?} \
                         (acyclic-by-construction is not a given here, unlike the \
                         generated fixtures)"
                    )
                });
            for selection in &selections {
                match lower(&grammar, &chart.forest, root, selection) {
                    Some(syntax) => {
                        let rendered = format!("{syntax:?}");
                        if !lowered.contains(&rendered) {
                            lowered.push(rendered);
                        }
                    }
                    None => declined += 1,
                }
            }
        }
        // Exactly one lowerable reading here: `contains` below is an
        // identity check, not a selection among several candidates.
        assert_eq!(
            lowered.len(),
            1,
            "the handwritten grammar admits exactly one lowerable reading of this clause",
        );
        assert!(
            lowered.contains(&best_debug),
            "the production-selected reading is in the enumerated set",
        );
        assert!(
            declined > 0,
            "the unlowerable nominal reading appears among selections and is dropped",
        );
    }
}
