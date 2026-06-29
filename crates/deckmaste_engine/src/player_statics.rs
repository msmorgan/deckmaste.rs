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
//! Scope today: `Reference::You` resolves to the source permanent's controller
//! (the "you" default both cards use); other player references and dynamic
//! (non-literal) `Count` magnitudes are documented seams that need the
//! resolve-time `Frame` machinery (`engine-resolve-effects`).

use deckmaste_core::Ability;
use deckmaste_core::Count;
use deckmaste_core::Int;
use deckmaste_core::PlayerAttr;
use deckmaste_core::PlayerMod;
use deckmaste_core::Reference;
use deckmaste_core::StaticEffect;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::player::PlayerId;
use crate::state::GameState;

/// Evaluate a `ModifyPlayer` magnitude. Only `Count::Literal` is supported
/// here (the canonical cards are literal `1`s); a dynamic count is a documented
/// seam — it contributes `0` to a `Raise`/`Lower` and an unevaluated `0` to a
/// `SetTo`, which is the conservative no-op until the `Frame`-bearing evaluator
/// is threaded in.
fn literal(count: &Count) -> Int {
    match count {
        Count::Literal(v) => (*v).cast_signed(),
        _ => 0,
    }
}

/// Resolve a `ModifyPlayer`'s affected-player reference. `You` is the source
/// permanent's controller ([CR#611.2c]); other references need the resolve-time
/// `Frame` and are skipped (a documented seam).
fn resolve_player_ref(reference: &Reference, controller: PlayerId) -> Option<PlayerId> {
    match reference {
        Reference::You => Some(controller),
        _ => None,
    }
}

impl GameState {
    /// Visit every battlefield `ModifyPlayer` static as `(affected, mod)`
    /// ([CR#611.3b] — statics function only on the battlefield). Effect sources
    /// are read from PRINTED abilities (cycle-safe; a `ModifyPlayer` granted by
    /// a layer-6 effect is a rare seam), flattened the same way the trigger and
    /// object-layer scans flatten composites (so a keyword-composite static is
    /// visited).
    fn for_each_player_mod(&self, mut visit: impl FnMut(PlayerId, &PlayerMod)) {
        for obj in self.objects.iter() {
            if obj.card_id().is_none() {
                continue; // player proxy — no static abilities
            }
            if obj.zone != Some(Zone::Battlefield) {
                continue;
            }
            let mut sources = Vec::new();
            for ability in crate::derive::printed_abilities(self, obj.id) {
                crate::derive::flatten_composites(ability, &mut sources);
            }
            for ability in &sources {
                let Ability::Static(sa) = ability else {
                    continue;
                };
                // Conditions skipped — the same seam the object-layer gather has.
                for effect in &sa.effects {
                    let StaticEffect::ModifyPlayer(reference, pmod) = effect else {
                        continue;
                    };
                    if let Some(p) = resolve_player_ref(reference, obj.controller) {
                        visit(p, pmod);
                    }
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
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Card;
    use deckmaste_core::CardFace;
    use deckmaste_core::Count;
    use deckmaste_core::PlayerAttr;
    use deckmaste_core::PlayerMod;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;
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
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        })
    }

    /// Put an enchantment carrying a single `ModifyPlayer` static onto the
    /// battlefield under `controller`.
    fn modify_player_permanent(state: &mut GameState, controller: PlayerId, pmod: PlayerMod) {
        use deckmaste_core::Zone;
        let card = Card::Normal(CardFace {
            name: "Test Player Static".into(),
            types: vec![Type::Enchantment],
            abilities: vec![Ability::Static(StaticAbility {
                condition: None,
                from: None,
                effects: vec![StaticEffect::ModifyPlayer(Reference::You, pmod)],
                characteristic_defining: false,
            })],
            ..CardFace::default()
        });
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
