//! DSL parser: `constructicon!` token stream → declaration IR. Syntax-only
//! and fail-fast: anything semantic (duplicate ordinals, bad paths, stratum
//! misuse) is deliberately left to the validator so the macro can report
//! every error at once with EC codes.

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::model::AstShape;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::DominanceEdge;
use crate::model::ElementDeclaration;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::FieldPath;
use crate::model::FormDeclaration;
use crate::model::GroupDeclaration;
use crate::model::Predicate;
use crate::model::SelectionPromise;
use crate::model::Spanned;
use crate::model::SurfaceAtom;
use crate::model::WitnessClass;
use crate::model::WitnessDeclaration;

mod kw {
    syn::custom_keyword!(group);
    syn::custom_keyword!(element);
    syn::custom_keyword!(construction);
    syn::custom_keyword!(internal);
    syn::custom_keyword!(bind);
    syn::custom_keyword!(own);
    syn::custom_keyword!(require);
    syn::custom_keyword!(derive);
    syn::custom_keyword!(witness);
    syn::custom_keyword!(stored);
    syn::custom_keyword!(derived);
    syn::custom_keyword!(free);
    syn::custom_keyword!(form);
    syn::custom_keyword!(when);
    syn::custom_keyword!(dominates);
    syn::custom_keyword!(selection);
    syn::custom_keyword!(packed);
    syn::custom_keyword!(unique);
    syn::custom_keyword!(deserialize);
    syn::custom_keyword!(opt);
    syn::custom_keyword!(hole);
    syn::custom_keyword!(lex);
    syn::custom_keyword!(seq);
    syn::custom_keyword!(all);
    syn::custom_keyword!(any);
}

pub fn parse_group(input: proc_macro2::TokenStream) -> syn::Result<GroupDeclaration> {
    syn::parse2::<GroupSyntax>(input).map(|g| g.0)
}

struct GroupSyntax(GroupDeclaration);

impl Parse for GroupSyntax {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<kw::group>()?;
        let name = spanned_ident(input)?;
        input.parse::<syn::Token![;]>()?;
        let mut elements = Vec::new();
        let mut constructions = Vec::new();
        while !input.is_empty() {
            if input.peek(kw::element) {
                elements.push(parse_element(input)?);
            } else {
                constructions.push(parse_construction(input)?);
            }
        }
        Ok(Self(GroupDeclaration {
            name,
            constructions,
            elements,
        }))
    }
}

fn spanned_ident(input: ParseStream<'_>) -> syn::Result<Spanned<String>> {
    let ident: syn::Ident = input.parse()?;
    Ok(Spanned {
        value: ident.to_string(),
        span: ident.span(),
    })
}

fn parse_element(input: ParseStream<'_>) -> syn::Result<ElementDeclaration> {
    input.parse::<kw::element>()?;
    let name = spanned_ident(input)?;
    let content;
    syn::braced!(content in input);
    let fields = parse_fields(&content)?;
    Ok(ElementDeclaration { name, fields })
}

fn parse_fields(input: ParseStream<'_>) -> syn::Result<Vec<FieldBinding>> {
    let mut fields = Vec::new();
    while !input.is_empty() {
        let field = spanned_ident(input)?;
        input.parse::<syn::Token![:]>()?;
        let kind = parse_kind(input)?;
        fields.push(FieldBinding { field, kind });
        if input.is_empty() {
            break;
        }
        input.parse::<syn::Token![,]>()?;
    }
    Ok(fields)
}

fn parse_kind(input: ParseStream<'_>) -> syn::Result<FieldKind> {
    if input.peek(kw::opt) {
        let opt = input.parse::<kw::opt>()?;
        if input.peek(kw::opt) {
            return Err(syn::Error::new(
                opt.span,
                "opt applies once; `opt opt` is not a kind",
            ));
        }
        let inner = parse_kind(input)?;
        return Ok(FieldKind::Optional {
            inner: Box::new(inner),
        });
    }
    if input.peek(kw::hole) {
        input.parse::<kw::hole>()?;
        let boxed = input.peek(syn::Token![box]);
        if boxed {
            input.parse::<syn::Token![box]>()?;
        }
        let category = spanned_type_path(input)?;
        return Ok(FieldKind::Subtree { category, boxed });
    }
    if input.peek(kw::lex) {
        input.parse::<kw::lex>()?;
        let codec = spanned_type_path(input)?;
        return Ok(FieldKind::Scalar { codec });
    }
    if input.peek(kw::seq) {
        input.parse::<kw::seq>()?;
        let element = spanned_ident(input)?;
        return Ok(FieldKind::Sequence { element });
    }
    Err(input.error("expected a field kind: opt / hole / lex / seq"))
}

