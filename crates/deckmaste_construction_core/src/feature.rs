use proc_macro2::Span;
use syn::Ident;

use crate::model;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Feature {
    Agreement,
    Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeatureValue {
    Bare,
    ThirdPersonSingular,
    Singular,
    Plural,
}

#[derive(Debug)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FeaturePlace {
    Construction(Feature),
    Role { field: Ident, feature: Feature },
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
}

pub(crate) fn lower_constant(
    feature: model::Feature,
    path: &syn::Path,
) -> syn::Result<FeatureValue> {
    let Some(name) = path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return Err(syn::Error::new_spanned(
            path,
            "feature value path cannot be empty",
        ));
    };
    let value = match (feature, name.as_str()) {
        (model::Feature::Agreement, "Bare") => FeatureValue::Bare,
        (model::Feature::Agreement, "ThirdPersonSingular") => FeatureValue::ThirdPersonSingular,
        (model::Feature::Number, "Singular") => FeatureValue::Singular,
        (model::Feature::Number, "Plural") => FeatureValue::Plural,
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
    };
    Ok(value)
}

impl From<model::Feature> for Feature {
    fn from(value: model::Feature) -> Self {
        match value {
            model::Feature::Agreement => Self::Agreement,
            model::Feature::Number => Self::Number,
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
