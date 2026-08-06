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
use super::Vocabulary;
use super::ability;
use super::adjective_comparison_state;
use super::clause;
use super::opacity;
use super::parse_support::EnglishForest;
use super::quantity_value;
use crate::forest::AlternativeSelection;
use crate::word::NounDeclension;
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
    project_generated_category(construction, value)
}

fn project_generated_category(
    _construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    value: deckmaste_construction_compiler::runtime::ErasedValue,
) -> Option<Lowered> {
    match _construction.id {
        "noun_phrase_coordination" => {
            let value = value
                .downcast::<crate::constructions::coordination::DerivedCoordinatedNounPhrase>()
                .ok()?;
            let mut noun_phrase = value.first().as_ref().clone();
            for member in value.rest() {
                noun_phrase = push_noun_phrase_coordination(noun_phrase, member.clone());
            }
            return Some(Lowered::NounPhrase(noun_phrase));
        }
        "shared_determiner_nominal" => {
            let value = value
                .downcast::<crate::constructions::coordination::DerivedCoordinatedNominalPhrase>()
                .ok()?;
            let first = value.first().clone();
            let mut rest = value.rest().clone();
            let mut complements = value.complements().clone();
            if complements.is_empty() {
                let earlier_has_relative = nominal_has_relative(&first)
                    || rest
                        .iter()
                        .take(rest.len().saturating_sub(1))
                        .any(|member| nominal_has_relative(&member.phrase));
                if let Some(last) = rest.last_mut() {
                    complements =
                        take_trailing_group_complements(earlier_has_relative, &mut last.phrase);
                }
            }
            return Some(Lowered::NounPhrase(NounPhrase::CoordinatedNominal(
                crate::syntax::CoordinatedNominalPhrase {
                    determiner: value.determiner().clone(),
                    first,
                    rest,
                    complements,
                },
            )));
        }
        _ => {}
    }
    #[cfg(test)]
    match _construction.id {
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
            codec: "Conjunction",
        } => {
            let Lowered::Conjunction(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Scalar { codec: "Comma" } => Some(Box::new(crate::features::Comma::Present)),
        K::Optional { inner } => erased_optional(*inner, value),
        K::Scalar { .. } | K::Sequence { .. } => None,
    }
}

fn erased_absent(
    kind: deckmaste_construction_compiler::runtime::FieldKindData,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match kind {
        K::Scalar { codec: "Comma" } => Some(Box::new(crate::features::Comma::Absent)),
        K::Optional { inner } => erased_optional_absent(*inner),
        K::Subtree { .. } | K::Scalar { .. } | K::Sequence { .. } => None,
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
    value: Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match inner {
        K::Scalar {
            codec: "Conjunction",
        } => {
            let Lowered::Conjunction(value) = value else {
                return None;
            };
            Some(Box::new(Some(value)))
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
            codec: "Conjunction",
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
        | RuleTag::NounPhraseDamageCoordination
        | RuleTag::SharedDeterminerNominal
        | RuleTag::NounPhraseSharedDeterminer
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
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::NounPhraseListSingle
        | RuleTag::NounPhraseListComma
        | RuleTag::NounPhraseCoordinationOxford
        | RuleTag::NounPhraseMinus
        | RuleTag::NounPhraseHalf
        | RuleTag::NounPhraseHalfRoundedUp
        | RuleTag::NounPhraseHalfRoundedDown
        | RuleTag::PrepositionalPhraseCoordinated
        | RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated
        | RuleTag::PrepositionalPhraseRulesObjectCoordinated
        | RuleTag::PrepositionalPhraseSharedDeterminer
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
        RuleTag::NounPhraseDamageCoordination => {
            let Lowered::Nominal(first) = take(children, 0)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                return None;
            };
            let conjunction = noun_phrase_conjunction(conjunction)?;
            let Lowered::Nominal(next) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(push_noun_phrase_coordination(
                NounPhrase::Nominal(first),
                crate::syntax::NounPhraseCoordination {
                    conjunction: Some(conjunction),
                    comma: crate::features::Comma::Absent,
                    phrase: NounPhrase::Nominal(next),
                },
            )))
        }
        RuleTag::SharedDeterminerNominal => {
            let Lowered::Ignored = take(children, 0)? else {
                return None;
            };
            let first = match take(children, 1)? {
                Lowered::Noun(head) => NominalPhrase {
                    determiner: None,
                    modifiers: Vec::new(),
                    head,
                    complements: Vec::new(),
                },
                Lowered::Nominal(first) => first,
                _ => return None,
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let conjunction = noun_phrase_conjunction(conjunction)?;
            let mut next = match take(children, 3)? {
                Lowered::Noun(head) => NominalPhrase {
                    determiner: None,
                    modifiers: Vec::new(),
                    head,
                    complements: Vec::new(),
                },
                Lowered::Nominal(next) => next,
                _ => return None,
            };
            let group_complements =
                take_trailing_group_complements(nominal_has_relative(&first), &mut next);
            Some(Lowered::NounPhrase(NounPhrase::CoordinatedNominal(
                crate::syntax::CoordinatedNominalPhrase {
                    determiner: crate::syntax::Determiner::Target(None),
                    first: Box::new(first),
                    rest: vec![crate::syntax::NominalPhraseCoordination {
                        conjunction: Some(conjunction),
                        comma: crate::features::Comma::Absent,
                        phrase: next,
                    }],
                    complements: group_complements,
                },
            )))
        }
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
        RuleTag::NounPhraseCoordination | RuleTag::NounPhraseAdditiveCoordination => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
                Conjunction::Plus
            } else {
                let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                    return None;
                };
                noun_phrase_conjunction(conjunction)?
            };
            let Lowered::NounPhrase(next) = take(children, 2)? else {
                return None;
            };
            let coordination = crate::syntax::NounPhraseCoordination {
                conjunction: Some(conjunction),
                comma: crate::features::Comma::Absent,
                phrase: next,
            };
            Some(Lowered::NounPhrase(push_noun_phrase_coordination(
                first,
                coordination,
            )))
        }
        RuleTag::NounPhraseSharedDeterminer | RuleTag::NounPhraseListSingle => take(children, 0),
        RuleTag::NounPhraseListComma | RuleTag::NounPhraseCoordinationOxford => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let (conjunction, next_index) = if tag == RuleTag::NounPhraseCoordinationOxford {
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                let conjunction = noun_phrase_conjunction(conjunction)?;
                (Some(conjunction), 3)
            } else {
                (None, 2)
            };
            let Lowered::NounPhrase(next) = take(children, next_index)? else {
                return None;
            };
            let coordination = crate::syntax::NounPhraseCoordination {
                conjunction,
                comma: crate::features::Comma::Present,
                phrase: next,
            };
            Some(Lowered::NounPhrase(push_noun_phrase_coordination(
                first,
                coordination,
            )))
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
        RuleTag::PrepositionalPhraseCoordinated
        | RuleTag::PrepositionalPhraseRulesObjectCoordinated => {
            let (conjunction_index, next_index, comma) =
                if children.len() == 5 { (3, 4, true) } else { (2, 3, false) };
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(first) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, conjunction_index)? else {
                return None;
            };
            let conjunction = noun_phrase_conjunction(conjunction)?;
            let Lowered::NounPhrase(next) = take(children, next_index)? else {
                return None;
            };
            let object = push_noun_phrase_coordination(
                first,
                crate::syntax::NounPhraseCoordination {
                    conjunction: Some(conjunction),
                    comma: crate::features::Comma::from(comma),
                    phrase: next,
                },
            );
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase::simple(
                preposition,
                Phrase::NounPhrase(Box::new(object)),
            )))
        }
        RuleTag::PrepositionalPhraseSharedDeterminer => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase::simple(
                preposition,
                Phrase::NounPhrase(Box::new(object)),
            )))
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

