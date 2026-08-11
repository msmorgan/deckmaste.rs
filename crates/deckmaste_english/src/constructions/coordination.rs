//! Generated phrase-coordination declarations.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "declaration adapters are reached through generated metadata and share one checked-builder signature"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::catalog::CatalogAtom;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::grammar::AdjectiveComparisonState;
use crate::grammar::Features;
use crate::grammar::NominalAttachmentPhase;
use crate::grammar::PredicateAttachment;
use crate::grammar::VerbDependent;
use crate::grammar::VerbPhrase;
use crate::grammar::extend_predicate_features;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AdjectivePhraseCoordination;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedModifier;
use crate::syntax::Determiner;
use crate::syntax::DevotionColors;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordArgument;
use crate::syntax::ModifierCoordination;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NominalPhraseCoordination;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseConjunction;
use crate::syntax::NounPhraseCoordination;
use crate::syntax::NounPhraseKind;
use crate::syntax::Polarity;
use crate::syntax::PowerToughness;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PrepositionalPhraseCoordination;
use crate::syntax::PrepositionalPhraseKind;
use crate::syntax::Quantity;
use crate::syntax::QuotedAbility;
use crate::syntax::RelativeClause;
use crate::syntax::TransitivePredicate;
use crate::word::InitialSound;
use crate::word::NounInstance;

type CoordinatedModifierValue = CoordinatedModifier;
type PrepositionalPhraseValue = PrepositionalPhrase;
type ModifierConjunct = NominalModifier;
type ModifierList = CoordinatedModifier;
type PrepositionalPhraseList = PrepositionalPhrase;

/// One member of the dedicated attributive `with` list used by token
/// descriptions.  The closed sum is intentionally narrower than either a
/// generic predicate object or a keyword-ability line: only a keyword ability
/// or one complete quoted ability can occupy the slot.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum WithAttributeMember {
    Keyword(KeywordAbility),
    Quoted(QuotedAbility),
}

/// A continuation in a [`WithAttributeList`].  Interior members are
/// asyndetic; the final member carries the coordinating conjunction.  Commas
/// are derivable from position and arity and therefore are not stored.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WithAttributeCoordination {
    conjunction: Option<Conjunction>,
    member: WithAttributeMember,
}

impl WithAttributeCoordination {
    #[must_use]
    pub const fn conjunction(&self) -> Option<Conjunction> {
        self.conjunction
    }

    #[must_use]
    pub const fn member(&self) -> &WithAttributeMember {
        &self.member
    }
}

/// A structurally coordinated attributive `with` list.  Checked construction
/// admits it on a nominal only when the closed list contains both semantic
/// member kinds.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WithAttributeList {
    first: WithAttributeMember,
    rest: Vec<WithAttributeCoordination>,
}

impl WithAttributeList {
    #[must_use]
    pub const fn first(&self) -> &WithAttributeMember {
        &self.first
    }

    #[must_use]
    pub fn rest(&self) -> &[WithAttributeCoordination] {
        &self.rest
    }

    fn is_open(&self) -> bool {
        self.rest.iter().all(|member| member.conjunction.is_none())
    }

    fn is_closed(&self) -> bool {
        let Some((last, prefix)) = self.rest.split_last() else {
            return false;
        };
        matches!(last.conjunction, Some(Conjunction::And | Conjunction::Or))
            && prefix.iter().all(|member| member.conjunction.is_none())
    }

    fn is_genuinely_mixed(&self) -> bool {
        let mut keyword = matches!(self.first, WithAttributeMember::Keyword(_));
        let mut quoted = matches!(self.first, WithAttributeMember::Quoted(_));
        for continuation in &self.rest {
            keyword |= matches!(continuation.member, WithAttributeMember::Keyword(_));
            quoted |= matches!(continuation.member, WithAttributeMember::Quoted(_));
        }
        keyword && quoted
    }
}

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_with_attribute_keyword_bare(
    ability: CatalogAtom,
) -> Result<KeywordAbility, DeclarationViolation> {
    ability
        .is_keyword_ability()
        .then_some(KeywordAbility {
            ability,
            argument: KeywordArgument::Absent,
        })
        .ok_or_else(|| {
            violation(
                "with_attribute_keyword_bare",
                "the identity is a keyword-ability catalog atom",
            )
        })
}

fn with_attribute_keyword_bare_parts(value: &KeywordAbility) -> CatalogAtom {
    value.ability.clone()
}

fn is_with_attribute_keyword_bare(value: &KeywordAbility) -> bool {
    value.ability.is_keyword_ability() && matches!(value.argument, KeywordArgument::Absent)
}

fn make_with_attribute_keyword_counted(
    ability: CatalogAtom,
    count: Quantity,
) -> Result<KeywordAbility, DeclarationViolation> {
    ability
        .is_keyword_ability()
        .then_some(KeywordAbility {
            ability,
            argument: KeywordArgument::Counted(count),
        })
        .ok_or_else(|| {
            violation(
                "with_attribute_keyword_counted",
                "the identity is a keyword-ability catalog atom",
            )
        })
}

fn with_attribute_keyword_counted_parts(value: &KeywordAbility) -> (CatalogAtom, Quantity) {
    let KeywordArgument::Counted(count) = value.argument else {
        unreachable!("the counted-keyword recognizer checks its argument")
    };
    (value.ability.clone(), count)
}

fn is_with_attribute_keyword_counted(value: &KeywordAbility) -> bool {
    value.ability.is_keyword_ability() && matches!(value.argument, KeywordArgument::Counted(_))
}

fn make_with_attribute_member_keyword(
    keyword: KeywordAbility,
) -> Result<WithAttributeMember, DeclarationViolation> {
    (is_with_attribute_keyword_bare(&keyword) || is_with_attribute_keyword_counted(&keyword))
        .then_some(WithAttributeMember::Keyword(keyword))
        .ok_or_else(|| {
            violation(
                "with_attribute_member_keyword",
                "the keyword member has the admitted bare or counted shape",
            )
        })
}

fn with_attribute_member_keyword_parts(value: &WithAttributeMember) -> KeywordAbility {
    let WithAttributeMember::Keyword(keyword) = value else {
        unreachable!("the keyword-member recognizer checks its semantic alternative")
    };
    keyword.clone()
}

fn is_with_attribute_member_keyword(value: &WithAttributeMember) -> bool {
    matches!(value, WithAttributeMember::Keyword(_))
}

fn make_with_attribute_member_quoted(
    quoted: QuotedAbility,
) -> Result<WithAttributeMember, DeclarationViolation> {
    Ok(WithAttributeMember::Quoted(quoted))
}

