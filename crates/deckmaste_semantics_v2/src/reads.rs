//! Structural reads over the syntax: kind projection and binding resolution.
//!
//! These are ports of `lean/Semantics/Check/Words.lean` and
//! `Check/Phrase.lean` — the parts that are pure reads. **Nothing that refuses
//! is ported.** The Lean gate is the only checker
//! (`docs/decisions/semantics-v2.md` §10, §13); this module exists because
//! lowering needs the same projections the checker computes, not because
//! anything here decides whether a term is well formed.
//!
//! ## Kinds
//!
//! Lean's `Kind` index is inferred rather than carried:
//! [`Predicate::kind`](kind_of_predicate) gives the kind a phrase fixes by
//! itself (`hasType` is an object predicate; `hasPossessor` is polymorphic and
//! gives `None`), and [`kind_or`](kind_or_predicate) falls back to the kind the
//! context expects.
//!
//! ## The antecedent stack
//!
//! [`Binding`] is one mention in the discourse; the stack is a list, newest
//! first. A [`Window`](crate::words::Window) selects the stretch of it a
//! pronoun resolves in, and [`view`] is that selection.
//!
//! ## Declared facts
//!
//! Two projections read a generated registry rather than the syntax: which kind
//! holds a designation, and which kind holds a named counter. That registry is
//! `plugins_v2/builtin`'s declarations, extracted by xtask — a table this crate
//! does not own and must not depend on. So the reads take it as a parameter:
//! see [`Facts`], with [`NoFacts`] as the table-free reading, which is exactly
//! the Lean fallback for an unregistered label.

use crate::phrase::NounPhrase;
use crate::phrase::Predicate;
use crate::words::AmountShape;
use crate::words::AttachmentSide;
use crate::words::CardType;
use crate::words::CounterKind;
use crate::words::Deed;
use crate::words::Determiner;
use crate::words::Kind;
use crate::words::Letter;
use crate::words::NounWord;
use crate::words::OutcomeSort;
use crate::words::PileFace;
use crate::words::Plurality;
use crate::words::ProjAxis;
use crate::words::QualitySort;
use crate::words::Reach;
use crate::words::Window;
use crate::words::Zone;

// ---------------------------------------------------------------------------
// Declared facts
// ---------------------------------------------------------------------------

/// The declared facts a projection consults: the registry columns that decide
/// what a label denotes. `plugins_v2/builtin`'s declarations are the source
/// (§15); this crate reads whatever the caller supplies.
pub trait Facts {
    /// The kind that holds `designation`, if the declaration says an object or
    /// a player holds it; `None` for a game-wide designation or an
    /// undeclared label. Lean `DesignationLabel.holder`.
    fn designation_holder(&self, designation: &str) -> Option<Kind>;

    /// The kind a named counter sits on. Lean `CounterFacts.holder`, whose
    /// fallback for an undeclared label is [`Kind::Object`].
    fn counter_holder(&self, label: &str) -> Kind {
        let _ = label;
        Kind::Object
    }
}

/// No declared facts: every designation is game-wide and every counter sits on
/// an object. The reading a caller with no registry in hand gets, and the same
/// answer the Lean gives an unregistered label.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoFacts;

impl Facts for NoFacts {
    fn designation_holder(&self, _designation: &str) -> Option<Kind> {
        None
    }
}

// ---------------------------------------------------------------------------
// Card types
// ---------------------------------------------------------------------------

/// Canonical order for remembered type facts; order in the written modifiers is
/// immaterial. Lean `normalizeTypes`.
#[must_use]
pub fn normalize_types(types: &[CardType]) -> Vec<CardType> {
    [
        CardType::Creature,
        CardType::Artifact,
        CardType::Land,
        CardType::Enchantment,
        CardType::Instant,
        CardType::Sorcery,
        CardType::Planeswalker,
        CardType::Battle,
        CardType::Kindred,
    ]
    .into_iter()
    .filter(|t| types.contains(t))
    .collect()
}

/// Lean `mergeTypes`.
#[must_use]
pub fn merge_types(left: &[CardType], right: &[CardType]) -> Vec<CardType> {
    let mut both = left.to_vec();
    both.extend_from_slice(right);
    normalize_types(&both)
}