/// Adds one noun-phrase coordination member. An explicitly parsed coordinated
/// nominal may extend under its determiner; complete noun phrases coordinate at
/// the outer level. Homogeneous runs stay flat, while a connective change opens
/// a new outer group instead of erasing the inner grouping.
pub(super) fn push_noun_phrase_coordination(
    first: NounPhrase,
    coordination: crate::syntax::NounPhraseCoordination,
) -> NounPhrase {
    let crate::syntax::NounPhraseCoordination {
        conjunction,
        comma,
        phrase: next,
    } = coordination;
    let mut first = first;
    if let NounPhrase::Nominal(host) = &mut first
        && let NounPhrase::Nominal(next_nominal) = &next
        && push_into_last_prepositional_object(host, conjunction, comma, next_nominal)
    {
        return first;
    }
    match (first, next) {
        (NounPhrase::CoordinatedNominal(mut coordinated), NounPhrase::Nominal(mut next))
            if coordinated.complements.is_empty()
                && next.determiner.is_none()
                && shared_determiner_accepts(&coordinated.determiner, &next)
                && shared_determiner_group_can_extend(&coordinated, conjunction, comma)
                && coordination_run_extends(
                    coordinated.rest.iter().map(|member| member.conjunction),
                    conjunction,
                    comma,
                ) =>
        {
            let earlier_has_relative = nominal_has_relative(&coordinated.first)
                || coordinated
                    .rest
                    .iter()
                    .any(|member| nominal_has_relative(&member.phrase));
            let group_complements =
                take_trailing_group_complements(earlier_has_relative, &mut next);
            coordinated
                .rest
                .push(crate::syntax::NominalPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                });
            coordinated.complements.extend(group_complements);
            NounPhrase::CoordinatedNominal(coordinated)
        }
        (NounPhrase::Nominal(mut first), NounPhrase::Nominal(mut next))
            if first.determiner.is_some()
                && shared_determiner_can_open(&first)
                && shared_determiner_member(conjunction, comma)
                && first
                    .determiner
                    .as_ref()
                    .is_some_and(|determiner| shared_determiner_accepts(determiner, &next)) =>
        {
            let group_complements =
                take_trailing_group_complements(nominal_has_relative(&first), &mut next);
            let determiner = first
                .determiner
                .take()
                .expect("the match guard requires one determiner");
            NounPhrase::CoordinatedNominal(crate::syntax::CoordinatedNominalPhrase {
                determiner,
                first: Box::new(first),
                rest: vec![crate::syntax::NominalPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                }],
                complements: group_complements,
            })
        }
        (NounPhrase::Coordinated(mut coordinated), next)
            if coordination_run_extends(
                coordinated.rest.iter().map(|member| member.conjunction),
                conjunction,
                comma,
            ) =>
        {
            let next = match next {
                NounPhrase::Nominal(next) if shared_determiner_member(conjunction, comma) => {
                    let mut next = Some(next);
                    if push_into_last_shared_determiner(
                        &mut coordinated,
                        conjunction,
                        comma,
                        &mut next,
                    ) {
                        return NounPhrase::Coordinated(coordinated);
                    }
                    NounPhrase::Nominal(
                        next.expect("an unsuccessful push leaves the member available"),
                    )
                }
                next => next,
            };
            coordinated
                .rest
                .push(crate::syntax::NounPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                });
            NounPhrase::Coordinated(coordinated)
        }
        (first, next) => NounPhrase::Coordinated(crate::syntax::CoordinatedNounPhrase {
            first: Box::new(first),
            rest: vec![crate::syntax::NounPhraseCoordination {
                conjunction,
                comma,
                phrase: next,
            }],
        }),
    }
}

