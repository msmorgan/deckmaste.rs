//! Guard normalization: the single authority on what "the guard's constant"
//! means, used identically at compile time and at match/render time.
//!
//! # The rule
//!
//! A guard is a param pre-binding — `when: [(0, "You")]`, read as "this frame
//! applies when param 0 is `You`". The stored string is a RON *spelling* of
//! the constant, and RON spellings are not unique: `Exactly(1)` and
//! `Range(Some(1), Some(1))` are the same
//! [`Quantity`](deckmaste_core::Quantity), because `Exactly` is a macro whose
//! expansion is the range. Comparing spellings would therefore miss silently.
//!
//! So guard satisfaction is defined on **fully-expanded canonical form**:
//!
//! 1. read the spelling as the type its param declares,
//! 2. [`expand_all`](macro_ron::Expand::expand_all) it — dropping every
//!    remembered macro invocation down to the value it stood for,
//! 3. take its [`View`].
//!
//! A card-side argument runs through step 2 and 3 of *the same code*
//! ([`normalized`]), and the guard holds exactly when the two `View`s are
//! equal. That single-authority property is the whole point: a second
//! implementation on the match side is a defect, not an optimization, because
//! the two would drift and the drift would present as a frame that quietly
//! stops matching.
//!
//! A consequence worth stating: a guard holds for **any parseable RON
//! spelling** of its constant. The catalog's readable sugar is therefore the
//! preferred stored spelling and is kept exactly as authored — nothing
//! rewrites `when: [(0, "Exactly(1)")]` into an expanded form on disk.
//!
//! # Groundness
//!
//! A guard's term must be **ground**: it names a constant, so a free
//! `Param(…)` in it is meaningless and is a compile error. `macro_ron`
//! rejects a stray `Param` at read time (there is no expansion frame for it
//! to resolve against), and [`ensure_ground`] re-checks the expanded tree so
//! the invariant is asserted on the value that is actually stored, not merely
//! implied by the reader.

use std::sync::LazyLock;

use macro_ron::Expand;
use macro_ron::MacroSet;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::View;
use crate::view;

/// The canonical `View` of an already-typed value: expand every remembered
/// macro invocation away, then view it.
///
/// **This is the function both sides of a guard comparison must call.** At
/// compile time [`normalize_source`] calls it on the guard's authored
/// spelling; at match and render time the unifier calls it on the card's
/// argument. Nothing else may define "canonical".
pub fn normalized<T: Expand + Serialize>(value: T) -> View {
    view::of(&value.expand_all())
}

/// Reads `source` at the type `param_type` names, expands it, and returns its
/// canonical [`View`].
///
/// `macros` decides which spellings parse: pass a [`MacroSet`] carrying the
/// plugin macros to accept the readable sugar (`Exactly(1)`), or
/// [`core_reader`] to accept only bare `deckmaste_core` constructors.
///
/// # Errors
/// If `param_type` is not a known param type, if `source` does not parse at
/// it, or if the expanded term is not ground.
pub fn normalize_source(macros: &MacroSet, param_type: &str, source: &str) -> anyhow::Result<View> {
    let read = reader(param_type).ok_or_else(|| {
        anyhow::anyhow!(
            "guard names param type `{param_type}`, which has no RON reader; \
             see `deckmaste_frames::guard`'s table"
        )
    })?;
    let normalized = read(macros, source).map_err(|error| {
        anyhow::anyhow!("guard constant `{source}` is not a valid `{param_type}`: {error}")
    })?;
    ensure_ground(&normalized)
        .map_err(|error| anyhow::anyhow!("guard constant `{source}`: {error}"))?;
    Ok(normalized)
}

