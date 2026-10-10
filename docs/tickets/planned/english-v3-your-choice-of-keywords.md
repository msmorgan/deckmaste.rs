---
needs: []
---
# Read *your choice of* + keyword alternatives: Target creature gains your choice of deathtouch or lifelink until end of turn.

## Why

*your choice of* reads before NPs (NP "your choice of a creature or a land"
2; "Put your choice of a +1/+1 counter or a flying counter on target
creature." 2) and keyword alternatives read as a granted ability ("Target
creature gains deathtouch or lifelink until end of turn." 1), but the two
together do not: NP "your choice of deathtouch or lifelink" 0, NP "your
choice of flying" 0, "Target creature gains your choice of deathtouch or
lifelink until end of turn." 0, "… gains your choice of flying, vigilance, or
lifelink until end of turn." 0. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:lifelink until` 9 (one
keyword of many; the rest are spread over `at:<keyword> until` keys).
`cargo xtask english-v3 grep -i '\bgains? your choice of\b'` finds **26**
faces, all unread; **25** have that line as their only failing line (plus
*loses your choice of*, Walking Sponge, 1 sole). All 27 continue with two to
four keyword names joined by *or*.

## Goal

After *gain* / *lose*, *your choice of* K1(, K2,) *or* Kn reads, the keyword
names being the alternatives chosen among: "Target creature gains your choice
of deathtouch or lifelink until end of turn.", with byte-exact realisation.

## Analysis

No CGEL claim: *choice of* + alternatives is an ordinary noun with an *of*
Complement, and that the alternatives may be keyword names is Oracle usage.
Project rulings throughout.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`KeywordGrantedAbility` (`[keyword: KeywordPhrase]`) feeding `core-verb:Gain`'s
frame `Predicate([Role("GrantedAbility")])` beside its
`Predicate([ObjectNounPhrase])` (`lexicon/verbs.ron`);
`KeywordComplementPreposition` (`[head: lexical(Preposition), " ",
complement: KeywordPhrase]`, `require head.KeywordComplement = Yes`), which
is how *with flying* reads: `vocab:Preposition/With` declares
`KeywordComplement: Yes`, `vocab:Preposition/Of` does not
(`lexicon/core.ron`); the noun *choice* with its *of* Complement, which
reads *your choice of a Clue token, a Food token, or a Treasure token* today.

Orchestrator rulings 2026-10-10:

- CK1. The phrase is an NP Object of *gain* / *lose*: [*your choice* [*of*
  keyword alternatives]]. It enters through the Object frame, not through
  the `GrantedAbility` frame, and no new Construction is added: the *of* PP
  takes its keyword Complement through `KeywordComplementPreposition`.
- CK2. The licence is declared. Preferred: the head noun selects the keyword
  Complement class for its *of* frame (data on *choice*'s nominal frame),
  so *of* + keyword reads only under a noun that declares it. Declaring
  `KeywordComplement: Yes` on *of* itself is the fallback and must be
  measured on every face containing *of* + a keyword name before it is
  accepted (17,681 *of* tokens; an *of flying* PP must not start modifying
  arbitrary nouns).
- CK3. The alternatives are a keyword coordination with *or*, including the
  serial-comma series (*flying, vigilance, or lifelink*). If
  `KeywordComplementPreposition`'s Complement is a single `KeywordPhrase`,
  widen it to the keyword series category the granted-ability route already
  coordinates; do not write a second coordination.
- CK4. One analysis per string. *gains deathtouch or lifelink until end of
  turn* keeps exactly its Reading. In the witness, *until end of turn*
  attaches to *gains* once; the twin to avoid is a Reading in which only the
  last keyword is inside the *of* PP (*your choice of deathtouch* coordinated
  with *lifelink*).
- CK5. Parameterised keywords in the list (*protection from red*: Jodah's
  Avenger; *banding*: Urza's Avenger) read if their keyword phrase reads
  alone; no special case.
- CK6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: the 18 covered *your choice of* faces (57 in all;
  counter, token and source alternatives) keep their counts. Report any increase with its shape.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Orcish Medicine: "Target creature gains your choice of lifelink or
  indestructible until end of turn."
- Alchemist's Gift: "Target creature gets +1/+1 and gains your choice of
  deathtouch or lifelink until end of turn."
- Shifting Ceratops: "{G}: This creature gains your choice of reach, trample,
  or haste until end of turn."
- Golem Artisan: "{2}: Target artifact creature gains your choice of flying,
  trample, or haste until end of turn."
- Walking Sponge: "{T}: Target creature loses your choice of flying, first
  strike, or trample until end of turn."

Controls that must not change: "Target creature gains deathtouch or lifelink
until end of turn." 1; Flycatcher Giraffid "This creature enters with your
choice of a reach counter or a vigilance counter on it." 5 (census); The
Third Doctor "When The Third Doctor enters, create your choice of a Clue
token, a Food token, or a Treasure token." 2 (census); Asha's Favor
"Enchanted creature has flying, first strike, and vigilance." 1 (census); NP
"a spell with flying" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\byour choice of\b|\b(gains?|loses?|has|have) [a-z ,]+ or [a-z ]+ until\b

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent (size
the selector with `cargo xtask english-v3 grep --count` and record it). If
the CK2 fallback is taken, the selector also takes every face matching *of* +
a declared keyword surface. Zero lost; CK6 for increases. Full `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (*your
choice* as Object head, the keyword alternatives as Complement of *of*; the
CK4 negative), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *put / create / with your choice of* … (11 unread faces), e.g. *put your
  choice of a +1/+1, first strike, or trample counter on* …: shared-head
  counter-kind premodifier coordination under one article.
- *enters with your choice of two different counters on it from among* …
  (Grimdancer); *it becomes your choice of 5/1 or 1/5* (Aquamorph Entity);
  *put your choice of a counter from among* … (Aragorn, Company Leader).
- Ballroom Brawlers (*both gain*), Hunter's Axe (inside a granted quote) if
  those hosts block.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
