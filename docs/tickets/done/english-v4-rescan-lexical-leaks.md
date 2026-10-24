---
needs: []
---
# Re-scan: lexical leaks and census deferral hygiene

Implements group C of the post-S7 re-scan (Argon, 2026-10-24). Each finding is a hypothesis; verify it by its witness before changing anything. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md).

## Change

- **MEDIUM-3.** The `for` + AdjP complement frame (for *tied for most common*) leaks into every `Prepositional(["for"])` verb (*votes for [AdjP wild or free]*). Restrict it to its hosts. Vote words (*bribery, evidence, fellowship, guilty, innocent, wild, free* …) are in both `vote_options.ron` and the general nouns/adjectives; remove the general entries that no non-vote face uses. Witnesses: Selvala's Stampede (192 Readings, expected about 32), Tivit (40, expected about 4).
- **MEDIUM-4.** *at random* attaches to Nominals: *Discard a card at random.* has 2 Readings, one with [NP a [card at random]]. Make it a predicate/clause-hosted adjunct only. Witness: Camouflage (4,288, expected about 1,750); `selection_idioms.txt:23`.
- **LOW-1.** Remove the `deferred.txt` Kutzil line, which no longer matches any unit, and make the census flag any deferral line that matches no corpus unit.

## Acceptance

Census before and after with no face lost; judgments hold; the witnesses above drop to their endorsed analyses.

## Landing note

Findings: MEDIUM-4 no longer reproduced at the claim (`ymlxxuwp` had already made *at random* Predicate-hosted: *Discard a card at random.* has 1 Reading, `selection_idioms.txt:23` holds, Camouflage's unit has 1,650); nothing changed for it. The *for* + AdjP frame is gone: *tied for greatest / most common* reads through a fused superlative head (`SuperlativeFusedHead`), and the analytic superlative now has Grade Superlative through a new Production field `unshared` (the mother does not share the listed features with its head). Vote words *bribery, death, feather, fellowship, knowledge, free, guilty, innocent, wild* left the general lexicon; *chaos* (*chaos ensues*), *evidence* (*evidence was collected*), *money* (a choice option) and *time* stay.

Census delta (measured on `ttutswwq` → `vqzvwpvo`): faces read 32,769 → 32,769; total Readings 344,348 → 344,020 and Surplus 142,949 → 142,909, over 10 units: Selvala's Stampede 192 → 16, Elrond of the White Council 64 → 16, Tivit, Seller of Secrets 40 → 8, Council Guardian 64 → 40, Plea for Power 32 → 8, Trial of a Time Lord 15 → 5, Capital Punishment 8 → 2, Messenger Jays 6 → 3, Tyrant's Choice 4 → 1, Everquill Phoenix 4 → 2. Lost faces: none. Refreshed onto `ttutswwq` with engine hygiene, duplicate trees and imperative coordination landed: 337,675 → 337,348 and Surplus 141,628 → 141,589, the same 10 units and no other (Messenger Jays 4 → 2 there).
Judgment set (`vqzvwpvo`, refreshed): 3,223 enforced and 319 pending, all enforced holding.
