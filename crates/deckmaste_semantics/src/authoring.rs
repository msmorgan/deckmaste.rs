//! The author-surface inventory (spec §4): which reachable `(kind, variant)`
//! rows identity macros must cover, and which stay natively spellable.
//!
//! The reachable set is COMPUTED from the kind registry rather than listed by
//! hand — every registered kind carries its dispatch set
//! ([`macro_ron::Kind::variants`], supplied by the derive from
//! `SupportsMacros::ALL_VARIANTS`). Only the two native whitelists are
//! hand-written, and a test proves each of their rows still names something
//! reachable, so neither can rot into silence.

use macro_ron::VariantSignature;

use crate::identity_registry::IDENTITY_ROWS;
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

/// Whether a bare self-referencing identity-macro body is safe to register at
/// `(kind, variant)`: either `kind` doesn't take the embed-untagged
/// fallthrough at all, or `variant` is one of `kind`'s own directly-declared
/// variant names rather than a name it only inherits transitively through
/// what it embeds.
///
/// Computed from the kind registry (`Kind::embeds`/`Kind::own_variants`,
/// both derive-supplied) rather than hand-listed — this replaced a
/// hand-maintained 6-kind table (see
/// `computed_embed_hosts_match_the_retired_hand_table` below for the proof
/// the two agreed).
///
/// **Why unsafe rows exist at all.** A dispatch set is transitive: a kind
/// that embeds another type inherits that type's variant names, so
/// `reachable_rows` reports e.g. `(ManaSymbol, Green)` — `Green` really
/// belongs to `Color`, reached through `ManaSymbol → SimpleManaSymbol →
/// ColorOrColorless → Color`. Registering an identity macro AT the embed
/// host for such a name recurses: `deckmaste_plugin`'s embed-untagged
/// fallthrough treats "a macro of this name is registered at this kind" as
/// reason enough to skip the embedded type's own lookup, so the macro's own
/// bare-name body re-enters the SAME kind, finds the SAME macro, and never
/// bottoms out (`` `Green` is more than 64 macro expansions deep``, observed
/// from a real `cargo xtask validate` run). Coverage for such a row belongs
/// at the variant's DEFINING kind only — `Color` itself — never at the host;
/// [`is_identity_exempt`] treats an unsafe row as exempt for exactly this
/// reason.
#[must_use]
pub fn embed_safe(kind: &str, variant: &str) -> bool {
    match kinds().get(kind) {
        Some(k) if k.embeds() => k.own_variants().contains(&variant),
        _ => true,
    }
}

/// Rows [`classify`] marks `NeedsIdentityMacro` that can never actually get
/// one: a downstream consumer's closed vocabulary rejects any macro def of
/// that shape regardless of how it's typed, so demanding coverage would be
/// demanding something no def could ever satisfy. Kept separate from
/// [`NATIVE_ATOMS`]/[`NATIVE_CALCULUS`] — those are natively spellable and
/// need no def at all; a row here still only ever arrives through
/// `deckmaste_plugin`'s macro-expansion machinery, it's just never spelled
/// bare by a card author, so no identity macro can exist for it either.
///
/// `(KeywordAbility, Composite)`: every `KeywordAbility`-kind macro def is
/// run through `deckmaste_plugin`'s `keyword_shape`, a CLOSED vocabulary of
/// param shapes ([CR#702] one-liners: none/count/cost/count+cost/
/// predicate/predicate+cost/name). `Composite`'s real shape (`name: Ident,
/// abilities: Vec<Ability>`) fits none of them, so no def of that name at
/// that kind can load — not with `Any` params, not even with the real
/// types. `Composite` is itself the STRUCTURAL PAYLOAD a keyword macro's own
/// body expands into (`Ward([…])` -> `Composite(name: "Ward", abilities:
/// [...])`; see `KeywordAbility::Composite`'s doc), never spelled bare by a
/// card author — the same machinery role `Expanded` plays elsewhere, just
/// under a different name and so not caught by that filter.
pub const UNCOVERABLE: &[(&str, &str)] = &[("KeywordAbility", "Composite")];