fn with_attribute_member_quoted_parts(value: &WithAttributeMember) -> QuotedAbility {
    let WithAttributeMember::Quoted(quoted) = value else {
        unreachable!("the quoted-member recognizer checks its semantic alternative")
    };
    quoted.clone()
}

fn is_with_attribute_member_quoted(value: &WithAttributeMember) -> bool {
    matches!(value, WithAttributeMember::Quoted(_))
}

fn make_with_attribute_list_single(
    first: WithAttributeMember,
) -> Result<WithAttributeList, DeclarationViolation> {
    Ok(WithAttributeList {
        first,
        rest: Vec::new(),
    })
}

fn with_attribute_list_single_parts(value: &WithAttributeList) -> WithAttributeMember {
    value.first.clone()
}

fn is_with_attribute_list_single(value: &WithAttributeList) -> bool {
    value.rest.is_empty()
}

fn make_with_attribute_list_comma(
    mut list: WithAttributeList,
    member: WithAttributeMember,
) -> Result<WithAttributeList, DeclarationViolation> {
    if !list.is_open() {
        return Err(violation(
            "with_attribute_list_comma",
            "the source attribute list is still open",
        ));
    }
    list.rest.push(WithAttributeCoordination {
        conjunction: None,
        member,
    });
    Ok(list)
}

fn with_attribute_list_comma_parts(
    value: &WithAttributeList,
) -> (WithAttributeList, WithAttributeMember) {
    let mut list = value.clone();
    let continuation = list
        .rest
        .pop()
        .expect("the comma-list recognizer requires a continuation");
    (list, continuation.member)
}

fn is_with_attribute_list_comma(value: &WithAttributeList) -> bool {
    !value.rest.is_empty() && value.is_open()
}

fn close_with_attribute_list(
    construction: &'static str,
    mut list: WithAttributeList,
    conjunction: Conjunction,
    member: WithAttributeMember,
    oxford: bool,
) -> Result<WithAttributeList, DeclarationViolation> {
    if !list.is_open()
        || !matches!(conjunction, Conjunction::And | Conjunction::Or)
        || (oxford && list.rest.is_empty())
        || (!oxford && !list.rest.is_empty())
    {
        return Err(violation(
            construction,
            "the conjunction closes a list with the declared arity and punctuation",
        ));
    }
    list.rest.push(WithAttributeCoordination {
        conjunction: Some(conjunction),
        member,
    });
    Ok(list)
}

fn make_with_attribute_list_conjoined(
    list: WithAttributeList,
    conjunction: Conjunction,
    member: WithAttributeMember,
) -> Result<WithAttributeList, DeclarationViolation> {
    close_with_attribute_list(
        "with_attribute_list_conjoined",
        list,
        conjunction,
        member,
        false,
    )
}

fn make_with_attribute_list_oxford(
    list: WithAttributeList,
    conjunction: Conjunction,
    member: WithAttributeMember,
) -> Result<WithAttributeList, DeclarationViolation> {
    close_with_attribute_list(
        "with_attribute_list_oxford",
        list,
        conjunction,
        member,
        true,
    )
}

fn with_attribute_list_closed_parts(
    value: &WithAttributeList,
) -> (WithAttributeList, Conjunction, WithAttributeMember) {
    let mut list = value.clone();
    let continuation = list
        .rest
        .pop()
        .expect("a closed attribute list has a final continuation");
    (
        list,
        continuation
            .conjunction
            .expect("a closed attribute list has a final conjunction"),
        continuation.member,
    )
}

fn is_with_attribute_list_conjoined(value: &WithAttributeList) -> bool {
    value.rest.len() == 1 && value.is_closed()
}

fn is_with_attribute_list_oxford(value: &WithAttributeList) -> bool {
    value.rest.len() >= 2 && value.is_closed()
}

fn make_nominal_with_attributes(
    nominal: NominalPhrase,
    attributes: WithAttributeList,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !attributes.is_closed() || !attributes.is_genuinely_mixed() {
        return Err(violation(
            "nominal_with_attributes",
            "the attributive with-list is closed and genuinely mixed",
        ));
    }
    nominal
        .try_attach_declared_with_attributes(attributes)
        .ok_or_else(|| {
            violation(
                "nominal_with_attributes",
                "the nominal attachment phase admits a general complement",
            )
        })
}

fn nominal_with_attributes_parts(value: &NominalPhrase) -> (NominalPhrase, WithAttributeList) {
    value
        .clone()
        .try_split_declared_with_attributes()
        .expect("the with-attribute recognizer checks its final complement")
}

fn is_nominal_with_attributes(value: &NominalPhrase) -> bool {
    value
        .clone()
        .try_split_declared_with_attributes()
        .is_some_and(|(_, attributes)| attributes.is_closed() && attributes.is_genuinely_mixed())
}

fn make_coordinated_adjective_phrase(
    first: Box<AdjectivePhrase>,
    rest: Vec<crate::syntax::AdjectivePhraseCoordination>,
) -> Result<CoordinatedAdjectivePhrase, DeclarationViolation> {
    let value = CoordinatedAdjectivePhrase::from_declaration_parts(first, rest);
    is_coordinated_adjective_phrase(&value)
        .then_some(value)
        .ok_or_else(|| {
            violation(
                "coordinated_adjective_phrase",
                "the adjective list has one final conjunction and at least two members",
            )
        })
}

/// Builds a complete predicative adjective coordination from semantic
/// members while keeping the declaration-owned sequence carrier private to
/// this module.
pub(crate) fn build_coordinated_adjective_members(
    first: AdjectivePhrase,
    rest: Vec<(Option<Conjunction>, AdjectivePhrase)>,
) -> Result<CoordinatedAdjectivePhrase, DeclarationViolation> {
    build_coordinated_adjective_phrase(
        Box::new(first),
        rest.into_iter()
            .map(|(conjunction, phrase)| {
                AdjectivePhraseCoordination::from_declaration_parts(conjunction, phrase)
            })
            .collect(),
    )
}

fn coordinated_adjective_phrase_parts(
    value: &CoordinatedAdjectivePhrase,
) -> (
    Box<AdjectivePhrase>,
    Vec<crate::syntax::AdjectivePhraseCoordination>,
) {
    (Box::new(value.first().clone()), value.rest().to_vec())
}

fn is_coordinated_adjective_phrase(value: &CoordinatedAdjectivePhrase) -> bool {
    let Some((last, prefix)) = value.rest().split_last() else {
        return false;
    };
    matches!(
        last.conjunction(),
        Some(Conjunction::And | Conjunction::Or | Conjunction::AndOr)
    ) && prefix.iter().all(|member| member.conjunction().is_none())
}

