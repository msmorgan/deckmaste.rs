use std::hash::Hash;
use std::hash::Hasher;

use proc_macro2::Span;
use syn::Ident;

use crate::identifier;
use crate::model;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Feature {
    Agreement,
    Number,
    Onset,
    PossessiveEnding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeatureValue {
    Bare,
    ThirdPersonSingular,
    Singular,
    Plural,
    Consonant,
    Vowel,
    EndsInS,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeatureResolution {
    Known(FeatureValue),
    Runtime,
    External,
}

#[derive(Debug, Clone)]
#[allow(
    dead_code,
    reason = "sealed feature IR is consumed by Task 4 code generation"
)]
pub(crate) struct Spanned<T> {
    value: T,
    span: Span,
}

#[allow(
    dead_code,
    reason = "sealed feature IR is consumed by Task 4 code generation"
)]
impl<T> Spanned<T> {
    pub(crate) fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub(crate) fn value(&self) -> &T {
        &self.value
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

impl Feature {
    pub(crate) fn domain(self) -> &'static [FeatureValue] {
        match self {
            Self::Agreement => &[FeatureValue::Bare, FeatureValue::ThirdPersonSingular],
            Self::Number => &[FeatureValue::Singular, FeatureValue::Plural],
            Self::Onset => &[FeatureValue::Consonant, FeatureValue::Vowel],
            Self::PossessiveEnding => &[FeatureValue::EndsInS, FeatureValue::Other],
        }
    }

    pub(crate) fn member(self, ident: &syn::Ident) -> syn::Result<FeatureValue> {
        let name = identifier::key(ident);
        self.domain()
            .iter()
            .copied()
            .find(|value| value.key() == name)
            .ok_or_else(|| {
                syn::Error::new(
                    ident.span(),
                    format!(
                        "unknown predicate member `{name}` for feature `{}`",
                        self.key()
                    ),
                )
            })
    }

    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Agreement => "agreement",
            Self::Number => "number",
            Self::Onset => "onset",
            Self::PossessiveEnding => "possessive_ending",
        }
    }
}

impl FeatureValue {
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Bare => "Bare",
            Self::ThirdPersonSingular => "ThirdPersonSingular",
            Self::Singular => "Singular",
            Self::Plural => "Plural",
            Self::Consonant => "Consonant",
            Self::Vowel => "Vowel",
            Self::EndsInS => "EndsInS",
            Self::Other => "Other",
        }
    }
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed feature IR is consumed by Task 4 code generation"
)]
pub(crate) enum FeatureExpr {
    Constant(Spanned<FeatureValue>),
    FromRole {
        role: Ident,
        feature: Feature,
    },
    MatchVocab {
        role: Ident,
        arms: Vec<(Spanned<Ident>, FeatureValue)>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum FeaturePlace {
    Construction(Feature),
    Role { field: Ident, feature: Feature },
}

impl PartialEq for FeaturePlace {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Construction(left), Self::Construction(right)) => left == right,
            (
                Self::Role {
                    field: left_field,
                    feature: left_feature,
                },
                Self::Role {
                    field: right_field,
                    feature: right_feature,
                },
            ) => left_feature == right_feature && identifier::same(left_field, right_field),
            (Self::Construction(_), Self::Role { .. })
            | (Self::Role { .. }, Self::Construction(_)) => false,
        }
    }
}

impl Eq for FeaturePlace {}

impl Hash for FeaturePlace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Construction(feature) => {
                0_u8.hash(state);
                feature.hash(state);
            }
            Self::Role { field, feature } => {
                1_u8.hash(state);
                identifier::key(field).hash(state);
                feature.hash(state);
            }
        }
    }
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed feature IR is consumed by Task 4 code generation"
)]
pub(crate) struct FeatureEquation {
    target: FeaturePlace,
    value: FeatureExpr,
}

#[allow(
    dead_code,
    reason = "sealed feature IR is consumed by Task 4 code generation"
)]
impl FeatureEquation {
    pub(crate) fn new(target: FeaturePlace, value: FeatureExpr) -> Self {
        Self { target, value }
    }

    pub(crate) fn target(&self) -> &FeaturePlace {
        &self.target
    }

