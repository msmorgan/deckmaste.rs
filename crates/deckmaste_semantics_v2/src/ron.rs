//! The RON dialect: the macro-position kinds and the reader options every
//! `plugins_v2` read goes through.
//!
//! A kind is one `SupportsMacros` enum, keyed by its serde name. Kinds exist
//! only to disambiguate same-name macros at different usage sites
//! (`docs/decisions/semantics-v2.md` §12); they are not Lean's
//! `MacroParameters` classes, which are a proof device.
//!
//! Which enums are kinds is mostly Lean's own answer: the types its
//! `attribute [semantic_expression]` lines name, in
//! `lean/Semantics/{Words,Phrase,Triggers,Abilities}.lean`. `TurnPart` is
//! macroable beyond that list, because a whole declaration family registers
//! at its name and its declarations are turn parts; every such type derives
//! `SupportsMacros`, and a unit test holds the derive-to-kind mapping total.
//! Alongside them are the declaration positions in [`DECLARATION_KINDS`],
//! which are loader tags rather than syntax types.

use macro_ron::Kind;
use macro_ron::KindSet;
use macro_ron::MacroSet;
use macro_ron::ParamTypeSet;
use macro_ron::SupportsMacros;

/// The serde name of the declaration position (`macro_ron::MacroDef`). A
/// meta-macro declares `kinds: [Macro]` and expands to a definition.
pub const MACRO_KIND: &str = "Macro";

/// The FAMILY kind each declaration meta-macro in
/// `plugins_v2/<plugin>/macros/meta/` names first. A meta may name a second,
/// SEMANTIC kind after it (`KeywordAction` → `Instruction`, `KeywordAbility`
/// and `AbilityWord` → `Ability`), which is what makes the declaration
/// invocable at the position its body occupies; the family kind is the
/// declaration's identity and is what `deckmaste_construction_core` reads.
///
/// Three families — `Subtype`, `CounterKind`, `TurnPart` — carry the serde
/// name of a v2 syntax type, so they need no second kind: a declaration
/// registered under one is already invocable by its bare name wherever a card
/// writes that type. `Type` does NOT: v2's card-type position is `CardType`,
/// so a `Type` declaration is a loader tag until a ticket reconciles the two
/// names. The remaining families are name-erasing loader tags that open no
/// card position at all, exactly as v1's `TypeDef` and `Counter` do.
pub const DECLARATION_KINDS: &[&str] = &[
    "AbilityWord",
    "CounterKind",
    "Designation",
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
    // The mana injection chain (§11.1): each of these carries an
    // `#[macro_ron(embed)]` constructor, and the untagged fall-through is a
    // per-kind fact, so the reader needs every link registered or a bare
    // `Green` stops one hop short.
    kinds.add(crate::words::ManaSymbol::kind());
    kinds.add(crate::words::SimpleManaSymbol::kind());
    kinds.add(crate::words::ColorOrColorless::kind());
    kinds.add(crate::phrase::ColorTerm::kind());
    kinds.add(crate::words::Color::kind());
    // A word type, not a `semantic_expression`, but macroable all the same:
    // `plugins_v2`'s turn-part declarations register at this very name, so the
    // kind must be the derived one that carries the type's dispatch set. A
    // hand-built `Kind::new("TurnPart")` registers the same position with an
    // empty variant list, and `macro_ron`'s cycle check then reads a
    // declaration named `Upkeep` whose body is `Upkeep` as a self-reference
    // rather than as the identity macro it is.
    kinds.add(crate::words::TurnPart::kind());
    // Not `semantic_expression`s either, but Vote's declaration signature
    // (`params: [Disclosure, Ballot]`) needs a registered kind to name each.
    kinds.add(crate::words::Disclosure::kind());
    kinds.add(crate::phrase::Ballot::kind());
    // `Delta` is generic in Lean (`Delta (α : Type)`) and generic in Rust, and
    // `#[derive(SupportsMacros)]` rejects generics — so its kind is
    // hand-built and carries no dispatch set. Registration is what a macro of
    // kind `Delta` needs; the checks that consult the dispatch set (the cycle
    // check, the restricted-read ban) lose precision here and nothing else.
    kinds.add(Kind::new("Delta"));
    // Positions the helper macro layer expands to that are not
    // `semantic_expression` types: a token's characteristic bundle, a joined
    // trigger header's possessor, and the two card-frame records. Each is a
    // hand-built kind for the same reason `Delta` is — the position needs
    // registering, and the dispatch set a derive would carry is either
    // unavailable (a struct) or unused here.
    kinds.add(Kind::new("CharacteristicBundle"));
    kinds.add(Kind::new("HeaderPossessor"));
    kinds.add(Kind::new("LevelBand"));
    kinds.add(Kind::new("PrototypeFrame"));
    // Declaration positions: `Macro` is where a meta-macro's product is read
    // (`macros/meta/`), and each declaration kind is where the declarations it
    // produces register.
    kinds.add(Kind::new(MACRO_KIND));
    for name in DECLARATION_KINDS {
        // A family whose name coincides with a syntax type is already
        // registered above, from the type's own derive; re-adding it would
        // replace that kind with a variant-less hand-built one.
        if !kinds.contains(name) {
            kinds.add(Kind::new(name));
        }
    }
    carve_out_the_non_expression_kinds(&kinds)
}

