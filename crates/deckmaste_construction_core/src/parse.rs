use proc_macro2::TokenStream;
use syn::Ident;
use syn::LitBool;
use syn::LitStr;
use syn::Pat;
use syn::Path;
use syn::Token;
use syn::braced;
use syn::bracketed;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::model::AbstractAlternative;
use crate::model::AbstractProduct;
use crate::model::AbstractSum;
use crate::model::BuildLeaf;
use crate::model::CodecAtomClass;
use crate::model::Construction;
use crate::model::ContextIdentityArm;
use crate::model::ContextIdentityCanonicalSource;
use crate::model::ContextIdentitySource;
use crate::model::ContextIdentitySourceArm;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Element;
use crate::model::Feature;
use crate::model::FeatureEquation;
use crate::model::FeatureMatchArm;
use crate::model::FeaturePlace;
use crate::model::FeatureSlot;
use crate::model::FeatureValue;
use crate::model::Field;
use crate::model::FieldCheck;
use crate::model::FieldKind;
use crate::model::FixedSurfaceAtomSource;
use crate::model::FixedSurfaceSource;
use crate::model::Form;
use crate::model::FormAtom;
use crate::model::FormGuardSource;
use crate::model::GeneratedCodecRecipe;
use crate::model::GeneratedIdentityRecipe;
use crate::model::LeafCallback;
use crate::model::LengthComparison;
use crate::model::Lexeme;
use crate::model::LexemeMember;
use crate::model::LexemeOverride;
use crate::model::Morphology;
use crate::model::OpenDeclarationAtom;
use crate::model::PositionalSeparatorSource;
use crate::model::RenderBinding;
use crate::model::RequireExprSource;
use crate::model::RequireSubjectSource;
use crate::model::Root;
use crate::model::SeparatorSource;
use crate::model::SequenceSurfaceSource;
use crate::model::SignedDecimalSignRoleSource;
use crate::model::SignedDecimalSignSpelling;
use crate::model::SignedDecimalSignTypeSource;
use crate::model::SignedDecimalSource;
use crate::model::SurfaceCaseTransition;
use crate::model::TerminalBinding;
use crate::model::TerminalBindingKind;
use crate::model::Traversal;
use crate::model::TraversalBranch;
use crate::model::TraversalBranchArm;
use crate::model::TraversalCall;
use crate::model::TraversalField;
use crate::model::TraversalKind;
use crate::model::TraversalPart;
use crate::model::UnsignedNumberSource;
use crate::model::UnsignedPrimitiveSource;
use crate::model::VerbOperand;
use crate::model::VisitMode;
use crate::model::Vocab;
use crate::model::VocabVariant;

mod keyword {
    syn::custom_keyword!(checked);
    syn::custom_keyword!(borrowed);
    syn::custom_keyword!(argument);
    syn::custom_keyword!(all);
    syn::custom_keyword!(any);
    syn::custom_keyword!(callback);
    syn::custom_keyword!(call);
    syn::custom_keyword!(codec);
    syn::custom_keyword!(construct);
    syn::custom_keyword!(construction);
    syn::custom_keyword!(context_identity);
    syn::custom_keyword!(copy);
    syn::custom_keyword!(derive);
    syn::custom_keyword!(eoi);
    syn::custom_keyword!(element);
    syn::custom_keyword!(field);
    syn::custom_keyword!(feature);
    syn::custom_keyword!(form);
    syn::custom_keyword!(generate);
    syn::custom_keyword!(identity);
    syn::custom_keyword!(is);
    syn::custom_keyword!(lex);
    syn::custom_keyword!(lexeme);
    syn::custom_keyword!(len);
    syn::custom_keyword!(leaf);
    syn::custom_keyword!(morphology);
    syn::custom_keyword!(mobile);
    syn::custom_keyword!(noun);
    syn::custom_keyword!(otherwise);
    syn::custom_keyword!(part);
    syn::custom_keyword!(pattern);
    syn::custom_keyword!(punctuation);
    syn::custom_keyword!(product);
    syn::custom_keyword!(render);
    syn::custom_keyword!(recipe);
    syn::custom_keyword!(require);
    syn::custom_keyword!(root);
    syn::custom_keyword!(scanner);
    syn::custom_keyword!(sentence_initial);
    syn::custom_keyword!(continuation);
    syn::custom_keyword!(standalone_render);
    syn::custom_keyword!(sum);
    syn::custom_keyword!(separated);
    syn::custom_keyword!(terminated);
    syn::custom_keyword!(by);
    syn::custom_keyword!(position);
    syn::custom_keyword!(opt);
    syn::custom_keyword!(seq);
    syn::custom_keyword!(traversal);
    syn::custom_keyword!(using);
    syn::custom_keyword!(value_type);
    syn::custom_keyword!(variant);
    syn::custom_keyword!(verb);
    syn::custom_keyword!(visit);
    syn::custom_keyword!(vocab);
    syn::custom_keyword!(when);
    syn::custom_keyword!(zeroable);
}

pub(crate) fn parse_declarations(tokens: TokenStream) -> syn::Result<Declarations> {
    syn::parse2(tokens)
}

impl Parse for Declarations {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut declarations = Vec::new();
        while !input.is_empty() {
            if input.peek(Token![#]) {
                return Err(deferred(input.span(), "doc comments"));
            }
            if input.peek(keyword::construction) {
                declarations.push(Declaration::Construction(parse_construction(input)?));
            } else if peek_ident(input, "abstract") {
                declarations.push(parse_abstract_declaration(input)?);
            } else if input.peek(keyword::vocab) {
                declarations.push(Declaration::Vocab(parse_vocab(input)?));
            } else if input.peek(keyword::morphology) {
                declarations.push(Declaration::Morphology(parse_morphology(input)?));
            } else if input.peek(keyword::lexeme) {
                declarations.push(Declaration::Lexeme(parse_lexeme(input)?));
            } else if input.peek(keyword::codec) {
                declarations.push(Declaration::Codec(parse_terminal_binding(
                    input,
                    BindingKind::Codec,
                )?));
            } else if input.peek(keyword::identity) {
                declarations.push(Declaration::Identity(parse_terminal_binding(
                    input,
                    BindingKind::Identity,
                )?));
            } else if input.peek(keyword::root) {
                declarations.push(Declaration::Root(parse_root(input)?));
            } else if input.peek(keyword::scanner) {
                return Err(deferred(input.span(), "scanner"));
            } else {
                return Err(input.error(
                    "expected construction, abstract, vocab, morphology, lexeme, codec, identity, or root declaration",
                ));
            }
        }
        Ok(Self { declarations })
    }
}

fn parse_abstract_declaration(input: ParseStream<'_>) -> syn::Result<Declaration> {
    let abstract_keyword = input.call(Ident::parse_any)?;
    debug_assert_eq!(abstract_keyword, "abstract");
    if input.peek(keyword::product) {
        return Ok(Declaration::AbstractProduct(parse_abstract_product(input)?));
    }
    if input.peek(keyword::sum) {
        return Ok(Declaration::AbstractSum(parse_abstract_sum(input)?));
    }
    Err(input.error("abstract declaration requires product or sum"))
}

fn parse_abstract_product(input: ParseStream<'_>) -> syn::Result<AbstractProduct> {
    input.parse::<keyword::product>()?;
    let name = input.call(Ident::parse_any)?;
    let content;
    braced!(content in input);
    let fields = parse_fields(&content, &name)?;

    let mut requirements = Vec::new();
    while input.peek(keyword::require) {
        requirements.push(parse_requirement(input)?);
    }
    Ok(AbstractProduct {
        name,
        fields,
        requirements,
    })
}

fn parse_abstract_sum(input: ParseStream<'_>) -> syn::Result<AbstractSum> {
    input.parse::<keyword::sum>()?;
    let name = input.call(Ident::parse_any)?;
    let content;
    braced!(content in input);
    let mut alternatives = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        let alternative_name = content.call(Ident::parse_any)?;
        let value_type = if content.peek(Token![:]) {
            content.parse::<Token![:]>()?;
            parse_generated_owned_path(&content)?
        } else {
            syn::parse_quote_spanned!(alternative_name.span()=> #alternative_name)
        };
        alternatives.push(AbstractAlternative {
            name: alternative_name,
            value_type,
        });
        content.parse::<Token![,]>()?;
    }
    Ok(AbstractSum { name, alternatives })
}

fn parse_construction(input: ParseStream<'_>) -> syn::Result<Construction> {
    input.parse::<keyword::construction>()?;
    let name = input.call(Ident::parse_any)?;
    input.parse::<Token![:]>()?;
    let category = parse_generated_owned_path(input)?;
    let content;
    braced!(content in input);

    if !content.peek(keyword::element) {
        return Err(content.error("construction requires exactly one named element product"));
    }
    let element = parse_element(&content)?;
    let mut requirements = Vec::new();
    let mut equations = Vec::new();
    let mut forms = Vec::new();

    while !content.is_empty() {
        if content.peek(Token![#]) {
            return Err(deferred(content.span(), "doc comments"));
        }
        if content.peek(keyword::checked) {
            return Err(content
                .error("`checked` metadata was retired after Stage 4; use generated invariants"));
        } else if content.peek(keyword::require) {
            requirements.push(parse_requirement(&content)?);
        } else if content.peek(keyword::derive) {
            equations.push(parse_equation(&content)?);
        } else if content.peek(keyword::form) {
            forms.push(parse_form(&content)?);
        } else if content.peek(keyword::when) {
            return Err(deferred(content.span(), "when"));
        } else if content.peek(keyword::otherwise) {
            return Err(deferred(content.span(), "otherwise"));
        } else if content.peek(keyword::element) {
            return Err(content.error("construction requires exactly one named element product"));
        } else {
            return Err(content.error("expected require, derive, or form after element"));
        }
    }

    validate_form_list(&forms, &name)?;
    Ok(Construction {
        name,
        category,
        element,
        requirements,
        equations,
        forms,
    })
}

fn parse_element(input: ParseStream<'_>) -> syn::Result<Element> {
    input.parse::<keyword::element>()?;
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let fields = parse_fields(&content, &name)?;
    Ok(Element { name, fields })
}

fn parse_fields(input: ParseStream<'_>, owner: &Ident) -> syn::Result<Vec<Field>> {
    let mut fields = Vec::new();
    let mut errors = None;
    while !input.is_empty() {
        reject_doc_comment(input)?;
        match parse_field(input, owner) {
            Ok(field) => fields.push(field),
            Err(error) => {
                combine(&mut errors, error);
                if !input.peek(Token![,]) {
                    break;
                }
                input.parse::<Token![,]>()?;
            }
        }
    }
    errors.map_or(Ok(fields), Err)
}

fn parse_field(input: ParseStream<'_>, owner: &Ident) -> syn::Result<Field> {
    let name = input.parse()?;
    input.parse::<Token![:]>()?;
    let mobile = if input.peek(keyword::mobile) {
        input.parse::<keyword::mobile>()?;
        true
    } else {
        false
    };
    let kind = parse_field_kind(input, owner, &name)?;
    let check = parse_field_check(
        input,
        matches!(&kind, FieldKind::Zeroable { .. }).then_some("zeroable-check"),
    )?;
    input.parse::<Token![,]>()?;
    Ok(Field {
        name,
        kind,
        mobile,
        check,
    })
}

fn parse_field_check(
    input: ParseStream<'_>,
    legacy_label: Option<&str>,
) -> syn::Result<Option<FieldCheck>> {
    if !input.peek(keyword::checked) {
        return Ok(None);
    }
    input.parse::<keyword::checked>()?;
    input.parse::<keyword::by>()?;
    let function = parse_generated_owned_callback_path(input)?;
    let content;
    parenthesized!(content in input);
    let mut arguments = Vec::new();
    while !content.is_empty() {
        let role = content.parse()?;
        content.parse::<Token![.]>()?;
        let feature = content.call(Ident::parse_any)?;
        let label = legacy_label.unwrap_or("field-check");
        let feature = feature_from_ident(&feature)
            .ok_or_else(|| content.error(format!("unknown {label} feature")))?;
        arguments.push(FeatureSlot { role, feature });
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
    }
    Ok(Some(FieldCheck {
        function,
        arguments,
    }))
}

fn parse_field_kind(input: ParseStream<'_>, owner: &Ident, role: &Ident) -> syn::Result<FieldKind> {
    if input.peek(keyword::zeroable) {
        input.parse::<keyword::zeroable>()?;
        let value_type = parse_generated_owned_path(input)?;
        let from = input.call(Ident::parse_any)?;
        if from != "from" {
            return Err(syn::Error::new(
                from.span(),
                "zeroable fields require `from`",
            ));
        }
        let item = Box::new(FieldKind::Category(parse_generated_owned_path(input)?));
        return Ok(FieldKind::Zeroable { value_type, item });
    }
    if input.peek(keyword::opt) {
        input.parse::<keyword::opt>()?;
        return parse_cardinality_inner(input, owner, role, "opt")
            .map(|item| FieldKind::Optional(Box::new(item)));
    }
    if input.peek(keyword::seq) {
        input.parse::<keyword::seq>()?;
        let item = parse_cardinality_inner(input, owner, role, "seq")?;
        let mut separator = None;
        let mut terminator = None;
        while input.peek(keyword::separated) || input.peek(keyword::terminated) {
            if input.peek(keyword::separated) {
                let policy = input.parse::<keyword::separated>()?;
                if separator.is_some() {
                    return Err(syn::Error::new(
                        policy.span(),
                        "duplicate sequence separator policy",
                    ));
                }
                input.parse::<keyword::by>()?;
                separator = Some(parse_separator_source(input)?);
            } else {
                let policy = input.parse::<keyword::terminated>()?;
                if terminator.is_some() {
                    return Err(syn::Error::new(
                        policy.span(),
                        "duplicate sequence terminator policy",
                    ));
                }
                input.parse::<keyword::by>()?;
                terminator = Some(parse_fixed_surface_source(input)?);
            }
        }
        return Ok(FieldKind::Sequence {
            item: Box::new(item),
            surface: SequenceSurfaceSource {
                separator,
                terminator,
            },
        });
    }
    parse_atomic_field_kind(input)
}

fn parse_cardinality_inner(
    input: ParseStream<'_>,
    owner: &Ident,
    role: &Ident,
    outer: &str,
) -> syn::Result<FieldKind> {
    let nested = if input.peek(keyword::opt) {
        input.parse::<keyword::opt>()?;
        Some("opt")
    } else if input.peek(keyword::seq) {
        input.parse::<keyword::seq>()?;
        Some("seq")
    } else {
        None
    };
    if let Some(nested) = nested {
        let item = parse_atomic_field_kind(input)?;
        let spelling = match item {
            FieldKind::Category(path) | FieldKind::Lex(path) | FieldKind::Identity(path) => path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            FieldKind::Zeroable { .. } | FieldKind::Optional(_) | FieldKind::Sequence { .. } => {
                unreachable!("atomic field kind")
            }
        };
        return Err(syn::Error::new(
            role.span(),
            format!("field `{owner}.{role}`: {outer} {nested} {spelling}"),
        ));
    }
    parse_atomic_field_kind(input)
}

fn parse_atomic_field_kind(input: ParseStream<'_>) -> syn::Result<FieldKind> {
    if input.peek(keyword::lex) {
        input.parse::<keyword::lex>()?;
        return Ok(FieldKind::Lex(parse_generated_owned_path(input)?));
    }
    if input.peek(keyword::identity) {
        input.parse::<keyword::identity>()?;
        return Ok(FieldKind::Identity(parse_generated_owned_path(input)?));
    }
    if input.peek(Token![<]) {
        return Err(generated_owned_path_error(input.span()));
    }
    if input.peek(Ident) {
        return Ok(FieldKind::Category(parse_generated_owned_path(input)?));
    }
    Err(input.error("expected a category, lex terminal, or identity terminal field"))
}

fn parse_separator_source(input: ParseStream<'_>) -> syn::Result<SeparatorSource> {
    if input.peek(keyword::position) {
        input.parse::<keyword::position>()?;
        let content;
        braced!(content in input);
        let mut rows = Vec::new();
        while !content.is_empty() {
            let class = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            let surface = parse_fixed_surface_source(&content)?;
            content.parse::<Token![;]>()?;
            rows.push(PositionalSeparatorSource { class, surface });
        }
        Ok(SeparatorSource::Positional(rows))
    } else {
        Ok(SeparatorSource::Uniform(parse_fixed_surface_source(input)?))
    }
}

fn parse_fixed_surface_source(input: ParseStream<'_>) -> syn::Result<FixedSurfaceSource> {
    let transition = if input.peek(keyword::sentence_initial) {
        input.parse::<keyword::sentence_initial>()?;
        Some(("sentence_initial", SurfaceCaseTransition::SentenceInitial))
    } else if input.peek(keyword::continuation) {
        input.parse::<keyword::continuation>()?;
        Some(("continuation", SurfaceCaseTransition::Continuation))
    } else {
        None
    };
    if let Some((name, transition)) = transition {
        let content;
        parenthesized!(content in input);
        let surface = parse_fixed_surface_source(&content)?;
        if surface.transition != SurfaceCaseTransition::Preserve || !content.is_empty() {
            return Err(content.error(format!("{name} accepts exactly one unnested fixed surface")));
        }
        return Ok(FixedSurfaceSource {
            atoms: surface.atoms,
            transition,
        });
    }
    let mut atoms = Vec::new();
    while input.peek(LitStr) || input.peek(keyword::lex) {
        if input.peek(LitStr) {
            atoms.push(FixedSurfaceAtomSource::Literal(input.parse()?));
        } else {
            input.parse::<keyword::lex>()?;
            let content;
            parenthesized!(content in input);
            let path = parse_generated_owned_path(&content)?;
            if !content.is_empty() {
                return Err(content.error("fixed lex surface atom accepts one path"));
            }
            atoms.push(FixedSurfaceAtomSource::Lex(path));
        }
    }
    if atoms.is_empty() {
        return Err(input.error("fixed surface requires a literal or lex(path) atom"));
    }
    Ok(FixedSurfaceSource {
        atoms,
        transition: SurfaceCaseTransition::Preserve,
    })
}

fn parse_requirement(input: ParseStream<'_>) -> syn::Result<RequireExprSource> {
    input.parse::<keyword::require>()?;
    let requirement = parse_require_expr(input)?;
    input.parse::<Token![;]>()?;
    Ok(requirement)
}

fn parse_require_expr(input: ParseStream<'_>) -> syn::Result<RequireExprSource> {
    if input.peek(keyword::all) {
        input.parse::<keyword::all>()?;
        return parse_require_group(input, RequireExprSource::All, "all");
    }
    if input.peek(keyword::any) {
        input.parse::<keyword::any>()?;
        return parse_require_group(input, RequireExprSource::Any, "any");
    }
    if input.peek(Token![!]) {
        return Err(deferred(input.span(), "Plan 05 structural declarations"));
    }

    if input.peek(keyword::len) {
        return parse_length_requirement(input);
    }

    let first = input.call(Ident::parse_any)?;
    let subject = if input.peek(Token![::]) || input.peek(syn::token::Paren) {
        return Err(deferred(first.span(), "Plan 05 structural declarations"));
    } else if input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        let feature_ident = input.call(Ident::parse_any)?;
        if feature_ident == "is_some" || feature_ident == "is_none" {
            let content;
            parenthesized!(content in input);
            if !content.is_empty() {
                return Err(content.error("optional-presence requirements accept no arguments"));
            }
            return Ok(RequireExprSource::OptionalPresence {
                role: first,
                present: feature_ident == "is_some",
            });
        }
        if input.peek(syn::token::Paren) {
            return Err(deferred(
                feature_ident.span(),
                "Plan 05 structural declarations",
            ));
        }
        let Some(feature) = feature_from_ident(&feature_ident) else {
            return Err(syn::Error::new(
                feature_ident.span(),
                format!("unknown require feature `{feature_ident}`"),
            ));
        };
        RequireSubjectSource::RoleFeature {
            role: first,
            feature,
        }
    } else if let Some(feature) = feature_from_ident(&first) {
        RequireSubjectSource::ConstructionFeature(feature)
    } else {
        RequireSubjectSource::Role(first)
    };

    let members = if input.peek(keyword::is) {
        input.parse::<keyword::is>()?;
        vec![parse_require_member(input)?]
    } else if input.peek(Token![in]) {
        input.parse::<Token![in]>()?;
        parse_require_members(input)?
    } else {
        return Err(deferred(input.span(), "Plan 05 structural declarations"));
    };
    Ok(RequireExprSource::In { subject, members })
}

