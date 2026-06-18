---
needs: []
---
The current `LkiSnapshot` implementation (UD-9) uses a hardcoded, field-enumerated struct. While efficient, this is "lax" and fragile: every time a new characteristic or property needs to be tracked across zone changes for triggers or damage attribution, the struct and all its population sites must be updated.

Proposed Investigation:
Evaluate moving to a more generic property mapping or a more comprehensive "Object Snapshot" that captures all derived characteristics at the time of the event, ensuring that triggers always have access to the full "last known information" without manual struct maintenance.

Caveat: Must maintain GC-freedom and avoid retaining live object references.
