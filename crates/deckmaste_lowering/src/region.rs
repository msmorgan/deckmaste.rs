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
}

thread_local! {
    static CONTEXTS: RefCell<Vec<Context>> = const { RefCell::new(Vec::new()) };
    /// The card whose text is being compiled, so an R1/R2 refusal names it.
    /// The ADR makes ambiguity a compilation error with provenance (law 12),
    /// and provenance starts with which card failed to compile.
    static CARD: RefCell<Option<Arc<str>>> = const { RefCell::new(None) };
}

/// Compile `f` with `card` as the diagnostic context. Nests: text lowered
/// inside another card's text (a created token's granted ability) restores the
/// outer name when it finishes.
pub(crate) fn in_card<T>(card: &str, f: impl FnOnce() -> T) -> T {
    let previous = CARD.with(|slot| slot.replace(Some(Arc::from(card))));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    CARD.with(|slot| *slot.borrow_mut() = previous);
    match result {
        Ok(value) => value,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// The card being compiled, or a placeholder when lowering runs outside one (a
/// unit test, a bare macro expansion).
fn card_context() -> String {
    CARD.with(|slot| {
        slot.borrow()
            .as_ref()
            .map_or_else(|| "<unknown card>".to_owned(), ToString::to_string)
    })
}

/// Refuse to compile the card's text, naming the card and the reason.
///
/// The ADR routes R1/R2 refusals here (law 12: resolution happens once, in
/// lowering, "as per-card diagnostics"). [`crate::lower_card`] turns the
/// refusal into a [`crate::Diagnostic`] for callers compiling a corpus; a
/// refusal that escapes it is a card-authoring bug and stays LOUD.
pub(crate) fn refuse(reason: &str) -> ! {
    panic!("{}: {reason}", card_context())
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
            antecedent.inherited = true;
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

/// Whether `antecedent` can answer a mention of `cardinality`/`want`.
fn reachable(antecedent: &Antecedent, cardinality: Cardinality, want: Option<Sort>) -> bool {
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
}

/// R1/R2 over one discourse tier, nearest first. `None` when the tier holds no
/// compatible antecedent; a panic when it holds two that the exact-then-widened
/// carve-out does not separate.
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
                    inherited: false,
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
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
                (Kind::Objects, Provenance::AnnouncedTarget(0)),
                (Kind::Objects, Provenance::AnnouncedTarget(1)),
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
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
                (Kind::Object, Provenance::EventObject),
                (Kind::Object, Provenance::EventPatient),
                (Kind::Object, Provenance::EventActor),
                (Kind::Object, Provenance::DefendingPlayer),
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
                super::define(Kind::Objects),
                super::define(Kind::Number),
                super::define(Kind::Object),
            ]
        });
        assert_eq!(params.len(), 3, "source, controller, X");
        assert_eq!(defs, [DefId(3), DefId(4), DefId(5)]);
    }

    /// A nested region declares its own intrinsic parameters FIRST and then
    /// captures the enclosing register file in definition order (ADR law 7),
    /// so a body reads nothing it did not declare.
    #[test]
    fn a_nested_region_declares_intrinsics_then_captures_the_enclosing_file() {
        let (_, inner) = super::in_region(RegionKind::Spell, 1, || {
            let (params, ()) = super::in_child([(Kind::Object, Provenance::LoopElement)], || ());
            params
        });
        assert_dense(&inner);
        assert_eq!(
            abi(&inner),
            vec![
                (Kind::Object, Provenance::LoopElement),
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
                (Kind::Objects, Provenance::AnnouncedTarget(0)),
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
            let (_, inner) = super::in_child([(Kind::Object, Provenance::LoopElement)], || {
                let (params, ()) =
                    super::in_child([(Kind::Object, Provenance::LoopElement)], || ());
                params
            });
            inner
        });
        assert_eq!(
            abi(&inner),
            vec![
                (Kind::Object, Provenance::LoopElement),
                (Kind::Object, Provenance::Capture(RefId(0))),
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
                (Kind::Number, Provenance::AnnouncedX),
            ]
        );
    }

    /// A carried body (a delayed trigger, a floating replacement) keeps its OWN
    /// intrinsic ABI and takes the enclosing registers as explicit captures
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
                (Kind::Object, Provenance::Capture(RefId(0))),
                (Kind::Object, Provenance::Capture(RefId(1))),
                (Kind::Number, Provenance::Capture(RefId(2))),
            ],
            "then the enclosing spell region's three registers"
        );
    }

    /// R1: "it" reads the NEAREST antecedent — the most recently pushed one.
    #[test]
    fn it_resolves_to_the_nearest_antecedent() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            let first = super::define(Kind::Object);
            super::push_antecedent(
                first.into(),
                Kind::Object,
                Cardinality::One,
                None,
                Site::Frame,
            );
            let second = super::define(Kind::Object);
            super::push_antecedent(
                second.into(),
                Kind::Object,
                Cardinality::One,
                None,
                Site::Frame,
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
            let product = super::define(Kind::Object);
            super::push_antecedent(
                product.into(),
                Kind::Object,
                Cardinality::One,
                None,
                Site::Frame,
            );
            super::push_antecedent(RefId(0), Kind::Object, Cardinality::One, None, Site::Loop);
            super::it()
        });
        assert_eq!(reference, Some(RefId(0)));
    }

    /// R2: two equally compatible antecedents on the general search are a
    /// refusal, not a silent pick — the resolver never guesses between two
    /// equally good readings.
    #[test]
    #[should_panic(expected = "ambiguous discourse anaphor")]
    fn a_second_compatible_antecedent_refuses() {
        let _ = super::in_region(RegionKind::Spell, 0, || {
            for _ in 0..2 {
                let def = super::define(Kind::Object);
                super::push_antecedent(
                    def.into(),
                    Kind::Object,
                    Cardinality::One,
                    Some(deckmaste_semantics::Sort::Card),
                    Site::Product,
                );
            }
            super::that(deckmaste_semantics::Sort::Card)
        });
    }

    /// R1 is what a SITE-preferred search runs: among the antecedents at the
    /// preferred site the nearest wins outright, which is why a clause that
    /// moves two cards in turn still reads "that card" as the second. R2's
    /// refusal (above) governs the general search that follows.
    #[test]
    fn two_antecedents_at_a_preferred_site_resolve_to_the_nearest() {
        let (_, reference) = super::in_region(RegionKind::Spell, 0, || {
            for _ in 0..2 {
                let def = super::define(Kind::Object);
                super::push_antecedent(
                    def.into(),
                    Kind::Object,
                    Cardinality::One,
                    Some(deckmaste_semantics::Sort::Card),
                    Site::Frame,
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
            let permanent = super::define(Kind::Object);
            super::push_antecedent(
                permanent.into(),
                Kind::Object,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Permanent),
                Site::Frame,
            );
            let card = super::define(Kind::Object);
            super::push_antecedent(
                card.into(),
                Kind::Object,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                Site::Frame,
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
            let player = super::define(Kind::Object);
            super::push_antecedent(
                player.into(),
                Kind::Object,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Player),
                Site::Frame,
            );
            let card = super::define(Kind::Object);
            super::push_antecedent(
                card.into(),
                Kind::Object,
                Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                Site::Frame,
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
            let group = super::define(Kind::Objects);
            super::push_antecedent(
                group.into(),
                Kind::Objects,
                Cardinality::Many,
                Some(deckmaste_semantics::Sort::Card),
                Site::Frame,
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
                let inner = super::define(Kind::Object);
                super::push_antecedent(
                    inner.into(),
                    Kind::Object,
                    Cardinality::One,
                    None,
                    Site::Frame,
                );
            });
            let (params, ()) = super::in_child([], || ());
            params
        });
        assert_eq!(
            abi(&captured),
            vec![
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
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
                let paid = super::define(Kind::Objects);
                super::push_antecedent(
                    paid.into(),
                    Kind::Objects,
                    Cardinality::One,
                    Some(deckmaste_semantics::Sort::Permanent),
                    Site::Frame,
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
                (Kind::Object, Provenance::Candidate),
                (Kind::Object, Provenance::Source),
                (Kind::Object, Provenance::Controller),
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
}
