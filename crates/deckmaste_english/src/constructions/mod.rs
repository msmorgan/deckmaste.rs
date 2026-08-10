//! Construction-group declarations and test-only compiler scaffolding.

pub(crate) mod adjective;
pub(crate) mod attachment;
pub(crate) mod clause;
pub(crate) mod coordination;
pub(crate) mod determiner {
    pub(crate) use crate::syntax::determiner_constructions::*;

    #[cfg(test)]
    mod tests {
        #[test]
        fn production_declaration_exposes_all_nine_d01_rows() {
            assert_eq!(super::GROUPS[0].constructions.len(), 9);
            assert_eq!(super::GROUPS[0].constructions[0].id, "determiner_closed");
            assert_eq!(
                super::GROUPS[0].constructions[8].id,
                "possessive_noun_adjective"
            );
        }
    }
}
#[cfg(test)]
pub(crate) mod law;
pub(crate) use crate::syntax::nominal_constructions as nominal;
pub(crate) mod nonfinite;
pub(crate) mod noun;
pub(crate) mod predicate;
#[cfg(test)]
pub(crate) mod probe;
pub(crate) mod quantity;
pub(crate) mod sentence;

use deckmaste_construction_compiler::runtime::GroupData;

pub(crate) static GROUPS: &[&GroupData] = &[
    adjective::GROUPS[0],
    attachment::GROUPS[0],
    clause::GROUPS[0],
    coordination::GROUPS[0],
    determiner::GROUPS[0],
    noun::GROUPS[0],
    nominal::GROUPS[0],
    nonfinite::GROUPS[0],
    predicate::GROUPS[0],
    quantity::GROUPS[0],
    sentence::GROUPS[0],
];
