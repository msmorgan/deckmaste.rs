//! Filling a macro's `template:` metadata into rules text.
//! `~` -> the subject; `{i}` -> the rendered i-th positional arg.

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

/// Fill a template. Returns `None` if any `{i}` can't be rendered cleanly
/// (caller then falls back to structural rendering — never emit a half-filled
/// template with a literal `{0}` left in it).
pub(super) fn fill(template: &str, subject: &str, args: &ExpansionArgs) -> Option<String> {
    let positional: &[String] = match args {
        ExpansionArgs::Positional(v) => v,
        ExpansionArgs::Named(_) => return None, // named-arg templates: not handled here
    };
    let mut out = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        match c {
            '~' => out.push_str(subject),
            '{' => {
                // parse the index up to '}'
                let mut idx = String::new();
                for (_, d) in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    idx.push(d);
                }
                let i: usize = idx.trim().parse().ok()?;
                let raw = positional.get(i)?;
                out.push_str(&render_arg(raw)?);
            }
            other => out.push(other),
        }
    }
    Some(out)
}

/// Render one raw-RON-source positional arg. v1: bare integers
/// (`PumpThisUntilEot`'s magnitudes) pass through verbatim; anything else
/// returns `None` so the caller falls back to structural rendering.
/// (Grammar-node args — filters, costs — are a later enhancement.)
fn render_arg(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.parse::<i64>().is_ok() { Some(t.to_string()) } else { None }
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
            Some("~ gets +{0}/+{1} until end of turn"),
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
            Some("{0}"),
            ExpansionArgs::Positional(vec!["SomeFilter".into()]),
        );
        assert_eq!(expanded(&e, "x"), None);
    }

    #[test]
    fn fills_pump_template() {
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +{0}/+{1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn returns_none_for_unknown_arg() {
        let args = ExpansionArgs::Positional(vec!["SomeFilter".into()]);
        let s = fill("do {0} things", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn returns_none_for_named_args() {
        let args = ExpansionArgs::Named(vec![("x".into(), "1".into())]);
        let s = fill("~ gets +{0}", "it", &args);
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
        // {1} doesn't exist
        let s = fill("~ gets +{0}/+{1}", "it", &args);
        assert_eq!(s, None);
    }
}
