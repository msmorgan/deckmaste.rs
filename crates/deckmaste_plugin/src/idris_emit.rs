//! Re-emit an EXPANDED semantic [`Card`] (i.e. after `Expand::expand_all`) as
//! an equivalent RAW `idris/src/Semantics.idr` `Card` expression — sugar-free,
//! using the mirror's real constructors (never the `^`/`^:` builder sugar,
//! never `Macros.idr` templates). Typechecking the emitted expression with
//! `idris2 --check` is the anaphora-soundness gate: Idris's dependent
//! `Normal`/`Reference`/`Selection` proofs make an unsound card (a dangling
//! `It`/`That`, an ambiguous antecedent, …) unrepresentable, so a card that
//! typechecks is sound by construction.
//!
//! The input is the SEMANTIC term, not its engine image
//! (`docs/decisions/semantics-spelling-lowering.md` §10): the gate certifies
//! semantic input, so the mirror models the semantics kernel — the
//! post-expansion, post-desugar normal form — and never depends on lowering.
//!
//! This module is a plain recursive `Card -> Result<String, Gap>` string
//! emitter — no parsing, no macro layer (the input has already gone through
//! `Plugin`+`Expand::expand_all`). Every function returns either a bare Idris
//! ATOM (no internal spaces, safe to splice unparenthesized) or a FULLY
//! PARENTHESIZED compound expression — so callers can always join emitted
//! pieces with a space and wrap once, with no risk of a stray unparenthesized
//! application.
//!
//! `deckmaste_semantics`'s grammar has drifted from `idris/src/Semantics.idr`
//! in real ways since the surface-corrections migration (new `EventFilter`
//! master forms, a `Deontic` family, a `Selection`/`Binder` split, …) — that
//! drift is exactly what this gate is meant to surface. Coverage is
//! intentionally partial: anything not yet mapped returns [`Gap`] rather than
//! guessing, and callers (the `idris-check` xtask command) report gaps as
//! coverage, not failures.
//!
//! Pedantic-lint note: this module is ~60 small `&NodeType -> R` string
//! emitters sharing one calling convention (every AST node arrives by
//! reference, since callers hold the real `Card`/`Ability`/… tree; every
//! emitter returns `Result` uniformly, even the handful that can't currently
//! fail, so growing coverage never has to change a signature). Clippy's
//! pedantic pass flags each of those choices in isolation
//! (`needless_pass_by_value`, `trivially_copy_pass_by_ref`, `ref_option`,
//! `unnecessary_wraps`) — allowed module-wide rather than annotated at each
//! of ~60 call sites for a style trade-off that's intentional and uniform.
#![allow(
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::ref_option,
    clippy::unnecessary_wraps,
    reason = "~60 uniform `&NodeType -> Result` emitters share one calling \
    convention (by-reference args; uniform `Result` return so coverage grows \
    without signature churn); these pedantic lints flag that intentional style \
    at each site — allowed module-wide, see the module note above"
)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_semantics::Ability;
use deckmaste_semantics::Action;
use deckmaste_semantics::Anchor;
use deckmaste_semantics::Arrangement;
use deckmaste_semantics::Card;
use deckmaste_semantics::CardFace;
use deckmaste_semantics::CharacteristicPredicate;
use deckmaste_semantics::Cmp;
use deckmaste_semantics::Color;
use deckmaste_semantics::ColorOrColorless;
use deckmaste_semantics::Condition;
use deckmaste_semantics::CopyException;
use deckmaste_semantics::CopyRetarget;
use deckmaste_semantics::CopySource;
use deckmaste_semantics::CopySpec;
use deckmaste_semantics::Cost;
use deckmaste_semantics::CostChange;
use deckmaste_semantics::CostComponent;
use deckmaste_semantics::Count;
use deckmaste_semantics::CountBound;
use deckmaste_semantics::Countable;
use deckmaste_semantics::CounterSpec;

type SemValue = Count;
use deckmaste_semantics::Deontic;
use deckmaste_semantics::DeonticAction;
use deckmaste_semantics::Destination;
use deckmaste_semantics::EnterRider;
use deckmaste_semantics::EventFilter;
use deckmaste_semantics::Expand;
use deckmaste_semantics::Ident;
use deckmaste_semantics::KeywordAbility;
use deckmaste_semantics::LifeOp;
use deckmaste_semantics::ManaCost;
use deckmaste_semantics::ManaProduction;
use deckmaste_semantics::ManaSpec;
use deckmaste_semantics::ManaSymbol;
use deckmaste_semantics::Modification;
use deckmaste_semantics::Normalize;
use deckmaste_semantics::NumericOp;
use deckmaste_semantics::OneShotEffect;
use deckmaste_semantics::PhaseStep;
use deckmaste_semantics::PlayerAttr;
use deckmaste_semantics::PlayerMod;
use deckmaste_semantics::Predicate;
use deckmaste_semantics::Property;
use deckmaste_semantics::Quantity;
use deckmaste_semantics::Reference;
use deckmaste_semantics::RelationPredicate;
use deckmaste_semantics::RetargetMode;
use deckmaste_semantics::Selection;
use deckmaste_semantics::SimpleManaSymbol;
use deckmaste_semantics::Sort;
use deckmaste_semantics::StatValue;
use deckmaste_semantics::StatePredicate;
use deckmaste_semantics::StaticEffect;
use deckmaste_semantics::Subtype;
use deckmaste_semantics::Supertype;
use deckmaste_semantics::SymbolPred;
use deckmaste_semantics::TargetSpec;
use deckmaste_semantics::Token;
use deckmaste_semantics::TokenSpec;
use deckmaste_semantics::Type;

use crate::plugin::Plugin;

/// A Rust grammar shape this emitter doesn't (yet) translate to Idris —
/// either a genuine expressiveness gap between the two grammars (report
/// honestly), or simply not implemented yet. Carries a short description of
/// what was hit and why.
#[derive(Debug, Clone, thiserror::Error)]
#[error("{0}")]
pub struct Gap(pub String);

type R = Result<String, Gap>;

fn gap(msg: impl Into<String>) -> Gap {
    Gap(msg.into())
}

/// Build `(head a1 a2 …)`, or the bare `head` when `args` is empty (a
/// nullary/unit constructor never needs parens).
fn app(head: &str, args: Arc<[String]>) -> String {
    if args.is_empty() {
        head.to_string()
    } else {
        format!("({} {})", head, args.join(" "))
    }
}

/// An Idris list literal `[a, b, c]` — self-delimited, safe to splice as an
/// argument with no extra parens.
fn ilist(items: Arc<[String]>) -> String {
    format!("[{}]", items.join(", "))
}

/// An Idris string literal. Rust's `Debug` escaping for `str` coincides with
/// Idris's for the common (ASCII, no exotic control chars) case — good enough
/// for card names / labels / tags.
fn ilit(s: &str) -> String {
    format!("{s:?}")
}

fn try_maybe<T>(o: &Option<T>, f: impl Fn(&T) -> R) -> Result<String, Gap> {
    match o {
        None => Ok("Nothing".to_string()),
        Some(v) => Ok(format!("(Just {})", f(v)?)),
    }
}

fn map_list<T>(items: &[T], f: impl Fn(&T) -> R) -> Result<String, Gap> {
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(f(item)?);
    }
    Ok(ilist(out.into()))
}

// ===========================================================================
// Leaf enums: Color / mana / Type / Supertype / Subtype / Zone / phase steps
// ===========================================================================

fn emit_color(c: Color) -> String {
    match c {
        Color::White => "White",
        Color::Blue => "Blue",
        Color::Black => "Black",
        Color::Red => "Red",
        Color::Green => "Green",
    }
    .to_string()
}

/// A [`SymbolPred`] as the Idris matcher algebra ([CR#700.5] devotion
/// counting). `AnyColor` has no single Idris constructor — it emits as the
/// disjunction over the five colors; `AnyType` (colorless included) has no
/// devotion twin at all, so it gaps.
fn emit_symbol_pred(p: &SymbolPred) -> R {
    Ok(match p {
        SymbolPred::CountsAs(c) => app("CountsAs", vec![emit_color(*c)].into()),
        SymbolPred::IsGeneric => "IsGeneric".to_string(),
        SymbolPred::And(ps) => app("And", vec![map_list(ps, emit_symbol_pred)?].into()),
        SymbolPred::Or(ps) => app("Or", vec![map_list(ps, emit_symbol_pred)?].into()),
        SymbolPred::Not(inner) => app("Not", vec![emit_symbol_pred(inner)?].into()),
        SymbolPred::AnyColor => app(
            "Or",
            vec![ilist(
                vec![
                    app("CountsAs", vec!["White".to_string()].into()),
                    app("CountsAs", vec!["Blue".to_string()].into()),
                    app("CountsAs", vec!["Black".to_string()].into()),
                    app("CountsAs", vec!["Red".to_string()].into()),
                    app("CountsAs", vec!["Green".to_string()].into()),
                ]
                .into(),
            )]
            .into(),
        ),
        SymbolPred::AnyType => {
            return Err(gap(
                "SymbolPred::AnyType (colorless included) has no devotion twin in Idris",
            ));
        }
    })
}

fn emit_color_or_colorless(c: ColorOrColorless) -> String {
    match c {
        ColorOrColorless::Colorless => "Nothing".to_string(),
        ColorOrColorless::Color(c) => format!("(Just {})", emit_color(c)),
    }
}

fn emit_simple_mana_symbol(s: &SimpleManaSymbol) -> String {
    match s {
        SimpleManaSymbol::Generic(n) => app("Generic", vec![n.to_string()].into()),
        SimpleManaSymbol::Specific(c) => app("Specific", vec![emit_color_or_colorless(*c)].into()),
    }
}

fn emit_mana_symbol(m: &ManaSymbol) -> String {
    match m {
        ManaSymbol::Variable => "Variable".to_string(),
        ManaSymbol::Snow => "SnowMana".to_string(),
        ManaSymbol::Hybrid(s, c) => app(
            "Hybrid",
            vec![emit_simple_mana_symbol(s), emit_color(*c)].into(),
        ),
        ManaSymbol::Phyrexian(c, mc) => app(
            "Phyrexian",
            vec![
                emit_color(*c),
                match mc {
                    None => "Nothing".to_string(),
                    Some(c2) => format!("(Just {})", emit_color(*c2)),
                },
            ]
            .into(),
        ),
        ManaSymbol::Simple(s) => app("Simple", vec![emit_simple_mana_symbol(s)].into()),
    }
}

fn emit_mana_cost(cost: &ManaCost) -> String {
    ilist(cost.iter().map(emit_mana_symbol).collect())
}

/// Map a canonical type NAME to its Idris `Type_` variant, gapping unknowns
/// (Dungeon, any novel open type) — the closed grammar is untouched. Card types
/// are open plugin data (`TypeDef`), so this keys on the name string; a name
/// with no `Type_` counterpart gaps exactly as `Dungeon` did.
pub(crate) fn emit_type_name(name: &str) -> R {
    Ok(match name {
        "Artifact" => "Artifact",
        "Battle" => "Battle",
        "Creature" => "Creature",
        "Enchantment" => "Enchantment",
        "Instant" => "Instant",
        "Kindred" => "Kindred",
        "Land" => "Land",
        "Planeswalker" => "Planeswalker",
        "Sorcery" => "Sorcery",
        other => {
            return Err(gap(format!(
                "type `{other}` has no Idris Type_ counterpart"
            )));
        }
    }
    .to_string())
}

fn emit_supertype(s: Supertype) -> String {
    match s {
        Supertype::Basic => "Basic",
        Supertype::Legendary => "Legendary",
        Supertype::Ongoing => "Ongoing",
        Supertype::Snow => "Snow",
        Supertype::World => "World",
    }
    .to_string()
}

/// The Idris `Category` constructor for a subtype, DERIVED from the open Rust
/// `Subtype.types` rather than a hand-mirrored `name -> constructor` table:
/// creature/kindred types → `Creature`, instant/sorcery → `Spell`, the rest
/// their own category. Because the category falls out of the data, any subtype
/// the macro layer declares emits with no per-name parity edit. `None` only
/// when `types` names no card type Idris models (e.g. `Dungeon`) — a genuine
/// coverage gap, not a bug.
fn category_idris(types: &[Type]) -> Option<&'static str> {
    if types
        .iter()
        .any(|t| matches!(t, Type::Creature | Type::Kindred))
    {
        Some("Creature")
    } else if types
        .iter()
        .any(|t| matches!(t, Type::Instant | Type::Sorcery))
    {
        Some("Spell")
    } else if types.contains(&Type::Enchantment) {
        Some("Enchantment")
    } else if types.contains(&Type::Artifact) {
        Some("Artifact")
    } else if types.contains(&Type::Land) {
        Some("Land")
    } else if types.contains(&Type::Battle) {
        Some("Battle")
    } else if types.contains(&Type::Planeswalker) {
        Some("Planeswalker")
    } else {
        None
    }
}

/// One subtype conferral, emitted into the Idris `Subtype`'s confers field.
/// Idris has no `Property` type — "a conferral IS an ability" — so the
/// `Ability` flavor (the only one any subtype uses today: the
/// Aura/Equipment/Fortification static rules, the Saga replacement, a basic
/// land's mana ability) emits as its inner ability. The other `Property`
/// flavors do not occur on subtypes; they gap if one ever appears.
fn emit_property(p: &Property) -> R {
    match p {
        Property::Ability(a) => emit_ability(a),
        Property::Continuous(..) => Err(gap("subtype confer Property::Continuous not mapped")),
        Property::StateBased { .. } => Err(gap("subtype confer Property::StateBased not mapped")),
        Property::TurnBased { .. } => Err(gap("subtype confer Property::TurnBased not mapped")),
    }
}

/// Emit an open `Subtype` value — `(MkSubtype <Category> "<name>" [<confers>])`
/// — with the category derived from `types` and the conferred abilities sourced
/// from the RON `confers` list. Used for a card's OWN subtypes, where the full
/// value is in hand (a name-only reference goes through [`emit_subtype_ref`]).
fn emit_subtype(s: &Subtype) -> R {
    let cat = category_idris(&s.types).ok_or_else(|| {
        gap(format!(
            "subtype {} has no Idris category (types: {:?})",
            s.name.as_str(),
            s.types
        ))
    })?;
    let confers = map_list(&s.confers, emit_property)?;
    Ok(format!("(MkSubtype {cat} {:?} {confers})", s.name.as_str()))
}

thread_local! {
    /// Subtype name -> its Idris `Category` constructor, rebuilt from the loaded
    /// plugin's subtype registry at each [`emit_card_expr`] entry. A subtype
    /// REFERENCE (a filter, or an `Alter Subtypes` op) names a subtype without
    /// carrying its `types`, so it resolves the category here — from the same
    /// open registry the macro layer declares — instead of a hardcoded table.
    static SUBTYPE_CATEGORY: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());
}

/// The card type a macro-registry row names, re-read as the semantic one.
///
/// The three `load_*` registry projections below are the ONLY place this
/// emitter touches [`deckmaste_core`]: a [`Plugin`]'s subtype/counter/
/// designation registries are the macro layer's DECLARATIONS, which the mirror
/// excludes (spec §10) and which stay engine-typed. [`Type`] is a fieldless
/// enum with the same variants in both grammars, so the row's card-type list
/// re-reads exactly for [`category_idris`], which the kernel path shares.
fn registry_type(t: deckmaste_core::Type) -> Type {
    match t {
        deckmaste_core::Type::Artifact => Type::Artifact,
        deckmaste_core::Type::Battle => Type::Battle,
        deckmaste_core::Type::Creature => Type::Creature,
        deckmaste_core::Type::Dungeon => Type::Dungeon,
        deckmaste_core::Type::Enchantment => Type::Enchantment,
        deckmaste_core::Type::Instant => Type::Instant,
        deckmaste_core::Type::Kindred => Type::Kindred,
        deckmaste_core::Type::Land => Type::Land,
        deckmaste_core::Type::Planeswalker => Type::Planeswalker,
        deckmaste_core::Type::Sorcery => Type::Sorcery,
    }
}

/// Populate [`SUBTYPE_CATEGORY`] from a plugin's subtype registry. Called once
/// per card at emit entry; subtypes whose `types` map to no category are simply
/// absent (a reference to one then gaps, like any other coverage gap).
fn load_subtype_categories<S>(subtypes: &HashMap<Ident, deckmaste_core::Subtype, S>) {
    SUBTYPE_CATEGORY.with(|m| {
        let mut m = m.borrow_mut();
        m.clear();
        for (name, sub) in subtypes {
            let types: Vec<Type> = sub.types.iter().copied().map(registry_type).collect();
            if let Some(cat) = category_idris(&types) {
                m.insert(name.as_str().to_string(), cat);
            }
        }
    });
}

/// Emit a subtype REFERENCE by name (filter / `Alter Subtypes` position). The
/// referent is an IDENTITY — `(MkSubtype <Category> "<name>" [])`, no confers
/// (a filter asks "is it an Aura?", not what an Aura confers) — with the
/// category resolved from the registry loaded at emit entry.
fn emit_subtype_ref(name: &str) -> R {
    let cat = SUBTYPE_CATEGORY
        .with(|m| m.borrow().get(name).copied())
        .ok_or_else(|| gap(format!("unmapped subtype in reference: {name}")))?;
    Ok(format!("(MkSubtype {cat} {name:?} [])"))
}

/// The Idris `Scope` token for a counter's carrier scope — `Object`/`Player`,
/// the two members of the open `CounterKind`/`Designation` value's scope index
/// (both map into `RefKind` via `scopeRef`).
fn scope_idris(scope: &deckmaste_core::CounterScope) -> &'static str {
    match scope {
        deckmaste_core::CounterScope::Object => "Object",
        deckmaste_core::CounterScope::Player => "Player",
    }
}

/// The Idris `Scope` token for a designation's scope. `DesignationScope::Game`
/// has no `RefKind`/`Scope` analogue (nothing in Idris is game-scoped, and no
/// macro registry row uses it), so it gaps (`None`).
fn designation_scope_idris(scope: &deckmaste_core::DesignationScope) -> Option<&'static str> {
    match scope {
        deckmaste_core::DesignationScope::Object => Some("Object"),
        deckmaste_core::DesignationScope::Player => Some("Player"),
        deckmaste_core::DesignationScope::Game => None,
    }
}

thread_local! {
    /// Counter name -> its Idris `Scope` token (`"Object"`/`"Player"`), rebuilt
    /// from the loaded plugin's counter registry at each [`emit_card_expr`]
    /// entry. A counter REFERENCE names a counter by identity only; its scope
    /// (a dependent type index on the reference-taking constructors) resolves
    /// here, from the same open registry the macro layer declares.
    static COUNTER_SCOPE: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());

    /// Designation name -> its Idris `Scope` token, mirror of [`COUNTER_SCOPE`]
    /// for designation references.
    static DESIGNATION_SCOPE: RefCell<HashMap<String, &'static str>> = RefCell::new(HashMap::new());
}

/// Populate [`COUNTER_SCOPE`] from a plugin's counter registry. Called once per
/// card at emit entry.
fn load_counter_scopes<S>(counters: &HashMap<Ident, deckmaste_core::Counter, S>) {
    COUNTER_SCOPE.with(|m| {
        let mut m = m.borrow_mut();
        m.clear();
        for counter in counters.values() {
            m.insert(
                counter.name.as_str().to_string(),
                scope_idris(&counter.scope),
            );
        }
    });
}

/// Populate [`DESIGNATION_SCOPE`] from a plugin's designation registry. A
/// `Stored` designation carries its scope; `Derived`/`DerivedIf` designations
/// carry none, so default to object scope. Game-scoped designations have no
/// Idris `Scope` and are simply absent (a reference then gaps).
fn load_designation_scopes<S>(designations: &HashMap<Ident, deckmaste_core::DesignationDecl, S>) {
    DESIGNATION_SCOPE.with(|m| {
        let mut m = m.borrow_mut();
        m.clear();
        for decl in designations.values() {
            let scope = match &decl.definition {
                deckmaste_core::DesignationDef::Stored { scope, .. } => {
                    let Some(scope) = designation_scope_idris(scope) else {
                        continue;
                    };
                    scope
                }
                deckmaste_core::DesignationDef::Derived(_)
                | deckmaste_core::DesignationDef::DerivedIf(_) => "Object",
            };
            m.insert(decl.name.as_str().to_string(), scope);
        }
    });
}

/// Emit a counter REFERENCE by name — `(MkCounterKind <Scope> "<name>" [])`, no
/// confers (a reference is an identity: "how many P1P1 counters?", not what
/// they confer) — with the scope resolved from the registry loaded at emit
/// entry. Gaps if the name isn't in the loaded counter registry.
fn counter_ref_idris(name: &str) -> R {
    let scope = COUNTER_SCOPE
        .with(|m| m.borrow().get(name).copied())
        .ok_or_else(|| gap(format!("counter not in registry: {name}")))?;
    Ok(format!("(MkCounterKind {scope} {name:?} [])"))
}

/// The keyword NAME (`Ability::Keyword`'s `KeywordAbility::Composite.name`,
/// or one of the 5 Rust-intrinsic bare variants) -> a Idris `KeywordSpec`
/// expression. Parameterized specs (`Hexproof`/`Protection`/`Banding`) are
/// emitted with a conservative default payload (`Nothing`/no source
/// restriction) since the concrete parameter doesn't survive Rust's
/// `Composite{name, abilities}` shape (only the DESUGARED abilities do,
/// which are still emitted faithfully) — a cosmetic approximation of the
/// TAG, not of the mechanics.
fn keywordspec_idris(name: &str) -> Option<String> {
    Some(
        match name {
            "Flying" => "Flying",
            "FirstStrike" => "FirstStrike",
            "DoubleStrike" => "DoubleStrike",
            "Deathtouch" => "Deathtouch",
            "Reach" => "Reach",
            "Trample" => "Trample",
            "Vigilance" => "Vigilance",
            "Flash" => "Flash",
            "Haste" => "Haste",
            "Indestructible" => "Indestructible",
            "Defender" => "Defender",
            "Shroud" => "Shroud",
            "Menace" => "Menace",
            "Hexproof" => "(Hexproof Nothing)",
            "Morph" => "Morph",
            "Flashback" => "Flashback",
            "Dash" => "Dash",
            "Evoke" => "Evoke",
            "Blitz" => "Blitz",
            "Prowl" => "Prowl",
            "Spectacle" => "Spectacle",
            "Devoid" => "Devoid",
            "Mutate" => "Mutate",
            _ => return None,
        }
        .to_string(),
    )
}

// ===========================================================================
// StatValue -> Count
// ===========================================================================

