//! Outcome-gate rendering ([CR#104],[CR#704]) — "You can't lose the game and
//! your opponents can't win the game." (Platinum Angel), "You can't win the
//! game and your opponents can't lose the game." (Abyssal Persecutor).
//! `StaticEffect::OutcomeGate` is NOT a deontic row (see the type's own
//! doc-comment); it gets its own small renderer rather than folding into
//! `deontic.rs`.

use deckmaste_semantics::OutcomeGateKind;
use deckmaste_semantics::Predicate;
use deckmaste_semantics::Reference;
use deckmaste_semantics::RelationPredicate;
use deckmaste_semantics::StaticEffect;

use super::Ctx;
use super::fragment;

/// An adjacent pair of `OutcomeGate` statics over complementary subjects
/// (Platinum Angel/Abyssal Persecutor pair one over `Ref(You)` with one over
/// `OpponentOf(Ref(You))`) merges into the single printed oracle sentence,
/// mirroring the Pacifism can't-attack-and-block merge
/// ([`super::deontic::merged_cant_attack_block`]). `None` when either isn't
/// an `OutcomeGate` (falls through to normal per-ability rendering) — `ctx`
/// is accepted only for parity with that sibling merge; nothing here is
/// subject-relative.
pub(super) fn merge_outcome_gates(
    first: &StaticEffect,
    second: &StaticEffect,
    _ctx: &Ctx,
) -> Option<String> {
    let (who1, gate1) = peel_outcome_gate(first)?;
    let (who2, gate2) = peel_outcome_gate(second)?;
    let subj1 = fragment::capitalize(gate_subject(who1)?);
    let subj2 = gate_subject(who2)?;
    Some(format!(
        "{subj1} can't {} the game and {subj2} can't {} the game.",
        gate_verb(gate1),
        gate_verb(gate2)
    ))
}

/// A lone `OutcomeGate` (no adjacent partner to merge with) as a full
/// sentence: "You can't lose the game." `None` for a `who` shape with no
/// established subject phrase.
pub(super) fn outcome_gate_sentence(who: &Predicate, gate: OutcomeGateKind) -> Option<String> {
    let subj = fragment::capitalize(gate_subject(who)?);
    Some(format!("{subj} can't {} the game.", gate_verb(gate)))
}

/// The `(who, gate)` pair inside an `OutcomeGate` static, seen through
/// macro provenance ([`StaticEffect::Expanded`]).
fn peel_outcome_gate(e: &StaticEffect) -> Option<(&Predicate, OutcomeGateKind)> {
    match e {
        StaticEffect::OutcomeGate { who, gate } => Some((who, *gate)),
        StaticEffect::Expanded(exp) => peel_outcome_gate(&exp.value),
        _ => None,
    }
}

/// "lose" / "win" — the verb an [`OutcomeGateKind`] suppresses.
fn gate_verb(gate: OutcomeGateKind) -> &'static str {
    match gate {
        OutcomeGateKind::CantLose => "lose",
        OutcomeGateKind::CantWin => "win",
    }
}

/// The lowercase player-subject phrase for an outcome gate's `who`:
/// `Ref(You)` -> "you"; `OpponentOf(Ref(You))` -> "your opponents". `None`
/// for any other shape (the caller falls back gracefully rather than
/// guessing).
fn gate_subject(who: &Predicate) -> Option<&'static str> {
    match fragment::strip_expanded(who) {
        Predicate::Ref(Reference::You) => Some("you"),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(
                fragment::strip_expanded(inner),
                Predicate::Ref(Reference::You)
            ) =>
        {
            Some("your opponents")
        }
        _ => None,
    }
}
