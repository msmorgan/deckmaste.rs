use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Range;

use crate::CompletedId;
use crate::Forest;
use crate::Materializer;
use crate::SegmentationError;
use crate::forest::Child;
use crate::forest::Edge;
use crate::forest::IntermediateId;

/// A contextual enumeration boundary, with all its admitted summary
/// alternatives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationUnit<C> {
    pub category: C,
    pub span: Range<usize>,
    pub nodes: BTreeSet<CompletedId>,
    pub retained_reason: Option<&'static str>,
    pub proposed_parts: Vec<(C, Range<usize>)>,
}

#[derive(Debug, Clone)]
enum Context<C> {
    Unit(EnumerationUnit<C>),
    Leaf(usize),
    Build {
        node: CompletedId,
        nodes: BTreeSet<CompletedId>,
        family: usize,
        children: Vec<Self>,
    },
}

/// Fixed surrounding constructors plus independently replaceable units.
#[derive(Debug, Clone)]
pub struct Decomposition<C> {
    context: Context<C>,
    units: Vec<EnumerationUnit<C>>,
}

impl<C> Decomposition<C> {
    #[must_use]
    pub fn units(&self) -> &[EnumerationUnit<C>] {
        &self.units
    }
}

impl<C: Clone + Ord, S, T> Forest<C, S, T> {
    /// Prove that the contextual node alternatives form a Cartesian product
    /// under one fixed constructor/leaf structure. Equal spans alone never
    /// authorize a split. An unproved region becomes a single residual unit.
    ///
    /// # Errors
    /// A cycle outside a stopping boundary is an enumeration error.
    pub fn decompose(
        &self,
        nodes: &[CompletedId],
        descend: &impl Fn(&C, &[usize]) -> bool,
        retain: &impl Fn(&C) -> bool,
    ) -> Result<Option<Decomposition<C>>, SegmentationError> {
        if nodes.is_empty() {
            return Ok(None);
        }
        let mut walk = ContextWalk {
            forest: self,
            descend,
            retain,
            cache: BTreeMap::new(),
            active: BTreeSet::new(),
        };
        let mut alternatives = Vec::new();
        for &node in nodes {
            alternatives.push(walk.completed(node)?);
        }
        let context = walk.merge(nodes, &alternatives);
        let mut units = Vec::new();
        context.units(&mut units);
        Ok(Some(Decomposition { context, units }))
    }

    /// Contextual subregions and their ranges in the outer ordered unit list.
    #[must_use]
    pub fn regions(
        &self,
        decomposition: &Decomposition<C>,
        category: &C,
    ) -> Vec<(Vec<CompletedId>, Decomposition<C>, Range<usize>)> {
        fn walk<C: Clone + Ord, S, T>(
            forest: &Forest<C, S, T>,
            context: &Context<C>,
            category: &C,
            offset: &mut usize,
            output: &mut Vec<(Vec<CompletedId>, Decomposition<C>, Range<usize>)>,
        ) {
            let start = *offset;
            match context {
                Context::Unit(_) => *offset += 1,
                Context::Leaf(_) => {}
                Context::Build {
                    node,
                    nodes,
                    children,
                    ..
                } => {
                    for child in children {
                        walk(forest, child, category, offset, output);
                    }
                    if &forest.completed[node.0].category == category {
                        let mut units = Vec::new();
                        context.units(&mut units);
                        output.push((
                            nodes.iter().copied().collect(),
                            Decomposition {
                                context: context.clone(),
                                units,
                            },
                            start..*offset,
                        ));
                    }
                }
            }
        }
        let mut result = Vec::new();
        walk(self, &decomposition.context, category, &mut 0, &mut result);
        result
    }

    /// Half-open chart span of a completed constituent.
    #[must_use]
    pub fn span(&self, node: CompletedId) -> Range<usize> {
        let node = &self.completed[node.0];
        node.start..node.end
    }

    /// Fill a proven context in surface order. The original chart summaries
    /// and completion states remain available to the materializer.
    ///
    /// # Errors
    /// Forwards materialization failures. The caller supplies exactly one value
    /// for every unit, in the order returned by `units`. Generated grammar
    /// materialization is independent of the stored Summary and State.
    ///
    /// # Panics
    /// Panics when the supplied value count differs from the unit count.
    pub fn rebuild<M: Materializer<S, T>>(
        &self,
        decomposition: &Decomposition<C>,
        values: &[M::Reading],
        materializer: &mut M,
    ) -> Result<M::Reading, M::Error> {
        assert_eq!(values.len(), decomposition.units.len());
        self.fill(&decomposition.context, &mut values.iter(), materializer)
    }

