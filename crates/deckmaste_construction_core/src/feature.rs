use std::hash::Hash;
use std::hash::Hasher;

use proc_macro2::Span;
use syn::Ident;

use crate::identifier;
use crate::model;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Feature {
    Agreement,
    BareLocativeLicense,
    Cardinality,
    Compoundability,
    Countability,
    ModifierLicense,
    DeterminerNumber,
    FusedHeadLicense,
    NominalForm,
    NominalLicense,
    NounComplement,
    Number,
    Onset,
    Participle,
    PossessiveEnding,
    PrepositionClass,
    Properness,
    Relationality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FeatureValue {
    Bare,
    ThirdPersonSingular,
    QualifiedOnly,
    BareAllowed,
    Singular,
    Plural,
    Consonant,
    Vowel,
    EndsInS,
    Other,
    Participle,
    Zero,
    One,
    TwoPlus,
    Compoundable,
    NonCompoundable,
    Count,
    Mass,
    Unrestricted,
    LocalDeterminer,
    SingularOnly,
    PluralOnly,
    Both,
    NominalOnly,
    FusedHead,
    BareSingularNoun,
    ModifiedSingularNoun,
    SingularCoordination,
    BarePluralNoun,
    ModifiedPluralNoun,
    PluralCoordination,
    MassNoun,
    AnyNominal,
    CountNominal,
    LicensedBareSingularNoun,
    LicensedMassOrPluralCount,
    Common,
    Proper,
    NonRelational,
    Relational,
    AdjunctCapable,
    PostmodifierOnly,
    PostmodifierBareLocative,
    SelectedOnly,
    NoComplement,
    OfComplement,
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
            Self::BareLocativeLicense => &[FeatureValue::QualifiedOnly, FeatureValue::BareAllowed],
            Self::Cardinality => &[FeatureValue::Zero, FeatureValue::One, FeatureValue::TwoPlus],
            Self::Compoundability => &[FeatureValue::Compoundable, FeatureValue::NonCompoundable],
            Self::Countability => &[FeatureValue::Count, FeatureValue::Mass],
            Self::ModifierLicense => &[FeatureValue::Unrestricted, FeatureValue::LocalDeterminer],
            Self::DeterminerNumber => &[
                FeatureValue::SingularOnly,
                FeatureValue::PluralOnly,
                FeatureValue::Both,
            ],
            Self::FusedHeadLicense => &[FeatureValue::NominalOnly, FeatureValue::FusedHead],
            Self::NominalForm => &[
                FeatureValue::BareSingularNoun,
                FeatureValue::ModifiedSingularNoun,
                FeatureValue::SingularCoordination,
                FeatureValue::BarePluralNoun,
                FeatureValue::ModifiedPluralNoun,
                FeatureValue::PluralCoordination,
                FeatureValue::MassNoun,
            ],
            Self::NominalLicense => &[
                FeatureValue::AnyNominal,
                FeatureValue::CountNominal,
                FeatureValue::LicensedBareSingularNoun,
                FeatureValue::LicensedMassOrPluralCount,
            ],
            Self::Number => &[FeatureValue::Singular, FeatureValue::Plural],
            Self::Onset => &[FeatureValue::Consonant, FeatureValue::Vowel],
            Self::Participle => &[FeatureValue::Participle],
            Self::PossessiveEnding => &[FeatureValue::EndsInS, FeatureValue::Other],
            Self::Properness => &[FeatureValue::Common, FeatureValue::Proper],
            Self::Relationality => &[FeatureValue::NonRelational, FeatureValue::Relational],
            Self::NounComplement => &[FeatureValue::NoComplement, FeatureValue::OfComplement],
            Self::PrepositionClass => &[
                FeatureValue::AdjunctCapable,
                FeatureValue::PostmodifierOnly,
                FeatureValue::PostmodifierBareLocative,
                FeatureValue::SelectedOnly,
            ],
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
            Self::BareLocativeLicense => "bare_locative_license",
            Self::Cardinality => "cardinality",
            Self::Compoundability => "compoundability",
            Self::Countability => "countability",
            Self::ModifierLicense => "modifier_license",
            Self::DeterminerNumber => "determiner_number",
            Self::FusedHeadLicense => "fused_head_license",
            Self::NominalForm => "nominal_form",
            Self::NominalLicense => "nominal_license",
            Self::Number => "number",
            Self::Onset => "onset",
            Self::Participle => "participle",
            Self::PossessiveEnding => "possessive_ending",
            Self::Properness => "properness",
            Self::Relationality => "relationality",
            Self::NounComplement => "noun_complement",
            Self::PrepositionClass => "preposition_class",
        }
    }
}