/// The types Lean tags `semantic_expression` — the ones whose constructors a
/// card may not write (§11.1's macro-only rule, Lean's
/// `Authoring.Form.onlyMacros`).
///
/// Every OTHER registered kind is a word type or a loader tag, and its
/// variants stay natively spellable: `White` at a `Color` position is the
/// colour, not a macro that has gone missing, exactly as Lean's `onlyMacros`
/// only refuses a `semantic_expression` constructor.
pub const EXPRESSION_KINDS: &[&str] = &[
    "Ability",
    "Amount",
    "Condition",
    "Cost",
    "Delta",
    "Duration",
    "GameEvent",
    "Instruction",
    "NounPhrase",
    "Predicate",
    "Quantity",
    "StaticSpec",
    "Timing",
    "TokenSpec",
    "UsageLimit",
    "Window",
    "ZoneExpr",
];

/// The variants of a HAND-BUILT kind that stay natively spellable under a
/// restricted read.
///
/// A kind registered from `#[derive(SupportsMacros)]` carries its own dispatch
/// set, so the carve-out below reads the variants off the kind. A hand-built
/// kind carries none — `Subtype` and `CounterKind` are registered because a
/// declaration family bears their name, not because the type derives — so the
/// variant names are written here. Both are word types, not
/// `semantic_expression`s, so a card writes them as it always did.
const HAND_BUILT_NATIVE: &[(&str, &[&str])] = &[
    ("Subtype", &["Of", "Spell"]),
    ("CounterKind", &["Boost", "Keyword", "Named"]),
    ("HeaderPossessor", &["NoPossessor", "ByPlayer", "ByTurn"]),
];

/// The literal leaves (§11.1): Lean tags these `semantic_literal`, and
/// `onlyMacros` admits a literal where it refuses a constructor. A card writes
/// the numeral, but the written-out form stays legal at the leaf.
const LITERAL_LEAVES: &[(&str, &str)] = &[("Amount", "Lit"), ("SimpleManaSymbol", "Generic")];

/// Marks every non-`semantic_expression` kind's variants natively spellable,
/// so a restricted read ([`macro_ron::MacroSet::read_str_restricted`], which
/// is how a card is read) suppresses exactly the constructors Lean's
/// `onlyMacros` refuses and no others.
fn carve_out_the_non_expression_kinds(kinds: &KindSet) -> KindSet {
    let mut out = KindSet::new();
    for kind in kinds.iter().cloned() {
        let variants = kind.variants();
        let name = kind.name().to_owned();
        let kind = if EXPRESSION_KINDS.contains(&name.as_str()) {
            let leaves: Vec<&'static str> = LITERAL_LEAVES
                .iter()
                .filter(|(kind, _)| *kind == name)
                .map(|(_, variant)| *variant)
                .collect();
            kind.natively_spellable(leaves)
        } else {
            let hand_built = HAND_BUILT_NATIVE
                .iter()
                .filter(|(kind, _)| *kind == name)
                .flat_map(|(_, variants)| variants.iter().copied());
            kind.natively_spellable(variants.iter().copied())
                .natively_spellable(hand_built)
        };
        out.add(kind);
    }
    out
}