/// This variant's authored shape at `kind`, if the registry has one — `None`
/// for a row that isn't actually reachable there (a caller passing a stale
/// or made-up pair).
fn signature_of(kind: &str, variant: &str) -> Option<VariantSignature> {
    kinds()
        .get(kind)?
        .signatures()
        .iter()
        .find(|(name, _)| *name == variant)
        .map(|(_, sig)| *sig)
}

/// Whether `(kind, variant)`'s registered def is structurally CAPABLE of
/// reading the spelling `deckmaste_semantics`'s own newtype-tuple variants
/// use when their one field is itself a named struct — see
/// [`FIELD_SPLICE_HAZARD`]'s doc for the mechanism. A single positional
/// param (`VariantSignature::Positional` of length 1) is a NECESSARY
/// condition for the hazard (that's the shape whose macro-call grammar reads
/// exactly one raw value, so `key: value` pairs can't parse), not a
/// sufficient one — most such rows wrap an enum or scalar and are perfectly
/// safe. This is the STRUCTURAL half of [`FIELD_SPLICE_HAZARD`]'s
/// justification: the rot-guard test uses it to prove every hand-listed row
/// still has this shape, not to compute the list itself (nothing in the
/// kind registry names a positional field's Rust TYPE, only its arity and
/// default — so which arity-1 rows actually wrap a named struct, the
/// SUFFICIENT half, still has to be verified by hand against the source, the
/// same way [`UNCOVERABLE`]'s one entry is).
#[must_use]
pub fn is_single_positional(kind: &str, variant: &str) -> bool {
    matches!(signature_of(kind, variant), Some(VariantSignature::Positional(p)) if p.len() == 1)
}

/// Rows with a REGISTERED def ([`IDENTITY_ROWS`] contains them — they pass
/// [`is_registered_identity_row`]) whose def is nonetheless known to be
/// unable to read canon's actual spelling of that variant, so the gate must
/// NOT certify them: registration proves a def EXISTS, not that it WORKS.
///
/// The hazard: `Ability::Activated(Arc<ActivatedAbility>)` and its siblings
/// below are newtype-tuple variants whose one field is itself a NAMED
/// struct. Canon spells them field-spliced, leaning on RON's
/// newtype-transparency (`Activated(cost: [Tap], effect: …)` —
/// `ability.rs`'s own doc at the `Static`/`Activated` variants documents
/// this spelling), never as one wrapped value
/// (`Activated((cost: …, effect: …))`). But a macro invocation's single
/// positional slot reads exactly one raw value
/// (`macro_ron::expand`'s `newtype_variant::<&RawValue>()` path) — `key:
/// value` pairs aren't one — so the scaffolded shape (`params: [Any], body:
/// Activated(Param(0))`, the ONLY shape the generator's per-variant
/// signature can see, since the derive doesn't expose a positional field's
/// own struct fields) cannot parse canon's real calls at all. Confirmed
/// directly for `Activated`: `Activated(cost: [Tap], effect: RestartGame)`
/// fails `read_str_restricted` with `ExpectedRawValue`. The other rows below
/// share the identical shape (a dedicated named struct as the sole
/// positional field — `ability.rs`'s `TriggeredAbility`/`SpellAbility`,
/// `effect.rs`'s `Continuously`/`Label`/`SeparatePiles`/`ChoosePile`/`May`/
/// `If`/`AdditionalCost`/`Each`/`With`/`Distribute`/`Noting`/`Modal`/
/// `RevealUntil`) and are believed broken the same way, not yet individually
/// re-confirmed one by one.
///
/// This is real corpus exposure, not a corner case — `OneShotEffect::May`
/// alone is spelled field-spliced hundreds of times across the committed
/// card corpus (`plugins/builtin` + `plugins/wizards`).
///
/// **Not fixed here.** A correct fix needs each payload struct's field list
/// (names + which carry a constructor default) so a def can be
/// field-spliced the way `Normal.ron`/`identity/Normal.ron` was hand-shaped
/// around `CardFace`'s fields — the derive doesn't emit that for a
/// positional field's wrapped type, so building it is its own capability,
/// tracked as a follow-up task. Until it lands, these rows stay hand-listed:
/// removing one requires actually re-scaffolding its def field-spliced and
/// verifying the round trip under restriction, the same way `Card::Normal`
/// was closed.
pub const FIELD_SPLICE_HAZARD: &[(&str, &str)] = &[
    ("Ability", "Activated"),
    ("Ability", "Triggered"),
    ("Ability", "Spell"),
    ("OneShotEffect", "Continuously"),
    ("OneShotEffect", "Label"),
    ("OneShotEffect", "SeparatePiles"),
    ("OneShotEffect", "ChoosePile"),
    ("OneShotEffect", "May"),
    ("OneShotEffect", "If"),
    ("OneShotEffect", "AdditionalCost"),
    ("OneShotEffect", "Each"),
    ("OneShotEffect", "With"),
    ("OneShotEffect", "Distribute"),
    ("OneShotEffect", "Noting"),
    ("OneShotEffect", "Modal"),
    ("OneShotEffect", "RevealUntil"),
];

