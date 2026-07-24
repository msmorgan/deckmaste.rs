use deckmaste_english::CatalogKind;
use deckmaste_english::Catalogs;
use deckmaste_english::parse_with_catalogs;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::IndependentClause;
use deckmaste_english::syntax::Predicate;
use deckmaste_english::syntax::SentenceBody;

#[test]
fn public_parser_returns_a_source_independent_grammar_tree() {
    let source = String::from("Draw a card.");
    let report = parse_with_catalogs(&source, &Catalogs::default());

    assert!(report.diagnostics().is_empty());
    assert!(matches!(
        report.ast().abilities[0].kind,
        AbilityKind::Paragraph(ref paragraph)
            if matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Imperative(
                    Predicate::Transitive(_)
                ))
            )
    ));

    let ast = report.into_ast();
    drop(source);

    assert_eq!(ast.render("Test Card", false).unwrap(), "Draw a card.");
}

/// Parses `source` as the named face and returns its rendered round-trip and a
/// pretty-printed AST for structural assertions. The parse and render share the
/// identity, so a recognized self-reference re-emits the face's own name.
fn parse_face(source: &str, catalogs: &Catalogs, name: &str, legendary: bool) -> (String, String) {
    let report = parse_with_identity(source, catalogs, name, legendary);
    let debug = format!("{:#?}", report.ast());
    let rendered = report
        .into_ast()
        .render(name, legendary)
        .expect("named face should render");
    (rendered, debug)
}

// --- Self-reference recognition (positives) --------------------------------

#[test]
fn single_word_full_name_is_a_full_self_reference() {
    let (rendered, ast) = parse_face(
        "Progenitus attacks.",
        &Catalogs::default(),
        "Progenitus",
        true,
    );
    assert_eq!(rendered, "Progenitus attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("FullName"),
        "AST:\n{ast}"
    );
}

