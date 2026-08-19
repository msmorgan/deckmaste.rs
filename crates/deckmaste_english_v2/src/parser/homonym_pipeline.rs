use std::collections::BTreeSet;

use RulePosition::Lexical as L;
use deckmaste_construction::constructions;
use macro_ron::v2::DeclarationIdentity;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::SurfaceFeature;

use super::diagnostic::SemanticTokenInventory;
use super::engine::LexicalMatch;
use super::engine::Observation;
use super::engine::Rule;
use super::engine::RulePosition;
use super::engine::parse_observed;
use super::materialize::materialize_with;
use crate::environment::ParserEnvironment;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BuildValue {
    Homonym(Homonym, Agreement),
    Leaf(Leaf),
}

#[derive(Debug, Default)]
struct ParseContext<'a>(std::marker::PhantomData<&'a ()>);

trait Render {
    fn render(&self, context: &ParseContext<'_>, environment: &ParserEnvironment) -> String;
}

struct Writer {
    output: String,
    capitalize_next: bool,
}

impl Writer {
    fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
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
    fn word_end(&self, running_text: &str) -> Option<usize> {
        let prefix = usize::from(self.position.case == CasePosition::Continuation);
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
            && self
                .text
                .get(end..)
                .and_then(|tail| tail.chars().next())
                .is_none_or(|character| !character.is_alphanumeric()))
        .then_some(end)
    }

    fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
        self.text
            .get(self.position.byte_offset..)?
            .starts_with(punctuation)
            .then_some(self.position.byte_offset + punctuation.len())
    }

    fn declaration_readings(
        &self,
        matcher: DeclarationMatcher,
    ) -> Vec<(usize, DeclarationIdentity, SurfaceFeature)> {
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
    }
}

constructions! {
    construction action: Homonym {
        element Action {}
        derive agreement = verb.agreement;
        derive verb.agreement = Values::Bare;
        form action = open_verb(KeywordAction, "Destroy");
    }
    construction ability: Homonym {
        element Ability {}
        derive agreement = verb.agreement;
        derive verb.agreement = Values::Bare;
        form ability = open_verb(KeywordAbility, "Destroy");
    }
    root Homonym { punctuation = "."; eoi = true; standalone_render = true; }
}

fn environment() -> ParserEnvironment {
    ParserEnvironment::try_from_declarations([
        macro_ron::v2::read_str(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"same",grammar:Verb(bare:"same",valence:Intransitive))"#,
        )
        .unwrap(),
        macro_ron::v2::read_str(
            "/synthetic/abilities/Destroy.ron",
            r#"KeywordAbility(name:"Destroy",spelling:"same",grammar:Verb(bare:"same",valence:Intransitive))"#,
        )
        .unwrap(),
    ])
    .unwrap()
}

#[derive(Default)]
struct IdentityTrace {
    tokens: SemanticTokenInventory<Lexical, (Leaf, Option<LexicalOwner>)>,
}

impl IdentityTrace {
    fn finish(self) -> Vec<String> {
        self.tokens
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

impl Observation<RuleId, Leaf, LexicalTerminal> for IdentityTrace {
    fn scanned(&mut self, start: usize, terminal: LexicalTerminal, end: usize, value: &Leaf) {
        if matches!(value, Leaf::Declaration(_)) {
            self.tokens.record_projected(
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
    offset: usize,
    terminal: LexicalTerminal,
    environment: &ParserEnvironment,
) -> Vec<LexicalMatch<Leaf>> {
    scan_lexical(
        &ScanInput {
            text,
            position: ScanPosition {
                byte_offset: offset,
                case: if offset == 0 {
                    CasePosition::DocumentInitial
                } else {
                    CasePosition::Continuation
                },
            },
            environment,
        },
        terminal,
    )
}

#[test]
fn generated_homonyms_survive_scan_build_and_trace_with_category_safe_identity() {
    let environment = environment();
    let context = ParseContext::default();
    let text = "Same.";
    assert_eq!(
        REQUIRED_DECLARATIONS
            .iter()
            .map(|matcher| (matcher.kind, matcher.name, matcher.position))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            (
                DeclarationKind::KeywordAction,
                "Destroy",
                macro_ron::v2::GrammarPosition::Verb,
            ),
            (
                DeclarationKind::KeywordAbility,
                "Destroy",
                macro_ron::v2::GrammarPosition::Verb,
            ),
        ])
    );
    let mut trace = IdentityTrace::default();
    let forest = parse_observed(
        RULES,
        Category::Homonym,
        text.len(),
        |terminal, offset| scan(text, offset, terminal, &environment),
        |_rule, _family, _forest| true,
        &mut trace,
    )
    .expect("both generated homonym rules parse");

    assert_eq!(forest.accepted_roots().count(), 2);
    let traced_owners = trace.finish();
    assert_eq!(traced_owners.len(), 2, "trace dedup retains both owners");
    assert_eq!(
        traced_owners.into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "declaration:keyword ability/Destroy".to_owned(),
            "declaration:keyword action/Destroy".to_owned(),
        ])
    );

    let built = materialize_with(
        &forest,
        RULES,
        RuleId::index,
        RuleId::construction,
        |terminal: LexicalTerminal| terminal.matcher,
        |leaf| BuildValue::Leaf(leaf.clone()),
        |rule, children| build(rule, children, &context),
    );
    assert_eq!(built.len(), 2, "the kernel retains both accepted roots");

    let mut visited = BTreeSet::new();
    for candidate in &built {
        let BuildValue::Homonym(homonym, Agreement::Bare) = &candidate.value else {
            panic!("open homonym materialization retains bare agreement")
        };
        assert_eq!(homonym.render(&context, &environment), text);
        let mut visitor = IdentityVisitor::default();
        match homonym {
            Homonym::Action(value) => walk_action(&mut visitor, value),
            Homonym::Ability(value) => walk_ability(&mut visitor, value),
        }
        assert_eq!(visitor.0.len(), 1);
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
