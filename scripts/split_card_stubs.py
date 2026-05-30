#!/usr/bin/env python3
"""Split the generated stub stream into a disposable gen store + the real card view.

Layout:
  ron/cards-gen/<oid>.ron       — generated stubs. Gitignored, regenerated wholesale from
                                  MTGJSON, overwritten every run. Disposable.
  ron/cards/<oid>.ron           — the real card view the engine reads. Each entry is EITHER a
                                  symlink -> ../cards-gen/<oid>.ron (not yet encoded) OR a
                                  real hand-authored file. You "encode for real" by replacing
                                  a symlink with a real file in place.
  ron/cards-by-name/<name>.ron  — browse/grep convenience -> ../cards/<oid>.ron. Gitignored,
                                  regenerated. '/' in a name becomes '-' ("Fire -- Ice").

Re-running is safe in both directions and survives MTGJSON changes:
  * gen stubs are always rewritten (derived data);
  * a real file in ron/cards/ is never overwritten or deleted — only reported if its card
    leaves the stream;
  * an old in-place Todo *file* (from the previous single-folder layout) is migrated to a
    symlink;
  * a symlink whose oid left the stream (narrowed filter, dropped card) is pruned.

Usage: python3 scripts/split_card_stubs.py build/cards_todo.ron
"""
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GEN = ROOT / "ron" / "cards-gen"
CARDS = ROOT / "ron" / "cards"
BY_NAME = ROOT / "ron" / "cards-by-name"
MARK = "// ===>\t"


def fs_name(name: str) -> str:
    # '/' is the only character illegal in a POSIX filename; map it (so '//' -> '--').
    return name.replace("/", "-").strip()


def is_stub_text(text: str) -> bool:
    return any(ln.startswith("Todo(") for ln in text.splitlines())


def main() -> None:
    if len(sys.argv) != 2:
        sys.exit("usage: split_card_stubs.py <generated-stub-file>")
    src = Path(sys.argv[1])
    GEN.mkdir(parents=True, exist_ok=True)
    CARDS.mkdir(parents=True, exist_ok=True)

    # 1. Write every gen stub (overwrite — gen is derived) and remember each card's name.
    names: dict[str, str] = {}
    cur_oid = cur_name = None
    buf: list[str] = []

    def flush() -> None:
        if cur_oid is None:
            return
        (GEN / f"{cur_oid}.ron").write_text("".join(buf).strip() + "\n", encoding="utf-8")
        names[cur_oid] = cur_name

    with src.open(encoding="utf-8") as fh:
        for line in fh:
            if line.startswith(MARK):
                flush()
                _, cur_oid, cur_name = line.rstrip("\n").split("\t", 2)
                buf = []
            else:
                buf.append(line)
        flush()
    seen = set(names)

    # 2. Ensure a real-view entry per gen oid: symlink unless a real file already exists.
    migrated = linked_new = 0
    for oid in seen:
        entry = CARDS / f"{oid}.ron"
        if entry.is_symlink():
            continue  # existing scaffolding symlink — leave it (it already points into gen)
        if entry.exists():  # a regular file
            if is_stub_text(entry.read_text(encoding="utf-8")):
                entry.unlink()  # old in-place stub from the prior layout -> migrate to symlink
                migrated += 1
            else:
                continue  # a real, hand-encoded card -> keep
        entry.symlink_to(os.path.relpath(GEN / f"{oid}.ron", CARDS))
        linked_new += 1

    # 3. Prune real-view symlinks whose oid left the stream; keep/report real files.
    pruned = 0
    for f in CARDS.glob("*.ron"):
        if f.stem in seen:
            continue
        if f.is_symlink():
            f.unlink()
            pruned += 1
        else:
            print(f"keep: encoded card no longer in stream: {f.name}", file=sys.stderr)

    # 4. Rebuild the by-name view over whatever ron/cards/ now holds.
    if BY_NAME.exists():
        for p in BY_NAME.iterdir():
            if p.is_symlink():
                p.unlink()
    else:
        BY_NAME.mkdir(parents=True)
    linked = collisions = 0
    taken: dict[str, str] = {}
    for f in sorted(CARDS.glob("*.ron")):
        name = names.get(f.stem)
        if name is None:
            continue  # an encoded card not in the current stream — no known display name
        slug = fs_name(name)
        if slug in taken and taken[slug] != f.stem:
            slug = f"{slug} ({f.stem[:8]})"
            collisions += 1
        taken[slug] = f.stem
        link = BY_NAME / f"{slug}.ron"
        if link.is_symlink() or link.exists():
            link.unlink()
        os.symlink(os.path.relpath(f, BY_NAME), link)
        linked += 1

    real = sum(1 for f in CARDS.glob("*.ron") if not f.is_symlink())
    stubs = sum(1 for f in CARDS.glob("*.ron") if f.is_symlink())
    print(
        f"gen: {len(seen)} stubs -> {GEN.name}/\n"
        f"cards/: {real} real, {stubs} symlinked stubs "
        f"(+{linked_new} new symlinks, {migrated} migrated, {pruned} pruned)\n"
        f"by-name: {linked} symlinks"
        + (f" ({collisions} collisions disambiguated)" if collisions else "")
    )


if __name__ == "__main__":
    main()
