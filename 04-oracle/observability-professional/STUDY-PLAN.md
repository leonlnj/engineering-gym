<!--
  Each module's bullet list below is the SPEC that `lesson-eval` quiz mode grades the lesson
  against — write it BEFORE writing the lesson, so the bar is set outside-in.
  The subtopics below are transcribed from the official course Table of Contents for
  "Oracle Cloud Infrastructure Observability Professional (2025): Hands-on Workshop" (7 modules,
  pp. 12-299). Extend each list with course-specific bullets while studying that module — see
  "How to Use This Plan".
-->

# OCI Observability Professional — Study Plan

The Oracle Cloud Infrastructure (OCI) Observability Professional course teaches how to run the
four observability signals on OCI: metrics through the Monitoring service, logs through Logging and
Log Analytics, resource state changes through Events, and distributed application performance
through Application Performance Monitoring — with Stack Monitoring layering a fleet-wide resource
model over the raw metric feed.

Track conventions (audience, snippet languages, trade-off pairs) live in [`../GUIDELINES.md`](../GUIDELINES.md).

## Teaching Objectives

Oracle's official learning-path outcomes — on completing this path you can:

- Understand how to proactively monitor cloud environments.
- Automatically respond to cloud resource changes in real-time.
- Diagnose issues quickly from centrally managed log data.
- Gain insights from a massive amount of log data with advanced analytics.
- Monitor performance of applications along with their distributed components.

**Course prerequisite (Oracle's own):** a foundational-level understanding of Oracle Cloud
Infrastructure and basic knowledge of cloud computing. This is narrower than
[`../GUIDELINES.md`](../GUIDELINES.md)'s audience (an experienced engineer new to OCI specifics);
the `GUIDELINES.md` audience is what governs how the lessons are pitched.

Oracle's five **"Skills you will learn"** statements (OCI Monitoring, OCI Events, OCI Logging, OCI
Log Analytics, OCI APM) are not repeated as a list here — each opens the module entry it names
(`02`, `04`, `03`, `05`, `06` respectively).

---

## Modules

One lesson per Oracle course module, numbered to match. Each module's bullet list is the coverage
spec for that lesson — add bullets as you study. File names follow `../GUIDELINES.md`
(`NN-<topic-slug>.md`, sequential per sub-track):

| Lesson | Course module | Pages |
| --- | --- | --- |
| `01-pillars-of-observability.md` | Pillars of Observability | 12–20 |
| `02-monitoring-service.md` | Monitoring Service | 21–51 |
| `03-logging-service.md` | Logging Service | 52–96 |
| `04-events-service.md` | Events Service | 97–135 |
| `05-log-analytics.md` | Log Analytics | 136–199 |
| `06-application-performance-monitoring.md` | Application Performance Monitoring | 200–271 |
| `07-stack-monitoring.md` | Stack Monitoring | 272–299 |

---

- **`01` · Pillars of Observability**
  - *From the official course TOC:*
    - What is Observability
      - Traditional Monitoring — generic concept; `GUIDELINES.md` caps this at one anchoring
        paragraph, not a dedicated section
      - Challenges with Traditional Monitoring — the known-unknowns ceiling: dashboards answer
        only the questions asked at instrumentation time
      - Definition: Observability — inferring internal state from external outputs (metrics, logs,
        traces, and events as the OCI fourth signal)
      - Comparing Monitoring and Observability — satisfied by this lesson's own trade-off
        treatment (see additional-depth group); not a separate section
    - Introducing Observability and Management Services
      - Observability & Management Services — the OCI service family this course covers
        (Monitoring, Logging, Log Analytics, Events, APM, Stack Monitoring) and how a signal flows
        between them
      - Use Case: Observability and Management in DevOps — satisfied by this lesson's own worked
        walkthrough; not a separate case study
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: monitoring vs. observability — pre-declared dashboards and alarms for known
      failure modes vs. the ability to answer a question about production that was not anticipated
      when the instrumentation was written. This restates the TOC's "Comparing Monitoring and
      Observability" header, so it is written once, here, not also as a TOC-section bullet.