fn parse_length_requirement(input: ParseStream<'_>) -> syn::Result<RequireExprSource> {
    input.parse::<keyword::len>()?;
    let content;
    parenthesized!(content in input);
    let subject: Path = parse_generated_owned_path(&content)?;
    let (owner, role) = if content.peek(Token![.]) {
        content.parse::<Token![.]>()?;
        (Some(subject), content.call(Ident::parse_any)?)
    } else {
        let Some(role) = subject.get_ident() else {
            return Err(syn::Error::new(
                subject.span(),
                "len subject requires a role or Type.role",
            ));
        };
        (None, role.clone())
    };
    if !content.is_empty() {
        return Err(content.error("len subject accepts one role"));
    }
    let comparison = if input.peek(Token![>=]) {
        input.parse::<Token![>=]>()?;
        LengthComparison::GreaterThanOrEqual
    } else if input.peek(Token![<=]) {
        input.parse::<Token![<=]>()?;
        LengthComparison::LessThanOrEqual
    } else if input.peek(Token![>]) {
        input.parse::<Token![>]>()?;
        LengthComparison::GreaterThan
    } else if input.peek(Token![<]) {
        input.parse::<Token![<]>()?;
        LengthComparison::LessThan
    } else if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        LengthComparison::Equal
    } else {
        return Err(input.error("len requirement requires a comparison"));
    };
    Ok(RequireExprSource::Length {
        owner,
        role,
        comparison,
        value: input.parse()?,
    })
}

fn parse_require_group(
    input: ParseStream<'_>,
    group: impl FnOnce(Vec<RequireExprSource>) -> RequireExprSource,
    name: &str,
) -> syn::Result<RequireExprSource> {
    let content;
    parenthesized!(content in input);
    let mut operands = Vec::new();
    while !content.is_empty() {
        operands.push(parse_require_expr(&content)?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
        if content.is_empty() {
            return Err(content.error(format!("require {name} does not allow a trailing comma")));
        }
    }
    if operands.len() < 2 {
        return Err(syn::Error::new(
            input.span(),
            format!("require {name} requires at least two operands"),
        ));
    }
    Ok(group(operands))
}

fn parse_require_members(input: ParseStream<'_>) -> syn::Result<Vec<Ident>> {
    let content;
    bracketed!(content in input);
    if content.is_empty() {
        return Err(content.error("require membership requires at least one member"));
    }
    let mut members = Vec::new();
    while !content.is_empty() {
        let member = parse_require_member(&content)?;
        if members.iter().any(|existing| existing == &member) {
            return Err(syn::Error::new(
                member.span(),
                format!("duplicate require member `{member}`"),
            ));
        }
        members.push(member);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
        if content.is_empty() {
            return Err(content.error("require membership does not allow a trailing comma"));
        }
    }
    Ok(members)
}

fn parse_require_member(input: ParseStream<'_>) -> syn::Result<Ident> {
    let member = input.call(Ident::parse_any)?;
    if input.peek(Token![::]) || input.peek(Token![.]) || input.peek(syn::token::Paren) {
        return Err(deferred(member.span(), "Plan 05 structural declarations"));
    }
    Ok(member)
}

fn parse_equation(input: ParseStream<'_>) -> syn::Result<FeatureEquation> {
    let derive_token = input.parse::<keyword::derive>()?;
    let first: Ident = input.parse()?;
    let target = if input.peek(Token![=]) {
        let Some(feature) = feature_from_ident(&first) else {
            return Err(deferred(first.span(), "derive target"));
        };
        FeaturePlace::Construction(feature)
    } else {
        if !input.peek(Token![.]) {
            return Err(deferred(derive_token.span(), "derive target"));
        }
        input.parse::<Token![.]>()?;
        let feature_ident: Ident = input.parse()?;
        let Some(feature) = feature_from_ident(&feature_ident) else {
            return Err(deferred(feature_ident.span(), "derive target"));
        };
        FeaturePlace::Role {
            field: first,
            feature,
        }
    };
    input.parse::<Token![=]>()?;

    let value = if input.peek(Token![match]) {
        input.parse::<Token![match]>()?;
        let role = input.parse()?;
        let arms_content;
        braced!(arms_content in input);
        let mut arms = Vec::new();
        while !arms_content.is_empty() {
            let variant = arms_content.parse()?;
            arms_content.parse::<Token![=>]>()?;
            let value = arms_content.parse()?;
            arms.push(FeatureMatchArm { variant, value });
            if arms_content.is_empty() {
                break;
            }
            arms_content.parse::<Token![,]>()?;
        }
        FeatureValue::Match { role, arms }
    } else if input.peek(Ident) && input.peek2(Token![.]) {
        let role = input.parse()?;
        input.parse::<Token![.]>()?;
        let feature_ident: Ident = input.parse()?;
        let Some(feature) = feature_from_ident(&feature_ident) else {
            return Err(deferred(feature_ident.span(), "derive source"));
        };
        FeatureValue::FromRole(FeatureSlot { role, feature })
    } else {
        let path = parse_generated_owned_path(input)?;
        FeatureValue::Constant(path)
    };
    input.parse::<Token![;]>()?;
    Ok(FeatureEquation { target, value })
}

fn feature_from_ident(ident: &Ident) -> Option<Feature> {
    match ident.to_string().as_str() {
        "concord_class" => Some(Feature::ConcordClass),
        "bare_locative_complement" => Some(Feature::BareLocativeComplement),
        "bare_locative_license" => Some(Feature::BareLocativeLicense),
        "cardinality" => Some(Feature::Cardinality),
        "compoundability" => Some(Feature::Compoundability),
        "countability" => Some(Feature::Countability),
        "homograph_license" => Some(Feature::HomographLicense),
        "manner_anaphor_class" => Some(Feature::MannerAnaphorClass),
        "modifier_license" => Some(Feature::ModifierLicense),
        "determiner_number" => Some(Feature::DeterminerNumber),
        "fused_head_license" => Some(Feature::FusedHeadLicense),
        "preposition_complement_kind" => Some(Feature::PrepositionComplementKind),
        "locative_temporal_license" => Some(Feature::LocativeTemporalLicense),
        "nominal_form" => Some(Feature::NominalForm),
        "nominal_license" => Some(Feature::NominalLicense),
        "number" => Some(Feature::Number),
        "onset" => Some(Feature::Onset),
        "possessive_ending" => Some(Feature::PossessiveEnding),
        "participle" => Some(Feature::Participle),
        "properness" => Some(Feature::Properness),
        "relationality" => Some(Feature::Relationality),
        "preposition_attachment" => Some(Feature::PrepositionAttachment),
        _ => None,
    }
}

fn lexeme_feature_from_ident(ident: &Ident) -> Option<Feature> {
    (ident == "Compoundability")
        .then_some(Feature::Compoundability)
        .or_else(|| (ident == "BareLocativeComplement").then_some(Feature::BareLocativeComplement))
        .or_else(|| (ident == "BareLocativeLicense").then_some(Feature::BareLocativeLicense))
        .or_else(|| (ident == "Countability").then_some(Feature::Countability))
        .or_else(|| (ident == "HomographLicense").then_some(Feature::HomographLicense))
        .or_else(|| (ident == "MannerAnaphorClass").then_some(Feature::MannerAnaphorClass))
        .or_else(|| (ident == "ModifierLicense").then_some(Feature::ModifierLicense))
        .or_else(|| (ident == "DeterminerNumber").then_some(Feature::DeterminerNumber))
        .or_else(|| (ident == "NominalLicense").then_some(Feature::NominalLicense))
        .or_else(|| {
            (ident == "PrepositionComplementKind").then_some(Feature::PrepositionComplementKind)
        })
        .or_else(|| {
            (ident == "LocativeTemporalLicense").then_some(Feature::LocativeTemporalLicense)
        })
        .or_else(|| (ident == "Properness").then_some(Feature::Properness))
        .or_else(|| (ident == "Relationality").then_some(Feature::Relationality))
        .or_else(|| (ident == "PrepositionAttachment").then_some(Feature::PrepositionAttachment))
        .or_else(|| feature_from_ident(ident))
}

fn parse_form(input: ParseStream<'_>) -> syn::Result<Form> {
    input.parse::<keyword::form>()?;
    let name = input.call(Ident::parse_any)?;
    let guard = if input.peek(keyword::when) {
        input.parse::<keyword::when>()?;
        FormGuardSource::When(parse_form_guard_expr(input)?)
    } else if input.peek(keyword::otherwise) {
        input.parse::<keyword::otherwise>()?;
        FormGuardSource::Otherwise
    } else {
        FormGuardSource::Unguarded
    };
    input.parse::<Token![=]>()?;
    let mut atoms = Vec::new();
    while !input.peek(Token![;]) {
        if input.peek(keyword::when) {
            return Err(deferred(input.span(), "when"));
        }
        if input.peek(keyword::otherwise) {
            return Err(deferred(input.span(), "otherwise"));
        }
        atoms.push(parse_form_atom(input, true)?);
    }
    input.parse::<Token![;]>()?;
    Ok(Form { name, guard, atoms })
}

#[expect(
    clippy::too_many_lines,
    reason = "form parsing keeps the closed atom syntax inventory in one dispatch"
)]
fn parse_form_atom(input: ParseStream<'_>, allow_bound: bool) -> syn::Result<FormAtom> {
    if input.peek(LitStr) {
        return Ok(FormAtom::Literal(input.parse()?));
    }

    let ident: Ident = input.parse()?;
    let atom = if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);
        match ident.to_string().as_str() {
            "licensed" if allow_bound => {
                let literal = content.parse::<LitStr>().map_err(|_| {
                    content.error("licensed form atoms require exactly one literal")
                })?;
                if !content.is_empty() {
                    return Err(content.error("licensed form atoms require exactly one literal"));
                }
                FormAtom::LicensedLiteral(literal)
            }
            "licensed" => {
                return Err(syn::Error::new(ident.span(), "licensed atoms cannot nest"));
            }
            "sentence_initial" if allow_bound => {
                let literal = content.parse::<LitStr>().map_err(|_| {
                    content.error("sentence_initial form atoms require exactly one literal")
                })?;
                if literal.value().is_empty() {
                    return Err(syn::Error::new_spanned(
                        literal,
                        "sentence_initial target must realize at least one byte",
                    ));
                }
                if !content.is_empty() {
                    return Err(
                        content.error("sentence_initial form atoms require exactly one literal")
                    );
                }
                FormAtom::SentenceInitial(literal)
            }
            "sentence_initial" => {
                return Err(syn::Error::new(
                    ident.span(),
                    "sentence_initial atoms cannot nest",
                ));
            }
            "structural" if allow_bound => {
                let literal = content.parse::<LitStr>().map_err(|_| {
                    content.error("structural form atoms require exactly one literal")
                })?;
                if literal.value().is_empty() {
                    return Err(syn::Error::new_spanned(
                        literal,
                        "structural target must realize at least one byte",
                    ));
                }
                if !content.is_empty() {
                    return Err(content.error("structural form atoms require exactly one literal"));
                }
                FormAtom::StructuralLiteral(literal)
            }
            "structural" => {
                return Err(syn::Error::new(
                    ident.span(),
                    "structural atoms cannot nest",
                ));
            }
            "prefix" | "suffix" if allow_bound => {
                let direction = if ident == "prefix" {
                    crate::model::BoundDirection::Prefix
                } else {
                    crate::model::BoundDirection::Suffix
                };
                let (affix, value) = match direction {
                    crate::model::BoundDirection::Prefix => {
                        let affix = content.parse()?;
                        content.parse::<Token![,]>()?;
                        let value = parse_form_atom(&content, false)?;
                        (affix, value)
                    }
                    crate::model::BoundDirection::Suffix => {
                        let value = parse_form_atom(&content, false)?;
                        content.parse::<Token![,]>()?;
                        let affix = content.parse()?;
                        (affix, value)
                    }
                };
                if !content.is_empty() {
                    return Err(content
                        .error("bound atoms accept exactly one fixed affix and one value atom"));
                }
                FormAtom::Bound(crate::model::BoundAtom {
                    direction,
                    affix,
                    value: Box::new(value),
                })
            }
            "prefix" | "suffix" => {
                return Err(syn::Error::new(ident.span(), "bound atoms cannot nest"));
            }
            "circumfix" if allow_bound => {
                let prefix = content.parse()?;
                content.parse::<Token![,]>()?;
                let value = parse_form_atom(&content, false)?;
                content.parse::<Token![,]>()?;
                let suffix = content.parse()?;
                if !content.is_empty() {
                    return Err(content
                        .error("circumfix atoms accept exactly two fixed affixes and one role"));
                }
                let FormAtom::Role(role) = value else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "circumfix atom values must be ordinary declared roles",
                    ));
                };
                FormAtom::Circumfix(crate::model::CircumfixAtom {
                    prefix,
                    role,
                    suffix,
                })
            }
            "circumfix" => {
                return Err(syn::Error::new(ident.span(), "circumfix atoms cannot nest"));
            }
            "verb" => {
                let path = parse_generated_owned_path(&content)?;
                if !content.is_empty() {
                    return Err(content.error("verb atoms accept exactly one operand"));
                }
                FormAtom::Verb(classify_verb_operand(path))
            }
            "open_verb" => {
                let kind = content.call(Ident::parse_any)?;
                content.parse::<Token![,]>()?;
                let name = content.parse()?;
                if !content.is_empty() {
                    return Err(
                        content.error("open_verb atoms accept exactly a declaration kind and name")
                    );
                }
                FormAtom::OpenVerb(OpenDeclarationAtom { kind, name })
            }
            "lex" => {
                let path = parse_generated_owned_path(&content)?;
                if !content.is_empty() {
                    return Err(content.error("form atoms accept exactly one role"));
                }
                if path.leading_colon.is_none() && path.segments.len() == 1 {
                    let segment = path.segments.first().expect("one path segment");
                    if matches!(segment.arguments, syn::PathArguments::None) {
                        FormAtom::Lex(segment.ident.clone())
                    } else {
                        FormAtom::FixedLex(path)
                    }
                } else {
                    FormAtom::FixedLex(path)
                }
            }
            "marked" => {
                let marker = parse_generated_owned_path(&content)?;
                content.parse::<Token![,]>()?;
                let role = content.call(Ident::parse_any)?;
                if !content.is_empty() {
                    return Err(content.error(
                        "marked form atoms accept exactly one marker and one category role",
                    ));
                }
                FormAtom::Marked(crate::model::MarkedAtom { marker, role })
            }
            "identity" | "noun" => {
                let role = content.parse()?;
                if !content.is_empty() {
                    return Err(content.error("form atoms accept exactly one role"));
                }
                if ident == "identity" {
                    FormAtom::Identity(role)
                } else {
                    FormAtom::Noun(role)
                }
            }
            _ => return Err(syn::Error::new(ident.span(), "unknown form atom")),
        }
    } else {
        FormAtom::Role(ident)
    };
    Ok(atom)
}

