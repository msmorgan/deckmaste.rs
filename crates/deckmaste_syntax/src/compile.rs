//! Binding a [`Grammar`] to a lexical environment: validation, name
//! resolution, and compilation of Schemas into Productions.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::SourceKind;

use crate::LoadError;
use crate::frames::Frame;
use crate::frames::FrameCatalog;
use crate::frames::FrameFamily;
use crate::frames::PrepositionFrame;
use crate::frames::SlotFill;
use crate::frames::SlotKind;
use crate::frames::SlotRelation;
use crate::grammar::Equation;
use crate::grammar::Grammar;
use crate::grammar::HostKind;
use crate::grammar::SchemaDecl;
use crate::grammar::Side;
use crate::grammar::SymbolDecl;
use crate::step::Gap;
use crate::step::GapRule;
use crate::step::Production;
use crate::step::Symbol;
use crate::step::Test;

/// Feature names that would police attachment rather than describe grammar.
const FORBIDDEN_SUFFIXES: [&str; 5] = ["Use", "Licence", "License", "Present", "Restriction"];

/// Lexical features the engine projects onto leaves, by grammar feature name.
pub(crate) const PROJECTED: [&str; 5] = ["Number", "Person", "Tense", "VerbForm", "Case"];

/// The lexical facts a bound grammar keeps about one lexeme.
#[derive(Debug, Clone)]
pub(crate) struct LexemeFacts {
    pub category: LexicalCategory,
    pub binding: Binding,
    pub frames: BTreeSet<usize>,
    pub preposition_frames: BTreeSet<usize>,
}

/// A grammar bound to a lexical environment, ready to parse, admit and realize.
#[derive(Debug)]
pub struct CompiledGrammar {
    pub(crate) categories: Vec<String>,
    pub(crate) category_features: Vec<Vec<usize>>,
    pub(crate) category_gap: Vec<bool>,
    pub(crate) features: Vec<String>,
    pub(crate) values: Vec<Vec<String>>,
    pub(crate) productions: Vec<Production>,
    pub(crate) by_id: BTreeMap<String, usize>,
    pub(crate) by_lhs: Vec<Vec<usize>>,
    pub(crate) root: usize,
    pub(crate) lexemes: BTreeMap<LexemeId, LexemeFacts>,
    pub(crate) frames: Vec<Frame>,
    pub(crate) preposition_frames: Vec<PrepositionFrame>,
    /// Prepositions whose `PP` slot would extend a head's selected frame,
    /// by (lexeme, selected frame).
    pub(crate) extensions: BTreeMap<(LexemeId, usize), Vec<LexemeId>>,
    pub(crate) classes: Vec<BTreeSet<LexemeId>>,
    /// Projected feature ids: Number, Person, Tense, `VerbForm`, Case.
    pub(crate) projected: [Option<usize>; 5],
    report: LoadReport,
}

/// Findings that do not prevent binding.
#[derive(Debug, Clone, Default)]
pub struct LoadReport {
    pub warnings: Vec<String>,
}

impl CompiledGrammar {
    /// Binds `grammar` to the lexicon and frame catalog.
    ///
    /// # Errors
    /// Any unknown name, ill-typed equation, forbidden feature name, grammar
    /// reference to a non-core or open-class lexeme, or frame that cannot be
    /// compiled.
    pub fn bind(
        grammar: &Grammar,
        lexicon: &Lexicon,
        catalog: &FrameCatalog,
    ) -> Result<Self, LoadError> {
        Binder::new(grammar, lexicon, catalog)?.finish()
    }

    #[must_use]
    pub fn report(&self) -> &LoadReport {
        &self.report
    }

    /// Stable ids of every Production, authored and Schema-derived.
    pub fn production_ids(&self) -> impl Iterator<Item = &str> {
        self.productions.iter().map(|p| p.id.as_str())
    }

    pub(crate) fn category_id(&self, name: &str) -> Option<usize> {
        self.categories.iter().position(|c| c == name)
    }
}

fn invalid(message: impl Into<String>) -> LoadError {
    LoadError::Invalid(message.into())
}

struct Binder<'a> {
    grammar: &'a Grammar,
    catalog: &'a FrameCatalog,
    out: CompiledGrammar,
    closed_words: BTreeMap<String, LexemeId>,
    class_names: BTreeMap<String, usize>,
}

/// One child of a Production under construction.
struct Child {
    name: String,
    symbol: Symbol,
    gap: GapRule,
}

/// A Production under construction, before equations are resolved.
struct Draft {
    id: String,
    lhs: usize,
    children: Vec<Child>,
    equations: Vec<(Path, Target)>,
    cost: u32,
    introduces: Gap,
    frame: Option<usize>,
    host_check: Option<(usize, usize, HostKind)>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Path {
    Mother(usize),
    Child(usize, usize),
}

enum Target {
    Path(Path),
    Value(u16),
}

impl Draft {
    fn new(id: String, lhs: usize, cost: u32) -> Self {
        Self {
            id,
            lhs,
            children: Vec::new(),
            equations: Vec::new(),
            cost,
            introduces: Gap::None,
            frame: None,
            host_check: None,
            coordinator: None,
            distinct_coordinator: Vec::new(),
            requires_coordination: None,
        }
    }

