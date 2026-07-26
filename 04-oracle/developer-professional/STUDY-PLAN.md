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
  - *From the official course TOC:*
    - Cloud Native Overview — satisfied by this lesson's own opening paragraph (what
      "cloud-native" means, and the common "runs in the cloud" misreading); not a dedicated section
    - Cloud-Native vs. Cloud-Enabled vs. Cloud-Based Applications: the three-tier classification
      for how modernized an application actually is — a lift-and-shift, a partially-modernized
      app, and a true cloud-native rebuild
    - Key Pillars of Cloud Native Development (Microservices, Containers, DevOps, CI/CD, Service
      Mesh) — the lesson covers Service Mesh as the fifth pillar, noting the managed **OCI Service
      Mesh** offering's retirement (May 31, 2025) and Istio-on-OKE as the current implementation
    - Benefits and Challenges of Cloud Native Development: agility, scalability, and cost
      elasticity vs. the distributed-systems and organizational cost of getting there — broader
      than the monolith-vs-microservices trade-off below
    - Microservice Architecture: decomposition drivers, and where a monolith still wins
    - The Twelve-Factor methodology as microservice design discipline
    - DevOps: Overview — generic concept; `GUIDELINES.md` caps this at one anchoring paragraph,
      no dedicated bullet
    - OCI DevOps Service — the pillar the exam actually tests in depth
      - Project as the umbrella resource; code repositories and external connections
      - Managing pull requests on native code repositories
      - Build pipelines and the `build_spec.yaml` contract: stages, exported variables, vault variables
      - Artifacts and the Artifact Registry hand-off from build to deploy
      - Deployment pipelines: environments, targets (OKE, instance groups, Functions), approval stages
      - Deployment strategies: rolling, blue-green, canary — and what "rollback" means in each
      - Triggers: push and PR events, exclusion filters, and the build-loop trap
    - OCI DevOps CI/CD — folded into the build/deployment pipeline sub-bullets above
    - Case Study: Developing Cloud Native Solutions on OCI — satisfied by this lesson's own
      worked walkthrough (§5, "One Commit to OKE"); not a separate case study
    - OCI Code Editor: scope and genuine use cases
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: monolith vs. microservices (independent deployability and scaling vs. distributed-
      systems operational cost)

- **`02` · Container-based Application Development**
  - *From the official course TOC:*
    - Containerization Overview, Docker Components, Working with Docker Images — generic Docker
      fundamentals; excluded per `GUIDELINES.md`'s audience rule ("do not re-teach container...
      basics"), not a gap
    - OCIR as an IAM-native registry resource: repository, tenancy namespace, region scoping
    - Registry authentication: Auth Tokens, Bearer Tokens, Security Tokens, and resource principals
    - Tags vs. digests, and the mutable-`latest` risk; repository immutability
    - Image lifecycle: retention policies, exempt versions, and unversioning an image
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - OCIR in the delivery pipeline: the artifact hand-off from Module `01`'s build stage
    - Image security: scanning and signing — *(named in the OCIR product docs, not the course
      TOC; full depth deferred to Module `09`, matching how Module `04`'s spec defers the same
      topic)*
    - Trade-off: Auth Token vs. resource principal for registry access (long-lived human credential
      vs. no credential at all)