fn parse_form_guard_expr(input: ParseStream<'_>) -> syn::Result<RequireExprSource> {
    if input.peek(keyword::all) {
        input.parse::<keyword::all>()?;
        return parse_form_guard_group(input, RequireExprSource::All, "all");
    }
    if input.peek(keyword::any) {
        input.parse::<keyword::any>()?;
        return parse_form_guard_group(input, RequireExprSource::Any, "any");
    }
    if input.peek(Token![!]) {
        return Err(input.error("form guards do not support negation"));
    }
    if input.peek(keyword::len) {
        return Err(input.error("form guards do not support length subjects"));
    }

    let role = input.call(Ident::parse_any)?;
    if input.peek(Token![::]) || input.peek(syn::token::Paren) {
        return Err(syn::Error::new(
            role.span(),
            "form guards require a role subject",
        ));
    }
    let subject = if input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        let predicate = input.call(Ident::parse_any)?;
        if predicate == "is_some" || predicate == "is_none" {
            let content;
            parenthesized!(content in input);
            if !content.is_empty() {
                return Err(content.error("optional-presence guards accept no arguments"));
            }
            return Ok(RequireExprSource::OptionalPresence {
                role,
                present: predicate == "is_some",
            });
        }
        let Some(feature) = feature_from_ident(&predicate) else {
            return Err(syn::Error::new(
                predicate.span(),
                format!("unknown form guard feature `{predicate}`"),
            ));
        };
        RequireSubjectSource::RoleFeature { role, feature }
    } else if let Some(feature) = feature_from_ident(&role) {
        RequireSubjectSource::ConstructionFeature(feature)
    } else {
        RequireSubjectSource::Role(role)
    };

    let members = if input.peek(keyword::is) {
        input.parse::<keyword::is>()?;
        vec![parse_require_member(input)?]
    } else if input.peek(Token![in]) {
        input.parse::<Token![in]>()?;
        parse_require_members(input)?
    } else {
        return Err(input.error("form guard requires `is` or `in`"));
    };
    Ok(RequireExprSource::In { subject, members })
}

fn parse_form_guard_group(
    input: ParseStream<'_>,
    group: impl FnOnce(Vec<RequireExprSource>) -> RequireExprSource,
    name: &str,
) -> syn::Result<RequireExprSource> {
    let content;
    parenthesized!(content in input);
    let mut operands = Vec::new();
    while !content.is_empty() {
        operands.push(parse_form_guard_expr(&content)?);
        if content.is_empty() {
            break;
        }
        content.parse::<Token![,]>()?;
        if content.is_empty() {
            return Err(content.error(format!("form guard {name} does not allow a trailing comma")));
        }
    }
    if operands.len() < 2 {
        return Err(syn::Error::new(
            input.span(),
            format!("form guard {name} requires at least two operands"),
        ));
    }
    Ok(group(operands))
}

fn validate_form_list(forms: &[Form], construction: &Ident) -> syn::Result<()> {
    if forms.is_empty() {
        return Err(syn::Error::new(
            construction.span(),
            "construction requires at least one form",
        ));
    }

    let mut names = std::collections::HashSet::new();
    let mut fallback = None;
    let mut errors = None;
    for (index, form) in forms.iter().enumerate() {
        if !names.insert(form.name.to_string()) {
            combine(
                &mut errors,
                syn::Error::new(
                    form.name.span(),
                    format!("duplicate form name `{}`", form.name),
                ),
            );
        }
        match form.guard {
            FormGuardSource::Unguarded if forms.len() != 1 => combine(
                &mut errors,
                syn::Error::new(
                    form.name.span(),
                    "an unguarded form cannot be mixed with guarded forms",
                ),
            ),
            FormGuardSource::Otherwise => {
                if fallback.replace(index).is_some() {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            form.name.span(),
                            "construction has multiple fallback forms",
                        ),
                    );
                }
                if index + 1 != forms.len() {
                    combine(
                        &mut errors,
                        syn::Error::new(form.name.span(), "fallback form must be last"),
                    );
                }
            }
            FormGuardSource::Unguarded | FormGuardSource::When(_) => {}
        }
    }
    if forms
        .iter()
        .any(|form| matches!(form.guard, FormGuardSource::When(_)))
        && fallback.is_none()
    {
        let form = forms
            .iter()
            .find(|form| matches!(form.guard, FormGuardSource::When(_)))
            .expect("guarded form exists");
        combine(
            &mut errors,
            syn::Error::new(
                form.name.span(),
                "guarded forms require a final fallback form",
            ),
        );
    }
    if fallback.is_some()
        && !forms
            .iter()
            .any(|form| matches!(form.guard, FormGuardSource::When(_)))
    {
        let form = forms
            .iter()
            .find(|form| matches!(form.guard, FormGuardSource::Otherwise))
            .expect("fallback form exists");
        combine(
            &mut errors,
            syn::Error::new(
                form.name.span(),
                "fallback form requires at least one guarded form",
            ),
        );
    }
    errors.map_or(Ok(()), Err)
}

fn classify_verb_operand(path: Path) -> VerbOperand {
    if path.leading_colon.is_none() && path.segments.len() == 1 {
        let segment = path.segments.first().expect("one path segment");
        if matches!(segment.arguments, syn::PathArguments::None) {
            return VerbOperand::Projected(segment.ident.clone());
        }
    }
    VerbOperand::Fixed(path)
}

fn parse_vocab(input: ParseStream<'_>) -> syn::Result<Vocab> {
    input.parse::<keyword::vocab>()?;
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let mut feature_defaults = Vec::new();
    let mut variants = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(keyword::feature) {
            content.parse::<keyword::feature>()?;
            let feature_ident = content.call(Ident::parse_any)?;
            let feature = lexeme_feature_from_ident(&feature_ident)
                .ok_or_else(|| syn::Error::new(feature_ident.span(), "unknown vocab feature"))?;
            content.parse::<Token![=]>()?;
            let value = content.call(Ident::parse_any)?;
            feature_defaults.push(crate::model::LexemeFeatureDefault { feature, value });
            content.parse::<Token![;]>()?;
            continue;
        }
        let name = content.parse()?;
        content.parse::<Token![=]>()?;
        let word = content.parse()?;
        let mut feature_overrides = Vec::new();
        if content.peek(syn::token::Brace) {
            let overrides_content;
            braced!(overrides_content in content);
            while !overrides_content.is_empty() {
                reject_doc_comment(&overrides_content)?;
                overrides_content.parse::<keyword::feature>()?;
                let feature_ident = overrides_content.call(Ident::parse_any)?;
                let feature = lexeme_feature_from_ident(&feature_ident).ok_or_else(|| {
                    syn::Error::new(feature_ident.span(), "unknown vocab feature")
                })?;
                overrides_content.parse::<Token![=]>()?;
                let value = overrides_content.call(Ident::parse_any)?;
                feature_overrides.push(crate::model::LexemeFeatureOverride { feature, value });
                overrides_content.parse::<Token![;]>()?;
            }
        }
        variants.push(VocabVariant {
            name,
            word,
            feature_overrides,
        });
        content.parse::<Token![,]>()?;
    }
    Ok(Vocab {
        name,
        feature_defaults,
        variants,
    })
}

fn parse_morphology(input: ParseStream<'_>) -> syn::Result<Morphology> {
    input.parse::<keyword::morphology>()?;
    let name = input.parse()?;
    let content;
    braced!(content in input);
    content.parse::<keyword::feature>()?;
    content.parse::<Token![=]>()?;
    let feature_ident: Ident = content.parse()?;
    let feature = match feature_ident.to_string().as_str() {
        "ConcordClass" => Feature::ConcordClass,
        "Number" => Feature::Number,
        "Participle" => Feature::Participle,
        _ => {
            return Err(syn::Error::new(
                feature_ident.span(),
                "morphology feature must be ConcordClass, Number, or Participle",
            ));
        }
    };
    content.parse::<Token![;]>()?;
    content.parse::<keyword::recipe>()?;
    content.parse::<Token![=]>()?;
    let recipe = content.call(Ident::parse_any)?;
    if content.peek(Token![::]) {
        return Err(syn::Error::new(
            recipe.span(),
            "morphology recipe must be one identifier",
        ));
    }
    content.parse::<Token![;]>()?;
    if !content.is_empty() {
        return Err(content.error("morphology contains unknown fields"));
    }
    Ok(Morphology {
        name,
        feature,
        recipe,
    })
}

