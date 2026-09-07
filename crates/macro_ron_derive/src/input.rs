//! Parses a `DeriveInput` into the grammar-enum IR the generators consume.

use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Expr;
use syn::Fields;
use syn::Ident;
use syn::Result;
use syn::Type;
use syn::spanned::Spanned as _;

/// Variant-level `#[macro_ron(...)]` markers (mutually exclusive).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    /// `Expanded(Expansion<Self>)` — invocation write-back + `expanded()`.
    Expanded,
    /// Untagged embed of another type. A newtype embed (`Ref(Reference)`)
    /// is name-erased (always written bare); a tuple embed (`By(Reference,
    /// PlayerAction)`) keeps its tag and writes bare only when every
    /// defaulted field equals its default; a STRUCT embed of exactly one
    /// field (`Simple { symbol: SimpleManaSymbol }`) keeps its tag on read
    /// and always writes bare.
    ///
    /// Defaulted fields of a tuple embed must implement `PartialEq` (the
    /// generated bare-write guard compares them against their defaults); a
    /// missing impl surfaces as a compile error inside the generated
    /// `Serialize`.
    Embed,
    /// Compartment: the payload enum's accepted names lift into this enum.
    Flatten,
    /// Bare-literal wrapper (`Count::Literal`) — Kind metadata only.
    Literal,
    /// Newtype over a NAMED STRUCT, spelled field-spliced in RON
    /// (`Activated(cost: …, effect: …)`, via `unwrap_variant_newtypes`).
    /// Read and write are the ordinary newtype ones; the marker only tells
    /// the signature emitter to report the payload's own fields
    /// (`<Payload as MacroFields>::FIELDS`) instead of one opaque positional
    /// slot, so a scaffolded identity macro mirrors the spelling canon uses.
    Spliced,
}

pub struct Field {
    pub ident: Option<Ident>,
    pub ty: Type,
    /// `#[macro_ron(default = "expr")]` — embed-variant fill value.
    pub default: Option<Expr>,
    /// Forwarded `#[serde(...)]` attributes (struct variants only).
    pub serde_attrs: Vec<Attribute>,
}

pub enum Shape {
    Unit,
    Newtype(Box<Field>),
    Tuple(Vec<Field>),
    Struct(Vec<Field>),
}

pub struct Variant {
    pub ident: Ident,
    pub marker: Option<Marker>,
    pub shape: Shape,
    /// `#[macro_ron(flatten, exclude(A, B))]` — payload variant names the
    /// compartment must NOT lift into the parent, so a name the parent
    /// forbids (e.g. `Destination` excludes `Library`/`Stack`, mirroring the
    /// Idris `DestinationOk` gate) is rejected on read instead of read as a
    /// `Zone`. Empty unless the variant is a `flatten` carrying `exclude(...)`.
    pub flatten_exclude: Vec<Ident>,
}

pub struct Input {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

impl Input {
    pub fn expanded(&self) -> Option<&Variant> {
        self.variants
            .iter()
            .find(|v| v.marker == Some(Marker::Expanded))
    }

    pub fn embed(&self) -> Option<&Variant> {
        self.variants
            .iter()
            .find(|v| v.marker == Some(Marker::Embed))
    }

