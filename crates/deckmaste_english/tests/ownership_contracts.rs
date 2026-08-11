//! Structural gates for declaration-owned syntax construction.

#[test]
fn active_adapters_do_not_reconstruct_invariant_bearing_outputs() {
    let adapters = [
        (
            "coordination.rs",
            include_str!("../src/coordination.rs"),
            &[
                "AdjectivePhraseCoordination::from_declaration_parts",
                "CoordinatedAdjectivePhrase::from_declaration_parts",
            ][..],
        ),
        (
            "clause.rs",
            include_str!("../src/clause.rs"),
            &[
                "RestrictionCoordination::from_declaration_parts",
                "NounPhrase::from_demonstrative_declaration",
            ][..],
        ),
        (
            "noun_phrase.rs",
            include_str!("../src/noun_phrase.rs"),
            &[
                "CoordinatedNounPhrase::try_new",
                "NounPhrase::from_coordination_declaration",
            ][..],
        ),
        (
            "grammar/lowering.rs",
            include_str!("../src/grammar/lowering.rs"),
            &[
                "NounPhrase::from_pronoun_declaration",
                "NounPhrase::from_demonstrative_declaration",
                "NounInstance::unchecked_singular",
                "NounInstance::unchecked_plural",
                "NounInstance::unchecked_mass",
            ][..],
        ),
        (
            "grammar/parse_nonterminal.rs",
            include_str!("../src/grammar/parse_nonterminal.rs"),
            &["NounPhrase::from_nominal_declaration"][..],
        ),
        (
            "grammar/clause/lowering.rs",
            include_str!("../src/grammar/clause/lowering.rs"),
            &[
                "AdjectivePhraseCoordination::from_declaration_parts",
                "CoordinatedAdjectivePhrase::from_declaration_parts",
            ][..],
        ),
    ];

    for (path, source, forbidden) in adapters {
        for constructor in forbidden {
            assert!(
                !source.contains(constructor),
                "{path} bypasses its declaration owner through {constructor}",
            );
        }
    }
}
