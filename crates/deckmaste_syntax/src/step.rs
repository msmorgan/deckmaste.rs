//! Productions as compiled, and the one step function shared by the chart
//! and by Admission: adding a child to a partial Production.

use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::WordForm;
use serde::Deserialize;
use serde::Serialize;

use crate::compile::CompiledGrammar;
use crate::grammar::HostKind;

/// The unfilled Gap a Constituent contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Gap {
    None,
    NP,
}

/// How a child's Gap relates to its mother's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GapRule {
    /// The child contains no Gap.
    Closed,
    /// The child's Gap, if any, is the mother's.
    Transparent,
    /// Every shared child has the same Gap, which is the mother's (across the board).
    Shared,
    /// The child has exactly this Gap, filled within the mother.
    Require(Gap),
}

/// A lexical leaf test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Test {
    Lexical(LexicalCategory),
    FreeLexical(LexicalCategory),
    Numeral(deckmaste_lexical::Numeral),
    Word(LexemeId),
    Class(usize),
    Words(std::collections::BTreeSet<LexemeId>),
    /// A lexeme that declares this Subcategorization Frame.
    Frame(usize),
    /// A preposition that declares this complement frame.
    PrepositionFrame(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Symbol {
    Cat {
        category: usize,
        binding: Option<deckmaste_lexical::Binding>,
        /// A selected `PP` slot: the phrase's preposition must be one of these.
        prepositions: Option<Vec<LexemeId>>,
        /// An unselected adjunct: a phrase with a preposition must admit this host.
        host: Option<HostKind>,
    },
    Leaf(Test),
    Punct(char, deckmaste_lexical::Binding),
}

/// A compiled Production.
#[derive(Debug, Clone)]
pub(crate) struct Production {
    pub id: String,
    pub lhs: usize,
    pub rhs: Vec<Symbol>,
    /// The name of each child: its grammatical function in the mother.
    pub names: Vec<String>,
    pub gaps: Vec<GapRule>,
    pub cost: u32,
    /// Constant bindings of each variable.
    pub constants: Vec<Option<u16>>,
    /// (feature, variable) bindings of each child.
    pub child_vars: Vec<Vec<(usize, usize)>>,
    /// (feature, variable) bindings of the mother, in category feature order.
    pub mother_vars: Vec<Option<usize>>,
    pub introduces: Gap,
    pub frame: Option<usize>,
    pub host_check: Option<(usize, usize, HostKind)>,
    pub coordinator: Option<usize>,
    pub distinct_coordinator: Vec<usize>,
    pub requires_coordination: Option<usize>,
    /// The child whose engine fields (preposition, extensions) the mother inherits.
    pub head: Option<usize>,
}

pub(crate) type Key = (Option<usize>, usize);

/// Builds a Production from resolved equations by unifying paths into variables.
#[expect(clippy::too_many_arguments, reason = "one internal call shape")]
pub(crate) fn resolve(
    grammar: &CompiledGrammar,
    id: String,
    lhs: usize,
    children: Vec<(String, Symbol, GapRule)>,
    equations: Vec<(Key, Result<Key, u16>)>,
    cost: u32,
    introduces: Gap,
    frame: Option<usize>,
    host_check: Option<(usize, usize, HostKind)>,
    coordinator: Option<usize>,
    distinct_coordinator: Vec<usize>,
    requires_coordination: Option<usize>,
) -> Result<Production, String> {
    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut keys: Vec<Key> = Vec::new();
    let mut parent: Vec<usize> = Vec::new();
    let mut constant: Vec<Option<u16>> = Vec::new();
    let var = |keys: &mut Vec<Key>,
               parent: &mut Vec<usize>,
               constant: &mut Vec<Option<u16>>,
               key: Key| {
        if let Some(i) = keys.iter().position(|k| *k == key) {
            return i;
        }
        keys.push(key);
        parent.push(parent.len());
        constant.push(None);
        keys.len() - 1
    };
    for (path, target) in equations {
        let a = var(&mut keys, &mut parent, &mut constant, path);
        match target {
            Ok(other) => {
                let b = var(&mut keys, &mut parent, &mut constant, other);
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                if ra != rb {
                    match (constant[ra], constant[rb]) {
                        (Some(x), Some(y)) if x != y => {
                            return Err("contradictory feature equations".to_owned());
                        }
                        (Some(x), _) | (_, Some(x)) => constant[rb] = Some(x),
                        _ => {}
                    }
                    parent[ra] = rb;
                }
            }
            Err(value) => {
                let r = find(&mut parent, a);
                if constant[r].is_some_and(|c| c != value) {
                    return Err("contradictory feature requirements".to_owned());
                }
                constant[r] = Some(value);
            }
        }
    }
    // Renumber roots densely.
    let mut roots = Vec::new();
    let slot_of: Vec<usize> = (0..keys.len())
        .map(|i| {
            let r = find(&mut parent, i);
            roots.iter().position(|&x| x == r).unwrap_or_else(|| {
                roots.push(r);
                roots.len() - 1
            })
        })
        .collect();
    let constants = roots.iter().map(|&r| constant[r]).collect();
    let mut child_vars = vec![Vec::new(); children.len()];
    let mut mother_vars = vec![None; grammar.category_features[lhs].len()];
    for (i, key) in keys.iter().enumerate() {
        match key {
            (Some(child), feature) => child_vars[*child].push((*feature, slot_of[i])),
            (None, feature) => {
                let position = grammar.category_features[lhs]
                    .iter()
                    .position(|f| f == feature)
                    .expect("validated mother feature");
                mother_vars[position] = Some(slot_of[i]);
            }
        }
    }
    let head = children.iter().position(|(name, ..)| name == "head");
    let names = children.iter().map(|(name, ..)| name.clone()).collect();
    let (rhs, gaps) = children.into_iter().map(|(_, s, g)| (s, g)).unzip();
    if introduces != Gap::None && !grammar.category_gap[lhs] {
        return Err("introduces a gap on a category that cannot contain one".to_owned());
    }
    Ok(Production {
        id,
        lhs,
        rhs,
        names,
        gaps,
        cost,
        constants,
        child_vars,
        mother_vars,
        introduces,
        frame,
        host_check,
        coordinator,
        distinct_coordinator,
        requires_coordination,
        head,
    })
}

/// The finite grammatical summary of a completed Constituent: everything a
/// parent may inspect. Two Constituents with equal summaries are packed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Summary {
    pub values: Box<[Option<u16>]>,
    pub gap: Gap,
    /// The preposition and complement frame heading a Preposition Phrase.
    pub preposition: Option<(LexemeId, usize)>,
    /// Prepositions whose PP would extend this phrase's selected frame.
    pub extensions: Vec<LexemeId>,
    /// The coordinator of a coordination.
    pub coordinator: Option<LexemeId>,
}

