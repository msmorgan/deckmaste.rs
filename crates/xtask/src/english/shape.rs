//! A generic structural view of the English AST, built by driving the AST's
//! `Serialize` derive.
//!
//! The derive is bought for its free, exhaustive, compiler-checked traversal,
//! not for any serialization format — it stays correct as the AST changes,
//! which a hand-written visitor does not. Nothing here routes through
//! `serde_json::Value`: externally-tagged enums collapse a unit variant and a
//! plain string to the same `Value::String`, so `Vocab::Damage` and the card
//! name "Bonfire of the Damned" would arrive indistinguishable. A `Serializer`
//! sees `serialize_unit_variant` and `serialize_str` as distinct calls, and
//! [`Shape`] keeps them distinct — string *values* are discarded outright,
//! which is what stops card names from becoming millions of signatures.
//!
//! Consumers ([`super::shapes`], [`super::lint`]) walk [`Shape`] rather than
//! the AST, so a new analysis costs a match arm instead of another 650-line
//! traversal to keep in sync.

use std::fmt;

use serde::Serialize;
use serde::ser;

/// One node of the AST, reduced to structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Shape {
    /// A primitive. The value is deliberately dropped — only its type remains.
    Scalar(&'static str),
    /// A unit variant or unit struct: `Vocab::Damage`, `RelativeGap::Object`.
    /// Lexical identity survives here, which is why it must not be conflated
    /// with `Scalar("str")`.
    Unit {
        name: &'static str,
        variant: Option<&'static str>,
    },
    /// A newtype struct or newtype variant wrapping one child.
    Newtype {
        name: &'static str,
        variant: Option<&'static str>,
        inner: Box<Shape>,
    },
    /// A struct or struct variant with named fields.
    Node {
        name: &'static str,
        variant: Option<&'static str>,
        fields: Vec<(&'static str, Shape)>,
    },
    /// A sequence, tuple, tuple struct, or tuple variant.
    Seq(Vec<Shape>),
    /// A map. The AST has none today; handled for completeness.
    Map(Vec<(Shape, Shape)>),
    /// `Option::None`. Load-bearing — a dropped determiner appears here.
    Absent,
}

impl Shape {
    /// This node's label, ignoring its children.
    pub(super) fn label(&self) -> String {
        match self {
            Self::Scalar(kind) => (*kind).to_string(),
            Self::Unit { name, variant } | Self::Newtype { name, variant, .. } => {
                qualify(name, *variant)
            }
            Self::Node { name, variant, .. } => qualify(name, *variant),
            Self::Seq(_) => "[..]".to_string(),
            Self::Map(_) => "{..}".to_string(),
            Self::Absent => "None".to_string(),
        }
    }

    /// The type name, disregarding which variant.
    pub(super) fn type_name(&self) -> Option<&'static str> {
        match self {
            Self::Unit { name, .. } | Self::Newtype { name, .. } | Self::Node { name, .. } => {
                Some(name)
            }
            _ => None,
        }
    }

    pub(super) fn variant(&self) -> Option<&'static str> {
        match self {
            Self::Unit { variant, .. }
            | Self::Newtype { variant, .. }
            | Self::Node { variant, .. } => *variant,
            _ => None,
        }
    }

    /// A named field of a struct-shaped node.
    pub(super) fn field(&self, key: &str) -> Option<&Shape> {
        match self {
            Self::Node { fields, .. } => fields
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, shape)| shape),
            _ => None,
        }
    }

    /// The elements of a sequence, or an empty slice for anything else.
    pub(super) fn elements(&self) -> &[Shape] {
        match self {
            Self::Seq(items) => items,
            _ => &[],
        }
    }

    /// Unwrap newtype layers, which carry no structure of their own.
    pub(super) fn unwrapped(&self) -> &Shape {
        match self {
            Self::Newtype { inner, .. } => inner.unwrapped(),
            other => other,
        }
    }

    /// Every node in the tree, parents before children.
    pub(super) fn walk(&self, visit: &mut impl FnMut(&Shape)) {
        visit(self);
        match self {
            Self::Newtype { inner, .. } => inner.walk(visit),
            Self::Node { fields, .. } => {
                for (_, shape) in fields {
                    shape.walk(visit);
                }
            }
            Self::Seq(items) => {
                for shape in items {
                    shape.walk(visit);
                }
            }
            Self::Map(entries) => {
                for (key, value) in entries {
                    key.walk(visit);
                    value.walk(visit);
                }
            }
            Self::Scalar(_) | Self::Unit { .. } | Self::Absent => {}
        }
    }

    /// A full nested rendering, used to compare two parses for equality.
    pub(super) fn fingerprint(&self) -> String {
        match self {
            Shape::Scalar(kind) => (*kind).to_string(),
            Shape::Unit { .. } => self.label(),
            Shape::Absent => "None".to_string(),
            Shape::Newtype { inner, .. } => format!("{}({})", self.label(), inner.fingerprint()),
            Shape::Node { fields, .. } => {
                let rendered: Vec<_> = fields
                    .iter()
                    .map(|(key, value)| format!("{key}: {}", value.fingerprint()))
                    .collect();
                format!("{}{{{}}}", self.label(), rendered.join(", "))
            }
            Shape::Seq(items) => {
                let rendered: Vec<_> = items.iter().map(Shape::fingerprint).collect();
                format!("[{}]", rendered.join(", "))
            }
            Shape::Map(entries) => {
                let rendered: Vec<_> = entries
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key.fingerprint(), value.fingerprint()))
                    .collect();
                format!("{{{}}}", rendered.join(", "))
            }
        }
    }
}

