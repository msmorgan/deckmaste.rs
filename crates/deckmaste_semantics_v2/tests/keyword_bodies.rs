//! Keyword declaration bodies are written the way a card is written.
//!
//! A card reads under the restriction `docs/decisions/semantics-v2.md` §11.1
//! states as "a card may write only macros": at an expression kind a
//! constructor has no native candidacy, so `Activated(…)` is refused where
//! `activated(…)` reads. A macro's own body is read without that restriction,
//! because it is the basis the macros are written over — which is why nothing
//! stopped the keyword families from spelling every term out in constructors.
//! This test reads what each `keyword_actions/` and `keyword_abilities/` file
//! writes under the card restriction, so their bodies are held to card
//! vocabulary as they are re-spelled.
//!
//! **What is read.** Each field a keyword file writes into its term: an
//! ability's `body` (the list of abilities) and `keyword_params`, an action's
//! `body` (the instruction), `agent` and `deed`. A declaration body has holes
//! — `Param(i)` stands for the declaration's `i`th argument — and a hole is
//! not something a card can write, so each is replaced by a fixed card-legal
//! sample of the declared parameter type before the read (see [`sample`]).
//! The samples are macros, numerals or native values, so they neither add nor
//! hide a constructor: whether the read passes is a fact about the file's own
//! text. Every substituted text must first read UNRESTRICTED; a file that
//! fails that is a broken test fixture, never an allowlist entry.
//!
//! **The ratchet.** [`ALLOWED_RAW`] lists the files that still write an
//! expression constructor. The test fails if a file outside it does, and
//! also if a file on it no longer does — so the list can only shrink, and a
//! re-spelled file must be struck from it in the same change.

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use ::ron::value::RawValue;
use deckmaste_semantics_v2::abilities::Ability;
use deckmaste_semantics_v2::abilities::Instruction;
use deckmaste_semantics_v2::abilities::KeywordParam;
use deckmaste_semantics_v2::phrase::NounPhrase;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::reader::ron_files_recursive;
use deckmaste_semantics_v2::words::Deed;
use macro_ron::MacroSet;
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// The keyword files that still write an expression constructor, by
/// `<family>/<stem>`. Strike a file when it is re-spelled; never add one.
const ALLOWED_RAW: &[&str] = &[
    "keyword_abilities/auraSwap",
    "keyword_abilities/demonstrate",
    "keyword_abilities/transfigure",
    "keyword_abilities/transmute",
    "keyword_actions/meld",
    "keyword_actions/search",
    "keyword_actions/shuffle",
    "keyword_actions/vote",
];

/// `plugins_v2/builtin`, or the plugin directory `KEYWORD_BODIES_PLUGIN`
/// names — a scratch copy whose bodies a rewrite is being tried on.
fn builtin_root() -> PathBuf {
    std::env::var_os("KEYWORD_BODIES_PLUGIN").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
        PathBuf::from,
    )
}

/// The fields of a keyword declaration file that become part of its term,
/// as written. Everything else (`name`, `spelling`, `grammar`) is metadata.
/// ron checks a struct's name against the file's, so each family's file
/// reads through its own name.
macro_rules! written {
    ($ty:ident, $name:literal) => {
        #[derive(Deserialize)]
        #[serde(rename = $name)]
        struct $ty {
            #[serde(default)]
            params: Option<Box<RawValue>>,
            #[serde(default)]
            keyword_params: Option<Box<RawValue>>,
            #[serde(default)]
            deed: Option<Box<RawValue>>,
            #[serde(default)]
            agent: Option<Box<RawValue>>,
            #[serde(default)]
            body: Option<Box<RawValue>>,
        }

        impl From<$ty> for Written {
            fn from(file: $ty) -> Self {
                Written {
                    params: file.params,
                    keyword_params: file.keyword_params,
                    deed: file.deed,
                    agent: file.agent,
                    body: file.body,
                }
            }
        }
    };
}

struct Written {
    params: Option<Box<RawValue>>,
    keyword_params: Option<Box<RawValue>>,
    deed: Option<Box<RawValue>>,
    agent: Option<Box<RawValue>>,
    body: Option<Box<RawValue>>,
}

written!(KeywordActionFile, "KeywordAction");
written!(KeywordAbilityFile, "KeywordAbility");

/// A card-legal argument of each declared parameter type: a macro, a
/// numeral, or a native value of a non-expression kind.
fn sample(ty: &str) -> &'static str {
    match ty {
        "Ability" => "mayBeginOnBattlefield",
        "Amount" | "Power" | "Toughness" => "3",
        "Ballot" => "ByCandidate(you)",
        "Condition" => "rolledDoubles",
        "Cost" => "tapSymbol",
        "Disclosure" => "Openly",
        "Instruction" => "drawGame",
        "Quality" => "creature",
        "Quantity" => "exactly(1)",
        "SearchScope" => "OneZone(yourLibrary)",
        "String" => "\"Sample\"",
        "Subject" => "you",
        "Subtype" => "zombie",
        "TokenSpec" => "asThose",
        other => panic!("no card-legal sample for a `{other}` parameter; add one"),
    }
}

