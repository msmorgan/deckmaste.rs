//! The unified frame engine, proven end to end on the seed lexicon: active,
//! passive, extraction across verb forms, and Right Node Raising, all derived
//! from active frames.

mod seed;

use deckmaste_lexical::Lexicon;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::Gap;
use deckmaste_syntax::Reading;
use deckmaste_syntax::ReadingNode;

struct Seed {
    lexicon: Lexicon,
    grammar: CompiledGrammar,
}

fn seed() -> Seed {
    let lexicon = seed::lexicon();
    let mut specification = deckmaste_english_v4::grammar().unwrap();
    // These independent Reading builders exercise Sentence, below Document.
    specification.root = "Sentence".into();
    let grammar = CompiledGrammar::bind(&specification, &lexicon, &seed::frames()).unwrap();
    Seed { lexicon, grammar }
}

/// One Constituent of a Reading, seen from outside: its Category, surface,
/// Gap, and the surfaces of its children.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Constituent {
    category: dpsi::Ident,
    surface: String,
    gap: Gap,
    production: dpsi::Ident,
    children: Vec<String>,
}

impl Seed {
    /// Every Reading of `text`, each checked for Admission and round-trip law
    /// 1.
    fn readings(&self, text: &str) -> Vec<Reading> {
        let parse = self.grammar.parse(&self.lexicon, text);
        assert!(
            parse.vocabulary_gaps.is_empty(),
            "{text}: {:?}",
            parse.vocabulary_gaps
        );
        let readings = parse.readings();
        for reading in &readings {
            assert_eq!(self.grammar.admit(reading, &self.lexicon), Ok(()));
            assert_eq!(self.grammar.realize(reading, &self.lexicon).unwrap(), text);
        }
        readings
    }

    fn surface(&self, reading: &Reading) -> String {
        self.grammar.realize(reading, &self.lexicon).unwrap()
    }

    fn constituents(&self, reading: &Reading) -> Vec<Constituent> {
        let mut out = Vec::new();
        self.walk(reading, &mut out);
        out
    }

    fn walk(&self, reading: &Reading, out: &mut Vec<Constituent>) {
        if let Reading::Node(ReadingNode {
            category,
            gap,
            production,
            children,
            ..
        }) = reading
        {
            out.push(Constituent {
                category: *category,
                surface: self.surface(reading),
                gap: *gap,
                production: *production,
                children: children.iter().map(|c| self.surface(c)).collect(),
            });
            for child in children {
                self.walk(child, out);
            }
        }
    }

    fn has(&self, reading: &Reading, category: &str, surface: &str) -> bool {
        self.constituents(reading)
            .iter()
            .any(|c| c.category == category && c.surface == surface)
    }

    /// The surfaces of the hosts the unselected phrase `adjunct` attaches to.
    fn hosts(&self, reading: &Reading, adjunct: &str) -> Vec<String> {
        self.constituents(reading)
            .into_iter()
            .filter(|c| c.production.starts_with("Adjunct:") && c.children[1] == adjunct)
            .map(|c| c.children[0].clone())
            .collect()
    }

    /// Law 2: a Reading rebuilt independently of the parse is admitted and is
    /// a Reading of the parse of its realization.
    fn reparses(&self, reading: &Reading) {
        let rebuilt: Reading = ron::from_str(&ron::to_string(reading).unwrap()).unwrap();
        assert_eq!(self.grammar.admit(&rebuilt, &self.lexicon), Ok(()));
        let surface = self.surface(&rebuilt);
        assert!(self.readings(&surface).contains(&rebuilt), "{surface}");
    }
}

#[test]
fn active_monotransitive() {
    let s = seed();
    let readings = s.readings("Destroy target creature.");
    assert_eq!(readings.len(), 1);
    assert!(s.has(&readings[0], "NounPhrase", "target creature"));
    s.reparses(&readings[0]);
}

