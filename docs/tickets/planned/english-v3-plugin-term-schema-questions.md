---
needs: [english-v3-plugin-term-definitions]
---
# Export deferred plugin terms through their owning schemas

Complete the seven declaration and Construction gaps deferred from
`english-v3-plugin-term-definitions`. Standard constraints apply. Extend the
owning family or declared class, never a member list or spelling guard. Preserve
independent Ambiguity under owner ruling W9-A (2026-10-10); one route means one
route for the new material. Retire superseded lexical and grammatical routes
together. Bodiless definitions are allowed in every family under owner ruling
PT-Q1 (2026-10-10); record each engine body and missing facts row as semantics
debt, following the Typecycling exemption precedent.

Probe evidence below is relative to
`target/english-v3/english-v3-plugin-term-definitions/`. Every witness/control
pair was re-probed with `cargo xtask english-v3 probe` on the claim parent
`vwktxkol` before implementation; `before-probes.json`,
`extra-before-probes.json`, and `gift-before-probes.json` preserve the text and
Readings. Counts are witness/control. Fresh supported-face counts were taken with `cargo xtask english-v3 grep`
on `kmynzkut`; unread counts use the claim-parent `before.json`. The JSON
attestations are `deferred-<question>-faces.json` and
`deferred-<question>-unread.json` in that directory. Historical target-shape
counts remain stamped `uoqyuxtnowpo`. Re-probe on the follow-up claim parent
before implementation. Do not search raw corpus
JSON. Build a bounded selector covering each affected surface anywhere plus
300 random covered faces (seed 20261008), and use the official census diff.

## 1. Party's noun-exporting family/kind and full party

**OWNER DECISION:** choose the helper family/kind that owns the party
noun stub, and its noun-export path. Owner answer (2026-10-10): party is a
bodiless stub with its own mechanism for choosing party members, “sort of like
a bundle”; that member-selection mechanism remains semantics debt. The answer
authorizes the stub but does not choose its family/kind.

Faces (`kmynzkut`, unread on claim parent): party 43 supported / 40 unread;
full party 12 / 11 (`deferred-party-*`, `deferred-full-party-*`).
Claim-parent probes (`before-probes.json`, labels `party`, `full-party`):
“When this creature enters, scry X, where X is the number of creatures in your
party.” 0 / 3 with *graveyard*; “At the beginning of combat on your turn, if
you have a full party, draw a card.” 0 / 1 with *a creature*.

Smallest schema change: let the chosen helper kind export the existing `Noun`
row. `nouns/` holds relevant semantic helpers, but the lexical reader currently
accepts only Predicate and GameEvent helpers from `predicates/` and `events/`.
Full party additionally needs the ordinary adjective *full* and its admitted
use; resolve that vocabulary/Construction gap separately from the party noun.

## 2. Adjective exports from predefined Role-token declarations

**OWNER DECISION:** permit the predefined-token family to export its declared
names as Adjective Lexemes through a family recipe. Owner answer (2026-10-10):
Roles belong beside `instructions/treasureToken.ron`; follow that family's
shape and `grammar:` conventions. Its current `CharacteristicBundle` kind
has no lexical reader or adjective-export convention.

Faces (`kmynzkut`, unread on claim parent): 38 supported / 34 unread matching
`(Wicked|Monster|Young Hero|Cursed|Royal|Sorcerer|Virtuous) Role`.
Claim-parent probes (`before-probes.json`, corresponding name labels):
“Create a <name> Role token attached to target creature you control.” gives
0 / 5 for Wicked, Monster, Young Hero, Cursed, Royal and Virtuous. Sorcerer
already gives 9 / 5: drop that witness and preserve its existing lexical owner.

Smallest schema change: a grammar/export recipe for the predefined-token
family supporting `Adjective`, including multiword names, with the declaration
retaining ownership. Declare the productive class, not a Role-name member list.

## 3. Noun exports for designation members day/night

**OWNER DECISION:** choose how a multi-member Designation declares member noun
exports while retaining its composite term. Existing owner:
`designations/dayNight.ron`, `FixedTerm(surface: "day or night")`, members
`day` and `night`; the members currently export no nouns.

