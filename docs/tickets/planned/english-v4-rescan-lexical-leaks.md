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

Census delta:
Judgment set:
