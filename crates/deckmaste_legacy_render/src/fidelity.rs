//! The render-back fidelity gate (strong form): every finished card renders
//! to templated English and is diffed against its oracle text from the
//! repo's Scryfall-derived snapshot (`data/derived/cards.jsonl`). The tree
//! must stay tethered to the text — this is the check no type system
//! performs, and the dominant real-world failure mode of hand encodings is
//! adjunct drift: encodings that silently drop what the oracle sentence
//! prints (enter riders like "tapped" / "under its owner's control",
//! "another", "an opponent controls", "where X is …" clauses).
//!
//! STRONG form: ANY normalized difference is a gate failure, not a warning —
//! a missing adjunct is a missing adjunct whether or not it is on the named
//! watch list (the list only classifies the report). Intended divergence
//! needs an inline `// waiver: <reason>` comment in the card file; waivers
//! are enumerable (`cargo xtask fidelity --waivers`) and STALE waivers (a
//! waived card that renders clean) fail the gate, so the inventory stays
//! honest.
//!
//! Normalization is deliberately thin — it removes only surface conventions
//! that carry no encoding information:
//! - reminder text (parenthesized) is stripped from the oracle side;
//! - self-reference collapses to `~` on both sides: the card's printed name
//!   (and its short name before a comma) and the oracle's "this <type-word>"
//!   self-references ([CR#201.5] a name self-reference means the object itself;
//!   the extraction pipeline uses the same `~` sigil);
//! - whitespace runs collapse.
//!
//! Everything else — riders, restrictors, `where X` clauses, word order —
//! must survive to the render byte-for-byte.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use deckmaste_plugin::layout::CARDS_DIR;
use deckmaste_plugin::layout::is_todo_source;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_plugin::plugin::read;
use deckmaste_plugin::plugin::ron_files_recursive;
use deckmaste_semantics::Card;
use deckmaste_semantics::CardFace;

use crate::render::render_card_face;

/// The oracle snapshot's default location, relative to the workspace root.
/// Derived from Scryfall Oracle Cards and gitignored with the rest of `data/` —
/// a bare checkout (CI) has no oracle, and the fidelity test skips loudly
/// there.
pub const ORACLE_SNAPSHOT: &str = "data/derived/cards.jsonl";

/// One oracle face: the printed characteristics the renderer is diffed
/// against.
#[derive(Debug, Clone)]
pub struct OracleEntry {
    pub mana_cost: String,
    pub type_line: String,
    pub text: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
}

/// One `cards.jsonl` row (the Scryfall-derived snapshot's line format). A row
/// is either a standalone card (`face: null`) or one face of a multi-face
/// card (`face` = the face's own name, `name` = the combined name).
#[derive(Debug, serde::Deserialize)]
struct Row {
    name: String,
    face: Option<String>,
    #[serde(rename = "manaCost")]
    mana_cost: Option<String>,
    #[serde(rename = "type")]
    type_line: Option<String>,
    text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
}

/// The loaded oracle: face name → entry. Standalone rows win over same-named
/// faces of multi-face cards (the real Lightning Bolt, not an adventure/
/// prepare face that shares its name).
pub struct Oracle {
    entries: BTreeMap<String, OracleEntry>,
    /// Names that only matched via a multi-face row — overridable by a
    /// later standalone row, never the other way around.
    face_only: std::collections::BTreeSet<String>,
}

impl Oracle {
    /// Loads the snapshot (one JSON object per line).
    ///
    /// # Errors
    /// If the file is unreadable or a line fails to parse as a row.
    pub fn load(path: &Path) -> anyhow::Result<Oracle> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("reading oracle snapshot {}", path.display()))?;
        let mut oracle = Oracle {
            entries: BTreeMap::new(),
            face_only: std::collections::BTreeSet::new(),
        };
        for (i, line) in source.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let row: Row = serde_json::from_str(line)
                .with_context(|| format!("{}:{}: parsing oracle row", path.display(), i + 1))?;
            let entry = OracleEntry {
                mana_cost: row.mana_cost.unwrap_or_default(),
                type_line: row.type_line.unwrap_or_default(),
                text: row.text.unwrap_or_default(),
                power: row.power,
                toughness: row.toughness,
            };
            match row.face {
                None => {
                    oracle.face_only.remove(&row.name);
                    oracle.entries.insert(row.name, entry);
                }
                Some(face) => {
                    if !oracle.entries.contains_key(&face) || oracle.face_only.contains(&face) {
                        oracle.face_only.insert(face.clone());
                        oracle.entries.insert(face, entry);
                    }
                }
            }
        }
        Ok(oracle)
    }

    #[must_use]
    pub fn get(&self, face_name: &str) -> Option<&OracleEntry> {
        self.entries.get(face_name)
    }
}

