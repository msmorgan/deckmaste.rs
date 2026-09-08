# Shared lexical model

`deckmaste_lexical_model` is the data-only vocabulary shared by independent
lexical analysis, generated constructions, and English v3. It describes
lexeme identities, lexical categories, inflectional forms, correlated feature
bundles, frame signatures, and the compact lexical values carried by grammar
leaves.

The crate deliberately has no tokenization, morphology, declaration loading,
catalog access, lookup index, source positions, provenance service, parser
state, or realization behavior. Those belong to the deep
`deckmaste_lexical` module. A lexical `Frame` is only an ordered signature of
grammatical slots and fixed markers; it is not a grammar production or an
admission algorithm.
