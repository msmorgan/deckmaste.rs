use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_core::Param;
use deckmaste_core::Provenance;
use deckmaste_core::RefId;
use deckmaste_core::Region;
use deckmaste_core::Uint;

use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::stack::ExecutionFrame;
use crate::trigger::EventPatient;

/// Stable identity of one entered core region during a resolution.
///
/// Every execution scope is materialized in the activation table; resolving
/// work items therefore carry only the stored identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationId {
    None,
    Stored(u64),
}

impl ActivationId {
    /// Frames constructed directly by legacy engine tests have no region.
    /// Lowered ability execution always replaces this sentinel at entry.
    pub const NONE: Self = Self::None;
}

/// One Entity-valued register product. `current` is the live object identity
/// (including a player's stable proxy) used by actions; `lki` is the snapshot
/// used by information queries after an object departs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReferenceProduct {
    pub(crate) current: Option<ObjectId>,
    pub(crate) lki: Option<LkiSnapshot>,
}

/// One register's runtime value.
///
/// `Unavailable` is the only shape a read can find empty, and after
/// `deckmaste_core::validate` it can only mean a bound-but-departed value —
/// never "never bound" (ADR law 10). A DECLARED capture is never unavailable
/// for this reason: its value is snapshotted at creation and travels with the
/// created body (see [`GameState::capture_snapshot`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Value {
    Unavailable,
    Object(ReferenceProduct),
    Objects(Vec<ReferenceProduct>),
    /// One temporary pile. Its members are object products; the pile itself
    /// is not an object ([CR#700.3b]).
    Pile(Vec<ReferenceProduct>),
    Number(Uint),
    Symbol(String),
}

/// One executable ability reached by flattening a granted ability value,
/// together with the values its root region captured when the granting
/// continuous effect was created. A composite keyword contributes one entry
/// per executable member; a primitive keyword contributes none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapturedAbility {
    pub(crate) ability: deckmaste_core::Ability,
    pub(crate) captures: Vec<(RefId, Value)>,
}

/// Runtime payload aligned with one `Modification::GainAbility` in a floating
/// continuous effect. The core ability remains immutable grammar; this
/// engine-owned companion is its closure environment.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct AbilityRuntime {
    pub(crate) flattened: Vec<CapturedAbility>,
}

