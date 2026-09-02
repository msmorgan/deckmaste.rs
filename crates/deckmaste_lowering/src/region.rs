use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_core::{DefId, Kind, Param, Provenance, RefId};
use deckmaste_semantics::{Ident, Sort};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RegionKind {
    Static,
    StaticEvent,
    Spell,
    Activated,
    Triggered,
    Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cardinality {
    One,
    Many,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Site {
    Product,
    Frame,
    Loop,
    Candidate,
    Allotment,
}

#[derive(Debug, Clone)]
struct Antecedent {
    reference: RefId,
    kind: Kind,
    cardinality: Cardinality,
    sort: Option<Sort>,
    site: Site,
}

#[derive(Debug, Clone)]
struct Context {
    params: Vec<Param>,
    definitions: Vec<Kind>,
    visible: Vec<bool>,
    source: Option<RefId>,
    controller: Option<RefId>,
    event_object: Option<RefId>,
    event_patient: Option<RefId>,
    event_actor: Option<RefId>,
    defending_player: Option<RefId>,
    targets: Vec<RefId>,
    x: Option<RefId>,
    antecedents: Vec<Antecedent>,
    named: HashMap<Ident, RefId>,
}

thread_local! {
    static CONTEXTS: RefCell<Vec<Context>> = const { RefCell::new(Vec::new()) };
}

fn with_pushed_context<T>(context: Context, f: impl FnOnce() -> T) -> (Context, T) {
    CONTEXTS.with(|contexts| contexts.borrow_mut().push(context));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    let context = CONTEXTS
        .with(|contexts| contexts.borrow_mut().pop())
        .expect("region context");
    match result {
        Ok(value) => (context, value),
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

fn push_param(params: &mut Vec<Param>, kind: Kind, provenance: Provenance) -> RefId {
    let ordinal = u32::try_from(params.len()).expect("a core region has at most u32::MAX params");
    let def = DefId(ordinal);
    params.push(Param {
        def,
        kind,
        provenance,
    });
    def.into()
}

fn context(kind: RegionKind, target_count: usize) -> Context {
    let mut params = Vec::new();
    let source = Some(push_param(&mut params, Kind::Object, Provenance::Source));
    let controller = Some(push_param(
        &mut params,
        Kind::Object,
        Provenance::Controller,
    ));

    let (event_object, event_patient, event_actor, defending_player, event_amount) = match kind {
        // Event-observing static families (replacement, prevention,
        // trigger multipliers, cant-happen) share the trigger provenance
        // prefix. Non-event statics receive unavailable values in these
        // slots; their stable region ABI remains uniform.
        RegionKind::Static | RegionKind::StaticEvent | RegionKind::Triggered => (
            Some(push_param(
                &mut params,
                Kind::Object,
                Provenance::EventObject,
            )),
            Some(push_param(
                &mut params,
                Kind::Object,
                Provenance::EventPatient,
            )),
            Some(push_param(
                &mut params,
                Kind::Object,
                Provenance::EventActor,
            )),
            Some(push_param(
                &mut params,
                Kind::Object,
                Provenance::DefendingPlayer,
            )),
            Some(push_param(
                &mut params,
                Kind::Number,
                Provenance::EventAmount,
            )),
        ),
        RegionKind::Spell | RegionKind::Activated | RegionKind::Mode => {
            (None, None, None, None, None)
        }
    };
    let targets = (0..target_count)
        .map(|index| {
            push_param(
                &mut params,
                Kind::Objects,
                Provenance::AnnouncedTarget(
                    u32::try_from(index).expect("an ability has at most u32::MAX targets"),
                ),
            )
        })
        .collect();
    let x = (!matches!(kind, RegionKind::Static | RegionKind::StaticEvent))
        .then(|| push_param(&mut params, Kind::Number, Provenance::AnnouncedX));
    let definitions: Vec<_> = params.iter().map(|param| param.kind).collect();
    let visible = vec![true; definitions.len()];
    let mut antecedents = Vec::new();
    if matches!(kind, RegionKind::StaticEvent | RegionKind::Triggered)
        && let Some(reference) = event_object
    {
        antecedents.push(Antecedent {
            reference,
            kind: Kind::Object,
            cardinality: Cardinality::One,
            sort: None,
            site: Site::Frame,
        });
    }
    if matches!(kind, RegionKind::StaticEvent | RegionKind::Triggered)
        && let Some(reference) = event_amount
    {
        antecedents.push(Antecedent {
            reference,
            kind: Kind::Number,
            cardinality: Cardinality::One,
            sort: Some(Sort::Amount),
            site: Site::Product,
        });
    }
    Context {
        params,
        definitions,
        visible,
        source,
        controller,
        event_object,
        event_patient,
        event_actor,
        defending_player,
        targets,
        x,
        antecedents,
        named: HashMap::new(),
    }
}

pub(crate) fn in_region<T>(
    kind: RegionKind,
    target_count: usize,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let (context, value) = with_pushed_context(context(kind, target_count), f);
    (context.params.into(), value)
}

pub(crate) fn is_active() -> bool {
    CONTEXTS.with(|contexts| !contexts.borrow().is_empty())
}

/// Enter an ability region carried by a value built inside another region.
/// Its intrinsic ABI remains its own (source/controller/event roles/targets),
/// followed by explicit captures of the enclosing register file.
pub(crate) fn in_carried_region<T>(
    kind: RegionKind,
    target_count: usize,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let parent = CONTEXTS
        .with(|contexts| contexts.borrow().last().cloned())
        .expect("carried core region outside a parent region");
    let mut child = context(kind, target_count);
    let captures: Vec<Option<RefId>> = parent
        .definitions
        .iter()
        .enumerate()
        .map(|(index, kind)| {
            if !parent.visible[index] {
                return None;
            }
            let outer = RefId(u32::try_from(index).expect("definition ordinal"));
            let captured = push_param(&mut child.params, *kind, Provenance::Capture(outer));
            child.definitions.push(*kind);
            child.visible.push(true);
            Some(captured)
        })
        .collect();
    let mut captured_antecedents = parent
        .antecedents
        .into_iter()
        .filter_map(|mut antecedent| {
            antecedent.reference = remap(antecedent.reference, &captures)?;
            Some(antecedent)
        })
        .collect::<Vec<_>>();
    captured_antecedents.extend(child.antecedents);
    child.antecedents = captured_antecedents;
    child.named = parent
        .named
        .into_iter()
        .filter_map(|(name, reference)| remap(reference, &captures).map(|r| (name, r)))
        .collect();
    let (child, value) = with_pushed_context(child, f);
    (child.params.into(), value)
}

fn remap(reference: RefId, captures: &[Option<RefId>]) -> Option<RefId> {
    captures.get(reference.0 as usize).copied().flatten()
}

/// Enter a nested region. Intrinsic parameters come first; outer registers
/// are captured in definition order. The stable ABI is more valuable here
/// than minimizing a serialized capture list.
pub(crate) fn in_child<T>(
    intrinsic: impl IntoIterator<Item = (Kind, Provenance)>,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let parent = CONTEXTS
        .with(|contexts| contexts.borrow().last().cloned())
        .expect("nested core region outside an ability region");
    let mut params = Vec::new();
    for (kind, provenance) in intrinsic {
        push_param(&mut params, kind, provenance);
    }
    let captures: Vec<Option<RefId>> =
        parent
            .definitions
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                if !parent.visible[index] {
                    return None;
                }
                let outer = RefId(u32::try_from(index).expect("definition ordinal"));
                let provenance = parent.params.get(index).map_or(
                    Provenance::Capture(outer),
                    |param| match &param.provenance {
                        Provenance::Source
                        | Provenance::Controller
                        | Provenance::EventObject
                        | Provenance::EventPatient
                        | Provenance::EventActor
                        | Provenance::EventAmount
                        | Provenance::DefendingPlayer
                        | Provenance::AnnouncedTarget(_)
                        | Provenance::AnnouncedX
                        | Provenance::Linked(_) => param.provenance.clone(),
                        Provenance::Capture(_)
                        | Provenance::LoopElement
                        | Provenance::Allotment
                        | Provenance::Candidate => Provenance::Capture(outer),
                    },
                );
                Some(push_param(&mut params, *kind, provenance))
            })
            .collect();
    let map = |value: Option<RefId>| value.and_then(|reference| remap(reference, &captures));
    let antecedents = parent
        .antecedents
        .into_iter()
        .filter_map(|mut antecedent| {
            antecedent.reference = remap(antecedent.reference, &captures)?;
            Some(antecedent)
        })
        .collect();
    let named = parent
        .named
        .into_iter()
        .filter_map(|(name, reference)| remap(reference, &captures).map(|r| (name, r)))
        .collect();
    let definitions: Vec<_> = params.iter().map(|param| param.kind).collect();
    let visible = vec![true; definitions.len()];
    let child = Context {
        params,
        definitions,
        visible,
        source: map(parent.source),
        controller: map(parent.controller),
        event_object: map(parent.event_object),
        event_patient: map(parent.event_patient),
        event_actor: map(parent.event_actor),
        defending_player: map(parent.defending_player),
        targets: parent
            .targets
            .into_iter()
            .filter_map(|r| remap(r, &captures))
            .collect(),
        x: map(parent.x),
        antecedents,
        named,
    };
    let (child, value) = with_pushed_context(child, f);
    (child.params.into(), value)
}

fn read(f: impl FnOnce(&Context) -> Option<RefId>) -> Option<RefId> {
    CONTEXTS.with(|contexts| contexts.borrow().last().and_then(f))
}

pub(crate) fn source() -> Option<RefId> {
    read(|context| context.source)
}
pub(crate) fn controller() -> Option<RefId> {
    read(|context| context.controller)
}
pub(crate) fn event_object() -> Option<RefId> {
    read(|context| context.event_object)
}
pub(crate) fn event_patient() -> Option<RefId> {
    read(|context| context.event_patient)
}
pub(crate) fn event_actor() -> Option<RefId> {
    read(|context| context.event_actor)
}
pub(crate) fn defending_player() -> Option<RefId> {
    read(|context| context.defending_player)
}
pub(crate) fn x() -> Option<RefId> {
    read(|context| context.x)
}
pub(crate) fn set_x(reference: RefId) {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow_mut()
            .last_mut()
            .expect("X binding outside core region")
            .x = Some(reference);
    });
}
pub(crate) fn target(index: usize) -> Option<RefId> {
    read(|context| context.targets.get(index).copied())
}

