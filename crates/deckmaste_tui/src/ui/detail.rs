//! The detail pane's text. An object's printed face plus its *derived*
//! [`Characteristics`](deckmaste_engine::Characteristics) are bridged into a
//! `core`-typed [`CardView`] and run through the engine-free `deckmaste_plugin`
//! renderer, so the pane shows real rules text over the live (pumped, animated,
//! control-changed) object — not the printed encoding.
use std::fmt::Write as _;

use deckmaste_authoring::Expand;
use deckmaste_authoring::StatValue as AuthoredStatValue;
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

    // The renderer reads AUTHORED terms; `chars` is derived engine state and is
    // core-typed in every field. Raise it component-wise through the load-time
    // index rather than substituting the printed authored card — a pumped,
    // animated, type-changed object is exactly what this pane exists to show,
    // so the derived values are the ones that must survive.
    //
    // Every component has an authored preimage: abilities are printed
    // (`lower` of an authored one), granted (a verbatim clone, [CR#613.1f]) or
    // conferred (from the registry); subtypes and card types are clones of
    // registry entries; supertypes are a closed enum. A miss is not an error —
    // it drops the value, and the renderer's own `[unrendered]` marker shows.
    let supertypes: Vec<_> = chars
        .supertypes
        .iter()
        .copied()
        .map(raise_supertype)
        .collect();
    let types: Vec<_> = chars
        .card_types
        .iter()
        .filter_map(|t| provenance.index.type_def(t).cloned())
        .collect();
    let subtypes: Vec<_> = chars
        .subtypes
        .iter()
        .filter_map(|s| provenance.index.subtype(s).cloned())
        .collect();
    let abilities: Vec<_> = chars
        .abilities
        .iter()
        .filter_map(|a| provenance.index.ability(a).cloned())
        .collect();

    // Name and mana cost aren't derived characteristics. The cost comes from
    // the authored card via the companion table; a token has no authored card
    // and no cost, which is the `None` this resolves to. Power/toughness derive
    // to concrete numbers — lift them back into the `StatValue` `CardView`
    // wants, now the authored one.
    let authored = card_id(state, id).and_then(|c| provenance.cards.get(c));
    let authored_face = authored.map(|c| authored_front_face(c));
    let power = chars.power.map(AuthoredStatValue::Number);
    let toughness = chars.toughness.map(AuthoredStatValue::Number);
    let card_view = CardView {
        name: &printed.name,
        mana_cost: authored_face.map(|f| &f.mana_cost),
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
    // recursively expanded grammar instead of the authored macro spelling.
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
    detail_text(&card)
}

/// The card a rendered object came from, or `None` for a token or emblem.
fn card_id(state: &GameState, id: ObjectId) -> Option<deckmaste_engine::CardId> {
    match state.objects.obj(id).source {
        ObjectSource::Card(c) => Some(c),
        _ => None,
    }
}

/// The authored front face — the one whose printed cost the pane shows.
fn authored_front_face(card: &deckmaste_authoring::Card) -> &deckmaste_authoring::CardFace {
    match card {
        deckmaste_authoring::Card::Normal(f) => f,
        deckmaste_authoring::Card::TwoFaced { front, .. } => front,
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
            s.contains(&*face(state.def(id)).name),
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
            .find(|&id| &*face(state.def(id)).name == "Elvish Visionary")
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
            .find(|&id| &*face(state.def(id)).name == "Mogg Fanatic")
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
}
