---
needs: []
---
**Collapse the two spellings of a spell's cast window.** Residue of
`workbench-fused-residues` (2026-09-04), which gave `Effect.AbilityAt.Spell`
a positional `(window : Maybe (Timing bs))` ahead of its instruction so
"Cast this spell only before the combat damage step" reads (Berserk, Blood
Frenzy). The slot also admits `DuringPart p w`, which spells the same
sentence as the deontic static `Static (OnlyDuring p w (Macros.deontic This
Permit ["Cast"] Patient NoDeonticPatient))` that `teleport`, `festival` and
`dazzlingBeautyCastRestriction` use — a fused pair.

Fix: decide which spelling is the core row (the timing slot on `Spell` is
positional and RON-shaped; the deontic static restates a permission the CR
gives as a timing restriction [CR#307.1] for sorceries and as "cast only"
text otherwise), retire the other by re-spelling its three sites, and keep
`dazzlingBeautyCastRestriction` spellable (a bare cast-restriction fragment
with no instruction — decide whether that is a `Spell` with an empty body
or belongs elsewhere). If the deontic clause `staticOnSpellCardOk` loses its
last user, delete it. Pin the retired spelling, probed non-vacuous.

Size: S. Done when: one spelling remains; the three sites and Berserk read
through it; build at its module count. Standard constraints apply,
including the RON-shaped constraint.

## As landed

- **Which spelling is the core row** — the positional `window` slot on
  `Effect.AbilityAt.Spell`. The Comprehensive Rules state a printed cast
  window as something the spell itself says: "Some spells state that they may
  be cast 'only [before/after] [a particular point in the combat phase]'"
  [CR#506.7], and its subrules keep that voice. The
  deontic static restated that as a permission granted to a patient, which the
  rules never phrase; the timing slot is also the RON-shaped, positional row
  the previous round landed. Retired: `Static (OnlyDuring p w (Macros.deontic
  This Permit ["Cast"] Patient NoDeonticPatient))` on a spell card.
- **Re-spelling the sites** — three, all same card, same asserted outcome:
  `teleport` (`Cards/Turn.idr`) now reads `Spell (Just (DuringPart
  DeclareAttackers Nothing)) (Macros.cantBeBlocked …)`; `festival`
  (`Cards/Deontic.idr`) reads `Spell (Just (DuringPart Upkeep (Just
  Macros.anOpponent))) (Macros.cantAttack …)`; `dazzlingBeautyCastRestriction`
  (`Cards/Turn.idr`) is below. Berserk and Blood Frenzy already read through
  the slot and are untouched.
- **Where `dazzlingBeautyCastRestriction` lives** — it is no longer an
  `Ability`; it is the window itself, `dazzlingBeautyCastRestriction : Timing
  []` = `DuringPart DeclareBlockers Nothing`, the value the card's `Spell`
  takes once "target unblocked attacking creature becomes blocked" is
  spellable. Not a `Spell` with an empty body: an empty instruction sequence is
  already refused (`ProofsZone` pins `Sequentially []`), and inventing a body
  would not be the printed card. The fragment idiom is the bench's own
  (`bedrockTortoiseWindow : StaticSpec []`, `karnRestart : Instruction []`,
  `whileScrying : Concurrent []`).
- **`staticOnSpellCardOk`** — kept; it did not lose its last user. Its deontic
  clause did, and that clause is deleted:

      staticOnSpellCardOk (OnlyDuring _ _ (Deontic _ Permit deeds Patient _ _ _ _)) = …

  The function still gates `AltCost This`, `CostsToCast This`, `AddedCost` and
  `Deontic _ Forbid _ Patient` on instants and sorceries (Force of Will,
  Vexing Shusher and the whole `Cards/Static.idr` alternative-cost family).
  With the clause gone, the surviving `OnlyDuring _ _ se` recursion routes the
  retired spelling to the `Deontic` fall-through, so a spell card carrying it
  is refused.
- **Pin** — `ProofsFaces.badCastWindowAsDeonticStatic` (an instant whose text
  is the deontic cast-window static plus a `Spell`), with its twin
  `ProofsFaces.okCastWindowOnSpell` (the same sentence through the window
  slot) beside it. Probed non-vacuous.
- **No macro added.** The slot is positional and the sites write it directly;
  a `castOnlyDuring` wrapper would only re-order.

## Landing record

Measured on change `nkxxroywsprv` (this working copy), against parent
`pqrsotksvoqs` (`kata: claim workbench-cast-window-fused-pair`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructors (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`) | 794 | 794 |
| `Unspellable` pins | 615 | 616 |
| `: Card` bench witnesses | 825 | 826 |
| spellings of a spell's cast window | 2 | 1 |
| diffstat (code + lock) | — | 5 files, 26 insertions, 14 deletions |

No constructor added or deleted: the fold removes a clause of a `Bool` gate,
not a row.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0; 46 module lines; 0 lines matching `^Error` or
  `warning` (case-insensitive).
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14214 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` → registered the rule this round cites; pruned the
  combat-timing subrule the previous round cited, whose only remaining citation
  sites are `docs/tickets/done/workbench-fused-residues.md` and
  `docs/tickets/done/workbench-singleton-sorts.md`, an excluded path.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → 1 citation site,
  `[CR#506.7]`, read against the claim citing it.

**Assurance counts**

- restored: 0 (the tree was green at 46/46 at the start).
- re-spelled: 3 — `teleport`, `festival` and `dazzlingBeautyCastRestriction`,
  each the same subject with the same asserted outcome in the surviving
  spelling.
- ignored with a blocker: 0.
- added: 1 pin (`badCastWindowAsDeonticStatic`) and 1 positive twin
  (`okCastWindowOnSpell`).
- removed: 0.
- pin non-vacuity probes run (mis-state, watch the message change): 1 of 1.
  Swapping the deontic static for `Static (AltCost This Nothing)` — a static
  the gate still admits on a spell card — turns the pin into
  `badCastWindowAsDeonticStatic MkCharacteristicsLaws is not a valid
  impossible case.`

**Deviations and additions**

1. **`staticOnSpellCardOk` is kept, not deleted.** The ticket makes deletion
   conditional on losing its last user; it has many (every `AltCost This` /
   `CostsToCast This` spell card). Only its deontic `Permit` clause lost its
   last user, and that is what was deleted.
2. **`dazzlingBeautyCastRestriction` changes type, not name.** `Ability` →
   `Timing []`. Its subject — Dazzling Beauty's printed cast restriction —
   still exists and is still checked; the fragment is re-spelled at the sort
   that now holds a cast window. Its docstring's one line is updated in place.
3. **The pin is a card-level refusal, in `ProofsFaces`.** The retired spelling
   is refused by the card-class gate (`classAbilityOk SpellCard (Static …)`),
   so it is pinned beside `badStaticOnSorcery`, the other pin over that gate,
   rather than in `ProofsDeontic` (which pins the deontic constructor's own
   obligations) or `ProofsMana` (which holds the window-slot pins).
4. **Cite chosen is [CR#506.7], not [CR#307.1].** [CR#307.1] states the
   sorcery timing as a permission a *player* has, which argues the opposite
   way; [CR#506.7] is the rule whose subject is the spell stating its own cast
   window, which is the claim the pin makes.

**STOPs taken**

None.
