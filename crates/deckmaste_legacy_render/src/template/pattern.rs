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
    /// An optional fragment `${prefix#value#suffix}`: `prefix`/`suffix` are
    /// literal text, `slot` binds the `value` param. Rendered (and matched)
    /// only when that param is present.
    Conditional {
        prefix: String,
        slot: Slot,
        suffix: String,
    },
    /// A count-driven literal-repetition hole (`${slot*\{E\}}`): the `slot`
    /// count determines how many times `literal` repeats. Rendered by emitting
    /// `literal` count-many times; matched by consuming a run of `literal` and
    /// folding the run length back into the `slot` as a plain number.
    Repeat { slot: Slot, literal: String },
}

/// A `${…}` argument hole: which param it binds, plus that param's declared
/// type (read from the macro's `params`) — the type the slot codec dispatches
/// on. Carried now so slot-bearing patterns compile fully; the matcher only
/// resolves nullary patterns until the codec lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Slot {
    pub(crate) key: SlotKey,
    pub(crate) ty: Ident,
    /// The `:modifier` codec ([typed-holes delta 6]), if any: `+` (sign-aware)
    /// or `sing|plur` (plural-aware). Applied on BOTH sides: the render side
    /// ([`crate::render::template`]) formats through it, and the matcher
    /// (`index::read_slot`) strips the sign / consumes the agreed plural
    /// noun. `None` for a bare `${i}`.
    pub(crate) modifier: Option<String>,
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
    /// Whether the macro declares any params. A param-less macro is invoked as
    /// a bare `Name`; a macro *with* params (even all-defaulted, like
    /// `Hexproof`/`Landwalk`, whose templates carry no slot) needs `Name(...)`,
    /// so it must not be emitted as a bare nullary invocation.
    pub(crate) has_params: bool,
    /// The macro's declared `plural:` override (metadata), if any — carried
    /// through from `MacroDef::plural()` for the parse-side plural match a
    /// future pass wires up (see `deckmaste_plugin::render::template` for the
    /// render-side twin).
    pub(crate) plural: Option<String>,
}

impl ParsePattern {
    /// No `${…}` slots in the template — a pure-literal / self-only shape.
    pub(crate) fn is_nullary(&self) -> bool {
        !self.segments.iter().any(|s| {
            matches!(
                s,
                Segment::Slot(_) | Segment::Conditional { .. } | Segment::Repeat { .. }
            )
        })
    }

    /// Emittable as a bare `Keyword(Name)`: no template slots *and* no params
    /// (a defaulted-param macro with a slot-less template is excluded — it
    /// needs the `Name()` form).
    pub(crate) fn emits_bare(&self) -> bool {
        self.is_nullary() && !self.has_params
    }

    /// Total literal length, for specificity ordering (longer = more specific).
    pub(crate) fn literal_len(&self) -> usize {
        self.segments
            .iter()
            .map(|s| match s {
                Segment::Literal(t) => t.chars().count(),
                Segment::SelfRef => 1,
                Segment::Slot(_) | Segment::Conditional { .. } | Segment::Repeat { .. } => 0,
            })
            .sum()
    }
}

/// Compile a `template` (with the macro's `params`, for slot typing, and its
/// `plural` override, if any) into a [`ParsePattern`]. Mirrors
/// [`crate::render::template::fill`]'s scanner: `~`, `${…}`, and
/// single-brace-literal are recognized identically, so a template renders and
/// parses by the same rules.
pub(crate) fn compile(
    macro_name: Ident,
    template: &str,
    params: &Params,
    plural: Option<&str>,
) -> ParsePattern {
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
                // Escape-aware body read, mirroring `render::template::fill_with`:
                // a backslash escapes the next char (`\{`/`\}` are a literal brace
                // inside the body), and only an unescaped `}` closes the hole. The
                // backslash is dropped, the escaped char kept.
                let mut key = String::new();
                while let Some(d) = chars.next() {
                    if d == '\\' {
                        if let Some(e) = chars.next() {
                            key.push(e);
                        }
                        continue;
                    }
                    if d == '}' {
                        break;
                    }
                    key.push(d);
                }
                flush(&mut lit, &mut segments);
                if key.matches('#').count() == 2 {
                    let mut parts = key.splitn(3, '#');
                    let prefix = parts.next().unwrap_or("").to_owned();
                    let value = parts.next().unwrap_or("").trim();
                    let suffix = parts.next().unwrap_or("").to_owned();
                    segments.push(Segment::Conditional {
                        prefix,
                        slot: slot_for(value, params),
                        suffix,
                    });
                } else if let Some((slot_spec, literal)) = key.split_once('*') {
                    // Count-driven literal repetition (`${slot*\{E\}}`): the
                    // (already-unescaped) `literal` after `*` repeats `slot`-many
                    // times — the parse twin of `render::template`'s repeat branch.
                    segments.push(Segment::Repeat {
                        slot: slot_for(slot_spec.trim(), params),
                        literal: literal.to_owned(),
                    });
                } else {
                    // 0 `#` = an ordinary slot. A malformed count (1 or 3+) degrades to a
                    // slot whose key won't resolve — builtin templates are author-controlled
                    // and covered by the `keywords.rs` "every builtin macro expands" test.
                    segments.push(Segment::Slot(slot_for(key.trim(), params)));
                }
            }
            other => lit.push(other),
        }
    }
    flush(&mut lit, &mut segments);
    ParsePattern {
        macro_name,
        segments,
        has_params: has_params(params),
        plural: plural.map(str::to_owned),
    }
}

