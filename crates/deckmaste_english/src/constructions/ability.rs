//! Declaration-owned ability-backend construction roots.

use deckmaste_construction_compiler::runtime::GroupData;
use deckmaste_construction_compiler::runtime::NonEmpty;
use deckmaste_construction_compiler::runtime::SeparatedNonEmpty;

#[allow(
    unused_imports,
    reason = "the construction DSL names this scalar codec in generated tokens"
)]
use crate::Numeral;
use crate::catalog::CatalogAtom;
use crate::syntax::Ability;
use crate::syntax::AbilityHeader;
use crate::syntax::AbilityKind;
use crate::syntax::ActivatedAbility;
use crate::syntax::ChapterAbility;
use crate::syntax::ClassLevelAbility;
use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::FlavorHeader;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordArgumentSeparator;
use crate::syntax::KeywordCost;
use crate::syntax::KeywordListSeparator;
use crate::syntax::LevelBandAbility;
use crate::syntax::LevelRange;
use crate::syntax::LoyaltyAbility;
use crate::syntax::ModalAbility;
use crate::syntax::NounPhrase;
use crate::syntax::NumberLiteral;
use crate::syntax::OracleSymbol;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::PowerToughness;
use crate::syntax::PredicatedArgument;
use crate::syntax::Preposition;
use crate::syntax::QuotedAbility;
use crate::syntax::RecoveredText;
use crate::syntax::RollRange;
use crate::syntax::RollRowAbility;
use crate::syntax::StationThresholdAbility;
use crate::syntax::TriggeredAbility;

type SymbolSequence = Vec<OracleSymbol>;

type Violation = deckmaste_construction_compiler::runtime::DeclarationViolation;

pub(crate) struct AbilityOwner {
    _private: (),
}

const ABILITY_OWNER: AbilityOwner = AbilityOwner { _private: () };

fn violation(construction: &'static str, requirement: &'static str) -> Violation {
    Violation {
        construction,
        requirement,
    }
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "generated bind adapters use a fallible signature; NonEmpty proves this constructor's only invariant"
)]
fn cost_from_parts(
    flavor_header: Option<FlavorHeader>,
    components: NonEmpty<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    Ok(Cost::from_parts(&ABILITY_OWNER, flavor_header, components))
}

fn cost_parts(value: &Cost) -> (Option<FlavorHeader>, NonEmpty<CostComponent>) {
    (
        value.flavor_header().cloned(),
        value.component_sequence().clone(),
    )
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "generated bind adapters use a fallible signature; SeparatedNonEmpty proves this constructor's topology"
)]
fn keyword_line_from_parts(
    abilities: SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    Ok(KeywordAbilityList::from_parts(
        &ABILITY_OWNER,
        abilities,
        trailing,
    ))
}

fn keyword_line_parts(
    value: &KeywordAbilityList,
) -> (
    SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
    Option<Paragraph>,
) {
    (
        value.separated_abilities().clone(),
        value.trailing().cloned(),
    )
}

fn ability_from_parts(
    header: Option<AbilityHeader>,
    kind: AbilityKind,
) -> Result<Ability, Violation> {
    validate_kind(&kind)?;
    Ok(Ability::from_parts(&ABILITY_OWNER, header, kind))
}

fn ability_parts(value: &Ability) -> (Option<AbilityHeader>, AbilityKind) {
    (value.header().cloned(), value.kind().clone())
}

fn paragraph_is_nonempty(value: &Paragraph) -> bool {
    !value.sentences.is_empty()
}

fn validate_kind(kind: &AbilityKind) -> Result<(), Violation> {
    let valid_range = |low: i32, high: i32| low <= high;
    let valid = match kind {
        AbilityKind::Activated(value) => paragraph_is_nonempty(&value.effect),
        AbilityKind::ClassLevel(value) => value.level.value > 0,
        AbilityKind::Chapter(value) => {
            !value.chapters.is_empty() && paragraph_is_nonempty(&value.body)
        }
        AbilityKind::RollRow(value) => {
            let range = match value.range {
                RollRange::Inclusive { low, high } => valid_range(low.value, high.value),
                RollRange::Single(_) | RollRange::OrMore(_) | RollRange::OrLess(_) => true,
            };
            range && paragraph_is_nonempty(&value.body)
        }
        AbilityKind::LevelBand(value) => match value.range {
            LevelRange::Band { low, high } => valid_range(low.value, high.value),
            LevelRange::AtLeast(_) => true,
        },
        AbilityKind::StationThreshold(_) | AbilityKind::Keyword(_) => true,
        AbilityKind::Triggered(value) => paragraph_is_nonempty(&value.effect),
        AbilityKind::Loyalty(value) => paragraph_is_nonempty(&value.effect),
        AbilityKind::Modal(value) => {
            !value.modes.is_empty()
                && value
                    .modes
                    .iter()
                    .all(|mode| paragraph_is_nonempty(&mode.body))
        }
        AbilityKind::Paragraph(value) => paragraph_is_nonempty(value),
    };
    valid
        .then_some(())
        .ok_or_else(|| violation("ability", "valid ability frame payload"))
}