fn reduce_coordinated_adjective_phrase_features(
    first: &Features,
    rest: &Features,
) -> Option<Features> {
    matches!(
        (first, rest),
        (
            Features::Adjective { .. },
            Features::GeneratedSequence { .. }
        )
    )
    .then_some(Features::None)
}

fn reduce_with_attribute_keyword_bare_features(ability: &Features) -> Option<Features> {
    matches!(ability, Features::None).then_some(Features::None)
}

fn reduce_with_attribute_keyword_counted_features(
    ability: &Features,
    count: &Features,
) -> Option<Features> {
    matches!((ability, count), (Features::None, Features::Quantity(_))).then_some(Features::None)
}

fn reduce_with_attribute_member_keyword_features(keyword: &Features) -> Option<Features> {
    matches!(keyword, Features::None).then_some(Features::WithAttributeMember {
        keyword: true,
        quoted: false,
    })
}

fn reduce_with_attribute_member_quoted_features(quoted: &Features) -> Option<Features> {
    matches!(quoted, Features::None).then_some(Features::WithAttributeMember {
        keyword: false,
        quoted: true,
    })
}

fn with_attribute_member_features(value: &Features) -> Option<(bool, bool)> {
    let Features::WithAttributeMember { keyword, quoted } = value else {
        return None;
    };
    Some((*keyword, *quoted))
}

fn reduce_with_attribute_list_single_features(first: &Features) -> Option<Features> {
    let (keyword, quoted) = with_attribute_member_features(first)?;
    Some(Features::WithAttributeList {
        keyword,
        quoted,
        len: 1,
        closed: false,
    })
}

fn reduce_with_attribute_list_comma_features(
    list: &Features,
    member: &Features,
) -> Option<Features> {
    let Features::WithAttributeList {
        keyword,
        quoted,
        len,
        closed: false,
    } = list
    else {
        return None;
    };
    let (next_keyword, next_quoted) = with_attribute_member_features(member)?;
    Some(Features::WithAttributeList {
        keyword: *keyword || next_keyword,
        quoted: *quoted || next_quoted,
        len: len + 1,
        closed: false,
    })
}

fn reduce_with_attribute_list_closed_features(
    list: &Features,
    conjunction: &Features,
    member: &Features,
    oxford: bool,
) -> Option<Features> {
    let Features::WithAttributeList {
        keyword,
        quoted,
        len,
        closed: false,
    } = list
    else {
        return None;
    };
    let Features::Conjunction(Conjunction::And | Conjunction::Or) = conjunction else {
        return None;
    };
    if (*len == 1) == oxford {
        return None;
    }
    let (next_keyword, next_quoted) = with_attribute_member_features(member)?;
    Some(Features::WithAttributeList {
        keyword: *keyword || next_keyword,
        quoted: *quoted || next_quoted,
        len: len + 1,
        closed: true,
    })
}

fn reduce_with_attribute_list_conjoined_features(
    list: &Features,
    conjunction: &Features,
    member: &Features,
) -> Option<Features> {
    reduce_with_attribute_list_closed_features(list, conjunction, member, false)
}

fn reduce_with_attribute_list_oxford_features(
    list: &Features,
    conjunction: &Features,
    member: &Features,
) -> Option<Features> {
    reduce_with_attribute_list_closed_features(list, conjunction, member, true)
}

fn reduce_nominal_with_attributes_features(
    nominal: &Features,
    attributes: &Features,
) -> Option<Features> {
    let Features::WithAttributeList {
        keyword: true,
        quoted: true,
        closed: true,
        ..
    } = attributes
    else {
        return None;
    };
    crate::grammar::reduction::reduce_nominal_prepositional(
        nominal,
        &Features::PrepositionalPhrase {
            preposition: crate::syntax::Preposition::With,
            nominal_attachment: true,
            role_members: vec![crate::grammar::PrepositionalRoleMember {
                preposition: crate::syntax::Preposition::With,
                nominal_attachment: true,
            }],
            shared_determiner_object: false,
            nearer_relative_host: false,
        },
    )
}

fn make_modifier_conjunct_adjective(
    adjective: AdjectivePhrase,
) -> Result<NominalModifier, DeclarationViolation> {
    Ok(NominalModifier::Adjective {
        polarity: Polarity::Positive,
        phrase: adjective,
    })
}

fn modifier_conjunct_adjective_parts(value: &NominalModifier) -> AdjectivePhrase {
    let NominalModifier::Adjective {
        polarity: Polarity::Positive,
        phrase,
    } = value
    else {
        unreachable!("the adjective-conjunct recognizer checks its semantic alternative")
    };
    phrase.clone()
}

fn is_modifier_conjunct_adjective(value: &NominalModifier) -> bool {
    matches!(
        value,
        NominalModifier::Adjective {
            polarity: Polarity::Positive,
            ..
        }
    )
}

fn make_modifier_conjunct_noun(
    noun: NounInstance,
) -> Result<NominalModifier, DeclarationViolation> {
    Ok(NominalModifier::Noun {
        polarity: Polarity::Positive,
        noun,
    })
}

fn modifier_conjunct_noun_parts(value: &NominalModifier) -> NounInstance {
    let NominalModifier::Noun {
        polarity: Polarity::Positive,
        noun,
    } = value
    else {
        unreachable!("the noun-conjunct recognizer checks its semantic alternative")
    };
    noun.clone()
}

fn is_modifier_conjunct_noun(value: &NominalModifier) -> bool {
    matches!(
        value,
        NominalModifier::Noun {
            polarity: Polarity::Positive,
            ..
        }
    )
}

fn make_modifier_conjunct_negated(
    modifier: NominalModifier,
) -> Result<NominalModifier, DeclarationViolation> {
    is_modifier_conjunct_negated(&modifier)
        .then_some(modifier)
        .ok_or_else(|| {
            violation(
                "modifier_conjunct_negated",
                "the lexical modifier carries negative polarity",
            )
        })
}

fn modifier_conjunct_negated_parts(value: &NominalModifier) -> NominalModifier {
    value.clone()
}

fn is_modifier_conjunct_negated(value: &NominalModifier) -> bool {
    matches!(
        value,
        NominalModifier::Adjective {
            polarity: Polarity::Negative,
            ..
        } | NominalModifier::Noun {
            polarity: Polarity::Negative,
            ..
        }
    )
}

fn make_modifier_list_single(
    first: NominalModifier,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    Ok(CoordinatedModifier::from_declaration_parts(
        Box::new(first),
        Vec::new(),
    ))
}

fn modifier_list_single_parts(value: &CoordinatedModifier) -> NominalModifier {
    value.first().clone()
}

fn is_modifier_list_single(value: &CoordinatedModifier) -> bool {
    value.rest().is_empty()
}

