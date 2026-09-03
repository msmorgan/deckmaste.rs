---
needs: []
---
**Restore the two lost bindings: a distributive `Does` must expose its
patient, and a player patient of `IsDealtDamage` must introduce a self
subject.** Cleanroom review 2026-09-03 F7, and audit-2 N1 / regression R-A.
Both are unintended refusals: a printed sentence stops being spellable.

## Distributive `Does` exposes no patient (N1, R-A)

`Effect.doesEffIntro ManyOf s v e = deedDelta e ++ nomIntro s` and
`Effect.deedDelta (Move what to _) = []`, so after `discard (each Opponent) (a
card)` no read reaches the discarded cards. The singular branch does the work:
`doesEffIntro OneOf s v (Move what to _) = afterMoveTo to (moveIntro (Just v)
what …)` stamps and exposes the moved noun. Probe: `Sequentially [discard
(each Opponent) (a (InZone handZ)), exile You (theVerbed "Discard" CardW)]`
typechecks on `szkmlzqnkysz-` and is refused on the current tree (`Can't find
an implementation for countReach (Verbed "Discard" CardW Attributive) OneOf
(effIntro (discard (each Opponent) …)) = 1`); the plural read `thoseVerbed` is
refused on both, so a distributive deed cannot be read back at all. The
fourteen bench `Does (each …)` sites have no follow-on read, which is why the
gates are green.

Fix: give the `ManyOf` branch the same `Move`/`SetStatus` clauses with the
moved noun's plurality lifted to `ManyOf` — a distributive deed moves
one-per-agent and reads back as "those cards" / "each card discarded this way"
— i.e. `doesEffIntro ManyOf s v (Move what to _) = afterMoveTo to (setZoneHead
… (pluralise (moveIntro (Just v) what …)))`. Add Syphon Mind's second sentence
("You draw a card for each card discarded this way") as the printed witness,
and a pin that the *singular* read is refused after a plural agent.

## Player patients of `IsDealtDamage` (F7)

`Triggers.idr:396–398`: `eventAfter (IsDealtDamage {k = Object} _ to) = …
selfSubjIntro to`, but the fall-through `eventAfter (IsDealtDamage _ to) = …
nomIntro to`, while `eventIntro` (`Triggers.idr:346`) uses `selfSubjIntro` for
every kind. `selfSubjDelta (AttachHost _ PlayerW)` (`Phrase.idr:2658`) is what
makes "enchanted player" readable as `They`; the player branch skips it.
Probe (Grievous Wound, VINTAGE — "Whenever enchanted player is dealt damage,
they lose half their life, rounded up"): `triggered Whenever (IsDealtDamage
AnyDamage (AttachHost Enchanted PlayerW)) (losesLife They (Half RoundUp
(PlayerStatOf LifeTotal They)))` → `Can't find an implementation for
countReach (Word PlayerW) OneOf (eventAfter (IsDealtDamage AnyDamage
(AttachHost Enchanted PlayerW))) = 1`; repeating the full noun typechecks.
This is why the card's trigger is absent from the bench.

Fix: `selfSubjIntro` in both branches.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; Syphon Mind's
second sentence and Grievous Wound's damage trigger are bench witnesses that
typecheck through the pronoun read (not by repeating the noun); the pin that
the singular read is refused after a plural agent is present and non-vacuous;
no existing pin's refusal changes reason. Standard constraints apply.
