//! Declaration-owned ability-backend construction roots.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::catalog::CatalogAtom;
use crate::syntax::Ability;
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
use crate::syntax::KeywordListSeparator;
use crate::syntax::LevelBandAbility;
use crate::syntax::LevelRange;
use crate::syntax::LoyaltyAbility;
use crate::syntax::ModalAbility;
use crate::syntax::Paragraph;
use crate::syntax::RollRange;
use crate::syntax::RollRowAbility;
use crate::syntax::StationThresholdAbility;
use crate::syntax::TriggeredAbility;

type Violation = deckmaste_construction_compiler::runtime::DeclarationViolation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AbilityHeader {
    AbilityWord(CatalogAtom),
    Flavor(FlavorHeader),
}

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

fn cost_from_parts(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    if components.is_empty() {
        return Err(violation("cost", "at least one cost component"));
    }
    Ok(Cost::from_parts(&ABILITY_OWNER, flavor_header, components))
}

fn cost_parts(value: &Cost) -> (Option<FlavorHeader>, Vec<CostComponent>) {
    (value.flavor_header().cloned(), value.components().to_vec())
}

fn keyword_line_from_parts(
    first: Vec<KeywordAbility>,
    rest: Vec<KeywordAbility>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    let mut abilities = Vec::with_capacity(rest.len() + 1);
    abilities.extend(first);
    abilities.extend(rest);
    if !keyword_line_has_valid_separator_topology(&abilities) {
        return Err(violation(
            "keyword_line",
            "first keyword has no separator and every later keyword has one",
        ));
    }
    Ok(KeywordAbilityList::from_parts(
        &ABILITY_OWNER,
        abilities,
        trailing,
    ))
}

fn keyword_line_parts(
    value: &KeywordAbilityList,
) -> (Vec<KeywordAbility>, Vec<KeywordAbility>, Option<Paragraph>) {
    let (first, rest) = value
        .abilities()
        .split_first()
        .expect("declaration-owned keyword lines are nonempty");
    (
        vec![first.clone()],
        rest.to_vec(),
        value.trailing().cloned(),
    )
}

fn keyword_line_has_valid_separator_topology(abilities: &[KeywordAbility]) -> bool {
    let Some((first, rest)) = abilities.split_first() else {
        return false;
    };
    first.preceding_separator.is_none()
        && rest.iter().all(|ability| {
            matches!(
                ability.preceding_separator,
                Some(KeywordListSeparator::Comma | KeywordListSeparator::Semicolon)
            )
        })
}

#[allow(
    clippy::too_many_arguments,
    reason = "the declaration enumerates all eleven AbilityKind payloads"
)]
fn ability_from_parts(
    header: Option<AbilityHeader>,
    activated: Option<ActivatedAbility>,
    class_level: Option<ClassLevelAbility>,
    chapter: Option<ChapterAbility>,
    roll_row: Option<RollRowAbility>,
    level_band: Option<LevelBandAbility>,
    station_threshold: Option<StationThresholdAbility>,
    triggered: Option<TriggeredAbility>,
    loyalty: Option<LoyaltyAbility>,
    modal: Option<ModalAbility>,
    keyword: Option<KeywordAbilityList>,
    paragraph: Option<Paragraph>,
) -> Result<Ability, Violation> {
    let mut kinds = Vec::new();
    kinds.extend(activated.map(AbilityKind::Activated));
    kinds.extend(class_level.map(AbilityKind::ClassLevel));
    kinds.extend(chapter.map(AbilityKind::Chapter));
    kinds.extend(roll_row.map(AbilityKind::RollRow));
    kinds.extend(level_band.map(AbilityKind::LevelBand));
    kinds.extend(station_threshold.map(AbilityKind::StationThreshold));
    kinds.extend(triggered.map(AbilityKind::Triggered));
    kinds.extend(loyalty.map(AbilityKind::Loyalty));
    kinds.extend(modal.map(AbilityKind::Modal));
    kinds.extend(keyword.map(AbilityKind::Keyword));
    kinds.extend(paragraph.map(AbilityKind::Paragraph));
    let [kind] = kinds
        .try_into()
        .map_err(|_| violation("ability", "exactly one ability kind"))?;
    validate_kind(&kind)?;
    let (ability_word, flavor_header) = match header {
        Some(AbilityHeader::AbilityWord(word)) => (Some(word), None),
        Some(AbilityHeader::Flavor(header)) => (None, Some(header)),
        None => (None, None),
    };
    Ok(Ability::from_parts(
        &ABILITY_OWNER,
        ability_word,
        flavor_header,
        kind,
    ))
}

