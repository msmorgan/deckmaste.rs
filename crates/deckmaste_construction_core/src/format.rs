use crate::EmissionPlan;
use crate::GeneratedItem;

pub(crate) fn validate_item(item: &GeneratedItem) -> syn::Result<()> {
    let _ = parse_one_item(item)?;
    Ok(())
}

pub(crate) fn format_plan(plan: &EmissionPlan) -> syn::Result<String> {
    let mut formatted = String::new();
    for item in plan.items() {
        formatted.push_str(&format_item(item)?);
    }
    Ok(formatted)
}

pub(crate) fn format_item(item: &GeneratedItem) -> syn::Result<String> {
    Ok(prettyplease::unparse(&parse_one_item(item)?))
}

fn parse_one_item(item: &GeneratedItem) -> syn::Result<syn::File> {
    let file = syn::parse2::<syn::File>(item.tokens.clone())?;
    if file.items.len() != 1 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "generated item {:?} must contain exactly one Rust item",
                item.key
            ),
        ));
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    use crate::GeneratedItem;
    use crate::ItemKey;
    use crate::NamedKind;

    #[test]
    fn public_item_formatter_rejects_zero_and_two_rust_items() {
        let expected = "generated item Named { kind: Type, name: \"Broken\" } must contain exactly one Rust item";
        for tokens in [TokenStream::new(), quote! { struct One; struct Two; }] {
            let item = GeneratedItem {
                key: ItemKey::Named {
                    kind: NamedKind::Type,
                    name: "Broken".to_owned(),
                },
                tokens,
                origins: Vec::new(),
            };
            let error = crate::format_generated_item(&item)
                .expect_err("non-single-item public input is rejected");
            assert_eq!(error.to_string(), expected);
        }
    }
}
