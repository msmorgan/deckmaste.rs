//! Continuous statics over PLAYER attributes ([CR#611]) — the player-side twin
//! of the object layer engine in [`crate::layer`].
//!
//! No [CR#613] layer system applies to players, so these fold directly: a
//! battlefield permanent's `Static(ModifyPlayer(who, mod))` ([CR#611.3b]
//! statics function only on the battlefield) adjusts the affected player's base
//! attribute. The two canonical cards are Exploration
//! (`Raise(LandPlaysPerTurn, 1)`, [CR#305.2]) and Reliquary Tower
//! (`NoMax(HandSizeLimit)`, [CR#402.2]).
//!
//! Scope today: the controller parameter resolves to the source permanent's controller
//! (the "you" default both cards use); other player references and dynamic
//! (non-literal) `Count` magnitudes are documented seams that need the
//! resolve-time `ExecutionFrame` machinery (`engine-resolve-effects`).

use deckmaste_core::Ability;
use deckmaste_core::Count;
use deckmaste_core::Int;
use deckmaste_core::PlayerAttr;
use deckmaste_core::PlayerMod;
use deckmaste_core::Reference;
use deckmaste_core::StaticSpec;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::player::PlayerId;
use crate::state::GameState;

/// Evaluate a `ModifyPlayer` magnitude. Only `Count::Literal` is supported
/// here (the canonical cards are literal `1`s); a dynamic count is a documented
/// seam — it contributes `0` to a `Raise`/`Lower` and an unevaluated `0` to a
/// `SetTo`, which is the conservative no-op until the `ExecutionFrame`-bearing evaluator
/// is threaded in.
fn literal(count: &Count) -> Int {
    match count {
        Count::Literal(v) => (*v).cast_signed(),
        _ => 0,
    }
}

/// Resolve a `ModifyPlayer`'s affected-player reference. `You` is the source
/// permanent's controller ([CR#611.2c]); other references need the resolve-time
/// `ExecutionFrame` and are skipped (a documented seam).
fn resolve_player_ref(
    reference: &Reference,
    controller: PlayerId,
    region: &deckmaste_core::Region<StaticSpec>,
) -> Option<PlayerId> {
    matches!(reference, Reference::Reg(id)
        if region.provenance_of(*id) == Some(&deckmaste_core::Provenance::Controller))
    .then_some(controller)
}

impl GameState {
    /// Visit every battlefield `ModifyPlayer` static as `(affected, mod)`
    /// ([CR#611.3b] — statics function only on the battlefield). Effect sources
    /// are read from the CURRENT face's printed abilities — a back-up two-faced
    /// permanent sources its player-static from its BACK face
    /// ([CR#712.8e]) — flattened the same way the trigger and object-layer
    /// scans flatten composites (so a keyword-composite static is visited).
    /// Reading printed (not layer-derived) abilities keeps this cycle-safe;
    /// a `ModifyPlayer` granted by a layer-6 effect is a rare seam.
    fn for_each_player_mod(&self, mut visit: impl FnMut(PlayerId, &PlayerMod)) {
        for obj in self.objects.iter() {
            if obj.card_id().is_none() {
                continue; // player proxy — no static abilities
            }
            if obj.zone != Some(Zone::Battlefield) {
                continue;
            }
            let mut sources = Vec::new();
            for ability in &crate::derive::face_of(self, obj.id)
                .characteristics
                .abilities
            {
                crate::derive::flatten_composites(ability, &mut sources);
            }
            for ability in &sources {
                let Ability::Static(effect) = ability else {
                    continue;
                };
                // Conditional `ModifyPlayer` statics are a separate player-
                // modifier seam; object-characteristic conditionals are
                // handled by the layer gather. `ModifyPlayer` is never
                // distributed via `Each` (its `Reference` already names the
                // affected player directly), so only the top-level shape is
                // matched here.
                if let StaticSpec::ModifyPlayer(reference, pmod) = &effect.body
                    && let Some(p) = resolve_player_ref(reference, obj.controller, effect)
                {
                    visit(p, pmod);
                }
            }
        }
    }