/// One field-level divergence between the render and the oracle.
#[derive(Debug, Clone)]
pub struct Diff {
    /// Which field: `"mana cost"`, `"type line"`, `"p/t"`, or `"rules"`.
    pub field: &'static str,
    pub rendered: String,
    pub oracle: String,
    /// Watch-listed adjuncts the oracle side prints and the render lost —
    /// the classification, not the gate (ANY diff fails).
    pub missing_adjuncts: Vec<&'static str>,
}

/// The per-card outcome.
#[derive(Debug, Clone)]
pub enum Outcome {
    /// Normalized render matches the oracle on every field.
    Clean,
    /// No oracle entry under this face name — a synthetic (test-only) card,
    /// or a canon typo (fatal for canon: canon encodes real cards).
    NoOracle,
    /// One or more normalized divergences.
    Diffs(Vec<Diff>),
}

/// One checked card file.
#[derive(Debug, Clone)]
pub struct CardFidelity {
    pub path: PathBuf,
    pub name: String,
    /// The inline `// waiver: <reason>` annotation, when present.
    pub waiver: Option<String>,
    pub outcome: Outcome,
}

impl CardFidelity {
    /// A failure the gate must report: an unwaivered diff, a waived card
    /// that is clean (stale waiver), or — under `strict_oracle` (canon) — a
    /// card with no oracle entry at all.
    #[must_use]
    pub fn failure(&self, strict_oracle: bool) -> Option<String> {
        match (&self.outcome, &self.waiver) {
            (Outcome::Diffs(diffs), None) => Some(format!("{} unwaivered diff(s)", diffs.len())),
            (Outcome::Clean, Some(reason)) => Some(format!(
                "stale waiver ({reason:?}) — the card renders clean; remove the annotation"
            )),
            (Outcome::NoOracle, _) if strict_oracle => {
                Some("no oracle entry — canon encodes real cards".to_string())
            }
            (Outcome::Diffs(_) | Outcome::Clean | Outcome::NoOracle, _) => None,
        }
    }
}

/// Checks one plugin: every finished (non-todo) card under `cards/`, each
/// face diffed against its oracle entry.
///
/// # Errors
/// If the plugin fails to load or a card file fails to read/parse — those
/// are load problems, not fidelity findings.
pub fn check_plugin(plugin_dir: &Path, oracle: &Oracle) -> anyhow::Result<Vec<CardFidelity>> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let mut out = Vec::new();
    for path in ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        // The gate renders the semantic term, not the engine image: semantic
        // terms keep their `Expanded` invocation provenance — and so the
        // rules-text templates the renderer needs — where lowered core
        // carries none once `lower` erases it. The RENDERING view of that
        // term drops identity-macro provenance, which carries no template
        // and would only hide the structure this renderer matches on.
        let card = plugin
            .rendering_card_from_str(&source)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))?;
        let waiver = waiver_annotation(&source);
        for face in faces(&card) {
            out.push(CardFidelity {
                path: path.clone(),
                name: face.name.to_string(),
                waiver: waiver.clone(),
                outcome: check_face(face, oracle),
            });
        }
    }
    Ok(out)
}

fn faces(card: &Card) -> Vec<&CardFace> {
    match card {
        Card::Normal(f) => vec![f],
        Card::TwoFaced { front, back, .. } => vec![front, back],
    }
}

/// The inline waiver annotation: the first `// waiver: <reason>` comment
/// line in the card source.
#[must_use]
pub fn waiver_annotation(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.trim_start()
            .strip_prefix("//")
            .map(str::trim_start)
            .and_then(|rest| rest.strip_prefix("waiver:"))
            .map(|reason| reason.trim().to_string())
    })
}

/// The adjunct watch list — the report's classification of what a diff
/// lost. Each entry is a lowercase needle probed against the normalized
/// oracle line.
const ADJUNCT_WATCH: [&str; 6] = [
    "another",
    "tapped",
    "under its owner's control",
    "under their owner's control",
    "an opponent controls",
    "where x",
];

