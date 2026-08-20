use crate::model::TerminalBindingKind;
use crate::semantic::SemanticPlan;
use crate::semantic::TerminalPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalBindingDeclarationKind {
    Codec,
    Identity,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TerminalBindingDeclaration {
    kind: TerminalBindingDeclarationKind,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MorphologyIrregular {
    identity: String,
    overrides: Vec<MorphologyOverride>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MorphologyOverride {
    feature: String,
    surface: String,
}

impl MorphologyIrregular {
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    #[must_use]
    pub fn overrides(&self) -> &[MorphologyOverride] {
        &self.overrides
    }
}

impl MorphologyOverride {
    #[must_use]
    pub fn feature(&self) -> &str {
        &self.feature
    }

    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }
}

impl TerminalBindingDeclaration {
    #[must_use]
    pub fn kind(&self) -> TerminalBindingDeclarationKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Returns the canonical declaration identity: `codec:<Name>` or
    /// `identity:<Name>`.
    pub fn canonical_identity(&self) -> String {
        format!("{}:{}", self.kind.canonical_prefix(), self.name)
    }
}

impl TerminalBindingDeclarationKind {
    const fn canonical_prefix(self) -> &'static str {
        match self {
            Self::Codec => "codec",
            Self::Identity => "identity",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EscapeHatchReport {
    mapping_layers: Vec<String>,
    handwritten_codecs: Vec<String>,
    stored_form_tags: Vec<String>,
    stored_spelling_codecs: Vec<String>,
    morphology_irregulars: Vec<MorphologyIrregular>,
    terminal_bindings: Vec<TerminalBindingDeclaration>,
    roots: Vec<String>,
}

impl EscapeHatchReport {
    #[must_use]
    /// Returns mapping-layer declarations in source order.
    ///
    /// This explicit category remains empty until mapping-layer declaration
    /// syntax exists.
    pub fn mapping_layers(&self) -> &[String] {
        &self.mapping_layers
    }

    #[must_use]
    /// Returns every handwritten codec declaration in source order.
    pub fn handwritten_codecs(&self) -> &[String] {
        &self.handwritten_codecs
    }

    #[must_use]
    /// Returns stored form-tag declarations in source order.
    ///
    /// This explicit category remains empty until stored form-tag declaration
    /// syntax exists.
    pub fn stored_form_tags(&self) -> &[String] {
        &self.stored_form_tags
    }

    #[must_use]
    /// Returns identities that store a selected spelling arm, in source order.
    pub fn stored_spelling_codecs(&self) -> &[String] {
        &self.stored_spelling_codecs
    }

    #[must_use]
    /// Returns closed lexeme members with explicit morphology overrides in
    /// declaration order. Override evidence remains in authored order.
    pub fn morphology_irregulars(&self) -> &[MorphologyIrregular] {
        &self.morphology_irregulars
    }

    #[must_use]
    pub fn terminal_bindings(&self) -> &[TerminalBindingDeclaration] {
        &self.terminal_bindings
    }

    #[must_use]
    pub fn roots(&self) -> &[String] {
        &self.roots
    }
}

pub(crate) fn escape_hatch_report(plan: &SemanticPlan) -> syn::Result<EscapeHatchReport> {
    let mapping_layers = Vec::new();
    let mut handwritten_codecs = Vec::new();
    let stored_form_tags = Vec::new();
    let mut stored_spelling_codecs = Vec::new();
    let mut morphology_irregulars = Vec::new();
    let mut terminal_bindings = Vec::new();
    let mut roots = Vec::new();

    for terminal in plan.terminals() {
        if let TerminalPlan::Lexeme(lexeme) = terminal {
            morphology_irregulars.extend(lexeme.irregulars().iter().map(|irregular| {
                MorphologyIrregular {
                    identity: format!("lexeme:{}/{}", lexeme.name(), irregular.member()),
                    overrides: irregular
                        .overrides()
                        .iter()
                        .map(|row| MorphologyOverride {
                            feature: surface_feature_key(row.feature()).to_owned(),
                            surface: row.surface().to_owned(),
                        })
                        .collect(),
                }
            }));
            continue;
        }
        if let TerminalPlan::ContextIdentity(identity) = terminal {
            if identity.origin().kind() != crate::DeclarationKind::Identity
                || identity.origin().name() != identity.name()
            {
                return Err(syn::Error::new(
                    identity.origin_span(),
                    format!(
                        "sealed context identity `{}` has mismatched declaration provenance",
                        identity.name()
                    ),
                ));
            }
            stored_spelling_codecs.push(identity.name().to_owned());
            continue;
        }
        let TerminalPlan::Binding(binding) = terminal else {
            continue;
        };
        let name = binding.name().to_owned();
        let expected_origin_kind = match binding.kind() {
            TerminalBindingKind::Codec => crate::DeclarationKind::Codec,
            TerminalBindingKind::Identity => crate::DeclarationKind::Identity,
        };
        if binding.origin().kind() != expected_origin_kind {
            return Err(syn::Error::new(
                binding.origin_span(),
                format!("sealed terminal binding `{name}` has a mismatched declaration kind"),
            ));
        }
        if binding.origin().name() != name {
            return Err(syn::Error::new(
                binding.origin_span(),
                format!("sealed terminal binding `{name}` has a mismatched declaration name"),
            ));
        }
        match binding.kind() {
            TerminalBindingKind::Codec => {
                handwritten_codecs.push(name.clone());
                terminal_bindings.push(TerminalBindingDeclaration {
                    kind: TerminalBindingDeclarationKind::Codec,
                    name,
                });
            }
            TerminalBindingKind::Identity => {
                if binding.has_stored_spelling() {
                    stored_spelling_codecs.push(name.clone());
                }
                terminal_bindings.push(TerminalBindingDeclaration {
                    kind: TerminalBindingDeclarationKind::Identity,
                    name,
                });
            }
        }
    }
    roots.extend(plan.roots().iter().map(|root| root.category().to_owned()));

    Ok(EscapeHatchReport {
        mapping_layers,
        handwritten_codecs,
        stored_form_tags,
        stored_spelling_codecs,
        morphology_irregulars,
        terminal_bindings,
        roots,
    })
}

fn surface_feature_key(feature: macro_ron::v2::SurfaceFeature) -> &'static str {
    match feature {
        macro_ron::v2::SurfaceFeature::Bare => "bare",
        macro_ron::v2::SurfaceFeature::ThirdPersonSingular => "third_person_singular",
        macro_ron::v2::SurfaceFeature::Singular => "singular",
        macro_ron::v2::SurfaceFeature::Plural => "plural",
        macro_ron::v2::SurfaceFeature::Fixed => "fixed",
    }
}

#[cfg(test)]
mod tests {
    use crate::TerminalBindingDeclaration;
    use crate::TerminalBindingDeclarationKind;
    use crate::test_support::synthetic_projection_expansion;

    #[test]
    fn report_is_a_read_only_source_order_view_of_the_seven_counted_kinds() {
        let expansion = synthetic_projection_expansion();
        let report = expansion.escape_hatches();

        assert!(report.mapping_layers().is_empty());
        assert_eq!(report.handwritten_codecs(), ["Resource", "Marker", "Pair"]);
        assert!(report.stored_form_tags().is_empty());
        assert_eq!(report.stored_spelling_codecs(), ["Handle"]);
        assert_eq!(
            report
                .terminal_bindings()
                .iter()
                .map(|binding| (binding.kind(), binding.name()))
                .collect::<Vec<_>>(),
            [
                (TerminalBindingDeclarationKind::Codec, "Resource"),
                (TerminalBindingDeclarationKind::Codec, "Marker"),
                (TerminalBindingDeclarationKind::Identity, "Handle"),
                (TerminalBindingDeclarationKind::Codec, "Pair"),
                (TerminalBindingDeclarationKind::Identity, "Record"),
            ]
        );
        assert_eq!(
            report
                .terminal_bindings()
                .iter()
                .map(TerminalBindingDeclaration::canonical_identity)
                .collect::<Vec<_>>(),
            [
                "codec:Resource",
                "codec:Marker",
                "identity:Handle",
                "codec:Pair",
                "identity:Record",
            ]
        );
        assert_eq!(report.roots(), ["Document"]);
    }

    #[test]
    fn generated_context_identity_remains_a_stored_spelling_but_not_a_terminal_binding() {
        let expansion = crate::generate(quote::quote! {
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction self_reference: NounPhrase {
                element SelfReferenceNp { spelling: identity SelfReferenceSpelling, }
                form self_reference = identity(spelling);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("generated context identity fixture validates");
        assert_eq!(
            expansion.escape_hatches().stored_spelling_codecs(),
            ["SelfReferenceSpelling"]
        );
        assert!(expansion.escape_hatches().terminal_bindings().is_empty());
    }

    #[test]
    fn report_exposes_source_ordered_complete_morphology_irregulars() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb {
                feature = Agreement;
                recipe = english_verb;
            }
            lexeme VerbLexeme using EnglishVerb {
                FirstIrregular = "first" {
                    Bare = "first bare",
                    ThirdPersonSingular = "first singular",
                },
                Regular = "regular",
                SecondIrregular = "second" {
                    ThirdPersonSingular = "second singular",
                },
            }
            construction action: Action {
                element ActionElement {}
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::FirstIrregular);
            }
            root Action { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("generated morphology report fixture validates");

        let irregulars = expansion.escape_hatches().morphology_irregulars();
        assert_eq!(irregulars.len(), 2);
        assert_eq!(irregulars[0].identity(), "lexeme:VerbLexeme/FirstIrregular");
        assert_eq!(
            irregulars[0]
                .overrides()
                .iter()
                .map(|row| (row.feature(), row.surface()))
                .collect::<Vec<_>>(),
            [
                ("bare", "first bare"),
                ("third_person_singular", "first singular"),
            ]
        );
        assert_eq!(
            irregulars[1].identity(),
            "lexeme:VerbLexeme/SecondIrregular"
        );
        assert_eq!(
            irregulars[1]
                .overrides()
                .iter()
                .map(|row| (row.feature(), row.surface()))
                .collect::<Vec<_>>(),
            [("third_person_singular", "second singular")]
        );
        assert!(
            irregulars
                .iter()
                .all(|row| row.identity() != "lexeme:VerbLexeme/Regular")
        );
    }
}
