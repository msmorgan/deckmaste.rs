//! A generic structural view of anything that derives `Serialize`, built by
//! driving the derive rather than hand-writing a visitor.
//!
//! This is the same pattern as `xtask/src/english/shape.rs`'s `Shape`, with
//! one deliberate change: `Shape` discards scalar *values* (so that, say,
//! ten thousand distinct card names collapse to one `Scalar("str")` shape
//! for corpus-wide pattern matching); [`View`] keeps them, because the
//! macro-frames unifier needs to recover a literal such as `Count = 3` from
//! the tree it walks.

use std::fmt;

use serde::Serialize;
use serde::ser;

use crate::compile::HoleClass;

/// One node of a `Serialize` value's tree, with scalar values preserved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// A primitive. `kind` is the serde data-model name of the
    /// `serialize_*` method that produced it (`"u32"`, `"str"`, `"bool"`,
    /// ...); `repr` is that value rendered to text.
    Scalar { kind: &'static str, repr: String },
    /// A unit struct or unit variant: lexical identity survives here, which
    /// is why it must not be conflated with `Scalar { kind: "str", .. }`.
    Unit {
        name: &'static str,
        variant: Option<&'static str>,
    },
    /// A newtype struct or newtype variant wrapping one child.
    Newtype {
        name: &'static str,
        variant: Option<&'static str>,
        inner: Box<View>,
    },
    /// A struct or struct variant with named fields, in declaration order.
    Node {
        name: &'static str,
        variant: Option<&'static str>,
        fields: Vec<(&'static str, View)>,
    },
    /// A sequence, tuple, tuple struct, or tuple variant.
    Seq(Vec<View>),
    /// A map.
    Map(Vec<(View, View)>),
    /// `Option::None`.
    Absent,
    /// A typed hole: the one node kind [`of`] never produces. The frame
    /// compiler puts these in by *relocation* — replacing the witness it
    /// substituted into a frame's text with the hole that witness stood
    /// for. See [`mod@crate::compile`].
    Hole { index: usize, class: HoleClass },
}

/// Build a [`View`] from anything that derives `Serialize`.
pub fn of<T: Serialize + ?Sized>(value: &T) -> View {
    // The builder is infallible; the error type exists only to satisfy serde.
    value.serialize(ViewBuilder).unwrap_or(View::Absent)
}

/// One hop down a [`View`]. Together these address a node without borrowing
/// it, which is what lets a [`CompiledFrame`](crate::compile::CompiledFrame)
/// keep a side table pointing into its own tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathStep {
    /// Into [`View::Node`]'s field of this name.
    Field(&'static str),
    /// Into [`View::Seq`]'s element at this index.
    Index(usize),
    /// Through [`View::Newtype`]'s single child.
    Inner,
    /// Into the key of [`View::Map`]'s entry at this index.
    MapKey(usize),
    /// Into the value of [`View::Map`]'s entry at this index.
    MapValue(usize),
}

impl fmt::Display for PathStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathStep::Field(name) => write!(formatter, ".{name}"),
            PathStep::Index(index) => write!(formatter, "[{index}]"),
            PathStep::Inner => formatter.write_str("()"),
            PathStep::MapKey(index) => write!(formatter, "{{{index}}}.key"),
            PathStep::MapValue(index) => write!(formatter, "{{{index}}}.value"),
        }
    }
}

/// An address inside a [`View`], from the root down.
///
/// Paths recorded by the frame compiler are valid against the *compiled*
/// tree, after relocation and citation normalization — that is the tree a
/// consumer holds.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreePath(pub Vec<PathStep>);

impl fmt::Display for TreePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return formatter.write_str("<root>");
        }
        for step in &self.0 {
            write!(formatter, "{step}")?;
        }
        Ok(())
    }
}

impl TreePath {
    /// This path with one more step on the end.
    #[must_use]
    pub fn then(&self, step: PathStep) -> TreePath {
        let mut steps = self.0.clone();
        steps.push(step);
        TreePath(steps)
    }

    /// Whether `self` addresses `other` or an ancestor of it.
    #[must_use]
    pub fn is_prefix_of(&self, other: &TreePath) -> bool {
        other.0.starts_with(&self.0)
    }

