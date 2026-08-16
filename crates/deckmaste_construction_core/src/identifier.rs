use syn::ext::IdentExt;

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