- **`02` · Monitoring Service**
  - *Oracle "Skills you will learn" — OCI Monitoring:* actively and passively monitor cloud
    environments using metrics, and define alarms for notifications.
  - *From the official course TOC:*
    - Monitoring Service Overview
      - OCI Monitoring Service: Getting Started
      - Monitoring Capabilities — what the service ingests (service metrics vs. custom metrics)
        and what it emits (alarms, dashboards, the Console Metrics Explorer)
      - Monitoring Service Workflow
    - Monitoring Concepts
      - Metrics — the metric data model: namespace, metric name, dimensions, resource group,
        and the datapoint stream
      - Intervals and Resolutions — the collection interval vs. the query aggregation window, and
        why they are not the same knob
      - Statistics — the aggregation functions (`mean`, `sum`, `rate`, `count`, `p50`/`p90`/…)
      - Alarms — alarm anatomy: metric query, threshold, trigger rule, pending duration
      - Metric: Query Components — the pieces of a Monitoring Query Language (MQL) expression
      - Demo: Monitoring Concepts — mechanics
    - Notifications Service — **defined once here**; lesson `04` (Events) references it as a
      rule-action destination rather than re-teaching it
      - Notifications Service: Overview — topics and subscriptions; protocol types (Email,
        PagerDuty, Slack, HTTPS, SMS, Function, Streaming)
      - Notifications Service: Creating a Topic
      - Demo: Notifications Service — mechanics
    - Alarms
      - Alarms Workflow — from metric breach to notification: firing, pending state, `OK`
        transition, repeat-notification cadence
      - Best Practices — alarm design: absence detection, suppression windows, message templates,
        one alarm per condition
      - Demo: Alarms — mechanics
    - Access and Limits
      - Ways to Access Monitoring — Console, CLI, SDK, `PostMetricData` / `SummarizeMetricsData`
        API, Query language
      - IAM Policies for Access — `metrics-read`, `alarms` verbs, the `metrics` resource
      - IAM Policies with Restricted Access — scoping a policy to a single metric namespace with a
        `where` condition
      - Limits of Monitoring Service — the exam-relevant caps: metric retention period, datapoint
        posting rate, dimensions per metric, alarms per compartment (tag with as-of dates in the
        Limits and Sources table per `GUIDELINES.md`)
    - Metric Queries
      - Building Metric Queries — the MQL grammar: `metric[interval]{dimension filter}.statistic`
        plus a window function
      - Sample Queries
      - Nested Queries — composing one MQL expression from another (e.g. `join()`, arithmetic
        across metrics)
      - Summary
      - Demo: Metric Queries — mechanics
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: a metric alarm vs. a log-query-based detection for spotting the same incident — a
      cheap, pre-aggregated numeric signal at fixed resolution and short retention vs.
      arbitrary-field matching over raw log events at higher cost and latency. Feeds
      `/lesson-drill cross` alongside lesson `05`'s Logging-vs-Log-Analytics pair.
    - Scope note: this lesson is the home for the metrics/alarms depth that
      `developer-professional/10` explicitly defers to this sub-track.

