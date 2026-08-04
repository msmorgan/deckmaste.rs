//! The law-harness fixture group: a dominance-free pair of same-category
//! constructions, giving `parse_as` a genuine bytes-with-two-ASTs fixture.
//! Like the probe, semantics are stubs by design — nothing here models
//! English.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Conjunction;

/// The holed internal category's own-mode type (never constructed at
/// runtime — generated reductions are Milestone-3 feature stubs).
#[expect(
    dead_code,
    reason = "exists only so the emitted sealed hole type compiles; generated reductions are Features::None stubs that never construct LawItem"
)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LawItem;

deckmaste_constructions_macro::constructions! {
    group law_probe;

    internal construction law_letter: LawItem {
        own LawLetterNode {
            word: lex Conjunction,
        }
        form only @ 0 = lex(word);
    }

    internal construction law_first: LawRoot {
        own LawFirstNode {
            item: hole LawItem,
        }
        form only @ 0 = item;
    }

    internal construction law_second: LawRoot {
        own LawSecondNode {
            item: hole LawItem,
        }
        form only @ 0 = item;
    }
}

// `LawRoot` deliberately has NO struct: no construction holes it, so the
// name exists only as an internal chart category.

pub(crate) static GROUPS: &[&GroupData] = &[&LAW_PROBE_DECLARATION];