/// A lexical leaf as a child sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeafFacts {
    pub integer: Option<i32>,
    pub notation: Option<deckmaste_lexical::Numeral>,
    pub lexeme: Option<LexemeId>,
    pub category: LexicalCategory,
    pub features: Vec<(usize, u16)>,
}

pub(crate) enum ChildView<'a> {
    Node(&'a Summary),
    Leaf(&'a LeafFacts),
    Punct,
}

/// The partial state of a Production whose first children have been found.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct State {
    vars: Box<[Option<u16>]>,
    gap: Option<Gap>,
    preposition: Option<(LexemeId, usize)>,
    extensions: Vec<LexemeId>,
    coordinator: Option<LexemeId>,
    conjunct_coordinators: Vec<LexemeId>,
    host_extensions: Vec<LexemeId>,
    adjunct_preposition: Option<LexemeId>,
}

impl Production {
    pub fn start(&self) -> State {
        State {
            vars: self.constants.clone().into_boxed_slice(),
            gap: None,
            preposition: None,
            extensions: Vec::new(),
            coordinator: None,
            conjunct_coordinators: Vec::new(),
            host_extensions: Vec::new(),
            adjunct_preposition: None,
        }
    }

    /// Adds child `position` to `state`, or `None` if it does not fit.
    pub fn accept(
        &self,
        grammar: &CompiledGrammar,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
    ) -> Option<State> {
        let mut next = state.clone();
        let lookup = |feature: usize| -> Option<u16> {
            match child {
                ChildView::Node(summary) => {
                    let category = match &self.rhs[position] {
                        Symbol::Cat { category, .. } => *category,
                        _ => return None,
                    };
                    let index = grammar.category_features[category]
                        .iter()
                        .position(|f| *f == feature)?;
                    summary.values[index]
                }
                ChildView::Leaf(leaf) => leaf
                    .features
                    .iter()
                    .find(|(f, _)| *f == feature)
                    .map(|(_, v)| *v),
                ChildView::Punct => None,
            }
        };
        for &(feature, var) in &self.child_vars[position] {
            if let Some(value) = lookup(feature) {
                match next.vars[var] {
                    Some(bound) if bound != value => return None,
                    _ => next.vars[var] = Some(value),
                }
            }
        }
        match child {
            ChildView::Node(summary) => {
                if let Symbol::Cat {
                    prepositions, host, ..
                } = &self.rhs[position]
                {
                    if let Some(list) = prepositions {
                        let (prep, _) = summary.preposition.as_ref()?;
                        if !list.contains(prep) {
                            return None;
                        }
                    }
                    if let (Some(kind), Some((_, frame))) = (host, &summary.preposition)
                        && !grammar.preposition_frames[*frame].hosts.admits(*kind)
                    {
                        return None;
                    }
                }
                match self.gaps[position] {
                    GapRule::Closed => {
                        if summary.gap != Gap::None {
                            return None;
                        }
                    }
                    GapRule::Require(gap) => {
                        if summary.gap != gap {
                            return None;
                        }
                    }
                    GapRule::Transparent => {
                        if summary.gap != Gap::None {
                            if next.gap.is_some_and(|g| g != Gap::None) {
                                return None;
                            }
                            next.gap = Some(summary.gap);
                        }
                    }
                    GapRule::Shared => match next.gap {
                        Some(g) if g != summary.gap => return None,
                        _ => next.gap = Some(summary.gap),
                    },
                }
                if self.requires_coordination == Some(position) && summary.coordinator.is_none() {
                    return None;
                }
                if self.distinct_coordinator.contains(&position)
                    && let Some(c) = &summary.coordinator
                {
                    next.conjunct_coordinators.push(*c);
                }
                if self.coordinator == Some(position) {
                    next.coordinator.clone_from(&summary.coordinator);
                }
                if self.head == Some(position) {
                    next.preposition.clone_from(&summary.preposition);
                    next.extensions.clone_from(&summary.extensions);
                }
                if let Some((host, adjunct, _)) = self.host_check {
                    if position == host {
                        next.host_extensions.clone_from(&summary.extensions);
                    }
                    if position == adjunct {
                        next.adjunct_preposition = summary.preposition.as_ref().map(|(p, _)| *p);
                    }
                }
            }
            ChildView::Leaf(leaf) => {
                if self.coordinator == Some(position) {
                    next.coordinator.clone_from(&leaf.lexeme);
                }
                if let Symbol::Leaf(Test::PrepositionFrame(frame)) = &self.rhs[position] {
                    next.preposition = leaf.lexeme.map(|l| (l, *frame));
                }
                if let (Symbol::Leaf(Test::Frame(frame)), Some(lexeme)) =
                    (&self.rhs[position], &leaf.lexeme)
                {
                    next.extensions = grammar
                        .extensions
                        .get(&(*lexeme, *frame))
                        .cloned()
                        .unwrap_or_default();
                }
            }
            ChildView::Punct => {}
        }
        Some(next)
    }

