//! Re-emit an EXPANDED [`Card`] (i.e. after `Expand::expand_all`) as an
//! equivalent RAW `idris/src/Core.idr` `Card` expression — sugar-free, using
//! Core's real constructors (never the `^`/`^:` builder sugar, never
//! `Macros.idr` templates). Typechecking the emitted expression with
//! `idris2 --check` is the anaphora-soundness gate: Idris's dependent
//! `Normal`/`Reference`/`Selection` proofs make an unsound card (a dangling
//! `It`/`That`, an ambiguous antecedent, …) unrepresentable, so a card that
//! typechecks is sound by construction.
//!
//! This module is a plain recursive `Card -> Result<String, Gap>` string
//! emitter — no parsing, no macro layer (the input has already gone through
//! `Plugin`+`Expand::expand_all`). Every function returns either a bare Idris
//! ATOM (no internal spaces, safe to splice unparenthesized) or a FULLY
//! PARENTHESIZED compound expression — so callers can always join emitted
//! pieces with a space and wrap once, with no risk of a stray unparenthesized
//! application.
//!
//! `deckmaste_core`'s grammar has drifted from `idris/src/Core.idr` in real
//! ways since the surface-corrections migration (new `EventFilter` master
//! forms, a `Deontic` family, a `Selection`/`Binder` split, …) — that drift is
//! exactly what this gate is meant to surface. Coverage is intentionally
//! partial: anything not yet mapped returns [`Gap`] rather than guessing, and
//! callers (the `idris-check` xtask command) report gaps as coverage, not
//! failures.
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
    clippy::unnecessary_wraps
)]

use std::cell::RefCell;
use std::collections::HashMap;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Anchor;
use deckmaste_core::Arrangement;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::Cmp;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Condition;
use deckmaste_core::Cost;
use deckmaste_core::CostChange;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::CountBound;
use deckmaste_core::Countable;
use deckmaste_core::Counter;
use deckmaste_core::CounterScope;
use deckmaste_core::CounterSpec;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::DesignationDecl;
use deckmaste_core::DesignationDef;
use deckmaste_core::DesignationScope;
use deckmaste_core::Destination;
use deckmaste_core::EnterRider;
use deckmaste_core::EventFilter;
use deckmaste_core::Ident;
use deckmaste_core::KeywordAbility;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaProduction;
use deckmaste_core::ManaSpec;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::NumericOp;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PhaseStep;
use deckmaste_core::PlayerAction;
use deckmaste_core::PlayerAttr;
use deckmaste_core::PlayerMod;
use deckmaste_core::Predicate;
use deckmaste_core::Property;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::Selection;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Sort;
use deckmaste_core::StatValue;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticEffect;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::SymbolPred;
use deckmaste_core::TargetSpec;
use deckmaste_core::Token;
use deckmaste_core::TokenSpec;
use deckmaste_core::Type;

use crate::plugin::Plugin;

/// A Rust grammar shape this emitter doesn't (yet) translate to Idris —
/// either a genuine expressiveness gap between the two grammars (report
/// honestly), or simply not implemented yet. Carries a short description of
/// what was hit and why.
#[derive(Debug, Clone)]
pub struct Gap(pub String);

impl std::fmt::Display for Gap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Gap {}

type R = Result<String, Gap>;

fn gap(msg: impl Into<String>) -> Gap {
    Gap(msg.into())
}

/// Build `(head a1 a2 …)`, or the bare `head` when `args` is empty (a
/// nullary/unit constructor never needs parens).
fn app(head: &str, args: Vec<String>) -> String {
    if args.is_empty() {
        head.to_string()
    } else {
        format!("({} {})", head, args.join(" "))
    }
}

/// An Idris list literal `[a, b, c]` — self-delimited, safe to splice as an
/// argument with no extra parens.
fn ilist(items: Vec<String>) -> String {
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
    Ok(ilist(out))
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
        SymbolPred::CountsAs(c) => app("CountsAs", vec![emit_color(*c)]),
        SymbolPred::IsGeneric => "IsGeneric".to_string(),
        SymbolPred::And(ps) => app("And", vec![map_list(ps, emit_symbol_pred)?]),
        SymbolPred::Or(ps) => app("Or", vec![map_list(ps, emit_symbol_pred)?]),
        SymbolPred::Not(inner) => app("Not", vec![emit_symbol_pred(inner)?]),
        SymbolPred::AnyColor => app(
            "Or",
            vec![ilist(vec![
                app("CountsAs", vec!["White".to_string()]),
                app("CountsAs", vec!["Blue".to_string()]),
                app("CountsAs", vec!["Black".to_string()]),
                app("CountsAs", vec!["Red".to_string()]),
                app("CountsAs", vec!["Green".to_string()]),
            ])],
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
        SimpleManaSymbol::Generic(n) => app("Generic", vec![n.to_string()]),
        SimpleManaSymbol::Specific(c) => app("Specific", vec![emit_color_or_colorless(*c)]),
    }
}