#[allow(
    clippy::type_complexity,
    reason = "the tuple is the generated declaration projection"
)]
fn ability_parts(
    value: &Ability,
) -> (
    Option<AbilityHeader>,
    Option<ActivatedAbility>,
    Option<ClassLevelAbility>,
    Option<ChapterAbility>,
    Option<RollRowAbility>,
    Option<LevelBandAbility>,
    Option<StationThresholdAbility>,
    Option<TriggeredAbility>,
    Option<LoyaltyAbility>,
    Option<ModalAbility>,
    Option<KeywordAbilityList>,
    Option<Paragraph>,
) {
    let header = value
        .ability_word()
        .cloned()
        .map(AbilityHeader::AbilityWord)
        .or_else(|| value.flavor_header().cloned().map(AbilityHeader::Flavor));
    let mut activated = None;
    let mut class_level = None;
    let mut chapter = None;
    let mut roll_row = None;
    let mut level_band = None;
    let mut station_threshold = None;
    let mut triggered = None;
    let mut loyalty = None;
    let mut modal = None;
    let mut keyword = None;
    let mut paragraph = None;
    match value.kind().clone() {
        AbilityKind::Activated(value) => activated = Some(value),
        AbilityKind::ClassLevel(value) => class_level = Some(value),
        AbilityKind::Chapter(value) => chapter = Some(value),
        AbilityKind::RollRow(value) => roll_row = Some(value),
        AbilityKind::LevelBand(value) => level_band = Some(value),
        AbilityKind::StationThreshold(value) => station_threshold = Some(value),
        AbilityKind::Triggered(value) => triggered = Some(value),
        AbilityKind::Loyalty(value) => loyalty = Some(value),
        AbilityKind::Modal(value) => modal = Some(value),
        AbilityKind::Keyword(value) => keyword = Some(value),
        AbilityKind::Paragraph(value) => paragraph = Some(value),
    }
    (
        header,
        activated,
        class_level,
        chapter,
        roll_row,
        level_band,
        station_threshold,
        triggered,
        loyalty,
        modal,
        keyword,
        paragraph,
    )
}

fn paragraph_is_nonempty(value: &Paragraph) -> bool {
    !value.sentences.is_empty()
}

fn validate_kind(kind: &AbilityKind) -> Result<(), Violation> {
    let valid_range = |low: i32, high: i32| low <= high;
    let valid = match kind {
        AbilityKind::Activated(value) => {
            !value.cost.components().is_empty() && paragraph_is_nonempty(&value.effect)
        }
        AbilityKind::ClassLevel(value) => {
            !value.cost.components().is_empty() && value.level.value > 0
        }
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
        AbilityKind::StationThreshold(_) => true,
        AbilityKind::Triggered(value) => paragraph_is_nonempty(&value.effect),
        AbilityKind::Loyalty(value) => paragraph_is_nonempty(&value.effect),
        AbilityKind::Modal(value) => {
            !value.modes.is_empty()
                && value
                    .modes
                    .iter()
                    .all(|mode| paragraph_is_nonempty(&mode.body))
        }
        AbilityKind::Keyword(value) => keyword_line_has_valid_separator_topology(value.abilities()),
        AbilityKind::Paragraph(value) => paragraph_is_nonempty(value),
    };
    valid
        .then_some(())
        .ok_or_else(|| violation("ability", "valid ability frame payload"))
}

pub(crate) fn kind_is_valid(kind: &AbilityKind) -> bool {
    validate_kind(kind).is_ok()
}

