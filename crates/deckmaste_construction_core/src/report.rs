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
    terminal_bindings: Vec<TerminalBindingDeclaration>,
    checked_constructor_bindings: Vec<String>,
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
    pub fn terminal_bindings(&self) -> &[TerminalBindingDeclaration] {
        &self.terminal_bindings
    }

    #[must_use]
    pub fn checked_constructor_bindings(&self) -> &[String] {
        &self.checked_constructor_bindings
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
    let mut terminal_bindings = Vec::new();
    let mut checked_constructor_bindings = Vec::new();
    let mut roots = Vec::new();

    for terminal in plan.terminals() {
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
    for construction in plan.constructions() {
        if construction.has_checked_constructor() {
            checked_constructor_bindings.push(construction.element_type().to_owned());
        }
    }
    roots.extend(plan.roots().iter().map(|root| root.category().to_owned()));

    Ok(EscapeHatchReport {
        mapping_layers,
        handwritten_codecs,
        stored_form_tags,
        stored_spelling_codecs,
        terminal_bindings,
        checked_constructor_bindings,
        roots,
    })
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
        assert_eq!(
            report.checked_constructor_bindings(),
            ["NestedNode", "DocumentNode"]
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
}
