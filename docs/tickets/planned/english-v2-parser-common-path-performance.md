---
needs: [english-v2-plan-09-effect-and-predicate-grammar]
---
Investigate and optimize the English-v2 full-corpus analysis path without
narrowing grammar, discarding candidates, adding semantic ranking, omitting
corpus units, or changing the 16.26-second hard ceiling.

Begin with a scheduled v1-versus-v2 performance investigation on the same
machine, toolchain, corpus snapshot, build profile, and separately launched
command boundaries. Attribute compilation, environment construction, corpus
loading, parsing/chart work, rendering, ownership inspection, selection, and
serialization independently before choosing an optimization. Optimization is
Plan 10-or-later work and follows the measured causal evidence.

The unchanged Plan 09 boundary samples carried by this ticket are:

~~~text
ELAPSED expand 3.49
ELAPSED report 2.09
ELAPSED parse 84.68
ELAPSED roundtrip 78.69
ELAPSED ambiguity 82.29
ELAPSED coverage 85.75
ELAPSED require-complete 79.92 EXIT 1
~~~

Expand and report passed the hard ceiling while exceeding their warning
thresholds. Parse, round-trip, ambiguity, coverage, and expected-incomplete
all breached it. The common full-corpus analysis path is the first diagnosis:
those five commands cluster at 78.69–85.75 seconds, while expansion and
declaration reporting complete in 3.49 and 2.09 seconds.

Acceptance keeps byte-exact rendering, total ownership, zero ties/exceptions,
the complete 32,641-unit outcome partition, and the ordinary add-only coverage
ratchet unchanged while every separately launched command satisfies the
16.26-second ceiling. Record fresh v1/v2 baselines, profiles, the chosen
architectural repair, and post-repair samples.
