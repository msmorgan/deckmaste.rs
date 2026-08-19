use syn::ext::IdentExt;

pub(crate) const VISITOR_TRAIT: &str = "Visitor";
pub(crate) const RULE_CATEGORY_TYPE: &str = "Category";
pub(crate) const RULE_CONSTRUCTION_TYPE: &str = "Construction";
pub(crate) const RULE_ID_TYPE: &str = "RuleId";
pub(crate) const RULE_ID_COUNT: &str = "COUNT";
pub(crate) const RULE_ID_CONSTRUCTION: &str = "construction";
pub(crate) const RULE_ID_INDEX: &str = "index";
pub(crate) const RULES_CONSTANT: &str = "RULES";
pub(crate) const BUILD_FUNCTION: &str = "build";
pub(crate) const AGREEMENT_TYPE: &str = "Agreement";
pub(crate) const NUMBER_TYPE: &str = "Number";
pub(crate) const FEATURE_CONSTRAINT_TYPE: &str = "FeatureConstraint";
pub(crate) const CASE_POSITION_TYPE: &str = "CasePosition";
pub(crate) const SCAN_POSITION_TYPE: &str = "ScanPosition";
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
    AGREEMENT_TYPE,
    NUMBER_TYPE,
    FEATURE_CONSTRAINT_TYPE,
    CASE_POSITION_TYPE,
    SCAN_POSITION_TYPE,
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
];

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
    let suffix = if standalone_root {
        format!("{}_body", snake_case(category))
    } else {
        snake_case(category)
    };
    format!("render_{suffix}")
}

pub(crate) fn feature_helper(feature: &str, category: &str) -> String {
    format!("{feature}_for_{}", snake_case(category))
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
