---
name: lesson-drill
description: >-
  Run an interactive, one-question-at-a-time active-recall session against a finished lesson (the
  NN-topic.md notes in this repo). Use when asked to drill, quiz me live, test my recall, or
  practice a lesson — as opposed to `lesson-eval` quiz mode, which generates a written mock
  assessment to audit the lesson's own coverage. `lesson-drill` is a study tool for the reader: it waits
  for a typed answer, grades it against the lesson, and remembers what was missed so the next
  session starts there. Also supports a cross-lesson mode that drills service/setting
  discrimination across a track.
---

# Lesson Drill

An **interactive study loop**, distinct from `lesson-craft` (writes a lesson) and `lesson-eval`
(audits a lesson's coverage/prose). This skill tests the *reader*: it asks one question, waits for
a typed answer, grades it against the lesson, and tracks misses across sessions so drilling
compounds instead of restarting cold each time.

Where `lesson-eval` quiz mode generates a written assessment *blind from the spec* to check
whether the lesson answers it — content-gap auditing, answer key visible in the same file —
`lesson-drill` generates questions *from the finished lesson*, withholds every answer until the user commits to
one, and grades strictly. It also does not cap recall questions the way `lesson-eval` does: a cert
exam rewards fast, correct recall of load-bearing facts, so this skill drills those at volume.

---

## Execution Workflow

### Step 0 — Resolve target

Take from the user's request:
- **Target lesson** — a lesson number within a track (e.g. `07`), a full path, or `cross` for the
  cross-lesson discrimination set (see Mode C). If no argument is given, use Mode D (auto-pick).
- **Question count** — default 10; honor a number if the user names one (e.g. "drill 07, 5
  questions").

Locate the track's parameter file exactly as `lesson-craft`'s Execution Workflow step 1 does — same
walk-up search from the lesson's folder toward the repo root, same halt-and-ask rule if it's
missing. Not restated here.

---

## Mode A — Drill a single lesson

### A1. Load

Read: the lesson body in full, its `STUDY-PLAN.md` module entry (the outside-in bar — drill what
the exam wants, not only what the lesson happens to emphasize), `GUIDELINES.md` for audience and
the declared **exam-ready / production-reasoning** threads (when the track declares them), and the
lesson's progress log if one exists at `<sub-track>/drills/<lesson-slug>.md`.

### A2. Build the question set

Generate fresh from the lesson body — never reuse a prior session's exact phrasing. Target mix for
`N` questions:

- **~40% recall** — load-bearing facts: service limits, defaults, required prerequisites,
  resource-model relationships. Mine the lesson's `Limits and Sources` table and its `> ⚠️`
  callouts first — those are already curated as the exam-relevant ones.
- **~30% apply** — a short scenario, then "what happens / what's the fix."
- **~20% discriminate** — which service or setting applies, and *what in the requirement decides
  it* (not just the name).
- **~10% why** — the mechanic behind a behavior; the production-reasoning thread.

Rules:
- **Previously-missed items go first.** Anything unresolved in the progress log is re-asked —
  reworded, not copied verbatim — before any new material. This is where the spacing effect
  actually happens; don't skip it because it's easier to write fresh questions.
- One question asks one thing. Never embed the answer or a hint in the question text.
- If a figure appears (a quota, a timeout, a limit), the question should test whether the user
  knows what it *forces*, not just the digit — matching `GUIDELINES.md`'s "shape over number" rule
  for volatile facts.
- For each question, silently note its topic, type, and a short model answer for grading — do not
  print this; it is your own scratch reference for Step A4.

### A3. Run it — one question at a time

For each question, print exactly:

```
Q<n> of <total> · <topic> · <type>
<question text>
```

Then **stop and wait for the user's typed answer.** Never print more than one question at once,
and never reveal the answer before grading. If the user types `skip`, mark it a miss with no
partial credit and move on. If the user types `stop`, end the session immediately, grade nothing
further, and proceed to Step A5 with whatever was answered.

### A4. Grade each answer

Immediately after each answer, print a verdict — `correct` / `partial` / `wrong` — followed by
**two or three lines at most**:

- Name the specific mechanic missed, don't re-lecture the whole topic.
- Cite the lesson section to reread, e.g. `→ §4.2`.
- **Grade strictly.** An answer that gestures at the right area without the specific mechanic is
  `partial`, not `correct`. A generous drill teaches nothing.
- **Grade the shape, not the number.** If the user gets the consequence of a limit right but the
  figure wrong (or vice versa), that's `partial` either way — per `GUIDELINES.md`'s "shape over
  number" rule, the reasoning is the load-bearing part.

Then immediately print the next question (Step A3) until the set is exhausted or the user stops.

### A5. Close out

Print: a score line (`7/10 correct · 2 partial · 1 wrong`), the list of missed items by topic, and
one sentence on what to re-drill first next time.

Then append to the progress log at `<sub-track>/drills/<lesson-slug>.md` (create the `drills/`
folder and file if absent). Format:

```markdown
# Drill log — <NN> <Lesson Title>

## <date> · <N> questions · <correct> correct · <partial> partial · <wrong> wrong
Missed:
- <short item name> _(type)_ — <one line on what was missed>. §<section>
Correct after a previous miss: <item> ✓ (missed <earlier date>)
```

Append-only — never rewrite a prior session's entry, matching how `assessments/` treats its
records as historical.

**Do not edit the lesson.** If the same item is missed across two separate sessions, say so
explicitly and name it as a candidate for a `> Nuance:` callout back in the lesson — surfacing the
candidate is the deliverable; only edit the lesson if the user separately asks, per this repo's
report-then-approve convention.

---

## Mode C — Cross-lesson discrimination (`/lesson-drill cross`)

Same mechanics as Mode A (one question at a time, strict grading, append-only log), different
sourcing and a different bar for "correct."

**Build the question set from:**
- Each module's `Trade-off: X vs. Y` bullet in the track's `STUDY-PLAN.md` (one per module).
- The track's declared **domain trade-off pairs** in `GUIDELINES.md`, if any.
- Scope notes that split a topic across modules (e.g. one module deferring a sub-topic to a later
  one, or two modules each owning a different layer of the same concern) — these make good
  discrimination questions precisely because they're easy to conflate.

**Question shape:** state a scenario with the requirement spelled out but the service or setting
withheld — never name the two options being discriminated between in the question itself.

**Grading bar:** correct requires naming **the discriminator** — the specific clause in the
requirement that forces the choice — not just landing on the right service name. "Queue, because
it needs replay" when replay wasn't the requirement is `wrong` even if "Queue" was the right
answer for a different reason.

Log to `<sub-track>/drills/cross-lesson.md`, same format as Mode A's log, with topic entries naming
the pair (e.g. `Queue vs Stream`) instead of a lesson section.

---

## Mode D — No target given (`/lesson-drill`)

Read every progress log under `<sub-track>/drills/`. Pick, in order:
1. The lesson with the most unresolved (never-since-corrected) misses.
2. If tied or no logs have misses, the lesson least recently drilled (oldest last-session date, or
   never drilled at all).

State the pick and the one-line reason before starting Mode A on it.

---

## Closeout

Every run ends with the Step A5 (or Mode C equivalent) summary already printed inline — no separate
report needed. State the log file path written to.
