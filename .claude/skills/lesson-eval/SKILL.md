---
name: lesson-eval
description: >-
  Audit and improve an existing technical lesson (the NN-topic.md notes in this repo) — grades
  the LESSON, not the reader. Use when asked to validate, audit, check, grade, find gaps in, or
  improve a finished lesson: "does this cover the spec," "what's missing," "is this written
  well" — as opposed to lesson-craft, which is for *writing* one, and lesson-drill, which quizzes
  *you* live and grades your typed answers (use that instead when asked to drill, quiz me, or
  test my recall). Two modes: REVIEW ("is this written well," the default when no mode is
  named) audits the prose against the lesson-craft rubric (weak writing); QUIZ ("does this cover
  the spec" / "what's missing") generates a fresh mock assessment blind from the topic spec and
  grades whether the lesson answers it (missing content). Loads the track's GUIDELINES.md (plus
  STUDY-PLAN.md when the track has one) as the content spec, and the lesson-craft skill as the
  craft rubric.
---

# Lesson Eval

A **process** for auditing a *finished* lesson and improving it. This skill does not redefine what a
good lesson is — it consumes existing definitions and adds the audit/grade/improve loop:

- **What good content looks like** → the lesson's entry in the track's `STUDY-PLAN.md` when the
  track has one (the authoritative spec for *what this lesson must cover*), plus `GUIDELINES.md`
  (audience, domain). Content without a study plan falls back to the declared scope (see A1).
- **What good writing looks like** → the `lesson-craft` skill (structure §1, depth bar §2, the §8
  One-Pass Test) — read under the track's declared **format mode** (see lesson-craft's "Format
  modes" section, which states exactly what a mode does and doesn't change: `deep-prose` or
  `scannable-reference`).

**Cite those sources in findings; do not restate their rules here.** This skill owns only the
procedure, the severity rubric, the report templates, and the approval gate.

The two angles are complementary. **Quiz** asks *"is the right content here at all?"* —
generated outside-in from the topic, so the lesson cannot teach to its own test. **Review** asks
*"is what's here written well?"*. Content gaps are invisible to a review; prose flaws are
invisible to a quiz. Both angles grade the *lesson*. A third, separate skill, **`lesson-drill`**,
grades the *reader* — an interactive, one-question-at-a-time recall session against a finished
lesson, distinct from this skill's written audit reports; route there instead when the request is
to be quizzed or drilled live.

---

## Execution Workflow

### Step 0 — Resolve target and mode

Take from the user's request:
- **Target lesson** — a path to any lesson file, or a lesson number within a track (e.g. `04`,
  `03-ai-platform-engineering/04-mcp-and-tool-use.md`, or `misc/security/passkey-login.md`). If
  missing or ambiguous, ask which file.
