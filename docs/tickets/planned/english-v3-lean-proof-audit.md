---
needs: [english-v3-lean-grammar-model]
---
# Make the v3 English model's claims nonvacuous

Audit the rebuilt English workbench before Rust treats it as design evidence.
Every named law must range over structures the modeled grammar can derive, use
the hypothesis it claims to test, and distinguish a real counterexample when
the law is weakened. Replace or retire the old proof-gap inventory according to
the new model rather than narrowing its statements until they become automatic.

The audit must account explicitly for Tense versus Finiteness and Word Form;
flat serial-comma Coordination versus genuine nested Coordination; head-owned
countability in genitives; correlated alternatives that are not a Cartesian
product; extraction and anchor projections; ordinary-word payload boundaries;
duplicate derivations; and previously unwitnessed adjective, mass-noun,
placement, sharing and auxiliary cases. Resolve the three unrelated public
`Reading` concepts and add the missing Oracle English glossary terms used by the
v3 model.

Acceptance gives every inherited proof-gap and residual-audit item a named
disposition: structurally repaired, superseded by the accepted v3 relation, or
transferred to a live implementation ticket with the obligation intact. No
theorem disappears without its replacement claim being named. The project
builds without `sorry`, the axiom audit is unchanged, and each central exclusion
fails under a deliberately weakened premise or has another non-circular
nonvacuity witness. Use Lean LSP MCP and `english/scripts/build`. Standard
constraints apply.