fn open_modifier_list(value: &CoordinatedModifier) -> bool {
    value
        .rest()
        .iter()
        .all(|member| member.conjunction().is_none())
}

fn make_modifier_list_comma(
    list: CoordinatedModifier,
    modifier: NominalModifier,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    if !open_modifier_list(&list) {
        return Err(violation(
            "modifier_list_comma",
            "the source list is still open",
        ));
    }
    let (first, mut rest) = list.into_declaration_parts();
    rest.push(ModifierCoordination::from_declaration_parts(None, modifier));
    Ok(CoordinatedModifier::from_declaration_parts(first, rest))
}

fn modifier_list_comma_parts(
    value: &CoordinatedModifier,
) -> (CoordinatedModifier, NominalModifier) {
    let (first, mut rest) = value.clone().into_declaration_parts();
    let member = rest
        .pop()
        .expect("the comma-list recognizer requires a continuation");
    let (_, modifier) = member.into_declaration_parts();
    (
        CoordinatedModifier::from_declaration_parts(first, rest),
        modifier,
    )
}

fn is_modifier_list_comma(value: &CoordinatedModifier) -> bool {
    !value.rest().is_empty() && open_modifier_list(value)
}

fn coordinating_conjunction(conjunction: Conjunction) -> bool {
    matches!(
        conjunction,
        Conjunction::And | Conjunction::Or | Conjunction::AndOr
    )
}

fn close_modifier_list(
    construction: &'static str,
    list: CoordinatedModifier,
    conjunction: Conjunction,
    modifier: NominalModifier,
    oxford: bool,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    if !open_modifier_list(&list)
        || !coordinating_conjunction(conjunction)
        || (oxford && list.rest().is_empty())
        || (!oxford && !list.rest().is_empty())
    {
        return Err(violation(
            construction,
            "the open list and final conjunction have the declared punctuation shape",
        ));
    }
    let (first, mut rest) = list.into_declaration_parts();
    rest.push(ModifierCoordination::from_declaration_parts(
        Some(conjunction),
        modifier,
    ));
    Ok(CoordinatedModifier::from_declaration_parts(first, rest))
}

fn make_coordinated_modifier_conjoined(
    list: CoordinatedModifier,
    conjunction: Conjunction,
    modifier: NominalModifier,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    close_modifier_list(
        "coordinated_modifier_conjoined",
        list,
        conjunction,
        modifier,
        false,
    )
}

fn make_coordinated_modifier_oxford(
    list: CoordinatedModifier,
    conjunction: Conjunction,
    modifier: NominalModifier,
) -> Result<CoordinatedModifier, DeclarationViolation> {
    close_modifier_list(
        "coordinated_modifier_oxford",
        list,
        conjunction,
        modifier,
        true,
    )
}

fn coordinated_modifier_parts(
    value: &CoordinatedModifier,
) -> (CoordinatedModifier, Conjunction, NominalModifier) {
    let (first, mut rest) = value.clone().into_declaration_parts();
    let member = rest
        .pop()
        .expect("the coordinated-list recognizer requires a final member");
    let (conjunction, modifier) = member.into_declaration_parts();
    let conjunction = conjunction.expect("the coordinated-list recognizer requires a conjunction");
    (
        CoordinatedModifier::from_declaration_parts(first, rest),
        conjunction,
        modifier,
    )
}

fn is_coordinated_modifier_conjoined(value: &CoordinatedModifier) -> bool {
    let [last] = value.rest() else {
        return false;
    };
    last.conjunction().is_some() && coordinating_conjunction(last.conjunction().unwrap())
}

fn is_coordinated_modifier_oxford(value: &CoordinatedModifier) -> bool {
    value.rest().len() >= 2
        && value.rest().last().is_some_and(|last| {
            last.conjunction().is_some()
                && coordinating_conjunction(last.conjunction().unwrap())
                && value.rest()[..value.rest().len() - 1]
                    .iter()
                    .all(|member| member.conjunction().is_none())
        })
}

fn make_nominal_coordinated_modifier(
    coordinated: CoordinatedModifier,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::nominal::build_nominal_coordinated_modifier(coordinated, nominal)
}

fn nominal_coordinated_modifier_parts(
    value: &NominalPhrase,
) -> (CoordinatedModifier, NominalPhrase) {
    let (modifier, nominal) =
        crate::constructions::nominal::project_nominal_coordinated_modifier_remainder(value)
            .expect("the nominal coordinated-modifier recognizer checks the prefix");
    let NominalModifier::Coordinated(coordinated) = modifier else {
        unreachable!("the nominal coordinated-modifier projection is typed")
    };
    (coordinated, nominal)
}

fn is_nominal_coordinated_modifier(value: &NominalPhrase) -> bool {
    crate::constructions::nominal::project_nominal_coordinated_modifier_remainder(value).is_some()
}

fn simple_prepositional(
    value: PrepositionalPhrase,
) -> Result<crate::syntax::SimplePrepositionalPhrase, DeclarationViolation> {
    value.into_simple().ok_or_else(|| {
        violation(
            "prepositional_phrase_sibling_coordinated",
            "each appended member is a simple prepositional phrase",
        )
    })
}

fn make_prepositional_phrase_list_pair(
    first: PrepositionalPhrase,
    second: PrepositionalPhrase,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    Ok(PrepositionalPhrase::coordinated(
        simple_prepositional(first)?,
        vec![PrepositionalPhraseCoordination {
            conjunction: None,
            phrase: simple_prepositional(second)?,
        }],
    ))
}

fn make_prepositional_phrase_list_comma(
    mut list: PrepositionalPhrase,
    next: PrepositionalPhrase,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    let next = simple_prepositional(next)?;
    if !matches!(list.kind(), PrepositionalPhraseKind::Coordinated(value) if value.rest().iter().all(|member| member.conjunction().is_none()))
    {
        return Err(violation(
            "prepositional_phrase_list_comma",
            "the source sibling list is still open",
        ));
    }
    let appended = list.push_coordination(PrepositionalPhraseCoordination {
        conjunction: None,
        phrase: next,
    });
    debug_assert!(appended);
    Ok(list)
}

fn prepositional_list_parts(
    value: &PrepositionalPhrase,
) -> (PrepositionalPhrase, PrepositionalPhrase) {
    let PrepositionalPhraseKind::Coordinated(coordinated) = value.kind() else {
        unreachable!("the prepositional-list recognizer checks its semantic alternative")
    };
    let mut rest = coordinated.rest().to_vec();
    let final_member = rest.pop().expect("a prepositional list has a continuation");
    let prefix = if rest.is_empty() {
        PrepositionalPhrase::from_prepositional_declaration(
            coordinated.first().preposition(),
            coordinated.first().object().clone(),
        )
    } else {
        PrepositionalPhrase::coordinated(coordinated.first().clone(), rest)
    };
    let next = PrepositionalPhrase::from_prepositional_declaration(
        final_member.phrase().preposition(),
        final_member.phrase().object().clone(),
    );
    (prefix, next)
}

