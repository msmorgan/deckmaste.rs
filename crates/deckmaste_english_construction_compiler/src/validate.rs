//! Layer-2 validation. `ValidatedGroup` is deliberately the ONLY door to
//! the emitter: its constructor is private to this module, so unvalidated
//! emission is unrepresentable.

use crate::diag::DiagCode;
use crate::diag::Diagnostic;
use crate::diag::sort_key;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::FieldPath;
use crate::model::GroupDeclaration;
use crate::model::KNOWN_COMBINATORS;
use crate::model::Predicate;
use crate::model::SurfaceAtom;
use crate::model::WitnessClass;

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

fn checks(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    check_identity(group, diags);
    check_dominance_cycles(group, diags);
    check_paths(group, diags);
    check_forms(group, diags);
    check_surface_domain(group, diags);
    check_constraints(group, diags);
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
                    // Call-site span: the edge map is keyed by id strings and
                    // carries no spans, so there is nothing here to point at.
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

/// Resolves a path against a construction's ast fields and, through
/// Sequence fields, the group's element declarations. `last` addresses a
/// sequence element. Returns the terminal FieldKind.
fn resolve_path<'g>(
    group: &'g GroupDeclaration,
    construction: &'g ConstructionDeclaration,
    path: &FieldPath,
) -> Option<&'g FieldKind> {
    let mut fields: &'g [FieldBinding] = construction.ast.fields();
    let mut resolved: Option<&'g FieldKind> = None;
    // Whether the IMMEDIATELY preceding step was a binding that resolved to
    // a Sequence. Gating on `resolved` instead would only mean "after SOME
    // earlier Sequence", since `last` leaves the resolved kind alone — and
    // that admits `rest.last.last`, which addresses nothing.
    let mut sequence_in_hand = false;
    let mut segments = path.segments.iter().peekable();
    while let Some(segment) = segments.next() {
        if segment == "last" {
            // `last` re-addresses the sequence element just entered, and
            // consumes that context: there is no second element to take.
            if !sequence_in_hand {
                return None;
            }
            sequence_in_hand = false;
            continue;
        }
        let binding = fields.iter().find(|b| &b.field.value == segment)?;
        resolved = Some(&binding.kind);
        sequence_in_hand = matches!(binding.kind, FieldKind::Sequence { .. });
        if segments.peek().is_some() {
            match &binding.kind {
                FieldKind::Sequence { element } => {
                    let declared = group
                        .elements
                        .iter()
                        .find(|e| e.name.value == element.value)?;
                    fields = &declared.fields;
                }
                _ => return None,
            }
        }
    }
    resolved
}

fn check_paths(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        let field_names: std::collections::HashSet<&str> = construction
            .ast
            .fields()
            .iter()
            .map(|b| b.field.value.as_str())
            .collect();

        for binding in construction.ast.fields() {
            if let FieldKind::Sequence { element } = &binding.kind {
                if !group.elements.iter().any(|e| e.name.value == element.value) {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::UnknownElement,
                            id,
                            format!(
                                "sequence field `{}` names undeclared element `{}`",
                                binding.field.value, element.value
                            ),
                        )
                        .with_span(element.span),
                    );
                }
            }
        }
        for form in &construction.forms {
            for atom in &form.surface {
                let path = match atom {
                    SurfaceAtom::Hole(path) | SurfaceAtom::Lexeme(path) => path,
                    SurfaceAtom::Literal(_) => continue,
                };
                if resolve_path(group, construction, path).is_none() {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::UnknownFieldPath,
                            id,
                            format!(
                                "form `{}` references unknown path `{}`",
                                form.name.value,
                                path.segments.join(".")
                            ),
                        )
                        .with_span(path.span),
                    );
                }
            }
        }
        for witness in &construction.witnesses {
            if field_names.contains(witness.name.value.as_str()) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::WitnessFieldCollision,
                        id,
                        format!(
                            "witness `{}` collides with an ast field of the same name",
                            witness.name.value
                        ),
                    )
                    .with_span(witness.name.span),
                );
            }
            if let WitnessClass::Stored { path } = &witness.class {
                if resolve_path(group, construction, path).is_none() {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::StoredWitnessPathUnknown,
                            id,
                            format!(
                                "stored witness `{}` names unknown path `{}`",
                                witness.name.value,
                                path.segments.join(".")
                            ),
                        )
                        .with_span(path.span),
                    );
                }
            }
        }
    }
}