- **`03` · Logging Service**
  - *Oracle "Skills you will learn" — OCI Logging:* centrally manage all types of logs (Service
    Logs, Custom Logs, Audit Logs) through a single pane of glass.
  - *From the official course TOC:*
    - Logging Service: Overview
      - OCI Logging Service — the unified log plane: one query surface over three log categories
      - Types of Logs — Service logs, Custom logs, Audit logs, and how each is enabled
      - Service Flow — log group → log → ingestion → search/archive/connector
    - Logging Concepts
      - Log Groups — the IAM and organisational container for logs; compartment placement
      - Logging Concepts — log object model: log OCID, category, retention, enabled/disabled state
    - Service Logs
      - Service Log Format — the common envelope OCI services emit (`oracle`, `data`, `source`,
        `time`, `type`)
      - Object Storage Logs — read/write access-event logging for buckets
      - Load Balancer Logs — access and error logs
      - VCN Flow Logs — accepted/rejected traffic records, and the capture-filter that scopes them
      - Demo: Service Logs — mechanics
    - Custom Logs
      - Custom Log Ingestion — application-emitted logs; the log OCID as the ingestion target
      - Using Unified Monitoring Agent — the Fluentd-based agent that ships custom logs from a
        compute instance; **distinct from the Management Agent** used for Log Analytics ingestion
        in lesson `05` — name the difference
      - Agent Communication Workflow — how the agent authenticates (dynamic group + policy) and
        posts
      - Agent Configuration — the agent config resource: inputs (tail a file, `syslog`), parsers,
        and the destination log
      - Demo: Custom Logs — mechanics
    - Access & Explore Logs
      - IAM Policies — `log-groups`, `log-content`, `unified-configuration` resource types and
        their verbs
      - Searching Logs — the Log Search page: time range, compartment scope, log selection
      - Viewing Log Events — a single log record's fields and the JSON detail view
    - Logging Queries
      - Log Search — the search entry points and saved searches
      - Logging Query Specification — the query language structure: `<source> | <operators>`
      - Log Streams — a live-tail continuous query
      - Fields — the built-in fields (`datetime`, `logContent`, `type`, …) and dotted paths into
        `data`
      - Data Types — string, number, boolean, array, object and how comparisons behave
      - Tabular Operators — `search`, `where`, `summarize`, `sort`, `top`, `head`
      - Scalar Operators — field access, arithmetic, string and date functions
      - Demo: Logging Queries — mechanics
    - Connector Hub — **owned here.** `developer-professional/10`'s spec carries Connector Hub as
      an unconfirmed extra; this course's TOC confirms it, so the full treatment lives in this
      lesson.
      - Overview and Key Concepts — a connector: source, optional task, target; the supported
        source/target service matrix
      - Connectors Workflow — creating a connector and the IAM policy it needs to move data
        between services
      - Take Actions for Use Cases — archive logs to Object Storage, fan logs to Streaming, invoke
        a Function on a log match, feed Monitoring or Log Analytics
      - Demo: Connector Hub — mechanics
    - Audit Logs
      - OCI Audit Service — always-on control-plane logging; no enablement step
      - Audit Log — the audit event schema and what counts as an auditable operation
      - Viewing Audit Log Events — Console search and the `SearchEvents` API
      - Reasons to use Audit logs — compliance, forensic reconstruction, change attribution
      - Required IAM policies — `audit-events` read access and the tenancy-level scope
      - Demo: Audit Logs — mechanics
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: Service logs vs. Custom logs — zero-config, fixed schema, emitted by OCI itself
      vs. arbitrary sources with an agent config and agent lifecycle to own.
    - Scope note: this lesson is the home for the Logging Service depth that
      `developer-professional/10` explicitly defers to this sub-track.

- **`04` · Events Service**
  - *Oracle "Skills you will learn" — OCI Events:* create rules to run automated actions based on
    state changes of OCI resources.
  - *Scope note (overlap with `developer-professional/08-serverless-events.md`):* the developer
    track already covers the CloudEvents envelope, rule pattern-matching syntax, and the three
    action types in full. This lesson keeps the module but pitches its depth at the **observability
    angle** — Events as a signal, rule reliability, and rule metrics — and cross-references
    `developer-professional/08` for envelope mechanics rather than re-deriving them.
  - *From the official course TOC:*
    - Events Concepts
      - Overview — event-driven automation with no polling loop to run
      - Using Events vs. Explicit Polling — satisfied by this lesson's own trade-off treatment
        (see additional-depth group); not a separate section
      - OCI Events Service Concepts — rules, conditions, actions; the compartment scope of a rule
      - An Example of OCI Events Service in Action
    - Event Messages and Event Types
      - What is an Event? — a resource lifecycle/state-change signal, not a metric or a log line
      - What does an Event look like? — the CloudEvents JSON envelope (`eventType`, `source`,
        `data`, `resourceId`); cross-reference `developer-professional/08` for full field detail
      - Services That Produce Events — the emitting-service catalogue and how to check coverage
      - OCI Service Event Types — the `com.oraclecloud.<service>.<action>` naming scheme
    - Rule Actions
      - Rule Action Destinations — the three targets: Streaming, Functions, Notifications
      - Streaming Service: Overview / Creating a Stream — service intro the reader already has from
        `developer-professional/06`; anchoring paragraph only, not a section — not a gap
      - Rule Action Type: Streaming — the IAM policy the Events service needs to write to a stream
      - Oracle Functions: Overview / Creating a Function — service intro the reader already has
        from `developer-professional/04`; anchoring paragraph only, not a section — not a gap
      - Rule Action Type: Functions — the dynamic group + policy letting Events invoke the function
      - Notifications Service: Overview / Creating a Topic — **defined in lesson `02`**; here it is
        referenced only as an action destination
      - Rule Action Type: Notifications — the policy letting Events publish to a topic
      - Demo: Rule Actions — mechanics
    - Working with Rules
      - What are Rules? — condition (event-type list + attribute/tag filter) plus action list
      - Typical Rule Design Workflow
      - Creating Rules
      - Configuring Rules in the OCI Console — the condition builder, attribute matching, tag
        matching
      - Rule Design Considerations — idempotent actions, the self-triggering loop trap, filtering
        as narrowly as possible, per-rule action limits
      - Rule Metrics — the Monitoring metrics Events publishes per rule (matched, invoked, failed,
        throttled) and using them to detect a broken rule
      - Demo: Working with Rules — mechanics
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: an Events rule vs. a Monitoring alarm as an automation trigger — a discrete
      resource **state change** (bucket created, DB System stopped) vs. a **metric threshold**
      crossing over a window. This restates the TOC's "Using Events vs. Explicit Polling" framing,
      so it is written once, here. Feeds `/lesson-drill cross`.