Faces (`kmynzkut`, broad `day|night`): 225 supported / 42 unread
(`deferred-day-night-*`). Historical target-shape count (`uoqyuxtnowpo`):
16 supported faces match *it's day/night*, *becomes
day/night*, or *neither day nor night* (broaden the selector to every occurrence
of day/night before narrowing any route). Claim-parent probes
(`before-probes.json`, `day`, `night`): “If it's day, draw a card.” 0 / 1;
“This spell costs {2} less to cast if it's night.” 0 / 2, substituting *your
turn* in each.

Smallest schema change: member-level noun export declarations or a family
recipe deriving the noun exports from the declared members. Preserve the
existing composite *day or night* owner and avoid a duplicate night-counter
noun owner.

## 4. Gift noun and promise verb

**OWNER DECISION:** allow `FixedKeyword` to export a Noun alongside its existing
verb export. Owner is `keyword_abilities/gift.ron`. Its `verb:` field already
allows bare *promise* independently of the keyword stem; no new verb field is
needed.

Faces (`kmynzkut`, broad `gift|promis`): 29 supported / 25 unread
(`deferred-gift-*`); historical *promised* count (`uoqyuxtnowpo`) is
24 touched / 24 sole-blocker faces.
Claim-parent probes: `before-probes.json`, `gift`, “If the gift was promised,
draw a card.” 0 / 2 replacing the clause with *If a creature was blocked*.
`gift-before-probes.json`, `gift-passive` and `gift-negative`, retain *the gift*
and substitute only *blocked*: “If the gift was promised, instead counter
target spell.” and “If the gift wasn't promised, you lose 2 life.” both 0 / 0.

Smallest schema change: a noun-export field or correlated export recipe on
FixedKeyword, using the existing Noun row and verb frame mechanism. Then declare
both gift and promise through the one keyword definition; verify passive use.

## 5. Bare Quality after bands with other

Faces (`kmynzkut`): 8 supported / 8 unread (`deferred-bands-*`). Claim-parent probes
(`before-probes.json`, `bands`, `bands-2`): “Bands with other legendary
creatures” and “Bands with other Dinosaurs” both 0 / 2 against corresponding
*Affinity for* controls. Trial FixedKeyword with a Quality parameter still
reads 0; trial evidence is `after-probes.json` and `after-probes.log`. The trial
was removed, so there is no unused route to retain.

Smallest Construction change: admit a bare Quality after a declared keyword
head with a Quality parameter. Current routes require a preposition-marked
Quality, a before-head Quality, or a bound suffix. Declare the class licence
for this parameter position rather than naming *bands with other* in a guard.
Use its owning keyword definition, distinct from ordinary banding.

## 6. Passive frames for banded with

**OWNER DECISION:** choose a Passive verb-frame export or a Complement-capable
participial-adjective export for keyword definitions.

Faces (`kmynzkut`): 3 supported / 3 unread (`deferred-banded-*`). Claim-parent probes
(`before-probes.json`, `banded`, `banded-2`): “creatures banded with other
creatures” and “a creature banded with another creature” both 0 / 1 against
*paired with*. Trial active Object + with + Object export still leaves both at
0 (`after-probes.json`, `after-probes.log`); that unused trial was removed.

Smallest schema change: add a Passive frame variant to KeywordVerbGrammar or a
Complement frame field to its participial-adjective row. Current variants are
Predicate, Auxiliary and ProVerb. Pair's Passive frame is supplied in core
lexicon data; the original ticket's PT1 forbids adding the band frame there.
Owner remains `keyword_abilities/banding.ron`; preserve existing banding.

## 7. Subtype-complement licence for empower Jace

Owner confirmed from the rules: Empower Jace is a keyword action
[CR#701.71a]. Faces (`kmynzkut`): 35 supported / 35 unread for empower
(`deferred-empower-*`).
Claim-parent probes (`before-probes.json`, `empower`; `extra-before-probes.json`,
`empower-subtype`, `empower-2`): “Empower Jace 3.” and “When this creature
enters, empower Jace 3.” both 0 / 1 with *amass Zombies*. The narrower control
“Amass Jace 3.” is also 0, with no vocabulary gaps.

Smallest Construction/class change: a declared licence for a bare singular
Subtype Complement in the existing Object Noun Phrase + Amount frame, where
that subtype denotes the action's parameter. **OWNER DECISION** if the licence
requires a new subtype export/frame schema. License the productive declared
class, never a Jace member-list exception. The vocabulary-only test fails until
this licence exists, so do not ship an unused empower verb export.
