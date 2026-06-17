//! A kind-scoped reverse index: `template → macro`. Built from a [`MacroSet`]'s
//! registered macros, it matches an oracle-text fragment back to the macro
//! whose template would render it. This ticket resolves NULLARY patterns (the
//! pure-literal / self-only templates); slot-bearing patterns are compiled and
//! held, but matching their `${…}` holes waits on the slot codec.

use std::collections::HashMap;

use macro_ron::Ident;
use macro_ron::MacroSet;

use super::pattern::ParsePattern;
use super::pattern::Segment;
use super::pattern::Slot;
use super::pattern::SlotKey;
use super::pattern::compile;

/// A successful reverse match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    /// The macro whose template matched.
    pub macro_name: Ident,
    /// How many bytes of `input` the match consumed (from the start).
    pub consumed: usize,
}

/// A successful slot-bearing match: the full macro invocation RON
/// (`Protection(ColorIs(Black))`) and how many bytes of `input` it consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotMatch {
    pub invocation: String,
    pub consumed: usize,
}

/// Kind-scoped reverse index over templated macros.
#[derive(Debug, Default)]
pub struct TemplateIndex {
    by_kind: HashMap<Ident, Vec<ParsePattern>>,
}

impl TemplateIndex {
    /// Build the index from every templated macro in `macros`, grouped by the
    /// kind it is registered under, each kind's patterns ordered most-specific
    /// first (greatest total literal length) so specific templates win.
    #[must_use]
    pub fn build(macros: &MacroSet) -> Self {
        let mut by_kind: HashMap<Ident, Vec<ParsePattern>> = HashMap::new();
        for (kind, def) in macros.iter() {
            let Some(template) = def.template() else { continue };
            by_kind
                .entry(*kind)
                .or_default()
                .push(compile(def.name, template, &def.params));
        }
        for patterns in by_kind.values_mut() {
            patterns.sort_by_key(|p| std::cmp::Reverse(p.literal_len()));
        }
        Self { by_kind }
    }

    /// Match `input` against the bare-emittable (param-less, slot-less)
    /// patterns of `kind`, most-specific first. Returns the matched macro
    /// and how much of `input` it consumed. Defaulted-param macros (e.g.
    /// `Hexproof`) are skipped here — they need the `Name(...)` form, not a
    /// bare nullary invocation.
    #[must_use]
    pub fn match_kind(&self, kind: &str, input: &str) -> Option<Match> {
        for pattern in self.by_kind.get(kind)? {
            if pattern.emits_bare()
                && let Some(consumed) = match_nullary(pattern, input)
            {
                return Some(Match {
                    macro_name: pattern.macro_name,
                    consumed,
                });
            }
        }
        None
    }

    /// Match `input` against the SLOT-bearing patterns of `kind`, filling each
    /// `${i}` via `slot_reader(declared_type, remaining_input) -> (arg_ron,
    /// consumed)` — codec-driven matching, so each slot is bounded by what its
    /// reader accepts, not by greedy literal capture. Returns the macro
    /// invocation. Nullary / defaulted-param patterns are not matched here (see
    /// [`Self::match_kind`]); a slot whose `slot_reader` declines fails the
    /// whole pattern.
    pub fn match_with<F>(&self, kind: &str, input: &str, mut slot_reader: F) -> Option<SlotMatch>
    where
        F: FnMut(&str, &str) -> Option<(String, usize)>,
    {
        for pattern in self.by_kind.get(kind)? {
            if pattern.is_nullary() {
                continue;
            }
            if let Some(m) = fill_pattern(pattern, input, &mut slot_reader) {
                return Some(m);
            }
        }
        None
    }
}

/// Format one matched slot argument: positional contributes the bare arg,
/// named the `name: arg` pair.
fn fmt_arg(key: &SlotKey, arg: &str) -> String {
    match key {
        SlotKey::Index(_) => arg.to_owned(),
        SlotKey::Name(name) => format!("{name}: {arg}"),
    }
}

/// Try to match an optional fragment (`prefix` literal, typed `slot`, `suffix`
/// literal) at `cursor`. Returns the slot's raw arg and the new cursor on a
/// full match, or `None` (the fragment is absent — caller leaves the cursor).
fn try_conditional<F>(
    prefix: &str,
    slot: &Slot,
    suffix: &str,
    input: &str,
    cursor: usize,
    slot_reader: &mut F,
) -> Option<(String, usize)>
where
    F: FnMut(&str, &str) -> Option<(String, usize)>,
{
    let after_prefix = cursor + prefix.len();
    if !input
        .get(cursor..after_prefix)?
        .eq_ignore_ascii_case(prefix)
    {
        return None;
    }
    let (arg, consumed) = slot_reader(slot.ty.as_str(), input.get(after_prefix..)?)?;
    let after_slot = after_prefix + consumed;
    let after_suffix = after_slot + suffix.len();
    if !input
        .get(after_slot..after_suffix)?
        .eq_ignore_ascii_case(suffix)
    {
        return None;
    }
    Some((arg, after_suffix))
}

