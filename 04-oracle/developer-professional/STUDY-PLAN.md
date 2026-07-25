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
  - Key Pillars of Cloud Native Development (Microservices, Containers, DevOps, CI/CD) —
    *(Service Mesh is in the original course-outline pillar list but not yet confirmed as actual
    lesson content; see open question below)*
  - Microservice Architecture: decomposition drivers, and where a monolith still wins
  - The Twelve-Factor methodology as microservice design discipline
  - OCI DevOps Service — the pillar the exam actually tests in depth
    - Project as the umbrella resource; code repositories and external connections
    - Managing pull requests on native code repositories
    - Build pipelines and the `build_spec.yaml` contract: stages, exported variables, vault variables
    - Artifacts and the Artifact Registry hand-off from build to deploy
    - Deployment pipelines: environments, targets (OKE, instance groups, Functions), approval stages
    - Deployment strategies: rolling, blue-green, canary — and what "rollback" means in each
    - Triggers: push and PR events, exclusion filters, and the build-loop trap
  - OCI Code Editor: scope and genuine use cases
  - Trade-off: monolith vs. microservices (independent deployability and scaling vs. distributed-
    systems operational cost)

- **`02` · Container-based Application Development**
  - OCIR as an IAM-native registry resource: repository, tenancy namespace, region scoping
  - Registry authentication: Auth Tokens, Bearer Tokens, Security Tokens, and resource principals
  - Tags vs. digests, and the mutable-`latest` risk; repository immutability
  - Image lifecycle: retention policies, exempt versions, and unversioning an image
  - OCIR in the delivery pipeline: the artifact hand-off from Module `01`'s build stage
  - Image security: scanning and signing — *(named in the OCIR docs TOC; full depth deferred to
    Module `09`, matching how Module `04`'s spec defers the same topic)*
  - Trade-off: Auth Token vs. resource principal for registry access (long-lived human credential
    vs. no credential at all)

- **`03` · Managed Kubernetes**
  - OKE cluster architecture: Basic vs. Enhanced clusters, and the control-plane/data-plane split
  - Node pool models: managed nodes vs. virtual nodes vs. self-managed nodes — *(self-managed
    nodes appear in the OKE docs TOC as a third create-cluster option; confirm course coverage
    before drafting — see open question below)*
  - Cluster access: kubeconfig generation, Cloud Shell, and public vs. private endpoints
  - Kubernetes Secrets encryption at the cluster level
  - Load balancer and persistent storage provisioning for OKE workloads
  - Scaling and upgrade strategies for node pools
  - OSOK (OCI Service Operator for Kubernetes) — provisioning OCI resources from Kubernetes manifests
  - Admission controllers and pod security policies — *(named in the OKE docs TOC; confirm course
    coverage before drafting — see open question below)*
  - OKE-specific behaviors vs. generic managed Kubernetes
  - Trade-off: managed nodes vs. virtual nodes (operational control and full feature access vs.
    zero node management)
  - Scope note: cluster-level observability (audit logs, application logs, metrics) deferred to
    Module `10`

- **`04` · Serverless Functions**
  - Applications and functions: the two-level resource model — the application as the subnet,
    shared-config, and logging boundary
  - Function images: FDK-generated vs. existing Docker images vs. custom Dockerfiles; the Fn
    Project CLI, and how OCI Functions differs from open-source Fn
  - Development lifecycle: build, push to OCIR, deploy, invoke
  - Configuration and dependencies: application-level vs. function-level config, read at runtime
  - Invocation paths: direct invoke (CLI/SDK/signed HTTP) vs. invoked *by* other services
    (API Gateway, Events, Notifications, alarms)
  - Invocation and scaling model: concurrency, memory and timeout limits, cold start, scale-to-zero
  - Access control: resource principals and dynamic groups — how a function authenticates to
    other OCI services; container permissions
  - Networking: subnet placement and private access to OCI services — what a function can reach
  - Function image security: scanning and signing (registry-side depth belongs to module 09)
  - Observability hooks: function logs, metrics, and tracing enabled here, analysed in module 10
  - Trade-off: Functions vs. always-on OKE workloads (cold-start latency and per-invocation cost
    vs. idle capacity)

- **`05` · API Management**
  - Gateways, deployments, routes, and the deployment specification
  - Backend types: HTTP/HTTPS URL, Functions, stock response, login and logout backends
  - Request and response policies: validation, transformation, response caching, rate limiting
  - Path parameters and context variables — the substitution language policies are written in
  - Authentication and authorization: authorizer functions vs. OAuth 2.0/OIDC with remote JWKS or
    static keys; multiple authentication servers
  - Transport security: custom domains and TLS certificates, CORS, mTLS and custom trust stores
  - Networking: public vs. private gateway placement and subnet requirements
  - Gateway logging and metrics (analysed in module 10)
  - Trade-off: gateway fronting vs. direct load-balancer exposure (edge policy and authn vs. one
    less hop)

- **`06` · OCI Streaming and Streaming with Apache Kafka**
  - Streams, partitions, and stream pools
  - Producer/consumer mechanics: publish, consume, consumer groups, cursors and offsets
  - Message retention and replay
  - Kafka compatibility on Streaming: the Kafka API surface and Kafka Connect
  - Streaming with Apache Kafka as a distinct service: starter vs. high-availability clusters,
    broker count and sizing, supported Kafka versions, cluster configuration
  - Kafka cluster security: SASL/SCRAM, mTLS, ACLs, private VCN connectivity
  - IAM and access control for streams and stream pools
  - Trade-off: serverless Streaming vs. managed Kafka clusters vs. self-managed Kafka (operational
    burden vs. throughput control vs. ecosystem/API completeness)

- **`07` · Serverless Queues**
  - Queues, producing and consuming with OCI Queue
  - Visibility timeout, and extending it by updating an in-flight message
  - Channels: ordering and consumer multiplexing within a single queue
  - Dead letter queues and the delivery-attempt count that feeds them
  - Delivery guarantees and failure handling
  - Queue IAM policies and access control
  - Trade-off: Queue vs. Stream (competing consumers vs. replayable partitioned log)

- **`08` · Serverless Events**
  - Services that produce events, and their event types
  - The event message envelope (CloudEvents schema) — the fields rules actually match on
  - Rules, filtering, and pattern matching, including attribute and tag matching
  - Actions: Functions, Streaming, Notifications — and the IAM prerequisites each action requires
  - Event metrics and rule troubleshooting
  - Trade-off: Events vs. Queue vs. Streaming (rule-routed notification vs. competing-consumer
    delivery vs. replayable log)

- **`09` · Testing & Securing Cloud Native Applications**
  - Vault fundamentals: vault types (Default vs. Virtual Private), key protection mode (HSM vs.
    Software)
  - Master encryption keys, data encryption keys, and envelope encryption
  - Secrets: versions, rotation states, and reading a secret from application code (SDK) vs.
    injecting it (DevOps vault variables, function configuration)
  - Key and secret lifecycle: rotation, backup, replication
  - Container image scanning: OCIR scanning, Vulnerability Scanning Service, function image
    scanning
  - Image signing and verification: signing with a Vault key, enforcing signed images on deploy
  - Testing practices for cloud-native services — *(no docs anchor found; ground in course content
    or cut — see module notes)*
  - Trade-off: secret injected at deploy time vs. fetched at runtime (rotation latency and blast
    radius vs. a startup dependency on Vault)

- **`10` · Monitoring & Troubleshooting Cloud Native Applications**
  - Metrics: namespaces, dimensions, aggregation, and MQL (Monitoring Query Language)
  - Custom metrics: publishing from application code
  - Alarms: conditions, intervals, suppression, and delivery through Notifications topics and
    subscriptions
  - Logging: custom vs. service vs. audit logs; log groups; the Logging query language; the agent
  - APM: domains and data keys, Trace Explorer, spans and trace attributes, service topology
  - Instrumentation: APM Java agent, browser agent, and OpenTelemetry ingest
  - End-to-end troubleshooting: correlating a request across gateway logs, function logs, and
    traces
  - Trade-off: metrics vs. logs vs. traces for a given diagnostic question
  - Scope note: what this lesson defers to the `observability-professional` sub-track

**Open questions for modules 01–03, 09–10** (resolve while studying, don't guess):

- Module 01's Key Pillars list originally named **Service Mesh** alongside Microservices,
  Containers, DevOps, and CI/CD — confirm whether the course actually teaches it (and on what OCI
  service, if any) before adding it back; it's dropped from the bullet list for now rather than
  asserted.
- Module 03's **self-managed nodes** and **admission controllers / pod security policies** both
  appear as real topics in the OKE docs TOC but aren't confirmed as course content — study the
  module before committing lesson depth to either.
- Module 09's "testing practices" bullet has no OCI docs anchor — ground it in what the course
  actually teaches, or cut it.
- Connector Hub (routing Logging/Monitoring/Streaming/Functions/Object Storage) is the natural
  glue across modules 06/08/10 but appears in none of them and isn't in the Logging docs TOC —
  add it to module 10 only if the course covers it.
- Confirm whether Container Instances is in scope before drafting module 04 — it would extend the
  module's trade-off bullet from a pair to a triple.

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
