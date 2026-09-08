use deckmaste_lexical_model::Frame;
use deckmaste_lexical_model::FrameItem;
use deckmaste_lexical_model::FrameSlot;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::ir::Build;
use crate::ir::DomainKind;
use crate::ir::Ir;
use crate::ir::Piece;
use crate::ir::Symbol;
use crate::parse::FieldType;

pub(crate) fn emit(ir: &Ir) -> syn::Result<TokenStream> {
    let runtime: syn::File = syn::parse_str(include_str!("runtime.rs.txt"))?;
    let module = &ir.module;
    let visibility = &ir.visibility;
    let categories = &ir.categories;
    let public = &categories[..ir.public_categories];
    let count = ir.features.len();
    let grammar = grammar(ir);
    let project = projection(ir);
    let ast = ast(ir);
    let materialize = materializer(ir);
    Ok(quote! {
        #visibility mod #module {
            #runtime
            const FEATURE_COUNT: usize = #count;
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Category { #(#categories),* }
            impl Category {
                fn is_public(self) -> bool { matches!(self, #(Self::#public)|*) }
            }
            #grammar
            #project
            #ast
            #materialize
        }
    })
}

fn grammar(ir: &Ir) -> TokenStream {
    let productions = ir.rules.iter().map(|rule| {
        let category = &ir.categories[rule.category];
        let symbols = rule.symbols.iter().map(|symbol| match symbol {
            Symbol::Literal(text) => quote!(::deckmaste_english_v3::Symbol::Literal(#text.into())),
            Symbol::Lexical(name) => { let name = format_ident!("{name}"); quote!(::deckmaste_english_v3::Symbol::Lexical(::deckmaste_lexical::Category::#name)) }
            Symbol::Nonterminal(index) => { let name = &ir.categories[*index]; quote!(::deckmaste_english_v3::Symbol::Nonterminal(Category::#name)) }
        });
        quote!(::deckmaste_english_v3::Production { category: Category::#category, symbols: vec![#(#symbols),*] })
    });
    let rules = ir.rules.iter().map(|rule| {
        let checks = rule.checks.iter().map(|checks| {
            let entries = checks
                .iter()
                .map(|(feature, register)| quote!((#feature, #register)));
            quote!(vec![#(#entries),*])
        });
        let initial = rule.initial.iter().map(|value| {
            value
                .as_ref()
                .map_or_else(|| quote!(None), |value| quote!(Some(#value)))
        });
        let exports = rule
            .exports
            .iter()
            .map(|(feature, register)| quote!((#feature, #register)));
        let release = rule
            .release
            .iter()
            .map(|registers| quote!(vec![#(#registers),*]));
        quote!(Rule {
            checks: vec![#(#checks),*],
            initial: vec![#(#initial),*],
            exports: vec![#(#exports),*],
            release: vec![#(#release),*]
        })
    });
    quote! { impl Default for Grammar {
        fn default() -> Self { Self { rules: vec![#(#rules),*], productions: vec![#(#productions),*] } }
    } }
}

fn projection(ir: &Ir) -> TokenStream {
    let features = ir.features.iter().enumerate().filter_map(|(index, domain)| match &domain.kind {
        DomainKind::Builtin(ty) if domain.name != "form" => {
            let field = format_ident!("{}", domain.name);
            let ty = format_ident!("{ty}");
            Some(quote!(base.values[#index] = features.#field.map(FeatureValue::#ty);))
        }
        DomainKind::Custom => {
            let name = &domain.name;
            let values = domain.values.iter().enumerate().map(|(value, name)| quote!(#name => Some(FeatureValue::Custom(#index, #value))));
            Some(quote!(base.values[#index] = properties.features.get(#name).and_then(|value| match value.as_str() { #(#values,)* _ => None });))
        }
        _ => None,
    });
    let frames = ir.frames.iter().map(|(_, frame)| frame_tokens(frame));
    quote! {
        fn project(features: ::deckmaste_english_v3::LexicalFeatures<'_>) -> Vec<Summary> {
            let ::deckmaste_english_v3::LexicalFeatures::Word { form, features, properties, .. } = features else { return vec![Summary::default()]; };
            let mut base = Summary::default();
            #(#features)*
            base.values[5] = Some(FeatureValue::WordForm(form));
            static FRAMES: ::std::sync::OnceLock<Vec<::deckmaste_lexical::Frame>> = ::std::sync::OnceLock::new();
            let declared_frames = FRAMES.get_or_init(|| vec![#(#frames),*]);
            let frame_choices: Vec<_> = if properties.frames.is_empty() { vec![None] } else {
                properties.frames.iter().enumerate().filter(|(index, frame)| !properties.frames[..*index].contains(frame)).map(|(index, _)| Some(index)).collect()
            };
            let count_uses: ::std::collections::BTreeSet<_> = if properties.countability.is_empty() { [None].into_iter().collect() } else {
                properties.countability.iter().map(|value| Some(*value == ::deckmaste_lexical::Countability::Count)).collect()
            };
            let mut summaries = vec![];
            for frame in frame_choices {
                for count_use in &count_uses {
                    let mut summary = base.clone();
                    summary.frame_choice = frame;
                    summary.count_use = *count_use;
                    summary.values[6] = count_use.map(FeatureValue::Countability);
                    summary.values[7] = frame.and_then(|index| declared_frames.iter().position(|f| f == &properties.frames[index])).map(FeatureValue::Frame);
                    summaries.push(summary);
                }
            }
            summaries
        }
    }
}

fn frame_tokens(frame: &Frame) -> TokenStream {
    let kind = &frame.kind;
    let items = frame.items.iter().map(frame_item);
    quote!(::deckmaste_lexical::Frame { kind: #kind.into(), items: vec![#(#items),*] })
}
fn frame_slot(slot: &FrameSlot) -> TokenStream {
    let relation = format_ident!("{}", format!("{:?}", slot.relation));
    let category = &slot.category;
    quote!(::deckmaste_lexical::FrameSlot { relation: ::deckmaste_lexical::Relation::#relation, category: #category.into() })
}
fn frame_item(item: &FrameItem) -> TokenStream {
    match item {
        FrameItem::Argument(slot) => {
            let slot = frame_slot(slot);
            quote!(::deckmaste_lexical::FrameItem::Argument(#slot))
        }
        FrameItem::Marker { vocabulary, member } => {
            quote!(::deckmaste_lexical::FrameItem::Marker { vocabulary: #vocabulary.into(), member: #member.into() })
        }
        FrameItem::Marked {
            vocabulary,
            member,
            slot,
        } => {
            let slot = frame_slot(slot);
            quote!(::deckmaste_lexical::FrameItem::Marked { vocabulary: #vocabulary.into(), member: #member.into(), slot: #slot })
        }
        FrameItem::Optional(item) => {
            let item = frame_item(item);
            quote!(::deckmaste_lexical::FrameItem::Optional(Box::new(#item)))
        }
        FrameItem::Literal(text) => quote!(::deckmaste_lexical::FrameItem::Literal(#text.into())),
    }
}

fn ast(ir: &Ir) -> TokenStream {
    let mut variants = vec![];
    let mut categories = vec![];
    let mut admits = vec![];
    let mut writes = vec![];
    let mut visits = vec![];
    let mut words = vec![];
    for constructor in &ir.constructors {
        let name = &constructor.name;
        let owner = name.to_string();
        let category = &ir.categories[constructor.category];
        let field_names: Vec<_> = constructor.fields.iter().map(|(name, _)| name).collect();
        let bindings: Vec<_> = (0..field_names.len())
            .map(|i| format_ident!("__field{i}"))
            .collect();
        let types = constructor.fields.iter().map(|(_, ty)| match ty {
            FieldType::Lexical(_) => quote!(Word),
            FieldType::One(_) => quote!(Box<Reading>),
            FieldType::Optional(_) => quote!(Option<Box<Reading>>),
            FieldType::Repeated(_, _) => quote!(Vec<Reading>),
        });
        variants.push(quote!(#name { form: usize, #(#field_names: #types),* }));
        categories.push(quote!(Self::#name { .. } => Category::#category));
        let mut admit_forms = vec![];
        let mut write_forms = vec![];
        let mut visit_forms = vec![];
        let mut word_forms = vec![];
        for (index, form) in constructor.forms.iter().enumerate() {
            let mut summaries = vec![];
            let mut write = vec![];
            let mut visit = vec![];
            let mut word = vec![];
            for piece in &form.pieces {
                match piece {
                    Piece::Literal(text) => {
                        summaries.push(quote!(None));
                        write.push(quote!(output.push_str(#text);));
                    }
                    Piece::Field(field) => {
                        let binding = &bindings[*field];
                        let ty = &constructor.fields[*field].1;
                        let category_name = match ty {
                            FieldType::Lexical(c)
                            | FieldType::One(c)
                            | FieldType::Optional(c)
                            | FieldType::Repeated(c, _) => format_ident!("{c}"),
                        };
                        let check_category = quote!(if child.category() != Category::#category_name { return Err(Error::Invalid(#owner, "constituent Category")); });
                        match ty {
                            FieldType::Lexical(_) => {
                                summaries.push(quote!(Some(#binding.admit(lexicon, ::deckmaste_lexical::Category::#category_name)?)));
                                write.push(quote!(output.word(#binding, lexicon)?;));
                                word.push(quote!(visitor(#binding);));
                            }
                            FieldType::One(_) => {
                                summaries.push(quote!(Some({ let child = #binding; #check_category child.admit_with(grammar, lexicon)? })));
                                write.push(quote!(#binding.write(lexicon, output)?;));
                                visit.push(quote!(#binding.visit(visitor)?;));
                                word.push(quote!(#binding.visit_words(visitor)?;));
                            }
                            FieldType::Optional(_) => {
                                summaries.push(quote!(Some({ if let Some(child) = #binding { #check_category child.admit_with(grammar, lexicon)?; } Summary::default() })));
                                write.push(quote!(if let Some(child) = #binding { child.write(lexicon, output)?; }));
                                visit.push(quote!(if let Some(child) = #binding { child.visit(visitor)?; }));
                                word.push(quote!(if let Some(child) = #binding { child.visit_words(visitor)?; }));
                            }
                            FieldType::Repeated(_, separator) => {
                                summaries.push(quote!(Some({ for child in #binding { #check_category child.admit_with(grammar, lexicon)?; } Summary::default() })));
                                write.push(quote!(for (index, child) in #binding.iter().enumerate() { if index != 0 { output.push_str(#separator); } child.write(lexicon, output)?; }));
                                visit
                                    .push(quote!(for child in #binding { child.visit(visitor)?; }));
                                word.push(
                                    quote!(for child in #binding { child.visit_words(visitor)?; }),
                                );
                            }
                        }
                    }
                }
            }
            let rule = form.rule;
            admit_forms.push(quote!(#index => {
                let summaries: Vec<Option<Summary>> = vec![#(#summaries),*];
                let rule = &grammar.rules[#rule];
                let mut state = rule.initial.clone();
                for (dot, summary) in summaries.iter().enumerate() {
                    state = rule.advance(dot, &state, summary.as_ref()).ok_or(Error::Invalid(#owner, "feature admission"))?;
                }
                rule.complete(&state).ok_or(Error::Invalid(#owner, "feature export"))
            }));
            write_forms.push(quote!(#index => { #(#write)* Ok(()) }));
            visit_forms.push(quote!(#index => { #(#visit)* Ok(()) }));
            word_forms.push(quote!(#index => { #(#word)* Ok(()) }));
        }
        let pattern = quote!(Self::#name { form, #(#field_names: #bindings),* });
        admits.push(quote!(#pattern => match form { #(#admit_forms,)* _ => Err(Error::Invalid(#owner, "surface alternative")) }));
        writes.push(quote!(#pattern => match form { #(#write_forms,)* _ => Err(Error::Invalid(#owner, "surface alternative")) }));
        visits.push(quote!(#pattern => match form { #(#visit_forms,)* _ => Err(Error::Invalid(#owner, "surface alternative")) }));
        words.push(quote!(#pattern => match form { #(#word_forms,)* _ => Err(Error::Invalid(#owner, "surface alternative")) }));
    }
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Reading { #(#variants),* }
        impl Reading {
            #[must_use]
            pub fn category(&self) -> Category { match self { #(#categories),* } }
            /// Check a constructed value directly against declaration admission.
            /// # Errors
            /// Reports an invalid form, lexical value, constituent or feature equation.
            pub fn admit(&self, lexicon: &::deckmaste_lexical::Lexicon) -> Result<Summary, Error> {
                let summary = self.admit_with(&Grammar::default(), lexicon)?;
                let mut output = Realization::new();
                self.write(lexicon, &mut output)?;
                output.check_context(lexicon)?;
                Ok(summary)
            }
            fn admit_with(&self, grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon) -> Result<Summary, Error> { match self { #(#admits),* } }
            /// Realize a valid value using the lexical environment and declared surfaces.
            /// # Errors
            /// Reports the same admission errors as `admit`.
            pub fn realize(&self, lexicon: &::deckmaste_lexical::Lexicon) -> Result<String, Error> {
                self.admit_with(&Grammar::default(), lexicon)?;
                let mut output = Realization::new();
                self.write(lexicon, &mut output)?;
                output.check_context(lexicon)?;
                Ok(output.finish())
            }
            fn write(&self, lexicon: &::deckmaste_lexical::Lexicon, output: &mut Realization) -> Result<(), Error> { match self { #(#writes),* } }
            /// Visit constructions in preorder, with children in declared surface order.
            /// # Errors
            /// Reports an invalid surface alternative.
            pub fn visit(&self, visitor: &mut impl FnMut(&Self)) -> Result<(), Error> {
                visitor(self);
                match self { #(#visits),* }
            }
            /// Visit lexical identities in declared surface order, without positions.
            /// # Errors
            /// Reports an invalid surface alternative.
            pub fn visit_words(&self, visitor: &mut impl FnMut(&Word)) -> Result<(), Error> { match self { #(#words),* } }
        }
    }
}

fn materializer(ir: &Ir) -> TokenStream {
    let cases = ir.rules.iter().enumerate().map(|(index, rule)| {
        let arity = rule.symbols.len();
        let body = match &rule.build {
            Build::Construction { constructor, form, slots } => {
                let constructor = &ir.constructors[*constructor];
                let name = &constructor.name;
                let fields: Vec<_> = constructor.fields.iter().map(|(name, _)| name).collect();
                let values: Vec<_> = (0..fields.len()).map(|i| format_ident!("__field{i}")).collect();
                let consume = rule.symbols.iter().enumerate().map(|(dot, _)| {
                    if let Some(field) = slots.iter().position(|s| *s == dot) {
                        let binding = &values[field];
                        let conversion = match constructor.fields[field].1 {
                            FieldType::Lexical(_) => quote!(value.word()?),
                            FieldType::One(_) => quote!(Box::new(value.node()?)),
                            FieldType::Optional(_) => quote!(value.optional()?),
                            FieldType::Repeated(_, _) => quote!(value.repeated()?),
                        };
                        quote!(let value = children.next().ok_or(Error::Internal)?; let #binding = #conversion;)
                    } else { quote!(if !matches!(children.next(), Some(Value::Literal)) { return Err(Error::Internal); }) }
                });
                quote!(#(#consume)* Ok(Value::Reading(Reading::#name { form: #form, #(#fields: #values),* })))
            }
            Build::Absent => quote!(Ok(Value::Optional(None))),
            Build::Present => quote!(Ok(Value::Optional(Some(children.next().ok_or(Error::Internal)?.node()?)))),
            Build::Empty => quote!(Ok(Value::Repeated(vec![]))),
            Build::Singleton => quote!(Ok(Value::Repeated(vec![children.next().ok_or(Error::Internal)?.node()?]))),
            Build::Sequence => quote!(Ok(Value::Repeated(children.next().ok_or(Error::Internal)?.repeated()?))),
            Build::Append => quote! {
                let mut sequence = children.next().ok_or(Error::Internal)?.repeated()?;
                if !matches!(children.next(), Some(Value::Literal)) { return Err(Error::Internal); }
                sequence.push(children.next().ok_or(Error::Internal)?.node()?);
                Ok(Value::Repeated(sequence))
            },
        };
        quote!(#index => { if children.len() != #arity { return Err(Error::Internal); } let mut children = children.into_iter(); #body })
    });
    quote!(impl Grammar { fn materialize(&self, production: usize, children: Vec<Value>) -> Result<Value, Error> { match production { #(#cases,)* _ => Err(Error::Internal) } } })
}
