mod cards;
mod cr;
mod diff;
mod io;
mod kind;
pub mod legacy;
mod set;

pub use crate::diff::DirectoryDiff;
pub use crate::diff::compare_directories;
pub use crate::kind::CatalogKind;
pub use crate::set::CatalogSet;
pub use crate::set::TypeLineParts;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use crate::CatalogKind;
    use crate::CatalogSet;

    // cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
    const CR_FIXTURE: &str = "\
205.2a The card types are artifact, creature, land, and sorcery.\n\
205.3g Artifacts have their own unique set of subtypes; these subtypes are called artifact types. The artifact types are Clue (see rule 111), and Vibranium.\n\
205.3h Enchantments have their own unique set of subtypes; these subtypes are called enchantment types. The enchantment types are Aura, and Saga.\n\
205.3i Lands have their own unique set of subtypes; these subtypes are called land types. The land types are Forest, Island, and Urza’s. Of that list, Forest is a basic land type.\n\
205.3j Planeswalkers have their own unique set of subtypes; these subtypes are called planeswalker types. The planeswalker types are Ajani, and Jace.\n\
205.3k Instants and sorceries share their lists of subtypes; these subtypes are called spell types. The spell types are Arcane, and Trap.\n\
205.3m Creatures and kindreds share their lists of subtypes; these subtypes are called creature types. One creature type is two words long: Time Lord. All other creature types are one word long: Advisor.\n\
205.3q Battles have a unique subtype, called a battle type. That battle type is Siege.\n\
205.4a An object can have one or more supertypes. A card’s supertypes are printed directly before its card types. The supertypes are basic, legendary, and snow.\n\
207.2c An ability word appears in italics at the beginning of some abilities. The ability words are landfall, and threshold.\n\
701. Keyword Actions\n\
701.1. Keyword action introduction\n\
701.2. Scry\n\
701.3. Tap and Untap\n\
702. Keyword Abilities\n\
702.1. Keyword ability introduction\n\
702.2. Daybound and Nightbound\n\
702.3. ∞ (Infinity)\n\
702.4a Landwalk is a generic term that appears within an object’s rules text as “[type]walk,” where [type] is usually a subtype.\n\
702.5a Typecycling is a variant of the cycling ability. “[Type]cycling [cost]” means “Pay [cost], Discard this card: Search your library for a [type] card, reveal it, put it into your hand, then shuffle.”\n\
702.6a Partner is a keyword ability that has several variants: partner with [name], choose a Background, and Doctor’s companion. “Partner—Friends forever” represents one such variant.\n\
702.7a Hexproof from is a variant of the hexproof ability.\n\
122.1b A keyword counter on a permanent or on a card in a zone other than the battlefield causes that object to gain that keyword. The keywords that a keyword counter can be are flying, first strike, double strike, deathtouch, and hexproof, as well as any variants of those keywords.\n";
    // cite: noncompliant end

    const ORACLE_FIXTURE: &str = concat!(
        r#"{"object":"card","id":"printing-legal","oracle_id":"oracle-legal","name":"Legal Name","layout":"normal","type_line":"Creature","keywords":["Islandwalk"],"legalities":{"vintage":"legal"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-faces","oracle_id":"oracle-faces","name":"Front // Back","layout":"modal_dfc","keywords":[],"legalities":{"vintage":"restricted"},"card_faces":[{"name":"Front","type_line":"Creature"},{"name":"Back","type_line":"Land"}]}"#,
        "\n",
        r#"{"object":"card","id":"printing-banned","oracle_id":"oracle-banned","name":"Banned","layout":"normal","keywords":[],"legalities":{"vintage":"banned"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-missing","oracle_id":"oracle-missing","name":"Missing","layout":"normal","keywords":[],"legalities":{}}"#,
        "\n",
    );

    const LEGACY_KEYWORD_FIXTURE: &str = concat!(
        r#"{"object":"card","id":"printing-legal","oracle_id":"oracle-legal","name":"Legal","layout":"normal","type_line":"Creature","keywords":["Islandwalk","Friends forever","Attack","Wind Walk"],"legalities":{"vintage":"legal"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-restricted","oracle_id":"oracle-restricted","name":"Restricted","layout":"normal","type_line":"Artifact","keywords":["Basic landcycling","Hexproof from","Choose a background"],"legalities":{"vintage":"restricted"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-banned","oracle_id":"oracle-banned","name":"Banned","layout":"normal","type_line":"Creature","keywords":["Forestwalk"],"legalities":{"vintage":"banned"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-missing","oracle_id":"oracle-missing","name":"Missing","layout":"normal","type_line":"Creature","keywords":["Swampwalk"],"legalities":{}}"#,
        "\n",
    );

    const REORDERED_ORACLE_FIXTURE: &str = concat!(
        r#"{"object":"card","id":"printing-missing","oracle_id":"oracle-missing","name":"Missing","layout":"normal","keywords":[],"legalities":{}}"#,
        "\n",
        r#"{"object":"card","id":"printing-faces","oracle_id":"oracle-faces","name":"Front // Back","layout":"modal_dfc","keywords":[],"legalities":{"vintage":"restricted"},"card_faces":[{"name":"Front","type_line":"Creature"},{"name":"Back","type_line":"Land"}]}"#,
        "\n",
        r#"{"object":"card","id":"printing-duplicate","oracle_id":"oracle-duplicate","name":"Legal Name","layout":"normal","type_line":"Creature","keywords":[],"legalities":{"vintage":"legal"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-banned","oracle_id":"oracle-banned","name":"Banned","layout":"normal","keywords":[],"legalities":{"vintage":"banned"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-legal","oracle_id":"oracle-legal","name":"Legal Name","layout":"normal","type_line":"Creature","keywords":[],"legalities":{"vintage":"legal"}}"#,
        "\n",
    );

    #[test]
    fn canonical_sources_build_the_exact_inventory() {
        let catalogs = CatalogSet::generate(CR_FIXTURE, ORACLE_FIXTURE.as_bytes()).unwrap();

        assert_eq!(
            catalogs.kinds().collect::<Vec<_>>(),
            CatalogKind::ALL.into_iter().collect::<Vec<_>>()
        );
        assert_eq!(
            values(&catalogs, CatalogKind::AbilityWords),
            ["Landfall", "Threshold"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::ArtifactTypes),
            ["Clue", "Vibranium"]
        );
        assert_eq!(values(&catalogs, CatalogKind::BattleTypes), ["Siege"]);
        assert_eq!(
            catalogs
                .get(CatalogKind::CardNames)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Back", "Front", "Legal Name"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::CardTypes),
            ["Artifact", "Creature", "Land", "Sorcery"]
        );
        assert_eq!(
            catalogs
                .get(CatalogKind::CounterKindPhrases)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Double strike", "First strike"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::CreatureTypes),
            ["Advisor", "Time Lord"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::EnchantmentTypes),
            ["Aura", "Saga"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::KeywordAbilities),
            ["Daybound", "Nightbound", "∞"]
        );
        for legacy_variant in [
            "Islandwalk",
            "Basic landcycling",
            "Friends forever",
            "Hexproof from",
            "Choose a Background",
        ] {
            assert!(
                !catalogs
                    .get(CatalogKind::KeywordAbilities)
                    .contains(legacy_variant)
            );
        }
        assert_eq!(
            values(&catalogs, CatalogKind::KeywordActions),
            ["Scry", "Tap", "Untap"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::LandTypes),
            ["Forest", "Island", "Urza's"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::PlaneswalkerTypes),
            ["Ajani", "Jace"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::SpellTypes),
            ["Arcane", "Trap"]
        );
        assert_eq!(
            values(&catalogs, CatalogKind::Supertypes),
            ["Basic", "Legendary", "Snow"]
        );
        assert!(
            !catalogs
                .get(CatalogKind::CardNames)
                .iter()
                .any(|name| name.contains(" // "))
        );
    }

    #[test]
    fn malformed_cr_lists_name_the_rejected_catalog() {
        let card_type_error = CatalogSet::generate(
            &CR_FIXTURE.replace("The card types are", "Card types include"),
            ORACLE_FIXTURE.as_bytes(),
        )
        .unwrap_err();
        assert!(card_type_error.to_string().contains("card-types"));

        let counter_error = CatalogSet::generate(
            &CR_FIXTURE.replace(
                ", as well as any variants of those keywords",
                ", and its variants",
            ),
            ORACLE_FIXTURE.as_bytes(),
        )
        .unwrap_err();
        assert!(counter_error.to_string().contains("counter-kind-phrases"));
    }

    #[test]
    fn malformed_keyword_heading_is_not_silently_omitted() {
        const ACTION_TWO: &str = concat!("701", ".2");
        let cr = CR_FIXTURE.replace(
            &format!("{ACTION_TWO}. Scry"),
            &format!("{ACTION_TWO} Scry"),
        );

        let error = CatalogSet::generate(&cr, ORACLE_FIXTURE.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("keyword-actions"));
        assert!(error.to_string().contains(&format!("{ACTION_TWO} Scry")));
    }

    #[test]
    fn keyword_section_title_lookalikes_are_rejected() {
        for (title, lookalike, catalog) in [
            ("701. Keyword Actions", "701. Scry", "keyword-actions"),
            ("702. Keyword Abilities", "702. Flying", "keyword-abilities"),
        ] {
            let cr = CR_FIXTURE.replace(title, lookalike);

            let error = CatalogSet::generate(&cr, ORACLE_FIXTURE.as_bytes()).unwrap_err();

            assert!(error.to_string().contains(catalog), "{error:#}");
            assert!(error.to_string().contains(lookalike), "{error:#}");
        }
    }

    #[test]
    fn duplicate_keyword_heading_numbers_are_rejected() {
        const ACTION_TWO: &str = concat!("701", ".2");
        const ACTION_THREE: &str = concat!("701", ".3");
        let cr = CR_FIXTURE.replace(
            &format!("{ACTION_THREE}. Tap and Untap"),
            &format!("{ACTION_TWO}. Tap and Untap"),
        );

        let error = CatalogSet::generate(&cr, ORACLE_FIXTURE.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("keyword-actions"));
        assert!(
            error
                .to_string()
                .contains(&format!("duplicate {ACTION_TWO} heading"))
        );
    }

    #[test]
    fn blank_cr_list_members_are_rejected() {
        let cr = CR_FIXTURE.replace("artifact, creature", "artifact, , creature");

        let error = CatalogSet::generate(&cr, ORACLE_FIXTURE.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("card-types"));
        assert!(error.to_string().contains("blank CR list member"));
    }

    #[test]
    fn card_names_are_canonical_across_source_order_and_duplicates() {
        let initial = CatalogSet::generate(CR_FIXTURE, ORACLE_FIXTURE.as_bytes()).unwrap();
        let reordered =
            CatalogSet::generate(CR_FIXTURE, REORDERED_ORACLE_FIXTURE.as_bytes()).unwrap();

        assert_eq!(initial, reordered);
    }

    #[test]
    fn composite_playable_name_without_a_face_name_is_rejected() {
        let oracle = ORACLE_FIXTURE.replace(
            "\"card_faces\":[{\"name\":\"Front\",",
            "\"card_faces\":[{\"name\":\"Front // Back\",",
        );
        let error = CatalogSet::generate(CR_FIXTURE, oracle.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("card-names"));
    }

    #[test]
    fn from_entries_rejects_partial_catalog_inventories() {
        let mut entries = BTreeMap::new();
        entries.insert(CatalogKind::AbilityWords, BTreeSet::new());

        let error = CatalogSet::from_entries(entries).unwrap_err();
        assert!(error.to_string().contains("artifact-types"));
    }

    #[test]
    fn canonical_keyword_abilities_exclude_all_legacy_card_variants() {
        let catalogs = CatalogSet::generate(CR_FIXTURE, LEGACY_KEYWORD_FIXTURE.as_bytes()).unwrap();

        for legacy_variant in [
            "Islandwalk",
            "Basic landcycling",
            "Friends forever",
            "Hexproof from",
            "Choose a Background",
        ] {
            assert!(
                !catalogs
                    .get(CatalogKind::KeywordAbilities)
                    .contains(legacy_variant)
            );
        }
    }

    fn values(catalogs: &CatalogSet, kind: CatalogKind) -> Vec<&str> {
        catalogs.get(kind).iter().map(String::as_str).collect()
    }
}
