use super::Adjective;
use super::AdjectiveComparisonState;
use super::AdjectivePhrase;
use super::AuxiliaryInstance;
use super::CardOrientation;
use super::CatalogKind;
use super::Clause;
use super::ComparisonComplement;
use super::ComparisonMarker;
use super::Conjunction;
use super::ContractedSubjectAuxiliary;
use super::ContractedSubjectKey;
use super::CopularRemainder;
use super::Determiner;
use super::EnglishGrammar;
use super::EnglishSurfaceWitness;
use super::ExistentialForm;
use super::ForestSymbol;
use super::FrequencyPhrase;
use super::GerundClause;
use super::InfinitiveClause;
use super::KeywordArgument;
use super::KeywordCost;
use super::MeaningKey;
use super::NodeId;
use super::NominalComplement;
use super::NominalModifier;
use super::NominalPhrase;
use super::Noun;
use super::NounForm;
use super::NounInstance;
use super::NounPhrase;
use super::NumberLiteral;
use super::OpacitySlot;
use super::OpaqueLexeme;
use super::OracleSymbol;
use super::Phrase;
use super::Polarity;
use super::Possessor;
use super::PowerToughness;
use super::PredicatedArgument;
use super::PredicatedQuality;
use super::Preposition;
use super::PrepositionalPhrase;
use super::PronounCase;
use super::PronounInstance;
use super::Quantity;
use super::RelativeClause;
use super::RelativeMarker;
use super::RuleImpl;
use super::RuleTag;
use super::Sentence;
use super::SetExceptionMarker;
use super::SetExceptionNounPhrase;
use super::SimpleClause;
use super::Subject;
use super::ThisCardForm;
use super::Verb;
use super::VerbAnalysis;
use super::VerbParticle;
use super::VerbPhrase;
use super::Vocab;
use super::ability;
use super::adjective_comparison_state;
use super::clause;
use super::opacity;
use super::parse_support::EnglishForest;
use super::quantity_value;
use crate::forest::AlternativeSelection;
use crate::word::Tense;

pub(super) enum GeneratedValue {
    Typed(deckmaste_construction_compiler::runtime::ErasedValue),
    Sequence(Vec<deckmaste_construction_compiler::runtime::ErasedValue>),
}

impl std::fmt::Debug for GeneratedValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Typed(_) => formatter.write_str("Typed(<generated>)"),
            Self::Sequence(values) => formatter
                .debug_tuple("Sequence")
                .field(&values.len())
                .finish(),
        }
    }
}

#[cfg(test)]
impl GeneratedValue {
    pub(super) fn is<T: std::any::Any>(&self) -> bool {
        matches!(self, Self::Typed(value) if value.is::<T>())
    }

    pub(super) fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        let Self::Typed(value) = self else {
            return None;
        };
        value.downcast_ref()
    }
}

#[allow(
    dead_code,
    reason = "elliptical-clause lowering is staged for later grammar milestones"
)]
#[derive(Debug)]
pub(super) enum Lowered {
    Number(NumberLiteral),
    Quantity(Quantity),
    Determiner(Determiner),
    Adjective(Adjective),
    AdjectivePhrase(AdjectivePhrase),
    ComparisonComplement(ComparisonComplement),
    Noun(NounInstance),
    NominalModifier(NominalModifier),
    CoordinatedModifier(crate::syntax::CoordinatedModifier),
    Nominal(NominalPhrase),
    DevotionColors(crate::syntax::DevotionColors),
    PossessiveNominal(NominalPhrase),
    NounPhrase(NounPhrase),
    Catalog(crate::catalog::CatalogAtom),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    CoinResult(crate::syntax::CoinSide),
    Frequency(FrequencyPhrase),
    Auxiliary(AuxiliaryInstance),
    SubjectAuxiliary(ContractedSubjectAuxiliary),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    /// One quality of a `kwgrant`-round predicated keyword argument.
    PredicatedQuality(crate::syntax::PredicatedQuality),
    /// A coordinated list of [`Self::PredicatedQuality`] members, in surface
    /// order.
    PredicatedArgument(crate::syntax::PredicatedArgument),
    /// Uniform payload for all six mana-list rules.
    ManaAmount(crate::syntax::PredicateObject),
    PowerToughness(PowerToughness),
    Pronoun(PronounInstance),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    Phrase(Phrase),
    PrepositionalPhrase(PrepositionalPhrase),
    Verb(VerbAnalysis),
    VerbPhrase(VerbPhrase),
    InfinitiveClause(InfinitiveClause),
    GerundClause(GerundClause),
    CopularRemainder(CopularRemainder),
    SimpleClause(SimpleClause),
    EllipticalClause(crate::syntax::EllipticalClause),
    Clause(Clause),
    RelativeClause(RelativeClause),
    Sentence(Sentence),
    Conjunction(Conjunction),
    Subordinator(crate::syntax::Subordinator),
    RelativeMarker(RelativeMarker),
    Existential(ExistentialForm),
    ExceptionRider(crate::syntax::ExceptionRider),
    /// One restriction-run member's adjunct sequence (one adjunct for most
    /// members, two for the flat `once each turn` adverb+temporal pair).
    RestrictionMember(Vec<crate::syntax::PredicateAdjunct>),
    RestrictionRun(crate::syntax::RestrictionRun),
    Generated(GeneratedValue),
    Ignored,
}

pub(super) fn lower<S: AlternativeSelection>(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    selection: &S,
) -> Option<Lowered> {
    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(selection.alternative(node)?)?;
    match forest_node.key.symbol {
        ForestSymbol::Lexical(_) => {
            return lower_lexical(
                grammar,
                forest_node.key.lexical_value()?,
                alternative.surface,
            );
        }
        ForestSymbol::Intermediate { .. } => return None,
        ForestSymbol::Nonterminal(_) => {}
    }
    let rule = alternative.rule?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut child_nodes = Vec::new();
    selected_rule_children(forest, *intermediate, selection, &mut child_nodes)?;
    let children = child_nodes
        .iter()
        .map(|&child| lower(grammar, forest, child, selection))
        .collect::<Option<Vec<_>>>()?;
    match grammar.impls.get(rule.index())? {
        RuleImpl::Handwritten(tag) => {
            let mut children = children;
            lower_rule(*tag, &mut children)
        }
        RuleImpl::Generated(generated) => lower_generated_construction(*generated, children),
        RuleImpl::GeneratedAux(generated) => lower_generated_aux(*generated, children),
    }
}

