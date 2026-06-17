//! Filling a macro's `template:` metadata into rules text.
//! `~` -> the subject; `${i}` / `${name}` -> the rendered positional / named
//! arg. Single-brace `{…}` is a literal game symbol (mana, `{T}`, …).

use deckmaste_core::Expansion;
use deckmaste_core::ExpansionArgs;

/// The one template-first hook: render a macro invocation through its own
/// rules-text `template`, if it carries a fillable one. `None` when there is no
/// template (or an arg can't be rendered) — the caller then falls back to
/// structural rendering of `e.value`. Every `X::Expanded` arm routes through
/// this so the "prefer the macro's text, else reconstruct it" rule is written
/// once, not re-derived per kind.
pub(super) fn expanded<T>(e: &Expansion<T>, subject: &str) -> Option<String> {
    fill(e.template.as_deref()?, subject, &e.args)
}

/// Fill a template. Returns `None` if any `${…}` can't be resolved/rendered
/// cleanly (caller then falls back to structural rendering — never emit a
/// half-filled template with a literal `${0}` left in it). A single-brace
/// `{…}` is a literal game symbol (mana, `{T}`, …) and passes through
/// untouched, which is why `${…}` — not `{…}` — is the placeholder sigil.
pub(super) fn fill(template: &str, subject: &str, args: &ExpansionArgs) -> Option<String> {
    let mut out = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        match c {
            '~' => out.push_str(subject),
            '$' if matches!(chars.peek(), Some((_, '{'))) => {
                chars.next(); // consume the '{'
                let mut content = String::new();
                for (_, d) in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    content.push(d);
                }
                if content.matches('#').count() == 2 {
                    // Conditional fragment: render prefix+value+suffix iff the value
                    // param is present; absent renders to nothing.
                    let mut parts = content.splitn(3, '#');
                    let prefix = parts.next().unwrap_or("");
                    let value = parts.next().unwrap_or("").trim();
                    let suffix = parts.next().unwrap_or("");
                    if let Some(raw) = lookup_arg(args, value) {
                        out.push_str(prefix);
                        out.push_str(&render_arg(raw)?);
                        out.push_str(suffix);
                    }
                } else {
                    out.push_str(&render_arg(lookup_arg(args, content.trim())?)?);
                }
            }
            other => out.push(other),
        }
    }
    Some(out)
}

/// Resolve a `${key}` reference against the invocation's args: a numeric key
/// indexes positional args; a name keys into named args. Named signatures carry
/// no indices, so a numeric key never resolves against them (and vice versa).
fn lookup_arg<'a>(args: &'a ExpansionArgs, key: &str) -> Option<&'a String> {
    match args {
        ExpansionArgs::Positional(v) => v.get(key.parse::<usize>().ok()?),
        ExpansionArgs::Named(pairs) => pairs
            .iter()
            .find(|(name, _)| name.as_str() == key)
            .map(|(_, raw)| raw),
    }
}

/// Render one raw-RON-source positional arg back to English (the `show`
/// direction): bare integers pass through; a `Filter` arg renders as its noun
/// (`ColorIs(Black)` → "black", `Type(Creature)` → "creature"); a `Cost` arg
/// renders as its symbols (`[Mana([Generic(2)])]` → "{2}"). Each is parsed with
/// the bare core reader, so the arg's type is recovered without a `MacroSet`.
/// Anything else (a filter with no clean noun, a verb-cost) returns `None`, so
/// the caller falls back to structural rendering.
fn render_arg(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.parse::<i64>().is_ok() {
        return Some(t.to_string());
    }
    if let Ok(filter) = deckmaste_core::ron::options().from_str::<deckmaste_core::Filter>(t) {
        let noun = super::fragment::filter_noun(&filter);
        if !noun.contains("[unrendered") {
            return Some(noun);
        }
    }
    // A Cost arg (`ward ${0}`, `equip ${0}`): a bracketed cost-component list.
    if let Ok(cost) =
        deckmaste_core::ron::options().from_str::<Vec<deckmaste_core::CostComponent>>(t)
    {
        return render_cost(&cost);
    }
    None
}