fn is_prepositional_phrase_list_pair(value: &PrepositionalPhrase) -> bool {
    matches!(value.kind(), PrepositionalPhraseKind::Coordinated(coordinated) if coordinated.rest().len() == 1 && coordinated.rest()[0].conjunction().is_none())
}

fn is_prepositional_phrase_list_comma(value: &PrepositionalPhrase) -> bool {
    matches!(value.kind(), PrepositionalPhraseKind::Coordinated(coordinated) if coordinated.rest().len() >= 2 && coordinated.rest().iter().all(|member| member.conjunction().is_none()))
}

fn make_prepositional_phrase_sibling_coordinated(
    first: Option<PrepositionalPhrase>,
    list: Option<PrepositionalPhrase>,
    comma: Option<Comma>,
    conjunction: Conjunction,
    next: PrepositionalPhrase,
) -> Result<PrepositionalPhrase, DeclarationViolation> {
    if !coordinating_conjunction(conjunction) {
        return Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the final connective coordinates phrases",
        ));
    }
    let next = simple_prepositional(next)?;
    match (first, list, comma) {
        (Some(first), None, None) => Ok(PrepositionalPhrase::coordinated(
            simple_prepositional(first)?,
            vec![PrepositionalPhraseCoordination {
                conjunction: Some(conjunction),
                phrase: next,
            }],
        )),
        (None, Some(mut list), Some(Comma::Present)) if matches!(list.kind(), PrepositionalPhraseKind::Coordinated(coordinated) if !coordinated.rest().is_empty() && coordinated.rest().iter().all(|member| member.conjunction().is_none())) =>
        {
            let appended = list.push_coordination(PrepositionalPhraseCoordination {
                conjunction: Some(conjunction),
                phrase: next,
            });
            debug_assert!(appended);
            Ok(list)
        }
        _ => Err(violation(
            "prepositional_phrase_sibling_coordinated",
            "the source sibling list and comma have the declared shape",
        )),
    }
}

fn prepositional_phrase_sibling_coordinated_parts(
    value: &PrepositionalPhrase,
) -> (
    Option<PrepositionalPhrase>,
    Option<PrepositionalPhrase>,
    Option<Comma>,
    Conjunction,
    PrepositionalPhrase,
) {
    let (first, next) = prepositional_list_parts(value);
    let PrepositionalPhraseKind::Coordinated(coordinated) = value.kind() else {
        unreachable!("the sibling-coordination recognizer checks its semantic alternative")
    };
    let conjunction = coordinated
        .rest()
        .last()
        .and_then(PrepositionalPhraseCoordination::conjunction)
        .expect("the sibling-coordination recognizer checks its final connective");
    if matches!(first.kind(), PrepositionalPhraseKind::Coordinated(_)) {
        (None, Some(first), Some(Comma::Present), conjunction, next)
    } else {
        (Some(first), None, None, conjunction, next)
    }
}

fn is_prepositional_phrase_sibling_coordinated(value: &PrepositionalPhrase) -> bool {
    matches!(value.kind(), PrepositionalPhraseKind::Coordinated(coordinated) if coordinated.rest().last().and_then(PrepositionalPhraseCoordination::conjunction).is_some() && coordinated.rest()[..coordinated.rest().len() - 1].iter().all(|member| member.conjunction().is_none()))
}

fn make_verb_phrase_coordinated_adjective(
    predicate: VerbPhrase,
    complement: CoordinatedAdjectivePhrase,
) -> Result<VerbPhrase, DeclarationViolation> {
    let Some(features) = predicate.declaration_core_features() else {
        return Err(violation(
            "verb_phrase_coordinated_adjective",
            "the source owner has valid predicate features",
        ));
    };
    if extend_predicate_features(&features, PredicateAttachment::AdjectiveComplement).is_none() {
        return Err(violation(
            "verb_phrase_coordinated_adjective",
            "the predicate licenses an adjective complement",
        ));
    }
    let (mut dependents, shell) = predicate.declaration_into_dependent_projection();
    dependents.push(VerbDependent::CoordinatedAdjective(complement));
    Ok(VerbPhrase::declaration_from_dependent_projection(
        shell, dependents,
    ))
}

fn verb_phrase_coordinated_adjective_parts(
    value: &VerbPhrase,
) -> (VerbPhrase, CoordinatedAdjectivePhrase) {
    let (predicate, dependent) = value
        .declaration_last_dependent_parts()
        .expect("the coordinated-adjective recognizer checks the final dependent");
    let VerbDependent::CoordinatedAdjective(complement) = dependent else {
        unreachable!("the coordinated-adjective projection is typed")
    };
    (predicate, complement)
}

fn is_verb_phrase_coordinated_adjective(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::CoordinatedAdjective(_)))
    ) && value.declaration_core_features().is_some()
}

fn make_nominal_power_toughness_complement(
    nominal: NominalPhrase,
    stats: PowerToughness,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::nominal::build_nominal_power_toughness_complement(nominal, stats)
}

fn nominal_power_toughness_complement_parts(
    value: &NominalPhrase,
) -> (NominalPhrase, PowerToughness) {
    crate::constructions::nominal::project_nominal_power_toughness_remainder(value)
        .expect("the power/toughness-complement recognizer checks the final complement")
}

fn is_nominal_power_toughness_complement(value: &NominalPhrase) -> bool {
    crate::constructions::nominal::project_nominal_power_toughness_remainder(value).is_some()
}

fn modifier_features(
    value: &Features,
) -> Option<(InitialSound, bool, Vec<crate::catalog::CatalogAtom>)> {
    match value {
        Features::CoordinatedModifier {
            initial_sound,
            all_adjectives,
            noun_heads,
        } => Some((*initial_sound, *all_adjectives, noun_heads.clone())),
        _ => None,
    }
}

fn reduce_modifier_conjunct_adjective_features(adjective: &Features) -> Option<Features> {
    let Features::Adjective {
        initial_sound,
        comparison: AdjectiveComparisonState::NotComparative,
        card_orientation: false,
        ..
    } = adjective
    else {
        return None;
    };
    Some(Features::CoordinatedModifier {
        initial_sound: *initial_sound,
        all_adjectives: true,
        noun_heads: Vec::new(),
    })
}