    fn child(&mut self, name: &str, symbol: Symbol, gap: GapRule) -> usize {
        self.children.push(Child {
            name: name.to_owned(),
            symbol,
            gap,
        });
        self.children.len() - 1
    }
}

impl<'a> Binder<'a> {
    #[expect(clippy::too_many_lines, reason = "validation in declaration order")]
    fn new(
        grammar: &'a Grammar,
        lexicon: &'a Lexicon,
        catalog: &'a FrameCatalog,
    ) -> Result<Self, LoadError> {
        let mut report = LoadReport::default();
        let features: Vec<String> = grammar.features.keys().cloned().collect();
        for name in &features {
            if FORBIDDEN_SUFFIXES.iter().any(|s| name.ends_with(s)) {
                return Err(invalid(format!(
                    "feature `{name}` is a policing flag, which the grammar may not declare"
                )));
            }
        }
        let values: Vec<Vec<String>> = grammar.features.values().cloned().collect();
        let feature_id = |name: &str| {
            features
                .iter()
                .position(|f| f == name)
                .ok_or_else(|| invalid(format!("unknown feature `{name}`")))
        };
        // Each coordinated Category gets an engine-owned serial tail Category.
        let mut declared = grammar.categories.clone();
        for schema in &grammar.schemas {
            if let SchemaDecl::Coordination { category, .. } = schema {
                let declaration = declared
                    .get(category)
                    .cloned()
                    .ok_or_else(|| invalid(format!("unknown category `{category}`")))?;
                declared.insert(format!("{category}#Serial"), declaration);
            }
        }
        let categories: Vec<String> = declared.keys().cloned().collect();
        let mut category_features = Vec::new();
        for (name, declaration) in &declared {
            let ids = declaration
                .features
                .iter()
                .map(|f| feature_id(f))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| invalid(format!("category `{name}`: {e}")))?;
            category_features.push(ids);
        }
        let category_gap = declared.values().map(|c| c.gap).collect();
        let root = categories
            .iter()
            .position(|c| *c == grammar.root)
            .ok_or_else(|| invalid(format!("unknown root category `{}`", grammar.root)))?;
        let projected = PROJECTED.map(|name| features.iter().position(|f| f == name));
        for (slot, name) in PROJECTED.iter().zip(projected) {
            let Some(id) = name else { continue };
            for value in crate::step::projected_values(slot) {
                if !values[id].iter().any(|v| v == value) {
                    return Err(invalid(format!(
                        "feature `{slot}` must include lexical value `{value}`"
                    )));
                }
            }
        }

        // Frames, deduplicated into global ids.
        let mut frames: Vec<Frame> = Vec::new();
        let mut preposition_frames: Vec<PrepositionFrame> = Vec::new();
        let mut lexemes = BTreeMap::new();
        for (id, lexeme) in lexicon.lexemes() {
            lexemes.insert(
                *id,
                LexemeFacts {
                    category: lexeme.category,
                    binding: lexeme.binding,
                    frames: BTreeSet::new(),
                    preposition_frames: BTreeSet::new(),
                },
            );
        }
        for (id, list) in &catalog.frames {
            let facts = lexemes
                .get_mut(id)
                .ok_or_else(|| invalid(format!("frame catalog names unknown lexeme `{id}`")))?;
            if facts.category == LexicalCategory::Preposition {
                return Err(invalid(format!(
                    "preposition `{id}` declares its frames under `prepositions`"
                )));
            }
            for frame in list {
                if !frame.is_well_formed() {
                    return Err(invalid(format!("frame {frame:?} of `{id}` is ill-formed")));
                }
                let index = intern(&mut frames, frame);
                facts.frames.insert(index);
            }
        }
        for (id, list) in &catalog.prepositions {
            let facts = lexemes
                .get_mut(id)
                .ok_or_else(|| invalid(format!("frame catalog names unknown lexeme `{id}`")))?;
            if facts.category != LexicalCategory::Preposition {
                return Err(invalid(format!("`{id}` is not a preposition")));
            }
            for frame in list {
                let index = intern(&mut preposition_frames, frame);
                facts.preposition_frames.insert(index);
            }
        }
        let mut referenced = Vec::new();
        for frame in frames
            .iter()
            .map(|f| f.slots.iter().map(|s| &s.fill).collect::<Vec<_>>())
            .chain(preposition_frames.iter().map(|f| vec![&f.complement]))
        {
            for fill in frame {
                if let SlotFill::PP(list) = fill {
                    referenced.extend(list.iter().copied());
                }
            }
        }
        for id in referenced {
            if lexemes
                .get(&id)
                .is_none_or(|facts| facts.preposition_frames.is_empty())
            {
                return Err(invalid(format!(
                    "a PP slot names `{id}`, which is not a preposition with frames"
                )));
            }
        }
        let mut extensions = BTreeMap::new();
        for (id, facts) in &lexemes {
            for &selected in &facts.frames {
                let mut preps = Vec::new();
                for &other in &facts.frames {
                    let (base, longer) = (&frames[selected], &frames[other]);
                    if longer.slots.len() == base.slots.len() + 1
                        && longer.slots[..base.slots.len()] == base.slots[..]
                        && let Some(last) = longer.slots.last()
                        && let SlotFill::PP(list) = &last.fill
                    {
                        preps.extend(list.iter().copied());
                    }
                }
                if !preps.is_empty() {
                    preps.sort();
                    preps.dedup();
                    extensions.insert((*id, selected), preps);
                }
            }
        }

