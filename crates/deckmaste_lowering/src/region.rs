use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_core::DefId;
use deckmaste_core::Kind;
use deckmaste_core::Param;
use deckmaste_core::Provenance;
use deckmaste_core::RefId;
use deckmaste_semantics::Ident;
use deckmaste_semantics::Sort;

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
    ExecutionFrame,
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
    /// True once the antecedent has crossed a region boundary and survives
    /// only as a declared capture (ADR law 7). A region's own antecedents
    /// outrank captured ones on the general search ([CR#608.2h]).
    inherited: bool,
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
    /// ADR law 8: the card's linked memory cells this region declares as
    /// `Provenance::Linked` parameters, and the register each landed in.
    linked: HashMap<Ident, RefId>,
}

/// The card-scoped linked-memory plan ([CR#607.1], ADR law 8).
///
/// Lowering is context-free per ability EXCEPT for what a region declares, so
/// a cell read in one ability and written in another needs the card's whole
/// text before either ability can be lowered. `lower_card` therefore lowers
/// the card twice when — and only when — the first pass sees a cell read at
/// all: pass one collects which abilities read which cells and which cells the
/// card writes, pass two declares the surviving cells as parameters. A read
/// whose cell no ability on the card writes is a LOWERING ERROR naming the
/// card, never a register that resolves to nothing at run time.
#[derive(Debug, Clone, Default)]
pub(crate) struct CellPlan {
    /// `(ability region ordinal, cell)` — every linked read, keyed by the
    /// ability region whose parameter list must carry it.
    reads: std::collections::BTreeSet<(u32, Ident)>,
    /// Every cell the card writes, with the runtime shape written.
    writes: HashMap<Ident, Kind>,
}

impl CellPlan {
    fn is_empty(&self) -> bool {
        self.reads.is_empty()
    }

    /// The cells one ability region declares, in a stable order.
    fn declared(&self, ability: u32) -> Vec<(Ident, Kind)> {
        self.reads
            .iter()
            .filter(|(owner, _)| *owner == ability)
            .filter_map(|(_, cell)| self.writes.get(cell).map(|kind| (*cell, *kind)))
            .collect()
    }
}

thread_local! {
    static CONTEXTS: RefCell<Vec<Context>> = const { RefCell::new(Vec::new()) };
    /// Pass one's accumulator; `Some` only while collecting.
    static COLLECT: RefCell<Option<CellPlan>> = const { RefCell::new(None) };
    /// Pass two's settled plan; `Some` only while lowering against it.
    static PLAN: RefCell<Option<CellPlan>> = const { RefCell::new(None) };
    /// The ability region currently being lowered, and the next ordinal to
    /// hand out. Ability regions are entered in a deterministic order, so the
    /// two passes agree on the numbering.
    static ABILITY: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    static NEXT_ABILITY: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// The card-level compiler context. Per-node [`crate::Lower`] calls stay
    /// infallible; a context-sensitive refusal is recorded here and observed
    /// by the card walk after the recursive lowering returns.
    static COMPILER: RefCell<CompilerContext> = const { RefCell::new(CompilerContext::new()) };
}

struct CompilerContext {
    card: Option<Arc<str>>,
    refusal: Option<String>,
}

struct CompilerGuard {
    previous: Option<CompilerContext>,
}

impl CompilerGuard {
    fn install(card: &str) -> Self {
        let previous = COMPILER.with(|slot| {
            slot.replace(CompilerContext {
                card: Some(Arc::from(card)),
                refusal: None,
            })
        });
        Self {
            previous: Some(previous),
        }
    }

    fn finish(mut self) -> CompilerContext {
        let previous = self.previous.take().expect("installed compiler context");
        COMPILER.with(|slot| slot.replace(previous))
    }
}

impl Drop for CompilerGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.take() {
            COMPILER.with(|slot| {
                slot.replace(previous);
            });
        }
    }
}

impl CompilerContext {
    const fn new() -> Self {
        Self {
            card: None,
            refusal: None,
        }
    }
}

/// Compile `f` with one explicit card-level compiler context. Only this walk
/// entry observes the refusal channel; the recursive [`crate::Lower`] map
/// remains infallible and its partially built value is discarded on refusal.
///
/// The previous context is restored before returning, so nested card lowering
/// keeps the outer card's diagnostic identity.
pub(crate) fn in_card<T>(card: &str, f: impl FnOnce() -> T) -> Result<T, String> {
    let guard = CompilerGuard::install(card);
    let value = f();
    let completed = guard.finish();
    completed.refusal.map_or(Ok(value), Err)
}

/// Refuse to compile the card's text, naming the card and the reason.
///
/// The ADR routes R1/R2 refusals here (law 12: resolution happens once, in
/// lowering, "as per-card diagnostics"). The first refusal wins; lowering may
/// finish the current infallible arm, but [`crate::lower_card`] discards its
/// image and returns the recorded [`crate::Diagnostic`]. Outside a card walk,
/// a refusal remains loud so isolated per-node lowering cannot lose it.
pub(crate) fn refuse(reason: &str) {
    COMPILER.with(|slot| {
        let mut context = slot.borrow_mut();
        let Some(card) = context.card.as_ref() else {
            panic!("<unknown card>: {reason}");
        };
        if context.refusal.is_none() {
            context.refusal = Some(format!("{card}: {reason}"));
        }
    });
}