/// Repairs the legacy attachment selected for `two counters on target
/// creature or artifact`: the parser closes `on target creature` before it
/// sees the bare singular continuation, but that continuation cannot be an
/// independent noun phrase. Move it under the determiner already inside the
/// host's final PP instead of coordinating it with the whole host nominal.
/// The same normalization repeats across an Oxford run.
fn push_into_last_prepositional_object(
    host: &mut NominalPhrase,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
    next: &NominalPhrase,
) -> bool {
    if !shared_determiner_member(conjunction, comma) {
        return false;
    }
    let Some(NominalComplement::Prepositional(preposition)) = host.complements.last_mut() else {
        return false;
    };
    let Phrase::NounPhrase(object) = preposition.head_mut().object.as_mut() else {
        return false;
    };
    match object.as_mut() {
        NounPhrase::CoordinatedNominal(coordinated)
            if coordinated.complements.is_empty()
                && shared_determiner_accepts(&coordinated.determiner, next)
                && shared_determiner_group_can_extend(coordinated, conjunction, comma)
                && coordination_run_extends(
                    coordinated.rest.iter().map(|member| member.conjunction),
                    conjunction,
                    comma,
                ) =>
        {
            let mut next = next.clone();
            let earlier_has_relative = nominal_has_relative(&coordinated.first)
                || coordinated
                    .rest
                    .iter()
                    .any(|member| nominal_has_relative(&member.phrase));
            let group_complements =
                take_trailing_group_complements(earlier_has_relative, &mut next);
            coordinated
                .rest
                .push(crate::syntax::NominalPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                });
            coordinated.complements.extend(group_complements);
            true
        }
        NounPhrase::Nominal(first)
            if first.determiner.is_some()
                && shared_determiner_can_open(first)
                && first
                    .determiner
                    .as_ref()
                    .is_some_and(|determiner| shared_determiner_accepts(determiner, next)) =>
        {
            let mut next = next.clone();
            let group_complements =
                take_trailing_group_complements(nominal_has_relative(first), &mut next);
            let determiner = first
                .determiner
                .take()
                .expect("the match guard requires one determiner");
            let first = first.clone();
            **object = NounPhrase::CoordinatedNominal(crate::syntax::CoordinatedNominalPhrase {
                determiner,
                first: Box::new(first),
                rest: vec![crate::syntax::NominalPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                }],
                complements: group_complements,
            });
            true
        }
        _ => false,
    }
}

