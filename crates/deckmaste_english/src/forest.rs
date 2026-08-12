use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::rc::Rc;

use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

use crate::chart::RuleId;
use crate::construction::ConstructionRegistry;
use crate::construction::ProductionId;
use crate::features::ExactParse;
use crate::features::SurfaceWitnessPayload;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ParseCost {
    pub(crate) opaque_words: u32,
    pub(crate) opaque_lexemes: u32,
    pub(crate) generic_rules: u32,
    /// A soft dispreference among competing readings of the same span, ranked
    /// below the structural fields but above `precedence`. Higher loses. It
    /// orders the ambiguous readings the scanner offers for a card's own name:
    ///
    /// - `0` — an ordinary reading (a catalog term, a lowercase common word, an
    ///   opaque proper noun): most preferred, so a self-reference wins only
    ///   when nothing else equally structural explains the tokens.
    /// - `1` — a full-name self-reference.
    /// - `2` — a shortened-name (nickname) self-reference.
    /// - `3` — a capitalized word read as a common Noun/Adjective/Verb solely
    ///   by sentence-initial license. This reading lowercases the word, so it
    ///   must lose to a self-reference that reproduces the capitalized surface
    ///   (a nickname that coincides with a common noun, e.g. `Carnage`, in a
    ///   re-parsed trigger event whose position 0 only looks sentence-initial).
    pub(crate) reading_dispreference: u32,
    /// Number of licensed, selectionally constrained attachment decisions.
    /// Unlike every dispreference in this cost, higher wins; distance then
    /// chooses the nearest boundary among readings with the same count.
    pub(crate) attachment_count: u32,
    /// Source-token distance crossed by those attachment decisions. Lower
    /// wins after `attachment_count`, so a host cannot swallow the beginning
    /// of its relative clause merely to start later.
    pub(crate) attachment_distance: u32,
    /// Source-token extent of a grammar-local open attachment. Higher wins;
    /// only dedicated constructions set it, after their categorical gates
    /// have excluded unrelated phrase types.
    pub(crate) attachment_extent: u32,
    pub(crate) precedence: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseCostDimension {
    OpaqueWords,
    OpaqueLexemes,
    GenericRules,
    ReadingDispreference,
    AttachmentCount,
    AttachmentDistance,
    AttachmentExtent,
    Precedence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionReason {
    Unique,
    Cost(ParseCostDimension),
    Dominance,
    StableIdentity,
}

impl ParseCost {
    #[must_use]
    pub const fn opaque_words(self) -> u32 {
        self.opaque_words
    }

    #[must_use]
    pub const fn opaque_lexemes(self) -> u32 {
        self.opaque_lexemes
    }

    #[must_use]
    pub const fn generic_rules(self) -> u32 {
        self.generic_rules
    }

    #[must_use]
    pub const fn reading_dispreference(self) -> u32 {
        self.reading_dispreference
    }

    #[must_use]
    pub const fn attachment_count(self) -> u32 {
        self.attachment_count
    }

    #[must_use]
    pub const fn attachment_distance(self) -> u32 {
        self.attachment_distance
    }

    #[must_use]
    pub const fn attachment_extent(self) -> u32 {
        self.attachment_extent
    }

    #[must_use]
    pub const fn precedence(self) -> u32 {
        self.precedence
    }

    /// Returns the first cost dimension that distinguishes two costs in the
    /// same lexicographic order used by [`Ord`].
    #[must_use]
    pub const fn decisive_dimension(self, other: Self) -> Option<ParseCostDimension> {
        if self.opaque_words != other.opaque_words {
            Some(ParseCostDimension::OpaqueWords)
        } else if self.opaque_lexemes != other.opaque_lexemes {
            Some(ParseCostDimension::OpaqueLexemes)
        } else if self.generic_rules != other.generic_rules {
            Some(ParseCostDimension::GenericRules)
        } else if self.reading_dispreference != other.reading_dispreference {
            Some(ParseCostDimension::ReadingDispreference)
        } else if self.attachment_count != other.attachment_count {
            Some(ParseCostDimension::AttachmentCount)
        } else if self.attachment_distance != other.attachment_distance {
            Some(ParseCostDimension::AttachmentDistance)
        } else if self.attachment_extent != other.attachment_extent {
            Some(ParseCostDimension::AttachmentExtent)
        } else if self.precedence != other.precedence {
            Some(ParseCostDimension::Precedence)
        } else {
            None
        }
    }
}

impl Ord for ParseCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.opaque_words
            .cmp(&other.opaque_words)
            .then_with(|| self.opaque_lexemes.cmp(&other.opaque_lexemes))
            .then_with(|| self.generic_rules.cmp(&other.generic_rules))
            .then_with(|| {
                self.reading_dispreference
                    .cmp(&other.reading_dispreference)
            })
            // More licensed decisions are preferred, hence reversed operands.
            .then_with(|| {
                other.attachment_count.cmp(&self.attachment_count)
            })
            .then_with(|| self.attachment_distance.cmp(&other.attachment_distance))
            .then_with(|| other.attachment_extent.cmp(&self.attachment_extent))
            .then_with(|| self.precedence.cmp(&other.precedence))
    }
}

