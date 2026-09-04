use std::collections::HashSet;

use crate::model::TerminalBindingKind;
use crate::semantic::FixedSurfaceAtomPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SeparatorPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::TerminalPlan;
use crate::semantic::ValueKindPlan;

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
    abstract_products: Vec<String>,
    abstract_sums: Vec<String>,
    optional_roles: Vec<String>,
    sequence_roles: Vec<String>,
    sequence_feature_roles: Vec<String>,
    uniform_separators: Vec<String>,
    positional_separator_tables: Vec<String>,
    terminators: Vec<String>,
    stored_separator_fields: Vec<String>,
    generated_form_boundaries: Vec<String>,
    stored_form_boundary_fields: Vec<String>,
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
    pub fn abstract_products(&self) -> &[String] {
        &self.abstract_products
    }

    #[must_use]
    pub fn abstract_sums(&self) -> &[String] {
        &self.abstract_sums
    }

    #[must_use]
    pub fn optional_roles(&self) -> &[String] {
        &self.optional_roles
    }

    #[must_use]
    pub fn sequence_roles(&self) -> &[String] {
        &self.sequence_roles
    }

    #[must_use]
    /// Returns statically nonempty sequence roles that participate in generated
    /// feature flow. The feature is derived, never stored.
    pub fn sequence_feature_roles(&self) -> &[String] {
        &self.sequence_feature_roles
    }

    #[must_use]
    pub fn uniform_separators(&self) -> &[String] {
        &self.uniform_separators
    }

    #[must_use]
    pub fn positional_separator_tables(&self) -> &[String] {
        &self.positional_separator_tables
    }

    #[must_use]
    pub fn terminators(&self) -> &[String] {
        &self.terminators
    }

    #[must_use]
    /// Returns sequence roles that store separator syntax in the public AST.
    /// Derived structural surfaces never contribute a row here.
    pub fn stored_separator_fields(&self) -> &[String] {
        &self.stored_separator_fields
    }

    #[must_use]
    /// Returns fixed circumfix boundaries generated from form syntax, in source
    /// order.
    pub fn generated_form_boundaries(&self) -> &[String] {
        &self.generated_form_boundaries
    }

    #[must_use]
    /// Returns AST fields that store circumfix boundary syntax.
    /// Generated form boundaries never contribute a row here.
    pub fn stored_form_boundary_fields(&self) -> &[String] {
        &self.stored_form_boundary_fields
    }

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

struct StructuralReportInventory {
    abstract_products: Vec<String>,
    abstract_sums: Vec<String>,
    optional_roles: Vec<String>,
    sequence_roles: Vec<String>,
    sequence_feature_roles: Vec<String>,
    uniform_separators: Vec<String>,
    positional_separator_tables: Vec<String>,
    terminators: Vec<String>,
    stored_separator_fields: Vec<String>,
}

