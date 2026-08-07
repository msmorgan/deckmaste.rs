//! The adapter probe: a minimal generated group proving the chart routes
//! `RuleImpl::Generated` end to end — assembly, scanning, packing, dominance
//! selection, decisions, and declaration-emitted typed lowering. The marker
//! semantics are deliberately synthetic; nothing here models English.

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProbeLensToken {
    pub(crate) word: Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProbeLensRecord {
    pub(crate) prefix: Vec<ProbeLensToken>,
    pub(crate) head: ProbeLensToken,
    pub(crate) suffix: Vec<ProbeLensToken>,
}

deckmaste_constructions_macro::constructions! {
    group adapter_probe;

    internal construction probe_word: ProbeItem {
        own ProbeWordNode {
            word: lex Conjunction,
        }
        // The `when` guards exist only to keep EC024 (ambiguous
        // linearization) satisfied — the chart adapter does not consult
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

pub(crate) static GROUPS: &[&GroupData] = &[
    &ADAPTER_PROBE_DECLARATION,
    &LENS_PROBE_DECLARATION,
    &PROBE_EXTRA_DECLARATION,
    &SEQUENCE_PROBE_DECLARATION,
];
pub(crate) static GROUPS_REVERSED: &[&GroupData] = &[
    &SEQUENCE_PROBE_DECLARATION,
    &PROBE_EXTRA_DECLARATION,
    &LENS_PROBE_DECLARATION,
    &ADAPTER_PROBE_DECLARATION,
];

deckmaste_constructions_macro::constructions! {
    group probe_extra;

    internal construction probe_extra_word: ProbeExtra {
        own ProbeExtraNode {
            word: lex Conjunction,
        }
        form only @ 0 = lex(word);
    }
}

deckmaste_constructions_macro::constructions! {
    group lens_probe;

    lens record bind ProbeLensRecord {
        prefix: vec ProbeLensToken,
        head: value ProbeLensToken,
        suffix: vec ProbeLensToken,
    }

    internal construction probe_lens_word: ProbeLensToken {
        bind ProbeLensToken {
            word: lex Conjunction,
        }
        form only @ 0 = lex(word);
        selection unique;
    }

    internal construction probe_lens_head: ProbeLensRecord {
        bind ProbeLensRecord {
            head: hole ProbeLensToken,
        }
        lens record {
            focus head with head;
        }
        form only @ 0 = head;
        selection unique;
    }

    internal construction probe_lens_prepend: ProbeLensRecord {
        bind ProbeLensRecord {
            member: hole ProbeLensToken,
            owner: hole ProbeLensRecord,
        }
        lens record from owner {
            prepend prefix with member;
        }
        form only @ 0 = member owner;
        selection unique;
    }
}

#[cfg(test)]
pub(crate) fn linearize_lens_record(value: &ProbeLensRecord) -> Result<String, String> {
    #[derive(Default)]
    struct Visitor {
        words: Vec<&'static str>,
    }

    impl Visitor {
        fn record(&mut self, value: &ProbeLensRecord) -> Result<(), String> {
            linearize_lens_probe_group_with(value, self).map_err(|error| format!("{error:?}"))
        }
    }

    impl deckmaste_construction_compiler::runtime::LinearizationVisitor for Visitor {
        type Error = String;

        fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
            self.words.push(literal);
            Ok(())
        }

        fn subtree<T: std::any::Any>(
            &mut self,
            category: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            let value = value as &dyn std::any::Any;
            match category {
                "ProbeLensToken" => {
                    let value = value
                        .downcast_ref::<ProbeLensToken>()
                        .ok_or_else(|| "lens token kept the wrong Rust type".to_owned())?;
                    linearize_probe_lens_word_with(value, self)
                        .map_err(|error| format!("{error:?}"))
                }
                "ProbeLensRecord" => {
                    let value = value
                        .downcast_ref::<ProbeLensRecord>()
                        .ok_or_else(|| "lens owner kept the wrong Rust type".to_owned())?;
                    self.record(value)
                }
                other => Err(format!("unexpected lens subtree category {other}")),
            }
        }

        fn scalar<T: std::any::Any>(
            &mut self,
            codec: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            if codec != "Conjunction" {
                return Err(format!("unexpected lens scalar codec {codec}"));
            }
            let word = (value as &dyn std::any::Any)
                .downcast_ref::<Conjunction>()
                .ok_or_else(|| "lens scalar kept the wrong Rust type".to_owned())?;
            self.words.push(word.spelling());
            Ok(())
        }
    }

    if !value.suffix.is_empty() {
        return Err("the prepend-only recursive probe cannot linearize a suffix".to_owned());
    }
    let mut visitor = Visitor::default();
    visitor.record(value)?;
    Ok(visitor.words.join(" "))
}

deckmaste_constructions_macro::constructions! {
    group sequence_probe;

    element probe_sequence_member {
        comma: lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole ProbeItem,
    }

    internal construction probe_sequence: ProbeSequenceRoot {
        own ProbeSequenceNode {
            first: hole ProbeItem,
            rest: seq probe_sequence_member,
        }
        require rest.len() >= 1;
        form only @ 0 = first rest;
    }
}