/// Lean `commonTypes`.
#[must_use]
pub fn common_types(left: &[CardType], right: &[CardType]) -> Vec<CardType> {
    let both: Vec<CardType> = left.iter().filter(|t| right.contains(t)).copied().collect();
    normalize_types(&both)
}

/// Lean `Payload.joinSeed`: an empty side contributes nothing, two populated
/// sides meet.
fn join_seed(left: &[CardType], right: &[CardType]) -> Vec<CardType> {
    if left.is_empty() {
        return right.to_vec();
    }
    if right.is_empty() {
        return left.to_vec();
    }
    common_types(left, right)
}

// ---------------------------------------------------------------------------
// Kinds
// ---------------------------------------------------------------------------

/// Lean `joinKinds`: two equal kinds are one, two different ones join.
#[must_use]
pub fn join_kinds(a: &Kind, b: &Kind) -> Kind {
    if a == b {
        a.clone()
    } else {
        Kind::Join {
            left: Box::new(a.clone()),
            right: Box::new(b.clone()),
        }
    }
}

/// Lean `Kind.lteAtom`: `x` (never itself a join) against a join on the right.
#[must_use]
pub fn kind_lte_atom(x: &Kind, y: &Kind) -> bool {
    match y {
        Kind::Join { left, right } => kind_lte_atom(x, left) || kind_lte_atom(x, right),
        _ => x == y,
    }
}

/// Lean `Kind.lte`: a join on the left, then a join on the right, then
/// equality.
#[must_use]
pub fn kind_lte(x: &Kind, y: &Kind) -> bool {
    match x {
        Kind::Join { left, right } => kind_lte(left, y) && kind_lte(right, y),
        _ => kind_lte_atom(x, y),
    }
}

/// Lean `NounWord.kind`.
#[must_use]
pub fn kind_of_noun_word(word: &NounWord) -> Kind {
    match word {
        NounWord::OfType { word, .. } | NounWord::Copied { word } => kind_of_noun_word(word),
        NounWord::Player => Kind::Player,
        NounWord::Join => Kind::Join {
            left: Box::new(Kind::Object),
            right: Box::new(Kind::Player),
        },
        NounWord::Pile => Kind::Pile,
        _ => Kind::Object,
    }
}

/// Lean `Reach.kind`.
#[must_use]
pub fn kind_of_reach(reach: &Reach) -> Kind {
    match reach {
        Reach::Parameter { shape } => shape.kind.clone(),
        Reach::Word { word } | Reach::UnionHalf { word } | Reach::Verbed { word, .. } => {
            kind_of_noun_word(word)
        }
        Reach::ThatTurn => Kind::TurnRef,
        _ => Kind::Object,
    }
}

/// Lean `CounterKind.scope`: which kind a counter of this sort sits on.
#[must_use]
pub fn scope_of_counter_kind(kind: &CounterKind, facts: &impl Facts) -> Kind {
    match kind {
        CounterKind::Boost { .. } | CounterKind::Keyword { .. } => Kind::Object,
        CounterKind::Named { label } => facts.counter_holder(label),
    }
}

/// Lean `ProjAxis.scope`: which kind the axis reads a value off.
#[must_use]
pub fn scope_of_proj_axis(axis: &ProjAxis, facts: &impl Facts) -> Kind {
    match axis {
        ProjAxis::Stat { .. } => Kind::Object,
        ProjAxis::PlayerStat { .. } => Kind::Player,
        ProjAxis::Counter { kind } => scope_of_counter_kind(kind, facts),
        ProjAxis::AnyCounter { kind } => kind.clone(),
    }
}

