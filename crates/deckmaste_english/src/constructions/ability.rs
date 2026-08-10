//! Declaration-owned ability-backend construction roots.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::FlavorHeader;

fn cost_from_parts(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    Ok(Cost::from_parts(flavor_header, components))
}

fn cost_parts(value: &Cost) -> (Option<FlavorHeader>, Vec<CostComponent>) {
    (value.flavor_header().cloned(), value.components().to_vec())
}

deckmaste_constructions_macro::constructions! {
    group ability backend ability;

    element cost_component bind CostComponent {}

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