fn check_forms(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        let mut produced: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for form in &construction.forms {
            if form.surface.is_empty() {
                diags.push(
                    Diagnostic::new(
                        DiagCode::EmptyProduction,
                        id,
                        format!(
                            "form `{}` has an empty surface; the chart forbids empty productions",
                            form.name.value
                        ),
                    )
                    .with_span(form.name.span),
                );
            }
            let mut consumed: std::collections::HashSet<String> = std::collections::HashSet::new();
            for atom in &form.surface {
                let path = match atom {
                    SurfaceAtom::Hole(path) | SurfaceAtom::Lexeme(path) => path,
                    SurfaceAtom::Literal(_) => continue,
                };
                let dotted = path.segments.join(".");
                if !consumed.insert(dotted.clone()) {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::HoleConsumedTwice,
                            id,
                            format!(
                                "form `{}` consumes `{dotted}` more than once",
                                form.name.value
                            ),
                        )
                        .with_span(path.span),
                    );
                }
                if let Some(binding) = construction
                    .ast
                    .fields()
                    .iter()
                    .find(|b| Some(&b.field.value) == path.segments.first())
                {
                    // Unresolvable paths already got EC010; skip, no sentinel.
                    // Sound only because `checks()` always runs check_paths in
                    // the same pass — if that ever becomes conditional/short-
                    // circuited, a bogus deeper segment would no longer be
                    // caught and this first-segment match would wrongly mark
                    // the field produced.
                    produced.insert(binding.field.value.as_str());
                }
            }
        }
        for binding in construction.ast.fields() {
            if !produced.contains(binding.field.value.as_str()) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::FieldNeverProduced,
                        id,
                        format!("ast field `{}` is produced by no form", binding.field.value),
                    )
                    .with_span(binding.field.span),
                );
            }
        }
    }
}

/// Which aspect of a path a constraint speaks about. Keying on this rather
/// than on a decorated path string (`"rest#len"`) is what lets a diagnostic
/// print the path the author actually wrote: a synthetic key has no author.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facet {
    /// The value itself: the set holds variant names.
    Value,
    /// Optionality: the set is a subset of `{"some", "none"}`.
    Presence,
    /// Sequence length: the set holds decimal lengths.
    Len,
}

/// A conservative finite abstraction of one predicate. Two forms overlap if
/// every commonly-constrained key has a non-empty intersection; a predicate
/// is contradictory if any of its own sets is empty.
#[derive(Debug, Clone, Default)]
struct Abstraction {
    /// (path as the author dotted it, facet) → the finite set that facet is
    /// restricted to. A key is present only when some predicate imposed a
    /// finite restriction on it; an absent key means "unconstrained", which
    /// is emphatically NOT the same as an empty set (that means "provably
    /// unsatisfiable"). `LenAtLeast` is open-ended and so adds no key.
    allowed: std::collections::HashMap<(String, Facet), std::collections::BTreeSet<String>>,
}

impl Abstraction {
    /// Constraining the same key twice intersects: that is what makes
    /// `All[In{And}, In{Or}]` provably empty and `IsSome ∧ IsNone` disjoint.
    fn intersect(&mut self, key: (String, Facet), values: std::collections::BTreeSet<String>) {
        match self.allowed.entry(key) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(values);
            }
            std::collections::hash_map::Entry::Occupied(mut slot) => {
                let narrowed = slot.get().intersection(&values).cloned().collect();
                slot.insert(narrowed);
            }
        }
    }

    /// Conjoins another abstraction into this one, key by key. Order of
    /// iteration is irrelevant: intersection is commutative and every key
    /// is independent, so the result is a pure function of the operand set.
    /// Keys absent from `other` carry no constraint and so are left alone —
    /// treating absence as the empty set would zero out a real constraint
    /// and fabricate a contradiction.
    fn intersect_all(&mut self, other: &Self) {
        for (key, values) in &other.allowed {
            self.intersect(key.clone(), values.clone());
        }
    }
}

fn abstract_predicate(predicate: &Predicate) -> Abstraction {
    let mut abstraction = Abstraction::default();
    collect(predicate, &mut abstraction);
    return abstraction;

    fn collect(predicate: &Predicate, into: &mut Abstraction) {
        match predicate {
            Predicate::In { path, allowed } => {
                into.intersect(
                    (path.segments.join("."), Facet::Value),
                    allowed.iter().cloned().collect(),
                );
            }
            Predicate::IsSome { path } => {
                into.intersect(
                    (path.segments.join("."), Facet::Presence),
                    std::iter::once("some".to_owned()).collect(),
                );
            }
            Predicate::IsNone { path } => {
                into.intersect(
                    (path.segments.join("."), Facet::Presence),
                    std::iter::once("none".to_owned()).collect(),
                );
            }
            Predicate::LenIs { path, len } => {
                into.intersect(
                    (path.segments.join("."), Facet::Len),
                    std::iter::once(len.to_string()).collect(),
                );
            }
            Predicate::LenAtLeast { .. } => {
                // Open-ended stratum: contributes no finite set, so it can
                // neither prove disjointness nor emptiness. Conservative.
            }
            Predicate::All(children) => {
                for child in children {
                    collect(child, into);
                }
            }
            Predicate::Any(children) => {
                // Union: conservative — drop constraints that differ across
                // branches by keeping only keys constrained in EVERY branch
                // with the union of their sets. A key missing from any
                // branch is unconstrained in that branch, hence
                // unconstrained in the union, hence contributes nothing.
                let mut branch_abstractions: Vec<Abstraction> = Vec::new();
                for child in children {
                    branch_abstractions.push(abstract_predicate(child));
                }
                if let Some(first) = branch_abstractions.first().cloned() {
                    for (key, set) in first.allowed {
                        let mut union = set;
                        let mut in_all = true;
                        for other in &branch_abstractions[1..] {
                            match other.allowed.get(&key) {
                                Some(other_set) => union.extend(other_set.iter().cloned()),
                                None => in_all = false,
                            }
                        }
                        // A sibling constraint on the same key (e.g. from an
                        // enclosing All) must be intersected against this
                        // union, not overwritten — route through `intersect`
                        // like every other arm.
                        if in_all {
                            into.intersect(key, union);
                        }
                    }
                }
            }
        }
    }
}