/// The param types a declaration's signature may name, each validated by
/// reading the argument as the v2 type it stands for.
///
/// The names are the ones `plugins_v2/builtin`'s declarations already write
/// (`params: [Cost]`, `params: [Amount, Cost]`): a keyword one-liner's typed
/// argument vocabulary. `Power` and `Toughness` are `Amount` under two names,
/// because a declaration says which stat its argument fills; `Quality` is a
/// `Predicate` and `Subject` a `NounPhrase`, the two ways a declaration takes
/// a description. `Any` and `String` come from `macro_ron` itself.
///
/// This is NOT Lean's `MacroParameters`, which is a proof device (§12); it is
/// the argument vocabulary the declaration files write, and a name absent from
/// here makes its declaration unregistrable.
#[must_use]
pub fn param_types() -> ParamTypeSet {
    let mut types = ParamTypeSet::default();
    types.add_typed::<crate::abilities::Cost>("Cost");
    types.add_typed::<crate::abilities::Ability>("Ability");
    types.add_typed::<crate::phrase::Amount>("Amount");
    types.add_typed::<crate::phrase::Amount>("Power");
    types.add_typed::<crate::phrase::Amount>("Toughness");
    types.add_typed::<crate::phrase::Predicate>("Quality");
    types.add_typed::<crate::phrase::Predicate>("Predicate");
    types.add_typed::<crate::phrase::NounPhrase>("Subject");
    types.add_typed::<crate::phrase::NounPhrase>("NounPhrase");
    types.add_typed::<crate::phrase::Condition>("Condition");
    // Every remaining `SupportsMacros` kind, under its own name: §12 "every
    // `SupportsMacros` kind is a parameter type," mirroring how `kinds()`
    // above is total over the same derive (`every_supports_macros_type_is_a_kind`).
    // A body needing a `Subtype`, `ZoneExpr`, `Quantity`, `TokenSpec`, or
    // `Instruction` argument (Amass, Create, Meld, Search, Face A Villainous
    // Choice) had no way to declare it before this.
    types.add_typed::<crate::phrase::GameEvent>("GameEvent");
    types.add_typed::<crate::phrase::Quantity>("Quantity");
    types.add_typed::<crate::phrase::ZoneExpr>("ZoneExpr");
    types.add_typed::<crate::triggers::Duration>("Duration");
    types.add_typed::<crate::triggers::Timing>("Timing");
    types.add_typed::<crate::triggers::UsageLimit>("UsageLimit");
    types.add_typed::<crate::abilities::Instruction>("Instruction");
    types.add_typed::<crate::abilities::StaticSpec>("StaticSpec");
    types.add_typed::<crate::abilities::TokenSpec>("TokenSpec");
    types.add_typed::<crate::words::Window>("Window");
    types.add_typed::<crate::words::TurnPart>("TurnPart");
    types.add_typed::<crate::words::Disclosure>("Disclosure");
    types.add_typed::<crate::phrase::Ballot>("Ballot");
    types.add_typed::<crate::words::ManaSymbol>("ManaSymbol");
    types.add_typed::<crate::words::SimpleManaSymbol>("SimpleManaSymbol");
    types.add_typed::<crate::words::ColorOrColorless>("ColorOrColorless");
    types.add_typed::<crate::phrase::ColorTerm>("ColorTerm");
    types.add_typed::<crate::words::Color>("Color");
    // Not a `SupportsMacros` kind (it has no macro dispatch of its own —
    // `plugins_v2` spells a subtype with the native `Of`/`Spell` constructor,
    // never a bare declared-subtype macro name), but a real v2 syntax type a
    // declaration's signature needs to name: Amass's amassed subtype
    // [CR#701.47a].
    types.add_typed::<crate::words::Subtype>("Subtype");
    // Likewise no dispatch set of its own (`plugins_v2` never writes a bare
    // `SearchScope` macro), but Search's declaration signature
    // (`search (scope : SearchScope) (quantity : Quantity) …`) needs to name
    // it [CR#701.23a].
    types.add_typed::<crate::phrase::SearchScope>("SearchScope");
    // The helper macro layer's argument vocabulary (§12): a signature ported
    // from `lean/Semantics/Macros.lean` names the Lean binder's own type, so
    // every mirror type a helper takes registers here. A list-typed parameter
    // registers under the PLURAL of its element type, v1's convention
    // (`Abilities` for `Vec<Ability>`); a Lean `abbrev` registers under the
    // alias's own name, since that is what the signature writes. `Nat` is
    // Lean's, and has no mirror type of its own.
    types.add_typed::<Vec<crate::abilities::Ability>>("Abilities");
    types.add_typed::<String>("AbilityWordLabel");
    types.add_typed::<crate::words::AggregateOp>("AggregateOp");
    types.add_typed::<crate::words::Arrangement>("Arrangement");
    types.add_typed::<crate::abilities::AsThough>("AsThough");
    types.add_typed::<crate::abilities::CharacteristicBundle>("CharacteristicBundle");
    types.add_typed::<crate::phrase::ChoiceDomain>("ChoiceDomain");
    types.add_typed::<Vec<crate::words::Color>>("Colors");
    types.add_typed::<crate::words::Comparator>("Comparator");
    types.add_typed::<crate::abilities::Compulsion>("Compulsion");
    types.add_typed::<crate::triggers::Concurrent>("Concurrent");
    types.add_typed::<crate::events::CounterBatch>("CounterBatch");
    types.add_typed::<crate::words::CounterKind>("CounterKind");
    types.add_typed::<crate::abilities::CounterKindSource>("CounterKindSource");
    types.add_typed::<crate::events::CounterMove>("CounterMove");
    types.add_typed::<crate::events::DamageKind>("DamageKind");
    types.add_typed::<crate::abilities::DamageScope>("DamageScope");
    types.add_typed::<crate::abilities::DeckCondition>("DeckCondition");
    types.add_typed::<crate::words::Deed>("Deed");
    types.add_typed::<Vec<crate::words::Deed>>("Deeds");
    types.add_typed::<crate::words::Delta<crate::phrase::Amount>>("Delta");
    types.add_typed::<crate::abilities::DeonticPatient>("DeonticPatient");
    types.add_typed::<crate::abilities::DeonticRider>("DeonticRider");
    types.add_typed::<String>("DesignationLabel");
    types.add_typed::<String>("FlavorWordLabel");
    types.add_typed::<Vec<crate::phrase::GameEvent>>("GameEvents");
    types.add_typed::<Vec<crate::triggers::JoinedHeader>>("JoinedHeaders");
    types.add_typed::<String>("KeywordLabel");
    types.add_typed::<crate::words::Kind>("Kind");
    types.add_typed::<crate::card::LevelRange>("LevelRange");
    types.add_typed::<crate::words::Lookback>("Lookback");
    types.add_typed::<Vec<crate::words::ManaSymbol>>("ManaCost");
    types.add_typed::<crate::words::ManaMatch>("ManaMatch");
    types.add_typed::<u32>("Nat");
    types.add_typed::<crate::words::NounWord>("NounWord");
    types.add_typed::<crate::words::Ordinal>("Ordinal");
    types.add_typed::<crate::words::PaidCostName>("PaidCostName");
    types.add_typed::<crate::words::PartQuant>("PartQuant");
    types.add_typed::<crate::words::Plurality>("Plurality");
    types.add_typed::<crate::words::ProjAxis>("ProjAxis");
    types.add_typed::<crate::words::QualitySort>("QualitySort");
    types.add_typed::<crate::words::RankPeriod>("RankPeriod");
    types.add_typed::<crate::words::Role>("Role");
    types.add_typed::<crate::abilities::SpendPurpose>("SpendPurpose");
    types.add_typed::<crate::words::Stat>("Stat");
    types.add_typed::<Vec<crate::abilities::StaticSpec>>("StaticSpecs");
    types.add_typed::<Vec<crate::words::Subtype>>("Subtypes");
    types.add_typed::<Vec<crate::abilities::TokenRider>>("TokenRiders");
    types.add_typed::<crate::words::VerbedMarking>("VerbedMarking");
    types
}

