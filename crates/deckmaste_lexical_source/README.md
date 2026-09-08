# Lexical source adaptation

`deckmaste_lexical_source` assembles the current authored vocabulary into
normalized [`deckmaste_lexical::Lexeme`] declarations. Its public interface is
independent of the corpus-reporting CLI and of any English parser.

The implementation currently contains an explicitly transitional adapter for
the declarations being salvaged from English v2. That adapter is an input
quarry, not an interface v3 consumers must implement or preserve. The planned
lexical-inventory work replaces its internal readers as declarations become
v3-owned.

Source trees are discovered as inventories. Declaration identity comes from
normalized declaration content; callers and tests do not name an individual
macro file.
