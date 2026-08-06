//! Layer-2 validation. `ValidatedGroup` is deliberately the ONLY door to
//! the emitter: its constructor is private to this module, so unvalidated
//! emission is unrepresentable.

use crate::diag::DiagCode;
use crate::diag::Diagnostic;
use crate::diag::sort_key;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::ElementDeclaration;
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
    #[must_use]
    pub fn group(&self) -> &'a GroupDeclaration {
        self.group
    }
}

/// # Errors
///
/// Returns every accumulated [`Diagnostic`] (sorted by [`sort_key`]) when
/// `group` fails any layer-2 check. The families, in the order [`checks`]
/// runs them: duplicate identities, distinct declared names colliding on
/// one generated identifier, dominance cycles, bad paths, kind mismatches,
/// malformed forms, an uncovered surface domain, contradictory constraints,
/// and stratum violations (a discourse-occurrence feature in a declaration,
/// or a free witness carrying a type outside the surface-witness stratum).
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
    check_element_shapes(group, diags);
    check_generated_names(group, diags);
    check_empty_bound_elements(group, diags);
    check_dominance_cycles(group, diags);
    check_paths(group, diags);
    check_kinds(group, diags);
    check_forms(group, diags);
    check_fallback_contract(group, diags);
    check_surface_domain(group, diags);
    check_constraints(group, diags);
    check_strata(group, diags);
}

/// Opaque whole-value predicates cannot prove their own complement. Canonical
/// selection is therefore total only when they have one unconditional
/// fallback, and source order is non-semantic only when that fallback is
/// unique. Explicit ordinal replay remains available for every form.
fn check_fallback_contract(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        let mut fallbacks = construction
            .forms
            .iter()
            .filter(|form| form.fallback)
            .collect::<Vec<_>>();

        if construction
            .forms
            .iter()
            .any(|form| form.value_guard.is_some())
            && fallbacks.is_empty()
        {
            let span = construction
                .forms
                .iter()
                .find_map(|form| form.value_guard.as_ref())
                .expect("a value-guarded form supplied the predicate span")
                .span;
            diags.push(
                Diagnostic::new(
                    DiagCode::UncoveredValueSpace,
                    id,
                    "opaque value-guarded forms require exactly one unguarded `otherwise` form for total canonical selection",
                )
                .with_span(span),
            );
        }

        if fallbacks.len() > 1 {
            fallbacks.sort_by_key(|form| form.name.value.as_str());
            let names = fallbacks
                .iter()
                .map(|form| format!("`{}`", form.name.value))
                .collect::<Vec<_>>()
                .join(", ");
            diags.push(
                Diagnostic::new(
                    DiagCode::AmbiguousLinearization,
                    id,
                    format!(
                        "canonical fallback must be unique; forms {names} are all marked `otherwise`"
                    ),
                )
                .with_span(fallbacks[0].name.span)
                .with_note("another fallback is here", fallbacks[1].name.span),
            );
        }

        for fallback in fallbacks {
            let guard_span = fallback
                .guard
                .as_ref()
                .map(|guard| guard.span)
                .or_else(|| fallback.value_guard.as_ref().map(|guard| guard.span));
            if let Some(span) = guard_span {
                diags.push(
                    Diagnostic::new(
                        DiagCode::AmbiguousLinearization,
                        id,
                        format!(
                            "canonical fallback form `{}` must be unguarded; `otherwise` already means the complement of every guarded form",
                            fallback.name.value
                        ),
                    )
                    .with_span(span),
                );
            }
        }
    }
}

fn check_identity(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    let mut seen_elements: std::collections::HashMap<&str, proc_macro2::Span> =
        std::collections::HashMap::new();
    for element in &group.elements {
        match seen_elements.entry(element.name.value.as_str()) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(element.name.span);
            }
            std::collections::hash_map::Entry::Occupied(first) => {
                diags.push(
                    Diagnostic::group(
                        DiagCode::DuplicateName,
                        format!(
                            "element `{}` is declared more than once in this group",
                            element.name.value
                        ),
                    )
                    .with_span(element.name.span)
                    .with_note("first declared here", *first.get()),
                );
            }
        }
        check_element_identity(element, diags);
    }

    let mut seen_ids: std::collections::HashMap<&str, proc_macro2::Span> =
        std::collections::HashMap::new();
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        match seen_ids.entry(id) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(construction.id.span);
            }
            std::collections::hash_map::Entry::Occupied(slot) => {
                diags.push(
                    Diagnostic::new(
                        DiagCode::DuplicateConstructionId,
                        id,
                        format!("construction id `{id}` is declared more than once in this group"),
                    )
                    .with_span(construction.id.span)
                    .with_note("first declared here", *slot.get()),
                );
            }
        }
        check_construction_identity(construction, diags);
    }
}

fn check_element_identity(element: &ElementDeclaration, diags: &mut Vec<Diagnostic>) {
    let mut seen_variants: std::collections::HashMap<&str, proc_macro2::Span> =
        std::collections::HashMap::new();
    for variant in &element.variants {
        match seen_variants.entry(variant.name.value.as_str()) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(variant.name.span);
            }
            std::collections::hash_map::Entry::Occupied(first) => {
                diags.push(
                    Diagnostic::group(
                        DiagCode::DuplicateName,
                        format!(
                            "variant name `{}` is declared more than once in element `{}`",
                            variant.name.value, element.name.value
                        ),
                    )
                    .with_span(variant.name.span)
                    .with_note("first declared here", *first.get()),
                );
            }
        }
    }
    for field in &element.fields {
        if matches!(field.kind, FieldKind::SurfaceScalar { .. }) && element.bind_path.is_none() {
            diags.push(
                Diagnostic::group(
                    DiagCode::InvalidElementShape,
                    format!(
                        "surface-only field `{}.{}` requires a bound semantic element",
                        element.name.value, field.field.value,
                    ),
                )
                .with_span(field.field.span),
            );
        }
    }
}

fn check_construction_identity(
    construction: &ConstructionDeclaration,
    diags: &mut Vec<Diagnostic>,
) {
    let id = construction.id.value.as_str();
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
    check_named_construction_items(
        construction,
        construction.forms.iter().map(|form| &form.name),
        "form",
        diags,
    );
    check_named_construction_items(
        construction,
        construction.witnesses.iter().map(|witness| &witness.name),
        "witness",
        diags,
    );
    if construction.deserialize
        && let crate::model::AstShape::Bind { .. } = &construction.ast
    {
        diags.push(
            Diagnostic::new(
                DiagCode::DeserializeRequiresOwn,
                id,
                "`deserialize` requires own mode; a bind construction has no generated type to deserialize into",
            )
            .with_span(construction.id.span),
        );
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

fn check_named_construction_items<'a>(
    construction: &ConstructionDeclaration,
    names: impl Iterator<Item = &'a crate::model::Spanned<String>>,
    kind: &str,
    diags: &mut Vec<Diagnostic>,
) {
    let mut seen: std::collections::HashMap<&str, proc_macro2::Span> =
        std::collections::HashMap::new();
    for name in names {
        match seen.entry(name.value.as_str()) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(name.span);
            }
            std::collections::hash_map::Entry::Occupied(first) => {
                diags.push(
                    Diagnostic::new(
                        DiagCode::DuplicateName,
                        construction.id.value.as_str(),
                        format!("{kind} name `{}` is declared more than once", name.value),
                    )
                    .with_span(name.span)
                    .with_note("first declared here", *first.get()),
                );
            }
        }
    }
}

fn check_element_shapes(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for element in &group.elements {
        if !element.variants.is_empty() && element.bind_path.is_none() {
            diags.push(
                Diagnostic::group(
                    DiagCode::InvalidElementShape,
                    format!(
                        "element `{}` declares enum variants but has no `bind` target",
                        element.name.value
                    ),
                )
                .with_span(element.variants[0].name.span),
            );
        }
        if element.bind_path.is_some() && !element.fields.is_empty() && !element.variants.is_empty()
        {
            diags.push(
                Diagnostic::group(
                    DiagCode::InvalidElementShape,
                    format!(
                        "bound element `{}` mixes struct fields and enum variants; declare exactly one mapping shape",
                        element.name.value
                    ),
                )
                .with_span(element.variants[0].name.span),
            );
        }
        for variant in &element.variants {
            if !matches!(variant.payload, FieldKind::Subtree { .. }) {
                diags.push(
                    Diagnostic::group(
                        DiagCode::InvalidElementVariant,
                        format!(
                            "variant `{}::{}` payload must be `hole TYPE` or `hole box TYPE`",
                            element.name.value, variant.name.value
                        ),
                    )
                    .with_span(variant.name.span),
                );
            }
        }
    }
    for construction in &group.constructions {
        for field in construction.ast.fields() {
            if matches!(field.kind, FieldKind::SurfaceScalar { .. }) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::InvalidElementShape,
                        &construction.id.value,
                        format!(
                            "surface-only field `{}` is valid only inside a bound element",
                            field.field.value,
                        ),
                    )
                    .with_span(field.field.span),
                );
            }
        }
    }
}