/// Lean `Predicate.kind?`: the kind this predicate fixes by itself. `None`
/// where the predicate is polymorphic and takes its kind from the context.
#[must_use]
pub fn kind_of_predicate(predicate: &Predicate, facts: &impl Facts) -> Option<Kind> {
    match predicate {
        Predicate::WithBindings { body, .. } | Predicate::InCaller { body, .. } => {
            kind_of_predicate(body, facts)
        }
        Predicate::AnyPlayer
        | Predicate::Opponent
        | Predicate::ChosenPlayer { .. }
        | Predicate::ChoseExtreme { .. } => Some(Kind::Player),
        Predicate::QualityNoun { sort, .. } => Some(Kind::Quality { sort: sort.clone() }),
        Predicate::CounterKindOn { .. } => Some(Kind::Quality {
            sort: QualitySort::CounterKind,
        }),
        Predicate::HasDesignation { designation, .. } => facts.designation_holder(designation),
        Predicate::Compare { axes, .. } => axes.first().map(|axis| scope_of_proj_axis(axis, facts)),
        Predicate::Superlative { domain: inner, .. }
        | Predicate::CompareOver { domain: inner, .. }
        | Predicate::Not { predicate: inner } => kind_of_predicate(inner, facts),
        Predicate::And { conjuncts } => kind_of_any(conjuncts, facts),
        Predicate::Or { disjuncts } => kind_of_all(disjuncts, facts),
        // The host side of an attachment is polymorphic ("the creature it is
        // attached to" is an object, "enchanted player" a player), so it takes
        // its kind from the context like the rest of this group.
        Predicate::Attachment {
            side: AttachmentSide::Host,
            ..
        }
        | Predicate::HasPossessor { .. }
        | Predicate::InCombat { .. }
        | Predicate::HappenedTo { .. }
        | Predicate::WithMostVotes
        | Predicate::Other
        | Predicate::NotChosen
        | Predicate::OtherThan { .. }
        | Predicate::CoinCameUp { .. }
        | Predicate::Targets { .. } => None,
        _ => Some(Kind::Object),
    }
}

/// Lean `Predicate.kindOfAny`: the first conjunct that fixes a kind.
#[must_use]
pub fn kind_of_any(predicates: &[Predicate], facts: &impl Facts) -> Option<Kind> {
    predicates.iter().find_map(|p| kind_of_predicate(p, facts))
}

/// Lean `Predicate.kindOfAll`: a disjunction names the join of its disjuncts'
/// kinds — "a creature or player" is one phrase at a joined kind.
#[must_use]
pub fn kind_of_all(predicates: &[Predicate], facts: &impl Facts) -> Option<Kind> {
    let mut out: Option<Kind> = None;
    for predicate in predicates.iter().rev() {
        out = match (kind_of_predicate(predicate, facts), out) {
            (Some(k), Some(rest)) => Some(join_kinds(&k, &rest)),
            (Some(k), None) => Some(k),
            (None, rest) => rest,
        };
    }
    out
}

/// Lean `Predicate.disjunctKinds`: the distinct kinds a disjunction's disjuncts
/// name, first appearance first.
#[must_use]
pub fn disjunct_kinds(predicates: &[Predicate], facts: &impl Facts) -> Vec<Kind> {
    let mut rest: Vec<Kind> = Vec::new();
    for predicate in predicates.iter().rev() {
        if let Some(kind) = kind_of_predicate(predicate, facts)
            && !rest.contains(&kind)
        {
            rest.insert(0, kind);
        }
    }
    rest
}

/// Lean `Predicate.joins`: whether a disjunction joins kinds ("creature or
/// player").
#[must_use]
pub fn predicate_joins(predicates: &[Predicate], facts: &impl Facts) -> bool {
    disjunct_kinds(predicates, facts).len() >= 2
}

/// Lean `Predicate.kindOr`: the predicate's own kind, or the one the context
/// expects.
#[must_use]
pub fn kind_or_predicate(default: &Kind, predicate: &Predicate, facts: &impl Facts) -> Kind {
    kind_of_predicate(predicate, facts).unwrap_or_else(|| default.clone())
}

/// Lean `Predicate.inKind`: whether a disjunct belongs to the `k` half of a
/// join — its kind is `k`, or unnamed.
#[must_use]
pub fn predicate_in_kind(predicate: &Predicate, kind: &Kind, facts: &impl Facts) -> bool {
    kind_of_predicate(predicate, facts).is_none_or(|own| own == *kind)
}

