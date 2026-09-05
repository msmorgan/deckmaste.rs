use syn::ext::IdentExt;

pub(crate) const VISITOR_TRAIT: &str = "Visitor";
pub(crate) const RULE_CATEGORY_TYPE: &str = "Category";
pub(crate) const RULE_CONSTRUCTION_TYPE: &str = "Construction";
pub(crate) const RULE_ID_TYPE: &str = "RuleId";
pub(crate) const RULE_ID_COUNT: &str = "COUNT";
pub(crate) const RULE_ID_PUBLIC_CONSTRUCTION: &str = "public_construction";
pub(crate) const RULE_ID_INDEX: &str = "index";
pub(crate) const RULES_CONSTANT: &str = "RULES";
pub(crate) const BUILD_FUNCTION: &str = "build";
pub(crate) const CHECKED_BUILD_FUNCTION: &str = "build_checked";
pub(crate) const RIGHTMOST_LEAF_TRAIT: &str = "RightmostLeaf";
pub(crate) const RIGHTMOST_LEAF_CATEGORY_TRAIT: &str = "RightmostLeafCategory";
pub(crate) const RIGHTMOST_LEAF_IS_FUNCTION: &str = "rightmost_leaf_is";
pub(crate) const RIGHT_PERIPHERY_PREPOSITION_TRAIT: &str = "RightPeripheryRolePreposition";
pub(crate) const INVARIANT_CONSTRUCTOR: &str = "new";
pub(crate) const CHECKED_CONSTRUCTOR: &str = "try_new";
pub(crate) const BUILD_REJECTION_TYPE: &str = "BuildRejection";
pub(crate) const BUILD_VIOLATION_TYPE: &str = "BuildViolation";
pub(crate) const ADMISSIBLE_SITES_TYPE: &str = "AdmissibleSites";
pub(crate) const ADMISSIBLE_SITES_FIELD: &str = "admissible_sites";
pub(crate) const ATTACHMENT_SITE_PATH_TYPE: &str = "AttachmentSitePath";
pub(crate) const ATTACHMENT_SITE_STEP_TYPE: &str = "AttachmentSiteStep";
pub(crate) const CONCORD_CLASS_TYPE: &str = "ConcordClass";
pub(crate) const CARDINALITY_TYPE: &str = "Cardinality";
pub(crate) const DETERMINER_NUMBER_TYPE: &str = "DeterminerNumber";
pub(crate) const FUSED_HEAD_LICENSE_TYPE: &str = "FusedHeadLicense";
pub(crate) const MANNER_ANAPHOR_CLASS_TYPE: &str = "MannerAnaphorClass";
pub(crate) const NOMINAL_FORM_TYPE: &str = "NominalForm";
pub(crate) const NOMINAL_LICENSE_TYPE: &str = "NominalLicense";
pub(crate) const NUMBER_TYPE: &str = "Number";
pub(crate) const ONSET_TYPE: &str = "Onset";
pub(crate) const POSSESSIVE_ENDING_TYPE: &str = "PossessiveEnding";
pub(crate) const PARTICIPLE_TYPE: &str = "Participle";
pub(crate) const FEATURE_CONSTRAINT_TYPE: &str = "FeatureConstraint";
pub(crate) const CASE_POSITION_TYPE: &str = "CasePosition";
pub(crate) const PREFIX_POSITION_TYPE: &str = "PrefixPosition";
pub(crate) const STRUCTURAL_TRANSITION_TYPE: &str = "StructuralTransition";
pub(crate) const SCAN_POSITION_TYPE: &str = "ScanPosition";
pub(crate) const SEQUENCE_OWNER_TYPE: &str = "SequenceOwner";
pub(crate) const FIXED_SURFACE_ATOM_TYPE: &str = "FixedSurfaceAtom";
pub(crate) const SEQUENCE_SEPARATOR_FUNCTION: &str = "sequence_separator";
pub(crate) const SEQUENCE_TERMINATOR_FUNCTION: &str = "sequence_terminator";
pub(crate) const DECLARATION_CLASS_TYPE: &str = "DeclarationClass";
pub(crate) const DECLARATION_MATCHER_TYPE: &str = "DeclarationMatcher";
pub(crate) const DECLARATION_LEAF_TYPE: &str = "DeclarationLeaf";
pub(crate) const LEXICAL_TYPE: &str = "Lexical";
pub(crate) const LEAF_TYPE: &str = "Leaf";
pub(crate) const TERMINAL_CLASS_TYPE: &str = "TerminalClass";
pub(crate) const LEXICAL_TERMINAL_TYPE: &str = "LexicalTerminal";
pub(crate) const LEXICAL_PROVENANCE_KIND_TYPE: &str = "LexicalProvenanceKind";
pub(crate) const LEXICAL_OWNER_TEMPLATE_TYPE: &str = "LexicalOwnerTemplate";
pub(crate) const LEXICAL_OWNER_TYPE: &str = "LexicalOwner";
pub(crate) const LEXICAL_OWNER_IDENTITY_TYPE: &str = "LexicalOwnerIdentity";
pub(crate) const FIXED_RUNTIME_TYPE_NAMES: &[&str] = &[
    CONCORD_CLASS_TYPE,
    CARDINALITY_TYPE,
    DETERMINER_NUMBER_TYPE,
    FUSED_HEAD_LICENSE_TYPE,
    MANNER_ANAPHOR_CLASS_TYPE,
    NOMINAL_FORM_TYPE,
    NOMINAL_LICENSE_TYPE,
    NUMBER_TYPE,
    ONSET_TYPE,
    POSSESSIVE_ENDING_TYPE,
    PARTICIPLE_TYPE,
    FEATURE_CONSTRAINT_TYPE,
    CASE_POSITION_TYPE,
    PREFIX_POSITION_TYPE,
    STRUCTURAL_TRANSITION_TYPE,
    SCAN_POSITION_TYPE,
    SEQUENCE_OWNER_TYPE,
    FIXED_SURFACE_ATOM_TYPE,
    DECLARATION_CLASS_TYPE,
    DECLARATION_MATCHER_TYPE,
    DECLARATION_LEAF_TYPE,
    LEXICAL_TYPE,
    LEAF_TYPE,
    TERMINAL_CLASS_TYPE,
    LEXICAL_TERMINAL_TYPE,
    LEXICAL_PROVENANCE_KIND_TYPE,
    LEXICAL_OWNER_TEMPLATE_TYPE,
    LEXICAL_OWNER_TYPE,
    LEXICAL_OWNER_IDENTITY_TYPE,
    BUILD_REJECTION_TYPE,
    BUILD_VIOLATION_TYPE,
    ADMISSIBLE_SITES_TYPE,
    ATTACHMENT_SITE_PATH_TYPE,
    ATTACHMENT_SITE_STEP_TYPE,
];