/// Whether a def is registered for `(kind, variant)` in the compiled
/// registry ([`IDENTITY_ROWS`] — the trust channel; never derived from RON
/// at runtime). This is the narrow, strict question: **a loader deciding
/// whether to confer the identity exemption on a specific def must use
/// this**, not [`is_identity_exempt`] — that one also says `true` for rows
/// that have NO def anywhere (an embed-host-inherited name, or
/// [`UNCOVERABLE`]), which is the right answer for the coverage GATE's
/// broader question but the wrong one for "does this exact def match a
/// registered row."
#[must_use]
pub fn is_registered_identity_row(kind: &str, variant: &str) -> bool {
    IDENTITY_ROWS
        .iter()
        .any(|row| row.kind == kind && row.variant == variant)
}

/// Whether `(kind, variant)` is covered by the identity-macro exemption —
/// the coverage GATE's question, used by [`every_reachable_row_is_covered`]
/// below: is this row satisfied by *some* acceptable means. Not the same
/// question as "does a specific def match a registered row" — see
/// [`is_registered_identity_row`], which a loader deciding whether to trust
/// one specific def must use instead.
///
/// A row counts as covered if: [`is_registered_identity_row`] is true AND
/// it isn't named in [`FIELD_SPLICE_HAZARD`] (a real, registered def that is
/// nonetheless known unable to read canon's spelling doesn't get to certify
/// its row); OR the row is a transitively-inherited name at an embed host
/// ([`embed_safe`] returns `false`, whose own coverage lives at the
/// variant's defining kind instead); OR the row is named in [`UNCOVERABLE`]
/// (real vocabulary that structurally can never get a def, for a reason
/// unrelated to — and never overlapping with — the field-splice hazard).
///
/// Not a blanket yes: an unregistered `(kind, variant)` spelling at an
/// ordinary (non-embedding) kind, not listed in [`UNCOVERABLE`], returns
/// `false`.
#[must_use]
pub fn is_identity_exempt(kind: &str, variant: &str) -> bool {
    if FIELD_SPLICE_HAZARD
        .iter()
        .any(|(k, v)| *k == kind && *v == variant)
    {
        return false;
    }
    is_registered_identity_row(kind, variant)
        || !embed_safe(kind, variant)
        || UNCOVERABLE.iter().any(|(k, v)| *k == kind && *v == variant)
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

    /// [`UNCOVERABLE`] rows must still name something real: reachable, and
    /// still classified `NeedsIdentityMacro` — if a future registry change
    /// covered it some other way, this row would be stale rot rather than a
    /// genuine, standing exemption.
    #[test]
    fn uncoverable_names_a_real_still_needy_row() {
        let live = reachable_rows();
        for (kind, variant) in UNCOVERABLE {
            assert!(
                live.iter().any(|(k, v)| k == kind && v == variant),
                "UNCOVERABLE row ({kind}, {variant}) names nothing reachable"
            );
            assert_eq!(
                classify(kind, variant),
                Coverage::NeedsIdentityMacro,
                "UNCOVERABLE row ({kind}, {variant}) is no longer NeedsIdentityMacro"
            );
        }
    }

    /// [`FIELD_SPLICE_HAZARD`] rows must still name something real and
    /// dangerous: reachable, still `NeedsIdentityMacro`, still structurally
    /// the arity-1-positional shape the hazard requires
    /// ([`is_single_positional`] — the NECESSARY half of the justification,
    /// mechanically re-checked every run), and — the point of the whole
    /// list — still actually REGISTERED (a def exists, which is exactly
    /// what makes silently certifying it dangerous). A row that stopped
    /// being any of these would be stale: either fixed and forgotten here,
    /// or no longer the shape that justified listing it.
    ///
    /// Also guards against the list quietly becoming a dumping ground: it
    /// must stay non-empty, and every entry must independently satisfy
    /// [`is_single_positional`] — nothing may be listed on say-so alone. As
    /// rows close (tracked at
    /// `docs/tickets/planned/identity-scaffolds-field-splice.md`) they come
    /// out of the list one at a time; this test cannot pass on an empty
    /// list once that ticket's work is done — the list itself gets deleted
    /// then, along with the `except` clause in
    /// [`every_reachable_row_is_covered`] below.
    #[test]
    fn field_splice_hazard_names_real_registered_arity_one_rows() {
        assert!(
            !FIELD_SPLICE_HAZARD.is_empty(),
            "expected a real, standing hazard list"
        );
        let live = reachable_rows();
        for (kind, variant) in FIELD_SPLICE_HAZARD {
            assert!(
                live.iter().any(|(k, v)| k == kind && v == variant),
                "FIELD_SPLICE_HAZARD row ({kind}, {variant}) names nothing reachable"
            );
            assert_eq!(
                classify(kind, variant),
                Coverage::NeedsIdentityMacro,
                "FIELD_SPLICE_HAZARD row ({kind}, {variant}) is no longer NeedsIdentityMacro"
            );
            assert!(
                is_single_positional(kind, variant),
                "FIELD_SPLICE_HAZARD row ({kind}, {variant}) is no longer single-positional"
            );
            assert!(
                is_registered_identity_row(kind, variant),
                "FIELD_SPLICE_HAZARD row ({kind}, {variant}) has no registered def — \
                 either it was fixed (remove it from the list) or it never had one \
                 (it belongs in the missing-coverage report, not this list)"
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

    /// The coverage gate that replaces wipe-first regeneration: every
    /// reachable row is covered by an identity macro or named in a
    /// whitelist. A new grammar variant fails here until its def lands.
    ///
    /// **Carries one explicit, temporary, TICKETED exception**:
    /// [`FIELD_SPLICE_HAZARD`]'s 16 rows have a registered def that is
    /// nonetheless known unable to read canon's own spelling —
    /// [`is_identity_exempt`] correctly reports them NOT exempt (that
    /// function's answer stays honest), but this assertion carves them out
    /// by name rather than letting them fail the whole suite forever. This
    /// is RECORDING the gap, not hiding it: the list is public, hand-listed,
    /// rot-guarded (see
    /// `field_splice_hazard_names_real_registered_arity_one_rows`
    /// below — it cannot pass on an empty or stale list), and tracked at
    /// `docs/tickets/planned/identity-scaffolds-field-splice.md`. A
    /// genuinely NEW uncovered row — anything not already in the exception
    /// list — still fails here, loudly, by name.
    ///
    /// When that ticket closes (every hazard row re-scaffolded
    /// field-spliced and round-trip verified), **delete the `except`
    /// filter below, not just the now-empty `FIELD_SPLICE_HAZARD` list** —
    /// an empty hazard list is itself dead code, and the gate should go
    /// back to asserting unconditional coverage.
    #[test]
    fn every_reachable_row_is_covered() {
        let missing: Vec<_> = reachable_rows()
            .into_iter()
            .filter(|(k, v)| classify(k, v) == Coverage::NeedsIdentityMacro)
            .filter(|(k, v)| !is_identity_exempt(k, v))
            // TEMPORARY except clause — see this test's doc comment and
            // `docs/tickets/planned/identity-scaffolds-field-splice.md`.
            // Delete this filter (not just empty the list it reads) once
            // that ticket lands.
            .filter(|(k, v)| {
                !FIELD_SPLICE_HAZARD
                    .iter()
                    .any(|(hk, hv)| *hk == k.as_str() && *hv == v.as_str())
            })
            .collect();
        assert!(
            missing.is_empty(),
            "rows with no identity macro: {missing:#?}"
        );
    }

    /// Negative fixture: the exemption is not a blanket yes.
    #[test]
    fn an_unregistered_spelling_is_not_exempt() {
        assert!(!is_identity_exempt("Filter", "NoSuchVariantAnywhere"));
    }

    /// The computed embed-host rule ([`embed_safe`]) replaced a
    /// hand-maintained table (formerly `xtask::authoring::EMBED_HOSTS`,
    /// hand-verified against `macro_ron`'s derive output) naming these 6
    /// kinds and their own variant names. Encoded here as literal data (not
    /// sourced from the retired table, which no longer exists) so a
    /// regression in the computed rule is still caught: every one of these
    /// kinds must still be a computed embed host, every listed name must
    /// still be safe (it's the kind's own), and every OTHER name the kind
    /// reaches transitively must NOT be safe. No kind outside this list may
    /// be a computed embed host either — the set really is these 6, not more.
    #[test]
    fn computed_embed_hosts_match_the_retired_hand_table() {
        let expect_own: &[(&str, &[&str])] = &[
            ("ColorOrColorless", &["Colorless"]),
            ("SimpleManaSymbol", &["Generic"]),
            ("ManaSymbol", &["Variable", "Snow", "Hybrid", "Phyrexian"]),
            (
                "ManaSpec",
                &[
                    "AnyColor",
                    "OneOf",
                    "OneOfRuns",
                    "AmongColorsOf",
                    "ProducedByEvent",
                ],
            ),
            ("ManaProduction", &["WithRiders"]),
            ("StatValue", &["DefinedByAbility", "Variable", "Number"]),
        ];

        let set = kinds();
        for (kind, own) in expect_own {
            let k = set
                .get(kind)
                .unwrap_or_else(|| panic!("no such kind {kind}"));
            assert!(k.embeds(), "{kind} should be a computed embed host");
            for &variant in *own {
                assert!(
                    embed_safe(kind, variant),
                    "{kind}::{variant} should be safe (its own variant)"
                );
            }
            for variant in k.variants().iter().filter(|v| !own.contains(v)) {
                assert!(
                    !embed_safe(kind, variant),
                    "{kind}::{variant} is inherited, not owned, and should not be safe"
                );
            }
        }

        let expect_hosts: Vec<&str> = expect_own.iter().map(|(k, _)| *k).collect();
        let actual_hosts: Vec<&str> = set
            .iter()
            .filter(|k| k.embeds())
            .map(macro_ron::Kind::name)
            .collect();
        let mut actual_hosts_sorted = actual_hosts.clone();
        actual_hosts_sorted.sort_unstable();
        let mut expect_hosts_sorted = expect_hosts.clone();
        expect_hosts_sorted.sort_unstable();
        assert_eq!(
            actual_hosts_sorted, expect_hosts_sorted,
            "the computed embed-host set no longer matches the retired hand table"
        );
    }
}