/// Lean `NounPhrase.kind?`.
#[must_use]
pub fn kind_of_noun_phrase(phrase: &NounPhrase, facts: &impl Facts) -> Option<Kind> {
    match phrase {
        NounPhrase::WithBindings { body, .. } | NounPhrase::InCaller { body, .. } => {
            kind_of_noun_phrase(body, facts)
        }
        NounPhrase::Gap { kind } | NounPhrase::TheRest { kind, .. } => Some(kind.clone()),
        NounPhrase::You
        | NounPhrase::CombatPlayer { .. }
        | NounPhrase::PlayerGroup { .. }
        | NounPhrase::PossessorOf { .. } => Some(Kind::Player),
        NounPhrase::Described { predicate, .. } => kind_of_predicate(predicate, facts),
        NounPhrase::EachOf { group } => kind_of_noun_phrase(group, facts),
        NounPhrase::And { phrases } | NounPhrase::Or { phrases } => {
            Some(kind_of_noun_phrases(phrases, facts))
        }
        NounPhrase::PileOf { .. } => Some(Kind::Pile),
        NounPhrase::Pro { reach, .. } => Some(kind_of_reach(reach)),
        NounPhrase::AttachHost { head, .. } => Some(kind_of_noun_word(head)),
        _ => Some(Kind::Object),
    }
}

/// Lean `NounPhrase.kindOfAll`.
#[must_use]
pub fn kind_of_noun_phrases(phrases: &[NounPhrase], facts: &impl Facts) -> Kind {
    let mut out: Option<Kind> = None;
    for phrase in phrases.iter().rev() {
        let own = kind_of_noun_phrase(phrase, facts).unwrap_or(Kind::Object);
        out = Some(match out {
            Some(rest) => join_kinds(&own, &rest),
            None => own,
        });
    }
    out.unwrap_or(Kind::Object)
}

/// Lean `NounPhrase.kindOr`.
#[must_use]
pub fn kind_or_noun_phrase(default: &Kind, phrase: &NounPhrase, facts: &impl Facts) -> Kind {
    kind_of_noun_phrase(phrase, facts).unwrap_or_else(|| default.clone())
}

// ---------------------------------------------------------------------------
// The antecedent stack
// ---------------------------------------------------------------------------

/// Provenance: the verb that last moved a binding, whether it was on the
/// battlefield then, and whether it changed zones. Lean `Stamp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stamp {
    pub verb: Deed,
    pub was_field: bool,
    pub moved: bool,
}

/// Lean `Origin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Token,
    Copy,
}

/// One slot of a lexical frame: its address in the caller's stack, and the
/// mention it holds.
pub type FrameSlot = (Option<Vec<usize>>, Determiner, Plurality, Payload);

/// What a binding knows about its referent. Lean indexes this by `Kind`; here
/// the kind is derived by [`Payload::kind`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Payload {
    Object {
        ty: Vec<CardType>,
        zone: Option<Zone>,
        prov: Option<Stamp>,
        orig: Option<Origin>,
        size: Option<u32>,
    },
    Player {
        chosen: bool,
    },
    Quality {
        q: QualitySort,
    },
    Outcome {
        sort: OutcomeSort,
    },
    Amount {
        shape: AmountShape,
    },
    Gap,
    Letter {
        l: Letter,
    },
    TurnRef,
    /// An ability object and its remembered location. Stack origin is the
    /// default; movement can remove it [CR#113.1c,724.1b,724.2b].
    Ability {
        orig: Option<Origin>,
        zone: Option<Zone>,
    },
    Pile {
        zone: Option<Zone>,
        size: Option<u32>,
        face: Option<PileFace>,
    },
    Join {
        l: Box<Payload>,
        r: Box<Payload>,
    },
    /// A lexical frame. Addresses are relative to the tail below the frame.
    ParameterFrame {
        scope: usize,
        caller_width: usize,
        slots: Vec<FrameSlot>,
    },
    /// A forgotten ordinary mention retained solely for an active lexical
    /// capture.
    Hidden {
        value: Box<Payload>,
    },
    /// Temporary caller-view masking; unlike forgetting, it is restored after
    /// the supplied body.
    Masked {
        scope: usize,
        value: Box<Payload>,
    },
}

impl Payload {
    /// The kind a payload denotes; an ability on the stack is an object
    /// [CR#113.1c]. Lean `Payload.kind`.
    #[must_use]
    pub fn kind(&self) -> Kind {
        match self {
            Payload::Object { .. } | Payload::Ability { .. } => Kind::Object,
            Payload::Player { .. } => Kind::Player,
            Payload::Quality { q } => Kind::Quality { sort: q.clone() },
            Payload::Outcome { .. } => Kind::Outcome,
            Payload::Gap
            | Payload::ParameterFrame { .. }
            | Payload::Hidden { .. }
            | Payload::Masked { .. } => Kind::Gap,
            Payload::Amount { .. } => Kind::Quality {
                sort: QualitySort::Number,
            },
            Payload::Letter { l } => Kind::Letter { letter: *l },
            Payload::TurnRef => Kind::TurnRef,
            Payload::Pile { .. } => Kind::Pile,
            Payload::Join { l, r } => Kind::Join {
                left: Box::new(l.kind()),
                right: Box::new(r.kind()),
            },
        }
    }

