use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::emit::LocalAllocator;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::category_renderer;
use crate::identifier::emitted_ident;
use crate::identifier::feature_helper;
use crate::identifier::key as identifier_key;
use crate::identifier::lexeme_surface_helper;
use crate::identifier::snake_case;
use crate::identifier::structural_sequence_renderer;
use crate::model::VisitMode;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AccessorMode;
use crate::semantic::AtomPlan;
use crate::semantic::BindingPlan;
use crate::semantic::BindingRenderPlan;
use crate::semantic::ConstructionFieldKind;
use crate::semantic::ConstructionFieldPlan;
use crate::semantic::ConstructionPlan;
use crate::semantic::FiniteDomainKindPlan;
use crate::semantic::FiniteDomainPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::FormPlan;
use crate::semantic::LexemePlan;
use crate::semantic::RootPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SeparatorPlan;
use crate::semantic::SignedDecimalPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::TerminalPlan;
use crate::semantic::UnsignedNumberKind;
use crate::semantic::UnsignedNumberPlan;
use crate::semantic::ValueKindPlan;
use crate::semantic::VocabPlan;

#[allow(
    clippy::too_many_lines,
    reason = "the phase finalizer preserves the pinned source-order item sequence"
)]
pub(crate) fn emit(validated: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated.constructions();
    let roots = validated.roots().iter().collect::<Vec<_>>();
    let categories = category_groups(constructions)
        .into_iter()
        .filter(|(category, _)| !validated.explicit_sum_owns_construction_category(category))
        .collect::<Vec<_>>();
    let nested_categories = constructions
        .iter()
        .flat_map(ConstructionPlan::fields)
        .filter(|field| field.kind() == ConstructionFieldKind::Category)
        .map(|field| field.terminal().to_owned())
        .chain(validated.products().iter().flat_map(|product| {
            product
                .fields()
                .iter()
                .filter_map(|field| match field.kind().value() {
                    ValueKindPlan::Category(category) => Some(category.clone()),
                    ValueKindPlan::Lex(_)
                    | ValueKindPlan::Identity(_)
                    | ValueKindPlan::Product(_)
                    | ValueKindPlan::Sum(_) => None,
                })
        }))
        .chain(validated.sums().iter().flat_map(|sum| {
            sum.alternatives().iter().filter_map(|alternative| {
                if let ValueKindPlan::Category(category) = alternative.value() {
                    Some(category.clone())
                } else {
                    None
                }
            })
        }))
        .collect::<HashSet<_>>();
    let root_names = roots
        .iter()
        .filter(|root| root.is_render_entry())
        .map(|root| root.category().to_owned())
        .collect::<HashSet<_>>();
    let mut items = Vec::new();
    let takes_environment = validated.needs_parser_environment();
    items.extend(emit_structural_surface_runtime(validated)?);
    items.extend(emit_structural_renderers(validated, &root_names)?);
    for root in roots.iter().filter(|root| root.is_render_entry()) {
        let category = root.category().to_owned();
        let write_function = ident(&format!("write_{}_render", snake_case(&category)));
        let collecting_function = ident(&format!("render_{}_with_claims", snake_case(&category)));
        let ty = ident(&category);
        let punctuation = root.punctuation().chars().next();
        let is_structural_root = validated
            .products()
            .iter()
            .any(|product| product.name() == category)
            || validated.sums().iter().any(|sum| sum.name() == category);
        let render_body = if is_structural_root {
            let helper = ident(&crate::identifier::prefixed("render_", &category));
            let environment = takes_environment.then(|| quote! { , environment });
            quote! { #helper(writer, value, context #environment); }
        } else if nested_categories.contains(&category) {
            let helper = render_category_name(&category, true);
            let capability = validated.category_render_capability(&category);
            if capability.requires_external_agreement() {
                return Err(internal(
                    "validated standalone render root requires external agreement",
                ));
            }
            let context = capability.requires_context().then(|| quote! { context });
            let environment = takes_environment.then(|| quote! { environment });
            let tail = signature_tail(&[None, context, environment]);
            quote! { #helper(writer, self #tail); }
        } else {
            let members = categories
                .iter()
                .find(|(name, _)| name == &category)
                .map(|(_, members)| members.as_slice())
                .ok_or_else(|| internal("validated root category is absent"))?;
            let allocator = render_allocator(validated, members, true, &root_names, false, true)?;
            let arms = render_arms(
                validated,
                members,
                true,
                &root_names,
                &allocator,
                &quote! { self },
            )?;
            quote! { match self { #(#arms)* } }
        };
        let environment = takes_environment
            .then(|| quote! { , environment: &crate::environment::ParserEnvironment });
        let environment_argument = takes_environment.then(|| quote! { , environment });
        let punctuation_statement = punctuation.map(|punctuation| {
            let stable_id =
                syn::LitStr::new(&format!("root:{category}/punctuation"), Span::call_site());
            quote! {
                writer.claim(
                    || LexicalOwner::static_owner(
                        LexicalProvenanceKind::FormLiteral,
                        #stable_id,
                    ),
                    |writer| writer.punctuation(#punctuation),
                );
            }
        });
        let origin = DeclarationKey::new(DeclarationKind::Root, category.clone());
        let writer = if is_structural_root {
            GeneratedItem::new(
                ItemKey::Named {
                    kind: NamedKind::Function,
                    name: write_function.to_string(),
                },
                quote! {
                fn #write_function(
                    value: &#ty,
                    writer: &mut Writer,
                    context: &ParseContext<'_> #environment,
                ) {
                    #render_body
                    #punctuation_statement
                }
                },
                vec![origin.clone()],
            )
        } else {
            GeneratedItem::new(
                ItemKey::Impl {
                    trait_name: None,
                    self_ty: category.clone(),
                },
                quote! {
                impl #ty {
                    fn #write_function(
                        &self,
                        writer: &mut Writer,
                        context: &ParseContext<'_> #environment,
                    ) {
                        #render_body
                        #punctuation_statement
                    }
                }
                },
                vec![origin.clone()],
            )
        };
        items.push(writer);
        let write_call = if is_structural_root {
            quote! { #write_function(self, &mut writer, context #environment_argument); }
        } else {
            quote! { self.#write_function(&mut writer, context #environment_argument); }
        };
        let collecting_write_call = if is_structural_root {
            quote! { #write_function(value, &mut writer, context #environment_argument); }
        } else {
            quote! { value.#write_function(&mut writer, context #environment_argument); }
        };
        items.push(GeneratedItem::new(
            ItemKey::Impl {
                trait_name: Some("Render".to_owned()),
                self_ty: category.clone(),
            },
            quote! {
                impl Render for #ty {
                    fn render(&self, context: &ParseContext<'_> #environment) -> String {
                        let mut writer = Writer::new();
                        #write_call
                        writer.finish()
                    }
                }
            },
            vec![origin.clone()],
        ));
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: collecting_function.to_string(),
            },
            quote! {
            pub(crate) fn #collecting_function(
                value: &#ty,
                context: &ParseContext<'_> #environment,
            ) -> (String, Vec<RawRenderedClaim>) {
                let mut claims = Vec::new();
                let mut writer = Writer::collecting(&mut claims);
                #collecting_write_call
                let rendered = writer.finish();
                (rendered, claims)
            }
            },
            vec![origin],
        ));
    }

    let render_order = render_category_order(&categories, &roots, &nested_categories);
    for category in render_order {
        let members = categories
            .iter()
            .find(|(name, _)| name == &category)
            .map(|(_, members)| members)
            .ok_or_else(|| internal("render category order is inconsistent"))?;
        if root_names.contains(&category) && !nested_categories.contains(&category) {
            continue;
        }
        let helper = render_category_name(&category, root_names.contains(&category));
        let ty = ident(&category);
        let capability = validated.category_render_capability(&category);
        let takes_agreement = capability.requires_external_agreement();
        let takes_context = capability.requires_context();
        let allocator = render_allocator(
            validated,
            members,
            false,
            &root_names,
            takes_agreement,
            takes_context,
        )?;
        let mut signature_allocator = allocator.clone();
        let argument = signature_allocator.allocate(&category_argument(&category));
        let agreement = takes_agreement.then(|| quote! { agreement: Agreement });
        let context = takes_context.then(|| quote! { context: &ParseContext<'_> });
        let environment = takes_environment
            .then(|| quote! { environment: &crate::environment::ParserEnvironment });
        let separators = signature_tail(&[agreement, context, environment]);
        let arms = render_arms(
            validated,
            members,
            false,
            &root_names,
            &signature_allocator,
            &quote! { #argument },
        )?;
        let tokens = quote! {
            fn #helper(writer: &mut Writer, #argument: &#ty #separators) {
                match #argument { #(#arms)* }
            }
        };
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: helper.to_string(),
            },
            tokens,
            members
                .iter()
                .map(|construction| {
                    DeclarationKey::new(
                        DeclarationKind::Construction,
                        construction.construction_id(),
                    )
                })
                .collect(),
        ));
    }

    for terminal in validated.terminals() {
        let TerminalPlan::Vocab(row) = terminal else {
            continue;
        };
        let function = ident(&format!("render_{}", snake_case(row.name())));
        let ty = emitted_ident(row.name(), row.name_ident().span());
        let mut allocator = LocalAllocator::default();
        allocator.reserve("writer");
        let argument = allocator.allocate(&render_vocab_argument(row.name()));
        let arms = row.variants().iter().map(|variant| {
            let name = emitted_ident(&identifier_key(variant.name()), variant.name().span());
            let word = variant.word();
            quote! { #ty::#name => writer.word(#word) }
        });
        let tokens = quote! {
            fn #function(writer: &mut Writer, #argument: #ty) {
                match #argument { #(#arms,)* }
            }
        };
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: function.to_string(),
            },
            tokens,
            vec![DeclarationKey::new(DeclarationKind::Vocab, row.name())],
        ));
    }

    items.extend(
        collect_vocab_feature_helpers(validated)?
            .into_iter()
            .map(emit_vocab_feature_helper),
    );

    if let Some(codec) = validated.runtime_signed_decimal() {
        let function = ident(&format!("render_{}", snake_case(codec.codec_name())));
        let ty = codec.codec_ident();
        let sign = codec.sign_type();
        let positive = codec.positive_variant();
        let negative = codec.negative_variant();
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: function.to_string(),
            },
            quote! {
                fn #function(writer: &mut Writer, number: &#ty) {
                    let magnitude = number.magnitude.to_string();
                    match number.sign {
                        #sign::#positive => writer.word(&magnitude),
                        #sign::#negative => writer.word(&format!("-{magnitude}")),
                    }
                }
            },
            vec![codec.origin().clone()],
        ));
    }

    for codec in validated.runtime_unsigned_numbers() {
        let codec_name = snake_case(codec.codec_name());
        let format = ident(&format!("format_{codec_name}"));
        let parse = ident(&format!("parse_{codec_name}"));
        let origin = vec![codec.origin().clone()];
        match codec.kind() {
            UnsignedNumberKind::EnglishCardinal => {
                items.push(GeneratedItem::new(
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name: format.to_string(),
                    },
                    english_cardinal_formatter(&format),
                    origin.clone(),
                ));
                let number = ident(&feature_helper("number", codec.codec_name()));
                let cardinality = ident(&feature_helper("cardinality", codec.codec_name()));
                let ty = codec.codec_ident();
                items.push(GeneratedItem::new(
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name: number.to_string(),
                    },
                    quote! {
                        fn #number(value: &#ty) -> Number {
                            if value.magnitude == 1 {
                                Number::Singular
                            } else {
                                Number::Plural
                            }
                        }
                    },
                    origin.clone(),
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name: cardinality.to_string(),
                    },
                    quote! {
                        fn #cardinality(value: &#ty) -> Cardinality {
                            match value.magnitude {
                                0 => Cardinality::Zero,
                                1 => Cardinality::One,
                                _ => Cardinality::TwoPlus,
                            }
                        }
                    },
                    origin.clone(),
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Named {
                        kind: NamedKind::Function,
                        name: parse.to_string(),
                    },
                    english_cardinal_parser(&format, &parse),
                    origin.clone(),
                ));
            }
            UnsignedNumberKind::UnsignedDecimal => match codec.magnitude() {
                crate::semantic::UnsignedPrimitive::U32 => {
                    items.push(GeneratedItem::new(
                        ItemKey::Named {
                            kind: NamedKind::Function,
                            name: format.to_string(),
                        },
                        unsigned_decimal_formatter(&format),
                        origin.clone(),
                    ));
                    items.push(GeneratedItem::new(
                        ItemKey::Named {
                            kind: NamedKind::Function,
                            name: parse.to_string(),
                        },
                        unsigned_decimal_parser(&format, &parse),
                        origin.clone(),
                    ));
                }
                crate::semantic::UnsignedPrimitive::NonZeroU32 => {
                    items.push(GeneratedItem::new(
                        ItemKey::Named {
                            kind: NamedKind::Function,
                            name: format.to_string(),
                        },
                        nonzero_unsigned_decimal_formatter(&format),
                        origin.clone(),
                    ));
                    items.push(GeneratedItem::new(
                        ItemKey::Named {
                            kind: NamedKind::Function,
                            name: parse.to_string(),
                        },
                        nonzero_unsigned_decimal_parser(&parse),
                        origin.clone(),
                    ));
                }
            },
        }
        let render = ident(&format!("render_{codec_name}"));
        let ty = codec.codec_ident();
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: render.to_string(),
            },
            quote! {
                fn #render(writer: &mut Writer, number: &#ty) {
                    writer.word(&#format(number.magnitude));
                }
            },
            origin,
        ));
    }

    for feature in [
        Feature::Agreement,
        Feature::Cardinality,
        Feature::Number,
        Feature::Onset,
        Feature::PossessiveEnding,
    ] {
        for (category, members) in &categories {
            if !validated.category_reads_feature(category, feature) {
                continue;
            }
            items.push(emit_feature_helper(validated, category, members, feature)?);
        }
    }
    for sum in validated.sums().iter().filter(|sum| {
        validated.sum_carries_agreement(sum.name())
            && !validated.sum_requires_external_agreement(sum.name())
    }) {
        items.push(emit_sum_agreement_helper(validated, sum)?);
    }
    Ok(items)
}

fn english_cardinal_formatter(function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(mut value: u32) -> String {
            const SMALL: [&str; 20] = [
                "zero", "one", "two", "three", "four", "five", "six", "seven",
                "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen",
                "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
            ];
            const TENS: [&str; 10] = [
                "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",
                "eighty", "ninety",
            ];

            fn below_hundred(value: u32) -> String {
                if value < 20 {
                    SMALL[value as usize].to_owned()
                } else {
                    let tens = TENS[(value / 10) as usize];
                    match value % 10 {
                        0 => tens.to_owned(),
                        units => format!("{tens}-{}", SMALL[units as usize]),
                    }
                }
            }

            fn below_thousand(mut value: u32) -> String {
                let mut surface = String::new();
                if value >= 100 {
                    surface.push_str(SMALL[(value / 100) as usize]);
                    surface.push_str(" hundred");
                    value %= 100;
                    if value != 0 {
                        surface.push(' ');
                    }
                }
                if value != 0 {
                    surface.push_str(&below_hundred(value));
                }
                surface
            }

            if value == 0 {
                return SMALL[0].to_owned();
            }
            let mut groups = Vec::with_capacity(4);
            for (scale, name) in [
                (1_000_000_000, "billion"),
                (1_000_000, "million"),
                (1_000, "thousand"),
            ] {
                let group = value / scale;
                if group != 0 {
                    groups.push(format!("{} {name}", below_thousand(group)));
                    value %= scale;
                }
            }
            if value != 0 {
                groups.push(below_thousand(value));
            }
            groups.join(", ")
        }
    }
}

fn english_cardinal_parser(formatter: &syn::Ident, function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(input: &str) -> Option<u32> {
            const SMALL: [&str; 20] = [
                "zero", "one", "two", "three", "four", "five", "six", "seven",
                "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen",
                "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
            ];
            const TENS: [&str; 10] = [
                "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy",
                "eighty", "ninety",
            ];

            fn below_hundred(input: &str) -> Option<u32> {
                if let Some(value) = SMALL.iter().position(|word| *word == input) {
                    return u32::try_from(value).ok();
                }
                if let Some(value) = TENS
                    .iter()
                    .position(|word| *word == input)
                    .filter(|value| *value >= 2)
                {
                    return u32::try_from(value).ok()?.checked_mul(10);
                }
                let (tens, units) = input.split_once('-')?;
                let tens = u32::try_from(
                    TENS.iter()
                        .position(|word| *word == tens)
                        .filter(|value| *value >= 2)?,
                )
                .ok()?;
                let units = u32::try_from(
                    SMALL[1..10].iter().position(|word| *word == units)? + 1,
                )
                .ok()?;
                tens.checked_mul(10)?.checked_add(units)
            }

            fn below_thousand(input: &str) -> Option<u32> {
                if let Some(value) = below_hundred(input) {
                    return Some(value);
                }
                let (hundreds, remainder) = input.split_once(" hundred")?;
                let hundreds = u32::try_from(
                    SMALL[1..10].iter().position(|word| *word == hundreds)? + 1,
                )
                .ok()?;
                let remainder = if remainder.is_empty() {
                    0
                } else {
                    below_hundred(remainder.strip_prefix(' ')?)?
                };
                hundreds.checked_mul(100)?.checked_add(remainder)
            }

            let mut total = 0_u32;
            let mut groups = input.split(", ").peekable();
            while let Some(group) = groups.next() {
                let (coefficient, scale) = [
                    (1_000_000_000, "billion"),
                    (1_000_000, "million"),
                    (1_000, "thousand"),
                ]
                .into_iter()
                .find_map(|(scale, name)| {
                    group
                        .strip_suffix(name)?
                        .strip_suffix(' ')
                        .map(|coefficient| (coefficient, scale))
                })
                .unwrap_or((group, 1));
                if scale == 1 && groups.peek().is_some() {
                    return None;
                }
                let coefficient = below_thousand(coefficient)?;
                total = total.checked_add(coefficient.checked_mul(scale)?)?;
            }
            (#formatter(total) == input).then_some(total)
        }
    }
}

fn unsigned_decimal_formatter(function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(value: u32) -> String {
            let digits = value.to_string();
            let first_group_len = match digits.len() % 3 {
                0 => 3,
                remainder => remainder,
            };
            let mut surface = String::with_capacity(digits.len() + (digits.len() - 1) / 3);
            surface.push_str(&digits[..first_group_len]);
            let mut group_start = first_group_len;
            while group_start < digits.len() {
                surface.push(',');
                surface.push_str(&digits[group_start..group_start + 3]);
                group_start += 3;
            }
            surface
        }
    }
}

fn unsigned_decimal_parser(formatter: &syn::Ident, function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(input: &str) -> Option<u32> {
            let value = input.replace(',', "").parse::<u32>().ok()?;
            (#formatter(value) == input).then_some(value)
        }
    }
}

fn nonzero_unsigned_decimal_formatter(function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(value: ::std::num::NonZeroU32) -> String {
            let digits = value.get().to_string();
            let first_group_len = match digits.len() % 3 {
                0 => 3,
                remainder => remainder,
            };
            let mut surface = String::with_capacity(digits.len() + (digits.len() - 1) / 3);
            surface.push_str(&digits[..first_group_len]);
            let mut group_start = first_group_len;
            while group_start < digits.len() {
                surface.push(',');
                surface.push_str(&digits[group_start..group_start + 3]);
                group_start += 3;
            }
            surface
        }
    }
}

