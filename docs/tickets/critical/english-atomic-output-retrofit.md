---
needs: [english-construction-projection-retrofit]
---
**Retrofit the quantity, noun, determiner, and possession outputs.**

Give Q01, N01, and D01 one truthful representation apiece. Preserve quantity
alternatives and notation, noun identity and number/mass form, determiner
classes, possession, agreement, and checked construction; remove duplicate
private representation enums, duplicate public kind enums where opacity does
not require both, capitalized enum-lookalike shims, manual legacy serializers,
and declaration-by-declaration public projections with no production caller.

Route spelling through named construction roles rather than the old
`Quantity`, `NounInstance`, `Determiner`, and `Possessor` serialized wrappers.
Retain semantic, inversion, agreement, invalid-ingress, and corpus tests;
delete tests whose only assertion is the obsolete Rust/serde layout.

Standard constraints apply.