    /// Lean `Payload.isHidden`.
    #[must_use]
    pub fn is_hidden(&self) -> bool {
        matches!(self, Payload::Hidden { .. } | Payload::Masked { .. })
    }

    /// Lean `Payload.visible`: the mention under any masking or forgetting.
    #[must_use]
    pub fn visible(&self) -> &Payload {
        match self {
            Payload::Hidden { value } | Payload::Masked { value, .. } => value.visible(),
            payload => payload,
        }
    }

    /// Lean `Payload.isForgotten`.
    #[must_use]
    pub fn is_forgotten(&self) -> bool {
        match self {
            Payload::Hidden { .. } => true,
            Payload::Masked { value, .. } => value.is_forgotten(),
            _ => false,
        }
    }

    /// Lean `Payload.withVisible`: replaces the visible mention, keeping this
    /// payload's masking.
    #[must_use]
    pub fn with_visible(&self, value: Payload) -> Payload {
        match self {
            Payload::Hidden { value: inner } => Payload::Hidden {
                value: Box::new(inner.with_visible(value)),
            },
            Payload::Masked {
                scope,
                value: inner,
            } => Payload::Masked {
                scope: *scope,
                value: Box::new(inner.with_visible(value)),
            },
            _ => value,
        }
    }

    /// Lean `Payload.withoutMasks`.
    #[must_use]
    pub fn without_masks(&self) -> Payload {
        match self {
            Payload::Masked { value, .. } => value.without_masks(),
            Payload::Hidden { value } => Payload::Hidden {
                value: Box::new(value.without_masks()),
            },
            payload => payload.clone(),
        }
    }

    /// Lean `Payload.restoreMasks`: puts this payload's masking back around
    /// `value`.
    #[must_use]
    pub fn restore_masks(&self, value: Payload) -> Payload {
        match self {
            Payload::Masked {
                scope,
                value: inner,
            } => Payload::Masked {
                scope: *scope,
                value: Box::new(inner.restore_masks(value)),
            },
            Payload::Hidden { value: inner } => inner.restore_masks(value),
            _ => value,
        }
    }

    /// Lean `Payload.zone`.
    #[must_use]
    pub fn zone(&self) -> Option<Zone> {
        match self {
            Payload::Object { zone, .. }
            | Payload::Ability { zone, .. }
            | Payload::Pile { zone, .. } => *zone,
            Payload::Join { l, r } => l.zone().or_else(|| r.zone()),
            _ => None,
        }
    }

    /// Lean `Payload.ty`.
    #[must_use]
    pub fn ty(&self) -> Vec<CardType> {
        match self {
            Payload::Object { ty, .. } => ty.clone(),
            Payload::Join { l, r } => join_seed(&l.ty(), &r.ty()),
            _ => Vec::new(),
        }
    }

    /// Lean `Payload.size`.
    #[must_use]
    pub fn size(&self) -> Option<u32> {
        match self {
            Payload::Object { size, .. } | Payload::Pile { size, .. } => *size,
            Payload::Join { l, r } => l.size().or_else(|| r.size()),
            _ => None,
        }
    }

    /// Lean `Payload.face`.
    #[must_use]
    pub fn face(&self) -> Option<PileFace> {
        match self {
            Payload::Pile { face, .. } => *face,
            _ => None,
        }
    }

    /// Lean `Payload.prov`.
    #[must_use]
    pub fn prov(&self) -> Option<&Stamp> {
        match self {
            Payload::Object { prov, .. } => prov.as_ref(),
            Payload::Join { l, r } => l.prov().or_else(|| r.prov()),
            _ => None,
        }
    }

    /// Lean `Payload.orig`.
    #[must_use]
    pub fn orig(&self) -> Option<Origin> {
        match self {
            Payload::Object { orig, .. } | Payload::Ability { orig, .. } => *orig,
            Payload::Join { l, r } => l.orig().or_else(|| r.orig()),
            _ => None,
        }
    }