- **Mode** — `review` (default) · `quiz` · `both`. `both` deliberately still runs quiz first (find
  content gaps), then review (quality of what's there, and of anything just added) — that order is
  load-bearing for the handoff (see A6), independent of which mode a bare invocation defaults to.
  `coverage` and `craft` are accepted as aliases for `quiz` and `review`, for anyone used to the
  old names.

Locate and load the track's parameter file exactly as `lesson-craft`'s Execution Workflow step 1
does — same search, same halt-and-ask rule; not restated here. While loading it, also check for a
**`Format mode: <name>`** line for Mode B to consult (see `lesson-craft`'s "Format modes" section
for the default when it's absent). Quiz mode also needs a content spec: the track's
`STUDY-PLAN.md` when one exists, otherwise the fallback in A1. Review mode also needs the
`lesson-craft` skill.

---

## Mode A — Quiz (assessment-driven)

The order matters: build the test **before** reading the answers.

### A1. Gather the spec — blind

Read **only**: the lesson *title*, the `GUIDELINES.md` purpose/audience, the lesson's
`STUDY-PLAN.md` paragraph when the track has one — otherwise the user's stated scope for the
lesson (ask for it if unclear) — and the track's declared framing/threads from its
`GUIDELINES.md`, if any (e.g. `03-ai-platform-engineering` declares an *augment* / *operate*
dual thread). **Do not open the lesson body yet.** From these plus your own domain knowledge, write a short list of
the **competencies** a competent treatment must deliver. This list is the external bar.

### A2. Generate the mock assessment

Write **8–15 questions**, weighted toward **apply / trade-off / why** (e.g. *"when would you choose
X over Y and why"*, *"why is it built this way"*, applied scenarios); include only a few recall
questions for load-bearing facts. For each question record:
- the **competency** it tests (from A1) and, when the track declares a framing, the **thread**,
- a short **model answer** (answer key) — so grading is grounded, not impressionistic.

**Make each run a *different* assessment.** First glance at the existing quizzes in the lesson's
`assessments/` bank (see A5) and deliberately diversify from them: rotate which competencies you
probe, vary the scenarios and angles, and avoid re-asking the same questions. The goal across runs
is a varied bank that exercises the topic from many directions, not the same quiz regenerated.

### A3. Grade the lesson

**Now** read the lesson body. For each question assign:
- **Covered** — the lesson answers it; cite the section name or a stable anchor.
- **Partial** — touched but incomplete; cite where and state what's missing.
- **Missing** — not addressed; state the gap.
- **Covered (stale)** — answered but the fact looks outdated; route to *Currency verification* below.

### A4. Quiz report (deliverable)

| Q | Competency | Thread | Verdict | Evidence / gap |
| :--- | :--- | :--- | :--- | :--- |

Omit the **Thread** column when the track declares no framing. Add a one-line score (e.g. *"11/14 covered · 2 partial · 1 missing"*) and the gaps ranked by
importance to the topic.

### A5. Save the quiz artifact

Each run is saved as a **new** file in the lesson's quiz bank — never overwrite a prior one. The bank
lives in an `assessments/` folder sibling to the lesson file, at
`<lesson-dir>/assessments/<lesson-slug>/` (create it if absent); name each quiz with a
zero-padded sequence number, `quiz-01.md`, `quiz-02.md`, … (scan the folder and use the next index).
So `03-ai-platform-engineering/04-mcp-and-tool-use.md` accumulates
`03-ai-platform-engineering/assessments/04-mcp-and-tool-use/quiz-01.md`, `quiz-02.md`, …, and
`misc/security/passkey-login.md` would use `misc/security/assessments/passkey-login/quiz-01.md`.
Structure (`<seq>` is this quiz's own sequence number, matching its `quiz-NN.md` filename — not the
lesson number, which appears in the source-spec line below):

```markdown
# Assessment <seq>: <Lesson Title>

> Generated by lesson-eval (quiz mode) on <date>. Source spec: STUDY-PLAN.md entry for NN
> (or the declared scope, for content without a study plan).

## Questions
1. <question>  _(competency · thread)_
...

## Answer key
1. <model answer> — **<verdict>**: <evidence/gap>
...

## Coverage summary
<score line + ranked gaps>
```

### A6. Improve (gated)

Present the report and **stop**. After the user approves which gaps to close, add the missing content
to the **correct section** of the lesson, written to the `lesson-craft` craft and the
track's snippet languages. The saved quiz records the verdicts *as found* (pre-improvement) — leave
them as the historical record; append a short "Resolved" note listing which gaps this run closed.
Then offer to run Mode B on the changed passages.

---

## Mode B — Review (One-Pass Test audit)

Load `lesson-craft` and audit the lesson against it — **cite its items, don't copy its
rules**. Before citing any structure, analogy, or bullet-vs-prose finding — the items
`lesson-craft`'s "Format modes" section names as mode-dependent — look up the track's declared mode
(from Step 0) and audit that point against *that mode's* rules instead: under `scannable-reference`
mode, bullets-over-prose or a dated Limits table are compliant, not findings. Every other item in
`lesson-craft`, including the full One-Pass Test checklist, is audited unconditionally, regardless
of mode. Collect findings into one table, most-severe first:

| Location | Severity | Rule | Finding | Proposed fix |
| :--- | :--- | :--- | :--- | :--- |

`Location` is a section name or stable anchor, never a bare `§N.M` number. `Rule` cites the authoring
item (e.g. *"§2 depth bar — <6 snippets"*, *"§8 wrong-models-refuted"*). Severity:

- **Blocker** — factual error or stale fact taught as current; missing Summary; a stated count not
  fully shown (misleads the reader).
- **Major** — depth-bar miss (no walkthrough, <6 snippets, undivided section), unrefuted wrong mental
  model, missing trade-off / pre-empted objection, inverted-depth headline, or a `deep-prose`-mode
  lesson with a required analogy entirely absent (§3.4/Format modes) — a missing required element,
  not a style nit.
- **Minor** — terminology nits, an analogy that is present but weak or imprecise, or formatting.

Present the table and **stop**. After approval, apply fixes (voice and snippet languages preserved),
then re-run the affected §8 items on each changed passage.

---

## Currency verification (both modes)

Fast-moving domains go stale. Extract the volatile claims for the lesson's domain — for AI:
model names/IDs, pricing, context windows, vLLM/KServe/MIG versions, MCP/API/protocol shapes; in
other domains: library and protocol versions, CLI flags, spec/standard status, CVEs and security
advisories — and verify each against a current source via `WebSearch`/`WebFetch`; use the
`claude-api` skill for Anthropic-specific facts rather than the open web. Mark each **confirmed / stale / unverifiable**. A stale fact is a **Blocker** in
review mode and a **Covered (stale)** verdict in quiz mode. Leave `unverifiable` claims in the
closeout for the user to confirm. (The web tools are deferred — fetch their schemas with ToolSearch
at run time.)

---

## Closeout

End any run with a short summary: what was reported, what was applied vs. deferred, the quiz artifact
path (quiz mode), and any claims left `unverifiable`. Never apply edits before the user approves the
report.