fn structural_report_inventory(plan: &SemanticPlan) -> StructuralReportInventory {
    let abstract_products = plan
        .products()
        .iter()
        .map(|product| product.name().to_owned())
        .collect();
    let abstract_sums = plan
        .sums()
        .iter()
        .map(|sum| sum.name().to_owned())
        .collect();
    let mut optional_roles = Vec::new();
    let mut sequence_roles = Vec::new();
    let mut sequence_feature_roles = Vec::new();
    let mut uniform_separators = Vec::new();
    let mut positional_separator_tables = Vec::new();
    let mut terminators = Vec::new();
    let mut stored_separator_fields = Vec::new();
    let mut structural_owners = plan
        .products()
        .iter()
        .map(|product| {
            (
                product.source_index(),
                product.name().to_owned(),
                product
                    .fields()
                    .iter()
                    .map(|field| (field.name().to_owned(), field.kind()))
                    .collect::<Vec<_>>(),
            )
        })
        .chain(plan.constructions().iter().map(|construction| {
            (
                construction.source_index(),
                construction.element_type().to_owned(),
                construction
                    .fields()
                    .iter()
                    .filter_map(|field| {
                        field
                            .structural_kind()
                            .map(|kind| (field.name().to_string(), kind))
                    })
                    .collect::<Vec<_>>(),
            )
        }))
        .collect::<Vec<_>>();
    structural_owners.sort_by_key(|(source_index, _, _)| *source_index);
    let separator_terminals = structural_owners
        .iter()
        .flat_map(|(_, _, fields)| fields)
        .filter_map(|(_, kind)| match kind {
            StructuralFieldKindPlan::Sequence { surface, .. } => surface.separator(),
            StructuralFieldKindPlan::Required(_)
            | StructuralFieldKindPlan::Zeroable(_)
            | StructuralFieldKindPlan::Optional(_) => None,
        })
        .flat_map(|separator| match separator {
            SeparatorPlan::Uniform(surface) => vec![surface],
            SeparatorPlan::Positional(rows) => rows
                .iter()
                .map(crate::semantic::PositionalSeparatorPlan::surface)
                .collect::<Vec<_>>(),
        })
        .flat_map(crate::semantic::FixedSurfacePlan::atoms)
        .filter_map(|atom| match atom {
            FixedSurfaceAtomPlan::Lex { terminal, .. } => Some(terminal.as_str()),
            FixedSurfaceAtomPlan::Literal(_) => None,
        })
        .collect::<HashSet<_>>();
    for (_, owner, fields) in structural_owners {
        for (field, kind) in fields {
            let role = format!("{owner}.{field}");
            match kind {
                StructuralFieldKindPlan::Required(value)
                | StructuralFieldKindPlan::Zeroable(value) => {
                    if stored_value_uses_separator_terminal(value, &separator_terminals) {
                        stored_separator_fields.push(role);
                    }
                }
                StructuralFieldKindPlan::Optional(value) => {
                    if stored_value_uses_separator_terminal(value, &separator_terminals) {
                        stored_separator_fields.push(role.clone());
                    }
                    optional_roles.push(role);
                }
                StructuralFieldKindPlan::Sequence { surface, .. } => {
                    sequence_roles.push(role.clone());
                    for feature in plan.sequence_features(&owner, &field) {
                        sequence_feature_roles.push(format!("{role}.{}", feature.key()));
                    }
                    match surface.separator() {
                        Some(SeparatorPlan::Uniform(_)) => {
                            uniform_separators.push(role.clone());
                        }
                        Some(SeparatorPlan::Positional(_)) => {
                            positional_separator_tables.push(role.clone());
                        }
                        None => {}
                    }
                    if surface.terminator().is_some() {
                        terminators.push(role);
                    }
                }
            }
        }
    }
    StructuralReportInventory {
        abstract_products,
        abstract_sums,
        optional_roles,
        sequence_roles,
        sequence_feature_roles,
        uniform_separators,
        positional_separator_tables,
        terminators,
        stored_separator_fields,
    }
}

fn stored_value_uses_separator_terminal(
    value: &ValueKindPlan,
    separator_terminals: &HashSet<&str>,
) -> bool {
    matches!(value, ValueKindPlan::Lex(terminal) if separator_terminals.contains(terminal.as_str()))
}

fn generated_form_boundaries(plan: &SemanticPlan) -> Vec<String> {
    plan.constructions()
        .iter()
        .flat_map(|construction| {
            construction.forms().iter().flat_map(move |form| {
                form.atoms()
                    .iter()
                    .enumerate()
                    .flat_map(move |(atom_index, atom)| match atom {
                        crate::semantic::AtomPlan::Circumfix { prefix, suffix, .. } => vec![
                            format!(
                                "{}.{}[{atom_index}].prefix={prefix}",
                                construction.construction_id(),
                                form.name(),
                            ),
                            format!(
                                "{}.{}[{atom_index}].suffix={suffix}",
                                construction.construction_id(),
                                form.name(),
                            ),
                        ]
                        .into_iter(),
                        crate::semantic::AtomPlan::SentenceInitialLiteral(surface) => {
                            vec![format!(
                                "{}.{}[{atom_index}].sentence_initial={surface}",
                                construction.construction_id(),
                                form.name(),
                            )]
                            .into_iter()
                        }
                        crate::semantic::AtomPlan::Literal(_)
                        | crate::semantic::AtomPlan::Category { .. }
                        | crate::semantic::AtomPlan::Lex { .. }
                        | crate::semantic::AtomPlan::LexFixed { .. }
                        | crate::semantic::AtomPlan::Identity { .. }
                        | crate::semantic::AtomPlan::Noun { .. }
                        | crate::semantic::AtomPlan::VerbFixed { .. }
                        | crate::semantic::AtomPlan::OpenDeclaration(_)
                        | crate::semantic::AtomPlan::Bound { .. } => Vec::new().into_iter(),
                    })
            })
        })
        .collect()
}