fn lower_generated_aux(
    rule: super::rules::GeneratedAuxRuleRef,
    mut children: Vec<Lowered>,
) -> Option<Lowered> {
    use super::rules::GeneratedAuxRuleRef as R;
    match rule {
        R::Transparent => {
            let [value]: [Lowered; 1] = children.try_into().ok()?;
            Some(value)
        }
        R::ElementStruct {
            group,
            element,
            present_fields,
        } => {
            let element = group.element_data.get(element)?;
            let mut children = children.drain(..);
            let mut fields = Vec::with_capacity(element.fields.len());
            for (index, field) in element.fields.iter().enumerate() {
                fields.push(if present_fields & (1_u64 << index) == 0 {
                    erased_absent(field.kind)?
                } else {
                    erased_field(field.kind, children.next()?)?
                });
            }
            if children.next().is_some() {
                return None;
            }
            let builder = *element.erased_builders.first()?;
            Some(Lowered::Generated(GeneratedValue::Typed(
                builder(fields).ok()?,
            )))
        }
        R::ElementVariant {
            group,
            element,
            variant,
        } => {
            let element = group.element_data.get(element)?;
            let declaration = element.variants.get(variant)?;
            let [child]: [Lowered; 1] = children.try_into().ok()?;
            let payload = erased_field(declaration.payload, child)?;
            Some(Lowered::Generated(GeneratedValue::Typed(
                element.erased_builders.get(variant)?(vec![payload]).ok()?,
            )))
        }
        R::SequenceSeed { group, element } => {
            group.element_data.get(element)?;
            let [Lowered::Generated(GeneratedValue::Typed(value))]: [Lowered; 1] =
                children.try_into().ok()?
            else {
                return None;
            };
            Some(Lowered::Generated(GeneratedValue::Sequence(vec![value])))
        }
        R::SequenceExtend { group, element } => {
            group.element_data.get(element)?;
            let [
                Lowered::Generated(GeneratedValue::Sequence(mut values)),
                Lowered::Generated(GeneratedValue::Typed(value)),
            ]: [Lowered; 2] = children.try_into().ok()?
            else {
                return None;
            };
            values.push(value);
            Some(Lowered::Generated(GeneratedValue::Sequence(values)))
        }
    }
}

fn lower_generated_construction(
    rule: super::rules::GeneratedRuleRef,
    children: Vec<Lowered>,
) -> Option<Lowered> {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::FieldKindData;

    let construction = rule.group.constructions.get(rule.construction)?;
    let form = construction.forms.get(rule.form)?;
    let mut children = children.into_iter();
    let preposition = match rule.context {
        super::rules::GeneratedRuleContext::Value => None,
        super::rules::GeneratedRuleContext::SharedPreposition => {
            let Lowered::Preposition(preposition) = children.next()? else {
                return None;
            };
            Some(preposition)
        }
    };
    let mut fields = std::iter::repeat_with(|| None)
        .take(construction.fields.len())
        .collect::<Vec<_>>();
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        if matches!(atom, AtomData::Literal(_)) {
            children.next()?;
            continue;
        }
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) => *path,
            AtomData::Literal(_) => unreachable!(),
        };
        let field_index = construction
            .fields
            .iter()
            .position(|field| field.name == path)?;
        let field = construction.fields[field_index];
        let value = if let FieldKindData::Sequence { element } = field.kind {
            let element = rule
                .group
                .element_data
                .iter()
                .find(|candidate| candidate.name == element)?;
            let values = if rule.sequence_atoms & (1_u64 << atom_index) == 0 {
                Vec::new()
            } else {
                let Lowered::Generated(GeneratedValue::Sequence(values)) = children.next()? else {
                    return None;
                };
                values
            };
            element.erased_sequence_builder?(values).ok()?
        } else {
            erased_field(field.kind, children.next()?)?
        };
        fields[field_index] = Some(value);
    }
    if children.next().is_some() {
        return None;
    }
    let fields = fields.into_iter().collect::<Option<Vec<_>>>()?;
    let value = construction.erased_builder?(fields).ok()?;
    let value = match construction.erased_projector {
        Some(projector) => projector(value).ok()?,
        None => value,
    };
    let projected = project_generated_category(construction, value)?;
    match (rule.context, preposition, projected) {
        (super::rules::GeneratedRuleContext::Value, None, projected) => Some(projected),
        (
            super::rules::GeneratedRuleContext::SharedPreposition,
            Some(preposition),
            Lowered::NounPhrase(object),
        ) => Some(Lowered::PrepositionalPhrase(PrepositionalPhrase::simple(
            preposition,
            Phrase::NounPhrase(Box::new(object)),
        ))),
        _ => None,
    }
}

fn project_generated_category(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    value: deckmaste_construction_compiler::runtime::ErasedValue,
) -> Option<Lowered> {
    if construction.category == "NounPhrase" {
        let value = value.downcast::<NounPhrase>().ok()?;
        return Some(Lowered::NounPhrase(*value));
    }
    #[cfg(test)]
    match construction.id {
        "probe_word" => {
            value
                .downcast::<crate::constructions::probe::ProbeWordNode>()
                .ok()?;
            return Some(Lowered::Generated(GeneratedValue::Typed(Box::new(
                crate::constructions::probe::ProbeItem,
            ))));
        }
        "probe_pick" => {
            value
                .downcast::<crate::constructions::probe::ProbePickNode>()
                .ok()?;
            return Some(Lowered::Generated(GeneratedValue::Typed(Box::new(
                crate::constructions::probe::ProbeRoot,
            ))));
        }
        "probe_pick_shadow" => {
            value
                .downcast::<crate::constructions::probe::ProbeShadowNode>()
                .ok()?;
            return Some(Lowered::Generated(GeneratedValue::Typed(Box::new(
                crate::constructions::probe::ProbeRoot,
            ))));
        }
        "law_letter" => {
            value
                .downcast::<crate::constructions::law::LawLetterNode>()
                .ok()?;
            return Some(Lowered::Generated(GeneratedValue::Typed(Box::new(
                crate::constructions::law::LawItem,
            ))));
        }
        _ => {}
    }
    Some(Lowered::Generated(GeneratedValue::Typed(value)))
}

