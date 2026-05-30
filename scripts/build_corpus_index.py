#!/usr/bin/env python3
"""Generate corpus/manifest.json and corpus/INDEX.md from the per-card catalog files.

The corpus/<NN-category>/<slug>.md files are the source of truth (one annotated
catalog entry per edge-case card, each with a sibling <slug>.ron draft). This script
scans them, parses out each card's identity / weirdness / cited rules, and writes:

  - corpus/manifest.json : machine-readable roster (one object per card)
  - corpus/INDEX.md      : human-readable index grouped by category

Optional first argument: a path to the workflow's result JSON (with
`.result.corpus.entries`), used only to enrich `weirdness` / `crRules` when the
markdown is terse. The script is fully reproducible from the .md files alone.

Usage:
  python3 scripts/build_corpus_index.py [optional_entries.json]
"""
import json
import os
import re
import sys
from glob import glob

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CORPUS = os.path.join(ROOT, "corpus")

CATEGORY_TITLES = {
    "01-alternative-costs": "Alternative & Additional Costs",
    "02-unusual-timing-permission": "Unusual Timing & Casting Permission",
    "03-other-zone-interaction": "Other-Zone Interaction (graveyard / exile / library)",
    "04-continuous-effects-layers": "Continuous Effects & the Layer System",
    "05-characteristic-defining": "Characteristic-Defining Abilities & “*” P/T",
    "06-copy-effects": "Copy Effects & Copiable Values",
    "07-replacement-prevention": "Replacement & Prevention Effects",
    "08-state-based-actions": "State-Based Actions",
    "09-triggered-abilities": "Triggered-Ability Oddities",
    "10-static-restrictions": "Static Restrictions, Permissions & Cost Alteration",
    "11-tokens-counters-stickers": "Tokens, Counters, Energy, Experience & Poison",
    "12-multipart-layouts": "Multi-Part Card Layouts",
    "13-attachments": "Attachments (Equipment / Aura / Fortification / mutate)",
    "14-combat": "Combat Weirdness",
    "15-mana-and-color": "Mana & Color Oddities",
    "16-targeting-and-choices": "Targeting, Protection & “Choose”",
    "17-special-actions-designations": "Special Actions & Game Designations",
    "18-game-rule-benders": "Game-Rule Benders",
}


def load_enrichment(path):
    """(category, slug) -> {'weirdness':..., 'crRules':[...], 'set':..., 'number':...}"""
    out = {}
    try:
        with open(path, encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, ValueError):
        return out
    entries = (((data.get("result") or {}).get("corpus") or {}).get("entries")) or []
    for e in entries:
        f = e.get("file", "")
        slug = os.path.basename(f)
        cat = os.path.basename(os.path.dirname(f))
        if slug and cat:
            out[(cat, slug)] = e
    return out


def parse_md(path):
    with open(path, encoding="utf-8") as fh:
        text = fh.read()

    name = None
    m = re.search(r"^#\s+(.+?)\s*$", text, re.M)
    if m:
        name = m.group(1).strip()

    setc = num = cost = typ = ""
    m = re.search(r"Set/N[o№]\D*?([A-Za-z0-9]+)\s*#\s*([^\s*]+)", text)
    if m:
        setc, num = m.group(1), m.group(2)
    m = re.search(r"Cost:\*\*\s*(.*?)\s*\*\*", text)
    if m:
        cost = m.group(1).strip().strip("—-").strip()
    m = re.search(r"Type:\*\*\s*([^\n*]+)", text)
    if m:
        typ = m.group(1).strip()

    weird = ""
    m = re.search(r"##\s*Why[^\n]*\n+(.+?)(?:\n##|\Z)", text, re.S | re.I)
    if m:
        weird = " ".join(m.group(1).split())

    block = text
    m = re.search(r"##\s*Governing rules[^\n]*\n(.+?)(?:\n##|\Z)", text, re.S | re.I)
    if m:
        block = m.group(1)
    crs = []
    for r in re.findall(r"CR\s*([0-9]{3}(?:\.[0-9]+[a-z]?)*)", block):
        if r not in crs:
            crs.append(r)

    return dict(name=name, set=setc, number=num, cost=cost, type=typ, weirdness=weird, crRules=crs)


def first_sentence(s, limit=220):
    s = s.strip()
    m = re.search(r"^(.+?[.!?])\s", s)
    out = m.group(1) if m else s
    if len(out) > limit:
        out = out[:limit].rstrip() + "…"
    return out


