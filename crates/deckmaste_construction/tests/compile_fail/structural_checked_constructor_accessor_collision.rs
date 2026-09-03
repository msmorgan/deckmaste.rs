#![allow(
    dead_code,
    unused_imports,
    reason = "the compile-fail fixture supplies the complete generated-code environment"
)]

use deckmaste_construction::constructions;

pub mod environment {
    pub(crate) struct ParserEnvironment;
}

mod fixture {
    use super::constructions;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TextSpan {
        start: usize,
        end: usize,
    }

    macro_rules! engine_unit_tests {
        ($tests:item) => {};
    }

    mod engine {
        include!("../../../deckmaste_english_v2/src/parser/engine.rs");
    }

    use RulePosition::Lexical as L;
    use RulePosition::Nonterminal as N;
    use engine::LexicalMatch as EngineLexicalMatch;
    use engine::Rule;
    use engine::RulePosition;
    use engine::StatefulLexicalMatch as EngineStatefulLexicalMatch;

    #[derive(Default)]
    struct ParseContext<'a> {
        marker: std::marker::PhantomData<&'a ()>,
    }

    trait Render {
        fn render(&self, context: &ParseContext<'_>) -> String;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RawRenderedClaim {
        start: usize,
        end: usize,
        owner: LexicalOwner,
    }

    enum ClaimSink<'a> {
        Noop,
        Collect(&'a mut Vec<RawRenderedClaim>),
    }

    struct Writer<'a> {
        output: String,
        case: CasePosition,
        prefix: PrefixPosition,
        claims: ClaimSink<'a>,
    }

    impl Writer<'_> {
        fn new() -> Self {
            Self {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
                claims: ClaimSink::Noop,
            }
        }

        fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
            Writer {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
                claims: ClaimSink::Collect(claims),
            }
        }

        fn claim(&mut self, owner: impl FnOnce() -> LexicalOwner, render: impl FnOnce(&mut Self)) {
            let start = self.output.len();
            render(self);
            let end = self.output.len();
            if let ClaimSink::Collect(claims) = &mut self.claims {
                claims.push(RawRenderedClaim {
                    start,
                    end,
                    owner: owner(),
                });
            }
        }

        fn word(&mut self, word: &str) {
            if self.prefix == PrefixPosition::WordOwnedSpace {
                self.output.push(' ');
            }
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
        }

        fn punctuation(&mut self, punctuation: char) {
            self.output.push(punctuation);
            self.case = if punctuation == '.' {
                CasePosition::SentenceInitial
            } else {
                CasePosition::Continuation
            };
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn structural_surface(&mut self, surface: &str, transition: StructuralTransition) {
            self.output.push_str(surface);
            self.case = transition.case_after(self.case);
            self.prefix = PrefixPosition::SurfaceOwned;
        }

        fn finish(self) -> String {
            self.output
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct LexicalMatch<T, O = ()> {
        end: usize,
        value: T,
        owner: Option<O>,
    }

    struct ScanInput<'a> {
        text: &'a str,
        position: ScanPosition,
        context: &'a ParseContext<'a>,
    }

    impl ScanInput<'_> {
        fn word_end(&self, running_text: &str) -> Option<usize> {
            let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let rendered = if matches!(
                self.position.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ) {
                let mut characters = running_text.chars();
                characters
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(characters)
                    .collect::<String>()
            } else {
                running_text.to_owned()
            };
            let end = self.position.byte_offset + prefix + rendered.len();
            let has_boundary = matches!(
                self.text.as_bytes().get(end),
                None | Some(b' ' | b',' | b'.')
            );
            (remainder.starts_with(&rendered) && has_boundary).then_some(end)
        }

        fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(punctuation)
                .then_some(self.position.byte_offset + punctuation.len())
        }

        fn structural_surface_end(&self, surface: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(surface)
                .then_some(self.position.byte_offset + surface.len())
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
        ) -> Vec<(
            usize,
            deckmaste_construction_core::macro_def::DeclarationIdentity,
            deckmaste_construction_core::macro_def::SurfaceFeature,
        )> {
            debug_assert_eq!(self.context.marker, std::marker::PhantomData);
            Vec::new()
        }
    }

    fn scan_bound_terminal(
        _input: &ScanInput<'_>,
        _terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        Vec::new()
    }

    constructions! {
        construction item: Item {
            element ItemValue {}
            form item = "item";
        }
        abstract product Holder { try_new: seq Item, }
        require len(Holder.try_new) >= 1;
        root Holder { eoi = true; standalone_render = true; }
    }

    #[cfg(feature = "parser-metrics")]
    mod metrics {
        pub(crate) enum MetricEvent {
            Prediction,
            Completion,
            CloneHeavy,
        }

        pub(crate) fn record(_rule_index: usize, _event: MetricEvent) {}

        pub(crate) fn record_work(_chart_columns_visited: u64, _scan_attempts: u64) {}
    }
}

fn main() {}
