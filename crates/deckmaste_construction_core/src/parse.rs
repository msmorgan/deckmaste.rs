use proc_macro2::TokenStream;
use syn::Ident;
use syn::LitBool;
use syn::LitStr;
use syn::Pat;
use syn::Path;
use syn::Token;
use syn::Visibility;
use syn::braced;
use syn::bracketed;
use syn::ext::IdentExt;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::spanned::Spanned;

use crate::model::BuildLeaf;
use crate::model::Checked;
use crate::model::CheckedAccessor;
use crate::model::CheckedVisibility;
use crate::model::CodecAtomClass;
use crate::model::Construction;
use crate::model::ConstructorArgument;
use crate::model::ConstructorBinding;
use crate::model::ContextIdentityArm;
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
use crate::model::FieldKind;
use crate::model::Form;
use crate::model::FormAtom;
use crate::model::LeafCallback;
use crate::model::Lexeme;
use crate::model::NonPublicVisibility;
use crate::model::RenderBinding;
use crate::model::RoleRefinement;
use crate::model::Root;
use crate::model::TerminalBinding;
use crate::model::TerminalBindingKind;
use crate::model::Traversal;
use crate::model::TraversalBranch;
use crate::model::TraversalBranchArm;
use crate::model::TraversalCall;
use crate::model::TraversalField;
use crate::model::TraversalKind;
use crate::model::TraversalPart;
use crate::model::VerbOperand;
use crate::model::VisitMode;
use crate::model::Vocab;
use crate::model::VocabVariant;

mod keyword {
    syn::custom_keyword!(checked);
    syn::custom_keyword!(borrowed);
    syn::custom_keyword!(argument);
    syn::custom_keyword!(access);
    syn::custom_keyword!(callback);
    syn::custom_keyword!(call);
    syn::custom_keyword!(codec);
    syn::custom_keyword!(construct);
    syn::custom_keyword!(construction);
    syn::custom_keyword!(constructor);
    syn::custom_keyword!(context);
    syn::custom_keyword!(context_identity);
    syn::custom_keyword!(copy);
    syn::custom_keyword!(derive);
    syn::custom_keyword!(eoi);
    syn::custom_keyword!(element);
    syn::custom_keyword!(field);
    syn::custom_keyword!(form);
    syn::custom_keyword!(generate);
    syn::custom_keyword!(identity);
    syn::custom_keyword!(is);
    syn::custom_keyword!(lex);
    syn::custom_keyword!(lexeme);
    syn::custom_keyword!(leaf);
    syn::custom_keyword!(morphology);
    syn::custom_keyword!(noun);
    syn::custom_keyword!(otherwise);
    syn::custom_keyword!(part);
    syn::custom_keyword!(pattern);
    syn::custom_keyword!(private);
    syn::custom_keyword!(punctuation);
    syn::custom_keyword!(render);
    syn::custom_keyword!(require);
    syn::custom_keyword!(root);
    syn::custom_keyword!(scanner);
    syn::custom_keyword!(standalone_render);
    syn::custom_keyword!(traversal);
    syn::custom_keyword!(value_type);
    syn::custom_keyword!(variant);
    syn::custom_keyword!(verb);
    syn::custom_keyword!(visibility);
    syn::custom_keyword!(visit);
    syn::custom_keyword!(vocab);
    syn::custom_keyword!(when);
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
            } else if input.peek(keyword::vocab) {
                declarations.push(Declaration::Vocab(parse_vocab(input)?));
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
            } else if input.peek(keyword::morphology) {
                return Err(deferred(input.span(), "morphology"));
            } else if input.peek(keyword::scanner) {
                return Err(deferred(input.span(), "scanner"));
            } else {
                return Err(input.error(
                    "expected construction, vocab, lexeme, codec, identity, or root declaration",
                ));
            }
        }
        Ok(Self { declarations })
    }
}

