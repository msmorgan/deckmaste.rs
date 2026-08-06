//! Construction-group declarations and test-only compiler scaffolding.

pub(crate) mod coordination;
#[cfg(test)]
pub(crate) mod law;
pub(crate) mod noun;
#[cfg(test)]
pub(crate) mod probe;
pub(crate) mod sentence;

use deckmaste_construction_compiler::runtime::GroupData;

pub(crate) static GROUPS: &[&GroupData] = &[
    coordination::GROUPS[0],
    noun::GROUPS[0],
    sentence::GROUPS[0],
];
