use std::path::Path;

use deckmaste_semantics_v2::abilities::{Ability, KeywordParam};
use deckmaste_semantics_v2::phrase::{NameSource, Predicate};
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::words::KeywordLabel;

#[test]
fn declared_quality_argument_preserves_the_printed_card_name() {
    let plugin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .unwrap();
    for name in ["Rory Williams", "Toothy, Imaginary Friend"] {
        let actual: Ability = plugin
            .macros
            .read_str(&format!("partnerWith(named(Printed({name:?})))"))
            .unwrap();
        assert_eq!(
            actual,
            Ability::Keyword {
                keyword: KeywordLabel::named("partnerWith"),
                params: vec![KeywordParam::Quality {
                    predicate: Predicate::Named {
                        source: Box::new(NameSource::Printed { name: name.into() }),
                    },
                }],
                body: vec![],
            }
        );
    }
}
