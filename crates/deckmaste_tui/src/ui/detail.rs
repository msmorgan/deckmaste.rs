//! The detail pane's text. An object's printed face plus its *derived*
//! [`Characteristics`](deckmaste_engine::Characteristics) are raised back to
//! SEMANTIC terms through the load-time provenance index, bridged into a
//! [`CardView`] and run through the engine-free `deckmaste_legacy_render`
//! renderer, so the pane shows real rules text over the live (pumped, animated,
//! control-changed) object — not the printed encoding.
//!
//! A derived value the index cannot raise is rendered as a visible
//! `[unrendered: …]` marker line, never dropped: the pane may fail to produce
//! prose, but it must not quietly show a shorter card than the object has.
use std::fmt::Write as _;

use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::ObjectSource;
use deckmaste_engine::StackObject;
use deckmaste_engine::face;
use deckmaste_legacy_render::render::CardView;
use deckmaste_legacy_render::render::RenderedCard;
use deckmaste_legacy_render::render::render as render_card_view;
use deckmaste_plugin::provenance::raise_supertype;
use deckmaste_semantics::Expand;
use deckmaste_semantics::StatValue as SemanticStatValue;
use ratatui::text::Text;

use crate::game::ProvenanceRefs;
use crate::ui::board::Selected;

/// Render the detail pane for the current selection.
#[must_use]
pub fn render(
    state: &GameState,
    view: &LayeredView,
    sel: Option<Selected>,
    provenance: ProvenanceRefs<'_>,
) -> Text<'static> {
    match sel {
        None => Text::from("(no selection)"),
        Some(Selected::Object(id)) => object_detail(state, view, id, provenance),
        Some(Selected::StackEntry(i)) => stack_detail(state, view, i, provenance),
    }
}

fn object_detail(
    state: &GameState,
    view: &LayeredView,
    id: ObjectId,
    provenance: ProvenanceRefs<'_>,
) -> Text<'static> {
    if let ObjectSource::Player(pid) = state.objects.obj(id).source {
        let p = state.player(pid);
        let hand = state.zones.hands[pid.index()].len();
        return Text::from(format!(
            "Player {}\n{} life\nhand: {hand} card(s)",
            pid.0, p.life
        ));
    }
    let printed = face(state.def(id));
    let chars = view.get(id);

    // The renderer reads SEMANTIC terms; `chars` is derived engine state and is
    // core-typed in every field. Raise it component-wise through the load-time
    // index rather than substituting the printed semantic card — a pumped,
    // animated, type-changed object is exactly what this pane exists to show,
    // so the derived values are the ones that must survive.
    //
    // Every component is expected to have a semantic preimage: abilities are
    // printed (`lower` of a semantic one), granted (a verbatim clone,
    // [CR#613.1f]) or conferred (from the registry); subtypes and card types
    // are clones of registry entries; supertypes are a closed enum. A miss is
    // not an error, but it is never silent: the unraised core value is kept
    // aside and appended as its own `[unrendered: …]` marker line, so a lost
    // ability shows as a marker instead of a vanilla body and a lost type
    // shows as a marker instead of a quietly shortened type line.
    let supertypes: Vec<_> = chars
        .supertypes
        .iter()
        .copied()
        .map(raise_supertype)
        .collect();
    let mut unraised: Vec<String> = Vec::new();
    let types = raise(chars.card_types.iter(), &mut unraised, |t| {
        provenance.index.type_def(t)
    });
    let subtypes = raise(chars.subtypes.iter(), &mut unraised, |s| {
        provenance.index.subtype(s)
    });
    let abilities = raise(chars.abilities.iter(), &mut unraised, |a| {
        provenance.index.ability(a)
    });

    // Name and mana cost aren't derived characteristics. The cost comes from
    // the semantic card via the companion table; a token has no semantic card
    // and no cost, which is the `None` this resolves to. Power/toughness derive
    // to concrete numbers — lift them back into the `StatValue` `CardView`
    // wants, now the semantic one.
    let semantic = card_id(state, id).and_then(|c| provenance.cards.get(c));
    let semantic_face = semantic.map(|c| semantic_front_face(c));
    let power = chars.power.map(SemanticStatValue::Number);
    let toughness = chars.toughness.map(SemanticStatValue::Number);
    let card_view = CardView {
        name: &printed.characteristics.name,
        mana_cost: semantic_face.map(|f| &f.mana_cost),
        supertypes: &supertypes,
        types: &types,
        subtypes: &subtypes,
        power: power.as_ref(),
        toughness: toughness.as_ref(),
        abilities: &abilities,
    };
    let mut card = render_card_view(&card_view);
    // Remembered macro invocations carry templates that usually produce the
    // best prose. If any rule still falls back to Debug output, rerender the
    // whole card without that provenance so the marker describes concrete,
    // recursively expanded grammar instead of the semantic macro spelling.
    if card.rules.iter().any(|rule| rule.contains("[unrendered")) {
        let expanded_abilities = abilities
            .iter()
            .cloned()
            .map(Expand::expand_all)
            .collect::<Vec<_>>();
        card = render_card_view(&CardView {
            abilities: &expanded_abilities,
            ..card_view
        });
    }
    // AFTER the re-render branch, which keys off the renderer's own markers:
    // these are not renderer output — the renderer never saw the value — so
    // re-rendering could neither remove nor improve them, and appending them
    // first would only trigger a pointless second pass.
    card.rules.extend(unraised);
    detail_text(&card)
}