deckmaste_constructions_macro::constructions! {
    group ability backend ability;

    element cost_component bind CostComponent {
        variant Symbols: hole Vec<OracleSymbol>,
        variant Clause: hole box crate::syntax::IndependentClause via Clause,
        variant Noun: hole box NounPhrase,
        variant Alternative(
            left: sum box cost_component,
            right: sum box cost_component,
        ),
        variant Recovered: hole RecoveredText,
    }
    element quoted_ability bind QuotedAbility {
        ability: hole box Ability,
        initial_uppercase: lex bool,
    }
    element phrase_argument bind Phrase {
        variant Clause: hole box crate::syntax::Clause via Clause,
        variant NounPhrase: hole box NounPhrase via NounPhrase,
        variant AdjectivePhrase: hole box crate::syntax::AdjectivePhrase via AdjectivePhrase,
        variant PrepositionalPhrase:
            hole box crate::syntax::PrepositionalPhrase via PrepositionalPhrase,
        variant Quantity: hole crate::syntax::Quantity via Quantity,
        variant Adverb: identity crate::word::Vocab via Adverb,
        variant CatalogAtom: identity CatalogAtom via CatalogAtom,
        variant ColorWord: lex crate::word::ColorWord,
        variant Cost: hole Cost via Cost,
        variant ThisCard: identity crate::syntax::ThisCardForm via ThisCard,
        variant OracleSymbol: identity OracleSymbol via OracleSymbol,
        variant SymbolSequence: identity SymbolSequence via SymbolSequence,
        variant NumberLiteral: lex NumberLiteral via Numeral,
        variant SignedScalar: lex crate::syntax::SignedScalar via SignedScalar,
        variant PowerToughness: lex PowerToughness via PowerToughness,
        variant EmbeddedAbility: hole box Ability via Ability,
        variant QuotedAbility: product box quoted_ability,
        variant Recovered: hole RecoveredText,
    }
    element predicated_quality bind crate::syntax::PredicatedQuality {
        preposition: opt lex Preposition,
        quality: sum phrase_argument,
    }
    element predicated_argument bind PredicatedArgument {
        qualities: seq predicated_quality,
    }
    element keyword_cost bind KeywordCost {
        variant Symbols: hole Vec<OracleSymbol>,
        variant Sentence {
            ability: hole box Ability,
        },
        variant Components {
            cost: hole Cost,
            terminal: lex bool,
        },
    }
    element keyword_argument bind KeywordArgument {
        variant Absent {},
        variant Counted: hole crate::syntax::Quantity,
        variant Costed: sum keyword_cost,
        variant CountedCost {
            count: lex NumberLiteral via Numeral,
            symbols: hole Vec<OracleSymbol>,
        },
        variant Predicated: product predicated_argument,
        variant Qualified: sum phrase_argument,
        variant Statted {
            symbols: hole Vec<OracleSymbol>,
            stats: lex PowerToughness via PowerToughness,
        },
        variant Named {
            separator: lex KeywordArgumentSeparator,
            label: lex String,
        },
        variant Recovered {
            text: hole RecoveredText,
        },
        variant RestrictedCost {
            preposition: opt lex Preposition,
            restriction: hole box NounPhrase,
            cost: sum keyword_cost,
        },
    }
    element keyword_ability bind KeywordAbility {
        ability: identity CatalogAtom via AbilityItem,
        argument: sum keyword_argument,
    }
    element ability_kind bind AbilityKind {
        variant Activated: hole ActivatedAbility,
        variant ClassLevel: hole ClassLevelAbility,
        variant Chapter: hole ChapterAbility,
        variant RollRow: hole RollRowAbility,
        variant LevelBand: hole LevelBandAbility,
        variant StationThreshold: hole StationThresholdAbility,
        variant Triggered: hole TriggeredAbility,
        variant Loyalty: hole LoyaltyAbility,
        variant Modal: hole ModalAbility,
        variant Keyword: hole KeywordAbilityList,
        variant Paragraph: hole Paragraph,
    }

    construction cost: Cost {
        bind Cost via cost_from_parts, cost_parts {
            flavor_header: opt hole FlavorHeader,
            components: nonempty seq cost_component,
        }
        evidence role "activation-cost root" from category;
        form header @ 0 when flavor_header.is_some() = flavor_header "—" components;
        form plain @ 1 otherwise = components;
        selection unique;
    }

    construction keyword_line: KeywordAbilityList {
        bind KeywordAbilityList via keyword_line_from_parts, keyword_line_parts {
            abilities: nonempty seq keyword_ability separated by lex KeywordListSeparator,
            trailing: opt hole Paragraph,
        }
        evidence role "keyword-ability list root" from category;
        form trailing @ 0 when trailing.is_some() = abilities trailing;
        form plain @ 1 otherwise = abilities;
        selection unique;
    }

    construction ability: Ability {
        bind Ability via ability_from_parts, ability_parts {
            header: opt hole AbilityHeader,
            kind: sum ability_kind,
        }
        evidence guard "decisive ability frame guard" from field kind;
        form activated @ 0 when kind.variant in [Activated] = header kind;
        form class_level @ 1 when kind.variant in [ClassLevel] = header kind;
        form chapter @ 2 when kind.variant in [Chapter] = header kind;
        form roll_row @ 3 when kind.variant in [RollRow] = header kind;
        form level_band @ 4 when kind.variant in [LevelBand] = header kind;
        form station_threshold @ 5 when kind.variant in [StationThreshold] = header kind;
        form triggered @ 6 when kind.variant in [Triggered] = header kind;
        form loyalty @ 7 when kind.variant in [Loyalty] = header kind;
        form modal @ 8 when kind.variant in [Modal] = header kind;
        form keyword @ 9 when kind.variant in [Keyword] = header kind;
        form paragraph @ 10 when kind.variant in [Paragraph] = header kind;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&ABILITY_DECLARATION];

pub(crate) fn build(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    let components = NonEmpty::try_from(components)
        .map_err(|_| violation("cost", "at least one cost component"))?;
    build_cost(flavor_header, components)
}

pub(crate) fn build_keyword_list(
    abilities: SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    build_keyword_line(abilities, trailing)
}

pub(crate) fn build_ability_root(
    header: Option<AbilityHeader>,
    kind: AbilityKind,
) -> Result<Ability, Violation> {
    build_ability(header, kind)
}

#[cfg(test)]
mod tests {
    fn keyword(spelling: &str) -> crate::catalog::CatalogAtom {
        let catalogs = crate::catalog::Catalogs::default()
            .with_catalog(crate::catalog::CatalogKind::KeywordAbility, [spelling]);
        let matched = catalogs
            .matches(spelling, crate::catalog::CatalogSlot::AbilityItem)
            .into_iter()
            .next()
            .expect("the test keyword is cataloged");
        let crate::catalog::CatalogValue::Atom(atom) = matched.value else {
            panic!("an ability-item catalog match retains an atom")
        };
        atom
    }

    #[test]
    fn ability_declaration_exposes_stable_family_order() {
        assert_eq!(
            super::ABILITY_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            ["cost", "keyword_line", "ability"],
        );
    }

    #[test]
    fn ability_declaration_owns_one_direct_sum_field() {
        let ability = super::ABILITY_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "ability")
            .expect("the ability construction is declared");
        assert_eq!(
            ability
                .fields
                .iter()
                .map(|field| field.name)
                .collect::<Vec<_>>(),
            ["header", "kind"],
        );
        assert_eq!(ability.forms.len(), 11);
        let kind = super::ABILITY_DECLARATION
            .element_data
            .iter()
            .find(|element| element.name == "ability_kind")
            .expect("the semantic ability-kind sum is declared");
        assert_eq!(kind.variants.len(), 11);
    }

    #[test]
    fn phrase_argument_declaration_exhausts_the_bound_phrase_sum() {
        let phrase = super::ABILITY_DECLARATION
            .element_data
            .iter()
            .find(|element| element.name == "phrase_argument")
            .expect("keyword phrase arguments are declared as a semantic sum");
        assert_eq!(
            phrase
                .variants
                .iter()
                .map(|variant| variant.name)
                .collect::<Vec<_>>(),
            [
                "Clause",
                "NounPhrase",
                "AdjectivePhrase",
                "PrepositionalPhrase",
                "Quantity",
                "Adverb",
                "CatalogAtom",
                "ColorWord",
                "Cost",
                "ThisCard",
                "OracleSymbol",
                "SymbolSequence",
                "NumberLiteral",
                "SignedScalar",
                "PowerToughness",
                "EmbeddedAbility",
                "QuotedAbility",
                "Recovered",
            ],
        );
    }

    #[test]
    fn checked_sequence_types_carry_nonempty_and_separator_invariants() {
        assert!(super::build(None, Vec::new()).is_err());

        let first = crate::syntax::KeywordAbility {
            ability: keyword("Flying"),
            argument: crate::syntax::KeywordArgument::Absent,
        };
        let second = crate::syntax::KeywordAbility {
            ability: keyword("First strike"),
            argument: crate::syntax::KeywordArgument::Absent,
        };
        let abilities = crate::syntax::SeparatedNonEmpty::new(
            first,
            vec![crate::syntax::Separated::new(
                crate::syntax::KeywordListSeparator::Semicolon,
                second,
            )],
        );
        let list = super::build_keyword_list(abilities, None)
            .expect("a typed separator edge satisfies keyword-line ingress");
        assert_eq!(list.len(), 2);
        assert_eq!(
            list.separated_abilities().rest()[0].separator(),
            &crate::syntax::KeywordListSeparator::Semicolon,
        );
    }
}