def norm_crs(items):
    """Reduce citation strings (which may be bare numbers or full sentences) to a
    deduped list of rule tokens, e.g. ['116.2h', '702.143a']."""
    seen = []
    for tok in re.findall(r"\b\d{3}(?:\.\d+[a-z]?)*", " ".join(items or [])):
        if tok not in seen:
            seen.append(tok)
    return seen


def main():
    enrich = load_enrichment(sys.argv[1]) if len(sys.argv) > 1 else {}

    records = []
    for md in sorted(glob(os.path.join(CORPUS, "*", "*.md"))):
        cat = os.path.basename(os.path.dirname(md))
        slug = os.path.splitext(os.path.basename(md))[0]
        if cat not in CATEGORY_TITLES:
            continue
        rec = parse_md(md)
        e = enrich.get((cat, slug), {})
        # Prefer clean structured enrichment for prose fields; markdown for identity.
        if e.get("weirdness"):
            rec["weirdness"] = e["weirdness"]
        if e.get("crRules"):
            rec["crRules"] = e["crRules"]
        if not rec["set"] and e.get("set"):
            rec["set"] = e["set"]
        if not rec["number"] and e.get("number"):
            rec["number"] = e["number"]
        if not rec["name"]:
            rec["name"] = e.get("name") or slug.replace("-", " ").title()
        rec["crRules"] = norm_crs(rec["crRules"])

        ron = md[:-3] + ".ron"
        rec.update(
            category=cat,
            slug=slug,
            md=os.path.relpath(md, ROOT),
            ron=os.path.relpath(ron, ROOT) if os.path.exists(ron) else None,
        )
        records.append(rec)

    records.sort(key=lambda r: (r["category"], r["name"].lower()))

    # manifest.json
    manifest = {
        "description": "Edge-case corpus: real paper, non-funny MTG cards exhibiting "
        "abnormal rules interactions. Source of truth = corpus/<category>/<slug>.md (+ .ron).",
        "selection": {
            "paper": "availability LIKE '%paper%'",
            "non_funny": "isFunny IS DISTINCT FROM true",
        },
        "count": len(records),
        "categories": {
            cat: sum(1 for r in records if r["category"] == cat)
            for cat in CATEGORY_TITLES
        },
        "cards": records,
    }
    with open(os.path.join(CORPUS, "manifest.json"), "w", encoding="utf-8") as fh:
        json.dump(manifest, fh, indent=2, ensure_ascii=False)
        fh.write("\n")

    # INDEX.md
    lines = []
    lines.append("# Edge-Case Corpus — Index")
    lines.append("")
    lines.append(
        f"**{len(records)} cards** across {len(CATEGORY_TITLES)} categories. Every card is a "
        "real, paper-available, black-bordered (`isFunny IS DISTINCT FROM true`) *Magic* card "
        "chosen because it does something the rules engine must handle that a vanilla card does "
        "not. See [`README.md`](README.md) for selection criteria and the dual `.md` + `.ron` "
        "storage; see [`manifest.json`](manifest.json) for the machine-readable roster."
    )
    lines.append("")
    lines.append("Generated by `scripts/build_corpus_index.py` — do not edit by hand.")
    lines.append("")
    for cat in CATEGORY_TITLES:
        group = [r for r in records if r["category"] == cat]
        if not group:
            continue
        lines.append(f"## {cat[:2]} — {CATEGORY_TITLES[cat]} ({len(group)})")
        lines.append("")
        for r in group:
            ident = r["name"]
            if r["set"] and r["number"]:
                ident += f" ({r['set']} #{r['number']})"
            elif r["set"]:
                ident += f" ({r['set']})"
            bits = [f"- **{ident}**"]
            w = first_sentence(r["weirdness"]) if r["weirdness"] else ""
            if w:
                bits.append(f" — {w}")
            if r["crRules"]:
                bits.append(f"  · _CR {', '.join(r['crRules'][:6])}_")
            lines.append("".join(bits))
        lines.append("")

    with open(os.path.join(CORPUS, "INDEX.md"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines))

    print(f"Wrote manifest.json and INDEX.md: {len(records)} cards")
    for cat in CATEGORY_TITLES:
        n = sum(1 for r in records if r["category"] == cat)
        miss_id = sum(1 for r in records if r["category"] == cat and not (r["set"] and r["number"]))
        print(f"  {cat:38s} {n:3d}" + (f"  ({miss_id} missing set/№)" if miss_id else ""))


if __name__ == "__main__":
    main()