#[test]
fn a_nickname_is_an_abbreviated_self_reference() {
    let (rendered, ast) = parse_face(
        "Ashcoat attacks.",
        &Catalogs::default(),
        "Ashcoat of the Shadow Swarm",
        true,
    );
    assert_eq!(rendered, "Ashcoat attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

#[test]
fn a_two_word_nickname_is_recognized_as_one_self_reference() {
    let (rendered, ast) = parse_face(
        "Sidar Jabari attacks.",
        &Catalogs::default(),
        "Sidar Jabari of Zhalfir",
        true,
    );
    assert_eq!(rendered, "Sidar Jabari attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

#[test]
fn a_roman_numeral_nickname_is_recognized() {
    // `King Darien XLVIII` shortens to `King Darien` by dropping the numeral.
    let (rendered, ast) = parse_face(
        "King Darien attacks.",
        &Catalogs::default(),
        "King Darien XLVIII",
        true,
    );
    assert_eq!(rendered, "King Darien attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

// --- Self-reference must lose to the surrounding grammar (traps) ------------

#[test]
fn sliver_stays_a_creature_type_not_a_self_reference() {
    // `Sliver Queen` shortens to `Sliver`, but in a token's type line `Sliver`
    // is the catalog creature type, which is equally structural and wins the tie.
    let catalogs = Catalogs::default()
        .with_catalog(CatalogKind::CreatureType, ["Sliver"])
        .with_catalog(CatalogKind::CardType, ["Creature"]);
    let (rendered, ast) = parse_face(
        "Create a 1/1 colorless Sliver creature token.",
        &catalogs,
        "Sliver Queen",
        true,
    );
    assert_eq!(rendered, "Create a 1/1 colorless Sliver creature token.");
    assert!(
        !ast.contains("ThisCard"),
        "Sliver read as a self-reference:\n{ast}"
    );
}

#[test]
fn a_nickname_that_is_a_common_noun_keeps_its_capital() {
    // `carnage` is a lowercase vocabulary noun; the nickname `Carnage` must win
    // over that sentence-initial reading so its capital survives the round-trip.
    let (rendered, ast) = parse_face(
        "Carnage attacks.",
        &Catalogs::default(),
        "Carnage, Blood Artist",
        true,
    );
    assert_eq!(rendered, "Carnage attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

#[test]
fn a_nickname_that_is_a_keyword_ability_keeps_its_capital() {
    // A keyword-ability noun renders lowercase (`prowl`); the nickname `Prowl`
    // must win so the printed name keeps its capital.
    let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Prowl"]);
    let (rendered, ast) = parse_face("Prowl attacks.", &catalogs, "Prowl, Stoic Strategist", true);
    assert_eq!(rendered, "Prowl attacks.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

#[test]
fn a_the_headed_nickname_keeps_its_capital_the() {
    // `The Beast` also parses as the lowercase `the` determiner plus a `Beast`
    // creature type; the nickname must win so the capital `The` survives.
    let catalogs = Catalogs::default().with_catalog(CatalogKind::CreatureType, ["Beast"]);
    let (rendered, ast) = parse_face(
        "Whenever a creature dies, untap The Beast.",
        &catalogs,
        "The Beast, Deathless Prince",
        true,
    );
    assert_eq!(rendered, "Whenever a creature dies, untap The Beast.");
    assert!(
        ast.contains("ThisCard(") && ast.contains("AbbreviatedName"),
        "AST:\n{ast}"
    );
}

// --- Copy-effect exception riders [CR#707.9] -------------------------------

/// A legendary identity (nickname `Nissa`, full name `Nissa Revane`) plus the
/// catalog terms the exception conjuncts below name.
fn copy_catalogs() -> Catalogs {
    Catalogs::new(
        ["Flying"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    )
    .with_catalog(CatalogKind::CreatureType, ["Illusion"])
    .with_catalog(CatalogKind::CardType, ["Creature", "Permanent"])
    .with_catalog(CatalogKind::Supertype, ["Legendary", "Snow"])
}

const NISSA: &str = "Nissa Revane";

#[test]
fn single_conjunct_exception_rider_round_trips() {
    // An enter-as-copy host with one power/toughness copular exception conjunct.
    let source = "You may have this creature enter as a copy of any creature on \
                  the battlefield, except it's 7/7.";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("Exception(") && ast.contains("PowerToughness("),
        "AST:\n{ast}"
    );
}

#[test]
fn multi_conjunct_oxford_exception_rider_round_trips() {
    // Sakashima-shaped: a name literal, a `legendary in addition to its other
    // types` copular conjunct, and a quoted ability, joined as an Oxford list on
    // an enter-as-copy host whose causee is the face's own name.
    let source = "You may have Nissa Revane enter as a copy of any creature on \
                  the battlefield, except its name is Nissa Revane, it's legendary \
                  in addition to its other types, and it has \"{2}{U}{U}: Return \
                  Nissa Revane to its owner's hand at the beginning of the next \
                  end step.\"";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("Exception(")
            && ast.contains("ExceptionConjunct")
            && ast.contains("ThisCard(")
            && ast.contains("QuotedAbility("),
        "AST:\n{ast}"
    );
}

#[test]
fn quoted_ability_exception_on_a_token_copy_round_trips() {
    // A token-copy host with a single quoted-ability exception conjunct.
    let source = "Create a token that's a copy of target permanent, except the \
                  token has \"When this token enters, if it's a creature, it \
                  fights up to one target creature you don't control.\"";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("Exception(") && ast.contains("QuotedAbility("),
        "AST:\n{ast}"
    );
}

#[test]
fn name_exception_on_a_becomes_copy_carries_a_self_reference() {
    // A becomes-copy host whose exception names the copy as the face itself; the
    // name literal is the existing self-reference structure, not a new payload.
    let source = "This creature becomes a copy of target creature, except its \
                  name is Nissa Revane and it isn't legendary.";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("Exception(") && ast.contains("ThisCard(") && ast.contains("FullName"),
        "AST:\n{ast}"
    );
}

// --- Family C: predicative-adjective coordination ---------------------------

/// Catalogs naming the card types and supertypes the Family C witnesses use;
/// colors are built-in adjectives and need no catalog entry.
fn predicative_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(
            CatalogKind::CardType,
            ["Creature", "Permanent", "Planeswalker"],
        )
        .with_catalog(CatalogKind::Supertype, ["Legendary", "Snow"])
}

#[test]
fn contracted_relative_copular_coordinates_color_adjectives() {
    // Fry / Aether Gust shape: `that's <color> or <color>` on a coordinated
    // relative antecedent.
    let source = "Fry deals 5 damage to target creature or planeswalker that's white or blue.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Fry", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("CoordinatedAdjective("), "AST:\n{ast}");
}

#[test]
fn matrix_copular_coordinates_supertype_adjectives() {
    // Moritte shape: `it's legendary and snow …`. Supertypes scan as both
    // adjective and noun, so this also exercises the all-adjective gate keeping
    // the adjective reading.
    let source = "It's legendary and snow in addition to its other types.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("CoordinatedAdjective("), "AST:\n{ast}");
}

#[test]
fn noncontracted_relative_copular_coordinates_with_and_or() {
    // Glistening Deluge shape: `that are <color> and/or <color>` on the
    // intransitive-`be` verb-phrase path, admitting the `and/or` connective.
    let source = "Creatures that are green and/or white get -2/-2 until end of turn.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("CoordinatedAdjective("), "AST:\n{ast}");
}

#[test]
fn coordinated_noun_object_after_a_verb_is_not_a_predicative_adjective() {
    // The gate: a bare coordinated *noun* pair after a verb keeps its ordinary
    // coordinated-noun-object parse and never reduces as a coordinated adjective
    // complement.
    let source = "Exile target creature or planeswalker.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(!ast.contains("CoordinatedAdjective("), "AST:\n{ast}");
}

// --- Family A: base power and toughness -------------------------------------

fn characteristic_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn base_power_and_toughness_stat_sets_a_characteristic_pair() {
    // Candlekeep Inspiration shape: `have base power and toughness X/X, where X
    // is …`. The `power and toughness` pair rides the existing noun-phrase
    // coordination under the shared `base` modifier; the `X/X` value is a
    // power/toughness complement on the final characteristic.
    let source = "Creatures you control have base power and toughness X/X, where X is the \
                  number of cards in your graveyard.";
    let (rendered, ast) = parse_face(source, &characteristic_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("Coordinated(") && ast.contains("PowerToughness("),
        "AST:\n{ast}"
    );
}

#[test]
fn base_power_or_toughness_quantity_bound_rides_the_existing_quantity_complement() {
    // Angelic Aberration stretch shape: the `or` pair with a `1 or less`
    // quantity bound reuses the existing quantity complement — no new machinery.
    let source = "Creatures you control have base power or toughness 1 or less.";
    let (rendered, ast) = parse_face(source, &characteristic_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("Coordinated("), "AST:\n{ast}");
}

// --- Activation-cost round --------------------------------------------------

fn cost_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Artifact", "Creature"])
}

#[test]
fn labeled_activation_cost_peels_a_flavor_header() {
    // Boast/Forecast/ability-word shape: an arbitrary label before a spaced
    // em dash heads the activation cost; the label is licensed flavor-header
    // opacity and the remainder parses as an ordinary cost.
    let source = "Boast — {1}{R}: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("FlavorHeader"), "AST:\n{ast}");
}

#[test]
fn alternative_cost_components_join_with_or() {
    // A top-level `or` between cost payments becomes an Alternative component
    // rather than recovering verbatim.
    let source = "{T} or {W}: Add {G}.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("Alternative("), "AST:\n{ast}");
}

#[test]
fn an_or_inside_a_cost_clause_does_not_split_into_alternatives() {
    // The gate on the alternative split: it is tried only after the whole
    // component fails to parse, so a coordinated noun object inside a cost
    // clause keeps its ordinary parse.
    let source = "{T}, Sacrifice an artifact or creature: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(!ast.contains("Alternative("), "AST:\n{ast}");
}

#[test]
fn discard_at_random_parses_as_a_cost_clause() {
    // `at random` rides the prepositional path with `random` as a mass noun
    // (the idiom is historically `at` + noun).
    let source = "{T}, Discard a card at random: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
}

#[test]
fn cost_noun_phrases_coordinate_with_and_or() {
    // Keskit/Mechtitan shape: `and/or` now joins noun phrases (the cost
    // witnesses attest it); `then` still never does.
    let source = "Sacrifice two artifacts and/or creatures: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("AndOr"), "AST:\n{ast}");
}

// --- R30: the library-iteration cluster -------------------------------------

/// Card-type and creature-type atoms the library-iteration witnesses name.
fn library_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(CatalogKind::CardType, ["Creature", "Land"])
        .with_catalog(CatalogKind::CreatureType, ["Creature"])
}

/// The pretty-printed AST with all whitespace stripped, so structural markers
/// match regardless of the debug formatter's indentation.
fn compact(ast: &str) -> String {
    ast.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn reveal_until_binds_an_iterated_action_with_a_clausal_until_complement() {
    // Ajani, Valiant Protector shape: the reveal loop bounded by a finite
    // `until you reveal a <kind> card` event clause. `until` scans as a
    // subordinator so the existing `<clause> <subordinator> <clause>` rule
    // attaches the complement.
    let source = "Reveal cards from the top of your library until you reveal a creature card.";
    let (rendered, ast) = parse_face(
        source,
        &library_catalogs(),
        "Ajani, Valiant Protector",
        true,
    );
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Subordinate(Until,Finite("),
        "expected a finite `until` subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn repeat_header_rides_the_clausal_until() {
    // Dance with Calamity / Eureka shape: `Repeat this process until <clause>`
    // reuses the same clausal-`until` complement — no repeat-specific rule.
    let source = "Repeat this process until the tie is broken.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Timesifter", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Subordinate(Until,Finite("),
        "expected a finite `until` subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn until_end_of_turn_stays_a_durational_preposition() {
    // The gate: adding the `until` subordinator must not hijack the durational
    // adjunct `until end of turn`, which stays a prepositional phrase — the two
    // readings are disjoint by what follows (a clause vs. a noun phrase).
    let source = "Target creature gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("preposition:Until,"),
        "expected the durational preposition\nAST:\n{ast}"
    );
    assert!(
        !compact(&ast).contains("Subordinate(Until,"),
        "durational `until end of turn` must not parse as a subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn the_other_is_an_anaphoric_fused_head_nominal() {
    // Sea Gate Oracle / Sleight of Hand shape: one verb distributes over two
    // coordinated object+destination pairs, the second object being the
    // anaphoric fused head `the other` (the sibling of `the rest`).
    let source = "Put one of them into your hand and the other on the bottom of your library.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Sea Gate Oracle", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("head:Singular(Word(Other,)"),
        "expected `other` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn the_others_plural_fused_head_nominal_round_trips() {
    // The plural anaphor `the others` declines regularly off the same head.
    let source = "Return the others to the battlefield.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("head:Plural(Word(Other,)"),
        "expected `others` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn attributive_other_stays_an_adjective_not_a_noun_modifier() {
    // The gate: `other` is scanned as a count noun only to license the fused
    // head, and its noun reading is dispreferenced. In modifier position the
    // attributive-adjective reading — equal in every structural cost — must
    // still win, never a `NominalModifier::Noun`.
    let source = "All other creatures get +1/+1.";
    let (rendered, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast)
            .contains("Adjective{polarity:Positive,phrase:AdjectivePhrase{head:Word(Other,)"),
        "expected `other` as an adjective modifier\nAST:\n{ast}"
    );
    assert!(
        !compact(&ast).contains("Noun{polarity:Positive,noun:Singular(Word(Other,)"),
        "`other` must not reduce as a noun modifier\nAST:\n{ast}"
    );
}

#[test]
fn top_of_library_peek_and_play_are_supported() {
    // Xanathar / Case of the Locked Hothouse shape (the in-scope conjuncts):
    // `you may look at the top card of their library any time` and `you may
    // play the top card of their library` already parse — a regression guard,
    // no new machinery.
    let peek = "You may look at the top card of their library any time.";
    let (rendered, ast) = parse_face(peek, &Catalogs::default(), "Xanathar, Guild Kingpin", true);
    assert_eq!(rendered, peek);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");

    let play = "You may play the top card of their library.";
    let (rendered, ast) = parse_face(play, &Catalogs::default(), "Xanathar, Guild Kingpin", true);
    assert_eq!(rendered, play);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
}

// --- R31: the choice / mode-header cluster ----------------------------------

/// The card-type atom the choice-cluster witnesses name.
fn choice_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn trailing_dash_body_licenses_an_or_coordinated_appositive() {
    // The Master, Gallifrey's End shape: a complete clause whose tail names a
    // choice, then a spaced ` — ` and an `or`-coordinated pair of full clauses
    // spelling the options. Both sides parse as clauses; the dash body attaches
    // as a `ComplexClause` appositive. Licensed on shape, never on any word.
    let source = "That player faces a villainous choice — They lose 4 life, or you create a token.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("kind:Appositive("),
        "expected a trailing dash-body appositive\nAST:\n{ast}"
    );
}

#[test]
fn dash_appositive_attaches_under_a_coordinated_predicate_matrix() {
    // This Is How It Ends shape: the matrix is itself a `then`-coordinated
    // predicate, and the appositive trails the whole clause.
    let source = "Target creature's owner shuffles it into their library, then faces a villainous \
                  choice — They lose 5 life, or they draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "This Is How It Ends", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(compact(&ast).contains("kind:Appositive("), "AST:\n{ast}");
}

#[test]
fn flavor_header_dash_is_not_read_as_an_appositive() {
    // Over-fire gate: a flavor header (`<label> — <sentence>`, the label not a
    // clause and the body a single sentence) stays a flavor header. The
    // appositive requires a clause matrix and an `or`-coordinated body, so a
    // non-clause label never fires it.
    let source = "Zorbo Rampage! — Draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        compact(&ast).contains("flavor_header:Some("),
        "expected a flavor header\nAST:\n{ast}"
    );
    assert!(
        !compact(&ast).contains("Appositive("),
        "a flavor header must not become an appositive\nAST:\n{ast}"
    );
}

#[test]
fn single_clause_dash_body_does_not_license_an_appositive() {
    // Over-fire gate: the dash body must be `or`-coordinated. A clause matrix
    // followed by a single clause after the dash fails that test, so no
    // appositive fires — the sentence stays a verbatim recovered span.
    let source = "That player faces a villainous choice — They lose 4 life.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        !compact(&ast).contains("Appositive("),
        "a single-clause dash body must not become an appositive\nAST:\n{ast}"
    );
}

#[test]
fn bare_power_toughness_sentence_is_a_verbless_body() {
    // Vincent's Limit Break tiered-mode body: `3/2.` is a whole sentence whose
    // content is the base power and toughness the mode sets. The period marks a
    // sentence; the value reuses the statistic the object and copular positions
    // already model, and the renderer re-derives the period.
    let source = "3/2.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("body:PowerToughness(PowerToughness{"),
        "expected a verbless power/toughness body\nAST:\n{ast}"
    );
}

#[test]
fn bare_power_toughness_without_a_period_stays_verbatim() {
    // Over-fire gate: a bare `N/N` with no terminal period is a level-band stat
    // line, not a sentence. It must stay a recovered span (rendered verbatim),
    // never a P/T body that would gain a spurious period.
    let source = "2/3";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        !compact(&ast).contains("body:PowerToughness("),
        "a periodless `N/N` must not become a P/T sentence body\nAST:\n{ast}"
    );
}

#[test]
fn those_characteristics_anaphor_parses_as_a_plural_nominal() {
    // Genku, Future Shaper / Outlaws' Merriment modal header: `... token with
    // those characteristics`. The blocker was a plural-form lemma; the singular
    // `characteristic` lemma lets the regular plural satisfy the `those`
    // demonstrative's required plural nominal.
    let source = "Create a creature token with those characteristics.";
    let (rendered, ast) = parse_face(source, &choice_catalogs(), "Genku, Future Shaper", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("head:Plural(Word(Regular(\"characteristic\""),
        "expected the singular `characteristic` lemma pluralized under `those`\nAST:\n{ast}"
    );
}

#[test]
fn descended_intervening_condition_parses_as_a_verb_clause() {
    // Molten Collapse modal header: `If you descended this turn, you may choose
    // both instead.` The bare `both` object and trailing `instead` already
    // parse; the sole blocker was the verb `descend`, now a regular-vocabulary
    // verb whose ordinary past tense heads the `if` condition.
    let source = "If you descended this turn, you may choose both instead.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Molten Collapse", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
}

// --- R32: the flavor-word sentence-header cluster ---------------------------

/// The flavor words the header-cluster witnesses name, plus the one card-type
/// atom their bodies need. `Exterminate!` is a real member ending in `!`: the
/// terminal-punctuation gate must still route it to the old paragraph path.
fn flavor_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(
            CatalogKind::FlavorWord,
            [
                "Chaos",
                "Polymorphine",
                "Make Them Pay",
                "Sanctified Rules of Combat",
                "Aerial Blast",
                "Exterminate!",
            ],
        )
        .with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn flavor_word_header_before_a_paragraph_peels_as_licensed_opacity() {
    // Callidus Assassin shape: a bare flavor-word label set off by a spaced em
    // dash blocked the whole ability. Peeling the catalog-member label leaves
    // the paragraph body to parse by the ordinary machinery, and the label is
    // reproduced verbatim as an ability-level flavor header (licensed opacity,
    // never recovery).
    let source = "Polymorphine — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Callidus Assassin", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("text: \"Polymorphine\""),
        "expected an ability-level flavor-word header\nAST:\n{ast}"
    );
    assert!(
        compact(&ast).contains("flavor_header:Some(FlavorHeader{text:\"Polymorphine\""),
        "the flavor header must sit at the ability level\nAST:\n{ast}"
    );
}

#[test]
fn flavor_word_header_before_a_trigger_lets_the_trigger_recover() {
    // Vincent, Vengeful Atoner shape: the label stands ahead of a trigger frame
    // no paragraph-level header could reach. Peeling it at the ability level
    // lets `Whenever …` recover as a triggered ability.
    let source = "Chaos — Whenever this creature attacks, draw a card.";
    let (rendered, ast) = parse_face(
        source,
        &flavor_catalogs(),
        "Vincent, Vengeful Atoner",
        false,
    );
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("text: \"Chaos\""), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("kind:Triggered("),
        "expected the trigger behind the label to recover\nAST:\n{ast}"
    );
}

#[test]
fn multi_word_flavor_word_header_peels_the_whole_label() {
    // `Make Them Pay` is a three-token flavor word: the peel matches the whole
    // catalog member, never a shorter prefix, and reproduces it verbatim.
    let source = "Make Them Pay — Draw a card.";
    let (rendered, ast) = parse_face(
        source,
        &flavor_catalogs(),
        "The Master, Gallifrey's End",
        false,
    );
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        ast.contains("text: \"Make Them Pay\",") && ast.contains("source_tokens: 3"),
        "expected the whole three-token label as one flavor header\nAST:\n{ast}"
    );
}

#[test]
fn flavor_word_header_uncovers_a_villainous_choice_appositive() {
    // Midnight Crusader Shuttle / The Master cascade: behind the un-peeled label
    // sat a villainous-choice appositive the prior round's machinery already
    // handles. Once the label peels, the inner `— <or-coordinated>` attaches as
    // a `ComplexClause` appositive with no new grammar.
    let source =
        "Chaos — That player faces a villainous choice — They lose 4 life, or you create a token.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Sycorax Commander", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(ast.contains("text: \"Chaos\""), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("kind:Appositive("),
        "expected the inner villainous-choice appositive to parse\nAST:\n{ast}"
    );
}

#[test]
fn flavor_word_header_inside_a_saga_chapter_body_peels() {
    // Summon: Primal Garuda shape: a saga chapter body opens with a flavor-word
    // label (`I — Aerial Blast — <effect>`). The chapter header `I` peels
    // structurally at the ability level; the paragraph-level peel then takes the
    // catalog-member label off the chapter body so the effect recovers. This is
    // the paragraph-start half of the licensing, one level below the trigger
    // case, reached only after the chapter frame splits the header.
    let source = "I — Aerial Blast — This creature deals 4 damage to target creature.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Summon: Primal Garuda", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(compact(&ast).contains("kind:Chapter("), "AST:\n{ast}");
    assert!(
        ast.contains("text: \"Aerial Blast\""),
        "expected the chapter body's flavor-word header to peel\nAST:\n{ast}"
    );
}

#[test]
fn saga_chapter_roman_header_is_not_a_flavor_word() {
    // Over-fire gate (a): a saga chapter's `I — <body>` carries a spaced em
    // dash, but `I` is not a flavor-word member, so the peel declines and the
    // chapter frame keeps it. Nothing peels as a flavor header.
    let source = "I — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Saga", false);
    assert_eq!(rendered, source);
    assert!(
        !compact(&ast).contains("flavor_header:Some("),
        "a roman chapter header must not peel as a flavor word\nAST:\n{ast}"
    );
    assert!(
        compact(&ast).contains("kind:Chapter("),
        "the roman numeral must stay a saga chapter header\nAST:\n{ast}"
    );
}

#[test]
fn a_capitalized_non_member_label_stays_recovered() {
    // Over-fire gate (b): a capitalized label that is not a catalog member is
    // never peeled — it falls through to a verbatim recovered span exactly as
    // before, so the peel is strictly catalog-licensed.
    let source = "Foobar — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        !compact(&ast).contains("flavor_header:Some("),
        "a non-member label must not peel\nAST:\n{ast}"
    );
    assert!(ast.contains("Recovered"), "AST:\n{ast}");
}

#[test]
fn a_terminal_punctuation_flavor_member_keeps_the_paragraph_path() {
    // Over-fire gate (c): `Exterminate!` is a real flavor-word member, but it
    // ends in inert terminal punctuation, so the ability-level peel declines and
    // the existing paragraph-level flavor-header peel keeps it. The header lands
    // on the paragraph, not the ability.
    let source = "Exterminate! — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("flavor_header:None,kind:Paragraph(Paragraph{flavor_header:Some("),
        "an exclamation-terminated member must keep the paragraph flavor-header path\nAST:\n{ast}"
    );
}

#[test]
fn a_cost_flavor_header_with_a_non_member_label_is_unaffected() {
    // Over-fire gate (d): the activation-cost flavor-header peel is untouched. A
    // bare non-member label ahead of a cost still lands on the cost, never the
    // ability, so activation-cost handling does not move.
    let source = "Sneaky — {T}: Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        compact(&ast).contains("Cost{flavor_header:Some("),
        "a non-member cost label must stay a cost flavor header\nAST:\n{ast}"
    );
    assert!(
        compact(&ast).contains("ability_word:None,flavor_header:None,kind:Activated("),
        "the ability-level flavor slot must stay empty for a cost header\nAST:\n{ast}"
    );
}

// --- R33: the lexical-sweep round --------------------------------------------

/// The card-type atoms the lexical-sweep witnesses name.
fn r33_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(
        CatalogKind::CardType,
        [
            "Creature",
            "Land",
            "Artifact",
            "Enchantment",
            "Planeswalker",
        ],
    )
}

#[test]
fn one_is_a_dispreferenced_fused_head_noun() {
    // Touch of Darkness shape: sentence-initial `One` coordinated with `more`
    // has no following head noun, so `one` fills the fused head itself — the
    // sibling of `other`'s fused head, dispreferenced the same way
    // (`is_fused_head_noun`) so it never outranks the ordinary numeral reading.
    let source = "One or more target creatures become black until end of turn.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Touch of Darkness", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("head:Singular(Word(One,)"),
        "expected `One` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn number_literal_one_still_wins_over_the_noun_reading() {
    // The gate: `one` is scanned as a count noun only to license the fused
    // head above. In an ordinary quantity position the numeral reading — equal
    // in every other structural cost — must still win the tiebreak, never a
    // `Noun::Word(One)`.
    let source = "Return up to one target creature card from your graveyard to the battlefield.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Badlands Revival", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("NumberLiteral{value:1,"),
        "expected `one` as a number literal\nAST:\n{ast}"
    );
    assert!(
        !compact(&ast).contains("Word(One,)"),
        "`one` must not reduce as a fused-head noun in quantity position\nAST:\n{ast}"
    );
}

#[test]
fn itself_reflexive_pronoun_round_trips() {
    // Asmoranomardicadaistinaculdacar shape: a creature-subject reflexive
    // object, the sibling of `each_other` at the same object-case pronoun
    // slot — no new grammar, just a new pronoun identity.
    let source = "Target creature deals 6 damage to itself.";
    let (rendered, ast) = parse_face(
        source,
        &r33_catalogs(),
        "Asmoranomardicadaistinaculdacar",
        false,
    );
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("pronoun:Itself,"),
        "expected the reflexive pronoun `itself`\nAST:\n{ast}"
    );
}

#[test]
fn himself_reflexive_pronoun_round_trips() {
    // Sarkhan the Mad shape: the same object-case reflexive identity, standing
    // in for a legendary planeswalker subject.
    let source = "Sarkhan deals damage to himself equal to that card's mana value.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Sarkhan the Mad", true);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("pronoun:Himself,"),
        "expected the reflexive pronoun `himself`\nAST:\n{ast}"
    );
}

#[test]
fn yours_absolute_pronoun_round_trips() {
    // Geyser Drake shape: the absolute possessive `yours`, a third object-case
    // identity at the same existing pronoun slot.
    let source = "During turns other than yours, spells you cast cost {1} less to cast.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Geyser Drake", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("pronoun:YoursAbsolute,"),
        "expected the absolute possessive pronoun `yours`\nAST:\n{ast}"
    );
}

#[test]
fn fewest_is_a_fused_head_superlative_noun() {
    // Balance shape: `the fewest` has no following head noun, so the
    // superlative itself heads the nominal (plain noun sense, no dispreference
    // needed — `fewest` has no competing reading).
    let source = "Each player chooses a number of lands they control equal to the number of \
        lands controlled by the player who controls the fewest, then sacrifices the rest.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Balance", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Fewest,"),
        "expected `fewest` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn most_is_an_attributive_superlative_adjective() {
    // No Witnesses shape: `the most creatures` modifies a following plural
    // head noun, exercising the adjective sense of the same `most` entry.
    let source = "Each player who controls the most creatures investigates.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "No Witnesses", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Most,"),
        "expected `most` as the superlative modifier\nAST:\n{ast}"
    );
}

#[test]
fn nearest_is_an_attributive_superlative_adjective() {
    // Mystic Barrier shape: `the nearest opponent`, the same attributive
    // superlative pattern as `most`.
    let source = "Each player may attack only the nearest opponent in the last chosen direction \
        and planeswalkers controlled by that opponent.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Mystic Barrier", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Nearest,"),
        "expected `nearest` as the superlative modifier\nAST:\n{ast}"
    );
}

#[test]
fn bid_bidding_and_bidder_forms_round_trip() {
    // Illicit Auction shape: the irregular verb `bid` (bid/bidding/bid), its
    // gerund-as-noun `the bidding` (the existing nominalization rule, no
    // separate vocabulary entry), and the regular noun `bidder` all combine in
    // one fully structured auction sequence.
    let source = "Each player may bid life for control of target creature. \
        You start the bidding with a bid of 0. \
        In turn order, each player may top the high bid. \
        The bidding ends if the high bid stands. \
        The high bidder loses life equal to the high bid and gains control of the creature.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Illicit Auction", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
}

#[test]
fn crew_ordinary_verb_coexists_with_the_crew_keyword_action() {
    // Canyon Vaulter shape: `crews` as an ordinary third-person verb in
    // running prose, distinct from (and non-conflicting with) the catalog-
    // driven `Crew N` keyword-action cost line.
    let source =
        "Whenever this creature crews an artifact, that artifact gains flying until end of turn.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Canyon Vaulter", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"crew\""),
        "expected `crew` as an ordinary verb\nAST:\n{ast}"
    );
}

#[test]
fn escape_verb_round_trips() {
    // Chainweb Aracnir shape: `escapes` as an ordinary present-tense verb,
    // distinct from the unrelated `Escape—<cost>` keyword-cost header line
    // (a separate, unspaced-em-dash frame this round does not touch).
    let source = "This creature escapes with three +1/+1 counters on it.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Chainweb Aracnir", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"escape\""),
        "expected `escape` as an ordinary verb\nAST:\n{ast}"
    );
}

