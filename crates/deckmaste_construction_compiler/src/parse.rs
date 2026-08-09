//! DSL parser: `constructions!` token stream → declaration IR. Syntax-only
//! and fail-fast: anything semantic (duplicate ordinals, bad paths, stratum
//! misuse) is deliberately left to the validator so the macro can report
//! every error at once with EC codes.

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::model::AstShape;
use crate::model::BindAdapter;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::DominanceEdge;
use crate::model::ElementDeclaration;
use crate::model::ElementVariantDeclaration;
use crate::model::EvidenceDeclaration;
use crate::model::EvidenceKind;
use crate::model::EvidenceSource;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::FieldPath;
use crate::model::FormDeclaration;
use crate::model::GroupDeclaration;
use crate::model::LensApplication;
use crate::model::LensDeclaration;
use crate::model::LensEdit;
use crate::model::LensEditKind;
use crate::model::LensFieldDeclaration;
use crate::model::LensFieldKind;
use crate::model::Predicate;
use crate::model::SelectionPromise;
use crate::model::Spanned;
use crate::model::SurfaceAtom;
use crate::model::WitnessClass;
use crate::model::WitnessDeclaration;

mod kw {
    syn::custom_keyword!(group);
    syn::custom_keyword!(element);
    syn::custom_keyword!(lens);
    syn::custom_keyword!(variant);
    syn::custom_keyword!(construction);
    syn::custom_keyword!(internal);
    syn::custom_keyword!(bind);
    syn::custom_keyword!(via);
    syn::custom_keyword!(own);
    syn::custom_keyword!(project);
    syn::custom_keyword!(require);
    syn::custom_keyword!(recognize);
    syn::custom_keyword!(derive);
    syn::custom_keyword!(evidence);
    syn::custom_keyword!(guard);
    syn::custom_keyword!(feature);
    syn::custom_keyword!(role);
    syn::custom_keyword!(requirement);
    syn::custom_keyword!(output);
    syn::custom_keyword!(field);
    syn::custom_keyword!(category);
    syn::custom_keyword!(witness);
    syn::custom_keyword!(stored);
    syn::custom_keyword!(derived);
    syn::custom_keyword!(free);
    syn::custom_keyword!(form);
    syn::custom_keyword!(when);
    syn::custom_keyword!(check);
    syn::custom_keyword!(inverse);
    syn::custom_keyword!(otherwise);
    syn::custom_keyword!(dominates);
    syn::custom_keyword!(dominated);
    syn::custom_keyword!(by);
    syn::custom_keyword!(selection);
    syn::custom_keyword!(packed);
    syn::custom_keyword!(unique);
    syn::custom_keyword!(deserialize);
    syn::custom_keyword!(opt);
    syn::custom_keyword!(hole);
    syn::custom_keyword!(lex);
    syn::custom_keyword!(identity);
    syn::custom_keyword!(surface);
    syn::custom_keyword!(seq);
    syn::custom_keyword!(all);
    syn::custom_keyword!(any);
    syn::custom_keyword!(value);
    syn::custom_keyword!(vec);
    syn::custom_keyword!(from);
    syn::custom_keyword!(focus);
    syn::custom_keyword!(with);
    syn::custom_keyword!(prepend);
    syn::custom_keyword!(append);
}

/// # Errors
///
/// Returns a [`syn::Error`] when `input` does not parse as the
/// `constructions!` DSL syntax (unexpected tokens, missing punctuation, an
/// unrecognized keyword, and so on). Semantic problems — duplicate
/// ordinals, bad paths, stratum misuse — are not caught here; see
/// [`crate::validate::validate`].
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
        let mut lenses = Vec::new();
        let mut constructions = Vec::new();
        while !input.is_empty() {
            if input.peek(kw::element) {
                elements.push(parse_element(input)?);
            } else if input.peek(kw::lens) {
                lenses.push(parse_lens_declaration(input)?);
            } else {
                constructions.push(parse_construction(input)?);
            }
        }
        Ok(Self(GroupDeclaration {
            name,
            constructions,
            elements,
            lenses,
        }))
    }
}

