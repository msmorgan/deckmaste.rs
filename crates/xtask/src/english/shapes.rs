//! `cargo xtask english shapes` — mine rare AST productions to generate
//! misparse candidates mechanically.
//!
//! Round-trip proves a parse is *lossless*, never *correct*: a modifier
//! attached to the wrong host renders back byte-identically and reports full
//! structural recovery, so no existing gate can see it. Rare-production mining
//! is the discovery instrument for that defect class — annotation defects
//! concentrate in low-frequency productions, because correct structures recur
//! across cards while misparses are idiosyncratic.
//!
//! **Output is a worklist, never a finding.** Rarity is not proof of a defect.
//! Confirmed families graduate into the sound checks in [`super::lint`]; those
//! are what report defects.

use std::collections::HashMap;

use anyhow::Result;
use clap::Args;
use clap::ValueEnum;
use deckmaste_english::parse_with_identity;

use crate::english::data::OracleDataArgs;
use crate::english::data::map_supported_faces;
use crate::english::shape;
use crate::english::shape::Shape;

/// Enums whose variants are lexicon identity rather than tree structure.
/// `--lexicalize none` collapses these to the bare enum name.
const LEXICON: &[&str] = &[
    "Vocab",
    "RegularVocab",
    "Adjective",
    "ColorWord",
    "Numeral",
    "Pronoun",
];

/// How many distinct cards to record per production.
const EXEMPLARS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Arity {
    /// Collapse a run of identical children to `Label+`.
    Collapse,
    /// Keep every list length distinct.
    Exact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Lexicalize {
    /// Collapse lexicon atoms, leaving pure tree shape.
    None,
    /// Keep lexicon atoms — `head: Vocab::Damage` distinguishes a rare
    /// attachment from a common one that shares its shape.
    Head,
}

#[derive(Debug, Args)]
pub(super) struct ShapesArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Whether lexicon atoms enter the signature. `head` is a refinement, not
    /// a default: every creature type in `CatalogAtom.vocab` becomes its own
    /// singleton, which floods the tail.
    #[arg(long, value_enum, default_value_t = Lexicalize::None)]
    lexicalize: Lexicalize,

    /// Whether list length is part of a signature.
    #[arg(long, value_enum, default_value_t = Arity::Collapse)]
    arity: Arity,

    /// 1 = local production only; 2 = prefix the parent field edge, which
    /// separates otherwise-identical shapes by where they attach.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    depth: u8,

    /// Report only productions occurring at most this many times — the tail.
    #[arg(long, default_value_t = 10)]
    max_count: usize,

    /// Maximum rows to print.
    #[arg(long, default_value_t = 200)]
    limit: usize,
}

#[derive(Debug, Clone, Copy)]
struct Config {
    lexicalize: Lexicalize,
    arity: Arity,
    depth: u8,
}

/// One production's corpus footprint.
#[derive(Debug, Default)]
struct Entry {
    count: usize,
    exemplars: Vec<String>,
}

impl Entry {
    fn observe(&mut self, card: &str) {
        self.count += 1;
        if self.exemplars.len() < EXEMPLARS && !self.exemplars.iter().any(|name| name == card) {
            self.exemplars.push(card.to_string());
        }
    }

    fn merge(&mut self, other: Entry) {
        self.count += other.count;
        for card in other.exemplars {
            if self.exemplars.len() < EXEMPLARS && !self.exemplars.contains(&card) {
                self.exemplars.push(card);
            }
        }
    }
}

type Productions = HashMap<String, Entry>;

/// A node's label as it appears inside its parent's signature.
fn label(shape: &Shape, config: Config) -> String {
    if let Shape::Seq(items) = shape {
        return format!("[{}]", sequence(items, config).join(", "));
    }
    match shape.type_name() {
        Some(name) if config.lexicalize == Lexicalize::None && LEXICON.contains(&name) => {
            name.to_string()
        }
        _ => shape.label(),
    }
}

