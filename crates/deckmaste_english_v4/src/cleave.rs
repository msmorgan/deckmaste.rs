//! Cleave's square brackets, transparent to the grammar.
//!
//! A cleave spell cast for its cleave cost has its text changed by "removing
//! all text found within square brackets in the spell's rules text"
//! [CR#702.148a]; the brackets mark that text and nothing else
//! (`docs/oracle-style-guide.md` §3, Bullets, vertical bars, and square
//! brackets). They are no constituent of the grammar, so a line of a cleave
//! face is read twice by the ordinary grammar: as its Uncleaved Text, the line
//! without the bracket characters, and as its Cleaved Text, the line without
//! the bracketed text. The bracketed spans are kept beside the Uncleaved Text,
//! outside any Reading, and the line is rebuilt from the two.

use std::ops::Range;

/// A line of a cleave face with balanced square brackets, read as its two
/// texts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleaveText {
    /// The line without its bracket characters.
    pub uncleaved: String,
    /// The line without its bracketed text: what the spell's text becomes when
    /// its cleave cost was paid [CR#702.148a].
    pub cleaved: String,
    /// The bracketed spans, as byte ranges of `uncleaved`.
    pub spans: Vec<Range<usize>>,
}

impl CleaveText {
    /// Splits a line at its square brackets, or `None` when it has no
    /// bracket pair, or a bracket that is nested or pairs with none.
    ///
    /// Removing the bracketed text leaves the spacing of the words around it:
    /// one space between two words, none before a period or comma, and none
    /// at either end of the line.
    #[must_use]
    pub fn split(line: &str) -> Option<Self> {
        let mut uncleaved = String::with_capacity(line.len());
        let mut cleaved = String::with_capacity(line.len());
        let mut spans = Vec::new();
        let mut open = None;
        let mut removed = false;
        for c in line.chars() {
            match (c, open) {
                ('[', None) => open = Some(uncleaved.len()),
                (']', Some(start)) => {
                    spans.push(start..uncleaved.len());
                    open = None;
                    removed = true;
                }
                ('[' | ']', _) => return None,
                (_, Some(_)) => uncleaved.push(c),
                (_, None) => {
                    uncleaved.push(c);
                    if removed {
                        removed = false;
                        let spaced = cleaved.is_empty() || cleaved.ends_with(' ');
                        if spaced && c == ' ' {
                            continue;
                        }
                        if matches!(c, '.' | ',') && cleaved.ends_with(' ') {
                            cleaved.pop();
                        }
                    }
                    cleaved.push(c);
                }
            }
        }
        if open.is_some() || spans.is_empty() {
            return None;
        }
        let cleaved = cleaved.trim_end().to_owned();
        Some(Self {
            uncleaved,
            cleaved,
            spans,
        })
    }

    /// The line as written: the Uncleaved Text with its brackets set back
    /// around its spans.
    #[must_use]
    pub fn rebuild(&self) -> String {
        let mut line = String::with_capacity(self.uncleaved.len() + 2 * self.spans.len());
        let mut at = 0;
        for span in &self.spans {
            line.push_str(&self.uncleaved[at..span.start]);
            line.push('[');
            line.push_str(&self.uncleaved[span.clone()]);
            line.push(']');
            at = span.end;
        }
        line.push_str(&self.uncleaved[at..]);
        line
    }
}

/// Whether a face's rules-text lines include a cleave keyword line,
/// *Cleave* and its cost [CR#702.148a], which makes the face's square
/// brackets cleave's.
#[must_use]
pub fn has_cleave(lines: &[String]) -> bool {
    lines.iter().any(|line| {
        line.strip_prefix("Cleave ")
            .is_some_and(|cost| cost.starts_with('{'))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each of the corpus's bracketed cleave lines, with its Uncleaved and
    /// Cleaved Texts.
    const LINES: &[(&str, &str, &str)] = &[
        (
            "Search your library for a [basic land] card, [reveal it,] put it into your hand, then shuffle.",
            "Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
            "Search your library for a card, put it into your hand, then shuffle.",
        ),
        (
            "Lantern Flare deals X damage to target creature or planeswalker and you gain X life. [X is the number of creatures you control.]",
            "Lantern Flare deals X damage to target creature or planeswalker and you gain X life. X is the number of creatures you control.",
            "Lantern Flare deals X damage to target creature or planeswalker and you gain X life.",
        ),
        (
            "Draw a card for each creature you control [with flying].",
            "Draw a card for each creature you control with flying.",
            "Draw a card for each creature you control.",
        ),
        (
            "Destroy target [attacking] creature.",
            "Destroy target attacking creature.",
            "Destroy target creature.",
        ),
        (
            "Return target nonland permanent [you control] to its owner's hand.",
            "Return target nonland permanent you control to its owner's hand.",
            "Return target nonland permanent to its owner's hand.",
        ),
        (
            "[X is the number of creatures you control.] Draw a card.",
            "X is the number of creatures you control. Draw a card.",
            "Draw a card.",
        ),
    ];

    #[test]
    fn a_line_splits_into_its_uncleaved_and_cleaved_texts_and_rebuilds() {
        for &(line, uncleaved, cleaved) in LINES {
            let split = CleaveText::split(line).unwrap();
            assert_eq!(split.uncleaved, uncleaved, "{line}");
            assert_eq!(split.cleaved, cleaved, "{line}");
            assert_eq!(split.rebuild(), line);
        }
    }

    #[test]
    fn a_line_without_one_balanced_unnested_pair_does_not_split() {
        for line in [
            "Destroy target creature.",
            "Destroy target [attacking creature.",
            "Destroy target attacking] creature.",
            "Destroy target [[attacking]] creature.",
            "Destroy target [attacking]] creature.",
        ] {
            assert_eq!(CleaveText::split(line), None, "{line}");
        }
    }

    #[test]
    fn a_cleave_keyword_line_marks_the_face() {
        let lines = |l: &[&str]| l.iter().map(|&s| s.to_owned()).collect::<Vec<_>>();
        assert!(has_cleave(&lines(&[
            "Cleave {1}{U}",
            "Counter target spell [that wasn't cast from its owner's hand]."
        ])));
        assert!(!has_cleave(&lines(&["[+1]: Draw a card."])));
        assert!(!has_cleave(&lines(&["Cleave the air."])));
    }
}