fn parse_lens_declaration(input: ParseStream<'_>) -> syn::Result<LensDeclaration> {
    input.parse::<kw::lens>()?;
    let name = spanned_ident(input)?;
    input.parse::<kw::bind>()?;
    let owner_type = spanned_type_path(input)?;
    let adapter = if input.peek(kw::via) {
        input.parse::<kw::via>()?;
        let constructor = spanned_type_path(input)?;
        input.parse::<syn::Token![,]>()?;
        let destructurer = spanned_type_path(input)?;
        Some(BindAdapter {
            constructor,
            destructurer,
        })
    } else {
        None
    };
    let content;
    syn::braced!(content in input);
    let mut fields = Vec::new();
    while !content.is_empty() {
        let name = spanned_ident(&content)?;
        content.parse::<syn::Token![:]>()?;
        let kind = if content.peek(kw::value) {
            content.parse::<kw::value>()?;
            LensFieldKind::Value {
                value_type: spanned_type_path(&content)?,
            }
        } else if content.peek(kw::opt) {
            content.parse::<kw::opt>()?;
            LensFieldKind::Optional {
                value_type: spanned_type_path(&content)?,
            }
        } else if content.peek(kw::vec) {
            content.parse::<kw::vec>()?;
            LensFieldKind::Vector {
                element_type: spanned_type_path(&content)?,
            }
        } else {
            return Err(content.error("expected a lens field kind: value / opt / vec"));
        };
        fields.push(LensFieldDeclaration { name, kind });
        if content.is_empty() {
            break;
        }
        content.parse::<syn::Token![,]>()?;
    }
    Ok(LensDeclaration {
        name,
        owner_type,
        adapter,
        fields,
    })
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
    let bind_path = if input.peek(kw::bind) {
        input.parse::<kw::bind>()?;
        Some(spanned_type_path(input)?)
    } else {
        None
    };
    let content;
    syn::braced!(content in input);
    let (fields, variants) = parse_element_members(&content)?;
    Ok(ElementDeclaration {
        name,
        bind_path,
        fields,
        variants,
    })
}

fn parse_element_members(
    input: ParseStream<'_>,
) -> syn::Result<(Vec<FieldBinding>, Vec<ElementVariantDeclaration>)> {
    let mut fields = Vec::new();
    let mut variants = Vec::new();
    while !input.is_empty() {
        if input.peek(kw::variant) {
            input.parse::<kw::variant>()?;
            let name = spanned_ident(input)?;
            input.parse::<syn::Token![:]>()?;
            let payload = parse_kind(input)?;
            variants.push(ElementVariantDeclaration { name, payload });
        } else {
            let field = spanned_ident(input)?;
            input.parse::<syn::Token![:]>()?;
            let kind = parse_kind(input)?;
            fields.push(FieldBinding { field, kind });
        }
        if input.is_empty() {
            break;
        }
        input.parse::<syn::Token![,]>()?;
    }
    Ok((fields, variants))
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
    if input.peek(kw::identity) {
        input.parse::<kw::identity>()?;
        let value_type = spanned_type_path(input)?;
        input.parse::<kw::via>()?;
        let provider = spanned_type_path(input)?;
        return Ok(FieldKind::Identity {
            value_type,
            provider,
        });
    }
    if input.peek(kw::surface) {
        input.parse::<kw::surface>()?;
        input.parse::<kw::lex>()?;
        let codec = spanned_type_path(input)?;
        return Ok(FieldKind::SurfaceScalar { codec });
    }
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
        let value_type = spanned_type_path(input)?;
        if input.peek(kw::via) {
            input.parse::<kw::via>()?;
            let codec = spanned_type_path(input)?;
            return Ok(FieldKind::TypedScalar { value_type, codec });
        }
        return Ok(FieldKind::Scalar { codec: value_type });
    }
    if input.peek(kw::seq) {
        input.parse::<kw::seq>()?;
        let element = spanned_ident(input)?;
        return Ok(FieldKind::Sequence { element });
    }
    Err(input.error("expected a field kind: identity / opt / hole / lex / surface lex / seq"))
}