- **`05` · Log Analytics**
  - *Oracle "Skills you will learn" — OCI Log Analytics:* log ingestion methods, identifying data
    patterns with enrichments, aggregations, correlations, and creating meaningful visualizations.
  - *From the official course TOC:*
    - Log Analytics Overview
      - What is Log Analytics? — a parse-on-ingest analytics store, distinct from the Logging
        service's raw event search
      - From Raw Logs to Insights — the parse → enrich → index → analyse pipeline
      - Log Analytics Helps You Answer… — the diagnostic question classes it targets
    - Onboarding Log Analytics
      - Log Analytics Data Flow — collection method → log source → parser → log group → Log Explorer
      - Source Types & Collection Methods — agent, service connector, Object Storage, on-demand
        upload
      - Logging-Analytics-Users and Logging-Analytics-Admins — the two default IAM groups and what
        each can do
      - Logging-Analytics-Super-Admins — the tenancy-wide administrative role and when it is needed
      - Demo: Log Analytics Onboarding — mechanics
    - Log Data Management
      - Data Governance and Access Controls — compartment scoping, group-based access, data-access
        control rules
      - Storage Management — active vs. archived storage tiers, retention period, recall from
        archive
      - Log Groups and Log Partitioning — the log group as the access and retention boundary
    - Log Explorer and Dashboards
      - Log Explorer Overview — the visual analysis surface (records, fields sidebar, visualise
        panel)
      - Log Explorer User Interface
      - Dashboards — saved searches pinned as widgets; out-of-the-box vs. custom dashboards
      - Demo: Log Explorer and Dashboards — mechanics
    - Entities
      - What is an Entity? — the modelled object a log line is *about* (a host, a database, a
        load balancer)
      - Log Analytics Entity — entity type, entity properties, association to a log source
      - Entity Example: Oracle Database Instance
    - Log Sources
      - What is a Log Source? — the definition binding a file pattern/collection method to a parser
        and a set of labels
      - Features of Log Sources — parsers, field extraction, labels, extended fields,
        source-level enrichment
      - Log Sources: Completing the Picture
    - Labels and Lookups
      - Fields, Labels and Lookups - Overview
      - Fields and Labels — parsed fields vs. condition-assigned labels used to tag and filter
      - Lookups — joining an external table (CSV) to enrich records at query time
    - Demo: Log Sources — mechanics
    - Basic Analytics
      - Filtering Log data
      - Removing Noise for Analysis — excluding known-benign patterns
      - Data Visualizations — the chart types Log Explorer offers and when each fits
      - Demo: Basic Analytics — mechanics
    - Query Language
      - Parts of Query Language — the pipe-delimited command structure
      - Dissecting a Query — a worked breakdown of `search … | where … | stats … | sort …`
      - Demo: Query Language — mechanics
    - Log Cluster
      - Log Reduction with Log Clustering — the `cluster` command grouping similar records to
        collapse volume
      - Demo: Log Clustering — mechanics
    - Link Analysis
      - Correlation with link command — grouping records into transactions across sources
      - Correlation Using the link Command — the `link` command syntax and the fields it groups on
      - Demo: Link Analysis — mechanics
    - Log Ingestion with Management Agent
      - Management Agent Cloud Service: Overview — the agent used for Log Analytics collection;
        **distinct from the Unified Monitoring Agent** used for the Logging service in lesson `03`
        — name the difference
      - When should you use Management Agent? — continuous collection from compute hosts and
        on-prem systems
      - Configure Log Collection from Compute Instances — the agent config, log source assignment,
        entity association
      - Demo: Log Ingestion with Management Agent — mechanics
    - Log Ingestion with Service Connector
      - Log Collection with Service Connector — routing OCI Logging or other sources into Log
        Analytics via Connector Hub
      - When should you use Service Connector for Logs Collection? — already-in-Logging data,
        no agent to deploy
      - Demo: Log Ingestion with Service Connector — mechanics
    - Log Ingestion with Object Storage
      - OCI Object Storage — bucket as a landing zone for log files
      - When should you use Object Storage Log Ingestion? — batch/historical loads, third-party
        exports
      - Object Collection Rule — the rule watching a bucket prefix and feeding matched objects to
        a log source
      - Object Collection Rule Creation: OCI CLI Example — the `oci log-analytics` command form
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: Logging Service search vs. Log Analytics — raw retrieval over retained log events
      with a light query language vs. parsed, enriched, correlated data (entities, labels,
      lookups, `link`) at the cost of ingest-time processing and a storage tier to manage.
      Feeds `/lesson-drill cross` alongside lesson `02`'s alarm-vs-log-query pair.

