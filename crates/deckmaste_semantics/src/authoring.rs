//! The author-surface inventory (spec §4): which reachable `(kind, variant)`
//! rows identity macros must cover, and which stay natively spellable.
//!
//! The reachable set is COMPUTED from the kind registry rather than listed by
//! hand — every registered kind carries its dispatch set
//! ([`macro_ron::Kind::variants`], supplied by the derive from
//! `SupportsMacros::ALL_VARIANTS`). Only the two native whitelists are
//! hand-written, and a test proves each of their rows still names something
//! reachable, so neither can rot into silence.

use crate::ron::kinds;

/// How a reachable row satisfies the closed inventory. Nothing may be
/// unclassified — that is what makes the inventory closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    /// The small built-in indexed scope calculus. Designated structural
    /// grammar by spec §4: pretending the calculus is macros would hide its
    /// binding behavior.
    NativeCalculus,
    /// A closed atom at a registered kind that stays natively spellable.
    NativeAtom,
    /// Everything else — an identity macro must cover this spelling.
    NeedsIdentityMacro,
}

/// The structural binding forms of spec §4's carve-out. They live on four
/// different enums, so the kind is part of each row.
pub const NATIVE_CALCULUS: &[(&str, &str)] = &[
    ("OneShotEffect", "Targeted"),
    // `Target(n)` and `Targets(n)` are the INDEXED announce-list entries, not
    // anaphora over the antecedent stack.
    ("Reference", "Target"),
    ("Selection", "Targets"),
    ("TargetSpec", "Distinct"),
];

/// Closed atoms at REGISTERED kinds that stay native. Deliberately small: a
/// kind outside the registry needs no entry, because suppression never reaches
/// it — `Cmp`, the phase/step enums and `FaceLayout` are unregistered by
/// design and are not listed here.
///
/// The five `KeywordAbility::ALL` intrinsics (`FirstStrike`, `DoubleStrike`,
/// `Deathtouch`, `Trample`, `Vigilance`, [CR#702]): `deckmaste_plugin`'s
/// `classification` test suite asserts every `KeywordAbility`-kind macro
/// classifies non-intrinsic — "intrinsics belong in the enum" is the
/// suite's own comment — so a macro literally named after one of these
/// would be flagged drift, not author vocabulary. Canon spells them bare
/// (`Deathtouch`, `Trample`, …) at the `KeywordAbility` position and no
/// macro exists for them anywhere; classifying them `NativeAtom` here is
/// what keeps the identity-macro scaffold generator (macro-author-surface
/// spec §5) from either forcing a scaffold the classification suite
/// rejects or leaving a real gap unclassified.
pub const NATIVE_ATOMS: &[(&str, &str)] = &[
    ("KeywordAbility", "FirstStrike"),
    ("KeywordAbility", "DoubleStrike"),
    ("KeywordAbility", "Deathtouch"),
    ("KeywordAbility", "Trample"),
    ("KeywordAbility", "Vigilance"),
];

/// Every `(kind, variant)` row the registry dispatches, sorted and deduped.
///
/// Struct kinds and the name-erasing loader tags (`Macro`, `KeywordAction`,
/// and kin) contribute nothing: they carry no variant dispatch, so their
/// dispatch set is empty and they drop out here rather than needing an
/// exclusion list.
#[must_use]
pub fn reachable_rows() -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = kinds()
        .iter()
        .flat_map(|kind| {
            let name = kind.name().to_string();
            kind.variants()
                .iter()
                .filter(|variant| **variant != EXPANDED)
                .map(move |variant| (name.clone(), (*variant).to_string()))
        })
        .collect();
    rows.sort();
    rows.dedup();
    rows
}

/// The invocation-provenance variant a `remembers_expansion` kind carries.
/// Machinery, not vocabulary: the macro layer synthesizes it and re-reads it
/// in a free (expansion) context, and an author never writes it — so it is not
/// reachable as authored input, and an identity macro of that name would
/// collide with the wrapper the layer emits.
///
/// Filtered at EVERY kind, not just remembering ones: a dispatch set is
/// transitive, so a kind that flattens or embeds a remembering type inherits
/// this name while itself reporting `remembers_invocation() == false`.
const EXPANDED: &str = "Expanded";

/// Which class a row belongs to.
#[must_use]
pub fn classify(kind: &str, variant: &str) -> Coverage {
    let listed = |list: &[(&str, &str)]| list.iter().any(|(k, v)| *k == kind && *v == variant);
    if listed(NATIVE_CALCULUS) {
        Coverage::NativeCalculus
    } else if listed(NATIVE_ATOMS) {
        Coverage::NativeAtom
    } else {
        Coverage::NeedsIdentityMacro
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The §4 carve-out, by the names the types actually use.
    #[test]
    fn structural_binding_forms_are_native_calculus() {
        for (kind, variant) in [
            ("OneShotEffect", "Targeted"),
            ("Reference", "Target"),
            ("Selection", "Targets"),
            ("TargetSpec", "Distinct"),
        ] {
            assert_eq!(
                classify(kind, variant),
                Coverage::NativeCalculus,
                "{kind}::{variant} is designated structural grammar (spec §4)"
            );
        }
    }

    /// A whitelist row naming a kind or variant that no longer exists is rot:
    /// it silently stops exempting anything and nothing complains. This is the
    /// check that keeps the hand-written half honest as the grammar moves.
    #[test]
    fn whitelists_name_only_reachable_rows() {
        let live = reachable_rows();
        for (kind, variant) in NATIVE_CALCULUS.iter().chain(NATIVE_ATOMS) {
            assert!(
                live.iter().any(|(k, v)| k == kind && v == variant),
                "whitelist row ({kind}, {variant}) names nothing reachable"
            );
        }
    }

    /// The registry really does hand back variant rows — a `reachable_rows`
    /// that silently returned nothing would make every later coverage claim
    /// vacuous.
    #[test]
    fn the_registry_yields_reachable_rows() {
        let rows = reachable_rows();
        assert!(
            rows.len() > 100,
            "only {} rows; dispatch sets missing?",
            rows.len()
        );
        assert!(rows.iter().any(|(k, v)| k == "Color" && v == "White"));
    }

    /// Invocation provenance is machinery, not author vocabulary: the layer
    /// synthesizes `Expanded(…)` and re-reads it free, so it is not an authored
    /// spelling and must never demand an identity macro of its own.
    #[test]
    fn invocation_provenance_is_not_reachable_as_authored_input() {
        assert!(
            !reachable_rows().iter().any(|(_, v)| v == EXPANDED),
            "`{EXPANDED}` is synthesized, not authored"
        );
    }

    /// `reachable_rows` filters provenance by NAME, so the name has to be the
    /// one every remembering kind actually uses. If a kind ever spells its
    /// provenance variant differently, this fires rather than letting the
    /// filter silently stop matching.
    #[test]
    fn every_remembering_kind_spells_its_provenance_variant_expanded() {
        for kind in kinds().iter().filter(|k| k.remembers_invocation()) {
            assert!(
                kind.variants().contains(&EXPANDED),
                "`{}` remembers invocations but has no `{EXPANDED}` variant",
                kind.name()
            );
        }
    }
}
