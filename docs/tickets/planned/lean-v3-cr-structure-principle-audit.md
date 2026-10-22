---
needs: [lean-v3-resolution-riders-and-guards, lean-v3-keyword-triggers-entries-and-parties]
---
# Audit earlier rulings and new Core constructors against the CR-structure principle

Standard constraints apply. The principle is `docs/decisions/semantics-v3.md` §0.1 item 5 (owner
ruling 2026-10-22), which refines the 2026-10-21 line that a Core AST distinction exists only
where the CR or the checker's anaphora resolution distinguishes it.

**This is an inventory to do, not a ruling.** It records nothing as decided beyond the four seed
rows below, which the owner has already ruled. Every new row it finds goes to the owner as a
proposed class before any change lands.

## What to audit

Read the ADR's rulings (every dated owner ruling in `semantics-v3.md`, and those it cites from
`lean/CONTRACTS.md`), and every Core constructor added to `lean/Semantics/Words.lean`,
`Events.lean` and `Abilities.lean` since `lean-v3-proofs-and-cards`. Flag each place that breaks
one of the principle's three parts:

- **(a) A CR structural distinction was dropped.** The CR distinguishes two rules structures
  (an intervening "if" from a plain condition [CR#603.4], a replacement from a trigger
  [CR#614.1,603.1], an additional from an alternative cost [CR#118.8,118.9], a turn-based action
  from an effect [CR#703.1]), and the AST encodes both as one, on the theory that no card
  notices.
- **(b) A one-off constructor could be a composition.** A constructor that exists for one CR
  definition, where existing primitives plus a Template carrying the printed wording would say
  the same thing.
- **(c) An AST workaround for a runtime check.** A node, field or variant whose only reason is to
  satisfy a checker rule that simulates runtime state (§0.1 item 4). The fix is to remove the
  check, not to keep the workaround. The inventory of such checker rules is
  [lean-v3-checker-runtime-state-audit](lean-v3-checker-runtime-state-audit.md); this ticket
  finds the AST side of them.

For each finding, give the site, the part it breaks, the CR rule it rests on (numbers from
`data/rules/cr.txt`), the cards that witness it, and a proposed fix.

## Known cases (resolved)

| Case | Part | Resolution |
|---|---|---|
| `ProjAxis.printed` | (b), (c) | Collapsed (owner ruling 2026-10-22, `lean-v3-cr-structure-principle`). The planeswalker and battle entry conferrals [CR#306.5b,310.4b] read `statOf (.stat .loyalty\|.defense) .this`; off the battlefield that already is the printed number [CR#306.5a,310.4a]. The projection existed only because the read was taken as timing-dependent. |
| `Duration.forThisEvent` | (b) | Renamed in `lean-v3-resolution-riders-and-guards` to `Duration.modifiesPrecedingAction`, a marker that later text modifies the preceding action [CR#608.2c], not a new kind of duration. |
| The countered ability | (a) | Ruled B (owner ruling 2026-10-22, `lean/CONTRACTS.md` "Movement destinations"): Counter stays one stack-to-graveyard enactment for a spell or an ability, and the 2026-09-06 ruling stands. An ability on the stack stops existing once it leaves its home zone, as a token does [CR#608.2n,704.5d], so engine consistency beats a subject-dependent case for [CR#701.6a]. `lean-v3-countered-ability-ceases` is not done. |
| Gift's intervening "if" | (a) | Kept. Gift's permanent half is "When this permanent enters, if its gift cost was paid, [effect]" [CR#702.174b], and the give-a-gift triggers [CR#702.174c] (*Jolly Gerbils*) depend on it; `lean-v3-keyword-triggers-entries-and-parties` §5 encodes the gift cost as the trigger's intervening "if". |
