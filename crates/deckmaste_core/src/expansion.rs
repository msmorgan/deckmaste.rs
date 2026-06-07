//! A remembered macro invocation, carried as a value.
//!
//! Every macroable enum kind has an `Expanded(Expansion<Self>)` variant (see
//! §9 of the building-blocks design). When the macro reader expands `M(args…)`
//! at one of those positions, it wraps the expansion in `Expanded` together
//! with the name and the raw argument source — so the file's meaning survives
//! as the engine sees it (`HasAbility(Flying)`, verb identity, provenance) and
//! so serialization writes the *invocation* back, not the expansion.
//!
//! Equality is provenance-sensitive **by design**: `Expanded(Flying, …)` is
//! never equal to its raw expansion, and two invocations of the same macro
//! with differently-spelled-but-equivalent arguments are unequal. Two files
//! that round-trip to the same text compare equal; that is the contract.

use serde::ser::{SerializeStructVariant, SerializeTupleVariant};
use serde::{Deserialize, Serialize, Serializer};

use crate::Ident;

/// A macro invocation's arguments, each kept as its raw RON source text so
/// serialization can re-emit the invocation verbatim.
///
/// The shape mirrors the macro's signature: positional calls (`M(a, b)`) keep
/// an ordered list, named calls (`M(x: a)`) keep name/source pairs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ExpansionArgs {
    /// Positional arguments, in order, each as raw RON source.
    Positional(Vec<String>),
    /// Named arguments, each a parameter name and its raw RON source.
    Named(Vec<(Ident, String)>),
}

impl ExpansionArgs {
    /// The no-arguments form: an empty positional list. Used as the serde
    /// `default` so a nullary invocation's `Expanded(name: …, value: …)` reads
    /// without an `args` field.
    #[must_use]
    pub fn none() -> Self { ExpansionArgs::Positional(Vec::new()) }

    /// Whether there are no arguments at all — drives the nullary
    /// serialization path (`serialize_unit_variant`, just the name).
    #[must_use]
    pub fn is_none(&self) -> bool {
        match self {
            ExpansionArgs::Positional(args) => args.is_empty(),
            ExpansionArgs::Named(args) => args.is_empty(),
        }
    }
}

/// A remembered macro invocation: the macro's name, the arguments as written,
/// and the value its body expanded to.
///
/// `Deserialize` is derived (the reader synthesizes the matching stream);
/// `Serialize` is manual and writes the **invocation**, not the struct — see
/// the impl. Equality is provenance-sensitive by design (see the module docs).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Expansion<T> {
    /// The macro's name, as invoked.
    pub name: Ident,
    /// The invocation's arguments, as raw source.
    #[serde(default = "ExpansionArgs::none")]
    pub args: ExpansionArgs,
    /// The value the macro body expanded to. Reconstructed on read, never
    /// serialized (serialization emits the invocation instead).
    pub value: Box<T>,
}

/// A RON source fragment, serialized verbatim: `ron::value::RawValue`'s own
/// `Serialize` re-emits its text exactly, so a stored argument's source is
/// written back unchanged. The source came out of the reader, so it is
/// already valid RON — but an `Expansion` constructed in Rust could carry
/// anything, and writing a *different* invocation silently would be data
/// corruption, so the malformed case is a serialization error instead.
fn raw<E: serde::ser::Error>(source: &str) -> Result<&ron::value::RawValue, E> {
    ron::value::RawValue::from_ron(source).map_err(|e| {
        E::custom(format_args!(
            "invalid stored argument source {source:?}: {e}"
        ))
    })
}

impl<T: Serialize> Serialize for Expansion<T> {
    /// Writes the invocation back — the whole point of remembering it.
    ///
    /// The variant name is `Ident::as_str()`, a `&'static str` from the
    /// interner, which is exactly what serde's variant-name lifetimes require.
    /// Argument source is re-emitted through `RawValue`, whose `Serialize`
    /// writes its text verbatim.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = self.name.as_str();
        match &self.args {
            ExpansionArgs::Positional(args) if args.is_empty() => {
                // Nullary: just the name. `Flying`, not `Flying()`.
                serializer.serialize_unit_variant("Expansion", 0, name)
            }
            ExpansionArgs::Positional(args) if args.len() == 1 => {
                // One positional argument: `M(<arg>)`.
                serializer.serialize_newtype_variant("Expansion", 0, name, raw(&args[0])?)
            }
            ExpansionArgs::Positional(args) => {
                // Several positional arguments: `M(<a>, <b>, …)`.
                let mut tv =
                    serializer.serialize_tuple_variant("Expansion", 0, name, args.len())?;
                for arg in args {
                    tv.serialize_field(raw(arg)?)?;
                }
                tv.end()
            }
            ExpansionArgs::Named(args) => {
                // Named arguments: `M(k: <v>, …)`. Field names are interned
                // `&'static str`, satisfying serde's field-name lifetimes.
                let mut sv =
                    serializer.serialize_struct_variant("Expansion", 0, name, args.len())?;
                for (key, value) in args {
                    sv.serialize_field(key.as_str(), raw(value)?)?;
                }
                sv.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Filter;

    fn write<T: Serialize>(value: &T) -> String { crate::ron::options().to_string(value).unwrap() }

    /// A placeholder inner value: serialization never emits it (the invocation
    /// is written instead), so any value works.
    fn inner() -> Filter { Filter::Any }

    #[test]
    fn nullary_serializes_as_the_bare_name() {
        let expansion = Expansion {
            name: "Flying".into(),
            args: ExpansionArgs::none(),
            value: Box::new(inner()),
        };
        assert_eq!(write(&expansion), "Flying");
    }

    #[test]
    fn one_positional_arg_serializes_as_a_call() {
        let expansion = Expansion {
            name: "LandType".into(),
            args: ExpansionArgs::Positional(vec![r#""Forest""#.to_owned()]),
            value: Box::new(inner()),
        };
        assert_eq!(write(&expansion), r#"LandType("Forest")"#);
    }

    #[test]
    fn several_positional_args_serialize_as_a_tuple_call() {
        let expansion = Expansion {
            name: "Pair".into(),
            args: ExpansionArgs::Positional(vec![r#""Forest""#.to_owned(), "Land".to_owned()]),
            value: Box::new(inner()),
        };
        assert_eq!(write(&expansion), r#"Pair("Forest",Land)"#);
    }

    #[test]
    fn named_args_serialize_as_a_struct_call() {
        let expansion = Expansion {
            name: "Boast".into(),
            args: ExpansionArgs::Named(vec![("cost".into(), r#""{1}""#.to_owned())]),
            value: Box::new(inner()),
        };
        assert_eq!(write(&expansion), r#"Boast(cost:"{1}")"#);
    }

    #[test]
    fn equality_is_provenance_sensitive() {
        let flying = Expansion {
            name: "Flying".into(),
            args: ExpansionArgs::none(),
            value: Box::new(inner()),
        };
        let reach = Expansion {
            name: "Reach".into(),
            args: ExpansionArgs::none(),
            value: Box::new(inner()),
        };
        // Same expansion, different invocation name: deliberately unequal.
        assert_ne!(flying, reach);
        assert_eq!(flying, flying.clone());
    }
}