/// Lower one target constraint with only earlier target slots in scope.
pub(crate) fn with_target_prefix<T>(count: usize, f: impl FnOnce() -> T) -> T {
    let (definitions, visible, targets, x) = CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let context = contexts
            .last_mut()
            .expect("target constraint outside core region");
        let definitions = context.definitions.clone();
        let visible = context.visible.clone();
        let targets = context.targets.clone();
        let x = context.x;
        let prefix = context
            .targets
            .get(count)
            .map_or(context.definitions.len(), |reference| reference.0 as usize);
        context.definitions.truncate(prefix);
        context.visible.truncate(prefix);
        context.targets.truncate(count);
        context.x = None;
        (definitions, visible, targets, x)
    });
    let value = f();
    CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let context = contexts
            .last_mut()
            .expect("target constraint outside core region");
        context.definitions = definitions;
        context.visible = visible;
        context.targets = targets;
        context.x = x;
    });
    value
}

pub(crate) fn define(kind: Kind) -> DefId {
    CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let context = contexts.last_mut().expect("definition outside core region");
        let def = DefId(u32::try_from(context.definitions.len()).expect("definition ordinal"));
        context.definitions.push(kind);
        context.visible.push(true);
        def
    })
}

pub(crate) fn bind_named(name: Ident, reference: RefId) {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow_mut()
            .last_mut()
            .expect("binding outside core region")
            .named
            .insert(name, reference);
    });
}