fn check_face(face: &CardFace, oracle: &Oracle) -> Outcome {
    let Some(entry) = oracle.get(&face.name) else {
        return Outcome::NoOracle;
    };
    let rendered = render_card_face(face);
    let mut diffs = Vec::new();

    if rendered.mana_cost != entry.mana_cost {
        diffs.push(diff("mana cost", &rendered.mana_cost, &entry.mana_cost));
    }
    if rendered.type_line != entry.type_line {
        diffs.push(diff("type line", &rendered.type_line, &entry.type_line));
    }
    let oracle_pt = match (&entry.power, &entry.toughness) {
        (None, None) => None,
        (p, t) => Some(format!(
            "{}/{}",
            p.as_deref().unwrap_or(""),
            t.as_deref().unwrap_or("")
        )),
    };
    if rendered.pt != oracle_pt {
        diffs.push(diff(
            "p/t",
            rendered.pt.as_deref().unwrap_or("-"),
            oracle_pt.as_deref().unwrap_or("-"),
        ));
    }

    let rendered_rules: Vec<String> = rendered
        .rules
        .iter()
        .map(|line| normalize(line, &face.name))
        .collect();
    let oracle_rules: Vec<String> = entry
        .text
        .lines()
        .map(strip_reminder)
        .filter(|line| !line.is_empty())
        .map(|line| normalize(&line, &face.name))
        .collect();
    let lines = rendered_rules.len().max(oracle_rules.len());
    for i in 0..lines {
        let r = rendered_rules.get(i).map_or("", String::as_str);
        let o = oracle_rules.get(i).map_or("", String::as_str);
        if r != o {
            diffs.push(diff("rules", r, o));
        }
    }

    if diffs.is_empty() { Outcome::Clean } else { Outcome::Diffs(diffs) }
}

fn diff(field: &'static str, rendered: &str, oracle: &str) -> Diff {
    let rendered_lower = rendered.to_lowercase();
    let oracle_lower = oracle.to_lowercase();
    let missing_adjuncts = ADJUNCT_WATCH
        .into_iter()
        .filter(|needle| oracle_lower.contains(needle) && !rendered_lower.contains(needle))
        .collect();
    Diff {
        field,
        rendered: rendered.to_string(),
        oracle: oracle.to_string(),
        missing_adjuncts,
    }
}

/// Strips oracle reminder text: parenthesized spans (oracle reminders never
/// nest), then collapses the whitespace the removal leaves behind.
fn strip_reminder(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut depth = 0usize;
    for c in line.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    collapse_spaces(out.trim())
}

/// The shared normalization (see the module docs): self-reference → `~`,
/// whitespace collapse.
///
/// Public so a caller comparing two *independently rendered* lines against
/// each other (not just against the oracle snapshot) — the macro-frames
/// round's own `cargo xtask macro pilot` G3 shadow-parity gate — normalizes
/// both sides through the exact same function this gate does, rather than
/// growing a second, driftable copy.
#[must_use]
pub fn normalize(line: &str, name: &str) -> String {
    let mut s = line.replace(name, "~");
    // A legendary's short name ("Elesh Norn, Grand Cenobite" → "Elesh
    // Norn") self-references the same object.
    if let Some((short, _)) = name.split_once(", ")
        && short.len() > 2
    {
        s = s.replace(short, "~");
    }
    // The modern self-reference spellings ([CR#201.5] — oracle text that
    // names "this <type-word>" means the object itself; older frames print
    // the card name, which the branch above already collapsed).
    for this in [
        "this creature",
        "this artifact",
        "this enchantment",
        "this land",
        "this planeswalker",
        "this permanent",
        "this spell",
        "this card",
        "this token",
    ] {
        s = s.replace(this, "~");
        s = s.replace(&capitalize(this), "~");
    }
    // Fold spelled-number energy ("six {E}") to a `{E}` run on BOTH sides so a
    // count past five diffs clean regardless of which side spells it (the
    // render threshold spells it, the oracle spells it — this makes them meet).
    s = deckmaste_plugin::energy::normalize_spelled_energy(&s);
    collapse_spaces(s.trim())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map_or_else(String::new, |first| first.to_uppercase().chain(c).collect())
}

fn collapse_spaces(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_space = false;
    for c in s.chars() {
        if c == ' ' {
            if !in_space {
                out.push(' ');
            }
            in_space = true;
        } else {
            in_space = false;
            out.push(c);
        }
    }
    // A stripped trailing reminder leaves " ." / " ," seams.
    out.replace(" .", ".").replace(" ,", ",")
}

