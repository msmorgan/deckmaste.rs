//! Productions as compiled, and the one step function shared by the chart
//! and by Admission: adding a child to a partial Production.

use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::WordForm;
use serde::Deserialize;
use serde::Serialize;

use crate::compile::CompiledGrammar;
use crate::frames::SlotKind;
use crate::grammar::HostKind;

/// A Complement that another frame of a phrase's head selects. A longer
/// frame's Preposition Phrase headed by the preposition, or phrase of the
/// slot's kind (*to*-infinitival in *choose to attack*), is not also an
/// unselected adjunct of the same head instance; a Preposition Phrase that a
/// frame selects in place of a predicative Complement (locative *on* after
/// *be*) is not also that head's predicative Complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Extension {
    Preposition(LexemeId),
    Slot(SlotKind),
    Predicative(LexemeId),
}

/// The unfilled Gap a Constituent contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Gap {
    None,
    NP,
    /// An extracted adjunct, as in an adverbial or pied-piped relative clause.
    Adjunct,
    /// An extracted predicative Complement (*whose turn it is*).
    Pred,
}

/// How a child's Gap relates to its mother's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GapRule {
    /// The child contains no Gap.
    Closed,
    /// The child's Gap, if any, is the mother's.
    Transparent,
    /// Every shared child has the same Gap, which is the mother's (across the
    /// board).
    Shared,
    /// The child has exactly this Gap, filled within the mother.
    Require(Gap),
    /// The child has exactly this Gap, which is the mother's.
    Raise(Gap),
}

/// How many lexical leaves a child must span: one, or more than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Weight {
    Light,
    Heavy,
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
    /// A free (unbound) lexeme that declares this Subcategorization Frame.
    FreeFrame(usize),
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
        /// An unselected adjunct: a phrase with a preposition must admit this
        /// host.
        host: Option<HostKind>,
    },
    Leaf(Test),
    Punct(char, deckmaste_lexical::Binding),
}

/// A compiled Production.
#[derive(Debug, Clone)]
pub(crate) struct Production {
    pub exclusions: Vec<(usize, usize, u16)>,
    /// Children that must be light or heavy.
    pub weights: Vec<(usize, Weight)>,
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
    /// The Frame Slot kind of a non-prepositional adjunct, for same-head
    /// complement priority.
    pub adjunct_slot: Option<SlotKind>,
    /// The predicative Complement child of a frame-headed phrase.
    pub predicative: Option<usize>,
    pub coordinator: Option<usize>,
    pub distinct_coordinator: Vec<usize>,
    pub requires_coordination: Option<usize>,
    /// The child whose engine fields (preposition, extensions) the mother
    /// inherits.
    pub head: Option<usize>,
}

pub(crate) type Key = (Option<usize>, usize);

/// Builds a Production from resolved equations by unifying paths into
/// variables.
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
        exclusions: Vec::new(),
        weights: Vec::new(),
        adjunct_slot: None,
        predicative: None,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Summary {
    pub first_glues: bool,
    pub last_glues: bool,
    pub values: Box<[Option<u16>]>,
    pub gap: Gap,
    pub gap_right_edge: bool,
    /// Its lexical leaves, counted to two.
    pub leaves: u8,
    /// The preposition and complement frame heading a Preposition Phrase.
    pub preposition: Option<(LexemeId, usize)>,
    /// Complements that would extend this phrase's selected frame.
    pub extensions: Vec<Extension>,
    /// The coordinator of a coordination.
    pub coordinator: Option<LexemeId>,
}

/// A lexical leaf as a child sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LeafFacts {
    pub binding: deckmaste_lexical::Binding,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct State {
    pub first_glues: Option<bool>,
    pub last_glues: bool,
    vars: Box<[Option<u16>]>,
    gap: Option<Gap>,
    gap_right_edge: bool,
    shared_gap_right_edge: bool,
    /// Lexical leaves so far, counted to two.
    leaves: u8,
    preposition: Option<(LexemeId, usize)>,
    extensions: Vec<Extension>,
    coordinator: Option<LexemeId>,
    conjunct_coordinators: Vec<LexemeId>,
    host_extensions: Vec<Extension>,
    adjunct_extension: Option<Extension>,
}

