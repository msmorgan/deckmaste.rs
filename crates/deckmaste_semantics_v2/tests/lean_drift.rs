//! The drift test:
//! `lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card,Rules}.lean`
//! and `lean/Semantics/Check/FactTypes.lean` against the eight Rust modules
//! that mirror them.
//!
//! Lean is the specification (`docs/decisions/semantics-v2.md` §10), so this
//! fails on any difference in EITHER direction: a declaration, constructor or
//! field one side has and the other lacks, and any disagreement in field order
//! (constructor argument order is textual order, §2, so it is meaning).
//!
//! Both sides are read from source: the Lean by the small line-oriented parser
//! below, the Rust by `syn`. Neither side maintains a hand-written inventory
//! that could itself go stale.
//!
//! The name mapping itself lives in the crate
//! ([`deckmaste_semantics_v2::lean_emit`]), because the card gate emits Lean
//! through the same correspondence this test reads it through. This test scans
//! the Lean names RAW and asserts the mapping round-trips on every one of them,
//! so a Lean spelling the emitter would get wrong fails here first.

#![allow(
    clippy::too_many_lines,
    reason = "the Lean scanner reads better as one pass than split across helpers"
)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// One declaration's shape: its members, each with its ordered field names.
/// A `structure` is one member under the empty name; an `abbrev` has none.
type Shape = BTreeMap<String, Vec<String>>;

const STRUCT_MEMBER: &str = "";

const PAIRS: &[(&str, &str, &str)] = &[
    (
        "Words",
        include_str!("../../../lean/Semantics/Words.lean"),
        include_str!("../src/words.rs"),
    ),
    (
        "Events",
        include_str!("../../../lean/Semantics/Events.lean"),
        include_str!("../src/events.rs"),
    ),
    (
        "Phrase",
        include_str!("../../../lean/Semantics/Phrase.lean"),
        include_str!("../src/phrase.rs"),
    ),
    (
        "Triggers",
        include_str!("../../../lean/Semantics/Triggers.lean"),
        include_str!("../src/triggers.rs"),
    ),
    (
        "Abilities",
        include_str!("../../../lean/Semantics/Abilities.lean"),
        include_str!("../src/abilities.rs"),
    ),
    (
        "Card",
        include_str!("../../../lean/Semantics/Card.lean"),
        include_str!("../src/card.rs"),
    ),
    (
        "Rules",
        include_str!("../../../lean/Semantics/Rules.lean"),
        include_str!("../src/rules.rs"),
    ),
    (
        "Check/FactTypes",
        include_str!("../../../lean/Semantics/Check/FactTypes.lean"),
        include_str!("../src/facts.rs"),
    ),
];

// ---------------------------------------------------------------------------
// The name mapping
// ---------------------------------------------------------------------------

use deckmaste_semantics_v2::lean_emit::lean_field;
use deckmaste_semantics_v2::lean_emit::lean_variant;
use deckmaste_semantics_v2::lean_emit::rust_field;
use deckmaste_semantics_v2::lean_emit::rust_variant;

// ---------------------------------------------------------------------------
// The Lean side
// ---------------------------------------------------------------------------

/// Strips block comments (`/- … -/`, doc `/-- … -/`, module `/-! … -/`) and
/// line comments, leaving the declaration skeleton.
fn strip_comments(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_block = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if in_block {
            if trimmed.ends_with("-/") {
                in_block = false;
            }
            out.push(String::new());
            continue;
        }
        if trimmed.starts_with("/-") {
            if !(trimmed.ends_with("-/") && trimmed.len() > 3) {
                in_block = true;
            }
            out.push(String::new());
            continue;
        }
        if trimmed.starts_with("--") {
            out.push(String::new());
            continue;
        }
        out.push(line.to_string());
    }
    out
}