/// Pass one: lower `f` with linked reads RECORDED instead of refused, and
/// return the plan the second pass needs.
pub(crate) fn collect_cells<T>(f: impl FnOnce() -> T) -> (CellPlan, T) {
    COLLECT.with(|slot| *slot.borrow_mut() = Some(CellPlan::default()));
    NEXT_ABILITY.with(|next| next.set(0));
    let value = f();
    let plan = COLLECT
        .with(|slot| slot.borrow_mut().take())
        .unwrap_or_default();
    (plan, value)
}

/// Pass two: lower `f` with `plan`'s cells declared as region parameters.
pub(crate) fn with_cells<T>(plan: CellPlan, f: impl FnOnce() -> T) -> T {
    PLAN.with(|slot| *slot.borrow_mut() = Some(plan));
    NEXT_ABILITY.with(|next| next.set(0));
    let value = f();
    PLAN.with(|slot| *slot.borrow_mut() = None);
    value
}

/// Whether the plan the first pass produced needs a second pass at all.
pub(crate) fn plan_is_empty(plan: &CellPlan) -> bool {
    plan.is_empty()
}

/// Read one of the card's linked memory cells ([CR#607.1]) from the region
/// currently being lowered — the fallback when the name is not bound in this
/// region. Returns the declared parameter in pass two, a placeholder in pass
/// one (whose result is discarded), and `None` outside a card compile, where
/// the caller's own refusal stands.
pub(crate) fn cell_read(name: &Ident) -> Option<RefId> {
    if let Some(reference) = read(|context| context.linked.get(name).copied()) {
        return Some(reference);
    }
    COLLECT.with(|slot| {
        slot.borrow_mut().as_mut().map(|plan| {
            plan.reads
                .insert((ABILITY.with(std::cell::Cell::get), *name));
            // Pass one's lowered value is discarded; only the plan escapes.
            RefId(0)
        })
    })
}

/// Declare that this card writes the linked memory cell `name` with runtime
/// shape `kind`, and report whether a reading ability elsewhere on the card
/// actually reads it — which is what makes the write a `Remember` instruction
/// rather than a purely local binding.
pub(crate) fn cell_write(name: &Ident, kind: Kind) -> bool {
    COLLECT.with(|slot| {
        if let Some(plan) = slot.borrow_mut().as_mut() {
            plan.writes.insert(*name, kind);
        }
    });
    PLAN.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|plan| plan.reads.iter().any(|(_, cell)| cell == name))
    })
}

/// Take the next ability-region ordinal and make it current for the duration
/// of `f`. Ability regions nest (a granted ability inside another's body), so
/// the previous ordinal is restored.
fn in_ability_region<T>(f: impl FnOnce(u32) -> T) -> T {
    let ordinal = NEXT_ABILITY.with(|next| {
        let ordinal = next.get();
        next.set(ordinal + 1);
        ordinal
    });
    let previous = ABILITY.with(|current| current.replace(ordinal));
    let value = f(ordinal);
    ABILITY.with(|current| current.set(previous));
    value
}

/// Push this ability region's declared linked-memory parameters ([CR#607.1]).
/// They follow the engine-supplied prefix and precede any capture, so the parameter
/// order stays a function of the region's kind plus its plan.
fn declare_linked(context: &mut Context, ability: u32) {
    let cells = PLAN.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|plan| plan.declared(ability))
            .unwrap_or_default()
    });
    for (cell, kind) in cells {
        let reference = push_param(&mut context.params, kind, Provenance::Linked(cell));
        context.definitions.push(kind);
        context.visible.push(true);
        context.linked.insert(cell, reference);
    }
}