    /// Lean `Payload.joined`.
    #[must_use]
    pub fn joined(&self) -> bool {
        matches!(self, Payload::Join { .. })
    }
}

/// One mention in the discourse. Lean `Binding`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub det: Determiner,
    pub plur: Plurality,
    pub payload: Payload,
}

impl Binding {
    /// The kind a binding denotes, derived from its payload. Lean
    /// `Binding.kind`.
    #[must_use]
    pub fn kind(&self) -> Kind {
        self.payload.kind()
    }

    /// Lean `Binding.isOperandFrame`.
    #[must_use]
    pub fn is_operand_frame(&self) -> bool {
        matches!(self.payload, Payload::ParameterFrame { .. })
    }
}

/// The antecedent stack: every mention textually earlier, newest first. Lean
/// `Bindings`.
pub type Bindings = Vec<Binding>;

/// Validates an expansion's introduction pattern against the current prefix. A
/// letter mention is optional: it introduces a binding only if the enclosing
/// context lacked it. Other mentions must match in order; they never search
/// past an unrelated binding. Lean `introductionWidth`.
#[must_use]
pub fn introduction_width(pattern: &[Kind], bindings: &[Binding]) -> Option<usize> {
    match (pattern.split_first(), bindings.split_first()) {
        (None, _) => Some(0),
        (Some((Kind::Letter { letter }, rest)), Some((head, tail))) => {
            if head.kind() == (Kind::Letter { letter: *letter }) {
                introduction_width(rest, tail).map(|n| n + 1)
            } else {
                introduction_width(rest, bindings)
            }
        }
        (Some((Kind::Letter { .. }, rest)), None) => introduction_width(rest, &[]),
        (Some((kind, rest)), Some((head, tail))) => {
            if head.kind() == *kind {
                introduction_width(rest, tail).map(|n| n + 1)
            } else {
                None
            }
        }
        (Some(_), None) => None,
    }
}

/// Reads the binding at `address`. An ordinary binding has one address
/// component; a value inside a lexical frame has two. Lean `bindingAt`.
#[must_use]
pub fn binding_at(bindings: &[Binding], address: &[usize]) -> Option<Binding> {
    match address {
        [i] => bindings.get(*i).map(|b| Binding {
            det: b.det,
            plur: b.plur,
            payload: b.payload.visible().clone(),
        }),
        [i, j] => {
            let Payload::ParameterFrame { slots, .. } = &bindings.get(*i)?.payload else {
                return None;
            };
            let (_, det, plur, payload) = slots.get(*j)?;
            Some(Binding {
                det: *det,
                plur: *plur,
                payload: payload.clone(),
            })
        }
        _ => None,
    }
}

/// Writes `value` at `address`, keeping the masking the slot already carried.
/// Lean `setBindingAt`.
#[must_use]
pub fn set_binding_at(bindings: &[Binding], address: &[usize], value: Binding) -> Bindings {
    let mut out = bindings.to_vec();
    match address {
        [i] => {
            if let Some(old) = out.get(*i) {
                let payload = old.payload.with_visible(value.payload.clone());
                out[*i] = Binding { payload, ..value };
            }
        }
        [i, j] => {
            if let Some(binding) = out.get_mut(*i)
                && let Payload::ParameterFrame { slots, .. } = &mut binding.payload
                && let Some(slot) = slots.get_mut(*j)
            {
                *slot = (slot.0.clone(), value.det, value.plur, value.payload);
            }
        }
        _ => {}
    }
    out
}

/// Lean `shiftAddress`.
#[must_use]
fn shift_address(by: usize, address: &[usize]) -> Vec<usize> {
    match address.split_first() {
        None => Vec::new(),
        Some((head, rest)) => {
            let mut out = vec![head + by];
            out.extend_from_slice(rest);
            out
        }
    }
}

/// The index of the lexical frame keyed `scope`.
fn frame_index(bindings: &[Binding], scope: usize) -> Option<usize> {
    bindings.iter().position(
        |b| matches!(&b.payload, Payload::ParameterFrame { scope: key, .. } if *key == scope),
    )
}

