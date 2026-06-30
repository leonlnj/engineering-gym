# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Personal self-learning repository. Each top-level folder (`NN-<topic>`) is an independent learning track — a course, tutorial, or deep-dive on a specific topic.

## Conventions

- Each learning track folder owns its **domain** conventions. Before working in any `NN-*` folder, read its local `GUIDELINES.md` (or `README.md`) first — that file is the source of truth for the track's purpose, audience, file naming, and domain parameters (snippet languages, acronyms, example trade-offs).
- The **writing craft** shared across tracks — document structure, depth bar, mental-model rules, diagrams, the One-Pass Test self-review — lives once in the `technical-lesson-authoring` skill (`.claude/skills/`). When writing or reviewing the `NN-*.md` learning notes, use that skill together with the folder's `GUIDELINES.md`. (The exercise-style track `01-system-design` is a different genre and keeps its own structure.)
- To **validate or improve a finished lesson**, use the `lesson-validation` skill: a *coverage* mode that generates a mock assessment from the topic and checks the lesson answers it (content gaps), and a *craft* mode that audits the prose against the authoring rubric (writing quality).
