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
//! Runs both behind `cargo xtask validate` (`validate::validate_plugin`,
//! batch reporting over a whole plugin) AND, since `cards-elab-load-gate`, as
//! the load-time GATE itself: `Plugin::load`/`load_with_prelude` walk every
//! finished card/token eagerly (`plugin::elaborate_finished`) with a staged
//! deny/warn rollout keyed on the plugin directory's own name
//! ([`Stage::for_root`]) — a live engine can never resolve a card that would
//! panic on an unbound reference.

pub mod tables;
mod walk;

use std::collections::HashMap;
use std::fmt;

use deckmaste_core::Card;
use deckmaste_core::Counter;
use deckmaste_core::DesignationDecl;
use deckmaste_core::Ident;
use deckmaste_core::KeywordDecl;
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
    /// The sorted singular anaphor `That(Sort)` (or `The(label)`'s singular
    /// read) with NO compatible antecedent on the stack ([CR#608.2d]) —
    /// unbound, sort-mismatched, or reaching only a Many antecedent.
    BindThat,
    /// The plural anaphor `They`/`Them(Sort)` with no compatible Many
    /// antecedent on the stack ([CR#608.2d]).
    BindThatGroup,
    /// `It` with no antecedent at all: outside every `Each`/`DivideAmong`/
    /// `Where`/`Pick` binder AND with an empty antecedent stack
    /// ([CR#608.2]).
    BindIt,
    /// `Allotment` read outside a `DivideAmong` body ([CR#601.2d]).
    BindAllotment,
    /// An event-role reference outside any event body ([CR#603.2e,608.2k]).
    BindEvent,
    /// A noted key read but never noted in scope ([CR#607.2]).
    BindNote,
    /// The R2 uniqueness gate: an anaphor (`It`/`That`/`They`/`Them`/
    /// `ThatMany`) with a SECOND same-kind, compatible antecedent in scope —
    /// resolution would be a guess, so it is a load error instead
    /// ([CR#608.2d]); the error text offers the `Label`/`The` and
    /// `Target(n)` fallbacks. STRICT until the corpus dry-run calibrates the
    /// gate; the one pre-approved loosening (exact-sort precedence) is the
    /// emitted `exact_sort_precedence` flag.
    BindAmbiguous,
    /// A labeled reference (`The`/`TheGroup`/a `ChoosePile` label) naming no
    /// label in scope, or one of the wrong cardinality ([CR#608.2d]).
    BindLabel,
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
    /// An event position that must bottom out in master forms doesn't — a
    /// bare `Not` where the lane requires a kind anchor, or `Not` over an
    /// unanchored operand ([CR#603.2]; no freeze-everything `CantHappen`).
    CapsAnchor,
    /// A conjunction over incompatible master forms, or a cause verb whose
    /// entailed fact form contradicts its pattern — the pattern can never
    /// match ([CR#603.2g]).
    CapsContradiction,
    /// A `Within` refinement in a live event lane — vacuous outside
    /// history counting ([CR#603.2]).
    LaneWithin,
    /// A `OneOrMore`/`Nth` refinement in a lane that forbids it
    /// ([CR#603.2c]).
    LaneBatch,
    /// An event construct the lane's bridge matcher cannot faithfully
    /// evaluate ([CR#603.2] — a pattern must match its occurrences, never
    /// silently mis-match): load-capped per the emitted bridge-caps table
    /// until the one-evaluator rebase (`engine-one-evaluator`) lifts it.
    BridgeCap,
    /// `Targeted` outside an announce root — replacement/static/loop
    /// position ([CR#115.1a..115.1e,601.2c]).
    PosTargeted,
    /// A damage-prevention effect written as a generic `Instead` — an
    /// `Instead` replacing a damage event with NOTHING. Prevention is its
    /// own marked class ([CR#615.1,615.1a] "prevent" effects), gated by
    /// `CantPrevent` ([CR#615.12]); a no-op damage `Instead` would dodge
    /// that gate, so it must be spelled `Prevention(…)`.
    PosPrevention,
    /// An enter rider on a non-battlefield destination ([CR#614.12] — riders
    /// modify how a permanent enters the battlefield; a card arriving
    /// anywhere else has no tapped/attacking arrival state).
    PosRider,
    /// An exchange-family batch primitive (`Effect::Simultaneous` /
    /// `Action::GainControl`) outside an exchange-family macro's body
    /// ([CR#701.12]) — the engine's simultaneous-batch wiring is
    /// exchange-only until it generalizes; the cap mirrors the
    /// `E-BRIDGE-CAP` discipline (reject at load, never silently
    /// misexecute).
    PosSimultaneous,
    /// A filter/reference kind conflicts with its slot's expected kind
    /// ([CR#109.1]).
    KindFilter,
    /// A counter kind used on a carrier its scope forbids ([CR#122.1]).
    KindCounterScope,
    /// A counter reference naming no declared counter kind ([CR#122.1]).
    KindCounterUndeclared,
    /// A designation used on a carrier its scope forbids ([CR#109.3]).
    KindDesignationScope,
    /// A noted key read with a domain its declared kind doesn't store — a
    /// linked reader refers only to the kind of information its writer
    /// noted ([CR#607.2]).
    KindNoteDomain,
    /// A deed's PATIENT kind conflicts with its relation's patient scope
    /// (the emitted `patientScope` rows): a blocked thing is an attacking
    /// creature ([CR#509.1a]), an attacked thing is a player, planeswalker,
    /// or battle ([CR#508.1b]), ….
    KindPatientScope,
    /// A face/token subtype VALUE whose category (governing card types)
    /// disagrees with the loaded registry declaration of the same name —
    /// the declaration is the authority for the [CR#205.3] category index.
    KindSubtypeCategory,
    /// A keyword use whose args don't fit the declared `ParamShape`
    /// ([CR#702] one-liners take typed args): a bare
    /// `Composite(name: "Ward", …)` spelled without its invocation loses
    /// the declared cost.
    KindKeywordShape,
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
    /// A block-arrangement bound satisfiable only by zero blockers — a
    /// declared block involves at least one, so the deontic row is dead
    /// ([CR#509.1]).
    FloorBlockQty,
    /// A modal choose-count exceeding the number of modes ([CR#700.2d]).
    FloorModalCount,
    /// A modal effect with no modes ([CR#700.2]).
    FloorModalEmpty,
    /// A divided amount statically smaller than the minimum group size
    /// ([CR#601.2d]).
    FloorDivide,
    /// A pile-shape floor ([CR#700.3]): a `SeparatePiles` with no pile
    /// labels, or duplicate pile labels.
    FloorPiles,
    /// An `Nth` occurrence index below one — a 0th occurrence never occurs
    /// ([CR#603.2g]).
    FloorNth,
    /// A bare `Library`/`Stack` zone as a `Move` destination ([CR#401.4] — an
    /// ordered-zone position exists only via `Library(Anchor)`; the stack is
    /// never a destination, objects reach it only by casting/activating/
    /// triggering, [CR#405.1]).
    FloorDestination,
    /// A `DeedAgent` with NEITHER arm present ([CR#702.11d,702.16b] — the
    /// two-armed agent constrains through at least one arm; an empty agent
    /// is meaningless).
    FloorDeedAgent,
    /// A cost `Do(action)` whose verb is not cost-eligible ([CR#118.3]).
    CostIneligible,
    /// `Count::X` read where no `{X}` is declared by the carrying cost
    /// ([CR#107.3]).
    CostX,
}

