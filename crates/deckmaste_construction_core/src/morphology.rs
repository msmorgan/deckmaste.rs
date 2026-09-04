use crate::macro_def::SurfaceFeature;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
    clippy::enum_variant_names,
    reason = "the private recipe names mirror the three sealed declaration-language recipes"
)]
pub(crate) enum MorphologyRecipe {
    EnglishVerb,
    EnglishNoun,
    EnglishParticiple,
}

impl MorphologyRecipe {
    pub(crate) fn from_ident(recipe: &syn::Ident) -> Option<Self> {
        match recipe.to_string().as_str() {
            "english_verb" => Some(Self::EnglishVerb),
            "english_noun" => Some(Self::EnglishNoun),
            "english_participle" => Some(Self::EnglishParticiple),
            _ => None,
        }
    }

    pub(crate) fn feature(self) -> crate::Feature {
        match self {
            Self::EnglishVerb => crate::Feature::ConcordClass,
            Self::EnglishNoun => crate::Feature::Number,
            Self::EnglishParticiple => crate::Feature::Participle,
        }
    }

    pub(crate) fn features(self) -> &'static [SurfaceFeature] {
        match self {
            Self::EnglishVerb => &[
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
            ],
            Self::EnglishNoun => &[SurfaceFeature::Singular, SurfaceFeature::Plural],
            Self::EnglishParticiple => &[SurfaceFeature::PAST_PARTICIPLE],
        }
    }

    pub(crate) fn feature_from_ident(self, feature: &syn::Ident) -> Option<SurfaceFeature> {
        match (self, feature.to_string().as_str()) {
            (Self::EnglishVerb, "Other") => Some(SurfaceFeature::PLAIN),
            (Self::EnglishVerb, "ThirdPersonSingular") => {
                Some(SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT)
            }
            (Self::EnglishNoun, "Singular") => Some(SurfaceFeature::Singular),
            (Self::EnglishNoun, "Plural") => Some(SurfaceFeature::Plural),
            (Self::EnglishParticiple, "Participle") => Some(SurfaceFeature::PAST_PARTICIPLE),
            _ => None,
        }
    }
}

pub(crate) fn derive_surface(
    recipe: MorphologyRecipe,
    lemma: &str,
    feature: SurfaceFeature,
) -> syn::Result<String> {
    // Open plurals stay unavailable until attested. Growing the lexical
    // override inventory does not authorize heuristic recipe expansion.
    let surface = match (recipe, feature) {
        (MorphologyRecipe::EnglishVerb, SurfaceFeature::PLAIN)
        | (MorphologyRecipe::EnglishNoun, SurfaceFeature::Singular) => lemma.to_owned(),
        (MorphologyRecipe::EnglishVerb, SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT)
        | (MorphologyRecipe::EnglishNoun, SurfaceFeature::Plural) => format!("{lemma}s"),
        (MorphologyRecipe::EnglishParticiple, SurfaceFeature::PAST_PARTICIPLE) => {
            format!("{lemma}ed")
        }
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "sealed morphology recipe has a mismatched feature axis",
            ));
        }
    };
    Ok(surface)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_def::InflectionalForm;

    #[test]
    fn verb_recipes_share_one_inflectional_form_dimension() {
        let other = syn::parse_quote!(Other);
        let third_person_singular = syn::parse_quote!(ThirdPersonSingular);
        let participle = syn::parse_quote!(Participle);
        assert_eq!(
            MorphologyRecipe::EnglishVerb
                .feature_from_ident(&other)
                .and_then(SurfaceFeature::inflectional_form),
            Some(InflectionalForm::Plain)
        );
        assert_eq!(
            MorphologyRecipe::EnglishVerb
                .feature_from_ident(&third_person_singular)
                .and_then(SurfaceFeature::inflectional_form),
            Some(InflectionalForm::ThirdPersonSingularPresent)
        );
        assert_eq!(
            MorphologyRecipe::EnglishParticiple
                .feature_from_ident(&participle)
                .and_then(SurfaceFeature::inflectional_form),
            Some(InflectionalForm::PastParticiple)
        );
        assert_eq!(
            SurfaceFeature::Singular.inflectional_form(),
            None,
            "nominal Number must not be treated as an Inflectional Form"
        );

        let plain = InflectionalForm::Plain.concord_class_applicability();
        assert!(plain.without_concord_class);
        assert!(plain.other);
        assert!(!plain.third_person_singular);

        let third_person =
            InflectionalForm::ThirdPersonSingularPresent.concord_class_applicability();
        assert!(!third_person.without_concord_class);
        assert!(!third_person.other);
        assert!(third_person.third_person_singular);

        let preterite = InflectionalForm::Preterite.concord_class_applicability();
        assert!(!preterite.without_concord_class);
        assert!(preterite.other);
        assert!(preterite.third_person_singular);

        for participial in [
            InflectionalForm::GerundParticiple,
            InflectionalForm::PastParticiple,
        ] {
            let applicability = participial.concord_class_applicability();
            assert!(applicability.without_concord_class);
            assert!(!applicability.other);
            assert!(!applicability.third_person_singular);
        }
    }

    #[test]
    fn strict_s_edge_regression() {
        for (recipe, identity, derived, lemma, identity_surface, derived_surface) in [
            (
                MorphologyRecipe::EnglishNoun,
                SurfaceFeature::Singular,
                SurfaceFeature::Plural,
                "class",
                "class",
                "classs",
            ),
            (
                MorphologyRecipe::EnglishNoun,
                SurfaceFeature::Singular,
                SurfaceFeature::Plural,
                "ally",
                "ally",
                "allys",
            ),
            (
                MorphologyRecipe::EnglishNoun,
                SurfaceFeature::Singular,
                SurfaceFeature::Plural,
                "dwarf",
                "dwarf",
                "dwarfs",
            ),
            (
                MorphologyRecipe::EnglishVerb,
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "cross",
                "cross",
                "crosss",
            ),
            (
                MorphologyRecipe::EnglishVerb,
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "try",
                "try",
                "trys",
            ),
            (
                MorphologyRecipe::EnglishVerb,
                SurfaceFeature::PLAIN,
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                "scoff",
                "scoff",
                "scoffs",
            ),
        ] {
            assert_eq!(
                derive_surface(recipe, lemma, identity).expect("identity feature derives"),
                identity_surface
            );
            assert_eq!(
                derive_surface(recipe, lemma, derived).expect("derived feature derives"),
                derived_surface
            );
        }
    }
}