    /// The mother's summary once every child has been accepted.
    pub fn finish(&self, grammar: &CompiledGrammar, state: &State) -> Option<Summary> {
        if let Some(c) = &state.coordinator
            && state.conjunct_coordinators.contains(c)
        {
            return None;
        }
        if let (Some((_, _, HostKind::Predicate)), Some(p)) =
            (self.host_check, &state.adjunct_preposition)
            && state.host_extensions.contains(p)
        {
            return None;
        }
        let gap = if self.introduces == Gap::None {
            state.gap.unwrap_or(Gap::None)
        } else {
            if state.gap.is_some_and(|g| g != Gap::None) {
                return None;
            }
            self.introduces
        };
        if gap != Gap::None && !grammar.category_gap[self.lhs] {
            return None;
        }
        let coordinated = self.coordinator.is_some();
        Some(Summary {
            values: self
                .mother_vars
                .iter()
                .map(|var| var.and_then(|v| state.vars[v]))
                .collect(),
            gap,
            preposition: if coordinated { None } else { state.preposition },
            extensions: if coordinated { Vec::new() } else { state.extensions.clone() },
            coordinator: state.coordinator,
        })
    }
}

impl CompiledGrammar {
    /// Whether a lexical leaf satisfies a test.
    pub(crate) fn leaf_matches(&self, test: &Test, leaf: &LeafFacts) -> bool {
        let facts = leaf.lexeme.as_ref().and_then(|l| self.lexemes.get(l));
        match test {
            Test::Lexical(category) => leaf.category == *category,
            Test::FreeLexical(category) => {
                leaf.category == *category
                    && facts.is_some_and(|f| f.binding == deckmaste_lexical::Binding::Free)
            }
            Test::Numeral(notation) => leaf.notation == Some(*notation),
            Test::Word(id) => leaf.lexeme.as_ref() == Some(id),
            Test::Words(words) => leaf.lexeme.is_some_and(|id| words.contains(&id)),
            Test::Class(class) => leaf
                .lexeme
                .as_ref()
                .is_some_and(|l| self.classes[*class].contains(l)),
            Test::Frame(frame) => facts.is_some_and(|f| f.frames.contains(frame)),
            Test::PrepositionFrame(frame) => {
                facts.is_some_and(|f| f.preposition_frames.contains(frame))
            }
        }
    }