/// EC006 — covers every declaration-derived top-level identifier
/// the emitter mints into the group's generated module, and checks that
/// each is unique in its Rust namespace. The type namespace contains owned
/// element structs, bound-enum borrowed-view enums, and own-mode construction
/// structs. The value namespace contains bound-enum parts/build functions and
/// bind-mode construction parts/build functions. This is a DIFFERENT
/// namespace from EC004's checks above:
/// EC004 catches two elements/forms/witnesses sharing a DECLARED name;
/// EC006 catches two declared names that map to the same GENERATED
/// identifier even though the declared names differ (e.g. two own-mode
/// constructions both naming their type `SameNode`, or an element
/// `foo_bar` colliding with `foo__bar` after `PascalCasing`) — collisions
/// EC004 cannot see because it never looks at the generated identifier.
///
/// `__assert_free_witness_payloads` (Task 1) is a single fixed function
/// name emitted at most once per group — never derived from a declared
/// name — so the two-different-declared-names collision EC006 exists to
/// catch cannot arise for it either.
///
/// Processes owned elements before constructions, matching `emit_group`'s
/// own emission order, so "first declared here" points at whichever one
/// rustc would actually see first. `DeclarationViolation` is deliberately not a
/// reserved name here: it now lives once in `runtime.rs`, not minted per
/// group, so it is no longer part of this namespace.
fn check_generated_names(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    let mut seen_types: std::collections::HashMap<
        String,
        (proc_macro2::Span, &'static str, String),
    > = std::collections::HashMap::new();
    for element in &group.elements {
        let (generated, kind) = if element.bind_path.is_none() {
            (crate::model::pascal_case(&element.name.value), "element")
        } else if !element.variants.is_empty() {
            (
                format!(
                    "{}VariantRef",
                    crate::model::pascal_case(&element.name.value)
                ),
                "bound-enum element view",
            )
        } else {
            continue;
        };
        record_generated_name(
            &mut seen_types,
            diags,
            &generated,
            element.name.span,
            kind,
            element.name.value.clone(),
        );
    }
    for construction in &group.constructions {
        if let crate::model::AstShape::Own { name, .. } = &construction.ast {
            record_generated_name(
                &mut seen_types,
                diags,
                &name.value,
                name.span,
                "own-mode construction",
                construction.id.value.clone(),
            );
        }
    }

    let mut seen_values: std::collections::HashMap<
        String,
        (proc_macro2::Span, &'static str, String),
    > = std::collections::HashMap::new();
    for element in group
        .elements
        .iter()
        .filter(|element| !element.variants.is_empty())
    {
        let parts = format!("parts_{}", element.name.value);
        record_generated_name(
            &mut seen_values,
            diags,
            &parts,
            element.name.span,
            "bound-enum element destructurer",
            element.name.value.clone(),
        );
        for variant in &element.variants {
            let builder = format!(
                "build_{}_{}",
                element.name.value,
                crate::model::snake_case(&variant.name.value),
            );
            record_generated_name(
                &mut seen_values,
                diags,
                &builder,
                variant.name.span,
                "bound-enum variant builder",
                format!("{}::{}", element.name.value, variant.name.value),
            );
        }
    }
    for construction in &group.constructions {
        let linearizer = format!("linearize_{}_with", construction.id.value);
        record_generated_name(
            &mut seen_values,
            diags,
            &linearizer,
            construction.id.span,
            "construction linearizer",
            construction.id.value.clone(),
        );
        if let crate::model::AstShape::Bind { .. } = &construction.ast {
            for prefix in ["build", "parts"] {
                let generated = format!("{prefix}_{}", construction.id.value);
                record_generated_name(
                    &mut seen_values,
                    diags,
                    &generated,
                    construction.id.span,
                    "bind-mode construction function",
                    construction.id.value.clone(),
                );
            }
        }
    }
}

/// EC007 — an empty bound mapping is intentionally opaque: the compiler can
/// prove that its target type exists, but cannot inspect or construct a value.
/// A direct construction sequence can carry the exact invariant that it is
/// always empty. Optional sequences and sequences nested in element fields
/// have no addressable predicate path in today's DSL, so they are rejected.
fn check_empty_bound_elements(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        for binding in construction.ast.fields() {
            let mut references = Vec::new();
            collect_sequence_references(&binding.kind, true, &mut references);
            for (element, is_direct) in references {
                if !is_empty_bound_element(group, &element.value) {
                    continue;
                }
                let directly_empty = is_direct
                    && has_direct_zero_length_requirement(construction, &binding.field.value);
                if directly_empty {
                    continue;
                }
                diags.push(
                    Diagnostic::new(
                        DiagCode::BoundElementMustBeEmpty,
                        &construction.id.value,
                        if is_direct {
                            format!(
                                "sequence field `{}` references empty bound element `{}`; add `require {}.len() == 0`",
                                binding.field.value, element.value, binding.field.value,
                            )
                        } else {
                            format!(
                                "optional sequence field `{}` references empty bound element `{}`; only a direct sequence field with `require {}.len() == 0` can prove it empty",
                                binding.field.value, element.value, binding.field.value,
                            )
                        },
                    )
                    .with_span(element.span),
                );
            }
        }
    }

    for declaration in &group.elements {
        for binding in &declaration.fields {
            let mut references = Vec::new();
            collect_sequence_references(&binding.kind, false, &mut references);
            for (element, _) in references {
                if !is_empty_bound_element(group, &element.value) {
                    continue;
                }
                diags.push(
                    Diagnostic::group(
                        DiagCode::BoundElementMustBeEmpty,
                        format!(
                            "element field `{}.{}` references empty bound element `{}`; element-nested sequences cannot carry a direct zero-length requirement",
                            declaration.name.value, binding.field.value, element.value,
                        ),
                    )
                    .with_span(element.span),
                );
            }
        }
    }
}

fn collect_sequence_references<'a>(
    kind: &'a FieldKind,
    is_direct: bool,
    into: &mut Vec<(&'a crate::model::Spanned<String>, bool)>,
) {
    match kind {
        FieldKind::Sequence { element } => into.push((element, is_direct)),
        FieldKind::Optional { inner } => collect_sequence_references(inner, false, into),
        FieldKind::Identity { .. }
        | FieldKind::Subtree { .. }
        | FieldKind::Scalar { .. }
        | FieldKind::SurfaceScalar { .. } => {}
    }
}

fn is_empty_bound_element(group: &GroupDeclaration, name: &str) -> bool {
    group.elements.iter().any(|candidate| {
        candidate.name.value == name
            && candidate.bind_path.is_some()
            && candidate.fields.is_empty()
            && candidate.variants.is_empty()
    })
}

fn has_direct_zero_length_requirement(
    construction: &ConstructionDeclaration,
    field_name: &str,
) -> bool {
    construction.constraints.iter().any(|constraint| {
        let Constraint::Require(requirement) = constraint else { return false };
        let Predicate::LenIs { path, len: 0 } = &requirement.value else {
            return false;
        };
        matches!(path.segments.as_slice(), [only] if only.value == field_name)
    })
}

fn recognition_predicate(constraint: &Constraint) -> Option<&crate::model::Spanned<Predicate>> {
    match constraint {
        Constraint::Require(predicate) | Constraint::Recognize(predicate) => Some(predicate),
        Constraint::DeriveFeature { .. } => None,
    }
}

fn record_generated_name(
    seen: &mut std::collections::HashMap<String, (proc_macro2::Span, &'static str, String)>,
    diags: &mut Vec<Diagnostic>,
    generated: &str,
    span: proc_macro2::Span,
    kind: &'static str,
    declared: String,
) {
    match seen.entry(generated.to_string()) {
        std::collections::hash_map::Entry::Vacant(slot) => {
            slot.insert((span, kind, declared));
        }
        std::collections::hash_map::Entry::Occupied(first) => {
            let (first_span, first_kind, first_declared) = first.get().clone();
            // An exact duplicate declaration in the same generated-item
            // family already has its identity diagnostic (EC001 or EC004).
            // EC006 exists for distinct declarations that normalize to one
            // Rust identifier, so do not report the same root cause twice.
            if kind == first_kind && declared == first_declared {
                return;
            }
            diags.push(
                Diagnostic::group(
                    DiagCode::GeneratedNameCollision,
                    format!(
                        "the generated identifier `{generated}` is used by both this {kind} and a previously declared {first_kind}"
                    ),
                )
                .with_span(span)
                .with_note("first declared here", first_span),
            );
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

#[derive(Debug, Clone, Copy)]
enum Resolved<'g> {
    Kind(&'g FieldKind),
    /// The path ends on `last`: the sequence's element, as a whole. The
    /// emitter needs the element's identity to type a hole; a bare
    /// `FieldKind` cannot name it (Milestone-1 gap 2).
    #[allow(
        dead_code,
        reason = "field read only by #[cfg(test)] matches so far; Task 8's emitter adds the first production reader"
    )]
    Element(&'g ElementDeclaration),
    /// The synthetic discriminant of a bound enum sequence element. It is
    /// addressable only as `<sequence>.(first|last|nonfinal).variant` and is
    /// compared by `in [...]` against the variants declared for that element.
    Variant,
}

#[derive(Debug, Clone, Copy)]
struct BadSegment {
    /// Index into `path.segments` of the first segment that failed to
    /// resolve. A sequence selector with no sequence in hand blames that
    /// selector segment.
    index: usize,
}

/// Resolves a path against a construction's ast fields and, through
/// Sequence fields, the group's element declarations. `last` addresses one
/// first/last address one endpoint and `nonfinal` quantifies the all-but-last
/// prefix; all three enter the sequence element declaration and consume that
/// context. `Optional` is terminal: no segment may follow it.
fn resolve_path<'g>(
    group: &'g GroupDeclaration,
    construction: &'g ConstructionDeclaration,
    path: &FieldPath,
) -> Result<Resolved<'g>, BadSegment> {
    let mut fields: &'g [FieldBinding] = construction.ast.fields();
    let mut resolved: Option<Resolved<'g>> = None;
    let mut element_in_hand: Option<&'g ElementDeclaration> = None;
    for (index, segment) in path.segments.iter().enumerate() {
        let bad = BadSegment { index };
        if matches!(segment.value.as_str(), "first" | "last" | "nonfinal") {
            if let Some(Resolved::Kind(FieldKind::Sequence { element })) = resolved {
                let declared = group
                    .elements
                    .iter()
                    .find(|e| e.name.value == element.value)
                    .ok_or(bad)?;
                resolved = Some(Resolved::Element(declared));
                element_in_hand = Some(declared);
                continue;
            }
            if resolved.is_some() {
                return Err(bad);
            }
        }
        if let Some(element) = element_in_hand.take() {
            if segment.value == "variant" && !element.variants.is_empty() {
                resolved = Some(Resolved::Variant);
                continue;
            }
            fields = &element.fields;
        } else if resolved.is_some() {
            // A named segment can only follow the construction root or an
            // element entered via `last`/`nonfinal`; scalars, subtrees,
            // optionals and unselected sequences have no addressable children.
            return Err(bad);
        }
        let binding = fields
            .iter()
            .find(|b| b.field.value == segment.value)
            .ok_or(bad)?;
        resolved = Some(Resolved::Kind(&binding.kind));
    }
    resolved.ok_or(BadSegment { index: 0 })
}

fn check_paths(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        check_construction_paths(group, construction, diags);
    }
}

fn check_construction_paths(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    diags: &mut Vec<Diagnostic>,
) {
    let id = construction.id.value.as_str();
    let field_names: std::collections::HashSet<&str> = construction
        .ast
        .fields()
        .iter()
        .map(|b| b.field.value.as_str())
        .collect();

    for binding in construction.ast.fields() {
        if let FieldKind::Sequence { element } = &binding.kind
            && !group.elements.iter().any(|e| e.name.value == element.value)
        {
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
    check_form_paths(group, construction, diags);
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
            if let Some(segment) = path
                .segments
                .iter()
                .find(|segment| segment.value == "nonfinal")
            {
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownFieldPath,
                        id,
                        format!(
                            "stored witness `{}` names `{}`: `nonfinal` is predicate-only",
                            witness.name.value,
                            path.dotted(),
                        ),
                    )
                    .with_span(segment.span),
                );
            } else if let Err(bad) = resolve_path(group, construction, path) {
                let segment = &path.segments[bad.index];
                diags.push(
                    Diagnostic::new(
                        DiagCode::StoredWitnessPathUnknown,
                        id,
                        format!(
                            "stored witness `{}` names `{}`: `{}` does not resolve",
                            witness.name.value,
                            path.dotted(),
                            segment.value
                        ),
                    )
                    .with_span(segment.span),
                );
            }
        }
    }
    // Require-clause and form-guard predicate paths are as unresolvable
    // as any surface/witness path — `DeriveFeature`'s target/args and
    // `WitnessClass::Derived`'s args are deliberately NOT walked here;
    // both are recorded deferrals for a later milestone.
    for constraint in &construction.constraints {
        let Some(predicate) = recognition_predicate(constraint) else {
            continue;
        };
        let mut paths: Vec<&FieldPath> = Vec::new();
        collect_all_paths(&predicate.value, &mut paths);
        for path in paths {
            if let Err(bad) = resolve_path(group, construction, path) {
                let segment = &path.segments[bad.index];
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownFieldPath,
                        id,
                        format!(
                            "require clause references `{}`: `{}` does not resolve",
                            path.dotted(),
                            segment.value
                        ),
                    )
                    .with_span(segment.span),
                );
            }
        }
    }
    for form in &construction.forms {
        let Some(guard) = &form.guard else { continue };
        let mut paths: Vec<&FieldPath> = Vec::new();
        collect_all_paths(&guard.value, &mut paths);
        for path in paths {
            if let Err(bad) = resolve_path(group, construction, path) {
                let segment = &path.segments[bad.index];
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownFieldPath,
                        id,
                        format!(
                            "form `{}` guard references `{}`: `{}` does not resolve",
                            form.name.value,
                            path.dotted(),
                            segment.value
                        ),
                    )
                    .with_span(segment.span),
                );
            }
        }
    }
}

