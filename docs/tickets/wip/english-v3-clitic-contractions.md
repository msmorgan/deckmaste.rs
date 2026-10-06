---
needs: []
---
# Give clitic be and have the frames of their full forms

## Why

Clitic auxiliaries on pronoun hosts (*it's*, *that's*, *you've*, *they're*,
*you're*) fail where the full form reads. On change `wlvwtnppyovn` (32,828
supported faces, 13,716 covered, 19,112 unread) they occur in a failing unit
of 1,318 unread faces and are the only recognised cause on 352. These are
surface-bucket counts, not gain forecasts.

Minimal pairs on that change (`cargo xtask english-v3 probe --readings 0`,
admitted roots):

| Contracted | Full |
|---|---|
| "If it's a land card, you may put it onto the battlefield tapped." 0 | "If it is …" 1 |
| "If it's tapped, put a stun counter on it." 0 | "If it is tapped, …" 1 |
| "Create a token that's a copy of target creature." 0 | "… that is a copy …" 1 |
| "Exile target creature that's attacking you." 0 | "… that is attacking you." 1 |
| "This creature can't attack unless you've cast a creature spell this turn." 0 | "… unless you have cast …" 1 |

Negative inflections already read: "This creature can't block." 1 and "This
creature isn't legendary." 1. Check *didn't*, *doesn't*, *wasn't* the same way
before scoping them in.

The likely cause is in the `frame_additions` block of
`crates/deckmaste_lexical_source/lexicon/core.ron`. It gives
`core-verb:BeContracted` (and `BeNegative`) only a `LocativeComplement`
Predicate frame and a `ParticipialPredicate` Auxiliary frame. The full *be*
also takes NP and AdjP predicative Complements. The contracted-*be* Predicate
frame was added by the generic-frame-consumption landing (its STOP 2, Urza's
Ruinous Blast) to stop an auxiliary-stranding misparse. Keep that witness's
two exact Readings. `core-verb:HaveContracted` has only a
`PastParticiplePredicate` Auxiliary frame, yet "you've cast" still fails, so
the *have* cause is something else. Find it.

## Goal

Each clitic form has exactly the frames of the full-form paradigm cell it
realises, so every attested contracted host reads with the same Readings as
its uncontracted twin. Where a frame is deliberately withheld from the clitic,
the withholding must come from a declared feature with a CGEL basis, not from
an ad hoc omission.

## Analysis

These are clitic versions of auxiliary verbs (CGEL, Ch. 18, §6.2,
pp. 1614–1616). The clitic forms of *are* and *have* (*they're*, *you're*,
*you've*) attach only to a preceding subject pronoun ([6]–[7], p. 1615). The
clitic forms of *is* and *has* are less restricted: their hosts include NP
Subjects and relative *that* (*the one that's up in the bedroom*, [8iii]).
*'s* is therefore ambiguous between *is* and *has*, as in *It's finished*
(p. 1615). Clitic auxiliaries are a reduced form of the same lexeme, a
property that only auxiliaries have (CGEL, Ch. 3, §2.1.5, p. 102). The clitic
inherits its verb's complementation; its host restriction is a
morphophonological fact, recorded on the lexeme, not in the frame list.

## Witnesses

- Risen Reef: "If it's a land card, you may put it onto the battlefield tapped."
- Shackle Slinger: "If it's tapped, put a stun counter on it."
- Mirrorpool: "{4}{C}, {T}, Sacrifice this land: Create a token that's a copy
  of target creature you control."
- Bounty Agent: "{T}, Sacrifice this creature: Destroy target legendary
  permanent that's an artifact, creature, or enchantment."
- Goblin Cohort: "This creature can't attack unless you've cast a creature
  spell this turn."
- Panglacial Wurm: "While you're searching your library, you may cast this card
  from your library."

Each witness's only failing unit is the one quoted, and each witness's
uncontracted twin reads (probed on `wlvwtnppyovn`).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. First determine why "Create a token that's a copy of target creature."
   fails and why "you've cast" fails, and record both causes.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/clitic-contractions-before.json` on the claim
   parent, stamped with its change id.
3. Write each witness and its uncontracted twin as tests first. The two must
   have the same Reading count and the same structure modulo the auxiliary
   leaf.
4. Keep the Urza's Ruinous Blast exact two-Reading test passing unchanged.
5. Iterate on `--face-id` selectors; verify on `--all` at the end.
6. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
7. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A clitic *'s* read as a
   Genitive where the uncontracted twin has *is*, or any auxiliary-stranding
   gap, is a defect, not a gain.
8. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
9. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
10. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *there's* and existential *there* (a separate unread bucket).
- Any cause the uncontracted twin also fails on. Damping Matrix's "…can't be
  activated unless they're mana abilities." fails uncontracted too ("unless
  they are mana abilities", 0 admitted roots).

## Landing record

In addition to the standard record: the two causes from Method step 1; a table
of clitic forms (`'s` as *is*, `'s` as *has*, `'re`, `'ve`, `'d`) with their
frames before and after; the Urza's Ruinous Blast result; timings as integer ns
and ns/B with host load and worker count.
