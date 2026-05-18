# Engineering Gym AI Agent Instructions (Root)

This file defines **repo-wide defaults** and a **folder context discovery policy**.

## Purpose
- Keep root instructions generic and reusable.
- Let each top-level folder define its own detailed working rules.

## Folder Context Discovery Policy
When assisting the user, resolve context in this order:

1. **Active file path** (editor context).
2. **User-stated folder or file** in the prompt.
3. **Nearest folder-local instruction file** from the working path upward.
4. Root defaults from this file.

## Folder-local Instruction Files
For any top-level learning track folder, look for one of these files (in order):
- `GUIDELINES.md`
- `README.md`
- `.copilot-instructions.md`

If found, treat it as the source of truth for that folder.

## Conflict Resolution
- Folder-local instruction overrides root defaults for files in that folder.
- If multiple local docs exist, use the first match in the lookup order above.
- If no folder-local file exists, apply root defaults only.

## Authoring Guidance for New Folders
When a new top-level folder is created, add a concise local guideline file that includes:
- Scope of the folder
- Preferred document/code structure
- Setup/Test/Cleanup expectations (or explicit “not required”)
- Tooling/runtime assumptions
- Any diagram or explanation style preferences

Keep local guidance practical and not overly specific so it stays reusable.

## Root Defaults (Apply Unless Overridden)
- Be educational: explain why, not only how.
- Include trade-offs for key decisions.
- Prefer concise, purposeful examples and diagrams.
- Keep commands copy-paste ready and grouped by execution phase when relevant.