impl FeatureValue {
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Bare => "Bare",
            Self::ThirdPersonSingular => "ThirdPersonSingular",
            Self::QualifiedOnly => "QualifiedOnly",
            Self::BareAllowed => "BareAllowed",
            Self::Singular => "Singular",
            Self::Plural => "Plural",
            Self::Consonant => "Consonant",
            Self::Vowel => "Vowel",
            Self::EndsInS => "EndsInS",
            Self::Other => "Other",
            Self::Participle => "Participle",
            Self::Zero => "Zero",
            Self::One => "One",
            Self::TwoPlus => "TwoPlus",
            Self::Compoundable => "Compoundable",
            Self::NonCompoundable => "NonCompoundable",
            Self::Count => "Count",
            Self::Mass => "Mass",
            Self::Unrestricted => "Unrestricted",
            Self::LocalDeterminer => "LocalDeterminer",
            Self::SingularOnly => "SingularOnly",
            Self::PluralOnly => "PluralOnly",
            Self::Both => "Both",
            Self::NominalOnly => "NominalOnly",
            Self::FusedHead => "FusedHead",
            Self::BareSingularNoun | Self::LicensedBareSingularNoun => "BareSingularNoun",
            Self::ModifiedSingularNoun => "ModifiedSingularNoun",
            Self::SingularCoordination => "SingularCoordination",
            Self::BarePluralNoun => "BarePluralNoun",
            Self::ModifiedPluralNoun => "ModifiedPluralNoun",
            Self::PluralCoordination => "PluralCoordination",
            Self::MassNoun => "MassNoun",
            Self::AnyNominal => "AnyNominal",
            Self::CountNominal => "CountNominal",
            Self::LicensedMassOrPluralCount => "MassOrPluralCount",
            Self::Common => "Common",
            Self::Proper => "Proper",
            Self::NonRelational => "NonRelational",
            Self::Relational => "Relational",
            Self::AdjunctCapable => "AdjunctCapable",
            Self::PostmodifierOnly => "PostmodifierOnly",
            Self::PostmodifierBareLocative => "PostmodifierBareLocative",
            Self::SelectedOnly => "SelectedOnly",
            Self::NoComplement => "NoComplement",
            Self::OfComplement => "OfComplement",
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
            Self::BareLocativeLicense => "bare_locative_license",
            Self::Cardinality => "cardinality",
            Self::Compoundability => "compoundability",
            Self::Countability => "countability",
            Self::ModifierLicense => "modifier_license",
            Self::DeterminerNumber => "determiner_number",
            Self::FusedHeadLicense => "fused_head_license",
            Self::NominalForm => "nominal_form",
            Self::NominalLicense => "nominal_license",
            Self::Number => "number",
            Self::Onset => "onset",
            Self::Participle => "participle",
            Self::PossessiveEnding => "possessive_ending",
            Self::Properness => "properness",
            Self::Relationality => "relationality",
            Self::NounComplement => "noun_complement",
            Self::PrepositionClass => "preposition_class",
        }
    }
}

