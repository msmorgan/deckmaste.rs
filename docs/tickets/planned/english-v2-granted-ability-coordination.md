---
needs: []
---
# Granted keyword lines and quoted abilities coordinate through the general machinery

**R6 — Group R.** Takes over the granted-line residue that
`english-v2-with-preposition` routed to
`english-v2-underspecified-adjunct-attachment`; that routing is superseded for
this item only (the attachment residue stays there).

Defect. `granted_keyword_line`
(`crates/deckmaste_english_v2/src/constructions.rs:4780`) is
`items: seq KeywordLineItem separated by " and "` — one uniform separator and one
member type. So a comma-separated line fails
(`with trample, haste, and "This creature can't block."`), a line mixing keyword
items with a quoted ability has no path (`Quoted` is a sibling
`PrepositionalComplement` arm, not a line member), and the postmodifier form
fails on `…a creature card with deathtouch, hexproof, reach, or trample…` while
`with deathtouch` parses. Same shape on the verb side: `Have`'s quoted-ability
frame takes one `QuotedAbility`, so `Enchanted creature has "…" and "…"` fails.

Pinned shape. Coordination is general machinery. Route the granted line through
the ordinary nominal/predicate coordination algebra — the positional separator
sequence with `and` / `or` / `and-or` as distinct semantic constructions, per the
rewrite ADR's declaration-language section — over a member category that admits
both a keyword-line item and a quoted ability. No per-grant coordination family,
no second separator table, no `require` naming a keyword.

Fences. A coordination family scoped to grants. A closed list of coordinable
keywords. A `checked by` naming a keyword lexeme or a card. Adding the `or` arm
only because a witness was found — all three coordinators are declared regardless
of counts.

Glossary: Coordination, Coordinator, Keyword Line Item, Quoted Ability, Granted
Ability. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.