fn reduce_modifier_conjunct_noun_features(noun: &Features) -> Option<Features> {
    let Features::Noun {
        identity,
        initial_sound,
        ..
    } = noun
    else {
        return None;
    };
    Some(Features::CoordinatedModifier {
        initial_sound: *initial_sound,
        all_adjectives: false,
        noun_heads: identity.iter().cloned().collect(),
    })
}

fn reduce_modifier_conjunct_negated_features(_modifier: &Features) -> Option<Features> {
    Some(Features::CoordinatedModifier {
        initial_sound: InitialSound::Consonant,
        all_adjectives: false,
        noun_heads: Vec::new(),
    })
}

fn reduce_modifier_list_single_features(modifier: &Features) -> Option<Features> {
    modifier_features(modifier).map(|_| modifier.clone())
}

fn combine_modifier_features(list: &Features, modifier: &Features) -> Option<Features> {
    let (initial_sound, all_adjectives, mut noun_heads) = modifier_features(list)?;
    let (_, appended_adjectives, appended_noun_heads) = modifier_features(modifier)?;
    noun_heads.extend(appended_noun_heads);
    Some(Features::CoordinatedModifier {
        initial_sound,
        all_adjectives: all_adjectives && appended_adjectives,
        noun_heads,
    })
}

fn reduce_modifier_list_comma_features(list: &Features, modifier: &Features) -> Option<Features> {
    combine_modifier_features(list, modifier)
}

fn reduce_coordinated_modifier_features(
    list: &Features,
    conjunction: &Features,
    modifier: &Features,
) -> Option<Features> {
    let Features::Conjunction(conjunction) = conjunction else {
        return None;
    };
    coordinating_conjunction(*conjunction).then(|| combine_modifier_features(list, modifier))?
}

fn reduce_nominal_coordinated_modifier_features(
    coordinated: &Features,
    nominal: &Features,
) -> Option<Features> {
    let (initial_sound, _, noun_heads) = modifier_features(coordinated)?;
    let Features::Nominal { head, .. } = nominal else {
        return None;
    };
    if head.as_ref().is_some_and(|head| noun_heads.contains(head)) {
        return None;
    }
    crate::grammar::reduction::nominal_with_prefix_features(
        nominal,
        initial_sound,
        false,
        AdjectiveComparisonState::NotComparative,
        true,
    )
}

fn combine_prepositional_features(
    first: &Features,
    next: &Features,
    shared_determiner_object: bool,
) -> Option<Features> {
    let Features::PrepositionalPhrase {
        preposition,
        shared_determiner_object: first_shared,
        role_members,
        ..
    } = first
    else {
        return None;
    };
    let Features::PrepositionalPhrase {
        nearer_relative_host,
        role_members: next_role_members,
        ..
    } = next
    else {
        return None;
    };
    let mut role_members = role_members.clone();
    role_members.extend(next_role_members.iter().copied());
    Some(Features::PrepositionalPhrase {
        preposition: *preposition,
        nominal_attachment: role_members.iter().all(|member| member.nominal_attachment),
        role_members,
        shared_determiner_object: shared_determiner_object && *first_shared,
        nearer_relative_host: *nearer_relative_host,
    })
}

pub(crate) fn reduce_prepositional_phrase_list_features(
    first: &Features,
    next: &Features,
) -> Option<Features> {
    combine_prepositional_features(first, next, true)
}

pub(crate) fn reduce_prepositional_phrase_sibling_features(
    first: Option<&Features>,
    list: Option<&Features>,
    _comma: Option<&Features>,
    conjunction: &Features,
    next: &Features,
) -> Option<Features> {
    let Features::Conjunction(conjunction) = conjunction else {
        return None;
    };
    let host = first.or(list)?;
    coordinating_conjunction(*conjunction)
        .then(|| combine_prepositional_features(host, next, false))?
}

fn reduce_verb_phrase_coordinated_adjective_features(
    predicate: &Features,
    complement: &Features,
) -> Option<Features> {
    let Features::CoordinatedModifier {
        all_adjectives: true,
        ..
    } = complement
    else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::AdjectiveComplement)
}

fn reduce_nominal_power_toughness_complement_features(
    nominal: &Features,
    _stats: &Features,
) -> Option<Features> {
    let Features::Nominal { attachment, .. } = nominal else {
        return None;
    };
    (!matches!(
        attachment,
        NominalAttachmentPhase::ReducedRecipientPassive
            | NominalAttachmentPhase::PostpositiveAdjective
            | NominalAttachmentPhase::Comparison
    ))
    .then(|| nominal.clone())
}