    /// Repairs this path after the element at `removed` was taken out of the
    /// [`View::Seq`] at `sequence`.
    ///
    /// Removing a sequence element renumbers every later sibling, so any
    /// recorded path that runs *through* one of them silently starts
    /// addressing its neighbour. That is not hypothetical here: citation
    /// normalization lifts a count out of a nominal's `modifiers`
    /// ([`Normalization::QuantityToDeterminer`](crate::Normalization)), and
    /// `41 <Param> card` really does park a second modifier after it. A path
    /// that does not pass through the sequence, or passes through an earlier
    /// sibling, is left alone.
    pub fn shift_after_removal(&mut self, sequence: &TreePath, removed: usize) {
        let depth = sequence.0.len();
        if self.0.len() <= depth || !self.0.starts_with(&sequence.0) {
            return;
        }
        if let PathStep::Index(index) = &mut self.0[depth]
            && *index > removed
        {
            *index -= 1;
        }
    }

    /// The node this path addresses, or `None` if it does not resolve.
    #[must_use]
    pub fn resolve<'tree>(&self, root: &'tree View) -> Option<&'tree View> {
        let mut node = root;
        for step in &self.0 {
            node = step_into(node, *step)?;
        }
        Some(node)
    }

    /// The node this path addresses, mutably.
    pub fn resolve_mut<'tree>(&self, root: &'tree mut View) -> Option<&'tree mut View> {
        let mut node = root;
        for step in &self.0 {
            node = step_into_mut(node, *step)?;
        }
        Some(node)
    }
}

fn step_into(node: &View, step: PathStep) -> Option<&View> {
    match (node, step) {
        (View::Node { fields, .. }, PathStep::Field(name)) => fields
            .iter()
            .find_map(|(key, value)| (*key == name).then_some(value)),
        (View::Seq(items), PathStep::Index(index)) => items.get(index),
        (View::Newtype { inner, .. }, PathStep::Inner) => Some(inner),
        (View::Map(entries), PathStep::MapKey(index)) => entries.get(index).map(|(key, _)| key),
        (View::Map(entries), PathStep::MapValue(index)) => {
            entries.get(index).map(|(_, value)| value)
        }
        _ => None,
    }
}

fn step_into_mut(node: &mut View, step: PathStep) -> Option<&mut View> {
    match (node, step) {
        (View::Node { fields, .. }, PathStep::Field(name)) => fields
            .iter_mut()
            .find_map(|(key, value)| (*key == name).then_some(value)),
        (View::Seq(items), PathStep::Index(index)) => items.get_mut(index),
        (View::Newtype { inner, .. }, PathStep::Inner) => Some(inner),
        (View::Map(entries), PathStep::MapKey(index)) => entries.get_mut(index).map(|(key, _)| key),
        (View::Map(entries), PathStep::MapValue(index)) => {
            entries.get_mut(index).map(|(_, value)| value)
        }
        _ => None,
    }
}

impl View {
    /// This node's children, each with the step that reaches it.
    #[must_use]
    pub fn children(&self) -> Vec<(PathStep, &View)> {
        match self {
            View::Node { fields, .. } => fields
                .iter()
                .map(|(name, value)| (PathStep::Field(name), value))
                .collect(),
            View::Seq(items) => items
                .iter()
                .enumerate()
                .map(|(index, item)| (PathStep::Index(index), item))
                .collect(),
            View::Newtype { inner, .. } => vec![(PathStep::Inner, inner.as_ref())],
            View::Map(entries) => entries
                .iter()
                .enumerate()
                .flat_map(|(index, (key, value))| {
                    [
                        (PathStep::MapKey(index), key),
                        (PathStep::MapValue(index), value),
                    ]
                })
                .collect(),
            View::Scalar { .. } | View::Unit { .. } | View::Absent | View::Hole { .. } => {
                Vec::new()
            }
        }
    }

    /// Every node in this tree, deepest-last within each branch, each with
    /// the path that reaches it from here.
    #[must_use]
    pub fn walk(&self) -> Vec<(TreePath, &View)> {
        let mut out = Vec::new();
        self.walk_into(&TreePath::default(), &mut out);
        out
    }

