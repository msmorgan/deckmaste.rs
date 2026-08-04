//! Layer-2 validation. `ValidatedGroup` is deliberately the ONLY door to
//! the emitter: its constructor is private to this module, so unvalidated
//! emission is unrepresentable.

use crate::diag::DiagCode;
use crate::diag::Diagnostic;
use crate::diag::sort_key;
use crate::model::GroupDeclaration;

#[derive(Debug)]
pub struct ValidatedGroup<'a> {
    group: &'a GroupDeclaration,
}

impl<'a> ValidatedGroup<'a> {
    pub fn group(&self) -> &'a GroupDeclaration {
        self.group
    }
}

pub fn validate(group: &GroupDeclaration) -> Result<ValidatedGroup<'_>, Vec<Diagnostic>> {
    let mut diags: Vec<Diagnostic> = Vec::new();
    checks(group, &mut diags);
    if diags.is_empty() {
        Ok(ValidatedGroup { group })
    } else {
        diags.sort_by_key(sort_key);
        Err(diags)
    }
}

// Each task in this plan appends one check family here.
fn checks(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    check_identity(group, diags);
    check_dominance_cycles(group, diags);
}

fn check_identity(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    let mut seen_ids: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        if !seen_ids.insert(id) {
            diags.push(
                Diagnostic::new(
                    DiagCode::DuplicateConstructionId,
                    id,
                    format!("construction id `{id}` is declared more than once in this group"),
                )
                .with_span(construction.id.span),
            );
        }
        let mut seen_ordinals: std::collections::HashSet<u16> = std::collections::HashSet::new();
        for form in &construction.forms {
            if !seen_ordinals.insert(form.ordinal.value) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::DuplicateOrdinal,
                        id,
                        format!(
                            "production ordinal {} is used by more than one form",
                            form.ordinal.value
                        ),
                    )
                    .with_span(form.ordinal.span),
                );
            }
        }
        for edge in &construction.dominance {
            if edge.winner.value == edge.loser.value {
                diags.push(
                    Diagnostic::new(
                        DiagCode::SelfDominance,
                        id,
                        format!("`{}` cannot dominate itself", edge.winner.value),
                    )
                    .with_span(edge.winner.span),
                );
            }
        }
    }
}