fn with_pushed_context<T>(context: Context, f: impl FnOnce() -> T) -> (Context, T) {
    CONTEXTS.with(|contexts| contexts.borrow_mut().push(context));
    let value = f();
    let context = CONTEXTS
        .with(|contexts| contexts.borrow_mut().pop())
        .expect("region context");
    (context, value)
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
    let source = Some(push_param(&mut params, Kind::Entity, Provenance::Source));
    let controller = Some(push_param(
        &mut params,
        Kind::Entity,
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
                Kind::Entity,
                Provenance::EventObject,
            )),
            Some(push_param(
                &mut params,
                Kind::Entity,
                Provenance::EventPatient,
            )),
            Some(push_param(
                &mut params,
                Kind::Entity,
                Provenance::EventActor,
            )),
            Some(push_param(
                &mut params,
                Kind::Entity,
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
                Kind::Entities,
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
            kind: Kind::Entity,
            cardinality: Cardinality::One,
            sort: None,
            site: Site::ExecutionFrame,
            inherited: false,
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
            inherited: false,
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
        linked: HashMap::new(),
    }
}

pub(crate) fn in_region<T>(
    kind: RegionKind,
    target_count: usize,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    in_ability_region(|ability| {
        let mut region = context(kind, target_count);
        declare_linked(&mut region, ability);
        let (context, value) = with_pushed_context(region, f);
        (context.params.into(), value)
    })
}

pub(crate) fn is_active() -> bool {
    CONTEXTS.with(|contexts| !contexts.borrow().is_empty())
}

/// Enter an ability region carried by a value built inside another region.
/// Its engine-supplied ABI remains its own (source/controller/event roles/targets),
/// followed by explicit captures of the enclosing register file.
pub(crate) fn in_carried_region<T>(
    kind: RegionKind,
    target_count: usize,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    in_ability_region(|ability| in_carried_region_inner(kind, target_count, ability, f))
}

fn in_carried_region_inner<T>(
    kind: RegionKind,
    target_count: usize,
    ability: u32,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let parent = CONTEXTS
        .with(|contexts| contexts.borrow().last().cloned())
        .expect("carried core region outside a parent region");
    let mut child = context(kind, target_count);
    declare_linked(&mut child, ability);
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
            antecedent.inherited = true;
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

/// Enter a nested region. Engine-supplied parameters come first; outer registers
/// are captured in definition order. The stable ABI is more valuable here
/// than minimizing a serialized capture list.
pub(crate) fn in_child<T>(
    supplied: impl IntoIterator<Item = (Kind, Provenance)>,
    f: impl FnOnce() -> T,
) -> (Arc<[Param]>, T) {
    let parent = CONTEXTS
        .with(|contexts| contexts.borrow().last().cloned())
        .expect("nested core region outside an ability region");
    let mut params = Vec::new();
    for (kind, provenance) in supplied {
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
                        | Provenance::Candidate(_) => Provenance::Capture(outer),
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
            antecedent.inherited = true;
            Some(antecedent)
        })
        .collect();
    let named = parent
        .named
        .into_iter()
        .filter_map(|(name, reference)| remap(reference, &captures).map(|r| (name, r)))
        .collect();
    // A nested region inherits its enclosing ability's linked cells: `in_child`
    // re-declares a parent `Provenance::Linked` parameter with the same
    // provenance, so the engine supplies it from the same memory cell.
    let linked = parent
        .linked
        .into_iter()
        .filter_map(|(cell, reference)| remap(reference, &captures).map(|r| (cell, r)))
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
        linked,
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
/// Bind the region's event-object slot to a payment product ([CR#118.8]).
/// Announcement regions (spell/activated/mode) declare no event roles, so the
/// paid product is what "the sacrificed creature" names there.
pub(crate) fn set_event_object(reference: RefId) {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow_mut()
            .last_mut()
            .expect("event-object binding outside core region")
            .event_object
            .get_or_insert(reference);
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

/// The newest antecedent's register together with its runtime shape — what a
/// `Noting` publishes into a memory cell.
pub(crate) fn newest_antecedent_typed() -> Option<(RefId, Kind)> {
    CONTEXTS.with(|contexts| {
        contexts.borrow().last().and_then(|context| {
            context
                .antecedents
                .first()
                .map(|antecedent| (antecedent.reference, antecedent.kind))
        })
    })
}

pub(crate) fn named(name: &Ident) -> Option<RefId> {
    read(|context| context.named.get(name).copied())
}

/// The declared register shape of `reference` in the region being built —
/// what a group read has to agree with ([CR#700.3b]: a pile register is not
/// an Entity group). `None` when the register is not (yet) declared here.
pub(crate) fn register_kind(reference: RefId) -> Option<Kind> {
    CONTEXTS.with(|contexts| {
        contexts
            .borrow()
            .last()
            .and_then(|context| context.definitions.get(reference.0 as usize).copied())
    })
}

/// The group spelling of a register read, in the collection domain the
/// register's own declared shape names ([CR#700.3a..700.3b]).
pub(crate) fn group_read(reference: RefId) -> deckmaste_core::Selection {
    match register_kind(reference) {
        Some(Kind::Pile) => deckmaste_core::Selection::Pile(reference),
        _ => deckmaste_core::Selection::Reg(reference),
    }
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
                    inherited: false,
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

/// Run `f` with its own ANAPHORA scope: bindings it introduces are readable
/// inside it and invisible after, while the definitions themselves stay live
/// (unlike [`scoped_antecedents`], which also retires them).
///
/// A cost block is exactly this shape ([CR#601.2b]): its payment subjects are
/// read by the verbs inside the block, and the ability body reads the paid
/// product only through the channel the announcement declares for it — never
/// as a bare "it" competing with the effect's own antecedents.
pub(crate) fn scoped_anaphora<T>(f: impl FnOnce() -> T) -> T {
    let saved = CONTEXTS.with(|contexts| {
        let contexts = contexts.borrow();
        let context = contexts.last().expect("scope outside core region");
        (context.antecedents.clone(), context.named.clone())
    });
    let value = f();
    CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let context = contexts.last_mut().expect("scope outside core region");
        context.antecedents = saved.0;
        context.named = saved.1;
    });
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

/// Whether an antecedent of sort `have` can answer a mention of `want`.
///
/// This is the semantic discourse relation: exact matches plus the three
/// widened nouns supported by the Idris model.
fn compatible(want: Sort, have: Sort) -> bool {
    use deckmaste_semantics::Sort;

    want == have
        || matches!(
            (want, have),
            (Sort::Card, Sort::Permanent | Sort::OfType(_))
                | (Sort::StackObject, Sort::Spell)
                | (Sort::Permanent, Sort::Token | Sort::OfType(_))
        )
}

/// Whether a register of shape `have` can answer a mention of `want` — the
/// Entity, number, and pile domains never cross ([CR#700.3b]).
fn kind_compatible(want: Sort, have: Kind) -> bool {
    match want {
        Sort::Amount => have == Kind::Number,
        Sort::Pile => have == Kind::Pile,
        _ => matches!(have, Kind::Entity | Kind::Entities),
    }
}

/// Whether `antecedent` can answer a mention of `cardinality`/`want`.
fn reachable(antecedent: &Antecedent, cardinality: Cardinality, want: Option<Sort>) -> bool {
    antecedent.cardinality == cardinality
        && want.map_or(
            matches!(antecedent.kind, Kind::Entity | Kind::Entities),
            |wanted| {
                antecedent.sort.map_or_else(
                    || kind_compatible(wanted, antecedent.kind),
                    |have| compatible(wanted, have),
                )
            },
        )
}

/// R1/R2 over one discourse tier, nearest first. `None` when the tier holds no
/// compatible antecedent; a panic when it holds two that the exact-then-widened
/// carve-out does not separate. A refusal records the collision on the
/// compiler context and returns the first register as a discarded placeholder.
fn resolve_tier<'a>(
    antecedents: impl Iterator<Item = &'a Antecedent>,
    cardinality: Cardinality,
    want: Option<Sort>,
) -> Option<RefId> {
    let mut candidates = antecedents.filter(|a| reachable(a, cardinality, want));
    let first = candidates.next()?;
    if let Some(second) = candidates.next() {
        let exact_then_widened = want.is_some_and(|wanted| {
            first.sort == Some(wanted)
                && second
                    .sort
                    .is_none_or(|have| have != wanted && compatible(wanted, have))
        });
        if !exact_then_widened {
            refuse(&format!(
                "ambiguous discourse anaphor during lowering — two compatible \
                 antecedents are in scope, {:?} ({:?}) and {:?} ({:?}); the rules \
                 supply no proximity tiebreak ([CR#608.2c]), so the card must name \
                 the one it means",
                first.reference, first.sort, second.reference, second.sort,
            ));
        }
    }
    Some(first.reference)
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
                    reachable(antecedent, cardinality, want).then_some(antecedent.reference)
                });
        }
        // Two discourse tiers, region-local before captured. A region is
        // applied once per entry ([CR#608.2h] determines its information
        // then), so a magnitude or product pinned inside a loop body is a
        // different value from the enclosing region's and outranks it; the
        // outer one survives only as a declared capture (ADR law 7). R2 still
        // refuses WITHIN a tier: two magnitudes pinned by the same region have
        // no proximity tiebreak in the rules ([CR#608.2c] — read the whole
        // text; [CR#607.1] pins a reference by linkage, never by position).
        resolve_tier(
            context.antecedents.iter().filter(|a| !a.inherited),
            cardinality,
            want,
        )
        .or_else(|| {
            resolve_tier(
                context.antecedents.iter().filter(|a| a.inherited),
                cardinality,
                want,
            )
        })
    })
}