fn emit_mana_symbol(m: &ManaSymbol) -> String {
    match m {
        ManaSymbol::Variable => "Variable".to_string(),
        ManaSymbol::Snow => "SnowMana".to_string(),
        ManaSymbol::Hybrid(s, c) => app("Hybrid", vec![emit_simple_mana_symbol(s), emit_color(*c)]),
        ManaSymbol::Phyrexian(c, mc) => app(
            "Phyrexian",
            vec![
                emit_color(*c),
                match mc {
                    None => "Nothing".to_string(),
                    Some(c2) => format!("(Just {})", emit_color(*c2)),
                },
            ],
        ),
        ManaSymbol::Simple(s) => app("Simple", vec![emit_simple_mana_symbol(s)]),
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

/// Populate [`SUBTYPE_CATEGORY`] from a plugin's subtype registry. Called once
/// per card at emit entry; subtypes whose `types` map to no category are simply
/// absent (a reference to one then gaps, like any other coverage gap).
fn load_subtype_categories<S>(subtypes: &HashMap<Ident, Subtype, S>) {
    SUBTYPE_CATEGORY.with(|m| {
        let mut m = m.borrow_mut();
        m.clear();
        for (name, sub) in subtypes {
            if let Some(cat) = category_idris(&sub.types) {
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
fn scope_idris(scope: &CounterScope) -> &'static str {
    match scope {
        CounterScope::Object => "Object",
        CounterScope::Player => "Player",
    }
}

/// The Idris `Scope` token for a designation's scope. `DesignationScope::Game`
/// has no `RefKind`/`Scope` analogue (nothing in Idris is game-scoped, and no
/// macro registry row uses it), so it gaps (`None`).
fn designation_scope_idris(scope: &DesignationScope) -> Option<&'static str> {
    match scope {
        DesignationScope::Object => Some("Object"),
        DesignationScope::Player => Some("Player"),
        DesignationScope::Game => None,
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
fn load_counter_scopes<S>(counters: &HashMap<Ident, Counter, S>) {
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
fn load_designation_scopes<S>(designations: &HashMap<Ident, DesignationDecl, S>) {
    DESIGNATION_SCOPE.with(|m| {
        let mut m = m.borrow_mut();
        m.clear();
        for decl in designations.values() {
            let scope = match &decl.definition {
                DesignationDef::Stored { scope, .. } => {
                    let Some(scope) = designation_scope_idris(scope) else {
                        continue;
                    };
                    scope
                }
                DesignationDef::Derived(_) | DesignationDef::DerivedIf(_) => "Object",
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
        StatValue::Number(n) => Ok(app("Literal", vec![n.to_string()])),
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
        Sort::OfType(t) => app("OfType", vec![emit_type_name(t.name().as_str())?]),
        Sort::Amount => "Amount".to_string(),
        Sort::Pile => "Pile".to_string(),
    })
}

fn emit_reference(r: &Reference) -> R {
    Ok(match r {
        Reference::This => "This".to_string(),
        Reference::You => "You".to_string(),
        // No definite "the opponent" Reference in Idris; the closest sound
        // reading is the unique object matching the opponent predicate.
        Reference::Opponent => "(Only OpponentOf)".to_string(),
        Reference::It => "It".to_string(),
        Reference::EventObject => "EventObject".to_string(),
        Reference::EventPatient => "EventPatient".to_string(),
        Reference::EventActor => "EventActor".to_string(),
        Reference::DefendingPlayer => "DefendingPlayer".to_string(),
        Reference::That(sort) => app("That", vec![emit_sort(sort)?]),
        // The nth announced target ([CR#115.3,601.2c]).
        Reference::Target(n) => app("Target", vec![n.to_string()]),
        Reference::ControllerOf(r) => app("ControllerOf", vec![emit_reference(r)?]),
        Reference::OwnerOf(r) => app("OwnerOf", vec![emit_reference(r)?]),
        Reference::AttachHostOf(r) => app("AttachHostOf", vec![emit_reference(r)?]),
        Reference::Bound(_) => {
            return Err(gap(
                "Reference::Bound (legacy role binding) has no Idris counterpart",
            ));
        }
        Reference::Linked(_) => return Err(gap("Reference::Linked has no Idris counterpart")),
        // A set-valued deal-time damage-source binding read only inside
        // `Is(Source, …)` on an engine SBA rule — never authored on a card, so
        // no Idris counterpart (the builtin SBA rules are engine data, not
        // emitted card definitions).
        Reference::Source => {
            return Err(gap(
                "Reference::Source (deal-time damage-source binding) has no Idris counterpart",
            ));
        }
        Reference::Expanded(_) => {
            return Err(gap(
                "unexpanded Reference macro invocation remained after expand_all",
            ));
        }
    })
}

/// `emit_reference`, specialized for a position where Idris's `Reference b k`
/// is genuinely kind-poly with nothing else to pin `k` (`DealDamage`'s
/// recipient — "any target"). Every OTHER `Reference` constructor already
/// carries its own kind-fixing proof (`It`'s antecedent, `EventPatient`'s
/// cap, `That`'s sort, …) or gets unified from a concretely-kinded consumer
/// elsewhere, so only the context-free `Target n` needs its kind pinned to
/// `Anything` — a bare `Target n` there would leave `k` an unsolved hole, and
/// forcing `Anything` on every `Reference` would wrongly override a caller
/// (`Attach`, `Move`, both `Reference b AnObject`) that needs a concrete kind.
/// `damageTarget` is the curly-free Core helper that bakes `{k = Anything}` at
/// the type-constructor level (the emitter must never author a brace).
fn emit_reference_anykind(r: &Reference) -> R {
    Ok(match r {
        Reference::Target(n) => format!("(damageTarget {n})"),
        _ => emit_reference(r)?,
    })
}

/// Convert a `Reference` used where Idris wants a `Predicate` (Idris's
/// `Sacrifice`/`ChooseOne`/… bake the choice INTO the predicate rather than
/// pre-resolving it via a binder, unlike Rust's newer split). Every reference
/// becomes `SameAs <ref>` (an already-resolved reference IS a predicate:
/// "equal to r").
fn reference_as_predicate(r: &Reference) -> R {
    Ok(app("SameAs", vec![emit_reference(r)?]))
}

// ===========================================================================
// Rust Predicate -> Idris Predicate
// ===========================================================================

fn emit_filter(f: &Predicate) -> R {
    Ok(match f {
        Predicate::Kind(k) => {
            use deckmaste_core::ObjectKind as K;
            match k {
                K::Ability => "(IsKind Ability)".to_string(),
                K::Card => "(IsKind Card)".to_string(),
                K::Emblem => "(IsKind Emblem)".to_string(),
                K::Spell => "(IsKind Spell)".to_string(),
                K::Token => "(IsKind Token)".to_string(),
                // Idris `ObjectKind` has no `Player` member (a player test is
                // the top player-predicate, not an object kind).
                K::Player => "Anyone".to_string(),
                K::CardCopy => {
                    return Err(gap(
                        "ObjectKind::CardCopy has no Idris ObjectKind counterpart",
                    ));
                }
            }
        }
        Predicate::Characteristic(cf) => emit_characteristic_filter(cf)?,
        Predicate::State(sf) => emit_state_filter(sf)?,
        Predicate::Relation(rf) => emit_relation_filter(rf)?,
        Predicate::Ref(r) => app("SameAs", vec![emit_reference(r)?]),
        Predicate::Adjacent(a, r) => app("Adjacent", vec![emit_adjacency(*a), emit_reference(r)?]),
        Predicate::FromSource(_) => {
            return Err(gap(
                "Predicate::FromSource has no Idris Predicate counterpart",
            ));
        }
        Predicate::And(fs) => app("And", vec![map_list(fs, emit_filter)?]),
        Predicate::Or(fs) => app("Or", vec![map_list(fs, emit_filter)?]),
        Predicate::Not(inner) => app("Not", vec![emit_filter(inner)?]),
        Predicate::Where(cond) => app("Where", vec![emit_condition(cond)?]),
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
            vec!["Types".to_string(), emit_type_name(name.as_str())?],
        ),
        CharacteristicPredicate::Subtype(name) => app(
            "HasChar",
            vec!["Subtypes".to_string(), emit_subtype_ref(name.as_str())?],
        ),
        CharacteristicPredicate::Supertype(s) => app(
            "HasChar",
            vec!["Supertypes".to_string(), emit_supertype(*s)],
        ),
        CharacteristicPredicate::ColorIs(c) => {
            app("HasChar", vec!["Colors".to_string(), emit_color(*c)])
        }
        CharacteristicPredicate::Named(name) => app("HasName", vec![ilit(name.as_str())]),
        CharacteristicPredicate::Stat(stat, cmp, count) => {
            let characteristic = numeric_characteristic(*stat)?;
            app(
                "StatCmp",
                vec![characteristic, emit_cmp(*cmp), emit_count(count)?],
            )
        }
        CharacteristicPredicate::Multicolored => "Multicolored".to_string(),
        CharacteristicPredicate::Colorless => "IsColorless".to_string(),
        CharacteristicPredicate::Has(kw) => {
            let spec = keywordspec_idris(kw.as_str())
                .ok_or_else(|| gap(format!("unmapped keyword in Has(): {}", kw.as_str())))?;
            app("HasKeyword", vec![spec])
        }
    })
}

/// `Stat` restricted to Idris's `Numeric`-gated axes (Power/Toughness/Defense)
/// — `StatCmp`/`TapTotal` demand one of these three.
fn numeric_characteristic(stat: deckmaste_core::Stat) -> R {
    use deckmaste_core::Stat as S;
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
        StatePredicate::InZone(z) => app("InZone", vec![emit_zone(*z)]),
        StatePredicate::Status(status) => {
            use deckmaste_core::Status as S;
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
            app("HasCounter", vec![k])
        }
        StatePredicate::Designated(name) => {
            let d = designation_ref_idris(name.as_str())?;
            app("HasDesignation", vec![d])
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
        StatePredicate::Targets(inner) => app("Targets", vec![emit_filter(inner)?]),
        StatePredicate::TargetCount(bound) => {
            let (cmp, count) = bound.split();
            app("TargetCount", vec![emit_cmp(cmp), emit_count(count)?])
        }
        StatePredicate::WasPaidWith(tag) => app("WasPaidWith", vec![ilit(tag.as_str())]),
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
            app("WasCastWith", vec![spec])
        }
        // The move-provenance twin of `WasCastFrom` ([CR#701.17a,701.9a] —
        // "milled"/"discarded" decompose over this).
        StatePredicate::WasPutFrom(z) => app("WasPutFrom", vec![emit_zone(*z)]),
    })
}

/// `Above`/`Below` — [`Predicate::Adjacent`]'s direction, identity on the
/// Idris `Adjacency` constructor name.
fn emit_adjacency(a: deckmaste_core::Adjacency) -> String {
    use deckmaste_core::Adjacency as A;
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
        RelationPredicate::ControlledBy(f) => app("ControlledBy", vec![emit_filter(f)?]),
        RelationPredicate::Controls(f) => app("Controls", vec![emit_filter(f)?]),
        RelationPredicate::Owner(f) => app("OwnedBy", vec![emit_filter(f)?]),
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

fn emit_zone(z: deckmaste_core::Zone) -> String {
    use deckmaste_core::Zone as Z;
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
            vec![emit_count(a)?, emit_cmp(*cmp), emit_count(b)?],
        ),
        Condition::Exists(f) => format!("(exists {})", emit_filter(f)?),
        Condition::Matches(r, f) => app("Matches", vec![emit_reference(r)?, emit_filter(f)?]),
        Condition::LegallyAttached(r) => app("LegallyAttached", vec![emit_reference(r)?]),
        Condition::Happened { .. } => {
            return Err(gap("Condition::Happened (history lookback) not yet mapped"));
        }
        Condition::Crossed { .. } => return Err(gap("Condition::Crossed not yet mapped")),
        Condition::PaidCost(tag) => app("PaidCost", vec![ilit(tag.as_str())]),
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
                vec!["This".to_string(), app("WasCastWith", vec![spec])],
            )
        }
        Condition::YourTurn => "yourTurn".to_string(),
        Condition::TurnOf(f) => app("TurnOf", vec![emit_filter(f)?]),
        Condition::DuringPhase(p) => app("During", vec![emit_phase_step(*p)?]),
        Condition::And(cs) => app("And", vec![map_list(cs, emit_condition)?]),
        Condition::Or(cs) => app("Or", vec![map_list(cs, emit_condition)?]),
        Condition::Not(inner) => app("Not", vec![emit_condition(inner)?]),
        Condition::Expanded(_) => {
            return Err(gap(
                "unexpanded Condition macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_phase_step(p: PhaseStep) -> R {
    use deckmaste_core::BeginningStep as B;
    use deckmaste_core::CombatStep as C;
    use deckmaste_core::EndingStep as E;
    Ok(match p {
        PhaseStep::Beginning(b) => app(
            "BeginningPhase",
            vec![
                match b {
                    B::Untap => "UntapStep",
                    B::Upkeep => "UpkeepStep",
                    B::Draw => "DrawStep",
                }
                .to_string(),
            ],
        ),
        PhaseStep::PrecombatMain => "(MainPhase PreCombat)".to_string(),
        PhaseStep::PostcombatMain => "(MainPhase PostCombat)".to_string(),
        PhaseStep::Combat(c) => app(
            "CombatPhase",
            vec![
                match c {
                    C::BeginningOfCombat => "BeginningOfCombatStep",
                    C::DeclareAttackers => "DeclareAttackersStep",
                    C::DeclareBlockers => "DeclareBlockersStep",
                    C::FirstCombatDamage => "FirstCombatDamageStep",
                    C::CombatDamage => "CombatDamageStep",
                    C::EndOfCombat => "EndOfCombatStep",
                }
                .to_string(),
            ],
        ),
        PhaseStep::Ending(e) => app(
            "EndingPhase",
            vec![
                match e {
                    E::End => "EndStep",
                    E::Cleanup => "CleanupStep",
                }
                .to_string(),
            ],
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
            vec![emit_reference(r)?, emit_symbol_pred(pred)?],
        ),
        // [CR#105.2]: the per-object twin of `Objects` — Embiggen's "number
        // of card types it has" = `CountDistinct Types (Singleton This)`.
        Countable::Singleton(r) => app("Singleton", vec![emit_reference(r)?]),
        // [CR#107.4]: mana spent to cast/activate `r`, filtered by `pred` —
        // Adamant's "if at least three white mana symbols were spent".
        Countable::ManaSpentMatching(r, pred) => app(
            "ManaSpentMatching",
            vec![emit_reference(r)?, emit_symbol_pred(pred)?],
        ),
    })
}

/// An [`AggregateOp`] as its Idris twin — identity on the fold operator name,
/// `AverageOf` carrying its [`RoundMode`](deckmaste_core::RoundMode).
fn emit_aggregate_op(op: &deckmaste_core::AggregateOp) -> String {
    use deckmaste_core::AggregateOp;
    match op {
        AggregateOp::SumOf => "SumOf".to_string(),
        AggregateOp::MinOf => "MinOf".to_string(),
        AggregateOp::MaxOf => "MaxOf".to_string(),
        AggregateOp::AverageOf(mode) => app(
            "AverageOf",
            vec![
                match mode {
                    deckmaste_core::RoundMode::RoundUp => "RoundUp",
                    deckmaste_core::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
            ],
        ),
    }
}

fn emit_count(c: &Count) -> R {
    Ok(match c {
        Count::X => "X".to_string(),
        Count::Literal(n) => app("Literal", vec![n.to_string()]),
        Count::CountOf(source) => match source {
            Countable::Objects(f) => format!("(CountMatching {})", emit_filter(f)?),
            // Every other `Countable` source (devotion's `ManaSymbols`,
            // `Singleton`, `ManaSpentMatching`) has no `CountMatching`-style
            // sugar — spell it out as the explicit `CountOf` application.
            other => app("CountOf", vec![emit_countable(other)?]),
        },
        Count::CountDistinct(characteristic, source) => {
            let c = collection_characteristic(*characteristic)?;
            let src = emit_countable(source)?;
            app("CountDistinct", vec![c, src])
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
                    vec![emit_filter(filter)?, emit_count(&proj.by)?],
                ),
                other => app(
                    "Project",
                    vec![emit_countable(other)?, emit_count(&proj.by)?],
                ),
            };
            app("Aggregate", vec![emit_aggregate_op(op), projected])
        }
        Count::StatOf(r, stat) => {
            use deckmaste_core::Stat as S;
            match stat {
                S::ManaValue => app("ManaValueOf", vec![emit_reference(r)?]),
                S::Loyalty => app(
                    "CountersOn",
                    vec!["Loyalty".to_string(), emit_reference(r)?],
                ),
                S::Power | S::Toughness | S::Defense => app(
                    "StatOf",
                    vec![emit_reference(r)?, numeric_characteristic(*stat)?],
                ),
            }
        }
        Count::CounterCount(r, kind) => {
            let k = counter_ref_idris(kind.as_str())?;
            app("CountersOn", vec![k, emit_reference(r)?])
        }
        // [CR#119.1,402.2]: a player's numeric attribute — the Idris
        // `PlayerStatOf` (the player-side twin of `StatOf`).
        Count::PlayerStatOf(r, attr) => app(
            "PlayerStatOf",
            vec![emit_reference(r)?, emit_player_attr(*attr)],
        ),
        // [CR#102.1]: opponent count. Idris has no dedicated constructor — it
        // is the cardinality of the opponent PLAYERS (`CountOf (Players
        // OpponentOf)`). Idris's `OpponentOf` is relative to `You`, so an
        // opponent count of any other player has no counterpart.
        Count::Opponents(r) => match r {
            deckmaste_core::Reference::You => "(CountOf (Players OpponentOf))".to_string(),
            other => {
                return Err(gap(format!(
                    "Count::Opponents({other:?}) has no Idris counterpart \
                     (Idris OpponentOf is relative to You)"
                )));
            }
        },
        Count::Min(a, b) => app("Min", vec![emit_count(a)?, emit_count(b)?]),
        Count::Max(a, b) => app("Max", vec![emit_count(a)?, emit_count(b)?]),
        Count::Plus(a, b) => app("Plus", vec![emit_count(a)?, emit_count(b)?]),
        Count::Minus(a, b) => app("Minus", vec![emit_count(a)?, emit_count(b)?]),
        Count::Times(a, b) => app("Times", vec![emit_count(a)?, emit_count(b)?]),
        Count::Half(mode, inner) => app(
            "Half",
            vec![
                match mode {
                    deckmaste_core::RoundMode::RoundUp => "RoundUp",
                    deckmaste_core::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
                emit_count(inner)?,
            ],
        ),
        // [CR#107.1a]: general integer division, `Half`'s dedicated /2 twin.
        Count::Divide(mode, a, b) => app(
            "Divide",
            vec![
                match mode {
                    deckmaste_core::RoundMode::RoundUp => "RoundUp",
                    deckmaste_core::RoundMode::RoundDown => "RoundDown",
                }
                .to_string(),
                emit_count(a)?,
                emit_count(b)?,
            ],
        ),
        // [CR#107.1]: remainder — parity checks (`Compare(Mod(x, 2), Eq, 0)`).
        Count::Mod(a, b) => app("Mod", vec![emit_count(a)?, emit_count(b)?]),
        // [CR#107.1]: exponentiation — doubling effects (`Pow(2, X)`).
        Count::Pow(a, b) => app("Pow", vec![emit_count(a)?, emit_count(b)?]),
        // [CR#115.9a]: how many times `r` was chosen as a target on
        // announcement — Strive's `Minus(TargetsOf(This), 1)`.
        Count::TargetsOf(r) => app("TargetsOf", vec![emit_reference(r)?]),
        Count::ThatMany | Count::ThatMuch => "ThatMany".to_string(),
        Count::Allotment => "Allotment".to_string(),
        Count::EventCount(..) => {
            return Err(gap("Count::EventCount (history lookback) not yet mapped"));
        }
        Count::EventSum(..) => {
            return Err(gap("Count::EventSum (history lookback) not yet mapped"));
        }
        Count::Noted(_) => return Err(gap("Count::Noted has no Idris counterpart")),
        Count::TimesPaid(tag) => app("TimesPaid", vec![ilit(tag.as_str())]),
        Count::Damage(r) => app("Damage", vec![emit_reference(r)?]),
        // The floated-mana-pool reader is a data-driven-strategy sensing
        // source ([CR#106.4]); the Idris grammar models card text, not play
        // policy, so it has no counterpart.
        Count::ManaAvailable(_) => {
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

fn collection_characteristic(c: deckmaste_core::Characteristic) -> R {
    use deckmaste_core::Characteristic as C;
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
        ],
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
            app("Range", vec![format!("(Just {c})"), format!("(Just {c})")])
        }
        CountBound::AtLeast(c) => app(
            "Range",
            vec![format!("(Just {})", emit_count(c)?), "Nothing".to_string()],
        ),
        CountBound::AtMost(c) => app(
            "Range",
            vec!["Nothing".to_string(), format!("(Just {})", emit_count(c)?)],
        ),
        CountBound::Greater(c) => {
            let n = lit(c).ok_or_else(|| {
                gap("CountBound::Greater on a dynamic Count can't be widened to a Quantity")
            })?;
            app(
                "Range",
                vec![format!("(Just (Literal {}))", n + 1), "Nothing".to_string()],
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
                vec!["Nothing".to_string(), format!("(Just (Literal {}))", n - 1)],
            )
        }
    })
}

// ===========================================================================
// Selection / Binder / TargetSpec
// ===========================================================================

fn emit_selection(s: &Selection) -> R {
    Ok(match s {
        Selection::SelectAll(f) => app("SelectAll", vec![emit_filter(f)?]),
        Selection::Union(gs) => app("Union", vec![map_list(gs, emit_selection)?]),
        Selection::Random(q, f) => app("Random", vec![emit_quantity(q)?, emit_filter(f)?]),
        Selection::AmongNoted(..) => {
            return Err(gap("Selection::AmongNoted has no Idris counterpart"));
        }
        Selection::TopOfLibrary { count, whose } => {
            app("topFrom", vec![emit_count(count)?, emit_reference(whose)?])
        }
        Selection::BottomOfLibrary { count, whose } => app(
            "bottomFrom",
            vec![emit_count(count)?, emit_reference(whose)?],
        ),
        // Unlike `TopOfLibrary`/`BottomOfLibrary`'s `topFrom`/`bottomFrom`
        // helpers (baking a `{default You}` implicit arg), Idris
        // `TopOfGraveyard`'s `whose` is EXPLICIT — emits as a direct `app`,
        // no curried helper needed.
        Selection::TopOfGraveyard { count, of } => app(
            "TopOfGraveyard",
            vec![emit_count(count)?, emit_reference(of)?],
        ),
        Selection::They => "They".to_string(),
        Selection::Them(sort) => app("Them", vec![emit_sort(sort)?]),
        Selection::PilesOf { .. } => return Err(gap("Selection::PilesOf not yet mapped")),
        // [CR#107.1]: the extremal element(s) of `proj` — shares `Project`
        // emission with `Count::Aggregate`; `op` is identity on the Idris
        // `AggregateOp` name (gated to `MinOf`/`MaxOf` at the Rust type by
        // the eval fizzle, not here). Idris's `Pick` is pinned to `Projection
        // b AnObject` ([CR#107.1] — a player-`Pick` has no consumer and no
        // Idris counterpart), so a `Players`-sourced `proj` here is an
        // authoring mistake, not a representable card — reported as a gap
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
                        vec![emit_countable(&proj.of)?, emit_count(&proj.by)?],
                    ),
                ],
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
            app("Target", vec![emit_quantity(q)?, pred])
        }
        TargetSpec::Distinct(idxs, inner) => app(
            "Distinct",
            vec![
                ilist(idxs.iter().map(usize::to_string).collect()),
                emit_target_spec(inner)?,
            ],
        ),
        TargetSpec::Expanded(_) => {
            return Err(gap(
                "unexpanded TargetSpec macro invocation remained after expand_all",
            ));
        }
    })
}

/// `deckmaste_core::Binder` -> Idris `Bindable`. Only the shapes actually
/// wired for resolution are mapped (`binder.rs` notes `Produce`/`Search*`
/// aren't yet engine-resolved either, so gapping them costs nothing today).
fn emit_binder(b: &deckmaste_core::Binder) -> R {
    use deckmaste_core::Binder as B;
    Ok(match b {
        B::TheRef(r) => app("TheRef", vec![emit_reference(r)?]),
        B::ChooseOne { filter, by } => app(
            "chooseOneBy",
            vec![emit_reference(by)?, emit_filter(filter)?],
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
            ],
        ),
        B::Existing(sel) => app("Existing", vec![emit_selection(sel)?]),
        B::Produce(action) => app("Produce", vec![emit_action(action)?]),
        B::SearchOne { filter, .. } => app("SearchOne", vec![emit_filter(filter)?]),
        B::Search {
            quantity, filter, ..
        } => app(
            "Search",
            vec![emit_quantity(quantity)?, emit_filter(filter)?],
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
    Ok(app("Costs", vec![ilist(components)]))
}

fn emit_cost_component(c: &CostComponent) -> R {
    Ok(match c {
        CostComponent::Mana(cost) => app("Mana", vec![emit_mana_cost(cost)]),
        CostComponent::ManaCostOf(r) => app("ManaCostOf", vec![emit_reference(r)?]),
        CostComponent::Tap => "(Do (Tap This))".to_string(),
        CostComponent::Untap => "(Do (Untap This))".to_string(),
        CostComponent::Do(action) => app("Do", vec![emit_player_action(action, &Reference::You)?]),
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
            ],
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

fn emit_with_cost_as_predicate_verb(binder: &deckmaste_core::Binder, body: &Cost) -> R {
    use deckmaste_core::Binder as B;
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
            PlayerAction::Sacrifice(Reference::That(_)) => {
                let sac = app(
                    "sacrificeBy",
                    vec![emit_reference(by)?, emit_filter(filter)?],
                );
                Ok(app("Do", vec![sac]))
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
        Destination::Zone(z) => app("ToZone", vec![emit_zone(*z)]),
        Destination::Library(anchor) => app("ToLibrary", vec![emit_anchor(anchor)?]),
    })
}

fn emit_anchor(a: &Anchor) -> R {
    Ok(match a {
        Anchor::FromTop(c) => app("FromTop", vec![emit_count(c)?]),
        Anchor::FromBottom(c) => app("FromBottom", vec![emit_count(c)?]),
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

/// Only an empty rider list, or a single `Attacking(Some(_))` rider, has an
/// Idris counterpart (`enteringAttacking`); anything else is a gap.
fn enter_riders_as_attacking(riders: &[EnterRider]) -> Result<Option<String>, Gap> {
    match riders {
        [] => Ok(None),
        [EnterRider::Attacking(Some(who))] => Ok(Some(format!("(Just {})", emit_reference(who)?))),
        _ => Err(gap(
            "EnterRider list has no Idris Move/MoveArranged counterpart beyond a lone Attacking(Some(_))",
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
            app("Some", vec![k, emit_count(count)?])
        }
        CounterSpec::AllKinds => "AllKinds".to_string(),
    })
}

// ===========================================================================
// Action / PlayerAction
// ===========================================================================

fn emit_action(a: &Action) -> R {
    Ok(match a {
        // `dealDamageFrom` exposes the required `source` positionally.
        Action::DealDamage(source, count, patient) => app(
            "dealDamageFrom",
            vec![
                emit_reference(source)?,
                emit_reference_anykind(patient)?,
                emit_count(count)?,
            ],
        ),
        Action::Destroy(r) => app("Destroy", vec![emit_reference(r)?]),
        Action::Counter(r) => app("Counter", vec![emit_reference(r)?]),
        Action::Attach { what, to } => {
            app("Attach", vec![emit_reference(what)?, emit_reference(to)?])
        }
        Action::Unattach(r) => app("Unattach", vec![emit_reference(r)?]),
        // `moveAttacking` exposes the (default-`Nothing`) `enteringAttacking`
        // positionally as a plain `Maybe`.
        Action::Move(r, dest, riders) => app(
            "moveAttacking",
            vec![
                emit_reference(r)?,
                emit_destination(dest)?,
                attacking_maybe(riders)?,
            ],
        ),
        Action::MoveGroup {
            group,
            arrangement,
            to,
            riders,
        } => {
            if !riders.is_empty() {
                return Err(gap("MoveGroup riders not yet mapped"));
            }
            app(
                "MoveArranged",
                vec![
                    emit_selection(group)?,
                    emit_arrangement(arrangement),
                    emit_destination(to)?,
                ],
            )
        }
        Action::GainControl(..) => {
            return Err(gap(
                "Action::GainControl has no Idris one-shot Action counterpart (only the continuous Modification)",
            ));
        }
        Action::ExtraPhase(..) => return Err(gap("Action::ExtraPhase has no Idris counterpart")),
        Action::BecomeDay | Action::BecomeNight => {
            return Err(gap("day/night has no Idris counterpart"));
        }
        Action::TheRingTempts(_) => {
            return Err(gap("Action::TheRingTempts has no Idris counterpart"));
        }
        Action::MoveCounters(spec, from, to) => app(
            "MoveCounters",
            vec![
                emit_counter_spec(spec)?,
                emit_reference(from)?,
                emit_reference(to)?,
            ],
        ),
        Action::CreateReplacement { .. } => {
            return Err(gap("Action::CreateReplacement has no Idris counterpart"));
        }
        // `Composite name body` ([CR#701]): the named keyword action —
        // `Composite Scry (Each …)` etc. The `KeywordActionSpec` is one of
        // the committed Idris constructors (`Scry | Surveil | Mill | Fight`);
        // any other keyword (e.g. Fateseal — no Idris spec yet) is a gap.
        Action::Composite { name, body } => {
            let spec = match name.as_str() {
                "Scry" | "Surveil" | "Mill" | "Fight" => name.as_str().to_string(),
                other => {
                    return Err(gap(format!(
                        "Composite keyword action {other:?} has no Idris KeywordActionSpec"
                    )));
                }
            };
            app("Composite", vec![spec, emit_effect(body)?])
        }
        Action::By(actor, pa) => emit_player_action(pa, actor)?,
    })
}

fn emit_player_action(pa: &PlayerAction, actor: &Reference) -> R {
    // The player verbs whose Idris constructor carries a `{default You actor}`
    // are emitted through the positional `<verb>By` helper, always passing the
    // actor. The verbs whose Idris constructor has NO actor slot
    // (`Reveal`/`PutCounters`/`Tap`/`Untap`/…) can only be the default `You`,
    // so a non-`You` actor there is a gap.
    match pa {
        PlayerAction::Draw(c) => Ok(app("drawBy", vec![emit_reference(actor)?, emit_count(c)?])),
        PlayerAction::Discard {
            count,
            what,
            random,
        } => {
            if what.is_some() || *random {
                return Err(gap(
                    "Discard{what|random} has no Idris Discard counterpart (Idris Discard is count-only)",
                ));
            }
            Ok(app(
                "discardBy",
                vec![emit_reference(actor)?, emit_count(count)?],
            ))
        }
        PlayerAction::GainLife(c) => Ok(app(
            "gainLifeBy",
            vec![emit_reference(actor)?, emit_count(c)?],
        )),
        PlayerAction::LoseLife(c) => Ok(app(
            "loseLifeBy",
            vec![emit_reference(actor)?, emit_count(c)?],
        )),
        PlayerAction::AddMana(count, production) => {
            let (mana, riders) = emit_mana_production(production)?;
            Ok(app(
                "addManaFull",
                vec![
                    emit_reference(actor)?,
                    emit_count(count)?,
                    mana,
                    ilist(riders),
                ],
            ))
        }
        PlayerAction::Create(count, spec, riders) => {
            // CreateToken has no `actor` field in Idris; the creator is
            // implicit.
            if !matches!(actor, Reference::You) {
                return Err(gap("Create has no Idris actor slot"));
            }
            Ok(app(
                "createTokenAttacking",
                vec![
                    emit_count(count)?,
                    emit_token_spec(spec)?,
                    attacking_maybe(riders)?,
                ],
            ))
        }
        PlayerAction::Sacrifice(r) => Ok(app(
            "sacrificeBy",
            vec![emit_reference(actor)?, reference_as_predicate(r)?],
        )),
        PlayerAction::Move(r, dest, riders) => {
            if !matches!(actor, Reference::You) {
                return Err(gap("Move has no Idris actor slot"));
            }
            emit_action(&Action::Move(r.clone(), dest.clone(), riders.clone()))
        }
        PlayerAction::Mill(n) => {
            // "[actor] mills n" = move the actor's own top n library cards to
            // the graveyard. Idris has no bare `Mill` verb (only the
            // `KeywordActionSpec` composite tag over the primitives, per
            // `Core.idr`'s `Composite` doc comment), so it's rebuilt here.
            let top = app("topFrom", vec![emit_count(n)?, emit_reference(actor)?]);
            let move_ = app(
                "MoveArranged",
                vec![
                    top,
                    "SameOrder".to_string(),
                    "(ToZone Graveyard)".to_string(),
                ],
            );
            Ok(app(
                "Composite",
                vec!["Mill".to_string(), format!("(Act {move_})")],
            ))
        }
        PlayerAction::Tap(r) => Ok(app("Tap", vec![emit_reference(r)?])),
        PlayerAction::Untap(r) => Ok(app("Untap", vec![emit_reference(r)?])),
        PlayerAction::PutCounters(r, kind, count) => {
            if !matches!(actor, Reference::You) {
                return Err(gap("PutCounters has no Idris actor slot"));
            }
            let k = counter_ref_idris(kind.as_str())?;
            Ok(app(
                "PutCounters",
                vec![k, emit_count(count)?, emit_reference(r)?],
            ))
        }
        PlayerAction::RemoveCounters(r, kind, count) => {
            let k = counter_ref_idris(kind.as_str())?;
            Ok(app(
                "RemoveCounters",
                vec![k, emit_count(count)?, emit_reference(r)?],
            ))
        }
        PlayerAction::Shuffle => Ok(app("shuffleBy", vec![emit_reference(actor)?])),
        PlayerAction::SetLife(c) => Ok(app(
            "setLifeToBy",
            vec![emit_reference(actor)?, emit_count(c)?],
        )),
        PlayerAction::Reveal { what, .. } => {
            if !matches!(actor, Reference::You) {
                return Err(gap("Reveal has no Idris actor slot"));
            }
            Ok(app("Reveal", vec![emit_reference(what)?]))
        }
        PlayerAction::RemoveDamage(r) => Ok(app("RemoveAllDamage", vec![emit_reference(r)?])),
        PlayerAction::WinGame => Ok(app(
            "Conclude",
            vec![app("WinGame", vec![emit_reference(actor)?])],
        )),
        PlayerAction::LoseGame => Ok(app(
            "Conclude",
            vec![app("LoseGame", vec![emit_reference(actor)?])],
        )),
        // [CR#705.1,706.1,901.9]: Idris's `RollDice`/`FlipCoins`/
        // `RollPlanarDie` carry NO actor slot at all (unlike
        // `Draw`/`GainLife`/…'s `{default You actor}`) — a non-`You` agent
        // is a gap, matching the `Tap`/`Untap`/`PutCounters`/`Create` family
        // above.
        PlayerAction::FlipCoins(count) => {
            if !matches!(actor, Reference::You) {
                return Err(gap("FlipCoins has no Idris actor slot"));
            }
            Ok(app("FlipCoins", vec![emit_count(count)?]))
        }
        PlayerAction::RollDice(count, sides) => {
            if !matches!(actor, Reference::You) {
                return Err(gap("RollDice has no Idris actor slot"));
            }
            Ok(app("RollDice", vec![emit_count(count)?, sides.to_string()]))
        }
        PlayerAction::RollPlanarDie => {
            if !matches!(actor, Reference::You) {
                return Err(gap("RollPlanarDie has no Idris actor slot"));
            }
            Ok("RollPlanarDie".to_string())
        }
        PlayerAction::VentureIntoDungeon
        | PlayerAction::GetEmblem(_)
        | PlayerAction::GetDesignation(_)
        | PlayerAction::ChooseAndNote(..)
        | PlayerAction::CopySpell(_)
        | PlayerAction::RestartGame => Err(gap(format!(
            "{pa:?} not yet mapped (no Idris counterpart or not implemented)"
        ))),
        PlayerAction::Expanded(_) => Err(gap(
            "unexpanded PlayerAction macro invocation remained after expand_all",
        )),
    }
}

fn emit_mana_production(p: &ManaProduction) -> Result<(String, Vec<String>), Gap> {
    let (spec, riders) = match p {
        ManaProduction::Bare(spec) => (spec, &[][..]),
        ManaProduction::WithRiders { mana, riders } => (mana, riders.as_slice()),
    };
    let mana = emit_mana_spec(spec)?;
    let mut out_riders = Vec::new();
    for r in riders {
        match r {
            deckmaste_core::ManaRider::SpendOnly(f) => {
                out_riders.push(app("SpendOnly", vec![emit_filter(f)?]));
            }
            deckmaste_core::ManaRider::GrantOnSpend(_)
            | deckmaste_core::ManaRider::TriggerOnSpend(_)
            | deckmaste_core::ManaRider::Persistent(_)
            | deckmaste_core::ManaRider::Snow
            | deckmaste_core::ManaRider::Expanded(_) => {
                return Err(gap("ManaRider variant not yet mapped"));
            }
        }
    }
    Ok((mana, out_riders))
}

fn emit_mana_spec(spec: &ManaSpec) -> R {
    Ok(match spec {
        ManaSpec::AnyColor => "AnyColor".to_string(),
        ManaSpec::OneOf(cs) => app(
            "OneOf",
            vec![ilist(
                cs.iter().map(|c| emit_color_or_colorless(*c)).collect(),
            )],
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
            )],
        ),
        ManaSpec::Specific(c) => app("OfColor", vec![emit_color_or_colorless(*c)]),
        ManaSpec::AmongColorsOf(r) => app("AmongColorsOf", vec![emit_reference(r)?]),
        ManaSpec::ProducedByEvent => "ProducedByEvent".to_string(),
    })
}

// ===========================================================================
// Token characteristics (for `Create`)
// ===========================================================================

fn emit_token_spec(spec: &TokenSpec) -> R {
    let token = match spec {
        TokenSpec::Token(t) => t.clone(),
        TokenSpec::Named(name) => name.resolve().ok_or_else(|| {
            gap(format!(
                "unresolvable predefined token name: {}",
                name.as_str()
            ))
        })?,
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
        ],
    ))
}

// ===========================================================================
// Modification / StaticEffect / Deontic
// ===========================================================================

fn emit_numeric_op(op: &NumericOp) -> R {
    Ok(match op {
        NumericOp::Set(c) => app("Set", vec![emit_count(c)?]),
        NumericOp::Up(c) => app("Up", vec![emit_count(c)?]),
        NumericOp::Down(c) => app("Down", vec![emit_count(c)?]),
    })
}

fn emit_collection_op<T>(op: &deckmaste_core::CollectionOp<T>, elem: impl Fn(&T) -> R) -> R {
    Ok(match op {
        deckmaste_core::CollectionOp::Set(items) => app("Set", vec![map_list(items, &elem)?]),
        deckmaste_core::CollectionOp::Add(item) => app("Add", vec![elem(item)?]),
        deckmaste_core::CollectionOp::Remove(item) => app("Remove", vec![elem(item)?]),
    })
}

/// One `Modification` -> a `Alter <characteristic> <op>` (or a bare special
/// like `SwitchPowerToughness`); flattened into the caller's list (a Rust
/// `Several` splices its members in, mirroring `Modification::flatten`).
fn emit_modification_ops(m: &Modification, out: &mut Vec<String>) -> Result<(), Gap> {
    match m {
        Modification::Power(op) => out.push(app(
            "Alter",
            vec!["Power".to_string(), emit_numeric_op(op)?],
        )),
        Modification::Toughness(op) => out.push(app(
            "Alter",
            vec!["Toughness".to_string(), emit_numeric_op(op)?],
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
            ],
        )),
        Modification::CardTypes(op) => out.push(app(
            "Alter",
            vec![
                "Types".to_string(),
                emit_collection_op(op, |t| emit_type_name(t.as_str()))?,
            ],
        )),
        Modification::Subtypes(op) => out.push(app(
            "Alter",
            vec![
                "Subtypes".to_string(),
                emit_collection_op(op, |ident: &Ident| emit_subtype_ref(ident.as_str()))?,
            ],
        )),
        Modification::Supertypes(op) => out.push(app(
            "Alter",
            vec![
                "Supertypes".to_string(),
                emit_collection_op(op, |s| Ok(emit_supertype(*s)))?,
            ],
        )),
        Modification::GainAbility(ability) => {
            out.push(app("GrantAbility", vec![emit_ability(ability)?]));
        }
        Modification::LoseAbility(name) => {
            let spec = keywordspec_idris(name.as_str()).ok_or_else(|| {
                gap(format!(
                    "unmapped keyword in LoseAbility: {}",
                    name.as_str()
                ))
            })?;
            out.push(app("LoseKeyword", vec![spec]));
        }
        Modification::LoseAllAbilities => out.push("LoseAbilities".to_string()),
        Modification::CantHaveAbility(_) => {
            return Err(gap(
                "Modification::CantHaveAbility has no Idris counterpart",
            ));
        }
        Modification::SetController(r) => out.push(app("GainControl", vec![emit_reference(r)?])),
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
            for m in inner {
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
        _ => Ok(app("ApplyAll", vec![ilist(ops)])),
    }
}

fn emit_player_mod(m: &PlayerMod) -> R {
    Ok(match m {
        PlayerMod::SetTo(attr, c) => app("SetTo", vec![emit_player_attr(*attr), emit_count(c)?]),
        PlayerMod::Raise(attr, c) => app("Raise", vec![emit_player_attr(*attr), emit_count(c)?]),
        PlayerMod::Lower(attr, c) => app("Lower", vec![emit_player_attr(*attr), emit_count(c)?]),
        PlayerMod::NoMax(attr) => app("NoMax", vec![emit_player_attr(*attr)]),
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
        CostChange::Reduce(cs) => app("Reduce", vec![emit_cost_components_as_mana(cs)?]),
        CostChange::Increase(cs) => app("Increase", vec![emit_cost_components_as_mana(cs)?]),
        CostChange::Additional { components } => {
            let mut out = Vec::with_capacity(components.len());
            for c in components {
                out.push(emit_cost_component(c)?);
            }
            app("Additional", vec![ilist(out)])
        }
        CostChange::Scaled { change, times } => app(
            "ScaledBy",
            vec![emit_cost_change(change)?, emit_count(times)?],
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

fn emit_replacement(r: &deckmaste_core::Replacement) -> R {
    use deckmaste_core::Replacement as Repl;
    Ok(match r {
        Repl::Instead { would, instead } => {
            let (kinds, facets) = emit_event_filter(would)?;
            app(
                "Replaces",
                vec![event_query(&kinds, &facets), emit_effect(instead)?],
            )
        }
        Repl::Also { would, also } => {
            let (kinds, facets) = emit_event_filter(would)?;
            app(
                "Also",
                vec![event_query(&kinds, &facets), emit_effect(also)?],
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

fn emit_prevention(p: &deckmaste_core::Prevention) -> R {
    use deckmaste_core::Prevention as P;
    Ok(match p {
        P::PreventAll { from, to, .. } => {
            let q = event_query(
                &["(DealDamage Nothing)".to_string()],
                &damage_facets(from, to)?,
            );
            app("Replaces", vec![q, "(Sequentially [])".to_string()])
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
                ],
            )
        }
        P::PreventNextInstance { .. } => {
            return Err(gap(
                "Prevention::PreventNextInstance not yet mapped (instance-vs-amount limiting has no Idris ReplaceLimit shape)",
            ));
        }
    })
}

fn damage_facets(from: &Predicate, to: &Predicate) -> Result<Vec<String>, Gap> {
    let mut facets = Vec::new();
    if !matches!(from, Predicate::Any) {
        facets.push(app("Agent", vec![emit_filter(from)?]));
    }
    if !matches!(to, Predicate::Any) {
        facets.push(app("Patient", vec![emit_filter(to)?]));
    }
    Ok(facets)
}

fn emit_static_effect(se: &StaticEffect) -> R {
    Ok(match se {
        StaticEffect::Modify(r, m) => {
            app("Modify", vec![emit_reference(r)?, emit_modification(m)?])
        }
        StaticEffect::Each(sel, inner) => app(
            "Each",
            vec![
                format!("(Existing {})", emit_selection(sel)?),
                emit_static_effect(inner)?,
            ],
        ),
        StaticEffect::Conditionally(cond, inner) => app(
            "While",
            vec![emit_condition(cond)?, emit_static_effect(inner)?],
        ),
        StaticEffect::Deontic(d) => emit_deontic(d)?,
        StaticEffect::CostModifier { of, change } => app(
            "CostModifier",
            vec![emit_filter(of)?, emit_cost_change(change)?],
        ),
        StaticEffect::CostOption(oc) => {
            let mut costs = Vec::with_capacity(oc.components.len());
            for c in &oc.components {
                costs.push(emit_cost_component(c)?);
            }
            let repeatable = if oc.repeatable { "True" } else { "False" };
            app(
                "costOptionRep",
                vec![ilit(oc.tag.as_str()), ilist(costs), repeatable.to_string()],
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
                ],
            )
        }
        StaticEffect::ModifyPlayer(r, m) => app(
            "ModifyPlayer",
            vec![emit_reference(r)?, emit_player_mod(m)?],
        ),
        StaticEffect::Replacement(r) => emit_replacement(r)?,
        StaticEffect::Prevention(p) => emit_prevention(p)?,
        StaticEffect::CantPrevent { .. } => {
            return Err(gap("StaticEffect::CantPrevent has no Idris counterpart"));
        }
        StaticEffect::SpendAsThough { .. } => {
            return Err(gap("StaticEffect::SpendAsThough has no Idris counterpart"));
        }
        StaticEffect::AsThough(_) => {
            return Err(gap(
                "StaticEffect::AsThough has no concrete Rust variants yet",
            ));
        }
        StaticEffect::Sba { when, then } => {
            app("Sba", vec![emit_condition(when)?, emit_effect(then)?])
        }
        StaticEffect::OutcomeGate { who, gate } => {
            let g = match gate {
                deckmaste_core::OutcomeGateKind::CantLose => "CantLose",
                deckmaste_core::OutcomeGateKind::CantWin => "CantWin",
            };
            app("OutcomeGate", vec![g.to_string(), emit_filter(who)?])
        }
        StaticEffect::CantHappen(ef) => {
            let (kinds, facets) = emit_event_filter(ef)?;
            app("CantHappen", vec![event_query(&kinds, &facets)])
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
                ],
            )
        }
        StaticEffect::PayPips(class, act) => {
            let class_s = match class {
                deckmaste_core::PipClass::Generic => "Generic".to_string(),
                deckmaste_core::PipClass::Colored(c) => app("Colored", vec![emit_color(*c)]),
            };
            let act_s = match act {
                deckmaste_core::PayAct::TapToPay(f) => app("TapToPay", vec![emit_filter(f)?]),
                deckmaste_core::PayAct::ExileToPay(f) => app("ExileToPay", vec![emit_filter(f)?]),
            };
            app("PayPips", vec![class_s, act_s])
        }
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
fn emit_ignore_rule(ir: &deckmaste_core::IgnoreRule) -> String {
    match ir {
        deckmaste_core::IgnoreRule::IgnoreLowest => "IgnoreLowest".to_string(),
        deckmaste_core::IgnoreRule::IgnoreChosen(n) => app("IgnoreChosen", vec![n.to_string()]),
    }
}

fn emit_deontic(d: &Deontic) -> R {
    Ok(match d {
        Deontic::Cant(action) => app("Constrain", vec!["Forbid".to_string(), emit_deed(action)?]),
        Deontic::Must(action) => app("Constrain", vec!["Require".to_string(), emit_deed(action)?]),
        Deontic::May(action) => emit_can(action)?,
        Deontic::Gate(action, costs) => {
            let mut cs = Vec::with_capacity(costs.len());
            for c in costs {
                cs.push(emit_cost_component(c)?);
            }
            app(
                "Priced",
                vec![
                    "AtDeclaration".to_string(),
                    format!("(Costs {})", ilist(cs)),
                    emit_deed(action)?,
                ],
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
                deckmaste_core::AlternativeCost::Free => "[]".to_string(),
                deckmaste_core::AlternativeCost::Components(cs) => {
                    let mut out = Vec::with_capacity(cs.len());
                    for c in cs {
                        out.push(emit_cost_component(c)?);
                    }
                    ilist(out)
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
                None => app("mayCastForFrom", vec![costs, from_zones]),
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
        let deed = app("Enact", vec!["Cast".to_string(), by_pred, what_pred]);
        let window_maybe = match window {
            None => "Nothing".to_string(),
            Some(deckmaste_core::Timing::InstantSpeed) => "(Just AsInstant)".to_string(),
            Some(deckmaste_core::Timing::SorcerySpeed) => "(Just AsSorcery)".to_string(),
            Some(_) => {
                return Err(gap(
                    "Timing::DuringTurn/DuringStep has no Idris Timing counterpart",
                ));
            }
        };
        return Ok(app("canWindow", vec![deed, window_maybe]));
    }
    Ok(app("Can", vec![emit_deed(action)?]))
}

/// `Deed.Enact`'s `patient : Predicate b k` carries a totally FREE `k` (no
/// function ties it to `patientScope r`, unlike `agent`'s `agentScope r`,
/// which reduces to a concrete kind since `r` is always a literal
/// constructor here) — so a default `Predicate::Any` patient can't elaborate as
/// the kind-polymorphic-empty `And []` (Idris is left with an unsolved `k`
/// hole). `Anyone` is Core.idr's own precedent for this
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
            vec!["Attack".to_string(), emit_filter(by)?, deed_patient(on)?],
        ),
        DeonticAction::Block { by, on, count } => match count {
            None => app(
                "Enact",
                vec!["Block".to_string(), emit_filter(by)?, deed_patient(on)?],
            ),
            Some(bound) => {
                let attacker = if matches!(on, Predicate::Any) { by } else { on };
                app(
                    "BlockedBy",
                    vec![emit_filter(attacker)?, count_bound_as_quantity(bound)?],
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
                vec!["Target".to_string(), agent, deed_patient(on)?],
            )
        }
        DeonticAction::Attach { what, to } => app(
            "Enact",
            vec!["Attach".to_string(), emit_filter(what)?, deed_patient(to)?],
        ),
        DeonticAction::Cast { .. } => return Err(gap("Deontic::Cant/Must(Cast) not yet mapped")),
        DeonticAction::Play { what, by, from } => {
            if from.is_some() {
                return Err(gap("DeonticAction::Play{from} not yet mapped"));
            }
            app(
                "Enact",
                vec!["Play".to_string(), emit_filter(by)?, deed_patient(what)?],
            )
        }
        DeonticAction::Activate { what, by } => app(
            "Enact",
            vec![
                "Activate".to_string(),
                emit_filter(by)?,
                deed_patient(what)?,
            ],
        ),
        DeonticAction::Regenerate { by, on } => app(
            "Enact",
            vec![
                "Regenerate".to_string(),
                emit_filter(by)?,
                deed_patient(on)?,
            ],
        ),
        // `cant (Enact Counter spellOrAbility (SameAs This))` ([CR#701.6a]):
        // `agentScope Counter = AnObject` is forced concretely (a literal
        // constructor here), so an `Any` agent resolves as the empty
        // conjunction like `Target`'s agent.
        DeonticAction::Counter { by, on } => app(
            "Enact",
            vec!["Counter".to_string(), emit_filter(by)?, deed_patient(on)?],
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
        ilist(kinds.to_vec()),
        ilist(facets.to_vec())
    )
}

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
fn emit_event_filter(ef: &EventFilter) -> Result<(Vec<String>, Vec<String>), Gap> {
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
                facets.push(app("Agent", vec![emit_filter(what)?]));
            }
            (vec![kind], facets)
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
                facets.push(app("Agent", vec![emit_filter(source)?]));
            }
            if !matches!(to, Predicate::Any) {
                facets.push(app("Patient", vec![emit_filter(to)?]));
            }
            (vec![kind], facets)
        }
        EventFilter::LifeGained { who, amount } => {
            reject_amount(amount)?;
            (vec!["GainLife".to_string()], actor_facet(who)?)
        }
        EventFilter::LifeLost { who, amount } => {
            reject_amount(amount)?;
            (vec!["LoseLife".to_string()], actor_facet(who)?)
        }
        EventFilter::Drawn { who, amount } => {
            reject_amount(amount)?;
            (vec!["Draw".to_string()], actor_facet(who)?)
        }
        EventFilter::CounterPlaced { kind, on, amount } => {
            reject_amount(amount)?;
            if kind.is_some() {
                return Err(gap(
                    "EventFilter::CounterPlaced{kind} not yet mapped (Idris PutCounters carries no kind)",
                ));
            }
            (vec!["PutCounters".to_string()], agent_facet(on)?)
        }
        EventFilter::CounterRemoved { kind, on, amount } => {
            reject_amount(amount)?;
            if kind.is_some() {
                return Err(gap("EventFilter::CounterRemoved{kind} not yet mapped"));
            }
            (vec!["RemoveCounters".to_string()], agent_facet(on)?)
        }
        EventFilter::Cast { who, what } => (
            vec!["(Begins Cast)".to_string()],
            actor_agent_facets(who, what)?,
        ),
        EventFilter::Played { who, what } => (
            vec!["(Begins Play)".to_string()],
            actor_agent_facets(who, what)?,
        ),
        EventFilter::ActivatedAb { who, what } => (
            vec!["(Begins Activate)".to_string()],
            actor_agent_facets(who, what)?,
        ),
        EventFilter::AttackDeclared { by, against } => {
            if !matches!(against, Predicate::Any) {
                return Err(gap(
                    "AttackDeclared{against} not yet mapped (no Idris facet for the defending player)",
                ));
            }
            (vec!["(Begins Attack)".to_string()], agent_facet(by)?)
        }
        EventFilter::BlockDeclared { by, of } => {
            if !matches!(of, Predicate::Any) {
                return Err(gap("BlockDeclared{of} not yet mapped"));
            }
            (vec!["(Begins Block)".to_string()], agent_facet(by)?)
        }
        EventFilter::Attached { what, to } => {
            if !matches!(to, Predicate::Any) {
                return Err(gap(
                    "Attached{to} not yet mapped (no Idris patient facet for Begins Attach)",
                ));
            }
            (vec!["(Begins Attach)".to_string()], agent_facet(what)?)
        }
        EventFilter::StateBecame { of, becomes } => {
            let kind = match becomes {
                deckmaste_core::StateChange::Tapped => "(Becomes Tapped)".to_string(),
                deckmaste_core::StateChange::Untapped => "(Becomes Untapped)".to_string(),
                deckmaste_core::StateChange::Phased(deckmaste_core::Phasing::Out) => {
                    "(Becomes PhasedOut)".to_string()
                }
                deckmaste_core::StateChange::Phased(deckmaste_core::Phasing::In) => {
                    return Err(gap(
                        "StateChange::Phased(In) has no Idris ObjectState transition (only PhasedOut)",
                    ));
                }
                deckmaste_core::StateChange::TurnedFace(deckmaste_core::Face::Down) => {
                    "(Becomes FaceDown)".to_string()
                }
                deckmaste_core::StateChange::TurnedFace(deckmaste_core::Face::Up) => {
                    return Err(gap(
                        "StateChange::TurnedFace(Up) has no Idris ObjectState transition",
                    ));
                }
            };
            (vec![kind], agent_facet(of)?)
        }
        EventFilter::BecomesTarget { .. } => {
            return Err(gap(
                "EventFilter::BecomesTarget has no Idris EventKind counterpart",
            ));
        }
        EventFilter::StepBegins { at, whose } => {
            let kind = format!("(BeginStep {})", emit_phase_step(*at)?);
            let facet = match whose {
                deckmaste_core::WhoseTurn::EachPlayers => None,
                deckmaste_core::WhoseTurn::Your => {
                    Some("(Whenever (TurnOf (SameAs You)))".to_string())
                }
                deckmaste_core::WhoseTurn::AnOpponents => {
                    Some("(Whenever (TurnOf OpponentOf))".to_string())
                }
            };
            (vec![kind], facet.into_iter().collect())
        }
        EventFilter::ControlChanged { of, to } => {
            let mut facets = Vec::new();
            if !matches!(of, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(of)?]));
            }
            if !matches!(to, Predicate::Any) {
                facets.push(app("Actor", vec![emit_filter(to)?]));
            }
            (vec!["GainControl".to_string()], facets)
        }
        EventFilter::DesignationChanged { .. } => {
            return Err(gap(
                "EventFilter::DesignationChanged has no Idris EventKind counterpart",
            ));
        }
        EventFilter::TokenCreated { what, by } => {
            let mut facets = Vec::new();
            if !matches!(what, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(what)?]));
            }
            if !matches!(by, Predicate::Any) {
                facets.push(app("Actor", vec![emit_filter(by)?]));
            }
            (vec!["CreateToken".to_string()], facets)
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
            vec![format!("(FlipCoin {})", opt_bool(*won))],
            actor_facet(by)?,
        ),
        EventFilter::DiceRolled { by } => (vec!["RollDice".to_string()], actor_facet(by)?),
        // [CR#106.12,106.12a]: the ONE event kind whose `producesMana` cap
        // is `True` — the tapped land is the Agent, its controller the
        // Actor (Dictate of Karametra's trigger: `MkEventQuery [TapForMana]
        // [Actor you, Agent (hasType Land)]`).
        EventFilter::TapForMana { what, by } => {
            let mut facets = actor_facet(by)?;
            if !matches!(what, Predicate::Any) {
                facets.push(app("Agent", vec![emit_filter(what)?]));
            }
            (vec!["TapForMana".to_string()], facets)
        }
        // [CR#901.9,901.9d]: the fixed six-face planar die — `face`
        // wildcards like `ZoneChanged`'s zones (`Nothing` = any face).
        EventFilter::RollPlanarDie { by, face } => (
            vec![format!("(RollPlanarDie {})", opt_planar_face(*face))],
            actor_facet(by)?,
        ),
        EventFilter::BecameDay | EventFilter::BecameNight => {
            return Err(gap("day/night events have no Idris EventKind counterpart"));
        }
        EventFilter::AllOf(fs) => merge_all_of(fs)?,
        EventFilter::OneOf(fs) => merge_one_of(fs)?,
        EventFilter::Not(_) => return Err(gap("EventFilter::Not not yet mapped")),
        EventFilter::OneOrMore(_) => return Err(gap("EventFilter::OneOrMore not yet mapped")),
        EventFilter::Nth { .. } => return Err(gap("EventFilter::Nth not yet mapped")),
        EventFilter::When(inner, cond) => {
            let (kinds, mut facets) = emit_event_filter(inner)?;
            facets.push(app("Whenever", vec![emit_condition(cond)?]));
            (kinds, facets)
        }
        EventFilter::Within(..) => {
            return Err(gap(
                "EventFilter::Within not yet mapped (a history-lane refinement, not valid on a live trigger)",
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

fn opt_zone(z: Option<deckmaste_core::Zone>) -> String {
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

fn emit_planar_face(f: deckmaste_core::PlanarFace) -> &'static str {
    match f {
        deckmaste_core::PlanarFace::Blank => "Blank",
        deckmaste_core::PlanarFace::Chaos => "Chaos",
        deckmaste_core::PlanarFace::Planeswalker => "Planeswalker",
    }
}

fn opt_planar_face(f: Option<deckmaste_core::PlanarFace>) -> String {
    match f {
        None => "Nothing".to_string(),
        Some(f) => format!("(Just {})", emit_planar_face(f)),
    }
}

fn actor_facet(who: &Predicate) -> Result<Vec<String>, Gap> {
    if matches!(who, Predicate::Any) {
        Ok(vec![])
    } else {
        Ok(vec![app("Actor", vec![emit_filter(who)?])])
    }
}

fn agent_facet(what: &Predicate) -> Result<Vec<String>, Gap> {
    if matches!(what, Predicate::Any) {
        Ok(vec![])
    } else {
        Ok(vec![app("Agent", vec![emit_filter(what)?])])
    }
}

fn actor_agent_facets(who: &Predicate, what: &Predicate) -> Result<Vec<String>, Gap> {
    let mut facets = actor_facet(who)?;
    facets.extend(agent_facet(what)?);
    Ok(facets)
}

/// `AllOf` merges constituents that share the same emitted kind list,
/// concatenating facets (the same occurrence, refined further).
fn merge_all_of(fs: &[EventFilter]) -> Result<(Vec<String>, Vec<String>), Gap> {
    let mut kinds: Option<Vec<String>> = None;
    let mut facets = Vec::new();
    for f in fs {
        let (k, mut fa) = emit_event_filter(f)?;
        if !k.is_empty() {
            match &kinds {
                None => kinds = Some(k),
                Some(existing) if *existing == k => {}
                Some(_) => return Err(gap("AllOf constituents specify conflicting event kinds")),
            }
        }
        facets.append(&mut fa);
    }
    Ok((kinds.unwrap_or_default(), facets))
}

/// `OneOf` widens the kind list when every disjunct shares the same facets
/// (the common case: "whenever a creature attacks or blocks").
fn merge_one_of(fs: &[EventFilter]) -> Result<(Vec<String>, Vec<String>), Gap> {
    let mut kinds = Vec::new();
    let mut shared_facets: Option<Vec<String>> = None;
    for f in fs {
        let (k, fa) = emit_event_filter(f)?;
        kinds.extend(k);
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
    Ok((kinds, shared_facets.unwrap_or_default()))
}

// ===========================================================================
// OneShotEffect
// ===========================================================================

fn emit_effect(e: &OneShotEffect) -> R {
    Ok(match e {
        // [CR#104.2b,104.3e]: win/lose live in Rust as `PlayerAction::
        // {WinGame,LoseGame}` (an `Action`, reached via `Act`), but Idris's
        // `Conclude : Outcome b -> OneShotEffect b` is its OWN top-level
        // `OneShotEffect` constructor, never wrapped in `Act` — the bridge
        // `emit_player_action`'s own `WinGame`/`LoseGame` arms already build
        // `(Conclude (WinGame/LoseGame actor))` whole; this arm must not
        // re-wrap that in `Act`.
        OneShotEffect::Act(a @ Action::By(_, PlayerAction::WinGame | PlayerAction::LoseGame)) => {
            emit_action(a)?
        }
        OneShotEffect::Act(a) => app("Act", vec![emit_action(a)?]),
        OneShotEffect::Sequentially(es) => app("Sequentially", vec![map_list(es, emit_effect)?]),
        // One pre-application snapshot, one batch ([CR#701.14a]) — a plain list
        // (no `SeqList` threading, unlike `Sequentially`). Fight's guarded body
        // rides this.
        OneShotEffect::Simultaneously(es) => {
            app("Simultaneously", vec![map_list(es, emit_effect)?])
        }
        OneShotEffect::Continuously(c) => app(
            "Continuously",
            vec![emit_duration(&c.duration)?, emit_static_effect(&c.effect)?],
        ),
        OneShotEffect::Until(duration, parts) => {
            let d = emit_duration(duration)?;
            let mut wrapped = Vec::with_capacity(parts.len());
            for p in parts {
                wrapped.push(app("Continuously", vec![d.clone(), emit_static_effect(p)?]));
            }
            app("Sequentially", vec![ilist(wrapped)])
        }
        OneShotEffect::Label { .. } => {
            return Err(gap(
                "OneShotEffect::Label has no Idris OneShotEffect counterpart",
            ));
        }
        OneShotEffect::SeparatePiles(sp) => emit_separate_piles(sp)?,
        OneShotEffect::ChoosePile(cp) => emit_choose_pile(cp)?,
        // `mayWith` takes the (default-`Nothing`) `ifDid`/`ifNot` positionally.
        OneShotEffect::May(m) => app(
            "mayWith",
            vec![
                emit_effect(&m.effect)?,
                opt_effect(&m.if_did)?,
                opt_effect(&m.if_not)?,
            ],
        ),
        // `ifElse` takes the (default-`Nothing`) `otherwise` positionally.
        OneShotEffect::If(i) => app(
            "ifElse",
            vec![
                emit_condition(&i.condition)?,
                emit_effect(&i.then)?,
                opt_effect(&i.otherwise)?,
            ],
        ),
        // `mayPayFull`/`mustPayBy` take the (default-`You`) actor positionally.
        OneShotEffect::MayPay(m) => app(
            "mayPayFull",
            vec![
                emit_reference(&m.actor)?,
                emit_cost(&m.cost)?,
                emit_effect(&m.and_then)?,
                opt_effect(&m.or_else)?,
            ],
        ),
        OneShotEffect::MustPay(m) => app(
            "mustPayBy",
            vec![
                emit_reference(&m.actor)?,
                emit_cost(&m.cost)?,
                emit_effect(&m.or_else)?,
            ],
        ),
        OneShotEffect::AdditionalCost(ac) => app(
            "AdditionalCost",
            vec![emit_cost(&ac.pay)?, emit_effect(&ac.body)?],
        ),
        OneShotEffect::Each(e) => app(
            "Each",
            vec![emit_binder(&e.binder)?, emit_effect(&e.effect)?],
        ),
        OneShotEffect::With(w) => app("With", vec![emit_binder(&w.binder)?, emit_effect(&w.body)?]),
        OneShotEffect::Distribute(d) => app(
            "Distribute",
            vec![
                emit_count(&d.amount)?,
                emit_binder(&d.binder)?,
                emit_effect(&d.body)?,
            ],
        ),
        OneShotEffect::Noting(_) => return Err(gap("OneShotEffect::Noting not yet mapped")),
        OneShotEffect::Delayed(ta) => {
            let (kinds, facets) = emit_event_filter(&ta.event)?;
            app(
                "Delayed",
                vec![event_query(&kinds, &facets), emit_effect(&ta.effect)?],
            )
        }
        OneShotEffect::Reflexive(ta) => app("Reflexive", vec![emit_effect(&ta.effect)?]),
        OneShotEffect::Modal(m) => emit_modal(m)?,
        OneShotEffect::Targeted(t) => emit_targeted(t)?,
        OneShotEffect::Repeat(n, body) => app("Repeat", vec![emit_count(n)?, emit_effect(body)?]),
        OneShotEffect::RevealUntil(r) => app(
            "RevealUntil",
            vec![
                emit_reference(&r.whose)?,
                emit_filter(&r.matches)?,
                emit_effect(&r.body)?,
            ],
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
fn opt_effect(e: &Option<Box<OneShotEffect>>) -> R {
    match e {
        None => Ok("Nothing".to_string()),
        Some(inner) => Ok(format!("(Just {})", emit_effect(inner)?)),
    }
}

fn emit_duration(d: &deckmaste_core::Duration) -> R {
    Ok(match d {
        deckmaste_core::Duration::FixedUntil(marker) => match marker {
            deckmaste_core::TurnMarker::EndOfTurn => "UntilEndOfTurn".to_string(),
            deckmaste_core::TurnMarker::EndOfCombat => {
                return Err(gap(
                    "TurnMarker::EndOfCombat has no Idris Duration counterpart",
                ));
            }
            deckmaste_core::TurnMarker::YourNextTurn => {
                return Err(gap(
                    "TurnMarker::YourNextTurn has no Idris Duration counterpart",
                ));
            }
        },
        deckmaste_core::Duration::UntilEvent(ef) => {
            let (kinds, facets) = emit_event_filter(ef)?;
            app("UntilEvent", vec![event_query(&kinds, &facets)])
        }
        deckmaste_core::Duration::ForAsLongAs(cond) => {
            app("ForAsLongAs", vec![emit_condition(cond)?])
        }
        deckmaste_core::Duration::ForThisEvent => "ForThisEvent".to_string(),
        deckmaste_core::Duration::EndOfGame => "Forever".to_string(),
    })
}

fn emit_targeted(t: &deckmaste_core::Targeted) -> R {
    let mut specs = Vec::with_capacity(t.targets.len());
    for ts in &t.targets {
        specs.push(emit_target_spec(ts)?);
    }
    Ok(format!(
        "(Targeted {} {})",
        ilist(specs),
        emit_effect(&t.effect)?
    ))
}

fn emit_modal(m: &deckmaste_core::Modal) -> R {
    let choose = emit_choose_spec(&m.choose)?;
    let mut modes = Vec::with_capacity(m.modes.len());
    for mode in &m.modes {
        modes.push(emit_mode(mode)?);
    }
    Ok(format!("(Modal {} {})", choose, ilist(modes)))
}

fn emit_choose_spec(cs: &deckmaste_core::ChooseSpec) -> R {
    let repeats = if cs.repeats { "True" } else { "False" };
    Ok(app(
        "mkChooseSpecRep",
        vec![emit_quantity(&cs.count)?, repeats.to_string()],
    ))
}

fn emit_mode(m: &deckmaste_core::Mode) -> R {
    let cost_maybe = match &m.cost {
        None => "Nothing".to_string(),
        Some(cs) => {
            let mut components = Vec::with_capacity(cs.len());
            for c in cs {
                components.push(emit_cost_component(c)?);
            }
            format!("(Just (Costs {}))", ilist(components))
        }
    };
    Ok(app("mkModeCost", vec![emit_effect(&m.effect)?, cost_maybe]))
}

fn emit_separate_piles(_sp: &deckmaste_core::SeparatePiles) -> R {
    Err(gap(
        "OneShotEffect::SeparatePiles not yet mapped (Idris's DivideAndChoose has a different two-pile shape)",
    ))
}

fn emit_choose_pile(_cp: &deckmaste_core::ChoosePile) -> R {
    Err(gap("OneShotEffect::ChoosePile not yet mapped"))
}

// ===========================================================================
// Ability / KeywordAbility
// ===========================================================================

/// Emit ONE ability (never splicing) — the Idris `Ability b` text.
fn emit_ability(a: &Ability) -> R {
    Ok(match a {
        Ability::Static(se) => app("Static", vec![emit_static_effect(se)?]),
        // `activatedFull` exposes window/limits/from/activationGuard
        // positionally (all default in the constructor).
        Ability::Activated(aa) => {
            let window = match aa.window {
                None | Some(deckmaste_core::Timing::InstantSpeed) => "AsInstant",
                Some(deckmaste_core::Timing::SorcerySpeed) => "AsSorcery",
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
                ],
            )
        }
        // `triggeredFull` exposes limits/from positionally.
        Ability::Triggered(ta) => {
            let (kinds, mut facets) = emit_event_filter(&ta.event)?;
            if let Some(cond) = &ta.condition {
                facets.push(app("Whenever", vec![emit_condition(cond)?]));
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
                ],
            )
        }
        Ability::Spell(sa) => app("Spell", vec![emit_effect(&sa.effect)?]),
        Ability::Keyword(ka) => app("Keyword", vec![emit_keyword_ability(ka)?]),
        Ability::Innate(inner) => emit_ability(inner)?,
        Ability::Expanded(_) => {
            return Err(gap(
                "unexpanded Ability macro invocation remained after expand_all",
            ));
        }
    })
}

fn emit_use_limit(l: &deckmaste_core::UseLimit) -> String {
    match l {
        deckmaste_core::UseLimit::OncePerTurn => "OncePerTurn",
        deckmaste_core::UseLimit::OncePerGame => "OncePerGame",
        deckmaste_core::UseLimit::LoyaltyOncePerTurn => "LoyaltyOncePerTurn",
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
            app("Composite", vec![tag, emit_ability_list(abilities)?])
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
fn emit_ability_or_splice(a: &Ability) -> Result<Vec<String>, Gap> {
    match a {
        Ability::Innate(inner) => emit_ability_or_splice(inner),
        Ability::Keyword(KeywordAbility::Composite { name, abilities })
            if keywordspec_idris(name.as_str()).is_none() =>
        {
            let mut out = Vec::new();
            for inner in abilities {
                out.extend(emit_ability_or_splice(inner)?);
            }
            Ok(out)
        }
        other => Ok(vec![emit_ability(other)?]),
    }
}

fn emit_ability_list(abs: &[Ability]) -> R {
    let mut items = Vec::new();
    for a in abs {
        items.extend(emit_ability_or_splice(a)?);
    }
    Ok(ilist(items))
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
        ],
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
/// # Errors
/// A [`Gap`] naming the first Rust grammar shape encountered with no (or
/// not-yet-implemented) Idris translation.
pub fn emit_card_expr(card: &Card, plugin: &Plugin) -> R {
    load_subtype_categories(&plugin.subtypes);
    load_counter_scopes(&plugin.counters);
    load_designation_scopes(&plugin.designations);
    match card {
        Card::Normal(face) => Ok(app("Normal", vec![emit_characteristics_from_face(face)?])),
        Card::TwoFaced { .. } => Err(gap("Card::TwoFaced not yet mapped")),
    }
}

/// Every non-todo card in `plugin_dir/cards/**/*.ron`, parsed through
/// `plugin`'s macro scope (its builtin sibling prelude already loaded) —
/// mirrors `validate::validate_plugin`'s card walk, returning the parsed
/// values (still macro-`Expanded`-wrapped; callers `expand_all()` them)
/// instead of pass/fail counts.
///
/// # Errors
/// If a directory isn't listable, a file isn't readable, or a non-todo card
/// doesn't parse.
pub fn load_all_cards(
    plugin_dir: &std::path::Path,
    plugin: &crate::plugin::Plugin,
) -> anyhow::Result<Vec<Card>> {
    use anyhow::Context;
    use deckmaste_core::plugin::CARDS_DIR;
    use deckmaste_core::plugin::is_todo_source;

    let mut cards = Vec::new();
    for path in crate::plugin::ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
        let source = crate::plugin::read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        let card: Card = plugin
            .macros
            .read_str(&source)
            .with_context(|| format!("parsing {}", path.display()))?;
        cards.push(card);
    }
    Ok(cards)
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
    use deckmaste_core::AlternativeCost;
    use deckmaste_core::CostTag;

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
            from: Some(deckmaste_core::Zone::Graveyard),
            window: None,
            cost: Some(AlternativeCost::Components(vec![CostComponent::Tap])),
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
            cost: Some(AlternativeCost::Components(vec![CostComponent::Tap])),
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

    /// `StatePredicate::WasCastWith` resolves its tag through the same
    /// `keywordspec_idris` table as `Cast.tag` (a `KeywordSpec`, unlike
    /// `WasPaidWith`'s bare string literal).
    #[test]
    fn was_cast_with_emits_keywordspec() {
        let out = emit_state_filter(&StatePredicate::WasCastWith(CostTag::from("Evoke")))
            .expect("WasCastWith(Evoke) should emit");
        assert_eq!(out, "(WasCastWith Evoke)");
    }

    /// `Condition::CastWith` has no direct Idris `Condition` counterpart, so
    /// it desugars to `Matches This (WasCastWith tag)`.
    #[test]
    fn condition_cast_with_desugars_to_matches() {
        let out = emit_condition(&Condition::CastWith(CostTag::from("Flashback")))
            .expect("CastWith(Flashback) should emit");
        assert_eq!(out, "(Matches This (WasCastWith Flashback))");
    }
}