impl Value {
    /// The live object this value names, if it names exactly one. Used to
    /// inspect a frozen capture without an activation to read it through.
    #[cfg(test)]
    pub(crate) fn captured_object(&self) -> Option<ObjectId> {
        match self {
            Self::Object(product) => product.current,
            Self::Objects(products) => match products.as_slice() {
                [product] => product.current,
                _ => None,
            },
            Self::Unavailable | Self::Pile(_) | Self::Number(_) | Self::Symbol(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Activation {
    /// Root region entry for this resolution. Nested mode/body regions share
    /// the root so removing the stack entry reclaims the whole family.
    root: ActivationId,
    params: Arc<[Param]>,
    values: Vec<Value>,
    context: ActivationContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActivationContext {
    source: ObjectId,
    controller: crate::player::PlayerId,
    source_lki: Option<LkiSnapshot>,
    defending_player: Option<crate::player::PlayerId>,
    event_object: Option<LkiSnapshot>,
    event_patient: Option<EventPatient>,
    event_actor: Option<crate::player::PlayerId>,
    event_amount: Option<Uint>,
    targets: Vec<Vec<ObjectId>>,
    x: Option<Uint>,
    produced_mana: Vec<deckmaste_core::ColorOrColorless>,
    crossed: Option<(Uint, Uint)>,
    inherited_replacements: std::collections::HashSet<crate::replace_registry::ReplacementKey>,
    contained_in_batch: bool,
    /// [CR#601.2b,702.33d]: which tagged optional costs were announced paid for
    /// this activation, with multiplicity. Set at promote for a spell or an
    /// activated ability; seeded from the source object's inherited record
    /// ([CR#400.7d]) for an ability of a permanent whose spell has already left
    /// the stack. `Condition::PaidCost` and `Count::TimesPaid` read THIS, never
    /// a stack scan by source id.
    paid_costs: Vec<(deckmaste_core::CostTag, Uint)>,
}

impl ActivationContext {
    fn new(source: ObjectId, controller: crate::player::PlayerId) -> Self {
        Self {
            source,
            controller,
            source_lki: None,
            defending_player: None,
            event_object: None,
            event_patient: None,
            event_actor: None,
            event_amount: None,
            targets: Vec::new(),
            x: None,
            produced_mana: Vec::new(),
            crossed: None,
            inherited_replacements: std::collections::HashSet::new(),
            contained_in_batch: false,
            paid_costs: Vec::new(),
        }
    }
}

impl crate::state::GameState {
    fn mint_activation_id(&self) -> ActivationId {
        let next = self.next_activation.get();
        self.next_activation
            .set(next.checked_add(1).expect("activation id overflow"));
        ActivationId::Stored(next)
    }

    /// Enter the closed source/controller region used by rule and cost
    /// scopes, and return its lightweight execution cursor.
    pub fn frame(&self, source: ObjectId, controller: crate::player::PlayerId) -> ExecutionFrame {
        let mut context = ActivationContext::new(source, controller);
        // [CR#400.7d]: an ability of a permanent reads what was paid to cast
        // the spell that became it.
        context.paid_costs = self
            .paid_costs_by_object
            .get(&source)
            .cloned()
            .unwrap_or_default();
        let params = deckmaste_core::source_controller_params();
        let values = vec![
            Value::Object(ReferenceProduct {
                current: self.objects.get(source).map(|_| source),
                lki: None,
            }),
            Value::Object(ReferenceProduct {
                current: Some(self.player(controller).object),
                lki: None,
            }),
        ];
        let id = self.mint_activation_id();
        self.activations.borrow_mut().insert(
            id,
            Activation {
                root: id,
                params,
                values,
                context,
            },
        );
        ExecutionFrame {
            activation: id,
            payment: None,
        }
    }

    /// Enter `region`, populating its declared parameter prefix from the
    /// resolution inputs carried by `frame`.
    pub(crate) fn enter_region<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
    ) -> ActivationId {
        self.enter_region_with(region, frame, &[], &[])
    }

    /// Evaluate a read-only probe inside a materialized region activation,
    /// then reclaim both the activation and the identity it temporarily
    /// consumed. Castability checks need this when a target filter reads an
    /// earlier announced-target register: `ActivationId::NONE` has no
    /// register file to receive trial prefixes, while merely probing legal
    /// actions must not perturb the ids a later real resolution will mint.
    pub(crate) fn with_temporary_region_activation<T, R>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
        probe: impl FnOnce(ActivationId) -> R,
    ) -> R {
        let next_activation = self.next_activation.get();
        let activation = self.enter_region(region, frame);
        let result = probe(activation);
        // The caller's frame is a real, persistent parent now. Family removal
        // would reclaim that parent too, while rule frames minted inside the
        // probe can have independent roots. The probe is read-only, so reclaim
        // exactly every identity allocated after its saved cursor regardless
        // of family, then make the allocation observationally invisible.
        self.activations.borrow_mut().retain(|id, _| match id {
            ActivationId::Stored(raw) => *raw < next_activation,
            ActivationId::None => true,
        });
        self.next_activation.set(next_activation);
        result
    }

    /// Enter a region whose declared captures are supplied from outside the
    /// activation table — a delayed or reflexive body ([CR#603.7,603.12])
    /// firing long after the region that created it was reclaimed. `captures`
    /// is the snapshot [`Self::capture_snapshot`] took at creation.
    pub(crate) fn enter_created_region<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
        captures: &[(RefId, Value)],
    ) -> ActivationId {
        self.enter_region_with(region, frame, &[], captures)
    }

    /// The values of `region`'s declared captures, read out of the register
    /// file `frame` is running in and FROZEN ([CR#603.7a] — a delayed
    /// triggered ability is created during that resolution).
    ///
    /// The read goes through [`Self::activation_product`]/
    /// [`Self::activation_objects`], so a product the creating effect moved to
    /// a public zone is chased to its new incarnation ONCE, here
    /// ([CR#400.7j]). It is never chased again at the firing site:
    /// [CR#603.7c] settles that a captured object which left the zone it was
    /// expected to be in is not affected, and that one which left and returned
    /// "is a new object and thus won't be affected".
    ///
    /// The list is keyed by the created region's OWN parameter, so supplying
    /// it is total over `region.captures()` — a declared capture cannot go
    /// unsupplied at firing.
    pub(crate) fn capture_snapshot<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
    ) -> Vec<(RefId, Value)> {
        region
            .captures()
            .map(|(here, outer, kind)| (here, self.snapshot_register(frame, outer, kind)))
            .collect()
    }

    /// Close over every executable region inside a granted ability. The
    /// snapshot happens while the granting resolution's activation is still
    /// live; later layer derivation and activation/trigger entry use only this
    /// frozen payload.
    pub(crate) fn capture_ability_runtime(
        &self,
        ability: &deckmaste_core::Ability,
        frame: &ExecutionFrame,
    ) -> AbilityRuntime {
        let mut flattened = Vec::new();
        self.capture_ability_runtime_into(ability, frame, &mut flattened);
        AbilityRuntime { flattened }
    }

    fn capture_ability_runtime_into(
        &self,
        ability: &deckmaste_core::Ability,
        frame: &ExecutionFrame,
        out: &mut Vec<CapturedAbility>,
    ) {
        use deckmaste_core::Ability;

        let captures = match ability {
            Ability::Static(region) => self.capture_snapshot(region, frame),
            Ability::Activated(activated) => self.capture_snapshot(&activated.effect, frame),
            Ability::Triggered(triggered) => self.capture_snapshot(&triggered.effect, frame),
            Ability::Spell(spell) => self.capture_snapshot(&spell.effect, frame),
            Ability::Keyword(deckmaste_core::KeywordAbility::Composite { abilities, .. }) => {
                for member in abilities {
                    self.capture_ability_runtime_into(member, frame, out);
                }
                return;
            }
            Ability::Keyword(_) => return,
        };
        out.push(CapturedAbility {
            ability: ability.clone(),
            captures,
        });
    }

    /// Capture companions for a flattened continuous-effect change list.
    /// Non-grant changes carry an empty entry so the vectors remain aligned.
    pub(crate) fn capture_grant_runtimes(
        &self,
        changes: &[deckmaste_core::Modification],
        frame: &ExecutionFrame,
    ) -> Vec<AbilityRuntime> {
        changes
            .iter()
            .map(|change| match change {
                deckmaste_core::Modification::GainAbility(ability) => {
                    self.capture_ability_runtime(ability, frame)
                }
                _ => AbilityRuntime::default(),
            })
            .collect()
    }

    /// Read nested grant captures through a created static region without
    /// making a characteristics query consume a durable activation id.
    pub(crate) fn capture_grant_runtimes_in_created_region<T>(
        &self,
        changes: &[deckmaste_core::Modification],
        region: &Region<T>,
        frame: &ExecutionFrame,
        captures: &[(RefId, Value)],
    ) -> Vec<AbilityRuntime> {
        let next_activation = self.next_activation.get();
        let activation = self.enter_created_region(region, frame, captures);
        let created = ExecutionFrame {
            activation,
            payment: frame.payment,
        };
        let runtimes = self.capture_grant_runtimes(changes, &created);
        self.remove_activation_family(activation);
        debug_assert_eq!(
            self.next_activation.get(),
            next_activation + 1,
            "capture-only region entry mints exactly one activation"
        );
        self.next_activation.set(next_activation);
        runtimes
    }

    /// Freeze one register of `frame`'s activation for a value that will
    /// OUTLIVE this resolution — a declared capture or a linked memory cell.
    ///
    /// The read chases a product the resolving effect moved to a public zone
    /// ([CR#400.7j]), so what is frozen is the object as the effect left it,
    /// not the identity it had before the move. After freezing it is never
    /// chased again ([CR#603.7c]).
    pub(crate) fn snapshot_register(
        &self,
        frame: &ExecutionFrame,
        reference: RefId,
        kind: deckmaste_core::Kind,
    ) -> Value {
        match kind {
            deckmaste_core::Kind::Entities | deckmaste_core::Kind::Pile => {
                let objects = self.activation_objects(frame.activation, reference);
                if objects.is_empty() {
                    self.frozen_register(frame, reference)
                } else if kind == deckmaste_core::Kind::Pile {
                    Value::Pile(pack_objects(self, &objects))
                } else {
                    Value::Objects(pack_objects(self, &objects))
                }
            }
            deckmaste_core::Kind::Entity => self
                .activation_product(frame.activation, reference)
                .map_or_else(|| self.frozen_register(frame, reference), Value::Object),
            deckmaste_core::Kind::Number | deckmaste_core::Kind::Symbol => {
                self.frozen_register(frame, reference)
            }
        }
    }

    /// The raw stored value of one register, with no chase — the fallback when
    /// the chasing read finds nothing live, so a departed object still carries
    /// its last known information across the boundary ([CR#608.2h]).
    fn frozen_register(&self, frame: &ExecutionFrame, reference: RefId) -> Value {
        self.activations
            .borrow()
            .get(&frame.activation)
            .and_then(|record| record.values.get(reference.0 as usize))
            .cloned()
            .unwrap_or(Value::Unavailable)
    }

    fn enter_region_with<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
        supplied: &[(Provenance, Value)],
        captures: &[(RefId, Value)],
    ) -> ActivationId {
        let context = self.activation_context(frame.activation);
        let values = region
            .params
            .iter()
            .map(|param| match &param.provenance {
                Provenance::Source => {
                    let lki = context.source_lki.clone();
                    let current = self.objects.get(context.source).map(|_| context.source);
                    Value::Object(ReferenceProduct { current, lki })
                }
                Provenance::Controller => Value::Object(ReferenceProduct {
                    current: Some(self.player(context.controller).object),
                    lki: None,
                }),
                Provenance::EventObject => {
                    context
                        .event_object
                        .clone()
                        .map_or(Value::Unavailable, |snapshot| {
                            let current =
                                self.objects.get(snapshot.object).map(|_| snapshot.object);
                            Value::Object(ReferenceProduct {
                                current,
                                lki: Some(snapshot),
                            })
                        })
                }
                Provenance::EventPatient => {
                    event_patient_value(self, context.event_patient.as_ref())
                        .unwrap_or(Value::Unavailable)
                }
                Provenance::EventActor => {
                    context.event_actor.map_or(Value::Unavailable, |player| {
                        Value::Object(ReferenceProduct {
                            current: Some(self.player(player).object),
                            lki: None,
                        })
                    })
                }
                Provenance::EventAmount => context
                    .event_amount
                    .map_or(Value::Unavailable, Value::Number),
                Provenance::DefendingPlayer => {
                    context
                        .defending_player
                        .map_or(Value::Unavailable, |player| {
                            Value::Object(ReferenceProduct {
                                current: Some(self.player(player).object),
                                lki: None,
                            })
                        })
                }
                Provenance::AnnouncedTarget(index) => context
                    .targets
                    .get(*index as usize)
                    .map_or(Value::Unavailable, |objects| {
                        Value::Objects(pack_objects(self, objects))
                    }),
                Provenance::AnnouncedX => context.x.map_or(Value::Unavailable, Value::Number),
                // A capture supplied explicitly is one that crossed a region
                // BOUNDARY: the created body's snapshot, frozen at creation
                // ([CR#603.7a]). Otherwise this is ordinary nesting inside one
                // resolution and the enclosing register file is still live.
                Provenance::Capture(reference) => captures
                    .iter()
                    .find(|(here, _)| *here == RefId(param.def.0))
                    .map(|(_, value)| value.clone())
                    .or_else(|| {
                        self.activations
                            .borrow()
                            .get(&frame.activation)
                            .and_then(|parent| parent.values.get(reference.0 as usize))
                            .cloned()
                    })
                    .unwrap_or(Value::Unavailable),
                // [CR#607.1]: a linked memory cell is read out of the card's
                // own memory, keyed by the reading ability's source — the
                // object both linked abilities are printed on.
                Provenance::Linked(cell) => self.memory_cell(context.source, cell),
                provenance @ (Provenance::LoopElement
                | Provenance::Allotment
                | Provenance::Candidate(_)) => supplied
                    .iter()
                    .find(|(candidate, _)| {
                        std::mem::discriminant(candidate) == std::mem::discriminant(provenance)
                    })
                    .map_or(Value::Unavailable, |(_, value)| value.clone()),
            })
            .collect();
        let id = self.mint_activation_id();
        let root = self
            .activations
            .borrow()
            .get(&frame.activation)
            .map_or(id, |parent| parent.root);
        self.activations.borrow_mut().insert(
            id,
            Activation {
                root,
                params: region.params.clone(),
                values,
                context,
            },
        );
        id
    }