pub(crate) fn it() -> Option<RefId> {
    resolve(Cardinality::One, None, Some(Site::Loop))
        .or_else(|| resolve(Cardinality::One, None, Some(Site::Candidate)))
        .or_else(|| resolve(Cardinality::One, None, Some(Site::ExecutionFrame)))
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
    resolve(Cardinality::One, Some(sort), Some(Site::ExecutionFrame))
        .or_else(|| resolve(Cardinality::One, Some(sort), None))
        // Semantic `That(Permanent)` also names a plural Choose binder in
        // costs such as "sacrifice three creatures". Core register kinds
        // preserve that cardinality even though the authored sort is singular.
        .or_else(|| resolve(Cardinality::Many, Some(sort), Some(Site::ExecutionFrame)))
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
    resolve(Cardinality::Many, sort, Some(Site::ExecutionFrame))
        .or_else(|| resolve(Cardinality::Many, sort, None))
}
pub(crate) fn amount() -> Option<RefId> {
    resolve(Cardinality::One, Some(Sort::Amount), None)
}
pub(crate) fn allotment() -> Option<RefId> {
    resolve(Cardinality::One, Some(Sort::Amount), Some(Site::Allotment))
}

/// A per-candidate predicate region whose declared Entity domain is the
/// narrowest one the lowered predicate's own atoms admit ([CR#109.1,102.1] —
/// ADR law 2). Every predicate region lowering builds goes through here, so a
/// Player-only filter never declares the Object domain and the reverse.
pub(crate) fn predicate_region(
    f: impl FnOnce() -> deckmaste_core::Predicate,
) -> deckmaste_core::Region<deckmaste_core::Predicate> {
    let region = candidate_region(f);
    let domain = region.body.subject_domain();
    let params: Vec<deckmaste_core::Param> = region
        .params
        .iter()
        .map(|param| match param.provenance {
            Provenance::Candidate(_) => deckmaste_core::Param {
                provenance: Provenance::Candidate(domain),
                ..param.clone()
            },
            _ => param.clone(),
        })
        .collect();
    deckmaste_core::Region::new(params.into(), region.body)
}

