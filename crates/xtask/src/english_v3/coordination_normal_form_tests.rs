use super::super::encoded;
use super::*;

fn record(name: &str, fields: &[(&str, Tree)]) -> Tree {
    Tree::Record(
        name.into(),
        fields
            .iter()
            .map(|(n, v)| ((*n).into(), v.clone()))
            .collect(),
    )
}
fn word(id: &str) -> Tree {
    record(
        "Word",
        &[(
            "value",
            Tree::Call(
                "Word".into(),
                vec![record(
                    "Word",
                    &[
                        (
                            "lexeme",
                            Tree::Call("Ident".into(), vec![atom(&format!("{id:?}"))]),
                        ),
                        ("form", atom("Plain")),
                        ("variant", atom("0")),
                        ("capitalization", atom("Lower")),
                    ],
                )],
            ),
        )],
    )
}
fn binary(cat: &str, left: Tree, marker: &str, right: Tree) -> Tree {
    record(
        "Coordination",
        &[
            ("form", atom("0")),
            ("category", atom(cat)),
            ("left", left),
            ("coordinator", word(&format!("vocab:Coordinator/{marker}"))),
            ("right", right),
        ],
    )
}
fn clause(left: Tree, marker: &str, right: Tree, comma: bool) -> Tree {
    record(
        "ClauseCoordination",
        &[
            ("form", atom(if comma { "0" } else { "1" })),
            ("left", left),
            ("coordinator", word(&format!("vocab:Coordinator/{marker}"))),
            ("right", right),
        ],
    )
}
fn bare(head: Tree) -> Tree {
    imperative(head)
}
fn nf(tree: Tree) -> Tree {
    normalize(tree).unwrap()
}
fn wrapper(name: &str, head: Tree, dependent: Tree) -> Tree {
    record(
        name,
        &[
            ("form", atom("0")),
            ("head", head),
            ("dependent", dependent),
        ],
    )
}