#[test]
fn everything_mass_noun_round_trips() {
    // Hexdrinker shape (`Protection from everything`) is a bare verbless
    // level-body line, licensed only inside the class-level frame this round
    // does not touch; the same `everything`-as-bare-object-of-`from` shape
    // exercised as an ordinary clause instead.
    let source = "Enchanted creature has protection from everything.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"everything\""),
        "expected `everything` as a mass noun\nAST:\n{ast}"
    );
}

#[test]
fn void_voter_and_fame_nouns_round_trip() {
    // Dauthi Voidwalker (`void` as a noun-modifier in the productive `<word>
    // counter` pattern), Elrond of the White Council (`voter` as a bare
    // subject noun), and Seize the Spotlight (`fame` as a bare object noun).
    let void = "If a card would be put into an opponent's graveyard from anywhere, instead \
        exile it with a void counter on it.";
    let (rendered, ast) = parse_face(void, &Catalogs::default(), "Dauthi Voidwalker", false);
    assert_eq!(rendered, void);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"void\""),
        "expected `void` as a noun modifier\nAST:\n{ast}"
    );

    let voter = "For each fellowship vote, the voter chooses a creature they control.";
    let (rendered, ast) = parse_face(voter, &r33_catalogs(), "Elrond of the White Council", true);
    assert_eq!(rendered, voter);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"voter\""),
        "expected `voter` as a bare noun\nAST:\n{ast}"
    );

    let fame = "Each opponent chooses fame or fortune.";
    let (rendered, ast) = parse_face(fame, &Catalogs::default(), "Seize the Spotlight", false);
    assert_eq!(rendered, fame);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"fame\""),
        "expected `fame` as a bare noun\nAST:\n{ast}"
    );
}

