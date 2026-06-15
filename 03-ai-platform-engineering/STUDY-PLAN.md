# AI for Platform Engineers — Study Plan

A curriculum for an experienced platform engineer (Kubernetes, cloud, CI/CD, IaC) who wants to evolve their craft for the AI era. It is **balanced** across two intertwined threads that recur in almost every lesson:

- **Augment** — use AI to change how *you* work: coding agents, the Model Context Protocol (MCP), and agentic automation of platform tasks.
- **Operate** — build and run the infrastructure AI workloads depend on: model serving, GPUs on Kubernetes, retrieval pipelines, evaluation, and governance.

Start with `00-overview.md`, then work through the lessons in order. Each lesson is a self-contained reference written to `GUIDELINES.md` — readable on its own, but assuming the foundations from earlier lessons.

---

## Phase 0 — Start Here

- **`00-overview.md` · The New Layer of the Stack** — Why AI is becoming a platform concern, not just an ML-team concern. Frames the platform engineer's dual mandate (augment + operate) and maps the whole track.

---

## Phase 1 — Foundations

The mental model everything else builds on. Skipping these makes the later infra lessons feel like cargo-culting.

- **`01` · LLM Fundamentals** — Tokens, context windows, embeddings, and inference. What a Large Language Model (LLM) actually computes, why output is probabilistic rather than deterministic, and what that means for anyone treating a model like an API.
- **`02` · Prompt & Context Engineering** — The context window as a scarce, billable resource. System prompts, structured/JSON output, grounding, and the techniques that move an LLM from "demo that works once" to "reliable component."

---

## Phase 2 — Augmenting How You Work

Using AI to compress and transform your own platform workflow.

- **`03` · AI Coding Agents & Agentic Workflows** — How an agent plans, calls tools, observes results, and loops. Using Claude Code / Copilot effectively on real platform tasks — and where they break down.
- **`04` · Model Context Protocol (MCP) & Tool Use** — How tool use works under the hood and how MCP standardises it. Exposing Kubernetes, cloud APIs, and internal services to agents; sketching an MCP server for ops, with the auth and blast-radius concerns that come with it.
- **`05` · Agentic Automation for Platform Ops** — Putting agents to work on IaC generation, runbook execution, incident triage, and CI/CD. Designing guardrails, approvals, and human-in-the-loop so autonomy stays bounded.

---

## Phase 3 — Building & Operating AI Infrastructure

The other half of the mandate: the new components a platform must now host, scale, and keep healthy.

- **`06` · Embeddings & Vector Databases** — Turning text into vectors and searching them by meaning. Vector indexes (HNSW, IVF), the recall-vs-latency trade-off, and when you genuinely need a vector store versus when Postgres will do.
- **`07` · Retrieval-Augmented Generation (RAG)** — Grounding an LLM on private data without retraining it. End-to-end architecture: ingestion, chunking, retrieval quality, re-ranking, and the failure modes that make RAG quietly return wrong answers.
- **`08` · Model Serving & Inference** — Serving models in production with vLLM and KServe. The KV-cache, continuous batching, quantization, and the Time To First Token (TTFT) vs. throughput trade-off — the inference-server equivalents of the latency/throughput tuning you already know.
- **`09` · GPUs on Kubernetes** — GPUs as a first-class, scarce, scheduled resource. Device plugins, the GPU operator, Multi-Instance GPU (MIG) and time-slicing, node pools, and autoscaling around hardware that costs an order of magnitude more than CPU.
- **`10` · LLMOps & Evals** — The delivery lifecycle for non-deterministic systems. Evaluation harnesses, observability and tracing, prompt/version management, drift detection, and why you cannot ship an LLM change on unit tests alone.

---

## Phase 4 — Cross-Cutting Concerns

Threads that span every layer above — and the capstone that ties them together.

- **`11` · AI Security, Cost & Governance** — Prompt injection, data leakage, and the new attack surface agents introduce. Token-cost control, model governance, RBAC for model access, and the build-vs-buy decision for self-hosting versus managed APIs.
- **`12` · Building an Internal AI Platform** — Capstone reference architecture. An LLM gateway, model routing, self-service golden paths, and a clear ownership model — assembling everything from lessons 01–11 into a platform other teams can safely build on.

---

## Closing

- **`13` · Closing Assessment** — A candid read of where the track leaves you: what it covers well, the topic gaps that remain, the role-fit picture, and — most importantly — the hands-on projects to build that convert this conceptual foundation into hireable, demonstrable competence. Read it after `12`.

---

## How to Use This Plan

The phases are ordered, but the two threads interleave deliberately — by the time you reach the infrastructure lessons (Phase 3), you will have already used the tools (Phase 2) you are now learning to operate. If your immediate need is one-sided (e.g. you must stand up model serving next sprint), jump to that lesson, but read `00`–`02` first; every later lesson assumes the foundations.
