# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Personal self-learning repository. Each top-level folder (`NN-<topic>`) is an independent learning track — a course, tutorial, or deep-dive on a specific topic.

## Conventions

- Each learning track folder owns its **domain** conventions, split across up to three files with distinct jobs. `GUIDELINES.md` is the source of truth for the track's purpose, audience, file naming, and domain parameters (snippet languages, acronyms, trade-off pairs) — the skills locate it by walking up from the lesson's folder toward the repo root, so one shared `GUIDELINES.md` can cover several sub-tracks (see `lesson-craft`'s Execution Workflow for the exact lookup).
- A track's `STUDY-PLAN.md`, where one exists, is the per-lesson content spec `lesson-eval` grades against.
- A track's `README.md` (e.g. `04-oracle/README.md`) is about *method* — how to work the track's tooling — distinct from a `GUIDELINES.md`-role `README.md` in a track with no dedicated guidelines file. Read whichever of these exist before working in a `NN-*` folder. (`01-system-design` is a different, exercise-style genre and keeps its own structure.)
- The **writing craft** shared across tracks — document structure, depth bar, mental-model rules, diagrams, the One-Pass Test self-review — lives once in the `lesson-craft` skill (`.claude/skills/`). When writing or reviewing the `NN-*.md` learning notes, use that skill together with the folder's `GUIDELINES.md`.
- To **validate or improve a finished lesson**, use the `lesson-eval` skill: a *quiz* mode that generates a mock assessment from the topic and checks the lesson answers it (content gaps), and a *review* mode (default) that audits the prose against the authoring rubric (writing quality).
- To **actively drill yourself** on a finished lesson, use the `lesson-drill` skill: an interactive, one-question-at-a-time recall session that grades the *reader's* typed answers, not the lesson — distinct from `lesson-eval`, which audits the lesson itself.

## Guidelines

Behavioral guidelines for working in this repo. The craft rules live in the skills; these govern
*conduct* — what to touch, when to ask, and what "done" means.

**Tradeoff:** These bias toward caution over speed. For trivial fixes (a typo, a broken link),
use judgment.

### 1. Think Before Writing

**Don't assume. Don't hide confusion. Surface trade-offs.**

- State your assumptions explicitly. If uncertain, ask.
- If a track's parameter file doesn't exist at all, stop and ask (the skills' own halt rule — see
  `lesson-craft`'s Execution Workflow); never guess the audience, snippet languages, or scope. If
  the file exists but leaves a needed parameter ambiguous, ask too — that goes further than the
  skill's own rule, which is a mechanical lookup, not a completeness check (a *missing*
  `Format mode:` line is a documented silent default, not a case to ask about).
- If a request has multiple readings, present them; don't pick silently — e.g. "check this lesson"
  could mean run `lesson-eval` or just skim it by eye. Mode selection *within* `lesson-eval` follows
  the skill's own stated default; that's not itself an ask-first case.
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

- "Write/revise a lesson" → it passes the linter script's hard checks, the One-Pass Test
  (lesson-craft §8), **and an independent `lesson-eval` review pass** before you call it done — the
  self-review alone is a demonstrated blind spot, not a formality to skip.
- "Improve a lesson" → run `lesson-eval`, present the report, and **stop for approval** before
  editing — the skills' report-then-approve gate is the contract, not a formality.
- Volatile facts (versions, prices, model names, exam topics) are verified against a current
  source, never written from memory.
- For multi-step work, state a brief plan with a verify step per item.

---

**These guidelines are working if:** diffs touch only what was asked, scaffolds stay empty until
the user fills them, reports come before edits, and clarifying questions come before writing
rather than after rework.
