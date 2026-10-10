use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::common;

const EVIL_TWIN: &str = "You may have this creature enter as a copy of any creature on the battlefield, except it has \"{U}{B}, {T}: Destroy target creature with the same name as this creature.\"";
const COPYCROOK: &str = "You may have this creature enter as a copy of any creature on the battlefield, except it has \"Whenever this creature attacks, it connives.\"";
const PHANTASMAL_IMAGE: &str = "You may have this creature enter as a copy of any creature on the battlefield, except it's an Illusion in addition to its other types and it has \"When this creature becomes the target of a spell or ability, sacrifice it.\"";
const MACHINE_GODS_EFFIGY: &str = "You may have this artifact enter as a copy of any creature on the battlefield, except it's an artifact and it has \"{T}: Add {U}.\"";

fn assert_enter_host(predicate: &Reading) {
    let head = match predicate {
        Reading::SelectedPredicate { head, .. }
        | Reading::DepictiveSelectedPredicativePredicate { head, .. } => head,
        Reading::PrepositionPredicate { head, .. } => return assert_enter_host(head),
        _ => panic!("exception must attach to the lexical enter predicate"),
    };
    let LexicalReading::Word(value) = &head.value else {
        panic!("enter predicate must have a lexical verb head");
    };
    assert_eq!(value.lexeme.as_ref(), "core-verb:Enter");
}

fn witness(card: &str, text: &str) {
    exception_scopes(card, text, &[]);
}

fn exception_scopes(card: &str, text: &str, additional_scopes: &[&str]) {
    let exception = text.split_once(", except ").unwrap().1;
    let exception = format!("except {exception}");
    let exception = exception.strip_suffix('.').unwrap_or(&exception);
    let expected: BTreeSet<_> = std::iter::once(exception)
        .chain(additional_scopes.iter().copied())
        .flat_map(|scope| {
            let values = common::readings_on_card(card, scope, Category::PrepositionPhrase);
            assert!(!values.is_empty(), "{card}: missing constituency {scope}");
            values
        })
        .collect();
    assert!(!expected.is_empty());
    let mut observed = BTreeSet::new();
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut attachments = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::LicensedMatrixComplementPredicate { head, modifier, .. } = node {
                    assert_enter_host(head);
                    assert_eq!(modifier.category(), Category::PrepositionPhrase);
                    attachments.push(modifier.as_ref().clone());
                }
                if let Reading::PostmodifiedNominal { modifier, .. } = node {
                    assert_ne!(modifier.realize(common::lexicon()).unwrap(), exception);
                }
            })
            .unwrap();
        let [attachment] = attachments.as_slice() else {
            panic!("each Reading must attach exactly one exception to enter");
        };
        observed.insert(attachment.clone());
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
    if additional_scopes.is_empty() {
        assert_eq!(observed, expected, "{card}: exception constituency");
    } else {
        // EX-R1 requires both values; other licensed Adjunct attachments remain.
        assert_eq!(
            observed
                .intersection(&expected)
                .cloned()
                .collect::<BTreeSet<_>>(),
            expected,
            "{card}: retain both exception constituencies"
        );
    }
}

#[test]
fn evil_twin() {
    witness("Evil Twin", EVIL_TWIN);
}

#[test]
fn copycrook() {
    witness("Copycrook", COPYCROOK);
}

#[test]
fn phantasmal_image() {
    // Orchestrator ruling EX-R1 (2026-10-10): retain both Coordination scopes.
    exception_scopes(
        "Phantasmal Image",
        PHANTASMAL_IMAGE,
        &["except it's an Illusion in addition to its other types"],
    );
}

#[test]
fn machine_gods_effigy() {
    // Orchestrator ruling EX-R1 (2026-10-10): retain both Coordination scopes.
    exception_scopes(
        "Machine God's Effigy",
        MACHINE_GODS_EFFIGY,
        &["except it's an artifact"],
    );
}

#[test]
fn finite_and_depictive_hosts_keep_the_exception_on_enter() {
    // Ticket EC1–EC2: the host licence survives each existing projection.
    for text in [
        "This creature enters as a copy of any creature on the battlefield, except it has flying.",
        "You may have this creature enter as a copy of any creature on the battlefield, except it has flying.",
        "You may have this creature enter as a copy of any creature on the battlefield, except it's an artifact.",
        "You may have this creature enter tapped as a copy of any creature on the battlefield, except it has flying.",
    ] {
        witness("Evil Twin", text);
    }
}

fn without_enter_licence() -> Lexicon {
    let mut entries: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    entries
        .iter_mut()
        .find(|entry| entry.id.as_str() == "core-verb:Enter")
        .unwrap()
        .properties
        .features
        .insert("MatrixComplementLicence".into(), "None".into());
    Lexicon::new(entries).unwrap()
}

#[test]
fn controls_preserve_their_complete_reading_sets() {
    let previous = without_enter_licence();
    for text in [
        "You may have this creature enter as a copy of any creature on the battlefield.",
        "This creature enters as a copy of any creature on the battlefield.",
        "You may have this creature enter tapped as a copy of any creature on the battlefield.",
        "Create a token that's a copy of target creature, except it has flying.",
        "This creature becomes a copy of target creature, except it has flying.",
        "Create a token that's a copy of target creature, except it's an artifact and it has flying.",
        "This creature becomes a copy of target creature, except it's an artifact in addition to its other types.",
    ] {
        let expected = common::readings_with_lexicon(&previous, text, Category::Document);
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(
            common::readings(text, Category::Document),
            expected,
            "{text}"
        );
    }
}

#[test]
fn exception_requires_the_declared_enter_licence() {
    let unlicensed = without_enter_licence();
    for text in [EVIL_TWIN, COPYCROOK, PHANTASMAL_IMAGE, MACHINE_GODS_EFFIGY] {
        assert!(common::readings_with_lexicon(&unlicensed, text, Category::Document).is_empty());
    }
}

#[test]
fn independently_composed_exception_roundtrips() {
    // Compose the existing constituents independently of parsing their combination.
    let head = common::readings(
        "enter as a copy of this creature",
        Category::SecondaryVerbPhrase,
    )
    .into_iter()
    .next()
    .unwrap();
    let modifier = common::readings("except it has flying", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let expected = Reading::LicensedMatrixComplementPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 1,
        head: Box::new(head),
        modifier: Box::new(modifier),
    };
    expected.admit(common::lexicon()).unwrap();
    let text = "enter as a copy of this creature, except it has flying";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::SecondaryVerbPhrase).contains(&expected));
}