#[test]
fn either_is_a_fused_head_pronoun_noun() {
    // Worms of the Earth shape: `does either` has no following head noun, so
    // `either` fills the fused head itself.
    let source = "If a player does either, destroy this enchantment.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Worms of the Earth", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"either\""),
        "expected `either` as a fused-head noun\nAST:\n{ast}"
    );
}

#[test]
fn much_and_many_are_fused_head_nouns_with_a_partitive_complement() {
    // Mana Reflection (`as much of that mana`) and Vorinclex, Monstrous Raider
    // (`that many of each of those kinds of counters`): both fill a partitive
    // `of`-complement head the dedicated `Quantity::ThatMuch`/`ThatMany`
    // determiner shapes don't reach — that shape only covers a simple
    // determined noun as the partitive whole, not a nested partitive.
    let much = "If you tap a permanent for mana, it produces twice as much of that mana instead.";
    let (rendered, ast) = parse_face(much, &Catalogs::default(), "Mana Reflection", false);
    assert_eq!(rendered, much);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"much\""),
        "expected `much` as a fused-head noun\nAST:\n{ast}"
    );

    let many = "If you would put one or more counters on a permanent or player, put twice that \
        many of each of those kinds of counters on that permanent or player instead.";
    let (rendered, ast) = parse_face(
        many,
        &Catalogs::default(),
        "Vorinclex, Monstrous Raider",
        true,
    );
    assert_eq!(rendered, many);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"many\""),
        "expected `many` as a fused-head noun\nAST:\n{ast}"
    );
}