impl Production {
    pub(crate) fn boundaries(&self, position: usize, child: &ChildView<'_>) -> (bool, bool) {
        use crate::reading::glues_after;
        use crate::reading::glues_before;
        let (mut first, mut last) = match child {
            ChildView::Node(summary) => (summary.first_glues, summary.last_glues),
            ChildView::Leaf(leaf) => (glues_before(leaf.binding), glues_after(leaf.binding)),
            ChildView::Punct => match self.rhs[position] {
                Symbol::Punct(_, binding) => (glues_before(binding), glues_after(binding)),
                _ => (false, false),
            },
        };
        if let Symbol::Cat {
            binding: Some(binding),
            ..
        } = self.rhs[position]
        {
            first |= glues_before(binding);
            last |= glues_after(binding);
        }
        (first, last)
    }

    pub(crate) fn spacing_matches(
        &self,
        state: &State,
        position: usize,
        child: &ChildView<'_>,
        gap: &[char],
    ) -> bool {
        if state.first_glues.is_none() {
            return true;
        }
        let (first, _) = self.boundaries(position, child);
        if state.last_glues || first { gap.is_empty() } else { gap == [' '] }
    }

    pub fn start(&self) -> State {
        State {
            first_glues: None,
            last_glues: false,
            vars: self.constants.clone().into_boxed_slice(),
            gap: None,
            gap_right_edge: true,
            shared_gap_right_edge: true,
            leaves: 0,
            preposition: None,
            extensions: Vec::new(),
            coordinator: None,
            conjunct_coordinators: Vec::new(),
            host_extensions: Vec::new(),
            adjunct_extension: None,
        }
    }

