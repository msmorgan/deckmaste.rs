#![allow(
    dead_code,
    reason = "the consumer compiles every generated phase while executing build, render, and visitor boundaries"
)]

use deckmaste_construction::constructions;

pub mod environment {
    use macro_ron::v2::DeclarationIdentity;
    use macro_ron::v2::GrammarRecipe;
    use macro_ron::v2::NormalizedDeclaration;
    use macro_ron::v2::Onset;
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

        pub(crate) fn onset(
            &self,
            id: &DeclarationIdentity,
            feature: SurfaceFeature,
        ) -> Option<Onset> {
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
                .map(macro_ron::v2::RealizedSurface::onset)
        }

        pub(crate) fn noun_rows(&self) -> Vec<(DeclarationIdentity, SurfaceFeature, Onset, &str)> {
            self.declarations
                .iter()
                .filter_map(|declaration| {
                    let grammar = declaration.grammar()?;
                    matches!(grammar.recipe(), GrammarRecipe::Noun)
                        .then_some((declaration.identity(), grammar))
                })
                .flat_map(|(id, grammar)| {
                    grammar.surfaces().iter().map(move |surface| {
                        (
                            id.clone(),
                            surface.feature(),
                            surface.onset(),
                            surface.text(),
                        )
                    })
                })
                .collect()
        }
    }
}

mod declaration_noun_fixture {
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
        case: CasePosition,
        prefix: PrefixPosition,
        claims: ClaimSink<'a>,
    }

    impl Writer<'_> {
        fn new() -> Self {
            Self {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
                claims: ClaimSink::Noop,
            }
        }

        fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
            Writer {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
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
            if self.prefix == PrefixPosition::WordOwnedSpace {
                self.output.push(' ');
            }
            if matches!(
                self.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ) {
                let mut characters = word.chars();
                if let Some(first) = characters.next() {
                    self.output.extend(first.to_uppercase());
                    self.output.push_str(characters.as_str());
                }
            } else {
                self.output.push_str(word);
            }
            self.case = CasePosition::Continuation;
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn suppress_next_space(&mut self) {
            self.prefix = PrefixPosition::SurfaceOwned;
        }

        fn punctuation(&mut self, punctuation: char) {
            self.output.push(punctuation);
            self.case = if punctuation == '.' {
                CasePosition::SentenceInitial
            } else {
                CasePosition::Continuation
            };
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn structural_surface(&mut self, surface: &str, transition: StructuralTransition) {
            self.output.push_str(surface);
            self.case = transition.case_after(self.case);
            self.prefix = PrefixPosition::SurfaceOwned;
        }

        fn finish(self) -> String {
            self.output
        }
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
        fn word_end(&self, running_text: &str, right_boundary: LexicalBoundary) -> Option<usize> {
            let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let rendered = if matches!(
                self.position.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ) {
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
                right_boundary,
                LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
            ) || matches!(
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

        fn structural_surface_end(&self, surface: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(surface)
                .then_some(self.position.byte_offset + surface.len())
        }

        fn declaration_noun_readings(
            &self,
            position: macro_ron::v2::GrammarPosition,
            wanted: FeatureConstraint<Number>,
            right_boundary: LexicalBoundary,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
            macro_ron::v2::Onset,
        )> {
            assert_eq!(position, macro_ron::v2::GrammarPosition::Noun);
            self.environment
                .noun_rows()
                .into_iter()
                .filter_map(|(id, feature, onset, surface)| {
                    let number = match feature {
                        macro_ron::v2::SurfaceFeature::Singular => Number::Singular,
                        macro_ron::v2::SurfaceFeature::Plural => Number::Plural,
                        _ => return None,
                    };
                    (matches!(wanted, FeatureConstraint::Any)
                        || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number))
                    .then(|| {
                        self.word_end(surface, right_boundary)
                            .map(|end| (end, id, feature, onset))
                    })
                    .flatten()
                })
                .collect()
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
            _right_boundary: LexicalBoundary,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
            macro_ron::v2::Onset,
        )> {
            debug_assert_eq!(self.context.marker, std::marker::PhantomData);
            Vec::new()
        }
    }

    constructions! {
        morphology EnglishNoun { feature = Number; recipe = english_noun; }
        lexeme NounLexeme using EnglishNoun {
            Player = "player",
            Artifact = "artifact" { Plural = "units", },
        }
        codec TypeNoun {
            generate declaration_noun {
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
        codec ArtifactSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Artifact)];
                feature = Number;
            }
        }
        codec BattleSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Battle)];
                feature = Number;
            }
        }
        codec EnchantmentSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Enchantment)];
                feature = Number;
            }
        }
        codec LandSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Land)];
                feature = Number;
            }
        }
        codec PlaneswalkerSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Planeswalker)];
                feature = Number;
            }
        }
        codec SpellSubtypeNoun {
            generate declaration_noun {
                position = Noun;
                kinds = [Subtype(Spell)];
                feature = Number;
            }
        }
        construction singular: NumberSource {
            element SingularNumberSource {}
            derive number = Values::Singular;
            form singular = "singular";
        }
        construction plural: NumberSource {
            element PluralNumberSource {}
            derive number = Values::Plural;
            form plural = "plural";
        }
        construction constant_pair: ConstantOutputPair {
            element ConstantOutputPairNode {
                left_source: NumberSource,
                left: lex TypeNoun,
                right_source: NumberSource,
                right: lex CreatureNoun,
            }
            derive left.number = left_source.number;
            derive right.number = right_source.number;
            derive number = Values::Singular;
            form constant_pair = left_source noun(left) right_source noun(right);
        }
        construction elsewhere_pair: ElsewhereOutputPair {
            element ElsewhereOutputPairNode {
                output_source: NumberSource,
                left_source: NumberSource,
                left: lex TypeNoun,
                right_source: NumberSource,
                right: lex CreatureNoun,
            }
            derive left.number = left_source.number;
            derive right.number = right_source.number;
            derive number = output_source.number;
            form elsewhere_pair = output_source left_source noun(left) right_source noun(right);
        }
        construction modified: Phrase {
            element ModifiedPhrase {
                modifier: lex TypeNoun,
                head: lex CreatureNoun,
            }
            derive modifier.number = Values::Singular;
            derive head.number = modifier.number;
            derive number = head.number;
            form modified = noun(modifier) noun(head);
        }
        construction article_noun: InflectedArticle {
            element ArticleNoun {
                source: NumberSource,
                head: lex CreatureNoun,
            }
            derive head.number = source.number;
            derive number = source.number;
            derive onset = head.onset;
            form an when head.onset is Vowel = "an" source noun(head);
            form a otherwise = "a" source noun(head);
        }
        root ConstantOutputPair { punctuation = "."; eoi = true; standalone_render = true; }
        root ElsewhereOutputPair { punctuation = "."; eoi = true; standalone_render = true; }
        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        root InflectedArticle { punctuation = "."; eoi = true; standalone_render = true; }
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
                "/synthetic/subtypes/artifact/Clue.ron",
                r#"Subtype(category:Artifact,name:"Clue",spelling:"Clue",grammar:Noun(singular:"Clue"))"#,
            ),
            declaration(
                "/synthetic/subtypes/battle/Siege.ron",
                r#"Subtype(category:Battle,name:"Siege",spelling:"Siege",grammar:Noun(singular:"Siege"))"#,
            ),
            declaration(
                "/synthetic/subtypes/enchantment/Aura.ron",
                r#"Subtype(category:Enchantment,name:"Aura",spelling:"Aura",grammar:Noun(singular:"Aura"))"#,
            ),
            declaration(
                "/synthetic/subtypes/land/Forest.ron",
                r#"Subtype(category:Land,name:"Forest",spelling:"Forest",grammar:Noun(singular:"Forest"))"#,
            ),
            declaration(
                "/synthetic/subtypes/planeswalker/Jace.ron",
                r#"Subtype(category:Planeswalker,name:"Jace",spelling:"Jace",grammar:Noun(singular:"Jace"))"#,
            ),
            declaration(
                "/synthetic/subtypes/spell/Arcane.ron",
                r#"Subtype(category:Spell,name:"Arcane",spelling:"Arcane",grammar:Noun(singular:"Arcane"))"#,
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

    fn scan<'a>(
        environment: &'a crate::environment::ParserEnvironment,
        context: &'a ParseContext<'a>,
        text: &'a str,
        byte_offset: usize,
        terminal_index: usize,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        scan_lexical(
            &ScanInput {
                text,
                position: ScanPosition {
                    byte_offset,
                    case: if byte_offset == 0 {
                        CasePosition::DocumentInitial
                    } else {
                        CasePosition::Continuation
                    },
                    prefix: if byte_offset == 0 {
                        PrefixPosition::None
                    } else {
                        PrefixPosition::WordOwnedSpace
                    },
                },
                environment,
                context,
            },
            LexicalTerminal {
                matcher: Lexical::DeclarationNoun(
                    terminal_index,
                    FeatureConstraint::Exact(Number::Singular),
                ),
                owner: LexicalOwnerTemplate::DeclarationNoun(terminal_index),
                right_boundary: LexicalBoundary::Separated,
            },
        )
    }

    fn scan_terminal<'a>(
        environment: &'a crate::environment::ParserEnvironment,
        context: &'a ParseContext<'a>,
        text: &'a str,
        byte_offset: usize,
        terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        scan_lexical(
            &ScanInput {
                text,
                position: ScanPosition {
                    byte_offset,
                    case: if byte_offset == 0 {
                        CasePosition::DocumentInitial
                    } else {
                        CasePosition::Continuation
                    },
                    prefix: if byte_offset == 0 {
                        PrefixPosition::None
                    } else {
                        PrefixPosition::WordOwnedSpace
                    },
                },
                environment,
                context,
            },
            terminal,
        )
    }

    fn phrase_rule_terminals() -> (LexicalTerminal, LexicalTerminal) {
        let rule = RULES
            .iter()
            .find(|rule| rule.id == RuleId::PhraseModified)
            .expect("the generated Phrase rule is present");
        let [L(modifier), L(head)] = rule.rhs else {
            panic!("the generated Phrase rule has two lexical noun roles")
        };
        assert!(matches!(
            modifier,
            LexicalTerminal {
                matcher: Lexical::DeclarationNoun(1, FeatureConstraint::Exact(Number::Singular),),
                owner: LexicalOwnerTemplate::DeclarationNoun(1),
                ..
            }
        ));
        assert!(
            matches!(
                head,
                LexicalTerminal {
                    matcher: Lexical::DeclarationNoun(2, FeatureConstraint::Any,),
                    owner: LexicalOwnerTemplate::DeclarationNoun(2),
                    ..
                }
            ),
            "unexpected generated head terminal: {head:?}"
        );
        (*modifier, *head)
    }

    fn assert_exact_subtype_domains(
        environment: &crate::environment::ParserEnvironment,
    ) -> macro_ron::v2::DeclarationIdentity {
        let subtype = |category, name| {
            macro_ron::v2::DeclarationIdentity::new(
                macro_ron::v2::DeclarationKind::Subtype(category),
                name,
            )
        };
        let clue = subtype(macro_ron::v2::SubtypeCategory::Artifact, "Clue");
        let siege = subtype(macro_ron::v2::SubtypeCategory::Battle, "Siege");
        let elf = subtype(macro_ron::v2::SubtypeCategory::Creature, "Elf");
        let aura = subtype(macro_ron::v2::SubtypeCategory::Enchantment, "Aura");
        let forest = subtype(macro_ron::v2::SubtypeCategory::Land, "Forest");
        let jace = subtype(macro_ron::v2::SubtypeCategory::Planeswalker, "Jace");
        let arcane = subtype(macro_ron::v2::SubtypeCategory::Spell, "Arcane");

        macro_rules! assert_exact_subtype_domain {
            ($noun:ty, $accepted:expr; $($rejected:expr),+ $(,)?) => {{
                assert!(<$noun>::new(environment, $accepted.clone()).is_some());
                $(
                    assert!(
                        <$noun>::new(environment, $rejected.clone()).is_none(),
                        "a compiled exact subtype terminal admitted a cross-family identity",
                    );
                )+
            }};
        }
        assert_exact_subtype_domain!(
            DeclarationArtifactSubtypeNoun,
            clue;
            siege, elf, aura, forest, jace, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationBattleSubtypeNoun,
            siege;
            clue, elf, aura, forest, jace, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationCreatureNoun,
            elf;
            clue, siege, aura, forest, jace, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationEnchantmentSubtypeNoun,
            aura;
            clue, siege, elf, forest, jace, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationLandSubtypeNoun,
            forest;
            clue, siege, elf, aura, jace, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationPlaneswalkerSubtypeNoun,
            jace;
            clue, siege, elf, aura, forest, arcane,
        );
        assert_exact_subtype_domain!(
            DeclarationSpellSubtypeNoun,
            arcane;
            clue, siege, elf, aura, forest, jace,
        );
        elf
    }

    pub(crate) fn run() {
        let environment = environment();
        let context = ParseContext::default();
        let relic =
            macro_ron::v2::DeclarationIdentity::new(macro_ron::v2::DeclarationKind::Type, "Relic");
        let elf = assert_exact_subtype_domains(&environment);

        let public_type = DeclarationTypeNoun::new(&environment, relic.clone())
            .expect("the public declaration noun stores a valid identity");
        assert_eq!(public_type.id(), &relic);
        assert!(DeclarationTypeNoun::new(&environment, elf.clone()).is_none());
        assert!(DeclarationCreatureNoun::new(&environment, elf.clone()).is_some());

        let (modifier_terminal, head_terminal) = phrase_rule_terminals();
        let modifier = scan_terminal(&environment, &context, "Relic Elf.", 0, modifier_terminal);
        let head = scan_terminal(&environment, &context, "Relic Elf.", 5, head_terminal);
        assert!(
            scan_terminal(&environment, &context, "Elf.", 0, modifier_terminal).is_empty(),
            "deliberately swapping the Creature declaration into the Type role fails",
        );
        assert!(
            scan_terminal(&environment, &context, "Relic.", 0, head_terminal).is_empty(),
            "deliberately swapping the Type declaration into the Creature role fails",
        );
        let swapped_modifier_index = LexicalTerminal {
            matcher: Lexical::DeclarationNoun(2, FeatureConstraint::Exact(Number::Singular)),
            owner: LexicalOwnerTemplate::DeclarationNoun(2),
            right_boundary: LexicalBoundary::Separated,
        };
        let swapped_head_index = LexicalTerminal {
            matcher: Lexical::DeclarationNoun(1, FeatureConstraint::Any),
            owner: LexicalOwnerTemplate::DeclarationNoun(1),
            right_boundary: LexicalBoundary::Separated,
        };
        assert!(
            scan_terminal(&environment, &context, "Relic.", 0, swapped_modifier_index,).is_empty(),
            "deliberately swapping the generated modifier terminal index fails",
        );
        assert!(
            scan_terminal(&environment, &context, "Elf.", 0, swapped_head_index,).is_empty(),
            "deliberately swapping the generated head terminal index fails",
        );
        assert!(
            scan_terminal(&environment, &context, "Clue.", 0, head_terminal).is_empty(),
            "the exact Creature family filter rejects an Artifact subtype",
        );
        assert!(
            scan(&environment, &context, "Relic.", 0, usize::MAX).is_empty(),
            "an unknown declaration-noun terminal index rejects without a union scan",
        );
        assert!(matches!(
            modifier.as_slice(),
            [LexicalMatch {
                value: Leaf::TypeNoun {
                    noun: TypeNoun::Declaration(noun),
                    number: Number::Singular,
                    onset: macro_ron::v2::Onset::Consonant,
                    ..
                },
                ..
            }] if noun.id() == &relic
        ));
        assert!(matches!(
            head.as_slice(),
            [LexicalMatch {
                value: Leaf::CreatureNoun {
                    noun: CreatureNoun::Declaration(noun),
                    number: Number::Singular,
                    onset: macro_ron::v2::Onset::Vowel,
                    ..
                },
                ..
            }] if noun.id() == &elf
        ));

        let built = build(
            RuleId::PhraseModified,
            &[
                BuildValue::Leaf(modifier[0].value.clone()),
                BuildValue::Leaf(head[0].value.clone()),
            ],
            &context,
        )
        .expect("singular modifier Number flows to the inherited-number head");
        let BuildValue::Phrase(phrase) = built else {
            panic!("the generated compound noun builds its declared root")
        };
        assert_eq!(
            Render::render(&phrase, &context, &environment),
            "Relic Elf."
        );

        let Leaf::CreatureNoun { noun, .. } = &head[0].value else { unreachable!() };
        let mismatched = Leaf::CreatureNoun {
            noun: noun.clone(),
            number: Number::Plural,
            onset: macro_ron::v2::Onset::Vowel,
            possessive_ending: PossessiveEnding::Other,
        };
        assert!(
            build(
                RuleId::PhraseModified,
                &[
                    BuildValue::Leaf(modifier[0].value.clone()),
                    BuildValue::Leaf(mismatched),
                ],
                &context,
            )
            .is_none(),
            "the private realized feature must agree with the sealed role Number",
        );

        let mut recorder = Recorder(Vec::new());
        walk_phrase(&mut recorder, &phrase);
        assert_eq!(recorder.0, ["type `Relic`", "creature subtype `Elf`"]);
    }

    fn number_source(number: Number) -> BuildValue {
        match number {
            Number::Singular => BuildValue::NumberSource(
                NumberSource::Singular(SingularNumberSource),
                Number::Singular,
            ),
            Number::Plural => {
                BuildValue::NumberSource(NumberSource::Plural(PluralNumberSource), Number::Plural)
            }
        }
    }

    fn declaration_leaf(noun: &Leaf, number: Number) -> BuildValue {
        match noun {
            Leaf::TypeNoun {
                noun,
                onset,
                possessive_ending,
                ..
            } => BuildValue::Leaf(Leaf::TypeNoun {
                noun: noun.clone(),
                number,
                onset: *onset,
                possessive_ending: *possessive_ending,
            }),
            Leaf::CreatureNoun {
                noun,
                onset,
                possessive_ending,
                ..
            } => BuildValue::Leaf(Leaf::CreatureNoun {
                noun: noun.clone(),
                number,
                onset: *onset,
                possessive_ending: *possessive_ending,
            }),
            _ => panic!("expected a declaration noun leaf"),
        }
    }

    fn independently_numbered_children(
        left: &Leaf,
        left_number: Number,
        right: &Leaf,
        right_number: Number,
    ) -> Vec<BuildValue> {
        vec![
            number_source(Number::Singular),
            declaration_leaf(left, left_number),
            number_source(Number::Plural),
            declaration_leaf(right, right_number),
        ]
    }

    pub(crate) fn assert_dynamic_role_number_guards_are_independent() {
        let environment = environment();
        let context = ParseContext::default();
        let left = &scan(&environment, &context, "Relic", 0, 1)[0].value;
        let right = &scan(&environment, &context, "Elf", 0, 2)[0].value;

        let valid = independently_numbered_children(left, Number::Singular, right, Number::Plural);
        assert!(
            build(RuleId::ConstantOutputPairConstantPair, &valid, &context).is_some(),
            "independent role Numbers do not inherit the constant construction Number",
        );

        let wrong_left =
            independently_numbered_children(left, Number::Plural, right, Number::Plural);
        assert!(
            build(
                RuleId::ConstantOutputPairConstantPair,
                &wrong_left,
                &context,
            )
            .is_none(),
            "only the left private Number must match left_source.number",
        );

        let wrong_right =
            independently_numbered_children(left, Number::Singular, right, Number::Singular);
        assert!(
            build(
                RuleId::ConstantOutputPairConstantPair,
                &wrong_right,
                &context,
            )
            .is_none(),
            "only the right private Number must match right_source.number",
        );
    }

    pub(crate) fn assert_irregular_noun_onset_rows_agree() {
        let environment = environment();
        let context = ParseContext::default();
        let article_rule = |id| {
            RULES
                .iter()
                .find(|rule| rule.id == id)
                .expect("the generated article rule is present")
        };
        let singular_rule = article_rule(RuleId::InflectedArticleArticleNounAn);
        let plural_rule = article_rule(RuleId::InflectedArticleArticleNounA);
        let noun_terminal = |rule: &Rule<Category, LexicalTerminal, RuleId>| {
            rule.rhs
                .iter()
                .find_map(|position| match position {
                    L(
                        terminal @ LexicalTerminal {
                            matcher: Lexical::DeclarationNoun(_, _),
                            ..
                        },
                    ) => Some(*terminal),
                    _ => None,
                })
                .expect("the article rule contains its noun terminal")
        };

        for (text, number, expected_onset, rule_id, expected_render) in [
            (
                "Artifact",
                Number::Singular,
                macro_ron::v2::Onset::Vowel,
                RuleId::InflectedArticleArticleNounAn,
                "An singular artifact.",
            ),
            (
                "Units",
                Number::Plural,
                macro_ron::v2::Onset::Consonant,
                RuleId::InflectedArticleArticleNounA,
                "A plural units.",
            ),
        ] {
            let rule = if number == Number::Singular { singular_rule } else { plural_rule };
            let scanned = scan_terminal(&environment, &context, text, 0, noun_terminal(rule));
            assert!(matches!(
                scanned.as_slice(),
                [LexicalMatch {
                    value: Leaf::CreatureNoun {
                        noun: CreatureNoun::Lexeme(NounLexeme::Artifact),
                        number: actual_number,
                        onset,
                        ..
                    },
                    ..
                }] if *actual_number == number && *onset == expected_onset
            ));
            let literal = if expected_onset == macro_ron::v2::Onset::Vowel { "an" } else { "a" };
            let built = build(
                rule_id,
                &[
                    BuildValue::Leaf(Leaf::Literal(literal)),
                    number_source(number),
                    BuildValue::Leaf(scanned[0].value.clone()),
                ],
                &context,
            )
            .expect("the exact realized noun row satisfies its article guard");
            let BuildValue::InflectedArticle(article, onset) = built else {
                panic!("the article construction carries its frozen onset")
            };
            assert_eq!(onset, expected_onset);
            assert_eq!(
                Render::render(&article, &context, &environment),
                expected_render
            );
            let inverse = if expected_onset == macro_ron::v2::Onset::Vowel {
                RuleId::InflectedArticleArticleNounA
            } else {
                RuleId::InflectedArticleArticleNounAn
            };
            assert!(
                build(
                    inverse,
                    &[
                        BuildValue::Leaf(Leaf::Literal(if literal == "an" { "a" } else { "an" })),
                        number_source(number),
                        BuildValue::Leaf(scanned[0].value.clone()),
                    ],
                    &context,
                )
                .is_none()
            );
        }
    }

    pub(crate) fn assert_construction_number_does_not_overconstrain_noun_roles() {
        let environment = environment();
        let context = ParseContext::default();
        let left = &scan(&environment, &context, "Relic", 0, 1)[0].value;
        let right = &scan(&environment, &context, "Elf", 0, 2)[0].value;
        let children = std::iter::once(number_source(Number::Singular))
            .chain(independently_numbered_children(
                left,
                Number::Singular,
                right,
                Number::Plural,
            ))
            .collect::<Vec<_>>();

        assert!(
            build(
                RuleId::ElsewhereOutputPairElsewherePair,
                &children,
                &context,
            )
            .is_some(),
            "construction Number derives from output_source without constraining either noun role",
        );
    }
}

