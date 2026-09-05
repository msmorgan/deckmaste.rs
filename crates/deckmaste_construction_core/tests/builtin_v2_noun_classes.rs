use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::NounClassSemantics;
use deckmaste_construction_core::macro_def::NounLocativeTemporalLicense;
use deckmaste_construction_core::macro_def::NounRelationality;
use deckmaste_construction_core::macro_def::read_builtin_v2;

#[test]
fn every_builtin_noun_declaration_inherits_its_class_semantics() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    let mut census = BTreeMap::new();

    for declaration in &declarations {
        let expected = match declaration.identity().kind() {
            DeclarationKind::Type | DeclarationKind::Subtype(_) => Some(NounClassSemantics {
                locative_temporal_license: NounLocativeTemporalLicense::ObjectAttachmentLicensed,
                relationality: NounRelationality::QualifiedRelational,
            }),
            DeclarationKind::TurnPart => Some(NounClassSemantics {
                locative_temporal_license: NounLocativeTemporalLicense::TemporalLicensed,
                relationality: NounRelationality::Relational,
            }),
            DeclarationKind::CounterKind => Some(NounClassSemantics {
                locative_temporal_license: NounLocativeTemporalLicense::OnLicensed,
                relationality: NounRelationality::NonRelational,
            }),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(declaration.noun_class(), Some(expected));
            *census
                .entry(match declaration.identity().kind() {
                    DeclarationKind::Type => "type",
                    DeclarationKind::Subtype(_) => "subtype",
                    DeclarationKind::TurnPart => "turn part",
                    DeclarationKind::CounterKind => "counter kind",
                    _ => unreachable!(),
                })
                .or_insert(0usize) += 1;
        }
    }

    assert_eq!(
        census,
        BTreeMap::from([
            ("counter kind", 71),
            ("subtype", 462),
            ("turn part", 19),
            ("type", 10),
        ])
    );
}
