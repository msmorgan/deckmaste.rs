#![allow(
    dead_code,
    reason = "the consumer compiles every generated phase while executing build, render, and visitor boundaries"
)]

use deckmaste_construction::constructions;

pub mod environment {
    use std::collections::HashMap;

    use deckmaste_construction_core::macro_def::DeclarationIdentity;
    use deckmaste_construction_core::macro_def::GrammarRecipe;
    use deckmaste_construction_core::macro_def::NormalizedDeclaration;
    use deckmaste_construction_core::macro_def::NounLocativeTemporalLicense;
    use deckmaste_construction_core::macro_def::NounRelationality;
    use deckmaste_construction_core::macro_def::Onset;
    use deckmaste_construction_core::macro_def::SurfaceFeature;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub(crate) struct CoreVerbIdentity;

    impl CoreVerbIdentity {
        #[expect(
            clippy::unused_self,
            reason = "the fixture preserves the generated consumer API used by real identities"
        )]
        pub(crate) const fn owner_id(self) -> &'static str {
            "fixture/core-verb"
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub(crate) enum VerbInventoryRef {
        Core(CoreVerbIdentity),
        Declaration(DeclarationIdentity),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct VerbInventoryReading {
        reference: VerbInventoryRef,
        onset: Onset,
    }

    impl VerbInventoryReading {
        pub(crate) fn new(reference: VerbInventoryRef, onset: Onset) -> Self {
            Self { reference, onset }
        }

        pub(crate) fn reference(&self) -> &VerbInventoryRef {
            &self.reference
        }

        pub(crate) const fn onset(&self) -> Onset {
            self.onset
        }
    }

    pub(crate) struct ParserEnvironment {
        declarations: Vec<NormalizedDeclaration>,
        noun_class_overrides:
            HashMap<DeclarationIdentity, (NounLocativeTemporalLicense, NounRelationality)>,
    }

    impl ParserEnvironment {
        pub(crate) fn new(declarations: Vec<NormalizedDeclaration>) -> Self {
            Self {
                declarations,
                noun_class_overrides: HashMap::new(),
            }
        }

        pub(crate) fn with_noun_class_override(
            mut self,
            id: DeclarationIdentity,
            locative_temporal_license: NounLocativeTemporalLicense,
            relationality: NounRelationality,
        ) -> Self {
            self.noun_class_overrides
                .insert(id, (locative_temporal_license, relationality));
            self
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
                .map(deckmaste_construction_core::macro_def::RealizedSurface::text)
        }

        pub(crate) fn grammar_recipe(&self, id: &DeclarationIdentity) -> Option<&GrammarRecipe> {
            self.declarations
                .iter()
                .find(|declaration| declaration.identity() == id)
                .and_then(NormalizedDeclaration::grammar)
                .map(deckmaste_construction_core::macro_def::GrammarRow::recipe)
        }

        pub(crate) fn declaration_noun_features(
            &self,
            id: &DeclarationIdentity,
        ) -> Option<(NounLocativeTemporalLicense, NounRelationality, bool)> {
            let declaration = self
                .declarations
                .iter()
                .find(|declaration| declaration.identity() == id)?;
            let (locative_temporal_license, relationality) =
                self.noun_class_overrides.get(id).copied().or_else(|| {
                    declaration.noun_class().map(|semantics| {
                        (semantics.locative_temporal_license, semantics.relationality)
                    })
                })?;
            let number_invariant = self.surface(id, SurfaceFeature::Singular)
                == self.surface(id, SurfaceFeature::Plural);
            Some((locative_temporal_license, relationality, number_invariant))
        }

        pub(crate) fn declarations(&self) -> &[NormalizedDeclaration] {
            &self.declarations
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
                .map(deckmaste_construction_core::macro_def::RealizedSurface::onset)
        }

        pub(crate) fn verb_inventory_surface(
            &self,
            reference: &VerbInventoryRef,
            feature: SurfaceFeature,
        ) -> Option<&str> {
            match reference {
                VerbInventoryRef::Core(_) => None,
                VerbInventoryRef::Declaration(id) => self.surface(id, feature),
            }
        }

        pub(crate) fn verb_inventory_onset(
            &self,
            reference: &VerbInventoryRef,
            feature: SurfaceFeature,
        ) -> Option<Onset> {
            match reference {
                VerbInventoryRef::Core(_) => None,
                VerbInventoryRef::Declaration(id) => self.onset(id, feature),
            }
        }

        #[expect(
            clippy::unused_self,
            reason = "the fixture preserves the environment lookup API used by generated code"
        )]
        pub(crate) fn verb_inventory_owner_id(
            &self,
            reference: &VerbInventoryRef,
        ) -> Option<&'static str> {
            match reference {
                VerbInventoryRef::Core(identity) => Some(identity.owner_id()),
                VerbInventoryRef::Declaration(_) => None,
            }
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
            position: deckmaste_construction_core::macro_def::GrammarPosition,
            wanted: FeatureConstraint<Number>,
            right_boundary: LexicalBoundary,
        ) -> Vec<(
            usize,
            deckmaste_construction_core::macro_def::DeclarationIdentity,
            deckmaste_construction_core::macro_def::SurfaceFeature,
            deckmaste_construction_core::macro_def::Onset,
        )> {
            assert_eq!(
                position,
                deckmaste_construction_core::macro_def::GrammarPosition::Noun
            );
            self.environment
                .noun_rows()
                .into_iter()
                .filter_map(|(id, feature, onset, surface)| {
                    let number = match feature {
                        deckmaste_construction_core::macro_def::SurfaceFeature::Singular => Number::Singular,
                        deckmaste_construction_core::macro_def::SurfaceFeature::Plural => Number::Plural,
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
            deckmaste_construction_core::macro_def::DeclarationIdentity,
            deckmaste_construction_core::macro_def::SurfaceFeature,
            deckmaste_construction_core::macro_def::Onset,
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
        codec Noun {
            generate declaration_noun {
                closed = NounLexeme;
                position = Noun;
                kinds = [Type, Subtype];
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
                left: lex Noun,
                right_source: NumberSource,
                right: lex Noun,
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
                left: lex Noun,
                right_source: NumberSource,
                right: lex Noun,
            }
            derive left.number = left_source.number;
            derive right.number = right_source.number;
            derive number = output_source.number;
            form elsewhere_pair = output_source left_source noun(left) right_source noun(right);
        }
        construction modified: Phrase {
            element ModifiedPhrase {
                modifier: lex Noun,
                head: lex Noun,
            }
            derive modifier.number = Values::Singular;
            derive head.number = modifier.number;
            derive number = head.number;
            form modified = noun(modifier) noun(head);
        }
        construction article_noun: InflectedArticle {
            element ArticleNoun {
                source: NumberSource,
                head: lex Noun,
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

    fn declaration(
        path: &str,
        source: &str,
    ) -> deckmaste_construction_core::macro_def::DeclarationSource {
        deckmaste_construction_core::macro_def::DeclarationSource::new(path, source)
    }

    fn environment() -> crate::environment::ParserEnvironment {
        let declarations = deckmaste_construction_core::macro_def::read_sources(vec![
            declaration(
                "/synthetic/types/Relic.ron",
                r#"Type(name:"Relic",spelling:"relic",grammar:Noun(singular:"relic",plural:"relic"))"#,
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
        crate::environment::ParserEnvironment::new(declarations).with_noun_class_override(
            deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                deckmaste_construction_core::macro_def::DeclarationKind::Type,
                "Relic",
            ),
            deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::Unlicensed,
            deckmaste_construction_core::macro_def::NounRelationality::NonRelational,
        )
    }

    struct Recorder(Vec<String>);

    impl Visitor for Recorder {
        fn visit_declaration(
            &mut self,
            declaration: &deckmaste_construction_core::macro_def::DeclarationIdentity,
        ) {
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
                    matcher: Lexical::DeclarationNoun(1, FeatureConstraint::Any,),
                    owner: LexicalOwnerTemplate::DeclarationNoun(1),
                    ..
                }
            ),
            "unexpected generated head terminal: {head:?}"
        );
        (*modifier, *head)
    }

    fn assert_aggregate_noun_domain(
        environment: &crate::environment::ParserEnvironment,
    ) -> deckmaste_construction_core::macro_def::DeclarationIdentity {
        let subtype = |category, name| {
            deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                deckmaste_construction_core::macro_def::DeclarationKind::Subtype(category),
                name,
            )
        };
        let declarations = [
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Artifact,
                "Clue",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Battle,
                "Siege",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Creature,
                "Elf",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Enchantment,
                "Aura",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Land,
                "Forest",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Planeswalker,
                "Jace",
            ),
            subtype(
                deckmaste_construction_core::macro_def::SubtypeCategory::Spell,
                "Arcane",
            ),
        ];
        for declaration in &declarations {
            assert!(
                DeclarationNoun::new(environment, declaration.clone()).is_some(),
                "every subtype family contributes to the aggregate noun inventory",
            );
        }
        assert!(
            DeclarationNoun::new(
                environment,
                deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                    deckmaste_construction_core::macro_def::DeclarationKind::KeywordAbility,
                    "Fraud",
                ),
            )
            .is_none(),
            "a noncontributing declaration kind stays outside the noun inventory",
        );
        declarations[2].clone()
    }
    pub(crate) fn run() {
        let environment = environment();
        let context = ParseContext::default();
        let relic = deckmaste_construction_core::macro_def::DeclarationIdentity::new(
            deckmaste_construction_core::macro_def::DeclarationKind::Type,
            "Relic",
        );
        let elf = assert_aggregate_noun_domain(&environment);

        let public_type = DeclarationNoun::new(&environment, relic.clone())
            .expect("the public declaration noun stores a valid identity");
        assert_eq!(public_type.id(), &relic);
        let licensed_subtype = DeclarationNoun::new(&environment, elf.clone())
            .expect("the licensed declaration noun stores a valid identity");
        assert_eq!(
            public_type.locative_temporal_license(),
            deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::Unlicensed,
        );
        assert_eq!(
            licensed_subtype.locative_temporal_license(),
            deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::ObjectAttachmentLicensed,
        );
        assert_eq!(
            public_type.relationality(),
            deckmaste_construction_core::macro_def::NounRelationality::NonRelational,
        );
        assert_eq!(
            licensed_subtype.relationality(),
            deckmaste_construction_core::macro_def::NounRelationality::QualifiedRelational,
        );
        assert!(public_type.number_invariant());
        assert!(!licensed_subtype.number_invariant());

        let (modifier_terminal, head_terminal) = phrase_rule_terminals();
        let modifier = scan_terminal(&environment, &context, "Relic Elf.", 0, modifier_terminal);
        let head = scan_terminal(&environment, &context, "Relic Elf.", 5, head_terminal);
        assert!(
            !scan_terminal(&environment, &context, "Elf.", 0, modifier_terminal).is_empty(),
            "one noun inventory admits a subtype in the modifier role",
        );
        assert!(
            !scan_terminal(&environment, &context, "Relic.", 0, head_terminal).is_empty(),
            "one noun inventory admits a type in the head role",
        );
        let unknown_terminal_index = LexicalTerminal {
            matcher: Lexical::DeclarationNoun(2, FeatureConstraint::Exact(Number::Singular)),
            owner: LexicalOwnerTemplate::DeclarationNoun(2),
            right_boundary: LexicalBoundary::Separated,
        };
        assert!(
            scan_terminal(&environment, &context, "Relic.", 0, unknown_terminal_index,).is_empty(),
            "an unknown declaration-noun terminal index rejects without a union scan",
        );
        assert!(
            !scan_terminal(&environment, &context, "Clue.", 0, head_terminal).is_empty(),
            "every subtype contributor enters the same noun terminal",
        );
        assert!(
            scan(&environment, &context, "Relic.", 0, usize::MAX).is_empty(),
            "an unknown declaration-noun terminal index rejects without a union scan",
        );
        assert!(matches!(
            modifier.as_slice(),
            [LexicalMatch {
                value: Leaf::Noun {
                    noun: Noun::Declaration(noun),
                    number: Number::Singular,
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
                    ..
                },
                ..
            }] if noun.id() == &relic
        ));
        assert!(matches!(
            head.as_slice(),
            [LexicalMatch {
                value: Leaf::Noun {
                    noun: Noun::Declaration(noun),
                    number: Number::Singular,
                    onset: deckmaste_construction_core::macro_def::Onset::Vowel,
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
        let BuildValue::Phrase(phrase, _) = built else {
            panic!("the generated compound noun builds its declared root")
        };
        assert_eq!(
            Render::render(&phrase, &context, &environment),
            "Relic Elf."
        );

        let Leaf::Noun { noun, .. } = &head[0].value else { unreachable!() };
        let mismatched = Leaf::Noun {
            noun: noun.clone(),
            number: Number::Plural,
            onset: deckmaste_construction_core::macro_def::Onset::Vowel,
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
                FeatureConstraint::Any,
            ),
            Number::Plural => BuildValue::NumberSource(
                NumberSource::Plural(PluralNumberSource),
                Number::Plural,
                FeatureConstraint::Any,
            ),
        }
    }

    fn declaration_leaf(noun: &Leaf, number: Number) -> BuildValue {
        match noun {
            Leaf::Noun {
                noun,
                onset,
                possessive_ending,
                ..
            } => BuildValue::Leaf(Leaf::Noun {
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
        let right = &scan(&environment, &context, "Elf", 0, 1)[0].value;

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
                deckmaste_construction_core::macro_def::Onset::Vowel,
                RuleId::InflectedArticleArticleNounAn,
                "An singular artifact.",
            ),
            (
                "Units",
                Number::Plural,
                deckmaste_construction_core::macro_def::Onset::Consonant,
                RuleId::InflectedArticleArticleNounA,
                "A plural units.",
            ),
        ] {
            let rule = if number == Number::Singular { singular_rule } else { plural_rule };
            let scanned = scan_terminal(&environment, &context, text, 0, noun_terminal(rule));
            assert!(matches!(
                scanned.as_slice(),
                [LexicalMatch {
                    value: Leaf::Noun {
                        noun: Noun::Lexeme(NounLexeme::Artifact),
                        number: actual_number,
                        onset,
                        ..
                    },
                    ..
                }] if *actual_number == number && *onset == expected_onset
            ));
            let literal = if expected_onset == deckmaste_construction_core::macro_def::Onset::Vowel
            {
                "an"
            } else {
                "a"
            };
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
            let BuildValue::InflectedArticle(article, onset, _) = built else {
                panic!("the article construction carries its frozen onset")
            };
            assert_eq!(onset, expected_onset);
            assert_eq!(
                Render::render(&article, &context, &environment),
                expected_render
            );
            let inverse = if expected_onset == deckmaste_construction_core::macro_def::Onset::Vowel
            {
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
        let right = &scan(&environment, &context, "Elf", 0, 1)[0].value;
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

pub mod declaration_verb_fixture {
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

        fn declaration_verb_readings(
            &self,
            start: usize,
            frame: &VerbFrameKey,
            feature: deckmaste_construction_core::macro_def::SurfaceFeature,
        ) -> Vec<(usize, crate::environment::VerbInventoryReading)> {
            assert_eq!(start, self.position.byte_offset);
            self.environment
                .declarations()
                .iter()
                .filter_map(|declaration| {
                    let grammar = declaration.grammar()?;
                    (declaration.identity().kind()
                        == deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction
                        && grammar.recipe().position()
                            == deckmaste_construction_core::macro_def::GrammarPosition::Verb)
                        .then_some((declaration, grammar))
                })
                .filter(|(declaration, grammar)| {
                    let deckmaste_construction_core::macro_def::GrammarRecipe::Verb { frame_set } =
                        grammar.recipe()
                    else {
                        return false;
                    };
                    fixture_frames_for(declaration.identity().name(), frame_set)
                        .contains(&frame.atoms())
                })
                .filter_map(|(declaration, grammar)| {
                    grammar
                        .surfaces()
                        .iter()
                        .find(|surface| surface.feature() == feature)
                        .and_then(|surface| {
                            self.word_end(surface.text(), LexicalBoundary::Separated)
                                .map(|end| {
                                    (
                                        end,
                                        crate::environment::VerbInventoryReading::new(
                                            crate::environment::VerbInventoryRef::Declaration(
                                                declaration.identity().clone(),
                                            ),
                                            surface.onset(),
                                        ),
                                    )
                                })
                        })
                })
                .collect()
        }

        fn declaration_readings(
            &self,
            _matcher: DeclarationMatcher,
            _right_boundary: LexicalBoundary,
        ) -> Vec<(
            usize,
            deckmaste_construction_core::macro_def::DeclarationIdentity,
            deckmaste_construction_core::macro_def::SurfaceFeature,
            deckmaste_construction_core::macro_def::Onset,
        )> {
            debug_assert_eq!(self.context.marker, std::marker::PhantomData);
            Vec::new()
        }
    }

    fn fixture_frames_for(
        name: &str,
        frame_set: &deckmaste_construction_core::macro_def::VerbFrameSet,
    ) -> &'static [&'static [VerbFrameAtom]] {
        use deckmaste_construction_core::macro_def::CustomTailAtom;
        use deckmaste_construction_core::macro_def::VerbFrameSet;

        const EMPTY: &[VerbFrameAtom] = &[];
        const OBJECT: &[VerbFrameAtom] = &[VerbFrameAtom::ObjectNounPhrase];
        const AMOUNT: &[VerbFrameAtom] = &[VerbFrameAtom::Amount];
        const CROSSED: &[VerbFrameAtom] = &[VerbFrameAtom::ObjectNounPhrase, VerbFrameAtom::Amount];
        const DUPLICATED: &[VerbFrameAtom] = &[VerbFrameAtom::Amount, VerbFrameAtom::Amount];
        const EXTRA: &[VerbFrameAtom] = &[VerbFrameAtom::Amount, VerbFrameAtom::Literal("extra")];
        const LEX_MARKED: &[VerbFrameAtom] = &[VerbFrameAtom::Lex("AmountWord", "One")];
        const NO_FRAMES: &[&[VerbFrameAtom]] = &[];
        const TRANSITIVE: &[&[VerbFrameAtom]] = &[OBJECT];
        const MEASURE_COMPLEMENT: &[&[VerbFrameAtom]] = &[AMOUNT];
        const SHAPE: &[&[VerbFrameAtom]] = &[EMPTY, AMOUNT];
        const CROSSED_ONLY: &[&[VerbFrameAtom]] = &[CROSSED];
        const DUPLICATED_ONLY: &[&[VerbFrameAtom]] = &[DUPLICATED];
        const EXTRA_ONLY: &[&[VerbFrameAtom]] = &[EXTRA];
        const LEX_MARKED_ONLY: &[&[VerbFrameAtom]] = &[LEX_MARKED];

        match (name, frame_set) {
            (
                "FirstAct" | "SecondAct" | "Cast" | "MissingAgreement" | "WrongKind",
                VerbFrameSet::Transitive,
            ) => TRANSITIVE,
            ("FirstAct" | "Count", VerbFrameSet::MeasureComplement) => MEASURE_COMPLEMENT,
            ("Shape", VerbFrameSet::Custom { frames })
                if frames.as_slice() == [Vec::new(), vec![CustomTailAtom::Amount]] =>
            {
                SHAPE
            }
            ("Crossed", VerbFrameSet::Custom { frames })
                if frames.as_slice()
                    == [vec![
                        CustomTailAtom::ObjectNounPhrase,
                        CustomTailAtom::Amount,
                    ]] =>
            {
                CROSSED_ONLY
            }
            ("DuplicatedTail", VerbFrameSet::Custom { frames })
                if frames.as_slice() == [vec![CustomTailAtom::Amount, CustomTailAtom::Amount]] =>
            {
                DUPLICATED_ONLY
            }
            ("Extra", VerbFrameSet::Custom { frames })
                if frames.as_slice()
                    == [vec![
                        CustomTailAtom::Amount,
                        CustomTailAtom::Literal("extra".to_owned()),
                    ]] =>
            {
                EXTRA_ONLY
            }
            ("LexMarked", VerbFrameSet::Custom { frames })
                if frames.as_slice()
                    == [vec![CustomTailAtom::Lex(
                        "AmountWord".to_owned(),
                        "One".to_owned(),
                    )]] =>
            {
                LEX_MARKED_ONLY
            }
            _ => NO_FRAMES,
        }
    }

    constructions! {
        morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
        morphology EnglishParticiple { feature = Participle; recipe = english_participle; }
        lexeme CoreVerb using EnglishVerb { Act = "act", }
        vocab ObjectWord { Object = "object", }
        vocab AmountWord { One = "one", }
        codec TransitiveVerb {
            generate declaration_verb {
                closed = CoreVerb;
                position = Verb;
                tail = [ObjectNounPhrase];
                feature = Agreement;
            }
        }
        codec MeasureComplementVerb {
            generate declaration_verb {
                position = Verb;
                tail = [Amount];
                feature = Agreement;
            }
        }
        codec IntransitiveVerb {
            generate declaration_verb {
                position = Verb;
                tail = [];
                feature = Agreement;
            }
        }
        codec TransitiveParticiple {
            generate declaration_verb {
                position = Verb;
                tail = [ObjectNounPhrase];
                feature = Participle;
            }
        }
        codec LexMarkedVerb {
            generate declaration_verb {
                position = Verb;
                tail = [lex(AmountWord::One)];
                feature = Agreement;
            }
        }
        construction transitive: VerbPhrase {
            element Transitive {
                head: lex TransitiveVerb,
                object: lex ObjectWord,
            }
            derive head.agreement = Values::Bare;
            form transitive = verb(head) lex(object);
        }
        construction measure_complement: MeasureComplementPhrase {
            element MeasureComplement {
                head: lex MeasureComplementVerb,
                amount: lex AmountWord,
            }
            derive head.agreement = Values::Bare;
            form measure_complement = verb(head) lex(amount);
        }
        construction intransitive: IntransitivePhrase {
            element Intransitive { head: lex IntransitiveVerb, }
            derive head.agreement = Values::Bare;
            form intransitive = verb(head);
        }
        construction lex_marked: LexMarkedPhrase {
            element LexMarked { head: lex LexMarkedVerb, }
            derive head.agreement = Values::Bare;
            form lex_marked = verb(head) lex(AmountWord::One);
        }
        construction participle: ParticiplePhrase {
            element Participial {
                head: lex TransitiveParticiple,
                object: lex ObjectWord,
            }
            form participle = verb(head) lex(object);
        }
        root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        root MeasureComplementPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        root IntransitivePhrase { punctuation = "."; eoi = true; standalone_render = true; }
        root LexMarkedPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        root ParticiplePhrase { punctuation = "."; eoi = true; standalone_render = true; }
    }

    impl crate::environment::ParserEnvironment {
        fn verb_frame_licenses(
            &self,
            reference: &crate::environment::VerbInventoryRef,
            frame: VerbFrameKey,
        ) -> bool {
            let crate::environment::VerbInventoryRef::Declaration(id) = reference else {
                return false;
            };
            if id.kind() != deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction {
                return false;
            }
            let Some(deckmaste_construction_core::macro_def::GrammarRecipe::Verb { frame_set }) =
                self.grammar_recipe(id)
            else {
                return false;
            };
            fixture_frames_for(id.name(), frame_set).contains(&frame.atoms())
        }

        fn verb_frame_prepositional_adjunct_licensed(
            &self,
            reference: &crate::environment::VerbInventoryRef,
            frame: VerbFrameKey,
        ) -> bool {
            let crate::environment::VerbInventoryRef::Declaration(id) = reference else {
                return false;
            };
            self.grammar_recipe(id).is_some_and(|recipe| {
                let deckmaste_construction_core::macro_def::GrammarRecipe::Verb { frame_set } =
                    recipe
                else {
                    return false;
                };
                fixture_frames_for(id.name(), frame_set).contains(&frame.atoms())
                    && frame_set.prepositional_adjunct_licensed()
            })
        }

        fn verb_frame_nonprepositional_adjunct_licensed(
            &self,
            reference: &crate::environment::VerbInventoryRef,
            frame: VerbFrameKey,
        ) -> bool {
            let crate::environment::VerbInventoryRef::Declaration(id) = reference else {
                return false;
            };
            self.grammar_recipe(id).is_some_and(|recipe| {
                let deckmaste_construction_core::macro_def::GrammarRecipe::Verb { frame_set } =
                    recipe
                else {
                    return false;
                };
                fixture_frames_for(id.name(), frame_set).contains(&frame.atoms())
                    && frame_set.nonprepositional_adjunct_licensed()
            })
        }
    }

    fn declaration_id(
        reference: &crate::environment::VerbInventoryRef,
    ) -> &deckmaste_construction_core::macro_def::DeclarationIdentity {
        let crate::environment::VerbInventoryRef::Declaration(id) = reference else {
            panic!("the fixture stores a declaration-backed verb")
        };
        id
    }

    fn declaration(
        path: &str,
        source: &str,
    ) -> deckmaste_construction_core::macro_def::DeclarationSource {
        deckmaste_construction_core::macro_def::DeclarationSource::new(path, source)
    }

    fn sources() -> Vec<deckmaste_construction_core::macro_def::DeclarationSource> {
        vec![
            declaration(
                "/synthetic/actions/FirstAct.ron",
                r#"KeywordAction(name:"FirstAct",spelling:"act",grammar:Verb(bare:"act",frame_set:Transitive))"#,
            ),
            declaration(
                "/synthetic/actions/Cast.ron",
                r#"KeywordAction(name:"Cast",spelling:"cast",grammar:Verb(bare:"cast",participle:"cast",frame_set:Transitive))"#,
            ),
            declaration(
                "/synthetic/actions/SecondAct.ron",
                r#"KeywordAction(name:"SecondAct",spelling:"act",grammar:Verb(bare:"act",frame_set:Transitive))"#,
            ),
            declaration(
                "/synthetic/actions/Count.ron",
                r#"KeywordAction(name:"Count",spelling:"count",grammar:Verb(bare:"count",frame_set:MeasureComplement))"#,
            ),
            declaration(
                "/synthetic/actions/Shape.ron",
                r#"KeywordAction(name:"Shape",spelling:"shape",grammar:Verb(bare:"shape",frame_set:Custom(frames:[[],[Amount]])))"#,
            ),
            declaration(
                "/synthetic/actions/Crossed.ron",
                r#"KeywordAction(name:"Crossed",spelling:"cross",grammar:Verb(bare:"cross",frame_set:Custom(frames:[[ObjectNounPhrase,Amount]])))"#,
            ),
            declaration(
                "/synthetic/actions/DuplicatedTail.ron",
                r#"KeywordAction(name:"DuplicatedTail",spelling:"double",grammar:Verb(bare:"double",frame_set:Custom(frames:[[Amount,Amount]])))"#,
            ),
            declaration(
                "/synthetic/actions/Extra.ron",
                r#"KeywordAction(name:"Extra",spelling:"extend",grammar:Verb(bare:"extend",frame_set:Custom(frames:[[Amount,Literal("extra")]])))"#,
            ),
            declaration(
                "/synthetic/actions/LexMarked.ron",
                r#"KeywordAction(name:"LexMarked",spelling:"mark",grammar:Verb(bare:"mark",frame_set:Custom(frames:[[Lex("AmountWord","One")]])))"#,
            ),
            declaration(
                "/synthetic/actions/WrongKind.ron",
                r#"KeywordAbility(name:"WrongKind",spelling:"mimic",grammar:Verb(bare:"mimic",frame_set:Transitive))"#,
            ),
            declaration(
                "/synthetic/actions/WrongPosition.ron",
                r#"KeywordAction(name:"WrongPosition",spelling:"static",grammar:FixedTerm(surface:"static"))"#,
            ),
            declaration(
                "/synthetic/actions/MissingAgreement.ron",
                r#"KeywordAction(name:"MissingAgreement",spelling:"wane",grammar:Verb(bare:"wane",third_person:Unavailable,frame_set:Transitive))"#,
            ),
        ]
    }

    fn environment_from_sources(
        sources: Vec<deckmaste_construction_core::macro_def::DeclarationSource>,
    ) -> crate::environment::ParserEnvironment {
        let declarations = deckmaste_construction_core::macro_def::read_sources(sources)
            .expect("synthetic declaration verbs normalize");
        crate::environment::ParserEnvironment::new(declarations)
    }

    fn environment() -> crate::environment::ParserEnvironment {
        environment_from_sources(sources())
    }

    fn id(
        environment: &crate::environment::ParserEnvironment,
        name: &str,
    ) -> crate::environment::VerbInventoryRef {
        let identity = environment
            .declarations()
            .iter()
            .find(|declaration| declaration.identity().name() == name)
            .unwrap_or_else(|| panic!("synthetic declaration `{name}` exists"))
            .identity()
            .clone();
        crate::environment::VerbInventoryRef::Declaration(identity)
    }

    fn scan<'a>(
        environment: &'a crate::environment::ParserEnvironment,
        context: &'a ParseContext<'a>,
        text: &'a str,
        terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        scan_lexical(
            &ScanInput {
                text,
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment,
                context,
            },
            terminal,
        )
    }

    fn rule_terminals(rule_id: RuleId) -> (LexicalTerminal, LexicalTerminal) {
        let rule = RULES
            .iter()
            .find(|rule| rule.id == rule_id)
            .expect("the generated declaration verb rule is present");
        let [L(verb), L(tail)] = rule.rhs else {
            panic!("the declaration verb rule has a verb head and one tail")
        };
        (*verb, *tail)
    }

    fn first_terminal(rule_id: RuleId) -> LexicalTerminal {
        let rule = RULES
            .iter()
            .find(|rule| rule.id == rule_id)
            .expect("the generated declaration verb rule is present");
        let Some(L(terminal)) = rule.rhs.first() else {
            panic!("the declaration verb rule begins with its projected verb")
        };
        *terminal
    }

    fn scanned_declaration_names(
        environment: &crate::environment::ParserEnvironment,
        text: &str,
        terminal: LexicalTerminal,
    ) -> Vec<String> {
        let context = ParseContext::default();
        scan(environment, &context, text, terminal)
            .into_iter()
            .filter_map(|candidate| match candidate.value {
                Leaf::TransitiveVerb {
                    verb: TransitiveVerb::Declaration(value),
                    ..
                } => Some(declaration_id(value.reference()).name().to_owned()),
                Leaf::MeasureComplementVerb { verb, .. } => {
                    Some(declaration_id(verb.reference()).name().to_owned())
                }
                Leaf::IntransitiveVerb { verb, .. } => {
                    Some(declaration_id(verb.reference()).name().to_owned())
                }
                Leaf::LexMarkedVerb { verb, .. } => {
                    Some(declaration_id(verb.reference()).name().to_owned())
                }
                _ => None,
            })
            .collect()
    }

    struct Recorder(Vec<String>);

    impl Visitor for Recorder {
        fn visit_declaration(
            &mut self,
            declaration: &deckmaste_construction_core::macro_def::DeclarationIdentity,
        ) {
            self.0.push(format!("head:{declaration}"));
        }

        fn visit_core_verb(&mut self, verb: CoreVerb) {
            self.0.push(format!("head:core:{verb:?}"));
        }

        fn visit_object_word(&mut self, object: ObjectWord) {
            self.0.push(format!("object:{object:?}"));
        }

        fn visit_amount_word(&mut self, amount: AmountWord) {
            self.0.push(format!("amount:{amount:?}"));
        }
    }

    pub(crate) fn run() {
        let environment = environment();
        let context = ParseContext::default();
        let (verb_terminal, object_terminal) = rule_terminals(RuleId::VerbPhraseTransitive);
        assert!(matches!(
            verb_terminal,
            LexicalTerminal {
                matcher: Lexical::DeclarationVerb(3, FeatureConstraint::Exact(Agreement::Bare)),
                owner: LexicalOwnerTemplate::DeclarationVerb(3),
                ..
            }
        ));

        let verbs = scan(&environment, &context, "Act object.", verb_terminal);
        assert_eq!(
            verbs.len(),
            3,
            "one closed and two open homonyms survive scanning"
        );
        assert!(matches!(
            &verbs[0].value,
            Leaf::TransitiveVerb {
                verb: TransitiveVerb::Lexeme(CoreVerb::Act),
                agreement: Agreement::Bare,
                onset: deckmaste_construction_core::macro_def::Onset::Vowel,
            }
        ));
        let open_ids = verbs[1..]
            .iter()
            .map(|candidate| match &candidate.value {
                Leaf::TransitiveVerb {
                    verb: TransitiveVerb::Declaration(declaration),
                    agreement: Agreement::Bare,
                    onset: deckmaste_construction_core::macro_def::Onset::Vowel,
                } => declaration_id(declaration.reference()).name(),
                other => panic!("unexpected declaration verb candidate: {other:?}"),
            })
            .collect::<Vec<_>>();
        assert_eq!(open_ids, ["FirstAct", "SecondAct"]);
        assert_eq!(
            verb_terminal
                .owner
                .instantiate(&verbs[1].value)
                .expect("the declaration verb leaf materializes its lexical owner")
                .stable_id(),
            "lexeme:keyword_action/FirstAct/bare",
        );

        let objects = scan(&environment, &context, "Object.", object_terminal);
        assert!(matches!(
            objects.as_slice(),
            [LexicalMatch {
                value: Leaf::ObjectWord(ObjectWord::Object),
                ..
            }]
        ));

        let mut built = Vec::new();
        for candidate in &verbs {
            let value = build(
                RuleId::VerbPhraseTransitive,
                &[
                    BuildValue::Leaf(candidate.value.clone()),
                    BuildValue::Leaf(objects[0].value.clone()),
                ],
                &context,
            )
            .expect("every preserved homonym materializes through the same rule");
            let BuildValue::VerbPhrase(phrase, _) = value else {
                panic!("the transitive rule builds its declared category")
            };
            assert!(matches!(phrase, VerbPhrase::Transitive(_)));
            built.push(phrase);
        }

        for (index, phrase) in built.iter().enumerate() {
            assert_eq!(
                Render::render(phrase, &context, &environment),
                "Act object."
            );
            let mut recorder = Recorder(Vec::new());
            walk_verb_phrase(&mut recorder, phrase);
            let expected_head = match index {
                0 => "head:core:Act".to_owned(),
                1 => "head:keyword action `FirstAct`".to_owned(),
                2 => "head:keyword action `SecondAct`".to_owned(),
                _ => unreachable!(),
            };
            assert_eq!(
                recorder.0,
                [expected_head, "object:Object".to_owned()],
                "the generated traversal visits the verb head before its object",
            );
        }

        let (rendered, claims) = render_verb_phrase_with_claims(&built[1], &context, &environment);
        assert_eq!(rendered, "Act object.");
        let claims = claims
            .iter()
            .map(|claim| (claim.start, claim.end, claim.owner.stable_id()))
            .collect::<Vec<_>>();
        assert_eq!(
            claims,
            [
                (0, 3, "lexeme:keyword_action/FirstAct/bare"),
                (3, 10, "vocab:ObjectWord/Object"),
                (10, 11, "root:VerbPhrase/punctuation"),
            ],
            "rendering emits a gap-free, overlap-free lexical ownership partition",
        );
    }

    pub(crate) fn run_participle() {
        let environment = environment();
        let context = ParseContext::default();
        let (verb_terminal, object_terminal) = rule_terminals(RuleId::ParticiplePhraseParticiple);
        assert!(matches!(
            verb_terminal,
            LexicalTerminal {
                matcher: Lexical::DeclarationParticiple(_),
                owner: LexicalOwnerTemplate::DeclarationVerb(_),
                ..
            }
        ));

        let verbs = scan(&environment, &context, "Cast object.", verb_terminal);
        let [verb] = verbs.as_slice() else {
            panic!("the explicit cast participle yields exactly one reading")
        };
        let Leaf::TransitiveParticiple {
            verb: identity,
            onset: deckmaste_construction_core::macro_def::Onset::Consonant,
        } = &verb.value
        else {
            panic!("the participle leaf stores only its checked declaration identity")
        };
        assert_eq!(declaration_id(identity.reference()).name(), "Cast");
        assert_eq!(
            verb_terminal
                .owner
                .instantiate(&verb.value)
                .expect("the participle materializes declaration provenance")
                .stable_id(),
            "lexeme:keyword_action/Cast/participle",
        );

        let objects = scan(&environment, &context, "Object.", object_terminal);
        let value = build(
            RuleId::ParticiplePhraseParticiple,
            &[
                BuildValue::Leaf(verb.value.clone()),
                BuildValue::Leaf(objects[0].value.clone()),
            ],
            &context,
        )
        .expect("the participle and its exact transitive tail build");
        let BuildValue::ParticiplePhrase(phrase, _) = value else {
            panic!("the participle rule builds its declared category")
        };
        assert_eq!(
            Render::render(&phrase, &context, &environment),
            "Cast object."
        );
        let mut recorder = Recorder(Vec::new());
        walk_participle_phrase(&mut recorder, &phrase);
        assert_eq!(recorder.0, ["head:keyword action `Cast`", "object:Object"]);
        let (rendered, claims) =
            render_participle_phrase_with_claims(&phrase, &context, &environment);
        assert_eq!(rendered, "Cast object.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 4, "lexeme:keyword_action/Cast/participle"),
                (4, 11, "vocab:ObjectWord/Object"),
                (11, 12, "root:ParticiplePhrase/punctuation"),
            ]
        );

        assert!(
            DeclarationTransitiveParticiple::new(&environment, id(&environment, "Cast")).is_some()
        );
        assert!(
            DeclarationTransitiveParticiple::new(&environment, id(&environment, "Count")).is_none()
        );
    }

    pub(crate) fn run_checked_constructors() {
        let environment = environment();
        let transitive = first_terminal(RuleId::VerbPhraseTransitive);
        assert!(
            DeclarationTransitiveVerb::new(&environment, id(&environment, "FirstAct")).is_some()
        );
        assert!(
            DeclarationMeasureComplementVerb::new(&environment, id(&environment, "Count"))
                .is_some()
        );
        assert!(DeclarationTransitiveVerb::new(&environment, id(&environment, "Count")).is_none());
        assert!(
            DeclarationMeasureComplementVerb::new(&environment, id(&environment, "FirstAct"))
                .is_none()
        );
        assert!(
            DeclarationTransitiveVerb::new(&environment, id(&environment, "WrongKind")).is_none()
        );
        assert!(
            DeclarationTransitiveVerb::new(&environment, id(&environment, "WrongPosition"))
                .is_none()
        );
        assert!(
            DeclarationTransitiveVerb::new(&environment, id(&environment, "MissingAgreement"))
                .is_none()
        );
        assert!(
            scanned_declaration_names(&environment, "Wane object.", transitive).is_empty(),
            "scanner materialization must enforce the complete wrapper invariant",
        );

        let absent = environment_from_sources(vec![declaration(
            "/disposable/Absent.ron",
            r#"KeywordAction(name:"Absent",spelling:"absent",grammar:Verb(bare:"absent",frame_set:Transitive))"#,
        )]);
        assert!(DeclarationTransitiveVerb::new(&environment, id(&absent, "Absent")).is_none());
    }

    pub(crate) fn run_custom_shapes() {
        let environment = environment();
        let transitive = first_terminal(RuleId::VerbPhraseTransitive);
        let measure_complement = first_terminal(RuleId::MeasureComplementPhraseMeasureComplement);
        let intransitive = first_terminal(RuleId::IntransitivePhraseIntransitive);

        assert_eq!(
            scanned_declaration_names(&environment, "Shape.", intransitive),
            ["Shape"]
        );
        assert_eq!(
            scanned_declaration_names(&environment, "Shape one.", measure_complement),
            ["Shape"]
        );
        assert!(scanned_declaration_names(&environment, "Shape object.", transitive).is_empty());
        assert!(
            DeclarationIntransitiveVerb::new(&environment, id(&environment, "Shape")).is_some()
        );
        assert!(
            DeclarationMeasureComplementVerb::new(&environment, id(&environment, "Shape"))
                .is_some()
        );
        assert!(DeclarationTransitiveVerb::new(&environment, id(&environment, "Shape")).is_none());

        let lex_marked = first_terminal(RuleId::LexMarkedPhraseLexMarked);
        assert_eq!(
            scanned_declaration_names(&environment, "Mark one.", lex_marked),
            ["LexMarked"],
            "a declared vocabulary tail marker enters its exact frame",
        );
        assert!(
            DeclarationLexMarkedVerb::new(&environment, id(&environment, "LexMarked")).is_some()
        );
        assert!(
            DeclarationTransitiveVerb::new(&environment, id(&environment, "LexMarked")).is_none()
        );
        assert!(
            DeclarationLexMarkedVerb::new(&environment, id(&environment, "Extra")).is_none(),
            "a literal tail must not satisfy a vocabulary tail marker",
        );

        for (name, surface) in [
            ("Crossed", "Cross"),
            ("DuplicatedTail", "Double"),
            ("Extra", "Extend"),
        ] {
            for terminal in [intransitive, measure_complement, transitive, lex_marked] {
                assert!(
                    scanned_declaration_names(&environment, surface, terminal).is_empty(),
                    "{name} must not enter a construction with a merely similar tail",
                );
            }
            let declaration = id(&environment, name);
            assert!(DeclarationIntransitiveVerb::new(&environment, declaration.clone()).is_none());
            assert!(
                DeclarationMeasureComplementVerb::new(&environment, declaration.clone()).is_none()
            );
            assert!(DeclarationTransitiveVerb::new(&environment, declaration).is_none());
        }

        let unsupported = deckmaste_construction_core::macro_def::read_sources(vec![declaration(
            "/disposable/Unsupported.ron",
            r#"KeywordAction(name:"Unsupported",spelling:"unsupported",grammar:Verb(bare:"unsupported",frame_set:Custom(frames:[[Clause]])))"#,
        )]);
        assert!(
            unsupported.is_err(),
            "unsupported tail atoms fail before runtime construction"
        );
    }

    pub(crate) fn run_literal_input_frame_table() {
        let environment = environment();
        let context = ParseContext::default();
        let readings = |text: &str, atoms: &'static [VerbFrameAtom]| {
            ScanInput {
                text,
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment: &environment,
                context: &context,
            }
            .declaration_verb_readings(
                0,
                &VerbFrameKey::new(atoms),
                deckmaste_construction_core::macro_def::SurfaceFeature::Bare,
            )
            .into_iter()
            .map(|(_, reading)| declaration_id(reading.reference()).name().to_owned())
            .collect::<Vec<_>>()
        };

        assert_eq!(
            readings("Act", &[VerbFrameAtom::ObjectNounPhrase]),
            ["FirstAct", "SecondAct"]
        );
        assert_eq!(readings("Count", &[VerbFrameAtom::Amount]), ["Count"]);
        assert_eq!(readings("Shape", &[]), ["Shape"]);
        assert_eq!(readings("Shape", &[VerbFrameAtom::Amount]), ["Shape"]);
        assert!(readings("Shape", &[VerbFrameAtom::ObjectNounPhrase]).is_empty());
    }

    pub(crate) fn run_open_only_runtime_boundaries() {
        let environment = environment();
        let context = ParseContext::default();
        let (verb_terminal, amount_terminal) =
            rule_terminals(RuleId::MeasureComplementPhraseMeasureComplement);
        let verbs = scan(&environment, &context, "Count one.", verb_terminal);
        let amounts = scan(&environment, &context, "One.", amount_terminal);
        let [verb] = verbs.as_slice() else {
            panic!("the open-only measure_complement codec yields exactly one Count identity")
        };
        let Leaf::MeasureComplementVerb {
            verb: identity,
            agreement: Agreement::Bare,
            onset: deckmaste_construction_core::macro_def::Onset::Consonant,
        } = &verb.value
        else {
            panic!("the open-only leaf stores its checked category-safe identity")
        };
        assert_eq!(declaration_id(identity.reference()).name(), "Count");

        let runtime_owner = verb_terminal
            .owner
            .instantiate(&verb.value)
            .expect("the open-only leaf materializes declaration provenance");
        assert_eq!(runtime_owner.kind(), LexicalProvenanceKind::Lexeme);
        assert_eq!(
            runtime_owner.stable_id(),
            "lexeme:keyword_action/Count/bare"
        );

        let [amount] = amounts.as_slice() else {
            panic!("the measure_complement tail yields exactly one amount word")
        };
        let value = build(
            RuleId::MeasureComplementPhraseMeasureComplement,
            &[
                BuildValue::Leaf(verb.value.clone()),
                BuildValue::Leaf(amount.value.clone()),
            ],
            &context,
        )
        .expect("the open-only declaration verb builds through its generated rule");
        let BuildValue::MeasureComplementPhrase(phrase, _) = value else {
            panic!("the generated rule builds its measure_complement category")
        };
        let MeasureComplementPhrase::MeasureComplement(stored) = &phrase;
        assert_eq!(declaration_id(stored.head.reference()).name(), "Count");
        assert_eq!(
            Render::render(&phrase, &context, &environment),
            "Count one."
        );

        let mut recorder = Recorder(Vec::new());
        walk_measure_complement_phrase(&mut recorder, &phrase);
        assert_eq!(
            recorder.0,
            [
                "head:keyword action `Count`".to_owned(),
                "amount:One".to_owned(),
            ],
        );

        let (rendered, claims) =
            render_measure_complement_phrase_with_claims(&phrase, &context, &environment);
        assert_eq!(rendered, "Count one.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.start, claim.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 5, "lexeme:keyword_action/Count/bare"),
                (5, 9, "vocab:AmountWord/One"),
                (9, 10, "root:MeasureComplementPhrase/punctuation"),
            ],
            "open-only rendering owns one exact disjoint complete lexical partition",
        );
    }

    pub(crate) fn run_frame_set_perturbation() {
        let baseline_sources = sources();
        let baseline_bytes = baseline_sources
            .iter()
            .find(|source| source.path.ends_with("FirstAct.ron"))
            .expect("the perturbed declaration exists")
            .source
            .clone();
        assert_eq!(baseline_bytes.matches("frame_set:Transitive").count(), 1);

        let baseline = environment_from_sources(baseline_sources.clone());
        let transitive = first_terminal(RuleId::VerbPhraseTransitive);
        let measure_complement = first_terminal(RuleId::MeasureComplementPhraseMeasureComplement);
        assert_eq!(
            scanned_declaration_names(&baseline, "Act object.", transitive),
            ["FirstAct", "SecondAct"]
        );
        assert!(scanned_declaration_names(&baseline, "Act one.", measure_complement).is_empty());

        let mut perturbed_sources = baseline_sources.clone();
        let perturbed = perturbed_sources
            .iter_mut()
            .find(|source| source.path.ends_with("FirstAct.ron"))
            .expect("the disposable declaration copy exists");
        perturbed.source =
            perturbed
                .source
                .replacen("frame_set:Transitive", "frame_set:MeasureComplement", 1);
        let perturbed_environment = environment_from_sources(perturbed_sources.clone());
        assert_eq!(
            scanned_declaration_names(&perturbed_environment, "Act object.", transitive),
            ["SecondAct"]
        );
        assert_eq!(
            scanned_declaration_names(&perturbed_environment, "Act one.", measure_complement),
            ["FirstAct"]
        );

        perturbed_sources
            .iter_mut()
            .find(|source| source.path.ends_with("FirstAct.ron"))
            .expect("the disposable declaration copy still exists")
            .source
            .clone_from(&baseline_bytes);
        assert_eq!(
            perturbed_sources
                .iter()
                .find(|source| source.path.ends_with("FirstAct.ron"))
                .expect("the restored declaration exists")
                .source,
            baseline_bytes,
        );
        let restored = environment_from_sources(perturbed_sources);
        assert_eq!(
            scanned_declaration_names(&restored, "Act object.", transitive),
            ["FirstAct", "SecondAct"]
        );
        assert!(scanned_declaration_names(&restored, "Act one.", measure_complement).is_empty());
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

    #[cfg(feature = "parser-metrics")]
    mod metrics {
        pub(crate) enum MetricEvent {
            Prediction,
            Completion,
            CloneHeavy,
        }

        pub(crate) fn record(_rule_index: usize, _event: MetricEvent) {}

        pub(crate) fn record_work(_chart_columns_visited: u64, _scan_attempts: u64) {}
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
        card_name_onset: deckmaste_construction_core::macro_def::Onset,
        abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset,
    }

    impl Default for ParseContext<'_> {
        fn default() -> Self {
            Self {
                sentinel: 0,
                card_name: "",
                abbreviated_card_name: "",
                card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
                abbreviated_card_name_onset:
                    deckmaste_construction_core::macro_def::Onset::Consonant,
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

        fn card_name_onset(&self) -> deckmaste_construction_core::macro_def::Onset {
            debug_assert_ne!(self.sentinel, 0);
            self.card_name_onset
        }

        fn abbreviated_card_name_onset(&self) -> deckmaste_construction_core::macro_def::Onset {
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
            deckmaste_construction_core::macro_def::DeclarationIdentity,
            deckmaste_construction_core::macro_def::SurfaceFeature,
            deckmaste_construction_core::macro_def::Onset,
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
        construction recursive_leaf: RecursiveNode {
            element RecursiveLeaf {}
            form recursive_leaf = "leaf";
        }
        construction recursive_optional: RecursiveNode {
            element RecursiveOptional { child: opt RecursiveNode, }
            form recursive_optional = "node" child;
        }
        construction guarded: Child {
            element GuardedChild { mode: lex Mode, child: Child, }
            require any(
                all(mode is One, child is Bare),
                all(mode is Many, child is Third)
            );
            derive agreement = Values::Bare;
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
        construction singular_number: NumberItem {
            element SingularNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Singular;
            derive onset = Values::Consonant;
            form singular_number = "one";
        }
        construction plural_number: NumberItem {
            element PluralNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Plural;
            derive onset = Values::Consonant;
            form plural_number = "many";
        }
        construction artifact_number: NumberItem {
            element ArtifactNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Singular;
            derive onset = Values::Vowel;
            form artifact_number = "artifact";
        }
        construction enchantment_number: NumberItem {
            element EnchantmentNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Singular;
            derive onset = Values::Vowel;
            form enchantment_number = "enchantment";
        }
        construction spell_number: NumberItem {
            element SpellNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Singular;
            derive onset = Values::Consonant;
            form spell_number = "spell";
        }
        construction ability_number: NumberItem {
            element AbilityNumberItem {}
            derive agreement = Values::Bare;
            derive number = Values::Singular;
            derive onset = Values::Vowel;
            form ability_number = "ability";
        }
        construction homogeneous_singular_numbers: HomogeneousNumberSequence {
            element HomogeneousSingularNumbers {
                members: seq NumberItem separated by " ",
            }
            require len(members) >= 2;
            derive members.number = Values::Singular;
            form homogeneous_singular_numbers = members;
        }
        construction relayed_numbers: RelayedNumberSequence {
            element RelayedNumbers {
                members: seq NumberItem separated by " or ",
            }
            require len(members) >= 2;
            derive agreement = members.agreement;
            derive number = members.number;
            derive onset = members.onset;
            form relayed_numbers = members;
        }
        construction singular_number_sequence_envelope: NumberSequenceEnvelope {
            element SingularNumberSequenceEnvelope {
                sequence: RelayedNumberSequence,
            }
            derive sequence.number = Values::Singular;
            form an when sequence.onset is Vowel = "an" sequence;
            form a otherwise = "a" sequence;
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
        construction uniform_mixed_child_choices: MixedChoiceSequence {
            element UniformMixedChildChoices {
                members: seq MixedAgreementChild separated by " ",
            }
            require len(members) >= 2;
            derive members.agreement = Values::Bare;
            form uniform_mixed_child_choices = members;
        }
        construction relayed_mixed_child_choices: RelayedMixedChoiceSequence {
            element RelayedMixedChildChoices {
                members: seq MixedAgreementChild separated by " ",
            }
            require len(members) >= 2;
            derive agreement = members.agreement;
            form relayed_mixed_child_choices = members;
        }
        construction intrinsic_third_mixed_choice: RelayedMixedChoiceSequence {
            element IntrinsicThirdMixedChoice {}
            derive agreement = Values::ThirdPersonSingular;
            form intrinsic_third_mixed_choice = "intrinsic third";
        }
        construction checked_bare_mixed_choice: RelayedMixedChoiceSequence {
            element CheckedBareMixedChoice { mode: lex Mode, }
            require mode is One;
            derive agreement = Values::Bare;
            form checked_bare_mixed_choice = lex(mode);
        }
        construction relayed_outer_mixed_choices: RelayedOuterMixedChoiceSequence {
            element RelayedOuterMixedChoices {
                members: seq OuterRelayedMixedChoice separated by " ",
            }
            require len(members) >= 1;
            derive agreement = members.agreement;
            form relayed_outer_mixed_choices = members;
        }
        construction mixed_relay_envelope: MixedRelayEnvelopeRoot {
            element MixedRelayEnvelope {
                choices: RelayedMixedChoiceSequence,
            }
            derive choices.agreement = Values::Bare;
            form mixed_relay_envelope = choices;
        }
        construction third_relay_envelope: MixedRelayEnvelopeRoot {
            element ThirdRelayEnvelope {
                choices: RelayedMixedChoiceSequence,
            }
            derive choices.agreement = Values::ThirdPersonSingular;
            form third_relay_envelope = choices;
        }
        construction bare_outer_mixed_relay_envelope: OuterMixedRelayEnvelopeRoot {
            element BareOuterMixedRelayEnvelope {
                choices: RelayedOuterMixedChoiceSequence,
            }
            derive choices.agreement = Values::Bare;
            form bare_outer_mixed_relay_envelope = choices;
        }
        construction third_outer_mixed_relay_envelope: OuterMixedRelayEnvelopeRoot {
            element ThirdOuterMixedRelayEnvelope {
                choices: RelayedOuterMixedChoiceSequence,
            }
            derive choices.agreement = Values::ThirdPersonSingular;
            form third_outer_mixed_relay_envelope = choices;
        }
        construction bare_direct_outer_mixed_relay_envelope: DirectOuterMixedRelayEnvelopeRoot {
            element BareDirectOuterMixedRelayEnvelope {
                choice: OuterRelayedMixedChoice,
            }
            derive choice.agreement = Values::Bare;
            form bare_direct_outer_mixed_relay_envelope = choice;
        }
        construction third_direct_outer_mixed_relay_envelope: DirectOuterMixedRelayEnvelopeRoot {
            element ThirdDirectOuterMixedRelayEnvelope {
                choice: OuterRelayedMixedChoice,
            }
            derive choice.agreement = Values::ThirdPersonSingular;
            form third_direct_outer_mixed_relay_envelope = choice;
        }
        construction direct_intrinsic_choice: DirectIntrinsicChoiceRoot {
            element DirectIntrinsicChoice {
                choice: AgreementChild,
            }
            form direct_intrinsic_choice = choice;
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
        abstract sum MixedAgreementChild { Child, Predicate, }
        abstract sum OuterRelayedMixedChoice { RelayedMixedChoiceSequence, }
        abstract product IntrinsicAgreementHolder {
            required: AgreementChild,
            optional: opt AgreementChild,
            members: seq AgreementChild separated by " ",
        }
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
        root HomogeneousNumberSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root RelayedNumberSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root NumberSequenceEnvelope { punctuation = "."; eoi = true; standalone_render = true; }
        root HomogeneousChoiceSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root RelayedChoiceSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root MixedChoiceSequence { punctuation = "."; eoi = true; standalone_render = true; }
        root RelayedMixedChoiceSequence { punctuation = "."; eoi = true; standalone_render = false; }
        root RelayedOuterMixedChoiceSequence { punctuation = "."; eoi = true; standalone_render = false; }
        root MixedRelayEnvelopeRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root OuterMixedRelayEnvelopeRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root DirectOuterMixedRelayEnvelopeRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root DirectIntrinsicChoiceRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root IntrinsicAgreementHolder { punctuation = "."; eoi = false; standalone_render = true; }
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

    #[allow(
        clippy::too_many_lines,
        reason = "this ABI assertion deliberately authenticates the complete generated owner table"
    )]
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
                    onset: deckmaste_construction_core::macro_def::Onset::Vowel,
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

        let kind = deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction;
        let declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                    kind, "Destroy",
                ),
                feature: deckmaste_construction_core::macro_def::SurfaceFeature::Bare,
                onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                id: deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                    kind, "Destroy",
                ),
                feature: deckmaste_construction_core::macro_def::SurfaceFeature::Bare,
                onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                id: deckmaste_construction_core::macro_def::DeclarationIdentity::new(
                    kind, "Destroy",
                ),
                feature: deckmaste_construction_core::macro_def::SurfaceFeature::Bare,
                onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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

        let kind = deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction;
        let position = deckmaste_construction_core::macro_def::GrammarPosition::Verb;

        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "Destroy",
                position,
                feature: FeatureConstraint::Exact(
                    deckmaste_construction_core::macro_def::SurfaceFeature::Bare,
                ),
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
                        onset: deckmaste_construction_core::macro_def::Onset::Vowel,
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
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
        let BuildValue::BeSentence(be_sentence, Agreement::Bare, _) = built else {
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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

    pub(super) fn assert_recursive_optional_renders_and_visits_through_generated_boxes() {
        #[derive(Default)]
        struct RecursiveVisitor(usize);

        impl Visitor for RecursiveVisitor {
            fn visit_recursive_leaf(&mut self, value: &RecursiveLeaf) {
                self.0 += 1;
                walk_recursive_leaf(self, value);
            }
        }

        let value = RecursiveNode::RecursiveOptional(RecursiveOptional {
            child: Box::new(Some(RecursiveNode::RecursiveLeaf(RecursiveLeaf))),
        });
        let mut writer = Writer::new();
        render_recursive_node(&mut writer, &value);
        assert_eq!(writer.finish(), "Node leaf");

        let mut visitor = RecursiveVisitor::default();
        visitor.visit_recursive_node(&value);
        assert_eq!(visitor.0, 1);
    }

    pub(super) fn assert_guarded_form_partition_boundaries() {
        let context = ParseContext {
            sentinel: 99,
            card_name: "card",
            abbreviated_card_name: "card",
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                    let BuildValue::PartitionRoot(root, _) =
                        built.expect("the rule accepts its exact finite partition")
                    else {
                        panic!("the selected rule builds its declared category")
                    };
                    assert_eq!(Render::render(&root, &context), expected[value_index]);
                } else {
                    assert!(
                        built.is_none(),
                        "rule {rule_index} accepted same-frame partition value {value_index}",
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
        assert!(matches!(that_rule, BuildValue::OptionalGuardRoot(..)));
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            BuildValue::StructuralAtom(
                StructuralAtom::StructuralAtom(StructuralAtomValue {
                    marker: StructuralWord::Alpha,
                }),
                FeatureConstraint::Any,
            ),
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            &[BuildValue::StructuralAtom(
                alpha.clone(),
                FeatureConstraint::Any,
            )],
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
                BuildValue::StructuralAtom(four[3].clone(), FeatureConstraint::Any),
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
                    BuildValue::StructuralAtom(item.clone(), FeatureConstraint::Any),
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                deckmaste_construction_core::macro_def::Onset::Vowel,
                RuleId::VerbHeadActOnset,
                RuleId::VerbArticleIndefiniteVerbAn,
                "An act.",
            ),
            (
                "Same",
                VerbLexeme::Other,
                "Other",
                deckmaste_construction_core::macro_def::Onset::Consonant,
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
            assert!(matches!(head, BuildValue::VerbHead(_, actual, _) if actual == onset));
            let literal = if onset == deckmaste_construction_core::macro_def::Onset::Vowel {
                "an"
            } else {
                "a"
            };
            let article = build(
                article_rule,
                &[BuildValue::Leaf(Leaf::Literal(literal)), head],
                &context,
            )
            .expect("the frozen verb onset selects the exact guarded article form");
            let BuildValue::VerbArticle(article, actual, _) = article else {
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
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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

        let singular_value = BuildValue::CircumfixValue(
            CircumfixValue::PlusTwo(PlusTwoValue),
            FeatureConstraint::Any,
        );
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
        let BuildValue::CircumfixSingularRoot(built_singular, _) = built else {
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
            &[BuildValue::CircumfixValue(
                value(CircumfixWord::Tap),
                FeatureConstraint::Any,
            )],
            &context,
        )
        .expect("the delegated sequence singleton builds its stored member");
        let sequence_middle = build(
            RuleId::BracedValuesValuesSequenceRecursive,
            &[
                BuildValue::CircumfixValue(value(CircumfixWord::WhiteBlue), FeatureConstraint::Any),
                BuildValue::Leaf(Leaf::Literal("}{")),
                sequence_tail.clone(),
            ],
            &context,
        )
        .expect("the delegated sequence helper builds through the second interior edge");
        let sequence_carrier = build(
            RuleId::BracedValuesValuesSequenceRecursive,
            &[
                BuildValue::CircumfixValue(
                    CircumfixValue::CircumfixTwo(CircumfixTwoValue),
                    FeatureConstraint::Any,
                ),
                BuildValue::Leaf(Leaf::Literal("}{")),
                sequence_middle.clone(),
            ],
            &context,
        )
        .expect("the delegated sequence helper builds through the first interior edge");
        for (member, tail) in [
            (
                BuildValue::CircumfixValue(value(CircumfixWord::WhiteBlue), FeatureConstraint::Any),
                sequence_tail,
            ),
            (
                BuildValue::CircumfixValue(
                    CircumfixValue::CircumfixTwo(CircumfixTwoValue),
                    FeatureConstraint::Any,
                ),
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
        let BuildValue::CircumfixSequenceRoot(built_sequence, _) = built else {
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            BuildValue::PrefixHead(
                _,
                deckmaste_construction_core::macro_def::Onset::Consonant,
                _
            )
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
        let BuildValue::PrefixArticle(article, actual_onset, _) = article else {
            panic!("the prefixed article builds its declared category")
        };
        assert_eq!(
            actual_onset,
            deckmaste_construction_core::macro_def::Onset::Consonant
        );
        assert_eq!(Render::render(&article, &context), "A one nonartifact.");

        let forest = parse_structural(Category::PrefixArticle, "A one nonartifact", &context);
        assert_eq!(forest.accepted_root_ids().count(), 1);

        for (value, article_rule, article_literal, expected_onset, expected_surface) in [
            (
                BoundWord::Elf,
                RuleId::PrefixArticlePrefixedArticleAn,
                "an",
                deckmaste_construction_core::macro_def::Onset::Vowel,
                "An many 2/Elf.",
            ),
            (
                BoundWord::Black,
                RuleId::PrefixArticlePrefixedArticleA,
                "a",
                deckmaste_construction_core::macro_def::Onset::Consonant,
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
                BuildValue::PrefixHead(_, actual, _) if actual == expected_onset
            ));
            let article = build(
                article_rule,
                &[BuildValue::Leaf(Leaf::Literal(article_literal)), head],
                &context,
            )
            .expect("the article guard consumes the form-local realized onset");
            let BuildValue::PrefixArticle(article, actual_onset, _) = article else {
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                BuildValue::PossessiveOwner(_, actual_number, actual_ending, _)
                    if actual_number == number && actual_ending == ending
            ));
            let possessive = build(
                possessive_rule,
                &[owner.clone(), BuildValue::Leaf(Leaf::Literal(suffix))],
                &context,
            )
            .expect("the exact Number/Ending partition selects one suffix form");
            let BuildValue::DerivedPossessiveRoot(possessive, _) = possessive else {
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
            FeatureConstraint::Any,
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
            FeatureConstraint::Any,
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
            (
                "artifact",
                deckmaste_construction_core::macro_def::Onset::Consonant,
                "A artifact.",
            ),
            (
                "card",
                deckmaste_construction_core::macro_def::Onset::Vowel,
                "An card.",
            ),
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
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
        };
        assert_generated_runtime_abi(&context);
        assert!(SelfRef::Full.valid_in(&context));
        assert!(!SelfRef::Abbreviated.valid_in(&context));
        assert_eq!(SelfRef::Full.surface(&context), "card");
        let abbreviated_context = ParseContext {
            sentinel: 99,
            card_name: "full card",
            abbreviated_card_name: "short",
            card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
            abbreviated_card_name_onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
            BuildValue::Source(Source::Source(SourceNode), Number::Singular, _)
        ));

        let one_children = |number| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(1),
                    number,
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(3),
                    number: right,
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
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

        let bare_child = BuildValue::Child(
            Child::Bare(BareChild),
            Agreement::Bare,
            FeatureConstraint::Any,
        );
        let third_child = BuildValue::Child(
            Child::Third(ThirdChild),
            Agreement::ThirdPersonSingular,
            FeatureConstraint::Any,
        );
        let parent = build(
            RuleId::ParentCategoryChain,
            std::slice::from_ref(&bare_child),
            &context,
        )
        .expect("a constant category writer flows into construction output");
        assert!(matches!(parent, BuildValue::Parent(_, Agreement::Bare, _)));
        assert!(
            build(
                RuleId::ParentCategoryChain,
                std::slice::from_ref(&third_child),
                &context,
            )
            .is_none()
        );

        let matching_child = BuildValue::Child(
            Child::Third(ThirdChild),
            Agreement::ThirdPersonSingular,
            FeatureConstraint::Any,
        );
        let mismatching_child = BuildValue::Child(
            Child::Third(ThirdChild),
            Agreement::Bare,
            FeatureConstraint::Any,
        );
        let refined_children = |child| vec![BuildValue::Leaf(Leaf::Mode(Mode::One)), child];
        let refined = build(
            RuleId::ParentRefined,
            &refined_children(matching_child),
            &context,
        )
        .expect("a refined writer flows through its category into construction output");
        assert!(matches!(
            refined,
            BuildValue::Parent(_, Agreement::ThirdPersonSingular, _)
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
                onset: deckmaste_construction_core::macro_def::Onset::Vowel,
            })],
            &context,
        )
        .expect("an implicit-verb constant flows into construction output");
        assert!(matches!(action, BuildValue::Action(_, Agreement::Bare, _)));
        assert!(
            build(
                RuleId::ActionAction,
                &[BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::ThirdPersonSingular,
                    onset: deckmaste_construction_core::macro_def::Onset::Vowel,
                })],
                &context,
            )
            .is_none()
        );

        let contextual = |agreement| {
            BuildValue::Predicate(
                Predicate::Contextual(ContextualPredicate),
                agreement,
                FeatureConstraint::Any,
            )
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
            BuildValue::ContextBound(
                ContextBound::ContextBound(ContextBoundNode {
                    pair: Pair {
                        context_value: 1,
                        child_slot: 2,
                        rule_code: 3,
                    },
                }),
                _
            )
        ));

        let feature_bound = build(
            RuleId::FeatureBoundAgreement,
            &[
                BuildValue::Leaf(Leaf::Marker(Marker::One)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: VerbLexeme::Act,
                    agreement: Agreement::ThirdPersonSingular,
                    onset: deckmaste_construction_core::macro_def::Onset::Vowel,
                }),
            ],
            &context,
        )
        .expect("a map local cannot shadow its carried agreement");
        assert!(matches!(
            feature_bound,
            BuildValue::FeatureBound(_, Agreement::ThirdPersonSingular, _)
        ));

        let collision = build(
            RuleId::CollisionCollision,
            &[
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(4),
                    number: Number::Singular,
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(5),
                    number: Number::Singular,
                    onset: deckmaste_construction_core::macro_def::Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                }),
                BuildValue::Leaf(Leaf::Mode(Mode::One)),
            ],
            &context,
        )
        .expect("adversarial preferred binders compile and build");
        assert!(matches!(
            collision,
            BuildValue::Collision(
                Collision::Collision(CollisionNode {
                    number: Head(4),
                    right_number: Head(5),
                    ..
                }),
                _
            )
        ));

        let keyword = build(
            RuleId::KeywordWhere,
            &[BuildValue::Leaf(Leaf::Marker(Marker::One))],
            &context,
        )
        .expect("a keyword-named construction builds with its carried feature");
        let BuildValue::Keyword(keyword, Agreement::Bare, _) = keyword else {
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
        let BuildValue::RawCategory(raw_category, _) = raw_category else {
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
                    FeatureConstraint::Any,
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
            Agreement::Bare => BuildValue::Child(bare(), Agreement::Bare, FeatureConstraint::Any),
            Agreement::ThirdPersonSingular => BuildValue::Child(
                third(),
                Agreement::ThirdPersonSingular,
                FeatureConstraint::Any,
            ),
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
        let BuildValue::HomogeneousSequence(sequence, _) = built else {
            panic!("the homogeneous sequence builds its declared category")
        };
        assert_eq!(Render::render(&sequence, &context), "Bare bare bare.");
    }

    pub(super) fn assert_sequence_number_is_homogeneous_across_every_boundary() {
        let singular = || NumberItem::SingularNumber(SingularNumberItem);
        let plural = || NumberItem::PluralNumber(PluralNumberItem);
        let artifact = || NumberItem::ArtifactNumber(ArtifactNumberItem);
        let enchantment = || NumberItem::EnchantmentNumber(EnchantmentNumberItem);
        let spell = || NumberItem::SpellNumber(SpellNumberItem);
        let ability = || NumberItem::AbilityNumber(AbilityNumberItem);

        for length in [2, 3, 4] {
            let members = (0..length).map(|_| singular()).collect::<Vec<_>>();
            let downward = HomogeneousSingularNumbers::try_new(members.clone())
                .expect("two through four singular members satisfy the downward Number writer");
            assert_eq!(downward.members(), members);
            let outward = RelayedNumbers::try_new(members)
                .expect("two through four members relay one homogeneous Number outward");
            assert_eq!(outward.members().len(), length);
        }

        let downward_rejection =
            HomogeneousSingularNumbers::try_new(vec![singular(), plural(), singular()])
                .expect_err("a mismatched middle member rejects the downward Number writer");
        assert_eq!(downward_rejection.owner(), "HomogeneousSingularNumbers");
        assert_eq!(downward_rejection.role(), "members");
        assert_eq!(
            downward_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "all members match derived grammatical Number",
            },
        );

        let relay_rejection = RelayedNumbers::try_new(vec![plural(), singular(), plural()])
            .expect_err("a mixed sequence cannot relay one homogeneous Number outward");
        assert_eq!(relay_rejection.owner(), "RelayedNumbers");
        assert_eq!(relay_rejection.role(), "members");
        assert_eq!(
            relay_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "all members share grammatical Number",
            },
        );

        let singular_relay = RelayedNumberSequence::RelayedNumbers(
            RelayedNumbers::try_new(vec![singular(), singular()])
                .expect("the relay is internally homogeneous at Singular"),
        );
        let envelope = SingularNumberSequenceEnvelope::try_new(singular_relay)
            .expect("the enclosing downward writer accepts the relayed Singular Number");
        assert_eq!(
            Render::render(
                &NumberSequenceEnvelope::SingularNumberSequenceEnvelope(envelope),
                &ParseContext::default(),
            ),
            "A one or one.",
        );

        for (members, expected) in [
            (
                vec![artifact(), enchantment()],
                "An artifact or enchantment.",
            ),
            (vec![spell(), ability()], "A spell or ability."),
        ] {
            let relayed = RelayedNumberSequence::RelayedNumbers(
                RelayedNumbers::try_new(members)
                    .expect("distinct sequence features relay independently"),
            );
            let envelope = SingularNumberSequenceEnvelope::try_new(relayed)
                .expect("the Singular Number gate accepts the coordinated members");
            assert_eq!(
                Render::render(
                    &NumberSequenceEnvelope::SingularNumberSequenceEnvelope(envelope),
                    &ParseContext::default(),
                ),
                expected,
            );
        }

        let plural_relay = RelayedNumberSequence::RelayedNumbers(
            RelayedNumbers::try_new(vec![plural(), plural()])
                .expect("the relay is internally homogeneous at Plural"),
        );
        let envelope_rejection = SingularNumberSequenceEnvelope::try_new(plural_relay)
            .expect_err("the enclosing Singular writer rejects a relayed Plural Number");
        assert_eq!(envelope_rejection.owner(), "SingularNumberSequenceEnvelope");
        assert_eq!(envelope_rejection.role(), "sequence");
        assert_eq!(
            envelope_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "value matches derived grammatical Number",
            },
        );

        let context = ParseContext::default();
        let accepted = parse_structural(Category::NumberSequenceEnvelope, "A one or one", &context);
        assert_eq!(accepted.accepted_root_ids().count(), 1);
        let rejected =
            parse_structural(Category::NumberSequenceEnvelope, "A many or many", &context);
        assert_eq!(
            rejected.accepted_root_ids().count(),
            1,
            "the scanner preserves the Plural reading before checked materialization rejects it",
        );
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
        let child = |value, agreement| BuildValue::Child(value, agreement, FeatureConstraint::Any);
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
        let BuildValue::SingletonSequence(sequence, _) = built else {
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

        let choice = |value, agreement| {
            BuildValue::AgreementChild(
                AgreementChild::Child(value),
                agreement,
                FeatureConstraint::Any,
            )
        };
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

    pub(super) fn assert_mixed_sum_sequence_checks_intrinsic_alternatives() {
        let bare = MixedAgreementChild::Child(Child::Bare(BareChild));
        let third = MixedAgreementChild::Child(Child::Third(ThirdChild));
        let contextual = MixedAgreementChild::Predicate(Predicate::Contextual(ContextualPredicate));

        let rejection = UniformMixedChildChoices::try_new(vec![third.clone(), contextual.clone()])
            .expect_err("an intrinsic Third alternative cannot satisfy derived Bare agreement");
        assert_eq!(rejection.owner(), "UniformMixedChildChoices");
        assert_eq!(rejection.role(), "members");
        assert_eq!(
            rejection.violation(),
            &BuildViolation::Invariant {
                identity: "all members match derived agreement",
            },
        );

        let accepted = UniformMixedChildChoices::try_new(vec![bare.clone(), contextual.clone()])
            .expect("an intrinsic Bare and contextual member satisfy derived Bare agreement");
        let root = MixedChoiceSequence::UniformMixedChildChoices(accepted);
        assert_eq!(Render::render(&root, &ParseContext::default()), "Bare act.");

        let context = ParseContext::default();
        let parsed = parse_structural(Category::MixedChoiceSequence, "Bare act", &context);
        assert_eq!(parsed.accepted_root_ids().count(), 1);
        let parsed_mismatch =
            parse_structural(Category::MixedChoiceSequence, "Third act", &context);
        assert_eq!(
            parsed_mismatch.accepted_root_ids().count(),
            1,
            "the chart preserves both lexical readings before generated materialization rejects their agreement mismatch",
        );

        let mixed = |value, agreement| {
            BuildValue::MixedAgreementChild(value, agreement, FeatureConstraint::Any)
        };
        let pair = build(
            RuleId::UniformMixedChildChoicesMembersSequenceLength2,
            &[
                mixed(bare.clone(), Agreement::Bare),
                BuildValue::Leaf(Leaf::Literal(" ")),
                mixed(contextual.clone(), Agreement::Bare),
            ],
            &context,
        )
        .expect("the mixed sum pair retains one homogeneous transient agreement");
        assert!(matches!(
            pair,
            BuildValue::UniformMixedChildChoicesMembersSequence(_, Agreement::Bare)
        ));
        assert!(
            build(
                RuleId::UniformMixedChildChoicesMembersSequenceLength2,
                &[
                    mixed(third.clone(), Agreement::ThirdPersonSingular),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    mixed(contextual.clone(), Agreement::Bare),
                ],
                &context,
            )
            .is_none(),
            "the generated exact build rejects a contextual carrier that mismatches the intrinsic alternative",
        );

        let fabricated = BuildValue::UniformMixedChildChoicesMembersSequence(
            vec![third, contextual],
            Agreement::Bare,
        );
        let materialization_rejection = build_checked(
            RuleId::MixedChoiceSequenceUniformMixedChildChoices,
            &[fabricated],
            &context,
        )
        .expect_err(
            "owner materialization rechecks the intrinsic member against its carried agreement",
        );
        assert_eq!(
            materialization_rejection.owner(),
            "UniformMixedChildChoices"
        );
        assert_eq!(materialization_rejection.role(), "members");
    }

    pub(super) fn assert_mixed_sum_sequence_relay_checks_intrinsic_constraints() {
        assert_mixed_sum_sequence_relay_constructor_checks();
        assert_mixed_sum_sequence_relay_build_and_parse_checks();
    }

    fn assert_mixed_sum_sequence_relay_constructor_checks() {
        let bare = || MixedAgreementChild::Child(Child::Bare(BareChild));
        let third = || MixedAgreementChild::Child(Child::Third(ThirdChild));
        let contextual =
            || MixedAgreementChild::Predicate(Predicate::Contextual(ContextualPredicate));

        RelayedMixedChildChoices::try_new(vec![third(), contextual()])
            .expect("a contextual alternative can realize the intrinsic Third agreement");
        RelayedMixedChildChoices::try_new(vec![contextual(), bare()])
            .expect("a contextual alternative can realize the intrinsic Bare agreement");
        RelayedMixedChildChoices::try_new(vec![contextual(), contextual()])
            .expect("an all-contextual sequence accepts one homogeneous external agreement");

        let rejection = RelayedMixedChildChoices::try_new(vec![third(), contextual(), bare()])
            .expect_err("conflicting intrinsic alternatives have no homogeneous agreement");
        assert_eq!(rejection.owner(), "RelayedMixedChildChoices");
        assert_eq!(rejection.role(), "members");
        assert_eq!(
            rejection.violation(),
            &BuildViolation::Invariant {
                identity: "all members share agreement",
            },
        );

        let relayed_third = RelayedMixedChoiceSequence::RelayedMixedChildChoices(
            RelayedMixedChildChoices::try_new(vec![third(), contextual()])
                .expect("the relay is internally homogeneous at Third"),
        );
        let envelope_rejection = MixedRelayEnvelope::try_new(relayed_third)
            .expect_err("a Bare writer rejects a relay constrained to intrinsic Third");
        assert_eq!(envelope_rejection.owner(), "MixedRelayEnvelope");
        assert_eq!(envelope_rejection.role(), "choices");
        assert_eq!(
            envelope_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "value matches derived agreement",
            },
        );

        let intrinsic_third =
            RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice);
        let intrinsic_rejection = MixedRelayEnvelope::try_new(intrinsic_third)
            .expect_err("a Bare writer rejects the category's intrinsic Third variant");
        assert_eq!(intrinsic_rejection.owner(), "MixedRelayEnvelope");
        assert_eq!(intrinsic_rejection.role(), "choices");
        let intrinsic_third =
            RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice);
        let third_envelope = ThirdRelayEnvelope::try_new(intrinsic_third)
            .expect("a Third writer accepts the category's intrinsic Third variant");
        assert_eq!(
            Render::render(
                &MixedRelayEnvelopeRoot::ThirdRelayEnvelope(third_envelope),
                &ParseContext::default(),
            ),
            "Intrinsic third.",
        );

        let checked_bare = CheckedBareMixedChoice::try_new(Mode::One)
            .expect("the unrelated checked variant accepts its legal value");
        let checked_category = RelayedMixedChoiceSequence::CheckedBareMixedChoice(checked_bare);
        let checked_envelope = MixedRelayEnvelope::try_new(checked_category)
            .expect("the Bare writer accepts the checked variant's intrinsic Bare agreement");
        assert_eq!(
            Render::render(
                &MixedRelayEnvelopeRoot::MixedRelayEnvelope(checked_envelope),
                &ParseContext::default(),
            ),
            "One.",
        );
        assert!(CheckedBareMixedChoice::try_new(Mode::Many).is_err());

        let relayed_bare = RelayedMixedChoiceSequence::RelayedMixedChildChoices(
            RelayedMixedChildChoices::try_new(vec![bare(), contextual()])
                .expect("the relay is internally homogeneous at Bare"),
        );
        let envelope = MixedRelayEnvelope::try_new(relayed_bare)
            .expect("a Bare writer accepts a relay constrained to intrinsic Bare");
        let root = MixedRelayEnvelopeRoot::MixedRelayEnvelope(envelope);
        assert_eq!(Render::render(&root, &ParseContext::default()), "Bare act.");
    }

    fn assert_mixed_sum_sequence_relay_build_and_parse_checks() {
        let third = || MixedAgreementChild::Child(Child::Third(ThirdChild));
        let contextual =
            || MixedAgreementChild::Predicate(Predicate::Contextual(ContextualPredicate));
        let context = ParseContext::default();
        let built_intrinsic = build_checked(
            RuleId::RelayedMixedChoiceSequenceIntrinsicThirdMixedChoice,
            &[BuildValue::Leaf(Leaf::Literal("intrinsic third"))],
            &context,
        )
        .expect("the intrinsic sibling has no checked-constructor emission failure")
        .expect("the intrinsic sibling materializes");
        assert!(matches!(
            built_intrinsic,
            BuildValue::RelayedMixedChoiceSequence(
                RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(_),
                Agreement::ThirdPersonSingular,
                _
            )
        ));
        assert!(
            build(
                RuleId::MixedRelayEnvelopeRootMixedRelayEnvelope,
                std::slice::from_ref(&built_intrinsic),
                &context,
            )
            .is_none(),
            "the generated Bare writer rejects the intrinsic Third carrier",
        );
        let built_third_envelope = build(
            RuleId::MixedRelayEnvelopeRootThirdRelayEnvelope,
            &[built_intrinsic],
            &context,
        )
        .expect("the generated Third writer accepts the intrinsic Third carrier");
        assert!(matches!(
            built_third_envelope,
            BuildValue::MixedRelayEnvelopeRoot(MixedRelayEnvelopeRoot::ThirdRelayEnvelope(_), _)
        ));

        let mixed = |value, agreement| {
            BuildValue::MixedAgreementChild(value, agreement, FeatureConstraint::Any)
        };
        let pair = build(
            RuleId::RelayedMixedChildChoicesMembersSequenceLength2,
            &[
                mixed(third(), Agreement::ThirdPersonSingular),
                BuildValue::Leaf(Leaf::Literal(" ")),
                mixed(contextual(), Agreement::ThirdPersonSingular),
            ],
            &context,
        )
        .expect("relay build preserves the homogeneous Third carrier");
        assert!(matches!(
            &pair,
            BuildValue::RelayedMixedChildChoicesMembersSequence(_, Agreement::ThirdPersonSingular)
        ));
        let built_owner = build_checked(
            RuleId::RelayedMixedChoiceSequenceRelayedMixedChildChoices,
            &[pair],
            &context,
        )
        .expect("relay owner preserves the intrinsic Third constraint")
        .expect("the exact owner rule builds");
        assert!(matches!(
            built_owner,
            BuildValue::RelayedMixedChoiceSequence(
                RelayedMixedChoiceSequence::RelayedMixedChildChoices(_),
                Agreement::ThirdPersonSingular,
                _
            )
        ));
        assert!(
            build(
                RuleId::RelayedMixedChildChoicesMembersSequenceLength2,
                &[
                    mixed(third(), Agreement::ThirdPersonSingular),
                    BuildValue::Leaf(Leaf::Literal(" ")),
                    mixed(contextual(), Agreement::Bare),
                ],
                &context,
            )
            .is_none(),
            "relay exact build rejects a mismatched contextual carrier",
        );

        let fabricated = BuildValue::RelayedMixedChildChoicesMembersSequence(
            vec![third(), contextual()],
            Agreement::Bare,
        );
        let materialization_rejection = build_checked(
            RuleId::RelayedMixedChoiceSequenceRelayedMixedChildChoices,
            &[fabricated],
            &context,
        )
        .expect_err("relay materialization checks the carried agreement against intrinsic members");
        assert_eq!(
            materialization_rejection.owner(),
            "RelayedMixedChildChoices"
        );
        assert_eq!(materialization_rejection.role(), "members");

        let parsed = parse_structural(Category::RelayedMixedChoiceSequence, "Third acts", &context);
        assert_eq!(parsed.accepted_root_ids().count(), 1);
        let parsed_intrinsic = parse_structural(
            Category::RelayedMixedChoiceSequence,
            "Intrinsic third",
            &context,
        );
        assert_eq!(parsed_intrinsic.accepted_root_ids().count(), 1);
        let parsed_mismatch =
            parse_structural(Category::RelayedMixedChoiceSequence, "Third act", &context);
        assert_eq!(
            parsed_mismatch.accepted_root_ids().count(),
            1,
            "the scanner preserves the conflicting lexical carrier before generated build rejects it",
        );

        let built_checked = build_checked(
            RuleId::RelayedMixedChoiceSequenceCheckedBareMixedChoice,
            &[BuildValue::Leaf(Leaf::Mode(Mode::One))],
            &context,
        )
        .expect("the checked sibling emits without demanding a relay FromRole source")
        .expect("the legal checked sibling materializes");
        assert!(matches!(
            built_checked,
            BuildValue::RelayedMixedChoiceSequence(
                RelayedMixedChoiceSequence::CheckedBareMixedChoice(_),
                Agreement::Bare,
                _
            )
        ));
        let checked_rejection = build_checked(
            RuleId::RelayedMixedChoiceSequenceCheckedBareMixedChoice,
            &[BuildValue::Leaf(Leaf::Mode(Mode::Many))],
            &context,
        )
        .expect_err("the checked sibling retains its unrelated invariant");
        assert_eq!(checked_rejection.owner(), "RelayedMixedChoiceSequence");

        let parsed_checked =
            parse_structural(Category::RelayedMixedChoiceSequence, "One", &context);
        assert_eq!(parsed_checked.accepted_root_ids().count(), 1);
        let scanned_illegal =
            parse_structural(Category::RelayedMixedChoiceSequence, "Many", &context);
        assert_eq!(
            scanned_illegal.accepted_root_ids().count(),
            1,
            "the scanner preserves the lexical checked variant before its invariant rejects materialization",
        );
    }

    pub(super) fn assert_outer_sum_preserves_selected_category_agreement_authority() {
        assert_outer_sum_selected_category_constructor_checks();
        assert_outer_sum_selected_category_build_checks();
    }

    fn assert_outer_sum_selected_category_constructor_checks() {
        let outer = |value| OuterRelayedMixedChoice::RelayedMixedChoiceSequence(value);
        let intrinsic_third =
            || RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice);
        let bare = || MixedAgreementChild::Child(Child::Bare(BareChild));
        let contextual =
            || MixedAgreementChild::Predicate(Predicate::Contextual(ContextualPredicate));

        let relayed_intrinsic = RelayedOuterMixedChoices::try_new(vec![outer(intrinsic_third())])
            .expect("the outer relay accepts a homogeneous intrinsic Third singleton");
        let relayed_intrinsic =
            RelayedOuterMixedChoiceSequence::RelayedOuterMixedChoices(relayed_intrinsic);
        let bare_rejection = BareOuterMixedRelayEnvelope::try_new(relayed_intrinsic.clone())
            .expect_err("a Bare writer rejects the outer sum's selected intrinsic Third sibling");
        assert_eq!(bare_rejection.owner(), "BareOuterMixedRelayEnvelope");
        assert_eq!(bare_rejection.role(), "choices");
        assert_eq!(
            bare_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "value matches derived agreement",
            },
        );

        let third_envelope = ThirdOuterMixedRelayEnvelope::try_new(relayed_intrinsic)
            .expect("a Third writer accepts the outer sum's selected intrinsic Third sibling");
        assert_eq!(
            Render::render(
                &OuterMixedRelayEnvelopeRoot::ThirdOuterMixedRelayEnvelope(third_envelope),
                &ParseContext::default(),
            ),
            "Intrinsic third.",
        );

        let compatible_relay = RelayedMixedChoiceSequence::RelayedMixedChildChoices(
            RelayedMixedChildChoices::try_new(vec![bare(), contextual()])
                .expect("the inner relay is homogeneous at Bare"),
        );
        let relayed_compatible = RelayedOuterMixedChoices::try_new(vec![outer(compatible_relay)])
            .expect("the outer relay accepts the compatible contextual path");
        let relayed_compatible =
            RelayedOuterMixedChoiceSequence::RelayedOuterMixedChoices(relayed_compatible);
        let compatible_envelope = BareOuterMixedRelayEnvelope::try_new(relayed_compatible)
            .expect("a Bare writer accepts the compatible relay through the outer sum");
        assert_eq!(
            Render::render(
                &OuterMixedRelayEnvelopeRoot::BareOuterMixedRelayEnvelope(compatible_envelope),
                &ParseContext::default(),
            ),
            "Bare act.",
        );

        let checked = CheckedBareMixedChoice::try_new(Mode::One)
            .expect("the unrelated checked sibling accepts its legal value");
        let relayed_checked = RelayedOuterMixedChoices::try_new(vec![outer(
            RelayedMixedChoiceSequence::CheckedBareMixedChoice(checked),
        )])
        .expect("the unrelated checked sibling crosses the outer sum without a relay source");
        let relayed_checked =
            RelayedOuterMixedChoiceSequence::RelayedOuterMixedChoices(relayed_checked);
        let checked_envelope = BareOuterMixedRelayEnvelope::try_new(relayed_checked)
            .expect("a Bare writer accepts the unrelated checked sibling");
        assert_eq!(
            Render::render(
                &OuterMixedRelayEnvelopeRoot::BareOuterMixedRelayEnvelope(checked_envelope),
                &ParseContext::default(),
            ),
            "One.",
        );
    }

    fn assert_outer_sum_selected_category_build_checks() {
        let outer = |value| OuterRelayedMixedChoice::RelayedMixedChoiceSequence(value);
        let intrinsic_third =
            || RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice);
        let context = ParseContext::default();
        let built_intrinsic = build_checked(
            RuleId::RelayedMixedChoiceSequenceIntrinsicThirdMixedChoice,
            &[BuildValue::Leaf(Leaf::Literal("intrinsic third"))],
            &context,
        )
        .expect("the inner intrinsic construction builds exactly")
        .expect("the inner intrinsic construction materializes");
        let built_outer = build(
            RuleId::OuterRelayedMixedChoiceRelayedMixedChoiceSequence,
            &[built_intrinsic],
            &context,
        )
        .expect("the explicit sum preserves the inner Third carrier");
        assert!(matches!(
            &built_outer,
            BuildValue::OuterRelayedMixedChoice(
                OuterRelayedMixedChoice::RelayedMixedChoiceSequence(
                    RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(_)
                ),
                Agreement::ThirdPersonSingular,
                _,
            )
        ));
        let built_sequence = build(
            RuleId::RelayedOuterMixedChoicesMembersSequenceSingleton,
            &[built_outer],
            &context,
        )
        .expect("the exact singleton sequence preserves the outer sum's Third carrier");
        let built_relay = build_checked(
            RuleId::RelayedOuterMixedChoiceSequenceRelayedOuterMixedChoices,
            &[built_sequence],
            &context,
        )
        .expect("the outer relay materializes without internal failure")
        .expect("the outer relay construction builds");
        assert!(matches!(
            &built_relay,
            BuildValue::RelayedOuterMixedChoiceSequence(
                RelayedOuterMixedChoiceSequence::RelayedOuterMixedChoices(_),
                Agreement::ThirdPersonSingular,
                _,
            )
        ));
        assert!(
            build(
                RuleId::OuterMixedRelayEnvelopeRootBareOuterMixedRelayEnvelope,
                std::slice::from_ref(&built_relay),
                &context,
            )
            .is_none(),
            "the exact Bare build rejects the relayed Third carrier",
        );
        let built_third_envelope = build(
            RuleId::OuterMixedRelayEnvelopeRootThirdOuterMixedRelayEnvelope,
            &[built_relay],
            &context,
        )
        .expect("the exact Third build accepts the relayed Third carrier");
        assert!(matches!(
            built_third_envelope,
            BuildValue::OuterMixedRelayEnvelopeRoot(
                OuterMixedRelayEnvelopeRoot::ThirdOuterMixedRelayEnvelope(_),
                _
            )
        ));

        let fabricated = BuildValue::RelayedOuterMixedChoicesMembersSequence(
            vec![outer(intrinsic_third())],
            Agreement::Bare,
        );
        let materialization_rejection = build_checked(
            RuleId::RelayedOuterMixedChoiceSequenceRelayedOuterMixedChoices,
            &[fabricated],
            &context,
        )
        .expect_err("owner materialization rechecks the outer sum's selected intrinsic authority");
        assert_eq!(
            materialization_rejection.owner(),
            "RelayedOuterMixedChoices"
        );
        assert_eq!(materialization_rejection.role(), "members");

        let parsed = parse_structural(
            Category::RelayedOuterMixedChoiceSequence,
            "Intrinsic third",
            &context,
        );
        assert_eq!(
            parsed.accepted_root_ids().count(),
            1,
            "the scanner/chart preserves the inner intrinsic path across the outer sum",
        );
    }

    pub(super) fn assert_direct_sum_role_invokes_recursive_selected_agreement_authority() {
        let outer = |value| OuterRelayedMixedChoice::RelayedMixedChoiceSequence(value);
        let intrinsic_third =
            || RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice);
        let bare = || MixedAgreementChild::Child(Child::Bare(BareChild));
        let contextual =
            || MixedAgreementChild::Predicate(Predicate::Contextual(ContextualPredicate));

        let bare_rejection = BareDirectOuterMixedRelayEnvelope::try_new(outer(intrinsic_third()))
            .expect_err("a Bare direct writer rejects the nested intrinsic Third construction");
        assert_eq!(bare_rejection.owner(), "BareDirectOuterMixedRelayEnvelope");
        assert_eq!(bare_rejection.role(), "choice");
        assert_eq!(
            bare_rejection.violation(),
            &BuildViolation::Invariant {
                identity: "value matches derived agreement",
            },
        );

        let third = ThirdDirectOuterMixedRelayEnvelope::try_new(outer(intrinsic_third()))
            .expect("a Third direct writer accepts the nested intrinsic Third construction");
        assert_eq!(
            Render::render(
                &DirectOuterMixedRelayEnvelopeRoot::ThirdDirectOuterMixedRelayEnvelope(third),
                &ParseContext::default(),
            ),
            "Intrinsic third.",
        );

        let compatible_relay = RelayedMixedChoiceSequence::RelayedMixedChildChoices(
            RelayedMixedChildChoices::try_new(vec![bare(), contextual()])
                .expect("the inner contextual relay is compatible at Bare"),
        );
        let compatible = BareDirectOuterMixedRelayEnvelope::try_new(outer(compatible_relay))
            .expect("a Bare direct writer accepts the compatible contextual relay");
        assert_eq!(
            Render::render(
                &DirectOuterMixedRelayEnvelopeRoot::BareDirectOuterMixedRelayEnvelope(compatible),
                &ParseContext::default(),
            ),
            "Bare act.",
        );

        let checked = CheckedBareMixedChoice::try_new(Mode::One)
            .expect("the unrelated checked sibling accepts its legal value");
        let checked = BareDirectOuterMixedRelayEnvelope::try_new(outer(
            RelayedMixedChoiceSequence::CheckedBareMixedChoice(checked),
        ))
        .expect("the Bare direct writer accepts the unrelated checked sibling");
        assert_eq!(
            Render::render(
                &DirectOuterMixedRelayEnvelopeRoot::BareDirectOuterMixedRelayEnvelope(checked),
                &ParseContext::default(),
            ),
            "One.",
        );

        let context = ParseContext::default();
        let built_intrinsic = build_checked(
            RuleId::RelayedMixedChoiceSequenceIntrinsicThirdMixedChoice,
            &[BuildValue::Leaf(Leaf::Literal("intrinsic third"))],
            &context,
        )
        .expect("the nested intrinsic construction builds exactly")
        .expect("the nested intrinsic construction materializes");
        let built_outer = build(
            RuleId::OuterRelayedMixedChoiceRelayedMixedChoiceSequence,
            &[built_intrinsic],
            &context,
        )
        .expect("the outer explicit sum preserves the nested Third carrier");
        assert!(
            build(
                RuleId::DirectOuterMixedRelayEnvelopeRootBareDirectOuterMixedRelayEnvelope,
                std::slice::from_ref(&built_outer),
                &context,
            )
            .is_none(),
            "the exact Bare build rejects the direct sum's Third carrier",
        );
        let built_third = build_checked(
            RuleId::DirectOuterMixedRelayEnvelopeRootThirdDirectOuterMixedRelayEnvelope,
            &[built_outer],
            &context,
        )
        .expect("the exact Third build has no checked-constructor failure")
        .expect("the exact Third build materializes");
        assert!(matches!(
            built_third,
            BuildValue::DirectOuterMixedRelayEnvelopeRoot(
                DirectOuterMixedRelayEnvelopeRoot::ThirdDirectOuterMixedRelayEnvelope(_),
                _
            )
        ));

        let fabricated = BuildValue::OuterRelayedMixedChoice(
            outer(intrinsic_third()),
            Agreement::Bare,
            FeatureConstraint::Any,
        );
        let materialization_rejection = build_checked(
            RuleId::DirectOuterMixedRelayEnvelopeRootBareDirectOuterMixedRelayEnvelope,
            &[fabricated],
            &context,
        )
        .expect_err("direct-writer materialization rechecks the selected nested authority");
        assert_eq!(
            materialization_rejection.owner(),
            "BareDirectOuterMixedRelayEnvelope"
        );
        assert_eq!(materialization_rejection.role(), "choice");

        let parsed = parse_structural(
            Category::DirectOuterMixedRelayEnvelopeRoot,
            "Intrinsic third",
            &context,
        );
        assert_eq!(
            parsed.accepted_root_ids().count(),
            2,
            "the scanner/chart preserves both direct writers before Agreement materialization",
        );
    }

    pub(super) fn assert_agreement_constrained_role_uses_checked_public_boundary() {
        let intrinsic_third = OuterRelayedMixedChoice::RelayedMixedChoiceSequence(
            RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(IntrinsicThirdMixedChoice),
        );
        assert!(BareDirectOuterMixedRelayEnvelope::new(intrinsic_third.clone()).is_none());
        let rejection = BareDirectOuterMixedRelayEnvelope::try_new(intrinsic_third.clone())
            .expect_err("the checked constructor rejects a mismatched intrinsic Agreement");
        assert_eq!(rejection.owner(), "BareDirectOuterMixedRelayEnvelope");
        assert_eq!(rejection.role(), "choice");

        let accepted = ThirdDirectOuterMixedRelayEnvelope::new(intrinsic_third.clone())
            .expect("the checked constructor accepts the matching intrinsic Agreement");
        assert_eq!(accepted.choice(), &intrinsic_third);
        assert_eq!(
            Render::render(
                &DirectOuterMixedRelayEnvelopeRoot::ThirdDirectOuterMixedRelayEnvelope(accepted),
                &ParseContext::default(),
            ),
            "Intrinsic third.",
        );
    }

    pub(super) fn assert_intrinsic_sum_product_fields_render_every_shape() {
        let choice = |child| AgreementChild::Child(child);
        let holder = IntrinsicAgreementHolder {
            required: choice(Child::Bare(BareChild)),
            optional: Some(choice(Child::Third(ThirdChild))),
            members: vec![
                choice(Child::Bare(BareChild)),
                choice(Child::Third(ThirdChild)),
            ],
        };
        assert_eq!(
            Render::render(&holder, &ParseContext::default()),
            "Bare third bare third.",
        );
    }

    pub(super) fn assert_direct_intrinsic_sum_render_derives_selected_agreement() {
        let choice = AgreementChild::Child(Child::Third(ThirdChild));
        let root = DirectIntrinsicChoiceRoot::DirectIntrinsicChoice(DirectIntrinsicChoice {
            choice: choice.clone(),
        });
        assert_eq!(Render::render(&root, &ParseContext::default()), "Third.",);

        let context = ParseContext::default();
        let built = build(
            RuleId::DirectIntrinsicChoiceRootDirectIntrinsicChoice,
            &[BuildValue::AgreementChild(
                choice,
                Agreement::ThirdPersonSingular,
                FeatureConstraint::Any,
            )],
            &context,
        )
        .expect("the exact direct-sum construction builds");
        let BuildValue::DirectIntrinsicChoiceRoot(built, _) = built else {
            panic!("the direct-sum rule builds its declared category")
        };
        assert_eq!(Render::render(&built, &context), "Third.");

        let parsed = parse_structural(Category::DirectIntrinsicChoiceRoot, "Third", &context);
        assert_eq!(parsed.accepted_root_ids().count(), 1);
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
            BuildValue::NonZeroQuantity(
                NonZeroQuantity::Positive(PositiveQuantityValue {
                    number: number.clone(),
                }),
                FeatureConstraint::Any
            )
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
fn declaration_verbs_preserve_identity_through_every_generated_boundary() {
    declaration_verb_fixture::run();
}

#[test]
fn declaration_participles_preserve_identity_through_every_generated_boundary() {
    declaration_verb_fixture::run_participle();
}

#[test]
fn declaration_verb_constructors_fail_closed() {
    declaration_verb_fixture::run_checked_constructors();
}

#[test]
fn declaration_verb_custom_shapes_match_only_exact_authored_tails() {
    declaration_verb_fixture::run_custom_shapes();
}

#[test]
fn declaration_verb_input_uses_a_literal_identity_frame_table() {
    declaration_verb_fixture::run_literal_input_frame_table();
}

#[test]
fn declaration_verb_open_only_codec_crosses_every_runtime_boundary() {
    declaration_verb_fixture::run_open_only_runtime_boundaries();
}

#[test]
fn declaration_verb_frame_set_perturbation_moves_frame_availability() {
    declaration_verb_fixture::run_frame_set_perturbation();
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
fn sequence_number_is_homogeneous_across_checked_build_render_scan_and_materialization() {
    fixture::assert_sequence_number_is_homogeneous_across_every_boundary();
}

#[test]
fn singleton_sequence_agreement_crosses_checked_build_render_scan_and_materialization() {
    fixture::assert_singleton_sequence_agreement_crosses_every_runtime_boundary();
}

#[test]
fn sum_sequence_agreement_uses_the_explicit_carrier_across_every_member_and_boundary() {
    fixture::assert_sum_sequence_agreement_uses_the_explicit_sum_carrier();
}

#[test]
fn mixed_sum_sequence_checks_intrinsic_alternatives_against_derived_agreement() {
    fixture::assert_mixed_sum_sequence_checks_intrinsic_alternatives();
}

#[test]
fn mixed_sum_sequence_relay_checks_every_intrinsic_constraint() {
    fixture::assert_mixed_sum_sequence_relay_checks_intrinsic_constraints();
}

#[test]
fn outer_sum_preserves_selected_category_agreement_authority() {
    fixture::assert_outer_sum_preserves_selected_category_agreement_authority();
}

#[test]
fn direct_sum_role_invokes_recursive_selected_agreement_authority() {
    fixture::assert_direct_sum_role_invokes_recursive_selected_agreement_authority();
}

#[test]
fn recursive_optional_fields_render_and_visit_through_generated_boxes() {
    fixture::assert_recursive_optional_renders_and_visits_through_generated_boxes();
}

#[test]
fn agreement_constrained_role_uses_checked_public_boundary() {
    fixture::assert_agreement_constrained_role_uses_checked_public_boundary();
}

#[test]
fn intrinsic_sum_product_fields_render_every_shape() {
    fixture::assert_intrinsic_sum_product_fields_render_every_shape();
}

#[test]
fn direct_intrinsic_sum_render_derives_selected_agreement_without_writer() {
    fixture::assert_direct_intrinsic_sum_render_derives_selected_agreement();
}