pub(crate) fn newest_antecedent() -> Option<RefId> {
    read(|context| {
        context
            .antecedents
            .first()
            .map(|antecedent| antecedent.reference)
    })
}

pub(crate) fn named(name: &Ident) -> Option<RefId> {
    read(|context| context.named.get(name).copied())
}

pub(crate) fn push_antecedent(
    reference: RefId,
    kind: Kind,
    cardinality: Cardinality,
    sort: Option<Sort>,
    site: Site,
) {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow_mut()
            .last_mut()
            .expect("antecedent outside core region")
            .antecedents
            .insert(
                0,
                Antecedent {
                    reference,
                    kind,
                    cardinality,
                    sort,
                    site,
                },
            );
    });
}

pub(crate) fn remove_antecedent(reference: RefId, site: Site) {
    CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let antecedents = &mut contexts
            .last_mut()
            .expect("antecedent outside core region")
            .antecedents;
        if let Some(index) = antecedents
            .iter()
            .position(|a| a.reference == reference && a.site == site)
        {
            antecedents.remove(index);
        }
    });
}

pub(crate) fn with_antecedent<T>(
    reference: RefId,
    kind: Kind,
    cardinality: Cardinality,
    sort: Option<Sort>,
    site: Site,
    f: impl FnOnce() -> T,
) -> T {
    push_antecedent(reference, kind, cardinality, sort, site);
    let value = f();
    remove_antecedent(reference, site);
    value
}

