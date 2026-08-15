mod cards;
mod cr;
mod kind;
mod set;

pub use crate::kind::CatalogKind;
pub use crate::set::CatalogSet;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use crate::CatalogKind;
    use crate::CatalogSet;

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
701.1. Keyword action introduction\n\
701.2. Scry\n\
701.3. Tap and Untap\n\
702.1. Keyword ability introduction\n\
702.2. Daybound and Nightbound\n\
702.3. ∞ (Infinity)\n\
122.1b A keyword counter on a permanent or on a card in a zone other than the battlefield causes that object to gain that keyword. The keywords that a keyword counter can be are flying, first strike, double strike, deathtouch, and hexproof, as well as any variants of those keywords.\n";

    const ATOMIC_FIXTURE: &str = r#"{
        "data": {
            "Legal Name": [{
                "name": "Legal Name", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Legal"}
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Front", "side": "a", "layout": "modal_dfc",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Restricted"}
            }, {
                "name": "Front // Back", "faceName": "Back", "side": "b", "layout": "modal_dfc",
                "types": ["Land"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Restricted"}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Banned"}
            }],
            "Null": [{
                "name": "Null", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": null}
            }],
            "Missing": [{
                "name": "Missing", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {}
            }]
        }
    }"#;

    const REORDERED_ATOMIC_FIXTURE: &str = r#"{
        "data": {
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Back", "side": "b", "layout": "modal_dfc",
                "types": ["Land"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Restricted"}
            }, {
                "name": "Front // Back", "faceName": "Front", "side": "a", "layout": "modal_dfc",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Restricted"}
            }],
            "Duplicate": [{
                "name": "Legal Name", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Legal"}
            }],
            "Missing": [{
                "name": "Missing", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {}
            }],
            "Null": [{
                "name": "Null", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": null}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal",
                "types": [], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Banned"}
            }],
            "Legal Name": [{
                "name": "Legal Name", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "legalities": {"vintage": "Legal"}
            }]
        }
    }"#;

    #[test]
    fn canonical_sources_build_the_exact_inventory() {
        let catalogs = CatalogSet::generate(CR_FIXTURE, ATOMIC_FIXTURE.as_bytes()).unwrap();

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
            ATOMIC_FIXTURE.as_bytes(),
        )
        .unwrap_err();
        assert!(card_type_error.to_string().contains("card-types"));

        let counter_error = CatalogSet::generate(
            &CR_FIXTURE.replace(
                ", as well as any variants of those keywords",
                ", and its variants",
            ),
            ATOMIC_FIXTURE.as_bytes(),
        )
        .unwrap_err();
        assert!(counter_error.to_string().contains("counter-kind-phrases"));
    }

    #[test]
    fn card_names_are_canonical_across_atomic_source_order_and_duplicates() {
        let initial = CatalogSet::generate(CR_FIXTURE, ATOMIC_FIXTURE.as_bytes()).unwrap();
        let reordered =
            CatalogSet::generate(CR_FIXTURE, REORDERED_ATOMIC_FIXTURE.as_bytes()).unwrap();

        assert_eq!(initial, reordered);
    }

    #[test]
    fn composite_playable_name_without_a_face_name_is_rejected() {
        let atomic = ATOMIC_FIXTURE.replace("\"faceName\": \"Front\", ", "");
        let error = CatalogSet::generate(CR_FIXTURE, atomic.as_bytes()).unwrap_err();

        assert!(error.to_string().contains("card-names"));
    }

    #[test]
    fn from_entries_rejects_partial_catalog_inventories() {
        let mut entries = BTreeMap::new();
        entries.insert(CatalogKind::AbilityWords, BTreeSet::new());

        let error = CatalogSet::from_entries(entries).unwrap_err();
        assert!(error.to_string().contains("artifact-types"));
    }

    fn values(catalogs: &CatalogSet, kind: CatalogKind) -> Vec<&str> {
        catalogs.get(kind).iter().map(String::as_str).collect()
    }
}