- **`03` · Managed Kubernetes**
  - *From the official course TOC:*
    - Introduction to Kubernetes — generic K8s fundamentals; excluded per `GUIDELINES.md`'s
      audience rule (the reader already knows Kubernetes), not a gap
    - OKE cluster architecture: Basic vs. Enhanced clusters, and the control-plane/data-plane split
    - Node pool models: managed nodes vs. virtual nodes vs. self-managed nodes
    - Prerequisite to create an OKE cluster: VCN/subnet requirements, the IAM policy and
      dynamic-group grants needed before creation, and the service limits that gate it
    - Policy configuration for cluster creation and deployment: the IAM policies OKE itself needs
      to create and manage a cluster and its node pools — distinct from a workload's own resource
      principal
    - Cluster access: kubeconfig generation, Cloud Shell, and public vs. private endpoints
    - Pulling images from a registry during deployment: the `imagePullSecret` requirement,
      continuing Module `02`'s registry-pull side — already satisfied by this lesson's walkthrough
    - OSOK (OCI Service Operator for Kubernetes) — provisioning OCI resources from Kubernetes
      manifests; the lesson's worked example now provisions a **MySQL DB System**, matching the
      official course demo (Autonomous Database, Streaming, and Queue remain named as other
      supported resource types)
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Kubernetes Secrets encryption at the cluster level
    - Load balancer and persistent storage provisioning for OKE workloads
    - Scaling and upgrade strategies for node pools
    - Admission controllers and pod security policies
    - OKE-specific behaviors vs. generic managed Kubernetes
    - Trade-off: managed nodes vs. virtual nodes (operational control and full feature access vs.
      zero node management)
    - Scope note: cluster-level observability (audit logs, application logs, metrics) deferred to
      Module `10`

- **`04` · Serverless Functions**
  - *From the official course TOC:*
    - Oracle Functions Overview
    - Functions Concepts: applications and functions, the two-level resource model — the
      application as the subnet, shared-config, and logging boundary; configuration and
      dependencies (application-level vs. function-level, read at runtime)
    - Understanding Functions Operations: function images (FDK-generated vs. existing Docker
      images vs. custom Dockerfiles; the Fn Project CLI, and how OCI Functions differs from
      open-source Fn); development lifecycle (build, push to OCIR, deploy, invoke); invocation
      paths (direct invoke via CLI/SDK/signed HTTP vs. invoked *by* other services — API Gateway,
      Events, Notifications, alarms); invocation and scaling model (concurrency, memory and
      timeout limits, cold start, scale-to-zero)
    - OCI Functions Use Cases: the typical shapes a function fills — event-driven backend logic,
      an API Gateway backend, scheduled automation, a data-processing step in a larger pipeline
    - Prerequisites to Creating an OCI Function: what must exist before an application or
      function can be created — a compartment, a VCN/subnet, and dynamic-group/policy grants
      - Networking: subnet placement and private access to OCI services — what a function can reach
      - Access control: resource principals and dynamic groups — how a function authenticates to
        other OCI services; container permissions
    - Managing Functions — folded into the development-lifecycle sub-bullet above
    - Scheduling OCI Functions: cron-expression scheduling via the Resource Scheduler, and why a
      scheduled invocation always runs Detached (no caller sitting around for a synchronous
      response)
    - Functions Integration with OCI Services — folded into the invocation-paths sub-bullet above
    - Pre-Built Functions Overview: Oracle's catalog of ready-made, verified Functions for common
      integration tasks, deployable without writing a handler
    - Function image security: scanning and signing (registry-side depth belongs to module 09)
    - OCI Functions: Logs, Metrics, and Tracing — function logs, metrics, and tracing enabled
      here, analysed in module 10
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: Functions vs. always-on OKE workloads (cold-start latency and per-invocation cost
      vs. idle capacity)

- **`05` · API Management**
  - *From the official course TOC:*
    - API Gateway Overview — generic concept; `GUIDELINES.md` caps this at one anchoring
      paragraph before getting to OCI specifics, no dedicated bullet
    - OCI API Gateway: Introduction — gateways, deployments, routes, and the deployment
      specification
    - OCI API Gateway: Concepts — backend types (HTTP/HTTPS URL, Functions, stock response,
      login and logout backends); request and response policies (validation, transformation,
      response caching, rate limiting); path parameters and context variables, the substitution
      language policies are written in
    - Prerequisites for using API Gateway: a VCN with a **regional** subnet (AD-specific subnets
      are not allowed), a DHCP options set with a working DNS resolver, backend reachability (an
      internet gateway if the backend is public), and the IAM policy letting a group specify that
      VCN/subnet — plus a public-IP grant if the gateway itself is public
      - Networking: public vs. private gateway placement and subnet requirements
    - Dynamic Authentication: authentication and authorization — authorizer functions vs. OAuth
      2.0/OIDC with remote JWKS or static keys; multiple authentication servers
    - Dynamic Routing: selecting a backend at request time from a header, query parameter,
      host/subdomain, path parameter, authorization claim, or usage plan — the mechanism behind a
      single gateway serving multiple tenants or backend versions
    - Monitoring APIs: gateway logging and metrics (analysed in module 10)
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Transport security: custom domains and TLS certificates, CORS, mTLS and custom trust stores
    - Trade-off: gateway fronting vs. direct load-balancer exposure (edge policy and authn vs. one
      less hop)

