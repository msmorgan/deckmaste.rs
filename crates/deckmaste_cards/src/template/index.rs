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
use super::pattern::compile;

/// A successful reverse match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    /// The macro whose template matched.
    pub macro_name: Ident,
    /// How many bytes of `input` the match consumed (from the start).
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

    /// Match `input` against the nullary patterns of `kind`, most-specific
    /// first. Returns the matched macro and how much of `input` it consumed.
    #[must_use]
    pub fn match_kind(&self, kind: &str, input: &str) -> Option<Match> {
        for pattern in self.by_kind.get(kind)? {
            if pattern.is_nullary()
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
}