    /// [CR#607.1]: read one linked memory cell of `owner`. An unwritten cell
    /// is `Unavailable` — the reading half of a linked pair whose writer never
    /// ran does nothing ([CR#607.5a]). For a LOWERED card that state is
    /// unreachable: lowering refuses a linked read whose cell no ability on
    /// the card writes.
    pub(crate) fn memory_cell(&self, owner: ObjectId, cell: &deckmaste_core::Ident) -> Value {
        self.memory
            .get(&(owner, *cell))
            .cloned()
            .unwrap_or(Value::Unavailable)
    }

    /// [CR#607.1]: write one linked memory cell of `owner` — the runtime half
    /// of `Instruction::Remember`.
    pub(crate) fn remember_cell(
        &mut self,
        owner: ObjectId,
        cell: deckmaste_core::Ident,
        value: Value,
    ) {
        self.memory.insert((owner, cell), value);
    }

    fn activation_context(&self, activation: ActivationId) -> ActivationContext {
        match activation {
            ActivationId::Stored(_) => self
                .activations
                .borrow()
                .get(&activation)
                .map(|record| record.context.clone())
                .expect("stored activation exists"),
            ActivationId::None => panic!("a region entry requires a parent activation"),
        }
    }

    pub(crate) fn activation_source(&self, activation: ActivationId) -> ObjectId {
        self.activation_context(activation).source
    }

