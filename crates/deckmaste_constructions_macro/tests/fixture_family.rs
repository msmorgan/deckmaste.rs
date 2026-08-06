//! The sealed fixture families, end to end through constructions!.
//!
//! `fixture_coordination` is the milestone's own-mode behavioral gate: the
//! same declaration that produced the committed golden
//! (`fixture_coordination_group()` in `deckmaste_construction_compiler`'s
//! `tests/golden_real.rs`, parsed from byte-identical DSL text in
//! `parse.rs`'s `fixture_dsl()`), compiled through the real `constructions!`
//! proc macro and exercised at runtime — not just checked as emitted source
//! text. `bind_probe` is the bind-mode sibling (CF-7): it exercises the
//! generated `build_*`/`parts_*` doors and the shared `DeclarationViolation`
//! type in the same way.
//!
//! This also closes the last open deletion-mapping row from Task 12: the
//! Milestone-0 spike's `ron` deserialize round-trip
//! (`spike.rs`'s `deserialize_routes_through_the_validator`-shaped test) was
//! retired with its successor explicitly deferred here.
//! `deserialize_routes_through_the_validator` below is that successor: a
//! round trip PLUS a genuine rejection through the real validating
//! `Deserialize` impl, not just a parse-and-discard.

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FixturePhrase;