fn parse_lexeme(input: ParseStream<'_>) -> syn::Result<Lexeme> {
    input.parse::<keyword::lexeme>()?;
    let name = input.parse()?;
    if !input.peek(keyword::using) {
        return Err(input.error("lexeme requires `using` morphology"));
    }
    input.parse::<keyword::using>()?;
    let morphology = input.parse()?;
    let content;
    braced!(content in input);
    let mut feature_defaults = Vec::new();
    let mut members = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(keyword::feature) {
            content.parse::<keyword::feature>()?;
            let feature_ident = content.call(Ident::parse_any)?;
            let feature = lexeme_feature_from_ident(&feature_ident)
                .ok_or_else(|| syn::Error::new(feature_ident.span(), "unknown lexeme feature"))?;
            content.parse::<Token![=]>()?;
            let value = content.call(Ident::parse_any)?;
            feature_defaults.push(crate::model::LexemeFeatureDefault { feature, value });
            content.parse::<Token![;]>()?;
            continue;
        }
        let member_name = content.parse()?;
        if !content.peek(Token![=]) {
            return Err(content.error("lexeme member requires an explicit lemma"));
        }
        content.parse::<Token![=]>()?;
        let lemma: LitStr = content.parse()?;
        if lemma.value().is_empty() {
            return Err(syn::Error::new(
                lemma.span(),
                "lexeme lemma must not be empty",
            ));
        }
        let mut overrides = Vec::new();
        let mut feature_overrides = Vec::new();
        if content.peek(syn::token::Brace) {
            let overrides_content;
            braced!(overrides_content in content);
            while !overrides_content.is_empty() {
                reject_doc_comment(&overrides_content)?;
                if overrides_content.peek(keyword::feature) {
                    overrides_content.parse::<keyword::feature>()?;
                    let feature_ident = overrides_content.call(Ident::parse_any)?;
                    let feature = lexeme_feature_from_ident(&feature_ident).ok_or_else(|| {
                        syn::Error::new(feature_ident.span(), "unknown lexeme feature")
                    })?;
                    overrides_content.parse::<Token![=]>()?;
                    let value = overrides_content.call(Ident::parse_any)?;
                    feature_overrides.push(crate::model::LexemeFeatureOverride { feature, value });
                    overrides_content.parse::<Token![;]>()?;
                    continue;
                }
                let feature = overrides_content.parse()?;
                overrides_content.parse::<Token![=]>()?;
                let surface: LitStr = overrides_content.parse()?;
                if surface.value().is_empty() {
                    return Err(syn::Error::new(
                        surface.span(),
                        "lexeme override surface must not be empty",
                    ));
                }
                overrides.push(LexemeOverride { feature, surface });
                overrides_content.parse::<Token![,]>()?;
            }
        }
        members.push(LexemeMember {
            name: member_name,
            lemma,
            overrides,
            feature_overrides,
        });
        content.parse::<Token![,]>()?;
    }
    Ok(Lexeme {
        name,
        morphology,
        feature_defaults,
        members,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BindingKind {
    Codec,
    Identity,
}

impl BindingKind {
    fn name(self) -> &'static str {
        match self {
            Self::Codec => "codec",
            Self::Identity => "identity",
        }
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the closed legacy and generated terminal grammars share one declaration boundary"
)]
fn parse_terminal_binding(
    input: ParseStream<'_>,
    kind: BindingKind,
) -> syn::Result<TerminalBinding> {
    parse_binding_keyword(input, kind)?;
    let name: Ident = input.parse()?;
    let content;
    braced!(content in input);
    if content.peek(keyword::generate) {
        let (generated, generated_identity) = match kind {
            BindingKind::Codec => (Some(parse_generated_codec(&content)?), None),
            BindingKind::Identity => (None, Some(parse_generated_identity(&content)?)),
        };
        if !content.is_empty() {
            return Err(content.error(format!(
                "generated {} cannot mix `generate` with legacy binding fields",
                kind.name()
            )));
        }
        return Ok(generated_terminal_binding(
            name,
            kind,
            generated,
            generated_identity,
        ));
    }
    let mut value_type = None;
    let mut codec_atom = None;
    let mut lexical_variant = None;
    let mut render = None;
    let mut build = None;
    let mut traversal = None;

    while !content.is_empty() {
        if content.peek(keyword::generate) {
            return Err(content.error(format!(
                "generated {} cannot mix `generate` with legacy binding fields",
                kind.name()
            )));
        }
        if content.peek(Token![#]) {
            return Err(deferred(content.span(), "doc comments"));
        }
        let slot: Ident = content.parse()?;
        match slot.to_string().as_str() {
            "atom" => {
                if kind == BindingKind::Identity {
                    return Err(syn::Error::new(
                        slot.span(),
                        "identity binding does not accept an atom slot",
                    ));
                }
                reject_duplicate(codec_atom.as_ref(), &slot)?;
                content.parse::<Token![=]>()?;
                let atom = content.call(Ident::parse_any)?;
                codec_atom = Some(match atom.to_string().as_str() {
                    "lex" => CodecAtomClass::Lex,
                    "noun" => CodecAtomClass::Noun,
                    _ => {
                        return Err(syn::Error::new(
                            atom.span(),
                            "codec atom class must be `lex` or `noun`",
                        ));
                    }
                });
                content.parse::<Token![;]>()?;
            }
            "value_type" => {
                reject_duplicate(value_type.as_ref(), &slot)?;
                content.parse::<Token![=]>()?;
                value_type = Some(content.parse()?);
                content.parse::<Token![;]>()?;
            }
            "lexical" => {
                reject_duplicate(lexical_variant.as_ref(), &slot)?;
                content.parse::<Token![=]>()?;
                lexical_variant = Some(content.parse()?);
                content.parse::<Token![;]>()?;
            }
            "render" => {
                reject_duplicate(render.as_ref(), &slot)?;
                if content.peek(Token![=]) {
                    content.parse::<Token![=]>()?;
                    render = Some(RenderBinding::Runtime(content.parse()?));
                    content.parse::<Token![;]>()?;
                } else if content.peek(keyword::context_identity) {
                    content.parse::<keyword::context_identity>()?;
                    let arms_content;
                    braced!(arms_content in content);
                    let mut arms = Vec::new();
                    while !arms_content.is_empty() {
                        let variant = arms_content.parse()?;
                        arms_content.parse::<Token![=>]>()?;
                        let accessor = arms_content.parse()?;
                        arms.push(ContextIdentityArm { variant, accessor });
                        arms_content.parse::<Token![,]>()?;
                    }
                    render = Some(RenderBinding::ContextIdentity(arms));
                } else {
                    return Err(content
                        .error("render requires `= runtime_path;` or `context_identity { ... }`"));
                }
            }
            "build" => {
                reject_duplicate(build.as_ref(), &slot)?;
                build = Some(parse_build_leaf(&content)?);
            }
            "traversal" => {
                reject_duplicate(traversal.as_ref(), &slot)?;
                traversal = Some(parse_traversal(&content)?);
            }
            _ => {
                return Err(deferred(
                    slot.span(),
                    &format!("generative {} content", kind.name()),
                ));
            }
        }
    }

    let codec_atom = match kind {
        BindingKind::Codec => codec_atom,
        BindingKind::Identity => None,
    };
    let leaf_slots = [lexical_variant.is_some(), render.is_some(), build.is_some()];
    if leaf_slots.iter().any(|present| *present) && !leaf_slots.iter().all(|present| *present) {
        return Err(syn::Error::new(
            name.span(),
            "terminal leaf metadata requires lexical, render, and build together",
        ));
    }
    if codec_atom.is_some() && !leaf_slots.iter().all(|present| *present) {
        return Err(syn::Error::new(
            name.span(),
            "an atom-capable codec binding requires lexical, render, and build",
        ));
    }
    if kind == BindingKind::Codec
        && codec_atom.is_none()
        && leaf_slots.iter().all(|present| *present)
    {
        return Err(syn::Error::new(
            name.span(),
            "codec binding requires explicit atom slot unless it is traversal-only",
        ));
    }
    Ok(TerminalBinding {
        name: name.clone(),
        kind: match kind {
            BindingKind::Codec => TerminalBindingKind::Codec,
            BindingKind::Identity => TerminalBindingKind::Identity,
        },
        generated: None,
        generated_identity: None,
        codec_atom,
        value_type: required(value_type, &name, "value_type")?,
        lexical_variant,
        render,
        build,
        traversal: required(traversal, &name, "traversal")?,
    })
}

fn parse_binding_keyword(input: ParseStream<'_>, kind: BindingKind) -> syn::Result<()> {
    match kind {
        BindingKind::Codec => input.parse::<keyword::codec>().map(|_| ()),
        BindingKind::Identity => input.parse::<keyword::identity>().map(|_| ()),
    }
}

fn generated_terminal_binding(
    name: Ident,
    kind: BindingKind,
    generated: Option<GeneratedCodecRecipe>,
    generated_identity: Option<GeneratedIdentityRecipe>,
) -> TerminalBinding {
    let value_type: syn::Type = syn::parse_quote_spanned!(name.span()=> #name);
    let lexical_variant: syn::Path = syn::parse_quote_spanned!(name.span()=> Lexical::#name);
    let codec_atom = match generated.as_ref() {
        Some(GeneratedCodecRecipe::DeclarationNoun(_)) => Some(CodecAtomClass::Noun),
        Some(GeneratedCodecRecipe::DeclarationDeterminative(_)) => Some(CodecAtomClass::Lex),
        Some(GeneratedCodecRecipe::DeclarationVerb(_)) => None,
        Some(
            GeneratedCodecRecipe::DeclarationTerm(_)
            | GeneratedCodecRecipe::SignedDecimal(_)
            | GeneratedCodecRecipe::EnglishCardinal(_)
            | GeneratedCodecRecipe::UnsignedDecimal(_)
            | GeneratedCodecRecipe::Unsupported { .. },
        )
        | None => (kind == BindingKind::Codec).then_some(CodecAtomClass::Lex),
    };
    TerminalBinding {
        name,
        kind: match kind {
            BindingKind::Codec => TerminalBindingKind::Codec,
            BindingKind::Identity => TerminalBindingKind::Identity,
        },
        generated,
        generated_identity,
        codec_atom,
        value_type,
        lexical_variant: (kind == BindingKind::Codec).then_some(lexical_variant),
        render: None,
        build: None,
        traversal: Traversal {
            parts: Vec::new(),
            visit_order: Vec::new(),
            callback_mode: None,
            argument: None,
            variants: Vec::new(),
            fields: Vec::new(),
            calls: Vec::new(),
            branches: Vec::new(),
            leaf_callbacks: Vec::new(),
        },
    }
}

fn parse_generated_identity(input: ParseStream<'_>) -> syn::Result<GeneratedIdentityRecipe> {
    input.parse::<keyword::generate>()?;
    let recipe = input.call(Ident::parse_any)?;
    let content;
    braced!(content in input);
    if recipe == "catalog_identity" {
        let mut provider_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            let value = content.call(Ident::parse_any)?;
            content.parse::<Token![;]>()?;
            if slot != "provider" {
                return Err(syn::Error::new(
                    slot.span(),
                    "catalog_identity recipe accepts only a `provider` field",
                ));
            }
            provider_slots.push(crate::model::GeneratedIdentSlot { slot, value });
        }
        return Ok(GeneratedIdentityRecipe::Catalog(
            crate::model::CatalogIdentitySource {
                recipe,
                provider_slots,
            },
        ));
    }
    if recipe != "context" {
        let _: TokenStream = content.parse()?;
        return Ok(GeneratedIdentityRecipe::Unsupported { name: recipe });
    }

    let mut arms = Vec::new();
    let mut canonical_slots = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(Token![_]) {
            let empty = content.parse::<Token![_]>()?;
            return Err(syn::Error::new(
                empty.span(),
                "context identity arm name cannot be empty",
            ));
        }
        let name = content.call(Ident::parse_any)?;
        if name == "canonical_on_collision" {
            content.parse::<Token![=]>()?;
            let arm = content.call(Ident::parse_any)?;
            content.parse::<Token![;]>()?;
            canonical_slots.push(ContextIdentityCanonicalSource { slot: name, arm });
        } else {
            content.parse::<Token![=>]>()?;
            let accessor = content.call(Ident::parse_any)?;
            content.parse::<Token![,]>()?;
            arms.push(ContextIdentitySourceArm {
                variant: name,
                accessor,
            });
        }
    }
    Ok(GeneratedIdentityRecipe::Context(ContextIdentitySource {
        recipe,
        arms,
        canonical_slots,
    }))
}

#[expect(
    clippy::too_many_lines,
    reason = "closed generated-codec recipes share one exhaustive parser dispatch"
)]
fn parse_generated_codec(input: ParseStream<'_>) -> syn::Result<GeneratedCodecRecipe> {
    input.parse::<keyword::generate>()?;
    let recipe = input.call(Ident::parse_any)?;
    let content;
    braced!(content in input);
    if recipe == "declaration_term" {
        let mut position_slots = Vec::new();
        let mut kind_slots = Vec::new();
        let mut param_slots = Vec::new();
        let mut param_policy_slots = Vec::new();
        let mut feature_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            match slot.to_string().as_str() {
                "kinds" => {
                    let kinds_content;
                    bracketed!(kinds_content in content);
                    let kinds = Punctuated::<Ident, Token![,]>::parse_terminated_with(
                        &kinds_content,
                        Ident::parse_any,
                    )?
                    .into_iter()
                    .collect();
                    kind_slots.push(crate::model::DeclarationVerbKindsSource { slot, kinds });
                }
                "params" => {
                    if content.peek(syn::token::Bracket) {
                        let params_content;
                        bracketed!(params_content in content);
                        let kinds = Punctuated::<Ident, Token![,]>::parse_terminated_with(
                            &params_content,
                            Ident::parse_any,
                        )?
                        .into_iter()
                        .collect();
                        param_slots.push(crate::model::DeclarationVerbKindsSource { slot, kinds });
                    } else {
                        let value = content.call(Ident::parse_any)?;
                        param_policy_slots.push(crate::model::GeneratedIdentSlot { slot, value });
                    }
                }
                "position" => {
                    let value = content.call(Ident::parse_any)?;
                    position_slots.push(crate::model::GeneratedIdentSlot { slot, value });
                }
                "feature" => {
                    let value = content.call(Ident::parse_any)?;
                    feature_slots.push(crate::model::GeneratedIdentSlot { slot, value });
                }
                _ => {
                    return Err(syn::Error::new(
                        slot.span(),
                        "declaration_term recipe accepts only `position`, `kinds`, `params`, and `feature` fields",
                    ));
                }
            }
            content.parse::<Token![;]>()?;
        }
        return Ok(GeneratedCodecRecipe::DeclarationTerm(
            crate::model::DeclarationTermSource {
                recipe,
                position_slots,
                kind_slots,
                param_slots,
                param_policy_slots,
                feature_slots,
            },
        ));
    }
    if recipe == "declaration_noun" {
        let mut closed_slots = Vec::new();
        let mut position_slots = Vec::new();
        let mut kind_slots = Vec::new();
        let mut feature_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            if slot == "kinds" {
                let kinds_content;
                bracketed!(kinds_content in content);
                let kinds = Punctuated::<crate::model::DeclarationNounKindSource, Token![,]>::parse_terminated_with(
                    &kinds_content,
                    |input| {
                        let kind = input.call(Ident::parse_any)?;
                        let subtype_family = if input.peek(syn::token::Paren) {
                            let family_content;
                            parenthesized!(family_content in input);
                            let family = family_content.call(Ident::parse_any)?;
                            if !family_content.is_empty() {
                                return Err(family_content.error(
                                    "declaration_noun subtype-family filter accepts one family",
                                ));
                            }
                            Some(family)
                        } else {
                            None
                        };
                        Ok(crate::model::DeclarationNounKindSource {
                            kind,
                            subtype_family,
                        })
                    },
                )?
                .into_iter()
                .collect();
                kind_slots.push(crate::model::DeclarationNounKindsSource { slot, kinds });
            } else {
                let value = content.call(Ident::parse_any)?;
                let row = crate::model::GeneratedIdentSlot { slot, value };
                match row.slot.to_string().as_str() {
                    "closed" => closed_slots.push(row),
                    "position" => position_slots.push(row),
                    "feature" => feature_slots.push(row),
                    _ => {
                        return Err(syn::Error::new(
                            row.slot.span(),
                            "declaration_noun recipe accepts only `closed`, `position`, `kinds`, and `feature` fields",
                        ));
                    }
                }
            }
            content.parse::<Token![;]>()?;
        }
        return Ok(GeneratedCodecRecipe::DeclarationNoun(
            crate::model::DeclarationNounSource {
                recipe,
                closed_slots,
                position_slots,
                kind_slots,
                feature_slots,
            },
        ));
    }
    if recipe == "declaration_determinative" {
        let mut closed_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            match slot.to_string().as_str() {
                "closed" => {
                    let members_input;
                    bracketed!(members_input in content);
                    let members = Punctuated::<crate::model::DeclarationDeterminativeMemberSource, Token![,]>::parse_terminated_with(&members_input, |input| {
                        let lemma = input.call(Ident::parse_any)?;
                        let fields_input;
                        braced!(fields_input in input);
                        let mut number_license_slots = Vec::new();
                        let mut fused_head_license_slots = Vec::new();
                        let mut nominal_license_slots = Vec::new();
                        let mut realization_slots = Vec::new();
                        while !fields_input.is_empty() {
                            let member_slot = fields_input.call(Ident::parse_any)?;
                            fields_input.parse::<Token![=]>()?;
                            match member_slot.to_string().as_str() {
                                "number_license" | "fused_head_license" | "nominal_license" => {
                                    let value = fields_input.call(Ident::parse_any)?;
                                    let row = crate::model::GeneratedIdentSlot { slot: member_slot, value };
                                    match row.slot.to_string().as_str() {
                                        "number_license" => number_license_slots.push(row),
                                        "fused_head_license" => fused_head_license_slots.push(row),
                                        "nominal_license" => nominal_license_slots.push(row),
                                        _ => unreachable!("matched declaration_determinative member slot"),
                                    }
                                }
                                "realizations" => {
                                    let realizations_input;
                                    bracketed!(realizations_input in fields_input);
                                    let realizations = Punctuated::<crate::model::DeclarationDeterminativeRealizationSource, Token![,]>::parse_terminated_with(&realizations_input, |input| {
                                        let fields_input;
                                        braced!(fields_input in input);
                                        let mut surface_slots = Vec::new();
                                        let mut phrase_number_slots = Vec::new();
                                        let mut following_onset_slots = Vec::new();
                                        while !fields_input.is_empty() {
                                            let realization_slot = fields_input.call(Ident::parse_any)?;
                                            fields_input.parse::<Token![=]>()?;
                                            match realization_slot.to_string().as_str() {
                                                "surface" => surface_slots.push(fields_input.parse()?),
                                                "phrase_number" | "following_onset" => {
                                                    let value = fields_input.call(Ident::parse_any)?;
                                                    let row = crate::model::GeneratedIdentSlot { slot: realization_slot, value };
                                                    if row.slot == "phrase_number" { phrase_number_slots.push(row); } else { following_onset_slots.push(row); }
                                                }
                                                _ => return Err(syn::Error::new(realization_slot.span(), "declaration_determinative realization accepts only `surface`, `phrase_number`, and `following_onset` fields")),
                                            }
                                            fields_input.parse::<Token![;]>()?;
                                        }
                                        Ok(crate::model::DeclarationDeterminativeRealizationSource { surface_slots, phrase_number_slots, following_onset_slots })
                                    })?.into_iter().collect();
                                    realization_slots.push(crate::model::DeclarationDeterminativeRealizationsSource { slot: member_slot, realizations });
                                }
                                _ => return Err(syn::Error::new(member_slot.span(), "declaration_determinative member accepts only `number_license`, `fused_head_license`, `nominal_license`, and `realizations` fields")),
                            }
                            fields_input.parse::<Token![;]>()?;
                        }
                        Ok(crate::model::DeclarationDeterminativeMemberSource { lemma, number_license_slots, fused_head_license_slots, nominal_license_slots, realization_slots })
                    })?.into_iter().collect();
                    closed_slots
                        .push(crate::model::DeclarationDeterminativeClosedSource { slot, members });
                }
                _ => {
                    return Err(syn::Error::new(
                        slot.span(),
                        "declaration_determinative recipe accepts only a `closed` field",
                    ));
                }
            }
            content.parse::<Token![;]>()?;
        }
        return Ok(GeneratedCodecRecipe::DeclarationDeterminative(
            crate::model::DeclarationDeterminativeSource {
                recipe,
                closed_slots,
            },
        ));
    }
    if recipe == "declaration_verb" {
        let mut closed_slots = Vec::new();
        let mut class_slots = Vec::new();
        let mut position_slots = Vec::new();
        let mut tail_slots = Vec::new();
        let mut feature_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            match slot.to_string().as_str() {
                "tail" => {
                    let tail_content;
                    bracketed!(tail_content in content);
                    let atoms = Punctuated::<
                        crate::model::DeclarationVerbTailAtomSource,
                        Token![,],
                    >::parse_terminated_with(&tail_content, |input| {
                        let label = if input.peek(Ident) && input.peek2(Token![:]) {
                            let label = input.call(Ident::parse_any)?;
                            input.parse::<Token![:]>()?;
                            Some(label)
                        } else {
                            None
                        };
                        let kind = if input.peek(LitStr) {
                            crate::model::DeclarationVerbTailAtomKindSource::Literal(input.parse()?)
                        } else {
                            let atom = input.call(Ident::parse_any)?;
                            if atom == "lex" {
                                let lexical;
                                parenthesized!(lexical in input);
                                crate::model::DeclarationVerbTailAtomKindSource::Lex(
                                    lexical.parse()?,
                                )
                            } else if atom == "marked" {
                                let marked_content;
                                parenthesized!(marked_content in input);
                                let marker = marked_content.parse()?;
                                marked_content.parse::<Token![,]>()?;
                                let role = marked_content.call(Ident::parse_any)?;
                                if !marked_content.is_empty() {
                                    return Err(marked_content.error(
                                        "marked tail atoms accept exactly one marker and one role",
                                    ));
                                }
                                crate::model::DeclarationVerbTailAtomKindSource::Marked {
                                    marker,
                                    role,
                                }
                            } else {
                                match atom.to_string().as_str() {
                                    "Amount" => {
                                        crate::model::DeclarationVerbTailAtomKindSource::Amount(
                                            atom,
                                        )
                                    }
                                    "ObjectNounPhrase" => {
                                        crate::model::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(atom)
                                    }
                                    "PredicativeComplement" => {
                                        crate::model::DeclarationVerbTailAtomKindSource::PredicativeComplement(atom)
                                    }
                                    _ => crate::model::DeclarationVerbTailAtomKindSource::Role(atom),
                                }
                            }
                        };
                        let optional = input.parse::<Token![?]>().is_ok();
                        if optional
                            && matches!(
                                kind,
                                crate::model::DeclarationVerbTailAtomKindSource::Literal(_)
                            )
                        {
                            return Err(syn::Error::new(
                                input.span(),
                                "declaration_verb tail literals cannot be optional",
                            ));
                        }
                        Ok(crate::model::DeclarationVerbTailAtomSource { label, optional, kind })
                    })?
                    .into_iter()
                    .collect();
                    tail_slots.push(crate::model::DeclarationVerbTailSource { slot, atoms });
                }
                "closed" | "class" | "position" | "feature" => {
                    let value = content.call(Ident::parse_any)?;
                    let row = crate::model::GeneratedIdentSlot { slot, value };
                    match row.slot.to_string().as_str() {
                        "closed" => closed_slots.push(row),
                        "class" => class_slots.push(row),
                        "position" => position_slots.push(row),
                        "feature" => feature_slots.push(row),
                        _ => unreachable!("matched declaration_verb identifier slot"),
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        slot.span(),
                        "declaration_verb recipe accepts only `closed`, `class`, `position`, `tail`, and `feature` fields",
                    ));
                }
            }
            content.parse::<Token![;]>()?;
        }
        return Ok(GeneratedCodecRecipe::DeclarationVerb(
            crate::model::DeclarationVerbSource {
                recipe,
                closed_slots,
                class_slots,
                position_slots,
                tail_slots,
                feature_slots,
            },
        ));
    }
    if matches!(
        recipe.to_string().as_str(),
        "english_cardinal" | "unsigned_decimal"
    ) {
        let mut magnitude_slots = Vec::new();
        while !content.is_empty() {
            reject_doc_comment(&content)?;
            let slot = content.call(Ident::parse_any)?;
            content.parse::<Token![=]>()?;
            if slot != "magnitude" {
                return Err(syn::Error::new(
                    slot.span(),
                    format!("{recipe} recipe accepts only a `magnitude` field"),
                ));
            }
            let primitive = content.call(Ident::parse_any)?;
            content.parse::<Token![;]>()?;
            magnitude_slots.push(unsigned_primitive_source(slot, primitive));
        }
        let source = UnsignedNumberSource {
            recipe: recipe.clone(),
            magnitude_slots,
        };
        return Ok(if recipe == "english_cardinal" {
            GeneratedCodecRecipe::EnglishCardinal(source)
        } else {
            GeneratedCodecRecipe::UnsignedDecimal(source)
        });
    }
    if recipe != "signed_decimal" {
        let _: TokenStream = content.parse()?;
        return Ok(GeneratedCodecRecipe::Unsupported { name: recipe });
    }

    let mut magnitude_slots = Vec::new();
    let mut sign_type_slots = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        let slot = content.call(Ident::parse_any)?;
        content.parse::<Token![=]>()?;
        match slot.to_string().as_str() {
            "magnitude" => {
                let primitive = content.call(Ident::parse_any)?;
                content.parse::<Token![;]>()?;
                magnitude_slots.push(unsigned_primitive_source(slot, primitive));
            }
            "sign_type" => {
                let name = content.call(Ident::parse_any)?;
                let roles_content;
                braced!(roles_content in content);
                let mut roles = Vec::new();
                while !roles_content.is_empty() {
                    let variant = roles_content.call(Ident::parse_any)?;
                    roles_content.parse::<Token![=]>()?;
                    let spelling = if roles_content.peek(LitStr) {
                        SignedDecimalSignSpelling::Literal(roles_content.parse()?)
                    } else {
                        let none = roles_content.call(Ident::parse_any)?;
                        if none != "none" {
                            return Err(syn::Error::new(
                                none.span(),
                                "signed_decimal sign spelling must be `none` or a string literal",
                            ));
                        }
                        SignedDecimalSignSpelling::None(none.span())
                    };
                    roles.push(SignedDecimalSignRoleSource { variant, spelling });
                    roles_content.parse::<Token![,]>()?;
                }
                content.parse::<Token![;]>()?;
                sign_type_slots.push(SignedDecimalSignTypeSource { slot, name, roles });
            }
            _ => {
                return Err(syn::Error::new(
                    slot.span(),
                    "signed_decimal recipe accepts only `magnitude` and `sign_type` fields",
                ));
            }
        }
    }
    Ok(GeneratedCodecRecipe::SignedDecimal(SignedDecimalSource {
        recipe,
        magnitude_slots,
        sign_type_slots,
    }))
}

fn unsigned_primitive_source(slot: Ident, primitive: Ident) -> UnsignedPrimitiveSource {
    match primitive.to_string().as_str() {
        "u32" => UnsignedPrimitiveSource::U32 { slot, primitive },
        "NonZeroU32" => UnsignedPrimitiveSource::NonZeroU32 { slot, primitive },
        _ => UnsignedPrimitiveSource::Unsupported { slot, primitive },
    }
}

