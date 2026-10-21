---
needs: []
---
# Untap-step statics: Templates named for their printed wording, built on `untap` and the deontic Templates

Standard constraints apply. This takes over the `doesntUntap` item of
[lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §2 ("raw `setStatus
.untapped` in `doesntUntap`").

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes (the semantic expression types in
  `Words.lean`, `Events.lean` and `Abilities.lean`) or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

## Rulings (owner, 2026-10-21)

1. Split `untapsDuring` (`Definitions/Actions.lean:565-567`) into one Template for the
   transitive wording ("Untap all permanents you control during each other player's untap step":
   *Seedborn Muse*, *Unwinding Clock*, *Drumbellower*) and one for the intransitive wording
   ("This creature untaps during each other player's untap step", which the ruling attributes to
   *Thousand Moons Infantry*).
2. Align `doesntUntap` and `mayDeclineUntap` with it.
3. Review the Templates declared in `Definitions/Actions.lean`. Move into the Template layer
   (`Templates.lean`) only those whose bodies need no composite Definition, as CONTRACTS.md's
   layering requires ("Card authoring and trusted expansion"). Write the standard `must`/`can`
   Templates (`Templates.lean:995-1006`) instead of raw `.must .any … {}` and `.can .any … {}`.

## Correction to ruling 1 (verified 2026-10-21 against Oracle)

*Thousand Moons Infantry* prints "Untap this creature during each other player's untap step." That
is the transitive imperative, the same wording as the other three. No Vintage-legal card prints
"<permanent> untaps during … untap step". The only intransitive "untaps during" in Oracle text is
phasing's reminder ("before you untap during each of your untap steps", *Shimmering Efreet*). There
the subject is a player, and it describes the untap step itself [CR#702.26a,502.3].

So this ticket:
- renames `untapsDuring` to the printed imperative, for example `untapDuring`;
- adds no intransitive Template, because nothing prints that wording.

If a card printing the intransitive form enters the bench, it gets its own Template over the same
tree. Under the governing principle, the wording alone puts no distinction in the Core AST.

**Owner ruling (2026-10-21) on this correction:** accepted. `untapsDuring` becomes
`untapDuring`, and no intransitive Template is added.

## Change

- **The untap Deed.** `doesntUntap` (`Templates.lean:1398-1399`) writes
  `cant any (.setStatus .untapped subject)`. That is a raw status change, not the untap Keyword
  Action [CR#701.26b]. Move it to the "Templates over Definitions" section of
  `Definitions/Actions.lean` and write `cant any (untap subject)`, matching ADR §3.7's "doesn't
  untap" row. A Template there can name the `untap` Definition (`Definitions/Actions.lean:153`);
  one in `Templates.lean` cannot.
- **Standard deontic Templates.** Replace raw constructors with the `can` and `must` Templates:
  - `mayDeclineUntap` (`:562-564`): its raw `.can .any … {}` goes, and its tree is rewritten as
    the `mayDeclineUntap` section below says;
  - `untapDuring` (`:565-567`): `.must .any … {}` becomes `must any …`;
  - `mayBeAttachedTo` (`:568-572`): `.can .any … {}` becomes `can any …`.
- **Layering review.** All 12 Templates declared in `Definitions/Actions.lean` read a composite
  Definition, so none moves to `Templates.lean`:

  | Template | Line | Definition it reads |
  |---|---|---|
  | `destroyCantRegenerate` | `:139` | `destroy` |
  | `exileUntil` | `:543` | `exile` |
  | `attachToIt` | `:546` | `attachTo` |
  | `controllerSacrifices` | `:550` | `sacrifice` |
  | `discardThenForEachNonland` | `:555` | `discard` |
  | `mayDeclineUntap` | `:563` | `untap` |
  | `untapsDuring` | `:566` | `untap` |
  | `mayBeAttachedTo` | `:570` | `attachTo` |
  | `createToken` | `:573` | `create` |
  | `createTappedToken` | `:576` | `create` |
  | `createTappedAttacking` | `:580` | `create` |
  | `createTokenCopy` | `:583` | `create` |

  Record that outcome rather than moving anything. `doesntUntap` moves the other way, into
  `Definitions/Actions.lean`.

## `mayDeclineUntap` means permission to leave the permanent tapped (owner ruling, 2026-10-21)

**Defect.** `mayDeclineUntap` expands to a permission to untap: `can any (untap X)` during your
untap step. Untapping is already the default, so the permission adds nothing. The printed meaning
is the opposite: a permission for the active player to leave X tapped. Cards printing it include
*Ashnod's Battle Gear*, *Helm of Possession*, *Amber Prison* and *Rust Tick*.

**Ruling.** Fix it in this ticket, so the tree means permission to leave the permanent tapped.

**Rule.** [CR#502.3] supports the ruling (checked against `data/rules/cr.txt`): "the active player
determines which permanents they control will untap … effects can keep one or more of a player's
permanents from untapping". So the untap step's untapping is a determination the active player
makes, which a "may choose not to untap" effect widens. The CR has no rule specific to "may
choose not to untap". [CR#701.43b] only mentions effects "causing it not to untap", for exert.

**Change.** Choose the tree. It must meet three conditions:
1. untapping stays the default;
2. the active player gets the option to keep X tapped during their untap step [CR#502.3];
3. it is not a permission to untap.

Candidates, in order of preference:
- a prohibition that applies only if the active player so chooses: `cant any (untap X)` within
  `duringUntapStep`, conditioned on that player's choice;
- a permission over the existing `skipUntap` action (`Abilities.lean:1150`), if one step's skip
  reads as that choice.

If neither fits without a new constructor, add the smallest one and say why in the landing. Keep
the printed wording on the `mayDeclineUntap` Template.

## Sites

Line numbers verified 2026-10-21.

- `untapsDuring` → `untapDuring`:
  - *Seedborn Muse* `Cards/Static.lean:481`
  - *Unwinding Clock* `:489`
  - *Thousand Moons Infantry* `:497`
  - *Drumbellower* `Cards/Keyword.lean:690`
- `doesntUntap`:
  - *Apathy* `Cards/Turn.lean:88`
  - *Time Vault* `Cards/Static.lean:35`
  - *Bombur, Gentle Dreamer* `Cards/Keyword.lean:269`
  - the pin `Proofs/Damage.lean:119`
- `mayDeclineUntap`:
  - *Ashnod's Battle Gear* `Cards/Choice.lean:188`
  - *Helm of Possession* `:869`

[lean-v3-another-contrast-anchor](lean-v3-another-contrast-anchor.md) rewrites the "other
player" predicate on the four `untapsDuring` card lines. Whichever ticket lands second rebases.

## Tests

- The `ok…` theorems for the sites above, and `Proofs/Damage.lean:119`, stay green under
  `./lean/scripts/build` (`--wfail`).
- **Add** a pin that `doesntUntap`'s expansion forbids the `untap` Definition's Deed, so a "can't
  untap" pattern and "doesn't untap during its controller's untap step" constrain the same Deed.
- **Add** a pin that the four untap-step cards expand through `must`. Assert the Template is
  used, not a restatement of the tree.
- **Add** a pin that `mayDeclineUntap X` no longer expands to a permission to untap X. Assert
  that it gives the active player the option to keep X tapped, and that *Ashnod's Battle Gear*
  and *Helm of Possession* still check.