    fn walk_into<'tree>(&'tree self, at: &TreePath, out: &mut Vec<(TreePath, &'tree View)>) {
        out.push((at.clone(), self));
        for (step, child) in self.children() {
            child.walk_into(&at.then(step), out);
        }
    }

    /// The `name` of a [`View::Node`], [`View::Newtype`] or [`View::Unit`] —
    /// the Rust type the node came from. `None` for everything else.
    #[must_use]
    pub fn type_name(&self) -> Option<&'static str> {
        match self {
            View::Node { name, .. } | View::Newtype { name, .. } | View::Unit { name, .. } => {
                Some(name)
            }
            _ => None,
        }
    }

    /// The `variant` of a [`View::Node`], [`View::Newtype`] or
    /// [`View::Unit`]. `None` for a struct (rather than a variant) and for
    /// everything else.
    #[must_use]
    pub fn variant_name(&self) -> Option<&'static str> {
        match self {
            View::Node { variant, .. }
            | View::Newtype { variant, .. }
            | View::Unit { variant, .. } => *variant,
            _ => None,
        }
    }

    /// Whether this node carries no content of its own: `None`, or an empty
    /// sequence.
    ///
    /// Load-bearing in relocation. A witness-substituted frame's tree has
    /// nodes whose *other* fields are structurally empty — `determiner: None`
    /// and `modifiers: []` around an opaque head noun — and a hole must be
    /// allowed to grow through those, because nothing in them came from the
    /// frame's own text. Anything else (a `Newtype`, a `Unit`, a `Scalar`)
    /// *is* frame material and stops the hole.
    #[must_use]
    pub fn is_vacuous(&self) -> bool {
        match self {
            View::Absent => true,
            View::Seq(items) => items.is_empty(),
            _ => false,
        }
    }
}

/// The walk cannot fail; `serde` still requires an error type.
#[derive(Debug, thiserror::Error)]
#[error("view construction cannot fail")]
struct Unreachable;

impl ser::Error for Unreachable {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Self
    }
}

struct ViewBuilder;

struct SeqBuilder {
    items: Vec<View>,
}

struct NodeBuilder {
    name: &'static str,
    variant: Option<&'static str>,
    fields: Vec<(&'static str, View)>,
}

struct MapBuilder {
    entries: Vec<(View, View)>,
    key: Option<View>,
}

impl ser::Serializer for ViewBuilder {
    type Ok = View;
    type Error = Unreachable;
    type SerializeSeq = SeqBuilder;
    type SerializeTuple = SeqBuilder;
    type SerializeTupleStruct = SeqBuilder;
    type SerializeTupleVariant = SeqBuilder;
    type SerializeMap = MapBuilder;
    type SerializeStruct = NodeBuilder;
    type SerializeStructVariant = NodeBuilder;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "bool",
            repr: value.to_string(),
        })
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "i8",
            repr: value.to_string(),
        })
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "i16",
            repr: value.to_string(),
        })
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "i32",
            repr: value.to_string(),
        })
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "i64",
            repr: value.to_string(),
        })
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "u8",
            repr: value.to_string(),
        })
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "u16",
            repr: value.to_string(),
        })
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "u32",
            repr: value.to_string(),
        })
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "u64",
            repr: value.to_string(),
        })
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "f32",
            repr: value.to_string(),
        })
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "f64",
            repr: value.to_string(),
        })
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "char",
            repr: value.to_string(),
        })
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "str",
            repr: value.to_string(),
        })
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "bytes",
            repr: format!("{value:?}"),
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Absent)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Scalar {
            kind: "unit",
            repr: "()".to_string(),
        })
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(View::Unit {
            name,
            variant: None,
        })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(View::Unit {
            name,
            variant: Some(variant),
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(View::Newtype {
            name,
            variant: None,
            inner: Box::new(value.serialize(ViewBuilder)?),
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(View::Newtype {
            name,
            variant: Some(variant),
            inner: Box::new(value.serialize(ViewBuilder)?),
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqBuilder { items: Vec::new() })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqBuilder { items: Vec::new() })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SeqBuilder { items: Vec::new() })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SeqBuilder { items: Vec::new() })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapBuilder {
            entries: Vec::new(),
            key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(NodeBuilder {
            name,
            variant: None,
            fields: Vec::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(NodeBuilder {
            name,
            variant: Some(variant),
            fields: Vec::new(),
        })
    }
}

impl SeqBuilder {
    fn push<T>(&mut self, value: &T) -> Result<(), Unreachable>
    where
        T: ?Sized + Serialize,
    {
        self.items.push(value.serialize(ViewBuilder)?);
        Ok(())
    }
}

impl ser::SerializeSeq for SeqBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Seq(self.items))
    }
}