pub(crate) fn escape_hatch_report(plan: &SemanticPlan) -> syn::Result<EscapeHatchReport> {
    let StructuralReportInventory {
        abstract_products,
        abstract_sums,
        optional_roles,
        sequence_roles,
        sequence_feature_roles,
        uniform_separators,
        positional_separator_tables,
        terminators,
        stored_separator_fields,
    } = structural_report_inventory(plan);
    let mapping_layers = Vec::new();
    let generated_form_boundaries = generated_form_boundaries(plan);
    let stored_form_boundary_fields = Vec::new();
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
            if identity.origin().kind() != crate::SourceDeclarationKind::Identity
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
            TerminalBindingKind::Codec => crate::SourceDeclarationKind::Codec,
            TerminalBindingKind::Identity => crate::SourceDeclarationKind::Identity,
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
                if binding.declaration_verb().is_some() {
                    continue;
                }
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
        abstract_products,
        abstract_sums,
        optional_roles,
        sequence_roles,
        sequence_feature_roles,
        uniform_separators,
        positional_separator_tables,
        terminators,
        stored_separator_fields,
        generated_form_boundaries,
        stored_form_boundary_fields,
        mapping_layers,
        handwritten_codecs,
        stored_form_tags,
        stored_spelling_codecs,
        morphology_irregulars,
        terminal_bindings,
        roots,
    })
}

fn surface_feature_key(feature: crate::macro_def::SurfaceFeature) -> &'static str {
    match feature {
        crate::macro_def::SurfaceFeature::PLAIN => "plain",
        crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
            "third_person_singular_present"
        }
        crate::macro_def::SurfaceFeature::Singular => "singular",
        crate::macro_def::SurfaceFeature::Plural => "plural",
        crate::macro_def::SurfaceFeature::PAST_PARTICIPLE => "past_participle",
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::Preterite,
        ) => "preterite",
        crate::macro_def::SurfaceFeature::Inflectional(
            crate::macro_def::InflectionalForm::GerundParticiple,
        ) => "gerund_participle",
        crate::macro_def::SurfaceFeature::Fixed => "fixed",
        crate::macro_def::SurfaceFeature::BlockLabel => "block_label",
    }
}

#[cfg(test)]
mod tests {
    use crate::TerminalBindingDeclaration;
    use crate::TerminalBindingDeclarationKind;
    use crate::test_support::synthetic_projection_expansion;

