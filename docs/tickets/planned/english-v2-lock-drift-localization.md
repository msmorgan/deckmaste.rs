---
needs: [english-v2-retire-authenticator-tests]
---
Localize lock drift (normalization-freeze review M1/L1/L5). When the
normalization digest changes, `--check` prints two hashes over 32,641
texts and nothing else; add a per-unit report path (e.g. `coverage
--normalization-diff <old-lock>`) that lists the unit ids and card names
whose normalized text differs, so the required landing-record note can be
written from evidence. Also pin `context_onset` (the one remaining
code-derived unit input) into the digest recipe or a sibling digest, and
delete the two dead schema-migration paths that accreted. Zero identity
change; standard constraints apply.