/// The declared parameter types, in order, from a `params: [A, B]` text.
fn param_types(params: Option<&RawValue>) -> Vec<String> {
    let Some(params) = params else {
        return Vec::new();
    };
    params
        .get_ron()
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(str::trim)
        .filter(|ty| !ty.is_empty())
        .map(str::to_owned)
        .collect()
}

/// `text` with every `Param(i)` hole replaced by the sample of parameter
/// `i`'s declared type.
fn fill_holes(text: &str, types: &[String]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("Param(") {
        out.push_str(&rest[..at]);
        let after = &rest[at + "Param(".len()..];
        let close = after.find(')').expect("a `Param(` hole closes");
        let index: usize = after[..close].trim().parse().unwrap_or_else(|_| {
            panic!("a keyword hole is positional: `Param({})`", &after[..close])
        });
        let ty = types
            .get(index)
            .unwrap_or_else(|| panic!("`Param({index})` has no declared parameter"));
        out.push_str(sample(ty));
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Reads `text` as `T` unrestricted (it must) and then restricted, returning
/// the restricted read's refusal if there is one.
fn restricted_refusal<T: DeserializeOwned>(
    macros: &MacroSet,
    file: &str,
    field: &str,
    text: &str,
) -> Option<String> {
    if let Err(error) = macros.read_str::<T>(text) {
        panic!("{file}: `{field}` with its holes filled does not read at all: {error}\n{text}");
    }
    macros
        .read_str_restricted::<T>(text)
        .err()
        .map(|error| format!("`{field}`: {error}"))
}

/// The first refusal among the term-bearing fields `file` writes, or `None`
/// when every one reads as card vocabulary.
fn refusal(macros: &MacroSet, family: &str, file: &str, source: &str) -> Option<String> {
    let options = deckmaste_semantics_v2::ron::raw_options();
    let written: Result<Written, _> = if family == "keyword_abilities" {
        options
            .from_str::<KeywordAbilityFile>(source)
            .map(Written::from)
    } else {
        options
            .from_str::<KeywordActionFile>(source)
            .map(Written::from)
    };
    let written =
        written.unwrap_or_else(|error| panic!("{file}: the declaration's fields read: {error}"));
    let types = param_types(written.params.as_deref());
    let filled = |raw: &RawValue| fill_holes(raw.get_ron(), &types);
    let mut refusals = Vec::new();
    if family == "keyword_abilities" {
        if let Some(raw) = written.keyword_params.as_deref() {
            refusals.push(restricted_refusal::<Vec<KeywordParam>>(
                macros,
                file,
                "keyword_params",
                &filled(raw),
            ));
        }
        if let Some(raw) = written.body.as_deref() {
            refusals.push(restricted_refusal::<Vec<Ability>>(
                macros,
                file,
                "body",
                &filled(raw),
            ));
        }
    } else {
        if let Some(raw) = written.deed.as_deref() {
            refusals.push(restricted_refusal::<Deed>(
                macros,
                file,
                "deed",
                &filled(raw),
            ));
        }
        if let Some(raw) = written.agent.as_deref() {
            refusals.push(restricted_refusal::<NounPhrase>(
                macros,
                file,
                "agent",
                &filled(raw),
            ));
        }
        if let Some(raw) = written.body.as_deref() {
            refusals.push(restricted_refusal::<Instruction>(
                macros,
                file,
                "body",
                &filled(raw),
            ));
        }
    }
    refusals.into_iter().flatten().next()
}

#[test]
fn keyword_bodies_are_written_in_card_vocabulary() {
    let root = builtin_root();
    let builtin = Plugin::load(&root).expect("the builtin declarations load");
    let allowed: BTreeSet<&str> = ALLOWED_RAW.iter().copied().collect();
    let mut read = 0;
    let mut raw = 0;
    let mut newly_raw = Vec::new();
    let mut now_clean = Vec::new();
    let mut seen = BTreeSet::new();
    for family in ["keyword_actions", "keyword_abilities"] {
        for path in ron_files_recursive(&root.join("macros").join(family))
            .expect("the keyword family directories list")
        {
            let stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect("a declaration file has a UTF-8 stem");
            let file = format!("{family}/{stem}");
            let source =
                std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{file}: {error}"));
            read += 1;
            let refused = refusal(&builtin.macros, family, &file, &source);
            let listed = allowed.contains(file.as_str());
            if refused.is_some() {
                raw += 1;
            }
            match (refused, listed) {
                (Some(why), false) => newly_raw.push(format!("{file}: {why}")),
                (None, true) => now_clean.push(file.clone()),
                _ => {}
            }
            seen.insert(file);
        }
    }
    println!("{read} keyword declaration file(s) read; {raw} still write a constructor");
    assert!(
        read >= 250,
        "only {read} keyword files read; the scan lost the corpus"
    );
    let stale: Vec<&&str> = allowed
        .iter()
        .filter(|file| !seen.contains(**file))
        .collect();
    assert!(
        stale.is_empty(),
        "ALLOWED_RAW names files that do not exist: {stale:?}"
    );
    assert!(
        newly_raw.is_empty(),
        "keyword bodies writing an expression constructor a card could not write (re-spell \
         them over macros; the allowlist only shrinks):\n{}",
        newly_raw.join("\n")
    );
    assert!(
        now_clean.is_empty(),
        "these files now read as card vocabulary; strike them from ALLOWED_RAW: {now_clean:?}"
    );
}