/// The raw `ron::Options` everything is read and written with.
///
/// `implicit_some` keeps `Option` fields flat and `unwrap_variant_newtypes`
/// lets a struct-carrying variant read flat (`HasType(type: Creature)` rather
/// than `HasType((type: Creature))`) — which is what makes a mirrored
/// constructor spell like its Lean original.
///
/// `unwrap_newtypes` is what makes an injection WRITE bare: a
/// `#[macro_ron(embed)]` struct variant serializes its payload through a
/// newtype struct named `Type.Variant`, which this extension drops, so
/// `ManaSymbol::Simple { … }` writes `Green`. The mirror declares no newtype
/// structs of its own, so the extension reaches nothing else.
#[must_use]
pub fn raw_options() -> ::ron::Options {
    ::ron::Options::default().with_default_extension(
        ::ron::extensions::Extensions::IMPLICIT_SOME
            | ::ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES
            | ::ron::extensions::Extensions::UNWRAP_NEWTYPES,
    )
}

/// An empty macro set over [`kinds`] reading [`raw_options`] — the base every
/// plugin load starts from before its `macros/` directory is folded in.
///
/// The set refuses unknown fields: a key a constructor does not declare is an
/// error naming both, never a silently skipped one. serde's default — skip
/// what you don't recognize — read a misspelling as an omission, so Fading and
/// Impending wrote `amount:` at a `quantity` field and got a count-less
/// removal, and `conferral:` arguments left over from a retired signature
/// passed unnoticed (§11).
///
/// It also reads constructors and named-signature macros applied positionally,
/// in their declared binder or parameter order — `Hybrid(Generic(1), Red)` or
/// `hasType(Creature)` — which is how the Lean bench writes applications.
#[must_use]
pub fn macro_set() -> MacroSet {
    MacroSet::new(kinds())
        .with_options(raw_options())
        .with_param_types(param_types())
        .denying_unknown_fields()
        .reading_positional_arguments()
}