        let mut closed_words = BTreeMap::new();
        for (name, id) in &grammar.closed_words {
            let id = LexemeId::new(id);
            check_closed(lexicon, &id, &format!("closed word `{name}`"))?;
            closed_words.insert(name.clone(), id);
        }
        let mut classes = Vec::new();
        let mut class_names = BTreeMap::new();
        for (name, members) in &grammar.lexical_classes {
            let mut set = BTreeSet::new();
            for id in members {
                let id = LexemeId::new(id);
                check_closed(lexicon, &id, &format!("lexical class `{name}`"))?;
                set.insert(id);
            }
            if set.len() < 2 {
                report.warnings.push(format!(
                    "lexical class `{name}` has {} member(s); a class needs two or more",
                    set.len()
                ));
            }
            class_names.insert(name.clone(), classes.len());
            classes.push(set);
        }

        for name in &grammar.typed_classes {
            if class_names.contains_key(name) {
                return Err(invalid(format!("duplicate lexical class `{name}`")));
            }
            class_names.insert(name.clone(), classes.len());
            let members: BTreeSet<_> = lexicon
                .lexemes()
                .iter()
                .filter(|(_, l)| l.properties.classes.contains(name))
                .map(|(id, _)| *id)
                .collect();
            if members.len() < 2 {
                report.warnings.push(format!(
                    "lexical class `{name}` has {} member(s); a class needs two or more",
                    members.len()
                ));
            }
            classes.push(members);
        }
        let by_lhs = vec![Vec::new(); categories.len()];
        Ok(Self {
            grammar,
            catalog,
            out: CompiledGrammar {
                categories,
                category_features,
                category_gap,
                features,
                values,
                productions: Vec::new(),
                by_id: BTreeMap::new(),
                by_lhs,
                root,
                lexemes,
                frames,
                preposition_frames,
                extensions,
                classes,
                projected,
                report,
            },
            closed_words,
            class_names,
        })
    }

    fn category(&self, name: &str) -> Result<usize, LoadError> {
        self.out
            .category_id(name)
            .ok_or_else(|| invalid(format!("unknown category `{name}`")))
    }

    fn feature(&self, name: &str) -> Result<usize, LoadError> {
        self.out
            .features
            .iter()
            .position(|f| f == name)
            .ok_or_else(|| invalid(format!("unknown feature `{name}`")))
    }

    fn value(&self, feature: usize, name: &str) -> Result<u16, LoadError> {
        let index = self.out.values[feature]
            .iter()
            .position(|v| v == name)
            .ok_or_else(|| {
                invalid(format!(
                    "unknown value `{name}` of feature `{}`",
                    self.out.features[feature]
                ))
            })?;
        Ok(u16::try_from(index).expect("feature domains are small"))
    }

    fn word(&self, name: &str) -> Result<LexemeId, LoadError> {
        self.closed_words
            .get(name)
            .copied()
            .ok_or_else(|| invalid(format!("unknown closed word `{name}`")))
    }

    fn coordinator_test(&self, name: &str) -> Result<Test, LoadError> {
        if let Some(id) = self.closed_words.get(name) {
            return Ok(Test::Word(*id));
        }
        self.class_names
            .get(name)
            .map(|&c| Test::Class(c))
            .ok_or_else(|| invalid(format!("unknown closed word or class `{name}`")))
    }

    fn slot_category(&self, kind: SlotKind) -> Result<usize, LoadError> {
        let name = self
            .grammar
            .slot_categories
            .get(&kind)
            .ok_or_else(|| invalid(format!("no slot category declared for {kind:?}")))?;
        self.category(name)
    }

    fn finish(mut self) -> Result<CompiledGrammar, LoadError> {
        for declaration in &self.grammar.productions {
            let draft = self.authored(declaration)?;
            self.add(draft)?;
        }
        for schema in &self.grammar.schemas {
            for draft in self.schema(schema)? {
                self.add(draft)?;
            }
        }
        Ok(self.out)
    }

