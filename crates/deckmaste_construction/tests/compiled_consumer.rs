#![allow(
    dead_code,
    reason = "the consumer compiles every generated phase while executing build, render, and visitor boundaries"
)]

use deckmaste_construction::constructions;

mod environment {
    use macro_ron::v2::DeclarationIdentity;
    use macro_ron::v2::GrammarRecipe;
    use macro_ron::v2::NormalizedDeclaration;
    use macro_ron::v2::SurfaceFeature;

    pub(crate) struct ParserEnvironment {
        declarations: Vec<NormalizedDeclaration>,
    }

    impl ParserEnvironment {
        pub(crate) fn new(declarations: Vec<NormalizedDeclaration>) -> Self {
            Self { declarations }
        }

        pub(crate) fn surface<'a>(
            &'a self,
            id: &DeclarationIdentity,
            feature: SurfaceFeature,
        ) -> Option<&'a str> {
            self.declarations
                .iter()
                .find(|declaration| declaration.identity() == id)
                .and_then(NormalizedDeclaration::grammar)
                .and_then(|grammar| {
                    grammar
                        .surfaces()
                        .iter()
                        .find(|surface| surface.feature() == feature)
                })
                .map(macro_ron::v2::RealizedSurface::text)
        }

        pub(crate) fn noun_rows(&self) -> Vec<(DeclarationIdentity, SurfaceFeature, &str)> {
            self.declarations
                .iter()
                .filter_map(|declaration| {
                    let grammar = declaration.grammar()?;
                    matches!(grammar.recipe(), GrammarRecipe::Noun)
                        .then_some((declaration.identity(), grammar))
                })
                .flat_map(|(id, grammar)| {
                    grammar
                        .surfaces()
                        .iter()
                        .map(move |surface| (id.clone(), surface.feature(), surface.text()))
                })
                .collect()
        }
    }
}

mod declaration_noun_fixture {
    use RulePosition::Lexical as L;

    use super::constructions;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RulePosition<Category, Lexical> {
        Nonterminal(Category),
        Lexical(Lexical),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Rule<Category: 'static, Lexical: 'static, RuleId> {
        id: RuleId,
        lhs: Category,
        rhs: &'static [RulePosition<Category, Lexical>],
    }

    #[derive(Default)]
    struct ParseContext<'a> {
        marker: std::marker::PhantomData<&'a ()>,
    }

    trait Render {
        fn render(
            &self,
            context: &ParseContext<'_>,
            environment: &crate::environment::ParserEnvironment,
        ) -> String;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RawRenderedClaim {
        start: usize,
        end: usize,
        owner: LexicalOwner,
    }

    enum ClaimSink<'a> {
        Noop,
        Collect(&'a mut Vec<RawRenderedClaim>),
    }

    struct Writer<'a> {
        output: String,
        capitalize_next: bool,
        claims: ClaimSink<'a>,
    }

    impl Writer<'_> {
        fn new() -> Self {
            Self {
                output: String::new(),
                capitalize_next: true,
                claims: ClaimSink::Noop,
            }
        }

        fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
            Writer {
                output: String::new(),
                capitalize_next: true,
                claims: ClaimSink::Collect(claims),
            }
        }

        fn claim(&mut self, owner: impl FnOnce() -> LexicalOwner, render: impl FnOnce(&mut Self)) {
            let start = self.output.len();
            render(self);
            let end = self.output.len();
            if let ClaimSink::Collect(claims) = &mut self.claims {
                claims.push(RawRenderedClaim {
                    start,
                    end,
                    owner: owner(),
                });
            }
        }

        fn word(&mut self, word: &str) {
            if !self.output.is_empty() {
                self.output.push(' ');
            }
            if self.capitalize_next {
                let mut characters = word.chars();
                if let Some(first) = characters.next() {
                    self.output.extend(first.to_uppercase());
                    self.output.push_str(characters.as_str());
                }
                self.capitalize_next = false;
            } else {
                self.output.push_str(word);
            }
        }

        fn punctuation(&mut self, punctuation: char) {
            self.output.push(punctuation);
        }

        fn finish(self) -> String {
            self.output
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BuildValue {
        Phrase(Phrase),
        PluralPhrase(PluralPhrase),
        Leaf(Leaf),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct LexicalMatch<T, O = ()> {
        end: usize,
        value: T,
        owner: Option<O>,
    }

    struct ScanInput<'a> {
        text: &'a str,
        position: ScanPosition,
        environment: &'a crate::environment::ParserEnvironment,
        context: &'a ParseContext<'a>,
    }

    impl ScanInput<'_> {
        fn word_end(&self, running_text: &str) -> Option<usize> {
            let prefix = usize::from(self.position.case == CasePosition::Continuation);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let rendered = if self.position.case == CasePosition::DocumentInitial {
                let mut characters = running_text.chars();
                characters
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(characters)
                    .collect::<String>()
            } else {
                running_text.to_owned()
            };
            let end = self.position.byte_offset + prefix + rendered.len();
            let has_boundary = matches!(
                self.text.as_bytes().get(end),
                None | Some(b' ' | b',' | b'.')
            );
            (remainder.starts_with(&rendered) && has_boundary).then_some(end)
        }

        fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(punctuation)
                .then_some(self.position.byte_offset + punctuation.len())
        }

        fn declaration_noun_readings(
            &self,
            position: macro_ron::v2::GrammarPosition,
            wanted: FeatureConstraint<Number>,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
        )> {
            assert_eq!(position, macro_ron::v2::GrammarPosition::Noun);
            self.environment
                .noun_rows()
                .into_iter()
                .filter_map(|(id, feature, surface)| {
                    let number = match feature {
                        macro_ron::v2::SurfaceFeature::Singular => Number::Singular,
                        macro_ron::v2::SurfaceFeature::Plural => Number::Plural,
                        _ => return None,
                    };
                    (matches!(wanted, FeatureConstraint::Any)
                        || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number))
                    .then(|| self.word_end(surface).map(|end| (end, id, feature)))
                    .flatten()
                })
                .collect()
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
        )> {
            debug_assert_eq!(self.context.marker, std::marker::PhantomData);
            Vec::new()
        }
    }

    fn scan_bound_terminal(
        _input: &ScanInput<'_>,
        _terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        Vec::new()
    }

    constructions! {
        lexeme NounLexeme { Player, }
        codec Noun {
            generate declaration_noun {
                closed = NounLexeme;
                position = Noun;
                kinds = [Type, Subtype];
                feature = Number;
            }
        }
        construction singular: Phrase {
            element SingularPhrase { head: lex Noun, }
            derive number = Values::Singular;
            form singular = noun(head);
        }
        construction plural: PluralPhrase {
            element PluralPhraseNode { head: lex Noun, }
            derive number = Values::Plural;
            form plural = noun(head);
        }
        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        root PluralPhrase { punctuation = "."; eoi = true; standalone_render = true; }
    }

    fn declaration(path: &str, source: &str) -> macro_ron::v2::DeclarationSource {
        macro_ron::v2::DeclarationSource::new(path, source)
    }

    fn environment() -> crate::environment::ParserEnvironment {
        let declarations = macro_ron::v2::read_sources(vec![
            declaration(
                "/synthetic/types/Relic.ron",
                r#"Type(name:"Relic",spelling:"relic",grammar:Noun(singular:"relic"))"#,
            ),
            declaration(
                "/synthetic/subtypes/creature/Elf.ron",
                r#"Subtype(category:Creature,name:"Elf",spelling:"Elf",grammar:Noun(singular:"Elf",plural:"Elves"))"#,
            ),
            declaration(
                "/synthetic/abilities/Fraud.ron",
                r#"KeywordAbility(name:"Fraud",spelling:"Fraud",grammar:Noun(singular:"Fraud"))"#,
            ),
        ])
        .expect("synthetic declaration sources normalize");
        crate::environment::ParserEnvironment::new(declarations)
    }

    struct Recorder(Vec<String>);

    impl Visitor for Recorder {
        fn visit_declaration(&mut self, declaration: &macro_ron::v2::DeclarationIdentity) {
            self.0.push(declaration.to_string());
        }
    }

    fn assert_build_render_and_visit(
        environment: &crate::environment::ParserEnvironment,
        context: &ParseContext<'_>,
        singular: &LexicalMatch<Leaf, LexicalOwner>,
        plural: &LexicalMatch<Leaf, LexicalOwner>,
    ) {
        let built = build(
            RuleId::PhraseSingular,
            &[
                BuildValue::Leaf(singular.value.clone()),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ],
            context,
        )
        .expect("generated declaration noun builds through its construction");
        let BuildValue::Phrase(phrase) = built else {
            panic!("singular declaration noun builds the declared root")
        };
        assert_eq!(Render::render(&phrase, context, environment), "Relic.");
        let (rendered, claims) = render_phrase_with_claims(&phrase, context, environment);
        assert_eq!(rendered, "Relic.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 5, "lexeme:type/Relic/singular"),
                (5, 6, "root:Phrase/punctuation"),
            ]
        );

        let built = build(
            RuleId::PluralPhrasePlural,
            &[
                BuildValue::Leaf(plural.value.clone()),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ],
            context,
        )
        .expect("generated plural declaration noun builds through its construction");
        let BuildValue::PluralPhrase(plural_phrase) = built else {
            panic!("plural declaration noun builds the declared plural root")
        };
        assert_eq!(
            Render::render(&plural_phrase, context, environment),
            "Elves."
        );
        let (rendered, claims) =
            render_plural_phrase_with_claims(&plural_phrase, context, environment);
        assert_eq!(rendered, "Elves.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 5, "lexeme:creature_subtype/Elf/plural"),
                (5, 6, "root:PluralPhrase/punctuation"),
            ]
        );

        let mut recorder = Recorder(Vec::new());
        walk_noun(
            &mut recorder,
            match &singular.value {
                Leaf::Noun { noun, .. } => noun,
                _ => unreachable!(),
            },
        );
        walk_plural_phrase(&mut recorder, &plural_phrase);
        assert_eq!(recorder.0, ["type `Relic`", "creature subtype `Elf`"]);
    }

    pub(crate) fn run() {
        let environment = environment();
        let context = ParseContext::default();
        let relic =
            macro_ron::v2::DeclarationIdentity::new(macro_ron::v2::DeclarationKind::Type, "Relic");
        let elf = macro_ron::v2::DeclarationIdentity::new(
            macro_ron::v2::DeclarationKind::Subtype(macro_ron::v2::SubtypeCategory::Creature),
            "Elf",
        );

        let relic_singular = DeclarationNoun::new(
            &environment,
            relic.clone(),
            macro_ron::v2::SurfaceFeature::Singular,
        )
        .expect("Type singular membership constructs");
        assert_eq!(relic_singular.id(), &relic);
        assert_eq!(
            relic_singular.feature(),
            macro_ron::v2::SurfaceFeature::Singular
        );
        assert!(
            DeclarationNoun::new(
                &environment,
                elf.clone(),
                macro_ron::v2::SurfaceFeature::Plural,
            )
            .is_some()
        );
        assert!(
            DeclarationNoun::new(
                &environment,
                macro_ron::v2::DeclarationIdentity::new(
                    macro_ron::v2::DeclarationKind::Type,
                    "Missing",
                ),
                macro_ron::v2::SurfaceFeature::Singular,
            )
            .is_none()
        );
        assert!(
            DeclarationNoun::new(
                &environment,
                macro_ron::v2::DeclarationIdentity::new(
                    macro_ron::v2::DeclarationKind::KeywordAbility,
                    "Fraud",
                ),
                macro_ron::v2::SurfaceFeature::Singular,
            )
            .is_none()
        );

        let scan = |text, byte_offset, case, wanted| {
            scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition { byte_offset, case },
                    environment: &environment,
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::Noun(wanted),
                    owner: LexicalOwnerTemplate::DeclarationNoun,
                },
            )
        };
        let singular = scan(
            "Relic.",
            0,
            CasePosition::DocumentInitial,
            FeatureConstraint::Exact(Number::Singular),
        );
        assert_eq!(singular.len(), 1);
        assert_eq!(singular[0].end, 5);
        assert!(matches!(
            &singular[0].value,
            Leaf::Noun { noun: Noun::Declaration(noun), number: Number::Singular }
                if noun.id() == &relic
        ));
        let owner = LexicalOwnerTemplate::DeclarationNoun
            .instantiate(&singular[0].value)
            .expect("declaration noun has an exact owner");
        assert_eq!(owner.kind(), LexicalProvenanceKind::Lexeme);
        assert_eq!(owner.stable_id(), "lexeme:type/Relic/singular");

        let plural = scan(
            "prefix Elves.",
            6,
            CasePosition::Continuation,
            FeatureConstraint::Exact(Number::Plural),
        );
        assert_eq!(plural.len(), 1);
        assert_eq!(plural[0].end, 12);
        assert!(matches!(
            &plural[0].value,
            Leaf::Noun { noun: Noun::Declaration(noun), number: Number::Plural }
                if noun.id() == &elf
                    && noun.feature() == macro_ron::v2::SurfaceFeature::Plural
        ));
        assert!(
            scan(
                "Fraud.",
                0,
                CasePosition::DocumentInitial,
                FeatureConstraint::Any,
            )
            .is_empty()
        );

        let closed = scan(
            "Player.",
            0,
            CasePosition::DocumentInitial,
            FeatureConstraint::Exact(Number::Singular),
        );
        assert!(matches!(
            closed.as_slice(),
            [LexicalMatch {
                end: 6,
                value: Leaf::Noun {
                    noun: Noun::Lexeme(NounLexeme::Player),
                    number: Number::Singular
                },
                owner: Some(_),
            }]
        ));

        assert_build_render_and_visit(&environment, &context, &singular[0], &plural[0]);
    }
}

