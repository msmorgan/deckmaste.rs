---
needs: []
---
# Document the reserved child names as the grammar contract

From `english-v4-child-name-control-and-category-fission-audit`, findings A
and B.

A child's name is its grammatical function (`Production::names`), and the
engine keys behaviour on some of them. Nowhere is that listed. Authors find out
by accident, and rename a head to dodge implicit agreement. The renamed child
then also loses its head lexeme in Readings and its place in the head line.

## Change (documentation only)

1. In the `deckmaste_syntax` README, add a table of the reserved names and what
   each one does:
   - `head`: implicit agreement; preposition and extension inheritance; the
     head line; Reading head lexeme; host words for Right Association;
     function-Category wrapping.
   - `first`, `last`, `rest`: the head line; conjunct balance and the
     coordination Attachment Class; the fronted-scope rule.
   - `adjunct`, `modifier`: unselected dependents for Right Association and
     the preposed-dependent Attachment Class. `adjunct` also marks the
     post-verbal Adjunct host that an auxiliary Verb Phrase never is.
   - `supplement`: reversed Right Association.
   - `complement`: carries a delimited Clause coordination up.
   - `object`, `shared`: Reading functions for postposed Modifiers and Right
     Node Raising.

   Point to the code site of each one. Say that a Production naming its
   semantic head otherwise forgoes all of the `head` items. In
   `crates/deckmaste_syntax/src/grammar.rs`, have the `ProductionDecl` and
   `SymbolDecl` docs point to the table.
2. In `crates/deckmaste_english_v4/grammar/english.ron`, give each Production
   that names its semantic head otherwise a comment naming the features it
   keeps out of head agreement:
   - DistributiveCoordinationDetermined (`nominal`);
   - TransparentQuantity and TemporalQuantity (`quantity`).
3. Fix comment drift in `english.ron`:
   - the LocativeHead comment sits above SymbolHead (about lines 130–131);
   - the FusableDeterminer comment sits above PositionPhrase (about lines
     195–196);
   - the NounPhrase comment stops mid-sentence ("… never passes up a Measure
     Gap, or", about line 157).

No grammar or engine behaviour changes. The census and judgment set are
identical.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
