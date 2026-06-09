//! The continuous-effects layer system ([CR#613]): the one place an object's
//! characteristics are derived. Consumers read a [`LayeredView`], never the
//! printed face. v1 computes base values only; layers 4-7 and the dependency
//! tiebreaker ([CR#613.8]) are explicit seams for later tasks.

use std::collections::BTreeMap;

use deckmaste_core::{Ability, Color, Int, ManaSymbol, Subtype, Supertype, Type};

use crate::object::ObjectId;
use crate::state::GameState;

/// An object's derived characteristics ([CR#613]). `power`/`toughness` are
/// `None` for objects with no P/T; a printed `*` with no CDA resolves to `0`
/// ([CR#208.2a]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub power: Option<Int>,
    pub toughness: Option<Int>,
    pub colors: Vec<Color>,
    pub card_types: Vec<Type>,
    pub subtypes: Vec<Subtype>,
    pub supertypes: Vec<Supertype>,
    pub abilities: Vec<Ability>,
}

/// Every live object's derived characteristics, computed in one pass.
#[derive(Debug, Clone)]
pub struct LayeredView(BTreeMap<ObjectId, Characteristics>);

impl LayeredView {
    /// Returns the derived characteristics for `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not a live object when the view was computed.
    #[must_use]
    pub fn get(&self, id: ObjectId) -> &Characteristics {
        self.0.get(&id).expect("live ObjectId in LayeredView")
    }

    #[must_use]
    pub fn power(&self, id: ObjectId) -> Option<Int> { self.get(id).power }

    #[must_use]
    pub fn toughness(&self, id: ObjectId) -> Option<Int> { self.get(id).toughness }
}

/// Resolve a printed `StatValue` to a base number. `*` with no CDA is `0`
/// ([CR#208.2a]); CDAs (layer 7a) overwrite this later.
fn base_stat(v: Option<&deckmaste_core::StatValue>) -> Option<Int> {
    match v {
        Some(deckmaste_core::StatValue::Number(n)) => Some(*n),
        Some(_) => Some(0), // DefinedByAbility / Variable: 0 until a 7a CDA sets it
        None => None,
    }
}

/// Collect the colors contributed by one mana symbol ([CR#202.2]).
fn symbol_colors(sym: &ManaSymbol) -> impl Iterator<Item = Color> {
    let mut buf: [Option<Color>; 2] = [None; 2];
    match sym {
        ManaSymbol::Simple(s) => {
            buf[0] = s.color();
        }
        ManaSymbol::Hybrid(s, c) => {
            buf[0] = s.color();
            buf[1] = Some(*c);
        }
        ManaSymbol::Phyrexian(c, c2) => {
            buf[0] = Some(*c);
            buf[1] = *c2;
        }
        ManaSymbol::Variable | ManaSymbol::Snow => {}
    }
    buf.into_iter().flatten()
}

/// Base characteristics from the printed face ([CR#613.1]): the object's
/// characteristics before any continuous effect. v1 reads the printed face
/// (no copy/face-down handling).
fn base_values(state: &GameState, id: ObjectId) -> Characteristics {
    let face = crate::derive::face(state.def(id));
    // Colors come from the colored mana symbols in the cost ([CR#202.2]);
    // color_indicator is the fallback for objects with no mana cost.
    let mut colors: Vec<Color> = Vec::new();
    for c in face.mana_cost.iter().flat_map(symbol_colors) {
        if !colors.contains(&c) {
            colors.push(c);
        }
    }
    if colors.is_empty() {
        colors.clone_from(&face.color_indicator);
    }
    Characteristics {
        power: base_stat(face.power.as_ref()),
        toughness: base_stat(face.toughness.as_ref()),
        colors,
        card_types: face.types.clone(),
        subtypes: face.subtypes.clone(),
        supertypes: face.supertypes.clone(),
        abilities: crate::derive::abilities(state, id)
            .into_iter()
            .cloned()
            .collect(),
    }
}

impl GameState {
    /// Derive every object's characteristics ([CR#613.5]: fresh, continuously).
    /// Recomputed on each call; callers that need many lookups call once and
    /// index the returned view. (Caching is a noted later optimization.)
    ///
    /// Only card-backed objects have a characteristics entry; player proxies
    /// ([CR#109]) are skipped — they are not game objects with a characteristic
    /// set in the rules sense.
    #[must_use]
    pub fn layers(&self) -> LayeredView {
        let mut working = BTreeMap::new();
        for obj in self.objects.iter() {
            if obj.card_id().is_none() {
                continue; // player proxy — no characteristics
            }
            working.insert(obj.id, base_values(self, obj.id));
        }
        // Layers 4-7 apply here (later tasks). Base values only for now.
        LayeredView(working)
    }
}