pub mod fixture {
    #![allow(
        clippy::too_many_arguments,
        reason = "the dense hygiene fixture deliberately exercises seven stored fields plus one derived field"
    )]

    use super::constructions;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TextSpan {
        start: usize,
        end: usize,
    }

    macro_rules! engine_unit_tests {
        ($tests:item) => {};
    }

    mod engine {
        include!("../../deckmaste_english_v2/src/parser/engine.rs");
    }

    use RulePosition::Lexical as L;
    use RulePosition::Nonterminal as N;
    use engine::LexicalMatch as EngineLexicalMatch;
    use engine::Rule;
    use engine::RulePosition;
    use engine::StatefulLexicalMatch as EngineStatefulLexicalMatch;

    struct ParseContext<'a> {
        sentinel: u8,
        card_name: &'a str,
        abbreviated_card_name: &'a str,
        card_name_onset: macro_ron::v2::Onset,
        abbreviated_card_name_onset: macro_ron::v2::Onset,
    }

    impl Default for ParseContext<'_> {
        fn default() -> Self {
            Self {
                sentinel: 0,
                card_name: "",
                abbreviated_card_name: "",
                card_name_onset: macro_ron::v2::Onset::Consonant,
                abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
            }
        }
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
        case: CasePosition,
        prefix: PrefixPosition,
        claims: ClaimSink<'a>,
    }

    impl Writer<'_> {
        fn new() -> Self {
            Self {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
                claims: ClaimSink::Noop,
            }
        }

        fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
            Writer {
                output: String::new(),
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
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
            if self.prefix == PrefixPosition::WordOwnedSpace {
                self.output.push(' ');
            }
            if matches!(
                self.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ) {
                let mut characters = word.chars();
                if let Some(first) = characters.next() {
                    self.output.extend(first.to_uppercase());
                    self.output.push_str(characters.as_str());
                }
            } else {
                self.output.push_str(word);
            }
            self.case = CasePosition::Continuation;
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn punctuation(&mut self, punctuation: char) {
            self.output.push(punctuation);
            self.case = if punctuation == '.' {
                CasePosition::SentenceInitial
            } else {
                CasePosition::Continuation
            };
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn suppress_next_space(&mut self) {
            self.prefix = PrefixPosition::SurfaceOwned;
        }

        fn identity(&mut self, identity: &str) {
            if self.prefix == PrefixPosition::WordOwnedSpace {
                self.output.push(' ');
            }
            self.output.push_str(identity);
            self.case = CasePosition::Continuation;
            self.prefix = PrefixPosition::WordOwnedSpace;
        }

        fn structural_surface(&mut self, surface: &str, transition: StructuralTransition) {
            self.output.push_str(surface);
            self.case = transition.case_after(self.case);
            self.prefix = PrefixPosition::SurfaceOwned;
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

    impl<'a> ParseContext<'a> {
        fn card_name(&self) -> &'a str {
            debug_assert_ne!(self.sentinel, 0);
            self.card_name
        }

        fn abbreviated_card_name(&self) -> &'a str {
            debug_assert_ne!(self.sentinel, 0);
            self.abbreviated_card_name
        }

        fn card_name_onset(&self) -> macro_ron::v2::Onset {
            debug_assert_ne!(self.sentinel, 0);
            self.card_name_onset
        }

        fn abbreviated_card_name_onset(&self) -> macro_ron::v2::Onset {
            debug_assert_ne!(self.sentinel, 0);
            self.abbreviated_card_name_onset
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
        fn word_end(&self, running_text: &str, right_boundary: LexicalBoundary) -> Option<usize> {
            let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let rendered = if matches!(
                self.position.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ) {
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
            let has_boundary = matches!(
                right_boundary,
                LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
            ) || match self.text.get(end..) {
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

        fn structural_surface_end(&self, surface: &str) -> Option<usize> {
            self.text[self.position.byte_offset..]
                .starts_with(surface)
                .then_some(self.position.byte_offset + surface.len())
        }

        fn identity_end(&self, exact_text: &str, right_boundary: LexicalBoundary) -> Option<usize> {
            let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let remainder = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let end = self.position.byte_offset + prefix + exact_text.len();
            (!exact_text.is_empty()
                && remainder.starts_with(exact_text)
                && (matches!(
                    right_boundary,
                    LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
                ) || matches!(
                    self.text.as_bytes().get(end),
                    None | Some(b' ' | b',' | b'.')
                )))
            .then_some(end)
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
            _right_boundary: LexicalBoundary,
        ) -> Vec<(
            usize,
            macro_ron::v2::DeclarationIdentity,
            macro_ron::v2::SurfaceFeature,
            macro_ron::v2::Onset,
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
        vocab OptionalWord { That = "that", Those = "those", }
        vocab Partition { First = "first", Second = "second", Other = "other", }
        vocab r#Marker { One = "marker", }
        vocab WriterWord { One = "writer", }
        vocab StructuralWord {
            Alpha = "alpha",
            Beta = "beta",
            Gamma = "gamma",
            Delta = "delta",
        }
        vocab BoundWord {
            Artifact = "artifact",
            Black = "black",
            Elf = "Elf",
            Daxos = "Daxos",
            Tarmogoyf = "Tarmogoyf",
            Players = "players",
            Merfolk = "Merfolk",
            Equipment = "Equipment",
        }
        vocab CircumfixWord {
            WhiteBlue = "W/U",
            Tap = "T",
        }
        vocab Letter { A = "a", B = "b", }
        morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
        lexeme VerbLexeme using EnglishVerb {
            Act = "act",
            Be = "be" { Bare = "are", ThirdPersonSingular = "is", },
            Collide = "same" { ThirdPersonSingular = "same", },
            Other = "same",
        }
        vocab r#VisitorLexeme { Act = "visit", }

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

        codec NonZeroScalarNumber {
            generate unsigned_decimal {
                magnitude = NonZeroU32;
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
        construction cardinal: CardinalQuantity {
            element CardinalQuantityValue { number: lex CardinalNumber, }
            derive cardinality = number.cardinality;
            derive number = number.number;
            form cardinal = lex(number);
        }
        construction positive: NonZeroQuantity {
            element PositiveQuantityValue { number: lex NonZeroScalarNumber, }
            form positive = lex(number);
        }
        construction counted_cardinal: CountedCardinal {
            element CountedCardinalValue { cardinal: CardinalQuantity, head: lex Head, }
            derive number = cardinal.number;
            form counted_cardinal = cardinal noun(head);
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
        construction guarded: Child {
            element GuardedChild { mode: lex Mode, child: Child, }
            require any(
                all(mode is One, child is Bare),
                all(mode is Many, child is Third)
            );
            derive agreement = Values::Bare;
            derive child.agreement = Values::Bare;
            form guarded = lex(mode) child;
        }
        construction uniform_children: HomogeneousSequence {
            element UniformChildren {
                members: seq Child separated by " ",
            }
            require len(members) >= 2;
            derive members.agreement = Values::Bare;
            form uniform_children = members;
        }
        construction relayed_children: RelayedSequence {
            element RelayedChildren {
                members: seq Child separated by " ",
            }
            require len(members) >= 2;
            derive agreement = members.agreement;
            form relayed_children = members;
        }
        construction singleton_children: SingletonSequence {
            element SingletonChildren {
                members: seq Child separated by " ",
            }
            require len(members) >= 1;
            derive members.agreement = Values::Bare;
            form singleton_children = members;
        }
        construction uniform_child_choices: HomogeneousChoiceSequence {
            element UniformChildChoices {
                members: seq AgreementChild separated by " ",
            }
            require len(members) >= 2;
            derive members.agreement = Values::Bare;
            form uniform_child_choices = members;
        }
        construction relayed_child_choices: RelayedChoiceSequence {
            element RelayedChildChoices {
                members: seq AgreementChild separated by " ",
            }
            require len(members) >= 2;
            derive agreement = members.agreement;
            form relayed_child_choices = members;
        }
        construction partitioned: PartitionRoot {
            element Partitioned { word: lex Partition, }
            form first when word is First = "alpha" lex(word);
            form second when word is Second = "beta" lex(word);
            form fallback otherwise = "omega" lex(word);
        }
        construction optional_guarded: OptionalGuardRoot {
            element OptionalGuarded { word: opt lex OptionalWord, mode: lex Mode, }
            require any(
                all(word.is_none(), mode is One),
                all(word.is_some(), word is That, mode is Many)
            );
            form that when word is That = lex(word) lex(mode);
            form fallback otherwise = lex(word) lex(mode);
        }
        construction optional_visited: OptionalVisitRoot {
            element OptionalVisited { word: opt lex OptionalWord, mode: lex Mode, }
            form absent when word.is_none() = lex(mode) lex(word);
            form that when word is That = lex(word) lex(mode);
            form fallback otherwise = lex(mode) lex(word);
        }
        construction non_plain: BoundRoot {
            element NonPlain { value: lex BoundWord, }
            require value is Black;
            form non_plain = "target" prefix("non", lex(value));
        }
        construction non_hyphen: BoundRoot {
            element NonHyphen { value: lex BoundWord, }
            require value is Elf;
            form non_hyphen = "target" prefix("non-", lex(value));
        }
        construction singular_possessive: BoundRoot {
            element SingularPossessive { owner: lex BoundWord, }
            require owner is Tarmogoyf;
            form singular_possessive = "target" suffix(lex(owner), "'s");
        }
        construction plural_possessive: BoundRoot {
            element PluralPossessive { owner: lex BoundWord, }
            require owner is Players;
            form plural_possessive = "target" suffix(lex(owner), "'");
        }
        construction shared_boundary: SharedBoundary {
            element SharedBoundaryValue { value: lex BoundWord, }
            require value is Black;
            form shared_boundary = lex(value);
        }
        construction separated_boundary: DualBoundaryRoot {
            element SeparatedBoundary { value: SharedBoundary, }
            form separated_boundary = "target" value "tail";
        }
        construction adjacent_boundary: DualBoundaryRoot {
            element AdjacentBoundary { value: SharedBoundary, }
            form adjacent_boundary = "target" suffix(value, "'s");
        }
        construction prefixed_onset: PrefixHead {
            element PrefixedOnset { kind: lex Mode, value: lex BoundWord, }
            derive onset = value.onset;
            form lexical when kind is One = lex(kind) prefix("non", lex(value));
            form punctuation otherwise = lex(kind) prefix("2/", lex(value));
        }
        construction prefixed_article: PrefixArticle {
            element PrefixedArticle { head: PrefixHead, }
            derive onset = head.onset;
            form an when head.onset is Vowel = "an" head;
            form a otherwise = "a" head;
        }
        construction bound_owner: PossessiveOwner {
            element BoundOwner { value: lex BoundWord, }
            derive number = match value {
                Artifact => Values::Singular,
                Black => Values::Singular,
                Elf => Values::Singular,
                Daxos => Values::Singular,
                Tarmogoyf => Values::Singular,
                Players => Values::Plural,
                Merfolk => Values::Plural,
                Equipment => Values::Plural,
            };
            derive possessive_ending = value.possessive_ending;
            form bound_owner = lex(value);
        }
        construction derived_possessive: DerivedPossessiveRoot {
            element DerivedPossessive { owner: PossessiveOwner, }
            derive number = owner.number;
            form singular when number is Singular = suffix(owner, "'s");
            form plural_s when all(
                number is Plural,
                owner.possessive_ending is EndsInS
            ) = suffix(owner, "'");
            form plural_other otherwise = suffix(owner, "'s");
        }
        construction plus_two: CircumfixValue {
            element PlusTwoValue {}
            form plus_two = "+2";
        }
        construction circumfix_two: CircumfixValue {
            element CircumfixTwoValue {}
            form circumfix_two = "2";
        }
        construction circumfix_word_value: CircumfixValue {
            element CircumfixValueNode { value: lex CircumfixWord, }
            form circumfix_word_value = lex(value);
        }
        construction bracketed_value: CircumfixSingularRoot {
            element BracketedValue { value: CircumfixValue, }
            form bracketed_value = circumfix("[", value, "]");
        }
        construction braced_values: CircumfixSequenceRoot {
            element BracedValues {
                values: seq CircumfixValue separated by "}{",
            }
            require len(values) >= 1;
            form braced_values = circumfix("{", values, "}");
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
            form action = verb(VerbLexeme::Act);
        }
        construction act_onset: VerbHead {
            element ActOnset {}
            derive verb.agreement = Values::Bare;
            derive onset = verb.onset;
            form act_onset = verb(VerbLexeme::Act);
        }
        construction other_onset: VerbHead {
            element OtherOnset {}
            derive verb.agreement = Values::Bare;
            derive onset = verb.onset;
            form other_onset = verb(VerbLexeme::Other);
        }
        construction indefinite_verb: VerbArticle {
            element IndefiniteVerb { head: VerbHead, }
            derive onset = head.onset;
            form an when head.onset is Vowel = "an" head;
            form a otherwise = "a" head;
        }

        construction bare_be: BeSentence {
            element BareBe {}
            derive agreement = verb.agreement;
            derive verb.agreement = Values::Bare;
            form bare_be = verb(VerbLexeme::Be);
        }

        construction contextual: Predicate {
            element ContextualPredicate {}
            derive agreement = verb.agreement;
            form contextual = verb(VerbLexeme::Act);
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

        construction context_bound: ContextBound {
            element ContextBoundNode { pair: lex Pair, }
            form context_bound = lex(pair);
        }
        construction identity_guard: ContextBound {
            element IdentityGuard { spelling: identity SelfRef, }
            form identity_guard = identity(spelling);
        }
        construction named_head: NamedHead {
            element NamedHeadValue { spelling: identity SelfRef, }
            derive onset = spelling.onset;
            form named_head = identity(spelling);
        }
        construction named_article: NamedArticle {
            element NamedArticleValue { head: NamedHead, }
            derive onset = head.onset;
            form an when head.onset is Vowel = "an" head;
            form a otherwise = "a" head;
        }

        construction agreement: FeatureBound {
            element AgreementNode { marker: lex Marker, }
            derive agreement = verb.agreement;
            form agreement = lex(marker) verb(VerbLexeme::Act);
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
            form contextual_named = lex(agreement) verb(VerbLexeme::Act);
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

        construction marker: MarkerCategory {
            element WalkMarker { marker: lex Marker, }
            form marker = lex(marker);
        }

        construction r#raw_leaf: r#RawCategory {
            element r#RawNode {}
            form raw_leaf = "raw";
        }

        construction structural_atom: StructuralAtom {
            element StructuralAtomValue { marker: lex StructuralWord, }
            form structural_atom = lex(marker);
        }
        construction inline_structural: InlineStructural {
            element InlineStructuralValue {
                spelling: identity SelfRef,
                items: seq StructuralAtom,
            }
            require len(InlineStructuralValue.items) >= 2;
            form inline_structural = identity(spelling) items;
        }
        construction letter_atom: LetterAtom {
            element LetterAtomValue { letter: lex Letter, }
            form letter_atom = lex(letter);
        }

        abstract sum Choice { Child, MarkerCategory, }
        abstract sum AgreementChild { Child, }
        abstract product Holder {
            maybe: opt Child,
            items: seq Choice terminated by ",",
        }
        require len(Holder.items) >= 1;
        abstract product OptionalStructural { maybe: opt StructuralAtom, }
        abstract product TerminatedStructural {
            items: seq StructuralAtom terminated by "<T>",
        }
        abstract product SeparatedStructural {
            items: seq StructuralAtom separated by "<S>",
        }
        abstract product CombinedStructural {
            items: seq StructuralAtom separated by "<S>" terminated by "<T>",
        }
        abstract product PositionalStructural {
            items: seq StructuralAtom separated by position {
                pair = "<P>";
                first = "<F>";
                middle = "<M>";
                last = "<L>";
            } terminated by "<T>",
        }
        require len(PositionalStructural.items) >= 1;
        abstract product SentenceStructural {
            items: seq LetterAtom separated by " " terminated by ".",
        }
        require len(SentenceStructural.items) >= 1;
        abstract product PeriodSeparatedStructural {
            items: seq LetterAtom separated by ".",
        }
        require len(PeriodSeparatedStructural.items) = 2;
        abstract product SingleAtomSentenceStructural {
            items: seq LetterAtom terminated by ". ",
        }
        require len(SingleAtomSentenceStructural.items) = 2;
        abstract product MultiAtomSentenceStructural {
            items: seq LetterAtom terminated by "." " ",
        }
        require len(MultiAtomSentenceStructural.items) = 2;
        abstract product SingletonBlock {
            sentences: seq LetterAtom terminated by ".",
        }
        require len(SingletonBlock.sentences) = 1;
        abstract product BlockDocument {
            blocks: seq SingletonBlock separated by "\n",
        }
        require len(BlockDocument.blocks) = 2;
        abstract product ContinuationStructural {
            items: seq StructuralAtom separated by position {
                pair = " and ";
                first = ", ";
                middle = "; ";
                last = ", and ";
            },
        }
        require len(ContinuationStructural.items) >= 1;
        abstract product BoundedPositionalStructural {
            items: seq StructuralAtom separated by position {
                first = "<BF>";
                middle = "<BM>";
                last = "<BL>";
            },
        }
        require len(BoundedPositionalStructural.items) >= 3;
        require len(BoundedPositionalStructural.items) <= 4;
        abstract sum TraversalChoice { StructuralAtom, MarkerCategory, }
        abstract product TraversalHolder {
            maybe: opt TraversalChoice,
            items: seq TraversalChoice separated by ", ",
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        root HomogeneousSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root RelayedSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root SingletonSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root HomogeneousChoiceSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root RelayedChoiceSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root PartitionRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root OptionalGuardRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root OptionalVisitRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root BoundRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root DualBoundaryRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root DerivedPossessiveRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root PrefixArticle { punctuation = "."; eoi = true; standalone_render = true; }
        root CircumfixSingularRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root CircumfixSequenceRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root BeSentence { punctuation = "."; eoi = true; standalone_render = true; }
        root VerbArticle { punctuation = "."; eoi = true; standalone_render = true; }
        root NamedArticle { punctuation = "."; eoi = true; standalone_render = true; }
        root RenderChild { punctuation = "."; eoi = false; standalone_render = true; }
        root HygieneRoot { punctuation = "!"; eoi = true; standalone_render = true; }
        root r#RawCategory { punctuation = "?"; eoi = true; standalone_render = true; }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum VisitEvent {
        Mode(Mode),
        OptionalWord(OptionalWord),
        Marker(Marker),
        VisitorLexeme(VisitorLexeme),
        Token(u8),
        Sign(Sign),
        SignedNumber(Sign, u32),
        CardinalNumber(u32),
        ScalarNumber(u32),
        NonZeroScalarNumber(std::num::NonZeroU32),
        SelfRef(SelfRef),
        StructuralWord(StructuralWord),
        CircumfixWord(CircumfixWord),
        PlusTwo,
    }

    #[derive(Default)]
    struct RecordingVisitor(Vec<VisitEvent>);

    impl Visitor for RecordingVisitor {
        fn visit_mode(&mut self, mode: Mode) {
            self.0.push(VisitEvent::Mode(mode));
        }

        fn visit_optional_word(&mut self, word: OptionalWord) {
            self.0.push(VisitEvent::OptionalWord(word));
        }

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

        fn visit_cardinal_number(&mut self, number: &CardinalNumber) {
            self.0.push(VisitEvent::CardinalNumber(number.magnitude));
        }

        fn visit_scalar_number(&mut self, number: &ScalarNumber) {
            self.0.push(VisitEvent::ScalarNumber(number.magnitude));
        }

        fn visit_non_zero_scalar_number(&mut self, number: &NonZeroScalarNumber) {
            self.0
                .push(VisitEvent::NonZeroScalarNumber(number.magnitude));
        }

        fn visit_self_ref(&mut self, spelling: SelfRef) {
            self.0.push(VisitEvent::SelfRef(spelling));
        }

        fn visit_structural_word(&mut self, word: StructuralWord) {
            self.0.push(VisitEvent::StructuralWord(word));
        }

        fn visit_circumfix_word(&mut self, word: CircumfixWord) {
            self.0.push(VisitEvent::CircumfixWord(word));
        }

        fn visit_plus_two_value(&mut self, _value: &PlusTwoValue) {
            self.0.push(VisitEvent::PlusTwo);
        }
    }

    fn assert_generated_lexical_owner_abi() {
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
                    declaration: "VerbLexeme",
                    member: "Act",
                },
                Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::Bare,
                    onset: macro_ron::v2::Onset::Vowel,
                },
                LexicalProvenanceKind::Lexeme,
                "lexeme:VerbLexeme/Act/bare",
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
        let declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: macro_ron::v2::DeclarationIdentity::new(kind, "Destroy"),
                feature: macro_ron::v2::SurfaceFeature::Bare,
                onset: macro_ron::v2::Onset::Consonant,
            }),
        );
        assert_eq!(declaration.kind(), LexicalProvenanceKind::Lexeme);
        assert_eq!(
            declaration.stable_id(),
            "lexeme:keyword_action/Destroy/bare"
        );
        assert!(
            std::mem::size_of::<(Leaf, Option<LexicalOwner>)>() <= 80,
            "the scanner value/owner carrier is {} bytes, expected at most 80",
            std::mem::size_of::<(Leaf, Option<LexicalOwner>)>()
        );
        assert!(
            std::mem::size_of::<LexicalOwner>() <= 32,
            "the hot owner carrier is {} bytes, expected at most 32",
            std::mem::size_of::<LexicalOwner>()
        );

        let shared_declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: macro_ron::v2::DeclarationIdentity::new(kind, "Destroy"),
                feature: macro_ron::v2::SurfaceFeature::Bare,
                onset: macro_ron::v2::Onset::Consonant,
            }),
        );
        let shared_clone = shared_declaration.clone();
        let second_shared_clone = shared_declaration.clone();
        #[cfg(test)]
        LexicalOwner::reset_label_constructions();
        assert_eq!(
            shared_declaration.stable_id(),
            "lexeme:keyword_action/Destroy/bare"
        );
        assert_eq!(shared_clone.stable_id(), shared_declaration.stable_id());
        assert_eq!(
            second_shared_clone.stable_id(),
            shared_declaration.stable_id()
        );
        #[cfg(test)]
        assert_eq!(
            LexicalOwner::label_constructions(),
            1,
            "declaration-owner clones share one lazy stable-label allocation"
        );
        let equivalent_declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: macro_ron::v2::DeclarationIdentity::new(kind, "Destroy"),
                feature: macro_ron::v2::SurfaceFeature::Bare,
                onset: macro_ron::v2::Onset::Consonant,
            }),
        );
        assert_eq!(shared_declaration, shared_clone);
        assert_eq!(
            shared_declaration.cmp(&shared_clone),
            std::cmp::Ordering::Equal
        );
        assert_eq!(shared_declaration, equivalent_declaration);
        assert_eq!(
            shared_declaration.cmp(&equivalent_declaration),
            std::cmp::Ordering::Equal,
            "lazy cache state is not part of owner identity"
        );
        assert_eq!(
            format!("{shared_declaration:?}"),
            "LexicalOwner { kind: Lexeme, stable_id: \"lexeme:keyword_action/Destroy/bare\" }"
        );
    }

    fn assert_generated_runtime_abi(context: &ParseContext<'_>) {
        assert_generated_lexical_owner_abi();
        assert_eq!(
            surface_for_verb_lexeme(VerbLexeme::Act, Agreement::Bare),
            "act"
        );
        assert_eq!(
            agreement_for_mode(Mode::One),
            Agreement::ThirdPersonSingular
        );

        let kind = macro_ron::v2::DeclarationKind::KeywordAction;
        let position = macro_ron::v2::GrammarPosition::Verb;

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
            right_boundary: LexicalBoundary::Separated,
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
                prefix: PrefixPosition::None,
            },
            context,
        };
        let terminal = LexicalTerminal {
            matcher: Lexical::Mode,
            owner: LexicalOwnerTemplate::Vocab {
                declaration: "Mode",
            },
            right_boundary: LexicalBoundary::Separated,
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

        assert_generated_verb_scanner_abi(context);
    }

    fn assert_generated_verb_scanner_abi(context: &ParseContext<'_>) {
        let scan_verb = |text, case, constraint| {
            scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset: 0,
                        case,
                        prefix: if case == CasePosition::DocumentInitial {
                            PrefixPosition::None
                        } else {
                            PrefixPosition::WordOwnedSpace
                        },
                    },
                    context,
                },
                LexicalTerminal {
                    matcher: Lexical::Verb(VerbLexeme::Act, constraint),
                    owner: LexicalOwnerTemplate::Lexeme {
                        declaration: "VerbLexeme",
                        member: "Act",
                    },
                    right_boundary: LexicalBoundary::Separated,
                },
            )
        };
        for (text, case, agreement, owner) in [
            (
                "Act",
                CasePosition::DocumentInitial,
                Agreement::Bare,
                "lexeme:VerbLexeme/Act/bare",
            ),
            (
                "Acts",
                CasePosition::DocumentInitial,
                Agreement::ThirdPersonSingular,
                "lexeme:VerbLexeme/Act/third_person_singular",
            ),
            (
                " act",
                CasePosition::Continuation,
                Agreement::Bare,
                "lexeme:VerbLexeme/Act/bare",
            ),
            (
                " acts",
                CasePosition::Continuation,
                Agreement::ThirdPersonSingular,
                "lexeme:VerbLexeme/Act/third_person_singular",
            ),
        ] {
            let matches = scan_verb(text, case, FeatureConstraint::Exact(agreement));
            assert!(matches!(
                matches.as_slice(),
                [LexicalMatch {
                    value: Leaf::Verb {
                        lexeme: VerbLexeme::Act,
                        agreement: actual,
                        onset: macro_ron::v2::Onset::Vowel,
                    },
                    owner: Some(_),
                    ..
                }] if *actual == agreement
            ));
            assert_eq!(matches[0].owner.as_ref().unwrap().stable_id(), owner);
        }
        assert!(
            scan_verb(
                "Actuator",
                CasePosition::DocumentInitial,
                FeatureConstraint::Any,
            )
            .is_empty()
        );

        assert_generated_verb_collision_abi(context);
        assert_generated_irregular_verb_abi(context);
    }

    fn assert_generated_verb_collision_abi(context: &ParseContext<'_>) {
        let collision_terminal = |lexeme, member, constraint| LexicalTerminal {
            matcher: Lexical::Verb(lexeme, constraint),
            owner: LexicalOwnerTemplate::Lexeme {
                declaration: "VerbLexeme",
                member,
            },
            right_boundary: LexicalBoundary::Separated,
        };
        let collision_input = ScanInput {
            text: "Same",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            context,
        };
        let same_feature_readings = scan_lexical(
            &collision_input,
            collision_terminal(VerbLexeme::Collide, "Collide", FeatureConstraint::Any),
        );
        assert_eq!(same_feature_readings.len(), 2);
        assert_eq!(
            same_feature_readings
                .iter()
                .map(|matched| matched.owner.as_ref().unwrap().stable_id())
                .collect::<Vec<_>>(),
            [
                "lexeme:VerbLexeme/Collide/bare",
                "lexeme:VerbLexeme/Collide/third_person_singular",
            ]
        );
        for (constraint, expected_owner) in [
            (
                FeatureConstraint::Exact(Agreement::Bare),
                "lexeme:VerbLexeme/Collide/bare",
            ),
            (
                FeatureConstraint::Exact(Agreement::ThirdPersonSingular),
                "lexeme:VerbLexeme/Collide/third_person_singular",
            ),
        ] {
            let exact = scan_lexical(
                &collision_input,
                collision_terminal(VerbLexeme::Collide, "Collide", constraint),
            );
            assert_eq!(exact.len(), 1, "Exact must retain one colliding feature");
            assert_eq!(exact[0].owner.as_ref().unwrap().stable_id(), expected_owner);
        }
        let other_member_readings = scan_lexical(
            &collision_input,
            collision_terminal(VerbLexeme::Other, "Other", FeatureConstraint::Any),
        );
        assert!(matches!(
            other_member_readings.as_slice(),
            [LexicalMatch {
                value: Leaf::Verb {
                    lexeme: VerbLexeme::Other,
                    agreement: Agreement::Bare,
                    onset: macro_ron::v2::Onset::Consonant,
                },
                owner: Some(_),
                ..
            }]
        ));
        assert_eq!(
            other_member_readings[0].owner.as_ref().unwrap().stable_id(),
            "lexeme:VerbLexeme/Other/bare"
        );
    }

    fn assert_generated_irregular_verb_abi(context: &ParseContext<'_>) {
        let bare_be = scan_lexical(
            &ScanInput {
                text: "Are.",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                context,
            },
            LexicalTerminal {
                matcher: Lexical::Verb(VerbLexeme::Be, FeatureConstraint::Exact(Agreement::Bare)),
                owner: LexicalOwnerTemplate::Lexeme {
                    declaration: "VerbLexeme",
                    member: "Be",
                },
                right_boundary: LexicalBoundary::Separated,
            },
        );
        assert_eq!(bare_be.len(), 1);
        assert_eq!(
            bare_be[0].owner.as_ref().unwrap().stable_id(),
            "lexeme:VerbLexeme/Be/bare"
        );
        let built = build(
            RuleId::BeSentenceBareBe,
            &[BuildValue::Leaf(bare_be[0].value.clone())],
            context,
        )
        .expect("the exact Bare Be scanner reading builds its generated rule");
        let BuildValue::BeSentence(be_sentence, Agreement::Bare) = built else {
            panic!("Bare Be rule produced the wrong generated category value")
        };
        let (rendered, claims) = render_be_sentence_with_claims(&be_sentence, context);
        assert_eq!(rendered, "Are.");
        assert!(claims.iter().any(|claim| {
            claim.owner.stable_id() == "lexeme:VerbLexeme/Be/bare"
                && &rendered[claim.start..claim.end] == "Are"
        }));
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
                position: ScanPosition {
                    byte_offset,
                    case,
                    prefix: if byte_offset == 0 {
                        PrefixPosition::None
                    } else {
                        PrefixPosition::WordOwnedSpace
                    },
                },
                context,
            };
            let terminal = LexicalTerminal {
                matcher: Lexical::Literal(literal),
                owner: LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::FormLiteral,
                    stable_id: "compiled-consumer/pre-punctuation-literal",
                },
                right_boundary: LexicalBoundary::Separated,
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
                prefix: PrefixPosition::SurfaceOwned,
            },
            context,
        };
        let terminal = LexicalTerminal {
            matcher: Lexical::Literal(punctuation),
            owner: LexicalOwnerTemplate::Static {
                kind: LexicalProvenanceKind::FormLiteral,
                stable_id: "compiled-consumer/punctuation",
            },
            right_boundary: LexicalBoundary::Separated,
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

    pub(super) fn assert_invariant_public_boundary() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let guarded = GuardedChild::new(Mode::One, Box::new(Child::Bare(BareChild)))
            .expect("valid finite-domain values construct the recursive product");
        assert_eq!(guarded.mode(), Mode::One);
        assert!(matches!(guarded.child(), Child::Bare(BareChild)));
        assert!(
            GuardedChild::new(Mode::Many, Box::new(Child::Bare(BareChild))).is_none(),
            "invalid vocabulary membership is rejected",
        );
        let vocab_rejection = GuardedChild::try_new(Mode::Many, Box::new(Child::Bare(BareChild)))
            .expect_err("the checked seam retains vocabulary rejection identity");
        assert_eq!(vocab_rejection.owner(), "Child");
        assert_eq!(vocab_rejection.role(), "guarded");
        assert_eq!(
            vocab_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "any(all(mode is One, child is Bare), all(mode is Many, child is Third))",
            },
        );
        assert!(
            GuardedChild::new(Mode::One, Box::new(Child::Third(ThirdChild))).is_none(),
            "invalid category membership is rejected",
        );
        let category_rejection =
            GuardedChild::try_new(Mode::One, Box::new(Child::Third(ThirdChild)))
                .expect_err("the checked seam retains category rejection identity");
        assert_eq!(category_rejection.owner(), "Child");
        assert_eq!(category_rejection.role(), "guarded");
        assert_eq!(category_rejection.violation(), vocab_rejection.violation());
        let second_branch = GuardedChild::try_new(Mode::Many, Box::new(Child::Third(ThirdChild)))
            .expect("the second distinguishable DNF branch remains valid");
        assert_eq!(second_branch.mode(), Mode::Many);
        assert!(matches!(second_branch.child(), Child::Third(ThirdChild)));

        let identity = IdentityGuard::new(SelfRef::Full, &context)
            .expect("the canonical context identity is always valid");
        assert_eq!(identity.spelling(), SelfRef::Full);
        assert!(
            IdentityGuard::new(SelfRef::Abbreviated, &context).is_none(),
            "a colliding noncanonical context identity is rejected",
        );
        let context_rejection = IdentityGuard::try_new(SelfRef::Abbreviated, &context)
            .expect_err("the checked seam retains context rejection identity");
        assert_eq!(context_rejection.owner(), "ContextBound");
        assert_eq!(context_rejection.role(), "identity_guard");
        assert_eq!(
            context_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "spelling valid in context",
            },
        );
    }

    pub(super) fn assert_guarded_form_partition_boundaries() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let rules = [
            RuleId::PartitionRootPartitionedFirst,
            RuleId::PartitionRootPartitionedSecond,
            RuleId::PartitionRootPartitionedFallback,
        ];
        let values = [Partition::First, Partition::Second, Partition::Other];
        let literals = ["alpha", "beta", "omega"];
        let expected = ["Alpha first.", "Beta second.", "Omega other."];

        for (rule_index, rule) in rules.into_iter().enumerate() {
            for (value_index, value) in values.into_iter().enumerate() {
                let built = build(
                    rule,
                    &[
                        BuildValue::Leaf(Leaf::Literal(literals[rule_index])),
                        BuildValue::Leaf(Leaf::Partition(value)),
                    ],
                    &context,
                );
                if rule_index == value_index {
                    let BuildValue::PartitionRoot(root) =
                        built.expect("the rule accepts its exact finite partition")
                    else {
                        panic!("the selected rule builds its declared category")
                    };
                    assert_eq!(Render::render(&root, &context), expected[value_index]);
                } else {
                    assert!(
                        built.is_none(),
                        "rule {rule_index} accepted same-shape partition value {value_index}",
                    );
                }
            }
        }
    }

    pub(super) fn assert_optional_vocab_guards_and_invariants() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let absent =
            OptionalGuarded::new(None, Mode::One).expect("the explicit absent branch constructs");
        assert_eq!(
            Render::render(&OptionalGuardRoot::OptionalGuarded(absent), &context),
            "One.",
        );
        let present = OptionalGuarded::new(Some(OptionalWord::That), Mode::Many)
            .expect("the allowed present vocabulary member constructs");
        assert_eq!(
            Render::render(&OptionalGuardRoot::OptionalGuarded(present), &context),
            "That many.",
        );
        for (word, mode) in [
            (None, Mode::Many),
            (Some(OptionalWord::That), Mode::One),
            (Some(OptionalWord::Those), Mode::One),
            (Some(OptionalWord::Those), Mode::Many),
        ] {
            assert!(
                OptionalGuarded::new(word, mode).is_none(),
                "the closed checked product rejects an unlisted optional-vocabulary cross-product",
            );
        }

        let present_helper = build(
            RuleId::OptionalGuardedWordOptionalPresent,
            &[BuildValue::Leaf(Leaf::OptionalWord(OptionalWord::That))],
            &context,
        )
        .expect("the optional structural helper folds the present vocab value");
        let that_rule = build(
            RuleId::OptionalGuardRootOptionalGuardedThat,
            &[
                present_helper.clone(),
                BuildValue::Leaf(Leaf::Mode(Mode::Many)),
            ],
            &context,
        )
        .expect("the membership-guarded scanner rule accepts its present value");
        assert!(matches!(that_rule, BuildValue::OptionalGuardRoot(_)));
        assert!(
            build(
                RuleId::OptionalGuardRootOptionalGuardedFallback,
                &[present_helper, BuildValue::Leaf(Leaf::Mode(Mode::Many)),],
                &context,
            )
            .is_none(),
            "the scanner fallback uses the same sealed guard and rejects the guarded member",
        );
        let absent_rule = build(RuleId::OptionalGuardedWordOptionalAbsent, &[], &context);
        assert!(
            absent_rule.is_some(),
            "the optional structural helper exposes the absent build leaf",
        );
    }

    pub(super) fn assert_optional_vocab_visitor_guard_selection() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        for (root, rendered, expected) in [
            (
                OptionalVisitRoot::OptionalVisited(OptionalVisited {
                    word: None,
                    mode: Mode::One,
                }),
                "One.",
                vec![VisitEvent::Mode(Mode::One)],
            ),
            (
                OptionalVisitRoot::OptionalVisited(OptionalVisited {
                    word: Some(OptionalWord::That),
                    mode: Mode::Many,
                }),
                "That many.",
                vec![
                    VisitEvent::OptionalWord(OptionalWord::That),
                    VisitEvent::Mode(Mode::Many),
                ],
            ),
            (
                OptionalVisitRoot::OptionalVisited(OptionalVisited {
                    word: Some(OptionalWord::Those),
                    mode: Mode::Many,
                }),
                "Many those.",
                vec![
                    VisitEvent::Mode(Mode::Many),
                    VisitEvent::OptionalWord(OptionalWord::Those),
                ],
            ),
        ] {
            assert_eq!(Render::render(&root, &context), rendered);
            let mut visitor = RecordingVisitor::default();
            visitor.visit_optional_visit_root(&root);
            assert_eq!(
                visitor.0, expected,
                "visitor form selection must use the same absent/member/fallback partition as rendering",
            );
        }
    }

    pub(super) fn assert_structural_product_public_boundary() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let inline_member = StructuralAtom::StructuralAtom(StructuralAtomValue {
            marker: StructuralWord::Alpha,
        });
        let inline_rejection =
            InlineStructuralValue::try_new(SelfRef::Full, vec![inline_member], &context)
                .expect_err("an inline element retains its product-level length rejection");
        assert_eq!(inline_rejection.owner(), "InlineStructuralValue");
        assert_eq!(inline_rejection.role(), "items");
        assert_eq!(
            inline_rejection.violation(),
            &BuildViolation::Length {
                minimum: 2,
                maximum: None,
                actual: 1,
            },
        );
        let invariant_rejection = InlineStructuralValue::try_new(
            SelfRef::Abbreviated,
            vec![
                StructuralAtom::StructuralAtom(StructuralAtomValue {
                    marker: StructuralWord::Alpha,
                }),
                StructuralAtom::StructuralAtom(StructuralAtomValue {
                    marker: StructuralWord::Beta,
                }),
            ],
            &context,
        )
        .expect_err("the authored context invariant remains independently attributed");
        assert_eq!(invariant_rejection.owner(), "InlineStructural");
        assert_eq!(invariant_rejection.role(), "inline_structural");
        assert_eq!(
            invariant_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "spelling valid in context",
            },
        );

        assert!(
            Holder::new(None, vec![]).is_none(),
            "the normalized nonempty bound rejects an empty structural sequence",
        );
        let rejection = Holder::try_new(None, vec![])
            .expect_err("the checked seam retains structural cardinality identity");
        assert_eq!(rejection.owner(), "Holder");
        assert_eq!(rejection.role(), "items");
        assert_eq!(
            rejection.violation(),
            &BuildViolation::Length {
                minimum: 1,
                maximum: None,
                actual: 0,
            },
        );
        assert_eq!(
            rejection.to_string(),
            "Holder.items: length 0 violates minimum 1",
        );
        let invalid_owner_children = [
            BuildValue::StructuralAtom(StructuralAtom::StructuralAtom(StructuralAtomValue {
                marker: StructuralWord::Alpha,
            })),
            BuildValue::Leaf(Leaf::Literal("<BF>")),
            BuildValue::BoundedPositionalStructuralItemsSequence(vec![]),
        ];
        let owner_rejection = build_checked(
            RuleId::BoundedPositionalStructuralItemsSequenceThreePlus,
            &invalid_owner_children,
            &context,
        )
        .expect_err("the generated owner rule retains its checked-constructor rejection");
        assert_eq!(owner_rejection.owner(), "BoundedPositionalStructural");
        assert_eq!(owner_rejection.role(), "items");
        assert_eq!(
            owner_rejection.violation(),
            &BuildViolation::Length {
                minimum: 3,
                maximum: Some(4),
                actual: 1,
            },
        );
        assert!(
            build(
                RuleId::BoundedPositionalStructuralItemsSequenceThreePlus,
                &invalid_owner_children,
                &context,
            )
            .is_none(),
            "the compatibility build API remains lossy",
        );
        let expected = Choice::Child(Child::Bare(BareChild));
        let holder = Holder::new(None, vec![expected.clone()])
            .expect("one structural member satisfies the normalized bound");
        assert_eq!(
            holder.items(),
            std::slice::from_ref(&expected),
            "the borrowed slice retains source order",
        );
    }

    fn parse_structural(
        category: Category,
        text: &str,
        context: &ParseContext<'_>,
    ) -> engine::Forest<RuleId, Leaf, LexicalOwner> {
        try_parse_structural(category, text, context).unwrap_or_else(|failure| {
            panic!(
                "structural grammar failed at {} for {category:?} on {text:?}",
                failure.offset
            )
        })
    }

    fn try_parse_structural(
        category: Category,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<
        engine::Forest<RuleId, Leaf, LexicalOwner>,
        engine::ChartFailure<Category, LexicalTerminal>,
    > {
        engine::parse_with_state(
            RULES,
            category,
            text.len(),
            &ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            |terminal, offset, position, suppress_right_boundary| {
                debug_assert_eq!(offset, position.byte_offset);
                let terminal = if suppress_right_boundary {
                    terminal.suppress_right_boundary()
                } else {
                    terminal
                };
                let scan_position = terminal.position_before(*position);
                scan_lexical(
                    &ScanInput {
                        text,
                        position: scan_position,
                        context,
                    },
                    terminal,
                )
                .into_iter()
                .map(|lexical_match| {
                    let end = lexical_match.end;
                    EngineStatefulLexicalMatch {
                        lexical: EngineLexicalMatch {
                            end,
                            value: lexical_match.value,
                            owner: lexical_match.owner,
                        },
                        state: terminal.position_after(*position, end),
                    }
                })
                .collect()
            },
            |_, _, _| true,
        )
    }

    fn assert_structural_accepts(category: Category, texts: &[&str]) {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        for text in texts {
            let forest = parse_structural(category, text, &context);
            assert!(
                forest.accepted_root_ids().next().is_some(),
                "{category:?} accepts {text:?} through the actual Earley chart",
            );
        }
    }

    pub(super) fn assert_structural_bnf_boundaries() {
        let helper_rule = RuleId::OptionalStructuralMaybeOptionalAbsent;
        assert_eq!(helper_rule.public_construction(), None);
        assert_eq!(helper_rule.owner(), "OptionalStructural");
        assert_eq!(helper_rule.role(), Some("maybe"));
        assert_eq!(helper_rule.state(), "optional_absent");

        assert_structural_accepts(Category::OptionalStructural, &["", "Alpha"]);
        assert_structural_accepts(
            Category::TerminatedStructural,
            &[
                "",
                "Alpha<T>",
                "Alpha<T>beta<T>",
                "Alpha<T>beta<T>gamma<T>delta<T>",
            ],
        );
        assert_structural_accepts(
            Category::SeparatedStructural,
            &["", "Alpha", "Alpha<S>beta", "Alpha<S>beta<S>gamma<S>delta"],
        );
        assert_structural_accepts(
            Category::CombinedStructural,
            &[
                "",
                "Alpha<T>",
                "Alpha<T><S>beta<T>",
                "Alpha<T><S>beta<T><S>gamma<T><S>delta<T>",
            ],
        );
        assert_structural_accepts(
            Category::PositionalStructural,
            &[
                "Alpha<T>",
                "Alpha<T><P>beta<T>",
                "Alpha<T><F>beta<T><L>gamma<T>",
                "Alpha<T><F>beta<T><M>gamma<T><L>delta<T>",
            ],
        );

        let atom = |marker| StructuralAtom::StructuralAtom(StructuralAtomValue { marker });
        let alpha = atom(StructuralWord::Alpha);
        let absent = build(
            RuleId::OptionalStructuralMaybeOptionalAbsent,
            &[],
            &ParseContext::default(),
        )
        .expect("optional absent helper folds");
        assert!(matches!(
            absent,
            BuildValue::OptionalStructuralMaybeOptional(None)
        ));
        let present = build(
            RuleId::OptionalStructuralMaybeOptionalPresent,
            &[BuildValue::StructuralAtom(alpha.clone())],
            &ParseContext::default(),
        )
        .expect("optional present helper folds");
        assert!(matches!(
            present,
            BuildValue::OptionalStructuralMaybeOptional(Some(_))
        ));

        let four = [
            alpha,
            atom(StructuralWord::Beta),
            atom(StructuralWord::Gamma),
            atom(StructuralWord::Delta),
        ];
        let tail = build(
            RuleId::CombinedStructuralItemsSequenceSingleton,
            &[
                BuildValue::StructuralAtom(four[3].clone()),
                BuildValue::Leaf(Leaf::Literal("<T>")),
            ],
            &ParseContext::default(),
        )
        .expect("singleton sequence helper folds");
        let mut built = tail;
        for item in four[..3].iter().rev() {
            built = build(
                RuleId::CombinedStructuralItemsSequenceRecursive,
                &[
                    BuildValue::StructuralAtom(item.clone()),
                    BuildValue::Leaf(Leaf::Literal("<T>")),
                    BuildValue::Leaf(Leaf::Literal("<S>")),
                    built,
                ],
                &ParseContext::default(),
            )
            .expect("recursive sequence helper folds");
        }
        let BuildValue::CombinedStructuralItemsSequence(values) = built else {
            panic!("recursive fold returns the one role-keyed carrier")
        };
        assert_eq!(values, four, "recursive folds preserve source order");
    }

    fn collect_first_family_claims(
        forest: &engine::Forest<RuleId, Leaf, LexicalOwner>,
        node: engine::NodeId,
        claims: &mut Vec<(usize, usize, String)>,
    ) {
        for child in &forest.node(node).families[0].children {
            match child {
                engine::Child::Node(child) => collect_first_family_claims(forest, *child, claims),
                engine::Child::Lexical(lexical) => {
                    if let Some(owner) = &lexical.owner {
                        claims.push((
                            lexical.span.start,
                            lexical.span.end,
                            owner.stable_id().to_owned(),
                        ));
                    }
                }
            }
        }
    }

    fn assert_exact_partition(text: &str, claims: &[(usize, usize, String)]) {
        let mut next = 0;
        for (start, end, _) in claims {
            assert_eq!(*start, next, "claim gap or overlap in {claims:?}");
            assert!(*end > *start, "zero-width lexical claim in {claims:?}");
            next = *end;
        }
        assert_eq!(next, text.len(), "claim partition does not cover {text:?}");
    }

    fn render_structural<T>(
        value: &T,
        context: &ParseContext<'_>,
        render: fn(&mut Writer<'_>, &T, &ParseContext<'_>),
    ) -> (String, Vec<(usize, usize, String)>) {
        let mut claims = Vec::new();
        let mut writer = Writer::collecting(&mut claims);
        render(&mut writer, value, context);
        let text = writer.finish();
        let claims = claims
            .into_iter()
            .map(|claim| (claim.start, claim.end, claim.owner.stable_id().to_owned()))
            .collect();
        (text, claims)
    }

    fn assert_structural_surface_lookup() {
        let pair = sequence_separator(SequenceOwner::PositionalStructuralItems, 2, 0);
        let three_first = sequence_separator(SequenceOwner::PositionalStructuralItems, 3, 0);
        let three_last = sequence_separator(SequenceOwner::PositionalStructuralItems, 3, 1);
        let four_middle = sequence_separator(SequenceOwner::PositionalStructuralItems, 4, 1);
        assert_eq!(pair[0].text, "<P>");
        assert_eq!(three_first[0].text, "<F>");
        assert_eq!(three_last[0].text, "<L>");
        assert_eq!(four_middle[0].text, "<M>");
        assert!(
            sequence_separator(SequenceOwner::PositionalStructuralItems, 4, usize::MAX,).is_empty()
        );
        assert_eq!(
            pair[0].stable_id,
            "structural:PositionalStructural/items/separator/pair/0",
        );
        assert_eq!(
            sequence_terminator(SequenceOwner::PositionalStructuralItems)[0].stable_id,
            "structural:PositionalStructural/items/terminator/0",
        );

        let single = sequence_terminator(SequenceOwner::SingleAtomSentenceStructuralItems);
        let multi = sequence_terminator(SequenceOwner::MultiAtomSentenceStructuralItems);
        assert_eq!(single.len(), 1);
        assert_eq!(multi.len(), 2);

        let writer_state_after = |atoms: &[FixedSurfaceAtom]| {
            let mut writer = Writer::new();
            writer.word("a");
            for atom in atoms {
                writer.structural_surface(atom.text, atom.transition);
            }
            (writer.case, writer.prefix)
        };
        assert_eq!(
            writer_state_after(single),
            (CasePosition::SentenceInitial, PrefixPosition::SurfaceOwned),
        );
        assert_eq!(writer_state_after(multi), writer_state_after(single));

        let scanner_state_after = |atoms: &[FixedSurfaceAtom]| {
            atoms.iter().fold(
                ScanPosition {
                    byte_offset: 1,
                    case: CasePosition::Continuation,
                    prefix: PrefixPosition::WordOwnedSpace,
                },
                |position, atom| {
                    atom.transition
                        .position_after(position, position.byte_offset + atom.text.len())
                },
            )
        };
        assert_eq!(
            scanner_state_after(single),
            ScanPosition {
                byte_offset: 3,
                case: CasePosition::SentenceInitial,
                prefix: PrefixPosition::SurfaceOwned,
            },
        );
        assert_eq!(scanner_state_after(multi), scanner_state_after(single));

        let period_separator =
            sequence_separator(SequenceOwner::PeriodSeparatedStructuralItems, 2, 0);
        assert_eq!(
            period_separator[0].transition,
            StructuralTransition::Preserve
        );

        for (member_count, edge_index) in [
            (0, 0),
            (1, 0),
            (2, 1),
            (3, 0),
            (usize::MAX, 0),
            (2, usize::MAX),
        ] {
            assert!(
                sequence_separator(
                    SequenceOwner::PeriodSeparatedStructuralItems,
                    member_count,
                    edge_index,
                )
                .is_empty(),
                "uniform lookup accepts invalid count/edge ({member_count}, {edge_index})",
            );
        }

        for (member_count, edge_index) in [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 2),
            (5, 0),
            (usize::MAX, 0),
            (4, usize::MAX),
        ] {
            assert!(
                sequence_separator(
                    SequenceOwner::BoundedPositionalStructuralItems,
                    member_count,
                    edge_index,
                )
                .is_empty(),
                "positional lookup accepts invalid count/edge ({member_count}, {edge_index})",
            );
        }
        assert_eq!(
            sequence_separator(SequenceOwner::BoundedPositionalStructuralItems, 3, 0)[0].text,
            "<BF>",
        );
        assert_eq!(
            sequence_separator(SequenceOwner::BoundedPositionalStructuralItems, 4, 1)[0].text,
            "<BM>",
        );
        assert_eq!(
            sequence_separator(SequenceOwner::BoundedPositionalStructuralItems, 4, 2)[0].text,
            "<BL>",
        );
    }

    fn assert_sentence_and_block_partitions(context: &ParseContext<'_>) {
        let letter = |letter| LetterAtom::LetterAtom(LetterAtomValue { letter });
        let sentence = SentenceStructural::new(vec![letter(Letter::A), letter(Letter::B)])
            .expect("two sentence members satisfy the nonempty bound");
        let (text, rendered_claims) =
            render_structural(&sentence, context, render_sentence_structural);
        assert_eq!(text, "A. B.");
        assert_eq!(
            rendered_claims,
            [
                (0, 1, "vocab:Letter/A".to_owned()),
                (
                    1,
                    2,
                    "structural:SentenceStructural/items/terminator/0".to_owned(),
                ),
                (
                    2,
                    3,
                    "structural:SentenceStructural/items/separator/uniform/0".to_owned(),
                ),
                (3, 4, "vocab:Letter/B".to_owned()),
                (
                    4,
                    5,
                    "structural:SentenceStructural/items/terminator/0".to_owned(),
                ),
            ],
        );
        assert_exact_partition(&text, &rendered_claims);

        let forest = parse_structural(Category::SentenceStructural, &text, context);
        let root = forest
            .accepted_root_ids()
            .next()
            .expect("one accepted sentence root");
        let mut parsed_claims = Vec::new();
        collect_first_family_claims(&forest, root, &mut parsed_claims);
        assert_eq!(parsed_claims, rendered_claims);
        assert_exact_partition(&text, &parsed_claims);

        let period_separated =
            PeriodSeparatedStructural::new(vec![letter(Letter::A), letter(Letter::B)])
                .expect("two period-separated members satisfy the exact bound");
        let (text, rendered_claims) = render_structural(
            &period_separated,
            context,
            render_period_separated_structural,
        );
        assert_eq!(
            text, "A.b",
            "a separator period does not establish sentence case"
        );
        let forest = parse_structural(Category::PeriodSeparatedStructural, &text, context);
        let root = forest
            .accepted_root_ids()
            .next()
            .expect("the lowercase continuation accepts through generated scan state");
        let mut parsed_claims = Vec::new();
        collect_first_family_claims(&forest, root, &mut parsed_claims);
        assert_eq!(parsed_claims, rendered_claims);
        assert_exact_partition(&text, &parsed_claims);

        let single_atom =
            SingleAtomSentenceStructural::new(vec![letter(Letter::A), letter(Letter::B)])
                .expect("two single-atom terminated members satisfy the exact bound");
        let multi_atom =
            MultiAtomSentenceStructural::new(vec![letter(Letter::A), letter(Letter::B)])
                .expect("two multi-atom terminated members satisfy the exact bound");
        let (single_text, single_claims) = render_structural(
            &single_atom,
            context,
            render_single_atom_sentence_structural,
        );
        let (multi_text, multi_claims) =
            render_structural(&multi_atom, context, render_multi_atom_sentence_structural);
        assert_eq!(single_text, "A. B. ");
        assert_eq!(multi_text, single_text);
        for (category, text, rendered_claims) in [
            (
                Category::SingleAtomSentenceStructural,
                single_text,
                single_claims,
            ),
            (
                Category::MultiAtomSentenceStructural,
                multi_text,
                multi_claims,
            ),
        ] {
            let forest = parse_structural(category, &text, context);
            let root = forest
                .accepted_root_ids()
                .next()
                .expect("period-bearing terminator establishes sentence case");
            let mut parsed_claims = Vec::new();
            collect_first_family_claims(&forest, root, &mut parsed_claims);
            assert_eq!(parsed_claims, rendered_claims);
            assert_exact_partition(&text, &parsed_claims);
        }

        let block_a = SingletonBlock::new(vec![letter(Letter::A)]).expect("one sentence in block");
        let block_b = SingletonBlock::new(vec![letter(Letter::B)]).expect("one sentence in block");
        let document = BlockDocument::new(vec![block_a, block_b]).expect("two blocks in document");
        let (text, rendered_claims) = render_structural(&document, context, render_block_document);
        assert_eq!(text, "A.\nB.");
        assert_eq!(
            rendered_claims,
            [
                (0, 1, "vocab:Letter/A".to_owned()),
                (
                    1,
                    2,
                    "structural:SingletonBlock/sentences/terminator/0".to_owned(),
                ),
                (
                    2,
                    3,
                    "structural:BlockDocument/blocks/separator/uniform/0".to_owned(),
                ),
                (3, 4, "vocab:Letter/B".to_owned()),
                (
                    4,
                    5,
                    "structural:SingletonBlock/sentences/terminator/0".to_owned(),
                ),
            ],
        );
        assert_exact_partition(&text, &rendered_claims);

        let forest = parse_structural(Category::BlockDocument, &text, context);
        let root = forest
            .accepted_root_ids()
            .next()
            .expect("one accepted document root");
        let mut parsed_claims = Vec::new();
        collect_first_family_claims(&forest, root, &mut parsed_claims);
        assert_eq!(parsed_claims, rendered_claims);
        assert_exact_partition(&text, &parsed_claims);
    }

    pub(super) fn assert_structural_render_scan_ownership_and_traversal() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let atom = |marker| StructuralAtom::StructuralAtom(StructuralAtomValue { marker });
        let alpha = atom(StructuralWord::Alpha);
        let beta = atom(StructuralWord::Beta);
        let gamma = atom(StructuralWord::Gamma);
        let delta = atom(StructuralWord::Delta);

        let cases = [
            render_structural(
                &TerminatedStructural {
                    items: vec![alpha.clone(), beta.clone()],
                },
                &context,
                render_terminated_structural,
            ),
            render_structural(
                &SeparatedStructural {
                    items: vec![alpha.clone(), beta.clone()],
                },
                &context,
                render_separated_structural,
            ),
            render_structural(
                &CombinedStructural {
                    items: vec![alpha.clone(), beta.clone()],
                },
                &context,
                render_combined_structural,
            ),
            render_structural(
                &PositionalStructural::new(vec![alpha.clone(), beta.clone()])
                    .expect("pair is allowed"),
                &context,
                render_positional_structural,
            ),
            render_structural(
                &PositionalStructural::new(vec![alpha.clone(), beta.clone(), gamma.clone()])
                    .expect("three members are allowed"),
                &context,
                render_positional_structural,
            ),
            render_structural(
                &PositionalStructural::new(vec![
                    alpha.clone(),
                    beta.clone(),
                    gamma.clone(),
                    delta.clone(),
                ])
                .expect("four members are allowed"),
                &context,
                render_positional_structural,
            ),
        ];
        assert_eq!(
            cases
                .iter()
                .map(|(text, _)| text.as_str())
                .collect::<Vec<_>>(),
            [
                "Alpha<T>beta<T>",
                "Alpha<S>beta",
                "Alpha<T><S>beta<T>",
                "Alpha<T><P>beta<T>",
                "Alpha<T><F>beta<T><L>gamma<T>",
                "Alpha<T><F>beta<T><M>gamma<T><L>delta<T>",
            ],
        );
        for (text, claims) in &cases {
            assert_exact_partition(text, claims);
        }

        assert_sentence_and_block_partitions(&context);

        assert_structural_surface_lookup();

        let continuation = ContinuationStructural::new(vec![alpha.clone(), beta.clone()])
            .expect("pair continuation is allowed");
        let (text, claims) =
            render_structural(&continuation, &context, render_continuation_structural);
        assert_eq!(text, "Alpha and beta");
        assert_eq!((claims[2].0, claims[2].1), (10, 14));
        assert_exact_partition(&text, &claims);
        let forest = parse_structural(Category::ContinuationStructural, &text, &context);
        let root = forest
            .accepted_root_ids()
            .next()
            .expect("non-sentence positional continuation parses through generated state");
        let mut parsed_claims = Vec::new();
        collect_first_family_claims(&forest, root, &mut parsed_claims);
        assert_eq!(parsed_claims, claims);
        assert_exact_partition(&text, &parsed_claims);

        let empty_traversal = TraversalHolder {
            maybe: None,
            items: vec![],
        };
        let mut empty_visitor = RecordingVisitor::default();
        walk_traversal_holder(&mut empty_visitor, &empty_traversal);
        assert_eq!(
            empty_visitor.0,
            [],
            "None and an empty sequence produce no value, helper, or policy callbacks",
        );

        let traversal = TraversalHolder {
            maybe: Some(TraversalChoice::StructuralAtom(alpha)),
            items: vec![
                TraversalChoice::StructuralAtom(beta),
                TraversalChoice::MarkerCategory(MarkerCategory::Marker(WalkMarker {
                    marker: Marker::One,
                })),
            ],
        };
        let mut visitor = RecordingVisitor::default();
        walk_traversal_holder(&mut visitor, &traversal);
        assert_eq!(
            visitor.0,
            [
                VisitEvent::StructuralWord(StructuralWord::Alpha),
                VisitEvent::StructuralWord(StructuralWord::Beta),
                VisitEvent::Marker(Marker::One),
            ],
            "the complete callback log contains only stored values in source order",
        );
    }

    pub(super) fn assert_verb_onset_transport_and_mutation_rejection() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let scan_verb = |text, lexeme, member| {
            scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::Verb(lexeme, FeatureConstraint::Exact(Agreement::Bare)),
                    owner: LexicalOwnerTemplate::Lexeme {
                        declaration: "VerbLexeme",
                        member,
                    },
                    right_boundary: LexicalBoundary::Separated,
                },
            )
        };

        for (text, lexeme, member, onset, head_rule, article_rule, rendered) in [
            (
                "Act",
                VerbLexeme::Act,
                "Act",
                macro_ron::v2::Onset::Vowel,
                RuleId::VerbHeadActOnset,
                RuleId::VerbArticleIndefiniteVerbAn,
                "An act.",
            ),
            (
                "Same",
                VerbLexeme::Other,
                "Other",
                macro_ron::v2::Onset::Consonant,
                RuleId::VerbHeadOtherOnset,
                RuleId::VerbArticleIndefiniteVerbA,
                "A same.",
            ),
        ] {
            let scanned = scan_verb(text, lexeme, member);
            assert!(matches!(
                scanned.as_slice(),
                [LexicalMatch {
                    value: Leaf::Verb {
                        lexeme: actual_lexeme,
                        agreement: Agreement::Bare,
                        onset: actual_onset,
                    },
                    ..
                }] if *actual_lexeme == lexeme && *actual_onset == onset
            ));
            let head = build(
                head_rule,
                &[BuildValue::Leaf(scanned[0].value.clone())],
                &context,
            )
            .expect("a normalized verb row builds its onset-carrying head");
            assert!(matches!(head, BuildValue::VerbHead(_, actual) if actual == onset));
            let literal = if onset == macro_ron::v2::Onset::Vowel { "an" } else { "a" };
            let article = build(
                article_rule,
                &[BuildValue::Leaf(Leaf::Literal(literal)), head],
                &context,
            )
            .expect("the frozen verb onset selects the exact guarded article form");
            let BuildValue::VerbArticle(article, actual) = article else {
                panic!("the wrapper carries the forwarded verb onset")
            };
            assert_eq!(actual, onset);
            assert_eq!(Render::render(&article, &context), rendered);
        }

        assert!(
            build(
                RuleId::VerbHeadActOnset,
                &[BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::Bare,
                    onset: macro_ron::v2::Onset::Consonant,
                })],
                &context,
            )
            .is_none(),
            "a corrupted frozen onset is rejected at the build boundary"
        );
    }

    pub(super) fn assert_bound_atoms_preserve_adjacent_disjoint_claims() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let cases = [
            (
                BoundRoot::NonPlain(NonPlain {
                    value: BoundWord::Black,
                }),
                "Target nonblack.",
                [(0, 6), (6, 10), (10, 15)],
            ),
            (
                BoundRoot::NonHyphen(NonHyphen {
                    value: BoundWord::Elf,
                }),
                "Target non-Elf.",
                [(0, 6), (6, 11), (11, 14)],
            ),
            (
                BoundRoot::SingularPossessive(SingularPossessive {
                    owner: BoundWord::Tarmogoyf,
                }),
                "Target Tarmogoyf's.",
                [(0, 6), (6, 16), (16, 18)],
            ),
            (
                BoundRoot::PluralPossessive(PluralPossessive {
                    owner: BoundWord::Players,
                }),
                "Target players'.",
                [(0, 6), (6, 14), (14, 15)],
            ),
        ];

        for (value, expected, expected_spans) in cases {
            let (rendered, rendered_claims) = render_bound_root_with_claims(&value, &context);
            assert_eq!(rendered, expected);
            let surface = expected
                .strip_suffix('.')
                .expect("bound root fixture has root punctuation");
            let rendered_claims = rendered_claims
                .into_iter()
                .filter(|claim| claim.end <= surface.len())
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id().to_owned()))
                .collect::<Vec<_>>();
            assert_eq!(
                rendered_claims
                    .iter()
                    .map(|(start, end, _)| (*start, *end))
                    .collect::<Vec<_>>(),
                expected_spans,
            );
            assert_exact_partition(surface, &rendered_claims);

            let forest = parse_structural(Category::BoundRoot, surface, &context);
            let roots = forest.accepted_root_ids().collect::<Vec<_>>();
            assert_eq!(roots.len(), 1, "{surface:?} has one accepted root");
            let mut parsed_claims = Vec::new();
            collect_first_family_claims(&forest, roots[0], &mut parsed_claims);
            assert_eq!(parsed_claims, rendered_claims);
            assert_exact_partition(surface, &parsed_claims);
        }
        for malformed in [
            "Targetnonblack",
            "Target  nonblack",
            "Target non black",
            "Target Tarmogoyf 's",
        ] {
            assert!(
                try_parse_structural(Category::BoundRoot, malformed, &context).is_err(),
                "ordinary boundaries keep exactly one separating space and bound boundaries keep none: {malformed:?}",
            );
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one authentic consumer keeps every circumfix boundary assertion together"
    )]
    pub(super) fn assert_circumfix_atoms_delegate_payloads_and_own_only_outer_boundaries() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let value = |value| CircumfixValue::CircumfixWordValue(CircumfixValueNode { value });
        let singular = CircumfixSingularRoot::BracketedValue(BracketedValue {
            value: CircumfixValue::PlusTwo(PlusTwoValue),
        });
        let sequence = CircumfixSequenceRoot::BracedValues(
            BracedValues::new(vec![
                CircumfixValue::CircumfixTwo(CircumfixTwoValue),
                value(CircumfixWord::WhiteBlue),
                value(CircumfixWord::Tap),
            ])
            .expect("three values satisfy the declared nonempty sequence"),
        );

        let cases = [
            (
                Category::CircumfixSingularRoot,
                "[+2]",
                render_circumfix_singular_root_with_claims(&singular, &context),
                vec![
                    (
                        0,
                        1,
                        "form:bracketed_value/bracketed_value/0/prefix".to_owned(),
                    ),
                    (1, 3, "form:plus_two/plus_two/0".to_owned()),
                    (
                        3,
                        4,
                        "form:bracketed_value/bracketed_value/0/suffix".to_owned(),
                    ),
                ],
            ),
            (
                Category::CircumfixSequenceRoot,
                "{2}{W/U}{T}",
                render_circumfix_sequence_root_with_claims(&sequence, &context),
                vec![
                    (0, 1, "form:braced_values/braced_values/0/prefix".to_owned()),
                    (1, 2, "form:circumfix_two/circumfix_two/0".to_owned()),
                    (
                        2,
                        4,
                        "structural:BracedValues/values/separator/uniform/0".to_owned(),
                    ),
                    (4, 7, "vocab:CircumfixWord/WhiteBlue".to_owned()),
                    (
                        7,
                        9,
                        "structural:BracedValues/values/separator/uniform/0".to_owned(),
                    ),
                    (9, 10, "vocab:CircumfixWord/Tap".to_owned()),
                    (
                        10,
                        11,
                        "form:braced_values/braced_values/0/suffix".to_owned(),
                    ),
                ],
            ),
        ];
        for (category, surface, (rendered, rendered_claims), expected_claims) in cases {
            assert_eq!(rendered, format!("{surface}."));
            let rendered_claims = rendered_claims
                .into_iter()
                .filter(|claim| claim.end <= surface.len())
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id().to_owned()))
                .collect::<Vec<_>>();
            assert_eq!(rendered_claims, expected_claims);
            assert_exact_partition(surface, &rendered_claims);

            let forest = parse_structural(category, surface, &context);
            let root = forest
                .accepted_root_ids()
                .next()
                .expect("one accepted circumfix root");
            let mut parsed_claims = Vec::new();
            collect_first_family_claims(&forest, root, &mut parsed_claims);
            assert_eq!(parsed_claims, rendered_claims);
            assert_exact_partition(surface, &parsed_claims);
        }

        let singular_value = BuildValue::CircumfixValue(CircumfixValue::PlusTwo(PlusTwoValue));
        let built = build(
            RuleId::CircumfixSingularRootBracketedValue,
            &[
                BuildValue::Leaf(Leaf::Literal("[")),
                singular_value.clone(),
                BuildValue::Leaf(Leaf::Literal("]")),
            ],
            &context,
        )
        .expect("the singular public rule builds through both fixed boundaries");
        let BuildValue::CircumfixSingularRoot(built_singular) = built else {
            panic!("the singular public build returns its root category")
        };
        assert_eq!(
            render_circumfix_singular_root_with_claims(&built_singular, &context).0,
            "[+2].",
            "the singular public build preserves its exact delegated payload",
        );
        for children in [
            vec![singular_value.clone(), BuildValue::Leaf(Leaf::Literal("]"))],
            vec![
                BuildValue::Leaf(Leaf::Literal("[[")),
                singular_value.clone(),
                BuildValue::Leaf(Leaf::Literal("]")),
            ],
            vec![BuildValue::Leaf(Leaf::Literal("[")), singular_value.clone()],
            vec![
                BuildValue::Leaf(Leaf::Literal("[")),
                singular_value.clone(),
                BuildValue::Leaf(Leaf::Literal("]]")),
            ],
        ] {
            assert!(
                build(
                    RuleId::CircumfixSingularRootBracketedValue,
                    &children,
                    &context
                )
                .is_none(),
                "removing or duplicating either outer singular boundary is rejected",
            );
        }

        let sequence_tail = build(
            RuleId::BracedValuesValuesSequenceSingleton,
            &[BuildValue::CircumfixValue(value(CircumfixWord::Tap))],
            &context,
        )
        .expect("the delegated sequence singleton builds its stored member");
        let sequence_middle = build(
            RuleId::BracedValuesValuesSequenceRecursive,
            &[
                BuildValue::CircumfixValue(value(CircumfixWord::WhiteBlue)),
                BuildValue::Leaf(Leaf::Literal("}{")),
                sequence_tail.clone(),
            ],
            &context,
        )
        .expect("the delegated sequence helper builds through the second interior edge");
        let sequence_carrier = build(
            RuleId::BracedValuesValuesSequenceRecursive,
            &[
                BuildValue::CircumfixValue(CircumfixValue::CircumfixTwo(CircumfixTwoValue)),
                BuildValue::Leaf(Leaf::Literal("}{")),
                sequence_middle.clone(),
            ],
            &context,
        )
        .expect("the delegated sequence helper builds through the first interior edge");
        for (member, tail) in [
            (
                BuildValue::CircumfixValue(value(CircumfixWord::WhiteBlue)),
                sequence_tail,
            ),
            (
                BuildValue::CircumfixValue(CircumfixValue::CircumfixTwo(CircumfixTwoValue)),
                sequence_middle,
            ),
        ] {
            for children in [
                vec![member.clone(), tail.clone()],
                vec![
                    member.clone(),
                    BuildValue::Leaf(Leaf::Literal("}}{{")),
                    tail.clone(),
                ],
            ] {
                assert!(
                    build(
                        RuleId::BracedValuesValuesSequenceRecursive,
                        &children,
                        &context,
                    )
                    .is_none(),
                    "removing or duplicating either delegated sequence edge is rejected",
                );
            }
        }
        let built = build(
            RuleId::CircumfixSequenceRootBracedValues,
            &[
                BuildValue::Leaf(Leaf::Literal("{")),
                sequence_carrier.clone(),
                BuildValue::Leaf(Leaf::Literal("}")),
            ],
            &context,
        )
        .expect("the sequence public rule builds through both fixed outer boundaries");
        let BuildValue::CircumfixSequenceRoot(built_sequence) = built else {
            panic!("the sequence public build returns its root category")
        };
        assert_eq!(
            render_circumfix_sequence_root_with_claims(&built_sequence, &context).0,
            "{2}{W/U}{T}.",
            "the helper and public builds preserve every delegated payload in source order",
        );
        for children in [
            vec![
                sequence_carrier.clone(),
                BuildValue::Leaf(Leaf::Literal("}")),
            ],
            vec![
                BuildValue::Leaf(Leaf::Literal("{{")),
                sequence_carrier.clone(),
                BuildValue::Leaf(Leaf::Literal("}")),
            ],
            vec![
                BuildValue::Leaf(Leaf::Literal("{")),
                sequence_carrier.clone(),
            ],
            vec![
                BuildValue::Leaf(Leaf::Literal("{")),
                sequence_carrier.clone(),
                BuildValue::Leaf(Leaf::Literal("}}")),
            ],
        ] {
            assert!(
                build(
                    RuleId::CircumfixSequenceRootBracedValues,
                    &children,
                    &context
                )
                .is_none(),
                "removing or duplicating either outer sequence boundary is rejected",
            );
        }

        let mutate = |surface: &str, index: usize, duplicate: bool| {
            let mut mutated = surface.to_owned();
            let byte = char::from(surface.as_bytes()[index]);
            if duplicate {
                mutated.insert(index, byte);
            } else {
                mutated.remove(index);
            }
            mutated
        };
        for (category, surface, boundary_bytes) in [
            (Category::CircumfixSingularRoot, "[+2]", vec![0, 3]),
            (
                Category::CircumfixSequenceRoot,
                "{2}{W/U}{T}",
                vec![0, 2, 3, 7, 8, 10],
            ),
        ] {
            for index in boundary_bytes {
                for duplicate in [false, true] {
                    let malformed = mutate(surface, index, duplicate);
                    assert!(
                        try_parse_structural(category, &malformed, &context).is_err(),
                        "each removed or duplicated outer/sequence boundary byte is rejected: {malformed:?}",
                    );
                }
            }
        }
        for malformed in ["[ +2]", "[+2 ]", "{2} {W/U}{T}", "{2}{w/u}{T}"] {
            let category = if malformed.starts_with('[') {
                Category::CircumfixSingularRoot
            } else {
                Category::CircumfixSequenceRoot
            };
            assert!(
                try_parse_structural(category, malformed, &context).is_err(),
                "circumfix adjacency and case transitions stay exact: {malformed:?}",
            );
        }

        let mut visitor = RecordingVisitor::default();
        walk_circumfix_singular_root(&mut visitor, &singular);
        assert_eq!(
            visitor.0,
            [VisitEvent::PlusTwo],
            "the singular circumfix walker delegates to its observable payload",
        );
        walk_circumfix_sequence_root(&mut visitor, &sequence);
        assert_eq!(
            visitor.0,
            [
                VisitEvent::PlusTwo,
                VisitEvent::CircumfixWord(CircumfixWord::WhiteBlue),
                VisitEvent::CircumfixWord(CircumfixWord::Tap),
            ],
            "visiting delegates only the stored singular and sequence payloads",
        );
    }

    pub(super) fn assert_same_origin_dual_boundary_predictions() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };

        for surface in ["Target black tail", "Target black's"] {
            let forest = parse_structural(Category::DualBoundaryRoot, surface, &context);
            assert_eq!(
                forest.accepted_root_ids().count(),
                1,
                "{surface:?} retains its exact boundary-specific root",
            );
        }
        for malformed in ["Target blacktail", "Target black 's"] {
            assert!(
                try_parse_structural(Category::DualBoundaryRoot, malformed, &context).is_err(),
                "{malformed:?} must not cross the ordinary/adjacent boundary contract",
            );
        }
    }

    pub(super) fn assert_bound_prefix_onset_is_form_local() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let head = build(
            RuleId::PrefixHeadPrefixedOnsetLexical,
            &[
                BuildValue::Leaf(Leaf::Mode(Mode::One)),
                BuildValue::Leaf(Leaf::Literal("non")),
                BuildValue::Leaf(Leaf::BoundWord(BoundWord::Artifact)),
            ],
            &context,
        )
        .expect("the fixed prefix overrides the vowel-initial value onset");
        assert!(matches!(
            head,
            BuildValue::PrefixHead(_, macro_ron::v2::Onset::Consonant)
        ));
        assert!(
            build(
                RuleId::PrefixArticlePrefixedArticleAn,
                &[BuildValue::Leaf(Leaf::Literal("an")), head.clone()],
                &context,
            )
            .is_none(),
            "the vowel article cannot accept the value's pre-prefix onset",
        );
        let article = build(
            RuleId::PrefixArticlePrefixedArticleA,
            &[BuildValue::Leaf(Leaf::Literal("a")), head],
            &context,
        )
        .expect("the consonant article accepts the realized prefix onset");
        let BuildValue::PrefixArticle(article, actual_onset) = article else {
            panic!("the prefixed article builds its declared category")
        };
        assert_eq!(actual_onset, macro_ron::v2::Onset::Consonant);
        assert_eq!(Render::render(&article, &context), "A one nonartifact.");

        let forest = parse_structural(Category::PrefixArticle, "A one nonartifact", &context);
        assert_eq!(forest.accepted_root_ids().count(), 1);

        for (value, article_rule, article_literal, expected_onset, expected_surface) in [
            (
                BoundWord::Elf,
                RuleId::PrefixArticlePrefixedArticleAn,
                "an",
                macro_ron::v2::Onset::Vowel,
                "An many 2/Elf.",
            ),
            (
                BoundWord::Black,
                RuleId::PrefixArticlePrefixedArticleA,
                "a",
                macro_ron::v2::Onset::Consonant,
                "A many 2/black.",
            ),
        ] {
            let head = build(
                RuleId::PrefixHeadPrefixedOnsetPunctuation,
                &[
                    BuildValue::Leaf(Leaf::Mode(Mode::Many)),
                    BuildValue::Leaf(Leaf::Literal("2/")),
                    BuildValue::Leaf(Leaf::BoundWord(value)),
                ],
                &context,
            )
            .expect("the punctuation form forwards its own payload onset");
            assert!(matches!(
                head,
                BuildValue::PrefixHead(_, actual) if actual == expected_onset
            ));
            let article = build(
                article_rule,
                &[BuildValue::Leaf(Leaf::Literal(article_literal)), head],
                &context,
            )
            .expect("the article guard consumes the form-local realized onset");
            let BuildValue::PrefixArticle(article, actual_onset) = article else {
                panic!("the punctuation-prefixed article builds its category")
            };
            assert_eq!(actual_onset, expected_onset);
            assert_eq!(Render::render(&article, &context), expected_surface);

            let punctuation_rule = RULES
                .iter()
                .find(|rule| rule.id == RuleId::PrefixHeadPrefixedOnsetPunctuation)
                .expect("the punctuation prefix rule is generated");
            let [L(_kind), L(prefix), L(payload)] = punctuation_rule.rhs else {
                panic!("the punctuation prefix rule has its selector, prefix, and payload")
            };
            let start = expected_surface
                .find(" 2/")
                .expect("prefix starts after article")
                + 1;
            let prefix_position = ScanPosition {
                byte_offset: start,
                case: CasePosition::Continuation,
                prefix: PrefixPosition::None,
            };
            let prefix_matches = scan_lexical(
                &ScanInput {
                    text: expected_surface,
                    position: prefix_position,
                    context: &context,
                },
                *prefix,
            );
            assert_eq!(prefix_matches.len(), 1, "the prefix scans exactly once");
            assert_eq!(prefix_matches[0].end, start + 2);
            assert_eq!(prefix_matches[0].value, Leaf::Literal("2/"));
            let payload_position = prefix.position_after(prefix_position, start + 2);
            assert_eq!(payload_position.prefix, PrefixPosition::SurfaceOwned);
            let payload_matches = scan_lexical(
                &ScanInput {
                    text: expected_surface,
                    position: payload_position,
                    context: &context,
                },
                *payload,
            );
            assert_eq!(payload_matches.len(), 1, "the adjacent payload scans once");
            assert_eq!(payload_matches[0].end, expected_surface.len() - 1);
            assert_eq!(payload_matches[0].value, Leaf::BoundWord(value));

            let forest = parse_structural(
                Category::PrefixArticle,
                expected_surface
                    .strip_suffix('.')
                    .expect("the standalone render owns the final punctuation"),
                &context,
            );
            assert_eq!(forest.accepted_root_ids().count(), 1, "{expected_surface}");
        }
    }

    pub(super) fn assert_possessive_ending_selects_guarded_suffix_forms() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        let cases = [
            (
                BoundWord::Daxos,
                Number::Singular,
                PossessiveEnding::EndsInS,
                RuleId::DerivedPossessiveRootDerivedPossessiveSingular,
                "'s",
                "Daxos's.",
            ),
            (
                BoundWord::Players,
                Number::Plural,
                PossessiveEnding::EndsInS,
                RuleId::DerivedPossessiveRootDerivedPossessivePluralS,
                "'",
                "Players'.",
            ),
            (
                BoundWord::Merfolk,
                Number::Plural,
                PossessiveEnding::Other,
                RuleId::DerivedPossessiveRootDerivedPossessivePluralOther,
                "'s",
                "Merfolk's.",
            ),
            (
                BoundWord::Equipment,
                Number::Plural,
                PossessiveEnding::Other,
                RuleId::DerivedPossessiveRootDerivedPossessivePluralOther,
                "'s",
                "Equipment's.",
            ),
        ];

        for (word, number, ending, possessive_rule, suffix, expected) in cases {
            let owner = build(
                RuleId::PossessiveOwnerBoundOwner,
                &[BuildValue::Leaf(Leaf::BoundWord(word))],
                &context,
            )
            .expect("the realized owner surface derives its sealed features");
            assert!(matches!(
                owner,
                BuildValue::PossessiveOwner(_, actual_number, actual_ending)
                    if actual_number == number && actual_ending == ending
            ));
            let possessive = build(
                possessive_rule,
                &[owner.clone(), BuildValue::Leaf(Leaf::Literal(suffix))],
                &context,
            )
            .expect("the exact Number/Ending partition selects one suffix form");
            let BuildValue::DerivedPossessiveRoot(possessive) = possessive else {
                panic!("the guarded suffix rule builds its declared category")
            };
            assert_eq!(Render::render(&possessive, &context), expected);

            let accepted = [
                (RuleId::DerivedPossessiveRootDerivedPossessiveSingular, "'s"),
                (RuleId::DerivedPossessiveRootDerivedPossessivePluralS, "'"),
                (
                    RuleId::DerivedPossessiveRootDerivedPossessivePluralOther,
                    "'s",
                ),
            ]
            .into_iter()
            .filter(|(rule, suffix)| {
                build(
                    *rule,
                    &[owner.clone(), BuildValue::Leaf(Leaf::Literal(suffix))],
                    &context,
                )
                .is_some()
            })
            .count();
            assert_eq!(accepted, 1, "{expected:?} has one build-valid form");
        }

        let daxos = BuildValue::PossessiveOwner(
            PossessiveOwner::BoundOwner(BoundOwner {
                value: BoundWord::Daxos,
            }),
            Number::Plural,
            PossessiveEnding::EndsInS,
        );
        assert!(
            build(
                RuleId::DerivedPossessiveRootDerivedPossessiveSingular,
                &[daxos, BuildValue::Leaf(Leaf::Literal("'s"))],
                &context,
            )
            .is_none(),
            "a Number mutation cannot cross the singular guarded build boundary",
        );

        let players = BuildValue::PossessiveOwner(
            PossessiveOwner::BoundOwner(BoundOwner {
                value: BoundWord::Players,
            }),
            Number::Plural,
            PossessiveEnding::Other,
        );
        assert!(
            build(
                RuleId::DerivedPossessiveRootDerivedPossessivePluralS,
                &[players, BuildValue::Leaf(Leaf::Literal("'"))],
                &context,
            )
            .is_none(),
            "a PossessiveEnding mutation cannot cross the EndsInS guarded build boundary",
        );
    }

    pub(super) fn assert_exact_name_render_uses_frozen_onset() {
        for (surface, frozen_onset, expected) in [
            ("artifact", macro_ron::v2::Onset::Consonant, "A artifact."),
            ("card", macro_ron::v2::Onset::Vowel, "An card."),
        ] {
            let context = ParseContext {
                sentinel: 99,
                card_name: surface,
                abbreviated_card_name: surface,
                card_name_onset: frozen_onset,
                abbreviated_card_name_onset: frozen_onset,
            };
            let named = NamedHead::NamedHead(NamedHeadValue {
                spelling: SelfRef::Full,
            });
            let article = NamedArticle::NamedArticle(NamedArticleValue { head: named });
            assert_eq!(Render::render(&article, &context), expected);
        }
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
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        assert_generated_runtime_abi(&context);
        assert!(SelfRef::Full.valid_in(&context));
        assert!(!SelfRef::Abbreviated.valid_in(&context));
        assert_eq!(SelfRef::Full.surface(&context), "card");
        let abbreviated_context = ParseContext {
            sentinel: 99,
            card_name: "full card",
            abbreviated_card_name: "short",
            card_name_onset: macro_ron::v2::Onset::Consonant,
            abbreviated_card_name_onset: macro_ron::v2::Onset::Consonant,
        };
        assert!(SelfRef::Abbreviated.valid_in(&abbreviated_context));
        assert_eq!(SelfRef::Abbreviated.surface(&abbreviated_context), "short");
        let identity_matches = scan_lexical(
            &ScanInput {
                text: "short",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                context: &abbreviated_context,
            },
            LexicalTerminal {
                matcher: Lexical::SelfRef,
                owner: LexicalOwnerTemplate::Identity {
                    declaration: "SelfRef",
                },
                right_boundary: LexicalBoundary::Separated,
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
                    onset: macro_ron::v2::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
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
                    onset: macro_ron::v2::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(3),
                    number: right,
                    onset: macro_ron::v2::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
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
                lexeme: VerbLexeme::Act,
                agreement: Agreement::Bare,
                onset: macro_ron::v2::Onset::Vowel,
            })],
            &context,
        )
        .expect("an implicit-verb constant flows into construction output");
        assert!(matches!(action, BuildValue::Action(_, Agreement::Bare)));
        assert!(
            build(
                RuleId::ActionAction,
                &[BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::ThirdPersonSingular,
                    onset: macro_ron::v2::Onset::Vowel,
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

        let context_bound = build(
            RuleId::ContextBoundContextBound,
            &[BuildValue::Leaf(Leaf::Pair(BoundLeaf::Pair(1, 2, 3)))],
            &context,
        )
        .expect("terminal slots cannot shadow the parser context ABI local");
        assert!(matches!(
            context_bound,
            BuildValue::ContextBound(ContextBound::ContextBound(ContextBoundNode {
                pair: Pair {
                    context_value: 1,
                    child_slot: 2,
                    rule_code: 3,
                },
            }))
        ));

        let feature_bound = build(
            RuleId::FeatureBoundAgreement,
            &[
                BuildValue::Leaf(Leaf::Marker(Marker::One)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::ThirdPersonSingular,
                    onset: macro_ron::v2::Onset::Vowel,
                }),
            ],
            &context,
        )
        .expect("a map local cannot shadow its carried agreement");
        assert!(matches!(
            feature_bound,
            BuildValue::FeatureBound(_, Agreement::ThirdPersonSingular)
        ));

        let collision = build(
            RuleId::CollisionCollision,
            &[
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(4),
                    number: Number::Singular,
                    onset: macro_ron::v2::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(5),
                    number: Number::Singular,
                    onset: macro_ron::v2::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
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
        .expect("a keyword-named construction builds with its carried feature");
        let BuildValue::Keyword(keyword, Agreement::Bare) = keyword else {
            panic!("`where` preserves its category value and known feature")
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
            rendered_hygiene_root, "Marker card marker act bare writer marker marker!",
            "allocated render locals preserve ABI values and generated helper calls",
        );
        assert_generated_punctuation_scan(&rendered_hygiene_root, Some("marker"), "!", &context);
        let context_free_nested_root = RenderChild::Wrapper(RenderChildNode {
            child: Child::Bare(BareChild),
        });
        assert_eq!(
            Render::render(&context_free_nested_root, &context),
            "Bare.",
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
        let walk_marker = WalkMarker {
            marker: Marker::One,
        };
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
                    prefix: PrefixPosition::None,
                },
                context: &context,
            },
            LexicalTerminal {
                matcher: Lexical::SignedNumber,
                owner: LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: "codec:SignedNumber",
                },
                right_boundary: LexicalBoundary::Separated,
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
            &[BuildValue::Leaf(Leaf::Literal("raw"))],
            &context,
        )
        .expect("raw nonkeyword declaration spellings build through canonical generated names");
        let BuildValue::RawCategory(raw_category) = raw_category else {
            panic!("raw nonkeyword root preserves its generated category value")
        };
        let rendered_raw_category = Render::render(&raw_category, &context);
        assert_eq!(rendered_raw_category, "Raw?");
        assert_generated_punctuation_scan(&rendered_raw_category, None, "?", &context);
        walk_raw_category(&mut recording, &raw_category);
    }

    fn assert_cardinal_number_literal_goldens() {
        for (value, cardinal) in [
            (0, "zero"),
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
            (10, "ten"),
            (11, "eleven"),
            (12, "twelve"),
            (13, "thirteen"),
            (14, "fourteen"),
            (15, "fifteen"),
            (16, "sixteen"),
            (17, "seventeen"),
            (18, "eighteen"),
            (19, "nineteen"),
            (20, "twenty"),
            (21, "twenty-one"),
            (30, "thirty"),
            (40, "forty"),
            (50, "fifty"),
            (60, "sixty"),
            (70, "seventy"),
            (80, "eighty"),
            (90, "ninety"),
            (99, "ninety-nine"),
            (100, "one hundred"),
            (101, "one hundred one"),
            (105, "one hundred five"),
            (999, "nine hundred ninety-nine"),
            (1_000, "one thousand"),
            (1_001, "one thousand, one"),
            (
                999_999,
                "nine hundred ninety-nine thousand, nine hundred ninety-nine",
            ),
            (1_000_000, "one million"),
            (1_000_001, "one million, one"),
            (
                999_999_999,
                "nine hundred ninety-nine million, nine hundred ninety-nine thousand, nine hundred ninety-nine",
            ),
            (1_000_000_000, "one billion"),
            (1_000_000_001, "one billion, one"),
            (
                u32::MAX - 1,
                "four billion, two hundred ninety-four million, nine hundred sixty-seven thousand, two hundred ninety-four",
            ),
            (
                u32::MAX,
                "four billion, two hundred ninety-four million, nine hundred sixty-seven thousand, two hundred ninety-five",
            ),
        ] {
            assert_eq!(format_cardinal_number(value), cardinal);
            assert_eq!(parse_cardinal_number(cardinal), Some(value));
        }
    }

    fn assert_scalar_number_literal_goldens() {
        for (value, decimal) in [
            (0, "0"),
            (9, "9"),
            (10, "10"),
            (11, "11"),
            (99, "99"),
            (100, "100"),
            (101, "101"),
            (999, "999"),
            (1_000, "1,000"),
            (1_001, "1,001"),
            (9_999, "9,999"),
            (10_000, "10,000"),
            (10_001, "10,001"),
            (99_999, "99,999"),
            (100_000, "100,000"),
            (100_001, "100,001"),
            (999_999, "999,999"),
            (1_000_000, "1,000,000"),
            (1_000_001, "1,000,001"),
            (9_999_999, "9,999,999"),
            (10_000_000, "10,000,000"),
            (10_000_001, "10,000,001"),
            (99_999_999, "99,999,999"),
            (100_000_000, "100,000,000"),
            (100_000_001, "100,000,001"),
            (999_999_999, "999,999,999"),
            (1_000_000_000, "1,000,000,000"),
            (1_000_000_001, "1,000,000,001"),
            (u32::MAX - 1, "4,294,967,294"),
            (u32::MAX, "4,294,967,295"),
        ] {
            assert_eq!(format_scalar_number(value), decimal);
            assert_eq!(parse_scalar_number(decimal), Some(value));
        }
    }

    pub(super) fn assert_unsigned_numeral_codecs_are_canonical_and_total() {
        assert_cardinal_number_literal_goldens();
        assert_scalar_number_literal_goldens();

        for rejected in [
            "",
            "One",
            " one",
            "negative one",
            "eleven hundred",
            "forty and five",
            "forty five",
            "one hundred and five",
            "one thousand one",
            "one thousand, zero",
            "one million, one million",
            "four billion, two hundred ninety-four million, nine hundred sixty-seven thousand, two hundred ninety-six",
        ] {
            assert_eq!(parse_cardinal_number(rejected), None, "{rejected:?}");
        }
        for rejected in [
            "",
            "00",
            "01",
            "+1",
            "-1",
            "1000",
            "1,00",
            "1,000,",
            "4,294,967,296",
        ] {
            assert_eq!(parse_scalar_number(rejected), None, "{rejected:?}");
        }

        for value in 0..=9_999 {
            let cardinal = format_cardinal_number(value);
            assert_eq!(parse_cardinal_number(&cardinal), Some(value));
            let decimal = format_scalar_number(value);
            assert_eq!(parse_scalar_number(&decimal), Some(value));
        }
        let mut value = 0x9e37_79b9_u32;
        for _ in 0..10_000 {
            value = value.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let cardinal = format_cardinal_number(value);
            assert_eq!(parse_cardinal_number(&cardinal), Some(value));
            let decimal = format_scalar_number(value);
            assert_eq!(parse_scalar_number(&decimal), Some(value));
        }

        let mut recording = RecordingVisitor::default();
        walk_cardinal_number(&mut recording, &CardinalNumber { magnitude: 21 });
        walk_scalar_number(&mut recording, &ScalarNumber { magnitude: 1_000 });
        assert_eq!(
            &recording.0[recording.0.len() - 2..],
            [
                VisitEvent::CardinalNumber(21),
                VisitEvent::ScalarNumber(1_000),
            ],
        );

        let context = ParseContext::default();
        for (magnitude, expected_cardinality, expected_number) in [
            (0, Cardinality::Zero, Number::Plural),
            (1, Cardinality::One, Number::Singular),
            (2, Cardinality::TwoPlus, Number::Plural),
            (u32::MAX, Cardinality::TwoPlus, Number::Plural),
        ] {
            let number = CardinalNumber { magnitude };
            let built = build(
                RuleId::CardinalQuantityCardinal,
                &[BuildValue::Leaf(Leaf::CardinalNumber(number.clone()))],
                &context,
            )
            .expect("cardinal magnitude supplies one-versus-other Number");
            assert_eq!(
                built,
                BuildValue::CardinalQuantity(
                    CardinalQuantity::Cardinal(CardinalQuantityValue { number }),
                    expected_cardinality,
                    expected_number,
                ),
            );
        }
    }

    pub(super) fn assert_sequence_agreement_is_uniform_across_every_member() {
        let bare = || Child::Bare(BareChild);
        let third = || Child::Third(ThirdChild);

        for length in [2, 3, 4] {
            let members = (0..length).map(|_| bare()).collect::<Vec<_>>();
            let inbound = UniformChildren::new(members.clone())
                .expect("two through four uniformly bare members satisfy the inbound writer");
            assert_eq!(inbound.members(), members);
            let outward = RelayedChildren::new(members)
                .expect("two through four homogeneous members satisfy the outward relay");
            assert_eq!(outward.members().len(), length);
        }

        for members in [
            vec![bare(), third(), bare()],
            vec![bare(), bare(), third()],
            vec![bare(), third(), bare(), bare()],
            vec![bare(), bare(), bare(), third()],
        ] {
            let inbound = UniformChildren::try_new(members.clone())
                .expect_err("a mismatched middle or final member rejects the inbound writer");
            assert_eq!(inbound.owner(), "UniformChildren");
            assert_eq!(inbound.role(), "members");
            assert_eq!(
                inbound.violation(),
                &BuildViolation::Invariant {
                    identity: "all members match derived agreement",
                },
            );
            let outward = RelayedChildren::try_new(members)
                .expect_err("a mismatched middle or final member rejects the outward relay");
            assert_eq!(outward.owner(), "RelayedChildren");
            assert_eq!(outward.role(), "members");
            assert_eq!(
                outward.violation(),
                &BuildViolation::Invariant {
                    identity: "all members share agreement",
                },
            );
        }

        let context = ParseContext::default();
        for surface in ["Bare bare", "Bare bare bare", "Bare bare bare bare"] {
            let forest = parse_structural(Category::HomogeneousSequence, surface, &context);
            assert_eq!(forest.accepted_root_ids().count(), 1, "{surface}");
        }

        let child = |agreement| match agreement {
            Agreement::Bare => BuildValue::Child(bare(), Agreement::Bare),
            Agreement::ThirdPersonSingular => {
                BuildValue::Child(third(), Agreement::ThirdPersonSingular)
            }
        };
        let pair = build(
            RuleId::UniformChildrenMembersSequenceLength2,
            &[
                child(Agreement::Bare),
                BuildValue::Leaf(Leaf::Literal(" ")),
                child(Agreement::Bare),
            ],
            &context,
        )
        .expect("the exact pair helper materializes its carried agreement");
        assert!(matches!(
            pair,
            BuildValue::UniformChildrenMembersSequence(_, Agreement::Bare)
        ));
        assert!(
            build(
                RuleId::UniformChildrenMembersSequenceLength2,
                &[
                    child(Agreement::Bare),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    child(Agreement::ThirdPersonSingular),
                ],
                &context,
            )
            .is_none(),
            "the exact pair helper rejects a mismatched final member",
        );
        assert!(
            build(
                RuleId::UniformChildrenMembersSequenceRecursive,
                &[
                    child(Agreement::ThirdPersonSingular),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    pair.clone(),
                ],
                &context,
            )
            .is_none(),
            "the recursive helper rejects a mismatched middle or final carrier",
        );
        let three = build(
            RuleId::UniformChildrenMembersSequenceRecursive,
            &[
                child(Agreement::Bare),
                BuildValue::Leaf(Leaf::Literal(" ")),
                pair,
            ],
            &context,
        )
        .expect("the recursive helper preserves homogeneous agreement");
        let built = build(
            RuleId::HomogeneousSequenceUniformChildren,
            std::slice::from_ref(&three),
            &context,
        )
        .expect("the owner materializes the checked public construction");
        let BuildValue::HomogeneousSequence(sequence) = built else {
            panic!("the homogeneous sequence builds its declared category")
        };
        assert_eq!(Render::render(&sequence, &context), "Bare bare bare.");
    }

    pub(super) fn assert_singleton_sequence_agreement_crosses_every_runtime_boundary() {
        let bare = || Child::Bare(BareChild);
        let third = || Child::Third(ThirdChild);

        let singleton = SingletonChildren::new(vec![bare()])
            .expect("the statically nonempty singleton satisfies its uniform agreement writer");
        assert_eq!(singleton.members(), &[bare()]);
        let rejection = SingletonChildren::try_new(vec![third()])
            .expect_err("a singleton with the wrong derived agreement rejects");
        assert_eq!(rejection.owner(), "SingletonChildren");
        assert_eq!(rejection.role(), "members");
        assert_eq!(
            rejection.violation(),
            &BuildViolation::Invariant {
                identity: "all members match derived agreement",
            },
        );

        let context = ParseContext::default();
        let child = |value, agreement| BuildValue::Child(value, agreement);
        let carrier = build(
            RuleId::SingletonChildrenMembersSequenceSingleton,
            &[child(bare(), Agreement::Bare)],
            &context,
        )
        .expect("the singleton helper materializes one agreement-bearing member");
        assert!(matches!(
            carrier,
            BuildValue::SingletonChildrenMembersSequence(_, Agreement::Bare)
        ));
        let built = build(
            RuleId::SingletonSequenceSingletonChildren,
            std::slice::from_ref(&carrier),
            &context,
        )
        .expect("the singleton owner materializes its checked public construction");
        let BuildValue::SingletonSequence(sequence) = built else {
            panic!("the singleton sequence builds its declared category")
        };
        assert_eq!(Render::render(&sequence, &context), "Bare.");

        let wrong_carrier = build(
            RuleId::SingletonChildrenMembersSequenceSingleton,
            &[child(third(), Agreement::ThirdPersonSingular)],
            &context,
        )
        .expect("one member is homogeneous with itself before the owner writer applies");
        let owner_rejection = build_checked(
            RuleId::SingletonSequenceSingletonChildren,
            &[wrong_carrier],
            &context,
        )
        .expect_err("materialization enforces the singleton owner's derived agreement");
        assert_eq!(owner_rejection.owner(), "SingletonChildren");
        assert_eq!(owner_rejection.role(), "members");

        let accepted = parse_structural(Category::SingletonSequence, "Bare", &context);
        assert_eq!(accepted.accepted_root_ids().count(), 1);
        let scanned_wrong_agreement =
            parse_structural(Category::SingletonSequence, "Third", &context);
        assert_eq!(
            scanned_wrong_agreement.accepted_root_ids().count(),
            1,
            "the scanner and chart preserve the lexical reading before checked materialization rejects it",
        );
    }

    pub(super) fn assert_sum_sequence_agreement_uses_the_explicit_sum_carrier() {
        let bare = || AgreementChild::Child(Child::Bare(BareChild));
        let third = || AgreementChild::Child(Child::Third(ThirdChild));

        for length in [2, 3, 4] {
            let members = (0..length).map(|_| bare()).collect::<Vec<_>>();
            let sequence = UniformChildChoices::new(members.clone())
                .expect("agreement-bearing sum members satisfy the uniform writer");
            assert_eq!(sequence.members(), members);
            let relayed = RelayedChildChoices::new(members)
                .expect("agreement-bearing sum members satisfy the homogeneous outward relay");
            assert_eq!(relayed.members().len(), length);
        }
        for members in [vec![bare(), third(), bare()], vec![bare(), bare(), third()]] {
            let rejection = UniformChildChoices::try_new(members)
                .expect_err("middle and final sum alternatives retain their carried agreement");
            assert_eq!(rejection.owner(), "UniformChildChoices");
            assert_eq!(rejection.role(), "members");
            assert_eq!(
                rejection.violation(),
                &BuildViolation::Invariant {
                    identity: "all members match derived agreement",
                },
            );
            let relay_rejection = RelayedChildChoices::try_new(vec![bare(), third(), bare()])
                .expect_err("the sum relay checks every member through its generated helper");
            assert_eq!(relay_rejection.owner(), "RelayedChildChoices");
            assert_eq!(relay_rejection.role(), "members");
        }

        let context = ParseContext::default();
        for surface in ["Bare bare", "Bare bare bare", "Bare bare bare bare"] {
            let forest = parse_structural(Category::HomogeneousChoiceSequence, surface, &context);
            assert_eq!(forest.accepted_root_ids().count(), 1, "{surface}");
        }
        for surface in ["Bare third bare", "Bare bare third"] {
            let forest = parse_structural(Category::HomogeneousChoiceSequence, surface, &context);
            assert_eq!(
                forest.accepted_root_ids().count(),
                1,
                "the scanner and chart preserve {surface:?} before materialization checks carried agreement",
            );
        }

        let rendered = HomogeneousChoiceSequence::UniformChildChoices(
            UniformChildChoices::new(vec![bare(), bare(), bare()])
                .expect("render fixture is uniformly bare"),
        );
        assert_eq!(Render::render(&rendered, &context), "Bare bare bare.");

        let choice =
            |value, agreement| BuildValue::AgreementChild(AgreementChild::Child(value), agreement);
        let pair = build(
            RuleId::UniformChildChoicesMembersSequenceLength2,
            &[
                choice(Child::Bare(BareChild), Agreement::Bare),
                BuildValue::Leaf(Leaf::Literal(" ")),
                choice(Child::Bare(BareChild), Agreement::Bare),
            ],
            &context,
        )
        .expect("the exact sum pair materializes its agreement carrier");
        assert!(matches!(
            pair,
            BuildValue::UniformChildChoicesMembersSequence(_, Agreement::Bare)
        ));
        assert!(
            build(
                RuleId::UniformChildChoicesMembersSequenceLength2,
                &[
                    choice(Child::Bare(BareChild), Agreement::Bare),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    choice(Child::Third(ThirdChild), Agreement::ThirdPersonSingular),
                ],
                &context,
            )
            .is_none(),
            "the exact sum pair rejects a mismatched final carrier",
        );
        assert!(
            build(
                RuleId::UniformChildChoicesMembersSequenceRecursive,
                &[
                    choice(Child::Third(ThirdChild), Agreement::ThirdPersonSingular),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    pair,
                ],
                &context,
            )
            .is_none(),
            "the recursive sum helper rejects a mismatched middle carrier",
        );
    }

    pub(super) fn assert_nonzero_unsigned_decimal_is_typed_canonical_and_exact() {
        let one = std::num::NonZeroU32::new(1).expect("one is nonzero");
        let maximum = std::num::NonZeroU32::new(u32::MAX).expect("u32::MAX is nonzero");
        assert_eq!(format_non_zero_scalar_number(one), "1");
        assert_eq!(parse_non_zero_scalar_number("1"), Some(one));
        assert_eq!(format_non_zero_scalar_number(maximum), "4,294,967,295");
        assert_eq!(parse_non_zero_scalar_number("4,294,967,295"), Some(maximum));

        for rejected in [
            "",
            "0",
            "00",
            "01",
            "+1",
            "-1",
            "1000",
            "1,00",
            "1,000,",
            "4,294,967,296",
            "1.",
        ] {
            assert_eq!(parse_non_zero_scalar_number(rejected), None, "{rejected:?}");
        }

        let context = ParseContext::default();
        for (surface, magnitude) in [("1", one), ("4,294,967,295", maximum)] {
            let input = format!("{surface}.");
            let matches = scan_lexical(
                &ScanInput {
                    text: &input,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::NonZeroScalarNumber,
                    owner: LexicalOwnerTemplate::Static {
                        kind: LexicalProvenanceKind::Codec,
                        stable_id: "codec:NonZeroScalarNumber",
                    },
                    right_boundary: LexicalBoundary::Separated,
                },
            );
            assert_eq!(matches.len(), 1, "{surface}");
            assert_eq!(matches[0].end, surface.len(), "{surface}");
            assert_eq!(
                matches[0].value,
                Leaf::NonZeroScalarNumber(NonZeroScalarNumber { magnitude })
            );
        }

        for rejected in ["0", "01", "+1", "-1", "4,294,967,296", "."] {
            let matches = scan_lexical(
                &ScanInput {
                    text: rejected,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::NonZeroScalarNumber,
                    owner: LexicalOwnerTemplate::Static {
                        kind: LexicalProvenanceKind::Codec,
                        stable_id: "codec:NonZeroScalarNumber",
                    },
                    right_boundary: LexicalBoundary::Separated,
                },
            );
            assert!(matches.is_empty(), "{rejected:?}: {matches:?}");
        }

        let number = NonZeroScalarNumber { magnitude: maximum };
        let mut writer = Writer::new();
        render_non_zero_scalar_number(&mut writer, &number);
        assert_eq!(writer.finish(), "4,294,967,295");

        let built = build(
            RuleId::NonZeroQuantityPositive,
            &[BuildValue::Leaf(Leaf::NonZeroScalarNumber(number.clone()))],
            &context,
        )
        .expect("the generated construction builds the typed nonzero leaf");
        assert_eq!(
            built,
            BuildValue::NonZeroQuantity(NonZeroQuantity::Positive(PositiveQuantityValue {
                number: number.clone(),
            }))
        );

        let mut recording = RecordingVisitor::default();
        walk_non_zero_scalar_number(&mut recording, &number);
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::NonZeroScalarNumber(maximum))
        );
    }
}

