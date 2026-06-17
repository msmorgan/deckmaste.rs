//! Ability-line parsers for `resolve`'s registry: a normalized oracle line →
//! the bare ability RON of one ability, or `None` to decline.

pub(crate) mod activated_ability;
pub(crate) mod cost;
pub(crate) mod count;
pub(crate) mod effect;
pub(crate) mod filter;
pub(crate) mod keyword_ability;
pub(crate) mod macro_template;
pub(crate) mod mana_ability;
pub(crate) mod modify;
pub(crate) mod replacement;
pub(crate) mod spell_ability;
pub(crate) mod static_ability;
pub(crate) mod triggered_ability;

/// Shared test helper: a [`ResolveCtx`](crate::resolve::ResolveCtx) carrying an
/// empty reverse index, for the bespoke parsers' unit tests (the macro-template
/// parser declines on an empty index, so it never shadows the line under test).
#[cfg(test)]
pub(crate) mod test_ctx {
    use std::sync::LazyLock;

    use deckmaste_cards::template::index::TemplateIndex;

    use crate::resolve::CardKind;
    use crate::resolve::ResolveCtx;

    static EMPTY: LazyLock<TemplateIndex> = LazyLock::new(TemplateIndex::default);

    pub(crate) fn ctx(kind: CardKind) -> ResolveCtx<'static> {
        ResolveCtx {
            kind,
            index: &EMPTY,
        }
    }
}