pub(crate) fn candidate_region<T>(f: impl FnOnce() -> T) -> deckmaste_core::Region<T> {
    if CONTEXTS.with(|contexts| contexts.borrow().is_empty()) {
        let mut params = Vec::new();
        let candidate = push_param(
            &mut params,
            Kind::Entity,
            Provenance::Candidate(deckmaste_core::Domain::Entity),
        );
        let source = push_param(&mut params, Kind::Entity, Provenance::Source);
        let controller = push_param(&mut params, Kind::Entity, Provenance::Controller);
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
                    kind: Kind::Entity,
                    cardinality: Cardinality::One,
                    sort: None,
                    site: Site::Candidate,
                    inherited: false,
                }],
                named: HashMap::new(),
                linked: HashMap::new(),
            },
            f,
        );
        return deckmaste_core::Region::new(context.params.into(), body);
    }
    let (params, body) = in_child(
        [(
            Kind::Entity,
            Provenance::Candidate(deckmaste_core::Domain::Entity),
        )],
        || {
            push_antecedent(
                RefId(0),
                Kind::Entity,
                Cardinality::One,
                None,
                Site::Candidate,
            );
            f()
        },
    );
    deckmaste_core::Region::new(params, body)
}

#[cfg(test)]
mod tests {
    use deckmaste_core::DefId;
    use deckmaste_core::Kind;
    use deckmaste_core::Param;
    use deckmaste_core::Provenance;
    use deckmaste_core::RefId;

    use super::Cardinality;
    use super::RegionKind;
    use super::Site;

    /// The `(kind, provenance)` pairs a region declares, in order.
    fn abi(params: &[Param]) -> Vec<(Kind, Provenance)> {
        params
            .iter()
            .map(|param| (param.kind, param.provenance.clone()))
            .collect()
    }

    /// Every region's parameters are dense and ordered from `DefId(0)` — the
    /// invariant `deckmaste_core::validate` enforces at load (ADR law 2).
    fn assert_dense(params: &[Param]) {
        for (index, param) in params.iter().enumerate() {
            assert_eq!(
                param.def,
                DefId(u32::try_from(index).expect("param count fits u32")),
                "parameter {index} is out of sequence"
            );
        }
    }

    /// An announcement region declares source, controller and announced X, plus
    /// one parameter per target slot in order ([CR#601.2b,601.2c], ADR law 2).
    #[test]
    fn an_announcement_region_declares_source_controller_targets_then_x() {
        let (params, ()) = super::in_region(RegionKind::Spell, 2, || ());
        assert_dense(&params);
        assert_eq!(
            abi(&params),
            vec![
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
                (Kind::Entities, Provenance::AnnouncedTarget(0)),
                (Kind::Entities, Provenance::AnnouncedTarget(1)),
                (Kind::Number, Provenance::AnnouncedX),
            ]
        );
    }

    /// An event-observing region declares the fixed event-role prefix, so a
    /// trigger and a replacement read the same registers (ADR law 2).
    #[test]
    fn an_event_region_declares_the_fixed_event_role_prefix() {
        let (params, ()) = super::in_region(RegionKind::Triggered, 0, || ());
        assert_dense(&params);
        assert_eq!(
            abi(&params),
            vec![
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
                (Kind::Entity, Provenance::EventObject),
                (Kind::Entity, Provenance::EventPatient),
                (Kind::Entity, Provenance::EventActor),
                (Kind::Entity, Provenance::DefendingPlayer),
                (Kind::Number, Provenance::EventAmount),
                (Kind::Number, Provenance::AnnouncedX),
            ]
        );
        assert_eq!(
            abi(&params)[..7],
            abi(&deckmaste_core::event_region_params())[..7],
            "the prefix is the one core publishes"
        );
    }

    /// A static ability pays no cost and announces no X ([CR#601.2b] applies to
    /// casting and activation, not to a static), so its region stops at the
    /// event roles.
    #[test]
    fn a_static_region_declares_no_announced_x() {
        let (params, ()) = super::in_region(RegionKind::Static, 0, || ());
        assert_dense(&params);
        assert!(
            !params
                .iter()
                .any(|param| param.provenance == Provenance::AnnouncedX),
            "a static ability has no announcement to declare X in"
        );
        assert_eq!(params.len(), 7);
    }

    /// Instruction definitions continue the parameter sequence densely
    /// (ADR law 3): the first definition of a spell region is register 3.
    #[test]
    fn definitions_continue_the_parameter_sequence() {
        let (params, defs) = super::in_region(RegionKind::Spell, 0, || {
            [
                super::define(Kind::Entities),
                super::define(Kind::Number),
                super::define(Kind::Entity),
            ]
        });
        assert_eq!(params.len(), 3, "source, controller, X");
        assert_eq!(defs, [DefId(3), DefId(4), DefId(5)]);
    }