deckmaste_constructions_macro::constructions! {
    group noun_coordination;

    element noun_phrase_member bind NounPhraseCoordination {
        comma: surface lex Comma,
        conjunction: opt lex NounPhraseConjunction,
        phrase: hole NounPhrase,
    }

    element nominal_phrase_member bind NominalPhraseCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole NominalPhrase,
    }

    element adjective_phrase_member bind AdjectivePhraseCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole AdjectivePhrase,
    }

    element nominal_complement bind NominalComplement {
        variant Adjective: hole AdjectivePhrase,
        variant CoordinatedAdjective: hole CoordinatedAdjectivePhrase,
        variant Prepositional: hole PrepositionalPhrase,
        variant Infinitive: hole InfinitiveClause,
        variant Relative: hole RelativeClause,
        variant ReducedRecipientPassive: hole TransitivePredicate,
        variant Quantity: hole Quantity,
        variant Devotion: hole DevotionColors,
        variant PowerToughness: hole PowerToughness,
        variant EventClause: hole box IndependentClause,
        variant WithAttributes: hole WithAttributeList,
        variant KeywordArgument: hole KeywordArgument,
    }

    internal construction with_attribute_keyword_bare: WithAttributeKeyword {
        bind KeywordAbility via make_with_attribute_keyword_bare, with_attribute_keyword_bare_parts {
            ability: identity CatalogAtom via AbilityItem,
        }
        derive features: Features = reduce_with_attribute_keyword_bare_features(ability);
        form only @ 0 inverse check(is_with_attribute_keyword_bare) = identity(ability);
        selection unique;
    }

    internal construction coordinated_adjective_phrase: CoordinatedAdjectivePhrase {
        bind CoordinatedAdjectivePhrase via make_coordinated_adjective_phrase, coordinated_adjective_phrase_parts {
            first: hole box AdjectivePhrase,
            rest: seq adjective_phrase_member,
        }
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And, Or, AndOr];
        derive features: Features = reduce_coordinated_adjective_phrase_features(first, rest);
        form only @ 0 inverse check(is_coordinated_adjective_phrase) = first rest;
        selection unique;
    }

    internal construction with_attribute_keyword_counted: WithAttributeKeyword {
        bind KeywordAbility via make_with_attribute_keyword_counted, with_attribute_keyword_counted_parts {
            ability: identity CatalogAtom via AbilityItem,
            count: hole Quantity,
        }
        derive features: Features = reduce_with_attribute_keyword_counted_features(ability, count);
        form only @ 0 inverse check(is_with_attribute_keyword_counted) = identity(ability) count;
        selection unique;
    }

    internal construction with_attribute_member_keyword: WithAttributeMember {
        bind WithAttributeMember via make_with_attribute_member_keyword, with_attribute_member_keyword_parts {
            keyword: hole KeywordAbility via WithAttributeKeyword,
        }
        derive features: Features = reduce_with_attribute_member_keyword_features(keyword);
        form only @ 0 inverse check(is_with_attribute_member_keyword) = keyword;
        selection unique;
    }

    internal construction with_attribute_member_quoted: WithAttributeMember {
        bind WithAttributeMember via make_with_attribute_member_quoted, with_attribute_member_quoted_parts {
            quoted: identity QuotedAbility via QuotedAbility,
        }
        derive features: Features = reduce_with_attribute_member_quoted_features(quoted);
        form only @ 0 inverse check(is_with_attribute_member_quoted) = identity(quoted);
        selection unique;
    }

    internal construction with_attribute_list_single: WithAttributeList {
        bind WithAttributeList via make_with_attribute_list_single, with_attribute_list_single_parts {
            first: hole WithAttributeMember,
        }
        derive features: Features = reduce_with_attribute_list_single_features(first);
        form only @ 0 inverse check(is_with_attribute_list_single) = first;
        selection unique;
    }

    internal construction with_attribute_list_comma: WithAttributeList {
        bind WithAttributeList via make_with_attribute_list_comma, with_attribute_list_comma_parts {
            list: hole WithAttributeList,
            member: hole WithAttributeMember,
        }
        derive features: Features = reduce_with_attribute_list_comma_features(list, member);
        form only @ 0 inverse check(is_with_attribute_list_comma) = list "," member;
        selection unique;
    }

    internal construction with_attribute_list_conjoined: WithAttributeList {
        bind WithAttributeList via make_with_attribute_list_conjoined, with_attribute_list_closed_parts {
            list: hole WithAttributeList,
            conjunction: lex Conjunction,
            member: hole WithAttributeMember,
        }
        require conjunction in [And, Or];
        derive features: Features = reduce_with_attribute_list_conjoined_features(list, conjunction, member);
        form only @ 0 inverse check(is_with_attribute_list_conjoined) = list lex(conjunction) member;
        selection unique;
    }

    internal construction with_attribute_list_oxford: WithAttributeList {
        bind WithAttributeList via make_with_attribute_list_oxford, with_attribute_list_closed_parts {
            list: hole WithAttributeList,
            conjunction: lex Conjunction,
            member: hole WithAttributeMember,
        }
        require conjunction in [And, Or];
        derive features: Features = reduce_with_attribute_list_oxford_features(list, conjunction, member);
        form only @ 0 inverse check(is_with_attribute_list_oxford) = list "," lex(conjunction) member;
        selection unique;
    }

    construction nominal_with_attributes: NominalPhrase {
        bind NominalPhrase via make_nominal_with_attributes, nominal_with_attributes_parts {
            nominal: hole NominalPhrase,
            attributes: hole WithAttributeList,
        }
        derive features: Features = reduce_nominal_with_attributes_features(nominal, attributes);
        form only @ 0 inverse check(is_nominal_with_attributes) = nominal "with" attributes;
        selection unique;
    }

    construction modifier_conjunct_adjective: ModifierConjunct {
        bind NominalModifier via make_modifier_conjunct_adjective, modifier_conjunct_adjective_parts {
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_modifier_conjunct_adjective_features(adjective);
        form only @ 0 inverse check(is_modifier_conjunct_adjective) = adjective;
        selection unique;
    }

    construction modifier_conjunct_noun: ModifierConjunct {
        bind NominalModifier via make_modifier_conjunct_noun, modifier_conjunct_noun_parts {
            noun: hole NounInstance,
        }
        derive features: Features = reduce_modifier_conjunct_noun_features(noun);
        form only @ 0 inverse check(is_modifier_conjunct_noun) = noun;
        selection unique;
    }

    construction modifier_conjunct_negated: ModifierConjunct {
        bind NominalModifier via make_modifier_conjunct_negated, modifier_conjunct_negated_parts {
            modifier: identity NominalModifier via NegatedModifier,
        }
        derive features: Features = reduce_modifier_conjunct_negated_features(modifier);
        form only @ 0 inverse check(is_modifier_conjunct_negated) = identity(modifier);
        selection unique;
    }

    construction modifier_list_single: ModifierList {
        bind CoordinatedModifierValue via make_modifier_list_single, modifier_list_single_parts {
            first: hole ModifierConjunct,
        }
        derive features: Features = reduce_modifier_list_single_features(first);
        form only @ 0 inverse check(is_modifier_list_single) = first;
        selection unique;
    }

    construction modifier_list_comma: ModifierList {
        bind CoordinatedModifierValue via make_modifier_list_comma, modifier_list_comma_parts {
            list: hole ModifierList,
            modifier: hole ModifierConjunct,
        }
        derive features: Features = reduce_modifier_list_comma_features(list, modifier);
        form only @ 0 inverse check(is_modifier_list_comma) = list "," modifier;
        selection unique;
    }

    construction coordinated_modifier_conjoined: CoordinatedModifier {
        bind CoordinatedModifierValue via make_coordinated_modifier_conjoined, coordinated_modifier_parts {
            list: hole ModifierList,
            conjunction: lex Conjunction,
            modifier: hole ModifierConjunct,
        }
        require conjunction in [And, Or, AndOr];
        derive features: Features = reduce_coordinated_modifier_features(list, conjunction, modifier);
        derive base_precedence: Features = reduce_coordinated_modifier_features(list, conjunction, modifier);
        form only @ 0 inverse check(is_coordinated_modifier_conjoined) = list lex(conjunction) modifier;
        selection unique;
    }

    construction coordinated_modifier_oxford: CoordinatedModifier {
        bind CoordinatedModifierValue via make_coordinated_modifier_oxford, coordinated_modifier_parts {
            list: hole ModifierList,
            conjunction: lex Conjunction,
            modifier: hole ModifierConjunct,
        }
        require conjunction in [And, Or, AndOr];
        derive features: Features = reduce_coordinated_modifier_features(list, conjunction, modifier);
        derive base_precedence: Features = reduce_coordinated_modifier_features(list, conjunction, modifier);
        form only @ 0 inverse check(is_coordinated_modifier_oxford) = list "," lex(conjunction) modifier;
        selection unique;
    }

    construction nominal_coordinated_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_coordinated_modifier, nominal_coordinated_modifier_parts {
            coordinated: hole CoordinatedModifier,
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_nominal_coordinated_modifier_features(coordinated, nominal);
        form only @ 0 inverse check(is_nominal_coordinated_modifier) = coordinated nominal;
        selection unique;
    }

    construction prepositional_phrase_list_pair: PrepositionalPhraseList {
        bind PrepositionalPhraseValue via make_prepositional_phrase_list_pair, prepositional_list_parts {
            first: hole PrepositionalPhrase,
            second: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_prepositional_phrase_list_features(first, second);
        form only @ 0 inverse check(is_prepositional_phrase_list_pair) = first "," second;
        selection unique;
    }

    construction prepositional_phrase_list_comma: PrepositionalPhraseList {
        bind PrepositionalPhraseValue via make_prepositional_phrase_list_comma, prepositional_list_parts {
            list: hole PrepositionalPhraseList,
            next: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_prepositional_phrase_list_features(list, next);
        form only @ 0 inverse check(is_prepositional_phrase_list_comma) = list "," next;
        selection unique;
    }

    construction prepositional_phrase_sibling_coordinated: PrepositionalPhrase {
        bind PrepositionalPhraseValue via make_prepositional_phrase_sibling_coordinated, prepositional_phrase_sibling_coordinated_parts {
            first: opt hole PrepositionalPhrase,
            list: opt hole PrepositionalPhraseList,
            comma: opt lex Comma,
            conjunction: lex Conjunction,
            next: hole PrepositionalPhrase,
        }
        require any(
            all(first.is_some(), list.is_none(), comma.is_none()),
            all(first.is_none(), list.is_some(), comma.is_some())
        );
        require conjunction in [And, Or, AndOr];
        derive features: Features = reduce_prepositional_phrase_sibling_features(first, list, comma, conjunction, next);
        form pair @ 0 when all(first.is_some(), list.is_none(), comma.is_none()) inverse check(is_prepositional_phrase_sibling_coordinated) = first lex(conjunction) next;
        form oxford @ 1 inverse check(is_prepositional_phrase_sibling_coordinated) otherwise = list lex(comma) lex(conjunction) next;
        selection unique;
    }

    construction verb_phrase_coordinated_adjective: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_coordinated_adjective, verb_phrase_coordinated_adjective_parts {
            predicate: hole VerbPhrase,
            complement: hole CoordinatedAdjectivePhrase,
        }
        derive features: Features = reduce_verb_phrase_coordinated_adjective_features(predicate, complement);
        form only @ 0 inverse check(is_verb_phrase_coordinated_adjective) = predicate complement;
        selection unique;
    }

    construction nominal_power_toughness_complement: NominalPhrase {
        bind NominalPhrase via make_nominal_power_toughness_complement, nominal_power_toughness_complement_parts {
            nominal: hole NominalPhrase,
            stats: hole PowerToughness,
        }
        derive features: Features = reduce_nominal_power_toughness_complement_features(nominal, stats);
        derive base_precedence: Features = reduce_nominal_power_toughness_complement_features(nominal, stats);
        form only @ 0 inverse check(is_nominal_power_toughness_complement) = nominal stats;
        selection unique;
    }

    construction noun_phrase_coordination: NounPhrase {
        own CoordinatedNounPhrase {
            first: hole box NounPhrase,
            rest: seq noun_phrase_member,
        }
        project Coordinated via project_noun_phrase_coordination_source;
        serialize;
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And, Or, Plus, AndOr];
        derive first = complete_noun_phrase_coordination(first, rest);
        form flat @ 0 = first rest;
        dominates prepositional_phrase;
        dominated by noun_phrase_nominal;
    }

    construction shared_determiner_nominal: NounPhrase {
        own CoordinatedNominalPhrase {
            determiner: hole Determiner,
            first: hole box NominalPhrase,
            rest: seq nominal_phrase_member,
            complements: seq nominal_complement,
        }
        project CoordinatedNominal via project_shared_determiner_nominal_source;
        serialize;
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And, Or, AndOr];
        require all(
            complements.first.variant in [Relative],
            complements.nonfinal.variant in [Relative],
            complements.last.variant in [Relative]
        );
        derive first = shared_determiner_coordination(determiner, first, rest, complements);
        form shared @ 0 = determiner first rest complements;
        dominates prepositional_phrase;
    }
}

