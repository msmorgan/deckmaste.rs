use deckmaste_lexical::{
    Category, Countability, Frame, FrameItem, FrameSlot, Relation, Source, SourceKind, WordForm,
};

use crate::common::LEXICON;

#[test]
fn predicate_and_event_lexemes_retain_their_definition_owner_and_category() {
    for (owner, family, name, lemma, category) in [
        (
            "lexeme:predicate/historic",
            "predicates",
            "historic",
            "historic",
            Category::Adjective,
        ),
        (
            "lexeme:predicate/outlaw",
            "predicates",
            "outlaw",
            "outlaw",
            Category::Noun,
        ),
        (
            "lexeme:event/commitsCrime",
            "events",
            "commitsCrime",
            "crime",
            Category::Noun,
        ),
        (
            "lexeme:event/commitsCrime/verb",
            "events",
            "commitsCrime",
            "commit",
            Category::Verb,
        ),
    ] {
        let lexeme = &LEXICON.lexemes()[owner];
        assert_eq!(lexeme.category, category);
        assert_eq!(lexeme.lemma, lemma);
        assert_eq!(
            lexeme.source,
            Source {
                kind: SourceKind::Plugin,
                path: format!("plugins_v2/builtin/macros/{family}/{name}.ron"),
                owner: if category == Category::Verb {
                    "lexeme:event/commitsCrime".into()
                } else {
                    owner.into()
                },
            }
        );
    }
    let outlaw = &LEXICON.lexemes()["lexeme:predicate/outlaw"];
    assert_eq!(outlaw.properties.countability, vec![Countability::Count]);
    assert_eq!(outlaw.properties.features["NounPremodifier"], "Yes");
    let negative = LEXICON
        .lexemes()
        .values()
        .find(|lexeme| lexeme.lemma == "non-outlaw")
        .unwrap();
    assert_eq!(negative.category, Category::Noun);
    assert_eq!(negative.source, outlaw.source);
}

#[test]
fn commit_has_an_ordinary_object_frame_and_only_attested_inflectional_forms() {
    let verb = &LEXICON.lexemes()["lexeme:event/commitsCrime/verb"];
    assert_eq!(
        verb.properties.frames,
        vec![Frame {
            kind: "Predicate".into(),
            items: vec![FrameItem::Argument(FrameSlot {
                relation: Relation::Object,
                category: "NounPhrase".into()
            })],
        }]
    );
    let participle = verb
        .forms
        .iter()
        .find(|slot| slot.form == WordForm::PastParticiple)
        .unwrap();
    assert_eq!(participle.surfaces, Some(vec!["committed".into()]));
    assert!(
        verb.forms
            .iter()
            .all(|slot| ![WordForm::Preterite, WordForm::GerundParticiple].contains(&slot.form))
    );
}
