#![allow(
    dead_code,
    reason = "the minimal consumer supplies and executes every generated catalog-identity boundary"
)]

mod catalog_fixture {
    use std::sync::Arc;

    use deckmaste_construction::constructions;

    use RulePosition::Lexical as L;

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

        fn identity(&mut self, identity: &str) {
            if self.prefix == PrefixPosition::WordOwnedSpace {
                self.output.push(' ');
            }
            self.output.push_str(identity);
            self.case = CasePosition::Continuation;
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

    pub mod environment {
        use std::sync::Arc;

        use macro_ron::v2::Onset;

        use super::CatalogProvider;

        #[derive(Debug, Clone)]
        pub(crate) struct CatalogRow {
            provider: CatalogProvider,
            canonical_identity: Arc<str>,
            canonical_surface: Arc<str>,
            onset: Onset,
        }

        impl CatalogRow {
            pub(crate) fn new(
                provider: CatalogProvider,
                canonical_identity: impl Into<Arc<str>>,
                canonical_surface: impl Into<Arc<str>>,
                onset: Onset,
            ) -> Self {
                Self {
                    provider,
                    canonical_identity: canonical_identity.into(),
                    canonical_surface: canonical_surface.into(),
                    onset,
                }
            }
        }

        pub(crate) struct ParserEnvironment {
            rows: Vec<CatalogRow>,
        }

        impl ParserEnvironment {
            pub(crate) fn new(rows: impl IntoIterator<Item = CatalogRow>) -> Self {
                Self {
                    rows: rows.into_iter().collect(),
                }
            }

            pub(crate) fn catalog_identity(
                &self,
                provider: CatalogProvider,
                canonical_identity: &str,
            ) -> Option<Arc<str>> {
                self.rows
                    .iter()
                    .find(|row| {
                        row.provider == provider
                            && row.canonical_identity.as_ref() == canonical_identity
                    })
                    .map(|row| row.canonical_identity.clone())
            }

            pub(crate) fn catalog_surface(
                &self,
                provider: CatalogProvider,
                canonical_identity: &str,
            ) -> Option<&str> {
                self.rows
                    .iter()
                    .find(|row| {
                        row.provider == provider
                            && row.canonical_identity.as_ref() == canonical_identity
                    })
                    .map(|row| row.canonical_surface.as_ref())
            }

            pub(crate) fn catalog_onset(
                &self,
                provider: CatalogProvider,
                canonical_identity: &str,
            ) -> Option<Onset> {
                self.rows
                    .iter()
                    .find(|row| {
                        row.provider == provider
                            && row.canonical_identity.as_ref() == canonical_identity
                    })
                    .map(|row| row.onset)
            }

            pub(crate) fn reading(
                &self,
                provider: CatalogProvider,
                canonical_surface: &str,
            ) -> Option<(Arc<str>, Onset)> {
                self.rows
                    .iter()
                    .find(|row| {
                        row.provider == provider
                            && row.canonical_surface.as_ref() == canonical_surface
                    })
                    .map(|row| (row.canonical_identity.clone(), row.onset))
            }
        }
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
            let boundary = matches!(
                right_boundary,
                LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
            ) || matches!(
                self.text.as_bytes().get(end),
                None | Some(b' ' | b',' | b'.')
            );
            (remainder.starts_with(&rendered) && boundary).then_some(end)
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

        fn catalog_identity_reading(
            &self,
            provider: CatalogProvider,
            right_boundary: LexicalBoundary,
        ) -> Option<(usize, Arc<str>, macro_ron::v2::Onset)> {
            let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
            let remainder = self.text.get(self.position.byte_offset..)?;
            let surface_text = (prefix == 0)
                .then_some(remainder)
                .or_else(|| remainder.strip_prefix(' '))?;
            let mut longest = None;
            for relative_end in surface_text
                .char_indices()
                .skip(1)
                .map(|(end, _)| end)
                .chain(std::iter::once(surface_text.len()))
            {
                let end = self.position.byte_offset + prefix + relative_end;
                if !matches!(
                    right_boundary,
                    LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
                ) && !matches!(
                    self.text.as_bytes().get(end),
                    None | Some(b' ' | b',' | b'.')
                ) {
                    continue;
                }
                if let Some((identity, onset)) = self
                    .environment
                    .reading(provider, &surface_text[..relative_end])
                {
                    longest = Some((end, identity, onset));
                }
            }
            longest
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
        identity CatalogName {
            generate catalog_identity { provider = PrimaryNames; }
        }
        identity OtherName {
            generate catalog_identity { provider = SecondaryNames; }
        }
        construction article: NamedRoot {
            element Article { name: identity CatalogName, }
            derive onset = name.onset;
            form an when name.onset is Vowel = "an" identity(name);
            form a otherwise = "a" identity(name);
        }
        construction catalog_possessive: CatalogPossessiveRoot {
            element CatalogPossessive { name: identity CatalogName, }
            derive number = Values::Plural;
            derive possessive_ending = name.possessive_ending;
            form singular when number is Singular = suffix(identity(name), "'s");
            form plural_s when all(
                number is Plural,
                name.possessive_ending is EndsInS
            ) =
                suffix(identity(name), "'");
            form plural_other otherwise = suffix(identity(name), "'s");
        }
        root NamedRoot { punctuation = "."; eoi = true; standalone_render = true; }
        root CatalogPossessiveRoot {
            punctuation = ".";
            eoi = true;
            standalone_render = true;
        }
    }

    fn environment() -> environment::ParserEnvironment {
        use macro_ron::v2::Onset;

        let vowel_spelling = String::from("Apple");
        let vowel_identity: Arc<str> = Arc::from("heuristic-vowel");
        let consonant_spelling = String::from("Banana");
        let consonant_identity: Arc<str> = Arc::from("heuristic-consonant");
        let rows = [
            environment::CatalogRow::new(
                CatalogProvider::PrimaryNames,
                vowel_identity,
                vowel_spelling,
                Onset::Consonant,
            ),
            environment::CatalogRow::new(
                CatalogProvider::PrimaryNames,
                consonant_identity,
                consonant_spelling,
                Onset::Vowel,
            ),
            environment::CatalogRow::new(
                CatalogProvider::SecondaryNames,
                "wrong-provider",
                "Apple",
                Onset::Vowel,
            ),
            environment::CatalogRow::new(
                CatalogProvider::PrimaryNames,
                "ends-in-s",
                "Players",
                Onset::Consonant,
            ),
            environment::CatalogRow::new(
                CatalogProvider::PrimaryNames,
                "other-ending",
                "Merfolk",
                Onset::Consonant,
            ),
        ];
        environment::ParserEnvironment::new(rows)
    }

    fn identity_terminal(rule_id: RuleId) -> LexicalTerminal {
        RULES
            .iter()
            .find(|rule| rule.id == rule_id)
            .expect("generated guarded article rule exists")
            .rhs
            .iter()
            .find_map(|position| match position {
                L(
                    terminal @ LexicalTerminal {
                        matcher: Lexical::CatalogIdentity(_),
                        ..
                    },
                ) => Some(*terminal),
                _ => None,
            })
            .expect("guarded article rule contains its generated catalog identity terminal")
    }

    fn scan_identity(
        environment: &environment::ParserEnvironment,
        context: &ParseContext<'_>,
        text: &str,
        byte_offset: usize,
        terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        scan_identity_at(
            environment,
            context,
            text,
            ScanPosition {
                byte_offset,
                case: CasePosition::Continuation,
                prefix: PrefixPosition::WordOwnedSpace,
            },
            terminal,
        )
    }

    fn scan_identity_at(
        environment: &environment::ParserEnvironment,
        context: &ParseContext<'_>,
        text: &str,
        position: ScanPosition,
        terminal: LexicalTerminal,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        scan_lexical(
            &ScanInput {
                text,
                position,
                environment,
                context,
            },
            terminal,
        )
    }

    fn catalog_possessive_rules() -> Vec<(RuleId, &'static str, LexicalTerminal)> {
        RULES
            .iter()
            .filter(|rule| rule.lhs == Category::CatalogPossessiveRoot)
            .filter_map(|rule| {
                let affix = rule.rhs.iter().find_map(|position| match position {
                    L(LexicalTerminal {
                        matcher: Lexical::Literal(actual @ ("'" | "'s")),
                        ..
                    }) => Some(*actual),
                    _ => None,
                });
                let identity = rule.rhs.iter().find_map(|position| match position {
                    L(
                        terminal @ LexicalTerminal {
                            matcher: Lexical::CatalogIdentity(_),
                            ..
                        },
                    ) => Some(*terminal),
                    _ => None,
                });
                Some((rule.id, affix?, identity?))
            })
            .collect()
    }

    #[derive(Default)]
    struct Recorder {
        providers: Vec<CatalogProvider>,
        identities: Vec<String>,
    }

    impl Visitor for Recorder {
        fn visit_catalog_provider(&mut self, provider: CatalogProvider) {
            self.providers.push(provider);
        }

        fn visit_catalog_name(&mut self, identity: &CatalogName) {
            self.identities
                .push(identity.canonical_identity().to_owned());
        }
    }

    #[test]
    fn catalog_only_generated_consumer_executes_frozen_onset_and_environment_abi() {
        use macro_ron::v2::Onset;

        let environment = environment();
        let context = ParseContext::default();
        let vowel_guard_terminal = identity_terminal(RuleId::NamedRootArticleAn);
        let consonant_guard_terminal = identity_terminal(RuleId::NamedRootArticleA);
        assert_eq!(vowel_guard_terminal, consonant_guard_terminal);
        assert_eq!(
            REQUIRED_CATALOG_PROVIDERS,
            [
                CatalogProvider::PrimaryNames,
                CatalogProvider::SecondaryNames
            ],
            "generated requirements are semantic-name lexicographic"
        );

        for (text, onset, selected, rejected, article, identity, expected) in [
            (
                "A Apple.",
                Onset::Consonant,
                RuleId::NamedRootArticleA,
                RuleId::NamedRootArticleAn,
                "a",
                "heuristic-vowel",
                "A Apple.",
            ),
            (
                "An Banana.",
                Onset::Vowel,
                RuleId::NamedRootArticleAn,
                RuleId::NamedRootArticleA,
                "an",
                "heuristic-consonant",
                "An Banana.",
            ),
        ] {
            let offset = article.len();
            let scanned = scan_identity(&environment, &context, text, offset, vowel_guard_terminal);
            assert!(matches!(
                scanned.as_slice(),
                [LexicalMatch {
                    value: Leaf::CatalogIdentity {
                        provider: CatalogProvider::PrimaryNames,
                        canonical_identity,
                        onset: actual_onset,
                        ..
                    },
                    ..
                }] if canonical_identity.as_ref() == identity && *actual_onset == onset
            ));
            assert_eq!(
                scanned[0].owner.as_ref().expect("scan owner").stable_id(),
                format!("identity:PrimaryNames/{identity}")
            );

            let children = [
                BuildValue::Leaf(Leaf::Literal(article)),
                BuildValue::Leaf(scanned[0].value.clone()),
            ];
            let built = build(selected, &children, &context)
                .expect("frozen provider onset satisfies its exact guarded form");
            assert!(
                build(rejected, &children, &context).is_none(),
                "the inverse heuristic-derived form is rejected"
            );
            let named = <NamedRoot as GeneratedRoot>::from_build(built)
                .expect("generated root unwraps its onset-carrying build value");
            let (rendered, claims) = render_named_root_with_claims(&named, &context, &environment);
            assert_eq!(rendered, expected);
            let surface = environment
                .catalog_surface(CatalogProvider::PrimaryNames, identity)
                .expect("render row exists");
            assert_eq!(&text[offset..scanned[0].end], format!(" {surface}"));
            assert!(claims.iter().any(|claim| {
                claim.owner.stable_id() == format!("identity:PrimaryNames/{identity}")
                    && rendered[claim.start..claim.end] == format!(" {surface}")
            }));
            assert_eq!(Render::render(&named, &context, &environment), expected);

            let mut recorder = Recorder::default();
            walk_named_root(&mut recorder, &named);
            assert_eq!(recorder.providers, [CatalogProvider::PrimaryNames]);
            assert_eq!(recorder.identities, [identity]);
        }

        let wrong_terminal = LexicalTerminal {
            matcher: Lexical::CatalogIdentity(usize::MAX),
            owner: LexicalOwnerTemplate::CatalogIdentity(usize::MAX),
            right_boundary: LexicalBoundary::Separated,
        };
        assert!(scan_identity(&environment, &context, "A Apple.", 1, wrong_terminal).is_empty());
        assert!(
            scan_identity(
                &environment,
                &context,
                "A Apple.",
                1,
                LexicalTerminal {
                    matcher: Lexical::CatalogIdentity(1),
                    owner: LexicalOwnerTemplate::CatalogIdentity(1),
                    right_boundary: LexicalBoundary::Separated,
                },
            )
            .iter()
            .all(|reading| matches!(
                &reading.value,
                Leaf::CatalogIdentity {
                    provider: CatalogProvider::SecondaryNames,
                    canonical_identity,
                    ..
                } if canonical_identity.as_ref() == "wrong-provider"
            )),
            "the other provider cannot be rebound as the explicit name provider"
        );
        assert!(CatalogName::new(&environment, "wrong-provider").is_none());

        let corrupted_provider = [
            BuildValue::Leaf(Leaf::Literal("a")),
            BuildValue::Leaf(Leaf::CatalogIdentity {
                provider: CatalogProvider::SecondaryNames,
                canonical_identity: Arc::from("wrong-provider"),
                onset: Onset::Consonant,
                possessive_ending: PossessiveEnding::Other,
            }),
        ];
        assert!(build(RuleId::NamedRootArticleA, &corrupted_provider, &context).is_none());

        let corrupted_onset = [
            BuildValue::Leaf(Leaf::Literal("a")),
            BuildValue::Leaf(Leaf::CatalogIdentity {
                provider: CatalogProvider::PrimaryNames,
                canonical_identity: Arc::from("heuristic-vowel"),
                onset: Onset::Vowel,
                possessive_ending: PossessiveEnding::Other,
            }),
        ];
        assert!(build(RuleId::NamedRootArticleA, &corrupted_onset, &context).is_none());
    }

    fn assert_catalog_possessive_ending_transport() {
        use macro_ron::v2::Onset;

        let environment = environment();
        let context = ParseContext::default();
        let rules = catalog_possessive_rules();
        assert_eq!(rules.len(), 3, "the fixture uses the three guarded forms");
        let identity_terminal = rules[0].2;
        assert!(
            rules
                .iter()
                .all(|(_, _, terminal)| *terminal == identity_terminal)
        );

        for (text, identity, ending, expected, spans) in [
            (
                "Players'",
                "ends-in-s",
                PossessiveEnding::EndsInS,
                "Players'.",
                [(0, 7), (7, 8)],
            ),
            (
                "Merfolk's",
                "other-ending",
                PossessiveEnding::Other,
                "Merfolk's.",
                [(0, 7), (7, 9)],
            ),
        ] {
            let scanned = scan_identity_at(
                &environment,
                &context,
                text,
                ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                identity_terminal,
            );
            assert!(matches!(
                scanned.as_slice(),
                [LexicalMatch {
                    end: 7,
                    value: Leaf::CatalogIdentity {
                        provider: CatalogProvider::PrimaryNames,
                        canonical_identity,
                        onset: actual_onset,
                        possessive_ending: actual_ending,
                    },
                    ..
                }] if canonical_identity.as_ref() == identity
                    && *actual_onset == Onset::Consonant
                    && *actual_ending == ending
            ));

            let identity_leaf = scanned[0].value.clone();
            let accepted = rules
                .iter()
                .filter_map(|(rule, affix, _)| {
                    let built = build(
                        *rule,
                        &[
                            BuildValue::Leaf(identity_leaf.clone()),
                            BuildValue::Leaf(Leaf::Literal(affix)),
                        ],
                        &context,
                    )?;
                    Some((*rule, *affix, built))
                })
                .collect::<Vec<_>>();
            assert_eq!(
                accepted.len(),
                1,
                "the scanned ending selects exactly one guarded suffix form",
            );

            let (selected, affix, built) = accepted
                .into_iter()
                .next()
                .expect("one guarded suffix form accepts the scanned ending");
            let possessive = <CatalogPossessiveRoot as GeneratedRoot>::from_build(built)
                .expect("generated root unwraps its ending-carrying build value");
            let (rendered, claims) =
                render_catalog_possessive_root_with_claims(&possessive, &context, &environment);
            assert_eq!(rendered, expected);
            let surface = expected.strip_suffix('.').expect("root punctuation exists");
            let claims = claims
                .into_iter()
                .filter(|claim| claim.end <= surface.len())
                .collect::<Vec<_>>();
            assert_eq!(
                claims
                    .iter()
                    .map(|claim| (claim.start, claim.end))
                    .collect::<Vec<_>>(),
                spans,
            );
            assert_eq!(&rendered[spans[0].0..spans[0].1], &text[..7]);
            assert_eq!(&rendered[spans[1].0..spans[1].1], affix);
            assert_eq!(
                claims[0].owner.stable_id(),
                format!("identity:PrimaryNames/{identity}"),
            );

            let mutated_ending = match ending {
                PossessiveEnding::EndsInS => PossessiveEnding::Other,
                PossessiveEnding::Other => PossessiveEnding::EndsInS,
            };
            let Leaf::CatalogIdentity {
                provider,
                canonical_identity,
                onset,
                ..
            } = identity_leaf
            else {
                panic!("catalog scan returns its catalog identity leaf")
            };
            let corrupted = Leaf::CatalogIdentity {
                provider,
                canonical_identity,
                onset,
                possessive_ending: mutated_ending,
            };
            assert!(
                build(
                    selected,
                    &[
                        BuildValue::Leaf(corrupted),
                        BuildValue::Leaf(Leaf::Literal(affix)),
                    ],
                    &context,
                )
                .is_none(),
                "a wrong catalog PossessiveEnding cannot cross the guarded build boundary",
            );
        }
    }

    #[test]
    fn catalog_identity_possessive_ending_drives_guarded_suffix_and_claims() {
        assert_catalog_possessive_ending_transport();
    }
}

pub use catalog_fixture::environment;