    pub fn literal(&self) -> Option<&Variant> {
        self.variants
            .iter()
            .find(|v| v.marker == Some(Marker::Literal))
    }
}

impl Variant {
    /// The embedded payload field of an `embed` variant: the single field
    /// without a `default` (a struct embed's one field). Only meaningful on
    /// variants `parse()` validated as embeds.
    pub fn embed_payload(&self) -> &Field {
        match &self.shape {
            Shape::Newtype(f) => f,
            Shape::Tuple(fs) => fs
                .iter()
                .find(|f| f.default.is_none())
                .expect("embed_payload: embed tuple needs one non-defaulted field; was this Input produced by parse()?"),
            Shape::Struct(fs) => fs
                .first()
                .expect("embed_payload: embed struct has exactly one field; was this Input produced by parse()?"),
            Shape::Unit => {
                unreachable!("embed_payload: embed is not a unit variant (validated in parse())")
            }
        }
    }
}

/// Generated impls carry no generic parameters — reject generic types up
/// front (both derives) so the failure is one clear error instead of a
/// missing-generics cascade inside the generated impls. Grammar types are
/// concrete; nothing in production needs this.
///
/// The error span points at the generic parameters themselves (or the where
/// clause when there are no params), not at the type ident, so the IDE
/// underlines the offending declaration.
pub fn reject_generics(input: &DeriveInput) -> Result<()> {
    if input.generics.params.is_empty() && input.generics.where_clause.is_none() {
        Ok(())
    } else {
        let span = if input.generics.params.is_empty() {
            input
                .generics
                .where_clause
                .as_ref()
                .expect("checked above that where_clause is Some")
                .span()
        } else {
            input.generics.params.span()
        };
        Err(Error::new(span, "generic types are not supported"))
    }
}

pub fn parse(input: &DeriveInput) -> Result<Input> {
    reject_generics(input)?;
    let Data::Enum(data) = &input.data else {
        return Err(Error::new(
            input.ident.span(),
            "SupportsMacros applies to enums (use #[derive(Expand)] for structs)",
        ));
    };
    // Serde attrs on the enum itself are not forwarded.
    if let Some(attr) = input.attrs.iter().find(|a| a.path().is_ident("serde")) {
        return Err(Error::new_spanned(
            attr,
            "#[serde(...)] on the enum is not supported by SupportsMacros \
             (serde attrs are only forwarded from struct-variant fields)",
        ));
    }
    let mut variants = Vec::new();
    for v in &data.variants {
        let (marker, flatten_exclude) = variant_marker(&v.attrs)?;
        let shape = match &v.fields {
            Fields::Unit => Shape::Unit,
            Fields::Unnamed(fs) if fs.unnamed.len() == 1 => {
                Shape::Newtype(Box::new(field(&fs.unnamed[0])?))
            }
            Fields::Unnamed(fs) => {
                Shape::Tuple(fs.unnamed.iter().map(field).collect::<Result<_>>()?)
            }
            Fields::Named(fs) => Shape::Struct(fs.named.iter().map(field).collect::<Result<_>>()?),
        };
        validate(&v.ident, marker, &shape)?;
        if !flatten_exclude.is_empty() && marker != Some(Marker::Flatten) {
            return Err(Error::new(
                v.ident.span(),
                "exclude(...) is only valid on a `flatten` variant",
            ));
        }
        variants.push(Variant {
            ident: v.ident.clone(),
            marker,
            shape,
            flatten_exclude,
        });
    }
    // One embed per enum: two untagged fall-throughs at one position would
    // race for the same identifier, and `Input::embed` would silently pick
    // the first. The reader's unambiguity is this check.
    if let Some(second) = variants
        .iter()
        .filter(|v| v.marker == Some(Marker::Embed))
        .nth(1)
    {
        return Err(Error::new(
            second.ident.span(),
            "an enum carries at most one `embed` variant              (two untagged fall-throughs would race for the same identifier)",
        ));
    }
    Ok(Input {
        ident: input.ident.clone(),
        variants,
    })
}

fn variant_marker(attrs: &[Attribute]) -> Result<(Option<Marker>, Vec<Ident>)> {
    // Serde attrs on a variant are not forwarded.
    if let Some(attr) = attrs.iter().find(|a| a.path().is_ident("serde")) {
        return Err(Error::new_spanned(
            attr,
            "#[serde(...)] on a variant is not forwarded by SupportsMacros",
        ));
    }
    let mut marker = None;
    let mut exclude = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("macro_ron")) {
        attr.parse_nested_meta(|meta| {
            // `exclude(A, B)` is a modifier, not a marker: collect the named
            // payload variants this `flatten` must not lift.
            if meta.path.is_ident("exclude") {
                return meta.parse_nested_meta(|inner| {
                    let ident = inner.path.require_ident()?.clone();
                    exclude.push(ident);
                    Ok(())
                });
            }
            let m = if meta.path.is_ident("expanded") {
                Marker::Expanded
            } else if meta.path.is_ident("embed") {
                Marker::Embed
            } else if meta.path.is_ident("flatten") {
                Marker::Flatten
            } else if meta.path.is_ident("literal") {
                Marker::Literal
            } else if meta.path.is_ident("spliced") {
                Marker::Spliced
            } else {
                return Err(meta.error(
                    "expected one of: expanded, embed, flatten, literal, spliced, exclude(...)",
                ));
            };
            if marker.replace(m).is_some() {
                return Err(meta.error("at most one macro_ron marker per variant"));
            }
            Ok(())
        })?;
    }
    Ok((marker, exclude))
}