impl ser::SerializeTuple for SeqBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Seq(self.items))
    }
}

impl ser::SerializeTupleStruct for SeqBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Seq(self.items))
    }
}

impl ser::SerializeTupleVariant for SeqBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Seq(self.items))
    }
}

impl ser::SerializeMap for MapBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(ViewBuilder)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(ViewBuilder)?;
        let key = self.key.take().unwrap_or(View::Absent);
        self.entries.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(View::Map(self.entries))
    }
}

impl NodeBuilder {
    fn push<T>(&mut self, key: &'static str, value: &T) -> Result<(), Unreachable>
    where
        T: ?Sized + Serialize,
    {
        self.fields.push((key, value.serialize(ViewBuilder)?));
        Ok(())
    }

    fn finish(self) -> View {
        View::Node {
            name: self.name,
            variant: self.variant,
            fields: self.fields,
        }
    }
}

impl ser::SerializeStruct for NodeBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish())
    }
}

impl ser::SerializeStructVariant for NodeBuilder {
    type Ok = View;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish())
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_english::Numeral;
    use deckmaste_english::features::Comma;
    use deckmaste_english::features::Conjunction;
    use deckmaste_english::features::GapState;
    use deckmaste_english::features::Onset;
    use deckmaste_english::features::PronounCase;
    use deckmaste_english::features::PronounClass;
    use deckmaste_english::syntax::ComparativeWord;
    use deckmaste_english::syntax::CoordinationJunction;
    use deckmaste_english::syntax::Demonstrative;
    use deckmaste_english::syntax::NounPhrase;
    use deckmaste_english::syntax::NounPhraseCoordination;
    use deckmaste_english::syntax::NumberLiteral;
    use deckmaste_english::syntax::OpaqueLexeme;
    use deckmaste_english::syntax::Quantity;
    use deckmaste_english::syntax::QuantityValue;
    use deckmaste_english::word::Noun;
    use deckmaste_english::word::NounInstance;
    use deckmaste_english::word::PronounInstance;
    use deckmaste_english::word::Vocab;

    use super::*;

    fn field<'a>(view: &'a View, wanted: &str) -> &'a View {
        let View::Node { fields, .. } = view else {
            panic!("expected a field-bearing View, got {view:#?}");
        };
        fields
            .iter()
            .find_map(|(name, value)| (*name == wanted).then_some(value))
            .unwrap_or_else(|| panic!("missing field {wanted:?} in {view:#?}"))
    }

    #[test]
    fn scalar_values_survive() {
        // shape.rs drops values; View must keep them — Count recovery depends on it.
        let a = of(&3u32);
        let b = of(&4u32);
        assert_ne!(a, b);
        assert_eq!(
            a,
            View::Scalar {
                kind: "u32",
                repr: "3".into()
            }
        );
    }

    #[test]
    fn unit_variants_keep_identity_and_structs_keep_fields() {
        #[derive(serde::Serialize)]
        enum E {
            A,
            B(u8),
        }
        #[derive(serde::Serialize)]
        struct S {
            x: E,
            y: Option<u8>,
        }
        let v = of(&S { x: E::A, y: None });
        let View::Node { name, fields, .. } = &v else { panic!() };
        assert_eq!(*name, "S");
        assert_eq!(fields[0].0, "x");
        assert_eq!(fields[1].1, View::Absent);
        assert_ne!(of(&E::A), of(&E::B(0)));
    }

    #[test]
    fn generated_noun_projection_keeps_typed_identity_and_opaque_bytes() {
        // The parser's generated noun builders still project the public
        // NounInstance value, so the generic spelling view must retain both
        // its identity variant and an opaque lexeme's exact source spelling.
        let known = of(&NounInstance::try_singular(Noun::Word(Vocab::Card))
            .expect("card has a validated singular form"));
        let opaque = of(
            &NounInstance::try_mass(Noun::Opaque(OpaqueLexeme::new("BlOrPlE")))
                .expect("opaque identities have a validated mass form"),
        );

        assert!(matches!(
            known,
            View::Newtype {
                name: "NounInstance",
                variant: Some("Singular"),
                ..
            }
        ));
        let View::Newtype {
            name: "NounInstance",
            variant: Some("Mass"),
            inner,
        } = opaque
        else {
            panic!("opaque noun form was erased")
        };
        let View::Newtype {
            name: "Noun",
            variant: Some("Opaque"),
            inner,
        } = *inner
        else {
            panic!("opaque identity was erased")
        };
        let View::Newtype {
            name: "OpaqueLexeme",
            variant: None,
            inner,
        } = *inner
        else {
            panic!("opaque spelling wrapper was erased")
        };
        assert_eq!(
            *inner,
            View::Scalar {
                kind: "str",
                repr: "BlOrPlE".to_owned(),
            }
        );
    }

    #[test]
    fn quantity_view_keeps_tuple_variant_notation_and_comparative_identity() {
        let number = NumberLiteral {
            value: 3,
            numeral: Numeral::Roman,
        };
        let comparison = of(&Quantity::try_or_comparison(
            QuantityValue::Literal(number),
            ComparativeWord::Greater,
        )
        .expect("a literal comparative quantity is valid"));
        let View::Node {
            name: "Quantity",
            variant: Some("OrComparison"),
            fields,
        } = comparison
        else {
            panic!("quantity comparison variant identity was erased")
        };
        assert_eq!(fields[0].0, "value");
        assert!(matches!(
            fields[0].1,
            View::Newtype {
                name: "QuantityValue",
                variant: Some("Literal"),
                ..
            }
        ));
        assert_eq!(
            fields[1],
            (
                "comparative",
                View::Unit {
                    name: "ComparativeWord",
                    variant: Some("Greater"),
                },
            )
        );

        let disjunction =
            of(&Quantity::try_or(number, number).expect("a literal disjunction quantity is valid"));
        assert!(matches!(
            disjunction,
            View::Node {
                name: "Quantity",
                variant: Some("Or"),
                ..
            }
        ));
        assert_ne!(
            of(
                &Quantity::try_or_comparison(
                    QuantityValue::Literal(number),
                    ComparativeWord::More,
                )
                .expect("a literal comparative quantity is valid"),
            ),
            disjunction,
            "the two binary quantity families must remain frame-distinguishable"
        );
    }

    #[test]
    fn canonical_feature_aliases_keep_legacy_view_names() {
        assert_eq!(
            of(&Onset::Vowel),
            View::Unit {
                name: "InitialSound",
                variant: Some("Vowel"),
            }
        );
        assert_eq!(
            field(
                &of(&PronounInstance {
                    pronoun: PronounClass::They,
                    case: PronounCase::Object,
                }),
                "pronoun",
            ),
            &View::Unit {
                name: "Pronoun",
                variant: Some("They"),
            }
        );
        assert_eq!(
            of(&GapState::Object),
            View::Unit {
                name: "RelativeGap",
                variant: Some("Object"),
            }
        );
    }

    #[test]
    fn conjunction_fields_keep_context_specific_legacy_view_names() {
        let predicate = of(&CoordinationJunction {
            conjunction: Some(Conjunction::Then),
            comma: Comma::Present,
        });
        assert_eq!(
            field(&predicate, "conjunction"),
            &View::Unit {
                name: "PredicateConjunction",
                variant: Some("Then"),
            }
        );

        let nominal = of(&NounPhraseCoordination {
            conjunction: Some(Conjunction::Plus),
            phrase: NounPhrase::Demonstrative(Demonstrative::This),
        });
        assert_eq!(
            field(&nominal, "conjunction"),
            &View::Unit {
                name: "NounPhraseConjunction",
                variant: Some("Plus"),
            }
        );
    }
}
