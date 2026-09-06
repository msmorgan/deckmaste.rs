# Context Map

Deckmaste has two bounded contexts. Each owns a vocabulary; a spelling shared
between them has the meaning assigned by the context in which it is used.

## Contexts

- [Game Model](./docs/contexts/game-model/CONTEXT.md): models Magic game
  concepts and the project-defined semantic vocabulary needed to express them
- [Oracle English](./docs/contexts/oracle-english/CONTEXT.md): models the
  grammar and realization of English Oracle text

## Relationships

- **Comprehensive Rules → Game Model**: the CR is the ultimate authority for
  Magic rules terms; project terms may extend that vocabulary without changing
  CR meanings.
- **Oracle English and Game Model**: Oracle English describes the linguistic
  structure of rules text; Game Model describes game concepts. The English
  NLP workbench and Semantics workbench are separate Lake projects with no
  interaction or dependency between them.
- **Shared spellings**: terms such as Object and Predicate may have different
  meanings in the two contexts. The owning glossary controls each meaning; a
  term borrowed by the other context retains its owner's meaning.