fn check_dominance_cycles(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    let ids: std::collections::HashSet<&str> = group
        .constructions
        .iter()
        .map(|c| c.id.value.as_str())
        .collect();
    let mut edges: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for construction in &group.constructions {
        for edge in &construction.dominance {
            let winner = edge.winner.value.as_str();
            let loser = edge.loser.value.as_str();
            if ids.contains(winner) && ids.contains(loser) && winner != loser {
                edges.entry(winner).or_default().push(loser);
            }
        }
    }
    // Deterministic traversal: sorted starts and sorted successor lists make
    // the emitted diagnostic a pure function of the edge SET — never of
    // HashSet iteration (per-process random seed) or edge declaration order.
    for successors in edges.values_mut() {
        successors.sort_unstable();
    }
    let mut starts: Vec<&str> = ids.iter().copied().collect();
    starts.sort_unstable();
    // Iterative DFS with three-color marking; a back edge is a cycle. The
    // cursor into each node's successor list lives in the stack entry, and
    // we re-borrow the stack per step so no long-lived &mut fights the push.
    let mut state: std::collections::HashMap<&str, u8> = std::collections::HashMap::new();
    for start in starts {
        if state.get(start).copied().unwrap_or(0) != 0 {
            continue;
        }
        let mut stack: Vec<(&str, usize)> = vec![(start, 0)];
        state.insert(start, 1);
        while let Some((node, next)) = stack.last().copied() {
            let successors = edges.get(node).map_or(&[][..], Vec::as_slice);
            if next < successors.len() {
                stack.last_mut().expect("stack is nonempty here").1 += 1;
                let successor = successors[next];
                match state.get(successor).copied().unwrap_or(0) {
                    0 => {
                        state.insert(successor, 1);
                        stack.push((successor, 0));
                    }
                    1 => diags.push(Diagnostic::new(
                        DiagCode::DominanceCycle,
                        successor,
                        format!("declared dominance cycles through `{successor}`"),
                    )),
                    _ => {}
                }
            } else {
                state.insert(node, 2);
                stack.pop();
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod fixtures {
    use crate::model::AstShape;
    use crate::model::ConstructionDeclaration;
    use crate::model::FieldBinding;
    use crate::model::FieldKind;
    use crate::model::FieldPath;
    use crate::model::FormDeclaration;
    use crate::model::GroupDeclaration;
    use crate::model::SelectionPromise;
    use crate::model::Spanned;
    use crate::model::SurfaceAtom;

    /// Smallest valid group: one construction, one scalar field, one form
    /// producing it. Every validator test perturbs a clone of this.
    pub(crate) fn minimal_group() -> GroupDeclaration {
        GroupDeclaration {
            name: Spanned::call_site("noun_coordination".to_owned()),
            elements: vec![],
            constructions: vec![ConstructionDeclaration {
                id: Spanned::call_site("noun_phrase_coordination".to_owned()),
                category: Spanned::call_site("NounPhrase".to_owned()),
                internal: false,
                ast: AstShape::Bind {
                    path: Spanned::call_site("crate::syntax::CoordinatedNounPhrase".to_owned()),
                    fields: vec![FieldBinding {
                        field: Spanned::call_site("conjunction".to_owned()),
                        kind: FieldKind::Scalar {
                            codec: Spanned::call_site("Conjunction".to_owned()),
                        },
                    }],
                },
                constraints: vec![],
                witnesses: vec![],
                forms: vec![FormDeclaration {
                    name: Spanned::call_site("binary".to_owned()),
                    ordinal: Spanned::call_site(0),
                    surface: vec![SurfaceAtom::Lexeme(FieldPath::call_site("conjunction"))],
                    guard: None,
                }],
                dominance: vec![],
                selection: SelectionPromise::Packed,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::minimal_group;
    use super::*;

    #[test]
    fn minimal_group_validates() {
        let group = minimal_group();
        let validated = validate(&group).expect("minimal group is valid");
        assert_eq!(validated.group().constructions.len(), 1);
    }

    fn codes(err: Vec<crate::diag::Diagnostic>) -> Vec<&'static str> {
        err.iter().map(|d| d.code.as_str()).collect()
    }

    #[test]
    fn duplicate_construction_ids_are_rejected() {
        let mut group = minimal_group();
        let twin = group.constructions[0].clone();
        group.constructions.push(twin);
        let err = validate(&group).expect_err("duplicate id");
        assert!(codes(err).contains(&"EC001"));
    }

    #[test]
    fn duplicate_ordinals_within_a_construction_are_rejected() {
        let mut group = minimal_group();
        let mut second = group.constructions[0].forms[0].clone();
        second.name = crate::model::Spanned::call_site("oxford".to_owned());
        // Same ordinal 0 on a second form: explicit ordinals exist precisely
        // so reordering can never silently renumber — reuse is an error.
        group.constructions[0].forms.push(second);
        let err = validate(&group).expect_err("duplicate ordinal");
        assert!(codes(err).contains(&"EC002"));
    }

    #[test]
    fn self_dominance_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
            });
        let err = validate(&group).expect_err("self dominance");
        assert!(codes(err).contains(&"EC040"));
    }

    #[test]
    fn in_group_dominance_cycles_are_rejected() {
        let mut group = minimal_group();
        let mut second = group.constructions[0].clone();
        second.id = crate::model::Spanned::call_site("noun_phrase_list_comma".to_owned());
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_list_comma".to_owned()),
            });
        second.dominance.push(crate::model::DominanceEdge {
            winner: crate::model::Spanned::call_site("noun_phrase_list_comma".to_owned()),
            loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
        });
        group.constructions.push(second);
        let err = validate(&group).expect_err("two-node cycle");
        assert!(codes(err).contains(&"EC041"));
    }

    #[test]
    fn edges_leaving_the_group_are_not_cycle_checked_here() {
        let mut group = minimal_group();
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_nominal".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
            });
        validate(&group).expect("external edge is a registry-time concern");
    }
}
