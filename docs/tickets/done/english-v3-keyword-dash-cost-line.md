---
needs: []
---
# Read a keyword whose cost follows a dash: Ward—Pay 3 life. / Flashback—{1}{R}, Pay 3 life. / Equip—{3}

## Why

The blocker report at trunk `vwlkmvns` puts 152 unread faces under the key
`at:. <end>` (the line never forms a root). The 2026-10-10 triage found 114 of
them are one shape: a cost-taking keyword, an em dash, then a cost list ending
in a full stop. The same shape is the keyword part of the `life .`,
`graveyard .` and `card .` keys; about 150 faces in all (upper bound).
"Flashback {1}{R}" reads; "Flashback—{1}{R}, Pay 3 life." reads 0; even
"Equip—{3}" reads 0. A cost list already reads before a colon:
"{2}, Pay 2 life: Draw a card." reads.

## Goal

A keyword that takes a cost parameter also reads with its parameter introduced
by a dash and written as a cost list (mana symbols and/or imperative cost
components, comma-separated, final full stop), with byte-exact realisation.

## Analysis

A cost-taking keyword has a cost parameter;
Oracle writes a non-mana or mixed cost after a dash. No CGEL citation applies:
this is an Oracle template, cited as a project ruling.

Existing routes: the keyword parameter separator (`keyword_parameter_separator`
and the `Dash` separator, `crates/deckmaste_english_v3/src/declarations.rs`
near the keyword separators) and the Cost category that the activated-ability
form reads before its colon.

Orchestrator rulings 2026-10-10:

- KD1. Refine the existing keyword-parameter route: its separator admits the
  dash form when the parameter is a Cost list, licensed by what the keyword's
  `grammar:` declaration says its parameter is (a cost). No new Construction
  per keyword and no keyword named in the grammar.
- KD2. The dash form's parameter is the existing Cost category. The final
  full stop is a selected literal of the form, not stored in the AST.
- KD3. One analysis per string: "Flashback {1}{R}" keeps exactly its Readings,
  and a mana-only cost after a dash ("Equip—{3}") is read only by the dash
  form.
- KD4. Ability words ("Landfall — Whenever …") are a different line form and
  must keep their Readings; the dash form applies only where the left side is
  a cost-taking keyword.
- KD5. Keywords whose dash parameter is not a cost ("Choose a Background",
  "Enchant—…", named modes "• Name — …") are out of scope.

## Witnesses

Re-probe on the claim parent first; expected 0 before:

- "Ward—Pay 3 life."
- "Flashback—{1}{R}, Pay 3 life."
- "Equip—{3}"
- "Escape—{3}{U}, Exile four other cards from your graveyard."
- "Eternalize—{2}{W}{W}, Discard a card."

Controls that must not change: "Flashback {1}{R}"; "Ward {2}"; "{2}, Pay 2
life: Draw a card."; "Landfall — Whenever a land you control enters, you gain
1 life."

## Method

Standard constraints apply. Bounded census: every supported face whose
analysed text matches `[A-Za-z]—` plus 300 random covered controls. Zero lost;
any Reading-count increase on a covered face is reported with its shape
before landing (KD3, KD4 make most of them defects).

## Landing record

1. **Census delta** (`vwlkmvns` → `kxwsyulp`): bounded covered faces
   302 → 453; total Readings 47,344 → 50,361. Gained 151; lost none;
   decreases, increases on previously covered faces, and digest-only changes
   all zero. Selector: every supported analysed face matching `[A-Za-z]—`
   (238), plus 300 random covered faces, seed 20261008: 538 identities.
   The claim parent `rsoqootn` differs from the baseline only in ticket docs.
   Evidence: `target/english-v3/english-v3-keyword-dash-cost-line/`
   (`manifest.json`, `before.json`, `after.json`, `diff.json`; builtin diff
   also run with `--lines`). After refresh, `rqrtmmyz` measures covered
   faces 454 and total Readings 50,364: gained 152 against `vwlkmvns`, with
   lost, decreases, increases, and digest-only changes still zero.
   Builtin `refresh-delta.json` isolates one additional gain, Walk the Aeons
   (3 Readings), from trunk `nouqyyouuops`'s transitive Take declaration;
   the other 537 selected faces are unchanged. Final evidence is
   `after-refresh.json` and `diff-refresh.json` in the same directory.
