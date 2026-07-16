# 02-redis-internal Guidelines

> **Authoring & review craft lives in the `lesson-craft` skill** (`.claude/skills/`). It holds the document structure, the depth bar, the writing rules, diagrams, and the One-Pass Test self-review — invoke it before writing or reviewing a lesson. This file records only what is specific to *this* track.

## Purpose

Lesson-by-lesson notes from a Redis internals video series. Each file is a self-contained, standalone recap — technically detailed enough to serve as a future reference, and educational enough to explain *why*, not just *what*. The goal is not a transcript: a good lesson lets someone who missed the video understand the concept deeply, including the reasoning behind every design decision.

## File Naming

`NN-<topic-slug>.md` — sequential number, lowercase, hyphenated words. Example: `04-memory-allocator.md`.

## Track parameters

These are the inputs the skill defers to this track for. The depth bar itself (snippet/diagram counts, walkthroughs, quantify) is the skill's standard — not repeated here.

- **Snippet languages**: C-style pseudocode for low-level internals (system calls, data structures, event loops); real syntax (Python, Go, Redis CLI commands) when showing how Redis is used from an application.
- **Acronyms to expand on first use**: RESP (Redis Serialization Protocol), RDB, AOF, TTL, FD (file descriptor), HA (high availability).
- **Domain trade-off pairs** to watch for and name explicitly: speed vs. durability (in-memory vs. persisted to disk); simplicity vs. throughput (single thread vs. multi-thread); memory vs. precision (approximate structures like HyperLogLog); availability vs. consistency (async vs. sync replication).
- **Example "Practical Limits" bullet** in the required style: *"**Speed vs. durability**: in-memory storage makes Redis fast but volatile — a crash loses any writes not yet flushed to disk."*

> Note: the earlier lessons in this track predate the skill's depth bar and may not all meet it yet (e.g. fewer snippets, no worked walkthrough). That is expected; the bar applies to new and revised lessons. `03-event-loops.md` is the exemplar to model depth on.
