use deckmaste_lexical::Category;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

use crate::common::LEXICON;

fn argument(relation: Relation) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation,
        category: "NounPhrase".into(),
    })
}

#[test]
fn keyword_verbs_retain_definition_ownership_regular_forms_and_selected_frames() {
    for (keyword, bare, items) in [
        ("crew", "crew", vec![argument(Relation::Object)]),
        ("mutate", "mutate", vec![]),
        ("exploit", "exploit", vec![argument(Relation::Object)]),
        ("escape", "escape", vec![]),
        ("haunt", "haunt", vec![argument(Relation::Object)]),
        (
            "soulbond",
            "pair",
            vec![
                argument(Relation::Object),
                FrameItem::Marker {
                    vocabulary: "Preposition".into(),
                    member: "With".into(),
                },
                argument(Relation::Object),
            ],
        ),
    ] {
        let owner = format!("lexeme:keyword_ability/{keyword}");
        let id = format!("{owner}/verb");
        let verb = &LEXICON.lexemes()[id.as_str()];
        let source = Source {
            kind: SourceKind::Plugin,
            path: format!("plugins_v2/builtin/macros/keyword_abilities/{keyword}.ron"),
            owner: owner.clone(),
        };
        assert_eq!(verb.category, Category::Verb);
        assert_eq!(verb.lemma, bare);
        assert_eq!(verb.source, source);
        assert_eq!(verb.forms, Lexeme::verb(&id, bare, source).forms);
        let mut expected = vec![Frame {
            kind: "Predicate".into(),
            items,
        }];
        if keyword == "soulbond" {
            expected.push(Frame {
                kind: "Passive".into(),
                items: vec![FrameItem::Marked {
                    vocabulary: "Preposition".into(),
                    member: "With".into(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "NounPhrase".into(),
                    },
                }],
            });
        }
        assert_eq!(verb.properties.frames, expected);
        assert_eq!(
            LEXICON.lexemes()[owner.as_str()].category,
            Category::Keyword
        );
        assert_eq!(LEXICON.lexemes()[owner.as_str()].lemma, keyword);
    }
    assert_eq!(
        LEXICON.lexemes()["lexeme:keyword_ability/exploit/verb"]
            .properties
            .features["PastParticipialPremodifier"],
        "Yes"
    );
}
