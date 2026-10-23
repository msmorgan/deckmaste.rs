//! Reading Normal Form: identifies only the reviewed structural alternatives
//! as one analysis. It never endorses a Reading; it only lets two sets of
//! Readings be compared under exactly the approved canonicalizations.

use deckmaste_lexical::LexicalReading;
use dpsi::Ident;

use crate::compile::CompiledGrammar;
use crate::compile::frame_name;
use crate::frames::FrameSlot;
use crate::frames::SlotFill;
use crate::frames::SlotRelation;
use crate::grammar::HostKind;
use crate::reading::Reading;
use crate::reading::ReadingNode;

impl CompiledGrammar {
    /// The Reading Normal Form of `reading` under the approved
    /// canonicalizations this engine derives:
    ///
    /// - the grouping within a chain joined by the same coordinator: every
    ///   grouping, binary or serial, becomes one node listing the conjuncts and
    ///   then the coordinator, without features of its own;
    /// - clause versus verb-phrase coordination of bare imperatives: a
    ///   Production whose child is `uncoordinated` (where the coordination one
    ///   level up is the same analysis) over a coordination of that child
    ///   becomes the coordination of the Production over each conjunct;
    /// - adjuncts below a non-modal auxiliary: a post-verbal Adjunct of an
    ///   uncoordinated auxiliary Verb Phrase becomes an Adjunct of the lexical
    ///   participial Verb Phrase that is its Complement;
    /// - same-head complement priority: a Preposition Phrase attached as a
    ///   Predicate adjunct to a head that has a longer frame selecting it
    ///   becomes that frame's complement;
    /// - fronted dependent outermost: a comma-set post-dependent of a Clause
    ///   that begins with a fronted dependent becomes a post-dependent of that
    ///   Clause's head, *Pre[,] [Clause, Post]* for *[Pre[,] Clause], Post*.
    #[must_use]
    pub fn normal_form(&self, reading: &Reading) -> Reading {
        self.normalize(&self.lower_adjuncts(reading))
    }

    fn normalize(&self, reading: &Reading) -> Reading {
        let Reading::Node(node) = reading else {
            return reading.clone();
        };
        let children: Vec<Reading> = node.children.iter().map(|c| self.normalize(c)).collect();
        let production = self
            .by_id
            .get(&node.production)
            .map(|&p| &self.productions[p]);
        if production.is_some_and(|p| p.coordinator.is_some()) {
            return Self::flat_coordination(node, children);
        }
        // A Reading stores no punctuation: a Production position becomes a
        // child index by skipping the punctuation before it.
        let child = |p: &crate::step::Production, position: usize| {
            p.rhs[..position]
                .iter()
                .filter(|s| !matches!(s, crate::step::Symbol::Punct(..)))
                .count()
        };
        if let Some(p) = production
            && let Some(&at) = p.uncoordinated.first()
            && let at = child(p, at)
            && let Some(Reading::Node(coordination)) = children.get(at)
            && coordination.production.starts_with("Coordination:")
            && !coordination.production.contains('/')
        {
            return self.distribute(node, &children, at, p.lhs);
        }
        if let Some(p) = production
            && p.hosts_comma_set_dependent()
            && let Some(outermost) = self.front_outermost(node, &children)
        {
            return outermost;
        }
        if let Some(p) = production
            && let Some((host, adjunct, HostKind::Predicate)) = p.host_check
            && adjunct > host
            && let Some(complement) =
                self.as_complement(&children[child(p, host)], &children[child(p, adjunct)])
        {
            return complement;
        }
        Reading::Node(ReadingNode {
            children,
            ..node.clone()
        })
    }

    /// `node`, the host of a comma-set post-dependent over normalized
    /// `children`, as the fronted dependents its first child begins with over
    /// `node` on that child's head, if it begins with any.
    fn front_outermost(&self, node: &ReadingNode, children: &[Reading]) -> Option<Reading> {
        let Some(Reading::Node(front)) = children.first() else {
            return None;
        };
        if !self.productions[*self.by_id.get(&front.production)?].fronts() {
            return None;
        }
        let mut hosted = children.to_vec();
        hosted[0] = front.children.last()?.clone();
        let inner = self.front_outermost(node, &hosted).unwrap_or_else(|| {
            Reading::Node(ReadingNode {
                children: hosted,
                ..node.clone()
            })
        });
        let mut outer = front.children.clone();
        *outer.last_mut()? = inner;
        Some(Reading::Node(ReadingNode {
            children: outer,
            ..front.clone()
        }))
    }