fn qualify(name: &str, variant: Option<&str>) -> String {
    match variant {
        Some(variant) => format!("{name}::{variant}"),
        None => name.to_string(),
    }
}

/// Build a [`Shape`] from anything the AST derives `Serialize` for.
pub(super) fn of<T: Serialize + ?Sized>(value: &T) -> Shape {
    // The builder is infallible; the error type exists only to satisfy serde.
    value.serialize(ShapeBuilder).unwrap_or(Shape::Absent)
}

/// The walk cannot fail; `serde` still requires an error type.
#[derive(Debug, thiserror::Error)]
#[error("shape construction cannot fail")]
pub(super) struct Unreachable;

impl ser::Error for Unreachable {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Self
    }
}

struct ShapeBuilder;

struct SeqBuilder {
    items: Vec<Shape>,
}

struct NodeBuilder {
    name: &'static str,
    variant: Option<&'static str>,
    fields: Vec<(&'static str, Shape)>,
}

struct MapBuilder {
    entries: Vec<(Shape, Shape)>,
    key: Option<Shape>,
}

impl ser::Serializer for ShapeBuilder {
    type Ok = Shape;
    type Error = Unreachable;
    type SerializeSeq = SeqBuilder;
    type SerializeTuple = SeqBuilder;
    type SerializeTupleStruct = SeqBuilder;
    type SerializeTupleVariant = SeqBuilder;
    type SerializeMap = MapBuilder;
    type SerializeStruct = NodeBuilder;
    type SerializeStructVariant = NodeBuilder;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("bool"))
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("int"))
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("float"))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("float"))
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("char"))
    }

    /// String *values* are discarded. Card names and catalog spellings would
    /// otherwise manufacture a unique signature per card.
    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("str"))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Absent)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Scalar("unit"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Unit {
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
        Ok(Shape::Unit {
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
        Ok(Shape::Newtype {
            name,
            variant: None,
            inner: Box::new(value.serialize(ShapeBuilder)?),
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
        Ok(Shape::Newtype {
            name,
            variant: Some(variant),
            inner: Box::new(value.serialize(ShapeBuilder)?),
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
        self.items.push(value.serialize(ShapeBuilder)?);
        Ok(())
    }
}

impl ser::SerializeSeq for SeqBuilder {
    type Ok = Shape;
    type Error = Unreachable;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Seq(self.items))
    }
}

impl ser::SerializeTuple for SeqBuilder {
    type Ok = Shape;
    type Error = Unreachable;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Seq(self.items))
    }
}

impl ser::SerializeTupleStruct for SeqBuilder {
    type Ok = Shape;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Seq(self.items))
    }
}

