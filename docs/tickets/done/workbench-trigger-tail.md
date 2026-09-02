# workbench-trigger-tail

Trigger-side leftovers surfaced by the copy round (2026-09-02):

- **The once-each-turn rider** — "This ability triggers only once each turn"
  as a LIMIT rider on triggered abilities ([CR#603.2h]); 32 supported lines.
  Iron Man, Bleeding Edge and Donal, Herald of Wings are blocked on nothing
  else.
- **The ability pronoun** — `It` is `Noun bs Object`, so "if it isn't a mana
  ability" (5 of the 15 copy-that-ability lines) cannot read the ability
  mention; wants the pronoun opened at the Ability kind.

Re-measure at claim.

## As landed

Corpus authority: `data/derived/cards.jsonl` filtered
`jq 'select(.supported)'`, all counts re-measured 2026-09-02 with
reminder text stripped. CR text via `data/rules/`.

### 1. The once-each-turn rider [CR#603.2h] — landed

**A premise correction first.** The ticket quoted the rider as "This
ability triggers only once each turn" and counted it at 32. Those are
two different lines and the 32 belongs to the other one:

| printed line | supported lines | what it is |
| --- | --- | --- |
| "This ability triggers only once each turn" | 122 | `UsageLimit.OncePerTurn`, already landed |
| "Activate only … once each turn" | 125 | the same value at `Activated` |
| **"Do this only once each turn"** | **32** | [CR#603.2h], the item — landed here |

The named payoffs write the third, so the ticket's carriers were right
and only its quotation was wrong.

- `UsageLimit.ActionOncePerTurn`, in the slot `OncePerTurn` already
  occupies. A third VALUE and not a second slot, because it answers the
  same question the header asks — how often may this fire — and
  [CR#603.2h] states its answer as one about triggering: "this ability
  triggers only if its source's controller has not yet taken the
  indicated action that turn".
- It is NOT interchangeable with `OncePerTurn`, and the row says why:
  what [CR#603.2h] caps is the DEED the sentence before it offered, so a
  turn in which the controller declined the deed leaves the ability free
  to trigger again, where `OncePerTurn` stops the trigger either way.
- `untriggeredLimitOk` keeps it off `Activated` and `Intercepts`, the
  rule stating it of a triggered ability in as many words and 0
  supported lines writing it on either. Pin
  `badActionLimitOnActivated`.
- Measured covariances, recorded rather than gated: all 32 write the
  capped deed as a MAY (the rule requires no such thing), and 31 of the
  32 sit under `When`/`Whenever` — the thirty-second, Night Shift of the
  Living Dead, writes "After you roll a die", a word [CR#603.1] does not
  list and `TriggerWord` therefore does not carry.
- Bench: **Iron Man, Bleeding Edge whole** (`ironManBleedingEdge`, with
  `ExceptNonlegendary`) and **Donal, Herald of Wings whole**
  (`donalHeraldOfWings`, with an `ExceptChars` bundle). Both were
  blocked on nothing else, as the ticket said.

### 2. The ability pronoun — landed

Count holds at **5** of the 15 "copy that ability" lines: Battlemage's
Bracers, Harsh Mentor, Illusionist's Bracers, Kurkesh, Onakke Ancient,
Rings of Brighthearth.

`Noun.ItAbility`, gated `countOnes Ability bs = 1` — `It`'s own gate at
the other kind. [CR#109.1] is the warrant: an object IS "an ability on
the stack, a card, a copy of a card, a token, a spell, a permanent, or
an emblem", so the printed "it" reaches an ability for the same reason
it reaches a permanent. Macros `itsAnAbility` / `itIsntAnAbility`; the
stale ledger note in `Cards.idr` ("`It` is `Noun bs Object` … has no
carrier") is corrected. Bench: **Rings of Brighthearth whole**
(`ringsOfBrighthearth`).

**A SECOND ROW rather than a kind-polymorphic `It`, and this was
measured, not preferred.** Opening `It` to `{k : Kind}` under a
`So (bareItKind k)` gate was implemented first and is NOT VIABLE:
`Kind`'s join constructor `(\/)` makes the kind an unbounded search
space, so wherever the consuming term is itself kind-polymorphic
(`Macros.deontic`, `Macros.counterSpell`, `CopyStack`) the pronoun's own
kind is a metavariable and elaboration goes super-linear — the full
build was OOM-killed by the kernel at 92.5 GB RSS, reproducibly, and
`{k = Object}` annotations at the six flagged sites did not fix it. The
separate row costs sixteen table clauses, changes no call site, and
carries the same rules content in its docstring. **Recorded so the next
round does not retry it.**

### Remainder

None from this ticket. Both items landed with their named carriers
benched whole.