#[test]
fn active_complement_from_a_frame() {
    let s = seed();
    let readings = s.readings("Put the card into your hand.");
    assert_eq!(readings.len(), 1);
    let vp = s
        .constituents(&readings[0])
        .into_iter()
        .find(|c| c.category == "VerbPhrase")
        .unwrap();
    assert_eq!(vp.children, ["Put", "the card", "into your hand"]);
}

#[test]
fn same_head_complement_priority() {
    let s = seed();
    let readings = s.readings("Search your library for a card.");
    assert_eq!(readings.len(), 1, "the complement reading only");
    let vp = s
        .constituents(&readings[0])
        .into_iter()
        .find(|c| c.category == "VerbPhrase")
        .unwrap();
    assert_eq!(vp.children, ["Search", "your library", "for a card"]);
    assert_eq!(s.hosts(&readings[0], "for a card"), Vec::<String>::new());
}

#[test]
fn keyword_with_phrase_modifies_the_nominal_only() {
    let s = seed();
    let readings = s.readings("Destroy target creature with flying.");
    assert_eq!(readings.len(), 1);
    assert_eq!(s.hosts(&readings[0], "with flying"), ["creature"]);
}

#[test]
fn cross_head_with_attachment_is_retained_and_ranked() {
    let s = seed();
    let readings = s.readings("Exile target creature with a counter on it.");
    // Readings in which the whole `with` phrase is one unselected Constituent;
    // others attach `on it` on its own, beside `with a counter`.
    let sites: Vec<Vec<String>> = readings
        .iter()
        .map(|r| s.hosts(r, "with a counter on it"))
        .filter(|hosts| !hosts.is_empty())
        .collect();
    assert_eq!(
        sites,
        [
            vec!["creature".to_owned()],
            vec!["Exile target creature".to_owned()]
        ],
        "both hosts retained, the nominal postmodifier ranked first"
    );
}

#[test]
fn seedborn_muse_keeps_both_temporal_attachments() {
    let s = seed();
    let text = "Untap all permanents you control during each other player's untap step.";
    let during = "during each other player's untap step";
    // Exactly one Reading per attachment site. Every Reading keeps `during ...`
    // whole: an Oracle possessor is never postmodified, so
    // `[all permanents you control during each other player]'s` is not a
    // possessor.
    let mut sites: Vec<Vec<String>> = s
        .readings(text)
        .iter()
        .map(|r| s.hosts(r, during))
        .collect();
    sites.sort();
    assert_eq!(
        sites,
        [
            vec!["Untap all permanents you control".to_owned()],
            vec!["control".to_owned()]
        ]
    );
}

#[test]
fn intransitive_and_copular() {
    let s = seed();
    assert_eq!(s.readings("Creatures attack.").len(), 1);
    let copular = s.readings("The card is a creature.");
    assert_eq!(copular.len(), 1);
    assert!(s.has(&copular[0], "Predicative", "a creature"));
    assert_eq!(
        s.readings("The cards is a creature."),
        Vec::<Reading>::new()
    );
}

#[test]
fn passives_derive_from_active_frames() {
    let s = seed();
    let finite = s.readings("The card is put into your hand.");
    assert_eq!(finite.len(), 1);
    assert!(s.has(&finite[0], "VerbPhrase", "put into your hand"));
    s.reparses(&finite[0]);
    // The verbal passive and the Adjectival Passive (settled analysis).
    let agent = s.readings("The creature is destroyed by the card.");
    assert_eq!(agent.len(), 2);
    let vp = agent
        .iter()
        .flat_map(|r| s.constituents(r))
        .find(|c| c.category == "VerbPhrase" && c.surface == "destroyed by the card")
        .unwrap();
    assert_eq!(vp.children, ["destroyed", "by the card"]);
    let reduced = s.readings("Exile each card put into your hand.");
    assert_eq!(reduced.len(), 1);
    assert!(s.has(&reduced[0], "Nominal", "card put into your hand"));
}