/// The address of a lexical frame's `index`-th operand: the caller slot it was
/// read from, or the slot itself. Lean `operandAddress`.
#[must_use]
pub fn operand_address(bindings: &[Binding], index: usize, scope: usize) -> Option<Vec<usize>> {
    let frame = frame_index(bindings, scope)?;
    let Payload::ParameterFrame { slots, .. } = &bindings.get(frame)?.payload else {
        return None;
    };
    let (address, ..) = slots.get(index)?;
    Some(match address {
        None => vec![frame, index],
        Some(address) => shift_address(frame + 1, address),
    })
}

/// The end of the caller's own stretch below the frame keyed `scope`. Lean
/// `callerBoundary`.
#[must_use]
pub fn caller_boundary(scope: usize, bindings: &[Binding]) -> Option<usize> {
    let frame = frame_index(bindings, scope)?;
    let Payload::ParameterFrame { caller_width, .. } = &bindings.get(frame)?.payload else {
        return None;
    };
    Some(frame + 1 + caller_width)
}

/// The addresses a window selects. Lean `windowAddresses`.
#[must_use]
pub fn window_addresses(window: &Window, bindings: &[Binding]) -> Vec<Vec<usize>> {
    let visible: Vec<(usize, &Binding)> = bindings
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_operand_frame() && !b.payload.is_hidden())
        .collect();
    let select = |xs: &[(usize, &Binding)]| -> Vec<Vec<usize>> {
        xs.iter().map(|(i, _)| vec![*i]).collect()
    };
    let introduced = |pattern: &[Kind]| -> Option<usize> {
        let stack: Vec<Binding> = visible.iter().map(|(_, b)| (*b).clone()).collect();
        introduction_width(pattern, &stack)
    };
    match window {
        Window::Parameter { scope, index } => {
            operand_address(bindings, *index as usize, *scope as usize)
                .into_iter()
                .collect()
        }
        Window::Whole => select(&visible),
        Window::Top { depth } => select(&visible[..visible.len().min(*depth as usize)]),
        Window::Below { depth } => select(&visible[visible.len().min(*depth as usize)..]),
        Window::Introduced { pattern } => introduced(pattern)
            .map(|n| select(&visible[..visible.len().min(n)]))
            .unwrap_or_default(),
        Window::OutsideIntroduced { pattern } => introduced(pattern)
            .map(|n| select(&visible[visible.len().min(n)..]))
            .unwrap_or_default(),
    }
}

/// The stretch of the stack a window resolves in. Lean `view`.
#[must_use]
pub fn view(window: &Window, bindings: &[Binding]) -> Bindings {
    window_addresses(window, bindings)
        .iter()
        .filter_map(|address| binding_at(bindings, address))
        .collect()
}

/// Hides macro-local mentions while retaining physical slots and lexical
/// references. Lean `enterCaller`.
#[must_use]
pub fn enter_caller(scope: usize, bindings: &[Binding]) -> Bindings {
    let boundary = caller_boundary(scope, bindings).unwrap_or(bindings.len());
    bindings
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if i < boundary && !b.is_operand_frame() {
                Binding {
                    payload: Payload::Masked {
                        scope,
                        value: Box::new(b.payload.clone()),
                    },
                    ..b.clone()
                }
            } else {
                b.clone()
            }
        })
        .collect()
}

