---
needs: [english-structural-recovery-zero]
design: true
---
**[design] Agentive `-er` nominalization as a productive shape.** User-directed
design item (2026-07-24): derive agent nouns from verbs instead of minting a
lexical TSV row per agent noun. Subsumes the `voter` and `bidder` rows landed
by the lexical sweep, and audits existing `-er` noun entries for rows the
derivation should own (irregulars and non-agentive `-er` nouns stay lexical).
Design round first: the derivation shape, the renderer inverse (including
consonant doubling, `bid` → `bidder`), dispreference against genuine lexical
noun readings, and which nominal slots admit the derived reading. Standard
constraints apply.

## Completion

The design was settled by user direction on 2026-07-28: agent nouns are
productive by default for registered lexical verbs. The public syntax stores
the derivation as `Noun::Agentive(Verb)`. The reverse vocabulary index emits
singular and plural count-noun readings into `Count` and `Either` slots, never
the mass-noun slot. Rendering is source-independent: final `e` takes `-r`
(`vote` → `voter`), while other verbs derive from the present-participle stem,
which preserves declared consonant doubling (`bid` → `bidding` → `bidder`).

Derived readings carry one point of reading dispreference. Explicit lexical
nouns therefore win true collisions: the audit retains `player`, `controller`,
`owner`, and the non-agentive game-object reading of `counter`. The derivation
now owns and removes the lexical TSV rows for `attacker`, `bidder`, `blocker`,
`caller`, `hunter`, `smasher`, `voter`, `voyager`, and `walker`. The combat-step
nominal guard accepts the resulting structural `attackers` and `blockers`, so
`declare attackers step` and `declare blockers step` retain their exact parse.

The supported-corpus recovery census is unchanged at 3,480 structural spans /
64,217 source tokens, including 3,345 clause spans / 63,129 source tokens.
Noun opacity remains 792 and flavor-header opacity remains 622.

All 573 library tests and 112 public-API tests pass. The supported corpus
round-trips 31,685/31,685 clean with zero mismatches or render errors. Clippy
reports only the repository's existing unrelated warnings; the citation audit
likewise reaches its two pre-existing malformed examples in the planned
comment-discipline ticket and no changed citation site.