#[test]
fn generated_morphology_output_is_type_correct_and_executes_every_boundary_case() {
    declaration_noun_fixture::run();
    fixture::run();
}

#[test]
fn dynamic_declaration_noun_role_guards_are_independent() {
    declaration_noun_fixture::assert_dynamic_role_number_guards_are_independent();
}

#[test]
fn construction_number_does_not_overconstrain_declaration_noun_roles() {
    declaration_noun_fixture::assert_construction_number_does_not_overconstrain_noun_roles();
}

#[test]
fn irregular_noun_rows_keep_exact_onset_through_scan_build_guard_and_render() {
    declaration_noun_fixture::assert_irregular_noun_onset_rows_agree();
}

#[test]
fn invariant_constructors_enforce_the_compiled_public_boundary() {
    fixture::assert_invariant_public_boundary();
}

#[test]
fn guarded_forms_build_and_render_their_exact_finite_partitions() {
    fixture::assert_guarded_form_partition_boundaries();
}

#[test]
fn optional_vocab_guards_and_invariants_share_the_closed_runtime_domain() {
    fixture::assert_optional_vocab_guards_and_invariants();
}

#[test]
fn optional_vocab_visitor_guards_select_observably_distinct_traversals() {
    fixture::assert_optional_vocab_visitor_guard_selection();
}