#[cfg(test)]
impl FeatureValue {
    pub(crate) fn snapshot(self) -> &'static str {
        match self {
            Self::Bare => "Bare",
            Self::ThirdPersonSingular => "ThirdPersonSingular",
            Self::QualifiedOnly => "QualifiedOnly",
            Self::BareAllowed => "BareAllowed",
            Self::Singular => "Singular",
            Self::Plural => "Plural",
            Self::Consonant => "Consonant",
            Self::Vowel => "Vowel",
            Self::EndsInS => "EndsInS",
            Self::Other => "Other",
            Self::Participle => "Participle",
            Self::Zero => "Zero",
            Self::One => "One",
            Self::TwoPlus => "TwoPlus",
            Self::Compoundable => "Compoundable",
            Self::NonCompoundable => "NonCompoundable",
            Self::Count => "Count",
            Self::Mass => "Mass",
            Self::Unrestricted => "Unrestricted",
            Self::LocalDeterminer => "LocalDeterminer",
            Self::SingularOnly => "SingularOnly",
            Self::PluralOnly => "PluralOnly",
            Self::Both => "Both",
            Self::NominalOnly => "NominalOnly",
            Self::FusedHead => "FusedHead",
            Self::BareSingularNoun | Self::LicensedBareSingularNoun => "BareSingularNoun",
            Self::ModifiedSingularNoun => "ModifiedSingularNoun",
            Self::SingularCoordination => "SingularCoordination",
            Self::BarePluralNoun => "BarePluralNoun",
            Self::ModifiedPluralNoun => "ModifiedPluralNoun",
            Self::PluralCoordination => "PluralCoordination",
            Self::MassNoun => "MassNoun",
            Self::AnyNominal => "AnyNominal",
            Self::CountNominal => "CountNominal",
            Self::LicensedMassOrPluralCount => "MassOrPluralCount",
            Self::Common => "Common",
            Self::Proper => "Proper",
            Self::NonRelational => "NonRelational",
            Self::Relational => "Relational",
            Self::AdjunctCapable => "AdjunctCapable",
            Self::PostmodifierOnly => "PostmodifierOnly",
            Self::PostmodifierBareLocative => "PostmodifierBareLocative",
            Self::SelectedOnly => "SelectedOnly",
            Self::NoComplement => "NoComplement",
            Self::OfComplement => "OfComplement",
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

#[allow(
    clippy::too_many_lines,
    reason = "the feature-value lowering table is deliberately exhaustive and literal"
)]
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
        (model::Feature::BareLocativeLicense, "QualifiedOnly") => FeatureValue::QualifiedOnly,
        (model::Feature::BareLocativeLicense, "BareAllowed") => FeatureValue::BareAllowed,
        (model::Feature::Number, "Singular") => FeatureValue::Singular,
        (model::Feature::Number, "Plural") => FeatureValue::Plural,
        (model::Feature::Onset, "Consonant") => FeatureValue::Consonant,
        (model::Feature::Onset, "Vowel") => FeatureValue::Vowel,
        (model::Feature::Participle, "Participle") => FeatureValue::Participle,
        (model::Feature::PossessiveEnding, "EndsInS") => FeatureValue::EndsInS,
        (model::Feature::PossessiveEnding, "Other") => FeatureValue::Other,
        (model::Feature::Cardinality, "Zero") => FeatureValue::Zero,
        (model::Feature::Cardinality, "One") => FeatureValue::One,
        (model::Feature::Cardinality, "TwoPlus") => FeatureValue::TwoPlus,
        (model::Feature::Compoundability, "Compoundable") => FeatureValue::Compoundable,
        (model::Feature::Compoundability, "NonCompoundable") => FeatureValue::NonCompoundable,
        (model::Feature::Countability, "Count") => FeatureValue::Count,
        (model::Feature::Countability, "Mass") => FeatureValue::Mass,
        (model::Feature::ModifierLicense, "Unrestricted") => FeatureValue::Unrestricted,
        (model::Feature::ModifierLicense, "LocalDeterminer") => FeatureValue::LocalDeterminer,
        (model::Feature::DeterminerNumber, "SingularOnly") => FeatureValue::SingularOnly,
        (model::Feature::DeterminerNumber, "PluralOnly") => FeatureValue::PluralOnly,
        (model::Feature::DeterminerNumber, "Both") => FeatureValue::Both,
        (model::Feature::FusedHeadLicense, "NominalOnly") => FeatureValue::NominalOnly,
        (model::Feature::FusedHeadLicense, "FusedHead") => FeatureValue::FusedHead,
        (model::Feature::NominalForm, "BareSingularNoun") => FeatureValue::BareSingularNoun,
        (model::Feature::NominalForm, "ModifiedSingularNoun") => FeatureValue::ModifiedSingularNoun,
        (model::Feature::NominalForm, "SingularCoordination") => FeatureValue::SingularCoordination,
        (model::Feature::NominalForm, "BarePluralNoun") => FeatureValue::BarePluralNoun,
        (model::Feature::NominalForm, "ModifiedPluralNoun") => FeatureValue::ModifiedPluralNoun,
        (model::Feature::NominalForm, "PluralCoordination") => FeatureValue::PluralCoordination,
        (model::Feature::NominalForm, "MassNoun") => FeatureValue::MassNoun,
        (model::Feature::NominalLicense, "AnyNominal") => FeatureValue::AnyNominal,
        (model::Feature::NominalLicense, "CountNominal") => FeatureValue::CountNominal,
        (model::Feature::NominalLicense, "BareSingularNoun") => {
            FeatureValue::LicensedBareSingularNoun
        }
        (model::Feature::NominalLicense, "MassOrPluralCount") => {
            FeatureValue::LicensedMassOrPluralCount
        }
        (model::Feature::Properness, "Common") => FeatureValue::Common,
        (model::Feature::Properness, "Proper") => FeatureValue::Proper,
        (model::Feature::Relationality, "NonRelational") => FeatureValue::NonRelational,
        (model::Feature::Relationality, "Relational") => FeatureValue::Relational,
        (model::Feature::NounComplement, "NoComplement") => FeatureValue::NoComplement,
        (model::Feature::NounComplement, "OfComplement") => FeatureValue::OfComplement,
        (model::Feature::PrepositionClass, "AdjunctCapable") => FeatureValue::AdjunctCapable,
        (model::Feature::PrepositionClass, "PostmodifierOnly") => FeatureValue::PostmodifierOnly,
        (model::Feature::PrepositionClass, "PostmodifierBareLocative") => {
            FeatureValue::PostmodifierBareLocative
        }
        (model::Feature::PrepositionClass, "SelectedOnly") => FeatureValue::SelectedOnly,
        (model::Feature::Agreement, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not an agreement value"),
            ));
        }
        (model::Feature::BareLocativeLicense, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a bare-locative-license value"),
            ));
        }
        (model::Feature::Cardinality, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a cardinality value"),
            ));
        }
        (model::Feature::Compoundability, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a compoundability value"),
            ));
        }
        (model::Feature::Countability, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a countability value"),
            ));
        }
        (model::Feature::ModifierLicense, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a modifier-license value"),
            ));
        }
        (model::Feature::DeterminerNumber, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a determiner-number value"),
            ));
        }
        (model::Feature::FusedHeadLicense, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a fused-head-license value"),
            ));
        }
        (model::Feature::NominalForm, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a nominal-form value"),
            ));
        }
        (model::Feature::NominalLicense, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a nominal-license value"),
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
        (model::Feature::Participle, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a participle value"),
            ));
        }
        (model::Feature::Properness, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a properness value"),
            ));
        }
        (model::Feature::NounComplement, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a noun-complement value"),
            ));
        }
        (model::Feature::PrepositionClass, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a preposition-class value"),
            ));
        }
        (model::Feature::Relationality, _) => {
            return Err(syn::Error::new_spanned(
                path,
                format!("`{name}` is not a relationality value"),
            ));
        }
    };
    Ok(value)
}