- **`06` · Application Performance Monitoring**
  - *Oracle "Skills you will learn" — OCI Application Performance Monitoring:* monitor application
    stacks with distributed tracing, real-user monitoring, and synthetic monitoring, providing
    deep visibility into end-user experience.
  - *From the official course TOC:*
    - Application Performance Monitoring (APM): Getting Started
      - Key Purposes of APM — trace, RUM, synthetic; where each answers a different question
      - Service Architecture — APM domain, data keys, collector endpoint, Trace Explorer
      - Fundamental Concepts
        - Key Concepts and Terminologies — span, trace, operation, service, dimension, APDEX
        - Glance APM Features in Oracle Cloud Console
      - Demo: Trace Explorer — mechanics
    - APM Domains
      - Perform Prerequisites — the IAM policy and compartment prerequisites
      - Setting Up APM Domain — free vs. paid data, data keys (private vs. public), region
    - Data Sources
      - Types of Data Sources
      - APM Tracer and APM Browser Agent
      - APM Browser Agent and Java Agent Hybrid
      - APM Java Agent and WebLogic Java
      - OpenTelemetry Operator for Kubernetes — OTel ingest path into an APM domain
    - Monolithic Applications and Microservices
      - Monolithic Applications
      - Microservices
      - Monolithic and Microservices: Comparison — generic architecture contrast; anchoring
        paragraph only per `GUIDELINES.md`, not a section
    - Fundamentals of Distributed Tracing
      - What is Distributed Tracing? — generic concept; anchoring paragraph only
      - How Distributed Tracing Works — context propagation, parent/child spans
      - Distributed Tracing Standards — W3C Trace Context, OpenTelemetry, and how APM ingests each
    - Distributed Tracing with APM
      - Generate Spans and Report to APM — the tracer → collector → domain path
      - Trace Data Collection — sampling, span attributes, the collector upload
      - Key Characteristics
    - Using Trace Explorer
      - Discover Trace Explorer Essentials
      - Trace Explorer Query — the query language over spans/traces and its aggregations
      - GeoMap View
      - Trace Details View — the waterfall, span detail, and linked logs
      - Enable Tracing with Oracle Functions — the Functions-side tracing enablement (cross-links
        `developer-professional/04` and `/10`)
      - Demo: Access APM & Features — mechanics
    - Dashboards and Metrics
      - Obtain Permissions to Use Dashboard and Metrics
      - Know Default APM Metrics and Dimensions — the `oracle_apm*` namespaces and their dimensions
      - Demo: Creating Custom Dashboard — mechanics
      - Demo: Creating Alarms for APM Metric — mechanics (uses the Monitoring alarm model from
        lesson `02`)
    - Application Performance Index (APDEX)
      - Using Apdex (Application Performance Index) — satisfied/tolerating/frustrated buckets, the
        target threshold, per-page and per-service APDEX
    - Span Filters and Metric Groups
      - Custom Dimension for Span Metrics
      - Enable Metric Groups — deriving a metric stream from a span filter
      - Demo: Configure metric Alarms using Span Filters — mechanics
    - Real User Monitoring
      - Collect User Information with the Browser Agent — page-load timing, AJAX, JS errors, real
        session geography
    - Synthetic Monitoring
      - Synthetic monitoring — scripted checks (REST, browser, scripted browser) on a schedule
      - Use Synthetic Data — how synthetic results feed the same Trace Explorer and dashboards
      - Demo: Synthetic Monitoring — mechanics
    - Dedicated Vantage Points
      - Dedicated Vantage Points – Why do they matter? — running synthetic checks from a network
        location Oracle's public vantage points cannot reach
      - Monitor Secured Applications Using Dedicated Vantage Points
      - Demo: Dedicated Vantage Points — mechanics
      - On-Premises Vantage Point (OPVP) for Synthetic Monitoring — the on-prem DVP form factor
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: Real User Monitoring vs. Synthetic Monitoring — the actual user population with
      no control over coverage or timing vs. deterministic, scheduled probes from chosen vantage
      points that run whether or not anyone is using the app.
    - Scope note: this lesson is the home for the APM depth that `developer-professional/10`
      explicitly defers to this sub-track.

