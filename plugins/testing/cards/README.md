# plugins/testing — mocks for combos real Magic never printed

Engine tests use real cards from `plugins/canon` (see `docs/card-data.md`).
A fake belongs here ONLY when the behavior under test exists in no real card —
verified against the full corpus, not vibes. Current residents:

- **Trample Deathtouch Creature** — no creature ever printed carries exactly
  {trample, deathtouch}; the pair pins the [CR#702.2c] lethal-is-one trample
  split.
- **Trample granter** — symmetric "all creatures have <intrinsic keyword>"
  statics don't exist (real ones are "you control"-scoped, and controller
  relations aren't evaluated in layers yet; the one symmetric grant in the
  game, Mass Hysteria's haste, isn't an intrinsic `KeywordAbility`).
- **Animate enchantments** — no real static type-REPLACES with a literal P/T
  set ("X is still a Y" adds; Opalescence-style sets P/T to mana value, which
  needs count evaluation in layers). Pins the [CR#613.6] layer-4 scope lock.

If a mechanic lands that makes one of these encodable with a real card,
canonize the real card and delete the mock.
