use crate::plan::{DeclarationKey, GeneratedItem, ItemKey, NamedKind, SourceDeclarationKind};
use crate::semantic::SemanticPlan;
use quote::{format_ident, quote};

pub(crate) fn emit(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    plan.frame_families
        .iter()
        .map(|family| emit_family(plan, family))
        .collect()
}

#[expect(
    clippy::too_many_lines,
    reason = "the generated frame family keeps its typed schema, rules and checked builders together"
)]
fn emit_family(
    plan: &SemanticPlan,
    family: &crate::frame_family::FrameFamilyPlan,
) -> GeneratedItem {
    let family_name = &family.name;
    let module = format_ident!(
        "{}",
        crate::identifier::snake_case(&family_name.to_string())
    );
    let cats = &family.categories;
    let cat_names = cats.iter().map(ToString::to_string).collect::<Vec<_>>();
    let mut child_variants = Vec::new();
    let mut child_build = Vec::new();
    let mut child_render = Vec::new();
    let mut child_walk = Vec::new();
    for cat in cats {
        let name = cat.to_string();
        let fields = [
            ("ConcordClass", plan.category_carries_concord_class(&name)),
            (
                "InflectionalForm",
                plan.carries_feature(&name, crate::feature::Feature::InflectionalForm),
            ),
            ("Cardinality", plan.category_carries_cardinality(&name)),
            ("Number", plan.category_carries_number(&name)),
            (
                "DeterminerNumber",
                plan.carries_feature(&name, crate::feature::Feature::DeterminerNumber),
            ),
            (
                "FusedHeadLicense",
                plan.carries_feature(&name, crate::feature::Feature::FusedHeadLicense),
            ),
            (
                "NominalLicense",
                plan.carries_feature(&name, crate::feature::Feature::NominalLicense),
            ),
            ("Onset", plan.category_carries_onset(&name)),
            (
                "PossessiveEnding",
                plan.category_carries_possessive_ending(&name),
            ),
        ]
        .into_iter()
        .filter(|(_, carry)| *carry)
        .map(|(ty, _)| format_ident!("{ty}"))
        .collect::<Vec<_>>();
        let bindings = fields
            .iter()
            .map(|ty| format_ident!("{}", crate::identifier::snake_case(&ty.to_string())))
            .collect::<Vec<_>>();
        child_variants.push(quote! { #cat(super::#cat, #(super::#fields,)* super::FeatureConstraint<super::Onset>) });
        child_build.push(quote! { super::BuildValue::#cat(value, #(#bindings,)* following_onset) => Some(Self::#cat(value.clone(), #(*#bindings,)* *following_onset)), });
        let walker = format_ident!("walk_{}", crate::identifier::snake_case(&name));
        child_walk.push(quote! { Self::#cat(value, ..) => super::#walker(visitor, value), });
        let capability = plan.category_render_capability(&name);
        let root = plan
            .roots()
            .iter()
            .find(|root| root.category() == name && root.is_render_entry());
        let renderer = format_ident!(
            "{}",
            crate::identifier::category_renderer(&name, root.is_some())
        );
        let context = capability.requires_context().then(|| quote! { , context });
        let environment = plan
            .needs_parser_environment()
            .then(|| quote! { , environment });
        let concord = capability
            .requires_external_concord_class()
            .then(|| quote! { , *concord_class });
        let call = quote! { super::#renderer(writer, value #concord #context #environment); };
        child_render.push(quote! { Self::#cat(value, #(#bindings,)* _) => { #call } });
    }
    let mut markers = Vec::new();
    let mut marker_resolve = Vec::new();
    let mut marker_surface = Vec::new();
    let mut marker_owner = Vec::new();
    let mut marker_walk = Vec::new();
    for vocab in plan.runtime_vocabs() {
        let vocabulary = vocab.name_ident();
        let vocabulary_name = vocab.name();
        let walker = format_ident!("walk_{}", crate::identifier::snake_case(vocabulary_name));
        for member in vocab.variants() {
            let variant = syn::Index::from(markers.len());
            let member_ident = member.name();
            let member_name = member_ident.to_string();
            let word = member.word();
            let owner = format!("vocab:{vocabulary_name}/{member_name}");
            markers.push((vocabulary.to_string(), member_ident.to_string()));
            marker_resolve
                .push(quote! { (#vocabulary_name, #member_name) => Some(Self(#variant)), });
            marker_surface.push(quote! { #variant => #word, });
            marker_owner.push(quote! { #variant => #owner, });
            marker_walk.push(
                quote! { #variant => super::#walker(visitor, super::#vocabulary::#member_ident), },
            );
        }
    }
    let coordinators = family
        .coordinators
        .iter()
        .map(|(v, m)| {
            syn::Index::from(
                markers
                    .iter()
                    .position(|(vocabulary, member)| {
                        vocabulary == &v.to_string() && member == &m.to_string()
                    })
                    .expect("sealed coordinator belongs to lexical inventory"),
            )
        })
        .collect::<Vec<_>>();
    let roles = family
        .roles
        .iter()
        .map(|(role, relation, category)| {
            let name = role.to_string();
            quote! { #name => Some((FrameRelation::#relation, ChildCategory::#category)), }
        })
        .collect::<Vec<_>>();
    let tokens = quote! {
        pub mod #module {
            use ::deckmaste_construction_core::frame::{FrameRegistry, FrameSchemaError, ResolvedFrameItem, SchemaId};
            use ::deckmaste_construction_core::macro_def::{FrameRelation, SurfaceFeature, VerbFrame};
            use super::{Rule, RulePosition, LexicalOwner, LexicalProvenanceKind, LexicalBoundary, LexicalMatch, ScanInput, ParseContext, Writer};
            use crate::environment::{ParserEnvironment, VerbInventoryRef};

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum ChildCategory { #(#cats),* }
            impl ChildCategory {
                fn resolve(name: &str) -> Option<Self> { match name { #(#cat_names => Some(Self::#cats),)* _ => None } }
                fn grammar(self) -> super::Category { match self { #(Self::#cats => super::Category::#cats,)* } }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum Child { #(#child_variants),* }
            impl Child {
                pub fn from_build(value: &super::BuildValue) -> Option<Self> { match value { #(#child_build)* _ => None } }
                pub fn category(&self) -> ChildCategory { match self { #(Self::#cats(..) => ChildCategory::#cats,)* } }
                fn render(&self, writer: &mut Writer<'_>, context: &ParseContext<'_>, environment: &ParserEnvironment) {
                    match self { #(#child_render)* }
                }
                fn walk<V: super::Visitor + ?Sized>(&self, visitor: &mut V) { match self { #(#child_walk)* } }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct Marker(usize);
            impl Marker {
                pub fn resolve(vocabulary: &str, member: &str) -> Option<Self> { match (vocabulary, member) { #(#marker_resolve)* _ => None } }
                fn surface(self) -> &'static str { match self.0 { #(#marker_surface)* _ => unreachable!("a Marker is resolved from the sealed inventory"), } }
                fn owner(self) -> LexicalOwner {
                    LexicalOwner::static_owner(LexicalProvenanceKind::Vocab, match self.0 { #(#marker_owner)* _ => unreachable!("a Marker is resolved from the sealed inventory"), })
                }
                fn render(self, writer: &mut Writer<'_>) { writer.claim(|| self.owner(), |writer| writer.word(self.surface())); }
                fn walk<V: super::Visitor + ?Sized>(self, visitor: &mut V) { match self.0 { #(#marker_walk)* _ => unreachable!("a Marker is resolved from the sealed inventory"), } }
            }
            const COORDINATORS: &[Marker] = &[#(Marker(#coordinators)),*];

            #[derive(Debug, Clone)]
            pub struct Lexeme {
                pub reference: VerbInventoryRef,
                pub forms: Vec<(SurfaceFeature, String)>,
                pub frames: Vec<VerbFrame>,
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Head {
                reference: VerbInventoryRef,
                feature: SurfaceFeature,
                row: usize,
            }
            impl Head {
                pub fn reference(&self) -> &VerbInventoryRef { &self.reference }
                pub fn feature(&self) -> SurfaceFeature { self.feature }
                fn owner(&self) -> LexicalOwner {
                    match &self.reference {
                        VerbInventoryRef::Core(identity) => LexicalOwner::static_owner(LexicalProvenanceKind::Lexeme, identity.owner_id()),
                        VerbInventoryRef::Declaration(identity) => LexicalOwner::declaration_owner(identity.clone(), self.feature),
                    }
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Member {
                pub position: usize,
                pub relation: Option<FrameRelation>,
                pub marker: Option<Marker>,
                pub child: Option<Child>,
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum Part {
                Member(Member),
                Coordination { start: usize, end: usize, conjuncts: Vec<Vec<Member>>, coordinator: Marker },
            }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #family_name { head: Head, schema: SchemaId, parts: Vec<Part> }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum Rejection { UnknownSchema, WrongHead, WrongMember, MissingRequired, EmptyConjunct, WrongChildren }
            impl #family_name {
                pub fn try_new(grammar: &PreparedGrammar, head: Head, schema: SchemaId, parts: Vec<Part>) -> Result<Self, Rejection> {
                    Self::check_parts(grammar, &head, schema, &parts)?;
                    Ok(Self { head, schema, parts })
                }
                fn check_parts(grammar: &PreparedGrammar, head: &Head, schema: SchemaId, parts: &[Part]) -> Result<(), Rejection> {
                    let items = grammar.registry.schemas().get(schema.0).ok_or(Rejection::UnknownSchema)?;
                    let row = grammar.heads.get(head.row).ok_or(Rejection::WrongHead)?;
                    if &row.head != head || !row.schemas.contains(&schema) { return Err(Rejection::WrongHead); }
                    let mut position = 0;
                    for part in parts {
                        match part {
                            Part::Member(member) => {
                                Self::skip(items, position, member.position)?;
                                Self::check_member(items, member)?;
                                position = member.position + 1;
                            }
                            Part::Coordination { start, end, conjuncts, coordinator } => {
                                Self::skip(items, position, *start)?;
                                if *start >= *end || *end > items.len() || conjuncts.len() < 2 || !COORDINATORS.contains(coordinator) { return Err(Rejection::WrongMember); }
                                for conjunct in conjuncts { Self::check_interval(items, *start, *end, conjunct)?; }
                                position = *end;
                            }
                        }
                    }
                    Self::skip(items, position, items.len())
                }
                fn skip(items: &[ResolvedFrameItem<ChildCategory, Marker>], start: usize, end: usize) -> Result<(), Rejection> {
                    if start > end || end > items.len() { return Err(Rejection::WrongMember); }
                    if items[start..end].iter().any(|item| !item.optional) { return Err(Rejection::MissingRequired); }
                    Ok(())
                }
                fn check_member(items: &[ResolvedFrameItem<ChildCategory, Marker>], member: &Member) -> Result<(), Rejection> {
                    let expected = items.get(member.position).ok_or(Rejection::WrongMember)?;
                    if expected.marker != member.marker || expected.complement != member.child.as_ref().map(|child| (member.relation.unwrap_or(FrameRelation::Complement), child.category())) || expected.complement.map(|c| c.0) != member.relation { return Err(Rejection::WrongMember); }
                    Ok(())
                }
                fn check_interval(items: &[ResolvedFrameItem<ChildCategory, Marker>], start: usize, end: usize, members: &[Member]) -> Result<(), Rejection> {
                    if members.is_empty() { return Err(Rejection::EmptyConjunct); }
                    let mut position = start;
                    for member in members {
                        if member.position >= end { return Err(Rejection::WrongMember); }
                        Self::skip(items, position, member.position)?;
                        Self::check_member(items, member)?;
                        position = member.position + 1;
                    }
                    Self::skip(items, position, end)
                }
                pub fn head(&self) -> &Head { &self.head }
                pub fn schema(&self) -> SchemaId { self.schema }
                pub fn parts(&self) -> &[Part] { &self.parts }
                pub fn write(&self, grammar: &PreparedGrammar, writer: &mut Writer<'_>, context: &ParseContext<'_>, environment: &ParserEnvironment) -> Result<(), Rejection> {
                    Self::check_parts(grammar, &self.head, self.schema, &self.parts)?;
                    let row = grammar.heads.get(self.head.row).ok_or(Rejection::WrongHead)?;
                    if row.head != self.head || !row.schemas.contains(&self.schema) { return Err(Rejection::WrongHead); }
                    writer.claim(|| self.head.owner(), |writer| writer.word(&row.surface));
                    for part in &self.parts {
                        match part {
                            Part::Member(member) => member.render(writer, context, environment),
                            Part::Coordination { conjuncts, coordinator, .. } => {
                                for (index, conjunct) in conjuncts.iter().enumerate() {
                                    if index > 0 { coordinator.render(writer); }
                                    for member in conjunct { member.render(writer, context, environment); }
                                }
                            }
                        }
                    }
                    Ok(())
                }
                pub fn walk<V: FrameVisitor + ?Sized>(&self, visitor: &mut V) {
                    visitor.visit_frame(self);
                    visitor.visit_head(&self.head);
                    for part in &self.parts {
                        match part {
                            Part::Member(member) => member.walk(visitor),
                            Part::Coordination { conjuncts, coordinator, .. } => {
                                for (index, conjunct) in conjuncts.iter().enumerate() {
                                    if index > 0 { coordinator.walk(visitor); }
                                    for member in conjunct { member.walk(visitor); }
                                }
                            }
                        }
                    }
                }
            }
            impl Member {
                fn render(&self, writer: &mut Writer<'_>, context: &ParseContext<'_>, environment: &ParserEnvironment) {
                    if let Some(marker) = self.marker { marker.render(writer); }
                    if let Some(child) = &self.child { child.render(writer, context, environment); }
                }
                fn walk<V: super::Visitor + ?Sized>(&self, visitor: &mut V) {
                    if let Some(marker) = self.marker { marker.walk(visitor); }
                    if let Some(child) = &self.child { child.walk(visitor); }
                }
            }
            pub trait FrameVisitor: super::Visitor {
                fn visit_frame(&mut self, _frame: &#family_name) {}
                fn visit_head(&mut self, _head: &Head) {}
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Category {
                Static(super::Category), Suffix(SchemaId, usize),
                Interval(SchemaId, usize, usize), Coordinate(SchemaId, usize, usize, usize),
                Phrase, Root,
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Terminal { Static(super::LexicalTerminal), Head(usize), Marker(Marker) }
            impl Terminal {
                pub fn position_before(self, position: super::ScanPosition) -> super::ScanPosition {
                    match self { Self::Static(terminal) => terminal.position_before(position), _ => position }
                }
                pub fn position_after(self, position: super::ScanPosition, end: usize) -> super::ScanPosition {
                    match self {
                        Self::Static(terminal) => terminal.position_after(position, end),
                        _ => super::ScanPosition { byte_offset: end, case: super::CasePosition::Continuation, prefix: super::PrefixPosition::WordOwnedSpace },
                    }
                }
                pub fn suppress_right_boundary(self) -> Self { match self { Self::Static(terminal) => Self::Static(terminal.suppress_right_boundary()), other => other } }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum RuleId { Static(super::RuleId), Frame(usize), Root }
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum Value {
                Static(super::BuildValue), Head(Head), Marker(Marker),
                Sequence(Vec<Part>), Interval(Vec<Member>), Coordination(Part), Phrase(#family_name),
            }
            #[derive(Debug, Clone)]
            struct HeadRow { head: Head, surface: String, schemas: Vec<SchemaId> }
            #[derive(Debug, Clone, Copy)]
            enum Action {
                End(bool), Skip,
                Item(SchemaId, usize, bool),
                Coordinate(SchemaId, usize, usize, usize, bool),
                AppendCoordination, Phrase(SchemaId),
            }
            struct Production {
                lhs: Category,
                rhs: Vec<RulePosition<Category, Terminal>>,
                action: Action,
            }
            pub struct PreparedGrammar {
                registry: FrameRegistry<ChildCategory, Marker>,
                heads: Vec<HeadRow>,
                productions: Vec<Production>,
                static_rhs: Vec<Vec<RulePosition<Category, Terminal>>>,
                root_rhs: Vec<RulePosition<Category, Terminal>>,
            }
            impl PreparedGrammar {
                pub fn new(lexemes: &[Lexeme]) -> Result<Self, FrameSchemaError> {
                    let mut registry = FrameRegistry::new();
                    let mut heads = Vec::new();
                    for lexeme in lexemes {
                        ::deckmaste_construction_core::frame::validate_verb_forms(&lexeme.forms)?;
                        if lexeme.frames.is_empty() { return Err(FrameSchemaError::MissingFrames); }
                        if heads.iter().any(|row: &HeadRow| row.head.reference == lexeme.reference) { return Err(FrameSchemaError::DuplicateLexeme); }
                        let mut schemas = Vec::new();
                        for frame in &lexeme.frames {
                            let id = registry.intern(frame, ChildCategory::resolve, Marker::resolve, |role| match role { #(#roles)* _ => None })?;
                            if !schemas.contains(&id) { schemas.push(id); }
                        }
                        for (feature, surface) in &lexeme.forms {
                            heads.push(HeadRow { head: Head { reference: lexeme.reference.clone(), feature: *feature, row: heads.len() }, surface: surface.clone(), schemas: schemas.clone() });
                        }
                    }
                    let mut grammar = Self {
                        registry, heads, productions: Vec::new(),
                        static_rhs: super::RULES.iter().map(|rule| rule.rhs.iter().map(|position| match *position {
                            RulePosition::Nonterminal(category) => RulePosition::Nonterminal(Category::Static(category)),
                            RulePosition::AdjacentNonterminal(category) => RulePosition::AdjacentNonterminal(Category::Static(category)),
                            RulePosition::Lexical(terminal) => RulePosition::Lexical(Terminal::Static(terminal)),
                        }).collect()).collect(),
                        root_rhs: vec![RulePosition::Nonterminal(Category::Phrase)],
                    };
                    grammar.prepare();
                    Ok(grammar)
                }
                pub fn schema_count(&self) -> usize { self.registry.schemas().len() }
                pub fn prepared_rule_count(&self) -> usize { self.productions.len() + self.static_rhs.len() + 1 }
                pub fn head(&self, row: usize) -> Option<Head> { self.heads.get(row).map(|row| row.head.clone()) }
                fn push(&mut self, lhs: Category, rhs: Vec<RulePosition<Category, Terminal>>, action: Action) {
                    self.productions.push(Production { lhs, rhs, action });
                }
                fn item_rhs(&self, schema: SchemaId, position: usize) -> Vec<RulePosition<Category, Terminal>> {
                    let item = &self.registry.schemas()[schema.0][position];
                    let mut rhs = Vec::new();
                    if let Some(marker) = item.marker { rhs.push(RulePosition::Lexical(Terminal::Marker(marker))); }
                    if let Some((_, category)) = item.complement { rhs.push(RulePosition::Nonterminal(Category::Static(category.grammar()))); }
                    rhs
                }
                fn prepare(&mut self) {
                    use RulePosition::{Nonterminal as N, Lexical as L};
                    for s in 0..self.registry.schemas().len() {
                        let schema = SchemaId(s);
                        let length = self.registry.schemas()[s].len();
                        self.push(Category::Suffix(schema, length), vec![], Action::End(false));
                        for start in 0..length {
                            let optional = self.registry.schemas()[s][start].optional;
                            let mut rhs = self.item_rhs(schema, start);
                            rhs.push(N(Category::Suffix(schema, start + 1)));
                            self.push(Category::Suffix(schema, start), rhs, Action::Item(schema, start, false));
                            if optional { self.push(Category::Suffix(schema, start), vec![N(Category::Suffix(schema, start + 1))], Action::Skip); }
                            for end in start + 1..=length {
                                for c in 0..COORDINATORS.len() {
                                    let coordinate = Category::Coordinate(schema, start, end, c);
                                    self.push(coordinate, vec![N(Category::Interval(schema, start, end)), L(Terminal::Marker(COORDINATORS[c])), N(Category::Interval(schema, start, end))], Action::Coordinate(schema, start, end, c, false));
                                    self.push(coordinate, vec![N(coordinate), L(Terminal::Marker(COORDINATORS[c])), N(Category::Interval(schema, start, end))], Action::Coordinate(schema, start, end, c, true));
                                    self.push(Category::Suffix(schema, start), vec![N(coordinate), N(Category::Suffix(schema, end))], Action::AppendCoordination);
                                }
                            }
                        }
                        for end in 1..=length {
                            self.push(Category::Interval(schema, end, end), vec![], Action::End(true));
                            for start in 0..end {
                                let mut rhs = self.item_rhs(schema, start);
                                rhs.push(N(Category::Interval(schema, start + 1, end)));
                                self.push(Category::Interval(schema, start, end), rhs, Action::Item(schema, start, true));
                                if self.registry.schemas()[s][start].optional {
                                    self.push(Category::Interval(schema, start, end), vec![N(Category::Interval(schema, start + 1, end))], Action::Skip);
                                }
                            }
                        }
                    }
                    for row in 0..self.heads.len() {
                        for schema in self.heads[row].schemas.clone() {
                            self.push(Category::Phrase, vec![L(Terminal::Head(row)), N(Category::Suffix(schema, 0))], Action::Phrase(schema));
                        }
                    }
                }
                pub fn rules(&self) -> Vec<Rule<'_, Category, Terminal, RuleId>> {
                    let mut rules = super::RULES.iter().zip(&self.static_rhs).map(|(rule, rhs)| Rule {
                        id: RuleId::Static(rule.id), lhs: Category::Static(rule.lhs), rhs,
                    }).collect::<Vec<_>>();
                    rules.extend(self.productions.iter().enumerate().map(|(index, production)| Rule { id: RuleId::Frame(index), lhs: production.lhs, rhs: &production.rhs }));
                    rules.push(Rule { id: RuleId::Root, lhs: Category::Root, rhs: &self.root_rhs });
                    rules
                }
                pub fn scan(&self, input: &ScanInput<'_>, terminal: Terminal) -> Vec<LexicalMatch<Value, LexicalOwner>> {
                    match terminal {
                        Terminal::Static(terminal) => super::scan_lexical(input, terminal).into_iter().map(|found| LexicalMatch { end: found.end, owner: found.owner, value: Value::Static(super::BuildValue::Leaf(found.value)) }).collect(),
                        Terminal::Head(index) => {
                            let Some(row) = self.heads.get(index) else { return Vec::new(); };
                            input.word_end(&row.surface, LexicalBoundary::Separated).map(|end| LexicalMatch { end, owner: Some(row.head.owner()), value: Value::Head(row.head.clone()) }).into_iter().collect()
                        }
                        Terminal::Marker(marker) => input.word_end(marker.surface(), LexicalBoundary::Separated).map(|end| LexicalMatch { end, owner: Some(marker.owner()), value: Value::Marker(marker) }).into_iter().collect(),
                    }
                }
                pub fn build(&self, rule: RuleId, children: &[Value], context: &ParseContext<'_>) -> Result<Option<Value>, Rejection> {
                    match rule {
                        RuleId::Static(rule) => {
                            let Some(children) = children.iter().map(|child| if let Value::Static(value) = child { Some(value.clone()) } else { None }).collect::<Option<Vec<_>>>() else { return Err(Rejection::WrongChildren); };
                            super::build_checked(rule, &children, context).map(|value| value.map(Value::Static)).map_err(|_| Rejection::WrongChildren)
                        }
                        RuleId::Root => match children {
                            [Value::Phrase(value)] => {
                                #family_name::check_parts(self, &value.head, value.schema, &value.parts)?;
                                Ok(Some(Value::Phrase(value.clone())))
                            }
                            _ => Err(Rejection::WrongChildren),
                        },
                        RuleId::Frame(index) => {
                            let action = self.productions.get(index).ok_or(Rejection::WrongChildren)?.action;
                            self.build_frame(action, children).map(Some)
                        }
                    }
                }
                fn build_frame(&self, action: Action, children: &[Value]) -> Result<Value, Rejection> {
                    match (action, children) {
                        (Action::End(false), []) => Ok(Value::Sequence(Vec::new())),
                        (Action::End(true), []) => Ok(Value::Interval(Vec::new())),
                        (Action::Skip, [value @ (Value::Sequence(_) | Value::Interval(_))]) => Ok(value.clone()),
                        (Action::Phrase(schema), [Value::Head(head), Value::Sequence(parts)]) => #family_name::try_new(self, head.clone(), schema, parts.clone()).map(Value::Phrase),
                        (Action::AppendCoordination, [Value::Coordination(part), Value::Sequence(tail)]) => {
                            let mut parts = vec![part.clone()]; parts.extend(tail.clone()); Ok(Value::Sequence(parts))
                        }
                        (Action::Item(schema, position, interval), children) => {
                            let item = &self.registry.schemas()[schema.0][position];
                            let mut index = 0;
                            let marker = if let Some(expected) = item.marker {
                                if !matches!(children.get(index), Some(Value::Marker(marker)) if *marker == expected) { return Err(Rejection::WrongMember); }
                                index += 1; Some(expected)
                            } else { None };
                            let child = if let Some((_, expected)) = item.complement {
                                let Some(Value::Static(value)) = children.get(index) else { return Err(Rejection::WrongChildren); };
                                let child = Child::from_build(value).ok_or(Rejection::WrongMember)?;
                                if child.category() != expected { return Err(Rejection::WrongMember); }
                                index += 1; Some(child)
                            } else { None };
                            let member = Member { position, relation: item.complement.map(|value| value.0), marker, child };
                            match (&children[index..], interval) {
                                ([Value::Interval(tail)], true) => { let mut members = vec![member]; members.extend(tail.clone()); Ok(Value::Interval(members)) }
                                ([Value::Sequence(tail)], false) => { let mut parts = vec![Part::Member(member)]; parts.extend(tail.clone()); Ok(Value::Sequence(parts)) }
                                _ => Err(Rejection::WrongChildren),
                            }
                        }
                        (Action::Coordinate(schema, start, end, c, repeated), [first, Value::Marker(marker), Value::Interval(last)]) => {
                            if *marker != COORDINATORS[c] { return Err(Rejection::WrongMember); }
                            let items = &self.registry.schemas()[schema.0];
                            #family_name::check_interval(items, start, end, last)?;
                            let mut conjuncts = match (repeated, first) {
                                (false, Value::Interval(first)) => {
                                    #family_name::check_interval(items, start, end, first)?;
                                    vec![first.clone()]
                                }
                                (true, Value::Coordination(Part::Coordination { start: old_start, end: old_end, conjuncts, coordinator })) if *old_start == start && *old_end == end && *coordinator == *marker => conjuncts.clone(),
                                _ => return Err(Rejection::WrongChildren),
                            };
                            conjuncts.push(last.clone());
                            Ok(Value::Coordination(Part::Coordination { start, end, conjuncts, coordinator: *marker }))
                        }
                        _ => Err(Rejection::WrongChildren),
                    }
                }
            }
        }
    };
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Module,
            name: module.to_string(),
        },
        tokens,
        vec![DeclarationKey::new(
            SourceDeclarationKind::FrameFamily,
            family_name.to_string(),
        )],
    )
}