- **`07` · Stack Monitoring**
  - *Scope note:* Stack Monitoring is a full 28-page workshop module (pp. 272–299) but is **not**
    named in Oracle's "Skills you will learn" list. Treat this lesson as workshop-scoped content —
    real and worth knowing, weighted below the five skills-list modules for exam purposes.
  - *From the official course TOC:*
    - Stack Monitoring Overview
      - Stack Monitoring: Discovery and Monitoring of Applications and Infrastructure in Hybrid
        Environments
      - Stack Monitoring Overview — the resource model: resource types, resource instances, and
        the topology that links them
      - Resource Home Pages — the per-resource-type health, metrics, and alarm view
      - Enterprise Health and Alarms: Monitor Across the Fleet at Scale
      - Demo: Stack Monitoring Overview — mechanics
    - Resource Discovery
      - Following Discovery Best Practices — the discovery job, required agents/credentials, and
        common failure modes
      - Demo: Resource Discovery — mechanics
    - Monitoring Custom Resources
      - Extensibility: Monitor Almost any Resource and Enrich Application Topology with Custom
        Resources
      - Monitoring Any Host Process as a Resource
      - Extending Stack Monitoring Out-of-the-Box Resource Types with Prometheus, Telegraf, and
        Collectd
      - Extending Application Topology with OCI Services
      - Demo: Monitoring Custom Resources — mechanics
    - Baselines and Anomalies
      - ML-Based Baselines and Anomalies — learned per-metric normal ranges and anomaly flags,
        distinct from a static Monitoring alarm threshold
    - Metric Extensions
      - Identifying App-Specific Metrics Using Metric Extensions — a SQL/JMX/OS-command probe that
        adds a metric to a resource type
      - Metric Extensions Life Cycle — draft → test → publish → deploy to resources
      - Demo: Metric Extensions — mechanics
    - Monitoring Templates
      - Demo: Monitoring Templates — a reusable set of alarm definitions applied across a resource
        group
    - Maintenance Windows
      - Demo: Maintenance Windows — suppressing alarms and health rollups during planned work
  - *Additional depth beyond the TOC (grounded in official OCI docs):*
    - Trade-off: Stack Monitoring vs. the raw Monitoring service — a fleet-wide resource model
      with out-of-the-box resource types, topology, and ML baselines vs. assembling the same
      picture by hand from metric namespaces and static alarms.

---

## How to Use This Plan

The per-module loop:

1. **Study** the Oracle course module; extend that module's bullet list above with what the
   course actually covers (keep bullets as subtopics — competencies to deliver, not lesson prose).
   The TOC above is transcribed from the official course; treat it as a floor, not a ceiling.
   Keep each module's bullets split into the two labeled groups — *From the official course TOC*
   and *Additional depth beyond the TOC (grounded in official OCI docs)* — adding any real
   TOC-named gaps found, and labeling docs-grounded extensions honestly.
2. **Draft**: ask Claude to write `NN-<topic-slug>.md` from the enriched spec, using the
   `lesson-craft` skill with `../GUIDELINES.md`.
3. **Quiz**: study the lesson, then run `lesson-eval` quiz mode — the quiz is generated blind
   from the spec, so it measures readiness against the outside-in bar, not the lesson's own
   wording. Feed gaps back into lesson edits.
4. Repeat per module; quiz banks accumulate under `assessments/`. The bank doubles as self-test
   material — answer each quiz cold before reading its answer key.
5. **Sweep** before the exam: run `lesson-eval` currency verification across all drafted lessons
   in one pass — the as-of tags on quota/limit figures (see `../GUIDELINES.md`, volatile facts)
   make stale numbers easy to find and re-verify.

Modules follow the course order and build on each other: pillars → the Monitoring (metrics)
service → the Logging service → Events → Log Analytics → APM → Stack Monitoring. Each later lesson
assumes the earlier ones — `04` reuses Notifications from `02`, `05` contrasts itself against the
Logging service from `03`, `06` and `07` both reuse the alarm model from `02`.

Do `developer-professional/` first (see `../README.md`): the observability exam assumes you can
already build and deploy the workloads being monitored. Where a service is shared, this plan's
module bullets name the `developer-professional` lesson that owns the base treatment so the two
sub-tracks do not duplicate each other.