fn check_form_paths(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    diags: &mut Vec<Diagnostic>,
) {
    let id = construction.id.value.as_str();
    for form in &construction.forms {
        for atom in &form.surface {
            let path = match atom {
                SurfaceAtom::Hole(path)
                | SurfaceAtom::Lexeme(path)
                | SurfaceAtom::Identity(path) => path,
                SurfaceAtom::Literal(_) => continue,
            };
            if let Some(segment) = path
                .segments
                .iter()
                .find(|segment| segment.value == "nonfinal")
            {
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownFieldPath,
                        id,
                        format!(
                            "form `{}` references `{}`: `nonfinal` is predicate-only",
                            form.name.value,
                            path.dotted(),
                        ),
                    )
                    .with_span(segment.span),
                );
                continue;
            }
            if let Err(bad) = resolve_path(group, construction, path) {
                let segment = &path.segments[bad.index];
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnknownFieldPath,
                        id,
                        format!(
                            "form `{}` references `{}`: `{}` does not resolve",
                            form.name.value,
                            path.dotted(),
                            segment.value
                        ),
                    )
                    .with_span(segment.span),
                );
            }
        }
    }
}

fn check_kinds(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        // EC014 — surface atoms must agree with the kind they resolve to.
        for form in &construction.forms {
            for atom in &form.surface {
                let (path, atom_kind) = match atom {
                    SurfaceAtom::Hole(path) => (path, "hole"),
                    SurfaceAtom::Lexeme(path) => (path, "lexeme"),
                    SurfaceAtom::Identity(path) => (path, "identity"),
                    SurfaceAtom::Literal(_) => continue,
                };
                let Ok(resolved) = resolve_path(group, construction, path) else {
                    continue; // EC010 already reported it
                };
                let is_scalar = resolved_is_scalar(&resolved);
                let is_identity = matches!(resolved, Resolved::Kind(FieldKind::Identity { .. }));
                if atom_kind == "identity" && !is_identity {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::SurfaceKindMismatch,
                            id,
                            format!(
                                "form `{}`: `{}` is not an identity; only identity fields render with identity(…)",
                                form.name.value,
                                path.dotted()
                            ),
                        )
                        .with_span(path.span),
                    );
                }
                if atom_kind == "lexeme" && !is_scalar {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::SurfaceKindMismatch,
                            id,
                            format!(
                                "form `{}`: `{}` is not a scalar; only scalars render with lex(…)",
                                form.name.value,
                                path.dotted()
                            ),
                        )
                        .with_span(path.span),
                    );
                }
                if atom_kind == "hole" && is_scalar {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::SurfaceKindMismatch,
                            id,
                            format!(
                                "form `{}`: `{}` is a scalar; write lex({}) to render it",
                                form.name.value,
                                path.dotted(),
                                path.dotted()
                            ),
                        )
                        .with_span(path.span),
                    );
                }
                if atom_kind == "hole" && is_identity {
                    diags.push(
                        Diagnostic::new(
                            DiagCode::SurfaceKindMismatch,
                            id,
                            format!(
                                "form `{}`: `{}` is an identity; write identity({}) to render it",
                                form.name.value,
                                path.dotted(),
                                path.dotted()
                            ),
                        )
                        .with_span(path.span),
                    );
                }
            }
        }
        // EC011 — presence predicates need an Optional target. Walk every
        // predicate the construction holds: require clauses and form guards.
        let mut presence_paths: Vec<&FieldPath> = Vec::new();
        for constraint in &construction.constraints {
            if let Some(predicate) = recognition_predicate(constraint) {
                collect_presence_paths(&predicate.value, &mut presence_paths);
            }
        }
        for form in &construction.forms {
            if let Some(guard) = &form.guard {
                collect_presence_paths(&guard.value, &mut presence_paths);
            }
        }
        for path in presence_paths {
            let Ok(resolved) = resolve_path(group, construction, path) else {
                continue; // EC010 already reported it (check_paths runs first)
            };
            if !matches!(resolved, Resolved::Kind(FieldKind::Optional { .. })) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::PresenceOnNonOptional,
                        id,
                        format!(
                            "`{}` is not optional, so is_some()/is_none() cannot constrain it",
                            path.dotted()
                        ),
                    )
                    .with_span(path.span),
                );
            }
        }
        // EC015 — In predicates need a scalar target and len() predicates
        // need a sequence target. Same two sources as EC011: require clauses
        // and form guards.
        let mut in_paths: Vec<&FieldPath> = Vec::new();
        let mut len_paths: Vec<&FieldPath> = Vec::new();
        for constraint in &construction.constraints {
            if let Some(predicate) = recognition_predicate(constraint) {
                collect_in_paths(&predicate.value, &mut in_paths);
                collect_len_paths(&predicate.value, &mut len_paths);
            }
        }
        for form in &construction.forms {
            if let Some(guard) = &form.guard {
                collect_in_paths(&guard.value, &mut in_paths);
                collect_len_paths(&guard.value, &mut len_paths);
            }
        }
        for path in in_paths {
            let Ok(resolved) = resolve_path(group, construction, path) else {
                continue; // EC010 already reported it (check_paths runs first)
            };
            if let Some(problem) = in_predicate_kind_problem(&resolved) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::PredicateKindMismatch,
                        id,
                        format!("`{}` {problem}", path.dotted()),
                    )
                    .with_span(path.span),
                );
            }
        }
        for path in len_paths {
            let Ok(resolved) = resolve_path(group, construction, path) else {
                continue; // EC010 already reported it (check_paths runs first)
            };
            if !matches!(resolved, Resolved::Kind(FieldKind::Sequence { .. })) {
                diags.push(
                    Diagnostic::new(
                        DiagCode::PredicateKindMismatch,
                        id,
                        format!(
                            "`{}` is not a sequence; len() constrains sequence fields",
                            path.dotted()
                        ),
                    )
                    .with_span(path.span),
                );
            }
        }
        // EC032 — require paths must be emitter-shallow, with one staged
        // widening: a path through a sequence's `last` or `nonfinal` segment
        // to an element scalar (or optional-scalar) field. The emitter
        // compiles `last` vacuously true when empty and `nonfinal` as `all`
        // over the all-but-last prefix. `resolve_path` only ever resolves a
        // multi-segment path through one of those explicit sequence-element
        // selectors, so admission here just narrows the resolved
        // FINAL kind to scalar/optional-scalar; a resolved-but-wrong-kind
        // path (e.g. `last` onto a Subtree field) still falls through to
        // the staged message. A predicate/kind mismatch on an ADMITTED path
        // (`len()` through `last` onto a scalar) is left to EC011/EC015,
        // which already resolve deep paths via `resolve_path` themselves.
        for constraint in &construction.constraints {
            let Some(predicate) = recognition_predicate(constraint) else {
                continue;
            };
            let mut deep: Vec<&FieldPath> = Vec::new();
            collect_all_paths(&predicate.value, &mut deep);
            for path in deep {
                if path.segments.len() <= 1 {
                    continue;
                }
                if let Ok(resolved) = resolve_path(group, construction, path)
                    && resolved_is_scalar(&resolved)
                {
                    continue;
                }
                diags.push(
                    Diagnostic::new(
                        DiagCode::UnsupportedConstraintPath,
                        id,
                        format!(
                            "`{}`: generated try_new checks support single-field paths; deeper constraint paths land when a family needs them",
                            path.dotted()
                        ),
                    )
                    .with_span(path.span),
                );
            }
        }
    }
}

/// Scalar for SURFACE purposes: a codec field, optional or not — the thing
/// `lex(…)` renders and a hole must not consume. **EC014, and also EC032's
/// sequence-selector admission gate below** (the same "codec field, optional
/// or not" question decides which element field a `.last` or `.nonfinal`
/// require path may target).
/// `lex(opt field)` is perfectly renderable, so an `Optional { Scalar }`
/// counts as scalar here.
///
/// Do NOT reuse this for EC015. EC015's `In` predicate compiles to a
/// `matches!` pattern that must match the FIELD'S ACTUAL RUST TYPE, and
/// `matches!(field, A | B)` against an `Option<Codec>`-typed field does not
/// type-check — "scalar for surface purposes" and "scalar for pattern-match
/// purposes" are different questions that happen to agree everywhere except
/// `Optional`, which is exactly the case that mattered. See
/// `in_predicate_kind_problem`, EC015's own predicate, below.
fn resolved_is_scalar(resolved: &Resolved<'_>) -> bool {
    match resolved {
        Resolved::Kind(FieldKind::Scalar { .. }) | Resolved::Variant => true,
        Resolved::Kind(FieldKind::Optional { inner }) => {
            matches!(**inner, FieldKind::Scalar { .. })
        }
        Resolved::Kind(
            FieldKind::Identity { .. }
            | FieldKind::Subtree { .. }
            | FieldKind::Sequence { .. }
            | FieldKind::SurfaceScalar { .. },
        )
        | Resolved::Element(_) => false,
    }
}