    fn authored(&self, declaration: &crate::grammar::ProductionDecl) -> Result<Draft, LoadError> {
        let context = |e: LoadError| invalid(format!("production `{}`: {e}", declaration.name));
        if declaration.rhs.is_empty() {
            return Err(context(invalid("empty right-hand side")));
        }
        let lhs = self.category(&declaration.lhs).map_err(context)?;
        let mut draft = Draft::new(declaration.name.clone(), lhs, declaration.cost);
        let mut required_gaps = BTreeMap::new();
        for equation in &declaration.equations {
            if let Equation::Require(path, value) = equation
                && let Some(child) = path.strip_suffix(".Gap")
            {
                let gap = match value.as_str() {
                    "None" => Gap::None,
                    "NP" => Gap::NP,
                    other => return Err(context(invalid(format!("unknown gap `{other}`")))),
                };
                required_gaps.insert(child.to_owned(), gap);
            }
        }
        for symbol in &declaration.rhs {
            let (name, symbol) = match symbol {
                SymbolDecl::Cat(name, category) => (
                    name.as_str(),
                    Symbol::Cat {
                        binding: None,
                        category: self.category(category).map_err(context)?,
                        prepositions: None,
                        host: None,
                    },
                ),
                SymbolDecl::Attached(name, category, binding) => (
                    name.as_str(),
                    Symbol::Cat {
                        category: self.category(category).map_err(context)?,
                        binding: Some(*binding),
                        prepositions: None,
                        host: None,
                    },
                ),
                SymbolDecl::UnsignedNumeral(name, notation) => {
                    (name.as_str(), Symbol::Leaf(Test::Numeral(*notation, true)))
                }
                SymbolDecl::Numeral(name, notation) => {
                    (name.as_str(), Symbol::Leaf(Test::Numeral(*notation, false)))
                }
                SymbolDecl::FreeLex(name, category) => {
                    (name.as_str(), Symbol::Leaf(Test::FreeLexical(*category)))
                }
                SymbolDecl::Lex(name, category) => {
                    (name.as_str(), Symbol::Leaf(Test::Lexical(*category)))
                }
                SymbolDecl::Word(name, word) => (
                    name.as_str(),
                    Symbol::Leaf(Test::Word(self.word(word).map_err(context)?)),
                ),
                SymbolDecl::Class(name, class) => (
                    name.as_str(),
                    Symbol::Leaf(Test::Class(*self.class_names.get(class).ok_or_else(
                        || context(invalid(format!("unknown class `{class}`"))),
                    )?)),
                ),
                SymbolDecl::Mark(c, binding) => ("", Symbol::Punct(*c, *binding)),
                SymbolDecl::Punct(c) => ("", Symbol::Punct(*c, Binding::Suffix)),
            };
            let gap = if let Some(gap) = required_gaps.remove(name) {
                GapRule::Require(gap)
            } else if declaration.transparent.iter().any(|t| t == name) {
                GapRule::Transparent
            } else {
                GapRule::Closed
            };
            draft.child(name, symbol, gap);
        }
        if let Some(name) = required_gaps.keys().next() {
            return Err(context(invalid(format!(
                "gap requirement on unknown child `{name}`"
            ))));
        }
        for name in &declaration.transparent {
            if !draft.children.iter().any(|c| c.name == *name) {
                return Err(context(invalid(format!(
                    "unknown transparent child `{name}`"
                ))));
            }
        }
        for equation in &declaration.equations {
            match equation {
                Equation::Agree(a, b) => {
                    let a = self.path(&draft, a).map_err(context)?;
                    let b = self.path(&draft, b).map_err(context)?;
                    draft.equations.push((a, Target::Path(b)));
                }
                Equation::Require(path, _) if path.ends_with(".Gap") => {}
                Equation::Require(path, value) => {
                    let path = self.path(&draft, path).map_err(context)?;
                    let feature = match path {
                        Path::Mother(f) | Path::Child(_, f) => f,
                    };
                    let value = self.value(feature, value).map_err(context)?;
                    draft.equations.push((path, Target::Value(value)));
                }
            }
        }
        Ok(draft)
    }

    fn path(&self, draft: &Draft, text: &str) -> Result<Path, LoadError> {
        let (owner, feature) = text
            .split_once('.')
            .ok_or_else(|| invalid(format!("path `{text}` is not `owner.Feature`")))?;
        let feature = self.feature(feature)?;
        if owner == "$" {
            if !self.out.category_features[draft.lhs].contains(&feature) {
                return Err(invalid(format!("mother does not carry `{text}`")));
            }
            return Ok(Path::Mother(feature));
        }
        let index = draft
            .children
            .iter()
            .position(|c| c.name == owner)
            .ok_or_else(|| invalid(format!("unknown child `{owner}`")))?;
        if !self.carries(&draft.children[index].symbol, feature) {
            return Err(invalid(format!("child `{owner}` does not carry `{text}`")));
        }
        Ok(Path::Child(index, feature))
    }

