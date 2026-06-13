---
needs: []
---
Allow a bare macro name to read as a zero-arg invocation when every param has a
default (`Hexproof` instead of today's required `Hexproof()`). Needs a probe-free
path through serde's one-shot `VariantAccess` or a macro-layer pre-scan.
