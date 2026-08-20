use macro_ron::v2::SurfaceFeature;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MorphologyRecipe {
    EnglishVerb,
    EnglishNoun,
}

impl MorphologyRecipe {
    pub(crate) fn from_ident(recipe: &syn::Ident) -> Option<Self> {
        match recipe.to_string().as_str() {
            "english_verb" => Some(Self::EnglishVerb),
            "english_noun" => Some(Self::EnglishNoun),
            _ => None,
        }
    }

    pub(crate) fn feature(self) -> crate::Feature {
        match self {
            Self::EnglishVerb => crate::Feature::Agreement,
            Self::EnglishNoun => crate::Feature::Number,
        }
    }

    pub(crate) fn features(self) -> &'static [SurfaceFeature] {
        match self {
            Self::EnglishVerb => &[SurfaceFeature::Bare, SurfaceFeature::ThirdPersonSingular],
            Self::EnglishNoun => &[SurfaceFeature::Singular, SurfaceFeature::Plural],
        }
    }

    pub(crate) fn feature_from_ident(self, feature: &syn::Ident) -> Option<SurfaceFeature> {
        match (self, feature.to_string().as_str()) {
            (Self::EnglishVerb, "Bare") => Some(SurfaceFeature::Bare),
            (Self::EnglishVerb, "ThirdPersonSingular") => Some(SurfaceFeature::ThirdPersonSingular),
            (Self::EnglishNoun, "Singular") => Some(SurfaceFeature::Singular),
            (Self::EnglishNoun, "Plural") => Some(SurfaceFeature::Plural),
            _ => None,
        }
    }
}

pub(crate) fn derive_surface(
    recipe: MorphologyRecipe,
    lemma: &str,
    feature: SurfaceFeature,
) -> syn::Result<String> {
    // Open plural recipes stay unavailable until attested. Growing the lexical
    // override inventory does not authorize heuristic recipe expansion.
    let surface = match (recipe, feature) {
        (MorphologyRecipe::EnglishVerb, SurfaceFeature::Bare)
        | (MorphologyRecipe::EnglishNoun, SurfaceFeature::Singular) => lemma.to_owned(),
        (MorphologyRecipe::EnglishVerb, SurfaceFeature::ThirdPersonSingular)
        | (MorphologyRecipe::EnglishNoun, SurfaceFeature::Plural) => format!("{lemma}s"),
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "sealed morphology recipe has a mismatched feature axis",
            ));
        }
    };
    Ok(surface)
}