    /// Encode fixed lexical values, literals, constructor identities and unit
    /// boundaries, without chart node identities or representative unit trees.
    /// `constructor` identifies a grammatical value's constructor, Category
    /// and form; it returns `None` for transparent compiler helper values.
    ///
    /// # Errors
    /// Forwards materialization failures.
    ///
    /// # Panics
    /// Panics when the supplied value count differs from the unit count.
    pub fn context_encoding<M: Materializer<S, T>>(
        &self,
        decomposition: &Decomposition<C>,
        values: &[M::Reading],
        materializer: &mut M,
        constructor: &impl Fn(&M::Reading) -> Option<String>,
    ) -> Result<String, M::Error>
    where
        C: std::fmt::Debug,
        M::Reading: std::fmt::Debug,
    {
        fn encode<'a, C: std::fmt::Debug, S, T, M: Materializer<S, T>>(
            forest: &Forest<C, S, T>,
            context: &Context<C>,
            values: &mut std::slice::Iter<'a, M::Reading>,
            materializer: &mut M,
            constructor: &impl Fn(&M::Reading) -> Option<String>,
        ) -> Result<(M::Reading, Vec<String>), M::Error>
        where
            M::Reading: std::fmt::Debug + 'a,
        {
            match context {
                Context::Unit(unit) => Ok((
                    values.next().expect("one value per unit").clone(),
                    vec![format!(
                        "unit({:?},{}..{})",
                        unit.category, unit.span.start, unit.span.end
                    )],
                )),
                Context::Leaf(id) => {
                    let leaf = &forest.leaves[*id];
                    let value = materializer.leaf(leaf, forest.leaf_summaries[*id].as_ref())?;
                    let key = match leaf {
                        crate::Leaf::Literal { text, .. } => format!("literal({text:?})"),
                        crate::Leaf::Lexical { .. } => format!("lexical({value:?})"),
                    };
                    Ok((value, vec![key]))
                }
                Context::Build {
                    node,
                    family,
                    children,
                    ..
                } => {
                    let completed = &forest.completed[node.0];
                    let family = completed.families[*family];
                    let mut child_values = Vec::new();
                    let mut encoding = Vec::new();
                    for child in children {
                        let (value, keys) =
                            encode(forest, child, values, materializer, constructor)?;
                        child_values.push(value);
                        encoding.extend(keys);
                    }
                    let value = materializer.build(
                        family.production,
                        &completed.summary,
                        &forest.intermediate[family.prefix.0].state,
                        child_values,
                    )?;
                    if let Some(identity) = constructor(&value) {
                        encoding = vec![format!("reading({identity:?},{encoding:?})")];
                    }
                    Ok((value, encoding))
                }
            }
        }
        assert_eq!(values.len(), decomposition.units.len());
        let (_, encoding) = encode(
            self,
            &decomposition.context,
            &mut values.iter(),
            materializer,
            constructor,
        )?;
        Ok(format!("{encoding:?}"))
    }

    fn fill<'a, M: Materializer<S, T>>(
        &self,
        context: &Context<C>,
        values: &mut std::slice::Iter<'a, M::Reading>,
        materializer: &mut M,
    ) -> Result<M::Reading, M::Error>
    where
        M::Reading: 'a,
    {
        match context {
            Context::Unit(_) => Ok(values.next().expect("one value per unit").clone()),
            Context::Leaf(id) => {
                materializer.leaf(&self.leaves[*id], self.leaf_summaries[*id].as_ref())
            }
            Context::Build {
                node,
                family,
                children,
                ..
            } => {
                let completed = &self.completed[node.0];
                let family = completed.families[*family];
                let children = children
                    .iter()
                    .map(|child| self.fill(child, values, materializer))
                    .collect::<Result<Vec<_>, _>>()?;
                materializer.build(
                    family.production,
                    &completed.summary,
                    &self.intermediate[family.prefix.0].state,
                    children,
                )
            }
        }
    }
}