    pub(crate) fn value(&self) -> &FeatureExpr {
        &self.value
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> String {
        format!("{} = {}", self.target.snapshot(), self.value.snapshot())
    }
}

#[cfg(test)]
impl FeaturePlace {
    pub(crate) fn snapshot(&self) -> String {
        match self {
            Self::Construction(feature) => feature.snapshot().to_owned(),
            Self::Role { field, feature } => format!("{field}.{}", feature.snapshot()),
        }
    }
}

#[cfg(test)]
impl FeatureExpr {
    fn snapshot(&self) -> String {
        match self {
            Self::Constant(value) => value.value().snapshot().to_owned(),
            Self::FromRole { role, feature } => format!("{role}.{}", feature.snapshot()),
            Self::MatchVocab { role, arms } => format!(
                "match {role} {{ {} }}",
                arms.iter()
                    .map(|(variant, value)| format!("{} => {}", variant.value(), value.snapshot()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[cfg(test)]
impl Feature {
    fn snapshot(self) -> &'static str {
        match self {
            Self::Agreement => "agreement",
            Self::Number => "number",
            Self::Onset => "onset",
            Self::PossessiveEnding => "possessive_ending",
        }
    }
}

#[cfg(test)]
impl FeatureValue {
    pub(crate) fn snapshot(self) -> &'static str {
        match self {
            Self::Bare => "Bare",
            Self::ThirdPersonSingular => "ThirdPersonSingular",
            Self::Singular => "Singular",
            Self::Plural => "Plural",
            Self::Consonant => "Consonant",
            Self::Vowel => "Vowel",
            Self::EndsInS => "EndsInS",
            Self::Other => "Other",
        }
    }
}

#[cfg(test)]
impl FeatureResolution {
    pub(crate) fn snapshot(self) -> String {
        match self {
            Self::Known(value) => format!("Known({})", value.snapshot()),
            Self::Runtime => "Runtime".to_owned(),
            Self::External => "External".to_owned(),
        }
    }
}

pub(crate) fn lower_constant(
    feature: model::Feature,
    path: &syn::Path,
) -> syn::Result<FeatureValue> {
    let Some(_) = path.segments.last() else {
        return Err(syn::Error::new_spanned(
            path,
            "feature value path cannot be empty",
        ));
    };
    let name = identifier::path_key(path);
    let value = match (feature, name.as_str()) {
        (model::Feature::Agreement, "Bare") => FeatureValue::Bare,
        (model::Feature::Agreement, "ThirdPersonSingular") => FeatureValue::ThirdPersonSingular,
        (model::Feature::Number, "Singular") => FeatureValue::Singular,
        (model::Feature::Number, "Plural") => FeatureValue::Plural,
        (model::Feature::Onset, "Consonant") => FeatureValue::Consonant,
        (model::Feature::Onset, "Vowel") => FeatureValue::Vowel,
        (model::Feature::PossessiveEnding, "EndsInS") => FeatureValue::EndsInS,
        (model::Feature::PossessiveEnding, "Other") => FeatureValue::Other,
        (model::Feature::Agreement, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not an agreement value"),
            ));
        }
        (model::Feature::Number, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a number value"),
            ));
        }
        (model::Feature::Onset, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not an onset value"),
            ));
        }
        (model::Feature::PossessiveEnding, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a possessive-ending value"),
            ));
        }
    };
    Ok(value)
}

impl From<model::Feature> for Feature {
    fn from(value: model::Feature) -> Self {
        match value {
            model::Feature::Agreement => Self::Agreement,
            model::Feature::Number => Self::Number,
            model::Feature::Onset => Self::Onset,
            model::Feature::PossessiveEnding => Self::PossessiveEnding,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FeatureValue;
    use super::lower_constant;
    use crate::model::Feature;

    #[test]
    fn feature_domains_are_closed_and_directional() {
        assert_eq!(
            lower_constant(Feature::Agreement, &syn::parse_quote!(Anything::Bare)).unwrap(),
            FeatureValue::Bare
        );
        assert_eq!(
            lower_constant(
                Feature::Agreement,
                &syn::parse_quote!(Anything::ThirdPersonSingular),
            )
            .unwrap(),
            FeatureValue::ThirdPersonSingular
        );
        assert_eq!(
            lower_constant(Feature::Number, &syn::parse_quote!(Anything::Singular)).unwrap(),
            FeatureValue::Singular
        );
        assert!(lower_constant(Feature::Number, &syn::parse_quote!(Anything::Bare)).is_err());
        assert!(lower_constant(Feature::Agreement, &syn::parse_quote!(Anything::Plural)).is_err());
    }
}
