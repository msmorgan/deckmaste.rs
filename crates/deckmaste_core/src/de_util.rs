//! Small deserialization helpers shared by the manual-serde modules
//! (`effect`, `event`, …) — a tuple-variant pair visitor.

use std::fmt;
use std::marker::PhantomData;

use serde::Deserialize;
use serde::de::{self, SeqAccess, Visitor};

/// Visits a 2-element tuple variant — `tuple_variant(2, Pair::new())` — into
/// its two typed elements. Used wherever a 2-field tuple variant appears in a
/// hand-written `visit_enum` (`DealDamage(Selection, Quantity)`,
/// `BeginningOf(Phase, WhoseTurn)`, …).
pub struct Pair<A, B>(PhantomData<(A, B)>);

impl<A, B> Pair<A, B> {
    #[must_use]
    pub fn new() -> Self { Self(PhantomData) }
}

impl<A, B> Default for Pair<A, B> {
    fn default() -> Self { Self::new() }
}

impl<'de, A: Deserialize<'de>, B: Deserialize<'de>> Visitor<'de> for Pair<A, B> {
    type Value = (A, B);

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("a 2-element tuple") }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let a = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let b = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok((a, b))
    }
}
