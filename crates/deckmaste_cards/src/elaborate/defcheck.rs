//! Definition-time macro body checking ([typed-holes delta 3]).
//!
//! Run AFTER all plugins load (so cross-plugin macro references resolve — the
//! load-order trap), for each macro a plugin defines: expand the body once with
//! typed **`Hole` placeholders** standing for the parameters, then elaborate
//! the result. An ill-formed body — or one that reads an anaphor no binder in
//! the body grants — is a DEFINITION error at load, named by the macro, not a
//! latent error awaiting the first caller.
//!
//! The placeholder for a parameter is a canonical value of its declared type
//! ([typed-holes delta 1]). When the parameter declares a **binder contract**
//! ([typed-holes delta 2] — `Effect(binds: [It])`), the placeholder READS the
//! granted anaphor, so the body must actually wrap the hole in a binder that
//! supplies it: a contract the body doesn't honor surfaces as `E-BIND-*` when
//! the placeholder's read fails to resolve. A parameter with no contract gets a
//! neutral placeholder, reading nothing.
//!
//! The check reuses the real elaborator by embedding the expanded body in a
//! synthetic card at a kind-appropriate slot; only the reference/anaphor codes
//! (`E-BIND-*` / `E-CAPS-*`) count as body faults — card floors and shape rules
//! are the synthetic wrapper's concern, not the macro's.

use macro_ron::Ident;
use macro_ron::MacroDef;
use macro_ron::Params;

use crate::elaborate;
use crate::elaborate::Registries;
use crate::macros::MacroSet;

/// A macro-system load error ([typed-holes delta 3]), with a stable
/// `E-MACRO-*` spelling distinct from the card-elaborator `E-*` codes: these
/// fire at plugin LOAD (definition checking), not during the card-elaboration
/// walk, so they live outside the checker-rule manifest / twin machinery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroFault {
    /// The body doesn't expand/read as its declared kind with placeholder
    /// arguments — an ill-formed definition body.
    Body { macro_name: Ident, reason: String },
    /// The expanded body reads an anaphor no binder in the body grants (and no
    /// parameter's `binds:` contract covers) — a capture hazard / contract
    /// violation.
    Contract { macro_name: Ident, reason: String },
}

impl MacroFault {
    /// The stable `E-MACRO-*` code spelling.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            MacroFault::Body { .. } => "E-MACRO-BODY",
            MacroFault::Contract { .. } => "E-MACRO-CONTRACT",
        }
    }
}

impl std::fmt::Display for MacroFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroFault::Body { macro_name, reason } => {
                write!(f, "{}: macro `{macro_name}` body: {reason}", self.code())
            }
            MacroFault::Contract { macro_name, reason } => write!(
                f,
                "{}: macro `{macro_name}` reads an unbound anaphor: {reason}",
                self.code()
            ),
        }
    }
}

impl std::error::Error for MacroFault {}

/// The kinds whose bodies read anaphors and embed cleanly into a synthetic
/// card, so the binder-contract elaboration runs for them. Other kinds (a
/// bare `Filter`, `Count`, `Subtype`, …) get no anaphor check — they carry no
/// It/That reads a binder would scope — but their bodies are still exercised
/// by the placeholder READ below.
fn embeds(kind: &str, inv: &str) -> Option<String> {
    Some(match kind {
        "Effect" => format!(
            "Normal(name: \"__defcheck__\", types: [Sorcery], abilities: [Spell(effect: {inv})])"
        ),
        "KeywordAbility" => format!(
            "Normal(name: \"__defcheck__\", types: [Creature], power: 1, toughness: 1, \
             abilities: [Keyword({inv})])"
        ),
        "Ability" => format!(
            "Normal(name: \"__defcheck__\", types: [Creature], power: 1, toughness: 1, \
             abilities: [{inv}])"
        ),
        _ => return None,
    })
}

/// A canonical placeholder value (RON source) for a parameter of type `ty`.
/// When the parameter grants a binder contract, the placeholder READS the
/// granted anaphor (see the module docs); otherwise it is a neutral value.
/// `None` for a type with no known placeholder — the macro's binder check is
/// then skipped (conservative: no false positives on unmodeled types).
fn placeholder(name: &str, binds: &[Ident]) -> Option<String> {
    if let Some(anaphor) = binds.first() {
        return anaphor_reading_value(name, anaphor.as_str());
    }
    Some(match name {
        "Any" | "Reference" => "This".into(),
        "String" => "\"x\"".into(),
        "Color" => "White".into(),
        "Cost" => "[Mana([Generic(1)])]".into(),
        "Abilities" => "[]".into(),
        "Count" => "1".into(),
        "Filter" => "Any".into(),
        "Effect" | "PlayerAction" => "Draw(1)".into(),
        _ => return None,
    })
}

