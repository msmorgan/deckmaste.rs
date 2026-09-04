//! Linked memory across two abilities of one card ([CR#607.1], [Core is
//! explicit regions] law 8).
//!
//! Linkage is between two abilities PRINTED ON ONE OBJECT, so what a cell
//! means is not knowable from either ability alone: lowering compiles the card
//! twice, collecting the cell reads and writes on the first pass and declaring
//! the surviving cells as `Provenance::Linked` parameters on the second. A
//! read whose cell no ability on the card writes is a LOWERING ERROR naming
//! the card — never a register that quietly resolves to nothing at run time,
//! which is the failure mode this stage exists to remove.

use deckmaste_semantics as sem;

fn face(name: &str, abilities: Vec<sem::Ability>) -> sem::CardFace {
    sem::CardFace {
        name: name.into(),
        types: vec![sem::TypeDef {
            name: "Artifact".into(),
            permanent: true,
            confers: [].into(),
        }],
        abilities,
        ..sem::CardFace::default()
    }
}

fn activated(effect: sem::OneShotEffect) -> sem::Ability {
    sem::Ability::Activated(
        sem::ActivatedAbility {
            ability_word: None,
            cost: sem::Cost([sem::CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: [].into(),
            effect,
        }
        .into(),
    )
}

fn exile_and_note(cell: &str) -> sem::Ability {
    activated(sem::OneShotEffect::Noting(sem::Noting {
        key: cell.into(),
        effect: std::sync::Arc::new(sem::OneShotEffect::Act(sem::Action::Move(
            sem::Reference::You,
            sem::Destination::Zone(sem::Zone::Exile),
            [].into(),
            None,
        ))),
    }))
}

fn act_on_cell(cell: &str) -> sem::Ability {
    activated(sem::OneShotEffect::Act(sem::Action::Move(
        sem::Reference::Linked(cell.into()),
        sem::Destination::Zone(sem::Zone::Battlefield),
        [].into(),
        None,
    )))
}

fn abilities(card: &deckmaste_card::Card) -> &[deckmaste_core::Ability] {
    let deckmaste_card::Card::Normal(face) = card else {
        panic!("a Normal card lowers to a Normal card");
    };
    &face.characteristics.abilities
}

fn region_of(ability: &deckmaste_core::Ability) -> &deckmaste_core::Region {
    match ability {
        deckmaste_core::Ability::Activated(a) => &a.effect,
        other => panic!("expected an activated ability, got {other:?}"),
    }
}

/// The writer becomes a `Remember` and the reader declares the cell as a
/// `Provenance::Linked` parameter of its OWN region: the two abilities share
/// no activation, so the cell is the whole channel ([CR#607.2a]).
#[test]
fn a_cell_written_by_one_ability_becomes_a_parameter_of_the_ability_that_reads_it() {
    let card = sem::Card::Normal(face(
        "Linked Pair Test",
        vec![exile_and_note("exiled"), act_on_cell("exiled")],
    ));
    let lowered = deckmaste_lowering::lower_card(card).expect("a linked pair compiles");
    let abilities = abilities(&lowered);
    assert_eq!(abilities.len(), 2);

    let writes: Vec<&deckmaste_core::Remember> = region_of(&abilities[0])
        .body
        .iter()
        .filter_map(|instruction| match instruction {
            deckmaste_core::Instruction::Remember(remember) => Some(remember),
            _ => None,
        })
        .collect();
    assert_eq!(
        writes.len(),
        1,
        "the noting ability publishes exactly one cell"
    );
    assert_eq!(&*writes[0].cell, "exiled");

    let reader = region_of(&abilities[1]);
    let declared: Vec<_> = reader
        .linked_cells()
        .map(|(here, cell, kind)| (here, cell.to_string(), kind))
        .collect();
    assert_eq!(
        declared.len(),
        1,
        "the reading ability declares exactly the cell it reads, got {:?}",
        reader.params
    );
    assert_eq!(&declared[0].1, "exiled");
    assert_eq!(
        writes[0].kind, declared[0].2,
        "writer and reader agree on the cell's runtime shape"
    );
    assert!(
        reader
            .body
            .iter()
            .any(|instruction| format!("{instruction:?}")
                .contains(&format!("Reg(RefId({}))", declared[0].0.0))),
        "the reading body reads the declared parameter"
    );
    // The reader's cell is validated as a whole ability region.
    deckmaste_core::validate_announced(
        reader,
        &[],
        match &abilities[1] {
            deckmaste_core::Ability::Activated(a) => &a.cost,
            other => panic!("expected an activated ability, got {other:?}"),
        },
    )
    .expect("the reading region is well formed");
}

/// A card that WRITES a cell nothing reads gets no `Remember`: the instruction
/// is the declaration, so a cell no ability reads is not a cell.
#[test]
fn a_cell_no_ability_reads_publishes_nothing() {
    let card = sem::Card::Normal(face("Unread Note Test", vec![exile_and_note("exiled")]));
    let lowered = deckmaste_lowering::lower_card(card).expect("an unread note compiles");
    let region = region_of(&abilities(&lowered)[0]);
    assert!(
        !region
            .body
            .iter()
            .any(|instruction| matches!(instruction, deckmaste_core::Instruction::Remember(_))),
        "no reader, no cell — the note stays a region-local binding"
    );
}

/// A linked read whose cell NO ability on the card writes is refused at
/// lowering, naming the card ([CR#607.1] — the two abilities have to be
/// printed on one object for the linkage to exist at all). This is the
/// guarantee that a declared read is never an unavailable value at run time.
#[test]
fn a_linked_read_with_no_writer_on_the_card_is_a_lowering_error() {
    let card = sem::Card::Normal(face("Dangling Link Test", vec![act_on_cell("exiled")]));
    let error = deckmaste_lowering::lower_card(card)
        .expect_err("a linked read with no writer cannot be compiled");
    assert_eq!(&*error.card, "Dangling Link Test");
    assert!(
        error.message.contains("has no declared cell"),
        "the diagnostic states the refusal, got {:?}",
        error.message
    );
}

/// FIXTURE — a `Composite` body with a CAPTURE LIST ([Core is explicit
/// regions] laws 1 and 7). A keyword ability granted inside another ability's
/// region carries its own abilities; each is a region, and because it is built
/// INSIDE an enclosing register file it declares that file explicitly: its own
/// intrinsic prefix, then one `Provenance::Capture` per enclosing register.
/// Nothing else crosses.
#[test]
fn a_composite_body_granted_inside_a_region_declares_a_capture_list() {
    let carried = sem::Ability::Triggered(
        sem::TriggeredAbility {
            ability_word: None,
            event: sem::EventFilter::ZoneChange {
                what: sem::Predicate::Ref(sem::Reference::This),
                from: None,
                to: Some(sem::Zone::Battlefield),
                cause: None,
            },
            from: None,
            condition: None,
            limits: [].into(),
            where_x: None,
            effect: sem::OneShotEffect::Act(sem::Action::ChangeLife(
                sem::Reference::You,
                sem::LifeOp::Up(sem::Count::Literal(1)),
            )),
        }
        .into(),
    );
    let card = sem::Card::Normal(face(
        "Granted Composite Test",
        vec![activated(sem::OneShotEffect::Until(
            sem::Duration::FixedUntil(sem::TurnMarker::EndOfTurn),
            [sem::StaticEffect::Modify(
                sem::Reference::This,
                sem::Modification::GainAbility(std::sync::Arc::new(sem::Ability::Keyword(
                    sem::KeywordAbility::Composite {
                        name: "Testword".into(),
                        abilities: vec![carried],
                    },
                ))),
            )]
            .into(),
        ))],
    ));
    let lowered = deckmaste_lowering::lower_card(card).expect("a granted composite compiles");
    let granted = carried_keyword_abilities(&abilities(&lowered)[0]);
    assert_eq!(granted.len(), 1, "the composite carries one ability");
    let deckmaste_core::Ability::Triggered(triggered) = granted[0] else {
        panic!(
            "expected the carried triggered ability, got {:?}",
            granted[0]
        );
    };
    let params = &triggered.effect.params;
    let intrinsic = deckmaste_core::event_region_params().len() + 1; // + announced X
    assert!(
        params.len() > intrinsic,
        "the carried region declares captures beyond its intrinsic prefix, got {params:?}"
    );
    for extra in &params[intrinsic..] {
        assert!(
            matches!(extra.provenance, deckmaste_core::Provenance::Capture(_)),
            "only a declared capture crosses a region boundary, found {:?}",
            extra.provenance
        );
    }
    // Every capture names a register the ENCLOSING region really declares —
    // which is what `deckmaste_core::validate` enforces at load.
    let enclosing = region_of(&abilities(&lowered)[0]);
    for (_, outer, _) in triggered.effect.captures() {
        assert!(
            (outer.0 as usize) < enclosing.params.len(),
            "capture of RefId({}) names no parameter of the enclosing region",
            outer.0
        );
    }
}

/// The abilities a granted `KeywordAbility::Composite` carries.
fn carried_keyword_abilities(ability: &deckmaste_core::Ability) -> Vec<&deckmaste_core::Ability> {
    fn from_static(effect: &deckmaste_core::StaticSpec) -> Vec<&deckmaste_core::Ability> {
        match effect {
            deckmaste_core::StaticSpec::Modify(
                _,
                deckmaste_core::Modification::GainAbility(granted),
            ) => match granted.as_ref() {
                deckmaste_core::Ability::Keyword(deckmaste_core::KeywordAbility::Composite {
                    abilities,
                    ..
                }) => abilities.iter().collect(),
                other => vec![other],
            },
            _ => Vec::new(),
        }
    }
    let mut out = Vec::new();
    for instruction in region_of(ability).body.iter() {
        if let deckmaste_core::Instruction::Until(_, parts) = instruction {
            for part in parts.iter() {
                out.extend(from_static(part));
            }
        }
    }
    out
}
