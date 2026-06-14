//! Keyword abilities render to their printed name.

use deckmaste_core::KeywordAbility;

/// The printed name of a keyword ability ("Flying", "Deathtouch", "Defender",
/// ...). `Composite`/`Expanded` carry the macro name; intrinsics map to their
/// spelling.
pub(super) fn keyword_name(k: &KeywordAbility) -> String { k.as_str().to_string() }
