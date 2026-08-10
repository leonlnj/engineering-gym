#!/usr/bin/env python3
"""
lint_lesson.py — mechanical checks for NN-topic.md lesson files.

Two tiers:
  HARD checks (exit 1 if any hit): citation-in-prose, bare section refs, broken TOC links.
    These are unambiguous rule violations per lesson-craft SKILL.md §3.13 and §7 — fix all of them.
  ADVISORY findings (exit 0, printed only): sentence-chain candidates. This is a generous
    heuristic, not a rule checker. A sentence is flagged if it carries 2+ "second-layer" signals:
    an em-dash aside, a clause-introducing connective (rather than / which / so / because / since /
    while / though / unless / and / but joining independent clauses), or a participial clause
    (", V-ing"). One signal is normal prose. Two is a candidate for splitting per §3.13 — not an
    automatic verdict.

    Two false-positive rates are on record, both from full-track sweeps on this repo:
    connective-only signals ran ~68% false-positive (78 flagged, 25 genuine); the participial
    signal added later ran ~33% (6 new candidates, 4 genuine). The participial signal exists
    because the connective-only version was tested against the actual passage that motivated this
    script — a block-volume/Availability-Domain callout in 03-managed-kubernetes.md — and missed
    it: that sentence's density came from a participial clause ("...delaying volume creation until
    the scheduler has already picked a node, rather than..."), not a second connective or em-dash.
    Read every hit; most are fine, but don't skip reading them — this tool has a demonstrated
    history of under-catching, not just over-catching.

Usage: python3 lint_lesson.py <file.md> [<file2.md> ...]
"""
import re
import sys

CITATION_RE = re.compile(r'\((?:as of|As of)\s+[A-Za-z]+ 20\d\d', re.IGNORECASE)
SECTION_REF_RE = re.compile(r'§\s?\d|Section\s+\d')
LIMITS_HEADING_RE = re.compile(r'^##\s+\d+\.\s*(Limits and Sources|Practical Limits and Trade-offs)', re.IGNORECASE)
TOC_ENTRY_RE = re.compile(r'^\d+\.\s+\[([^\]]+)\]\(#([a-z0-9-]+)\)')
HEADING_RE = re.compile(r'^##\s+(\d+)\.\s+(.+)$')

CONNECTIVES = [
    r'\brather than\b', r',\s*which\b', r',\s*so that\b', r',\s*because\b',
    r',\s*since\b', r',\s*while\b', r',\s*though\b', r',\s*unless\b',
    r',\s*so\b', r',\s*and\b', r',\s*but\b',
]

# Lowercase-guarded: a bare `,\s+\w+ing\b` also matches capitalized proper nouns that happen to
# end in "-ing" (e.g. ", Streaming" as in "Functions, Streaming, or Notifications") and misreads
# a service name as a participial verb. Requiring a lowercase start excludes those while still
# catching real mid-sentence participles (delaying, confirming, picking, showing, delivering).
PARTICIPIAL_RE = re.compile(r',\s+[a-z]\w*ing\b')

def slugify(heading_text):
    s = heading_text.lower()
    s = re.sub(r'[^a-z0-9\s-]', '', s)
    s = re.sub(r'\s+', '-', s.strip())
    return s

def chain_signals(sentence):
    asides = sentence.count('—') // 2 if sentence.count('—') % 2 == 0 else sentence.count('—')
    conn = sum(len(re.findall(p, sentence, flags=re.IGNORECASE)) for p in CONNECTIVES)
    participial = len(PARTICIPIAL_RE.findall(sentence))
    return asides, conn, participial

def lint(path):
    text = open(path, encoding='utf-8').read()
    lines = text.split('\n')
    hard_failures = []
    advisory = []

    # Find where the Limits/Practical-Limits section starts, to exempt it from the citation check
    limits_start = None
    for i, line in enumerate(lines):
        if LIMITS_HEADING_RE.match(line):
            limits_start = i
            break

    headings = {}  # slug -> line number, for TOC validation
    for i, line in enumerate(lines, 1):
        m = HEADING_RE.match(line)
        if m:
            headings[slugify(f"{m.group(1)}. {m.group(2)}")] = i

    for i, line in enumerate(lines, 1):
        in_limits = limits_start is not None and i > limits_start

        if not in_limits and CITATION_RE.search(line):
            hard_failures.append(f"{path}:{i}: inline citation in prose (belongs in Limits table) — {line.strip()[:120]}")

        if SECTION_REF_RE.search(line) and '#' not in line.split('§')[0][-3:]:
            # crude guard against matching markdown anchors like #2-something; real hits are prose refs
            if not re.search(r'#\d', line):
                hard_failures.append(f"{path}:{i}: bare section reference (use a name-based ref instead) — {line.strip()[:120]}")

        toc = TOC_ENTRY_RE.match(line.strip())
        if toc:
            slug = toc.group(2)
            if slug not in headings:
                hard_failures.append(f"{path}:{i}: TOC link '#{slug}' does not resolve to any heading")

        if not line or line[0] in '|>*#`-' or re.match(r'^\d+\.', line):
            if not line.startswith('>'):
                continue
        for s in re.split(r'(?<=[.!?])\s+(?=[A-Z*`("])', line):
            if len(s) < 40:
                continue
            asides, conn, participial = chain_signals(s)
            total = asides + conn + participial
            if total >= 2:
                advisory.append((total, i, s.strip()[:180]))

    print(f"\n=== {path} ===")
    if hard_failures:
        print(f"HARD FAILURES ({len(hard_failures)}):")
        for f in hard_failures:
            print(f"  {f}")
    else:
        print("Hard checks: clean.")

    if advisory:
        advisory.sort(reverse=True)
        print(f"\nAdvisory: {len(advisory)} chain-candidate sentence(s) — read each, most will be fine:")
        for total, i, s in advisory:
            print(f"  [{total}] line {i}: {s}")

    return len(hard_failures)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: lint_lesson.py <file.md> [...]")
        sys.exit(2)
    total_failures = sum(lint(p) for p in sys.argv[1:])
    sys.exit(1 if total_failures else 0)
