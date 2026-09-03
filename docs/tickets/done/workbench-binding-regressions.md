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

## As landed

- N1/R-A: `Effect.doesEffIntro` gains `ManyOf` `Move` and `SetStatus` clauses mirroring the `OneOf` ones, with the moved noun's delta lifted to `ManyOf` through the new `Effect.distributedDelta` (`pluralizeDelta` over the prefix of `moveIntro`/`stampIntro` above `agentIntro s`) and `++ nomIntro s` as the tail; `DoesGroup` was not resurrected and the `ManyOf` fall-through is unchanged.
- N1/R-A bench: `Cards.syphonMind` is now both printed sentences — `Sequentially [discard (each otherPlayer) (a (InZone handZ)), ForEachOf (thoseVerbedThisWay "Discard" CardW) (Draw You (Lit 1))]`.
- N1/R-A proofs: witness `ProofsAnaphora.distributedDeedReadsBackPlural` (the audit's R-A probe term, plural read) and pin `ProofsAnaphora.badDistributedDiscardSingular` (the same sentence with the singular read).
- F7: `Triggers.eventAfter`'s two `IsDealtDamage` clauses fold into one `outcomeB DamageDealt :: selfSubjIntro to`, matching `eventIntro`.
- F7 bench: `Cards.grievousWoundLifeLock` gains its printed damage trigger, reading the patient back as `They`.
- F7 proofs: witness `ProofsAnaphora.enchantedPlayerDamageReadsBackAsThey` (the cleanroom probe term).
- Undone, outside the letter: `doesPreIntro`/`doesAnnIntro`/`doesRiderIntro` keep their `ManyOf` fall-throughs, so an `InsteadOf`/`ThisWay`/rider read of a distributive patient is still refused. The ticket names `doesEffIntro` only.

## Landing record

Numbers before/after:

- modules 23/23 → 23/23, 0 Error and 0 Warning both.
- `Unspellable` pins in `Proofs*` 568 → 569; `ProofsAnaphora` defs +3 (2 witnesses, 1 pin).
- `Cards.idr` defs 1564 → 1564; `syphonMind` 1 printed sentence → 2; `grievousWoundLifeLock` 2 abilities → 3.
- `eventAfter` `IsDealtDamage` clauses 2 → 1; `doesEffIntro` clauses 4 → 6, plus `Effect.distributedDelta`.

Gates (foreground):

- `cd idris && rm -rf build && ./scripts/build` → `23/23: Building Cards (src/Cards.idr)`, no Error and no Warning lines.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` → `checked 17719 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/br.diff && cargo xtask cite audit --diff < /tmp/br.diff` → `audited 0 citation site(s) — nothing selected`; the diff makes no CR claim, `cite bless` was not run and `cr-citations.lock` is untouched.

Probes (each applied in place, message read, then reverted):

- `badDistributedDiscardSingular` mis-stated to the plural read → `Error: badDistributedDiscardSingular Refl is not a valid impossible case.` Non-vacuous.
- `doesEffIntro`'s `ManyOf` `Move` clause reverted → `distributedDeedReadsBackPlural` fails with `Can't find an implementation for countReach (Verbed "Discard" CardW Attributive) ManyOf (effIntro (discard (each Opponent) (a (InZone handZ)))) = 1`, the audit's R-A message.
- `eventAfter`'s fold reverted → `enchantedPlayerDamageReadsBackAsThey` fails with `Can't find an implementation for countReach (Word PlayerW) OneOf (eventAfter (IsDealtDamage AnyDamage (AttachHost Enchanted PlayerW))) = 1`, the cleanroom F7 message.
- `badDistributedMillSingular` re-probed (`It` → `Them`) → impossible clause rejected: after the intro change the plural read of a distributed mill is spellable and only the singular is refused, so the pin's reason is unchanged.
- The five `TheVerbed` pins re-probed and each rejected `impossible` once mis-stated: `badVerbedWrongVerb`, `badVerbedWrongNoun`, `badVerbedAmbig`, `badDiscardedCreatureWord` (`Proofs.idr`) and `badCostReadsSiblingDeed` (`ProofsC.idr`, whose hole is `costNounOk (Pro (Verbed _ _ _) _) = False`). All five take a singular agent and are outside the change's blast radius.

Assurance counts: restored 0, re-spelled 0, ignored 0, added 3 in `Proofs*` (2 witnesses, 1 pin) plus 2 printed bench sentences, removed 0.

Deviations and additions:

- `Effect.distributedDelta` is an addition beyond the ticket's letter: the ticket sketched the pluralised move inline, and the two new clauses share one helper spelled in `effDelta`'s `take … minus length bs` idiom instead.
- The new pin carries no CR citation. The refusal is anaphoric uniqueness after a distributive agent, not a sentence the rules make meaningless; its sibling `badDistributedMillSingular` carries none either.
- Syphon Mind's second sentence uses `ForEachOf (thoseVerbedThisWay …)`, the idiom already in `martyrsCry` and `descentOfTheDragons`, rather than a new macro.

STOP taken: none.