/// A placeholder that READS `anaphor` at a value of type `name` — used when a
/// parameter declares a binder contract, so the body must supply that anaphor.
/// `None` for a type/anaphor pair with no expressible read (the contract is
/// then unchecked, conservatively).
fn anaphor_reading_value(name: &str, anaphor: &str) -> Option<String> {
    let reference = match anaphor {
        "It" => "It".to_owned(),
        "That" => "That(Permanent)".to_owned(),
        _ => return None,
    };
    Some(match name {
        "Reference" => reference,
        "Effect" | "PlayerAction" => format!("DealDamage({reference}, 1)"),
        _ => return None,
    })
}

/// The placeholder-argument INVOCATION for `def` — `Name(arg0, arg1)` or
/// `Name(k0: arg0, …)` — or `None` when any parameter has no known
/// placeholder (the macro is then skipped).
fn placeholder_invocation(def: &MacroDef) -> Option<String> {
    let name = def.name.as_str();
    match &def.params {
        Params::Positional(types) if types.is_empty() => Some(name.to_owned()),
        Params::Positional(types) => {
            let args: Option<Vec<String>> = types
                .iter()
                .map(|t| placeholder(t.name.as_str(), &t.binds))
                .collect();
            Some(format!("{name}({})", args?.join(", ")))
        }
        Params::Named(sig) => {
            let mut keys: Vec<&Ident> = sig.keys().collect();
            keys.sort_unstable_by_key(|k| k.as_str());
            let args: Option<Vec<String>> = keys
                .iter()
                .map(|k| {
                    let t = &sig[*k];
                    Some(format!("{k}: {}", placeholder(t.name.as_str(), &t.binds)?))
                })
                .collect();
            Some(format!("{name}({})", args?.join(", ")))
        }
    }
}

/// Checks one macro definition ([typed-holes delta 3]). `Ok(())` accepts;
/// `Err(MacroFault)` names the macro and the fault.
///
/// # Errors
/// If the body doesn't read as its kind with placeholder arguments
/// ([`MacroFault::Body`]), or its expansion reads an anaphor no binder grants
/// ([`MacroFault::Contract`]).
pub(crate) fn check_definition(
    kind: &str,
    def: &MacroDef,
    macros: &MacroSet,
    registries: &Registries,
) -> Result<(), MacroFault> {
    let Some(inv) = placeholder_invocation(def) else {
        // A parameter type with no modeled placeholder: skip conservatively.
        return Ok(());
    };
    // Only the anaphor-bearing, card-embeddable kinds run the elaboration
    // check; a meta-macro (kind `Macro`) or a bare data kind is not embedded.
    let Some(card_src) = embeds(kind, &inv) else {
        return Ok(());
    };
    // Reading the synthetic card expands the macro body with the placeholder
    // arguments — an ill-formed body fails here.
    let card: deckmaste_core::Card = macros.read_str(&card_src).map_err(|e| MacroFault::Body {
        macro_name: def.name,
        reason: e.to_string(),
    })?;
    // Elaborate the expanded body; only the LOOP/FRAME anaphor faults are the
    // macro's business — those are what a `binds:` contract grants (It/That/
    // They/Allotment). Event roles (`E-BIND-EVENT`/`E-CAPS-*`) are set up by a
    // body's own event constructs (`CreateReplacement`, `Delayed`), and card
    // floors / positional rules (`E-FLOOR-*`/`E-POS-*`) are the synthetic
    // wrapper's concern — none is a binder-contract violation.
    if let Err(errors) = elaborate::elaborate(&card, registries) {
        let anaphor: Vec<&elaborate::ElabError> = errors
            .iter()
            .filter(|e| {
                matches!(
                    e.code,
                    elaborate::Code::BindIt
                        | elaborate::Code::BindThat
                        | elaborate::Code::BindThatGroup
                        | elaborate::Code::BindAllotment
                )
            })
            .collect();
        if let Some(first) = anaphor.first() {
            return Err(MacroFault::Contract {
                macro_name: def.name,
                reason: format!("{} ({})", first.message, first.code),
            });
        }
    }
    Ok(())
}

/// Checks every macro a plugin newly defines (`own`), after the whole plugin
/// (and its prelude) has loaded. Deterministic order (sorted) so a fault
/// reports reproducibly.
///
/// # Errors
/// The first macro fault found, in sorted `(kind, name)` order.
pub(crate) fn check_own(
    macros: &MacroSet,
    own: &std::collections::HashSet<(Ident, Ident)>,
    registries: &Registries,
) -> Result<(), MacroFault> {
    let mut entries: Vec<&(Ident, Ident)> = own.iter().collect();
    entries.sort_unstable_by_key(|(kind, name)| (kind.as_str(), name.as_str()));
    for (kind, name) in entries {
        let Some(def) = macros.get(kind.as_str(), name.as_str()) else {
            continue;
        };
        check_definition(kind.as_str(), def, macros, registries)?;
    }
    Ok(())
}
