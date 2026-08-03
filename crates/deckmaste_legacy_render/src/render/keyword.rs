//! Keyword abilities render to their printed name — or, when the keyword is a
//! macro carrying a fillable `template`, to the filled template so
//! parameterized keywords show their argument (`Protection from black`,
//! `Enchant creature`).

use deckmaste_core::ExpansionArgs;
use deckmaste_core::KeywordAbility;

/// The printed text of a keyword ability. A PARAMETERIZED keyword (its macro
/// carries args) renders via the filled `template` so its argument shows
/// (`Protection(ColorIs(Black))` → "protection from black"); a nullary keyword
/// renders as its bare printed name (`Flying`, `Deathtouch`, …), which keeps
/// the catalog's capitalization for keyword-only lines. A template that can't
/// fill its args (e.g. a cost arg, not yet renderable) also falls back to the
/// name.
pub(super) fn keyword_name(k: &KeywordAbility) -> String {
    if let KeywordAbility::Expanded(exp) = k
        && has_args(&exp.args)
        && let Some(text) = super::template::expanded(exp, "")
    {
        return text;
    }
    display_name(k.as_str())
}

/// The PRINTED spelling of a keyword's catalog name: multi-word keywords
/// carry an interior capital in the enum spelling (`DoubleStrike`,
/// `FirstStrike`) and print with a space and lowercase continuation
/// ("Double strike", "First strike").
fn display_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    for (i, c) in name.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            out.push(' ');
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn has_args(args: &ExpansionArgs) -> bool {
    match args {
        ExpansionArgs::Positional(v) => !v.is_empty(),
        ExpansionArgs::Named(m) => !m.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use deckmaste_plugin::plugin::Plugin;

    use super::keyword_name;

    fn kw(src: &str) -> deckmaste_core::KeywordAbility {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        plugin.macros.read_str(src).unwrap()
    }

    #[test]
    fn parameterized_keyword_renders_its_filter_arg_via_template() {
        assert_eq!(
            keyword_name(&kw("Protection(ColorIs(Black))")),
            "protection from black"
        );
        assert_eq!(
            keyword_name(&kw("Enchant(Type(Creature))")),
            "enchant creature"
        );
    }

    #[test]
    fn nullary_keyword_renders_its_capitalized_name() {
        // Nullary keywords keep the catalog's casing (keyword-only lines show
        // "Flying", not the lowercase template).
        assert_eq!(keyword_name(&kw("Flying")), "Flying");
    }

    #[test]
    fn cost_keyword_renders_its_cost_arg() {
        // Ward's blessed signature is named (`cost:` + the optional ward-{X}
        // `where_x`, [CR#702.21b]); a plain ward renders without the
        // where-X fragment, a ward-{X} with a supplied definition prints it.
        assert_eq!(
            keyword_name(&kw("Ward(cost: [Mana([Generic(2)])])")),
            "ward {2}"
        );
        assert_eq!(
            keyword_name(&kw("Equip([Mana([Generic(3)])])")),
            "equip {3}"
        );
    }

    #[test]
    fn ward_x_renders_its_where_x_clause() {
        assert_eq!(
            keyword_name(&kw(
                "Ward(cost: [Mana([Variable])],                  where_x: CounterCount(You, Experience))"
            )),
            "ward {X}, where X is the number of experience counters you have"
        );
    }
}
