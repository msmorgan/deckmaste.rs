---
needs: []
---
**The colour-property adjective class is one member short.** `Create a 1/1
monocolored Servo artifact creature token.` and `This permanent is
monocolored.` both select; replace *monocolored* with *colorless* and both
fail. `vocab Color` holds the five colour words and two of the three
colour-cardinality adjectives, and nothing else in the grammar spells the
third.

Sizing from the 2026-09-05 failure census (measured on change `osnrsxuvkrwo`,
19,198 / 32,641 covered, 13,443 parse failures, first-failure byte attribution;
re-measure at claim): **387 units** — the largest family in the corpus that no
ticket owns.

Not a scanner defect. `docs/tickets/fog.md` recorded this family as "scanner
splits `color` + `less`"; that diagnosis was wrong and is corrected by this
ticket. The probe shows the scanner offering `Noun { noun: Color }` over bytes
`12..18` of `Create a 1/1 colorless Servo…` and the parse failing at `18..22`
expecting `'`, `'s`, `, `, `, then `, `.` — a possessive-adjacency continuation.
`has_lexical_boundary` (`crates/deckmaste_english_v2/src/parser/scan.rs:1184`)
does enforce a right word boundary; the mid-word reading survives only because
an adjacency-marked follower suppresses it, and that branch then dies with no
effect on the outcome. The unit fails for one reason: **no lexeme spans
`colorless`**. Do not "fix the scanner" here.

Defect sentences (census, first-failure offset in the whole-face unit):

- Abstruse Interference — `You create a 1/1 colorless Eldrazi Scion creature
  token.` — `parse failed at bytes 82..86; expected `'`, `'s`, `, `, `, then `, `.``
- Access Denied — `Create X 1/1 colorless Thopter artifact creature tokens with
  flying, where X is that spell's mana value.` — `parse failed at bytes 40..44`
- Adverse Conditions — `Create a 1/1 colorless Eldrazi Scion creature token.` —
  `parse failed at bytes 128..132`
- Probe, attributive: `Create a 1/1 colorless Servo artifact creature token.` —
  `parse_failure span_start=18 span_end=22`
- Probe, predicative: `This permanent is colorless.` — `parse_failure
  span_start=23 span_end=27`
- Controls that select today: `Create a 1/1 monocolored Servo artifact creature
  token.`, `Create a 1/1 white Servo artifact creature token.`, `This permanent
  is monocolored.`

Pinned shape: **add `Colorless = "colorless"` to `vocab Color`
(`crates/deckmaste_english_v2/src/constructions.rs:184`) and change nothing
else.**

The enumeration is complete by construction against the colour system the class
realizes, not against the census. [CR#105.1] fixes the five colour words —
white, blue, black, red, green — all five present. [CR#105.2a..105.2c] fixes
the three colour-cardinality adjectives: monocolored [CR#105.2a] present,
multicolored [CR#105.2b] present, colorless [CR#105.2c] **absent**. Eight
members complete the class; the grammar declares seven. There is no ninth: any
further colour word would have to be a sixth colour, which [CR#105.1] excludes.

The four consumers of `lex Color` are all standalone `lex(color)` forms — no
host fusion, nothing embedded inside another form — so the new member reaches
every position the existing members reach with no construction work:

```
1751:    construction predicative_color: PredicativeColorComplement {
1753:        form predicative_color = lex(color);
2463:    construction color_modifier: NominalModifier {
2469:        form color_modifier = lex(color);
2562:    construction non_color_modifier: NominalModifier {
2568:        form non_color_modifier = prefix("non", lex(color));
4082:    construction fused_color_nominal: Nominal {
```

Witness chains, checked link by link for constituency:

- `Create a 1/1 colorless Servo artifact creature token.` — `lex Color` →
  `color_modifier` (form `lex(color)`, a standalone `NominalModifier`) →
  nominal → object. Every link is a node; `monocolored` selects through this
  exact chain today.
- `This permanent is colorless.` — `lex Color` → `predicative_color` (form
  `lex(color)`) → `PredicativeColorComplement` → `PredicativeComplement`
  (`:1236`) → copular predicate. `monocolored` selects through this exact chain
  today.
- `fused_color_nominal` and `non_color_modifier` take the member with no
  further work; both derive `onset` from the member (`derive onset =
  color.onset`), so *colorless* is consonantal like *monocolored* and the
  *a*/*an* choice needs no special handling.

Affected subset. Surface: `\bcolorless\b`. Touched constructions:
`color_modifier`, `non_color_modifier`, `predicative_color`,
`fused_color_nominal` — every card whose parent-tip selected path contains one
of the four, because the vocabulary they read gains a member. Witnesses: the
three census cards above. Negatives that must not move: any card using
*monocolored*, *multicolored*, or a bare colour word in those four positions.

Pre-ruled, so no STOP is spent on it: the new member makes
`non_color_modifier` admit *noncolorless* and `fused_color_nominal` admit a
fused *colorless* head. **That is correct and is not a defect** — attestation is
provenance, not a filter (rewrite ADR), and a landed construction admits its
full linguistic domain whether or not the corpus prints it. What *is* a STOP is
a real corpus identity that starts selecting a **wrong** analysis; check the
four consumers for that specifically.

Ruled against.

- A `colorless` form literal in any construction, or a construction minted to
  spell it. The class member is the whole change.
- Touching the scanner, `has_lexical_boundary`, or the adjacency suppression.
  The mid-word `color` reading is a dead branch, not the cause.
- Adding *colored*, *hybrid*, or any further member. The class is closed at
  eight by [CR#105.1] and [CR#105.2a..105.2c].
- Narrowing an existing consumer to keep a number.

STOP-and-report: any `require` or `checked by` naming a colour, the new member,
a noun, or a card; any dominance edge or exception entry added to resolve a
rivalry the new member exposes.

Routed, not in scope. The vocabulary is **misnamed**: [CR#105.4] states
"'Multicolored' is not a color. Neither is 'colorless.'", and the class already
holds *monocolored* and *multicolored* today, so `vocab Color` names a colour
system while holding a colour-**property** class. This ticket does not create
that defect and does not fix it; adding the eighth member sharpens it. Mint the
rename at landing (blast radius measured 2026-09-05: five sites in
`constructions.rs` plus `ast.rs`, nothing outside `deckmaste_english_v2`), and
record it as a routed obligation in the landing record.

Acceptance. The standard landing record (PROVE / DISCLOSE / REPORT per
CLAUDE.md and the rewrite ADR's 2026-09-04 amendment), plus:

- the five probe sentences above select, with *colorless* rendering byte-exact
  in both attributive and predicative position;
- the three controls still select and their selected analyses are unchanged;
- the construction count is unchanged — this landing adds no construction;
- both byte-exact laws green with total ownership, zero unresolved ties;
- `docs/tickets/fog.md`'s "scanner splits `color` + `less`" line is deleted, not
  merely re-counted: the diagnosis it records is wrong.

Baseline: measured on change `osnrsxuvkrwo`, 19,198 covered of 32,641, 13,443
parse failures, 0 unresolved ties, 0 internal failures. Re-measure at claim.

Glossary. `docs/contexts/oracle-english/CONTEXT.md` defines no colour term at
all. If the landing needs one — for the class this vocabulary holds, as
distinct from the Game Model's colours — amend that glossary through the
`domain-modeling` skill as part of the work and disclose it.

Tier: **terra** — one declared vocabulary member, four consumers verified as
standalone forms, no construction, no seam, no compiler work.

Standard constraints apply.
