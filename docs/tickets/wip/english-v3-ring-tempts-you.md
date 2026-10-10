---
needs: []
---
# Read the keyword action declared as a clause, and its designation noun: The Ring tempts you. / your Ring-bearer

## Why

`plugins_v2/builtin/macros/keyword_actions/theRingTemptsYou.ron` declares
`grammar: FixedClause(surface: "the Ring tempts you")`, the only `FixedClause`
in the plugin tree. The loader
(`crates/deckmaste_lexical_source/src/legacy/plugins.rs`, the `FixedTerm |
FixedClause` arm) turns it into one invariant lexeme of Category `Keyword`
(`lexeme:keyword_action/theRingTemptsYou`) spanning all four words. No
Construction reads a `Keyword` lexeme as a Clause, and *Ring* and *tempts*
are not words on their own, so nothing reads. Blocker report at trunk
`uoqyuxtn`: key `at:the ring` 41.

`cargo xtask english-v3 grep '\b[Tt]he Ring tempts you\b'` finds **54** faces
(58 matches), all unread; **45** have that line as their only failing line.
Positions: main clause 48 (*When this creature enters, the Ring tempts you.*,
*The Ring tempts you.*, *…, then the Ring tempts you*), trigger condition
*Whenever / When the Ring tempts you, …* 8, perfect *the Ring has tempted you
N or more times this game* **2** faces (0 sole). `grep 'Ring-bearer'` finds
**12** faces, all unread (9 sole-line; 8 of the 12 lines have no *tempts*);
the designation `designations/ringBearer.ron` is `FixedTerm(surface:
"Ring-bearer")` and "your Ring-bearer" reads 0 as an NP, while "your
commander" (`Noun(singular: …)`) reads 1.

## Goal

*the Ring tempts you* reads as an ordinary finite clause in main, coordinated
and trigger-condition position, and *Ring-bearer* reads as a count noun
(*your Ring-bearer*, *Ring-bearers*), with byte-exact realisation.

## Analysis

No CGEL claim is needed: Subject *the Ring*, verb *tempt*, Object *you* is a
canonical transitive clause, and the trigger and perfect forms are that
clause under existing Constructions. The choice between a fixed clause leaf
and a compositional clause is a project ruling.

