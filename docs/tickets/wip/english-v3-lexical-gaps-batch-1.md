---
needs: [english-v3-dead-lexeme-audit]
---
# Close six small composition gaps around existing lexemes

## Why

An unread-face reconnaissance on change `wlvwtnppyovn` (32,828 supported
faces, 13,716 covered, 19,112 unread) found six frequent surfaces that never
read. Every word in them already has a Lexical Analysis: the corpus report
lists none of them as unknown words. Each gap is a missing composition for an
existing lexeme or notation, not a missing word. Counts are "faces touched /
faces where it is the only recognised cause"; they are surface-bucket counts,
not gain forecasts.

| Item | Touched / sole | Lexical analysis today | Probe (admitted roots) |
|---|---|---|---|
| *combat* premodifying *damage* | 1,120 / 396 | `lexeme:turn_part/combat` (noun) + `lexeme:CommonNoun/Damage` | "Whenever this creature deals combat damage to a player, draw a card." 0; without *combat*, 1 |
| *defending player* | 245 / 98 | `lexeme:Verb/Defend` GerundParticiple + `lexeme:CommonNoun/Player` | "Whenever this creature attacks, defending player loses 1 life." 0; *target player*, 1 |
| plural (bare) genitive *owners'*, *controllers'* | 205 / 97 | `lexeme:CommonNoun/Owner` Plural + `vocab:Genitive/Sibilant` | "Return all permanents to their owners' hands." 0; *owner's*, 1 |
| hybrid and Phyrexian mana symbols | 203 / 75 | `vocab:FixedCostSymbol/HybridRedGreen`, `…/PhyrexianRed` exist; `{2/U}` has no lexical match | "{R/G}: This creature gains flying until end of turn." 0; `{R}`, 1; "you may pay {R/W}." 0; `{R}`, 1 |
| *at random* | 476 / 63 | `vocab:Preposition/At` + `vocab:Adjective/Random` | "When this creature enters, discard a card at random." 0; without it, 1 |
| colour-negative *nonblack*, *nonred*, … | 119 / 87 (every failing unit contains one) | `vocab:NegativePrefix/Non` + `vocab:ColorWord/*` | "Destroy target nonblack creature." 0 (likewise nonred, nonwhite, nongreen, nonblue); *nonartifact*, *nonland*: 1 |

No covered face contains a non- colour adjective, while 320 covered faces
contain *non-* + a card type. Attested symbol spellings in supported faces:
ten two-colour hybrids (`{W/B}` 47 occurrences … `{W/U}` 18), five Phyrexian
(`{W/P}`, `{U/P}`, `{B/P}`, `{R/P}`, `{G/P}`), and the monocolored hybrids
`{2/U}`, `{2/B}`, `{2/R}`, `{2/G}` (one occurrence each). `{2/W}` is not
attested and must not be added (pruning rule).

## Goal

Each item reads in its attested hosts, through declared features. One item
must not be fixed by widening another.

## Analysis

*Combat* in *combat damage* is a noun used as an attributive modifier. It stays
a noun and forms a composite nominal with *damage*, not a compound noun:
CGEL, Ch. 6, §2.4.1, p. 537, [27]; Ch. 5, §14.4, pp. 448–449. The glossary's
Premodifier entry already records that type nouns license this function by
declaration. *combat* needs the same declared availability, not an Adjective
analysis. *Defending* in *defending player* is a gerund-participial
Premodifier, like *attacking creature* (the Premodifier entry, CGEL, Ch. 6,
§2.4.3, pp. 541–542). Its lexical declaration must record that distribution.
*Owners'* is the bare genitive of a plural noun ending in *s*, written as a
bare apostrophe (CGEL, Ch. 18, §4.2, p. 1595, [35]). It must compose like
*owner's* with plural Number on the possessor. *At random* is a
Preposition + Adjective idiom of the *at first*, *at last* type (CGEL, Ch. 7,
§3.2, p. 626, [23]). It is a PP whose Preposition Function Licence must admit
the clause Adjunct function it has in *discard a card at random*. *Non-* is a
negative prefix (CGEL, Ch. 19, §5.5, p. 1687). It already composes with type
nouns but not with Color Words. The Affix's declared host classes are the
place to fix that.

## Witnesses

- Fell Flagship: "Whenever this Vehicle deals combat damage to a player, that
  player discards a card."
- Odious Witch: "Whenever this creature attacks, defending player loses 1 life
  and you gain 1 life."
- Upheaval: "Return all permanents to their owners' hands."
- Vexing Shusher: "{R/G}: Target spell can't be countered." (Phyrexian:
  Immolating Souleater, "{R/P}: This creature gets +1/+0 until end of turn.")
- Spellgorger Barbarian: "When this creature enters, discard a card at random."
- Doom Blade: "Destroy target nonblack creature."

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Lands after `english-v3-dead-lexeme-audit`, which deletes zero-token
   lexemes; re-run the probes above on the refreshed tree first and drop any
   item that already reads.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/lexical-gaps-1-before.json` on the claim parent,
   stamped with its change id.
3. Land a series of logical commits, one per item. Write each witness as a
   test first, then iterate on `--face-id` selectors for that item, and verify
   on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading, covering every item. A
   negative or wrong analysis that starts parsing (for example *combat* read as
   an Adjective, or *owners'* read as *owners* + stray mark) is a defect, not
   a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Only attested spellings may be added (pruning rule): add `{2/U}`,
   `{2/B}`, `{2/R}`, `{2/G}`, not `{2/W}`.
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Other *non-* compositions that already read.
- Cost-modification sentences ("costs {1} less") and Phyrexian payment rules;
  this ticket only makes the symbols read where a plain symbol reads.

## Landing record

In addition to the standard record: per item, before/after covered and touched
faces stamped with change ids; the declared feature each fix reads; newly
covered faces by item; timings as integer ns and ns/B with host load and
worker count.