    fn flat_coordination(node: &ReadingNode, children: Vec<Reading>) -> Reading {
        let category = Ident::new(base(&node.category));
        let id = flat_id(&category);
        // The chain's coordinator: its own leaf, or the one its serial tail
        // carries.
        let own = children.iter().rev().find_map(|child| match child {
            Reading::Leaf(leaf) => Some(leaf.clone()),
            Reading::Node(inner) if inner.production == id => split(inner).1,
            Reading::Node(_) => None,
        });
        let same = |other: Option<&LexicalReading>| match (&own, other) {
            (Some(LexicalReading::Word(a)), Some(LexicalReading::Word(b))) => a.lexeme == b.lexeme,
            _ => false,
        };
        let mut conjuncts = Vec::new();
        for child in children {
            match child {
                Reading::Leaf(_) => {}
                Reading::Node(inner)
                    if inner.production == id && same(split(&inner).1.as_ref()) =>
                {
                    conjuncts.extend(split(&inner).0);
                }
                other @ Reading::Node(_) => conjuncts.push(other),
            }
        }
        conjuncts.extend(own.map(Reading::Leaf));
        // A coordination's features follow from its conjuncts and
        // coordinator, so the one node carries none of its own.
        Reading::Node(ReadingNode {
            production: id,
            category,
            children: conjuncts,
            selected_frame: None,
            features: std::collections::BTreeMap::new(),
            ..node.clone()
        })
    }

    /// `node` over the flat coordination at child `at`, as the coordination
    /// of `node` over each conjunct (the coordinator stays last).
    fn distribute(
        &self,
        node: &ReadingNode,
        children: &[Reading],
        at: usize,
        lhs: usize,
    ) -> Reading {
        let Reading::Node(coordination) = &children[at] else {
            unreachable!("a coordination child")
        };
        let mut conjuncts: Vec<Reading> = Vec::new();
        let mut coordinator = None;
        for conjunct in &coordination.children {
            match conjunct {
                Reading::Leaf(leaf) => coordinator = Some(Reading::Leaf(leaf.clone())),
                Reading::Node(inner) => {
                    let mut own = children.to_vec();
                    own[at] = conjunct.clone();
                    // The mother takes the features it shares with this
                    // conjunct from the conjunct, as it would from its head.
                    let mut features = node.features.clone();
                    for &feature in &self.category_features[lhs] {
                        let name = self.features[feature];
                        if let Some(value) = inner.features.get(&name) {
                            features.insert(name, *value);
                        }
                    }
                    conjuncts.push(self.normalize(&Reading::Node(ReadingNode {
                        children: own,
                        features,
                        ..node.clone()
                    })));
                }
            }
        }
        let category = Ident::new(base(&node.category));
        let flat = Reading::Node(ReadingNode {
            production: flat_id(&category),
            category,
            children: conjuncts.into_iter().chain(coordinator).collect(),
            selected_frame: None,
            ..node.clone()
        });
        // A coordination of coordinations by one coordinator is one chain.
        let Reading::Node(flat_node) = &flat else { unreachable!() };
        Self::flat_coordination(flat_node, flat_node.children.clone())
    }

    /// Adjuncts below a non-modal auxiliary, on the Reading as built: every
    /// Adjunct Production over an uncoordinated auxiliary Verb Phrase becomes
    /// the auxiliary over that Production of its participial Complement's
    /// Verb Phrase, which the Production takes its features from.
    fn lower_adjuncts(&self, reading: &Reading) -> Reading {
        let Reading::Node(node) = reading else {
            return reading.clone();
        };
        let children = node
            .children
            .iter()
            .map(|c| self.lower_adjuncts(c))
            .collect();
        self.lower(node, children)
    }

    /// `node` over its lowered `children`, lowered if it is an Adjunct
    /// Production over an auxiliary Verb Phrase.
    fn lower(&self, node: &ReadingNode, children: Vec<Reading>) -> Reading {
        let lowered = self
            .by_id
            .get(&node.production)
            .filter(|&&p| self.productions[p].post_adjunct_host(0))
            .and_then(|_| self.below_auxiliary(node, &children));
        lowered.unwrap_or_else(|| {
            Reading::Node(ReadingNode {
                children,
                ..node.clone()
            })
        })
    }

