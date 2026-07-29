use super::*;

fn fixture_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"])
        .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
        .with_catalog(CatalogKind::Supertype, ["Basic", "Legendary"])
}

#[test]
fn every_reserved_literal_surface_is_a_known_word() {
    // has_known_word's own invariant: a word the lexicon already knows —
    // including hand-written literal lexemes, not just vocabulary/catalog
    // slots — must never be reported as unknown (and so must never be
    // opacified). Extends pluralposs's
    // `a_known_literal_lexeme_is_never_opacifiable` (which folds into this
    // test) to the typed literal-slot table.
    let catalogs = fixture_catalogs();
    for &slot in EnglishLexicalSlot::LITERAL_SLOTS {
        assert!(
            !slot.literal_surfaces().is_empty(),
            "literal slot has no declared surface: {slot:?}"
        );
        for (index, &literal) in slot.literal_surfaces().iter().enumerate() {
            if !slot.reserves_literal_for_opacity(index) {
                continue;
            }
            let surface = crate::surface::lex(literal);
            let grammar = EnglishGrammar::new(literal, &catalogs, Nonterminal::NounPhrase);
            assert!(
                grammar.has_known_word(&surface.tokens, 0),
                "{literal} from {slot:?} must be a known word"
            );
        }
    }
}

#[test]
fn measured_literal_opacity_opt_outs_are_explicit() {
    let opted_out = [
        (EnglishLexicalSlot::VerbParticle(VerbParticle::Out), "out"),
        (EnglishLexicalSlot::QuantityBoth, "both"),
        (EnglishLexicalSlot::Half, "half"),
        (EnglishLexicalSlot::Rounded, "rounded"),
        (EnglishLexicalSlot::RatherThan, "rather"),
        (EnglishLexicalSlot::Existential, "there's"),
        (EnglishLexicalSlot::SubjectAuxiliary, "you've"),
        (EnglishLexicalSlot::SubjectAuxiliary, "he's"),
        (EnglishLexicalSlot::SubjectAuxiliary, "she's"),
        (EnglishLexicalSlot::SubjectAuxiliary, "they're"),
        (EnglishLexicalSlot::SubjectAuxiliary, "they've"),
    ];
    for (slot, expected) in opted_out {
        let index = slot
            .literal_surfaces()
            .iter()
            .position(|surface| *surface == expected)
            .unwrap_or_else(|| panic!("missing {expected:?} from {slot:?}"));
        assert!(
            !slot.reserves_literal_for_opacity(index),
            "{expected:?} must remain opacity-visible pending its audit"
        );
    }
}

#[test]
fn reserved_literals_are_never_opaque_nouns() {
    // Per-literal parse assertions for the seven reserved literals with
    // corpus witnesses (litaudit-plan.md §1.2). Whichever of the two
    // outcomes ((i) re-parse via the literal's real production, or (ii)
    // the sentence honestly loses its only complete parse) the face lands
    // in, no `Opaque(OpaqueLexeme(<literal>))` node may survive in the
    // tree: an `Err` result (outcome (ii): no complete parse) or an `Ok`
    // result whose debug dump contains no reserved-literal `OpaqueLexeme`
    // both satisfy the invariant.
    let catalogs = fixture_catalogs();
    let witnesses: &[(&str, &str)] = &[
        (
            "there",
            "Exile target creature card from a graveyard that was put there this turn.",
        ),
        (
            "up",
            "Whenever this creature deals combat damage to a player, return up to that many target permanents that player controls to their owner's hand.",
        ),
        (
            "than",
            "Damage that would reduce your life total to less than 1 reduces it to 1 instead.",
        ),
        (
            "it's",
            "Whenever one or more +1/+1 counters are put on another permanent you control, if it's the first time +1/+1 counters have been put on that permanent this turn, put a +1/+1 counter on this creature.",
        ),
        (
            "that's",
            "Target creature card in your graveyard that's an artifact or that has mana value 3 or less gains escape until end of turn.",
        ),
        (
            "down",
            "You may return this card from your graveyard to the battlefield face up or face down.",
        ),
        (
            "minus",
            "If one or more -1/-1 counters would be put on a creature you control, that many -1/-1 counters minus one are put on it instead.",
        ),
    ];
    for (literal, source) in witnesses {
        match parse_nonterminal(source, &catalogs, Nonterminal::Sentence) {
            Err(_) => {
                // Outcome (ii): the sentence honestly loses its only
                // complete parse once the literal can no longer be
                // swallowed as an opaque noun. No tree, so no opaque node
                // of any kind can survive.
            }
            Ok(parsed) => {
                let dump = format!("{parsed:#?}");
                let needle = format!("OpaqueLexeme(\n        \"{literal}\"");
                let needle_inline = format!("OpaqueLexeme(\"{literal}\")");
                assert!(
                    !dump.contains(&needle) && !dump.contains(&needle_inline),
                    "{literal}: reserved literal survived as an opaque noun: {dump}"
                );
            }
        }
    }
}

#[test]
fn the_opacity_gate_is_the_predicates_only_caller() {
    // Pins litaudit-plan.md §4 structurally: has_known_word gates
    // opacification only, at its two call sites in `scan`'s
    // `EnglishLexicalSlot::Opaque` arm. This is the executable half; the
    // source-grep gate (`rg -n 'has_known_word'`) is run by the mechanic
    // per round and is not itself a test.
    let catalogs = fixture_catalogs();

    // Call site 1 (`already_known`, :2027): a known-word start yields no
    // opaque candidates at all.
    let source = "who";
    let surface = crate::surface::lex(source);
    let grammar = EnglishGrammar::with_opacity_mode(
        source,
        &catalogs,
        Nonterminal::NounPhrase,
        OpacityMode::OpaqueNouns,
        SelfReference::default(),
    );
    let matches = grammar.scan(
        EnglishLexicalSlot::Opaque(OpacitySlot::Noun(NounForm::Singular)),
        &surface.tokens,
        0,
    );
    assert!(
        matches.is_empty(),
        "a known-word start must never yield an opaque candidate: {matches:?}"
    );

    // Call site 2 (the `retain`, :2032-2035): an opaque candidate that
    // would span a known reserved literal is dropped even when the head
    // token itself is unknown.
    let source = "gloopmonster minus one";
    let surface = crate::surface::lex(source);
    let grammar = EnglishGrammar::with_opacity_mode(
        source,
        &catalogs,
        Nonterminal::NounPhrase,
        OpacityMode::OpaqueNouns,
        SelfReference::default(),
    );
    let matches = grammar.scan(
        EnglishLexicalSlot::Opaque(OpacitySlot::Noun(NounForm::Singular)),
        &surface.tokens,
        0,
    );
    assert!(
        matches.iter().all(|candidate| !(1..candidate.end)
            .any(|index| grammar.has_known_word(&surface.tokens, index))),
        "no surviving opaque candidate may span a known word: {matches:?}"
    );
}