    /// Projects a lexical alternative into the leaf a Production sees.
    pub(crate) fn leaf_facts(&self, reading: &LexicalReading) -> Option<LeafFacts> {
        match reading {
            LexicalReading::Word(value) => {
                let facts = self.lexemes.get(&value.lexeme)?;
                let f = &value.features;
                let verb_form = match value.form {
                    WordForm::Plain => Some("Plain"),
                    WordForm::Present | WordForm::Preterite => Some("Finite"),
                    WordForm::GerundParticiple => Some("GerundParticiple"),
                    WordForm::PastParticiple => Some("PastParticiple"),
                    _ => None,
                };
                let names = [
                    f.number.map(|n| format!("{n:?}")),
                    f.person.map(|p| format!("{p:?}")),
                    f.tense.map(|t| format!("{t:?}")),
                    verb_form.map(str::to_owned),
                    f.case.map(|c| format!("{c:?}")),
                ];
                let mut features = Vec::new();
                for (id, name) in self.projected.iter().zip(names) {
                    if let (Some(id), Some(name)) = (id, name) {
                        let value = self.values[*id].iter().position(|v| *v == name)?;
                        features.push((*id, u16::try_from(value).ok()?));
                    }
                }
                Some(LeafFacts {
                    integer: None,
                    notation: None,
                    lexeme: Some(value.lexeme),
                    category: facts.category,
                    features,
                })
            }
            LexicalReading::Numeral {
                value, notation, ..
            } => Some(LeafFacts {
                integer: Some(*value),
                notation: Some(*notation),
                lexeme: None,
                category: LexicalCategory::Numeral,
                features: if *notation == deckmaste_lexical::Numeral::Cardinal {
                    self.projected[0]
                        .and_then(|id| {
                            let number = if *value == 1 { "Singular" } else { "Plural" };
                            self.values[id]
                                .iter()
                                .position(|v| v == number)
                                .and_then(|v| u16::try_from(v).ok())
                                .map(|v| (id, v))
                        })
                        .into_iter()
                        .collect()
                } else {
                    Vec::new()
                },
            }),
            LexicalReading::FlavorWord { .. } => Some(LeafFacts {
                integer: None,
                notation: None,
                lexeme: None,
                category: LexicalCategory::FlavorWord,
                features: Vec::new(),
            }),
        }
    }
}

/// Every lexical value the engine projects onto a grammar feature of this name.
pub(crate) fn projected_values(feature: &str) -> &'static [&'static str] {
    match feature {
        "Number" => &["Singular", "Plural"],
        "Person" => &["First", "Second", "Third"],
        "Tense" => &["Present", "Past"],
        "VerbForm" => &["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Case" => &["Nominative", "Accusative", "Genitive"],
        _ => &[],
    }
}