    /// A nested region declares its own engine-supplied parameters FIRST and then
    /// captures the enclosing register file in definition order (ADR law 7),
    /// so a body reads nothing it did not declare.
    #[test]
    fn a_nested_region_declares_supplied_params_then_captures_the_enclosing_file() {
        let (_, inner) = super::in_region(RegionKind::Spell, 1, || {
            let (params, ()) = super::in_child([(Kind::Entity, Provenance::LoopElement)], || ());
            params
        });
        assert_dense(&inner);
        assert_eq!(
            abi(&inner),
            vec![
                (Kind::Entity, Provenance::LoopElement),
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
                (Kind::Entities, Provenance::AnnouncedTarget(0)),
                (Kind::Number, Provenance::AnnouncedX),
            ],
            "the element takes register 0 and the enclosing parameters follow"
        );
    }

    /// A capture of a nested-only role (a loop element, an allotment, a
    /// candidate) becomes an explicit `Capture` of the outer register rather
    /// than repeating the role — the outer loop's element is not this body's
    /// own element (ADR law 7).
    #[test]
    fn a_nested_only_role_is_captured_not_repeated() {
        let (_, inner) = super::in_region(RegionKind::Spell, 0, || {
            let (_, inner) = super::in_child([(Kind::Entity, Provenance::LoopElement)], || {
                let (params, ()) =
                    super::in_child([(Kind::Entity, Provenance::LoopElement)], || ());
                params
            });
            inner
        });
        assert_eq!(
            abi(&inner),
            vec![
                (Kind::Entity, Provenance::LoopElement),
                (Kind::Entity, Provenance::Capture(RefId(0))),
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
                (Kind::Number, Provenance::AnnouncedX),
            ]
        );
    }

    /// A carried body (a delayed trigger, a floating replacement) keeps its OWN
    /// engine-supplied ABI and takes the enclosing registers as explicit captures
    /// afterwards ([CR#603.7,603.12], ADR law 7).
    #[test]
    fn a_carried_region_keeps_its_own_abi_then_captures() {
        let (_, carried) = super::in_region(RegionKind::Spell, 0, || {
            let (params, ()) = super::in_carried_region(RegionKind::Triggered, 0, || ());
            params
        });
        assert_dense(&carried);
        let roles = abi(&carried);
        assert_eq!(
            roles[..8],
            abi(&super::in_region(RegionKind::Triggered, 0, || ()).0)[..8],
            "the carried trigger's own ABI comes first"
        );
        assert_eq!(
            roles[8..],
            vec![
                (Kind::Entity, Provenance::Capture(RefId(0))),
                (Kind::Entity, Provenance::Capture(RefId(1))),
                (Kind::Number, Provenance::Capture(RefId(2))),
            ],
            "then the enclosing spell region's three registers"
        );
    }