fn spanned_type_path(input: ParseStream<'_>) -> syn::Result<Spanned<String>> {
    let path: syn::Path = input.parse()?;
    let span = path
        .segments
        .last()
        .map_or_else(proc_macro2::Span::call_site, |s| s.ident.span());
    // Render through `ToTokens` rather than hand-joining segment idents: a
    // manual `s.ident.to_string()` join silently drops any angle-bracketed
    // generic arguments a segment carries (`syn::Path` parses `Wrapper<Inner>`
    // just fine, but only the bare idents would survive the join), and the
    // dropped-generics failure is silent downstream too: `emit.rs`'s
    // `parse_type` reparses this string as a `syn::Type` via `syn::parse_str`,
    // and a bare path like `Wrapper` is itself a valid (wrong) type, so no
    // parse error ever fires.
    //
    // `quote!` inserts a space around every rendered token, so the raw
    // `to_string()` would turn `crate::Foo` into `"crate :: Foo"`. Stripping
    // whitespace afterward recovers the exact compact source spelling: a
    // `syn::Path` grammar (segments, `::`, angle-bracketed generic args) never
    // places two bare identifiers or keywords adjacent with nothing between
    // them, so removing whitespace can never fuse two tokens into one. That
    // keeps diagnostics that quote a qualified codec/category path readable,
    // and keeps `stratum_of`'s terminal-ident `rsplit("::")` working exactly
    // as it did before this path could carry generics at all.
    let rendered: String = quote::quote!(#path)
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    Ok(Spanned {
        value: rendered,
        span,
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

#[allow(
    clippy::too_many_lines,
    reason = "the construction body keeps its closed statement grammar in one dispatch loop"
)]
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
    let (ast, bind_adapter) = parse_shape(&content)?;
    let mut projection = None;
    let mut lens = None;
    let mut constraints = Vec::new();
    let mut evidence = None;
    let mut witnesses = Vec::new();
    let mut forms = Vec::new();
    let mut dominance = Vec::new();
    let mut selection = SelectionPromise::Packed;
    let mut deserialize = false;
    while !content.is_empty() {
        if content.peek(kw::project) {
            content.parse::<kw::project>()?;
            let variant = spanned_ident(&content)?;
            content.parse::<syn::Token![;]>()?;
            if projection.is_some() {
                return Err(syn::Error::new(
                    variant.span,
                    "project may be declared at most once",
                ));
            }
            projection = Some(variant);
        } else if content.peek(kw::lens) {
            let application = parse_lens_application(&content)?;
            if lens.is_some() {
                return Err(syn::Error::new(
                    application.owner.span,
                    "lens may be declared at most once",
                ));
            }
            lens = Some(application);
        } else if content.peek(kw::recognize) {
            content.parse::<kw::recognize>()?;
            let keyword = content.parse::<kw::require>()?;
            let predicate = parse_pred(&content)?;
            content.parse::<syn::Token![;]>()?;
            constraints.push(Constraint::Recognize(Spanned {
                value: predicate,
                span: keyword.span,
            }));
        } else if content.peek(kw::require) {
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
            let feature_type = if content.peek(syn::Token![:]) {
                content.parse::<syn::Token![:]>()?;
                Some(spanned_type_path(&content)?)
            } else {
                None
            };
            content.parse::<syn::Token![=]>()?;
            let combinator = spanned_ident(&content)?;
            let args_content;
            syn::parenthesized!(args_content in content);
            let args = parse_path_list(&args_content)?;
            content.parse::<syn::Token![;]>()?;
            constraints.push(Constraint::DeriveFeature {
                target,
                feature_type,
                combinator,
                args,
            });
        } else if content.peek(kw::evidence) {
            let declaration = parse_evidence(&content)?;
            if evidence.is_some() {
                return Err(syn::Error::new(
                    declaration.label.span,
                    "evidence may be declared at most once",
                ));
            }
            evidence = Some(declaration);
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
        } else if content.peek(kw::dominated) {
            let keyword = content.parse::<kw::dominated>()?;
            content.parse::<kw::by>()?;
            let winner = spanned_ident(&content)?;
            content.parse::<syn::Token![;]>()?;
            dominance.push(DominanceEdge {
                winner,
                loser: Spanned {
                    value: id.value.clone(),
                    span: keyword.span,
                },
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
                "expected project / lens / recognize require / require / derive / evidence / witness / form / dominates / dominated by / selection / deserialize",
            ));
        }
    }
    Ok(ConstructionDeclaration {
        id,
        category,
        internal,
        ast,
        bind_adapter,
        lens,
        projection,
        constraints,
        evidence,
        witnesses,
        forms,
        dominance,
        selection,
        deserialize,
    })
}

fn parse_evidence(input: ParseStream<'_>) -> syn::Result<EvidenceDeclaration> {
    input.parse::<kw::evidence>()?;
    let kind = if input.peek(kw::guard) {
        input.parse::<kw::guard>()?;
        EvidenceKind::Guard
    } else if input.peek(kw::feature) {
        input.parse::<kw::feature>()?;
        EvidenceKind::Feature
    } else if input.peek(kw::role) {
        input.parse::<kw::role>()?;
        EvidenceKind::Role
    } else {
        return Err(input.error("expected guard / feature / role evidence kind"));
    };
    let label = input.parse::<syn::LitStr>()?;
    let label = Spanned {
        value: label.value(),
        span: label.span(),
    };
    input.parse::<kw::from>()?;
    let source = if input.peek(kw::requirement) {
        input.parse::<kw::requirement>()?;
        EvidenceSource::Requirement(parse_path(input)?)
    } else if input.peek(kw::output) {
        input.parse::<kw::output>()?;
        EvidenceSource::Output(parse_path(input)?)
    } else if input.peek(kw::field) {
        input.parse::<kw::field>()?;
        EvidenceSource::Field(parse_path(input)?)
    } else if input.peek(kw::category) {
        input.parse::<kw::category>()?;
        EvidenceSource::Category
    } else {
        return Err(input.error("expected requirement / output / field / category evidence source"));
    };
    input.parse::<syn::Token![;]>()?;
    Ok(EvidenceDeclaration {
        kind,
        label,
        source,
    })
}

fn parse_lens_application(input: ParseStream<'_>) -> syn::Result<LensApplication> {
    input.parse::<kw::lens>()?;
    let owner = spanned_ident(input)?;
    let source = if input.peek(kw::from) {
        input.parse::<kw::from>()?;
        Some(spanned_ident(input)?)
    } else {
        None
    };
    let content;
    syn::braced!(content in input);
    let mut edits = Vec::new();
    while !content.is_empty() {
        let kind = if content.peek(kw::focus) {
            content.parse::<kw::focus>()?;
            LensEditKind::Focus
        } else if content.peek(kw::prepend) {
            content.parse::<kw::prepend>()?;
            LensEditKind::Prepend
        } else if content.peek(kw::append) {
            content.parse::<kw::append>()?;
            LensEditKind::Append
        } else {
            return Err(content.error("expected a lens edit: focus / prepend / append"));
        };
        let target = parse_path(&content)?;
        content.parse::<kw::with>()?;
        let value = spanned_ident(&content)?;
        let adapter = if content.peek(kw::via) {
            content.parse::<kw::via>()?;
            let constructor = spanned_type_path(&content)?;
            content.parse::<syn::Token![,]>()?;
            let destructurer = spanned_type_path(&content)?;
            Some(BindAdapter {
                constructor,
                destructurer,
            })
        } else {
            None
        };
        content.parse::<syn::Token![;]>()?;
        edits.push(LensEdit {
            target,
            value,
            kind,
            adapter,
        });
    }
    Ok(LensApplication {
        owner,
        source,
        edits,
    })
}

fn parse_shape(input: ParseStream<'_>) -> syn::Result<(AstShape, Option<BindAdapter>)> {
    if input.peek(kw::bind) {
        input.parse::<kw::bind>()?;
        let path = spanned_type_path(input)?;
        let adapter = if input.peek(kw::via) {
            input.parse::<kw::via>()?;
            let constructor = spanned_type_path(input)?;
            input.parse::<syn::Token![,]>()?;
            let destructurer = spanned_type_path(input)?;
            Some(BindAdapter {
                constructor,
                destructurer,
            })
        } else {
            None
        };
        let content;
        syn::braced!(content in input);
        let fields = parse_fields(&content)?;
        return Ok((AstShape::Bind { path, fields }, adapter));
    }
    input.parse::<kw::own>()?;
    let name = spanned_ident(input)?;
    let content;
    syn::braced!(content in input);
    let fields = parse_fields(&content)?;
    Ok((AstShape::Own { name, fields }, None))
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
    if input.is_empty() {
        return Ok(Vec::new());
    }
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
    let (guard, value_guard) = if input.peek(kw::when) {
        let keyword = input.parse::<kw::when>()?;
        if input.peek(kw::check) {
            input.parse::<kw::check>()?;
            let content;
            syn::parenthesized!(content in input);
            (None, Some(spanned_type_path(&content)?))
        } else {
            let guard = Some(Spanned {
                value: parse_pred(input)?,
                span: keyword.span,
            });
            let value_guard = if input.peek(kw::check) {
                input.parse::<kw::check>()?;
                let content;
                syn::parenthesized!(content in input);
                Some(spanned_type_path(&content)?)
            } else {
                None
            };
            (guard, value_guard)
        }
    } else {
        (None, None)
    };
    let inverse_guard = if input.peek(kw::inverse) {
        input.parse::<kw::inverse>()?;
        input.parse::<kw::check>()?;
        let content;
        syn::parenthesized!(content in input);
        Some(spanned_type_path(&content)?)
    } else {
        None
    };
    let fallback = if input.peek(kw::otherwise) {
        input.parse::<kw::otherwise>()?;
        true
    } else {
        false
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
        value_guard,
        inverse_guard,
        fallback,
    })
}

