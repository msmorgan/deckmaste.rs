//! Construction-group declarations and test-only compiler scaffolding.

pub(crate) mod adjective;
pub(crate) mod attachment;
pub(crate) mod clause;
pub(crate) mod coordination;
pub(crate) mod determiner {
    pub(crate) use crate::syntax::determiner_constructions::*;

    #[cfg(test)]
    mod tests {
        #[test]
        fn production_declaration_exposes_all_nine_d01_rows() {
            assert_eq!(super::GROUPS[0].constructions.len(), 9);
            assert_eq!(super::GROUPS[0].constructions[0].id, "determiner_closed");
            assert_eq!(
                super::GROUPS[0].constructions[8].id,
                "possessive_noun_adjective"
            );
        }
    }
}
#[cfg(test)]
pub(crate) mod law;
pub(crate) use crate::syntax::nominal_constructions as nominal;
pub(crate) mod nonfinite;
pub(crate) mod noun;
pub(crate) mod noun_phrase;
pub(crate) mod predicate;
pub(crate) mod prepositional;
#[cfg(test)]
pub(crate) mod probe;
pub(crate) mod quantity;
pub(crate) mod relative;
pub(crate) mod sentence;

use deckmaste_construction_compiler::runtime::GroupData;

pub(crate) static GROUPS: &[&GroupData] = &[
    adjective::GROUPS[0],
    attachment::GROUPS[0],
    clause::GROUPS[0],
    coordination::GROUPS[0],
    determiner::GROUPS[0],
    noun::GROUPS[0],
    noun_phrase::GROUPS[0],
    nominal::GROUPS[0],
    nonfinite::GROUPS[0],
    predicate::GROUPS[0],
    prepositional::GROUPS[0],
    quantity::GROUPS[0],
    relative::GROUPS[0],
    sentence::GROUPS[0],
];

#[cfg(test)]
mod tests {
    use crate::construction::ConstructionBackend;
    use crate::construction::ConstructionId;
    use crate::construction::ConstructionOwner;

    const R01_IDS: [&str; 8] = [
        "relative_object",
        "relative_object_contracted_subject",
        "relative_subject_contracted_auxiliary",
        "relative_subject",
        "relative_subject_distributive_each",
        "relative_contracted_copular_noun",
        "relative_contracted_copular_adjective",
        "relative_contracted_copular_prepositional",
    ];

    #[test]
    fn r01_is_one_generated_group_in_stable_order_with_its_single_dominance_edge() {
        let group = super::GROUPS
            .iter()
            .copied()
            .find(|group| group.name == "relative")
            .expect("R01 has one production declaration group");
        assert_eq!(
            group
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            R01_IDS,
        );
        let edges = group
            .constructions
            .iter()
            .flat_map(|construction| {
                construction
                    .dominates
                    .iter()
                    .map(move |subordinate| (construction.id, *subordinate))
            })
            .collect::<Vec<_>>();
        assert_eq!(edges, [("relative_subject", "relative_object")]);

        for id in R01_IDS {
            let family = crate::construction_family(ConstructionId::new(id))
                .unwrap_or_else(|| panic!("missing R01 family {id}"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{id}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{id}");
        }
        let c01 = crate::construction_family(ConstructionId::new(
            "relative_contracted_copular_coordinated_adjective",
        ))
        .expect("C01 coordinated relative remains registered");
        assert_eq!(c01.owner(), ConstructionOwner::Handwritten);
    }
}
