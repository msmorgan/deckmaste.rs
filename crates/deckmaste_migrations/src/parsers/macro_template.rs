//! The macro-template parser: route an oracle line back to the macro whose
//! `template` renders it, via the reverse
//! [`TemplateIndex`](deckmaste_cards::template::index::TemplateIndex). It
//! claims whole-line keyword templates — nullary (`flying` → `Keyword(Flying)`)
//! and parameterized (`protection from black` →
//! `Keyword(Protection(ColorIs(Black)))`, each `${i}` slot filled via the typed
//! slot readers). It leads the registry, emitting the macro invocation instead
//! of a hand-built node; on no match it declines and the bespoke parsers take
//! over (first-match-wins).

use crate::resolve::ResolveCtx;

/// Recognize a whole-line `KeywordAbility` macro and emit its invocation:
/// nullary (`flying` → `Keyword(Flying)`) via the bare-name index, or
/// parameterized (`protection from black` →
/// `Keyword(Protection(ColorIs(Black)))`) by filling each `${i}` slot via the
/// typed slot readers. Declines (so a bespoke parser handles the line) on no
/// full-line match.
// Never errors, but must match the `AbilityParser` fn-pointer type.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn resolve_line(line: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let line = line.trim();
    // Nullary (param-less) keyword.
    if let Some(m) = ctx.index.match_kind("KeywordAbility", line)
        && m.consumed == line.len()
    {
        return Ok(Some(format!("Keyword({})", m.macro_name)));
    }
    // Parameterized keyword: fill `${i}` slots via the declared-type readers.
    if let Some(m) = ctx.index.match_with("KeywordAbility", line, slot_reader)
        && m.consumed == line.len()
    {
        return Ok(Some(format!("Keyword({})", m.invocation)));
    }
    Ok(None)
}

/// Read one keyword-template slot of declared type `ty` from the rest of the
/// line, reusing the bespoke keyword arg parsers. The slot is the line's tail,
/// so a successful read consumes all of `input`.
fn slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    let arg = match ty {
        "Filter" => super::keyword_ability::quality_filter(input.trim())?,
        "Cost" => super::keyword_ability::cost_arg(input).ok().flatten()?,
        _ => return None,
    };
    Some((arg, input.len()))
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
    fn claims_conditional_param_keyword() {
        // The conditional template now claims both forms via the same macro.
        let idx = builtin_index();
        assert_eq!(
            run("Hexproof", &idx).as_deref(),
            Some("Keyword(Hexproof())")
        );
        assert_eq!(
            run("hexproof from black", &idx).as_deref(),
            Some("Keyword(Hexproof(from: ColorIs(Black)))")
        );
    }

    #[test]
    fn routes_parameterized_keyword_through_slots() {
        // The `${0}` slot is filled by the declared-type reader: Filter for
        // Protection, Cost for Ward — same invocations the bespoke parser emits.
        let idx = builtin_index();
        assert_eq!(
            run("Protection from black", &idx).as_deref(),
            Some("Keyword(Protection(ColorIs(Black)))")
        );
        assert_eq!(
            run("Ward {2}", &idx).as_deref(),
            Some("Keyword(Ward([Mana([Generic(2)])]))")
        );
    }
}