    /// R1: "it" reads the NEAREST antecedent — the most recently pushed one.
    #[test]
    fn it_resolves_to_the_nearest_antecedent() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            let first = super::define(Kind::Entity);
            super::push_antecedent(
                first.into(),
                Kind::Entity,
                Cardinality::One,
                None,
                Site::ExecutionFrame,
            );
            let second = super::define(Kind::Entity);
            super::push_antecedent(
                second.into(),
                Kind::Entity,
                Cardinality::One,
                None,
                Site::ExecutionFrame,
            );
            super::it()
        });
        assert_eq!(reference, Some(RefId(4)), "the later definition wins");
    }

    /// A loop element outranks an ordinary frame antecedent: inside a body that
    /// iterates, "it" is the element ([CR#608.2]).
    #[test]
    fn it_prefers_a_loop_element_over_a_frame_antecedent() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            let product = super::define(Kind::Entity);
            super::push_antecedent(
                product.into(),
                Kind::Entity,
                Cardinality::One,
                None,
                Site::ExecutionFrame,
            );
            super::push_antecedent(RefId(0), Kind::Entity, Cardinality::One, None, Site::Loop);
            super::it()
        });
        assert_eq!(reference, Some(RefId(0)));
    }

    /// R2: two equally compatible antecedents on the general search are a
    /// refusal, not a silent pick — the resolver never guesses between two
    /// equally good readings.
    #[test]
    fn a_second_compatible_antecedent_refuses() {
        let error = crate::lower_for_test(|| {
            super::in_region(RegionKind::Spell, 0, || {
                for _ in 0..2 {
                    let def = super::define(Kind::Entity);
                    super::push_antecedent(
                        def.into(),
                        Kind::Entity,
                        Cardinality::One,
                        Some(deckmaste_semantics::Sort::Card),
                        Site::Product,
                    );
                }
                super::that(deckmaste_semantics::Sort::Card)
            })
        })
        .expect_err("two compatible antecedents are ambiguous");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error.message.contains("ambiguous discourse anaphor"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    /// R1 is what a SITE-preferred search runs: among the antecedents at the
    /// preferred site the nearest wins outright, which is why a clause that
    /// moves two cards in turn still reads "that card" as the second. R2's
    /// refusal (above) governs the general search that follows.
    #[test]
    fn two_antecedents_at_a_preferred_site_resolve_to_the_nearest() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            for _ in 0..2 {
                let def = super::define(Kind::Entity);
                super::push_antecedent(
                    def.into(),
                    Kind::Entity,
                    Cardinality::One,
                    Some(deckmaste_semantics::Sort::Card),
                    Site::ExecutionFrame,
                );
            }
            super::that(deckmaste_semantics::Sort::Card)
        });
        assert_eq!(reference, Some(RefId(4)), "the later definition wins");
    }

    /// An EXACT sort match followed by a widened one is not ambiguous: "that
    /// card" prefers the card antecedent over a permanent one.
    #[test]
    fn an_exact_sort_beats_a_widened_one_without_refusing() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            let permanent = super::define(Kind::Entity);
            super::push_antecedent(
                permanent.into(),
                Kind::Entity,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Permanent),
                Site::ExecutionFrame,
            );
            let card = super::define(Kind::Entity);
            super::push_antecedent(
                card.into(),
                Kind::Entity,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                Site::ExecutionFrame,
            );
            super::that(deckmaste_semantics::Sort::Card)
        });
        assert_eq!(reference, Some(RefId(4)));
    }

    /// A sorted anaphor SKIPS an incompatible antecedent rather than taking the
    /// nearest: "that player" is not the card the previous clause moved.
    #[test]
    fn a_sorted_anaphor_skips_an_incompatible_antecedent() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            let player = super::define(Kind::Entity);
            super::push_antecedent(
                player.into(),
                Kind::Entity,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Player),
                Site::ExecutionFrame,
            );
            let card = super::define(Kind::Entity);
            super::push_antecedent(
                card.into(),
                Kind::Entity,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                Site::ExecutionFrame,
            );
            super::that(deckmaste_semantics::Sort::Player)
        });
        assert_eq!(
            reference,
            Some(RefId(3)),
            "the player register, not the card"
        );
    }

    /// Cardinality separates the singular and plural anaphors: a group
    /// antecedent answers "they", never "that".
    #[test]
    fn cardinality_separates_the_singular_and_plural_anaphors() {
        let (_, (singular, plural)) = super::in_region(RegionKind::Spell, 0, || {
            let group = super::define(Kind::Entities);
            super::push_antecedent(
                group.into(),
                Kind::Entities,
                Cardinality::Many,
                Some(deckmaste_semantics::Sort::Card),
                Site::ExecutionFrame,
            );
            (
                super::they(Some(deckmaste_semantics::Sort::Card)),
                super::amount(),
            )
        });
        assert_eq!(singular, Some(RefId(3)), "the group answers `they`");
        assert_eq!(plural, None, "a group is not an amount");
    }

    /// An allotment is its own channel ([CR#601.2d]): a plain pinned amount in
    /// scope answers "that much" but never a distribution's share.
    #[test]
    fn an_allotment_read_is_its_own_channel() {
        let (_, (amount, allotment)) = super::in_region(RegionKind::Spell, 0, || {
            let pinned = super::define(Kind::Number);
            super::push_antecedent(
                pinned.into(),
                Kind::Number,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                Site::Product,
            );
            (super::amount(), super::allotment())
        });
        assert_eq!(amount, Some(RefId(3)));
        assert_eq!(allotment, None);
    }

    /// A block scope retires the definitions it introduced (ADR law 6): a
    /// sibling block neither reads them nor captures them.
    #[test]
    fn a_scoped_block_retires_its_own_definitions() {
        let (_, captured) = super::in_region(RegionKind::Spell, 0, || {
            super::scoped_antecedents(|| {
                let inner = super::define(Kind::Entity);
                super::push_antecedent(
                    inner.into(),
                    Kind::Entity,
                    Cardinality::One,
                    None,
                    Site::ExecutionFrame,
                );
            });
            let (params, ()) = super::in_child([], || ());
            params
        });
        assert_eq!(
            abi(&captured),
            vec![
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
                (Kind::Number, Provenance::AnnouncedX),
            ],
            "the closed block's definition is not visible to capture"
        );
    }

    /// A cost block scopes its ANAPHORA without retiring its definitions
    /// ([CR#601.2b]): the payment subject is read by the verbs that spend it,
    /// and the ability body reads the paid product only through the channel
    /// the announcement declares — but the register itself stays live.
    #[test]
    fn a_cost_block_scopes_anaphora_but_keeps_its_definitions_live() {
        let (_, (after, captured)) = super::in_region(RegionKind::Activated, 0, || {
            super::scoped_anaphora(|| {
                let paid = super::define(Kind::Entities);
                super::push_antecedent(
                    paid.into(),
                    Kind::Entities,
                    Cardinality::One,
                    Some(deckmaste_semantics::Sort::Permanent),
                    Site::ExecutionFrame,
                );
            });
            let (params, ()) = super::in_child([], || ());
            (super::that(deckmaste_semantics::Sort::Permanent), params)
        });
        assert_eq!(after, None, "the payment subject is out of anaphoric scope");
        assert_eq!(
            captured.len(),
            4,
            "but its register is still live and capturable"
        );
    }

    /// A target constraint sees only the target slots announced BEFORE it
    /// ([CR#601.2c]): slot 1's filter may read slot 0, never itself or a later
    /// slot, and never the announced X that follows the whole list.
    #[test]
    fn a_target_constraint_sees_only_earlier_slots() {
        let (_, seen) = super::in_region(RegionKind::Spell, 3, || {
            super::with_target_prefix(1, || (super::target(0), super::target(1), super::x()))
        });
        assert_eq!(seen.0, Some(RefId(2)), "slot 0 is in scope");
        assert_eq!(seen.1, None, "slot 1 is the one being constrained");
        assert_eq!(seen.2, None, "X is announced after the target list");
    }

    /// The prefix is RESTORED after the constraint: the ability body sees every
    /// slot and X again.
    #[test]
    fn the_full_announcement_returns_after_a_target_constraint() {
        let (_, seen) = super::in_region(RegionKind::Spell, 2, || {
            super::with_target_prefix(0, || ());
            (super::target(0), super::target(1), super::x())
        });
        assert_eq!(
            seen,
            (Some(RefId(2)), Some(RefId(3)), Some(RefId(4))),
            "the constraint's narrowing is undone"
        );
    }

    /// A named role is a register alias ([CR#607]): binding a name and reading
    /// it back yields the same register, and an unbound name yields nothing.
    #[test]
    fn a_named_role_aliases_a_register() {
        let (_, (bound, unbound)) = super::in_region(RegionKind::Spell, 0, || {
            super::bind_named("kept".into(), RefId(2));
            (
                super::named(&deckmaste_semantics::Ident::from("kept")),
                super::named(&deckmaste_semantics::Ident::from("absent")),
            )
        });
        assert_eq!(bound, Some(RefId(2)));
        assert_eq!(unbound, None);
    }

    /// A predicate lowered with no enclosing region still gets one: the
    /// candidate is parameter zero, followed by source and controller, so a
    /// carrier-relative filter has registers to read (ADR law 1).
    #[test]
    fn a_candidate_region_stands_alone_when_there_is_no_enclosing_region() {
        assert!(!super::is_active(), "no region is open");
        let region = super::candidate_region(|| deckmaste_core::Predicate::Any);
        assert_dense(&region.params);
        assert_eq!(
            abi(&region.params),
            vec![
                (
                    Kind::Entity,
                    Provenance::Candidate(deckmaste_core::Domain::Entity)
                ),
                (Kind::Entity, Provenance::Source),
                (Kind::Entity, Provenance::Controller),
            ]
        );
    }

    /// Inside a candidate region the candidate answers "it" ([CR#608.2] — the
    /// role the retired `Subject` named).
    #[test]
    fn a_candidate_region_answers_it_with_its_candidate() {
        let region = super::candidate_region(super::it);
        assert_eq!(region.body, Some(RefId(0)));
    }

    /// A lone announced target is an unambiguous antecedent, so the corpus's
    /// bare "it" after a single `target` clause resolves ([CR#601.2c]); two or
    /// more slots must be named explicitly and leave the anaphor unbound.
    #[test]
    fn a_lone_announced_target_answers_it_but_two_do_not() {
        let (_, one) = super::in_region(RegionKind::Spell, 1, super::it);
        assert_eq!(one, Some(RefId(2)));
        let (_, two) = super::in_region(RegionKind::Spell, 2, super::it);
        assert_eq!(two, None, "two slots are ambiguous — name them");
    }

    /// Drift guard for the Entity boundary: the SAME predicate must declare
    /// the same candidate domain whichever path builds its region
    /// ([CR#109.1,102.1] — ADR law 2). The two paths are lowering's
    /// [`super::predicate_region`] and core's `Region::over`/`Region::candidate`;
    /// the first case is the Ascend gate's count, an Object-domain census on
    /// both. A hand-written region that disagrees with either is drift.
    #[test]
    fn both_predicate_region_paths_declare_the_same_candidate_domain() {
        use crate::Lower as _;

        for (spelling, expected) in [
            (
                "And([InZone(Battlefield), ControlledBy(Ref(You))])",
                deckmaste_core::Domain::Object,
            ),
            ("Kind(Player)", deckmaste_core::Domain::Player),
            ("Any", deckmaste_core::Domain::Entity),
        ] {
            let semantic: deckmaste_semantics::Predicate = deckmaste_semantics::ron::options()
                .from_str(spelling)
                .expect("the spelling parses as a semantic predicate");
            let lowered = super::predicate_region(|| semantic.lower());
            assert_eq!(
                lowered.candidate_domain(),
                expected,
                "lowering declared the wrong domain for `{spelling}`"
            );
            assert_eq!(
                deckmaste_core::Region::over(lowered.body.clone()).candidate_domain(),
                lowered.candidate_domain(),
                "`Region::over` disagrees with lowering for `{spelling}`"
            );
            assert_eq!(
                deckmaste_core::Region::candidate(lowered.body.clone()).candidate_domain(),
                lowered.candidate_domain(),
                "`Region::candidate` disagrees with lowering for `{spelling}`"
            );
        }
    }
}