#[test]
fn print_and_expansion_nouns_round_trip() {
    // Apocalypse Chime shape: `printed` (the regular verb `print`'s past
    // participle) and `expansion` (a regular noun) complete the surrounding
    // reduced-relative and prepositional-phrase shapes. `originally` stays
    // out of vocabulary this round (its adverb sense regressed elsewhere — see
    // the residue notes), so it fills the reduced relative's subject slot as
    // licensed opacity rather than as an adverb; this is a real, measured,
    // already-landed improvement over the prior full-clause failure.
    let source = "Destroy all nontoken permanents with a name originally printed in the \
        Homelands expansion. They can't be regenerated.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Apocalypse Chime", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"print\""),
        "expected `print` as the reduced-relative verb\nAST:\n{ast}"
    );
    assert!(
        compact(&ast).contains("Regular(\"expansion\""),
        "expected `expansion` as a bare noun\nAST:\n{ast}"
    );
}

#[test]
fn level_and_rad_nouns_round_trip() {
    // The Class-card `level counter` idiom and Mariposa Military Base's `rad
    // counter`: both are the same productive `<word> counter(s)` noun-
    // modifier pattern as `void` above.
    let level = "{1}: Put a level counter on this.";
    let (rendered, ast) = parse_face(level, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, level);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"level\""),
        "expected `level` as a noun modifier\nAST:\n{ast}"
    );

    let rad = "You may have this land enter tapped. If you do, you get two rad counters.";
    let (rendered, ast) = parse_face(rad, &r33_catalogs(), "Mariposa Military Base", false);
    assert_eq!(rendered, rad);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("Regular(\"rad\""),
        "expected `rad` as a noun modifier\nAST:\n{ast}"
    );
}

#[test]
fn between_preposition_licenses_a_coordinated_number_range() {
    // By Invitation Only shape: `between 0 and 13` is the primary corpus
    // motivation for the new `Preposition::Between` variant — a plain
    // addition to the closed preposition set feeding the existing generic
    // `PrepositionalPhrase` production, not a new chart rule.
    let source = "Choose a number between 0 and 13.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "By Invitation Only", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("preposition:Between,"),
        "expected the new `between` preposition\nAST:\n{ast}"
    );
}

#[test]
fn between_preposition_licenses_the_difference_between_shape() {
    // Jaws of Defeat shape (the `difference between X and Y` idiom): a second,
    // independent corpus motivation for `Preposition::Between`, guarding the
    // new enum variant's exhaustive wiring (scanner, renderer, and the
    // `PrepositionalPhrase` production) against a narrower reading that only
    // happened to fit the number-range case above.
    let source = "That player loses life equal to the difference between its power and its \
        toughness.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(!ast.contains("Recovered"), "AST:\n{ast}");
    assert!(
        compact(&ast).contains("preposition:Between,"),
        "expected the new `between` preposition\nAST:\n{ast}"
    );
}
