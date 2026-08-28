Make keyword-line admission generative (parameter-landing-review F1-F3).
The landed design welded cost-shape categorization into parse admission:
abstract sum KeywordLineItem grew to 12 members, one per payload-vector x
separator-layout pair with hardcoded separator literals, so admissible
variants ("Ward—{2}", "Cycling—{2}", "Reinforce 3 {1}{G}") reject. Built
faithfully from a pre-amendment ruling text; the amended ruling: admission
admits separator variants (space or em dash with a mana cost; em dash
with a sentence-shaped cost; a sentence cost without a dash stays
ill-formed — the dash is a constituent boundary there).

- F1: introduce the separator as a constituent (a KeywordCostSeparator
  sum), collapsing the 12-member cross product. IMPORTANT STOP FENCE:
  construction_core has no parse-many/render-one canonicalization
  mechanism; if the fix appears to require building one, STOP and report
  the design question before writing compiler capability. Byte-exact laws
  on attested (house-styled) units must hold throughout; expect near-zero
  coverage change — this ticket buys generative fidelity and structural
  economy, not units.
- F2: KeywordSubject takes the general reference-phrase category — the
  landed fix added a third enumerated member (Coordination) instead; 13
  Enchant units still fail on postmodifiers the ordinary reference stack
  already parses.
- F3: 10 authored parameter vectors are inert ([Ability] x8, [Condition],
  [Cost, Power, Toughness]) with no consuming codec, and ParameterType is
  an unvalidated String — validate declared types against the consumable
  set so an inert vector is a load error, and either consume or explicitly
  defer the three shapes (deferral = a named unsupported class, not
  silence).
- Carried loose ends: planar-controller stub note and the vacuous
  VerbLexeme assertion (xtask report.rs:799) from earlier reviews.

Stub-side separator/layout fields remain banned (the relapse test stays).
Standard constraints apply.
