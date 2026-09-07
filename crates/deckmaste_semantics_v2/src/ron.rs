//! The RON dialect: the macro-position kinds and the reader options every
//! `plugins_v2` read goes through.
//!
//! A kind is one `SupportsMacros` enum, keyed by its serde name. Kinds exist
//! only to disambiguate same-name macros at different usage sites
//! (`docs/decisions/semantics-v2.md` §12); they are not Lean's
//! `MacroParameters` classes, which are a proof device.
//!
//! Which enums are kinds is Lean's own answer: the types its
//! `attribute [semantic_expression]` lines name, in
//! `lean/Semantics/{Words,Phrase,Triggers,Abilities}.lean`. A macro may stand
//! at exactly those positions.

use macro_ron::Kind;
use macro_ron::KindSet;
use macro_ron::MacroSet;
use macro_ron::SupportsMacros;

/// The serde name of the declaration position (`macro_ron::MacroDef`). A
/// meta-macro declares `kinds: [Macro]` and expands to a definition.
pub const MACRO_KIND: &str = "Macro";

/// The kinds the declaration meta-macros in `plugins_v2/<plugin>/macros/meta/`
/// produce. Four of them — `Subtype`, `CounterKind`, `TurnPart`, `Type` — name
/// a position a card also writes, so a declaration registered under one is
/// invocable there by its bare name; the rest are name-erasing loader tags
/// that open no card position, exactly as v1's `TypeDef` and `Counter` do.
pub const DECLARATION_KINDS: &[&str] = &[
    "AbilityWord",
    "CounterKind",
    "Designation",
    "FlavorWord",
    "KeywordAbility",
    "KeywordAction",
    "Subtype",
    "TurnPart",
    "Type",
];

/// Every macro-position kind in the v2 dialect, keyed by serde name.
///
/// The kind facts (`expanded`/`embed`/`literal` markers) come from each type's
/// `#[derive(SupportsMacros)]`, so this only enumerates the types. v2 declares
/// none of those markers: a mirrored constructor is the Lean constructor and
/// nothing else, so no type carries an `Expanded` provenance variant and
/// expansion is name-erasing.
#[must_use]
pub fn kinds() -> KindSet {
    let mut kinds = KindSet::new();
    // `lean/Semantics/Phrase.lean`
    kinds.add(crate::phrase::GameEvent::kind());
    kinds.add(crate::phrase::NounPhrase::kind());
    kinds.add(crate::phrase::Predicate::kind());
    kinds.add(crate::phrase::Amount::kind());
    kinds.add(crate::phrase::Quantity::kind());
    kinds.add(crate::phrase::ZoneExpr::kind());
    kinds.add(crate::phrase::Condition::kind());
    // `lean/Semantics/Triggers.lean`
    kinds.add(crate::triggers::Duration::kind());
    kinds.add(crate::triggers::Timing::kind());
    kinds.add(crate::triggers::UsageLimit::kind());
    // `lean/Semantics/Abilities.lean`
    kinds.add(crate::abilities::Instruction::kind());
    kinds.add(crate::abilities::StaticSpec::kind());
    kinds.add(crate::abilities::Cost::kind());
    kinds.add(crate::abilities::Ability::kind());
    kinds.add(crate::abilities::TokenSpec::kind());
    // `lean/Semantics/Words.lean`
    kinds.add(crate::words::Window::kind());
    // `Delta` is generic in Lean (`Delta (α : Type)`) and generic in Rust, and
    // `#[derive(SupportsMacros)]` rejects generics — so its kind is
    // hand-built and carries no dispatch set. Registration is what a macro of
    // kind `Delta` needs; the checks that consult the dispatch set (the cycle
    // check, the restricted-read ban) lose precision here and nothing else.
    kinds.add(Kind::new("Delta"));
    // Declaration positions: `Macro` is where a meta-macro's product is read
    // (`macros/meta/`), and each declaration kind is where the declarations it
    // produces register.
    kinds.add(Kind::new(MACRO_KIND));
    for name in DECLARATION_KINDS {
        kinds.add(Kind::new(name));
    }
    kinds
}

/// The raw `ron::Options` everything is read and written with.
///
/// `implicit_some` keeps `Option` fields flat and `unwrap_variant_newtypes`
/// lets a struct-carrying variant read flat (`HasType(type: Creature)` rather
/// than `HasType((type: Creature))`) — which is what makes a mirrored
/// constructor spell like its Lean original.
#[must_use]
pub fn raw_options() -> ::ron::Options {
    ::ron::Options::default().with_default_extension(
        ::ron::extensions::Extensions::IMPLICIT_SOME
            | ::ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

/// An empty macro set over [`kinds`] reading [`raw_options`] — the base every
/// plugin load starts from before its `macros/` directory is folded in.
#[must_use]
pub fn macro_set() -> MacroSet {
    MacroSet::new(kinds()).with_options(raw_options())
}

#[cfg(test)]
mod tests {
    use super::kinds;

    /// Every enum Lean marks `semantic_expression` is a registered kind: that
    /// list is what "a macro may occupy it" means (§12).
    #[test]
    fn every_semantic_expression_type_is_a_kind() {
        let kinds = kinds();
        for name in [
            "GameEvent",
            "NounPhrase",
            "Predicate",
            "Amount",
            "Quantity",
            "ZoneExpr",
            "Condition",
            "Duration",
            "Timing",
            "UsageLimit",
            "Instruction",
            "StaticSpec",
            "Cost",
            "Ability",
            "TokenSpec",
            "Window",
            "Delta",
        ] {
            assert!(kinds.contains(name), "`{name}` must be a registered kind");
        }
    }

    /// Every declaration kind the meta-macros produce is registered, or a
    /// `plugins_v2` declaration of that class fails to load with
    /// `UnknownKind`.
    #[test]
    fn every_declaration_kind_is_registered() {
        let kinds = kinds();
        assert!(kinds.contains(super::MACRO_KIND));
        for name in super::DECLARATION_KINDS {
            assert!(kinds.contains(name), "`{name}` must be a registered kind");
        }
    }

    /// A derived kind carries its dispatch set, or the checks that consult it
    /// (the cycle check, the restricted-read ban) silently weaken.
    #[test]
    fn derived_kinds_carry_their_dispatch_set() {
        let kinds = kinds();
        for name in ["Instruction", "Predicate", "NounPhrase", "Ability"] {
            let kind = kinds.get(name).expect("registered above");
            assert!(
                !kind.variants().is_empty(),
                "`{name}` registered without its dispatch set"
            );
        }
    }

    /// No v2 type remembers its invocation: without an `Expanded` variant an
    /// expansion is read through and the term is the Lean term.
    #[test]
    fn no_kind_remembers_its_invocation() {
        for kind in kinds().iter() {
            assert!(
                !kind.remembers_invocation(),
                "`{}` remembers invocations; v2 mirrors Lean constructors only",
                kind.name()
            );
        }
    }
}
