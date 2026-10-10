---
needs: []
---
# Read a quoted ability that does not end its sentence: It has "{T}: Draw a card" and "{T}: Add {G}."

## Why

A quoted ability reads only when it is the last thing in its sentence, with
the full stop inside the quotation. Anywhere else the quotation has no full
stop and reads nowhere: before a coordinator, before a second quotation,
before a keyword, before a duration Adjunct. So the gap is the unterminated
quotation, and coordination is only its commonest setting. Blocker report at
trunk `uoqyuxtnowpo`: key `at:" and` 24 sole-blocker faces. `cargo xtask
english-v3 grep '[a-z0-9}]" [a-z]'` finds **86** faces (83 unread); 84
failing lines match, **75** of them the only failing line of their face. What
follows the closing quote on those 75: *and "…"* 23, *until end of turn* 12,
*and equip {N}* 8, *until …* 7, *and loses / gains / is / it …* 9, other 16.

The 3 covered faces show the edge: Ogre Marauder's *gains "this creature
can't be blocked" until end of turn* reads because its quotation is a
lower-case bare Clause.

## Goal

A quotation holding an ability (static sentence, triggered ability, activated
ability with its cost) reads without its final full stop when the quotation
does not end the sentence, in every position a quoted Granted Ability already
reads: coordinated with another quotation or a keyword in either order,
followed by a predicate coordinate, followed by an Adjunct.

## Analysis

No CGEL citation applies: where the full stop sits relative to the closing
quotation mark is Oracle typography. Everything below is a project ruling.
Glossary: **Quotation**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`QuotedText` (`["\"", text: Document, "\""]`; the Document's last Sentence
must carry its full stop), `QuotedClause` (`["\"", clause: Clause, "\""]`,
`boundary Interior`: the unterminated form, a bare lower-case Clause only),
`QuotedKeyword`, `QuotedGrantedAbility` (`quotation.QuotationStructure =
Simple`), `GrantedAbilityCoordination` (`quoted_ability_combination` already
has rows `(Keyword, Quoted)`, `(Quoted, Keyword)`, `(Quoted, Quoted)`), the
`QuotedTextSeries` / `GrantedAbilitySeries` instances, and the Sentence
separator `sentence_terminal(QuotedFullStop) => ""`. The coordination tables
are complete; what is missing is an unterminated quotation whose content is
an Ability with sentence-initial capitalisation.

Orchestrator rulings 2026-10-10:

- QA1. Refine `QuotedClause` (the existing unterminated quotation) so its
  content may be an Ability: capitalised, with a cost, a trigger or an italic
  head. No second quotation Category and no new coordination Construction;
  the existing series and `GrantedAbilityCoordination` do the rest.
- QA2. The licence is a declared terminal feature of the quoted content
  (terminated or not), the way `terminal_punctuation` already drives
  `sentence_terminal`. No test on the word after the quote, on a verb, or on
  a card.
- QA3. One analysis per string. The twins to avoid: (a) `It has "{T}: Draw a
  card."` keeps exactly 1 Reading: the final full stop belongs to the
  quotation (`QuotedFullStop`); it must not also read as an unterminated
  quotation plus a Sentence full stop; (b) a lower-case quoted Clause (Ogre
  Marauder) keeps its one analysis and gains no Ability twin; (c) `flying and
  "…"` (keyword first) keeps its Readings.
- QA4. Only the last Sentence of the quoted content drops its full stop. A
  multi-sentence unterminated quotation (`"A. B" and …`) is licensed only if
  attested: record the count either way.
- QA5. A terminated quotation in non-final position (`"….", and`) is
  unattested (`[a-z0-9}]"[,.]` 0 faces): do not license it and do not add a
  check to block it.
- QA6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected (QA3); any increase is reported with
  its shape before landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0.

- Glowcap Lantern: `Equipped creature has "You may look at the top card of
  your library any time" and "Whenever this creature attacks, it explores."`
- Saheeli, Filigree Master: `−4: You get an emblem with "Artifact creatures
  you control get +1/+1" and "Artifact spells you cast cost {1} less to
  cast."`
- Pursued Whale: `… creates a 1/1 red Pirate creature token with "This token
  can't block" and "Creatures you control attack each combat if able."`
- Iron Hills Blacksmith: `… token named Axe with "Equipped creature gets
  +1/+0" and equip {2}.` (control, order swapped: `… with equip {2} and
  "Equipped creature gets +1/+0."` 3)
- Diviner's Wand: `Equipped creature has "Whenever you draw a card, this
  creature gets +1/+1 and gains flying until end of turn" and "{4}: Draw a
  card."`

Sentence probes, 0 today: `It has "{T}: Draw a card" and "{T}: Add {G}."`;
`It has "{T}: Add {G}" and flying.`; `Target creature gains "{T}: Add {G}"
until end of turn.`; `It has "This creature can't block" and is a Wizard.`
Controls that must not change: `It has "{T}: Draw a card."` 1; `It has flying
and "{T}: Add {G}."` 1; `Until end of turn, target creature gains "{T}: Add
{G}."` 1; `Create a 1/1 red Pirate creature token with "This token can't
block."` 3; `Equipped creature has flying and is a Wizard in addition to its
other types.` 3.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face containing `"` (1,145 faces) plus 300 random covered controls,
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; QA6 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3`, once, at the end: a rebuild after a grammar edit takes
about 3 minutes and the test binary about 7, so batch edits. Tests assert the
role (the quotation's content is an Ability; it is unterminated; both
coordinates are Granted Abilities; the QA3 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *…, except it has "…"* after *enter as a copy of* (key `at:" <end>`, 14 of
  its 25 sole-blocker faces): `english-v3-enter-as-copy-except-clause`. The
  quotation there is sentence-final and already reads.
- *For as long as that creature has a bounty counter on it, it has …* (3
  sole-blocker faces under `at:" <end>`): the preposed *for as long as*
  Adjunct fails with a keyword too (`…, it has flying.` 0).
- A quoted keyword with a parameter: `Enchanted creature has "Cumulative
  upkeep {1}."` 0 (2 faces).
- *This creature escapes with "…"* (1 face).
- Quoted `"bands with other"` (8 faces): `english-v3-plugin-term-definitions`.
- Inner failures of the quoted ability itself.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
