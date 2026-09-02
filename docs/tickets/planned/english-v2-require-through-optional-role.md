---
needs: [english-v2-pp-construction]
---
`require` cannot read a feature through an `opt Category` role: the generated
invariant receives `&Option<T>` and code generation has no unwrap for it.
Teach the invariant emitter to map over an optional category role (absent
satisfies the predicate), then dissolve `SourcePhrase` and `ControlPhrase`
back into the general prepositional phrase under a preposition-class guard —
they exist only because a codec tail cannot carry an optional literal and the
slots could not otherwise be fenced. Standard constraints apply.
