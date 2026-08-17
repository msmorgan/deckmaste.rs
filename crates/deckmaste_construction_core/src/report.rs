use crate::Declaration;
use crate::RenderBinding;
use crate::ValidatedDeclarations;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;

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
    /// Returns identities with a context-identity render binding of two or
    /// more arms, in source order.
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

pub(crate) fn escape_hatch_report(validated: &ValidatedDeclarations) -> EscapeHatchReport {
    let mapping_layers = Vec::new();
    let mut handwritten_codecs = Vec::new();
    let stored_form_tags = Vec::new();
    let mut stored_spelling_codecs = Vec::new();
    let mut terminal_bindings = Vec::new();
    let mut checked_constructor_bindings = Vec::new();
    let mut roots = Vec::new();

    for declaration in &validated.raw().declarations {
        match declaration {
            Declaration::Codec(binding) => {
                let name = identifier_key(&binding.name);
                handwritten_codecs.push(name.clone());
                terminal_bindings.push(TerminalBindingDeclaration {
                    kind: TerminalBindingDeclarationKind::Codec,
                    name,
                });
            }
            Declaration::Identity(binding) => {
                let name = identifier_key(&binding.name);
                if matches!(
                    binding.render,
                    Some(RenderBinding::ContextIdentity(ref arms)) if arms.len() >= 2
                ) {
                    stored_spelling_codecs.push(name.clone());
                }
                terminal_bindings.push(TerminalBindingDeclaration {
                    kind: TerminalBindingDeclarationKind::Identity,
                    name,
                });
            }
            Declaration::Construction(construction) if construction.checked.is_some() => {
                checked_constructor_bindings.push(identifier_key(&construction.element.name));
            }
            Declaration::Root(root) => roots.push(path_key(&root.category)),
            Declaration::Construction(_) | Declaration::Vocab(_) | Declaration::Lexeme(_) => {}
        }
    }

    EscapeHatchReport {
        mapping_layers,
        handwritten_codecs,
        stored_form_tags,
        stored_spelling_codecs,
        terminal_bindings,
        checked_constructor_bindings,
        roots,
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
        assert_eq!(
            report.checked_constructor_bindings(),
            ["NestedNode", "DocumentNode"]
        );
        assert_eq!(report.roots(), ["Document"]);
    }
}