/// Splits `(a b : T) (c : U)` into its field names, in order. A group naming
/// several fields at one type contributes each of them.
fn lean_args(rest: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for ch in rest.chars() {
        match ch {
            '(' => {
                depth += 1;
                if depth == 1 {
                    current.clear();
                    continue;
                }
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    groups.push(std::mem::take(&mut current));
                    continue;
                }
            }
            _ => {}
        }
        if depth >= 1 {
            current.push(ch);
        }
    }
    groups.iter().flat_map(|g| lean_binder(g)).collect()
}

/// The field names of one `a b : T` binder group.
fn lean_binder(group: &str) -> Vec<String> {
    let Some((names, _)) = group.split_once(':') else {
        return Vec::new();
    };
    names.split_whitespace().map(ToString::to_string).collect()
}

/// Whether a line opens a structure field (`name : Type`), as opposed to
/// continuing the previous one.
fn is_field_line(trimmed: &str) -> bool {
    let Some((names, _)) = trimmed.split_once(':') else {
        return false;
    };
    !names.trim().is_empty()
        && names.split_whitespace().all(|n| {
            n.chars()
                .next()
                .is_some_and(|c| c.is_alphabetic() || c == '_')
                && n.chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        })
}

fn lean_declarations(source: &str) -> BTreeMap<String, Shape> {
    let lines = strip_comments(source);
    let mut out: BTreeMap<String, Shape> = BTreeMap::new();
    let mut index = 0usize;
    while index < lines.len() {
        let line = &lines[index];
        let trimmed = line.trim();
        let indent = line.len() - line.trim_start().len();

        if let Some(rest) = trimmed.strip_prefix("abbrev ")
            && let Some((name, _)) = rest.split_once(":=")
            && !name.contains('.')
        {
            out.insert(name.trim().to_string(), Shape::new());
            index += 1;
            continue;
        }

        let opener = trimmed
            .strip_prefix("inductive ")
            .map(|rest| (true, rest))
            .or_else(|| trimmed.strip_prefix("structure ").map(|rest| (false, rest)));
        let Some((is_enum, rest)) = opener else {
            index += 1;
            continue;
        };
        if !rest.ends_with("where") {
            index += 1;
            continue;
        }
        let name = rest
            .split_whitespace()
            .next()
            .expect("a declaration names a type")
            .to_string();

        let mut shape = Shape::new();
        let mut members: Vec<(String, Vec<String>)> = Vec::new();
        let mut pending: Option<String> = None;
        index += 1;
        while index < lines.len() {
            let body = &lines[index];
            let body_trimmed = body.trim();
            if body_trimmed.is_empty() {
                index += 1;
                continue;
            }
            let body_indent = body.len() - body.trim_start().len();
            if body_indent <= indent || body_trimmed.starts_with("deriving") {
                break;
            }
            if is_enum {
                if let Some(stripped) = body_trimmed.strip_prefix('|') {
                    if let Some(text) = pending.take() {
                        members.extend(lean_entry(&text));
                    }
                    pending = Some(stripped.trim().to_string());
                } else if let Some(text) = pending.as_mut() {
                    text.push(' ');
                    text.push_str(body_trimmed);
                }
            } else if is_field_line(body_trimmed) {
                members.push((
                    STRUCT_MEMBER.to_string(),
                    lean_binder(body_trimmed.split(":=").next().unwrap_or(body_trimmed)),
                ));
            }
            index += 1;
        }
        if let Some(text) = pending.take() {
            members.extend(lean_entry(&text));
        }

        if is_enum {
            for (member, fields) in members {
                shape.insert(member, fields);
            }
        } else {
            let fields = members.into_iter().flat_map(|(_, f)| f).collect();
            shape.insert(STRUCT_MEMBER.to_string(), fields);
        }
        out.insert(name, shape);
    }
    out
}

/// Reads one entry of an inductive's body: either a run of unit constructors
/// written on one line (`| one | many`) or a single constructor with its
/// binder groups.
fn lean_entry(text: &str) -> Vec<(String, Vec<String>)> {
    let parts = lean_unit_run(text);
    if parts.len() > 1
        && parts
            .iter()
            .all(|p| p.chars().all(|c| c.is_alphanumeric() || c == '_'))
    {
        return parts.iter().map(|p| (p.clone(), Vec::new())).collect();
    }
    vec![lean_constructor(text)]
}