    /// Fold every applicable `ModifyPlayer` static over `base` for `player`'s
    /// `attr`. Returns the effective value, or `None` when a `NoMax` op removed
    /// the cap ("you have no maximum hand size"). `NoMax` wins over numeric ops
    /// (a removed cap has no value); otherwise `SetTo` overwrites and
    /// `Raise`/`Lower` add and subtract.
    #[must_use]
    pub fn effective_player_attr(
        &self,
        player: PlayerId,
        attr: PlayerAttr,
        base: Int,
    ) -> Option<Int> {
        let mut value = base;
        let mut no_max = false;
        self.for_each_player_mod(|target, pmod| {
            if target != player {
                return;
            }
            match pmod {
                PlayerMod::SetTo(a, n) if *a == attr => value = literal(n),
                PlayerMod::Raise(a, n) if *a == attr => value += literal(n),
                PlayerMod::Lower(a, n) if *a == attr => value -= literal(n),
                PlayerMod::NoMax(a) if *a == attr => no_max = true,
                _ => {}
            }
        });
        if no_max { None } else { Some(value) }
    }

    /// The player's effective maximum hand size ([CR#402.2]), or `None` for "no
    /// maximum" (Reliquary Tower). Base is the player's stored cap (normally
    /// 7).
    #[must_use]
    pub fn effective_max_hand_size(&self, player: PlayerId) -> Option<Uint> {
        let base = Int::try_from(self.player(player).max_hand_size).unwrap_or(Int::MAX);
        self.effective_player_attr(player, PlayerAttr::HandSizeLimit, base)
            .map(|v| Uint::try_from(v.max(0)).unwrap_or(0))
    }

    /// The player's effective land plays per turn ([CR#305.2]), base one. A
    /// (nonsensical) `NoMax(LandPlaysPerTurn)` reads as unlimited.
    #[must_use]
    pub fn effective_land_plays_per_turn(&self, player: PlayerId) -> Uint {
        let v = self
            .effective_player_attr(player, PlayerAttr::LandPlaysPerTurn, 1)
            .unwrap_or(Int::MAX);
        Uint::try_from(v.max(0)).unwrap_or(0)
    }

    /// A player's numeric attribute as a `Count` magnitude ([CR#119.1] life,
    /// [CR#402.2] hand size) — the reader behind
    /// [`Count::PlayerStatOf`](deckmaste_core::Count::PlayerStatOf). `Life`
    /// ([CR#119.1]) reads the raw total; `HandSize` counts the cards currently
    /// in the player's hand; `HandSizeLimit`/`LandPlaysPerTurn` fold the
    /// player-static caps (an unbounded `NoMax` cap reads as `Uint::MAX`). A
    /// count is a non-negative magnitude ([CR#107.1b]), so a negative life
    /// total clamps to `0`.
    #[must_use]
    pub fn player_attr(&self, player: PlayerId, attr: PlayerAttr) -> Uint {
        match attr {
            PlayerAttr::Life => Uint::try_from(self.player(player).life.max(0)).unwrap_or(0),
            PlayerAttr::HandSize => {
                Uint::try_from(self.zones.hands[player.index()].len()).unwrap_or(Uint::MAX)
            }
            PlayerAttr::HandSizeLimit => self.effective_max_hand_size(player).unwrap_or(Uint::MAX),
            PlayerAttr::LandPlaysPerTurn => self.effective_land_plays_per_turn(player),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_card::Characteristics;
    use deckmaste_core::Ability;
    use deckmaste_core::Count;
    use deckmaste_core::PlayerAttr;
    use deckmaste_core::PlayerMod;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::Type;

    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// Put an enchantment carrying a single `ModifyPlayer` static onto the
    /// battlefield under `controller`.
    fn modify_player_permanent(state: &mut GameState, controller: PlayerId, pmod: PlayerMod) {
        use deckmaste_core::Zone;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Player Static".into(),
            types: vec![Type::Enchantment.def()],
            abilities: vec![Ability::r#static(StaticSpec::ModifyPlayer(
                Reference::Reg(deckmaste_core::RefId(1)),
                pmod,
            ))],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
    }