/// The visible form of a provenance miss: the core value's `Debug`, truncated.
///
/// Deliberately not prose. The pane has no semantic term for this value, and
/// inventing one would be worse than saying so; `Debug` at least identifies
/// WHICH value went unrecovered. Truncated because a derived ability's `Debug`
/// can run to thousands of characters and would push the rest of the card off
/// the pane.
fn unrendered_marker(value: &impl std::fmt::Debug) -> String {
    /// Chars of `Debug` kept before eliding — about one pane line.
    const WIDTH: usize = 120;
    let debug = format!("{value:?}");
    match debug.char_indices().nth(WIDTH) {
        Some((cut, _)) => format!("[unrendered: {}…]", &debug[..cut]),
        None => format!("[unrendered: {debug}]"),
    }
}

/// Raises each derived value to its semantic term, collecting the misses into
/// `unraised` as marker lines. A partition, not a filter: dropping a miss would
/// make the pane render a shorter card than the object actually is.
fn raise<'a, C, A>(
    derived: impl Iterator<Item = &'a C>,
    unraised: &mut Vec<String>,
    lookup: impl Fn(&'a C) -> Option<&'a A>,
) -> Vec<A>
where
    C: std::fmt::Debug + 'a,
    A: Clone + 'a,
{
    let mut raised = Vec::new();
    for value in derived {
        match lookup(value) {
            Some(semantic) => raised.push(semantic.clone()),
            None => unraised.push(unrendered_marker(value)),
        }
    }
    raised
}

/// The `CardId` a rendered object came from, or `None` for a player object. A
/// token or emblem still answers `Some`: it is minted with a `CardId` past the
/// end of the deck's card table, so it is `CardProvenance::get` that resolves
/// it to `None`, not this.
fn card_id(state: &GameState, id: ObjectId) -> Option<deckmaste_engine::CardId> {
    match state.objects.obj(id).source {
        ObjectSource::Card(c) => Some(c),
        ObjectSource::Player(_) => None,
    }
}

/// The semantic front face — the one whose printed cost the pane shows.
fn semantic_front_face(card: &deckmaste_semantics::Card) -> &deckmaste_semantics::CardFace {
    match card {
        deckmaste_semantics::Card::Normal(f) => f,
        deckmaste_semantics::Card::TwoFaced { front, .. } => front,
    }
}

