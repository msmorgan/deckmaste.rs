//! The reserved witness vocabulary: what a frame's hole sigils are replaced
//! by so the frame text can be handed to an ordinary English parse.
//!
//! # Why witnesses at all
//!
//! A frame is authored with sigils — `<Param(0)>`, `~` — that are not
//! English. The compiler cannot parse them, and it must not teach the parser
//! about them ([`deckmaste_english`] is a leaf and stays one). So it
//! *substitutes*: every sigil becomes a token that parses at the category the
//! hole occupies, the whole frame parses as ordinary text, and the resulting
//! tree is then walked to put the holes back where the witnesses landed
//! (`crate::compile`'s relocation step). The witness is scaffolding; it never
//! survives into a [`CompiledFrame`](crate::compile::CompiledFrame).
//!
//! # The two carriers
//!
//! **Lexical opacity carries the phrasal holes.** An unknown noun is not an
//! error in `deckmaste_english` — it is
//! [`Noun::Opaque`](deckmaste_english::word::Noun), kept explicit rather than
//! guessed, and a fragment containing one is still
//! [`clean`](deckmaste_english::FragmentReport::clean). So `zzhole0` parses
//! as a noun anywhere a noun can stand, with **no catalog registration at
//! all**: the caller's [`Catalogs`](deckmaste_english::Catalogs) pass through
//! untouched. (The plan anticipated registering synthetic atoms into a
//! template-local catalog clone. That turned out to be both unnecessary and
//! actively harmful — `Catalogs::with_catalog` *replaces* a kind's source
//! list and there is no public reader for the old one, so a clone-and-add
//! would silently drop the caller's real vocabulary for that kind.)
//!
//! Opacity also buys number ambiguity for free, which is what makes the
//! inflection test work: the chart reads `zzplayer0` as singular in
//! `zzplayer0 draws …` and as plural in `zzplayer0 draw …`, so one witness
//! covers both authorings of a subject-alternating frame and no re-witnessing
//! pass is needed.
//!
//! **Reserved numerals carry the count holes**, because a numeral position is
//! not a noun position: `<Param(1)>` in `draw <Param(1)> cards` has to lex as
//! a number. The six values are arbitrary but *unmistakable* — every one is
//! a two-digit prime that no Magic card prints as a count — so a witness can
//! be recognized in the parsed tree by value alone.
//!
//! # Reserved, and enforced
//!
//! The ordinary carriers are reserved round-wide: an authored frame containing
//! `zz` in any case, or any of [`RESERVED_NUMERALS`], is a build error
//! ([`reserved_tokens`]). Without that check a frame could smuggle in text
//! that relocation would mistake for a hole.
//!
//! The singular retry uses `1`, which cannot be globally reserved because
//! literal `+1/+1` material is valid in frames. Its safety check is instead
//! structural: relocation still requires the witness to occur exactly once.

use std::fmt;

/// Every synthetic witness starts with this, so one case-insensitive
/// substring test rejects an authored frame that collides with the reserved
/// vocabulary.
pub const WITNESS_PREFIX: &str = "zz";

/// The face name a frame is parsed under, so `~` (self-reference) resolves.
/// Substituted into the frame text in place of the sigil and passed to
/// [`parse_fragment`](deckmaste_english::parse_fragment) as the card name,
/// which is what makes the parser build a
/// [`NounPhrase::ThisCard`](deckmaste_english::syntax::NounPhrase) there.
pub const SELF_WITNESS: &str = "Zzframeself";

/// The ordinary count-hole witnesses, one per numeric slot in a frame.
/// Two-digit primes: distinctive enough that a `41` in a parsed tree is a
/// witness and never a frame literal, given [`reserved_tokens`] rejects an
/// authored `41`.
pub const RESERVED_NUMERALS: [u32; 6] = [41, 43, 47, 53, 59, 61];

/// The singular-safe retry witness. The frame compiler uses it only when an
/// ordinary plural witness cannot parse a singular citation authoring such as
/// `<Count> card` under strict quantity/noun agreement.
pub const SINGULAR_NUMERAL: u32 = 1;

/// Which carrier a witness uses — the thing relocation looks for in the
/// parsed tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessKind {
    /// An opaque noun. Found in the tree as a `str` scalar equal to the
    /// witness text (the `OpaqueLexeme`'s spelling).
    Lexeme,
    /// A reserved numeral. Found in the tree as an integer scalar whose
    /// `repr` is the witness text.
    Numeral,
    /// The self-reference. Found *structurally* — as a `NounPhrase::ThisCard`
    /// node — because the card name never survives into the tree as a
    /// spelling.
    SelfReference,
}

/// One substituted witness: the text that replaced a sigil, and how to find
/// it again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub text: String,
    pub kind: WitnessKind,
}

impl fmt::Display for Witness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

impl Witness {
    /// The self-reference witness, standing for a `~` sigil.
    #[must_use]
    pub fn self_reference() -> Witness {
        Witness {
            text: SELF_WITNESS.to_string(),
            kind: WitnessKind::SelfReference,
        }
    }
}

/// Whether a declared param type is witnessed by a numeral rather than an
/// opaque noun.
///
/// Deliberately a short allow-list rather than a guess: a type that is *not*
/// listed gets the lexeme witness, and if that turns out to be ungrammatical
/// at the position the frame puts it in, the frame fails to parse and
/// `compile` reports it by name. A wrong guess here would instead produce a
/// frame that parses into the wrong shape, which is far worse.
#[must_use]
pub fn is_numeric_param(param_type: &str) -> bool {
    matches!(param_type, "Count" | "Counts" | "Uint")
}

