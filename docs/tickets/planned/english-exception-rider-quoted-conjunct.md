---
needs: [english-test-structural-assertions]
---
**Keep a final quoted-ability conjunct inside an Oxford exception rider.** The
typed-assertion conversion exposed that `except A, B, and it has "…"` currently
lowers as a two-member `ExceptionRider` followed by an outer coordinated
clause. The old debug-string test passed because it independently found an
exception node, an exception conjunct, and a quoted ability anywhere in the
tree.

Make the final `and <clause>` member part of the existing exception list when
the same top-level `except` rider owns the preceding comma-separated members.
Keep ordinary post-rider clause coordination outside it; license the boundary
from punctuation and constituent shape, never spelling or card identity. Add
typed positive and over-fire assertions, preserve byte-exact rendering, and
report the recovery delta. Standard constraints apply.
