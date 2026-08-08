//! Construction-group declarations and test-only compiler scaffolding.

pub(crate) mod coordination;
#[cfg(test)]
pub(crate) mod law;
pub(crate) use crate::syntax::nominal_constructions as nominal;
pub(crate) mod noun;
pub(crate) mod predicate;
#[cfg(test)]
pub(crate) mod probe;
pub(crate) mod quantity;
pub(crate) mod sentence;

use deckmaste_construction_compiler::runtime::GroupData;

pub(crate) static GROUPS: &[&GroupData] = &[
    coordination::GROUPS[0],
    noun::GROUPS[0],
    nominal::GROUPS[0],
    quantity::GROUPS[0],
    sentence::GROUPS[0],
];
