---
needs: []
---
# Export verbless Predicate permission from the state-Adjective class

Replace the one-member *saddled* licence with the declared state-Adjective
class permission, preserving existing owners and one adjectival route.
Standard constraints apply.

## Analysis

Owner correction (user, 2026-10-10, verbatim): "hypothetical lines such as attacks while equipped/enchanted/haunted and shit should work too, not a specific construction for saddled".

- **VW7** (user, 2026-10-10): permission belongs to the state-predicate class
  by derivation, with no member list. Every existing state/participial
  Adjective exports `VerblessPredicateUse: Yes` through its declared class,
  origin or participial derivation: keyword states, status Adjectives, and
  adjectival past participles already predicative after *be/become/remain*.
  Delete the one-member class row. Each derivation exports the permission.
  If a class has no declared marker, STOP and report it; do not list words.
- **VW8** supersedes VW2's speculative-member exclusion and VW4 as needed:
  *while tapped/equipped/enchanted/haunted/blocked* must read even where
  unattested. Reuse the adjectival route of *is tapped*, with no subjectless
  passive twin. W9-A preserves independent ambiguity; record each count
  beside its *is <word>* control.
- **VW9**: before/after probes for *This creature gets +1/+1 while saddled.*,
  *This creature attacks while equipped/enchanted/haunted/tapped/blocked.*,
  plus previous witnesses and controls. Selector: every face containing
  *while*, *if able* or *as long as*, plus 300 random covered faces, seed
  20261008. Bounded official census/diff only. Zero covered-face changes
  expected; report any increase and its shape. Full suites and clippy for
  touched crates. Tests assert class behaviour with at least four members
  from different derivations, not list equality. Six-item landing record and
  standard refresh rules apply. If covered faces change or a class lacks
  a marker, STOP. Otherwise finish, refresh, integrate, drop, and report the
  change id, class markers and probe table without another review pause.

## Method

Inspect the existing state-Adjective derivations and declarations; re-probe
all witnesses and controls before implementation. Census selector
`\bwhile\b|\bif able\b|\bas long as\b` case-insensitively, union with the
300 official random covered controls. Use an identity manifest, six workers,
zero samples per face and `/usr/bin/time -v`; compare with the official diff.

## VW7 STOP: missing haunted state-Adjective declaration

Inspection and complete before probes at `znzxkzuk` establish:

- Native participial-Adjective classes declare `AdjectiveOrigin: Participial`
  for the existing tap/untap/block and exile/crew derivations.
- Keyword `participial_adjective` recipes derive equipped, enchanted and
  suspended through `legacy::plugins::participial_adjective_lexeme`.
- Designation Adjectives, including saddled, come from declared Designation
  identities with `grammar: Adjective`; they have a different export path.
- **Haunted has no declared state-Adjective derivation or class marker.**
  `plugins_v2/builtin/macros/keyword_abilities/haunt.ron` declares a Verb
  export only, with no `participial_adjective` recipe; neither native
  participial class includes it. The complete *This creature is haunted.*
  probe contains one `PassiveComplement` / `PassivePredicate` Reading over
  `lexeme:keyword_ability/haunt/verb`, not an Adjectival Complement.
  *This creature remains haunted.* reads 0. Deriving an Adjective from every
  verbal past participle would exceed the existing state-Adjective class;
  adding a word-specific member would contradict VW7. STOP for a ruling on
  the missing declared state-Adjective source before implementation.

Requested before probes (complete, 0 issues; *is* and *remains* controls use
*This creature* as Subject):

| Predicate | Requested while witness | is control | remains control |
|---|---:|---:|---:|
| saddled (*gets +1/+1*) | 1 | 2 | 1 |
| equipped (*attacks*) | 0 | 1 | 1 |
| enchanted (*attacks*) | 0 | 1 | 1 |
| haunted (*attacks*) | 0 | 1, passive only | 0 |
| tapped (*attacks*) | 0 | 2 | 1 |
| blocked (*attacks*) | 0 | 2 | 1 |

Additional state controls, *is/remains*: untapped 2/1, crewed 2/1,
exiled 2/1, suspended 1/1. Prior Ghoda, Jaguar, Mascot and Dracosaur witnesses
already read 1 each; their controls without *while saddled* also read 1
and are retained as controls, not failing witnesses. The local *attacks
while saddled* reads 1, *has flying while tapped* reads 0; finite *while
it's saddled* 2, ordinary attack-trigger control 1, finite *while it's
tapped* 4. Every probe enumerated completely with 0 issues.

Evidence: `target/english-v3/english-v3-verbless-state-predicate-class/`
contains `probe-inputs.json` and `before-probe-*.json`, produced with
`cargo xtask english-v3 probe --text … --category sentence --readings 64
--json` (plus `--as-card` for the previous card witnesses and controls).
No implementation, test, or lexicon change has been made. No census, suite
or clippy was run because the required class-marker STOP occurs first.
