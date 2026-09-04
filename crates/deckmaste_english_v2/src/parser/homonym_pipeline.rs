use std::collections::BTreeSet;

use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;
use deckmaste_construction::constructions;
use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::SurfaceFeature;

use super::diagnostic::SemanticScannerMatchInventory;
use super::engine::Child;
use super::engine::Family;
use super::engine::Forest;
use super::engine::LexicalMatch;
use super::engine::Observation;
use super::engine::Rule;
use super::engine::RulePosition;
use super::engine::StatefulLexicalMatch;
use super::engine::parse_observed_with_state;
use super::materialize::materialize_with;
use crate::environment::ParserEnvironment;

#[derive(Debug, Default)]
struct ParseContext<'a>(std::marker::PhantomData<&'a ()>);

trait Render {
    fn render(&self, context: &ParseContext<'_>, environment: &ParserEnvironment) -> String;
}

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
    capitalize_next: bool,
    claims: ClaimSink<'a>,
}

impl Writer<'_> {
    fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
            claims: ClaimSink::Noop,
        }
    }

    fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
        Writer {
            output: String::new(),
            capitalize_next: true,
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
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        if self.capitalize_next {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
            self.capitalize_next = false;
        } else {
            self.output.push_str(word);
        }
    }

    fn punctuation(&mut self, punctuation: char) {
        self.output.push(punctuation);
    }

    #[allow(
        dead_code,
        reason = "the synthetic grammar uses the generated structural renderer ABI conditionally"
    )]
    fn structural_surface(&mut self, surface: &str, transition: StructuralTransition) {
        self.output.push_str(surface);
        let current = if self.capitalize_next {
            CasePosition::SentenceInitial
        } else {
            CasePosition::Continuation
        };
        self.capitalize_next = matches!(
            transition.case_after(current),
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        );
    }

    fn finish(self) -> String {
        self.output
    }
}

struct ScanInput<'a> {
    text: &'a str,
    position: ScanPosition,
    environment: &'a ParserEnvironment,
}

impl ScanInput<'_> {
    fn word_end(&self, running_text: &str, right_boundary: LexicalBoundary) -> Option<usize> {
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(self.position.byte_offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let rendered = if self.position.case == CasePosition::DocumentInitial {
            crate::orthography::initial_surface(running_text)
        } else {
            running_text.to_owned()
        };
        let end = self.position.byte_offset + prefix + rendered.len();
        (remainder.starts_with(&rendered)
            && (matches!(
                right_boundary,
                LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
            ) || self
                .text
                .get(end..)
                .and_then(|tail| tail.chars().next())
                .is_none_or(|character| !character.is_alphanumeric())))
        .then_some(end)
    }

    fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
        self.text
            .get(self.position.byte_offset..)?
            .starts_with(punctuation)
            .then_some(self.position.byte_offset + punctuation.len())
    }

    fn structural_surface_end(&self, surface: &str) -> Option<usize> {
        self.text
            .get(self.position.byte_offset..)?
            .starts_with(surface)
            .then_some(self.position.byte_offset + surface.len())
    }

    fn declaration_readings(
        &self,
        matcher: DeclarationMatcher,
        _right_boundary: LexicalBoundary,
    ) -> Vec<(usize, DeclarationIdentity, SurfaceFeature, Onset)> {
        super::scan::lookup_declaration_readings(
            self.text,
            self.position.byte_offset,
            self.position.case == CasePosition::DocumentInitial,
            self.environment,
            matcher.kind,
            matcher.name,
            matcher.position,
            |feature| match matcher.feature {
                FeatureConstraint::Any => true,
                FeatureConstraint::Exact(expected) => feature == expected,
            },
        )
        .into_iter()
        .map(|(end, id, feature)| {
            let onset = self
                .environment
                .onset(&id, feature)
                .expect("normalized declaration reading has frozen onset");
            (end, id, feature, onset)
        })
        .collect()
    }
}