#[test]
fn structural_constructors_enforce_the_compiled_public_boundary() {
    fixture::assert_structural_product_public_boundary();
}

#[test]
fn structural_helpers_parse_and_fold_through_ordinary_bnf() {
    fixture::assert_structural_bnf_boundaries();
}

#[test]
fn structural_surfaces_ownership_and_traversal_execute_generated_code() {
    fixture::assert_structural_render_scan_ownership_and_traversal();
}

#[test]
fn verb_onset_survives_scan_build_forwarding_guard_and_render() {
    fixture::assert_verb_onset_transport_and_mutation_rejection();
}

#[test]
fn bound_atoms_scan_and_render_adjacent_disjoint_claims() {
    fixture::assert_bound_atoms_preserve_adjacent_disjoint_claims();
}

#[test]
fn circumfix_atoms_delegate_payloads_and_own_only_fixed_outer_boundaries() {
    fixture::assert_circumfix_atoms_delegate_payloads_and_own_only_outer_boundaries();
}

#[test]
fn same_origin_nested_category_keeps_separated_and_adjacent_earley_identities_distinct() {
    fixture::assert_same_origin_dual_boundary_predictions();
}

#[test]
fn bound_prefix_realized_onset_is_selected_per_form() {
    fixture::assert_bound_prefix_onset_is_form_local();
}