impl Code {
    /// Every active code, in manifest order — the drift pin against the
    /// emitted checker-rule manifest.
    pub const ALL: [Code; 48] = [
        Code::BindTarget,
        Code::BindThat,
        Code::BindThatGroup,
        Code::BindIt,
        Code::BindAllotment,
        Code::BindEvent,
        Code::BindNote,
        Code::BindAmbiguous,
        Code::BindLabel,
        Code::CapsObject,
        Code::CapsActor,
        Code::CapsPatient,
        Code::CapsDefender,
        Code::CapsAmount,
        Code::CapsAnchor,
        Code::CapsContradiction,
        Code::LaneWithin,
        Code::LaneBatch,
        Code::BridgeCap,
        Code::PosTargeted,
        Code::PosRider,
        Code::PosPrevention,
        Code::PosSimultaneous,
        Code::KindFilter,
        Code::KindCounterScope,
        Code::KindCounterUndeclared,
        Code::KindDesignationScope,
        Code::KindNoteDomain,
        Code::KindPatientScope,
        Code::KindSubtypeCategory,
        Code::KindKeywordShape,
        Code::FloorTypes,
        Code::FloorSubtype,
        Code::FloorLoyalty,
        Code::FloorDefense,
        Code::FloorTokenTypes,
        Code::FloorRange,
        Code::FloorTargetQty,
        Code::FloorBlockQty,
        Code::FloorModalCount,
        Code::FloorModalEmpty,
        Code::FloorDivide,
        Code::FloorPiles,
        Code::FloorNth,
        Code::FloorDestination,
        Code::FloorDeedAgent,
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
            Code::BindAmbiguous => "E-BIND-AMBIGUOUS",
            Code::BindLabel => "E-BIND-LABEL",
            Code::CapsObject => "E-CAPS-OBJECT",
            Code::CapsActor => "E-CAPS-ACTOR",
            Code::CapsPatient => "E-CAPS-PATIENT",
            Code::CapsDefender => "E-CAPS-DEFENDER",
            Code::CapsAmount => "E-CAPS-AMOUNT",
            Code::CapsAnchor => "E-CAPS-ANCHOR",
            Code::CapsContradiction => "E-CAPS-CONTRADICTION",
            Code::LaneWithin => "E-LANE-WITHIN",
            Code::LaneBatch => "E-LANE-BATCH",
            Code::BridgeCap => "E-BRIDGE-CAP",
            Code::PosTargeted => "E-POS-TARGETED",
            Code::PosRider => "E-POS-RIDER",
            Code::PosPrevention => "E-POS-PREVENTION",
            Code::PosSimultaneous => "E-POS-SIMULTANEOUS",
            Code::KindFilter => "E-KIND-FILTER",
            Code::KindCounterScope => "E-KIND-COUNTER-SCOPE",
            Code::KindCounterUndeclared => "E-KIND-COUNTER-UNDECLARED",
            Code::KindDesignationScope => "E-KIND-DESIGNATION-SCOPE",
            Code::KindNoteDomain => "E-KIND-NOTE-DOMAIN",
            Code::KindPatientScope => "E-KIND-PATIENT-SCOPE",
            Code::KindSubtypeCategory => "E-KIND-SUBTYPE-CATEGORY",
            Code::KindKeywordShape => "E-KIND-KEYWORD-SHAPE",
            Code::FloorTypes => "E-FLOOR-TYPES",
            Code::FloorSubtype => "E-FLOOR-SUBTYPE",
            Code::FloorLoyalty => "E-FLOOR-LOYALTY",
            Code::FloorDefense => "E-FLOOR-DEFENSE",
            Code::FloorTokenTypes => "E-FLOOR-TOKEN-TYPES",
            Code::FloorRange => "E-FLOOR-RANGE",
            Code::FloorTargetQty => "E-FLOOR-TARGET-QTY",
            Code::FloorBlockQty => "E-FLOOR-BLOCK-QTY",
            Code::FloorModalCount => "E-FLOOR-MODAL-COUNT",
            Code::FloorModalEmpty => "E-FLOOR-MODAL-EMPTY",
            Code::FloorDivide => "E-FLOOR-DIVIDE",
            Code::FloorPiles => "E-FLOOR-PILES",
            Code::FloorNth => "E-FLOOR-NTH",
            Code::FloorDestination => "E-FLOOR-DESTINATION",
            Code::FloorDeedAgent => "E-FLOOR-DEED-AGENT",
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
/// declared subtypes, counter kinds, designations, and keyword shapes
/// (`Plugin::subtypes` / `Plugin::counters` / `Plugin::designations` /
/// `Plugin::keywords`). Each row carries its dependent index
/// (category/scope/shape) as data, so the open vocabularies stay open while
/// the checker still enforces the index.
pub struct Registries<'a> {
    pub subtypes: &'a HashMap<Ident, Subtype>,
    pub counters: &'a HashMap<Ident, Counter>,
    pub designations: &'a HashMap<Ident, DesignationDecl>,
    pub keywords: &'a HashMap<Ident, KeywordDecl>,
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

/// The staged `Plugin::load` rollout (`cards-elab-load-gate`): whether a
/// plugin's malformed cards/tokens fail its WHOLE load, or are individually
/// skipped with a counted warning. Computed from the plugin directory's own
/// name — a property of the load call, not a global switch: `wizards` (the
/// generated corpus) warns for one milestone; every hand-authored plugin —
/// and any other directory, including an ad hoc test tempdir — denies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Any elaboration failure fails the whole
    /// `Plugin::load`/`load_with_prelude` call outright.
    Deny,
    /// An elaboration failure is skipped — the offending card/token is never
    /// reachable via `Plugin::card`/`Plugin::token` — but the load itself
    /// still succeeds; findings accumulate in `Plugin::elab_report`.
    Warn,
}

impl Stage {
    /// `root`'s own final path component decides: only a directory literally
    /// named `wizards` warns. Preserved by copying a plugin elsewhere (e.g. a
    /// temp-dir drift demonstration) as long as the directory keeps its name.
    #[must_use]
    pub fn for_root(root: &std::path::Path) -> Stage {
        if root.file_name().and_then(|f| f.to_str()) == Some("wizards") {
            Stage::Warn
        } else {
            Stage::Deny
        }
    }
}

/// One binding resolved while walking a card — the review-facing twin of an
/// [`ElabError`]: not a violation, but which antecedent an anaphor bound to.
/// Surfaced by `cargo xtask elaborate --dump <card>`
/// ([`elaborate_with_resolutions`]); the load-gate's plain [`elaborate`] never
/// pays for collecting it.
#[derive(Debug, Clone)]
pub struct Resolution {
    /// The same breadcrumb shape as [`ElabError::path`].
    pub path: String,
    /// What the anaphor at `path` resolved to.
    pub description: String,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.description)
    }
}