constructions! {
    vocab StructuralWord {
        Alpha = "alpha",
        Beta = "beta",
        Gamma = "gamma",
        Delta = "delta",
    }
    construction structural_atom: StructuralAtom {
        element StructuralAtomValue { marker: lex StructuralWord, }
        form structural_atom = lex(marker);
    }
    construction guarded_structural: GuardedStructural {
        element GuardedStructuralValue { marker: lex StructuralWord, }
        require marker is Alpha;
        form guarded_structural = lex(marker);
    }
    abstract product OptionalStructural { maybe: opt StructuralAtom, }
    abstract product ExactPairStructural {
        items: seq StructuralAtom separated by position { pair = "<P>"; } terminated by "<T>",
    }
    require len(ExactPairStructural.items) = 2;
    abstract product TerminatedStructural {
        items: seq StructuralAtom terminated by "<T>",
    }
    abstract product SeparatedStructural {
        items: seq StructuralAtom separated by "<S>",
    }
    abstract product CombinedStructural {
        items: seq StructuralAtom separated by "<S>" terminated by "<T>",
    }
    abstract product PositionalStructural {
        items: seq StructuralAtom separated by position {
            pair = "<P>";
            first = "<F>";
            middle = "<M>";
            last = "<L>";
        } terminated by "<T>",
    }
    require len(PositionalStructural.items) >= 1;
    abstract product BoundedUniformStructural {
        items: seq StructuralAtom separated by "<S>" terminated by "<T>",
    }
    require len(BoundedUniformStructural.items) >= 2;
    require len(BoundedUniformStructural.items) <= 4;
    abstract product BoundedPositionalStructural {
        items: seq StructuralAtom separated by position {
            pair = "<P>";
            first = "<F>";
            middle = "<M>";
            last = "<L>";
        } terminated by "<T>",
    }
    require len(BoundedPositionalStructural.items) >= 2;
    require len(BoundedPositionalStructural.items) <= 4;
    construction action: Homonym {
        element Action {}
        derive concord_class = verb.concord_class;
        derive verb.concord_class = Values::Other;
        form action = open_verb(KeywordAction, "Destroy");
    }
    construction ability: Homonym {
        element Ability {}
        derive concord_class = verb.concord_class;
        derive verb.concord_class = Values::Other;
        form ability = open_verb(KeywordAbility, "Destroy");
    }
    construction nested_root_leaf: NestedRoot {
        element NestedRootValue { marker: lex StructuralWord, }
        form nested_root_leaf = lex(marker);
    }
    construction outer_root: OuterRoot {
        element OuterRootValue { inner: NestedRoot, }
        form outer_root = inner;
    }
    root Homonym { punctuation = "."; eoi = true; standalone_render = true; }
    root NestedRoot { punctuation = "!"; eoi = true; standalone_render = false; }
    root OuterRoot { punctuation = "."; eoi = true; standalone_render = true; }
}

fn environment() -> ParserEnvironment {
    ParserEnvironment::try_from_declarations([
        deckmaste_construction_core::macro_def::read_str(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"homonym",grammar:Verb(bare:"homonym",frame_set:Intransitive))"#,
        )
        .unwrap(),
        deckmaste_construction_core::macro_def::read_str(
            "/synthetic/abilities/Destroy.ron",
            r#"KeywordAbility(name:"Destroy",spelling:"homonym",grammar:Verb(bare:"homonym",frame_set:Intransitive))"#,
        )
        .unwrap(),
    ])
    .unwrap()
}

#[derive(Default)]
struct IdentityTrace {
    scanner_matches: SemanticScannerMatchInventory<Lexical, (Leaf, Option<LexicalOwner>)>,
}