/// Reserved for private standalone-root render dispatch emitted by the
/// compiler.
///
/// Authored declaration identifiers using this prefix are rejected during
/// namespace validation, so no legal declaration can produce this value name.
pub(crate) const PRIVATE_ROOT_RENDERER_PREFIX: &str =
    "__deckmaste_construction_internal_render_root_";

/// Returns the semantic Rust name of an authored identifier.
///
/// Rawness is syntax, not identity: `payload` and `r#payload` occupy the same
/// namespace and must use the same lookup key throughout validation and code
/// generation.
pub(crate) fn key(ident: &syn::Ident) -> String {
    ident.unraw().to_string()
}

pub(crate) fn spelling_key(spelling: &str) -> String {
    spelling.strip_prefix("r#").unwrap_or(spelling).to_owned()
}

pub(crate) fn same(left: &syn::Ident, right: &syn::Ident) -> bool {
    key(left) == key(right)
}

pub(crate) fn path_key(path: &syn::Path) -> String {
    path.segments
        .last()
        .map_or_else(String::new, |segment| key(&segment.ident))
}

/// Returns whether an authored identifier uses raw syntax for a Rust keyword.
///
/// The declaration compiler deliberately does not preserve rawness in derived
/// public names. Raw nonkeywords therefore lower to their semantic spelling,
/// while a raw keyword is rejected at validation rather than reaching an
/// emitter that cannot represent the same identity consistently.
pub(crate) fn is_raw_keyword(ident: &syn::Ident) -> bool {
    ident.to_string().starts_with("r#") && is_rust_keyword(&key(ident))
}

pub(crate) fn pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}

pub(crate) fn snake_case(name: &str) -> String {
    let characters = name.chars().collect::<Vec<_>>();
    let mut result = String::new();
    for (index, character) in characters.iter().copied().enumerate() {
        if character == '_' {
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            continue;
        }
        if character.is_uppercase() {
            let previous_is_lower =
                index > 0 && characters[index - 1] != '_' && characters[index - 1].is_lowercase();
            let acronym_boundary = index > 0
                && characters[index - 1].is_uppercase()
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.is_lowercase());
            if (previous_is_lower || acronym_boundary) && !result.ends_with('_') {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        } else {
            result.push(character);
        }
    }
    while result.ends_with('_') {
        result.pop();
    }
    result
}