mod fixture {
    use RulePosition::Lexical as L;
    use RulePosition::Nonterminal as N;

    use super::constructions;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RulePosition<Category, Lexical> {
        Nonterminal(Category),
        Lexical(Lexical),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Rule<Category: 'static, Lexical: 'static, RuleId> {
        id: RuleId,
        lhs: Category,
        rhs: &'static [RulePosition<Category, Lexical>],
    }

    #[derive(Default)]
    struct ParseContext<'a> {
        sentinel: u8,
        card_name: &'a str,
        abbreviated_card_name: &'a str,
    }

    trait Render {
        fn render(&self, context: &ParseContext<'_>) -> String;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RawRenderedClaim {
        start: usize,
        end: usize,
        owner: LexicalOwner,
    }

    enum ClaimSink<'a> {
        Noop,
        Collect(&'a mut Vec<RawRenderedClaim>),
    }

    struct Writer<'a> {
        output: String,
        claims: ClaimSink<'a>,
    }

    impl Writer<'_> {
        fn new() -> Self {
            Self {
                output: String::new(),
                claims: ClaimSink::Noop,
            }
        }

        fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
            Writer {
                output: String::new(),
                claims: ClaimSink::Collect(claims),
            }
        }

        fn claim(&mut self, owner: impl FnOnce() -> LexicalOwner, render: impl FnOnce(&mut Self)) {
            let start = self.output.len();
            render(self);
            let end = self.output.len();
            if let ClaimSink::Collect(claims) = &mut self.claims {
                claims.push(RawRenderedClaim {
                    start,
                    end,
                    owner: owner(),
                });
            }
        }

        fn word(&mut self, word: &str) {
            if !self.output.is_empty() {
                self.output.push(' ');
            }
            self.output.push_str(word);
        }

        fn punctuation(&mut self, punctuation: char) {
            self.output.push(punctuation);
        }

        fn identity(&mut self, identity: &str) {
            self.word(identity);
        }

        fn finish(self) -> String {
            self.output
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Head(u8);

    fn render_head(writer: &mut Writer, head: &Head, number: Number) {
        let suffix = match number {
            Number::Singular => "s",
            Number::Plural => "p",
        };
        writer.word(&format!("{}{}", head.0, suffix));
    }

    fn agreement_for_mode(mode: Mode) -> Agreement {
        match mode {
            Mode::One => Agreement::ThirdPersonSingular,
            Mode::Many => Agreement::Bare,
        }
    }

    fn inflect(lexeme: Verbs, agreement: Agreement) -> &'static str {
        match (lexeme, agreement) {
            (Verbs::Act, Agreement::Bare) => "act",
            (Verbs::Act, Agreement::ThirdPersonSingular) => "acts",
        }
    }

    impl<'a> ParseContext<'a> {
        fn card_name(&self) -> &'a str {
            debug_assert_ne!(self.sentinel, 0);
            self.card_name
        }

        fn abbreviated_card_name(&self) -> &'a str {
            debug_assert_ne!(self.sentinel, 0);
            self.abbreviated_card_name
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Token(u8);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RawBranchToken {
        Marker(Marker),
    }

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "the declared borrowed codec ABI is the behavior under test"
    )]
    fn render_token(writer: &mut Writer, token: &Token) {
        writer.word(&format!("token{}", token.0));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Pair {
        context_value: u8,
        child_slot: u8,
        rule_code: u8,
    }

    impl Pair {
        #[allow(
            clippy::trivially_copy_pass_by_ref,
            reason = "the fixture preserves parser-slice references through the declared build call"
        )]
        fn new(context: &u8, children: &u8, rule: &u8) -> Self {
            Self {
                context_value: *context,
                child_slot: *children,
                rule_code: *rule,
            }
        }
    }

    fn render_pair(writer: &mut Writer, pair: &Pair) {
        writer.word(&format!(
            "{}:{}:{}",
            pair.context_value, pair.child_slot, pair.rule_code
        ));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BoundLeaf {
        Pair(u8, u8, u8),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BuildValue {
        Source(Source, Number),
        Phrase(Phrase),
        PairPhrase(PairPhrase),
        Child(Child, Agreement),
        Parent(Parent, Agreement),
        Action(Action, Agreement),
        Predicate(Predicate, Agreement),
        Container(Container),
        CheckedContext(CheckedContext),
        FeatureChecked(FeatureChecked, Agreement),
        Collision(Collision),
        RenderChild(RenderChild),
        RawPayload(RawPayload),
        Keyword(Keyword, Agreement),
        HygieneRoot(HygieneRoot),
        ContextEnvelope(ContextEnvelope),
        VisitCategory(VisitCategory),
        CheckedMarker(CheckedMarker),
        RawCategory(RawCategory),
        Leaf(Leaf),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct LexicalMatch<T, O = ()> {
        end: usize,
        value: T,
        owner: Option<O>,
    }

    struct ScanInput<'a> {
        text: &'a str,
        position: ScanPosition,
        context: &'a ParseContext<'a>,
    }

    impl ScanInput<'_> {
        fn word_end(&self, running_text: &str) -> Option<usize> {
            let prefix = usize::from(self.position.case == CasePosition::Continuation);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let rendered = if self.position.case == CasePosition::DocumentInitial {
                let mut chars = running_text.chars();
                chars
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(chars)
                    .collect::<String>()
            } else {
                running_text.to_owned()
            };
            let end = self.position.byte_offset + prefix + rendered.len();
            let has_boundary = match self.text.get(end..) {
                Some("") => true,
                Some(trailing) => trailing
                    .chars()
                    .next()
                    .is_some_and(|character| !character.is_alphanumeric()),
                None => false,
            };
            (remainder.starts_with(&rendered) && has_boundary).then_some(end)
        }

        fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(punctuation)
                .then_some(self.position.byte_offset + punctuation.len())
        }

        fn identity_end(&self, exact_text: &str) -> Option<usize> {
            let prefix = usize::from(self.position.case == CasePosition::Continuation);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let end = self.position.byte_offset + prefix + exact_text.len();
            (!exact_text.is_empty()
                && remainder.starts_with(exact_text)
                && matches!(
                    self.text.as_bytes().get(end),
                    None | Some(b' ' | b',' | b'.')
                ))
            .then_some(end)
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
        )> {
            debug_assert!(self.position.byte_offset <= self.text.len());
            Vec::new()
        }
    }

    fn scan_bound_terminal(
        _input: &ScanInput<'_>,
        _terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        Vec::new()
    }

    constructions! {
        vocab Mode { One = "one", Many = "many", }
        vocab r#Marker { One = "marker", }
        vocab WriterWord { One = "writer", }
        lexeme Verbs { Act, }
        lexeme r#VisitorLexeme { Act, }

        identity r#SelfRef {
            generate context {
                Full => card_name,
                Abbreviated => abbreviated_card_name,
                canonical_on_collision = Full;
            }
        }

        codec r#Token {
            atom = lex;
            value_type = Token;
            lexical = Lexical::Token;
            render = render_token;
            build { pattern = BuildValue::Token(token); construct = token; }
            traversal {
                callback = borrowed;
                argument = visitor;
                call visitor::visit_token(borrowed(visitor));
            }
        }

        codec SignedNumber {
            generate signed_decimal {
                magnitude = u32;
                sign_type = Sign {
                    Positive = none,
                    Negative = "-",
                };
            }
        }


        codec RawBranchToken {
            value_type = RawBranchToken;
            traversal {
                callback = borrowed;
                argument = r#payload;
                variant Marker;
                match payload {
                    RawBranchToken::Marker(r#walk_marker: Marker) =>
                        r#walk_marker(copy(walk_marker)),
                }
            }
        }

        codec Head {
            atom = noun;
            value_type = Head;
            lexical = Lexical::Head;
            render = render_head;
            build { pattern = BuildValue::Head(head); construct = head; }
            traversal {
                callback = borrowed;
                argument = head;
                call visitor::visit_head(borrowed(head));
            }
        }

        codec Pair {
            atom = lex;
            value_type = Pair;
            lexical = Lexical::Pair;
            render = render_pair;
            build {
                pattern = BuildValue::Pair(r#context, children, rule);
                construct = Pair::new(context, children, rule);
            }
            traversal {
                callback = borrowed;
                argument = pair;
                call visitor::visit_pair(borrowed(pair));
            }
        }

        construction source: Source {
            element SourceNode {}
            derive number = Values::Singular;
            form source = "source";
        }
        construction one: Phrase {
            element OnePhrase { source: Source, head: lex Head, }
            derive number = source.number;
            form one = source noun(head);
        }
        construction two: PairPhrase {
            element TwoPhrase { source: Source, left: lex Head, right: lex Head, }
            derive number = source.number;
            form two = source noun(left) noun(right);
        }

        construction bare: Child {
            element BareChild {}
            derive agreement = Values::Bare;
            form bare = "bare";
        }
        construction third: Child {
            element ThirdChild {}
            derive agreement = Values::ThirdPersonSingular;
            form third = "third";
        }
        construction refined: Parent {
            element RefinedParent { mode: lex Mode, child: Child, }
            require mode is One;
            derive agreement = child.agreement;
            derive child.agreement = mode.agreement;
            derive mode.agreement = match mode {
                One => Values::ThirdPersonSingular,
                Many => Values::Bare,
            };
            form refined = lex(mode) child;
        }
        construction category_chain: Parent {
            element CategoryChain { child: Child, }
            derive agreement = child.agreement;
            derive child.agreement = Values::Bare;
            form category_chain = child;
        }

        construction action: Action {
            element ActionNode {}
            derive agreement = verb.agreement;
            derive verb.agreement = Values::Bare;
            form action = verb(Verbs::Act);
        }

        construction contextual: Predicate {
            element ContextualPredicate {}
            derive agreement = verb.agreement;
            form contextual = verb(Verbs::Act);
        }
        construction constant_container: Container {
            element ConstantContainer { predicate: Predicate, }
            derive predicate.agreement = Values::Bare;
            form constant_container = predicate;
        }
        construction from_role_container: Container {
            element FromRoleContainer { source: Child, predicate: Predicate, }
            derive predicate.agreement = source.agreement;
            form from_role_container = source predicate;
        }
        construction agreement_relay: Container {
            element RelayContainer { first: Predicate, second: Predicate, }
            derive second.agreement = first.agreement;
            derive first.agreement = Values::Bare;
            form agreement_relay = first second;
        }

        construction checked_context: CheckedContext {
            element CheckedContextNode { pair: lex Pair, }
            checked {
                visibility pair = pub(crate);
                constructor = CheckedContextNode::new(pair, context);
            }
            form checked_context = lex(pair);
        }

        construction agreement: FeatureChecked {
            element AgreementNode { marker: lex Marker, }
            checked {
                visibility marker = pub(crate);
                constructor = AgreementNode::new(marker);
            }
            derive agreement = verb.agreement;
            form agreement = lex(marker) verb(Verbs::Act);
        }

        construction collision: Collision {
            element CollisionNode {
                mode: lex Mode,
                number: lex Head,
                right_number: lex Head,
            }
            derive number = match mode {
                One => Values::Singular,
                Many => Values::Plural,
            };
            form collision = noun(number) noun(right_number) lex(mode);
        }

        construction contextual_named: Predicate {
            element ContextualNamed { agreement: lex Marker, }
            derive agreement = verb.agreement;
            form contextual_named = lex(agreement) verb(Verbs::Act);
        }

        construction wrapper: RenderChild {
            element RenderChildNode { child: Child, }
            form wrapper = child;
        }

        construction raw_payload: RawPayload {
            element RawPayloadNode { r#payload: lex Marker, }
            form raw_payload = lex(payload);
        }

        construction where: Keyword {
            element WhereNode { r#payload: lex Marker, }
            checked {
                visibility payload = private;
                access payload = payload;
                constructor = WhereNode::new(payload);
            }
            derive agreement = Values::Bare;
            form where = lex(payload);
        }

        construction hygiene_root: HygieneRoot {
            element HygieneRootNode {
                r#writer: lex Marker,
                r#context: identity SelfRef,
                predicate: Predicate,
                render_child: RenderChild,
                writer_word: lex WriterWord,
                raw_payload: RawPayload,
                keyword: Keyword,
            }
            derive predicate.agreement = Values::Bare;
            form hygiene_root = lex(r#writer) identity(r#context) predicate render_child
                lex(writer_word) raw_payload keyword;
        }

        construction context_envelope: ContextEnvelope {
            element ContextEnvelopeNode { inner: HygieneRoot, }
            form context_envelope = inner;
        }

        construction visit_node: VisitCategory {
            element VisitNode { visitor: lex Marker, }
            form visit_node = lex(visitor);
        }

        construction checked_marker: CheckedMarker {
            element WalkMarker { marker: lex Marker, }
            checked {
                visibility marker = private;
                access marker = marker;
                constructor = WalkMarker::new(marker);
            }
            form checked_marker = lex(marker);
        }

        construction r#raw_leaf: r#RawCategory {
            element r#RawNode {}
            form raw_leaf = "raw";
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        root RenderChild { punctuation = "."; eoi = false; standalone_render = true; }
        root HygieneRoot { punctuation = "!"; eoi = true; standalone_render = true; }
        root r#RawCategory { punctuation = "?"; eoi = true; standalone_render = true; }
    }

    impl CheckedContextNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(mut pair: Pair, context: &ParseContext<'_>) -> Option<Self> {
            pair.context_value = context.sentinel;
            Some(Self { pair })
        }
    }

    impl AgreementNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(marker: Marker) -> Option<Self> {
            Some(Self { marker })
        }
    }

    impl WhereNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(payload: Marker) -> Option<Self> {
            Some(Self { payload })
        }

        fn payload(&self) -> Marker {
            self.r#payload
        }
    }

    impl WalkMarker {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(marker: Marker) -> Option<Self> {
            Some(Self { marker })
        }

        fn marker(&self) -> Marker {
            self.marker
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum VisitEvent {
        Marker(Marker),
        VisitorLexeme(VisitorLexeme),
        Token(u8),
        Sign(Sign),
        SignedNumber(Sign, u32),
        SelfRef(SelfRef),
    }

    #[derive(Default)]
    struct RecordingVisitor(Vec<VisitEvent>);

    impl Visitor for RecordingVisitor {
        fn visit_marker(&mut self, marker: Marker) {
            self.0.push(VisitEvent::Marker(marker));
        }

        fn visit_visitor_lexeme(&mut self, lexeme: VisitorLexeme) {
            self.0.push(VisitEvent::VisitorLexeme(lexeme));
        }

        fn visit_token(&mut self, token: &Token) {
            self.0.push(VisitEvent::Token(token.0));
        }

        fn visit_sign(&mut self, sign: Sign) {
            self.0.push(VisitEvent::Sign(sign));
        }

        fn visit_signed_number(&mut self, number: &SignedNumber) {
            self.0
                .push(VisitEvent::SignedNumber(number.sign, number.magnitude));
        }

        fn visit_self_ref(&mut self, spelling: SelfRef) {
            self.0.push(VisitEvent::SelfRef(spelling));
        }
    }

    fn assert_generated_runtime_abi(context: &ParseContext<'_>) {
        let owner = |template: LexicalOwnerTemplate, leaf: &Leaf| {
            template
                .instantiate(leaf)
                .expect("every non-EOI terminal has an owner")
        };

        let vocab = owner(
            LexicalOwnerTemplate::Vocab {
                declaration: "Mode",
            },
            &Leaf::Mode(Mode::One),
        );
        assert_eq!(vocab.kind(), LexicalProvenanceKind::Vocab);
        assert_eq!(vocab.stable_id(), "vocab:Mode/One");

        for (template, leaf, kind, stable_id) in [
            (
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::FormLiteral,
                    stable_id: "form:fixture",
                },
                Leaf::Literal("fixture"),
                LexicalProvenanceKind::FormLiteral,
                "form:fixture",
            ),
            (
                LexicalOwnerTemplate::Lexeme {
                    declaration: "Verbs",
                    member: "Act",
                },
                Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::Bare,
                },
                LexicalProvenanceKind::Lexeme,
                "lexeme:Verbs/Act",
            ),
            (
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: "codec:Token",
                },
                Leaf::Token(Token(1)),
                LexicalProvenanceKind::Codec,
                "codec:Token",
            ),
            (
                LexicalOwnerTemplate::Identity {
                    declaration: "SelfRef",
                },
                Leaf::SelfRef(SelfRef::Full),
                LexicalProvenanceKind::Identity,
                "identity:SelfRef/Full",
            ),
        ] {
            let owner = owner(template, &leaf);
            assert_eq!(owner.kind(), kind);
            assert_eq!(owner.stable_id(), stable_id);
        }

        assert_eq!(
            LexicalOwnerTemplate::None.instantiate(&Leaf::EndOfInput),
            None
        );

        let kind = macro_ron::v2::DeclarationKind::KeywordAction;
        let position = macro_ron::v2::GrammarPosition::Verb;
        let declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: macro_ron::v2::DeclarationIdentity::new(kind, "Destroy"),
                feature: macro_ron::v2::SurfaceFeature::Bare,
            }),
        );
        assert_eq!(declaration.kind(), LexicalProvenanceKind::Lexeme);
        assert_eq!(
            declaration.stable_id(),
            "lexeme:keyword_action/Destroy/bare"
        );

        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "Destroy",
                position,
                feature: FeatureConstraint::Exact(macro_ron::v2::SurfaceFeature::Bare),
            }),
            owner: LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
        };
        let TerminalClass::Declaration(class) = terminal.class() else {
            panic!("open declaration matcher retains its category-safe class")
        };
        assert_eq!(class.kind(), kind);
        assert_eq!(class.position(), position);

        let input = ScanInput {
            text: "One",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
            },
            context,
        };
        let terminal = LexicalTerminal {
            matcher: Lexical::Mode,
            owner: LexicalOwnerTemplate::Vocab {
                declaration: "Mode",
            },
        };
        assert_eq!(
            scan_lexical(&input, terminal),
            [LexicalMatch {
                end: 3,
                value: Leaf::Mode(Mode::One),
                owner: Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Vocab,
                    "vocab:Mode/One",
                )),
            }]
        );
    }

    fn assert_generated_punctuation_scan(
        rendered: &str,
        preceding_literal: Option<&'static str>,
        punctuation: &'static str,
        context: &ParseContext<'_>,
    ) {
        let start = rendered
            .len()
            .checked_sub(punctuation.len())
            .expect("rendered text contains its punctuation");
        assert_eq!(&rendered[start..], punctuation);
        if let Some(literal) = preceding_literal {
            let literal_start = start
                .checked_sub(literal.len())
                .expect("rendered text contains the preceding literal");
            assert_eq!(&rendered[literal_start..start], literal);
            let (byte_offset, case) = if literal_start == 0 {
                (0, CasePosition::DocumentInitial)
            } else {
                assert_eq!(&rendered[literal_start - 1..literal_start], " ");
                (literal_start - 1, CasePosition::Continuation)
            };
            let input = ScanInput {
                text: rendered,
                position: ScanPosition { byte_offset, case },
                context,
            };
            let terminal = LexicalTerminal {
                matcher: Lexical::Literal(literal),
                owner: LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::FormLiteral,
                    stable_id: "compiled-consumer/pre-punctuation-literal",
                },
            };

            assert_eq!(
                scan_lexical(&input, terminal),
                [LexicalMatch {
                    end: start,
                    value: Leaf::Literal(literal),
                    owner: Some(LexicalOwner::static_owner(
                        LexicalProvenanceKind::FormLiteral,
                        "compiled-consumer/pre-punctuation-literal",
                    )),
                }],
                "the preceding generated word must recognize adjacent punctuation as its boundary"
            );
        }
        let input = ScanInput {
            text: rendered,
            position: ScanPosition {
                byte_offset: start,
                case: CasePosition::Continuation,
            },
            context,
        };
        let terminal = LexicalTerminal {
            matcher: Lexical::Literal(punctuation),
            owner: LexicalOwnerTemplate::Static {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: "compiled-consumer/punctuation",
            },
        };

        assert_eq!(
            scan_lexical(&input, terminal),
            [LexicalMatch {
                end: rendered.len(),
                value: Leaf::Literal(punctuation),
                owner: Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::FormLiteral,
                    "compiled-consumer/punctuation",
                )),
            }]
        );
    }

    #[allow(
        clippy::too_many_lines,
        reason = "one authentic compiled consumer executes the complete boundary matrix"
    )]
    pub(super) fn run() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
        };
        assert_generated_runtime_abi(&context);
        assert!(SelfRef::Full.valid_in(&context));
        assert!(!SelfRef::Abbreviated.valid_in(&context));
        assert_eq!(SelfRef::Full.surface(&context), "card");
        let abbreviated_context = ParseContext {
            sentinel: 99,
            card_name: "full card",
            abbreviated_card_name: "short",
        };
        assert!(SelfRef::Abbreviated.valid_in(&abbreviated_context));
        assert_eq!(SelfRef::Abbreviated.surface(&abbreviated_context), "short");
        let identity_matches = scan_lexical(
            &ScanInput {
                text: "short",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                },
                context: &abbreviated_context,
            },
            LexicalTerminal {
                matcher: Lexical::SelfRef,
                owner: LexicalOwnerTemplate::Identity {
                    declaration: "SelfRef",
                },
            },
        );
        assert_eq!(
            identity_matches,
            [LexicalMatch {
                end: 5,
                value: Leaf::SelfRef(SelfRef::Abbreviated),
                owner: Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Identity,
                    "identity:SelfRef/Abbreviated",
                )),
            }]
        );
        let source = build(
            RuleId::SourceSource,
            &[BuildValue::Leaf(Leaf::Literal("source"))],
            &context,
        )
        .expect("source builds with its parser-domain number payload");
        assert!(matches!(
            source,
            BuildValue::Source(Source::Source(SourceNode), Number::Singular)
        ));

        let one_children = |number| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(1),
                    number,
                }),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ]
        };
        assert!(build(RuleId::PhraseOne, &one_children(Number::Singular), &context).is_some());
        assert!(build(RuleId::PhraseOne, &one_children(Number::Plural), &context).is_none());

        let two_children = |left, right| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(2),
                    number: left,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(3),
                    number: right,
                }),
            ]
        };
        assert!(
            build(
                RuleId::PairPhraseTwo,
                &two_children(Number::Singular, Number::Singular),
                &context,
            )
            .is_some()
        );
        for children in [
            two_children(Number::Plural, Number::Singular),
            two_children(Number::Singular, Number::Plural),
        ] {
            assert!(build(RuleId::PairPhraseTwo, &children, &context).is_none());
        }

        let bare_child = BuildValue::Child(Child::Bare(BareChild), Agreement::Bare);
        let third_child =
            BuildValue::Child(Child::Third(ThirdChild), Agreement::ThirdPersonSingular);
        let parent = build(
            RuleId::ParentCategoryChain,
            std::slice::from_ref(&bare_child),
            &context,
        )
        .expect("a constant category writer flows into construction output");
        assert!(matches!(parent, BuildValue::Parent(_, Agreement::Bare)));
        assert!(
            build(
                RuleId::ParentCategoryChain,
                std::slice::from_ref(&third_child),
                &context,
            )
            .is_none()
        );

        let matching_child =
            BuildValue::Child(Child::Third(ThirdChild), Agreement::ThirdPersonSingular);
        let mismatching_child = BuildValue::Child(Child::Third(ThirdChild), Agreement::Bare);
        let refined_children = |child| vec![BuildValue::Leaf(Leaf::Mode(Mode::One)), child];
        let refined = build(
            RuleId::ParentRefined,
            &refined_children(matching_child),
            &context,
        )
        .expect("a refined writer flows through its category into construction output");
        assert!(matches!(
            refined,
            BuildValue::Parent(_, Agreement::ThirdPersonSingular)
        ));
        assert!(
            build(
                RuleId::ParentRefined,
                &refined_children(mismatching_child),
                &context,
            )
            .is_none()
        );

        let action = build(
            RuleId::ActionAction,
            &[BuildValue::Leaf(Leaf::Verb {
                lexeme: Verbs::Act,
                agreement: Agreement::Bare,
            })],
            &context,
        )
        .expect("an implicit-verb constant flows into construction output");
        assert!(matches!(action, BuildValue::Action(_, Agreement::Bare)));
        assert!(
            build(
                RuleId::ActionAction,
                &[BuildValue::Leaf(Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::ThirdPersonSingular,
                })],
                &context,
            )
            .is_none()
        );

        let contextual = |agreement| {
            BuildValue::Predicate(Predicate::Contextual(ContextualPredicate), agreement)
        };
        assert!(
            build(
                RuleId::ContainerConstantContainer,
                &[contextual(Agreement::Bare)],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerConstantContainer,
                &[contextual(Agreement::ThirdPersonSingular)],
                &context,
            )
            .is_none()
        );
        assert!(
            build(
                RuleId::ContainerFromRoleContainer,
                &[bare_child.clone(), contextual(Agreement::Bare),],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerFromRoleContainer,
                &[bare_child, contextual(Agreement::ThirdPersonSingular)],
                &context,
            )
            .is_none()
        );
        assert!(
            build(
                RuleId::ContainerAgreementRelay,
                &[contextual(Agreement::Bare), contextual(Agreement::Bare),],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerAgreementRelay,
                &[
                    contextual(Agreement::Bare),
                    contextual(Agreement::ThirdPersonSingular),
                ],
                &context,
            )
            .is_none()
        );

        let checked_context = build(
            RuleId::CheckedContextCheckedContext,
            &[BuildValue::Leaf(Leaf::Pair(BoundLeaf::Pair(1, 2, 3)))],
            &context,
        )
        .expect("terminal slots cannot shadow the parser context ABI local");
        assert!(matches!(
            checked_context,
            BuildValue::CheckedContext(CheckedContext::CheckedContext(CheckedContextNode {
                pair: Pair {
                    context_value: 99,
                    child_slot: 2,
                    rule_code: 3,
                },
            }))
        ));

        let feature_checked = build(
            RuleId::FeatureCheckedAgreement,
            &[
                BuildValue::Leaf(Leaf::Marker(Marker::One)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::ThirdPersonSingular,
                }),
            ],
            &context,
        )
        .expect("a checked map local cannot shadow its carried agreement");
        assert!(matches!(
            feature_checked,
            BuildValue::FeatureChecked(_, Agreement::ThirdPersonSingular)
        ));

        let collision = build(
            RuleId::CollisionCollision,
            &[
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(4),
                    number: Number::Singular,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(5),
                    number: Number::Singular,
                }),
                BuildValue::Leaf(Leaf::Mode(Mode::One)),
            ],
            &context,
        )
        .expect("adversarial preferred binders compile and build");
        assert!(matches!(
            collision,
            BuildValue::Collision(Collision::Collision(CollisionNode {
                number: Head(4),
                right_number: Head(5),
                ..
            }))
        ));

        let keyword = build(
            RuleId::KeywordWhere,
            &[BuildValue::Leaf(Leaf::Marker(Marker::One))],
            &context,
        )
        .expect("a keyword-named checked construction builds with its carried feature");
        let BuildValue::Keyword(keyword, Agreement::Bare) = keyword else {
            panic!("checked `where` preserves its category value and known feature")
        };

        let hygiene_root = HygieneRoot::HygieneRoot(HygieneRootNode {
            r#writer: Marker::One,
            r#context: SelfRef::Full,
            predicate: Predicate::ContextualNamed(ContextualNamed {
                agreement: Marker::One,
            }),
            render_child: RenderChild::Wrapper(RenderChildNode {
                child: Child::Bare(BareChild),
            }),
            writer_word: WriterWord::One,
            raw_payload: RawPayload::RawPayload(RawPayloadNode {
                r#payload: Marker::One,
            }),
            keyword,
        });
        let rendered_hygiene_root = Render::render(&hygiene_root, &context);
        assert_eq!(
            rendered_hygiene_root, "marker card marker act bare writer marker marker!",
            "allocated render locals preserve ABI values and generated helper calls",
        );
        assert_generated_punctuation_scan(&rendered_hygiene_root, Some("marker"), "!", &context);
        let context_free_nested_root = RenderChild::Wrapper(RenderChildNode {
            child: Child::Bare(BareChild),
        });
        assert_eq!(
            Render::render(&context_free_nested_root, &context),
            "bare.",
            "a context-free nested category and its standalone root share one helper arity",
        );

        let mut recording = RecordingVisitor::default();
        walk_visit_node(
            &mut recording,
            &VisitNode {
                visitor: Marker::One,
            },
        );
        walk_visitor_lexeme(&mut recording, VisitorLexeme::Act);
        walk_token(&mut recording, &Token(7));
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::Token(7)),
            "an ordinary declared visitor callback remains executable",
        );
        walk_raw_branch_token(&mut recording, &RawBranchToken::Marker(Marker::One));
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::Marker(Marker::One)),
            "an ordinary generated walker callback in a branch remains executable",
        );
        let walk_marker = WalkMarker::new(Marker::One).expect("checked walker fixture");
        walk_walk_marker(&mut recording, &walk_marker);
        assert_eq!(
            recording.0,
            [
                VisitEvent::Marker(Marker::One),
                VisitEvent::VisitorLexeme(VisitorLexeme::Act),
                VisitEvent::Token(7),
                VisitEvent::Marker(Marker::One),
                VisitEvent::Marker(Marker::One),
            ],
            "generated walkers retain callback order and the allocated operand values",
        );

        let signed = SignedNumber {
            sign: Sign::Negative,
            magnitude: 0,
        };
        let signed_matches = scan_lexical(
            &ScanInput {
                text: "-0",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                },
                context: &context,
            },
            LexicalTerminal {
                matcher: Lexical::SignedNumber,
                owner: LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: "codec:SignedNumber",
                },
            },
        );
        assert_eq!(signed_matches.len(), 1);
        assert_eq!(signed_matches[0].end, 2);
        assert_eq!(signed_matches[0].value, Leaf::SignedNumber(signed.clone()));
        let mut signed_writer = Writer::new();
        render_signed_number(&mut signed_writer, &signed);
        assert_eq!(signed_writer.finish(), "-0");
        walk_signed_number(&mut recording, &signed);
        assert_eq!(
            &recording.0[recording.0.len() - 2..],
            [
                VisitEvent::Sign(Sign::Negative),
                VisitEvent::SignedNumber(Sign::Negative, 0)
            ],
        );
        walk_self_ref(&mut recording, SelfRef::Abbreviated);
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::SelfRef(SelfRef::Abbreviated)),
            "the generated identity walker calls its leaf callback"
        );

        let raw_category = build(
            RuleId::RawCategoryRawLeaf,
            &[
                BuildValue::Leaf(Leaf::Literal("raw")),
                BuildValue::Leaf(Leaf::Literal("?")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ],
            &context,
        )
        .expect("raw nonkeyword declaration spellings build through canonical generated names");
        let BuildValue::RawCategory(raw_category) = raw_category else {
            panic!("raw nonkeyword root preserves its generated category value")
        };
        let rendered_raw_category = Render::render(&raw_category, &context);
        assert_eq!(rendered_raw_category, "raw?");
        assert_generated_punctuation_scan(&rendered_raw_category, None, "?", &context);
        walk_raw_category(&mut recording, &raw_category);
    }
}

#[test]
fn generated_output_is_type_correct_and_executes_every_boundary_case() {
    declaration_noun_fixture::run();
    fixture::run();
}
