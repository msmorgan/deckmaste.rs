//! The load-time elaborator: one walk over each loaded card's grammar tree
//! with a running binding context, refusing every shape the Rust type system
//! can't — unbound anaphors, event-role reads the event doesn't supply,
//! misplaced `Targeted`, kind mismatches, carrier-scope violations,
//! well-formedness floors, cost-ineligible payments. Serialization erases
//! structural guarantees, so anything a RON file can spell but the rules
//! forbid must be refused HERE, at load — never at resolve time (the
//! engine's `.expect()` sites on unbound `That`/`It`/`Target`/event roles
//! are exactly what these errors preempt).
//!
//! Table-driven: rule values come from `crates/deckmaste_cards/tables/*.ron`,
//! generated from the Idris reference model by `idris/src/EmitTables.idr`
//! (see [`tables`]). Every rule has a stable `E-*` code (the emitted
//! checker-rule manifest) and a twin reject fixture under
//! `crates/deckmaste_cards/tests/reject/`.
//!
//! In this ticket the elaborator runs behind `cargo xtask validate`
//! (`validate::validate_plugin`); wiring into `Plugin::load` with the
//! deny/warn rollout is the `cards-elab-load-gate` ticket.

pub mod tables;
mod walk;

use std::collections::HashMap;
use std::fmt;

use deckmaste_core::Card;
use deckmaste_core::Counter;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
use deckmaste_core::Token;

/// A stable elaboration error code. The full code space is laid out by the
/// emitted checker-rule manifest (`tables/checker-rules.ron`); the
/// `E-COPY-EXCEPT` and `E-MACRO-*` families are reserved there and activate
/// in the tickets that mint their grammar nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
    /// A target slot referenced but not announced in scope
    /// ([CR#115.3,601.2c]) — including targets dropped by a `Delayed` body
    /// ([CR#603.7c]).
    BindTarget,
    /// Singular `That` read outside a one-binder `With` ([CR#608.2d]).
    BindThat,
    /// Group `That` read outside a many-binder `With` ([CR#608.2d]).
    BindThatGroup,
    /// `It` read outside an `Each`/`DivideAmong`/`Where`/`Pick` binder
    /// ([CR#608.2]).
    BindIt,
    /// `Allotment` read outside a `DivideAmong` body ([CR#601.2d]).
    BindAllotment,
    /// An event-role reference outside any event body ([CR#603.2e,608.2k]).
    BindEvent,
    /// A noted key read but never noted in scope ([CR#607.2]).
    BindNote,
    /// `EventObject` read where the event caps supply no object
    /// ([CR#603.2e,608.2k]).
    CapsObject,
    /// `EventActor` read where the event caps supply no actor
    /// ([CR#603.2e,608.2k]).
    CapsActor,
    /// `EventPatient` read where the event fixes no patient kind
    /// ([CR#120.3,608.2k]).
    CapsPatient,
    /// `DefendingPlayer` read where no combat onset supplies one
    /// ([CR#506.2,508.5]).
    CapsDefender,
    /// An amount anaphor (`ThatMuch`) or amount aggregation (`EventSum`)
    /// without an amount-guaranteeing antecedent ([CR#107.3,608.2i]).
    CapsAmount,
    /// `Targeted` outside an announce root — replacement/static/loop
    /// position ([CR#115.1a..115.1e,601.2c]).
    PosTargeted,
    /// A filter/reference kind conflicts with its slot's expected kind
    /// ([CR#109.1]).
    KindFilter,
    /// A counter kind used on a carrier its scope forbids ([CR#122.1]).
    KindCounterScope,
    /// A counter reference naming no declared counter kind ([CR#122.1]).
    KindCounterUndeclared,
    /// A designation used on a carrier its scope forbids ([CR#109.3]).
    KindDesignationScope,
    /// A card/token face with no card types ([CR#109.3]).
    FloorTypes,
    /// A subtype whose governing card type is absent from the face
    /// ([CR#205.3d]).
    FloorSubtype,
    /// Printed loyalty on a non-Planeswalker face ([CR#209]).
    FloorLoyalty,
    /// Printed defense on a non-Battle face ([CR#210]).
    FloorDefense,
    /// A token spec with non-permanent card types ([CR#111.1,110.4]).
    FloorTokenTypes,
    /// A literal quantity range with inverted bounds ([CR#115.6]).
    FloorRange,
    /// A target slot whose quantity permits zero targets ([CR#115.1]).
    FloorTargetQty,
    /// A modal choose-count exceeding the number of modes ([CR#700.2d]).
    FloorModalCount,
    /// A modal effect with no modes ([CR#700.2]).
    FloorModalEmpty,
    /// A divided amount statically smaller than the minimum group size
    /// ([CR#601.2d]).
    FloorDivide,
    /// A cost `Do(action)` whose verb is not cost-eligible ([CR#118.3]).
    CostIneligible,
    /// `Count::X` read where no `{X}` is declared by the carrying cost
    /// ([CR#107.3]).
    CostX,
}

