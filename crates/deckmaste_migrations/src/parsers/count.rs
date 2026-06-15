//! Dynamic-count parsing: a trailing clause that resolves to
//! `Count::CountOf(<filter>)` ([CR#107.3], "for each"). Three surface forms,
//! all mapping to the same `CountOf`:
//!   ", where X is the number of <filter>"   (`Variable`)
//!   " for each <filter>"                     (`ForEach`)
//!   " equal to the number of <filter>"       (`EqualTo`)
//! Engine history tallies (`Count::Query`) are a deferred follow-up.

use std::sync::LazyLock;

use regex::Regex;

use crate::parsers::filter;

/// ", where <var> is the number of <filter>" — `.+` is greedy, so it anchors
/// on the LAST "where … is the number of" (the trailing clause).
static WHERE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r",?\s*where (\w+) is the number of (.+)$").unwrap());

/// How a peeled count clause binds to the amount left in the head text.
pub(super) enum Binder {
    /// "for each <filter>": the head carries a UNIT base the caller verifies.
    ForEach,
    /// ", where <var> is the number of <filter>": the head's amount word is
    /// `var`.
    Variable(String),
    /// "equal to the number of <filter>": the head has no amount slot.
    EqualTo,
}

/// A peeled dynamic-count clause.
pub(super) struct CountClause<'a> {
    /// `body` with the count clause removed (trailing comma/space trimmed).
    pub head: &'a str,
    /// `CountOf(<filter>)` RON.
    pub count: String,
    pub binder: Binder,
}

/// Peel a trailing dynamic-count clause off a normalized, period-stripped
/// effect `body`, or `None`. Strict: an unparseable filter declines — a wrong
/// filter would graduate a wrong card.
pub(super) fn strip(body: &str) -> Option<CountClause<'_>> {
    let body = body.trim_end();

    // ", where <var> is the number of <filter>"
    if let Some(caps) = WHERE_RE.captures(body) {
        let span = caps.get(0).unwrap();
        return Some(CountClause {
            head: body[..span.start()].trim_end(),
            count: count_of(&caps[2])?,
            binder: Binder::Variable(caps[1].to_string()),
        });
    }
    // " equal to the number of <filter>" (before the broader "for each").
    // `rfind`: the count clause is the trailing one — split at its last start.
    if let Some(idx) = body.rfind(" equal to the number of ") {
        let phrase = &body[idx + " equal to the number of ".len()..];
        return Some(CountClause {
            head: body[..idx].trim_end(),
            count: count_of(phrase)?,
            binder: Binder::EqualTo,
        });
    }
    // " for each <filter>"
    if let Some(idx) = body.rfind(" for each ") {
        let phrase = &body[idx + " for each ".len()..];
        return Some(CountClause {
            head: body[..idx].trim_end(),
            count: count_of(phrase)?,
            binder: Binder::ForEach,
        });
    }
    None
}

/// `<filter phrase>` -> `CountOf(<filter RON>)`, or `None` when the filter
/// doesn't parse.
fn count_of(phrase: &str) -> Option<String> {
    Some(format!("CountOf({})", filter::parse_phrase(phrase.trim())?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_each_maps_to_countof() {
        let c = strip("{G} for each Elf you control").unwrap();
        assert_eq!(c.head, "{G}");
        assert_eq!(
            c.count,
            "CountOf(AllOf([Permanent, Subtype(\"Elf\"), ControlledBy(Ref(You))]))"
        );
        assert!(matches!(c.binder, Binder::ForEach));
    }

    #[test]
    fn where_variable_maps_to_countof() {
        let c = strip(
            "Create X 1/1 red Goblin creature tokens, where X is the number of Goblins you control",
        )
        .unwrap();
        assert_eq!(c.head, "Create X 1/1 red Goblin creature tokens");
        assert_eq!(
            c.count,
            "CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))]))"
        );
        match c.binder {
            Binder::Variable(v) => assert_eq!(v, "X"),
            _ => panic!("expected Variable"),
        }
    }

    #[test]
    fn equal_to_maps_to_countof() {
        let c = strip("damage to any target equal to the number of Goblins you control").unwrap();
        assert_eq!(c.head, "damage to any target");
        assert_eq!(
            c.count,
            "CountOf(AllOf([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))]))"
        );
        assert!(matches!(c.binder, Binder::EqualTo));
    }

    #[test]
    fn on_the_battlefield_filter() {
        // "on the battlefield" is consumed, but the head still carries the
        // battlefield scope ([CR#109.2]) so a count never reaches a Stack-zone
        // copy of an Elf (e.g. a cast Elf spell).
        let c = strip("{G} for each Elf on the battlefield").unwrap();
        assert_eq!(c.count, "CountOf(AllOf([Permanent, Subtype(\"Elf\")]))");
    }

    #[test]
    fn no_clause_or_bad_filter_declines() {
        assert!(strip("{G}{G}").is_none());
        assert!(strip("Draw 3 cards").is_none());
        // Filter that filter::parse_phrase can't handle -> whole clause declines.
        assert!(strip("Add {G} for each creature wearing a hat").is_none());
    }
}