impl ser::SerializeTupleVariant for SeqBuilder {
    type Ok = Shape;
    type Error = Unreachable;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Seq(self.items))
    }
}

impl ser::SerializeMap for MapBuilder {
    type Ok = Shape;
    type Error = Unreachable;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(ShapeBuilder)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(ShapeBuilder)?;
        let key = self.key.take().unwrap_or(Shape::Absent);
        self.entries.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Shape::Map(self.entries))
    }
}

impl NodeBuilder {
    fn push<T>(&mut self, key: &'static str, value: &T) -> Result<(), Unreachable>
    where
        T: ?Sized + Serialize,
    {
        self.fields.push((key, value.serialize(ShapeBuilder)?));
        Ok(())
    }

    fn finish(self) -> Shape {
        Shape::Node {
            name: self.name,
            variant: self.variant,
            fields: self.fields,
        }
    }
}

impl ser::SerializeStruct for NodeBuilder {
    type Ok = Shape;
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
    type Ok = Shape;
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
    use super::*;

    #[derive(Clone, Serialize)]
    enum Leaf {
        Alpha,
        Beta,
    }

    #[derive(Serialize)]
    struct Fixture {
        name: String,
        children: Vec<Leaf>,
        tag: Option<Leaf>,
    }

    fn fixture(children: Vec<Leaf>, tag: Option<Leaf>) -> Fixture {
        Fixture {
            name: "Bonfire of the Damned".to_string(),
            children,
            tag,
        }
    }

    #[test]
    fn a_unit_variant_stays_distinct_from_a_string() {
        let shape = of(&fixture(vec![Leaf::Alpha], None));
        let name = shape.field("name").expect("the fixture has a name field");
        let children = shape.field("children").expect("children field");
        assert_eq!(*name, Shape::Scalar("str"), "string values are discarded");
        assert_eq!(
            children.elements()[0].label(),
            "Leaf::Alpha",
            "lexical identity survives as a unit variant, not a string"
        );
    }

    #[test]
    fn an_absent_option_is_distinguishable_from_a_present_one() {
        let absent = of(&fixture(vec![], None));
        let present = of(&fixture(vec![], Some(Leaf::Beta)));
        assert_eq!(absent.field("tag"), Some(&Shape::Absent));
        assert_eq!(
            present.field("tag").map(Shape::label),
            Some("Leaf::Beta".to_string())
        );
    }

    #[test]
    fn walk_reaches_every_node() {
        let shape = of(&fixture(vec![Leaf::Alpha, Leaf::Beta], Some(Leaf::Alpha)));
        let mut labels = Vec::new();
        shape.walk(&mut |node| labels.push(node.label()));
        assert_eq!(
            labels
                .iter()
                .filter(|label| *label == "Leaf::Alpha")
                .count(),
            2
        );
        assert!(labels.contains(&"Leaf::Beta".to_string()));
        assert!(labels.contains(&"Fixture".to_string()));
    }

    #[test]
    fn fingerprint_renders_nested_structure_canonically() {
        let shape = Shape::Seq(vec![Shape::Newtype {
            name: "Wrapper",
            variant: None,
            inner: Box::new(Shape::Absent),
        }]);
        assert_eq!(shape.fingerprint(), "[Wrapper(None)]");
    }

    #[test]
    fn fingerprint_distinguishes_two_groupings_of_the_same_leaves() {
        let flat = Shape::Seq(vec![Shape::Absent, Shape::Absent]);
        let nested = Shape::Seq(vec![Shape::Seq(vec![Shape::Absent]), Shape::Absent]);
        assert_ne!(
            flat.fingerprint(),
            nested.fingerprint(),
            "grouping must survive fingerprinting or minimal pairs cannot detect collapse"
        );
    }
}