fn spanned_type_path(input: ParseStream<'_>) -> syn::Result<Spanned<String>> {
    let path: syn::Path = input.parse()?;
    let rendered = path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");
    let prefixed = if path.leading_colon.is_some() { format!("::{rendered}") } else { rendered };
    Ok(Spanned {
        value: prefixed,
        span: path
            .segments
            .last()
            .map_or_else(proc_macro2::Span::call_site, |s| s.ident.span()),
    })
}

fn parse_path(input: ParseStream<'_>) -> syn::Result<FieldPath> {
    let first: syn::Ident = input.parse()?;
    let whole_span = first.span();
    let mut segments = vec![Spanned {
        value: first.to_string(),
        span: first.span(),
    }];
    // A dot continues the path UNLESS it starts a predicate method
    // (.len() / .is_some() / .is_none()) — the caller peeks for those.
    while input.peek(syn::Token![.]) && input.peek2(syn::Ident) && !peek_method(input) {
        input.parse::<syn::Token![.]>()?;
        let segment: syn::Ident = input.parse()?;
        segments.push(Spanned {
            value: segment.to_string(),
            span: segment.span(),
        });
    }
    Ok(FieldPath {
        segments,
        span: whole_span,
    })
}

fn peek_method(input: ParseStream<'_>) -> bool {
    // fork() lets us look past the dot without consuming.
    let fork = input.fork();
    if fork.parse::<syn::Token![.]>().is_err() {
        return false;
    }
    let Ok(ident) = fork.parse::<syn::Ident>() else { return false };
    matches!(ident.to_string().as_str(), "len" | "is_some" | "is_none")
        && fork.peek(syn::token::Paren)
}

fn parse_construction(input: ParseStream<'_>) -> syn::Result<ConstructionDeclaration> {
    let internal = input.peek(kw::internal);
    if internal {
        input.parse::<kw::internal>()?;
    }
    input.parse::<kw::construction>()?;
    let id = spanned_ident(input)?;
    input.parse::<syn::Token![:]>()?;
    let category = spanned_ident(input)?;
    let content;
    syn::braced!(content in input);
    let ast = parse_shape(&content)?;
    let mut constraints = Vec::new();
    let mut witnesses = Vec::new();
    let mut forms = Vec::new();
    let mut dominance = Vec::new();
    let mut selection = SelectionPromise::Packed;
    let mut deserialize = false;
    while !content.is_empty() {
        if content.peek(kw::require) {
            let keyword = content.parse::<kw::require>()?;
            let predicate = parse_pred(&content)?;
            content.parse::<syn::Token![;]>()?;
            constraints.push(Constraint::Require(Spanned {
                value: predicate,
                span: keyword.span,
            }));
        } else if content.peek(kw::derive) {
            content.parse::<kw::derive>()?;
            let target = parse_path(&content)?;
            content.parse::<syn::Token![=]>()?;
            let combinator = spanned_ident(&content)?;
            let args_content;
            syn::parenthesized!(args_content in content);
            let args = parse_path_list(&args_content)?;
            content.parse::<syn::Token![;]>()?;
            constraints.push(Constraint::DeriveFeature {
                target,
                combinator,
                args,
            });
        } else if content.peek(kw::witness) {
            content.parse::<kw::witness>()?;
            let name = spanned_ident(&content)?;
            content.parse::<syn::Token![=]>()?;
            let class = parse_witness_class(&content)?;
            content.parse::<syn::Token![;]>()?;
            witnesses.push(WitnessDeclaration { name, class });
        } else if content.peek(kw::form) {
            forms.push(parse_form(&content)?);
        } else if content.peek(kw::dominates) {
            let keyword = content.parse::<kw::dominates>()?;
            let loser = spanned_ident(&content)?;
            content.parse::<syn::Token![;]>()?;
            dominance.push(DominanceEdge {
                winner: Spanned {
                    value: id.value.clone(),
                    span: keyword.span,
                },
                loser,
            });
        } else if content.peek(kw::selection) {
            content.parse::<kw::selection>()?;
            if content.peek(kw::unique) {
                content.parse::<kw::unique>()?;
                selection = SelectionPromise::Unique;
            } else {
                content.parse::<kw::packed>()?;
                selection = SelectionPromise::Packed;
            }
            content.parse::<syn::Token![;]>()?;
        } else if content.peek(kw::deserialize) {
            content.parse::<kw::deserialize>()?;
            content.parse::<syn::Token![;]>()?;
            deserialize = true;
        } else {
            return Err(content.error(
                "expected require / derive / witness / form / dominates / selection / deserialize",
            ));
        }
    }
    Ok(ConstructionDeclaration {
        id,
        category,
        internal,
        ast,
        constraints,
        witnesses,
        forms,
        dominance,
        selection,
        deserialize,
    })
}

