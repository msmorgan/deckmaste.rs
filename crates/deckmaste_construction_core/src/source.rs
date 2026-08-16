use proc_macro2::TokenTree;
use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::model::Invocation;

pub(crate) fn invocation_from_source(source: &str) -> syn::Result<Invocation> {
    let file = syn::parse_file(source)?;
    let mut fence = SourceFence::default();
    fence.visit_file(&file);
    if let Some(error) = fence.error {
        return Err(error);
    }

    fence.invocation.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected exactly one direct constructions! invocation",
        )
    })
}

#[derive(Default)]
struct SourceFence {
    invocation: Option<Invocation>,
    error: Option<syn::Error>,
}

impl<'ast> Visit<'ast> for SourceFence {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if self.error.is_some() {
            return;
        }
        if let Err(error) = inspect_macro(mac, &mut self.invocation) {
            self.error = Some(error);
            return;
        }
        syn::visit::visit_macro(self, mac);
    }
}

fn inspect_macro(mac: &syn::Macro, invocation: &mut Option<Invocation>) -> syn::Result<()> {
    let path = &mac.path;
    let Some(last) = path.segments.last() else {
        return Ok(());
    };
    let name = last.ident.to_string();

    if name == "include" {
        return Err(deferred(path.span(), "include!"));
    }
    if (name == "vocab" || name == "lexeme") && path.segments.len() == 1 {
        return Err(deferred(path.span(), &format!("separate {name}! macro")));
    }
    if name != "constructions" {
        return Ok(());
    }
    if path.segments.len() != 1 || path.leading_colon.is_some() {
        return Err(syn::Error::new(
            path.span(),
            "constructions! invocation must be direct",
        ));
    }
    if invocation.is_some() {
        return Err(syn::Error::new(
            path.span(),
            "expected exactly one constructions! invocation",
        ));
    }

    let tokens = mac.tokens.clone();
    reject_indirection(&tokens)?;
    *invocation = Some(Invocation {
        tokens,
        span: path.span(),
    });
    Ok(())
}

fn reject_indirection(tokens: &proc_macro2::TokenStream) -> syn::Result<()> {
    let mut trees = tokens.clone().into_iter();
    if let Some(TokenTree::Literal(literal)) = trees.next() {
        return Err(deferred(literal.span(), "path/string arguments"));
    }

    if let Some(span) = include_span(tokens.clone()) {
        return Err(deferred(span, "include!"));
    }

    if syn::parse2::<syn::Path>(tokens.clone()).is_ok() {
        return Err(deferred(tokens.span(), "path/string arguments"));
    }
    Ok(())
}

fn include_span(tokens: proc_macro2::TokenStream) -> Option<proc_macro2::Span> {
    let trees: Vec<_> = tokens.into_iter().collect();
    for (index, tree) in trees.iter().enumerate() {
        if let TokenTree::Ident(ident) = tree {
            if ident == "include"
                && matches!(trees.get(index + 1), Some(TokenTree::Punct(punct)) if punct.as_char() == '!')
            {
                return Some(ident.span());
            }
        }
        if let TokenTree::Group(group) = tree {
            if let Some(span) = include_span(group.stream()) {
                return Some(span);
            }
        }
    }
    None
}

fn deferred(span: proc_macro2::Span, construct: &str) -> syn::Error {
    syn::Error::new(span, format!("{construct} unimplemented in MVP"))
}

#[cfg(test)]
mod tests {
    fn extract(source: &str) -> syn::Result<crate::Invocation> {
        crate::invocation_from_source(source)
    }

    #[test]
    fn extracts_the_only_direct_grammar_wide_invocation() {
        let invocation = extract(
            r#"
                use something::Useful;
                constructions! {
                    vocab Pronoun { You = "you", }
                    root Ability { punctuation = "."; eoi = true; standalone_render = true; }
                }
            "#,
        )
        .expect("one direct invocation is accepted");
        let declarations = crate::parse_declarations(invocation.tokens)
            .expect("extracted tokens are the declaration stream");
        assert_eq!(declarations.declarations.len(), 2);
    }

    #[test]
    fn rejects_multiple_and_separate_tier_invocations() {
        let multiple =
            extract("constructions! {} constructions! {}").expect_err("second invocation rejected");
        assert!(multiple.to_string().contains("exactly one"));

        for name in ["vocab", "lexeme"] {
            let error = extract(&format!("{name}! {{ Anything }}"))
                .expect_err("separate tier macro rejected")
                .to_string();
            assert!(error.contains(name), "{name}: {error}");
            assert!(error.contains("unimplemented in MVP"), "{name}: {error}");
        }
    }

    #[test]
    fn rejects_indirect_arguments_and_includes() {
        let cases = [
            (
                "constructions!(\"declarations.rs\");",
                "path/string arguments",
            ),
            (
                "constructions!(crate::declarations);",
                "path/string arguments",
            ),
            ("constructions!(include!(\"declarations.rs\"));", "include!"),
            ("include!(\"declarations.rs\");", "include!"),
        ];

        for (source, construct) in cases {
            let error = extract(source)
                .expect_err("indirection rejected")
                .to_string();
            assert!(error.contains(construct), "{construct}: {error}");
            assert!(
                error.contains("unimplemented in MVP"),
                "{construct}: {error}"
            );
        }
    }

    #[test]
    fn source_wide_fence_rejects_nested_second_invocation_and_deep_include() {
        let nested = extract(
            r#"
                constructions! { vocab Pronoun { You = "you", } }
                mod nested {
                    constructions! { lexeme VerbLexeme { Be, } }
                }
            "#,
        )
        .expect_err("an inline-module invocation is still a second source-wide invocation")
        .to_string();
        assert!(nested.contains("exactly one"), "{nested}");

        let include = extract(
            r#"
                constructions! {
                    vocab Pronoun { You = "you", }
                    wrapper { deeper(include!("declarations.rs")) }
                }
            "#,
        )
        .expect_err("include is fenced recursively after valid declaration tokens")
        .to_string();
        assert!(include.contains("include!"), "{include}");
        assert!(include.contains("unimplemented in MVP"), "{include}");
    }
}