    fn carries(&self, symbol: &Symbol, feature: usize) -> bool {
        match symbol {
            Symbol::Cat { category, .. } => {
                self.out.category_features[*category].contains(&feature)
            }
            Symbol::Leaf(_) => self.out.projected.contains(&Some(feature)),
            Symbol::Punct(..) => false,
        }
    }

    /// Resolves a draft's equations into variables and adds it.
    fn add(&mut self, mut draft: Draft) -> Result<(), LoadError> {
        if let Some(head) = draft.children.iter().position(|c| c.name == "head") {
            for &feature in &self.out.category_features[draft.lhs] {
                if self.carries(&draft.children[head].symbol, feature) {
                    draft.equations.push((
                        Path::Mother(feature),
                        Target::Path(Path::Child(head, feature)),
                    ));
                }
            }
        }
        let production = crate::step::resolve(
            &self.out,
            draft.id.clone(),
            draft.lhs,
            draft
                .children
                .into_iter()
                .map(|c| (c.name, c.symbol, c.gap))
                .collect(),
            draft
                .equations
                .into_iter()
                .map(|(path, target)| {
                    let key = |p: Path| match p {
                        Path::Mother(f) => (None, f),
                        Path::Child(i, f) => (Some(i), f),
                    };
                    (
                        key(path),
                        match target {
                            Target::Path(p) => Ok(key(p)),
                            Target::Value(v) => Err(v),
                        },
                    )
                })
                .collect(),
            draft.cost,
            draft.introduces,
            draft.frame,
            draft.host_check,
            draft.coordinator,
            draft.distinct_coordinator,
            draft.requires_coordination,
        )
        .map_err(|e| invalid(format!("production `{}`: {e}", draft.id)))?;
        if self.out.by_id.contains_key(&production.id) {
            return Err(invalid(format!(
                "duplicate production id `{}`",
                production.id
            )));
        }
        let index = self.out.productions.len();
        self.out.by_id.insert(production.id.clone(), index);
        self.out.by_lhs[production.lhs].push(index);
        self.out.productions.push(production);
        Ok(())
    }

    fn schema(&self, schema: &SchemaDecl) -> Result<Vec<Draft>, LoadError> {
        match schema {
            SchemaDecl::Parameters {
                head,
                phrase,
                binding,
            } => self.parameters(*head, phrase, *binding),
            SchemaDecl::Valence {
                head,
                phrase,
                cost,
                passive,
            } => self.valence(*head, phrase, *cost, passive.as_deref()),
            SchemaDecl::Adjunct {
                host,
                adjunct,
                side,
                host_kind,
                cost,
            } => {
                let host_id = self.category(host)?;
                let adjunct_id = self.category(adjunct)?;
                let mut draft = Draft::new(
                    format!("Adjunct:{host}+{adjunct}/{side:?}/{host_kind:?}"),
                    host_id,
                    *cost,
                );
                let host_symbol = Symbol::Cat {
                    binding: None,
                    category: host_id,
                    prepositions: None,
                    host: None,
                };
                let adjunct_symbol = Symbol::Cat {
                    binding: None,
                    category: adjunct_id,
                    prepositions: None,
                    host: Some(*host_kind),
                };
                // CGEL: an unselected dependent of a nominal is a Modifier, of
                // a predicate an Adjunct.
                let function = match host_kind {
                    HostKind::Nominal => "modifier",
                    HostKind::Predicate => "adjunct",
                };
                let (h, a) = if *side == Side::After {
                    let h = draft.child("head", host_symbol, GapRule::Transparent);
                    (h, draft.child(function, adjunct_symbol, GapRule::Closed))
                } else {
                    let a = draft.child(function, adjunct_symbol, GapRule::Closed);
                    (draft.child("head", host_symbol, GapRule::Transparent), a)
                };
                draft.host_check = Some((h, a, *host_kind));
                Ok(vec![draft])
            }
            SchemaDecl::Coordination {
                category,
                coordinators,
                cost,
            } => self.coordination(category, coordinators, *cost),
        }
    }

