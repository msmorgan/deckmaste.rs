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

fn violation(requirement: &'static str) -> Violation {
    Violation {
        construction: "ability",
        requirement,
    }
}

fn cost_from_parts(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    Ok(Cost::from_parts(flavor_header, components))
}

fn cost_parts(value: &Cost) -> (Option<FlavorHeader>, Vec<CostComponent>) {
    (value.flavor_header().cloned(), value.components().to_vec())
}

fn keyword_line_from_parts(
    abilities: Vec<KeywordAbility>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    Ok(KeywordAbilityList::from_parts(abilities, trailing))
}

fn keyword_line_parts(value: &KeywordAbilityList) -> (Vec<KeywordAbility>, Option<Paragraph>) {
    (value.abilities().to_vec(), value.trailing().cloned())
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
        .map_err(|_| violation("exactly one ability kind"))?;
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
        AbilityKind::Keyword(value) => !value.abilities().is_empty(),
        AbilityKind::Paragraph(value) => paragraph_is_nonempty(value),
    };
    valid
        .then_some(())
        .ok_or_else(|| violation("valid ability frame payload"))
}

pub(crate) fn kind_is_valid(kind: &AbilityKind) -> bool {
    validate_kind(kind).is_ok()
}

deckmaste_constructions_macro::constructions! {
    group ability backend ability;

    element cost_component bind CostComponent {}
    element keyword_ability bind KeywordAbility {}

    construction cost: Cost {
        bind Cost via cost_from_parts, cost_parts {
            flavor_header: opt hole FlavorHeader,
            components: seq cost_component,
        }
        evidence role "activation-cost root" from category;
        form header @ 0 when flavor_header.is_some() = flavor_header "—" components;
        form plain @ 1 otherwise = components;
        selection unique;
    }

    construction keyword_line: KeywordAbilityList {
        bind KeywordAbilityList via keyword_line_from_parts, keyword_line_parts {
            abilities: seq keyword_ability,
            trailing: opt hole Paragraph,
        }
        require abilities.len() >= 1;
        evidence role "keyword-ability list root" from category;
        form trailing @ 0 when trailing.is_some() = abilities trailing;
        form plain @ 1 otherwise = abilities;
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

pub(crate) fn parts(value: &Cost) -> (Option<FlavorHeader>, Vec<CostComponent>) {
    parts_cost(value)
}

pub(crate) fn build_keyword_list(
    abilities: Vec<KeywordAbility>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    build_keyword_line(abilities, trailing)
}

pub(crate) fn keyword_list_parts(
    value: &KeywordAbilityList,
) -> (Vec<KeywordAbility>, Option<Paragraph>) {
    parts_keyword_line(value)
}

pub(crate) fn build_ability_root(
    ability_word: Option<CatalogAtom>,
    flavor_header: Option<FlavorHeader>,
    kind: AbilityKind,
) -> Result<Ability, Violation> {
    let header = match (ability_word, flavor_header) {
        (Some(_), Some(_)) => {
            return Err(violation(
                "ability-word and flavor-word headers are exclusive",
            ));
        }
        (Some(word), None) => Some(AbilityHeader::AbilityWord(word)),
        (None, Some(header)) => Some(AbilityHeader::Flavor(header)),
        (None, None) => None,
    };
    let (
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
    ) = match kind {
        AbilityKind::Activated(value) => (
            Some(value),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::ClassLevel(value) => (
            None,
            Some(value),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::Chapter(value) => (
            None,
            None,
            Some(value),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::RollRow(value) => (
            None,
            None,
            None,
            Some(value),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::LevelBand(value) => (
            None,
            None,
            None,
            None,
            Some(value),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::StationThreshold(value) => (
            None,
            None,
            None,
            None,
            None,
            Some(value),
            None,
            None,
            None,
            None,
            None,
        ),
        AbilityKind::Triggered(value) => (
            None,
            None,
            None,
            None,
            None,
            None,
            Some(value),
            None,
            None,
            None,
            None,
        ),
        AbilityKind::Loyalty(value) => (
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(value),
            None,
            None,
            None,
        ),
        AbilityKind::Modal(value) => (
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(value),
            None,
            None,
        ),
        AbilityKind::Keyword(value) => (
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(value),
            None,
        ),
        AbilityKind::Paragraph(value) => (
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(value),
        ),
    };
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
        .map_err(|_| violation("exactly one declared ability form"))?;
    visitor
        .0
        .ok_or_else(|| violation("exactly one declared ability form"))
}

#[cfg(test)]
mod tests {
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