/// Scalar for `In`-PATTERN purposes (EC015 only — do not reuse for EC014's
/// `resolved_is_scalar` above, which answers a different question). `In`
/// emits `matches!(field, Codec::A | Codec::B)`, so the target must be a
/// non-generic scalar codec, optional or not:
///
/// - `Optional { Scalar }` is admitted under the adopted ruling: `f in [A, B]`
///   on an optional field reads as "absent, or present and in the set"
///   (`matches!(f, None | Some(A | B))`) — vacuously true when absent. This is
///   the reading that composes: an author who wants strictness writes
///   `all(f.is_some(), f in [...])` to recover it. The opposite reading
///   (absence fails) does not compose in the other direction, so it is not the
///   one adopted.
/// - A generic codec (`Wrapper<Inner>`) is rejected because `Wrapper<Inner>::A`
///   is not legal Rust in pattern position — turbofish would work but a second
///   special case buys nothing EC015's one rule ("`In` targets a plain scalar
///   codec") doesn't already give for free.
///
/// Returns the diagnostic text to render after the backtick-quoted path
/// (`None` means "no problem").
fn in_predicate_kind_problem(resolved: &Resolved<'_>) -> Option<&'static str> {
    let codec = match resolved {
        Resolved::Kind(FieldKind::Scalar { codec }) => codec,
        Resolved::Kind(FieldKind::Optional { inner }) => match &**inner {
            FieldKind::Scalar { codec } => codec,
            FieldKind::Optional { .. }
            | FieldKind::Identity { .. }
            | FieldKind::Subtree { .. }
            | FieldKind::Sequence { .. }
            | FieldKind::SurfaceScalar { .. } => {
                return Some(
                    "is not a scalar; `in [...]` compares a scalar codec against its variants",
                );
            }
        },
        Resolved::Kind(
            FieldKind::Identity { .. }
            | FieldKind::Subtree { .. }
            | FieldKind::Sequence { .. }
            | FieldKind::SurfaceScalar { .. },
        )
        | Resolved::Element(_) => {
            return Some(
                "is not a scalar; `in [...]` compares a scalar codec against its variants",
            );
        }
        Resolved::Variant => return None,
    };
    if codec.value.contains('<') {
        Some("has a generic codec; generic codecs are not supported in `in [...]` predicates")
    } else {
        None
    }
}

fn collect_presence_paths<'p>(predicate: &'p Predicate, into: &mut Vec<&'p FieldPath>) {
    match predicate {
        Predicate::IsSome { path } | Predicate::IsNone { path } => into.push(path),
        Predicate::All(children) | Predicate::Any(children) => {
            for child in children {
                collect_presence_paths(child, into);
            }
        }
        Predicate::LenAtLeast { .. } | Predicate::LenIs { .. } | Predicate::In { .. } => {}
    }
}

fn collect_in_paths<'p>(predicate: &'p Predicate, into: &mut Vec<&'p FieldPath>) {
    match predicate {
        Predicate::In { path, .. } => into.push(path),
        Predicate::All(children) | Predicate::Any(children) => {
            for child in children {
                collect_in_paths(child, into);
            }
        }
        Predicate::IsSome { .. }
        | Predicate::IsNone { .. }
        | Predicate::LenAtLeast { .. }
        | Predicate::LenIs { .. } => {}
    }
}

fn collect_len_paths<'p>(predicate: &'p Predicate, into: &mut Vec<&'p FieldPath>) {
    match predicate {
        Predicate::LenAtLeast { path, .. } | Predicate::LenIs { path, .. } => into.push(path),
        Predicate::All(children) | Predicate::Any(children) => {
            for child in children {
                collect_len_paths(child, into);
            }
        }
        Predicate::IsSome { .. } | Predicate::IsNone { .. } | Predicate::In { .. } => {}
    }
}

