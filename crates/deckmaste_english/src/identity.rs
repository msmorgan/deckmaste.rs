//! Face identity for self-reference recognition.
//!
//! The parser and renderer both need to know which card face they are working
//! on so that the face's own name — written in full or as the 2024 Oracle
//! shortened "nickname" — is recognized as a self-reference rather than left as
//! opaque residue. [`short_name`] is the single shared derivation of the
//! shortened form; [`SelfReference`] precomputes the lexed token spellings the
//! scanner matches against.

use crate::numeral::Numeral;
use crate::surface::lex;

/// Derives the shortened name a legendary face uses to refer to itself, or
/// `None` when the face has no distinct shortened form.
///
/// A shortened self-reference is not a separate object: [CR#201.5c] states that
/// "instances of a card's shortened name used in this manner are treated as
/// though they used the card's full name." That is precisely why
/// [`crate::syntax::ThisCardForm::AbbreviatedName`] and
/// [`crate::syntax::ThisCardForm::FullName`] are two surface forms of one
/// referent and neither stores its spelling — the spelling is re-derived here
/// from the face name. The rule licenses treating both forms identically; it
/// does **not** enumerate the truncation patterns, which are `WotC` editorial
/// conventions the survey of the supported corpus fixes as:
///
/// 1. A non-legendary face has no shortened self-reference.
/// 2. A comma name shortens to its entire pre-comma string.
/// 3. A `The …` name is never shortened.
/// 4. A name ending in a canonical Roman numeral shortens to the name minus
///    that numeral (e.g. `King Darien XLVIII` → `King Darien`).
/// 5. Otherwise the prefix before the first ` the ` / ` of ` (e.g. `Agatha of
///    the Vile Cauldron` → `Agatha`, `Sidar Jabari of Zhalfir` → `Sidar
///    Jabari`).
/// 6. Otherwise the first word.
/// 7. A form equal to the full name (single-word names such as `Progenitus`) is
///    not a distinct shortened form.
#[must_use]
pub(crate) fn short_name(name: &str, is_legendary: bool) -> Option<&str> {
    if !is_legendary {
        return None;
    }
    let candidate = if let Some((pre_comma, _)) = name.split_once(',') {
        pre_comma.trim()
    } else if name.starts_with("The ") {
        return None;
    } else if let Some(without_numeral) = strip_trailing_roman_numeral(name) {
        without_numeral
    } else if let Some(prefix) = prefix_before_epithet(name) {
        prefix
    } else {
        name.split(' ').next().unwrap_or(name)
    };
    (candidate != name && !candidate.is_empty()).then_some(candidate)
}

/// Returns the name with a trailing canonical Roman-numeral word removed, or
/// `None` when the last word is not a canonical Roman numeral. Canonical form
/// is enforced by the shared numeral machinery, not a fresh pattern.
fn strip_trailing_roman_numeral(name: &str) -> Option<&str> {
    let (prefix, last_word) = name.rsplit_once(' ')?;
    Numeral::Roman.parse(last_word).ok()?;
    Some(prefix.trim_end())
}

/// Returns the prefix before the first ` the ` or ` of ` separator, whichever
/// occurs first, or `None` when neither appears.
fn prefix_before_epithet(name: &str) -> Option<&str> {
    [" the ", " of "]
        .into_iter()
        .filter_map(|separator| name.find(separator))
        .min()
        .map(|index| &name[..index])
}

/// The lexed token spellings of a face's full name and, when distinct, its
/// shortened name. The scanner matches these sequences against the source
/// tokens so a name that lexes to several tokens (commas, hyphens, apostrophes,
/// accented words) is recognized as one self-reference.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SelfReference {
    full_name: Vec<String>,
    nickname: Option<Vec<String>>,
}

impl SelfReference {
    /// Precomputes the self-reference token sequences for a face. An empty name
    /// (an anonymous parse) yields a value that recognizes nothing.
    #[must_use]
    pub(crate) fn new(name: &str, is_legendary: bool) -> Self {
        Self {
            full_name: token_spellings(name),
            nickname: short_name(name, is_legendary).map(token_spellings),
        }
    }

