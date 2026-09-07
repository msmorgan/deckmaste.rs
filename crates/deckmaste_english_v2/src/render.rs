use crate::constructions::CasePosition;
use crate::constructions::LexicalOwner;
use crate::constructions::PrefixPosition;
use crate::constructions::StructuralTransition;
use crate::context::ParseContext;
use crate::parser::TextSpan;
use crate::parser::ownership::RawRenderedClaim;

pub trait Render {
    fn render(
        &self,
        context: &ParseContext<'_>,
        environment: &crate::environment::ParserEnvironment,
    ) -> String;
}

pub(crate) enum ClaimSink<'a> {
    Noop,
    Collect(&'a mut Vec<RawRenderedClaim>),
}

pub(crate) struct Writer<'a> {
    output: String,
    case: CasePosition,
    prefix: PrefixPosition,
    last_word: Option<(usize, CasePosition)>,
    following_onset: Option<deckmaste_construction_core::macro_def::Onset>,
    claims: ClaimSink<'a>,
}

impl Writer<'_> {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            case: CasePosition::DocumentInitial,
            prefix: PrefixPosition::None,
            last_word: None,
            following_onset: None,
            claims: ClaimSink::Noop,
        }
    }

    pub(crate) fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
        Writer {
            output: String::new(),
            case: CasePosition::DocumentInitial,
            prefix: PrefixPosition::None,
            last_word: None,
            following_onset: None,
            claims: ClaimSink::Collect(claims),
        }
    }

    pub(crate) fn with_following_onset(
        &mut self,
        onset: Option<deckmaste_construction_core::macro_def::Onset>,
        render: impl FnOnce(&mut Self),
    ) {
        let previous = std::mem::replace(&mut self.following_onset, onset);
        render(self);
        self.following_onset = previous;
    }

    pub(crate) const fn following_onset(
        &self,
    ) -> Option<deckmaste_construction_core::macro_def::Onset> {
        self.following_onset
    }

    pub(crate) fn claim(
        &mut self,
        owner: impl FnOnce() -> LexicalOwner,
        render: impl FnOnce(&mut Self),
    ) {
        let start = self.output.len();
        render(self);
        let end = self.output.len();
        if let ClaimSink::Collect(claims) = &mut self.claims {
            claims.push(RawRenderedClaim {
                span: TextSpan { start, end },
                owner: owner(),
            });
        }
    }

    pub(crate) fn word(&mut self, word: &str) {
        match self.prefix {
            PrefixPosition::WordOwnedSpace => self.output.push(' '),
            PrefixPosition::SurfaceOwned | PrefixPosition::None => {}
        }
        let word_start = self.output.len();
        let word_case = self.case;
        if matches!(
            self.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        ) {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
        } else {
            self.output.push_str(word);
        }
        self.case = CasePosition::Continuation;
        self.prefix = PrefixPosition::WordOwnedSpace;
        self.last_word = Some((word_start, word_case));
    }

    pub(crate) fn suppress_next_space(&mut self) {
        self.prefix = PrefixPosition::SurfaceOwned;
    }

    pub(crate) fn bind_declared_suffix(&mut self) {
        let Some((start, CasePosition::Continuation)) = self.last_word else {
            self.suppress_next_space();
            return;
        };
        let Some(first) = self.output[start..].chars().next() else {
            self.suppress_next_space();
            return;
        };
        let replacement = first.to_lowercase().collect::<String>();
        let old_end = start + first.len_utf8();
        if replacement != first.to_string() {
            let delta = replacement.len().cast_signed() - first.len_utf8().cast_signed();
            self.output.replace_range(start..old_end, &replacement);
            if let ClaimSink::Collect(claims) = &mut self.claims {
                for claim in claims.iter_mut() {
                    if claim.span.start >= old_end {
                        claim.span.start = claim.span.start.saturating_add_signed(delta);
                    }
                    if claim.span.end >= old_end {
                        claim.span.end = claim.span.end.saturating_add_signed(delta);
                    }
                }
            }
        }
        self.suppress_next_space();
    }

    pub(crate) fn identity(&mut self, identity: &str) {
        if self.prefix == PrefixPosition::WordOwnedSpace {
            self.output.push(' ');
        }
        self.output.push_str(identity);
        self.case = CasePosition::Continuation;
        self.prefix = PrefixPosition::WordOwnedSpace;
        self.last_word = None;
    }

    pub(crate) fn punctuation(&mut self, mark: char) {
        self.output.push(mark);
        self.case = if mark == '.' {
            CasePosition::SentenceInitial
        } else {
            CasePosition::Continuation
        };
        self.prefix = PrefixPosition::WordOwnedSpace;
        self.last_word = None;
    }

    #[allow(
        dead_code,
        reason = "generated structural renderers use this ABI when the authored grammar has structural sequences"
    )]
    pub(crate) fn structural_surface(&mut self, surface: &str, transition: StructuralTransition) {
        self.output.push_str(surface);
        self.case = transition.case_after(self.case);
        self.prefix = PrefixPosition::SurfaceOwned;
        self.last_word = None;
    }

    #[cfg(test)]
    pub(crate) const fn case_position(&self) -> CasePosition {
        self.case
    }

    #[cfg(test)]
    pub(crate) const fn prefix_position(&self) -> PrefixPosition {
        self.prefix
    }

    pub(crate) fn finish(self) -> String {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::constructions::CasePosition;
    use crate::constructions::LexicalOwner;
    use crate::constructions::LexicalProvenanceKind;
    use crate::constructions::PrefixPosition;
    use crate::constructions::StructuralTransition;
    use crate::context::ParseContext;
    use crate::environment::canonical_test_environment;
    use crate::parser::Parser;

    #[test]
    fn render_provenance_claims_spaces_punctuation_and_structured_terminals() {
        let environment = canonical_test_environment();
        let parser = Parser::new(environment.clone()).unwrap();
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let ability = parser.parse("Destroy target creature.", &context).unwrap();
        let (rendered, claims) =
            crate::constructions::render_ability_with_claims(&ability, &context, &environment);
        assert_eq!(rendered, "Destroy target creature.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.span.start, claim.span.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 7, "lexeme:keyword_action/destroy/bare"),
                (7, 14, "vocab:TargetingMarker/Target"),
                (14, 23, "lexeme:type/creature/singular"),
                (23, 24, "structural:Sentences/sentences/terminator/0"),
            ]
        );
    }

    #[test]
    fn render_provenance_noop_sink_never_constructs_an_owner_label() {
        let labels = Cell::new(0);
        let mut writer = super::Writer::new();
        writer.claim(
            || {
                labels.set(labels.get() + 1);
                LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, "form:test/test/0")
            },
            |writer| writer.word("test"),
        );
        assert_eq!(writer.finish(), "Test");
        assert_eq!(labels.get(), 0);
    }

    #[test]
    fn declared_suffix_binding_applies_sentence_case_to_the_bound_word() {
        let mut initial = super::Writer::new();
        initial.word("Island");
        initial.bind_declared_suffix();
        initial.word("walk");
        assert_eq!(initial.finish(), "Islandwalk");

        let mut running = super::Writer::new();
        running.word("creature");
        running.word("Island");
        running.bind_declared_suffix();
        running.word("walk");
        assert_eq!(running.finish(), "Creature islandwalk");
    }

    #[test]
    fn structural_writer_separates_case_from_prefix_ownership() {
        let owner = |id| LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, id);
        let mut claims = Vec::new();
        let mut writer = super::Writer::collecting(&mut claims);
        writer.claim(|| owner("word:destroy"), |writer| writer.word("destroy"));
        writer.claim(|| owner("word:target"), |writer| writer.word("target"));
        writer.claim(|| owner("word:creature"), |writer| writer.word("creature"));
        writer.claim(|| owner("terminator"), |writer| writer.punctuation('.'));
        writer.claim(
            || owner("separator"),
            |writer| writer.structural_surface(" ", StructuralTransition::Preserve),
        );
        writer.claim(|| owner("word:you"), |writer| writer.word("you"));

        assert_eq!(writer.case_position(), CasePosition::Continuation);
        assert_eq!(writer.prefix_position(), PrefixPosition::WordOwnedSpace);
        assert_eq!(writer.finish(), "Destroy target creature. You");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.span.start, claim.span.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 7, "word:destroy"),
                (7, 14, "word:target"),
                (14, 23, "word:creature"),
                (23, 24, "terminator"),
                (24, 25, "separator"),
                (25, 28, "word:you"),
            ],
            "the separator-owned byte and following sentence-initial word are disjoint",
        );
    }

    #[test]
    fn structural_positional_separator_preserves_continuation_case_without_a_word_prefix() {
        let mut claims = Vec::new();
        let mut writer = super::Writer::collecting(&mut claims);
        writer.claim(
            || LexicalOwner::static_owner(LexicalProvenanceKind::Vocab, "word:alpha"),
            |writer| writer.word("alpha"),
        );
        writer.claim(
            || LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, "separator"),
            |writer| writer.structural_surface(", ", StructuralTransition::Preserve),
        );
        assert_eq!(writer.case_position(), CasePosition::Continuation);
        assert_eq!(writer.prefix_position(), PrefixPosition::SurfaceOwned);
        writer.claim(
            || LexicalOwner::static_owner(LexicalProvenanceKind::Vocab, "word:beta"),
            |writer| writer.word("beta"),
        );

        assert_eq!(writer.finish(), "Alpha, beta");
        assert_eq!(claims[1].span, crate::parser::TextSpan { start: 5, end: 7 });
        assert_eq!(
            claims[2].span,
            crate::parser::TextSpan { start: 7, end: 11 }
        );
    }
}