fn project_noun_phrase_coordination_source(value: &NounPhrase) -> Option<&CoordinatedNounPhrase> {
    match value.kind() {
        NounPhraseKind::Coordinated(value) => Some(value),
        _ => None,
    }
}

fn project_shared_determiner_nominal_source(
    value: &NounPhrase,
) -> Option<&CoordinatedNominalPhrase> {
    match value.kind() {
        NounPhraseKind::CoordinatedNominal(value) => Some(value),
        _ => None,
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_COORDINATION_DECLARATION];

/// Builds the public noun-phrase output of the coordination declaration.
pub(crate) fn build_noun_phrase_coordination_value(
    first: Box<NounPhrase>,
    rest: Vec<NounPhraseCoordination>,
) -> Result<NounPhrase, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    CoordinatedNounPhrase::try_new(first, rest).map(NounPhrase::from_coordination_declaration)
}

#[cfg(test)]
pub(crate) fn build_noun_phrase_coordination(
    first: Box<NounPhrase>,
    rest: Vec<NounPhraseCoordination>,
) -> Result<CoordinatedNounPhrase, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    CoordinatedNounPhrase::try_new(first, rest)
}

#[cfg(test)]
pub(crate) fn build_shared_determiner_nominal(
    determiner: Determiner,
    first: Box<NominalPhrase>,
    rest: Vec<NominalPhraseCoordination>,
    complements: Vec<NominalComplement>,
) -> Result<CoordinatedNominalPhrase, deckmaste_construction_compiler::runtime::DeclarationViolation>
{
    CoordinatedNominalPhrase::try_new(determiner, first, rest, complements)
}

impl Clone for CoordinatedNounPhrase {
    fn clone(&self) -> Self {
        Self::try_new(self.first().clone(), self.rest().clone())
            .expect("an existing coordinated noun phrase satisfies its declaration")
    }
}

impl Clone for CoordinatedNominalPhrase {
    fn clone(&self) -> Self {
        Self::try_new(
            self.determiner().clone(),
            self.first().clone(),
            self.rest().clone(),
            self.complements().clone(),
        )
        .expect("an existing coordinated nominal phrase satisfies its declaration")
    }
}