fn erased_field(
    kind: deckmaste_construction_compiler::runtime::FieldKindData,
    value: Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match kind {
        K::Subtree { category, boxed } => erased_subtree(category, boxed, value),
        K::Scalar {
            codec: "Conjunction" | "NounPhraseConjunction",
        } => {
            let Lowered::Conjunction(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Scalar { codec: "Comma" } | K::SurfaceScalar { codec: "Comma" } => {
            Some(Box::new(crate::features::Comma::Present))
        }
        K::Optional { inner } => erased_optional(*inner, &value),
        K::Scalar { .. } | K::SurfaceScalar { .. } | K::Sequence { .. } => None,
    }
}

fn erased_absent(
    kind: deckmaste_construction_compiler::runtime::FieldKindData,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match kind {
        K::Scalar { codec: "Comma" } | K::SurfaceScalar { codec: "Comma" } => {
            Some(Box::new(crate::features::Comma::Absent))
        }
        K::Optional { inner } => erased_optional_absent(*inner),
        K::Subtree { .. } | K::Scalar { .. } | K::SurfaceScalar { .. } | K::Sequence { .. } => None,
    }
}

fn erased_subtree(
    category: &'static str,
    boxed: bool,
    value: Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    macro_rules! typed {
        ($variant:ident, $value:expr) => {{
            let Lowered::$variant(value) = $value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }};
    }
    match category {
        "NounPhrase" => typed!(NounPhrase, value),
        "NominalPhrase" => typed!(Nominal, value),
        "Determiner" => typed!(Determiner, value),
        "AdjectivePhrase" => typed!(AdjectivePhrase, value),
        "CoordinatedAdjectivePhrase" => {
            let Lowered::CoordinatedModifier(value) = value else {
                return None;
            };
            let value = clause::coordinated_modifier_as_adjectives(value)?;
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "PrepositionalPhrase" => typed!(PrepositionalPhrase, value),
        "InfinitiveClause" => typed!(InfinitiveClause, value),
        "RelativeClause" => typed!(RelativeClause, value),
        "Quantity" => typed!(Quantity, value),
        "DevotionColors" => typed!(DevotionColors, value),
        "PowerToughness" => typed!(PowerToughness, value),
        "TransitivePredicate" => {
            let Lowered::VerbPhrase(value) = value else {
                return None;
            };
            let value = clause::finish_reduced_recipient_passive(value)?;
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "IndependentClause" => {
            let Lowered::Clause(Clause::Independent(value)) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "KeywordArgument" => {
            let Lowered::PredicatedArgument(value) = value else {
                return None;
            };
            let value = KeywordArgument::Predicated(value);
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        _ => {
            let Lowered::Generated(GeneratedValue::Typed(value)) = value else {
                return None;
            };
            Some(value)
        }
    }
}

fn erased_optional(
    inner: deckmaste_construction_compiler::runtime::FieldKindData,
    value: &Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match inner {
        K::Scalar {
            codec: "Conjunction" | "NounPhraseConjunction",
        } => {
            let Lowered::Conjunction(value) = value else {
                return None;
            };
            Some(Box::new(Some(*value)))
        }
        K::Scalar { codec: "Comma" } => Some(Box::new(Some(crate::features::Comma::Present))),
        _ => None,
    }
}

fn erased_optional_absent(
    inner: deckmaste_construction_compiler::runtime::FieldKindData,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match inner {
        K::Scalar {
            codec: "Conjunction" | "NounPhraseConjunction",
        } => Some(Box::new(None::<crate::features::Conjunction>)),
        K::Scalar { codec: "Comma" } => Some(Box::new(None::<crate::features::Comma>)),
        _ => None,
    }
}

pub(super) fn selected_rule_children<S: AlternativeSelection>(
    forest: &EnglishForest,
    intermediate: NodeId,
    selection: &S,
    children: &mut Vec<NodeId>,
) -> Option<()> {
    let node = forest.node(intermediate);
    if !matches!(node.key.symbol, ForestSymbol::Intermediate { .. }) {
        return None;
    }
    let alternative = node
        .alternatives
        .get(selection.alternative(intermediate)?)?;
    match alternative.children.as_slice() {
        [child] => children.push(*child),
        [previous, child] => {
            selected_rule_children(forest, *previous, selection, children)?;
            children.push(*child);
        }
        _ => return None,
    }
    Some(())
}

pub(super) fn lower_lexical(
    grammar: &EnglishGrammar<'_, '_>,
    meaning: &MeaningKey,
    surface: EnglishSurfaceWitness,
) -> Option<Lowered> {
    Some(match meaning {
        MeaningKey::Literal(_) | MeaningKey::Punctuation(_) => Lowered::Ignored,
        MeaningKey::Number(number) => Lowered::Number(*number),
        MeaningKey::Quantity(quantity) => Lowered::Quantity(*quantity),
        MeaningKey::Determiner(determiner) => Lowered::Determiner(determiner.clone()),
        MeaningKey::Noun(noun) => Lowered::Noun(noun.clone()),
        MeaningKey::Adjective(adjective) => Lowered::Adjective(adjective.clone()),
        MeaningKey::Adverb(adverb) => Lowered::Adverb(*adverb),
        MeaningKey::VerbParticle(particle) => Lowered::VerbParticle(*particle),
        MeaningKey::CoinResult(side) => Lowered::CoinResult(*side),
        MeaningKey::Frequency(frequency) => Lowered::Frequency(*frequency),
        MeaningKey::Pronoun(pronoun) => Lowered::Pronoun(*pronoun),
        MeaningKey::Auxiliary(auxiliary) => {
            Lowered::Auxiliary(auxiliary.with_contraction(surface.contraction()?))
        }
        MeaningKey::SubjectAuxiliary(subject_auxiliary) => {
            if surface.contraction()? != crate::features::Contraction::Contracted {
                return None;
            }
            let subject = match subject_auxiliary.subject {
                ContractedSubjectKey::Pronoun(pronoun) => NounPhrase::Pronoun {
                    pronoun,
                    case: PronounCase::Subject,
                },
                ContractedSubjectKey::Demonstrative(demonstrative) => {
                    NounPhrase::Demonstrative(demonstrative)
                }
            };
            Lowered::SubjectAuxiliary(ContractedSubjectAuxiliary {
                subject: Subject(subject),
                auxiliary: subject_auxiliary
                    .auxiliary
                    .with_contraction(crate::features::Contraction::Full),
            })
        }
        MeaningKey::Verb(verb) => Lowered::Verb(verb.clone()),
        MeaningKey::Catalog(atom) => Lowered::Catalog(atom.clone()),
        MeaningKey::OracleSymbol(symbol) => Lowered::OracleSymbol(symbol.clone()),
        MeaningKey::SymbolSequence(symbols) => Lowered::SymbolSequence(symbols.clone()),
        MeaningKey::QuotedAbility(span) => {
            let interior = span.text(grammar.source)?;
            Lowered::Phrase(Phrase::QuotedAbility(Box::new(
                ability::parse_quoted_ability_fragment(
                    interior,
                    grammar.catalogs,
                    &grammar.self_reference,
                ),
            )))
        }
        MeaningKey::PowerToughness(power_toughness) => Lowered::PowerToughness(*power_toughness),
        MeaningKey::Conjunction(conjunction) => Lowered::Conjunction(*conjunction),
        MeaningKey::Subordinator(subordinator) => Lowered::Subordinator(*subordinator),
        MeaningKey::RelativeMarker(marker) => Lowered::RelativeMarker(*marker),
        MeaningKey::Existential(key) => {
            Lowered::Existential(ExistentialForm::new(key.verb_slot, surface.contraction()?)?)
        }
        MeaningKey::ThisCard(form) => Lowered::ThisCard(*form),
        MeaningKey::Preposition(preposition) => Lowered::Preposition(*preposition),
        MeaningKey::NegatedModifier(key) => Lowered::NominalModifier(key.build()),
        MeaningKey::Opaque(key) => {
            let opaque = OpaqueLexeme::new(key.span.text(grammar.source)?);
            match key.slot {
                OpacitySlot::Noun(form) => {
                    let noun = Noun::Opaque(opaque);
                    Lowered::Noun(match form {
                        NounForm::Singular => NounInstance::Singular(noun),
                        NounForm::Plural => NounInstance::Plural(noun),
                        NounForm::Mass => NounInstance::Mass(noun),
                    })
                }
            }
        }
    })
}

#[allow(
    clippy::too_many_lines,
    reason = "the rule-tag dispatch is intentionally one flat match over every tag"
)]
pub(super) fn lower_rule(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => lower_quantity_or_determiner(tag, children),
        RuleTag::FrequencyPhrase => take(children, 0),
        RuleTag::FrequencyPhraseAdverb => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            if adverb != Vocab::Only {
                return None;
            }
            let Lowered::Frequency(freq) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Frequency(crate::syntax::FrequencyPhrase {
                bound: crate::syntax::FrequencyBound::NoMoreThan,
                count: freq.count,
            }))
        }
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun
        | RuleTag::PossessiveNounAdjective => lower_possessive_noun_phrase(tag, children),
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::AdjectivePhraseDegreeMeasure
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalCombatStepName
        | RuleTag::NominalNegatedModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalInfinitive
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalKeywordSymbolArgument
        | RuleTag::PredicatedQualityFrom
        | RuleTag::PredicatedArgumentFromSingle
        | RuleTag::PredicatedArgumentFromExtend
        | RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::PredicatedQualityBare
        | RuleTag::PredicatedArgumentBareSingle
        | RuleTag::PredicatedArgumentBareExtend
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument
        | RuleTag::NominalPowerToughnessComplement
        | RuleTag::NominalRelative
        | RuleTag::RulesObjectNominalBase
        | RuleTag::RulesObjectFollowupNominalRelative
        | RuleTag::RulesObjectFollowupNominalPrepositional
        | RuleTag::NominalReducedRecipientPassive
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalPostpositiveAdjectiveConjoinedPrepositional
        | RuleTag::NominalPostpositiveAdjectiveConjoined
        | RuleTag::NominalPostpositiveAdjectiveAsyndetic
        | RuleTag::NominalPostpositiveAdjectiveOxford
        | RuleTag::NominalComparison
        | RuleTag::NominalDevotion
        | RuleTag::DevotionColorSingle
        | RuleTag::DevotionColorPair
        | RuleTag::NominalTimesClause
        | RuleTag::ModifierConjunctAdjective
        | RuleTag::ModifierConjunctNoun
        | RuleTag::ModifierConjunctNegated
        | RuleTag::ModifierListSingle
        | RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford
        | RuleTag::NominalCoordinatedModifier => lower_nominal(tag, children),
        RuleTag::NounPhraseNominal
        | RuleTag::RulesObjectNounPhrase
        | RuleTag::NounPhraseSetExceptionBare
        | RuleTag::NounPhraseSetExceptionFor
        | RuleTag::ReducedRecipientPassiveTheme
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhraseDemonstrative
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseEachPartitive
        | RuleTag::NounPhraseAnyNumberOf
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalObject => lower_phrase(tag, children),
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhrasePassiveSharedDeterminerPrepositional
        | RuleTag::VerbPhraseExceptBy
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectDistributiveEach
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateGerundBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::CopularRemainderNegated
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::ClauseVariableValueConstraint
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeSubjectDistributiveEach
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionRun
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective
        | RuleTag::Sentence => clause::lower_clause(tag, children),
        RuleTag::NounOpaque => opacity::lower_opacity(tag, children),
    }
}

pub(super) fn lower_possessive_noun_phrase(
    tag: RuleTag,
    children: &mut [Lowered],
) -> Option<Lowered> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::PossessiveNominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::PossessiveNounDetermined => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::PossessiveNominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::PossessiveNominal(nominal))
        }
        RuleTag::DeterminerPossessiveNoun => {
            let Lowered::PossessiveNominal(possessor) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::Nominal(possessor))),
            )))
        }
        RuleTag::PossessiveNounAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            let Lowered::PossessiveNominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            if introduces_proper_name(&adjective) {
                open_name_interior(&mut nominal);
            }
            nominal.modifiers.insert(
                0,
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase: adjective,
                },
            );
            Some(Lowered::PossessiveNominal(nominal))
        }
        _ => None,
    }
}

