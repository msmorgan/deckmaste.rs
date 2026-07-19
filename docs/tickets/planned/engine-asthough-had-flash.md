---
needs: [core-asthough-glaring-spotlight]
---
**Implement the POSITIVE `AsThough` counterfactual — "as though they had
[keyword]", seeded by flash-granters.** The `AsThough::Counterfactual { premise,
then }` primitive and the *negative* (remove-a-keyword) overlay landed with
`core-asthough-glaring-spotlight` (Glaring Spotlight: `premise = Not(Has(Hexproof))`
suppresses hexproof's `Cant(Target)` for the targeting check). This ticket adds
the *positive* direction: `premise = Has(Flash)` grants a scoped set of spells
instant-speed casting **by invoking the Flash keyword**, not by inlining its row.

## The seed cards

- **Leyline of Anticipation** / **Vedalken Orrery**: "You may cast spells as
  though they had flash." — a permanent static `AsThough(Has(Flash),
  May(Cast(what: <your nonland spells>, by: Ref(You))))`. Note `then` carries NO
  `window: InstantSpeed` — the timing comes from the counterfactually-added Flash
  keyword's own row.
- **Alchemist's Refuge** (current Oracle — the printed land-play clause was
  errata'd out; verify with the mtg-rules `scripts/card`): `{T}: Add {C}` plus
  `{G}{U}, {T}:` → `Until(EndOfTurn, [AsThough(Has(Flash), May(Cast(<your
  spells>)))])` — the activated, duration-scoped twin of Leyline's static.

## Why this is NOT an inline `May(Cast(window: InstantSpeed))`

Authoring the granter as a hand-written `May(Cast(window: InstantSpeed))` row
INLINES Flash's definition — it never invokes the Flash keyword ability,
duplicates the keyword's meaning (breaks single-source-of-truth), and reads as a
global timing permission rather than a per-checker counterfactual. Ruling: an
"as though [keyword]" effect must be the counterfactual overlay that INVOKES the
keyword. The ONE legitimate `May(Cast(InstantSpeed))` author is the Flash keyword
macro itself (`plugins/builtin/macros/keyword/Flash.ron`, `what: Ref(This)`).

## The blocker this ticket must clear (engine infrastructure)

The negative direction is cheap: the keyword's ability is already on the object,
so the overlay *masks* it (`legal::masked_self_rows_forbid`). The positive
direction needs the engine to answer "what does the Flash keyword *do*?" so it
can ADD Flash's ability to the candidate spell — and today it can't:
`derive::composite_members` only reads an ALREADY-EXPANDED `Composite`, keyword
macros expand at card-load time in the cards crate, and `GameState` carries
subtype/type registries but **no keyword→ability registry**. So this ticket must
first add a **runtime keyword-definition registry to `GameState`** (populated
from the loaded plugin, mirroring the subtype/type registries), then a resolver
the overlay uses to materialize the premise keyword on the candidate.

## Shape (extends the landed seam)

- Engine: a cast-timing overlay symmetric to the targeting one — in
  `castable_cost_ignoring_mana`'s timing check (`cast.rs` ~800, where
  `may_cast_rows` is consulted), when an active `AsThough(Has(K), May(Cast{by,
  what}))` selects the spell, materialize K's ability on the candidate and
  re-collect its `may_cast_rows` so K's own row lifts timing. Reuse the
  `astough_*_rows` collector pattern from `legal.rs`.
- The overlay realization generalizes `premise`: `Not(Has(K))` → mask K (done);
  `Has(K)` → add K (new, needs the resolver).
- Render arm for the positive premise ("as though they had flash") in
  `render/ability.rs::asthough_counterfactual` (the negative arm is in place).
- Idris re-emit already handles it generically (`AsThough (Matches This (Has
  Flash)) (Can (Enact Cast ...))` via the landed `emit_static_effect` arm).
- Semantic test: a sorcery-speed spell is uncastable at instant speed normally,
  castable under the granter, and still sorcery-speed for a player without it.

Standard constraints apply.