/// Elaborates one card like [`elaborate`], additionally collecting every
/// resolved binding (target slots, `That`/`It`, event roles, notes,
/// allotment, `{X}`, counter/designation scopes) for review.
#[must_use = "dropping the result discards both the elaboration outcome and the resolutions"]
pub fn elaborate_with_resolutions(
    card: &Card,
    registries: &Registries,
) -> (Result<ElabCard, Vec<ElabError>>, Vec<Resolution>) {
    let (errors, resolutions) = walk::card_traced(card, registries);
    let result = if errors.is_empty() {
        Ok(ElabCard { card: card.clone() })
    } else {
        Err(errors)
    };
    (result, resolutions)
}

/// Elaborates one token like [`elaborate_token`], additionally collecting
/// every resolved binding for review.
#[must_use = "dropping the result discards both the elaboration outcome and the resolutions"]
pub fn elaborate_token_with_resolutions(
    token: &Token,
    registries: &Registries,
) -> (Result<(), Vec<ElabError>>, Vec<Resolution>) {
    let (errors, resolutions) = walk::token_traced(token, registries);
    let result = if errors.is_empty() { Ok(()) } else { Err(errors) };
    (result, resolutions)
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

#[cfg(test)]
mod stage_tests {
    use std::path::Path;

    use super::Stage;

    /// Only a directory literally named `wizards` warns — the staged
    /// rollout table names hand-authored plugins (and everything else,
    /// including a test tempdir) as deny.
    #[test]
    fn only_a_directory_named_wizards_warns() {
        assert_eq!(
            Stage::for_root(Path::new("/some/where/plugins/wizards")),
            Stage::Warn
        );
        // Preserved by copying the plugin elsewhere, as long as the
        // directory keeps its name (the temp-copy drift demonstration).
        assert_eq!(
            Stage::for_root(Path::new("/tmp/xyz123/wizards")),
            Stage::Warn
        );
        for other in [
            "/some/where/plugins/builtin",
            "/some/where/plugins/canon",
            "/some/where/plugins/testing",
            "/some/where/plugins/demo",
            "/tmp/xyz123",
            "/tmp/xyz123/not-wizards",
        ] {
            assert_eq!(
                Stage::for_root(Path::new(other)),
                Stage::Deny,
                "{other} must deny"
            );
        }
    }
}
