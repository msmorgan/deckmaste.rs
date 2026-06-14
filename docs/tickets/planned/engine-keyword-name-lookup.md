---
needs: []
---
`combat.rs:39` `has_keyword_named` re-walks an object's derived abilities by hand and
matches keyword names WITHOUT the `Ability::Expanded` look-through, so an
Expanded-wrapped keyword (e.g. lifelink granted via a composite/static) is missed in
the combat damage path. Replace its body with
`view.get(id).abilities.iter().any(|a| layer::ability_is_named(a, name))`, reusing the
canonical per-ability name matcher (`layer.rs:470`) — dedups and fixes the latent miss.
