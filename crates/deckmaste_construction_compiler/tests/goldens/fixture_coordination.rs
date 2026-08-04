mod __constructions_fixture_coordination {
    #![allow(
        clippy::nonminimal_bool,
        reason = "the `if !(…)` wrapper negates arbitrary author predicates \
                          uniformly, including `matches!` and nested `&&`/`||` trees; \
                          a De Morgan pass would add a whole rewriting layer for zero \
                          semantic gain"
    )]
    #![allow(
        clippy::must_use_candidate,
        reason = "accessors are generated per declared field, not chosen"
    )]
    #![allow(
        clippy::missing_errors_doc,
        reason = "try_new's error conditions are the declaration's `require` \
                          clauses, which the author wrote and can read"
    )]
    use super::*;
    fn __parts_fixture_member(value: &BoundMember) -> (&Comma, &FixturePhrase) {
        let BoundMember { comma, phrase } = value;
        (comma, phrase)
    }
    const _: fn(&BoundMember) -> (&Comma, &FixturePhrase) = __parts_fixture_member;
    const _: fn(&BoundPayload) = |_| {};
    #[derive(Debug, PartialEq, Eq)]
    pub struct FixturePairNode {
        members: Vec<BoundMember>,
        conjunction: Conjunction,
    }
    impl FixturePairNode {
        pub fn try_new(
            members: Vec<BoundMember>,
            conjunction: Conjunction,
        ) -> Result<Self, ::deckmaste_construction_compiler::runtime::DeclarationViolation>
        {
            if !(members.len() >= 2) {
                return Err(
                    ::deckmaste_construction_compiler::runtime::DeclarationViolation {
                        construction: "fixture_pair",
                        requirement: "members.len() >= 2",
                    },
                );
            }
            if !(matches!(conjunction, Conjunction::And | Conjunction::Or)) {
                return Err(
                    ::deckmaste_construction_compiler::runtime::DeclarationViolation {
                        construction: "fixture_pair",
                        requirement: "conjunction in [And, Or]",
                    },
                );
            }
            if !(members
                .last()
                .is_none_or(|member| matches!(member.comma, Comma::Present)))
            {
                return Err(
                    ::deckmaste_construction_compiler::runtime::DeclarationViolation {
                        construction: "fixture_pair",
                        requirement: "members.last.comma in [Present]",
                    },
                );
            }
            Ok(Self {
                members,
                conjunction,
            })
        }
        pub fn members(&self) -> &Vec<BoundMember> {
            &self.members
        }
        pub fn conjunction(&self) -> &Conjunction {
            &self.conjunction
        }
    }
    #[derive(Debug, PartialEq, Eq)]
    pub struct FixtureSoloNode {
        phrase: FixturePhrase,
        alt: Option<FixturePhrase>,
    }
    impl FixtureSoloNode {
        pub fn try_new(
            phrase: FixturePhrase,
            alt: Option<FixturePhrase>,
        ) -> Result<Self, ::deckmaste_construction_compiler::runtime::DeclarationViolation>
        {
            if !(alt.is_none()) {
                return Err(
                    ::deckmaste_construction_compiler::runtime::DeclarationViolation {
                        construction: "fixture_solo",
                        requirement: "alt.is_none()",
                    },
                );
            }
            Ok(Self { phrase, alt })
        }
        pub fn phrase(&self) -> &FixturePhrase {
            &self.phrase
        }
        pub fn alt(&self) -> &Option<FixturePhrase> {
            &self.alt
        }
    }
    #[allow(
        dead_code,
        reason = "the trait bound on assert_payload is the check; nothing calls this"
    )]
    fn __assert_free_witness_payloads() {
        fn assert_payload<T: ::deckmaste_features::SurfaceWitnessPayload>() {}
        assert_payload::<Comma>();
    }
    impl<'de> serde::Deserialize<'de> for FixtureSoloNode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(serde::Deserialize)]
            struct Raw {
                phrase: FixturePhrase,
                alt: Option<FixturePhrase>,
            }
            let raw = Raw::deserialize(deserializer)?;
            FixtureSoloNode::try_new(raw.phrase, raw.alt).map_err(|violation| {
                serde::de::Error::custom(format!(
                    "{}: {}",
                    violation.construction, violation.requirement
                ))
            })
        }
    }
    pub static FIXTURE_COORDINATION_DECLARATION: ::deckmaste_construction_compiler::runtime::GroupData = ::deckmaste_construction_compiler::runtime::GroupData {
        name: "fixture_coordination",
        elements: &["fixture_member", "empty_payload"],
        element_data: &[
            ::deckmaste_construction_compiler::runtime::ElementData {
                name: "fixture_member",
                bind_path: Some("BoundMember"),
                fields: &[
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "comma",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Scalar {
                            codec: "Comma",
                        },
                    },
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "phrase",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Subtree {
                            category: "FixturePhrase",
                            boxed: false,
                        },
                    },
                ],
            },
            ::deckmaste_construction_compiler::runtime::ElementData {
                name: "empty_payload",
                bind_path: Some("BoundPayload"),
                fields: &[],
            },
        ],
        constructions: &[
            ::deckmaste_construction_compiler::runtime::ConstructionData {
                id: "fixture_pair",
                category: "FixturePair",
                internal: false,
                own_type: Some("FixturePairNode"),
                bind_path: None,
                fields: &[
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "members",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Sequence {
                            element: "fixture_member",
                        },
                    },
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "conjunction",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Scalar {
                            codec: "Conjunction",
                        },
                    },
                ],
                witnesses: &[
                    ::deckmaste_construction_compiler::runtime::WitnessData {
                        name: "oxford",
                        class: ::deckmaste_construction_compiler::runtime::WitnessClassData::Stored {
                            path: "members.last.comma",
                        },
                    },
                ],
                deserialize: false,
                selection_unique: false,
                dominates: &["fixture_solo"],
                forms: &[
                    ::deckmaste_construction_compiler::runtime::FormData {
                        name: "plain",
                        ordinal: 0u16,
                        guarded: true,
                        atoms: &[
                            ::deckmaste_construction_compiler::runtime::AtomData::Hole(
                                "members",
                            ),
                            ::deckmaste_construction_compiler::runtime::AtomData::Lexeme(
                                "conjunction",
                            ),
                        ],
                    },
                    ::deckmaste_construction_compiler::runtime::FormData {
                        name: "fancy",
                        ordinal: 1u16,
                        guarded: true,
                        atoms: &[
                            ::deckmaste_construction_compiler::runtime::AtomData::Hole(
                                "members",
                            ),
                            ::deckmaste_construction_compiler::runtime::AtomData::Literal(
                                ",",
                            ),
                            ::deckmaste_construction_compiler::runtime::AtomData::Lexeme(
                                "conjunction",
                            ),
                        ],
                    },
                ],
            },
            ::deckmaste_construction_compiler::runtime::ConstructionData {
                id: "fixture_solo",
                category: "FixturePair",
                internal: false,
                own_type: Some("FixtureSoloNode"),
                bind_path: None,
                fields: &[
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "phrase",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Subtree {
                            category: "FixturePhrase",
                            boxed: false,
                        },
                    },
                    ::deckmaste_construction_compiler::runtime::FieldData {
                        name: "alt",
                        kind: ::deckmaste_construction_compiler::runtime::FieldKindData::Optional {
                            inner: &::deckmaste_construction_compiler::runtime::FieldKindData::Subtree {
                                category: "FixturePhrase",
                                boxed: false,
                            },
                        },
                    },
                ],
                witnesses: &[
                    ::deckmaste_construction_compiler::runtime::WitnessData {
                        name: "gap",
                        class: ::deckmaste_construction_compiler::runtime::WitnessClassData::Free {
                            ty: "Comma",
                        },
                    },
                ],
                deserialize: true,
                selection_unique: true,
                dominates: &[],
                forms: &[
                    ::deckmaste_construction_compiler::runtime::FormData {
                        name: "only",
                        ordinal: 0u16,
                        guarded: false,
                        atoms: &[
                            ::deckmaste_construction_compiler::runtime::AtomData::Hole(
                                "phrase",
                            ),
                            ::deckmaste_construction_compiler::runtime::AtomData::Hole(
                                "alt",
                            ),
                        ],
                    },
                ],
            },
        ],
    };
}
pub use __constructions_fixture_coordination::FIXTURE_COORDINATION_DECLARATION;
pub use __constructions_fixture_coordination::FixturePairNode;
pub use __constructions_fixture_coordination::FixtureSoloNode;
