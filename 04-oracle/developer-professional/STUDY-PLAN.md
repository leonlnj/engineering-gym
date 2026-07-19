<!--
  Each module's bullet list below is the SPEC that `lesson-eval` coverage mode grades the lesson
  against — write it BEFORE writing the lesson, so the bar is set outside-in.
  The subtopics below are derived from Oracle's official course Teaching Objectives and module
  outline (pasted in by the author). Extend each list with course-specific bullets while studying
  that module — see "How to Use This Plan".
-->

# OCI Developer Professional — Study Plan

The Oracle Cloud Infrastructure (OCI) Developer Professional course is designed for cloud developers and architects, offering in-depth knowledge and hands-on skills to harness the power of OCI Developer services for cutting-edge cloud-native application development and seamless deployment.

Track conventions (audience, snippet languages, trade-off pairs) live in [`../GUIDELINES.md`](../GUIDELINES.md).

## Teaching Objectives

- Understand cloud-native fundamentals and microservices architecture.
- Deploy and manage containerized applications with OCI Kubernetes Engine (OKE) and OCI Container Registry service.
- Implement serverless functions for scalable and cost-effective application components and manage APIs effectively using OCI API Gateway.
- Leverage serverless offerings such as streaming, queues, and events for real-time data processing, reliable message queuing, and event-driven architectures.
- Build event-streaming solutions using OCI Streaming with Apache Kafka for scalable, Kafka-compatible messaging and real-time data processing.
- Enhance and secure cloud-native applications using OCI Vault, image scanning, and best practices for testing.
- Gain insights into application performance using OCI's observability services.

---

## Modules

One lesson per Oracle course module, numbered to match. Each module's bullet list is the coverage
spec for that lesson — add bullets as you study.

- **`01` · Cloud Native Fundamentals**
  - Key Pillars of Cloud Native Development (Microservices, Containers, DevOps, CI/CD, Service Mesh)
  - Microservice Architecture
  - Design methodology of microservice, 12 factor methodology
  - DevOps Overview
  - OCI DevOps Service
  - OCI Code Editor

- **`02` · Container-based Application Development**
  - Developer workflow: Dockerfile → build → tag → push
  - OCI Container Registry (OCIR): repository structure, push/pull
  - Registry authentication (auth tokens)
  - Image versioning and lifecycle
  - OCIR in build and deployment pipelines

- **`03` · Managed Kubernetes**
  - OKE cluster architecture and node pools
  - Provisioning a cluster and connecting with kubectl
  - Deploying workloads from OCIR
  - Exposing services
  - Scaling and upgrades
  - OKE-specific behaviors vs. generic managed Kubernetes

- **`04` · Serverless Functions**
  - OCI Functions development lifecycle (build, deploy, invoke)
  - Configuration and dependencies
  - Invocation and scaling model
  - Cold-start behavior
  - Cost/latency reasoning: functions vs. always-on containers

- **`05` · API Management**
  - API Gateway deployments, routes, and backends (Functions, OKE, HTTP)
  - Request/response policies
  - Authentication and authorization at the edge
  - Rate limiting
  - Gateway fronting vs. direct exposure

- **`06` · OCI Streaming (Serverless and Managed)**
  - Streams and partitions
  - Producer/consumer mechanics
  - Kafka-compatible API
  - Serverless Streaming vs. managed Apache Kafka
  - Replayability, throughput, and operational-burden trade-offs

- **`07` · Serverless Queues**
  - Producing and consuming with OCI Queue
  - Visibility timeout
  - Delivery guarantees and failure handling
  - Queue vs. stream (competing consumers vs. replayable log)

- **`08` · Serverless Events**
  - Event types emitted by OCI services
  - Rules and pattern matching
  - Actions: Functions, Streaming, Notifications
  - Events vs. Queue vs. Streaming

- **`09` · Testing & Securing Cloud Native Applications**
  - OCI Vault: secrets and key management from application code
  - Container image scanning
  - Signed images in OCIR
  - Testing best practices for cloud-native services

- **`10` · Monitoring & Troubleshooting Cloud Native Applications**
  - Monitoring metrics and alarms
  - Logging
  - Application Performance Monitoring (APM) tracing
  - End-to-end troubleshooting of deployed applications

---

## How to Use This Plan

The per-module loop:

1. **Study** the Oracle course module; extend that module's bullet list above with what the
   course actually covers (keep bullets as subtopics — competencies to deliver, not lesson prose).
2. **Draft**: ask Claude to write `NN-<topic-slug>.md` from the enriched spec, using the
   `lesson-craft` skill with `../GUIDELINES.md`.
3. **Quiz**: study the lesson, then run `lesson-eval` coverage mode — the quiz is generated blind
   from the spec, so it measures readiness against the outside-in bar, not the lesson's own
   wording. Feed gaps back into lesson edits.
4. Repeat per module; quiz banks accumulate under `assessments/`. The bank doubles as self-test
   material — answer each quiz cold before reading its answer key.
5. **Sweep** before the exam: run `lesson-eval` currency verification across all drafted lessons
   in one pass — the as-of tags on quota/limit figures (see `../GUIDELINES.md`, volatile facts)
   make stale numbers easy to find and re-verify.

Modules follow the course order and build on each other (fundamentals → containers → OKE →
serverless → messaging → security → observability); later lessons assume the earlier ones.
