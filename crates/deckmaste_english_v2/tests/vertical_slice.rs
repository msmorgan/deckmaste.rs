use std::path::Path;

use deckmaste_catalogs::CatalogKind;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::catalogs::ParserCatalogs;

fn catalogs() -> ParserCatalogs {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    ParserCatalogs::load(&path).expect("canonical generated catalogs load")
}

#[test]
fn catalog_identity_rejects_a_spelling_absent_from_the_bound_catalog() {
    let catalogs = catalogs();
    assert!(CatalogIdentity::new(&catalogs, CatalogKind::CardTypes, "Creature").is_some());
    assert!(
        CatalogIdentity::new(&catalogs, CatalogKind::CardTypes, "Definitely Not A Type").is_none()
    );
}

#[test]
fn abbreviated_self_reference_requires_a_bound_abbreviation() {
    let full_only = SelfName::new("Nameless One", None::<String>).unwrap();
    assert!(SelfReferenceNp::new(full_only, SelfReferenceSpelling::Abbreviated).is_none());

    let zacama = SelfName::new("Zacama, Primal Calamity", Some("Zacama")).unwrap();
    assert!(SelfReferenceNp::new(zacama, SelfReferenceSpelling::Abbreviated).is_some());
}

#[test]
fn triggered_ability_rejects_any_effect_count_other_than_one() {
    let event = Clause::Event(EventClause {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::Connive(Connive),
    });
    assert!(Triggered::new(TriggerWord::Whenever, event.clone(), Vec::new()).is_none());
    let effect = Sentence::Imperative(Imperative {
        predicate: VerbPhrase::Connive(Connive),
    });
    assert!(Triggered::new(TriggerWord::Whenever, event, vec![effect.clone(), effect],).is_none());
}
