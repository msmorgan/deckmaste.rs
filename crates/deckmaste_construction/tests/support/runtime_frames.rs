use super::constructions;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextSpan {
    start: usize,
    end: usize,
}
macro_rules! engine_unit_tests {
    ($tests:item) => {};
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

mod engine {
    include!("../../../deckmaste_english_v2/src/parser/engine.rs");
}
use RulePosition::{Lexical as L, Nonterminal as N};
use engine::{Rule, RulePosition};
#[derive(Default)]
struct ParseContext<'a> {
    marker: std::marker::PhantomData<&'a ()>,
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

    fn suppress_next_space(&mut self) {
        self.prefix = PrefixPosition::SurfaceOwned;
    }

    fn identity(&mut self, identity: &str) {
        if self.prefix == PrefixPosition::WordOwnedSpace {
            self.output.push(' ');
        }
        self.output.push_str(identity);
        self.case = CasePosition::Continuation;
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
    #[expect(
        clippy::unused_self,
        reason = "the empty declaration inventory implements the generated scanner input ABI"
    )]
    fn declaration_readings(
        &self,
        _matcher: DeclarationMatcher,
        _boundary: LexicalBoundary,
    ) -> Vec<(
        usize,
        deckmaste_construction_core::macro_def::DeclarationIdentity,
        deckmaste_construction_core::macro_def::SurfaceFeature,
        deckmaste_construction_core::macro_def::Onset,
    )> {
        Vec::new()
    }

    fn word_end(&self, running_text: &str, right_boundary: LexicalBoundary) -> Option<usize> {
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(self.position.byte_offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let rendered = if matches!(
            self.position.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        ) {
            let mut chars = running_text.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
                .collect::<String>()
        } else {
            running_text.to_owned()
        };
        let end = self.position.byte_offset + prefix + rendered.len();
        let has_boundary = matches!(
            right_boundary,
            LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
        ) || match self.text.get(end..) {
            Some("") => true,
            Some(trailing) => trailing
                .chars()
                .next()
                .is_some_and(|character| !character.is_alphanumeric()),
            None => false,
        };
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
}

trait Render {
    fn render(&self, context: &ParseContext<'_>) -> String;
}

constructions! {
    vocab Noun { Object = "object", Creature = "creature", Artifact = "artifact", }
    vocab Preposition { With = "with", To = "to", }
    vocab Measure { One = "one", Two = "two", }
    vocab Coordinator { And = "and", Or = "or", AndOr = "and/or", }
    construction noun_phrase: NounPhrase {
        element Nominal { word: lex Noun, }
        form nominal = lex(word);
    }
    construction preposition_phrase: PrepositionPhrase {
        element Prepositional { preposition: lex Preposition, complement: NounPhrase, }
        form prepositional = lex(preposition) complement;
    }
    construction measure_phrase: MeasurePhrase {
        element Measured { word: lex Measure, }
        form measured = lex(word);
    }
    root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
    frame_family LexicalVerbPhrase {
        categories: [NounPhrase, PrepositionPhrase, MeasurePhrase],
        roles: [ObjectNounPhrase = Object(NounPhrase), Amount = Complement(MeasurePhrase)],
        coordinators: [Coordinator::And, Coordinator::Or, Coordinator::AndOr],
    }
}

use deckmaste_construction_core::macro_def::{
    DeclarationIdentity, DeclarationKind, FrameComplement, FrameItem, FrameLexicalRef,
    FrameRelation, SurfaceFeature, VerbFrame,
};
use lexical_verb_phrase::{Lexeme, LexicalVerbPhrase, PreparedGrammar, Value};

fn argument(relation: FrameRelation, category: &str) -> FrameItem {
    FrameItem::Argument(FrameComplement {
        relation,
        category: category.to_owned(),
    })
}
fn marked(category: &str) -> FrameItem {
    FrameItem::Marked(
        FrameLexicalRef {
            vocabulary: "Preposition".to_owned(),
            member: "To".to_owned(),
        },
        FrameComplement {
            relation: FrameRelation::Complement,
            category: category.to_owned(),
        },
    )
}
fn lexeme(name: &str, surface: &str, frames: Vec<VerbFrame>) -> Lexeme {
    Lexeme {
        reference: crate::environment::VerbInventoryRef::Declaration(DeclarationIdentity::new(
            DeclarationKind::KeywordAction,
            name,
        )),
        forms: vec![(SurfaceFeature::PLAIN, surface.to_owned())],
        frames,
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Candidate {
    value: Value,
    claims: Vec<RawRenderedClaim>,
}

fn parse(grammar: &PreparedGrammar, text: &str) -> Vec<Candidate> {
    fn materialize(
        grammar: &PreparedGrammar,
        forest: &engine::Forest<lexical_verb_phrase::RuleId, Value, LexicalOwner>,
        id: engine::NodeId,
        context: &ParseContext<'_>,
        memo: &mut std::collections::HashMap<engine::NodeId, Vec<Candidate>>,
    ) -> Vec<Candidate> {
        if let Some(values) = memo.get(&id) {
            return values.clone();
        }
        let node = forest.node(id);
        let mut values = Vec::new();
        for family in &node.families {
            let mut combinations = vec![(Vec::new(), Vec::new())];
            for child in &family.children {
                let children = match child {
                    engine::Child::Node(id) => materialize(grammar, forest, *id, context, memo),
                    engine::Child::Lexical(lexical) => vec![Candidate {
                        value: lexical.value.clone(),
                        claims: lexical
                            .owner
                            .as_ref()
                            .map(|owner| RawRenderedClaim {
                                start: lexical.span.start,
                                end: lexical.span.end,
                                owner: owner.clone(),
                            })
                            .into_iter()
                            .collect(),
                    }],
                };
                combinations = combinations
                    .into_iter()
                    .flat_map(|(values, claims)| {
                        children.iter().map(move |child| {
                            let mut values = values.clone();
                            values.push(child.value.clone());
                            let mut claims = claims.clone();
                            claims.extend(child.claims.clone());
                            (values, claims)
                        })
                    })
                    .collect();
            }
            for (children, claims) in combinations {
                if let Ok(Some(value)) = grammar.build(node.rule, &children, context) {
                    let candidate = Candidate { value, claims };
                    if !values.contains(&candidate) {
                        values.push(candidate);
                    }
                }
            }
        }
        memo.insert(id, values.clone());
        values
    }
    let context = ParseContext::default();
    let rules = grammar.rules();
    let forest = engine::parse_with_state(
        &rules,
        lexical_verb_phrase::Category::Root,
        text.len(),
        &ScanPosition {
            byte_offset: 0,
            case: CasePosition::DocumentInitial,
            prefix: PrefixPosition::None,
        },
        |terminal, offset, position, suppress| {
            assert_eq!(offset, position.byte_offset);
            let terminal = if suppress { terminal.suppress_right_boundary() } else { terminal };
            grammar
                .scan(
                    &ScanInput {
                        text,
                        position: terminal.position_before(*position),
                        context: &context,
                    },
                    terminal,
                )
                .into_iter()
                .map(|found| engine::StatefulLexicalMatch {
                    state: terminal.position_after(*position, found.end),
                    lexical: engine::LexicalMatch {
                        end: found.end,
                        value: found.value,
                        owner: found.owner,
                    },
                })
                .collect()
        },
        |_, _, _| true,
    );
    let Ok(forest) = forest else {
        return Vec::new();
    };
    let mut memo = std::collections::HashMap::new();
    let mut candidates = Vec::new();
    for root in forest.accepted_root_ids() {
        for candidate in materialize(grammar, &forest, root, &context, &mut memo) {
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }
    candidates
}

fn roundtrip(grammar: &PreparedGrammar, text: &str) -> Vec<LexicalVerbPhrase> {
    let candidates = parse(grammar, text);
    assert!(!candidates.is_empty(), "{text:?} has a checked parse");
    let mut phrases = Vec::new();
    for candidate in candidates {
        let Value::Phrase(phrase) = candidate.value else {
            panic!("root must build a phrase")
        };
        let mut claims = Vec::new();
        let mut writer = Writer::collecting(&mut claims);
        phrase
            .write(
                grammar,
                &mut writer,
                &ParseContext::default(),
                &crate::environment::ParserEnvironment::new(Vec::new()),
            )
            .unwrap();
        assert_eq!(writer.finish(), text);
        assert_eq!(
            claims, candidate.claims,
            "all source ranges and lexical identities survive materialization"
        );
        phrases.push(phrase);
    }
    phrases
}

#[test]
fn runtime_schema_is_shared_and_new_order_is_data_only() {
    let np = argument(FrameRelation::Object, "NounPhrase");
    let frame = vec![np.clone()];
    let prepare = std::time::Instant::now();
    let grammar = PreparedGrammar::new(&[
        lexeme("FirstAct", "act", vec![frame.clone()]),
        lexeme("SecondAct", "perform", vec![frame]),
    ])
    .unwrap();
    let prepare_elapsed = prepare.elapsed();
    assert_eq!(grammar.schema_count(), 1);
    let parsing = std::time::Instant::now();
    let first = roundtrip(&grammar, "Act object");
    let second = roundtrip(&grammar, "Perform creature");
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(first[0].schema(), second[0].schema());
    assert_ne!(first[0].head().reference(), second[0].head().reference());
    eprintln!(
        "shared schema: rules={}, preparation={prepare_elapsed:?}, parse+render={:?}",
        grammar.prepared_rule_count(),
        parsing.elapsed()
    );

    let ordered = vec![
        np,
        FrameItem::Optional(Box::new(marked("PrepositionPhrase"))),
        argument(FrameRelation::Complement, "MeasurePhrase"),
    ];
    let prepare = std::time::Instant::now();
    let changed = PreparedGrammar::new(&[lexeme("FirstAct", "act", vec![ordered])]).unwrap();
    let prepare_elapsed = prepare.elapsed();
    let parsing = std::time::Instant::now();
    let present = roundtrip(&changed, "Act object to with creature one");
    let absent = roundtrip(&changed, "Act object two");
    assert_eq!(present.len(), 1);
    assert_eq!(absent.len(), 1);
    assert_eq!(present[0].parts().len(), 3);
    assert_eq!(absent[0].parts().len(), 2);
    assert!(parse(&changed, "Act object").is_empty());
    assert!(parse(&changed, "Act one object").is_empty());
    eprintln!(
        "ordered optional schema: rules={}, preparation={prepare_elapsed:?}, parse+render={:?}",
        changed.prepared_rule_count(),
        parsing.elapsed()
    );
}

#[test]
fn checked_frame_rejects_wrong_category_missing_member_and_wrong_head() {
    use lexical_verb_phrase::{Part, Rejection};
    let grammar = PreparedGrammar::new(&[
        lexeme(
            "Act",
            "act",
            vec![vec![
                argument(FrameRelation::Object, "NounPhrase"),
                argument(FrameRelation::Complement, "MeasurePhrase"),
            ]],
        ),
        lexeme("Wait", "wait", vec![vec![]]),
    ])
    .unwrap();
    let phrase = roundtrip(&grammar, "Act creature one").remove(0);
    let mut wrong = phrase.parts().to_vec();
    let Part::Member(measure) = wrong.pop().unwrap() else { panic!() };
    let Part::Member(noun) = &mut wrong[0] else { panic!() };
    noun.child = measure.child;
    assert_eq!(
        LexicalVerbPhrase::try_new(&grammar, phrase.head().clone(), phrase.schema(), wrong),
        Err(Rejection::WrongMember)
    );
    assert_eq!(
        LexicalVerbPhrase::try_new(
            &grammar,
            phrase.head().clone(),
            phrase.schema(),
            phrase.parts()[..1].to_vec()
        ),
        Err(Rejection::MissingRequired)
    );
    assert_eq!(
        LexicalVerbPhrase::try_new(
            &grammar,
            grammar.head(1).unwrap(),
            phrase.schema(),
            phrase.parts().to_vec()
        ),
        Err(Rejection::WrongHead)
    );
}

#[test]
fn frame_parts_coordinate_nonempty_same_schema_intervals() {
    use lexical_verb_phrase::Part;
    let grammar = PreparedGrammar::new(&[lexeme(
        "Act",
        "act",
        vec![vec![
            argument(FrameRelation::Object, "NounPhrase"),
            argument(FrameRelation::Complement, "MeasurePhrase"),
        ]],
    )])
    .unwrap();
    for coordinator in ["and", "or", "and/or"] {
        let text = format!("Act object one {coordinator} creature two {coordinator} artifact one");
        let phrases = roundtrip(&grammar, &text);
        assert_eq!(phrases.len(), 1);
        let [
            Part::Coordination {
                start,
                end,
                conjuncts,
                ..
            },
        ] = phrases[0].parts()
        else {
            panic!("one frame-part coordination")
        };
        assert_eq!((*start, *end), (0, 2));
        assert_eq!(conjuncts.len(), 3);
        assert!(conjuncts.iter().all(|members| members.len() == 2));
    }
    assert!(parse(&grammar, "Act object one and creature").is_empty());
    assert!(parse(&grammar, "Act and").is_empty());
}

#[test]
fn environment_rejects_unresolved_frame_references() {
    use deckmaste_construction_core::frame::FrameSchemaError;
    let cases = [
        (
            argument(FrameRelation::Object, "Unknown"),
            FrameSchemaError::UnknownCategory("Unknown".to_owned()),
        ),
        (
            FrameItem::Role("Unknown".to_owned()),
            FrameSchemaError::UnknownRole("Unknown".to_owned()),
        ),
        (
            FrameItem::Fixed(FrameLexicalRef {
                vocabulary: "Preposition".to_owned(),
                member: "Unknown".to_owned(),
            }),
            FrameSchemaError::UnknownLexicalReference(
                "Preposition".to_owned(),
                "Unknown".to_owned(),
            ),
        ),
    ];
    for (item, expected) in cases {
        let error = PreparedGrammar::new(&[lexeme("Act", "act", vec![vec![item]])])
            .err()
            .expect("unknown schema reference rejects during preparation");
        assert_eq!(error, expected);
    }
}

#[test]
fn generated_frame_keeps_typed_children_and_total_leaf_traversal() {
    use lexical_verb_phrase::{Child, FrameVisitor, Head, Marker, Member, Part};
    #[derive(Default)]
    struct Leaves {
        words: Vec<String>,
        frames: usize,
        constructions: usize,
    }
    impl Visitor for Leaves {
        fn visit_noun(&mut self, value: Noun) {
            self.words.push(format!("{value:?}"));
        }
        fn visit_preposition(&mut self, value: Preposition) {
            self.words.push(format!("{value:?}"));
        }
        fn visit_nominal(&mut self, value: &Nominal) {
            self.constructions += 1;
            walk_nominal(self, value);
        }
    }
    impl FrameVisitor for Leaves {
        fn visit_frame(&mut self, _: &LexicalVerbPhrase) {
            self.frames += 1;
        }
        fn visit_head(&mut self, head: &Head) {
            let crate::environment::VerbInventoryRef::Declaration(identity) = head.reference()
            else {
                panic!()
            };
            self.words.push(identity.name().to_owned());
        }
    }
    let grammar = PreparedGrammar::new(&[lexeme(
        "Act",
        "act",
        vec![vec![
            argument(FrameRelation::Object, "NounPhrase"),
            FrameItem::Optional(Box::new(FrameItem::Fixed(FrameLexicalRef {
                vocabulary: "Preposition".to_owned(),
                member: "To".to_owned(),
            }))),
        ]],
    )])
    .unwrap();
    let phrase = roundtrip(&grammar, "Act creature to").remove(0);
    assert_eq!(
        phrase.parts(),
        [
            Part::Member(Member {
                position: 0,
                relation: Some(FrameRelation::Object),
                marker: None,
                child: Some(Child::NounPhrase(
                    NounPhrase::NounPhrase(Nominal {
                        word: Noun::Creature
                    }),
                    FeatureConstraint::Any
                ))
            }),
            Part::Member(Member {
                position: 1,
                relation: None,
                marker: Some(Marker::resolve("Preposition", "To").unwrap()),
                child: None
            }),
        ]
    );
    assert_eq!(
        roundtrip(&grammar, "Act creature")[0].parts(),
        &phrase.parts()[..1]
    );
    let mut leaves = Leaves::default();
    phrase.walk(&mut leaves);
    assert_eq!(leaves.words, ["Act", "Creature", "To"]);
    assert_eq!((leaves.frames, leaves.constructions), (1, 1));
}

#[test]
fn optional_intervals_cannot_create_empty_conjuncts() {
    let grammar = PreparedGrammar::new(&[lexeme(
        "Act",
        "act",
        vec![vec![FrameItem::Optional(Box::new(argument(
            FrameRelation::Object,
            "NounPhrase",
        )))]],
    )])
    .unwrap();
    assert_eq!(roundtrip(&grammar, "Act").len(), 1);
    assert_eq!(roundtrip(&grammar, "Act object and creature").len(), 1);
    for text in ["Act and", "Act object and", "Act and creature"] {
        assert!(
            parse(&grammar, text).is_empty(),
            "{text:?} cannot coordinate an empty frame interval"
        );
    }
}

#[test]
fn standalone_child_punctuation_does_not_enter_the_frame() {
    let noun = NounPhrase::NounPhrase(Nominal {
        word: Noun::Creature,
    });
    assert_eq!(noun.render(&ParseContext::default()), "Creature.");
    let grammar = PreparedGrammar::new(&[lexeme(
        "Act",
        "act",
        vec![vec![argument(FrameRelation::Object, "NounPhrase")]],
    )])
    .unwrap();
    assert_eq!(roundtrip(&grammar, "Act creature").len(), 1);
}