fn emit_stat_value(v: &StatValue) -> R {
    match v {
        StatValue::Number(n) => Ok(app("Literal", vec![n.to_string()].into())),
        // The embedded amount language maps straight onto the Idris `Count`
        // (`CharValue Power = Count`) — a dynamic base value.
        StatValue::Count(c) => emit_count(c),
        StatValue::DefinedByAbility => Err(gap(
            "StatValue::DefinedByAbility has no Idris Count value (CDA P/T isn't a printed value)",
        )),
        StatValue::Variable => Err(gap(
            "StatValue::Variable (X loyalty) has no Idris Count mapping yet",
        )),
    }
}

// ===========================================================================
// Reference / Sort
// ===========================================================================

fn emit_sort(s: &Sort) -> R {
    Ok(match s {
        Sort::Player => "Player".to_string(),
        Sort::Card => "Card".to_string(),
        Sort::Token => "Token".to_string(),
        Sort::Spell => "Spell".to_string(),
        Sort::StackObject => "StackObject".to_string(),
        Sort::Permanent => "Permanent".to_string(),
        Sort::OfType(t) => app("OfType", vec![emit_type_name(t.name().as_str())?].into()),
        Sort::Amount => "Amount".to_string(),
        Sort::Pile => "Pile".to_string(),
    })
}

#[expect(
    clippy::match_wildcard_for_single_variants,
    reason = "the remaining semantic-only variant is intentionally not a reference-emitter case"
)]
fn emit_reference(r: &Reference) -> R {
    Ok(match r {
        Reference::This => "This".to_string(),
        Reference::Single(selection) => app("Single", vec![emit_selection(selection)?].into()),
        Reference::You => "You".to_string(),
        // No definite "the opponent" Reference in Idris; the closest sound
        // reading is the unique object matching the opponent predicate.
        Reference::Opponent => "(Only OpponentOf)".to_string(),
        Reference::It => "It".to_string(),
        Reference::EventObject => "EventObject".to_string(),
        Reference::EventPatient => "EventPatient".to_string(),
        Reference::EventActor => "EventActor".to_string(),
        Reference::DefendingPlayer => "DefendingPlayer".to_string(),
        Reference::That(sort) => app("That", vec![emit_sort(sort)?].into()),
        // The nth announced target ([CR#115.3,601.2c]).
        Reference::Target(n) => app("Target", vec![n.to_string()].into()),
        Reference::ControllerOf(r) => app("ControllerOf", vec![emit_reference(r)?].into()),
        Reference::Coalesce(rs) => app("Coalesce", vec![map_list(rs, emit_reference)?].into()),
        Reference::OwnerOf(r) => app("OwnerOf", vec![emit_reference(r)?].into()),
        Reference::AttachHostOf(r) => app("AttachHostOf", vec![emit_reference(r)?].into()),
        Reference::Bound(_) => {
            return Err(gap(
                "Reference::Bound (legacy role binding) has no Idris counterpart",
            ));
        }
        Reference::Linked(_) => return Err(gap("Reference::Linked has no Idris counterpart")),
        Reference::Expanded(_) => {
            return Err(gap(
                "unexpanded Reference macro invocation remained after expand_all",
            ));
        }
        other => return Err(gap(format!("Reference {other:?} has no Idris counterpart"))),
    })
}

/// `emit_reference`, specialized for a position where Idris's `Reference b k`
/// is genuinely kind-poly with nothing else to pin `k` (`DealDamage`'s
/// recipient — "any target"). Every OTHER `Reference` constructor already
/// carries its own kind-fixing proof (`It`'s antecedent, `EventPatient`'s
/// cap, `That`'s sort, …) or gets unified from a concretely-kinded consumer
/// elsewhere — `Target n` included, now that a positional read proves its slot
/// ([CR#115.3,601.2c]) and so takes that slot's kind. It needed the
/// `{k = Anything}` `damageTarget` helper only while it was context-free, when
/// a bare `Target n` in a kind-poly position left `k` an unsolved hole; the
/// slot answers that now, and answers it more precisely (Skred's recipient is
/// its creature slot's `AnObject`, not `Anything`).
fn emit_reference_anykind(r: &Reference) -> R {
    emit_reference(r)
}

/// Convert a `Reference` used where Idris wants a `Predicate` (Idris's
/// `Sacrifice`/`ChooseOne`/… bake the choice INTO the predicate rather than
/// pre-resolving it via a binder, unlike Rust's newer split). Every reference
/// becomes `SameAs <ref>` (an already-resolved reference IS a predicate:
/// "equal to r").
fn reference_as_predicate(r: &Reference) -> R {
    Ok(app("SameAs", vec![emit_reference(r)?].into()))
}

// ===========================================================================
// Rust Predicate -> Idris Predicate
// ===========================================================================