    /// `node`, an Adjunct Production over the auxiliary Verb Phrase that is
    /// its first child, as the auxiliary over that Production of its
    /// participial Complement's head Verb Phrase.
    fn below_auxiliary(&self, node: &ReadingNode, children: &[Reading]) -> Option<Reading> {
        let Some(Reading::Node(auxiliary)) = children.first() else {
            return None;
        };
        let p = &self.productions[*self.by_id.get(&auxiliary.production)?];
        p.auxiliary?;
        let Some(Reading::Node(complement)) = auxiliary.children.last() else {
            return None;
        };
        let c = &self.productions[*self.by_id.get(&complement.production)?];
        let head = c.rhs[..c.head?]
            .iter()
            .filter(|s| !matches!(s, crate::step::Symbol::Punct(..)))
            .count();
        let Some(Reading::Node(lexical)) = complement.children.get(head) else {
            return None;
        };
        if base(&lexical.category) != base(&node.category) {
            return None;
        }
        // The Production over the lexical Verb Phrase takes its features from
        // that head; the Gap passes through the auxiliary and its Complement
        // unchanged.
        let mut own = children.to_vec();
        own[0] = Reading::Node(lexical.clone());
        let lowered = self.lower(
            &ReadingNode {
                features: lexical.features.clone(),
                ..node.clone()
            },
            own,
        );
        let mut inner = complement.children.clone();
        inner[head] = lowered;
        let mut outer = auxiliary.children.clone();
        *outer.last_mut()? = Reading::Node(ReadingNode {
            children: inner,
            gap: node.gap,
            ..complement.clone()
        });
        Some(Reading::Node(ReadingNode {
            children: outer,
            gap: node.gap,
            ..auxiliary.clone()
        }))
    }

    /// The complement analysis equivalent to attaching `adjunct` to `host`,
    /// if `host`'s head has a frame selecting it.
    fn as_complement(&self, host: &Reading, adjunct: &Reading) -> Option<Reading> {
        let (Reading::Node(host), Reading::Node(phrase)) = (host, adjunct) else {
            return None;
        };
        let frame = host.selected_frame.as_ref()?;
        let suffix = host
            .production
            .strip_prefix(&frame_name(&host.category, frame))?;
        if suffix.contains("+by") {
            return None;
        }
        let Some(Reading::Leaf(LexicalReading::Word(head))) = host.children.first() else {
            return None;
        };
        let Some(Reading::Leaf(LexicalReading::Word(preposition))) = phrase.children.first() else {
            return None;
        };
        let facts = self.lexemes.get(&head.lexeme)?;
        for &candidate in &facts.frames {
            let longer = &self.frames[candidate];
            if longer.slots.len() == frame.slots.len() + 1
                && longer.slots[..frame.slots.len()] == frame.slots[..]
                && let Some(FrameSlot {
                    relation: SlotRelation::Complement,
                    fill: SlotFill::PP(list),
                }) = longer.slots.last()
                && list.contains(&preposition.lexeme)
            {
                let id = format!("{}{suffix}", frame_name(&host.category, longer));
                let (&id, _) = self.by_id.get_key_value(id.as_str())?;
                let mut children = host.children.clone();
                children.push(Reading::Node(phrase.clone()));
                return Some(Reading::Node(ReadingNode {
                    production: id,
                    selected_frame: Some(longer.clone()),
                    children,
                    ..host.clone()
                }));
            }
        }
        None
    }
}

fn base(category: &str) -> &str {
    category.split_once('#').map_or(category, |(base, _)| base)
}

fn flat_id(category: &str) -> Ident {
    Ident::new(&format!("Coordination:{}", base(category)))
}

fn split(node: &ReadingNode) -> (Vec<Reading>, Option<LexicalReading>) {
    let mut conjuncts = node.children.clone();
    let coordinator = match conjuncts.last() {
        Some(Reading::Leaf(leaf)) => Some(leaf.clone()),
        _ => None,
    };
    if coordinator.is_some() {
        conjuncts.pop();
    }
    (conjuncts, coordinator)
}
