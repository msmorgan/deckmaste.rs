use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_core::Param;
use deckmaste_core::Provenance;
use deckmaste_core::RefId;
use deckmaste_core::Region;
use deckmaste_core::Uint;

use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::stack::Frame;
use crate::trigger::EventPatient;

/// Stable identity of one entered core region during a resolution.
///
/// Bare evaluation scopes carry only their source/controller pair. Entering a
/// region materializes that scope in the activation table; resolving work
/// items therefore carry only the stored identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationId {
    None,
    Bare {
        source: ObjectId,
        controller: crate::player::PlayerId,
    },
    Stored(u64),
}

impl ActivationId {
    /// Frames constructed directly by legacy engine tests have no region.
    /// Lowered ability execution always replaces this sentinel at entry.
    pub const NONE: Self = Self::None;

    pub(crate) const fn bare(source: ObjectId, controller: crate::player::PlayerId) -> Self {
        Self::Bare { source, controller }
    }
}

/// One object-valued register product. `current` is the live identity used by
/// actions; `lki` is the snapshot used by information queries after departure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReferenceProduct {
    pub(crate) current: Option<ObjectId>,
    pub(crate) lki: Option<LkiSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Unavailable,
    Object(ReferenceProduct),
    Objects(Vec<ReferenceProduct>),
    Number(Uint),
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
    targets: Vec<Vec<ObjectId>>,
    x: Option<Uint>,
}