    fn parameters(
        &self,
        head: LexicalCategory,
        phrase: &str,
        binding: Option<Binding>,
    ) -> Result<Vec<Draft>, LoadError> {
        use crate::frames::ParameterPart;
        let lhs = self.category(phrase)?;
        let mut drafts = Vec::new();
        for (id, patterns) in &self.catalog.parameters {
            let facts =
                self.out.lexemes.get(id).ok_or_else(|| {
                    invalid(format!("parameter pattern names unknown lexeme `{id}`"))
                })?;
            if facts.category != head || binding.is_some_and(|b| b != facts.binding) {
                continue;
            }
            for (index, parts) in patterns.iter().enumerate() {
                if parts
                    .iter()
                    .filter(|p| matches!(p, ParameterPart::Head))
                    .count()
                    != 1
                {
                    return Err(invalid(format!(
                        "parameter pattern of `{id}` needs one head"
                    )));
                }
                let mut draft = Draft::new(format!("Parameters:{phrase}/{id}/{index}"), lhs, 0);
                for part in parts {
                    let (name, symbol) = match part {
                        ParameterPart::Head => ("head", Symbol::Leaf(Test::Word(*id))),
                        ParameterPart::Slot(kind) => (
                            "complement",
                            Symbol::Cat {
                                binding: None,
                                category: self.slot_category(*kind)?,
                                prepositions: None,
                                host: None,
                            },
                        ),
                        ParameterPart::Word(word) => {
                            if !self.out.lexemes.contains_key(word) {
                                return Err(invalid(format!("unknown parameter marker `{word}`")));
                            }
                            ("marker", Symbol::Leaf(Test::Word(*word)))
                        }
                        ParameterPart::Words(words) => {
                            if words.is_empty()
                                || words
                                    .iter()
                                    .any(|word| !self.out.lexemes.contains_key(word))
                            {
                                return Err(invalid(format!(
                                    "invalid parameter word class of `{id}`"
                                )));
                            }
                            (
                                "complement",
                                Symbol::Leaf(Test::Words(words.iter().copied().collect())),
                            )
                        }
                        ParameterPart::Mark(c, binding) => ("", Symbol::Punct(*c, *binding)),
                    };
                    draft.child(name, symbol, GapRule::Closed);
                }
                drafts.push(draft);
            }
        }
        Ok(drafts)
    }

    fn valence(
        &self,
        head: LexicalCategory,
        phrase: &str,
        cost: u32,
        passive: Option<&str>,
    ) -> Result<Vec<Draft>, LoadError> {
        let lhs = self.category(phrase)?;
        let gap_capable = self.out.category_gap[lhs];
        let mut drafts = Vec::new();
        if head == LexicalCategory::Preposition {
            let used: BTreeSet<usize> = self
                .out
                .lexemes
                .values()
                .flat_map(|f| f.preposition_frames.iter().copied())
                .collect();
            for index in used {
                let frame = &self.out.preposition_frames[index];
                let mut draft = Draft::new(
                    format!("Valence:{phrase}/{:?}@{:?}", frame.complement, frame.hosts),
                    lhs,
                    cost,
                );
                draft.child(
                    "head",
                    Symbol::Leaf(Test::PrepositionFrame(index)),
                    GapRule::Closed,
                );
                if frame.complement != SlotFill::Intransitive {
                    draft.child(
                        "complement",
                        self.slot_symbol(&frame.complement)?,
                        GapRule::Transparent,
                    );
                }
                drafts.push(draft);
                if gap_capable && frame.complement == SlotFill::NP {
                    let mut stranded = Draft::new(
                        format!(
                            "Valence:{phrase}/{:?}@{:?}/gap0",
                            frame.complement, frame.hosts
                        ),
                        lhs,
                        cost,
                    );
                    stranded.child(
                        "head",
                        Symbol::Leaf(Test::PrepositionFrame(index)),
                        GapRule::Closed,
                    );
                    stranded.introduces = Gap::NP;
                    drafts.push(stranded);
                }
            }
            return Ok(drafts);
        }
        let mut used = BTreeSet::new();
        for (id, facts) in &self.out.lexemes {
            if facts.category != head {
                continue;
            }
            if facts.frames.is_empty()
                && head == LexicalCategory::Verb
                && !self.closed_words.values().any(|w| w == id)
            {
                return Err(invalid(format!(
                    "verb `{id}` has no Subcategorization Frame"
                )));
            }
            used.extend(facts.frames.iter().copied());
        }
        if head != LexicalCategory::Verb {
            let mut bare = Draft::new(format!("Valence:{phrase}/Bare"), lhs, cost);
            bare.child(
                "head",
                Symbol::Leaf(Test::FreeLexical(head)),
                GapRule::Closed,
            );
            drafts.push(bare);
        }
        let voice = self.out.features.iter().position(|f| f == "Voice");
        let voice_value = |name: &str| -> Result<Option<(usize, u16)>, LoadError> {
            match voice {
                Some(v) if self.out.category_features[lhs].contains(&v) => {
                    Ok(Some((v, self.value(v, name)?)))
                }
                _ => Ok(None),
            }
        };
        for index in used {
            let frame = &self.out.frames[index];
            let name = frame_name(phrase, frame);
            let slots: Vec<&SlotFill> = frame.slots.iter().map(|s| &s.fill).collect();
            let gappable = gap_capable && head == LexicalCategory::Verb;
            // Active, with every NP slot optionally extracted.
            let active = voice_value("Active")?;
            drafts.extend(
                self.valence_family(lhs, &name, index, cost, &slots, 0, None, active, gappable)?,
            );
            if let (Some(by), true) = (passive, head == LexicalCategory::Verb)
                && matches!(slots.first(), Some(SlotFill::NP))
            {
                let passive_voice = voice_value("Passive")?.ok_or_else(|| {
                    invalid(format!(
                        "passive schema on `{phrase}` needs a Voice feature"
                    ))
                })?;
                let form = self.feature("VerbForm")?;
                let participle = (form, self.value(form, "PastParticiple")?);
                let by = self.word(by)?;
                for agent in [false, true] {
                    let mut rest: Vec<&SlotFill> = slots[1..].to_vec();
                    let by_np = SlotFill::NP;
                    if agent {
                        rest.push(&by_np);
                    }
                    let tag = if agent { "passive+by" } else { "passive" };
                    for mut draft in self.valence_family(
                        lhs,
                        &format!("{name}/{tag}"),
                        index,
                        cost,
                        &rest,
                        1,
                        agent.then_some(by),
                        Some(passive_voice),
                        gappable,
                    )? {
                        draft
                            .equations
                            .push((Path::Mother(participle.0), Target::Value(participle.1)));
                        drafts.push(draft);
                    }
                }
            }
        }
        Ok(drafts)
    }

