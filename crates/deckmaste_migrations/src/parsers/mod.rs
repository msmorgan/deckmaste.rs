//! Ability-line parsers for `resolve`'s registry: a normalized oracle line →
//! the bare ability RON of one ability, or `None` to decline.

pub(crate) mod activated_ability;
pub(crate) mod cost;
pub(crate) mod count;
pub(crate) mod effect;
pub(crate) mod filter;
pub(crate) mod keyword_ability;
pub(crate) mod mana_ability;
pub(crate) mod modify;
pub(crate) mod spell_ability;
pub(crate) mod static_ability;
pub(crate) mod triggered_ability;