deckmaste_constructions_macro::constructions! {
    group ability backend ability;

    element cost_component bind CostComponent {}
    element keyword_ability bind KeywordAbility {
        preceding_separator: opt lex KeywordListSeparator,
        ability: identity CatalogAtom via AbilityItem,
        argument: hole KeywordArgument,
    }

    construction cost: Cost {
        bind Cost via cost_from_parts, cost_parts {
            flavor_header: opt hole FlavorHeader,
            components: seq cost_component,
        }
        require components.len() >= 1;
        evidence role "activation-cost root" from category;
        form header @ 0 when flavor_header.is_some() = flavor_header "—" components;
        form plain @ 1 otherwise = components;
        selection unique;
    }

    construction keyword_line: KeywordAbilityList {
        bind KeywordAbilityList via keyword_line_from_parts, keyword_line_parts {
            first: seq keyword_ability,
            rest: seq keyword_ability,
            trailing: opt hole Paragraph,
        }
        require first.len() == 1;
        require first.first.preceding_separator.is_none();
        require rest.first.preceding_separator.is_some();
        require rest.nonfinal.preceding_separator.is_some();
        require rest.last.preceding_separator.is_some();
        evidence role "keyword-ability list root" from category;
        form trailing @ 0 when trailing.is_some() = first rest trailing;
        form plain @ 1 otherwise = first rest;
        selection unique;
    }

    construction ability: Ability {
        bind Ability via ability_from_parts, ability_parts {
            header: opt hole AbilityHeader,
            activated: opt hole ActivatedAbility,
            class_level: opt hole ClassLevelAbility,
            chapter: opt hole ChapterAbility,
            roll_row: opt hole RollRowAbility,
            level_band: opt hole LevelBandAbility,
            station_threshold: opt hole StationThresholdAbility,
            triggered: opt hole TriggeredAbility,
            loyalty: opt hole LoyaltyAbility,
            modal: opt hole ModalAbility,
            keyword: opt hole KeywordAbilityList,
            paragraph: opt hole Paragraph,
        }
        require any(activated.is_some(), class_level.is_some(), chapter.is_some(), roll_row.is_some(), level_band.is_some(), station_threshold.is_some(), triggered.is_some(), loyalty.is_some(), modal.is_some(), keyword.is_some(), paragraph.is_some());
        evidence guard "decisive ability frame guard" from requirement activated;
        form activated @ 0 when all(activated.is_some(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header activated;
        form class_level @ 1 when all(activated.is_none(), class_level.is_some(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header class_level;
        form chapter @ 2 when all(activated.is_none(), class_level.is_none(), chapter.is_some(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header chapter;
        form roll_row @ 3 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_some(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header roll_row;
        form level_band @ 4 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_some(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header level_band;
        form station_threshold @ 5 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_some(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header station_threshold;
        form triggered @ 6 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_some(), loyalty.is_none(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header triggered;
        form loyalty @ 7 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_some(), modal.is_none(), keyword.is_none(), paragraph.is_none()) = header loyalty;
        form modal @ 8 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_some(), keyword.is_none(), paragraph.is_none()) = header modal;
        form keyword @ 9 when all(activated.is_none(), class_level.is_none(), chapter.is_none(), roll_row.is_none(), level_band.is_none(), station_threshold.is_none(), triggered.is_none(), loyalty.is_none(), modal.is_none(), keyword.is_some(), paragraph.is_none()) = header keyword;
        form paragraph @ 10 otherwise = header paragraph;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&ABILITY_DECLARATION];

pub(crate) fn build(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    build_cost(flavor_header, components)
}

pub(crate) fn parts(value: &Cost) -> Result<(Option<FlavorHeader>, Vec<CostComponent>), Violation> {
    if value.components().is_empty() {
        return Err(violation("cost", "at least one cost component"));
    }
    Ok(parts_cost(value))
}

pub(crate) fn build_keyword_list(
    abilities: Vec<KeywordAbility>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    let mut abilities = abilities.into_iter();
    let first = abilities
        .next()
        .ok_or_else(|| violation("keyword_line", "at least one keyword ability"))?;
    build_keyword_line(vec![first], abilities.collect(), trailing)
}

pub(crate) fn keyword_list_parts(
    value: &KeywordAbilityList,
) -> Result<(Vec<KeywordAbility>, Option<Paragraph>), Violation> {
    if !keyword_line_has_valid_separator_topology(value.abilities()) {
        return Err(violation(
            "keyword_line",
            "first keyword has no separator and every later keyword has one",
        ));
    }
    let (first, rest, trailing) = parts_keyword_line(value);
    Ok((first.into_iter().chain(rest).collect(), trailing))
}

#[derive(Default)]
struct AbilityKindParts {
    activated: Option<ActivatedAbility>,
    class_level: Option<ClassLevelAbility>,
    chapter: Option<ChapterAbility>,
    roll_row: Option<RollRowAbility>,
    level_band: Option<LevelBandAbility>,
    station_threshold: Option<StationThresholdAbility>,
    triggered: Option<TriggeredAbility>,
    loyalty: Option<LoyaltyAbility>,
    modal: Option<ModalAbility>,
    keyword: Option<KeywordAbilityList>,
    paragraph: Option<Paragraph>,
}

impl From<AbilityKind> for AbilityKindParts {
    fn from(kind: AbilityKind) -> Self {
        match kind {
            AbilityKind::Activated(value) => Self {
                activated: Some(value),
                ..Self::default()
            },
            AbilityKind::ClassLevel(value) => Self {
                class_level: Some(value),
                ..Self::default()
            },
            AbilityKind::Chapter(value) => Self {
                chapter: Some(value),
                ..Self::default()
            },
            AbilityKind::RollRow(value) => Self {
                roll_row: Some(value),
                ..Self::default()
            },
            AbilityKind::LevelBand(value) => Self {
                level_band: Some(value),
                ..Self::default()
            },
            AbilityKind::StationThreshold(value) => Self {
                station_threshold: Some(value),
                ..Self::default()
            },
            AbilityKind::Triggered(value) => Self {
                triggered: Some(value),
                ..Self::default()
            },
            AbilityKind::Loyalty(value) => Self {
                loyalty: Some(value),
                ..Self::default()
            },
            AbilityKind::Modal(value) => Self {
                modal: Some(value),
                ..Self::default()
            },
            AbilityKind::Keyword(value) => Self {
                keyword: Some(value),
                ..Self::default()
            },
            AbilityKind::Paragraph(value) => Self {
                paragraph: Some(value),
                ..Self::default()
            },
        }
    }
}

pub(crate) fn build_ability_root(
    ability_word: Option<CatalogAtom>,
    flavor_header: Option<FlavorHeader>,
    kind: AbilityKind,
) -> Result<Ability, Violation> {
    let header = match (ability_word, flavor_header) {
        (Some(_), Some(_)) => {
            return Err(violation(
                "ability",
                "ability-word and flavor-word headers are exclusive",
            ));
        }
        (Some(word), None) => Some(AbilityHeader::AbilityWord(word)),
        (None, Some(header)) => Some(AbilityHeader::Flavor(header)),
        (None, None) => None,
    };
    let AbilityKindParts {
        activated,
        class_level,
        chapter,
        roll_row,
        level_band,
        station_threshold,
        triggered,
        loyalty,
        modal,
        keyword,
        paragraph,
    } = kind.into();
    build_ability(
        header,
        activated,
        class_level,
        chapter,
        roll_row,
        level_band,
        station_threshold,
        triggered,
        loyalty,
        modal,
        keyword,
        paragraph,
    )
}

pub(crate) fn ability_root_parts(
    value: &Ability,
) -> Result<(Option<CatalogAtom>, Option<FlavorHeader>, AbilityKind), Violation> {
    validate_kind(value.kind())?;
    if value.ability_word().is_some() && value.flavor_header().is_some() {
        return Err(violation(
            "ability",
            "ability-word and flavor-word headers are exclusive",
        ));
    }
    Ok((
        value.ability_word().cloned(),
        value.flavor_header().cloned(),
        value.kind().clone(),
    ))
}

#[derive(Default)]
struct FormOrdinal(Option<u16>);

impl deckmaste_construction_compiler::runtime::LinearizationVisitor for FormOrdinal {
    type Error = std::convert::Infallible;

    fn begin_form(
        &mut self,
        construction: &'static str,
        _form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        assert_eq!(construction, "ability");
        self.0 = Some(ordinal);
        Ok(())
    }

    fn literal(&mut self, _literal: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        _category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        _codec: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub(crate) fn ability_form_ordinal(value: &Ability) -> Result<u16, Violation> {
    let mut visitor = FormOrdinal::default();
    linearize_ability_with(value, &mut visitor)
        .map_err(|_| violation("ability", "exactly one declared ability form"))?;
    visitor
        .0
        .ok_or_else(|| violation("ability", "exactly one declared ability form"))
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
    fn every_raw_ability_root_constructor_requires_the_declaration_owner() {
        let _: fn(
            &super::AbilityOwner,
            Option<crate::syntax::FlavorHeader>,
            Vec<crate::syntax::CostComponent>,
        ) -> crate::syntax::Cost = crate::syntax::Cost::from_parts;
        let _: fn(
            &super::AbilityOwner,
            Vec<crate::syntax::KeywordAbility>,
            Option<crate::syntax::Paragraph>,
        ) -> crate::syntax::KeywordAbilityList = crate::syntax::KeywordAbilityList::from_parts;
        let _: fn(
            &super::AbilityOwner,
            Option<crate::catalog::CatalogAtom>,
            Option<crate::syntax::FlavorHeader>,
            crate::syntax::AbilityKind,
        ) -> crate::syntax::Ability = crate::syntax::Ability::from_parts;
    }

    #[test]
    fn keyword_line_projection_revalidates_separator_topology() {
        let malformed = crate::syntax::KeywordAbilityList::from_parts(
            &super::ABILITY_OWNER,
            vec![
                crate::syntax::KeywordAbility {
                    preceding_separator: None,
                    ability: keyword("Flying"),
                    argument: crate::syntax::KeywordArgument::Absent,
                },
                crate::syntax::KeywordAbility {
                    preceding_separator: None,
                    ability: keyword("First strike"),
                    argument: crate::syntax::KeywordArgument::Absent,
                },
            ],
            None,
        );
        assert!(super::keyword_list_parts(&malformed).is_err());
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
}
