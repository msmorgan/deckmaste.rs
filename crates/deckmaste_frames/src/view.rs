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
}

/// Build a [`View`] from anything that derives `Serialize`.
pub fn of<T: Serialize + ?Sized>(value: &T) -> View {
    // The builder is infallible; the error type exists only to satisfy serde.
    value.serialize(ViewBuilder).unwrap_or(View::Absent)
}

/// The walk cannot fail; `serde` still requires an error type.
#[derive(Debug)]
struct Unreachable;

impl fmt::Display for Unreachable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("view construction cannot fail")
    }
}

impl std::error::Error for Unreachable {}

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
    use super::*;

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
}