pub(super) fn lower_quantity_or_determiner(
    tag: RuleTag,
    children: &mut [Lowered],
) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact => {
            let Lowered::Number(number) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::Exact(number)))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(quantity))
        }
        RuleTag::QuantityUpTo => {
            let Lowered::Number(number) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::UpTo(quantity_value(number))))
        }
        RuleTag::QuantityThatMany => Some(Lowered::Quantity(Quantity::ThatMany)),
        RuleTag::QuantityThatMuch => Some(Lowered::Quantity(Quantity::ThatMuch)),
        RuleTag::DeterminerClosed => take(children, 0),
        RuleTag::DeterminerTarget => Some(Lowered::Determiner(Determiner::Target(None))),
        RuleTag::DeterminerQuantifiedTarget => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Target(Some(quantity))))
        }
        RuleTag::DeterminerQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Quantity(quantity)))
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        _ => None,
    }
}

/// Whether an adjectival modifier is the `named` participle that introduces a
/// proper name (e.g. `creature named Storm Crow`).
pub(super) fn introduces_proper_name(adjective: &AdjectivePhrase) -> bool {
    matches!(
        adjective.head,
        Adjective::Participle(_, Verb::Word(Vocab::Name))
    )
}

/// Re-labels keyword-ability catalog atoms inside a proper name so they are
/// carried as case-preserved opaque tokens. Inside `named Storm Crow`, `Storm`
/// is the first word of the name, not the keyword ability, so it must keep its
/// matched source spelling instead of being lowercased by the keyword-atom noun
/// case policy. Everything to the right of the `named` participle — the
/// accumulated modifiers and the head — is name interior at the point this
/// runs.
///
/// Only keyword-ability atoms are re-labelled: their spelling is the matched
/// surface, so opacifying them is casing-faithful, whereas subtype/type atoms
/// already render case- and inflection-faithfully (their spelling is a
/// canonical singular). This also leaves the adjectival `differently named
/// <type>` reading — which shares this flat shape but carries no keyword atom —
/// untouched.
pub(super) fn open_name_interior(nominal: &mut NominalPhrase) {
    detach_keyword_noun(&mut nominal.head);
    for modifier in &mut nominal.modifiers {
        if let NominalModifier::Noun { noun, .. } = modifier {
            detach_keyword_noun(noun);
        }
    }
}

