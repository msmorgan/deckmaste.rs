---
needs: []
---
**`vocab Color` is a false name.** Its eight members combine the five Color
values ([CR#105.1]) with three color-property adjectives, and [CR#105.4] holds
that neither *multicolored* nor *colorless* is a color. Rename the vocabulary
and all five declaration consumers plus the `ast.rs` use so the Oracle-English
name does not claim that every member is a Game Model Color.

Pinned scope: a behaviour-preserving vocabulary rename within
`deckmaste_english_v2`; no member, construction, scanner, selection rule, or
lexical licensing change. Choose the replacement name against the Oracle
English glossary before implementation and record any glossary amendment it
needs.

Routed by `english-v2-tail-color-property-adjectives` on 2026-09-05. The
measured blast radius is five sites in `constructions.rs` plus `ast.rs`, with
nothing outside `deckmaste_english_v2`.
