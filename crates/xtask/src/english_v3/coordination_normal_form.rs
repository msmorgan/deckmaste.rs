//! The two reviewed coordination equivalences, composed with nominal normalization.
use super::{Result, Tree, ensure};

fn atom(s: &str) -> Tree {
    Tree::Atom(s.into())
}
fn category(tree: &Tree) -> Result<String> {
    match tree.field("category") {
        Some(Tree::Atom(s)) => Ok(s.clone()),
        _ => anyhow::bail!("coordination category missing: {tree:?}"),
    }
}
fn shape(tree: &Tree, expected: &[&str], forms: &[&str]) -> Result<()> {
    let Tree::Record(name, fields) = tree else { unreachable!() };
    ensure!(
        fields
            .iter()
            .map(|(n, _)| n.as_str())
            .eq(expected.iter().copied())
            && forms.iter().any(|f| tree.field("form") == Some(&atom(f))),
        "coordination normal-form adapter needs review: {name} shape changed"
    );
    Ok(())
}
fn field(tree: &Tree, name: &str) -> Tree {
    tree.field(name).unwrap().clone()
}

// Full Word values stay on the links; only this exact ID is used to decide
// whether adjacent joins may associate. Then and But never associate here.
fn marker(word: &Tree) -> Result<&str> {
    let id = word
        .word_id()
        .ok_or_else(|| anyhow::anyhow!("coordinator is not a Word"))?;
    ensure!(
        matches!(
            id,
            "vocab:Coordinator/And"
                | "vocab:Coordinator/Or"
                | "vocab:Coordinator/AndOr"
                | "vocab:Coordinator/Then"
                | "vocab:Coordinator/But"
        ),
        "coordination normal-form adapter needs review: coordinator {id}"
    );
    Ok(id)
}
fn associative(id: &str) -> bool {
    matches!(
        id,
        "vocab:Coordinator/And" | "vocab:Coordinator/Or" | "vocab:Coordinator/AndOr"
    )
}
fn link(word: Tree, comma: bool) -> Tree {
    Tree::List(vec![atom(if comma { ", " } else { " " }), word])
}
fn comma() -> Tree {
    Tree::List(vec![atom(", ")])
}
fn chain(category: String, id: String, members: Vec<Tree>, links: Vec<Tree>) -> Tree {
    let mut flat_members = Vec::new();
    let mut flat_links = Vec::new();
    let mut links = links.into_iter();
    for (i, member) in members.into_iter().enumerate() {
        if i > 0 {
            flat_links.push(links.next().unwrap());
        }
        match member {
            Tree::Coordination {
                category: child_category,
                marker,
                members,
                links,
            } if associative(&id) && child_category == category && marker == id => {
                flat_members.extend(members);
                flat_links.extend(links);
            }
            other => flat_members.push(other),
        }
    }
    Tree::Coordination {
        category,
        marker: id,
        members: flat_members,
        links: flat_links,
    }
}