pub(crate) fn prefixed(prefix: &str, semantic_name: &str) -> String {
    format!("{prefix}{}", snake_case(semantic_name))
}

pub(crate) fn category_renderer(category: &str, standalone_root: bool) -> String {
    if standalone_root {
        format!("{PRIVATE_ROOT_RENDERER_PREFIX}{}", snake_case(category))
    } else {
        format!("render_{}", snake_case(category))
    }
}

pub(crate) fn feature_helper(feature: &str, category: &str) -> String {
    format!("{feature}_for_{}", snake_case(category))
}

pub(crate) fn lexeme_surface_helper(lexeme: &str) -> String {
    format!("surface_for_{}", snake_case(lexeme))
}

pub(crate) fn structural_sequence_aggregate(owner: &str, role: &str) -> String {
    format!("{}{}Sequence", pascal_case(owner), pascal_case(role))
}

pub(crate) fn structural_sequence_category(owner: &str, role: &str) -> String {
    format!("{}Category", structural_sequence_aggregate(owner, role))
}

pub(crate) fn structural_sequence_counted_category(
    owner: &str,
    role: &str,
    count: usize,
) -> String {
    format!(
        "{}Count{count}Category",
        structural_sequence_aggregate(owner, role)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum StructuralHelperCategoryState {
    Optional,
    UnboundedSequence,
    UniformCount(usize),
    PositionalCount(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StructuralSequenceStyle {
    Uniform,
    Positional,
}

pub(crate) fn structural_sequence_helper_categories(
    owner: &str,
    role: &str,
    maximum: Option<usize>,
    style: StructuralSequenceStyle,
) -> Vec<(StructuralHelperCategoryState, String)> {
    let base = structural_sequence_category(owner, role);
    match (maximum, style) {
        (None, _) => vec![(StructuralHelperCategoryState::UnboundedSequence, base)],
        (Some(maximum), StructuralSequenceStyle::Uniform) => (1..=maximum)
            .map(|count| {
                let name = if count == 1 {
                    base.clone()
                } else {
                    structural_sequence_counted_category(owner, role, count)
                };
                (StructuralHelperCategoryState::UniformCount(count), name)
            })
            .collect(),
        (Some(maximum), StructuralSequenceStyle::Positional) => (2..maximum)
            .map(|position| {
                let name = if position == 2 {
                    base.clone()
                } else {
                    structural_sequence_counted_category(owner, role, position)
                };
                (
                    StructuralHelperCategoryState::PositionalCount(position),
                    name,
                )
            })
            .collect(),
    }
}

pub(crate) fn structural_sequence_rule(owner: &str, role: &str) -> String {
    format!("{}Rule", structural_sequence_aggregate(owner, role))
}

pub(crate) fn structural_sequence_builder(owner: &str, role: &str) -> String {
    prefixed("build_", &format!("{owner}_{role}_sequence"))
}

pub(crate) fn structural_sequence_renderer(owner: &str, role: &str) -> String {
    prefixed("render_", &format!("{owner}_{role}_sequence"))
}

pub(crate) fn structural_sequence_walker(owner: &str, role: &str) -> String {
    prefixed("walk_", &format!("{owner}_{role}_sequence"))
}

/// Returns the legal deterministic spelling used for a generated local.
///
/// Local bindings may originate in authored raw fields, so keywords receive a
/// readable suffix. The encoded fallback closes the policy for internal
/// derived fragments as well: no caller can pass unchecked text through to
/// identifier construction.
pub(crate) fn local_name(preferred: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let semantic_name = spelling_key(preferred);
    if !is_rust_keyword(&semantic_name) && syn::parse_str::<syn::Ident>(&semantic_name).is_ok() {
        return semantic_name;
    }
    if is_rust_keyword(&semantic_name) {
        return format!("{}_value", semantic_name.to_lowercase());
    }

    let mut encoded = String::from("_generated_local");
    for byte in semantic_name.bytes() {
        encoded.push('_');
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

pub(crate) fn emitted_ident(name: &str, span: proc_macro2::Span) -> syn::Ident {
    syn::parse_str::<syn::Ident>(name)
        .map(|mut ident| {
            ident.set_span(span);
            ident
        })
        .expect("sealed generated identifier must be legal Rust syntax")
}

pub(crate) fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "Self"
            | "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "gen"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "union"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}
