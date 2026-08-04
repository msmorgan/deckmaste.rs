#![allow(
    unused_imports,
    reason = "the macro under test emits compile_error! instead of code, so the codec imports go unused by design"
)]

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FixturePhrase;
use deckmaste_features::Comma;
use deckmaste_features::Conjunction;

deckmaste_constructions_macro::constructions! {
    group fixture_coordination;

    element fixture_member {
        comma: opt lex Comma,
        phrase: hole FixturePhrase,
    }

    construction fixture_pair: FixturePair {
        own FixturePairNode {
            members: seq fixture_member,
            conjunction: lex Conjunction,
        }
        require members.len() >= 2;
        require conjunction in [And, Or];
        witness oxford = stored members.last.comma;
        form plain @ 0 when conjunction in [And] = members lex(conjunction);
        form fancy @ 0 when conjunction in [Or] = members "," lex(conjunction);
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
}

fn main() {}