/// Rejects a term that still holds a free `Param(…)` or an unexpanded
/// remembered invocation.
///
/// # Reachability
///
/// **Neither branch fires through [`normalize_source`] today**, and that is a
/// property of the layers below rather than of this check:
///
/// - a free `Param` is refused earlier, by `macro_ron`'s reader — a top-level
///   read has no expansion frame for the hole to resolve against, so it errors
///   with "`Param(0)` outside any macro expansion"
///   (`crates/macro_ron/src/expand.rs:161-173`);
/// - an unexpanded invocation (which views as an `Expansion` node, not as the
///   `Expanded` variant holding it) cannot survive [`normalized`], because that
///   is exactly what `expand_all` strips.
///
/// It is kept, and tested at its own entry point, because it asserts the
/// user-ruled invariant on *the value that is actually stored* in
/// [`CompiledGuard::value`](crate::CompiledGuard) rather than inferring it
/// from two other crates' current behaviour. If either of those layers ever
/// admits such a term — a `Quote`-deferred `Param`, a reader that resolves
/// holes leniently, a hand-built guard value — this is the thing that
/// notices.
///
/// # Errors
/// Naming the offending node.
pub fn ensure_ground(term: &View) -> anyhow::Result<()> {
    for (path, node) in term.walk() {
        if node.type_name() == Some("Param") || node.variant_name() == Some("Param") {
            anyhow::bail!("a guard constant must be ground; found a free `Param` at {path}");
        }
        // A *remembered* invocation views as the invocation it re-serializes
        // to — `Expansion` with the macro's name as its variant, wrapping the
        // argument's RAW SOURCE TEXT — not as the `Expanded` variant that
        // holds it. Which is the whole reason the ruling puts guard
        // satisfaction on expanded form: comparing un-normalized views would
        // be comparing RON source strings.
        if node.type_name() == Some("Expansion") {
            anyhow::bail!(
                "a guard constant must be fully expanded; found the unexpanded invocation \
                 `{}` at {path} (this is a bug in `normalized`, not in the authored guard)",
                node.variant_name().unwrap_or("?"),
            );
        }
    }
    Ok(())
}

/// A `MacroSet` that knows deckmaste's RON dialect and its bare-numeral
/// literal positions, but **no plugin macros**.
///
/// Enough for a guard spelled as a plain core constructor (`You`, `This`,
/// `Range(Some(1), Some(1))`), and nothing more: it cannot read the readable
/// sugar the frame catalog prefers (`Exactly(1)` is a `Quantity` *macro*, and
/// `Quantity` itself has only `Range` and `Expanded`). So this is a fixture
/// and a fallback, not a default — [`compile`](crate::compile::compile) takes
/// the macro set as a required argument precisely so that a caller cannot
/// reach for this one by accident and get a guard rejected that the round's
/// own catalog authored.
#[must_use]
pub fn core_reader() -> &'static MacroSet {
    static READER: LazyLock<MacroSet> = LazyLock::new(|| {
        MacroSet::new(deckmaste_core::ron::kinds()).with_options(deckmaste_core::ron::raw_options())
    });
    &READER
}

/// Reads one RON spelling at a fixed Rust type and canonicalizes it.
type Reader = fn(&MacroSet, &str) -> anyhow::Result<View>;

fn read_normalized<T: DeserializeOwned + Expand + Serialize>(
    macros: &MacroSet,
    source: &str,
) -> anyhow::Result<View> {
    Ok(normalized(macros.read_str::<T>(source)?))
}