fn abstractions_overlap(a: &Abstraction, b: &Abstraction) -> bool {
    for (key, a_values) in &a.allowed {
        if let Some(b_values) = b.allowed.get(key) {
            if a_values.intersection(b_values).next().is_none() {
                return false; // provably disjoint on this key
            }
        }
    }
    true // no key proves them apart — conservative overlap
}

/// Renders EC023 for one facet. The author's own path is printed verbatim
/// and the facet is spelled out in words: an abstraction key is an internal
/// index, and no author ever wrote one.
fn uncovered_message(key: &(String, Facet), missing: &[&String]) -> String {
    let (path, facet) = key;
    match facet {
        Facet::Value => {
            let values: Vec<&str> = missing.iter().map(|value| value.as_str()).collect();
            format!(
                "admitted values for `{path}` have no accepting form: {}",
                values.join(", ")
            )
        }
        Facet::Presence => {
            let states: Vec<&str> = missing.iter().map(|value| presence_word(value)).collect();
            format!(
                "`{path}` has no accepting form when it is {}",
                states.join(" or ")
            )
        }
        Facet::Len => {
            let lengths: Vec<&str> = missing.iter().map(|value| value.as_str()).collect();
            let noun = if lengths.len() == 1 { "length" } else { "lengths" };
            format!(
                "`{path}` has no accepting form at {noun} {}",
                lengths.join(", ")
            )
        }
    }
}

/// Renders EC030 for one facet, under the same rule as [`uncovered_message`].
fn contradiction_message(key: &(String, Facet)) -> String {
    let (path, facet) = key;
    match facet {
        Facet::Value => format!("requirements on `{path}` admit no value at all"),
        Facet::Presence => format!("requirements on `{path}` demand it be both present and absent"),
        Facet::Len => format!("requirements on `{path}` admit no length at all"),
    }
}

fn presence_word(value: &str) -> &'static str {
    match value {
        "none" => "absent",
        // The Presence facet's set is populated only by the `IsSome`/
        // `IsNone` arms, so "some" is its only other inhabitant.
        _ => "present",
    }
}

/// The conjunction of every `require` clause on one construction.
///
/// Multiple clauses are conjunctive, so a key's admitted set is their
/// INTERSECTION — folding clause abstractions in with `insert` would let
/// whichever clause was written last decide the answer, making both EC023
/// and EC030 depend on declaration order. `origin` records the span of the
/// first clause that constrained each key, so a diagnostic derived from the
/// conjunction can still point at source.
struct RequiredFacts {
    admitted: Abstraction,
    origin: std::collections::HashMap<(String, Facet), proc_macro2::Span>,
}

fn required_facts(construction: &ConstructionDeclaration) -> RequiredFacts {
    let mut admitted = Abstraction::default();
    let mut origin: std::collections::HashMap<(String, Facet), proc_macro2::Span> =
        std::collections::HashMap::new();
    for constraint in &construction.constraints {
        let Constraint::Require(predicate) = constraint else { continue };
        let clause = abstract_predicate(&predicate.value);
        for key in clause.allowed.keys() {
            origin.entry(key.clone()).or_insert(predicate.span);
        }
        admitted.intersect_all(&clause);
    }
    RequiredFacts { admitted, origin }
}

