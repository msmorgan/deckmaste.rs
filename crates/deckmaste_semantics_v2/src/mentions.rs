//! First mention binds, later mentions refer back.
//!
//! A registry macro's referent parameter (`Subject` or `NounPhrase`, with or
//! without a default) is introduced by the body's first mention of it; every
//! later mention is a back-reference to that binding, resolved by its Binding
//! Identity rather than by a pronoun search (`docs/decisions/semantics-v2.md`
//! §7, ruling 2026-10-06). Substitution by copy stays for a parameter the body
//! mentions once, so such a body is returned untouched and expands
//! byte-identically.
//!
//! The rewrite is on the declaration's body text, before the expander sees it:
//! the first `Param(p)` becomes `firstMention(id: "<macro>.<p>", phrase:
//! Param(p))` and each later one `laterMention(id: "<macro>.<p>", phrase:
//! Param(p))`, macros for the constructors Lean calls `NounPhrase.firstMention`
//! and `laterMention`. The expander
//! stays a dumb substituter ([macros-are-declarative]); the identity is a name
//! fixed by the declaration, never a position, and the checker reads the most
//! recent binding recorded under it, so two calls of one macro each read their
//! own. "First" is textual order, which is constructor argument order and so
//! the order the checker threads the context in (§2).
//!
//! [macros-are-declarative]: ../../../docs/decisions/macros-are-declarative.md

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;

use macro_ron::MacroDef;
use macro_ron::MacroSet;
use macro_ron::Params;

/// The two macros the rewrite writes, one per mention form. They are loader
/// vocabulary, registered with every v2 macro set rather than declared under
/// `plugins_v2`: a declaration body's arguments are read as author text, so the
/// wrapper around a forwarded hole must be a macro there, while no card writes
/// either (`tests/corpus.rs`), as Lean tags both constructors
/// `internal_expansion`.
const MENTION_MACROS: &[&str] = &[
    "(name: \"firstMention\", kinds: [NounPhrase], \
     params: {\"id\": String, \"phrase\": NounPhrase}, \
     body: FirstMention(id: Param(id), phrase: Param(phrase)))",
    "(name: \"laterMention\", kinds: [NounPhrase], \
     params: {\"id\": String, \"phrase\": NounPhrase}, \
     body: LaterMention(id: Param(id), phrase: Param(phrase)))",
];

/// The names of the mention macros, which no card may write.
pub const MENTION_MACRO_NAMES: &[&str] = &["firstMention", "laterMention"];

/// Registers the mention macros with `macros`.
///
/// # Panics
/// If a definition above fails to read or register, which is a defect here.
pub fn register(macros: &mut MacroSet) {
    for source in MENTION_MACROS {
        let definition = macros
            .read_str::<MacroDef<()>>(source)
            .expect("the mention macro definitions read");
        macros
            .replace(&definition)
            .expect("the mention macro definitions register");
    }
}

/// The parameter types that name a referent.
const REFERENT_TYPES: &[&str] = &["Subject", "NounPhrase"];

/// `body` with every referent parameter it mentions more than once rewritten so
/// the first mention introduces and later mentions refer by identity, or `None`
/// when no parameter is mentioned twice. `name` is the declaration's name, the
/// stem of each identity.
#[must_use]
pub fn bind_first_mentions(name: &str, params: &Params, body: &str) -> Option<String> {
    let referents: Vec<String> = match params {
        Params::Positional(types) => types
            .iter()
            .enumerate()
            .filter(|(_, ty)| REFERENT_TYPES.contains(&ty.name.as_str()))
            .map(|(index, _)| index.to_string())
            .collect(),
        Params::Named(named) => named
            .iter()
            .filter(|(_, ty)| REFERENT_TYPES.contains(&ty.name.as_str()))
            .map(|(param, _)| param.as_str().to_owned())
            .collect(),
    };
    if referents.is_empty() {
        return None;
    }
    let holes = param_holes(body);
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for hole in &holes {
        *counts.entry(hole.param.as_str()).or_default() += 1;
    }
    let rebound: Vec<&String> = referents
        .iter()
        .filter(|param| counts.get(param.as_str()).copied().unwrap_or(0) > 1)
        .collect();
    if rebound.is_empty() {
        return None;
    }
    let mut out = String::with_capacity(body.len() + 64 * holes.len());
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut cursor = 0;
    for hole in &holes {
        if !rebound.iter().any(|param| **param == hole.param) {
            continue;
        }
        let text = &body[hole.start..hole.end];
        let form = if seen.insert(hole.param.as_str()) { "firstMention" } else { "laterMention" };
        out.push_str(&body[cursor..hole.start]);
        write!(
            out,
            "{form}(id: \"{name}.{param}\", phrase: {text})",
            param = hole.param
        )
        .expect("writing to a String cannot fail");
        cursor = hole.end;
    }
    out.push_str(&body[cursor..]);
    Some(out)
}

/// One `Param(p)` hole in a body: its byte span and the parameter it names.
struct Hole {
    start: usize,
    end: usize,
    param: String,
}

/// Every `Param(p)` hole in `body`, in textual order, skipping string literals
/// and comments.
fn param_holes(body: &str) -> Vec<Hole> {
    let bytes = body.as_bytes();
    let mut holes = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                i += 1;
            }
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    i += 1;
                }
                i += 2;
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                if &body[start..i] != "Param" {
                    continue;
                }
                let mut j = i;
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if bytes.get(j) != Some(&b'(') {
                    continue;
                }
                let Some(close) = body[j..].find(')') else {
                    continue;
                };
                let param = body[j + 1..j + close].trim().to_owned();
                let end = j + close + 1;
                holes.push(Hole { start, end, param });
                i = end;
            }
            _ => i += 1,
        }
    }
    holes
}

#[cfg(test)]
mod tests {
    use macro_ron::ParamType;
    use macro_ron::Params;

    use super::bind_first_mentions;

    fn positional(types: &[&str]) -> Params {
        Params::Positional(types.iter().map(|ty| ParamType::plain(*ty)).collect())
    }

    #[test]
    fn a_single_mention_is_untouched() {
        assert_eq!(
            bind_first_mentions("destroy", &positional(&["Subject"]), "Destroy(Param(0))"),
            None
        );
    }

    #[test]
    fn the_first_mention_binds_and_later_ones_refer() {
        assert_eq!(
            bind_first_mentions(
                "harness",
                &positional(&["Subject"]),
                "doIf(not(matches(Param(0), x)), gain(Param( 0 ), \"Param(0)\")) // Param(0)"
            )
            .unwrap(),
            "doIf(not(matches(firstMention(id: \"harness.0\", phrase: Param(0)), x)), \
             gain(laterMention(id: \"harness.0\", phrase: Param( 0 )), \"Param(0)\")) // Param(0)"
        );
    }

    #[test]
    fn only_referent_parameters_are_rebound() {
        assert_eq!(
            bind_first_mentions(
                "dredge",
                &positional(&["Amount"]),
                "seq([mill(Param(0)), draw(Param(0))])"
            ),
            None
        );
        let named = Params::Named(vec![("subject".into(), ParamType::plain("NounPhrase"))]);
        assert_eq!(
            bind_first_mentions("regen", &named, "a(Param(subject), Param(subject))").unwrap(),
            "a(firstMention(id: \"regen.subject\", phrase: Param(subject)), \
             laterMention(id: \"regen.subject\", phrase: Param(subject)))"
        );
    }
}
