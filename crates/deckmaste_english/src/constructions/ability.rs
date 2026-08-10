//! Declaration-owned ability-backend construction roots.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::FlavorHeader;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::Paragraph;

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
            ["cost", "keyword_line"],
        );
    }
}
