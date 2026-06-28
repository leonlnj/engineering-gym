# 03-ai-platform-engineering Guidelines

> **Authoring & review craft lives in the `technical-lesson-authoring` skill** (`.claude/skills/`). It holds the document structure, the depth bar, the writing rules, diagrams, and the One-Pass Test self-review — invoke it before writing or reviewing a lesson. This file records only what is specific to *this* track.

## Purpose

Self-study notes on **AI for platform engineers** — equipping someone already fluent in platform tooling, Kubernetes, and cloud to evolve their craft for the AI era. Each file is a self-contained, standalone reference: technically detailed enough to return to later, and educational enough to explain *why*, not just *what*. The goal is not a summary of blog posts; a good lesson lets a platform engineer understand an AI concept deeply — including the reasoning behind each design decision — and connect it back to how they build and operate platforms.

The track has two intertwined threads, and most lessons touch both:
- **Augment** — using AI to evolve how you work (coding agents, MCP, agentic automation of platform tasks).
- **Operate** — building and running the infrastructure AI workloads need (model serving, GPUs on Kubernetes, RAG, evals, governance).

Start at `00-overview.md`. The full curriculum lives in `STUDY-PLAN.md`.

## Audience

Assume the reader knows Kubernetes, cloud, and CI/CD well, but is new to AI internals — lean on that shared platform vocabulary to explain the unfamiliar.

## File Naming

`NN-<topic-slug>.md` — sequential number, lowercase, hyphenated words (e.g. `08-model-serving.md`). `00-overview.md` is the entry point; numbered lessons `01`–`12` follow the order in `STUDY-PLAN.md`.

## Track parameters

These are the inputs the skill defers to this track for. The depth bar itself (snippet/diagram counts, walkthroughs, quantify) is the skill's standard — not repeated here.

- **Snippet languages**: **YAML** for Kubernetes manifests and CI config, **Python** for LLM/SDK calls and RAG pipelines, **JSON** for MCP/tool-definition schemas and API payloads, and CLI commands (`kubectl`, `helm`, cloud CLIs) for operational steps. Reach for pseudocode only for an algorithm no real tool expresses cleanly.
- **Acronyms to expand on first use**: **LLM** (Large Language Model), **MCP** (Model Context Protocol), **RAG** (Retrieval-Augmented Generation), **GPU** (Graphics Processing Unit), **vLLM**, **KServe**, **MIG** (Multi-Instance GPU), **TTFT** (Time To First Token), **KV-cache** (Key-Value cache), **RBAC** (Role-Based Access Control), **SLO** (Service Level Objective), **IaC** (Infrastructure as Code).
- **Domain trade-off pairs** to watch for and name explicitly: capability vs. cost (a larger model is smarter but costs more per token and per GPU-hour); latency vs. throughput (small batches respond fast, large batches serve more per GPU); velocity vs. governance (self-service AI ships faster but widens the blast radius for data leakage and prompt injection); control vs. operational burden (self-hosting gives data control but means owning GPUs, scaling, upgrades); determinism vs. flexibility (a fixed pipeline is predictable, an agent that picks its own steps is adaptable but harder to bound).
- **Example "Practical Limits" bullet** in the required style: *"**Probabilistic vs. deterministic**: an LLM call is not a pure function — the same prompt can return different output, so you cannot cache or assert on it the way you would a REST API response."*