    #[test]
    fn derived_guarded_forms_never_become_stored_form_tags() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Word { That = "that", Those = "those", }
                construction demonstrative: NounPhrase {
                    element Demonstrative { word: lex Word, }
                    form that when word is That = lex(word);
                    form those otherwise = lex(word);
                }
                root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("guarded report fixture parses"),
        )
        .expect("guarded report fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("guarded report seals");

        assert!(report.stored_form_tags().is_empty());
    }

    #[test]
    fn structural_inventory_is_source_ordered_and_separator_storage_stays_empty() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction node: Node {
                    element NodeValue {}
                    form node = "node";
                }
                abstract sum Choice { node: Node, }
                abstract product Holder {
                    maybe: opt Node,
                    items: seq Choice
                        separated by position {
                            pair = " and ";
                            first = ", ";
                            middle = ", ";
                            last = ", and ";
                        }
                        terminated by ".",
                }
                require len(Holder.items) >= 2;
                root Node { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural inventory fixture parses"),
        )
        .expect("structural inventory fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("sealed report");

        assert_eq!(report.abstract_products(), ["Holder"]);
        assert_eq!(report.abstract_sums(), ["Choice"]);
        assert_eq!(report.optional_roles(), ["Holder.maybe"]);
        assert_eq!(report.sequence_roles(), ["Holder.items"]);
        assert!(report.uniform_separators().is_empty());
        assert_eq!(report.positional_separator_tables(), ["Holder.items"]);
        assert_eq!(report.terminators(), ["Holder.items"]);
        assert!(report.stored_separator_fields().is_empty());
    }

    #[test]
    fn homogeneous_sequence_feature_flow_is_reported_without_stored_fields() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction bare: Item {
                    element BareItem {}
                    derive concord_class = Values::Other;
                    form bare = "bare";
                }
                construction coordinated: Root {
                    element Coordinated {
                        members: seq Item separated by " ",
                    }
                    require len(members) >= 2;
                    derive concord_class = members.concord_class;
                    form coordinated = members;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("sequence feature report fixture parses"),
        )
        .expect("sequence feature report fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("sealed report");

        assert_eq!(
            report.sequence_feature_roles(),
            ["Coordinated.members.concord_class"]
        );
        assert!(report.stored_separator_fields().is_empty());
        assert!(report.stored_form_tags().is_empty());
    }

    #[test]
    fn circumfix_boundaries_are_generated_rows_without_stored_fields() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction bracketed: Root {
                    element Bracketed { value: Item, }
                    form bracketed = circumfix("[", value, "]");
                }
                construction braced: Root {
                    element Braced { values: seq Item separated by "}{", }
                    require len(values) >= 1;
                    form braced = circumfix("{", values, "}");
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("circumfix report fixture parses"),
        )
        .expect("circumfix report fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("circumfix report seals");

        assert_eq!(
            report.generated_form_boundaries(),
            [
                "bracketed.bracketed[0].prefix=[",
                "bracketed.bracketed[0].suffix=]",
                "braced.braced[0].prefix={",
                "braced.braced[0].suffix=}",
            ],
        );
        assert!(report.stored_form_boundary_fields().is_empty());
        assert!(report.stored_separator_fields().is_empty());
        assert_eq!(report.sequence_roles(), ["Braced.values"]);
    }

    #[test]
    fn separator_terminal_reused_by_a_stored_ast_field_is_reported() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab SeparatorWord { And = "and", }
                construction node: Node {
                    element NodeValue {}
                    form node = "node";
                }
                abstract sum Choice { node: Node, }
                abstract product Holder {
                    separator: lex SeparatorWord,
                    items: seq Choice separated by " " lex(SeparatorWord::And),
                }
                root Node { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("separator leakage fixture parses"),
        )
        .expect("separator leakage fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("sealed report");

        assert_eq!(report.stored_separator_fields(), ["Holder.separator"]);
    }

    #[test]
    fn sentence_initial_form_surface_is_reported_without_a_stored_ast_field() {
        let semantic = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction item: Root {
                    element ItemValue {}
                    form item = "item" sentence_initial(": ") "next";
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("sentence-initial report fixture parses"),
        )
        .expect("sentence-initial report fixture validates")
        .into_semantic();
        let report = super::escape_hatch_report(&semantic).expect("report seals");

        assert_eq!(
            report.generated_form_boundaries(),
            ["item.item[1].sentence_initial=: "],
        );
        assert!(report.stored_form_boundary_fields().is_empty());
    }

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
    fn generated_declaration_verb_is_not_a_handwritten_codec_escape_hatch() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme CoreVerb using EnglishVerb { Act = "act", }
            codec ActionVerb {
                generate declaration_verb {
                    closed = CoreVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            construction action: Root {
                element Action { head: lex ActionVerb, }
                derive head.concord_class = Values::Other;
                form action = verb(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("generated declaration-verb report fixture validates");
        let report = expansion.escape_hatches();

        assert!(report.handwritten_codecs().is_empty());
        assert!(report.terminal_bindings().is_empty());
    }

    #[test]
    fn report_exposes_source_ordered_complete_morphology_irregulars() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb {
                feature = ConcordClass;
                recipe = english_verb;
            }
            lexeme VerbLexeme using EnglishVerb {
                FirstIrregular = "first" {
                    Other = "first bare",
                    ThirdPersonSingular = "first singular",
                },
                Regular = "regular",
                SecondIrregular = "second" {
                    ThirdPersonSingular = "second singular",
                },
            }
            construction action: Action {
                element ActionElement {}
                derive verb.concord_class = Values::Other;
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
                ("plain", "first bare"),
                ("third_person_singular_present", "first singular"),
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
            [("third_person_singular_present", "second singular")]
        );
        assert!(
            irregulars
                .iter()
                .all(|row| row.identity() != "lexeme:VerbLexeme/Regular")
        );
    }
}