/// Renders one card's findings as the report block `cargo xtask fidelity`
/// prints (empty for a clean, unwaivered card).
#[must_use]
pub fn report_block(card: &CardFidelity, strict_oracle: bool) -> String {
    let mut out = String::new();
    let failure = card.failure(strict_oracle);
    match (&card.outcome, failure) {
        (Outcome::Diffs(diffs), failure) => {
            let status = match (&card.waiver, failure) {
                (Some(reason), _) => format!("WAIVED ({reason})"),
                (None, _) => "FAIL".to_string(),
            };
            let _ = writeln!(out, "{}: {status}", card.path.display());
            for d in diffs {
                let _ = writeln!(out, "  {} differs:", d.field);
                let _ = writeln!(out, "    rendered: {}", d.rendered);
                let _ = writeln!(out, "    oracle:   {}", d.oracle);
                if !d.missing_adjuncts.is_empty() {
                    let _ = writeln!(
                        out,
                        "    missing required adjunct(s): {}",
                        d.missing_adjuncts.join(", ")
                    );
                }
            }
        }
        (Outcome::Clean, Some(stale)) => {
            let _ = writeln!(out, "{}: {stale}", card.path.display());
        }
        (Outcome::NoOracle, Some(missing)) => {
            let _ = writeln!(out, "{}: {missing}", card.path.display());
        }
        _ => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waiver_annotation_reads_the_comment_line() {
        let source = "// a header\n// waiver: divided-damage phrasing is a renderer seam\nNormal()";
        assert_eq!(
            waiver_annotation(source).as_deref(),
            Some("divided-damage phrasing is a renderer seam")
        );
        assert_eq!(waiver_annotation("Normal()"), None);
    }

    /// Reminder text strips without regex-level surprises: the whole
    /// parenthesized span goes, surrounding spacing heals.
    #[test]
    fn reminder_text_strips() {
        assert_eq!(
            strip_reminder(
                "Flying (This creature can't be blocked except by creatures with flying or reach.)"
            ),
            "Flying"
        );
        assert_eq!(
            strip_reminder("Cycling {2} ({2}, Discard this card: Draw a card.) Landfall."),
            "Cycling {2} Landfall."
        );
    }

    /// Self-reference collapses to `~` from every spelling: printed name,
    /// legendary short name, and the modern "this <type-word>" forms.
    #[test]
    fn self_reference_normalizes_to_tilde() {
        assert_eq!(
            normalize(
                "Lightning Bolt deals 3 damage to any target.",
                "Lightning Bolt"
            ),
            "~ deals 3 damage to any target."
        );
        assert_eq!(
            normalize(
                "When this creature dies, it deals 1 damage.",
                "Footlight Fiend"
            ),
            "When ~ dies, it deals 1 damage."
        );
        assert_eq!(
            normalize(
                "Elesh Norn, Grand Cenobite is white.",
                "Elesh Norn, Grand Cenobite"
            ),
            "~ is white."
        );
        assert_eq!(
            normalize("Elesh Norn tells you so.", "Elesh Norn, Grand Cenobite"),
            "~ tells you so."
        );
    }

    /// The classification names the watch-listed adjuncts an oracle line
    /// prints and the render lost — and only those.
    #[test]
    fn diffs_classify_missing_adjuncts() {
        let d = diff(
            "rules",
            "Put that card onto the battlefield.",
            "Put that card onto the battlefield tapped under its owner's control.",
        );
        assert_eq!(
            d.missing_adjuncts,
            vec!["tapped", "under its owner's control"]
        );
        let clean = diff(
            "rules",
            "Destroy another target creature.",
            "Destroy another target creature you control.",
        );
        assert!(clean.missing_adjuncts.is_empty());
    }

    /// The gate verdicts: unwaivered diffs fail, waived diffs pass, stale
    /// waivers fail, and a missing oracle entry fails only under
    /// `strict_oracle`.
    #[test]
    fn failure_policy() {
        let card = |outcome: Outcome, waiver: Option<&str>| CardFidelity {
            path: PathBuf::from("x.ron"),
            name: "X".to_string(),
            waiver: waiver.map(str::to_string),
            outcome,
        };
        let d = Outcome::Diffs(vec![diff("rules", "a", "b")]);
        assert!(card(d.clone(), None).failure(false).is_some());
        assert!(card(d, Some("known")).failure(false).is_none());
        assert!(card(Outcome::Clean, Some("stale")).failure(false).is_some());
        assert!(card(Outcome::Clean, None).failure(false).is_none());
        assert!(card(Outcome::NoOracle, None).failure(true).is_some());
        assert!(card(Outcome::NoOracle, None).failure(false).is_none());
    }
}
