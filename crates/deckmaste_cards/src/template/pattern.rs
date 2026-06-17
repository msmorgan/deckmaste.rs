//! Compiling a macro `template` into a parse pattern — the reverse of
//! [`crate::render::template::fill`]. `~` = the subject; `${i}` / `${name}` =
//! a typed argument slot; single-brace `{…}` = a literal game symbol.

use macro_ron::Ident;
use macro_ron::Params;

/// One piece of a compiled template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Segment {
    /// Literal text, matched verbatim (case-folded) at match time.
    Literal(String),
    /// The subject/self marker `~`.
    SelfRef,
    /// A typed argument hole (`${i}` / `${name}`).
    Slot(Slot),
}

/// A `${…}` argument hole: which param it binds, plus that param's declared
/// type (read from the macro's `params`) — the type the slot codec dispatches
/// on. Carried now so slot-bearing patterns compile fully; the matcher only
/// resolves nullary patterns until the codec lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Slot {
    pub(crate) key: SlotKey,
    pub(crate) ty: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SlotKey {
    Index(usize),
    Name(Ident),
}

/// A macro's `template` compiled to an ordered segment sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsePattern {
    pub(crate) macro_name: Ident,
    pub(crate) segments: Vec<Segment>,
}

impl ParsePattern {
    /// No `${…}` slots — a pure-literal / self-only template, the shape the
    /// nullary matcher handles today.
    pub(crate) fn is_nullary(&self) -> bool {
        !self.segments.iter().any(|s| matches!(s, Segment::Slot(_)))
    }

    /// Total literal length, for specificity ordering (longer = more specific).
    pub(crate) fn literal_len(&self) -> usize {
        self.segments
            .iter()
            .map(|s| match s {
                Segment::Literal(t) => t.chars().count(),
                Segment::SelfRef => 1,
                Segment::Slot(_) => 0,
            })
            .sum()
    }
}

/// Compile a `template` (with the macro's `params`, for slot typing) into a
/// [`ParsePattern`]. Mirrors [`crate::render::template::fill`]'s scanner: `~`,
/// `${…}`, and single-brace-literal are recognized identically, so a template
/// renders and parses by the same rules.
pub(crate) fn compile(macro_name: Ident, template: &str, params: &Params) -> ParsePattern {
    let mut segments = Vec::new();
    let mut lit = String::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '~' => {
                flush(&mut lit, &mut segments);
                segments.push(Segment::SelfRef);
            }
            '$' if chars.peek() == Some(&'{') => {
                chars.next(); // consume the '{'
                let mut key = String::new();
                for d in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    key.push(d);
                }
                flush(&mut lit, &mut segments);
                segments.push(Segment::Slot(slot_for(key.trim(), params)));
            }
            other => lit.push(other),
        }
    }
    flush(&mut lit, &mut segments);
    ParsePattern {
        macro_name,
        segments,
    }
}

/// Move any accumulated literal text into a [`Segment::Literal`].
fn flush(lit: &mut String, segments: &mut Vec<Segment>) {
    if !lit.is_empty() {
        segments.push(Segment::Literal(std::mem::take(lit)));
    }
}

/// Resolve a slot's key and declared type against the macro's `params`: a
/// numeric key indexes a positional param, a name keys a named param. An
/// unresolvable type falls back to the empty `Ident` (the `Any`-like default).
fn slot_for(key: &str, params: &Params) -> Slot {
    if let Ok(i) = key.parse::<usize>() {
        let ty = match params {
            Params::Positional(v) => v.get(i).map_or_else(Ident::default, |p| p.name),
            Params::Named(_) => Ident::default(),
        };
        Slot {
            key: SlotKey::Index(i),
            ty,
        }
    } else {
        let ty = match params {
            Params::Named(m) => m.get(key).map_or_else(Ident::default, |p| p.name),
            Params::Positional(_) => Ident::default(),
        };
        Slot {
            key: SlotKey::Name(Ident::new(key)),
            ty,
        }
    }
}

#[cfg(test)]
mod tests {
    use macro_ron::ParamType;

    use super::*;

    fn pos(types: &[&str]) -> Params {
        Params::Positional(types.iter().map(|t| ParamType::plain(*t)).collect())
    }

    #[test]
    fn compiles_nullary_literal() {
        let p = compile("Flying".into(), "flying", &Params::default());
        assert_eq!(p.segments, vec![Segment::Literal("flying".into())]);
        assert!(p.is_nullary());
    }

    #[test]
    fn compiles_self_only() {
        let p = compile("AsEnters".into(), "as ~ enters", &Params::default());
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("as ".into()),
                Segment::SelfRef,
                Segment::Literal(" enters".into()),
            ]
        );
        assert!(p.is_nullary());
    }

    #[test]
    fn compiles_typed_slot_from_params() {
        let p = compile(
            "Protection".into(),
            "protection from ${0}",
            &pos(&["Filter"]),
        );
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("protection from ".into()),
                Segment::Slot(Slot {
                    key: SlotKey::Index(0),
                    ty: "Filter".into()
                }),
            ]
        );
        assert!(!p.is_nullary());
    }

    #[test]
    fn single_brace_is_literal_in_pattern() {
        // mana etc. pass through as literal, same rule as the renderer.
        let p = compile("M".into(), "add {C}", &Params::default());
        assert_eq!(p.segments, vec![Segment::Literal("add {C}".into())]);
    }
}