    pub(crate) fn activation_controller(
        &self,
        activation: ActivationId,
    ) -> crate::player::PlayerId {
        self.activation_context(activation).controller
    }

    pub(crate) fn activation_source_lki(&self, activation: ActivationId) -> Option<LkiSnapshot> {
        self.activation_context(activation).source_lki
    }

    /// [CR#702.33d]: how many times the tagged optional cost was announced
    /// paid for this activation — 0 when it was not.
    pub(crate) fn activation_times_paid(
        &self,
        activation: ActivationId,
        tag: &deckmaste_core::CostTag,
    ) -> Uint {
        if activation == ActivationId::NONE {
            return 0;
        }
        self.activation_context(activation)
            .paid_costs
            .iter()
            .find(|(candidate, _)| candidate == tag)
            .map_or(0, |(_, times)| *times)
    }

    /// [CR#601.2b]: record the announced optional-cost payments on the register
    /// file announcement and resolution share.
    pub(crate) fn activation_set_paid_costs(
        &self,
        activation: ActivationId,
        paid: &[(deckmaste_core::CostTag, Uint)],
    ) {
        let mut activations = self.activations.borrow_mut();
        if let Some(record) = activations.get_mut(&activation) {
            record.context.paid_costs = paid.to_vec();
        }
    }

    pub(crate) fn activation_x(&self, activation: ActivationId) -> Option<Uint> {
        self.activation_context(activation).x
    }

    pub(crate) fn activation_target_objects(
        &self,
        activation: ActivationId,
        index: usize,
    ) -> Vec<ObjectId> {
        self.activation_context(activation)
            .targets
            .get(index)
            .cloned()
            .unwrap_or_default()
    }

    fn materialize_frame(&self, frame: &mut ExecutionFrame) {
        if matches!(frame.activation, ActivationId::Stored(_)) {
            return;
        }
        let context = self.activation_context(frame.activation);
        let id = self.mint_activation_id();
        self.activations.borrow_mut().insert(
            id,
            Activation {
                root: id,
                params: Arc::from([]),
                values: Vec::new(),
                context,
            },
        );
        frame.activation = id;
    }