    /// The full-name token spellings; empty for an anonymous parse.
    pub(crate) fn full_name(&self) -> &[String] {
        &self.full_name
    }

    /// The shortened-name token spellings, when the face has a distinct one.
    pub(crate) fn nickname(&self) -> Option<&[String]> {
        self.nickname.as_deref()
    }
}

/// Lexes `text` and collects each token's exact source spelling.
fn token_spellings(text: &str) -> Vec<String> {
    lex(text)
        .tokens
        .iter()
        .filter_map(|token| token.span.text(text))
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_legendary_faces_have_no_shortened_name() {
        assert_eq!(short_name("Grizzly Bears", false), None);
        assert_eq!(short_name("Marit Lage's Slumber", false), None);
    }

    #[test]
    fn comma_names_shorten_to_the_pre_comma_string() {
        assert_eq!(short_name("Aang, A Lot to Learn", true), Some("Aang"));
        assert_eq!(short_name("Gut, True Soul Zealot", true), Some("Gut"));
        // A comma outranks a leading `The `.
        assert_eq!(
            short_name("The Balrog, Durin's Bane", true),
            Some("The Balrog")
        );
    }

    #[test]
    fn the_names_without_a_comma_are_never_shortened() {
        assert_eq!(short_name("The First Sliver", true), None);
        assert_eq!(short_name("The Ur-Dragon", true), None);
    }

    #[test]
    fn trailing_roman_numerals_are_stripped() {
        assert_eq!(short_name("King Darien XLVIII", true), Some("King Darien"));
        assert_eq!(short_name("Parhelion II", true), Some("Parhelion"));
        assert_eq!(
            short_name("Grand Arbiter Augustin IV", true),
            Some("Grand Arbiter Augustin")
        );
    }

    #[test]
    fn epithet_prefixes_pick_up_one_or_two_word_nicknames() {
        assert_eq!(short_name("Abaddon the Despoiler", true), Some("Abaddon"));
        assert_eq!(
            short_name("Agatha of the Vile Cauldron", true),
            Some("Agatha")
        );
        assert_eq!(
            short_name("Sidar Jabari of Zhalfir", true),
            Some("Sidar Jabari")
        );
        assert_eq!(short_name("Tor Wauki the Younger", true), Some("Tor Wauki"));
        assert_eq!(
            short_name("General Kudro of Drannith", true),
            Some("General Kudro")
        );
    }

    #[test]
    fn no_comma_no_epithet_names_shorten_to_the_first_word() {
        assert_eq!(short_name("Sliver Queen", true), Some("Sliver"));
        assert_eq!(short_name("Zurgo Bellstriker", true), Some("Zurgo"));
        assert_eq!(short_name("Nissa Revane", true), Some("Nissa"));
    }

    #[test]
    fn single_word_names_have_no_distinct_shortened_form() {
        assert_eq!(short_name("Progenitus", true), None);
        assert_eq!(short_name("Griselbrand", true), None);
    }

    fn full_name_of(name: &str, is_legendary: bool) -> Vec<String> {
        SelfReference::new(name, is_legendary).full_name().to_vec()
    }

    fn nickname_of(name: &str, is_legendary: bool) -> Option<Vec<String>> {
        SelfReference::new(name, is_legendary)
            .nickname()
            .map(<[String]>::to_vec)
    }

    #[test]
    fn self_reference_lexes_multi_token_names() {
        assert_eq!(
            full_name_of("Aang, A Lot to Learn", true),
            ["Aang", ",", "A", "Lot", "to", "Learn"]
        );
        assert_eq!(
            nickname_of("Aang, A Lot to Learn", true).as_deref(),
            Some(["Aang".to_owned()].as_slice())
        );
    }

    #[test]
    fn accented_and_connected_names_stay_single_tokens() {
        assert_eq!(
            full_name_of("Altaïr Ibn-La'Ahad", true),
            ["Altaïr", "Ibn-La'Ahad"]
        );
        assert_eq!(
            nickname_of("Altaïr Ibn-La'Ahad", true).as_deref(),
            Some(["Altaïr".to_owned()].as_slice())
        );
    }
}