    /// One frame's phrase and, when `gappable`, a variant per extracted NP
    /// slot. `slots` are the frame's slots from `first_slot` on (a passive
    /// drops the Object), plus a trailing agent NP when `by` is given.
    #[expect(clippy::too_many_arguments, reason = "one internal call shape")]
    fn valence_family(
        &self,
        lhs: usize,
        name: &str,
        frame: usize,
        cost: u32,
        slots: &[&SlotFill],
        first_slot: usize,
        by: Option<LexemeId>,
        voice: Option<(usize, u16)>,
        gappable: bool,
    ) -> Result<Vec<Draft>, LoadError> {
        let mut drafts = Vec::new();
        let extractable: Vec<Option<usize>> = std::iter::once(None)
            .chain(
                slots
                    .iter()
                    .enumerate()
                    .filter(|(_, fill)| gappable && ***fill == SlotFill::NP)
                    .map(|(i, _)| Some(i)),
            )
            .collect();
        let agent_slot = by.as_ref().map(|_| slots.len() - 1);
        for gap in extractable {
            if gap.is_some() && gap == agent_slot {
                continue;
            }
            let id = match gap {
                None => name.to_owned(),
                Some(i) => format!("{name}/gap{i}"),
            };
            let mut draft = Draft::new(id, lhs, cost);
            draft.frame = Some(frame);
            draft.child("head", Symbol::Leaf(Test::Frame(frame)), GapRule::Closed);
            for (i, fill) in slots.iter().enumerate() {
                if Some(i) == gap {
                    continue;
                }
                if Some(i) == agent_slot
                    && let Some(by) = &by
                {
                    draft.child("by", Symbol::Leaf(Test::Word(*by)), GapRule::Closed);
                    draft.child("agent", self.slot_symbol(fill)?, GapRule::Closed);
                    continue;
                }
                let relation = self.out.frames[frame].slots[first_slot + i].relation;
                draft.child(
                    relation_name(relation),
                    self.slot_symbol(fill)?,
                    GapRule::Transparent,
                );
            }
            if gap.is_some() {
                draft.introduces = Gap::NP;
            }
            if let Some((feature, value)) = voice {
                draft
                    .equations
                    .push((Path::Mother(feature), Target::Value(value)));
            }
            drafts.push(draft);
        }
        Ok(drafts)
    }

    fn slot_symbol(&self, fill: &SlotFill) -> Result<Symbol, LoadError> {
        Ok(Symbol::Cat {
            binding: None,
            category: self.slot_category(fill.kind())?,
            prepositions: match fill {
                SlotFill::PP(list) => Some(list.clone()),
                _ => None,
            },
            host: None,
        })
    }