/// The witness for the `slot`-th *numeric* hole in a frame.
///
/// # Errors
/// If a frame has more numeric holes than there are [`RESERVED_NUMERALS`].
pub fn numeral(slot: usize) -> anyhow::Result<Witness> {
    let value = RESERVED_NUMERALS.get(slot).copied().ok_or_else(|| {
        anyhow::anyhow!(
            "a frame may hold at most {} count holes (the reserved numerals {RESERVED_NUMERALS:?})",
            RESERVED_NUMERALS.len(),
        )
    })?;
    Ok(Witness {
        text: value.to_string(),
        kind: WitnessKind::Numeral,
    })
}

/// A numeric witness whose value licenses a singular count-noun surface.
#[must_use]
pub fn singular_numeral() -> Witness {
    Witness {
        text: SINGULAR_NUMERAL.to_string(),
        kind: WitnessKind::Numeral,
    }
}

/// The witness for a phrasal hole at param index `param`, whose declared type
/// is `param_type`.
///
/// The `zzplayer` / `zzhole` split is diagnostic only — both spell an opaque
/// noun and parse identically — but it makes a dumped compile
/// (`macro inspect`) readable, and a parse failure name the kind of slot that
/// failed.
#[must_use]
pub fn lexeme(param_type: &str, param: usize) -> Witness {
    let stem = if param_type == "Reference" { "zzplayer" } else { "zzhole" };
    Witness {
        text: format!("{stem}{param}"),
        kind: WitnessKind::Lexeme,
    }
}

/// The reserved tokens an authored frame text illegally contains, in the
/// order found — empty when the frame is clean.
///
/// Round-wide constraint: witnesses are reserved, so a frame may not spell
/// one itself. `zz` is matched case-insensitively and anywhere in a word
/// (`Zzframeself`, `zzhole0`, and any future witness family all start with
/// it); a reserved numeral is matched as a whole digit run, so `410` and
/// `141` are fine and a bare `41` is not.
///
/// The case fold is deliberately **ASCII-only**: the byte offset `find`
/// returns is applied back to `text`, so the fold has to be
/// length-preserving. [`str::to_lowercase`] is not — `İ` lowercases to 3
/// bytes from 2 and `ẞ` to 2 from 3 — and a frame text carrying either
/// before a `zz` would shift the offset off a `char` boundary and panic the
/// build. `WITNESS_PREFIX` is `zz`, pure ASCII, so
/// [`str::to_ascii_lowercase`] recognizes exactly the same set of carriers
/// with no such hazard.
#[must_use]
pub fn reserved_tokens(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let lowered = text.to_ascii_lowercase();
    if let Some(at) = lowered.find(WITNESS_PREFIX) {
        let run: String = text[at..]
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect();
        found.push(run);
    }
    let mut rest = text;
    while let Some(start) = rest.find(|c: char| c.is_ascii_digit()) {
        let digits: String = rest[start..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        if digits
            .parse::<u32>()
            .is_ok_and(|value| RESERVED_NUMERALS.contains(&value))
        {
            found.push(digits.clone());
        }
        rest = &rest[start + digits.len()..];
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_tokens_flag_both_carriers_and_nothing_else() {
        assert_eq!(
            reserved_tokens("draw <Param(0)> cards"),
            Vec::<String>::new()
        );
        assert_eq!(
            reserved_tokens("deals 410 damage to 141"),
            Vec::<String>::new()
        );
        assert_eq!(reserved_tokens("deals 41 damage"), vec!["41".to_string()]);
        assert_eq!(reserved_tokens("draw 1 card"), Vec::<String>::new());
        assert_eq!(
            reserved_tokens("draw <Param(1)> card"),
            Vec::<String>::new()
        );
        assert_eq!(
            reserved_tokens("Zzframeself gets +1/+1"),
            vec!["Zzframeself".to_string()]
        );
        // Case-insensitive, and every numeral in the run is reported.
        assert_eq!(
            reserved_tokens("gets +47/+53"),
            vec!["47".to_string(), "53".to_string()]
        );
    }

    /// The case fold has to be length-preserving, because the byte offset it
    /// produces is applied back to the *original* text. `İ` (U+0130) is 2
    /// bytes and `str::to_lowercase`s to 3; `ẞ` (U+1E9E) is 3 and lowercases
    /// to 2. Under the old `to_lowercase()` both of these shifted the `zz`
    /// offset off a `char` boundary and panicked. Verified non-vacuous by
    /// construction: each string below places a witness carrier *after* a
    /// non-length-preserving character, which is the only arrangement that
    /// can trip the slice.
    #[test]
    fn a_case_fold_that_is_not_length_preserving_does_not_panic_the_slice() {
        assert_eq!(reserved_tokens("İ zzhole0"), vec!["zzhole0".to_string()]);
        assert_eq!(reserved_tokens("ẞ zzhole0"), vec!["zzhole0".to_string()]);
        assert_eq!(reserved_tokens("İẞİ no carrier"), Vec::<String>::new());
    }

    #[test]
    fn numeral_witnesses_are_unique_per_slot_and_run_out_loudly() {
        let mut all: Vec<String> = (0..RESERVED_NUMERALS.len())
            .map(|slot| numeral(slot).unwrap().text)
            .collect();
        all.push(singular_numeral().text);
        let mut sorted = all.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), all.len());
        assert!(numeral(RESERVED_NUMERALS.len()).is_err());
    }

    #[test]
    fn every_witness_spelling_is_itself_reserved() {
        // The round-wide constraint has to be self-consistent: whatever the
        // table hands out must be exactly what `reserved_tokens` rejects.
        for text in [
            lexeme("Predicate", 0).text,
            lexeme("Reference", 2).text,
            Witness::self_reference().text,
            numeral(0).unwrap().text,
        ] {
            assert!(!reserved_tokens(&text).is_empty(), "{text} is not reserved");
        }
    }
}
