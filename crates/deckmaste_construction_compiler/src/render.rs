//! Diagnostics → compile errors. Every diagnostic and every note becomes
//! its own `compile_error!` at its own span — the all-errors-reported
//! contract, without nightly Diagnostic APIs.

use proc_macro2::TokenStream;
use quote::quote_spanned;

use crate::diag::Diagnostic;

#[must_use]
pub fn to_compile_errors(diags: &[Diagnostic]) -> TokenStream {
    let mut out = TokenStream::new();
    for diag in diags {
        let code = diag.code.as_str();
        let message = match &diag.construction {
            Some(id) => format!("{code}: {} (construction `{id}`)", diag.message),
            None => format!("{code}: {}", diag.message),
        };
        out.extend(quote_spanned! {diag.span=> ::core::compile_error!(#message); });
        for note in &diag.notes {
            let note_message = format!("note ({code}): {}", note.message);
            out.extend(quote_spanned! {note.span=> ::core::compile_error!(#note_message); });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::DiagCode;

    #[test]
    fn every_diagnostic_and_note_becomes_a_compile_error() {
        let diags = vec![
            Diagnostic::new(DiagCode::DuplicateOrdinal, "alpha", "ordinal 0 reused")
                .with_note("first declared here", proc_macro2::Span::call_site()),
            Diagnostic::group(DiagCode::DuplicateName, "element `m` declared twice"),
        ];
        let rendered = to_compile_errors(&diags).to_string();
        assert_eq!(rendered.matches("compile_error").count(), 3);
        assert!(rendered.contains("EC002: ordinal 0 reused (construction `alpha`)"));
        assert!(rendered.contains("note (EC002): first declared here"));
        assert!(rendered.contains("EC004: element `m` declared twice"));
    }
}
