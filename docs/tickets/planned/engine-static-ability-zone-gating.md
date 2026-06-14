---
needs: []
---
[design] Uniform static-ability zone-gating so every GameObject is handled
identically — no engine special-case for "only permanents have (functioning)
static abilities."

Static-ability functioning is battlefield-default for non-instant/sorcery
objects ([CR#611.3b,604.2,113.6]). **Proposal: make "the zone(s) an ability
functions in" a first-class property of every ability** — the [CR#113.6]
taxonomy *is* that vocabulary — rather than the engine asking "is this a
permanent?" out of band. The rules text usually *implies* the zone ("as long as
~ isn't on the battlefield", "from your graveyard", an emblem's abilities), so
the parser can frequently infer the function-zone; authors set it explicitly
only when needed. Then every GameObject reads the same shape and a single
generic check ("is the object in one of this ability's function-zones?") decides
functioning. This is the static-ability analogue of how basic lands carry their
mana ability through `confers:` rather than an engine special-case.

"Battlefield" is just the default value of the new property; the familiar
`Is(Permanent)` gate is the battlefield case expressed in this vocabulary. The
zone-crossing cases are then NOT exceptions in engine code — they're just
abilities with a different function-zone value, e.g. **Grist, the Hunger Tide**
= `everywhere-except-battlefield`, a graveyard static = `graveyard`, an emblem =
`command`, a CDA = `all-zones`. Reference points for the vocabulary:

- CDAs — function in all zones ([CR#604.3,113.6a]).
- Zone-declaring statics — function only-in / everywhere-except stated zones
  ([CR#113.6b,113.6c]). **Grist, the Hunger Tide** is the canonical case ("As
  long as Grist isn't on the battlefield, it's a 1/1 Insect creature in
  addition to its other types") — it functions precisely when the object is
  NOT a permanent, so it must NOT get the `Is(Permanent)` gate. (Note: Grist's
  is conditional, so it functions via [CR#113.6c], not the strict-CDA
  [CR#113.6a]; [CR#604.3a] criterion 5 excludes conditional setters from being
  true CDAs.)
- Cost / cast-modifying / can't-be-countered statics on the stack
  ([CR#113.6d,113.6e,113.6g]); play-zone & which-zones-castable-from and
  pre-game / deck-construction ([CR#113.6f,113.6n]); enters-modifying
  ([CR#113.6h,113.6i]); command-zone emblems/plane/vanguard/scheme/conspiracy
  ([CR#113.6p,114.4]).

Design questions for the dialogue:
- Where the gate is injected: authoring/keyword macro vs a derive-time wrap vs
  the layer gather.
- The zone-predicate vocabulary: a bare `Is(Permanent)` vs a richer
  "functions-in-zone(s)" declaration that mirrors the [CR#113.6] taxonomy (so
  hand/graveyard/stack/command statics are first-class, not just battlefield).
- How it composes with the `Innate` wrapper (from engine-attach — Innate
  statics are subtype rules immune to ability-removal; do they also need the
  zone gate, or is "functions on the battlefield" implied?) and with the
  layers system (gather already restricts static abilities to the battlefield
  at `layer.rs` ~:315 — `[CR#611.3b]`; this proposal would move that decision
  into the data).
- Migration of existing authored statics.

Needs a design dialogue before implementation (design-pause boundary).

## Dependent: Reconfigure [CR#702.151b] type-suppression (deferred from engine-attach)

engine-attach built Reconfigure's two activated abilities (attach / unattach) but
**seam'd** the "this Equipment isn't a creature while attached to a creature"
static — it needs the machinery this ticket designs, plus one more primitive:

1. **Conditional static evaluation in the layer `gather`.** Today `gather`
   constructs `ActiveEffect`s from a `StaticAbility` WITHOUT consulting
   `sa.condition` (the [CR#611.3a] seam), so a conditional static (`Is(Ref(This),
   AttachedTo(Creature))` → not-a-creature) would apply *unconditionally* —
   making the Equipment non-creature even while unattached (an active bug). Honoring
   static `condition`s in gather is squarely this ticket's territory.
2. **A remove-card-type `Modification`** (e.g. `RemoveCardType(Creature)`): layer 4
   has add-type / `AllCreatureTypes` ops but no "stop being a creature" op. Small
   addition, do it alongside.

Once both exist, Reconfigure's suppression is just an authored conditional static on
the keyword macro: `Static(condition: Is(Ref(This), AttachedTo(Creature)),
effects: [Modify(of: Of(Ref(This)), changes: [RemoveCardType(Creature)])])`. The
`#[ignore]`'d `reconfigure_suppresses_creature` test in engine-attach pins the
intended behavior.

Surfaced during `engine-attach` (the attachment SBAs/legality + the `Innate`
mechanism made the "treat all GameObjects the same" question concrete; Reconfigure's
suppression is the concrete first customer of conditional-static gating). Not
blocking engine-attach.
