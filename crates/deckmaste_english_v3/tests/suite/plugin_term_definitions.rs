use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Category as LexicalCategory;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;

use crate::common;

#[test]
fn plugin_terms_retain_their_definition_owners_and_categories() {
    for (owner, suffix, family, name, category, kind) in [
        (
            "predicate",
            "",
            "predicates",
            "modified",
            LexicalCategory::Adjective,
            SourceKind::Plugin,
        ),
        (
            "predicate",
            "",
            "predicates",
            "poisoned",
            LexicalCategory::Adjective,
            SourceKind::Plugin,
        ),
        (
            "event",
            "/verb",
            "events",
            "expend",
            LexicalCategory::Verb,
            SourceKind::Plugin,
        ),
        (
            "event",
            "/verb",
            "events",
            "descend",
            LexicalCategory::Verb,
            SourceKind::Plugin,
        ),
        (
            "keyword_ability",
            "",
            "keyword_abilities",
            "multikicker",
            LexicalCategory::Keyword,
            SourceKind::Keyword,
        ),
        (
            "keyword_ability",
            "/verb",
            "keyword_abilities",
            "convoke",
            LexicalCategory::Verb,
            SourceKind::Plugin,
        ),
        (
            "keyword_ability",
            "/verb",
            "keyword_abilities",
            "devour",
            LexicalCategory::Verb,
            SourceKind::Plugin,
        ),
        (
            "keyword_ability",
            "/verb",
            "keyword_abilities",
            "evolve",
            LexicalCategory::Verb,
            SourceKind::Plugin,
        ),
    ] {
        let owner = format!("lexeme:{owner}/{name}");
        let id = format!("{owner}{suffix}");
        let lexeme = &common::lexicon().lexemes()[id.as_str()];
        assert_eq!(lexeme.category, category, "{id}");
        assert_eq!(lexeme.source.kind, kind, "{id}");
        assert_eq!(lexeme.source.owner, owner, "{id}");
        assert_eq!(
            lexeme.source.path,
            format!("plugins_v2/builtin/macros/{family}/{name}.ron")
        );
    }
}

#[test]
fn attested_plugin_terms_fill_their_declared_roles() {
    // Oracle units: Envoy of the Ancestors, Skyward Spider, Corrupted Resolve,
    // Wolfbriar Elemental, Junkblade Bruiser, Enterprising Scallywag,
    // Zephyr Singer, Hellkite Hatchling, and Watchful Radstag.
    for (text, role, surface, owner) in [
        (
            "Modified creatures you control have lifelink.",
            Category::AdjectivePhrase,
            "Modified",
            "lexeme:predicate/modified",
        ),
        (
            "This creature has flying as long as it's modified.",
            Category::AdjectivePhrase,
            "modified",
            "lexeme:predicate/modified",
        ),
        (
            "Counter target spell if its controller is poisoned.",
            Category::AdjectivePhrase,
            "poisoned",
            "lexeme:predicate/poisoned",
        ),
        (
            "Multikicker {1}{G}",
            Category::KeywordPhrase,
            "Multikicker {1}{G}",
            "lexeme:keyword_ability/multikicker",
        ),
        (
            "Whenever you expend 4, this creature gets +2/+1 until end of turn.",
            Category::FinitePredicate,
            "expend 4",
            "lexeme:event/expend/verb",
        ),
        (
            "At the beginning of your end step, if you descended this turn, create a Treasure token.",
            Category::FinitePredicate,
            "descended",
            "lexeme:event/descend/verb",
        ),
        (
            "When this creature enters, put a flying counter on each creature that convoked it.",
            Category::FinitePredicate,
            "convoked it",
            "lexeme:keyword_ability/convoke/verb",
        ),
        (
            "This creature has flying and trample if it devoured a creature.",
            Category::FinitePredicate,
            "devoured a creature",
            "lexeme:keyword_ability/devour/verb",
        ),
        (
            "Whenever this creature evolves, create a token that's a copy of it.",
            Category::FinitePredicate,
            "evolves",
            "lexeme:keyword_ability/evolve/verb",
        ),
    ] {
        common::assert_constituents(text, Category::Document, &[(role, surface)]);
        for reading in common::readings(text, Category::Document) {
            let mut selected = Vec::new();
            reading
                .visit(&mut |node| {
                    let (Reading::Adjective { head, .. }
                    | Reading::SelectedPredicate { head, .. }
                    | Reading::FiniteObjectGap { head, .. }
                    | Reading::CostKeyword { head, .. }) = node
                    else {
                        return;
                    };
                    if let LexicalReading::Word(value) = &head.value
                        && value.lexeme == owner
                    {
                        assert_eq!(node.category(), role, "{text}");
                        selected.push(value.lexeme);
                    }
                })
                .unwrap();
            assert_eq!(
                selected,
                vec![deckmaste_lexical::LexemeId::new(owner)],
                "{text}"
            );
        }
    }
}

#[test]
fn independently_constructed_predicate_adjectives_preserve_both_roundtrip_laws() {
    // The predicative constituents on Skyward Spider and Corrupted Resolve.
    for name in ["modified", "poisoned"] {
        let owner = format!("lexeme:predicate/{name}");
        let value = common::lexicon()
            .values()
            .find(|value| {
                value.lexeme == owner.as_str() && value.capitalization == SurfaceCase::Declared
            })
            .unwrap()
            .clone();
        let expected = Reading::Adjective {
            form: 0,
            head: Word {
                value: LexicalReading::Word(value),
                frame: None,
                countability: None,
            },
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), name);
        assert_eq!(
            common::readings(name, Category::AdjectivePhrase),
            BTreeSet::from([expected])
        );
    }
}
