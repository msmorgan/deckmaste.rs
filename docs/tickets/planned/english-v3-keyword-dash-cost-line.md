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

Six items as specified in `docs/tickets/README.md`.