/// Under `--arity collapse`, a run of identical children becomes `Label+`.
/// Without this, list *length* masquerades as list *shape*: a ten-member
/// coordination is an unremarkable card but a unique signature, and the tail
/// fills with long-but-ordinary lists instead of defects. One versus many is
/// linguistically load-bearing; the exact count is not.
fn sequence(items: &[Shape], config: Config) -> Vec<String> {
    let labels = items.iter().map(|item| label(item, config));
    match config.arity {
        Arity::Exact => labels.collect(),
        Arity::Collapse => {
            let mut collapsed: Vec<String> = Vec::new();
            let mut repeated = false;
            for item in labels {
                if collapsed.last().is_some_and(|previous| *previous == item) {
                    repeated = true;
                    continue;
                }
                if repeated && let Some(previous) = collapsed.last_mut() {
                    previous.push('+');
                }
                repeated = false;
                collapsed.push(item);
            }
            if repeated && let Some(previous) = collapsed.last_mut() {
                previous.push('+');
            }
            collapsed
        }
    }
}

/// Emit one production per composite node.
fn collect(shape: &Shape, config: Config, edge: Option<&str>, card: &str, into: &mut Productions) {
    let own = label(shape, config);
    let production = match shape {
        Shape::Newtype { inner, .. } => Some(format!("{own}({})", label(inner, config))),
        Shape::Node { fields, .. } => {
            let rendered: Vec<_> = fields
                .iter()
                .map(|(key, value)| format!("{key}: {}", label(value, config)))
                .collect();
            Some(format!("{own}{{{}}}", rendered.join(", ")))
        }
        _ => None,
    };

    if let Some(production) = production {
        let key = if config.depth >= 2 {
            format!("{} > {production}", edge.unwrap_or("(root)"))
        } else {
            production
        };
        into.entry(key).or_default().observe(card);
    }

    match shape {
        Shape::Newtype { inner, .. } => {
            let edge = format!("{own}.0");
            collect(inner, config, Some(&edge), card, into);
        }
        Shape::Node { fields, .. } => {
            for (key, value) in fields {
                let edge = format!("{own}.{key}");
                collect(value, config, Some(&edge), card, into);
            }
        }
        Shape::Seq(items) => {
            for item in items {
                collect(item, config, edge, card, into);
            }
        }
        Shape::Map(entries) => {
            for (key, value) in entries {
                collect(key, config, edge, card, into);
                collect(value, config, edge, card, into);
            }
        }
        Shape::Scalar(_) | Shape::Unit { .. } | Shape::Absent => {}
    }
}