/// Extends a shared-determiner group that begins at the final member of an
/// outer coordination. In `this creature or another creature or artifact`,
/// the second `or` belongs under `another`; the first remains the connective
/// between the complete `this ...` selection and that shared group.
fn push_into_last_shared_determiner(
    coordinated: &mut crate::syntax::CoordinatedNounPhrase,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
    next: &mut Option<NominalPhrase>,
) -> bool {
    let Some(candidate) = next.as_ref() else {
        return false;
    };
    let Some(last) = coordinated.rest.last_mut() else {
        return false;
    };
    match &mut last.phrase {
        NounPhrase::CoordinatedNominal(inner)
            if inner.complements.is_empty()
                && shared_determiner_accepts(&inner.determiner, candidate)
                && shared_determiner_group_can_extend(inner, conjunction, comma)
                && coordination_run_extends(
                    inner.rest.iter().map(|member| member.conjunction),
                    conjunction,
                    comma,
                ) =>
        {
            let earlier_has_relative = nominal_has_relative(&inner.first)
                || inner
                    .rest
                    .iter()
                    .any(|member| nominal_has_relative(&member.phrase));
            let mut next = next
                .take()
                .expect("the candidate was borrowed from this option");
            let group_complements =
                take_trailing_group_complements(earlier_has_relative, &mut next);
            inner.rest.push(crate::syntax::NominalPhraseCoordination {
                conjunction,
                comma,
                phrase: next,
            });
            inner.complements.extend(group_complements);
            true
        }
        NounPhrase::Nominal(first)
            if shared_determiner_can_open(first)
                && first
                    .determiner
                    .as_ref()
                    .is_some_and(|determiner| shared_determiner_accepts(determiner, candidate)) =>
        {
            let mut next = next
                .take()
                .expect("the candidate was borrowed from this option");
            let group_complements =
                take_trailing_group_complements(nominal_has_relative(first), &mut next);
            let mut first = first.clone();
            let determiner = first
                .determiner
                .take()
                .expect("the match guard requires one determiner");
            last.phrase = NounPhrase::CoordinatedNominal(crate::syntax::CoordinatedNominalPhrase {
                determiner,
                first: Box::new(first),
                rest: vec![crate::syntax::NominalPhraseCoordination {
                    conjunction,
                    comma,
                    phrase: next,
                }],
                complements: group_complements,
            });
            true
        }
        _ => false,
    }
}

fn nominal_has_relative(nominal: &NominalPhrase) -> bool {
    nominal
        .complements
        .iter()
        .any(|complement| matches!(complement, NominalComplement::Relative(_)))
}

/// With no parallel relative on an earlier member, a trailing relative scopes
/// over the completed selection (`another target creature or artifact you
/// control`). Move it and every following complement together so their surface
/// order remains intact. Parallel relatives remain member-local.
fn take_trailing_group_complements(
    earlier_has_relative: bool,
    next: &mut NominalPhrase,
) -> Vec<NominalComplement> {
    if earlier_has_relative {
        return Vec::new();
    }
    let Some(relative_index) = next
        .complements
        .iter()
        .position(|complement| matches!(complement, NominalComplement::Relative(_)))
    else {
        return Vec::new();
    };
    next.complements.split_off(relative_index)
}

/// Whether one coordination edge belongs to nominal material rather than an
/// arithmetic value. An asyndetic member is only licensed after a comma; the
/// closing member must use a noun-phrase conjunction, never additive `plus`.
fn shared_determiner_member(
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
) -> bool {
    matches!(
        conjunction,
        Some(Conjunction::And | Conjunction::Or | Conjunction::AndOr)
    ) || conjunction.is_none() && comma.is_present()
}