impl Code {
    /// Every active code, in manifest order — the drift pin against the
    /// emitted checker-rule manifest.
    pub const ALL: [Code; 29] = [
        Code::BindTarget,
        Code::BindThat,
        Code::BindThatGroup,
        Code::BindIt,
        Code::BindAllotment,
        Code::BindEvent,
        Code::BindNote,
        Code::CapsObject,
        Code::CapsActor,
        Code::CapsPatient,
        Code::CapsDefender,
        Code::CapsAmount,
        Code::PosTargeted,
        Code::KindFilter,
        Code::KindCounterScope,
        Code::KindCounterUndeclared,
        Code::KindDesignationScope,
        Code::FloorTypes,
        Code::FloorSubtype,
        Code::FloorLoyalty,
        Code::FloorDefense,
        Code::FloorTokenTypes,
        Code::FloorRange,
        Code::FloorTargetQty,
        Code::FloorModalCount,
        Code::FloorModalEmpty,
        Code::FloorDivide,
        Code::CostIneligible,
        Code::CostX,
    ];

    /// The stable `E-*` spelling — what fixtures assert and `xtask validate`
    /// prints.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Code::BindTarget => "E-BIND-TARGET",
            Code::BindThat => "E-BIND-THAT",
            Code::BindThatGroup => "E-BIND-THAT-GROUP",
            Code::BindIt => "E-BIND-IT",
            Code::BindAllotment => "E-BIND-ALLOTMENT",
            Code::BindEvent => "E-BIND-EVENT",
            Code::BindNote => "E-BIND-NOTE",
            Code::CapsObject => "E-CAPS-OBJECT",
            Code::CapsActor => "E-CAPS-ACTOR",
            Code::CapsPatient => "E-CAPS-PATIENT",
            Code::CapsDefender => "E-CAPS-DEFENDER",
            Code::CapsAmount => "E-CAPS-AMOUNT",
            Code::PosTargeted => "E-POS-TARGETED",
            Code::KindFilter => "E-KIND-FILTER",
            Code::KindCounterScope => "E-KIND-COUNTER-SCOPE",
            Code::KindCounterUndeclared => "E-KIND-COUNTER-UNDECLARED",
            Code::KindDesignationScope => "E-KIND-DESIGNATION-SCOPE",
            Code::FloorTypes => "E-FLOOR-TYPES",
            Code::FloorSubtype => "E-FLOOR-SUBTYPE",
            Code::FloorLoyalty => "E-FLOOR-LOYALTY",
            Code::FloorDefense => "E-FLOOR-DEFENSE",
            Code::FloorTokenTypes => "E-FLOOR-TOKEN-TYPES",
            Code::FloorRange => "E-FLOOR-RANGE",
            Code::FloorTargetQty => "E-FLOOR-TARGET-QTY",
            Code::FloorModalCount => "E-FLOOR-MODAL-COUNT",
            Code::FloorModalEmpty => "E-FLOOR-MODAL-EMPTY",
            Code::FloorDivide => "E-FLOOR-DIVIDE",
            Code::CostIneligible => "E-COST-INELIGIBLE",
            Code::CostX => "E-COST-X",
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One elaboration error: the code, where in the tree, and why.
#[derive(Debug, Clone)]
pub struct ElabError {
    pub code: Code,
    /// A breadcrumb into the card value (`abilities[1].effect.Sequence[0]`).
    pub path: String,
    pub message: String,
}

impl fmt::Display for ElabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.code, self.message, self.path)
    }
}

/// The loaded registries the checker validates against — the plugin's
/// declared subtypes and counter kinds (`Plugin::subtypes` /
/// `Plugin::counters`).
pub struct Registries<'a> {
    pub subtypes: &'a HashMap<Ident, Subtype>,
    pub counters: &'a HashMap<Ident, Counter>,
}

/// An elaborated card — proof the walk found nothing to refuse. A thin
/// wrapper for now; the resolved-reference IR (computed anaphor indices,
/// `cards.elab.lock`) lands with the anaphor-surface ticket.
#[derive(Debug, Clone)]
pub struct ElabCard {
    pub card: Card,
}

/// Elaborates one card: every face's floors, every ability tree walked with
/// the running binding context, all errors collected (not first-error).
///
/// # Errors
/// Every rule violation found, each with its stable [`Code`].
pub fn elaborate(card: &Card, registries: &Registries) -> Result<ElabCard, Vec<ElabError>> {
    let errors = walk::card(card, registries);
    if errors.is_empty() {
        Ok(ElabCard { card: card.clone() })
    } else {
        Err(errors)
    }
}

/// Elaborates one standalone token definition (`tokens/**/*.ron`), with the
/// token floors ([CR#111.1,110.4] permanent types) in place of the card face
/// floors.
///
/// # Errors
/// Every rule violation found, each with its stable [`Code`].
pub fn elaborate_token(token: &Token, registries: &Registries) -> Result<(), Vec<ElabError>> {
    let errors = walk::token(token, registries);
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[cfg(test)]
mod manifest_tests {
    use super::*;

    /// No checker rule without a table row: the elaborator's code list and
    /// the emitted checker-rule manifest are the same set, in the same
    /// order. (The manifest also carries the reserved `E-COPY-EXCEPT` /
    /// `E-MACRO-*` families once their nodes exist; today both lists are
    /// exactly the active set.)
    #[test]
    fn codes_match_the_emitted_manifest() {
        let manifest: Vec<&str> = tables::tables()
            .rules
            .iter()
            .map(|r| r.code.as_str())
            .collect();
        let codes: Vec<&str> = Code::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(codes, manifest, "Code::ALL must mirror checker-rules.ron");
    }
}
