#![allow(
    dead_code,
    reason = "the minimal consumer supplies and executes every generated catalog-identity boundary"
)]

mod catalog_fixture {
    use std::sync::Arc;

    use RulePosition::Lexical as L;
    use deckmaste_construction::constructions;

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
        fn word_end(&self, running_text: &str) -> Option<usize> {
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
        root NamedRoot { punctuation = "."; eoi = true; standalone_render = true; }
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
        scan_lexical(
            &ScanInput {
                text,
                position: ScanPosition {
                    byte_offset,
                    case: CasePosition::Continuation,
                    prefix: PrefixPosition::WordOwnedSpace,
                },
                environment,
                context,
            },
            terminal,
        )
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
            }),
        ];
        assert!(build(RuleId::NamedRootArticleA, &corrupted_provider, &context).is_none());

        let corrupted_onset = [
            BuildValue::Leaf(Leaf::Literal("a")),
            BuildValue::Leaf(Leaf::CatalogIdentity {
                provider: CatalogProvider::PrimaryNames,
                canonical_identity: Arc::from("heuristic-vowel"),
                onset: Onset::Vowel,
            }),
        ];
        assert!(build(RuleId::NamedRootArticleA, &corrupted_onset, &context).is_none());
    }
}

pub use catalog_fixture::environment;
