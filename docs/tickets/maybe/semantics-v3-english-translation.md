---
needs: [semantics-v3-dsl-and-plugins, english-v4-cutover-v3]
---
**[port-required: semantics-v3] Translate retained `deckmaste_english_v4`
Readings into `deckmaste_semantics_v3` terms** (`docs/decisions/semantics-v3.md`
§0.1, `docs/decisions/english-v4.md`), with the v3 DSL as the cached form.
Parked until `english_v4` is cut over and the `Definition` / `Template` bodies
carry their spellings. Distinct from `deckmaste_migrations`' regex extraction.