#[test]
fn realized_possessive_ending_selects_guarded_suffix_forms_and_rejects_mutations() {
    fixture::assert_possessive_ending_selects_guarded_suffix_forms();
}

#[test]
fn exact_name_wrapper_render_uses_frozen_context_onset() {
    fixture::assert_exact_name_render_uses_frozen_onset();
}

#[test]
fn unsigned_numeral_codecs_cover_canonical_surfaces_bounds_and_round_trips() {
    fixture::assert_unsigned_numeral_codecs_are_canonical_and_total();
}

#[test]
fn nonzero_unsigned_decimal_is_typed_canonical_and_exact() {
    fixture::assert_nonzero_unsigned_decimal_is_typed_canonical_and_exact();
}

#[test]
fn sequence_agreement_is_uniform_across_every_member_and_boundary() {
    fixture::assert_sequence_agreement_is_uniform_across_every_member();
}

#[test]
fn singleton_sequence_agreement_crosses_checked_build_render_scan_and_materialization() {
    fixture::assert_singleton_sequence_agreement_crosses_every_runtime_boundary();
}

#[test]
fn sum_sequence_agreement_uses_the_explicit_carrier_across_every_member_and_boundary() {
    fixture::assert_sum_sequence_agreement_uses_the_explicit_sum_carrier();
}