fn parse_build_leaf(input: ParseStream<'_>) -> syn::Result<BuildLeaf> {
    let content;
    braced!(content in input);
    let mut pattern = None;
    let mut construct = None;
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(keyword::pattern) {
            let slot = content.parse::<keyword::pattern>()?;
            if pattern.is_some() {
                return Err(syn::Error::new(slot.span(), "duplicate pattern slot"));
            }
            content.parse::<Token![=]>()?;
            pattern = Some(content.call(Pat::parse_single)?);
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::construct) {
            let slot = content.parse::<keyword::construct>()?;
            if construct.is_some() {
                return Err(syn::Error::new(slot.span(), "duplicate construct slot"));
            }
            content.parse::<Token![=]>()?;
            construct = Some(content.parse()?);
            content.parse::<Token![;]>()?;
        } else {
            return Err(content.error("binding build requires pattern and construct slots"));
        }
    }
    let span = input.span();
    Ok(BuildLeaf {
        pattern: pattern.ok_or_else(|| syn::Error::new(span, "binding build requires pattern"))?,
        construct: construct
            .ok_or_else(|| syn::Error::new(span, "binding build requires construct"))?,
    })
}

fn parse_traversal(input: ParseStream<'_>) -> syn::Result<Traversal> {
    let content;
    braced!(content in input);
    let mut parts = Vec::new();
    let mut visit_order = Vec::new();
    let mut callback_mode = None;
    let mut argument = None;
    let mut variants = Vec::new();
    let mut fields = Vec::new();
    let mut calls = Vec::new();
    let mut branches = Vec::new();
    let mut leaf_callbacks = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(keyword::callback) {
            let slot = content.parse::<keyword::callback>()?;
            if callback_mode.is_some() {
                return Err(syn::Error::new(slot.span(), "duplicate callback slot"));
            }
            content.parse::<Token![=]>()?;
            callback_mode = Some(parse_visit_mode(&content)?);
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::argument) {
            let slot = content.parse::<keyword::argument>()?;
            if argument.is_some() {
                return Err(syn::Error::new(
                    slot.span(),
                    "duplicate traversal value slot",
                ));
            }
            content.parse::<Token![=]>()?;
            argument = Some(content.parse()?);
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::variant) {
            content.parse::<keyword::variant>()?;
            variants.push(content.parse()?);
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::field) {
            content.parse::<keyword::field>()?;
            let name = content.parse()?;
            content.parse::<Token![:]>()?;
            let value_type = content.parse()?;
            fields.push(TraversalField { name, value_type });
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::call) {
            content.parse::<keyword::call>()?;
            calls.push(parse_traversal_call(&content)?);
            content.parse::<Token![;]>()?;
        } else if content.peek(keyword::leaf) {
            content.parse::<keyword::leaf>()?;
            let name = content.parse()?;
            content.parse::<Token![:]>()?;
            let value_type = content.parse()?;
            content.parse::<Token![=]>()?;
            let mode = parse_visit_mode(&content)?;
            content.parse::<Token![;]>()?;
            leaf_callbacks.push(LeafCallback {
                name,
                value_type,
                mode,
            });
        } else if content.peek(Token![match]) {
            content.parse::<Token![match]>()?;
            let value = syn::Expr::Path(content.parse()?);
            let arms_content;
            braced!(arms_content in content);
            let mut arms = Vec::new();
            while !arms_content.is_empty() {
                if arms_content.peek(Token![_]) {
                    return Err(arms_content
                        .error("traversal match arm requires a closed variant-binding pattern"));
                }
                let variant: Path = arms_content.parse().map_err(|_| {
                    arms_content
                        .error("traversal match arm requires a closed variant-binding pattern")
                })?;
                let binding_content;
                parenthesized!(binding_content in arms_content);
                let binding = binding_content.parse()?;
                binding_content.parse::<Token![:]>()?;
                let value_type = binding_content.parse()?;
                if !binding_content.is_empty() {
                    return Err(binding_content
                        .error("closed variant-binding pattern accepts one typed binding"));
                }
                arms_content.parse::<Token![=>]>()?;
                let call = parse_traversal_call(&arms_content)?;
                arms.push(TraversalBranchArm {
                    variant,
                    binding,
                    value_type,
                    call,
                });
                arms_content.parse::<Token![,]>()?;
            }
            branches.push(TraversalBranch { value, arms });
        } else if content.peek(keyword::part) {
            content.parse::<keyword::part>()?;
            let name = content.parse()?;
            content.parse::<Token![=]>()?;
            let kind_ident: Ident = content.parse()?;
            let kind = match kind_ident.to_string().as_str() {
                "scalar" => TraversalKind::Scalar,
                "identity" => TraversalKind::Identity,
                "subtree" => TraversalKind::Subtree,
                _ => {
                    return Err(syn::Error::new(
                        kind_ident.span(),
                        "unknown traversal part kind",
                    ));
                }
            };
            let value_content;
            parenthesized!(value_content in content);
            let value = value_content.parse()?;
            if !value_content.is_empty() {
                return Err(value_content.error("traversal part accepts exactly one expression"));
            }
            content.parse::<Token![;]>()?;
            parts.push(TraversalPart { name, kind, value });
        } else if content.peek(keyword::visit) {
            content.parse::<keyword::visit>()?;
            visit_order.push(content.parse()?);
            content.parse::<Token![;]>()?;
        } else {
            return Err(content.error("expected part or visit in traversal schema"));
        }
    }
    Ok(Traversal {
        parts,
        visit_order,
        callback_mode,
        argument,
        variants,
        fields,
        calls,
        branches,
        leaf_callbacks,
    })
}

fn parse_visit_mode(input: ParseStream<'_>) -> syn::Result<VisitMode> {
    if input.peek(keyword::copy) {
        input.parse::<keyword::copy>()?;
        Ok(VisitMode::Copy)
    } else if input.peek(keyword::borrowed) {
        input.parse::<keyword::borrowed>()?;
        Ok(VisitMode::Borrowed)
    } else {
        Err(input.error("visitor mode must be `copy` or `borrowed`"))
    }
}

fn parse_traversal_call(input: ParseStream<'_>) -> syn::Result<TraversalCall> {
    let callback = parse_generated_owned_callback_path(input)?;
    let content;
    parenthesized!(content in input);
    if !content.peek(keyword::copy) && !content.peek(keyword::borrowed) {
        return Err(generated_owned_path_error(callback.span()));
    }
    let mode = parse_visit_mode(&content)?;
    let value_content;
    parenthesized!(value_content in content);
    let value = value_content.parse()?;
    if !value_content.is_empty() || !content.is_empty() {
        return Err(content.error("traversal callback accepts one copy/borrowed expression"));
    }
    Ok(TraversalCall {
        callback,
        mode,
        value,
    })
}

fn parse_root(input: ParseStream<'_>) -> syn::Result<Root> {
    input.parse::<keyword::root>()?;
    let category = parse_generated_owned_path(input)?;
    let content;
    braced!(content in input);
    let mut punctuation = None;
    let mut eoi = None;
    let mut standalone_render = None;
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        let slot: Ident = content.parse()?;
        content.parse::<Token![=]>()?;
        match slot.to_string().as_str() {
            "punctuation" => {
                reject_duplicate(punctuation.as_ref(), &slot)?;
                punctuation = Some(content.parse()?);
            }
            "eoi" => {
                reject_duplicate(eoi.as_ref(), &slot)?;
                eoi = Some(content.parse::<LitBool>()?.value);
            }
            "standalone_render" => {
                reject_duplicate(standalone_render.as_ref(), &slot)?;
                standalone_render = Some(content.parse::<LitBool>()?.value);
            }
            _ => return Err(syn::Error::new(slot.span(), "unknown root metadata slot")),
        }
        content.parse::<Token![;]>()?;
    }
    let span = category.span();
    Ok(Root {
        category,
        punctuation,
        eoi: eoi.ok_or_else(|| syn::Error::new(span, "root requires eoi"))?,
        standalone_render: standalone_render
            .ok_or_else(|| syn::Error::new(span, "root requires standalone_render"))?,
    })
}

fn parse_generated_owned_path(input: ParseStream<'_>) -> syn::Result<Path> {
    if input.peek(Token![<]) {
        return Err(generated_owned_path_error(input.span()));
    }
    let path = input.parse()?;
    if input.peek(syn::token::Paren) {
        return Err(generated_owned_path_error(input.span()));
    }
    Ok(path)
}

fn peek_ident(input: ParseStream<'_>, expected: &str) -> bool {
    let fork = input.fork();
    fork.call(Ident::parse_any)
        .is_ok_and(|ident| ident == expected)
}

fn parse_generated_owned_callback_path(input: ParseStream<'_>) -> syn::Result<Path> {
    if input.peek(Token![<]) {
        return Err(generated_owned_path_error(input.span()));
    }
    input.parse()
}

fn generated_owned_path_error(span: proc_macro2::Span) -> syn::Error {
    syn::Error::new(
        span,
        "compiler-generated identity requires a qself-free, non-generic identifier path",
    )
}

fn reject_duplicate<T>(slot: Option<&T>, name: &Ident) -> syn::Result<()> {
    if slot.is_some() {
        Err(syn::Error::new(
            name.span(),
            format!("duplicate {name} slot"),
        ))
    } else {
        Ok(())
    }
}

fn combine(errors: &mut Option<syn::Error>, error: syn::Error) {
    if let Some(existing) = errors {
        existing.combine(error);
    } else {
        *errors = Some(error);
    }
}

fn required<T>(slot: Option<T>, name: &Ident, slot_name: &str) -> syn::Result<T> {
    slot.ok_or_else(|| {
        syn::Error::new(
            name.span(),
            format!("binding requires explicit {slot_name} slot"),
        )
    })
}