impl From<model::Feature> for Feature {
    fn from(value: model::Feature) -> Self {
        match value {
            model::Feature::Agreement => Self::Agreement,
            model::Feature::BareLocativeLicense => Self::BareLocativeLicense,
            model::Feature::Cardinality => Self::Cardinality,
            model::Feature::Compoundability => Self::Compoundability,
            model::Feature::Countability => Self::Countability,
            model::Feature::ModifierLicense => Self::ModifierLicense,
            model::Feature::DeterminerNumber => Self::DeterminerNumber,
            model::Feature::FusedHeadLicense => Self::FusedHeadLicense,
            model::Feature::NominalForm => Self::NominalForm,
            model::Feature::NominalLicense => Self::NominalLicense,
            model::Feature::NounComplement => Self::NounComplement,
            model::Feature::Number => Self::Number,
            model::Feature::Onset => Self::Onset,
            model::Feature::Participle => Self::Participle,
            model::Feature::PossessiveEnding => Self::PossessiveEnding,
            model::Feature::PrepositionClass => Self::PrepositionClass,
            model::Feature::Properness => Self::Properness,
            model::Feature::Relationality => Self::Relationality,
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
        assert_eq!(
            lower_constant(
                Feature::Relationality,
                &syn::parse_quote!(Anything::Relational),
            )
            .unwrap(),
            FeatureValue::Relational
        );
        assert!(
            lower_constant(Feature::Relationality, &syn::parse_quote!(Anything::Common),).is_err()
        );
        assert_eq!(
            lower_constant(
                Feature::FusedHeadLicense,
                &syn::parse_quote!(Anything::FusedHead),
            )
            .unwrap(),
            FeatureValue::FusedHead
        );
        assert_eq!(
            lower_constant(
                Feature::FusedHeadLicense,
                &syn::parse_quote!(Anything::NominalOnly),
            )
            .unwrap(),
            FeatureValue::NominalOnly
        );
        assert!(
            lower_constant(
                Feature::FusedHeadLicense,
                &syn::parse_quote!(Anything::Both),
            )
            .is_err()
        );
    }
}