impl PartialOrd for ParseCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for ParseCost {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for ParseCost {
    fn add_assign(&mut self, rhs: Self) {
        self.opaque_words = self.opaque_words.saturating_add(rhs.opaque_words);
        self.opaque_lexemes = self.opaque_lexemes.saturating_add(rhs.opaque_lexemes);
        self.generic_rules = self.generic_rules.saturating_add(rhs.generic_rules);
        self.reading_dispreference = self
            .reading_dispreference
            .saturating_add(rhs.reading_dispreference);
        self.attachment_count = self.attachment_count.saturating_add(rhs.attachment_count);
        self.attachment_distance = self
            .attachment_distance
            .saturating_add(rhs.attachment_distance);
        self.attachment_extent = self.attachment_extent.saturating_add(rhs.attachment_extent);
        self.precedence = self.precedence.saturating_add(rhs.precedence);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct NodeId(usize);

impl NodeId {
    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ForestSymbol<N, L> {
    Nonterminal(N),
    Lexical(L),
    Intermediate { rule: RuleId, dot: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ForestFeatures<F> {
    Constituent(F),
    Prefix(Rc<[F]>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ForestStats {
    constituent_nodes: usize,
    intermediate_nodes: usize,
    packed_alternatives: usize,
    max_alternatives: usize,
}

impl ForestStats {
    #[must_use]
    pub const fn constituent_nodes(self) -> usize {
        self.constituent_nodes
    }

    #[must_use]
    pub const fn intermediate_nodes(self) -> usize {
        self.intermediate_nodes
    }

    #[must_use]
    pub const fn packed_alternatives(self) -> usize {
        self.packed_alternatives
    }

    #[must_use]
    pub const fn max_alternatives(self) -> usize {
        self.max_alternatives
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeKey<N, L, F, M> {
    pub(crate) symbol: ForestSymbol<N, L>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    features: ForestFeatures<F>,
    lexical_value: Option<M>,
}

impl<N, L, F, M> NodeKey<N, L, F, M> {
    pub(crate) fn nonterminal(symbol: N, start: usize, end: usize, features: F) -> Self {
        Self {
            symbol: ForestSymbol::Nonterminal(symbol),
            start,
            end,
            features: ForestFeatures::Constituent(features),
            lexical_value: None,
        }
    }

    pub(crate) fn lexical(slot: L, start: usize, end: usize, features: F, value: M) -> Self {
        Self {
            symbol: ForestSymbol::Lexical(slot),
            start,
            end,
            features: ForestFeatures::Constituent(features),
            lexical_value: Some(value),
        }
    }

    pub(crate) fn intermediate(
        rule: RuleId,
        dot: usize,
        start: usize,
        end: usize,
        prefix_features: Rc<[F]>,
    ) -> Self {
        Self {
            symbol: ForestSymbol::Intermediate { rule, dot },
            start,
            end,
            features: ForestFeatures::Prefix(prefix_features),
            lexical_value: None,
        }
    }

    pub(crate) fn constituent_features(&self) -> Option<&F> {
        match &self.features {
            ForestFeatures::Constituent(features) => Some(features),
            ForestFeatures::Prefix(_) => None,
        }
    }

    pub(crate) fn lexical_value(&self) -> Option<&M> {
        self.lexical_value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PackedAlternative<W: SurfaceWitnessPayload> {
    pub(crate) rule: Option<RuleId>,
    pub(crate) production: Option<ProductionId>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) local_cost: ParseCost,
    pub(crate) surface: W,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForestNode<N, L, F, M, W: SurfaceWitnessPayload> {
    pub(crate) key: NodeKey<N, L, F, M>,
    pub(crate) alternatives: Vec<PackedAlternative<W>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InternResult {
    pub(crate) node: NodeId,
    pub(crate) node_was_new: bool,
    pub(crate) alternative_was_new: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ParseForest<N, L, F, M, W: SurfaceWitnessPayload> {
    nodes: Vec<ForestNode<N, L, F, M, W>>,
    node_ids: HashMap<NodeKey<N, L, F, M>, NodeId>,
}

impl<N, L, F, M, W> ParseForest<N, L, F, M, W>
where
    N: Clone + Eq + Hash,
    L: Clone + Eq + Hash,
    F: Clone + Eq + Hash,
    M: Clone + Eq + Hash,
    W: Clone + Eq + SurfaceWitnessPayload,
{
    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_ids: HashMap::new(),
        }
    }

    pub(crate) fn intern_node(
        &mut self,
        key: NodeKey<N, L, F, M>,
        alternative: PackedAlternative<W>,
    ) -> InternResult {
        match self.node_ids.entry(key) {
            Entry::Occupied(entry) => {
                let node = *entry.get();
                let alternatives = &mut self.nodes[node.index()].alternatives;
                let alternative_was_new = !alternatives.contains(&alternative);
                if alternative_was_new {
                    alternatives.push(alternative);
                }
                InternResult {
                    node,
                    node_was_new: false,
                    alternative_was_new,
                }
            }
            Entry::Vacant(entry) => {
                let key = entry.key().clone();
                let node = NodeId(self.nodes.len());
                entry.insert(node);
                self.nodes.push(ForestNode {
                    key,
                    alternatives: vec![alternative],
                });
                InternResult {
                    node,
                    node_was_new: true,
                    alternative_was_new: true,
                }
            }
        }
    }

    pub(crate) fn node(&self, node: NodeId) -> &ForestNode<N, L, F, M, W> {
        &self.nodes[node.index()]
    }

    pub(crate) fn nodes(&self) -> impl Iterator<Item = &ForestNode<N, L, F, M, W>> {
        self.nodes.iter()
    }

    /// Enumerates every per-node-consistent selection reachable from `root`,
    /// decrementing `remaining` once per completed selection. Completing a
    /// selection with `remaining` at 0 is
    /// [`SelectionEnumerationError::BudgetExhausted`] — the caller's budget
    /// is a hard ceiling, never a truncation point.
    #[cfg(test)]
    pub(crate) fn enumerate_selections(
        &self,
        root: NodeId,
        remaining: &mut usize,
    ) -> Result<Vec<ChoiceMap>, SelectionEnumerationError> {
        if let Some(node) = self.reachable_cycle(root) {
            return Err(SelectionEnumerationError::Cycle(node));
        }
        let mut choice = vec![None; self.nodes.len()];
        let mut out = Vec::new();
        self.expand_selections(vec![root], &mut choice, &mut out, remaining)?;
        Ok(out)
    }

    #[cfg(test)]
    fn expand_selections(
        &self,
        mut agenda: Vec<NodeId>,
        choice: &mut Vec<Option<usize>>,
        out: &mut Vec<ChoiceMap>,
        remaining: &mut usize,
    ) -> Result<(), SelectionEnumerationError> {
        let node = loop {
            match agenda.pop() {
                None => {
                    if *remaining == 0 {
                        return Err(SelectionEnumerationError::BudgetExhausted);
                    }
                    *remaining -= 1;
                    out.push(ChoiceMap {
                        alternatives: choice.clone(),
                    });
                    return Ok(());
                }
                // A shared node reached again inside one derivation: its
                // choice (and its children, already on an earlier agenda)
                // are settled — the per-node map binds every occurrence.
                Some(node) if choice[node.index()].is_some() => {}
                Some(node) => break node,
            }
        };
        for (index, alternative) in self.node(node).alternatives.iter().enumerate() {
            choice[node.index()] = Some(index);
            let mut next = agenda.clone();
            next.extend(alternative.children.iter().copied());
            self.expand_selections(next, choice, out, remaining)?;
            choice[node.index()] = None;
        }
        Ok(())
    }

    #[cfg(test)]
    fn reachable_cycle(&self, root: NodeId) -> Option<NodeId> {
        let mut visiting = vec![false; self.nodes.len()];
        let mut visited = vec![false; self.nodes.len()];
        self.cycle_visit(root, &mut visiting, &mut visited)
    }

    #[cfg(test)]
    fn cycle_visit(
        &self,
        node: NodeId,
        visiting: &mut [bool],
        visited: &mut [bool],
    ) -> Option<NodeId> {
        if visiting[node.index()] {
            return Some(node);
        }
        if visited[node.index()] {
            return None;
        }
        visiting[node.index()] = true;
        for alternative in &self.node(node).alternatives {
            for &child in &alternative.children {
                if let Some(cycle) = self.cycle_visit(child, visiting, visited) {
                    return Some(cycle);
                }
            }
        }
        visiting[node.index()] = false;
        visited[node.index()] = true;
        None
    }

    #[allow(
        dead_code,
        reason = "the retained-alternative exact-result carrier is staged for its public consumer"
    )]
    pub(crate) fn exact_result<T>(
        &self,
        node: NodeId,
        alternative: usize,
        ast: T,
    ) -> Option<ExactParse<T, W>> {
        let surface = self
            .node(node)
            .alternatives
            .get(alternative)?
            .surface
            .clone();
        Some(ExactParse::new(ast, surface))
    }

    pub(crate) fn stats(&self) -> ForestStats {
        let mut stats = ForestStats::default();
        for node in &self.nodes {
            match node.key.symbol {
                ForestSymbol::Nonterminal(_) | ForestSymbol::Lexical(_) => {
                    stats.constituent_nodes += 1;
                }
                ForestSymbol::Intermediate { .. } => stats.intermediate_nodes += 1,
            }
            stats.packed_alternatives += node.alternatives.len();
            stats.max_alternatives = stats.max_alternatives.max(node.alternatives.len());
        }
        stats
    }

    #[cfg(test)]
    pub(crate) fn best_root(
        &self,
        roots: impl IntoIterator<Item = NodeId>,
        registry: &ConstructionRegistry,
    ) -> Result<Option<(NodeId, BestParse)>, ForestError> {
        self.best_root_matching(roots, registry, |_, _| true)
    }

    /// Ranks complete roots by ordinary cost and stable derivation identity,
    /// returning the first root accepted by `matches`.
    ///
    /// The packed choices for every root are computed once. This lets a
    /// consumer reject a root for a property outside the forest's structural
    /// cost model (notably, inability to lower into the public syntax tree)
    /// without changing the winner whenever the original best root qualifies.
    pub(crate) fn best_root_matching(
        &self,
        roots: impl IntoIterator<Item = NodeId>,
        registry: &ConstructionRegistry,
        mut matches: impl FnMut(NodeId, &BestParse) -> bool,
    ) -> Result<Option<(NodeId, BestParse)>, ForestError> {
        let mut selection = SelectionState::new(self.nodes.len());
        let mut candidates = Vec::new();
        for root in roots {
            let cost = self.best_cost(root, registry, &mut selection)?;
            candidates.push((
                cost,
                selection.identities[root.index()].clone(),
                root.index(),
                root,
            ));
        }
        candidates.sort_unstable();
        let mut best = BestParse {
            cost: ParseCost::default(),
            costs: selection.costs,
            alternatives: selection.alternatives,
            selected_productions: selection.selected_productions,
            decision_productions: selection.decision_productions,
            candidate_productions: selection.candidate_productions,
            equal_cost_alternatives: selection.equal_cost_alternatives,
            tied_alternatives: selection.tied_alternatives,
            reasons: selection.reasons,
        };
        for (cost, _, _, root) in candidates {
            best.cost = cost;
            if matches(root, &best) {
                return Ok(Some((root, best)));
            }
        }
        Ok(None)
    }

    fn best_cost(
        &self,
        node: NodeId,
        registry: &ConstructionRegistry,
        selection: &mut SelectionState,
    ) -> Result<ParseCost, ForestError> {
        if let Some(cost) = selection.costs[node.index()] {
            return Ok(cost);
        }
        if selection.visiting[node.index()] {
            return Err(ForestError::Cycle(node));
        }
        selection.visiting[node.index()] = true;

        let forest_node = self.node(node);
        let mut scored = Vec::with_capacity(forest_node.alternatives.len());
        for (alternative_index, alternative) in forest_node.alternatives.iter().enumerate() {
            let mut cost = alternative.local_cost;
            let mut identity = alternative.production.into_iter().collect::<Vec<_>>();
            for &child in &alternative.children {
                cost += self.best_cost(child, registry, selection)?;
                identity.extend_from_slice(&selection.identities[child.index()]);
            }
            scored.push(ScoredAlternative {
                cost,
                index: alternative_index,
                identity,
            });
        }

        selection.visiting[node.index()] = false;
        let cost = scored
            .iter()
            .map(|candidate| candidate.cost)
            .min()
            .ok_or(ForestError::MissingAlternative(node))?;
        let equal = scored
            .iter()
            .filter(|candidate| candidate.cost == cost)
            .collect::<Vec<_>>();
        selection.equal_cost_alternatives[node.index()]
            .extend(equal.iter().map(|candidate| candidate.index));

        let mut viable = equal
            .iter()
            .copied()
            .filter(|candidate| {
                !equal
                    .iter()
                    .copied()
                    .any(|other| strictly_dominates(registry, &other.identity, &candidate.identity))
            })
            .collect::<Vec<_>>();
        viable.sort_unstable_by(|left, right| {
            left.identity
                .cmp(&right.identity)
                .then_with(|| left.index.cmp(&right.index))
        });
        selection.tied_alternatives[node.index()]
            .extend(viable.iter().map(|candidate| candidate.index));

        let selected = viable
            .first()
            .ok_or(ForestError::MissingAlternative(node))?;
        let alternative = selected.index;
        let reason = if viable.len() < equal.len() {
            SelectionReason::Dominance
        } else if viable.len() > 1 {
            SelectionReason::StableIdentity
        } else if scored.len() > 1 {
            let next_cost = scored
                .iter()
                .map(|candidate| candidate.cost)
                .filter(|candidate_cost| *candidate_cost > cost)
                .min()
                .expect("a unique cheapest alternative must have a cost competitor");
            SelectionReason::Cost(
                cost.decisive_dimension(next_cost)
                    .expect("different costs must have a decisive dimension"),
            )
        } else {
            SelectionReason::Unique
        };

        let mut representatives = vec![None; forest_node.alternatives.len()];
        for candidate in &equal {
            let mut dominance_productions = Vec::new();
            let mut unmatched = Vec::new();
            for other in &equal {
                if candidate.index == other.index {
                    continue;
                }
                if let Some((dominant, _)) =
                    dominance_pair(registry, &candidate.identity, &other.identity)
                {
                    dominance_productions.push(dominant);
                }
                if let Some((_, subordinate)) =
                    dominance_pair(registry, &other.identity, &candidate.identity)
                {
                    dominance_productions.push(subordinate);
                }
                if let Some((candidate_production, _)) =
                    divergent_productions(&candidate.identity, &other.identity)
                {
                    unmatched.push(candidate_production);
                }
            }
            representatives[candidate.index] = dominance_productions
                .into_iter()
                .min()
                .or_else(|| unmatched.into_iter().min())
                .or(forest_node.alternatives[candidate.index].production)
                .or_else(|| candidate.identity.first().copied());
        }

        selection.costs[node.index()] = Some(cost);
        selection.alternatives[node.index()] = Some(alternative);
        selection.selected_productions[node.index()] =
            forest_node.alternatives[alternative].production;
        selection.decision_productions[node.index()] = representatives[alternative];
        selection.candidate_productions[node.index()] = representatives;
        selection.identities[node.index()].clone_from(&selected.identity);
        selection.reasons[node.index()] = Some(reason);
        Ok(cost)
    }
}

#[derive(Debug)]
struct ScoredAlternative {
    cost: ParseCost,
    index: usize,
    identity: Vec<ProductionId>,
}

#[derive(Debug)]
struct SelectionState {
    costs: Vec<Option<ParseCost>>,
    alternatives: Vec<Option<usize>>,
    selected_productions: Vec<Option<ProductionId>>,
    decision_productions: Vec<Option<ProductionId>>,
    candidate_productions: Vec<Vec<Option<ProductionId>>>,
    identities: Vec<Vec<ProductionId>>,
    equal_cost_alternatives: Vec<Vec<usize>>,
    tied_alternatives: Vec<Vec<usize>>,
    reasons: Vec<Option<SelectionReason>>,
    visiting: Vec<bool>,
}

impl SelectionState {
    fn new(nodes: usize) -> Self {
        Self {
            costs: vec![None; nodes],
            alternatives: vec![None; nodes],
            selected_productions: vec![None; nodes],
            decision_productions: vec![None; nodes],
            candidate_productions: vec![Vec::new(); nodes],
            identities: vec![Vec::new(); nodes],
            equal_cost_alternatives: vec![Vec::new(); nodes],
            tied_alternatives: vec![Vec::new(); nodes],
            reasons: vec![None; nodes],
            visiting: vec![false; nodes],
        }
    }
}

fn strictly_dominates(
    registry: &ConstructionRegistry,
    dominant: &[ProductionId],
    subordinate: &[ProductionId],
) -> bool {
    dominance_pair(registry, dominant, subordinate).is_some()
        && dominance_pair(registry, subordinate, dominant).is_none()
}

fn dominance_pair(
    registry: &ConstructionRegistry,
    dominant: &[ProductionId],
    subordinate: &[ProductionId],
) -> Option<(ProductionId, ProductionId)> {
    let (dominant, subordinate) = divergent_productions(dominant, subordinate)?;
    registry
        .dominates(dominant.construction, subordinate.construction)
        .then_some((dominant, subordinate))
}

fn divergent_productions(
    left: &[ProductionId],
    right: &[ProductionId],
) -> Option<(ProductionId, ProductionId)> {
    left.iter()
        .copied()
        .zip(right.iter().copied())
        .find(|(left, right)| left != right)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BestParse {
    pub(crate) cost: ParseCost,
    costs: Vec<Option<ParseCost>>,
    alternatives: Vec<Option<usize>>,
    selected_productions: Vec<Option<ProductionId>>,
    decision_productions: Vec<Option<ProductionId>>,
    candidate_productions: Vec<Vec<Option<ProductionId>>>,
    equal_cost_alternatives: Vec<Vec<usize>>,
    tied_alternatives: Vec<Vec<usize>>,
    reasons: Vec<Option<SelectionReason>>,
}

impl BestParse {
    pub(crate) fn alternative(&self, node: NodeId) -> Option<usize> {
        self.alternatives.get(node.index()).copied().flatten()
    }

    pub(crate) fn node_cost(&self, node: NodeId) -> Option<ParseCost> {
        self.costs.get(node.index()).copied().flatten()
    }

    pub(crate) fn selected_production(&self, node: NodeId) -> Option<ProductionId> {
        self.selected_productions
            .get(node.index())
            .copied()
            .flatten()
    }

    pub(crate) fn decision_production(&self, node: NodeId) -> Option<ProductionId> {
        self.decision_productions
            .get(node.index())
            .copied()
            .flatten()
    }

    pub(crate) fn candidate_production(
        &self,
        node: NodeId,
        alternative: usize,
    ) -> Option<ProductionId> {
        self.candidate_productions
            .get(node.index())?
            .get(alternative)
            .copied()
            .flatten()
    }

    pub(crate) fn equal_cost_alternatives(&self, node: NodeId) -> &[usize] {
        self.equal_cost_alternatives
            .get(node.index())
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn tied_alternatives(&self, node: NodeId) -> &[usize] {
        self.tied_alternatives
            .get(node.index())
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn reason(&self, node: NodeId) -> Option<SelectionReason> {
        self.reasons.get(node.index()).copied().flatten()
    }
}

/// A per-node alternative choice consulted by lowering. [`BestParse`] is the
/// production selection; the law harness substitutes explicit `ChoiceMap`s
/// so one lowering serves both.
pub(crate) trait AlternativeSelection {
    fn alternative(&self, node: NodeId) -> Option<usize>;
}

impl AlternativeSelection for BestParse {
    fn alternative(&self, node: NodeId) -> Option<usize> {
        self.alternatives.get(node.index()).copied().flatten()
    }
}

/// An explicit per-node alternative assignment: one enumerated derivation.
/// `None` marks nodes the derivation never reaches.
#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChoiceMap {
    alternatives: Vec<Option<usize>>,
}

#[cfg(test)]
impl ChoiceMap {
    /// The production selection as an explicit map — the equivalence bridge
    /// between best-parse lowering and harness lowering.
    pub(crate) fn from_best(best: &BestParse) -> Self {
        Self {
            alternatives: best.alternatives.clone(),
        }
    }
}

#[cfg(test)]
impl AlternativeSelection for ChoiceMap {
    fn alternative(&self, node: NodeId) -> Option<usize> {
        self.alternatives.get(node.index()).copied().flatten()
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectionEnumerationError {
    /// The forest reaches a cycle from this root. Conservative: the check is
    /// selection-independent (any reachable cycle refuses enumeration, even
    /// one no complete selection would enter) — sufficient for law fixtures,
    /// which are acyclic by construction, and it keeps recursive consumers
    /// (`lower`) safe from unbounded derivations.
    Cycle(NodeId),
    BudgetExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ForestError {
    Cycle(NodeId),
    MissingAlternative(NodeId),
}

#[cfg(test)]
mod tests {
    use super::AlternativeSelection;
    use super::NodeKey;
    use super::PackedAlternative;
    use super::ParseCost;
    use super::ParseCostDimension;
    use super::ParseForest;
    use super::SelectionReason;
    use crate::chart::RuleId;
    use crate::construction::ConstructionBackend;
    use crate::construction::ConstructionEvidence;
    use crate::construction::ConstructionFamily;
    use crate::construction::ConstructionId;
    use crate::construction::ConstructionRegistry;
    use crate::construction::DominanceEdge;
    use crate::construction::ProductionId;
    use crate::features::Comma;

    fn production(construction: &'static str, ordinal: u16) -> ProductionId {
        ProductionId {
            construction: ConstructionId::new(construction),
            ordinal,
        }
    }

    fn test_family(id: ConstructionId) -> ConstructionFamily {
        ConstructionFamily::new(
            id,
            ConstructionBackend::Chart,
            ConstructionEvidence::structural("test production"),
        )
    }

    fn test_registry(
        ids: &[ConstructionId],
        edges: impl IntoIterator<Item = DominanceEdge>,
    ) -> ConstructionRegistry {
        ConstructionRegistry::new(ids.iter().copied().map(test_family), edges)
            .expect("test registry must be valid")
    }

    fn empty_registry() -> ConstructionRegistry {
        test_registry(&[], [])
    }

    #[test]
    fn surface_witnesses_survive_packing_into_distinct_exact_results() {
        let mut forest = ParseForest::<&str, (), (), &str, Comma>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());
        let absent = forest.intern_node(
            key.clone(),
            PackedAlternative {
                rule: Some(RuleId::new(0)),
                production: None,
                children: Vec::new(),
                local_cost: ParseCost::default(),
                surface: Comma::Absent,
            },
        );
        let present = forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(0)),
                production: None,
                children: Vec::new(),
                local_cost: ParseCost::default(),
                surface: Comma::Present,
            },
        );
        assert_eq!(absent.node, present.node, "chart node is shared");
        assert_eq!(forest.node(absent.node).alternatives.len(), 2);
        assert_ne!(
            forest.exact_result(absent.node, 0, "same ast").unwrap(),
            forest.exact_result(absent.node, 1, "same ast").unwrap(),
        );
    }

    #[test]
    fn attachment_preferences_are_lexicographic() {
        let unlicensed = ParseCost::default();
        let farther_licensed = ParseCost {
            attachment_count: 1,
            attachment_distance: 4,
            ..ParseCost::default()
        };
        let nearer_licensed = ParseCost {
            attachment_count: 1,
            attachment_distance: 2,
            ..ParseCost::default()
        };
        let nearer_and_more_complete = ParseCost {
            attachment_count: 1,
            attachment_distance: 2,
            attachment_extent: 5,
            ..ParseCost::default()
        };
        assert!(farther_licensed < unlicensed);
        assert!(nearer_licensed < farther_licensed);
        assert!(nearer_and_more_complete < nearer_licensed);
    }

    #[test]
    fn equal_nodes_pack_alternatives_and_choose_the_lower_cost() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());

        let first = forest.intern_node(
            key.clone(),
            PackedAlternative {
                rule: Some(RuleId::new(4)),
                production: None,
                children: Vec::new(),
                local_cost: ParseCost {
                    precedence: 2,
                    ..ParseCost::default()
                },
                surface: (),
            },
        );
        let second = forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(3)),
                production: None,
                children: Vec::new(),
                local_cost: ParseCost {
                    precedence: 1,
                    ..ParseCost::default()
                },
                surface: (),
            },
        );

        assert_eq!(first.node, second.node);
        assert!(first.node_was_new);
        assert!(!second.node_was_new);
        assert_eq!(forest.node(first.node).alternatives.len(), 2);

        let (_, best) = forest
            .best_root([first.node], &empty_registry())
            .expect("the node is acyclic")
            .expect("there is a root");
        assert_eq!(best.cost.precedence, 1);
        assert_eq!(best.alternative(first.node), Some(1));
    }

    #[test]
    fn equal_cost_uses_stable_production_identity_and_preserves_incomparable_ties() {
        let alpha = production("alpha", 0);
        let zeta = production("zeta", 0);
        let registry = test_registry(&[alpha.construction, zeta.construction], []);

        for order in [[zeta, alpha], [alpha, zeta]] {
            let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
            let key = NodeKey::nonterminal("expression", 0, 1, ());
            let root = forest
                .intern_node(
                    key.clone(),
                    PackedAlternative {
                        rule: Some(RuleId::new(4)),
                        production: Some(order[0]),
                        children: Vec::new(),
                        local_cost: ParseCost::default(),
                        surface: (),
                    },
                )
                .node;
            forest.intern_node(
                key,
                PackedAlternative {
                    rule: Some(RuleId::new(3)),
                    production: Some(order[1]),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                    surface: (),
                },
            );

            let (_, best) = forest
                .best_root([root], &registry)
                .expect("the node is acyclic")
                .expect("there is a root");
            assert_eq!(best.selected_production(root), Some(alpha));
            let viable = best
                .tied_alternatives(root)
                .iter()
                .map(|&index| forest.node(root).alternatives[index].production.unwrap())
                .collect::<Vec<_>>();
            assert_eq!(viable, [alpha, zeta]);
            assert_eq!(best.reason(root), Some(SelectionReason::StableIdentity));
        }
    }

    #[test]
    fn declared_dominance_removes_only_the_subordinate_from_viable_ties() {
        let dominant = production("generic", 0);
        let subordinate = production("specific", 0);
        let registry = test_registry(
            &[dominant.construction, subordinate.construction],
            [DominanceEdge::new(
                dominant.construction,
                subordinate.construction,
            )],
        );
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());
        let root = forest
            .intern_node(
                key.clone(),
                PackedAlternative {
                    rule: Some(RuleId::new(1)),
                    production: Some(subordinate),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                    surface: (),
                },
            )
            .node;
        forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(9)),
                production: Some(dominant),
                children: Vec::new(),
                local_cost: ParseCost::default(),
                surface: (),
            },
        );

        let (_, best) = forest
            .best_root([root], &registry)
            .expect("the node is acyclic")
            .expect("there is a root");
        assert_eq!(best.tied_alternatives(root), &[1]);
        assert_eq!(best.equal_cost_alternatives(root), &[0, 1]);
        assert_eq!(best.reason(root), Some(SelectionReason::Dominance));
    }

    #[test]
    fn a_named_cost_dimension_remains_decisive_before_dominance() {
        let dominant = production("generic", 0);
        let subordinate = production("specific", 0);
        let registry = test_registry(
            &[dominant.construction, subordinate.construction],
            [DominanceEdge::new(
                dominant.construction,
                subordinate.construction,
            )],
        );
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());
        let root = forest
            .intern_node(
                key.clone(),
                PackedAlternative {
                    rule: Some(RuleId::new(1)),
                    production: Some(dominant),
                    children: Vec::new(),
                    local_cost: ParseCost {
                        precedence: 1,
                        ..ParseCost::default()
                    },
                    surface: (),
                },
            )
            .node;
        forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(9)),
                production: Some(subordinate),
                children: Vec::new(),
                local_cost: ParseCost::default(),
                surface: (),
            },
        );

        let (_, best) = forest
            .best_root([root], &registry)
            .expect("the node is acyclic")
            .expect("there is a root");
        assert_eq!(best.alternative(root), Some(1));
        assert_eq!(best.node_cost(root), Some(ParseCost::default()));
        assert_eq!(
            best.reason(root),
            Some(SelectionReason::Cost(ParseCostDimension::Precedence))
        );
    }

    #[test]
    fn best_root_uses_cost_then_stable_production_identity() {
        let mut forest = ParseForest::<&str, (), &str, &str, ()>::new();
        let zeta = production("zeta_root", 0);
        let alpha = production("alpha_root", 0);
        let registry = test_registry(&[alpha.construction, zeta.construction], []);
        let first_node = forest
            .intern_node(
                NodeKey::nonterminal("sentence", 0, 1, "first"),
                PackedAlternative {
                    rule: Some(RuleId::new(1)),
                    production: Some(zeta),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                    surface: (),
                },
            )
            .node;
        let second_node = forest
            .intern_node(
                NodeKey::nonterminal("sentence", 0, 1, "second"),
                PackedAlternative {
                    rule: Some(RuleId::new(2)),
                    production: Some(alpha),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                    surface: (),
                },
            )
            .node;

        let (selected, _) = forest
            .best_root([first_node, second_node], &registry)
            .expect("the nodes are acyclic")
            .expect("there are roots");

        assert_eq!(selected, second_node);
    }

    fn leaf_alternative(rule: usize) -> PackedAlternative<()> {
        PackedAlternative {
            rule: Some(RuleId::new(rule)),
            production: None,
            children: Vec::new(),
            local_cost: ParseCost::default(),
            surface: (),
        }
    }

    fn parent_alternative(rule: usize, children: Vec<super::NodeId>) -> PackedAlternative<()> {
        PackedAlternative {
            children,
            ..leaf_alternative(rule)
        }
    }

    #[test]
    fn enumeration_multiplies_independent_choices() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let x = forest
            .intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(1))
            .node;
        forest.intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(2));
        let y = forest
            .intern_node(NodeKey::nonterminal("y", 1, 2, ()), leaf_alternative(3))
            .node;
        forest.intern_node(NodeKey::nonterminal("y", 1, 2, ()), leaf_alternative(4));
        let root = forest
            .intern_node(
                NodeKey::nonterminal("root", 0, 2, ()),
                parent_alternative(5, vec![x, y]),
            )
            .node;
        let mut remaining = usize::MAX;
        let selections = forest
            .enumerate_selections(root, &mut remaining)
            .expect("acyclic forest enumerates");
        assert_eq!(selections.len(), 4, "2 x-choices times 2 y-choices");
        for selection in &selections {
            assert_eq!(selection.alternative(root), Some(0));
            assert!(selection.alternative(x).is_some());
            assert!(selection.alternative(y).is_some());
        }
    }

    #[test]
    fn shared_nodes_choose_once_per_selection() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let x = forest
            .intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(1))
            .node;
        forest.intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(2));
        let root = forest
            .intern_node(
                NodeKey::nonterminal("root", 0, 2, ()),
                parent_alternative(5, vec![x, x]),
            )
            .node;
        let mut remaining = usize::MAX;
        let selections = forest
            .enumerate_selections(root, &mut remaining)
            .expect("acyclic forest enumerates");
        // A per-node map binds both occurrences to the same choice: 2, not 4.
        assert_eq!(selections.len(), 2);
    }

    #[test]
    fn unreached_nodes_stay_unchosen() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let x = forest
            .intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(1))
            .node;
        let y = forest
            .intern_node(NodeKey::nonterminal("y", 0, 1, ()), leaf_alternative(2))
            .node;
        let root = forest
            .intern_node(
                NodeKey::nonterminal("root", 0, 1, ()),
                parent_alternative(3, vec![x]),
            )
            .node;
        forest.intern_node(
            NodeKey::nonterminal("root", 0, 1, ()),
            parent_alternative(4, vec![y]),
        );
        let mut remaining = usize::MAX;
        let selections = forest
            .enumerate_selections(root, &mut remaining)
            .expect("acyclic forest enumerates");
        assert_eq!(selections.len(), 2);
        let via_x = selections
            .iter()
            .find(|selection| selection.alternative(root) == Some(0))
            .expect("the x-branch selection exists");
        assert_eq!(via_x.alternative(x), Some(0));
        assert_eq!(
            via_x.alternative(y),
            None,
            "y is not part of the x-branch derivation"
        );
    }

    #[test]
    fn exhausting_the_budget_is_a_loud_error() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let x = forest
            .intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(1))
            .node;
        forest.intern_node(NodeKey::nonterminal("x", 0, 1, ()), leaf_alternative(2));
        let y = forest
            .intern_node(NodeKey::nonterminal("y", 1, 2, ()), leaf_alternative(3))
            .node;
        forest.intern_node(NodeKey::nonterminal("y", 1, 2, ()), leaf_alternative(4));
        let root = forest
            .intern_node(
                NodeKey::nonterminal("root", 0, 2, ()),
                parent_alternative(5, vec![x, y]),
            )
            .node;
        let mut remaining = 3;
        assert_eq!(
            forest.enumerate_selections(root, &mut remaining),
            Err(super::SelectionEnumerationError::BudgetExhausted),
            "4 selections cannot fit a budget of 3"
        );
    }

    #[test]
    fn a_reachable_cycle_is_refused() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let node = forest
            .intern_node(NodeKey::nonterminal("loop", 0, 1, ()), leaf_alternative(1))
            .node;
        forest.intern_node(
            NodeKey::nonterminal("loop", 0, 1, ()),
            parent_alternative(2, vec![node]),
        );
        let mut remaining = usize::MAX;
        assert_eq!(
            forest.enumerate_selections(node, &mut remaining),
            Err(super::SelectionEnumerationError::Cycle(node)),
        );
    }

    #[test]
    fn choice_map_mirrors_best_parse_selections() {
        let mut forest = ParseForest::<&str, (), (), &str, ()>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());
        let root = forest
            .intern_node(
                key.clone(),
                PackedAlternative {
                    rule: Some(RuleId::new(4)),
                    production: None,
                    children: Vec::new(),
                    local_cost: ParseCost {
                        precedence: 2,
                        ..ParseCost::default()
                    },
                    surface: (),
                },
            )
            .node;
        forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(3)),
                production: None,
                children: Vec::new(),
                local_cost: ParseCost {
                    precedence: 1,
                    ..ParseCost::default()
                },
                surface: (),
            },
        );
        let (_, best) = forest
            .best_root([root], &empty_registry())
            .expect("the node is acyclic")
            .expect("there is a root");
        let map = super::ChoiceMap::from_best(&best);
        assert_eq!(map.alternative(root), best.alternative(root));
        assert_eq!(map.alternative(root), Some(1));
    }
}