fn helper(category: &str) -> String {
    match category {
        "Nominal" => "NominalSeries".into(),
        "SecondaryVerbPhrase" => "SecondaryPredicateSeries".into(),
        "CoordinatedFiniteClause" => "FiniteClauseSeries".into(),
        "FocusedAdjunctCoordination" => "FocusedAdjunctSeries".into(),
        other => format!("{other}Series"),
    }
}
// A helper is consumed only under its matching serial result. Correlative
// helpers and standalone helpers remain exact records.
fn series(tree: &Tree, expected: &str) -> Result<(Vec<Tree>, Vec<Tree>, String)> {
    ensure!(
        category(tree)? == expected,
        "coordination helper category changed"
    );
    let Tree::Record(name, _) = tree else { unreachable!() };
    match name.as_str() {
        "CoordinationSeriesEnd" => {
            shape(
                tree,
                &["form", "category", "left", "coordinator", "right"],
                &["0"],
            )?;
            let word = field(tree, "coordinator");
            let id = marker(&word)?.to_owned();
            Ok((
                vec![field(tree, "left"), field(tree, "right")],
                vec![link(word, true)],
                id,
            ))
        }
        "CoordinationSeriesContinuation" => {
            shape(tree, &["form", "category", "left", "rest"], &["0"])?;
            let (mut members, mut links, id) = series(tree.field("rest").unwrap(), expected)?;
            members.insert(0, field(tree, "left"));
            links.insert(0, comma());
            Ok((members, links, id))
        }
        _ => anyhow::bail!("coordination helper constructor changed: {name}"),
    }
}
fn imperative(predicate: Tree) -> Tree {
    match predicate {
        Tree::Coordination {
            category,
            marker,
            members,
            links,
        } if category == "SecondaryVerbPhrase" => chain(
            "Clause".into(),
            marker,
            members.into_iter().map(imperative).collect(),
            links,
        ),
        head => Tree::Record(
            "Imperative".into(),
            vec![
                ("form".into(), atom("0")),
                (
                    "predicate".into(),
                    Tree::Record(
                        "BarePredicate".into(),
                        vec![("form".into(), atom("0")), ("head".into(), head)],
                    ),
                ),
            ],
        ),
    }
}

pub(super) fn normalize(tree: Tree) -> Result<Tree> {
    let tree = match tree {
        Tree::Record(name, fields) => Tree::Record(
            name,
            fields
                .into_iter()
                .map(|(n, v)| Ok((n, normalize(v)?)))
                .collect::<Result<_>>()?,
        ),
        Tree::Call(name, values) => Tree::Call(
            name,
            values.into_iter().map(normalize).collect::<Result<_>>()?,
        ),
        Tree::List(values) => Tree::List(values.into_iter().map(normalize).collect::<Result<_>>()?),
        Tree::Atom(_) => tree,
        _ => anyhow::bail!("coordination normalization cannot run twice"),
    };
    let Tree::Record(name, _) = &tree else { return Ok(tree) };
    match name.as_str() {
        "Coordination" | "ClauseCoordination" => {
            let clause = name == "ClauseCoordination";
            shape(
                &tree,
                if clause {
                    &["form", "left", "coordinator", "right"]
                } else {
                    &["form", "category", "left", "coordinator", "right"]
                },
                if clause { &["0", "1"] } else { &["0"] },
            )?;
            let word = field(&tree, "coordinator");
            let id = marker(&word)?.to_owned();
            let comma = if clause {
                tree.field("form") == Some(&atom("0"))
            } else {
                id == "vocab:Coordinator/Then"
            };
            Ok(chain(
                if clause { "Clause".into() } else { category(&tree)? },
                id,
                vec![field(&tree, "left"), field(&tree, "right")],
                vec![link(word, comma)],
            ))
        }
        "SerialCoordination" => {
            shape(&tree, &["form", "category", "left", "rest"], &["0"])?;
            let cat = category(&tree)?;
            // This schema also spells asyndetic shared-head premodifiers:
            // its rest is a Premodifier, not a coordinator-ending series.
            // Those irregular paths are outside the reviewed grouping rule.
            if cat == "PremodifierCoordination" {
                return Ok(tree);
            }
            let (mut members, mut links, id) = series(tree.field("rest").unwrap(), &helper(&cat))?;
            members.insert(0, field(&tree, "left"));
            links.insert(0, comma());
            Ok(chain(cat, id, members, links))
        }
        "Imperative" => {
            shape(&tree, &["form", "predicate"], &["0"])?;
            let predicate = tree.field("predicate").unwrap();
            if let Tree::Record(name, _) = predicate
                && name == "BarePredicate"
            {
                shape(predicate, &["form", "head"], &["0"])?;
                Ok(imperative(field(predicate, "head")))
            } else {
                Ok(tree)
            }
        }
        _ => Ok(tree),
    }
}

#[cfg(test)]
#[path = "coordination_normal_form_tests.rs"]
mod tests;
