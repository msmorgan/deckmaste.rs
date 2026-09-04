---
needs: [english-v2-np-postmodifiers, english-v2-require-through-optional-role]
---
**Replace the overloaded weak `Agreement::{Bare, ThirdPersonSingular}` axis
with `ConcordClass::{Other, ThirdPersonSingular}`.** Use the [`Agreement`,
`Concord Class`, `Finiteness`, and `Inflectional Form`
definitions](../../contexts/oracle-english/CONTEXT.md). Concord Class is the derived two-way
morphological equivalence class needed by relevant present-tense paradigms; it
is not underlying Person and Number, Finiteness, or an Inflectional Form.

Delete the old Agreement domain rather than retaining redundant aliases.
Preserve the English-v2 decision's existing one-member `Participle` axis and
its rule that no authored or public AST form tag is stored. If the compiler
needs a transient morphological feature, use an `InflectionalForm` inventory
such as plain, third-person-singular present, preterite, gerund-participle, and
past participle, with Finiteness represented separately when required. Do not
introduce a `Finite | Base | PastParticiple` pseudo-paradigm.

Append a dated superseding amendment to the English-v2 rewrite decision rather
than rewriting its history. Pin every currently supported *was/were* path: the
weak two-class stage may keep exceptional copula handling until explicit Person
and Number land, but it may not misclassify those forms to make the enum look
uniform. Acceptance shows the same Inflectional Form dimension can combine with
the applicable Concord Classes and that plain or participial forms do not
acquire spurious agreement values.
