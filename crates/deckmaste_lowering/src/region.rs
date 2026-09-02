use std::cell::RefCell;
use std::sync::Arc;

use deckmaste_core::DefId;
use deckmaste_core::Kind;
use deckmaste_core::Param;
use deckmaste_core::Provenance;
use deckmaste_core::RefId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RegionKind {
    Spell,
    Activated,
    Triggered,
    Mode,
}

#[derive(Debug, Clone)]
struct Context {
    params: Arc<[Param]>,
    source: RefId,
    controller: RefId,
    event_object: Option<RefId>,
    event_patient: Option<RefId>,
    event_actor: Option<RefId>,
    defending_player: RefId,
    targets: Arc<[RefId]>,
    x: RefId,
}

thread_local! {
    static CONTEXTS: RefCell<Vec<Context>> = const { RefCell::new(Vec::new()) };
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
    let source = push_param(&mut params, Kind::Object, Provenance::Source);
    let controller = push_param(&mut params, Kind::Object, Provenance::Controller);
    // Every ability-shaped region uses the same ABI prefix. Channels that a
    // particular invocation cannot supply are represented as unavailable;
    // keeping their ordinals stable makes mode/delayed regions independently
    // closed and gives target slots one fixed offset.
    let _ = kind;
    let event_object = Some(push_param(
        &mut params,
        Kind::Object,
        Provenance::EventObject,
    ));
    let event_patient = Some(push_param(
        &mut params,
        Kind::Object,
        Provenance::EventPatient,
    ));
    let event_actor = Some(push_param(
        &mut params,
        Kind::Object,
        Provenance::EventActor,
    ));
    let defending_player = push_param(&mut params, Kind::Object, Provenance::DefendingPlayer);
    let targets: Arc<[RefId]> = (0..target_count)
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
    let x = push_param(&mut params, Kind::Number, Provenance::AnnouncedX);
    Context {
        params: params.into(),
        source,
        controller,
        event_object,
        event_patient,
        event_actor,
        defending_player,
        targets,
        x,
    }
}

pub(crate) fn in_region<T>(
    kind: RegionKind,
    target_count: usize,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let context = context(kind, target_count);
    let params = context.params.clone();
    CONTEXTS.with(|contexts| contexts.borrow_mut().push(context));
    let value = f();
    let popped = CONTEXTS.with(|contexts| contexts.borrow_mut().pop());
    debug_assert!(popped.is_some());
    (params, value)
}

fn read(f: impl FnOnce(&Context) -> RefId) -> Option<RefId> {
    CONTEXTS.with(|contexts| contexts.borrow().last().map(f))
}

pub(crate) fn source() -> Option<RefId> {
    read(|context| context.source)
}

pub(crate) fn controller() -> Option<RefId> {
    read(|context| context.controller)
}

pub(crate) fn event_object() -> Option<RefId> {
    CONTEXTS.with(|contexts| contexts.borrow().last().and_then(|c| c.event_object))
}

pub(crate) fn event_patient() -> Option<RefId> {
    CONTEXTS.with(|contexts| contexts.borrow().last().and_then(|c| c.event_patient))
}

pub(crate) fn event_actor() -> Option<RefId> {
    CONTEXTS.with(|contexts| contexts.borrow().last().and_then(|c| c.event_actor))
}

pub(crate) fn defending_player() -> Option<RefId> {
    read(|context| context.defending_player)
}

pub(crate) fn target(index: usize) -> Option<RefId> {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow()
            .last()
            .and_then(|context| context.targets.get(index).copied())
    })
}

pub(crate) fn x() -> Option<RefId> {
    read(|context| context.x)
}