#[test]
fn same_coordinator_grouping_retains_link_and_member_order() {
    for marker in ["And", "Or", "AndOr"] {
        let left = binary(
            "NounPhrase",
            binary("NounPhrase", atom("a"), marker, atom("b")),
            marker,
            atom("c"),
        );
        let right = binary(
            "NounPhrase",
            atom("a"),
            marker,
            binary("NounPhrase", atom("b"), marker, atom("c")),
        );
        assert_eq!(nf(left), nf(right.clone()));
        let reversed = binary(
            "NounPhrase",
            atom("b"),
            marker,
            binary("NounPhrase", atom("a"), marker, atom("c")),
        );
        assert_ne!(nf(right), nf(reversed));
    }
}
#[test]
fn bare_imperative_levels_compose_with_grouping() {
    let vp = binary(
        "SecondaryVerbPhrase",
        atom("draw"),
        "And",
        binary("SecondaryVerbPhrase", atom("gain"), "And", atom("create")),
    );
    let clauses = clause(
        clause(bare(atom("draw")), "And", bare(atom("gain")), false),
        "And",
        bare(atom("create")),
        false,
    );
    assert_eq!(nf(bare(vp)), nf(clauses));
}
#[test]
fn then_level_is_not_then_grouping() {
    assert_eq!(
        nf(bare(binary(
            "SecondaryVerbPhrase",
            atom("a"),
            "Then",
            atom("b")
        ))),
        nf(clause(bare(atom("a")), "Then", bare(atom("b")), true))
    );
    for marker in ["Then", "But"] {
        assert_ne!(
            nf(binary(
                "Clause",
                binary("Clause", atom("a"), marker, atom("b")),
                marker,
                atom("c")
            )),
            nf(binary(
                "Clause",
                atom("a"),
                marker,
                binary("Clause", atom("b"), marker, atom("c"))
            ))
        );
    }
}
#[test]
fn mixed_coordinators_correlatives_and_categories_are_boundaries() {
    assert_ne!(
        nf(binary(
            "NounPhrase",
            binary("NounPhrase", atom("a"), "And", atom("b")),
            "Or",
            atom("c")
        )),
        nf(binary(
            "NounPhrase",
            atom("a"),
            "And",
            binary("NounPhrase", atom("b"), "Or", atom("c"))
        ))
    );
    let ordinary = binary("NounPhrase", atom("a"), "And", atom("b"));
    assert_ne!(
        nf(ordinary.clone()),
        nf(record("BothCoordination", &[("body", ordinary.clone())]))
    );
    assert_ne!(
        nf(ordinary),
        nf(binary("Nominal", atom("a"), "And", atom("b")))
    );
}
#[test]
fn serial_imperatives_preserve_every_comma() {
    fn serial(cat: &str, helper: &str, members: [Tree; 3]) -> Tree {
        let [a, b, c] = members;
        record(
            "SerialCoordination",
            &[
                ("form", atom("0")),
                ("category", atom(cat)),
                ("left", a),
                (
                    "rest",
                    record(
                        "CoordinationSeriesEnd",
                        &[
                            ("form", atom("0")),
                            ("category", atom(helper)),
                            ("left", b),
                            ("coordinator", word("vocab:Coordinator/And")),
                            ("right", c),
                        ],
                    ),
                ),
            ],
        )
    }
    assert!(
        normalize(serial(
            "FocusedAdjunctCoordination",
            "FocusedAdjunctSeries",
            [atom("a"), atom("b"), atom("c")]
        ))
        .is_ok()
    );
    let vp = serial(
        "SecondaryVerbPhrase",
        "SecondaryPredicateSeries",
        [atom("a"), atom("b"), atom("c")],
    );
    let clauses = serial(
        "Clause",
        "ClauseSeries",
        [bare(atom("a")), bare(atom("b")), bare(atom("c"))],
    );
    assert_eq!(nf(bare(vp)), nf(clauses));
    assert_ne!(
        nf(clause(atom("a"), "And", atom("b"), true)),
        nf(clause(atom("a"), "And", atom("b"), false))
    );
}
#[test]
fn subjects_modals_quantifiers_dependents_and_gaps_do_not_distribute() {
    for name in [
        "FiniteClause",
        "ModalPredicate",
        "Quantifier",
        "WithAdjunct",
        "ObjectGap",
        "GapBinding",
        "Only",
    ] {
        let shared = wrapper(
            name,
            binary("SecondaryVerbPhrase", atom("a"), "And", atom("b")),
            atom("x"),
        );
        let first = binary(
            "SecondaryVerbPhrase",
            wrapper(name, atom("a"), atom("x")),
            "And",
            atom("b"),
        );
        assert_ne!(nf(bare(shared)), nf(bare(first)), "{name}");
    }
}
#[test]
fn lexical_form_variant_and_capitalization_are_exact() {
    let original = word("verb:draw");
    let mut alternatives = vec![word("verb:gain")];
    for (field, value) in [
        ("form", "Preterite"),
        ("variant", "1"),
        ("capitalization", "Initial"),
    ] {
        let mut changed = original.clone();
        let Tree::Record(_, outer) = &mut changed else { unreachable!() };
        let Tree::Call(_, values) = &mut outer[0].1 else { unreachable!() };
        let Tree::Record(_, fields) = &mut values[0] else { unreachable!() };
        fields.iter_mut().find(|(n, _)| n == field).unwrap().1 = atom(value);
        alternatives.push(changed);
    }
    for changed in alternatives {
        assert_ne!(
            nf(binary("Clause", original.clone(), "And", atom("b"))),
            nf(binary("Clause", changed, "And", atom("b")))
        );
    }
}
#[test]
fn attachment_heads_and_barrier_scope_stay_distinct_while_reordering_runs() {
    let pre = |head: &str| {
        format!(
            "PremodifiedNominal {{ form: 0, modifier: Adjective {{ form: 0, head: Word {{ value: Word(Word {{ lexeme: Ident(\"vocab:ColorWord/Red\") }}) }} }}, head: {head} }}"
        )
    };
    let post = |head: &str| {
        format!("PostmodifiedNominal {{ form: 0, head: {head}, modifier: WithFlying }}")
    };
    let chain_a = post(&pre("Creature"));
    let equivalent_a = pre(&post("Creature"));
    assert_eq!(encoded(&chain_a).unwrap(), encoded(&equivalent_a).unwrap());
    let on_creature = format!("Pair {{ left: {chain_a}, right: {} }}", pre("Artifact"));
    let on_artifact = format!(
        "Pair {{ left: {}, right: {} }}",
        pre("Creature"),
        post(&pre("Artifact"))
    );
    assert_ne!(
        encoded(&on_creature).unwrap(),
        encoded(&on_artifact).unwrap()
    );
    let wide = format!("Only {{ head: {chain_a} }}");
    let narrow = post(&format!("Only {{ head: {} }}", pre("Creature")));
    assert_ne!(encoded(&wide).unwrap(), encoded(&narrow).unwrap());
}
#[test]
fn recognized_shape_changes_fail_closed() {
    let mut malformed = binary("NounPhrase", atom("a"), "And", atom("b"));
    let Tree::Record(_, fields) = &mut malformed else { unreachable!() };
    fields.push(("extra".into(), atom("scope")));
    assert!(normalize(malformed).is_err());
    assert!(normalize(binary("NounPhrase", atom("a"), "Unknown", atom("b"))).is_err());
}

