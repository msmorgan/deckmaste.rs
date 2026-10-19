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
    ///   then the coordinator;
    /// - same-head complement priority: a Preposition Phrase attached as a
    ///   Predicate adjunct to a head that has a longer frame selecting it
    ///   becomes that frame's complement.
    #[must_use]
    pub fn normal_form(&self, reading: &Reading) -> Reading {
        let Reading::Node(node) = reading else {
            return reading.clone();
        };
        let children: Vec<Reading> = node.children.iter().map(|c| self.normal_form(c)).collect();
        let production = self
            .by_id
            .get(&node.production)
            .map(|&p| &self.productions[p]);
        if production.is_some_and(|p| p.coordinator.is_some()) {
            return Self::flat_coordination(node, children);
        }
        if let Some(p) = production
            && let Some((host, adjunct, HostKind::Predicate)) = p.host_check
            && let Some(complement) = self.as_complement(&children[host], &children[adjunct])
        {
            return complement;
        }
        Reading::Node(ReadingNode {
            children,
            ..node.clone()
        })
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
        Reading::Node(ReadingNode {
            production: id,
            category,
            children: conjuncts,
            selected_frame: None,
            ..node.clone()
        })
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