    /// Clone one activation for a child continuation while keeping it in the
    /// same resolution family. Subsequent parameter writes are isolated from
    /// sibling work items.
    pub(crate) fn fork_frame(&self, frame: &ExecutionFrame) -> ExecutionFrame {
        let context = self.activation_context(frame.activation);
        let (root, params, values) = self
            .activations
            .borrow()
            .get(&frame.activation)
            .map_or((frame.activation, Arc::from([]), Vec::new()), |record| {
                (record.root, record.params.clone(), record.values.clone())
            });
        let id = self.mint_activation_id();
        self.activations.borrow_mut().insert(
            id,
            Activation {
                root,
                params,
                values,
                context,
            },
        );
        let mut child = frame.clone();
        child.activation = id;
        child
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the setter takes ownership of the optional snapshot stored in the activation"
    )]
    pub(crate) fn frame_set_source_lki(
        &self,
        frame: &mut ExecutionFrame,
        lki: Option<LkiSnapshot>,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        record.context.source_lki.clone_from(&lki);
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::Source) {
                *value = Value::Object(ReferenceProduct {
                    current: self
                        .objects
                        .get(record.context.source)
                        .map(|_| record.context.source),
                    lki: lki.clone(),
                });
            }
        }
    }

    pub(crate) fn frame_set_defending_player(
        &self,
        frame: &mut ExecutionFrame,
        player: Option<crate::player::PlayerId>,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        record.context.defending_player = player;
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::DefendingPlayer) {
                *value = player.map_or(Value::Unavailable, |player| {
                    Value::Object(ReferenceProduct {
                        current: Some(self.player(player).object),
                        lki: None,
                    })
                });
            }
        }
    }

    /// Rebind the controller parameter for a cost-local frame. In a
    /// resolution-time optional payment, semantic `You` inside the cost names
    /// the payer, while the consequence resumes with the containing effect's
    /// original controller.
    pub(crate) fn frame_set_controller(
        &self,
        frame: &mut ExecutionFrame,
        player: crate::player::PlayerId,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        record.context.controller = player;
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::Controller) {
                *value = Value::Object(ReferenceProduct {
                    current: Some(self.player(player).object),
                    lki: None,
                });
            }
        }
    }

    pub(crate) fn frame_set_event_bindings(
        &self,
        frame: &mut ExecutionFrame,
        object: Option<LkiSnapshot>,
        actor: Option<crate::player::PlayerId>,
        patient: Option<EventPatient>,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        let context = &mut record.context;
        context.event_object = object;
        context.event_actor = actor;
        context.event_patient = patient;
        let event_object = context.event_object.clone();
        let event_actor = context.event_actor;
        let event_patient = context.event_patient.clone();
        for (param, value) in record.params.iter().zip(&mut record.values) {
            *value = match param.provenance {
                Provenance::EventObject => {
                    event_object.clone().map_or(Value::Unavailable, |snapshot| {
                        Value::Object(ReferenceProduct {
                            current: self.objects.get(snapshot.object).map(|_| snapshot.object),
                            lki: Some(snapshot),
                        })
                    })
                }
                Provenance::EventPatient => {
                    event_patient_value(self, event_patient.as_ref()).unwrap_or(Value::Unavailable)
                }
                Provenance::EventActor => event_actor.map_or(Value::Unavailable, |player| {
                    Value::Object(ReferenceProduct {
                        current: Some(self.player(player).object),
                        lki: None,
                    })
                }),
                _ => continue,
            };
        }
    }

    pub(crate) fn activation_product(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Option<ReferenceProduct> {
        let activations = self.activations.borrow();
        let record = activations.get(&activation)?;
        let is_instruction_product = record.params.get(reference.0 as usize).is_none();
        let mut product = match record.values.get(reference.0 as usize)? {
            Value::Object(product) => Some(product.clone()),
            Value::Objects(objects) => objects.first().cloned(),
            Value::Unavailable | Value::Pile(_) | Value::Number(_) | Value::Symbol(_) => None,
        }?;
        if product
            .current
            .is_some_and(|object| self.objects.get(object).is_none())
        {
            product.current = if is_instruction_product {
                product
                    .current
                    .map(|object| self.chase_moved(object))
                    .filter(|object| self.objects.get(*object).is_some())
            } else {
                None
            };
        }
        Some(product)
    }

    pub(crate) fn activation_objects(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Vec<ObjectId> {
        let activations = self.activations.borrow();
        let Some(record) = activations.get(&activation) else {
            return Vec::new();
        };
        let is_instruction_product = record.params.get(reference.0 as usize).is_none();
        let live = |object| {
            if self.objects.get(object).is_some() {
                Some(object)
            } else if is_instruction_product {
                let moved = self.chase_moved(object);
                self.objects.get(moved).map(|_| moved)
            } else {
                None
            }
        };
        match record.values.get(reference.0 as usize) {
            Some(Value::Objects(objects) | Value::Pile(objects)) => objects
                .iter()
                .filter_map(|product| product.current.and_then(live))
                .collect(),
            Some(Value::Object(product)) => product.current.and_then(live).into_iter().collect(),
            Some(Value::Unavailable | Value::Number(_) | Value::Symbol(_)) | None => Vec::new(),
        }
    }

    pub(crate) fn activation_number(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Option<Uint> {
        match self
            .activations
            .borrow()
            .get(&activation)?
            .values
            .get(reference.0 as usize)?
        {
            Value::Number(number) => Some(*number),
            Value::Unavailable
            | Value::Object(_)
            | Value::Objects(_)
            | Value::Pile(_)
            | Value::Symbol(_) => None,
        }
    }

    pub(crate) fn activation_symbol(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Option<String> {
        match self
            .activations
            .borrow()
            .get(&activation)?
            .values
            .get(reference.0 as usize)?
        {
            Value::Symbol(symbol) => Some(symbol.clone()),
            Value::Unavailable
            | Value::Object(_)
            | Value::Objects(_)
            | Value::Pile(_)
            | Value::Number(_) => None,
        }
    }

    pub(crate) fn activation_provenance(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Option<Provenance> {
        self.activations
            .borrow()
            .get(&activation)?
            .params
            .get(reference.0 as usize)
            .map(|param| param.provenance.clone())
    }

    pub(crate) fn activation_parameter(
        &self,
        activation: ActivationId,
        provenance: &Provenance,
    ) -> Option<RefId> {
        self.activations
            .borrow()
            .get(&activation)?
            .params
            .iter()
            .find(|param| &param.provenance == provenance)
            .map(|param| param.def.into())
    }

    pub(crate) fn activation_reference_is(
        &self,
        activation: ActivationId,
        reference: RefId,
        provenance: &Provenance,
    ) -> bool {
        self.activation_provenance(activation, reference).as_ref() == Some(provenance)
            || (activation == ActivationId::NONE
                && match provenance {
                    Provenance::Source => {
                        deckmaste_core::Reference::Reg(reference)
                            == deckmaste_core::Reference::source_parameter()
                    }
                    Provenance::Controller => {
                        deckmaste_core::Reference::Reg(reference)
                            == deckmaste_core::Reference::controller_parameter()
                    }
                    _ => false,
                })
    }

    /// The announced-target slots the register file currently holds
    /// ([CR#601.2c]) — empty before the announcement writes any, and empty for
    /// an `ActivationId::NONE` probe.
    #[must_use]
    pub(crate) fn announced_targets(&self, activation: ActivationId) -> Vec<Vec<ObjectId>> {
        self.activations
            .borrow()
            .get(&activation)
            .map(|record| record.context.targets.clone())
            .unwrap_or_default()
    }

    /// Fill the announced-target parameters after target selection commits.
    pub(crate) fn activation_set_targets(
        &self,
        activation: ActivationId,
        targets: &[Vec<ObjectId>],
    ) {
        let packed: Vec<Vec<ReferenceProduct>> = targets
            .iter()
            .map(|slot| pack_objects(self, slot))
            .collect();
        let mut activations = self.activations.borrow_mut();
        let Some(record) = activations.get_mut(&activation) else {
            return;
        };
        record.context.targets = targets.to_vec();
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if let Provenance::AnnouncedTarget(index) = param.provenance {
                *value = targets
                    .get(index as usize)
                    .and_then(|_| packed.get(index as usize))
                    .cloned()
                    .map_or(Value::Unavailable, Value::Objects);
            }
        }
    }

    pub(crate) fn frame_set_targets(&self, frame: &mut ExecutionFrame, targets: &[Vec<ObjectId>]) {
        self.materialize_frame(frame);
        self.activation_set_targets(frame.activation, targets);
    }

    pub(crate) fn frame_set_x(&self, frame: &mut ExecutionFrame, x: Option<Uint>) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        record.context.x = x;
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::AnnouncedX) {
                *value = x.map_or(Value::Unavailable, Value::Number);
            }
        }
    }

    pub(crate) fn enter_candidate_region<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
        candidate: ObjectId,
    ) -> ActivationId {
        let value = Value::Object(
            pack_objects(self, &[candidate])
                .into_iter()
                .next()
                .expect("one candidate"),
        );
        self.enter_region_with(
            region,
            frame,
            &[(Provenance::Candidate(region.candidate_domain()), value)],
            &[],
        )
    }

    /// A GONE candidate's region entry: the candidate binds as the snapshot
    /// the caller already holds, not as its stale id.
    ///
    /// [CR#608.2i]: a look-back read does not require its subjects to still be
    /// in the zone they were in. The candidate of a leaves-the-battlefield
    /// trigger's per-candidate predicate is exactly that subject, so the
    /// register it fills must keep an identity a history participant filter
    /// can compare against — and after the object left the store, a stale id
    /// packs to a product with neither `current` nor `lki`, which names
    /// nothing. Mirrors `event_patient_value`'s snapshot-backed product; a
    /// candidate that IS still live keeps its live identity too, so an
    /// action-side read is unaffected.
    pub(crate) fn enter_candidate_region_snapshot<T>(
        &self,
        region: &Region<T>,
        frame: &ExecutionFrame,
        candidate: &LkiSnapshot,
    ) -> ActivationId {
        let value = Value::Object(ReferenceProduct {
            current: self.objects.get(candidate.object).map(|_| candidate.object),
            lki: Some(candidate.clone()),
        });
        self.enter_region_with(
            region,
            frame,
            &[(Provenance::Candidate(region.candidate_domain()), value)],
            &[],
        )
    }

    pub(crate) fn enter_loop_region(
        &self,
        region: &Region,
        frame: &ExecutionFrame,
        element: ObjectId,
        allotment: Option<Uint>,
    ) -> ActivationId {
        let object = Value::Object(
            pack_objects(self, &[element])
                .into_iter()
                .next()
                .expect("one element"),
        );
        let mut supplied = vec![(Provenance::LoopElement, object)];
        if let Some(amount) = allotment {
            supplied.push((Provenance::Allotment, Value::Number(amount)));
        }
        self.enter_region_with(region, frame, &supplied, &[])
    }

    fn activation_write(&self, activation: ActivationId, def: deckmaste_core::DefId, value: Value) {
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&activation)
            .expect("stored activation exists");
        let index = def.0 as usize;
        if record.values.len() <= index {
            record.values.resize(index + 1, Value::Unavailable);
        }
        record.values[index] = value;
    }

    pub(crate) fn activation_write_object(
        &self,
        activation: ActivationId,
        def: deckmaste_core::DefId,
        object: ObjectId,
    ) {
        let product = pack_objects(self, &[object])
            .into_iter()
            .next()
            .expect("one object");
        self.activation_write(activation, def, Value::Object(product));
    }

    pub(crate) fn activation_write_objects(
        &self,
        activation: ActivationId,
        def: deckmaste_core::DefId,
        objects: &[ObjectId],
    ) {
        self.activation_write(activation, def, Value::Objects(pack_objects(self, objects)));
    }

    pub(crate) fn activation_write_number(
        &self,
        activation: ActivationId,
        def: deckmaste_core::DefId,
        number: Uint,
    ) {
        self.activation_write(activation, def, Value::Number(number));
    }

    pub(crate) fn activation_write_symbol(
        &self,
        activation: ActivationId,
        def: deckmaste_core::DefId,
        symbol: String,
    ) {
        self.activation_write(activation, def, Value::Symbol(symbol));
    }

    pub(crate) fn activation_crossed(&self, activation: ActivationId) -> Option<(Uint, Uint)> {
        self.activation_context(activation).crossed
    }

    pub(crate) fn activation_produced_mana(
        &self,
        activation: ActivationId,
    ) -> Vec<deckmaste_core::ColorOrColorless> {
        self.activation_context(activation).produced_mana
    }

    pub(crate) fn activation_inherited_replacements(
        &self,
        activation: ActivationId,
    ) -> std::collections::HashSet<crate::replace_registry::ReplacementKey> {
        self.activation_context(activation).inherited_replacements
    }

    pub(crate) fn activation_contained_in_batch(&self, activation: ActivationId) -> bool {
        self.activation_context(activation).contained_in_batch
    }

    pub(crate) fn frame_set_event_extras(
        &self,
        frame: &mut ExecutionFrame,
        amount: Option<Uint>,
        produced_mana: Vec<deckmaste_core::ColorOrColorless>,
        crossed: Option<(Uint, Uint)>,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let record = activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists");
        record.context.event_amount = amount;
        record.context.produced_mana = produced_mana;
        record.context.crossed = crossed;
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::EventAmount) {
                *value = amount.map_or(Value::Unavailable, Value::Number);
            }
        }
    }

    pub(crate) fn frame_set_action_context(
        &self,
        frame: &mut ExecutionFrame,
        inherited: std::collections::HashSet<crate::replace_registry::ReplacementKey>,
        contained: bool,
    ) {
        self.materialize_frame(frame);
        let mut activations = self.activations.borrow_mut();
        let context = &mut activations
            .get_mut(&frame.activation)
            .expect("materialized frame exists")
            .context;
        context.inherited_replacements = inherited;
        context.contained_in_batch = contained;
    }

    /// Freeze last-known information for every register product that names an
    /// object immediately before that object leaves its zone ([CR#608.2h] —
    /// "if it's no longer in that zone … the effect uses the object's last
    /// known information").
    ///
    /// This reaches past the live activation table into the values that
    /// OUTLIVE a resolution: a delayed trigger's declared captures and the
    /// cards' linked memory cells. Both are frozen ids, never re-chased, so
    /// without this an object that departs after the snapshot was taken would
    /// carry last-known information from the wrong moment.
    pub(crate) fn activation_departed(&mut self, object: ObjectId, snapshot: &LkiSnapshot) {
        for record in self.activations.borrow_mut().values_mut() {
            for value in &mut record.values {
                freeze_value(value, object, snapshot);
            }
        }
        for trigger in &mut self.delayed_triggers {
            for (_, value) in &mut trigger.bindings.captures {
                freeze_value(value, object, snapshot);
            }
        }
        for value in self.memory.values_mut() {
            freeze_value(value, object, snapshot);
        }
    }

    /// Fill the announced-X parameter: with the value chosen at announcement
    /// ([CR#601.2b]), or with a triggered ability's `where_x` at region entry
    /// ([CR#702.21b]).
    pub(crate) fn activation_set_x(&self, activation: ActivationId, x: Uint) {
        let mut activations = self.activations.borrow_mut();
        let Some(record) = activations.get_mut(&activation) else {
            return;
        };
        record.context.x = Some(x);
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::AnnouncedX) {
                *value = Value::Number(x);
            }
        }
    }

    pub(crate) fn clone_activation(&mut self, activation: ActivationId) -> ActivationId {
        let mut record = self
            .activations
            .borrow()
            .get(&activation)
            .cloned()
            .unwrap_or_else(|| Activation {
                root: activation,
                params: Arc::from([]),
                values: Vec::new(),
                context: self.activation_context(activation),
            });
        let id = self.mint_activation_id();
        // A copied stack object resolves independently of the original. Its
        // cloned register values therefore begin a fresh lifetime family.
        record.root = id;
        self.activations.borrow_mut().insert(id, record);
        id
    }

    /// Reclaim one completed/countered resolution's root activation and every
    /// nested region entered beneath it.
    pub(crate) fn remove_activation_family(&self, activation: ActivationId) {
        let Some(root) = self
            .activations
            .borrow()
            .get(&activation)
            .map(|record| record.root)
        else {
            return;
        };
        self.activations
            .borrow_mut()
            .retain(|_, record| record.root != root);
    }
}