Existing routes: `FiniteClause` and `DeterminedNounPhrase`
(`crates/deckmaste_english_v3/src/declarations.rs`) need no change. The
nearest declaration precedents are `ventureIntoTheDungeon.ron` (`grammar:
Verb(bare: "venture into the dungeon", third_person: …, frame_set:
Intransitive)`; "Venture into the dungeon." 1, "Whenever you venture into the
dungeon, draw a card." 1), the `verb:` field of `FixedKeyword`
(`keyword_verb_lexeme` in the loader) and designation nouns such as
`monarch.ron` (`grammar: Noun(singular: "monarch")`).

Orchestrator rulings 2026-10-10:

- RT1. Compositional, not a clause leaf. The contract counts no opaque
  catch-all leaf as realisation, and a leaf cannot read the attested perfect
  *has tempted you*. No Construction that turns a `Keyword` lexeme into a
  Clause is added.
- RT2. Data and declaration reader only. The words are *Ring* (a count noun
  written with an initial capital; *the Ring* is an ordinary definite NP) and
  the transitive verb *tempt* (plain, *tempts*, *tempted*) with one Object
  frame. They are declared by the plugin definition, so their provenance stays
  `theRingTemptsYou`: extend the `FixedClause` declaration with the noun and
  verb it is made of (as `FixedKeyword` carries `verb:`) and have the loader
  mint those lexemes. If that needs more than the declaration schema, its
  reader and the loader arm, STOP.
- RT3. The four-word `Keyword` lexeme is no longer minted for English; if
  another consumer still needs the fixed surface, it keeps reading the
  declaration, not an English lexeme. One analysis per string: *the Ring
  tempts you* must not also read through any keyword route.
- RT4. *Ring-bearer* becomes `grammar: Noun(singular: "Ring-bearer")`
  (plural by the default rule; *Ring-bearers* is attested once), keeping
  `lexeme:designation/ringBearer` as owner.
- RT5. No guard names *Ring*, *tempt* or the action. Nothing restricts
  *tempt* to this Subject and Object: the corpus has no other use, and the
  pruning rule forbids a check whose only job is to block one.
- RT6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected (no covered face contains *Ring* or
  *tempt*); report any with its shape before landing. Plugin definitions
  change, so `cargo xtask lean check` and `cargo xtask facts check` run.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--readings 3`): witness 0, control as shown (3+ =
enumeration capped). Re-probe on the claim parent first.

- Uruk-hai Berserker: "When this creature enters, the Ring tempts you."
  (control "When this creature enters, you become the monarch." 1)
- Witch-king of Angmar (document): "Whenever one or more creatures deal
  combat damage to you, each opponent sacrifices a creature of their choice
  that dealt combat damage to you this turn. The Ring tempts you." (control
  "… You draw a card." 3+)
- One Ring to Rule Them All (document): "I — The Ring tempts you, then each
  player mills cards equal to your Ring-bearer's power." (control "I — You
  draw a card, then each player mills cards equal to your commander's power."
  1)
- Lord of the Nazgûl: "Wraiths you control have protection from
  Ring-bearers." (control "… protection from Zombies." 2)

Also 0 today: "Whenever the Ring tempts you, draw a card." (control "Whenever
you venture into the dungeon, draw a card." 1); NP "your Ring-bearer" (NP
"your commander" 1). Controls that must not change: "Venture into the
dungeon." 1; "You become the monarch." 1; "Put a +1/+1 counter on your
commander." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(Ring|Ring-bearers?|monarch|initiative|[Vv]entures?|commander)\b

(355 faces: the 59 Ring faces and the keyword-action and designation control
classes) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent.
Zero lost; RT6 for increases. Full `cargo test -p` and `cargo clippy -p …
--tests -- -D warnings` for `deckmaste_lexical_source`,
`deckmaste_construction_core` (it owns the `FixedClause` declaration and
recipe) and `deckmaste_english_v3`, plus `cargo test -p xtask english_v3`,
once, at the end. No grammar edit is expected, so no grammar rebuild; plugin
`grammar:` data loads at runtime. Tests assert the role (Subject NP, verb,
Object, the trigger-condition clause, the designation as head noun under a
genitive Determiner), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *if you chose a creature other than <name> as your Ring-bearer* (4 faces:
  Faramir, Galadriel of Lothlórien, Gandalf, Aragorn): the control "… if you
  chose a creature other than Galadriel as your commander, scry 3." reads 0;
  *choose NP as NP* is a separate frame gap. These faces stay unread.
- *the Ring has tempted you two or more times this game* (2 faces, neither
  sole): in scope only if RT2 plus the existing perfect and frequency routes
  read it; otherwise list.
- Any change to the keyword action's body or to Semantics.

## Landing record

1. **Census delta** — Claim-parent evidence `mqrlzszk` (identical bounded Readings to
   the coordinator census; claim `yptsourp`) → `sztumtmq`: 498 → 542 covered faces,
   8,868 → 9,361 Readings. 44 gains; lost faces none; decreases, increases and
   digest-only changes all zero; zero internal issues. Selector
   `\b(Ring|Ring-bearers?|monarch|initiative|[Vv]entures?|commander)\b` found 315
   faces; the diff-generated 300 covered controls (seed 20261008) give 614 distinct
   identities. After refresh at `nskzmmvt`, 543 covered faces and 9,530
   Readings: the sole additional gain is Thunderclap Drake (169 Readings),
   inherited from `zunolrnv`'s preverbal *next*. Final totals are 45 gains and
   zero lost/decreased/increased/digest-only faces, with zero internal issues.
   Evidence: `target/english-v3/english-v3-ring-tempts-you/`, including
   `before.json`, `after.json`, `diff.json`, `manifest.json`,
   `after-refresh.json`, `refresh-diff.json` and `final-diff.json`.
2. **Witnesses** — Claim-parent probes at `mqrlzszk` → `sztumtmq`: Uruk-hai
   Berserker 0 → 1; Witch-king of Angmar's document passage 0 → 57; One Ring to
   Rule Them All's chapter passage 0 → 1; Lord of the Nazgûl's protection line
   0 → 2; trigger fragment 0 → 1; NP *your Ring-bearer* 0 → 1. No witness was
   already covered. All controls retain their measured counts (Witch-king
   control capped at 3+, the others exact); the three additional action/
   commander controls each retain 1.
3. **Spot checks** — At `sztumtmq`, ten gained faces have the intended selected
   analysis: Bilbo, Retired Burglar — finite consequence after the coordinated
   enters/leaves condition; Birthday Escape and Bombadil's Song — a separate
   finite Ring clause; Dúnedain Rangers — an indefinite Ring-bearer Object and
   a finite Ring consequence; Elrond, Lord of Rivendell — a finite Ring
   consequence under *if*; Frodo Baggins — a finite Ring consequence and a
   possessive Ring-bearer NP as copular Complement; Gollum, Patient Plotter —
   a finite Ring consequence after *when*; Lord of the Nazgûl — plural noun as
   protection's Nominal Keyword Quality; One Ring to Rule Them All — finite
   coordinated Ring clause and a genitive Ring-bearer NP in the power comparison;
   Witch-king of Angmar — a separate finite Ring clause. Each clause has
   *the Ring* as Subject, *tempt* as transitive head and *you* as Object.
   No unintended-only gain found in these checks.
4. **Deviations and STOPs** — No new Construction or frame kind; schema,
   declaration reader and loader changes follow RT2, with the opaque Keyword
   lexeme retired and the semantic fixed surface retained under RT3. No STOP
   or glossary gap. Refresh's test-registration conflict was resolved on
   `sztumtmq`, retaining Ring and preposed-frequency exactly once in `suite_5`;
   the concurrent bank refusal was retried successfully. The perfect-frequency
   fragment remains unread after refresh at `nskzmmvt`, as RT2's
   allowed residual on Frodo, Adventurous Hobbit and Frodo, Sauron's Bane;
   *choose NP as NP* stays out of scope. Lean checks omitted under the explicit
   user budget. A reasoned length expectation preserves the closed declaration
   dispatcher; the designation test's expected-recipe table was extracted
   without changing its value assertions.
5. **Tests** — After refresh at `nskzmmvt`, passed `cargo test -p deckmaste_english_v3`
   (810), `cargo test -p deckmaste_lexical_source` (82),
   `cargo test -p deckmaste_construction_core` (473), and
   `cargo test -p xtask english_v3` (56 selected tests). Passed
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`;
   initial English/core clippy failures were the length/default style issues
   resolved above. `cargo xtask facts check`,
   `cargo xtask cite check --list-noncompliant`, `cargo xtask cite check`, and
   `jj --no-pager diff --git | cargo xtask cite audit --diff` passed;
   no new citations need blessing. Assurance at `sztumtmq`:
   restored 0 / re-spelled 2 / ignored 0 / added 6 / removed 0. The action
   inventory test additionally retains and strengthens its existing assertions.
6. **Timing** — Final bounded census at `nskzmmvt`: 18.78 s wall time under
   `/usr/bin/time -v`, 6 workers, 614 faces, `--samples-per-face 0`.