fn parse_construction(input: ParseStream<'_>) -> syn::Result<Construction> {
    input.parse::<keyword::construction>()?;
    let name = input.call(Ident::parse_any)?;
    input.parse::<Token![:]>()?;
    let category = input.parse()?;
    let content;
    braced!(content in input);

    if !content.peek(keyword::element) {
        return Err(content.error("construction requires exactly one named element product"));
    }
    let element = parse_element(&content)?;
    let mut checked = None;
    let mut requirements = Vec::new();
    let mut equations = Vec::new();
    let mut form = None;

    while !content.is_empty() {
        if content.peek(Token![#]) {
            return Err(deferred(content.span(), "doc comments"));
        }
        if content.peek(keyword::checked) {
            if checked.is_some() {
                return Err(content.error("duplicate checked metadata"));
            }
            checked = Some(parse_checked(&content)?);
        } else if content.peek(keyword::require) {
            requirements.push(parse_requirement(&content)?);
        } else if content.peek(keyword::derive) {
            equations.push(parse_equation(&content)?);
        } else if content.peek(keyword::form) {
            if form.is_some() {
                return Err(deferred(content.span(), "multiple forms"));
            }
            form = Some(parse_form(&content)?);
        } else if content.peek(keyword::when) {
            return Err(deferred(content.span(), "when"));
        } else if content.peek(keyword::otherwise) {
            return Err(deferred(content.span(), "otherwise"));
        } else if content.peek(keyword::element) {
            return Err(content.error("construction requires exactly one named element product"));
        } else {
            return Err(content.error("expected checked, require, derive, or form after element"));
        }
    }

    let form =
        form.ok_or_else(|| syn::Error::new(name.span(), "construction requires exactly one form"))?;
    Ok(Construction {
        name,
        category,
        element,
        checked,
        requirements,
        equations,
        form,
    })
}

fn parse_element(input: ParseStream<'_>) -> syn::Result<Element> {
    input.parse::<keyword::element>()?;
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let mut fields = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        fields.push(parse_field(&content)?);
    }
    Ok(Element { name, fields })
}

fn parse_field(input: ParseStream<'_>) -> syn::Result<Field> {
    let name = input.parse()?;
    input.parse::<Token![:]>()?;
    let kind = if input.peek(keyword::lex) {
        input.parse::<keyword::lex>()?;
        FieldKind::Lex(input.parse()?)
    } else if input.peek(keyword::identity) {
        input.parse::<keyword::identity>()?;
        FieldKind::Identity(input.parse()?)
    } else if input.peek(Ident) {
        let fork = input.fork();
        let possible_deferred: Ident = fork.parse()?;
        match possible_deferred.to_string().as_str() {
            "opt" => return Err(deferred(input.span(), "opt")),
            "seq" => return Err(deferred(input.span(), "seq")),
            _ => FieldKind::Category(input.parse()?),
        }
    } else {
        return Err(input.error("expected a category, lex terminal, or identity terminal field"));
    };
    input.parse::<Token![,]>()?;
    Ok(Field { name, kind })
}

fn parse_checked(input: ParseStream<'_>) -> syn::Result<Checked> {
    input.parse::<keyword::checked>()?;
    let content;
    braced!(content in input);
    let mut visibilities = Vec::new();
    let mut accessors = Vec::new();
    let mut constructor = None;

    while !content.is_empty() {
        reject_doc_comment(&content)?;
        if content.peek(keyword::visibility) {
            content.parse::<keyword::visibility>()?;
            let role = content.parse()?;
            content.parse::<Token![=]>()?;
            let visibility = if content.peek(keyword::private) {
                let token = content.parse::<keyword::private>()?;
                NonPublicVisibility::Private(token.span())
            } else {
                let visibility: Visibility = content.parse()?;
                match visibility {
                    restricted @ Visibility::Restricted(_) => {
                        NonPublicVisibility::Restricted(restricted)
                    }
                    Visibility::Public(public) => {
                        return Err(syn::Error::new(
                            public.span(),
                            "checked visibility must be nonpublic",
                        ));
                    }
                    Visibility::Inherited => {
                        return Err(content.error("expected private or restricted visibility"));
                    }
                }
            };
            content.parse::<Token![;]>()?;
            visibilities.push(CheckedVisibility { role, visibility });
        } else if content.peek(keyword::access) {
            content.parse::<keyword::access>()?;
            let role = content.parse()?;
            content.parse::<Token![=]>()?;
            let method = content.parse()?;
            content.parse::<Token![;]>()?;
            accessors.push(CheckedAccessor { role, method });
        } else if content.peek(keyword::constructor) {
            if constructor.is_some() {
                return Err(content.error("duplicate checked constructor"));
            }
            content.parse::<keyword::constructor>()?;
            content.parse::<Token![=]>()?;
            let path = content.parse()?;
            let arguments_content;
            parenthesized!(arguments_content in content);
            let arguments = arguments_content
                .parse_terminated(parse_constructor_argument, Token![,])?
                .into_iter()
                .collect();
            content.parse::<Token![;]>()?;
            constructor = Some(ConstructorBinding { path, arguments });
        } else {
            return Err(
                content.error("expected visibility, access, or constructor in checked metadata")
            );
        }
    }

    let constructor =
        constructor.ok_or_else(|| input.error("checked metadata requires a constructor"))?;
    Ok(Checked {
        visibilities,
        accessors,
        constructor,
    })
}