#[test]
fn real_bare_imperative_chains_have_one_normal_form() {
    use crate::english_v3::tests::lexicon;
    use deckmaste_english_v3::grammar::{Category, Grammar, Value};
    use std::collections::BTreeSet;
    let grammar = Grammar::with_lexicon(lexicon());
    for text in [
        "Draw a card and gain 1 life.",
        "Draw a card and gain 1 life and create a token.",
        "Draw a card and gain 1 life and create a token and discard a card.",
        "Scry 2, then draw a card.",
    ] {
        let input = lexicon().analyze(text);
        let forest =
            deckmaste_english_v3::parse(&grammar, lexicon(), &input, &Category::Sentence).unwrap();
        let keys: BTreeSet<_> = forest
            .readings(&grammar)
            .map(|value| {
                let Value::Reading(reading) = value.unwrap() else {
                    panic!("Reading expected")
                };
                assert_eq!(reading.realize_with(&grammar, lexicon()).unwrap(), text);
                super::super::fingerprint(&reading).unwrap()
            })
            .collect();
        // The instruction list is one analysis; no assertion requires a
        // particular redundant tree to continue to be produced.
        assert_eq!(keys.len(), 1, "{text}");
    }
}

#[test]
fn attachment_stays_on_its_member_when_coordination_is_flattened() {
    let attached_a = wrapper("Postmodifier", atom("a"), atom("relative"));
    let attached_c = wrapper("Postmodifier", atom("c"), atom("relative"));
    let left = binary(
        "NounPhrase",
        binary("NounPhrase", attached_a.clone(), "And", atom("b")),
        "And",
        atom("c"),
    );
    let same_head = binary(
        "NounPhrase",
        attached_a,
        "And",
        binary("NounPhrase", atom("b"), "And", atom("c")),
    );
    let different_head = binary(
        "NounPhrase",
        atom("a"),
        "And",
        binary("NounPhrase", atom("b"), "And", attached_c),
    );
    assert_eq!(nf(left), nf(same_head.clone()));
    assert_ne!(nf(same_head), nf(different_head));
}

#[test]
fn asyndetic_premodifiers_are_not_coordinator_ending_series() {
    let make = |left, rest| {
        record(
            "SerialCoordination",
            &[
                ("form", atom("0")),
                ("category", atom("PremodifierCoordination")),
                ("left", left),
                ("rest", rest),
            ],
        )
    };
    let tree = make(atom("noncreature"), atom("nonland"));
    assert_eq!(nf(tree.clone()), tree);
    assert_ne!(
        nf(make(tree, atom("nonenchantment"))),
        nf(make(
            atom("noncreature"),
            make(atom("nonland"), atom("nonenchantment"))
        ))
    );
}