/// Whether a macro's signature declares any parameters.
fn has_params(params: &Params) -> bool {
    match params {
        Params::Positional(v) => !v.is_empty(),
        Params::Named(m) => !m.is_empty(),
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
/// A trailing `:modifier` ([typed-holes delta 6]) — `${0:+}`, `${n:card|cards}`
/// — is split off the key and carried on the slot.
fn slot_for(spec: &str, params: &Params) -> Slot {
    let (key, modifier) = match spec.split_once(':') {
        Some((key, modifier)) => (key.trim(), Some(modifier.trim().to_owned())),
        None => (spec.trim(), None),
    };
    if let Ok(i) = key.parse::<usize>() {
        let ty = match params {
            Params::Positional(v) => v.get(i).map_or_else(Ident::default, |p| p.name),
            Params::Named(_) => Ident::default(),
        };
        Slot {
            key: SlotKey::Index(i),
            ty,
            modifier,
        }
    } else {
        let ty = match params {
            Params::Named(m) => m
                .iter()
                .find(|(name, _)| name.as_str() == key)
                .map_or_else(Ident::default, |(_, p)| p.name),
            Params::Positional(_) => Ident::default(),
        };
        Slot {
            key: SlotKey::Name(Ident::new(key)),
            ty,
            modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use macro_ron::ParamType;
    use macro_ron::Params;

    use super::*;

    fn pos(types: &[&str]) -> Params {
        Params::Positional(types.iter().map(|t| ParamType::plain(*t)).collect())
    }

    #[test]
    fn compiles_nullary_literal() {
        let p = compile("Flying".into(), "flying", &Params::default(), None);
        assert_eq!(p.segments, vec![Segment::Literal("flying".into())]);
        assert!(p.is_nullary());
    }

    #[test]
    fn compiles_self_only() {
        let p = compile("AsEnters".into(), "as ~ enters", &Params::default(), None);
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
            &pos(&["Predicate"]),
            None,
        );
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("protection from ".into()),
                Segment::Slot(Slot {
                    key: SlotKey::Index(0),
                    ty: "Predicate".into(),
                    modifier: None,
                }),
            ]
        );
        assert!(!p.is_nullary());
    }

    /// [typed-holes delta 6] A `:modifier` codec is split off the key and
    /// carried on the slot, so a sign/plural template still compiles and
    /// resolves the param's declared type from the bare key.
    #[test]
    fn compiles_slot_with_a_modifier_codec() {
        let p = compile(
            "Mill".into(),
            "mills ${0:card|cards}",
            &pos(&["Count"]),
            None,
        );
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("mills ".into()),
                Segment::Slot(Slot {
                    key: SlotKey::Index(0),
                    ty: "Count".into(),
                    modifier: Some("card|cards".into()),
                }),
            ]
        );

        let sign = compile("Pump".into(), "gets ${0:+}", &pos(&["Count"]), None);
        assert_eq!(
            sign.segments,
            vec![
                Segment::Literal("gets ".into()),
                Segment::Slot(Slot {
                    key: SlotKey::Index(0),
                    ty: "Count".into(),
                    modifier: Some("+".into()),
                }),
            ]
        );
    }

    /// The repeat construct `${0*\{E\}}` compiles to a `Repeat` segment whose
    /// `literal` is the unescaped braced glyph. (Rust source doubles each
    /// backslash to spell the single-backslash template value.)
    #[test]
    fn compiles_repeat_construct() {
        let p = compile(
            "PayEnergy".into(),
            "Pay ${0*\\{E\\}}",
            &pos(&["Uint"]),
            None,
        );
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("Pay ".into()),
                Segment::Repeat {
                    slot: Slot {
                        key: SlotKey::Index(0),
                        ty: "Uint".into(),
                        modifier: None,
                    },
                    literal: "{E}".into(),
                },
            ]
        );
        assert!(!p.is_nullary());
        assert!(!p.emits_bare());
    }

    #[test]
    fn single_brace_is_literal_in_pattern() {
        // mana etc. pass through as literal, same rule as the renderer.
        let p = compile("M".into(), "add {C}", &Params::default(), None);
        assert_eq!(p.segments, vec![Segment::Literal("add {C}".into())]);
    }

    #[test]
    fn compiles_conditional_fragment() {
        let p = compile(
            "Hexproof".into(),
            "hexproof${ from #from#}",
            &Params::Named(
                [("from".into(), ParamType::plain("Predicate"))]
                    .into_iter()
                    .collect(),
            ),
            None,
        );
        assert_eq!(
            p.segments,
            vec![
                Segment::Literal("hexproof".into()),
                Segment::Conditional {
                    prefix: " from ".into(),
                    slot: Slot {
                        key: SlotKey::Name("from".into()),
                        ty: "Predicate".into(),
                        modifier: None,
                    },
                    suffix: String::new(),
                },
            ]
        );
        assert!(!p.is_nullary());
    }

    /// A macro's `plural:` override is carried onto the compiled pattern
    /// verbatim; a macro without one compiles with `plural: None`.
    #[test]
    fn compiles_carries_the_plural_override() {
        let p = compile(
            "Merfolk".into(),
            "Merfolk",
            &Params::default(),
            Some("Merfolk"),
        );
        assert_eq!(p.plural.as_deref(), Some("Merfolk"));

        let none = compile("Flying".into(), "flying", &Params::default(), None);
        assert_eq!(none.plural, None);
    }
}
