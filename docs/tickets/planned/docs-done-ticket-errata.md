---
needs: []
---
# Add erratum notes to done tickets whose rule claims are wrong

Twenty-eight done tickets (29 items) state a rule claim that the CR contradicts, or record
work that did not land as the ticket says. Each ticket is a record of its
landing, so do not rewrite it. Add an erratum note at the claim, dated and
citing the governing rule, and leave the original text in place. Where the
wrong claim still drives live work, the note points to the successor ticket
that owns the fix. Items 1–15 need no successor, because the live code or a
live ticket already follows the right rule. Items 16–19 each have an engine
successor, and items 20–29 each have a Lean successor.

Leave alone the historical wrong citations in done tickets that
`engine-cr-citation-corrections` corrects in live code (the trigger-ordering,
hidden-zone-redirect and hexproof-from cites). That ticket owns the live ones.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
entries 17 (the off-stack part), 21 (the closure-flip part), 29–39, 44 and 46
(gitignored; read it from the default checkout), and, for items 16–19,
`departures-engine.md` #13, #18, #15 and #21; for items 20–29,
`departures-lean.md` #8, #10, #12, #5, #17, #16, #21 and #4. Every claim was re-read at
head on 2026-10-21. Line numbers are current.

## Errata

1. **`semantics-v2-off-stack-statics.md:48`.** The engine-owned daybound and
   nightbound rules are listed as [CR#702.145c,702.145d,702.145g,502.2]. The
   list omits the nightbound transform on a day mismatch [CR#702.145f]. Add
   it.
2. **`semantics-v2-off-stack-statics.md:170`.** "the CR writes 'except by'
   only for those three" (flying, fear, intimidate). Reach and menace use it
   too [CR#702.9b,702.13b,702.17b,702.36b,702.111b]. The real reason menace
   stays off `byObjects` is that its exception counts the blocking set. It is
   not a predicate on one object. Shadow and horsemanship say "can't be
   blocked by creatures without …" [CR#702.28b,702.31b]. The landed design is
   right.
3. **`workbench-closure-flip-risks.md:107,220`.** These lines call the
   keyword-counter list "[CR#122.1b]'s fifteen-name list". The rule adds "as
   well as any variants of those keywords" [CR#122.1b]. Eligibility is "is,
   or is a variant of, one of the fifteen".
4. **`core-saga-chapters.md:29`.** "Real ranges are always contiguous." The
   rules impose no contiguity [CR#714.2c,107.15b], and Blink prints `I, III`
   and `II, IV`. No code assumes contiguity.
5. **`core-fight-primitive-to-macro.md:6,22`.** These lines cite
   [CR#701.14a] for simultaneous fight damage "before either checks lethal".
   [CR#701.14a] does not say the damage is simultaneous. Simultaneity comes
   from [CR#608.2f] (rulings: Soul-Scar Mage 2017-04-18, Ezuri's Predation
   2015-11-04). State-based actions are not checked mid-resolution
   [CR#704.3], so "checks lethal" is not the reason. Simultaneity matters when
   damage becomes -1/-1 counters. Keep [CR#701.14a] for the power-damage
   clause. The landed `.simultaneously` is correct.
6. **`ability-conferral-without-innate.md:61,85,143`.** These lines cite
   [CR#305.7] for "losing all abilities removes the type-conferred mana
   ability". [CR#305.7] governs setting a land's subtype. The supporting
   rules are [CR#305.6,113.12,613.1f] (ruling: Humility 2007-02-01). The
   conclusion stands.
7. **`plugins-v2-keyword-body-defects.md:42`.** Costs are paid "in any
   order" [CR#601.2h]. The rule has two tiers: first the costs without random
   elements or library-to-public-zone moves, then the rest, in any order
   within each tier [CR#601.2h]. The engine already implements the tiers.
8. **`workbench-supplements-out.md:10–17`.** The ticket drops `Dungeon` from
   `CardType` and calls emblems the future home of dungeon semantics. Dungeon
   is a card type [CR#205.2a]. A dungeon card goes into the command zone with
   a venture marker [CR#701.49a] and is completed as it is removed from the
   game [CR#309.7]. An emblem has no types [CR#114.3]. Lean and Rust have
   since restored `CardType.dungeon`, and `planned/engine-dungeons.md`
   already models dungeon cards. The note marks the ticket superseded.
9. **`semantics-v2-macro-bodies-keyword-abilities.md:393`.** "PowerUp (also
   has no [CR#702] entry …)". Power-up is [CR#702.193a]. Drop that clause.
   The `KeywordParam` lacking an `Ability` variant stands alone.
10. **`english-keyword-ability-parameters.md:116–118`.** "The CR states a
    written form explicitly for only three keywords". At least seven rules
    use the word "written" [CR#702.5a,702.16a,702.57a,702.139a,702.164a,702.167a,702.174a],
    and many more give a quoted template with "means" (for example
    [CR#702.6a]). The keyword-to-argument-shape table now lives in the v4
    catalog's parameter frames, so no replacement ticket is needed.
11. **`english-v2-copular-complement-sum.md:320`.** "The ordering is a rules
    fact": the claim is that a card's name in its own text is the
    self-reference. That holds only when the text refers to the object it is
    on by name [CR#201.5]. Counterexamples: Seven Dwarves ("each other
    creature named Seven Dwarves"), Dragon's Approach, and "each Garruk you
    control", which is a subtype [CR#205.3j]. Restate it as a ranking
    tie-break, not a rules fact. `done/english-v4-self-name-ranking.md`
    already ranks it so, so no replacement ticket is needed.
12. **`english-v3-name-lexeme-scope.md:128`.** This line attributes to
    [CR#201.5c] the claim that text on one face may name another face.
    [CR#201.5c] covers only shortened names. Each face has its own
    characteristics, name included [CR#712.8]. Cite [CR#712.8], and call the
    lexical scoping a project decision.
13. **`english-v3-phrasal-keyword-surfaces.md:58`.** PK2 says each partner
    variant "is not the keyword partner plus a dashed parameter". The rules
    define one `partner—[text]` template over a closed set of texts, with
    pairing by same text [CR#702.124i]. [CR#702.124f] says only that the
    variants are distinct and cannot be combined. Separate surfaces remain
    acceptable.
14. **`workbench-keyword-labels.md:108`.** "The `Put`-shaped exception
    exists, three times over" (level, chapter, class level
    [CR#711.2,714.2,716.2]). Station symbols are also keyword abilities
    defined outside 702 [CR#702.184b,721.2], and so are a Case's two labels
    [CR#719.3,719.3a]. The conclusion (an open keyword vocabulary) stands.
    `planned/english-v4-case-to-solve-label.md` already treats *To solve* as
    a keyword-ability frame.
15. **`workbench-event-zone-4-zone-catalog-and-reader-payload.md:197,429`
    and `workbench-placement-and-origin-gates.md:21,114`.** These lines cite
    [CR#603.6a] for "put ONTO the battlefield, never 'into' it".
    [CR#603.6a] is about when enters abilities trigger and how they are
    written. It says nothing about prepositions. The pattern is a matter of
    Oracle wording only. Drop the citation, and rest the battlefield exclusion
    on the construction boundary: *enters* has its own form.
16. **`engine-target-distinctness.md` §3 (lines 71–84).** This section
    specifies that "illegal targets are excluded from `Target(n)`/plural reads"
    on resolution [CR#608.2b]. Only the exclusion of departed targets landed.
    The resolving spell or ability still affects, and reads information from,
    a target that is still in its zone but has become illegal. Erratum: the
    specified exclusion did not land; successor
    `critical/engine-resolution-illegal-targets`.
17. **`engine-deathtouch-sba-window.md` (lines 16–18 and the fix).** The
    ticket specifies a window that closes at each state-based-action check
    [CR#704.5h], and its rule reading is right. The landing instead attached
    deathtouch provenance to the damage mark, so the window lasts as long as
    the mark [CR#120.6]. That contradicts [CR#704.5h]. Erratum: the landing
    departed from the spec; successor `planned/engine-turn-and-sba-timing` §3.
18. **`runner-hidden-info.md:35`.** The ticket says that the `look_grants`
    ledger is one "that shuffle/remint clears". A shuffle is not a zone change,
    and `handle_shuffled` only permutes the library, so grants survive a
    shuffle. Shuffling randomizes the library "so that no player knows their
    order" [CR#701.24a]. Erratum: the claim is false at head; successor
    `planned/engine-look-grants-lifecycle`.
19. **`kw-bushido.md:21–23`.** The ticket records as a seam that multi-block
    "blocks" under-counts, because `scan_triggers` dedups `Blocked` once per
    attacker. This is not a tolerable seam. "Whenever [a creature] blocks"
    triggers once each combat for each blocking creature [CR#509.3a], so two
    bushido creatures double-blocking one attacker both trigger. Erratum: it is
    a defect; successor `planned/engine-combat-block-status` §2.
20. **`core-saga-chapters.md:39–42`.** The ticket says that under Doubling
    Season a Saga "enters with 2 lore and increments by 2". Entering with 2
    is right [CR#714.3a,614.16]. The increment is a turn-based action
    [CR#714.3c], not an effect, because an effect comes from a spell or
    ability [CR#609.1]. Doubling Season applies only to effects. Erratum: the
    increment stays 1; successor `planned/lean-v3-saga-rules-fidelity` §3.
21. **`workbench-bare-plural-det.md:77–80`.** The re-spelled witness gives
    Lightmine Field's amount as the size of the triggering group ("the number
    of those creatures"). The count is taken when the effect applies
    [CR#608.2h], so it counts the creatures attacking at that point: a
    creature removed from combat [CR#506.4] isn't counted, and one put onto
    the battlefield attacking is. Erratum: the amount is the number of
    attacking creatures at resolution; successor
    `planned/lean-v3-rules-and-card-fidelity` §5.
22. **`workbench-coordination-1-condition-disjunction.md:126–129`.** The
    ticket places Quakebringer's "This ability triggers only if …" in "the
    trigger's intervening 'if'" [CR#603.4]. The intervening-if rule applies
    only to an "if" that immediately follows the trigger condition
    [CR#603.4]. A "triggers only if" condition is checked when the event
    occurs and is not rechecked on resolution. Erratum: it is a trigger
    guard, not an intervening if; successor
    `planned/lean-v3-rules-and-card-fidelity` §5.
23. **`workbench-anaphora-d-creation-and-replacement.md:241`.** "D1 — 'first
    time … each turn' cap — LANDED" records the ordinal as a usage limit. An
    ability triggers, and a replacement applies, by matching an event
    [CR#603.2,614.1]. The rulings (Moonlit Meditation 2025-07-25) read "the
    first time … each turn" as a test of the turn's history, not a cap on
    uses. Erratum: the landed encoding is wrong; successor
    `planned/lean-v3-event-ordinals-and-counter-thresholds` §1.
24. **`workbench-transform-tail.md:56`.** The ticket says Cult of the Waxing
    Moon's bench is "whole … for the complement". At head the Lean
    encoding has no "non-Human creature" complement. A "transforms into"
    trigger tests the characteristic immediately after the transform
    [CR#701.27e]. Erratum: the complement did not land; successor
    `planned/lean-v3-rules-and-card-fidelity` §5.
25. **`lean-v3-trigger-words.md:70`.** This line counts Xenosquirrels
    (`Cards/Description.lean:495`) among the `after` trigger-word sites. "After you roll a
    die" is not a trigger: triggered abilities use "when", "whenever" or "at"
    [CR#113.3c]. The ability is a roll modifier, applied before the result is
    final [CR#706.2,706.2a]. Erratum: successor
    `planned/lean-v3-keyword-triggers-entries-and-parties` §10.
26. **`workbench-witness-sweep-residues.md:222–228`.** "`TriggerWord.After`":
    the ticket adds `After` as a trigger word and rewrites Xenosquirrels as a
    `Triggered After` ability. This is the same error as item 25
    [CR#113.3c,706.2,706.2a]. Same successor (§10).
27. **`workbench-amount-comparison-and-quantity.md:495–519`.** The ticket
    makes `NthOccurrence` the reading of "the Nth [kind] counter" headers.
    Such a trigger fires when counters are put on so that the object had
    fewer than N before and N or more after [CR#122.7]. That is a threshold,
    not an occurrence count, and the two diverge after removals or batch
    placement. Erratum: successor
    `planned/lean-v3-event-ordinals-and-counter-thresholds` §2.
28. **`workbench-randomness-residues.md:253,270`.** Line 270 records the
    blocker for "After you roll a die, …" as "a fourth trigger word", and
    line 253 benches Xenosquirrels' modifier as a triggered ability. The
    ability is a roll modifier, applied before the result is final
    [CR#706.2,706.2a]. It is not a triggered ability [CR#113.3c]. Erratum:
    replace "a fourth trigger word" with "a roll-modifier ability"; successor
    `planned/lean-v3-keyword-triggers-entries-and-parties` §10.
29. **`lean-v3-ast-merges-by-rules-meaning.md` ("Sites").** It lists *Phyrexian
    Infiltrator* (`Cards/Anaphora.lean:66`) among the `gainControl` card sites
    that "need no change". Its "exchange control" is one exchange: if the
    whole exchange can't be completed, no part of it occurs [CR#701.12a], and
    it does nothing when one player controls both permanents [CR#701.12b].
    Two independent gains of control lose both conditions. Erratum:
    successor `planned/lean-v3-rules-and-card-fidelity` §3 (`exchangeControl`).