impl IdentityTrace {
    fn finish(self) -> Vec<String> {
        self.scanner_matches
            .into_bounded_by(
                usize::MAX,
                |left, right| format!("{left:?}").cmp(&format!("{right:?}")),
                |left, right| format!("{left:?}").cmp(&format!("{right:?}")),
                |terminal| format!("{terminal:?}"),
                |(_, owner)| {
                    owner
                        .as_ref()
                        .map_or_else(|| "none".to_owned(), |owner| owner.stable_id().to_owned())
                },
            )
            .items()
            .iter()
            .map(|token| token.value_label_v1().to_owned())
            .collect()
    }
}

impl Observation<RuleId, Leaf, LexicalTerminal, LexicalOwner> for IdentityTrace {
    fn scanned(&mut self, start: usize, terminal: LexicalTerminal, end: usize, value: &Leaf) {
        if matches!(value, Leaf::Declaration(_)) {
            self.scanner_matches.record_projected(
                start,
                end,
                terminal,
                value,
                |terminal| terminal.matcher,
                |terminal, value| (value.clone(), terminal.owner.instantiate(value)),
            );
        }
    }
}

#[derive(Default)]
struct IdentityVisitor(Vec<DeclarationIdentity>);

impl Visitor for IdentityVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0.push(declaration.clone());
    }
}

fn scan(
    text: &str,
    position: ScanPosition,
    terminal: LexicalTerminal,
    environment: &ParserEnvironment,
) -> Vec<StatefulLexicalMatch<Leaf, LexicalOwner, ScanPosition>> {
    let scan_position = terminal.position_before(position);
    scan_lexical(
        &ScanInput {
            text,
            position: scan_position,
            environment,
        },
        terminal,
    )
    .into_iter()
    .map(|lexical| StatefulLexicalMatch {
        state: terminal.position_after(position, lexical.end),
        lexical,
    })
    .collect()
}

fn parse_fixture(
    category: Category,
    text: &str,
    environment: &ParserEnvironment,
) -> Result<
    Forest<RuleId, Leaf, LexicalOwner>,
    super::engine::ChartFailure<Category, LexicalTerminal>,
> {
    parse_observed_with_state(
        RULES,
        category,
        text.len(),
        &ScanPosition {
            byte_offset: 0,
            case: CasePosition::DocumentInitial,
            prefix: PrefixPosition::None,
        },
        |terminal, offset, position, suppress_right_boundary| {
            debug_assert_eq!(offset, position.byte_offset);
            let terminal = if suppress_right_boundary {
                terminal.suppress_right_boundary()
            } else {
                terminal
            };
            scan(text, *position, terminal, environment)
        },
        |_rule, _family, _forest| true,
        &mut (),
    )
}

fn materialize_fixture(
    forest: &Forest<RuleId, Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
) -> Vec<
    super::materialize::MaterializedCandidate<
        BuildValue,
        Construction,
        Category,
        Lexical,
        Leaf,
        LexicalOwner,
    >,
> {
    materialize_with(
        forest,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| Ok(build(rule, children, context)),
    )
}

fn structural_markers(items: &[StructuralAtom]) -> Vec<StructuralWord> {
    items
        .iter()
        .map(|item| match item {
            StructuralAtom::StructuralAtom(value) => value.marker,
        })
        .collect()
}

#[test]
fn generated_invariant_rejection_survives_earley_materialization() {
    let environment = environment();
    let context = ParseContext::default();
    let forest = parse_fixture(Category::GuardedStructural, "Beta", &environment)
        .expect("the Earley forest accepts the construction surface before semantic checking");

    let (built, rejection) = super::materialize::materialize_with_rejection(
        &forest,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| build_checked(rule, children, &context),
    );

    assert!(built.is_empty());
    let rejection = rejection.expect("the generated rejection survives materialization");
    assert_eq!(rejection.owner(), "GuardedStructural");
    assert_eq!(rejection.role(), "guarded_structural");
    assert_eq!(
        *rejection.violation(),
        BuildViolation::Invariant {
            identity: "marker is Alpha",
        }
    );
    assert_eq!(
        rejection.to_string(),
        "GuardedStructural.guarded_structural: invariant `marker is Alpha` rejected"
    );
}

