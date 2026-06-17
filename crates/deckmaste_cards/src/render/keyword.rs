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
    k.as_str().to_string()
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

    use super::keyword_name;
    use crate::plugin::Plugin;

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
        assert_eq!(keyword_name(&kw("Ward([Mana([Generic(2)])])")), "ward {2}");
        assert_eq!(
            keyword_name(&kw("Equip([Mana([Generic(3)])])")),
            "equip {3}"
        );
    }
}