pub(super) fn detach_keyword_noun(noun: &mut NounInstance) {
    let inner = match noun {
        NounInstance::Singular(inner) | NounInstance::Plural(inner) | NounInstance::Mass(inner) => {
            inner
        }
    };
    if let Noun::Catalog(atom) = inner
        && atom.kind == CatalogKind::KeywordAbility
    {
        *inner = Noun::Opaque(OpaqueLexeme::new(atom.spelling()));
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "lowering nominals is intentionally long"
)]
pub(super) fn lower_nominal(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun | RuleTag::RulesObjectNominalBase => take(children, 0),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            let orientation = match tag {
                RuleTag::AdjectivePhraseFaceUp => CardOrientation::FaceUp,
                RuleTag::AdjectivePhraseFaceDown => CardOrientation::FaceDown,
                _ => return None,
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head: Adjective::CardOrientation(orientation),
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhrase => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhraseComparison => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: None,
                head,
                complements: vec![crate::syntax::AdjectiveComplement::Comparison(comparison)],
            }))
        }
        RuleTag::AdjectivePhraseDegreeMeasure => {
            let Lowered::Number(number) = take(children, 0)? else {
                return None;
            };
            let Lowered::Adjective(head) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                degree: Some(number),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::ComparisonStandard => {
            let standard = match take(children, 0)? {
                Lowered::Clause(clause) => Phrase::Clause(Box::new(clause)),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                Lowered::AdjectivePhrase(adjective) => Phrase::AdjectivePhrase(Box::new(adjective)),
                _ => return None,
            };
            Some(Lowered::Phrase(standard))
        }
        RuleTag::ComparisonThan | RuleTag::ComparisonThanOrEqualTo => {
            let standard_index = if tag == RuleTag::ComparisonThan { 1 } else { 2 };
            let Lowered::Phrase(standard) = take(children, standard_index)? else {
                return None;
            };
            let marker = if tag == RuleTag::ComparisonThan {
                ComparisonMarker::Than
            } else {
                ComparisonMarker::ThanOrEqualTo
            };
            Some(Lowered::ComparisonComplement(ComparisonComplement {
                marker,
                standard: Box::new(standard),
            }))
        }
        RuleTag::NominalNoun => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::NominalCombatStepName => {
            let Lowered::Noun(participants) = take(children, 1)? else {
                return None;
            };
            let Lowered::Noun(head) = take(children, 2)? else {
                return None;
            };
            // Redundant by design (§2.4): the literal-token slots are the
            // defense. If this guard ever fires, the slots have a bug — it
            // is not the safety mechanism.
            if !matches!(
                participants,
                NounInstance::Plural(
                    Noun::Word(_) | Noun::Agentive(Verb::Word(Vocab::Attack | Vocab::Block))
                )
            ) {
                return None;
            }
            if !matches!(head, NounInstance::Singular(Noun::Word(_))) {
                return None;
            }
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: vec![NominalModifier::CombatStepName { participants }],
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::NominalAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            if introduces_proper_name(&adjective) {
                open_name_interior(&mut nominal);
            }
            nominal.modifiers.insert(
                0,
                NominalModifier::Adjective {
                    polarity: Polarity::Positive,
                    phrase: adjective,
                },
            );
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalNounModifier => {
            let Lowered::Noun(noun) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.modifiers.insert(
                0,
                NominalModifier::Noun {
                    polarity: Polarity::Positive,
                    noun,
                },
            );
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalNegatedModifier => {
            let Lowered::NominalModifier(modifier) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.modifiers.insert(0, modifier);
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityModifier => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPowerToughnessModifier => {
            let Lowered::PowerToughness(modifier) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::PowerToughness(modifier));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalDeterminer => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPrepositional | RuleTag::RulesObjectFollowupNominalPrepositional => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(preposition) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Prepositional(preposition));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalInfinitive => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::InfinitiveClause(infinitive) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Infinitive(clause::finish_infinitive(
                    infinitive,
                )?));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityComplement => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::Quantity(quantity) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::PredicatedQualityFrom => {
            let quality = match take(children, 1)? {
                Lowered::Adjective(Adjective::Color(color)) => Phrase::ColorWord(color),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                _ => return None,
            };
            Some(Lowered::PredicatedQuality(PredicatedQuality {
                preposition: Some(Preposition::From),
                quality,
            }))
        }
        RuleTag::PredicatedQualityBare => {
            let quality = match take(children, 0)? {
                Lowered::Adjective(Adjective::Color(color)) => Phrase::ColorWord(color),
                Lowered::AdjectivePhrase(adjective) => Phrase::AdjectivePhrase(Box::new(adjective)),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                _ => return None,
            };
            Some(Lowered::PredicatedQuality(PredicatedQuality {
                preposition: None,
                quality,
            }))
        }
        RuleTag::PredicatedArgumentFromSingle | RuleTag::PredicatedArgumentBareSingle => {
            let Lowered::PredicatedQuality(quality) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::PredicatedArgument(PredicatedArgument {
                qualities: vec![quality],
            }))
        }
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend => {
            let Lowered::PredicatedArgument(mut argument) = take(children, 0)? else {
                return None;
            };
            let Lowered::PredicatedQuality(quality) = take(children, 2)? else {
                return None;
            };
            argument.qualities.push(quality);
            Some(Lowered::PredicatedArgument(argument))
        }
        RuleTag::NominalKeywordPredicatedArgument
        | RuleTag::NominalKeywordAtomCarriedPredicatedArgument => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::PredicatedArgument(argument) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::KeywordArgument(
                    KeywordArgument::Predicated(argument),
                )],
            }))
        }
        RuleTag::NominalKeywordSymbolArgument => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let symbols = match take(children, 1)? {
                Lowered::OracleSymbol(symbol) => vec![symbol],
                Lowered::SymbolSequence(symbols) => symbols,
                _ => return None,
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::KeywordArgument(KeywordArgument::Costed(
                    KeywordCost::Symbols(symbols),
                ))],
            }))
        }
        RuleTag::NominalPowerToughnessComplement => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PowerToughness(power_toughness) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::PowerToughness(power_toughness));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalRelative | RuleTag::RulesObjectFollowupNominalRelative => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::RelativeClause(relative) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Relative(relative));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalReducedRecipientPassive => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::ReducedRecipientPassive(
                    clause::finish_reduced_recipient_passive(predicate)?,
                ));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPostpositiveAdjective => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(adjective) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Adjective(adjective));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPostpositiveAdjectiveConjoinedPrepositional => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                return None;
            };
            let conjunction = noun_phrase_conjunction(conjunction)?;
            let Lowered::AdjectivePhrase(mut adjective) = take(children, 2)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(preposition) = take(children, 3)? else {
                return None;
            };
            if !matches!(adjective.head, Adjective::Participle(Tense::Past, _))
                || preposition.head().preposition != Preposition::By
            {
                return None;
            }
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::Prepositional(
                    preposition,
                ));
            let previous = nominal.complements.pop()?;
            let coordinated = match previous {
                NominalComplement::Adjective(first) => crate::syntax::CoordinatedAdjectivePhrase {
                    first: Box::new(first),
                    rest: vec![crate::syntax::AdjectivePhraseCoordination {
                        conjunction: Some(conjunction),
                        phrase: adjective,
                    }],
                },
                NominalComplement::CoordinatedAdjective(mut coordinated) => {
                    let previous_conjunction = coordinated
                        .rest
                        .iter()
                        .rev()
                        .find_map(|member| member.conjunction);
                    if previous_conjunction.is_some_and(|previous| previous != conjunction) {
                        return None;
                    }
                    coordinated
                        .rest
                        .push(crate::syntax::AdjectivePhraseCoordination {
                            conjunction: Some(conjunction),
                            phrase: adjective,
                        });
                    coordinated
                }
                _ => return None,
            };
            nominal
                .complements
                .push(NominalComplement::CoordinatedAdjective(coordinated));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPostpositiveAdjectiveConjoined
        | RuleTag::NominalPostpositiveAdjectiveAsyndetic
        | RuleTag::NominalPostpositiveAdjectiveOxford => {
            let (conjunction_index, adjective_index) = match tag {
                RuleTag::NominalPostpositiveAdjectiveConjoined => (Some(1), 2),
                RuleTag::NominalPostpositiveAdjectiveAsyndetic => (None, 2),
                RuleTag::NominalPostpositiveAdjectiveOxford => (Some(2), 3),
                _ => unreachable!("matched postpositive coordination tag"),
            };
            let conjunction = match conjunction_index {
                Some(index) => {
                    let Lowered::Conjunction(conjunction) = take(children, index)? else {
                        return None;
                    };
                    Some(noun_phrase_conjunction(conjunction)?)
                }
                None => None,
            };
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(adjective) = take(children, adjective_index)? else {
                return None;
            };
            let previous = nominal.complements.pop()?;
            let coordinated = match previous {
                NominalComplement::Adjective(first) => crate::syntax::CoordinatedAdjectivePhrase {
                    first: Box::new(first),
                    rest: vec![crate::syntax::AdjectivePhraseCoordination {
                        conjunction,
                        phrase: adjective,
                    }],
                },
                NominalComplement::CoordinatedAdjective(mut coordinated) => {
                    let previous_conjunction = coordinated
                        .rest
                        .iter()
                        .rev()
                        .find_map(|member| member.conjunction);
                    if matches!(
                        (previous_conjunction, conjunction),
                        (Some(previous), Some(next)) if previous != next
                    ) {
                        return None;
                    }
                    coordinated
                        .rest
                        .push(crate::syntax::AdjectivePhraseCoordination {
                            conjunction,
                            phrase: adjective,
                        });
                    coordinated
                }
                _ => return None,
            };
            nominal
                .complements
                .push(NominalComplement::CoordinatedAdjective(coordinated));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalComparison => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            let adjective = nominal.modifiers.iter_mut().rev().find_map(|modifier| {
                let NominalModifier::Adjective {
                    phrase: adjective, ..
                } = modifier
                else {
                    return None;
                };
                (matches!(
                    adjective_comparison_state(&adjective.head),
                    AdjectiveComparisonState::Pending(_)
                ) && !adjective.complements.iter().any(|complement| {
                    matches!(
                        complement,
                        crate::syntax::AdjectiveComplement::Comparison(_)
                            | crate::syntax::AdjectiveComplement::PostnominalComparison(_)
                    )
                }))
                .then_some(adjective)
            })?;
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::PostnominalComparison(
                    comparison,
                ));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::DevotionColorSingle => {
            let Lowered::Adjective(Adjective::Color(color)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::DevotionColors(
                crate::syntax::DevotionColors::Color(color),
            ))
        }
        RuleTag::DevotionColorPair => {
            let Lowered::Adjective(Adjective::Color(first)) = take(children, 0)? else {
                return None;
            };
            let Lowered::Adjective(Adjective::Color(second)) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::DevotionColors(
                crate::syntax::DevotionColors::Pair(first, second),
            ))
        }
        RuleTag::NominalDevotion => {
            let Lowered::Catalog(atom) = take(children, 0)? else {
                return None;
            };
            let Lowered::DevotionColors(colors) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head: NounInstance::Singular(Noun::Catalog(atom)),
                complements: vec![NominalComplement::Devotion(colors)],
            }))
        }
        RuleTag::NominalTimesClause => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::EventClause(Box::new(clause))],
            }))
        }
        RuleTag::ModifierConjunctAdjective => {
            let Lowered::AdjectivePhrase(phrase) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase,
            }))
        }
        RuleTag::ModifierConjunctNoun => {
            let Lowered::Noun(noun) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(NominalModifier::Noun {
                polarity: Polarity::Positive,
                noun,
            }))
        }
        RuleTag::ModifierConjunctNegated => {
            // The `non-` lexeme already lowers to a negated `NominalModifier`.
            let Lowered::NominalModifier(modifier) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NominalModifier(modifier))
        }
        RuleTag::ModifierListSingle => {
            let Lowered::NominalModifier(first) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::CoordinatedModifier(
                crate::syntax::CoordinatedModifier {
                    first: Box::new(first),
                    rest: Vec::new(),
                },
            ))
        }
        RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford => {
            let Lowered::CoordinatedModifier(mut coordinated) = take(children, 0)? else {
                return None;
            };
            let (conjunction, modifier_index) = match tag {
                RuleTag::ModifierListComma => (None, 2),
                RuleTag::CoordinatedModifierConjoined => (Some(1), 2),
                RuleTag::CoordinatedModifierOxford => (Some(2), 3),
                _ => return None,
            };
            let conjunction = match conjunction {
                Some(index) => {
                    let Lowered::Conjunction(conjunction) = take(children, index)? else {
                        return None;
                    };
                    Some(noun_phrase_conjunction(conjunction)?)
                }
                None => None,
            };
            let Lowered::NominalModifier(modifier) = take(children, modifier_index)? else {
                return None;
            };
            coordinated.rest.push(crate::syntax::ModifierCoordination {
                conjunction,
                modifier,
            });
            Some(Lowered::CoordinatedModifier(coordinated))
        }
        RuleTag::NominalCoordinatedModifier => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Coordinated(coordinated));
            Some(Lowered::Nominal(nominal))
        }
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "lowering noun phrases is intentionally long"
)]
pub(super) fn lower_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounPhraseSetExceptionBare | RuleTag::NounPhraseSetExceptionFor => {
            let (marker, comma, excluded_index) = match (tag, children.len()) {
                (RuleTag::NounPhraseSetExceptionBare, 3) => (SetExceptionMarker::Bare, false, 2),
                (RuleTag::NounPhraseSetExceptionBare, 4) => (SetExceptionMarker::Bare, true, 3),
                (RuleTag::NounPhraseSetExceptionFor, 4) => (SetExceptionMarker::For, false, 3),
                (RuleTag::NounPhraseSetExceptionFor, 5) => (SetExceptionMarker::For, true, 4),
                _ => return None,
            };
            let Lowered::NounPhrase(included) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(excluded) = take(children, excluded_index)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::SetException(
                SetExceptionNounPhrase {
                    included: Box::new(included),
                    marker,
                    comma: crate::features::Comma::from(comma),
                    excluded: Box::new(excluded),
                },
            )))
        }
        RuleTag::NounPhraseNominal
        | RuleTag::RulesObjectNounPhrase
        | RuleTag::ReducedRecipientPassiveTheme => {
            let Lowered::Nominal(nominal) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Nominal(nominal)))
        }
        RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal => {
            let Lowered::Pronoun(pronoun) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Pronoun {
                pronoun: pronoun.pronoun,
                case: pronoun.case,
            }))
        }
        RuleTag::NounPhraseQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Quantity(quantity)))
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::ThisCard(form)))
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        RuleTag::NounPhraseDemonstrative => {
            let Lowered::Determiner(crate::syntax::Determiner::Demonstrative(demonstrative)) =
                take(children, 0)?
            else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Demonstrative(
                demonstrative,
            )))
        }
        RuleTag::NounPhrasePartitive => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Preposition(Preposition::Of) = take(children, 1)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Partitive(
                crate::syntax::PartitiveNounPhrase {
                    head: crate::syntax::PartitiveHead::Quantity(quantity),
                    whole: Box::new(whole),
                },
            )))
        }
        RuleTag::NounPhraseEachPartitive => {
            let Lowered::Determiner(crate::syntax::Determiner::Each) = take(children, 0)? else {
                return None;
            };
            let Lowered::Preposition(Preposition::Of) = take(children, 1)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Partitive(
                crate::syntax::PartitiveNounPhrase {
                    head: crate::syntax::PartitiveHead::Each,
                    whole: Box::new(whole),
                },
            )))
        }
        RuleTag::NounPhraseAnyNumberOf => {
            // Lower to the byte-identical ordinary nominal shape: the two
            // analyses (this notional-plural reduce vs. the formal-singular
            // `NounPhraseNominal` path) must differ only in parse features,
            // never in stored or rendered structure [`anof` round].
            let Lowered::Determiner(determiner @ crate::syntax::Determiner::Any) =
                take(children, 0)?
            else {
                return None;
            };
            let Lowered::Noun(head @ NounInstance::Singular(Noun::Word(Vocab::Number))) =
                take(children, 1)?
            else {
                return None;
            };
            let Lowered::Preposition(preposition @ Preposition::Of) = take(children, 2)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 3)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Nominal(NominalPhrase {
                determiner: Some(determiner),
                modifiers: Vec::new(),
                head,
                complements: vec![NominalComplement::Prepositional(
                    PrepositionalPhrase::simple(preposition, Phrase::NounPhrase(Box::new(whole))),
                )],
            })))
        }
        RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => lower_arithmetic_phrase(tag, children),
        RuleTag::PrepositionalObject => {
            let phrase = match take(children, 0)? {
                Lowered::NounPhrase(object) => Phrase::NounPhrase(Box::new(object)),
                Lowered::PrepositionalPhrase(object) => {
                    Phrase::PrepositionalPhrase(Box::new(object))
                }
                Lowered::GerundClause(object) => Phrase::Clause(Box::new(Clause::Dependent(
                    crate::syntax::DependentClause::Gerund(object),
                ))),
                Lowered::Adverb(object) => Phrase::Adverb(object),
                _ => return None,
            };
            Some(Lowered::Phrase(phrase))
        }
        RuleTag::PrepositionalPhrase => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Phrase(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase::simple(
                preposition,
                object,
            )))
        }
        RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated => {
            let Lowered::PrepositionalPhrase(first) = take(children, 0)? else {
                return None;
            };
            // The serial comma is not recorded — the renderer derives it from
            // member count. Only the connective is carried, since which of
            // `and`/`or`/`and-or` closes the list is not derivable.
            let (conjunction, next_index) = if matches!(
                tag,
                RuleTag::PrepositionalPhraseListPair | RuleTag::PrepositionalPhraseListComma
            ) {
                // An asyndetic member of the run: `from Vampires,`.
                (None, 2)
            } else if children.len() == 4 {
                // The Oxford close: `…, and from Zombies`.
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                (Some(noun_phrase_conjunction(conjunction)?), 3)
            } else {
                // The bare two-member form: `from blue and from black`.
                let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                    return None;
                };
                (Some(noun_phrase_conjunction(conjunction)?), 2)
            };
            let Lowered::PrepositionalPhrase(next) = take(children, next_index)? else {
                return None;
            };
            // Only a simple phrase can join as a member; a nested coordination
            // would flatten two different bracketings into one shape.
            let PrepositionalPhrase::Simple(next) = next else {
                return None;
            };
            let coordination = crate::syntax::PrepositionalPhraseCoordination {
                conjunction,
                phrase: next,
            };
            Some(Lowered::PrepositionalPhrase(match first {
                PrepositionalPhrase::Simple(first) => PrepositionalPhrase::Coordinated(
                    crate::syntax::CoordinatedPrepositionalPhrase {
                        first: Box::new(first),
                        rest: vec![coordination],
                    },
                ),
                PrepositionalPhrase::Coordinated(mut coordinated) => {
                    coordinated.rest.push(coordination);
                    PrepositionalPhrase::Coordinated(coordinated)
                }
            }))
        }
        _ => None,
    }
}