#[test]
fn successful_generated_build_dominates_a_rejected_alternative() {
    let environment = environment();
    let context = ParseContext::default();
    let rejected = parse_fixture(Category::GuardedStructural, "Beta", &environment)
        .expect("the rejected surface still has a syntactic forest");
    let accepted = parse_fixture(Category::GuardedStructural, "Alpha", &environment)
        .expect("the accepted surface has a syntactic forest");

    let rejected_root = rejected
        .accepted_root_ids()
        .next()
        .expect("one rejected root");
    let accepted_root = accepted
        .accepted_root_ids()
        .next()
        .expect("one accepted root");
    let rejected_node = rejected.node(rejected_root).clone();
    let mut accepted_node = accepted.node(accepted_root).clone();
    accepted_node.families.extend(rejected_node.families);
    let forest = Forest::from_test_parts(vec![accepted_node], vec![super::engine::NodeId(0)]);

    let (built, terminal_rejection) = super::materialize::materialize_with_rejection(
        &forest,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| build_checked(rule, children, &context),
    );
    assert_eq!(
        built.len(),
        1,
        "one successful family dominates the rejection"
    );
    assert!(terminal_rejection.is_none());
}

fn materialized_structural_markers(value: &BuildValue) -> Vec<StructuralWord> {
    match value {
        BuildValue::OptionalStructural(value) => value
            .maybe
            .as_ref()
            .into_iter()
            .flat_map(|item| structural_markers(std::slice::from_ref(item)))
            .collect(),
        BuildValue::ExactPairStructural(value) => structural_markers(&value.items),
        BuildValue::TerminatedStructural(value) => structural_markers(&value.items),
        BuildValue::SeparatedStructural(value) => structural_markers(&value.items),
        BuildValue::CombinedStructural(value) => structural_markers(&value.items),
        BuildValue::PositionalStructural(value) => structural_markers(&value.items),
        BuildValue::BoundedUniformStructural(value) => structural_markers(&value.items),
        BuildValue::BoundedPositionalStructural(value) => structural_markers(&value.items),
        value => panic!("unexpected structural materialization {value:?}"),
    }
}

#[test]
fn cross_root_nesting_materializes_from_base_children_without_root_sentinels() {
    let environment = environment();
    let context = ParseContext::default();
    let forest = parse_fixture(Category::OuterRoot, "Alpha", &environment)
        .expect("the ordinary nested categories parse without root punctuation");

    let built = materialize_fixture(&forest, &context);

    assert_eq!(built.len(), 1);
    let BuildValue::OuterRoot(OuterRoot::OuterRoot(value), _) = built[0].value.as_ref() else {
        panic!("the nested category materializes as the outer root value");
    };
    assert!(matches!(
        value.inner,
        NestedRoot::NestedRootLeaf(NestedRootValue {
            marker: StructuralWord::Alpha,
        })
    ));
}