fn parse_constructor_argument(input: ParseStream<'_>) -> syn::Result<ConstructorArgument> {
    if input.peek(keyword::context) {
        let token = input.parse::<keyword::context>()?;
        return Ok(ConstructorArgument::Context(token.span()));
    }

    let ident: Ident = input.parse()?;
    if ident == "vec" && input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        let content;
        bracketed!(content in input);
        let role = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("vec! constructor arguments accept exactly one role"));
        }
        return Ok(ConstructorArgument::VecRole {
            span: ident.span(),
            role,
        });
    }
    if input.peek(Token![.]) || input.peek(Token![::]) || input.peek(syn::token::Paren) {
        return Err(syn::Error::new(
            ident.span(),
            "checked constructor arguments must be roles, context, or vec![role]",
        ));
    }
    Ok(ConstructorArgument::Role(ident))
}

fn parse_requirement(input: ParseStream<'_>) -> syn::Result<RoleRefinement> {
    let token = input.parse::<keyword::require>()?;
    let role = input.parse()?;
    if !input.peek(keyword::is) {
        return Err(deferred(token.span(), "general require"));
    }
    input.parse::<keyword::is>()?;
    let variant = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(RoleRefinement { role, variant })
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
        if feature_ident != "agreement" {
            return Err(deferred(feature_ident.span(), "derive target"));
        }
        FeaturePlace::Role {
            field: first,
            feature: Feature::Agreement,
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
        let path: Path = input.parse()?;
        FeatureValue::Constant(path)
    };
    input.parse::<Token![;]>()?;
    Ok(FeatureEquation { target, value })
}

fn feature_from_ident(ident: &Ident) -> Option<Feature> {
    match ident.to_string().as_str() {
        "agreement" => Some(Feature::Agreement),
        "number" => Some(Feature::Number),
        _ => None,
    }
}

fn parse_form(input: ParseStream<'_>) -> syn::Result<Form> {
    input.parse::<keyword::form>()?;
    let name = input.call(Ident::parse_any)?;
    if input.peek(keyword::when) {
        return Err(deferred(input.span(), "when"));
    }
    if input.peek(keyword::otherwise) {
        return Err(deferred(input.span(), "otherwise"));
    }
    input.parse::<Token![=]>()?;
    let mut atoms = Vec::new();
    while !input.peek(Token![;]) {
        if input.peek(keyword::when) {
            return Err(deferred(input.span(), "when"));
        }
        if input.peek(keyword::otherwise) {
            return Err(deferred(input.span(), "otherwise"));
        }
        if input.peek(LitStr) {
            atoms.push(FormAtom::Literal(input.parse()?));
            continue;
        }

        let ident: Ident = input.parse()?;
        let atom = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            match ident.to_string().as_str() {
                "verb" => {
                    let path: Path = content.parse()?;
                    if !content.is_empty() {
                        return Err(content.error("verb atoms accept exactly one operand"));
                    }
                    FormAtom::Verb(classify_verb_operand(path))
                }
                "lex" | "identity" | "noun" => {
                    let role = content.parse()?;
                    if !content.is_empty() {
                        return Err(content.error("form atoms accept exactly one role"));
                    }
                    match ident.to_string().as_str() {
                        "lex" => FormAtom::Lex(role),
                        "identity" => FormAtom::Identity(role),
                        "noun" => FormAtom::Noun(role),
                        _ => unreachable!(),
                    }
                }
                _ => return Err(syn::Error::new(ident.span(), "unknown form atom")),
            }
        } else {
            FormAtom::Role(ident)
        };
        atoms.push(atom);
    }
    input.parse::<Token![;]>()?;
    Ok(Form { name, atoms })
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
    let mut variants = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        let name = content.parse()?;
        content.parse::<Token![=]>()?;
        let word = content.parse()?;
        variants.push(VocabVariant { name, word });
        content.parse::<Token![,]>()?;
    }
    Ok(Vocab { name, variants })
}