/// The noun-phrase connective a coordinating conjunction denotes, or `None`
/// when it never joins phrases (`then` sequences clauses).
fn noun_phrase_conjunction(
    conjunction: crate::features::Conjunction,
) -> Option<crate::features::Conjunction> {
    match conjunction {
        crate::features::Conjunction::And
        | crate::features::Conjunction::Or
        | crate::features::Conjunction::AndOr => Some(conjunction),
        crate::features::Conjunction::Then | crate::features::Conjunction::Plus => None,
    }
}

pub(super) fn lower_arithmetic_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let value = match tag {
        RuleTag::NounPhraseMinus => {
            let Lowered::NounPhrase(left) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(right) = take(children, 2)? else {
                return None;
            };
            crate::syntax::ArithmeticValue::Minus {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown => {
            let Lowered::NounPhrase(value) = take(children, 1)? else {
                return None;
            };
            let rounding = match tag {
                RuleTag::NounPhraseHalfRoundedUp => Some(crate::syntax::Rounding::Up),
                RuleTag::NounPhraseHalfRoundedDown => Some(crate::syntax::Rounding::Down),
                _ => None,
            };
            crate::syntax::ArithmeticValue::Half {
                value: Box::new(value),
                rounding,
            }
        }
        _ => return None,
    };
    Some(Lowered::NounPhrase(NounPhrase::Arithmetic(value)))
}

pub(super) fn take(children: &mut [Lowered], index: usize) -> Option<Lowered> {
    let child = children.get_mut(index)?;
    Some(std::mem::replace(child, Lowered::Ignored))
}