fn parse_shape(input: ParseStream<'_>) -> syn::Result<AstShape> {
    if input.peek(kw::bind) {
        input.parse::<kw::bind>()?;
        let path = spanned_type_path(input)?;
        let content;
        syn::braced!(content in input);
        let fields = parse_fields(&content)?;
        return Ok(AstShape::Bind { path, fields });
    }
    input.parse::<kw::own>()?;
    let name = spanned_ident(input)?;
    let content;
    syn::braced!(content in input);
    let fields = parse_fields(&content)?;
    Ok(AstShape::Own { name, fields })
}

fn parse_witness_class(input: ParseStream<'_>) -> syn::Result<WitnessClass> {
    if input.peek(kw::stored) {
        input.parse::<kw::stored>()?;
        return Ok(WitnessClass::Stored {
            path: parse_path(input)?,
        });
    }
    if input.peek(kw::derived) {
        input.parse::<kw::derived>()?;
        let combinator = spanned_ident(input)?;
        let args_content;
        syn::parenthesized!(args_content in input);
        let args = parse_path_list(&args_content)?;
        return Ok(WitnessClass::Derived { combinator, args });
    }
    input.parse::<kw::free>()?;
    Ok(WitnessClass::Free {
        ty: spanned_type_path(input)?,
    })
}

fn parse_path_list(input: ParseStream<'_>) -> syn::Result<Vec<FieldPath>> {
    let mut paths = vec![parse_path(input)?];
    while input.peek(syn::Token![,]) {
        input.parse::<syn::Token![,]>()?;
        paths.push(parse_path(input)?);
    }
    Ok(paths)
}

fn parse_form(input: ParseStream<'_>) -> syn::Result<FormDeclaration> {
    input.parse::<kw::form>()?;
    let name = spanned_ident(input)?;
    input.parse::<syn::Token![@]>()?;
    let ordinal_lit: syn::LitInt = input.parse()?;
    let ordinal = Spanned {
        value: ordinal_lit.base10_parse::<u16>()?,
        span: ordinal_lit.span(),
    };
    let guard = if input.peek(kw::when) {
        let keyword = input.parse::<kw::when>()?;
        Some(Spanned {
            value: parse_pred(input)?,
            span: keyword.span,
        })
    } else {
        None
    };
    input.parse::<syn::Token![=]>()?;
    let mut surface = Vec::new();
    while !input.peek(syn::Token![;]) {
        surface.push(parse_atom(input)?);
    }
    input.parse::<syn::Token![;]>()?;
    Ok(FormDeclaration {
        name,
        ordinal,
        surface,
        guard,
    })
}

fn parse_atom(input: ParseStream<'_>) -> syn::Result<SurfaceAtom> {
    if input.peek(syn::LitStr) {
        let lit: syn::LitStr = input.parse()?;
        return Ok(SurfaceAtom::Literal(Spanned {
            value: lit.value(),
            span: lit.span(),
        }));
    }
    // `lex(path)` renders a scalar; a bare field named `lex` (no parens)
    // still parses as an ordinary hole path.
    if input.peek(kw::lex) && input.peek2(syn::token::Paren) {
        input.parse::<kw::lex>()?;
        let content;
        syn::parenthesized!(content in input);
        return Ok(SurfaceAtom::Lexeme(parse_path(&content)?));
    }
    Ok(SurfaceAtom::Hole(parse_path(input)?))
}