fn nonzero_unsigned_decimal_parser(function: &syn::Ident) -> TokenStream {
    quote! {
        fn #function(input: &str) -> Option<::std::num::NonZeroU32> {
            let value = input.replace(',', "").parse::<u32>().ok()?;
            let digits = value.to_string();
            let first_group_len = match digits.len() % 3 {
                0 => 3,
                remainder => remainder,
            };
            let mut canonical = String::with_capacity(digits.len() + (digits.len() - 1) / 3);
            canonical.push_str(&digits[..first_group_len]);
            let mut group_start = first_group_len;
            while group_start < digits.len() {
                canonical.push(',');
                canonical.push_str(&digits[group_start..group_start + 3]);
                group_start += 3;
            }
            (canonical == input).then_some(())?;
            ::std::num::NonZeroU32::new(value)
        }
    }
}

fn emit_structural_surface_runtime(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let carriers = structural_sequence_carriers(plan);
    if carriers.is_empty() {
        return Ok(Vec::new());
    }
    let owner_variants = carriers.iter().map(|carrier| {
        ident(&format!(
            "{}{}",
            crate::identifier::pascal_case(carrier.owner),
            crate::identifier::pascal_case(carrier.field.name()),
        ))
    });
    let separator_arms = sequence_separator_arms(plan, &carriers)?;
    let terminator_arms = sequence_terminator_arms(plan, &carriers)?;
    let origins = plan.declaration_keys().to_vec();
    Ok(vec![
        GeneratedItem::new(
            ItemKey::named_type(crate::identifier::SEQUENCE_OWNER_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum SequenceOwner { #(#owner_variants),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type(crate::identifier::FIXED_SURFACE_ATOM_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct FixedSurfaceAtom {
                    pub(crate) text: &'static str,
                    pub(crate) stable_id: &'static str,
                    pub(crate) transition: StructuralTransition,
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: crate::identifier::SEQUENCE_SEPARATOR_FUNCTION.to_owned(),
            },
            quote! {
                pub(crate) fn sequence_separator(
                    owner: SequenceOwner,
                    member_count: usize,
                    edge_index: usize,
                ) -> &'static [FixedSurfaceAtom] {
                    match owner { #(#separator_arms,)* }
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: crate::identifier::SEQUENCE_TERMINATOR_FUNCTION.to_owned(),
            },
            quote! {
                pub(crate) fn sequence_terminator(
                    owner: SequenceOwner,
                ) -> &'static [FixedSurfaceAtom] {
                    match owner { #(#terminator_arms,)* }
                }
            },
            origins,
        ),
    ])
}

fn sequence_separator_arms(
    plan: &SemanticPlan,
    carriers: &[super::StructuralCarrier<'_>],
) -> syn::Result<Vec<TokenStream>> {
    carriers
        .iter()
        .map(|carrier| -> syn::Result<TokenStream> {
            let variant = ident(&format!(
                "{}{}",
                crate::identifier::pascal_case(carrier.owner),
                crate::identifier::pascal_case(carrier.field.name()),
            ));
            let StructuralFieldKindPlan::Sequence {
                bounds, surface, ..
            } = carrier.field.kind()
            else {
                return Err(internal("sequence surface owner is not a sequence"));
            };
            let minimum = bounds.min();
            let valid_member_count = bounds.max().map_or_else(
                || quote! { member_count >= #minimum },
                |maximum| quote! { (#minimum..=#maximum).contains(&member_count) },
            );
            let valid_edge = quote! {
                #valid_member_count
                    && member_count
                        .checked_sub(1)
                        .is_some_and(|edge_count| edge_index < edge_count)
            };
            let body = match surface.separator() {
                None => quote! { &[] },
                Some(SeparatorPlan::Uniform(surface)) => {
                    let atoms = fixed_surface_atoms(
                        plan,
                        carrier.owner,
                        carrier.field.name(),
                        surface,
                        crate::emit::rules::StructuralSurfacePolicy::SeparatorUniform,
                    )?;
                    quote! { if #valid_edge { #atoms } else { &[] } }
                }
                Some(SeparatorPlan::Positional(rows)) => {
                    let arms = positional_separator_arms(plan, carrier, rows)?;
                    quote! {{ if !(#valid_edge) { return &[]; } #(#arms)* &[] }}
                }
            };
            Ok(quote! { SequenceOwner::#variant => #body })
        })
        .collect()
}

fn positional_separator_arms(
    plan: &SemanticPlan,
    carrier: &super::StructuralCarrier<'_>,
    rows: &[crate::semantic::PositionalSeparatorPlan],
) -> syn::Result<Vec<TokenStream>> {
    rows.iter()
        .map(|row| -> syn::Result<TokenStream> {
            let condition = match row.class() {
                crate::semantic::EdgeClass::Pair => {
                    quote! { member_count == 2 && edge_index == 0 }
                }
                crate::semantic::EdgeClass::First => {
                    quote! { member_count >= 3 && edge_index == 0 }
                }
                crate::semantic::EdgeClass::Middle => quote! {
                    member_count >= 4 && edge_index > 0 && edge_index < member_count - 2
                },
                crate::semantic::EdgeClass::Last => {
                    quote! { member_count >= 3 && edge_index == member_count - 2 }
                }
            };
            let atoms = fixed_surface_atoms(
                plan,
                carrier.owner,
                carrier.field.name(),
                row.surface(),
                crate::emit::rules::StructuralSurfacePolicy::SeparatorPositional(row.class()),
            )?;
            Ok(quote! { if #condition { return #atoms; } })
        })
        .collect()
}

fn sequence_terminator_arms(
    plan: &SemanticPlan,
    carriers: &[super::StructuralCarrier<'_>],
) -> syn::Result<Vec<TokenStream>> {
    carriers
        .iter()
        .map(|carrier| -> syn::Result<TokenStream> {
            let variant = ident(&format!(
                "{}{}",
                crate::identifier::pascal_case(carrier.owner),
                crate::identifier::pascal_case(carrier.field.name()),
            ));
            let StructuralFieldKindPlan::Sequence { surface, .. } = carrier.field.kind() else {
                return Err(internal("sequence terminator owner is not a sequence"));
            };
            let atoms = surface.terminator().map_or_else(
                || Ok(quote! { &[] }),
                |terminator| {
                    fixed_surface_atoms(
                        plan,
                        carrier.owner,
                        carrier.field.name(),
                        terminator,
                        crate::emit::rules::StructuralSurfacePolicy::Terminator,
                    )
                },
            )?;
            Ok(quote! { SequenceOwner::#variant => #atoms })
        })
        .collect()
}

fn structural_sequence_carriers(plan: &SemanticPlan) -> Vec<super::StructuralCarrier<'_>> {
    super::structural_carriers(plan)
        .into_iter()
        .filter(|carrier| matches!(carrier.kind, super::StructuralCarrierKind::Sequence))
        .collect()
}

fn fixed_surface_atoms(
    plan: &SemanticPlan,
    owner: &str,
    role: &str,
    surface: &crate::semantic::FixedSurfacePlan,
    policy: crate::emit::rules::StructuralSurfacePolicy,
) -> syn::Result<TokenStream> {
    let atoms = surface
        .atoms()
        .iter()
        .enumerate()
        .map(|(atom_index, atom)| -> syn::Result<TokenStream> {
            let text = fixed_surface_atom_text(plan, atom)?;
            let stable_id = syn::LitStr::new(
                &crate::emit::rules::structural_surface_stable_id(owner, role, policy, atom_index),
                Span::call_site(),
            );
            let transition = crate::emit::rules::emit_structural_transition(
                crate::emit::rules::structural_surface_transition(surface, policy, atom_index),
            );
            Ok(quote! {
                FixedSurfaceAtom {
                    text: #text,
                    stable_id: #stable_id,
                    transition: #transition,
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { &[#(#atoms),*] })
}

fn fixed_surface_atom_text(
    plan: &SemanticPlan,
    atom: &crate::semantic::FixedSurfaceAtomPlan,
) -> syn::Result<syn::LitStr> {
    match atom {
        crate::semantic::FixedSurfaceAtomPlan::Literal(value) => {
            Ok(syn::LitStr::new(value, Span::call_site()))
        }
        crate::semantic::FixedSurfaceAtomPlan::Lex { terminal, variant } => {
            let vocab = plan
                .terminals()
                .iter()
                .find_map(|terminal_plan| match terminal_plan {
                    TerminalPlan::Vocab(vocab) if vocab.name() == terminal => Some(vocab),
                    _ => None,
                })
                .ok_or_else(|| internal("fixed lexical surface lacks its vocabulary"))?;
            let value = vocab
                .variants()
                .iter()
                .find(|candidate| identifier_key(candidate.name()) == *variant)
                .map(crate::semantic::VocabVariantPlan::word)
                .ok_or_else(|| internal("fixed lexical surface lacks its vocabulary member"))?;
            Ok(value.clone())
        }
    }
}

fn emit_structural_renderers(
    plan: &SemanticPlan,
    root_names: &HashSet<String>,
) -> syn::Result<Vec<GeneratedItem>> {
    let mut items = Vec::new();
    for product in plan.products() {
        for field in product.fields() {
            if matches!(field.kind(), StructuralFieldKindPlan::Sequence { .. }) {
                items.push(emit_sequence_renderer(
                    plan,
                    product.name(),
                    field,
                    root_names,
                    DeclarationKind::AbstractProduct,
                )?);
            }
        }
        items.push(emit_product_renderer(plan, product, root_names)?);
    }
    for construction in plan.constructions() {
        for field in construction
            .fields()
            .iter()
            .filter_map(|field| field.structural_plan())
        {
            if matches!(field.kind(), StructuralFieldKindPlan::Sequence { .. }) {
                items.push(emit_sequence_renderer(
                    plan,
                    construction.element_type(),
                    field,
                    root_names,
                    DeclarationKind::Construction,
                )?);
            }
        }
        if plan.explicit_sum_owns_construction_category(construction.category()) {
            items.push(emit_explicit_sum_owned_construction_renderer(
                plan,
                construction,
                root_names,
            )?);
        }
    }
    for sum in plan.sums() {
        items.push(emit_sum_renderer(plan, sum, root_names)?);
    }
    Ok(items)
}

fn emit_explicit_sum_owned_construction_renderer(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    root_names: &HashSet<String>,
) -> syn::Result<GeneratedItem> {
    let function_name = crate::identifier::prefixed("render_", construction.element_type());
    let function = ident(&function_name);
    let ty = ident(construction.element_type());
    let mut allocator = LocalAllocator::default();
    allocator.reserve("writer");
    allocator.reserve("context");
    allocator.reserve("environment");
    let argument = allocator.allocate(&snake_case(construction.element_type()));
    let locals = RenderLocals {
        whole: Some(argument.clone()),
        fields: HashMap::new(),
        category: quote! { #argument },
    };
    let forms = construction
        .forms()
        .iter()
        .enumerate()
        .map(|(form_index, form)| {
            let statements = render_atoms(plan, construction, form, &locals, root_names)?;
            let guard =
                super::emit_form_guard_expression(construction, form_index, |domain, value| {
                    render_guard_atom(plan, construction, &locals, domain, value)
                })?;
            Ok((guard, statements))
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let body = if forms.len() == 1 {
        let (_, statements) = forms
            .into_iter()
            .next()
            .ok_or_else(|| internal("construction has no sealed render form"))?;
        quote! { #(#statements)* }
    } else {
        let branches = forms.iter().map(|(guard, statements)| {
            let guard = guard.as_ref().expect("multi-form rows are guarded");
            quote! { if #guard { #(#statements)* } else }
        });
        quote! { #(#branches)* { unreachable!("sealed form partition is total") } }
    };
    let environment = plan
        .needs_parser_environment()
        .then(|| quote! { , environment: &crate::environment::ParserEnvironment });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(
                writer: &mut Writer,
                #argument: &#ty,
                context: &ParseContext<'_> #environment,
            ) { #body }
        },
        vec![DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        )],
    ))
}

fn emit_product_renderer(
    plan: &SemanticPlan,
    product: &crate::semantic::ProductPlan,
    root_names: &HashSet<String>,
) -> syn::Result<GeneratedItem> {
    let function_name = crate::identifier::prefixed("render_", product.name());
    let function = ident(&function_name);
    let ty = ident(product.name());
    let value = ident(&snake_case(product.name()));
    let statements = product
        .fields()
        .iter()
        .map(|field| render_structural_field(plan, product.name(), field, &value, root_names))
        .collect::<syn::Result<Vec<_>>>()?;
    let environment = plan.needs_parser_environment().then(|| {
        quote! { , environment: &crate::environment::ParserEnvironment }
    });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            pub(crate) fn #function(
                writer: &mut Writer,
                #value: &#ty,
                context: &ParseContext<'_> #environment,
            ) { #(#statements)* }
        },
        vec![DeclarationKey::new(
            DeclarationKind::AbstractProduct,
            product.name(),
        )],
    ))
}

fn emit_sum_renderer(
    plan: &SemanticPlan,
    sum: &crate::semantic::SumPlan,
    root_names: &HashSet<String>,
) -> syn::Result<GeneratedItem> {
    let function_name = crate::identifier::prefixed("render_", sum.name());
    let function = ident(&function_name);
    let ty = ident(sum.name());
    let value = ident(&snake_case(sum.name()));
    let agreement_carry = plan.sum_carries_agreement(sum.name());
    let arms = sum
        .alternatives()
        .iter()
        .map(|alternative| -> syn::Result<TokenStream> {
            let variant = ident(alternative.name());
            let binding = ident("value");
            let statement = if agreement_carry {
                render_structural_value_with_feature(
                    plan,
                    alternative.value(),
                    &quote! { #binding },
                    root_names,
                    Feature::Agreement,
                    quote! { agreement },
                )?
            } else {
                render_structural_value(plan, alternative.value(), quote! { #binding }, root_names)?
            };
            Ok(quote! { #ty::#variant(#binding) => { #statement } })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let match_value = if sum.alternatives().is_empty() {
        quote! { *#value }
    } else {
        quote! { #value }
    };
    let environment = plan.needs_parser_environment().then(|| {
        quote! { , environment: &crate::environment::ParserEnvironment }
    });
    let agreement = agreement_carry.then(|| quote! { , agreement: Agreement });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            pub(crate) fn #function(
                writer: &mut Writer,
                #value: &#ty #agreement,
                context: &ParseContext<'_> #environment,
            ) { match #match_value { #(#arms),* } }
        },
        vec![DeclarationKey::new(
            DeclarationKind::AbstractSum,
            sum.name(),
        )],
    ))
}

fn emit_sequence_renderer(
    plan: &SemanticPlan,
    owner: &str,
    field: &crate::semantic::StructuralFieldPlan,
    root_names: &HashSet<String>,
    kind: DeclarationKind,
) -> syn::Result<GeneratedItem> {
    let StructuralFieldKindPlan::Sequence { item, .. } = field.kind() else {
        return Err(internal(
            "structural sequence renderer received a non-sequence",
        ));
    };
    let function_name = structural_sequence_renderer(owner, field.name());
    let function = ident(&function_name);
    let item_ty = super::value_kind_type(item);
    let owner_variant = ident(&format!(
        "{}{}",
        crate::identifier::pascal_case(owner),
        crate::identifier::pascal_case(field.name()),
    ));
    let sequence_feature = plan.sequence_feature(owner, field.name());
    let render_value = if let Some(feature) = sequence_feature {
        render_structural_value_with_feature(
            plan,
            item,
            &quote! { value },
            root_names,
            feature,
            quote! { sequence_feature },
        )?
    } else {
        render_structural_value(plan, item, quote! { value }, root_names)?
    };
    let feature_parameter = sequence_feature.map(|feature| {
        let ty = super::feature_type(feature);
        quote! { , sequence_feature: #ty }
    });
    let context = structural_value_requires_context(plan, item)?
        .then(|| quote! { , context: &ParseContext<'_> });
    let environment = plan.needs_parser_environment().then(|| {
        quote! { , environment: &crate::environment::ParserEnvironment }
    });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(
                writer: &mut Writer,
                values: &[#item_ty] #feature_parameter #context #environment,
            ) {
                for (index, value) in values.iter().enumerate() {
                    #render_value
                    for atom in sequence_terminator(SequenceOwner::#owner_variant) {
                        writer.claim(
                            || LexicalOwner::static_owner(
                                LexicalProvenanceKind::FormLiteral,
                                atom.stable_id,
                            ),
                            |writer| writer.structural_surface(
                                atom.text,
                                atom.transition,
                            ),
                        );
                    }
                    if index + 1 < values.len() {
                        for atom in sequence_separator(
                            SequenceOwner::#owner_variant,
                            values.len(),
                            index,
                        ) {
                            writer.claim(
                                || LexicalOwner::static_owner(
                                    LexicalProvenanceKind::FormLiteral,
                                    atom.stable_id,
                                ),
                                |writer| writer.structural_surface(
                                    atom.text,
                                    atom.transition,
                                ),
                            );
                        }
                    }
                }
            }
        },
        vec![DeclarationKey::new(kind, owner)],
    ))
}

fn render_structural_value_with_feature(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    expression: &TokenStream,
    root_names: &HashSet<String>,
    feature: Feature,
    feature_value: TokenStream,
) -> syn::Result<TokenStream> {
    if feature != Feature::Agreement {
        return Err(internal("unsupported generated sequence feature renderer"));
    }
    match value {
        ValueKindPlan::Category(category) => {
            let capability = plan.category_render_capability(category);
            let function = render_category_name(category, root_names.contains(category));
            let agreement = capability
                .requires_external_agreement()
                .then_some(feature_value);
            let context = capability.requires_context().then(|| quote! { context });
            let environment = plan
                .needs_parser_environment()
                .then(|| quote! { environment });
            let tail = signature_tail(&[agreement, context, environment]);
            Ok(quote! { #function(writer, #expression #tail); })
        }
        ValueKindPlan::Sum(sum) if plan.sum_carries_agreement(sum) => {
            let function = ident(&crate::identifier::prefixed("render_", sum));
            let environment = plan
                .needs_parser_environment()
                .then(|| quote! { , environment });
            Ok(quote! {
                #function(writer, #expression, #feature_value, context #environment);
            })
        }
        ValueKindPlan::Product(_)
        | ValueKindPlan::Sum(_)
        | ValueKindPlan::Lex(_)
        | ValueKindPlan::Identity(_) => {
            Err(internal("sequence feature item does not carry agreement"))
        }
    }
}

fn render_structural_field(
    plan: &SemanticPlan,
    owner: &str,
    field: &crate::semantic::StructuralFieldPlan,
    whole: &syn::Ident,
    root_names: &HashSet<String>,
) -> syn::Result<TokenStream> {
    let name = ident(field.name());
    match field.kind() {
        StructuralFieldKindPlan::Required(value) => {
            render_structural_value(plan, value, quote! { &#whole.#name }, root_names)
        }
        StructuralFieldKindPlan::Optional(value) => {
            let statement = render_structural_value(plan, value, quote! { value }, root_names)?;
            Ok(quote! {
                if let Some(value) = #whole.#name.as_ref() { #statement }
            })
        }
        StructuralFieldKindPlan::Sequence { item, .. } => {
            let function = ident(&structural_sequence_renderer(owner, field.name()));
            let context =
                structural_value_requires_context(plan, item)?.then(|| quote! { , context });
            let environment = plan
                .needs_parser_environment()
                .then(|| quote! { , environment });
            Ok(quote! { #function(writer, &#whole.#name #context #environment); })
        }
    }
}

fn structural_value_requires_context(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
) -> syn::Result<bool> {
    match value {
        ValueKindPlan::Category(category) => {
            Ok(plan.category_render_capability(category).requires_context())
        }
        ValueKindPlan::Product(_) | ValueKindPlan::Sum(_) => Ok(true),
        ValueKindPlan::Lex(_) => Ok(false),
        ValueKindPlan::Identity(name) => {
            if find_context_identity(plan, name).is_some() {
                Ok(true)
            } else if find_catalog_identity(plan, name).is_some() {
                Ok(false)
            } else {
                find_binding(plan, name)?;
                Ok(true)
            }
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "structural rendering exhaustively dispatches every sealed value family"
)]
fn render_structural_value(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    expression: TokenStream,
    root_names: &HashSet<String>,
) -> syn::Result<TokenStream> {
    let environment = plan
        .needs_parser_environment()
        .then(|| quote! { , environment });
    match value {
        ValueKindPlan::Category(category) => {
            let capability = plan.category_render_capability(category);
            if capability.requires_external_agreement() {
                return Err(internal(
                    "structural category rendering cannot supply external agreement",
                ));
            }
            let function = render_category_name(category, root_names.contains(category));
            let context = capability.requires_context().then(|| quote! { , context });
            Ok(quote! { #function(writer, #expression #context #environment); })
        }
        ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let function = ident(&crate::identifier::prefixed("render_", name));
            Ok(quote! { #function(writer, #expression, context #environment); })
        }
        ValueKindPlan::Lex(name) => {
            if let Some(vocab) = find_vocab(plan, name) {
                let function = ident(&format!("render_{}", snake_case(vocab.name())));
                let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
                let owner_arms = vocab.variants().iter().map(|variant| {
                    let member =
                        emitted_ident(&identifier_key(variant.name()), variant.name().span());
                    let stable_id = syn::LitStr::new(
                        &format!("vocab:{}/{}", vocab.name(), member),
                        Span::call_site(),
                    );
                    quote! {
                        #ty::#member => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Vocab,
                            #stable_id,
                        )
                    }
                });
                Ok(quote! {
                    writer.claim(
                        || match *#expression { #(#owner_arms,)* },
                        |writer| #function(writer, *#expression),
                    );
                })
            } else if let Some(codec) = find_signed_decimal(plan, name) {
                let function = ident(&format!("render_{}", snake_case(codec.codec_name())));
                let stable_id = syn::LitStr::new(&format!("codec:{name}"), Span::call_site());
                Ok(quote! {
                    writer.claim(
                        || LexicalOwner::static_owner(
                            LexicalProvenanceKind::Codec,
                            #stable_id,
                        ),
                        |writer| #function(writer, #expression),
                    );
                })
            } else if let Some(codec) = find_unsigned_number(plan, name) {
                let function = ident(&format!("render_{}", snake_case(codec.codec_name())));
                let stable_id = syn::LitStr::new(&format!("codec:{name}"), Span::call_site());
                Ok(quote! {
                    writer.claim(
                        || LexicalOwner::static_owner(
                            LexicalProvenanceKind::Codec,
                            #stable_id,
                        ),
                        |writer| #function(writer, #expression),
                    );
                })
            } else {
                let binding = find_binding(plan, name)?;
                let Some(BindingRenderPlan::Runtime(function)) = binding.render() else {
                    return Err(internal("structural lexical value lacks runtime rendering"));
                };
                let expression = match binding.traversal().mode() {
                    VisitMode::Copy => quote! { *#expression },
                    VisitMode::Borrowed => expression,
                };
                let (kind, prefix) = match binding.kind() {
                    crate::model::TerminalBindingKind::Codec => {
                        (quote! { LexicalProvenanceKind::Codec }, "codec")
                    }
                    crate::model::TerminalBindingKind::Identity => {
                        (quote! { LexicalProvenanceKind::Identity }, "identity")
                    }
                };
                let stable_id = syn::LitStr::new(&format!("{prefix}:{name}"), Span::call_site());
                Ok(quote! {
                    writer.claim(
                        || LexicalOwner::static_owner(#kind, #stable_id),
                        |writer| #function(writer, #expression),
                    );
                })
            }
        }
        ValueKindPlan::Identity(name) => {
            if let Some(identity) = find_context_identity(plan, name) {
                let ty = identity.ident();
                let owner_arms = identity.arms().iter().map(|arm| {
                    let member = arm.variant();
                    let stable_id =
                        syn::LitStr::new(&format!("identity:{name}/{member}"), Span::call_site());
                    quote! {
                        #ty::#member => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Identity,
                            #stable_id,
                        )
                    }
                });
                Ok(quote! {
                    writer.claim(
                        || match *#expression { #(#owner_arms,)* },
                        |writer| writer.identity((#expression).surface(context)),
                    );
                })
            } else if find_catalog_identity(plan, name).is_some() {
                Ok(quote! {
                    writer.claim(
                        || LexicalOwner::catalog_owner(
                            (#expression).provider(),
                            (#expression).canonical_identity.clone(),
                        ),
                        |writer| writer.identity(
                            environment
                                .catalog_surface(
                                    (#expression).provider(),
                                    (#expression).canonical_identity(),
                                )
                                .expect("validated catalog identity remains in its frozen provider"),
                        ),
                    );
                })
            } else {
                let binding = find_binding(plan, name)?;
                let Some(BindingRenderPlan::Runtime(function)) = binding.render() else {
                    return Err(internal("structural identity lacks runtime rendering"));
                };
                let expression = match binding.traversal().mode() {
                    VisitMode::Copy => quote! { *#expression },
                    VisitMode::Borrowed => expression,
                };
                let stable_id = syn::LitStr::new(&format!("identity:{name}"), Span::call_site());
                Ok(quote! {
                    writer.claim(
                        || LexicalOwner::static_owner(
                            LexicalProvenanceKind::Identity,
                            #stable_id,
                        ),
                        |writer| #function(writer, #expression, context),
                    );
                })
            }
        }
    }
}

struct VocabFeatureHelper<'a> {
    feature: Feature,
    vocab: &'a VocabPlan,
    values: Vec<FeatureValue>,
    origins: Vec<DeclarationKey>,
}

fn collect_vocab_feature_helpers(
    validated: &SemanticPlan,
) -> syn::Result<Vec<VocabFeatureHelper<'_>>> {
    let mut helpers: Vec<VocabFeatureHelper<'_>> = Vec::new();
    for construction in validated.constructions() {
        for equation in validated.feature_equations(construction.construction_id()) {
            let FeatureExpr::MatchVocab { role, arms } = equation.value() else {
                continue;
            };
            let feature = match equation.target() {
                FeaturePlace::Construction(feature) | FeaturePlace::Role { feature, .. } => {
                    *feature
                }
            };
            let field = construction.field(&identifier_key(role))?;
            let vocab = find_vocab(validated, field.terminal()).ok_or_else(|| {
                internal("sealed vocabulary feature match is not rooted in a vocabulary")
            })?;
            let values = vocab
                .variants()
                .iter()
                .map(|variant| {
                    arms.iter()
                        .find(|(member, _)| {
                            identifier_key(member.value()) == identifier_key(variant.name())
                        })
                        .map(|(_, value)| *value)
                        .ok_or_else(|| {
                            internal("sealed vocabulary feature match is not exhaustive")
                        })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let origin = DeclarationKey::new(
                DeclarationKind::Construction,
                construction.construction_id(),
            );
            if let Some(existing) = helpers
                .iter_mut()
                .find(|helper| helper.feature == feature && helper.vocab.name() == vocab.name())
            {
                if existing.values != values {
                    return Err(internal(&format!(
                        "sealed vocabulary feature mapping is inconsistent for `{}.{}`",
                        vocab.name(),
                        feature_name(feature),
                    )));
                }
                if !existing.origins.contains(&origin) {
                    existing.origins.push(origin);
                }
            } else {
                helpers.push(VocabFeatureHelper {
                    feature,
                    vocab,
                    values,
                    origins: vec![origin],
                });
            }
        }
    }
    Ok(helpers)
}

fn emit_vocab_feature_helper(helper: VocabFeatureHelper<'_>) -> GeneratedItem {
    let function_name = feature_helper(feature_name(helper.feature), helper.vocab.name());
    let function = ident(&function_name);
    let ty = emitted_ident(helper.vocab.name(), helper.vocab.name_ident().span());
    let return_ty = match helper.feature {
        Feature::Agreement => quote! { Agreement },
        Feature::Cardinality => quote! { Cardinality },
        Feature::Number => quote! { Number },
        Feature::Onset => quote! { Onset },
        Feature::PossessiveEnding => quote! { PossessiveEnding },
    };
    let arms = helper
        .vocab
        .variants()
        .iter()
        .zip(helper.values)
        .map(|(variant, value)| {
            let variant = emitted_ident(&identifier_key(variant.name()), variant.name().span());
            let value = feature_value(value);
            quote! { #ty::#variant => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: #ty) -> #return_ty {
                match value { #(#arms,)* }
            }
        },
        helper.origins,
    )
}

fn signature_tail(parts: &[Option<TokenStream>]) -> TokenStream {
    let parts = parts.iter().flatten();
    quote! { #(, #parts)* }
}

fn render_base_allocator(
    validated: &SemanticPlan,
    root_impl: bool,
    takes_agreement: bool,
    takes_context: bool,
) -> LocalAllocator {
    let mut allocator = LocalAllocator::default();
    allocator.reserve("writer");
    if root_impl {
        allocator.reserve("self");
    }
    if takes_agreement {
        allocator.reserve("agreement");
    }
    if takes_context {
        allocator.reserve("context");
    }
    if validated.needs_parser_environment() {
        allocator.reserve("environment");
    }
    allocator
}

#[expect(
    clippy::too_many_lines,
    reason = "exhaustive allocator reservation follows every render atom shape"
)]
fn render_allocator(
    validated: &SemanticPlan,
    members: &[&ConstructionPlan],
    root_impl: bool,
    root_names: &HashSet<String>,
    takes_agreement: bool,
    takes_context: bool,
) -> syn::Result<LocalAllocator> {
    let mut allocator = render_base_allocator(validated, root_impl, takes_agreement, takes_context);
    for construction in members {
        let fields = construction
            .fields()
            .iter()
            .map(|field| (field.name_key(), field))
            .collect::<HashMap<_, _>>();
        for atom in construction.forms().iter().flat_map(FormPlan::atoms) {
            let atom = atom.value_atom();
            match atom {
                AtomPlan::Literal(_) | AtomPlan::SentenceInitialLiteral(_) => {}
                AtomPlan::Category { role, .. } => {
                    let field = fields
                        .get(role)
                        .ok_or_else(|| internal("resolved role is absent"))?;
                    if field.kind() != ConstructionFieldKind::Category {
                        return Err(internal("bare role is not a category"));
                    }
                    let category = field.terminal();
                    allocator.reserve(
                        render_category_name(category, root_names.contains(category)).to_string(),
                    );
                    if validated.category_requires_external_agreement(category)
                        && let Some(equation) = validated
                            .feature_equations(construction.construction_id())
                            .iter()
                            .find(|equation| {
                                matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == role.as_str())
                            })
                    {
                        reserve_feature_callees(
                            validated,
                            construction,
                            equation.value(),
                            &mut allocator,
                        )?;
                    }
                }
                AtomPlan::Lex { role, .. } => {
                    let field = fields
                        .get(role)
                        .ok_or_else(|| internal("resolved lexical role is absent"))?;
                    let terminal = field.terminal();
                    if let Some(vocab) = find_vocab(validated, terminal) {
                        allocator.reserve(format!("render_{}", snake_case(vocab.name())));
                    } else if let Some(codec) = find_signed_decimal(validated, terminal) {
                        allocator.reserve(format!("render_{}", snake_case(codec.codec_name())));
                    } else if let Some(codec) = find_unsigned_number(validated, terminal) {
                        allocator.reserve(format!("render_{}", snake_case(codec.codec_name())));
                    } else if let BindingRenderPlan::Runtime(path) =
                        find_binding(validated, terminal)?
                            .render()
                            .ok_or_else(|| internal("lex terminal lacks render metadata"))?
                    {
                        reserve_bare_path(&mut allocator, path);
                    }
                }
                AtomPlan::Identity { role, .. } => {
                    let field = fields
                        .get(role)
                        .ok_or_else(|| internal("resolved identity role is absent"))?;
                    if find_context_identity(validated, field.terminal()).is_none()
                        && find_catalog_identity(validated, field.terminal()).is_none()
                        && let BindingRenderPlan::Runtime(path) =
                            find_binding(validated, field.terminal())?
                                .render()
                                .ok_or_else(|| internal("identity lacks render metadata"))?
                    {
                        reserve_bare_path(&mut allocator, path);
                    }
                }
                AtomPlan::VerbFixed { terminal, .. } => {
                    let lexeme = find_lexeme(validated, terminal)
                        .ok_or_else(|| internal("fixed verb lacks its sealed lexeme plan"))?;
                    allocator.reserve(lexeme_surface_helper(lexeme.name()));
                    let equations = validated.feature_equations(construction.construction_id());
                    if let Some(equation) = equations.iter().find(|equation| {
                        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == "verb")
                    }) {
                        reserve_feature_callees(
                            validated,
                            construction,
                            equation.value(),
                            &mut allocator,
                        )?;
                    }
                }
                AtomPlan::OpenDeclaration(_) => {
                    let equations = validated.feature_equations(construction.construction_id());
                    if let Some(equation) = equations.iter().find(|equation| {
                        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == "verb")
                    }) {
                        reserve_feature_callees(
                            validated,
                            construction,
                            equation.value(),
                            &mut allocator,
                        )?;
                    }
                }
                AtomPlan::Noun { role, .. } => {
                    let field = fields
                        .get(role)
                        .ok_or_else(|| internal("resolved noun role is absent"))?;
                    if validated
                        .runtime_declaration_noun_for(field.terminal())
                        .is_none()
                    {
                        if let Some(lexeme) = find_lexeme(validated, field.terminal()) {
                            allocator.reserve(lexeme_surface_helper(lexeme.name()));
                        } else {
                            let binding = find_binding(validated, field.terminal())?;
                            let Some(BindingRenderPlan::Runtime(path)) = binding.render() else {
                                return Err(internal("noun terminal lacks runtime render binding"));
                            };
                            reserve_bare_path(&mut allocator, path);
                        }
                    } else if let Some((_, codec)) =
                        validated.runtime_declaration_noun_for(field.terminal())
                        && codec.closed_lexeme().is_some()
                    {
                        let lexeme = validated.runtime_noun_lexeme().ok_or_else(|| {
                            internal("declaration noun lacks its sealed lexeme plan")
                        })?;
                        allocator.reserve(lexeme_surface_helper(lexeme.name()));
                    }
                    let role_number = FeaturePlace::Role {
                        field: syn::Ident::new(role, construction.origin_span()),
                        feature: Feature::Number,
                    };
                    if let Some(writer) = validated
                        .feature_equations(construction.construction_id())
                        .iter()
                        .find(|equation| equation.target() == &role_number)
                    {
                        reserve_feature_callees(
                            validated,
                            construction,
                            writer.value(),
                            &mut allocator,
                        )?;
                    } else {
                        allocator.reserve(feature_helper("number", construction.category()));
                    }
                }
                AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
                    unreachable!("value_atom removes form wrappers")
                }
            }
        }
    }
    Ok(allocator)
}

fn reserve_bare_path(allocator: &mut LocalAllocator, path: &syn::Path) {
    if path.segments.len() == 1 {
        allocator.reserve_ident(&path.segments[0].ident);
    }
}

fn reserve_feature_callees(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    expression: &FeatureExpr,
    allocator: &mut LocalAllocator,
) -> syn::Result<()> {
    let FeatureExpr::FromRole {
        role,
        feature: source_feature,
    } = expression
    else {
        return Ok(());
    };
    if !construction
        .fields()
        .iter()
        .any(|field| field.name_key() == identifier_key(role))
    {
        if let Some(writer) = validated
            .feature_equations(construction.construction_id())
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
            })
        {
            reserve_feature_callees(validated, construction, writer.value(), allocator)?;
        }
        return Ok(());
    }
    let field = construction.field(&identifier_key(role))?;
    if let Some(writer) = validated
            .feature_equations(construction.construction_id())
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
            })
        && (field.kind() == ConstructionFieldKind::Category
            || !matches!(writer.value(), FeatureExpr::MatchVocab { .. }))
    {
        return reserve_feature_callees(validated, construction, writer.value(), allocator);
    }
    if matches!(*source_feature, Feature::Onset | Feature::PossessiveEnding)
        && field.kind() != ConstructionFieldKind::Category
    {
        return Ok(());
    }
    if matches!(*source_feature, Feature::Cardinality | Feature::Number)
        && field.kind() == ConstructionFieldKind::Lex
        && let Some(codec) = find_unsigned_number(validated, field.terminal())
        && codec.kind() == UnsignedNumberKind::EnglishCardinal
    {
        allocator.reserve(feature_helper(
            feature_name(*source_feature),
            codec.codec_name(),
        ));
        return Ok(());
    }
    let source = match field.kind() {
        ConstructionFieldKind::Category => field.terminal().to_owned(),
        ConstructionFieldKind::Lex => {
            let (_, vocabulary) =
                canonical_lexical_feature_lowering(validated, construction, role, *source_feature)?;
            vocabulary.to_owned()
        }
        ConstructionFieldKind::Identity => {
            return Err(internal(
                "identity roles cannot provide grammatical features",
            ));
        }
    };
    allocator.reserve(format!(
        "{}_for_{}",
        feature_name(*source_feature),
        snake_case(&source)
    ));
    Ok(())
}

struct RenderLocals {
    whole: Option<syn::Ident>,
    fields: HashMap<String, syn::Ident>,
    category: TokenStream,
}

fn render_guard_atom(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    locals: &RenderLocals,
    domain: &FiniteDomainPlan,
    value: &FiniteValuePlan,
) -> syn::Result<TokenStream> {
    match (domain.kind(), value) {
        (FiniteDomainKindPlan::Vocab { terminal, .. }, FiniteValuePlan::Vocab(variant)) => {
            let field_value = field_value(construction, domain.role(), locals)?;
            let terminal = ident(terminal);
            let variant = ident(variant);
            let value = copy_value(construction, domain.role(), field_value)?;
            Ok(quote! { matches!(#value, #terminal::#variant) })
        }
        (
            FiniteDomainKindPlan::OptionalVocab { terminal, .. },
            FiniteValuePlan::OptionalVocab(variant),
        ) => {
            let field_value = field_value(construction, domain.role(), locals)?;
            if let Some(variant) = variant {
                let terminal = ident(terminal);
                let variant = ident(variant);
                let value = copy_value(construction, domain.role(), field_value)?;
                Ok(quote! { matches!(#value, Some(#terminal::#variant)) })
            } else {
                Ok(quote! { #field_value.is_none() })
            }
        }
        (FiniteDomainKindPlan::OptionalPresence, FiniteValuePlan::OptionalPresence(present)) => {
            let field_value = field_value(construction, domain.role(), locals)?;
            Ok(quote! { #field_value.is_some() == #present })
        }
        (FiniteDomainKindPlan::Feature { feature, .. }, FiniteValuePlan::Feature(value)) => {
            let actual = if domain.role().starts_with('@') {
                let equation = validated
                    .feature_equations(construction.construction_id())
                    .iter()
                    .find(|equation| equation.target() == &FeaturePlace::Construction(*feature))
                    .ok_or_else(|| internal("construction feature guard has no equation"))?;
                feature_expr(validated, construction, equation.value(), *feature, locals)?
            } else {
                let role = syn::Ident::new(domain.role(), construction.origin_span());
                let expression = FeatureExpr::FromRole {
                    role,
                    feature: *feature,
                };
                feature_expr(validated, construction, &expression, *feature, locals)?
            };
            let expected = feature_value(*value);
            Ok(quote! { #actual == #expected })
        }
        _ => Err(internal("form guard domain and assignment value disagree")),
    }
}

fn render_arm_requires_block(construction: &ConstructionPlan, root_impl: bool) -> bool {
    matches!(
        construction.forms()[0].atoms(),
        [AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. }]
    ) || (!root_impl && matches!(construction.forms()[0].atoms(), [AtomPlan::Category { .. }]))
}

fn render_arms(
    validated: &SemanticPlan,
    members: &[&ConstructionPlan],
    root_impl: bool,
    root_names: &HashSet<String>,
    allocator: &LocalAllocator,
    category_value: &TokenStream,
) -> syn::Result<Vec<TokenStream>> {
    members
        .iter()
        .map(|construction| {
            let mut allocator = allocator.clone();
            let variant = ident(construction.category_variant());
            let element = ident(construction.element_type());
            let qualifier = if root_impl {
                quote! { Self }
            } else {
                let category = ident(construction.category());
                quote! { #category }
            };
            let whole = needs_whole_value(construction)
                .then(|| allocator.allocate(construction.construction_id()));
            let mut field_locals = HashMap::new();
            let pattern = if construction.fields().is_empty() {
                quote! { #qualifier::#variant(#element) }
            } else if let Some(whole) = &whole {
                quote! { #qualifier::#variant(#whole) }
            } else {
                let fields = construction
                    .fields()
                    .iter()
                    .map(|field| {
                        let source = field.name();
                        let local = allocator.allocate_ident(source);
                        field_locals.insert(identifier_key(source), local.clone());
                        if local == *source {
                            quote! { #source }
                        } else {
                            quote! { #source: #local }
                        }
                    })
                    .collect::<Vec<_>>();
                quote! { #qualifier::#variant(#element { #(#fields),* }) }
            };
            let locals = RenderLocals {
                whole,
                fields: field_locals,
                category: category_value.clone(),
            };
            let form_bodies = construction
                .forms()
                .iter()
                .enumerate()
                .map(|(form_index, form)| {
                    let statements =
                        render_atoms(validated, construction, form, &locals, root_names)?;
                    let guard = super::emit_form_guard_expression(
                        construction,
                        form_index,
                        |domain, value| {
                            render_guard_atom(validated, construction, &locals, domain, value)
                        },
                    )?;
                    Ok((guard, statements))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            if construction.forms().len() > 1 {
                let branches = form_bodies.iter().map(|(guard, statements)| {
                    let guard = guard.as_ref().expect("multi-form rows are guarded");
                    quote! { if #guard { #(#statements)* } else }
                });
                return Ok(quote! {
                    #pattern => { #(#branches)* { unreachable!("sealed form partition is total") } }
                });
            }
            let (_, statements) = form_bodies
                .into_iter()
                .next()
                .ok_or_else(|| internal("construction has no sealed render form"))?;
            if statements.len() == 1 && !render_arm_requires_block(construction, root_impl) {
                let statement = syn::parse2::<syn::Stmt>(
                    statements
                        .into_iter()
                        .next()
                        .ok_or_else(|| internal("one render statement is absent"))?,
                )?;
                let syn::Stmt::Expr(expression, _) = statement else {
                    return Err(internal(
                        "render atom did not lower to an expression statement",
                    ));
                };
                Ok(quote! { #pattern => #expression, })
            } else {
                Ok(quote! { #pattern => { #(#statements)* } })
            }
        })
        .collect()
}

fn render_atoms(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
) -> syn::Result<Vec<TokenStream>> {
    let method_writer = quote! { writer };
    let fields = construction
        .fields()
        .iter()
        .map(|field| (field.name_key(), field))
        .collect::<HashMap<_, _>>();
    form.atoms()
        .iter()
        .enumerate()
        .map(|(atom_index, atom)| {
            if let AtomPlan::Bound {
                direction,
                affix,
                value,
            } = atom
            {
                return render_bound_atom(
                    validated,
                    construction,
                    form,
                    atom_index,
                    *direction,
                    affix,
                    value,
                    locals,
                    root_names,
                    &fields,
                );
            }
            if let AtomPlan::Circumfix {
                prefix,
                value,
                suffix,
            } = atom
            {
                return render_circumfix_atom(
                    validated,
                    construction,
                    form,
                    atom_index,
                    prefix,
                    value,
                    suffix,
                    locals,
                    root_names,
                    &fields,
                );
            }
            if let Some(role) = render_atom_role(atom)
                && let Some(field) = fields.get(role)
                && let Some(structural) = field.structural_plan()
            {
                return render_construction_structural_field(
                    validated,
                    construction,
                    role,
                    structural,
                    locals,
                    root_names,
                );
            }
            if matches!(atom, AtomPlan::Category { .. }) {
                return render_atom_statement(
                    validated,
                    construction,
                    atom,
                    locals,
                    root_names,
                    &fields,
                );
            }
            let statement =
                render_atom_statement(validated, construction, atom, locals, root_names, &fields)?;
            let owner = render_owner(
                validated,
                construction,
                form.name(),
                atom_index,
                atom,
                locals,
            )?;
            Ok(quote! {
                #method_writer.claim(
                    || #owner,
                    |writer| { #statement },
                );
            })
        })
        .collect()
}

#[expect(
    clippy::too_many_arguments,
    reason = "bound rendering preserves the enclosing form and value ownership inputs"
)]
fn render_bound_atom(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    atom_index: usize,
    direction: crate::semantic::BoundDirectionPlan,
    affix: &str,
    value: &AtomPlan,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<TokenStream> {
    let affix_statement = render_fixed_surface_statement(affix);
    let affix_id = syn::LitStr::new(
        &format!(
            "form:{}/{}/{atom_index}/affix",
            construction.construction_id(),
            form.name(),
        ),
        Span::call_site(),
    );
    let affix_claim = quote! {
        writer.claim(
            || LexicalOwner::static_owner(
                LexicalProvenanceKind::FormLiteral,
                #affix_id,
            ),
            |writer| { #affix_statement },
        );
    };
    let value_statement = if let Some(role) = render_atom_role(value)
        && let Some(field) = fields.get(role)
        && let Some(structural) = field.structural_plan()
    {
        render_construction_structural_field(
            validated,
            construction,
            role,
            structural,
            locals,
            root_names,
        )?
    } else {
        render_atom_statement(validated, construction, value, locals, root_names, fields)?
    };
    let value_render = if matches!(value, AtomPlan::Category { .. }) {
        value_statement
    } else {
        let owner = render_owner(
            validated,
            construction,
            form.name(),
            atom_index,
            value,
            locals,
        )?;
        quote! {
            writer.claim(
                || #owner,
                |writer| { #value_statement },
            );
        }
    };
    Ok(match direction {
        crate::semantic::BoundDirectionPlan::Prefix => quote! {
            #affix_claim
            writer.suppress_next_space();
            #value_render
        },
        crate::semantic::BoundDirectionPlan::Suffix => quote! {
            #value_render
            writer.suppress_next_space();
            #affix_claim
        },
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "circumfix rendering preserves the enclosing form and delegated payload inputs"
)]
fn render_circumfix_atom(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    atom_index: usize,
    prefix: &str,
    value: &AtomPlan,
    suffix: &str,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<TokenStream> {
    let affix_claim = |surface: &str, side: &str| {
        let statement = render_fixed_surface_statement(surface);
        let stable_id = syn::LitStr::new(
            &format!(
                "form:{}/{}/{atom_index}/{side}",
                construction.construction_id(),
                form.name(),
            ),
            Span::call_site(),
        );
        quote! {
            writer.claim(
                || LexicalOwner::static_owner(
                    LexicalProvenanceKind::FormLiteral,
                    #stable_id,
                ),
                |writer| { #statement },
            );
        }
    };
    let prefix_claim = affix_claim(prefix, "prefix");
    let suffix_claim = affix_claim(suffix, "suffix");
    let value_statement = if let Some(role) = render_atom_role(value)
        && let Some(field) = fields.get(role)
        && let Some(structural) = field.structural_plan()
    {
        render_construction_structural_field(
            validated,
            construction,
            role,
            structural,
            locals,
            root_names,
        )?
    } else {
        render_atom_statement(validated, construction, value, locals, root_names, fields)?
    };
    let value_render = if matches!(value, AtomPlan::Category { .. }) {
        value_statement
    } else {
        let owner = render_owner(
            validated,
            construction,
            form.name(),
            atom_index,
            value,
            locals,
        )?;
        quote! {
            writer.claim(
                || #owner,
                |writer| { #value_statement },
            );
        }
    };
    Ok(quote! {
        #prefix_claim
        writer.suppress_next_space();
        #value_render
        writer.suppress_next_space();
        #suffix_claim
    })
}

fn render_fixed_surface_statement(surface: &str) -> TokenStream {
    if surface.chars().count() == 1
        && surface
            .chars()
            .all(|character| character.is_ascii_punctuation())
    {
        let punctuation = surface
            .chars()
            .next()
            .expect("one-character fixed surface is present");
        quote! { writer.punctuation(#punctuation); }
    } else {
        let surface = syn::LitStr::new(surface, Span::call_site());
        quote! { writer.word(#surface); }
    }
}

fn render_atom_statement(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    atom: &AtomPlan,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<TokenStream> {
    let call_writer = quote! { writer };
    let method_writer = quote! { writer };
    match atom {
        AtomPlan::Literal(value) => Ok(render_fixed_surface_statement(value)),
        AtomPlan::SentenceInitialLiteral(value) => {
            let literal = syn::LitStr::new(value, Span::call_site());
            Ok(quote! {
                #method_writer.structural_surface(
                    #literal,
                    StructuralTransition::SentenceInitial,
                );
            })
        }
        AtomPlan::Category { role, .. } => {
            let field = fields
                .get(role)
                .ok_or_else(|| internal("resolved role is absent"))?;
            if field.kind() != ConstructionFieldKind::Category {
                return Err(internal("bare role is not a category"));
            }
            let category = field.terminal();
            let helper = render_category_name(category, root_names.contains(category));
            let value = field_value(construction, role, locals)?;
            let capability = validated.category_render_capability(category);
            let agreement = if capability.requires_external_agreement() {
                role_agreement(validated, construction, role, locals)?
            } else {
                None
            };
            let context = capability.requires_context().then(|| quote! { context });
            let environment = validated
                .needs_parser_environment()
                .then(|| quote! { environment });
            let tail = signature_tail(&[agreement, context, environment]);
            Ok(quote! { #helper(#call_writer, #value #tail); })
        }
        AtomPlan::Lex { role, .. } => {
            let field = fields
                .get(role)
                .ok_or_else(|| internal("resolved lexical role is absent"))?;
            let terminal = field.terminal();
            let value = field_value(construction, role, locals)?;
            if let Some(vocab) = find_vocab(validated, terminal) {
                let function = ident(&format!("render_{}", snake_case(vocab.name())));
                let value = copy_value(construction, role, value)?;
                Ok(quote! { #function(#call_writer, #value); })
            } else if let Some(codec) = find_signed_decimal(validated, terminal) {
                let function = ident(&format!("render_{}", snake_case(codec.codec_name())));
                Ok(quote! { #function(#call_writer, #value); })
            } else if let Some(codec) = find_unsigned_number(validated, terminal) {
                let function = ident(&format!("render_{}", snake_case(codec.codec_name())));
                Ok(quote! { #function(#call_writer, #value); })
            } else {
                let binding = find_binding(validated, terminal)?;
                let Some(BindingRenderPlan::Runtime(function)) = binding.render() else {
                    return Err(internal("lex terminal lacks runtime render binding"));
                };
                let value = match binding.traversal().mode() {
                    VisitMode::Copy => copy_value(construction, role, value)?,
                    VisitMode::Borrowed => value,
                };
                Ok(quote! { #function(#call_writer, #value); })
            }
        }
        AtomPlan::Identity { role, .. } => {
            let field = fields
                .get(role)
                .ok_or_else(|| internal("resolved identity role is absent"))?;
            let value = field_value(construction, role, locals)?;
            if find_context_identity(validated, field.terminal()).is_some() {
                let value = copy_value(construction, role, value)?;
                Ok(quote! { #method_writer.identity((#value).surface(context)); })
            } else if find_catalog_identity(validated, field.terminal()).is_some() {
                Ok(quote! {
                    #method_writer.identity(
                        environment
                            .catalog_surface((#value).provider(), (#value).canonical_identity())
                            .expect("validated catalog identity remains in its frozen provider"),
                    );
                })
            } else {
                let binding = find_binding(validated, field.terminal())?;
                match binding
                    .render()
                    .ok_or_else(|| internal("identity lacks render metadata"))?
                {
                    BindingRenderPlan::Runtime(function) => {
                        let value = match binding.traversal().mode() {
                            VisitMode::Copy => copy_value(construction, role, value)?,
                            VisitMode::Borrowed => value,
                        };
                        Ok(quote! { #function(#call_writer, #value, context); })
                    }
                    BindingRenderPlan::ContextIdentity(arms) => {
                        let ty = binding.value_type_name();
                        let match_arms = arms.iter().map(|arm| {
                            let variant = arm.variant();
                            let accessor = arm.accessor();
                            crate::emit::call_match_arm(
                                &quote! { #ty::#variant },
                                &quote! { #method_writer.identity(context.#accessor()) },
                                12,
                            )
                        });
                        let value = match binding.traversal().mode() {
                            VisitMode::Copy => copy_value(construction, role, value)?,
                            VisitMode::Borrowed => value,
                        };
                        Ok(quote! { match #value { #(#match_arms),* } })
                    }
                }
            }
        }
        AtomPlan::VerbFixed {
            terminal,
            path: variant,
            ..
        } => render_fixed_verb_atom(validated, construction, locals, terminal, variant),
        AtomPlan::OpenDeclaration(open) => {
            render_open_declaration(validated, construction, open, locals, &method_writer)
        }
        AtomPlan::Noun { role, .. } => {
            let field = fields
                .get(role)
                .ok_or_else(|| internal("resolved noun role is absent"))?;
            render_noun_atom(
                validated,
                construction,
                field,
                role,
                locals,
                &method_writer,
                &call_writer,
            )
        }
        AtomPlan::Bound { .. } => Err(internal("bound atoms must be rendered as two claims")),
        AtomPlan::Circumfix { .. } => {
            Err(internal("circumfix atoms must be rendered as three claims"))
        }
    }
}

fn render_construction_structural_field(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    field: &crate::semantic::StructuralFieldPlan,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
) -> syn::Result<TokenStream> {
    let value = field_value(construction, role, locals)?;
    match field.kind() {
        StructuralFieldKindPlan::Required(kind) => {
            if matches!(kind, ValueKindPlan::Sum(sum) if plan.sum_carries_agreement(sum)) {
                let agreement =
                    role_agreement(plan, construction, role, locals)?.ok_or_else(|| {
                        internal("agreement-bearing sum role lacks an agreement writer")
                    })?;
                render_structural_value_with_feature(
                    plan,
                    kind,
                    &value,
                    root_names,
                    Feature::Agreement,
                    agreement,
                )
            } else {
                render_structural_value(plan, kind, value, root_names)
            }
        }
        StructuralFieldKindPlan::Optional(kind) => {
            let render = render_structural_value(plan, kind, quote! { value }, root_names)?;
            Ok(quote! { if let Some(value) = #value { #render } })
        }
        StructuralFieldKindPlan::Sequence { item, .. } => {
            let function = ident(&structural_sequence_renderer(
                construction.element_type(),
                field.name(),
            ));
            let context =
                structural_value_requires_context(plan, item)?.then(|| quote! { , context });
            let environment = plan
                .needs_parser_environment()
                .then(|| quote! { , environment });
            let feature = plan
                .sequence_feature(construction.element_type(), field.name())
                .map(|feature| {
                    sequence_role_feature_value(plan, construction, role, locals, item, feature)
                        .map(|value| quote! { , #value })
                })
                .transpose()?;
            Ok(quote! { #function(writer, #value #feature #context #environment); })
        }
    }
}

fn sequence_role_feature_value(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    locals: &RenderLocals,
    item: &ValueKindPlan,
    feature: Feature,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new(role, construction.origin_span()),
        feature,
    };
    if let Some(writer) = plan
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == &target)
    {
        return feature_expr(plan, construction, writer.value(), feature, locals);
    }
    if feature == Feature::Agreement
        && !locals.category.is_empty()
        && plan.category_requires_external_agreement(construction.category())
        && plan
            .feature_equations(construction.construction_id())
            .iter()
            .any(|equation| {
                matches!(
                    (equation.target(), equation.value()),
                    (
                        FeaturePlace::Construction(Feature::Agreement),
                        FeatureExpr::FromRole {
                            role: source,
                            feature: Feature::Agreement,
                        },
                    ) if identifier_key(source) == role
                )
            })
    {
        return Ok(quote! { agreement });
    }
    let helper_owner = match item {
        ValueKindPlan::Category(category) => category,
        ValueKindPlan::Sum(sum) if plan.sum_carries_agreement(sum) => sum,
        ValueKindPlan::Sum(_)
        | ValueKindPlan::Product(_)
        | ValueKindPlan::Lex(_)
        | ValueKindPlan::Identity(_) => {
            return Err(internal("sequence feature item does not carry agreement"));
        }
    };
    let values = field_value(construction, role, locals)?;
    let helper = ident(&feature_helper(feature_name(feature), helper_owner));
    Ok(quote! {
        #helper(
            #values
                .first()
                .expect("validated sequence feature source is statically nonempty")
        )
    })
}

fn render_atom_role(atom: &AtomPlan) -> Option<&str> {
    match atom.value_atom() {
        AtomPlan::Category { role, .. }
        | AtomPlan::Lex { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => Some(role),
        AtomPlan::Literal(_)
        | AtomPlan::SentenceInitialLiteral(_)
        | AtomPlan::VerbFixed { .. }
        | AtomPlan::OpenDeclaration(_) => None,
        AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
            unreachable!("value_atom removes form wrappers")
        }
    }
}

fn render_fixed_verb_atom(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    locals: &RenderLocals,
    terminal: &str,
    variant: &syn::Path,
) -> syn::Result<TokenStream> {
    let method_writer = quote! { writer };
    let agreement = verb_agreement(validated, construction, locals)?;
    let lexeme = find_lexeme(validated, terminal)
        .ok_or_else(|| internal("fixed verb terminal lacks its lexeme plan"))?;
    let surface = ident(&lexeme_surface_helper(lexeme.name()));
    Ok(quote! { #method_writer.word(#surface(#variant, #agreement)); })
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive generated owner lowering keeps every atom family visibly aligned"
)]
fn render_owner(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form_name: &str,
    atom_index: usize,
    atom: &AtomPlan,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let atom = atom.value_atom();
    match atom {
        AtomPlan::Literal(_) | AtomPlan::SentenceInitialLiteral(_) => {
            let stable_id = syn::LitStr::new(
                &format!(
                    "form:{}/{}/{}",
                    construction.construction_id(),
                    form_name,
                    atom_index,
                ),
                Span::call_site(),
            );
            Ok(quote! {
                LexicalOwner::static_owner(
                    LexicalProvenanceKind::FormLiteral,
                    #stable_id,
                )
            })
        }
        AtomPlan::Category { .. } => Err(internal("category atoms do not own lexical spans")),
        AtomPlan::Lex { role, .. } => {
            let field = construction.field(role)?;
            let terminal = field.terminal();
            let value = field_value(construction, role, locals)?;
            if let Some(vocab) = find_vocab(validated, terminal) {
                let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
                let value = copy_value(construction, role, value)?;
                let arms = vocab.variants().iter().map(|variant| {
                    let member =
                        emitted_ident(&identifier_key(variant.name()), variant.name().span());
                    let stable_id = syn::LitStr::new(
                        &format!("vocab:{}/{}", vocab.name(), member),
                        Span::call_site(),
                    );
                    quote! {
                        #ty::#member => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Vocab,
                            #stable_id,
                        )
                    }
                });
                return Ok(quote! { match #value { #(#arms,)* } });
            }
            let (kind, prefix) = if find_signed_decimal(validated, terminal).is_some()
                || find_unsigned_number(validated, terminal).is_some()
            {
                (quote! { LexicalProvenanceKind::Codec }, "codec")
            } else {
                let binding = find_binding(validated, terminal)?;
                match binding.kind() {
                    crate::model::TerminalBindingKind::Codec => {
                        (quote! { LexicalProvenanceKind::Codec }, "codec")
                    }
                    crate::model::TerminalBindingKind::Identity => {
                        (quote! { LexicalProvenanceKind::Identity }, "identity")
                    }
                }
            };
            let stable_id = syn::LitStr::new(&format!("{prefix}:{terminal}"), Span::call_site());
            Ok(quote! { LexicalOwner::static_owner(#kind, #stable_id) })
        }
        AtomPlan::Identity { role, .. } => {
            let field = construction.field(role)?;
            let terminal = field.terminal();
            let value = field_value(construction, role, locals)?;
            if let Some(identity) = find_context_identity(validated, terminal) {
                let ty = identity.ident();
                let value = copy_value(construction, role, value)?;
                let arms = identity.arms().iter().map(|arm| {
                    let member = arm.variant();
                    let stable_id = syn::LitStr::new(
                        &format!("identity:{terminal}/{member}"),
                        Span::call_site(),
                    );
                    quote! {
                        #ty::#member => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Identity,
                            #stable_id,
                        )
                    }
                });
                return Ok(quote! { match #value { #(#arms,)* } });
            }
            if find_catalog_identity(validated, terminal).is_some() {
                return Ok(quote! {
                    LexicalOwner::catalog_owner(
                        (#value).provider(),
                        (#value).canonical_identity.clone(),
                    )
                });
            }
            let stable_id = syn::LitStr::new(&format!("identity:{terminal}"), Span::call_site());
            Ok(quote! {
                LexicalOwner::static_owner(LexicalProvenanceKind::Identity, #stable_id)
            })
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let agreement = verb_agreement(validated, construction, locals)?;
            let lexeme = find_lexeme(validated, terminal)
                .ok_or_else(|| internal("fixed verb terminal lacks its lexeme plan"))?;
            let arms = lexeme
                .surfaces()
                .iter()
                .filter(|row| row.member() == variant)
                .map(|row| {
                    let feature = match row.feature() {
                        macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                        macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                            quote! { Agreement::ThirdPersonSingular }
                        }
                        macro_ron::v2::SurfaceFeature::Singular
                        | macro_ron::v2::SurfaceFeature::Plural
                        | macro_ron::v2::SurfaceFeature::Fixed => {
                            unreachable!("validated verb lexeme has the Agreement feature axis")
                        }
                    };
                    let stable_id =
                        crate::emit::closed_lexeme_owner_id(terminal, variant, row.feature());
                    quote! {
                        #feature => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Lexeme,
                            #stable_id,
                        )
                    }
                });
            Ok(quote! { match #agreement { #(#arms,)* } })
        }
        AtomPlan::OpenDeclaration(open) => {
            let agreement = verb_agreement(validated, construction, locals)?;
            let feature = quote! {
                match #agreement {
                    Agreement::Bare => ::macro_ron::v2::SurfaceFeature::Bare,
                    Agreement::ThirdPersonSingular => {
                        ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                    }
                }
            };
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            Ok(quote! {{
                let id = ::macro_ron::v2::DeclarationIdentity::new(#kind, #name);
                LexicalOwner::declaration_owner(id, #feature)
            }})
        }
        AtomPlan::Noun { role, .. } => {
            let field = construction.field(role)?;
            let value = field_value(construction, role, locals)?;
            let number = noun_role_number(validated, construction, role, locals)?;
            if let Some(lexeme) = find_lexeme(validated, field.terminal()) {
                let noun = lexeme.name_ident();
                let arms = lexeme.surfaces().iter().map(|row| {
                    let member = emitted_ident(row.member(), Span::call_site());
                    let feature = match row.feature() {
                        macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                        macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                        macro_ron::v2::SurfaceFeature::Bare
                        | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                        | macro_ron::v2::SurfaceFeature::Fixed => {
                            unreachable!("validated noun lexeme has the Number feature axis")
                        }
                    };
                    let stable_id = crate::emit::closed_lexeme_owner_id(
                        lexeme.name(),
                        row.member(),
                        row.feature(),
                    );
                    quote! {
                        (#noun::#member, #feature) => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Lexeme,
                            #stable_id,
                        )
                    }
                });
                return Ok(quote! { match (#value, #number) { #(#arms,)* } });
            }
            let Some((_, codec)) = validated.runtime_declaration_noun_for(field.terminal()) else {
                let stable_id =
                    syn::LitStr::new(&format!("codec:{}", field.terminal()), Span::call_site());
                return Ok(quote! {
                    LexicalOwner::static_owner(LexicalProvenanceKind::Codec, #stable_id)
                });
            };
            let noun = codec.codec_ident();
            let closed_arms = if let Some(closed) = codec.closed_lexeme() {
                validated
                    .runtime_noun_lexeme()
                    .expect("validated declaration noun has a closed lexeme")
                    .surfaces()
                    .iter()
                    .map(|row| {
                    let member = emitted_ident(row.member(), Span::call_site());
                    let feature = match row.feature() {
                        macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                        macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                        macro_ron::v2::SurfaceFeature::Bare
                        | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                        | macro_ron::v2::SurfaceFeature::Fixed => {
                            unreachable!("validated noun lexeme has the Number feature axis")
                        }
                    };
                    let stable_id = crate::emit::closed_lexeme_owner_id(
                        &closed.to_string(),
                        row.member(),
                        row.feature(),
                    );
                    quote! {
                        (#noun::Lexeme(#closed::#member), #feature) => LexicalOwner::static_owner(
                            LexicalProvenanceKind::Lexeme,
                            #stable_id,
                        )
                    }
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            Ok(quote! {
                match (#value, #number) {
                    #(#closed_arms,)*
                    (#noun::Declaration(declaration), _) => LexicalOwner::declaration_owner(
                        declaration.id().clone(),
                        match #number {
                            Number::Singular => ::macro_ron::v2::SurfaceFeature::Singular,
                            Number::Plural => ::macro_ron::v2::SurfaceFeature::Plural,
                        },
                    ),
                }
            })
        }
        AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
            unreachable!("value_atom removes form wrappers")
        }
    }
}

fn render_noun_atom(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    field: &ConstructionFieldPlan,
    role: &str,
    locals: &RenderLocals,
    method_writer: &TokenStream,
    call_writer: &TokenStream,
) -> syn::Result<TokenStream> {
    let number = noun_role_number(validated, construction, role, locals)?;
    let value = field_value(construction, role, locals)?;
    if let Some(lexeme) = find_lexeme(validated, field.terminal()) {
        let surface = ident(&lexeme_surface_helper(lexeme.name()));
        return Ok(quote! { #method_writer.word(#surface(*#value, #number)); });
    }
    let Some((_, codec)) = validated.runtime_declaration_noun_for(field.terminal()) else {
        let binding = find_binding(validated, field.terminal())?;
        let Some(BindingRenderPlan::Runtime(function)) = binding.render() else {
            return Err(internal("noun terminal lacks runtime render binding"));
        };
        return Ok(quote! { #function(#call_writer, #value, #number); });
    };

    let noun = codec.codec_ident();
    let closed_arm = if let Some(closed) = codec.closed_lexeme() {
        let lexeme = validated
            .runtime_noun_lexeme()
            .ok_or_else(|| internal("validated declaration noun lacks a closed lexeme plan"))?;
        if closed != lexeme.name() {
            return Err(internal(
                "declaration noun closed lexeme plan is inconsistent",
            ));
        }
        let surface = ident(&lexeme_surface_helper(lexeme.name()));
        Some(quote! {
            #noun::Lexeme(lexeme) => {
                #method_writer.word(#surface(*lexeme, #number));
            }
        })
    } else {
        None
    };
    Ok(quote! {
        match #value {
            #closed_arm
            #noun::Declaration(declaration) => {
                let surface = environment
                    .surface(
                        declaration.id(),
                        match #number {
                            Number::Singular => ::macro_ron::v2::SurfaceFeature::Singular,
                            Number::Plural => ::macro_ron::v2::SurfaceFeature::Plural,
                        },
                    )
                    .expect("stored declaration noun remains in its parser environment");
                #method_writer.word(surface);
            }
        }
    })
}

fn noun_role_number(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new(role, construction.origin_span()),
        feature: Feature::Number,
    };
    if let Some(equation) = validated
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == &target)
    {
        return feature_expr(
            validated,
            construction,
            equation.value(),
            Feature::Number,
            locals,
        );
    }
    let number = ident(&feature_helper("number", construction.category()));
    let category_value = &locals.category;
    Ok(quote! { #number(#category_value) })
}

fn render_open_declaration(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    open: &crate::semantic::OpenDeclarationAtomPlan,
    locals: &RenderLocals,
    method_writer: &TokenStream,
) -> syn::Result<TokenStream> {
    let agreement = verb_agreement(validated, construction, locals)?;
    let feature = quote! {
        match #agreement {
            Agreement::Bare => ::macro_ron::v2::SurfaceFeature::Bare,
            Agreement::ThirdPersonSingular => {
                ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular
            }
        }
    };
    let kind = crate::emit::declaration_kind(open.kind());
    let name = syn::LitStr::new(open.name(), Span::call_site());
    Ok(quote! {
        #method_writer.word(
            environment
                .surface(
                    &::macro_ron::v2::DeclarationIdentity::new(#kind, #name),
                    #feature,
                )
                .expect("parser validated the required declaration surface"),
        );
    })
}

fn role_agreement(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    locals: &RenderLocals,
) -> syn::Result<Option<TokenStream>> {
    let equation = validated.feature_equations(construction.construction_id()).iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == role)
    });
    equation
        .map(|equation| {
            feature_expr(
                validated,
                construction,
                equation.value(),
                Feature::Agreement,
                locals,
            )
        })
        .transpose()
}

fn verb_agreement(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let equations = validated.feature_equations(construction.construction_id());
    if let Some(equation) = equations.iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == "verb")
    }) {
        return feature_expr(
            validated,
            construction,
            equation.value(),
            Feature::Agreement,
            locals,
        );
    }
    if equations.iter().any(|equation| {
        matches!(equation.target(), FeaturePlace::Construction(Feature::Agreement))
            && matches!(equation.value(), FeatureExpr::FromRole { role, feature: Feature::Agreement } if identifier_key(role) == "verb")
    }) {
        return Ok(quote! { agreement });
    }
    Err(internal("verb atom lacks validated agreement flow"))
}

#[allow(
    clippy::too_many_lines,
    reason = "the exhaustive sealed feature-expression lowering matrix is kept together"
)]
fn feature_expr(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    expression: &FeatureExpr,
    _feature: Feature,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    Ok(match expression {
        FeatureExpr::Constant(value) => feature_value(*value.value()),
        FeatureExpr::FromRole {
            role,
            feature: source_feature,
        } => {
            if !construction
                .fields()
                .iter()
                .any(|field| field.name_key() == identifier_key(role))
            {
                if let Some(writer) = validated
                    .feature_equations(construction.construction_id())
                    .iter()
                    .find(|equation| {
                        matches!(
                            equation.target(),
                            FeaturePlace::Role { field, feature }
                                if identifier_key(field) == identifier_key(role) && feature == source_feature
                        )
                    })
                {
                    return feature_expr(
                        validated,
                        construction,
                        writer.value(),
                        *source_feature,
                        locals,
                    );
                }
                return match source_feature {
                    Feature::Agreement => Ok(quote! { agreement }),
                    Feature::Cardinality => Err(internal("verb slot does not provide cardinality")),
                    Feature::Onset => implicit_verb_onset(validated, construction, locals),
                    Feature::Number => Err(internal("verb slot does not provide number")),
                    Feature::PossessiveEnding => {
                        Err(internal("verb slot does not provide possessive ending"))
                    }
                };
            }
            let role_key = identifier_key(role);
            let field = construction.field(&role_key)?;
            if let Some(writer) = validated
                    .feature_equations(construction.construction_id())
                    .iter()
                    .find(|equation| {
                        matches!(
                            equation.target(),
                            FeaturePlace::Role { field, feature }
                                if identifier_key(field) == identifier_key(role) && feature == source_feature
                        )
                    })
                && (field.kind() == ConstructionFieldKind::Category
                    || !matches!(writer.value(), FeatureExpr::MatchVocab { .. }))
            {
                return feature_expr(
                    validated,
                    construction,
                    writer.value(),
                    *source_feature,
                    locals,
                );
            }
            let role_value = field_value(construction, &role_key, locals)?;
            if let Some(structural) = field.structural_plan()
                && let StructuralFieldKindPlan::Sequence { item, .. } = structural.kind()
                && validated.sequence_feature(construction.element_type(), &role_key)
                    == Some(*source_feature)
            {
                let helper_owner = match item {
                    ValueKindPlan::Category(category) => category,
                    ValueKindPlan::Sum(sum) if validated.sum_carries_agreement(sum) => sum,
                    ValueKindPlan::Sum(_)
                    | ValueKindPlan::Product(_)
                    | ValueKindPlan::Lex(_)
                    | ValueKindPlan::Identity(_) => {
                        return Err(internal(
                            "sequence feature source item does not carry agreement",
                        ));
                    }
                };
                let helper = ident(&feature_helper(feature_name(*source_feature), helper_owner));
                return Ok(quote! {
                    #helper(
                        #role_value
                            .first()
                            .expect("validated sequence feature source is statically nonempty")
                    )
                });
            }
            if matches!(*source_feature, Feature::Cardinality | Feature::Number)
                && field.kind() == ConstructionFieldKind::Lex
                && let Some(codec) = find_unsigned_number(validated, field.terminal())
                && codec.kind() == UnsignedNumberKind::EnglishCardinal
            {
                let function = ident(&feature_helper(
                    feature_name(*source_feature),
                    codec.codec_name(),
                ));
                return Ok(quote! { #function(#role_value) });
            }
            if *source_feature == Feature::Onset {
                let payload_onset = if field.kind() == ConstructionFieldKind::Category {
                    let function = ident(&feature_helper("onset", field.terminal()));
                    let environment = validated
                        .needs_parser_environment()
                        .then(|| quote! { , environment });
                    quote! { #function(#role_value, context #environment) }
                } else {
                    lexical_onset_expr(
                        validated,
                        construction,
                        &role_key,
                        field,
                        role_value,
                        locals,
                    )?
                };
                return bound_prefix_onset(
                    validated,
                    construction,
                    &role_key,
                    locals,
                    &payload_onset,
                )
                .map(|realized| realized.unwrap_or(payload_onset));
            }
            if *source_feature == Feature::PossessiveEnding
                && field.kind() != ConstructionFieldKind::Category
            {
                return lexical_possessive_ending_expr(
                    validated,
                    construction,
                    &role_key,
                    field,
                    role_value,
                    locals,
                );
            }
            let (source, value) = match field.kind() {
                ConstructionFieldKind::Category => (field.terminal().to_owned(), role_value),
                ConstructionFieldKind::Lex => {
                    let (writer_role, vocabulary) = canonical_lexical_feature_lowering(
                        validated,
                        construction,
                        role,
                        *source_feature,
                    )?;
                    let writer_value =
                        field_value(construction, &identifier_key(writer_role), locals)?;
                    (
                        vocabulary.to_owned(),
                        copy_value(construction, &identifier_key(writer_role), writer_value)?,
                    )
                }
                ConstructionFieldKind::Identity => {
                    return Err(internal(
                        "identity roles cannot provide grammatical features",
                    ));
                }
            };
            let function = ident(&feature_helper(feature_name(*source_feature), &source));
            if matches!(*source_feature, Feature::Onset | Feature::PossessiveEnding) {
                let environment = validated
                    .needs_parser_environment()
                    .then(|| quote! { , environment });
                quote! { #function(#value, context #environment) }
            } else {
                quote! { #function(#value) }
            }
        }
        FeatureExpr::MatchVocab { role, arms } => {
            let role_key = identifier_key(role);
            let field = construction.field(&role_key)?;
            let ty = ident(field.terminal());
            let match_arms = arms.iter().map(|(variant, value)| {
                let variant = variant.value();
                let value = feature_value(*value);
                quote! { #ty::#variant => #value }
            });
            let role_value = field_value(construction, &role_key, locals)?;
            let role_value = copy_value(construction, &role_key, role_value)?;
            quote! { match #role_value { #(#match_arms),* } }
        }
    })
}

fn implicit_verb_onset(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let atom = construction
        .forms()
        .iter()
        .flat_map(FormPlan::atoms)
        .find(|atom| {
            matches!(
                atom,
                AtomPlan::VerbFixed { .. } | AtomPlan::OpenDeclaration(_)
            )
        })
        .ok_or_else(|| internal("validated verb onset source has no verb atom"))?;
    let agreement = verb_agreement(validated, construction, locals)?;
    match atom {
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            if !validated.terminal_provides_onset(terminal) {
                return Err(internal("validated fixed verb lacks onset capability"));
            }
            let lexeme = find_lexeme(validated, terminal)
                .ok_or_else(|| internal("fixed verb onset lacks its sealed lexeme"))?;
            let arms = lexeme
                .surfaces()
                .iter()
                .filter(|row| row.member() == variant)
                .map(|row| {
                    let agreement = match row.feature() {
                        macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                        macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                            quote! { Agreement::ThirdPersonSingular }
                        }
                        macro_ron::v2::SurfaceFeature::Singular
                        | macro_ron::v2::SurfaceFeature::Plural
                        | macro_ron::v2::SurfaceFeature::Fixed => {
                            unreachable!("validated verb lexeme has the Agreement feature axis")
                        }
                    };
                    let onset = super::onset(row.onset());
                    quote! { #agreement => #onset }
                });
            Ok(quote! { match #agreement { #(#arms,)* } })
        }
        AtomPlan::OpenDeclaration(open) => {
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            Ok(quote! {
                environment
                    .onset(
                        &::macro_ron::v2::DeclarationIdentity::new(#kind, #name),
                        match #agreement {
                            Agreement::Bare => ::macro_ron::v2::SurfaceFeature::Bare,
                            Agreement::ThirdPersonSingular => {
                                ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                            }
                        },
                    )
                    .expect("required open verb remains in its normalized parser environment")
            })
        }
        AtomPlan::Literal(_)
        | AtomPlan::SentenceInitialLiteral(_)
        | AtomPlan::Category { .. }
        | AtomPlan::Lex { .. }
        | AtomPlan::Identity { .. }
        | AtomPlan::Noun { .. }
        | AtomPlan::Bound { .. }
        | AtomPlan::Circumfix { .. } => unreachable!("verb onset selected a verb atom"),
    }
}

fn bound_prefix_onset(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    locals: &RenderLocals,
    payload_onset: &TokenStream,
) -> syn::Result<Option<TokenStream>> {
    let mut has_bound_prefix = false;
    let mut form_onsets = Vec::new();
    for form in construction.forms() {
        let affix = form.atoms().iter().find_map(|atom| {
            let AtomPlan::Bound {
                direction: crate::semantic::BoundDirectionPlan::Prefix,
                affix,
                value,
            } = atom
            else {
                return None;
            };
            matches!(
                value.as_ref(),
                AtomPlan::Category { role: found, .. }
                    | AtomPlan::Lex { role: found, .. }
                    | AtomPlan::Identity { role: found, .. }
                    | AtomPlan::Noun { role: found, .. }
                    if found == role
            )
            .then_some(affix)
        });
        has_bound_prefix |= affix.is_some();
        form_onsets.push(affix.and_then(|surface| {
            ::macro_ron::v2::normalize_surface_onset(surface, None).map(super::onset)
        }));
    }
    if !has_bound_prefix {
        return Ok(None);
    }
    if form_onsets.len() == 1 {
        return Ok(Some(
            form_onsets
                .pop()
                .flatten()
                .unwrap_or_else(|| payload_onset.clone()),
        ));
    }
    let branches = form_onsets
        .into_iter()
        .enumerate()
        .map(|(form_index, onset)| {
            let guard =
                super::emit_form_guard_expression(construction, form_index, |domain, value| {
                    render_guard_atom(validated, construction, locals, domain, value)
                })?
                .ok_or_else(|| internal("multi-form bound-prefix onset form has no guard"))?;
            let onset = onset.unwrap_or_else(|| payload_onset.clone());
            Ok(quote! { if #guard { #onset } else })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(Some(quote! {
        #(#branches)* { unreachable!("sealed form partition is total") }
    }))
}

fn lexical_onset_expr(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    field: &crate::semantic::ConstructionFieldPlan,
    role_value: TokenStream,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    if let Some(vocab) = find_vocab(validated, field.terminal()) {
        let ty = ident(vocab.name());
        let value = copy_value(construction, role, role_value)?;
        let arms = vocab.variants().iter().map(|variant| {
            let member = variant.name();
            let onset = super::onset(variant.onset());
            quote! { #ty::#member => #onset }
        });
        return Ok(quote! { match #value { #(#arms,)* } });
    }
    if let Some(identity) = find_context_identity(validated, field.terminal()) {
        let ty = identity.ident();
        let value = copy_value(construction, role, role_value)?;
        let arms = identity.arms().iter().map(|arm| {
            let member = arm.variant();
            let accessor = ident(&format!("{}_onset", identifier_key(arm.accessor())));
            quote! { #ty::#member => context.#accessor() }
        });
        return Ok(quote! { match #value { #(#arms,)* } });
    }
    if find_catalog_identity(validated, field.terminal()).is_some() {
        return Ok(quote! {
            environment
                .catalog_onset((#role_value).provider(), (#role_value).canonical_identity())
                .expect("validated catalog identity remains in its frozen provider")
        });
    }
    if let Some(lexeme) = find_lexeme(validated, field.terminal()) {
        let noun = lexeme.name_ident();
        let number = noun_role_number(validated, construction, role, locals)?;
        let arms = lexeme.surfaces().iter().map(|row| {
            let member = ident(row.member());
            let feature = match row.feature() {
                macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                macro_ron::v2::SurfaceFeature::Bare
                | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                | macro_ron::v2::SurfaceFeature::Fixed => {
                    unreachable!("validated noun lexeme has the Number feature axis")
                }
            };
            let onset = super::onset(row.onset());
            quote! { (#noun::#member, #feature) => #onset }
        });
        return Ok(quote! { match (#role_value, #number) { #(#arms,)* } });
    }
    let Some((_, codec)) = validated.runtime_declaration_noun_for(field.terminal()) else {
        return Err(internal(
            "lexical onset source has no sealed terminal onset plan",
        ));
    };
    let noun = codec.codec_ident();
    let number = noun_role_number(validated, construction, role, locals)?;
    let closed_arms = if let Some(closed) = codec.closed_lexeme() {
        validated
            .runtime_noun_lexeme()
            .expect("validated declaration noun has a closed lexeme")
            .surfaces()
            .iter()
            .map(|row| {
                let member = ident(row.member());
                let number = match row.feature() {
                    macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                    macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                    macro_ron::v2::SurfaceFeature::Bare
                    | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                    | macro_ron::v2::SurfaceFeature::Fixed => {
                        unreachable!("validated noun lexeme has the Number feature axis")
                    }
                };
                let onset = super::onset(row.onset());
                quote! { (#noun::Lexeme(#closed::#member), #number) => #onset }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    Ok(quote! {
        match (#role_value, #number) {
            #(#closed_arms,)*
            (#noun::Declaration(declaration), number) => environment
                .onset(
                    declaration.id(),
                    match number {
                        Number::Singular => ::macro_ron::v2::SurfaceFeature::Singular,
                        Number::Plural => ::macro_ron::v2::SurfaceFeature::Plural,
                    },
                )
                .expect("stored declaration noun remains in its normalized parser environment"),
        }
    })
}

fn lexical_possessive_ending_expr(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    role: &str,
    field: &crate::semantic::ConstructionFieldPlan,
    role_value: TokenStream,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    if let Some(vocab) = find_vocab(validated, field.terminal()) {
        let ty = ident(vocab.name());
        let value = copy_value(construction, role, role_value)?;
        let arms = vocab.variants().iter().map(|variant| {
            let member = variant.name();
            let ending = possessive_ending_for_surface(&variant.word().value());
            quote! { #ty::#member => #ending }
        });
        return Ok(quote! { match #value { #(#arms,)* } });
    }
    if let Some(identity) = find_context_identity(validated, field.terminal()) {
        let ty = identity.ident();
        let value = copy_value(construction, role, role_value)?;
        let arms = identity.arms().iter().map(|arm| {
            let member = arm.variant();
            let accessor = arm.accessor();
            let ending = runtime_possessive_ending(&quote! { context.#accessor() });
            quote! { #ty::#member => #ending }
        });
        return Ok(quote! { match #value { #(#arms,)* } });
    }
    if find_catalog_identity(validated, field.terminal()).is_some() {
        return Ok(runtime_possessive_ending(&quote! {
            environment
                .catalog_surface((#role_value).provider(), (#role_value).canonical_identity())
                .expect("validated catalog identity remains in its frozen provider")
        }));
    }
    if let Some(lexeme) = find_lexeme(validated, field.terminal()) {
        let noun = lexeme.name_ident();
        let number = noun_role_number(validated, construction, role, locals)?;
        let arms = lexeme.surfaces().iter().map(|row| {
            let member = ident(row.member());
            let feature = match row.feature() {
                macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                macro_ron::v2::SurfaceFeature::Bare
                | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                | macro_ron::v2::SurfaceFeature::Fixed => {
                    unreachable!("validated noun lexeme has the Number feature axis")
                }
            };
            let ending = possessive_ending_for_surface(row.surface());
            quote! { (#noun::#member, #feature) => #ending }
        });
        return Ok(quote! { match (#role_value, #number) { #(#arms,)* } });
    }
    let Some((_, codec)) = validated.runtime_declaration_noun_for(field.terminal()) else {
        return Err(internal(
            "possessive-ending source has no sealed lexical surface plan",
        ));
    };
    let noun = codec.codec_ident();
    let number = noun_role_number(validated, construction, role, locals)?;
    let closed_arms = if let Some(closed) = codec.closed_lexeme() {
        validated
            .runtime_noun_lexeme()
            .expect("validated declaration noun has a closed lexeme")
            .surfaces()
            .iter()
            .map(|row| {
                let member = ident(row.member());
                let number = match row.feature() {
                    macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                    macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                    macro_ron::v2::SurfaceFeature::Bare
                    | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                    | macro_ron::v2::SurfaceFeature::Fixed => {
                        unreachable!("validated noun lexeme has the Number feature axis")
                    }
                };
                let ending = possessive_ending_for_surface(row.surface());
                quote! { (#noun::Lexeme(#closed::#member), #number) => #ending }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let dynamic = runtime_possessive_ending(&quote! {
        environment
            .surface(
                declaration.id(),
                match number {
                    Number::Singular => ::macro_ron::v2::SurfaceFeature::Singular,
                    Number::Plural => ::macro_ron::v2::SurfaceFeature::Plural,
                },
            )
            .expect("stored declaration noun remains in its normalized parser environment")
    });
    Ok(quote! {
        match (#role_value, #number) {
            #(#closed_arms,)*
            (#noun::Declaration(declaration), number) => #dynamic,
        }
    })
}

fn possessive_ending_for_surface(surface: &str) -> TokenStream {
    if surface
        .as_bytes()
        .last()
        .is_some_and(|byte| matches!(byte, b's' | b'S'))
    {
        quote! { PossessiveEnding::EndsInS }
    } else {
        quote! { PossessiveEnding::Other }
    }
}

fn runtime_possessive_ending(surface: &TokenStream) -> TokenStream {
    quote! {
        if (#surface)
            .as_bytes()
            .last()
            .is_some_and(|byte| matches!(byte, b's' | b'S'))
        {
            PossessiveEnding::EndsInS
        } else {
            PossessiveEnding::Other
        }
    }
}

fn canonical_lexical_feature_lowering<'a>(
    validated: &'a SemanticPlan,
    construction: &'a ConstructionPlan,
    role: &syn::Ident,
    feature: Feature,
) -> syn::Result<(&'a syn::Ident, &'a str)> {
    let writer = validated
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| {
            matches!(
                equation.target(),
                FeaturePlace::Role {
                    field,
                    feature: writer_feature,
                } if identifier_key(field) == identifier_key(role) && *writer_feature == feature
            )
        })
        .ok_or_else(|| {
            internal(&format!(
                "lexical feature read `{role}.{}` lacks its exact local {} writer",
                feature_name(feature),
                feature_name(feature),
            ))
        })?;
    let FeatureExpr::MatchVocab {
        role: writer_role, ..
    } = writer.value()
    else {
        return Err(internal(&format!(
            "lexical feature read `{role}.{}` requires an exhaustive local match writer",
            feature_name(feature),
        )));
    };
    if identifier_key(writer_role) != identifier_key(role) {
        return Err(internal(&format!(
            "lexical feature read `{role}.{}` has a mismatched local match writer",
            feature_name(feature),
        )));
    }
    let vocabulary = construction.field(&identifier_key(writer_role))?;
    if vocabulary.kind() != ConstructionFieldKind::Lex {
        return Err(internal(
            "lexical feature writer is not rooted in a vocabulary role",
        ));
    }
    Ok((writer_role, vocabulary.terminal()))
}

fn emit_feature_helper(
    validated: &SemanticPlan,
    category: &str,
    members: &[&ConstructionPlan],
    feature: Feature,
) -> syn::Result<GeneratedItem> {
    let function = ident(&feature_helper(feature_name(feature), category));
    let ty = ident(category);
    let mut allocator = LocalAllocator::default();
    for construction in members {
        let equation = validated
            .feature_equations(construction.construction_id())
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Construction(found) if *found == feature)
            })
            .ok_or_else(|| internal("feature helper construction lacks equation"))?;
        reserve_feature_callees(validated, construction, equation.value(), &mut allocator)?;
    }
    let argument = allocator.allocate(&category_argument(category));
    let return_ty = match feature {
        Feature::Agreement => quote! { Agreement },
        Feature::Cardinality => quote! { Cardinality },
        Feature::Number => quote! { Number },
        Feature::Onset => quote! { Onset },
        Feature::PossessiveEnding => quote! { PossessiveEnding },
    };
    let mut entries: Vec<(TokenStream, String, TokenStream)> = Vec::new();
    for construction in members {
        let mut arm_allocator = allocator.clone();
        let equation = validated.feature_equations(construction.construction_id()).iter().find(|equation| {
            matches!(equation.target(), FeaturePlace::Construction(found) if *found == feature)
        }).ok_or_else(|| internal("feature helper construction lacks equation"))?;
        let variant = ident(construction.category_variant());
        let element = ident(construction.element_type());
        if let FeatureExpr::MatchVocab { role, arms: values } = equation.value()
            && !needs_whole_value(construction)
        {
            let field = construction.field(&identifier_key(role))?;
            let field_type = ident(field.terminal());
            let other_fields = construction
                .fields()
                .iter()
                .filter(|candidate| candidate.name_key() != identifier_key(role))
                .map(|candidate| {
                    let name = candidate.name();
                    quote! { #name: _ }
                })
                .collect::<Vec<_>>();
            for (value_variant, value) in values {
                let value_variant = value_variant.value();
                let value = feature_value(*value);
                let pattern = quote! { #ty::#variant(#element { #role: #field_type::#value_variant, #(#other_fields),* }) };
                entries.push((pattern, value.to_string(), value));
            }
        } else {
            let mut roles = feature_roles(validated, construction, equation.value())?;
            extend_bound_prefix_guard_roles(validated, construction, equation.value(), &mut roles)?;
            let (pattern, locals) = feature_constant_pattern(
                validated,
                construction,
                &ty,
                &variant,
                &element,
                &roles,
                &mut arm_allocator,
            );
            let value = feature_expr(validated, construction, equation.value(), feature, &locals)?;
            let value_key = value.to_string();
            let group_key = if needs_whole_value(construction) || !roles.is_empty() {
                format!("{}:{value_key}", construction.construction_id())
            } else {
                value_key
            };
            entries.push((pattern, group_key, value));
        }
    }
    let mut groups: Vec<(String, TokenStream, Vec<TokenStream>)> = Vec::new();
    for (pattern, key, value) in entries {
        if let Some((_, _, patterns)) = groups.iter_mut().find(|(found, _, _)| found == &key) {
            patterns.push(pattern);
        } else {
            groups.push((key, value, vec![pattern]));
        }
    }
    let arms = groups
        .into_iter()
        .map(|(_, value, patterns)| quote! { #(#patterns)|* => #value });
    let realized_surface_feature = matches!(feature, Feature::Onset | Feature::PossessiveEnding);
    let context = realized_surface_feature.then(|| quote! { , context: &ParseContext<'_> });
    let environment = (realized_surface_feature && validated.needs_parser_environment())
        .then(|| quote! { , environment: &crate::environment::ParserEnvironment });
    let used_context = realized_surface_feature.then(|| quote! { let _ = context; });
    let used_environment = (realized_surface_feature && validated.needs_parser_environment())
        .then(|| quote! { let _ = environment; });
    let tokens = quote! {
        #[expect(
            clippy::unnested_or_patterns,
            reason = "equal derived feature values intentionally share flat construction patterns"
        )]
        fn #function(#argument: &#ty #context #environment) -> #return_ty {
            #used_context
            #used_environment
            match #argument { #(#arms,)* }
        }
    };
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        tokens,
        members
            .iter()
            .map(|construction| {
                DeclarationKey::new(
                    DeclarationKind::Construction,
                    construction.construction_id(),
                )
            })
            .collect(),
    ))
}

fn emit_sum_agreement_helper(
    validated: &SemanticPlan,
    sum: &crate::semantic::SumPlan,
) -> syn::Result<GeneratedItem> {
    let function_name = feature_helper("agreement", sum.name());
    let function = ident(&function_name);
    let ty = ident(sum.name());
    let arms = sum
        .alternatives()
        .iter()
        .map(|alternative| {
            let variant = ident(alternative.name());
            let helper_name = match alternative.value() {
                ValueKindPlan::Category(category)
                    if validated.category_carries_agreement(category) =>
                {
                    category
                }
                ValueKindPlan::Sum(nested)
                    if validated.sum_carries_agreement(nested)
                        && !validated.sum_requires_external_agreement(nested) =>
                {
                    nested
                }
                ValueKindPlan::Category(_)
                | ValueKindPlan::Sum(_)
                | ValueKindPlan::Product(_)
                | ValueKindPlan::Lex(_)
                | ValueKindPlan::Identity(_) => {
                    return Err(internal(
                        "agreement-bearing sum alternative lacks a generated agreement helper",
                    ));
                }
            };
            let helper = ident(&feature_helper("agreement", helper_name));
            Ok(quote! { #ty::#variant(value) => #helper(value) })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: &#ty) -> Agreement {
                match value { #(#arms),* }
            }
        },
        vec![DeclarationKey::new(
            DeclarationKind::AbstractSum,
            sum.name(),
        )],
    ))
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps the feature-pattern lowering interface uniformly fallible"
)]
fn feature_roles(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    expression: &FeatureExpr,
) -> syn::Result<HashSet<String>> {
    fn collect(
        validated: &SemanticPlan,
        construction: &ConstructionPlan,
        expression: &FeatureExpr,
        roles: &mut HashSet<String>,
        visiting: &mut HashSet<(String, Feature)>,
    ) -> syn::Result<()> {
        match expression {
            FeatureExpr::Constant(_) => Ok(()),
            FeatureExpr::MatchVocab { role, .. } => {
                roles.insert(identifier_key(role));
                Ok(())
            }
            FeatureExpr::FromRole {
                role,
                feature: source_feature,
            } => {
                let key = (identifier_key(role), *source_feature);
                if !visiting.insert(key.clone()) {
                    return Err(internal("sealed feature flow contains a cycle"));
                }
                let field = construction
                    .fields()
                    .iter()
                    .find(|field| field.name_key() == identifier_key(role));
                let writer = validated
                    .feature_equations(construction.construction_id())
                    .iter()
                    .find(|equation| {
                        matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
                    });
                let derived_role = field.is_some_and(|field| {
                    field.kind() == ConstructionFieldKind::Category
                        || validated
                            .runtime_declaration_noun_for(field.terminal())
                            .is_some()
                });
                let result = if field.is_none() || derived_role && writer.is_some() {
                    if let Some(writer) = writer {
                        collect(validated, construction, writer.value(), roles, visiting)
                    } else {
                        Ok(())
                    }
                } else {
                    roles.insert(identifier_key(role));
                    Ok(())
                };
                visiting.remove(&key);
                result
            }
        }
    }

    let mut roles = HashSet::new();
    collect(
        validated,
        construction,
        expression,
        &mut roles,
        &mut HashSet::new(),
    )?;
    Ok(roles)
}

fn extend_bound_prefix_guard_roles(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    expression: &FeatureExpr,
    roles: &mut HashSet<String>,
) -> syn::Result<()> {
    let FeatureExpr::FromRole {
        role,
        feature: Feature::Onset,
    } = expression
    else {
        return Ok(());
    };
    let role = identifier_key(role);
    let has_bound_prefix = construction.forms().iter().any(|form| {
        form.atoms().iter().any(|atom| {
            let AtomPlan::Bound {
                direction: crate::semantic::BoundDirectionPlan::Prefix,
                value,
                ..
            } = atom
            else {
                return false;
            };
            matches!(
                value.as_ref(),
                AtomPlan::Category { role: found, .. }
                    | AtomPlan::Lex { role: found, .. }
                    | AtomPlan::Identity { role: found, .. }
                    | AtomPlan::Noun { role: found, .. }
                    if found == &role
            )
        })
    });
    if !has_bound_prefix {
        return Ok(());
    }
    for domain in construction
        .forms()
        .iter()
        .filter_map(|form| form.guard().predicate())
        .flat_map(crate::semantic::FinitePredicatePlan::domains)
    {
        if !domain.role().starts_with('@') {
            roles.insert(domain.role().to_owned());
            continue;
        }
        let crate::semantic::FiniteDomainKindPlan::Feature { feature, .. } = domain.kind() else {
            return Err(internal(
                "construction-owned form guard domain is not a feature",
            ));
        };
        let equation = validated
            .feature_equations(construction.construction_id())
            .iter()
            .find(|equation| equation.target() == &FeaturePlace::Construction(*feature))
            .ok_or_else(|| internal("construction feature guard has no equation"))?;
        roles.extend(feature_roles(validated, construction, equation.value())?);
    }
    Ok(())
}

fn feature_constant_pattern(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    category: &syn::Ident,
    variant: &syn::Ident,
    element: &syn::Ident,
    roles: &HashSet<String>,
    allocator: &mut LocalAllocator,
) -> (TokenStream, RenderLocals) {
    if construction.fields().is_empty() {
        return (
            quote! { #category::#variant(#element) },
            RenderLocals {
                whole: None,
                fields: HashMap::new(),
                category: TokenStream::new(),
            },
        );
    }
    if needs_whole_value(construction) {
        let whole = allocator.allocate(construction.construction_id());
        let pattern = quote! { #category::#variant(#whole) };
        return (
            pattern,
            RenderLocals {
                whole: Some(whole),
                fields: HashMap::new(),
                category: TokenStream::new(),
            },
        );
    }
    let mut fields = Vec::new();
    let mut locals = HashMap::new();
    for field in construction.fields() {
        let name = field.name();
        if roles.contains(&identifier_key(name)) {
            let local = allocator.allocate_ident(name);
            locals.insert(identifier_key(name), local.clone());
            if local == *name {
                fields.push(quote! { #name });
            } else {
                fields.push(quote! { #name: #local });
            }
            continue;
        }
        let pattern = if field.kind() == ConstructionFieldKind::Lex {
            if let Some(vocab) = find_vocab(validated, field.terminal()) {
                let ty = ident(vocab.name());
                let variants = vocab.variants().iter().map(|variant| {
                    let variant = ident(&identifier_key(variant.name()));
                    quote! { #ty::#variant }
                });
                quote! { #name: #(#variants)|* }
            } else {
                quote! { #name: _ }
            }
        } else {
            quote! { #name: _ }
        };
        fields.push(pattern);
    }
    (
        quote! { #category::#variant(#element { #(#fields),* }) },
        RenderLocals {
            whole: None,
            fields: locals,
            category: TokenStream::new(),
        },
    )
}

fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::Bare => quote! { Agreement::Bare },
        FeatureValue::ThirdPersonSingular => quote! { Agreement::ThirdPersonSingular },
        FeatureValue::Singular => quote! { Number::Singular },
        FeatureValue::Plural => quote! { Number::Plural },
        FeatureValue::Consonant => quote! { Onset::Consonant },
        FeatureValue::Vowel => quote! { Onset::Vowel },
        FeatureValue::EndsInS => quote! { PossessiveEnding::EndsInS },
        FeatureValue::Other => quote! { PossessiveEnding::Other },
        FeatureValue::Zero => quote! { Cardinality::Zero },
        FeatureValue::One => quote! { Cardinality::One },
        FeatureValue::TwoPlus => quote! { Cardinality::TwoPlus },
    }
}

fn render_category_order(
    categories: &[(String, Vec<&ConstructionPlan>)],
    roots: &[&RootPlan],
    nested: &HashSet<String>,
) -> Vec<String> {
    let mut order = Vec::new();
    let mut queued = HashSet::new();
    let known_categories = categories
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<HashSet<_>>();
    for root in roots {
        let root_name = root.category().to_owned();
        if nested.contains(&root_name) {
            continue;
        }
        if let Some((_, members)) = categories.iter().find(|(name, _)| name == &root_name) {
            enqueue_role_categories(members, &known_categories, &mut order, &mut queued);
        }
    }
    let mut index = 0;
    while index < order.len() {
        let name = order[index].clone();
        if let Some((_, members)) = categories.iter().find(|(category, _)| category == &name) {
            enqueue_role_categories(members, &known_categories, &mut order, &mut queued);
        }
        index += 1;
    }
    for (name, _) in categories {
        if queued.insert(name.clone()) {
            order.push(name.clone());
        }
    }
    order
}

fn enqueue_role_categories(
    members: &[&ConstructionPlan],
    known_categories: &HashSet<String>,
    order: &mut Vec<String>,
    queued: &mut HashSet<String>,
) {
    for construction in members {
        for atom in construction.forms().iter().flat_map(FormPlan::atoms) {
            let AtomPlan::Category { category, .. } = atom else { continue };
            if known_categories.contains(category) && queued.insert(category.clone()) {
                order.push(category.clone());
            }
        }
    }
}

fn category_groups(constructions: &[ConstructionPlan]) -> Vec<(String, Vec<&ConstructionPlan>)> {
    let mut result: Vec<(String, Vec<&ConstructionPlan>)> = Vec::new();
    for construction in constructions {
        if let Some((_, members)) = result
            .iter_mut()
            .find(|(name, _)| name == construction.category())
        {
            members.push(construction);
        } else {
            result.push((construction.category().to_owned(), vec![construction]));
        }
    }
    result
}

fn find_vocab<'a>(validated: &'a SemanticPlan, name: &str) -> Option<&'a VocabPlan> {
    for terminal in validated.terminals() {
        if let TerminalPlan::Vocab(row) = terminal
            && row.name() == name
        {
            return Some(row);
        }
    }
    None
}

fn find_lexeme<'a>(validated: &'a SemanticPlan, name: &str) -> Option<&'a LexemePlan> {
    validated
        .terminals()
        .iter()
        .find_map(|terminal| match terminal {
            TerminalPlan::Lexeme(lexeme) if lexeme.name() == name => Some(lexeme),
            TerminalPlan::Lexeme(_)
            | TerminalPlan::Vocab(_)
            | TerminalPlan::Binding(_)
            | TerminalPlan::ContextIdentity(_)
            | TerminalPlan::CatalogIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::UnsignedNumber(_)
            | TerminalPlan::DeclarationNoun(_) => None,
        })
}

fn find_binding<'a>(validated: &'a SemanticPlan, name: &str) -> syn::Result<&'a BindingPlan> {
    for terminal in validated.terminals() {
        if let TerminalPlan::Binding(row) = terminal
            && row.name() == name
        {
            return Ok(row);
        }
    }
    Err(internal("resolved terminal binding is absent"))
}

fn find_context_identity<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> Option<&'a crate::semantic::ContextIdentityPlan> {
    validated
        .runtime_context_identities()
        .find(|identity| identity.name() == name)
}

fn find_catalog_identity<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> Option<&'a crate::semantic::CatalogIdentityPlan> {
    validated
        .runtime_catalog_identities()
        .find_map(|(_, identity)| (identity.name() == name).then_some(identity))
}

fn find_signed_decimal<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> Option<&'a SignedDecimalPlan> {
    validated
        .runtime_signed_decimal()
        .filter(|codec| codec.codec_name() == name)
}

fn find_unsigned_number<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> Option<&'a UnsignedNumberPlan> {
    validated
        .runtime_unsigned_numbers()
        .find(|codec| codec.codec_name() == name)
}

fn field_value(
    construction: &ConstructionPlan,
    role: &str,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let field = construction.field(role)?;
    if field.accessor_mode().is_some() {
        let whole = locals
            .whole
            .as_ref()
            .ok_or_else(|| internal("render accessor lacks its allocated whole local"))?;
        let method = field.name();
        Ok(quote! { #whole.#method() })
    } else if let Some(whole) = &locals.whole {
        let role = field.name();
        Ok(quote! { &#whole.#role })
    } else {
        let local = locals
            .fields
            .get(role)
            .ok_or_else(|| internal("public render field lacks its allocated local"))?;
        Ok(quote! { #local })
    }
}

fn copy_value(
    construction: &ConstructionPlan,
    role: &str,
    value: TokenStream,
) -> syn::Result<TokenStream> {
    let field = construction.field(role)?;
    Ok(if field.accessor_mode() == Some(AccessorMode::Copy) {
        value
    } else {
        quote! { *#value }
    })
}

fn needs_whole_value(construction: &ConstructionPlan) -> bool {
    construction
        .fields()
        .iter()
        .any(|field| field.accessor_mode().is_some())
}

fn render_category_name(category: &str, root: bool) -> syn::Ident {
    ident(&category_renderer(category, root))
}

fn category_argument(category: &str) -> String {
    let snake = snake_case(category);
    if snake.ends_with("_phrase") { "phrase".to_owned() } else { snake }
}
fn render_vocab_argument(name: &str) -> String {
    let snake = snake_case(name);
    snake.strip_suffix("_word").unwrap_or(&snake).to_owned()
}

fn feature_name(feature: Feature) -> &'static str {
    match feature {
        Feature::Agreement => "agreement",
        Feature::Cardinality => "cardinality",
        Feature::Number => "number",
        Feature::Onset => "onset",
        Feature::PossessiveEnding => "possessive_ending",
    }
}
fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::manual_assert,
        clippy::match_wildcard_for_single_variants,
        clippy::too_many_lines,
        reason = "literal full-surface structural oracles retain detailed mismatch output"
    )]
    use quote::ToTokens;

    #[test]
    fn closed_noun_onset_helper_matches_the_exact_member_and_number_row() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun {
                Irregular = "artifact" { Plural = "relics", },
            }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type];
                    feature = Number;
                }
            }
            construction common: NounPhrase {
                element Common { head: lex Noun, }
                derive number = Values::Singular;
                derive onset = head.onset;
                form common = noun(head);
            }
            construction wrapper: Root {
                element Wrapper { head: NounPhrase, }
                derive onset = head.onset;
                form an when head.onset is Vowel = "an" head;
                form a otherwise = "a" head;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("irregular closed noun rows compile");
        let helper = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { kind: crate::NamedKind::Function, name } if name == "onset_for_noun_phrase"))
            .expect("noun-phrase onset helper is generated")
            .tokens
            .to_string();
        for expected in [
            "Noun :: Lexeme (NounLexeme :: Irregular) , Number :: Singular",
            "Noun :: Lexeme (NounLexeme :: Irregular) , Number :: Plural",
            "Onset :: Vowel",
            "Onset :: Consonant",
        ] {
            assert!(helper.contains(expected), "missing `{expected}`: {helper}");
        }
    }

    #[test]
    fn guarded_form_renderer_selects_atoms_from_the_same_finite_partition() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { That = "that", Those = "those", Other = "other", }
            construction demonstrative: NounPhrase {
                element Demonstrative { word: lex Word, }
                form that when word is That = "singular" lex(word);
                form those when word is Those = "plural" lex(word);
                form fallback otherwise = "fallback" lex(word);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("guarded renderer fixture generates");
        let source = expansion
            .items()
            .iter()
            .filter(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name: None } if self_ty == "NounPhrase"))
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        for fragment in [
            "Word :: That",
            "Word :: Those",
            "word (\"singular\")",
            "word (\"plural\")",
            "word (\"fallback\")",
            "form:demonstrative/that/0",
            "form:demonstrative/those/0",
            "form:demonstrative/fallback/0",
        ] {
            assert!(
                source.contains(fragment),
                "renderer lacks `{fragment}`: {source}"
            );
        }
    }

    #[test]
    fn structural_surface_lookup_is_total_and_owner_ids_retain_policy_and_edge_class() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { Alpha = "alpha", Beta = "beta", }
            construction atom: Atom {
                element AtomValue { word: lex Word, }
                form atom = lex(word);
            }
            abstract sum Branch { Atom, }
            abstract product Holder {
                maybe: opt Branch,
                items: seq Branch separated by position {
                    pair = "<P>";
                    first = "<F>";
                    middle = "<M>";
                    last = "<L>";
                } terminated by ".",
            }
            require len(Holder.items) >= 1;
            root Atom { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("structural render fixture validates");

        let source = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named { name, .. }
                        if matches!(
                            name.as_str(),
                            "SequenceOwner"
                                | "FixedSurfaceAtom"
                                | "sequence_separator"
                                | "sequence_terminator"
                                | "render_holder_items_sequence"
                                | "render_holder"
                                | "render_branch"
                        )
                )
            })
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        for fragment in [
            "enum SequenceOwner { HolderItems }",
            "fn sequence_separator (owner : SequenceOwner , member_count : usize , edge_index : usize",
            "structural:Holder/items/separator/pair/0",
            "structural:Holder/items/separator/first/0",
            "structural:Holder/items/separator/middle/0",
            "structural:Holder/items/separator/last/0",
            "structural:Holder/items/terminator/0",
            "sequence_terminator (SequenceOwner :: HolderItems)",
            "sequence_separator (SequenceOwner :: HolderItems , values . len () , index",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }

        let sequence = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "render_holder_items_sequence"))
            .expect("sequence renderer");
        let body = sequence.tokens.to_string();
        let member = body.find("render_branch").expect("member render");
        let terminator = body.find("sequence_terminator").expect("terminator lookup");
        let separator = body.find("sequence_separator").expect("separator lookup");
        assert!(
            member < terminator && terminator < separator,
            "each member renders before its terminator and following separator: {body}",
        );
    }

    #[test]
    fn authored_sentence_initial_transitions_are_uniform_in_rules_and_rendering() {
        let expansion = crate::generate(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction clause: Clause {
                element ClauseValue { left: Item, right: Item, }
                form clause = left sentence_initial(": ") right;
            }
            abstract product Pair {
                values: seq Clause separated by sentence_initial(", "),
            }
            require len(Pair.values) >= 1;
            root Clause { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("annotated form and separator surfaces generate");
        let source = expansion.tokens().to_string();

        for expected in [
            "LexicalOwnerTemplate :: TransitionedStatic",
            "transition : StructuralTransition :: SentenceInitial",
            "writer . structural_surface (\": \" , StructuralTransition :: SentenceInitial",
            "structural:Pair/values/separator/uniform/0",
        ] {
            assert!(source.contains(expected), "missing `{expected}`: {source}");
        }
    }

    #[test]
    fn context_free_construction_sequence_renderer_does_not_require_parse_context() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { Alpha = "alpha", Beta = "beta", }
            construction atom: Atom {
                element AtomValue { word: lex Word, }
                form atom = lex(word);
            }
            construction holder: Root {
                element HolderValue { items: seq Atom separated by ", ", }
                require len(HolderValue.items) >= 2;
                form holder = items;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("context-free construction sequence fixture validates");

        let sequence = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "render_holder_value_items_sequence"))
            .expect("sequence renderer")
            .tokens
            .to_string();
        assert!(
            !sequence.contains("ParseContext"),
            "a context-free member renderer must not require parse context: {sequence}",
        );

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !source.contains("render_holder_value_items_sequence (writer , items , context"),
            "a context-free construction must not pass absent parse context: {source}",
        );
    }

    #[test]
    fn category_feature_guard_threads_parse_context_through_nested_renderer() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { Opponent = "opponent", Player = "player", }
            construction noun: Nominal {
                element NounValue { word: lex Word, }
                derive onset = word.onset;
                form noun = lex(word);
            }
            construction article: Qualified {
                element ArticleValue { nominal: Nominal, }
                form an when nominal.onset is Vowel = "an" nominal;
                form a otherwise = "a" nominal;
            }
            construction root: Root {
                element RootValue { qualified: Qualified, }
                form root = qualified;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("category-feature guard fixture validates");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains(
                "fn render_qualified (writer : & mut Writer , qualified : & Qualified , context : & ParseContext"
            ),
            "guarded category renderer receives parse context: {source}",
        );
        assert!(
            source.contains("render_qualified (writer , qualified , context"),
            "the parent renderer forwards parse context: {source}",
        );
    }

    #[test]
    fn optional_contextual_category_threads_parse_context_for_some_and_none() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { Alpha = "alpha", }
            construction nominal: Nominal {
                element NominalValue { word: lex Word, }
                derive onset = word.onset;
                form nominal = lex(word);
            }
            construction qualified: Clause {
                element QualifiedValue { nominal: Nominal, }
                form vowel when nominal.onset is Vowel = "an" nominal;
                form consonant otherwise = "a" nominal;
            }
            construction direct: Direct {
                element DirectValue { clause: Clause, }
                form direct = clause;
            }
            construction optional: Ability {
                element OptionalValue { clause: opt Clause, word: opt lex Word, }
                form optional = clause lex(word);
            }
            construction root: Root {
                element RootValue { ability: Ability, }
                form root = ability;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("optional contextual category fixture validates");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for required in [
            "fn render_direct (writer : & mut Writer , direct : & Direct , context : & ParseContext",
            "fn render_ability (writer : & mut Writer , ability : & Ability , context : & ParseContext",
            "if let Some (value) = clause { render_clause (writer , value , context)",
        ] {
            assert!(
                source.contains(required),
                "optional contextual category must receive parse context: {source}",
            );
        }
        assert!(
            !source.contains("render_word (writer , value , context"),
            "non-contextual optional lexemes must not acquire parse context: {source}",
        );
    }

    #[test]
    fn uninhabited_abstract_sum_only_admits_an_absent_optional_field() {
        let expansion = crate::generate(quote::quote! {
            abstract sum ConditionClause {}
            construction triggered: Ability {
                element TriggeredValue { intervening_if: opt ConditionClause, }
                form triggered = intervening_if;
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an uninhabited condition sum validates under an optional field");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for required in [
            "pub enum ConditionClause { }",
            "pub intervening_if : Option < ConditionClause >",
            "TriggeredValueInterveningIfOptionalAbsent",
            "TriggeredValueInterveningIfOptionalPresent",
            "RuleId :: TriggeredValueInterveningIfOptionalAbsent => match children",
            "BuildValue :: TriggeredValueInterveningIfOptional (None)",
            "match * condition_clause { }",
        ] {
            assert!(
                source.contains(required),
                "uninhabited optional condition surface is missing `{required}`: {source}",
            );
        }
        for forbidden in ["ConditionClause ::", "lhs : Category :: ConditionClause"] {
            assert!(
                !source.contains(forbidden),
                "uninhabited condition must not admit a Some path `{forbidden}`: {source}",
            );
        }
        assert_eq!(
            source.matches("match * condition_clause { }").count(),
            2,
            "uninhabited sum renderer and walker must both dereference before exhaustive matching: {source}",
        );
    }

    #[test]
    fn invariant_mixed_fields_render_through_sealed_access_modes() {
        let plan = crate::test_support::invariant_access_semantic_plan();
        assert!(
            plan.boxed_fields()
                .contains(&("writer".to_owned(), "child".to_owned()))
        );
        let ast = crate::emit::ast::emit(&plan)
            .expect("mixed invariant AST emits")
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "pub struct WalkMode { pub plain : Plain , mode : Mode , child : Box < Node > , spelling : SelfReferenceSpelling , pub visitor : Marker }",
            "pub const fn mode (& self) -> Mode",
            "pub const fn child (& self) -> & Node",
            "pub const fn spelling (& self) -> SelfReferenceSpelling",
        ] {
            assert!(
                ast.contains(fragment),
                "missing AST evidence `{fragment}`: {ast}"
            );
        }
        let items = super::emit(&plan).expect("mixed invariant render emits");
        let render = items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "render_node"
                )
            })
            .expect("Node render helper")
            .tokens
            .to_string();
        let feature = items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "agreement_for_node"
                )
            })
            .expect("Node agreement helper")
            .tokens
            .to_string();
        let number_feature = items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "number_for_node"
                )
            })
            .expect("Node number helper")
            .tokens
            .to_string();

        for fragment in [
            "Node :: Writer (writer_2)",
            "render_plain (writer , * & writer_2 . plain)",
            "render_mode (writer , writer_2 . mode ())",
            "render_node (writer , writer_2 . child () , context)",
            "writer . identity ((writer_2 . spelling ()) . surface (context))",
            "render_marker (writer , * & writer_2 . visitor)",
        ] {
            assert!(render.contains(fragment), "missing `{fragment}`: {render}");
        }
        for forbidden in [
            "WalkMode {",
            "* writer_2 . mode ()",
            "& writer_2 . child ()",
            "writer_2 . mode)",
            "writer_2 . child)",
            "writer_2 . spelling)",
        ] {
            assert!(
                !render.contains(forbidden),
                "forbidden invariant projection `{forbidden}`: {render}",
            );
        }
        assert!(
            feature.contains("Node :: Writer (writer) => agreement_for_mode (writer . mode ())"),
            "feature helper must use the Copy mode accessor: {feature}",
        );
        assert!(!feature.contains("WalkMode {"), "{feature}");
        assert!(!feature.contains("* writer . mode ()"), "{feature}");
        assert!(
            number_feature
                .contains("Node :: Writer (writer) => number_for_node (writer . child ())"),
            "feature helper must use the borrowed child accessor: {number_feature}",
        );
        assert!(!number_feature.contains("WalkMode {"), "{number_feature}");
        assert!(
            !number_feature.contains("& writer . child ()"),
            "{number_feature}"
        );
    }

    #[test]
    fn invariant_compound_membership_renders_without_a_legacy_projection_gate() {
        let expansion = crate::generate(quote::quote! {
            vocab Mode { One = "one", Two = "two", Three = "three", }
            construction only: Root {
                element Only { mode: lex Mode, }
                require mode in [One, Two];
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("compound membership emits through the generated invariant AST");
        let render = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Root" && trait_name.is_none()))
            .expect("compound fixture root render")
            .tokens
            .to_string();
        assert!(render.contains("Self :: Only (only)"), "{render}");
        assert!(
            render.contains("render_mode (writer , only . mode ())"),
            "{render}"
        );
        assert!(!render.contains("Only { mode }"), "{render}");
    }

    #[test]
    fn generated_morphology_render_owners_include_the_realized_feature() {
        let expansion = crate::test_support::generated_morphology_expansion();
        let source = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Impl { trait_name, self_ty }
                        if trait_name.is_none() && self_ty == "Sentence"
                )
            })
            .expect("generated morphology root write impl is render-owned")
            .tokens
            .to_string();
        for owner in [
            "lexeme:VerbLexeme/InventedLemma/bare",
            "lexeme:VerbLexeme/InventedLemma/third_person_singular",
            "lexeme:VerbLexeme/Be/bare",
            "lexeme:VerbLexeme/Be/third_person_singular",
            "lexeme:NounLexeme/TwoWords/singular",
            "lexeme:NounLexeme/TwoWords/plural",
        ] {
            assert!(
                source.contains(owner),
                "missing render owner `{owner}`: {source}"
            );
        }
    }

    #[test]
    fn raw_fields_and_keyword_constructions_lower_to_valid_render_locals() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }

            construction payload: PayloadBox {
                element PayloadNode { r#payload: lex Marker, }
                form payload = lex(r#payload);
            }
            construction writer_field: WriterField {
                element WriterFieldNode { r#writer: lex Marker, }
                form writer_field = lex(r#writer);
            }
            construction where: Keyword {
                element KeywordNode { marker: lex Marker, }
                derive agreement = Values::Bare;
                form where = lex(marker);
            }
            construction root: Root {
                element RootNode {
                    payload: PayloadBox,
                    writer_field: WriterField,
                    keyword: Keyword,
                }
                derive verb.agreement = keyword.agreement;
                form root = payload writer_field keyword verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("raw fields and a DSL-keyword construction generate without panic");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "PayloadNode { r#payload : payload }",
            "render_marker (writer , * payload)",
            "WriterFieldNode { r#writer : writer_2 }",
            "render_marker (writer , * writer_2)",
            "Keyword :: Where (KeywordNode { marker })",
            "render_marker (writer , * marker)",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
        syn::parse2::<syn::File>(expansion.tokens()).expect("the complete expansion reparses");
    }

    #[test]
    fn generated_render_locals_are_hygienic_across_abi_and_helper_names() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            vocab WriterWord { One = "writer", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            identity SelfRef {
                value_type = SelfRef;
                lexical = Lexical::SelfRef;
                render context_identity { Full => card_name, }
                build { pattern = BuildValue::SelfRef(value); construct = value; }
                traversal {
                    callback = copy;
                    argument = value;
                    variant Full;
                }
            }

            construction child: Child {
                element ChildNode {}
                derive agreement = Values::Bare;
                form child = "child";
            }
            construction contextual: Action {
                element ContextualAction { agreement: lex Marker, }
                derive agreement = verb.agreement;
                form contextual = lex(agreement) verb(Verbs::Act);
            }
            construction wrapper: RenderChild {
                element Wrapper { child: Child, }
                form wrapper = child;
            }
            construction wrapped: AgreementForChild {
                element Wrapped { child: Child, }
                derive agreement = child.agreement;
                form wrapped = child;
            }
            construction writer: WriterRender {
                element WriterRenderNode { marker: lex Marker, }
                form writer = lex(marker);
            }
            construction root: Root {
                element RootNode {
                    writer: lex Marker,
                    context: identity SelfRef,
                    action: Action,
                    wrapper: RenderChild,
                    wrapped: AgreementForChild,
                    word: lex WriterWord,
                }
                derive action.agreement = Values::Bare;
                derive verb.agreement = wrapped.agreement;
                form root = lex(writer) identity(context) action wrapper wrapped lex(word)
                    verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("adversarial render names remain valid declaration vocabulary");

        let source = expansion
            .items()
            .iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Impl { trait_name, .. } => {
                    trait_name.is_none() || trait_name.as_deref() == Some("Render")
                }
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } => {
                    name.starts_with("render_")
                        || name.starts_with("agreement_for_")
                        || name.starts_with("number_for_")
                }
                crate::ItemKey::Named { .. } => false,
            })
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "ContextualAction { agreement : agreement_2 }",
            "render_marker (writer , * agreement_2)",
            "surface_for_verbs (Verbs :: Act , agreement)",
            "RootNode { writer : writer_2 , context : context_2",
            "render_marker (writer , * writer_2)",
            "writer . identity (context . card_name ())",
            "match * context_2",
            "fn render_writer_word (writer : & mut Writer , writer_2 : WriterWord)",
            "fn render_render_child (writer : & mut Writer , render_child_2 : & RenderChild)",
            "match render_child_2",
            "render_child (writer , child)",
            "fn agreement_for_agreement_for_child (agreement_for_child_2 : & AgreementForChild)",
            "match agreement_for_child_2",
            "agreement_for_child (child)",
            "WriterRender :: Writer (WriterRenderNode { marker })",
            "render_marker (writer , * marker)",
        ] {
            assert!(
                source.contains(fragment),
                "missing hygienic `{fragment}`: {source}"
            );
        }
        for shadowed in [
            "ContextualAction { agreement })",
            "RootNode { writer , context",
            "writer : & mut Writer , writer : WriterWord",
            "writer : & mut Writer , render_child : & RenderChild",
            "fn agreement_for_agreement_for_child (agreement_for_child : & AgreementForChild)",
        ] {
            assert!(
                !source.contains(shadowed),
                "shadowed binding survived as `{shadowed}`: {source}",
            );
        }
    }

    #[test]
    fn role_derived_noun_render_uses_declared_feature_helper() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("role-derived noun render lowers");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains("number_for_source (source)"),
            "the derived feature helper must read the declared source role: {source}"
        );
        assert!(
            source.contains("render_head (writer , head , number_for_phrase (self))"),
            "noun rendering must consume the construction feature helper: {source}"
        );
    }

    #[test]
    fn generated_vocab_feature_helpers_are_defined() {
        let expansion = crate::test_support::generated_morphology_expansion();
        let helper = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "agreement_for_pronoun"
                )
            })
            .expect("the vocabulary feature helper is emitted");
        assert_eq!(
            helper
                .origins
                .iter()
                .map(crate::DeclarationKey::name)
                .collect::<Vec<_>>(),
            ["statement", "question"],
        );
        assert_eq!(
            parse(helper),
            syn::parse_quote! {
                fn agreement_for_pronoun(value: Pronoun) -> Agreement {
                    match value {
                        Pronoun::It => Agreement::ThirdPersonSingular,
                        Pronoun::You => Agreement::Bare,
                    }
                }
            }
        );
    }

    #[test]
    fn inconsistent_vocab_feature_helpers_are_rejected_as_sealed_plan_errors() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Pronoun { It = "it", You = "you", }
                construction first: Root {
                    element First { pronoun: lex Pronoun, }
                    derive pronoun.agreement = match pronoun {
                        It => Values::ThirdPersonSingular,
                        You => Values::Bare,
                    };
                    derive agreement = pronoun.agreement;
                    form first = lex(pronoun);
                }
                construction second: Root {
                    element Second { pronoun: lex Pronoun, }
                    derive pronoun.agreement = match pronoun {
                        It => Values::Bare,
                        You => Values::ThirdPersonSingular,
                    };
                    derive agreement = pronoun.agreement;
                    form second = lex(pronoun);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("inconsistent sealed-helper fixture parses"),
        )
        .expect("each exhaustive equation validates independently");

        let error = super::emit(validated.semantic())
            .expect_err("one vocabulary helper cannot represent inconsistent sealed mappings");
        assert_eq!(
            error.to_string(),
            "sealed vocabulary feature mapping is inconsistent for `Pronoun.agreement`",
        );
    }

    #[test]
    fn zero_noun_vocab_match_emits_exact_category_feature_arms() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_without_noun_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("zero-noun feature helpers lower");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains(
                "Source :: Source (SourceNode { count : Count :: One , }) => Number :: Singular"
            ) && source.contains(
                "Source :: Source (SourceNode { count : Count :: Many , }) => Number :: Plural"
            ),
            "the stored vocab drives the exact number helper arms: {source}"
        );
    }

    #[test]
    fn two_noun_render_calls_share_the_declared_category_number() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("two noun render atoms lower");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "render_head (writer , left , number_for_phrase (self))",
            "render_head (writer , right , number_for_phrase (self))",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
    }

    #[test]
    fn explicit_lexical_feature_dependencies_drive_render_lowering() {
        let expansion = crate::generate(quote::quote! {
            vocab Person { One = "one", Many = "many", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb {
                Be = "be" { Bare = "are", ThirdPersonSingular = "is", },
            }
            construction only: Root {
                element Only { person: lex Person, }
                require person is One;
                derive person.agreement = match person {
                    One => Values::Bare,
                    Many => Values::ThirdPersonSingular,
                };
                derive agreement = person.agreement;
                derive verb.agreement = person.agreement;
                form only = lex(person) verb(Verbs::Be);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an explicit lexical provider is valid");
        let implementation = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Root" && trait_name.is_none()))
            .expect("root write impl");
        let source = implementation.tokens.to_string();
        assert!(
            source.contains("agreement_for_person (only . person ())"),
            "the consuming verb operand uses the canonical helper from the explicit writer: {source}",
        );
    }

    #[test]
    fn canonical_lexical_feature_lowering_requires_the_exact_writer() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Person { One = "one", Many = "many", }
                construction only: Root {
                    element Only { person: lex Person, }
                    derive person.agreement = match person {
                        One => Values::Bare,
                        Many => Values::ThirdPersonSingular,
                    };
                    derive agreement = person.agreement;
                    form only = lex(person);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .expect("the exact lexical writer validates");
        let plan = validated.semantic();
        let construction = plan.constructions().first().expect("construction row");
        let role = syn::parse_quote!(person);
        let (writer_role, vocabulary) = super::canonical_lexical_feature_lowering(
            plan,
            construction,
            &role,
            crate::feature::Feature::Agreement,
        )
        .expect("the exact role.feature writer is retrievable");
        assert_eq!(writer_role, "person");
        assert_eq!(vocabulary, "Person");
        let missing = super::canonical_lexical_feature_lowering(
            plan,
            construction,
            &role,
            crate::feature::Feature::Number,
        )
        .expect_err("a different role.feature writer cannot satisfy the read");
        assert!(
            missing.to_string().contains("exact local number writer"),
            "{missing}"
        );
    }

    #[test]
    fn representative_root_metadata_controls_standalone_rendering() {
        let expansion = crate::test_support::representative_expansion();
        let render_impls = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Impl { trait_name, .. }
                        if trait_name.as_deref() == Some("Render")
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(render_impls.len(), 1);
        assert!(matches!(
            &render_impls[0].key,
            crate::ItemKey::Impl { self_ty, .. } if self_ty == "Action"
        ));
        let write_impl = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Action" && trait_name.is_none()))
            .expect("root write impl");
        let syn::Item::Impl(item) = parse(write_impl) else {
            panic!("root write item is an impl");
        };
        let syn::ImplItem::Fn(method) = &item.items[0] else {
            panic!("root render impl contains a method");
        };
        assert!(
            method
                .block
                .to_token_stream()
                .to_string()
                .contains("writer . punctuation ('.')")
        );
    }

    #[test]
    fn standalone_abstract_product_root_uses_generated_structural_renderer_only_when_selected() {
        let expansion = crate::generate(quote::quote! {
            construction only: Cat { element Only {} form only = "only"; }
            abstract product NestedOnly { child: Cat, }
            abstract product Document { children: seq Cat, }
            require len(Document.children) >= 1;
            root NestedOnly { eoi = false; standalone_render = false; }
            root Document { eoi = true; standalone_render = true; }
        })
        .expect("a standalone structural root reaches render emission");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for required in [
            "impl Document",
            "impl Render for Document",
            "fn write_document_render",
            "render_document_with_claims",
            "render_document (writer , value , context)",
        ] {
            assert!(source.contains(required), "missing `{required}`: {source}");
        }
        for forbidden in [
            "impl NestedOnly",
            "impl Render for NestedOnly",
            "render_nested_only_with_claims",
            "write_nested_only_render",
        ] {
            assert!(
                !source.contains(forbidden),
                "nested-only structural root acquired `{forbidden}`: {source}",
            );
        }
    }

    #[test]
    fn non_standalone_root_with_parent_supplied_agreement_reaches_emission() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction action: Child {
                element ActionNode {}
                derive agreement = verb.agreement;
                form action = verb(Verbs::Act);
            }
            construction parent: Parent {
                element ParentNode { child: Child, }
                derive child.agreement = Values::Bare;
                form parent = child;
            }
            root Child { punctuation = "!"; eoi = true; standalone_render = false; }
            root Parent { punctuation = "."; eoi = false; standalone_render = true; }
        })
        .expect("a nested-only root may receive agreement from its parent");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains("render_child (writer , child , Agreement :: Bare)"),
            "the parent must supply the nested-only root's agreement: {source}",
        );
        assert!(
            !source.contains("impl Render for Child"),
            "the nested-only root must not acquire standalone rendering: {source}",
        );
        for forbidden in ["write_child_render", "render_child_with_claims"] {
            assert!(
                !source.contains(forbidden),
                "the nested-only root must not acquire `{forbidden}`: {source}",
            );
        }
    }

    #[test]
    fn synthetic_projection_has_exact_render_ownership_and_feature_dispatch() {
        let expansion = crate::test_support::synthetic_projection_expansion();
        let render = expansion
            .items()
            .iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Impl {
                    self_ty,
                    trait_name,
                } => {
                    self_ty == "Document"
                        && (trait_name.is_none() || trait_name.as_deref() == Some("Render"))
                }
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } => {
                    name.starts_with("render_")
                        || name == "agreement_for_expr"
                        || name == "number_for_expr"
                }
                _ => false,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            render
                .iter()
                .map(|item| match &item.key {
                    crate::ItemKey::Impl {
                        self_ty,
                        trait_name: Some(_),
                    } => format!("impl Render for {self_ty}"),
                    crate::ItemKey::Impl {
                        self_ty,
                        trait_name: None,
                    } => format!("impl {self_ty}"),
                    crate::ItemKey::Named { name, .. } => name.clone(),
                })
                .collect::<Vec<_>>(),
            [
                "impl Document",
                "impl Render for Document",
                "render_document_with_claims",
                "render_expr",
                "render_predicate",
                "render_tag",
                "render_mode",
                "agreement_for_expr",
                "number_for_expr",
            ],
        );
        assert_eq!(
            render
                .iter()
                .map(|item| {
                    item.origins
                        .iter()
                        .map(crate::DeclarationKey::name)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            [
                vec!["Document"],
                vec!["Document"],
                vec!["Document"],
                vec!["leaf", "nested"],
                vec!["action", "idle"],
                vec!["solo"],
                vec!["Mode"],
                vec!["leaf", "nested"],
                vec!["leaf", "nested"],
            ],
        );

        let root = parse(render[0]).to_token_stream().to_string();
        for fragment in [
            "Self :: Document (document)",
            "render_expr (writer , document . subject ())",
            "render_predicate (writer , & document . predicate , agreement_for_expr (document . subject ()))",
            "Handle :: Primary => writer . identity (context . primary_name ())",
            "Handle :: Alias => writer . identity (context . alias_name ())",
            "render_pair (writer , & document . pair)",
            "writer . punctuation ('!')",
        ] {
            assert!(root.contains(fragment), "root render lacks `{fragment}`");
        }

        let agreement = parse(render[7]).to_token_stream().to_string();
        assert!(agreement.contains("Expr :: Leaf"));
        assert!(
            agreement.contains(
                "mode : Mode :: Solo , resource : _ }) => Agreement :: ThirdPersonSingular"
            )
        );
        assert!(agreement.contains("mode : Mode :: Group , resource : _ }) => Agreement :: Bare"));
        assert!(agreement.contains("Expr :: Nested"));
        assert!(agreement.contains("agreement_for_expr (next)"));

        let number = parse(render[8]).to_token_stream().to_string();
        assert!(number.contains("mode : Mode :: Solo , resource : _ }) => Number :: Singular"));
        assert!(number.contains("mode : Mode :: Group , resource : _ }) => Number :: Plural"));
        assert!(number.contains("number_for_expr (next)"));
    }
    fn parse(item: &crate::GeneratedItem) -> syn::Item {
        syn::parse2::<syn::File>(item.tokens.clone())
            .unwrap()
            .items
            .into_iter()
            .next()
            .unwrap()
    }
}