fn collect_all_paths<'p>(predicate: &'p Predicate, into: &mut Vec<&'p FieldPath>) {
    match predicate {
        Predicate::IsSome { path }
        | Predicate::IsNone { path }
        | Predicate::LenAtLeast { path, .. }
        | Predicate::LenIs { path, .. }
        | Predicate::In { path, .. } => into.push(path),
        Predicate::All(children) | Predicate::Any(children) => {
            for child in children {
                collect_all_paths(child, into);
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
                    SurfaceAtom::Hole(path)
                    | SurfaceAtom::Lexeme(path)
                    | SurfaceAtom::Identity(path) => path,
                    SurfaceAtom::Literal(_) => continue,
                };
                let dotted = path.dotted();
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
                    .find(|b| Some(&b.field.value) == path.segments.first().map(|s| &s.value))
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

fn abstract_predicate(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    predicate: &Predicate,
) -> Abstraction {
    let mut abstraction = Abstraction::default();
    collect_abstraction(group, construction, predicate, &mut abstraction);
    abstraction
}

/// Whether `path` resolves (against `group`/`construction`) to an optional
/// scalar. Threaded down from `abstract_predicate` so `Predicate::In` can
/// tell an optional target from a non-optional one — the abstraction
/// builder otherwise has no field-kind knowledge of its own. An
/// unresolvable path answers `false`; EC010 (`check_paths`) reports it
/// separately and the abstraction pass just skips the finer question.
fn path_is_optional_scalar(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    path: &FieldPath,
) -> bool {
    matches!(
        resolve_path(group, construction, path),
        Ok(Resolved::Kind(FieldKind::Optional { inner })) if matches!(**inner, FieldKind::Scalar { .. })
    )
}

fn collect_abstraction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    predicate: &Predicate,
    into: &mut Abstraction,
) {
    match predicate {
        Predicate::In { path, allowed } => {
            // Reading B: `f in [...]` on an optional scalar is satisfiable
            // by absence, so it must NOT contribute a finite Value key here.
            // If it did, EC030 could intersect two guards like
            // `All[In{And}]` and `All[In{Or}]` down to the empty set and
            // report "admits no value at all" — a conclusion absence
            // falsifies (both are satisfied by the field being absent).
            // Conservative, like `LenAtLeast`: no key beats a wrong key.
            if !path_is_optional_scalar(group, construction, path) {
                into.intersect(
                    (path.dotted(), Facet::Value),
                    allowed.iter().cloned().collect(),
                );
            }
        }
        Predicate::IsSome { path } => {
            into.intersect(
                (path.dotted(), Facet::Presence),
                std::iter::once("some".to_owned()).collect(),
            );
        }
        Predicate::IsNone { path } => {
            into.intersect(
                (path.dotted(), Facet::Presence),
                std::iter::once("none".to_owned()).collect(),
            );
        }
        Predicate::LenIs { path, len } => {
            into.intersect(
                (path.dotted(), Facet::Len),
                std::iter::once(len.to_string()).collect(),
            );
        }
        Predicate::LenAtLeast { .. } => {
            // Open-ended stratum: contributes no finite set, so it can
            // neither prove disjointness nor emptiness. Conservative.
        }
        Predicate::All(children) => {
            for child in children {
                collect_abstraction(group, construction, child, into);
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
                branch_abstractions.push(abstract_predicate(group, construction, child));
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

fn abstractions_overlap(a: &Abstraction, b: &Abstraction) -> bool {
    for (key, a_values) in &a.allowed {
        if let Some(b_values) = b.allowed.get(key)
            && a_values.intersection(b_values).next().is_none()
        {
            return false; // provably disjoint on this key
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

fn required_facts(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> RequiredFacts {
    let mut admitted = Abstraction::default();
    let mut origin: std::collections::HashMap<(String, Facet), proc_macro2::Span> =
        std::collections::HashMap::new();
    for constraint in &construction.constraints {
        let Some(predicate) = recognition_predicate(constraint) else {
            continue;
        };
        let clause = abstract_predicate(group, construction, &predicate.value);
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
                form.guard.as_ref().map_or_else(Abstraction::default, |g| {
                    abstract_predicate(group, construction, &g.value)
                })
            })
            .collect();
        if construction.forms.len() > 1 && !has_free_witness {
            for left in 0..guards.len() {
                for right in (left + 1)..guards.len() {
                    if construction.forms[left].fallback || construction.forms[right].fallback {
                        continue;
                    }
                    if abstractions_overlap(&guards[left], &guards[right]) {
                        // Sorted: explicit ordinals exist so that form order
                        // is not semantic, so the message must not change
                        // when the two declarations are swapped.
                        let mut named = [
                            construction.forms[left].name.value.as_str(),
                            construction.forms[right].name.value.as_str(),
                        ];
                        named.sort_unstable();
                        let (first_span, second_span) =
                            if named[0] == construction.forms[left].name.value {
                                (
                                    construction.forms[left].name.span,
                                    construction.forms[right].name.span,
                                )
                            } else {
                                (
                                    construction.forms[right].name.span,
                                    construction.forms[left].name.span,
                                )
                            };
                        diags.push(
                            Diagnostic::new(
                                DiagCode::AmbiguousLinearization,
                                id,
                                format!(
                                    "forms `{}` and `{}` can both match the same value and no witness discriminates them",
                                    named[0], named[1]
                                ),
                            )
                            .with_span(first_span)
                            .with_note("the other form is here", second_span),
                        );
                    }
                }
            }
        }
        // Coverage: for every path with a finite admitted set declared by
        // constraints, the union of form-guard sets must reach every value.
        let required = required_facts(group, construction);
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
        let required = required_facts(group, construction);
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
                Constraint::Require(_) | Constraint::Recognize(_) => None,
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

fn stratum_of(type_ident: &str) -> Option<deckmaste_features::FeatureStratum> {
    // The DSL may write a qualified path; classification is by terminal ident.
    let terminal = type_ident.rsplit("::").next().unwrap_or(type_ident);
    deckmaste_features::TYPE_STRATA
        .iter()
        .find(|(name, _)| *name == terminal)
        .map(|(_, stratum)| *stratum)
}

fn check_strata(group: &GroupDeclaration, diags: &mut Vec<Diagnostic>) {
    use deckmaste_features::FeatureStratum;
    // Every site where a declaration names a vocabulary type: scalar codecs
    // (construction fields and element fields) and free-witness payloads.
    for construction in &group.constructions {
        let id = construction.id.value.as_str();
        for binding in construction.ast.fields() {
            codec_site(id, &binding.kind, diags);
        }
        for witness in &construction.witnesses {
            let WitnessClass::Free { ty } = &witness.class else { continue };
            match stratum_of(&ty.value) {
                Some(FeatureStratum::SurfaceWitness) | None => {}
                Some(FeatureStratum::DiscourseOccurrence) => diags.push(
                    Diagnostic::new(
                        DiagCode::DiscourseFeatureExcluded,
                        id,
                        format!(
                            "`{}` is a discourse-occurrence feature; discourse features are excluded from construction declarations",
                            ty.value
                        ),
                    )
                    .with_span(ty.span),
                ),
                Some(_) => diags.push(
                    Diagnostic::new(
                        DiagCode::FreeWitnessStratum,
                        id,
                        format!(
                            "free witness `{}` carries `{}`, which is not surface-witness stratum; chart-relevant facts belong in the AST, not the trace-only channel",
                            witness.name.value, ty.value
                        ),
                    )
                    .with_span(ty.span),
                ),
            }
        }
    }
    for element in &group.elements {
        for binding in &element.fields {
            // Element fields have no construction; the group owns them.
            element_codec_site(&binding.kind, diags);
        }
    }
}

fn codec_site(id: &str, kind: &FieldKind, diags: &mut Vec<Diagnostic>) {
    let codec = match kind {
        FieldKind::Scalar { codec } | FieldKind::SurfaceScalar { codec } => codec,
        FieldKind::Optional { inner } => return codec_site(id, inner, diags),
        FieldKind::Identity { .. } | FieldKind::Subtree { .. } | FieldKind::Sequence { .. } => {
            return;
        }
    };
    if stratum_of(&codec.value) == Some(deckmaste_features::FeatureStratum::DiscourseOccurrence) {
        diags.push(
            Diagnostic::new(
                DiagCode::DiscourseFeatureExcluded,
                id,
                format!(
                    "`{}` is a discourse-occurrence feature; discourse features are excluded from construction declarations",
                    codec.value
                ),
            )
            .with_span(codec.span),
        );
    }
}

fn element_codec_site(kind: &FieldKind, diags: &mut Vec<Diagnostic>) {
    let codec = match kind {
        FieldKind::Scalar { codec } | FieldKind::SurfaceScalar { codec } => codec,
        FieldKind::Optional { inner } => return element_codec_site(inner, diags),
        FieldKind::Identity { .. } | FieldKind::Subtree { .. } | FieldKind::Sequence { .. } => {
            return;
        }
    };
    if stratum_of(&codec.value) == Some(deckmaste_features::FeatureStratum::DiscourseOccurrence) {
        diags.push(
            Diagnostic::group(
                DiagCode::DiscourseFeatureExcluded,
                format!(
                    "`{}` is a discourse-occurrence feature; discourse features are excluded from construction declarations",
                    codec.value
                ),
            )
            .with_span(codec.span),
        );
    }
}

#[cfg(test)]
pub(crate) mod fixtures {
    use crate::model::AstShape;
    use crate::model::Constraint;
    use crate::model::ConstructionDeclaration;
    use crate::model::FieldBinding;
    use crate::model::FieldKind;
    use crate::model::FieldPath;
    use crate::model::FormDeclaration;
    use crate::model::GroupDeclaration;
    use crate::model::Predicate;
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
                bind_adapter: None,
                projection: None,
                constraints: vec![],
                witnesses: vec![],
                forms: vec![FormDeclaration {
                    name: Spanned::call_site("binary".to_owned()),
                    ordinal: Spanned::call_site(0),
                    surface: vec![SurfaceAtom::Lexeme(FieldPath::call_site("conjunction"))],
                    guard: None,
                    value_guard: None,
                    fallback: false,
                }],
                dominance: vec![],
                selection: SelectionPromise::Packed,
                deserialize: false,
            }],
        }
    }

    /// Own-mode sibling of `minimal_group`: one scalar field, one require.
    pub(crate) fn minimal_own_group() -> GroupDeclaration {
        let mut group = minimal_group();
        group.constructions[0].ast = AstShape::Own {
            name: Spanned::call_site("MinimalNode".to_owned()),
            fields: vec![FieldBinding {
                field: Spanned::call_site("conjunction".to_owned()),
                kind: FieldKind::Scalar {
                    codec: Spanned::call_site("Conjunction".to_owned()),
                },
            }],
        };
        group.constructions[0]
            .constraints
            .push(Constraint::Require(Spanned::call_site(Predicate::In {
                path: FieldPath::call_site("conjunction"),
                allowed: vec!["And".to_owned(), "Or".to_owned()],
            })));
        group
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

    #[test]
    fn surface_scalar_requires_a_bound_semantic_element() {
        let group = crate::parse::parse_group(quote::quote! {
            group invalid_surface_field;
            element member {
                comma: surface lex Comma,
            }
        })
        .expect("surface-only scalar syntax parses before semantic validation");
        let err = validate(&group).expect_err("owned elements cannot erase declared fields");
        assert_eq!(codes(&err), vec!["EC008"]);
        assert_eq!(
            message_for(&err, "EC008"),
            "surface-only field `member.comma` requires a bound semantic element",
        );
    }

    #[test]
    fn surface_scalar_is_rejected_on_a_construction_field() {
        let group = crate::parse::parse_group(quote::quote! {
            group invalid_surface_field;
            construction c: Phrase {
                own Node { comma: surface lex Comma, }
                form only @ 0 = lex(comma);
            }
        })
        .expect("surface-only scalar syntax parses before semantic validation");
        let err = validate(&group).expect_err("surface derivation needs sequence position");
        assert!(codes(&err).contains(&"EC008"));
        assert_eq!(
            message_for(&err, "EC008"),
            "surface-only field `comma` is valid only inside a bound element",
        );
    }

    fn codes(err: &[crate::diag::Diagnostic]) -> Vec<&'static str> {
        err.iter().map(|d| d.code.as_str()).collect()
    }

    /// The rendered text of the one diagnostic carrying `code`. Asserting on
    /// the exact text is what pins "the author's own path, never an internal
    /// abstraction key" — a code-only assertion cannot see message quality.
    /// Collects every match rather than taking the first: a second
    /// diagnostic under the same code would silently pin the wrong one.
    fn message_for(err: &[crate::diag::Diagnostic], code: &str) -> String {
        let matches: Vec<&crate::diag::Diagnostic> =
            err.iter().filter(|d| d.code.as_str() == code).collect();
        match matches.as_slice() {
            [one] => one.message.clone(),
            [] => panic!("expected a {code} diagnostic; got {err:?}"),
            many => panic!("expected exactly one {code} diagnostic; got {many:?}"),
        }
    }

    /// `minimal_group()` plus `rest: Sequence<NounPhraseCoordination>`, the
    /// element it names, and a form atom producing it.
    fn group_with_sequence_field() -> crate::model::GroupDeclaration {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            bind_path: None,
            variants: vec![],
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
                crate::model::FieldPath::call_site("rest.last.comma"),
            ));
        group
    }

    #[test]
    fn duplicate_construction_ids_are_rejected() {
        let mut group = minimal_group();
        let twin = group.constructions[0].clone();
        group.constructions.push(twin);
        let err = validate(&group).expect_err("duplicate id");
        assert!(codes(&err).contains(&"EC001"));
    }

    #[test]
    fn duplicate_id_notes_the_first_declaration() {
        let mut group = minimal_group();
        let mut dup = group.constructions[0].clone();
        dup.forms[0].ordinal = crate::model::Spanned::call_site(1);
        group.constructions.push(dup);
        let err = validate(&group).expect_err("duplicate id must be rejected");
        let diag = err
            .iter()
            .find(|d| d.code == DiagCode::DuplicateConstructionId)
            .expect("EC001");
        assert_eq!(diag.notes.len(), 1);
        assert_eq!(diag.notes[0].message, "first declared here");
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
        assert!(codes(&err).contains(&"EC002"));
    }

    #[test]
    fn duplicate_element_name_is_group_scoped() {
        let mut group = minimal_group();
        let element = crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        };
        group.elements.push(element.clone());
        group.elements.push(element);
        let err = validate(&group).expect_err("duplicate element name must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC004"]);
        assert!(
            err[0].construction.is_none(),
            "element names belong to the group"
        );
        assert_eq!(err[0].notes[0].message, "first declared here");
    }

    #[test]
    fn bound_element_cannot_mix_struct_fields_and_enum_variants() {
        let group = crate::parse::parse_group(quote::quote! {
            group g;
            element member bind Member {
                phrase: hole Phrase,
                variant Phrase: hole Phrase,
            }
        })
        .expect("mixed syntax parses so validation can report a stable diagnostic");
        let err = validate(&group).expect_err("one bound mapping cannot be both struct and enum");
        assert_eq!(codes(&err), vec!["EC008"]);
        assert_eq!(
            message_for(&err, "EC008"),
            "bound element `member` mixes struct fields and enum variants; declare exactly one mapping shape",
        );
    }

    #[test]
    fn owned_element_cannot_declare_enum_variants() {
        let group = crate::parse::parse_group(quote::quote! {
            group g;
            element member {
                variant Phrase: hole Phrase,
            }
        })
        .expect("variant syntax parses independently of semantic ownership");
        let err = validate(&group).expect_err("only an existing bound enum can supply variants");
        assert_eq!(codes(&err), vec!["EC008"]);
        assert_eq!(
            message_for(&err, "EC008"),
            "element `member` declares enum variants but has no `bind` target",
        );
    }

    #[test]
    fn bound_element_variant_names_are_unique() {
        let group = crate::parse::parse_group(quote::quote! {
            group g;
            element member bind Member {
                variant Phrase: hole Phrase,
                variant Phrase: hole OtherPhrase,
            }
        })
        .expect("duplicate names are semantic, not syntactic");
        let err = validate(&group).expect_err("variant names identify distinct Rust enum cases");
        assert_eq!(codes(&err), vec!["EC004"]);
        assert_eq!(
            message_for(&err, "EC004"),
            "variant name `Phrase` is declared more than once in element `member`",
        );
        assert_eq!(err[0].notes[0].message, "first declared here");
    }

    #[test]
    fn bound_element_variant_payload_must_be_a_subtree_hole() {
        let group = crate::parse::parse_group(quote::quote! {
            group g;
            element member bind Member {
                variant Phrase: lex PhraseCodec,
            }
        })
        .expect("all field kinds remain syntax so validation can explain the restriction");
        let err = validate(&group).expect_err("enum cases carry one typed syntax payload");
        assert_eq!(codes(&err), vec!["EC009"]);
        assert_eq!(
            message_for(&err, "EC009"),
            "variant `member::Phrase` payload must be `hole TYPE` or `hole box TYPE`",
        );
    }

    fn group_with_empty_bound_element() -> crate::model::GroupDeclaration {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("empty_payload".to_owned()),
            bind_path: Some(crate::model::Spanned::call_site("BoundPayload".to_owned())),
            variants: vec![],
            fields: vec![],
        });
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("payloads".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("empty_payload".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("payloads"),
            ));
        group
    }

    #[test]
    fn empty_bound_element_requires_direct_zero_length_in_every_reference() {
        let group = group_with_empty_bound_element();
        let err = validate(&group).expect_err("an opaque empty mapping must never consume a value");
        assert_eq!(codes(&err), vec!["EC007"]);
        assert_eq!(
            message_for(&err, "EC007"),
            "sequence field `payloads` references empty bound element `empty_payload`; add `require payloads.len() == 0`",
        );
    }

    #[test]
    fn direct_zero_length_requirement_proves_empty_bound_element_safe() {
        let mut group = group_with_empty_bound_element();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::LenIs {
                    path: crate::model::FieldPath::call_site("payloads"),
                    len: 0,
                }),
            ));
        validate(&group).expect("the existing direct len-is-zero predicate proves emptiness");
    }

    #[test]
    fn typed_bound_enum_element_can_be_nonempty() {
        let mut group = group_with_empty_bound_element();
        group.elements[0]
            .variants
            .push(crate::model::ElementVariantDeclaration {
                name: crate::model::Spanned::call_site("Phrase".to_owned()),
                payload: crate::model::FieldKind::Subtree {
                    category: crate::model::Spanned::call_site("Phrase".to_owned()),
                    boxed: false,
                },
            });
        validate(&group).expect("a total enum mapping removes the opaque-empty restriction");
    }

    #[test]
    fn optional_sequence_cannot_bypass_empty_bound_element_rule() {
        let mut group = group_with_empty_bound_element();
        let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast else {
            panic!("fixture binds");
        };
        let sequence = fields[1].kind.clone();
        fields[1].kind = crate::model::FieldKind::Optional {
            inner: Box::new(sequence),
        };
        let err =
            validate(&group).expect_err("an optional opaque sequence has no provable length path");
        assert_eq!(codes(&err), vec!["EC007"]);
        assert_eq!(
            message_for(&err, "EC007"),
            "optional sequence field `payloads` references empty bound element `empty_payload`; only a direct sequence field with `require payloads.len() == 0` can prove it empty",
        );
    }

    #[test]
    fn element_nested_sequence_cannot_bypass_empty_bound_element_rule() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("empty_payload".to_owned()),
            bind_path: Some(crate::model::Spanned::call_site("BoundPayload".to_owned())),
            variants: vec![],
            fields: vec![],
        });
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("container".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("payloads".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("empty_payload".to_owned()),
                },
            }],
        });
        let err =
            validate(&group).expect_err("an element-nested opaque sequence has no require path");
        assert_eq!(codes(&err), vec!["EC007"]);
        assert!(
            err[0].construction.is_none(),
            "element diagnostics are group-scoped"
        );
        assert_eq!(
            message_for(&err, "EC007"),
            "element field `container.payloads` references empty bound element `empty_payload`; element-nested sequences cannot carry a direct zero-length requirement",
        );
    }

    #[test]
    fn duplicate_form_name_is_construction_scoped() {
        let mut group = minimal_group();
        let mut twin = group.constructions[0].forms[0].clone();
        // Different ordinal so EC002 doesn't also fire and pollute the
        // full-equality assertion below.
        twin.ordinal = crate::model::Spanned::call_site(1);
        group.constructions[0].forms.push(twin);
        // Two identical (unguarded) forms would also trip EC024 (ambiguous
        // linearization). A free witness makes `check_surface_domain` skip
        // that overlap check entirely, isolating EC004.
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("disambiguator".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        let err = validate(&group).expect_err("duplicate form name must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC004"]);
        assert_eq!(
            err[0].construction.as_deref(),
            Some("noun_phrase_coordination")
        );
        assert_eq!(err[0].notes[0].message, "first declared here");
    }

    #[test]
    fn duplicate_witness_name_is_construction_scoped() {
        let mut group = minimal_group();
        let witness = crate::model::WitnessDeclaration {
            name: crate::model::Spanned::call_site("oxford_comma".to_owned()),
            class: crate::model::WitnessClass::Free {
                ty: crate::model::Spanned::call_site("Comma".to_owned()),
            },
        };
        group.constructions[0].witnesses.push(witness.clone());
        group.constructions[0].witnesses.push(witness);
        let err = validate(&group).expect_err("duplicate witness name must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC004"]);
        assert_eq!(
            err[0].construction.as_deref(),
            Some("noun_phrase_coordination")
        );
        assert_eq!(err[0].notes[0].message, "first declared here");
    }

    #[test]
    fn two_own_mode_constructions_with_the_same_type_name_are_rejected() {
        // Different construction ids (so EC001 doesn't also fire), same
        // literal Rust type name — the emitter would produce two
        // `pub struct MinimalNode` in one module (E0428).
        let mut group = crate::validate::fixtures::minimal_own_group();
        let mut second = group.constructions[0].clone();
        second.id = crate::model::Spanned::call_site("second_construction".to_owned());
        group.constructions.push(second);
        let err = validate(&group).expect_err("colliding own-mode type names must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC006"]);
        assert!(
            err[0].construction.is_none(),
            "generated-name collisions are group-scoped"
        );
        assert_eq!(err[0].notes[0].message, "first declared here");
    }

    #[test]
    fn element_pascal_case_collision_with_own_mode_type_is_rejected() {
        // The element's PascalCased name equals another construction's
        // literal Rust type name — the emitter would produce a struct and
        // an unrelated element sharing one identifier.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("minimal_node".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        });
        let err = validate(&group)
            .expect_err("element/own-mode generated-name collision must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC006"]);
    }

    #[test]
    fn two_elements_with_colliding_pascal_case_names_are_rejected() {
        // `foo_bar` and `foo__bar` are different declared names but the
        // same PascalCase transform (`pascal_case` skips empty split parts).
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("foo_bar".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        });
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("foo__bar".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        });
        let err =
            validate(&group).expect_err("colliding element PascalCase names must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC006"]);
    }

    #[test]
    fn bound_enum_view_type_collisions_are_rejected_before_emission() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        let crate::model::AstShape::Own { name, .. } = &mut group.constructions[0].ast else {
            panic!("fixture owns");
        };
        *name = crate::model::Spanned::call_site("BoundVariantVariantRef".to_owned());
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("bound_variant".to_owned()),
            bind_path: Some(crate::model::Spanned::call_site("BoundVariant".to_owned())),
            fields: vec![],
            variants: vec![crate::model::ElementVariantDeclaration {
                name: crate::model::Spanned::call_site("Phrase".to_owned()),
                payload: crate::model::FieldKind::Subtree {
                    category: crate::model::Spanned::call_site("Phrase".to_owned()),
                    boxed: false,
                },
            }],
        });
        let err = validate(&group).expect_err("generated type namespace must be collision-free");
        assert_eq!(codes(&err), vec!["EC006"]);
        assert!(message_for(&err, "EC006").contains("BoundVariantVariantRef"));
    }

    #[test]
    fn bound_enum_builder_suffix_collisions_are_rejected_before_emission() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("bound_variant".to_owned()),
            bind_path: Some(crate::model::Spanned::call_site("BoundVariant".to_owned())),
            fields: vec![],
            variants: vec![
                crate::model::ElementVariantDeclaration {
                    name: crate::model::Spanned::call_site("URLValue".to_owned()),
                    payload: crate::model::FieldKind::Subtree {
                        category: crate::model::Spanned::call_site("Phrase".to_owned()),
                        boxed: false,
                    },
                },
                crate::model::ElementVariantDeclaration {
                    name: crate::model::Spanned::call_site("UrlValue".to_owned()),
                    payload: crate::model::FieldKind::Subtree {
                        category: crate::model::Spanned::call_site("Phrase".to_owned()),
                        boxed: false,
                    },
                },
            ],
        });
        let err = validate(&group).expect_err("two variants cannot mint one builder function");
        assert_eq!(codes(&err), vec!["EC006"]);
        assert!(message_for(&err, "EC006").contains("build_bound_variant_url_value"));
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
        assert!(codes(&err).contains(&"EC040"));
    }

    #[test]
    fn deserialize_on_bind_mode_is_rejected() {
        // minimal_group's sole construction is Bind-mode; `deserialize` has
        // no generated own-type to route through, so it is nonsensical
        // there, not merely unimplemented.
        let mut group = minimal_group();
        group.constructions[0].deserialize = true;
        let err = validate(&group).expect_err("bind mode has no type to deserialize into");
        assert_eq!(codes(&err), vec!["EC005"]);
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
        assert!(codes(&err).contains(&"EC041"));
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
        assert!(codes(&err).contains(&"EC010"));
    }

    #[test]
    fn unknown_middle_segment_is_named_and_spanned() {
        let mut group = minimal_group();
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("conjunction.ghost.tail"),
            ));
        let err = validate(&group).expect_err("bad path must be rejected");
        let message = message_for(&err, "EC010");
        assert!(
            message.contains("`ghost`"),
            "message names the failing segment: {message}"
        );
        assert!(
            message.contains("conjunction.ghost.tail"),
            "and the whole path: {message}"
        );
    }

    #[test]
    fn sequence_fields_resolve_through_declared_elements() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            bind_path: None,
            variants: vec![],
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
            own @ crate::model::AstShape::Own { .. } => own,
        };
        // `rest` must be produced by some form (EC021 arrives in Task 11);
        // reference it so this test isolates path resolution. Entry into the
        // element is explicit via `last` — implicit mid-path entry is gone.
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("rest.last.comma"),
            ));
        validate(&group).expect("rest.last.comma resolves through the element");
    }

    #[test]
    fn leading_last_with_no_sequence_context_is_rejected() {
        let mut group = minimal_group();
        // The group DOES declare an element, and — critically — that element
        // has a `conjunction` field, matching the path's second segment.
        // Review round 1 found this test passed for the wrong reason against
        // a broken sequence-in-hand guard that fell back to
        // `group.elements.first()`: with `minimal_group()`'s empty
        // `elements`, that broken fallback had nothing to fall back to and
        // failed anyway, by accident rather than because the guard works. An
        // element with no matching field would only weaken the accident (it
        // would still fail, just one segment deeper) — giving it the exact
        // field the path asks for next means the broken fallback resolves
        // the WHOLE path successfully, so only an intact guard rejects it.
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("conjunction".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Conjunction".to_owned()),
                },
            }],
        });
        // `last` re-addresses a sequence element; as the first segment there
        // is no sequence in scope yet for it to re-address.
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("last.conjunction"),
            ));
        let err = validate(&group).expect_err("last with no preceding sequence");
        assert!(codes(&err).contains(&"EC010"));
    }

    #[test]
    fn last_after_a_non_sequence_field_is_rejected() {
        // The case `leading_last_with_no_sequence_context_is_rejected` can't
        // reach: `last` here does NOT come first — it follows `conjunction`,
        // which resolves to a Scalar, not a Sequence. Under the broken guard
        // review round 1 described, this wrongly resolved to
        // `Ok(Resolved::Element(..))` via `group.elements.first()`, because
        // `resolved` being `Some(Kind(Scalar))` (not `None`) doesn't stop a
        // fallback keyed only on "isn't a Sequence".
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        });
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("conjunction.last"),
            ));
        let err =
            validate(&group).expect_err("`last` after a non-sequence field addresses nothing");
        assert!(codes(&err).contains(&"EC010"));
    }

    #[test]
    fn last_after_a_sequence_field_still_resolves() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("NounPhraseCoordination".to_owned()),
            bind_path: None,
            variants: vec![],
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
            own @ crate::model::AstShape::Own { .. } => own,
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
            "form `binary` references `rest.last.last`: `last` does not resolve"
        );
    }

    /// `resolve_path` unit-level, exercising `Resolved` directly (private
    /// access from this module, same as every other `resolve_path` test):
    /// `last` yields the element itself, and a segment after it re-enters
    /// the element's own fields.
    #[test]
    fn last_resolves_to_the_element_declaration() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            }],
        });
        group.constructions[0].ast = crate::model::AstShape::Own {
            name: crate::model::Spanned::call_site("Node".to_owned()),
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            }],
        };
        let c = &group.constructions[0];
        match resolve_path(
            &group,
            c,
            &crate::model::FieldPath::call_site("members.last"),
        ) {
            Ok(Resolved::Element(e)) => assert_eq!(e.name.value, "m"),
            other => panic!("expected Element, got {other:?}"),
        }
        match resolve_path(
            &group,
            c,
            &crate::model::FieldPath::call_site("members.last.comma"),
        ) {
            Ok(Resolved::Kind(crate::model::FieldKind::Optional { .. })) => {}
            other => panic!("expected Optional kind, got {other:?}"),
        }
        let err = resolve_path(
            &group,
            c,
            &crate::model::FieldPath::call_site("members.last.ghost"),
        )
        .unwrap_err();
        assert_eq!(err.index, 2);
    }

    #[test]
    fn nonfinal_resolves_to_an_element_field_for_require_predicates() {
        let mut group = group_with_sequence_field();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest.nonfinal.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        validate(&group).expect("a nonfinal quantified element field is valid in require");
    }

    #[test]
    fn nonfinal_is_rejected_outside_require_predicates() {
        let mut group = group_with_sequence_field();
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("rest.nonfinal.comma"),
            ));
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("interior".to_owned()),
                class: crate::model::WitnessClass::Stored {
                    path: crate::model::FieldPath::call_site("rest.nonfinal.comma"),
                },
            });
        let err = validate(&group).expect_err("a quantified path is not one renderable value");
        let messages = err
            .iter()
            .filter(|diagnostic| diagnostic.code.as_str() == "EC010")
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>();
        assert!(
            messages.iter().any(|message| message.contains(
                "form `binary` references `rest.nonfinal.comma`: `nonfinal` is predicate-only"
            )),
            "form diagnostic names the predicate-only segment: {messages:?}",
        );
        assert!(
            messages.iter().any(|message| message.contains(
                "stored witness `interior` names `rest.nonfinal.comma`: `nonfinal` is predicate-only"
            )),
            "witness diagnostic names the predicate-only segment: {messages:?}",
        );
    }

    #[test]
    fn nonfinal_without_a_sequence_context_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("conjunction.nonfinal.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        let err = validate(&group).expect_err("nonfinal must follow a sequence");
        assert_eq!(
            message_for(&err, "EC010"),
            "require clause references `conjunction.nonfinal.comma`: `nonfinal` does not resolve"
        );
    }

    /// The regression this function actually suffered twice: Milestone 1
    /// let a named segment walk into a sequence's element implicitly. The
    /// new contract requires an explicit `last` — `members.comma` must fail
    /// on `comma` (index 1), not resolve through `members` implicitly.
    #[test]
    fn members_field_without_last_is_rejected() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            }],
        });
        group.constructions[0].ast = crate::model::AstShape::Own {
            name: crate::model::Spanned::call_site("Node".to_owned()),
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            }],
        };
        let c = &group.constructions[0];
        let err = resolve_path(
            &group,
            c,
            &crate::model::FieldPath::call_site("members.comma"),
        )
        .unwrap_err();
        assert_eq!(
            err.index, 1,
            "`comma` (not `members`) is the segment that fails to resolve"
        );
    }

    /// `members_field_without_last_is_rejected` above never exercises the
    /// "named segment must follow root or a `last`-entered element" guard:
    /// `comma` isn't findable in the unadvanced top-level `fields` pointer
    /// whether or not the guard exists, so that test passes even with the
    /// guard deleted. This test puts a name at the top level that IS
    /// findable there — `conjunction`, from `minimal_group` — alongside the
    /// sequence field, so the guard's presence is the only thing standing
    /// between the correct `Err` and a wrongly-successful
    /// `Ok(Resolved::Kind(Scalar))`.
    #[test]
    fn named_segment_after_unlasted_sequence_is_rejected_even_when_the_name_collides() {
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![],
        });
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        let c = &group.constructions[0];
        let err = resolve_path(
            &group,
            c,
            &crate::model::FieldPath::call_site("members.conjunction"),
        )
        .unwrap_err();
        assert_eq!(
            err.index, 1,
            "the guard, not an unresolved name, must be what rejects this"
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
            own @ crate::model::AstShape::Own { .. } => own,
        };
        let err = validate(&group).expect_err("phantom element");
        assert!(codes(&err).contains(&"EC003"));
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
        assert!(codes(&err).contains(&"EC012"));
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
        assert!(codes(&err).contains(&"EC013"));
    }

    #[test]
    fn bogus_require_path_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsSome {
                    path: crate::model::FieldPath::call_site("nonexistent"),
                }),
            ));
        let err = validate(&group).expect_err("bogus require path must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC010"]);
    }

    #[test]
    fn bogus_form_guard_path_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::IsSome {
                path: crate::model::FieldPath::call_site("nonexistent"),
            },
        ));
        let err = validate(&group).expect_err("bogus form guard path must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC010"]);
    }

    #[test]
    fn is_some_on_a_scalar_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsSome {
                    path: crate::model::FieldPath::call_site("conjunction"),
                }),
            ));
        let err = validate(&group).expect_err("presence predicate on scalar must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC011"]);
    }

    #[test]
    fn scalar_consumed_as_hole_is_rejected() {
        let mut group = minimal_group();
        // minimal_group's form renders `conjunction` via Lexeme; make it a Hole.
        group.constructions[0].forms[0].surface = vec![crate::model::SurfaceAtom::Hole(
            crate::model::FieldPath::call_site("conjunction"),
        )];
        let err = validate(&group).expect_err("scalar hole must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC014"]);
    }

    #[test]
    fn identity_field_requires_an_identity_surface_atom() {
        let mut group = minimal_group();
        let fields = group.constructions[0].ast.fields();
        let field = fields[0].field.clone();
        let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast else {
            unreachable!()
        };
        fields[0].kind = crate::model::FieldKind::Identity {
            value_type: crate::model::Spanned::call_site("FixtureLexeme".to_owned()),
            provider: crate::model::Spanned::call_site("FixtureLexicon".to_owned()),
        };
        let err = validate(&group).expect_err("lex(…) must not erase a typed identity");
        assert_eq!(codes(&err), vec!["EC014"]);

        group.constructions[0].forms[0].surface = vec![crate::model::SurfaceAtom::Identity(
            crate::model::FieldPath {
                segments: vec![field],
                span: proc_macro2::Span::call_site(),
            },
        )];
        validate(&group).expect("identity(…) matches the declared identity kind");
    }

    #[test]
    fn deep_require_path_is_rejected() {
        // `members.last.sub` resolves (unlike a doubled `last` or a
        // subtree-then-more path), and `sub` being Optional clears EC011 (it
        // IS optional) — but its inner kind is Subtree, not Scalar, so
        // EC032's sequence-selector admission gate is the only thing left to reject
        // it. A still-deep, still-refused shape now that
        // `members.last.comma` (Optional<Scalar>) is admitted.
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("sub".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Subtree {
                        category: crate::model::Spanned::call_site("Whatever".to_owned()),
                        boxed: false,
                    }),
                },
            }],
        });
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("members"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsNone {
                    path: crate::model::FieldPath::call_site("members.last.sub"),
                }),
            ));
        let err = validate(&group).expect_err("a non-scalar `.last` target must still be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC032"]);
    }

    /// Positive control for the widening: `.last` onto a plain (non-
    /// optional) element scalar admits cleanly, and the abstraction pass
    /// (`path_is_optional_scalar`) correctly treats it as NOT optional —
    /// `group_with_sequence_field`'s `comma` field is `Scalar`, not
    /// `Optional<Scalar>`, unlike the `fixture_pair` case exercised
    /// end-to-end in `fixture_family.rs`.
    #[test]
    fn last_path_onto_a_plain_scalar_element_field_validates_clean() {
        let mut group = group_with_sequence_field();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest.last.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        validate(&group).expect("`.last` onto a plain scalar element field is admitted");
    }

    #[test]
    fn is_none_on_optional_field_validates_clean() {
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("opt".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("opt"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsNone {
                    path: crate::model::FieldPath::call_site("opt"),
                }),
            ));
        validate(&group).expect("`is_none` on a genuinely Optional single-segment field is clean");
    }

    #[test]
    fn in_predicate_on_a_sequence_is_rejected() {
        let mut group = group_with_sequence_field();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest"),
                    allowed: vec!["x".to_owned()],
                }),
            ));
        let err = validate(&group).expect_err("`in` on a Sequence field must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC015"]);
    }

    #[test]
    fn in_predicate_on_an_optional_scalar_is_admitted() {
        // `own N { alt: opt lex Conjunction } require alt in [And, Or];` —
        // adopted ruling (reading B): admitted, compiling to
        // `matches!(alt, None | Some(Conjunction::And | Conjunction::Or))`.
        // Absence is vacuously true; an author wanting strictness composes
        // `all(alt.is_some(), alt in [...])` instead. See
        // `in_predicate_kind_problem` for the full rationale.
        let mut group = crate::validate::fixtures::minimal_own_group();
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("alt".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Conjunction".to_owned()),
                    }),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("alt"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("alt"),
                    allowed: vec!["And".to_owned(), "Or".to_owned()],
                }),
            ));
        validate(&group).expect("`in` on an Optional<Scalar> field is admitted under reading B");
    }

    #[test]
    fn in_predicate_on_a_generic_codec_is_rejected() {
        // A generic codec renders `Wrapper<Inner>::A` in pattern position,
        // which is not legal Rust (needs turbofish). EC015 rejects rather
        // than special-casing the render.
        let mut group = crate::validate::fixtures::minimal_own_group();
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields[0].kind = crate::model::FieldKind::Scalar {
                codec: crate::model::Spanned::call_site("Wrapper<Conjunction>".to_owned()),
            };
        }
        let err = validate(&group).expect_err("`in` on a generic codec must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC015"]);
    }

    /// Positive control for EC015's `In` target rule: a plain, non-optional,
    /// non-generic scalar codec still validates clean. `emit.rs`'s
    /// `in_predicate_emits_a_closed_match` pins that this same shape still
    /// emits the closed `matches!`.
    #[test]
    fn in_predicate_on_a_plain_scalar_validates_clean() {
        let group = crate::validate::fixtures::minimal_own_group();
        validate(&group).expect("`in` on a plain non-optional scalar codec is clean");
    }

    #[test]
    fn len_predicate_on_a_scalar_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::LenAtLeast {
                    path: crate::model::FieldPath::call_site("conjunction"),
                    min: 2,
                }),
            ));
        let err = validate(&group).expect_err("`len()` on a Scalar field must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC015"]);
    }

    /// Positive control: the sanctioned predicate/kind pairings — `in` on a
    /// `Scalar` (as a require clause and as a form guard), `len()` on a
    /// `Sequence`, and presence on an `Optional` — all validate clean. Two
    /// forms partition `conjunction`'s admitted `{And, Or}` so EC023
    /// coverage doesn't fire alongside the shapes under test.
    #[test]
    fn sanctioned_predicate_kind_pairings_validate_clean() {
        let mut group = group_with_sequence_field();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("alt".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("alt"),
            ));
        group.constructions[0].forms[0].guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["And".to_owned()],
            },
        ));
        let mut or_form = group.constructions[0].forms[0].clone();
        or_form.name = crate::model::Spanned::call_site("or_form".to_owned());
        or_form.ordinal = crate::model::Spanned::call_site(1);
        or_form.surface = vec![crate::model::SurfaceAtom::Lexeme(
            crate::model::FieldPath::call_site("conjunction"),
        )];
        or_form.guard = Some(crate::model::Spanned::call_site(
            crate::model::Predicate::In {
                path: crate::model::FieldPath::call_site("conjunction"),
                allowed: vec!["Or".to_owned()],
            },
        ));
        group.constructions[0].forms.push(or_form);
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("conjunction"),
                    allowed: vec!["And".to_owned(), "Or".to_owned()],
                }),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::LenAtLeast {
                    path: crate::model::FieldPath::call_site("rest"),
                    min: 2,
                }),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsNone {
                    path: crate::model::FieldPath::call_site("alt"),
                }),
            ));
        validate(&group).expect(
            "in-on-scalar (require and guard), len()-on-sequence, and presence-on-optional are all sanctioned",
        );
    }

    #[test]
    fn empty_surface_is_rejected() {
        let mut group = minimal_group();
        group.constructions[0].forms[0].surface.clear();
        let err = validate(&group).expect_err("empty production");
        assert!(codes(&err).contains(&"EC020"));
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
        assert!(codes(&err).contains(&"EC021"));
    }

    #[test]
    fn double_consumption_within_one_form_is_rejected() {
        let mut group = minimal_group();
        let duplicate =
            crate::model::SurfaceAtom::Lexeme(crate::model::FieldPath::call_site("conjunction"));
        group.constructions[0].forms[0].surface.push(duplicate);
        let err = validate(&group).expect_err("double consumption");
        assert!(codes(&err).contains(&"EC022"));
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
        let diag = err
            .iter()
            .find(|d| d.code == DiagCode::AmbiguousLinearization)
            .expect("EC024");
        assert_eq!(diag.notes.len(), 1);
        assert_eq!(diag.notes[0].message, "the other form is here");
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
    fn opaque_value_guard_requires_a_canonical_fallback() {
        // Mutation caught: treat an opaque whole-value guard as if it proved
        // total coverage, leaving unmatched values with no canonical form.
        let mut group = minimal_group();
        group.constructions[0].forms[0].value_guard = Some(crate::model::Spanned::call_site(
            "takes_guarded_form".to_owned(),
        ));

        let err = validate(&group).expect_err("opaque guards cannot prove total coverage");
        assert_eq!(codes(&err), vec!["EC023"]);
    }

    #[test]
    fn canonical_fallback_must_be_unique() {
        // Mutation caught: let emitter declaration order choose between two
        // canonical fallbacks for the same semantic value.
        let mut group = minimal_group();
        group.constructions[0].forms[0].fallback = true;
        let mut second = group.constructions[0].forms[0].clone();
        second.name = crate::model::Spanned::call_site("second".to_owned());
        second.ordinal = crate::model::Spanned::call_site(1);
        group.constructions[0].forms.push(second);

        let err = validate(&group).expect_err("two canonical fallbacks are ambiguous");
        assert_eq!(codes(&err), vec!["EC024"]);
    }

    #[test]
    fn canonical_fallback_must_be_unguarded() {
        // Mutation caught: accept `when ... otherwise`, whose explicit guard
        // and unconditional fallback meanings disagree during selection.
        let mut group = minimal_group();
        group.constructions[0].forms[0].fallback = true;
        group.constructions[0].forms[0].guard =
            Some(crate::model::Spanned::call_site(conjunction_in(&["And"])));

        let err = validate(&group).expect_err("a canonical fallback is unconditional");
        assert_eq!(codes(&err), vec!["EC024"]);
    }

    #[test]
    fn fallback_diagnostic_is_independent_of_declaration_order() {
        // Mutation caught: report or select whichever duplicate fallback was
        // first, making source order observable despite explicit ordinals.
        let mut forward = minimal_group();
        forward.constructions[0].forms[0].fallback = true;
        let mut alpha = forward.constructions[0].forms[0].clone();
        forward.constructions[0].forms[0].name =
            crate::model::Spanned::call_site("zeta".to_owned());
        alpha.name = crate::model::Spanned::call_site("alpha".to_owned());
        alpha.ordinal = crate::model::Spanned::call_site(1);
        forward.constructions[0].forms.push(alpha);
        let mut backward = forward.clone();
        backward.constructions[0].forms.reverse();

        let render = |group: &crate::model::GroupDeclaration| {
            validate(group)
                .expect_err("duplicate fallbacks must be rejected")
                .into_iter()
                .map(|diagnostic| format!("{}:{}", diagnostic.code.as_str(), diagnostic.message))
                .collect::<Vec<_>>()
        };
        assert_eq!(render(&forward), render(&backward));
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
        assert!(codes(&err).contains(&"EC023"));
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
        assert!(codes(&err).contains(&"EC030"));
    }

    #[test]
    fn optional_in_contributes_no_value_key_so_it_cannot_fabricate_ec030() {
        // Mirror of `contradictory_requirements_are_rejected`, but the
        // target is `Optional<Scalar>`. `alt in [And]` and `alt in [Or]`
        // would intersect to the empty set — and wrongly report EC030 — if
        // an optional `In` contributed a finite Value key the way a
        // non-optional `In` does. It must not: under reading B each clause
        // is independently satisfied by `alt` being absent, so the
        // conjunction is not a contradiction. Positive control for the "no
        // key" comment on `collect_abstraction`'s `In` arm, the same way
        // `len_at_least_contributes_no_finite_length_set` is the positive
        // control for `LenAtLeast`'s identical "no key" behavior.
        let mut group = crate::validate::fixtures::minimal_own_group();
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("alt".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Conjunction".to_owned()),
                    }),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("alt"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("alt"),
                    allowed: vec!["And".to_owned()],
                }),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("alt"),
                    allowed: vec!["Or".to_owned()],
                }),
            ));
        validate(&group).expect(
            "optional `In` contributes no Value key, so `alt in [And]` and `alt in [Or]` do not intersect to empty",
        );
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
        assert!(codes(&err).contains(&"EC030"));
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
        assert!(codes(&err).contains(&"EC030"));
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
        // `conjunction` is a bare Scalar, so `is_some` on it would now trip
        // EC011 (Task 5); this test's target is EC030 (contradiction
        // detection), so it needs a genuinely Optional field to isolate that.
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("opt".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Lexeme(
                crate::model::FieldPath::call_site("opt"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::IsSome {
                    path: crate::model::FieldPath::call_site("opt"),
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
        assert!(codes(&err).contains(&"EC031"));
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
        let reported = codes(&err);
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
                .map(|d| {
                    format!(
                        "{}:{}",
                        d.construction.as_deref().unwrap_or(""),
                        d.code.as_str()
                    )
                })
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
                .map(|d| {
                    format!(
                        "{}:{}:{}",
                        d.construction.as_deref().unwrap_or(""),
                        d.code.as_str(),
                        d.message
                    )
                })
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
            own @ crate::model::AstShape::Own { .. } => own,
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

    #[test]
    fn free_witness_must_be_surface_witness_stratum() {
        let mut group = minimal_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("pick".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Conjunction".to_owned()),
                },
            });
        let err = validate(&group).expect_err("selection-stratum free witness must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC050"]);
    }

    #[test]
    fn discourse_types_are_excluded_from_declarations() {
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields[0].kind = crate::model::FieldKind::Scalar {
                codec: crate::model::Spanned::call_site("OccurrenceRole".to_owned()),
            };
        }
        let err = validate(&group).expect_err("discourse codec must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC051"]);
    }

    #[test]
    fn stored_surface_witness_codecs_and_free_surface_types_pass() {
        // The decision's positive space, pinned: stored commas are legal AST
        // fields, and a SurfaceWitness-stratum free witness is the sanctioned
        // shape. expect() is equality-strength (no diagnostics at all).
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields[0].kind = crate::model::FieldKind::Scalar {
                codec: crate::model::Spanned::call_site("Comma".to_owned()),
            };
        }
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("style".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        validate(&group).expect("stored comma + free Comma witness is the sanctioned shape");
    }

    #[test]
    fn discourse_types_are_excluded_from_element_fields_group_scoped() {
        // The element-field EC051 site is group-scoped: elements belong to
        // the group, not a construction, so its diagnostic must carry
        // `construction: None` rather than being attributed to whichever
        // construction happens to reference the element.
        let mut group = minimal_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("Mention".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("role".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("OccurrenceRole".to_owned()),
                },
            }],
        });
        let err =
            validate(&group).expect_err("discourse codec on an element field must be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC051"]);
        assert!(
            err[0].construction.is_none(),
            "element-field diagnostics are group-scoped, not construction-scoped: {:?}",
            err[0].construction
        );
    }

    #[test]
    fn qualified_type_paths_classify_by_terminal_ident() {
        // `stratum_of` splits on `::` and classifies by the terminal ident —
        // exactly how the DSL would write a qualified path. A whole-string
        // match would find nothing in `TYPE_STRATA`, treat the type as an
        // absent (permitted) custom payload, and this would wrongly pass.
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields[0].kind = crate::model::FieldKind::Scalar {
                codec: crate::model::Spanned::call_site("some::path::OccurrenceRole".to_owned()),
            };
        }
        let err = validate(&group).expect_err("qualified discourse codec must still be rejected");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC051"]);
    }

    #[test]
    fn generic_wrapped_discourse_type_is_not_caught_by_the_stratum_check() {
        // Companion to the previous test, at the opposite pole: a generic
        // path like `Wrapper<OccurrenceRole>` (the shape the parser now
        // renders faithfully instead of dropping the generic argument) has
        // no terminal ident that equals a bare `TYPE_STRATA` name — `stratum_of`
        // finds nothing and treats it as an unlisted custom payload. That is
        // the correct call: a generic wrapper is not itself the
        // discourse-occurrence type, so this must validate clean, not EC051.
        let mut group = minimal_group();
        if let crate::model::AstShape::Bind { fields, .. } = &mut group.constructions[0].ast {
            fields[0].kind = crate::model::FieldKind::Scalar {
                codec: crate::model::Spanned::call_site("Wrapper<OccurrenceRole>".to_owned()),
            };
        }
        validate(&group)
            .expect("a generic-wrapped payload is absent from TYPE_STRATA, hence permitted");
    }
}