/// The relative clause in `text` has an object Gap and modifies `head`.
fn assert_gap(s: &Seed, text: &str, nominal: &str, gapped: &str) {
    let readings = s.readings(text);
    assert!(!readings.is_empty(), "{text}");
    for reading in &readings {
        let constituents = s.constituents(reading);
        let relative = constituents
            .iter()
            .find(|c| c.category == "Nominal" && c.surface == nominal)
            .unwrap_or_else(|| panic!("{text}: no nominal {nominal:?}"));
        assert_eq!(relative.production, "NominalPostmodifier");
        assert!(
            constituents
                .iter()
                .any(|c| c.category == "VerbPhrase" && c.surface == gapped && c.gap == Gap::NP),
            "{text}: {gapped:?} has the Gap"
        );
    }
    s.reparses(&readings[0]);
}

#[test]
fn extraction_across_verb_forms() {
    let s = seed();
    assert_gap(
        &s,
        "Exile the card you put into your hand.",
        "card you put into your hand",
        "put into your hand",
    );
    assert_gap(
        &s,
        "Exile the card you are putting into your hand.",
        "card you are putting into your hand",
        "are putting into your hand",
    );
    assert_gap(
        &s,
        "Exile the card you have put into your hand.",
        "card you have put into your hand",
        "have put into your hand",
    );
    // A stranded preposition, in the active and in the passive.
    assert_gap(
        &s,
        "Exile the hand you put the card into.",
        "hand you put the card into",
        "put the card into",
    );
    assert_gap(
        &s,
        "Exile the hand the card is put into.",
        "hand the card is put into",
        "is put into",
    );
}

#[test]
fn right_node_raising_shares_a_complement() {
    let s = seed();
    let finite = s.readings("Each creature enters or leaves the battlefield.");
    assert_eq!(finite.len(), 1);
    let shared = s
        .constituents(&finite[0])
        .into_iter()
        .find(|c| c.surface == "enters or leaves the battlefield")
        .unwrap();
    assert_eq!(shared.children, ["enters or leaves", "the battlefield"]);
    s.reparses(&finite[0]);
    let imperative = s.readings("Put into your hand or exile the card.");
    assert_eq!(imperative.len(), 1);
    assert!(s.has(&imperative[0], "VerbPhrase", "Put into your hand or exile"));
    // A focused Object may follow the selected PP without coordination.
    let shifted = s.readings("Put into your hand the card.");
    assert_ne!(shifted, [] as [Reading; 0]);
    assert!(s.has(&shifted[0], "NounPhrase", "the card"));
}

#[test]
fn ungrammatical_controls() {
    let s = seed();
    for text in [
        "Put the card.",
        "Destroy creature target.",
        "Search your library for.",
        "The card you put the card into your hand.",
        "Each creature enter or leaves the battlefield.",
    ] {
        assert!(s.readings(text).is_empty(), "{text}");
    }
}

#[test]
fn grammar_names_no_open_class_or_catalog_word() {
    let text = deckmaste_english_v4::GRAMMAR.to_lowercase();
    for lemma in seed::OPEN_CLASS {
        assert!(!text.contains(&format!("\"{lemma}\"")), "{lemma}");
        assert!(!text.contains(&format!(":{lemma}\"")), "{lemma}");
    }
    // A closed word pointing at a keyword action is a binding error.
    let grammar = deckmaste_english_v4::GRAMMAR.replace("\"verb:have\"", "\"verb:destroy\"");
    let bound = deckmaste_syntax::Grammar::from_ron_str(&grammar)
        .and_then(|g| CompiledGrammar::bind(&g, &seed::lexicon(), &seed::frames()));
    assert!(bound.is_err());
}

