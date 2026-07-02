---
needs: []
---
DONE — mirroring the Idris `Card` (the concept-canonical shape): the layouts
are ONE parameterized constructor, not one variant each. `Card::ModalDfc(a,
b)` became `Card::TwoFaced { layout: FaceLayout, front, back }` with
`FaceLayout = Transforming | ModalDfc | Split | Adventure | Flip`
([CR#712.2,712.3,709.1,715.1,710.1]; two full faces per [CR#712.8]).
`TodoCard` mirrors it byte-identically; extraction still reads `normal` /
`modal_dfc` only (widening layout coverage is
[[pipeline-layout-extraction]]); per-layout engine behavior stays with
[[engine-transform]] etc. Single-faced census-§4 "layouts" (saga, class,
leveler, case) are deliberately NOT card shapes — their mechanics ride
subtypes/abilities on `Normal` cards, as the Idris `subtypeConfers` already
models for Saga ([CR#714.3c]). Meld is likewise excluded (a meld back face is
shared across two cards — its own future shape).

---

Original framing: `Card` variants beyond `Normal`/`ModalDfc`: transform,
saga, adventure, split, flip, etc. (see the census §4 layout table). Grammar
prerequisite for most layout-specific engine and extraction work.