/// Lay out a rendered card as the detail pane's plain text: name, cost (when
/// any), type line, P/T (when any), then one line per rule.
fn detail_text(card: &RenderedCard) -> Text<'static> {
    let mut s = String::new();
    let _ = writeln!(s, "{}", card.name);
    if !card.mana_cost.is_empty() {
        let _ = writeln!(s, "{}", card.mana_cost);
    }
    let _ = writeln!(s, "{}", card.type_line);
    if let Some(pt) = &card.pt {
        let _ = writeln!(s, "{pt}");
    }
    for rule in &card.rules {
        let _ = writeln!(s, "{rule}");
    }
    Text::from(s)
}

fn stack_detail(
    state: &GameState,
    view: &LayeredView,
    i: usize,
    provenance: ProvenanceRefs<'_>,
) -> Text<'static> {
    let Some(entry) = state.stack.get(i) else {
        return Text::from("(empty)");
    };
    match &entry.object {
        StackObject::Spell(id) => object_detail(state, view, *id, provenance),
        StackObject::Triggered { .. } => Text::from(format!(
            "triggered ability\ncontroller: P{}",
            entry.controller.0
        )),
        StackObject::Activated { .. } => Text::from(format!(
            "activated ability\ncontroller: P{}",
            entry.controller.0
        )),
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::face;
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    fn text_to_string(t: &Text) -> String {
        t.lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The driver, not just its state: the detail pane renders through the
    /// provenance the driver carries, so dropping it would drop the channel
    /// under test.
    fn opening() -> Driver {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        d
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn none_is_placeholder() {
        let d = opening();
        let state = &d.state;
        let view = state.layers();
        assert_eq!(
            text_to_string(&render(state, &view, None, d.provenance_refs())),
            "(no selection)"
        );
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn object_detail_names_the_card_and_type_line() {
        let d = opening();
        let state = &d.state;
        let view = state.layers();
        let id = *state.zones.hands[0].first().expect("nonempty hand");
        let s = text_to_string(&render(
            state,
            &view,
            Some(Selected::Object(id)),
            d.provenance_refs(),
        ));
        assert!(
            s.contains(&*face(state.def(id)).characteristics.name),
            "detail names the card: {s}"
        );
    }

    /// Elvish Visionary's `Triggered(ThisEnters, Draw(1))` renders as a real
    /// sentence ("…draw a card."), proving the detail pane runs the
    /// `deckmaste_plugin` renderer over derived characteristics rather than
    /// Debug-formatting the abilities.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn object_detail_renders_abilities_as_prose_not_debug() {
        let d = opening();
        let state = &d.state;
        let view = state.layers();
        let id = state
            .objects
            .iter()
            .filter(|o| o.card_id().is_some())
            .map(|o| o.id)
            .find(|&id| &*face(state.def(id)).characteristics.name == "Elvish Visionary")
            .expect("Elvish Visionary in game");
        let s = text_to_string(&render(
            state,
            &view,
            Some(Selected::Object(id)),
            d.provenance_refs(),
        ));
        assert!(s.contains("draw a card"), "renders effect as prose: {s}");
        assert!(!s.contains("Triggered"), "no Debug ability form: {s}");
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn object_detail_renders_mogg_fanatic() {
        let d = opening();
        let state = &d.state;
        let view = state.layers();
        let id = state
            .objects
            .iter()
            .filter(|o| o.card_id().is_some())
            .map(|o| o.id)
            .find(|&id| &*face(state.def(id)).characteristics.name == "Mogg Fanatic")
            .expect("Mogg Fanatic in game");
        let s = text_to_string(&render(
            state,
            &view,
            Some(Selected::Object(id)),
            d.provenance_refs(),
        ));
        assert!(s.contains("Sacrifice"), "renders sacrifice cost: {s}");
        assert!(s.contains("1 damage"), "renders damage effect: {s}");
    }

    /// A layer-6 grant renders as PROSE in the pane, not as an `[unrendered]`
    /// marker — the whole reason the provenance index exists.
    ///
    /// A keyword counter ([CR#122.1b]) is the grant path that reaches an object
    /// without appearing on any card: the layer pass folds the counter
    /// registry's `Continuous(This, GainAbility(Keyword(Flying)))` into a
    /// continuous effect and pushes the payload verbatim ([CR#613.1f]). Nothing
    /// on the host card mentions it, so the index's counter entry is the only
    /// thing that can raise it back to semantic text.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn object_detail_renders_a_layer_six_grant_as_prose() {
        use deckmaste_core::Type;
        use deckmaste_core::Zone;

        let mut d = opening();
        // Host chosen for a clean baseline render, so the assertions below are
        // about the GRANT and not about whatever the host's own text does.
        let host = {
            let state = &d.state;
            let view = state.layers();
            state.zones.hands[0]
                .iter()
                .copied()
                .filter(|&id| view.get(id).has_type(Type::Creature))
                .find(|&id| {
                    !text_to_string(&render(
                        state,
                        &view,
                        Some(Selected::Object(id)),
                        d.provenance_refs(),
                    ))
                    .contains("[unrendered")
                })
                .expect("a hand creature whose printed text renders cleanly")
        };

        // Onto the battlefield (static/continuous effects function only there,
        // [CR#611.3b]) with a flying counter on it.
        d.state.zones.hands[0].retain(|&id| id != host);
        d.state.zones.battlefield.push(host);
        let obj = d.state.objects.obj_mut(host);
        obj.zone = Some(Zone::Battlefield);
        obj.counters.insert("FlyingCounter".into(), 1);

        let state = &d.state;
        let view = state.layers();
        assert!(
            view.get(host).abilities.len() > face(state.def(host)).characteristics.abilities.len(),
            "the counter granted an ability the printed card does not have",
        );
        let s = text_to_string(&render(
            state,
            &view,
            Some(Selected::Object(host)),
            d.provenance_refs(),
        ));
        assert!(
            s.to_lowercase().contains("flying"),
            "the granted flying renders as prose: {s}"
        );
        assert!(
            !s.contains("[unrendered"),
            "the grant did not degrade to a marker: {s}"
        );
    }

    /// The miss fallback, which is the whole point of not using `filter_map`:
    /// a derived value the index cannot raise degrades VISIBLY instead of
    /// vanishing, and never panics.
    ///
    /// The pane is fed an EMPTY index — the strongest form of "an ability that
    /// cannot be in the index", and one no corpus change can accidentally
    /// satisfy. Every raisable component must then come back as its own
    /// `[unrendered: …]` line: one per ability, card type and subtype. Dropping
    /// them instead would render a real creature as a nameless vanilla body.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn object_detail_marks_every_provenance_miss() {
        use deckmaste_plugin::provenance::ProvenanceIndex;

        let d = opening();
        let state = &d.state;
        let view = state.layers();
        let id = state
            .objects
            .iter()
            .filter(|o| o.card_id().is_some())
            .map(|o| o.id)
            .find(|&id| &*face(state.def(id)).characteristics.name == "Elvish Visionary")
            .expect("Elvish Visionary in game");

        let empty = ProvenanceIndex::default();
        let s = text_to_string(&render(
            state,
            &view,
            Some(Selected::Object(id)),
            ProvenanceRefs {
                cards: d.provenance_refs().cards,
                index: &empty,
            },
        ));

        let chars = view.get(id);
        let expected = chars.abilities.len() + chars.card_types.len() + chars.subtypes.len();
        assert!(expected > 0, "the fixture has something to lose");
        let markers = s.lines().filter(|l| l.contains("[unrendered")).count();
        assert_eq!(
            markers, expected,
            "one marker per unraisable component, none silently dropped: {s}",
        );
        assert!(
            s.contains(&*face(state.def(id)).characteristics.name),
            "the card is still named: {s}"
        );
    }
}
