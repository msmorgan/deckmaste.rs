use crate::Declaration;
use crate::ValidatedDeclarations;

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
}

#[derive(Debug, PartialEq, Eq)]
pub struct EscapeHatchReport {
    terminal_bindings: Vec<TerminalBindingDeclaration>,
    checked_constructor_bindings: Vec<String>,
    roots: Vec<String>,
}

impl EscapeHatchReport {
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
    let mut terminal_bindings = Vec::new();
    let mut checked_constructor_bindings = Vec::new();
    let mut roots = Vec::new();

    for declaration in &validated.raw().declarations {
        match declaration {
            Declaration::Codec(binding) => terminal_bindings.push(TerminalBindingDeclaration {
                kind: TerminalBindingDeclarationKind::Codec,
                name: binding.name.to_string(),
            }),
            Declaration::Identity(binding) => {
                terminal_bindings.push(TerminalBindingDeclaration {
                    kind: TerminalBindingDeclarationKind::Identity,
                    name: binding.name.to_string(),
                });
            }
            Declaration::Construction(construction) if construction.checked.is_some() => {
                checked_constructor_bindings.push(construction.element.name.to_string());
            }
            Declaration::Root(root) => roots.push(
                root.category
                    .segments
                    .last()
                    .map_or_else(String::new, |segment| segment.ident.to_string()),
            ),
            Declaration::Construction(_) | Declaration::Vocab(_) | Declaration::Lexeme(_) => {}
        }
    }

    EscapeHatchReport {
        terminal_bindings,
        checked_constructor_bindings,
        roots,
    }
}

#[cfg(test)]
mod tests {
    use crate::TerminalBindingDeclarationKind;
    use crate::test_support::synthetic_projection_expansion;

    #[test]
    fn report_is_a_read_only_source_order_view_of_the_three_counted_kinds() {
        let expansion = synthetic_projection_expansion();
        let report = expansion.escape_hatches();

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
            report.checked_constructor_bindings(),
            ["NestedNode", "DocumentNode"]
        );
        assert_eq!(report.roots(), ["Document"]);
    }
}