    /// Exploration: `Raise(LandPlaysPerTurn, 1)` lifts the controller to two
    /// land plays; opponents stay at the base one ([CR#305.2]).
    #[test]
    fn exploration_raises_land_plays_for_controller_only() {
        let mut state = game();
        modify_player_permanent(
            &mut state,
            PlayerId(0),
            PlayerMod::Raise(PlayerAttr::LandPlaysPerTurn, Count::Literal(1)),
        );
        assert_eq!(state.effective_land_plays_per_turn(PlayerId(0)), 2);
        assert_eq!(state.effective_land_plays_per_turn(PlayerId(1)), 1);
    }

    /// Reliquary Tower: `NoMax(HandSizeLimit)` removes the controller's cap
    /// (None = no maximum); opponents keep the base seven ([CR#402.2]).
    #[test]
    fn reliquary_tower_removes_max_hand_size_for_controller_only() {
        let mut state = game();
        modify_player_permanent(
            &mut state,
            PlayerId(0),
            PlayerMod::NoMax(PlayerAttr::HandSizeLimit),
        );
        assert_eq!(state.effective_max_hand_size(PlayerId(0)), None);
        assert_eq!(state.effective_max_hand_size(PlayerId(1)), Some(7));
    }

    /// [Task 5b][CR#712.8e]: a battlefield permanent showing its BACK face
    /// sources its `ModifyPlayer` player-static from the back face. A
    /// transforming DFC whose FRONT carries no player-static and whose BACK
    /// raises land plays is inert front-up ([CR#712.8d]) and lifts its
    /// controller to two land plays once back-up ([CR#712.8e]).
    #[test]
    fn back_up_permanent_sources_player_static_from_back_face() {
        use deckmaste_card::Card;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::Zone;

        use crate::object::Side;

        let front = CardFace::from(Characteristics {
            name: "Quiet Front".into(),
            types: vec![Type::Enchantment.def()],
            ..Characteristics::default()
        });
        let back = CardFace::from(Characteristics {
            name: "Exploring Back".into(),
            types: vec![Type::Enchantment.def()],
            abilities: vec![Ability::r#static(StaticSpec::ModifyPlayer(
                Reference::Reg(deckmaste_core::RefId(1)),
                PlayerMod::Raise(PlayerAttr::LandPlaysPerTurn, Count::Literal(1)),
            ))],
            ..Characteristics::default()
        });
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        };
        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        // Front-up: the front face carries no player-static — the base one land
        // play ([CR#712.8d]).
        assert_eq!(
            state.effective_land_plays_per_turn(PlayerId(0)),
            1,
            "front-up permanent contributes no ModifyPlayer static"
        );

        state.objects.obj_mut(id).side = Side::Back;

        // Back-up: the back face's Exploration-style static now applies
        // ([CR#712.8e]).
        assert_eq!(
            state.effective_land_plays_per_turn(PlayerId(0)),
            2,
            "back-up permanent sources its back face's ModifyPlayer static"
        );
    }

    /// Two Explorations stack additively ([CR#611] — independent continuous
    /// effects each apply): base one + two raises = three land plays.
    #[test]
    fn two_raises_stack_additively() {
        let mut state = game();
        for _ in 0..2 {
            modify_player_permanent(
                &mut state,
                PlayerId(0),
                PlayerMod::Raise(PlayerAttr::LandPlaysPerTurn, Count::Literal(1)),
            );
        }
        assert_eq!(state.effective_land_plays_per_turn(PlayerId(0)), 3);
    }
}
