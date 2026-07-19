//! The shared `Condition`-macro routing: an English condition phrase (the
//! clause after "if"/"as long as") → a `Condition`-kind macro invocation, via
//! the reverse
//! [`TemplateIndex`](deckmaste_cards::template::index::TemplateIndex).
//! A nullary phrase ("you have the city's blessing") matches through the
//! bare-name index; a slot-bearing phrase ("you control ${0}", "${0} is
//! ${1}") fills each `${i}` via [`slot_reader`]. The one routing point BOTH
//! condition positions share — [`crate::parsers::effect::parse_if`]'s one-shot
//! `If` and [`crate::parsers::static_ability`]'s `Conditionally` composer — so
//! a new condition phrasing is added by authoring a macro once, for either
//! position, per the "new condition phrasings are a macro, never a hand-rolled
//! recognizer" ruling.

use crate::parsers::filter;
use crate::resolve::ResolveCtx;

/// Route a condition phrase to its `Condition`-kind macro invocation. Tries
/// the bare-name (nullary) index first, then the slot-bearing one; `None` to
/// decline (an unrecognized phrase, or a match that doesn't consume the
/// phrase in full — both matchers already judge that internally).
///
/// # Errors
/// Propagates a same-kind ambiguous match from the index (a generation
/// error, not a decline — see [`TemplateIndex::match_kind`]).
pub(crate) fn resolve(phrase: &str, ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let phrase = phrase.trim();
    if let Some(m) = ctx.index.match_kind("Condition", phrase)? {
        return Ok(Some(m.macro_name.to_string()));
    }
    if let Some(m) = ctx.index.match_with("Condition", phrase, slot_reader)? {
        return Ok(Some(m.invocation));
    }
    Ok(None)
}

/// The closed subject-reference vocabulary a `Condition` template's
/// `Reference` slot accepts — exactly the self/attach-host subjects
/// [`crate::parsers::modify::subject_to_filter`] already grants the
/// static-effect side, spelled as the BARE `Reference` RON `Condition::Matches`
/// takes (its first slot is a bare `Reference`, not a `Predicate::Ref`
/// wrapper). Shared with [`crate::parsers::static_ability`]'s pronoun-anaphora
/// rewrite, so both sides recognize the identical subject set.
///
/// "equipped creature" is deliberately NOT included: the renderer's
/// `AttachHostOf(This)` subject phrase is hardcoded to "enchanted creature"
/// (a pre-existing gap in `crates/deckmaste_cards/src/render/fragment.rs`'s
/// `reference_subject`), so accepting it here would parse but never
/// round-trip — left `Unparsed` until that gap closes.
pub(crate) const SUBJECT_WORDS: &[(&str, &str)] = &[
    ("~", "This"),
    ("it", "This"),
    ("enchanted creature", "AttachHostOf(This)"),
    ("enchanted permanent", "AttachHostOf(This)"),
];

/// The typed slot reader for `Condition`-macro templates: a `Reference` slot
/// reads [`SUBJECT_WORDS`], bounded to its own word span (a literal like
/// " is " follows); a `Predicate` slot reads the REST of the phrase as an
/// object description ([`filter::parse_phrase`] — "an artifact", "a Human", "a
/// black creature"), or, when there's no head noun, a bare adjective/state
/// word this composition already has vocabulary for: a color, "legendary"
/// ([`deckmaste_core::Supertype::Legendary`]), or the
/// "attacking"/"blocking"/"tapped"/"untapped" combat/tap-state words.
fn slot_reader(ty: &str, input: &str) -> Option<(String, usize)> {
    match ty {
        "Reference" => subject_reference(input),
        "Predicate" => condition_predicate(input),
        _ => None,
    }
}

/// Match one of [`SUBJECT_WORDS`] at the START of `input`, requiring a
/// trailing word boundary so "it" doesn't prefix-match "items".
fn subject_reference(input: &str) -> Option<(String, usize)> {
    for (word, ron) in SUBJECT_WORDS {
        let Some(rest) = input.get(..word.len()) else {
            continue;
        };
        if !rest.eq_ignore_ascii_case(word) {
            continue;
        }
        let boundary = input[word.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric());
        if boundary {
            return Some(((*ron).to_string(), word.len()));
        }
    }
    None
}

/// A `Condition`-macro `Predicate` slot's reader — see [`slot_reader`]'s doc
/// for the vocabulary. Consumes the whole remaining phrase (the slot is the
/// condition clause's tail).
fn condition_predicate(input: &str) -> Option<(String, usize)> {
    let phrase = input.trim_end();
    if let Some(pred) = filter::parse_phrase(phrase) {
        return Some((pred, phrase.len()));
    }
    let bare = match phrase.to_ascii_lowercase().as_str() {
        "legendary" => "Supertype(Legendary)".to_owned(),
        "attacking" => "Attacking".to_owned(),
        "blocking" => "Blocking".to_owned(),
        "tapped" => "Status(Tapped)".to_owned(),
        "untapped" => "Status(Untapped)".to_owned(),
        other => {
            return filter::color_ident(other).map(|c| (format!("ColorIs({c})"), phrase.len()));
        }
    };
    Some((bare, phrase.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve::CardKind;

    fn resolve_builtin(phrase: &str) -> Option<String> {
        resolve(
            phrase,
            &crate::parsers::test_ctx::builtin_ctx(CardKind::Permanent),
        )
        .unwrap()
    }

    #[test]
    fn nullary_condition_macro_routes() {
        assert_eq!(
            resolve_builtin("you have the city's blessing").as_deref(),
            Some("YouHaveTheCitysBlessing")
        );
    }

    #[test]
    fn you_control_routes_with_an_object_predicate() {
        assert_eq!(
            resolve_builtin("you control an artifact").as_deref(),
            Some("YouControl(Type(Artifact))")
        );
    }

    #[test]
    #[cfg_attr(
        not(scryfall_catalogs),
        ignore = "needs data/catalogs (gitignored); catalog-dependent subtype parse"
    )]
    fn an_opponent_controls_routes() {
        assert_eq!(
            resolve_builtin("an opponent controls an Island").as_deref(),
            Some("AnOpponentControls(And([Permanent, Subtype(Island)]))")
        );
    }

    #[test]
    fn subject_is_routes_self_reference_and_bare_state() {
        assert_eq!(
            resolve_builtin("~ is attacking").as_deref(),
            Some("SubjectIs(This, Attacking)")
        );
        assert_eq!(
            resolve_builtin("it is untapped").as_deref(),
            Some("SubjectIs(This, Status(Untapped))")
        );
    }

    #[test]
    fn subject_is_routes_attach_host_and_bare_color() {
        assert_eq!(
            resolve_builtin("enchanted creature is black").as_deref(),
            Some("SubjectIs(AttachHostOf(This), ColorIs(Black))")
        );
    }

    #[test]
    fn subject_isnt_routes_negation() {
        assert_eq!(
            resolve_builtin("enchanted creature isn't legendary").as_deref(),
            Some("SubjectIsnt(AttachHostOf(This), Supertype(Legendary))")
        );
    }

    #[test]
    fn unrecognized_phrase_declines() {
        assert_eq!(resolve_builtin("the moon is full"), None);
    }
}
