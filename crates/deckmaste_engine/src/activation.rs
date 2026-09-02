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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationId(pub(crate) u64);

impl ActivationId {
    /// Frames constructed directly by legacy engine tests have no region.
    /// Lowered ability execution always replaces this sentinel at entry.
    pub const NONE: Self = Self(u64::MAX);
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
}

impl crate::state::GameState {
    /// Enter `region`, populating its declared parameter prefix from the
    /// resolution inputs carried by `frame`.
    pub(crate) fn enter_region(&mut self, region: &Region, frame: &Frame) -> ActivationId {
        let values = region
            .params
            .iter()
            .map(|param| match &param.provenance {
                Provenance::Source => {
                    let lki = frame.this.clone();
                    let current = self.objects.get(frame.source).map(|_| frame.source);
                    Value::Object(ReferenceProduct { current, lki })
                }
                Provenance::Controller => Value::Object(ReferenceProduct {
                    current: Some(self.player(frame.controller).object),
                    lki: None,
                }),
                Provenance::EventObject => frame
                    .anaphora
                    .that_object
                    .clone()
                    .map(|snapshot| {
                        let current = self.objects.get(snapshot.object).map(|_| snapshot.object);
                        Value::Object(ReferenceProduct {
                            current,
                            lki: Some(snapshot),
                        })
                    })
                    .or_else(|| event_patient_value(self, frame.anaphora.that_patient.as_ref()))
                    .unwrap_or(Value::Unavailable),
                Provenance::EventPatient => {
                    event_patient_value(self, frame.anaphora.that_patient.as_ref())
                        .unwrap_or(Value::Unavailable)
                }
                Provenance::EventActor => {
                    frame
                        .anaphora
                        .that_player
                        .map_or(Value::Unavailable, |player| {
                            Value::Object(ReferenceProduct {
                                current: Some(self.player(player).object),
                                lki: None,
                            })
                        })
                }
                Provenance::DefendingPlayer => {
                    frame.defending_player.map_or(Value::Unavailable, |player| {
                        Value::Object(ReferenceProduct {
                            current: Some(self.player(player).object),
                            lki: None,
                        })
                    })
                }
                Provenance::AnnouncedTarget(index) => frame
                    .anaphora
                    .targets
                    .get(*index as usize)
                    .map_or(Value::Unavailable, |objects| {
                        Value::Objects(pack_objects(self, objects))
                    }),
                Provenance::AnnouncedX => {
                    frame.anaphora.x.map_or(Value::Unavailable, Value::Number)
                }
                Provenance::Capture(_)
                | Provenance::Linked(_)
                | Provenance::LoopElement
                | Provenance::Allotment
                | Provenance::Candidate => Value::Unavailable,
            })
            .collect();
        let id = ActivationId(self.next_activation);
        self.next_activation = self
            .next_activation
            .checked_add(1)
            .expect("activation id overflow");
        let root = self
            .activations
            .get(&frame.activation)
            .map_or(id, |parent| parent.root);
        self.activations.insert(
            id,
            Activation {
                root,
                params: region.params.clone(),
                values,
            },
        );
        id
    }

    pub(crate) fn activation_product(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Option<ReferenceProduct> {
        match self
            .activations
            .get(&activation)?
            .values
            .get(reference.0 as usize)?
        {
            Value::Object(product) => Some(product.clone()),
            Value::Objects(objects) => objects.first().cloned(),
            Value::Unavailable | Value::Number(_) => None,
        }
    }

    pub(crate) fn activation_objects(
        &self,
        activation: ActivationId,
        reference: RefId,
    ) -> Vec<ObjectId> {
        match self
            .activations
            .get(&activation)
            .and_then(|record| record.values.get(reference.0 as usize))
        {
            Some(Value::Objects(objects)) => objects
                .iter()
                .filter_map(|product| product.current)
                .collect(),
            Some(Value::Object(product)) => product.current.into_iter().collect(),
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
            .get(&activation)?
            .values
            .get(reference.0 as usize)?
        {
            Value::Number(number) => Some(*number),
            Value::Unavailable | Value::Object(_) | Value::Objects(_) => None,
        }
    }

    /// Fill the announced-target parameters after target selection commits.
    pub(crate) fn activation_set_targets(
        &mut self,
        activation: ActivationId,
        targets: &[Vec<ObjectId>],
    ) {
        let packed: Vec<Vec<ReferenceProduct>> = targets
            .iter()
            .map(|slot| pack_objects(self, slot))
            .collect();
        let Some(record) = self.activations.get_mut(&activation) else {
            return;
        };
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

    /// Freeze last-known information for every active register product that
    /// names an object immediately before that object leaves its zone.
    pub(crate) fn activation_departed(&mut self, object: ObjectId, snapshot: &LkiSnapshot) {
        for record in self.activations.values_mut() {
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
        let Some(record) = self.activations.get_mut(&activation) else {
            return;
        };
        for (param, value) in record.params.iter().zip(&mut record.values) {
            if matches!(param.provenance, Provenance::AnnouncedX) {
                *value = Value::Number(x);
            }
        }
    }

    pub(crate) fn clone_activation(&mut self, activation: ActivationId) -> ActivationId {
        let mut record = self
            .activations
            .get(&activation)
            .cloned()
            .unwrap_or(Activation {
                root: activation,
                params: Arc::from([]),
                values: Vec::new(),
            });
        let id = ActivationId(self.next_activation);
        self.next_activation = self
            .next_activation
            .checked_add(1)
            .expect("activation id overflow");
        // A copied stack object resolves independently of the original. Its
        // cloned register values therefore begin a fresh lifetime family.
        record.root = id;
        self.activations.insert(id, record);
        id
    }

    /// Reclaim one completed/countered resolution's root activation and every
    /// nested region entered beneath it.
    pub(crate) fn remove_activation_family(&mut self, activation: ActivationId) {
        let Some(root) = self.activations.get(&activation).map(|record| record.root) else {
            return;
        };
        self.activations.retain(|_, record| record.root != root);
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

        assert_eq!(state.activations.len(), 3);
        state.remove_activation_family(root);
        assert_eq!(state.activations.len(), 1);
        assert!(state.activation_product(cloned, RefId(0)).is_none());
        state.remove_activation_family(cloned);
        assert!(state.activations.is_empty());
    }
}