/// Reads one constructor entry. A bare `a | b | c` run of unit constructors is
/// returned as its first; the caller never sees a run because
/// [`lean_declarations`] splits on `|` before calling — except within one
/// entry, which this handles.
fn lean_constructor(text: &str) -> (String, Vec<String>) {
    let head = text.split('|').next().unwrap_or(text).trim();
    let mut parts = head.splitn(2, char::is_whitespace);
    let name = parts.next().unwrap_or_default();
    let rest = parts.next().unwrap_or_default();
    (name.to_string(), lean_args(rest))
}

/// Expands the `| a | b | c` unit runs a single Lean line may carry.
fn lean_unit_run(text: &str) -> Vec<String> {
    text.split('|')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(ToString::to_string)
        .collect()
}

// ---------------------------------------------------------------------------
// The Rust side
// ---------------------------------------------------------------------------

fn rust_declarations(source: &str) -> BTreeMap<String, Shape> {
    let file = syn::parse_file(source).expect("the crate's own module parses as Rust");
    let mut out: BTreeMap<String, Shape> = BTreeMap::new();
    for item in &file.items {
        match item {
            syn::Item::Enum(item) => {
                let mut shape = Shape::new();
                for variant in &item.variants {
                    let fields = match &variant.fields {
                        syn::Fields::Named(named) => named
                            .named
                            .iter()
                            .map(|f| {
                                f.ident
                                    .as_ref()
                                    .expect("a named field has an identifier")
                                    .to_string()
                            })
                            .collect(),
                        syn::Fields::Unit => Vec::new(),
                        syn::Fields::Unnamed(_) => panic!(
                            "{}::{} is a tuple variant; a mirrored constructor keeps Lean's \
                             field names, so every payload variant is a struct variant",
                            item.ident, variant.ident
                        ),
                    };
                    shape.insert(variant.ident.to_string(), fields);
                }
                out.insert(item.ident.to_string(), shape);
            }
            syn::Item::Struct(item) => {
                let syn::Fields::Named(named) = &item.fields else {
                    panic!("{} must be a named-field struct", item.ident);
                };
                let fields = named
                    .named
                    .iter()
                    .map(|f| {
                        f.ident
                            .as_ref()
                            .expect("a named field has an identifier")
                            .to_string()
                    })
                    .collect();
                let mut shape = Shape::new();
                shape.insert(STRUCT_MEMBER.to_string(), fields);
                out.insert(item.ident.to_string(), shape);
            }
            syn::Item::Type(item) => {
                out.insert(item.ident.to_string(), Shape::new());
            }
            _ => {}
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The comparison
// ---------------------------------------------------------------------------

/// One declaration's Lean shape as its Rust shape: the mapping applied to
/// every constructor and field name the scan read raw.
fn translate(shape: &Shape) -> Shape {
    shape
        .iter()
        .map(|(member, fields)| {
            let member = if member == STRUCT_MEMBER {
                STRUCT_MEMBER.to_string()
            } else {
                rust_variant(member)
            };
            (member, fields.iter().map(|f| rust_field(f)).collect())
        })
        .collect()
}

fn describe(member: &str) -> String {
    if member == STRUCT_MEMBER {
        "(fields)".to_string()
    } else {
        member.to_string()
    }
}

#[test]
fn rust_mirrors_the_lean_syntax_declaration_for_declaration() {
    let mut report = String::new();
    for (file, lean_source, rust_source) in PAIRS {
        let lean: BTreeMap<String, Shape> = lean_declarations(lean_source)
            .iter()
            .map(|(name, shape)| (name.clone(), translate(shape)))
            .collect();
        let rust = rust_declarations(rust_source);

        for name in lean.keys() {
            if !rust.contains_key(name) {
                writeln!(report, "{file}: Lean declares `{name}`; Rust does not").unwrap();
            }
        }
        for name in rust.keys() {
            if !lean.contains_key(name) {
                writeln!(report, "{file}: Rust declares `{name}`; Lean does not").unwrap();
            }
        }
        for (name, lean_shape) in &lean {
            let Some(rust_shape) = rust.get(name) else {
                continue;
            };
            for member in lean_shape.keys() {
                if !rust_shape.contains_key(member) {
                    writeln!(
                        report,
                        "{file}: `{name}` has Lean member `{}`; Rust does not",
                        describe(member)
                    )
                    .unwrap();
                }
            }
            for member in rust_shape.keys() {
                if !lean_shape.contains_key(member) {
                    writeln!(
                        report,
                        "{file}: `{name}` has Rust member `{}`; Lean does not",
                        describe(member)
                    )
                    .unwrap();
                }
            }
            for (member, lean_fields) in lean_shape {
                let Some(rust_fields) = rust_shape.get(member) else {
                    continue;
                };
                if lean_fields != rust_fields {
                    writeln!(
                        report,
                        "{file}: `{name}::{}` fields differ\n  Lean: {lean_fields:?}\n  Rust: \
                         {rust_fields:?}",
                        describe(member)
                    )
                    .unwrap();
                }
            }
        }
    }
    assert!(report.is_empty(), "Lean/Rust drift:\n{report}");
}

/// The mirror is not trivially empty: a scanner that read nothing would make
/// the drift comparison vacuous.
#[test]
fn the_drift_scan_finds_the_whole_syntax() {
    let mut declarations = 0usize;
    let mut members = 0usize;
    for (_, lean_source, _) in PAIRS {
        let lean = lean_declarations(lean_source);
        declarations += lean.len();
        members += lean.values().map(BTreeMap::len).sum::<usize>();
    }
    assert!(
        declarations >= 180 && members >= 800,
        "the Lean scan found {declarations} declarations and {members} members; the six syntax \
         files carry far more, so the scanner is broken rather than the mirror"
    );
}

/// The name mapping is the one the crate documents.
#[test]
fn the_name_mapping_is_the_documented_one() {
    assert_eq!(rust_variant("hasType"), "HasType");
    assert_eq!(rust_variant("return_"), "Return");
    assert_eq!(rust_variant("self"), "Self_");
    assert_eq!(rust_field("from_"), "from");
    assert_eq!(rust_field("type"), "r#type");
    assert_eq!(rust_field("by_"), "by");
    assert_eq!(rust_field("callerWidth"), "caller_width");
    assert_eq!(lean_unit_run("up | down"), vec!["up", "down"]);
}

/// The card gate writes Lean back through the inverse of this mapping, so
/// every constructor and field name the mirrored Lean files declare has to
/// survive the round trip. A Lean spelling that does not is a name the emitted
/// term would get wrong — an unbuildable module, or worse, a different
/// constructor.
#[test]
fn every_lean_name_round_trips_through_the_emitters_mapping() {
    let mut broken = String::new();
    let mut names = 0usize;
    for (file, lean_source, _) in PAIRS {
        for (declaration, shape) in lean_declarations(lean_source) {
            for (member, fields) in shape {
                if member != STRUCT_MEMBER {
                    names += 1;
                    let back = lean_variant(&rust_variant(&member));
                    if back != member {
                        writeln!(
                            broken,
                            "{file}: `{declaration}.{member}` comes back as `{back}`"
                        )
                        .unwrap();
                    }
                }
                for field in fields {
                    names += 1;
                    let back = lean_field(&rust_field(&field));
                    if back != field {
                        writeln!(
                            broken,
                            "{file}: `{declaration}.{member}`'s field `{field}` comes back as \
                             `{back}`"
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    assert!(
        names >= 800,
        "the scan found only {names} names to round-trip"
    );
    assert!(broken.is_empty(), "names the mapping loses:\n{broken}");
}