fn emit_filter(f: &Predicate) -> R {
    Ok(match f {
        Predicate::Kind(k) => {
            use deckmaste_semantics::ObjectKind as Ok;
            match k {
                Ok::Ability => "(IsKind Ability)".to_string(),
                Ok::Card => "(IsKind Card)".to_string(),
                Ok::Emblem => "(IsKind Emblem)".to_string(),
                Ok::Spell => "(IsKind Spell)".to_string(),
                Ok::Token => "(IsKind Token)".to_string(),
                // Idris `ObjectKind` has no `Player` member (a player test is
                // the top player-predicate, not an object kind).
                Ok::Player => "Anyone".to_string(),
                Ok::CardCopy => {
                    return Err(gap(
                        "ObjectKind::CardCopy has no Idris ObjectKind counterpart",
                    ));
                }
            }
        }
        Predicate::Characteristic(cf) => emit_characteristic_filter(cf)?,
        Predicate::State(sf) => emit_state_filter(sf)?,
        Predicate::Relation(rf) => emit_relation_filter(rf)?,
        Predicate::Ref(r) => app("SameAs", vec![emit_reference(r)?].into()),
        Predicate::Adjacent(a, r) => app(
            "Adjacent",
            vec![emit_adjacency(*a), emit_reference(r)?].into(),
        ),
        // [CR#119.1]: the player-scope stat twin of `StatCmp` — the Idris
        // `PlayerStatCmp` (`emit_player_attr`/`emit_cmp` are the same idiom
        // `Count::PlayerStatOf` uses below).
        Predicate::PlayerStatCmp(attr, cmp, count) => app(
            "PlayerStatCmp",
            vec![emit_player_attr(*attr), emit_cmp(*cmp), emit_count(count)?].into(),
        ),
        Predicate::FromSource(_) => {
            return Err(gap(
                "Predicate::FromSource has no Idris Predicate counterpart",
            ));
        }
        Predicate::And(fs) => app("And", vec![map_list(fs, emit_filter)?].into()),
        Predicate::Or(fs) => app("Or", vec![map_list(fs, emit_filter)?].into()),
        Predicate::Not(inner) => app("Not", vec![emit_filter(inner)?].into()),
        Predicate::Where(cond) => app("Where", vec![emit_condition(cond)?].into()),
        // The vacuous conjunction is universally (if trivially) true at any
        // kind — the one "matches anything" Predicate Idris has.
        Predicate::Any => "(And [])".to_string(),
        Predicate::Expanded(_) => {
            return Err(gap(
                "unexpanded Predicate macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_characteristic_filter(cf: &CharacteristicPredicate) -> R {
    Ok(match cf {
        CharacteristicPredicate::Type(name) => app(
            "HasChar",
            vec!["Types".to_string(), emit_type_name(name.name().as_str())?].into(),
        ),
        CharacteristicPredicate::Subtype(name) => app(
            "HasChar",
            vec![
                "Subtypes".to_string(),
                emit_subtype_ref(name.name().as_str())?,
            ]
            .into(),
        ),
        CharacteristicPredicate::Supertype(s) => app(
            "HasChar",
            vec!["Supertypes".to_string(), emit_supertype(*s)].into(),
        ),
        CharacteristicPredicate::ColorIs(c) => {
            app("HasChar", vec!["Colors".to_string(), emit_color(*c)].into())
        }
        CharacteristicPredicate::Named(name) => app("HasName", vec![ilit(name.as_str())].into()),
        CharacteristicPredicate::Stat(stat, cmp, count) => {
            let characteristic = numeric_characteristic(*stat)?;
            app(
                "StatCmp",
                vec![characteristic, emit_cmp(*cmp), emit_count(count)?].into(),
            )
        }
        CharacteristicPredicate::Multicolored => "Multicolored".to_string(),
        CharacteristicPredicate::Colorless => "IsColorless".to_string(),
        CharacteristicPredicate::Has(kw) => {
            let spec = keywordspec_idris(kw.as_str())
                .ok_or_else(|| gap(format!("unmapped keyword in Has(): {}", kw.as_str())))?;
            app("HasKeyword", vec![spec].into())
        }
    })
}

/// `Stat` restricted to Idris's `Numeric`-gated axes (Power/Toughness/Defense)
/// — `StatCmp`/`TapTotal` demand one of these three.
fn numeric_characteristic(stat: deckmaste_semantics::Stat) -> R {
    use deckmaste_semantics::Stat as S;
    Ok(match stat {
        S::Power => "Power",
        S::Toughness => "Toughness",
        S::Defense => "Defense",
        S::ManaValue => return Err(gap("Stat::ManaValue isn't a Numeric Characteristic in Idris (use ManaValueOf)")),
        S::Loyalty => return Err(gap("Stat::Loyalty isn't a Numeric Characteristic in Idris (loyalty is read via CountersOn Loyalty)")),
    }
    .to_string())
}

fn emit_cmp(c: Cmp) -> String {
    match c {
        Cmp::Eq => "Eq",
        Cmp::AtLeast => "AtLeast",
        Cmp::AtMost => "AtMost",
        Cmp::Greater => "Greater",
        Cmp::Less => "Less",
    }
    .to_string()
}

fn emit_state_filter(sf: &StatePredicate) -> R {
    Ok(match sf {
        StatePredicate::InZone(z) => app("InZone", vec![emit_zone(*z)].into()),
        StatePredicate::Status(status) => {
            use deckmaste_semantics::Status as S;
            match status {
                S::Tapped => "(HasState Tapped)".to_string(),
                S::Untapped => "(HasState Untapped)".to_string(),
                S::PhasedOut => "(HasState PhasedOut)".to_string(),
                S::FaceDown => "(HasState FaceDown)".to_string(),
                S::PhasedIn => "(Not (HasState PhasedOut))".to_string(),
                S::FaceUp => "(Not (HasState FaceDown))".to_string(),
                S::Flipped | S::Unflipped => {
                    return Err(gap(
                        "Status::Flipped/Unflipped has no Idris ObjectState counterpart",
                    ));
                }
            }
        }
        StatePredicate::HasCounter(c) => {
            let k = counter_ref_idris(c.as_str())?;
            app("HasCounter", vec![k].into())
        }
        StatePredicate::Designated(name) => {
            let d = designation_ref_idris(name.as_str())?;
            app("HasDesignation", vec![d].into())
        }
        StatePredicate::RelatedBy(..) => {
            return Err(gap(
                "StatePredicate::RelatedBy has no Idris Predicate counterpart",
            ));
        }
        StatePredicate::Attacking => "(Holds Attack Agent)".to_string(),
        StatePredicate::Blocking => "(Holds Block Agent)".to_string(),
        // "attacking and unblocked": approximated as attacking AND not filling
        // the blocked-patient role.
        StatePredicate::Unblocked => {
            "(And [Holds Attack Agent, Not (Holds Block Patient)])".to_string()
        }
        StatePredicate::Targets(inner) => app("Targets", vec![emit_filter(inner)?].into()),
        StatePredicate::TargetCount(bound) => {
            let (cmp, count) = bound.split();
            app(
                "TargetCount",
                vec![emit_cmp(cmp), emit_count(count)?].into(),
            )
        }
        StatePredicate::WasPaidWith(tag) => app("WasPaidWith", vec![ilit(tag.as_str())].into()),
        // Unlike `WasPaidWith` (a bare string label), Idris `WasCastWith`
        // takes a `KeywordSpec`, so the tag resolves through the same
        // keyword table `Cast.tag` does, not `ilit`.
        StatePredicate::WasCastWith(tag) => {
            let spec = keywordspec_idris(tag.as_str()).ok_or_else(|| {
                gap(format!(
                    "WasCastWith keyword not in KeywordSpec table: {}",
                    tag.as_str()
                ))
            })?;
            app("WasCastWith", vec![spec].into())
        }
        // The move-provenance twin of `WasCastFrom` ([CR#701.17a,701.9a] —
        // "milled"/"discarded" decompose over this).
        StatePredicate::WasPutFrom(z) => app("WasPutFrom", vec![emit_zone(*z)].into()),
        // `SummoningSick` has no Idris counterpart; it appears only in the
        // (Idris-invisible) combatant type-confer, never in emitted card text.
        StatePredicate::SummoningSick => {
            return Err(gap(
                "StatePredicate::SummoningSick has no Idris Predicate counterpart",
            ));
        }
    })
}

/// `Above`/`Below` — [`Predicate::Adjacent`]'s direction, identity on the
/// Idris `Adjacency` constructor name.
fn emit_adjacency(a: deckmaste_semantics::Adjacency) -> String {
    use deckmaste_semantics::Adjacency as A;
    match a {
        A::Above => "Above",
        A::Below => "Below",
    }
    .to_string()
}

/// Emit a designation REFERENCE by name — `(MkDesignation <Scope> "<name>" [])`
/// — with the scope resolved from the registry loaded at emit entry. Gaps if
/// the name isn't in the loaded registry (including a game-scoped or otherwise
/// unmapped designation).
fn designation_ref_idris(name: &str) -> R {
    let scope = DESIGNATION_SCOPE
        .with(|m| m.borrow().get(name).copied())
        .ok_or_else(|| gap(format!("designation not in registry: {name}")))?;
    Ok(format!("(MkDesignation {scope} {name:?} [])"))
}

fn emit_relation_filter(rf: &RelationPredicate) -> R {
    Ok(match rf {
        RelationPredicate::ControlledBy(f) => app("ControlledBy", vec![emit_filter(f)?].into()),
        RelationPredicate::Controls(f) => app("Controls", vec![emit_filter(f)?].into()),
        RelationPredicate::Owner(f) => app("OwnedBy", vec![emit_filter(f)?].into()),
        RelationPredicate::OpponentOf(f) => {
            if matches!(f.as_ref(), Predicate::Ref(Reference::You)) {
                "OpponentOf".to_string()
            } else {
                return Err(gap(
                    "OpponentOf(<non-You>) has no Idris counterpart (Idris's OpponentOf is always relative to You)",
                ));
            }
        }
        RelationPredicate::TeammateOf(f) => {
            if matches!(f.as_ref(), Predicate::Ref(Reference::You)) {
                "TeammateOf".to_string()
            } else {
                return Err(gap("TeammateOf(<non-You>) has no Idris counterpart"));
            }
        }
        RelationPredicate::AttachedTo(_) => {
            return Err(gap(
                "Predicate RelationPredicate::AttachedTo has no Idris Predicate counterpart",
            ));
        }
        RelationPredicate::Attachment(_) => {
            return Err(gap(
                "RelationPredicate::Attachment has no Idris Predicate counterpart",
            ));
        }
    })
}

fn emit_zone(z: deckmaste_semantics::Zone) -> String {
    use deckmaste_semantics::Zone as Z;
    match z {
        Z::Battlefield => "Battlefield",
        Z::Command => "Command",
        Z::Exile => "Exile",
        Z::Graveyard => "Graveyard",
        Z::Hand => "Hand",
        Z::Library => "Library",
        Z::Stack => "Stack",
    }
    .to_string()
}

// ===========================================================================
// Condition
// ===========================================================================

fn emit_condition(c: &Condition) -> R {
    Ok(match c {
        Condition::Compare(a, cmp, b) => app(
            "Compare",
            vec![emit_count(a)?, emit_cmp(*cmp), emit_count(b)?].into(),
        ),
        Condition::Exists(f) => format!("(exists {})", emit_filter(f)?),
        Condition::Matches(r, f) => {
            app("Matches", vec![emit_reference(r)?, emit_filter(f)?].into())
        }
        Condition::DealtDamageBy(r, f) => app(
            "DealtDamageBy",
            vec![emit_reference(r)?, emit_filter(f)?].into(),
        ),
        Condition::LegallyAttached(r) => app("LegallyAttached", vec![emit_reference(r)?].into()),
        Condition::Happened { .. } => {
            return Err(gap("Condition::Happened (history lookback) not yet mapped"));
        }
        Condition::Crossed { .. } => return Err(gap("Condition::Crossed not yet mapped")),
        Condition::PaidCost(tag) => app("PaidCost", vec![ilit(tag.as_str())].into()),
        // Idris's `Condition` namespace has no `CastWith` of its own (only
        // the `WasCastWith` PREDICATE) — desugar to `Matches This
        // (WasCastWith tag)`, mirroring `Condition::Matches`'s `Matches` shape.
        Condition::CastWith(tag) => {
            let spec = keywordspec_idris(tag.as_str()).ok_or_else(|| {
                gap(format!(
                    "CastWith keyword not in KeywordSpec table: {}",
                    tag.as_str()
                ))
            })?;
            app(
                "Matches",
                vec!["This".to_string(), app("WasCastWith", vec![spec].into())].into(),
            )
        }
        Condition::YourTurn => "yourTurn".to_string(),
        Condition::TurnOf(f) => app("TurnOf", vec![emit_filter(f)?].into()),
        Condition::DuringPhase(p) => app("During", vec![emit_phase_step(*p)?].into()),
        Condition::And(cs) => app("And", vec![map_list(cs, emit_condition)?].into()),
        Condition::Or(cs) => app("Or", vec![map_list(cs, emit_condition)?].into()),
        Condition::Not(inner) => app("Not", vec![emit_condition(inner)?].into()),
        Condition::Expanded(_) => {
            return Err(gap(
                "unexpanded Condition macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_phase_step(p: PhaseStep) -> R {
    use deckmaste_semantics::BeginningStep as Bs;
    use deckmaste_semantics::CombatStep as Cs;
    use deckmaste_semantics::EndingStep as Es;
    Ok(match p {
        PhaseStep::Beginning(b) => app(
            "BeginningPhase",
            vec![
                match b {
                    Bs::Untap => "UntapStep",
                    Bs::Upkeep => "UpkeepStep",
                    Bs::Draw => "DrawStep",
                }
                .to_string(),
            ]
            .into(),
        ),
        PhaseStep::PrecombatMain => "(MainPhase PreCombat)".to_string(),
        PhaseStep::PostcombatMain => "(MainPhase PostCombat)".to_string(),
        PhaseStep::Combat(c) => app(
            "CombatPhase",
            vec![
                match c {
                    Cs::BeginningOfCombat => "BeginningOfCombatStep",
                    Cs::DeclareAttackers => "DeclareAttackersStep",
                    Cs::DeclareBlockers => "DeclareBlockersStep",
                    Cs::FirstCombatDamage => "FirstCombatDamageStep",
                    Cs::CombatDamage => "CombatDamageStep",
                    Cs::EndOfCombat => "EndOfCombatStep",
                }
                .to_string(),
            ]
            .into(),
        ),
        PhaseStep::Ending(e) => app(
            "EndingPhase",
            vec![
                match e {
                    Es::End => "EndStep",
                    Es::Cleanup => "CleanupStep",
                }
                .to_string(),
            ]
            .into(),
        ),
    })
}

// ===========================================================================
// Count / Quantity
// ===========================================================================

/// A [`Countable`] as the Idris `Countable`/`Project` source. `Objects` wraps
/// the filter; `ManaSymbols` names the pip source ([CR#700.5] devotion).
fn emit_countable(c: &Countable) -> R {
    Ok(match c {
        Countable::Objects(f) => format!("(Objects {})", emit_filter(f)?),
        // [CR#119.1]: the cross-player fold source (Arbiter of Knollridge/
        // Balance) — `Count::Aggregate`'s emission arm below prefers the
        // `eachPlayer` sugar over this raw spelling; this arm still backs
        // `Count::CountOf(Players(..))` (Idris's `CountOf (Players ..)`,
        // e.g. opponent counts) and any other bare-`Countable` position.
        Countable::Players(f) => format!("(Players {})", emit_filter(f)?),
        Countable::ManaSymbols(r, pred) => app(
            "ManaSymbols",
            vec![emit_reference(r)?, emit_symbol_pred(pred)?].into(),
        ),
        // [CR#105.2]: the per-object twin of `Objects` — Embiggen's "number
        // of card types it has" = `CountDistinct Types (Singleton This)`.
        Countable::Singleton(r) => app("Singleton", vec![emit_reference(r)?].into()),
        // [CR#107.4]: mana spent to cast/activate `r`, filtered by `pred` —
        // Adamant's "if at least three white mana symbols were spent".
        Countable::ManaSpentMatching(r, pred) => app(
            "ManaSpentMatching",
            vec![emit_reference(r)?, emit_symbol_pred(pred)?].into(),
        ),
    })
}

/// An [`AggregateOp`] as its Idris twin — identity on the fold operator name,
/// `AverageOf` carrying its [`RoundMode`](deckmaste_semantics::RoundMode).
fn emit_aggregate_op(op: &deckmaste_semantics::AggregateOp) -> String {
    use deckmaste_semantics::AggregateOp;
    match op {
        AggregateOp::SumOf => "SumOf".to_string(),
        AggregateOp::MinOf => "MinOf".to_string(),
        AggregateOp::MaxOf => "MaxOf".to_string(),
        AggregateOp::AverageOf(mode) => app(
            "AverageOf",
            vec![
                match mode {
                    deckmaste_semantics::RoundMode::RoundUp => "RoundUp",
                    deckmaste_semantics::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
            ]
            .into(),
        ),
    }
}

fn emit_count(c: &Count) -> R {
    Ok(match c {
        SemValue::X => "X".to_string(),
        Count::Literal(n) => app("Literal", vec![n.to_string()].into()),
        Count::CountOf(source) => match source {
            Countable::Objects(f) => format!("(CountMatching {})", emit_filter(f)?),
            // Every other `Countable` source (devotion's `ManaSymbols`,
            // `Singleton`, `ManaSpentMatching`) has no `CountMatching`-style
            // sugar — spell it out as the explicit `CountOf` application.
            other => app("CountOf", vec![emit_countable(other)?].into()),
        },
        Count::CountDistinct(characteristic, source) => {
            let c = collection_characteristic(*characteristic)?;
            let src = emit_countable(source)?;
            app("CountDistinct", vec![c, src].into())
        }
        // [CR#107.1]: fold a per-element `Project` — devotion's own shape
        // ([CR#700.5]). A `Players`-sourced projection ([CR#119.1] Arbiter of
        // Knollridge/Balance) emits through the Idris `eachPlayer` sugar
        // (`eachPlayer p acc = Project (Players p) acc`) rather than the raw
        // `Project (Players ..)` spelling — every other source stays the raw
        // `Project` application.
        Count::Aggregate(op, proj) => {
            let projected = match &proj.of {
                Countable::Players(filter) => app(
                    "eachPlayer",
                    vec![emit_filter(filter)?, emit_count(&proj.by)?].into(),
                ),
                other => app(
                    "Project",
                    vec![emit_countable(other)?, emit_count(&proj.by)?].into(),
                ),
            };
            app("Aggregate", vec![emit_aggregate_op(op), projected].into())
        }
        Count::StatOf(r, stat) => {
            use deckmaste_semantics::Stat as S;
            match stat {
                S::ManaValue => app("ManaValueOf", vec![emit_reference(r)?].into()),
                S::Loyalty => app(
                    "CountersOn",
                    vec!["Loyalty".to_string(), emit_reference(r)?].into(),
                ),
                S::Power | S::Toughness | S::Defense => app(
                    "StatOf",
                    vec![emit_reference(r)?, numeric_characteristic(*stat)?].into(),
                ),
            }
        }
        Count::CounterCount(r, kind) => {
            let k = counter_ref_idris(kind.as_str())?;
            app("CountersOn", vec![k, emit_reference(r)?].into())
        }
        // [CR#119.1,402.2]: a player's numeric attribute — the Idris
        // `PlayerStatOf` (the player-side twin of `StatOf`).
        Count::PlayerStatOf(r, attr) => app(
            "PlayerStatOf",
            vec![emit_reference(r)?, emit_player_attr(*attr)].into(),
        ),
        // [CR#102.1]: opponent count. Idris has no dedicated constructor — it
        // is the cardinality of the opponent PLAYERS (`CountOf (Players
        // OpponentOf)`). Idris's `OpponentOf` is relative to `You`, so an
        // opponent count of any other player has no counterpart.
        Count::Opponents(r) => match r {
            deckmaste_semantics::Reference::You => "(CountOf (Players OpponentOf))".to_string(),
            other => {
                return Err(gap(format!(
                    "Count::Opponents({other:?}) has no Idris counterpart \
                     (Idris OpponentOf is relative to You)"
                )));
            }
        },
        Count::Min(a, b) => app("Min", vec![emit_count(a)?, emit_count(b)?].into()),
        Count::Max(a, b) => app("Max", vec![emit_count(a)?, emit_count(b)?].into()),
        Count::Plus(a, b) => app("Plus", vec![emit_count(a)?, emit_count(b)?].into()),
        Count::Minus(a, b) => app("Minus", vec![emit_count(a)?, emit_count(b)?].into()),
        Count::Times(a, b) => app("Times", vec![emit_count(a)?, emit_count(b)?].into()),
        Count::Half(mode, inner) => app(
            "Half",
            vec![
                match mode {
                    deckmaste_semantics::RoundMode::RoundUp => "RoundUp",
                    deckmaste_semantics::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
                emit_count(inner)?,
            ]
            .into(),
        ),
        // [CR#107.1a]: general integer division, `Half`'s dedicated /2 twin.
        Count::Divide(mode, a, b) => app(
            "Divide",
            vec![
                match mode {
                    deckmaste_semantics::RoundMode::RoundUp => "RoundUp",
                    deckmaste_semantics::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
                emit_count(a)?,
                emit_count(b)?,
            ]
            .into(),
        ),
        // [CR#107.1]: remainder — parity checks (`Compare(Mod(x, 2), Eq, 0)`).
        Count::Mod(a, b) => app("Mod", vec![emit_count(a)?, emit_count(b)?].into()),
        // [CR#107.1]: exponentiation — doubling effects (`Pow(2, X)`).
        Count::Pow(a, b) => app("Pow", vec![emit_count(a)?, emit_count(b)?].into()),
        // [CR#115.9a]: how many times `r` was chosen as a target on
        // announcement — Strive's `Minus(TargetsOf(This), 1)`.
        Count::TargetsOf(r) => app("TargetsOf", vec![emit_reference(r)?].into()),
        Count::ThatMany | Count::ThatMuch => "ThatMany".to_string(),
        Count::Allotment => "Allotment".to_string(),
        Count::EventCount(..) => {
            return Err(gap("Count::EventCount (history lookback) not yet mapped"));
        }
        Count::EventSum(..) => {
            return Err(gap("Count::EventSum (history lookback) not yet mapped"));
        }
        SemValue::Noted(_) => {
            return Err(gap("the noted-count form has no Idris counterpart"));
        }
        Count::TimesPaid(tag) => app("TimesPaid", vec![ilit(tag.as_str())].into()),
        Count::Damage(r) => app("Damage", vec![emit_reference(r)?].into()),
        // The floated-mana-pool reader is a data-driven-strategy sensing
        // source ([CR#106.4]); the Idris grammar models card text, not play
        // policy, so it has no counterpart.
        Count::ManaAvailable(_) | Count::ManaAvailableKind(..) => {
            return Err(gap(
                "Count::ManaAvailable (floated mana pool) is a strategy-only reader \
                 with no Idris counterpart",
            ));
        }
        Count::Expanded(_) => {
            return Err(gap(
                "unexpanded Count macro invocation remained after expand_all",
            ));
        }
    })
}

fn collection_characteristic(c: deckmaste_semantics::Characteristic) -> R {
    use deckmaste_semantics::Characteristic as C;
    Ok(match c {
        C::Colors => "Colors",
        C::Types => "Types",
        C::Subtypes => "Subtypes",
        C::Supertypes => "Supertypes",
        C::Power => "Power",
        C::Toughness => "Toughness",
        C::Defense => "Defense",
        C::ManaCost => "ManaCost",
        C::Name => "Name",
        C::BasicLandTypes => "BasicLandTypes",
    }
    .to_string())
}

fn emit_quantity(q: &Quantity) -> R {
    let (lo, hi) = q.bounds();
    Ok(app(
        "Range",
        vec![
            try_maybe(&lo.cloned(), emit_count)?,
            try_maybe(&hi.cloned(), emit_count)?,
        ]
        .into(),
    ))
}

/// A `CountBound` widened into a `Quantity` range — only exact for
/// `Eq`/`AtLeast`/`AtMost`; `Greater`/`Less` need `±1` arithmetic, only
/// possible when the bound is a literal.
fn count_bound_as_quantity(b: &CountBound) -> R {
    let lit = |c: &Count| match c {
        Count::Literal(n) => Some(*n),
        _ => None,
    };
    Ok(match b {
        CountBound::Eq(c) => {
            let c = emit_count(c)?;
            app(
                "Range",
                vec![format!("(Just {c})"), format!("(Just {c})")].into(),
            )
        }
        CountBound::AtLeast(c) => app(
            "Range",
            vec![format!("(Just {})", emit_count(c)?), "Nothing".to_string()].into(),
        ),
        CountBound::AtMost(c) => app(
            "Range",
            vec!["Nothing".to_string(), format!("(Just {})", emit_count(c)?)].into(),
        ),
        CountBound::Greater(c) => {
            let n = lit(c).ok_or_else(|| {
                gap("CountBound::Greater on a dynamic Count can't be widened to a Quantity")
            })?;
            app(
                "Range",
                vec![format!("(Just (Literal {}))", n + 1), "Nothing".to_string()].into(),
            )
        }
        CountBound::Less(c) => {
            let n = lit(c).ok_or_else(|| {
                gap("CountBound::Less on a dynamic Count can't be widened to a Quantity")
            })?;
            if n == 0 {
                return Err(gap("CountBound::Less(0) has no non-negative Quantity"));
            }
            app(
                "Range",
                vec!["Nothing".to_string(), format!("(Just (Literal {}))", n - 1)].into(),
            )
        }
    })
}

// ===========================================================================
// Selection / Binder / TargetSpec
// ===========================================================================

fn emit_selection(s: &Selection) -> R {
    Ok(match s {
        Selection::SelectAll(f) => app("SelectAll", vec![emit_filter(f)?].into()),
        Selection::Union(gs) => app("Union", vec![map_list(gs, emit_selection)?].into()),
        Selection::Random(q, f) => app("Random", vec![emit_quantity(q)?, emit_filter(f)?].into()),
        Selection::AmongNoted(..) => {
            return Err(gap("Selection::AmongNoted has no Idris counterpart"));
        }
        Selection::TopOfLibrary { count, whose } => app(
            "topFrom",
            vec![emit_count(count)?, emit_reference(whose)?].into(),
        ),
        Selection::BottomOfLibrary { count, whose } => app(
            "bottomFrom",
            vec![emit_count(count)?, emit_reference(whose)?].into(),
        ),
        // The whole library as one collection ([CR#701.24a]) — lands here with
        // its first consumer, `Shuffle(Selection)`.
        Selection::LibraryOf(who) => app("LibraryOf", vec![emit_reference(who)?].into()),
        // Additive Rust-side terms with no Idris counterpart YET. `InChosenOrder`/
        // `ValidTargetsFor` land with the [CR#707.10d] for-each-could-target copy
        // family they compose. Reported as gaps meanwhile, exactly as
        // `AmongNoted` and `PilesOf` are — no canon card spells them, so the
        // re-emit gate stays green.
        Selection::InChosenOrder(..) => {
            return Err(gap(
                "Selection::InChosenOrder has no Idris counterpart yet (lands with the \
                 for-each-could-target copy family)",
            ));
        }
        Selection::ValidTargetsFor(_) => {
            return Err(gap(
                "Selection::ValidTargetsFor has no Idris counterpart yet (lands with the \
                 for-each-could-target copy family)",
            ));
        }
        // Unlike `TopOfLibrary`/`BottomOfLibrary`'s `topFrom`/`bottomFrom`
        // helpers (baking a `{default You}` implicit arg), Idris
        // `TopOfGraveyard`'s `whose` is EXPLICIT — emits as a direct `app`,
        // no curried helper needed.
        Selection::TopOfGraveyard { count, of } => app(
            "TopOfGraveyard",
            vec![emit_count(count)?, emit_reference(of)?].into(),
        ),
        // [CR#115.3,601.2c]: the nth announced slot read as its whole group —
        // the plural twin of `Reference::Target`, and like it a POSITIONAL,
        // proof-free read of the announce list rather than an anaphor over the
        // antecedent stack. Idris disambiguates the name from `Predicate.Targets`
        // by argument type (a `Nat` index, not a `Predicate`) — the same
        // deliberate clash `Reference.Target` already carries.
        Selection::Targets(n) => app("Targets", vec![n.to_string()].into()),
        Selection::They => "They".to_string(),
        Selection::Them(sort) => app("Them", vec![emit_sort(sort)?].into()),
        Selection::PilesOf { .. } => return Err(gap("Selection::PilesOf not yet mapped")),
        // [CR#107.1]: the extremal element(s) of `proj` — shares `Project`
        // emission with `Count::Aggregate`; `op` is identity on the Idris
        // `AggregateOp` name (gated to `MinOf`/`MaxOf` at the Rust type by
        // the eval fizzle, not here). Idris's `Pick` is pinned to `Projection
        // b AnObject` ([CR#107.1] — a player-`Pick` has no consumer and no
        // Idris counterpart), so a `Players`-sourced `proj` here is an
        // semantic-input error, not a representable card — reported as a gap
        // rather than emitted as ill-typed Idris.
        Selection::Pick { op, proj } => {
            if matches!(proj.of, Countable::Players(_)) {
                return Err(gap(
                    "Selection::Pick is pinned to AnObject in Idris; a Players-sourced \
                     projection has no counterpart",
                ));
            }
            app(
                "Pick",
                vec![
                    emit_aggregate_op(op),
                    app(
                        "Project",
                        vec![emit_countable(&proj.of)?, emit_count(&proj.by)?].into(),
                    ),
                ]
                .into(),
            )
        }
        Selection::Expanded(_) => {
            return Err(gap(
                "unexpanded Selection macro invocation remained after expand_all",
            ));
        }
    })
}

/// `TargetSpec.Target`'s filter is a `Predicate b k` with `k` free (like the
/// `Deed` patient), so a bare "any target" filter needs a concretely-kinded
/// value too — reproduced here (sugar-free) from `Macros.idr`'s own
/// `anyTarget` ([CR#115.4]: creature/planeswalker/battle permanent OR any
/// player) rather than the kind-ambiguous empty conjunction.
fn any_target_predicate() -> String {
    "(Or [(And [(InZone Battlefield), (HasChar Types Battle)]), \
(And [(InZone Battlefield), (HasChar Types Creature)]), \
(And [(InZone Battlefield), (HasChar Types Planeswalker)]), \
Anyone])"
        .to_string()
}

fn emit_target_spec(t: &TargetSpec) -> R {
    Ok(match t {
        TargetSpec::Target(q, f) => {
            let pred = if matches!(f, Predicate::Any) {
                any_target_predicate()
            } else {
                emit_filter(f)?
            };
            app("Target", vec![emit_quantity(q)?, pred].into())
        }
        TargetSpec::Distinct(idxs, inner) => app(
            "Distinct",
            vec![
                ilist(idxs.iter().map(usize::to_string).collect()),
                emit_target_spec(inner)?,
            ]
            .into(),
        ),
        TargetSpec::Expanded(_) => {
            return Err(gap(
                "unexpanded TargetSpec macro invocation remained after expand_all",
            ));
        }
    })
}

/// `deckmaste_semantics::Binder` -> Idris `Bindable`. Only the shapes actually
/// wired for resolution are mapped (`binder.rs` notes `Produce`/`Search*`
/// aren't yet engine-resolved either, so gapping them costs nothing today).
fn emit_binder(b: &deckmaste_semantics::Binder) -> R {
    use deckmaste_semantics::Binder as B;
    Ok(match b {
        B::TheRef(r) => app("TheRef", vec![emit_reference(r)?].into()),
        B::ChooseOne { filter, by } => app(
            "chooseOneBy",
            vec![emit_reference(by)?, emit_filter(filter)?].into(),
        ),
        B::Choose {
            quantity,
            filter,
            by,
        } => app(
            "chooseBy",
            vec![
                emit_reference(by)?,
                emit_quantity(quantity)?,
                emit_filter(filter)?,
            ]
            .into(),
        ),
        B::Existing(sel) => app("Existing", vec![emit_selection(sel)?].into()),
        B::Produce(action) => app("Produce", vec![emit_action_off_effect_path(action)?].into()),
        B::SearchOne { filter, .. } => app("SearchOne", vec![emit_filter(filter)?].into()),
        B::Search {
            quantity, filter, ..
        } => app(
            "Search",
            vec![emit_quantity(quantity)?, emit_filter(filter)?].into(),
        ),
        B::Expanded(_) => {
            return Err(gap(
                "unexpanded Binder macro invocation remained after expand_all",
            ));
        }
    })
}

// ===========================================================================
// Cost
// ===========================================================================

fn emit_cost(cost: &Cost) -> R {
    let normalized = cost.clone().normalize();
    let mut components = Vec::with_capacity(normalized.len());
    for c in &normalized {
        components.push(emit_cost_component(c)?);
    }
    Ok(app("Costs", vec![ilist(components.into())].into()))
}

fn emit_cost_component(c: &CostComponent) -> R {
    Ok(match c {
        CostComponent::Mana(cost) => app("Mana", vec![emit_mana_cost(cost)].into()),
        CostComponent::ManaCostOf(r) => app("ManaCostOf", vec![emit_reference(r)?].into()),
        CostComponent::Tap => "(Do (Tap This))".to_string(),
        CostComponent::Untap => "(Do (Untap This))".to_string(),
        // `Do : Action b -> Cost b` is UNRESTRICTED in Idris; the widened
        // Rust `Do(Box<Action>)` now matches it one-to-one — a bare player
        // verb arrives as `By(You, …)` and re-emits through the `<verb>By`
        // helpers, a discard composite emits as `Composite (Discard …) …`.
        CostComponent::Do(action) => app("Do", vec![emit_action_off_effect_path(action)?].into()),
        CostComponent::Cost(_) => {
            return Err(gap("CostComponent::Cost should have been normalized away"));
        }
        CostComponent::TapTotal {
            stat,
            cmp,
            count,
            filter,
        } => app(
            "TapTotal",
            vec![
                numeric_characteristic(*stat)?,
                emit_cmp(*cmp),
                emit_count(count)?,
                emit_filter(filter)?,
            ]
            .into(),
        ),
        CostComponent::With { binder, body } => {
            // The common "sacrifice/tap a chosen one" cost shape: a
            // ChooseOne binder whose body pays with the bound choice. Idris's
            // verbs already bake the choice into a Predicate, so this
            // desugars to a single Cost component naming the filter — no
            // Binder needed on the Idris side.
            emit_with_cost_as_predicate_verb(binder, body)?
        }
        CostComponent::Expanded(_) => {
            return Err(gap(
                "unexpanded CostComponent macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_with_cost_as_predicate_verb(binder: &deckmaste_semantics::Binder, body: &Cost) -> R {
    use deckmaste_semantics::Binder as B;
    let B::ChooseOne { filter, by } = binder else {
        return Err(gap(
            "CostComponent::With over a non-ChooseOne binder not yet mapped",
        ));
    };
    let normalized = body.clone().normalize();
    if normalized.len() != 1 {
        return Err(gap("CostComponent::With body isn't a single component"));
    }
    match &normalized[0] {
        CostComponent::Do(action) => match action.as_ref() {
            Action::Sacrifice(_, Reference::That(_)) => {
                let sac = app(
                    "Sacrifice",
                    vec![emit_reference(by)?, emit_filter(filter)?].into(),
                );
                Ok(app("Do", vec![sac].into()))
            }
            _ => Err(gap(
                "CostComponent::With body isn't the recognized Sacrifice(That) shape",
            )),
        },
        _ => Err(gap("CostComponent::With body isn't a Do(...) component")),
    }
}

// ===========================================================================
// Destination / EnterRider / Arrangement / CounterSpec
// ===========================================================================

fn emit_destination(d: &Destination) -> R {
    Ok(match d {
        Destination::Zone(z) => app("ToZone", vec![emit_zone(*z)].into()),
        Destination::Library(anchor) => app("ToLibrary", vec![emit_anchor(anchor)?].into()),
    })
}

fn emit_anchor(a: &Anchor) -> R {
    Ok(match a {
        Anchor::FromTop(c) => app("FromTop", vec![emit_count(c)?].into()),
        Anchor::FromBottom(c) => app("FromBottom", vec![emit_count(c)?].into()),
    })
}

fn emit_arrangement(a: &Arrangement) -> String {
    match a {
        Arrangement::AnyOrder | Arrangement::ChosenOrder(_) => "ChosenOrder",
        Arrangement::SameOrder => "SameOrder",
        Arrangement::RandomOrder => "RandomOrder",
    }
    .to_string()
}

/// The [CR#115.7a..115.7d] four-way `Action::Retarget` discriminant — a 1:1
/// rename match with Idris's `RetargetMode`.
fn emit_retarget_mode(m: &RetargetMode) -> String {
    match m {
        RetargetMode::ChangeAll => "ChangeAll",
        RetargetMode::ChangeOne => "ChangeOne",
        RetargetMode::ChangeAny => "ChangeAny",
        RetargetMode::ChooseNew => "ChooseNew",
    }
    .to_string()
}

/// Only an empty rider list, or a single `Attacking(Some(_))` rider, has an
/// Idris counterpart (`enteringAttacking`); anything else is a gap.
fn enter_riders_as_attacking(riders: &[EnterRider]) -> Result<Option<String>, Gap> {
    match riders {
        [] => Ok(None),
        [EnterRider::Attacking(Some(who))] => Ok(Some(format!("(Just {})", emit_reference(who)?))),
        // enters-as-a-copy ([CR#707.5]): the copy self-modification lowers to
        // `Modify This (BecomeCopyOf <src>)` (`emit_copy_modification`), but it
        // is NOT an attacking `Maybe` — `moveAttacking`/`createTokenAttacking`
        // have no slot for it. Its real Idris carrier is the "as ~ enters, …"
        // ETB-replacement seam, which no Rust card wraps `AsCopy` through yet
        // (see `render::effect`), so this stays a gap pending that carrier
        // (idris-copy-asenters-carrier). The payload mapping is exercised by a
        // direct unit test (`as_copy_lowers_to_become_copy_of`).
        [EnterRider::AsCopy(_)] => Err(gap(
            "EnterRider::AsCopy ([CR#707.5]) lowers to `Modify This (BecomeCopyOf …)` but has no \
             moveAttacking/createTokenAttacking slot; its ETB-replacement carrier is not yet \
             emitted (idris-copy-asenters-carrier)",
        )),
        _ => Err(gap(
            "EnterRider list has no Idris Move/MoveGroup counterpart beyond a lone Attacking(Some(_))",
        )),
    }
}

/// The `enteringAttacking` argument as a plain Idris `Maybe` — `Nothing` for
/// no rider, `(Just <ref>)` for a lone `Attacking(Some(_))` — for the
/// positional `moveAttacking`/`createTokenAttacking` helpers.
fn attacking_maybe(riders: &[EnterRider]) -> R {
    Ok(enter_riders_as_attacking(riders)?.unwrap_or_else(|| "Nothing".to_string()))
}

fn emit_counter_spec(c: &CounterSpec) -> R {
    Ok(match c {
        CounterSpec::Named(kind, count) => {
            let k = counter_ref_idris(kind.as_str())?;
            app("Some", vec![k, emit_count(count)?].into())
        }
        CounterSpec::AllKinds => "AllKinds".to_string(),
    })
}

// ===========================================================================
// Action
// ===========================================================================

/// Peel a remembered macro invocation to its expanded value — the coordinate
/// extractors below read the canonical body, not the provenance wrapper.
fn peel_os(e: &OneShotEffect) -> &OneShotEffect {
    match e {
        OneShotEffect::Expanded(x) => peel_os(&x.value),
        other => other,
    }
}

/// The single-move patient of a keyword-action body — `Destroy`'s / bound
/// `Discard`'s `Move(r, …)` head names `r`.
fn composite_move_src(body: &OneShotEffect) -> Option<&Reference> {
    match peel_os(body) {
        OneShotEffect::Act(Action::Move(r, _, _, _)) => Some(r),
        _ => None,
    }
}

/// The `(whose, count)` of a slice-verb body ([CR#121,701.17a,701.22a]): the
/// `TopOfLibrary` selection in a draw/scry `Each` binder or a mill `MoveGroup`.
fn top_of_library(body: &OneShotEffect) -> Option<(&Reference, &Count)> {
    match peel_os(body) {
        OneShotEffect::Each(each) => match &each.binder {
            deckmaste_semantics::Binder::Existing(Selection::TopOfLibrary { count, whose }) => {
                Some((whose, count))
            }
            _ => None,
        },
        OneShotEffect::Act(Action::MoveGroup {
            group: Selection::TopOfLibrary { count, whose },
            ..
        }) => Some((whose, count)),
        _ => None,
    }
}

/// The `(whose, count)` of a chosen/random discard body ([CR#701.9b]): its
/// `With` binder's [`deckmaste_semantics::discard_body_whose`]/
/// [`deckmaste_semantics::discard_body_count`]. `None` for a bound single-move
/// discard.
fn discard_choice(body: &OneShotEffect) -> Option<(&Reference, &Count)> {
    Some((
        deckmaste_semantics::discard_body_whose(body)?,
        deckmaste_semantics::discard_body_count(body)?,
    ))
}

/// Reconstruct the parameterized `KeywordActionSpec` constructor from a verb
/// NAME plus the coordinates carried in its expanded body ([CR#701]) — the
/// `KeywordAction` atom enum retired, so name + body-shape is the source of
/// truth (shared by the engine's resolve dispatch and the renderer). ENTITY-
/// keyed: each atom names only the coordinates it fixes — Scry/Surveil the
/// count (your library by definition), Mill/Draw the performing player (count
/// rides the enclosing `Batch`), Fateseal the fatesealed player plus the count,
/// Discard the discarding player plus the choice count, Destroy/Fight their
/// objects. Any unrecognized verb is a gap against the current Idris mirror.
fn emit_keyword_spec(name: &str, body: &OneShotEffect) -> R {
    match name {
        "Destroy" => {
            let r = composite_move_src(body)
                .ok_or_else(|| gap("Destroy composite body is not a Move"))?;
            Ok(app("Destroy", vec![emit_reference(r)?].into()))
        }
        "Scry" | "Surveil" => {
            // Definitionally YOUR library ([CR#701.22a,701.25a]): the atom carries
            // only the count; the top-slice `whose` is implicit.
            let (_whose, count) = top_of_library(body)
                .ok_or_else(|| gap(format!("{name} composite body has no TopOfLibrary slice")))?;
            Ok(app(name, vec![emit_count(count)?].into()))
        }
        // SLICE family ([CR#701.17a]): the atom carries the performing player;
        // the count rides the enclosing `Batch`, and each contained atom is a
        // single top-slot slice — so no count arg here. Mill is alone in this
        // family now: draw is [CR#121], NOT a keyword action ([CR#701]), so it
        // never reaches the `Composite` lane at all.
        "Mill" => {
            let (whose, _count) = top_of_library(body)
                .ok_or_else(|| gap(format!("{name} composite body has no TopOfLibrary slice")))?;
            Ok(app(name, vec![emit_reference(whose)?].into()))
        }
        "Fateseal" => {
            // The named player's library top ([CR#701.20a]) plus the count.
            let (whose, count) = top_of_library(body)
                .ok_or_else(|| gap("Fateseal composite body has no TopOfLibrary slice"))?;
            Ok(app(
                "Fateseal",
                vec![emit_reference(whose)?, emit_count(count)?].into(),
            ))
        }
        "Discard" => {
            // Chosen/random form: the `With` binder carries whose + count
            // (Task 8: `Choose`/`Selection::Random` over the hand filter, not
            // a bespoke selection). Bound single-move form ("discard this
            // card"): the discarding player rides the patient; emit the
            // implicit `You` performer over one card.
            if let Some((whose, count)) = discard_choice(body) {
                Ok(app(
                    "Discard",
                    vec![emit_reference(whose)?, emit_count(count)?].into(),
                ))
            } else if composite_move_src(body).is_some() {
                Ok(app(
                    "Discard",
                    vec![
                        emit_reference(&Reference::You)?,
                        emit_count(&Count::Literal(1))?,
                    ]
                    .into(),
                ))
            } else {
                Err(gap(
                    "Discard composite body is neither a chosen/random choice nor a Move",
                ))
            }
        }
        "Fight" => {
            let (a, b) = deckmaste_semantics::fight_body_fighters(body)
                .ok_or_else(|| gap("Fight composite body has no fighters"))?;
            Ok(app(
                "Fight",
                vec![emit_reference(a)?, emit_reference(b)?].into(),
            ))
        }
        other => Err(gap(format!(
            "Composite keyword action {other} has no Idris KeywordActionSpec"
        ))),
    }
}

/// `emit_action`, gated for the two entry points that call into `Action`
/// directly rather than through `emit_effect` (`CostComponent::Do`,
/// `Binder::Produce`). `emit_effect`'s own `OneShotEffect::Act(Create { token:
/// TokenSpec::Copy(_), .. })` arm is the ONE place that Batch-wraps a count>1
/// token-copy's multiplicity — `emit_action`'s `Create` arm never reads
/// `count` for the `TokenSpec::Copy` case (the `Copy` Action itself carries
/// no multiplicity slot, [CR#707.2]), relying entirely on that outer `Batch`.
/// Reached off that path, a count != 1 would silently emit a single `Copy`,
/// dropping the multiplier — gap instead of guessing.
fn emit_action_off_effect_path(action: &Action) -> R {
    if let Action::Create {
        count,
        token: TokenSpec::Copy(_),
        ..
    } = action
        && count.literal_value() != Some(1)
    {
        return Err(gap(
            "token-copy Create with count != 1 reached off the Batch-wrapped effect path (Do/ \
             Produce) has no per-unit Idris Copy multiplicity slot",
        ));
    }
    emit_action(action)
}

#[expect(
    clippy::too_many_lines,
    reason = "one match arm per verb; splitting would scatter the verb→Idris-constructor mapping this function documents as a whole"
)]
fn emit_action(a: &Action) -> R {
    match a {
        Action::DealDamage(source, count, patient) => Ok(app(
            "DealDamage",
            vec![
                emit_reference(source)?,
                emit_count(count)?,
                emit_reference_anykind(patient)?,
            ]
            .into(),
        )),
        // Destroy is not a bespoke verb — it emits through the `Composite` arm
        // below (`Composite (Destroy r) (Act (moveAttacking r Graveyard))`).
        Action::Counter(r) => Ok(app("Counter", vec![emit_reference(r)?].into())),
        Action::Transform(r) => Ok(app("Transform", vec![emit_reference(r)?].into())),
        // core-copy-grammar Task 5: the cease-to-exist verb the copy-cease
        // SBA now speaks through (`sba.rs`) — engine-internal only, never
        // authored on a card face, so it has no Idris counterpart, same
        // deferred-gap shape as `ExtraPhase`/`CreateReplacement` below.
        Action::Cease(_) => Err(gap("Action::Cease has no Idris counterpart")),
        Action::Attach { what, to } => Ok(app(
            "Attach",
            vec![emit_reference(what)?, emit_reference(to)?].into(),
        )),
        Action::Unattach(r) => Ok(app("Unattach", vec![emit_reference(r)?].into())),
        // `moveAttacking` exposes the (default-`Nothing`) `enteringAttacking`
        // positionally as a plain `Maybe`; it does NOT forward `from` (no
        // Idris counterpart in its own signature). The unguarded default
        // (`from = None`) keeps this terse wrapper call untouched — `Move`'s
        // own `{default Nothing from}` applies unspoken; a guard bypasses the
        // wrapper for the raw `Move` constructor with BOTH named fields
        // spelled explicitly ([CR#701.8a,701.9a,701.17a]).
        Action::Move(r, dest, riders, from) => {
            let r_ = emit_reference(r)?;
            let dest_ = emit_destination(dest)?;
            let ea = attacking_maybe(riders)?;
            Ok(match from {
                None => app("moveAttacking", vec![r_, dest_, ea].into()),
                Some(z) => format!(
                    "(Move {r_} {dest_} {{enteringAttacking = {ea}}} {{from = (Just {})}})",
                    emit_zone(*z)
                ),
            })
        }
        Action::MoveGroup {
            group,
            arrangement,
            to,
            riders,
        } => {
            if !riders.is_empty() {
                return Err(gap("MoveGroup riders not yet mapped"));
            }
            Ok(app(
                "MoveGroup",
                vec![
                    emit_selection(group)?,
                    emit_arrangement(arrangement),
                    emit_destination(to)?,
                ]
                .into(),
            ))
        }
        Action::GainControl(..) => Err(gap(
            "Action::GainControl has no Idris one-shot Action counterpart (only the continuous Modification)",
        )),
        Action::ExtraPhase(..) => Err(gap("Action::ExtraPhase has no Idris counterpart")),
        Action::MoveCounters(spec, from, to) => Ok(app(
            "MoveCounters",
            vec![
                emit_counter_spec(spec)?,
                emit_reference(from)?,
                emit_reference(to)?,
            ]
            .into(),
        )),
        Action::CreateReplacement { .. } => {
            Err(gap("Action::CreateReplacement has no Idris counterpart"))
        }
        // `Composite <spec> body` ([CR#701]): the named keyword action —
        // `Composite (Scry You 2) (Each …)` etc. The `KeywordAction` atom enum
        // retired, so the parameterized `KeywordActionSpec` constructor is
        // RECONSTRUCTED from the verb NAME plus the coordinates carried in its
        // expanded body (`emit_keyword_spec`). Mill/Draw are `Batch`-wrapped, so
        // their per-unit `Composite` carries a single-card slice, emitted via
        // this arm inside the `Batch`. `Fateseal` has no Idris spec yet — a gap.
        Action::Composite { name, body } => {
            let spec = emit_keyword_spec(name.as_str(), body)?;
            Ok(app("Composite", vec![spec, emit_effect(body)?].into()))
        }
        // [CR#104.2b,104.3e]: win/lose ride `Action::{WinGame,LoseGame}` in
        // Rust, but Idris keeps them OUTSIDE the `Action` data type — a
        // sibling `Outcome` wrapped by `Conclude : Outcome b -> OneShotEffect
        // b`, never an `Act`. This arm builds `(Conclude (WinGame/LoseGame
        // patient))` whole; `emit_effect`'s `Act` arm must not re-wrap it (see
        // its own anti-double-wrap carve-out, right next to this comment's twin).
        Action::WinGame(patient) => Ok(app(
            "Conclude",
            vec![app("WinGame", vec![emit_reference(patient)?].into())].into(),
        )),
        Action::LoseGame(patient) => Ok(app(
            "Conclude",
            vec![app("LoseGame", vec![emit_reference(patient)?].into())].into(),
        )),
        // change a player's life total ([CR#119.3,119.9]) — the merged
        // `GainLife`/`LoseLife`/`SetLifeTo` family; Idris mirrors the merge
        // one-for-one with its own `ChangeLife`/`LifeOp`.
        Action::ChangeLife(patient, op) => {
            let life_op = match op {
                LifeOp::Set(c) => app("Set", vec![emit_count(c)?].into()),
                LifeOp::Up(c) => app("Up", vec![emit_count(c)?].into()),
                LifeOp::Down(c) => app("Down", vec![emit_count(c)?].into()),
            };
            Ok(app(
                "ChangeLife",
                vec![emit_reference(patient)?, life_op].into(),
            ))
        }
        Action::AddMana(recipient, count, production) => {
            let (mana, riders) = emit_mana_production(production)?;
            Ok(app(
                "addManaFull",
                vec![
                    emit_reference(recipient)?,
                    emit_count(count)?,
                    mana,
                    ilist(riders),
                ]
                .into(),
            ))
        }
        Action::Create {
            agent,
            count,
            token,
            riders,
        } => {
            // A token copy is the `Copy` Action, not `createTokenAttacking` —
            // `emit_create_token_copy` builds the per-unit copy; the `count`
            // rides a `Batch` in `emit_effect`, not this arm. `Copy` has no
            // Idris creator slot.
            if let TokenSpec::Copy(cs) = token {
                if !matches!(agent, Reference::You) {
                    return Err(gap("token-copy Create has no Idris agent slot"));
                }
                return emit_create_token_copy(cs, riders);
            }
            Ok(app(
                "createTokenAttacking",
                vec![
                    emit_reference(agent)?,
                    emit_count(count)?,
                    emit_token_spec(token)?,
                    attacking_maybe(riders)?,
                ]
                .into(),
            ))
        }
        Action::Sacrifice(agent, what) => Ok(app(
            "Sacrifice",
            vec![emit_reference(agent)?, reference_as_predicate(what)?].into(),
        )),
        // ONE card ([CR#121.1,121.2]) — a bare per-card draw reached OFF the
        // `Batch` path, so the count is 1 here. `Batch(n, Act(DrawCard(who)))`
        // is folded to `Act (DrawCard who n)` by `emit_effect` instead (via
        // `emit_batched_draw`), because Idris's `Action.DrawCard` carries the
        // count on the verb.
        Action::DrawCard(who) => Ok(app(
            "DrawCard",
            vec![
                emit_reference(who)?,
                emit_count(&deckmaste_semantics::Count::Literal(1))?,
            ]
            .into(),
        )),
        Action::GetEmblem(_, _)
        | Action::GetDesignation(_, _)
        | Action::ChooseValue(_, _, _)
        | Action::RestartGame => Err(gap(format!(
            "{a:?} not yet mapped (no Idris counterpart or not implemented)"
        ))),
        Action::Tap(r) => Ok(app("Tap", vec![emit_reference(r)?].into())),
        Action::Untap(r) => Ok(app("Untap", vec![emit_reference(r)?].into())),
        // A stack copy ([CR#707.10]) lowers to the shared `Copy` Action (source
        // + copiable-value exceptions, same helpers the token-copy `Create`
        // path uses). `controller`/`retarget` have no Idris slot — a non-`You`
        // controller, or a retarget mode beyond `AsIs`, is a gap.
        Action::CopySpell {
            controller,
            spec,
            retarget,
        } => {
            if !matches!(controller, Reference::You) {
                return Err(gap("Action::CopySpell has no Idris controller slot"));
            }
            if !matches!(retarget, CopyRetarget::AsIs) {
                return Err(gap(
                    "Action::CopySpell retarget modes beyond AsIs have no Idris counterpart",
                ));
            }
            Ok(app(
                "Copy",
                vec![
                    emit_copy_source(&spec.source)?,
                    ilist(emit_copy_exception_mods(&spec.exceptions)?),
                ]
                .into(),
            ))
        }
        // [CR#608.2g]: casting a referenced card as a resolution effect. The
        // Idris north-star `Semantics.idr` has no resolution-time `Cast` effect verb
        // (casting there rides the 601 deontic-permission pipeline, `Enact
        // Cast`), so the probe records this as an unmapped gap rather than
        // data-fying an intrinsic it doesn't model.
        Action::Cast(..) => Err(gap(
            "Action::Cast (resolution-time cast-as-effect, [CR#608.2g]) has no Idris \
             OneShotEffect counterpart — Idris casts via the 601 permission pipeline",
        )),
        // [CR#707.12]: casting a COPY of an object as a resolution effect — the
        // same class of gap as `Action::Cast` above. Idris has no
        // resolution-time cast verb (casting rides the 601 deontic-permission
        // pipeline, `Enact Cast`), so the copy-cast has no `OneShotEffect`
        // counterpart either; the `CopySpec` it carries has no cast verb to
        // attach to. Its own arm (not the generic catch-all) so the reason is
        // explicit.
        Action::CastCopy(..) => Err(gap(
            "Action::CastCopy (cast-a-copy-as-effect, [CR#707.12]) has no Idris \
             OneShotEffect counterpart — Idris casts via the 601 permission pipeline, with no \
             resolution-time cast verb (same class as Action::Cast)",
        )),
        // `mode`/`of`/`by` are all plain positional now (Law 2 stripped `by`'s
        // `{default You}`), so this is a direct `app` — no default-arg-elision
        // special-casing needed any more.
        Action::Retarget { mode, of, by } => Ok(app(
            "Retarget",
            vec![
                emit_retarget_mode(mode),
                emit_reference(of)?,
                emit_reference(by)?,
            ]
            .into(),
        )),
        // [CR#705.1,706.1,901.9]: `agent` is now a real, non-defaulted slot on
        // all three (it used to arrive only via the deleted `By` wrapper, and
        // Idris's own constructors carried no actor slot at all).
        Action::FlipCoins(agent, count, called) => Ok(app(
            "FlipCoins",
            vec![
                emit_reference(agent)?,
                emit_count(count)?,
                if *called { "True" } else { "False" }.to_string(),
            ]
            .into(),
        )),
        Action::RollDice(agent, count, sides) => Ok(app(
            "RollDice",
            vec![
                emit_reference(agent)?,
                emit_count(count)?,
                sides.to_string(),
            ]
            .into(),
        )),
        Action::RollPlanarDie(agent) => {
            Ok(app("RollPlanarDie", vec![emit_reference(agent)?].into()))
        }
        Action::SetGameDesignation(name, value) => Ok(app(
            "SetGameDesignation",
            vec![ilit(name.as_str()), ilit(value.as_str())].into(),
        )),
        Action::PutCounters(r, kind, count) => {
            let k = counter_ref_idris(kind.as_str())?;
            Ok(app(
                "PutCounters",
                vec![k, emit_count(count)?, emit_reference(r)?].into(),
            ))
        }
        Action::RemoveCounters(r, kind, count) => {
            let k = counter_ref_idris(kind.as_str())?;
            Ok(app(
                "RemoveCounters",
                vec![k, emit_count(count)?, emit_reference(r)?].into(),
            ))
        }
        // "shuffle a collection" ([CR#701.24a]) — the ACTOR CONCEPT IS GONE,
        // replaced by the collection argument (Law 3). Only `LibraryOf` has an
        // Idris counterpart so far (added alongside this verb); any other
        // `Selection` is a gap via `emit_selection`.
        Action::Shuffle(sel) => Ok(app("Shuffle", vec![emit_selection(sel)?].into())),
        Action::Reveal { what, .. } => Ok(app("Reveal", vec![emit_reference(what)?].into())),
        Action::RemoveDamage(r) => Ok(app("RemoveDamage", vec![emit_reference(r)?].into())),
        // [CR#118.12]: the slotless `Cost` -> `Action` adapter. Bare
        // effect-position `Pay` is unspellable (no legal host), so in
        // practice this is only reached through the `May` arm's own direct
        // `mayPayCostBy` emission below — never via generic recursion — but
        // `Action` is total-by-enumeration on the Idris side too, so this
        // stays a real arm rather than a gap.
        Action::Pay(cost) => Ok(app("Pay", vec![emit_cost(cost)?].into())),
        Action::Expanded(_) => Err(gap(
            "unexpanded Action macro invocation remained after expand_all",
        )),
    }
}

fn emit_mana_production(p: &ManaProduction) -> Result<(String, Arc<[String]>), Gap> {
    let (spec, riders) = match p {
        ManaProduction::Bare(spec) => (spec, &[][..]),
        ManaProduction::WithRiders { mana, riders } => (mana, riders.as_ref()),
    };
    let mana = emit_mana_spec(spec)?;
    let mut out_riders = Vec::new();
    for r in riders {
        match r {
            deckmaste_semantics::ManaRider::SpendOnly(f) => {
                out_riders.push(app("SpendOnly", vec![emit_filter(f)?].into()));
            }
            deckmaste_semantics::ManaRider::GrantOnSpend(_)
            | deckmaste_semantics::ManaRider::TriggerOnSpend(_)
            | deckmaste_semantics::ManaRider::Persistent(_)
            | deckmaste_semantics::ManaRider::Snow
            | deckmaste_semantics::ManaRider::Expanded(_) => {
                return Err(gap("ManaRider variant not yet mapped"));
            }
        }
    }
    Ok((mana, out_riders.into()))
}

fn emit_mana_spec(spec: &ManaSpec) -> R {
    Ok(match spec {
        ManaSpec::AnyColor => "AnyColor".to_string(),
        ManaSpec::OneOf(cs) => app(
            "OneOf",
            vec![ilist(
                cs.iter().map(|c| emit_color_or_colorless(*c)).collect(),
            )]
            .into(),
        ),
        // The filterland cycle: a choice among multi-symbol runs. Each run is
        // itself an Idris list of `Maybe Color`, so the argument is a list of
        // lists.
        ManaSpec::OneOfRuns(runs) => app(
            "OneOfRuns",
            vec![ilist(
                runs.iter()
                    .map(|run| ilist(run.iter().map(|c| emit_color_or_colorless(*c)).collect()))
                    .collect(),
            )]
            .into(),
        ),
        ManaSpec::Specific(c) => app("OfColor", vec![emit_color_or_colorless(*c)].into()),
        ManaSpec::AmongColorsOf(r) => app("AmongColorsOf", vec![emit_reference(r)?].into()),
        ManaSpec::ProducedByEvent => "ProducedByEvent".to_string(),
    })
}

// ===========================================================================
// Token characteristics (for `Create`)
// ===========================================================================

fn emit_token_spec(spec: &TokenSpec) -> R {
    let token = match spec {
        TokenSpec::Token(t) => t.clone(),
        TokenSpec::Named(name) => name
            .resolve()
            .ok_or_else(|| {
                gap(format!(
                    "unresolvable predefined token name: {}",
                    name.as_str()
                ))
            })?
            .into(),
        // A token copy ([CR#707.2]) does NOT go through this characteristics
        // emitter: the `Action::Create` arm intercepts `TokenSpec::Copy`
        // upstream and emits the `Copy` Action (source + copiable-value
        // exceptions) instead of `createTokenAttacking`. This arm is thus
        // unreachable in the Create path; kept for match exhaustiveness.
        TokenSpec::Copy(_) => {
            return Err(gap(
                "TokenSpec::Copy is emitted as the `Copy` Action by the Create arm, \
                 not through emit_token_spec",
            ));
        }
    };
    emit_token_characteristics(&token)
}

fn emit_token_characteristics(t: &Token) -> R {
    let colors = ilist(t.color_indicator.iter().map(|c| emit_color(*c)).collect());
    let types = map_list(&t.types, |ty| emit_type_name(ty.name.as_str()))?;
    let supertypes = ilist(t.supertypes.iter().map(|s| emit_supertype(*s)).collect());
    let subtypes = map_list(&t.subtypes, emit_subtype)?;
    let abilities = emit_ability_list(&t.abilities)?;
    let power = try_maybe(&t.power, emit_stat_value)?;
    let toughness = try_maybe(&t.toughness, emit_stat_value)?;
    Ok(app(
        "MkCharacteristics",
        vec![
            "Nothing".to_string(),
            "[]".to_string(),
            colors,
            types,
            supertypes,
            subtypes,
            abilities,
            power,
            toughness,
            "Nothing".to_string(),
            "Nothing".to_string(),
        ]
        .into(),
    ))
}

// ===========================================================================
// Modification / StaticEffect / Deontic
// ===========================================================================

fn emit_numeric_op(op: &NumericOp) -> R {
    Ok(match op {
        NumericOp::Set(v) => app("Set", vec![emit_stat_value(v)?].into()),
        NumericOp::Up(c) => app("Up", vec![emit_count(c)?].into()),
        NumericOp::Down(c) => app("Down", vec![emit_count(c)?].into()),
    })
}

fn emit_collection_op<T>(op: &deckmaste_semantics::CollectionOp<T>, elem: impl Fn(&T) -> R) -> R {
    Ok(match op {
        deckmaste_semantics::CollectionOp::Set(items) => {
            app("Set", vec![map_list(items, &elem)?].into())
        }
        deckmaste_semantics::CollectionOp::Add(item) => app("Add", vec![elem(item)?].into()),
        deckmaste_semantics::CollectionOp::Remove(item) => app("Remove", vec![elem(item)?].into()),
    })
}

/// One `Modification` -> a `Alter <characteristic> <op>` (or a bare special
/// like `SwitchPowerToughness`); flattened into the caller's list (a Rust
/// `Several` splices its members in, mirroring `Modification::flatten`).
fn emit_modification_ops(m: &Modification, out: &mut Vec<String>) -> Result<(), Gap> {
    match m {
        Modification::Power(op) => out.push(app(
            "Alter",
            vec!["Power".to_string(), emit_numeric_op(op)?].into(),
        )),
        Modification::Toughness(op) => out.push(app(
            "Alter",
            vec!["Toughness".to_string(), emit_numeric_op(op)?].into(),
        )),
        Modification::BaseLoyalty(_) | Modification::BaseDefense(_) => {
            // Idris's `Alter` has no dedicated loyalty/defense-BASE axis
            // distinct from the counter-driven value; not yet mapped.
            return Err(gap("Modification::BaseLoyalty/BaseDefense not yet mapped"));
        }
        Modification::SwitchPowerToughness => {
            out.push("(Alter Power (Set (StatOf This Toughness)))".to_string());
        }
        Modification::Colors(op) => out.push(app(
            "Alter",
            vec![
                "Colors".to_string(),
                emit_collection_op(op, |c| Ok(emit_color(*c)))?,
            ]
            .into(),
        )),
        Modification::CardTypes(op) => out.push(app(
            "Alter",
            vec![
                "Types".to_string(),
                emit_collection_op(op, |t| emit_type_name(t.as_str()))?,
            ]
            .into(),
        )),
        Modification::Subtypes(op) => out.push(app(
            "Alter",
            vec![
                "Subtypes".to_string(),
                emit_collection_op(op, |s: &deckmaste_semantics::SubtypeRef| {
                    emit_subtype_ref(s.as_str())
                })?,
            ]
            .into(),
        )),
        Modification::Supertypes(op) => out.push(app(
            "Alter",
            vec![
                "Supertypes".to_string(),
                emit_collection_op(op, |s| Ok(emit_supertype(*s)))?,
            ]
            .into(),
        )),
        Modification::GainAbility(ability) => {
            out.push(app("GrantAbility", vec![emit_ability(ability)?].into()));
        }
        Modification::LoseAbility(name) => {
            let spec = keywordspec_idris(name.as_str()).ok_or_else(|| {
                gap(format!(
                    "unmapped keyword in LoseAbility: {}",
                    name.as_str()
                ))
            })?;
            out.push(app("LoseKeyword", vec![spec].into()));
        }
        Modification::LoseAllAbilities => out.push("LoseAbilities".to_string()),
        Modification::CantHaveAbility(_) => {
            return Err(gap(
                "Modification::CantHaveAbility has no Idris counterpart",
            ));
        }
        Modification::SetController(r) => {
            out.push(app("GainControl", vec![emit_reference(r)?].into()));
        }
        Modification::SetText(_) => {
            return Err(gap(
                "Modification::SetText has no Idris counterpart (Idris only has ChangeText's word-class swap)",
            ));
        }
        Modification::AllCreatureTypes => {
            return Err(gap("Modification::AllCreatureTypes not yet mapped"));
        }
        Modification::BecomeBasicLandType(_) => {
            return Err(gap("Modification::BecomeBasicLandType not yet mapped"));
        }
        Modification::Several(inner) => {
            for m in inner.iter() {
                emit_modification_ops(m, out)?;
            }
        }
        Modification::Expanded(_) => {
            return Err(gap(
                "unexpanded Modification macro invocation remained after expand_all",
            ));
        }
    }
    Ok(())
}

fn emit_modification(m: &Modification) -> R {
    let mut ops = Vec::new();
    emit_modification_ops(m, &mut ops)?;
    match ops.len() {
        0 => Err(gap("Modification produced no Idris ops")),
        1 => Ok(ops.into_iter().next().expect("len checked")),
        _ => Ok(app("ApplyAll", vec![ilist(ops.into())].into())),
    }
}

// ===========================================================================
// Copy grammar ([CR#707]) — the shared source/exception lowering the token
// copy (`Copy`'s mod list), becomes-a-copy (`Modify … BecomeCopyOf`), and
// enters-as-a-copy carriers reuse. Exceptions are SEPARATE higher-layer mods,
// never bundled into the copy constructor's own args (Idris doctrine).
// ===========================================================================

/// The copied object ([CR#707.1]): a referenced object, or — for the
/// graveyard/exile self-copy keywords (Embalm/Eternalize copy the exiled
/// card) — the copying card itself, the Idris self reference `This`.
fn emit_copy_source(src: &CopySource) -> R {
    match src {
        CopySource::Object(r) => emit_reference(r),
        CopySource::SelfCard => Ok("This".to_string()),
    }
}

/// The "…, except …" characteristic changes on a copy ([CR#707.9]) as sibling
/// Idris `Modification`s (flattened via `emit_modification_ops`, so a Rust
/// `Several` splices its members in). Only `CopyException::Modify` has an Idris
/// analog; `Retain`/`AdditionalEffect` ([CR#707.9c..707.9e]) are deferred.
fn emit_copy_exception_mods(exceptions: &[CopyException]) -> Result<Arc<[String]>, Gap> {
    let mut mods = Vec::new();
    for exc in exceptions {
        match exc {
            CopyException::Modify(m) => emit_modification_ops(m, &mut mods)?,
            CopyException::Retain(_) | CopyException::AdditionalEffect(_) => {
                return Err(gap(
                    "CopyException::Retain/AdditionalEffect ([CR#707.9c..707.9e]) has no Idris \
                     analog — deferred (idris-copy-retain-additionaleffect)",
                ));
            }
        }
    }
    Ok(mods.into())
}

/// The copiable-value `Modification` a becomes-a-copy / enters-as-a-copy effect
/// installs ([CR#707.2], layer 1): `BecomeCopyOf <src>` bare, or — with
/// "except" characteristic changes ([CR#707.9]) — `ApplyAll [BecomeCopyOf
/// <src>, <exc mods…>]`, each exception a SIBLING higher-layer mod, never
/// bundled into `BecomeCopyOf` (Semantics.idr's documented stance). The
/// token-copy `Copy` Action does NOT go through here — its own
/// modification-list arg carries the exceptions directly (no `BecomeCopyOf`,
/// since the `Copy` verb itself is the copy).
fn emit_copy_modification(cs: &CopySpec) -> R {
    let become_copy = app("BecomeCopyOf", vec![emit_copy_source(&cs.source)?].into());
    let exc_mods = emit_copy_exception_mods(&cs.exceptions)?;
    if exc_mods.is_empty() {
        return Ok(become_copy);
    }
    let mut mods = Vec::with_capacity(1 + exc_mods.len());
    mods.push(become_copy);
    mods.extend(exc_mods.iter().cloned());
    Ok(app("ApplyAll", vec![ilist(mods.into())].into()))
}

/// A token copy ([CR#707.2]) inside `Action::Create`: the `Copy` Action
/// carrying the source plus its "except …" copiable-value changes ([CR#707.9])
/// in its own modification-list arg — NOT `createTokenAttacking`, and NO
/// `BecomeCopyOf` wrapper (the `Copy` verb itself is the copy). It has no count
/// slot (multiplicity rides a `Batch` owned by `emit_effect`) and no
/// attacking-rider slot, so a non-empty rider list gaps.
fn emit_create_token_copy(cs: &CopySpec, riders: &[EnterRider]) -> R {
    if !riders.is_empty() {
        return Err(gap(
            "token-copy Create with enter riders has no Idris Copy slot",
        ));
    }
    Ok(app(
        "Copy",
        vec![
            emit_copy_source(&cs.source)?,
            ilist(emit_copy_exception_mods(&cs.exceptions)?),
        ]
        .into(),
    ))
}

fn emit_player_mod(m: &PlayerMod) -> R {
    Ok(match m {
        PlayerMod::SetTo(attr, c) => app(
            "SetTo",
            vec![emit_player_attr(*attr), emit_count(c)?].into(),
        ),
        PlayerMod::Raise(attr, c) => app(
            "Raise",
            vec![emit_player_attr(*attr), emit_count(c)?].into(),
        ),
        PlayerMod::Lower(attr, c) => app(
            "Lower",
            vec![emit_player_attr(*attr), emit_count(c)?].into(),
        ),
        PlayerMod::NoMax(attr) => app("NoMax", vec![emit_player_attr(*attr)].into()),
    })
}

fn emit_player_attr(a: PlayerAttr) -> String {
    match a {
        PlayerAttr::Life => "Life",
        PlayerAttr::HandSize => "HandSize",
        PlayerAttr::HandSizeLimit => "HandSizeLimit",
        PlayerAttr::LandPlaysPerTurn => "LandPlaysPerTurn",
    }
    .to_string()
}

fn emit_cost_change(c: &CostChange) -> R {
    Ok(match c {
        CostChange::Reduce(cs) => app("Reduce", vec![emit_cost_components_as_mana(cs)?].into()),
        CostChange::Increase(cs) => app("Increase", vec![emit_cost_components_as_mana(cs)?].into()),
        CostChange::Additional { components } => {
            let mut out = Vec::with_capacity(components.len());
            for c in components.iter() {
                out.push(emit_cost_component(c)?);
            }
            app("Additional", vec![ilist(out.into())].into())
        }
        CostChange::Scaled { change, times } => app(
            "ScaledBy",
            vec![emit_cost_change(change)?, emit_count(times)?].into(),
        ),
    })
}

/// `Reduce`/`Increase` in Idris are mana-only (`ManaCost`); the common Rust
/// shape is a single `CostComponent::Mana(cost)`.
fn emit_cost_components_as_mana(cs: &[CostComponent]) -> R {
    match cs {
        [CostComponent::Mana(cost)] => Ok(emit_mana_cost(cost)),
        _ => Err(gap(
            "Reduce/Increase with a non-single-Mana component list not yet mapped",
        )),
    }
}

fn emit_replacement(r: &deckmaste_semantics::Replacement) -> R {
    use deckmaste_semantics::Replacement as Repl;
    Ok(match r {
        Repl::Instead { would, instead } => {
            let (kinds, facets) = emit_event_filter(would)?;
            app(
                "Replaces",
                vec![event_query(&kinds, &facets), emit_effect(instead)?].into(),
            )
        }
        Repl::Also { would, also } => {
            let (kinds, facets) = emit_event_filter(would)?;
            app(
                "Also",
                vec![event_query(&kinds, &facets), emit_effect(also)?].into(),
            )
        }
        Repl::Skip { .. } => {
            return Err(gap(
                "Replacement::Skip not yet mapped (Idris models a skip as Replaces with an empty body)",
            ));
        }
        Repl::Expanded(_) => {
            return Err(gap(
                "unexpanded Replacement macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_prevention(p: &deckmaste_semantics::Prevention) -> R {
    use deckmaste_semantics::Prevention as P;
    Ok(match p {
        P::PreventAll { from, to, .. } => {
            let q = event_query(
                &["(DealDamage Nothing)".to_string()],
                &damage_facets(from, to)?,
            );
            app("Replaces", vec![q, "(Sequentially [])".to_string()].into())
        }
        P::PreventNext { n, from, to, .. } => {
            let q = event_query(
                &["(DealDamage Nothing)".to_string()],
                &damage_facets(from, to)?,
            );
            // `replacesLimit` exposes the (default-`Unlimited`) `limit`
            // positionally.
            app(
                "replacesLimit",
                vec![
                    q,
                    "(Sequentially [])".to_string(),
                    format!("(UpTo {})", emit_count(n)?),
                ]
                .into(),
            )
        }
        P::PreventNextInstance { .. } => {
            return Err(gap(
                "Prevention::PreventNextInstance not yet mapped (instance-vs-amount limiting has no Idris ReplaceLimit shape)",
            ));
        }
    })
}

fn damage_facets(from: &Predicate, to: &Predicate) -> Result<Arc<[String]>, Gap> {
    let mut facets = Vec::new();
    if !matches!(from, Predicate::Any) {
        facets.push(app("Agent", vec![emit_filter(from)?].into()));
    }
    if !matches!(to, Predicate::Any) {
        facets.push(app("Patient", vec![emit_filter(to)?].into()));
    }
    Ok(facets.into())
}

fn emit_static_effect(se: &StaticEffect) -> R {
    Ok(match se {
        StaticEffect::Modify(r, m) => app(
            "Modify",
            vec![emit_reference(r)?, emit_modification(m)?].into(),
        ),
        StaticEffect::Each(sel, inner) => app(
            "Each",
            vec![
                format!("(Existing {})", emit_selection(sel)?),
                emit_static_effect(inner)?,
            ]
            .into(),
        ),
        StaticEffect::Conditionally(cond, inner) => app(
            "While",
            vec![emit_condition(cond)?, emit_static_effect(inner)?].into(),
        ),
        StaticEffect::Deontic(d) => emit_deontic(d)?,
        StaticEffect::CostModifier { of, change } => app(
            "CostModifier",
            vec![emit_filter(of)?, emit_cost_change(change)?].into(),
        ),
        StaticEffect::CostOption(oc) => {
            let mut costs = Vec::with_capacity(oc.components.len());
            for c in oc.components.iter() {
                costs.push(emit_cost_component(c)?);
            }
            let repeatable = if oc.repeatable { "True" } else { "False" };
            app(
                "costOptionRep",
                vec![
                    ilit(oc.tag.as_str()),
                    ilist(costs.into()),
                    repeatable.to_string(),
                ]
                .into(),
            )
        }
        StaticEffect::TriggerMultiplier {
            cause,
            extra,
            affected,
        } => {
            let (kinds, facets) = emit_event_filter(cause)?;
            app(
                "triggerMultiplierFor",
                vec![
                    event_query(&kinds, &facets),
                    emit_count(extra)?,
                    emit_filter(affected)?,
                ]
                .into(),
            )
        }
        StaticEffect::ModifyPlayer(r, m) => app(
            "ModifyPlayer",
            vec![emit_reference(r)?, emit_player_mod(m)?].into(),
        ),
        StaticEffect::Replacement(r) => emit_replacement(r)?,
        StaticEffect::Prevention(p) => emit_prevention(p)?,
        StaticEffect::CantPrevent { .. } => {
            return Err(gap("StaticEffect::CantPrevent has no Idris counterpart"));
        }
        StaticEffect::SpendAsThough { .. } => {
            return Err(gap("StaticEffect::SpendAsThough has no Idris counterpart"));
        }
        // Idris `AsThough : Condition -> StaticEffect -> StaticEffect`
        // ([CR#609.4]): the Rust `Counterfactual { premise, then }` re-emits as
        // `AsThough (Matches This premise) <then>` — the premise is the
        // counterfactual `Matches This (Not (Has Hexproof))`, wrapping the inner
        // permission `then` as its `Can` clause (Glaring Spotlight).
        StaticEffect::AsThough(deckmaste_semantics::AsThough::Counterfactual { premise, then }) => {
            let premise_cond = app(
                "Matches",
                vec![
                    emit_reference(&deckmaste_semantics::Reference::This)?,
                    emit_filter(premise)?,
                ]
                .into(),
            );
            app("AsThough", vec![premise_cond, emit_deontic(then)?].into())
        }
        StaticEffect::AsThough(deckmaste_semantics::AsThough::Expanded(_)) => {
            return Err(gap(
                "StaticEffect::AsThough macro provenance (Expanded) is not re-emitted",
            ));
        }
        StaticEffect::Sba { when, then } => app(
            "Sba",
            vec![emit_condition(when)?, emit_effect(then)?].into(),
        ),
        StaticEffect::OutcomeGate { who, gate } => {
            let g = match gate {
                deckmaste_semantics::OutcomeGateKind::CantLose => "CantLose",
                deckmaste_semantics::OutcomeGateKind::CantWin => "CantWin",
            };
            app("OutcomeGate", vec![g.to_string(), emit_filter(who)?].into())
        }
        StaticEffect::CantHappen(ef) => {
            let (kinds, facets) = emit_event_filter(ef)?;
            app("CantHappen", vec![event_query(&kinds, &facets)].into())
        }
        StaticEffect::ReplaceRoll {
            query,
            extra,
            ignore,
        } => {
            let (kinds, facets) = emit_event_filter(query)?;
            app(
                "ReplaceRoll",
                vec![
                    event_query(&kinds, &facets),
                    emit_count(extra)?,
                    emit_ignore_rule(ignore),
                ]
                .into(),
            )
        }
        StaticEffect::PayPips(class, act) => {
            let class_s = match class {
                deckmaste_semantics::PipClass::Generic => "Generic".to_string(),
                // Idris `PipClass.Colored` is nullary: it marks "a colored pip,
                // matched via the pay predicate" — the specific color is carried
                // by the `TapToPay`/`ExileToPay` filter (`HasChar Colors <c>`),
                // so the Rust-side `Color` payload is intentionally not forwarded.
                deckmaste_semantics::PipClass::Colored(_) => "Colored".to_string(),
            };
            let act_s = match act {
                deckmaste_semantics::PayAct::TapToPay(f) => {
                    app("TapToPay", vec![emit_filter(f)?].into())
                }
                deckmaste_semantics::PayAct::ExileToPay(f) => {
                    app("ExileToPay", vec![emit_filter(f)?].into())
                }
            };
            app("PayPips", vec![class_s, act_s].into())
        }
        // [CR#707.4] "becomes a copy of" — core-copy-grammar Task 6's
        // grammar-only seam; the Idris mapping is Task 8's.
        // becomes-a-copy ([CR#707.4], layer 1) — `Modify <who> (BecomeCopyOf
        // <src>)`, or `Modify <who> (ApplyAll [BecomeCopyOf <src>, <except
        // mods…>])` when the copy carries "except …" characteristic changes
        // ([CR#707.9]) as sibling higher-layer mods. NO new Idris constructor:
        // composed from the existing `Modify`/`BecomeCopyOf`/`ApplyAll`/`Alter`.
        StaticEffect::BecomesCopy(who, cs) => app(
            "Modify",
            vec![emit_reference(who)?, emit_copy_modification(cs)?].into(),
        ),
        StaticEffect::Expanded(_) => {
            return Err(gap(
                "unexpanded StaticEffect macro invocation remained after expand_all",
            ));
        }
    })
}

/// `IgnoreLowest`/`IgnoreChosen n` — the latter's `Nat` payload emits as a
/// bare decimal literal, matching this file's other `Nat`-payload
/// conventions (e.g. `RollDice`'s `sides.to_string()`).
fn emit_ignore_rule(ir: &deckmaste_semantics::IgnoreRule) -> String {
    match ir {
        deckmaste_semantics::IgnoreRule::IgnoreLowest => "IgnoreLowest".to_string(),
        deckmaste_semantics::IgnoreRule::IgnoreChosen(n) => {
            app("IgnoreChosen", vec![n.to_string()].into())
        }
    }
}

fn emit_deontic(d: &Deontic) -> R {
    Ok(match d {
        Deontic::Cant(action) => app(
            "Constrain",
            vec!["Forbid".to_string(), emit_deed(action)?].into(),
        ),
        Deontic::Must(action) => app(
            "Constrain",
            vec!["Require".to_string(), emit_deed(action)?].into(),
        ),
        Deontic::May(action) => emit_can(action)?,
        Deontic::Gate(action, costs) => {
            let mut cs = Vec::with_capacity(costs.len());
            for c in costs.iter() {
                cs.push(emit_cost_component(c)?);
            }
            app(
                "Priced",
                vec![
                    "AtDeclaration".to_string(),
                    format!("(Costs {})", ilist(cs.into())),
                    emit_deed(action)?,
                ]
                .into(),
            )
        }
        Deontic::Expanded(_) => {
            return Err(gap(
                "unexpanded Deontic macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_can(action: &DeonticAction) -> R {
    if let DeonticAction::Cast {
        what,
        by,
        from,
        window,
        cost,
        tag,
    } = action
    {
        if let Some(alt) = cost {
            if !matches!(what, Predicate::Ref(Reference::This)) || !matches!(by, Predicate::Any) {
                return Err(gap(
                    "May(Cast) with a non-default what/by has no Idris MayCastFor counterpart",
                ));
            }
            if window.is_some() {
                return Err(gap("May(Cast{cost: Some, window: Some}) not yet mapped"));
            }
            let costs = match alt {
                deckmaste_semantics::AlternativeCost::Free => "[]".to_string(),
                deckmaste_semantics::AlternativeCost::Components(cs) => {
                    let mut out = Vec::with_capacity(cs.len());
                    for c in cs.iter() {
                        out.push(emit_cost_component(c)?);
                    }
                    ilist(out.into())
                }
            };
            // `from` defaults to `[Hand]`; `mayCastForFrom` takes it
            // positionally (leaving `tag`/`when` at their defaults).
            let from_zones = match from {
                None => "[Hand]".to_string(),
                Some(z) => format!("[{}]", emit_zone(*z)),
            };
            // With no `tag`, keep the terse `mayCastForFrom` helper (leaves
            // `tag`/`when` at their Idris defaults). With a `tag`, that
            // helper has no way to thread it through, so call `MayCastFor`
            // directly with named-field syntax for both `from` and `tag`.
            return Ok(match tag {
                None => app("mayCastForFrom", vec![costs, from_zones].into()),
                Some(t) => {
                    let spec = keywordspec_idris(t.as_str()).ok_or_else(|| {
                        gap(format!(
                            "Cast.tag keyword not in KeywordSpec table: {}",
                            t.as_str()
                        ))
                    })?;
                    format!("(MayCastFor {costs} {{from = {from_zones}, tag = Just {spec}}})")
                }
            });
        }
        // No alternative cost: a plain cast permission (flash-shaped).
        let by_pred = if matches!(by, Predicate::Any) {
            "(SameAs You)".to_string()
        } else {
            emit_filter(by)?
        };
        let what_pred = if matches!(what, Predicate::Any) {
            "(SameAs This)".to_string()
        } else {
            emit_filter(what)?
        };
        let deed = app("Enact", vec!["Cast".to_string(), by_pred, what_pred].into());
        let window_maybe = match window {
            None => "Nothing".to_string(),
            Some(deckmaste_semantics::Timing::InstantSpeed) => "(Just AsInstant)".to_string(),
            Some(deckmaste_semantics::Timing::SorcerySpeed) => "(Just AsSorcery)".to_string(),
            Some(_) => {
                return Err(gap(
                    "Timing::DuringTurn/DuringStep has no Idris Timing counterpart",
                ));
            }
        };
        return Ok(app("canWindow", vec![deed, window_maybe].into()));
    }
    Ok(app("Can", vec![emit_deed(action)?].into()))
}

/// `Deed.Enact`'s `patient : Predicate b k` carries a totally FREE `k` (no
/// function ties it to `patientScope r`, unlike `agent`'s `agentScope r`,
/// which reduces to a concrete kind since `r` is always a literal
/// constructor here) — so a default `Predicate::Any` patient can't elaborate as
/// the kind-polymorphic-empty `And []` (Idris is left with an unsolved `k`
/// hole). `Anyone` is Semantics.idr's own precedent for this
/// (`Defender = cant (Enact Attack (SameAs This) Anyone)`): a concretely
/// `APlayer`-kinded "no restriction" stand-in, since `Enact`'s `k` isn't
/// actually forced to match `patientScope` at the type level.
fn deed_patient(f: &Predicate) -> R {
    if matches!(f, Predicate::Any) {
        Ok("Anyone".to_string())
    } else {
        emit_filter(f)
    }
}

fn emit_deed(action: &DeonticAction) -> R {
    Ok(match action {
        DeonticAction::Attack { by, on } => app(
            "Enact",
            vec!["Attack".to_string(), emit_filter(by)?, deed_patient(on)?].into(),
        ),
        DeonticAction::Block { by, on, count } => match count {
            None => app(
                "Enact",
                vec!["Block".to_string(), emit_filter(by)?, deed_patient(on)?].into(),
            ),
            Some(bound) => {
                let attacker = if matches!(on, Predicate::Any) { by } else { on };
                app(
                    "BlockedBy",
                    vec![emit_filter(attacker)?, count_bound_as_quantity(bound)?].into(),
                )
            }
        },
        DeonticAction::Target { by, on } => {
            // A `source` of `Any` (plain `Hexproof()` — its `from` param
            // defaults to `Predicate::Any`) is NO source restriction, so it drops
            // cleanly: the agent is fully captured by `stack_object`. Only a
            // CONCRETE quality source (hexproof-/protection-from-[quality],
            // [CR#702.11d,702.16b]) needs an Idris counterpart it doesn't yet
            // have — that rides the AsThough/quality machinery (the Glaring
            // Spotlight followup), so it stays gapped.
            if by
                .source
                .as_ref()
                .is_some_and(|f| !matches!(f, Predicate::Any))
            {
                return Err(gap(
                    "DeedAgent.source (a CONCRETE ability-source quality, e.g. hexproof-from-[quality]) has no Idris Enact-Target counterpart",
                ));
            }
            // `agentScope Target = AnObject` is forced concretely (Target is
            // a literal constructor here), so the empty conjunction resolves
            // fine as the agent (unlike the patient above).
            let agent = match &by.stack_object {
                None => "(And [])".to_string(),
                Some(f) => emit_filter(f)?,
            };
            app(
                "Enact",
                vec!["Target".to_string(), agent, deed_patient(on)?].into(),
            )
        }
        DeonticAction::Attach { what, to } => app(
            "Enact",
            vec!["Attach".to_string(), emit_filter(what)?, deed_patient(to)?].into(),
        ),
        DeonticAction::Cast { .. } => return Err(gap("Deontic::Cant/Must(Cast) not yet mapped")),
        DeonticAction::Play { what, by, from } => {
            if from.is_some() {
                return Err(gap("DeonticAction::Play{from} not yet mapped"));
            }
            app(
                "Enact",
                vec!["Play".to_string(), emit_filter(by)?, deed_patient(what)?].into(),
            )
        }
        DeonticAction::Activate { what, by, cost } => {
            if cost.is_some() {
                return Err(gap(
                    "DeonticAction::Activate{cost} has no Idris counterpart",
                ));
            }
            app(
                "Enact",
                vec![
                    "Activate".to_string(),
                    emit_filter(by)?,
                    deed_patient(what)?,
                ]
                .into(),
            )
        }
        DeonticAction::Regenerate { by, on } => app(
            "Enact",
            vec![
                "Regenerate".to_string(),
                emit_filter(by)?,
                deed_patient(on)?,
            ]
            .into(),
        ),
        // `cant (Enact Counter spellOrAbility (SameAs This))` ([CR#701.6a]):
        // `agentScope Counter = AnObject` is forced concretely (a literal
        // constructor here), so an `Any` agent resolves as the empty
        // conjunction like `Target`'s agent.
        DeonticAction::Counter { by, on } => app(
            "Enact",
            vec!["Counter".to_string(), emit_filter(by)?, deed_patient(on)?].into(),
        ),
        // [CR#502.3] "doesn't untap": a single-patient deed with no agent
        // slot (untapping is a turn-based action, not a deed some object
        // performs), so it has no `Enact Relation agent patient` counterpart
        // in the current Idris grammar. Gapped until a card graduates and the
        // agentless-deed shape is designed on the Idris side — no card uses
        // it yet, so this never fires.
        DeonticAction::Untap { .. } => {
            return Err(gap(
                "DeonticAction::Untap has no Idris Enact counterpart yet",
            ));
        }
        DeonticAction::Expanded(_) => {
            return Err(gap(
                "unexpanded DeonticAction macro invocation remained after expand_all",
            ));
        }
    })
}

// ===========================================================================
// EventFilter -> (kinds, facets) -> EventQuery
// ===========================================================================

fn event_query(kinds: &[String], facets: &[String]) -> String {
    format!(
        "(MkEventQuery {} {})",
        ilist(kinds.to_vec().into()),
        ilist(facets.to_vec().into())
    )
}

/// The `(kinds, facets)` string lists that `MkEventQuery` bundles — the return
/// shape shared by [`emit_event_filter`] and the `merge_*` combinators.
type KindsAndFacets = (Arc<[String]>, Arc<[String]>);

/// Translate a Rust `EventFilter` master form into an Idris
/// `(kinds : List EventKind, facets : List Facet)` pair — the two lists
/// `MkEventQuery` bundles. Not a 1:1 grammar (Rust's cause-triple/amount-bound
/// refinements have no Idris counterpart), so several fields are gapped
/// rather than silently dropped.
// One flat per-EventFilter-master-form dispatch, matching the project's
// existing style for this shape (clippy.toml: "the engine has several flat
// per-variant match dispatch functions ... that read better whole than
// carved into helpers"); this one legitimately runs past the 150-line bar.
#[expect(
    clippy::too_many_lines,
    reason = "one match arm per EventFilter master form; splitting would scatter the kind/facet mapping this function documents as a whole"
)]
fn emit_event_filter(ef: &EventFilter) -> Result<KindsAndFacets, Gap> {
    Ok(match ef {
        EventFilter::ZoneChange {
            what,
            from,
            to,
            cause,
        } => {
            if cause.is_some() {
                return Err(gap(
                    "EventFilter::ZoneChange{cause} not yet mapped (Idris EventKind has no cause coordinate)",
                ));
            }
            let kind = format!("(ZoneChanged {} {})", opt_zone(*from), opt_zone(*to));
            let mut facets = Vec::new();
            if !matches!(what, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(what)?].into()));
            }
            (vec![kind].into(), facets.into())
        }
        EventFilter::Damage {
            source,
            to,
            combat,
            amount,
        } => {
            if amount.is_some() {
                return Err(gap(
                    "EventFilter::Damage{amount} not yet mapped (no Idris amount-bound facet)",
                ));
            }
            let kind = format!("(DealDamage {})", opt_bool(*combat));
            let mut facets = Vec::new();
            if !matches!(source, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(source)?].into()));
            }
            if !matches!(to, Predicate::Any) {
                facets.push(app("Patient", vec![emit_filter(to)?].into()));
            }
            (vec![kind].into(), facets.into())
        }
        EventFilter::LifeGained { who, amount } => {
            reject_amount(amount)?;
            (vec!["GainLife".to_string()].into(), actor_facet(who)?)
        }
        EventFilter::LifeLost { who, amount } => {
            reject_amount(amount)?;
            (vec!["LoseLife".to_string()].into(), actor_facet(who)?)
        }
        EventFilter::Drawn { who, amount } => {
            reject_amount(amount)?;
            (vec!["Draw".to_string()].into(), actor_facet(who)?)
        }
        // The named `Act`-fact filter lowers to the verb's `EventKind`
        // (`Destroy`/`Discard`/`Draw`/`Mill`/`Scry`/`Surveil`/
        // `Fateseal`/`Fight` — mostly [CR#701] keyword actions, plus `Draw`,
        // which is [CR#121.1]) — the verb pins the kind, the performer `who` rides
        // the `Actor` facet and the affected object `on` the `Agent` facet
        // ([CR#616.1,701.9c] name-fact matching). `cause` (agency/cost narrowing —
        // "cycles" as a discard) has no Idris coordinate, so a narrowed filter is
        // a gap.
        EventFilter::Act {
            verb,
            who,
            on,
            cause,
        } => {
            if cause.is_some() {
                return Err(gap(
                    "EventFilter::Act{cause} has no Idris EventKind coordinate (agency/cost narrowing)",
                ));
            }
            (
                vec![act_event_kind(verb.as_str())?].into(),
                actor_agent_facets(who, on)?,
            )
        }
        EventFilter::CounterPlaced {
            kind,
            on,
            amount,
            cause,
        } => {
            reject_amount(amount)?;
            if kind.is_some() {
                return Err(gap(
                    "EventFilter::CounterPlaced{kind} not yet mapped (Idris PutCounters carries no kind)",
                ));
            }
            if cause.is_some() {
                return Err(gap(
                    "EventFilter::CounterPlaced{cause} has no Idris EventKind coordinate (agency/cost narrowing)",
                ));
            }
            (vec!["PutCounters".to_string()].into(), agent_facet(on)?)
        }
        EventFilter::CounterRemoved {
            kind,
            on,
            amount,
            cause,
        } => {
            reject_amount(amount)?;
            if kind.is_some() {
                return Err(gap("EventFilter::CounterRemoved{kind} not yet mapped"));
            }
            if cause.is_some() {
                return Err(gap(
                    "EventFilter::CounterRemoved{cause} has no Idris EventKind coordinate (agency/cost narrowing)",
                ));
            }
            (vec!["RemoveCounters".to_string()].into(), agent_facet(on)?)
        }
        EventFilter::Cast { who, what } => (
            vec!["(Begins Cast)".to_string()].into(),
            actor_agent_facets(who, what)?,
        ),
        // [CR#707.10]: the copy-on-stack plumbing lands here (P0.W4 seam);
        // no Idris `EventKind` counterpart exists yet.
        EventFilter::Copied { .. } => {
            return Err(gap(
                "EventFilter::Copied has no Idris EventKind counterpart yet ([CR#707.10])",
            ));
        }
        EventFilter::Played { who, what } => (
            vec!["(Begins Play)".to_string()].into(),
            actor_agent_facets(who, what)?,
        ),
        EventFilter::ActivatedAb { who, what } => (
            vec!["(Begins Activate)".to_string()].into(),
            actor_agent_facets(who, what)?,
        ),
        EventFilter::ManaAbilityActivated { what, by } => (
            vec!["ManaAbilityActivated".to_string()].into(),
            actor_agent_facets(by, what)?,
        ),
        EventFilter::ManaProduced { what, by } => (
            vec!["ManaProduced".to_string()].into(),
            actor_agent_facets(by, what)?,
        ),
        EventFilter::ManaAdded { what, by } => (
            vec!["ManaAdded".to_string()].into(),
            actor_agent_facets(by, what)?,
        ),
        EventFilter::AttackDeclared { by, against } => {
            if !matches!(against, Predicate::Any) {
                return Err(gap(
                    "AttackDeclared{against} not yet mapped (no Idris facet for the defending player)",
                ));
            }
            (vec!["(Begins Attack)".to_string()].into(), agent_facet(by)?)
        }
        EventFilter::BlockDeclared { by, of } => {
            if !matches!(of, Predicate::Any) {
                return Err(gap("BlockDeclared{of} not yet mapped"));
            }
            (vec!["(Begins Block)".to_string()].into(), agent_facet(by)?)
        }
        EventFilter::Attached { what, to } => {
            if !matches!(to, Predicate::Any) {
                return Err(gap(
                    "Attached{to} not yet mapped (no Idris patient facet for Begins Attach)",
                ));
            }
            (
                vec!["(Begins Attach)".to_string()].into(),
                agent_facet(what)?,
            )
        }
        EventFilter::StateBecame { of, becomes, cause } => {
            if cause.is_some() {
                return Err(gap(
                    "EventFilter::StateBecame{cause} has no Idris EventKind coordinate (agency/cost narrowing)",
                ));
            }
            let kind = match becomes {
                deckmaste_semantics::StateChange::Tapped => "(Becomes Tapped)".to_string(),
                deckmaste_semantics::StateChange::Untapped => "(Becomes Untapped)".to_string(),
                deckmaste_semantics::StateChange::Phased(deckmaste_semantics::Phasing::Out) => {
                    "(Becomes PhasedOut)".to_string()
                }
                deckmaste_semantics::StateChange::Phased(deckmaste_semantics::Phasing::In) => {
                    return Err(gap(
                        "StateChange::Phased(In) has no Idris ObjectState transition (only PhasedOut)",
                    ));
                }
                deckmaste_semantics::StateChange::TurnedFace(deckmaste_semantics::Face::Down) => {
                    "(Becomes FaceDown)".to_string()
                }
                deckmaste_semantics::StateChange::TurnedFace(deckmaste_semantics::Face::Up) => {
                    return Err(gap(
                        "StateChange::TurnedFace(Up) has no Idris ObjectState transition",
                    ));
                }
                deckmaste_semantics::StateChange::Transformed => {
                    return Err(gap(
                        "StateChange::Transformed has no Idris ObjectState transition",
                    ));
                }
            };
            (vec![kind].into(), agent_facet(of)?)
        }
        EventFilter::BecomesTarget { .. } => {
            return Err(gap(
                "EventFilter::BecomesTarget has no Idris EventKind counterpart",
            ));
        }
        EventFilter::StepBegins { at, whose } => {
            let kind = format!("(BeginStep {})", emit_phase_step(*at)?);
            let facet = match whose {
                deckmaste_semantics::WhoseTurn::EachPlayers => None,
                deckmaste_semantics::WhoseTurn::Your => {
                    Some("(Whenever (TurnOf (SameAs You)))".to_string())
                }
                deckmaste_semantics::WhoseTurn::AnOpponents => {
                    Some("(Whenever (TurnOf OpponentOf))".to_string())
                }
            };
            (vec![kind].into(), facet.into_iter().collect())
        }
        EventFilter::ControlChanged { of, to } => {
            let mut facets = Vec::new();
            if !matches!(of, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(of)?].into()));
            }
            if !matches!(to, Predicate::Any) {
                facets.push(app("Actor", vec![emit_filter(to)?].into()));
            }
            (vec!["GainControl".to_string()].into(), facets.into())
        }
        EventFilter::DesignationChanged { .. } => {
            return Err(gap(
                "EventFilter::DesignationChanged has no Idris EventKind counterpart",
            ));
        }
        // [CR#701.24a,701.20a]: the T5 exposure rows — no Idris `EventKind`
        // counterpart exists for either yet (Psychic Surgery is authorable
        // via engine filter matching alone; Idris coverage is a later seam).
        EventFilter::Shuffled { .. } => {
            return Err(gap(
                "EventFilter::Shuffled has no Idris EventKind counterpart yet",
            ));
        }
        EventFilter::Revealed { .. } => {
            return Err(gap(
                "EventFilter::Revealed has no Idris EventKind counterpart yet",
            ));
        }
        EventFilter::TokenCreated { what, by } => {
            let mut facets = Vec::new();
            if !matches!(what, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(what)?].into()));
            }
            if !matches!(by, Predicate::Any) {
                facets.push(app("Actor", vec![emit_filter(by)?].into()));
            }
            (vec!["CreateToken".to_string()].into(), facets.into())
        }
        EventFilter::Used { .. } => {
            return Err(gap("EventFilter::Used has no Idris EventKind counterpart"));
        }
        // [CR#705.1,705.2]: `won` is the "won/lost the flip" cap Idris's own
        // `FlipCoin : Maybe Bool -> EventKind` carries on the KIND itself
        // (like `ZoneChanged`'s zones) — the wildcarded `Nothing` = any
        // flip, `Just True`/`Just False` = won/lost (Chance Encounter's
        // trigger: `MkEventQuery [FlipCoin (Just True)] [Actor you]`).
        EventFilter::CoinFlipped { by, won } => (
            vec![format!("(FlipCoin {})", opt_bool(*won))].into(),
            actor_facet(by)?,
        ),
        EventFilter::DiceRolled { by } => (vec!["RollDice".to_string()].into(), actor_facet(by)?),
        // [CR#106.12,106.12a]: the tapped land is the Agent, its controller the
        // Actor (Dictate of Karametra's trigger: `MkEventQuery [TapForMana]
        // [Actor you, Agent (hasType Land)]`).
        EventFilter::TapForMana { what, by } => {
            let mut facets = actor_facet(by)?.to_vec();
            if !matches!(what, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(what)?].into()));
            }
            (vec!["TapForMana".to_string()].into(), facets.into())
        }
        // [CR#901.9,901.9d]: the fixed six-face planar die — `face`
        // wildcards like `ZoneChanged`'s zones (`Nothing` = any face).
        EventFilter::RollPlanarDie { by, face } => (
            vec![format!("(RollPlanarDie {})", opt_planar_face(*face))].into(),
            actor_facet(by)?,
        ),
        EventFilter::AllOf(fs) => merge_all_of(fs)?,
        EventFilter::OneOf(fs) => merge_one_of(fs)?,
        EventFilter::Not(_) => return Err(gap("EventFilter::Not not yet mapped")),
        EventFilter::OneOrMore(_) => return Err(gap("EventFilter::OneOrMore not yet mapped")),
        EventFilter::Nth { .. } => return Err(gap("EventFilter::Nth not yet mapped")),
        EventFilter::When(inner, cond) => {
            let (kinds, facets) = emit_event_filter(inner)?;
            let mut facets = facets.to_vec();
            facets.push(app("Whenever", vec![emit_condition(cond)?].into()));
            (kinds, facets.into())
        }
        EventFilter::Within(..) => {
            return Err(gap(
                "EventFilter::Within not yet mapped (a history-lane refinement, not valid on a live trigger)",
            ));
        }
        EventFilter::Before(..) => {
            return Err(gap(
                "EventFilter::Before not yet mapped (the storm history-lane cast-order refinement, \
                 reached only under the already-gapped Count::EventCount)",
            ));
        }
        EventFilter::Expanded(_) => {
            return Err(gap(
                "unexpanded EventFilter macro invocation remained after expand_all",
            ));
        }
    })
}

fn reject_amount(amount: &Option<CountBound>) -> Result<(), Gap> {
    if amount.is_some() {
        Err(gap(
            "EventFilter{amount} not yet mapped (no Idris amount-bound facet on an EventQuery)",
        ))
    } else {
        Ok(())
    }
}

fn opt_zone(z: Option<deckmaste_semantics::Zone>) -> String {
    match z {
        None => "Nothing".to_string(),
        Some(z) => format!("(Just {})", emit_zone(z)),
    }
}

fn opt_bool(b: Option<bool>) -> String {
    match b {
        None => "Nothing".to_string(),
        Some(true) => "(Just True)".to_string(),
        Some(false) => "(Just False)".to_string(),
    }
}

fn emit_planar_face(f: deckmaste_semantics::PlanarFace) -> &'static str {
    match f {
        deckmaste_semantics::PlanarFace::Blank => "Blank",
        deckmaste_semantics::PlanarFace::Chaos => "Chaos",
        deckmaste_semantics::PlanarFace::Planeswalker => "Planeswalker",
    }
}

fn opt_planar_face(f: Option<deckmaste_semantics::PlanarFace>) -> String {
    match f {
        None => "Nothing".to_string(),
        Some(f) => format!("(Just {})", emit_planar_face(f)),
    }
}

fn actor_facet(who: &Predicate) -> Result<Arc<[String]>, Gap> {
    if matches!(who, Predicate::Any) {
        Ok(vec![].into())
    } else {
        Ok(vec![app("Actor", vec![emit_filter(who)?].into())].into())
    }
}

fn agent_facet(what: &Predicate) -> Result<Arc<[String]>, Gap> {
    if matches!(what, Predicate::Any) {
        Ok(vec![].into())
    } else {
        Ok(vec![app("Agent", vec![emit_filter(what)?].into())].into())
    }
}

fn actor_agent_facets(who: &Predicate, what: &Predicate) -> Result<Arc<[String]>, Gap> {
    let mut facets = actor_facet(who)?.to_vec();
    facets.extend(agent_facet(what)?.iter().cloned());
    Ok(facets.into())
}

/// Map an `Act`-fact verb name to its Idris `EventKind` constructor for an
/// `EventFilter::Act` lowering. The verb-named kinds
/// (`Destroy`/`Discard`/`Draw` and the added
/// `Mill`/`Scry`/`Surveil`/`Fateseal`/ `Fight`) sit beside one another in
/// `Semantics.idr`'s `EventKind`; an unrecognized verb is a gap.
///
/// This vocabulary is the set of named action FACTS the engine commits, which
/// is BROADER than the keyword actions [CR#701] enumerates: `Draw` belongs here
/// as the name-fact a "whenever you draw a card" trigger reads ([CR#121.1]),
/// even though drawing is a game action ([CR#121]) and not a keyword action.
fn act_event_kind(verb: &str) -> Result<String, Gap> {
    match verb {
        "Destroy" | "Discard" | "Draw" | "Mill" | "Scry" | "Surveil" | "Fateseal" | "Fight" => {
            Ok(verb.to_string())
        }
        other => Err(gap(format!(
            "EventFilter::Act verb {other} has no Idris EventKind counterpart"
        ))),
    }
}

/// `AllOf` merges constituents that share the same emitted kind list,
/// concatenating facets (the same occurrence, refined further).
fn merge_all_of(fs: &[EventFilter]) -> Result<KindsAndFacets, Gap> {
    let mut kinds: Option<Arc<[String]>> = None;
    let mut facets = Vec::new();
    for f in fs {
        let (k, fa) = emit_event_filter(f)?;
        if !k.is_empty() {
            match &kinds {
                None => kinds = Some(k),
                Some(existing) if *existing == k => {}
                Some(_) => return Err(gap("AllOf constituents specify conflicting event kinds")),
            }
        }
        facets.extend(fa.iter().cloned());
    }
    Ok((kinds.unwrap_or_default(), facets.into()))
}

/// `OneOf` widens the kind list when every disjunct shares the same facets
/// (the common case: "whenever a creature attacks or blocks").
fn merge_one_of(fs: &[EventFilter]) -> Result<KindsAndFacets, Gap> {
    let mut kinds = Vec::new();
    let mut shared_facets: Option<Arc<[String]>> = None;
    for f in fs {
        let (k, fa) = emit_event_filter(f)?;
        kinds.extend(k.iter().cloned());
        match &shared_facets {
            None => shared_facets = Some(fa),
            Some(existing) if *existing == fa => {}
            Some(_) => {
                return Err(gap(
                    "OneOf disjuncts have differing facets, unrepresentable as one shared EventQuery",
                ));
            }
        }
    }
    Ok((kinds.into(), shared_facets.unwrap_or_default()))
}

// ===========================================================================
// OneShotEffect
// ===========================================================================

/// Fold `Batch(n, Act(DrawCard(who)))` into Idris's count-carrying
/// `Action.DrawCard` — `Act (DrawCard who n)`. `None` for any other `Batch`
/// body, which then emits the ordinary shell form.
///
/// Rust models "draw N" as N SEQUENTIAL single-card draws, because the
/// individual card draw is the replaceable unit ([CR#121.2]) and the `Batch` is
/// the instruction level a count-referring replacement bites ([CR#121.2a]).
/// Idris's `Action.Draw` instead carries the count on the verb, and its
/// `actionIntro` derives the card/amount anaphora FROM that count — so "draw
/// three cards, then gain THAT MUCH life" needs the 3 there, not on an
/// enclosing `Batch`. Folding keeps ONE draw spelling per side.
///
/// (Drawing never rides the `Composite`/`KeywordActionSpec` lane at all: it is
/// [CR#121], not one of the keyword actions [CR#701] enumerates, and [CR#121.5]
/// makes it irreducible — there is no body to reconstruct coordinates from.)
fn emit_batched_draw(n: &Count, body: &OneShotEffect) -> Result<Option<String>, Gap> {
    let OneShotEffect::Act(Action::DrawCard(who)) = peel_os(body) else {
        return Ok(None);
    };
    Ok(Some(app(
        "Act",
        vec![app(
            "DrawCard",
            vec![emit_reference(who)?, emit_count(n)?].into(),
        )]
        .into(),
    )))
}

fn emit_effect(e: &OneShotEffect) -> R {
    Ok(match e {
        // [CR#104.2b,104.3e]: win/lose live in Rust as `Action::
        // {WinGame,LoseGame}` (reached via `Act`), but Idris's
        // `Conclude : Outcome b -> OneShotEffect b` is its OWN top-level
        // `OneShotEffect` constructor, never wrapped in `Act` — the bridge
        // `emit_action`'s own `WinGame`/`LoseGame` arms already build
        // `(Conclude (WinGame/LoseGame patient))` whole; this arm must not
        // re-wrap that in `Act`.
        OneShotEffect::Act(a @ (Action::WinGame(_) | Action::LoseGame(_))) => emit_action(a)?,
        // Token copy ([CR#707.2]): the `Copy` Action carries no multiplicity of
        // its own (unlike `createTokenAttacking`'s count arg), so "create N
        // copies" wraps the single-copy `Act` in a `Batch` — the Mill/Draw
        // per-unit precedent. `emit_action` builds the bare `(Copy src mods)`;
        // count == 1 needs no `Batch`.
        OneShotEffect::Act(
            a @ Action::Create {
                count,
                token: TokenSpec::Copy(_),
                ..
            },
        ) => {
            let single = app("Act", vec![emit_action(a)?].into());
            if count.literal_value() == Some(1) {
                single
            } else {
                app("Batch", vec![emit_count(count)?, single].into())
            }
        }
        OneShotEffect::Act(a) => app("Act", vec![emit_action(a)?].into()),
        OneShotEffect::Sequentially(es) => {
            app("Sequentially", vec![map_list(es, emit_effect)?].into())
        }
        // One pre-application snapshot, one batch ([CR#701.14a]) — a plain list
        // (no `SeqList` threading, unlike `Sequentially`). Fight's guarded body
        // rides this.
        OneShotEffect::Simultaneously(es) => {
            app("Simultaneously", vec![map_list(es, emit_effect)?].into())
        }
        OneShotEffect::Continuously(c) => app(
            "Continuously",
            vec![emit_duration(&c.duration)?, emit_static_effect(&c.effect)?].into(),
        ),
        OneShotEffect::Until(duration, parts) => {
            let d = emit_duration(duration)?;
            let mut wrapped = Vec::with_capacity(parts.len());
            for p in parts.iter() {
                wrapped.push(app(
                    "Continuously",
                    vec![d.clone(), emit_static_effect(p)?].into(),
                ));
            }
            app("Sequentially", vec![ilist(wrapped.into())].into())
        }
        OneShotEffect::Label { .. } => {
            return Err(gap(
                "OneShotEffect::Label has no Idris OneShotEffect counterpart",
            ));
        }
        OneShotEffect::SeparatePiles(sp) => emit_separate_piles(sp)?,
        OneShotEffect::ChoosePile(cp) => emit_choose_pile(cp)?,
        // The collapsed `MayPay`/`MustPay` shape ([CR#118.12a]): `effect` is
        // `Pay(cost)`, so it routes through `mayPayCostBy`, which carries
        // `who` as a required positional arg (mirroring Rust's `May.who`,
        // Law 2) — the MustPay punisher is `ifDid: None`; a branchless
        // `May(Pay(cost))` is both `None`, same smart constructor either way.
        // Every OTHER `May` takes `mayWith`, whose (default-`Nothing`)
        // `ifDid`/`ifNot` are positional; `who` has no Idris slot there at
        // all (every other `May` stays implicitly You, Idris's pre-existing
        // simplification), so a non-`You` offeree is a gap rather than
        // silently dropped.
        OneShotEffect::May(m) => match m.effect.as_ref() {
            OneShotEffect::Act(Action::Pay(cost)) => app(
                "mayPayCostBy",
                vec![
                    emit_reference(&m.who)?,
                    emit_cost(cost)?,
                    opt_effect(&m.if_did)?,
                    opt_effect(&m.if_not)?,
                ]
                .into(),
            ),
            _ if !matches!(m.who, Reference::You) => {
                return Err(gap("OneShotEffect::May has no Idris `who` slot"));
            }
            _ => app(
                "mayWith",
                vec![
                    emit_effect(&m.effect)?,
                    opt_effect(&m.if_did)?,
                    opt_effect(&m.if_not)?,
                ]
                .into(),
            ),
        },
        // `ifElse` takes the (default-`Nothing`) `otherwise` positionally.
        OneShotEffect::If(i) => app(
            "ifElse",
            vec![
                emit_condition(&i.condition)?,
                emit_effect(&i.then)?,
                opt_effect(&i.otherwise)?,
            ]
            .into(),
        ),
        OneShotEffect::AdditionalCost(ac) => app(
            "AdditionalCost",
            vec![emit_cost(&ac.pay)?, emit_effect(&ac.body)?].into(),
        ),
        OneShotEffect::Each(e) => app(
            "Each",
            vec![emit_binder(&e.binder)?, emit_effect(&e.effect)?].into(),
        ),
        OneShotEffect::With(w) => app(
            "With",
            vec![emit_binder(&w.binder)?, emit_effect(&w.body)?].into(),
        ),
        OneShotEffect::Distribute(d) => app(
            "Distribute",
            vec![
                emit_count(&d.amount)?,
                emit_binder(&d.binder)?,
                emit_effect(&d.body)?,
            ]
            .into(),
        ),
        OneShotEffect::Noting(_) => return Err(gap("OneShotEffect::Noting not yet mapped")),
        OneShotEffect::Delayed(ta) => {
            let (kinds, facets) = emit_event_filter(&ta.event)?;
            app(
                "Delayed",
                vec![event_query(&kinds, &facets), emit_effect(&ta.effect)?].into(),
            )
        }
        OneShotEffect::Reflexive(ta) => app("Reflexive", vec![emit_effect(&ta.effect)?].into()),
        OneShotEffect::Modal(m) => emit_modal(m)?,
        OneShotEffect::Targeted(t) => emit_targeted(t)?,
        OneShotEffect::Repeat(n, body) => {
            app("Repeat", vec![emit_count(n)?, emit_effect(body)?].into())
        }
        // A batched draw folds into Idris's count-carrying verb
        // (`emit_batched_draw`); every other `Batch` is the SHELL form, emitting
        // exactly like `Repeat` at this stage (see the Idris `Batch` doc
        // comment) — the aggregate-count tier is a later pass's job.
        OneShotEffect::Batch(n, body) => match emit_batched_draw(n, body)? {
            Some(folded) => folded,
            None => app("Batch", vec![emit_count(n)?, emit_effect(body)?].into()),
        },
        OneShotEffect::RevealUntil(r) => app(
            "RevealUntil",
            vec![
                emit_reference(&r.whose)?,
                emit_filter(&r.matches)?,
                emit_effect(&r.body)?,
            ]
            .into(),
        ),
        OneShotEffect::Expanded(_) => {
            return Err(gap(
                "unexpanded OneShotEffect macro invocation remained after expand_all",
            ));
        }
    })
}

/// An optional sub-effect as a plain Idris `Maybe` — `Nothing`, or `(Just
/// <effect>)` — for the positional `mayWith`/`ifElse`/`mayPayFull` helpers.
fn opt_effect(e: &Option<Arc<OneShotEffect>>) -> R {
    match e {
        None => Ok("Nothing".to_string()),
        Some(inner) => Ok(format!("(Just {})", emit_effect(inner)?)),
    }
}

fn emit_duration(d: &deckmaste_semantics::Duration) -> R {
    Ok(match d {
        deckmaste_semantics::Duration::FixedUntil(marker) => match marker {
            deckmaste_semantics::TurnMarker::EndOfTurn => "UntilEndOfTurn".to_string(),
            deckmaste_semantics::TurnMarker::EndOfCombat => "UntilEndOfCombat".to_string(),
            deckmaste_semantics::TurnMarker::YourNextTurn => "UntilYourNextTurn".to_string(),
        },
        deckmaste_semantics::Duration::UntilEvent(ef) => {
            let (kinds, facets) = emit_event_filter(ef)?;
            app("UntilEvent", vec![event_query(&kinds, &facets)].into())
        }
        deckmaste_semantics::Duration::ForAsLongAs(cond) => {
            app("ForAsLongAs", vec![emit_condition(cond)?].into())
        }
        // `ForThisEvent` is an engine-level instruction-scoped rider
        // ([CR#611.2a]), NOT an Idris `Duration`: the Idris model expresses
        // "it can't be regenerated" as a plain `cant (Enact Regenerate …)`,
        // never a durationed continuous effect. Report it as a coverage GAP
        // (not a failure) rather than emit a constructor that does not exist.
        deckmaste_semantics::Duration::ForThisEvent => {
            return Err(gap(
                "Duration::ForThisEvent is an engine rider, not an Idris Duration \
                 (Idris models no-regen via cants)",
            ));
        }
        deckmaste_semantics::Duration::EndOfGame => "Forever".to_string(),
    })
}

fn emit_targeted(t: &deckmaste_semantics::Targeted) -> R {
    let mut specs = Vec::with_capacity(t.targets.len());
    for ts in t.targets.iter() {
        specs.push(emit_target_spec(ts)?);
    }
    Ok(format!(
        "(Targeted {} {})",
        ilist(specs.into()),
        emit_effect(&t.effect)?
    ))
}

fn emit_modal(m: &deckmaste_semantics::Modal) -> R {
    let choose = emit_choose_spec(&m.choose)?;
    let mut modes = Vec::with_capacity(m.modes.len());
    for mode in m.modes.iter() {
        modes.push(emit_mode(mode)?);
    }
    Ok(format!("(Modal {} {})", choose, ilist(modes.into())))
}

fn emit_choose_spec(cs: &deckmaste_semantics::ChooseSpec) -> R {
    let repeats = if cs.repeats { "True" } else { "False" };
    Ok(app(
        "mkChooseSpecRep",
        vec![emit_quantity(&cs.count)?, repeats.to_string()].into(),
    ))
}

fn emit_mode(m: &deckmaste_semantics::Mode) -> R {
    let cost_maybe = match &m.cost {
        None => "Nothing".to_string(),
        Some(cs) => {
            let mut components = Vec::with_capacity(cs.len());
            for c in cs.iter() {
                components.push(emit_cost_component(c)?);
            }
            format!("(Just (Costs {}))", ilist(components.into()))
        }
    };
    Ok(app(
        "mkModeCost",
        vec![emit_effect(&m.effect)?, cost_maybe].into(),
    ))
}

fn emit_separate_piles(_sp: &deckmaste_semantics::SeparatePiles) -> R {
    Err(gap(
        "OneShotEffect::SeparatePiles not yet mapped (Idris's DivideAndChoose has a different two-pile shape)",
    ))
}

fn emit_choose_pile(_cp: &deckmaste_semantics::ChoosePile) -> R {
    Err(gap("OneShotEffect::ChoosePile not yet mapped"))
}

// ===========================================================================
// Ability / KeywordAbility
// ===========================================================================

/// Emit ONE ability (never splicing) — the Idris `Ability b` text.
fn emit_ability(a: &Ability) -> R {
    Ok(match a {
        Ability::Static(se) => app("Static", vec![emit_static_effect(se)?].into()),
        // `activatedFull` exposes window/limits/from/activationGuard
        // positionally (all default in the constructor).
        Ability::Activated(aa) => {
            let window = match aa.window {
                None | Some(deckmaste_semantics::Timing::InstantSpeed) => "AsInstant",
                Some(deckmaste_semantics::Timing::SorcerySpeed) => "AsSorcery",
                Some(_) => {
                    return Err(gap(
                        "ActivatedAbility.window not yet mapped (only Instant/SorcerySpeed)",
                    ));
                }
            };
            let limits = ilist(aa.limits.iter().map(emit_use_limit).collect());
            let from = match aa.from {
                None => "[Battlefield]".to_string(),
                Some(z) => format!("[{}]", emit_zone(z)),
            };
            let guard = match &aa.condition {
                None => "Nothing".to_string(),
                Some(cond) => format!("(Just {})", emit_condition(cond)?),
            };
            app(
                "activatedFull",
                vec![
                    emit_cost(&aa.cost)?,
                    emit_effect(&aa.effect)?,
                    window.to_string(),
                    limits,
                    from,
                    guard,
                ]
                .into(),
            )
        }
        // `triggeredFull` exposes limits/from positionally.
        Ability::Triggered(ta) => {
            let (kinds, facets) = emit_event_filter(&ta.event)?;
            let mut facets = facets.to_vec();
            if let Some(cond) = &ta.condition {
                facets.push(app("Whenever", vec![emit_condition(cond)?].into()));
            }
            if ta.where_x.is_some() {
                return Err(gap("TriggeredAbility.where_x not yet mapped"));
            }
            let limits = ilist(ta.limits.iter().map(emit_use_limit).collect());
            let from = match ta.from {
                None => "[Battlefield]".to_string(),
                Some(z) => format!("[{}]", emit_zone(z)),
            };
            app(
                "triggeredFull",
                vec![
                    event_query(&kinds, &facets),
                    emit_effect(&ta.effect)?,
                    limits,
                    from,
                ]
                .into(),
            )
        }
        Ability::Spell(sa) => app("Spell", vec![emit_effect(&sa.effect)?].into()),
        Ability::Keyword(ka) => app("Keyword", vec![emit_keyword_ability(ka)?].into()),
        Ability::Innate(inner) => emit_ability(inner)?,
        Ability::Expanded(_) => {
            return Err(gap(
                "unexpanded Ability macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_use_limit(l: &deckmaste_semantics::UseLimit) -> String {
    match l {
        deckmaste_semantics::UseLimit::OncePerTurn => "OncePerTurn",
        deckmaste_semantics::UseLimit::OncePerGame => "OncePerGame",
        deckmaste_semantics::UseLimit::LoyaltyOncePerTurn => "LoyaltyOncePerTurn",
    }
    .to_string()
}

fn emit_keyword_ability(ka: &KeywordAbility) -> R {
    Ok(match ka {
        KeywordAbility::FirstStrike => "(Bare FirstStrike)".to_string(),
        KeywordAbility::DoubleStrike => "(Bare DoubleStrike)".to_string(),
        KeywordAbility::Deathtouch => "(Bare Deathtouch)".to_string(),
        KeywordAbility::Trample => "(Bare Trample)".to_string(),
        KeywordAbility::Vigilance => "(Bare Vigilance)".to_string(),
        KeywordAbility::Composite { name, abilities } => {
            let tag = keywordspec_idris(name.as_str())
                .ok_or_else(|| gap(format!("unmapped keyword name: {}", name.as_str())))?;
            app("Composite", vec![tag, emit_ability_list(abilities)?].into())
        }
        KeywordAbility::Expanded(_) => {
            return Err(gap(
                "unexpanded KeywordAbility macro invocation remained after expand_all",
            ));
        }
    })
}

/// Emit one ability, SPLICING an unmappable `Composite` keyword's inner
/// abilities in place of the (untranslatable) tag — e.g. Rust's `Enchant`
/// keyword has no Idris `KeywordSpec` tag at all, but its four desugared
/// abilities (the targeting spell, the two `Cant` rows, the attach
/// replacement) are all faithfully representable on their own. Losing only
/// the "this is named Enchant" bookkeeping, never the mechanics.
fn emit_ability_or_splice(a: &Ability) -> Result<Arc<[String]>, Gap> {
    match a {
        Ability::Innate(inner) => emit_ability_or_splice(inner),
        Ability::Keyword(KeywordAbility::Composite { name, abilities })
            if keywordspec_idris(name.as_str()).is_none() =>
        {
            let mut out = Vec::new();
            for inner in abilities {
                out.extend(emit_ability_or_splice(inner)?.iter().cloned());
            }
            Ok(out.into())
        }
        other => Ok(vec![emit_ability(other)?].into()),
    }
}

fn emit_ability_list(abs: &[Ability]) -> R {
    let mut items = Vec::new();
    for a in abs {
        items.extend(emit_ability_or_splice(a)?.iter().cloned());
    }
    Ok(ilist(items.into()))
}

// ===========================================================================
// Characteristics / Card
// ===========================================================================

fn emit_characteristics_from_face(face: &CardFace) -> R {
    let mana_cost = emit_mana_cost(&face.mana_cost);
    let colors = ilist(
        face.color_indicator
            .iter()
            .map(|c| emit_color(*c))
            .collect(),
    );
    let types = map_list(&face.types, |t| emit_type_name(t.name.as_str()))?;
    let supertypes = ilist(face.supertypes.iter().map(|s| emit_supertype(*s)).collect());
    let subtypes = map_list(&face.subtypes, emit_subtype)?;
    let abilities = emit_ability_list(&face.abilities)?;
    let power = try_maybe(&face.power, emit_stat_value)?;
    let toughness = try_maybe(&face.toughness, emit_stat_value)?;
    let loyalty = try_maybe(&face.loyalty, emit_stat_value)?;
    let defense = try_maybe(&face.defense, emit_stat_value)?;
    Ok(app(
        "MkCharacteristics",
        vec![
            format!("(Just {})", ilit(&face.name)),
            mana_cost,
            colors,
            types,
            supertypes,
            subtypes,
            abilities,
            power,
            toughness,
            loyalty,
            defense,
        ]
        .into(),
    ))
}

/// Emit a `Card`'s equivalent RAW Idris `Card` expression — `Normal
/// (MkCharacteristics …)` for a single-faced card. `Card::TwoFaced` isn't
/// mapped yet (Idris's `TwoFaced` needs two full `Face`s and this emitter has
/// no two-faced canon fixture to validate against).
///
/// `plugin` supplies the loaded subtype/counter/designation registries; they
/// resolve the category of a subtype and the scope of a counter/designation
/// named by REFERENCE (a filter, an `Alter Subtypes` op, a `CountersOn`, …),
/// which carry a name without the rest of the registry row.
///
/// The input is the SEMANTIC term as loaded; this expands it to the kernel's
/// post-expansion normal form itself, so no caller can hand the mirror a
/// remembered macro invocation (spec §10).
///
/// # Errors
/// A [`Gap`] naming the first semantic grammar shape encountered with no (or
/// not-yet-implemented) Idris translation.
pub fn emit_card_expr(card: &Card, plugin: &Plugin) -> R {
    load_subtype_categories(&plugin.subtypes);
    load_counter_scopes(&plugin.counters);
    load_designation_scopes(&plugin.designations);
    match card.clone().expand_all() {
        Card::Normal(face) => Ok(app(
            "Normal",
            vec![emit_characteristics_from_face(&face)?].into(),
        )),
        Card::TwoFaced { .. } => Err(gap("Card::TwoFaced not yet mapped")),
    }
}

/// Every non-todo card in `plugin_dir/cards/**/*.ron`, parsed through
/// `plugin`'s macro scope (its builtin sibling prelude already loaded) —
/// mirrors `validate::validate_plugin`'s card walk, returning the loaded
/// pairs (still macro-`Expanded`-wrapped; callers
/// [`LoadedCard::expanded`](crate::LoadedCard::expanded) them) instead of
/// pass/fail counts.
///
/// # Errors
/// If a directory isn't listable, a file isn't readable, or a non-todo card
/// doesn't parse.
pub fn load_all_cards(
    plugin_dir: &std::path::Path,
    plugin: &crate::plugin::Plugin,
) -> anyhow::Result<Arc<[crate::LoadedCard]>> {
    use anyhow::Context;
    use deckmaste_core::plugin::CARDS_DIR;
    use deckmaste_core::plugin::is_todo_source;

    let mut cards = Vec::new();
    for path in crate::plugin::ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
        let source = crate::plugin::read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        // The pair: the mirror reads `.semantic` (spec §10), and callers keep
        // the engine image for whatever else they do with a loaded card.
        let card = plugin
            .card_from_str(&source)
            .with_context(|| format!("parsing {}", path.display()))?;
        cards.push(card);
    }
    Ok(cards.into())
}

/// The printed name of a card's primary face — what to display/sanitize into
/// a `card_<Name>` Idris identifier.
#[must_use]
pub fn card_display_name(card: &Card) -> &str {
    match card {
        Card::Normal(face) => &face.name,
        Card::TwoFaced { front, .. } => &front.name,
    }
}

/// A valid Idris identifier suffix for `card_<Name>` — strips everything but
/// ASCII alphanumerics, so "Elesh Norn, Grand Cenobite" becomes
/// `"EleshNornGrandCenobite"`. Collisions across the 57-card canon corpus are
/// not expected; a caller wanting collision safety can append an index.
#[must_use]
pub fn sanitize_ident(name: &str) -> String {
    let mut out: String = name.chars().filter(char::is_ascii_alphanumeric).collect();
    if out.is_empty() || out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

#[cfg(test)]
mod tests {
    use deckmaste_semantics::AlternativeCost;
    use deckmaste_semantics::CostTag;

    use super::*;

    /// `Cast(… cost: Components([Tap]), tag: Flashback)` threads the tag
    /// through as `MayCastFor … {tag = Just Flashback}` — the `mayCastForFrom`
    /// shortcut can't carry a tag, so the tagged case falls back to a
    /// directly-named-field `MayCastFor` call ([CR#702.34a]).
    #[test]
    fn cast_tag_emits_maycastfor_with_tag() {
        let action = DeonticAction::Cast {
            what: Predicate::Ref(Reference::This),
            by: Predicate::Any,
            from: Some(deckmaste_semantics::Zone::Graveyard),
            window: None,
            cost: Some(AlternativeCost::Components(vec![CostComponent::Tap].into())),
            tag: Some(CostTag::from("Flashback")),
        };
        let out = emit_can(&action).expect("Cast.tag should emit");
        assert!(
            out.contains("MayCastFor"),
            "expected a MayCastFor call, got: {out}"
        );
        assert!(
            out.contains("tag = Just Flashback"),
            "expected the tag threaded through as `tag = Just Flashback`, got: {out}"
        );
        assert!(
            out.contains("from = [Graveyard]"),
            "expected the `from` zone to still be threaded through, got: {out}"
        );
    }

    /// An untagged `Cast` cost keeps the terse `mayCastForFrom` shape (no
    /// `tag`/`when` field noise) — the tagged path must not regress it.
    #[test]
    fn cast_without_tag_still_uses_maycastforfrom() {
        let action = DeonticAction::Cast {
            what: Predicate::Ref(Reference::This),
            by: Predicate::Any,
            from: None,
            window: None,
            cost: Some(AlternativeCost::Components(vec![CostComponent::Tap].into())),
            tag: None,
        };
        let out = emit_can(&action).expect("Cast without tag should emit");
        assert!(
            out.contains("mayCastForFrom"),
            "expected mayCastForFrom, got: {out}"
        );
        assert!(
            !out.contains("tag"),
            "untagged Cast shouldn't mention `tag` at all, got: {out}"
        );
    }

    /// `Action::Retarget { mode, of, by }` emits as a fully positional
    /// `(Retarget <mode> <of> <by>)` — Law 2 stripped `by`'s old
    /// `{default You}`, so there is no more terse/named-field split; every
    /// call is the same shape regardless of `by` ([CR#115.7d,707.10c]).
    #[test]
    fn retarget_you_emits_fully_positional() {
        let action = Action::Retarget {
            mode: RetargetMode::ChooseNew,
            of: Reference::This,
            by: Reference::You,
        };
        let out = emit_action(&action).expect("Retarget with `by = You` should emit");
        assert_eq!(out, "(Retarget ChooseNew This You)");
    }

    /// A non-default `by` (e.g. an opponent picking new targets, à la Bolt
    /// Bend) emits the exact same positional shape — no named-field fallback
    /// needed any more ([CR#115.7d,707.10c]).
    #[test]
    fn retarget_other_emits_fully_positional() {
        let action = Action::Retarget {
            mode: RetargetMode::ChangeOne,
            of: Reference::This,
            by: Reference::Opponent,
        };
        let out = emit_action(&action).expect("Retarget with a non-default `by` should emit");
        assert_eq!(out, "(Retarget ChangeOne This (Only OpponentOf))");
    }

    /// A `TopOfLibrary`-slice body for a slice/reorder verb
    /// ([CR#121,701.17a,701.22a]) — the `Each` binder `emit_keyword_spec` reads
    /// its coordinates off. The `effect` is inert (only the binder is read).
    fn top_slice_body(count: Count, whose: Reference) -> OneShotEffect {
        OneShotEffect::Each(deckmaste_semantics::Each {
            binder: deckmaste_semantics::Binder::Existing(Selection::TopOfLibrary { count, whose }),
            effect: Arc::new(OneShotEffect::Act(Action::move_to(
                Reference::It,
                deckmaste_semantics::Zone::Graveyard,
            ))),
        })
    }

    /// [CR#701.22a,701.25a]: scry/surveil re-key entity-only — the atom carries
    /// just the COUNT (your library by definition), no performer.
    #[test]
    fn scry_spec_carries_only_the_count() {
        let out = emit_keyword_spec("Scry", &top_slice_body(Count::Literal(2), Reference::You))
            .expect("Scry should emit");
        assert!(out.contains("Scry"), "expected the Scry tag, got: {out}");
        assert!(
            !out.contains("You"),
            "scry carries only the count, no performer: {out}"
        );
    }

    /// [CR#701.17a]: mill re-keys to the performing PLAYER only — the count
    /// rides the enclosing `Batch`, so the per-unit atom has no count arg.
    #[test]
    fn mill_spec_carries_only_the_player() {
        let mill = emit_action(&Action::mill_one(Reference::You)).expect("mill should emit");
        assert!(mill.contains("Mill You"), "mill carries the player: {mill}");
    }

    /// Draw does NOT go through `KeywordActionSpec`: it is [CR#121], not a
    /// keyword action ([CR#701]), so `DrawCard(who)` emits the Idris
    /// `Action.DrawCard` verb directly. A bare per-unit draw reached off the
    /// `Batch` path carries count 1 ([CR#121.2]).
    #[test]
    fn draw_emits_the_player_verb_not_a_keyword_spec() {
        let draw = emit_action(&Action::draw_one(Reference::You)).expect("draw should emit");
        assert!(
            draw.contains("DrawCard") && draw.contains("You"),
            "draw emits the player verb with its actor: {draw}"
        );
        assert!(
            !draw.contains("Composite"),
            "draw is not a keyword action, so it must not ride the Composite lane: {draw}"
        );
    }

    /// [CR#121.2a]: `Batch(n, Act(DrawCard(who)))` FOLDS into Idris's
    /// count-carrying `Action.DrawCard`, because `actionIntro` derives the
    /// card/amount anaphora from that count ("draw three cards, then gain THAT
    /// MUCH life") — an enclosing `Batch` would leave the anaphora at 1.
    #[test]
    fn batched_draw_folds_its_count_onto_the_idris_verb() {
        let out = emit_effect(&deckmaste_semantics::OneShotEffect::draw(
            Reference::You,
            Count::Literal(3),
        ))
        .expect("a batched draw should emit");
        assert!(
            out.contains("DrawCard") && out.contains("Literal 3"),
            "the Batch count folds onto the verb: {out}"
        );
        assert!(
            !out.contains("Batch"),
            "the Batch must be folded away, not emitted alongside: {out}"
        );
    }

    /// [CR#701.20a]: fateseal now emits (previously an Idris gap) — the
    /// fatesealed player plus the count.
    #[test]
    fn fateseal_spec_emits_player_and_count() {
        let out = emit_keyword_spec(
            "Fateseal",
            &top_slice_body(Count::Literal(1), Reference::Opponent),
        )
        .expect("Fateseal should emit (was a gap)");
        assert!(out.contains("Fateseal"), "expected Fateseal, got: {out}");
    }

    /// [CR#701]: an `EventFilter::Act` lowers to its verb `EventKind` plus the
    /// Actor/Agent facets — previously a hard gap; a `cause`-narrowed filter
    /// still gaps (no Idris agency coordinate).
    #[test]
    fn act_filter_lowers_to_the_verb_event_kind() {
        let (kinds, _facets) = emit_event_filter(&EventFilter::Act {
            verb: deckmaste_semantics::VerbName::from("Mill"),
            who: Predicate::Ref(Reference::You),
            on: Predicate::Any,
            cause: None,
        })
        .expect("EventFilter::Act should now lower (was a gap)");
        assert_eq!(kinds, vec!["Mill".to_string()].into());
    }

    /// `StatePredicate::WasCastWith` resolves its tag through the same
    /// `keywordspec_idris` table as `Cast.tag` (a `KeywordSpec`, unlike
    /// `WasPaidWith`'s bare string literal).
    #[test]
    fn was_cast_with_emits_keywordspec() {
        let out = emit_state_filter(&StatePredicate::WasCastWith(CostTag::from("Evoke")))
            .expect("WasCastWith(Evoke) should emit");
        assert_eq!(out, "(WasCastWith Evoke)");
    }

    /// [CR#701.8a]: destroy has no bespoke Idris `Action` — it emits through the
    /// parameterized `KeywordActionSpec` composite whose body IS the
    /// Battlefield → Graveyard `moveAttacking`. Same
    /// `Composite`/`moveAttacking` machinery scry uses (proven to
    /// typecheck), now with the `Destroy` tag.
    #[test]
    fn destroy_emits_composite_over_a_graveyard_move() {
        let out = emit_action(&deckmaste_semantics::Action::destroy(Reference::This))
            .expect("the destroy composite should emit");
        assert!(
            out.contains("Composite"),
            "expected a Composite, got: {out}"
        );
        assert!(
            out.contains("Destroy This"),
            "expected the Destroy tag, got: {out}"
        );
        assert!(
            out.contains("moveAttacking") && out.contains("Graveyard"),
            "expected the body to be the →Graveyard move, got: {out}"
        );
    }

    /// The common, unguarded `Move` (`from: None`) keeps the terse
    /// `moveAttacking` wrapper untouched — must not regress once `Move` grows
    /// the `from` fizzle-guard slot.
    #[test]
    fn move_without_guard_still_uses_moveattacking() {
        let action = deckmaste_semantics::Action::move_to(
            Reference::This,
            deckmaste_semantics::Zone::Graveyard,
        );
        let out = emit_action(&action).expect("an unguarded Move should emit");
        assert_eq!(out, "(moveAttacking This (ToZone Graveyard) Nothing)");
    }

    /// [CR#701.8a,701.9a,701.17a]: a guarded `Move` (`from: Some(_)`) can't
    /// route through `moveAttacking` — that wrapper's own Idris signature has
    /// no `from` slot to name — so it bypasses the wrapper for the raw `Move`
    /// constructor with BOTH named fields spelled explicitly.
    #[test]
    fn move_from_guard_emits_named_field_on_the_raw_constructor() {
        let action = deckmaste_semantics::Action::move_if_in(
            Reference::This,
            deckmaste_semantics::Zone::Hand,
            deckmaste_semantics::Zone::Graveyard,
        );
        let out = emit_action(&action).expect("a guarded Move should emit");
        assert_eq!(
            out,
            "(Move This (ToZone Graveyard) {enteringAttacking = Nothing} {from = (Just Hand)})"
        );
    }

    /// `Condition::CastWith` has no direct Idris `Condition` counterpart, so
    /// it desugars to `Matches This (WasCastWith tag)`.
    #[test]
    fn condition_cast_with_desugars_to_matches() {
        let out = emit_condition(&Condition::CastWith(CostTag::from("Flashback")))
            .expect("CastWith(Flashback) should emit");
        assert_eq!(out, "(Matches This (WasCastWith Flashback))");
    }

    /// The `AsThough` counterfactual overlay re-emits as Idris
    /// `AsThough (Matches This premise) <then>` ([CR#609.4], Glaring
    /// Spotlight): the premise becomes a `Matches This …` condition
    /// wrapping the inner permission emitted as its `Can (Enact …)` clause.
    /// Guards the closed emitter gap without needing the (toolchain-gated)
    /// `idris2 --check`.
    #[test]
    fn as_though_counterfactual_wraps_premise_as_matches_this() {
        let effect = StaticEffect::AsThough(deckmaste_semantics::AsThough::Counterfactual {
            premise: Predicate::Not(Arc::new(Predicate::Characteristic(
                CharacteristicPredicate::Has(deckmaste_semantics::KeywordRef::from("Hexproof")),
            ))),
            then: Arc::new(Deontic::May(DeonticAction::Target {
                by: deckmaste_semantics::DeedAgent::default(),
                on: Predicate::r#type(deckmaste_semantics::Type::Creature),
            })),
        });
        let out = emit_static_effect(&effect).expect("AsThough Counterfactual should emit");
        assert!(
            out.starts_with("(AsThough (Matches This"),
            "premise wraps as `Matches This …`, got: {out}"
        );
        assert!(out.contains("Hexproof"), "keyword survives, got: {out}");
        assert!(
            out.contains("Enact Target"),
            "inner permission is a Target deed, got: {out}"
        );
    }

    /// A novel open type name with no `Type_` counterpart gaps (never
    /// panics), exactly as `Dungeon` does today; a canonical name still
    /// emits.
    #[test]
    fn novel_type_name_gaps_not_panics() {
        let err = emit_type_name("Contraption").unwrap_err();
        let text = err.to_string();
        assert!(text.contains("type `Contraption` has no Idris Type_ counterpart"));
        assert_eq!(emit_type_name("Creature").unwrap(), "Creature");
    }

    // ---- core-copy-grammar Task 8: copy grammar ([CR#707]) → Idris parity ----

    fn copy_spec(source: CopySource, exceptions: Arc<[CopyException]>) -> CopySpec {
        CopySpec {
            source,
            exceptions: exceptions.to_vec(),
        }
    }

    /// The "except it's a 4/4" exception pair ([CR#707.9d]): a P and a T set,
    /// each a `CopyException::Modify` — shared by the token-copy and
    /// becomes-a-copy assertions.
    fn four_four_exceptions() -> Arc<[CopyException]> {
        vec![
            CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(4)))),
            CopyException::Modify(Modification::Toughness(NumericOp::Set(StatValue::Number(
                4,
            )))),
        ]
        .into()
    }

    fn token_copy_effect(count: Count, cs: CopySpec) -> OneShotEffect {
        OneShotEffect::Act(Action::Create {
            agent: Reference::You,
            count,
            token: TokenSpec::Copy(cs.into()),
            riders: [].into(),
        })
    }

    /// [CR#707.2]: a bare single token copy is the `Copy` Action with an empty
    /// modification list — NOT `createTokenAttacking` — and no `Batch` (count
    /// == 1 needs no multiplicity wrapper).
    #[test]
    fn token_copy_bare_emits_copy_action() {
        let out = emit_effect(&token_copy_effect(
            Count::Literal(1),
            copy_spec(CopySource::Object(Reference::Target(0)), [].into()),
        ))
        .expect("a bare token copy should emit");
        assert_eq!(out, "(Act (Copy (Target 0) []))");
    }

    /// [CR#707.1]: a self-card token copy (Embalm/Eternalize copy the exiled
    /// card) reads its source as the Idris self reference `This`.
    #[test]
    fn token_copy_selfcard_emits_this_source() {
        let out = emit_effect(&token_copy_effect(
            Count::Literal(1),
            copy_spec(CopySource::SelfCard, [].into()),
        ))
        .expect("a self-card token copy should emit");
        assert_eq!(out, "(Act (Copy This []))");
    }

    /// [CR#707.2]: "create N copies" has no count slot on `Copy`, so it rides a
    /// `Batch` wrapper — the Mill/Draw per-unit precedent.
    #[test]
    fn token_copy_count_gt_one_wraps_in_batch() {
        let out = emit_effect(&token_copy_effect(
            Count::Literal(2),
            copy_spec(CopySource::Object(Reference::Target(0)), [].into()),
        ))
        .expect("a 2x token copy should emit");
        assert_eq!(out, "(Batch (Literal 2) (Act (Copy (Target 0) [])))");
    }

    /// A count>1 token-copy `Create` reached via `CostComponent::Do` —
    /// OFF the `emit_effect` path that Batch-wraps the multiplicity (there is
    /// no such wrapper here) — must gap rather than silently emit a single
    /// `Copy`, dropping the multiplier.
    #[test]
    fn token_copy_count_gt_one_via_cost_do_gaps() {
        let action = Action::Create {
            agent: Reference::You,
            count: Count::Literal(2),
            token: TokenSpec::Copy(
                copy_spec(CopySource::Object(Reference::Target(0)), [].into()).into(),
            ),
            riders: [].into(),
        };
        let err = emit_cost_component(&CostComponent::Do(Arc::new(action)))
            .expect_err("a count>1 token-copy Do should gap, not silently drop the multiplier");
        assert!(
            err.to_string().contains("multiplicity"),
            "expected the count!=1 off-effect-path gap, got: {err}"
        );
    }

    /// A count==1 token-copy `Create` via `CostComponent::Do` still emits
    /// normally (the gate only fires on count != 1) — the "off effect path"
    /// wrapper must not regress the count==1 case's plain `Copy` emission.
    #[test]
    fn token_copy_count_one_via_cost_do_still_emits() {
        let action = Action::Create {
            agent: Reference::You,
            count: Count::Literal(1),
            token: TokenSpec::Copy(
                copy_spec(CopySource::Object(Reference::Target(0)), [].into()).into(),
            ),
            riders: [].into(),
        };
        let out = emit_cost_component(&CostComponent::Do(Arc::new(action)))
            .expect("a count==1 token-copy Do should still emit");
        assert_eq!(out, "(Do (Copy (Target 0) []))");
    }

    /// [CR#707.9d]: "a copy, except it's a 4/4" — the copiable-value
    /// alterations ride `Copy`'s own modification list as sibling `Alter`s, not
    /// a `BecomeCopyOf` wrapper (the `Copy` verb itself IS the copy).
    #[test]
    fn token_copy_with_exception_carries_alters_in_the_list() {
        let out = emit_effect(&token_copy_effect(
            Count::Literal(1),
            copy_spec(
                CopySource::Object(Reference::Target(0)),
                four_four_exceptions(),
            ),
        ))
        .expect("a 4/4 token copy should emit");
        assert_eq!(
            out,
            "(Act (Copy (Target 0) [(Alter Power (Set (Literal 4))), (Alter Toughness (Set (Literal 4)))]))"
        );
    }

    /// [CR#707.9c..707.9e]: `Retain`/`AdditionalEffect` exceptions have no Idris
    /// analog — a deferred gap, never a guess.
    #[test]
    fn token_copy_with_retain_exception_gaps() {
        let err = emit_effect(&token_copy_effect(
            Count::Literal(1),
            copy_spec(
                CopySource::Object(Reference::Target(0)),
                vec![CopyException::Retain(
                    deckmaste_semantics::Characteristic::Colors,
                )]
                .into(),
            ),
        ))
        .expect_err("a Retain exception should gap");
        assert!(
            err.to_string()
                .contains("idris-copy-retain-additionaleffect"),
            "expected the deferred-follow-up marker, got: {err}"
        );
    }

    /// [CR#707.4]: "X becomes a copy of Y" composes from the existing
    /// `Modify`/`BecomeCopyOf` — no new Idris constructor.
    #[test]
    fn becomes_copy_bare_emits_modify_become_copy_of() {
        let out = emit_static_effect(&StaticEffect::BecomesCopy(
            Reference::This,
            copy_spec(CopySource::Object(Reference::Target(0)), [].into()),
        ))
        .expect("a bare becomes-a-copy should emit");
        assert_eq!(out, "(Modify This (BecomeCopyOf (Target 0)))");
    }

    /// [CR#707.4,707.9]: a becomes-a-copy with "except" mods bundles the
    /// alterations as SIBLING higher-layer mods under `ApplyAll` — the copy
    /// (`BecomeCopyOf`) and each `Alter` are peers, never nested.
    #[test]
    fn becomes_copy_with_exception_wraps_apply_all() {
        let out = emit_static_effect(&StaticEffect::BecomesCopy(
            Reference::This,
            copy_spec(
                CopySource::Object(Reference::Target(0)),
                four_four_exceptions(),
            ),
        ))
        .expect("a 4/4 becomes-a-copy should emit");
        assert_eq!(
            out,
            "(Modify This (ApplyAll [(BecomeCopyOf (Target 0)), (Alter Power (Set (Literal 4))), (Alter Toughness (Set (Literal 4)))]))"
        );
    }

    /// [CR#707.5]: an `EnterRider::AsCopy` payload lowers to `BecomeCopyOf
    /// <src>` (the same self-modification a becomes-a-copy installs); as the
    /// arrival body it is `Modify This (BecomeCopyOf <src>)`. The
    /// ETB-replacement carrier that would host it is not yet emitted, so the
    /// rider-list path gaps pointing at that deferred carrier.
    #[test]
    fn as_copy_lowers_to_become_copy_of() {
        let cs = copy_spec(CopySource::Object(Reference::Target(0)), [].into());
        let rider = EnterRider::AsCopy(cs.clone());

        // The payload mapping (shared with becomes-a-copy).
        let mapping = emit_copy_modification(&cs).expect("AsCopy payload should lower");
        assert_eq!(mapping, "(BecomeCopyOf (Target 0))");
        // ...as an arrival self-modification body.
        assert_eq!(
            format!("(Modify This {mapping})"),
            "(Modify This (BecomeCopyOf (Target 0)))"
        );

        // The rider-list carrier is still a (documented) gap.
        let err = enter_riders_as_attacking(std::slice::from_ref(&rider))
            .expect_err("AsCopy has no attacking-Maybe carrier yet");
        assert!(
            err.to_string().contains("idris-copy-asenters-carrier"),
            "expected the deferred-carrier marker, got: {err}"
        );
    }

    /// [CR#707.12]: casting a copy has no Idris resolution-time cast verb — its
    /// own gap arm (not the generic catch-all), citing the reason.
    #[test]
    fn cast_copy_gaps_with_its_own_reason() {
        let err = emit_action(&Action::CastCopy(
            Reference::You,
            copy_spec(CopySource::Object(Reference::Target(0)), [].into()),
        ))
        .expect_err("CastCopy should gap");
        let msg = err.to_string();
        assert!(
            msg.contains("[CR#707.12]"),
            "expected the [CR#707.12] cite, got: {msg}"
        );
        assert!(
            msg.contains("601 permission pipeline"),
            "expected the 601-pipeline reasoning, got: {msg}"
        );
    }
}
