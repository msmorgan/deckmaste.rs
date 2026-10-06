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
    let selected_runtime: syn::File = syn::parse_str(include_str!("selected_frame.rs.txt"))?;
    let module = &ir.module;
    let visibility = &ir.visibility;
    let categories = &ir.categories;
    let public = &categories[..ir.public_categories];
    let count = ir.features.len();
    let positional_capitalization = ir.positional_capitalization;
    let grammar = grammar(ir);
    let project = projection(ir);
    let ast = ast(ir);
    let materialize = materializer(ir);
    Ok(quote! {
        #visibility mod #module {
            #runtime
            #selected_runtime
            const FEATURE_COUNT: usize = #count;
            const POSITIONAL_CAPITALIZATION: bool = #positional_capitalization;
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
        let table_exports = rule.table_exports.iter().map(|export| {
            let feature = export.feature;
            let registers = &export.registers;
            let rows = ir.tables[export.table].rows.iter().map(|(inputs, output)| {
                quote!((vec![#(#inputs),*], #output))
            });
            quote!(TableExport { feature: #feature, registers: vec![#(#registers),*], rows: vec![#(#rows),*] })
        });
        let table_requirements = rule.table_requirements.iter().map(|requirement| {
            let registers = &requirement.registers;
            let expected = &requirement.expected;
            let rows = ir.tables[requirement.table].rows.iter().map(|(inputs, output)| {
                quote!((vec![#(#inputs),*], #output))
            });
            quote!(TableRequirement { registers: vec![#(#registers),*], rows: vec![#(#rows),*], expected: #expected })
        });
        let segments = match &rule.segments {
            Some(crate::ir::Segments::Build(kind)) => quote!(SegmentRule::Build { kind: #kind.into(), candidates: vec![] }),
            Some(crate::ir::Segments::Share(left, right)) => quote!(SegmentRule::Share(#left, #right)),
            Some(crate::ir::Segments::Discharge(head, tail)) => quote!(SegmentRule::Discharge(#head, #tail)),
            None => quote!(SegmentRule::None),
        };
        let boundary = rule.boundary.as_ref().map_or_else(|| quote!(None), |name| quote!(Some(Boundary::#name)));
        let onset = rule.onset.as_ref().map_or_else(|| quote!(None), |name| quote!(Some(::deckmaste_lexical::Onset::#name)));
        let size = rule.symbols.len();
        quote!(Rule {
            guards: vec![FrameGuard::Any; #size],
            segments: #segments,
            boundary: #boundary,
            onset: #onset,
            checks: vec![#(#checks),*],
            initial: vec![#(#initial),*],
            exports: vec![#(#exports),*],
            table_exports: vec![#(#table_exports),*],
            table_requirements: vec![#(#table_requirements),*],
            release: vec![#(#release),*]
        })
    });
    let templates: Vec<_> = ir
        .constructors
        .iter()
        .flat_map(|c| c.forms.iter())
        .filter(|form| {
            form.fields
                .iter()
                .any(|(_, ty)| matches!(ty, FieldType::SelectedFrame(_, _)))
        })
        .map(|form| {
            let kind = form
                .fields
                .iter()
                .find_map(|(_, ty)| {
                    if let FieldType::SelectedFrame(_, kind) = ty { Some(kind) } else { None }
                })
                .unwrap();
            let rule = form.rule;
            quote!((#rule, #kind))
        })
        .collect();
    let canonical_frames = ir.frames.iter().map(|(_, frame)| frame_tokens(frame));
    let required = !templates.is_empty() || ir.rules.iter().any(|rule| rule.segments.is_some());
    let static_count = ir.rules.len();
    let categories = ir
        .frame_categories
        .iter()
        .map(|(source, index)| {
            let name = &ir.categories[*index];
            let text = source.to_string();
            quote!(#text => Some(Category::#name))
        })
        .chain(
            ir.categories
                .iter()
                .take(ir.public_categories)
                .filter(|name| {
                    !ir.frame_categories
                        .iter()
                        .any(|(source, _)| source == *name)
                })
                .map(|name| {
                    let text = name.to_string();
                    quote!(#text => Some(Category::#name))
                }),
        );
    quote! {
        const SELECTED_FRAME_TEMPLATES: &[(usize, &str)] = &[#(#templates),*];
        const REQUIRES_ENVIRONMENT: bool = #required;
        impl Default for Grammar {
            fn default() -> Self { Self { rules: vec![#(#rules),*], productions: vec![#(#productions),*], programs: vec![], canonical_frames: vec![#(#canonical_frames),*], marker_classes: vec![], expansions: vec![], unsupported: vec![], static_count: #static_count, compiled: false } }
        }
        impl Grammar {
            fn frame_category(name: &str) -> Option<Category> { match name { #(#categories,)* _ => None } }
            fn for_admission() -> &'static Self {
                static GRAMMAR: ::std::sync::OnceLock<Grammar> = ::std::sync::OnceLock::new();
                GRAMMAR.get_or_init(Self::default)
            }
        }
    }
}

fn projection(ir: &Ir) -> TokenStream {
    let features = ir
        .features
        .iter()
        .enumerate()
        .filter_map(|(index, domain)| match &domain.kind {
            DomainKind::Builtin(ty) if domain.name != "form" => {
                let field = format_ident!("{}", domain.name);
                let ty = format_ident!("{ty}");
                Some(quote!(base.values[#index] = features.#field.map(FeatureValue::#ty);))
            }
            DomainKind::Custom { default } => {
                let name = &domain.name;
                let values = domain.values.iter().enumerate().map(
                    |(value, name)| quote!(#name => Some(FeatureValue::Custom(#index, #value))),
                );
                let absent = default.map_or_else(
                    || quote!(None),
                    |value| quote!(Some(FeatureValue::Custom(#index, #value))),
                );
                Some(
                    quote!(base.values[#index] = match properties.features.get(#name) {
                Some(value) => match value.as_str() { #(#values,)* _ => None },
                None => #absent,
            };),
                )
            }
            _ => None,
        });
    let frames = ir.frames.iter().map(|(_, frame)| frame_tokens(frame));
    quote! {
        fn project(features: ::deckmaste_english_v3::LexicalFeatures<'_>) -> Vec<Summary> {
            let (form, features, properties, surface) = match features {
                ::deckmaste_english_v3::LexicalFeatures::Word { form, features, properties, surface, .. } => (form, features, properties, surface),
                ::deckmaste_english_v3::LexicalFeatures::Numeral { value, notation, surface } => return project_numeral(value, notation, surface),
                ::deckmaste_english_v3::LexicalFeatures::FlavorWord { surface } => return vec![Summary::lexical(surface)],
            };
            let mut base = Summary::lexical(surface);
            #(#features)*
            base.values[5] = Some(FeatureValue::WordForm(form));
            base.values[10] = Some(FeatureValue::Framing(!properties.frames.is_empty()));
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

        fn project_numeral(value: i32, notation: ::deckmaste_lexical::Numeral, surface: ::deckmaste_lexical::SurfaceFeatures) -> Vec<Summary> {
            use ::deckmaste_lexical::Numeral;
            let mut summary = Summary::lexical(surface);
            let kind = match notation {
                Numeral::Cardinal => 0,
                Numeral::Ordinal => 1,
                Numeral::Arabic(false) => 2,
                Numeral::Arabic(true) => 3,
                Numeral::Roman => 4,
            };
            summary.values[8] = Some(FeatureValue::NumeralKind(kind));
            summary.values[9] = Some(FeatureValue::NumeralSize(value.unsigned_abs() >= 1000));
            summary.values[11] = Some(FeatureValue::NumeralSign(value.is_negative()));
            if matches!(notation, Numeral::Cardinal | Numeral::Arabic(_)) {
                summary.values[0] = Some(FeatureValue::Number(if value.unsigned_abs() == 1 {
                    ::deckmaste_lexical::Number::Singular
                } else {
                    ::deckmaste_lexical::Number::Plural
                }));
            }
            vec![summary]
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

fn form_pieces(
    form: &crate::ir::Form,
    bindings: &[syn::Ident],
    owner: &str,
) -> [Vec<TokenStream>; 4] {
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
                let ty = &form.fields[*field].1;
                if matches!(ty, FieldType::SelectedFrame(_, _)) {
                    visit.push(quote!(for value in #binding { value.visit(visitor)?; }));
                    word.push(quote!(for value in #binding { value.visit_words(visitor)?; }));
                    continue;
                }
                let category_name = match ty {
                    FieldType::Lexical(c)
                    | FieldType::One(c)
                    | FieldType::Optional(c)
                    | FieldType::Repeated(c, _) => format_ident!("{c}"),
                    FieldType::SelectedFrame(_, _) => unreachable!(),
                };
                let check_category = quote!(if child.category() != Category::#category_name { return Err(Error::Invalid(#owner, "constituent Category")); });
                match ty {
                    FieldType::SelectedFrame(_, _) => unreachable!(),
                    FieldType::Lexical(_) => {
                        summaries.push(quote!(Some(#binding.admit(grammar, lexicon, ::deckmaste_lexical::Category::#category_name)?)));
                        write.push(quote!(output.word(#binding, lexicon)?;));
                        word.push(quote!(visitor(#binding);));
                    }
                    FieldType::One(_) => {
                        summaries.push(quote!(Some({ let child = #binding; #check_category child.admit_with(grammar, lexicon)? })));
                        write.push(quote!(#binding.write(grammar, lexicon, output)?;));
                        visit.push(quote!(#binding.visit(visitor)?;));
                        word.push(quote!(#binding.visit_words(visitor)?;));
                    }
                    FieldType::Optional(_) => {
                        summaries.push(quote!(Some({ if let Some(child) = #binding { #check_category child.admit_with(grammar, lexicon)? } else { Summary::default() } })));
                        write.push(quote!(if let Some(child) = #binding { child.write(grammar, lexicon, output)?; }));
                        visit
                            .push(quote!(if let Some(child) = #binding { child.visit(visitor)?; }));
                        word.push(
                            quote!(if let Some(child) = #binding { child.visit_words(visitor)?; }),
                        );
                    }
                    FieldType::Repeated(_, separator) => {
                        summaries.push(quote!(Some({ let mut result = Summary::default(); for child in #binding { #check_category let summary = child.admit_with(grammar, lexicon)?; result.surface = result.surface.append(summary.surface); } result.values[12] = result.surface.onset.map(FeatureValue::Onset); result })));
                        write.push(quote!(for (index, child) in #binding.iter().enumerate() { if index != 0 { output.push_str(#separator); } child.write(grammar, lexicon, output)?; }));
                        visit.push(quote!(for child in #binding { child.visit(visitor)?; }));
                        word.push(quote!(for child in #binding { child.visit_words(visitor)?; }));
                    }
                }
            }
        }
    }
    [summaries, write, visit, word]
}

fn form_admission(
    form: &crate::ir::Form,
    bindings: &[syn::Ident],
    owner: &str,
    summaries: &[TokenStream],
    write: &mut Vec<TokenStream>,
) -> TokenStream {
    let selected = form
        .fields
        .iter()
        .position(|(_, ty)| matches!(ty, FieldType::SelectedFrame(_, _)));
    let rule = form.rule;
    if let Some(field) = selected {
        let binding = &bindings[field];
        let binding_head = &bindings[0];
        let head = match &form.fields[0].1 {
            FieldType::Lexical(_) => quote!(FrameHead::Lexical(#binding_head)),
            FieldType::One(_) => quote!(FrameHead::Constituent(#binding_head)),
            _ => unreachable!(),
        };
        *write = vec![quote!(grammar.write_selected(#rule, #head, #binding, lexicon, output)?;)];
        quote!(grammar.admit_selected(#rule, #head, #binding, lexicon))
    } else {
        quote! {
            let summaries: Vec<Option<Summary>> = vec![#(#summaries),*];
            let rule = &grammar.rules[#rule];
            let mut state = State::new(rule.initial.clone());
            for (dot, summary) in summaries.iter().enumerate() {
                state = rule.advance(dot, &state, summary.as_ref()).ok_or(Error::Invalid(#owner, "feature admission"))?;
            }
            rule.complete(&state).ok_or(Error::Invalid(#owner, "feature export"))
        }
    }
}

fn ast(ir: &Ir) -> TokenStream {
    let mut variants = vec![];
    let mut categories = vec![];
    let mut constructions = vec![];
    let mut admits = vec![];
    let mut node_methods = vec![];
    let mut writes = vec![];
    let mut visits = vec![];
    let mut words = vec![];
    for (constructor_index, constructor) in ir.constructors.iter().enumerate() {
        let name = &constructor.name;
        let owner = name.to_string();
        let category = &ir.categories[constructor.category];
        let field_names: Vec<_> = constructor.fields.iter().map(|(name, _)| name).collect();
        let bindings: Vec<_> = (0..field_names.len())
            .map(|i| format_ident!("__field{i}"))
            .collect();
        let types = constructor.fields.iter().map(|(_, ty)| match ty {
            FieldType::Lexical(_) => quote!(Word),
            FieldType::SelectedFrame(_, _) => quote!(Vec<FrameValue>),
            FieldType::One(_) => quote!(Box<Reading>),
            FieldType::Optional(_) => quote!(Option<Box<Reading>>),
            FieldType::Repeated(_, _) => quote!(Vec<Reading>),
        });
        let category_field = if constructor.shared {
            quote!(category: Category,)
        } else {
            TokenStream::new()
        };
        variants.push(quote!(#name { form: usize, #category_field #(#field_names: #types),* }));
        categories.push(if constructor.shared {
            quote!(Self::#name { category, .. } => *category)
        } else {
            quote!(Self::#name { .. } => Category::#category)
        });
        constructions.push(quote!(Self::#name { .. } => #owner));
        let category_bind = if constructor.shared { quote!(category,) } else { TokenStream::new() };
        let pattern = quote!(Self::#name { #category_bind form, #(#field_names: #bindings),* });
        let bind = if ir.constructors.len() == 1 {
            quote!(let #pattern = self;)
        } else {
            quote!(let #pattern = self else { return Err(Error::Internal); };)
        };
        let mut admit_forms = vec![];
        let mut write_forms = vec![];
        let mut visit_forms = vec![];
        let mut word_forms = vec![];
        let mut surface_forms = std::collections::BTreeSet::new();
        for form in &constructor.forms {
            let index = form.index;
            let category_name = &ir.categories[form.category];
            let admit_key = if constructor.shared {
                quote!((Category::#category_name, #index))
            } else {
                quote!(#index)
            };
            let [summaries, mut write, visit, word] = form_pieces(form, &bindings, &owner);
            let admission = form_admission(form, &bindings, &owner, &summaries, &mut write);
            if constructor.shared {
                let method = format_ident!(
                    "__schema_admit_{}_{}_{}",
                    constructor_index,
                    form.category,
                    index
                );
                node_methods.push(quote! {
                    #[inline(never)]
                    fn #method(&self, grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon) -> Result<Summary, Error> {
                        #bind
                        #admission
                    }
                });
                admit_forms.push(quote!(#admit_key => self.#method(grammar, lexicon)));
            } else {
                admit_forms.push(quote!(#admit_key => { #admission }));
            }
            if surface_forms.insert(index) {
                write_forms.push(quote!(#index => { #(#write)* Ok(()) }));
                visit_forms.push(quote!(#index => { #(#visit)* Ok(()) }));
                word_forms.push(quote!(#index => { #(#word)* Ok(()) }));
            }
        }
        let ([admit, write, visit, word], methods) = emit_node_methods(
            constructor_index,
            name,
            &bind,
            &owner,
            [admit_forms, write_forms, visit_forms, word_forms],
            constructor.shared,
        );
        admits.push(admit);
        writes.push(write);
        visits.push(visit);
        words.push(word);
        node_methods.extend(methods);
    }
    let cost_methods = emit_cost_methods(ir);
    let structural_traits = emit_structural_traits(ir);
    quote! {
        #[derive(Debug)]
        pub enum Reading { #(#variants),* }
        #structural_traits
        impl Reading {
            #[must_use]
            pub fn category(&self) -> Category { match self { #(#categories),* } }
            /// The declaration that constructs this node.
            #[must_use]
            pub fn construction(&self) -> &'static str { match self { #(#constructions),* } }
            #cost_methods
            /// Check a constructed value directly against declaration admission.
            /// # Errors
            /// Reports an invalid form, lexical value, constituent or feature equation.
            pub fn admit(&self, lexicon: &::deckmaste_lexical::Lexicon) -> Result<Summary, Error> {
                if REQUIRES_ENVIRONMENT { return GrammarEnvironment::new(lexicon).admit(self); }
                let grammar = Grammar::for_admission();
                let summary = self.admit_with(grammar, lexicon)?;
                let mut output = Realization::new();
                self.write(grammar, lexicon, &mut output)?;
                output.check_context(lexicon)?;
                Ok(summary)
            }
            pub fn admit_with(&self, grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon) -> Result<Summary, Error> {
                if REQUIRES_ENVIRONMENT && !grammar.compiled { return Err(Error::EnvironmentRequired); }
                match self { #(#admits),* }
            }
            #(#node_methods)*
            /// Realize a valid value using the lexical environment and declared surfaces.
            /// # Errors
            /// Reports the same admission errors as `admit`.
            pub fn realize_with(&self, grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon) -> Result<String, Error> {
                grammar.realize(self, lexicon)
            }
            pub fn realize(&self, lexicon: &::deckmaste_lexical::Lexicon) -> Result<String, Error> {
                if REQUIRES_ENVIRONMENT { return GrammarEnvironment::new(lexicon).realize(self); }
                let grammar = Grammar::for_admission();
                self.admit_with(grammar, lexicon)?;
                let mut output = Realization::new();
                self.write(grammar, lexicon, &mut output)?;
                output.check_context(lexicon)?;
                Ok(output.finish())
            }
            fn write(&self, grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon, output: &mut Realization) -> Result<(), Error> { match self { #(#writes),* } }
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

fn emit_cost_methods(ir: &Ir) -> TokenStream {
    let costs = ir.constructors.iter().map(|constructor| {
        let name = &constructor.name;
        let cost = constructor.cost;
        quote!(Self::#name { .. } => #cost)
    });
    quote! {
            /// The declared preference cost of this construction.
            #[must_use]
            pub fn local_cost(&self) -> u64 { match self { #(#costs),* } }
            /// Sum construction costs throughout this reading; lexical leaves cost zero.
            /// # Errors
            /// Reports an invalid surface alternative or a cost exceeding `u64`.
            pub fn total_cost(&self) -> Result<u64, Error> {
                let mut total = Some(0_u64);
                self.visit(&mut |node| {
                    total = total.and_then(|value| value.checked_add(node.local_cost()));
                })?;
                total.ok_or(Error::CostOverflow)
            }
    }
}

// Keep recursive structural operations proportional to the selected variant.
// Variant and field order exactly match Rust's derived structural ordering.
fn emit_structural_traits(ir: &Ir) -> TokenStream {
    let mut ranks = vec![];
    let mut clones = vec![];
    let mut comparisons = vec![];
    let mut methods = vec![];
    for (index, constructor) in ir.constructors.iter().enumerate() {
        let name = &constructor.name;
        let mut fields: Vec<_> = constructor
            .fields
            .iter()
            .map(|(name, _)| name.clone())
            .collect();
        if constructor.shared {
            fields.insert(0, format_ident!("category"));
        }
        let left: Vec<_> = (0..fields.len())
            .map(|i| format_ident!("__left{i}"))
            .collect();
        let right: Vec<_> = (0..fields.len())
            .map(|i| format_ident!("__right{i}"))
            .collect();
        let clone_method = format_ident!("__clone{index}");
        let compare_method = format_ident!("__compare{index}");
        ranks.push(quote!(Self::#name { .. } => #index));
        clones.push(quote!(Self::#name { .. } => self.#clone_method()));
        comparisons.push(quote!(Self::#name { .. } => self.#compare_method(other)));
        let fallback = if ir.constructors.len() == 1 {
            TokenStream::new()
        } else {
            quote!(else { unreachable!() })
        };
        methods.push(quote! {
            #[inline(never)]
            fn #clone_method(&self) -> Self {
                let Self::#name { form, #(#fields),* } = self #fallback;
                Self::#name { form: *form, #(#fields: #fields.clone()),* }
            }
            #[inline(never)]
            fn #compare_method(&self, other: &Self) -> ::core::cmp::Ordering {
                let Self::#name { form: left_form, #(#fields: #left),* } = self #fallback;
                let Self::#name { form: right_form, #(#fields: #right),* } = other #fallback;
                let ordering = left_form.cmp(right_form);
                if ordering != ::core::cmp::Ordering::Equal { return ordering; }
                #(
                    let ordering = #left.cmp(#right);
                    if ordering != ::core::cmp::Ordering::Equal { return ordering; }
                )*
                ::core::cmp::Ordering::Equal
            }
        });
    }
    quote! {
        impl Reading {
            fn __variant_rank(&self) -> usize { match self { #(#ranks),* } }
            #(#methods)*
        }
        impl ::core::clone::Clone for Reading {
            fn clone(&self) -> Self { match self { #(#clones),* } }
        }
        impl ::core::cmp::Ord for Reading {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                let ordering = self.__variant_rank().cmp(&other.__variant_rank());
                if ordering != ::core::cmp::Ordering::Equal { return ordering; }
                match self { #(#comparisons),* }
            }
        }
        impl ::core::cmp::PartialOrd for Reading {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> { Some(self.cmp(other)) }
        }
        impl ::core::cmp::PartialEq for Reading {
            fn eq(&self, other: &Self) -> bool { self.cmp(other) == ::core::cmp::Ordering::Equal }
        }
        impl ::core::cmp::Eq for Reading {}
    }
}

fn emit_node_methods(
    constructor: usize,
    name: &syn::Ident,
    bind: &TokenStream,
    owner: &str,
    forms: [Vec<TokenStream>; 4],
    shared: bool,
) -> ([TokenStream; 4], Vec<TokenStream>) {
    let operations = [
        (
            "admit",
            quote!(grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon),
            quote!(grammar, lexicon),
            quote!(Summary),
        ),
        (
            "write",
            quote!(grammar: &Grammar, lexicon: &::deckmaste_lexical::Lexicon, output: &mut Realization),
            quote!(grammar, lexicon, output),
            quote!(()),
        ),
        (
            "visit",
            quote!(visitor: &mut impl FnMut(&Self)),
            quote!(visitor),
            quote!(()),
        ),
        (
            "words",
            quote!(visitor: &mut impl FnMut(&Word)),
            quote!(visitor),
            quote!(()),
        ),
    ];
    let mut dispatch = std::array::from_fn(|_| TokenStream::new());
    let mut methods = Vec::new();
    for (index, ((prefix, parameters, arguments, result), forms)) in
        operations.into_iter().zip(forms).enumerate()
    {
        let method = format_ident!("__{prefix}{constructor}");
        dispatch[index] = quote!(Self::#name { .. } => self.#method(#arguments));
        // Keep recursive frames proportional to the selected construction,
        // rather than reserving debug-build temporaries for the entire grammar.
        let selector =
            if shared && index == 0 { quote!((*category, *form)) } else { quote!(*form) };
        methods.push(quote! {
            #[inline(never)]
            fn #method(&self, #parameters) -> Result<#result, Error> {
                #bind
                match #selector { #(#forms,)* _ => Err(Error::Invalid(#owner, "surface alternative")) }
            }
        });
    }
    (dispatch, methods)
}

fn materializer(ir: &Ir) -> TokenStream {
    let mut methods = Vec::new();
    let cases: Vec<_> = ir.rules.iter().enumerate().filter(|(_, rule)| {
        !matches!(&rule.build, Build::Construction { constructor, .. } if ir.constructors[*constructor].fields.iter().any(|(_, ty)| matches!(ty, FieldType::SelectedFrame(_, _))))
    }).map(|(index, rule)| {
        let arity = rule.symbols.len();
        let body = match &rule.build {
            Build::Construction { constructor, form, slots } => {
                let constructor = &ir.constructors[*constructor];
                let name = &constructor.name;
                let result_category = &ir.categories[rule.category];
                let category_value = if constructor.shared { quote!(category: Category::#result_category,) } else { TokenStream::new() };
                let fields: Vec<_> = constructor.fields.iter().map(|(name, _)| name).collect();
                let values: Vec<_> = (0..fields.len()).map(|i| format_ident!("__field{i}")).collect();
                let consume = rule.symbols.iter().enumerate().map(|(dot, _)| {
                    if let Some(field) = slots.iter().position(|s| *s == dot) {
                        let binding = &values[field];
                        let conversion = match constructor.fields[field].1 {
                            FieldType::Lexical(_) => quote!(value.word()?),
                            FieldType::SelectedFrame(_, _) => quote!(return Err(Error::Internal)),
                            FieldType::One(_) => quote!(Box::new(value.node()?)),
                            FieldType::Optional(_) => quote!(value.optional()?),
                            FieldType::Repeated(_, _) => quote!(value.repeated()?),
                        };
                        quote!(let value = children.next().ok_or(Error::Internal)?; let #binding = #conversion;)
                    } else { quote!(if !matches!(children.next(), Some(Value::Literal)) { return Err(Error::Internal); }) }
                });
                quote!(#(#consume)* Ok(Value::Reading(Reading::#name { #category_value form: #form, #(#fields: #values),* })))
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
        let method = format_ident!("__materialize{index}");
        // Debug builds otherwise reserve temporaries for every production in
        // one frame, making stack use grow with the entire grammar.
        methods.push(quote! {
            #[inline(never)]
            fn #method(&self, children: Vec<Value>) -> Result<Value, Error> {
                if children.len() != #arity { return Err(Error::Internal); }
                let mut children = children.into_iter();
                #body
            }
        });
        quote!(#index => self.#method(children))
    }).collect();
    let selected_builds = ir.constructors.iter().flat_map(|constructor| {
        constructor.forms.iter().filter(|form| form.fields.iter().any(|(_, ty)| matches!(ty, FieldType::SelectedFrame(_, _)))).map(|form| {
            let rule = form.rule;
            let name = &constructor.name;
            let head = &constructor.fields[0].0;
            let tail = &constructor.fields[1].0;
            let index = form.index;
            let category = &ir.categories[form.category];
            let category = if constructor.shared { quote!(category: Category::#category,) } else { TokenStream::new() };
            let selector = match &form.fields[0].1 {
                FieldType::Lexical(_) => quote!(head.word()?),
                FieldType::One(_) => quote!(Box::new(head.node()?)),
                _ => unreachable!(),
            };
            quote!(#rule => Ok(Value::Reading(Reading::#name { form: #index, #category #head: #selector, #tail: values })))
        })
    });
    quote!(impl Grammar {
        fn build_selected(&self, template: usize, head: Value, values: Vec<FrameValue>) -> Result<Value, Error> {
            match template { #(#selected_builds,)* _ => Err(Error::Internal) }
        }
        fn materialize(&self, production: usize, children: Vec<Value>) -> Result<Value, Error> {
            match production { #(#cases,)* _ => self.materialize_selected(production, children) }
        }
        #(#methods)*
    })
}
