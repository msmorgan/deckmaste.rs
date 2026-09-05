// Checker columns not carried by the keyword-action declarations.
pub(super) const ACTION_OVERLAY: &[(&str, &str)] = &[
    (
        "Destroy",
        r#"{ participle := some "destroyed", dest := some .graveyard, agentRole := playerAgent, patientRole := fieldObject }"#,
    ),
    (
        "Sacrifice",
        r#"{ participle := some "sacrificed", dest := some .graveyard, agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩, feature := some .sacrificing, bounded := true }"#,
    ),
    (
        "Exile",
        r#"{ participle := some "exiled", dest := some .exile, agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }"#,
    ),
    (
        "Discard",
        r#"{ participle := some "discarded", dest := some .graveyard, agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, some .hand⟩ }"#,
    ),
    (
        "Mill",
        r#"{ participle := some "milled", dest := some .graveyard, agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, some .library⟩ }"#,
    ),
    ("Scry", r"{ stepwise := true, agentRole := playerAgent }"),
    ("Surveil", r"{ stepwise := true, agentRole := playerAgent }"),
    (
        "Tap",
        r#"{ participle := some "tapped", agentRole := playerAgent, patientRole := fieldObject, feature := some .tapping }"#,
    ),
    (
        "Untap",
        r#"{ participle := some "untapped", agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩, bounded := true }"#,
    ),
    (
        "Search",
        r"{ loci := [.battlefield, .graveyard, .exile, .hand, .library, .stack, .command], agentRole := playerAgent, feature := some .librarySearch, bounded := true }",
    ),
    (
        "Shuffle",
        r"{ loci := [.library], agentRole := playerAgent }",
    ),
    ("Proliferate", r"{ agentRole := playerAgent }"),
    ("The Ring Tempts You", r"{ }"),
    (
        "Transform",
        r"{ intransitive := true, agentRole := playerAgent, patientRole := fieldObject }",
    ),
    (
        "Convert",
        r"{ intransitive := true, agentRole := playerAgent, patientRole := fieldObject }",
    ),
    (
        "Meld",
        r"{ dest := some .battlefield, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }",
    ),
    (
        "Cast",
        r"{ agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], spellTypes⟩, true, some .stack⟩, counterfactual := some .object, rides := true, plays := true, bounded := true }",
    ),
    (
        "Play",
        r"{ agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], allTypes⟩, true, none⟩, counterfactual := some .object, rides := true, plays := true, bounded := true }",
    ),
    (
        "Counter",
        r"{ agentRole := ⟨none, true, some .stack⟩, patientRole := ⟨some ⟨.object, [.spell, .ability], spellTypes⟩, true, some .stack⟩, rides := true }",
    ),
    (
        "Activate",
        r"{ agentRole := playerAgent, patientRole := ⟨some ⟨.object, [.ability], []⟩, true, some .stack⟩, bounded := true }",
    ),
    (
        "Regenerate",
        r#"{ participle := some "regenerated", agentRole := ⟨none, true, none⟩, patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩, rides := true, bounded := true }"#,
    ),
    ("Vote", r"{ agentRole := playerAgent, bounded := true }"),
    ("Venture Into The Dungeon", r"{}"),
    ("Adapt", r"{}"),
    (
        "Airbend",
        r"{ dest := some .exile, agentRole := playerAgent }",
    ),
    ("Amass", r"{ agentRole := playerAgent }"),
    ("Assemble", r"{}"),
    ("Attach", r"{ agentRole := playerAgent }"),
    ("Behold", r"{ agentRole := playerAgent }"),
    ("Blight", r"{ agentRole := playerAgent }"),
    ("Bolster", r"{}"),
    ("Clash", r"{ agentRole := playerAgent }"),
    (
        "Cloak",
        r"{ dest := some .battlefield, agentRole := playerAgent }",
    ),
    (
        "Collect Evidence",
        r"{ dest := some .exile, agentRole := playerAgent }",
    ),
    ("Connive", r"{}"),
    (
        "Create",
        r"{ dest := some .battlefield, agentRole := playerAgent }",
    ),
    ("Detain", r"{}"),
    ("Discover", r"{ agentRole := playerAgent }"),
    ("Double", r"{}"),
    ("Earthbend", r"{ agentRole := playerAgent }"),
    ("Endure", r"{}"),
    ("Exchange", r"{ agentRole := playerAgent }"),
    ("Exert", r"{ agentRole := playerAgent }"),
    ("Explore", r"{}"),
    ("Face A Villainous Choice", r"{ agentRole := playerAgent }"),
    (
        "Fateseal",
        r"{ stepwise := true, agentRole := playerAgent, opponentsLibrary := true }",
    ),
    ("Fight", r"{}"),
    ("Forage", r"{ agentRole := playerAgent }"),
    ("Goad", r"{ agentRole := playerAgent }"),
    ("Harness", r"{ }"),
    ("Heal", r"{}"),
    (
        "Incubate",
        r"{ dest := some .battlefield, agentRole := playerAgent }",
    ),
    (
        "Investigate",
        r"{ dest := some .battlefield, agentRole := playerAgent }",
    ),
    ("Learn", r"{}"),
    (
        "Manifest",
        r"{ dest := some .battlefield, agentRole := playerAgent }",
    ),
    ("Manifest Dread", r"{ agentRole := playerAgent }"),
    ("Monstrosity", r"{ }"),
    ("Populate", r"{}"),
    ("Recruit", r"{ agentRole := playerAgent }"),
    ("Reveal", r"{ agentRole := playerAgent }"),
    ("Support", r"{}"),
    ("Suspect", r"{ agentRole := playerAgent }"),
    ("Time Travel", r"{}"),
    ("Triple", r"{}"),
    ("Waterbend", r"{ agentRole := playerAgent }"),
];
