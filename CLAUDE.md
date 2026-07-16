# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Personal self-learning repository. Each top-level folder (`NN-<topic>`) is an independent learning track — a course, tutorial, or deep-dive on a specific topic.

## Conventions

- Each learning track folder owns its **domain** conventions. Before working in any `NN-*` folder, read its local `GUIDELINES.md` (or `README.md`) first — that file is the source of truth for the track's purpose, audience, file naming, and domain parameters (snippet languages, acronyms, example trade-offs).
- The **writing craft** shared across tracks — document structure, depth bar, mental-model rules, diagrams, the One-Pass Test self-review — lives once in the `lesson-craft` skill (`.claude/skills/`). When writing or reviewing the `NN-*.md` learning notes, use that skill together with the folder's `GUIDELINES.md`. (The exercise-style track `01-system-design` is a different genre and keeps its own structure.)
- To **validate or improve a finished lesson**, use the `lesson-eval` skill: a *coverage* mode that generates a mock assessment from the topic and checks the lesson answers it (content gaps), and a *craft* mode that audits the prose against the authoring rubric (writing quality).

## Guidelines

Behavioral guidelines for working in this repo. The craft rules live in the skills; these govern
*conduct* — what to touch, when to ask, and what "done" means.

**Tradeoff:** These bias toward caution over speed. For trivial fixes (a typo, a broken link),
use judgment.

### 1. Think Before Writing

**Don't assume. Don't hide confusion. Surface trade-offs.**

- State your assumptions explicitly. If uncertain, ask.
- If the track's `GUIDELINES.md` parameters are missing or ambiguous, stop and ask — never guess
  the audience, snippet languages, or scope (the skills' halt-and-ask rule).
- If a request has multiple readings ("improve this lesson" — coverage? craft? both?), present
  them; don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.

### 2. Structure Is Not Content

**Deliver exactly the artifact asked for — scaffold, plan, or lesson — never the next one
uninvited.**

- A scaffold request produces skeletons and TODO markers, not draft prose.
- A study-plan request produces specs, not the lessons that satisfy them.
- Depth is the skills' standard, not a length target: every added paragraph must teach a distinct
  mechanic. If it restates or pads, cut it.
- Don't add lessons, sections, or "helpful extras" beyond what was asked.

### 3. Surgical Edits

**Touch only the passages named. Preserve the author's voice.**

- Don't "improve" adjacent prose, headings, or formatting you weren't asked to touch. Match the
  existing voice and snippet languages, even where you'd write it differently.
- If you notice an unrelated flaw (stale fact, weak analogy, broken diagram), report it — don't
  fix it unasked.
- Do fix what *your* edit breaks: renumbered sections, stale `## Contents` links, dangling
  cross-references (the lesson-craft sweep exists for this).
- Quiz artifacts under `assessments/` are historical records — append (e.g. a "Resolved" note),
  never rewrite verdicts.

### 4. Goal-Driven Execution

**Define what "done" means in this repo's own terms. Verify before declaring it.**

- "Write/revise a lesson" → it passes the One-Pass Test (lesson-craft §8) before you call it done.
- "Improve a lesson" → run `lesson-eval`, present the report, and **stop for approval** before
  editing — the skills' report-then-approve gate is the contract, not a formality.
- Volatile facts (versions, prices, model names, exam topics) are verified against a current
  source, never written from memory.
- For multi-step work, state a brief plan with a verify step per item.

---

**These guidelines are working if:** diffs touch only what was asked, scaffolds stay empty until
the user fills them, reports come before edits, and clarifying questions come before writing
rather than after rework.