#[derive(Debug, PartialEq, Eq)]
pub enum FixturePair {
    Pair(FixturePairNode),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundMember {
    pub comma: Comma,
    pub phrase: FixturePhrase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoundPayload {
    Present,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoundVariant {
    Phrase(FixturePhrase),
    Boxed(Box<FixturePhrase>),
}

use deckmaste_features::Comma;
use deckmaste_features::Conjunction;

deckmaste_constructions_macro::constructions! {
    group fixture_coordination;

    element fixture_member {
        comma: opt lex Comma,
        phrase: hole FixturePhrase,
    }

    element bound_fixture_member bind BoundMember {
        comma: lex Comma,
        phrase: hole FixturePhrase,
    }

    element bound_variant bind BoundVariant {
        variant Phrase: hole FixturePhrase,
        variant Boxed: hole box FixturePhrase,
    }

    element empty_payload bind BoundPayload {}

    construction fixture_pair: FixturePair {
        own FixturePairNode {
            members: seq fixture_member,
            conjunction: lex Conjunction,
        }
        project Pair;
        require members.len() >= 2;
        require conjunction in [And, Or];
        require members.last.comma in [Present];
        witness oxford = stored members.last.comma;
        form plain @ 0 when conjunction in [And] = members lex(conjunction);
        form fancy @ 1 when conjunction in [Or] = members "," lex(conjunction);
        dominates fixture_solo;
    }

    construction fixture_solo: FixturePair {
        own FixtureSoloNode {
            phrase: hole FixturePhrase,
            alt: opt hole FixturePhrase,
        }
        require alt.is_none();
        witness gap = free Comma;
        form only @ 0 = phrase alt;
        selection unique;
        deserialize;
    }

    construction fixture_tagged: FixturePair {
        own FixtureTaggedNode {
            phrase: hole FixturePhrase,
            tag: opt lex Conjunction,
        }
        require tag in [And, Or];
        form only @ 0 = phrase lex(tag);
    }

    construction fixture_variant_sequence: FixturePair {
        own FixtureVariantSequenceNode {
            members: seq bound_variant,
        }
        require members.len() >= 1;
        recognize require members.first.variant in [Phrase];
        form only @ 0 = members;
    }
}

#[test]
fn try_new_enforces_every_require() {
    let member = || FixtureMember {
        comma: None,
        phrase: FixturePhrase,
    };
    let pair = FixturePairNode::try_new(vec![member(), member()], Conjunction::And)
        .expect("two members + and is admitted");
    assert_eq!(pair.conjunction(), &Conjunction::And);
    assert_eq!(pair.members().len(), 2);

    let short = FixturePairNode::try_new(vec![member()], Conjunction::And)
        .expect_err("one member violates the length require");
    assert_eq!(short.construction, "fixture_pair");
    assert_eq!(short.requirement, "members.len() >= 2");

    let bad_word = FixturePairNode::try_new(vec![member(), member()], Conjunction::Then)
        .expect_err("Then is outside the admitted set");
    assert_eq!(bad_word.requirement, "conjunction in [And, Or]");

    // Conjunction::Or is admitted too — the `In` check is a genuine set
    // membership test, not an accidental match on `And` alone.
    let or_pair = FixturePairNode::try_new(vec![member(), member()], Conjunction::Or)
        .expect("Or is also admitted by `conjunction in [And, Or]`");
    assert_eq!(or_pair.conjunction(), &Conjunction::Or);
}

#[test]
fn last_path_requires_are_vacuous_on_empty_and_checked_on_the_last_member() {
    // fixture_pair also requires members.len() >= 2, so exercise the .last
    // check through values that pass the length gate.
    let plain = |comma| FixtureMember {
        comma,
        phrase: FixturePhrase,
    };
    let ok = FixturePairNode::try_new(
        vec![plain(None), plain(Some(Comma::Present))],
        Conjunction::And,
    )
    .expect("a Present last comma satisfies the .last require");
    assert_eq!(ok.members().len(), 2);
    // Reading-B nesting: an ABSENT optional on the last member is vacuously
    // admitted by `in [Present]`.
    FixturePairNode::try_new(vec![plain(None), plain(None)], Conjunction::And)
        .expect("an absent last comma is vacuously admitted (optional reading B)");
    let rejected = FixturePairNode::try_new(
        vec![plain(None), plain(Some(Comma::Absent))],
        Conjunction::And,
    )
    .expect_err("a present-but-Absent last comma violates the require");
    assert_eq!(rejected.requirement, "members.last.comma in [Present]");
}

#[test]
fn optional_in_is_vacuously_true_when_absent() {
    let absent = FixtureTaggedNode::try_new(FixturePhrase, None)
        .expect("reading B: an absent optional satisfies `in [...]` vacuously");
    assert_eq!(absent.tag(), &None);
    let admitted = FixtureTaggedNode::try_new(FixturePhrase, Some(Conjunction::Or))
        .expect("Or is in the admitted set");
    assert_eq!(admitted.tag(), &Some(Conjunction::Or));
    let rejected = FixtureTaggedNode::try_new(FixturePhrase, Some(Conjunction::Then))
        .expect_err("Then is present and outside the set");
    assert_eq!(rejected.requirement, "tag in [And, Or]");
}

#[test]
fn recognition_only_requirement_does_not_narrow_the_typed_builder() {
    let admitted = FixtureVariantSequenceNode::try_new(vec![BoundVariant::Phrase(FixturePhrase)])
        .expect("the declared first variant is admitted");
    assert_eq!(admitted.members().len(), 1);

    let admitted =
        FixtureVariantSequenceNode::try_new(vec![BoundVariant::Boxed(Box::new(FixturePhrase))])
            .expect("recognition-only requirements do not reject valid typed AST values");
    assert_eq!(admitted.members().len(), 1);
}

#[test]
fn deserialize_routes_through_the_validator() {
    // Round trip: a valid payload deserializes through the real, generated
    // `Deserialize` impl (in-body `Raw` mirror -> `try_new`), not a bypassed
    // `Self { .. }` literal.
    let ok: FixtureSoloNode = ron::from_str("(phrase: (), alt: None)").expect("valid payload");
    assert_eq!(ok.phrase(), &FixturePhrase);
    assert_eq!(ok.alt(), &None);

    // Rejection: a structurally well-formed payload that violates the
    // construction's `require alt.is_none()` is rejected by `try_new`
    // through the deserializer, with the violation surfacing in the error.
    let err = ron::from_str::<FixtureSoloNode>("(phrase: (), alt: Some(()))")
        .expect_err("alt must be none");
    let message = err.to_string();
    assert!(
        message.contains("fixture_solo"),
        "error names the construction: {message}"
    );
    assert!(
        message.contains("alt.is_none()"),
        "error names the requirement: {message}"
    );
}

#[test]
fn declaration_data_traces_to_the_one_declaration() {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::PredicateData;
    use deckmaste_construction_compiler::runtime::WitnessClassData;
    let data = &FIXTURE_COORDINATION_DECLARATION;
    assert_eq!(data.name, "fixture_coordination");
    assert_eq!(
        data.elements,
        &[
            "fixture_member",
            "bound_fixture_member",
            "bound_variant",
            "empty_payload",
        ],
    );
    assert_eq!(data.element_data.len(), 4);
    assert_eq!(data.element_data[0].name, "fixture_member");
    assert_eq!(data.element_data[0].bind_path, None);
    assert_eq!(
        data.element_data[0]
            .fields
            .iter()
            .map(|field| field.name)
            .collect::<Vec<_>>(),
        vec!["comma", "phrase"],
        "element fields preserve declaration order exactly",
    );
    assert_eq!(
        data.element_data[0].fields[0].kind,
        FieldKindData::Optional {
            inner: &FieldKindData::Scalar { codec: "Comma" },
        }
    );
    assert_eq!(
        data.element_data[0].fields[1].kind,
        FieldKindData::Subtree {
            category: "FixturePhrase",
            boxed: false,
        }
    );
    assert_eq!(data.element_data[1].name, "bound_fixture_member");
    assert_eq!(data.element_data[1].bind_path, Some("BoundMember"));
    assert_eq!(
        data.element_data[1]
            .fields
            .iter()
            .map(|field| field.name)
            .collect::<Vec<_>>(),
        vec!["comma", "phrase"],
        "bound element fields preserve declaration order exactly",
    );
    assert_eq!(
        data.element_data[1].fields[0].kind,
        FieldKindData::Scalar { codec: "Comma" }
    );
    assert_eq!(
        data.element_data[1].fields[1].kind,
        FieldKindData::Subtree {
            category: "FixturePhrase",
            boxed: false,
        }
    );
    assert_eq!(data.element_data[2].name, "bound_variant");
    assert_eq!(data.element_data[2].bind_path, Some("BoundVariant"));
    assert!(data.element_data[2].fields.is_empty());
    assert_eq!(
        data.element_data[2]
            .variants
            .iter()
            .map(|variant| variant.name)
            .collect::<Vec<_>>(),
        vec!["Phrase", "Boxed"],
        "bound enum variants preserve declaration order exactly",
    );
    assert_eq!(
        data.element_data[2].variants[0].payload,
        FieldKindData::Subtree {
            category: "FixturePhrase",
            boxed: false,
        },
    );
    assert_eq!(
        data.element_data[2].variants[1].payload,
        FieldKindData::Subtree {
            category: "FixturePhrase",
            boxed: true,
        },
    );
    assert_eq!(data.element_data[3].name, "empty_payload");
    assert_eq!(data.element_data[3].bind_path, Some("BoundPayload"));
    assert!(data.element_data[3].fields.is_empty());
    assert!(data.element_data[3].variants.is_empty());
    let ids: Vec<&str> = data.constructions.iter().map(|c| c.id).collect();
    assert_eq!(
        ids,
        vec![
            "fixture_pair",
            "fixture_solo",
            "fixture_tagged",
            "fixture_variant_sequence"
        ]
    );
    let pair = &data.constructions[0];
    assert_eq!(pair.own_type, Some("FixturePairNode"));
    assert_eq!(pair.dominates, &["fixture_solo"]);
    assert!(!pair.deserialize);
    let ordinals: Vec<u16> = pair.forms.iter().map(|f| f.ordinal).collect();
    assert_eq!(ordinals, vec![0, 1]);
    assert_eq!(
        pair.forms[1].atoms,
        &[
            AtomData::Hole("members"),
            AtomData::Literal(","),
            AtomData::Lexeme("conjunction"),
        ]
    );
    assert_eq!(pair.fields.len(), 2);
    let variant_sequence = &data.constructions[3];
    assert_eq!(variant_sequence.requirements.len(), 1);
    assert_eq!(variant_sequence.recognition_requirements.len(), 1);
    assert_eq!(
        variant_sequence.recognition_requirements[0].description,
        "members.first.variant in [Phrase]"
    );
    assert_eq!(
        variant_sequence.recognition_requirements[0].predicate,
        PredicateData::In {
            path: "members.first.variant",
            allowed: &["Phrase"],
        }
    );
    assert_eq!(pair.fields[0].name, "members");
    assert_eq!(
        pair.fields[0].kind,
        FieldKindData::Sequence {
            element: "fixture_member"
        }
    );
    assert_eq!(
        pair.fields[1].kind,
        FieldKindData::Scalar {
            codec: "Conjunction"
        }
    );
    assert_eq!(pair.witnesses.len(), 1);
    assert_eq!(pair.witnesses[0].name, "oxford");
    assert_eq!(
        pair.witnesses[0].class,
        WitnessClassData::Stored {
            path: "members.last.comma"
        }
    );
    let solo = &data.constructions[1];
    assert!(solo.deserialize);
    assert!(solo.selection_unique);
    assert_eq!(
        solo.witnesses,
        &[deckmaste_construction_compiler::runtime::WitnessData {
            name: "gap",
            class: WitnessClassData::Free { ty: "Comma" },
        }]
    );
    assert_eq!(
        solo.fields[1].kind,
        FieldKindData::Optional {
            inner: &FieldKindData::Subtree {
                category: "FixturePhrase",
                boxed: false
            },
        }
    );
}

#[test]
fn bound_enum_mapping_builds_and_destructures_every_declared_variant() {
    let phrase = build_bound_variant_phrase(FixturePhrase);
    assert!(matches!(
        parts_bound_variant(&phrase),
        BoundVariantVariantRef::Phrase(payload) if *payload == FixturePhrase
    ));

    let boxed = build_bound_variant_boxed(Box::new(FixturePhrase));
    assert!(matches!(
        parts_bound_variant(&boxed),
        BoundVariantVariantRef::Boxed(payload) if **payload == FixturePhrase
    ));
}

#[test]
fn declaration_metadata_builders_restore_typed_elements_and_constructions() {
    use deckmaste_construction_compiler::runtime::ErasedBuildError;

    let data = &FIXTURE_COORDINATION_DECLARATION;
    let member_builder = data.element_data[0].erased_builders[0];
    let member = member_builder(vec![Box::new(None::<Comma>), Box::new(FixturePhrase)])
        .expect("the emitted element builder accepts declaration-order fields")
        .downcast::<FixtureMember>()
        .expect("the erased element has its generated concrete type");
    assert_eq!(member.comma, None);
    let sequence_builder = data.element_data[0]
        .erased_sequence_builder
        .expect("every constructible element exposes a typed sequence collector");
    let members = sequence_builder(vec![member as Box<dyn std::any::Any>])
        .expect("the collector restores Vec<Element>")
        .downcast::<Vec<FixtureMember>>()
        .expect("the erased sequence has its declared concrete element type");
    assert_eq!(members.len(), 1);

    let variant_builder = data.element_data[2].erased_builders[1];
    let variant = variant_builder(vec![Box::new(Box::new(FixturePhrase))])
        .expect("the emitted enum builder accepts its declared payload")
        .downcast::<BoundVariant>()
        .expect("the erased enum has its bound concrete type");
    assert!(matches!(*variant, BoundVariant::Boxed(_)));

    let pair_builder = data.constructions[0]
        .erased_builder
        .expect("own-mode constructions always expose an erased builder");
    let pair = pair_builder(vec![
        Box::new(vec![
            FixtureMember {
                comma: None,
                phrase: FixturePhrase,
            },
            FixtureMember {
                comma: None,
                phrase: FixturePhrase,
            },
        ]),
        Box::new(Conjunction::And),
    ])
    .expect("the erased door routes through try_new")
    .downcast::<FixturePairNode>()
    .expect("the erased construction has its generated concrete type");
    assert_eq!(pair.members().len(), 2);

    let error = pair_builder(vec![Box::new(FixturePhrase), Box::new(Conjunction::And)])
        .expect_err("a wrong erased field type is diagnosed at its declaration slot");
    assert!(matches!(
        error,
        ErasedBuildError::WrongFieldType {
            owner: "fixture_pair",
            field: "members",
            ..
        }
    ));
}

#[test]
fn declaration_projection_wraps_the_owned_type_in_its_public_category() {
    let construction = &FIXTURE_COORDINATION_DECLARATION.constructions[0];
    assert_eq!(construction.projection_variant, Some("Pair"));

    let value = FixturePairNode::try_new(
        vec![
            FixtureMember {
                comma: None,
                phrase: FixturePhrase,
            },
            FixtureMember {
                comma: None,
                phrase: FixturePhrase,
            },
        ],
        Conjunction::And,
    )
    .expect("the owned construction is valid");
    let projected =
        construction
            .erased_projector
            .expect("a declared projection emits an erased projector")(Box::new(value))
        .expect("the projector accepts its construction's owned type")
        .downcast::<FixturePair>()
        .expect("the projector returns the declared public category");
    assert!(matches!(*projected, FixturePair::Pair(_)));
}

#[derive(Default)]
struct RecordingLinearizer {
    events: Vec<String>,
    tokens: Vec<&'static str>,
}

impl RecordingLinearizer {
    fn rendered(&self) -> String {
        let mut rendered = String::new();
        for token in &self.tokens {
            if !(rendered.is_empty() || *token == ",") {
                rendered.push(' ');
            }
            rendered.push_str(token);
        }
        rendered
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor for RecordingLinearizer {
    type Error = std::convert::Infallible;

    fn begin_form(
        &mut self,
        construction: &'static str,
        form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        self.events
            .push(format!("form:{construction}:{form}:{ordinal}"));
        Ok(())
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), Self::Error> {
        self.events.push(format!("literal:{literal}"));
        self.tokens.push(literal);
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        assert_eq!(category, "FixturePhrase");
        assert!(
            (value as &dyn std::any::Any)
                .downcast_ref::<FixturePhrase>()
                .is_some()
        );
        self.events.push(format!("subtree:{category}"));
        self.tokens.push("phrase");
        Ok(())
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let value = value as &dyn std::any::Any;
        match codec {
            "Comma" => {
                let comma = value.downcast_ref::<Comma>().expect("typed Comma callback");
                self.events.push(format!("comma:{comma:?}"));
                if comma.is_present() {
                    self.tokens.push(",");
                }
            }
            "Conjunction" => {
                let conjunction = value
                    .downcast_ref::<Conjunction>()
                    .expect("typed Conjunction callback");
                self.events.push(format!("conjunction:{conjunction:?}"));
                self.tokens.push(conjunction.spelling());
            }
            other => panic!("unexpected codec {other}"),
        }
        Ok(())
    }

    fn begin_sequence(&mut self, field: &'static str, len: usize) -> Result<(), Self::Error> {
        self.events.push(format!("sequence:{field}:{len}"));
        Ok(())
    }

    fn optional(&mut self, field: &'static str, present: bool) -> Result<(), Self::Error> {
        self.events.push(format!("optional:{field}:{present}"));
        Ok(())
    }

    fn stored_witness<T: std::any::Any>(
        &mut self,
        name: &'static str,
        path: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let comma = (value as &dyn std::any::Any)
            .downcast_ref::<Option<Comma>>()
            .expect("the stored witness keeps its optional Comma type");
        self.events.push(format!("witness:{name}:{path}:{comma:?}"));
        Ok(())
    }
}

#[test]
fn emitted_linearizer_replays_form_structure_and_presence_values() {
    let pair = FixturePairNode::try_new(
        vec![
            FixtureMember {
                comma: Some(Comma::Absent),
                phrase: FixturePhrase,
            },
            FixtureMember {
                comma: Some(Comma::Present),
                phrase: FixturePhrase,
            },
        ],
        Conjunction::And,
    )
    .expect("the declaration admits the typed fixture");
    let mut visitor = RecordingLinearizer::default();
    linearize_fixture_pair_with(&pair, &mut visitor).expect("one guarded form matches");

    assert_eq!(visitor.rendered(), "phrase, phrase and");
    assert!(visitor.events.contains(&"comma:Absent".to_owned()));
    assert!(visitor.events.contains(&"comma:Present".to_owned()));
    assert!(
        visitor
            .events
            .contains(&"form:fixture_pair:plain:0".to_owned())
    );
    assert!(
        visitor
            .events
            .contains(&"witness:oxford:members.last.comma:Some(Present)".to_owned())
    );
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundPair {
    pub left: FixturePhrase,
    pub right: Option<FixturePhrase>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundMembers {
    pub members: Vec<BoundMember>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EmptyBoundSequence {
    pub payloads: Vec<BoundPayload>,
}

deckmaste_constructions_macro::constructions! {
    group bind_probe;

    element fixture_member bind BoundMember {
        comma: lex Comma,
        phrase: hole FixturePhrase,
    }

    element empty_payload bind BoundPayload {}

    construction bind_pair: FixturePair {
        bind BoundPair {
            left: hole FixturePhrase,
            right: opt hole FixturePhrase,
        }
        require right.is_some();
        form both @ 0 when right.is_some() = left right;
    }

    construction bind_members: FixturePair {
        bind BoundMembers {
            members: seq fixture_member,
        }
        form only @ 0 = members;
    }

    construction empty_bound_sequence: FixturePair {
        bind EmptyBoundSequence {
            payloads: seq empty_payload,
        }
        require payloads.len() == 0;
        form only @ 0 = payloads;
    }
}

#[test]
fn bind_builder_enforces_requires_and_destructures() {
    let pair = build_bind_pair(FixturePhrase, Some(FixturePhrase))
        .expect("present right side is admitted");
    let (left, right) = parts_bind_pair(&pair);
    assert_eq!(left, &FixturePhrase);
    assert_eq!(right, &Some(FixturePhrase));

    let violation =
        build_bind_pair(FixturePhrase, None).expect_err("absent right side violates the require");
    assert_eq!(violation.construction, "bind_pair");
    assert_eq!(violation.requirement, "right.is_some()");

    let invalid = BoundPair {
        left: FixturePhrase,
        right: None,
    };
    let error = linearize_bind_pair_with(&invalid, &mut RecordingLinearizer::default())
        .expect_err("a handwritten bind value outside every form is explicit");
    assert_eq!(
        error,
        deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
            construction: "bind_pair",
        },
    );
}

#[test]
fn bind_construction_uses_handwritten_bound_element_type() {
    let members = vec![BoundMember {
        comma: Comma::Present,
        phrase: FixturePhrase,
    }];
    let value = build_bind_members(members).expect("bound member vector builds");
    let projected = parts_bind_members(&value);
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].comma, Comma::Present);
}

#[test]
fn empty_bound_element_sequence_is_checked_at_runtime() {
    let value = build_empty_bound_sequence(Vec::new()).expect("the proved-empty sequence builds");
    let payloads = parts_empty_bound_sequence(&value);
    assert!(payloads.is_empty());
    let violation = build_empty_bound_sequence(vec![BoundPayload::Present])
        .expect_err("a non-empty opaque sequence violates its direct requirement");
    assert_eq!(violation.requirement, "payloads.len() == 0");
}