/// Render a cost-component list to its symbol/word text (`[Mana([Generic(2)])]`
/// → "{2}"). Declines on any component without a simple rendering (e.g. a
/// `Do(...)` verb cost), so the keyword falls back to its bare name.
fn render_cost(cost: &[deckmaste_core::CostComponent]) -> Option<String> {
    use deckmaste_core::CostComponent;
    let mut out = String::new();
    for component in cost {
        match component {
            CostComponent::Mana(mc) => out.push_str(&super::card::mana_cost(Some(mc))),
            CostComponent::Tap => out.push_str("{T}"),
            CostComponent::Untap => out.push_str("{Q}"),
            _ => return None,
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Expansion;
    use deckmaste_core::ExpansionArgs;

    use super::expanded;
    use super::fill;

    /// A placeholder expanded value: `expanded` never looks at it (it renders
    /// from `template`/`args`), so any type works.
    fn exp(template: Option<&str>, args: ExpansionArgs) -> Expansion<u8> {
        Expansion {
            name: "M".into(),
            args,
            template: template.map(str::to_owned),
            value: Box::new(0),
        }
    }

    #[test]
    fn expanded_prefers_a_nullary_template() {
        // The AnyTarget shape: nullary, no `~`, no `{i}` — the template *is* the
        // text, used in preference to reconstructing it from `value`.
        let e = exp(Some("any target"), ExpansionArgs::none());
        assert_eq!(expanded(&e, "ignored").as_deref(), Some("any target"));
    }

    #[test]
    fn expanded_fills_subject_and_args() {
        let e = exp(
            Some("~ gets +${0}/+${1} until end of turn"),
            ExpansionArgs::Positional(vec!["1".into(), "1".into()]),
        );
        assert_eq!(
            expanded(&e, "Goblin").as_deref(),
            Some("Goblin gets +1/+1 until end of turn")
        );
    }

    #[test]
    fn expanded_without_template_is_none() {
        // No template -> caller falls back to structural rendering of `value`.
        let e = exp(None, ExpansionArgs::none());
        assert_eq!(expanded(&e, "x"), None);
    }

    #[test]
    fn expanded_with_unrenderable_arg_is_none() {
        // A grammar-node arg the v1 filler can't render -> fall back, never a
        // half-filled template.
        let e = exp(
            Some("${0}"),
            ExpansionArgs::Positional(vec!["SomeFilter".into()]),
        );
        assert_eq!(expanded(&e, "x"), None);
    }

    #[test]
    fn fills_pump_template() {
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +${0}/+${1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn returns_none_for_unknown_arg() {
        let args = ExpansionArgs::Positional(vec!["SomeFilter".into()]);
        let s = fill("do ${0} things", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn tilde_expands_to_subject() {
        let args = ExpansionArgs::Positional(vec![]);
        let s = fill("~ is here", "Goblin Guide", &args);
        assert_eq!(s.as_deref(), Some("Goblin Guide is here"));
    }

    #[test]
    fn missing_positional_returns_none() {
        let args = ExpansionArgs::Positional(vec!["1".into()]);
        // ${1} doesn't exist
        let s = fill("~ gets +${0}/+${1}", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn fills_dollar_brace_positional() {
        // New sigil: `${i}` is the positional placeholder.
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +${0}/+${1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn fills_named_arg() {
        // `${name}` resolves against named args.
        let args =
            ExpansionArgs::Named(vec![("pow".into(), "2".into()), ("tou".into(), "3".into())]);
        let s = fill("~ gets +${pow}/+${tou} until end of turn", "Ogre", &args);
        assert_eq!(s.as_deref(), Some("Ogre gets +2/+3 until end of turn"));
    }

    #[test]
    fn single_brace_passes_through_literally() {
        // Single-brace `{…}` is a literal game symbol (mana), not a placeholder.
        let args = ExpansionArgs::Positional(vec![]);
        let s = fill("add {C}{C}", "x", &args);
        assert_eq!(s.as_deref(), Some("add {C}{C}"));
    }

    #[test]
    fn dollar_index_against_named_args_is_none() {
        // Named signatures have no indices: `${0}` can't resolve against named args.
        let args = ExpansionArgs::Named(vec![("x".into(), "1".into())]);
        let s = fill("~ gets +${0}", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn fills_conditional_present_and_absent() {
        let present = fill(
            "hexproof${ from #from#}",
            "ignored",
            &ExpansionArgs::Named(vec![("from".into(), "ColorIs(Blue)".into())]),
        );
        assert_eq!(present.as_deref(), Some("hexproof from blue"));

        let absent = fill("hexproof${ from #from#}", "ignored", &ExpansionArgs::none());
        assert_eq!(absent.as_deref(), Some("hexproof"));
    }
}