fn reject_doc_comment(input: ParseStream<'_>) -> syn::Result<()> {
    if input.peek(Token![#]) {
        Err(deferred(input.span(), "doc comments"))
    } else {
        Ok(())
    }
}

fn deferred(span: proc_macro2::Span, construct: &str) -> syn::Error {
    syn::Error::new(span, format!("{construct} unimplemented in MVP"))
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::too_many_lines,
        reason = "parser fixtures pin one complete declaration stream"
    )]
    use quote::ToTokens;

    use crate::Declaration;
    use crate::Feature;
    use crate::FeaturePlace;
    use crate::FeatureValue;
    use crate::FormAtom;
    use crate::FormGuardSource;
    use crate::RequireExprSource;
    use crate::RequireSubjectSource;
    use crate::TraversalKind;
    use crate::VerbOperand;

    const GOLDEN_SHAPED_DECLARATIONS: &str = r#"
        construction triggered: Ability {
            element Triggered {
                trigger: lex TriggerWord,
                event: Clause,
                effect: Sentence,
            }
            require event is Event;
            form triggered = lex(trigger) event "," effect;
        }

        construction with_where: Sentence {
            element WithWhere {
                body: Sentence,
                clause: Clause,
            }
            require clause is Where;
            form with_where = body "," clause;
        }

        construction count: NounPhrase {
            element CountNp {
                head: lex Noun,
                controller: lex Pronoun,
                threshold: lex SignedNumber,
            }
            require controller is You;
            derive number = NounNumber::Plural;
            form count = noun(head) lex(controller) verb(VerbLexeme::Control)
                "with" "power" lex(threshold) "or" "less";
        }

        construction imperative: Sentence {
            element Imperative { predicate: VerbPhrase, }
            derive concord_class = ConcordClass::Other;
            form imperative = predicate;
        }

        construction declarative: Sentence {
            element Declarative {
                subject: NounPhrase,
                predicate: VerbPhrase,
            }
            derive predicate.concord_class = subject.concord_class;
            form declarative = subject predicate;
        }

        construction demonstrative: NounPhrase {
            element DemonstrativeNp {
                word: lex Demonstrative,
                head: lex Noun,
            }
            derive number = match word {
                That => NounNumber::Singular,
                Those => NounNumber::Plural,
            };
            form demonstrative = lex(word) noun(head);
        }

        vocab Pronoun {
            You = "you",
            It = "it",
        }

        lexeme VerbLexeme using EnglishVerb {
            Be = "be",
            Deal = "deal",
        }

        codec SignedNumber {
            atom = lex;
            value_type = crate::ast::SignedNumber;
            lexical = Lexical::SignedNumber;
            render = crate::render::render_signed_number;
            build {
                pattern = BuildValue::SignedNumber(value);
                construct = value;
            }
            traversal {
                part sign = scalar(value.sign);
                part whole = scalar(value.whole);
                visit sign;
                visit whole;
            }
        }

        identity CatalogIdentity {
            value_type = crate::ast::CatalogIdentity;
            lexical = Lexical::CatalogIdentity;
            render = crate::render::render_catalog_identity;
            build {
                pattern = BuildValue::CatalogIdentity(identity, spelling);
                construct = crate::ast::CatalogIdentity::new(identity, spelling);
            }
            traversal {
                part identity = identity(identity);
                part spelling = scalar(spelling);
                visit identity;
                visit spelling;
            }
        }

        root crate::ast::Ability {
            punctuation = ".";
            eoi = true;
            standalone_render = true;
        }
    "#;

    fn parse(input: &str) -> syn::Result<crate::Declarations> {
        crate::parse_declarations(input.parse().expect("test declaration tokenizes"))
    }

    fn path(path: &syn::Path) -> String {
        path.to_token_stream().to_string()
    }

    fn construction_with_requirement(requirement: &str) -> String {
        format!(
            "construction predicate: Predicate {{ element PredicateNode {{ subject: Subject, }} {requirement} form predicate = subject; }}"
        )
    }

    #[test]
    fn parses_checked_required_category_field() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction checked: Root {
                element Checked {
                    head: Determinative checked by determinative_is_fused(head.fused_head_license),
                }
                form checked = head;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("checked required category field parses");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("first declaration is a construction");
        };
        let field = &construction.element.fields[0];
        assert!(
            matches!(&field.kind, crate::FieldKind::Category(path) if path.is_ident("Determinative"))
        );
        let check = field.check.as_ref().expect("field carries its callback");
        assert_eq!(path(&check.function), "determinative_is_fused");
        assert!(matches!(
            check.arguments.as_slice(),
            [crate::FeatureSlot { role, feature: crate::Feature::FusedHeadLicense }]
                if role == "head"
        ));
    }

    #[test]
    fn parses_checked_required_lexical_field() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction checked: Root {
                element Checked {
                    head: lex DeterminativeHead
                        checked by determinative_is_fused(head.fused_head_license),
                }
                form checked = lex(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("checked required lexical field parses");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("first declaration is a construction");
        };
        let field = &construction.element.fields[0];
        assert!(
            matches!(&field.kind, crate::FieldKind::Lex(path) if path.is_ident("DeterminativeHead"))
        );
        assert!(matches!(
            field.check.as_ref().map(|check| check.arguments.as_slice()),
            Some([crate::FeatureSlot { role, feature: crate::Feature::FusedHeadLicense }])
                if role == "head"
        ));
    }

    #[test]
    fn parses_mobile_role_annotation_without_changing_its_value_kind() {
        let declarations = parse(
            r"
                construction host: Root {
                    element Host { tail: mobile Child, }
                    form host = tail;
                }
            ",
        )
        .expect("mobile role annotation parses");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("first declaration is a construction");
        };
        let [field] = construction.element.fields.as_slice() else {
            panic!("Host has one field");
        };
        assert_eq!(field.name, "tail");
        assert!(field.mobile);
        assert!(matches!(
            &field.kind,
            crate::FieldKind::Category(path) if path.is_ident("Child")
        ));
    }

    #[test]
    fn parses_structural_declarations() {
        let declarations = parse(
            r#"
                abstract sum Choice { Left: LeftNode, Right: RightNode, }
                abstract product Holder {
                    maybe: opt LeftNode,
                    items: seq Choice
                        separated by position {
                            pair = " and ";
                            first = ", ";
                            middle = ", ";
                            last = ", and ";
                        }
                        terminated by ".",
                }
                require len(Holder.items) >= 1;

                construction sentence: Sentence {
                    element SentenceNode {
                        words: seq Sentence separated by " " terminated by ".",
                    }
                    form sentence = words;
                }

                abstract product SurfaceAtoms {
                    items: seq Choice separated by " " lex(SeparatorWord::A),
                }
                abstract sum DocumentBlock { Ability, }
            "#,
        )
        .expect("structural declaration source parses");

        assert_eq!(declarations.declarations.len(), 5);
        let Declaration::AbstractSum(choice) = &declarations.declarations[0] else {
            panic!("first declaration is Choice");
        };
        assert_eq!(choice.name, "Choice");
        assert!(matches!(
            choice.alternatives.as_slice(),
            [crate::AbstractAlternative { name, value_type }, crate::AbstractAlternative { name: right_name, value_type: right_type }]
                if name == "Left" && value_type.is_ident("LeftNode")
                    && right_name == "Right" && right_type.is_ident("RightNode")
        ));

        let Declaration::AbstractProduct(holder) = &declarations.declarations[1] else {
            panic!("second declaration is Holder");
        };
        assert_eq!(holder.name, "Holder");
        assert!(matches!(
            &holder.fields[0],
            crate::Field {
                name,
                kind: crate::FieldKind::Optional(item),
                mobile: false,
                check: None,
            }
                if name == "maybe"
                    && matches!(item.as_ref(), crate::FieldKind::Category(path) if path.is_ident("LeftNode"))
        ));
        let crate::FieldKind::Sequence { item, surface } = &holder.fields[1].kind else {
            panic!("Holder.items is a sequence");
        };
        assert_eq!(holder.fields[1].name, "items");
        assert!(
            matches!(item.as_ref(), crate::FieldKind::Category(path) if path.is_ident("Choice"))
        );
        let Some(crate::SeparatorSource::Positional(rows)) = &surface.separator else {
            panic!("Holder.items has positional separators");
        };
        assert_eq!(rows.len(), 4);
        for ((row, class), literal) in rows
            .iter()
            .zip(["pair", "first", "middle", "last"])
            .zip([" and ", ", ", ", ", ", and "])
        {
            assert_eq!(row.class, class);
            assert!(
                matches!(row.surface.atoms.as_slice(), [crate::FixedSurfaceAtomSource::Literal(value)] if value.value() == literal)
            );
        }
        assert!(
            matches!(surface.terminator.as_ref(), Some(crate::FixedSurfaceSource { atoms, .. }) if matches!(atoms.as_slice(), [crate::FixedSurfaceAtomSource::Literal(value)] if value.value() == "."))
        );
        assert!(matches!(
            holder.requirements.as_slice(),
            [RequireExprSource::Length { owner: Some(owner), role, comparison: crate::LengthComparison::GreaterThanOrEqual, value }]
                if owner.is_ident("Holder") && role == "items" && value.base10_parse::<u8>().is_ok_and(|number| number == 1)
        ));

        let Declaration::Construction(sentence) = &declarations.declarations[2] else {
            panic!("third declaration is sentence");
        };
        assert!(matches!(
            sentence.element.fields.as_slice(),
            [crate::Field {
                name,
                kind: crate::FieldKind::Sequence { item, surface },
                mobile: false,
                check: None,
            }]
                if name == "words"
                    && matches!(item.as_ref(), crate::FieldKind::Category(path) if path.is_ident("Sentence"))
                    && matches!(surface.separator.as_ref(), Some(crate::SeparatorSource::Uniform(crate::FixedSurfaceSource { atoms, .. }))
                        if matches!(atoms.as_slice(), [crate::FixedSurfaceAtomSource::Literal(value)] if value.value() == " "))
                    && matches!(surface.terminator.as_ref(), Some(crate::FixedSurfaceSource { atoms, .. })
                        if matches!(atoms.as_slice(), [crate::FixedSurfaceAtomSource::Literal(value)] if value.value() == "."))
        ));

        let Declaration::AbstractProduct(surface_atoms) = &declarations.declarations[3] else {
            panic!("fourth declaration is SurfaceAtoms");
        };
        let crate::FieldKind::Sequence { surface, .. } = &surface_atoms.fields[0].kind else {
            panic!("SurfaceAtoms.items is a sequence");
        };
        let Some(crate::SeparatorSource::Uniform(crate::FixedSurfaceSource { atoms, .. })) =
            &surface.separator
        else {
            panic!("SurfaceAtoms.items has a uniform separator");
        };
        let [
            crate::FixedSurfaceAtomSource::Literal(space),
            crate::FixedSurfaceAtomSource::Lex(article),
        ] = atoms.as_slice()
        else {
            panic!("separator retains its literal and fixed terminal atoms");
        };
        assert_eq!(space.value(), " ");
        assert_eq!(article.segments.len(), 2);
        assert_eq!(article.segments[0].ident, "SeparatorWord");
        assert_eq!(article.segments[1].ident, "A");

        let Declaration::AbstractSum(document_block) = &declarations.declarations[4] else {
            panic!("fifth declaration is DocumentBlock");
        };
        assert!(matches!(
            document_block.alternatives.as_slice(),
            [crate::AbstractAlternative { name, value_type }]
                if name == "Ability" && value_type.is_ident("Ability")
        ));
    }

    #[test]
    fn parses_structural_rejects_nested_cardinality() {
        for (source, spelling, expected, wrong_owner) in [
            (
                "abstract product AbstractOwner { value: opt opt Node, }",
                "opt opt Node",
                "field `AbstractOwner.value`: opt opt Node",
                "ElementOwner.value",
            ),
            (
                "construction build: Category { element ElementOwner { value: seq opt Node, } form build = value; }",
                "seq opt Node",
                "field `ElementOwner.value`: seq opt Node",
                "AbstractOwner.value",
            ),
            (
                "construction build: Category { element ElementOwner { value: seq seq Node, } form build = value; }",
                "seq seq Node",
                "field `ElementOwner.value`: seq seq Node",
                "AbstractOwner.value",
            ),
        ] {
            let error = parse(source)
                .expect_err("nested cardinality must be rejected by the parser")
                .to_string();
            assert!(error.contains(expected), "{spelling}: {error}");
            assert!(
                !error.contains(wrong_owner),
                "{spelling} must identify its own owner rather than `{wrong_owner}`: {error}"
            );
        }
    }

    #[test]
    fn parses_unqualified_length_and_root_without_punctuation() {
        let declarations = parse(
            r"
                abstract product Holder { items: seq Node, }
                require len(items) = 0;
                root Sentence { eoi = true; standalone_render = true; }
            ",
        )
        .expect("source-level structural rows parse before semantic validation");

        let Declaration::AbstractProduct(holder) = &declarations.declarations[0] else {
            panic!("first declaration is Holder");
        };
        assert!(matches!(
            holder.requirements.as_slice(),
            [RequireExprSource::Length { owner: None, role, comparison: crate::LengthComparison::Equal, value }]
                if role == "items" && value.base10_parse::<u8>().is_ok_and(|number| number == 0)
        ));
        let Declaration::Root(root) = &declarations.declarations[1] else {
            panic!("second declaration is root");
        };
        assert!(root.punctuation.is_none());
    }

    #[test]
    fn require_parses_the_closed_predicate_language() {
        let cases = [
            "require event is Event;",
            "require controller in [You];",
            "require all(mode in [One, Two], concord_class is Other);",
            "require any(subject.number is Singular, subject.number is Plural);",
            "require optional.is_some();",
            "require optional.is_none();",
        ];

        for requirement in cases {
            parse(&construction_with_requirement(requirement))
                .unwrap_or_else(|error| panic!("{requirement}: {error}"));
        }
    }

    #[test]
    fn require_preserves_the_complete_predicate_tree_and_source_order() {
        let declarations = parse(
            r"
                construction predicate: Predicate {
                    element PredicateNode { subject: Subject, }
                    require event is Event;
                    require controller in [You];
                    require all(mode in [One, Two], concord_class is Other);
                    require any(subject.number is Singular, subject.number is Plural);
                    require optional.is_some();
                    require optional.is_none();
                    form predicate = subject;
                }
            ",
        )
        .expect("closed predicates parse");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("fixture contains one construction");
        };

        assert_eq!(construction.requirements.len(), 6);
        assert!(matches!(
            &construction.requirements[0],
            RequireExprSource::In { subject: RequireSubjectSource::Role(role), members }
                if role == "event" && members.len() == 1 && members[0] == "Event"
        ));
        assert!(matches!(
            &construction.requirements[1],
            RequireExprSource::In { subject: RequireSubjectSource::Role(role), members }
                if role == "controller" && members.len() == 1 && members[0] == "You"
        ));
        assert!(matches!(
            &construction.requirements[2],
            RequireExprSource::All(operands)
                if matches!(
                    &operands[0],
                    RequireExprSource::In {
                        subject: RequireSubjectSource::Role(role), members,
                    } if role == "mode" && members.iter().map(ToString::to_string).collect::<Vec<_>>() == ["One", "Two"]
                ) && matches!(
                    &operands[1],
                    RequireExprSource::In {
                        subject: RequireSubjectSource::ConstructionFeature(Feature::ConcordClass), members,
                    } if members.len() == 1 && members[0] == "Other"
                )
        ));
        assert!(matches!(
            &construction.requirements[3],
            RequireExprSource::Any(operands)
                if operands.iter().map(|operand| match operand {
                    RequireExprSource::In {
                        subject: RequireSubjectSource::RoleFeature { role, feature: Feature::Number },
                        members,
                    } if role == "subject" && members.len() == 1 => members[0].to_string(),
                    _ => String::new(),
                }).collect::<Vec<_>>() == ["Singular", "Plural"]
        ));
        assert!(matches!(
            &construction.requirements[4],
            RequireExprSource::OptionalPresence { role, present: true }
                if role == "optional"
        ));
        assert!(matches!(
            &construction.requirements[5],
            RequireExprSource::OptionalPresence { role, present: false }
                if role == "optional"
        ));
    }

    #[test]
    fn require_rejects_malformed_and_deferred_predicates() {
        let cases = [
            ("require controller in [];", "at least one member"),
            ("require controller in [You,, Opponent];", "expected ident"),
            (
                "require controller in [You, You];",
                "duplicate require member `You`",
            ),
            ("require all(mode is One);", "at least two operands"),
            ("require any(mode is One);", "at least two operands"),
            (
                "require controller::kind is You;",
                "Plan 05 structural declarations",
            ),
            (
                "require subject.gender is Masculine;",
                "unknown require feature",
            ),
            (
                "require members.len() is Three;",
                "Plan 05 structural declarations",
            ),
            (
                "require optional.is_some(Present);",
                "optional-presence requirements accept no arguments",
            ),
            ("require mode == One;", "Plan 05 structural declarations"),
            (
                "require !controller is You;",
                "Plan 05 structural declarations",
            ),
            (
                "require callback(subject);",
                "Plan 05 structural declarations",
            ),
        ];

        for (requirement, expected) in cases {
            let error = parse(&construction_with_requirement(requirement))
                .expect_err("malformed predicate must be rejected")
                .to_string();
            assert!(error.contains(expected), "{requirement}: {error}");
        }
    }

    #[test]
    fn guarded_forms_parse_in_authored_order() {
        let declarations = parse(
            r"
                construction demonstrative: NounPhrase {
                    element DemonstrativeNp { word: lex Demonstrative, head: lex Noun, }
                    form that when word is That = lex(word) noun(head);
                    form those otherwise = lex(word) noun(head);
                }
            ",
        )
        .expect("guarded forms parse");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("fixture contains one construction");
        };
        assert_eq!(construction.forms.len(), 2);
        assert_eq!(construction.forms[0].name, "that");
        assert!(matches!(
            &construction.forms[0].guard,
            FormGuardSource::When(RequireExprSource::In {
                subject: RequireSubjectSource::Role(role),
                members,
            }) if role == "word" && matches!(members.as_slice(), [member] if member == "That")
        ));
        assert_eq!(construction.forms[1].name, "those");
        assert!(matches!(
            construction.forms[1].guard,
            FormGuardSource::Otherwise
        ));

        let declarations = parse(
            r"
                construction optional_mode: NounPhrase {
                    element OptionalMode { optional: opt NounPhrase, mode: lex Mode, }
                    form present when all(optional.is_some(), mode in [One, Two]) = lex(mode);
                    form absent otherwise = lex(mode);
                }
            ",
        )
        .expect("optional-presence guards compose with membership");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("fixture contains one construction");
        };
        assert!(matches!(
            &construction.forms[0].guard,
            FormGuardSource::When(RequireExprSource::All(operands))
                if matches!(operands.as_slice(), [
                    RequireExprSource::OptionalPresence { role, present: true },
                    RequireExprSource::In { subject: RequireSubjectSource::Role(mode), members },
                ] if role == "optional" && mode == "mode"
                    && members.iter().map(ToString::to_string).collect::<Vec<_>>() == ["One", "Two"])
        ));
    }

    #[test]
    fn guarded_forms_reject_invalid_source_shapes() {
        let cases = [
            (
                "form one = word; form two otherwise = word;",
                "unguarded form cannot be mixed",
            ),
            (
                "form one when word is One = word;",
                "require a final fallback",
            ),
            (
                "form one otherwise = word;",
                "requires at least one guarded form",
            ),
            (
                "form one otherwise = word; form two otherwise = word;",
                "multiple fallback",
            ),
            (
                "form one otherwise = word; form two when word is One = word;",
                "fallback form must be last",
            ),
            (
                "form one when word is One = word; form one otherwise = word;",
                "duplicate form name `one`",
            ),
            (
                "form one when len(word) = 1 = word; form two otherwise = word;",
                "do not support length subjects",
            ),
            (
                "form one when !word is One = word; form two otherwise = word;",
                "do not support negation",
            ),
            (
                "form one when callback(word) = word; form two otherwise = word;",
                "require a role subject",
            ),
        ];
        for (forms, expected) in cases {
            let source = format!(
                "construction guarded: Category {{ element Guarded {{ word: lex Word, }} {forms} }}"
            );
            let error = parse(&source)
                .expect_err("invalid guarded form declaration is rejected")
                .into_compile_error()
                .to_string();
            assert!(error.contains(expected), "{forms}: {error}");
        }
    }

    fn tokens(value: &impl ToTokens) -> String {
        value.to_token_stream().to_string()
    }

    #[test]
    fn generated_morphology_parses_spelled_lexemes() {
        let declarations = parse(
            r#"
                morphology EnglishVerb {
                    feature = ConcordClass;
                    recipe = english_verb;
                }
                morphology EnglishNoun {
                    feature = Number;
                    recipe = english_noun;
                }
                lexeme VerbLexeme using EnglishVerb {
                    Deal = "deal",
                    Be = "be" {
                        Other = "are",
                        ThirdPersonSingular = "is",
                    },
                }
                lexeme NounLexeme using EnglishNoun {
                    Player = "player",
                }
            "#,
        )
        .expect("sealed morphology and spelled lexemes parse");

        assert_eq!(declarations.declarations.len(), 4);
        let Declaration::Morphology(verb_morphology) = &declarations.declarations[0] else {
            panic!("first declaration is the verb morphology")
        };
        assert_eq!(verb_morphology.name, "EnglishVerb");
        assert_eq!(verb_morphology.feature, Feature::ConcordClass);
        assert_eq!(verb_morphology.recipe, "english_verb");
        let Declaration::Morphology(noun_morphology) = &declarations.declarations[1] else {
            panic!("second declaration is the noun morphology")
        };
        assert_eq!(noun_morphology.name, "EnglishNoun");
        assert_eq!(noun_morphology.feature, Feature::Number);
        assert_eq!(noun_morphology.recipe, "english_noun");

        let Declaration::Lexeme(verbs) = &declarations.declarations[2] else {
            panic!("third declaration is the verb lexeme")
        };
        assert_eq!(verbs.name, "VerbLexeme");
        assert_eq!(verbs.morphology, "EnglishVerb");
        assert_eq!(
            verbs
                .members
                .iter()
                .map(|member| (
                    member.name.to_string(),
                    member.lemma.value(),
                    member
                        .overrides
                        .iter()
                        .map(|row| (row.feature.to_string(), row.surface.value()))
                        .collect::<Vec<_>>(),
                ))
                .collect::<Vec<_>>(),
            [
                ("Deal".to_owned(), "deal".to_owned(), vec![]),
                (
                    "Be".to_owned(),
                    "be".to_owned(),
                    vec![
                        ("Other".to_owned(), "are".to_owned()),
                        ("ThirdPersonSingular".to_owned(), "is".to_owned()),
                    ],
                ),
            ]
        );
        let Declaration::Lexeme(nouns) = &declarations.declarations[3] else {
            panic!("fourth declaration is the noun lexeme")
        };
        assert_eq!(nouns.name, "NounLexeme");
        assert_eq!(nouns.morphology, "EnglishNoun");
        assert_eq!(nouns.members[0].name, "Player");
        assert_eq!(nouns.members[0].lemma.value(), "player");
        assert!(nouns.members[0].overrides.is_empty());
    }

    #[test]
    fn generated_participle_morphology_parses_as_one_sealed_axis() {
        let declarations = parse(
            r#"
                morphology EnglishParticiple {
                    feature = Participle;
                    recipe = english_participle;
                }
                lexeme ParticipleVerb using EnglishParticiple {
                    Deal = "deal" { Participle = "dealt", },
                    Turn = "turn",
                }
                construction passive: PassivePredicate {
                    element Passive { head: lex ParticipleVerb, }
                    form passive = lex(head);
                }
            "#,
        )
        .expect("the finite participle axis and its sole form atom parse");

        let Declaration::Morphology(morphology) = &declarations.declarations[0] else {
            panic!("first declaration is the participle morphology")
        };
        assert_eq!(morphology.feature, Feature::Participle);
        let Declaration::Construction(passive) = &declarations.declarations[2] else {
            panic!("third declaration is the passive frame")
        };
        assert!(matches!(
            &passive.forms[0].atoms[0],
            FormAtom::Lex(role) if role == "head"
        ));
    }

    #[test]
    fn generated_morphology_parses_rejects_recipe_paths() {
        let error = parse(
            r"
                morphology EnglishVerb {
                    feature = ConcordClass;
                    recipe = crate::english_verb;
                }
            ",
        )
        .expect_err("a morphology recipe must be one identifier")
        .to_string();

        assert!(error.contains("recipe"), "{error}");
    }

    #[test]
    fn generated_morphology_parses_requires_using() {
        let error = parse(r#"lexeme VerbLexeme { Deal = "deal", }"#)
            .expect_err("a lexeme must select a morphology")
            .to_string();

        assert!(error.contains("using"), "{error}");
    }

    #[test]
    fn generated_morphology_parses_requires_lemma() {
        let error = parse("lexeme VerbLexeme using EnglishVerb { Deal, }")
            .expect_err("a lexeme member must spell its lemma")
            .to_string();

        assert!(error.contains("lemma"), "{error}");
    }

    #[test]
    fn generated_morphology_parses_rejects_empty_surfaces() {
        for source in [
            r#"lexeme VerbLexeme using EnglishVerb { Deal = "", }"#,
            r#"lexeme VerbLexeme using EnglishVerb { Deal = "deal" { Other = "", }, }"#,
        ] {
            let error = parse(source)
                .expect_err("lexical surface literals must not be empty")
                .to_string();
            assert!(error.contains("empty"), "{error}");
        }
    }

    #[test]
    fn bound_prefix_and_suffix_atoms_parse_the_four_approved_shapes() {
        let declarations = crate::parse_declarations(quote::quote! {
            vocab Modifier { Black = "black", Elf = "Elf", }
            construction plain_prefix: Root {
                element PlainPrefix { modifier: lex Modifier, }
                form plain_prefix = prefix("non", lex(modifier));
            }
            construction hyphen_prefix: Root {
                element HyphenPrefix { modifier: lex Modifier, }
                form hyphen_prefix = prefix("non-", lex(modifier));
            }
            construction singular_suffix: Root {
                element SingularSuffix { owner: Root, }
                form singular_suffix = suffix(owner, "'s");
            }
            construction plural_suffix: Root {
                element PluralSuffix { owner: Root, }
                form plural_suffix = suffix(owner, "'");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the four closed bound-atom spellings parse");

        assert_eq!(declarations.declarations.len(), 6);
    }

    #[test]
    fn circumfix_atoms_parse_required_and_nonempty_sequence_roles() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction singular: Root {
                element Singular { value: Item, }
                form singular = circumfix("[", value, "]");
            }
            construction sequence: Root {
                element Sequence { values: seq Item separated by "}{", }
                require len(values) >= 1;
                form sequence = circumfix("{", values, "}");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("fixed circumfixes over singular and sequence roles parse");

        assert_eq!(declarations.declarations.len(), 4);
    }

    #[test]
    fn sentence_initial_fixed_surfaces_parse_for_forms_and_sequence_separators() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item" sentence_initial(":");
            }
            abstract product Items {
                values: seq Item separated by sentence_initial(", "),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("sentence-initial transition wrappers parse around exact fixed surfaces");

        let crate::Declaration::Construction(item) = &declarations.declarations[0] else {
            panic!("first declaration is the construction")
        };
        assert!(matches!(
            item.forms[0].atoms.as_slice(),
            [
                crate::FormAtom::Literal(_),
                crate::FormAtom::SentenceInitial(_)
            ]
        ));
        let crate::Declaration::AbstractProduct(items) = &declarations.declarations[1] else {
            panic!("second declaration is the product")
        };
        let crate::FieldKind::Sequence { surface, .. } = &items.fields[0].kind else {
            panic!("product field is a sequence")
        };
        let Some(crate::SeparatorSource::Uniform(separator)) = &surface.separator else {
            panic!("sequence has one uniform separator")
        };
        assert_eq!(
            separator.transition,
            crate::model::SurfaceCaseTransition::SentenceInitial,
        );

        for (malformed, expected) in [
            (
                quote::quote! { sentence_initial() },
                "unexpected end of input, sentence_initial form atoms require exactly one literal",
            ),
            (
                quote::quote! { sentence_initial(value) },
                "sentence_initial form atoms require exactly one literal",
            ),
            (
                quote::quote! { sentence_initial(sentence_initial(":")) },
                "sentence_initial form atoms require exactly one literal",
            ),
            (
                quote::quote! { sentence_initial(":", ",") },
                "sentence_initial form atoms require exactly one literal",
            ),
        ] {
            let source = quote::quote! {
                construction invalid: Root {
                    element Invalid { value: Root, }
                    form invalid = #malformed;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            };
            let error = crate::parse_declarations(source)
                .expect_err("invalid transition target, nesting, or duplication rejects")
                .to_string();
            assert_eq!(error, expected);
        }

        let error = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item separated by sentence_initial(sentence_initial(", ")),
            }
        })
        .expect_err("a duplicated fixed-surface transition annotation rejects")
        .to_string();
        assert_eq!(
            error,
            "unexpected end of input, sentence_initial accepts exactly one unnested fixed surface",
        );

        let error = crate::parse_declarations(quote::quote! {
            construction invalid: Root {
                element Invalid {}
                form invalid = "item" sentence_initial("");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a sentence-initial annotation cannot own zero bytes")
        .to_string();
        assert_eq!(
            error,
            "sentence_initial target must realize at least one byte",
        );
    }

    #[test]
    fn structural_form_surfaces_parse_as_nonempty_unnested_literals() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = structural(" ") "item";
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a structural form surface parses");

        let crate::Declaration::Construction(item) = &declarations.declarations[0] else {
            panic!("first declaration is the construction")
        };
        assert!(matches!(
            item.forms[0].atoms.as_slice(),
            [
                crate::FormAtom::StructuralLiteral(_),
                crate::FormAtom::Literal(_)
            ]
        ));

        for malformed in [
            quote::quote! { structural() },
            quote::quote! { structural(value) },
            quote::quote! { structural("") },
            quote::quote! { structural(structural(" ")) },
            quote::quote! { structural(" ", " ") },
        ] {
            let source = quote::quote! {
                construction invalid: Root {
                    element Invalid {}
                    form invalid = #malformed;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            };
            assert!(
                crate::parse_declarations(source).is_err(),
                "invalid structural form surface rejects: {malformed}",
            );
        }
    }

    #[test]
    fn licensed_fixed_surfaces_parse_only_as_unnested_form_atoms() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = licensed("item");
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a licensed fixed surface parses as a form atom");

        let crate::Declaration::Construction(item) = &declarations.declarations[0] else {
            panic!("first declaration is the construction")
        };
        assert!(matches!(
            item.forms[0].atoms.as_slice(),
            [crate::FormAtom::LicensedLiteral(_)]
        ));

        for (malformed, expected) in [
            (
                quote::quote! { licensed() },
                "unexpected end of input, licensed form atoms require exactly one literal",
            ),
            (
                quote::quote! { licensed(value) },
                "licensed form atoms require exactly one literal",
            ),
            (
                quote::quote! { licensed(licensed("item")) },
                "licensed form atoms require exactly one literal",
            ),
            (
                quote::quote! { licensed("item", "other") },
                "licensed form atoms require exactly one literal",
            ),
        ] {
            let source = quote::quote! {
                construction invalid: Root {
                    element Invalid { value: Root, }
                    form invalid = #malformed;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            };
            let error = crate::parse_declarations(source)
                .expect_err("an invalid licensed fixed surface rejects")
                .to_string();
            assert_eq!(error, expected);
        }
    }

    #[test]
    fn continuation_fixed_surface_parses_only_for_sequence_separators() {
        let declarations = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item separated by continuation(" Then ") terminated by ".",
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("continuation transition parses on an exact sequence separator");

        let crate::Declaration::AbstractProduct(items) = &declarations.declarations[1] else {
            panic!("second declaration is the structural sequence")
        };
        let crate::FieldKind::Sequence { surface, .. } = &items.fields[0].kind else {
            panic!("the structural field is a sequence")
        };
        let Some(crate::SeparatorSource::Uniform(separator)) = &surface.separator else {
            panic!("the sequence has one uniform separator")
        };
        assert_eq!(
            separator.transition,
            crate::model::SurfaceCaseTransition::Continuation,
        );

        for malformed in [
            quote::quote! { continuation(sentence_initial(" Then ")) },
            quote::quote! { sentence_initial(continuation(" Then ")) },
            quote::quote! { continuation(continuation(" Then ")) },
        ] {
            let source = quote::quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                abstract product Items {
                    values: seq Item separated by #malformed,
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            };
            assert!(
                crate::parse_declarations(source).is_err(),
                "case transitions cannot nest: {malformed}",
            );
        }

        let misplaced = crate::parse_declarations(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = continuation("item");
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            misplaced.is_err(),
            "continuation is not a general-purpose form atom"
        );
    }

    #[test]
    fn circumfix_atoms_reject_nonroles_extra_operands_and_nesting() {
        let rejected = [
            (
                quote::quote! { circumfix("[", "value", "]") },
                "ordinary declared roles",
            ),
            (
                quote::quote! { circumfix("[", verb(Verbs::Act), "]") },
                "ordinary declared roles",
            ),
            (
                quote::quote! { circumfix("[", open_verb(KeywordAction, "act"), "]") },
                "ordinary declared roles",
            ),
            (
                quote::quote! { circumfix("[", callback(value), "]") },
                "unknown form atom",
            ),
            (
                quote::quote! { circumfix("[", value, "]", "extra") },
                "exactly two fixed affixes and one role",
            ),
            (
                quote::quote! { circumfix("[", prefix("non", value), "]") },
                "bound atoms cannot nest",
            ),
            (
                quote::quote! { circumfix("[", circumfix("{", value, "}"), "]") },
                "circumfix atoms cannot nest",
            ),
            (
                quote::quote! { prefix("non", circumfix("[", value, "]")) },
                "circumfix atoms cannot nest",
            ),
        ];

        for (atom, diagnostic) in rejected {
            let source = quote::quote! {
                construction invalid: Root {
                    element Invalid { value: Root, }
                    form invalid = #atom;
                }
            };
            let error = crate::parse_declarations(source)
                .expect_err("forbidden circumfix shapes must be rejected")
                .to_string();
            assert!(error.contains(diagnostic), "{error}");
        }
    }

    #[test]
    fn bound_atoms_reject_nesting_extra_values_and_callbacks() {
        let rejected = [
            (
                quote::quote! { prefix("non", suffix(owner, "'s")) },
                "bound atoms cannot nest",
            ),
            (
                quote::quote! { prefix("non", owner, lex(modifier)) },
                "bound atoms accept exactly one fixed affix and one value atom",
            ),
            (
                quote::quote! { prefix("non", callback(owner)) },
                "unknown form atom",
            ),
        ];

        for (atom, diagnostic) in rejected {
            let source = quote::quote! {
                construction invalid: Root {
                    element Invalid { owner: Root, modifier: lex Modifier, }
                    form invalid = #atom;
                }
            };
            let error = crate::parse_declarations(source)
                .expect_err("non-value bound atom shapes must be rejected")
                .to_string();
            assert!(error.contains(diagnostic), "{error}");
        }
    }

    #[test]
    fn top_level_fallback_lists_every_supported_declaration() {
        let error = parse("unsupported")
            .expect_err("an unknown top-level declaration is rejected")
            .to_string();

        assert_eq!(
            error,
            "expected construction, abstract, vocab, morphology, lexeme, codec, identity, or root declaration"
        );
    }

    #[test]
    fn parses_golden_shaped_mvp_stream_without_losing_data() {
        let declarations =
            parse(GOLDEN_SHAPED_DECLARATIONS).expect("golden-shaped MVP declarations parse");
        assert_eq!(declarations.declarations.len(), 11);

        let Declaration::Construction(triggered) = &declarations.declarations[0] else {
            panic!("first declaration should remain the construction");
        };
        assert_eq!(triggered.name, "triggered");
        assert_eq!(path(&triggered.category), "Ability");
        assert_eq!(
            triggered
                .element
                .fields
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<_>>(),
            ["trigger", "event", "effect"]
        );

        assert_eq!(
            declarations.declarations[0..3]
                .iter()
                .map(|declaration| {
                    let Declaration::Construction(construction) = declaration else {
                        panic!("first declarations are constructions");
                    };
                    let Some((role, variant)) = construction.requirements[0].as_role_refinement()
                    else {
                        panic!("golden requirements are role refinements");
                    };
                    (role.to_string(), variant.to_string())
                })
                .collect::<Vec<_>>(),
            [
                ("event".into(), "Event".into()),
                ("clause".into(), "Where".into()),
                ("controller".into(), "You".into())
            ]
        );

        assert!(
            matches!(triggered.forms[0].atoms[0], FormAtom::Lex(ref role) if role == "trigger")
        );
        assert!(matches!(triggered.forms[0].atoms[1], FormAtom::Role(ref role) if role == "event"));
        assert!(
            matches!(triggered.forms[0].atoms[2], FormAtom::Literal(ref word) if word.value() == ",")
        );
        assert!(
            matches!(triggered.forms[0].atoms[3], FormAtom::Role(ref role) if role == "effect")
        );

        let Declaration::Construction(count) = &declarations.declarations[2] else {
            panic!("third declaration is count");
        };
        let FormAtom::Verb(VerbOperand::Fixed(control)) = &count.forms[0].atoms[2] else {
            panic!("count uses a fixed control verb");
        };
        assert_eq!(path(control), "VerbLexeme :: Control");

        let Declaration::Construction(imperative) = &declarations.declarations[3] else {
            panic!("fourth declaration is imperative");
        };
        assert!(matches!(
            imperative.equations[0].target,
            FeaturePlace::Construction(Feature::ConcordClass)
        ));
        let FeatureValue::Constant(imperative_value) = &imperative.equations[0].value else {
            panic!("imperative concord_class is constant");
        };
        assert_eq!(path(imperative_value), "ConcordClass :: Other");

        let Declaration::Construction(declarative) = &declarations.declarations[4] else {
            panic!("fifth declaration is declarative");
        };
        assert!(matches!(
            &declarative.equations[0].target,
            FeaturePlace::Role { field, feature: Feature::ConcordClass } if field == "predicate"
        ));
        let FeatureValue::FromRole(source) = &declarative.equations[0].value else {
            panic!("declarative concord_class comes from its subject");
        };
        assert_eq!(source.role, "subject");
        assert_eq!(source.feature, Feature::ConcordClass);

        let Declaration::Construction(demonstrative) = &declarations.declarations[5] else {
            panic!("sixth declaration is demonstrative");
        };
        assert!(matches!(
            demonstrative.equations[0].target,
            FeaturePlace::Construction(Feature::Number)
        ));
        let FeatureValue::Match { role, arms } = &demonstrative.equations[0].value else {
            panic!("demonstrative number retains its match equation");
        };
        assert_eq!(role, "word");
        assert_eq!(
            arms.iter()
                .map(|arm| (arm.variant.to_string(), path(&arm.value)))
                .collect::<Vec<_>>(),
            [
                ("That".into(), "NounNumber :: Singular".into()),
                ("Those".into(), "NounNumber :: Plural".into()),
            ]
        );

        let Declaration::Codec(codec) = &declarations.declarations[8] else {
            panic!("codec should retain its source position");
        };
        assert_eq!(codec.codec_atom, Some(crate::CodecAtomClass::Lex));
        assert_eq!(
            path(codec.value_type.path()),
            "crate :: ast :: SignedNumber"
        );
        assert_eq!(
            path(codec.lexical_variant.as_ref().unwrap()),
            "Lexical :: SignedNumber"
        );
        let Some(crate::RenderBinding::Runtime(render)) = codec.render.as_ref() else {
            panic!("codec has a runtime render binding");
        };
        assert_eq!(path(render), "crate :: render :: render_signed_number");
        let build = codec.build.as_ref().unwrap();
        assert_eq!(tokens(&build.pattern), "BuildValue :: SignedNumber (value)");
        assert_eq!(tokens(&build.construct), "value");
        assert_eq!(
            codec
                .traversal
                .parts
                .iter()
                .map(|part| (part.name.to_string(), part.kind, tokens(&part.value)))
                .collect::<Vec<_>>(),
            [
                ("sign".into(), TraversalKind::Scalar, "value . sign".into()),
                (
                    "whole".into(),
                    TraversalKind::Scalar,
                    "value . whole".into()
                ),
            ]
        );
        assert_eq!(
            codec
                .traversal
                .visit_order
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["sign", "whole"]
        );

        let Declaration::Identity(identity) = &declarations.declarations[9] else {
            panic!("identity should retain its source position");
        };
        assert_eq!(identity.codec_atom, None);
        assert_eq!(
            path(identity.value_type.path()),
            "crate :: ast :: CatalogIdentity"
        );
        assert_eq!(
            path(identity.lexical_variant.as_ref().unwrap()),
            "Lexical :: CatalogIdentity"
        );
        let Some(crate::RenderBinding::Runtime(render)) = identity.render.as_ref() else {
            panic!("identity has a runtime render binding");
        };
        assert_eq!(path(render), "crate :: render :: render_catalog_identity");
        let build = identity.build.as_ref().unwrap();
        assert_eq!(
            tokens(&build.pattern),
            "BuildValue :: CatalogIdentity (identity , spelling)"
        );
        assert_eq!(
            tokens(&build.construct),
            "crate :: ast :: CatalogIdentity :: new (identity , spelling)"
        );
        assert_eq!(
            identity
                .traversal
                .parts
                .iter()
                .map(|part| (part.name.to_string(), part.kind, tokens(&part.value)))
                .collect::<Vec<_>>(),
            [
                (
                    "identity".into(),
                    TraversalKind::Identity,
                    "identity".into()
                ),
                ("spelling".into(), TraversalKind::Scalar, "spelling".into()),
            ]
        );
        assert_eq!(
            identity
                .traversal
                .visit_order
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["identity", "spelling"]
        );

        let Declaration::Root(root) = &declarations.declarations[10] else {
            panic!("root should retain its source position");
        };
        assert_eq!(path(&root.category), "crate :: ast :: Ability");
        assert_eq!(
            root.punctuation.as_ref().map(syn::LitStr::value).as_deref(),
            Some(".")
        );
        assert!(root.eoi);
        assert!(root.standalone_render);
    }

    #[test]
    fn checked_metadata_is_rejected_with_the_retirement_diagnostic() {
        let error = parse(
            r#"
                construction only: Root {
                    element Only {}
                    checked {}
                    form only = "only";
                }
            "#,
        )
        .expect_err("retired checked metadata is rejected")
        .to_string();

        assert_eq!(
            error,
            "`checked` metadata was retired after Stage 4; use generated invariants"
        );
    }

    trait TypePath {
        fn path(&self) -> &syn::Path;
    }

    impl TypePath for syn::Type {
        fn path(&self) -> &syn::Path {
            let syn::Type::Path(path) = self else {
                panic!("fixture uses a path type");
            };
            &path.path
        }
    }

    #[test]
    fn feature_places_accept_construction_and_authorized_role_targets_only() {
        let declarations = parse(
            r"
                construction imperative: Sentence {
                    element Imperative { predicate: VerbPhrase, }
                    derive concord_class = ConcordClass::Other;
                    form imperative = predicate;
                }
                construction count: NounPhrase {
                    element CountNp { head: lex Noun, }
                    derive number = NounNumber::Plural;
                    form count = noun(head);
                }
                construction declarative: Sentence {
                    element Declarative {
                        subject: NounPhrase,
                        predicate: VerbPhrase,
                    }
                    derive predicate.concord_class = subject.concord_class;
                    form declarative = subject predicate;
                }
            ",
        )
        .expect("construction and role concord_class targets are in the MVP");

        let expected = [
            FeaturePlace::Construction(Feature::ConcordClass),
            FeaturePlace::Construction(Feature::Number),
            FeaturePlace::Role {
                field: syn::parse_str("predicate").expect("identifier"),
                feature: Feature::ConcordClass,
            },
        ];
        for (declaration, expected) in declarations.declarations.iter().zip(expected) {
            let Declaration::Construction(construction) = declaration else {
                panic!("fixtures contain only constructions");
            };
            match (&construction.equations[0].target, expected) {
                (FeaturePlace::Construction(actual), FeaturePlace::Construction(expected)) => {
                    assert_eq!(*actual, expected);
                }
                (
                    FeaturePlace::Role { field, feature },
                    FeaturePlace::Role {
                        field: expected_field,
                        feature: expected_feature,
                    },
                ) => {
                    assert_eq!(field, &expected_field);
                    assert_eq!(*feature, expected_feature);
                }
                (actual, expected) => panic!("feature target mismatch: {actual:?} != {expected:?}"),
            }
        }

        let Declaration::Construction(declarative) = &declarations.declarations[2] else {
            panic!("third declaration is declarative");
        };
        let FeatureValue::FromRole(source) = &declarative.equations[0].value else {
            panic!("declarative equation should retain its role source");
        };
        assert_eq!(source.role, "subject");
        assert_eq!(source.feature, Feature::ConcordClass);

        let role_number = parse(
            "construction x: X { element XNode { role: X, } derive role.number = NounNumber::Plural; form x = role; }",
        )
        .expect("role-level Number is an architecture-authorized target");
        let Declaration::Construction(role_number) = &role_number.declarations[0] else {
            panic!("role Number fixture is a construction")
        };
        assert!(matches!(
            role_number.equations[0].target,
            FeaturePlace::Role {
                feature: Feature::Number,
                ..
            }
        ));
    }

    #[test]
    fn verb_parser_distinguishes_fixed_paths_from_deferred_projected_roles() {
        let declarations = parse(
            r#"
                construction where_clause: Clause {
                    element WhereClause {
                        variable: lex Variable,
                        value: NounPhrase,
                    }
                    form where_clause = "where" lex(variable) verb(VerbLexeme::Be)
                        "the" "number" "of" value;
                }
                construction count: NounPhrase {
                    element CountNp {
                        head: lex Noun,
                        controller: lex Pronoun,
                        threshold: lex SignedNumber,
                    }
                    form count = noun(head) lex(controller) verb(VerbLexeme::Control)
                        "with" "power" lex(threshold) "or" "less";
                }
                construction destroy: VerbPhrase {
                    element Destroy { object: NounPhrase, }
                    form destroy = verb(VerbLexeme::Destroy) object;
                }
                construction projected: VerbPhrase {
                    element Projected { word: lex VerbLexeme, }
                    form projected = verb(word);
                }
            "#,
        )
        .expect("parsing preserves fixed paths and deferred projected roles for validation");

        let expected_fixed = [
            "VerbLexeme :: Be",
            "VerbLexeme :: Control",
            "VerbLexeme :: Destroy",
        ];
        for (declaration, expected) in declarations.declarations.iter().take(3).zip(expected_fixed)
        {
            let Declaration::Construction(construction) = declaration else {
                panic!("fixtures contain only constructions");
            };
            let operand = construction.forms[0]
                .atoms
                .iter()
                .find_map(|atom| match atom {
                    FormAtom::Verb(operand) => Some(operand),
                    _ => None,
                })
                .expect("fixture has a verb atom");
            let VerbOperand::Fixed(path) = operand else {
                panic!("golden verb constants must remain fixed paths");
            };
            assert_eq!(path.to_token_stream().to_string(), expected);
        }

        let Declaration::Construction(projected) = &declarations.declarations[3] else {
            panic!("fourth declaration is projected");
        };
        let FormAtom::Verb(VerbOperand::Projected(field)) = &projected.forms[0].atoms[0] else {
            panic!("bare verb operand should remain a projected role");
        };
        assert_eq!(field, "word");
    }

    #[test]
    fn nested_doc_comments_receive_the_named_mvp_error() {
        let cases = [
            "vocab Word { /// docs\n One = \"one\", }",
            "lexeme VerbLexeme using EnglishVerb { /// docs\n Be = \"be\", }",
            r"codec Number {
                value_type = Number;
                lexical = Lexical::Number;
                render = render_number;
                build { /// docs
                    pattern = BuildValue::Number(value);
                    construct = value;
                }
                traversal {}
            }",
            r"codec Number {
                value_type = Number;
                lexical = Lexical::Number;
                render = render_number;
                build {
                    pattern = BuildValue::Number(value);
                    construct = value;
                }
                traversal { /// docs
                    part value = scalar(value);
                    visit value;
                }
            }",
            "root Ability { /// docs\n punctuation = \".\"; eoi = true; standalone_render = true; }",
        ];

        for input in cases {
            let error = parse(input)
                .expect_err("nested doc comment is deferred")
                .to_string();
            assert!(error.contains("doc comments"), "{error}");
            assert!(error.contains("unimplemented in MVP"), "{error}");
        }
    }

    #[test]
    fn rejects_remaining_deferred_declaration_forms_by_name() {
        let cases = [
            (
                "construction x: X { element XNode { value: X, } require value != None; form x = value; }",
                "Plan 05 structural declarations",
            ),
            ("scanner Words { anything }", "scanner"),
            (
                "construction x: X { element XNode { value: X, } derive value.case = Case::Upper; form x = value; }",
                "derive target",
            ),
            (
                "/// docs\nconstruction x: X { element XNode { value: X, } form x = value; }",
                "doc comments",
            ),
        ];

        for (input, construct) in cases {
            let error = parse(input)
                .expect_err("deferred syntax must be rejected")
                .to_string();
            assert!(error.contains(construct), "{construct}: {error}");
            assert!(
                error.contains("unimplemented in MVP"),
                "{construct}: {error}"
            );
        }
    }

    #[test]
    fn parses_signed_decimal_generated_codec_source() {
        let declarations = parse(
            r#"
                codec SignedNumber {
                    generate signed_decimal {
                        magnitude = u32;
                        sign_type = Sign {
                            Positive = none,
                            Negative = "-",
                        };
                    }
                }
            "#,
        )
        .expect("the closed signed_decimal recipe parses");

        assert_eq!(declarations.declarations.len(), 1);
        assert!(matches!(
            declarations.declarations[0],
            Declaration::Codec(_)
        ));
    }

    #[test]
    fn parses_unsigned_cardinal_and_decimal_generated_codec_sources() {
        let declarations = parse(
            r"
                codec CardinalNumber {
                    generate english_cardinal {
                        magnitude = u32;
                    }
                }
                codec ScalarNumber {
                    generate unsigned_decimal {
                        magnitude = u32;
                    }
                }
            ",
        )
        .expect("the two closed unsigned numeral recipes parse");

        let [Declaration::Codec(cardinal), Declaration::Codec(decimal)] =
            declarations.declarations.as_slice()
        else {
            panic!("both declarations are generated codecs")
        };
        assert!(matches!(
            cardinal.generated,
            Some(crate::GeneratedCodecRecipe::EnglishCardinal(_))
        ));
        assert!(matches!(
            decimal.generated,
            Some(crate::GeneratedCodecRecipe::UnsignedDecimal(_))
        ));
    }

    #[test]
    fn parses_declaration_noun_generated_codec_source() {
        let declarations = parse(
            r#"
                lexeme NounLexeme using EnglishNoun { Player = "player", }
                codec Noun {
                    generate declaration_noun {
                        closed = NounLexeme;
                        position = Noun;
                        kinds = [Type, Subtype];
                        feature = Number;
                    }
                }
            "#,
        )
        .expect("the closed declaration_noun recipe parses");

        assert_eq!(declarations.declarations.len(), 2);
        assert!(matches!(
            declarations.declarations[1],
            Declaration::Codec(_)
        ));
    }

    #[test]
    fn parses_declaration_term_generated_codec_source() {
        let declarations = parse(
            r"
                codec KeywordAbility {
                    generate declaration_term {
                        position = FixedKeyword;
                        kinds = [KeywordAbility];
                        params = [Cost, Quality];
                        feature = Participle;
                    }
                }
            ",
        )
        .expect("the featureless declaration_term recipe parses");

        let [Declaration::Codec(binding)] = declarations.declarations.as_slice() else {
            panic!("one generated codec is retained")
        };
        let Some(crate::GeneratedCodecRecipe::DeclarationTerm(source)) = &binding.generated else {
            panic!("the codec retains a typed declaration_term recipe")
        };
        assert_eq!(source.position_slots[0].value, "FixedKeyword");
        assert_eq!(source.kind_slots[0].kinds[0], "KeywordAbility");
        assert_eq!(source.feature_slots[0].value, "Participle");
        assert_eq!(
            source.param_slots[0]
                .kinds
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            ["Cost", "Quality"],
        );
    }

    #[test]
    fn parses_declaration_verb_generated_codec_into_typed_tail_atoms() {
        let declarations = parse(
            r#"
                lexeme CoreTransitiveVerb using EnglishVerb { Destroy = "destroy", }
                codec TransitiveVerb {
                    generate declaration_verb {
                        closed = CoreTransitiveVerb;
                        class = Auxiliary;
                        position = Verb;
                        tail = ["with", lex(Preposition::For)?, Amount, ObjectNounPhrase];
                        feature = ConcordClass;
                    }
                }
            "#,
        )
        .expect("the finite declaration_verb source syntax parses");

        let Declaration::Codec(binding) = &declarations.declarations[1] else {
            panic!("the second declaration is the generated codec")
        };
        let Some(crate::GeneratedCodecRecipe::DeclarationVerb(source)) = &binding.generated else {
            panic!("the codec retains a typed declaration_verb recipe")
        };
        assert_eq!(source.closed_slots[0].value, "CoreTransitiveVerb");
        assert_eq!(source.class_slots[0].value, "Auxiliary");
        assert_eq!(source.position_slots[0].value, "Verb");
        assert!(matches!(
            source.tail_slots[0].atoms.as_slice(),
            [
                crate::DeclarationVerbTailAtomSource {
                    label: None,
                    optional: false,
                    kind: crate::DeclarationVerbTailAtomKindSource::Literal(literal),
                },
                crate::DeclarationVerbTailAtomSource {
                    label: None,
                    optional: true,
                    kind: crate::DeclarationVerbTailAtomKindSource::Lex(preposition),
                },
                crate::DeclarationVerbTailAtomSource {
                    label: None,
                    optional: false,
                    kind: crate::DeclarationVerbTailAtomKindSource::Amount(_),
                },
                crate::DeclarationVerbTailAtomSource {
                    label: None,
                    optional: false,
                    kind: crate::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(_),
                },
            ] if literal.value() == "with" && quote::quote!(#preposition).to_string() == "Preposition :: For"
        ));
        assert_eq!(source.feature_slots[0].value, "ConcordClass");
    }

    #[test]
    fn parses_declaration_verb_labels_and_preserves_their_source_roles() {
        let declarations = parse(
            r#"
                codec SearchForVerb {
                    generate declaration_verb {
                        position = Verb;
                        tail = [
                            location: ObjectNounPhrase,
                            "for",
                            sought: ObjectNounPhrase,
                        ];
                        feature = ConcordClass;
                    }
                }
            "#,
        )
        .expect("labeled declaration_verb tail atoms parse");

        let Declaration::Codec(binding) = &declarations.declarations[0] else {
            panic!("the declaration is a generated codec")
        };
        let Some(crate::GeneratedCodecRecipe::DeclarationVerb(source)) = &binding.generated else {
            panic!("the codec retains a typed declaration_verb recipe")
        };
        assert!(
            source.class_slots.is_empty(),
            "Predicate is the source default"
        );
        let [location, literal, sought] = source.tail_slots[0].atoms.as_slice() else {
            panic!("the exact labeled Search tail is preserved")
        };
        assert_eq!(
            location.label.as_ref().map(ToString::to_string).as_deref(),
            Some("location")
        );
        assert!(matches!(
            location.kind,
            crate::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(_)
        ));
        assert!(literal.label.is_none());
        assert!(matches!(
            &literal.kind,
            crate::DeclarationVerbTailAtomKindSource::Literal(value) if value.value() == "for"
        ));
        assert_eq!(
            sought.label.as_ref().map(ToString::to_string).as_deref(),
            Some("sought")
        );
        assert!(matches!(
            sought.kind,
            crate::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(_)
        ));
    }

    #[test]
    fn parses_distinct_declaration_noun_domain_filters() {
        parse(
            r#"
                lexeme NounLexeme using EnglishNoun { Player = "player", }
                codec TypeNoun {
                    generate declaration_noun {
                        closed = NounLexeme;
                        position = Noun;
                        kinds = [Type];
                        feature = Number;
                    }
                }
                codec CreatureNoun {
                    generate declaration_noun {
                        closed = NounLexeme;
                        position = Noun;
                        kinds = [Subtype(Creature)];
                        feature = Number;
                    }
                }
            "#,
        )
        .expect("exact kind and subtype-family declaration noun filters parse");
    }

    #[test]
    fn parses_context_identity_generated_source_into_typed_rows() {
        let declarations = parse(
            r"
                identity SelfReferenceSpelling {
                    generate context {
                        Full => card_name,
                        Abbreviated => abbreviated_card_name,
                        canonical_on_collision = Full;
                    }
                }
            ",
        )
        .expect("the closed context identity recipe parses");

        let Declaration::Identity(binding) = &declarations.declarations[0] else {
            panic!("the declaration remains an identity")
        };
        let Some(crate::GeneratedIdentityRecipe::Context(source)) = &binding.generated_identity
        else {
            panic!("the identity retains a typed context recipe")
        };
        assert_eq!(source.recipe, "context");
        assert_eq!(
            source
                .arms
                .iter()
                .map(|arm| (arm.variant.to_string(), arm.accessor.to_string()))
                .collect::<Vec<_>>(),
            [
                ("Full".to_owned(), "card_name".to_owned()),
                ("Abbreviated".to_owned(), "abbreviated_card_name".to_owned()),
            ]
        );
        assert_eq!(source.canonical_slots.len(), 1);
        assert_eq!(source.canonical_slots[0].arm, "Full");
    }

    #[test]
    fn parses_catalog_identity_generated_source_into_typed_rows() {
        let declarations = parse(
            r"
                identity CardName {
                    generate catalog_identity {
                        provider = CardNames;
                    }
                }
            ",
        )
        .expect("the closed catalog identity recipe parses");

        let Declaration::Identity(binding) = &declarations.declarations[0] else {
            panic!("the declaration remains an identity")
        };
        let Some(crate::GeneratedIdentityRecipe::Catalog(source)) = &binding.generated_identity
        else {
            panic!("the identity retains a typed catalog recipe")
        };
        assert_eq!(source.recipe, "catalog_identity");
        assert_eq!(source.provider_slots.len(), 1);
        assert_eq!(source.provider_slots[0].slot, "provider");
        assert_eq!(source.provider_slots[0].value, "CardNames");
    }

    #[test]
    fn rejects_multiple_unguarded_forms() {
        let error = parse(
            "construction x: X { element XNode { value: X, } form one = value; form two = value; }",
        )
        .expect_err("an unguarded form cannot be mixed with another form")
        .to_string();
        assert!(error.contains("unguarded form cannot be mixed"), "{error}");
    }

    #[test]
    fn codecs_require_one_closed_atom_class() {
        let binding = |atom: &str| {
            format!(
                r"codec Value {{
                    {atom}
                    value_type = Value;
                    lexical = Lexical::Value;
                    render = render_value;
                    build {{ pattern = BuildValue::Value(value); construct = value; }}
                    traversal {{ part value = scalar(value); visit value; }}
                }}"
            )
        };

        let missing = parse(&binding(""))
            .expect_err("codec atom class is mandatory")
            .to_string();
        assert!(
            missing.contains("codec binding requires explicit atom slot"),
            "{missing}"
        );

        let duplicate = parse(&binding("atom = lex; atom = noun;"))
            .expect_err("codec atom class occurs once")
            .to_string();
        assert!(duplicate.contains("duplicate atom slot"), "{duplicate}");

        let invalid = parse(&binding("atom = identity;"))
            .expect_err("codec atom class is closed")
            .to_string();
        assert!(
            invalid.contains("codec atom class must be `lex` or `noun`"),
            "{invalid}"
        );

        let identity = binding("atom = lex;").replacen("codec Value", "identity Value", 1);
        let identity_error = parse(&identity)
            .expect_err("identity bindings have an intrinsic atom class")
            .to_string();
        assert!(
            identity_error.contains("identity binding does not accept an atom slot"),
            "{identity_error}"
        );
    }

    #[test]
    fn parses_mandatory_named_element_product_separately_from_rule_identity() {
        let declarations = parse(
            r"
                construction event: Clause {
                    element EventClause {
                        subject: NounPhrase,
                        predicate: VerbPhrase,
                    }
                    derive predicate.concord_class = subject.concord_class;
                    form event = subject predicate;
                }
            ",
        )
        .expect("named element product is the mandatory MVP construction shape");
        let Declaration::Construction(construction) = &declarations.declarations[0] else {
            panic!("fixture is a construction");
        };
        assert_eq!(construction.name, "event");
        assert_eq!(construction.element.name, "EventClause");
        assert_eq!(construction.element.fields.len(), 2);
    }

    #[test]
    fn rejects_missing_or_repeated_element_products() {
        let missing = parse("construction x: X { form x = \"x\"; }")
            .expect_err("element product is mandatory")
            .to_string();
        assert!(
            missing.contains("exactly one named element product"),
            "{missing}"
        );

        let repeated =
            parse("construction x: X { element First {} element Second {} form x = \"x\"; }")
                .expect_err("only one element product is allowed")
                .to_string();
        assert!(
            repeated.contains("exactly one named element product"),
            "{repeated}"
        );
    }
}