fn parse_atom(input: ParseStream<'_>) -> syn::Result<SurfaceAtom> {
    if input.peek(kw::identity) && input.peek2(syn::token::Paren) {
        input.parse::<kw::identity>()?;
        let content;
        syn::parenthesized!(content in input);
        return Ok(SurfaceAtom::Identity(parse_path(&content)?));
    }
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
    /// and to Task 13's `constructions!` invocation.
    fn fixture_dsl() -> proc_macro2::TokenStream {
        quote::quote! {
            group fixture_coordination;

            element fixture_member {
                comma: opt lex Comma,
                phrase: hole FixturePhrase,
            }

            element bound_fixture_member bind BoundMember {
                comma: lex Comma,
                phrase: hole FixturePhrase,
            }

            element bound_variant bind BoundVariant {
                variant Phrase: hole FixturePhrase,
                variant Boxed: hole box FixturePhrase,
            }

            element empty_payload bind BoundPayload {}

            construction fixture_pair: FixturePair {
                own FixturePairNode {
                    members: seq fixture_member,
                    conjunction: lex Conjunction,
                }
                project Pair;
                require members.len() >= 2;
                require conjunction in [And, Or];
                require members.last.comma in [Present];
                witness oxford = stored members.last.comma;
                form plain @ 0 when conjunction in [And] = members lex(conjunction);
                form fancy @ 1 when conjunction in [Or] = members "," lex(conjunction);
                dominates fixture_solo;
                dominated by fixture_outer;
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
    fn evidence_sources_are_preserved_in_declaration_metadata() {
        let parsed = parse_group(quote::quote! {
            group evidence_sources;

            construction guarded: Phrase {
                own Guarded { conjunction: lex Conjunction, }
                require conjunction in [And];
                evidence guard "conjunction gate" from requirement conjunction;
            }
            construction featured: Phrase {
                own Featured { nominal: hole Phrase, }
                evidence feature "attachment phase" from output attachment;
            }
            construction role: Phrase {
                own Role { nominal: hole Phrase, }
                evidence role "chart role" from category;
            }
            construction framed: Phrase {
                own Framed { predicate: hole Predicate, }
                evidence guard "passive frame" from field predicate.frame;
            }
        })
        .expect("all evidence source syntaxes parse");

        let guarded = parsed.constructions[0]
            .evidence
            .as_ref()
            .expect("guard evidence");
        assert_eq!(guarded.kind, EvidenceKind::Guard);
        assert_eq!(guarded.label.value, "conjunction gate");
        assert!(matches!(
            &guarded.source,
            EvidenceSource::Requirement(path) if path.dotted() == "conjunction"
        ));
        assert!(matches!(
            parsed.constructions[1]
                .evidence
                .as_ref()
                .map(|evidence| &evidence.source),
            Some(EvidenceSource::Output(path)) if path.dotted() == "attachment"
        ));
        assert!(matches!(
            parsed.constructions[2]
                .evidence
                .as_ref()
                .map(|evidence| &evidence.source),
            Some(EvidenceSource::Category)
        ));
        assert!(matches!(
            parsed.constructions[3]
                .evidence
                .as_ref()
                .map(|evidence| &evidence.source),
            Some(EvidenceSource::Field(path)) if path.dotted() == "predicate.frame"
        ));
    }

    #[test]
    fn nonfinal_quantifier_is_preserved_as_its_own_path_segment() {
        let parsed = parse_group(quote::quote! {
            group quantified;
            element member { comma: lex Comma, }
            construction list: List {
                own ListNode { members: seq member, }
                require members.nonfinal.comma in [Present];
                form only @ 0 = members;
            }
        })
        .expect("nonfinal uses ordinary dotted-path syntax");
        let Constraint::Require(requirement) = &parsed.constructions[0].constraints[0] else {
            panic!("expected require constraint");
        };
        let Predicate::In { path, allowed } = &requirement.value else {
            panic!("expected In predicate");
        };
        assert_eq!(
            path.segments
                .iter()
                .map(|segment| segment.value.as_str())
                .collect::<Vec<_>>(),
            ["members", "nonfinal", "comma"],
        );
        assert_eq!(
            allowed.iter().map(String::as_str).collect::<Vec<_>>(),
            ["Present"],
        );
    }

    #[test]
    fn surface_scalar_parses_as_a_nonsemantic_element_field() {
        let parsed = parse_group(quote::quote! {
            group surface_fields;
            element member bind Member {
                comma: surface lex Comma,
                phrase: hole Phrase,
            }
        })
        .expect("surface-only scalar syntax parses");
        assert_eq!(
            parsed.elements[0].fields[0].kind,
            FieldKind::SurfaceScalar {
                codec: Spanned::call_site("Comma".to_owned()),
            },
        );
    }

    #[test]
    fn bound_enum_variants_parse_as_typed_element_members() {
        let parsed = parse_group(quote::quote! {
            group typed_elements;
            element nominal_complement bind NominalComplement {
                variant Adjective: hole AdjectivePhrase,
                variant EventClause: hole box IndependentClause,
            }
        })
        .expect("a bound enum element accepts typed variant declarations");
        assert!(
            parsed.elements[0].fields.is_empty(),
            "enum variants are not struct fields"
        );
        assert_eq!(
            parsed.elements[0].variants,
            vec![
                ElementVariantDeclaration {
                    name: Spanned::call_site("Adjective".to_owned()),
                    payload: FieldKind::Subtree {
                        category: Spanned::call_site("AdjectivePhrase".to_owned()),
                        boxed: false,
                    },
                },
                ElementVariantDeclaration {
                    name: Spanned::call_site("EventClause".to_owned()),
                    payload: FieldKind::Subtree {
                        category: Spanned::call_site("IndependentClause".to_owned()),
                        boxed: true,
                    },
                },
            ]
        );
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

    #[test]
    fn generic_typepath_preserves_generic_arguments() {
        // Before the fix, `spanned_type_path` hand-joined `syn::Path`
        // segment idents, silently dropping `<Inner>`. The failure mode was
        // worse than a parse error: `emit.rs`'s `parse_type` reparses the
        // stored string as a `syn::Type`, and a bare path like `Wrapper` is
        // itself a valid (wrong) type, so nothing downstream ever noticed.
        let g = parse_group(quote::quote! {
            group g;
            construction c: Cat {
                own N { f: lex Wrapper<Inner>, }
                form only @ 0 = lex(f);
            }
        })
        .expect("generic TYPEPATH parses");
        let codec = match &g.constructions[0].ast.fields()[0].kind {
            FieldKind::Scalar { codec } | FieldKind::TypedScalar { codec, .. } => codec,
            other => panic!("expected Scalar, got {other:?}"),
        };
        assert_eq!(codec.value, "Wrapper<Inner>");
        // Round-trip through the exact reparse `parse_type` performs, token
        // stream to token stream so the check is whitespace-insensitive.
        let reparsed: syn::Type = syn::parse_str(&codec.value).expect("reparses as a Rust type");
        let expected: syn::Type = syn::parse_str("Wrapper<Inner>").expect("control parses");
        assert_eq!(
            quote::quote!(#reparsed).to_string(),
            quote::quote!(#expected).to_string(),
            "the reparsed type must still be Wrapper<Inner>, generics included"
        );
    }

    #[test]
    fn leading_colon_typepath_round_trips() {
        let g = parse_group(quote::quote! {
            group g;
            construction c: Cat {
                own N { f: lex ::std::num::NonZeroU32, }
                form only @ 0 = lex(f);
            }
        })
        .expect("leading-colon TYPEPATH parses");
        let codec = match &g.constructions[0].ast.fields()[0].kind {
            FieldKind::Scalar { codec } | FieldKind::TypedScalar { codec, .. } => codec,
            other => panic!("expected Scalar, got {other:?}"),
        };
        assert_eq!(codec.value, "::std::num::NonZeroU32");
    }

    #[test]
    fn qualified_bound_element_path_parses_into_the_model_and_round_trips() {
        let parsed = parse_group(quote::quote! {
            group g;
            element member bind crate::syntax::BoundMember {
                phrase: hole FixturePhrase,
            }
        })
        .expect("qualified bound element path parses");
        assert_eq!(
            parsed.elements,
            vec![ElementDeclaration {
                name: Spanned::call_site("member".to_owned()),
                bind_path: Some(Spanned::call_site("crate::syntax::BoundMember".to_owned(),)),
                variants: vec![],
                fields: vec![FieldBinding {
                    field: Spanned::call_site("phrase".to_owned()),
                    kind: FieldKind::Subtree {
                        category: Spanned::call_site("FixturePhrase".to_owned()),
                        boxed: false,
                    },
                }],
            }],
        );
        let stored = &parsed.elements[0].bind_path.as_ref().expect("bound").value;
        let reparsed: syn::Type = syn::parse_str(stored).expect("stored path reparses");
        let expected: syn::Type =
            syn::parse_str("crate::syntax::BoundMember").expect("control parses");
        assert_eq!(
            quote::quote!(#reparsed).to_string(),
            quote::quote!(#expected).to_string(),
            "the stored qualified path must preserve every segment",
        );
    }

    /// Coverage sweep, snippet 1: `hole box TYPEPATH` (nested inside `opt`),
    /// `all(...)`/`any(...)` (recursive predicates), `path.len() == INT`,
    /// `path.is_some()`, a `derived` witness, and an explicit
    /// `selection packed;` — none of these had ever been exercised through
    /// `parse_group` before (only the implicit `Packed` default was).
    #[test]
    fn coverage_sweep_predicates_boxed_hole_derived_witness_explicit_selection() {
        let g = parse_group(quote::quote! {
            group cov1;

            element member {
                tag: lex Tag,
            }

            construction pack: Pack {
                own PackNode {
                    members: seq member,
                    alt: opt hole box BoxedPhrase,
                }
                require all(members.len() == 3, alt.is_some());
                require any(alt.is_some(), members.len() == 3);
                witness picked = derived from_first(members, alt);
                form only @ 0 = members alt;
                selection packed;
            }
        })
        .expect("coverage-sweep snippet 1 parses");
        let construction = &g.constructions[0];

        // `opt hole box TYPEPATH`.
        match &construction.ast.fields()[1].kind {
            FieldKind::Optional { inner } => match inner.as_ref() {
                FieldKind::Subtree { category, boxed } => {
                    assert_eq!(category.value, "BoxedPhrase");
                    assert!(*boxed, "`hole box` must set boxed = true");
                }
                other => panic!("expected boxed Subtree, got {other:?}"),
            },
            other => panic!("expected Optional, got {other:?}"),
        }

        // `all(...)` / `any(...)`, each holding a `len() == INT` and an
        // `is_some()` child — the highest-risk (recursive) production.
        assert_eq!(construction.constraints.len(), 2);
        let all_children = match &construction.constraints[0] {
            Constraint::Require(pred) => match &pred.value {
                Predicate::All(children) => children,
                other => panic!("expected All(...), got {other:?}"),
            },
            other @ (Constraint::Recognize(_) | Constraint::DeriveFeature { .. }) => {
                panic!("expected Require, got {other:?}")
            }
        };
        let any_children = match &construction.constraints[1] {
            Constraint::Require(pred) => match &pred.value {
                Predicate::Any(children) => children,
                other => panic!("expected Any(...), got {other:?}"),
            },
            other @ (Constraint::Recognize(_) | Constraint::DeriveFeature { .. }) => {
                panic!("expected Require, got {other:?}")
            }
        };
        for children in [all_children, any_children] {
            assert_eq!(children.len(), 2);
            let has_len_3 = children.iter().any(|p| {
                matches!(p, Predicate::LenIs { path, len: 3 } if path.segments[0].value == "members")
            });
            let has_is_some = children.iter().any(
                |p| matches!(p, Predicate::IsSome { path } if path.segments[0].value == "alt"),
            );
            assert!(has_len_3, "missing members.len() == 3 in {children:?}");
            assert!(has_is_some, "missing alt.is_some() in {children:?}");
        }

        // `witness ... = derived IDENT(path, ...);`.
        match &construction.witnesses[0].class {
            WitnessClass::Derived { combinator, args } => {
                assert_eq!(combinator.value, "from_first");
                let names: Vec<&str> = args.iter().map(|p| p.segments[0].value.as_str()).collect();
                assert_eq!(names, vec!["members", "alt"]);
            }
            other => panic!("expected Derived witness, got {other:?}"),
        }

        // Explicit `selection packed;` exercises the `packed` arm of the
        // selection statement — same resulting value as the implicit
        // default, but a different parse path was never run before.
        assert_eq!(construction.selection, SelectionPromise::Packed);
    }

    /// Coverage sweep, snippet 2: `internal construction`, the `bind
    /// TYPEPATH { ... }` shape (never produced by the parser in any prior
    /// test — only `own` was), and `derive path = IDENT(path, ...);`.
    #[test]
    fn coverage_sweep_internal_bind_shape_and_derive_feature() {
        let g = parse_group(quote::quote! {
            group cov2;

            internal construction hidden: Cat {
                bind covered::path::Target {
                    f: lex Tag,
                }
                derive f = combine(f);
                form only @ 0 = lex(f);
            }
        })
        .expect("coverage-sweep snippet 2 parses");
        let construction = &g.constructions[0];
        assert!(
            construction.internal,
            "`internal construction` must set internal = true"
        );

        match &construction.ast {
            AstShape::Bind { path, fields } => {
                assert_eq!(path.value, "covered::path::Target");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].field.value, "f");
            }
            other @ AstShape::Own { .. } => panic!("expected Bind shape, got {other:?}"),
        }

        match &construction.constraints[0] {
            Constraint::DeriveFeature {
                target,
                feature_type,
                combinator,
                args,
            } => {
                assert_eq!(target.segments[0].value, "f");
                assert!(feature_type.is_none());
                assert_eq!(combinator.value, "combine");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0].segments[0].value, "f");
            }
            other @ (Constraint::Require(_) | Constraint::Recognize(_)) => {
                panic!("expected DeriveFeature, got {other:?}")
            }
        }
    }
}
