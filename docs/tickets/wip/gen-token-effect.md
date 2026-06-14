---
needs: []
---
Generator task. Parse token-creation effects in the extract→graduate pipeline:
"create N <P/T> <colors> <subtypes> creature token(s)" → a `Create`/`TokenSpec`
effect, including tokens carrying granted (quoted) abilities (extract.rs already
normalizes the token self-reference). The engine side — `TokenCreated`, the
synthesized token `CardInstance` — is built ([[engine-tokens]]); this is the
parse/template side, previously design-gated. Highest-priority generator gap for the
demo: token swarms are both tribes' go-wide identity, and even a fixed-count maker
("create three 1/1 red Goblin creature tokens") needs only this. `effect.rs` today
parses just damage and draw. Feeds canon-goblins-elves; part of tui-client.