#[test]
fn generated_structural_rows_parse_and_materialize_with_exact_bounds_and_order() {
    let environment = environment();
    let context = ParseContext::default();
    let cases = [
        (Category::OptionalStructural, "", Vec::new()),
        (
            Category::OptionalStructural,
            "Alpha",
            vec![StructuralWord::Alpha],
        ),
        (
            Category::ExactPairStructural,
            "Alpha<T><P>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::TerminatedStructural,
            "Alpha<T>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::SeparatedStructural,
            "Alpha<S>beta",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (Category::CombinedStructural, "", Vec::new()),
        (
            Category::CombinedStructural,
            "Alpha<T>",
            vec![StructuralWord::Alpha],
        ),
        (
            Category::CombinedStructural,
            "Alpha<T><S>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::CombinedStructural,
            "Alpha<T><S>beta<T><S>gamma<T><S>delta<T>",
            vec![
                StructuralWord::Alpha,
                StructuralWord::Beta,
                StructuralWord::Gamma,
                StructuralWord::Delta,
            ],
        ),
        (
            Category::PositionalStructural,
            "Alpha<T><P>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::PositionalStructural,
            "Alpha<T><F>beta<T><M>gamma<T><L>delta<T>",
            vec![
                StructuralWord::Alpha,
                StructuralWord::Beta,
                StructuralWord::Gamma,
                StructuralWord::Delta,
            ],
        ),
        (
            Category::BoundedUniformStructural,
            "Alpha<T><S>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::BoundedUniformStructural,
            "Alpha<T><S>beta<T><S>gamma<T><S>delta<T>",
            vec![
                StructuralWord::Alpha,
                StructuralWord::Beta,
                StructuralWord::Gamma,
                StructuralWord::Delta,
            ],
        ),
        (
            Category::BoundedPositionalStructural,
            "Alpha<T><P>beta<T>",
            vec![StructuralWord::Alpha, StructuralWord::Beta],
        ),
        (
            Category::BoundedPositionalStructural,
            "Alpha<T><F>beta<T><L>gamma<T>",
            vec![
                StructuralWord::Alpha,
                StructuralWord::Beta,
                StructuralWord::Gamma,
            ],
        ),
        (
            Category::BoundedPositionalStructural,
            "Alpha<T><F>beta<T><M>gamma<T><L>delta<T>",
            vec![
                StructuralWord::Alpha,
                StructuralWord::Beta,
                StructuralWord::Gamma,
                StructuralWord::Delta,
            ],
        ),
    ];

    for (category, text, expected) in cases {
        let forest = parse_fixture(category, text, &environment).unwrap_or_else(|failure| {
            panic!("{category:?} failed at {} on {text:?}", failure.offset)
        });
        let built = materialize_fixture(&forest, &context);
        assert_eq!(built.len(), 1, "one materialized value for {text:?}");
        let actual = materialized_structural_markers(&built[0].value);
        assert_eq!(
            actual, expected,
            "source order survives parse and fold for {text:?}"
        );
    }

    for (category, text) in [
        (Category::ExactPairStructural, "Alpha<T>"),
        (
            Category::ExactPairStructural,
            "Alpha<T><P>beta<T><P>gamma<T>",
        ),
        (Category::BoundedUniformStructural, "Alpha<T>"),
        (
            Category::BoundedUniformStructural,
            "Alpha<T><S>beta<T><S>gamma<T><S>delta<T><S>alpha<T>",
        ),
        (Category::BoundedUniformStructural, "Alpha<S><T>beta<T>"),
        (Category::BoundedPositionalStructural, "Alpha<T>"),
        (
            Category::BoundedPositionalStructural,
            "Alpha<T><F>beta<T><M>gamma<T><M>delta<T><L>alpha<T>",
        ),
        (
            Category::BoundedPositionalStructural,
            "Alpha<T><P>beta<T><P>gamma<T>",
        ),
        (Category::CombinedStructural, "Alpha<T><S>"),
        (
            Category::PositionalStructural,
            "Alpha<T><F>beta<T><L>gamma<T><M>delta<T>",
        ),
    ] {
        assert!(
            parse_fixture(category, text, &environment).is_err(),
            "{category:?} must reject invalid cardinality or surface order {text:?}",
        );
    }
}

fn structural_rule_label(rule: RuleId) -> String {
    super::diagnostic::rule_label_from_metadata(
        rule.public_construction()
            .map(|construction| format!("{construction:?}")),
        rule.owner(),
        rule.role(),
        rule.state(),
    )
}

#[test]
fn generated_helper_cycle_is_an_internal_materialization_failure_with_owner_role_path() {
    let environment = environment();
    let context = ParseContext::default();
    let parsed = parse_fixture(Category::OptionalStructural, "", &environment)
        .expect("the nullable generated helper produces an accepted Earley forest");
    assert!(parsed.accepted_root_ids().next().is_some());
    let helper = parsed
        .nodes()
        .find_map(|(id, node)| {
            (node.rule == RuleId::OptionalStructuralMaybeOptionalAbsent).then_some(id)
        })
        .expect("the accepted forest contains the generated absent-helper node");
    let roots = parsed.accepted_root_ids().collect::<Vec<_>>();
    let mut nodes = parsed
        .nodes()
        .map(|(_, node)| node.clone())
        .collect::<Vec<_>>();
    nodes[helper.0].families = vec![Family {
        children: vec![Child::Node(helper)],
    }];
    let cycle_only = Forest::from_test_parts(nodes, roots);

    let (built, trace) = super::materialize::materialize_with_trace(
        &cycle_only,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| Ok(build(rule, children, &context)),
        structural_rule_label,
        super::diagnostic::TraceLimits::new(16),
    );
    assert!(
        built.is_empty(),
        "the cycle-only accepted forest cannot materialize"
    );
    assert_eq!(trace.cycles().total(), 1);
    assert_eq!(
        trace.cycles().items()[0].construction_path().items(),
        [
            "OptionalStructural [public]",
            "OptionalStructural.maybe [optional_absent]",
        ],
    );

    let parsed = parse_fixture(
        Category::BoundedUniformStructural,
        "Alpha<T><S>beta<T>",
        &environment,
    )
    .expect("the finite counted helper produces an accepted Earley forest");
    let helper = parsed
        .nodes()
        .find_map(|(id, node)| {
            (node.rule == RuleId::BoundedUniformStructuralItemsSequenceCount2Final).then_some(id)
        })
        .expect("the accepted forest contains the generated counted-final helper node");
    let roots = parsed.accepted_root_ids().collect::<Vec<_>>();
    let mut nodes = parsed
        .nodes()
        .map(|(_, node)| node.clone())
        .collect::<Vec<_>>();
    nodes[helper.0].families = vec![Family {
        children: vec![Child::Node(helper)],
    }];
    let cycle_only = Forest::from_test_parts(nodes, roots);
    let (built, trace) = super::materialize::materialize_with_trace(
        &cycle_only,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| Ok(build(rule, children, &context)),
        structural_rule_label,
        super::diagnostic::TraceLimits::new(16),
    );
    assert!(
        built.is_empty(),
        "the counted cycle-only accepted forest cannot materialize"
    );
    assert_eq!(trace.cycles().total(), 1);
    assert_eq!(
        trace.cycles().items()[0].construction_path().items(),
        [
            "BoundedUniformStructural.items [sequence_non_empty]",
            "BoundedUniformStructural.items [sequence_count_1_continue]",
            "BoundedUniformStructural.items [sequence_count_2_final]",
        ],
    );

    let ordinary_failure = parse_fixture(Category::ExactPairStructural, "Alpha<T>", &environment)
        .expect_err("a short exact pair is an ordinary chart failure");
    assert_eq!(ordinary_failure.offset, "Alpha<T>".len());
}

#[test]
fn generated_homonyms_survive_scan_build_and_trace_with_category_safe_identity() {
    let environment = environment();
    let context = ParseContext::default();
    let text = "Homonym.";
    assert_eq!(
        REQUIRED_DECLARATIONS
            .iter()
            .map(|matcher| (matcher.kind, matcher.name, matcher.position))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            (
                DeclarationKind::KeywordAction,
                "Destroy",
                deckmaste_construction_core::macro_def::GrammarPosition::Verb,
            ),
            (
                DeclarationKind::KeywordAbility,
                "Destroy",
                deckmaste_construction_core::macro_def::GrammarPosition::Verb,
            ),
        ])
    );
    let mut trace = IdentityTrace::default();
    let adapter = Category::Homonym.root_adapter(<Homonym as GeneratedRoot>::EOI);
    let content_length = text.len()
        - adapter
            .iter()
            .map(|terminal| match terminal.matcher {
                Lexical::Literal(literal) => literal.len(),
                Lexical::EndOfInput => 0,
                _ => unreachable!("generated root adapter is punctuation plus optional EOI"),
            })
            .sum::<usize>();
    let forest = parse_observed_with_state(
        RULES,
        Category::Homonym,
        content_length,
        &ScanPosition {
            byte_offset: 0,
            case: CasePosition::DocumentInitial,
            prefix: PrefixPosition::None,
        },
        |terminal, offset, position, suppress_right_boundary| {
            debug_assert_eq!(offset, position.byte_offset);
            let terminal = if suppress_right_boundary {
                terminal.suppress_right_boundary()
            } else {
                terminal
            };
            scan(text, *position, terminal, &environment)
        },
        |_rule, _family, _forest| true,
        &mut trace,
    )
    .expect("both generated homonym rules parse");
    let mut root_position = ScanPosition {
        byte_offset: content_length,
        case: CasePosition::Continuation,
        prefix: PrefixPosition::WordOwnedSpace,
    };
    let root_values = adapter
        .iter()
        .map(|&terminal| {
            let scanned = scan(text, root_position, terminal, &environment)
                .into_iter()
                .next()
                .expect("the generated Homonym adapter consumes its declared suffix");
            root_position = scanned.state;
            scanned.lexical.value
        })
        .collect::<Vec<_>>();
    assert_eq!(root_position.byte_offset, text.len());
    assert_eq!(
        root_values,
        vec![Leaf::Literal("."), Leaf::EndOfInput],
        "the generated root adapter owns punctuation and EOI outside base productions",
    );

    assert_eq!(forest.accepted_roots().count(), 2);
    let traced_owners = trace.finish();
    assert_eq!(traced_owners.len(), 2, "trace dedup retains both owners");
    assert_eq!(
        traced_owners.into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "lexeme:keyword_ability/Destroy/bare".to_owned(),
            "lexeme:keyword_action/Destroy/bare".to_owned(),
        ])
    );

    let built = materialize_with(
        &forest,
        RULES,
        RuleId::index,
        RuleId::public_construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| Ok(build(rule, children, &context)),
    );
    assert_eq!(built.len(), 2, "the kernel retains both accepted roots");

    let mut visited = BTreeSet::new();
    for candidate in &built {
        let BuildValue::Homonym(homonym, ConcordClass::Other, _) = candidate.value.as_ref() else {
            panic!("open homonym materialization retains bare concord_class")
        };
        assert_eq!(homonym.render(&context, &environment), text);
        let (rendered, claims) = render_homonym_with_claims(homonym, &context, &environment);
        assert_eq!(rendered, text);
        assert_eq!(claims.len(), 2);
        assert_eq!((claims[0].start, claims[0].end), (0, 7));
        assert_eq!((claims[1].start, claims[1].end), (7, 8));
        assert_eq!(claims[0].owner.kind(), LexicalProvenanceKind::Lexeme);
        let mut visitor = IdentityVisitor::default();
        match homonym {
            Homonym::Action(value) => walk_action(&mut visitor, value),
            Homonym::Ability(value) => walk_ability(&mut visitor, value),
        }
        assert_eq!(visitor.0.len(), 1);
        let kind_key = match visitor.0[0].kind() {
            DeclarationKind::KeywordAction => "keyword_action",
            DeclarationKind::KeywordAbility => "keyword_ability",
            _ => unreachable!("fixture contains only the two verb homonyms"),
        };
        assert_eq!(
            claims[0].owner.stable_id(),
            format!("lexeme:{kind_key}/Destroy/bare")
        );
        visited.insert((visitor.0[0].kind(), visitor.0[0].name().to_owned()));
    }
    assert_eq!(
        visited,
        BTreeSet::from([
            (DeclarationKind::KeywordAction, "Destroy".to_owned()),
            (DeclarationKind::KeywordAbility, "Destroy".to_owned()),
        ])
    );

    assert_ne!(
        built[0].value, built[1].value,
        "materialization dedup must retain both category-safe identities"
    );
}