pub(super) fn run(args: &ShapesArgs) -> Result<()> {
    let data = args.data.load()?;
    let config = Config {
        lexicalize: args.lexicalize,
        arity: args.arity,
        depth: args.depth,
    };

    let per_face = map_supported_faces(&data.faces, |_, card| {
        let report = parse_with_identity(
            &card.oracle_text,
            &data.catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let mut productions = Productions::new();
        collect(
            &shape::of(report.ast()),
            config,
            None,
            card.printed_name(),
            &mut productions,
        );
        productions
    });

    let mut productions = Productions::new();
    for face in per_face {
        for (signature, entry) in face {
            productions.entry(signature).or_default().merge(entry);
        }
    }

    let total_distinct = productions.len();
    let total_nodes: usize = productions.values().map(|entry| entry.count).sum();

    let mut tail: Vec<_> = productions
        .into_iter()
        .filter(|(_, entry)| entry.count <= args.max_count)
        .collect();
    tail.sort_by(|(left_signature, left), (right_signature, right)| {
        left.count
            .cmp(&right.count)
            .then_with(|| left_signature.cmp(right_signature))
    });

    let shown = tail.len().min(args.limit);
    println!("count\tsignature\texemplars");
    for (signature, entry) in tail.iter().take(args.limit) {
        println!(
            "{}\t{signature}\t{}",
            entry.count,
            entry.exemplars.join(" | ")
        );
    }

    eprintln!(
        "{total_distinct} distinct productions over {total_nodes} nodes; \
         {} at or below {} occurrences, {shown} shown",
        tail.len(),
        args.max_count,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    #[derive(Clone, Serialize)]
    enum Leaf {
        Alpha,
        Beta,
    }

    #[derive(Serialize)]
    struct Fixture {
        name: String,
        children: Vec<Leaf>,
        tag: Option<Leaf>,
    }

    fn fixture(children: Vec<Leaf>, tag: Option<Leaf>) -> Fixture {
        Fixture {
            name: "Bonfire of the Damned".to_string(),
            children,
            tag,
        }
    }

    fn signatures(value: &Fixture, config: Config) -> Vec<String> {
        let mut productions = Productions::new();
        collect(&shape::of(value), config, None, "Fixture", &mut productions);
        let mut keys: Vec<_> = productions.into_keys().collect();
        keys.sort();
        keys
    }

    fn config(arity: Arity, depth: u8) -> Config {
        Config {
            lexicalize: Lexicalize::Head,
            arity,
            depth,
        }
    }

    #[test]
    fn string_values_never_enter_a_signature() {
        let found = signatures(
            &fixture(vec![Leaf::Alpha], None),
            config(Arity::Collapse, 1),
        );
        assert!(
            found.iter().all(|signature| !signature.contains("Bonfire")),
            "card names must not reach signatures: {found:?}"
        );
        assert!(
            found
                .iter()
                .any(|signature| signature.contains("name: str")),
            "string fields collapse to their type: {found:?}"
        );
    }

    #[test]
    fn an_absent_option_is_visible_in_the_signature() {
        let absent = signatures(&fixture(vec![], None), config(Arity::Collapse, 1));
        let present = signatures(
            &fixture(vec![], Some(Leaf::Beta)),
            config(Arity::Collapse, 1),
        );
        assert!(
            absent
                .iter()
                .any(|signature| signature.contains("tag: None"))
        );
        assert!(
            present
                .iter()
                .any(|signature| signature.contains("tag: Leaf::Beta"))
        );
    }

    #[test]
    fn arity_collapse_makes_list_length_irrelevant() {
        let short = signatures(
            &fixture(vec![Leaf::Alpha, Leaf::Alpha], None),
            config(Arity::Collapse, 1),
        );
        let long = signatures(
            &fixture(vec![Leaf::Alpha; 9], None),
            config(Arity::Collapse, 1),
        );
        assert_eq!(
            short, long,
            "list length must not masquerade as list shape under --arity collapse"
        );
        assert!(
            short
                .iter()
                .any(|signature| signature.contains("[Leaf::Alpha+]"))
        );
    }

    #[test]
    fn arity_exact_keeps_list_length() {
        let short = signatures(
            &fixture(vec![Leaf::Alpha, Leaf::Alpha], None),
            config(Arity::Exact, 1),
        );
        let long = signatures(
            &fixture(vec![Leaf::Alpha; 9], None),
            config(Arity::Exact, 1),
        );
        assert_ne!(short, long);
    }

    #[test]
    fn a_heterogeneous_run_keeps_its_boundaries() {
        let found = signatures(
            &fixture(vec![Leaf::Alpha, Leaf::Alpha, Leaf::Beta], None),
            config(Arity::Collapse, 1),
        );
        assert!(
            found
                .iter()
                .any(|signature| signature.contains("[Leaf::Alpha+, Leaf::Beta]")),
            "collapse must not merge distinct neighbours: {found:?}"
        );
    }

    #[test]
    fn depth_two_prefixes_the_parent_edge() {
        let found = signatures(
            &fixture(vec![Leaf::Alpha], None),
            config(Arity::Collapse, 2),
        );
        assert!(
            found
                .iter()
                .any(|signature| signature.starts_with("(root) >")),
            "the root production carries no parent edge: {found:?}"
        );
    }
}
