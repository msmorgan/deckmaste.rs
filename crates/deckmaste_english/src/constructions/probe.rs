//! The adapter probe: a minimal generated group proving the chart routes
//! `RuleImpl::Generated` end to end — assembly, scanning, packing, dominance
//! selection, decisions, and `Ignored` lowering. Semantics are Milestone-3
//! stubs by design; nothing here models English.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Comma;
use crate::features::Conjunction;

/// Internal chart categories double as the own-mode hole types. Declaration
/// metadata references them through erased builders; chart lowering is still
/// staged.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ProbeItem;
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ProbeRoot;

deckmaste_constructions_macro::constructions! {
    group adapter_probe;

    internal construction probe_word: ProbeItem {
        own ProbeWordNode {
            word: lex Conjunction,
        }
        // The `when` guards exist only to keep EC024 (ambiguous
        // linearization) satisfied — the M3 chart adapter does not consult
        // guards at all (see `generated.rs`'s `register_generated`), so
        // which value each guard names has no bearing on chart matching.
        form only @ 0 when word in [Or] = lex(word);
        // Non-zero, non-consecutive ordinal on purpose: an incremental
        // per-construction counter (0, 1, 2, ...) could never produce `7`,
        // so a test pinning this value to the decision's production ordinal
        // proves the chart carries the declared `form … @ N` ordinal rather
        // than a renumbering. See
        // `generated_form_ordinal_is_the_declared_ordinal_not_a_counter` in
        // `grammar/parse_nonterminal.rs`.
        form padded @ 7 when word in [And] = "," lex(word);
    }

    internal construction probe_pick: ProbeRoot {
        own ProbePickNode {
            item: hole ProbeItem,
        }
        form only @ 0 = item;
        dominates probe_pick_shadow;
    }

    internal construction probe_pick_shadow: ProbeRoot {
        own ProbeShadowNode {
            item: hole ProbeItem,
        }
        form only @ 0 = item;
    }

    internal construction probe_pair: ProbePairRoot {
        own ProbePairNode {
            first: hole ProbeRoot,
            tail: lex Comma,
            second: hole ProbeRoot,
        }
        form only @ 0 = first lex(tail) second;
    }
}

// `ProbePairRoot` and `ProbeExtra` (below) deliberately have NO structs: no
// construction holes them, so the emitter never renders them as field types —
// those names exist only as internal chart categories.

pub(crate) static GROUPS: &[&GroupData] = &[&ADAPTER_PROBE_DECLARATION, &PROBE_EXTRA_DECLARATION];
pub(crate) static GROUPS_REVERSED: &[&GroupData] =
    &[&PROBE_EXTRA_DECLARATION, &ADAPTER_PROBE_DECLARATION];

deckmaste_constructions_macro::constructions! {
    group probe_extra;

    internal construction probe_extra_word: ProbeExtra {
        own ProbeExtraNode {
            word: lex Conjunction,
        }
        form only @ 0 = lex(word);
    }
}
