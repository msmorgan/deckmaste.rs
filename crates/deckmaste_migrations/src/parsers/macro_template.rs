//! The macro-template parser: route an oracle line back to the macro whose
//! `template` renders it, via the reverse
//! [`TemplateIndex`](deckmaste_cards::template::index::TemplateIndex). Today it
//! claims whole-line NULLARY keyword templates (`flying` → `Keyword(Flying)`);
//! slot-bearing templates (parameterized keywords) wait on the slot codec. It
//! leads the registry, emitting the macro invocation instead of a hand-built
//! node; on no match it declines and the bespoke parsers take
//! over (first-match-wins).

use crate::resolve::ResolveCtx;

/// Recognize a whole-line nullary `KeywordAbility` macro and emit its
/// invocation. Declines when the index has no full-line match, so a bespoke
/// parser (or the native-keyword path) handles the line.
// Never errors, but must match the `AbilityParser` fn-pointer type.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let line = line.trim();
    let Some(m) = ctx.index.match_kind("KeywordAbility", line) else {
        return Ok(None);
    };
    // A keyword-ability line *is* just the keyword: only claim a full-line
    // match, never a prefix.
    if m.consumed != line.len() {
        return Ok(None);
    }
    Ok(Some(format!("Keyword({})", m.macro_name)))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_cards::template::index::TemplateIndex;

    use super::*;
    use crate::resolve::CardKind;

    fn builtin_index() -> TemplateIndex {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        TemplateIndex::build(&plugin.macros)
    }

    fn run(line: &str, index: &TemplateIndex) -> Option<String> {
        resolve_line(
            line,
            &ResolveCtx {
                kind: CardKind::Permanent,
                index,
            },
        )
        .unwrap()
    }

    #[test]
    fn routes_nullary_keyword_through_the_index() {
        let idx = builtin_index();
        assert_eq!(run("Flying", &idx).as_deref(), Some("Keyword(Flying)"));
    }

    #[test]
    fn case_folds_the_match() {
        let idx = builtin_index();
        assert_eq!(run("flying", &idx).as_deref(), Some("Keyword(Flying)"));
    }

    #[test]
    fn declines_unknown_line() {
        let idx = builtin_index();
        assert_eq!(run("When ~ dies, draw a card.", &idx), None);
    }

    #[test]
    fn declines_on_empty_index() {
        let idx = TemplateIndex::default();
        assert_eq!(run("Flying", &idx), None);
    }

    #[test]
    fn declines_defaulted_param_keyword() {
        // Hexproof declares a defaulted param: its bare form is `Hexproof()`,
        // not `Hexproof`, so the index must NOT claim it here — the bespoke
        // keyword parser emits the correct parenthesized form.
        let idx = builtin_index();
        assert_eq!(run("Hexproof", &idx), None);
    }
}