/// The param-type name a guard may be written at, mapped to the Rust type its
/// spelling is read as.
///
/// Deliberately a mirror of `deckmaste_plugin::macros::param_types()` — the
/// registry that decides which names are legal in a `params:` list — rather
/// than a dependency on it: `deckmaste_plugin` sits *above* this crate (it
/// loads plugins, which will one day include compiled frames), so the arrow
/// cannot point that way. The key set must stay in step with that function;
/// a name missing here surfaces as a clear "has no RON reader" error rather
/// than as a wrong answer.
fn reader(param_type: &str) -> Option<Reader> {
    use deckmaste_core as dc;
    macro_rules! table {
        ($($name:literal => $type:ty),* $(,)?) => {
            match param_type {
                $($name => Some(read_normalized::<$type> as Reader),)*
                _ => None,
            }
        };
    }
    table! {
        "Ability" => dc::Ability,
        "Abilities" => Vec<dc::Ability>,
        "Action" => dc::Action,
        "AsThough" => dc::AsThough,
        "CardTypePredicate" => dc::Predicate,
        "Color" => dc::Color,
        "Condition" => dc::Condition,
        "Cost" => Vec<dc::CostComponent>,
        "CostComponent" => dc::CostComponent,
        "Count" => dc::Count,
        "Counts" => Vec<dc::Count>,
        "CounterRef" => dc::CounterRef,
        "Destination" => dc::Destination,
        "EventFilter" => dc::EventFilter,
        "KeywordAbility" => dc::KeywordAbility,
        "ManaRider" => dc::ManaRider,
        "Modification" => dc::Modification,
        "NumericOp" => dc::NumericOp,
        "OneShotEffect" => dc::OneShotEffect,
        "Predicate" => dc::Predicate,
        "Preference" => dc::strategy::Preference,
        "Quantity" => dc::Quantity,
        "Reference" => dc::Reference,
        "Replacement" => dc::Replacement,
        "Selection" => dc::Selection,
        "StaticEffect" => dc::StaticEffect,
        "String" => String,
        "Subtype" => dc::Subtype,
        "TargetSpec" => dc::TargetSpec,
        "TypeDef" => dc::TypeDef,
        "Uint" => dc::Uint,
        "Zone" => dc::Zone,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ruling's motivating case, in the one form that proves it: two
    /// different authored spellings of the same constant must normalize to
    /// the same `View`, so a guard written either way holds either way.
    ///
    /// `Exactly` is a plugin macro, so it needs a reader that knows it; both
    /// spellings are compared through the same entry point.
    #[test]
    fn one_constant_two_spellings_normalize_alike() {
        let core = core_reader();
        let long = normalize_source(core, "Quantity", "Range(Some(1), Some(1))").unwrap();
        // `Quantity::one()` is the same value a card-side argument would
        // carry; it goes through `normalized`, the identical function.
        let from_value = normalized(deckmaste_core::Quantity::one());
        assert_eq!(long, from_value);
    }

    /// A remembered macro invocation reaching the normalizer as a *value*
    /// (which is how the card side arrives) is expanded away, so its
    /// provenance cannot make an otherwise-equal argument miss its guard.
    #[test]
    fn remembered_provenance_does_not_survive_normalization() {
        use deckmaste_core::Count;
        use deckmaste_core::Quantity;
        let plain = Quantity::Range(Some(Count::Literal(1)), Some(Count::Literal(1)));
        let remembered = Quantity::Expanded(macro_ron::Expansion {
            name: "Exactly".into(),
            args: macro_ron::ExpansionArgs::Positional(vec!["1".into()]),
            template: None,
            value: Box::new(plain.clone()),
        });
        assert_ne!(view::of(&plain), view::of(&remembered), "control");
        assert_eq!(normalized(plain), normalized(remembered));
    }

    /// A free `Param` never reaches [`ensure_ground`]: `macro_ron`'s reader
    /// refuses it first. Asserted on *that* wording, because asserting on
    /// "Param" alone would pass on the echoed spelling in the error's own
    /// prefix and so would pass with the whole check deleted.
    #[test]
    fn a_free_param_in_a_guard_is_refused_by_the_reader() {
        let error = normalize_source(core_reader(), "Reference", "Param(0)").unwrap_err();
        let text = format!("{error:#}");
        assert!(
            text.contains("outside any macro expansion"),
            "expected the reader's own refusal, got: {text}"
        );
    }

    /// The user-ruled groundness invariant, exercised at the entry point that
    /// actually implements it. `normalize_source` cannot reach it (see
    /// [`ensure_ground`]'s reachability note), so it is driven directly, on a
    /// term of exactly the shape the ruling forbids.
    #[test]
    fn ensure_ground_rejects_a_free_param_term() {
        let free = View::Newtype {
            name: "Param",
            variant: None,
            inner: Box::new(View::Scalar {
                kind: "u32",
                repr: "0".into(),
            }),
        };
        // Nested, so the walk (not just a root check) is what finds it.
        let term = View::Node {
            name: "Quantity",
            variant: Some("Range"),
            fields: vec![("lo", free.clone()), ("hi", View::Absent)],
        };
        for term in [free, term] {
            let error = ensure_ground(&term).unwrap_err();
            let text = format!("{error:#}");
            assert!(text.contains("must be ground"), "{text}");
        }
        // And a genuinely ground term passes.
        ensure_ground(&normalized(deckmaste_core::Reference::You)).unwrap();
    }

    /// The other half of the invariant: a value that was *not* run through
    /// `expand_all` is refused rather than stored as if it were canonical.
    #[test]
    fn ensure_ground_rejects_an_unexpanded_term() {
        let unexpanded = view::of(&deckmaste_core::Quantity::Expanded(macro_ron::Expansion {
            name: "Exactly".into(),
            args: macro_ron::ExpansionArgs::Positional(vec!["1".into()]),
            template: None,
            value: Box::new(deckmaste_core::Quantity::one()),
        }));
        let error = ensure_ground(&unexpanded).unwrap_err();
        assert!(format!("{error:#}").contains("fully expanded"), "{error:#}");
    }

    #[test]
    fn an_unknown_param_type_is_named_in_the_error() {
        let error = normalize_source(core_reader(), "Nonesuch", "You").unwrap_err();
        assert!(format!("{error:#}").contains("Nonesuch"));
    }

    #[test]
    fn a_bad_spelling_is_reported_against_its_type() {
        let error = normalize_source(core_reader(), "Reference", "NotAReference").unwrap_err();
        let text = format!("{error:#}");
        assert!(text.contains("Reference"), "{text}");
    }
}