    /// Records the head's and an adjunct's frame facts for same-head
    /// complement priority, or `None` when a predicative Complement is a
    /// Preposition Phrase a rival frame selects.
    fn priority(&self, next: &mut State, position: usize, summary: &Summary) -> Option<()> {
        if self.predicative == Some(position)
            && let Some((p, _)) = summary.preposition
            && next.extensions.contains(&Extension::Predicative(p))
        {
            return None;
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
                next.adjunct_extension = summary
                    .preposition
                    .as_ref()
                    .map(|(p, _)| Extension::Preposition(*p))
                    .or(self.adjunct_slot.map(Extension::Slot));
            }
        }
        Some(())
    }

    /// The lexical leaves child `position` adds, counted to two, or `None` if
    /// it is not the weight the Production requires.
    fn weigh(&self, position: usize, child: &ChildView<'_>) -> Option<u8> {
        let leaves = match child {
            ChildView::Node(summary) => summary.leaves,
            ChildView::Leaf(_) => 1,
            ChildView::Punct => 0,
        };
        match self.weights.iter().find(|(at, _)| *at == position) {
            Some((_, Weight::Light)) if leaves != 1 => None,
            Some((_, Weight::Heavy)) if leaves < 2 => None,
            _ => Some(leaves),
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
        if state.gap.is_some_and(|gap| gap != Gap::None) {
            next.gap_right_edge = false;
        }
        // A free child does not begin with an atom glued to the one before.
        if let (
            Symbol::Cat {
                binding: Some(deckmaste_lexical::Binding::Free),
                ..
            },
            ChildView::Node(summary),
        ) = (&self.rhs[position], child)
            && summary.first_glues
        {
            return None;
        }
        let (first, last) = self.boundaries(position, child);
        next.first_glues.get_or_insert(first);
        next.last_glues = last;
        next.leaves = (next.leaves + self.weigh(position, child)?).min(2);
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
        if self
            .exclusions
            .iter()
            .any(|&(at, feature, value)| at == position && lookup(feature) == Some(value))
        {
            return None;
        }
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
                if summary.gap != Gap::None {
                    next.gap_right_edge = summary.gap_right_edge;
                    if matches!(self.gaps[position], GapRule::Shared) {
                        next.shared_gap_right_edge &= summary.gap_right_edge;
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
                    GapRule::Raise(gap) => {
                        if summary.gap != gap || next.gap.is_some_and(|g| g != Gap::None) {
                            return None;
                        }
                        next.gap = Some(gap);
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
                if self.requires_coordination == Some(position)
                    && (summary.coordinator.is_none() || !summary.gap_right_edge)
                {
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
                self.priority(&mut next, position, summary)?;
            }
            ChildView::Leaf(leaf) => {
                if self.coordinator == Some(position) {
                    next.coordinator.clone_from(&leaf.lexeme);
                }
                if let Symbol::Leaf(Test::PrepositionFrame(frame)) = &self.rhs[position] {
                    next.preposition = leaf.lexeme.map(|l| (l, *frame));
                }
                if let (Symbol::Leaf(Test::Frame(frame) | Test::FreeFrame(frame)), Some(lexeme)) =
                    (&self.rhs[position], &leaf.lexeme)
                {
                    next.extensions = grammar
                        .extensions
                        .get(&(*lexeme, Some(*frame)))
                        .cloned()
                        .unwrap_or_default();
                }
                if let (Symbol::Leaf(Test::FreeLexical(_)), Some(lexeme)) =
                    (&self.rhs[position], leaf.lexeme)
                {
                    next.extensions = grammar
                        .extensions
                        .get(&(lexeme, None))
                        .cloned()
                        .unwrap_or_default();
                }
                if self.host_check.is_some_and(|(host, _, _)| host == position) {
                    next.host_extensions.clone_from(&next.extensions);
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
        if let (Some((_, _, _)), Some(e)) = (self.host_check, &state.adjunct_extension)
            && state.host_extensions.contains(e)
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
            first_glues: state.first_glues.unwrap_or(false),
            last_glues: state.last_glues,
            values: self
                .mother_vars
                .iter()
                .map(|var| var.and_then(|v| state.vars[v]))
                .collect(),
            gap,
            gap_right_edge: self.introduces != Gap::None
                || (state.gap_right_edge && state.shared_gap_right_edge),
            leaves: state.leaves,
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
            Test::FreeFrame(frame) => facts.is_some_and(|f| {
                f.frames.contains(frame) && f.binding == deckmaste_lexical::Binding::Free
            }),
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
                    Some(format!("{:?}", value.form)),
                    facts.countability.clone(),
                    None,
                ];
                let mut features = facts.features.clone();
                for (id, name) in self.projected.iter().zip(names) {
                    if let (Some(id), Some(name)) = (id, name) {
                        let value = self.values[*id].iter().position(|v| *v == name)?;
                        features.push((*id, u16::try_from(value).ok()?));
                    }
                }
                Some(LeafFacts {
                    binding: facts.binding,
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
                binding: deckmaste_lexical::Binding::Free,
                integer: Some(*value),
                notation: Some(*notation),
                lexeme: None,
                category: LexicalCategory::Numeral,
                features: {
                    let number = matches!(
                        notation,
                        deckmaste_lexical::Numeral::Cardinal
                            | deckmaste_lexical::Numeral::Arabic(_)
                    )
                    .then_some(if *value == 1 { "Singular" } else { "Plural" });
                    let sign = match value.cmp(&0) {
                        std::cmp::Ordering::Less => "Negative",
                        std::cmp::Ordering::Equal => "Zero",
                        std::cmp::Ordering::Greater => "Positive",
                    };
                    [(self.projected[0], number), (self.projected[7], Some(sign))]
                        .into_iter()
                        .filter_map(|(id, name)| {
                            let (id, name) = (id?, name?);
                            let value = self.values[id].iter().position(|v| v == name)?;
                            Some((id, u16::try_from(value).ok()?))
                        })
                        .collect()
                },
            }),
            LexicalReading::FlavorWord { .. } => Some(LeafFacts {
                binding: deckmaste_lexical::Binding::Free,
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
        "Sign" => &["Negative", "Zero", "Positive"],
        "Person" => &["First", "Second", "Third"],
        "Tense" => &["Present", "Past"],
        "VerbForm" => &["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Case" => &["Nominative", "Accusative", "Genitive"],
        "Countability" => &["Count", "Mass"],
        "Inflection" => &[
            "Invariant",
            "IndependentGenitive",
            "Singular",
            "Plural",
            "Plain",
            "Present",
            "Preterite",
            "Irrealis",
            "GerundParticiple",
            "PastParticiple",
        ],
        _ => &[],
    }
}