fn parse_pred(input: ParseStream<'_>) -> syn::Result<Predicate> {
    if input.peek(kw::all) && input.peek2(syn::token::Paren) {
        input.parse::<kw::all>()?;
        let content;
        syn::parenthesized!(content in input);
        return Ok(Predicate::All(parse_pred_list(&content)?));
    }
    if input.peek(kw::any) && input.peek2(syn::token::Paren) {
        input.parse::<kw::any>()?;
        let content;
        syn::parenthesized!(content in input);
        return Ok(Predicate::Any(parse_pred_list(&content)?));
    }
    let path = parse_path(input)?;
    if input.peek(syn::Token![.]) {
        // parse_path stopped here because a predicate method follows.
        input.parse::<syn::Token![.]>()?;
        let method: syn::Ident = input.parse()?;
        let empty;
        syn::parenthesized!(empty in input);
        if !empty.is_empty() {
            return Err(empty.error("predicate methods take no arguments"));
        }
        return match method.to_string().as_str() {
            "len" => {
                if input.peek(syn::Token![>=]) {
                    input.parse::<syn::Token![>=]>()?;
                    let min: syn::LitInt = input.parse()?;
                    Ok(Predicate::LenAtLeast {
                        path,
                        min: min.base10_parse()?,
                    })
                } else {
                    input.parse::<syn::Token![==]>()?;
                    let len: syn::LitInt = input.parse()?;
                    Ok(Predicate::LenIs {
                        path,
                        len: len.base10_parse()?,
                    })
                }
            }
            "is_some" => Ok(Predicate::IsSome { path }),
            "is_none" => Ok(Predicate::IsNone { path }),
            other => Err(syn::Error::new(
                method.span(),
                format!("unknown predicate method `{other}`; expected len / is_some / is_none"),
            )),
        };
    }
    input.parse::<syn::Token![in]>()?;
    let content;
    syn::bracketed!(content in input);
    let mut allowed = Vec::new();
    loop {
        let variant: syn::Ident = content.parse()?;
        allowed.push(variant.to_string());
        if content.is_empty() {
            break;
        }
        content.parse::<syn::Token![,]>()?;
    }
    Ok(Predicate::In { path, allowed })
}

fn parse_pred_list(input: ParseStream<'_>) -> syn::Result<Vec<Predicate>> {
    let mut preds = vec![parse_pred(input)?];
    while input.peek(syn::Token![,]) {
        input.parse::<syn::Token![,]>()?;
        preds.push(parse_pred(input)?);
    }
    Ok(preds)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The full fixture family — byte-identical to the DSL text in the plan,
    /// to `tests/golden_real.rs`'s `fixture_family_parses_to_the_handbuilt_ir`,
    /// and to Task 13's `constructicon!` invocation.
    fn fixture_dsl() -> proc_macro2::TokenStream {
        quote::quote! {
            group fixture_coordination;

            element fixture_member {
                comma: opt lex Comma,
                phrase: hole FixturePhrase,
            }

            construction fixture_pair: FixturePair {
                own FixturePairNode {
                    members: seq fixture_member,
                    conjunction: lex Conjunction,
                }
                require members.len() >= 2;
                require conjunction in [And, Or];
                witness oxford = stored members.last.comma;
                form plain @ 0 when conjunction in [And] = members lex(conjunction);
                form fancy @ 1 when conjunction in [Or] = members "," lex(conjunction);
                dominates fixture_solo;
            }

            construction fixture_solo: FixturePair {
                own FixtureSoloNode {
                    phrase: hole FixturePhrase,
                    alt: opt hole FixturePhrase,
                }
                require alt.is_none();
                witness gap = free Comma;
                form only @ 0 = phrase alt;
                selection unique;
                deserialize;
            }
        }
    }

    #[test]
    fn stored_witness_path_has_one_segment_per_source_token() {
        let parsed = parse_group(fixture_dsl()).expect("fixture DSL parses");
        let witness_path = match &parsed.constructions[0].witnesses[0].class {
            WitnessClass::Stored { path } => path,
            other => panic!("expected stored witness, got {other:?}"),
        };
        let segments: Vec<&str> = witness_path
            .segments
            .iter()
            .map(|s| s.value.as_str())
            .collect();
        assert_eq!(segments, vec!["members", "last", "comma"]);
        // Span FIDELITY (each segment's span sits on its own source token)
        // cannot be asserted here: quote!-built streams carry only call_site
        // spans. The authoritative span gates are Task 12's compile-fail
        // fixtures 4-5, where rustc renders the caret.
    }

    #[test]
    fn opt_opt_is_a_parse_error() {
        let err = parse_group(quote::quote! {
            group g;
            construction c: Cat {
                own N { f: opt opt lex Comma, }
                require f.is_none();
                form only @ 0 = lex(f);
            }
        })
        .expect_err("opt opt must not parse");
        assert!(
            err.to_string().contains("opt"),
            "error names the construct: {err}"
        );
    }

    #[test]
    fn duplicate_ordinal_still_parses_and_validates_to_ec002() {
        // The parser is syntax-only; semantic rejection stays the
        // validator's, so the macro pipeline reports ALL errors at once.
        let mut parsed = parse_group(fixture_dsl()).expect("parses");
        parsed.constructions[0].forms[1].ordinal = crate::model::Spanned::call_site(0);
        let err = crate::validate::validate(&parsed).expect_err("EC002");
        let codes: Vec<&str> = err.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes, vec!["EC002"]);
    }
}