fn freeze_value(value: &mut Value, object: ObjectId, snapshot: &LkiSnapshot) {
    match value {
        Value::Object(product) if product.current == Some(object) => {
            product.lki = Some(snapshot.clone());
        }
        Value::Objects(products) | Value::Pile(products) => {
            for product in products {
                if product.current == Some(object) {
                    product.lki = Some(snapshot.clone());
                }
            }
        }
        Value::Unavailable | Value::Object(_) | Value::Number(_) | Value::Symbol(_) => {}
    }
}

fn pack_objects(state: &crate::state::GameState, objects: &[ObjectId]) -> Vec<ReferenceProduct> {
    objects
        .iter()
        .copied()
        .map(|object| ReferenceProduct {
            current: state.objects.get(object).map(|_| object),
            lki: state
                .objects
                .get(object)
                .filter(|object| object.zone.is_some())
                .map(|_| LkiSnapshot::capture(state, object)),
        })
        .collect()
}

fn event_patient_value(
    state: &crate::state::GameState,
    patient: Option<&EventPatient>,
) -> Option<Value> {
    patient.map(|patient| match patient {
        EventPatient::Object(snapshot) => Value::Object(ReferenceProduct {
            current: state.objects.get(snapshot.object).map(|_| snapshot.object),
            lki: Some(snapshot.clone()),
        }),
        EventPatient::Player(player) => Value::Object(ReferenceProduct {
            current: Some(state.player(*player).object),
            lki: None,
        }),
    })
}