fn parse_lexeme(input: ParseStream<'_>) -> syn::Result<Lexeme> {
    input.parse::<keyword::lexeme>()?;
    let name = input.parse()?;
    let content;
    braced!(content in input);
    let mut variants = Vec::new();
    while !content.is_empty() {
        reject_doc_comment(&content)?;
        variants.push(content.parse()?);
        content.parse::<Token![,]>()?;
    }
    Ok(Lexeme { name, variants })
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

fn parse_terminal_binding(
    input: ParseStream<'_>,
    kind: BindingKind,
) -> syn::Result<TerminalBinding> {
    match kind {
        BindingKind::Codec => {
            input.parse::<keyword::codec>()?;
        }
        BindingKind::Identity => {
            input.parse::<keyword::identity>()?;
        }
    }
    let name: Ident = input.parse()?;
    let content;
    braced!(content in input);
    let mut value_type = None;
    let mut codec_atom = None;
    let mut lexical_variant = None;
    let mut render = None;
    let mut build = None;
    let mut traversal = None;

    while !content.is_empty() {
        if content.peek(keyword::generate) {
            return Err(deferred(
                content.span(),
                &format!("generative {}", kind.name()),
            ));
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
        codec_atom,
        value_type: required(value_type, &name, "value_type")?,
        lexical_variant,
        render,
        build,
        traversal: required(traversal, &name, "traversal")?,
    })
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
    let callback = input.parse()?;
    let content;
    parenthesized!(content in input);
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
    let category: Path = input.parse()?;
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
        punctuation: punctuation
            .ok_or_else(|| syn::Error::new(span, "root requires punctuation"))?,
        eoi: eoi.ok_or_else(|| syn::Error::new(span, "root requires eoi"))?,
        standalone_render: standalone_render
            .ok_or_else(|| syn::Error::new(span, "root requires standalone_render"))?,
    })
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

    use crate::ConstructorArgument;
    use crate::Declaration;
    use crate::Feature;
    use crate::FeaturePlace;
    use crate::FeatureValue;
    use crate::FormAtom;
    use crate::NonPublicVisibility;
    use crate::TraversalKind;
    use crate::VerbOperand;

    const GOLDEN_SHAPED_DECLARATIONS: &str = r#"
        construction triggered: Ability {
            element Triggered {
                trigger: lex TriggerWord,
                event: Clause,
                effect: Sentence,
            }
            checked {
                visibility trigger = pub(crate);
                visibility event = pub(crate);
                visibility effect = pub(crate);
                constructor = Triggered::new(trigger, event, vec![effect]);
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
            derive agreement = Agreement::Bare;
            form imperative = predicate;
        }

        construction declarative: Sentence {
            element Declarative {
                subject: NounPhrase,
                predicate: VerbPhrase,
            }
            derive predicate.agreement = subject.agreement;
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

        lexeme VerbLexeme {
            Be,
            Deal,
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

    fn tokens(value: &impl ToTokens) -> String {
        value.to_token_stream().to_string()
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

        let checked = triggered
            .checked
            .as_ref()
            .expect("checked metadata retained");
        assert_eq!(checked.visibilities.len(), 3);
        assert!(
            checked.visibilities.iter().all(|visibility| matches!(
                visibility.visibility,
                NonPublicVisibility::Restricted(_)
            ))
        );
        assert_eq!(path(&checked.constructor.path), "Triggered :: new");
        assert!(
            matches!(checked.constructor.arguments[0], ConstructorArgument::Role(ref role) if role == "trigger")
        );
        assert!(
            matches!(checked.constructor.arguments[1], ConstructorArgument::Role(ref role) if role == "event")
        );
        assert!(
            matches!(checked.constructor.arguments[2], ConstructorArgument::VecRole { ref role, .. } if role == "effect")
        );

        assert_eq!(
            declarations.declarations[0..3]
                .iter()
                .map(|declaration| {
                    let Declaration::Construction(construction) = declaration else {
                        panic!("first declarations are constructions");
                    };
                    let requirement = &construction.requirements[0];
                    (
                        requirement.role.to_string(),
                        requirement.variant.to_string(),
                    )
                })
                .collect::<Vec<_>>(),
            [
                ("event".into(), "Event".into()),
                ("clause".into(), "Where".into()),
                ("controller".into(), "You".into())
            ]
        );

        assert!(matches!(triggered.form.atoms[0], FormAtom::Lex(ref role) if role == "trigger"));
        assert!(matches!(triggered.form.atoms[1], FormAtom::Role(ref role) if role == "event"));
        assert!(
            matches!(triggered.form.atoms[2], FormAtom::Literal(ref word) if word.value() == ",")
        );
        assert!(matches!(triggered.form.atoms[3], FormAtom::Role(ref role) if role == "effect"));

        let Declaration::Construction(count) = &declarations.declarations[2] else {
            panic!("third declaration is count");
        };
        let FormAtom::Verb(VerbOperand::Fixed(control)) = &count.form.atoms[2] else {
            panic!("count uses a fixed control verb");
        };
        assert_eq!(path(control), "VerbLexeme :: Control");

        let Declaration::Construction(imperative) = &declarations.declarations[3] else {
            panic!("fourth declaration is imperative");
        };
        assert!(matches!(
            imperative.equations[0].target,
            FeaturePlace::Construction(Feature::Agreement)
        ));
        let FeatureValue::Constant(imperative_value) = &imperative.equations[0].value else {
            panic!("imperative agreement is constant");
        };
        assert_eq!(path(imperative_value), "Agreement :: Bare");

        let Declaration::Construction(declarative) = &declarations.declarations[4] else {
            panic!("fifth declaration is declarative");
        };
        assert!(matches!(
            &declarative.equations[0].target,
            FeaturePlace::Role { field, feature: Feature::Agreement } if field == "predicate"
        ));
        let FeatureValue::FromRole(source) = &declarative.equations[0].value else {
            panic!("declarative agreement comes from its subject");
        };
        assert_eq!(source.role, "subject");
        assert_eq!(source.feature, Feature::Agreement);

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
        assert_eq!(root.punctuation.value(), ".");
        assert!(root.eoi);
        assert!(root.standalone_render);
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
                    derive agreement = Agreement::Bare;
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
                    derive predicate.agreement = subject.agreement;
                    form declarative = subject predicate;
                }
            ",
        )
        .expect("construction and role agreement targets are in the MVP");

        let expected = [
            FeaturePlace::Construction(Feature::Agreement),
            FeaturePlace::Construction(Feature::Number),
            FeaturePlace::Role {
                field: syn::parse_str("predicate").expect("identifier"),
                feature: Feature::Agreement,
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
        assert_eq!(source.feature, Feature::Agreement);

        let error = parse(
            "construction x: X { element XNode { role: X, } derive role.number = NounNumber::Plural; form x = role; }",
        )
        .expect_err("role.number is not an architecture-authorized target")
        .to_string();
        assert!(error.contains("derive target"), "{error}");
        assert!(error.contains("unimplemented in MVP"), "{error}");
    }

    #[test]
    fn verb_atoms_accept_fixed_paths_and_projected_roles() {
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
        .expect("fixed verb paths and a projected lexeme role are distinct MVP operands");

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
            let operand = construction
                .form
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
        let FormAtom::Verb(VerbOperand::Projected(field)) = &projected.form.atoms[0] else {
            panic!("bare verb operand should remain a projected role");
        };
        assert_eq!(field, "word");
    }

    #[test]
    fn nested_doc_comments_receive_the_named_mvp_error() {
        let cases = [
            "vocab Word { /// docs\n One = \"one\", }",
            "lexeme VerbLexeme { /// docs\n Be, }",
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
    fn rejects_deferred_declaration_forms_by_name() {
        let cases = [
            (
                "construction x: X { element XNode { xs: opt X, } form x = xs; }",
                "opt",
            ),
            (
                "construction x: X { element XNode { xs: seq X, } form x = xs; }",
                "seq",
            ),
            (
                "construction x: X { element XNode { value: X, } require value != None; form x = value; }",
                "general require",
            ),
            (
                "construction x: X { element XNode { value: X, } form x when value = value; }",
                "when",
            ),
            (
                "construction x: X { element XNode { value: X, } form x otherwise = value; }",
                "otherwise",
            ),
            ("codec X { generate { anything } }", "generative codec"),
            (
                "identity X { generate { anything } }",
                "generative identity",
            ),
            ("morphology English { anything }", "morphology"),
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
    fn rejects_a_second_form_by_name() {
        let error = parse(
            "construction x: X { element XNode { value: X, } form one = value; form two = value; }",
        )
        .expect_err("a second form is deferred")
        .to_string();
        assert!(error.contains("multiple forms"), "{error}");
        assert!(error.contains("unimplemented in MVP"), "{error}");
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
                    derive predicate.agreement = subject.agreement;
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