fn check_surface_domain(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        let has_free_witness = construction
            .witnesses
            .iter()
            .any(|w| matches!(w.class, WitnessClass::Free { .. }));
        let guards: Vec<Abstraction> = construction
            .forms
            .iter()
            .map(|form| {
                form.guard
                    .as_ref()
                    .map_or_else(Abstraction::default, |g| abstract_predicate(&g.value))
            })
            .collect();
        if construction.forms.len() > 1 && !has_free_witness {
            for left in 0..guards.len() {
                for right in (left + 1)..guards.len() {
                    if abstractions_overlap(&guards[left], &guards[right]) {
                        // Sorted: explicit ordinals exist so that form order
                        // is not semantic, so the message must not change
                        // when the two declarations are swapped.
                        let mut named = [
                            construction.forms[left].name.value.as_str(),
                            construction.forms[right].name.value.as_str(),
                        ];
                        named.sort_unstable();
                        diags.push(
                            Diagnostic::new(
                                DiagCode::AmbiguousLinearization,
                                id,
                                format!(
                                    "forms `{}` and `{}` can both match the same value and no witness discriminates them",
                                    named[0], named[1]
                                ),
                            )
                            .with_span(construction.forms[left].name.span),
                        );
                    }
                }
            }
        }
        // Coverage: for every path with a finite admitted set declared by
        // constraints, the union of form-guard sets must reach every value.
        let required = required_facts(construction);
        for (key, admitted_values) in &required.admitted.allowed {
            let mut covered: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            let mut any_unguarded_form = false;
            for abstraction in &guards {
                match abstraction.allowed.get(key) {
                    Some(values) => covered.extend(values.iter().cloned()),
                    None => any_unguarded_form = true,
                }
            }
            if !any_unguarded_form && !admitted_values.is_subset(&covered) {
                let missing: Vec<&String> = admitted_values.difference(&covered).collect();
                let span = *required
                    .origin
                    .get(key)
                    .expect("every admitted key was recorded when its clause was folded in");
                diags.push(
                    Diagnostic::new(
                        DiagCode::UncoveredValueSpace,
                        id,
                        uncovered_message(key, &missing),
                    )
                    .with_span(span),
                );
            }
        }
    }
}