pub(crate) type ActivationTable = HashMap<ActivationId, Activation>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::ExecutionFrame;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn bare_game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: HashMap::new(),
            subtypes: HashMap::new(),
            types: HashMap::new(),
        })
    }

    #[test]
    fn completed_resolution_reclaims_nested_family_but_not_stack_copy() {
        let mut state = bare_game();
        let source = state.player(PlayerId(0)).object;
        let region = Region::closed(());
        let seed = state.frame(source, PlayerId(0));
        let root = state.enter_region(&region, &seed);
        let nested_frame = ExecutionFrame {
            activation: root,
            payment: None,
        };
        let nested = state.enter_region(&region, &nested_frame);
        let cloned = state.clone_activation(nested);

        assert_eq!(state.activations.borrow().len(), 4);
        state.remove_activation_family(root);
        assert_eq!(state.activations.borrow().len(), 1);
        assert!(state.activation_product(cloned, RefId(0)).is_none());
        state.remove_activation_family(cloned);
        assert!(state.activations.borrow().is_empty());
    }

    /// [CR#608.2i]: a look-back read's subjects need not still be in the zone
    /// they were in. So the candidate register of a departed candidate's
    /// predicate region has to keep an identity a history participant filter
    /// can compare against — and the stale id alone does not: it packs to a
    /// product with neither a live object nor a snapshot, which names nothing.
    #[test]
    fn a_departed_candidate_binds_its_snapshot_where_its_stale_id_names_nothing() {
        let mut state = bare_game();
        let source = state.player(PlayerId(0)).object;
        let candidate = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(deckmaste_core::Zone::Battlefield),
        );
        let snapshot = LkiSnapshot::capture(&state, candidate);
        state.objects.remove(candidate);

        let region = Region::unary(
            deckmaste_core::Kind::Entity,
            Provenance::Candidate(deckmaste_core::Domain::Entity),
            (),
        );
        let frame = state.frame(source, PlayerId(0));

        let by_id = state.enter_candidate_region(&region, &frame, candidate);
        assert_eq!(
            state.activation_product(by_id, RefId(0)),
            Some(ReferenceProduct {
                current: None,
                lki: None,
            }),
            "the stale id packs to a product that names nothing"
        );

        let by_snapshot = state.enter_candidate_region_snapshot(&region, &frame, &snapshot);
        let product = state
            .activation_product(by_snapshot, RefId(0))
            .expect("the candidate register is bound");
        assert_eq!(product.current, None, "the candidate is gone");
        assert_eq!(
            product.lki.map(|lki| lki.object),
            Some(candidate),
            "…and still names itself through its last-known information"
        );
    }

    /// A temporary pile is a group of its member objects, never an object in
    /// its own right ([CR#700.3b]).
    #[test]
    fn a_pile_value_is_an_object_group_not_an_object() {
        let state = bare_game();
        let source = state.player(PlayerId(0)).object;
        let region = Region::closed(());
        let activation = state.enter_region(&region, &state.frame(source, PlayerId(0)));
        state.activation_write(
            activation,
            deckmaste_core::DefId(0),
            Value::Pile(pack_objects(&state, &[source])),
        );

        assert_eq!(state.activation_product(activation, RefId(0)), None);
        assert_eq!(state.activation_objects(activation, RefId(0)), vec![source]);
    }
}