2. **Witnesses** (`kxwsyulp`, complete Document enumeration):
   “Ward—Pay 3 life.” 1; “Flashback—{1}{R}, Pay 3 life.” 1;
   “Equip—{3}” 1; “Escape—{3}{U}, Exile four other cards from your
   graveyard.” 3; “Eternalize—{2}{W}{W}, Discard a card.” 1.
   Each read zero on the claim-parent tree before implementation. The four
   controls retain their claim-parent counts (1, 1, 1, 2 respectively).
   All after probes are byte exact, complete, and issue-free at `kxwsyulp`.
3. **Spot checks** (`kxwsyulp`): ten newly covered faces select the existing
   CostKeyword dash form with a Cost parameter. Aboroth: imperative Put cost;
   Acorn Harvest: mana symbols then Pay life; Axebane Ferox: imperative
   Collect evidence cost; Flash of Defiance: mana symbols then Pay life;
   Sweet Oblivion: mana symbols then Exile cards, retaining the existing
   preposition attachments; Sunscourge Champion: mana symbols then Discard
   a card; Demonmail Hauberk: Sacrifice a creature; Deepcavern Imp: Discard
   a card; Dragon's Eye Savants: Reveal a blue card with the in-hand
   preposition attachment; Constant Mists: Sacrifice a land. The final full
   stop is the selected terminal literal, outside the Cost tree. Scoped
   keyword-phrase probes are complete, byte exact, and issue-free.
4. **Deviations and STOPs**: no STOPs and no new production Construction or
   lexical Frame. Selecting the terminal while preserving the existing Cost
   tree required generic compiler support for `last(items).Feature`, with a
   declared scalar default for an empty repeat. Cost Component Form was a
   glossary gap; it is now defined in the owning Oracle English context,
   with the Cost citation and KD2–KD3 distinguished. No route is superseded:
   the existing space form is retained under KD3. Beyond the literal witness
   checks, two compiler diagnostic tests and a synthetic four-Construction
   fixture (Symbols, Action, Tail, List) test the projection and both
   roundtrip laws. The Landfall control asserts the complete value set,
   retaining both ability-word and flavor-word trees shown by the before
   probe.
   Accepted review (2026-10-10, `rqrtmmyz`): the ticket's witness
   “Equip—{3}” is **UNATTESTED**. Zero supported lines end in a symbols-only
   dash cost; the only symbols-final dash costs are the five
   “Suspend X—{X}{W}{W}. X can't be 0.” lines, which carry a full stop.
   Thus `(Symbols, None) => ""` in `keyword_cost_terminal` rests on ruling
   KD3 alone and must be revisited when the Suspend X— form is ticketed.
   The review reports 41 still-unread dash lines, with these class counts:
   named Partner variants 19; non-Cost parameter classes: Suspend X— 5,
   Splice onto Arcane— 5, Modular—Sunburst 1; Cost-internal gaps and riders 16.
5. **Tests** (`kxwsyulp`): `cargo test -p deckmaste_english_v3` passed
   (735 tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed; `cargo test -p deckmaste_construction_v3_core` passed (36 tests);
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`
   passed. After the vocabulary-bearing refresh, all four commands passed
   again in full at `rqrtmmyz` (736 English tests, 36 construction-core
   tests). Both doc-test suites passed with no tests. This ticket restored 0,
   re-spelled 0, ignored 0, added 7, and removed 0 tests; no removals to justify.
   Citation check reports no noncompliant citations and zero stale citations;
   bless introduced no lock change, and the piped diff audit's sole citation
   was read against its rule text.
6. **Timing** (`kxwsyulp`): 19:15.97 wall time, 6 workers, under
   `/usr/bin/time -v`, including the xtask dependency build. This is the
   bounded census required by the binding verification budget.