#[test]
fn chart_and_forest_sizes() {
    let s = seed();
    println!(
        "{:<75} {:>6} {:>6} {:>6} {:>4}",
        "sentence", "items", "nodes", "fams", "read"
    );
    for text in [
        "Destroy target creature.",
        "Put the card into your hand.",
        "Search your library for a card.",
        "Exile target creature with a counter on it.",
        "Untap all permanents you control during each other player's untap step.",
        "Exile the card you have put into your hand.",
        "Put into your hand or exile the card.",
    ] {
        let parse = s.grammar.parse(&s.lexicon, text);
        let m = parse.forest.metrics();
        println!(
            "{text:<75} {:>6} {:>6} {:>6} {:>4}",
            m.chart_items,
            m.completed_nodes,
            m.families,
            parse.readings().len()
        );
    }
}

fn node(reading: &Reading) -> &ReadingNode {
    reading.node().expect("a node")
}

#[test]
fn normal_form_identifies_same_head_complement_priority() {
    use deckmaste_syntax::frames::Frame;
    use deckmaste_syntax::frames::FrameFamily;
    use deckmaste_syntax::frames::FrameSlot;
    use deckmaste_syntax::frames::SlotFill;
    use deckmaste_syntax::frames::SlotRelation;
    let s = seed();
    let complement = s.readings("Search your library for a card.").remove(0);
    let sentence = node(&complement);
    let clause = node(&sentence.children[0]);
    let vp = node(&clause.children[0]);
    // Hand-build the analysis same-head priority excludes: the monotransitive
    // frame with `for a card` as an unselected predicate adjunct.
    let monotransitive = ReadingNode {
        production: dpsi::Ident::new("Valence:VerbPhrase/Monotransitive(NP)"),
        selected_frame: Some(Frame {
            family: FrameFamily::Monotransitive,
            slots: vec![FrameSlot {
                relation: SlotRelation::Object,
                fill: SlotFill::NP,
            }],
        }),
        children: vp.children[..2].to_vec(),
        ..vp.clone()
    };
    let adjunct = ReadingNode {
        production: dpsi::Ident::new("Adjunct:VerbPhrase+PrepositionPhrase/After/Predicate"),
        selected_frame: None,
        children: vec![Reading::Node(monotransitive), vp.children[2].clone()],
        ..vp.clone()
    };
    let excluded = Reading::Node(ReadingNode {
        children: vec![Reading::Node(ReadingNode {
            children: vec![Reading::Node(adjunct)],
            ..clause.clone()
        })],
        ..sentence.clone()
    });
    assert!(s.grammar.admit(&excluded, &s.lexicon).is_err());
    assert_eq!(
        s.grammar.normal_form(&excluded),
        s.grammar.normal_form(&complement)
    );
}

#[test]
fn normal_form_identifies_groupings_of_one_coordinator() {
    let s = seed();
    let serial = s
        .readings("Destroy target creature, exile target creature, or untap target creature.")
        .remove(0);
    let binary = s
        .readings("Destroy target creature or exile target creature.")
        .remove(0);
    // Hand-build `[[Destroy … or exile …] or untap …]` from the parsed parts.
    let clause = node(&node(&serial).children[0]);
    let top = node(&clause.children[0]);
    let tail = node(&top.children[1]);
    let nested = ReadingNode {
        production: dpsi::Ident::new("Coordination:VerbPhrase/binary"),
        children: vec![
            node(&node(&binary).children[0]).children[0].clone(),
            tail.children[1].clone(),
            tail.children[2].clone(),
        ],
        ..top.clone()
    };
    let nested = Reading::Node(ReadingNode {
        children: vec![Reading::Node(ReadingNode {
            children: vec![Reading::Node(nested)],
            ..clause.clone()
        })],
        ..node(&serial).clone()
    });
    assert!(
        s.grammar.admit(&nested, &s.lexicon).is_err(),
        "one tree per chain"
    );
    let normal = s.grammar.normal_form(&serial);
    assert_eq!(s.grammar.normal_form(&nested), normal);
    let chain = node(&node(&node(&normal).children[0]).children[0]);
    assert_eq!(
        chain.children.len(),
        4,
        "three conjuncts and the coordinator"
    );
}