fn check_constraints(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        // Contradictions are judged on the CONJUNCTION of every `require`
        // clause, so `require in(x,{And})` alongside `require in(x,{Or})`
        // is caught even though neither clause is empty on its own.
        let required = required_facts(construction);
        for (key, values) in &required.admitted.allowed {
            if values.is_empty() {
                let span = *required
                    .origin
                    .get(key)
                    .expect("every admitted key was recorded when its clause was folded in");
                diags.push(
                    Diagnostic::new(
                        DiagCode::ContradictoryConstraints,
                        id,
                        contradiction_message(key),
                    )
                    .with_span(span),
                );
            }
        }
        // Combinators are checked wherever one is named, not only on the
        // `derive_feature` arm: a derived witness names the same concept in
        // another syntactic position and the guarantee is about the concept.
        // (Its `args` paths are deliberately a later milestone's business.)
        let combinators = construction
            .constraints
            .iter()
            .filter_map(|constraint| match constraint {
                Constraint::DeriveFeature { combinator, .. } => Some(combinator),
                Constraint::Require(_) => None,
            })
            .chain(
                construction
                    .witnesses
                    .iter()
                    .filter_map(|witness| match &witness.class {
                        WitnessClass::Derived { combinator, .. } => Some(combinator),
                        WitnessClass::Stored { .. } | WitnessClass::Free { .. } => None,
                    }),
            );
        for combinator in combinators {
            if !KNOWN_COMBINATORS.contains(&combinator.value.as_str()) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownCombinator,
                        id,
                        format!(
                            "unknown feature combinator `{}`; a missing combinator is a compiler addition, never a closure",
                            combinator.value
                        ),
                    )
                    .with_span(combinator.span),
                );
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

    /// The rendered text of the one diagnostic carrying `code`. Asserting on
    /// the exact text is what pins "the author's own path, never an internal
    /// abstraction key" — a code-only assertion cannot see message quality.
    fn message_for(err: &[crate::diag::Diagnostic], code: &str) -> String {
        err.iter()
            .find(|d| d.code.as_str() == code)
            .unwrap_or_else(|| panic!("expected a {code} diagnostic; got {err:?}"))
            .message
            .clone()
    }

    /// `minimal_group()` plus `rest: Sequence<NounPhraseCoordination>`, the
    /// element it names, and a form atom producing it.
    fn group_with_sequence_field() -> crate::model::GroupDeclaration {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            }],
        });
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("rest".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("rest.comma"),
            ));
        group
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
    fn unknown_field_paths_in_forms_are_rejected() {
        let mut group = minimal_group();
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("ghost"),
            ));
        let err = validate(&group).expect_err("ghost path");
        assert!(codes(err).contains(&"EC010"));
    }

    #[test]
    fn sequence_fields_resolve_through_declared_elements() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            }],
        });
        group.constructions[0].ast = match group.constructions[0].ast.clone() {
            crate::model::AstShape::Bind { path, mut fields } => {
                fields.push(crate::model::FieldBinding {
                    field: crate::model::Spanned::call_site("rest".to_owned()),
                    kind: crate::model::FieldKind::Sequence {
                        element: crate::model::Spanned::call_site(
                            "NounPhraseCoordination".to_owned(),
                        ),
                    },
                });
                crate::model::AstShape::Bind { path, fields }
            }
            own => own,
        };
        // `rest` must be produced by some form (EC021 arrives in Task 11);
        // reference it so this test isolates path resolution.
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("rest.comma"),
            ));
        validate(&group).expect("rest.comma resolves through the element");
    }

    #[test]
    fn leading_last_with_no_sequence_context_is_rejected() {
        let mut group = minimal_group();
        // `last` re-addresses a sequence element; as the first segment there
        // is no sequence in scope yet for it to re-address.
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("last.conjunction"),
            ));
        let err = validate(&group).expect_err("last with no preceding sequence");
        assert!(codes(err).contains(&"EC010"));
    }

    #[test]
    fn last_after_a_sequence_field_still_resolves() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            }],
        });
        group.constructions[0].ast = match group.constructions[0].ast.clone() {
            crate::model::AstShape::Bind { path, mut fields } => {
                fields.push(crate::model::FieldBinding {
                    field: crate::model::Spanned::call_site("rest".to_owned()),
                    kind: crate::model::FieldKind::Sequence {
                        element: crate::model::Spanned::call_site(
                            "NounPhraseCoordination".to_owned(),
                        ),
                    },
                });
                crate::model::AstShape::Bind { path, fields }
            }
            own => own,
        };
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("rest.last.comma"),
            ));
        validate(&group).expect("last after a resolved Sequence field still resolves");
    }

    #[test]
    fn stacked_last_segments_are_rejected() {
        // The first `last` consumes the sequence context `rest` opened; the
        // second has none left to consume and addresses nothing. The sibling
        // test `last_after_a_sequence_field_still_resolves` is the other
        // half of this pair: it goes red if the gate rejects a lone `last`.
        let mut group = group_with_sequence_field();
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("rest.last.last"),
            ));
        let err = validate(&group).expect_err("`rest.last.last` addresses nothing");
        assert_eq!(
            message_for(&err, "EC010"),
            "form `binary` references unknown path `rest.last.last`"
        );
    }

    #[test]
    fn sequence_naming_an_undeclared_element_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0].ast = match group.constructions[0].ast.clone() {
            crate::model::AstShape::Bind { path, mut fields } => {
                fields.push(crate::model::FieldBinding {
                    field: crate::model::Spanned::call_site("rest".to_owned()),
                    kind: crate::model::FieldKind::Sequence {
                        element: crate::model::Spanned::call_site("Phantom".to_owned()),
                    },
                });
                crate::model::AstShape::Bind { path, fields }
            }
            own => own,
        };
        let err = validate(&group).expect_err("phantom element");
        assert!(codes(err).contains(&"EC003"));
    }

    #[test]
    fn witness_names_may_not_collide_with_ast_fields() {
        let mut group = minimal_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("conjunction".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        let err = validate(&group).expect_err("collision");
        assert!(codes(err).contains(&"EC012"));
    }

    #[test]
    fn stored_witness_paths_must_resolve() {
        let mut group = minimal_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("shadow_comma".to_owned()),
                class: crate::model::WitnessClass::Stored {
                    path: crate::model::FieldPath::call_site("ghost.comma"),
                },
            });
        let err = validate(&group).expect_err("stored path unknown");
        assert!(codes(err).contains(&"EC013"));
    }

    #[test]
    fn empty_surface_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0].forms[0].surface.clear();
        let err = validate(&group).expect_err("empty production");
        assert!(codes(err).contains(&"EC020"));
    }

    #[test]
    fn fields_no_form_produces_are_rejected() {
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("orphan".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        }
        let err = validate(&group).expect_err("orphan field");
        assert!(codes(err).contains(&"EC021"));
    }

    #[test]
    fn double_consumption_within_one_form_is_rejected() {
        let mut group = minimal_group();
        let duplicate =
            crate::model::SurfaceAtom::Lexeme(crate::model::FieldPath::call_site("conjunction"));
        group.constructions[0].forms[0].surface.push(duplicate);
        let err = validate(&group).expect_err("double consumption");
        assert!(codes(err).contains(&"EC022"));
    }

    fn conjunction_in(allowed: &[&str]) -> crate::model::Predicate {
        crate::model::Predicate::In {
            path: crate::model::FieldPath::call_site("conjunction"),
            allowed: allowed.iter().map(|value| (*value).to_owned()).collect(),
        }
    }

    fn requires_conjunction_in(allowed: &[&str]) -> crate::model::Constraint {
        crate::model::Constraint::Require(crate::model::Spanned::call_site(conjunction_in(allowed)))
    }

    fn guarded_two_form_group(
        first: crate::model::Predicate,
        second: crate::model::Predicate,
    ) -> crate::model::GroupDeclaration {
        let mut group = minimal_group();
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(first));
        let mut oxford = group.constructions[0].forms[0].clone();
        oxford.name = crate::model::Spanned::call_site("oxford".to_owned());
        oxford.ordinal = crate::model::Spanned::call_site(1);
        oxford.guard = Some(crate::model::Spanned::call_site(second));
        group.constructions[0].forms.push(oxford);
        group
    }

    #[test]
    fn overlapping_guards_without_witness_are_rejected() {
        let group = guarded_two_form_group(
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["And".to_owned(), "Or".to_owned()],
            },
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["Or".to_owned(), "Plus".to_owned()],
            },
        );
        let err = validate(&group).expect_err("Or satisfies both guards");
        assert!(codes(err).contains(&"EC024"));
    }

    #[test]
    fn disjoint_guards_pass() {
        let group = guarded_two_form_group(
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["And".to_owned()],
            },
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["Or".to_owned()],
            },
        );
        validate(&group).expect("disjoint guards are exactly the promise");
    }

    #[test]
    fn a_form_set_that_cannot_cover_its_guarded_paths_is_rejected() {
        // Two forms both guarded to strict subsets of nothing shared and no
        // unguarded fallback: values outside both sets have no form.
        let mut group = guarded_two_form_group(
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["And".to_owned()],
            },
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["Or".to_owned()],
            },
        );
        // Mark the domain as larger than the union by adding a third
        // constrained value via a constraint mentioning Plus.
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("conjunction"),
                    allowed: vec!["And".to_owned(), "Or".to_owned(), "Plus".to_owned()],
                }),
            ));
        let err = validate(&group).expect_err("Plus is admitted but no form accepts it");
        assert!(codes(err).contains(&"EC023"));
    }

    #[test]
    fn contradictory_requirements_are_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::All(vec![
                    crate::model::Predicate::In {
                        path: crate::model::FieldPath::call_site("conjunction"),
                        allowed: vec!["And".to_owned()],
                    },
                    crate::model::Predicate::In {
                        path: crate::model::FieldPath::call_site("conjunction"),
                        allowed: vec!["Or".to_owned()],
                    },
                ])),
            ));
        let err = validate(&group).expect_err("And ∩ Or is empty");
        assert!(codes(err).contains(&"EC030"));
    }

    #[test]
    fn any_arm_intersects_with_a_sibling_constraint_on_the_same_path() {
        // All[ In{conjunction: {Plus}}, Any[ In{conjunction: {And}}, In{conjunction:
        // {Or}} ] ] True set is {Plus} ∩ {And, Or} = empty: contradictory.
        // Before the fix, the Any arm's `into.allowed.insert(path, union)`
        // overwrote the sibling `In{Plus}` entry instead of intersecting with
        // it, yielding the non-empty {And, Or} and silently missing EC030.
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::All(vec![
                    crate::model::Predicate::In {
                        path: crate::model::FieldPath::call_site("conjunction"),
                        allowed: vec!["Plus".to_owned()],
                    },
                    crate::model::Predicate::Any(vec![
                        crate::model::Predicate::In {
                            path: crate::model::FieldPath::call_site("conjunction"),
                            allowed: vec!["And".to_owned()],
                        },
                        crate::model::Predicate::In {
                            path: crate::model::FieldPath::call_site("conjunction"),
                            allowed: vec!["Or".to_owned()],
                        },
                    ]),
                ])),
            ));
        let err = validate(&group).expect_err("{Plus} ∩ {And, Or} is empty");
        assert!(codes(err).contains(&"EC030"));
    }

    #[test]
    fn any_arm_does_not_fabricate_a_contradiction_when_the_true_set_is_nonempty() {
        // All[ In{conjunction: {And}}, Any[ In{conjunction: {And}}, In{conjunction:
        // {Or}} ] ] True set is {And} ∩ {And, Or} = {And}: non-empty, so EC030
        // must not fire. This is the case a naive `union.unwrap_or_default()`
        // fix would break: treating a `None` union (no finite constraint
        // from that Any branch) as the empty set would intersect a real
        // constraint down to nothing and fabricate EC030.
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::All(vec![
                    crate::model::Predicate::In {
                        path: crate::model::FieldPath::call_site("conjunction"),
                        allowed: vec!["And".to_owned()],
                    },
                    crate::model::Predicate::Any(vec![
                        crate::model::Predicate::In {
                            path: crate::model::FieldPath::call_site("conjunction"),
                            allowed: vec!["And".to_owned()],
                        },
                        crate::model::Predicate::In {
                            path: crate::model::FieldPath::call_site("conjunction"),
                            allowed: vec!["Or".to_owned()],
                        },
                    ]),
                ])),
            ));
        validate(&group).expect("{And} ∩ {And, Or} = {And} is non-empty; EC030 must not fire");
    }

    #[test]
    fn separate_require_clauses_are_conjunctive_for_contradictions() {
        // `require in(conjunction, {And})` and `require in(conjunction, {Or})`
        // are individually satisfiable but jointly unsatisfiable. Abstracting
        // each clause on its own can never see this; only the conjunction can.
        let mut group = minimal_group();
        for allowed in [vec!["And".to_owned()], vec!["Or".to_owned()]] {
            group.constructions[0]
                .constraints
                .push(crate::model::Constraint::Require(
                    crate::model::Spanned::call_site(crate::model::Predicate::In {
                        path: crate::model::FieldPath::call_site("conjunction"),
                        allowed,
                    }),
                ));
        }
        let err = validate(&group).expect_err("{And} ∩ {Or} across two clauses is empty");
        assert!(codes(err).contains(&"EC030"));
    }

    #[test]
    fn admitted_set_is_the_intersection_of_require_clauses_in_either_order() {
        // Forms cover exactly {And, Or}. The clauses admit {And,Or,Plus} and
        // {And,Or}; their conjunction is {And,Or}, which IS covered, so no
        // EC023 in either clause order. Folding the clauses with `insert`
        // instead of intersecting makes the LAST clause win, so reversing
        // them fabricates an EC023 naming `Plus`.
        let mut group = guarded_two_form_group(conjunction_in(&["And"]), conjunction_in(&["Or"]));
        group.constructions[0]
            .constraints
            .push(requires_conjunction_in(&["And", "Or", "Plus"]));
        group.constructions[0]
            .constraints
            .push(requires_conjunction_in(&["And", "Or"]));

        let mut reversed = group.clone();
        reversed.constructions[0].constraints.reverse();

        validate(&group).expect("{And,Or,Plus} ∩ {And,Or} = {And,Or} is fully covered");
        validate(&reversed).expect("the same conjunction, written in the other order");
    }

    #[test]
    fn a_presence_contradiction_reports_the_authors_path() {
        // `IsSome ∧ IsNone` on one path is unsatisfiable. The path in the
        // message must be the one the author wrote — the presence facet is
        // an internal index, and no author ever wrote `conjunction#some`.
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::All(vec![
                    crate::model::Predicate::IsSome {
                        path: crate::model::FieldPath::call_site("conjunction"),
                    },
                    crate::model::Predicate::IsNone {
                        path: crate::model::FieldPath::call_site("conjunction"),
                    },
                ])),
            ));
        let err = validate(&group).expect_err("present ∧ absent is unsatisfiable");
        assert_eq!(
            message_for(&err, "EC030"),
            "requirements on `conjunction` demand it be both present and absent"
        );
    }

    #[test]
    fn a_presence_requirement_on_its_own_is_satisfiable() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsSome {
                    path: crate::model::FieldPath::call_site("conjunction"),
                }),
            ));
        validate(&group).expect("`is_some` alone constrains nothing to emptiness");
    }

    #[test]
    fn a_presence_coverage_gap_words_the_missing_state() {
        // Required present, but the only form is guarded to absent.
        let mut group = minimal_group();
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::IsNone {
                path: crate::model::FieldPath::call_site("conjunction"),
            },
        ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsSome {
                    path: crate::model::FieldPath::call_site("conjunction"),
                }),
            ));
        let err = validate(&group).expect_err("no form accepts a present `conjunction`");
        assert_eq!(
            message_for(&err, "EC023"),
            "`conjunction` has no accepting form when it is present"
        );
    }

    #[test]
    fn a_length_coverage_gap_reports_the_authors_path() {
        // Required length 2, but the only form is guarded to length 3.
        let mut group = group_with_sequence_field();
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::LenIs {
                path: crate::model::FieldPath::call_site("rest"),
                len: 3,
            },
        ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::LenIs {
                    path: crate::model::FieldPath::call_site("rest"),
                    len: 2,
                }),
            ));
        let err = validate(&group).expect_err("length 2 is admitted but only length 3 has a form");
        assert_eq!(
            message_for(&err, "EC023"),
            "`rest` has no accepting form at length 2"
        );
    }

    #[test]
    fn len_at_least_contributes_no_finite_length_set() {
        // `len >= 2 ∧ len == 3` is satisfiable. Were the open-ended stratum
        // to contribute the finite {2}, intersecting it with {3} would empty
        // the set and fabricate EC030.
        let mut group = group_with_sequence_field();
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::LenIs {
                path: crate::model::FieldPath::call_site("rest"),
                len: 3,
            },
        ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::All(vec![
                    crate::model::Predicate::LenAtLeast {
                        path: crate::model::FieldPath::call_site("rest"),
                        min: 2,
                    },
                    crate::model::Predicate::LenIs {
                        path: crate::model::FieldPath::call_site("rest"),
                        len: 3,
                    },
                ])),
            ));
        validate(&group).expect("`len >= 2` and `len == 3` are jointly satisfiable at 3");
    }

    #[test]
    fn unknown_combinators_are_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::DeriveFeature {
                target: crate::model::FieldPath::call_site("conjunction"),
                combinator: crate::model::Spanned::call_site("summon_grammar_demon".to_owned()),
                args: vec![],
            });
        let err = validate(&group).expect_err("unknown combinator");
        assert!(codes(err).contains(&"EC031"));
    }

    #[test]
    fn derived_witnesses_name_a_known_combinator() {
        // Same concept as `derive_feature`, different syntactic position:
        // the compiler-addition guarantee is about combinators, not about
        // one arm of one enum.
        let mut group = minimal_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("oxford_comma".to_owned()),
                class: crate::model::WitnessClass::Derived {
                    combinator: crate::model::Spanned::call_site("summon_grammar_demon".to_owned()),
                    args: vec![],
                },
            });
        let err = validate(&group).expect_err("unknown combinator on a derived witness");
        assert_eq!(
            message_for(&err, "EC031"),
            "unknown feature combinator `summon_grammar_demon`; a missing combinator is a compiler addition, never a closure"
        );
    }

    #[test]
    fn derived_witnesses_with_a_known_combinator_pass() {
        let mut group = minimal_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("oxford_comma".to_owned()),
                class: crate::model::WitnessClass::Derived {
                    combinator: crate::model::Spanned::call_site("from_first".to_owned()),
                    args: vec![],
                },
            });
        validate(&group).expect("`from_first` is in KNOWN_COMBINATORS");
    }

    #[test]
    fn known_combinators_pass_check_constraints_cleanly() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::DeriveFeature {
                target: crate::model::FieldPath::call_site("conjunction"),
                combinator: crate::model::Spanned::call_site("fixed".to_owned()),
                args: vec![],
            });
        validate(&group).expect("`fixed` is in KNOWN_COMBINATORS");
    }

    #[test]
    fn all_errors_are_reported_together() {
        let mut group = minimal_group();
        group.constructions[0].forms[0].surface.clear(); // EC020
        let twin = group.constructions[0].clone();
        group.constructions.push(twin); // EC001
        let err = validate(&group).expect_err("two independent errors");
        let reported = codes(err);
        assert!(reported.contains(&"EC001"));
        assert!(reported.contains(&"EC020"));
    }

    #[test]
    fn diagnostic_order_is_independent_of_declaration_order() {
        let mut group = minimal_group();
        group.constructions[0].forms[0].surface.clear();
        let mut second = group.constructions[0].clone();
        second.id = crate::model::Spanned::call_site("noun_phrase_list_comma".to_owned());
        group.constructions.push(second);

        let mut reversed = group.clone();
        reversed.constructions.reverse();

        let forward = validate(&group).expect_err("both invalid");
        let backward = validate(&reversed).expect_err("both invalid");
        let render = |diags: Vec<crate::diag::Diagnostic>| {
            diags
                .iter()
                .map(|d| format!("{}:{}", d.construction, d.code.as_str()))
                .collect::<Vec<_>>()
        };
        assert_eq!(render(forward), render(backward));
    }

    #[test]
    fn diagnostic_output_is_independent_of_form_and_constraint_order() {
        // The existing order test reverses CONSTRUCTIONS — the one axis that
        // already worked. Forms and require clauses are the axes that broke:
        // guards {And,Or} and {Or} overlap on Or (EC024) and together cover
        // exactly {And,Or}, while the clauses admit {And,Or,Plus} and
        // {And,Or}, whose conjunction {And,Or} IS covered (no EC023).
        // Reversing forms swaps EC024's two names unless they are sorted;
        // reversing clauses fabricates an EC023 on Plus unless the clauses
        // are intersected rather than overwritten.
        let mut group =
            guarded_two_form_group(conjunction_in(&["And", "Or"]), conjunction_in(&["Or"]));
        group.constructions[0]
            .constraints
            .push(requires_conjunction_in(&["And", "Or", "Plus"]));
        group.constructions[0]
            .constraints
            .push(requires_conjunction_in(&["And", "Or"]));

        let mut reversed = group.clone();
        reversed.constructions[0].forms.reverse();
        reversed.constructions[0].constraints.reverse();

        let render = |diags: Vec<crate::diag::Diagnostic>| {
            diags
                .iter()
                .map(|d| format!("{}:{}:{}", d.construction, d.code.as_str(), d.message))
                .collect::<Vec<_>>()
        };
        let forward = render(validate(&group).expect_err("the guards overlap on Or"));
        let backward = render(validate(&reversed).expect_err("the guards overlap on Or"));
        assert_eq!(forward, backward);
        // Pinned exactly, so the two orders cannot agree on a wrong answer.
        assert_eq!(
            forward,
            vec![
                "noun_phrase_coordination:EC024:forms `binary` and `oxford` can both match the same value and no witness discriminates them"
            ]
        );
    }

    #[test]
    fn an_owned_ast_shape_resolves_its_fields() {
        // Every other fixture binds an existing type; this is the only
        // exercise of the `Own` arm of `AstShape::fields()`. Were that arm to
        // hand back no fields, the form's `conjunction` atom would resolve to
        // nothing and EC010 would fire.
        let mut group = minimal_group();
        group.constructions[0].ast = match group.constructions[0].ast.clone() {
            crate::model::AstShape::Bind { fields, .. } => crate::model::AstShape::Own {
                name: crate::model::Spanned::call_site("CoordinatedNounPhrase".to_owned()),
                fields,
            },
            own => own,
        };
        validate(&group).expect("an owned shape's fields resolve exactly like a bound shape's");
    }

    #[test]
    fn edges_leaving_the_group_are_not_cycle_checked_here() {
        let mut group = minimal_group();
        // A reciprocal pair through the external id `noun_phrase_nominal`: if
        // the in-group membership filter were silently dropped, this pair
        // would itself form a two-node cycle and validate would report
        // EC041. Passing here is discriminating proof the filter is applied,
        // not just an unreachable external node that trivially can't cycle.
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_nominal".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
            });
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_nominal".to_owned()),
            });
        validate(&group).expect("external edges are a registry-time concern");
    }
}