/// A determinerless singular count nominal is not a complete English noun
/// phrase. If the preceding conjunct carries a determiner that licenses the
/// same form, that determiner scopes over both nominal members. A plural or
/// mass continuation only supplies the same evidence when the determiner has
/// an explicit number constraint (`two artifacts and creatures`, `all Auras
/// and Equipment`); unconstrained `the` and possessives do not claim an
/// otherwise complete bare phrase. Invariant catalog plurals can arrive via a
/// singular lexical alternative, so their declension supplies the plural form.
fn shared_determiner_accepts(determiner: &Determiner, nominal: &NominalPhrase) -> bool {
    shared_determiner_accepts_morphology(determiner, nominal)
}

/// A PP or infinitive closes its member after that member has joined the
/// group. Only an explicit Oxford continuation can prove that a comma-marked,
/// PP-bearing member was not the end of the list.
fn shared_determiner_group_can_extend(
    coordinated: &crate::syntax::CoordinatedNominalPhrase,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
) -> bool {
    coordinated.rest.last().is_none_or(|last| {
        shared_determiner_can_open(&last.phrase)
            || last.comma.is_present() && comma.is_present() && conjunction.is_some()
    })
}

fn shared_determiner_accepts_morphology(determiner: &Determiner, nominal: &NominalPhrase) -> bool {
    if nominal.determiner.is_some()
        || matches!(
            determiner,
            Determiner::Demonstrative(crate::syntax::Demonstrative::This)
        ) && nominal.modifiers.iter().any(|modifier| {
            matches!(
                modifier,
                NominalModifier::Adjective {
                    phrase: AdjectivePhrase {
                        head: Adjective::Participle(_, Verb::Word(Vocab::Equip | Vocab::Enchant),),
                        ..
                    },
                    ..
                }
            )
        })
    {
        return false;
    }
    let form = match nominal.head {
        NounInstance::Singular(_) => NounForm::Singular,
        NounInstance::Plural(_) => NounForm::Plural,
        NounInstance::Mass(_) => NounForm::Mass,
    };
    let cardinality = determiner.noun_cardinality();
    if form != NounForm::Singular && cardinality == crate::syntax::NounCardinality::Unconstrained {
        return false;
    }
    super::reduction::cardinality_accepts(cardinality, form)
        || form == NounForm::Singular
            && singular_has_invariant_plural(&nominal.head)
            && super::reduction::cardinality_accepts(cardinality, NounForm::Plural)
}

/// A PP or infinitive closes its nominal host before a following coordination.
/// Reopening `the ... power to target player or planeswalker` at `or` would
/// make `the` scope over `power ... or planeswalker`; the coordination instead
/// belongs inside the already-attached recipient PP. A relative also closes an
/// otherwise complete plural or mass member: unlike a bare singular, its next
/// conjunct is not morphological proof of shared scope (`all permanents they
/// control that are one or more colors`).
fn shared_determiner_can_open(nominal: &NominalPhrase) -> bool {
    let relative_closes_complete_head = !matches!(nominal.head, NounInstance::Singular(_))
        && nominal
            .complements
            .iter()
            .any(|complement| matches!(complement, NominalComplement::Relative(_)));
    !relative_closes_complete_head
        && nominal.complements.iter().all(|complement| {
            !matches!(
                complement,
                NominalComplement::Prepositional(_) | NominalComplement::Infinitive(_)
            )
        })
}

fn singular_has_invariant_plural(head: &NounInstance) -> bool {
    let NounInstance::Singular(noun) = head else {
        return false;
    };
    let vocab = match noun {
        Noun::Word(vocab) => Some(*vocab),
        Noun::Catalog(atom) => atom.vocab,
        Noun::Die(_) | Noun::Gerund(_) | Noun::Agentive(_) | Noun::Opaque(_) => None,
    };
    vocab
        .and_then(|vocab| Vocabulary::new().noun_definition(vocab))
        .is_some_and(|definition| definition.declension == NounDeclension::Invariant)
}

fn coordination_run_extends(
    conjunctions: impl DoubleEndedIterator<Item = Option<Conjunction>>,
    conjunction: Option<Conjunction>,
    comma: crate::features::Comma,
) -> bool {
    comma.is_present() || conjunctions.rev().flatten().next() == conjunction
}
