use super::Adjective;
use super::AdjectivePhrase;
use super::AuxiliaryInstance;
use super::Clause;
use super::ComparisonComplement;
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
use super::MeaningKey;
use super::NodeId;
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
use super::PowerToughness;
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
use super::SimpleClause;
use super::Subject;
use super::ThisCardForm;
use super::VerbAnalysis;
use super::VerbParticle;
use super::VerbPhrase;
use super::Vocab;
use super::ability;
use super::clause;
use super::parse_support::EnglishForest;
use crate::constructions::nominal::ReducedRecipientPassiveTheme;
use crate::constructions::nominal::RulesObjectFollowupNominal;
use crate::constructions::nominal::RulesObjectNominal;
use crate::forest::AlternativeSelection;
use crate::syntax::ComparativeWord;
use crate::syntax::PrepositionalObject;
use crate::syntax::PrepositionalObjectKind;

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
    ComparativeWord(ComparativeWord),
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
    PreverbModifier(crate::syntax::PreverbModifier),
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
    SetExceptionMarker(crate::syntax::SetExceptionMarker),
    PartitiveHead(crate::syntax::PartitiveHead),
    Rounding(crate::syntax::Rounding),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    Phrase(Phrase),
    PrepositionalObject(PrepositionalObject),
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
    ExceptionRiderList(crate::syntax::ExceptionRiderList),
    /// One restriction-run member's adjunct sequence (one adjunct for most
    /// members, two for the flat `once each turn` adverb+temporal pair).
    RestrictionMember(crate::syntax::RestrictionMember),
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
                    erased_field(group.name, field.kind, children.next()?)?
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
            let payload = erased_field(group.name, declaration.payload, child)?;
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
        super::rules::GeneratedRuleContext::Value
        | super::rules::GeneratedRuleContext::ObjectGap
        | super::rules::GeneratedRuleContext::ReducedRecipientPassive => None,
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
            AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => *path,
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
            erased_field(rule.group.name, field.kind, children.next()?)?
        };
        fields[field_index] = Some(value);
    }
    if children.next().is_some() {
        return None;
    }
    for (field, value) in construction.fields.iter().zip(&mut fields) {
        if value.is_none() && matches!(field.kind, FieldKindData::Optional { .. }) {
            *value = erased_absent(field.kind);
        }
    }
    let fields = fields.into_iter().collect::<Option<Vec<_>>>()?;
    let value = construction.erased_partial_builder?(fields).ok()?;
    let value = match construction.erased_projector {
        Some(projector) => projector(value).ok()?,
        None => value,
    };
    if form
        .erased_recognizer
        .is_some_and(|recognizer| !recognizer(value.as_ref()))
    {
        return None;
    }
    let projected = project_generated_category(construction, value)?;
    match (rule.context, preposition, projected) {
        (
            super::rules::GeneratedRuleContext::Value
            | super::rules::GeneratedRuleContext::ObjectGap
            | super::rules::GeneratedRuleContext::ReducedRecipientPassive,
            None,
            projected,
        ) => Some(projected),
        (
            super::rules::GeneratedRuleContext::SharedPreposition,
            Some(preposition),
            Lowered::NounPhrase(object),
        ) => Some(Lowered::PrepositionalPhrase(
            crate::constructions::prepositional::expect_prepositional_phrase(
                preposition,
                PrepositionalObjectKind::NounPhrase(Box::new(object)),
            ),
        )),
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "generated declaration categories have one explicit typed projection each"
)]
fn project_generated_category(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
    value: deckmaste_construction_compiler::runtime::ErasedValue,
) -> Option<Lowered> {
    if construction.category == "NounPhrase" {
        let value = value.downcast::<NounPhrase>().ok()?;
        return Some(Lowered::NounPhrase(*value));
    }
    if construction.category == "RulesObjectNounPhrase" {
        let value = value
            .downcast::<crate::constructions::noun_phrase::RulesObjectNounPhrase>()
            .ok()?;
        return Some(Lowered::NounPhrase(value.into_noun_phrase()));
    }
    if construction.category == "ReducedRecipientPassiveTheme" {
        let value = value.downcast::<ReducedRecipientPassiveTheme>().ok()?;
        return Some(Lowered::NounPhrase(value.into_noun_phrase()));
    }
    if construction.category == "NominalPhrase" {
        let value = value.downcast::<NominalPhrase>().ok()?;
        return Some(Lowered::Nominal(*value));
    }
    if construction.category == "PossessiveNominal" {
        let value = value.downcast::<NominalPhrase>().ok()?;
        return Some(Lowered::PossessiveNominal(*value));
    }
    if construction.category == "RulesObjectNominal" {
        let value = value.downcast::<RulesObjectNominal>().ok()?;
        return Some(Lowered::Nominal(value.into_nominal()));
    }
    if construction.category == "RulesObjectFollowupNominal" {
        let value = value.downcast::<RulesObjectFollowupNominal>().ok()?;
        return Some(Lowered::Nominal(value.into_nominal()));
    }
    if matches!(
        construction.category,
        "PredicatedQualityFrom" | "PredicatedQualityBare"
    ) {
        let value = value.downcast::<crate::syntax::PredicatedQuality>().ok()?;
        return Some(Lowered::PredicatedQuality(*value));
    }
    if matches!(
        construction.category,
        "PredicatedArgumentFrom" | "PredicatedArgumentBare"
    ) {
        let value = value.downcast::<crate::syntax::PredicatedArgument>().ok()?;
        return Some(Lowered::PredicatedArgument(*value));
    }
    if construction.category == "DevotionColors" {
        let value = value.downcast::<crate::syntax::DevotionColors>().ok()?;
        return Some(Lowered::DevotionColors(*value));
    }
    if construction.category == "Noun" {
        let value = value.downcast::<NounInstance>().ok()?;
        return Some(Lowered::Noun(*value));
    }
    if construction.category == "Adjective" {
        let value = value.downcast::<Adjective>().ok()?;
        return Some(Lowered::Adjective(*value));
    }
    if construction.category == "AdjectivePhrase" {
        let value = value.downcast::<AdjectivePhrase>().ok()?;
        return Some(Lowered::AdjectivePhrase(*value));
    }
    if construction.category == "ComparisonStandard" {
        let value = value.downcast::<Phrase>().ok()?;
        return Some(Lowered::Phrase(*value));
    }
    if construction.category == "ComparisonComplement" {
        let value = value.downcast::<ComparisonComplement>().ok()?;
        return Some(Lowered::ComparisonComplement(*value));
    }
    if construction.category == "Quantity" {
        let value = value.downcast::<Quantity>().ok()?;
        return Some(Lowered::Quantity(*value));
    }
    if construction.category == "Determiner" {
        let value = value.downcast::<Determiner>().ok()?;
        return Some(Lowered::Determiner(*value));
    }
    if construction.category == "Sentence" {
        let value = value.downcast::<Sentence>().ok()?;
        return Some(Lowered::Sentence(*value));
    }
    if matches!(construction.category, "Clause" | "IndependentClause") {
        let value = value.downcast::<Clause>().ok()?;
        return Some(Lowered::Clause(*value));
    }
    if construction.category == "ExceptionRider" {
        let value = value.downcast::<crate::syntax::ExceptionRider>().ok()?;
        return Some(Lowered::ExceptionRider(*value));
    }
    if construction.category == "ExceptionRiderList" {
        let value = value.downcast::<crate::syntax::ExceptionRiderList>().ok()?;
        return Some(Lowered::ExceptionRiderList(*value));
    }
    if construction.category == "RestrictionMember" {
        let value = value.downcast::<crate::syntax::RestrictionMember>().ok()?;
        return Some(Lowered::RestrictionMember(*value));
    }
    if construction.category == "SimpleClause" {
        let value = value.downcast::<SimpleClause>().ok()?;
        return Some(Lowered::SimpleClause(*value));
    }
    if construction.category == "CopularRemainder" {
        let value = value.downcast::<CopularRemainder>().ok()?;
        return Some(Lowered::CopularRemainder(*value));
    }
    if construction.category == "EllipticalClause" {
        let value = value.downcast::<crate::syntax::EllipticalClause>().ok()?;
        return Some(Lowered::EllipticalClause(*value));
    }
    if construction.category == "InfinitiveClause" {
        let value = value.downcast::<crate::syntax::InfinitiveClause>().ok()?;
        return Some(Lowered::InfinitiveClause(*value));
    }
    if construction.category == "GerundClause" {
        let value = value.downcast::<crate::syntax::GerundClause>().ok()?;
        return Some(Lowered::GerundClause(*value));
    }
    if construction.category == "PrepositionalPhrase" {
        let value = value.downcast::<PrepositionalPhrase>().ok()?;
        return Some(Lowered::PrepositionalPhrase(*value));
    }
    if construction.category == "PrepositionalObject" {
        let value = value.downcast::<PrepositionalObject>().ok()?;
        return Some(Lowered::PrepositionalObject(*value));
    }
    if construction.category == "Verb" {
        let value = value.downcast::<VerbAnalysis>().ok()?;
        return Some(Lowered::Verb(*value));
    }
    if construction.category == "VerbPhrase" {
        let value = value.downcast::<VerbPhrase>().ok()?;
        return Some(Lowered::VerbPhrase(*value));
    }
    if construction.category == "FrequencyPhrase" {
        let value = value.downcast::<FrequencyPhrase>().ok()?;
        return Some(Lowered::Frequency(*value));
    }
    if construction.category == "RelativeClause" {
        let value = value.downcast::<RelativeClause>().ok()?;
        return Some(Lowered::RelativeClause(*value));
    }
    if matches!(construction.category, "ManaAmount" | "ManaAmountList") {
        let value = value.downcast::<crate::syntax::PredicateObject>().ok()?;
        return Some(Lowered::ManaAmount(*value));
    }
    if construction.category == "CoordinatedManaAmount" {
        let value = value
            .downcast::<crate::syntax::CoordinatedPredicateObject>()
            .ok()?;
        return Some(Lowered::ManaAmount(
            crate::syntax::PredicateObject::Coordinated(*value),
        ));
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

#[allow(
    clippy::too_many_lines,
    reason = "one exhaustive erased adapter table covers every declared English field kind"
)]
fn erased_field(
    group: &'static str,
    kind: deckmaste_construction_compiler::runtime::FieldKindData,
    value: Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    use deckmaste_construction_compiler::runtime::FieldKindData as K;
    match kind {
        K::Identity {
            value_type: "VerbAnalysis",
            ..
        } => {
            let Lowered::Verb(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "AuxiliaryInstance",
            ..
        } => {
            let Lowered::Auxiliary(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "ContractedSubjectAuxiliary",
            provider: "SubjectAuxiliary",
        } => {
            let Lowered::SubjectAuxiliary(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "RelativeMarker",
            provider: "RelativeMarker",
        } => {
            let Lowered::RelativeMarker(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "ExistentialForm",
            provider: "Existential",
        } => {
            let Lowered::Existential(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Preposition",
            provider: "Preposition",
        }
        | K::Scalar {
            codec: "Preposition",
        } => {
            let Lowered::Preposition(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Vocab",
            provider: "Adverb" | "SentenceAdverbial",
        } => {
            let Lowered::Adverb(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Subordinator",
            provider: "Subordinator",
        } => {
            let Lowered::Subordinator(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "PreverbModifier",
            provider: "PreverbAdverb",
        } => {
            let Lowered::PreverbModifier(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "VerbParticle",
            provider: "VerbParticle",
        } => {
            let Lowered::VerbParticle(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "CoinSide",
            provider: "CoinResult",
        } => {
            let Lowered::CoinResult(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "FrequencyPhrase",
            provider: "Frequency",
        } => {
            let Lowered::Frequency(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "NounInstance",
            ..
        } => {
            let Lowered::Noun(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "ClosedDeterminer",
            provider: "Determiner",
        } => {
            let Lowered::Determiner(value) = value else {
                return None;
            };
            Some(Box::new(
                crate::constructions::determiner::parts_determiner_closed(&value),
            ))
        }
        K::Identity {
            value_type: "ThisCardForm",
            provider: "ThisCard" | "FullThisCard" | "PossessiveThisCard",
        } => {
            let Lowered::ThisCard(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Pronoun",
            provider: "SubjectPronoun" | "ObjectPronoun" | "Reciprocal",
        } => {
            let Lowered::Pronoun(value) = value else {
                return None;
            };
            Some(Box::new(value.pronoun))
        }
        K::Identity {
            value_type: "Demonstrative",
            provider: "Demonstrative",
        } => {
            let Lowered::Determiner(value) = value else {
                return None;
            };
            let crate::syntax::DeterminerKind::Demonstrative(value) = value.kind() else {
                return None;
            };
            Some(Box::new(*value))
        }
        K::Identity {
            value_type: "SetExceptionMarker",
            provider: "SetExceptionMarker",
        } => {
            let Lowered::SetExceptionMarker(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "PartitiveHead",
            provider: "PartitiveEach",
        } => {
            let Lowered::PartitiveHead(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Rounding",
            provider: "RoundingUp" | "RoundingDown",
        } => {
            let Lowered::Rounding(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "Adjective",
            provider: "Adjective",
        } => {
            let Lowered::Adjective(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "NominalModifier",
            ..
        } => {
            let Lowered::NominalModifier(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "CatalogAtom",
            ..
        } => {
            let Lowered::Catalog(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "QuotedAbility",
            provider: "QuotedAbility",
        } => {
            let Lowered::Phrase(Phrase::QuotedAbility(value)) = value else {
                return None;
            };
            Some(Box::new(*value))
        }
        K::Identity {
            value_type: "OracleSymbol",
            provider: "OracleSymbol",
        }
        | K::Scalar {
            codec: "OracleSymbol",
        } => {
            let Lowered::OracleSymbol(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Identity {
            value_type: "SymbolSequence",
            provider: "SymbolSequence",
        }
        | K::Scalar {
            codec: "SymbolSequence",
        } => {
            let Lowered::SymbolSequence(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Subtree {
            category: "InfinitiveClause",
            boxed,
        } if group == "predicate" => {
            let Lowered::InfinitiveClause(value) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        K::Subtree {
            category: "ComparisonStandard",
            boxed,
        } if group == "adjective" => {
            let Lowered::Phrase(value) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
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
        K::TypedScalar {
            value_type: "NumberLiteral",
            codec: "Numeral" | "DegreeMeasureNumeral",
        } => {
            let Lowered::Number(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::TypedScalar {
            value_type: "QuantityValue",
            codec: "Numeral",
        } => {
            let Lowered::Number(value) = value else {
                return None;
            };
            Some(Box::new(super::quantity_value(value)))
        }
        K::TypedScalar {
            value_type: "PowerToughness",
            codec: "PowerToughness",
        } => {
            let Lowered::PowerToughness(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Scalar {
            codec: "ComparativeWord",
        } => {
            let Lowered::ComparativeWord(value) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Scalar { codec: "ColorWord" } => {
            let Lowered::Adjective(Adjective::Color(value)) = value else {
                return None;
            };
            Some(Box::new(value))
        }
        K::Optional { inner } => erased_optional(*inner, &value),
        K::Identity { .. }
        | K::Scalar { .. }
        | K::TypedScalar { .. }
        | K::SurfaceScalar { .. }
        | K::Sequence { .. } => None,
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
        K::Identity { .. }
        | K::Subtree { .. }
        | K::Scalar { .. }
        | K::TypedScalar { .. }
        | K::SurfaceScalar { .. }
        | K::Sequence { .. } => None,
    }
}

fn erased_transitive_predicate(
    boxed: bool,
    value: Lowered,
) -> Option<deckmaste_construction_compiler::runtime::ErasedValue> {
    let value = match value {
        Lowered::VerbPhrase(value) => Box::new(clause::finish_reduced_recipient_passive(value)?)
            as deckmaste_construction_compiler::runtime::ErasedValue,
        Lowered::Generated(GeneratedValue::Typed(value))
            if value.is::<crate::syntax::TransitivePredicate>() =>
        {
            value
        }
        _ => return None,
    };
    if boxed {
        let value = value
            .downcast::<crate::syntax::TransitivePredicate>()
            .ok()?;
        Some(Box::new(value))
    } else {
        Some(value)
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
        "Verb" => typed!(Verb, value),
        "VerbPhrase" => typed!(VerbPhrase, value),
        "NounInstance" => typed!(Noun, value),
        "Adjective" => typed!(Adjective, value),
        "NounPhrase" => typed!(NounPhrase, value),
        "ReducedRecipientPassiveTheme" => {
            let Lowered::NounPhrase(value) = value else {
                return None;
            };
            let value = ReducedRecipientPassiveTheme::from_noun_phrase(value);
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "NominalPhrase" => typed!(Nominal, value),
        "PossessiveNominal" => typed!(PossessiveNominal, value),
        "RulesObjectNominal" => {
            let Lowered::Nominal(value) = value else {
                return None;
            };
            let value = RulesObjectNominal::from_nominal(value);
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "RulesObjectFollowupNominal" => {
            let Lowered::Nominal(value) = value else {
                return None;
            };
            let value = RulesObjectFollowupNominal::from_nominal(value);
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
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
        "PrepositionalObject" => {
            let Lowered::PrepositionalObject(value) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "ComparisonComplement" => typed!(ComparisonComplement, value),
        "InfinitiveClause" => {
            let Lowered::InfinitiveClause(value) = value else {
                return None;
            };
            let value = clause::finish_infinitive(value)?;
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "GerundClause" => typed!(GerundClause, value),
        "FrequencyPhrase" => typed!(Frequency, value),
        "RelativeClause" => typed!(RelativeClause, value),
        "Predicate" => {
            let Lowered::VerbPhrase(value) = value else {
                return None;
            };
            let value = crate::constructions::relative::project_predicate_hole(value).ok()?;
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "ObjectGapPredicate" => {
            let Lowered::VerbPhrase(value) = value else {
                return None;
            };
            let value =
                crate::constructions::relative::project_object_gap_predicate_hole(value).ok()?;
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "Quantity" => typed!(Quantity, value),
        "DevotionColors" => typed!(DevotionColors, value),
        "PowerToughness" => typed!(PowerToughness, value),
        "ManaAmount" | "ManaAmountList" => typed!(ManaAmount, value),
        "CoordinatedManaAmount" => {
            let Lowered::ManaAmount(crate::syntax::PredicateObject::Coordinated(value)) = value
            else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "TransitivePredicate" => erased_transitive_predicate(boxed, value),
        "IndependentClause" => {
            let Lowered::Clause(Clause::Independent(value)) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "Clause" => {
            let Lowered::Clause(value) = value else {
                return None;
            };
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "ExceptionRider" => typed!(ExceptionRider, value),
        "ExceptionRiderList" => typed!(ExceptionRiderList, value),
        "RestrictionMember" => typed!(RestrictionMember, value),
        "SimpleClause" => typed!(SimpleClause, value),
        "CopularRemainder" => typed!(CopularRemainder, value),
        "KeywordArgument" => {
            let Lowered::PredicatedArgument(value) = value else {
                return None;
            };
            let value = KeywordArgument::Predicated(value);
            if boxed { Some(Box::new(Box::new(value))) } else { Some(Box::new(value)) }
        }
        "PredicatedQualityFrom" | "PredicatedQualityBare" => {
            typed!(PredicatedQuality, value)
        }
        "PredicatedArgumentFrom" | "PredicatedArgumentBare" => {
            typed!(PredicatedArgument, value)
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
        K::Scalar {
            codec: "ComparativeWord",
        } => {
            let Lowered::ComparativeWord(value) = value else {
                return None;
            };
            Some(Box::new(Some(*value)))
        }
        K::Scalar { codec: "ColorWord" } => {
            let Lowered::Adjective(Adjective::Color(value)) = value else {
                return None;
            };
            Some(Box::new(Some(*value)))
        }
        K::Scalar {
            codec: "OracleSymbol",
        } => {
            let Lowered::OracleSymbol(value) = value else {
                return None;
            };
            Some(Box::new(Some(value.clone())))
        }
        K::Scalar {
            codec: "SymbolSequence",
        } => {
            let Lowered::SymbolSequence(value) = value else {
                return None;
            };
            Some(Box::new(Some(value.clone())))
        }
        K::TypedScalar {
            value_type: "Vocab",
            codec: "Adverb",
        } => {
            let Lowered::Adverb(value) = value else {
                return None;
            };
            Some(Box::new(Some(*value)))
        }
        K::Subtree {
            category,
            boxed: false,
        } => match category {
            "NounPhrase" => {
                let Lowered::NounPhrase(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "AdjectivePhrase" | "ComparisonAdjectivePhrase" => {
                let Lowered::AdjectivePhrase(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "PrepositionalPhrase" => {
                let Lowered::PrepositionalPhrase(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "GerundClause" => {
                let Lowered::GerundClause(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "Clause" => {
                let Lowered::Clause(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "ExceptionRider" => {
                let Lowered::ExceptionRider(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "ExceptionRiderList" => {
                let Lowered::ExceptionRiderList(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "RestrictionMember" => {
                let Lowered::RestrictionMember(value) = value else {
                    return None;
                };
                Some(Box::new(Some(value.clone())))
            }
            "RulesObjectNominal" => {
                let Lowered::Nominal(value) = value else {
                    return None;
                };
                Some(Box::new(Some(RulesObjectNominal::from_nominal(
                    value.clone(),
                ))))
            }
            "RulesObjectFollowupNominal" => {
                let Lowered::Nominal(value) = value else {
                    return None;
                };
                Some(Box::new(Some(RulesObjectFollowupNominal::from_nominal(
                    value.clone(),
                ))))
            }
            _ => None,
        },
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
        K::Scalar {
            codec: "ComparativeWord",
        } => Some(Box::new(None::<crate::syntax::ComparativeWord>)),
        K::Scalar { codec: "ColorWord" } => Some(Box::new(None::<crate::word::ColorWord>)),
        K::Scalar {
            codec: "OracleSymbol",
        } => Some(Box::new(None::<crate::syntax::OracleSymbol>)),
        K::Scalar {
            codec: "SymbolSequence",
        } => Some(Box::new(None::<Vec<crate::syntax::OracleSymbol>>)),
        K::TypedScalar {
            value_type: "Vocab",
            codec: "Adverb",
        } => Some(Box::new(None::<crate::word::Vocab>)),
        K::Subtree {
            category: "NounPhrase",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::NounPhrase>)),
        K::Subtree {
            category: "AdjectivePhrase" | "ComparisonAdjectivePhrase",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::AdjectivePhrase>)),
        K::Subtree {
            category: "PrepositionalPhrase",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::PrepositionalPhrase>)),
        K::Subtree {
            category: "GerundClause",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::GerundClause>)),
        K::Subtree {
            category: "Clause",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::Clause>)),
        K::Subtree {
            category: "ExceptionRider",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::ExceptionRider>)),
        K::Subtree {
            category: "ExceptionRiderList",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::ExceptionRiderList>)),
        K::Subtree {
            category: "RestrictionMember",
            boxed: false,
        } => Some(Box::new(None::<crate::syntax::RestrictionMember>)),
        K::Subtree {
            category: "RulesObjectNominal",
            boxed: false,
        } => Some(Box::new(None::<RulesObjectNominal>)),
        K::Subtree {
            category: "RulesObjectFollowupNominal",
            boxed: false,
        } => Some(Box::new(None::<RulesObjectFollowupNominal>)),
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
        MeaningKey::Literal(_) | MeaningKey::GeneratedLiteral(_) | MeaningKey::Punctuation(_) => {
            Lowered::Ignored
        }
        MeaningKey::Number(number) => Lowered::Number(*number),
        MeaningKey::ComparativeWord(word) => Lowered::ComparativeWord(*word),
        MeaningKey::Determiner(determiner) => Lowered::Determiner(determiner.clone()),
        MeaningKey::Noun(noun) => Lowered::Noun(noun.clone()),
        MeaningKey::Adjective(adjective) => Lowered::Adjective(adjective.clone()),
        MeaningKey::Adverb(adverb) => Lowered::Adverb(*adverb),
        MeaningKey::PreverbModifier(crate::grammar::PreverbModifierKey::Next) => {
            Lowered::PreverbModifier(crate::syntax::PreverbModifier::Next)
        }
        MeaningKey::VerbParticle(particle) => Lowered::VerbParticle(*particle),
        MeaningKey::CoinResult(side) => Lowered::CoinResult(*side),
        MeaningKey::Frequency(frequency) => Lowered::Frequency(*frequency),
        MeaningKey::Pronoun(pronoun) => Lowered::Pronoun(*pronoun),
        MeaningKey::SetExceptionMarker(marker) => Lowered::SetExceptionMarker(*marker),
        MeaningKey::PartitiveHead(head) => Lowered::PartitiveHead(*head),
        MeaningKey::Rounding(rounding) => Lowered::Rounding(*rounding),
        MeaningKey::Auxiliary(auxiliary) => {
            Lowered::Auxiliary(auxiliary.with_contraction(surface.contraction()?))
        }
        MeaningKey::SubjectAuxiliary(subject_auxiliary) => {
            if surface.contraction()? != crate::features::Contraction::Contracted {
                return None;
            }
            let subject = match subject_auxiliary.subject {
                ContractedSubjectKey::Pronoun(pronoun) => {
                    NounPhrase::from_pronoun_declaration(pronoun, PronounCase::Subject)
                }
                ContractedSubjectKey::Demonstrative(demonstrative) => {
                    NounPhrase::from_demonstrative_declaration(demonstrative)
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
                        NounForm::Singular => NounInstance::unchecked_singular(noun),
                        NounForm::Plural => NounInstance::unchecked_plural(noun),
                        NounForm::Mass => NounInstance::unchecked_mass(noun),
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
        RuleTag::NominalPowerToughnessComplement
        | RuleTag::ModifierConjunctAdjective
        | RuleTag::ModifierConjunctNoun
        | RuleTag::ModifierConjunctNegated
        | RuleTag::ModifierListSingle
        | RuleTag::ModifierListComma
        | RuleTag::CoordinatedModifierConjoined
        | RuleTag::CoordinatedModifierOxford
        | RuleTag::NominalCoordinatedModifier => lower_nominal(tag, children),
        RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated => lower_phrase(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic
        | RuleTag::VerbPhraseCoordinatedAdjective => clause::lower_clause(tag, children),
    }
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
pub(super) fn lower_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::PrepositionalPhraseListPair
        | RuleTag::PrepositionalPhraseListComma
        | RuleTag::PrepositionalPhraseSiblingCoordinated => {
            let Lowered::PrepositionalPhrase(first) = take(children, 0)? else {
                return None;
            };
            let (conjunction, next_index) = if matches!(
                tag,
                RuleTag::PrepositionalPhraseListPair | RuleTag::PrepositionalPhraseListComma
            ) {
                (None, 2)
            } else if children.len() == 4 {
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                (Some(noun_phrase_conjunction(conjunction)?), 3)
            } else {
                let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                    return None;
                };
                (Some(noun_phrase_conjunction(conjunction)?), 2)
            };
            let Lowered::PrepositionalPhrase(next) = take(children, next_index)? else {
                return None;
            };
            let next = next.into_simple()?;
            let coordination = crate::syntax::PrepositionalPhraseCoordination {
                conjunction,
                phrase: next,
            };
            let first = if let Some(first) = first.clone().into_simple() {
                PrepositionalPhrase::coordinated(first, vec![coordination])
            } else {
                let mut first = first;
                first.push_coordination(coordination).then_some(first)?
            };
            Some(Lowered::PrepositionalPhrase(first))
        }
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "lowering nominals is intentionally long"
)]
pub(super) fn lower_nominal(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NominalPowerToughnessComplement => {
            let Lowered::Nominal(nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PowerToughness(power_toughness) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(
                crate::constructions::nominal::build_nominal_power_toughness_complement(
                    nominal,
                    power_toughness,
                )
                .ok()?,
            ))
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
            let Lowered::Nominal(nominal) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Nominal(
                crate::constructions::nominal::build_nominal_coordinated_modifier(
                    coordinated,
                    nominal,
                )
                .ok()?,
            ))
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

pub(super) fn take(children: &mut [Lowered], index: usize) -> Option<Lowered> {
    let child = children.get_mut(index)?;
    Some(std::mem::replace(child, Lowered::Ignored))
}