impl ActivationContext {
    fn bare(source: ObjectId, controller: crate::player::PlayerId) -> Self {
        Self {
            source,
            controller,
            source_lki: None,
            defending_player: None,
            event_object: None,
            event_patient: None,
            event_actor: None,
            targets: Vec::new(),
            x: None,
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

    /// Enter `region`, populating its declared parameter prefix from the
    /// resolution inputs carried by `frame`.
    pub(crate) fn enter_region(&self, region: &Region, frame: &Frame) -> ActivationId {
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
                Provenance::Capture(_)
                | Provenance::Linked(_)
                | Provenance::LoopElement
                | Provenance::Allotment
                | Provenance::Candidate => Value::Unavailable,
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

    fn activation_context(&self, activation: ActivationId) -> ActivationContext {
        match activation {
            ActivationId::Bare { source, controller } => {
                ActivationContext::bare(source, controller)
            }
            ActivationId::Stored(_) => self
                .activations
                .borrow()
                .get(&activation)
                .map(|record| record.context.clone())
                .expect("stored activation exists"),
            ActivationId::None => panic!("a region entry requires source/controller bindings"),
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

    pub(crate) fn activation_defending_player(
        &self,
        activation: ActivationId,
    ) -> Option<crate::player::PlayerId> {
        self.activation_context(activation).defending_player
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

    pub(crate) fn activation_context_product(
        &self,
        activation: ActivationId,
        provenance: &Provenance,
    ) -> Option<ReferenceProduct> {
        if activation == ActivationId::NONE {
            return None;
        }
        let context = self.activation_context(activation);
        let live = |object| self.objects.get(object).map(|_| object);
        match provenance {
            Provenance::Source => Some(ReferenceProduct {
                current: live(context.source),
                lki: context.source_lki,
            }),
            Provenance::Controller => Some(ReferenceProduct {
                current: Some(self.player(context.controller).object),
                lki: None,
            }),
            Provenance::EventObject => context.event_object.map(|snapshot| ReferenceProduct {
                current: live(snapshot.object),
                lki: Some(snapshot),
            }),
            Provenance::EventPatient => match context.event_patient {
                Some(EventPatient::Object(snapshot)) => Some(ReferenceProduct {
                    current: live(snapshot.object),
                    lki: Some(snapshot),
                }),
                Some(EventPatient::Player(player)) => Some(ReferenceProduct {
                    current: Some(self.player(player).object),
                    lki: None,
                }),
                None => None,
            },
            Provenance::EventActor => context.event_actor.map(|player| ReferenceProduct {
                current: Some(self.player(player).object),
                lki: None,
            }),
            Provenance::DefendingPlayer => {
                context.defending_player.map(|player| ReferenceProduct {
                    current: Some(self.player(player).object),
                    lki: None,
                })
            }
            Provenance::AnnouncedTarget(index) => context
                .targets
                .get(*index as usize)
                .and_then(|slot| slot.iter().copied().find(|object| live(*object).is_some()))
                .map(|object| ReferenceProduct {
                    current: Some(object),
                    lki: None,
                }),
            Provenance::AnnouncedX
            | Provenance::Capture(_)
            | Provenance::Linked(_)
            | Provenance::LoopElement
            | Provenance::Allotment
            | Provenance::Candidate => None,
        }
    }

    fn materialize_frame(&self, frame: &mut Frame) {
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
    pub(crate) fn fork_frame(&self, frame: &Frame) -> Frame {
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
    pub(crate) fn frame_set_source_lki(&self, frame: &mut Frame, lki: Option<LkiSnapshot>) {
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
        frame: &mut Frame,
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

    pub(crate) fn frame_set_event_bindings(
        &self,
        frame: &mut Frame,
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
        let mut product = match self
            .activations
            .borrow()
            .get(&activation)?
            .values
            .get(reference.0 as usize)?
        {
            Value::Object(product) => Some(product.clone()),
            Value::Objects(objects) => objects.first().cloned(),
            Value::Unavailable | Value::Number(_) => None,
        }?;
        if product
            .current
            .is_some_and(|object| self.objects.get(object).is_none())
        {
            product.current = None;
        }
        Some(product)
    }

    pub(crate) fn activation_objects(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Vec<ObjectId> {
        match self
            .activations
            .borrow()
            .get(&activation)
            .and_then(|record| record.values.get(reference.0 as usize))
        {
            Some(Value::Objects(objects)) => objects
                .iter()
                .filter_map(|product| product.current)
                .filter(|object| self.objects.get(*object).is_some())
                .collect(),
            Some(Value::Object(product)) => product
                .current
                .filter(|object| self.objects.get(*object).is_some())
                .into_iter()
                .collect(),
            Some(Value::Unavailable | Value::Number(_)) | None => Vec::new(),
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
            Value::Unavailable | Value::Object(_) | Value::Objects(_) => None,
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

    pub(crate) fn frame_set_targets(&self, frame: &mut Frame, targets: &[Vec<ObjectId>]) {
        self.materialize_frame(frame);
        self.activation_set_targets(frame.activation, targets);
    }

    pub(crate) fn frame_set_x(&self, frame: &mut Frame, x: Option<Uint>) {
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

    /// Freeze last-known information for every active register product that
    /// names an object immediately before that object leaves its zone.
    pub(crate) fn activation_departed(&mut self, object: ObjectId, snapshot: &LkiSnapshot) {
        for record in self.activations.borrow_mut().values_mut() {
            for value in &mut record.values {
                match value {
                    Value::Object(product) if product.current == Some(object) => {
                        product.current = None;
                        product.lki = Some(snapshot.clone());
                    }
                    Value::Objects(products) => {
                        for product in products {
                            if product.current == Some(object) {
                                product.current = None;
                                product.lki = Some(snapshot.clone());
                            }
                        }
                    }
                    Value::Unavailable | Value::Object(_) | Value::Number(_) => {}
                }
            }
        }
    }

    /// Fill the announced-X parameter after the value is chosen.
    pub(crate) fn activation_set_x(&mut self, activation: ActivationId, x: Uint) {
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
    pub(crate) fn remove_activation_family(&mut self, activation: ActivationId) {
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
    use crate::player::PlayerId;
    use crate::stack::Frame;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};

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
        let region = Region::new(Arc::from([]), Arc::from([]));
        let root = state.enter_region(&region, &Frame::bare(source, PlayerId(0)));
        let mut nested_frame = Frame::bare(source, PlayerId(0));
        nested_frame.activation = root;
        let nested = state.enter_region(&region, &nested_frame);
        let cloned = state.clone_activation(nested);

        assert_eq!(state.activations.borrow().len(), 3);
        state.remove_activation_family(root);
        assert_eq!(state.activations.borrow().len(), 1);
        assert!(state.activation_product(cloned, RefId(0)).is_none());
        state.remove_activation_family(cloned);
        assert!(state.activations.borrow().is_empty());
    }
}