pub(crate) fn scoped_antecedents<T>(f: impl FnOnce() -> T) -> T {
    let saved = CONTEXTS.with(|contexts| {
        let contexts = contexts.borrow();
        let context = contexts.last().expect("scope outside core region");
        (
            context.antecedents.clone(),
            context.named.clone(),
            context.visible.clone(),
        )
    });
    let value = f();
    CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let context = contexts.last_mut().expect("scope outside core region");
        context.antecedents = saved.0;
        context.named = saved.1;
        let added = context.visible.len() - saved.2.len();
        context.visible = saved.2;
        context.visible.extend(std::iter::repeat_n(false, added));
    });
    value
}

fn compatible(want: Sort, have: Sort) -> bool {
    match (want, have) {
        (Sort::Player, Sort::Player)
        | (Sort::Card, Sort::Card)
        | (Sort::Token, Sort::Token)
        | (Sort::Spell, Sort::Spell)
        | (Sort::StackObject, Sort::Spell | Sort::StackObject)
        | (Sort::Permanent, Sort::Permanent | Sort::OfType(_) | Sort::Token)
        | (Sort::Amount, Sort::Amount)
        | (Sort::Pile, Sort::Pile) => true,
        (Sort::OfType(a), Sort::OfType(b)) => a == b,
        _ => false,
    }
}

fn kind_compatible(want: Sort, have: Kind) -> bool {
    match want {
        Sort::Amount => have == Kind::Number,
        Sort::Pile => have == Kind::Objects,
        Sort::Player
        | Sort::Card
        | Sort::Token
        | Sort::Spell
        | Sort::StackObject
        | Sort::Permanent
        | Sort::OfType(_) => matches!(have, Kind::Object | Kind::Objects),
    }
}

fn resolve(cardinality: Cardinality, want: Option<Sort>, prefer: Option<Site>) -> Option<RefId> {
    CONTEXTS.with(|contexts| {
        let contexts = contexts.borrow();
        let context = contexts.last()?;
        if let Some(site) = prefer {
            return context
                .antecedents
                .iter()
                .find(|antecedent| antecedent.site == site)
                .and_then(|antecedent| {
                    let reachable = want.map_or(
                        matches!(antecedent.kind, Kind::Object | Kind::Objects),
                        |wanted| {
                            antecedent.sort.map_or_else(
                                || kind_compatible(wanted, antecedent.kind),
                                |have| compatible(wanted, have),
                            )
                        },
                    );
                    (antecedent.cardinality == cardinality && reachable)
                        .then_some(antecedent.reference)
                });
        }
        let mut candidates = context.antecedents.iter().filter(|antecedent| {
            antecedent.cardinality == cardinality
                && want.map_or(
                    matches!(antecedent.kind, Kind::Object | Kind::Objects),
                    |wanted| {
                        antecedent.sort.map_or_else(
                            || kind_compatible(wanted, antecedent.kind),
                            |have| compatible(wanted, have),
                        )
                    },
                )
        });
        let first = candidates.next()?;
        if let Some(second) = candidates.next() {
            let exact_then_widened = want.is_some_and(|wanted| {
                first.sort == Some(wanted)
                    && second
                        .sort
                        .is_none_or(|have| have != wanted && compatible(wanted, have))
            });
            assert!(
                exact_then_widened,
                "ambiguous discourse anaphor during lowering"
            );
        }
        Some(first.reference)
    })
}