impl<C: Clone> Context<C> {
    fn units(&self, output: &mut Vec<EnumerationUnit<C>>) {
        match self {
            Self::Unit(unit) => output.push(unit.clone()),
            Self::Build { children, .. } => {
                for child in children {
                    child.units(output);
                }
            }
            Self::Leaf(_) => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape<C> {
    Unit(C, usize, usize),
    Leaf(usize),
    Build(usize, Vec<Self>),
}

struct ContextWalk<'a, C, S, T, F, R> {
    forest: &'a Forest<C, S, T>,
    descend: &'a F,
    retain: &'a R,
    cache: BTreeMap<CompletedId, Context<C>>,
    active: BTreeSet<CompletedId>,
}

impl<C: Clone + Ord, S, T, F: Fn(&C, &[usize]) -> bool, R: Fn(&C) -> bool>
    ContextWalk<'_, C, S, T, F, R>
{
    fn whole(
        &self,
        nodes: &[CompletedId],
        reason: Option<&'static str>,
        proposed_parts: Vec<(C, Range<usize>)>,
    ) -> Context<C> {
        let node = &self.forest.completed[nodes[0].0];
        Context::Unit(EnumerationUnit {
            category: node.category.clone(),
            span: node.start..node.end,
            nodes: nodes.iter().copied().collect(),
            retained_reason: reason,
            proposed_parts,
        })
    }

    fn completed(&mut self, id: CompletedId) -> Result<Context<C>, SegmentationError> {
        if let Some(context) = self.cache.get(&id) {
            return Ok(context.clone());
        }
        let node = &self.forest.completed[id.0];
        if !(self.descend)(
            &node.category,
            &node
                .families
                .iter()
                .map(|family| family.production)
                .collect::<Vec<_>>(),
        ) {
            return Ok(self.whole(&[id], None, Vec::new()));
        }
        if !self.active.insert(id) {
            return Err(SegmentationError::Cycle);
        }
        let mut alternatives = Vec::new();
        for (family_index, family) in node.families.iter().enumerate() {
            for children in self.intermediate(family.prefix)? {
                alternatives.push(Context::Build {
                    node: id,
                    nodes: BTreeSet::from([id]),
                    family: family_index,
                    children,
                });
            }
        }
        self.active.remove(&id);
        let mut result = self.merge(&[id], &alternatives);
        let mut units = Vec::new();
        result.units(&mut units);
        if (self.retain)(&node.category)
            && let Some(unproved) = units.iter().find(|unit| !(self.retain)(&unit.category))
        {
            result = self.whole(
                &[id],
                unproved.retained_reason.or(Some("factorization-unproved")),
                unproved.proposed_parts.clone(),
            );
        }
        self.cache.insert(id, result.clone());
        Ok(result)
    }

    fn intermediate(
        &mut self,
        id: IntermediateId,
    ) -> Result<Vec<Vec<Context<C>>>, SegmentationError> {
        let mut alternatives = Vec::new();
        for edge in &self.forest.intermediate[id.0].edges {
            match *edge {
                Edge::Empty => alternatives.push(Vec::new()),
                Edge::Append { prefix, child } => {
                    let child = match child {
                        Child::Completed(id) => self.completed(id)?,
                        Child::Leaf(id) => Context::Leaf(id),
                    };
                    for mut children in self.intermediate(prefix)? {
                        children.push(child.clone());
                        alternatives.push(children);
                    }
                }
            }
        }
        Ok(alternatives)
    }

    fn shape(&self, context: &Context<C>) -> Shape<C> {
        match context {
            Context::Unit(unit) => {
                Shape::Unit(unit.category.clone(), unit.span.start, unit.span.end)
            }
            Context::Leaf(id) => Shape::Leaf(*id),
            Context::Build {
                node,
                family,
                children,
                ..
            } => Shape::Build(
                self.forest.completed[node.0].families[*family].production,
                children.iter().map(|child| self.shape(child)).collect(),
            ),
        }
    }

    fn merge(&self, nodes: &[CompletedId], alternatives: &[Context<C>]) -> Context<C> {
        let Some(first) = alternatives.first() else {
            return self.whole(nodes, Some("factorization-unproved"), Vec::new());
        };
        let mut first_units = Vec::new();
        first.units(&mut first_units);
        let proposed = first_units
            .iter()
            .map(|unit| (unit.category.clone(), unit.span.clone()))
            .collect::<Vec<_>>();
        let shape = self.shape(first);
        let mut union = first_units.clone();
        let mut allowed = BTreeSet::new();
        for alternative in alternatives {
            let mut units = Vec::new();
            alternative.units(&mut units);
            if self.shape(alternative) != shape {
                let boundaries = units
                    .iter()
                    .map(|unit| (unit.category.clone(), unit.span.clone()))
                    .collect::<Vec<_>>();
                return self.whole(
                    nodes,
                    Some(if boundaries == proposed {
                        "alternative-wrapper"
                    } else {
                        "boundary-disagreement"
                    }),
                    proposed,
                );
            }
            for (all, part) in union.iter_mut().zip(&units) {
                all.nodes.extend(&part.nodes);
            }
            allowed.extend(tuples(&units));
        }
        if union
            .windows(2)
            .any(|pair| pair[0].span.end > pair[1].span.start)
        {
            return self.whole(nodes, Some("shared-material"), proposed);
        }
        // Compare the actual permitted tuples, not their cardinality.
        if tuples(&union) != allowed {
            return self.whole(nodes, Some("correlated-choices"), proposed);
        }
        let mut context = first.clone();
        for alternative in &alternatives[1..] {
            merge_context_nodes(&mut context, alternative);
        }
        replace_units(&mut context, &mut union.into_iter());
        context
    }
}

fn merge_context_nodes<C>(target: &mut Context<C>, source: &Context<C>) {
    if let (
        Context::Build {
            nodes: left,
            children: a,
            ..
        },
        Context::Build {
            nodes: right,
            children: b,
            ..
        },
    ) = (target, source)
    {
        left.extend(right);
        for (left, right) in a.iter_mut().zip(b) {
            merge_context_nodes(left, right);
        }
    }
}

fn tuples<C>(units: &[EnumerationUnit<C>]) -> BTreeSet<Vec<CompletedId>> {
    let mut tuples = BTreeSet::from([Vec::new()]);
    for unit in units {
        tuples = tuples
            .into_iter()
            .flat_map(|prefix| {
                unit.nodes.iter().map(move |&node| {
                    let mut tuple = prefix.clone();
                    tuple.push(node);
                    tuple
                })
            })
            .collect();
    }
    tuples
}

fn replace_units<C>(
    context: &mut Context<C>,
    units: &mut impl Iterator<Item = EnumerationUnit<C>>,
) {
    match context {
        Context::Unit(unit) => *unit = units.next().expect("matching unit shapes"),
        Context::Build { children, .. } => {
            for child in children {
                replace_units(child, units);
            }
        }
        Context::Leaf(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forest::Completed;
    use crate::forest::Family;
    use crate::forest::Intermediate;
    type TestForest = Forest<&'static str, u8, ()>;

    fn forest() -> TestForest {
        Forest {
            intermediate: vec![],
            completed: vec![],
            leaves: vec![],
            leaf_summaries: vec![],
            roots: vec![],
            arities: vec![],
            metrics: crate::Metrics::default(),
        }
    }

    fn node(
        forest: &mut TestForest,
        category: &'static str,
        span: Range<usize>,
        summary: u8,
        paths: &[(usize, Vec<CompletedId>)],
    ) -> CompletedId {
        let mut families = Vec::new();
        for (production, children) in paths {
            let mut prefix = IntermediateId(forest.intermediate.len());
            forest.intermediate.push(Intermediate {
                state: (),
                edges: vec![Edge::Empty],
                edge_set: BTreeSet::from([Edge::Empty]),
            });
            for &child in children {
                let edge = Edge::Append {
                    prefix,
                    child: Child::Completed(child),
                };
                prefix = IntermediateId(forest.intermediate.len());
                forest.intermediate.push(Intermediate {
                    state: (),
                    edges: vec![edge],
                    edge_set: BTreeSet::from([edge]),
                });
            }
            families.push(Family {
                production: *production,
                prefix,
            });
        }
        let id = CompletedId(forest.completed.len());
        forest.completed.push(Completed {
            category,
            start: span.start,
            end: span.end,
            summary,
            family_set: families.iter().copied().collect(),
            families,
        });
        id
    }

    #[test]
    fn identical_spans_do_not_factor_correlated_summary_choices() {
        let mut graph = forest();
        let left_plain = node(&mut graph, "Sentence", 0..1, 0, &[]);
        let left_other = node(&mut graph, "Sentence", 0..1, 1, &[]);
        let right_plain = node(&mut graph, "Sentence", 2..3, 0, &[]);
        let right_other = node(&mut graph, "Sentence", 2..3, 1, &[]);
        let root = node(
            &mut graph,
            "Paragraph",
            0..3,
            0,
            &[
                (0, vec![left_plain, right_plain]),
                (0, vec![left_other, right_other]),
            ],
        );
        let plan = graph
            .decompose(&[root], &|category, _| *category != "Sentence", &|_| true)
            .unwrap()
            .unwrap();
        assert_eq!(plan.units().len(), 1);
        assert_eq!(plan.units()[0].category, "Paragraph");
        assert_eq!(plan.units()[0].retained_reason, Some("correlated-choices"));
        assert_eq!(
            plan.units()[0].proposed_parts,
            [("Sentence", 0..1), ("Sentence", 2..3)]
        );
        // Adding both missing tuples, rather than merely matching left_plain
        // count, establishes the independent product.
        let root = node(
            &mut graph,
            "Paragraph",
            0..3,
            0,
            &[
                (0, vec![left_plain, right_plain]),
                (0, vec![left_other, right_other]),
                (0, vec![left_plain, right_other]),
                (0, vec![left_other, right_plain]),
            ],
        );
        let plan = graph
            .decompose(&[root], &|category, _| *category != "Sentence", &|_| true)
            .unwrap()
            .unwrap();
        assert_eq!(
            plan.units()
                .iter()
                .map(|unit| unit.nodes.clone())
                .collect::<Vec<_>>(),
            [
                BTreeSet::from([left_plain, left_other]),
                BTreeSet::from([right_plain, right_other])
            ]
        );
    }

    #[test]
    fn boundary_wrapper_and_shared_material_disagreements_remain_whole() {
        for (paths, reason) in [
            (vec![(0, vec![0, 1]), (0, vec![2])], "boundary-disagreement"),
            (
                vec![(0, vec![0, 1]), (1, vec![0, 1])],
                "alternative-wrapper",
            ),
            (vec![(0, vec![0, 0])], "shared-material"),
        ] {
            let mut graph = forest();
            let ids = [
                node(&mut graph, "Sentence", 0..1, 0, &[]),
                node(&mut graph, "Sentence", 2..3, 0, &[]),
                node(&mut graph, "Sentence", 0..3, 0, &[]),
            ];
            let paths = paths
                .into_iter()
                .map(|(production, children)| {
                    (production, children.into_iter().map(|i| ids[i]).collect())
                })
                .collect::<Vec<_>>();
            let root = node(&mut graph, "Paragraph", 0..3, 0, &paths);
            let plan = graph
                .decompose(&[root], &|category, _| *category != "Sentence", &|_| true)
                .unwrap()
                .unwrap();
            assert_eq!(plan.units().len(), 1);
            assert_eq!(plan.units()[0].retained_reason, Some(reason));
        }
    }

    #[test]
    fn context_digest_tracks_literals_and_wrappers_without_representative_trees() {
        struct Builder;
        impl crate::Materializer<u8, ()> for Builder {
            type Reading = String;
            type Error = std::convert::Infallible;
            fn leaf(&mut self, _: &crate::Leaf, _: Option<&u8>) -> Result<String, Self::Error> {
                Ok("Literal".into())
            }
            fn build(
                &mut self,
                _: usize,
                _: &u8,
                &(): &(),
                children: Vec<String>,
            ) -> Result<String, Self::Error> {
                Ok(format!("Root({children:?})"))
            }
        }
        let mut graph = forest();
        let unit = node(&mut graph, "Sentence", 0..1, 0, &[]);
        let root = node(&mut graph, "Paragraph", 0..2, 0, &[(0, vec![unit])]);
        graph.leaves.push(crate::Leaf::Literal {
            start: 1,
            end: 2,
            text: ".".into(),
        });
        graph.leaf_summaries.push(None);
        let family = &mut graph.completed[root.0].families[0];
        let edge = Edge::Append {
            prefix: family.prefix,
            child: Child::Leaf(0),
        };
        family.prefix = IntermediateId(graph.intermediate.len());
        graph.intermediate.push(Intermediate {
            state: (),
            edges: vec![edge],
            edge_set: BTreeSet::from([edge]),
        });
        let plan = graph
            .decompose(&[root], &|category, _| *category != "Sentence", &|_| true)
            .unwrap()
            .unwrap();
        let encode = |graph: &TestForest, representative: &str, wrapper: &str| {
            graph
                .context_encoding(&plan, &[representative.into()], &mut Builder, &|_| {
                    Some(wrapper.into())
                })
                .unwrap()
        };
        let original = encode(&graph, "first tree", "Root");
        assert_eq!(original, encode(&graph, "different tree", "Root"));
        assert_ne!(original, encode(&graph, "first tree", "OtherWrapper"));
        graph.leaves[0] = crate::Leaf::Literal {
            start: 1,
            end: 2,
            text: "!".into(),
        };
        assert_ne!(original, encode(&graph, "first tree", "Root"));
    }

    #[test]
    fn cycles_outside_units_are_errors_and_no_root_never_decomposes() {
        let mut graph = forest();
        assert!(
            graph
                .decompose(&[], &|_, _| true, &|_| true)
                .unwrap()
                .is_none()
        );
        let root = node(
            &mut graph,
            "Paragraph",
            0..1,
            0,
            &[(0, vec![CompletedId(0)])],
        );
        assert_eq!(
            graph
                .decompose(&[root], &|_, _| true, &|_| true)
                .unwrap_err(),
            SegmentationError::Cycle
        );
    }
}