- **`06` · OCI Streaming and Streaming with Apache Kafka**
  - *From the official course TOC (the course module is titled "Serverless Streaming" —
    Kafka-cluster content below is this track's own scope extension, not TOC-named):*
    - Introduction: streams, partitions, and stream pools
    - Features: Kafka compatibility on Streaming — the Kafka API surface and Kafka Connect
      *(a feature of serverless Streaming itself, distinct from the separate Streaming with
      Apache Kafka service below)*; producer/consumer mechanics: publish, consume, consumer
      groups, cursors and offsets
    - Fundamentals: message retention and replay; IAM and access control for streams and stream
      pools
    - Use Cases: log aggregation and SIEM ingestion, clickstream analysis, IoT telemetry
      ingestion, event-driven microservices and analytics pipelines
    - Demo: Working with OCI Streams — mechanics
  - *Additional depth beyond the TOC (grounded in official OCI docs — a separate, non-serverless
    product this track chose to cover alongside the module, not part of "Serverless Streaming"):*
    - Streaming with Apache Kafka as a distinct service: starter vs. high-availability clusters,
      broker count and sizing, supported Kafka versions, cluster configuration
    - Kafka cluster security: SASL/SCRAM, mTLS, ACLs, private VCN connectivity
    - Trade-off: serverless Streaming vs. managed Kafka clusters vs. self-managed Kafka (operational
      burden vs. throughput control vs. ecosystem/API completeness)

- **`07` · Serverless Queues**
  - *From the official course TOC (every current bullet already maps to a TOC header — no
    additional-depth group needed for this module):*
    - Queue: Introduction — queues, producing and consuming with OCI Queue
    - OCI Queue Features: visibility timeout, and extending it by updating an in-flight message;
      channels — ordering and consumer multiplexing within a single queue; dead letter queues and
      the delivery-attempt count that feeds them
    - OCI Queue Fundamentals: delivery guarantees and failure handling
    - OCI Queue Operations: queue IAM policies and access control
    - OCI Queue Use Cases: decoupling components for independent scaling, queue-triggered
      Functions for task processing, absorbing traffic spikes ahead of a smaller, slower pool of
      consumers
    - Which Messaging Solution?: Trade-off: Queue vs. Stream (competing consumers vs. replayable
      partitioned log)
    - Demo: OCI Queue — mechanics

- **`08` · Serverless Events**
  - *From the official course TOC:*
    - Fundamental Concepts: services that produce events, and their event types
    - Event Messages and Event Types: the event message envelope (CloudEvents schema) — the
      fields rules actually match on; rules, filtering, and pattern matching, including
      attribute and tag matching
    - Rule Actions: actions — Functions, Streaming, Notifications — and the IAM prerequisites
      each action requires
    - Events Use Cases: reacting to Object Storage/Compute/database lifecycle events, fanning a
      single event out to multiple actions, and event-driven automation that needs no polling
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Event metrics and rule troubleshooting
    - Trade-off: Events vs. Queue vs. Streaming (rule-routed notification vs. competing-consumer
      delivery vs. replayable log)

- **`09` · Testing & Securing Cloud Native Applications**
  - *From the official course TOC:*
    - Cloud Native Testing Overview — generic concept; `GUIDELINES.md` caps this at one
      anchoring paragraph before getting to OCI specifics
    - Cloud Native Testing Strategies — confirmed **real course content** (this resolves the
      module's earlier open question — it is not a docs-anchor-less topic to cut); the specific
      strategies taught (contract testing and resilience/chaos testing are the plausible
      candidates) aren't visible from the TOC title alone — fill this in while studying the module
    - OCI Vault: Introduction — vault fundamentals: vault types (Default vs. Virtual Private),
      key protection mode (HSM vs. Software)
    - OCI Vault: Integration with OCI Services — master encryption keys, data encryption keys,
      and envelope encryption; how DevOps and Functions consume a Vault key
    - OCI Vault: Secrets — secrets: versions, rotation states, and reading a secret from
      application code (SDK) vs. injecting it (DevOps vault variables, function configuration);
      key and secret lifecycle: rotation, backup, replication
    - Image Security — container image scanning: OCIR scanning, Vulnerability Scanning Service,
      function image scanning
    - OKE: Container Image Security — a **cluster-level** image verification policy naming which
      Vault master encryption keys must have signed an image, enforced via the
      `ImagePolicyWebhook` admission path — the OKE-side counterpart to Module `03`'s existing
      admission-controllers bullet, not a duplicate of it
    - Enforcing the Use of Signed Images from OCIR — the separate **Functions-side** enforcement:
      an application-level signature-verification policy naming a Vault key, gating function
      create/update/invoke to only signed images — a distinct mechanism from OKE's cluster-level
      policy above
    - Function Container Permissions — already covered in Module `04` §5.3 (the unprivileged
      `fn` user, stripped Linux capabilities); this module revisits it under the security framing
      rather than introducing new mechanics
    - OCI Certificates Integration — the OCI Certificates service: certificate authorities,
      certificates, and CA bundles, issued or imported, with automatic renewal
    - Custom Trust Store — the Certificates-service-side trust store that Module `05`'s API
      Gateway mTLS bullet already names consuming
    - Mutual TLS (mTLS) Support — cross-references Module `05`'s existing "mTLS and custom trust
      stores" bullet, written before this service was confirmed as the actual source
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: secret injected at deploy time vs. fetched at runtime (rotation latency and blast
      radius vs. a startup dependency on Vault)

- **`10` · Monitoring & Troubleshooting Cloud Native Applications**
  - *From the official course TOC:*
    - Overview — generic concept, one anchoring paragraph
    - Monitoring: metrics — namespaces, dimensions, aggregation, and MQL (Monitoring Query
      Language); custom metrics published from application code; alarms — conditions, intervals,
      suppression, and delivery through Notifications topics and subscriptions
    - Logging Service: custom vs. service vs. audit logs; log groups; the Logging query language;
      the agent
    - Demo: Functions Metrics & Logs; Demo: API Gateway Metrics & Logs; Demo: OKE Cluster Metrics
      & Logs — mechanics; these three confirm Modules `03`, `04`, and `05`'s own "deferred to
      Module 10" scope notes were each correctly deferred here
    - Application Performance Monitoring: APM domains and data keys, Trace Explorer, spans and
      trace attributes, service topology; instrumentation — APM Java agent, browser agent, and
      OpenTelemetry ingest
    - Demo: Oracle Functions Tracing; Demo: Microservice Application Distributed Tracing —
      mechanics, mapping to end-to-end troubleshooting below
    - Demo: Debugging Container Applications in OKE — OKE-specific debugging technique
      (`kubectl logs`/`describe`/events) tied into the Logging Service, distinct from the
      cross-service correlation bullet below
    - End-to-end troubleshooting: correlating a request across gateway logs, function logs, and
      traces
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: metrics vs. logs vs. traces for a given diagnostic question
    - Scope note: what this lesson defers to the `observability-professional` sub-track

**Open questions for module 10** (resolve while studying, don't guess):

- Connector Hub (routing Logging/Monitoring/Streaming/Functions/Object Storage) is the natural
  glue across modules 06/08/10 but appears in none of them and isn't in the Logging docs TOC —
  add it to module 10 only if the course covers it.

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