#[cfg(test)]
mod tests {
    use super::kinds;
    use super::param_types;

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

    /// The names of the types this crate derives `SupportsMacros` on, read
    /// from its own sources.
    ///
    /// The mapping "one kind per `SupportsMacros` enum"
    /// (`docs/decisions/semantics-v2.md` §12) has to stay total as the mirror
    /// grows, and a hand-maintained list beside [`kinds`] would only restate
    /// it. The derive attribute is the fact, so the test reads the fact.
    fn supports_macros_types() -> Vec<String> {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&src)
            .expect("the crate's own `src` is readable")
            .map(|entry| entry.expect("a readable directory entry").path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
            .collect();
        files.sort();
        let mut names = Vec::new();
        for file in files {
            let text = std::fs::read_to_string(&file).expect("a readable source file");
            let mut lines = text.lines();
            while let Some(line) = lines.next() {
                let trimmed = line.trim_start();
                if !trimmed.starts_with("#[derive(") || !trimmed.contains("SupportsMacros") {
                    continue;
                }
                let declaration = lines
                    .by_ref()
                    .find(|line| {
                        let trimmed = line.trim_start();
                        trimmed.starts_with("pub enum ") || trimmed.starts_with("pub struct ")
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "a `SupportsMacros` derive in {} names no type",
                            file.display()
                        )
                    });
                let name = declaration
                    .trim_start()
                    .trim_start_matches("pub enum ")
                    .trim_start_matches("pub struct ")
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .expect("a type name follows `pub enum`/`pub struct`");
                names.push(name.to_owned());
            }
        }
        names
    }

    /// Every `SupportsMacros` type this crate declares is a registered kind:
    /// that is what "kinds are one per `SupportsMacros` enum" (§12) means, and
    /// a type that derives the trait without being registered is a position
    /// macros silently cannot occupy.
    #[test]
    fn every_supports_macros_type_is_a_kind() {
        let kinds = kinds();
        let types = supports_macros_types();
        assert!(
            types.len() >= 17,
            "only {} `SupportsMacros` derives found; the scan lost the sources it reads",
            types.len()
        );
        for name in &types {
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

    /// Every param type the builtin declarations name is registered, or the
    /// declaration naming it is unregistrable.
    #[test]
    fn every_declared_param_type_is_registered() {
        let types = super::param_types();
        for name in [
            "Any",
            "String",
            "Cost",
            "Ability",
            "Amount",
            "Power",
            "Toughness",
            "Quality",
            "Subject",
            "Condition",
        ] {
            assert!(
                types.contains(name),
                "`{name}` must be a registered param type"
            );
        }
    }

    /// Every `SupportsMacros` type this crate declares is also a registered
    /// param type: §12 "every `SupportsMacros` kind is a parameter type,"
    /// the same totality [`every_supports_macros_type_is_a_kind`] proves for
    /// `kinds()`. A type absent here makes a declaration needing it as an
    /// argument unregistrable (the crate gap `Amass`, `Create`, `Meld`,
    /// `Search` and `Face A Villainous Choice` hit).
    #[test]
    fn every_supports_macros_type_is_a_param_type() {
        let types = param_types();
        for name in supports_macros_types() {
            assert!(
                types.contains(&name),
                "`{name}` must be a registered param type"
            );
        }
    }

    /// A derived kind carries its dispatch set, or the checks that consult it
    /// (the cycle check, the restricted-read ban) silently weaken.
    #[test]
    fn derived_kinds_carry_their_dispatch_set() {
        let kinds = kinds();
        for name in [
            "Instruction",
            "Predicate",
            "NounPhrase",
            "Ability",
            "TurnPart",
        ] {
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
