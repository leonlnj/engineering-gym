# Engineering Gym AI Agent Instructions (Root)

This file is a pointer for GitHub Copilot. The repo's actual conventions live elsewhere and are not
duplicated here, so they can't drift out of sync:

- **`CLAUDE.md`** (repo root) — repo-wide conventions and working guidelines (what to touch, when to
  ask, what "done" means).
- **A track's `GUIDELINES.md`** (or the `README.md` that serves that role) — that track's domain
  parameters: purpose, audience, file naming, snippet languages, acronyms, trade-off pairs.
- **`.claude/skills/lesson-craft` and `.claude/skills/lesson-eval`** — the shared writing craft
  (document structure, depth bar, the One-Pass Test) and the audit/grade/improve loop for a
  finished lesson.
- **`.claude/skills/lesson-drill`** — an interactive, one-question-at-a-time active-recall session
  for studying a finished lesson; grades the reader's answers, not the lesson itself.

Read `CLAUDE.md` first; it also explains how track-local files are found.

## Authoring Guidance for New Folders

When a new top-level `NN-<topic>` folder is created, add a concise local `GUIDELINES.md` that
includes:
- Scope of the folder
- Preferred document/code structure
- Setup/Test/Cleanup expectations (or explicit "not required")
- Tooling/runtime assumptions
- Any diagram or explanation style preferences

Keep local guidance practical and not overly specific so it stays reusable.