fn field(f: &syn::Field) -> Result<Field> {
    let mut default = None;
    for attr in f.attrs.iter().filter(|a| a.path().is_ident("macro_ron")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                let parsed = lit.parse::<Expr>()?;
                if default.replace(parsed).is_some() {
                    return Err(meta.error("duplicate `default` on a single field"));
                }
                Ok(())
            } else {
                Err(meta.error("expected `default = \"expr\"` on a field"))
            }
        })?;
    }
    let serde_attrs = f
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .cloned()
        .collect();
    Ok(Field {
        ident: f.ident.clone(),
        ty: f.ty.clone(),
        default,
        serde_attrs,
    })
}

fn validate(ident: &Ident, marker: Option<Marker>, shape: &Shape) -> Result<()> {
    match marker {
        Some(Marker::Expanded | Marker::Literal | Marker::Flatten | Marker::Spliced)
            if !matches!(shape, Shape::Newtype(_)) =>
        {
            return Err(Error::new(
                ident.span(),
                "expanded/literal/flatten/spliced markers require a newtype variant",
            ));
        }
        Some(Marker::Expanded | Marker::Literal | Marker::Flatten | Marker::Spliced) | None => {}
        Some(Marker::Embed) => match shape {
            Shape::Newtype(_) => {}
            Shape::Tuple(fs) => {
                let bare = fs.iter().filter(|f| f.default.is_none()).count();
                if bare != 1 {
                    return Err(Error::new(
                        ident.span(),
                        "a tuple embed needs exactly one non-defaulted field (the payload)",
                    ));
                }
                // A tuple embed with no defaulted fields would generate a
                // bare-write guard of `if =>` (empty &&-chain), which is
                // syntactically invalid. A single-field embed should be a
                // newtype variant instead.
                let defaulted = fs.iter().filter(|f| f.default.is_some()).count();
                if defaulted == 0 {
                    return Err(Error::new(
                        ident.span(),
                        "a tuple embed needs at least one defaulted field \
                         (a single-field embed should be a newtype variant)",
                    ));
                }
            }
            // A struct embed carries a binder name (`Simple { symbol: … }`,
            // mirroring the Lean constructor's own field), so the injection
            // rule reaches the mirror's struct variants and not only v1's
            // newtypes. Exactly one field: two fields carry information the
            // payload cannot, so eliding the constructor would lose it.
            Shape::Struct(fs) => {
                if fs.len() != 1 {
                    return Err(Error::new(
                        ident.span(),
                        "a struct embed needs exactly one field (the payload)",
                    ));
                }
                if fs[0].default.is_some() {
                    return Err(Error::new(
                        ident.span(),
                        "#[macro_ron(default = ...)] is not used on struct variants \
                         (forward #[serde(default = ...)] instead)",
                    ));
                }
            }
            Shape::Unit => {
                return Err(Error::new(
                    ident.span(),
                    "embed marker requires a newtype, tuple, or one-field struct variant",
                ));
            }
        },
    }
    // `#[macro_ron(default = ...)]` on a struct-variant field is meaningless
    // (struct variants forward `#[serde(default = ...)]` instead); reject it.
    if !matches!(marker, Some(Marker::Embed))
        && matches!(shape, Shape::Struct(fs) if fs.iter().any(|f| f.default.is_some()))
    {
        return Err(Error::new(
            ident.span(),
            "#[macro_ron(default = ...)] is not used on struct variants \
             (forward #[serde(default = ...)] instead)",
        ));
    }
    // A non-embed *tuple* variant may carry trailing defaults: the short form
    // omits them on write and fills them on read (`PairPlusDefault`). The
    // defaulted fields must form a contiguous suffix — a required field after a
    // defaulted one has no positional reading.
    if !matches!(marker, Some(Marker::Embed))
        && let Shape::Tuple(fs) = shape
    {
        let first_default = fs.iter().position(|f| f.default.is_some());
        if let Some(first) = first_default
            && fs[first..].iter().any(|f| f.default.is_none())
        {
            return Err(Error::new(
                ident.span(),
                "defaulted tuple fields must be a trailing run \
                 (no required field after a defaulted one)",
            ));
        }
    }
    // Serde attrs on fields of non-struct shapes are silently ignored — reject
    // them.
    let non_struct_serde_fields: Vec<&Attribute> = match shape {
        Shape::Unit | Shape::Struct(_) => vec![],
        Shape::Newtype(f) => f.serde_attrs.first().into_iter().collect(),
        Shape::Tuple(fields) => fields
            .iter()
            .filter_map(|f| f.serde_attrs.first())
            .collect(),
    };
    if let Some(attr) = non_struct_serde_fields.first() {
        return Err(Error::new_spanned(
            *attr,
            "#[serde(...)] on this field is not forwarded by SupportsMacros \
             (only struct-variant fields forward serde attrs)",
        ));
    }
    Ok(())
}