pub(crate) fn it() -> Option<RefId> {
    resolve(Cardinality::One, None, Some(Site::Loop))
        .or_else(|| resolve(Cardinality::One, None, Some(Site::Candidate)))
        .or_else(|| resolve(Cardinality::One, None, Some(Site::Frame)))
        .or_else(|| resolve(Cardinality::One, None, None))
        .or_else(|| resolve(Cardinality::Many, None, None))
        // The generated corpus still spells a lone announced target as `It`.
        // Scope elaboration can resolve that form without ambiguity; two or
        // more target slots must use their explicit `Target(n)` names.
        .or_else(|| {
            read(|context| match context.targets.as_slice() {
                [target] => Some(*target),
                _ => None,
            })
        })
}
pub(crate) fn that(sort: Sort) -> Option<RefId> {
    resolve(Cardinality::One, Some(sort), Some(Site::Frame))
        .or_else(|| resolve(Cardinality::One, Some(sort), None))
        // Semantic `That(Permanent)` also names a plural Choose binder in
        // costs such as "sacrifice three creatures". Core register kinds
        // preserve that cardinality even though the authored sort is singular.
        .or_else(|| resolve(Cardinality::Many, Some(sort), Some(Site::Frame)))
        .or_else(|| resolve(Cardinality::Many, Some(sort), None))
        // A lone announced target is also an unambiguous sorted antecedent.
        // The semantic corpus uses `That(Creature)` after naming Target(0)
        // explicitly in an earlier clause of the same ability.
        .or_else(|| {
            read(|context| match context.targets.as_slice() {
                [target] => Some(*target),
                _ => None,
            })
        })
}
pub(crate) fn they(sort: Option<Sort>) -> Option<RefId> {
    resolve(Cardinality::Many, sort, Some(Site::Frame))
        .or_else(|| resolve(Cardinality::Many, sort, None))
}
pub(crate) fn amount() -> Option<RefId> {
    resolve(Cardinality::One, Some(Sort::Amount), None)
}
pub(crate) fn allotment() -> Option<RefId> {
    resolve(Cardinality::One, Some(Sort::Amount), Some(Site::Allotment))
}

pub(crate) fn candidate_region<T>(f: impl FnOnce() -> T) -> deckmaste_core::Region<T> {
    if CONTEXTS.with(|contexts| contexts.borrow().is_empty()) {
        let mut params = Vec::new();
        let candidate = push_param(&mut params, Kind::Object, Provenance::Candidate);
        let source = push_param(&mut params, Kind::Object, Provenance::Source);
        let controller = push_param(&mut params, Kind::Object, Provenance::Controller);
        let definitions: Vec<_> = params.iter().map(|param| param.kind).collect();
        let visible = vec![true; definitions.len()];
        let (context, body) = with_pushed_context(
            Context {
                params,
                definitions,
                visible,
                source: Some(source),
                controller: Some(controller),
                event_object: None,
                event_patient: None,
                event_actor: None,
                defending_player: None,
                targets: Vec::new(),
                x: None,
                antecedents: vec![Antecedent {
                    reference: candidate,
                    kind: Kind::Object,
                    cardinality: Cardinality::One,
                    sort: None,
                    site: Site::Candidate,
                }],
                named: HashMap::new(),
            },
            f,
        );
        return deckmaste_core::Region::new(context.params.into(), body);
    }
    let (params, body) = in_child([(Kind::Object, Provenance::Candidate)], || {
        push_antecedent(
            RefId(0),
            Kind::Object,
            Cardinality::One,
            None,
            Site::Candidate,
        );
        f()
    });
    deckmaste_core::Region::new(params, body)
}