    #[expect(clippy::many_single_char_names, reason = "conjunct positions")]
    fn coordination(
        &self,
        category: &str,
        coordinators: &str,
        cost: u32,
    ) -> Result<Vec<Draft>, LoadError> {
        let lhs = self.category(category)?;
        let coordinator = self.coordinator_test(coordinators)?;
        let conjunct = || Symbol::Cat {
            binding: None,
            category: lhs,
            prepositions: None,
            host: None,
        };
        let agree_all = |draft: &mut Draft, conjuncts: &[usize]| {
            for &feature in &self.out.category_features[lhs] {
                for &c in conjuncts {
                    draft
                        .equations
                        .push((Path::Mother(feature), Target::Path(Path::Child(c, feature))));
                }
            }
        };
        let mut drafts = Vec::new();
        // Binary: X Coord X.
        let mut binary = Draft::new(format!("Coordination:{category}/binary"), lhs, cost);
        let a = binary.child("first", conjunct(), GapRule::Shared);
        let c = binary.child(
            "coordinator",
            Symbol::Leaf(coordinator.clone()),
            GapRule::Closed,
        );
        let b = binary.child("last", conjunct(), GapRule::Shared);
        binary.coordinator = Some(c);
        binary.distinct_coordinator = vec![a, b];
        agree_all(&mut binary, &[a, b]);
        drafts.push(binary);
        // Serial with the Oxford comma: X , X , Coord X — one tree per chain.
        let tail_name = format!("{category}#Serial");
        let tail = self.out.category_id(&tail_name).ok_or_else(|| {
            invalid(format!(
                "coordination of `{category}` needs a category `{tail_name}`"
            ))
        })?;
        let tail_symbol = Symbol::Cat {
            binding: None,
            category: tail,
            prepositions: None,
            host: None,
        };
        let mut serial = Draft::new(format!("Coordination:{category}/serial"), lhs, cost);
        let a = serial.child("first", conjunct(), GapRule::Shared);
        serial.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let t = serial.child("rest", tail_symbol.clone(), GapRule::Shared);
        serial.coordinator = Some(t);
        serial.distinct_coordinator = vec![a];
        agree_all(&mut serial, &[a, t]);
        drafts.push(serial);
        let mut more = Draft::new(format!("Coordination:{category}/serial-more"), tail, 0);
        let a = more.child("first", conjunct(), GapRule::Shared);
        more.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let t = more.child("rest", tail_symbol, GapRule::Shared);
        more.coordinator = Some(t);
        more.distinct_coordinator = vec![a];
        agree_all(&mut more, &[a, t]);
        drafts.push(more);
        let mut end = Draft::new(format!("Coordination:{category}/serial-end"), tail, 0);
        let a = end.child("first", conjunct(), GapRule::Shared);
        end.child("", Symbol::Punct(',', Binding::Suffix), GapRule::Closed);
        let c = end.child("coordinator", Symbol::Leaf(coordinator), GapRule::Closed);
        let b = end.child("last", conjunct(), GapRule::Shared);
        end.coordinator = Some(c);
        end.distinct_coordinator = vec![a, b];
        agree_all(&mut end, &[a, b]);
        drafts.push(end);
        // Right Node Raising: a coordination with a shared NP gap, filled once.
        if self.out.category_gap[lhs] {
            let np = self.slot_category(SlotKind::NP)?;
            let mut raising = Draft::new(format!("Coordination:{category}/shared"), lhs, cost);
            let h = raising.child("head", conjunct(), GapRule::Require(Gap::NP));
            raising.child(
                "shared",
                Symbol::Cat {
                    binding: None,
                    category: np,
                    prepositions: None,
                    host: None,
                },
                GapRule::Closed,
            );
            raising.requires_coordination = Some(h);
            drafts.push(raising);
        }
        Ok(drafts)
    }
}

pub(crate) fn frame_name(phrase: &str, frame: &Frame) -> String {
    let slots: Vec<String> = frame
        .slots
        .iter()
        .map(|slot| match &slot.fill {
            SlotFill::PP(list) => format!(
                "PP[{}]",
                list.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            fill => format!("{:?}", fill.kind()),
        })
        .collect();
    let family = match frame.family {
        FrameFamily::Intransitive => "Intransitive".to_owned(),
        family => format!("{family:?}"),
    };
    format!("Valence:{phrase}/{family}({})", slots.join(","))
}

fn intern<T: PartialEq + Clone>(list: &mut Vec<T>, item: &T) -> usize {
    if let Some(index) = list.iter().position(|x| x == item) {
        return index;
    }
    list.push(item.clone());
    list.len() - 1
}

/// The CR-catalog ban: the grammar names only core closed-class lexemes.
fn check_closed(lexicon: &Lexicon, id: &LexemeId, context: &str) -> Result<(), LoadError> {
    let lexeme = lexicon
        .lexemes()
        .get(id)
        .ok_or_else(|| invalid(format!("{context}: unknown lexeme `{id}`")))?;
    if lexeme.source.kind != SourceKind::Core {
        return Err(invalid(format!(
            "{context}: `{id}` comes from {:?}, not the core closed class; catalog vocabulary never appears in the grammar",
            lexeme.source.kind
        )));
    }
    if !matches!(
        lexeme.category,
        LexicalCategory::Determinative
            | LexicalCategory::Pronoun
            | LexicalCategory::Preposition
            | LexicalCategory::Coordinator
            | LexicalCategory::Subordinator
            | LexicalCategory::Clitic
            | LexicalCategory::Verb
    ) {
        return Err(invalid(format!(
            "{context}: `{id}` is a {:?}, not a closed-class word",
            lexeme.category
        )));
    }
    Ok(())
}

/// The child name of a Frame Slot: the grammatical function it bears.
fn relation_name(relation: SlotRelation) -> &'static str {
    match relation {
        SlotRelation::Object => "object",
        SlotRelation::IndirectObject => "indirect_object",
        SlotRelation::Complement => "complement",
        SlotRelation::PredicativeComplement => "predicative_complement",
    }
}