/// Restores the incoming visibility without undoing updates or permanent
/// forgetting. Lean `leaveCaller`.
#[must_use]
pub fn leave_caller(before: &[Binding], after: &[Binding]) -> Bindings {
    let width = after.len().saturating_sub(before.len());
    let mut out: Bindings = after[..width].to_vec();
    for (old, current) in before.iter().zip(after[width..].iter()) {
        out.push(Binding {
            payload: old.payload.restore_masks(current.payload.without_masks()),
            ..current.clone()
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::PossessorAxis;

    fn object() -> Binding {
        Binding {
            det: Determiner::A,
            plur: Plurality::One,
            payload: Payload::Object {
                ty: vec![CardType::Creature],
                zone: Some(Zone::Battlefield),
                prov: None,
                orig: None,
                size: None,
            },
        }
    }

    fn player() -> Binding {
        Binding {
            det: Determiner::The,
            plur: Plurality::One,
            payload: Payload::Player { chosen: false },
        }
    }

    /// "a creature or player" is one phrase at a joined kind.
    #[test]
    fn a_disjunction_joins_its_disjuncts_kinds() {
        let disjuncts = vec![
            Predicate::HasType {
                r#type: CardType::Creature,
            },
            Predicate::AnyPlayer,
        ];
        assert_eq!(
            kind_of_all(&disjuncts, &NoFacts),
            Some(Kind::Join {
                left: Box::new(Kind::Object),
                right: Box::new(Kind::Player),
            })
        );
        assert!(predicate_joins(&disjuncts, &NoFacts));
    }

    /// A polymorphic predicate names no kind of its own and takes the
    /// context's.
    #[test]
    fn a_polymorphic_predicate_takes_the_contexts_kind() {
        let predicate = Predicate::HasPossessor {
            axis: PossessorAxis::Controller,
            possessor: Box::new(NounPhrase::You),
        };
        assert_eq!(kind_of_predicate(&predicate, &NoFacts), None);
        assert_eq!(
            kind_or_predicate(&Kind::Player, &predicate, &NoFacts),
            Kind::Player
        );
        assert!(predicate_in_kind(&predicate, &Kind::Object, &NoFacts));
    }

    /// A host-side attachment predicate is polymorphic; an object predicate is
    /// not.
    #[test]
    fn an_attachment_host_names_no_kind() {
        let host = Predicate::Attachment {
            side: AttachmentSide::Host,
            word: None,
            counterpart: None,
        };
        assert_eq!(kind_of_predicate(&host, &NoFacts), None);
        let attachment = Predicate::Attachment {
            side: AttachmentSide::Attachment,
            word: None,
            counterpart: None,
        };
        assert_eq!(kind_of_predicate(&attachment, &NoFacts), Some(Kind::Object));
    }

    /// A joined kind is reached by each of its halves and by neither half
    /// alone in the other direction.
    #[test]
    fn a_join_is_reached_by_each_half() {
        let join = Kind::Join {
            left: Box::new(Kind::Object),
            right: Box::new(Kind::Player),
        };
        assert!(kind_lte(&Kind::Object, &join));
        assert!(kind_lte(&Kind::Player, &join));
        assert!(!kind_lte(&join, &Kind::Object));
        assert!(kind_lte(&join, &join));
    }

    /// The whole window is every visible mention; `top`/`below` cut it.
    #[test]
    fn windows_select_stretches_of_the_stack() {
        let stack = vec![object(), player()];
        assert_eq!(view(&Window::Whole, &stack).len(), 2);
        assert_eq!(view(&Window::Top { depth: 1 }, &stack), vec![object()]);
        assert_eq!(view(&Window::Below { depth: 1 }, &stack), vec![player()]);
        assert_eq!(view(&Window::Top { depth: 9 }, &stack).len(), 2);
    }

    /// A masked mention is invisible to a window and restored by
    /// `leave_caller`.
    #[test]
    fn masking_hides_a_mention_and_leaving_restores_it() {
        let stack = vec![object(), player()];
        let masked = vec![
            Binding {
                payload: Payload::Masked {
                    scope: 0,
                    value: Box::new(object().payload),
                },
                ..object()
            },
            player(),
        ];
        assert_eq!(view(&Window::Whole, &masked), vec![player()]);
        assert_eq!(leave_caller(&stack, &masked), stack);
    }

    /// An introduction pattern matches in order and skips an absent letter.
    #[test]
    fn an_introduction_pattern_matches_in_order() {
        let stack = vec![object(), player()];
        assert_eq!(
            introduction_width(&[Kind::Object, Kind::Player], &stack),
            Some(2)
        );
        assert_eq!(introduction_width(&[Kind::Player], &stack), None);
        assert_eq!(
            introduction_width(&[Kind::Letter { letter: Letter::X }, Kind::Object], &stack),
            Some(1)
        );
    }

    /// A designation's holder kind comes from the declared facts, not the
    /// syntax.
    #[test]
    fn a_designation_takes_its_holder_kind_from_the_facts() {
        struct Monarch;
        impl Facts for Monarch {
            fn designation_holder(&self, designation: &str) -> Option<Kind> {
                (designation == "Monarch").then_some(Kind::Player)
            }
        }
        let predicate = Predicate::HasDesignation {
            designation: "Monarch".to_string(),
            holder: None,
        };
        assert_eq!(kind_of_predicate(&predicate, &Monarch), Some(Kind::Player));
        assert_eq!(kind_of_predicate(&predicate, &NoFacts), None);
    }
}