/// Walk a slot-bearing pattern against `input`: literals match (case-folded),
/// each slot is read by `slot_reader`. Returns the invocation + bytes consumed,
/// or `None` if any literal mismatches or a slot reader declines.
fn fill_pattern<F>(pattern: &ParsePattern, input: &str, slot_reader: &mut F) -> Option<SlotMatch>
where
    F: FnMut(&str, &str) -> Option<(String, usize)>,
{
    let mut cursor = 0usize;
    let mut args: Vec<String> = Vec::new();
    for seg in &pattern.segments {
        match seg {
            Segment::Literal(t) => {
                if !input.get(cursor..cursor + t.len())?.eq_ignore_ascii_case(t) {
                    return None;
                }
                cursor += t.len();
            }
            Segment::SelfRef => {
                if input.get(cursor..cursor + 1)? != "~" {
                    return None;
                }
                cursor += 1;
            }
            Segment::Slot(slot) => {
                let (arg, consumed) = slot_reader(slot.ty.as_str(), input.get(cursor..)?)?;
                args.push(fmt_arg(&slot.key, &arg));
                cursor += consumed;
            }
            Segment::Conditional {
                prefix,
                slot,
                suffix,
            } => {
                if let Some((arg, new_cursor)) =
                    try_conditional(prefix, slot, suffix, input, cursor, slot_reader)
                {
                    args.push(fmt_arg(&slot.key, &arg));
                    cursor = new_cursor;
                }
                // Absent: no arg, cursor unchanged.
            }
        }
    }
    Some(SlotMatch {
        invocation: format!("{}({})", pattern.macro_name, args.join(", ")),
        consumed: cursor,
    })
}

/// Match a nullary pattern against the start of `input` (case-folded),
/// requiring a trailing word boundary. Returns the byte length consumed.
fn match_nullary(pattern: &ParsePattern, input: &str) -> Option<usize> {
    let mut target = String::new();
    for seg in &pattern.segments {
        match seg {
            Segment::Literal(t) => target.push_str(t),
            Segment::SelfRef => target.push('~'),
            Segment::Slot(_) => return None,
            Segment::Conditional { .. } => return None,
        }
    }
    let n = target.len();
    if !input.get(..n)?.eq_ignore_ascii_case(&target) {
        return None;
    }
    // Trailing word boundary: nullary "flash" must not match the start of
    // "flashbacky".
    if input[n..].chars().next().is_some_and(char::is_alphanumeric) {
        return None;
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::plugin::Plugin;

    fn builtin() -> TemplateIndex {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        TemplateIndex::build(&plugin.macros)
    }

    #[test]
    fn matches_nullary_keyword() {
        let m = builtin()
            .match_kind("KeywordAbility", "flying")
            .expect("flying matches");
        assert_eq!(m.macro_name.as_str(), "Flying");
        assert_eq!(m.consumed, "flying".len());
    }

    #[test]
    fn matches_any_target_targetspec() {
        let m = builtin()
            .match_kind("TargetSpec", "any target")
            .expect("any target matches");
        assert_eq!(m.macro_name.as_str(), "AnyTarget");
    }

    #[test]
    fn unknown_text_does_not_match() {
        assert!(builtin().match_kind("KeywordAbility", "blinking").is_none());
    }

    #[test]
    fn nullary_does_not_prefix_match_a_longer_word() {
        // A slot-bearing "flashback ${0}" is skipped; a nullary "flash" (if any)
        // must not eat the start of "flashback {2}".
        assert!(
            builtin()
                .match_kind("KeywordAbility", "flashback {2}")
                .is_none()
        );
    }

    #[test]
    fn fills_a_typed_slot_via_reader() {
        // Protection is now `params: [Filter]`; the slot reader is handed the
        // declared type and the remaining input, and returns the arg RON.
        let m = builtin()
            .match_with("KeywordAbility", "protection from black", |ty, rest| {
                assert_eq!(ty, "Filter");
                Some((format!("ColorIs({})", rest.trim()), rest.len()))
            })
            .expect("protection from <x> matches");
        assert_eq!(m.invocation, "Protection(ColorIs(black))");
        assert_eq!(m.consumed, "protection from black".len());
    }

    #[test]
    fn matches_conditional_named_and_absent() {
        let idx = builtin();
        let present = idx
            .match_with("KeywordAbility", "hexproof from blue", |ty, rest| {
                assert_eq!(ty, "Filter");
                Some((format!("ColorIs({})", rest.trim()), rest.len()))
            })
            .expect("hexproof from <x> matches");
        assert_eq!(present.invocation, "Hexproof(from: ColorIs(blue))");
        assert_eq!(present.consumed, "hexproof from blue".len());

        let absent = idx
            .match_with("KeywordAbility", "hexproof", |_, _| None)
            .expect("bare hexproof matches");
        assert_eq!(absent.invocation, "Hexproof()");
        assert_eq!(absent.consumed, "hexproof".len());
    }
}
